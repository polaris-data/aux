#![cfg(feature = "ws")]
use aux_rs_core::{StandardTransport, TransportConfig};
use aux_rs_phoenix::*;
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio::{
    net::TcpListener,
    time::{Duration, timeout},
};
use tokio_tungstenite::{accept_async, tungstenite::Message};
#[tokio::test]
async fn reconnect_reports_gap_before_fresh_snapshot() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (send_gap, wait_gap) = tokio::sync::oneshot::channel::<()>();
    let server = tokio::spawn(async move {
        let mut wait_gap = Some(wait_gap);
        for attempt in 0..2 {
            let (socket, _) = listener.accept().await.unwrap();
            let mut socket = accept_async(socket).await.unwrap();
            let frame = socket.next().await.unwrap().unwrap();
            let frame: Value = serde_json::from_str(frame.to_text().unwrap()).unwrap();
            assert_eq!(frame["type"], "subscribe");
            let base = if attempt == 0 { 10 } else { 100 };
            socket.send(Message::Text(json!({"channel":"exchange","messageType":"snapshot","sequenceNumber":base,"markets":[]}).to_string().into())).await.unwrap();
            socket.send(Message::Text(json!({"channel":"exchange","messageType":"delta","sequenceNumber":base+1,"ops":[]}).to_string().into())).await.unwrap();
            if attempt == 0 {
                wait_gap.take().unwrap().await.unwrap();
                socket.send(Message::Text(json!({"channel":"exchange","messageType":"delta","sequenceNumber":base+3,"ops":[]}).to_string().into())).await.unwrap();
            } else {
                while socket.next().await.is_some() {}
            }
        }
    });
    let mut config = TransportConfig::default();
    config.reconnect.initial_delay = Duration::from_millis(1);
    config.reconnect.max_delay = Duration::from_millis(2);
    let transport = StandardTransport::new(config).unwrap();
    let exchange =
        Phoenix::custom(format!("http://{addr}"), format!("ws://{addr}"), "fixture").unwrap();
    let client = PhoenixClient::with_transport(exchange, transport.clone());
    let spec: Subscription =
        serde_json::from_value(json!({"channel":"exchange","encoding":"json"})).unwrap();
    let mut stream = client.subscribe(spec).await.unwrap();
    let first = timeout(Duration::from_secs(3), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(first.0["sequenceNumber"], 10);
    let second = stream.next().await.unwrap().unwrap();
    assert_eq!(second.0["sequenceNumber"], 11);
    send_gap.send(()).unwrap();
    let gap = stream.next().await.unwrap();
    assert!(matches!(gap,Err(aux_rs_core::Error::Transport(e)) if e.is_gap()));
    let fresh = timeout(Duration::from_secs(3), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(fresh.0["sequenceNumber"], 100);
    stream.close().await.unwrap();
    transport.shutdown().await.unwrap();
    timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}
