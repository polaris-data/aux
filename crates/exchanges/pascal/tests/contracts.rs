// Generated named-method contracts; no exchange requests.
use aux_rs_core::{HttpRequest, HttpResponse, HttpTransport, TransportError};
use aux_rs_pascal as v;
use serde_json::Value;
use std::sync::{Arc, Mutex};
#[derive(Clone)]
struct Mock {
    response: Value,
    sent: Arc<Mutex<Vec<HttpRequest>>>,
}
impl HttpTransport for Mock {
    async fn execute(&self, r: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.sent.lock().unwrap().push(r);
        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: serde_json::to_vec(&self.response).unwrap(),
        })
    }
}
#[derive(Clone)]
struct Auth;
impl v::Authentication for Auth {
    fn identity(&self) -> String {
        "fixture".into()
    }
    async fn prepare(
        &self,
        _: &v::Pascal,
        _: &v::Operation,
        r: &mut HttpRequest,
    ) -> Result<(), v::Error> {
        r.headers.push(("x-fixture-auth".into(), "true".into()));
        Ok(())
    }
}
#[tokio::test]
async fn read_account_state_snapshot() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.account_state_snapshot"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::AccountStateSnapshotParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .account_state_snapshot(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/accounts/fixture"));
}
#[tokio::test]
async fn read_account_bio() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.account_bio"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::AccountBioParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.account_bio(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/accounts/fixture/bio"));
}
#[tokio::test]
async fn read_builder_stats() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.builder_stats"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::BuilderStatsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.builder_stats(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(
        parsed
            .path()
            .ends_with("/api/v1/accounts/fixture/builder-stats")
    );
}
#[tokio::test]
async fn read_deposit_address() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.deposit_address"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::DepositAddressParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.deposit_address(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(
        parsed
            .path()
            .ends_with("/api/v1/accounts/fixture/deposit-address")
    );
}
#[tokio::test]
async fn read_fill_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.fill_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::FillHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.fill_history(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/accounts/fixture/fills"));
}
#[tokio::test]
async fn read_pn_l_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.pn_l_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::PnLHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.pn_l_history(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/accounts/fixture/history"));
}
#[tokio::test]
async fn read_market_pn_l_movers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.market_pn_l_movers"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::MarketPnLMoversParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.market_pn_l_movers(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(
        parsed
            .path()
            .ends_with("/api/v1/accounts/fixture/pnl-movers")
    );
}
#[tokio::test]
async fn read_position_resolution_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.position_resolution_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::PositionResolutionHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .position_resolution_history(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(
        parsed
            .path()
            .ends_with("/api/v1/accounts/fixture/position-resolutions")
    );
}
#[tokio::test]
async fn read_referrals() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.referrals"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::ReferralsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.referrals(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(
        parsed
            .path()
            .ends_with("/api/v1/accounts/fixture/referrals")
    );
}
#[tokio::test]
async fn read_taker_discount_terms() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.taker_discount_terms"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::TakerDiscountTermsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .taker_discount_terms(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(
        parsed
            .path()
            .ends_with("/api/v1/accounts/fixture/taker-discount-terms")
    );
}
#[tokio::test]
async fn read_trading_keys() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.trading_keys"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::TradingKeysParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.trading_keys(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(
        parsed
            .path()
            .ends_with("/api/v1/accounts/fixture/trading-keys")
    );
}
#[tokio::test]
async fn read_transfer_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.transfer_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::TransferHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.transfer_history(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(
        parsed
            .path()
            .ends_with("/api/v1/accounts/fixture/transfers")
    );
}
#[tokio::test]
async fn read_order_books() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.order_books"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::OrderBooksParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.order_books(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/books"));
}
#[tokio::test]
async fn read_mark_price_candles() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.mark_price_candles"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::MarkPriceCandlesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.mark_price_candles(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/mark-price-candles"));
}
#[tokio::test]
async fn read_batch_mark_price_candles() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.batch_mark_price_candles"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::BatchMarkPriceCandlesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .batch_mark_price_candles(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/mark-price-candles/batch"));
}
#[tokio::test]
async fn read_list_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.list_markets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::ListMarketsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.list_markets(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/markets"));
}
#[tokio::test]
async fn read_resolved_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.resolved_markets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::ResolvedMarketsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.resolved_markets(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/markets/resolved"));
}
#[tokio::test]
async fn read_market_search() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.market_search"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::MarketSearchParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.market_search(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/markets/search"));
}
#[tokio::test]
async fn read_market_text_search() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.market_text_search"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::MarketTextSearchParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.market_text_search(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/markets/text-search"));
}
#[tokio::test]
async fn read_referral_code() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.referral_code"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::ReferralCodeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.referral_code(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/referral-codes/fixture"));
}
#[tokio::test]
async fn read_server_time() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.server_time"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::ServerTimeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.server_time(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/time"));
}
#[tokio::test]
async fn read_top_holders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.top_holders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::TopHoldersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.top_holders(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/top-holders"));
}
#[tokio::test]
async fn read_trade_candles() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.trade_candles"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::TradeCandlesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.trade_candles(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/trade-candles"));
}
#[tokio::test]
async fn read_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.trades"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::TradesParams = serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.trades(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/trades"));
}
#[tokio::test]
async fn read_batch_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.batch_trades"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::BatchTradesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.batch_trades(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/trades/batch"));
}
#[tokio::test]
async fn read_trading_key_accounts() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.trading_key_accounts"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::TradingKeyAccountsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .trading_key_accounts(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(
        parsed
            .path()
            .ends_with("/api/v1/trading-keys/fixture/accounts")
    );
}
#[tokio::test]
async fn read_usernames() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.usernames"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::UsernamesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.usernames(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/usernames"));
}
#[tokio::test]
async fn read_closed_position_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["read.closed_position_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::read::ClosedPositionHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .closed_position_history(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Get);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(
        parsed
            .path()
            .ends_with("/api/v2/accounts/fixture/position-instances")
    );
}
#[tokio::test]
async fn write_set_an_account_bio() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["write.set_an_account_bio"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::write::SetAnAccountBioParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.set_an_account_bio(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/account-bios"));
}
#[tokio::test]
async fn write_cancel_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["write.cancel_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::write::CancelOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.cancel_orders(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/cancels"));
}
#[tokio::test]
async fn write_register_a_deposit_address() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["write.register_a_deposit_address"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::write::RegisterADepositAddressParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .register_a_deposit_address(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/deposit-addresses"));
}
#[tokio::test]
async fn write_mass_cancel_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["write.mass_cancel_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::write::MassCancelOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.mass_cancel_orders(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/mass-cancels"));
}
#[tokio::test]
async fn write_place_and_replace_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["write.place_and_replace_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::write::PlaceAndReplaceOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .place_and_replace_orders(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/orders"));
}
#[tokio::test]
async fn write_generate_a_referral_code() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["write.generate_a_referral_code"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::write::GenerateAReferralCodeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .generate_a_referral_code(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/referral-codes"));
}
#[tokio::test]
async fn write_apply_a_referral_code() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["write.apply_a_referral_code"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::write::ApplyAReferralCodeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .apply_a_referral_code(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/referral-codes/apply"));
}
#[tokio::test]
async fn write_claim_a_custom_referral_code() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["write.claim_a_custom_referral_code"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::write::ClaimACustomReferralCodeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .claim_a_custom_referral_code(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/referral-codes/custom"));
}
#[tokio::test]
async fn write_revoke_a_trading_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["write.revoke_a_trading_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::write::RevokeATradingKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .revoke_a_trading_key(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/trading-key-revocations"));
}
#[tokio::test]
async fn write_create_a_trading_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["write.create_a_trading_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::write::CreateATradingKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .create_a_trading_key(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/trading-keys"));
}
#[tokio::test]
async fn write_set_a_username() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["write.set_a_username"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::write::SetAUsernameParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.set_a_username(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/usernames"));
}
#[tokio::test]
async fn write_withdraw_collateral() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["write.withdraw_collateral"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PascalClient::with_transport(v::Pascal::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::write::WithdrawCollateralParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .withdraw_collateral(params)
        .await
        .unwrap_or_else(|e| {
            if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
                panic!("fixture decode: {d}");
            }
            panic!("{e}");
        });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Post);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/api/v1/withdrawals"));
}
