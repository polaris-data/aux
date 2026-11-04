#![cfg(feature = "http")]
use aux_rs::{
    ByteStream, HttpMethod, HttpRequest, StandardTransport, StreamingHttpTransport,
    TransportConfig, TransportErrorKind,
};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
async fn server(response: Vec<u8>, interval: Option<Duration>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = [0; 8192];
        let read = socket.read(&mut request).await.unwrap();
        assert!(read > 0);
        for chunk in response.chunks(1024) {
            if socket.write_all(chunk).await.is_err() {
                break;
            }
            if let Some(interval) = interval {
                tokio::time::sleep(interval).await;
            }
        }
    });
    format!("http://{address}/data")
}
fn request(url: String) -> HttpRequest {
    HttpRequest {
        method: HttpMethod::Get,
        url,
        headers: Vec::new(),
        body: Vec::new(),
    }
}
#[tokio::test]
async fn streaming_is_not_limited_by_total_response_size_or_duration() {
    let bytes = vec![b'x'; 64 * 1024];
    let mut response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        bytes.len()
    )
    .into_bytes();
    response.extend(&bytes);
    let url = server(response, Some(Duration::from_millis(3))).await;
    let transport = StandardTransport::new(TransportConfig {
        timeout: Duration::from_millis(100),
        max_message_bytes: 16 * 1024,
        ..TransportConfig::default()
    })
    .unwrap();
    let mut response = transport.execute_stream(request(url)).await.unwrap();
    assert_eq!(response.status, 200);
    let mut read = Vec::new();
    while let Some(chunk) = response.body.next().await {
        read.extend(chunk.unwrap());
    }
    assert_eq!(read, bytes);
    transport.shutdown().await.unwrap();
}
#[tokio::test]
async fn truncated_response_reports_terminal_error() {
    let url = server(
        b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\n\r\nshort".to_vec(),
        None,
    )
    .await;
    let transport = StandardTransport::new(TransportConfig::default()).unwrap();
    let mut response = transport.execute_stream(request(url)).await.unwrap();
    let mut failed = false;
    while let Some(chunk) = response.body.next().await {
        if chunk.is_err() {
            failed = true;
            break;
        }
    }
    assert!(failed);
    assert!(response.body.next().await.is_none());
}
#[tokio::test]
async fn shutdown_closes_unpolled_bodies_and_discards_queued_bytes() {
    let mut bytes = b"HTTP/1.1 200 OK\r\nContent-Length: 65536\r\n\r\n".to_vec();
    bytes.extend(vec![1; 65536]);
    let url = server(bytes, None).await;
    let transport = StandardTransport::new(TransportConfig::default()).unwrap();
    let mut response = transport.execute_stream(request(url)).await.unwrap();
    tokio::time::timeout(Duration::from_secs(2), transport.shutdown())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        response.body.next().await.unwrap().unwrap_err().kind,
        TransportErrorKind::Closed
    );
    assert!(response.body.next().await.is_none());
}
