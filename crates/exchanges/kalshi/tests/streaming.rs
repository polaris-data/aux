#![cfg(feature = "ws")]
use aux_rs_core::{
    RequestOutcome, TransportConfig, TransportError, TransportErrorKind, WebSocketStream,
    WebSocketTransport,
};
use aux_rs_kalshi::*;
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};
use tokio::{
    net::TcpListener,
    sync::oneshot,
    time::{Duration, timeout},
};
use tokio_tungstenite::tungstenite::{
    Message,
    handshake::server::{Request, Response},
};
#[derive(Clone)]
struct Signer(Arc<AtomicU64>);
impl RequestSigner for Signer {
    async fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        assert!(
            std::str::from_utf8(message)
                .unwrap()
                .ends_with("GET/socket")
        );
        Ok(self.0.fetch_add(1, Ordering::SeqCst).to_le_bytes().to_vec())
    }
}
#[tokio::test]
async fn signed_recovery_delivers_gap_before_fresh_snapshot() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let headers = Arc::new(Mutex::new(Vec::new()));
    let server_headers = headers.clone();
    let (release, wait) = oneshot::channel();
    let server = tokio::spawn(async move {
        let mut wait = Some(wait);
        for round in 0..2 {
            let (socket, _) = listener.accept().await.unwrap();
            let h = server_headers.clone();
            let mut ws = tokio_tungstenite::accept_hdr_async(
                socket,
                move |req: &Request, response: Response| {
                    assert_eq!(req.headers()["kalshi-access-key"], "fixture");
                    h.lock().unwrap().push(
                        req.headers()["kalshi-access-signature"]
                            .to_str()
                            .unwrap()
                            .to_owned(),
                    );
                    Ok(response)
                },
            )
            .await
            .unwrap();
            let request = ws.next().await.unwrap().unwrap();
            let v: Value = serde_json::from_str(request.to_text().unwrap()).unwrap();
            assert_eq!(v["cmd"], "subscribe");
            ws.send(Message::Text(json!({"type":"subscribed","id":v["id"],"msg":{"sid":round+1,"channel":"orderbook_delta"}}).to_string().into())).await.unwrap();
            ws.send(Message::Text(json!({"type":"orderbook_snapshot","sid":round+1,"seq":1,"msg":{"market_ticker":"TEST","yes_dollars":[["0.5500","10.00"]],"no_dollars":[]}}).to_string().into())).await.unwrap();
            if round == 0 {
                wait.take().unwrap().await.unwrap();
                ws.send(Message::Text(json!({"type":"orderbook_delta","sid":1,"seq":3,"msg":{"market_ticker":"TEST","price_dollars":"0.5500","delta_fp":"1.00","side":"yes"}}).to_string().into())).await.unwrap();
            } else {
                while let Some(Ok(message)) = ws.next().await {
                    if message.is_close() {
                        break;
                    }
                }
            }
        }
    });
    let config = TransportConfig {
        reconnect: aux_rs_core::BackoffConfig {
            max_retries: 1,
            initial_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(10),
        },
        ..Default::default()
    };
    let exchange = Kalshi::demo()
        .custom(
            "predictions",
            "http://unused",
            Some(&format!("ws://{address}/socket")),
        )
        .unwrap();
    let count = Arc::new(AtomicU64::new(0));
    let client = KalshiClient::with_config(exchange, config)
        .unwrap()
        .with_auth(KalshiAuth::new("fixture", Signer(count.clone())).unwrap());
    let mut stream = client
        .subscribe(Subscription {
            service: "predictions".into(),
            params: json!({"channel":"orderbook_delta","market_tickers":["TEST"]}),
        })
        .await
        .unwrap();
    loop {
        let e = timeout(Duration::from_secs(3), stream.next())
            .await
            .unwrap()
            .unwrap()
            .unwrap();
        if e.0["type"] == "orderbook_snapshot" {
            break;
        }
    }
    release.send(()).unwrap();
    let mut gap = false;
    loop {
        let e = timeout(Duration::from_secs(3), stream.next())
            .await
            .unwrap()
            .unwrap();
        match e {
            Err(aux_rs_core::Error::Transport(e)) if e.is_gap() => gap = true,
            Ok(e) if e.0["type"] == "orderbook_snapshot" => {
                assert!(gap);
                break;
            }
            Ok(_) => {}
            Err(e) => panic!("{e}"),
        }
    }
    assert_eq!(count.load(Ordering::SeqCst), 2);
    {
        let h = headers.lock().unwrap();
        assert_ne!(h[0], h[1]);
    }
    stream.close().await.unwrap();
    client.transport().shutdown().await.unwrap();
    server.await.unwrap();
}
struct Legacy;
struct Empty;
impl WebSocketStream for Empty {
    async fn next(&mut self) -> Option<Result<String, TransportError>> {
        None
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        Ok(())
    }
}
impl WebSocketTransport for Legacy {
    type Stream = Empty;
    async fn subscribe<S: aux_rs_core::WebSocketSubscription>(
        &self,
        _: &str,
        _: S,
    ) -> Result<Empty, TransportError> {
        panic!("unsupported transport must fail before dialing")
    }
}
#[tokio::test]
async fn legacy_custom_transport_rejects_signed_upgrades() {
    let client = KalshiClient::with_transport(Kalshi::demo(), Legacy)
        .with_auth(KalshiAuth::new("fixture", Signer(Arc::new(AtomicU64::new(0)))).unwrap());
    let result = client
        .subscribe(Subscription {
            service: "predictions".into(),
            params: json!({"channel":"ticker"}),
        })
        .await;
    assert!(matches!(
        result,
        Err(aux_rs_core::Error::Transport(TransportError {
            kind: TransportErrorKind::Configuration,
            outcome: RequestOutcome::NotSent,
            ..
        }))
    ));
}
