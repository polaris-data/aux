use crate::{convert, errors};
use aux_rs::hex;
use aux_rs::{hyperliquid as hl, lighter as lt};
use hl::Signer as _;
use lt::Signer as _;
use pyo3::{prelude::*, types::PyBytes};
use std::{collections::BTreeMap, sync::Arc};
use zeroize::Zeroizing;

pub fn hl_network(network: &str) -> PyResult<hl::Network> {
    match network {
        "testnet" => Ok(hl::Network::Testnet),
        "mainnet" => Ok(hl::Network::Mainnet),
        _ => Err(errors::ValidationError::new_err(
            "network must be 'testnet' or 'mainnet'",
        )),
    }
}
pub fn lt_chain(network: &str, chain_id: Option<u32>) -> PyResult<u32> {
    Ok(chain_id.unwrap_or(match hl_network(network)? {
        hl::Network::Testnet => 300,
        hl::Network::Mainnet => 304,
    }))
}
pub fn address(value: &str) -> PyResult<hl::Address> {
    value.parse().map_err(errors::hl_native)
}
pub fn options(
    expired_at: Option<i64>,
    attributes: Option<BTreeMap<u8, i64>>,
) -> PyResult<lt::TxOptions> {
    let mut options = match expired_at {
        Some(v) => lt::TxOptions::new(v),
        None => lt::TxOptions::short_lived().map_err(errors::lt_native)?,
    };
    options.attributes =
        lt::TxAttributes::new(attributes.unwrap_or_default()).map_err(errors::lt_native)?;
    Ok(options)
}
pub fn transaction(kind: &str, request: &Bound<'_, PyAny>) -> PyResult<lt::Transaction> {
    let tx = match kind {
        "create" => lt::Transaction::CreateOrder(convert::parse(request)?),
        "cancel" => lt::Transaction::CancelOrder(convert::parse(request)?),
        _ => {
            return Err(errors::ValidationError::new_err(
                "transaction kind must be 'create' or 'cancel'",
            ));
        }
    };
    tx.validate().map_err(errors::lt_native)?;
    Ok(tx)
}

#[pyclass(module = "aux_py._native", frozen)]
#[derive(Clone)]
pub struct HyperliquidSigningPayload {
    inner: hl::SigningPayload,
}
#[pymethods]
impl HyperliquidSigningPayload {
    fn recover_address(&self, py: Python<'_>, signature: &Bound<'_, PyAny>) -> PyResult<String> {
        let signature: hl::Signature = convert::parse(signature)?;
        let payload = self.inner.clone();
        py.detach(move || signature.recover_address(&payload))
            .map(|a| a.to_string())
            .map_err(|_| errors::SigningError::new_err("invalid Hyperliquid signature"))
    }
    #[getter]
    fn digest<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.inner.digest())
    }
    #[getter]
    fn network(&self) -> &'static str {
        match self.inner.network() {
            hl::Network::Mainnet => "mainnet",
            hl::Network::Testnet => "testnet",
        }
    }
    #[getter]
    fn scheme(&self) -> &'static str {
        match self.inner.scheme() {
            hl::signing::SigningScheme::L1Action => "l1_action",
            hl::signing::SigningScheme::UserSignedAction => "user_signed_action",
        }
    }
}
#[pyclass(module = "aux_py._native", frozen)]
#[derive(Clone)]
pub struct LighterSigningPayload {
    inner: lt::SigningPayload,
}
#[pymethods]
impl LighterSigningPayload {
    fn verify(&self, py: Python<'_>, signature: &str, public_key: &str) -> PyResult<bool> {
        let signature = hex::decode(signature.strip_prefix("0x").unwrap_or(signature))
            .ok()
            .and_then(|v| v.try_into().ok())
            .ok_or_else(|| errors::SigningError::new_err("signature must encode 80 bytes"))?;
        let public_key = hex::decode(public_key.strip_prefix("0x").unwrap_or(public_key))
            .ok()
            .and_then(|v| v.try_into().ok())
            .ok_or_else(|| errors::SigningError::new_err("public key must encode 40 bytes"))?;
        let payload = self.inner.clone();
        py.detach(move || {
            lt::Signature::from_bytes(signature)?
                .verify(&payload, lt::PublicKey::from_bytes(public_key)?)
        })
        .map_err(errors::lt_native)?;
        Ok(true)
    }
    #[getter]
    fn digest<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.inner.digest())
    }
    #[getter]
    fn chain_id(&self) -> u32 {
        self.inner.chain_id()
    }
    #[getter]
    fn account_index(&self) -> i64 {
        self.inner.account_index()
    }
    #[getter]
    fn api_key_index(&self) -> u8 {
        self.inner.api_key_index()
    }
    #[getter]
    fn purpose(&self) -> PyResult<Py<PyAny>> {
        convert::output(match self.inner.purpose() {
            lt::signing::SigningPurpose::Transaction { tx_type, nonce } => {
                serde_json::json!({"type":"transaction","tx_type":tx_type,"nonce":nonce})
            }
            lt::signing::SigningPurpose::AuthToken { deadline } => {
                serde_json::json!({"type":"auth_token","deadline":deadline})
            }
        })
    }
}

/// A cancelled Rust operation also cancels the user callback on its own loop.
struct CallbackLease {
    task: Py<PyAny>,
    armed: bool,
}
impl Drop for CallbackLease {
    fn drop(&mut self) {
        if self.armed {
            let _ = Python::try_attach(|py| self.task.call_method0(py, "cancel"));
        }
    }
}
async fn callback(callback: &Py<PyAny>, payload: Py<PyAny>) -> PyResult<Py<PyAny>> {
    let (mut lease, future) = Python::attach(|py| -> PyResult<_> {
        let locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
        let package = py.import("aux_py")?;
        let coroutine = package
            .getattr("_call_signer")?
            .call1((callback, payload))?;
        let schedule = py.import("asyncio")?.getattr("run_coroutine_threadsafe")?;
        let task = locals
            .context(py)
            .call_method1("run", (schedule, coroutine, locals.event_loop(py)))?;
        let awaitable = package.getattr("_await_signer")?.call1((&task,))?;
        let future = pyo3_async_runtimes::into_future_with_locals(&locals, awaitable)?;
        Ok((
            CallbackLease {
                task: task.unbind(),
                armed: true,
            },
            future,
        ))
    })?;
    let result = future.await;
    lease.armed = false;
    result
}
enum HlKind {
    Local(hl::LocalSigner),
    Remote {
        address: hl::Address,
        callback: Py<PyAny>,
    },
}
#[derive(Clone)]
pub struct HlSigner(Arc<HlKind>);
impl hl::Signer for HlSigner {
    fn address(&self) -> hl::Address {
        match &*self.0 {
            HlKind::Local(s) => s.address(),
            HlKind::Remote { address, .. } => *address,
        }
    }
    async fn sign(&self, payload: hl::SigningPayload) -> Result<hl::Signature, hl::SignerError> {
        match &*self.0 {
            HlKind::Local(s) => s.sign(payload).await,
            HlKind::Remote { callback: cb, .. } => {
                let object = Python::attach(|py| {
                    Py::new(py, HyperliquidSigningPayload { inner: payload }).map(|o| o.into_any())
                })
                .map_err(|_| hl::SignerError::Unavailable)?;
                let value = callback(cb, object)
                    .await
                    .map_err(|_| hl::SignerError::Unavailable)?;
                Python::attach(|py| convert::parse(value.bind(py)))
                    .map_err(|_| hl::SignerError::InvalidSignature)
            }
        }
    }
}
enum LtKind {
    Local(lt::LocalSigner),
    Remote {
        key: lt::PublicKey,
        callback: Py<PyAny>,
    },
}
#[derive(Clone)]
pub struct LtSigner(Arc<LtKind>);
impl lt::Signer for LtSigner {
    fn public_key(&self) -> lt::PublicKey {
        match &*self.0 {
            LtKind::Local(s) => s.public_key(),
            LtKind::Remote { key, .. } => *key,
        }
    }
    async fn sign(&self, payload: &lt::SigningPayload) -> Result<lt::Signature, lt::Error> {
        match &*self.0 {
            LtKind::Local(s) => s.sign(payload).await,
            LtKind::Remote { callback: cb, .. } => {
                let object = Python::attach(|py| {
                    Py::new(
                        py,
                        LighterSigningPayload {
                            inner: payload.clone(),
                        },
                    )
                    .map(|o| o.into_any())
                })
                .map_err(|_| lt::Error::Signing("callback unavailable"))?;
                let value = callback(cb, object)
                    .await
                    .map_err(|_| lt::Error::Signing("callback failed"))?;
                let bytes = Python::attach(|py| value.extract::<String>(py))
                    .ok()
                    .and_then(|v| hex::decode(v.strip_prefix("0x").unwrap_or(&v)).ok())
                    .and_then(|v| v.try_into().ok())
                    .ok_or(lt::Error::InvalidSignature)?;
                lt::Signature::from_bytes(bytes)
            }
        }
    }
}

#[pyclass(module = "aux_py._native", frozen)]
#[derive(Clone)]
pub struct HyperliquidSigner {
    pub inner: HlSigner,
}
#[pymethods]
impl HyperliquidSigner {
    #[new]
    fn new(py: Python<'_>, private_key: String) -> PyResult<Self> {
        let private_key = Zeroizing::new(private_key);
        let signer = py
            .detach(move || private_key.parse())
            .map_err(|_| errors::SigningError::new_err("invalid Hyperliquid private key"))?;
        Ok(Self {
            inner: HlSigner(Arc::new(HlKind::Local(signer))),
        })
    }
    #[staticmethod]
    fn remote(address: &str, callback: Bound<'_, PyAny>) -> PyResult<Self> {
        if !callback.is_callable() {
            return Err(errors::ValidationError::new_err(
                "signer callback must be callable",
            ));
        }
        Ok(Self {
            inner: HlSigner(Arc::new(HlKind::Remote {
                address: self::address(address)?,
                callback: callback.unbind(),
            })),
        })
    }
    #[getter]
    fn address(&self) -> String {
        self.inner.address().to_string()
    }
    fn sign<'py>(
        &self,
        py: Python<'py>,
        payload: PyRef<'_, HyperliquidSigningPayload>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let payload = payload.inner.clone();
        let signer = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let signature = signer
                .sign(payload.clone())
                .await
                .map_err(|_| errors::SigningError::new_err("Hyperliquid signing failed"))?;
            if signature.recover_address(&payload).ok() != Some(signer.address()) {
                return Err(errors::SigningError::new_err("signature identity mismatch"));
            }
            convert::output(signature)
        })
    }
    #[pyo3(signature=(action,*,nonce,network="testnet",vault_address=None,expires_after=None))]
    fn sign_action<'py>(
        &self,
        py: Python<'py>,
        action: &Bound<'_, PyAny>,
        nonce: u64,
        network: &str,
        vault_address: Option<&str>,
        expires_after: Option<u64>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let action: hl::Action = convert::parse(action)?;
        action.validate().map_err(errors::hl_native)?;
        let network = hl_network(network)?;
        let vault = vault_address.map(address).transpose()?;
        let mut auth = hl::HyperliquidAuth::direct(self.inner.clone());
        if let Some(vault) = vault {
            auth = auth.with_vault(vault);
        }
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            convert::output(
                auth.prepare(
                    action,
                    network,
                    hl::ExecutionOptions {
                        nonce: Some(nonce),
                        expires_after,
                    },
                )
                .await
                .map_err(errors::hl_native)?,
            )
        })
    }
    #[pyo3(signature=(action,*,network="testnet",signature_chain_id=421_614))]
    fn sign_user_action<'py>(
        &self,
        py: Python<'py>,
        action: &Bound<'_, PyAny>,
        network: &str,
        signature_chain_id: u64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let action: hl::signing::UserSignedAction = convert::parse(action)?;
        let network = hl_network(network)?;
        let signer = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let payload = action
                .signing_payload(network, signature_chain_id)
                .map_err(errors::hl_native)?;
            let signature = signer
                .sign(payload.clone())
                .await
                .map_err(|_| errors::SigningError::new_err("Hyperliquid signing failed"))?;
            if signature.recover_address(&payload).ok() != Some(signer.address()) {
                return Err(errors::SigningError::new_err("signature identity mismatch"));
            }
            convert::output(signature)
        })
    }
    fn __repr__(&self) -> String {
        format!("HyperliquidSigner(address='{}')", self.address())
    }
}
#[pyclass(module = "aux_py._native", frozen)]
#[derive(Clone)]
pub struct LighterSigner {
    pub inner: LtSigner,
}
#[pymethods]
impl LighterSigner {
    #[new]
    fn new(py: Python<'_>, private_key: String) -> PyResult<Self> {
        let signer = py
            .detach(move || lt::LocalSigner::from_hex(Zeroizing::new(private_key)))
            .map_err(errors::lt_native)?;
        Ok(Self {
            inner: LtSigner(Arc::new(LtKind::Local(signer))),
        })
    }
    #[staticmethod]
    fn remote(public_key: &str, callback: Bound<'_, PyAny>) -> PyResult<Self> {
        if !callback.is_callable() {
            return Err(errors::ValidationError::new_err(
                "signer callback must be callable",
            ));
        }
        let key = hex::decode(public_key.strip_prefix("0x").unwrap_or(public_key))
            .ok()
            .and_then(|v| v.try_into().ok())
            .ok_or_else(|| errors::ValidationError::new_err("public key must encode 40 bytes"))?;
        Ok(Self {
            inner: LtSigner(Arc::new(LtKind::Remote {
                key: lt::PublicKey::from_bytes(key).map_err(errors::lt_native)?,
                callback: callback.unbind(),
            })),
        })
    }
    #[getter]
    fn public_key(&self) -> String {
        hex::encode(self.inner.public_key().as_bytes())
    }
    fn sign<'py>(
        &self,
        py: Python<'py>,
        payload: PyRef<'_, LighterSigningPayload>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let payload = payload.inner.clone();
        let signer = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let sig = signer.sign(&payload).await.map_err(errors::lt_native)?;
            sig.verify(&payload, signer.public_key())
                .map_err(errors::lt_native)?;
            Ok(hex::encode(sig.as_bytes()))
        })
    }
    #[pyo3(signature=(kind,request,*,account_index,api_key_index,nonce,network="testnet",chain_id=None,expired_at=None,attributes=None))]
    #[allow(clippy::too_many_arguments)]
    fn sign_transaction<'py>(
        &self,
        py: Python<'py>,
        kind: &str,
        request: &Bound<'_, PyAny>,
        account_index: i64,
        api_key_index: u8,
        nonce: i64,
        network: &str,
        chain_id: Option<u32>,
        expired_at: Option<i64>,
        attributes: Option<BTreeMap<u8, i64>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let tx = transaction(kind, request)?;
        let chain = lt_chain(network, chain_id)?;
        let options = options(expired_at, attributes)?;
        let signer = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let prepared = tx
                .prepare(chain, account_index, api_key_index, nonce, options)
                .map_err(errors::lt_native)?;
            let signed = prepared.sign(&signer).await.map_err(errors::lt_native)?;
            let info: serde_json::Value = serde_json::from_str(signed.tx_info()).map_err(|_| {
                errors::DecodeError::new_err("native transaction conversion failed")
            })?;
            convert::output(
                serde_json::json!({"tx_type":signed.tx_type(),"tx_info":info,"digest":hex::encode(signed.signing_payload().digest())}),
            )
        })
    }
    #[pyo3(signature=(*,account_index,api_key_index,deadline,network="testnet",chain_id=None))]
    fn auth_token<'py>(
        &self,
        py: Python<'py>,
        account_index: i64,
        api_key_index: u8,
        deadline: i64,
        network: &str,
        chain_id: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let auth = lt::LighterAuth::new(
            self.inner.clone(),
            lt_chain(network, chain_id)?,
            account_index,
            api_key_index,
        )
        .map_err(errors::lt_native)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(auth
                .token(deadline)
                .await
                .map_err(errors::lt_native)?
                .as_str()
                .to_owned())
        })
    }
    fn __repr__(&self) -> String {
        format!("LighterSigner(public_key='{}')", self.public_key())
    }
}

#[pyfunction]
#[pyo3(signature=(action,*,nonce,vault_address=None,expires_after=None))]
pub fn hyperliquid_action_hash(
    py: Python<'_>,
    action: &Bound<'_, PyAny>,
    nonce: u64,
    vault_address: Option<&str>,
    expires_after: Option<u64>,
) -> PyResult<String> {
    let action: hl::Action = convert::parse(action)?;
    action.validate().map_err(errors::hl_native)?;
    let vault = vault_address.map(address).transpose()?;
    py.detach(move || hl::signing::action_hash(&action, vault, nonce, expires_after))
        .map(hex::encode)
        .map_err(errors::hl_native)
}
#[pyfunction]
#[pyo3(signature=(order,*,sz_decimals,is_spot=false))]
pub fn validate_hyperliquid_order(
    py: Python<'_>,
    order: &Bound<'_, PyAny>,
    sz_decimals: u32,
    is_spot: bool,
) -> PyResult<()> {
    let order: hl::Order = convert::parse(order)?;
    py.detach(move || order.validate_precision(sz_decimals, is_spot))
        .map_err(errors::hl_native)
}
#[pyfunction]
pub fn scale_lighter_decimal(value: &str, decimals: u8) -> PyResult<u64> {
    lt::scale_decimal(value, decimals).map_err(errors::lt_native)
}

#[pyfunction]
#[pyo3(signature=(action,*,nonce,network="testnet",vault_address=None,expires_after=None))]
pub fn hyperliquid_signing_payload(
    py: Python<'_>,
    action: &Bound<'_, PyAny>,
    nonce: u64,
    network: &str,
    vault_address: Option<&str>,
    expires_after: Option<u64>,
) -> PyResult<HyperliquidSigningPayload> {
    let action: hl::Action = convert::parse(action)?;
    action.validate().map_err(errors::hl_native)?;
    let network = hl_network(network)?;
    let vault = vault_address.map(address).transpose()?;
    let inner = py
        .detach(move || {
            hl::signing::l1_signing_payload(&action, vault, nonce, expires_after, network)
        })
        .map_err(errors::hl_native)?;
    Ok(HyperliquidSigningPayload { inner })
}
#[pyfunction]
#[pyo3(signature=(action,*,network="testnet",signature_chain_id=421_614))]
pub fn hyperliquid_user_signing_payload(
    py: Python<'_>,
    action: &Bound<'_, PyAny>,
    network: &str,
    signature_chain_id: u64,
) -> PyResult<HyperliquidSigningPayload> {
    let action: hl::signing::UserSignedAction = convert::parse(action)?;
    let network = hl_network(network)?;
    let inner = py
        .detach(move || action.signing_payload(network, signature_chain_id))
        .map_err(errors::hl_native)?;
    Ok(HyperliquidSigningPayload { inner })
}
#[pyfunction]
#[pyo3(signature=(*,account_index,api_key_index,deadline,network="testnet",chain_id=None))]
pub fn lighter_auth_signing_payload(
    py: Python<'_>,
    account_index: i64,
    api_key_index: u8,
    deadline: i64,
    network: &str,
    chain_id: Option<u32>,
) -> PyResult<LighterSigningPayload> {
    let chain = lt_chain(network, chain_id)?;
    let inner = py
        .detach(move || lt::auth_signing_payload(chain, account_index, api_key_index, deadline))
        .map_err(errors::lt_native)?;
    Ok(LighterSigningPayload { inner })
}
#[pyfunction]
#[pyo3(signature=(kind,request,*,account_index,api_key_index,nonce,network="testnet",chain_id=None,expired_at=None,attributes=None))]
#[allow(clippy::too_many_arguments)]
pub fn lighter_transaction_signing_payload(
    py: Python<'_>,
    kind: &str,
    request: &Bound<'_, PyAny>,
    account_index: i64,
    api_key_index: u8,
    nonce: i64,
    network: &str,
    chain_id: Option<u32>,
    expired_at: Option<i64>,
    attributes: Option<BTreeMap<u8, i64>>,
) -> PyResult<LighterSigningPayload> {
    let tx = transaction(kind, request)?;
    let chain = lt_chain(network, chain_id)?;
    let options = options(expired_at, attributes)?;
    let inner = py
        .detach(move || {
            tx.prepare(chain, account_index, api_key_index, nonce, options)
                .map(|p| p.signing_payload().clone())
        })
        .map_err(errors::lt_native)?;
    Ok(LighterSigningPayload { inner })
}
