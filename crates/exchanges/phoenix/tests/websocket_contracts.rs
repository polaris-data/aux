use aux_rs_core::{TransportError, WebSocketStream, WebSocketSubscription, WebSocketTransport};
use aux_rs_phoenix::*;
use serde_json::{Value, json};
#[derive(Clone)]
struct Inspect {
    fixture: Value,
}
struct Stream;
impl WebSocketStream for Stream {
    async fn next(&mut self) -> Option<Result<String, TransportError>> {
        None
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        Ok(())
    }
}
impl WebSocketTransport for Inspect {
    type Stream = Stream;
    fn supports_handshake_headers(&self) -> bool {
        true
    }
    async fn subscribe<S: WebSocketSubscription>(
        &self,
        url: &str,
        spec: S,
    ) -> Result<Stream, TransportError> {
        let headers = spec.handshake_headers().await?;
        if self.fixture["private"] == true {
            assert!(!headers.is_empty())
        }
        assert!(url.ends_with("/v1/ws"));
        let frame: Value = serde_json::from_str(&spec.subscribe_message()).unwrap();
        assert_eq!(
            frame,
            json!({"type":"subscribe","subscription":self.fixture["specification"]})
        );
        assert!(spec.validate_message("malformed").is_err());
        let first = self.fixture["frame"].to_string();
        spec.validate_message(&first)?;
        let event = decode_event(&first).unwrap();
        assert_eq!(event, self.fixture["frame"]);
        Ok(Stream)
    }
}
#[tokio::test]
async fn all_native_stream_contracts() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    for fixture in fixtures.as_object().unwrap().values() {
        let inspect = Inspect {
            fixture: fixture.clone(),
        };
        let client = PhoenixClient::with_transport(Phoenix::mainnet(), inspect).with_auth(
            Credentials::new(
                Some(zeroize::Zeroizing::new("fixture-key".into())),
                Some(zeroize::Zeroizing::new("fixture-token".into())),
                None,
            )
            .unwrap(),
        );
        let spec: Subscription = serde_json::from_value(fixture["specification"].clone()).unwrap();
        let mut stream = client.subscribe(spec).await.unwrap();
        stream.close().await.unwrap();
    }
}
