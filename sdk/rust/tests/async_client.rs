#![cfg(all(feature = "async", feature = "metric-models"))]

use aux_rs::{
    MetricClient,
    http::{OutboundRequest, TransportResponse},
    metric_models::EmptyRequest,
};
use std::thread;

#[tokio::test(flavor = "current_thread")]
async fn typed_and_raw_async_calls_preserve_the_application_transport() {
    let executor = thread::current().id();
    let client = MetricClient::new().with_api_key("fixture").with_transport(
        move |request: OutboundRequest| {
            assert_ne!(thread::current().id(), executor);
            assert!(request.url.ends_with("/public/v1/chains"));
            Ok(TransportResponse {
                status: 200,
                headers: vec![],
                body: r#"{"chains":[]}"#.to_owned(),
            })
        },
    );
    let raw = client.clone().into_async();
    let typed = client.typed().into_async();
    let request: EmptyRequest = serde_json::from_value(serde_json::json!({})).unwrap();
    let (first, second) = tokio::join!(raw.chains(&serde_json::json!({})), typed.chains(&request));
    assert_eq!(first.unwrap()["chains"], serde_json::json!([]));
    assert!(second.unwrap().chains.is_empty());
}

#[tokio::test(flavor = "current_thread")]
async fn cancelled_sse_read_can_resume_and_drop_disconnects_the_peer() {
    use std::{
        io::{BufRead, BufReader, Read, Write},
        net::TcpListener,
        sync::mpsc,
        time::Duration,
    };
    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    let origin = format!("http://{}", server.local_addr().unwrap());
    let (release, event) = mpsc::channel();
    let peer = thread::spawn(move || {
        let (mut sock, _) = server.accept().unwrap();
        sock.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let mut reader = BufReader::new(sock.try_clone().unwrap());
        let mut line = String::new();
        loop {
            line.clear();
            assert!(reader.read_line(&mut line).unwrap() > 0);
            if line == "\r\n" {
                break;
            }
        }
        sock.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\n\r\n")
            .unwrap();
        event.recv_timeout(Duration::from_secs(5)).unwrap();
        sock.write_all(b"data: {\"price\":12345678901234567891.01}\r\n\r\n")
            .unwrap();
        assert_eq!(sock.read(&mut [0]).unwrap(), 0);
    });
    let stream = aux_rs::PythHermesClient::new()
        .with_origin(origin)
        .into_async()
        .stream_price_updates(&serde_json::json!({"ids":["ab".repeat(32)]}))
        .await
        .unwrap();
    let timeout = Duration::from_millis(25);
    assert!(
        tokio::time::timeout(timeout, stream.next_event())
            .await
            .is_err()
    );
    release.send(()).unwrap();
    let price = tokio::time::timeout(Duration::from_secs(3), stream.next_event())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(price["price"].to_string(), "12345678901234567891.01");
    assert!(
        tokio::time::timeout(timeout, stream.next_bytes())
            .await
            .is_err()
    );
    drop(stream);
    peer.join().unwrap();
}
