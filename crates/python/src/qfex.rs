use crate::{
    convert, errors,
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::StandardTransport;
use aux_rs::qfex as q;
use pyo3::prelude::*;
use std::sync::Arc;
use zeroize::Zeroizing;
#[derive(Clone)]
struct Clients {
    public: q::QfexClient<StandardTransport>,
    private: Option<q::QfexClient<StandardTransport, q::QfexAuth>>,
}
impl Clients {
    fn authenticated(&self) -> PyResult<q::QfexClient<StandardTransport, q::QfexAuth>> {
        self.private.clone().ok_or_else(|| {
            errors::AuthenticationError::new_err("configure a QFEX public/secret API-key pair")
        })
    }
}
impl state::ClientResources for Clients {
    fn transport(&self) -> &StandardTransport {
        self.public.transport()
    }
}
#[pyclass(module = "aux_py._native", frozen)]
pub struct QfexClient {
    state: Arc<ClientState<Clients>>,
}
#[pymethods]
impl QfexClient {
    #[new]
    #[pyo3(signature=(*,network="testnet",http_url=None,ws_url=None,trade_ws_url=None,public_key=None,secret_key=None,account_id=None,builder_code=None,timeout_ms=10_000,subscription_capacity=128))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        trade_ws_url: Option<String>,
        public_key: Option<String>,
        secret_key: Option<String>,
        account_id: Option<String>,
        builder_code: Option<String>,
        timeout_ms: u64,
        subscription_capacity: usize,
    ) -> PyResult<Self> {
        let secret_key = secret_key.map(Zeroizing::new);
        let exchange = match (http_url, ws_url, trade_ws_url) {
            (Some(h), Some(w), Some(t)) => q::Qfex::custom(h, w, t),
            (None, None, None) => match network {
                "testnet" => q::Qfex::testnet(),
                "mainnet" => q::Qfex::mainnet(),
                _ => {
                    return Err(errors::ValidationError::new_err(
                        "network must be testnet or mainnet",
                    ));
                }
            },
            _ => {
                return Err(errors::ValidationError::new_err(
                    "custom QFEX endpoints require http_url, ws_url, and trade_ws_url",
                ));
            }
        };
        let auth = match (public_key, secret_key) {
            (Some(p), Some(s)) => {
                let mut a =
                    q::QfexAuth::new(p, s).map_err(|e| errors::qfex(aux_rs::Error::Exchange(e)))?;
                if let Some(id) = account_id {
                    a = a
                        .with_account(id)
                        .map_err(|e| errors::qfex(aux_rs::Error::Exchange(e)))?;
                }
                if let Some(code) = builder_code {
                    a = a
                        .with_builder(code)
                        .map_err(|e| errors::qfex(aux_rs::Error::Exchange(e)))?;
                }
                Some(a)
            }
            (None, None) if account_id.is_none() && builder_code.is_none() => None,
            _ => {
                return Err(errors::AuthenticationError::new_err(
                    "provide both public_key and secret_key",
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
        let public = state::build(py, || q::QfexClient::with_config(exchange, config))?;
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
    #[pyo3(signature=(*,ticker=None))]
    fn refdata<'py>(&self, py: Python<'py>, ticker: Option<String>) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.public;
        self.state.run(py, async move {
            c.refdata(q::RefdataRequest { ticker })
                .await
                .map_err(errors::qfex)
        })
    }
    #[pyo3(signature=(*,symbol=None))]
    fn contracts<'py>(
        &self,
        py: Python<'py>,
        symbol: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.public;
        self.state.run(py, async move {
            c.contracts(q::ContractsRequest { symbol })
                .await
                .map_err(errors::qfex)
        })
    }
    fn order_book<'py>(&self, py: Python<'py>, symbol: String) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.public;
        self.state.run(py, async move {
            c.order_book(q::OrderBookRequest::new(symbol))
                .await
                .map_err(errors::qfex)
        })
    }
    fn positions<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(py, async move { c.positions().await.map_err(errors::qfex) })
    }
    fn account_balances<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.account_balances().await.map_err(errors::qfex)
        })
    }
    #[pyo3(signature=(*,symbol=None,limit=100,offset=0))]
    fn user_orders<'py>(
        &self,
        py: Python<'py>,
        symbol: Option<String>,
        limit: u32,
        offset: u32,
    ) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.user_orders(q::UserOrdersRequest {
                symbol,
                limit,
                offset,
            })
            .await
            .map_err(errors::qfex)
        })
    }
    fn add_order<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let mut v: serde_json::Value = convert::parse(request)?;
        for key in ["quantity", "price"] {
            if let Some(s) = v.get(key).and_then(serde_json::Value::as_str) {
                let n = s.parse::<serde_json::Number>().map_err(|_| {
                    errors::ValidationError::new_err("expected a decimal quantity/price")
                })?;
                v[key] = serde_json::Value::Number(n);
            }
        }
        let r: q::AddOrder = serde_json::from_value(v)
            .map_err(|_| errors::ValidationError::new_err("invalid native QFEX order"))?;
        r.validate()
            .map_err(|e| errors::qfex(aux_rs::Error::Exchange(e)))?;
        let c = self.state.client()?.authenticated()?;
        self.state.run(
            py,
            async move { c.add_order(r).await.map_err(errors::qfex) },
        )
    }
    fn cancel_order<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: q::CancelOrder = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state.run(
            py,
            async move { c.cancel_order(r).await.map_err(errors::qfex) },
        )
    }
    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        channel: &str,
        symbol: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let channel: q::Channel = serde_json::from_value(serde_json::json!(channel))
            .map_err(|_| errors::ValidationError::new_err("unknown QFEX channel"))?;
        let c = self.state.client()?.public;
        self.state.stream(py, async move {
            c.subscribe(q::Subscription::new(channel, symbol))
                .await
                .map(NativeHandle::Qfex)
                .map_err(errors::qfex)
        })
    }
    fn subscribe_account<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.authenticated()?;
        self.state.stream(py, async move {
            let mut s = c.trading_session().await.map_err(errors::qfex)?;
            s.subscribe_account().await.map_err(errors::qfex)?;
            Ok(NativeHandle::QfexTrade(s))
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn __repr__(&self) -> &'static str {
        "QfexClient()"
    }
}
impl Drop for QfexClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
