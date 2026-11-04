use aux_rs_core::{
    RequestOutcome, TransportError, TransportErrorKind, WebSocketConnection, WebSocketConnector,
};
use aux_rs_pascal::*;
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use zeroize::Zeroizing;
#[derive(Clone, Default)]
struct Peer {
    sent: Arc<Mutex<Vec<Value>>>,
    timeout: Arc<Mutex<bool>>,
}
impl WebSocketConnection for Peer {
    async fn send(&mut self, text: String) -> Result<(), TransportError> {
        self.sent
            .lock()
            .unwrap()
            .push(serde_json::from_str(&text).unwrap());
        Ok(())
    }
    async fn next(&mut self) -> Option<Result<String, TransportError>> {
        if !std::mem::replace(&mut *self.timeout.lock().unwrap(), true) {
            Some(Err(TransportError::new(
                TransportErrorKind::Timeout,
                RequestOutcome::Unknown,
                "fixture timeout",
            )))
        } else {
            Some(Ok(json!({"id":"1","type":"place","payload":[{"status":"success","data":{"order":{"id":"42"}}}]}).to_string()))
        }
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
#[tokio::test]
async fn uncertain_commands_are_not_replayed() {
    let peer = Peer::default();
    let auth = PascalAuth::new(
        "GmaDrppBC7P5ARKV8g3djiwP89vz1jLK23V2GBjuAEGB",
        Arc::new(LocalSigner::new(Zeroizing::new([8; 32]))),
    )
    .unwrap();
    let client = PascalClient::with_transport(Pascal::canary(), peer.clone()).with_auth(auth);
    let mut session = client.open_rpc().await.unwrap();
    let orders = json!([{"client_order_id":"42","symbol":"TEST","side":"BID","price":"0.550000","size":"10"}]);
    assert!(session.place_orders(orders.clone()).await.is_err());
    assert_eq!(session.pending().unwrap().id, 1);
    assert!(matches!(
        session.place_orders(orders).await,
        Err(aux_rs_core::Error::Exchange(Error::PendingOutcome))
    ));
    assert_eq!(peer.sent.lock().unwrap().len(), 1);
    assert_eq!(session.reconcile().await.unwrap()["id"], "1");
    assert!(session.pending().is_none());
    assert!(peer.sent.lock().unwrap()[0]["payload"][0]["auth"]["signature"].is_string());
}
