use aux_rs_arcus::*;
use aux_rs_core::WebSocketSubscription;
use aux_rs_core::{HttpRequest, HttpResponse, HttpTransport, TransportError};
use serde_json::{Value, json};
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

fn auth() -> ArcusAuth<LocalSigner> {
    ArcusAuth::new(
        LocalSigner::from_bytes(Zeroizing::new([7; 32])),
        "0xABCDEF0000000000000000000000000000000001",
        2,
    )
    .unwrap()
}
fn market() -> MarketInfo {
    MarketInfo {
        market_id: 1,
        market_display_name: "BTC-USD".into(),
        tick_size: "0.1".into(),
        step_size: "0.00000001".into(),
        status: "ONLINE".into(),
        extra: Default::default(),
    }
}
fn order() -> OrderRequest {
    OrderRequest {
        market_id: 1,
        order_side: Side::Buy,
        order_type: OrderType::Limit,
        quantity: "0.001".into(),
        price: "50000.5".into(),
        time_in_force: TimeInForce::Alo,
        good_til_time: "4102444800000000".into(),
        client_id: Some("MixedCaseId".into()),
        reduce_only: false,
    }
}
#[tokio::test]
async fn reads_preserve_native_shapes_units_and_unknown_fields() {
    let m = Mock::response(include_str!("fixtures/markets.json"));
    let c = ArcusClient::with_transport(Arcus::testnet(), m.clone());
    let r = c.markets(MarketsRequest::default()).await.unwrap();
    assert_eq!(r.markets[0].market_id, 1);
    assert!(r.markets[0].extra.contains_key("tickTiers"));
    assert_eq!(
        m.requests.lock().unwrap()[0].url,
        "https://api.testnet.arcus.xyz/v1/markets"
    );
    let m = Mock::response(include_str!("fixtures/book.json"));
    let c = ArcusClient::with_transport(Arcus::testnet(), m);
    assert_eq!(
        c.order_book(OrderBookRequest::new("BTC-USD"))
            .await
            .unwrap()
            .bids[0][0],
        "70072"
    );
}
#[tokio::test]
async fn account_reads_do_not_require_credentials() {
    let m = Mock::response(include_str!("fixtures/account.json"));
    let c = ArcusClient::with_transport(Arcus::testnet(), m.clone());
    let a = c
        .account(AccountRequest::new("0x0000000000000000000000000000000000000001", 0).unwrap())
        .await
        .unwrap();
    assert_eq!(a.equity, "1000.000000000000000001");
    assert_eq!(a.sequence_number, u64::MAX);
    assert!(m.requests.lock().unwrap()[0].headers.is_empty());
}
#[test]
fn exact_native_units_reject_rounding_and_overflow() {
    let m = market();
    assert_eq!(m.price_ticks("50000.50").unwrap(), 500005);
    assert_eq!(m.size_quantums("0.001").unwrap(), 100000);
    for p in ["50000.51", "NaN", "1e3", "-1", "0", ".1"] {
        assert!(m.price_ticks(p).is_err(), "{p}");
    }
    assert!(
        m.size_quantums("999999999999999999999999999999999999999999")
            .is_err()
    );
}
#[tokio::test]
async fn typed_payload_matches_independent_ed25519_reference() {
    let a = auth();
    let expected: Value = serde_json::from_str(include_str!("fixtures/signing.json")).unwrap();
    let bytes = a
        .order_payload(&market(), &order(), 1770000000000000000)
        .unwrap();
    assert_eq!(std::str::from_utf8(&bytes).unwrap(), expected["payload"]);
    let signer = LocalSigner::from_bytes(Zeroizing::new([7; 32]));
    assert_eq!(
        hex::encode(signer.sign(&bytes).await.unwrap()),
        expected["signature"]
    );
    assert_eq!(hex::encode(signer.public_key()), expected["public_key"]);
    assert!(!format!("{signer:?} {a:?}").contains(&hex::encode([7; 32])));
    let cancel = a
        .cancel_payload(
            &CancelOrderRequest {
                market_id: 1,
                order_id: None,
                client_id: Some("MixedCaseId".into()),
            },
            1770000000000000000,
        )
        .unwrap();
    assert_eq!(
        std::str::from_utf8(&cancel).unwrap(),
        r#"{"ad":"0xabcdef0000000000000000000000000000000001","ai":2,"c":"MixedCaseId","ct":1770000000000000000,"m":1,"op":2,"v":1}"#
    );
}
#[tokio::test]
async fn trading_headers_and_body_share_timestamp_and_writes_are_single_attempt() {
    let m = Mock::response(r#"{"status":"ACK","orderId":"123"}"#);
    let c = ArcusClient::with_transport(Arcus::testnet(), m.clone()).with_auth(auth());
    assert_eq!(
        c.place_order(&market(), order()).await.unwrap().status,
        "ACK"
    );
    let rs = m.requests.lock().unwrap();
    assert_eq!(rs.len(), 1);
    let r = &rs[0];
    assert_eq!(r.url, "https://api.testnet.arcus.xyz/v1/placeOrder");
    let body: Value = serde_json::from_slice(&r.body).unwrap();
    let ts = r
        .headers
        .iter()
        .find(|(k, _)| k == "X-Timestamp")
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    assert_eq!(body["timestamp"], ts);
    let sig = r.headers.iter().find(|(k, _)| k == "X-Signature").unwrap();
    let key = ed25519_dalek::VerifyingKey::from_bytes(
        &LocalSigner::from_bytes(Zeroizing::new([7; 32])).public_key(),
    )
    .unwrap();
    let bytes = auth().order_payload(&market(), &order(), ts).unwrap();
    let sig = ed25519_dalek::Signature::from_slice(&hex::decode(&sig.1).unwrap()).unwrap();
    key.verify_strict(&bytes, &sig).unwrap();
}
#[tokio::test]
async fn bad_order_and_signer_are_rejected_before_network() {
    let m = Mock::default();
    let c = ArcusClient::with_transport(Arcus::testnet(), m.clone()).with_auth(auth());
    let mut r = order();
    r.price = "1.234".into();
    assert!(c.place_order(&market(), r).await.is_err());
    assert!(m.requests.lock().unwrap().is_empty());
    let e = Error::Api {
        status: Some(403),
        body: json!({"error":"secret-echo"}),
    };
    assert!(!format!("{e:?}").contains("secret-echo"));
    assert!(
        c.cancel_order(CancelOrderRequest {
            market_id: 1,
            order_id: Some("123".into()),
            client_id: Some("abc".into())
        })
        .await
        .is_err()
    );
}
#[test]
fn websocket_routes_native_views_and_requires_a_fresh_delta_snapshot() {
    let s = Subscription::account(
        Channel::Orders,
        "0x0000000000000000000000000000000000000001",
        2,
    )
    .unwrap();
    let message = json!({"type":"channel_data","channel":"orders","id":"0x0000000000000000000000000000000000000001","accountIndex":2,"contents":{}});
    assert!(s.matches(&message.to_string()));
    let mut other = message;
    other["accountIndex"] = json!(3);
    assert!(!s.matches(&other.to_string()));
    let s = Subscription::market(Channel::OrderBookUpdates, "BTC-USD");
    assert!(s.requires_fresh_snapshot());
    assert!(!s.is_initial_snapshot(r#"{"type":"channel_data","contents":{}}"#));
    assert!(!s.is_initial_snapshot(r#"{"type":"subscribed","contents":{}}"#));
    assert!(!s.is_initial_snapshot(r#"{"type":"subscribed","contents":null}"#));
    assert!(s.is_initial_snapshot(r#"{"type":"subscribed","contents":{"bids":[],"asks":[]}}"#));
    let frames: Vec<Value> = serde_json::from_str(include_str!("fixtures/stream.json")).unwrap();
    let e = Event::decode(&frames[1].to_string()).unwrap();
    assert_eq!(
        e.decode_contents::<OrderBook>().unwrap().bids[0][0],
        "70072"
    );
}

#[tokio::test]
async fn cloned_clients_share_signing_state_and_use_distinct_timestamps() {
    let m = Mock::response(r#"{"status":"ACK"}"#);
    m.responses.lock().unwrap().push_back(HttpResponse {
        status: 202,
        headers: vec![],
        body: br#"{"status":"ACK"}"#.to_vec(),
    });
    let c = ArcusClient::with_transport(Arcus::testnet(), m.clone()).with_auth(auth());
    let other = c.clone();
    let market = market();
    let (first, second) = tokio::join!(
        c.place_order(&market, order()),
        other.place_order(&market, order())
    );
    first.unwrap();
    second.unwrap();
    let requests = m.requests.lock().unwrap();
    let timestamps = requests
        .iter()
        .map(|r| {
            r.headers
                .iter()
                .find(|(k, _)| k == "X-Timestamp")
                .unwrap()
                .1
                .clone()
        })
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(timestamps.len(), 2);
}

#[test]
fn numeric_market_filters_match_normalized_envelopes_in_their_own_scope() {
    let mut s = Subscription::account(
        Channel::Orders,
        "0x0000000000000000000000000000000000000001",
        2,
    )
    .unwrap();
    s.market = Some("1".into());
    let event = json!({"type":"channel_data","channel":"orders","id":"0x0000000000000000000000000000000000000001","accountIndex":2,"market":"BTC-USD","contents":{}});
    assert!(s.matches(&event.to_string()));
    let mut other = s.clone();
    other.market = Some("2".into());
    assert_ne!(s.connection_scope(), other.connection_scope());
}
