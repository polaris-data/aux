use crate::{
    convert, errors,
    signers::{self, HlSigner, HyperliquidSigner},
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{
    StandardTransport,
    hyperliquid::{self as hl},
};
use pyo3::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
struct Clients {
    public: hl::HyperliquidClient<StandardTransport>,
    private: Option<hl::HyperliquidClient<StandardTransport, hl::HyperliquidAuth<HlSigner>>>,
}
impl Clients {
    fn authenticated(
        &self,
    ) -> PyResult<hl::HyperliquidClient<StandardTransport, hl::HyperliquidAuth<HlSigner>>> {
        self.private.clone().ok_or_else(|| {
            errors::AuthenticationError::new_err("configure a Hyperliquid signer for trading")
        })
    }
}
impl state::ClientResources for Clients {
    fn transport(&self) -> &StandardTransport {
        self.public.transport()
    }
}
#[pyclass(module = "aux_py._native", frozen)]
pub struct HyperliquidClient {
    state: Arc<ClientState<Clients>>,
}
#[pymethods]
impl HyperliquidClient {
    #[new]
    #[pyo3(signature=(*,network="testnet",http_url=None,ws_url=None,timeout_ms=10_000,subscription_capacity=128,signer=None,account_address=None,vault_address=None,read_retries=2,reconnect_retries=3,initial_backoff_ms=100,max_backoff_ms=2000,heartbeat_interval_ms=20_000,idle_timeout_ms=60_000,max_message_bytes=8_388_608,reconnect_stable_ms=30_000))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        timeout_ms: u64,
        subscription_capacity: usize,
        signer: Option<PyRef<'_, HyperliquidSigner>>,
        account_address: Option<&str>,
        vault_address: Option<&str>,
        read_retries: usize,
        reconnect_retries: usize,
        initial_backoff_ms: u64,
        max_backoff_ms: u64,
        heartbeat_interval_ms: u64,
        idle_timeout_ms: u64,
        max_message_bytes: usize,
        reconnect_stable_ms: u64,
    ) -> PyResult<Self> {
        let network = signers::hl_network(network)?;
        let exchange = match (http_url, ws_url) {
            (Some(http), Some(ws)) => hl::Hyperliquid::custom(http, ws).with_network(network),
            (None, None) => match network {
                hl::Network::Testnet => hl::Hyperliquid::testnet(),
                hl::Network::Mainnet => hl::Hyperliquid::mainnet(),
            },
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
        let auth = match signer {
            Some(signer) => {
                let mut auth = match account_address {
                    Some(account) => {
                        hl::HyperliquidAuth::agent(signer.inner.clone(), signers::address(account)?)
                    }
                    None => hl::HyperliquidAuth::direct(signer.inner.clone()),
                };
                if let Some(vault) = vault_address {
                    auth = auth.with_vault(signers::address(vault)?);
                }
                Some(auth)
            }
            None => {
                if account_address.is_some() || vault_address.is_some() {
                    return Err(errors::AuthenticationError::new_err(
                        "account/vault trading context requires a signer; public queries accept explicit account addresses",
                    ));
                }
                None
            }
        };
        let public = state::build(py, || {
            hl::HyperliquidClient::with_config(exchange.clone(), config)
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
    #[getter]
    fn signer_address(&self) -> PyResult<Option<String>> {
        Ok(self
            .state
            .client()?
            .private
            .map(|p| p.auth().signer_address().to_string()))
    }
    #[getter]
    fn account_address(&self) -> PyResult<Option<String>> {
        Ok(self
            .state
            .client()?
            .private
            .map(|p| p.auth().account_address().to_string()))
    }
    #[pyo3(signature=(*,dex=None))]
    fn meta<'py>(&self, py: Python<'py>, dex: Option<String>) -> PyResult<Bound<'py, PyAny>> {
        let p = self.state.client()?.public;
        self.state.run(py, async move {
            p.meta_with(hl::MetaRequest { dex })
                .await
                .map_err(errors::hl)
        })
    }
    #[pyo3(signature=(coin,*,n_sig_figs=None,mantissa=None))]
    fn l2_book<'py>(
        &self,
        py: Python<'py>,
        coin: String,
        n_sig_figs: Option<u8>,
        mantissa: Option<u8>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = hl::L2BookRequest {
            coin,
            n_sig_figs,
            mantissa,
        };
        request.validate().map_err(errors::hl_native)?;
        let p = self.state.client()?.public;
        self.state.run(
            py,
            async move { p.l2_book(request).await.map_err(errors::hl) },
        )
    }
    #[pyo3(signature=(user,*,dex=None))]
    fn clearinghouse_state<'py>(
        &self,
        py: Python<'py>,
        user: &str,
        dex: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = hl::UserStateRequest {
            user: signers::address(user)?,
            dex,
        };
        let p = self.state.client()?.public;
        self.state.run(py, async move {
            p.clearinghouse_state(request).await.map_err(errors::hl)
        })
    }
    #[pyo3(signature=(user,*,dex=None))]
    fn open_orders<'py>(
        &self,
        py: Python<'py>,
        user: &str,
        dex: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = hl::OpenOrdersRequest {
            user: signers::address(user)?,
            dex,
        };
        let p = self.state.client()?.public;
        self.state.run(py, async move {
            p.open_orders(request).await.map_err(errors::hl)
        })
    }
    #[pyo3(signature=(request,*,nonce=None,expires_after=None))]
    fn order<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'_, PyAny>,
        nonce: Option<u64>,
        expires_after: Option<u64>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: hl::OrderRequest = convert::parse(request)?;
        hl::Action::Order(request.clone())
            .validate()
            .map_err(errors::hl_native)?;
        let p = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            p.order_with_options(
                request,
                hl::ExecutionOptions {
                    nonce,
                    expires_after,
                },
            )
            .await
            .map_err(errors::hl)
        })
    }
    #[pyo3(signature=(request,*,nonce=None,expires_after=None))]
    fn cancel<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'_, PyAny>,
        nonce: Option<u64>,
        expires_after: Option<u64>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: hl::CancelRequest = convert::parse(request)?;
        hl::Action::Cancel(request.clone())
            .validate()
            .map_err(errors::hl_native)?;
        let p = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            p.cancel_with_options(
                request,
                hl::ExecutionOptions {
                    nonce,
                    expires_after,
                },
            )
            .await
            .map_err(errors::hl)
        })
    }
    #[pyo3(signature=(request,*,nonce=None,expires_after=None))]
    fn cancel_by_cloid<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'_, PyAny>,
        nonce: Option<u64>,
        expires_after: Option<u64>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request: hl::CancelByCloidRequest = convert::parse(request)?;
        hl::Action::CancelByCloid(request.clone())
            .validate()
            .map_err(errors::hl_native)?;
        let p = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            p.cancel_by_cloid_with_options(
                request,
                hl::ExecutionOptions {
                    nonce,
                    expires_after,
                },
            )
            .await
            .map_err(errors::hl)
        })
    }
    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        spec: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let spec: hl::Subscription = convert::parse(spec)?;
        spec.validate().map_err(errors::hl_native)?;
        let p = self.state.client()?.public;
        self.state.stream(py, async move {
            Ok(NativeHandle::Hyperliquid {
                handle: p.subscribe(spec).await.map_err(errors::hl)?,
                book_only: false,
            })
        })
    }
    fn subscribe_l2_book<'py>(&self, py: Python<'py>, coin: String) -> PyResult<Bound<'py, PyAny>> {
        let spec = hl::Subscription::L2Book { coin };
        spec.validate().map_err(errors::hl_native)?;
        let p = self.state.client()?.public;
        self.state.stream(py, async move {
            Ok(NativeHandle::Hyperliquid {
                handle: p.subscribe(spec).await.map_err(errors::hl)?,
                book_only: true,
            })
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
}
impl Drop for HyperliquidClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
