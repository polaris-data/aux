use aux_rs_core::{
    HttpRequest, HttpResponse, HttpTransport, RequestOutcome, TransportError, TransportErrorKind,
};
use aux_rs_phoenix::*;
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
#[derive(Clone)]
struct Mock {
    calls: Arc<Mutex<Vec<HttpRequest>>>,
    status: u16,
    body: Vec<u8>,
    uncertain: bool,
}
impl HttpTransport for Mock {
    async fn execute(&self, r: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.calls.lock().unwrap().push(r);
        if self.uncertain {
            return Err(TransportError::new(
                TransportErrorKind::Timeout,
                RequestOutcome::Unknown,
                "fixture timeout",
            ));
        }
        Ok(HttpResponse {
            status: self.status,
            headers: vec![],
            body: self.body.clone(),
        })
    }
}
fn transport(body: &str) -> Mock {
    Mock {
        calls: Arc::default(),
        status: 200,
        body: body.as_bytes().to_vec(),
        uncertain: false,
    }
}
#[tokio::test]
async fn native_errors_and_ambiguous_writes_are_not_replayed() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
    let private = serde_json::from_str::<Value>(include_str!("../coverage.json")).unwrap()["rest"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["verb"] == "post" && r["private"] == true)
        .unwrap()
        .clone();
    let method = private["method"].as_str().unwrap();
    let operations: Vec<Value> = serde_json::from_str::<Value>(include_str!("../coverage.json"))
        .unwrap()["rest"]
        .as_array()
        .unwrap()
        .clone();
    assert!(!operations.is_empty());
    let mut t = transport("{}");
    t.uncertain = true;
    let calls = t.calls.clone();
    let c = PhoenixClient::with_transport(Phoenix::mainnet(), t).with_auth(
        Credentials::new(Some(zeroize::Zeroizing::new("fixture".into())), None, None).unwrap(),
    );
    let op = Operation {
        method: aux_rs_core::HttpMethod::Post,
        path: "/fixture/write",
        authenticated: true,
        safe_read: false,
        parameters: &[Parameter {
            name: "body",
            location: Location::Body,
            required: true,
            collection: "csv",
        }],
    };
    let result: Result<Value, _> = c
        .execute(&op, json!({"body":fixtures[method]["request"]}))
        .await;
    assert!(
        matches!(result,Err(aux_rs_core::Error::Transport(e)) if e.outcome==RequestOutcome::Unknown)
    );
    assert_eq!(calls.lock().unwrap().len(), 1);
    let t = transport(r#"{"status":"ERROR","error":{"code":1122,"message":"native rejection"}}"#);
    let c = PhoenixClient::with_transport(Phoenix::mainnet(), t);
    let result: Result<Value, _> = c
        .execute(
            &Operation {
                method: aux_rs_core::HttpMethod::Get,
                path: "/fixture/read",
                authenticated: false,
                safe_read: true,
                parameters: &[],
            },
            json!({}),
        )
        .await;
    assert!(
        matches!(result,Err(aux_rs_core::Error::Exchange(Error::Api{body,..})) if body["error"]["code"]==1122)
    );
}
#[test]
fn endpoint_and_credentials_validation() {
    assert!(
        Phoenix::custom(
            "https://secret:password@example.com",
            "wss://example.com",
            "custom"
        )
        .is_err()
    );
    assert!(
        Credentials::new(
            None,
            Some(zeroize::Zeroizing::new("bad\r\nheader".into())),
            None
        )
        .is_err()
    );
    let expired = Credentials::new(
        Some(zeroize::Zeroizing::new("private-secret".into())),
        None,
        Some(0),
    )
    .unwrap();
    assert!(expired.headers(true).is_err());
    assert!(!format!("{expired:?}").contains("private-secret"));
    let c = Credentials::default();
    c.replace(c.clone());
    assert!(c.headers(true).is_err());
}
