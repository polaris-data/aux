use crate::{
    convert, errors,
    signers::callback_with_locals,
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{Exchange, StandardTransport, paradex as v};
use pyo3::{prelude::*, types::PyAny};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use v::Signer as _;
use zeroize::Zeroizing;
mod operations;
mod rpc;
pub use rpc::ParadexRpcSession;
fn native(error: aux_rs::Error<v::Error>) -> PyErr {
    errors::paradex(error)
}
fn local(error: v::Error) -> PyErr {
    native(aux_rs::Error::Exchange(error))
}
fn input<T: DeserializeOwned>(value: Option<Bound<'_, PyAny>>) -> PyResult<T> {
    let value = value
        .as_ref()
        .map(convert::from_python)
        .transpose()?
        .unwrap_or(serde_json::json!({}));
    serde_json::from_value(value).map_err(|_| {
        errors::ValidationError::new_err("request does not match the native Paradex schema")
    })
}
enum Kind {
    Stark(v::LocalStarkSigner),
    Evm(v::LocalEvmSigner),
    Remote {
        kind: v::SignerKind,
        public_key: String,
        callback: Py<PyAny>,
    },
    None,
}
#[derive(Clone)]
pub(crate) struct Signer {
    inner: Arc<Kind>,
    locals: Option<pyo3_async_runtimes::TaskLocals>,
}
impl Signer {
    fn scoped(&self, py: Python<'_>) -> PyResult<Self> {
        Ok(Self {
            inner: self.inner.clone(),
            locals: Some(pyo3_async_runtimes::tokio::get_current_locals(py)?),
        })
    }
}
impl v::Signer for Signer {
    fn kind(&self) -> v::SignerKind {
        match &*self.inner {
            Kind::Stark(_) | Kind::None => v::SignerKind::Starknet,
            Kind::Evm(_) => v::SignerKind::Evm,
            Kind::Remote { kind, .. } => *kind,
        }
    }
    fn public_key(&self) -> String {
        match &*self.inner {
            Kind::Stark(s) => s.public_key(),
            Kind::Evm(s) => s.public_key(),
            Kind::Remote { public_key, .. } => public_key.clone(),
            Kind::None => String::new(),
        }
    }
    async fn sign(&self, payload: &v::SigningPayload) -> Result<Vec<String>, v::Error> {
        match &*self.inner {
            Kind::Stark(s) => s.sign(payload).await,
            Kind::Evm(s) => s.sign(payload).await,
            Kind::None => Err(v::Error::Signing("no signer attached")),
            Kind::Remote { callback, .. } => {
                let value=convert::output(serde_json::json!({"digest":format!("0x{}",aux_rs::hex::encode(payload.digest)),"scheme":match payload.scheme{v::SigningScheme::StarknetMessage=>"starknet_message",v::SigningScheme::EvmPersonal=>"evm_personal"},"typed_data":payload.typed_data,"personal_message":payload.personal_message})).map_err(|_|v::Error::Signing("callback payload conversion failed"))?;
                let result = callback_with_locals(callback, value, self.locals.as_ref())
                    .await
                    .map_err(|_| v::Error::Signing("signer callback failed"))?;
                Python::attach(|py| {
                    let value = convert::from_python(result.bind(py))
                        .map_err(|_| v::Error::Signing("invalid callback signature"))?;
                    match value {
                        serde_json::Value::String(s) => Ok(vec![s]),
                        serde_json::Value::Array(xs) => xs
                            .into_iter()
                            .map(|x| match x {
                                serde_json::Value::String(s) => Ok(s),
                                serde_json::Value::Number(n) => Ok(n.to_string()),
                                _ => Err(v::Error::Signing("invalid callback signature")),
                            })
                            .collect(),
                        _ => Err(v::Error::Signing(
                            "callback must return a signature string or list",
                        )),
                    }
                })
            }
        }
    }
}
#[pyclass(module = "aux_py._native", frozen)]
#[derive(Clone)]
pub struct ParadexSigner {
    inner: Signer,
}
#[pymethods]
impl ParadexSigner {
    #[new]
    #[pyo3(signature=(private_key,*,kind="starknet"))]
    fn new(private_key: String, kind: &str) -> PyResult<Self> {
        let key = Zeroizing::new(private_key);
        let inner = match kind {
            "starknet" => Kind::Stark(v::LocalStarkSigner::from_hex(key).map_err(local)?),
            "evm" => Kind::Evm(v::LocalEvmSigner::from_hex(key).map_err(local)?),
            _ => {
                return Err(errors::ValidationError::new_err(
                    "kind must be starknet or evm",
                ));
            }
        };
        Ok(Self {
            inner: Signer {
                inner: Arc::new(inner),
                locals: None,
            },
        })
    }
    #[staticmethod]
    #[pyo3(signature=(public_key,callback,*,kind="starknet"))]
    fn remote(public_key: &str, callback: Bound<'_, PyAny>, kind: &str) -> PyResult<Self> {
        if !callback.is_callable() {
            return Err(errors::ValidationError::new_err(
                "signer callback must be callable",
            ));
        }
        let public_key = format!("{:#x}", v::felt(public_key).map_err(local)?);
        let kind = match kind {
            "starknet" => v::SignerKind::Starknet,
            "evm" => v::SignerKind::Evm,
            _ => {
                return Err(errors::ValidationError::new_err(
                    "kind must be starknet or evm",
                ));
            }
        };
        Ok(Self {
            inner: Signer {
                inner: Arc::new(Kind::Remote {
                    kind,
                    public_key,
                    callback: callback.unbind(),
                }),
                locals: None,
            },
        })
    }
    #[getter]
    fn public_key(&self) -> String {
        self.inner.public_key()
    }
    fn sign<'py>(&self, py: Python<'py>, payload: Bound<'_, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let value = convert::from_python(&payload)?;
        let mut digest = [0; 32];
        let text = value["digest"]
            .as_str()
            .ok_or_else(|| errors::ValidationError::new_err("digest is required"))?;
        aux_rs::hex::decode_to_slice(text.strip_prefix("0x").unwrap_or(text), &mut digest)
            .map_err(|_| errors::ValidationError::new_err("invalid digest"))?;
        let scheme = match value["scheme"].as_str() {
            Some("starknet_message") => v::SigningScheme::StarknetMessage,
            Some("evm_personal") => v::SigningScheme::EvmPersonal,
            _ => return Err(errors::ValidationError::new_err("invalid signing scheme")),
        };
        let payload = v::SigningPayload {
            digest,
            scheme,
            typed_data: value.get("typed_data").filter(|v| !v.is_null()).cloned(),
            personal_message: value["personal_message"].as_str().map(str::to_owned),
        };
        let signer = self.inner.scoped(py)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            signer.sign(&payload).await.map_err(local)
        })
    }
    fn __repr__(&self) -> &'static str {
        "ParadexSigner()"
    }
}
#[derive(Clone)]
struct Resources {
    public: v::ParadexClient<StandardTransport>,
    auth: Option<v::ParadexAuth<Signer>>,
}
impl state::ClientResources for Resources {
    fn transport(&self) -> &StandardTransport {
        self.public.transport()
    }
}
impl Resources {
    fn authenticated(
        &self,
        py: Python<'_>,
    ) -> PyResult<v::ParadexClient<StandardTransport, v::ParadexAuth<Signer>>> {
        let auth = self.auth.as_ref().ok_or_else(|| {
            errors::AuthenticationError::new_err("attach a Paradex account and signer or token")
        })?;
        Ok(self
            .public
            .clone()
            .with_auth(auth.with_signer(auth.signer().scoped(py)?).map_err(local)?))
    }
}
#[pyclass(module = "aux_py._native")]
pub struct ParadexClient {
    state: Arc<ClientState<Resources>>,
}
#[pymethods]
impl ParadexClient {
    fn open_rpc<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?.authenticated(py)?;
        let state = self.state.clone();
        self.state.object(py, async move {
            let inner = client.open_rpc().await.map_err(native)?;
            Python::attach(|py| {
                Py::new(
                    py,
                    ParadexRpcSession {
                        inner: Arc::new(tokio::sync::Mutex::new(inner)),
                        state,
                        closing: tokio::sync::watch::channel(false).0,
                    },
                )
                .map(|p| p.into_any())
            })
        })
    }
    #[new]
    #[pyo3(signature=(*,network="testnet",http_url=None,ws_url=None,chain_id=None,evm_chain_id=None,siwe_domain=None,siwe_uri=None,signer=None,account=None,subkey=None,token=None,read_only=true,timeout_ms=10000,subscription_capacity=128,read_retries=2,reconnect_retries=3,initial_backoff_ms=100,max_backoff_ms=2000,heartbeat_interval_ms=20000,idle_timeout_ms=60000,max_message_bytes=8388608,reconnect_stable_ms=30000))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        chain_id: Option<String>,
        evm_chain_id: Option<u64>,
        siwe_domain: Option<String>,
        siwe_uri: Option<String>,
        signer: Option<PyRef<'_, ParadexSigner>>,
        account: Option<String>,
        subkey: Option<String>,
        token: Option<String>,
        read_only: bool,
        timeout_ms: u64,
        subscription_capacity: usize,
        read_retries: usize,
        reconnect_retries: usize,
        initial_backoff_ms: u64,
        max_backoff_ms: u64,
        heartbeat_interval_ms: u64,
        idle_timeout_ms: u64,
        max_message_bytes: usize,
        reconnect_stable_ms: u64,
    ) -> PyResult<Self> {
        let base = match network {
            "testnet" => v::Paradex::testnet(),
            "mainnet" => v::Paradex::mainnet(),
            _ => {
                return Err(errors::ValidationError::new_err(
                    "network must be testnet or mainnet",
                ));
            }
        };
        if signer.is_some() && (http_url.is_some() || ws_url.is_some()) && chain_id.is_none() {
            return Err(errors::ValidationError::new_err(
                "custom authenticated endpoints require chain_id",
            ));
        }
        if signer
            .as_ref()
            .is_some_and(|s| s.inner.kind() == v::SignerKind::Evm)
            && (http_url.is_some() || ws_url.is_some())
            && (evm_chain_id.is_none() || siwe_domain.is_none() || siwe_uri.is_none())
        {
            return Err(errors::ValidationError::new_err(
                "custom EVM authentication requires evm_chain_id, siwe_domain, and siwe_uri",
            ));
        }
        let exchange = v::Paradex::custom(
            http_url.unwrap_or_else(|| base.http_url().into()),
            ws_url.unwrap_or_else(|| base.ws_url().into()),
            chain_id.unwrap_or(base.chain_id),
            evm_chain_id.unwrap_or(base.evm_chain_id),
            siwe_domain.unwrap_or(base.siwe_domain),
            siwe_uri.unwrap_or(base.siwe_uri),
        );
        let config = state::config(
            timeout_ms,
            subscription_capacity,
            read_retries,
            reconnect_retries,
            initial_backoff_ms,
            max_backoff_ms,
            heartbeat_interval_ms,
            idle_timeout_ms,
            max_message_bytes,
            reconnect_stable_ms,
        )?;
        let public = state::build(py, || v::ParadexClient::with_config(exchange, config))?;
        let auth = if signer.is_some() || token.is_some() || account.is_some() {
            let account = account.ok_or_else(|| {
                errors::ValidationError::new_err("account is required with signer or token")
            })?;
            let s = signer.map(|s| s.inner.clone()).unwrap_or(Signer {
                inner: Arc::new(Kind::None),
                locals: None,
            });
            let auth =
                v::ParadexAuth::new(public.exchange(), &account, s, subkey).map_err(local)?;
            if let Some(token) = token {
                auth.tokens()
                    .set(Zeroizing::new(token), read_only)
                    .map_err(local)?;
            }
            Some(auth)
        } else {
            None
        };
        Ok(Self {
            state: ClientState::new(Resources { public, auth }),
        })
    }
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    #[getter]
    fn authenticated(&self) -> PyResult<bool> {
        Ok(self.state.client()?.auth.is_some())
    }
    fn authenticate<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?.authenticated(py)?;
        self.state.run(
            py,
            async move { client.authenticate().await.map_err(native) },
        )
    }
    #[pyo3(signature=(token,*,read_only=true))]
    fn set_token(&self, token: String, read_only: bool) -> PyResult<()> {
        self.state
            .client()?
            .auth
            .as_ref()
            .ok_or_else(|| errors::AuthenticationError::new_err("configure an account first"))?
            .tokens()
            .set(Zeroizing::new(token), read_only)
            .map_err(local)
    }
    fn onboarding_typed_data(&self) -> PyResult<Py<PyAny>> {
        let client = self.state.client()?.public;
        convert::output(v::onboarding_typed_data(client.exchange()).map_err(local)?)
    }
    #[pyo3(signature=(statement,nonce,issued_at,expiration=None))]
    fn sign_siwe<'py>(
        &self,
        py: Python<'py>,
        statement: String,
        nonce: String,
        issued_at: String,
        expiration: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?.authenticated(py)?;
        self.state.run(py, async move {
            client
                .auth()
                .sign_siwe(
                    client.exchange(),
                    &statement,
                    &nonce,
                    &issued_at,
                    expiration.as_deref(),
                )
                .await
                .map_err(local)
        })
    }
    fn evm_account_address<'py>(
        &self,
        py: Python<'py>,
        address: String,
        class_hash: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.state.client()?;
        self.state.run(py, async move {
            v::evm_account_address(&address, &class_hash).map_err(local)
        })
    }
    fn sign_typed_data<'py>(
        &self,
        py: Python<'py>,
        data: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let data = convert::from_python(&data)?;
        let client = self.state.client()?.authenticated(py)?;
        self.state.run(py, async move {
            client
                .auth()
                .sign_typed_data(client.exchange(), data)
                .await
                .map_err(local)
        })
    }
    #[pyo3(signature=(nonce,expiration,trades,*,parent=None))]
    fn block_signature<'py>(
        &self,
        py: Python<'py>,
        nonce: String,
        expiration: u64,
        trades: Bound<'_, PyAny>,
        parent: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let trades = convert::from_python(&trades)?;
        let client = self.state.client()?.authenticated(py)?;
        self.state.run(py, async move {
            client
                .auth()
                .block_signature(
                    client.exchange(),
                    &nonce,
                    expiration,
                    &trades,
                    parent.as_deref(),
                )
                .await
                .map_err(local)
        })
    }
    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        specification: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let spec: v::Subscription = convert::parse(&specification)?;
        spec.channel().map_err(local)?;
        if spec.is_private() {
            let client = self.state.client()?.authenticated(py)?;
            self.state.stream(py, async move {
                client
                    .subscribe_private(spec)
                    .await
                    .map(NativeHandle::Paradex)
                    .map_err(native)
            })
        } else {
            let client = self.state.client()?.public;
            self.state.stream(py, async move {
                client
                    .subscribe(spec)
                    .await
                    .map(NativeHandle::Paradex)
                    .map_err(native)
            })
        }
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn __repr__(&self) -> &'static str {
        "ParadexClient()"
    }
}
impl Drop for ParadexClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
