use aux_rs_core::{
    Error as ClientError, HttpRequest, HttpResponse, HttpTransport, RequestOutcome, TransportError,
    TransportErrorKind,
};
use aux_rs_metric as m;
use m::{EvmSigner, U256};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use zeroize::Zeroizing;
#[derive(Clone)]
struct Rpc {
    state: Arc<Mutex<State>>,
    arrived: Arc<tokio::sync::Notify>,
    resume: Arc<tokio::sync::Notify>,
}
struct State {
    chain: u64,
    sends: Vec<Vec<u8>>,
    safe: Vec<bool>,
    fail: bool,
    block: bool,
    seen: bool,
    wrong_hash: bool,
}
impl Rpc {
    fn new(chain: u64) -> Self {
        Self {
            state: Arc::new(Mutex::new(State {
                chain,
                sends: vec![],
                safe: vec![],
                fail: false,
                block: false,
                seen: false,
                wrong_hash: false,
            })),
            arrived: Default::default(),
            resume: Default::default(),
        }
    }
}
impl Rpc {
    async fn run(&self, request: HttpRequest, safe: bool) -> Result<HttpResponse, TransportError> {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        let method = body["method"].as_str().unwrap();
        let (result, fail, block) = {
            let mut s = self.state.lock().unwrap();
            let value = match method {
                "eth_chainId" => json!(format!("0x{:x}", s.chain)),
                "eth_getCode" => json!("0x6000"),
                "eth_getTransactionCount" => json!("0x0"),
                "eth_sendRawTransaction" => {
                    let raw: m::Bytes = body["params"][0].as_str().unwrap().parse().unwrap();
                    s.sends.push(raw.to_vec());
                    s.safe.push(safe);
                    if s.wrong_hash {
                        json!(m::B256::ZERO)
                    } else {
                        json!(alloy_primitives::keccak256(&raw))
                    }
                }
                "eth_getTransactionByHash" => {
                    if s.seen {
                        json!({"hash":body["params"][0],"from":signer().address(),"nonce":"0x0"})
                    } else {
                        Value::Null
                    }
                }
                "eth_getTransactionReceipt" => Value::Null,
                _ => panic!("unexpected RPC {method}"),
            };
            (
                value,
                s.fail && method == "eth_sendRawTransaction",
                s.block && method == "eth_sendRawTransaction",
            )
        };
        if block {
            self.arrived.notify_one();
            self.resume.notified().await;
        }
        if fail {
            return Err(TransportError::new(
                TransportErrorKind::Timeout,
                RequestOutcome::Unknown,
                "test send outcome unknown",
            ));
        }
        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: serde_json::to_vec(&json!({"jsonrpc":"2.0","id":body["id"],"result":result}))
                .unwrap(),
        })
    }
}
impl HttpTransport for Rpc {
    async fn execute(&self, r: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.run(r, false).await
    }
    async fn execute_read(&self, r: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.run(r, true).await
    }
}
fn signer() -> m::LocalEvmSigner {
    m::LocalEvmSigner::from_hex(Zeroizing::new(format!("{:064x}", 1))).unwrap()
}
fn prepared(chain: u64) -> m::PreparedTransaction {
    m::PreparedTransaction {
        chain_id: chain,
        to: "0x1111111111111111111111111111111111111111"
            .parse()
            .unwrap(),
        value: U256::ZERO,
        data: "0x1234".parse().unwrap(),
        nonce: 99,
        gas_limit: 150000,
        gas_price: None,
        max_fee_per_gas: Some(20_000_000_000),
        max_priority_fee_per_gas: Some(1_000_000_000),
    }
}
fn client(rpc: &Rpc, chain: u64) -> m::MetricChainClient<Rpc> {
    m::MetricChainClient::with_transport(
        m::ChainConfig::new(chain, "http://localhost/read", "http://localhost/write").unwrap(),
        rpc.clone(),
    )
}
#[tokio::test]
async fn independent_clients_share_nonces_even_when_rpc_pending_count_is_stale() {
    use alloy_consensus::Transaction;
    use alloy_eips::eip2718::Decodable2718;
    let rpc = Rpc::new(777001);
    let a = client(&rpc, 777001);
    let b = client(&rpc, 777001);
    a.send_transaction(prepared(777001), &signer())
        .await
        .unwrap();
    b.send_transaction(prepared(777001), &signer())
        .await
        .unwrap();
    let s = rpc.state.lock().unwrap();
    let nonces: Vec<_> = s
        .sends
        .iter()
        .map(|b| {
            alloy_consensus::TxEnvelope::decode_2718(&mut b.as_slice())
                .unwrap()
                .nonce()
        })
        .collect();
    assert_eq!(nonces, [0, 1]);
    assert_eq!(s.safe, [false, false]);
}
#[tokio::test]
async fn timeouts_quarantine_until_matching_native_reconciliation() {
    let rpc = Rpc::new(777002);
    rpc.state.lock().unwrap().fail = true;
    let c = client(&rpc, 777002);
    assert!(
        c.send_transaction(prepared(777002), &signer())
            .await
            .is_err()
    );
    let pending = c.pending(signer().address()).unwrap();
    assert!(!c.reconcile(signer().address()).await.unwrap());
    assert!(matches!(
        c.send_transaction(prepared(777002), &signer()).await,
        Err(ClientError::Exchange(m::Error::PendingOutcome))
    ));
    assert_eq!(rpc.state.lock().unwrap().sends.len(), 1);
    rpc.state.lock().unwrap().seen = true;
    assert!(c.reconcile(signer().address()).await.unwrap());
    assert!(c.pending(signer().address()).is_none());
    assert!(c.receipt(pending.hash).await.unwrap().is_none());
}
#[tokio::test]
async fn cancellation_after_transmission_preserves_pending_hash_and_never_replays() {
    let rpc = Rpc::new(777003);
    rpc.state.lock().unwrap().block = true;
    let c = client(&rpc, 777003);
    let worker = c.clone();
    let task =
        tokio::spawn(async move { worker.send_transaction(prepared(777003), &signer()).await });
    rpc.arrived.notified().await;
    task.abort();
    let _ = task.await;
    assert!(c.pending(signer().address()).is_some());
    assert!(
        c.send_transaction(prepared(777003), &signer())
            .await
            .is_err()
    );
    assert_eq!(rpc.state.lock().unwrap().sends.len(), 1);
    let hash = c.pending(signer().address()).unwrap().hash;
    assert!(
        c.resync_nonce(signer().address(), m::B256::ZERO)
            .await
            .is_err()
    );
    assert_eq!(c.resync_nonce(signer().address(), hash).await.unwrap(), 0);
    assert!(c.pending(signer().address()).is_none());
}
#[tokio::test]
async fn wrong_chain_code_identity_and_hash_fail_before_another_write() {
    let rpc = Rpc::new(777004);
    let c = client(&rpc, 777005);
    assert!(
        c.send_transaction(prepared(777005), &signer())
            .await
            .is_err()
    );
    assert!(rpc.state.lock().unwrap().sends.is_empty());
    let config = m::ChainConfig::new(777004, "http://localhost/read", "http://localhost/write")
        .unwrap()
        .with_code_hash(prepared(777004).to, m::B256::ZERO);
    let c = m::MetricChainClient::with_transport(config, rpc.clone());
    assert!(
        c.send_transaction(prepared(777004), &signer())
            .await
            .is_err()
    );
    assert!(rpc.state.lock().unwrap().sends.is_empty());
    let c = client(&rpc, 777004);
    rpc.state.lock().unwrap().wrong_hash = true;
    assert!(
        c.send_transaction(prepared(777004), &signer())
            .await
            .is_err()
    );
    assert!(c.pending(signer().address()).is_some());
    assert_eq!(rpc.state.lock().unwrap().sends.len(), 1);
}
#[tokio::test]
async fn unresolved_identity_survives_dropping_all_clients() {
    let rpc = Rpc::new(777006);
    rpc.state.lock().unwrap().fail = true;
    let c = client(&rpc, 777006);
    assert!(
        c.send_transaction(prepared(777006), &signer())
            .await
            .is_err()
    );
    let pending = c.pending(signer().address()).unwrap();
    drop(c);
    let c = client(&rpc, 777006);
    assert_eq!(c.pending(signer().address()).unwrap().hash, pending.hash);
    assert!(
        c.send_transaction(prepared(777006), &signer())
            .await
            .is_err()
    );
    assert_eq!(rpc.state.lock().unwrap().sends.len(), 1);
}
