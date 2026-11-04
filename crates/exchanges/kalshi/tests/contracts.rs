// Generated named-method contracts; no exchange requests.
use aux_rs_core::{HttpRequest, HttpResponse, HttpTransport, TransportError};
use aux_rs_kalshi as v;
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
        _: &v::Kalshi,
        _: &v::Operation,
        r: &mut HttpRequest,
    ) -> Result<(), v::Error> {
        r.headers.push(("x-fixture-auth".into(), "true".into()));
        Ok(())
    }
}
#[tokio::test]
async fn predictions_upgrade_account_api_usage_level() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.upgrade_account_api_usage_level"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::UpgradeAccountApiUsageLevelParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .upgrade_account_api_usage_level(params)
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
    assert!(parsed.path().ends_with("/account/api_usage_level/upgrade"));
}
#[tokio::test]
async fn predictions_get_account_api_usage_level_volume_progress() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_account_api_usage_level_volume_progress"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetAccountApiUsageLevelVolumeProgressParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_account_api_usage_level_volume_progress(params)
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
            .ends_with("/account/api_usage_level/volume_progress")
    );
}
#[tokio::test]
async fn predictions_get_account_endpoint_costs() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_account_endpoint_costs"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetAccountEndpointCostsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_account_endpoint_costs(params)
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
    assert!(parsed.path().ends_with("/account/endpoint_costs"));
}
#[tokio::test]
async fn predictions_get_account_api_limits() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_account_api_limits"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetAccountApiLimitsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_account_api_limits(params)
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
    assert!(parsed.path().ends_with("/account/limits"));
}
#[tokio::test]
async fn predictions_get_api_keys() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_api_keys"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetApiKeysParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_api_keys(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/api_keys"));
}
#[tokio::test]
async fn predictions_create_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.create_api_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CreateApiKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.create_api_key(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/api_keys"));
}
#[tokio::test]
async fn predictions_generate_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.generate_api_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GenerateApiKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.generate_api_key(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/api_keys/generate"));
}
#[tokio::test]
async fn predictions_delete_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.delete_api_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::DeleteApiKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.delete_api_key(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/api_keys/fixture"));
}
#[tokio::test]
async fn predictions_get_block_trade_proposals() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_block_trade_proposals"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetBlockTradeProposalsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_block_trade_proposals(params)
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
            .ends_with("/communications/block-trade-proposals")
    );
}
#[tokio::test]
async fn predictions_propose_block_trade() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.propose_block_trade"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::ProposeBlockTradeParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .propose_block_trade(params)
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
            .ends_with("/communications/block-trade-proposals")
    );
}
#[tokio::test]
async fn predictions_accept_block_trade_proposal() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.accept_block_trade_proposal"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::AcceptBlockTradeProposalParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .accept_block_trade_proposal(params)
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
            .ends_with("/communications/block-trade-proposals/fixture/accept")
    );
}
#[tokio::test]
async fn predictions_get_communications_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_communications_id"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetCommunicationsIdParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_communications_id(params)
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
    assert!(parsed.path().ends_with("/communications/id"));
}
#[tokio::test]
async fn predictions_get_quotes() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_quotes"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetQuotesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_quotes(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/communications/quotes"));
}
#[tokio::test]
async fn predictions_create_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.create_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CreateQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.create_quote(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/communications/quotes"));
}
#[tokio::test]
async fn predictions_delete_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.delete_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::DeleteQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.delete_quote(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/communications/quotes/fixture"));
}
#[tokio::test]
async fn predictions_get_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_quote(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/communications/quotes/fixture"));
}
#[tokio::test]
async fn predictions_accept_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.accept_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::AcceptQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.accept_quote(params).await.unwrap_or_else(|e| {
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
    assert!(
        parsed
            .path()
            .ends_with("/communications/quotes/fixture/accept")
    );
}
#[tokio::test]
async fn predictions_confirm_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.confirm_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::ConfirmQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.confirm_quote(params).await.unwrap_or_else(|e| {
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
    assert!(
        parsed
            .path()
            .ends_with("/communications/quotes/fixture/confirm")
    );
}
#[tokio::test]
async fn predictions_get_rfq_batches() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_rfq_batches"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetRfqBatchesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_rfq_batches(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/communications/rfq_batches"));
}
#[tokio::test]
async fn predictions_create_rfq_batch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.create_rfq_batch"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CreateRfqBatchParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.create_rfq_batch(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/communications/rfq_batches"));
}
#[tokio::test]
async fn predictions_delete_rfq_batch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.delete_rfq_batch"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::DeleteRfqBatchParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.delete_rfq_batch(params).await.unwrap_or_else(|e| {
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
    assert!(
        parsed
            .path()
            .ends_with("/communications/rfq_batches/fixture")
    );
}
#[tokio::test]
async fn predictions_get_rfq_batch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_rfq_batch"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetRfqBatchParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_rfq_batch(params).await.unwrap_or_else(|e| {
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
            .ends_with("/communications/rfq_batches/fixture")
    );
}
#[tokio::test]
async fn predictions_accept_rfq_batch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.accept_rfq_batch"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::AcceptRfqBatchParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.accept_rfq_batch(params).await.unwrap_or_else(|e| {
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
    assert!(
        parsed
            .path()
            .ends_with("/communications/rfq_batches/fixture/accept")
    );
}
#[tokio::test]
async fn predictions_get_rfq_batch_best_quotes() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_rfq_batch_best_quotes"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetRfqBatchBestQuotesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_rfq_batch_best_quotes(params)
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
            .ends_with("/communications/rfq_batches/fixture/best_quotes")
    );
}
#[tokio::test]
async fn predictions_get_rfq_batch_quotes() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_rfq_batch_quotes"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetRfqBatchQuotesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_rfq_batch_quotes(params)
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
            .ends_with("/communications/rfq_batches/fixture/quotes")
    );
}
#[tokio::test]
async fn predictions_create_rfq_batch_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.create_rfq_batch_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CreateRfqBatchQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .create_rfq_batch_quote(params)
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
            .ends_with("/communications/rfq_batches/fixture/quotes")
    );
}
#[tokio::test]
async fn predictions_delete_rfq_batch_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.delete_rfq_batch_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::DeleteRfqBatchQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .delete_rfq_batch_quote(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/communications/rfq_batches/fixture/quotes/fixture")
    );
}
#[tokio::test]
async fn predictions_confirm_rfq_batch_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.confirm_rfq_batch_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::ConfirmRfqBatchQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .confirm_rfq_batch_quote(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/communications/rfq_batches/fixture/quotes/fixture/confirm")
    );
}
#[tokio::test]
async fn predictions_get_rf_qs() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_rf_qs"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetRfQsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_rf_qs(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/communications/rfqs"));
}
#[tokio::test]
async fn predictions_create_rfq() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.create_rfq"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CreateRfqParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.create_rfq(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/communications/rfqs"));
}
#[tokio::test]
async fn predictions_delete_rfq() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.delete_rfq"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::DeleteRfqParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.delete_rfq(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/communications/rfqs/fixture"));
}
#[tokio::test]
async fn predictions_get_rfq() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_rfq"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetRfqParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_rfq(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/communications/rfqs/fixture"));
}
#[tokio::test]
async fn predictions_delete_rfq_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.delete_rfq_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::DeleteRfqQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.delete_rfq_quote(params).await.unwrap_or_else(|e| {
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
    assert!(
        parsed
            .path()
            .ends_with("/communications/rfqs/fixture/quotes/fixture")
    );
}
#[tokio::test]
async fn predictions_get_rfq_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_rfq_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetRfqQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_rfq_quote(params).await.unwrap_or_else(|e| {
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
            .ends_with("/communications/rfqs/fixture/quotes/fixture")
    );
}
#[tokio::test]
async fn predictions_accept_rfq_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.accept_rfq_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::AcceptRfqQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.accept_rfq_quote(params).await.unwrap_or_else(|e| {
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
    assert!(
        parsed
            .path()
            .ends_with("/communications/rfqs/fixture/quotes/fixture/accept")
    );
}
#[tokio::test]
async fn predictions_confirm_rfq_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.confirm_rfq_quote"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::ConfirmRfqQuoteParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.confirm_rfq_quote(params).await.unwrap_or_else(|e| {
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
    assert!(
        parsed
            .path()
            .ends_with("/communications/rfqs/fixture/quotes/fixture/confirm")
    );
}
#[tokio::test]
async fn predictions_get_events() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_events"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetEventsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_events(params).await.unwrap_or_else(|e| {
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
async fn predictions_get_event_fee_changes() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_event_fee_changes"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetEventFeeChangesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_event_fee_changes(params)
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
    assert!(parsed.path().ends_with("/events/fee_changes"));
}
#[tokio::test]
async fn predictions_get_multivariate_events() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_multivariate_events"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetMultivariateEventsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_multivariate_events(params)
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
    assert!(parsed.path().ends_with("/events/multivariate"));
}
#[tokio::test]
async fn predictions_get_event() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_event"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetEventParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_event(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/events/fixture"));
}
#[tokio::test]
async fn predictions_get_event_metadata() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_event_metadata"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetEventMetadataParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_event_metadata(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/events/fixture/metadata"));
}
#[tokio::test]
async fn predictions_get_exchange_schedule() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_exchange_schedule"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetExchangeScheduleParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_exchange_schedule(params)
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
    assert!(parsed.path().ends_with("/exchange/schedule"));
}
#[tokio::test]
async fn predictions_get_exchange_status() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_exchange_status"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetExchangeStatusParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_exchange_status(params)
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
    assert!(parsed.path().ends_with("/exchange/status"));
}
#[tokio::test]
async fn predictions_get_user_data_timestamp() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_user_data_timestamp"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetUserDataTimestampParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_user_data_timestamp(params)
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
    assert!(parsed.path().ends_with("/exchange/user_data_timestamp"));
}
#[tokio::test]
async fn predictions_get_fcm_fills() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_fcm_fills"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetFcmFillsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_fcm_fills(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/fcm/fills"));
}
#[tokio::test]
async fn predictions_get_fcm_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_fcm_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetFcmOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_fcm_orders(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/fcm/orders"));
}
#[tokio::test]
async fn predictions_get_fcm_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_fcm_positions"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetFcmPositionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_fcm_positions(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/fcm/positions"));
}
#[tokio::test]
async fn predictions_create_fcm_position_transfer() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.create_fcm_position_transfer"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CreateFcmPositionTransferParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .create_fcm_position_transfer(params)
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
    assert!(parsed.path().ends_with("/fcm/positions/transfers"));
}
#[tokio::test]
async fn predictions_list_fcm_subtraders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.list_fcm_subtraders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::ListFcmSubtradersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .list_fcm_subtraders(params)
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
    assert!(parsed.path().ends_with("/fcm/subtraders"));
}
#[tokio::test]
async fn predictions_create_fcm_subtrader() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.create_fcm_subtrader"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CreateFcmSubtraderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .create_fcm_subtrader(params)
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
    assert!(parsed.path().ends_with("/fcm/subtraders"));
}
#[tokio::test]
async fn predictions_get_fcm_subtrader_blocked_categories() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_fcm_subtrader_blocked_categories"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetFcmSubtraderBlockedCategoriesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_fcm_subtrader_blocked_categories(params)
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
            .ends_with("/fcm/subtraders/blocked_categories")
    );
}
#[tokio::test]
async fn predictions_update_fcm_subtrader_blocked_categories() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.update_fcm_subtrader_blocked_categories"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::UpdateFcmSubtraderBlockedCategoriesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .update_fcm_subtrader_blocked_categories(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/fcm/subtraders/blocked_categories")
    );
}
#[tokio::test]
async fn predictions_delete_fcm_event_contract_daily_cap() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.delete_fcm_event_contract_daily_cap"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::DeleteFcmEventContractDailyCapParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .delete_fcm_event_contract_daily_cap(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/fcm/subtraders/event_contract_daily_cap")
    );
}
#[tokio::test]
async fn predictions_get_fcm_event_contract_daily_cap() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_fcm_event_contract_daily_cap"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetFcmEventContractDailyCapParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_fcm_event_contract_daily_cap(params)
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
            .ends_with("/fcm/subtraders/event_contract_daily_cap")
    );
}
#[tokio::test]
async fn predictions_update_fcm_event_contract_daily_cap() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.update_fcm_event_contract_daily_cap"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::UpdateFcmEventContractDailyCapParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .update_fcm_event_contract_daily_cap(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/fcm/subtraders/event_contract_daily_cap")
    );
}
#[tokio::test]
async fn predictions_get_historical_cutoff() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_historical_cutoff"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetHistoricalCutoffParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_historical_cutoff(params)
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
    assert!(parsed.path().ends_with("/historical/cutoff"));
}
#[tokio::test]
async fn predictions_get_fills_historical() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_fills_historical"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetFillsHistoricalParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_fills_historical(params)
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
    assert!(parsed.path().ends_with("/historical/fills"));
}
#[tokio::test]
async fn predictions_get_historical_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_historical_markets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetHistoricalMarketsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_historical_markets(params)
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
    assert!(parsed.path().ends_with("/historical/markets"));
}
#[tokio::test]
async fn predictions_get_historical_market() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_historical_market"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetHistoricalMarketParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_historical_market(params)
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
    assert!(parsed.path().ends_with("/historical/markets/fixture"));
}
#[tokio::test]
async fn predictions_get_market_candlesticks_historical() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_market_candlesticks_historical"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetMarketCandlesticksHistoricalParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_market_candlesticks_historical(params)
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
            .ends_with("/historical/markets/fixture/candlesticks")
    );
}
#[tokio::test]
async fn predictions_get_historical_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_historical_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetHistoricalOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_historical_orders(params)
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
    assert!(parsed.path().ends_with("/historical/orders"));
}
#[tokio::test]
async fn predictions_get_historical_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_historical_positions"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetHistoricalPositionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_historical_positions(params)
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
    assert!(parsed.path().ends_with("/historical/positions"));
}
#[tokio::test]
async fn predictions_get_trades_historical() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_trades_historical"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetTradesHistoricalParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_trades_historical(params)
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
    assert!(parsed.path().ends_with("/historical/trades"));
}
#[tokio::test]
async fn predictions_get_incentive_programs() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_incentive_programs"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetIncentiveProgramsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_incentive_programs(params)
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
    assert!(parsed.path().ends_with("/incentive_programs"));
}
#[tokio::test]
async fn predictions_get_live_datas() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_live_datas"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetLiveDatasParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_live_datas(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/live_data/batch"));
}
#[tokio::test]
async fn predictions_get_event_live_data() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_event_live_data"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetEventLiveDataParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_event_live_data(params)
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
    assert!(parsed.path().ends_with("/live_data/events/fixture"));
}
#[tokio::test]
async fn predictions_get_live_data_by_milestone() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_live_data_by_milestone"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetLiveDataByMilestoneParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_live_data_by_milestone(params)
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
    assert!(parsed.path().ends_with("/live_data/milestone/fixture"));
}
#[tokio::test]
async fn predictions_get_game_stats() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_game_stats"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetGameStatsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_game_stats(params).await.unwrap_or_else(|e| {
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
            .ends_with("/live_data/milestone/fixture/game_stats")
    );
}
#[tokio::test]
async fn predictions_get_weather_index() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_weather_index"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetWeatherIndexParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_weather_index(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/live_data/weather/fixture"));
}
#[tokio::test]
async fn predictions_get_weather_index_calibrations() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_weather_index_calibrations"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetWeatherIndexCalibrationsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_weather_index_calibrations(params)
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
            .ends_with("/live_data/weather/fixture/calibrations")
    );
}
#[tokio::test]
async fn predictions_get_live_data() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_live_data"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetLiveDataParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_live_data(params).await.unwrap_or_else(|e| {
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
            .ends_with("/live_data/fixture/milestone/fixture")
    );
}
#[tokio::test]
async fn predictions_get_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_markets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetMarketsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_markets(params).await.unwrap_or_else(|e| {
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
async fn predictions_batch_get_market_candlesticks() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.batch_get_market_candlesticks"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::BatchGetMarketCandlesticksParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .batch_get_market_candlesticks(params)
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
    assert!(parsed.path().ends_with("/markets/candlesticks"));
}
#[tokio::test]
async fn predictions_get_market_orderbooks() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_market_orderbooks"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetMarketOrderbooksParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_market_orderbooks(params)
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
    assert!(parsed.path().ends_with("/markets/orderbooks"));
}
#[tokio::test]
async fn predictions_get_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_trades"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetTradesParams =
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
    assert!(parsed.path().ends_with("/markets/trades"));
}
#[tokio::test]
async fn predictions_get_market() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_market"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetMarketParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_market(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/markets/fixture"));
}
#[tokio::test]
async fn predictions_get_market_orderbook() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_market_orderbook"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetMarketOrderbookParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_market_orderbook(params)
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
    assert!(parsed.path().ends_with("/markets/fixture/orderbook"));
}
#[tokio::test]
async fn predictions_get_milestones() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_milestones"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetMilestonesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_milestones(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/milestones"));
}
#[tokio::test]
async fn predictions_get_milestone() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_milestone"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetMilestoneParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_milestone(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/milestones/fixture"));
}
#[tokio::test]
async fn predictions_get_multivariate_event_collections() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_multivariate_event_collections"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetMultivariateEventCollectionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_multivariate_event_collections(params)
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
    assert!(parsed.path().ends_with("/multivariate_event_collections"));
}
#[tokio::test]
async fn predictions_get_multivariate_event_collection() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_multivariate_event_collection"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetMultivariateEventCollectionParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_multivariate_event_collection(params)
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
            .ends_with("/multivariate_event_collections/fixture")
    );
}
#[tokio::test]
async fn predictions_create_market_in_multivariate_event_collection() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.create_market_in_multivariate_event_collection"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CreateMarketInMultivariateEventCollectionParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .create_market_in_multivariate_event_collection(params)
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
            .ends_with("/multivariate_event_collections/fixture")
    );
}
#[tokio::test]
async fn predictions_get_balance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_balance"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetBalanceParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_balance(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/portfolio/balance"));
}
#[tokio::test]
async fn predictions_get_credit_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_credit_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetCreditHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_credit_history(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/portfolio/credit_history"));
}
#[tokio::test]
async fn predictions_get_deposits() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_deposits"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetDepositsParams =
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
    assert!(parsed.path().ends_with("/portfolio/deposits"));
}
#[tokio::test]
async fn predictions_cancel_all_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.cancel_all_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CancelAllOrdersParams =
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
    assert!(parsed.path().ends_with("/portfolio/events/orders"));
}
#[tokio::test]
async fn predictions_create_order_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.create_order_v2"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CreateOrderV2Params =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.create_order_v2(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/portfolio/events/orders"));
}
#[tokio::test]
async fn predictions_batch_cancel_orders_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.batch_cancel_orders_v2"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::BatchCancelOrdersV2Params =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .batch_cancel_orders_v2(params)
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
    assert!(parsed.path().ends_with("/portfolio/events/orders/batched"));
}
#[tokio::test]
async fn predictions_batch_create_orders_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.batch_create_orders_v2"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::BatchCreateOrdersV2Params =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .batch_create_orders_v2(params)
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
    assert!(parsed.path().ends_with("/portfolio/events/orders/batched"));
}
#[tokio::test]
async fn predictions_create_order_with_exit_trigger() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.create_order_with_exit_trigger"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CreateOrderWithExitTriggerParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .create_order_with_exit_trigger(params)
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
            .ends_with("/portfolio/events/orders/with_exit_trigger")
    );
}
#[tokio::test]
async fn predictions_cancel_order_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.cancel_order_v2"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CancelOrderV2Params =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.cancel_order_v2(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/portfolio/events/orders/fixture"));
}
#[tokio::test]
async fn predictions_amend_order_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.amend_order_v2"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::AmendOrderV2Params =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.amend_order_v2(params).await.unwrap_or_else(|e| {
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
            .ends_with("/portfolio/events/orders/fixture/amend")
    );
}
#[tokio::test]
async fn predictions_decrease_order_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.decrease_order_v2"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::DecreaseOrderV2Params =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.decrease_order_v2(params).await.unwrap_or_else(|e| {
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
            .ends_with("/portfolio/events/orders/fixture/decrease")
    );
}
#[tokio::test]
async fn predictions_get_fills() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_fills"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetFillsParams =
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
    assert!(parsed.path().ends_with("/portfolio/fills"));
}
#[tokio::test]
async fn predictions_intra_exchange_instance_transfer() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.intra_exchange_instance_transfer"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::IntraExchangeInstanceTransferParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .intra_exchange_instance_transfer(params)
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
            .ends_with("/portfolio/intra_exchange_instance_transfer")
    );
}
#[tokio::test]
async fn predictions_get_intra_exchange_instance_transfers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_intra_exchange_instance_transfers"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetIntraExchangeInstanceTransfersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_intra_exchange_instance_transfers(params)
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
            .ends_with("/portfolio/intra_exchange_instance_transfers")
    );
}
#[tokio::test]
async fn predictions_get_intra_exchange_instance_transfer() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_intra_exchange_instance_transfer"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetIntraExchangeInstanceTransferParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_intra_exchange_instance_transfer(params)
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
            .ends_with("/portfolio/intra_exchange_instance_transfers/fixture")
    );
}
#[tokio::test]
async fn predictions_get_order_groups() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_order_groups"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetOrderGroupsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_order_groups(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/portfolio/order_groups"));
}
#[tokio::test]
async fn predictions_create_order_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.create_order_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CreateOrderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.create_order_group(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/portfolio/order_groups/create"));
}
#[tokio::test]
async fn predictions_delete_order_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.delete_order_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::DeleteOrderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.delete_order_group(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/portfolio/order_groups/fixture"));
}
#[tokio::test]
async fn predictions_get_order_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_order_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetOrderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_order_group(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/portfolio/order_groups/fixture"));
}
#[tokio::test]
async fn predictions_update_order_group_limit() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.update_order_group_limit"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::UpdateOrderGroupLimitParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .update_order_group_limit(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/portfolio/order_groups/fixture/limit")
    );
}
#[tokio::test]
async fn predictions_reset_order_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.reset_order_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::ResetOrderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.reset_order_group(params).await.unwrap_or_else(|e| {
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
    assert!(
        parsed
            .path()
            .ends_with("/portfolio/order_groups/fixture/reset")
    );
}
#[tokio::test]
async fn predictions_trigger_order_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.trigger_order_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::TriggerOrderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .trigger_order_group(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/portfolio/order_groups/fixture/trigger")
    );
}
#[tokio::test]
async fn predictions_get_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetOrdersParams =
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
    assert!(parsed.path().ends_with("/portfolio/orders"));
}
#[tokio::test]
async fn predictions_get_order_queue_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_order_queue_positions"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetOrderQueuePositionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_order_queue_positions(params)
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
    assert!(parsed.path().ends_with("/portfolio/orders/queue_positions"));
}
#[tokio::test]
async fn predictions_get_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_order"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetOrderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_order(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/portfolio/orders/fixture"));
}
#[tokio::test]
async fn predictions_get_order_queue_position() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_order_queue_position"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetOrderQueuePositionParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_order_queue_position(params)
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
            .ends_with("/portfolio/orders/fixture/queue_position")
    );
}
#[tokio::test]
async fn predictions_get_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_positions"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetPositionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_positions(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/portfolio/positions"));
}
#[tokio::test]
async fn predictions_delete_event_contract_exit_trigger() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.delete_event_contract_exit_trigger"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::DeleteEventContractExitTriggerParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .delete_event_contract_exit_trigger(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/portfolio/positions/fixture/exit_trigger")
    );
}
#[tokio::test]
async fn predictions_get_event_contract_exit_triggers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_event_contract_exit_triggers"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetEventContractExitTriggersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_event_contract_exit_triggers(params)
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
            .ends_with("/portfolio/positions/fixture/exit_trigger")
    );
}
#[tokio::test]
async fn predictions_set_event_contract_exit_trigger() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.set_event_contract_exit_trigger"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::SetEventContractExitTriggerParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .set_event_contract_exit_trigger(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/portfolio/positions/fixture/exit_trigger")
    );
}
#[tokio::test]
async fn predictions_get_settlements() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_settlements"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetSettlementsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_settlements(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/portfolio/settlements"));
}
#[tokio::test]
async fn predictions_create_subaccount() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.create_subaccount"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::CreateSubaccountParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.create_subaccount(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/portfolio/subaccounts"));
}
#[tokio::test]
async fn predictions_get_subaccount_balances() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_subaccount_balances"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetSubaccountBalancesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_subaccount_balances(params)
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
    assert!(parsed.path().ends_with("/portfolio/subaccounts/balances"));
}
#[tokio::test]
async fn predictions_get_subaccount_netting() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_subaccount_netting"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetSubaccountNettingParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_subaccount_netting(params)
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
    assert!(parsed.path().ends_with("/portfolio/subaccounts/netting"));
}
#[tokio::test]
async fn predictions_update_subaccount_netting() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.update_subaccount_netting"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::UpdateSubaccountNettingParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .update_subaccount_netting(params)
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
    assert!(parsed.path().ends_with("/portfolio/subaccounts/netting"));
}
#[tokio::test]
async fn predictions_apply_subaccount_transfer() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.apply_subaccount_transfer"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::ApplySubaccountTransferParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .apply_subaccount_transfer(params)
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
    assert!(parsed.path().ends_with("/portfolio/subaccounts/transfer"));
}
#[tokio::test]
async fn predictions_get_subaccount_transfers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_subaccount_transfers"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetSubaccountTransfersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_subaccount_transfers(params)
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
    assert!(parsed.path().ends_with("/portfolio/subaccounts/transfers"));
}
#[tokio::test]
async fn predictions_get_portfolio_resting_order_total_value() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_portfolio_resting_order_total_value"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetPortfolioRestingOrderTotalValueParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_portfolio_resting_order_total_value(params)
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
            .ends_with("/portfolio/summary/total_resting_order_value")
    );
}
#[tokio::test]
async fn predictions_get_target_balance_allocation() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_target_balance_allocation"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetTargetBalanceAllocationParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_target_balance_allocation(params)
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
            .ends_with("/portfolio/target_balance_allocation")
    );
}
#[tokio::test]
async fn predictions_set_target_balance_allocation() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.set_target_balance_allocation"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::SetTargetBalanceAllocationParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .set_target_balance_allocation(params)
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
            .ends_with("/portfolio/target_balance_allocation")
    );
}
#[tokio::test]
async fn predictions_get_withdrawals() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_withdrawals"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetWithdrawalsParams =
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
    assert!(parsed.path().ends_with("/portfolio/withdrawals"));
}
#[tokio::test]
async fn predictions_get_filters_for_sports() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_filters_for_sports"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetFiltersForSportsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_filters_for_sports(params)
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
    assert!(parsed.path().ends_with("/search/filters_by_sport"));
}
#[tokio::test]
async fn predictions_get_tags_for_series_categories() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_tags_for_series_categories"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetTagsForSeriesCategoriesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_tags_for_series_categories(params)
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
    assert!(parsed.path().ends_with("/search/tags_by_categories"));
}
#[tokio::test]
async fn predictions_get_series_list() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_series_list"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetSeriesListParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_series_list(params).await.unwrap_or_else(|e| {
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
async fn predictions_get_series_fee_changes() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_series_fee_changes"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetSeriesFeeChangesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_series_fee_changes(params)
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
    assert!(parsed.path().ends_with("/series/fee_changes"));
}
#[tokio::test]
async fn predictions_get_series() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_series"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetSeriesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_series(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/series/fixture"));
}
#[tokio::test]
async fn predictions_get_market_candlesticks_by_event() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_market_candlesticks_by_event"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetMarketCandlesticksByEventParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_market_candlesticks_by_event(params)
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
            .ends_with("/series/fixture/events/fixture/candlesticks")
    );
}
#[tokio::test]
async fn predictions_get_event_forecast_percentiles_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_event_forecast_percentiles_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetEventForecastPercentilesHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_event_forecast_percentiles_history(params)
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
            .ends_with("/series/fixture/events/fixture/forecast_percentile_history")
    );
}
#[tokio::test]
async fn predictions_get_market_candlesticks() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_market_candlesticks"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetMarketCandlesticksParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_market_candlesticks(params)
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
            .ends_with("/series/fixture/markets/fixture/candlesticks")
    );
}
#[tokio::test]
async fn predictions_get_structured_targets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_structured_targets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetStructuredTargetsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_structured_targets(params)
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
    assert!(parsed.path().ends_with("/structured_targets"));
}
#[tokio::test]
async fn predictions_get_structured_target() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["predictions.get_structured_target"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::predictions::GetStructuredTargetParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_structured_target(params)
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
    assert!(parsed.path().ends_with("/structured_targets/fixture"));
}
#[tokio::test]
async fn perps_get_perps_account_api_limits() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_perps_account_api_limits"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetPerpsAccountApiLimitsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_perps_account_api_limits(params)
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
    assert!(parsed.path().ends_with("/account/limits/perps"));
}
#[tokio::test]
async fn perps_get_margin_balance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_balance"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginBalanceParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_margin_balance(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/margin/balance"));
}
#[tokio::test]
async fn perps_get_margin_corporate_actions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_corporate_actions"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginCorporateActionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_corporate_actions(params)
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
    assert!(parsed.path().ends_with("/margin/corporate_actions"));
}
#[tokio::test]
async fn perps_delete_cross_margin_exit_trigger() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.delete_cross_margin_exit_trigger"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::DeleteCrossMarginExitTriggerParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .delete_cross_margin_exit_trigger(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/margin/cross/positions/fixture/exit_trigger")
    );
}
#[tokio::test]
async fn perps_get_cross_margin_exit_triggers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_cross_margin_exit_triggers"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetCrossMarginExitTriggersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_cross_margin_exit_triggers(params)
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
            .ends_with("/margin/cross/positions/fixture/exit_trigger")
    );
}
#[tokio::test]
async fn perps_set_cross_margin_exit_trigger() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.set_cross_margin_exit_trigger"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::SetCrossMarginExitTriggerParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .set_cross_margin_exit_trigger(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/margin/cross/positions/fixture/exit_trigger")
    );
}
#[tokio::test]
async fn perps_delete_cross_margin_exit_trigger_by_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.delete_cross_margin_exit_trigger_by_id"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::DeleteCrossMarginExitTriggerByIdParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .delete_cross_margin_exit_trigger_by_id(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/margin/cross/positions/fixture/exit_trigger/fixture")
    );
}
#[tokio::test]
async fn perps_update_cross_margin_exit_trigger() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.update_cross_margin_exit_trigger"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::UpdateCrossMarginExitTriggerParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .update_cross_margin_exit_trigger(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/margin/cross/positions/fixture/exit_trigger/fixture")
    );
}
#[tokio::test]
async fn perps_get_margin_enabled() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_enabled"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginEnabledParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_margin_enabled(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/margin/enabled"));
}
#[tokio::test]
async fn perps_get_margin_exchange_status() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_exchange_status"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginExchangeStatusParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_exchange_status(params)
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
    assert!(parsed.path().ends_with("/margin/exchange/status"));
}
#[tokio::test]
async fn perps_delete_fcm_notional_risk_limit() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.delete_fcm_notional_risk_limit"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::DeleteFcmNotionalRiskLimitParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .delete_fcm_notional_risk_limit(params)
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
    assert!(parsed.path().ends_with("/margin/fcm/notional_risk_limit"));
}
#[tokio::test]
async fn perps_update_fcm_notional_risk_limit() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.update_fcm_notional_risk_limit"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::UpdateFcmNotionalRiskLimitParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .update_fcm_notional_risk_limit(params)
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
    assert!(parsed.path().ends_with("/margin/fcm/notional_risk_limit"));
}
#[tokio::test]
async fn perps_create_margin_fcm_subtrader() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.create_margin_fcm_subtrader"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::CreateMarginFcmSubtraderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .create_margin_fcm_subtrader(params)
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
    assert!(parsed.path().ends_with("/margin/fcm/subtraders"));
}
#[tokio::test]
async fn perps_delete_fcm_subtrader_risk_controls() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.delete_fcm_subtrader_risk_controls"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::DeleteFcmSubtraderRiskControlsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .delete_fcm_subtrader_risk_controls(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/margin/fcm/subtraders/risk_controls")
    );
}
#[tokio::test]
async fn perps_get_fcm_subtrader_risk_controls() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_fcm_subtrader_risk_controls"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetFcmSubtraderRiskControlsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_fcm_subtrader_risk_controls(params)
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
            .ends_with("/margin/fcm/subtraders/risk_controls")
    );
}
#[tokio::test]
async fn perps_update_fcm_subtrader_risk_controls() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.update_fcm_subtrader_risk_controls"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::UpdateFcmSubtraderRiskControlsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .update_fcm_subtrader_risk_controls(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/margin/fcm/subtraders/risk_controls")
    );
}
#[tokio::test]
async fn perps_get_margin_fee_tier_rates() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_fee_tier_rates"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginFeeTierRatesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_fee_tier_rates(params)
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
    assert!(parsed.path().ends_with("/margin/fee_tier_rates"));
}
#[tokio::test]
async fn perps_get_margin_fee_tiers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_fee_tiers"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginFeeTiersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_fee_tiers(params)
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
    assert!(parsed.path().ends_with("/margin/fee_tiers"));
}
#[tokio::test]
async fn perps_get_margin_fills() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_fills"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginFillsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_margin_fills(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/margin/fills"));
}
#[tokio::test]
async fn perps_get_margin_funding_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_funding_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginFundingHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_funding_history(params)
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
    assert!(parsed.path().ends_with("/margin/funding_history"));
}
#[tokio::test]
async fn perps_get_margin_funding_rate_estimate() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_funding_rate_estimate"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginFundingRateEstimateParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_funding_rate_estimate(params)
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
    assert!(parsed.path().ends_with("/margin/funding_rates/estimate"));
}
#[tokio::test]
async fn perps_get_margin_historical_funding_rates() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_historical_funding_rates"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginHistoricalFundingRatesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_historical_funding_rates(params)
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
    assert!(parsed.path().ends_with("/margin/funding_rates/historical"));
}
#[tokio::test]
async fn perps_get_margin_premium_index() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_premium_index"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginPremiumIndexParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_premium_index(params)
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
            .ends_with("/margin/funding_rates/premium_index")
    );
}
#[tokio::test]
async fn perps_delete_isolated_margin_exit_trigger() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.delete_isolated_margin_exit_trigger"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::DeleteIsolatedMarginExitTriggerParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .delete_isolated_margin_exit_trigger(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/margin/isolated/positions/fixture/exit_trigger")
    );
}
#[tokio::test]
async fn perps_get_isolated_margin_exit_triggers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_isolated_margin_exit_triggers"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetIsolatedMarginExitTriggersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_isolated_margin_exit_triggers(params)
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
            .ends_with("/margin/isolated/positions/fixture/exit_trigger")
    );
}
#[tokio::test]
async fn perps_set_isolated_margin_exit_trigger() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.set_isolated_margin_exit_trigger"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::SetIsolatedMarginExitTriggerParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .set_isolated_margin_exit_trigger(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/margin/isolated/positions/fixture/exit_trigger")
    );
}
#[tokio::test]
async fn perps_get_margin_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_markets"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginMarketsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_margin_markets(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/margin/markets"));
}
#[tokio::test]
async fn perps_get_margin_market_orderbooks() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_market_orderbooks"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginMarketOrderbooksParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_market_orderbooks(params)
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
    assert!(parsed.path().ends_with("/margin/markets/orderbooks"));
}
#[tokio::test]
async fn perps_get_margin_market_schedules() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_market_schedules"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginMarketSchedulesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_market_schedules(params)
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
    assert!(parsed.path().ends_with("/margin/markets/schedules"));
}
#[tokio::test]
async fn perps_get_margin_market() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_market"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginMarketParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_margin_market(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/margin/markets/fixture"));
}
#[tokio::test]
async fn perps_get_margin_market_candlesticks() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_market_candlesticks"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginMarketCandlesticksParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_market_candlesticks(params)
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
            .ends_with("/margin/markets/fixture/candlesticks")
    );
}
#[tokio::test]
async fn perps_get_margin_market_orderbook() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_market_orderbook"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginMarketOrderbookParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_market_orderbook(params)
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
    assert!(parsed.path().ends_with("/margin/markets/fixture/orderbook"));
}
#[tokio::test]
async fn perps_get_margin_notional_risk_limit() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_notional_risk_limit"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginNotionalRiskLimitParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_notional_risk_limit(params)
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
    assert!(parsed.path().ends_with("/margin/notional_risk_limit"));
}
#[tokio::test]
async fn perps_get_margin_order_groups() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_order_groups"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginOrderGroupsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_order_groups(params)
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
    assert!(parsed.path().ends_with("/margin/order_groups"));
}
#[tokio::test]
async fn perps_create_margin_order_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.create_margin_order_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::CreateMarginOrderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .create_margin_order_group(params)
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
    assert!(parsed.path().ends_with("/margin/order_groups/create"));
}
#[tokio::test]
async fn perps_delete_margin_order_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.delete_margin_order_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::DeleteMarginOrderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .delete_margin_order_group(params)
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
    assert!(parsed.path().ends_with("/margin/order_groups/fixture"));
}
#[tokio::test]
async fn perps_get_margin_order_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_order_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginOrderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_order_group(params)
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
    assert!(parsed.path().ends_with("/margin/order_groups/fixture"));
}
#[tokio::test]
async fn perps_update_margin_order_group_limit() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.update_margin_order_group_limit"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::UpdateMarginOrderGroupLimitParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .update_margin_order_group_limit(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/margin/order_groups/fixture/limit")
    );
}
#[tokio::test]
async fn perps_reset_margin_order_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.reset_margin_order_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::ResetMarginOrderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .reset_margin_order_group(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/margin/order_groups/fixture/reset")
    );
}
#[tokio::test]
async fn perps_trigger_margin_order_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.trigger_margin_order_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::TriggerMarginOrderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .trigger_margin_order_group(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/margin/order_groups/fixture/trigger")
    );
}
#[tokio::test]
async fn perps_cancel_all_margin_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.cancel_all_margin_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::CancelAllMarginOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .cancel_all_margin_orders(params)
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
    assert!(parsed.path().ends_with("/margin/orders"));
}
#[tokio::test]
async fn perps_get_margin_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_orders"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginOrdersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_margin_orders(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/margin/orders"));
}
#[tokio::test]
async fn perps_create_margin_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.create_margin_order"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::CreateMarginOrderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .create_margin_order(params)
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
    assert!(parsed.path().ends_with("/margin/orders"));
}
#[tokio::test]
async fn perps_cancel_margin_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.cancel_margin_order"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::CancelMarginOrderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .cancel_margin_order(params)
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
    assert!(parsed.path().ends_with("/margin/orders/fixture"));
}
#[tokio::test]
async fn perps_get_margin_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_order"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginOrderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_margin_order(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/margin/orders/fixture"));
}
#[tokio::test]
async fn perps_amend_margin_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.amend_margin_order"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::AmendMarginOrderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.amend_margin_order(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/margin/orders/fixture/amend"));
}
#[tokio::test]
async fn perps_decrease_margin_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.decrease_margin_order"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::DecreaseMarginOrderParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .decrease_margin_order(params)
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
    assert!(parsed.path().ends_with("/margin/orders/fixture/decrease"));
}
#[tokio::test]
async fn perps_get_margin_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_positions"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginPositionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_positions(params)
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
    assert!(parsed.path().ends_with("/margin/positions"));
}
#[tokio::test]
async fn perps_get_margin_risk() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_risk"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginRiskParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_margin_risk(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/margin/risk"));
}
#[tokio::test]
async fn perps_get_margin_risk_parameters() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_risk_parameters"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginRiskParametersParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .get_margin_risk_parameters(params)
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
    assert!(parsed.path().ends_with("/margin/risk_parameters"));
}
#[tokio::test]
async fn perps_get_margin_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.get_margin_trades"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::GetMarginTradesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client.get_margin_trades(params).await.unwrap_or_else(|e| {
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
    assert!(parsed.path().ends_with("/margin/trades"));
}
#[tokio::test]
async fn perps_intra_exchange_instance_transfer() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.intra_exchange_instance_transfer"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::IntraExchangeInstanceTransferParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .intra_exchange_instance_transfer(params)
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
            .ends_with("/portfolio/intra_exchange_instance_transfer")
    );
}
#[tokio::test]
async fn perps_create_margin_subaccount() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.create_margin_subaccount"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::CreateMarginSubaccountParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .create_margin_subaccount(params)
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
    assert!(parsed.path().ends_with("/portfolio/margin/subaccounts"));
}
#[tokio::test]
async fn perps_apply_margin_subaccount_transfer() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["perps.apply_margin_subaccount_transfer"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::perps::ApplyMarginSubaccountTransferParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .apply_margin_subaccount_transfer(params)
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
            .ends_with("/portfolio/margin/subaccounts/transfer")
    );
}
#[tokio::test]
async fn clearing_clearing_list_margin_fcm_api_keys() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_list_margin_fcm_api_keys"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingListMarginFcmApiKeysParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_list_margin_fcm_api_keys(params)
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
    assert!(parsed.path().ends_with("/fcm/margin/api_keys"));
}
#[tokio::test]
async fn clearing_clearing_create_margin_fcm_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_create_margin_fcm_api_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingCreateMarginFcmApiKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_create_margin_fcm_api_key(params)
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
    assert!(parsed.path().ends_with("/fcm/margin/api_keys"));
}
#[tokio::test]
async fn clearing_clearing_generate_margin_fcm_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_generate_margin_fcm_api_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGenerateMarginFcmApiKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_generate_margin_fcm_api_key(params)
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
    assert!(parsed.path().ends_with("/fcm/margin/api_keys/generate"));
}
#[tokio::test]
async fn clearing_clearing_delete_margin_fcm_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_delete_margin_fcm_api_key"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingDeleteMarginFcmApiKeyParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_delete_margin_fcm_api_key(params)
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
    assert!(parsed.path().ends_with("/fcm/margin/api_keys/fixture"));
}
#[tokio::test]
async fn clearing_clearing_get_margin_subtrader_groups() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_margin_subtrader_groups"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetMarginSubtraderGroupsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_margin_subtrader_groups(params)
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
    assert!(parsed.path().ends_with("/fcm/margin/subtrader_groups"));
}
#[tokio::test]
async fn clearing_clearing_create_margin_subtrader_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_create_margin_subtrader_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingCreateMarginSubtraderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_create_margin_subtrader_group(params)
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
    assert!(parsed.path().ends_with("/fcm/margin/subtrader_groups"));
}
#[tokio::test]
async fn clearing_clearing_delete_margin_subtrader_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_delete_margin_subtrader_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingDeleteMarginSubtraderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_delete_margin_subtrader_group(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/fcm/margin/subtrader_groups/fixture")
    );
}
#[tokio::test]
async fn clearing_clearing_update_margin_subtrader_group() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_update_margin_subtrader_group"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingUpdateMarginSubtraderGroupParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_update_margin_subtrader_group(params)
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
    assert!(
        parsed
            .path()
            .ends_with("/fcm/margin/subtrader_groups/fixture")
    );
}
#[tokio::test]
async fn clearing_clearing_get_active_margin_obligations() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_active_margin_obligations"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetActiveMarginObligationsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_active_margin_obligations(params)
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
    assert!(parsed.path().ends_with("/margin/active_obligations"));
}
#[tokio::test]
async fn clearing_clearing_estimate_portfolio_maintenance_margin() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_estimate_portfolio_maintenance_margin"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingEstimatePortfolioMaintenanceMarginParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_estimate_portfolio_maintenance_margin(params)
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
            .ends_with("/margin/estimate_maintenance_margin")
    );
}
#[tokio::test]
async fn clearing_clearing_get_maintenance_margin_metadata() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_maintenance_margin_metadata"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetMaintenanceMarginMetadataParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_maintenance_margin_metadata(params)
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
            .ends_with("/margin/estimate_maintenance_margin/metadata")
    );
}
#[tokio::test]
async fn clearing_clearing_get_funding_estimate_by_asset_class() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_funding_estimate_by_asset_class"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetFundingEstimateByAssetClassParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_funding_estimate_by_asset_class(params)
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
            .ends_with("/margin/funding_estimate_by_asset_class")
    );
}
#[tokio::test]
async fn clearing_clearing_get_member_funding_payments() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_member_funding_payments"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetMemberFundingPaymentsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_member_funding_payments(params)
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
    assert!(parsed.path().ends_with("/margin/funding_payments"));
}
#[tokio::test]
async fn clearing_clearing_get_margin_funding_schedule() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_margin_funding_schedule"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetMarginFundingScheduleParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_margin_funding_schedule(params)
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
    assert!(parsed.path().ends_with("/margin/funding_schedule"));
}
#[tokio::test]
async fn clearing_clearing_get_guaranty_fund_balance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_guaranty_fund_balance"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetGuarantyFundBalanceParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_guaranty_fund_balance(params)
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
    assert!(parsed.path().ends_with("/margin/guaranty_fund_balance"));
}
#[tokio::test]
async fn clearing_clearing_get_large_trader_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_large_trader_positions"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetLargeTraderPositionsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_large_trader_positions(params)
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
    assert!(parsed.path().ends_with("/margin/large_trader_positions"));
}
#[tokio::test]
async fn clearing_clearing_get_obligation_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_obligation_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetObligationHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_obligation_history(params)
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
    assert!(parsed.path().ends_with("/margin/obligation_history"));
}
#[tokio::test]
async fn clearing_clearing_get_obligation_corporate_action_payments() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_obligation_corporate_action_payments"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetObligationCorporateActionPaymentsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_obligation_corporate_action_payments(params)
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
            .ends_with("/margin/obligations/fixture/corporate_action_payments")
    );
}
#[tokio::test]
async fn clearing_clearing_get_obligation_funding_payments() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_obligation_funding_payments"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetObligationFundingPaymentsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_obligation_funding_payments(params)
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
            .ends_with("/margin/obligations/fixture/funding_payments")
    );
}
#[tokio::test]
async fn clearing_clearing_get_obligation_maintenance_margin_details() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_obligation_maintenance_margin_details"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetObligationMaintenanceMarginDetailsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_obligation_maintenance_margin_details(params)
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
            .ends_with("/margin/obligations/fixture/maintenance_margin_details")
    );
}
#[tokio::test]
async fn clearing_clearing_get_obligation_settlement_details() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_obligation_settlement_details"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetObligationSettlementDetailsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_obligation_settlement_details(params)
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
            .ends_with("/margin/obligations/fixture/settlement_details")
    );
}
#[tokio::test]
async fn clearing_clearing_get_margin_reports() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_margin_reports"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetMarginReportsParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_margin_reports(params)
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
    assert!(parsed.path().ends_with("/margin/reports"));
}
#[tokio::test]
async fn clearing_clearing_get_settlement_balance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_settlement_balance"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetSettlementBalanceParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_settlement_balance(params)
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
    assert!(parsed.path().ends_with("/margin/settlement_balance"));
}
#[tokio::test]
async fn clearing_clearing_get_settlement_balance_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_settlement_balance_history"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetSettlementBalanceHistoryParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_settlement_balance_history(params)
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
            .ends_with("/margin/settlement_balance_history")
    );
}
#[tokio::test]
async fn clearing_clearing_get_settlement_balance_withdrawal() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_settlement_balance_withdrawal"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetSettlementBalanceWithdrawalParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_settlement_balance_withdrawal(params)
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
            .ends_with("/margin/settlement_balance_withdrawal")
    );
}
#[tokio::test]
async fn clearing_clearing_withdraw_settlement_balance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_withdraw_settlement_balance"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingWithdrawSettlementBalanceParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_withdraw_settlement_balance(params)
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
            .ends_with("/margin/settlement_balance_withdrawal")
    );
}
#[tokio::test]
async fn clearing_clearing_get_settlement_estimate_by_asset_class() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_settlement_estimate_by_asset_class"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetSettlementEstimateByAssetClassParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_settlement_estimate_by_asset_class(params)
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
            .ends_with("/margin/settlement_estimate_by_asset_class")
    );
}
#[tokio::test]
async fn clearing_clearing_get_settlement_prices() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let fixture = &fixtures["clearing.clearing_get_settlement_prices"];
    let mock = Mock {
        response: fixture["response"].clone(),
        sent: Default::default(),
    };
    let client =
        v::KalshiPerpsClient::with_transport(v::Kalshi::mainnet(), mock.clone()).with_auth(Auth);
    let params: v::clearing::ClearingGetSettlementPricesParams =
        serde_json::from_value(fixture["request"].clone()).unwrap();
    let result = client
        .clearing_get_settlement_prices(params)
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
    assert!(parsed.path().ends_with("/margin/settlement_prices"));
}
