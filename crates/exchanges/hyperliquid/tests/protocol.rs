use aux_rs_hyperliquid::HyperliquidClient;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use aux_rs_core::{
    Endpoint, Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpResponse, HttpTransport,
    TransportError, WebSocketSubscription,
};
use aux_rs_hyperliquid::{Error, Event, Hyperliquid, L2BookRequest, MetaRequest, Subscription};
use serde_json::{Value, json};

const META: &str = include_str!("fixtures/meta.json");
const BOOK: &str = include_str!("fixtures/book.json");
const TRADES: &str = include_str!("fixtures/trades.json");

fn response(status: u16, body: &str) -> HttpResponse {
    HttpResponse {
        status,
        headers: vec![],
        body: body.as_bytes().to_vec(),
    }
}

#[derive(Clone)]
struct MockHttp {
    requests: Arc<Mutex<Vec<HttpRequest>>>,
    responses: Arc<Mutex<VecDeque<HttpResponse>>>,
}

impl MockHttp {
    fn new(responses: Vec<HttpResponse>) -> Self {
        Self {
            requests: Arc::default(),
            responses: Arc::new(Mutex::new(responses.into())),
        }
    }
}

impl HttpTransport for MockHttp {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.requests.lock().unwrap().push(request);
        Ok(self
            .responses
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected request"))
    }
}

#[tokio::test]
async fn custom_transport_preserves_native_requests_responses_and_clone_state() {
    let transport = MockHttp::new(vec![response(200, META), response(200, BOOK)]);
    let client = HyperliquidClient::with_transport(
        Hyperliquid::custom("http://proxy/hl/", "ws://proxy/ws"),
        transport.clone(),
    );
    let meta = client
        .meta_with(MetaRequest {
            dex: Some("xyz".into()),
        })
        .await
        .unwrap();
    assert_eq!(meta.universe[1].name, "xyz:SAMPLE");
    assert_eq!(meta.universe[1].extra["futureField"], "preserved");
    assert_eq!(meta.extra["collateralToken"], 0);
    assert_eq!(meta.margin_tables[0].1.margin_tiers[0].lower_bound, "0.0");
    let book = client
        .clone()
        .l2_book(L2BookRequest::new("ETH"))
        .await
        .unwrap();
    assert_eq!(book.levels[0][0].px, "3000.00");
    assert_eq!(book.levels[0][0].sz, "1.2300");
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].method, HttpMethod::Post);
    assert_eq!(requests[0].url, "http://proxy/hl/info");
    assert_eq!(
        requests[0].headers,
        [("content-type".into(), "application/json".into())]
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&requests[0].body).unwrap(),
        json!({"type":"meta", "dex":"xyz"})
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&requests[1].body).unwrap(),
        json!({"type":"l2Book", "coin":"ETH"})
    );
}

#[tokio::test]
async fn invalid_book_aggregation_fails_before_transport() {
    let transport = MockHttp::new(vec![]);
    let client = HyperliquidClient::with_transport(Hyperliquid::testnet(), transport.clone());
    for request in [
        L2BookRequest::new(" "),
        L2BookRequest {
            coin: "ETH".into(),
            n_sig_figs: Some(1),
            mantissa: None,
        },
        L2BookRequest {
            coin: "ETH".into(),
            n_sig_figs: Some(4),
            mantissa: Some(2),
        },
        L2BookRequest {
            coin: "ETH".into(),
            n_sig_figs: Some(5),
            mantissa: Some(3),
        },
    ] {
        assert!(matches!(
            client.l2_book(request).await,
            Err(ClientError::Exchange(Error::InvalidRequest(_)))
        ));
    }
    assert!(transport.requests.lock().unwrap().is_empty());
    let request = L2BookRequest {
        coin: "@1".into(),
        n_sig_figs: Some(5),
        mantissa: Some(2),
    }
    .request(&Hyperliquid::testnet())
    .unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&request.body).unwrap(),
        json!({"type":"l2Book","coin":"@1","nSigFigs":5,"mantissa":2})
    );
}

#[test]
fn http_errors_and_malformed_successes_are_distinct() {
    assert!(matches!(MetaRequest::decode(response(429, "rate limited")),
        Err(Error::Http { status: 429, body }) if body == "rate limited"));
    assert!(
        matches!(MetaRequest::decode(response(200, r#"{"status":"err","response":"native failure"}"#)),
        Err(Error::Api { message }) if message == "native failure")
    );
    assert!(matches!(
        MetaRequest::decode(response(200, r#""native error""#)),
        Err(Error::Api { .. })
    ));
    assert!(matches!(
        MetaRequest::decode(response(200, "not JSON")),
        Err(Error::Decode(_))
    ));
    assert!(matches!(
        L2BookRequest::decode(response(200, r#"{"coin":"ETH","time":1,"levels":[[]]}"#)),
        Err(Error::Decode(_))
    ));
    assert!(matches!(
        L2BookRequest::decode(response(
            200,
            r#"{"coin":"ETH","time":1,"levels":[[{"px":3000.1,"sz":"1","n":1}],[]]}"#
        )),
        Err(Error::Decode(_))
    ));
}

#[test]
fn subscription_routing_is_native_and_never_silently_drops_bad_frames() {
    let book = Subscription::L2Book { coin: "ETH".into() };
    let trades = Subscription::Trades { coin: "ETH".into() };
    assert_eq!(
        serde_json::from_str::<Value>(&book.subscribe_message()).unwrap(),
        json!({"method":"subscribe","subscription":{"type":"l2Book","coin":"ETH"}})
    );
    assert_eq!(
        serde_json::from_str::<Value>(&book.unsubscribe_message()).unwrap()["method"],
        "unsubscribe"
    );
    let book_message = format!(r#"{{"channel":"l2Book","data":{BOOK}}}"#);
    assert!(book.matches(&book_message));
    assert!(!trades.matches(&book_message));
    assert!(!Subscription::L2Book { coin: "BTC".into() }.matches(&book_message));
    assert!(trades.matches(TRADES));
    assert!(!book.matches(TRADES));
    assert!(book.matches("bad JSON"));
    assert!(book.matches(r#"{"channel":"error","data":"invalid subscription"}"#));
    assert!(!book.matches(r#"{"channel":"pong"}"#));
    let ack = r#"{"channel":"subscriptionResponse","data":{"method":"subscribe","subscription":{"type":"l2Book","coin":"ETH"}}}"#;
    assert!(book.matches(ack));
    assert!(!trades.matches(ack));
    assert!(matches!(
        Event::decode(ack),
        Ok(Event::SubscriptionResponse(_))
    ));
    let malformed_ack = r#"{"channel":"subscriptionResponse","data":{"method":"subscribe"}}"#;
    assert!(book.matches(malformed_ack));
    assert!(matches!(
        Event::decode(malformed_ack),
        Err(Error::Decode(_))
    ));
}

#[test]
fn native_events_keep_exact_values_ids_and_unknown_data() {
    let Event::Trades(trades) = Event::decode(TRADES).unwrap() else {
        panic!("expected trades");
    };
    assert_eq!(trades[0].sz, "0.0100");
    assert_eq!(trades[0].tid, 1_125_899_906_842_623);
    let Event::Unknown { channel, data } =
        Event::decode(r#"{"channel":"futureChannel","data":{"new":"value"}}"#).unwrap()
    else {
        panic!("expected unknown channel");
    };
    assert_eq!(channel, "futureChannel");
    assert_eq!(data["new"], "value");
    assert!(matches!(
        Event::decode(r#"{"channel":"error","data":"invalid subscription"}"#),
        Err(Error::Api { .. })
    ));
    assert!(matches!(Event::decode("bad JSON"), Err(Error::Decode(_))));
}

#[test]
fn explicit_networks_and_request_debug_output() {
    assert_eq!(
        Hyperliquid::testnet().http_url(),
        "https://api.hyperliquid-testnet.xyz"
    );
    assert_eq!(
        Hyperliquid::mainnet().ws_url(),
        "wss://api.hyperliquid.xyz/ws"
    );
    let request = HttpRequest {
        method: HttpMethod::Post,
        url: "https://host/?secret=url".into(),
        headers: vec![("authorization".into(), "secret-header".into())],
        body: b"secret-body".to_vec(),
    };
    let debug = format!("{request:?}");
    assert!(!debug.contains("secret"));
    assert!(debug.contains("Post"));
    assert!(
        !format!(
            "{:?}",
            Hyperliquid::custom("https://host/?secret", "wss://host/?secret")
        )
        .contains("secret")
    );
}
