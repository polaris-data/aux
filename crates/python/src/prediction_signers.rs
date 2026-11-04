use crate::{convert, errors, signers::callback_with_locals};
use aux_rs::{kalshi as k, pascal as p, polymarket as m};
use base64::{Engine, engine::general_purpose::STANDARD};
use m::EvmSigner as _;
use p::Signer as _;
use pyo3::{prelude::*, types::PyAny};
use std::sync::Arc;
use zeroize::Zeroizing;
enum Kind {
    Kalshi(k::LocalSigner),
    Pascal(p::LocalSigner),
    Remote {
        public_key: [u8; 32],
        public_pem: Option<String>,
        callback: Py<PyAny>,
    },
}
#[derive(Clone)]
pub(crate) struct MessageSigner {
    inner: Arc<Kind>,
    locals: Option<pyo3_async_runtimes::TaskLocals>,
}
impl MessageSigner {
    pub(crate) fn scoped(&self, py: Python<'_>) -> PyResult<Self> {
        Ok(Self {
            inner: self.inner.clone(),
            locals: Some(pyo3_async_runtimes::tokio::get_current_locals(py)?),
        })
    }
    async fn remote_sign(&self, message: &[u8], pascal: bool) -> Result<Vec<u8>, String> {
        let Kind::Remote {
            public_key,
            public_pem,
            callback,
        } = &*self.inner
        else {
            return Err("not a remote signer".into());
        };
        let payload=convert::output(serde_json::json!({"message_base64":STANDARD.encode(message),"message_hex":aux_rs::hex::encode(message),"public_key":if pascal{Some(bs58::encode(public_key).into_string())}else{None},"scheme":if pascal{"pascal_ed25519"}else{"kalshi_request"}})).map_err(|_|"callback payload conversion failed")?;
        let result = callback_with_locals(callback, payload, self.locals.as_ref())
            .await
            .map_err(|_| "signer callback failed")?;
        let text = Python::attach(|py| result.bind(py).extract::<String>())
            .map_err(|_| "callback must return an encoded signature")?;
        let sig = if pascal {
            bs58::decode(text)
                .into_vec()
                .map_err(|_| "invalid base58 signature")?
        } else {
            STANDARD
                .decode(text)
                .map_err(|_| "invalid base64 signature")?
        };
        if let Some(pem) = public_pem {
            k::verify_request_signature(pem, message, &sig)
                .map_err(|_| "remote signature does not match public key")?;
        }
        Ok(sig)
    }
}
impl k::RequestSigner for MessageSigner {
    async fn sign(&self, m: &[u8]) -> Result<Vec<u8>, k::Error> {
        match &*self.inner {
            Kind::Kalshi(s) => s.sign(m).await,
            Kind::Pascal(s) => s
                .sign(m)
                .await
                .map(|s| s.to_vec())
                .map_err(|_| k::Error::Signing("Ed25519 signing failed")),
            Kind::Remote { .. } => self
                .remote_sign(m, false)
                .await
                .map_err(|_| k::Error::Signing("remote request signing failed")),
        }
    }
}
impl p::Signer for MessageSigner {
    fn public_key(&self) -> [u8; 32] {
        match &*self.inner {
            Kind::Pascal(s) => s.public_key(),
            Kind::Remote { public_key, .. } => *public_key,
            Kind::Kalshi(_) => [0; 32],
        }
    }
    async fn sign(&self, m: &[u8]) -> Result<[u8; 64], p::Error> {
        match &*self.inner {
            Kind::Pascal(s) => s.sign(m).await,
            Kind::Remote { .. } => self
                .remote_sign(m, true)
                .await
                .map_err(|_| p::Error::Signing("remote Ed25519 signing failed"))?
                .try_into()
                .map_err(|_| p::Error::Signing("64-byte signature required")),
            Kind::Kalshi(_) => Err(p::Error::Signing("wrong signer kind")),
        }
    }
}
#[pyclass(module = "aux_py._native", frozen)]
#[derive(Clone)]
pub struct KalshiSigner {
    pub(crate) inner: MessageSigner,
}
#[pymethods]
impl KalshiSigner {
    #[new]
    fn new(private_key_pem: String) -> PyResult<Self> {
        Ok(Self {
            inner: MessageSigner {
                inner: Arc::new(Kind::Kalshi(
                    k::LocalSigner::from_pem(Zeroizing::new(private_key_pem))
                        .map_err(|e| errors::SigningError::new_err(e.to_string()))?,
                )),
                locals: None,
            },
        })
    }
    #[staticmethod]
    fn remote(public_key_pem: String, callback: Bound<'_, PyAny>) -> PyResult<Self> {
        if !callback.is_callable() {
            return Err(errors::ValidationError::new_err(
                "callback must be callable",
            ));
        }
        Ok(Self {
            inner: MessageSigner {
                inner: Arc::new(Kind::Remote {
                    public_key: [0; 32],
                    public_pem: Some(public_key_pem),
                    callback: callback.unbind(),
                }),
                locals: None,
            },
        })
    }
    fn __repr__(&self) -> &'static str {
        "KalshiSigner()"
    }
}
#[pyclass(module = "aux_py._native", frozen)]
#[derive(Clone)]
pub struct PascalSigner {
    pub(crate) inner: MessageSigner,
}
#[pymethods]
impl PascalSigner {
    #[new]
    fn new(private_key_hex: String) -> PyResult<Self> {
        Ok(Self {
            inner: MessageSigner {
                inner: Arc::new(Kind::Pascal(
                    p::LocalSigner::from_hex(Zeroizing::new(private_key_hex))
                        .map_err(|e| errors::SigningError::new_err(e.to_string()))?,
                )),
                locals: None,
            },
        })
    }
    #[staticmethod]
    fn remote(public_key: String, callback: Bound<'_, PyAny>) -> PyResult<Self> {
        if !callback.is_callable() {
            return Err(errors::ValidationError::new_err(
                "callback must be callable",
            ));
        }
        Ok(Self {
            inner: MessageSigner {
                inner: Arc::new(Kind::Remote {
                    public_key: p::public_key(&public_key)
                        .map_err(|e| errors::ValidationError::new_err(e.to_string()))?,
                    public_pem: None,
                    callback: callback.unbind(),
                }),
                locals: None,
            },
        })
    }
    #[getter]
    fn public_key(&self) -> String {
        bs58::encode(self.inner.public_key()).into_string()
    }
    fn __repr__(&self) -> &'static str {
        "PascalSigner()"
    }
}
#[pyclass(module = "aux_py._native", frozen)]
#[derive(Clone)]
pub struct PolymarketSigner {
    pub(crate) inner: crate::metric::Signer,
}
#[pymethods]
impl PolymarketSigner {
    #[new]
    fn new(private_key: String) -> PyResult<Self> {
        Ok(Self {
            inner: crate::metric::MetricSigner::new(private_key)?.inner,
        })
    }
    #[staticmethod]
    fn remote(address: &str, callback: Bound<'_, PyAny>) -> PyResult<Self> {
        Ok(Self {
            inner: crate::metric::MetricSigner::remote(address, callback)?.inner,
        })
    }
    #[getter]
    fn address(&self) -> String {
        self.inner.address().to_string()
    }
    fn sign_typed_data<'py>(
        &self,
        py: Python<'py>,
        payload: Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let payload = convert::from_python(&payload)?;
        let signer = self.inner.scoped(py)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            m::sign_typed_data(&signer, &payload)
                .await
                .map(|s| s.to_string())
                .map_err(|e| errors::SigningError::new_err(e.to_string()))
        })
    }
    fn __repr__(&self) -> &'static str {
        "PolymarketSigner()"
    }
}
#[pymethods]
impl PascalSigner {
    #[pyo3(signature=(owner,deployment_id,kind,request,*,client_ts_ms,recv_window_ms=5000))]
    #[allow(clippy::too_many_arguments)]
    fn sign_request<'py>(
        &self,
        py: Python<'py>,
        owner: &str,
        deployment_id: u8,
        kind: String,
        request: Bound<'_, PyAny>,
        client_ts_ms: u64,
        recv_window_ms: u32,
    ) -> PyResult<Bound<'py, PyAny>> {
        let mut request = convert::from_python(&request)?;
        let mut auth = p::PascalAuth::new(owner, self.inner.scoped(py)?)
            .map_err(|e| errors::pascal(aux_rs::Error::Exchange(e)))?;
        auth.recv_window_ms = recv_window_ms;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            auth.sign_request_at(deployment_id, &kind, &mut request, client_ts_ms)
                .await
                .map_err(|e| errors::pascal(aux_rs::Error::Exchange(e)))?;
            convert::output(request)
        })
    }
}
#[pymethods]
impl PolymarketSigner {
    #[pyo3(signature=(order,*,system="ctf",negative_risk=false))]
    fn sign_order<'py>(
        &self,
        py: Python<'py>,
        order: Bound<'_, PyAny>,
        system: &str,
        negative_risk: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        let order = convert::from_python(&order)?;
        let system = match system {
            "ctf" | "v1" => m::PositionSystem::Ctf,
            "v2" => m::PositionSystem::V2,
            _ => return Err(errors::ValidationError::new_err("system must be ctf or v2")),
        };
        let signer = self.inner.scoped(py)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            m::sign_order(
                &signer,
                &order,
                m::OrderDomain::polygon(system, negative_risk),
            )
            .await
            .map_err(|e| errors::polymarket(aux_rs::Error::Exchange(e)))
        })
    }
}
