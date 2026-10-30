#![cfg(all(feature = "lighter", feature = "ws"))]
use aux_rs::lighter::LighterClient;
use aux_rs::{Error, TransportConfig, TransportErrorKind, lighter::*};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::{sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::oneshot,
    time::timeout,
};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};
use zeroize::Zeroizing;

const BOOK: &str = include_str!("../../exchanges/lighter/tests/fixtures/book_stream.json");
const TRADES: &str = include_str!("../../exchanges/lighter/tests/fixtures/trades_stream.json");
async fn send(socket: &mut WebSocketStream<TcpStream>, text: impl Into<String>) {
    socket
        .send(Message::Text(text.into().into()))
        .await
        .unwrap();
}
async fn next(socket: &mut WebSocketStream<TcpStream>) -> Value {
    let message = timeout(Duration::from_secs(3), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    serde_json::from_str(message.to_text().unwrap()).unwrap()
}

#[tokio::test]
async fn greeting_multiplexing_json_pong_and_cleanup() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (ready, start) = oneshot::channel();
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(tcp).await.unwrap();
        send(&mut socket, r#"{"type":"connected","session_id":"test"}"#).await;
        assert_eq!(
            next(&mut socket).await,
            json!({"type":"subscribe","channel":"order_book/4099"})
        );
        assert_eq!(
            next(&mut socket).await,
            json!({"type":"subscribe","channel":"trade/4099"})
        );
        start.await.unwrap();
        send(&mut socket, r#"{"type":"ping"}"#).await;
        assert_eq!(next(&mut socket).await, json!({"type":"pong"}));
        send(&mut socket, TRADES).await;
        send(&mut socket, BOOK).await;
        // No duplicate pong or duplicate subscribe may appear before these.
        assert_eq!(
            next(&mut socket).await,
            json!({"type":"unsubscribe","channel":"trade/4099"})
        );
        assert_eq!(
            next(&mut socket).await,
            json!({"type":"unsubscribe","channel":"order_book/4099"})
        );
        assert!(matches!(
            timeout(Duration::from_secs(3), socket.next())
                .await
                .unwrap(),
            Some(Ok(Message::Close(_)))
        ));
    });
    let client = LighterClient::new(Lighter::custom(
        "http://unused",
        format!("ws://{address}/stream"),
        300,
    ))
    .unwrap();
    let mut book = client
        .subscribe(Subscription::OrderBook { market_id: 4099 })
        .await
        .unwrap();
    let mut trades = client
        .subscribe(Subscription::Trades { market_id: 4099 })
        .await
        .unwrap();
    ready.send(()).unwrap();
    assert!(matches!(
        timeout(Duration::from_secs(3), book.next()).await.unwrap(),
        Some(Ok(Event::OrderBook(_)))
    ));
    assert!(matches!(trades.next().await, Some(Ok(Event::Trades(_)))));
    trades.unsubscribe().await.unwrap();
    book.unsubscribe().await.unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn late_book_consumer_gets_its_own_snapshot_without_resetting_existing_stream() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let mut first = tokio_tungstenite::accept_async(tcp).await.unwrap();
        next(&mut first).await;
        send(&mut first, BOOK).await;
        let (tcp, _) = timeout(Duration::from_secs(3), listener.accept())
            .await
            .unwrap()
            .unwrap();
        let mut second = tokio_tungstenite::accept_async(tcp).await.unwrap();
        next(&mut second).await;
        let mut fresh: Value = serde_json::from_str(BOOK).unwrap();
        fresh["order_book"]["nonce"] = 3650252.into();
        send(&mut second, fresh.to_string()).await;
        assert_eq!(
            next(&mut second).await,
            json!({"type":"unsubscribe","channel":"order_book/4099"})
        );
        let _ = second.next().await;
        fresh["type"] = "update/order_book".into();
        fresh["order_book"]["begin_nonce"] = 3650251.into();
        send(&mut first, fresh.to_string()).await;
        assert_eq!(
            next(&mut first).await,
            json!({"type":"unsubscribe","channel":"order_book/4099"})
        );
        let _ = first.next().await;
    });
    let client = LighterClient::new(Lighter::custom(
        "http://unused",
        format!("ws://{address}"),
        300,
    ))
    .unwrap();
    let mut first = client
        .subscribe(Subscription::OrderBook { market_id: 4099 })
        .await
        .unwrap();
    assert!(first.next().await.unwrap().unwrap().is_snapshot());
    let mut late = client
        .subscribe(Subscription::OrderBook { market_id: 4099 })
        .await
        .unwrap();
    match late.next().await.unwrap().unwrap() {
        Event::OrderBook(book) => {
            assert!(book.event_type.starts_with("subscribed/"));
            assert_eq!(book.order_book.nonce, 3650252);
        }
        _ => panic!(),
    }
    late.unsubscribe().await.unwrap();
    let delta = first.next().await.unwrap().unwrap();
    assert!(!delta.is_snapshot());
    first.unsubscribe().await.unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn protocol_errors_overflow_and_disconnect_are_explicit() {
    for mode in 0..3 {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let (go, start) = oneshot::channel();
        let (done, finished) = oneshot::channel();
        let server = tokio::spawn(async move {
            let (tcp, _) = listener.accept().await.unwrap();
            let mut socket = tokio_tungstenite::accept_async(tcp).await.unwrap();
            next(&mut socket).await;
            start.await.unwrap();
            if mode == 0 {
                send(&mut socket, "{").await;
                send(
                    &mut socket,
                    r#"{"error":{"code":20001,"message":"echo-token"}}"#,
                )
                .await;
            }
            if mode == 1 {
                for _ in 0..10 {
                    send(&mut socket, BOOK).await;
                }
            }
            if mode == 2 {
                socket.close(None).await.unwrap();
            }
            done.send(()).unwrap();
            let _ = timeout(Duration::from_secs(3), socket.next()).await;
        });
        let config = TransportConfig {
            reconnect: aux_rs::BackoffConfig::disabled(),
            subscription_capacity: if mode == 1 { 1 } else { 16 },
            ..TransportConfig::default()
        };
        let client = LighterClient::with_config(
            Lighter::custom("http://unused", format!("ws://{address}"), 300),
            config,
        )
        .unwrap();
        let mut stream = client
            .subscribe(Subscription::OrderBook { market_id: 4099 })
            .await
            .unwrap();
        go.send(()).unwrap();
        finished.await.unwrap();
        if mode == 0 {
            assert!(matches!(
                stream.next().await,
                Some(Err(Error::Exchange(aux_rs::lighter::Error::Decode(_))))
            ));
            let error = stream.next().await.unwrap().unwrap_err();
            assert!(!format!("{error:?}").contains("echo-token"));
            assert!(matches!(
                error,
                Error::Exchange(aux_rs::lighter::Error::Api { code: 20001, .. })
            ));
        } else {
            // Let the driver process queued frames before checking the latched terminal state.
            tokio::time::sleep(Duration::from_millis(30)).await;
            let error = stream.next().await.unwrap().unwrap_err();
            match error {
                Error::Transport(e) => assert_eq!(
                    e.kind,
                    if mode == 1 {
                        TransportErrorKind::Overflow
                    } else {
                        TransportErrorKind::Disconnected
                    }
                ),
                _ => panic!(),
            }
        }
        let _ = stream.unsubscribe().await;
        server.await.unwrap();
    }
}

#[tokio::test]
async fn private_order_stream_has_native_auth_and_isolated_scope() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(tcp).await.unwrap();
        let message = next(&mut socket).await;
        assert_eq!(message["channel"], "account_all_orders/37");
        assert_eq!(message["auth"].as_str().unwrap().split(':').count(), 4);
        send(&mut socket,r#"{"type":"subscribed/account_all_orders","channel":"account_all_orders:37","orders":{}}"#).await;
        assert_eq!(
            next(&mut socket).await,
            json!({"type":"unsubscribe","channel":"account_all_orders/37"})
        );
        let _ = socket.next().await;
    });
    let mut key = [0; 40];
    key[0] = 1;
    let signer = Arc::new(LocalSigner::from_bytes(Zeroizing::new(key)).unwrap());
    let auth = LighterAuth::new(signer, 300, 37, 3).unwrap();
    let client = LighterClient::new(Lighter::custom(
        "http://unused",
        format!("ws://{address}"),
        300,
    ))
    .unwrap()
    .with_auth(auth);
    let mut stream = client.subscribe_account_orders().await.unwrap();
    assert!(matches!(
        stream.next().await,
        Some(Ok(Event::AccountOrders(_)))
    ));
    stream.unsubscribe().await.unwrap();
    server.await.unwrap();
}
