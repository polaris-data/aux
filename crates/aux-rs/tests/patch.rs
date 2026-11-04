#![cfg(feature = "http")]
use aux_rs::{HttpMethod, HttpRequest, HttpTransport, StandardTransport, TransportConfig};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
#[tokio::test]
async fn patch_is_a_single_native_http_request() {
    let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = server.local_addr().unwrap();
    let peer = tokio::spawn(async move {
        let (mut socket, _) = server.accept().await.unwrap();
        let mut request = Vec::new();
        loop {
            let mut bytes = [0; 1024];
            let n = socket.read(&mut bytes).await.unwrap();
            assert_ne!(n, 0);
            request.extend(&bytes[..n]);
            if request.ends_with(b"{\"op\":1}") {
                break;
            }
        }
        let text = String::from_utf8(request).unwrap();
        assert!(text.starts_with("PATCH /native?x=1 HTTP/1.1\r\n"));
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}")
            .await
            .unwrap();
    });
    let transport = StandardTransport::new(TransportConfig::default()).unwrap();
    let result = transport
        .execute(HttpRequest {
            method: HttpMethod::Patch,
            url: format!("http://{addr}/native?x=1"),
            headers: vec![("Content-Type".into(), "application/json".into())],
            body: br#"{"op":1}"#.to_vec(),
        })
        .await
        .unwrap();
    assert_eq!(result.status, 200);
    peer.await.unwrap();
    transport.shutdown().await.unwrap();
}
