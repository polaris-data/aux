// Generated contract fixtures; synthetic, not funded verification.
use aux_rs_core::{HttpRequest, HttpResponse, HttpTransport, TransportError};
use aux_rs_extended::*;
use serde_json::Value;
use std::sync::{Arc, Mutex};
#[derive(Clone)]
struct Mock {
    response: Vec<u8>,
    sent: Arc<Mutex<Vec<HttpRequest>>>,
}
impl HttpTransport for Mock {
    async fn execute(&self, r: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.sent.lock().unwrap().push(r);
        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: self.response.clone(),
        })
    }
}
#[tokio::test]
async fn contract_get_assets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_assets"];
    let request: GetAssetsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_assets(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_asset_price() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_asset_price"];
    let request: GetAssetPriceParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_asset_price(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_markets"];
    let request: GetMarketsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_markets(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_markets_stats() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_markets_stats"];
    let request: GetMarketsStatsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_markets_stats(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_market_statistics() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_market_statistics"];
    let request: GetMarketStatisticsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_market_statistics(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_market_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_market_trades"];
    let request: GetMarketTradesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_market_trades(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_orderbook_snapshot() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_orderbook_snapshot"];
    let request: GetOrderbookSnapshotParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_orderbook_snapshot(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_funding_rates_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_funding_rates_history"];
    let request: GetFundingRatesHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_funding_rates_history(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_trading_candles() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trading_candles"];
    let request: GetTradingCandlesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trading_candles(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_mark_price_candles() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_mark_price_candles"];
    let request: GetMarkPriceCandlesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_mark_price_candles(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_index_price_candles() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_index_price_candles"];
    let request: GetIndexPriceCandlesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_index_price_candles(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_open_interests() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_open_interests"];
    let request: GetOpenInterestsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_open_interests(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_public_leaderboard() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_public_leaderboard"];
    let request: GetPublicLeaderboardParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_public_leaderboard(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_public_referral_link() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_public_referral_link"];
    let request: GetPublicReferralLinkParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_public_referral_link(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_builder_dashboard() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_builder_dashboard"];
    let request: GetBuilderDashboardParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_builder_dashboard(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_vault_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_vault_positions"];
    let request: GetVaultPositionsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_vault_positions(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_vault_position_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_vault_position_history"];
    let request: GetVaultPositionHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_vault_position_history(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_vault_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_vault_orders"];
    let request: GetVaultOrdersParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_vault_orders(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_vault_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_vault_trades"];
    let request: GetVaultTradesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_vault_trades(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_vault_funding_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_vault_funding_history"];
    let request: GetVaultFundingHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_vault_funding_history(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_exchange_status() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_exchange_status"];
    let request: GetExchangeStatusParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_exchange_status(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_settings() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_settings"];
    let request: GetSettingsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_settings(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_starknet_domain() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_starknet_domain"];
    let request: GetStarknetDomainParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_starknet_domain(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_accounts() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_accounts"];
    let request: GetAccountsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_accounts(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_account() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account"];
    let request: GetAccountParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_account(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_update_account_info() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_account_info"];
    let request: UpdateAccountInfoParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.update_account_info(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Patch);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_leverage() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_leverage"];
    let request: GetLeverageParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_leverage(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_update_leverage() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_leverage"];
    let request: UpdateLeverageParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.update_leverage(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Patch);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_api_keys() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_api_keys"];
    let request: GetApiKeysParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_api_keys(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_create_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["create_api_key"];
    let request: CreateApiKeyParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.create_api_key(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_delete_api_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["delete_api_key"];
    let request: DeleteApiKeyParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.delete_api_key(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Delete);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_balance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_balance"];
    let request: GetBalanceParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_balance(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_spot_balances() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_spot_balances"];
    let request: GetSpotBalancesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_spot_balances(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_positions"];
    let request: GetPositionsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_positions(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_positions_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_positions_history"];
    let request: GetPositionsHistoryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_positions_history(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_open_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_open_orders"];
    let request: GetOpenOrdersParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_open_orders(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_order"];
    let request: GetOrderParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_order(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_orders_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_orders_history"];
    let request: GetOrdersHistoryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_orders_history(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_order_by_external_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_order_by_external_id"];
    let request: GetOrderByExternalIdParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_order_by_external_id(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_place_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["place_order"];
    let request: PlaceOrderParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.place_order(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_cancel_order_by_external_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["cancel_order_by_external_id"];
    let request: CancelOrderByExternalIdParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.cancel_order_by_external_id(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Delete);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_create_scaled_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["create_scaled_order"];
    let request: CreateScaledOrderParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.create_scaled_order(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_cancel_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["cancel_order"];
    let request: CancelOrderParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.cancel_order(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Delete);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_mass_cancel() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["mass_cancel"];
    let request: MassCancelParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.mass_cancel(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trades"];
    let request: GetTradesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trades(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_user_funding_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_user_funding_history"];
    let request: GetUserFundingHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_user_funding_history(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_update_funding_rate_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_funding_rate_config"];
    let request: UpdateFundingRateConfigParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.update_funding_rate_config(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_transfer_patch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["transfer_patch"];
    let request: TransferPatchParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.transfer_patch(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Patch);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_transfer() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["transfer"];
    let request: TransferParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.transfer(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_post_user_transfer_onchain() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["post_user_transfer_onchain"];
    let request: PostUserTransferOnchainParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.post_user_transfer_onchain(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_withdrawals() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_withdrawals"];
    let request: GetWithdrawalsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_withdrawals(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_withdraw() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["withdraw"];
    let request: WithdrawParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.withdraw(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_onchain_withdrawal() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["onchain_withdrawal"];
    let request: OnchainWithdrawalParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.onchain_withdrawal(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_create_deposit() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["create_deposit"];
    let request: CreateDepositParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.create_deposit(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_bridge_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_bridge_config"];
    let request: GetBridgeConfigParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_bridge_config(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_bridge_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_bridge_quote"];
    let request: GetBridgeQuoteParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_bridge_quote(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_commit_bridge_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["commit_bridge_quote"];
    let request: CommitBridgeQuoteParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.commit_bridge_quote(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_validate_wallet() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["validate_wallet"];
    let request: ValidateWalletParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.validate_wallet(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_wallet_validation() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_wallet_validation"];
    let request: GetWalletValidationParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_wallet_validation(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_fees() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_fees"];
    let request: GetFeesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_fees(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_trading_fee_stats() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trading_fee_stats"];
    let request: GetTradingFeeStatsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trading_fee_stats(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_user_leaderboard() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_user_leaderboard"];
    let request: GetUserLeaderboardParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_user_leaderboard(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_set_deadman_switch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["set_deadman_switch"];
    let request: SetDeadmanSwitchParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.set_deadman_switch(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_claims() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_claims"];
    let request: GetClaimsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_claims(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_claim_testing_funds() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["claim_testing_funds"];
    let request: ClaimTestingFundsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.claim_testing_funds(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_claim() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_claim"];
    let request: GetClaimParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_claim(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_next_claim_time() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_next_claim_time"];
    let request: GetNextClaimTimeParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_next_claim_time(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_partner_rewards() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_partner_rewards"];
    let request: GetPartnerRewardsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_partner_rewards(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_affiliate_rewards() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_affiliate_rewards"];
    let request: GetAffiliateRewardsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_affiliate_rewards(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_earned_rewards() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_earned_rewards"];
    let request: GetEarnedRewardsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_earned_rewards(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_reward_leaderboard_stats() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_reward_leaderboard_stats"];
    let request: GetRewardLeaderboardStatsParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_reward_leaderboard_stats(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_rebates_csv() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_rebates_csv"];
    let request: GetRebatesCsvParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_rebates_csv(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_rebate_stats() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_rebate_stats"];
    let request: GetRebateStatsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_rebate_stats(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_referral_status() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_referral_status"];
    let request: GetReferralStatusParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_referral_status(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_referral_links() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_referral_links"];
    let request: GetReferralLinksParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_referral_links(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_create_referral_link() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["create_referral_link"];
    let request: CreateReferralLinkParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.create_referral_link(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_update_referral_link() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_referral_link"];
    let request: UpdateReferralLinkParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.update_referral_link(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Put);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_use_referral_code() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["use_referral_code"];
    let request: UseReferralCodeParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.use_referral_code(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_referral_rebates_by_period_csv() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_referral_rebates_by_period_csv"];
    let request: GetReferralRebatesByPeriodCsvParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_referral_rebates_by_period_csv(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_referral_rebates_by_user_csv() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_referral_rebates_by_user_csv"];
    let request: GetReferralRebatesByUserCsvParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_referral_rebates_by_user_csv(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_referral_rebates_by_subaffiliate_csv() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_referral_rebates_by_subaffiliate_csv"];
    let request: GetReferralRebatesBySubaffiliateCsvParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_referral_rebates_by_subaffiliate_csv(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_referral_rebates_by_link_csv() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_referral_rebates_by_link_csv"];
    let request: GetReferralRebatesByLinkCsvParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_referral_rebates_by_link_csv(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_referral_dashboard() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_referral_dashboard"];
    let request: GetReferralDashboardParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_referral_dashboard(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_affiliate() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_affiliate"];
    let request: GetAffiliateParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_affiliate(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_asset_operations() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["asset_operations"];
    let request: AssetOperationsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.asset_operations(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_asset_operations() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_asset_operations"];
    let request: GetAssetOperationsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_asset_operations(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_client() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_client"];
    let request: GetClientParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_client(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_mark_wallet_checked() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["mark_wallet_checked"];
    let request: MarkWalletCheckedParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.mark_wallet_checked(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Put);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_export_csv_zip() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["export_csv_zip"];
    let request: ExportCsvZipParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.export_csv_zip(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_wrapper() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_wrapper"];
    let request: GetWrapperParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_wrapper(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_place_rfq_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["place_rfq_order"];
    let request: PlaceRfqOrderParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.place_rfq_order(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_delete_user_order_order_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["delete_user_order_order_id"];
    let request: DeleteUserOrderOrderIdParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.delete_user_order_order_id(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Delete);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_builder_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_builder_trades"];
    let request: GetBuilderTradesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_builder_trades(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_info_assets_asset_name_price() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_info_assets_asset_name_price"];
    let request: GetInfoAssetsAssetNamePriceParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_info_assets_asset_name_price(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_candles_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_candles_history"];
    let request: GetCandlesHistoryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_candles_history(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_deposit_to_vault() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["deposit_to_vault"];
    let request: DepositToVaultParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.deposit_to_vault(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_withdraw_from_vault() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["withdraw_from_vault"];
    let request: WithdrawFromVaultParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.withdraw_from_vault(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_order_by_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_order_by_id"];
    let request: GetOrderByIdParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_order_by_id(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_user_orders_external_external_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_user_orders_external_external_id"];
    let request: GetUserOrdersExternalExternalIdParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_user_orders_external_external_id(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_onboard_client() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["onboard_client"];
    let request: OnboardClientParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.onboard_client(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_onboard_subaccount() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["onboard_subaccount"];
    let request: OnboardSubaccountParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.onboard_subaccount(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Post);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_info_candles_market_candle_type() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_info_candles_market_candle_type"];
    let request: GetInfoCandlesMarketCandleTypeParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_info_candles_market_candle_type(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_account_equity_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_equity_history"];
    let request: GetAccountEquityHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_account_equity_history(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_account_pn_l_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_pn_l_history"];
    let request: GetAccountPnLHistoryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_account_pn_l_history(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_account_pn_l_percentage_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_pn_l_percentage_history"];
    let request: GetAccountPnLPercentageHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_account_pn_l_percentage_history(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_cumulative_account_pn_l_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_cumulative_account_pn_l_history"];
    let request: GetCumulativeAccountPnLHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_cumulative_account_pn_l_history(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_cumulative_account_pn_l_percentage_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_cumulative_account_pn_l_percentage_history"];
    let request: GetCumulativeAccountPnLPercentageHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_cumulative_account_pn_l_percentage_history(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_account_vault_equity_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_vault_equity_history"];
    let request: GetAccountVaultEquityHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_account_vault_equity_history(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_account_max_drawdown_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_max_drawdown_history"];
    let request: GetAccountMaxDrawdownHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_account_max_drawdown_history(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_account_funding_chart() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_funding_chart"];
    let request: GetAccountFundingChartParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_account_funding_chart(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_account_portfolio_summary() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_portfolio_summary"];
    let request: GetAccountPortfolioSummaryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_account_portfolio_summary(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_account_health() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_health"];
    let request: GetAccountHealthParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_account_health(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_account_performance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_performance"];
    let request: GetAccountPerformanceParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_account_performance(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_account_funding_stats() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_funding_stats"];
    let request: GetAccountFundingStatsParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_account_funding_stats(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_account_funding_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_funding_history"];
    let request: GetAccountFundingHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_account_funding_history(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_vault_performance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_vault_performance"];
    let request: GetVaultPerformanceParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_vault_performance(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_vault_summary() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_vault_summary"];
    let request: GetVaultSummaryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_vault_summary(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_interest_rate_curves_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_interest_rate_curves_history"];
    let request: GetInterestRateCurvesHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_interest_rate_curves_history(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_latest_interest_rate_curve() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_latest_interest_rate_curve"];
    let request: GetLatestInterestRateCurveParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_latest_interest_rate_curve(request)
        .await
        .unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_interest_key_metrics() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_interest_key_metrics"];
    let request: GetInterestKeyMetricsParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_interest_key_metrics(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_interest_daily_metrics() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_interest_daily_metrics"];
    let request: GetInterestDailyMetricsParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_interest_daily_metrics(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_interest_payment_chart() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_interest_payment_chart"];
    let request: GetInterestPaymentChartParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_interest_payment_chart(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
#[tokio::test]
async fn contract_get_interest_payments_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_interest_payments_history"];
    let request: GetInterestPaymentsHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = ExtendedClient::with_transport(Extended::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_interest_payments_history(request).await.unwrap();
    let result = serde_json::to_value(result).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(result["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method, aux_rs_core::HttpMethod::Get);
    assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));
    let body = if requests[0].body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&requests[0].body).unwrap()
    };
    assert_eq!(body, f["body"]);
}
