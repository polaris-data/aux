use crate::prelude::*;
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
use std::sync::Arc;

#[derive(Clone)]
struct Clients {
    public: hl::HyperliquidClient<StandardTransport>,
    private: Option<hl::HyperliquidClient<StandardTransport, hl::HyperliquidAuth<HlSigner>>>,
}
impl Clients {
    pub(crate) fn authenticated(
        &self,
    ) -> Result<hl::HyperliquidClient<StandardTransport, hl::HyperliquidAuth<HlSigner>>> {
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
pub struct HyperliquidClient {
    state: Arc<ClientState<Clients>>,
}
impl HyperliquidClient {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        timeout_ms: u64,
        subscription_capacity: usize,
        signer: Option<&HyperliquidSigner>,
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
    ) -> Result<Self> {
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
        let public = state::build(|| hl::HyperliquidClient::with_config(exchange.clone(), config))?;
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
    pub(crate) fn signer_address(&self) -> Result<Option<String>> {
        Ok(self
            .state
            .client()?
            .private
            .map(|p| p.auth().signer_address().to_string()))
    }
    pub(crate) fn account_address(&self) -> Result<Option<String>> {
        Ok(self
            .state
            .client()?
            .private
            .map(|p| p.auth().account_address().to_string()))
    }
    pub(crate) async fn meta(&self, dex: Option<String>) -> Result<Output> {
        let p = self.state.client()?.public;
        self.state
            .run(async move {
                p.meta_with(hl::MetaRequest { dex })
                    .await
                    .map_err(errors::hl)
            })
            .await
    }
    pub(crate) async fn l2_book(
        &self,
        coin: String,
        n_sig_figs: Option<u8>,
        mantissa: Option<u8>,
    ) -> Result<Output> {
        let request = hl::L2BookRequest {
            coin,
            n_sig_figs,
            mantissa,
        };
        request.validate().map_err(errors::hl_native)?;
        let p = self.state.client()?.public;
        self.state
            .run(async move { p.l2_book(request).await.map_err(errors::hl) })
            .await
    }
    pub(crate) async fn clearinghouse_state(
        &self,
        user: &str,
        dex: Option<String>,
    ) -> Result<Output> {
        let request = hl::UserStateRequest {
            user: signers::address(user)?,
            dex,
        };
        let p = self.state.client()?.public;
        self.state
            .run(async move { p.clearinghouse_state(request).await.map_err(errors::hl) })
            .await
    }
    pub(crate) async fn open_orders(&self, user: &str, dex: Option<String>) -> Result<Output> {
        let request = hl::OpenOrdersRequest {
            user: signers::address(user)?,
            dex,
        };
        let p = self.state.client()?.public;
        self.state
            .run(async move { p.open_orders(request).await.map_err(errors::hl) })
            .await
    }
    pub(crate) async fn order(
        &self,
        request: &Input,
        nonce: Option<u64>,
        expires_after: Option<u64>,
    ) -> Result<Output> {
        let request: hl::OrderRequest = convert::parse(request)?;
        hl::Action::Order(request.clone())
            .validate()
            .map_err(errors::hl_native)?;
        let p = self.state.client()?.authenticated()?;
        self.state
            .run(async move {
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
            .await
    }
    pub(crate) async fn cancel(
        &self,
        request: &Input,
        nonce: Option<u64>,
        expires_after: Option<u64>,
    ) -> Result<Output> {
        let request: hl::CancelRequest = convert::parse(request)?;
        hl::Action::Cancel(request.clone())
            .validate()
            .map_err(errors::hl_native)?;
        let p = self.state.client()?.authenticated()?;
        self.state
            .run(async move {
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
            .await
    }
    pub(crate) async fn cancel_by_cloid(
        &self,
        request: &Input,
        nonce: Option<u64>,
        expires_after: Option<u64>,
    ) -> Result<Output> {
        let request: hl::CancelByCloidRequest = convert::parse(request)?;
        hl::Action::CancelByCloid(request.clone())
            .validate()
            .map_err(errors::hl_native)?;
        let p = self.state.client()?.authenticated()?;
        self.state
            .run(async move {
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
            .await
    }
    pub(crate) async fn subscribe(&self, spec: &Input) -> Result<Subscription> {
        let spec: hl::Subscription = convert::parse(spec)?;
        spec.validate().map_err(errors::hl_native)?;
        let p = self.state.client()?.public;
        self.state
            .stream(async move {
                Ok(NativeHandle::Hyperliquid {
                    handle: p.subscribe(spec).await.map_err(errors::hl)?,
                    book_only: false,
                })
            })
            .await
    }
    pub(crate) async fn subscribe_l2_book(&self, coin: String) -> Result<Subscription> {
        let spec = hl::Subscription::L2Book { coin };
        spec.validate().map_err(errors::hl_native)?;
        let p = self.state.client()?.public;
        self.state
            .stream(async move {
                Ok(NativeHandle::Hyperliquid {
                    handle: p.subscribe(spec).await.map_err(errors::hl)?,
                    book_only: true,
                })
            })
            .await
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
    }
}
impl Drop for HyperliquidClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
