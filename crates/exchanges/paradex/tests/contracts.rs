// Generated from the pinned specification. Fixtures are synthetic contract examples.
use aux_rs_core::{Error as ClientError, HttpRequest, HttpResponse, HttpTransport};
use aux_rs_paradex::*;
use serde_json::Value;
use std::sync::{Arc, Mutex};
#[derive(Clone)]
struct Mock {
    response: Vec<u8>,
    sent: Arc<Mutex<Vec<HttpRequest>>>,
}
impl HttpTransport for Mock {
    async fn execute(
        &self,
        request: HttpRequest,
    ) -> Result<HttpResponse, aux_rs_core::TransportError> {
        self.sent.lock().unwrap().push(request);
        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: self.response.clone(),
        })
    }
}
#[derive(Clone)]
struct Auth {
    token: TokenState,
}
impl Authentication for Auth {
    fn account(&self) -> &str {
        "0x1"
    }
    fn token(&self) -> &TokenState {
        &self.token
    }
    async fn prepare(
        &self,
        _: &Paradex,
        _: &Operation,
        request: &mut HttpRequest,
    ) -> Result<(), Error> {
        request
            .headers
            .push(("Authorization".into(), "Bearer fixture".into()));
        Ok(())
    }
}
#[tokio::test]
async fn contract_get_account() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account"];
    let request: GetAccountParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountSummaryResponse, ClientError<Error>> =
        client.get_account(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_compliance_actions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_compliance_actions"];
    let request: GetComplianceActionsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_compliance_actions(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_record_compliance_action() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["record_compliance_action"];
    let request: RecordComplianceActionParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.record_compliance_action(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_account_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_history"];
    let request: GetAccountHistoryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountHistoricalDataResp, ClientError<Error>> =
        client.get_account_history(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_accounts_info() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_accounts_info"];
    let request: GetAccountsInfoParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesGetAccountsInfoResponse, ClientError<Error>> =
        client.get_accounts_info(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_list_subkeys() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["list_subkeys"];
    let request: ListSubkeysParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.list_subkeys(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_register_subkey() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["register_subkey"];
    let request: RegisterSubkeyParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.register_subkey(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_activate_subkey() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["activate_subkey"];
    let request: ActivateSubkeyParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<RequestsActivateSubkeyResponse, ClientError<Error>> =
        client.activate_subkey(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_subkey() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_subkey"];
    let request: GetSubkeyParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesSubkey, ClientError<Error>> = client.get_subkey(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_subkey() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_subkey"];
    let request: UpdateSubkeyParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesSubkey, ClientError<Error>> = client.update_subkey(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Put);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_revoke_subkey() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["revoke_subkey"];
    let request: RevokeSubkeyParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesRevokeSubkeyResponse, ClientError<Error>> =
        client.revoke_subkey(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_subkey_allowed_cidrs() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_subkey_allowed_cidrs"];
    let request: UpdateSubkeyAllowedCidrsParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesSubkey, ClientError<Error>> =
        client.update_subkey_allowed_cidrs(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Put);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_subkey_block_trade_signing() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_subkey_block_trade_signing"];
    let request: UpdateSubkeyBlockTradeSigningParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesSubkey, ClientError<Error>> =
        client.update_subkey_block_trade_signing(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Put);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_account_margin() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_margin"];
    let request: GetAccountMarginParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesGetAccountMarginConfigsResp, ClientError<Error>> =
        client.get_account_margin(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_upsert_account_margin() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["upsert_account_margin"];
    let request: UpsertAccountMarginParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesUpdateAccountMarginConfigResp, ClientError<Error>> =
        client.upsert_account_margin(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_account_mmp() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_mmp"];
    let request: GetAccountMmpParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesGetAccountMmpConfigsResp, ClientError<Error>> =
        client.get_account_mmp(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_reset_account_mmp() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["reset_account_mmp"];
    let request: ResetAccountMmpParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountMmpResetResp, ClientError<Error>> =
        client.reset_account_mmp(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_upsert_account_mmp() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["upsert_account_mmp"];
    let request: UpsertAccountMmpParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountMmpConfigResp, ClientError<Error>> =
        client.upsert_account_mmp(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_delete_account_mmp() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["delete_account_mmp"];
    let request: DeleteAccountMmpParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.delete_account_mmp(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_rfq_enable() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["rfq_enable"];
    let request: RfqEnableParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesParadigmEnableResponse, ClientError<Error>> =
        client.rfq_enable(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_account_profile() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_profile"];
    let request: GetAccountProfileParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountProfileResp, ClientError<Error>> =
        client.get_account_profile(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_account_profile() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_account_profile"];
    let request: UpdateAccountProfileParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountProfileResp, ClientError<Error>> =
        client.update_account_profile(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Put);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_account_market_max_slippage() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_account_market_max_slippage"];
    let request: UpdateAccountMarketMaxSlippageParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountProfileResp, ClientError<Error>> =
        client.update_account_market_max_slippage(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_account_notification_preferences() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_account_notification_preferences"];
    let request: UpdateAccountNotificationPreferencesParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountProfileResp, ClientError<Error>> = client
        .update_account_notification_preferences(request)
        .await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_last_seen() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_last_seen"];
    let request: UpdateLastSeenParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountProfileResp, ClientError<Error>> =
        client.update_last_seen(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_account_profile_referral_code() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_account_profile_referral_code"];
    let request: UpdateAccountProfileReferralCodeParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountProfileResp, ClientError<Error>> =
        client.update_account_profile_referral_code(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_refresh_account_inventory() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["refresh_account_inventory"];
    let request: RefreshAccountInventoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesRefreshInventoryResp, ClientError<Error>> =
        client.refresh_account_inventory(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_account_size_currency_display() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_account_size_currency_display"];
    let request: UpdateAccountSizeCurrencyDisplayParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountProfileResp, ClientError<Error>> =
        client.update_account_size_currency_display(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_account_profile_username() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_account_profile_username"];
    let request: UpdateAccountProfileUsernameParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountProfileResp, ClientError<Error>> =
        client.update_account_profile_username(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_account_referrer() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_account_referrer"];
    let request: UpdateAccountReferrerParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountProfileResp, ClientError<Error>> =
        client.update_account_referrer(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_account_settings() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_account_settings"];
    let request: GetAccountSettingsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountSettingsResp, ClientError<Error>> =
        client.get_account_settings(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_account_trading_value_display() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_account_trading_value_display"];
    let request: UpdateAccountTradingValueDisplayParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountProfileResp, ClientError<Error>> =
        client.update_account_trading_value_display(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_sub_accounts() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_sub_accounts"];
    let request: GetSubAccountsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesGetSubAccountsResponse, ClientError<Error>> =
        client.get_sub_accounts(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_archive_sub_account() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["archive_sub_account"];
    let request: ArchiveSubAccountParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountInfoResponse, ClientError<Error>> =
        client.archive_sub_account(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_accounts() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_accounts"];
    let request: GetAccountsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_accounts(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_post_account_terminal_token() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["post_account_terminal_token"];
    let request: PostAccountTerminalTokenParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAuthResp, ClientError<Error>> =
        client.post_account_terminal_token(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_list_tokens() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["list_tokens"];
    let request: ListTokensParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> = client.list_tokens(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_create_token() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["create_token"];
    let request: CreateTokenParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesCreateTokenResponse, ClientError<Error>> =
        client.create_token(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_revoke_token() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["revoke_token"];
    let request: RevokeTokenParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesRevokeTokenResponse, ClientError<Error>> =
        client.revoke_token(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_token_allowed_cidrs() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_token_allowed_cidrs"];
    let request: UpdateTokenAllowedCidrsParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesApiToken, ClientError<Error>> =
        client.update_token_allowed_cidrs(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Put);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_open_algo_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_open_algo_orders"];
    let request: GetOpenAlgoOrdersParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_open_algo_orders(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_create_algo_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["create_algo_order"];
    let request: CreateAlgoOrderParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAlgoOrderResp, ClientError<Error>> =
        client.create_algo_order(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_algo_orders_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_algo_orders_history"];
    let request: GetAlgoOrdersHistoryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_algo_orders_history(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_algo_order_by_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_algo_order_by_id"];
    let request: GetAlgoOrderByIdParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAlgoOrderResp, ClientError<Error>> =
        client.get_algo_order_by_id(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_cancel_algo_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["cancel_algo_order"];
    let request: CancelAlgoOrderParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.cancel_algo_order(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_jwt() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_jwt"];
    let request: GetJwtParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesAuthResp, ClientError<Error>> = client.get_jwt(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_balance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_balance"];
    let request: GetBalanceParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> = client.get_balance(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_bbo() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_bbo"];
    let request: GetBboParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesBBOResp, ClientError<Error>> = client.get_bbo(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_bbo_interactive() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_bbo_interactive"];
    let request: GetBboInteractiveParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesBBOResp, ClientError<Error>> =
        client.get_bbo_interactive(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_block_trades_list() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["block_trades_list"];
    let request: BlockTradesListParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.block_trades_list(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_block_trades_create() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["block_trades_create"];
    let request: BlockTradesCreateParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesBlockTradeDetailFullResponse, ClientError<Error>> =
        client.block_trades_create(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_block_trades_get() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["block_trades_get"];
    let request: BlockTradesGetParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesBlockTradeDetailFullResponse, ClientError<Error>> =
        client.block_trades_get(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_block_trades_cancel() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["block_trades_cancel"];
    let request: BlockTradesCancelParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<std::collections::BTreeMap<String, String>, ClientError<Error>> =
        client.block_trades_cancel(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_block_trades_execute() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["block_trades_execute"];
    let request: BlockTradesExecuteParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesBlockTradeDetailFullResponse, ClientError<Error>> =
        client.block_trades_execute(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_block_offers_list() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["block_offers_list"];
    let request: BlockOffersListParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.block_offers_list(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_block_offers_create() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["block_offers_create"];
    let request: BlockOffersCreateParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesBlockTradeDetailFullResponse, ClientError<Error>> =
        client.block_offers_create(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_block_offers_get() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["block_offers_get"];
    let request: BlockOffersGetParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesBlockTradeDetailFullResponse, ClientError<Error>> =
        client.block_offers_get(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_block_offers_cancel() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["block_offers_cancel"];
    let request: BlockOffersCancelParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<std::collections::BTreeMap<String, String>, ClientError<Error>> =
        client.block_offers_cancel(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_block_offers_execute() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["block_offers_execute"];
    let request: BlockOffersExecuteParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesBlockTradeDetailFullResponse, ClientError<Error>> =
        client.block_offers_execute(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_create_deposit_address() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["create_deposit_address"];
    let request: CreateDepositAddressParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesDepositAddressResp, ClientError<Error>> =
        client.create_deposit_address(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_list_deposit_networks() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["list_deposit_networks"];
    let request: ListDepositNetworksParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesDepositNetworksResp, ClientError<Error>> =
        client.list_deposit_networks(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_deposit_address() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_deposit_address"];
    let request: GetDepositAddressParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesDepositAddressDetailsResp, ClientError<Error>> =
        client.get_deposit_address(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_dime_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_dime_config"];
    let request: GetDimeConfigParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesDimeConfigResponse, ClientError<Error>> =
        client.get_dime_config(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_fills() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_fills"];
    let request: GetFillsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> = client.get_fills(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_funding_data() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_funding_data"];
    let request: GetFundingDataParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_funding_data(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_funding() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_funding"];
    let request: GetFundingParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> = client.get_funding(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_insurance_fund() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_insurance_fund"];
    let request: GetInsuranceFundParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesInsuranceAccountResp, ClientError<Error>> =
        client.get_insurance_fund(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_jwks() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["jwks"];
    let request: JwksParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesJwksResponse, ClientError<Error>> = client.jwks(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_liquidations() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_liquidations"];
    let request: GetLiquidationsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_liquidations(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_markets"];
    let request: GetMarketsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> = client.get_markets(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_markets_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_markets_history"];
    let request: GetMarketsHistoryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesMarketHistoryResponse, ClientError<Error>> =
        client.get_markets_history(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_markets_klines() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["markets_klines"];
    let request: MarketsKlinesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.markets_klines(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_market_settlement_price() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_market_settlement_price"];
    let request: GetMarketSettlementPriceParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesMarketSettlementResp, ClientError<Error>> =
        client.get_market_settlement_price(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_markets_summary() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_markets_summary"];
    let request: GetMarketsSummaryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_markets_summary(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_onboarding() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_onboarding"];
    let request: GetOnboardingParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesGetOnboardingResp, ClientError<Error>> =
        client.get_onboarding(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_onboarding() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["onboarding"];
    let request: OnboardingParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> = client.onboarding(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_orderbook() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_orderbook"];
    let request: GetOrderbookParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesAskBidArray, ClientError<Error>> =
        client.get_orderbook(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_impact_price() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_impact_price"];
    let request: GetImpactPriceParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesImpactPriceResp, ClientError<Error>> =
        client.get_impact_price(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_orderbook_interactive() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_orderbook_interactive"];
    let request: GetOrderbookInteractiveParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesAskBidArray, ClientError<Error>> =
        client.get_orderbook_interactive(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_open_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_open_orders"];
    let request: GetOpenOrdersParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_open_orders(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_orders_new() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["orders_new"];
    let request: OrdersNewParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesOrderResp, ClientError<Error>> = client.orders_new(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_orders_cancel_all() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["orders_cancel_all"];
    let request: OrdersCancelAllParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.orders_cancel_all(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_orders"];
    let request: GetOrdersParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> = client.get_orders(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_orders_new_batch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["orders_new_batch"];
    let request: OrdersNewBatchParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesBatchResponse, ClientError<Error>> =
        client.orders_new_batch(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_orders_cancel_batch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["orders_cancel_batch"];
    let request: OrdersCancelBatchParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesCancelOrderBatchResponse, ClientError<Error>> =
        client.orders_cancel_batch(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_orders_get_by_client_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["orders_get_by_client_id"];
    let request: OrdersGetByClientIdParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesOrderResp, ClientError<Error>> =
        client.orders_get_by_client_id(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_orders_cancel_by_client_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["orders_cancel_by_client_id"];
    let request: OrdersCancelByClientIdParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.orders_cancel_by_client_id(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_orders_get() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["orders_get"];
    let request: OrdersGetParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesOrderResp, ClientError<Error>> = client.orders_get(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_orders_modify() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["orders_modify"];
    let request: OrdersModifyParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesOrderResp, ClientError<Error>> =
        client.orders_modify(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Put);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_orders_cancel() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["orders_cancel"];
    let request: OrdersCancelParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.orders_cancel(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_positions"];
    let request: GetPositionsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_positions(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_referral_qr_code() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_referral_qr_code"];
    let request: GetReferralQrCodeParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<String, ClientError<Error>> = client.get_referral_qr_code(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_referrals_summary() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_referrals_summary"];
    let request: GetReferralsSummaryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_referrals_summary(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_rfq_list() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["rfq_list"];
    let request: RfqListParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesRfqResults, ClientError<Error>> = client.rfq_list(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_rfq_create() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["rfq_create"];
    let request: RfqCreateParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesRfqAcceptedResponse, ClientError<Error>> =
        client.rfq_create(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_rfq_draft_list() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["rfq_draft_list"];
    let request: RfqDraftListParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.rfq_draft_list(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_rfq_draft_create() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["rfq_draft_create"];
    let request: RfqDraftCreateParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesRfqDraftResult, ClientError<Error>> =
        client.rfq_draft_create(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_rfq_draft_delete() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["rfq_draft_delete"];
    let request: RfqDraftDeleteParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.rfq_draft_delete(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_rfq_markets() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["rfq_markets"];
    let request: RfqMarketsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesRfqMarketsResult, ClientError<Error>> =
        client.rfq_markets(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_rfq_platform_state() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["rfq_platform_state"];
    let request: RfqPlatformStateParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesRfqPlatformStateResult, ClientError<Error>> =
        client.rfq_platform_state(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_rfq_delete() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["rfq_delete"];
    let request: RfqDeleteParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesRfqAcceptedResponse, ClientError<Error>> =
        client.rfq_delete(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Delete);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_rfq_bbo() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["rfq_bbo"];
    let request: RfqBboParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesRfqBboResult, ClientError<Error>> = client.rfq_bbo(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_rfq_execute() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["rfq_execute"];
    let request: RfqExecuteParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesRfqAcceptedResponse, ClientError<Error>> =
        client.rfq_execute(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_staking_balance_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_staking_balance_history"];
    let request: GetStakingBalanceHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesAccountHistoricalDataResp, ClientError<Error>> =
        client.get_staking_balance_history(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_global_staking_balance_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_global_staking_balance_history"];
    let request: GetGlobalStakingBalanceHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesAccountHistoricalDataResp, ClientError<Error>> =
        client.get_global_staking_balance_history(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_staking_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_staking_config"];
    let request: GetStakingConfigParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesStakingConfigResponse, ClientError<Error>> =
        client.get_staking_config(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_staking_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_staking_history"];
    let request: GetStakingHistoryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_staking_history(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_staking_summary() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_staking_summary"];
    let request: GetStakingSummaryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesStakingSummaryResponse, ClientError<Error>> =
        client.get_staking_summary(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_announcements() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_announcements"];
    let request: GetAnnouncementsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_announcements(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_system_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_system_config"];
    let request: GetSystemConfigParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesSystemConfigResponse, ClientError<Error>> =
        client.get_system_config(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_margin_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_margin_config"];
    let request: GetMarginConfigParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_margin_config(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_portfolio_margin_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_portfolio_margin_config"];
    let request: GetPortfolioMarginConfigParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_portfolio_margin_config(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_system_state() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_system_state"];
    let request: GetSystemStateParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesSystemStateResponse, ClientError<Error>> =
        client.get_system_state(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_system_stats() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_system_stats"];
    let request: GetSystemStatsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesSystemStatsResponse, ClientError<Error>> =
        client.get_system_stats(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_system_time() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_system_time"];
    let request: GetSystemTimeParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesSystemTimeResponse, ClientError<Error>> =
        client.get_system_time(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_volume_tiers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_volume_tiers"];
    let request: GetVolumeTiersParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_volume_tiers(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_tradebusts() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["tradebusts"];
    let request: TradebustsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> = client.tradebusts(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["trades"];
    let request: TradesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> = client.trades(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_transactions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["transactions"];
    let request: TransactionsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.transactions(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_transfers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_transfers"];
    let request: GetTransfersParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_transfers(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_evm_auth() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["evm_auth"];
    let request: EvmAuthParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesAuthResp, ClientError<Error>> = client.evm_auth(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_evm_onboarding() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["evm_onboarding"];
    let request: EvmOnboardingParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.evm_onboarding(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_vaults_get() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["vaults_get"];
    let request: VaultsGetParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> = client.vaults_get(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_vaults_create() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["vaults_create"];
    let request: VaultsCreateParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesVaultResp, ClientError<Error>> =
        client.vaults_create(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_vaults_get_account_summary() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["vaults_get_account_summary"];
    let request: VaultsGetAccountSummaryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.vaults_get_account_summary(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_vaults_get_analytics() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["vaults_get_analytics"];
    let request: VaultsGetAnalyticsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesVaultAnalyticsResp, ClientError<Error>> =
        client.vaults_get_analytics(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_vaults_get_balance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["vaults_get_balance"];
    let request: VaultsGetBalanceParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.vaults_get_balance(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_vaults_get_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["vaults_get_config"];
    let request: VaultsGetConfigParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesVaultsConfigResponse, ClientError<Error>> =
        client.vaults_get_config(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_vaults_get_historical_data() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["vaults_get_historical_data"];
    let request: VaultsGetHistoricalDataParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.vaults_get_historical_data(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_vaults_get_mine() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["vaults_get_mine"];
    let request: VaultsGetMineParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.vaults_get_mine(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_vaults_get_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["vaults_get_positions"];
    let request: VaultsGetPositionsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.vaults_get_positions(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_vaults_get_summary() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["vaults_get_summary"];
    let request: VaultsGetSummaryParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.vaults_get_summary(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_vaults_get_transfers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["vaults_get_transfers"];
    let request: VaultsGetTransfersParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.vaults_get_transfers(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_xp_balance_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_xp_balance_v2"];
    let request: GetXpBalanceV2Params = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesGetXPBalanceResponseV2, ClientError<Error>> =
        client.get_xp_balance_v2(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_transfer_fee_config_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_transfer_fee_config_v2"];
    let request: GetTransferFeeConfigV2Params =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ServicesTransferFeeConfigResponse, ClientError<Error>> =
        client.get_transfer_fee_config_v2(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_public_xp_transfers_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_public_xp_transfers_v2"];
    let request: GetPublicXpTransfersV2Params =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_public_xp_transfers_v2(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_create_xp_transfer_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["create_xp_transfer_v2"];
    let request: CreateXpTransferV2Params = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<ResponsesCreateTransferResponse, ClientError<Error>> =
        client.create_xp_transfer_v2(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_xp_transfer_by_id_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_xp_transfer_by_id_v2"];
    let request: GetXpTransferByIdV2Params = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesXPTransfer, ClientError<Error>> =
        client.get_xp_transfer_by_id_v2(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_xp_transfers_v2() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_xp_transfers_v2"];
    let request: GetXpTransfersV2Params = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.get_xp_transfers_v2(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_subkey_jwt() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_subkey_jwt"];
    let request: GetSubkeyJwtParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport);
    let response: Result<ResponsesAuthResp, ClientError<Error>> =
        client.get_subkey_jwt(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_get_referrals_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["get_referrals_config"];
    let request: GetReferralsConfigParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Map<String, serde_json::Value>, ClientError<Error>> =
        client.get_referrals_config(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Get);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
#[tokio::test]
async fn contract_update_account_max_slippage() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["update_account_max_slippage"];
    let request: UpdateAccountMaxSlippageParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&f["response"].clone()).unwrap(),
        sent: sent.clone(),
    };
    let client = ParadexClient::with_transport(Paradex::testnet(), transport).with_auth(Auth {
        token: TokenState::default(),
    });
    let response: Result<serde_json::Map<String, serde_json::Value>, ClientError<Error>> =
        client.update_account_max_slippage(request).await;
    let value = serde_json::to_value(response.unwrap()).unwrap();
    if f["response"].get("future_field").is_some() {
        assert_eq!(value["future_field"], f["response"]["future_field"]);
    }
    let requests = sent.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let wire = &requests[0];
    assert!(
        wire.url.ends_with(f["route"].as_str().unwrap()),
        "{}",
        wire.url
    );
    assert_eq!(wire.method, aux_rs_core::HttpMethod::Post);
    let body = if wire.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&wire.body).unwrap()
    };
    assert_eq!(body, f["body"]);
    for (key, value) in f["headers"].as_object().unwrap() {
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == key && v == value.as_str().unwrap())
        );
    }
}
