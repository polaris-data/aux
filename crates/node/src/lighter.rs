use crate::prelude::*;
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
use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone)]
struct Clients {
    public: lt::LighterClient<StandardTransport>,
    private: Option<lt::LighterClient<StandardTransport, lt::LighterAuth<LtSigner>>>,
}
impl Clients {
    pub(crate) fn authenticated(
        &self,
    ) -> Result<lt::LighterClient<StandardTransport, lt::LighterAuth<LtSigner>>> {
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
pub struct LighterClient {
    state: Arc<ClientState<Clients>>,
}
impl LighterClient {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        chain_id: Option<u32>,
        timeout_ms: u64,
        subscription_capacity: usize,
        signer: Option<&LighterSigner>,
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
    ) -> Result<Self> {
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
        let public = state::build(|| lt::LighterClient::with_config(exchange.clone(), config))?;
        let private = auth.map(|auth| public.clone().with_auth(auth));
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
    pub(crate) async fn order_books(&self, market_id: Option<i64>) -> Result<Output> {
        let p = self.state.client()?.public;
        let request = lt::OrderBooksRequest { market_id };
        request.request(p.exchange()).map_err(errors::lt_native)?;
        self.state
            .run(async move { p.order_books(request).await.map_err(errors::lt) })
            .await
    }
    pub(crate) async fn order_book_orders(&self, market_id: i64, limit: u16) -> Result<Output> {
        let p = self.state.client()?.public;
        let request = lt::OrderBookOrdersRequest { market_id, limit };
        request.request(p.exchange()).map_err(errors::lt_native)?;
        self.state
            .run(async move { p.order_book_orders(request).await.map_err(errors::lt) })
            .await
    }
    pub(crate) async fn recent_trades(&self, market_id: i64, limit: u16) -> Result<Output> {
        let p = self.state.client()?.public;
        let request = lt::RecentTradesRequest { market_id, limit };
        request.request(p.exchange()).map_err(errors::lt_native)?;
        self.state
            .run(async move { p.recent_trades(request).await.map_err(errors::lt) })
            .await
    }
    pub(crate) async fn account(
        &self,
        account_index: Option<i64>,
        l1_address: Option<String>,
    ) -> Result<Output> {
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
        self.state
            .run(async move { p.account(request).await.map_err(errors::lt) })
            .await
    }
    pub(crate) async fn next_nonce(&self, account_index: i64, api_key_index: u8) -> Result<Output> {
        let p = self.state.client()?.public;
        let request = lt::NextNonceRequest {
            account_index,
            api_key_index,
        };
        request.request(p.exchange()).map_err(errors::lt_native)?;
        self.state
            .run(async move { p.request(request).await.map_err(errors::lt) })
            .await
    }
    pub(crate) async fn active_orders(&self, market_id: Option<i64>) -> Result<Output> {
        let p = self.state.client()?.authenticated()?;
        self.state
            .run(async move { p.active_orders(market_id).await.map_err(errors::lt) })
            .await
    }
    pub(crate) async fn create_order(
        &self,
        request: &Input,
        expired_at: Option<i64>,
        attributes: Option<BTreeMap<u8, i64>>,
    ) -> Result<Output> {
        let request: lt::CreateOrder = convert::parse(request)?;
        request.validate().map_err(errors::lt_native)?;
        let options = signers::options(expired_at, attributes)?;
        let p = self.state.client()?.authenticated()?;
        self.state
            .run(async move { p.create_order(request, options).await.map_err(errors::lt) })
            .await
    }
    pub(crate) async fn cancel_order(
        &self,
        request: &Input,
        expired_at: Option<i64>,
        attributes: Option<BTreeMap<u8, i64>>,
    ) -> Result<Output> {
        let request: lt::CancelOrder = convert::parse(request)?;
        request.validate().map_err(errors::lt_native)?;
        let options = signers::options(expired_at, attributes)?;
        let p = self.state.client()?.authenticated()?;
        self.state
            .run(async move { p.cancel_order(request, options).await.map_err(errors::lt) })
            .await
    }
    pub(crate) async fn resync_nonce(&self) -> Result<Output> {
        let p = self.state.client()?.authenticated()?;
        self.state
            .run(async move { p.resync_nonce().await.map_err(errors::lt) })
            .await
    }
    pub(crate) async fn auth_token(&self, deadline: i64) -> Result<Output> {
        let p = self.state.client()?.authenticated()?;
        self.state
            .run(async move {
                Ok(p.auth()
                    .token(deadline)
                    .await
                    .map_err(errors::lt_native)?
                    .as_str()
                    .to_owned())
            })
            .await
    }
    pub(crate) async fn subscribe_order_book(&self, market_id: i64) -> Result<Subscription> {
        self.subscribe_native(lt::Subscription::OrderBook { market_id })
            .await
    }
    pub(crate) async fn subscribe_trades(&self, market_id: i64) -> Result<Subscription> {
        self.subscribe_native(lt::Subscription::Trades { market_id })
            .await
    }
    pub(crate) async fn subscribe_account(&self, account_index: i64) -> Result<Subscription> {
        self.subscribe_native(lt::Subscription::Account { account_index })
            .await
    }
    pub(crate) async fn subscribe_account_orders(&self) -> Result<Subscription> {
        let p = self.state.client()?.authenticated()?;
        self.state
            .stream(async move {
                Ok(NativeHandle::Lighter(
                    p.subscribe_account_orders().await.map_err(errors::lt)?,
                ))
            })
            .await
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
    }
}
impl LighterClient {
    pub(crate) async fn subscribe_native(&self, spec: lt::Subscription) -> Result<Subscription> {
        let p = self.state.client()?.public;
        self.state
            .stream(async move {
                Ok(NativeHandle::Lighter(
                    p.subscribe(spec).await.map_err(errors::lt)?,
                ))
            })
            .await
    }
}
impl Drop for LighterClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
