use crate::*;
use aux_rs_core::{
    Error as ClientError, Exchange, WebSocketStream, WebSocketSubscription, WebSocketTransport,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Channel {
    #[serde(rename = "markets")]
    Markets,
    #[serde(rename = "l2Orderbook")]
    OrderBook,
    #[serde(rename = "l2OrderbookUpdates")]
    OrderBookUpdates,
    #[serde(rename = "trades")]
    Trades,
    #[serde(rename = "bbo")]
    Bbo,
    #[serde(rename = "account")]
    Account,
    #[serde(rename = "positions")]
    Positions,
    #[serde(rename = "orders")]
    Orders,
    #[serde(rename = "userFills")]
    UserFills,
}
impl Channel {
    fn account(self) -> bool {
        matches!(
            self,
            Self::Account | Self::Positions | Self::Orders | Self::UserFills
        )
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Subscription {
    pub channel: Channel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_index: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_levels: Option<u8>,
}
impl Subscription {
    pub fn market(channel: Channel, id: impl Into<String>) -> Self {
        Self {
            channel,
            id: Some(id.into()),
            account_index: None,
            market: None,
            n_levels: None,
        }
    }
    pub fn account(channel: Channel, value: impl AsRef<str>, index: u8) -> Result<Self, Error> {
        let r = AccountRequest::new(value, index)?;
        Ok(Self {
            channel,
            id: Some(r.address),
            account_index: Some(index),
            market: None,
            n_levels: None,
        })
    }
    pub fn markets() -> Self {
        Self {
            channel: Channel::Markets,
            id: None,
            account_index: None,
            market: None,
            n_levels: None,
        }
    }
    pub fn validate(&self) -> Result<(), Error> {
        if self.channel.account() {
            let a = self
                .id
                .as_ref()
                .ok_or(Error::InvalidRequest("account channel requires an address"))?;
            let normalized = address(a)?;
            if &normalized != a {
                return Err(Error::InvalidRequest(
                    "subscription address must be normalized lowercase",
                ));
            }
            if self.account_index.is_none_or(|i| i > 9) {
                return Err(Error::InvalidRequest(
                    "account channel requires accountIndex from 0 to 9",
                ));
            }
        } else if self.account_index.is_some() {
            return Err(Error::InvalidRequest(
                "accountIndex is only valid for account channels",
            ));
        } else if !matches!(self.channel, Channel::Markets) {
            nonempty(self.id.as_deref().unwrap_or(""))?;
        }
        if self.market.is_some()
            && !matches!(
                self.channel,
                Channel::Positions | Channel::Orders | Channel::UserFills
            )
        {
            return Err(Error::InvalidRequest(
                "market filter is invalid for this channel",
            ));
        }
        if self.n_levels.is_some_and(|n| n == 0 || n > 100)
            || self.n_levels.is_some()
                && !matches!(self.channel, Channel::OrderBook | Channel::OrderBookUpdates)
        {
            return Err(Error::InvalidRequest(
                "nLevels requires an orderbook channel and 1 to 100 levels",
            ));
        }
        Ok(())
    }
    fn wire(&self, kind: &str) -> String {
        let mut v = serde_json::to_value(self).expect("subscription fields serialize");
        v["type"] = Value::String(kind.into());
        v.to_string()
    }
}
impl WebSocketSubscription for Subscription {
    fn key(&self) -> String {
        self.wire("subscribe")
    }
    // Isolate each native view, including account index and market filters. Native
    // error frames may omit identity and must never leak into a different view.
    fn connection_scope(&self) -> String {
        self.key()
    }
    fn subscribe_message(&self) -> String {
        self.wire("subscribe")
    }
    fn unsubscribe_message(&self) -> String {
        self.wire("unsubscribe")
    }
    fn requires_fresh_snapshot(&self) -> bool {
        matches!(self.channel, Channel::OrderBookUpdates)
    }
    fn is_initial_snapshot(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            v["type"] == "subscribed"
                && v["contents"]["bids"].is_array()
                && v["contents"]["asks"].is_array()
        })
    }
    fn is_data_message(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            v["type"] == "channel_data"
                || (v["type"] == "subscribed" && v.get("contents").is_some())
        })
    }
    fn matches(&self, text: &str) -> bool {
        let Ok(v) = serde_json::from_str::<Value>(text) else {
            return true;
        };
        if v["type"] == "error" || v["type"] == "connected" {
            return true;
        }
        let expected = serde_json::to_value(self).expect("subscription serializes");
        if v["channel"] != expected["channel"] {
            return false;
        }
        if self.id.as_ref().is_some_and(|id| v["id"] != *id) {
            return false;
        }
        if self.account_index.is_some_and(|i| v["accountIndex"] != i) {
            return false;
        }
        // The server normalizes numeric market filters to display names in
        // envelopes. The dedicated connection scope already isolates that view.
        true
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_index: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<Value>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
impl Event {
    pub fn decode(text: &str) -> Result<Self, Error> {
        let v: Value = serde_json::from_str(text)?;
        if v["type"] == "error" {
            return Err(Error::Api {
                status: None,
                body: v,
            });
        }
        serde_json::from_value(v).map_err(Error::Decode)
    }
    pub fn decode_contents<T: DeserializeOwned>(&self) -> Result<T, Error> {
        serde_json::from_value(
            self.contents
                .clone()
                .ok_or(Error::InvalidRequest("event has no contents"))?,
        )
        .map_err(Error::Decode)
    }
}
pub struct SubscriptionHandle<S> {
    stream: S,
}
impl<S: WebSocketStream> SubscriptionHandle<S> {
    pub async fn next(&mut self) -> Option<Result<Event, ClientError<Error>>> {
        self.stream.next().await.map(|r| {
            r.map_err(ClientError::Transport)
                .and_then(|t| Event::decode(&t).map_err(ClientError::Exchange))
        })
    }
    pub async fn close(&mut self) -> Result<(), ClientError<Error>> {
        self.stream.close().await.map_err(ClientError::Transport)
    }
    pub async fn unsubscribe(&mut self) -> Result<(), ClientError<Error>> {
        self.close().await
    }
}
impl<T: WebSocketTransport, A: Sync> ArcusClient<T, A> {
    pub async fn subscribe(
        &self,
        s: Subscription,
    ) -> Result<SubscriptionHandle<T::Stream>, ClientError<Error>> {
        s.validate().map_err(ClientError::Exchange)?;
        Ok(SubscriptionHandle {
            stream: self.transport.subscribe(self.exchange.ws_url(), s).await?,
        })
    }
}
