#![cfg(all(feature = "ws", feature = "hyperliquid", feature = "lighter"))]
use aux_rs::{hyperliquid as hl, lighter as lt, *};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::oneshot,
    time::{Instant, timeout},
};
use tokio_tungstenite::{WebSocketStream as Socket, tungstenite::Message};
const BOOK: &str = include_str!("../crates/exchanges/hyperliquid/tests/fixtures/book.json");
const LIGHTER_BOOK: &str =
    include_str!("../crates/exchanges/lighter/tests/fixtures/book_stream.json");
fn config() -> TransportConfig {
    TransportConfig {
        timeout: Duration::from_millis(500),
        reconnect: BackoffConfig {
            max_retries: 2,
            initial_delay: Duration::from_millis(15),
            max_delay: Duration::from_millis(30),
        },
        ..Default::default()
    }
}
async fn socket(listener: &TcpListener) -> Socket<TcpStream> {
    let (tcp, _) = timeout(Duration::from_secs(2), listener.accept())
        .await
        .unwrap()
        .unwrap();
    tokio_tungstenite::accept_async(tcp).await.unwrap()
}
async fn next(socket: &mut Socket<TcpStream>) -> Value {
    let msg = timeout(Duration::from_secs(2), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    serde_json::from_str(msg.to_text().unwrap()).unwrap()
}
async fn send(socket: &mut Socket<TcpStream>, value: Value) {
    socket
        .send(Message::Text(value.to_string().into()))
        .await
        .unwrap();
}
fn book(time: u64) -> Value {
    let mut data: Value = serde_json::from_str(BOOK).unwrap();
    data["time"] = time.into();
    json!({"channel":"l2Book","data":data})
}
fn hl_exchange(addr: std::net::SocketAddr) -> hl::Hyperliquid {
    hl::Hyperliquid::custom("http://unused", format!("ws://{addr}"))
}

#[tokio::test]
async fn reconnect_reports_gap_clears_old_frames_and_resubscribes_each_shared_key_once() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (ready, recovered) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut first = socket(&listener).await;
        for _ in 0..2 {
            next(&mut first).await;
        }
        send(&mut first, book(1)).await;
        first.close(None).await.unwrap();
        let mut second = socket(&listener).await;
        let mut types = Vec::new();
        for _ in 0..2 {
            types.push(
                next(&mut second).await["subscription"]["type"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            );
        }
        types.sort();
        assert_eq!(types, ["l2Book", "trades"]);
        send(&mut second, book(2)).await;
        send(&mut second, json!({"channel":"trades","data":[]})).await;
        ready.send(()).unwrap();
        assert_eq!(next(&mut second).await["subscription"]["type"], "trades");
        assert_eq!(next(&mut second).await["subscription"]["type"], "l2Book");
        let _ = second.next().await;
    });
    let p = hl::HyperliquidClient::with_config(hl_exchange(addr), config()).unwrap();
    let mut books = p
        .subscribe(hl::Subscription::L2Book { coin: "ETH".into() })
        .await
        .unwrap();
    let mut duplicate = p
        .subscribe(hl::Subscription::L2Book { coin: "ETH".into() })
        .await
        .unwrap();
    let mut trades = p
        .subscribe(hl::Subscription::Trades { coin: "ETH".into() })
        .await
        .unwrap();
    recovered.await.unwrap();
    for stream in [&mut books, &mut duplicate, &mut trades] {
        assert!(matches!(stream.next().await,Some(Err(Error::Transport(e))) if e.is_gap()));
    }
    assert!(matches!(books.next().await,Some(Ok(hl::Event::L2Book(v))) if v.time==2));
    assert!(matches!(duplicate.next().await,Some(Ok(hl::Event::L2Book(v))) if v.time==2));
    assert!(matches!(
        trades.next().await,
        Some(Ok(hl::Event::Trades(_)))
    ));
    duplicate.unsubscribe().await.unwrap();
    trades.unsubscribe().await.unwrap();
    books.unsubscribe().await.unwrap();
    server.await.unwrap();
    p.transport().shutdown().await.unwrap();
}

#[tokio::test]
async fn lighter_recovery_requires_a_native_snapshot_before_deltas() {
    for invalid in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (ready, recovered) = oneshot::channel();
        let server = tokio::spawn(async move {
            let mut first = socket(&listener).await;
            next(&mut first).await;
            send(&mut first, serde_json::from_str(LIGHTER_BOOK).unwrap()).await;
            first.close(None).await.unwrap();
            let mut second = socket(&listener).await;
            assert_eq!(
                next(&mut second).await,
                json!({"type":"subscribe","channel":"order_book/4099"})
            );
            let mut value: Value = serde_json::from_str(LIGHTER_BOOK).unwrap();
            value["order_book"]["nonce"] = 100.into();
            if invalid {
                value["type"] = "update/order_book".into();
            }
            send(&mut second, value.clone()).await;
            ready.send(()).unwrap();
            if !invalid {
                value["type"] = "update/order_book".into();
                value["order_book"]["begin_nonce"] = 100.into();
                value["order_book"]["nonce"] = 101.into();
                value["order_book"]["bids"][0]["size"] = "0.0000".into();
                send(&mut second, value).await;
            }
            assert_eq!(
                next(&mut second).await,
                json!({"type":"unsubscribe","channel":"order_book/4099"})
            );
            let _ = second.next().await;
        });
        let p = lt::LighterClient::with_config(
            lt::Lighter::custom("http://unused", format!("ws://{addr}"), 300),
            config(),
        )
        .unwrap();
        let mut stream = p
            .subscribe(lt::Subscription::OrderBook { market_id: 4099 })
            .await
            .unwrap();
        recovered.await.unwrap();
        if invalid {
            // A terminal protocol fault can supersede the not-yet-consumed gap.
            let mut result = stream.next().await;
            if matches!(&result,Some(Err(Error::Transport(e))) if e.is_gap()) {
                result = stream.next().await;
            }
            assert!(
                matches!(result,Some(Err(Error::Transport(e))) if e.kind==TransportErrorKind::Protocol)
            );
            assert!(stream.next().await.is_none());
        } else {
            assert!(matches!(stream.next().await,Some(Err(Error::Transport(e))) if e.is_gap()));
            assert!(
                matches!(stream.next().await,Some(Ok(lt::Event::OrderBook(v))) if v.event_type=="subscribed/order_book"&&v.order_book.nonce==100)
            );
            assert!(
                matches!(stream.next().await,Some(Ok(lt::Event::OrderBook(v))) if v.order_book.nonce==101&&v.order_book.bids[0].size=="0.0000")
            );
        }
        stream.unsubscribe().await.unwrap();
        server.await.unwrap();
        p.transport().shutdown().await.unwrap();
    }
}

#[tokio::test]
async fn reconnect_budget_does_not_reset_for_handshakes_or_acknowledgements() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        for _ in 0..3 {
            let mut s = socket(&listener).await;
            let subscription = next(&mut s).await;
            send(
                &mut s,
                json!({"channel":"subscriptionResponse","data":subscription}),
            )
            .await;
            s.close(None).await.unwrap();
        }
        assert!(
            timeout(Duration::from_millis(100), listener.accept())
                .await
                .is_err()
        );
    });
    let p = hl::HyperliquidClient::with_config(hl_exchange(addr), config()).unwrap();
    let mut stream = p
        .subscribe(hl::Subscription::Trades { coin: "ETH".into() })
        .await
        .unwrap();
    let mut gaps = 0;
    loop {
        let event = timeout(Duration::from_secs(2), stream.next())
            .await
            .unwrap();
        match event {
            Some(Err(Error::Transport(e))) if e.is_gap() => gaps += 1,
            Some(Err(Error::Transport(e))) => {
                assert_eq!(e.kind, TransportErrorKind::Disconnected);
                break;
            }
            Some(Ok(hl::Event::SubscriptionResponse(_))) => {}
            other => panic!("unexpected {other:?}"),
        }
    }
    assert!(gaps > 0);
    assert!(stream.next().await.is_none());
    server.await.unwrap();
    p.transport().shutdown().await.unwrap();
}

#[tokio::test]
async fn unsubscribe_and_drop_during_backoff_stop_recovery_without_another_dial() {
    for drop_only in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let mut s = socket(&listener).await;
            next(&mut s).await;
            s.close(None).await.unwrap();
            assert!(
                timeout(Duration::from_millis(150), listener.accept())
                    .await
                    .is_err()
            );
        });
        let mut cfg = config();
        cfg.reconnect.initial_delay = Duration::from_secs(2);
        cfg.reconnect.max_delay = Duration::from_secs(2);
        let p = hl::HyperliquidClient::with_config(hl_exchange(addr), cfg).unwrap();
        let mut stream = p
            .subscribe(hl::Subscription::Trades { coin: "ETH".into() })
            .await
            .unwrap();
        assert!(matches!(stream.next().await,Some(Err(Error::Transport(e))) if e.is_gap()));
        let began = Instant::now();
        if !drop_only {
            stream.unsubscribe().await.unwrap();
        }
        drop(stream);
        timeout(Duration::from_millis(200), p.transport().shutdown())
            .await
            .unwrap()
            .unwrap();
        assert!(began.elapsed() < Duration::from_millis(200));
        server.await.unwrap();
    }
}

#[tokio::test]
async fn shutdown_cancels_a_reconnect_handshake_and_all_clones_stay_closed() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (seen, dial_started) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut s = socket(&listener).await;
        next(&mut s).await;
        s.close(None).await.unwrap();
        let (mut tcp, _) = listener.accept().await.unwrap();
        seen.send(()).unwrap();
        let mut bytes = [0; 1024];
        use tokio::io::AsyncReadExt;
        while tcp.read(&mut bytes).await.unwrap_or(0) > 0 {}
        assert!(
            timeout(Duration::from_millis(80), listener.accept())
                .await
                .is_err()
        );
    });
    let p = hl::HyperliquidClient::with_config(hl_exchange(addr), config()).unwrap();
    let mut stream = p
        .subscribe(hl::Subscription::Trades { coin: "ETH".into() })
        .await
        .unwrap();
    dial_started.await.unwrap();
    timeout(Duration::from_millis(200), p.transport().shutdown())
        .await
        .unwrap()
        .unwrap();
    let result = stream.next().await;
    assert!(matches!(result,Some(Err(Error::Transport(e))) if e.kind==TransportErrorKind::Closed));
    assert!(
        p.clone()
            .subscribe(hl::Subscription::Trades { coin: "ETH".into() })
            .await
            .is_err()
    );
    stream.unsubscribe().await.unwrap();
    server.await.unwrap();
}

#[derive(Clone)]
struct Expiring {
    valid: Arc<AtomicBool>,
}
impl WebSocketSubscription for Expiring {
    fn resubscribe_allowed(&self) -> bool {
        self.valid.load(Ordering::SeqCst)
    }
    fn key(&self) -> String {
        "private".into()
    }
    fn subscribe_message(&self) -> String {
        r#"{"type":"subscribe"}"#.into()
    }
    fn unsubscribe_message(&self) -> String {
        r#"{"type":"unsubscribe"}"#.into()
    }
    fn matches(&self, _: &str) -> bool {
        true
    }
}
#[tokio::test]
async fn expired_credentials_are_never_replayed_during_recovery() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let valid = Arc::new(AtomicBool::new(true));
    let state = valid.clone();
    let server = tokio::spawn(async move {
        let mut s = socket(&listener).await;
        next(&mut s).await;
        state.store(false, Ordering::SeqCst);
        s.close(None).await.unwrap();
        assert!(
            timeout(Duration::from_millis(100), listener.accept())
                .await
                .is_err()
        );
    });
    let transport = StandardTransport::new(config()).unwrap();
    let mut stream = transport
        .subscribe(&format!("ws://{addr}"), Expiring { valid })
        .await
        .unwrap();
    let mut result = stream.next().await;
    if matches!(&result,Some(Err(e)) if e.is_gap()) {
        result = stream.next().await;
    }
    assert!(matches!(result,Some(Err(e)) if e.kind==TransportErrorKind::Authentication));
    assert!(stream.next().await.is_none());
    stream.close().await.unwrap();
    transport.shutdown().await.unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn binary_and_oversized_frames_are_terminal_without_recovery() {
    for oversized in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let mut s = socket(&listener).await;
            next(&mut s).await;
            s.send(if oversized {
                Message::Text("x".repeat(512).into())
            } else {
                Message::Binary(vec![1, 2, 3].into())
            })
            .await
            .unwrap();
            let _ = s.next().await;
            assert!(
                timeout(Duration::from_millis(80), listener.accept())
                    .await
                    .is_err()
            );
        });
        let p = hl::HyperliquidClient::with_config(
            hl_exchange(addr),
            TransportConfig {
                max_message_bytes: 128,
                ..config()
            },
        )
        .unwrap();
        let mut stream = p
            .subscribe(hl::Subscription::Trades { coin: "ETH".into() })
            .await
            .unwrap();
        assert!(
            matches!(stream.next().await,Some(Err(Error::Transport(e))) if e.kind==if oversized{TransportErrorKind::ResponseTooLarge}else{TransportErrorKind::Protocol})
        );
        assert!(stream.next().await.is_none());
        stream.unsubscribe().await.unwrap();
        server.await.unwrap();
        p.transport().shutdown().await.unwrap();
    }
}

#[tokio::test]
async fn a_quiet_stable_connection_resets_the_reconnect_budget() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (ready, recovered) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut first = socket(&listener).await;
        next(&mut first).await;
        first.close(None).await.unwrap();
        let mut quiet = socket(&listener).await;
        next(&mut quiet).await;
        tokio::time::sleep(Duration::from_millis(40)).await;
        quiet.close(None).await.unwrap();
        let mut third = socket(&listener).await;
        next(&mut third).await;
        send(&mut third, book(3)).await;
        ready.send(()).unwrap();
        next(&mut third).await;
        let _ = third.next().await;
    });
    let mut cfg = config();
    cfg.reconnect.max_retries = 1;
    cfg.reconnect_stable_after = Duration::from_millis(20);
    let p = hl::HyperliquidClient::with_config(hl_exchange(addr), cfg).unwrap();
    let mut stream = p
        .subscribe(hl::Subscription::L2Book { coin: "ETH".into() })
        .await
        .unwrap();
    recovered.await.unwrap();
    assert!(matches!(stream.next().await,Some(Err(Error::Transport(e))) if e.is_gap()));
    assert!(matches!(stream.next().await,Some(Ok(hl::Event::L2Book(v))) if v.time==3));
    stream.unsubscribe().await.unwrap();
    server.await.unwrap();
    p.transport().shutdown().await.unwrap();
}

#[tokio::test]
async fn shutdown_also_waits_for_and_cancels_initial_handshakes() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (seen, started) = oneshot::channel();
    let server = tokio::spawn(async move {
        let (mut tcp, _) = listener.accept().await.unwrap();
        seen.send(()).unwrap();
        use tokio::io::AsyncReadExt;
        let mut bytes = [0; 1024];
        while tcp.read(&mut bytes).await.unwrap_or(0) > 0 {}
    });
    let transport = StandardTransport::new(config()).unwrap();
    let clone = transport.clone();
    let task = tokio::spawn(async move {
        clone
            .subscribe(
                &format!("ws://{addr}"),
                hl::Subscription::Trades { coin: "ETH".into() },
            )
            .await
    });
    started.await.unwrap();
    timeout(Duration::from_millis(200), transport.shutdown())
        .await
        .unwrap()
        .unwrap();
    match task.await.unwrap() {
        Err(error) => assert_eq!(error.kind, TransportErrorKind::Closed),
        Ok(_) => panic!("connection should be cancelled"),
    };
    server.await.unwrap();
}
