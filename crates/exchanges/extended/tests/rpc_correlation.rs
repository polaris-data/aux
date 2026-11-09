#![cfg(feature = "ws")]
use aux_rs_core::{StandardTransport, TransportConfig};
use aux_rs_extended::*;
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio::{
    net::TcpListener,
    time::{Duration, timeout},
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[tokio::test]
async fn rpc_correlates_replies_topics_and_native_heartbeats() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut socket = accept_async(socket).await.unwrap();
        let request: Value =
            serde_json::from_str(socket.next().await.unwrap().unwrap().to_text().unwrap()).unwrap();
        assert_eq!(request["method"], "subscribe");
        for frame in [
            json!({"jsonrpc":"2.0","id":"wrong-id","result":{"wrong":true}}),
            json!({"subscription":"orderbooks.OTHER","seq":99,"type":"ORDERBOOKS.SNAPSHOT","data":{}}),
            json!({"jsonrpc":"2.0","id":request["id"],"result":{}}),
            json!({"subscription":"orderbooks.BTC-USD","seq":1,"type":"ORDERBOOKS.SNAPSHOT","data":{"m":"BTC-USD","b":[],"a":[]}}),
        ] {
            socket
                .send(Message::Text(frame.to_string().into()))
                .await
                .unwrap();
        }
        while let Some(frame) = socket.next().await {
            match frame.unwrap() {
                Message::Text(text) => {
                    let frame: Value = serde_json::from_str(&text).unwrap();
                    if frame["method"] == "ping" {
                        socket
                            .send(Message::Text(
                                json!({"jsonrpc":"2.0","id":frame["id"],"result":{"pong":true}})
                                    .to_string()
                                    .into(),
                            ))
                            .await
                            .unwrap();
                    } else if frame["method"] == "unsubscribe" {
                        break;
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });
    let config = TransportConfig {
        heartbeat_interval: Duration::from_millis(20),
        idle_timeout: Duration::from_secs(2),
        ..Default::default()
    };
    let transport = StandardTransport::new(config).unwrap();
    let exchange =
        Extended::custom(format!("http://{addr}"), format!("ws://{addr}"), "fixture").unwrap();
    let client = ExtendedClient::with_transport(exchange, transport.clone());
    let mut stream = client
        .subscribe(Subscription {
            channel: "orderbooks".into(),
            market: Some("BTC-USD".into()),
            rpc: true,
            params: json!({}),
        })
        .await
        .unwrap();
    let ack = timeout(Duration::from_secs(3), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert!(ack.0["result"].is_object());
    let snapshot = timeout(Duration::from_secs(3), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(snapshot.0["seq"], 1);
    let pong = timeout(Duration::from_secs(3), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(pong.0["result"]["pong"], true);
    stream.close().await.unwrap();
    transport.shutdown().await.unwrap();
    timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}
