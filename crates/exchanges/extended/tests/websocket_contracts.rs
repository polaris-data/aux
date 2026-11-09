use aux_rs_core::{TransportError, WebSocketStream, WebSocketSubscription, WebSocketTransport};
use aux_rs_extended::*;
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
        assert!(headers.iter().any(|(k, _)| k == "User-Agent"));
        assert!(url.ends_with(self.fixture["path"].as_str().unwrap()));
        if self.fixture["specification"]["rpc"] == true {
            let frame: Value = serde_json::from_str(&spec.subscribe_message()).unwrap();
            assert_eq!(frame["method"], "subscribe");
            assert_eq!(frame["jsonrpc"], "2.0");
            let ack = json!({"jsonrpc":"2.0","id":frame["id"],"result":{}});
            assert!(spec.matches(&ack.to_string()));
            assert!(!spec.matches(&json!({"id":"unrelated","result":{}}).to_string()));
            assert!(
                !spec.matches(&json!({"subscription":"unrelated","data":{},"seq":9}).to_string())
            );
            assert!(spec.heartbeat_message().unwrap().contains("ping"));
        } else {
            assert!(spec.subscribe_message().is_empty());
        }
        let _ = json!({});
        assert!(spec.validate_message("malformed").is_err());
        let first = self.fixture["frame"].to_string();
        assert!(spec.matches(&first));
        spec.validate_message(&first)?;
        if self.fixture["specification"]["rpc"] == true
            && self.fixture["specification"]["channel"] == "orderbooks"
        {
            assert!(spec.is_initial_snapshot(&first));
        }
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
        let client = ExtendedClient::with_transport(Extended::mainnet(), inspect).with_auth(
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

#[tokio::test]
async fn captured_mainnet_rpc_snapshot_starts_a_fresh_baseline() {
    let samples: Value = serde_json::from_str(include_str!("fixtures/rpc_public.json")).unwrap();
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let mut fixture = fixtures["orderbooks_rpc"].clone();
    assert!(fixture.is_object());
    fixture["frame"] = samples[1].clone();
    let client = ExtendedClient::with_transport(
        Extended::mainnet(),
        Inspect {
            fixture: fixture.clone(),
        },
    );
    let specification = serde_json::from_value(fixture["specification"].clone()).unwrap();
    client
        .subscribe(specification)
        .await
        .unwrap()
        .close()
        .await
        .unwrap();
}
