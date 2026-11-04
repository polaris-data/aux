// Generated named-method contracts; no exchange requests.
use aux_rs_core::{HttpRequest, HttpResponse, HttpTransport, TransportError};
use aux_rs_polymarket as v;
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
        _: &v::Polymarket,
        _: &v::Operation,
        r: &mut HttpRequest,
    ) -> Result<(), v::Error> {
        r.headers.push(("x-fixture-auth".into(), "true".into()));
        Ok(())
    }
}
#[tokio::test]
async fn clob_clob_delete_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_delete_api_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobDeleteApiKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_delete_api_key(params)
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
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/auth/api-key"));
}
#[tokio::test]
async fn clob_clob_create_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_create_api_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobCreateApiKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_create_api_key(params)
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
    assert!(parsed.path().ends_with("/auth/api-key"));
}
#[tokio::test]
async fn clob_clob_get_api_keys() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_api_keys"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetApiKeysParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_api_keys(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/auth/api-keys"));
}
#[tokio::test]
async fn clob_clob_get_closed_only_mode() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_closed_only_mode"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetClosedOnlyModeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_closed_only_mode(params)
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
    assert!(parsed.path().ends_with("/auth/ban-status/closed-only"));
}
#[tokio::test]
async fn clob_clob_revoke_builder_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_revoke_builder_api_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobRevokeBuilderApiKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_revoke_builder_api_key(params)
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
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/auth/builder-api-key"));
}
#[tokio::test]
async fn clob_clob_get_builder_api_keys() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_builder_api_keys"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetBuilderApiKeysParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_builder_api_keys(params)
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
    assert!(parsed.path().ends_with("/auth/builder-api-key"));
}
#[tokio::test]
async fn clob_clob_create_builder_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_create_builder_api_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobCreateBuilderApiKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_create_builder_api_key(params)
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
    assert!(parsed.path().ends_with("/auth/builder-api-key"));
}
#[tokio::test]
async fn clob_clob_derive_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_derive_api_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobDeriveApiKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_derive_api_key(params)
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
    assert!(parsed.path().ends_with("/auth/derive-api-key"));
}
#[tokio::test]
async fn clob_clob_get_balance_allowance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_balance_allowance"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetBalanceAllowanceParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_balance_allowance(params)
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
    assert!(parsed.path().ends_with("/balance-allowance"));
}
#[tokio::test]
async fn clob_clob_update_balance_allowance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_update_balance_allowance"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobUpdateBalanceAllowanceParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_update_balance_allowance(params)
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
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Put);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/balance-allowance"));
}
#[tokio::test]
async fn clob_clob_get_update_balance_allowance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_update_balance_allowance"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetUpdateBalanceAllowanceParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_update_balance_allowance(params)
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
    assert!(parsed.path().ends_with("/balance-allowance/update"));
}
#[tokio::test]
async fn clob_clob_get_batch_prices_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_batch_prices_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetBatchPricesHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_batch_prices_history(params)
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
    assert!(parsed.path().ends_with("/batch-prices-history"));
}
#[tokio::test]
async fn clob_clob_get_book() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_book"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetBookParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_book(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/book"));
}
#[tokio::test]
async fn clob_clob_get_books_get() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_books_get"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetBooksGetParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_books_get(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/books"));
}
#[tokio::test]
async fn clob_clob_get_books_post() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_books_post"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetBooksPostParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_books_post(params)
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
    assert!(parsed.path().ends_with("/books"));
}
#[tokio::test]
async fn clob_clob_get_builder_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_builder_trades"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetBuilderTradesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_builder_trades(params)
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
    assert!(parsed.path().ends_with("/builder/trades"));
}
#[tokio::test]
async fn clob_clob_cancel_all_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_cancel_all_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobCancelAllOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_cancel_all_orders(params)
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
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/cancel-all"));
}
#[tokio::test]
async fn clob_clob_cancel_market_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_cancel_market_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobCancelMarketOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_cancel_market_orders(params)
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
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/cancel-market-orders"));
}
#[tokio::test]
async fn clob_clob_get_clob_market_info() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_clob_market_info"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetClobMarketInfoParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_clob_market_info(params)
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
    assert!(parsed.path().ends_with("/clob-markets/fixture"));
}
#[tokio::test]
async fn clob_clob_get_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_order"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetOrderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_order(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/data/order/fixture"));
}
#[tokio::test]
async fn clob_clob_get_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_orders(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/data/orders"));
}
#[tokio::test]
async fn clob_clob_get_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_trades"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetTradesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_trades(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/data/trades"));
}
#[tokio::test]
async fn clob_clob_get_fee_rate() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_fee_rate"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetFeeRateParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_fee_rate(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/fee-rate"));
}
#[tokio::test]
async fn clob_clob_get_fee_rate_by_path() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_fee_rate_by_path"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetFeeRateByPathParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_fee_rate_by_path(params)
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
    assert!(parsed.path().ends_with("/fee-rate/fixture"));
}
#[tokio::test]
async fn clob_clob_send_heartbeat() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_send_heartbeat"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobSendHeartbeatParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_send_heartbeat(params)
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
    assert!(parsed.path().ends_with("/heartbeats"));
}
#[tokio::test]
async fn clob_clob_get_last_trade_price() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_last_trade_price"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetLastTradePriceParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_last_trade_price(params)
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
    assert!(parsed.path().ends_with("/last-trade-price"));
}
#[tokio::test]
async fn clob_clob_get_last_trades_prices_get() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_last_trades_prices_get"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetLastTradesPricesGetParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_last_trades_prices_get(params)
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
    assert!(parsed.path().ends_with("/last-trades-prices"));
}
#[tokio::test]
async fn clob_clob_get_last_trades_prices_post() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_last_trades_prices_post"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetLastTradesPricesPostParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_last_trades_prices_post(params)
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
    assert!(parsed.path().ends_with("/last-trades-prices"));
}
#[tokio::test]
async fn clob_clob_get_market_by_token() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_market_by_token"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetMarketByTokenParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_market_by_token(params)
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
    assert!(parsed.path().ends_with("/markets-by-token/fixture"));
}
#[tokio::test]
async fn clob_clob_get_markets_live_activity() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_markets_live_activity"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetMarketsLiveActivityParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_markets_live_activity(params)
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
    assert!(parsed.path().ends_with("/markets/live-activity"));
}
#[tokio::test]
async fn clob_clob_get_market_live_activity() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_market_live_activity"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetMarketLiveActivityParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_market_live_activity(params)
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
    assert!(parsed.path().ends_with("/markets/live-activity/fixture"));
}
#[tokio::test]
async fn clob_clob_get_midpoint() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_midpoint"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetMidpointParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_midpoint(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/midpoint"));
}
#[tokio::test]
async fn clob_clob_get_midpoints_get() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_midpoints_get"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetMidpointsGetParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_midpoints_get(params)
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
    assert!(parsed.path().ends_with("/midpoints"));
}
#[tokio::test]
async fn clob_clob_get_midpoints_post() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_midpoints_post"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetMidpointsPostParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_midpoints_post(params)
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
    assert!(parsed.path().ends_with("/midpoints"));
}
#[tokio::test]
async fn clob_clob_get_neg_risk() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_neg_risk"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetNegRiskParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_neg_risk(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/neg-risk"));
}
#[tokio::test]
async fn clob_clob_get_neg_risk_by_path() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_neg_risk_by_path"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetNegRiskByPathParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_neg_risk_by_path(params)
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
    assert!(parsed.path().ends_with("/neg-risk/fixture"));
}
#[tokio::test]
async fn clob_clob_drop_notifications() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_drop_notifications"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobDropNotificationsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_drop_notifications(params)
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
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/notifications"));
}
#[tokio::test]
async fn clob_clob_get_notifications() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_notifications"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetNotificationsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_notifications(params)
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
    assert!(parsed.path().ends_with("/notifications"));
}
#[tokio::test]
async fn clob_clob_cancel_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_cancel_order"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobCancelOrderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_cancel_order(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/order"));
}
#[tokio::test]
async fn clob_clob_post_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_post_order"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobPostOrderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_post_order(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/order"));
}
#[tokio::test]
async fn clob_clob_get_order_scoring() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_order_scoring"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetOrderScoringParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_order_scoring(params)
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
    assert!(parsed.path().ends_with("/order-scoring"));
}
#[tokio::test]
async fn clob_clob_cancel_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_cancel_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobCancelOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_cancel_orders(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/orders"));
}
#[tokio::test]
async fn clob_clob_post_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_post_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobPostOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_post_orders(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/orders"));
}
#[tokio::test]
async fn clob_clob_get_orders_scoring() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_orders_scoring"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetOrdersScoringParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_orders_scoring(params)
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
    assert!(parsed.path().ends_with("/orders-scoring"));
}
#[tokio::test]
async fn clob_clob_post_orders_scoring() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_post_orders_scoring"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobPostOrdersScoringParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_post_orders_scoring(params)
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
    assert!(parsed.path().ends_with("/orders-scoring"));
}
#[tokio::test]
async fn clob_clob_get_price() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_price"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetPriceParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_price(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/price"));
}
#[tokio::test]
async fn clob_clob_get_prices_get() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_prices_get"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetPricesGetParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_prices_get(params)
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
    assert!(parsed.path().ends_with("/prices"));
}
#[tokio::test]
async fn clob_clob_get_prices_post() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_prices_post"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetPricesPostParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_prices_post(params)
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
    assert!(parsed.path().ends_with("/prices"));
}
#[tokio::test]
async fn clob_clob_get_prices_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_prices_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetPricesHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_prices_history(params)
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
    assert!(parsed.path().ends_with("/prices-history"));
}
#[tokio::test]
async fn clob_clob_get_current_rebated_fees() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_current_rebated_fees"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetCurrentRebatedFeesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_current_rebated_fees(params)
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
    assert!(parsed.path().ends_with("/rebates/current"));
}
#[tokio::test]
async fn clob_clob_get_current_rewards() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_current_rewards"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetCurrentRewardsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_current_rewards(params)
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
    assert!(parsed.path().ends_with("/rewards/markets/current"));
}
#[tokio::test]
async fn clob_clob_get_multi_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_multi_markets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetMultiMarketsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_multi_markets(params)
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
    assert!(parsed.path().ends_with("/rewards/markets/multi"));
}
#[tokio::test]
async fn clob_clob_get_raw_rewards_for_market() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_raw_rewards_for_market"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetRawRewardsForMarketParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_raw_rewards_for_market(params)
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
    assert!(parsed.path().ends_with("/rewards/markets/fixture"));
}
#[tokio::test]
async fn clob_clob_get_earnings_for_user_for_day() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_earnings_for_user_for_day"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetEarningsForUserForDayParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_earnings_for_user_for_day(params)
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
    assert!(parsed.path().ends_with("/rewards/user"));
}
#[tokio::test]
async fn clob_clob_get_user_earnings_and_markets_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_user_earnings_and_markets_config"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetUserEarningsAndMarketsConfigParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_user_earnings_and_markets_config(params)
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
    assert!(parsed.path().ends_with("/rewards/user/markets"));
}
#[tokio::test]
async fn clob_clob_get_reward_percentages_for_user() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_reward_percentages_for_user"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetRewardPercentagesForUserParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_reward_percentages_for_user(params)
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
    assert!(parsed.path().ends_with("/rewards/user/percentages"));
}
#[tokio::test]
async fn clob_clob_get_total_earnings_for_user_for_day() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_total_earnings_for_user_for_day"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetTotalEarningsForUserForDayParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_total_earnings_for_user_for_day(params)
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
    assert!(parsed.path().ends_with("/rewards/user/total"));
}
#[tokio::test]
async fn clob_clob_get_sampling_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_sampling_markets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetSamplingMarketsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_sampling_markets(params)
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
    assert!(parsed.path().ends_with("/sampling-markets"));
}
#[tokio::test]
async fn clob_clob_get_sampling_simplified_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_sampling_simplified_markets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetSamplingSimplifiedMarketsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_sampling_simplified_markets(params)
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
    assert!(parsed.path().ends_with("/sampling-simplified-markets"));
}
#[tokio::test]
async fn clob_clob_get_simplified_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_simplified_markets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetSimplifiedMarketsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_simplified_markets(params)
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
    assert!(parsed.path().ends_with("/simplified-markets"));
}
#[tokio::test]
async fn clob_clob_get_spread() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_spread"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetSpreadParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_spread(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/spread"));
}
#[tokio::test]
async fn clob_clob_get_spreads() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_spreads"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetSpreadsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_spreads(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/spreads"));
}
#[tokio::test]
async fn clob_clob_get_tick_size() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_tick_size"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetTickSizeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_tick_size(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/tick-size"));
}
#[tokio::test]
async fn clob_clob_get_tick_size_by_path() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_tick_size_by_path"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetTickSizeByPathParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_tick_size_by_path(params)
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
    assert!(parsed.path().ends_with("/tick-size/fixture"));
}
#[tokio::test]
async fn clob_clob_get_time() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_time"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetTimeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.clob_get_time(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/time"));
}
#[tokio::test]
async fn clob_clob_send_heartbeat_v1() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_send_heartbeat_v1"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobSendHeartbeatV1Params =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_send_heartbeat_v1(params)
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
    assert!(parsed.path().ends_with("/v1/heartbeats"));
}
#[tokio::test]
async fn clob_clob_get_rate_limit_tier() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_get_rate_limit_tier"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobGetRateLimitTierParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_get_rate_limit_tier(params)
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
    assert!(parsed.path().ends_with("/v1/user/rate-limit-tier"));
}
#[tokio::test]
async fn clob_clob_list_session_signers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clob.clob_list_session_signers"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clob::ClobListSessionSignersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clob_list_session_signers(params)
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
    assert!(parsed.path().ends_with("/v1/user/session-signers"));
}
#[tokio::test]
async fn gamma_gamma_list_comments() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_list_comments"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaListCommentsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_list_comments(params)
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
    assert!(parsed.path().ends_with("/comments"));
}
#[tokio::test]
async fn gamma_gamma_get_comments_by_user_address() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_comments_by_user_address"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetCommentsByUserAddressParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_comments_by_user_address(params)
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
    assert!(parsed.path().ends_with("/comments/user_address/fixture"));
}
#[tokio::test]
async fn gamma_gamma_get_comments_by_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_comments_by_id"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetCommentsByIdParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_comments_by_id(params)
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
    assert!(parsed.path().ends_with("/comments/1"));
}
#[tokio::test]
async fn gamma_gamma_list_events() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_list_events"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaListEventsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.gamma_list_events(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/events"));
}
#[tokio::test]
async fn gamma_gamma_list_event_creators() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_list_event_creators"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaListEventCreatorsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_list_event_creators(params)
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
    assert!(parsed.path().ends_with("/events/creators"));
}
#[tokio::test]
async fn gamma_gamma_get_event_creator() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_event_creator"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetEventCreatorParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_event_creator(params)
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
    assert!(parsed.path().ends_with("/events/creators/1"));
}
#[tokio::test]
async fn gamma_gamma_list_events_keyset() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_list_events_keyset"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaListEventsKeysetParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_list_events_keyset(params)
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
    assert!(parsed.path().ends_with("/events/keyset"));
}
#[tokio::test]
async fn gamma_gamma_list_events_pagination() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_list_events_pagination"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaListEventsPaginationParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_list_events_pagination(params)
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
    assert!(parsed.path().ends_with("/events/pagination"));
}
#[tokio::test]
async fn gamma_gamma_list_sport_events_results() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_list_sport_events_results"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaListSportEventsResultsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_list_sport_events_results(params)
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
    assert!(parsed.path().ends_with("/events/results"));
}
#[tokio::test]
async fn gamma_gamma_get_event_by_slug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_event_by_slug"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetEventBySlugParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_event_by_slug(params)
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
    assert!(parsed.path().ends_with("/events/slug/fixture"));
}
#[tokio::test]
async fn gamma_gamma_get_event() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_event"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetEventParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.gamma_get_event(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/events/1"));
}
#[tokio::test]
async fn gamma_gamma_get_event_comments_count() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_event_comments_count"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetEventCommentsCountParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_event_comments_count(params)
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
    assert!(parsed.path().ends_with("/events/1/comments/count"));
}
#[tokio::test]
async fn gamma_gamma_get_event_tags() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_event_tags"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetEventTagsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_event_tags(params)
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
    assert!(parsed.path().ends_with("/events/1/tags"));
}
#[tokio::test]
async fn gamma_gamma_get_event_tweet_count() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_event_tweet_count"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetEventTweetCountParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_event_tweet_count(params)
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
    assert!(parsed.path().ends_with("/events/1/tweet-count"));
}
#[tokio::test]
async fn gamma_gamma_list_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_list_markets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaListMarketsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.gamma_list_markets(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/markets"));
}
#[tokio::test]
async fn gamma_gamma_get_abridged_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_abridged_markets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetAbridgedMarketsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_abridged_markets(params)
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
    assert!(parsed.path().ends_with("/markets/abridged"));
}
#[tokio::test]
async fn gamma_gamma_get_markets_information() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_markets_information"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetMarketsInformationParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_markets_information(params)
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
    assert!(parsed.path().ends_with("/markets/information"));
}
#[tokio::test]
async fn gamma_gamma_list_markets_keyset() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_list_markets_keyset"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaListMarketsKeysetParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_list_markets_keyset(params)
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
    assert!(parsed.path().ends_with("/markets/keyset"));
}
#[tokio::test]
async fn gamma_gamma_get_market_by_slug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_market_by_slug"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetMarketBySlugParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_market_by_slug(params)
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
    assert!(parsed.path().ends_with("/markets/slug/fixture"));
}
#[tokio::test]
async fn gamma_gamma_get_market() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_market"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetMarketParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.gamma_get_market(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/markets/1"));
}
#[tokio::test]
async fn gamma_gamma_get_market_description() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_market_description"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetMarketDescriptionParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_market_description(params)
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
    assert!(parsed.path().ends_with("/markets/1/description"));
}
#[tokio::test]
async fn gamma_gamma_get_market_tags() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_market_tags"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetMarketTagsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_market_tags(params)
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
    assert!(parsed.path().ends_with("/markets/1/tags"));
}
#[tokio::test]
async fn gamma_gamma_get_public_profile_by_user_address() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_public_profile_by_user_address"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetPublicProfileByUserAddressParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_public_profile_by_user_address(params)
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
    assert!(parsed.path().ends_with("/profiles/user_address/fixture"));
}
#[tokio::test]
async fn gamma_gamma_get_public_profile() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_public_profile"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetPublicProfileParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_public_profile(params)
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
    assert!(parsed.path().ends_with("/public-profile"));
}
#[tokio::test]
async fn gamma_gamma_public_search() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_public_search"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaPublicSearchParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_public_search(params)
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
    assert!(parsed.path().ends_with("/public-search"));
}
#[tokio::test]
async fn gamma_gamma_list_series() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_list_series"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaListSeriesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.gamma_list_series(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/series"));
}
#[tokio::test]
async fn gamma_gamma_get_series_summary_by_slug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_series_summary_by_slug"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetSeriesSummaryBySlugParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_series_summary_by_slug(params)
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
    assert!(parsed.path().ends_with("/series-summary/slug/fixture"));
}
#[tokio::test]
async fn gamma_gamma_get_series_summary_by_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_series_summary_by_id"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetSeriesSummaryByIdParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_series_summary_by_id(params)
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
    assert!(parsed.path().ends_with("/series-summary/1"));
}
#[tokio::test]
async fn gamma_gamma_get_series() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_series"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetSeriesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.gamma_get_series(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/series/1"));
}
#[tokio::test]
async fn gamma_gamma_get_series_comments_count() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_series_comments_count"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetSeriesCommentsCountParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_series_comments_count(params)
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
    assert!(parsed.path().ends_with("/series/1/comments/count"));
}
#[tokio::test]
async fn gamma_gamma_get_sports_metadata() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_sports_metadata"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetSportsMetadataParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_sports_metadata(params)
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
    assert!(parsed.path().ends_with("/sports"));
}
#[tokio::test]
async fn gamma_gamma_get_sports_market_types() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_sports_market_types"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetSportsMarketTypesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_sports_market_types(params)
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
    assert!(parsed.path().ends_with("/sports/market-types"));
}
#[tokio::test]
async fn gamma_gamma_get_gamma_status() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_gamma_status"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetGammaStatusParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_gamma_status(params)
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
    assert!(parsed.path().ends_with("/status"));
}
#[tokio::test]
async fn gamma_gamma_list_tags() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_list_tags"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaListTagsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.gamma_list_tags(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/tags"));
}
#[tokio::test]
async fn gamma_gamma_get_tag_by_slug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_tag_by_slug"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetTagBySlugParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_tag_by_slug(params)
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
    assert!(parsed.path().ends_with("/tags/slug/fixture"));
}
#[tokio::test]
async fn gamma_gamma_get_related_tags_by_slug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_related_tags_by_slug"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetRelatedTagsBySlugParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_related_tags_by_slug(params)
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
    assert!(parsed.path().ends_with("/tags/slug/fixture/related-tags"));
}
#[tokio::test]
async fn gamma_gamma_get_tags_related_to_a_tag_by_slug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_tags_related_to_a_tag_by_slug"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetTagsRelatedToATagBySlugParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_tags_related_to_a_tag_by_slug(params)
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
            .ends_with("/tags/slug/fixture/related-tags/tags")
    );
}
#[tokio::test]
async fn gamma_gamma_get_tag() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_tag"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetTagParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.gamma_get_tag(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/tags/1"));
}
#[tokio::test]
async fn gamma_gamma_get_related_tags_by_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_related_tags_by_id"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetRelatedTagsByIdParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_related_tags_by_id(params)
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
    assert!(parsed.path().ends_with("/tags/1/related-tags"));
}
#[tokio::test]
async fn gamma_gamma_get_tags_related_to_a_tag_by_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_tags_related_to_a_tag_by_id"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetTagsRelatedToATagByIdParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .gamma_get_tags_related_to_a_tag_by_id(params)
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
    assert!(parsed.path().ends_with("/tags/1/related-tags/tags"));
}
#[tokio::test]
async fn gamma_gamma_list_teams() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_list_teams"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaListTeamsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.gamma_list_teams(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/teams"));
}
#[tokio::test]
async fn gamma_gamma_get_team() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["gamma.gamma_get_team"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::gamma::GammaGetTeamParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.gamma_get_team(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/teams/1"));
}
#[tokio::test]
async fn relayer_relayer_get_deployed() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["relayer.relayer_get_deployed"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::relayer::RelayerGetDeployedParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .relayer_get_deployed(params)
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
    assert!(parsed.path().ends_with("/deployed"));
}
#[tokio::test]
async fn relayer_relayer_get_nonce() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["relayer.relayer_get_nonce"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::relayer::RelayerGetNonceParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.relayer_get_nonce(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/nonce"));
}
#[tokio::test]
async fn relayer_relayer_get_relay_payload() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["relayer.relayer_get_relay_payload"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::relayer::RelayerGetRelayPayloadParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .relayer_get_relay_payload(params)
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
    assert!(parsed.path().ends_with("/relay-payload"));
}
#[tokio::test]
async fn relayer_relayer_get_relayer_api_keys() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["relayer.relayer_get_relayer_api_keys"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::relayer::RelayerGetRelayerApiKeysParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .relayer_get_relayer_api_keys(params)
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
    assert!(parsed.path().ends_with("/relayer/api/keys"));
}
#[tokio::test]
async fn relayer_relayer_post_submit() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["relayer.relayer_post_submit"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::relayer::RelayerPostSubmitParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .relayer_post_submit(params)
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
    assert!(parsed.path().ends_with("/submit"));
}
#[tokio::test]
async fn relayer_relayer_get_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["relayer.relayer_get_transaction"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::relayer::RelayerGetTransactionParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .relayer_get_transaction(params)
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
    assert!(parsed.path().ends_with("/transaction"));
}
#[tokio::test]
async fn relayer_relayer_get_transactions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["relayer.relayer_get_transactions"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::relayer::RelayerGetTransactionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .relayer_get_transactions(params)
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
    assert!(parsed.path().ends_with("/transactions"));
}
#[tokio::test]
async fn relayer_relayer_get_execute_params() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["relayer.relayer_get_execute_params"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::relayer::RelayerGetExecuteParamsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .relayer_get_execute_params(params)
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
    assert!(parsed.path().ends_with("/v1/account/transactions/params"));
}
#[tokio::test]
async fn relayer_relayer_authorizations() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["relayer.relayer_authorizations"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::relayer::RelayerAuthorizationsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .relayer_authorizations(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/v1/session-signers/authorizations")
    );
}
#[tokio::test]
async fn relayer_relayer_revocations() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["relayer.relayer_revocations"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::relayer::RelayerRevocationsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .relayer_revocations(params)
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
    assert!(parsed.path().ends_with("/v1/session-signers/revocations"));
}
#[tokio::test]
async fn combos_rfq_combos_rfq_submit_maker_confirmation() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["combos_rfq.combos_rfq_submit_maker_confirmation"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::combos_rfq::CombosRfqSubmitMakerConfirmationParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .combos_rfq_submit_maker_confirmation(params)
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
    assert!(parsed.path().ends_with("/v1/maker/confirmations"));
}
#[tokio::test]
async fn combos_rfq_combos_rfq_submit_maker_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["combos_rfq.combos_rfq_submit_maker_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::combos_rfq::CombosRfqSubmitMakerQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .combos_rfq_submit_maker_quote(params)
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
    assert!(parsed.path().ends_with("/v1/maker/quotes"));
}
#[tokio::test]
async fn combos_rfq_combos_rfq_cancel_maker_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["combos_rfq.combos_rfq_cancel_maker_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::combos_rfq::CombosRfqCancelMakerQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .combos_rfq_cancel_maker_quote(params)
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
    assert!(parsed.path().ends_with("/v1/maker/quotes/cancel"));
}
#[tokio::test]
async fn combos_rfq_combos_rfq_get_combo_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["combos_rfq.combos_rfq_get_combo_markets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::combos_rfq::CombosRfqGetComboMarketsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .combos_rfq_get_combo_markets(params)
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
    assert!(parsed.path().ends_with("/v1/rfq/combo-markets"));
}
#[tokio::test]
async fn bridge_bridge_post_deposit() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["bridge.bridge_post_deposit"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::bridge::BridgePostDepositParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .bridge_post_deposit(params)
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
    assert!(parsed.path().ends_with("/deposit"));
}
#[tokio::test]
async fn bridge_bridge_post_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["bridge.bridge_post_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::bridge::BridgePostQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.bridge_post_quote(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/quote"));
}
#[tokio::test]
async fn bridge_bridge_get_status_address() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["bridge.bridge_get_status_address"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::bridge::BridgeGetStatusAddressParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .bridge_get_status_address(params)
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
    assert!(parsed.path().ends_with("/status/fixture"));
}
#[tokio::test]
async fn bridge_bridge_get_supported_assets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["bridge.bridge_get_supported_assets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::bridge::BridgeGetSupportedAssetsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .bridge_get_supported_assets(params)
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
    assert!(parsed.path().ends_with("/supported-assets"));
}
#[tokio::test]
async fn bridge_bridge_post_withdraw() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["bridge.bridge_post_withdraw"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::bridge::BridgePostWithdrawParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .bridge_post_withdraw(params)
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
    assert!(parsed.path().ends_with("/withdraw"));
}
#[tokio::test]
async fn data_data_get_activity() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_activity"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetActivityParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.data_get_activity(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v2/activity"));
}
#[tokio::test]
async fn data_data_get_combos() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_combos"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetCombosParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.data_get_combos(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v2/activity/combos"));
}
#[tokio::test]
async fn data_data_get_approvals() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_approvals"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetApprovalsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.data_get_approvals(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v2/approvals"));
}
#[tokio::test]
async fn data_data_get_biggest_winners() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_biggest_winners"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetBiggestWinnersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .data_get_biggest_winners(params)
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
    assert!(parsed.path().ends_with("/v2/biggest-winners"));
}
#[tokio::test]
async fn data_data_get_builders_leaderboard() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_builders_leaderboard"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetBuildersLeaderboardParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .data_get_builders_leaderboard(params)
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
    assert!(parsed.path().ends_with("/v2/builders/leaderboard"));
}
#[tokio::test]
async fn data_data_get_builders_volume() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_builders_volume"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetBuildersVolumeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .data_get_builders_volume(params)
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
    assert!(parsed.path().ends_with("/v2/builders/volume"));
}
#[tokio::test]
async fn data_data_get_holders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_holders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetHoldersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.data_get_holders(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v2/holders"));
}
#[tokio::test]
async fn data_data_get_leaderboard() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_leaderboard"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetLeaderboardParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .data_get_leaderboard(params)
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
    assert!(parsed.path().ends_with("/v2/leaderboard"));
}
#[tokio::test]
async fn data_data_get_live_volume() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_live_volume"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetLiveVolumeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .data_get_live_volume(params)
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
    assert!(parsed.path().ends_with("/v2/live-volume"));
}
#[tokio::test]
async fn data_data_get_oi() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_oi"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetOiParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.data_get_oi(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v2/oi"));
}
#[tokio::test]
async fn data_data_get_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_positions"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetPositionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.data_get_positions(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v2/positions"));
}
#[tokio::test]
async fn data_data_get_combo_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_combo_positions"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetComboPositionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .data_get_combo_positions(params)
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
    assert!(parsed.path().ends_with("/v2/positions/combos"));
}
#[tokio::test]
async fn data_data_get_prices_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_prices_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetPricesHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .data_get_prices_history(params)
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
    assert!(parsed.path().ends_with("/v2/prices-history"));
}
#[tokio::test]
async fn data_data_get_resolutions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_resolutions"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetResolutionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .data_get_resolutions(params)
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
    assert!(parsed.path().ends_with("/v2/resolutions"));
}
#[tokio::test]
async fn data_data_get_status() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_status"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetStatusParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.data_get_status(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v2/status"));
}
#[tokio::test]
async fn data_data_get_tokens() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_tokens"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetTokensParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.data_get_tokens(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v2/tokens"));
}
#[tokio::test]
async fn data_data_get_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_trades"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetTradesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.data_get_trades(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v2/trades"));
}
#[tokio::test]
async fn data_data_get_user_pnl_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_user_pnl_v2"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetUserPnlV2Params =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .data_get_user_pnl_v2(params)
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
    assert!(parsed.path().ends_with("/v2/user-pnl"));
}
#[tokio::test]
async fn data_data_get_user_stats() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_user_stats"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetUserStatsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .data_get_user_stats(params)
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
    assert!(parsed.path().ends_with("/v2/user-stats"));
}
#[tokio::test]
async fn data_data_get_user_volume() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_user_volume"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetUserVolumeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .data_get_user_volume(params)
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
    assert!(parsed.path().ends_with("/v2/user-volume"));
}
#[tokio::test]
async fn data_data_get_value() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["data.data_get_value"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::data::DataGetValueParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.data_get_value(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v2/value"));
}
#[tokio::test]
async fn perps_get_auto_cancel() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_auto_cancel"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetAutoCancelParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_auto_cancel(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/auto-cancel"));
}
#[tokio::test]
async fn perps_get_account_backstops() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_account_backstops"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetAccountBackstopsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_account_backstops(params)
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
    assert!(parsed.path().ends_with("/v1/account/backstops"));
}
#[tokio::test]
async fn perps_get_balances() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_balances"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetBalancesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_balances(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/balances"));
}
#[tokio::test]
async fn perps_update_builder() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.update_builder"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::UpdateBuilderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.update_builder(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Patch);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/account/builder"));
}
#[tokio::test]
async fn perps_get_builder_approvals() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_builder_approvals"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetBuilderApprovalsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_builder_approvals(params)
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
    assert!(parsed.path().ends_with("/v1/account/builder-approvals"));
}
#[tokio::test]
async fn perps_approve_builder() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.approve_builder"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::ApproveBuilderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.approve_builder(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/builder-approvals"));
}
#[tokio::test]
async fn perps_get_builder_earnings() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_builder_earnings"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetBuilderEarningsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_builder_earnings(params)
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
    assert!(parsed.path().ends_with("/v1/account/builder-earnings"));
}
#[tokio::test]
async fn perps_get_builder_earnings_summary() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_builder_earnings_summary"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetBuilderEarningsSummaryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_builder_earnings_summary(params)
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
            .ends_with("/v1/account/builder-earnings-summary")
    );
}
#[tokio::test]
async fn perps_get_chases() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_chases"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetChasesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_chases(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/chases"));
}
#[tokio::test]
async fn perps_get_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_config"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetConfigParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_config(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/config"));
}
#[tokio::test]
async fn perps_get_credentials() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_credentials"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetCredentialsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_credentials(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/credentials"));
}
#[tokio::test]
async fn perps_get_deposits() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_deposits"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetDepositsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_deposits(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/deposits"));
}
#[tokio::test]
async fn perps_get_equity_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_equity_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetEquityHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_equity_history(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/equity"));
}
#[tokio::test]
async fn perps_get_fills() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_fills"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetFillsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_fills(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/fills"));
}
#[tokio::test]
async fn perps_get_account_funding() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_account_funding"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetAccountFundingParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_account_funding(params)
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
    assert!(parsed.path().ends_with("/v1/account/funding"));
}
#[tokio::test]
async fn perps_internal_transfer() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.internal_transfer"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::InternalTransferParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.internal_transfer(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/internal-transfer"));
}
#[tokio::test]
async fn perps_get_internal_transfers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_internal_transfers"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetInternalTransfersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_internal_transfers(params)
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
    assert!(parsed.path().ends_with("/v1/account/internal-transfers"));
}
#[tokio::test]
async fn perps_create_account_invite() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.create_account_invite"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::CreateAccountInviteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .create_account_invite(params)
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
    assert!(parsed.path().ends_with("/v1/account/invite"));
}
#[tokio::test]
async fn perps_get_account_limits() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_account_limits"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetAccountLimitsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_account_limits(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/limits"));
}
#[tokio::test]
async fn perps_get_account_notifications() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_account_notifications"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetAccountNotificationsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_account_notifications(params)
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
    assert!(parsed.path().ends_with("/v1/account/notifications"));
}
#[tokio::test]
async fn perps_mark_account_notifications_read() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.mark_account_notifications_read"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::MarkAccountNotificationsReadParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .mark_account_notifications_read(params)
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
    assert!(parsed.path().ends_with("/v1/account/notifications/read"));
}
#[tokio::test]
async fn perps_get_open_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_open_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetOpenOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_open_orders(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/open-orders"));
}
#[tokio::test]
async fn perps_get_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_orders(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/orders"));
}
#[tokio::test]
async fn perps_get_pnl_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_pnl_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetPnlHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_pnl_history(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/pnl"));
}
#[tokio::test]
async fn perps_get_portfolio() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_portfolio"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetPortfolioParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_portfolio(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/portfolio"));
}
#[tokio::test]
async fn perps_delete_proxy() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.delete_proxy"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::DeleteProxyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.delete_proxy(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/account/proxy"));
}
#[tokio::test]
async fn perps_create_proxy() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.create_proxy"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::CreateProxyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.create_proxy(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/proxy"));
}
#[tokio::test]
async fn perps_get_account_referral() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_account_referral"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetAccountReferralParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_account_referral(params)
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
    assert!(parsed.path().ends_with("/v1/account/referral"));
}
#[tokio::test]
async fn perps_apply_referral() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.apply_referral"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::ApplyReferralParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.apply_referral(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/referral"));
}
#[tokio::test]
async fn perps_get_account_rewards() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_account_rewards"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetAccountRewardsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_account_rewards(params)
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
    assert!(parsed.path().ends_with("/v1/account/rewards"));
}
#[tokio::test]
async fn perps_get_account_stats() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_account_stats"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetAccountStatsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_account_stats(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/stats"));
}
#[tokio::test]
async fn perps_get_twaps() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_twaps"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetTwapsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_twaps(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/twaps"));
}
#[tokio::test]
async fn perps_withdraw() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.withdraw"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::WithdrawParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.withdraw(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/withdraw"));
}
#[tokio::test]
async fn perps_get_withdrawals() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_withdrawals"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetWithdrawalsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_withdrawals(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/account/withdrawals"));
}
#[tokio::test]
async fn perps_post_blp_enroll() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.post_blp_enroll"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::PostBlpEnrollParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.post_blp_enroll(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/blp/enroll"));
}
#[tokio::test]
async fn perps_get_blp_enrollment() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_blp_enrollment"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetBlpEnrollmentParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_blp_enrollment(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/blp/enrollment"));
}
#[tokio::test]
async fn perps_get_blp_liquidations() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_blp_liquidations"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetBlpLiquidationsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_blp_liquidations(params)
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
    assert!(parsed.path().ends_with("/v1/blp/liquidations"));
}
#[tokio::test]
async fn perps_get_assets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_assets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetAssetsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_assets(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/assets"));
}
#[tokio::test]
async fn perps_get_bbo() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_bbo"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetBboParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_bbo(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/bbo"));
}
#[tokio::test]
async fn perps_get_book() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_book"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetBookParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_book(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/book"));
}
#[tokio::test]
async fn perps_get_builder() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_builder"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetBuilderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_builder(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/builder"));
}
#[tokio::test]
async fn perps_get_competition() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_competition"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetCompetitionParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_competition(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/competition"));
}
#[tokio::test]
async fn perps_get_competition_account() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_competition_account"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetCompetitionAccountParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_competition_account(params)
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
    assert!(parsed.path().ends_with("/v1/info/competition/account"));
}
#[tokio::test]
async fn perps_get_competition_leaderboard() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_competition_leaderboard"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetCompetitionLeaderboardParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_competition_leaderboard(params)
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
    assert!(parsed.path().ends_with("/v1/info/competition/leaderboard"));
}
#[tokio::test]
async fn perps_get_competition_widget() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_competition_widget"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetCompetitionWidgetParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_competition_widget(params)
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
    assert!(parsed.path().ends_with("/v1/info/competition/widget"));
}
#[tokio::test]
async fn perps_get_competitions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_competitions"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetCompetitionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_competitions(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/competitions"));
}
#[tokio::test]
async fn perps_get_exchange() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_exchange"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetExchangeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_exchange(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/exchange"));
}
#[tokio::test]
async fn perps_get_exchange_statistics() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_exchange_statistics"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetExchangeStatisticsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_exchange_statistics(params)
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
    assert!(parsed.path().ends_with("/v1/info/exchange-stats"));
}
#[tokio::test]
async fn perps_get_info_fees() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_info_fees"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetInfoFeesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_info_fees(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/fees"));
}
#[tokio::test]
async fn perps_get_funding_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_funding_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetFundingHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_funding_history(params)
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
    assert!(parsed.path().ends_with("/v1/info/funding"));
}
#[tokio::test]
async fn perps_get_index() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_index"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetIndexParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_index(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/index"));
}
#[tokio::test]
async fn perps_get_instruments() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_instruments"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetInstrumentsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_instruments(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/instruments"));
}
#[tokio::test]
async fn perps_check_invite_code() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.check_invite_code"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::CheckInviteCodeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.check_invite_code(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/invite"));
}
#[tokio::test]
async fn perps_get_klines() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_klines"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetKlinesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_klines(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/klines"));
}
#[tokio::test]
async fn perps_get_leaderboard() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_leaderboard"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetLeaderboardParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_leaderboard(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/leaderboard"));
}
#[tokio::test]
async fn perps_get_limit_tiers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_limit_tiers"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetLimitTiersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_limit_tiers(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/limit-tiers"));
}
#[tokio::test]
async fn perps_get_mark_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_mark_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetMarkHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_mark_history(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/mark-history"));
}
#[tokio::test]
async fn perps_get_ping() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_ping"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetPingParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_ping(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/ping"));
}
#[tokio::test]
async fn perps_get_public_portfolio() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_public_portfolio"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetPublicPortfolioParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_public_portfolio(params)
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
    assert!(parsed.path().ends_with("/v1/info/portfolio"));
}
#[tokio::test]
async fn perps_get_public_position_fills() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_public_position_fills"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetPublicPositionFillsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_public_position_fills(params)
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
    assert!(parsed.path().ends_with("/v1/info/position-fills"));
}
#[tokio::test]
async fn perps_get_public_position_snapshots() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_public_position_snapshots"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetPublicPositionSnapshotsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_public_position_snapshots(params)
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
    assert!(parsed.path().ends_with("/v1/info/position-snapshots"));
}
#[tokio::test]
async fn perps_get_registered() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_registered"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetRegisteredParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_registered(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/registered"));
}
#[tokio::test]
async fn perps_get_statistics() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_statistics"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetStatisticsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_statistics(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/statistics"));
}
#[tokio::test]
async fn perps_get_tickers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_tickers"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetTickersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_tickers(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/tickers"));
}
#[tokio::test]
async fn perps_get_time() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_time"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetTimeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_time(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/time"));
}
#[tokio::test]
async fn perps_get_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_trades"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::GetTradesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_trades(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/info/trades"));
}
#[tokio::test]
async fn perps_set_auto_cancel() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.set_auto_cancel"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::SetAutoCancelParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.set_auto_cancel(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Patch);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/trade/auto-cancel"));
}
#[tokio::test]
async fn perps_cancel_chase() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.cancel_chase"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::CancelChaseParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.cancel_chase(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/trade/chases"));
}
#[tokio::test]
async fn perps_create_chase() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.create_chase"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::CreateChaseParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.create_chase(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/trade/chases"));
}
#[tokio::test]
async fn perps_update_leverage() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.update_leverage"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::UpdateLeverageParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.update_leverage(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Patch);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/trade/leverage"));
}
#[tokio::test]
async fn perps_update_leverages() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.update_leverages"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::UpdateLeveragesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.update_leverages(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Patch);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/trade/leverage/batch"));
}
#[tokio::test]
async fn perps_update_margin() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.update_margin"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::UpdateMarginParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.update_margin(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Patch);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/trade/margin"));
}
#[tokio::test]
async fn perps_cancel_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.cancel_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::CancelOrdersParams =
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
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/trade/orders"));
}
#[tokio::test]
async fn perps_modify_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.modify_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::ModifyOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.modify_orders(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Patch);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/trade/orders"));
}
#[tokio::test]
async fn perps_create_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.create_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::CreateOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.create_orders(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/trade/orders"));
}
#[tokio::test]
async fn perps_cancel_orders_by_coid() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.cancel_orders_by_coid"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::CancelOrdersByCoidParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .cancel_orders_by_coid(params)
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
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/trade/orders-coid"));
}
#[tokio::test]
async fn perps_modify_orders_by_coid() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.modify_orders_by_coid"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::ModifyOrdersByCoidParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .modify_orders_by_coid(params)
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
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Patch);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/trade/orders-coid"));
}
#[tokio::test]
async fn perps_cancel_all_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.cancel_all_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::CancelAllOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.cancel_all_orders(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/trade/orders/all"));
}
#[tokio::test]
async fn perps_cancel_twap() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.cancel_twap"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::CancelTwapParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.cancel_twap(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Delete);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/trade/twaps"));
}
#[tokio::test]
async fn perps_control_twap() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.control_twap"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::ControlTwapParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.control_twap(params).await.unwrap_or_else(|e| {
        if let aux_rs_core::Error::Exchange(v::Error::Decode(d)) = e {
            panic!("fixture decode: {d}");
        }
        panic!("{e}");
    });
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value, fixture["response"]);
    let sent = mock.sent.lock().unwrap();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].method, aux_rs_core::HttpMethod::Patch);
    let parsed = url::Url::parse(&sent[0].url).unwrap();
    assert!(!parsed.path().contains("{"));
    assert!(parsed.path().ends_with("/v1/trade/twaps"));
}
#[tokio::test]
async fn perps_create_twap() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.create_twap"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client = v::PolymarketPerpsClient::with_transport(v::Polymarket::mainnet(), mock.clone())
        .with_auth(Auth);
    let params: v::perps::CreateTwapParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.create_twap(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/v1/trade/twaps"));
}
#[tokio::test]
async fn collateral_return_collateral_return_plan() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["collateral_return.collateral_return_plan"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::collateral_return::CollateralReturnPlanParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .collateral_return_plan(params)
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
    assert!(parsed.path().ends_with("/v1/collateral-return/plan"));
}
#[tokio::test]
async fn collateral_return_collateral_return_submit() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["collateral_return.collateral_return_submit"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::collateral_return::CollateralReturnSubmitParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .collateral_return_submit(params)
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
    assert!(parsed.path().ends_with("/v1/collateral-return/submit"));
}
#[tokio::test]
async fn builder_gateway_builder_gateway_create_rfq() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["builder_gateway.builder_gateway_create_rfq"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::builder_gateway::BuilderGatewayCreateRfqParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .builder_gateway_create_rfq(params)
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
    assert!(parsed.path().ends_with("/v1/builder/rfq/requests"));
}
#[tokio::test]
async fn builder_gateway_builder_gateway_get_rfq() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["builder_gateway.builder_gateway_get_rfq"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::builder_gateway::BuilderGatewayGetRfqParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .builder_gateway_get_rfq(params)
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
    assert!(parsed.path().ends_with("/v1/builder/rfq/requests/fixture"));
}
#[tokio::test]
async fn builder_gateway_builder_gateway_accept_rfq() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["builder_gateway.builder_gateway_accept_rfq"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::PolymarketClient::with_transport(v::Polymarket::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::builder_gateway::BuilderGatewayAcceptRfqParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .builder_gateway_accept_rfq(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/v1/builder/rfq/requests/fixture/accept")
    );
}
