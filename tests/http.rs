#![cfg(all(feature = "hyperliquid", feature = "http"))]

use aux_rs::hyperliquid::HyperliquidClient;
use std::time::Duration;

use aux_rs::{
    Error, HttpMethod, HttpRequest, HttpTransport, RequestOutcome, StandardTransport,
    TransportConfig, TransportErrorKind, hyperliquid::Hyperliquid,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::timeout,
};

const META: &str = include_str!("../crates/exchanges/hyperliquid/tests/fixtures/meta.json");

async fn read_request(stream: &mut TcpStream) -> (String, Vec<u8>) {
    let mut buffer = Vec::new();
    let header_end = loop {
        if let Some(position) = buffer.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
            break position + 4;
        }
        let mut chunk = [0; 1024];
        let count = timeout(Duration::from_secs(3), stream.read(&mut chunk))
            .await
            .unwrap()
            .unwrap();
        assert!(count > 0, "connection closed before request headers");
        buffer.extend_from_slice(&chunk[..count]);
    };
    let headers = String::from_utf8(buffer[..header_end].to_vec()).unwrap();
    let length = headers
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().unwrap())
        })
        .unwrap_or(0);
    while buffer.len() < header_end + length {
        let mut chunk = [0; 1024];
        let count = timeout(Duration::from_secs(3), stream.read(&mut chunk))
            .await
            .unwrap()
            .unwrap();
        assert!(count > 0, "connection closed before request body");
        buffer.extend_from_slice(&chunk[..count]);
    }
    (headers, buffer[header_end..header_end + length].to_vec())
}

async fn reply(stream: &mut TcpStream, status: u16, body: &str, extra_headers: &str) {
    let response = format!(
        "HTTP/1.1 {status} Response\r\nContent-Length: {}\r\nContent-Type: application/json\r\n{extra_headers}\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes()).await.unwrap();
}

#[tokio::test]
async fn cloned_providers_reuse_the_http_connection_and_preserve_native_body() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        for _ in 0..2 {
            let (headers, body) = read_request(&mut stream).await;
            assert!(headers.starts_with("POST /proxy/info HTTP/1.1"));
            assert!(
                headers
                    .to_ascii_lowercase()
                    .contains("content-type: application/json")
            );
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
                serde_json::json!({"type":"meta"})
            );
            reply(&mut stream, 200, META, "").await;
        }
    });
    let client = HyperliquidClient::new(Hyperliquid::custom(
        format!("http://{address}/proxy/"),
        "ws://unused",
    ))
    .unwrap();
    assert_eq!(client.meta().await.unwrap().universe[0].name, "ETH");
    assert_eq!(client.clone().meta().await.unwrap().universe[0].name, "ETH");
    timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn error_status_is_returned_without_retry_or_redirect() {
    for (status, extra_headers) in [
        (429, ""),
        (503, ""),
        (307, "Location: http://127.0.0.1:1/should-not-follow\r\n"),
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            read_request(&mut stream).await;
            reply(&mut stream, status, "native failure", extra_headers).await;
        });
        let client = HyperliquidClient::with_config(
            Hyperliquid::custom(format!("http://{address}"), "ws://unused"),
            TransportConfig {
                read_retry: aux_rs::BackoffConfig::disabled(),
                ..Default::default()
            },
        )
        .unwrap();
        assert!(matches!(client.meta().await,
            Err(Error::Exchange(aux_rs::hyperliquid::Error::Http { status: actual, body }))
            if actual == status && body == "native failure"));
        server.await.unwrap();
    }
}

#[tokio::test]
async fn response_limit_and_timeout_are_explicit_transport_errors() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        read_request(&mut stream).await;
        reply(&mut stream, 200, META, "").await;
    });
    let client = HyperliquidClient::with_config(
        Hyperliquid::custom(format!("http://{address}"), "ws://unused"),
        TransportConfig {
            max_message_bytes: 8,
            ..TransportConfig::default()
        },
    )
    .unwrap();
    assert!(matches!(client.meta().await,
        Err(Error::Transport(error)) if error.kind == TransportErrorKind::ResponseTooLarge));
    server.await.unwrap();

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        read_request(&mut stream).await;
        let mut byte = [0];
        // The client closes after its request deadline; no sleeps are needed.
        timeout(Duration::from_secs(3), stream.read(&mut byte))
            .await
            .unwrap()
            .unwrap();
    });
    let client = HyperliquidClient::with_config(
        Hyperliquid::custom(format!("http://{address}"), "ws://unused"),
        TransportConfig {
            timeout: Duration::from_millis(100),
            read_retry: aux_rs::BackoffConfig::disabled(),
            ..TransportConfig::default()
        },
    )
    .unwrap();
    assert!(matches!(client.meta().await,
        Err(Error::Transport(error)) if error.kind == TransportErrorKind::Timeout && error.outcome == RequestOutcome::Unknown));
    server.await.unwrap();
}

#[tokio::test]
async fn invalid_requests_do_not_expose_endpoint_credentials() {
    let transport = StandardTransport::new(TransportConfig::default()).unwrap();
    let error = transport
        .execute(HttpRequest {
            method: HttpMethod::Post,
            url: "not a url?secret=must-not-leak".into(),
            headers: vec![],
            body: vec![],
        })
        .await
        .unwrap_err();
    assert_eq!(error.outcome, RequestOutcome::NotSent);
    assert!(!format!("{error:?} {error}").contains("must-not-leak"));
}
