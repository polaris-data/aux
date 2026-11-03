use aux_rs_core::{
    Endpoint, Error as ClientError, Exchange, HttpRequest, HttpResponse, HttpTransport,
};
use aux_rs_monaco::*;
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};
use zeroize::Zeroizing;
const PAIR: &str = "550e8400-e29b-41d4-a716-446655440000";
const ORDER: &str = "123e4567-e89b-12d3-a456-426614174000";
type Responses = Arc<Mutex<VecDeque<(u16, Vec<u8>)>>>;
#[derive(Clone, Default)]
struct Recorder {
    requests: Arc<Mutex<Vec<HttpRequest>>>,
    responses: Responses,
    reads: Arc<AtomicUsize>,
}
impl Recorder {
    fn respond(&self, status: u16, body: Value) {
        self.responses
            .lock()
            .unwrap()
            .push_back((status, serde_json::to_vec(&body).unwrap()));
    }
}
impl HttpTransport for Recorder {
    async fn execute(&self, r: HttpRequest) -> Result<HttpResponse, aux_rs_core::TransportError> {
        self.requests.lock().unwrap().push(r);
        let (status, body) = self
            .responses
            .lock()
            .unwrap()
            .pop_front()
            .expect("fixture response");
        Ok(HttpResponse {
            status,
            body,
            headers: vec![],
        })
    }
    async fn execute_read(
        &self,
        r: HttpRequest,
    ) -> Result<HttpResponse, aux_rs_core::TransportError> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        self.execute(r).await
    }
}
fn authenticated(t: Recorder) -> MonacoClient<Recorder, MonacoAuth<LocalSessionSigner>> {
    let e = Monaco::testnet();
    let signer = LocalSessionSigner::from_bytes(Zeroizing::new([1; 32]));
    let key = aux_rs_core::hex::encode(signer.public_key());
    let a = MonacoAuth::restore(
        e.clone(),
        signer,
        SessionInfo {
            address: format!("0x{}", "11".repeat(20)),
            client_id: "fixture".into(),
            session_public_key: key,
            expires_at: unix_millis().unwrap() / 1000 + 3600,
            chain_id: 1328,
        },
        RequestSigning::Legacy,
    )
    .unwrap();
    MonacoClient::with_transport(e, t).with_auth(a)
}
fn market() -> TradingPairData {
    serde_json::from_value(json!({"id":PAIR,"tickSize":"0.01","quantityStepSize":"0.001","minOrderSize":"0.001","maxOrderSize":"1000","marketType":"MARGIN","minLeverage":"1","maxLeverage":"50"})).unwrap()
}
fn order() -> CreateOrderRequest {
    let mut r = CreateOrderRequest::limit(PAIR, "BUY", "100.0100", "1.000");
    r.leverage = Some("5".into());
    r.idempotency_key = Some("saved-key".into());
    r
}
#[test]
fn endpoints_normalize_api_bases_validate_ids_and_preserve_query_order() {
    for base in [
        "https://example.com",
        "https://example.com/api/",
        "https://example.com/api/v1/",
    ] {
        let e = Monaco::custom(base, "wss://example.com/ws", 1328).unwrap();
        assert_eq!(e.http_url(), "https://example.com");
        let r = TradingPairsRequest {
            page: Some(2),
            page_size: Some(20),
            base_token: Some("A/B +€".into()),
            ..Default::default()
        }
        .request(&e)
        .unwrap();
        assert_eq!(
            r.url,
            "https://example.com/api/v1/market/pairs?page=2&pageSize=20&baseToken=A%2FB%20%2B%E2%82%AC"
        );
        assert!(
            r.headers
                .iter()
                .any(|(n, v)| n == "User-Agent" && v.starts_with("aux-rs-monaco/"))
        );
    }
    assert!(Monaco::custom("https://user:secret@host", "wss://host/ws", 1328).is_err());
    assert!(Monaco::custom("https://host?token=secret", "wss://host/ws", 1328).is_err());
    assert!(
        OrderBookRequest {
            trading_pair_id: "../../orders".into(),
            ..Default::default()
        }
        .request(&Monaco::testnet())
        .is_err()
    );
    assert!(
        TradingPairsRequest {
            page_size: Some(101),
            ..Default::default()
        }
        .request(&Monaco::testnet())
        .is_err()
    );
}
#[test]
fn exact_precision_and_native_product_rules_are_checked_without_rounding() {
    let m = market();
    let mut r = order();
    assert!(r.validate(&m).is_ok());
    for bad in ["1e-3", "NaN", "0", "-1", "1.0001"] {
        r.quantity = bad.into();
        assert!(r.validate(&m).is_err(), "{bad}");
    }
    r = order();
    r.price = Some("100.001".into());
    assert!(r.validate(&m).is_err());
    r = order();
    r.idempotency_key = Some("a b".into());
    assert!(r.validate(&m).is_err());
    r = order();
    r.post_only = Some(true);
    r.time_in_force = Some("IOC".into());
    assert!(r.validate(&m).is_err());
    r = order();
    r.trading_mode = Some("SPOT".into());
    assert!(r.validate(&m).is_err());
    r = order();
    r.leverage = None;
    assert!(r.validate(&m).is_err());
    r.reduce_only = Some(true);
    assert!(r.validate(&m).is_ok());
    let mut spot = m.clone();
    spot.market_type = Some("SPOT".into());
    r = CreateOrderRequest::limit(PAIR, "SELL", "2.00", "1.000");
    assert!(r.validate(&spot).is_ok());
    r = CreateOrderRequest::market(PAIR, "BUY", "1");
    r.time_in_force = Some("IOC".into());
    assert!(r.validate(&spot).is_err());
}
#[tokio::test]
async fn public_and_protected_reads_have_native_paths_and_only_public_reads_use_retry_executor() {
    let t = Recorder::default();
    let c = authenticated(t.clone());
    t.respond(200,json!({"tradingPairs":[{"id":PAIR,"tickSize":"0.0100","quantityStepSize":"0.001","futureField":9007199254740993u64}],"total":1}));
    let markets = c.trading_pairs(Default::default()).await.unwrap();
    let m = &markets.trading_pairs.unwrap()[0];
    assert_eq!(m.tick_size.as_deref(), Some("0.0100"));
    assert_eq!(m.extra["futureField"], 9007199254740993u64);
    macro_rules! read {
        ($method:ident,$r:expr,$path:expr) => {{
            t.respond(200, json!({}));
            c.$method($r).await.unwrap();
            assert!(
                t.requests
                    .lock()
                    .unwrap()
                    .last()
                    .unwrap()
                    .url
                    .contains($path)
            );
        }};
    }
    read!(
        trading_pair,
        TradingPairRequest {
            trading_pair_id: PAIR.into()
        },
        &format!("/market/pairs/{PAIR}")
    );
    read!(
        market_metadata,
        MarketMetadataRequest {
            trading_pair_id: PAIR.into()
        },
        "/metadata"
    );
    read!(
        order_book,
        OrderBookRequest {
            trading_pair_id: PAIR.into(),
            ..Default::default()
        },
        "/orderbook/"
    );
    read!(
        trades,
        TradesRequest {
            trading_pair_id: PAIR.into(),
            ..Default::default()
        },
        "/trades/"
    );
    read!(
        mark_price,
        MarkPriceRequest {
            trading_pair_id: PAIR.into()
        },
        "/mark-price"
    );
    read!(
        index_price,
        IndexPriceRequest {
            trading_pair_id: PAIR.into()
        },
        "/index-price"
    );
    read!(
        funding,
        FundingRequest {
            trading_pair_id: PAIR.into()
        },
        "/funding"
    );
    read!(
        funding_history,
        FundingHistoryRequest {
            trading_pair_id: PAIR.into(),
            ..Default::default()
        },
        "/funding/history"
    );
    read!(
        balances,
        BalancesRequest {
            page_token: Some("a/b+".into()),
            ..Default::default()
        },
        "pageToken=a%2Fb%2B"
    );
    read!(
        parent_margin_account,
        Default::default(),
        "/parent-margin-account"
    );
    read!(margin_accounts, Default::default(), "/margin/accounts");
    read!(positions, Default::default(), "/positions");
    read!(
        orders,
        OrdersRequest {
            status: Some("FILLED,PARTIALLY_FILLED".into()),
            ..Default::default()
        },
        "status=FILLED%2CPARTIALLY_FILLED"
    );
    read!(
        order,
        OrderRequest {
            order_id: ORDER.into()
        },
        &format!("/orders/{ORDER}")
    );
    assert_eq!(t.reads.load(Ordering::SeqCst), 9);
    for (i, r) in t.requests.lock().unwrap().iter().enumerate() {
        assert_eq!(
            r.headers.iter().any(|(n, _)| n == "X-Monaco-Signature"),
            i >= 9
        );
    }
}
#[tokio::test]
async fn writes_preserve_idempotency_native_batch_outcomes_and_never_replay_ambiguous_responses() {
    let t = Recorder::default();
    let c = authenticated(t.clone());
    t.respond(200,json!({"orderId":ORDER,"status":"SUCCESS","matchResult":{"status":"REJECTED","reason":"FOK_UNFILLED"}}));
    let result = c.place_order(&market(), order()).await.unwrap();
    assert_eq!(
        result.match_result.unwrap().status.as_deref(),
        Some("REJECTED")
    );
    let submitted: Value = serde_json::from_slice(&t.requests.lock().unwrap()[0].body).unwrap();
    assert_eq!(submitted["idempotencyKey"], "saved-key");
    assert_eq!(submitted["price"], "100.0100");
    t.respond(503, json!({"code":"ORDER_SUBMISSION_UNAVAILABLE"}));
    assert!(matches!(
        c.place_order(&market(), order()).await,
        Err(ClientError::Exchange(Error::OutcomeUncertain { .. }))
    ));
    assert_eq!(t.requests.lock().unwrap().len(), 2);
    t.respond(
        409,
        json!({"code":"IDEMPOTENCY_KEY_CONFLICT","message":"conflict"}),
    );
    assert!(
        matches!(c.place_order(&market(),order()).await,Err(ClientError::Exchange(Error::Api {status:Some(409),body})) if body["code"]=="IDEMPOTENCY_KEY_CONFLICT")
    );
    let item: BatchCreateOrderItem =
        serde_json::from_value(serde_json::to_value(order()).unwrap()).unwrap();
    t.respond(200,json!({"results":[{"success":true,"orderId":ORDER},{"success":false,"error":{"code":"POST_ONLY_WOULD_CROSS","message":"cross"}}],"totalRequested":2,"totalSucceeded":1,"totalFailed":1}));
    let batch = c
        .batch_create(
            &[market()],
            BatchCreateOrdersRequest {
                orders: vec![item.clone(), item],
            },
        )
        .await
        .unwrap();
    assert_eq!(
        batch.results.unwrap()[1]
            .error
            .as_ref()
            .unwrap()
            .code
            .as_deref(),
        Some("POST_ONLY_WOULD_CROSS")
    );
    t.respond(200, json!({}));
    c.replace_order(
        &market(),
        ORDER,
        ReplaceOrderRequest {
            price: Some("99.00".into()),
            ..Default::default()
        },
    )
    .await
    .unwrap();
    t.respond(200, json!({}));
    c.cancel_order(CancelOrderRequest {
        order_id: ORDER.into(),
        ..Default::default()
    })
    .await
    .unwrap();
    t.respond(200, json!({}));
    c.batch_replace(
        &[market()],
        BatchReplaceOrdersRequest {
            orders: vec![BatchReplaceOrderItem {
                order_id: ORDER.into(),
                price: Some("99".into()),
                ..Default::default()
            }],
        },
    )
    .await
    .unwrap();
    t.respond(200, json!({}));
    c.batch_cancel(BatchCancelOrdersRequest {
        order_ids: vec![ORDER.into()],
    })
    .await
    .unwrap();
    t.respond(200, json!({}));
    c.cancel_all(Some(PAIR)).await.unwrap();
    assert!(
        t.requests
            .lock()
            .unwrap()
            .last()
            .unwrap()
            .url
            .ends_with(PAIR)
    );
    t.respond(200, json!({}));
    c.cancel_all(None).await.unwrap();
    assert!(t.requests.lock().unwrap().last().unwrap().body.is_empty());
    assert_eq!(t.reads.load(Ordering::SeqCst), 0);
    t.responses
        .lock()
        .unwrap()
        .push_back((202, b"{truncated".to_vec()));
    assert!(matches!(
        c.place_order(&market(), order()).await,
        Err(ClientError::Exchange(Error::OutcomeUncertain {
            status: Some(202),
            ..
        }))
    ));
}
#[tokio::test]
async fn login_binds_wallet_session_and_expiry_refresh_and_revoke_share_state() {
    let t = Recorder::default();
    let public = MonacoClient::with_transport(Monaco::testnet(), t.clone());
    let wallet =
        LocalWalletSigner::from_hex(Zeroizing::new(format!("{}1", "0".repeat(63)))).unwrap();
    let session = LocalSessionSigner::from_bytes(Zeroizing::new([2; 32]));
    let pubkey = aux_rs_core::hex::encode(session.public_key());
    let now = unix_millis().unwrap() / 1000;
    t.respond(
        200,
        json!({"nonce":"nonce","message":format!("Authorize {pubkey}"),"expiresAt":now+300}),
    );
    t.respond(
        200,
        json!({"expiresAt":now+3600,"user":{"address":wallet.address()}}),
    );
    let auth = public
        .login(&wallet, "client", session, Default::default())
        .await
        .unwrap();
    let clone = auth.clone();
    let client = public.with_auth(auth);
    {
        let requests = t.requests.lock().unwrap();
        let verify: Value = serde_json::from_slice(&requests[1].body).unwrap();
        assert_eq!(verify["sessionPublicKey"], pubkey);
        assert_eq!(verify["chainId"], "1328");
    }
    t.respond(200, json!({"expiresAt":now+7200}));
    client.refresh_session().await.unwrap();
    assert_eq!(clone.session_info().expires_at, now + 7200);
    t.respond(200, json!({"message":"revoked"}));
    client.revoke_session().await.unwrap();
    assert!(!clone.is_active());
    let sent = t.requests.lock().unwrap().len();
    assert!(client.orders(Default::default()).await.is_err());
    assert_eq!(t.requests.lock().unwrap().len(), sent);
}

#[test]
fn captured_staging_market_data_decodes_with_exact_native_values() {
    let markets: ListTradingPairsResponse =
        serde_json::from_str(include_str!("fixtures/markets.json")).unwrap();
    let pairs = markets.trading_pairs.unwrap();
    assert!(
        pairs
            .iter()
            .any(|p| p.market_type.as_deref() == Some("SPOT"))
    );
    assert!(
        pairs
            .iter()
            .any(|p| p.market_type.as_deref() == Some("MARGIN"))
    );
    let book: GetOrderbookResponse =
        serde_json::from_str(include_str!("fixtures/book.json")).unwrap();
    assert!(book.data.unwrap().bids.unwrap()[0].price.is_some());
    let trades: GetTradesResponse =
        serde_json::from_str(include_str!("fixtures/trades.json")).unwrap();
    assert!(
        trades.trades.unwrap()[0]
            .data
            .as_ref()
            .unwrap()
            .quantity
            .is_some()
    );
    let _: GetFundingStateResponse =
        serde_json::from_str(include_str!("fixtures/funding.json")).unwrap();
    let _: GetMarkPriceResponse = serde_json::from_str(include_str!("fixtures/mark.json")).unwrap();
    let _: GetIndexPriceResponse =
        serde_json::from_str(include_str!("fixtures/index.json")).unwrap();
}

#[tokio::test]
async fn cancelling_revocation_still_invalidates_all_local_clones() {
    struct Block;
    impl HttpTransport for Block {
        async fn execute(
            &self,
            _: HttpRequest,
        ) -> Result<HttpResponse, aux_rs_core::TransportError> {
            std::future::pending().await
        }
    }
    let a = authenticated(Recorder::default()).auth().clone();
    let clone = a.clone();
    let client = MonacoClient::with_transport(Monaco::testnet(), Block).with_auth(a);
    assert!(
        tokio::time::timeout(
            std::time::Duration::from_millis(25),
            client.revoke_session()
        )
        .await
        .is_err()
    );
    assert!(!clone.is_active());
}

#[test]
fn subscription_channels_are_validated_and_canonical_for_native_live_routing() {
    assert_eq!(
        Subscription::Trades {
            trading_pair_id: "*".into()
        }
        .channel()
        .unwrap(),
        "trades:*"
    );
    let upper = PAIR.to_ascii_uppercase();
    assert_eq!(
        Subscription::Orderbook {
            trading_pair_id: upper.clone(),
            mode: "SPOT".into(),
            magnitude: None,
            denomination: Some("quote".into())
        }
        .channel()
        .unwrap(),
        format!("orderbook:{PAIR}:SPOT:*:quote")
    );
    assert_eq!(
        Subscription::Bbo {
            trading_pair_id: upper.clone(),
            mode: "MARGIN".into()
        }
        .channel()
        .unwrap(),
        format!("bbo:{PAIR}:MARGIN")
    );
    assert_eq!(
        AccountSubscription::Orders {
            trading_pair_id: Some(upper),
            mode: Some("MARGIN".into())
        }
        .channel()
        .unwrap(),
        format!("orders:{PAIR}:MARGIN")
    );
    assert!(
        Subscription::Depth {
            trading_pair_id: "*".into(),
            mode: "MARGIN".into()
        }
        .channel()
        .is_err()
    );
    assert!(
        Subscription::Orderbook {
            trading_pair_id: PAIR.into(),
            mode: "SPOT".into(),
            magnitude: Some("0.0000007".into()),
            denomination: None
        }
        .channel()
        .is_err()
    );
}
