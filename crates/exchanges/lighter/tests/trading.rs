use aux_rs_core::{
    Error as ClientError, HttpRequest, HttpResponse, HttpTransport, RequestOutcome, TransportError,
    TransportErrorKind,
};
use aux_rs_lighter::LighterClient;
use aux_rs_lighter::*;
use serde_json::{Value, json};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicI64, AtomicUsize, Ordering},
};
use zeroize::Zeroizing;

#[derive(Default)]
struct State {
    nonces: Mutex<Vec<i64>>,
    mode: AtomicUsize,
    fetches: AtomicUsize,
    in_flight: AtomicUsize,
    entered: tokio::sync::Notify,
    release: tokio::sync::Notify,
}
#[derive(Clone, Default)]
struct Mock(Arc<State>);
fn response(value: Value) -> HttpResponse {
    HttpResponse {
        status: 200,
        headers: vec![],
        body: serde_json::to_vec(&value).unwrap(),
    }
}
fn unescape(value: &str) -> String {
    let mut out = Vec::new();
    let mut bytes = value.bytes();
    while let Some(byte) = bytes.next() {
        if byte == b'%' {
            let a = bytes.next().unwrap();
            let b = bytes.next().unwrap();
            out.push(u8::from_str_radix(std::str::from_utf8(&[a, b]).unwrap(), 16).unwrap());
        } else {
            out.push(byte)
        }
    }
    String::from_utf8(out).unwrap()
}
impl HttpTransport for Mock {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        if request.url.contains("nextNonce") {
            self.0.fetches.fetch_add(1, Ordering::SeqCst);
            return Ok(response(json!({"code":200,"nonce":42})));
        }
        if request.url.contains("accountActiveOrders") {
            let token = &request
                .headers
                .iter()
                .find(|(k, _)| k == "authorization")
                .unwrap()
                .1;
            assert_eq!(token.split(':').count(), 4);
            assert!(!format!("{request:?}").contains(token));
            return Ok(response(json!({"code":200,"orders":[]})));
        }
        assert!(request.url.ends_with("/api/v1/sendTx"));
        assert_eq!(request.headers[0].1, "application/x-www-form-urlencoded");
        let body = String::from_utf8(request.body).unwrap();
        let info: Value =
            serde_json::from_str(&unescape(body.split("&tx_info=").nth(1).unwrap())).unwrap();
        self.0
            .nonces
            .lock()
            .unwrap()
            .push(info["Nonce"].as_i64().unwrap());
        assert_eq!(
            self.0.in_flight.fetch_add(1, Ordering::SeqCst),
            0,
            "same API key submitted concurrently"
        );
        let mode = self.0.mode.load(Ordering::SeqCst);
        if mode == 3 {
            self.0.entered.notify_one();
            self.0.release.notified().await;
        } else {
            tokio::task::yield_now().await;
        }
        self.0.in_flight.fetch_sub(1, Ordering::SeqCst);
        match mode {
            1 => Err(TransportError::new(
                TransportErrorKind::Timeout,
                RequestOutcome::Unknown,
                "mock ambiguous submission",
            )),
            2 => Ok(response(json!({"code":21104,"message":"native rejection"}))),
            4 => Ok(HttpResponse {
                status: 200,
                headers: vec![],
                body: b"malformed".to_vec(),
            }),
            5 => Err(TransportError::new(
                TransportErrorKind::Connection,
                RequestOutcome::NotSent,
                "mock before send",
            )),
            _ => Ok(response(
                json!({"code":200,"tx_hash":"native-hash","predicted_execution_time_ms":1}),
            )),
        }
    }
}
fn signer() -> Arc<LocalSigner> {
    let mut key = [0; 40];
    key[0] = 1;
    Arc::new(LocalSigner::from_bytes(Zeroizing::new(key)).unwrap())
}
fn account() -> i64 {
    static NEXT: AtomicI64 = AtomicI64::new(1000000);
    NEXT.fetch_add(1, Ordering::SeqCst)
}
fn order() -> CancelOrder {
    CancelOrder {
        market_index: 4096,
        index: 123,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn independently_constructed_handlers_share_sequence_and_submission_lock() {
    let mock = Mock::default();
    let account = account();
    let signer = signer();
    let a = LighterClient::with_transport(Lighter::testnet(), mock.clone())
        .with_auth(LighterAuth::new(signer.clone(), 300, account, 3).unwrap());
    let b = LighterClient::with_transport(Lighter::testnet(), mock.clone())
        .with_auth(LighterAuth::new(signer, 300, account, 3).unwrap());
    let mut tasks = Vec::new();
    for i in 0..12 {
        let client = if i % 2 == 0 { a.clone() } else { b.clone() };
        tasks.push(tokio::spawn(async move {
            client
                .cancel_order(order(), TxOptions::new(1800000000000))
                .await
                .unwrap()
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }
    assert_eq!(*mock.0.nonces.lock().unwrap(), (42..54).collect::<Vec<_>>());
    assert_eq!(mock.0.fetches.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn outcomes_quarantine_until_explicit_resync_without_replay() {
    for mode in [1, 2, 4, 5] {
        let mock = Mock::default();
        mock.0.mode.store(mode, Ordering::SeqCst);
        let client = LighterClient::with_transport(Lighter::testnet(), mock.clone())
            .with_auth(LighterAuth::new(signer(), 300, account(), 3).unwrap());
        let first = client
            .cancel_order(order(), TxOptions::new(1800000000000))
            .await;
        assert!(first.is_err());
        if mode == 2 {
            assert!(matches!(
                first,
                Err(ClientError::Exchange(Error::Api { code: 21104, .. }))
            ));
        }
        assert!(matches!(
            client
                .cancel_order(order(), TxOptions::new(1800000000000))
                .await,
            Err(ClientError::Exchange(Error::NonceUncertain))
        ));
        assert_eq!(mock.0.nonces.lock().unwrap().len(), 1);
        assert_eq!(mock.0.fetches.load(Ordering::SeqCst), 1);
        assert_eq!(client.resync_nonce().await.unwrap(), 42);
        mock.0.mode.store(0, Ordering::SeqCst);
        client
            .cancel_order(order(), TxOptions::new(1800000000000))
            .await
            .unwrap();
        assert_eq!(*mock.0.nonces.lock().unwrap(), vec![42, 42]);
    }
}

#[tokio::test]
async fn cancellation_during_submission_retains_quarantine() {
    let mock = Mock::default();
    mock.0.mode.store(3, Ordering::SeqCst);
    let client = LighterClient::with_transport(Lighter::testnet(), mock.clone())
        .with_auth(LighterAuth::new(signer(), 300, account(), 3).unwrap());
    let cloned = client.clone();
    let task = tokio::spawn(async move {
        cloned
            .cancel_order(order(), TxOptions::new(1800000000000))
            .await
    });
    tokio::time::timeout(std::time::Duration::from_secs(3), mock.0.entered.notified())
        .await
        .unwrap();
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    assert!(matches!(
        client
            .cancel_order(order(), TxOptions::new(1800000000000))
            .await,
        Err(ClientError::Exchange(Error::NonceUncertain))
    ));
    assert_eq!(mock.0.nonces.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn chain_context_identity_scopes_and_authenticated_reads() {
    let mock = Mock::default();
    let account = account();
    let auth = LighterAuth::new(signer(), 300, account, 3).unwrap();
    let wrong =
        LighterClient::with_transport(Lighter::mainnet(), mock.clone()).with_auth(auth.clone());
    assert!(matches!(
        wrong
            .cancel_order(order(), TxOptions::new(1800000000000))
            .await,
        Err(ClientError::Exchange(Error::NetworkMismatch))
    ));
    assert_eq!(mock.0.fetches.load(Ordering::SeqCst), 0);
    let first = LighterClient::with_transport(Lighter::testnet(), mock.clone()).with_auth(auth);
    assert!(
        first
            .active_orders(Some(4096))
            .await
            .unwrap()
            .orders
            .is_empty()
    );
    first
        .cancel_order(order(), TxOptions::new(1800000000000))
        .await
        .unwrap();
    for (chain, key, acct) in [
        (300, 4, account),
        (304, 3, account),
        (300, 3, account + 1000000),
    ] {
        let network = if chain == 300 {
            Lighter::testnet()
        } else {
            Lighter::mainnet()
        };
        let p = LighterClient::with_transport(network, mock.clone())
            .with_auth(LighterAuth::new(signer(), chain, acct, key).unwrap());
        p.cancel_order(order(), TxOptions::new(1800000000000))
            .await
            .unwrap();
    }
    assert_eq!(*mock.0.nonces.lock().unwrap(), vec![42; 4]);
    assert_eq!(mock.0.fetches.load(Ordering::SeqCst), 4);
}

struct WrongSigner {
    key: PublicKey,
}
impl Signer for WrongSigner {
    fn public_key(&self) -> PublicKey {
        self.key
    }
    async fn sign(&self, _: &SigningPayload) -> Result<Signature, Error> {
        Signature::from_bytes([0; 80])
    }
}
#[tokio::test]
async fn validation_and_bad_custom_signer_never_submit_or_consume_nonce() {
    let mock = Mock::default();
    let account = account();
    let auth = LighterAuth::new(signer(), 300, account, 3).unwrap();
    let client = LighterClient::with_transport(Lighter::testnet(), mock.clone()).with_auth(auth);
    assert!(
        client
            .cancel_order(
                CancelOrder {
                    market_index: 255,
                    index: 1
                },
                TxOptions::new(1800000000000)
            )
            .await
            .is_err()
    );
    assert_eq!(mock.0.fetches.load(Ordering::SeqCst), 0);
    let bad = LighterClient::with_transport(Lighter::testnet(), mock.clone()).with_auth(
        LighterAuth::new(
            WrongSigner {
                key: signer().public_key(),
            },
            300,
            account,
            3,
        )
        .unwrap(),
    );
    assert!(matches!(
        bad.cancel_order(order(), TxOptions::new(1800000000000))
            .await,
        Err(ClientError::Exchange(Error::InvalidSignature))
    ));
    assert!(mock.0.nonces.lock().unwrap().is_empty());
    client
        .cancel_order(order(), TxOptions::new(1800000000000))
        .await
        .unwrap();
    assert_eq!(*mock.0.nonces.lock().unwrap(), vec![42]);
}
