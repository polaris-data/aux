use aux_rs_core::{
    HttpMethod, HttpRequest, StreamingHttpResponse, StreamingHttpTransport, TransportError,
};
use aux_rs_data::{BytesBody, HttpClient, Params, header};
use aux_rs_dune::*;
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

type ResponseQueue = Arc<Mutex<VecDeque<(u16, Vec<(String, String)>, Vec<u8>)>>>;
#[derive(Clone, Default, Debug)]
struct Mock {
    requests: Arc<Mutex<Vec<HttpRequest>>>,
    responses: ResponseQueue,
}
impl Mock {
    fn push(&self, status: u16, headers: Vec<(String, String)>, body: impl Into<Vec<u8>>) {
        self.responses
            .lock()
            .unwrap()
            .push_back((status, headers, body.into()));
    }
    fn client(&self) -> DuneClient<Self> {
        DuneClient::with_transport(
            "https://api.dune.test/api/v1",
            "fixture-secret",
            self.clone(),
        )
        .unwrap()
    }
}
impl StreamingHttpTransport for Mock {
    type Body = BytesBody;
    async fn execute_stream(
        &self,
        request: HttpRequest,
    ) -> std::result::Result<StreamingHttpResponse<Self::Body>, TransportError> {
        self.requests.lock().unwrap().push(request);
        let (status, headers, body) = self
            .responses
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected request or implicit retry");
        Ok(StreamingHttpResponse {
            status,
            headers,
            body: BytesBody(Some(body)),
        })
    }
}
fn value(v: impl Serialize) -> Value {
    serde_json::to_value(v).unwrap()
}
fn fixtures() -> Vec<Value> {
    serde_json::from_str(include_str!("fixtures/http.json")).unwrap()
}
#[tokio::test]
async fn native_http_inventory() {
    let fixtures = fixtures();
    assert_eq!(fixtures.len(), 9);
    for fixture in &fixtures {
        let mock = Mock::default();
        let headers = fixture["response_headers"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap().into()))
            .collect();
        mock.push(
            200,
            headers,
            fixture["response_body"].as_str().unwrap().as_bytes(),
        );
        let request = fixture["request_json"].as_str().unwrap_or("{}");
        let client = mock.client();
        let output = match fixture["operation"].as_str().unwrap() {
            "execute_sql" => value(
                client
                    .execute_sql(serde_json::from_str(request).unwrap())
                    .await
                    .unwrap(),
            ),
            "execute_query" => value(
                client
                    .execute_query(123, serde_json::from_str(request).unwrap())
                    .await
                    .unwrap(),
            ),
            "execute_query_pipeline" => value(
                client
                    .execute_query_pipeline(123, serde_json::from_str(request).unwrap())
                    .await
                    .unwrap(),
            ),
            "get_execution_status" => value(client.get_execution_status("exec-1").await.unwrap()),
            "get_execution_results" => value(
                client
                    .get_execution_results("exec-1", serde_json::from_str(request).unwrap())
                    .await
                    .unwrap(),
            ),
            "get_execution_results_csv" => value(
                client
                    .get_execution_results_csv("exec-1", serde_json::from_str(request).unwrap())
                    .await
                    .unwrap(),
            ),
            "get_latest_query_results" => value(
                client
                    .get_latest_query_results(123, serde_json::from_str(request).unwrap())
                    .await
                    .unwrap(),
            ),
            "get_latest_query_results_csv" => value(
                client
                    .get_latest_query_results_csv(123, serde_json::from_str(request).unwrap())
                    .await
                    .unwrap(),
            ),
            "cancel_execution" => value(client.cancel_execution("exec-1").await.unwrap()),
            name => panic!("uncovered operation {name}"),
        };
        let csv = fixture["operation"].as_str().unwrap().ends_with("_csv");
        if csv {
            assert_eq!(output["data"], fixture["response_body"]);
            assert_eq!(output["next_offset"], 1);
            assert_eq!(
                output["next_uri"],
                fixture["response_headers"]["X-Dune-Next-Uri"]
            );
        } else {
            assert_eq!(
                output,
                serde_json::from_str::<Value>(fixture["response_body"].as_str().unwrap()).unwrap()
            );
        }
        let requests = mock.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        let wire = &requests[0];
        assert_eq!(
            format!("{:?}", wire.method).to_uppercase(),
            fixture["verb"].as_str().unwrap()
        );
        let (path, query) = wire
            .url
            .trim_start_matches("https://api.dune.test")
            .split_once('?')
            .unwrap_or((wire.url.trim_start_matches("https://api.dune.test"), ""));
        assert_eq!(path, fixture["path"].as_str().unwrap());
        assert_eq!(
            header(&wire.headers, "x-dune-api-key"),
            Some("fixture-secret")
        );
        assert!(header(&wire.headers, "authorization").is_none());
        assert!(!wire.url.contains("fixture-secret"));
        if wire.method == HttpMethod::Post && !fixture["request_json"].is_null() {
            assert_eq!(
                header(&wire.headers, "content-type"),
                Some("application/json")
            );
            assert_eq!(
                serde_json::from_slice::<Value>(&wire.body).unwrap(),
                serde_json::from_str::<Value>(request).unwrap()
            );
        } else {
            assert!(wire.body.is_empty());
        }
        if fixture["query"].as_object().unwrap().is_empty() {
            assert!(query.is_empty());
        } else {
            for expected in [
                "allow_partial_results=true",
                "ignore_max_credits_per_request=true",
                "limit=1",
                "offset=0",
                "columns=id%2Camount",
                "filters=amount+%3E+0",
                "sort_by=id+desc",
            ] {
                assert!(query.contains(expected), "{query}: missing {expected}");
            }
        }
    }
}
#[tokio::test]
async fn states_defaults_sampling_and_dynamic_results() {
    let mock = Mock::default();
    for state in [
        "QUERY_STATE_PENDING",
        "QUERY_STATE_EXECUTING",
        "QUERY_STATE_COMPLETED",
        "QUERY_STATE_FAILED",
        "QUERY_STATE_CANCELLED",
        "QUERY_STATE_EXPIRED",
        "QUERY_STATE_COMPLETED_PARTIAL",
        "FUTURE_STATE",
    ] {
        mock.push(200, vec![], json!({"execution_id":"raw-sql", "state":state,"error":{"type":"syntax_error","message":"bad SQL","metadata":{"line":1}},"future":true}).to_string());
        let result = mock.client().get_execution_status("raw-sql").await.unwrap();
        assert_eq!(result.state, state);
        assert!(result.query_id.is_none());
        assert_eq!(result.error.unwrap().kind.as_deref(), Some("syntax_error"));
        assert_eq!(result.extra["future"], true);
    }
    mock.push(200,vec![],r#"{"execution_id":"raw-sql","state":"QUERY_STATE_COMPLETED","result":{"rows":[],"metadata":{"row_count":0}}}"#);
    let result = mock
        .client()
        .get_execution_results(
            "raw-sql",
            ResultOptions {
                sample_count: Some(10),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert!(result.result.unwrap().rows.unwrap().is_empty());
    assert!(
        mock.requests
            .lock()
            .unwrap()
            .last()
            .unwrap()
            .url
            .ends_with("?sample_count=10")
    );
    mock.push(
        200,
        vec![],
        r#"{"execution_id":"raw-sql","state":"QUERY_STATE_PENDING"}"#,
    );
    assert!(
        mock.client()
            .get_execution_results("raw-sql", ResultOptions::default())
            .await
            .unwrap()
            .result
            .is_none()
    );
    assert!(
        !mock
            .requests
            .lock()
            .unwrap()
            .last()
            .unwrap()
            .url
            .contains('?')
    );
    mock.push(
        200,
        vec![],
        r#"{"execution_id":"exec-1","state":"QUERY_STATE_PENDING"}"#,
    );
    mock.client()
        .execute_query(123, ExecuteQueryRequest::default())
        .await
        .unwrap();
    assert_eq!(mock.requests.lock().unwrap().last().unwrap().body, b"{}");
    mock.push(200, vec![], "id\n");
    let csv = mock
        .client()
        .get_latest_query_results_csv(123, ResultOptions::default())
        .await
        .unwrap();
    assert!(csv.next_offset.is_none() && csv.next_uri.is_none());
}
#[tokio::test]
async fn errors_redaction_and_no_replays() {
    for (status, kind) in [
        (400, "vendor"),
        (401, "authentication"),
        (402, "vendor"),
        (403, "entitlement"),
        (404, "vendor"),
        (429, "rate_limit"),
        (500, "vendor"),
    ] {
        let mock = Mock::default();
        mock.push(
            status,
            vec![],
            r#"{"error":"fixture-secret rejected","nested":["fixture-secret"]}"#,
        );
        let client = mock.client();
        let error = client
            .execute_sql(ExecuteSqlRequest {
                sql: "select 1".into(),
                performance: None,
            })
            .await
            .unwrap_err();
        assert_eq!(error.kind(), kind);
        assert!(!format!("{error:?}").contains("fixture-secret"));
        if let Error::Api {
            status: actual,
            body,
        } = error
        {
            assert_eq!(actual, status);
            assert!(!body.to_string().contains("fixture-secret"));
            assert!(body.to_string().contains("[redacted]"));
        } else {
            panic!("native error was lost");
        }
        assert_eq!(mock.requests.lock().unwrap().len(), 1);
        assert!(!format!("{client:?}").contains("fixture-secret"));
    }
    for body in ["not JSON", "{}", r#"{"execution_id":"x","state":false}"#] {
        let mock = Mock::default();
        mock.push(200, vec![], body);
        assert!(matches!(
            mock.client().get_execution_status("x").await,
            Err(Error::Decode(_))
        ));
    }
    let mock = Mock::default();
    mock.push(502, vec![], "fixture-secret plain text");
    let error = mock.client().get_execution_status("x").await.unwrap_err();
    if let Error::Api { body, .. } = error {
        assert_eq!(body, "[redacted] plain text");
    } else {
        panic!("missing native body");
    }
    for (headers, body) in [
        (
            vec![("X-Dune-Next-Offset".into(), "invalid".into())],
            vec![],
        ),
        (vec![], vec![0xff]),
        (vec![], vec![b'x'; aux_rs_data::MAX_RECORD_BYTES + 1]),
    ] {
        let mock = Mock::default();
        mock.push(200, headers, body);
        assert!(matches!(
            mock.client()
                .get_execution_results_csv("x", ResultOptions::default())
                .await,
            Err(Error::Decode(_))
        ));
    }
}
#[tokio::test]
async fn credential_redirect_policy() {
    for same_origin in [true, false] {
        let mock = Mock::default();
        let target = if same_origin {
            "https://api.dune.test/api/v1/moved"
        } else {
            "https://other.test/results"
        };
        mock.push(302, vec![("location".into(), target.into())], Vec::new());
        mock.push(
            200,
            vec![],
            r#"{"execution_id":"x","state":"QUERY_STATE_PENDING"}"#,
        );
        mock.client().get_execution_status("x").await.unwrap();
        let requests = mock.requests.lock().unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(
            header(&requests[1].headers, "X-Dune-API-Key").is_some(),
            same_origin
        );
    }
    // Existing Bearer clients retain their prior redirect behavior.
    let mock = Mock::default();
    mock.push(
        302,
        vec![("location".into(), "https://other.test/result".into())],
        Vec::new(),
    );
    mock.push(200, vec![], "{}");
    let http = HttpClient::new("https://vendor.test/", mock.clone())
        .unwrap()
        .with_auth("Bearer secret".into(), "secret".into());
    http.json(
        http.request(HttpMethod::Get, "result", &Params::new(), false)
            .unwrap(),
    )
    .await
    .unwrap();
    assert!(header(&mock.requests.lock().unwrap()[1].headers, "authorization").is_none());
    let mock = Mock::default();
    mock.push(
        302,
        vec![("location".into(), "http://api.dune.test/result".into())],
        Vec::new(),
    );
    assert!(matches!(
        mock.client().get_execution_status("x").await,
        Err(Error::Decode(_))
    ));
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
    let mock = Mock::default();
    mock.push(
        307,
        vec![("location".into(), "https://api.dune.test/new".into())],
        Vec::new(),
    );
    assert!(matches!(
        mock.client().cancel_execution("x").await,
        Err(Error::Decode(_))
    ));
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
}
#[tokio::test]
async fn invalid_requests_do_not_send() {
    let mock = Mock::default();
    let client = mock.client();
    assert!(
        client
            .execute_query(0, ExecuteQueryRequest::default())
            .await
            .is_err()
    );
    for id in ["", ".", "..", "with space"] {
        assert!(client.get_execution_status(id).await.is_err());
    }
    assert!(
        client
            .execute_sql(ExecuteSqlRequest {
                sql: "  ".into(),
                performance: None
            })
            .await
            .is_err()
    );
    for options in [
        ResultOptions {
            sample_count: Some(1),
            limit: Some(1),
            ..Default::default()
        },
        ResultOptions {
            sample_count: Some(1),
            offset: Some(0),
            ..Default::default()
        },
        ResultOptions {
            sample_count: Some(1),
            filters: Some("a=1".into()),
            ..Default::default()
        },
    ] {
        assert!(client.get_latest_query_results(123, options).await.is_err());
    }
    assert!(mock.requests.lock().unwrap().is_empty());
    for key in ["", "with space", "bad\nkey"] {
        assert!(DuneClient::with_transport(API_URL, key, mock.clone()).is_err());
    }
    mock.push(
        200,
        vec![],
        r#"{"execution_id":"x","state":"QUERY_STATE_PENDING"}"#,
    );
    client.get_execution_status("safe/encoded").await.unwrap();
    assert!(
        mock.requests.lock().unwrap()[0]
            .url
            .ends_with("execution/safe%2Fencoded/status")
    );
}
