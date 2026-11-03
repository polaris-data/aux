use crate::{Error, client::validate_url};
use alloy_consensus::{
    SignableTransaction, Transaction, TxEip1559, TxEnvelope, TxLegacy,
    transaction::SignerRecoverable,
};
use alloy_eips::eip2718::{Decodable2718, Encodable2718};
use alloy_primitives::{Address, B256, Bytes, Signature, TxKind, U256};
use alloy_sol_types::SolCall;
use aux_rs_core::{DefaultTransport, Error as ClientError, HttpMethod, HttpRequest, HttpTransport};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    future::Future,
    sync::atomic::{AtomicU64, Ordering},
    sync::{Arc, Mutex, OnceLock},
};
use zeroize::Zeroizing;

#[derive(Clone)]
pub struct ChainConfig {
    pub chain_id: u64,
    pub read_rpc: String,
    pub write_rpc: String,
    pub code_hashes: std::collections::BTreeMap<Address, B256>,
}
impl std::fmt::Debug for ChainConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChainConfig")
            .field("chain_id", &self.chain_id)
            .field("pinned_contracts", &self.code_hashes.len())
            .finish_non_exhaustive()
    }
}
impl ChainConfig {
    pub fn new(
        chain_id: u64,
        read_rpc: impl Into<String>,
        write_rpc: impl Into<String>,
    ) -> Result<Self, Error> {
        let read_rpc = read_rpc.into();
        let write_rpc = write_rpc.into();
        for rpc in [&read_rpc, &write_rpc] {
            let mut url =
                url::Url::parse(rpc).map_err(|_| Error::InvalidRequest("invalid RPC endpoint"))?;
            url.set_query(None);
            validate_url(url.as_str())?;
        }
        if chain_id == 0 {
            return Err(Error::InvalidRequest("chain_id must be positive"));
        }
        Ok(Self {
            chain_id,
            read_rpc,
            write_rpc,
            code_hashes: Default::default(),
        })
    }
    pub fn with_code_hash(mut self, address: Address, hash: B256) -> Self {
        self.code_hashes.insert(address, hash);
        self
    }
}
pub trait EvmSigner: Send + Sync {
    fn address(&self) -> Address;
    fn sign_digest(&self, digest: B256) -> impl Future<Output = Result<Signature, Error>> + Send;
}
impl<S: EvmSigner + ?Sized> EvmSigner for Arc<S> {
    fn address(&self) -> Address {
        (**self).address()
    }
    async fn sign_digest(&self, digest: B256) -> Result<Signature, Error> {
        (**self).sign_digest(digest).await
    }
}
pub struct LocalEvmSigner {
    key: k256::ecdsa::SigningKey,
    address: Address,
}
impl std::fmt::Debug for LocalEvmSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalEvmSigner")
            .field("address", &self.address)
            .finish_non_exhaustive()
    }
}
impl LocalEvmSigner {
    pub fn from_hex(secret: Zeroizing<String>) -> Result<Self, Error> {
        let raw = Zeroizing::new(
            alloy_primitives::hex::decode(secret.as_str())
                .map_err(|_| Error::Signing("invalid private key"))?,
        );
        let key = k256::ecdsa::SigningKey::from_slice(&raw)
            .map_err(|_| Error::Signing("invalid private key"))?;
        let address = Address::from_public_key(key.verifying_key());
        Ok(Self { key, address })
    }
}
impl EvmSigner for LocalEvmSigner {
    fn address(&self) -> Address {
        self.address
    }
    async fn sign_digest(&self, digest: B256) -> Result<Signature, Error> {
        let (sig, rec) = self
            .key
            .sign_prehash_recoverable(digest.as_slice())
            .map_err(|_| Error::Signing("digest signing failed"))?;
        Ok(Signature::from_signature_and_parity(sig, rec.is_y_odd()))
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PreparedTransaction {
    pub chain_id: u64,
    pub to: Address,
    #[serde(default)]
    pub value: U256,
    pub data: Bytes,
    pub nonce: u64,
    pub gas_limit: u64,
    /// Supplying gas_price selects a legacy EIP-155 transaction.
    pub gas_price: Option<u128>,
    pub max_fee_per_gas: Option<u128>,
    pub max_priority_fee_per_gas: Option<u128>,
}
impl PreparedTransaction {
    pub fn validate(&self) -> Result<(), Error> {
        if self.chain_id == 0 || self.gas_limit == 0 || self.nonce == u64::MAX {
            return Err(Error::InvalidRequest(
                "positive chain and gas limit required",
            ));
        }
        match (
            self.gas_price,
            self.max_fee_per_gas,
            self.max_priority_fee_per_gas,
        ) {
            (Some(_), None, None) => Ok(()),
            (None, Some(max), Some(tip)) if tip <= max => Ok(()),
            _ => Err(Error::InvalidRequest(
                "supply legacy gasPrice or EIP-1559 maxFeePerGas and maxPriorityFeePerGas",
            )),
        }
    }
    fn legacy(&self) -> TxLegacy {
        TxLegacy {
            chain_id: Some(self.chain_id),
            nonce: self.nonce,
            gas_price: self.gas_price.unwrap_or_default(),
            gas_limit: self.gas_limit,
            to: TxKind::Call(self.to),
            value: self.value,
            input: self.data.clone(),
        }
    }
    fn dynamic(&self) -> TxEip1559 {
        TxEip1559 {
            chain_id: self.chain_id,
            nonce: self.nonce,
            gas_limit: self.gas_limit,
            max_fee_per_gas: self.max_fee_per_gas.unwrap_or_default(),
            max_priority_fee_per_gas: self.max_priority_fee_per_gas.unwrap_or_default(),
            to: TxKind::Call(self.to),
            value: self.value,
            input: self.data.clone(),
            access_list: Default::default(),
        }
    }
    pub fn signing_hash(&self) -> Result<B256, Error> {
        self.validate()?;
        Ok(if self.gas_price.is_some() {
            self.legacy().signature_hash()
        } else {
            self.dynamic().signature_hash()
        })
    }
    pub async fn sign<S: EvmSigner>(&self, signer: &S) -> Result<SignedTransaction, Error> {
        let hash = self.signing_hash()?;
        let signature = signer.sign_digest(hash).await?;
        if signature.normalize_s().is_some() {
            return Err(Error::Signing("noncanonical high-S external signature"));
        }
        if signature
            .recover_address_from_prehash(&hash)
            .map_err(|_| Error::Signing("invalid external signature"))?
            != signer.address()
        {
            return Err(Error::Signing("signature does not match signer"));
        }
        let envelope: TxEnvelope = if self.gas_price.is_some() {
            self.legacy().into_signed(signature).into()
        } else {
            self.dynamic().into_signed(signature).into()
        };
        let bytes = envelope.encoded_2718();
        Ok(SignedTransaction {
            hash: alloy_primitives::keccak256(&bytes),
            bytes: bytes.into(),
            signer: signer.address(),
            nonce: self.nonce,
            chain_id: self.chain_id,
        })
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub hash: B256,
    pub bytes: Bytes,
    pub signer: Address,
    pub nonce: u64,
    pub chain_id: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingTransaction {
    pub hash: B256,
    pub nonce: u64,
}
#[derive(Default)]
struct Nonces {
    next: Option<u64>,
    busy: bool,
    pending: Option<PendingTransaction>,
}
type NonceRegistry = Mutex<HashMap<(u64, Address), Arc<Mutex<Nonces>>>>;
fn nonce_state(chain: u64, address: Address) -> Arc<Mutex<Nonces>> {
    static REGISTRY: OnceLock<NonceRegistry> = OnceLock::new();
    let mut registry = REGISTRY
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    registry.entry((chain, address)).or_default().clone()
}
struct Reservation {
    state: Arc<Mutex<Nonces>>,
}
impl Reservation {
    fn new(state: Arc<Mutex<Nonces>>) -> Result<Self, Error> {
        let mut n = state.lock().unwrap_or_else(|e| e.into_inner());
        if n.busy || n.pending.is_some() {
            return Err(Error::PendingOutcome);
        }
        n.busy = true;
        drop(n);
        Ok(Self { state })
    }
    fn pending(&self, tx: &SignedTransaction) {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).pending = Some(PendingTransaction {
            hash: tx.hash,
            nonce: tx.nonce,
        });
    }
    fn accepted(&self, nonce: u64) {
        let mut n = self.state.lock().unwrap_or_else(|e| e.into_inner());
        n.pending = None;
        n.next = nonce.checked_add(1);
    }
}
impl Drop for Reservation {
    fn drop(&mut self) {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).busy = false;
    }
}

#[derive(Clone)]
pub struct MetricChainClient<T = DefaultTransport> {
    config: ChainConfig,
    transport: T,
    request_id: Arc<AtomicU64>,
    identities: Arc<Mutex<HashMap<Address, Arc<Mutex<Nonces>>>>>,
}
impl<T> std::fmt::Debug for MetricChainClient<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetricChainClient")
            .field("chain_id", &self.config.chain_id)
            .finish_non_exhaustive()
    }
}
impl<T> MetricChainClient<T> {
    pub fn with_transport(config: ChainConfig, transport: T) -> Self {
        Self {
            config,
            transport,
            request_id: Arc::new(AtomicU64::new(1)),
            identities: Default::default(),
        }
    }
    pub fn config(&self) -> &ChainConfig {
        &self.config
    }
    pub fn transport(&self) -> &T {
        &self.transport
    }
    fn state(&self, address: Address) -> Arc<Mutex<Nonces>> {
        self.identities
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .entry(address)
            .or_insert_with(|| nonce_state(self.config.chain_id, address))
            .clone()
    }
    pub fn pending(&self, address: Address) -> Option<PendingTransaction> {
        self.state(address)
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .pending
            .clone()
    }
}
#[cfg(feature = "http")]
impl MetricChainClient<aux_rs_core::StandardTransport> {
    pub fn with_config(
        config: ChainConfig,
        transport_config: aux_rs_core::TransportConfig,
    ) -> Result<Self, aux_rs_core::TransportError> {
        Ok(Self::with_transport(
            config,
            aux_rs_core::StandardTransport::new(transport_config)?,
        ))
    }

    pub fn new(config: ChainConfig) -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(config, Default::default())
    }
}
impl<T: HttpTransport> MetricChainClient<T> {
    async fn rpc(
        &self,
        method: &str,
        params: Value,
        write: bool,
    ) -> Result<Value, ClientError<Error>> {
        self.transport.ensure_open()?;
        let id = self.request_id.fetch_add(1, Ordering::Relaxed);
        let request = HttpRequest {
            method: HttpMethod::Post,
            url: if write {
                &self.config.write_rpc
            } else {
                &self.config.read_rpc
            }
            .clone(),
            headers: vec![("content-type".into(), "application/json".into())],
            body: serde_json::to_vec(
                &json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}),
            )
            .map_err(|e| ClientError::Exchange(e.into()))?,
        };
        let response = if method == "eth_sendRawTransaction" {
            self.transport.execute(request).await?
        } else {
            self.transport.execute_read(request).await?
        };
        let v: Value = crate::client::decode_response(response).map_err(ClientError::Exchange)?;
        if v["id"] != id || v["jsonrpc"] != "2.0" {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "mismatched JSON-RPC response",
            )));
        }
        if let Some(e) = v.get("error") {
            return Err(ClientError::Exchange(Error::Rpc {
                code: e["code"].as_i64().unwrap_or_default(),
                message: e["message"].as_str().unwrap_or_default().into(),
                data: e.get("data").cloned(),
            }));
        }
        v.get("result")
            .cloned()
            .ok_or(ClientError::Exchange(Error::InvalidRequest(
                "missing JSON-RPC result",
            )))
    }
    pub async fn validate_chain(&self) -> Result<(), ClientError<Error>> {
        for write in [false, true] {
            let v = self.rpc("eth_chainId", json!([]), write).await?;
            let chain = parse_quantity(&v).map_err(ClientError::Exchange)?;
            if chain != self.config.chain_id {
                return Err(ClientError::Exchange(Error::InvalidRequest(
                    "RPC chain differs from configured chain",
                )));
            }
        }
        Ok(())
    }
    pub async fn code(&self, address: Address, block: &str) -> Result<Bytes, ClientError<Error>> {
        let v = self
            .rpc("eth_getCode", json!([address, block]), false)
            .await?;
        serde_json::from_value(v).map_err(|e| ClientError::Exchange(e.into()))
    }
    pub async fn call<C: SolCall>(
        &self,
        to: Address,
        call: &C,
        from: Option<Address>,
        value: U256,
        block: &str,
    ) -> Result<C::Return, ClientError<Error>> {
        let bytes = self
            .eth_call(to, call.abi_encode().into(), from, value, block)
            .await?;
        C::abi_decode_returns_validate(&bytes)
            .map_err(|e| ClientError::Exchange(Error::Abi(e.to_string())))
    }
    pub async fn eth_call(
        &self,
        to: Address,
        data: Bytes,
        from: Option<Address>,
        value: U256,
        block: &str,
    ) -> Result<Bytes, ClientError<Error>> {
        let mut args = json!({"to":to,"data":data,"value":value});
        if let Some(from) = from {
            args["from"] = json!(from);
        }
        let v = self.rpc("eth_call", json!([args, block]), false).await?;
        serde_json::from_value(v).map_err(|e| ClientError::Exchange(e.into()))
    }
    pub async fn logs(&self, filter: Value) -> Result<Vec<Value>, ClientError<Error>> {
        let v = self.rpc("eth_getLogs", json!([filter]), false).await?;
        serde_json::from_value(v).map_err(|e| ClientError::Exchange(e.into()))
    }
    pub async fn transaction_count(&self, address: Address) -> Result<u64, ClientError<Error>> {
        let v = self
            .rpc(
                "eth_getTransactionCount",
                json!([address, "pending"]),
                false,
            )
            .await?;
        parse_quantity(&v).map_err(ClientError::Exchange)
    }
    pub async fn receipt(&self, hash: B256) -> Result<Option<Value>, ClientError<Error>> {
        let v = self
            .rpc("eth_getTransactionReceipt", json!([hash]), false)
            .await?;
        Ok((!v.is_null()).then_some(v))
    }
    pub async fn transaction(&self, hash: B256) -> Result<Option<Value>, ClientError<Error>> {
        let v = self
            .rpc("eth_getTransactionByHash", json!([hash]), false)
            .await?;
        Ok((!v.is_null()).then_some(v))
    }
    pub async fn estimate_gas(&self, request: Value) -> Result<u64, ClientError<Error>> {
        let v = self.rpc("eth_estimateGas", json!([request]), false).await?;
        parse_quantity(&v).map_err(ClientError::Exchange)
    }
    pub async fn block_number(&self) -> Result<u64, ClientError<Error>> {
        let v = self.rpc("eth_blockNumber", json!([]), false).await?;
        parse_quantity(&v).map_err(ClientError::Exchange)
    }
    pub async fn block(&self, number: &str) -> Result<Option<Value>, ClientError<Error>> {
        let v = self
            .rpc("eth_getBlockByNumber", json!([number, false]), false)
            .await?;
        Ok((!v.is_null()).then_some(v))
    }
    pub async fn native_balance(
        &self,
        address: Address,
        block: &str,
    ) -> Result<U256, ClientError<Error>> {
        let v = self
            .rpc("eth_getBalance", json!([address, block]), false)
            .await?;
        serde_json::from_value(v).map_err(|e| ClientError::Exchange(e.into()))
    }
    pub async fn gas_price(&self) -> Result<U256, ClientError<Error>> {
        let v = self.rpc("eth_gasPrice", json!([]), false).await?;
        serde_json::from_value(v).map_err(|e| ClientError::Exchange(e.into()))
    }
    /// Raw submission is a single attempt. Callers own sequencing for externally prepared bytes.
    pub async fn submit_raw(&self, bytes: Bytes) -> Result<B256, ClientError<Error>> {
        validate_signed_transaction(&bytes, self.config.chain_id).map_err(ClientError::Exchange)?;
        self.validate_chain().await?;
        let expected = alloy_primitives::keccak256(&bytes);
        let v = self
            .rpc("eth_sendRawTransaction", json!([bytes]), true)
            .await
            .map_err(|e| match e {
                ClientError::Exchange(cause) if !matches!(cause, Error::Rpc { .. }) => {
                    ClientError::Exchange(Error::UncertainSubmission {
                        hash: expected,
                        cause: Box::new(cause),
                    })
                }
                e => e,
            })?;
        let hash: B256 = serde_json::from_value(v).map_err(|e| {
            ClientError::Exchange(Error::UncertainSubmission {
                hash: expected,
                cause: Box::new(e.into()),
            })
        })?;
        if hash != expected {
            return Err(ClientError::Exchange(Error::UncertainSubmission {
                hash: expected,
                cause: Box::new(Error::InvalidRequest(
                    "RPC returned another transaction hash",
                )),
            }));
        }
        Ok(hash)
    }
    /// Fills a coordinated nonce, signs once, and submits once. Other fields remain caller-owned.
    pub async fn send_transaction<S: EvmSigner>(
        &self,
        mut prepared: PreparedTransaction,
        signer: &S,
    ) -> Result<B256, ClientError<Error>> {
        if prepared.chain_id != self.config.chain_id {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "transaction has another chain ID",
            )));
        }
        self.transport.ensure_open()?;
        self.validate_chain().await?;
        let code = self.code(prepared.to, "latest").await?;
        if code.is_empty() {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "transaction target has no contract code",
            )));
        }
        if self
            .config
            .code_hashes
            .get(&prepared.to)
            .is_some_and(|expected| *expected != alloy_primitives::keccak256(&code))
        {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "contract bytecode differs from configured identity",
            )));
        }
        let state = self.state(signer.address());
        let reservation = Reservation::new(state.clone()).map_err(ClientError::Exchange)?;
        let remote = self.transaction_count(signer.address()).await?;
        let next = state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .next
            .unwrap_or(remote);
        prepared.nonce = remote.max(next);
        let signed = prepared.sign(signer).await.map_err(ClientError::Exchange)?;
        reservation.pending(&signed);
        let result = self.submit_raw(signed.bytes).await;
        match &result {
            Ok(_) => reservation.accepted(signed.nonce),
            Err(ClientError::Transport(e)) if e.outcome == aux_rs_core::RequestOutcome::NotSent => {
                state.lock().unwrap_or_else(|e| e.into_inner()).pending = None;
            }
            _ => {}
        }
        result
    }
    /// Only a matching native transaction observation resolves an ambiguous send automatically.
    pub async fn reconcile(&self, address: Address) -> Result<bool, ClientError<Error>> {
        let Some(pending) = self.pending(address) else {
            return Ok(true);
        };
        if let Some(tx) = self.transaction(pending.hash).await? {
            if tx["hash"].as_str().and_then(|s| s.parse::<B256>().ok()) != Some(pending.hash)
                || tx["from"].as_str().and_then(|s| s.parse::<Address>().ok()) != Some(address)
            {
                return Err(ClientError::Exchange(Error::InvalidRequest(
                    "reconciliation identity mismatch",
                )));
            }
            let state = self.state(address);
            let mut n = state.lock().unwrap_or_else(|e| e.into_inner());
            if n.pending.as_ref().is_some_and(|p| p.hash == pending.hash) {
                n.pending = None;
                n.next = pending.nonce.checked_add(1);
            }
            return Ok(true);
        }
        Ok(false)
    }
    /// Explicit recovery after the application has reconciled rejection, replacement or dropping.
    /// An absent transaction does not by itself prove it was never submitted.
    pub async fn resync_nonce(
        &self,
        address: Address,
        expected_pending_hash: B256,
    ) -> Result<u64, ClientError<Error>> {
        let state = self.state(address);
        {
            let n = state.lock().unwrap_or_else(|e| e.into_inner());
            if n.busy
                || n.pending
                    .as_ref()
                    .is_none_or(|p| p.hash != expected_pending_hash)
            {
                return Err(ClientError::Exchange(Error::PendingOutcome));
            }
        }
        let remote = self.transaction_count(address).await?;
        let mut n = state.lock().unwrap_or_else(|e| e.into_inner());
        if n.busy
            || n.pending
                .as_ref()
                .is_none_or(|p| p.hash != expected_pending_hash)
        {
            return Err(ClientError::Exchange(Error::PendingOutcome));
        }
        n.pending = None;
        n.next = Some(remote);
        Ok(remote)
    }
}
fn parse_quantity(v: &Value) -> Result<u64, Error> {
    let s = v
        .as_str()
        .ok_or(Error::InvalidRequest("invalid RPC quantity"))?;
    u64::from_str_radix(
        s.strip_prefix("0x")
            .ok_or(Error::InvalidRequest("invalid RPC quantity"))?,
        16,
    )
    .map_err(|_| Error::InvalidRequest("RPC quantity overflows u64"))
}

pub(crate) fn validate_signed_transaction(bytes: &[u8], chain_id: u64) -> Result<Address, Error> {
    let mut input = bytes;
    let tx = TxEnvelope::decode_2718(&mut input)
        .map_err(|_| Error::InvalidRequest("invalid signed EVM transaction"))?;
    if !input.is_empty() || tx.chain_id() != Some(chain_id) {
        return Err(Error::InvalidRequest(
            "trailing transaction bytes or signing chain mismatch",
        ));
    }
    tx.recover_signer()
        .map_err(|_| Error::Signing("invalid transaction signature"))
}
