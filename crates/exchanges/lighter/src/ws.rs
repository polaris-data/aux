use crate::LighterClient;
use crate::*;
use aux_rs_core::{
    Error as ClientError, Exchange, WebSocketStream, WebSocketSubscription, WebSocketTransport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Subscription {
    OrderBook { market_id: i64 },
    Trades { market_id: i64 },
    Account { account_index: i64 },
}
impl Subscription {
    fn spec(&self) -> Result<WireSubscription, Error> {
        let channel = match self {
            Self::OrderBook { market_id } => {
                trading::validate_market(*market_id)?;
                format!("order_book/{market_id}")
            }
            Self::Trades { market_id } => {
                trading::validate_market(*market_id)?;
                format!("trade/{market_id}")
            }
            Self::Account { account_index } => {
                trading::validate_identity(*account_index, 0)?;
                format!("account_all/{account_index}")
            }
        };
        Ok(WireSubscription {
            channel,
            token: None,
        })
    }
}
#[derive(Clone)]
struct WireSubscription {
    channel: String,
    token: Option<AuthToken>,
}
impl WireSubscription {
    fn message(&self, kind: &str) -> String {
        let mut value = serde_json::json!({"type":kind,"channel":self.channel});
        if kind == "subscribe"
            && let Some(token) = &self.token
        {
            value["auth"] = token.as_str().into();
        }
        value.to_string()
    }
}
impl WebSocketSubscription for WireSubscription {
    fn resubscribe_allowed(&self) -> bool {
        self.token
            .as_ref()
            .is_none_or(|token| crate::auth::unix_seconds().is_ok_and(|now| now < token.deadline()))
    }
    fn is_initial_snapshot(&self, text: &str) -> bool {
        let Ok(value) = serde_json::from_str::<Value>(text) else {
            return true;
        };
        value.get("error").is_some()
            || value["type"]
                .as_str()
                .is_none_or(|kind| kind.starts_with("subscribed/"))
    }
    fn is_data_message(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).ok().is_some_and(|v| {
            v["type"]
                .as_str()
                .is_some_and(|k| k.starts_with("subscribed/") || k.starts_with("update/"))
        })
    }
    fn requires_fresh_snapshot(&self) -> bool {
        true
    }
    fn key(&self) -> String {
        self.channel.clone()
    }
    fn connection_scope(&self) -> String {
        self.token
            .as_ref()
            .map(AuthToken::scope)
            .unwrap_or_default()
    }
    fn subscribe_message(&self) -> String {
        self.message("subscribe")
    }
    fn unsubscribe_message(&self) -> String {
        self.message("unsubscribe")
    }
    fn response_message(&self, text: &str) -> Option<String> {
        let value: Value = serde_json::from_str(text).ok()?;
        (value["type"] == "ping").then(|| r#"{"type":"pong"}"#.into())
    }
    fn matches(&self, text: &str) -> bool {
        let Ok(value) = serde_json::from_str::<Value>(text) else {
            return true;
        };
        if value.get("error").is_some() {
            return true;
        }
        if matches!(value["type"].as_str(), Some("ping" | "pong" | "connected")) {
            return false;
        }
        value["channel"]
            .as_str()
            .is_none_or(|c| c == self.channel.replace('/', ":"))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BookLevel {
    pub price: String,
    pub size: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamBook {
    pub asks: Vec<BookLevel>,
    pub bids: Vec<BookLevel>,
    pub offset: i64,
    pub nonce: i64,
    /// Native begin_nonce permits callers to check continuity; no reconstruction.
    #[serde(default)]
    pub begin_nonce: Option<i64>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BookEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub channel: String,
    pub order_book: StreamBook,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradesEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub channel: String,
    pub trades: Vec<Trade>,
    #[serde(default)]
    pub liquidation_trades: Vec<Trade>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountAsset {
    pub symbol: String,
    pub asset_id: i64,
    pub balance: String,
    pub locked_balance: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub channel: String,
    pub positions: BTreeMap<String, Position>,
    pub assets: BTreeMap<String, AccountAsset>,
    pub trades: BTreeMap<String, Vec<Trade>>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountOrdersEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub channel: String,
    pub orders: BTreeMap<String, Vec<Order>>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Event {
    OrderBook(BookEvent),
    Trades(TradesEvent),
    Account(Box<AccountEvent>),
    AccountOrders(AccountOrdersEvent),
    Shutdown(Value),
    Unknown(Value),
}
impl Event {
    pub fn decode(text: &str) -> Result<Self, Error> {
        let value: Value = serde_json::from_str(text).map_err(Error::Decode)?;
        if let Some(error) = value.get("error") {
            let code = error["code"]
                .as_i64()
                .ok_or(Error::Subscription("malformed native error"))?;
            let message = error["message"].as_str().map(str::to_owned);
            let extra = error
                .as_object()
                .map(|o| {
                    o.iter()
                        .filter(|(k, _)| *k != "code" && *k != "message")
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect()
                })
                .unwrap_or_default();
            return Err(Error::Api {
                code,
                message,
                extra,
                http_status: None,
            });
        }
        let kind = value["type"]
            .as_str()
            .ok_or(Error::Subscription("missing native event type"))?;
        match kind {
            "subscribed/order_book" | "update/order_book" => serde_json::from_value(value)
                .map(Self::OrderBook)
                .map_err(Error::Decode),
            "subscribed/trade" | "update/trade" => serde_json::from_value(value)
                .map(Self::Trades)
                .map_err(Error::Decode),
            "subscribed/account_all" | "update/account_all" => serde_json::from_value(value)
                .map(Self::Account)
                .map_err(Error::Decode),
            "subscribed/account_all_orders" | "update/account_all_orders" => {
                serde_json::from_value(value)
                    .map(Self::AccountOrders)
                    .map_err(Error::Decode)
            }
            "shutdown" => Ok(Self::Shutdown(value)),
            _ => Ok(Self::Unknown(value)),
        }
    }
    pub fn is_snapshot(&self) -> bool {
        match self {
            Self::OrderBook(v) => v.event_type.starts_with("subscribed/"),
            Self::Trades(v) => v.event_type.starts_with("subscribed/"),
            Self::Account(v) => v.event_type.starts_with("subscribed/"),
            Self::AccountOrders(v) => v.event_type.starts_with("subscribed/"),
            _ => false,
        }
    }
}
pub struct SubscriptionHandle<S> {
    stream: S,
}
impl<S: WebSocketStream> SubscriptionHandle<S> {
    pub async fn next(&mut self) -> Option<Result<Event, ClientError<Error>>> {
        self.stream.next().await.map(|r| {
            r.map_err(ClientError::Transport)
                .and_then(|text| Event::decode(&text).map_err(ClientError::Exchange))
        })
    }
    pub async fn unsubscribe(&mut self) -> Result<(), ClientError<Error>> {
        self.stream.close().await.map_err(ClientError::Transport)
    }
}

impl<T: WebSocketTransport, A: Sync> LighterClient<T, A> {
    pub async fn subscribe(
        &self,
        subscription: Subscription,
    ) -> Result<SubscriptionHandle<T::Stream>, ClientError<Error>> {
        let spec = subscription.spec().map_err(ClientError::Exchange)?;
        let stream = self
            .transport()
            .subscribe(self.exchange().ws_url(), spec)
            .await?;
        Ok(SubscriptionHandle { stream })
    }
}

impl<T: WebSocketTransport, A: AuthHandler> LighterClient<T, A> {
    pub async fn subscribe_account_orders(
        &self,
    ) -> Result<SubscriptionHandle<T::Stream>, ClientError<Error>> {
        self.auth()
            .check_exchange(self.exchange())
            .map_err(ClientError::Exchange)?;
        let token = sign_auth_token(
            self.auth(),
            auth::unix_seconds().map_err(ClientError::Exchange)? + 600,
        )
        .await
        .map_err(ClientError::Exchange)?;
        token
            .check(self.exchange(), self.auth().account_index())
            .map_err(ClientError::Exchange)?;
        let spec = WireSubscription {
            channel: format!("account_all_orders/{}", self.auth().account_index()),
            token: Some(token),
        };
        let stream = self
            .transport()
            .subscribe(self.exchange().ws_url(), spec)
            .await?;
        Ok(SubscriptionHandle { stream })
    }
}
