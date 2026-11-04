use aux_rs_core::{HttpRequest, HttpResponse, HttpTransport, TransportError};
use aux_rs_pascal::*;
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;
use zeroize::Zeroizing;
#[derive(Clone, Default)]
struct Peer {
    sent: Arc<Mutex<Vec<Value>>>,
    started: Arc<Notify>,
}
impl HttpTransport for Peer {
    async fn execute(&self, r: HttpRequest) -> Result<HttpResponse, TransportError> {
        let first = {
            let mut sent = self.sent.lock().unwrap();
            sent.push(serde_json::from_slice(&r.body).unwrap());
            sent.len() == 1
        };
        if first {
            self.started.notify_one();
            std::future::pending::<()>().await;
        }
        let f: Value = serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();
        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: serde_json::to_vec(&f["write.register_a_deposit_address"]["response"]).unwrap(),
        })
    }
}
fn request(owner: &str) -> write::RegisterADepositAddressParams {
    serde_json::from_value(json!({"body":{"owner":owner}})).unwrap()
}
#[tokio::test]
async fn cancelled_wallet_submission_blocks_only_its_deployment_until_reconciled() {
    let peer = Peer::default();
    let signer = Arc::new(LocalSigner::new(Zeroizing::new([77; 32])));
    let owner = bs58::encode(signer.public_key()).into_string();
    let auth = PascalAuth::new(&owner, signer).unwrap();
    let client =
        PascalClient::with_transport(Pascal::canary(), peer.clone()).with_auth(auth.clone());
    let first = client.clone();
    let first_owner = owner.clone();
    let task = tokio::spawn(async move {
        first
            .register_a_deposit_address(request(&first_owner))
            .await
    });
    tokio::time::timeout(std::time::Duration::from_secs(2), peer.started.notified())
        .await
        .unwrap();
    assert!(matches!(
        client.register_a_deposit_address(request(&owner)).await,
        Err(aux_rs_core::Error::Exchange(Error::PendingOutcome))
    ));
    task.abort();
    assert!(matches!(task.await, Err(e) if e.is_cancelled()));
    assert!(auth.pending(2));
    let other =
        PascalClient::with_transport(Pascal::mainnet(), peer.clone()).with_auth(auth.clone());
    other
        .register_a_deposit_address(request(&owner))
        .await
        .unwrap();
    assert!(auth.pending(2));
    assert!(!auth.pending(3));
    auth.reconcile_wallet(2).unwrap();
    client
        .register_a_deposit_address(request(&owner))
        .await
        .unwrap();
    assert!(!auth.pending(2));
    let rows = peer.sent.lock().unwrap();
    let first = rows[0]["auth"]["client_ts_ms"]
        .as_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let last = rows[2]["auth"]["client_ts_ms"]
        .as_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    assert!(last > first);
}
