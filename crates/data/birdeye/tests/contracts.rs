use aux_rs_birdeye::*;
use aux_rs_core::{
    HttpMethod, HttpRequest, StreamingHttpResponse, StreamingHttpTransport, TransportError,
};
use aux_rs_data::{BytesBody, header};
use serde_json::Value;
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
    fn client(&self) -> BirdeyeClient<Self> {
        BirdeyeClient::with_transport("https://birdeye.test", "fixture-secret", self.clone())
            .unwrap()
            .with_chain("base")
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
include!("generated/dispatch.rs");
#[tokio::test]
async fn native_http_inventory() {
    let rows: Vec<Value> = serde_json::from_str(include_str!("fixtures/http.json")).unwrap();
    assert_eq!(rows.len(), 46);
    for row in rows {
        let mock = Mock::default();
        mock.push(
            200,
            vec![],
            row["response_body"].as_str().unwrap().as_bytes(),
        );
        let output = invoke(
            &mock.client(),
            row["operation"].as_str().unwrap(),
            row["request_json"].as_str().unwrap(),
        )
        .await
        .unwrap();
        let expected: Value = serde_json::from_str(row["response_body"].as_str().unwrap()).unwrap();
        assert_eq!(serde_json::to_value(&output).unwrap(), expected);
        assert_eq!(output.data["value"].to_string(), "0.1000000000000000001");
        assert_eq!(
            output.data["rawAmount"].to_string(),
            "123456789012345678901234567890"
        );
        let requests = mock.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        let request = &requests[0];
        let url = url::Url::parse(&request.url).unwrap();
        assert_eq!(url.path(), row["path"].as_str().unwrap());
        assert_eq!(
            format!("{:?}", request.method).to_uppercase(),
            row["verb"].as_str().unwrap()
        );
        let query: std::collections::BTreeMap<String, String> = url
            .query_pairs()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        assert_eq!(serde_json::to_value(query).unwrap(), row["query"]);
        assert_eq!(
            header(&request.headers, "X-API-KEY"),
            Some("fixture-secret")
        );
        assert_eq!(header(&request.headers, "x-chain"), Some("base"));
        assert!(header(&request.headers, "Authorization").is_none());
        if let Some(body) = row["body_json"].as_str() {
            assert_eq!(request.method, HttpMethod::Post);
            assert_eq!(
                header(&request.headers, "content-type"),
                Some("application/json")
            );
            assert_eq!(
                serde_json::from_slice::<Value>(&request.body).unwrap(),
                serde_json::from_str::<Value>(body).unwrap()
            );
        } else {
            assert!(request.body.is_empty());
        }
    }
}
#[tokio::test]
async fn native_failures_nulls_defaults_and_redaction() {
    for status in [200, 400, 401, 403, 422, 429, 500] {
        let mock = Mock::default();
        mock.push(
            status,
            vec![],
            r#"{"success":false,"message":"fixture-secret rejected","code":"native-code"}"#,
        );
        let client = mock.client();
        let error = client
            .price(PriceRequest {
                address: "TokenA".into(),
                ..Default::default()
            })
            .await
            .unwrap_err();
        if let Error::Api {
            status: actual,
            body,
        } = error
        {
            assert_eq!(actual, status);
            assert_eq!(body["code"], "native-code");
            assert!(!body.to_string().contains("fixture-secret"));
        } else {
            panic!("native rejection lost");
        }
        assert_eq!(mock.requests.lock().unwrap().len(), 1);
        assert!(!format!("{client:?}").contains("fixture-secret"));
    }
    for data in ["null", "[]", "{}"] {
        let mock = Mock::default();
        mock.push(200, vec![], format!("{{\"success\":true,\"data\":{data}}}"));
        mock.client()
            .price(PriceRequest {
                address: "TokenA".into(),
                ..Default::default()
            })
            .await
            .unwrap();
        assert!(
            mock.requests.lock().unwrap()[0]
                .url
                .ends_with("price?address=TokenA")
        );
    }
    for data in ["not JSON", "{}", r#"{"success":"true","data":null}"#] {
        let mock = Mock::default();
        mock.push(200, vec![], data);
        assert!(matches!(
            mock.client()
                .price(PriceRequest {
                    address: "TokenA".into(),
                    ..Default::default()
                })
                .await,
            Err(Error::Decode(_))
        ));
    }
    let mock = Mock::default();
    mock.push(200, vec![], r#"{"success":true,"data":{"items":[]}}"#);
    mock.client()
        .token_list_scroll(TokenListScrollRequest {
            scroll_id: Some("cursor-1".into()),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(
        mock.requests.lock().unwrap()[0]
            .url
            .ends_with("?scroll_id=cursor-1")
    );
}
#[tokio::test]
async fn redirects_and_invalid_credentials() {
    let mock = Mock::default();
    mock.push(
        302,
        vec![("location".into(), "https://other.test/result".into())],
        Vec::new(),
    );
    mock.push(200, vec![], r#"{"success":true,"data":null}"#);
    mock.client()
        .price(PriceRequest {
            address: "TokenA".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(header(&mock.requests.lock().unwrap()[1].headers, "X-API-KEY").is_none());
    for key in ["", "with space", "key\n"] {
        assert!(BirdeyeClient::with_transport(API_URL, key, Mock::default()).is_err());
    }
    for chain in ["", "chain\r\n"] {
        assert!(mock.client().with_chain(chain).is_err());
    }
    let mock = Mock::default();
    mock.push(
        307,
        vec![("location".into(), "https://birdeye.test/new".into())],
        Vec::new(),
    );
    assert!(
        mock.client()
            .multi_price_post(MultiPricePostRequest {
                body: MultiPricePostBody {
                    list_address: "TokenA,TokenB".into()
                },
                ..Default::default()
            })
            .await
            .is_err()
    );
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
}
