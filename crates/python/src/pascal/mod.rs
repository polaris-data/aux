use crate::{
    convert, errors,
    prediction_signers::*,
    state::{self, ClientResources, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{StandardTransport, pascal as v};
use pyo3::{prelude::*, types::PyAny};
use serde_json::Value;
use std::sync::Arc;
use v::{Authentication, StreamAuthentication};
fn native(error: aux_rs::Error<v::Error>) -> PyErr {
    errors::pascal(error)
}
fn local(error: v::Error) -> PyErr {
    native(aux_rs::Error::Exchange(error))
}
fn input<T: serde::de::DeserializeOwned>(value: Option<Bound<'_, PyAny>>) -> PyResult<T> {
    let v = value
        .as_ref()
        .map(convert::from_python)
        .transpose()?
        .unwrap_or(serde_json::json!({}));
    serde_json::from_value(v)
        .map_err(|_| errors::ValidationError::new_err("request does not match the native schema"))
}
#[derive(Clone)]
pub(crate) enum Auth {
    None,
    Key(v::PascalAuth<MessageSigner>),
}
impl Auth {
    fn scoped(&self, py: Python<'_>) -> PyResult<Self> {
        Ok(match self {
            Self::Key(a) => Self::Key(a.clone().with_signer(a.signer().scoped(py)?)),
            Self::None => Self::None,
        })
    }
}
impl Authentication for Auth {
    fn identity(&self) -> String {
        match self {
            Self::None => "public".into(),
            Self::Key(a) => a.identity(),
        }
    }
    fn begin(&self, e: &v::Pascal, o: &v::Operation) -> Result<v::Submission, v::Error> {
        match self {
            Self::Key(a) => a.begin(e, o),
            Self::None => Err(v::Error::Authentication("signer required")),
        }
    }
    async fn prepare(
        &self,
        e: &v::Pascal,
        o: &v::Operation,
        r: &mut aux_rs::HttpRequest,
    ) -> Result<(), v::Error> {
        match self {
            Self::Key(a) => a.prepare(e, o, r).await,
            Self::None => Err(v::Error::Authentication("owner and signer required")),
        }
    }
}
impl StreamAuthentication for Auth {
    fn stream_identity(&self) -> String {
        self.identity()
    }
    fn stream_credentials(&self, _: &str) -> Result<Option<Value>, v::Error> {
        Ok(None)
    }
}

#[derive(Clone)]
struct PascalClientResources {
    client: v::PascalClient<StandardTransport, Auth>,
    auth: Auth,
}
impl ClientResources for PascalClientResources {
    fn transport(&self) -> &StandardTransport {
        self.client.transport()
    }
}
impl PascalClientResources {
    fn scoped(&self, py: Python<'_>) -> PyResult<v::PascalClient<StandardTransport, Auth>> {
        Ok(self.client.clone().with_auth(self.auth.scoped(py)?))
    }
}
#[pyclass(module = "aux_py._native")]
pub struct PascalClient {
    state: Arc<ClientState<PascalClientResources>>,
}
#[pymethods]
impl PascalClient {
    #[new]
    #[pyo3(signature=(*,network="canary",endpoints=None,websocket=None,owner=None,signer=None,timeout_ms=10000,subscription_capacity=128))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        endpoints: Option<Bound<'_, PyAny>>,
        websocket: Option<Bound<'_, PyAny>>,
        owner: Option<String>,
        signer: Option<PyRef<'_, PascalSigner>>,
        timeout_ms: u64,
        subscription_capacity: usize,
    ) -> PyResult<Self> {
        let mut exchange = match network {
            "canary" => v::Pascal::canary(),
            "mainnet" | "production" => v::Pascal::mainnet(),
            _ => {
                return Err(errors::ValidationError::new_err(
                    "network must be canary or mainnet",
                ));
            }
        };
        if let Some(e) = endpoints {
            let values: std::collections::BTreeMap<String, String> = convert::parse(&e)?;
            for (k, u) in values {
                exchange = exchange.custom(&k, &u, None).map_err(local)?;
            }
        }
        if let Some(w) = websocket {
            let values: std::collections::BTreeMap<String, String> = convert::parse(&w)?;
            for (k, u) in values {
                v::validate_url(&u, true).map_err(local)?;
                if !exchange.websocket.contains_key(&k) {
                    return Err(errors::ValidationError::new_err(
                        "unknown WebSocket service",
                    ));
                }
                exchange.websocket.insert(k, u);
            }
        }
        let auth = match (owner, signer) {
            (None, None) => Auth::None,
            (Some(owner), Some(s)) => {
                Auth::Key(v::PascalAuth::new(&owner, s.inner.clone()).map_err(local)?)
            }
            _ => {
                return Err(errors::ValidationError::new_err(
                    "owner and signer are required together",
                ));
            }
        };
        let config = aux_rs::TransportConfig {
            timeout: std::time::Duration::from_millis(timeout_ms),
            subscription_capacity,
            ..Default::default()
        };
        let client = state::build(py, || v::PascalClient::with_config(exchange, config))?
            .with_auth(auth.clone());
        Ok(Self {
            state: ClientState::new(PascalClientResources { client, auth }),
        })
    }
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    #[getter]
    fn authenticated(&self) -> PyResult<bool> {
        Ok(!matches!(self.state.client()?.auth, Auth::None))
    }
    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        specification: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let specification: v::Subscription = convert::parse(&specification)?;
        let client = self.state.client()?.scoped(py)?;
        self.state.stream(py, async move {
            client
                .subscribe(specification)
                .await
                .map(NativeHandle::Pascal)
                .map_err(native)
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn __repr__(&self) -> &'static str {
        "PascalClient()"
    }
}
impl Drop for PascalClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
mod pascal_client_operations;

mod rpc;
pub use rpc::PascalRpcSession;

mod chain;
pub use chain::PascalChainClient;
#[pymethods]
impl PascalClient {
    #[getter]
    fn wallet_outcome_pending(&self) -> PyResult<bool> {
        let c = self.state.client()?;
        Ok(match &c.auth {
            Auth::Key(a) => a.pending(c.client.exchange().deployment_id),
            Auth::None => false,
        })
    }
    fn reconcile_wallet(&self) -> PyResult<()> {
        let c = self.state.client()?;
        match &c.auth {
            Auth::Key(a) => a
                .reconcile_wallet(c.client.exchange().deployment_id)
                .map_err(local),
            Auth::None => Err(errors::AuthenticationError::new_err(
                "owner signer required",
            )),
        }
    }
}
