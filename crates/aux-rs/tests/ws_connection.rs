#![cfg(feature = "ws")]
use aux_rs::{
    StandardTransport, TransportConfig, TransportErrorKind, WebSocketConnection, WebSocketConnector,
};
use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::TcpListener,
    time::{Duration, timeout},
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[tokio::test]
async fn isolated_connection_roundtrips_and_closes_without_pooling() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let mut s = accept_async(tcp).await.unwrap();
        assert_eq!(
            s.next().await.unwrap().unwrap(),
            Message::Text("command".into())
        );
        s.send(Message::Ping(vec![1, 2, 3].into())).await.unwrap();
        s.send(Message::Text("reply".into())).await.unwrap();
        while let Some(frame) = s.next().await {
            if matches!(frame, Ok(Message::Close(_))) {
                break;
            }
        }
    });
    let transport = StandardTransport::new(TransportConfig::default()).unwrap();
    let mut c = transport.connect(&format!("ws://{address}")).await.unwrap();
    c.send("command".into()).await.unwrap();
    assert_eq!(
        c.next_with_timeout(Duration::from_secs(1))
            .await
            .unwrap()
            .unwrap(),
        "reply"
    );
    c.close().await.unwrap();
    timeout(Duration::from_secs(1), transport.shutdown())
        .await
        .unwrap()
        .unwrap();
    server.await.unwrap();
}
#[tokio::test]
async fn shutdown_interrupts_live_connections_and_rejects_new_connections() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let mut s = accept_async(tcp).await.unwrap();
        while let Some(frame) = s.next().await {
            if matches!(frame, Ok(Message::Close(_))) {
                break;
            }
        }
    });
    let transport = StandardTransport::new(TransportConfig::default()).unwrap();
    let mut c = transport.connect(&format!("ws://{address}")).await.unwrap();
    timeout(Duration::from_secs(1), transport.shutdown())
        .await
        .unwrap()
        .unwrap();
    let e = c.next().await.unwrap().unwrap_err();
    assert_eq!(e.kind, TransportErrorKind::Closed);
    assert!(transport.connect(&format!("ws://{address}")).await.is_err());
    server.await.unwrap();
}
#[tokio::test]
async fn response_timeout_preserves_later_data() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (go, wait) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let mut s = accept_async(tcp).await.unwrap();
        wait.await.unwrap();
        s.send(Message::Text("later".into())).await.unwrap();
        while let Some(frame) = s.next().await {
            if matches!(frame, Ok(Message::Close(_))) {
                break;
            }
        }
    });
    let transport = StandardTransport::new(TransportConfig::default()).unwrap();
    let mut c = transport.connect(&format!("ws://{address}")).await.unwrap();
    let e = c
        .next_with_timeout(Duration::from_millis(10))
        .await
        .unwrap()
        .unwrap_err();
    assert_eq!(e.kind, TransportErrorKind::Timeout);
    go.send(()).unwrap();
    assert_eq!(
        c.next_with_timeout(Duration::from_secs(1))
            .await
            .unwrap()
            .unwrap(),
        "later"
    );
    c.close().await.unwrap();
    transport.shutdown().await.unwrap();
    server.await.unwrap();
}
#[tokio::test]
async fn disconnected_command_is_never_replayed() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let mut s = accept_async(tcp).await.unwrap();
        assert_eq!(
            s.next().await.unwrap().unwrap(),
            Message::Text("order".into())
        );
        s.close(None).await.unwrap();
        assert!(
            timeout(Duration::from_millis(100), listener.accept())
                .await
                .is_err()
        );
    });
    let transport = StandardTransport::new(TransportConfig::default()).unwrap();
    let mut c = transport.connect(&format!("ws://{address}")).await.unwrap();
    c.send("order".into()).await.unwrap();
    assert!(
        c.next_with_timeout(Duration::from_secs(1))
            .await
            .unwrap()
            .is_err()
    );
    transport.shutdown().await.unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn read_connections_backpressure_and_drain_clean_replay_close() {
    use tokio_tungstenite::tungstenite::protocol::{CloseFrame, frame::coding::CloseCode};
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let mut socket = accept_async(tcp).await.unwrap();
        for i in 0..1000 {
            socket
                .send(Message::Text(i.to_string().into()))
                .await
                .unwrap();
        }
        socket
            .close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: "replay complete".into(),
            }))
            .await
            .unwrap();
    });
    let transport = StandardTransport::new(TransportConfig {
        subscription_capacity: 2,
        ..TransportConfig::default()
    })
    .unwrap();
    let mut connection = transport
        .connect_read(&format!("ws://{address}"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(20)).await;
    for i in 0..1000 {
        assert_eq!(connection.next().await.unwrap().unwrap(), i.to_string());
    }
    assert!(connection.next().await.is_none());
    connection.close().await.unwrap();
    transport.shutdown().await.unwrap();
    server.await.unwrap();
}
