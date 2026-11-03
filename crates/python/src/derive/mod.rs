use crate::{
    convert, errors,
    signers::callback_with_locals,
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{Exchange, StandardTransport, derive as v};
use pyo3::{prelude::*, types::PyAny};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use v::{Authentication as _, Signer as _};
use zeroize::Zeroizing;
mod operations;
mod rpc;
pub use rpc::DeriveRpcSession;
fn native(error: aux_rs::Error<v::Error>) -> PyErr {
    errors::derive(error)
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
        errors::ValidationError::new_err("request does not match the native Derive schema")
    })
}
enum Kind {
    Local(v::LocalSigner),
    Remote {
        address: [u8; 20],
        callback: Py<PyAny>,
    },
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
    fn address(&self) -> [u8; 20] {
        match &*self.inner {
            Kind::Local(s) => s.address(),
            Kind::Remote { address, .. } => *address,
        }
    }
    async fn sign(&self, payload: &v::SigningPayload) -> Result<[u8; 65], v::Error> {
        match &*self.inner {
            Kind::Local(s) => s.sign(payload).await,
            Kind::Remote { callback, .. } => {
                let value=convert::output(serde_json::json!({"digest":format!("0x{}",aux_rs::hex::encode(payload.digest)),"chain_id":payload.chain_id,"scheme":match payload.scheme{v::SigningScheme::Login=>"login",v::SigningScheme::Action=>"action"}})).map_err(|_|v::Error::Signing("callback payload conversion failed"))?;
                let result = callback_with_locals(callback, value, self.locals.as_ref())
                    .await
                    .map_err(|_| v::Error::Signing("signer callback failed"))?;
                let text = Python::attach(|py| result.extract::<String>(py))
                    .map_err(|_| v::Error::Signing("callback must return a hex signature"))?;
                let mut sig = [0; 65];
                aux_rs::hex::decode_to_slice(text.strip_prefix("0x").unwrap_or(&text), &mut sig)
                    .map_err(|_| v::Error::Signing("callback returned an invalid signature"))?;
                Ok(sig)
            }
        }
    }
}
#[pyclass(module = "aux_py._native", frozen)]
#[derive(Clone)]
pub struct DeriveSigner {
    inner: Signer,
}
#[pymethods]
impl DeriveSigner {
    #[new]
    fn new(private_key: String) -> PyResult<Self> {
        Ok(Self {
            inner: Signer {
                inner: Arc::new(Kind::Local(
                    v::LocalSigner::from_hex(Zeroizing::new(private_key)).map_err(local)?,
                )),
                locals: None,
            },
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
            inner: Signer {
                inner: Arc::new(Kind::Remote {
                    address: v::address(address).map_err(local)?,
                    callback: callback.unbind(),
                }),
                locals: None,
            },
        })
    }
    #[getter]
    fn address(&self) -> String {
        format!("0x{}", aux_rs::hex::encode(self.inner.address()))
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
            Some("login") => v::SigningScheme::Login,
            Some("action") => v::SigningScheme::Action,
            _ => return Err(errors::ValidationError::new_err("invalid signing scheme")),
        };
        let payload = v::SigningPayload {
            digest,
            scheme,
            chain_id: value["chain_id"]
                .as_u64()
                .ok_or_else(|| errors::ValidationError::new_err("chain_id is required"))?,
        };
        let signer = self.inner.scoped(py)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            signer
                .sign(&payload)
                .await
                .map(|s| format!("0x{}", aux_rs::hex::encode(s)))
                .map_err(local)
        })
    }
    fn __repr__(&self) -> &'static str {
        "DeriveSigner()"
    }
}
#[derive(Clone)]
struct Resources {
    public: v::DeriveClient<StandardTransport>,
    auth: Option<v::DeriveAuth<Signer>>,
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
    ) -> PyResult<v::DeriveClient<StandardTransport, v::DeriveAuth<Signer>>> {
        let auth = self.auth.as_ref().ok_or_else(|| {
            errors::AuthenticationError::new_err("attach a Derive signer and wallet")
        })?;
        let scoped = v::DeriveAuth::new(
            self.public.exchange(),
            auth.wallet(),
            auth.signer().scoped(py)?,
        )
        .map_err(local)?;
        Ok(self.public.clone().with_auth(scoped))
    }
}
#[pyclass(module = "aux_py._native")]
pub struct DeriveClient {
    state: Arc<ClientState<Resources>>,
}
#[pymethods]
impl DeriveClient {
    fn open_rpc<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let resources = self.state.client()?;
        let authenticated = resources.auth.is_some();
        let client = if authenticated {
            Some(resources.authenticated(py)?)
        } else {
            None
        };
        let state = self.state.clone();
        self.state.object(py, async move {
            let inner = if let Some(client) = client {
                rpc::NativeSession::Authenticated(client.open_rpc().await.map_err(native)?)
            } else {
                rpc::NativeSession::Public(
                    resources.public.open_public_rpc().await.map_err(native)?,
                )
            };
            Python::attach(|py| {
                Py::new(
                    py,
                    DeriveRpcSession {
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
    #[pyo3(signature=(*,network="testnet",http_url=None,ws_url=None,chain_id=None,matching=None,signer=None,wallet=None,timeout_ms=10000,subscription_capacity=128,read_retries=2,reconnect_retries=3,initial_backoff_ms=100,max_backoff_ms=2000,heartbeat_interval_ms=20000,idle_timeout_ms=60000,max_message_bytes=8388608,reconnect_stable_ms=30000))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        chain_id: Option<u64>,
        matching: Option<String>,
        signer: Option<PyRef<'_, DeriveSigner>>,
        wallet: Option<String>,
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
            "testnet" => v::Derive::testnet(),
            "mainnet" => v::Derive::mainnet(),
            _ => {
                return Err(errors::ValidationError::new_err(
                    "network must be testnet or mainnet",
                ));
            }
        };
        if signer.is_some()
            && (http_url.is_some() || ws_url.is_some())
            && (chain_id.is_none() || matching.is_none())
        {
            return Err(errors::ValidationError::new_err(
                "custom authenticated endpoints require chain_id and matching",
            ));
        }
        let exchange = v::Derive::custom(
            http_url.unwrap_or_else(|| base.http_url().into()),
            ws_url.unwrap_or_else(|| base.ws_url().into()),
            chain_id.unwrap_or(base.chain_id),
            matching.unwrap_or(base.matching),
        )
        .map_err(local)?;
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
        let public = state::build(py, || v::DeriveClient::with_config(exchange, config))?;
        let auth = signer
            .map(|s| {
                let owner = wallet
                    .unwrap_or_else(|| format!("0x{}", aux_rs::hex::encode(s.inner.address())));
                v::DeriveAuth::new(public.exchange(), &owner, s.inner.clone()).map_err(local)
            })
            .transpose()?;
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
                    .map(NativeHandle::Derive)
                    .map_err(native)
            })
        } else {
            let client = self.state.client()?.public;
            self.state.stream(py, async move {
                client
                    .subscribe(spec)
                    .await
                    .map(NativeHandle::Derive)
                    .map_err(native)
            })
        }
    }
    fn sign_action<'py>(
        &self,
        py: Python<'py>,
        action: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let action: v::ActionPayload = convert::parse(&action)?;
        let client = self.state.client()?.authenticated(py)?;
        self.state.run(py, async move {
            client
                .auth()
                .sign_action(client.exchange(), &action)
                .await
                .map_err(local)
        })
    }
    fn prepare_signed<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'_, PyAny>,
        module_data: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let mut request = convert::from_python(&request)?;
        let data: v::ModuleData = convert::parse(&module_data)?;
        let client = self.state.client()?.authenticated(py)?;
        self.state.run(py, async move {
            v::sign_request(client.exchange(), client.auth(), &mut request, &data)
                .await
                .map_err(local)?;
            Ok(request)
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn __repr__(&self) -> &'static str {
        "DeriveClient()"
    }
}
impl Drop for DeriveClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
