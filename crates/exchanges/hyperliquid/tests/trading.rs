use aux_rs_core::{
    Error as ClientError, HttpRequest, HttpResponse, HttpTransport, RequestOutcome, TransportError,
    TransportErrorKind,
};
use aux_rs_hyperliquid::HyperliquidClient;
use aux_rs_hyperliquid::{
    Action, Address, Cancel, CancelByCloid, CancelByCloidRequest, CancelRequest, Error,
    ExchangeResponse, ExecutionOptions, Hyperliquid, HyperliquidAuth, ItemStatus, LocalSigner,
    Network, NonceManager, Order, OrderRequest, Signature, Signer, SignerError, SigningPayload,
    Tif, signing::l1_signing_payload,
};
use serde_json::{Value, json};
use std::{
    collections::{HashSet, VecDeque},
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

const TEST_KEY: &str = "0x0123456789012345678901234567890123456789012345678901234567890123";
const ACCEPTED: &str =
    r#"{"status":"ok","response":{"type":"order","data":{"statuses":[{"resting":{"oid":42}}]}}}"#;

#[derive(Clone)]
struct MockHttp {
    requests: Arc<Mutex<Vec<HttpRequest>>>,
    responses: Arc<Mutex<VecDeque<Result<String, TransportError>>>>,
}
impl MockHttp {
    fn new(responses: Vec<Result<String, TransportError>>) -> Self {
        Self {
            requests: Arc::default(),
            responses: Arc::new(Mutex::new(responses.into())),
        }
    }
}
impl HttpTransport for MockHttp {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        self.requests.lock().unwrap().push(request);
        let body = self
            .responses
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected HTTP execution")?;
        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: body.into_bytes(),
        })
    }
}
fn auth() -> HyperliquidAuth<LocalSigner> {
    HyperliquidAuth::agent(TEST_KEY.parse().unwrap(), Address::from_bytes([2; 20]))
}
fn order() -> Order {
    Order::limit(4, true, "1670.1", "0.0147", Tif::Ioc)
}
fn request() -> OrderRequest {
    OrderRequest::new(vec![order()])
}

#[tokio::test]
async fn native_order_cancel_and_cloid_envelopes_preserve_batch_rejections() {
    let transport = MockHttp::new(vec![
        Ok(r#"{"status":"ok","response":{"type":"order","data":{"statuses":[{"resting":{"oid":1}},{"filled":{"totalSz":"0.0147","avgPx":"1670.1","oid":2}},{"error":"Insufficient margin"}]}}}"#.into()),
        Ok(r#"{"status":"ok","response":{"type":"cancel","data":{"statuses":["success",{"error":"Order not found"}]}}}"#.into()),
        Ok(r#"{"status":"err","response":"native batch rejection"}"#.into()),
    ]);
    let vault = Address::from_bytes([3; 20]);
    let client = HyperliquidClient::with_transport(Hyperliquid::testnet(), transport.clone())
        .with_auth(auth().with_vault(vault));
    assert_eq!(client.auth().account_address(), vault);
    assert_ne!(
        client.auth().account_address(),
        client.auth().signer_address()
    );
    let result = client
        .order_with_options(
            OrderRequest::new(vec![order(), order(), order()]),
            ExecutionOptions {
                nonce: Some(1000),
                expires_after: Some(2000),
            },
        )
        .await
        .unwrap();
    let ExchangeResponse::Ok(response) = result else {
        panic!("expected native order result");
    };
    assert_eq!(response.data.statuses.len(), 3);
    assert!(
        matches!(&response.data.statuses[0], ItemStatus::Resting { resting } if resting.oid == 1)
    );
    assert!(
        matches!(&response.data.statuses[1], ItemStatus::Filled { filled } if filled.total_sz == "0.0147")
    );
    assert!(
        matches!(&response.data.statuses[2], ItemStatus::Error { error } if error == "Insufficient margin")
    );
    let cancelled = client
        .cancel_with_options(
            CancelRequest {
                cancels: vec![Cancel { asset: 4, oid: 1 }, Cancel { asset: 4, oid: 2 }],
            },
            ExecutionOptions {
                nonce: Some(1001),
                expires_after: None,
            },
        )
        .await
        .unwrap();
    assert!(
        matches!(cancelled, ExchangeResponse::Ok(response) if matches!(&response.data.statuses[0], ItemStatus::Text(text) if text == "success"))
    );
    let cloid = "0x00000000000000000000000000000001".parse().unwrap();
    assert!(
        matches!(client.cancel_by_cloid_with_options(CancelByCloidRequest { cancels: vec![CancelByCloid { asset: 4, cloid }] },
        ExecutionOptions { nonce: Some(1002), expires_after: None }).await.unwrap(), ExchangeResponse::Err(error) if error == "native batch rejection")
    );
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests.len(), 3);
    for wire in requests.iter() {
        assert_eq!(wire.url, "https://api.hyperliquid-testnet.xyz/exchange");
        let body: Value = serde_json::from_slice(&wire.body).unwrap();
        let action: Action = serde_json::from_value(body["action"].clone()).unwrap();
        let signature: Signature = serde_json::from_value(body["signature"].clone()).unwrap();
        let payload = l1_signing_payload(
            &action,
            Some(vault),
            body["nonce"].as_u64().unwrap(),
            body["expiresAfter"].as_u64(),
            Network::Testnet,
        )
        .unwrap();
        assert_eq!(
            signature.recover_address(&payload).unwrap(),
            client.auth().signer_address()
        );
        assert_eq!(body["vaultAddress"], vault.to_string());
        assert!(!format!("{wire:?}").contains("signature"));
    }
    let cancel: Value = serde_json::from_slice(&requests[1].body).unwrap();
    assert_eq!(
        cancel["action"],
        json!({"type":"cancel","cancels":[{"a":4,"o":1},{"a":4,"o":2}]})
    );
    let cancel: Value = serde_json::from_slice(&requests[2].body).unwrap();
    assert_eq!(
        cancel["action"],
        json!({"type":"cancelByCloid","cancels":[{"asset":4,"cloid":cloid}]})
    );
}

struct RecordingSigner {
    local: LocalSigner,
    calls: Arc<AtomicUsize>,
    fail: bool,
    spoof_address: bool,
}
impl Signer for RecordingSigner {
    fn address(&self) -> Address {
        if self.spoof_address {
            Address::from_bytes([1; 20])
        } else {
            self.local.address()
        }
    }
    async fn sign(&self, payload: SigningPayload) -> Result<Signature, SignerError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.fail {
            Err(SignerError::Unavailable)
        } else {
            self.local.sign(payload).await
        }
    }
}

#[tokio::test]
async fn validation_and_signing_failures_never_reach_transport() {
    let calls = Arc::new(AtomicUsize::new(0));
    let transport = MockHttp::new(vec![]);
    let signer = RecordingSigner {
        local: TEST_KEY.parse().unwrap(),
        calls: calls.clone(),
        fail: false,
        spoof_address: false,
    };
    let client = HyperliquidClient::with_transport(Hyperliquid::testnet(), transport.clone())
        .with_auth(HyperliquidAuth::direct(signer));
    for price in ["1.00", "NaN", "-1", "0", "01", "1e3", "1.", ".1"] {
        let mut invalid = order();
        invalid.limit_px = price.into();
        assert!(matches!(
            client.order(OrderRequest::new(vec![invalid])).await,
            Err(ClientError::Exchange(Error::InvalidRequest(_)))
        ));
    }
    assert!(client.order(OrderRequest::new(vec![])).await.is_err());
    assert!(
        client
            .cancel(CancelRequest { cancels: vec![] })
            .await
            .is_err()
    );
    assert!(
        client
            .order_with_options(
                request(),
                ExecutionOptions {
                    nonce: Some(10),
                    expires_after: Some(9)
                }
            )
            .await
            .is_err()
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert!(transport.requests.lock().unwrap().is_empty());

    for (fail, spoof_address) in [(true, false), (false, true)] {
        let signer = RecordingSigner {
            local: TEST_KEY.parse().unwrap(),
            calls: calls.clone(),
            fail,
            spoof_address,
        };
        let client = HyperliquidClient::with_transport(Hyperliquid::testnet(), transport.clone())
            .with_auth(HyperliquidAuth::direct(signer));
        assert!(matches!(
            client.order(request()).await,
            Err(ClientError::Exchange(Error::Signing(_)))
        ));
    }
    assert!(transport.requests.lock().unwrap().is_empty());
    let client = HyperliquidClient::with_transport(
        Hyperliquid::custom("http://proxy", "ws://proxy"),
        transport.clone(),
    )
    .with_auth(auth());
    assert!(matches!(
        client.order(request()).await,
        Err(ClientError::Exchange(Error::InvalidRequest(_)))
    ));
}

#[tokio::test]
async fn ambiguous_submissions_are_not_retried_and_nonces_are_not_reused() {
    let error = TransportError::new(
        TransportErrorKind::Timeout,
        RequestOutcome::Unknown,
        "simulated timeout after sending",
    );
    let transport = MockHttp::new(vec![Err(error.clone()), Err(error)]);
    let client = HyperliquidClient::with_transport(Hyperliquid::testnet(), transport.clone())
        .with_auth(auth());
    for _ in 0..2 {
        assert!(
            matches!(client.order(request()).await, Err(ClientError::Transport(error)) if error.outcome == RequestOutcome::Unknown)
        );
    }
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    let nonces: Vec<_> = requests
        .iter()
        .map(|r| {
            serde_json::from_slice::<Value>(&r.body).unwrap()["nonce"]
                .as_u64()
                .unwrap()
        })
        .collect();
    assert!(nonces[1] > nonces[0]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn independent_auth_clients_and_vault_clones_share_signer_nonces() {
    let transport = MockHttp::new((0..32).map(|_| Ok(ACCEPTED.into())).collect());
    let first = HyperliquidClient::with_transport(Hyperliquid::testnet(), transport.clone())
        .with_auth(auth());
    let second = HyperliquidClient::with_transport(Hyperliquid::testnet(), transport.clone())
        .with_auth(auth().with_vault(Address::from_bytes([7; 20])));
    let mut tasks = Vec::new();
    for index in 0..32 {
        let client = if index % 2 == 0 {
            first.clone()
        } else {
            second.clone()
        };
        tasks.push(tokio::spawn(async move {
            client.order(request()).await.unwrap()
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests.len(), 32);
    let nonces: HashSet<_> = requests
        .iter()
        .map(|r| {
            serde_json::from_slice::<Value>(&r.body).unwrap()["nonce"]
                .as_u64()
                .unwrap()
        })
        .collect();
    assert_eq!(nonces.len(), 32);
}

#[tokio::test]
async fn explicit_nonces_bypass_allocation_and_signed_envelope_debug_is_redacted() {
    let auth = auth();
    for _ in 0..2 {
        let signed = auth
            .prepare(
                Action::Order(request()),
                Network::Testnet,
                ExecutionOptions {
                    nonce: Some(42),
                    expires_after: None,
                },
            )
            .await
            .unwrap();
        assert_eq!(signed.nonce(), 42);
        let debug = format!("{signed:?} {auth:?}");
        assert!(!debug.contains(&signed.signature().r));
        assert!(!debug.contains(&TEST_KEY[2..]));
    }
}

#[test]
fn nonce_clock_rollback_reconstruction_and_exhaustion() {
    let address = Address::from_bytes([11; 20]);
    let manager = NonceManager::shared_for(address);
    assert_eq!(manager.next_at(100).unwrap(), 100);
    assert_eq!(manager.next_at(90).unwrap(), 101);
    drop(manager);
    assert_eq!(NonceManager::shared_for(address).next_at(100).unwrap(), 102);
    let address = Address::from_bytes([12; 20]);
    let handles: Vec<_> = (0..16)
        .map(|_| {
            std::thread::spawn(move || {
                let manager = NonceManager::shared_for(address);
                (0..64)
                    .map(|_| manager.next_at(1000).unwrap())
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    let nonces: HashSet<_> = handles
        .into_iter()
        .flat_map(|handle| handle.join().unwrap())
        .collect();
    assert_eq!(nonces.len(), 16 * 64);
    let manager = NonceManager::shared_for(Address::from_bytes([13; 20]));
    assert_eq!(manager.next_at(u64::MAX).unwrap(), u64::MAX);
    assert!(matches!(manager.next_at(0), Err(Error::NonceExhausted)));
}

#[test]
fn market_precision_is_checked_without_rounding() {
    Order::limit(1, true, "1234.5", "0.1234", Tif::Gtc)
        .validate_precision(4, false)
        .unwrap();
    assert!(
        Order::limit(1, true, "1234.56", "0.1234", Tif::Gtc)
            .validate_precision(4, false)
            .is_err()
    );
    assert!(
        Order::limit(1, true, "1234.5", "0.12345", Tif::Gtc)
            .validate_precision(4, false)
            .is_err()
    );
    Order::limit(1, true, "1234567", "1", Tif::Gtc)
        .validate_precision(4, false)
        .unwrap();
    assert!(
        Order::limit(1, true, "1.2345", "1", Tif::Gtc)
            .validate_precision(4, false)
            .is_err()
    );
    Order::limit(10000, true, "1.2345", "1", Tif::Gtc)
        .validate_precision(4, true)
        .unwrap();
}

struct CustomAuth {
    inner: HyperliquidAuth<LocalSigner>,
    calls: Arc<AtomicUsize>,
    network_override: Option<Network>,
    signer_override: Option<Address>,
}
impl aux_rs_hyperliquid::AuthHandler for CustomAuth {
    fn account_address(&self) -> Address {
        self.inner.account_address()
    }
    fn signer_address(&self) -> Address {
        self.signer_override
            .unwrap_or_else(|| self.inner.signer_address())
    }
    async fn prepare(
        &self,
        action: Action,
        network: Network,
        options: ExecutionOptions,
    ) -> Result<aux_rs_hyperliquid::SignedRequest, Error> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.inner
            .prepare(action, self.network_override.unwrap_or(network), options)
            .await
    }
}

#[tokio::test]
async fn custom_auth_handlers_use_the_same_native_trading_extensions() {
    let calls = Arc::new(AtomicUsize::new(0));
    let transport = MockHttp::new(vec![Ok(ACCEPTED.into())]);
    let client = HyperliquidClient::with_transport(Hyperliquid::testnet(), transport.clone())
        .with_auth(CustomAuth {
            inner: auth(),
            calls: calls.clone(),
            network_override: None,
            signer_override: None,
        });
    client.order(request()).await.unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(transport.requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn custom_handler_network_and_signer_context_is_checked_before_submission() {
    for (network_override, signer_override) in [
        (Some(Network::Mainnet), None),
        (None, Some(Address::from_bytes([99; 20]))),
    ] {
        let transport = MockHttp::new(vec![]);
        let client = HyperliquidClient::with_transport(Hyperliquid::testnet(), transport.clone())
            .with_auth(CustomAuth {
                inner: auth(),
                calls: Arc::default(),
                network_override,
                signer_override,
            });
        assert!(matches!(
            client.order(request()).await,
            Err(ClientError::Exchange(
                Error::InvalidRequest(_) | Error::Signing(SignerError::AddressMismatch)
            ))
        ));
        assert!(transport.requests.lock().unwrap().is_empty());
    }
}
