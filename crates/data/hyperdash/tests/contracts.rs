use aux_rs_core::{HttpMethod, HttpRequest, StreamingHttpResponse, StreamingHttpTransport};
use aux_rs_data::BytesBody;
use aux_rs_hyperdash::{
    Error, GetCandlesRequest, GetStatusRequest, HyperdashClient, HyperdashConfig,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::BTreeMap, sync::Mutex};
use url::Url;

const KEY: &str = "hdk_sk_fixture_secret";
const PAGE: &[u8] = br#"{"data":[{"t":18446744073709551615,"o":"1.2500","q":null,"vendor_extra":"field"},{"o":"0.5000"}],"has_more":true,"next_cursor":"next/+","meta":{},"new":true}"#;

#[derive(Deserialize)]
struct Fixture {
    operation: String,
    method: String,
    path: String,
    auth: bool,
    query: BTreeMap<String, String>,
    headers: BTreeMap<String, String>,
    body: Option<Value>,
    request: Value,
    status: u16,
    text: bool,
    response: Value,
}

struct Case {
    response: (u16, Vec<u8>),
    request: Mutex<Option<HttpRequest>>,
}

impl Case {
    fn new(status: u16, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            response: (status, bytes.into()),
            request: Mutex::default(),
        }
    }

    fn take(&self) -> HttpRequest {
        self.request.lock().unwrap().take().unwrap()
    }
}

impl StreamingHttpTransport for Case {
    type Body = BytesBody;

    async fn execute_stream(
        &self,
        request: HttpRequest,
    ) -> Result<StreamingHttpResponse<Self::Body>, aux_rs_core::TransportError> {
        assert!(self.request.lock().unwrap().replace(request).is_none());
        Ok(StreamingHttpResponse {
            status: self.response.0,
            headers: vec![
                ("x-request-id".into(), "fixture-id".into()),
                ("X-Credits-Charged".into(), "123".into()),
                ("X-Credit-Price-Version".into(), "2".into()),
                ("X-Credits-Low".into(), "true".into()),
                ("X-Has-More".into(), "true".into()),
                ("X-Next-Cursor".into(), "after/+=".into()),
                ("X-As-Of".into(), "1700000000000".into()),
                ("X-Complete-To".into(), "1699999999000".into()),
                ("X-Truncated".into(), "false".into()),
                ("X-Max-Rows".into(), "100".into()),
                ("X-Incomplete".into(), "false".into()),
                ("Retry-After".into(), "3".into()),
            ],
            body: BytesBody(Some(self.response.1.clone())),
        })
    }
}

fn config() -> HyperdashConfig {
    HyperdashConfig {
        api_key: Some(KEY.into()),
        ..HyperdashConfig::default()
    }
}

#[tokio::test]
async fn every_route_preserves_native_dispatch() {
    let fixtures: Vec<Fixture> = serde_json::from_str(include_str!("dispatch.json")).unwrap();
    assert_eq!(fixtures.len(), 259);
    for fixture in fixtures {
        for (suffix, accept, text) in [
            ("", "application/json", None),
            ("_csv", "text/csv", Some("id,name\r\n1,name\r\n")),
            ("_ndjson", "application/x-ndjson", Some("{\"id\":1}\n")),
        ] {
            if text.is_some() && !fixture.text {
                continue;
            }
            let body = text.map_or_else(
                || serde_json::to_vec(&fixture.response).unwrap(),
                |text| text.as_bytes().to_vec(),
            );
            let client =
                HyperdashClient::with_transport(config(), Case::new(fixture.status, body)).unwrap();
            let response = client
                .dispatch(
                    &format!("{}{suffix}", fixture.operation),
                    fixture.request.clone(),
                )
                .await;
            let response = serde_json::to_value(response.unwrap_or_else(|error| {
                panic!("{}: {error}", fixture.operation);
            }))
            .unwrap();
            assert_eq!(response["status"], fixture.status);
            assert_eq!(
                response["data"],
                text.map_or_else(|| fixture.response.clone(), |text| json!(text))
            );
            assert_eq!(response["metadata"]["next_cursor"], "after/+=");
            let request = client.transport().take();
            let url = Url::parse(&request.url).unwrap();
            assert_eq!(url.path(), fixture.path, "{}", fixture.operation);
            assert_eq!(
                url.query_pairs().into_owned().collect::<BTreeMap<_, _>>(),
                fixture.query,
                "{}",
                fixture.operation
            );
            assert_eq!(
                format!("{:?}", request.method).to_uppercase(),
                fixture.method
            );
            assert_eq!(
                aux_rs_data::header(&request.headers, "accept"),
                Some(accept)
            );
            assert_eq!(
                aux_rs_data::header(&request.headers, "authorization").is_some(),
                fixture.auth
            );
            for (name, value) in &fixture.headers {
                assert_eq!(
                    aux_rs_data::header(&request.headers, name),
                    Some(value.as_str())
                );
            }
            if let Some(expected) = &fixture.body {
                assert_eq!(
                    aux_rs_data::header(&request.headers, "content-type"),
                    Some("application/json")
                );
                assert_eq!(
                    &serde_json::from_slice::<Value>(&request.body).unwrap(),
                    expected
                );
            } else {
                assert!(request.body.is_empty());
            }
        }
    }
}

fn candles(coin: &str) -> GetCandlesRequest {
    serde_json::from_value(json!({"coin": coin, "limit": 5, "cursor": "a+/=", "max_credits": 7}))
        .unwrap()
}

#[tokio::test]
async fn typed_data_retains_presence_precision_and_path_segments() {
    let client = HyperdashClient::with_transport(config(), Case::new(200, PAGE)).unwrap();
    let response = client.get_candles(candles("xyz:TSLA/#?")).await.unwrap();
    assert_eq!(response.data.data[0].t, Some(18_446_744_073_709_551_615));
    assert_eq!(response.data.data[0].o.as_deref(), Some("1.2500"));
    assert_eq!(response.data.data[0].q, Some(None));
    assert_eq!(response.data.data[1].q, None);
    assert_eq!(response.data.data[0].extra["vendor_extra"], "field");
    assert_eq!(response.data.extra["new"], true);
    assert_eq!(response.metadata.credits_charged, Some(123));
    assert_eq!(response.metadata.credits_low, Some(true));
    assert_eq!(response.metadata.incomplete, Some(false));
    assert_eq!(response.metadata.complete_to, Some(1_699_999_999_000));
    let request = client.transport().take();
    assert!(
        request
            .url
            .contains("/v1/markets/xyz%3ATSLA%2F%23%3F/candles")
    );
    assert!(request.url.contains("cursor=a%2B%2F%3D"));
    assert_eq!(
        aux_rs_data::header(&request.headers, "x-max-credits"),
        Some("7")
    );
    assert_eq!(
        aux_rs_data::header(&request.headers, "authorization"),
        Some("Bearer hdk_sk_fixture_secret")
    );
    assert!(matches!(
        client.get_candles(candles("..")).await,
        Err(Error::InvalidRequest(_))
    ));
    assert!(client.transport().request.lock().unwrap().is_none());
}

#[tokio::test]
async fn errors_limits_and_credits_are_caller_controlled() {
    let client = HyperdashClient::with_transport(
        config(),
        Case::new(
            429,
            serde_json::to_vec(&json!({
                "code": "rate_limited",
                "detail": format!("Bearer {KEY}"),
                "retry_after_s": 3,
                KEY: KEY,
            }))
            .unwrap(),
        ),
    )
    .unwrap();
    let error = client.get_candles(candles("BTC")).await.unwrap_err();
    assert_eq!(error.kind(), "rate_limit");
    assert!(!format!("{error:?}").contains(KEY));
    let Error::Api(error) = error else {
        panic!("expected API error");
    };
    assert_eq!(error.status, 429);
    assert_eq!(error.metadata.retry_after.as_deref(), Some("3"));
    assert_eq!(error.problem["code"], "rate_limited");
    assert_eq!(error.problem["detail"], "[redacted]");
    assert!(!error.problem.to_string().contains(KEY));
    assert_eq!(client.transport().take().method, HttpMethod::Get);
    assert!(matches!(
        client
            .dispatch(
                "create_job",
                json!({
                    "body": {"kind": "account_fills_summary", "params": {}}
                })
            )
            .await,
        Err(Error::InvalidRequest(_))
    ));
    assert!(matches!(
        client
            .dispatch(
                "create_job",
                json!({
                    "idempotency_key": "fixture", "body": {
                        "kind": "account_fills_summary", "params": {"blob": "x".repeat(65_536)}
                    }
                })
            )
            .await,
        Err(Error::InvalidRequest(_))
    ));
    assert!(client.transport().request.lock().unwrap().is_none());

    let large = serde_json::to_vec(&json!({
        "data": [], "has_more": false, "next_cursor": null, "meta": {},
        "blob": "x".repeat(2 * 1024 * 1024),
    }))
    .unwrap();
    let client = HyperdashClient::with_transport(config(), Case::new(200, large)).unwrap();
    assert!(matches!(
        client.get_candles(candles("BTC")).await,
        Err(Error::Decode(_))
    ));
    client.transport().take();
    assert!(
        client
            .dispatch("get_accounts_batch", json!({"addresses": "0x1111"}))
            .await
            .is_ok()
    );
    client.transport().take();

    let client = HyperdashClient::with_transport(
        HyperdashConfig::default(),
        Case::new(200, br#"{"as_of":12,"datasets":[]}"#.as_slice()),
    )
    .unwrap();
    assert!(matches!(
        client.get_candles(candles("BTC")).await,
        Err(Error::InvalidRequest(_))
    ));
    assert_eq!(
        client
            .get_status(GetStatusRequest::default())
            .await
            .unwrap()
            .data
            .as_of,
        12
    );
    assert!(aux_rs_data::header(&client.transport().take().headers, "authorization").is_none());
}

#[cfg(feature = "http")]
#[tokio::test]
async fn built_in_http_sends_one_request_and_respects_the_configured_url() {
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let peer = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut received = Vec::new();
        while !received.ends_with(b"\r\n\r\n") {
            received.push(socket.read_u8().await.unwrap());
        }
        let text = String::from_utf8(received).unwrap().to_ascii_lowercase();
        assert!(text.starts_with("get /proxy/v1/markets/%23232960/candles?"));
        assert!(text.contains(&format!("authorization: bearer {KEY}\r\n")));
        assert!(text.contains("accept: text/csv\r\n"));
        socket.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\nX-Next-Cursor: cursor\r\nConnection: close\r\n\r\nx,y\r\n").await.unwrap();
    });
    let client = HyperdashClient::with_config(
        HyperdashConfig {
            api_url: format!("http://{address}/proxy"),
            ..config()
        },
        Default::default(),
    )
    .unwrap();
    let response = client.get_candles_csv(candles("#232960")).await.unwrap();
    assert_eq!(response.data, "x,y\r\n");
    assert_eq!(response.metadata.next_cursor.as_deref(), Some("cursor"));
    peer.await.unwrap();
    client.transport().shutdown().await.unwrap();
}
