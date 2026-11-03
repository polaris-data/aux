#![cfg(all(feature = "monaco", feature = "ws"))]
use aux_rs::{monaco::*, *};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::oneshot,
    time::{sleep, timeout},
};
use tokio_tungstenite::{WebSocketStream as Socket, tungstenite::Message};
use zeroize::Zeroizing;
const PAIR: &str = "550e8400-e29b-41d4-a716-446655440000";
fn config() -> TransportConfig {
    TransportConfig {
        timeout: Duration::from_millis(500),
        reconnect: BackoffConfig {
            max_retries: 2,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(20),
        },
        ..Default::default()
    }
}
async fn socket(listener: &TcpListener) -> Socket<TcpStream> {
    let (tcp, _) = timeout(Duration::from_secs(3), listener.accept())
        .await
        .unwrap()
        .unwrap();
    tokio_tungstenite::accept_async(tcp).await.unwrap()
}
async fn next(socket: &mut Socket<TcpStream>) -> Value {
    loop {
        let m = timeout(Duration::from_secs(3), socket.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        if let Message::Text(s) = m {
            return serde_json::from_str(&s).unwrap();
        }
    }
}
async fn send(socket: &mut Socket<TcpStream>, v: Value) {
    socket
        .send(Message::Text(v.to_string().into()))
        .await
        .unwrap();
}
fn exchange(addr: std::net::SocketAddr) -> Monaco {
    Monaco::custom("http://unused", format!("ws://{addr}/ws"), 1328).unwrap()
}
struct CountingSigner {
    inner: LocalSessionSigner,
    count: Arc<AtomicUsize>,
}
impl SessionSigner for CountingSigner {
    fn public_key(&self) -> [u8; 32] {
        self.inner.public_key()
    }
    async fn sign(&self, m: &[u8]) -> Result<[u8; 64], monaco::Error> {
        self.count.fetch_add(1, Ordering::SeqCst);
        self.inner.sign(m).await
    }
}
fn auth(e: &Monaco, seed: u8, count: Arc<AtomicUsize>) -> MonacoAuth<CountingSigner> {
    let signer = CountingSigner {
        inner: LocalSessionSigner::from_bytes(Zeroizing::new([seed; 32])),
        count,
    };
    let public = hex::encode(signer.public_key());
    MonacoAuth::restore(
        e.clone(),
        signer,
        SessionInfo {
            address: format!("0x{}", "11".repeat(20)),
            client_id: "fixture".into(),
            session_public_key: public,
            expires_at: unix_millis().unwrap() / 1000 + 3600,
            chain_id: 1328,
        },
        RequestSigning::Legacy,
    )
    .unwrap()
}
async fn data<S: WebSocketStream>(handle: &mut SubscriptionHandle<S>) -> Event {
    loop {
        let e = timeout(Duration::from_secs(3), handle.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        if matches!(e.kind.as_str(), "Snapshot" | "Event") {
            return e;
        }
    }
}
#[tokio::test]
async fn private_handshake_precedes_subscriptions_reconnect_resigns_and_reports_gap_before_snapshot()
 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let e = exchange(listener.local_addr().unwrap());
    let count = Arc::new(AtomicUsize::new(0));
    let (ready, recovered) = oneshot::channel();
    let (release, wait) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut first = socket(&listener).await;
        let a = next(&mut first).await;
        assert_eq!(a["type"], "Authenticate");
        assert!(
            timeout(Duration::from_millis(25), first.next())
                .await
                .is_err()
        );
        send(&mut first, json!({"type":"Authenticated"})).await;
        assert_eq!(
            next(&mut first).await,
            json!({"type":"Subscribe","channels":["orders"]})
        );
        send(
            &mut first,
            json!({"type":"Subscribed","channels":["orders"]}),
        )
        .await;
        send(
            &mut first,
            json!({"type":"Snapshot","channel":"orders","data":[]}),
        )
        .await;
        sleep(Duration::from_millis(5)).await;
        first.close(None).await.unwrap();
        let mut second = socket(&listener).await;
        let b = next(&mut second).await;
        assert_eq!(b["type"], "Authenticate");
        assert!(b["timestamp"].as_u64().unwrap() > a["timestamp"].as_u64().unwrap());
        assert_ne!(b["signature"], a["signature"]);
        send(&mut second, json!({"type":"Authenticated"})).await;
        assert_eq!(next(&mut second).await["channels"][0], "orders");
        send(
            &mut second,
            json!({"type":"Subscribed","channels":["orders"]}),
        )
        .await;
        send(
            &mut second,
            json!({"type":"Snapshot","channel":"orders","data":[{"version":9007199254740993u64}]}),
        )
        .await;
        ready.send(()).unwrap();
        let _ = wait.await;
        assert_eq!(next(&mut second).await["type"], "Unsubscribe");
        let _ = second.next().await;
    });
    let c = MonacoClient::with_config(e.clone(), config())
        .unwrap()
        .with_auth(auth(&e, 1, count.clone()));
    let mut h = c
        .subscribe_account(AccountSubscription::Orders {
            trading_pair_id: None,
            mode: None,
        })
        .await
        .unwrap();
    recovered.await.unwrap();
    assert!(matches!(h.next().await,Some(Err(aux_rs::Error::Transport(e))) if e.is_gap()));
    let event = data(&mut h).await;
    assert_eq!(event.kind, "Snapshot");
    assert_eq!(event.data.unwrap()[0]["version"], 9007199254740993u64);
    assert_eq!(count.load(Ordering::SeqCst), 2);
    release.send(()).unwrap();
    h.close().await.unwrap();
    c.transport().shutdown().await.unwrap();
    server.await.unwrap();
}
#[tokio::test]
async fn revoked_credentials_wake_quiet_streams_and_auth_failure_never_subscribes() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let e = exchange(listener.local_addr().unwrap());
    let (ready, online) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut ws = socket(&listener).await;
        assert_eq!(next(&mut ws).await["type"], "Authenticate");
        send(&mut ws, json!({"type":"Authenticated"})).await;
        next(&mut ws).await;
        send(
            &mut ws,
            json!({"type":"Snapshot","channel":"balances","data":[]}),
        )
        .await;
        ready.send(()).unwrap();
        let _ = ws.next().await;
        let mut rejected = socket(&listener).await;
        assert_eq!(next(&mut rejected).await["type"], "Authenticate");
        send(
            &mut rejected,
            json!({"type":"Error","code":"AUTH_FAILED","message":"expired"}),
        )
        .await;
        let reply = timeout(Duration::from_millis(250), rejected.next()).await;
        if let Ok(Some(Ok(Message::Text(text)))) = reply {
            panic!("sent after failed auth: {text}");
        }
    });
    let a = auth(&e, 1, Arc::default());
    let c = MonacoClient::with_config(e.clone(), config())
        .unwrap()
        .with_auth(a.clone());
    let mut h = c
        .subscribe_account(AccountSubscription::Balances)
        .await
        .unwrap();
    online.await.unwrap();
    data(&mut h).await;
    let pending =
        tokio::spawn(async move { timeout(Duration::from_millis(250), h.next()).await.unwrap() });
    sleep(Duration::from_millis(10)).await;
    a.invalidate();
    assert!(
        matches!(pending.await.unwrap(),Some(Err(aux_rs::Error::Transport(e))) if e.kind==TransportErrorKind::Authentication)
    );
    let other = MonacoClient::with_config(e.clone(), config())
        .unwrap()
        .with_auth(auth(&e, 2, Arc::default()));
    assert!(
        matches!(other.subscribe_account(AccountSubscription::Balances).await,Err(aux_rs::Error::Transport(e)) if e.kind==TransportErrorKind::Authentication)
    );
    c.transport().shutdown().await.unwrap();
    other.transport().shutdown().await.unwrap();
    server.await.unwrap();
}
#[tokio::test]
async fn independent_accounts_use_distinct_connections_and_late_consumers_get_new_snapshots() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let e = exchange(listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let mut sockets = Vec::new();
        let mut keys = Vec::new();
        for n in 0..3 {
            let mut ws = socket(&listener).await;
            keys.push(next(&mut ws).await["public_key"].clone());
            send(&mut ws, json!({"type":"Authenticated"})).await;
            next(&mut ws).await;
            send(&mut ws,json!({"type":"Snapshot","channel":"balances","data":[{"total_balance":n.to_string()}]})).await;
            sockets.push(ws);
        }
        assert_ne!(keys[0], keys[1]);
        assert_eq!(keys[0], keys[2]);
        for mut ws in sockets {
            let _ = ws.next().await;
        }
    });
    let c = MonacoClient::with_config(e.clone(), config()).unwrap();
    let a = c.clone().with_auth(auth(&e, 1, Arc::default()));
    let b = c.clone().with_auth(auth(&e, 2, Arc::default()));
    let mut one = a
        .subscribe_account(AccountSubscription::Balances)
        .await
        .unwrap();
    let mut two = b
        .subscribe_account(AccountSubscription::Balances)
        .await
        .unwrap();
    let mut late = a
        .subscribe_account(AccountSubscription::Balances)
        .await
        .unwrap();
    for (n, h) in [&mut one, &mut two, &mut late].into_iter().enumerate() {
        assert_eq!(
            data(h).await.data.unwrap()[0]["total_balance"],
            n.to_string()
        );
    }
    c.transport().shutdown().await.unwrap();
    server.await.unwrap();
}
#[tokio::test]
async fn public_depth_allows_overlapping_links_preserves_integers_and_flags_broken_links() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let e = exchange(listener.local_addr().unwrap());
    let channel = format!("depth:{PAIR}:MARGIN");
    let raw_channel = channel.clone();
    let server = tokio::spawn(async move {
        let mut ws = socket(&listener).await;
        assert_eq!(next(&mut ws).await["channels"][0], raw_channel);
        send(
            &mut ws,
            json!({"type":"Subscribed","channels":[raw_channel]}),
        )
        .await;
        send(
            &mut ws,
            json!({"type":"Error","channel":raw_channel,"code":"SNAPSHOT_UNAVAILABLE"}),
        )
        .await;
        send(&mut ws,json!({"type":"Snapshot","channel":raw_channel,"data":{"book_seq":9007199254740993u64,"bids":[],"asks":[]}})).await;
        send(&mut ws,json!({"type":"Event","channel":raw_channel,"data":{"book_seq":9007199254740995u64,"prev_book_seq":9007199254740992u64,"bids":[],"asks":[]}})).await;
        send(&mut ws,json!({"type":"Event","channel":raw_channel,"data":{"book_seq":9007199254741000u64,"prev_book_seq":9007199254740999u64}})).await;
        let _ = ws.next().await;
    });
    let c = MonacoClient::with_config(e, config()).unwrap();
    let mut h = c
        .subscribe(monaco::Subscription::Depth {
            trading_pair_id: PAIR.into(),
            mode: "MARGIN".into(),
        })
        .await
        .unwrap();
    assert_eq!(h.next().await.unwrap().unwrap().kind, "Subscribed");
    assert!(matches!(h.next().await,Some(Err(aux_rs::Error::Transport(e))) if e.is_gap()));
    assert_eq!(h.next().await.unwrap().unwrap().kind, "Error");
    assert_eq!(
        data(&mut h).await.data.unwrap()["book_seq"],
        9007199254740993u64
    );
    assert_eq!(data(&mut h).await.kind, "Event");
    assert!(matches!(h.next().await,Some(Err(aux_rs::Error::Transport(e))) if e.is_gap()));
    assert!(h.next().await.is_none());
    c.transport().shutdown().await.unwrap();
    server.await.unwrap();
}
#[tokio::test]
async fn shutdown_cancels_an_unresponsive_authentication_handshake() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let e = exchange(listener.local_addr().unwrap());
    let (sent, received) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut ws = socket(&listener).await;
        next(&mut ws).await;
        sent.send(()).unwrap();
        let _ = ws.next().await;
    });
    let c = MonacoClient::with_config(e.clone(), config())
        .unwrap()
        .with_auth(auth(&e, 1, Arc::default()));
    let other = c.clone();
    let pending =
        tokio::spawn(async move { other.subscribe_account(AccountSubscription::Account).await });
    received.await.unwrap();
    timeout(Duration::from_millis(250), c.transport().shutdown())
        .await
        .unwrap()
        .unwrap();
    assert!(pending.await.unwrap().is_err());
    server.await.unwrap();
}

#[tokio::test]
async fn quiet_private_stream_terminates_at_session_expiry() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let e = exchange(listener.local_addr().unwrap());
    let signer = LocalSessionSigner::from_bytes(Zeroizing::new([8; 32]));
    let public = hex::encode(signer.public_key());
    let a = MonacoAuth::restore(
        e.clone(),
        signer,
        SessionInfo {
            address: format!("0x{}", "11".repeat(20)),
            client_id: "fixture".into(),
            session_public_key: public,
            expires_at: unix_millis().unwrap() / 1000 + 2,
            chain_id: 1328,
        },
        RequestSigning::Legacy,
    )
    .unwrap();
    let server = tokio::spawn(async move {
        let mut ws = socket(&listener).await;
        next(&mut ws).await;
        send(&mut ws, json!({"type":"Authenticated"})).await;
        next(&mut ws).await;
        send(
            &mut ws,
            json!({"type":"Snapshot","channel":"account","data":[]}),
        )
        .await;
        let _ = ws.next().await;
    });
    let c = MonacoClient::with_config(e, config()).unwrap().with_auth(a);
    let mut h = c
        .subscribe_account(AccountSubscription::Account)
        .await
        .unwrap();
    data(&mut h).await;
    let end = timeout(Duration::from_secs(3), h.next()).await.unwrap();
    assert!(
        matches!(end,Some(Err(aux_rs::Error::Transport(e))) if e.kind==TransportErrorKind::Authentication)
    );
    assert!(h.next().await.is_none());
    c.transport().shutdown().await.unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn remote_session_invalidation_preserves_native_error_and_forbids_reconnect() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let e = exchange(listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let mut ws = socket(&listener).await;
        next(&mut ws).await;
        send(&mut ws, json!({"type":"Authenticated"})).await;
        next(&mut ws).await;
        send(
            &mut ws,
            json!({"type":"Snapshot","channel":"orders","data":[]}),
        )
        .await;
        send(
            &mut ws,
            json!({"type":"Error","code":"SESSION_INVALID","message":"session revoked"}),
        )
        .await;
        let _ = ws.next().await;
        assert!(
            timeout(Duration::from_millis(100), listener.accept())
                .await
                .is_err()
        );
    });
    let c = MonacoClient::with_config(e.clone(), config())
        .unwrap()
        .with_auth(auth(&e, 3, Arc::default()));
    let mut h = c
        .subscribe_account(AccountSubscription::Orders {
            trading_pair_id: None,
            mode: None,
        })
        .await
        .unwrap();
    loop {
        match timeout(Duration::from_secs(2), h.next()).await.unwrap() {
            Some(Ok(_)) => continue,
            Some(Err(aux_rs::Error::Exchange(monaco::Error::Api { body, .. }))) => {
                assert_eq!(body["code"], "SESSION_INVALID");
                break;
            }
            other => panic!("expected native session error, got {other:?}"),
        }
    }
    assert!(!c.auth().is_active());
    assert!(h.next().await.is_none());
    c.transport().shutdown().await.unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn missing_account_snapshot_signals_gap_and_allows_live_reconciliation() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let e = exchange(listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let mut ws = socket(&listener).await;
        next(&mut ws).await;
        send(&mut ws, json!({"type":"Authenticated"})).await;
        next(&mut ws).await;
        send(
            &mut ws,
            json!({"type":"Error","channel":"balances","code":"SNAPSHOT_UNAVAILABLE"}),
        )
        .await;
        send(
            &mut ws,
            json!({"type":"Event","channel":"balances","data":{"version":2}}),
        )
        .await;
        let _ = ws.next().await;
    });
    let c = MonacoClient::with_config(e.clone(), config())
        .unwrap()
        .with_auth(auth(&e, 4, Arc::default()));
    let mut h = c
        .subscribe_account(AccountSubscription::Balances)
        .await
        .unwrap();
    assert_eq!(h.next().await.unwrap().unwrap().kind, "Authenticated");
    assert!(matches!(h.next().await,Some(Err(aux_rs::Error::Transport(e))) if e.is_gap()));
    assert_eq!(
        h.next().await.unwrap().unwrap().extra["code"],
        "SNAPSHOT_UNAVAILABLE"
    );
    assert_eq!(h.next().await.unwrap().unwrap().kind, "Event");
    h.close().await.unwrap();
    c.transport().shutdown().await.unwrap();
    server.await.unwrap();
}
