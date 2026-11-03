use crate::{
    convert, errors,
    signers::{callback, callback_with_locals},
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{StandardTransport, hex, monaco as m};
use m::{SessionSigner as _, WalletSigner as _};
use pyo3::{prelude::*, types::PyBytes};
use std::sync::{Arc, Mutex};
use zeroize::Zeroizing;
fn native(e: m::Error) -> PyErr {
    errors::monaco(aux_rs::Error::Exchange(e))
}
fn signing(value: &str) -> PyResult<m::RequestSigning> {
    match value {
        "legacy" => Ok(m::RequestSigning::Legacy),
        "rfc9421" => Ok(m::RequestSigning::Rfc9421),
        _ => Err(errors::ValidationError::new_err(
            "request_signing must be legacy or rfc9421",
        )),
    }
}
enum SessionKind {
    Local(m::LocalSessionSigner),
    Remote {
        public: [u8; 32],
        callback: Py<PyAny>,
    },
}
#[derive(Clone)]
pub(crate) struct Session(Arc<SessionKind>);
impl m::SessionSigner for Session {
    fn public_key(&self) -> [u8; 32] {
        match &*self.0 {
            SessionKind::Local(s) => s.public_key(),
            SessionKind::Remote { public, .. } => *public,
        }
    }
    async fn sign(&self, message: &[u8]) -> Result<[u8; 64], m::Error> {
        self.sign_in(message, None).await
    }
}
impl Session {
    async fn sign_in(
        &self,
        message: &[u8],
        locals: Option<&pyo3_async_runtimes::TaskLocals>,
    ) -> Result<[u8; 64], m::Error> {
        match &*self.0 {
            SessionKind::Local(s) => s.sign(message).await,
            SessionKind::Remote { callback: cb, .. } => {
                let input = Python::attach(|py| PyBytes::new(py, message).into_any().unbind());
                let value = callback_with_locals(cb, input, locals)
                    .await
                    .map_err(|_| m::Error::Signing("session callback failed"))?;
                let value = Python::attach(|py| value.extract::<String>(py))
                    .map_err(|_| m::Error::Signing("callback must return a hex signature"))?;
                let mut result = [0; 64];
                hex::decode_to_slice(value.strip_prefix("0x").unwrap_or(&value), &mut result)
                    .map_err(|_| m::Error::Signing("invalid session signature"))?;
                Ok(result)
            }
        }
    }
}
// The transport driver runs outside the initiating Rust/Python task. Retain
// the subscribe call's asyncio loop/context for its initial and reconnect auth.
struct ScopedSession {
    session: Session,
    locals: pyo3_async_runtimes::TaskLocals,
}
impl m::SessionSigner for ScopedSession {
    fn public_key(&self) -> [u8; 32] {
        self.session.public_key()
    }
    async fn sign(&self, message: &[u8]) -> Result<[u8; 64], m::Error> {
        self.session.sign_in(message, Some(&self.locals)).await
    }
}
#[pyclass(module = "aux_py._native", frozen)]
#[derive(Clone)]
pub struct MonacoSessionSigner {
    inner: Session,
}
#[pymethods]
impl MonacoSessionSigner {
    #[new]
    #[pyo3(signature=(seed=None))]
    fn new(seed: Option<String>) -> PyResult<Self> {
        let signer = match seed {
            Some(seed) => m::LocalSessionSigner::from_hex(Zeroizing::new(seed)),
            None => m::LocalSessionSigner::generate(),
        }
        .map_err(native)?;
        Ok(Self {
            inner: Session(Arc::new(SessionKind::Local(signer))),
        })
    }
    #[staticmethod]
    fn remote(public_key: &str, callback: Bound<'_, PyAny>) -> PyResult<Self> {
        if !callback.is_callable() {
            return Err(errors::ValidationError::new_err(
                "signer callback must be callable",
            ));
        }
        let mut public = [0; 32];
        hex::decode_to_slice(public_key, &mut public).map_err(|_| {
            errors::ValidationError::new_err("expected a 32-byte session public key")
        })?;
        Ok(Self {
            inner: Session(Arc::new(SessionKind::Remote {
                public,
                callback: callback.unbind(),
            })),
        })
    }
    #[getter]
    fn public_key(&self) -> String {
        hex::encode(self.inner.public_key())
    }
    /// Explicit secret export; the client never persists or includes this in responses.
    fn export_seed(&self) -> PyResult<String> {
        match &*self.inner.0 {
            SessionKind::Local(s) => Ok(hex::encode(*s.seed())),
            _ => Err(errors::SigningError::new_err(
                "remote signer has no local seed",
            )),
        }
    }
    fn sign<'py>(&self, py: Python<'py>, message: Vec<u8>) -> PyResult<Bound<'py, PyAny>> {
        let signer = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            signer.sign(&message).await.map(hex::encode).map_err(native)
        })
    }
    fn __repr__(&self) -> &'static str {
        "MonacoSessionSigner()"
    }
}
enum WalletKind {
    Local(m::LocalWalletSigner),
    Remote {
        address: String,
        callback: Py<PyAny>,
    },
}
#[derive(Clone)]
struct Wallet(Arc<WalletKind>);
impl m::WalletSigner for Wallet {
    fn address(&self) -> String {
        match &*self.0 {
            WalletKind::Local(s) => s.address(),
            WalletKind::Remote { address, .. } => address.clone(),
        }
    }
    async fn sign_message(&self, message: &str) -> Result<String, m::Error> {
        match &*self.0 {
            WalletKind::Local(s) => s.sign_message(message).await,
            WalletKind::Remote { callback: cb, .. } => {
                let input =
                    Python::attach(|py| message.into_pyobject(py).unwrap().into_any().unbind());
                let result = callback(cb, input)
                    .await
                    .map_err(|_| m::Error::Signing("wallet callback failed"))?;
                Python::attach(|py| result.extract::<String>(py))
                    .map_err(|_| m::Error::Signing("wallet callback must return hex"))
            }
        }
    }
}
#[pyclass(module = "aux_py._native", frozen)]
#[derive(Clone)]
pub struct MonacoWalletSigner {
    inner: Wallet,
}
#[pymethods]
impl MonacoWalletSigner {
    #[new]
    fn new(private_key: String) -> PyResult<Self> {
        Ok(Self {
            inner: Wallet(Arc::new(WalletKind::Local(
                m::LocalWalletSigner::from_hex(Zeroizing::new(private_key)).map_err(native)?,
            ))),
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
            inner: Wallet(Arc::new(WalletKind::Remote {
                address: m::wallet_address(address).map_err(native)?,
                callback: callback.unbind(),
            })),
        })
    }
    #[getter]
    fn address(&self) -> String {
        self.inner.address()
    }
    fn sign_message<'py>(&self, py: Python<'py>, message: String) -> PyResult<Bound<'py, PyAny>> {
        let signer = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            signer.sign_message(&message).await.map_err(native)
        })
    }
    fn __repr__(&self) -> &'static str {
        "MonacoWalletSigner()"
    }
}
type Private = m::MonacoClient<StandardTransport, m::MonacoAuth<Session>>;
#[derive(Clone)]
struct Clients {
    public: m::MonacoClient<StandardTransport>,
    private: Arc<Mutex<Option<Private>>>,
}
impl state::ClientResources for Clients {
    fn transport(&self) -> &StandardTransport {
        self.public.transport()
    }
}
impl Clients {
    fn authenticated(&self) -> PyResult<Private> {
        self.private
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
            .filter(|c| c.auth().is_active())
            .ok_or_else(|| {
                errors::AuthenticationError::new_err(
                    "configure an active Monaco session or call login",
                )
            })
    }
    fn replace(&self, client: Private) {
        let old = self
            .private
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .replace(client);
        if let Some(old) = old {
            old.auth().invalidate();
        }
    }
}
#[pyclass(module = "aux_py._native", frozen)]
pub struct MonacoClient {
    state: Arc<ClientState<Clients>>,
}
#[pymethods]
impl MonacoClient {
    #[new]
    #[pyo3(signature=(*,network="testnet",http_url=None,ws_url=None,chain_id=None,timeout_ms=10_000,subscription_capacity=128,read_retries=2,reconnect_retries=3,initial_backoff_ms=100,max_backoff_ms=2000,heartbeat_interval_ms=20_000,idle_timeout_ms=60_000,max_message_bytes=8_388_608,reconnect_stable_ms=30_000))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        chain_id: Option<u64>,
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
        let exchange = match (http_url, ws_url) {
            (Some(h), Some(w)) => m::Monaco::custom(
                h,
                w,
                chain_id.ok_or_else(|| {
                    errors::ValidationError::new_err("custom endpoints require chain_id")
                })?,
            )
            .map_err(native)?,
            (None, None) => {
                if chain_id.is_some() {
                    return Err(errors::ValidationError::new_err(
                        "chain_id is only used with custom endpoints",
                    ));
                }
                match network {
                    "mainnet" => m::Monaco::mainnet(),
                    "testnet" => m::Monaco::testnet(),
                    _ => {
                        return Err(errors::ValidationError::new_err(
                            "network must be testnet or mainnet",
                        ));
                    }
                }
            }
            _ => {
                return Err(errors::ValidationError::new_err(
                    "custom endpoints require http_url and ws_url",
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
        let public = state::build(py, || m::MonacoClient::with_config(exchange, config))?;
        Ok(Self {
            state: ClientState::new(Clients {
                public,
                private: Arc::new(Mutex::new(None)),
            }),
        })
    }
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    #[getter]
    fn authenticated(&self) -> PyResult<bool> {
        Ok(self.state.client()?.authenticated().is_ok())
    }
    #[getter]
    fn session_info(&self) -> PyResult<Py<PyAny>> {
        convert::output(self.state.client()?.authenticated()?.auth().session_info())
    }
    #[getter]
    fn session_signer(&self) -> PyResult<MonacoSessionSigner> {
        Ok(MonacoSessionSigner {
            inner: self
                .state
                .client()?
                .authenticated()?
                .auth()
                .signer()
                .clone(),
        })
    }
    #[pyo3(signature=(wallet,client_id,*,session_signer=None,invite_code=None,referral_code=None,request_signing="legacy"))]
    #[allow(clippy::too_many_arguments)]
    fn login<'py>(
        &self,
        py: Python<'py>,
        wallet: PyRef<'_, MonacoWalletSigner>,
        client_id: String,
        session_signer: Option<PyRef<'_, MonacoSessionSigner>>,
        invite_code: Option<String>,
        referral_code: Option<String>,
        request_signing: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?;
        let wallet = wallet.inner.clone();
        let signer = match session_signer {
            Some(s) => s.inner.clone(),
            None => Session(Arc::new(SessionKind::Local(
                m::LocalSessionSigner::generate().map_err(native)?,
            ))),
        };
        let options = m::LoginOptions {
            invite_code,
            referral_code,
            request_signing: signing(request_signing)?,
        };
        self.state.run(py, async move {
            let auth = c
                .public
                .login(&wallet, client_id, signer, options)
                .await
                .map_err(errors::monaco)?;
            let info = auth.session_info();
            c.replace(c.public.clone().with_auth(auth));
            Ok(info)
        })
    }
    #[pyo3(signature=(signer,info,*,request_signing="legacy"))]
    fn restore_session(
        &self,
        signer: PyRef<'_, MonacoSessionSigner>,
        info: &Bound<'_, PyAny>,
        request_signing: &str,
    ) -> PyResult<()> {
        let c = self.state.client()?;
        let info: m::SessionInfo = convert::parse(info)?;
        let auth = m::MonacoAuth::restore(
            c.public.exchange().clone(),
            signer.inner.clone(),
            info,
            signing(request_signing)?,
        )
        .map_err(native)?;
        c.replace(c.public.clone().with_auth(auth));
        Ok(())
    }
    fn refresh_session<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.refresh_session().await.map_err(errors::monaco)
        })
    }
    fn revoke_session<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.revoke_session().await.map_err(errors::monaco)
        })
    }
    fn create_challenge<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::ChallengeRequest = convert::parse(request)?;
        let c = self.state.client()?.public;
        self.state.run(py, async move {
            c.create_challenge(r).await.map_err(errors::monaco)
        })
    }
    fn verify_signature<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::VerifyRequest = convert::parse(request)?;
        let c = self.state.client()?.public;
        self.state.run(py, async move {
            c.verify_signature(r).await.map_err(errors::monaco)
        })
    }
    #[pyo3(signature=(request=None))]
    fn trading_pairs<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::TradingPairsRequest =
            request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state.run(py, async move {
            c.trading_pairs(r).await.map_err(errors::monaco)
        })
    }
    #[pyo3(signature=(request=None))]
    fn trading_pair<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::TradingPairRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state.run(py, async move {
            c.trading_pair(r).await.map_err(errors::monaco)
        })
    }
    #[pyo3(signature=(request=None))]
    fn market_metadata<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::MarketMetadataRequest =
            request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state.run(py, async move {
            c.market_metadata(r).await.map_err(errors::monaco)
        })
    }
    #[pyo3(signature=(request=None))]
    fn order_book<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::OrderBookRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state.run(
            py,
            async move { c.order_book(r).await.map_err(errors::monaco) },
        )
    }
    #[pyo3(signature=(request=None))]
    fn trades<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::TradesRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state
            .run(py, async move { c.trades(r).await.map_err(errors::monaco) })
    }
    #[pyo3(signature=(request=None))]
    fn mark_price<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::MarkPriceRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state.run(
            py,
            async move { c.mark_price(r).await.map_err(errors::monaco) },
        )
    }
    #[pyo3(signature=(request=None))]
    fn index_price<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::IndexPriceRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state.run(
            py,
            async move { c.index_price(r).await.map_err(errors::monaco) },
        )
    }
    #[pyo3(signature=(request=None))]
    fn funding<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::FundingRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state.run(
            py,
            async move { c.funding(r).await.map_err(errors::monaco) },
        )
    }
    #[pyo3(signature=(request=None))]
    fn funding_history<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::FundingHistoryRequest =
            request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state.run(py, async move {
            c.funding_history(r).await.map_err(errors::monaco)
        })
    }
    #[pyo3(signature=(request=None))]
    fn balances<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::BalancesRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.authenticated()?;
        self.state.run(
            py,
            async move { c.balances(r).await.map_err(errors::monaco) },
        )
    }
    #[pyo3(signature=(request=None))]
    fn parent_margin_account<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::ParentMarginAccountRequest =
            request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.parent_margin_account(r).await.map_err(errors::monaco)
        })
    }
    #[pyo3(signature=(request=None))]
    fn margin_accounts<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::MarginAccountsRequest =
            request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.margin_accounts(r).await.map_err(errors::monaco)
        })
    }
    #[pyo3(signature=(request=None))]
    fn positions<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::PositionsRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.authenticated()?;
        self.state.run(
            py,
            async move { c.positions(r).await.map_err(errors::monaco) },
        )
    }
    #[pyo3(signature=(request=None))]
    fn orders<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::OrdersRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(py, async move { c.orders(r).await.map_err(errors::monaco) })
    }
    #[pyo3(signature=(request=None))]
    fn order<'py>(
        &self,
        py: Python<'py>,
        request: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::OrderRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(py, async move { c.order(r).await.map_err(errors::monaco) })
    }
    fn place_order<'py>(
        &self,
        py: Python<'py>,
        market: &Bound<'_, PyAny>,
        request: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let market: m::TradingPairData = convert::parse(market)?;
        let r: m::CreateOrderRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.place_order(&market, r).await.map_err(errors::monaco)
        })
    }
    fn replace_order<'py>(
        &self,
        py: Python<'py>,
        market: &Bound<'_, PyAny>,
        order_id: String,
        request: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let market: m::TradingPairData = convert::parse(market)?;
        let r: m::ReplaceOrderRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.replace_order(&market, &order_id, r)
                .await
                .map_err(errors::monaco)
        })
    }
    fn cancel_order<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::CancelOrderRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.cancel_order(r).await.map_err(errors::monaco)
        })
    }
    fn batch_create<'py>(
        &self,
        py: Python<'py>,
        markets: &Bound<'_, PyAny>,
        request: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let markets: Vec<m::TradingPairData> = convert::parse(markets)?;
        let r: m::BatchCreateOrdersRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.batch_create(&markets, r).await.map_err(errors::monaco)
        })
    }
    fn batch_replace<'py>(
        &self,
        py: Python<'py>,
        markets: &Bound<'_, PyAny>,
        request: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let markets: Vec<m::TradingPairData> = convert::parse(markets)?;
        let r: m::BatchReplaceOrdersRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.batch_replace(&markets, r).await.map_err(errors::monaco)
        })
    }
    fn batch_cancel<'py>(
        &self,
        py: Python<'py>,
        request: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let r: m::BatchCancelOrdersRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.batch_cancel(r).await.map_err(errors::monaco)
        })
    }
    #[pyo3(signature=(trading_pair_id=None))]
    fn cancel_all<'py>(
        &self,
        py: Python<'py>,
        trading_pair_id: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let c = self.state.client()?.authenticated()?;
        self.state.run(py, async move {
            c.cancel_all(trading_pair_id.as_deref())
                .await
                .map_err(errors::monaco)
        })
    }
    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        specification: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let value: serde_json::Value = convert::parse(specification)?;
        let c = self.state.client()?;
        if matches!(
            value["type"].as_str(),
            Some("orders" | "balances" | "positions" | "account")
        ) {
            let r: m::AccountSubscription = serde_json::from_value(value).map_err(|_| {
                errors::ValidationError::new_err("invalid Monaco account subscription")
            })?;
            let c = c.authenticated()?;
            let scoped = ScopedSession {
                session: c.auth().signer().clone(),
                locals: pyo3_async_runtimes::tokio::get_current_locals(py)?,
            };
            let auth = c.auth().with_signer(scoped).map_err(native)?;
            let c = c.with_auth(auth);
            self.state.stream(py, async move {
                c.subscribe_account(r)
                    .await
                    .map(NativeHandle::Monaco)
                    .map_err(errors::monaco)
            })
        } else {
            let r: m::Subscription = serde_json::from_value(value).map_err(|_| {
                errors::ValidationError::new_err("invalid Monaco public subscription")
            })?;
            self.state.stream(py, async move {
                c.public
                    .subscribe(r)
                    .await
                    .map(NativeHandle::Monaco)
                    .map_err(errors::monaco)
            })
        }
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn __repr__(&self) -> &'static str {
        "MonacoClient()"
    }
}
impl Drop for MonacoClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
