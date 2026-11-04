use aux_rs_core::{
    HttpRequest, HttpResponse, HttpTransport, RequestOutcome, TransportError, TransportErrorKind,
};
use aux_rs_pascal::*;
use serde_json::{Value, json};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use zeroize::Zeroizing;
#[derive(Clone, Default)]
struct Peer {
    height: Arc<AtomicU64>,
    sends: Arc<AtomicU64>,
    confirmed: Arc<AtomicU64>,
}
impl HttpTransport for Peer {
    async fn execute(&self, r: HttpRequest) -> Result<HttpResponse, TransportError> {
        let b: Value = serde_json::from_slice(&r.body).unwrap();
        let value = match b["method"].as_str().unwrap() {
            "getBlockHeight" => json!(self.height.load(Ordering::SeqCst)),
            "getSignatureStatuses" => {
                json!({"value":[{"err":null,"confirmationStatus":if self.confirmed.load(Ordering::SeqCst)>0{"confirmed"}else{"processed"}}]})
            }
            "sendTransaction" => {
                self.sends.fetch_add(1, Ordering::SeqCst);
                return Err(TransportError::new(
                    TransportErrorKind::Timeout,
                    RequestOutcome::Unknown,
                    "fixture timeout",
                ));
            }
            _ => panic!("unexpected RPC"),
        };
        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: serde_json::to_vec(&json!({"jsonrpc":"2.0","id":b["id"],"result":value}))
                .unwrap(),
        })
    }
}
fn prepared() -> PreparedDeposit {
    let row: Value = serde_json::from_str(include_str!("fixtures/solana.json")).unwrap();
    let s = |k: &str| row[k].as_str().unwrap();
    PreparedDeposit::new(
        s("owner"),
        s("source"),
        s("destination"),
        s("mint"),
        25_000_000,
        6,
        s("blockhash"),
        999,
    )
    .unwrap()
}
#[tokio::test]
async fn expiry_unknown_submission_confirmation_and_explicit_reconciliation() {
    let peer = Peer::default();
    let p = prepared();
    let signed = p
        .sign(&LocalSigner::new(Zeroizing::new([7; 32])))
        .await
        .unwrap();
    let config =
        SolanaConfig::new("http://127.0.0.1:8899?api-key=public-test", &p.mint, 6).unwrap();
    let c = PascalChainClient::with_transport(config, peer.clone());
    peer.height.store(1000, Ordering::SeqCst);
    assert!(c.submit(&signed).await.is_err());
    assert_eq!(peer.sends.load(Ordering::SeqCst), 0);
    peer.height.store(10, Ordering::SeqCst);
    assert!(c.submit(&signed).await.is_err());
    assert_eq!(c.pending(), std::slice::from_ref(&signed.signature));
    assert!(matches!(
        c.submit(&signed).await,
        Err(aux_rs_core::Error::Exchange(Error::PendingOutcome))
    ));
    assert_eq!(peer.sends.load(Ordering::SeqCst), 1);
    c.reconcile(&signed.signature).await.unwrap();
    assert_eq!(c.pending().len(), 1, "processed is not confirmed");
    assert!(c.clear_reconciled_submission("unrelated").is_err());
    peer.confirmed.store(1, Ordering::SeqCst);
    c.reconcile(&signed.signature).await.unwrap();
    assert!(c.pending().is_empty());
}
#[tokio::test]
async fn altered_deposit_metadata_and_wrong_authority_are_rejected() {
    let mut p = prepared();
    let signer = LocalSigner::new(Zeroizing::new([7; 32]));
    assert!(
        p.sign(&LocalSigner::new(Zeroizing::new([8; 32])))
            .await
            .is_err()
    );
    p.amount += 1;
    assert!(p.sign(&signer).await.is_err());
    p.amount -= 1;
    p.message[0] ^= 1;
    assert!(p.sign(&signer).await.is_err());
}
