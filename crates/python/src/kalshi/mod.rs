use crate::{
    convert, errors,
    prediction_signers::*,
    state::{self, ClientResources, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{StandardTransport, kalshi as v};
use pyo3::{prelude::*, types::PyAny};
use serde_json::Value;
use std::sync::Arc;
use v::{Authentication, StreamAuthentication};
use zeroize::Zeroizing;
fn native(error: aux_rs::Error<v::Error>) -> PyErr {
    errors::kalshi(error)
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
    Key(v::KalshiAuth<MessageSigner>),
    Bearer(v::KalshiBearerAuth),
}
impl Auth {
    fn scoped(&self, py: Python<'_>) -> PyResult<Self> {
        Ok(match self {
            Self::Key(a) => Self::Key(a.clone().with_signer(a.signer().scoped(py)?)),
            s => s.clone(),
        })
    }
}
impl Authentication for Auth {
    fn identity(&self) -> String {
        match self {
            Self::None => "public".into(),
            Self::Key(a) => a.identity(),
            Self::Bearer(a) => a.identity(),
        }
    }
    async fn prepare(
        &self,
        e: &v::Kalshi,
        o: &v::Operation,
        r: &mut aux_rs::HttpRequest,
    ) -> Result<(), v::Error> {
        match self {
            Self::Key(a) => a.prepare(e, o, r).await,
            Self::Bearer(a) => a.prepare(e, o, r).await,
            Self::None => Err(v::Error::Authentication("API credentials required")),
        }
    }
}
impl StreamAuthentication for Auth {
    fn stream_identity(&self) -> String {
        self.identity()
    }
    async fn stream_headers(&self, u: &str) -> Result<Vec<(String, String)>, v::Error> {
        match self {
            Self::Key(a) => a.stream_headers(u).await,
            _ => Err(v::Error::Authentication(
                "signed WebSocket credentials required",
            )),
        }
    }
    fn stream_credentials(&self, _: &str) -> Result<Option<Value>, v::Error> {
        Ok(None)
    }
}

#[derive(Clone)]
struct KalshiClientResources {
    client: v::KalshiClient<StandardTransport, Auth>,
    auth: Auth,
}
impl ClientResources for KalshiClientResources {
    fn transport(&self) -> &StandardTransport {
        self.client.transport()
    }
}
impl KalshiClientResources {
    fn scoped(&self, py: Python<'_>) -> PyResult<v::KalshiClient<StandardTransport, Auth>> {
        Ok(self.client.clone().with_auth(self.auth.scoped(py)?))
    }
}
#[pyclass(module = "aux_py._native")]
pub struct KalshiClient {
    state: Arc<ClientState<KalshiClientResources>>,
}
#[pymethods]
impl KalshiClient {
    #[new]
    #[pyo3(signature=(*,network="demo",endpoints=None,websocket=None,key_id=None,signer=None,bearer_token=None,timeout_ms=10000,subscription_capacity=128))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        endpoints: Option<Bound<'_, PyAny>>,
        websocket: Option<Bound<'_, PyAny>>,
        key_id: Option<String>,
        signer: Option<PyRef<'_, KalshiSigner>>,
        bearer_token: Option<String>,
        timeout_ms: u64,
        subscription_capacity: usize,
    ) -> PyResult<Self> {
        let mut exchange = match network {
            "demo" | "testnet" => v::Kalshi::demo(),
            "mainnet" | "production" => v::Kalshi::mainnet(),
            _ => {
                return Err(errors::ValidationError::new_err(
                    "network must be demo or mainnet",
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
        let auth = if let Some(token) = bearer_token {
            if key_id.is_some() || signer.is_some() {
                return Err(errors::ValidationError::new_err(
                    "select API key or bearer credentials",
                ));
            }
            Auth::Bearer(v::KalshiBearerAuth::new(Zeroizing::new(token)).map_err(local)?)
        } else {
            match (key_id, signer) {
                (None, None) => Auth::None,
                (Some(id), Some(s)) => {
                    Auth::Key(v::KalshiAuth::new(id, s.inner.clone()).map_err(local)?)
                }
                _ => {
                    return Err(errors::ValidationError::new_err(
                        "key_id and signer are required together",
                    ));
                }
            }
        };
        let config = aux_rs::TransportConfig {
            timeout: std::time::Duration::from_millis(timeout_ms),
            subscription_capacity,
            ..Default::default()
        };
        let client = state::build(py, || v::KalshiClient::with_config(exchange, config))?
            .with_auth(auth.clone());
        Ok(Self {
            state: ClientState::new(KalshiClientResources { client, auth }),
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
                .map(NativeHandle::Kalshi)
                .map_err(native)
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn __repr__(&self) -> &'static str {
        "KalshiClient()"
    }
}
impl Drop for KalshiClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
mod kalshi_client_operations;
#[derive(Clone)]
struct KalshiPerpsClientResources {
    client: v::KalshiPerpsClient<StandardTransport, Auth>,
    auth: Auth,
}
impl ClientResources for KalshiPerpsClientResources {
    fn transport(&self) -> &StandardTransport {
        self.client.transport()
    }
}
impl KalshiPerpsClientResources {
    fn scoped(&self, py: Python<'_>) -> PyResult<v::KalshiPerpsClient<StandardTransport, Auth>> {
        Ok(self.client.clone().with_auth(self.auth.scoped(py)?))
    }
}
#[pyclass(module = "aux_py._native")]
pub struct KalshiPerpsClient {
    state: Arc<ClientState<KalshiPerpsClientResources>>,
}
#[pymethods]
impl KalshiPerpsClient {
    #[new]
    #[pyo3(signature=(*,network="demo",endpoints=None,websocket=None,key_id=None,signer=None,bearer_token=None,timeout_ms=10000,subscription_capacity=128))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        endpoints: Option<Bound<'_, PyAny>>,
        websocket: Option<Bound<'_, PyAny>>,
        key_id: Option<String>,
        signer: Option<PyRef<'_, KalshiSigner>>,
        bearer_token: Option<String>,
        timeout_ms: u64,
        subscription_capacity: usize,
    ) -> PyResult<Self> {
        let mut exchange = match network {
            "demo" | "testnet" => v::Kalshi::demo(),
            "mainnet" | "production" => v::Kalshi::mainnet(),
            _ => {
                return Err(errors::ValidationError::new_err(
                    "network must be demo or mainnet",
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
        let auth = if let Some(token) = bearer_token {
            if key_id.is_some() || signer.is_some() {
                return Err(errors::ValidationError::new_err(
                    "select API key or bearer credentials",
                ));
            }
            Auth::Bearer(v::KalshiBearerAuth::new(Zeroizing::new(token)).map_err(local)?)
        } else {
            match (key_id, signer) {
                (None, None) => Auth::None,
                (Some(id), Some(s)) => {
                    Auth::Key(v::KalshiAuth::new(id, s.inner.clone()).map_err(local)?)
                }
                _ => {
                    return Err(errors::ValidationError::new_err(
                        "key_id and signer are required together",
                    ));
                }
            }
        };
        let config = aux_rs::TransportConfig {
            timeout: std::time::Duration::from_millis(timeout_ms),
            subscription_capacity,
            ..Default::default()
        };
        let client = state::build(py, || v::KalshiPerpsClient::with_config(exchange, config))?
            .with_auth(auth.clone());
        Ok(Self {
            state: ClientState::new(KalshiPerpsClientResources { client, auth }),
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
                .map(NativeHandle::Kalshi)
                .map_err(native)
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn __repr__(&self) -> &'static str {
        "KalshiPerpsClient()"
    }
}
impl Drop for KalshiPerpsClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
mod kalshi_perps_client_operations;

mod rpc;
pub use rpc::KalshiRpcSession;
