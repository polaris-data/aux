#![cfg(all(feature = "monaco", feature = "http"))]
use aux_rs::{monaco as m, *};
use serde_json::{Value, json};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::{sleep, timeout},
};
use zeroize::Zeroizing;
const PAIR: &str = "550e8400-e29b-41d4-a716-446655440000";
async fn request(socket: &mut TcpStream) -> (String, Vec<u8>) {
    let mut bytes = Vec::new();
    let end = loop {
        if let Some(n) = bytes.windows(4).position(|s| s == b"\r\n\r\n") {
            break n + 4;
        }
        let mut b = [0; 4096];
        let n = socket.read(&mut b).await.unwrap();
        assert!(n > 0);
        bytes.extend_from_slice(&b[..n]);
    };
    let head = String::from_utf8(bytes[..end].to_vec()).unwrap();
    let len = head
        .lines()
        .find_map(|l| {
            l.split_once(':')
                .filter(|(n, _)| n.eq_ignore_ascii_case("content-length"))
                .map(|(_, v)| v.trim().parse::<usize>().unwrap())
        })
        .unwrap_or(0);
    while bytes.len() < end + len {
        let mut b = [0; 4096];
        let n = socket.read(&mut b).await.unwrap();
        assert!(n > 0);
        bytes.extend_from_slice(&b[..n]);
    }
    (head, bytes[end..end + len].to_vec())
}
async fn reply(socket: &mut TcpStream, status: u16, body: &str, headers: &str) {
    socket.write_all(format!("HTTP/1.1 {status} Response\r\nContent-Length: {}\r\nConnection: close\r\n{headers}\r\n{body}",body.len()).as_bytes()).await.unwrap();
}
fn config() -> TransportConfig {
    TransportConfig {
        timeout: Duration::from_millis(150),
        read_retry: BackoffConfig {
            max_retries: 2,
            initial_delay: Duration::from_millis(5),
            max_delay: Duration::from_millis(20),
        },
        ..Default::default()
    }
}
fn exchange(addr: std::net::SocketAddr) -> m::Monaco {
    m::Monaco::custom(format!("http://{addr}/api"), "ws://unused", 1328).unwrap()
}
fn auth(e: &m::Monaco, protocol: m::RequestSigning) -> m::MonacoAuth<m::LocalSessionSigner> {
    use m::SessionSigner;
    let signer = m::LocalSessionSigner::from_bytes(Zeroizing::new([1; 32]));
    let public = hex::encode(signer.public_key());
    m::MonacoAuth::restore(
        e.clone(),
        signer,
        m::SessionInfo {
            address: format!("0x{}", "11".repeat(20)),
            client_id: "fixture".into(),
            session_public_key: public,
            expires_at: m::unix_millis().unwrap() / 1000 + 3600,
            chain_id: 1328,
        },
        protocol,
    )
    .unwrap()
}
fn market() -> m::TradingPairData {
    serde_json::from_value(
        json!({"id":PAIR,"marketType":"SPOT","tickSize":"0.01","quantityStepSize":"0.001"}),
    )
    .unwrap()
}
#[tokio::test]
async fn safe_public_reads_retry_429_and_preserve_native_final_errors() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let e = exchange(listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        for _ in 0..3 {
            let (mut socket, _) = listener.accept().await.unwrap();
            let (head, body) = request(&mut socket).await;
            assert!(head.starts_with("GET /api/v1/market/pairs "));
            assert!(body.is_empty());
            assert!(head.to_lowercase().contains("user-agent: aux-rs-monaco/"));
            assert!(!head.to_lowercase().contains("x-monaco"));
            reply(
                &mut socket,
                429,
                "{\"code\":\"RESOURCE_EXHAUSTED\",\"details\":{\"retryAfter\":1}}",
                "Retry-After: 0\r\n",
            )
            .await;
        }
        assert!(
            timeout(Duration::from_millis(50), listener.accept())
                .await
                .is_err()
        );
    });
    let c = m::MonacoClient::with_config(e, config()).unwrap();
    assert!(
        matches!(c.trading_pairs(Default::default()).await,Err(Error::Exchange(m::Error::Api {status:Some(429),body})) if body["code"]=="RESOURCE_EXHAUSTED")
    );
    c.transport().shutdown().await.unwrap();
    server.await.unwrap();
}
#[tokio::test]
async fn signed_writes_send_once_and_redirects_never_forward_credentials() {
    for (status, protocol) in [
        (503, m::RequestSigning::Legacy),
        (307, m::RequestSigning::Rfc9421),
        (400, m::RequestSigning::Legacy),
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let e = exchange(listener.local_addr().unwrap());
        let redirected = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let redirect = redirected.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let (head, body) = request(&mut socket).await;
            assert!(head.starts_with("POST /api/v1/orders "));
            let value: Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(value["price"], "100.0100");
            assert_eq!(value["idempotencyKey"], "persisted-submission");
            match protocol {
                m::RequestSigning::Legacy => {
                    assert!(head.to_lowercase().contains("x-monaco-signature:"))
                }
                m::RequestSigning::Rfc9421 => {
                    assert!(head.to_lowercase().contains("signature-input:"));
                    assert!(!head.to_lowercase().contains("x-monaco"));
                }
            }
            reply(
                &mut socket,
                status,
                "{\"code\":\"POST_ONLY_WOULD_CROSS\"}",
                &format!("Location: http://{redirect}/stolen\r\n"),
            )
            .await;
            assert!(
                timeout(Duration::from_millis(75), listener.accept())
                    .await
                    .is_err()
            );
            assert!(
                timeout(Duration::from_millis(75), redirected.accept())
                    .await
                    .is_err()
            );
        });
        let a = auth(&e, protocol);
        let c = m::MonacoClient::with_config(e, config())
            .unwrap()
            .with_auth(a);
        let mut r = m::CreateOrderRequest::limit(PAIR, "BUY", "100.0100", "1.000");
        r.idempotency_key = Some("persisted-submission".into());
        let result = c.place_order(&market(), r).await;
        if status == 503 {
            assert!(matches!(
                result,
                Err(Error::Exchange(m::Error::OutcomeUncertain {
                    status: Some(503),
                    ..
                }))
            ));
        } else {
            assert!(
                matches!(result,Err(Error::Exchange(m::Error::Api {status:Some(s),..})) if s==status)
            );
        }
        c.transport().shutdown().await.unwrap();
        server.await.unwrap();
    }
}
#[tokio::test]
async fn timeout_after_sending_is_unknown_and_protected_reads_are_not_replayed() {
    for read in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let e = exchange(listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            request(&mut socket).await;
            sleep(Duration::from_millis(225)).await;
            assert!(
                timeout(Duration::from_millis(75), listener.accept())
                    .await
                    .is_err()
            );
        });
        let a = auth(&e, m::RequestSigning::Legacy);
        let c = m::MonacoClient::with_config(e, config())
            .unwrap()
            .with_auth(a);
        let error = if read {
            c.orders(Default::default()).await.map(|_| ()).unwrap_err()
        } else {
            c.place_order(
                &market(),
                m::CreateOrderRequest::limit(PAIR, "BUY", "1.00", "1"),
            )
            .await
            .map(|_| ())
            .unwrap_err()
        };
        assert!(matches!(error,Error::Transport(e) if e.outcome==RequestOutcome::Unknown));
        c.transport().shutdown().await.unwrap();
        server.await.unwrap();
    }
}
