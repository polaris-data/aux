use aux_rs_core::{
    ByteStream, HttpRequest, StreamingHttpResponse, StreamingHttpTransport, TransportError,
};
use aux_rs_pyth::*;
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

fn test_config() -> PythConfig {
    PythConfig {
        api_url: "http://fixture/v1/".into(),
        router_url: "http://fixture/".into(),
        hermes_url: "http://fixture/hermes/".into(),
        api_key: Some("fixture-key".into()),
        publisher_token: Some("publisher-key".into()),
        ..Default::default()
    }
}
