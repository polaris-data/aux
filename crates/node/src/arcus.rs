use crate::prelude::*;
use crate::{
    convert, errors,
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::StandardTransport;
use aux_rs::arcus as a;
use aux_rs::hex;
use std::sync::Arc;
use zeroize::Zeroizing;
pub struct ArcusSigner {
    inner: Arc<a::LocalSigner>,
}
impl ArcusSigner {
    pub(crate) fn new(seed: String) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(
                a::LocalSigner::from_hex(Zeroizing::new(seed))
                    .map_err(|e| errors::arcus(aux_rs::Error::Exchange(e)))?,
            ),
        })
    }
    pub(crate) fn public_key(&self) -> String {
        use a::Signer;
        hex::encode(self.inner.public_key())
    }
    pub(crate) fn __repr__(&self) -> &'static str {
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
    pub(crate) fn authenticated(&self) -> Result<Private> {
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
pub struct ArcusClient {
    state: Arc<ClientState<Clients>>,
}
impl ArcusClient {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        signer: Option<&ArcusSigner>,
        address: Option<String>,
        account_index: u8,
        timeout_ms: u64,
        subscription_capacity: usize,
    ) -> Result<Self> {
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
        let public = state::build(|| a::ArcusClient::with_config(exchange, config))?;
        let private = auth.map(|a| public.clone().with_auth(a));
        Ok(Self {
            state: ClientState::new(Clients { public, private }),
        })
    }
    pub(crate) fn closed(&self) -> bool {
        self.state.is_closed()
    }
    pub(crate) fn authenticated(&self) -> Result<bool> {
        Ok(self.state.client()?.private.is_some())
    }
    pub(crate) async fn markets(&self, market: Option<String>) -> Result<Output> {
        let c = self.state.client()?.public;
        self.state
            .run(async move {
                c.markets(a::MarketsRequest { market })
                    .await
                    .map_err(errors::arcus)
            })
            .await
    }
    pub(crate) async fn order_book(&self, market: String, n_levels: u8) -> Result<Output> {
        let c = self.state.client()?.public;
        self.state
            .run(async move {
                c.order_book(a::OrderBookRequest { market, n_levels })
                    .await
                    .map_err(errors::arcus)
            })
            .await
    }
    pub(crate) async fn trades(&self, market: String, limit: u16) -> Result<Output> {
        let c = self.state.client()?.public;
        self.state
            .run(async move {
                c.trades(a::TradesRequest { market, limit })
                    .await
                    .map_err(errors::arcus)
            })
            .await
    }
    pub(crate) async fn account(&self, address: String, account_index: u8) -> Result<Output> {
        let r = a::AccountRequest::new(address, account_index)
            .map_err(|e| errors::arcus(aux_rs::Error::Exchange(e)))?;
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.account(r).await.map_err(errors::arcus) })
            .await
    }
    pub(crate) async fn positions(&self, address: String, account_index: u8) -> Result<Output> {
        let r = a::AccountRequest::new(address, account_index)
            .map_err(|e| errors::arcus(aux_rs::Error::Exchange(e)))?;
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.positions(r).await.map_err(errors::arcus) })
            .await
    }
    pub(crate) async fn open_orders(&self, address: String, account_index: u8) -> Result<Output> {
        let r = a::AccountRequest::new(address, account_index)
            .map_err(|e| errors::arcus(aux_rs::Error::Exchange(e)))?;
        let c = self.state.client()?.public;
        self.state
            .run(async move { c.open_orders(r).await.map_err(errors::arcus) })
            .await
    }
    pub(crate) async fn place_order(&self, market: &Input, request: &Input) -> Result<Output> {
        let m: a::MarketInfo = convert::parse(market)?;
        let r: a::OrderRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.place_order(&m, r).await.map_err(errors::arcus) })
            .await
    }
    pub(crate) async fn cancel_order(&self, request: &Input) -> Result<Output> {
        let r: a::CancelOrderRequest = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.cancel_order(r).await.map_err(errors::arcus) })
            .await
    }
    pub(crate) async fn subscribe(&self, specification: &Input) -> Result<Subscription> {
        let r: a::Subscription = convert::parse(specification)?;
        r.validate()
            .map_err(|e| errors::arcus(aux_rs::Error::Exchange(e)))?;
        let c = self.state.client()?.public;
        self.state
            .stream(async move {
                c.subscribe(r)
                    .await
                    .map(NativeHandle::Arcus)
                    .map_err(errors::arcus)
            })
            .await
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
    }
    pub(crate) fn __repr__(&self) -> &'static str {
        "ArcusClient()"
    }
}
impl Drop for ArcusClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
