use crate::*;
use aux_rs_core::{
    AuthenticationMessage, Error as ClientError, Exchange, RequestOutcome, TransportError,
    TransportErrorKind, WebSocketStream, WebSocketSubscription, WebSocketTransport,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};
static IDS: AtomicU64 = AtomicU64::new(1);
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Subscription {
    AuctionsWatch,
    MarginWatch,
    Orderbook {
        instrument_name: String,
        group: u16,
        depth: u16,
    },
    SpotFeed {
        currency: String,
    },
    Balances {
        subaccount_id: u64,
    },
    BestQuotes {
        subaccount_id: u64,
    },
    Orders {
        subaccount_id: u64,
    },
    Quotes {
        subaccount_id: u64,
    },
    AccountTrades {
        subaccount_id: u64,
    },
    TickerSlim {
        instrument_name: String,
        interval: u16,
    },
    Trades {
        instrument_name: String,
    },
    TradesByType {
        instrument_type: String,
        currency: String,
    },
    Rfqs {
        wallet: String,
    },
}
fn component(s: &str) -> Result<&str, Error> {
    if s.is_empty()
        || !s
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_".contains(&b))
    {
        return Err(Error::InvalidRequest("invalid channel component"));
    }
    Ok(s)
}
impl Subscription {
    pub fn is_private(&self) -> bool {
        matches!(
            self,
            Self::Balances { .. }
                | Self::BestQuotes { .. }
                | Self::Orders { .. }
                | Self::Quotes { .. }
                | Self::AccountTrades { .. }
                | Self::Rfqs { .. }
        )
    }
    pub fn channel(&self) -> Result<String, Error> {
        Ok(match self {
            Self::AuctionsWatch => "auctions.watch".into(),
            Self::MarginWatch => "margin.watch".into(),
            Self::Orderbook {
                instrument_name,
                group,
                depth,
            } => {
                if ![1, 10, 100].contains(group) || ![1, 10, 20, 100].contains(depth) {
                    return Err(Error::InvalidRequest("invalid Derive book group or depth"));
                }
                format!("orderbook.{}.{group}.{depth}", component(instrument_name)?)
            }
            Self::SpotFeed { currency } => format!("spot_feed.{}", component(currency)?),
            Self::Balances { subaccount_id } => format!("{subaccount_id}.balances"),
            Self::BestQuotes { subaccount_id } => format!("{subaccount_id}.best.quotes"),
            Self::Orders { subaccount_id } => format!("{subaccount_id}.orders"),
            Self::Quotes { subaccount_id } => format!("{subaccount_id}.quotes"),
            Self::AccountTrades { subaccount_id } => format!("{subaccount_id}.trades"),
            Self::TickerSlim {
                instrument_name,
                interval,
            } => {
                if ![100, 1000].contains(interval) {
                    return Err(Error::InvalidRequest("invalid ticker interval"));
                }
                format!("ticker_slim.{}.{interval}", component(instrument_name)?)
            }
            Self::Trades { instrument_name } => format!("trades.{}", component(instrument_name)?),
            Self::TradesByType {
                instrument_type,
                currency,
            } => {
                if !["erc20", "option", "perp"].contains(&instrument_type.as_str()) {
                    return Err(Error::InvalidRequest("invalid instrument type"));
                }
                format!("trades.{instrument_type}.{}", component(currency)?)
            }
            Self::Rfqs { wallet } => {
                format!("0x{}.rfqs", aux_rs_core::hex::encode(address(wallet)?))
            }
        })
    }
}
/// Native envelope, including acknowledgements, publication metadata and unknown fields.
#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Event {
    pub fields: serde_json::Map<String, Value>,
}
impl Event {
    pub fn decode(text: &str) -> Result<Self, Error> {
        if text == "pong" {
            return Ok(Self {
                fields: serde_json::from_value(json!({"method":"pong"}))?,
            });
        }
        let value: Value = serde_json::from_str(text)?;
        if value.get("error").is_some_and(|v| !v.is_null()) {
            return Err(Error::Api {
                status: None,
                body: value,
            });
        }
        if value["result"]["status"]
            .as_object()
            .is_some_and(|statuses| statuses.values().any(|v| v != "ok"))
        {
            return Err(Error::Api {
                status: None,
                body: value,
            });
        }
        if value.get("result").is_none()
            && !(value["params"]["channel"].is_string() && value["params"].get("data").is_some())
        {
            return Err(Error::InvalidRequest("malformed Derive WebSocket envelope"));
        }
        serde_json::from_value(value).map_err(Error::Decode)
    }
    pub fn channel(&self) -> Option<&str> {
        self.fields.get("params")?["channel"].as_str()
    }
    pub fn payload<T: serde::de::DeserializeOwned>(&self) -> Result<T, Error> {
        serde_json::from_value(
            self.fields
                .get("params")
                .and_then(|p| p.get("data"))
                .cloned()
                .ok_or(Error::InvalidRequest("event has no channel data"))?,
        )
        .map_err(Error::Decode)
    }
}
trait Login: Send + Sync {
    fn message(&self) -> Pin<Box<dyn Future<Output = Result<Value, Error>> + Send + '_>>;
    fn scope(&self) -> String;
}
struct AuthLogin<A> {
    auth: A,
    exchange: Derive,
}
impl<A: Authentication> Login for AuthLogin<A> {
    fn message(&self) -> Pin<Box<dyn Future<Output = Result<Value, Error>> + Send + '_>> {
        Box::pin(self.auth.login(&self.exchange))
    }
    fn scope(&self) -> String {
        format!(
            "{}:{}:{}",
            self.exchange.chain_id,
            self.auth.wallet(),
            aux_rs_core::hex::encode(self.auth.signer_address())
        )
    }
}
struct Wire {
    channel: String,
    id: u64,
    auth: Option<Arc<dyn Login>>,
}
fn transport(e: Error) -> TransportError {
    let kind = if matches!(e, Error::Authentication(_) | Error::Signing(_)) {
        TransportErrorKind::Configuration
    } else {
        TransportErrorKind::Protocol
    };
    TransportError::new(
        kind,
        RequestOutcome::NotSent,
        "Derive WebSocket authentication or protocol failed",
    )
}
impl WebSocketSubscription for Wire {
    fn authentication_message(&self) -> AuthenticationMessage<'_> {
        Box::pin(async move {
            match &self.auth{Some(auth)=>Ok(Some(json!({"id":"aux-login","method":"public/login","params":auth.message().await.map_err(transport)?}).to_string())),None=>Ok(None)}
        })
    }
    fn authentication_result(&self, text: &str) -> Option<Result<(), TransportError>> {
        let value: Value = serde_json::from_str(text).ok()?;
        if value["id"] != "aux-login" {
            return None;
        }
        Some(
            if value.get("error").is_some_and(|e| !e.is_null()) || !value["result"].is_array() {
                Err(transport(Error::Authentication("login rejected")))
            } else {
                Ok(())
            },
        )
    }
    fn connection_scope(&self) -> String {
        self.auth
            .as_ref()
            .map(|a| a.scope())
            .unwrap_or("derive-public".into())
    }
    fn key(&self) -> String {
        self.channel.clone()
    }
    fn requires_fresh_snapshot(&self) -> bool {
        self.channel.starts_with("orderbook.")
    }
    fn subscribe_message(&self) -> String {
        json!({"id":self.id,"method":"subscribe","params":{"channels":[self.channel]}}).to_string()
    }
    fn unsubscribe_message(&self) -> String {
        json!({"id":self.id,"method":"unsubscribe","params":{"channels":[self.channel]}})
            .to_string()
    }
    fn matches(&self, text: &str) -> bool {
        if text == "pong" {
            return true;
        }
        let Ok(value) = serde_json::from_str::<Value>(text) else {
            return true;
        };
        value["params"]["channel"] == self.channel
            || value["id"] == self.id
            || value.get("id").is_none()
                && (value.get("error").is_some() || value["params"].get("channel").is_none())
    }
    fn is_snapshot_control(&self, text: &str) -> bool {
        !self.is_data_message(text)
    }
    fn is_data_message(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            v["params"]["channel"] == self.channel && v["params"].get("data").is_some()
        })
    }
    fn heartbeat_message(&self) -> Option<String> {
        Some("ping".into())
    }
}
pub struct SubscriptionHandle<S> {
    stream: S,
    ended: bool,
}
impl<S: WebSocketStream> SubscriptionHandle<S> {
    pub async fn next(&mut self) -> Option<Result<Event, ClientError<Error>>> {
        if self.ended {
            return None;
        }
        match self.stream.next().await {
            None => {
                self.ended = true;
                None
            }
            Some(Err(e)) => {
                if e.kind != TransportErrorKind::Gap {
                    self.ended = true;
                }
                Some(Err(ClientError::Transport(e)))
            }
            Some(Ok(text)) => {
                let value = Event::decode(&text).map_err(ClientError::Exchange);
                if value.is_err() {
                    self.ended = true;
                    let _ = self.stream.close().await;
                }
                Some(value)
            }
        }
    }
    pub async fn close(&mut self) -> Result<(), ClientError<Error>> {
        self.ended = true;
        self.stream.close().await.map_err(ClientError::Transport)
    }
}
impl<T: WebSocketTransport, A: Sync> DeriveClient<T, A> {
    pub async fn subscribe(
        &self,
        subscription: Subscription,
    ) -> Result<SubscriptionHandle<T::Stream>, ClientError<Error>> {
        if subscription.is_private() {
            return Err(ClientError::Exchange(Error::Authentication(
                "private channel requires subscribe_private",
            )));
        }
        let channel = subscription.channel().map_err(ClientError::Exchange)?;
        let stream = self
            .transport
            .subscribe(
                self.exchange.ws_url(),
                Wire {
                    channel,
                    id: IDS.fetch_add(1, Ordering::Relaxed),
                    auth: None,
                },
            )
            .await?;
        Ok(SubscriptionHandle {
            stream,
            ended: false,
        })
    }
}
impl<T: WebSocketTransport, A: Authentication + Clone + 'static> DeriveClient<T, A> {
    pub async fn subscribe_private(
        &self,
        subscription: Subscription,
    ) -> Result<SubscriptionHandle<T::Stream>, ClientError<Error>> {
        let channel = subscription.channel().map_err(ClientError::Exchange)?;
        let stream = self
            .transport
            .subscribe(
                self.exchange.ws_url(),
                Wire {
                    channel,
                    id: IDS.fetch_add(1, Ordering::Relaxed),
                    auth: Some(Arc::new(AuthLogin {
                        auth: self.auth.clone(),
                        exchange: self.exchange.clone(),
                    })),
                },
            )
            .await?;
        Ok(SubscriptionHandle {
            stream,
            ended: false,
        })
    }
}
