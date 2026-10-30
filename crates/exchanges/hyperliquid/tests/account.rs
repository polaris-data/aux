use aux_rs_core::{
    HttpRequest, HttpResponse, HttpTransport, TransportError, WebSocketSubscription,
};
use aux_rs_hyperliquid::HyperliquidClient;
use aux_rs_hyperliquid::{
    Address, Event, Hyperliquid, OpenOrdersRequest, Subscription, UserStateRequest,
};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};

const STATE: &str = include_str!("fixtures/account.json");
const ORDERS: &str = include_str!("fixtures/open_orders.json");
#[derive(Clone, Default)]
struct MockHttp(Arc<Mutex<Vec<HttpRequest>>>);
impl HttpTransport for MockHttp {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        let value: Value = serde_json::from_slice(&request.body).unwrap();
        let body = match value["type"].as_str().unwrap() {
            "clearinghouseState" => STATE,
            "openOrders" => ORDERS,
            _ => panic!("unexpected endpoint"),
        };
        self.0.lock().unwrap().push(request);
        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: body.as_bytes().to_vec(),
        })
    }
}

#[tokio::test]
async fn public_account_reads_use_explicit_account_addresses_and_keep_exact_positions() {
    let transport = MockHttp::default();
    let user: Address = "0xaAbBcCdDeEfF0011223344556677889900112233"
        .parse()
        .unwrap();
    let client = HyperliquidClient::with_transport(Hyperliquid::testnet(), transport.clone());
    let state = client
        .clearinghouse_state(UserStateRequest {
            user,
            dex: Some("xyz".into()),
        })
        .await
        .unwrap();
    assert_eq!(state.withdrawable, "980.0000");
    let position = &state.asset_positions[0].position;
    assert_eq!(position.szi, "-0.0335");
    assert_eq!(position.entry_px.as_deref(), Some("3000.10"));
    assert_eq!(position.liquidation_px, None);
    let orders = client
        .open_orders(OpenOrdersRequest::new(user))
        .await
        .unwrap();
    assert_eq!(orders[0].oid, 9_007_199_254_740_993);
    assert_eq!(orders[0].sz, "0.0100");
    let requests = transport.0.lock().unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&requests[0].body).unwrap(),
        json!({"type":"clearinghouseState","user":user,"dex":"xyz"})
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&requests[1].body).unwrap(),
        json!({"type":"openOrders","user":user})
    );
}

#[test]
fn native_account_events_and_routing_preserve_user_and_dex_context() {
    let user = Address::from_bytes([1; 20]);
    let other = Address::from_bytes([2; 20]);
    let state = Subscription::ClearinghouseState {
        user,
        dex: Some("xyz".into()),
    };
    let text = json!({"channel":"clearinghouseState","data":{"dex":"xyz","user":user,"clearinghouseState":serde_json::from_str::<Value>(STATE).unwrap()}}).to_string();
    assert!(state.matches(&text));
    assert!(
        !Subscription::ClearinghouseState {
            user: other,
            dex: Some("xyz".into())
        }
        .matches(&text)
    );
    assert!(!Subscription::ClearinghouseState { user, dex: None }.matches(&text));
    assert!(
        matches!(Event::decode(&text), Ok(Event::ClearinghouseState(event)) if event.user == user && event.clearinghouse_state.asset_positions[0].position.szi == "-0.0335")
    );
    let text = json!({"channel":"openOrders","data":{"dex":"","user":user,"orders":serde_json::from_str::<Value>(ORDERS).unwrap()}}).to_string();
    assert!(
        matches!(Event::decode(&text), Ok(Event::OpenOrders(event)) if event.orders[0].oid == 9_007_199_254_740_993)
    );
    let text = json!({"channel":"orderUpdates","data":[{"order":serde_json::from_str::<Value>(ORDERS).unwrap()[0],"status":"canceled","statusTimestamp":1754450974232u64}]}).to_string();
    assert!(
        matches!(Event::decode(&text), Ok(Event::OrderUpdates(updates)) if updates[0].order.orig_sz.as_deref() == Some("0.0200"))
    );
    let fills = Subscription::UserFills {
        user,
        aggregate_by_time: None,
    };
    let text = json!({"channel":"userFills","data":{"isSnapshot":true,"user":user,"fills":[]}})
        .to_string();
    assert!(fills.matches(&text));
    assert!(
        !Subscription::UserFills {
            user: other,
            aggregate_by_time: None
        }
        .matches(&text)
    );
    assert!(
        matches!(Event::decode(&text), Ok(Event::UserFills(event)) if event.is_snapshot == Some(true))
    );
    assert_eq!(
        fills.key(),
        Subscription::UserFills {
            user,
            aggregate_by_time: Some(false)
        }
        .key()
    );
    assert_eq!(
        Subscription::OpenOrders { user, dex: None }.key(),
        Subscription::OpenOrders {
            user,
            dex: Some("".into())
        }
        .key()
    );
}

#[test]
fn account_and_fill_aggregation_scopes_cannot_be_mixed() {
    let user = Address::from_bytes([1; 20]);
    let other = Address::from_bytes([2; 20]);
    let updates = Subscription::OrderUpdates { user };
    assert_ne!(
        updates.connection_scope(),
        Subscription::OrderUpdates { user: other }.connection_scope()
    );
    assert_eq!(
        updates.connection_scope(),
        Subscription::OpenOrders { user, dex: None }.connection_scope()
    );
    assert_ne!(
        updates.connection_scope(),
        Subscription::Trades { coin: "ETH".into() }.connection_scope()
    );
    assert_ne!(
        Subscription::UserFills {
            user,
            aggregate_by_time: None
        }
        .connection_scope(),
        Subscription::UserFills {
            user,
            aggregate_by_time: Some(true)
        }
        .connection_scope()
    );
}
