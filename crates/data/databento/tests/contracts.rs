use aux_rs_core::{
    ByteStream, HttpRequest, StreamingHttpResponse, StreamingHttpTransport, TransportError,
};
use aux_rs_data::{BytesBody, DecodedBody};
use aux_rs_databento::{DbnEvent, DbnStream, Error, HistoricalClient, ReferenceClient, params};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
#[derive(Clone, Default, Debug)]
struct Mock {
    requests: Arc<Mutex<Vec<HttpRequest>>>,
    response: Arc<Mutex<Vec<u8>>>,
    status: u16,
}
impl StreamingHttpTransport for Mock {
    type Body = BytesBody;
    async fn execute_stream(
        &self,
        request: HttpRequest,
    ) -> Result<StreamingHttpResponse<BytesBody>, TransportError> {
        self.requests.lock().unwrap().push(request);
        Ok(StreamingHttpResponse {
            status: if self.status == 0 { 200 } else { self.status },
            headers: Vec::new(),
            body: BytesBody(Some(self.response.lock().unwrap().clone())),
        })
    }
}
const KEY: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaa12345";
#[tokio::test]
async fn native_http_inventory() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/http.json")).unwrap();
    for fixture in fixtures.as_array().unwrap() {
        let mock = Mock::default();
        *mock.response.lock().unwrap() = fixture["response"].to_string().into_bytes();
        let history =
            HistoricalClient::with_transport("http://localhost/v0", KEY, mock.clone()).unwrap();
        let reference =
            ReferenceClient::with_transport("http://localhost/v0", KEY, mock.clone()).unwrap();
        let p = params(fixture["request"].clone()).unwrap();
        let name = fixture["operation"].as_str().unwrap();
        let value = match name {
            "list_publishers" => history.list_publishers(p).await,
            "list_datasets" => history.list_datasets(p).await,
            "list_schemas" => history.list_schemas(p).await,
            "list_fields" => history.list_fields(p).await,
            "list_unit_prices" => history.list_unit_prices(p).await,
            "get_dataset_condition" => history.get_dataset_condition(p).await,
            "get_dataset_range" => history.get_dataset_range(p).await,
            "get_record_count" => history.get_record_count(p).await,
            "get_billable_size" => history.get_billable_size(p).await,
            "get_cost" => history.get_cost(p).await,
            "resolve" => history.resolve(p).await,
            "submit_job" => history.submit_job(p).await,
            "list_jobs" => history.list_jobs(p).await,
            "get_job_details" => history.get_job_details(p).await,
            "list_files" => history.list_files(p).await,
            "list_events" => reference.list_events(p).await,
            "list_enums" => reference.list_enums(p).await,
            "security_master_get_range" => {
                let mut stream = reference.security_master_get_range(p).await.unwrap();
                stream.next().await.unwrap()
            }
            "security_master_get_last" => {
                let mut stream = reference.security_master_get_last(p).await.unwrap();
                stream.next().await.unwrap()
            }
            "corporate_actions_get_range" => {
                let mut stream = reference.corporate_actions_get_range(p).await.unwrap();
                stream.next().await.unwrap()
            }
            "adjustment_factors_get_range" => {
                let mut stream = reference.adjustment_factors_get_range(p).await.unwrap();
                stream.next().await.unwrap()
            }
            _ => panic!("uninventoried operation {name}"),
        }
        .unwrap();
        assert_eq!(value, fixture["response"]);
        let requests = mock.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        let wire = &requests[0];
        let url = url::Url::parse(&wire.url).unwrap();
        assert_eq!(url.path(), fixture["path"].as_str().unwrap());
        assert_eq!(
            format!("{:?}", wire.method),
            fixture["verb"].as_str().unwrap()
        );
        assert!(wire.headers.iter().any(|(k, v)| k == "authorization"
            && v == "Basic YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhMTIzNDU6"));
        let encoded = if wire.body.is_empty() {
            url.query().unwrap_or_default().as_bytes()
        } else {
            &wire.body
        };
        let pairs =
            url::form_urlencoded::parse(encoded).collect::<std::collections::HashMap<_, _>>();
        for (key, expected) in fixture["wire_params"].as_object().unwrap() {
            assert_eq!(
                pairs.get(key.as_str()).map(|s| s.as_ref()),
                expected.as_str(),
                "{name}: {key}"
            );
        }
    }
}
struct Fragments(std::collections::VecDeque<Vec<u8>>);
impl ByteStream for Fragments {
    async fn next(&mut self) -> Option<Result<Vec<u8>, TransportError>> {
        self.0.pop_front().map(Ok)
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        self.0.clear();
        Ok(())
    }
}
fn dbn(bytes: &[u8], fragment: usize, encoding: &str) -> DbnStream {
    DbnStream::new(
        DecodedBody::new(
            Fragments(bytes.chunks(fragment).map(|b| b.to_vec()).collect()),
            encoding,
        )
        .unwrap(),
    )
}
#[tokio::test]
async fn all_dbn_schemas_and_control_records() {
    let expected: Value = serde_json::from_str(include_str!("fixtures/schemas.json")).unwrap();
    for (bytes, encoding) in [
        (include_bytes!("fixtures/schemas.dbn").as_slice(), ""),
        (
            include_bytes!("fixtures/schemas.dbn.zst").as_slice(),
            "zstd",
        ),
    ] {
        let mut stream = dbn(bytes, 1, encoding);
        let batch = stream.read_batch(100).await.unwrap();
        assert_eq!(batch.len(), 24);
        assert!(matches!(&batch[0],DbnEvent::Metadata(m) if m.dataset=="GLBX.MDP3"));
        for (actual, expected) in batch[1..].iter().zip(expected.as_array().unwrap()) {
            assert_eq!(
                actual.to_json().unwrap(),
                expected["record"],
                "schema {}",
                expected["schema"]
            );
        }
        assert!(stream.next().await.is_none());
        assert!(stream.metadata().is_some());
    }
}
#[tokio::test]
async fn ts_out_and_truncation() {
    let bytes = include_bytes!("fixtures/ts_out.dbn");
    let mut stream = dbn(bytes, 3, "");
    assert!(matches!(
        stream.next().await.unwrap().unwrap(),
        DbnEvent::Metadata(_)
    ));
    match stream.next().await.unwrap().unwrap() {
        DbnEvent::Record(record) => {
            assert_eq!(record.ts_out, Some(1704067200000000999));
            let output = record.to_json().unwrap();
            assert_eq!(
                output["ts_out"].as_str().unwrap_or_default(),
                "1704067200000000999"
            );
        }
        _ => panic!("expected record"),
    }
    for bytes in [
        bytes[..5].to_vec(),
        bytes[..bytes.len() - 1].to_vec(),
        b"DBN\x03\xff\xff\xff\xff".to_vec(),
    ] {
        let mut stream = dbn(&bytes, 2, "");
        let mut failed = false;
        while let Some(event) = stream.next().await {
            if event.is_err() {
                failed = true;
                break;
            }
        }
        assert!(failed);
        assert!(stream.next().await.is_none());
    }
}
#[tokio::test]
async fn timeseries_downloads_and_no_batch_submission_retry() {
    let mock = Mock::default();
    *mock.response.lock().unwrap() = include_bytes!("fixtures/schemas.dbn").to_vec();
    let client =
        HistoricalClient::with_transport("http://localhost/v0", KEY, mock.clone()).unwrap();
    let mut stream=client.get_range(params(json!({"dataset":"GLBX.MDP3","schema":"trades","symbols":["ES.c.0"],"start":"2024-01-01"})).unwrap()).await.unwrap();
    assert!(matches!(
        stream.next().await.unwrap().unwrap(),
        DbnEvent::Metadata(_)
    ));
    let bytes = client
        .timeseries(params(json!({"encoding":"dbn","compression":"none"})).unwrap())
        .await
        .unwrap()
        .collect(1024 * 1024)
        .await
        .unwrap();
    assert_eq!(bytes, include_bytes!("fixtures/schemas.dbn"));
    let bytes = client
        .download("http://storage.local/file.dbn")
        .await
        .unwrap()
        .collect(1024 * 1024)
        .await
        .unwrap();
    assert_eq!(bytes, include_bytes!("fixtures/schemas.dbn"));
    assert!(
        !mock
            .requests
            .lock()
            .unwrap()
            .last()
            .unwrap()
            .headers
            .iter()
            .any(|(k, _)| k == "authorization")
    );
    let mock = Mock {
        status: 429,
        ..Mock::default()
    };
    *mock.response.lock().unwrap() = format!("{{\"message\":\"rejected {KEY}\"}}").into_bytes();
    let client =
        HistoricalClient::with_transport("http://localhost/v0", KEY, mock.clone()).unwrap();
    let e = client.submit_job(Default::default()).await.unwrap_err();
    assert_eq!(e.kind(), "rate_limit");
    assert_eq!(mock.requests.lock().unwrap().len(), 1);
    if let Error::Api { body, .. } = e {
        assert_eq!(body["message"], "rejected [redacted]");
    }
}

#[tokio::test]
async fn native_file_helper() {
    let mock = Mock::default();
    *mock.response.lock().unwrap() = include_bytes!("fixtures/schemas.dbn").to_vec();
    let client = HistoricalClient::with_transport("http://localhost/v0", KEY, mock).unwrap();
    let dir = std::env::temp_dir().join(format!("aux-databento-file-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("fixture.dbn");
    client
        .download_to("http://localhost/file.dbn", &path)
        .await
        .unwrap();
    assert_eq!(
        std::fs::read(path).unwrap(),
        include_bytes!("fixtures/schemas.dbn")
    );
    std::fs::remove_dir_all(dir).unwrap();
}
