use aux_rs_chainlink::*;
use aux_rs_core::{
    ByteStream, HttpRequest, StreamingHttpResponse, StreamingHttpTransport, TransportError,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, VecDeque};

#[derive(Clone)]
struct FixtureTransport(Value);
struct Body(VecDeque<Vec<u8>>);
impl ByteStream for Body {
    async fn next(&mut self) -> Option<std::result::Result<Vec<u8>, TransportError>> {
        self.0.pop_front().map(Ok)
    }
    async fn close(&mut self) -> std::result::Result<(), TransportError> {
        self.0.clear();
        Ok(())
    }
}

impl StreamingHttpTransport for FixtureTransport {
    type Body = Body;
    async fn execute_stream(
        &self,
        request: HttpRequest,
    ) -> std::result::Result<StreamingHttpResponse<Body>, TransportError> {
        let row = &self.0;
        let url = url::Url::parse(&request.url).unwrap();
        assert_eq!(url.path(), row["path"]);
        let mut query = BTreeMap::<String, Vec<String>>::new();
        for (k, v) in url.query_pairs() {
            query
                .entry(k.into_owned())
                .or_default()
                .push(v.into_owned());
        }
        assert_eq!(json!(query), row["query"]);
        assert_eq!(
            format!("{:?}", request.method).to_ascii_uppercase(),
            row["verb"]
        );
        match row["kind"].as_str().unwrap() {
            "post" => assert_eq!(
                serde_json::from_slice::<Value>(&request.body).unwrap(),
                row["body"]
            ),
            "form" => assert_eq!(
                json!(
                    url::form_urlencoded::parse(&request.body)
                        .into_owned()
                        .collect::<BTreeMap<_, _>>()
                ),
                row["body"]
            ),
            _ => assert!(request.body.is_empty()),
        }
        let headers = serde_json::from_value::<BTreeMap<String, String>>(row["headers"].clone())
            .unwrap()
            .into_iter()
            .collect();
        let body = row["response"]
            .as_str()
            .unwrap()
            .as_bytes()
            .chunks(3)
            .map(<[u8]>::to_vec)
            .collect();
        Ok(StreamingHttpResponse {
            status: row["status"].as_u64().unwrap() as u16,
            headers,
            body: Body(body),
        })
    }
}

enum TestOutput {
    Response(NativeResponse),
    Stream(JsonStream),
}
include!("generated/dispatch.rs");

#[tokio::test]
async fn native_http_inventory() {
    let rows: Vec<Value> = serde_json::from_str(include_str!("fixtures/operations.json")).unwrap();
    for row in rows {
        let kind = row["kind"].as_str().unwrap();
        if kind.ends_with("_ws") {
            continue;
        }
        let result = dispatch(&row, FixtureTransport(row.clone())).await;
        if row["status"] == 400 {
            assert!(matches!(result, Err(Error::Api { status: 400, .. })));
            continue;
        }
        match result.unwrap() {
            TestOutput::Response(response) => {
                assert_eq!(response.status, row["status"]);
                let expected = if kind == "text" {
                    row["response"].clone()
                } else {
                    serde_json::from_str(row["response"].as_str().unwrap()).unwrap()
                };
                assert_eq!(response.data, expected);
                if row["method"] == "price_range" {
                    assert_eq!(response.headers[0].1, "7");
                }
            }
            TestOutput::Stream(mut stream) => {
                if kind == "sse" {
                    assert_eq!(
                        stream.next().await.unwrap().unwrap(),
                        json!({"binary":{"data":["abcd"]},"parsed":[]})
                    );
                } else {
                    assert_eq!(
                        stream.next().await.unwrap().unwrap(),
                        json!({"heartbeat":1748525528})
                    );
                    assert_eq!(
                        stream.next().await.unwrap().unwrap()["p"].to_string(),
                        "1.0000000000000000001e+21"
                    );
                }
                assert!(matches!(
                    stream.next().await,
                    Some(Err(Error::Transport(_)))
                ));
                assert!(stream.next().await.is_none());
            }
        }
    }
}

fn test_config() -> ChainlinkConfig {
    ChainlinkConfig {
        api_url: Some("http://fixture/".into()),
        candles_url: Some("http://fixture/".into()),
        api_key: Some("fixture-key".into()),
        api_secret: Some("fixture-secret".into()),
        token: Some("fixture-token".into()),
        ..ChainlinkConfig::new(Network::Testnet)
    }
}

#[test]
fn pinned_report_schemas_and_signature_vector() {
    let rows: Vec<Value> = serde_json::from_str(include_str!("fixtures/reports.json")).unwrap();
    for row in rows {
        let full = row["full_report"].as_str().unwrap();
        let result = serde_json::to_value(decode_report(full).unwrap()).unwrap();
        assert_eq!(result["version"], row["version"]);
        assert_eq!(result["report_body"], row["report_body"]);
        assert_eq!(result["full_report"], full);
        if let Some(field) = row["key_field"].as_str() {
            assert_eq!(result["fields"][field], row["key_value"]);
        } else {
            assert_eq!(result["kind"], "unknown");
        }
        assert!(decode_report(&full[..full.len() - 2]).is_err());
    }
    let auth = Credentials::new("fixture-key".into(), "fixture-secret".into()).unwrap();
    let headers = auth
        .headers_at(
            "GET",
            "https://test.invalid/api/v1/ws?feedIDs=a%2Cb",
            b"",
            1700000000000,
        )
        .unwrap();
    assert_eq!(
        headers[2].1,
        "0f77e2e9b31a57508ab1a2848c3b043ca65724e01a59a6af6ed86c26d0998fde"
    );
}

fn test_authorization(response: NativeResponse<AuthorizationResponse>) -> Result<TestOutput> {
    assert_eq!(response.data.d.access_token, "fixture-token");
    assert_eq!(response.data.d.expiration, 1747203979);
    Ok(TestOutput::Response(NativeResponse {
        status: response.status,
        headers: response.headers,
        data: serde_json::to_value(response.data).unwrap(),
    }))
}
