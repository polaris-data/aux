use aux_rs_core::{Endpoint, HttpResponse};
use aux_rs_lighter::*;
use serde_json::{Value, json};
fn response(body: &str) -> HttpResponse {
    HttpResponse {
        status: 200,
        headers: vec![],
        body: body.as_bytes().to_vec(),
    }
}

#[test]
fn native_public_fixtures_and_request_validation() {
    let network = Lighter::testnet();
    let books = OrderBooksRequest::decode(response(include_str!("fixtures/markets.json"))).unwrap();
    assert_eq!(books.order_books[0].market_id, 4099);
    assert_eq!(books.order_books[0].scale_price("3.0000").unwrap(), 30000);
    assert!(books.order_books[0].scale_price("3.00001").is_err());
    let book =
        OrderBookOrdersRequest::decode(response(include_str!("fixtures/book.json"))).unwrap();
    assert_eq!(book.bids[0].order_index, 577305177233555425);
    assert_eq!(book.bids[0].price, "3.0000");
    let trades =
        RecentTradesRequest::decode(response(include_str!("fixtures/trades.json"))).unwrap();
    assert_eq!(trades.trades[0].price, "3.8000");
    let accounts = AccountRequest::decode(response(include_str!("fixtures/account.json"))).unwrap();
    assert_eq!(accounts.accounts[0].collateral, "3000000000.000000");
    assert_eq!(
        OrderBookOrdersRequest {
            market_id: 4096,
            limit: 250
        }
        .request(&network)
        .unwrap()
        .url,
        "https://testnet.zklighter.elliot.ai/api/v1/orderBookOrders?market_id=4096&limit=250"
    );
    assert!(
        OrderBookOrdersRequest {
            market_id: 255,
            limit: 250
        }
        .request(&network)
        .is_err()
    );
    assert!(
        RecentTradesRequest {
            market_id: 4096,
            limit: 101
        }
        .request(&network)
        .is_err()
    );
    assert!(
        AccountRequest::L1Address("0xinvalid&auth=x".into())
            .request(&network)
            .is_err()
    );
    assert!(
        NextNonceRequest {
            account_index: 1,
            api_key_index: 255
        }
        .request(&network)
        .is_err()
    );
    assert!(AccountRequest::Index(-1).request(&network).is_err());
}

#[test]
fn exact_fixed_point_never_rounds_or_overflows() {
    assert_eq!(scale_decimal("0.000000000000000001", 18).unwrap(), 1);
    assert_eq!(scale_decimal("18446744073709551615", 0).unwrap(), u64::MAX);
    assert_eq!(scale_decimal("3.5", 4).unwrap(), 35000);
    for input in [
        "1.00001", "-1", "+1", "1e3", "NaN", ".1", "1.", " 1", "1.2.3", "",
    ] {
        assert!(scale_decimal(input, 4).is_err(), "{input}");
    }
    assert!(scale_decimal("18446744073709551616", 0).is_err());
    assert!(scale_decimal("1", 20).is_err());
    // Unknown native fields can also contain monetary JSON numbers. Keep their
    // source precision, rather than converting through Value's f64 storage.
    let response = NextNonceRequest::decode(response(
        r#"{"code":200,"nonce":42,"future_amount":1.123456789012345678901234567890}"#,
    ))
    .unwrap();
    assert_eq!(
        response.extra["future_amount"].to_string(),
        "1.123456789012345678901234567890"
    );
}

#[test]
fn codes_rate_limits_and_credential_echo_are_preserved_but_not_formatted() {
    let mut r = response(r#"{"code":21104,"message":"secret-token-value","request_id":"r1"}"#);
    r.status = 429;
    let error = OrderBooksRequest::decode(r).unwrap_err();
    assert!(!format!("{error} {error:?}").contains("secret-token-value"));
    match error {
        Error::Api {
            code,
            message,
            extra,
            http_status,
        } => {
            assert_eq!(code, 21104);
            assert_eq!(http_status, Some(429));
            assert_eq!(message.as_deref(), Some("secret-token-value"));
            assert_eq!(extra["request_id"], "r1");
        }
        _ => panic!(),
    }
    assert!(matches!(
        OrderBooksRequest::decode(response("garbage")),
        Err(Error::Decode(_))
    ));
    let r = HttpResponse {
        status: 503,
        headers: vec![],
        body: b"gateway failure".to_vec(),
    };
    assert!(matches!(
        OrderBooksRequest::decode(r),
        Err(Error::Http { status: 503, .. })
    ));
}

#[test]
fn native_stream_snapshots_deltas_errors_and_unknown_events() {
    for (fixture, kind) in [
        (include_str!("fixtures/book_stream.json"), "book"),
        (include_str!("fixtures/trades_stream.json"), "trades"),
        (include_str!("fixtures/account_stream.json"), "account"),
    ] {
        let event = Event::decode(fixture).unwrap();
        assert!(event.is_snapshot());
        match event {
            Event::OrderBook(v) => {
                assert_eq!(kind, "book");
                assert_eq!(v.order_book.bids[0].size, "11.68");
                assert_eq!(v.order_book.nonce, 3650251);
            }
            Event::Trades(_) => assert_eq!(kind, "trades"),
            Event::Account(v) => {
                assert_eq!(kind, "account");
                assert_eq!(v.assets["3"].balance, "3000000000.000000");
            }
            _ => panic!(),
        }
    }
    let mut delta: Value = serde_json::from_str(include_str!("fixtures/book_stream.json")).unwrap();
    delta["type"] = "update/order_book".into();
    delta["order_book"]["bids"][0]["size"] = "0.0000".into();
    let event = Event::decode(&delta.to_string()).unwrap();
    assert!(!event.is_snapshot());
    if let Event::OrderBook(book) = event {
        assert_eq!(book.order_book.bids[0].size, "0.0000");
    } else {
        panic!()
    }
    assert!(matches!(
        Event::decode(r#"{"error":{"code":20001,"message":"redact me"}}"#),
        Err(Error::Api { code: 20001, .. })
    ));
    assert!(Event::decode("{").is_err());
    assert!(Event::decode(r#"{"type":"update/order_book","channel":"order_book:4099"}"#).is_err());
    assert!(matches!(
        Event::decode(&json!({"type":"new-native-event","data":[1,2]}).to_string()),
        Ok(Event::Unknown(_))
    ));
}

#[test]
fn native_order_and_attribute_constraints() {
    let vectors: Value =
        serde_json::from_str(include_str!("fixtures/signing_vectors.json")).unwrap();
    let mut order: CreateOrder =
        serde_json::from_value(vectors["vectors"][0]["tx_info"].clone()).unwrap();
    assert!(order.validate().is_ok());
    order.time_in_force = 0;
    assert!(order.validate().is_err());
    order.order_expiry = 0;
    assert!(order.validate().is_ok());
    order.order_type = 1;
    assert!(order.validate().is_ok());
    order.reduce_only = 0;
    order.base_amount = 0;
    assert!(order.validate().is_err());
    order.reduce_only = 1;
    assert!(order.validate().is_ok());
    for map in [
        json!({"4":1}),
        json!({"2":50}),
        json!({"1":37,"2":50,"6":1}),
        json!({"6":3,"7":1}),
        json!({"8":281474976710656_i64}),
        json!({"9":0}),
    ] {
        let attrs: TxAttributes = serde_json::from_value(map).unwrap();
        assert!(attrs.validate().is_err());
    }
    assert!(
        CancelOrder {
            market_index: 0,
            index: (1i64 << 60)
        }
        .validate()
        .is_err()
    );
}
