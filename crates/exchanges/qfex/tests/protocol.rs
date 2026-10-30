use aux_rs_core::{
    Error as ClientError, WebSocketConnection, WebSocketConnector, WebSocketSubscription,
};
use aux_rs_core::{HttpRequest, HttpResponse, HttpTransport, TransportError};
use aux_rs_qfex::*;
use serde_json::{Value, json};
use std::time::Duration;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use zeroize::Zeroizing;
#[derive(Clone, Default)]
struct Mock {
    requests: Arc<Mutex<Vec<HttpRequest>>>,
    responses: Arc<Mutex<VecDeque<HttpResponse>>>,
}
impl Mock {
    fn response(body: &str) -> Self {
        let m = Self::default();
        m.responses.lock().unwrap().push_back(HttpResponse {
            status: 200,
            headers: vec![],
            body: body.as_bytes().to_vec(),
        });
        m
    }
}
impl HttpTransport for Mock {
    async fn execute(&self, r: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.requests.lock().unwrap().push(r);
        Ok(self
            .responses
            .lock()
            .unwrap()
            .pop_front()
            .expect("no extra requests"))
    }
}

#[tokio::test]
async fn native_reads_preserve_precision_and_pin_contract() {
    let m = Mock::response(include_str!("fixtures/markets.json"));
    let c = QfexClient::with_transport(Qfex::testnet(), m.clone());
    let r = c
        .refdata(RefdataRequest {
            ticker: Some("AAPL-USD".into()),
        })
        .await
        .unwrap();
    assert_eq!(r.data[0].symbol, "AAPL-USD");
    assert_eq!(
        m.requests.lock().unwrap()[0].url,
        "https://api.qfex.io/refdata?api_version=2026-10-29&ticker=AAPL-USD"
    );
    let m = Mock::response(include_str!("fixtures/book.json"));
    let c = QfexClient::with_transport(Qfex::testnet(), m);
    assert_eq!(
        c.order_book(OrderBookRequest::new("AAPL-USD"))
            .await
            .unwrap()
            .bids[0][0],
        "300"
    );
}
#[tokio::test]
async fn hmac_account_reads_use_fresh_headers_and_exact_numbers() {
    let a = QfexAuth::new("qfex_pub_test", Zeroizing::new("public-test-secret".into())).unwrap();
    let expected: Value = serde_json::from_str(include_str!("fixtures/auth.json")).unwrap();
    assert_eq!(a.signature("c0ffee", 1760545414).unwrap(), expected["hmac"]);
    assert!(!format!("{a:?}").contains("public-test-secret"));
    let h1 = a.headers().unwrap();
    let h2 = a.headers().unwrap();
    assert_ne!(h1[2].1, h2[2].1);
    let m = Mock::response(include_str!("fixtures/positions.json"));
    let c = QfexClient::with_transport(Qfex::testnet(), m.clone()).with_auth(a);
    let r = c.positions().await.unwrap();
    assert_eq!(
        r.balance.available_balance.to_string(),
        "1000.000000000000000001"
    );
    assert_eq!(
        m.requests.lock().unwrap()[0].headers[0].0,
        "x-qfex-public-key"
    );
}
#[test]
fn public_stream_routing_and_native_payloads() {
    let s = Subscription::new(Channel::OrderBook, "AAPL-USD");
    let frames: Vec<Value> = serde_json::from_str(include_str!("fixtures/stream.json")).unwrap();
    for v in frames {
        let text = v.to_string();
        assert!(s.matches(&text));
        let e = Event::decode(&text).unwrap();
        if e.kind == "level2" {
            let books = e.payloads::<OrderBookUpdate>().unwrap();
            assert_eq!(books[0].bid[0][0], "300");
        }
    }
    assert!(!s.matches(r#"{"type":"level2","symbol":"OTHER"}"#));
    let e =
        Event::decode(r#"{"error_code":"PermissionDenied","message":"secret-echo"}"#).unwrap_err();
    assert!(!format!("{e:?}").contains("secret-echo"));
}
#[derive(Clone, Default)]
struct Socket {
    sent: Arc<Mutex<Vec<Value>>>,
    frames: Arc<Mutex<VecDeque<String>>>,
}
impl Socket {
    fn push(&self, v: Value) {
        self.frames.lock().unwrap().push_back(v.to_string());
    }
}
impl WebSocketConnection for Socket {
    async fn send(&mut self, text: String) -> Result<(), TransportError> {
        self.sent
            .lock()
            .unwrap()
            .push(serde_json::from_str(&text).unwrap());
        Ok(())
    }
    async fn next(&mut self) -> Option<Result<String, TransportError>> {
        let v = self.frames.lock().unwrap().pop_front();
        match v {
            Some(v) => Some(Ok(v)),
            None => std::future::pending().await,
        }
    }
    async fn next_with_timeout(&mut self, _: Duration) -> Option<Result<String, TransportError>> {
        self.next().await
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        Ok(())
    }
}
impl WebSocketConnector for Socket {
    type Connection = Self;
    async fn connect(&self, _: &str) -> Result<Self, TransportError> {
        Ok(self.clone())
    }
}
fn order() -> AddOrder {
    AddOrder {
        symbol: "AAPL-USD".into(),
        side: Side::Buy,
        order_type: OrderType::Limit,
        order_time_in_force: TimeInForce::Gtc,
        quantity: "1.230000000000000001".parse().unwrap(),
        price: Some("200.01".parse().unwrap()),
        client_order_id: "MixedCaseId".into(),
        reduce_only: 0,
    }
}
fn client(socket: Socket) -> QfexClient<Socket, QfexAuth> {
    QfexClient::with_transport(Qfex::testnet(), socket).with_auth(
        QfexAuth::new("qfex_pub_test", Zeroizing::new("public-test-secret".into())).unwrap(),
    )
}
#[tokio::test]
async fn authenticated_commands_wait_for_auth_and_send_once() {
    let s = Socket::default();
    s.push(json!({"authenticated":true}));
    s.push(json!({"order_response":{"order_id":"abc","client_order_id":"MixedCaseId","symbol":"AAPL-USD","status":"ACK"}}));
    let mut c = client(s.clone()).trading_session().await.unwrap();
    let r = c.add_order(order()).await.unwrap();
    assert_eq!(r.status, "ACK");
    let sent = s.sent.lock().unwrap();
    assert_eq!(sent.len(), 3);
    assert_eq!(sent[0]["type"], "auth");
    assert_eq!(sent[1]["params"]["channels"][0], "order_responses");
    assert_eq!(
        sent[2]["params"]["quantity"].to_string(),
        "1.230000000000000001"
    );
}
#[tokio::test]
async fn rejected_auth_never_sends_orders() {
    let s = Socket::default();
    s.push(json!({"authenticated":false}));
    assert!(client(s.clone()).trading_session().await.is_err());
    assert_eq!(s.sent.lock().unwrap().len(), 1);
}
#[tokio::test]
async fn cancelled_submission_blocks_following_commands_until_native_reconciliation() {
    let s = Socket::default();
    s.push(json!({"authenticated":true}));
    let mut c = client(s.clone()).trading_session().await.unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(10), c.add_order(order()))
            .await
            .is_err()
    );
    assert!(matches!(
        c.add_order(order()).await,
        Err(ClientError::Exchange(Error::PendingOutcome))
    ));
    assert_eq!(s.sent.lock().unwrap().len(), 3);
    s.push(json!({"order_response":{"client_order_id":"MixedCaseId","order_id":"abc","symbol":"AAPL-USD","status":"OPEN"}}));
    assert!(c.next().await.unwrap().is_ok());
}

#[tokio::test]
async fn unconsumed_account_events_fail_explicitly_instead_of_hiding_command_outcomes() {
    let s = Socket::default();
    s.push(json!({"authenticated":true}));
    for _ in 0..129 {
        s.push(json!({"type":"balance","balance":{"available_balance":100}}));
    }
    let mut c = client(s.clone()).trading_session().await.unwrap();
    let error = c.add_order(order()).await.unwrap_err();
    assert!(matches!(
        error,
        ClientError::Transport(aux_rs_core::TransportError {
            kind: aux_rs_core::TransportErrorKind::Overflow,
            ..
        })
    ));
    assert_eq!(s.sent.lock().unwrap().len(), 3);
}
