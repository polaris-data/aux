// Generated contract fixtures; synthetic, not funded verification.
use aux_rs_core::{HttpRequest, HttpResponse, HttpTransport, TransportError};
use aux_rs_phoenix::*;
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
async fn contract_get_exchange() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_exchange"];
    let request: GetExchangeParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_exchange(request).await.unwrap();
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
async fn contract_get_exchange_keys() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_exchange_keys"];
    let request: GetExchangeKeysParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_exchange_keys(request).await.unwrap();
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
async fn contract_list_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["list_markets"];
    let request: ListMarketsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.list_markets(request).await.unwrap();
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
async fn contract_get_market() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_market"];
    let request: GetMarketParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_market(request).await.unwrap();
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
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
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
async fn contract_get_exchange_snapshot() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_exchange_snapshot"];
    let request: GetExchangeSnapshotParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_exchange_snapshot(request).await.unwrap();
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
async fn contract_build_register_trader_instructions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["build_register_trader_instructions"];
    let request: BuildRegisterTraderInstructionsParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .build_register_trader_instructions(request)
        .await
        .unwrap();
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
async fn contract_submit_register_trader_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["submit_register_trader_transaction"];
    let request: SubmitRegisterTraderTransactionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .submit_register_trader_transaction(request)
        .await
        .unwrap();
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
async fn contract_get_orderbook() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_orderbook"];
    let request: GetOrderbookParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_orderbook(request).await.unwrap();
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
async fn contract_get_mark_price() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_mark_price"];
    let request: GetMarkPriceParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_mark_price(request).await.unwrap();
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
async fn contract_get_market_stats_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_market_stats_history"];
    let request: GetMarketStatsHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_market_stats_history(request).await.unwrap();
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
async fn contract_get_next_market_calendar_transition() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_next_market_calendar_transition"];
    let request: GetNextMarketCalendarTransitionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_next_market_calendar_transition(request)
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
async fn contract_get_next_commodity_market_transition() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_next_commodity_market_transition"];
    let request: GetNextCommodityMarketTransitionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_next_commodity_market_transition(request)
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
async fn contract_get_market_calendar() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_market_calendar"];
    let request: GetMarketCalendarParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_market_calendar(request).await.unwrap();
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
async fn contract_get_market_calendar_by_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_market_calendar_by_id"];
    let request: GetMarketCalendarByIdParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_market_calendar_by_id(request).await.unwrap();
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
async fn contract_get_commodity_market_calendar() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_commodity_market_calendar"];
    let request: GetCommodityMarketCalendarParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_commodity_market_calendar(request).await.unwrap();
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
async fn contract_get_candles() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_candles"];
    let request: GetCandlesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_candles(request).await.unwrap();
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
async fn contract_get_trader() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trader"];
    let request: GetTraderParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trader(request).await.unwrap();
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
async fn contract_get_trader_capabilities() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trader_capabilities"];
    let request: GetTraderCapabilitiesParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trader_capabilities(request).await.unwrap();
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
async fn contract_get_trader_state() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trader_state"];
    let request: GetTraderStateParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trader_state(request).await.unwrap();
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
async fn contract_get_user_pn_l() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_user_pn_l"];
    let request: GetUserPnLParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_user_pn_l(request).await.unwrap();
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
async fn contract_get_trader_portfolio_values() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trader_portfolio_values"];
    let request: GetTraderPortfolioValuesParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trader_portfolio_values(request).await.unwrap();
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
async fn contract_get_trader_pn_l() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trader_pn_l"];
    let request: GetTraderPnLParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trader_pn_l(request).await.unwrap();
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
async fn contract_get_trader_market_pn_l() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trader_market_pn_l"];
    let request: GetTraderMarketPnLParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trader_market_pn_l(request).await.unwrap();
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
async fn contract_get_user_collateral_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_user_collateral_history"];
    let request: GetUserCollateralHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_user_collateral_history(request).await.unwrap();
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
async fn contract_get_trader_collateral_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trader_collateral_history"];
    let request: GetTraderCollateralHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trader_collateral_history(request).await.unwrap();
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
async fn contract_get_v1_traders_by_trader_pubkey_collateral_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_v1_traders_by_trader_pubkey_collateral_history"];
    let request: GetV1TradersByTraderPubkeyCollateralHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_v1_traders_by_trader_pubkey_collateral_history(request)
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
async fn contract_get_user_liquidation_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_user_liquidation_history"];
    let request: GetUserLiquidationHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_user_liquidation_history(request).await.unwrap();
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
async fn contract_get_trader_funding_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trader_funding_history"];
    let request: GetTraderFundingHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trader_funding_history(request).await.unwrap();
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
async fn contract_get_user_hourly_funding() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_user_hourly_funding"];
    let request: GetUserHourlyFundingParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_user_hourly_funding(request).await.unwrap();
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
async fn contract_get_funding_rate_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_funding_rate_history"];
    let request: GetFundingRateHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_funding_rate_history(request).await.unwrap();
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
async fn contract_get_funding_overview() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_funding_overview"];
    let request: GetFundingOverviewParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_funding_overview(request).await.unwrap();
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
async fn contract_get_order_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_order_history"];
    let request: GetOrderHistoryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_order_history(request).await.unwrap();
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
async fn contract_get_order_history_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_order_history_v2"];
    let request: GetOrderHistoryV2Params = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_order_history_v2(request).await.unwrap();
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
async fn contract_get_trade_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trade_history"];
    let request: GetTradeHistoryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trade_history(request).await.unwrap();
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
async fn contract_get_user_trade_history_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_user_trade_history_v2"];
    let request: GetUserTradeHistoryV2Params =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_user_trade_history_v2(request).await.unwrap();
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
async fn contract_get_trader_trade_history_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_trader_trade_history_v2"];
    let request: GetTraderTradeHistoryV2Params =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_trader_trade_history_v2(request).await.unwrap();
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
async fn contract_get_market_fills() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_market_fills"];
    let request: GetMarketFillsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_market_fills(request).await.unwrap();
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
async fn contract_list_trader_notifications() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["list_trader_notifications"];
    let request: ListTraderNotificationsParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.list_trader_notifications(request).await.unwrap();
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
async fn contract_acknowledge_notifications_up_to_timestamp() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["acknowledge_notifications_up_to_timestamp"];
    let request: AcknowledgeNotificationsUpToTimestampParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .acknowledge_notifications_up_to_timestamp(request)
        .await
        .unwrap();
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
async fn contract_acknowledge_notifications() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["acknowledge_notifications"];
    let request: AcknowledgeNotificationsParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.acknowledge_notifications(request).await.unwrap();
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
async fn contract_build_isolated_limit_order_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["build_isolated_limit_order_transaction"];
    let request: BuildIsolatedLimitOrderTransactionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .build_isolated_limit_order_transaction(request)
        .await
        .unwrap();
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
async fn contract_build_isolated_market_order_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["build_isolated_market_order_transaction"];
    let request: BuildIsolatedMarketOrderTransactionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .build_isolated_market_order_transaction(request)
        .await
        .unwrap();
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
async fn contract_build_cancel_conditional_order_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["build_cancel_conditional_order_transaction"];
    let request: BuildCancelConditionalOrderTransactionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .build_cancel_conditional_order_transaction(request)
        .await
        .unwrap();
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
async fn contract_build_stop_loss_order_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["build_stop_loss_order_transaction"];
    let request: BuildStopLossOrderTransactionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .build_stop_loss_order_transaction(request)
        .await
        .unwrap();
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
async fn contract_build_cancel_stop_loss_order_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["build_cancel_stop_loss_order_transaction"];
    let request: BuildCancelStopLossOrderTransactionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .build_cancel_stop_loss_order_transaction(request)
        .await
        .unwrap();
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
async fn contract_build_attached_conditional_order_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["build_attached_conditional_order_transaction"];
    let request: BuildAttachedConditionalOrderTransactionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .build_attached_conditional_order_transaction(request)
        .await
        .unwrap();
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
async fn contract_build_position_conditional_order_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["build_position_conditional_order_transaction"];
    let request: BuildPositionConditionalOrderTransactionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .build_position_conditional_order_transaction(request)
        .await
        .unwrap();
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
async fn contract_build_isolated_limit_order_transaction_with_conditionals() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["build_isolated_limit_order_transaction_with_conditionals"];
    let request: BuildIsolatedLimitOrderTransactionWithConditionalsParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .build_isolated_limit_order_transaction_with_conditionals(request)
        .await
        .unwrap();
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
async fn contract_build_isolated_limit_order_transaction_with_liquidation_estimate() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["build_isolated_limit_order_transaction_with_liquidation_estimate"];
    let request: BuildIsolatedLimitOrderTransactionWithLiquidationEstimateParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .build_isolated_limit_order_transaction_with_liquidation_estimate(request)
        .await
        .unwrap();
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
async fn contract_build_isolated_market_order_transaction_with_liquidation_estimate() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["build_isolated_market_order_transaction_with_liquidation_estimate"];
    let request: BuildIsolatedMarketOrderTransactionWithLiquidationEstimateParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .build_isolated_market_order_transaction_with_liquidation_estimate(request)
        .await
        .unwrap();
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
async fn contract_validate_invite() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["validate_invite"];
    let request: ValidateInviteParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.validate_invite(request).await.unwrap();
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
async fn contract_check_wallet_invite_status() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["check_wallet_invite_status"];
    let request: CheckWalletInviteStatusParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.check_wallet_invite_status(request).await.unwrap();
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
async fn contract_activate_referral() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["activate_referral"];
    let request: ActivateReferralParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.activate_referral(request).await.unwrap();
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
async fn contract_get_referral_activation_permission() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_referral_activation_permission"];
    let request: GetReferralActivationPermissionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .get_referral_activation_permission(request)
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
async fn contract_activate_referral_with_user_funded_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["activate_referral_with_user_funded_transaction"];
    let request: ActivateReferralWithUserFundedTransactionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .activate_referral_with_user_funded_transaction(request)
        .await
        .unwrap();
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
async fn contract_log_in_with_privy_token() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["log_in_with_privy_token"];
    let request: LogInWithPrivyTokenParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.log_in_with_privy_token(request).await.unwrap();
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
async fn contract_create_wallet_transaction_challenge() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["create_wallet_transaction_challenge"];
    let request: CreateWalletTransactionChallengeParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .create_wallet_transaction_challenge(request)
        .await
        .unwrap();
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
async fn contract_log_in_with_signed_wallet_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["log_in_with_signed_wallet_transaction"];
    let request: LogInWithSignedWalletTransactionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .log_in_with_signed_wallet_transaction(request)
        .await
        .unwrap();
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
async fn contract_create_service_login_challenge() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["create_service_login_challenge"];
    let request: CreateServiceLoginChallengeParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client
        .create_service_login_challenge(request)
        .await
        .unwrap();
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
async fn contract_log_in_with_service_signature() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["log_in_with_service_signature"];
    let request: LogInWithServiceSignatureParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.log_in_with_service_signature(request).await.unwrap();
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
async fn contract_get_wallet_login_nonce() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_wallet_login_nonce"];
    let request: GetWalletLoginNonceParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_wallet_login_nonce(request).await.unwrap();
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
async fn contract_log_in_with_wallet_signature() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["log_in_with_wallet_signature"];
    let request: LogInWithWalletSignatureParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.log_in_with_wallet_signature(request).await.unwrap();
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
async fn contract_refresh_session() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["refresh_session"];
    let request: RefreshSessionParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.refresh_session(request).await.unwrap();
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
async fn contract_log_out() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["log_out"];
    let request: LogOutParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.log_out(request).await.unwrap();
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
async fn contract_get_jwks() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_jwks"];
    let request: GetJwksParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"]).unwrap(),
        sent: sent.clone(),
    };
    let client = PhoenixClient::with_transport(Phoenix::mainnet(), transport).with_auth(
        Credentials::new(None, Some(zeroize::Zeroizing::new("fixture".into())), None).unwrap(),
    );
    let result = client.get_jwks(request).await.unwrap();
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
