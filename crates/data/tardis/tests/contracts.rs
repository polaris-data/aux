use aux_rs_core::{
    HttpRequest, StreamingHttpResponse, StreamingHttpTransport, TransportError,
    WebSocketConnection, WebSocketConnector,
};
use aux_rs_data::BytesBody;
use aux_rs_tardis::*;
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
type ResponseQueue = Arc<Mutex<std::collections::VecDeque<(u16, Vec<u8>, String)>>>;
#[derive(Clone, Default, Debug)]
struct Mock {
    requests: Arc<Mutex<Vec<HttpRequest>>>,
    responses: ResponseQueue,
}
impl Mock {
    fn push(&self, status: u16, body: Vec<u8>, encoding: &str) {
        self.responses
            .lock()
            .unwrap()
            .push_back((status, body, encoding.into()));
    }
}
impl StreamingHttpTransport for Mock {
    type Body = BytesBody;
    async fn execute_stream(
        &self,
        request: HttpRequest,
    ) -> std::result::Result<StreamingHttpResponse<BytesBody>, TransportError> {
        self.requests.lock().unwrap().push(request);
        let (status, body, encoding) = self
            .responses
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected request");
        Ok(StreamingHttpResponse {
            status,
            headers: vec![("content-encoding".into(), encoding)],
            body: BytesBody(Some(body)),
        })
    }
}
#[tokio::test]
async fn native_http_inventory() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/http.json")).unwrap();
    for fixture in fixtures.as_array().unwrap() {
        let mock = Mock::default();
        let expected = fixture["response"].clone();
        mock.push(200, expected.to_string().into_bytes(), "");
        let client = TardisClient::with_transport(
            "http://localhost/v1",
            "http://localhost/datasets/v1",
            mock.clone(),
        )
        .unwrap()
        .with_api_key("fixture-secret");
        let name = fixture["operation"].as_str().unwrap();
        let output = match name {
            "exchanges" => client.exchanges().await,
            "exchange_details" => client.exchange_details("bitmex").await,
            "exchange_summary" => client.exchange_summary("bitmex").await,
            "symbols" => client
                .symbols(
                    params(
                        json!({"exchanges":["bitmex","deribit"],"baseCurrency":"BTC","limit":2}),
                    )
                    .unwrap(),
                )
                .await,
            "api_key_info" => client.api_key_info().await,
            "instruments" => {
                client
                    .instruments(
                        "bitmex",
                        params(json!({"filter":{"baseCurrency":"BTC","active":true}})).unwrap(),
                    )
                    .await
            }
            "instrument" => client.instrument("bitmex", "BTC/USD:PERP").await,
            _ => panic!("uninventoried operation: {name}"),
        }
        .unwrap();
        assert_eq!(output, expected);
        let requests = mock.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        let wire = &requests[0];
        let url = url::Url::parse(&wire.url).unwrap();
        assert_eq!(url.path(), fixture["path"].as_str().unwrap());
        assert_eq!(format!("{:?}", wire.method), "Get");
        assert!(
            wire.headers
                .iter()
                .any(|(k, v)| k == "authorization" && v == "Bearer fixture-secret")
        );
        if name == "symbols" {
            assert_eq!(
                url.query_pairs().find(|(k, _)| k == "exchanges").unwrap().1,
                "bitmex,deribit"
            );
        }
        if name == "instruments" {
            let filter = url
                .query_pairs()
                .find(|(k, _)| k == "filter")
                .unwrap()
                .1
                .into_owned();
            assert_eq!(
                serde_json::from_str::<Value>(&filter).unwrap(),
                json!({"active":true,"baseCurrency":"BTC"})
            );
        }
        assert!(!format!("{client:?}").contains("fixture-secret"));
    }
}
#[tokio::test]
async fn compressed_raw_feed_and_exclusive_replay() {
    let mock = Mock::default();
    mock.push(
        200,
        include_bytes!("fixtures/feed.ndjson.gz").to_vec(),
        "gzip",
    );
    let client = TardisClient::with_transport(
        "http://localhost/v1",
        "http://localhost/datasets/v1",
        mock.clone(),
    )
    .unwrap();
    let mut stream = client
        .data_feeds(FeedRequest::new("bitmex", "2024-01-01"))
        .await
        .unwrap();
    let batch = stream.read_batch(4).await.unwrap();
    assert_eq!(batch.len(), 2);
    assert_eq!(batch[0]["localTimestamp"], "2024-01-01T00:00:00.000000123Z");
    assert_eq!(
        batch[0]["message"]["price"].to_string(),
        "123.123456789123456789"
    );
    assert_eq!(batch[0]["message"]["id"], json!(u64::MAX));
    assert_eq!(batch[1], json!({"type":"disconnect"}));
    assert!(stream.next().await.is_none());
    mock.push(200, Vec::new(), "");
    mock.push(200, include_bytes!("fixtures/feed.ndjson").to_vec(), "");
    let mut replay = client
        .replay(ReplayRequest {
            exchange: "bitmex".into(),
            from: "2024-01-01".into(),
            to: "2024-01-01T00:02:00Z".into(),
            filters: Vec::new(),
        })
        .unwrap();
    assert_eq!(replay.read_batch(4).await.unwrap().len(), 2);
    assert!(replay.next().await.is_none());
    let requests = mock.requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    let offsets = requests[1..]
        .iter()
        .map(|r| {
            url::Url::parse(&r.url)
                .unwrap()
                .query_pairs()
                .find(|(k, _)| k == "offset")
                .unwrap()
                .1
                .into_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(offsets, vec!["0", "1"]);
}
#[tokio::test]
async fn dataset_bytes_and_native_errors() {
    let mock = Mock::default();
    let bytes = include_bytes!("fixtures/feed.ndjson.gz").to_vec();
    mock.push(200, bytes.clone(), "");
    let client = TardisClient::with_transport(
        "http://localhost/v1",
        "http://localhost/datasets/v1",
        mock.clone(),
    )
    .unwrap()
    .with_api_key("fixture-secret");
    let body = client
        .dataset(DatasetRequest {
            range: Some("bytes=0-1023".into()),
            exchange: "bitmex".into(),
            data_type: "trades".into(),
            date: "2024-01-01".into(),
            symbol: "BTC/USD:PERP".into(),
        })
        .await
        .unwrap()
        .collect(1024)
        .await
        .unwrap();
    assert_eq!(body, bytes);
    assert!(
        mock.requests.lock().unwrap()[0]
            .url
            .ends_with("BTC%2FUSD%3APERP.csv.gz")
    );
    for status in [401, 403, 429, 500] {
        mock.push(
            status,
            json!({"code":276,"message":"native rejection fixture-secret"})
                .to_string()
                .into_bytes(),
            "",
        );
        let e = client.exchanges().await.unwrap_err();
        assert!(matches!(e,Error::Api{status:s,..} if s==status));
        assert!(!format!("{e:?}").contains("fixture-secret"));
        if let Error::Api { body, .. } = e {
            assert_eq!(body["message"], "native rejection [redacted]");
        }
    }
    assert_eq!(mock.requests.lock().unwrap().len(), 5);
}
#[tokio::test]
async fn machine_http_and_all_normalized_schemas() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/normalized.json")).unwrap();
    let body = fixtures
        .as_array()
        .unwrap()
        .iter()
        .map(Value::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    for normalized in [false, true] {
        let mock = Mock::default();
        mock.push(200, body.as_bytes().to_vec(), "");
        let client =
            TardisMachineClient::with_transport("http://localhost:8000", mock.clone()).unwrap();
        let options = json!([{"exchange":"bitmex","from":"2024-01-01","to":"2024-01-02","dataTypes":["trade","book_change"]}]);
        let mut stream = if normalized {
            client.replay_normalized(options.clone()).await.unwrap()
        } else {
            client.replay(options.clone()).await.unwrap()
        };
        assert_eq!(
            stream.read_batch(100).await.unwrap(),
            *fixtures.as_array().unwrap()
        );
        let requests = mock.requests.lock().unwrap();
        let url = url::Url::parse(&requests[0].url).unwrap();
        assert_eq!(
            url.path(),
            if normalized {
                "/replay-normalized"
            } else {
                "/replay"
            }
        );
        assert_eq!(
            serde_json::from_str::<Value>(
                &url.query_pairs().find(|(k, _)| k == "options").unwrap().1
            )
            .unwrap(),
            options
        );
    }
}
#[derive(Clone)]
struct WsMock {
    url: Arc<Mutex<String>>,
    sent: Arc<Mutex<Vec<String>>>,
}
struct Connection {
    sent: Arc<Mutex<Vec<String>>>,
    events: std::collections::VecDeque<String>,
}
impl WebSocketConnector for WsMock {
    type Connection = Connection;
    async fn connect(&self, url: &str) -> std::result::Result<Connection, TransportError> {
        *self.url.lock().unwrap() = url.into();
        Ok(Connection {
            sent: self.sent.clone(),
            events: vec!["{\"type\":\"trade\",\"price\":0.000000000000000001}".into()].into(),
        })
    }
}
impl WebSocketConnection for Connection {
    async fn next_with_timeout(
        &mut self,
        _: std::time::Duration,
    ) -> Option<std::result::Result<String, TransportError>> {
        self.events.pop_front().map(Ok)
    }
    async fn next(&mut self) -> Option<std::result::Result<String, TransportError>> {
        self.events.pop_front().map(Ok)
    }
    async fn send(&mut self, text: String) -> std::result::Result<(), TransportError> {
        self.sent.lock().unwrap().push(text);
        Ok(())
    }
    async fn close(&mut self) -> std::result::Result<(), TransportError> {
        self.events.clear();
        Ok(())
    }
}
#[tokio::test]
async fn machine_websocket_inventory() {
    for path in ["ws-replay", "ws-replay-normalized", "ws-stream-normalized"] {
        let mock = WsMock {
            url: Arc::default(),
            sent: Arc::default(),
        };
        let client =
            TardisMachineClient::with_transport("https://localhost:8000", mock.clone()).unwrap();
        let options = json!({"exchange":"bitmex","dataTypes":["trade"]});
        let subscribe = json!({"op":"subscribe","args":["trade:XBTUSD"]});
        let mut stream = match path {
            "ws-replay" => client
                .ws_replay(
                    "bitmex",
                    "2024-01-01",
                    "2024-01-02",
                    vec![subscribe.clone()],
                    Some("fixture-session"),
                )
                .await
                .unwrap(),
            "ws-replay-normalized" => client.ws_replay_normalized(options.clone()).await.unwrap(),
            _ => client.stream_normalized(options.clone()).await.unwrap(),
        };
        let url = url::Url::parse(&mock.url.lock().unwrap()).unwrap();
        assert_eq!(url.scheme(), "wss");
        assert_eq!(url.path(), format!("/{path}"));
        if path == "ws-replay" {
            assert_eq!(
                mock.sent.lock().unwrap().as_slice(),
                &[subscribe.to_string()]
            );
        } else {
            assert_eq!(
                serde_json::from_str::<Value>(
                    &url.query_pairs().find(|(k, _)| k == "options").unwrap().1
                )
                .unwrap(),
                options
            );
        }
        assert_eq!(
            stream.next().await.unwrap().unwrap()["price"].to_string(),
            "0.000000000000000001"
        );
        stream.close().await.unwrap();
        assert!(stream.next().await.is_none());
    }
}
#[test]
fn request_validation() {
    let client = TardisClient::with_transport(
        "http://localhost/v1",
        "http://localhost/v1",
        Mock::default(),
    )
    .unwrap();
    for (from, to) in [
        ("2024-01-02", "2024-01-01"),
        ("2024-01-01T00:00:01Z", "2024-01-02"),
        ("bad", "2024-01-02"),
    ] {
        assert!(
            client
                .replay(ReplayRequest {
                    exchange: "bitmex".into(),
                    from: from.into(),
                    to: to.into(),
                    filters: Vec::new()
                })
                .is_err()
        );
    }
    let mut request = FeedRequest::new("bitmex", "2024-01-01");
    request.slice_size = 10;
    assert!(request.validate().is_err());
}

#[tokio::test]
async fn native_file_helper() {
    let mock = Mock::default();
    let bytes = include_bytes!("fixtures/feed.ndjson.gz").to_vec();
    mock.push(200, bytes.clone(), "");
    let client =
        TardisClient::with_transport("http://localhost/v1", "http://localhost/v1", mock).unwrap();
    let dir = std::env::temp_dir().join(format!("aux-tardis-file-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("fixture.gz");
    let size = client
        .download_dataset(
            DatasetRequest {
                exchange: "bitmex".into(),
                data_type: "trades".into(),
                date: "2024-01-01".into(),
                symbol: "XBTUSD".into(),
                range: None,
            },
            &path,
        )
        .await
        .unwrap();
    assert_eq!(size, bytes.len() as u64);
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    std::fs::remove_dir_all(dir).unwrap();
}

#[tokio::test]
async fn all_csv_download_types() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/datasets.json")).unwrap();
    for fixture in fixtures.as_array().unwrap() {
        let bytes = std::fs::read(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures")
                .join(fixture["file"].as_str().unwrap()),
        )
        .unwrap();
        let mock = Mock::default();
        mock.push(200, bytes.clone(), "");
        let client = TardisClient::with_transport(
            "http://localhost/v1",
            "http://localhost/v1",
            mock.clone(),
        )
        .unwrap();
        let kind = fixture["data_type"].as_str().unwrap();
        let body = client
            .dataset(DatasetRequest {
                exchange: "bitmex".into(),
                data_type: kind.into(),
                date: "2024-01-01".into(),
                symbol: "XBTUSD".into(),
                range: None,
            })
            .await
            .unwrap();
        assert_eq!(body.collect(1024 * 1024).await.unwrap(), bytes);
        assert!(
            mock.requests.lock().unwrap()[0]
                .url
                .contains(&format!("/{kind}/"))
        );
    }
}
