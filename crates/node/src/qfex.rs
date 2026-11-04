use crate::prelude::*;
use crate::{
    convert, errors,
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::StandardTransport;
use aux_rs::qfex as q;
use std::sync::Arc;
use zeroize::Zeroizing;
#[derive(Clone)]
struct Clients {
    public: q::QfexClient<StandardTransport>,
    private: Option<q::QfexClient<StandardTransport, q::QfexAuth>>,
}
impl Clients {
    pub(crate) fn authenticated(&self) -> Result<q::QfexClient<StandardTransport, q::QfexAuth>> {
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
pub struct QfexClient {
    state: Arc<ClientState<Clients>>,
}
impl QfexClient {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
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
    ) -> Result<Self> {
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
        let public = state::build(|| q::QfexClient::with_config(exchange, config))?;
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
    pub(crate) async fn refdata(&self, ticker: Option<String>) -> Result<Output> {
        let c = self.state.client()?.public;
        self.state
            .run(async move {
                c.refdata(q::RefdataRequest { ticker })
                    .await
                    .map_err(errors::qfex)
            })
            .await
    }
    pub(crate) async fn contracts(&self, symbol: Option<String>) -> Result<Output> {
        let c = self.state.client()?.public;
        self.state
            .run(async move {
                c.contracts(q::ContractsRequest { symbol })
                    .await
                    .map_err(errors::qfex)
            })
            .await
    }
    pub(crate) async fn order_book(&self, symbol: String) -> Result<Output> {
        let c = self.state.client()?.public;
        self.state
            .run(async move {
                c.order_book(q::OrderBookRequest::new(symbol))
                    .await
                    .map_err(errors::qfex)
            })
            .await
    }
    pub(crate) async fn positions(&self) -> Result<Output> {
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.positions().await.map_err(errors::qfex) })
            .await
    }
    pub(crate) async fn account_balances(&self) -> Result<Output> {
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.account_balances().await.map_err(errors::qfex) })
            .await
    }
    pub(crate) async fn user_orders(
        &self,
        symbol: Option<String>,
        limit: u32,
        offset: u32,
    ) -> Result<Output> {
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move {
                c.user_orders(q::UserOrdersRequest {
                    symbol,
                    limit,
                    offset,
                })
                .await
                .map_err(errors::qfex)
            })
            .await
    }
    pub(crate) async fn add_order(&self, request: &Input) -> Result<Output> {
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
        self.state
            .run(async move { c.add_order(r).await.map_err(errors::qfex) })
            .await
    }
    pub(crate) async fn cancel_order(&self, request: &Input) -> Result<Output> {
        let r: q::CancelOrder = convert::parse(request)?;
        let c = self.state.client()?.authenticated()?;
        self.state
            .run(async move { c.cancel_order(r).await.map_err(errors::qfex) })
            .await
    }
    pub(crate) async fn subscribe(&self, channel: &str, symbol: String) -> Result<Subscription> {
        let channel: q::Channel = serde_json::from_value(serde_json::json!(channel))
            .map_err(|_| errors::ValidationError::new_err("unknown QFEX channel"))?;
        let c = self.state.client()?.public;
        self.state
            .stream(async move {
                c.subscribe(q::Subscription::new(channel, symbol))
                    .await
                    .map(NativeHandle::Qfex)
                    .map_err(errors::qfex)
            })
            .await
    }
    pub(crate) async fn subscribe_account(&self) -> Result<Subscription> {
        let c = self.state.client()?.authenticated()?;
        self.state
            .stream(async move {
                let mut s = c.trading_session().await.map_err(errors::qfex)?;
                s.subscribe_account().await.map_err(errors::qfex)?;
                Ok(NativeHandle::QfexTrade(s))
            })
            .await
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
    }
    pub(crate) fn __repr__(&self) -> &'static str {
        "QfexClient()"
    }
}
impl Drop for QfexClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
