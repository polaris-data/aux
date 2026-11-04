use aux_rs_core::{
    RequestOutcome, TransportError, TransportErrorKind, WebSocketConnection, WebSocketConnector,
};
use aux_rs_polymarket::*;
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use zeroize::Zeroizing;
#[derive(Clone, Default)]
struct Peer {
    sent: Arc<Mutex<Vec<Value>>>,
    queue: Arc<Mutex<VecDeque<Result<String, TransportError>>>>,
}
impl WebSocketConnection for Peer {
    async fn send(&mut self, text: String) -> Result<(), TransportError> {
        let v: Value = serde_json::from_str(&text).unwrap();
        self.sent.lock().unwrap().push(v.clone());
        let mut q = self.queue.lock().unwrap();
        if v["type"] == "auth" {
            q.push_back(Ok(json!({"type":"auth","success":true}).to_string()));
        } else if v["type"] == "RFQ_QUOTE_CANCEL" {
            assert!(v.get("id").is_none());
            q.push_back(Ok(
                json!({"type":"ACK_RFQ_QUOTE_CANCEL","rfq_id":v["rfq_id"],"quote_id":"unrelated"})
                    .to_string(),
            ));
            q.push_back(Ok(json!({"type":"ACK_RFQ_QUOTE_CANCEL","rfq_id":v["rfq_id"],"quote_id":v["quote_id"]}).to_string()));
        } else if v["op"]["type"] == "auth" {
            q.push_back(Ok(json!({"id":v["id"],"data":{"status":"ok"}}).to_string()));
        } else {
            assert!(v["sig"].as_str().unwrap().starts_with("0x"));
            q.push_back(Err(TransportError::new(
                TransportErrorKind::Timeout,
                RequestOutcome::Unknown,
                "fixture timeout",
            )));
            q.push_back(Ok(json!({"id":v["id"],"data":[{"status":"ok","oid":42},{"status":"err","error":"insufficient_margin"}]}).to_string()));
        }
        Ok(())
    }
    async fn next(&mut self) -> Option<Result<String, TransportError>> {
        self.queue.lock().unwrap().pop_front()
    }
    async fn next_with_timeout(
        &mut self,
        _: std::time::Duration,
    ) -> Option<Result<String, TransportError>> {
        self.next().await
    }
    async fn close(&mut self) -> Result<(), TransportError> {
        Ok(())
    }
}
impl WebSocketConnector for Peer {
    type Connection = Self;
    async fn connect(&self, _: &str) -> Result<Self, TransportError> {
        Ok(self.clone())
    }
}
fn signer() -> Arc<LocalEvmSigner> {
    Arc::new(LocalEvmSigner::from_hex(Zeroizing::new("07".repeat(32))).unwrap())
}
#[tokio::test]
async fn rfq_safe_identity_native_ids_and_quote_correlation() {
    let peer = Peer::default();
    let auth = PolymarketAuth::new(
        signer(),
        "0x1111111111111111111111111111111111111111"
            .parse()
            .unwrap(),
    )
    .with_api(
        ApiCredentials::new(
            "public-key".into(),
            Zeroizing::new("cHVibGljLXNlY3JldA==".into()),
            Zeroizing::new("public-pass".into()),
        )
        .unwrap(),
    )
    .with_rfq_signature_type(2)
    .unwrap();
    assert!(auth.clone().with_rfq_signature_type(4).is_err());
    assert!(auth.clone().with_rfq_signature_type(0).is_err());
    let c = PolymarketClient::with_transport(Polymarket::mainnet(), peer.clone()).with_auth(auth);
    let mut session = c.open_rpc().await.unwrap();
    assert_eq!(
        peer.sent.lock().unwrap()[0]["identity"]["signature_type"],
        2
    );
    let ack = session
        .cancel_quote(json!({"rfq_id":"r1","quote_id":"q1"}))
        .await
        .unwrap();
    assert_eq!(ack["quote_id"], "q1");
    assert_eq!(
        session.next_event().await.unwrap().unwrap()["quote_id"],
        "unrelated"
    );
    assert!(session.pending().is_none());
    assert_eq!(peer.sent.lock().unwrap().len(), 2);
}
#[tokio::test]
async fn perps_auth_commands_unknown_outcome_and_mixed_batch_are_not_replayed() {
    let peer = Peer::default();
    let signer = signer();
    let auth = PolymarketPerpsAuth {
        owner: None,
        proxy: Some(signer.clone()),
        credentials: Some(
            PerpsCredentials::new(
                signer.address(),
                Zeroizing::new("public-secret".into()),
                None,
            )
            .unwrap(),
        ),
        withdrawal_contract: None,
    };
    let c =
        PolymarketPerpsClient::with_transport(Polymarket::mainnet(), peer.clone()).with_auth(auth);
    let mut session = c.open_rpc().await.unwrap();
    let orders = json!([{"iid":1,"buy":true,"p":"0.5","qty":"10","tif":"gtc","po":false}]);
    assert!(session.create_orders(orders.clone()).await.is_err());
    assert!(session.pending().is_some());
    assert!(matches!(
        session.create_orders(orders).await,
        Err(aux_rs_core::Error::Exchange(Error::PendingOutcome))
    ));
    assert_eq!(peer.sent.lock().unwrap().len(), 2);
    let ack = session.reconcile().await.unwrap();
    assert_eq!(ack["data"][1]["status"], "err");
    assert!(session.pending().is_none());
}
