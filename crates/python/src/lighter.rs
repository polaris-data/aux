use crate::{
    convert, errors,
    signers::{self, LighterSigner, LtSigner},
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{
    Endpoint, StandardTransport,
    lighter::{self as lt},
};
use pyo3::prelude::*;
use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone)]
struct Clients {
    public: lt::LighterClient<StandardTransport>,
    private: Option<lt::LighterClient<StandardTransport, lt::LighterAuth<LtSigner>>>,
}
impl Clients {
    fn authenticated(
        &self,
    ) -> PyResult<lt::LighterClient<StandardTransport, lt::LighterAuth<LtSigner>>> {
        self.private.clone().ok_or_else(|| {
            errors::AuthenticationError::new_err(
                "configure a Lighter signer and account/API-key indexes",
            )
        })
    }
}
impl state::ClientResources for Clients {
    fn transport(&self) -> &StandardTransport {
        self.public.transport()
    }
}
#[pyclass(module = "aux_py._native", frozen)]
pub struct LighterClient {
    state: Arc<ClientState<Clients>>,
}
#[pymethods]
impl LighterClient {
    #[new]
    #[pyo3(signature=(*,network="testnet",http_url=None,ws_url=None,chain_id=None,timeout_ms=10_000,subscription_capacity=128,signer=None,account_index=None,api_key_index=None,read_retries=2,reconnect_retries=3,initial_backoff_ms=100,max_backoff_ms=2000,heartbeat_interval_ms=20_000,idle_timeout_ms=60_000,max_message_bytes=8_388_608,reconnect_stable_ms=30_000))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        chain_id: Option<u32>,
        timeout_ms: u64,
        subscription_capacity: usize,
        signer: Option<PyRef<'_, LighterSigner>>,
        account_index: Option<i64>,
        api_key_index: Option<u8>,
        read_retries: usize,
        reconnect_retries: usize,
        initial_backoff_ms: u64,
        max_backoff_ms: u64,
        heartbeat_interval_ms: u64,
        idle_timeout_ms: u64,
        max_message_bytes: usize,
        reconnect_stable_ms: u64,
    ) -> PyResult<Self> {
        let chain = signers::lt_chain(network, chain_id)?;
        let exchange = match (http_url, ws_url) {
            (Some(http), Some(ws)) => lt::Lighter::custom(http, ws, chain),
            (None, None) => {
                if chain_id.is_some() {
                    return Err(errors::ValidationError::new_err(
                        "chain_id overrides require custom endpoints",
                    ));
                }
                match network {
                    "testnet" => lt::Lighter::testnet(),
                    _ => lt::Lighter::mainnet(),
                }
            }
            _ => {
                return Err(errors::ValidationError::new_err(
                    "provide both http_url and ws_url for custom endpoints",
                ));
            }
        };
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
        let auth = match (signer, account_index, api_key_index) {
            (Some(signer), Some(account), Some(key)) => Some(
                lt::LighterAuth::new(signer.inner.clone(), chain, account, key)
                    .map_err(errors::lt_native)?,
            ),
            (None, None, None) => None,
            _ => {
                return Err(errors::AuthenticationError::new_err(
                    "provide signer, account_index and api_key_index together",
                ));
            }
        };
        let public = state::build(py, || {
            lt::LighterClient::with_config(exchange.clone(), config)
        })?;
        let private = auth.map(|auth| public.clone().with_auth(auth));
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
    #[pyo3(signature=(*,market_id=None))]
    fn order_books<'py>(
        &self,
        py: Python<'py>,
        market_id: Option<i64>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let p = self.state.client()?.public;
        let request = lt::OrderBooksRequest { market_id };
        request.request(p.exchange()).map_err(errors::lt_native)?;
        self.state.run(py, async move {
            p.order_books(request).await.map_err(errors::lt)
        })
    }
    #[pyo3(signature=(market_id,*,limit=50))]
    fn order_book_orders<'py>(
        &self,
        py: Python<'py>,
        market_id: i64,
        limit: u16,
    ) -> PyResult<Bound<'py, PyAny>> {
        let p = self.state.client()?.public;
        let request = lt::OrderBookOrdersRequest { market_id, limit };
        request.request(p.exchange()).map_err(errors::lt_native)?;
        self.state.run(py, async move {
            p.order_book_orders(request).await.map_err(errors::lt)
        })
    }
    #[pyo3(signature=(market_id,*,limit=50))]
    fn recent_trades<'py>(
        &self,
        py: Python<'py>,
        market_id: i64,
        limit: u16,
    ) -> PyResult<Bound<'py, PyAny>> {
        let p = self.state.client()?.public;
        let request = lt::RecentTradesRequest { market_id, limit };
        request.request(p.exchange()).map_err(errors::lt_native)?;
        self.state.run(py, async move {
            p.recent_trades(request).await.map_err(errors::lt)
        })
    }
    #[pyo3(signature=(*,account_index=None,l1_address=None))]
    fn account<'py>(
        &self,
        py: Python<'py>,
        account_index: Option<i64>,
        l1_address: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = match (account_index, l1_address) {
            (Some(index), None) => lt::AccountRequest::Index(index),
            (None, Some(address)) => lt::AccountRequest::L1Address(address),
            _ => {
                return Err(errors::ValidationError::new_err(
                    "provide exactly one account_index or l1_address",
                ));
            }
        };
        let p = self.state.client()?.public;
        request.request(p.exchange()).map_err(errors::lt_native)?;
        self.state.run(
            py,
            async move { p.account(request).await.map_err(errors::lt) },
        )
    }
    fn next_nonce<'py>(
        &self,
        py: Python<'py>,
        account_index: i64,
        api_key_index: u8,
    ) -> PyResult<Bound<'py, PyAny>> {
        let p = self.state.client()?.public;
        let request = lt::NextNonceRequest {
            account_index,
            api_key_index,
        };
        request.request(p.exchange()).map_err(errors::lt_native)?;
        self.state.run(
            py,
            async move { p.request(request).await.map_err(errors::lt) },
        )
    }
    #[pyo3(signature=(*,market_id=None))]
    fn active_orders<'py>(
        &self,
        py: Python<'py>,
        market_id: Option<i64>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let p = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            p.active_orders(market_id).await.map_err(errors::lt)
        })
    }
    #[pyo3(signature=(request,*,expired_at=None,attributes=None))]
    fn create_order<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'_, PyAny>,
        expired_at: Option<i64>,
        attributes: Option<BTreeMap<u8, i64>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: lt::CreateOrder = convert::parse(request)?;
        request.validate().map_err(errors::lt_native)?;
        let options = signers::options(expired_at, attributes)?;
        let p = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            p.create_order(request, options).await.map_err(errors::lt)
        })
    }
    #[pyo3(signature=(request,*,expired_at=None,attributes=None))]
    fn cancel_order<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'_, PyAny>,
        expired_at: Option<i64>,
        attributes: Option<BTreeMap<u8, i64>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: lt::CancelOrder = convert::parse(request)?;
        request.validate().map_err(errors::lt_native)?;
        let options = signers::options(expired_at, attributes)?;
        let p = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            p.cancel_order(request, options).await.map_err(errors::lt)
        })
    }
    fn resync_nonce<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let p = self.state.client()?.authenticated()?;
        self.state.run(
            py,
            async move { p.resync_nonce().await.map_err(errors::lt) },
        )
    }
    fn auth_token<'py>(&self, py: Python<'py>, deadline: i64) -> PyResult<Bound<'py, PyAny>> {
        let p = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            Ok(p.auth()
                .token(deadline)
                .await
                .map_err(errors::lt_native)?
                .as_str()
                .to_owned())
        })
    }
    fn subscribe_order_book<'py>(
        &self,
        py: Python<'py>,
        market_id: i64,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.subscribe_native(py, lt::Subscription::OrderBook { market_id })
    }
    fn subscribe_trades<'py>(
        &self,
        py: Python<'py>,
        market_id: i64,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.subscribe_native(py, lt::Subscription::Trades { market_id })
    }
    fn subscribe_account<'py>(
        &self,
        py: Python<'py>,
        account_index: i64,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.subscribe_native(py, lt::Subscription::Account { account_index })
    }
    fn subscribe_account_orders<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let p = self.state.client()?.authenticated()?;
        self.state.stream(py, async move {
            Ok(NativeHandle::Lighter(
                p.subscribe_account_orders().await.map_err(errors::lt)?,
            ))
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
}
impl LighterClient {
    fn subscribe_native<'py>(
        &self,
        py: Python<'py>,
        spec: lt::Subscription,
    ) -> PyResult<Bound<'py, PyAny>> {
        let p = self.state.client()?.public;
        self.state.stream(py, async move {
            Ok(NativeHandle::Lighter(
                p.subscribe(spec).await.map_err(errors::lt)?,
            ))
        })
    }
}
impl Drop for LighterClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
