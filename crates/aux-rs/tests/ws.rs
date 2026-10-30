#![cfg(all(feature = "hyperliquid", feature = "ws"))]

use aux_rs::hyperliquid::HyperliquidClient;
use std::time::Duration;

use aux_rs::{
    Error, TransportConfig, TransportErrorKind,
    hyperliquid::{Event, Hyperliquid, Subscription},
};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::oneshot,
    time::timeout,
};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

const BOOK: &str = include_str!("../../exchanges/hyperliquid/tests/fixtures/book.json");
const TRADES: &str = include_str!("../../exchanges/hyperliquid/tests/fixtures/trades.json");

async fn next_json(socket: &mut WebSocketStream<TcpStream>) -> Value {
    let message = timeout(Duration::from_secs(3), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    serde_json::from_str(message.to_text().unwrap()).unwrap()
}

async fn send(socket: &mut WebSocketStream<TcpStream>, text: impl Into<String>) {
    socket
        .send(Message::Text(text.into().into()))
        .await
        .unwrap();
}

fn network(address: std::net::SocketAddr) -> Hyperliquid {
    Hyperliquid::custom("http://unused", format!("ws://{address}/ws"))
}

fn book_message() -> String {
    format!(r#"{{"channel":"l2Book","data":{BOOK}}}"#)
}

#[tokio::test]
async fn shared_socket_routes_native_events_and_reference_counts_subscriptions() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (ready, start) = oneshot::channel();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
        let first = next_json(&mut socket).await;
        assert_eq!(first["subscription"]["type"], "l2Book");
        send(
            &mut socket,
            json!({"channel":"subscriptionResponse", "data":first}).to_string(),
        )
        .await;
        let second = next_json(&mut socket).await;
        assert_eq!(second["subscription"]["type"], "trades");
        send(
            &mut socket,
            json!({"channel":"subscriptionResponse", "data":second}).to_string(),
        )
        .await;
        start.await.unwrap();
        send(&mut socket, TRADES.replace("ETH", "BTC")).await;
        send(&mut socket, book_message()).await;
        send(&mut socket, TRADES).await;
        // A duplicate book consumer must not send another remote subscribe,
        // nor unsubscribe while another consumer still owns the subscription.
        let remove_trades = next_json(&mut socket).await;
        assert_eq!(
            remove_trades,
            json!({"method":"unsubscribe","subscription":{"type":"trades","coin":"ETH"}})
        );
        let remove_book = next_json(&mut socket).await;
        assert_eq!(
            remove_book,
            json!({"method":"unsubscribe","subscription":{"type":"l2Book","coin":"ETH"}})
        );
        assert!(matches!(
            timeout(Duration::from_secs(3), socket.next())
                .await
                .unwrap(),
            Some(Ok(Message::Close(_)))
        ));
    });
    let client = HyperliquidClient::new(network(address)).unwrap();
    let mut book = client
        .subscribe(Subscription::L2Book { coin: "ETH".into() })
        .await
        .unwrap();
    let mut trades = client
        .clone()
        .subscribe(Subscription::Trades { coin: "ETH".into() })
        .await
        .unwrap();
    let mut duplicate = client
        .clone()
        .subscribe(Subscription::L2Book { coin: "ETH".into() })
        .await
        .unwrap();
    ready.send(()).unwrap();
    for stream in [&mut book, &mut duplicate] {
        timeout(Duration::from_secs(3), async {
            loop {
                if let Event::L2Book(book) = stream.next().await.unwrap().unwrap() {
                    assert_eq!(book.levels[0][0].sz, "1.2300");
                    break;
                }
            }
        })
        .await
        .unwrap();
    }
    timeout(Duration::from_secs(3), async {
        loop {
            if let Event::Trades(trades) = trades.next().await.unwrap().unwrap() {
                assert_eq!(trades[0].coin, "ETH");
                break;
            }
        }
    })
    .await
    .unwrap();
    book.unsubscribe().await.unwrap();
    assert!(book.next().await.is_none());
    trades.close().await.unwrap();
    duplicate.close().await.unwrap();
    timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn overflow_is_terminal_and_cannot_be_hidden_by_buffered_data() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (overflowed, observed) = oneshot::channel();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
        next_json(&mut socket).await;
        send(&mut socket, book_message()).await;
        send(&mut socket, book_message()).await;
        assert_eq!(next_json(&mut socket).await["method"], "unsubscribe");
        overflowed.send(()).unwrap();
        let _ = socket.next().await;
    });
    let client = HyperliquidClient::with_config(
        network(address),
        TransportConfig {
            subscription_capacity: 1,
            ..TransportConfig::default()
        },
    )
    .unwrap();
    let mut stream = client
        .subscribe(Subscription::L2Book { coin: "ETH".into() })
        .await
        .unwrap();
    timeout(Duration::from_secs(3), observed)
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(stream.next().await, Some(Err(Error::Transport(error)))
        if error.kind == TransportErrorKind::Overflow)
    );
    assert!(stream.next().await.is_none());
    timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn disconnect_is_terminal_and_a_new_subscription_can_connect_again() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        for _ in 0..2 {
            let (stream, _) = listener.accept().await.unwrap();
            let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
            next_json(&mut socket).await;
            socket.close(None).await.unwrap();
        }
    });
    let client = HyperliquidClient::with_config(
        network(address),
        TransportConfig {
            reconnect: aux_rs::BackoffConfig::disabled(),
            ..Default::default()
        },
    )
    .unwrap();
    for _ in 0..2 {
        let mut stream = client
            .subscribe(Subscription::Trades { coin: "ETH".into() })
            .await
            .unwrap();
        let event = timeout(Duration::from_secs(3), stream.next())
            .await
            .unwrap();
        assert!(
            matches!(event, Some(Err(Error::Transport(error))) if error.kind == TransportErrorKind::Disconnected)
        );
        assert!(stream.next().await.is_none());
    }
    timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn malformed_frames_and_native_errors_are_exposed_and_drop_cleans_up() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
        next_json(&mut socket).await;
        send(&mut socket, "not JSON").await;
        send(
            &mut socket,
            r#"{"channel":"error","data":"native failure"}"#,
        )
        .await;
        assert_eq!(next_json(&mut socket).await["method"], "unsubscribe");
        assert!(matches!(socket.next().await, Some(Ok(Message::Close(_)))));
    });
    let client = HyperliquidClient::new(network(address)).unwrap();
    let mut stream = client
        .subscribe(Subscription::L2Book { coin: "ETH".into() })
        .await
        .unwrap();
    assert!(matches!(
        timeout(Duration::from_secs(3), stream.next())
            .await
            .unwrap(),
        Some(Err(Error::Exchange(aux_rs::hyperliquid::Error::Decode(_))))
    ));
    assert!(
        matches!(timeout(Duration::from_secs(3), stream.next()).await.unwrap(),
        Some(Err(Error::Exchange(aux_rs::hyperliquid::Error::Api { message }))) if message == "native failure")
    );
    drop(stream);
    timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn native_heartbeats_keep_idle_subscriptions_alive() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
        next_json(&mut socket).await;
        assert_eq!(next_json(&mut socket).await, json!({"method":"ping"}));
        send(&mut socket, r#"{"channel":"pong"}"#).await;
        send(&mut socket, book_message()).await;
        // More heartbeat ticks are allowed while the test consumer is running.
        loop {
            if next_json(&mut socket).await["method"] == "unsubscribe" {
                break;
            }
        }
        let _ = socket.next().await;
    });
    let client = HyperliquidClient::with_config(
        network(address),
        TransportConfig {
            heartbeat_interval: Duration::from_millis(20),
            idle_timeout: Duration::from_secs(1),
            ..TransportConfig::default()
        },
    )
    .unwrap();
    let mut stream = client
        .subscribe(Subscription::L2Book { coin: "ETH".into() })
        .await
        .unwrap();
    assert!(matches!(
        timeout(Duration::from_secs(3), stream.next())
            .await
            .unwrap(),
        Some(Ok(Event::L2Book(_)))
    ));
    stream.close().await.unwrap();
    timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn cancelled_next_does_not_consume_an_event() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (ready, start) = oneshot::channel();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
        next_json(&mut socket).await;
        start.await.unwrap();
        send(&mut socket, book_message()).await;
        assert_eq!(next_json(&mut socket).await["method"], "unsubscribe");
        let _ = socket.next().await;
    });
    let client = HyperliquidClient::new(network(address)).unwrap();
    let mut stream = client
        .subscribe(Subscription::L2Book { coin: "ETH".into() })
        .await
        .unwrap();
    // Poll next once, then cancel while the server is waiting for our signal.
    {
        let next = stream.next();
        tokio::pin!(next);
        let state = std::future::poll_fn(|cx| {
            std::task::Poll::Ready(std::future::Future::poll(next.as_mut(), cx))
        })
        .await;
        assert!(matches!(state, std::task::Poll::Pending));
    }
    ready.send(()).unwrap();
    assert!(matches!(
        timeout(Duration::from_secs(3), stream.next())
            .await
            .unwrap(),
        Some(Ok(Event::L2Book(_)))
    ));
    stream.close().await.unwrap();
    timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn missing_heartbeat_responses_terminate_idle_sessions() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
        next_json(&mut socket).await;
        loop {
            match timeout(Duration::from_secs(3), socket.next())
                .await
                .unwrap()
            {
                Some(Ok(Message::Text(text))) => {
                    assert_eq!(
                        serde_json::from_str::<Value>(&text).unwrap()["method"],
                        "ping"
                    );
                    // Deliberately do not send the native heartbeat pong.
                }
                Some(Ok(Message::Close(_))) => break,
                other => panic!("expected heartbeat or close, got {other:?}"),
            }
        }
    });
    let client = HyperliquidClient::with_config(
        network(address),
        TransportConfig {
            heartbeat_interval: Duration::from_millis(20),
            idle_timeout: Duration::from_millis(80),
            reconnect: aux_rs::BackoffConfig::disabled(),
            ..TransportConfig::default()
        },
    )
    .unwrap();
    let mut stream = client
        .subscribe(Subscription::L2Book { coin: "ETH".into() })
        .await
        .unwrap();
    assert!(
        matches!(timeout(Duration::from_secs(3), stream.next()).await.unwrap(),
        Some(Err(Error::Transport(error))) if error.kind == TransportErrorKind::Timeout)
    );
    assert!(stream.next().await.is_none());
    timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn overflowing_one_consumer_does_not_terminate_other_consumers() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (ready, start) = oneshot::channel();
    let (again, next) = oneshot::channel();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
        next_json(&mut socket).await;
        start.await.unwrap();
        send(&mut socket, book_message()).await;
        next.await.unwrap();
        send(&mut socket, book_message()).await;
        assert_eq!(next_json(&mut socket).await["method"], "unsubscribe");
        let _ = socket.next().await;
    });
    let client = HyperliquidClient::with_config(
        network(address),
        TransportConfig {
            subscription_capacity: 1,
            ..TransportConfig::default()
        },
    )
    .unwrap();
    let mut fast = client
        .subscribe(Subscription::L2Book { coin: "ETH".into() })
        .await
        .unwrap();
    let mut slow = client
        .subscribe(Subscription::L2Book { coin: "ETH".into() })
        .await
        .unwrap();
    ready.send(()).unwrap();
    assert!(matches!(
        timeout(Duration::from_secs(3), fast.next()).await.unwrap(),
        Some(Ok(Event::L2Book(_)))
    ));
    again.send(()).unwrap();
    assert!(matches!(
        timeout(Duration::from_secs(3), fast.next()).await.unwrap(),
        Some(Ok(Event::L2Book(_)))
    ));
    assert!(
        matches!(timeout(Duration::from_secs(3), slow.next()).await.unwrap(),
        Some(Err(Error::Transport(error))) if error.kind == TransportErrorKind::Overflow)
    );
    fast.close().await.unwrap();
    timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn account_streams_without_user_fields_use_isolated_connections() {
    use aux_rs::hyperliquid::Address;
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let mut handlers = Vec::new();
        for _ in 0..2 {
            let (stream, _) = listener.accept().await.unwrap();
            handlers.push(tokio::spawn(async move {
                let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();
                let subscribed = next_json(&mut socket).await;
                let user: Address = subscribed["subscription"]["user"].as_str().unwrap().parse().unwrap();
                let oid = user.as_bytes()[19] as u64;
                send(&mut socket, json!({"channel":"subscriptionResponse","data":subscribed}).to_string()).await;
                send(&mut socket, json!({"channel":"orderUpdates","data":[{
                    "order":{"coin":"ETH","limitPx":"3000","oid":oid,"side":"B","sz":"1","timestamp":1},
                    "status":"open","statusTimestamp":2
                }]}).to_string()).await;
                let unsubscribe = next_json(&mut socket).await;
                assert_eq!(unsubscribe["method"], "unsubscribe");
                assert_eq!(unsubscribe["subscription"]["user"], user.to_string());
                assert!(matches!(timeout(Duration::from_secs(3), socket.next()).await.unwrap(), Some(Ok(Message::Close(_)))));
            }));
        }
        for handler in handlers {
            handler.await.unwrap();
        }
    });
    let client = HyperliquidClient::new(network(address)).unwrap();
    let mut first = client
        .subscribe(Subscription::OrderUpdates {
            user: Address::from_bytes([11; 20]),
        })
        .await
        .unwrap();
    let mut second = client
        .subscribe(Subscription::OrderUpdates {
            user: Address::from_bytes([22; 20]),
        })
        .await
        .unwrap();
    for (stream, expected_oid) in [(&mut first, 11), (&mut second, 22)] {
        timeout(Duration::from_secs(3), async {
            loop {
                if let Event::OrderUpdates(updates) = stream.next().await.unwrap().unwrap() {
                    assert_eq!(updates[0].order.oid, expected_oid);
                    break;
                }
            }
        })
        .await
        .unwrap();
    }
    first.close().await.unwrap();
    second.close().await.unwrap();
    timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}
