// Generated from the pinned specification. Fixtures are synthetic contract examples.
use aux_rs_core::{Error as ClientError, HttpRequest, HttpResponse, HttpTransport};
use aux_rs_derive::*;
use serde_json::{Value, json};
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
struct Auth;
impl Authentication for Auth {
    fn wallet(&self) -> &str {
        "0x0000000000000000000000000000000000000001"
    }
    fn signer_address(&self) -> [u8; 20] {
        [1; 20]
    }
    fn nonce(&self) -> Result<String, Error> {
        Ok("1".into())
    }
    async fn login(&self, _: &Derive) -> Result<Value, Error> {
        Ok(json!({"wallet":self.wallet(),"timestamp":"1","signature":"fixture"}))
    }
    async fn sign_action(&self, _: &Derive, _: &ActionPayload) -> Result<String, Error> {
        Ok("fixture".into())
    }
}
#[tokio::test]
async fn contract_private_change_subaccount_label() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_change_subaccount_label"];
    let request: ChangeSubaccountLabelRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PrivateChangeSubaccountLabelResponse, ClientError<Error>> =
        client.private_change_subaccount_label(request).await;
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
async fn contract_private_delete_subaccount() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_delete_subaccount"];
    let request: DeleteSubaccountRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<DeleteSubaccountResponse, ClientError<Error>> =
        client.private_delete_subaccount(request).await;
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
async fn contract_private_get_all_portfolios() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_all_portfolios"];
    let request: GetAllPortfoliosRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<Vec<Subaccount>, ClientError<Error>> =
        client.private_get_all_portfolios(request).await;
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
async fn contract_private_get_collaterals() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_collaterals"];
    let request: GetCollateralsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PrivateGetCollateralsResponse, ClientError<Error>> =
        client.private_get_collaterals(request).await;
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
async fn contract_private_get_margin() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_margin"];
    let request: PrivateGetMarginRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<GetMarginResponse, ClientError<Error>> =
        client.private_get_margin(request).await;
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
async fn contract_private_get_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_positions"];
    let request: GetPositionsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PrivateGetPositionsResponse, ClientError<Error>> =
        client.private_get_positions(request).await;
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
async fn contract_private_get_subaccount() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_subaccount"];
    let request: GetSubaccountRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<Subaccount, ClientError<Error>> =
        client.private_get_subaccount(request).await;
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
async fn contract_private_get_subaccounts() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_subaccounts"];
    let request: GetSubaccountsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PrivateGetSubaccountsResponse, ClientError<Error>> =
        client.private_get_subaccounts(request).await;
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
async fn contract_public_get_margin() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_margin"];
    let request: PublicGetMarginRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<GetMarginResponse, ClientError<Error>> =
        client.public_get_margin(request).await;
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
async fn contract_public_get_subaccounts() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_subaccounts"];
    let request: PublicGetSubaccountsRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<PrivateGetSubaccountsResponse, ClientError<Error>> =
        client.public_get_subaccounts(request).await;
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
async fn contract_public_get_wallet() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_wallet"];
    let request: PublicGetWalletRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<PublicGetWalletResponse, ClientError<Error>> =
        client.public_get_wallet(request).await;
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
async fn contract_private_edit_session_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_edit_session_key"];
    let request: EditSessionKeyRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<SessionKey, ClientError<Error>> =
        client.private_edit_session_key(request).await;
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
async fn contract_private_session_keys() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_session_keys"];
    let request: SessionKeysRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PrivateSessionKeysResponse, ClientError<Error>> =
        client.private_session_keys(request).await;
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
async fn contract_private_set_session_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_set_session_key"];
    let request: SetSessionKeyRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PrivateSetSessionKeyResponse, ClientError<Error>> =
        client.private_set_session_key(request).await;
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
async fn contract_private_set_session_key_debug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_set_session_key_debug"];
    let request: SetSessionKeyRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<SetSessionKeyDebugResponse, ClientError<Error>> =
        client.private_set_session_key_debug(request).await;
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
async fn contract_public_get_wallets_from_session_key() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_wallets_from_session_key"];
    let request: GetWalletsFromSessionKeyRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<PublicGetWalletsFromSessionKeyResponse, ClientError<Error>> =
        client.public_get_wallets_from_session_key(request).await;
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
async fn contract_private_get_account() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_account"];
    let request: GetAccountRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PrivateGetAccountResponse, ClientError<Error>> =
        client.private_get_account(request).await;
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
async fn contract_private_cancel() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel"];
    let request: CancelOrderRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<Order, ClientError<Error>> = client.private_cancel(request).await;
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
async fn contract_private_cancel_algo_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_algo_order"];
    let request: CancelAlgoOrderRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<Order, ClientError<Error>> =
        client.private_cancel_algo_order(request).await;
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
async fn contract_private_cancel_all() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_all"];
    let request: CancelAllRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<CancelAllResponse, ClientError<Error>> =
        client.private_cancel_all(request).await;
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
async fn contract_private_cancel_all_algo_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_all_algo_orders"];
    let request: CancelAllAlgoOrdersRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<CancelAllAlgoOrdersResponse, ClientError<Error>> =
        client.private_cancel_all_algo_orders(request).await;
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
async fn contract_private_cancel_all_trigger_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_all_trigger_orders"];
    let request: CancelAllTriggerOrdersRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<CancelAllTriggerOrdersResponse, ClientError<Error>> =
        client.private_cancel_all_trigger_orders(request).await;
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
async fn contract_private_cancel_by_instrument() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_by_instrument"];
    let request: CancelByInstrumentRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<CancelByInstrumentResponse, ClientError<Error>> =
        client.private_cancel_by_instrument(request).await;
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
async fn contract_private_cancel_by_label() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_by_label"];
    let request: CancelByLabelRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<CancelByLabelResponse, ClientError<Error>> =
        client.private_cancel_by_label(request).await;
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
async fn contract_private_cancel_by_nonce() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_by_nonce"];
    let request: CancelByNonceRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<CancelByNonceResponse, ClientError<Error>> =
        client.private_cancel_by_nonce(request).await;
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
async fn contract_private_cancel_trigger_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_trigger_order"];
    let request: CancelTriggerOrderRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<Order, ClientError<Error>> =
        client.private_cancel_trigger_order(request).await;
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
async fn contract_private_get_algo_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_algo_orders"];
    let request: GetAlgoOrdersRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<Vec<Order>, ClientError<Error>> =
        client.private_get_algo_orders(request).await;
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
async fn contract_private_get_open_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_open_orders"];
    let request: GetOpenOrdersRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<AggregatedOrdersResult, ClientError<Error>> =
        client.private_get_open_orders(request).await;
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
async fn contract_private_get_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_order"];
    let request: GetOrderRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<Order, ClientError<Error>> = client.private_get_order(request).await;
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
async fn contract_private_get_trigger_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_trigger_orders"];
    let request: GetTriggerOrdersRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<AggregatedTriggerOrdersResult, ClientError<Error>> =
        client.private_get_trigger_orders(request).await;
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
async fn contract_private_order() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_order"];
    let request: CreateOrderRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<OrderCreatedResponse, ClientError<Error>> =
        client.private_order(request).await;
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
async fn contract_private_order_debug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_order_debug"];
    let request: CreateOrderRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<OrderDebugResponse, ClientError<Error>> =
        client.private_order_debug(request).await;
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
async fn contract_private_order_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_order_quote"];
    let request: OrderQuoteRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<OrderQuoteResponse, ClientError<Error>> =
        client.private_order_quote(request).await;
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
async fn contract_private_replace() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_replace"];
    let request: ReplaceOrderRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<ReplaceOrderResponse, ClientError<Error>> =
        client.private_replace(request).await;
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
async fn contract_public_order_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_order_quote"];
    let request: OrderQuoteRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<OrderQuoteResponse, ClientError<Error>> =
        client.public_order_quote(request).await;
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
async fn contract_private_cancel_batch_quotes() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_batch_quotes"];
    let request: CancelBatchQuotesRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<CancelBatchResult, ClientError<Error>> =
        client.private_cancel_batch_quotes(request).await;
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
async fn contract_private_cancel_batch_rfqs() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_batch_rfqs"];
    let request: CancelBatchRfqsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<CancelBatchRfqsResponse, ClientError<Error>> =
        client.private_cancel_batch_rfqs(request).await;
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
async fn contract_private_cancel_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_quote"];
    let request: CancelQuoteRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<Quote, ClientError<Error>> = client.private_cancel_quote(request).await;
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
async fn contract_private_cancel_rfq() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_rfq"];
    let request: CancelRfqRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<CancelRfqResponse, ClientError<Error>> =
        client.private_cancel_rfq(request).await;
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
async fn contract_private_execute_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_execute_quote"];
    let request: ExecuteQuoteRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<QuoteExecuteResponse, ClientError<Error>> =
        client.private_execute_quote(request).await;
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
async fn contract_private_get_maker_rfq_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_maker_rfq_history"];
    let request: GetMakerRfqHistoryRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<RFQPollResponse, ClientError<Error>> =
        client.private_get_maker_rfq_history(request).await;
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
async fn contract_private_get_quotes() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_quotes"];
    let request: GetQuotesRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<QuoteGetResponse, ClientError<Error>> =
        client.private_get_quotes(request).await;
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
async fn contract_private_get_rfqs() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_rfqs"];
    let request: GetRfqsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<RFQGetResponse, ClientError<Error>> =
        client.private_get_rfqs(request).await;
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
async fn contract_private_poll_quotes() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_poll_quotes"];
    let request: PollQuotesRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<QuotePollResponse, ClientError<Error>> =
        client.private_poll_quotes(request).await;
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
async fn contract_private_poll_rfqs() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_poll_rfqs"];
    let request: PollRfqsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<RFQPollResponse, ClientError<Error>> =
        client.private_poll_rfqs(request).await;
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
async fn contract_private_replace_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_replace_quote"];
    let request: ReplaceQuoteRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<QuoteReplaceResponse, ClientError<Error>> =
        client.private_replace_quote(request).await;
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
async fn contract_private_rfq_get_best_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_rfq_get_best_quote"];
    let request: RfqGetBestQuoteRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<RfqGetBestQuoteResponse, ClientError<Error>> =
        client.private_rfq_get_best_quote(request).await;
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
async fn contract_private_send_quote() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_send_quote"];
    let request: SendQuoteRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<Quote, ClientError<Error>> = client.private_send_quote(request).await;
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
async fn contract_private_send_rfq() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_send_rfq"];
    let request: SendRfqRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<Rfq, ClientError<Error>> = client.private_send_rfq(request).await;
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
async fn contract_public_execute_quote_debug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_execute_quote_debug"];
    let request: PublicExecuteQuoteDebugRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<QuoteExecuteDebugResult, ClientError<Error>> =
        client.public_execute_quote_debug(request).await;
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
async fn contract_public_send_quote_debug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_send_quote_debug"];
    let request: PublicSendQuoteDebugRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<QuoteSendDebugResult, ClientError<Error>> =
        client.public_send_quote_debug(request).await;
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
async fn contract_private_liquidate() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_liquidate"];
    let request: PrivateLiquidateRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PrivateLiquidateResponse, ClientError<Error>> =
        client.private_liquidate(request).await;
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
async fn contract_public_get_liquidation_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_liquidation_history"];
    let request: GetLiquidationHistoryRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<LiquidationHistoryResult, ClientError<Error>> =
        client.public_get_liquidation_history(request).await;
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
async fn contract_public_get_live_auctions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_live_auctions"];
    let request: EmptyRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<PublicGetLiveAuctionsResponse, ClientError<Error>> =
        client.public_get_live_auctions(request).await;
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
async fn contract_public_margin_watch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_margin_watch"];
    let request: MarginWatchRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<PublicMarginWatchResponse, ClientError<Error>> =
        client.public_margin_watch(request).await;
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
async fn contract_private_cancel_all_vault_requests() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_cancel_all_vault_requests"];
    let request: CancelVaultRequestRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<VaultCancelResponse, ClientError<Error>> =
        client.private_cancel_all_vault_requests(request).await;
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
async fn contract_private_get_live_vault_requests() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_live_vault_requests"];
    let request: GetLiveVaultRequestsRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<MultipleVaultRequestsResponse, ClientError<Error>> =
        client.private_get_live_vault_requests(request).await;
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
async fn contract_private_get_shareholder_vaults() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_shareholder_vaults"];
    let request: GetShareholderVaultsRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<VaultIdsResponse, ClientError<Error>> =
        client.private_get_shareholder_vaults(request).await;
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
async fn contract_private_get_vault_request_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_vault_request_history"];
    let request: GetVaultRequestHistoryRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PaginatedVaultRequestHistory, ClientError<Error>> =
        client.private_get_vault_request_history(request).await;
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
async fn contract_private_get_vault_shares() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_vault_shares"];
    let request: GetVaultSharesRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<VaultSharesResponse, ClientError<Error>> =
        client.private_get_vault_shares(request).await;
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
async fn contract_private_request_vault_deposit() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_request_vault_deposit"];
    let request: RequestVaultDepositRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<VaultRequestAckResponse, ClientError<Error>> =
        client.private_request_vault_deposit(request).await;
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
async fn contract_private_request_vault_withdraw() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_request_vault_withdraw"];
    let request: RequestVaultWithdrawRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<VaultRequestAckResponse, ClientError<Error>> =
        client.private_request_vault_withdraw(request).await;
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
async fn contract_public_get_vault() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_vault"];
    let request: GetVaultRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<Vault, ClientError<Error>> = client.public_get_vault(request).await;
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
async fn contract_public_get_vault_action_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_vault_action_history"];
    let request: GetVaultActionHistoryRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<PaginatedVaultActionHistory, ClientError<Error>> =
        client.public_get_vault_action_history(request).await;
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
async fn contract_public_get_vault_performance_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_vault_performance_history"];
    let request: GetVaultPerformanceHistoryRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<VaultPerformanceHistoryResult, ClientError<Error>> =
        client.public_get_vault_performance_history(request).await;
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
async fn contract_public_get_vaults() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_vaults"];
    let request: GetVaultsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<VaultsResponse, ClientError<Error>> =
        client.public_get_vaults(request).await;
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
async fn contract_private_burn_vault_shares() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_burn_vault_shares"];
    let request: BurnSharesRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<VaultSettleResponse, ClientError<Error>> =
        client.private_burn_vault_shares(request).await;
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
async fn contract_private_create_vault() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_create_vault"];
    let request: CreateVaultRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<VaultCreateResponse, ClientError<Error>> =
        client.private_create_vault(request).await;
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
async fn contract_private_force_burn() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_force_burn"];
    let request: ForceBurnRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<VaultForceBurnResponse, ClientError<Error>> =
        client.private_force_burn(request).await;
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
async fn contract_private_get_curated_vaults() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_curated_vaults"];
    let request: GetCuratedVaultsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<VaultIdsResponse, ClientError<Error>> =
        client.private_get_curated_vaults(request).await;
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
async fn contract_private_get_live_burn_requests() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_live_burn_requests"];
    let request: GetLiveBurnRequestsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<MultipleVaultRequestsResponse, ClientError<Error>> =
        client.private_get_live_burn_requests(request).await;
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
async fn contract_private_get_live_mint_requests() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_live_mint_requests"];
    let request: GetLiveMintRequestsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<MultipleVaultRequestsResponse, ClientError<Error>> =
        client.private_get_live_mint_requests(request).await;
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
async fn contract_private_mint_vault_shares() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_mint_vault_shares"];
    let request: MintSharesRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<VaultSettleResponse, ClientError<Error>> =
        client.private_mint_vault_shares(request).await;
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
async fn contract_private_reject_deposit_request() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_reject_deposit_request"];
    let request: RejectDepositRequestRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<VaultRequestAckResponse, ClientError<Error>> =
        client.private_reject_deposit_request(request).await;
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
async fn contract_private_update_vault_info() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_update_vault_info"];
    let request: UpdateVaultInfoRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<OffchainAckResponse, ClientError<Error>> =
        client.private_update_vault_info(request).await;
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
async fn contract_private_get_deposit_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_deposit_history"];
    let request: GetDepositHistoryRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<DepositHistoryResult, ClientError<Error>> =
        client.private_get_deposit_history(request).await;
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
async fn contract_private_get_erc20_transfer_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_erc20_transfer_history"];
    let request: GetErc20TransferHistoryRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<TransferHistoryResult, ClientError<Error>> =
        client.private_get_erc20_transfer_history(request).await;
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
async fn contract_private_get_funding_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_funding_history"];
    let request: GetFundingHistoryRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PerpSettlementHistoryResponse, ClientError<Error>> =
        client.private_get_funding_history(request).await;
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
async fn contract_private_get_interest_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_interest_history"];
    let request: GetInterestHistoryRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<InterestHistoryResult, ClientError<Error>> =
        client.private_get_interest_history(request).await;
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
async fn contract_private_get_option_settlement_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_option_settlement_history"];
    let request: GetOptionSettlementHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<OptionSettlementHistoryResponse, ClientError<Error>> =
        client.private_get_option_settlement_history(request).await;
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
async fn contract_private_get_order_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_order_history"];
    let request: GetOrderHistoryRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PaginatedOrdersResult, ClientError<Error>> =
        client.private_get_order_history(request).await;
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
async fn contract_private_get_trade_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_trade_history"];
    let request: GetTradeHistoryRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PaginatedTradesResult, ClientError<Error>> =
        client.private_get_trade_history(request).await;
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
async fn contract_private_get_withdrawal_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_withdrawal_history"];
    let request: GetWithdrawalHistoryRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<WithdrawalHistoryResult, ClientError<Error>> =
        client.private_get_withdrawal_history(request).await;
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
async fn contract_private_get_mmp_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_mmp_config"];
    let request: MmpScopeRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<Vec<MmpConfigResult>, ClientError<Error>> =
        client.private_get_mmp_config(request).await;
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
async fn contract_private_reset_mmp() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_reset_mmp"];
    let request: MmpScopeRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<ResetMmpResponse, ClientError<Error>> =
        client.private_reset_mmp(request).await;
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
async fn contract_private_set_mmp_config() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_set_mmp_config"];
    let request: SetMmpConfigRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<SetMmpConfigResponse, ClientError<Error>> =
        client.private_set_mmp_config(request).await;
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
async fn contract_private_transfer_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_transfer_positions"];
    let request: TransferPositionsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<TransferPositionsResponse, ClientError<Error>> =
        client.private_transfer_positions(request).await;
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
async fn contract_private_transfer_positions_debug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_transfer_positions_debug"];
    let request: TransferPositionsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<TransferPositionsDebugResponse, ClientError<Error>> =
        client.private_transfer_positions_debug(request).await;
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
async fn contract_private_transfer_spot() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_transfer_spot"];
    let request: PrivateTransferSpotRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PrivateTransferSpotResponse, ClientError<Error>> =
        client.private_transfer_spot(request).await;
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
async fn contract_private_transfer_spot_debug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_transfer_spot_debug"];
    let request: PrivateTransferSpotRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<TransferSpotDebugResponse, ClientError<Error>> =
        client.private_transfer_spot_debug(request).await;
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
async fn contract_private_transfer_spot_external() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_transfer_spot_external"];
    let request: PrivateTransferSpotExternalRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PrivateTransferSpotExternalResponse, ClientError<Error>> =
        client.private_transfer_spot_external(request).await;
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
async fn contract_private_transfer_spot_external_debug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_transfer_spot_external_debug"];
    let request: PrivateTransferSpotExternalRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<TransferSpotExternalDebugResponse, ClientError<Error>> =
        client.private_transfer_spot_external_debug(request).await;
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
async fn contract_private_update_whitelisted_recipients() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_update_whitelisted_recipients"];
    let request: UpdateWhitelistedRecipientsRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<UpdateWhitelistedRecipientsResponse, ClientError<Error>> =
        client.private_update_whitelisted_recipients(request).await;
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
async fn contract_private_update_whitelisted_recipients_debug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_update_whitelisted_recipients_debug"];
    let request: UpdateWhitelistedRecipientsRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<UpdateWhitelistedRecipientsDebugResponse, ClientError<Error>> = client
        .private_update_whitelisted_recipients_debug(request)
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
async fn contract_private_withdraw() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_withdraw"];
    let request: PrivateWithdrawRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PrivateWithdrawResponse, ClientError<Error>> =
        client.private_withdraw(request).await;
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
async fn contract_public_withdraw_debug() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_withdraw_debug"];
    let request: PublicWithdrawDebugRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<serde_json::Value, ClientError<Error>> =
        client.public_withdraw_debug(request).await;
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
async fn contract_private_get_withdraw_addresses() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_withdraw_addresses"];
    let request: GetWithdrawAddressesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<GetWithdrawAddressesResult, ClientError<Error>> =
        client.private_get_withdraw_addresses(request).await;
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
async fn contract_private_register_bridge_deposit() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_register_bridge_deposit"];
    let request: RegisterBridgeDepositParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<PendingDepositBridgeOrigin, ClientError<Error>> =
        client.private_register_bridge_deposit(request).await;
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
async fn contract_private_register_withdraw_address() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_register_withdraw_address"];
    let request: RegisterWithdrawAddressParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<RegisterWithdrawAddressResult, ClientError<Error>> =
        client.private_register_withdraw_address(request).await;
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
async fn contract_public_get_onchain_action_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_onchain_action_history"];
    let request: GetOnchainActionHistoryParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<GetOnchainActionHistoryResponse, ClientError<Error>> =
        client.public_get_onchain_action_history(request).await;
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
async fn contract_public_get_pending_deposits() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_pending_deposits"];
    let request: GetPendingDepositsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<GetPendingDepositsResult, ClientError<Error>> =
        client.public_get_pending_deposits(request).await;
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
async fn contract_public_register_deposit_address() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_register_deposit_address"];
    let request: RegisterDepositAddressParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<RegisterDepositAddressResult, ClientError<Error>> =
        client.public_register_deposit_address(request).await;
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
async fn contract_public_decode_action() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_decode_action"];
    let request: DecodeActionRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<DecodeActionResponse, ClientError<Error>> =
        client.public_decode_action(request).await;
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
async fn contract_public_get_rate_limits() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_rate_limits"];
    let request: EmptyRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<RateLimitResult, ClientError<Error>> =
        client.public_get_rate_limits(request).await;
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
async fn contract_public_get_live_incidents() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_live_incidents"];
    let request: GetLiveIncidentsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<GetLiveIncidentsResponse, ClientError<Error>> =
        client.public_get_live_incidents(request).await;
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
async fn contract_public_get_time() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_time"];
    let request: EmptyRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<serde_json::Number, ClientError<Error>> =
        client.public_get_time(request).await;
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
async fn contract_public_get_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_transaction"];
    let request: GetTransactionParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<GetTransactionResult, ClientError<Error>> =
        client.public_get_transaction(request).await;
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
async fn contract_public_get_all_currencies() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_all_currencies"];
    let request: EmptyRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<Vec<Currency>, ClientError<Error>> =
        client.public_get_all_currencies(request).await;
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
async fn contract_public_get_all_instruments() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_all_instruments"];
    let request: GetAllInstrumentsRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<GetAllInstrumentsResponse, ClientError<Error>> =
        client.public_get_all_instruments(request).await;
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
async fn contract_public_get_all_live_instruments() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_all_live_instruments"];
    let request: EmptyRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<Vec<String>, ClientError<Error>> =
        client.public_get_all_live_instruments(request).await;
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
async fn contract_public_get_currency() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_currency"];
    let request: GetCurrencyRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<Currency, ClientError<Error>> = client.public_get_currency(request).await;
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
async fn contract_public_get_funding_rate_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_funding_rate_history"];
    let request: GetFundingRateHistoryRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<FundingRateHistoryResult, ClientError<Error>> =
        client.public_get_funding_rate_history(request).await;
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
async fn contract_public_get_index_chart_data() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_index_chart_data"];
    let request: GetIndexChartDataRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<Vec<IndexCandle>, ClientError<Error>> =
        client.public_get_index_chart_data(request).await;
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
async fn contract_public_get_instrument() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_instrument"];
    let request: GetInstrumentRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<Instrument, ClientError<Error>> =
        client.public_get_instrument(request).await;
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
async fn contract_public_get_interest_rate_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_interest_rate_history"];
    let request: GetInterestRateHistoryRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<InterestRateHistoryResult, ClientError<Error>> =
        client.public_get_interest_rate_history(request).await;
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
async fn contract_public_get_latest_signed_feeds() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_latest_signed_feeds"];
    let request: GetLatestSignedFeedsRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<GetLatestSignedFeedsResponse, ClientError<Error>> =
        client.public_get_latest_signed_feeds(request).await;
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
async fn contract_public_get_option_settlement_prices() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_option_settlement_prices"];
    let request: GetOptionSettlementPricesRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<OptionSettlementPricesResult, ClientError<Error>> =
        client.public_get_option_settlement_prices(request).await;
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
async fn contract_public_get_perp_impact_twap() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_perp_impact_twap"];
    let request: GetPerpImpactTwapRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<PerpImpactTwapResult, ClientError<Error>> =
        client.public_get_perp_impact_twap(request).await;
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
async fn contract_public_get_risk_universes() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_risk_universes"];
    let request: EmptyRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<Vec<RiskUniverse>, ClientError<Error>> =
        client.public_get_risk_universes(request).await;
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
async fn contract_public_get_ticker() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_ticker"];
    let request: GetTickerRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<TickerSlimSnapshot, ClientError<Error>> =
        client.public_get_ticker(request).await;
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
async fn contract_public_get_tickers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_tickers"];
    let request: GetTickersRequest = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<GetTickersResponse, ClientError<Error>> =
        client.public_get_tickers(request).await;
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
async fn contract_public_get_trade_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_trade_history"];
    let request: GetPublicTradeHistoryRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<PublicTradesResult, ClientError<Error>> =
        client.public_get_trade_history(request).await;
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
async fn contract_public_get_tradingview_chart_data() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_tradingview_chart_data"];
    let request: GetTradingviewChartDataRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<Vec<TradingviewCandle>, ClientError<Error>> =
        client.public_get_tradingview_chart_data(request).await;
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
async fn contract_public_get_maker_program_scores() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_maker_program_scores"];
    let request: GetMakerProgramScoresParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<GetMakerProgramScoresResponse, ClientError<Error>> =
        client.public_get_maker_program_scores(request).await;
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
async fn contract_public_get_maker_programs() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_maker_programs"];
    let request: GetMakerProgramsParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<Vec<ProgramResponse>, ClientError<Error>> =
        client.public_get_maker_programs(request).await;
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
async fn contract_public_get_all_referral_codes() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_all_referral_codes"];
    let request: GetAllReferralCodesParams = serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<Vec<Referrer>, ClientError<Error>> =
        client.public_get_all_referral_codes(request).await;
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
async fn contract_public_get_referral_performance() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["public_get_referral_performance"];
    let request: GetReferralPerformanceParams =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport);
    let response: Result<GetReferralPerformanceResult, ClientError<Error>> =
        client.public_get_referral_performance(request).await;
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
async fn contract_private_get_subaccount_value_history() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let f = &fixtures["private_get_subaccount_value_history"];
    let request: GetSubaccountValueHistoryRequest =
        serde_json::from_value(f["request"].clone()).unwrap();
    let sent = Arc::new(Mutex::new(Vec::new()));
    let transport = Mock {
        response: serde_json::to_vec(&json!({"result":f["response"]})).unwrap(),
        sent: sent.clone(),
    };
    let client = DeriveClient::with_transport(Derive::testnet(), transport).with_auth(Auth);
    let response: Result<SubaccountValueHistoryResult, ClientError<Error>> =
        client.private_get_subaccount_value_history(request).await;
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
