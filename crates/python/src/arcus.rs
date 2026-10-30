use crate::{
    convert, errors,
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::StandardTransport;
use aux_rs::arcus as a;
use aux_rs::hex;
use pyo3::prelude::*;
use std::sync::Arc;
use zeroize::Zeroizing;
#[pyclass(module = "aux_py._native", frozen)]
pub struct ArcusSigner {
    inner: Arc<a::LocalSigner>,
}
#[pymethods]
impl ArcusSigner {
    #[new]
    fn new(seed: String) -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(
                a::LocalSigner::from_hex(Zeroizing::new(seed))
                    .map_err(|e| errors::arcus(aux_rs::Error::Exchange(e)))?,
            ),
        })
    }
    #[getter]
    fn public_key(&self) -> String {
        use a::Signer;
        hex::encode(self.inner.public_key())
    }
    fn __repr__(&self) -> &'static str {
        "ArcusSigner()"
    }
}
type Private = a::ArcusClient<StandardTransport, a::ArcusAuth<Arc<a::LocalSigner>>>;
#[derive(Clone)]
struct Clients {
    public: a::ArcusClient<StandardTransport>,
    private: Option<Private>,
}
impl Clients {
    fn authenticated(&self) -> PyResult<Private> {
        self.private.clone().ok_or_else(|| {
            errors::AuthenticationError::new_err("configure an Arcus signer and master address")
        })
    }
}
impl state::ClientResources for Clients {
    fn transport(&self) -> &StandardTransport {
        self.public.transport()
    }
}
#[pyclass(module = "aux_py._native", frozen)]
pub struct ArcusClient {
    state: Arc<ClientState<Clients>>,
}
#[pymethods]
impl ArcusClient {
    #[new]
    #[pyo3(signature=(*,network="testnet",http_url=None,ws_url=None,signer=None,address=None,account_index=0,timeout_ms=10_000,subscription_capacity=128))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        signer: Option<PyRef<'_, ArcusSigner>>,
        address: Option<String>,
        account_index: u8,
        timeout_ms: u64,
        subscription_capacity: usize,
    ) -> PyResult<Self> {
        let exchange = match (http_url, ws_url) {
            (Some(h), Some(w)) => a::Arcus::custom(h, w),
            (None, None) => match network {
                "testnet" => a::Arcus::testnet(),
                "mainnet" => a::Arcus::mainnet(),
                _ => {
                    return Err(errors::ValidationError::new_err(
                        "network must be testnet or mainnet",
                    ));
                }
            },
            _ => {
                return Err(errors::ValidationError::new_err(
                    "custom Arcus endpoints require http_url and ws_url",
                ));
            }
        };
        let auth = match (signer, address) {
            (Some(s), Some(address)) => Some(
                a::ArcusAuth::new(s.inner.clone(), address, account_index)
                    .map_err(|e| errors::arcus(aux_rs::Error::Exchange(e)))?,
            ),
            (None, None) if account_index == 0 => None,
            _ => {
                return Err(errors::AuthenticationError::new_err(
                    "provide both signer and address",
                ));
            }
        };
        let config = state::config(
            timeout_ms,
            subscription_capacity,
            2,
            3,
            100,
            2000,
            20_000,
            60_000,
            8_388_608,
            30_000,
        )?;
        let public = state::build(py, || a::ArcusClient::with_config(exchange, config))?;
        let private = auth.map(|a| public.clone().with_auth(a));
        Ok(Self {
            state: ClientState::new(Clients { public, private }),
        })
    }
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    #[getter]
    fn authenticated(&self) -> PyResult<bool> {
        Ok(self.state.client()?.private.is_some())
    }
    #[pyo3(signature=(*,market=None))]
    fn markets<'py>(&self, py: Python<'py>, market: Option<String>) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.public;
        self.state.run(py, async move {
            c.markets(a::MarketsRequest { market })
                .await
                .map_err(errors::arcus)
        })
    }
    #[pyo3(signature=(market,*,n_levels=20))]
    fn order_book<'py>(
        &self,
        py: Python<'py>,
        market: String,
        n_levels: u8,
    ) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.public;
        self.state.run(py, async move {
            c.order_book(a::OrderBookRequest { market, n_levels })
                .await
                .map_err(errors::arcus)
        })
    }
    #[pyo3(signature=(market,*,limit=100))]
    fn trades<'py>(
        &self,
        py: Python<'py>,
        market: String,
        limit: u16,
    ) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.public;
        self.state.run(py, async move {
            c.trades(a::TradesRequest { market, limit })
                .await
                .map_err(errors::arcus)
        })
    }
    #[pyo3(signature=(address,*,account_index=0))]
    fn account<'py>(
        &self,
        py: Python<'py>,
        address: String,
        account_index: u8,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r = a::AccountRequest::new(address, account_index)
            .map_err(|e| errors::arcus(aux_rs::Error::Exchange(e)))?;
        let c = self.state.client()?.public;
        self.state
            .run(py, async move { c.account(r).await.map_err(errors::arcus) })
    }
    #[pyo3(signature=(address,*,account_index=0))]
    fn positions<'py>(
        &self,
        py: Python<'py>,
        address: String,
        account_index: u8,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r = a::AccountRequest::new(address, account_index)
            .map_err(|e| errors::arcus(aux_rs::Error::Exchange(e)))?;
        let c = self.state.client()?.public;
        self.state.run(
            py,
            async move { c.positions(r).await.map_err(errors::arcus) },
        )
    }
    #[pyo3(signature=(address,*,account_index=0))]
    fn open_orders<'py>(
        &self,
        py: Python<'py>,
        address: String,
        account_index: u8,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r = a::AccountRequest::new(address, account_index)
            .map_err(|e| errors::arcus(aux_rs::Error::Exchange(e)))?;
        let c = self.state.client()?.public;
        self.state.run(
            py,
            async move { c.open_orders(r).await.map_err(errors::arcus) },
        )
    }
    fn place_order<'py>(
        &self,
        py: Python<'py>,
        market: &Bound<'_, PyAny>,
        request: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let m: a::MarketInfo = convert::parse(market)?;
        let r: a::OrderRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.place_order(&m, r).await.map_err(errors::arcus)
        })
    }
    fn cancel_order<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: a::CancelOrderRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state.run(
            py,
            async move { c.cancel_order(r).await.map_err(errors::arcus) },
        )
    }
    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        specification: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: a::Subscription = convert::parse(specification)?;
        r.validate()
            .map_err(|e| errors::arcus(aux_rs::Error::Exchange(e)))?;
        let c = self.state.client()?.public;
        self.state.stream(py, async move {
            c.subscribe(r)
                .await
                .map(NativeHandle::Arcus)
                .map_err(errors::arcus)
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn __repr__(&self) -> &'static str {
        "ArcusClient()"
    }
}
impl Drop for ArcusClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
