use crate::prelude::*;
use crate::{
    convert, errors,
    signers::callback,
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{StandardTransport, hex, monaco as m};
use m::{SessionSigner as _, WalletSigner as _};
use std::sync::{Arc, Mutex};
use zeroize::Zeroizing;
fn native(e: m::Error) -> crate::errors::Error {
    errors::monaco(aux_rs::Error::Exchange(e))
}
fn signing(value: &str) -> Result<m::RequestSigning> {
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
        callback: Callback,
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
        self.sign_in(message).await
    }
}
impl Session {
    pub(crate) async fn sign_in(&self, message: &[u8]) -> Result<[u8; 64], m::Error> {
        match &*self.0 {
            SessionKind::Local(s) => s.sign(message).await,
            SessionKind::Remote { callback: cb, .. } => {
                let input =
                    context::evaluate(|| convert::output(message).expect("bytes serialize"));
                let value = callback(cb, input)
                    .await
                    .map_err(|_| m::Error::Signing("session callback failed"))?;
                let value = context::evaluate(|| convert::parse::<String>(&value))
                    .map_err(|_| m::Error::Signing("callback must return a hex signature"))?;
                let mut result = [0; 64];
                hex::decode_to_slice(value.strip_prefix("0x").unwrap_or(&value), &mut result)
                    .map_err(|_| m::Error::Signing("invalid session signature"))?;
                Ok(result)
            }
        }
    }
}
#[derive(Clone)]
pub struct MonacoSessionSigner {
    inner: Session,
}
impl MonacoSessionSigner {
    pub(crate) fn new(seed: Option<String>) -> Result<Self> {
        let signer = match seed {
            Some(seed) => m::LocalSessionSigner::from_hex(Zeroizing::new(seed)),
            None => m::LocalSessionSigner::generate(),
        }
        .map_err(native)?;
        Ok(Self {
            inner: Session(Arc::new(SessionKind::Local(signer))),
        })
    }
    pub(crate) fn remote(public_key: &str, callback: Callback) -> Result<Self> {
        let mut public = [0; 32];
        hex::decode_to_slice(public_key, &mut public).map_err(|_| {
            errors::ValidationError::new_err("expected a 32-byte session public key")
        })?;
        Ok(Self {
            inner: Session(Arc::new(SessionKind::Remote { public, callback })),
        })
    }
    pub(crate) fn public_key(&self) -> String {
        hex::encode(self.inner.public_key())
    }
    /// Explicit secret export; the client never persists or includes this in responses.
    pub(crate) fn export_seed(&self) -> Result<String> {
        match &*self.inner.0 {
            SessionKind::Local(s) => Ok(hex::encode(*s.seed())),
            _ => Err(errors::SigningError::new_err(
                "remote signer has no local seed",
            )),
        }
    }
    pub(crate) async fn sign(&self, message: Vec<u8>) -> Result<Output> {
        let signer = self.inner.clone();
        convert::future_output(async move {
            signer.sign(&message).await.map(hex::encode).map_err(native)
        })
        .await
    }
    pub(crate) fn __repr__(&self) -> &'static str {
        "MonacoSessionSigner()"
    }
}
enum WalletKind {
    Local(m::LocalWalletSigner),
    Remote { address: String, callback: Callback },
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
                    context::evaluate(|| convert::output(message).expect("string serializes"));
                let result = callback(cb, input)
                    .await
                    .map_err(|_| m::Error::Signing("wallet callback failed"))?;
                context::evaluate(|| convert::parse::<String>(&result))
                    .map_err(|_| m::Error::Signing("wallet callback must return hex"))
            }
        }
    }
}
#[derive(Clone)]
pub struct MonacoWalletSigner {
    inner: Wallet,
}
impl MonacoWalletSigner {
    pub(crate) fn new(private_key: String) -> Result<Self> {
        Ok(Self {
            inner: Wallet(Arc::new(WalletKind::Local(
                m::LocalWalletSigner::from_hex(Zeroizing::new(private_key)).map_err(native)?,
            ))),
        })
    }
    pub(crate) fn remote(address: &str, callback: Callback) -> Result<Self> {
        Ok(Self {
            inner: Wallet(Arc::new(WalletKind::Remote {
                address: m::wallet_address(address).map_err(native)?,
                callback,
            })),
        })
    }
    pub(crate) fn address(&self) -> String {
        self.inner.address()
    }
    pub(crate) async fn sign_message(&self, message: String) -> Result<Output> {
        let signer = self.inner.clone();
        convert::future_output(async move { signer.sign_message(&message).await.map_err(native) })
            .await
    }
    pub(crate) fn __repr__(&self) -> &'static str {
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
    pub(crate) fn authenticated(&self) -> Result<Private> {
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
    pub(crate) fn replace(&self, client: Private) {
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
pub struct MonacoClient {
    state: Arc<ClientState<Clients>>,
}
impl MonacoClient {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
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
    ) -> Result<Self> {
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
        let public = state::build(|| m::MonacoClient::with_config(exchange, config))?;
        Ok(Self {
            state: ClientState::new(Clients {
                public,
                private: Arc::new(Mutex::new(None)),
            }),
        })
    }
    pub(crate) fn closed(&self) -> bool {
        self.state.is_closed()
    }
    pub(crate) fn authenticated(&self) -> Result<bool> {
        Ok(self.state.client()?.authenticated().is_ok())
    }
    pub(crate) fn session_info(&self) -> Result<Output> {
        convert::output(self.state.client()?.authenticated()?.auth().session_info())
    }
    pub(crate) fn session_signer(&self) -> Result<MonacoSessionSigner> {
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
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn login(
        &self,
        wallet: &MonacoWalletSigner,
        client_id: String,
        session_signer: Option<&MonacoSessionSigner>,
        invite_code: Option<String>,
        referral_code: Option<String>,
        request_signing: &str,
    ) -> Result<Output> {
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
        self.state
            .run(async move {
                let auth = c
                    .public
                    .login(&wallet, client_id, signer, options)
                    .await
                    .map_err(errors::monaco)?;
                let info = auth.session_info();
                c.replace(c.public.clone().with_auth(auth));
                Ok(info)
            })
            .await
    }
    pub(crate) fn restore_session(
        &self,
        signer: &MonacoSessionSigner,
        info: &Input,
        request_signing: &str,
    ) -> Result<()> {
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
    pub(crate) async fn refresh_session(&self) -> Result<Output> {
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.refresh_session().await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn revoke_session(&self) -> Result<Output> {
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.revoke_session().await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn create_challenge(&self, request: &Input) -> Result<Output> {
        let r: m::ChallengeRequest = convert::parse(request)?;
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.create_challenge(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn verify_signature(&self, request: &Input) -> Result<Output> {
        let r: m::VerifyRequest = convert::parse(request)?;
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.verify_signature(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn trading_pairs(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::TradingPairsRequest =
            request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.trading_pairs(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn trading_pair(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::TradingPairRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.trading_pair(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn market_metadata(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::MarketMetadataRequest =
            request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.market_metadata(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn order_book(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::OrderBookRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.order_book(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn trades(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::TradesRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.trades(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn mark_price(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::MarkPriceRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.mark_price(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn index_price(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::IndexPriceRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.index_price(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn funding(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::FundingRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.funding(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn funding_history(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::FundingHistoryRequest =
            request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.funding_history(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn balances(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::BalancesRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.balances(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn parent_margin_account(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::ParentMarginAccountRequest =
            request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.parent_margin_account(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn margin_accounts(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::MarginAccountsRequest =
            request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.margin_accounts(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn positions(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::PositionsRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.positions(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn orders(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::OrdersRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.orders(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn order(&self, request: Option<&Input>) -> Result<Output> {
        let r: m::OrderRequest = request.map(convert::parse).transpose()?.unwrap_or_default();
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.order(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn place_order(&self, market: &Input, request: &Input) -> Result<Output> {
        let market: m::TradingPairData = convert::parse(market)?;
        let r: m::CreateOrderRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.place_order(&market, r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn replace_order(
        &self,
        market: &Input,
        order_id: String,
        request: &Input,
    ) -> Result<Output> {
        let market: m::TradingPairData = convert::parse(market)?;
        let r: m::ReplaceOrderRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move {
                c.replace_order(&market, &order_id, r)
                    .await
                    .map_err(errors::monaco)
            })
            .await
    }
    pub(crate) async fn cancel_order(&self, request: &Input) -> Result<Output> {
        let r: m::CancelOrderRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.cancel_order(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn batch_create(&self, markets: &Input, request: &Input) -> Result<Output> {
        let markets: Vec<m::TradingPairData> = convert::parse(markets)?;
        let r: m::BatchCreateOrdersRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.batch_create(&markets, r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn batch_replace(&self, markets: &Input, request: &Input) -> Result<Output> {
        let markets: Vec<m::TradingPairData> = convert::parse(markets)?;
        let r: m::BatchReplaceOrdersRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.batch_replace(&markets, r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn batch_cancel(&self, request: &Input) -> Result<Output> {
        let r: m::BatchCancelOrdersRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.batch_cancel(r).await.map_err(errors::monaco) })
            .await
    }
    pub(crate) async fn cancel_all(&self, trading_pair_id: Option<String>) -> Result<Output> {
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move {
                c.cancel_all(trading_pair_id.as_deref())
                    .await
                    .map_err(errors::monaco)
            })
            .await
    }
    pub(crate) async fn subscribe(&self, specification: &Input) -> Result<Subscription> {
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
            let scoped = c.auth().signer().clone();
            let auth = c.auth().with_signer(scoped).map_err(native)?;
            let c = c.with_auth(auth);
            self.state
                .stream(async move {
                    c.subscribe_account(r)
                        .await
                        .map(NativeHandle::Monaco)
                        .map_err(errors::monaco)
                })
                .await
        } else {
            let r: m::Subscription = serde_json::from_value(value).map_err(|_| {
                errors::ValidationError::new_err("invalid Monaco public subscription")
            })?;
            self.state
                .stream(async move {
                    c.public
                        .subscribe(r)
                        .await
                        .map(NativeHandle::Monaco)
                        .map_err(errors::monaco)
                })
                .await
        }
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
    }
    pub(crate) fn __repr__(&self) -> &'static str {
        "MonacoClient()"
    }
}
impl Drop for MonacoClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
