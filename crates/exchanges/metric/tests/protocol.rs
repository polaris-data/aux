use aux_rs_core::{Error as ClientError, HttpRequest, HttpResponse, HttpTransport, TransportError};
use aux_rs_metric as m;
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use zeroize::Zeroizing;
#[derive(Clone, Default)]
struct Mock {
    responses: Arc<Mutex<VecDeque<HttpResponse>>>,
    requests: Arc<Mutex<Vec<(HttpRequest, bool)>>>,
}
impl Mock {
    fn queue(&self, status: u16, v: Value) {
        self.responses.lock().unwrap().push_back(HttpResponse {
            status,
            headers: vec![],
            body: serde_json::to_vec(&v).unwrap(),
        });
    }
    fn last(&self) -> (HttpRequest, bool) {
        self.requests.lock().unwrap().last().unwrap().clone()
    }
    async fn run(&self, r: HttpRequest, safe: bool) -> Result<HttpResponse, TransportError> {
        self.requests.lock().unwrap().push((r, safe));
        Ok(self
            .responses
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected request"))
    }
}
impl HttpTransport for Mock {
    async fn execute(&self, r: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.run(r, false).await
    }
    async fn execute_read(&self, r: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.run(r, true).await
    }
}
fn first(source: &str) -> Value {
    serde_json::from_str::<Vec<Value>>(source)
        .unwrap()
        .remove(0)
}
#[tokio::test]
async fn all_eight_http_contracts_preserve_native_data_and_retry_semantics() {
    let t = Mock::default();
    let c = m::MetricClient::with_transport(m::Metric::new(1).unwrap(), t.clone());
    t.queue(
        200,
        serde_json::from_str(include_str!("fixtures/chains.json")).unwrap(),
    );
    assert!(!c.chains().await.unwrap().chains.is_empty());
    assert!(t.last().1);
    assert!(!t.last().0.headers.iter().any(|(k, _)| k == "authorization"));
    t.queue(
        200,
        serde_json::from_str(include_str!("fixtures/factory_settings.json")).unwrap(),
    );
    assert!(
        c.factory_settings()
            .await
            .unwrap()
            .fields
            .contains_key("chains")
    );
    for cross in [false, true] {
        t.queue(200, first(include_str!("fixtures/metadata.json")));
        let r = m::MetadataRequest {
            count: Some(1),
            include24h: Some(true),
            min_tvl_fiat: Some(0.into()),
            ..Default::default()
        };
        let response = if cross {
            c.cross_chain_metadata(r).await.unwrap()
        } else {
            c.metadata(r).await.unwrap()
        };
        assert!(!response.data.is_empty());
        let (url, safe) = t.last();
        assert!(safe);
        let u = url::Url::parse(&url.url).unwrap();
        let q = u.query_pairs().collect::<std::collections::HashMap<_, _>>();
        assert_eq!(q["minTvlFiat"], "0");
        assert_eq!(q["include24h"], "true");
    }
    let pool = "0x1111111111111111111111111111111111111111";
    t.queue(200, first(include_str!("fixtures/bid_ask.json")));
    let b = c.bid_ask(pool, Some(false)).await.unwrap();
    assert_eq!(b.bid_adj.as_deref(), Some("49940417631045696435290"));
    assert!(!b.depth.unwrap().bids.is_empty());
    t.queue(200, first(include_str!("fixtures/batch_bid_ask.json")));
    let b = c
        .batch_bid_ask(m::BatchBidAskRequest {
            pools: vec![pool.into()],
            depth: Some(false),
        })
        .await
        .unwrap();
    assert!(b.results[0].quote.is_some());
    assert_eq!(b.results[1].error.as_ref().unwrap().status, 404);
    assert!(t.last().1);
    t.queue(200, first(include_str!("fixtures/route.json")));
    let r = c
        .route(m::RouteRequest {
            token_in: pool.into(),
            token_out: "0x2222222222222222222222222222222222222222".into(),
            trade_type: None,
            amount_in: Some("50000000000000000".into()),
            amount_out: None,
            max_routes: Some(1),
            pool_address: None,
        })
        .await
        .unwrap();
    assert_eq!(r.amount_out, "136120897");
    assert!(t.last().1);
    let c = c.with_auth(m::TradingKey::new(Zeroizing::new("test-only-key".into())).unwrap());
    t.queue(200, first(include_str!("fixtures/pamm_quote.json")));
    let response = c
        .pamm_quote(m::PammQuoteRequest {
            tx: "0x".into(),
            block_number: 7,
            replacement_uuid: "0x00000000000000000000000000000001".into(),
            replacement_seq_number: 2,
            disable_cross_region_sharing: None,
            response_timeout_ms: Some(0),
            max_timestamp: None,
            always_include: None,
            quote_address: None,
            pool_addresses: None,
            asset_pairs: None,
        })
        .await
        .unwrap();
    assert!(!response.providers.is_empty());
    let (request, safe) = t.last();
    assert!(!safe);
    assert!(
        request
            .headers
            .iter()
            .any(|(k, v)| k == "authorization" && v == "Bearer test-only-key")
    );
    assert!(!format!("{request:?}").contains("test-only-key"));
}
#[tokio::test]
async fn unavailable_prices_errors_and_invalid_inputs_are_explicit() {
    let t = Mock::default();
    let c = m::MetricClient::with_transport(m::Metric::new(1).unwrap(), t.clone());
    let v: Vec<Value> = serde_json::from_str(include_str!("fixtures/bid_ask.json")).unwrap();
    t.queue(200, v[1].clone());
    let b = c
        .bid_ask("0x1111111111111111111111111111111111111111", None)
        .await
        .unwrap();
    assert_eq!(b.price_provider_status, "feed_down");
    assert_eq!(b.bid_adj.as_deref(), Some("0"));
    t.queue(422, json!({"code":"no_route","error":"native-message"}));
    assert!(matches!(
        c.route(m::RouteRequest {
            token_in: "0x1111111111111111111111111111111111111111".into(),
            token_out: "0x2222222222222222222222222222222222222222".into(),
            trade_type: None,
            amount_in: Some("1".into()),
            amount_out: None,
            max_routes: None,
            pool_address: None
        })
        .await,
        Err(ClientError::Exchange(m::Error::Api { status: 422, .. }))
    ));
    let count = t.requests.lock().unwrap().len();
    assert!(
        c.batch_bid_ask(m::BatchBidAskRequest {
            pools: vec![],
            depth: None
        })
        .await
        .is_err()
    );
    assert!(c.bid_ask("../../credentials", None).await.is_err());
    assert_eq!(t.requests.lock().unwrap().len(), count);
    let e = m::Error::Api {
        status: 401,
        body: json!({"error":"test-secret"}),
    };
    assert!(!format!("{e:?}").contains("test-secret"));
    assert!(m::TradingKey::new(Zeroizing::new("bad\nkey".into())).is_err());
}
