#![cfg(all(feature = "lighter", feature = "http"))]
use aux_rs::lighter::LighterClient;
use aux_rs::lighter::*;
use serde_json::{Value, json};
use std::{sync::Arc, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::timeout,
};
use zeroize::Zeroizing;

async fn request(socket: &mut TcpStream) -> (String, Vec<u8>) {
    let mut bytes = Vec::new();
    let end = loop {
        if let Some(i) = bytes.windows(4).position(|b| b == b"\r\n\r\n") {
            break i + 4;
        }
        let mut b = [0; 1024];
        let n = timeout(Duration::from_secs(3), socket.read(&mut b))
            .await
            .unwrap()
            .unwrap();
        assert!(n > 0);
        bytes.extend_from_slice(&b[..n]);
    };
    let headers = String::from_utf8(bytes[..end].to_vec()).unwrap();
    let length = headers
        .lines()
        .find_map(|l| {
            let (k, v) = l.split_once(':')?;
            k.eq_ignore_ascii_case("content-length")
                .then(|| v.trim().parse::<usize>().unwrap())
        })
        .unwrap_or(0);
    while bytes.len() < end + length {
        let mut b = [0; 1024];
        let n = socket.read(&mut b).await.unwrap();
        assert!(n > 0);
        bytes.extend_from_slice(&b[..n]);
    }
    (headers, bytes[end..end + length].to_vec())
}
async fn reply(socket: &mut TcpStream, body: &str) {
    socket.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
}

#[tokio::test]
async fn pooled_lighter_http_preserves_paths_form_body_and_auth_header() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let (headers, _) = request(&mut socket).await;
        assert!(headers.starts_with("GET /proxy/api/v1/orderBooks "));
        reply(
            &mut socket,
            include_str!("../crates/exchanges/lighter/tests/fixtures/markets.json"),
        )
        .await;
        let (headers, _) = request(&mut socket).await;
        assert!(headers.starts_with(
            "GET /proxy/api/v1/accountActiveOrders?account_index=5000000&market_id=4096 "
        ));
        let header = headers
            .lines()
            .find(|l| l.to_ascii_lowercase().starts_with("authorization:"))
            .unwrap();
        assert_eq!(header.split(':').count(), 5);
        reply(&mut socket, r#"{"code":200,"orders":[]}"#).await;
        let (headers, _) = request(&mut socket).await;
        assert!(
            headers
                .starts_with("GET /proxy/api/v1/nextNonce?account_index=5000000&api_key_index=3 ")
        );
        reply(&mut socket, r#"{"code":200,"nonce":42}"#).await;
        let (headers, body) = request(&mut socket).await;
        assert!(headers.starts_with("POST /proxy/api/v1/sendTx "));
        assert!(
            headers
                .to_ascii_lowercase()
                .contains("content-type: application/x-www-form-urlencoded")
        );
        let body = String::from_utf8(body).unwrap();
        assert!(body.starts_with("tx_type=15&tx_info=%7B"));
        assert!(body.contains("%22Nonce%22%3A42"));
        assert!(body.contains("%22L2TxAttributes%22%3Anull"));
        assert!(!body.contains("authorization"));
        reply(&mut socket,&json!({"code":200,"tx_hash":"native-hash","predicted_execution_time_ms":1,"volume_quota_remaining":77}).to_string()).await;
    });
    let mut key = [0; 40];
    key[0] = 1;
    let auth = LighterAuth::new(
        Arc::new(LocalSigner::from_bytes(Zeroizing::new(key)).unwrap()),
        300,
        5000000,
        3,
    )
    .unwrap();
    let client = LighterClient::new(Lighter::custom(
        format!("http://{address}/proxy"),
        "ws://unused",
        300,
    ))
    .unwrap()
    .with_auth(auth);
    assert_eq!(
        client
            .order_books(OrderBooksRequest::default())
            .await
            .unwrap()
            .order_books[0]
            .market_id,
        4099
    );
    assert!(
        client
            .active_orders(Some(4096))
            .await
            .unwrap()
            .orders
            .is_empty()
    );
    let response = client
        .cancel_order(
            CancelOrder {
                market_index: 4096,
                index: 123,
            },
            TxOptions::new(1800000000000),
        )
        .await
        .unwrap();
    assert_eq!(response.tx_hash, "native-hash");
    assert_eq!(response.extra["volume_quota_remaining"], Value::from(77));
    server.await.unwrap();
}
