use crate::HyperliquidClient;

use aux_rs_core::{
    Error as ClientError, Exchange, WebSocketStream, WebSocketSubscription, WebSocketTransport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    AccountStateEvent, Address, Error, L2Book, OpenOrdersEvent, OrderUpdate, Trade, UserFills,
    endpoints::validate_coin,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Subscription {
    #[serde(rename = "l2Book")]
    L2Book { coin: String },
    #[serde(rename = "trades")]
    Trades { coin: String },
    #[serde(rename = "orderUpdates")]
    OrderUpdates { user: Address },
    #[serde(rename = "userFills")]
    UserFills {
        user: Address,
        #[serde(rename = "aggregateByTime", skip_serializing_if = "Option::is_none")]
        aggregate_by_time: Option<bool>,
    },
    #[serde(rename = "clearinghouseState")]
    ClearinghouseState {
        user: Address,
        #[serde(skip_serializing_if = "Option::is_none")]
        dex: Option<String>,
    },
    #[serde(rename = "openOrders")]
    OpenOrders {
        user: Address,
        #[serde(skip_serializing_if = "Option::is_none")]
        dex: Option<String>,
    },
}

impl Subscription {
    pub fn validate(&self) -> Result<(), Error> {
        match self {
            Self::L2Book { coin } | Self::Trades { coin } => validate_coin(coin),
            _ => Ok(()),
        }
    }

    fn wire(&self, method: &str) -> String {
        serde_json::json!({ "method": method, "subscription": self }).to_string()
    }
}

impl WebSocketSubscription for Subscription {
    fn is_data_message(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).ok().is_some_and(|v| {
            matches!(
                v["channel"].as_str(),
                Some(
                    "l2Book"
                        | "trades"
                        | "clearinghouseState"
                        | "openOrders"
                        | "orderUpdates"
                        | "userFills"
                )
            )
        })
    }
    fn connection_scope(&self) -> String {
        let (user, aggregate) = match self {
            Self::OrderUpdates { user }
            | Self::ClearinghouseState { user, .. }
            | Self::OpenOrders { user, .. } => (user, false),
            Self::UserFills {
                user,
                aggregate_by_time,
            } => (user, *aggregate_by_time == Some(true)),
            _ => return String::new(),
        };
        // orderUpdates omits user; fills also omit the aggregation parameter.
        // These contexts must not share a socket when they cannot be demuxed.
        format!("hyperliquid-account:{user}:aggregate:{aggregate}")
    }
    fn key(&self) -> String {
        let mut normalized = self.clone();
        match &mut normalized {
            Self::UserFills {
                aggregate_by_time, ..
            } if *aggregate_by_time == Some(false) => *aggregate_by_time = None,
            Self::ClearinghouseState { dex, .. } | Self::OpenOrders { dex, .. }
                if dex.as_deref() == Some("") =>
            {
                *dex = None
            }
            _ => {}
        }
        serde_json::to_string(&normalized).expect("subscription is JSON serializable")
    }
    fn subscribe_message(&self) -> String {
        self.wire("subscribe")
    }
    fn unsubscribe_message(&self) -> String {
        self.wire("unsubscribe")
    }
    fn heartbeat_message(&self) -> Option<String> {
        Some(r#"{"method":"ping"}"#.into())
    }

    fn matches(&self, text: &str) -> bool {
        let Ok(value) = serde_json::from_str::<Value>(text) else {
            return true;
        };
        match value["channel"].as_str() {
            Some("subscriptionResponse") => {
                match serde_json::from_value::<SubscriptionResponse>(value["data"].clone()) {
                    Ok(response) => response.subscription.key() == self.key(),
                    Err(_) => true,
                }
            }
            Some("l2Book") => match self {
                Self::L2Book { coin } => value["data"]["coin"]
                    .as_str()
                    .is_none_or(|actual| actual == coin),
                _ => false,
            },
            Some("trades") => match self {
                Self::Trades { coin } => value["data"].as_array().is_none_or(|trades| {
                    trades
                        .iter()
                        .all(|trade| trade["coin"].as_str().is_none_or(|actual| actual == coin))
                }),
                _ => false,
            },
            Some("orderUpdates") => matches!(self, Self::OrderUpdates { .. }),
            Some("userFills") => match self {
                Self::UserFills { user, .. } => matches_user(&value["data"], *user),
                _ => false,
            },
            Some("clearinghouseState") | Some("openOrders") => {
                let expected_channel = value["channel"].as_str().unwrap_or_default();
                match self {
                    Self::ClearinghouseState { user, dex }
                        if expected_channel == "clearinghouseState" =>
                    {
                        matches_user(&value["data"], *user) && matches_dex(&value["data"], dex)
                    }
                    Self::OpenOrders { user, dex } if expected_channel == "openOrders" => {
                        matches_user(&value["data"], *user) && matches_dex(&value["data"], dex)
                    }
                    _ => false,
                }
            }
            Some("pong") => false,
            // Route native errors, unknown channels, and malformed envelopes so
            // the caller sees them explicitly instead of silently losing data.
            _ => true,
        }
    }
}

fn matches_user(data: &Value, user: Address) -> bool {
    match data["user"]
        .as_str()
        .and_then(|value| value.parse::<Address>().ok())
    {
        Some(actual) => actual == user,
        None => true, // malformed payloads must reach the decoder
    }
}
fn matches_dex(data: &Value, dex: &Option<String>) -> bool {
    data["dex"]
        .as_str()
        .is_none_or(|actual| actual == dex.as_deref().unwrap_or_default())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriptionResponse {
    pub method: String,
    pub subscription: Subscription,
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Event {
    SubscriptionResponse(SubscriptionResponse),
    L2Book(L2Book),
    Trades(Vec<Trade>),
    OrderUpdates(Vec<OrderUpdate>),
    UserFills(UserFills),
    ClearinghouseState(Box<AccountStateEvent>),
    OpenOrders(OpenOrdersEvent),
    Pong,
    Unknown { channel: String, data: Value },
}

impl Event {
    pub fn decode(text: &str) -> Result<Self, Error> {
        #[derive(Deserialize)]
        struct Envelope {
            channel: String,
            #[serde(default)]
            data: Value,
        }
        let envelope: Envelope = serde_json::from_str(text).map_err(Error::Decode)?;
        match envelope.channel.as_str() {
            "subscriptionResponse" => serde_json::from_value(envelope.data)
                .map(Self::SubscriptionResponse)
                .map_err(Error::Decode),
            "l2Book" => serde_json::from_value(envelope.data)
                .map(Self::L2Book)
                .map_err(Error::Decode),
            "trades" => serde_json::from_value(envelope.data)
                .map(Self::Trades)
                .map_err(Error::Decode),
            "orderUpdates" => serde_json::from_value(envelope.data)
                .map(Self::OrderUpdates)
                .map_err(Error::Decode),
            "userFills" => serde_json::from_value(envelope.data)
                .map(Self::UserFills)
                .map_err(Error::Decode),
            "clearinghouseState" => serde_json::from_value(envelope.data)
                .map(Self::ClearinghouseState)
                .map_err(Error::Decode),
            "openOrders" => serde_json::from_value(envelope.data)
                .map(Self::OpenOrders)
                .map_err(Error::Decode),
            "pong" => Ok(Self::Pong),
            "error" => Err(Error::Api {
                message: envelope
                    .data
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| envelope.data.to_string()),
            }),
            _ => Ok(Self::Unknown {
                channel: envelope.channel,
                data: envelope.data,
            }),
        }
    }
}

pub struct SubscriptionHandle<S> {
    stream: S,
}

impl<S: WebSocketStream> SubscriptionHandle<S> {
    pub async fn next(&mut self) -> Option<Result<Event, ClientError<Error>>> {
        self.stream.next().await.map(|result| {
            result
                .map_err(ClientError::Transport)
                .and_then(|text| Event::decode(&text).map_err(ClientError::Exchange))
        })
    }

    /// Detach this consumer; the remote subscription remains while other
    /// handles use it. The last handle sends unsubscribe and closes the socket.
    pub async fn unsubscribe(&mut self) -> Result<(), ClientError<Error>> {
        self.stream.close().await.map_err(ClientError::Transport)
    }

    pub async fn close(&mut self) -> Result<(), ClientError<Error>> {
        self.unsubscribe().await
    }
}

impl<T: WebSocketTransport, A: Sync> HyperliquidClient<T, A> {
    pub async fn subscribe(
        &self,
        subscription: Subscription,
    ) -> Result<SubscriptionHandle<T::Stream>, ClientError<Error>> {
        subscription.validate().map_err(ClientError::Exchange)?;
        let stream = self
            .transport()
            .subscribe(self.exchange().ws_url(), subscription)
            .await?;
        Ok(SubscriptionHandle { stream })
    }
}
