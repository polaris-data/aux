use crate::*;
use aux_rs_core::{
    AuthenticationMessage, Error as ClientError, Exchange, RequestOutcome, TransportError,
    TransportErrorKind, WebSocketStream, WebSocketSubscription, WebSocketTransport,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::atomic::{AtomicU64, Ordering};
static IDS: AtomicU64 = AtomicU64::new(1);
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Subscription {
    Account,
    BalanceEvents,
    Bbo {
        market: String,
    },
    BlockTrades,
    Fills {
        market: String,
    },
    FundingData {
        market: String,
    },
    FundingPayments {
        market: String,
    },
    FundingRateComparison {
        refresh_rate: String,
    },
    Markets {
        market: Option<String>,
    },
    MarketsSummary {
        market: Option<String>,
    },
    Mmp,
    OrderBook {
        market: String,
        feed_type: String,
        refresh_rate: String,
        price_tick: Option<String>,
    },
    Orders {
        market: String,
    },
    Positions,
    Rfq {
        rfq_id: Option<String>,
    },
    Tradebusts,
    Trades {
        market: String,
    },
    Transaction,
    Transfers,
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
            Self::Account
                | Self::BalanceEvents
                | Self::BlockTrades
                | Self::Fills { .. }
                | Self::FundingPayments { .. }
                | Self::Mmp
                | Self::Orders { .. }
                | Self::Positions
                | Self::Rfq { .. }
                | Self::Tradebusts
                | Self::Transaction
                | Self::Transfers
        )
    }
    pub fn channel(&self) -> Result<String, Error> {
        Ok(match self {
            Self::Account => "account".into(),
            Self::BalanceEvents => "balance_events".into(),
            Self::BlockTrades => "block_trades".into(),
            Self::Bbo { market } => format!("bbo.{}", component(market)?),
            Self::Fills { market } => format!("fills.{}", component(market)?),
            Self::FundingData { market } => format!("funding_data.{}", component(market)?),
            Self::FundingPayments { market } => format!("funding_payments.{}", component(market)?),
            Self::FundingRateComparison { refresh_rate } => {
                if !["500ms", "1000ms"].contains(&refresh_rate.as_str()) {
                    return Err(Error::InvalidRequest("invalid funding comparison rate"));
                }
                format!("funding_rate_comparison.ALL@{refresh_rate}")
            }
            Self::Markets { market } => match market {
                Some(m) => format!("markets.{}", component(m)?),
                None => "markets".into(),
            },
            Self::MarketsSummary { market } => match market {
                Some(m) => format!("markets_summary.{}", component(m)?),
                None => "markets_summary".into(),
            },
            Self::Mmp => "mmp".into(),
            Self::OrderBook {
                market,
                feed_type,
                refresh_rate,
                price_tick,
            } => {
                if !["snapshot", "deltas", "interactive", "interactive_deltas"]
                    .contains(&feed_type.as_str())
                    || !["50ms", "100ms", "200ms", "500ms", "1000ms"]
                        .contains(&refresh_rate.as_str())
                {
                    return Err(Error::InvalidRequest("invalid book feed or refresh rate"));
                }
                let mut channel = format!(
                    "order_book.{}.{feed_type}@15@{refresh_rate}",
                    component(market)?
                );
                if let Some(tick) = price_tick {
                    if !tick.bytes().all(|b| b.is_ascii_digit() || b == b'.') || tick.is_empty() {
                        return Err(Error::InvalidRequest("invalid price tick"));
                    }
                    channel.push('@');
                    channel.push_str(tick);
                }
                channel
            }
            Self::Orders { market } => format!("orders.{}", component(market)?),
            Self::Positions => "positions".into(),
            Self::Rfq { rfq_id } => match rfq_id {
                Some(id) => format!("rfq.{}", component(id)?),
                None => "rfq".into(),
            },
            Self::Tradebusts => "tradebusts".into(),
            Self::Trades { market } => format!("trades.{}", component(market)?),
            Self::Transaction => "transaction".into(),
            Self::Transfers => "transfers".into(),
        })
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Event {
    #[cfg_attr(feature = "typescript", ts(type = "{ [key: string]: LosslessValue }"))]
    pub fields: serde_json::Map<String, Value>,
}
impl Event {
    pub fn decode(text: &str) -> Result<Self, Error> {
        let value: Value = serde_json::from_str(text)?;
        if value.get("error").is_some_and(|e| !e.is_null()) {
            return Err(Error::Api {
                status: None,
                body: value,
            });
        }
        if value.get("result").is_none()
            && !(value["params"]["channel"].is_string() && value["params"].get("data").is_some())
            && value["method"] != "heartbeat"
            && value["method"] != "subscribed"
        {
            return Err(Error::InvalidRequest(
                "malformed Paradex WebSocket envelope",
            ));
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
    pub fn sbe_frame(&self) -> Result<Option<sbe::DecodedFrame>, Error> {
        self.fields
            .get("sbe")
            .cloned()
            .map(serde_json::from_value)
            .transpose()
            .map_err(Error::Decode)
    }
}
struct Wire {
    channel: String,
    id: u64,
    account: Option<String>,
    tokens: Option<TokenState>,
}
fn auth_error() -> TransportError {
    TransportError::new(
        TransportErrorKind::Authentication,
        RequestOutcome::NotSent,
        "Paradex authentication failed; explicitly renew credentials before reconnecting",
    )
}
impl WebSocketSubscription for Wire {
    fn decode_binary_message(&self, bytes: &[u8]) -> Result<String, TransportError> {
        let frame = sbe::decode(bytes).map_err(|_| {
            TransportError::new(
                TransportErrorKind::Protocol,
                RequestOutcome::Unknown,
                "invalid or unsupported Paradex SBE frame",
            )
        })?;
        let data = frame.data().map_err(|_| {
            TransportError::new(
                TransportErrorKind::Protocol,
                RequestOutcome::Unknown,
                "invalid Paradex SBE payload",
            )
        })?;
        let template = frame.template_id();
        let base = match template {
            1 => "trades",
            2 => "bbo",
            3 => "order_book",
            4 => "markets_summary",
            5 => "funding_data",
            6 => "funding_rate_comparison",
            20 => "orders",
            21 => "fills",
            22 => "positions",
            23 => "account",
            40 => return Ok(json!({"method":"heartbeat","sbe":frame}).to_string()),
            41 => return Ok(json!({"method":"subscribed","sbe":frame}).to_string()),
            _ => {
                return Err(TransportError::new(
                    TransportErrorKind::Protocol,
                    RequestOutcome::Unknown,
                    "unsupported SBE template",
                ));
            }
        };
        if self.channel.split('.').next() != Some(base) {
            return Err(TransportError::new(
                TransportErrorKind::Protocol,
                RequestOutcome::Unknown,
                "SBE template does not match subscription",
            ));
        }
        if let Some(account) = &self.account
            && let Some(native_account) = data.get("account").and_then(Value::as_str)
            && felt(native_account).ok() != felt(account).ok()
        {
            return Err(TransportError::new(
                TransportErrorKind::Protocol,
                RequestOutcome::Unknown,
                "SBE account does not match authenticated session",
            ));
        }
        if let Some(market) = data.get("market").and_then(Value::as_str) {
            let subscribed = self.channel.split('.').nth(1).unwrap_or("ALL");
            if subscribed != "ALL" && !subscribed.starts_with("ALL@") && subscribed != market {
                return Err(TransportError::new(
                    TransportErrorKind::Protocol,
                    RequestOutcome::Unknown,
                    "SBE market does not match subscription",
                ));
            }
        }
        Ok(json!({"jsonrpc":"2.0","method":"subscription","params":{"channel":self.channel,"data":data},"sbe":frame}).to_string())
    }
    fn authentication_message(&self) -> AuthenticationMessage<'_> {
        Box::pin(async move {
            self.tokens.as_ref().map(|t|t.bearer(false).map(|token|json!({"jsonrpc":"2.0","id":"aux-auth","method":"auth","params":{"bearer":&*token}}).to_string()).map_err(|_|auth_error())).transpose()
        })
    }
    fn authentication_result(&self, text: &str) -> Option<Result<(), TransportError>> {
        let v: Value = serde_json::from_str(text).ok()?;
        if v["id"] != "aux-auth" {
            return None;
        }
        Some(
            if v.get("error").is_some_and(|e| !e.is_null()) || v.get("result").is_none() {
                Err(auth_error())
            } else {
                Ok(())
            },
        )
    }
    // One full channel per socket avoids ambiguous SBE book routing. It also
    // prevents token permissions from crossing independently owned auth states.
    fn connection_scope(&self) -> String {
        format!(
            "paradex-sbe-1:3:{}:{}",
            self.tokens
                .as_ref()
                .map(TokenState::scope)
                .unwrap_or_default(),
            self.channel
        )
    }
    fn key(&self) -> String {
        self.channel.clone()
    }
    fn requires_fresh_snapshot(&self) -> bool {
        self.channel.starts_with("order_book.")
    }
    fn is_initial_snapshot(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            v["params"]["data"]["pkgType"] == "SNAPSHOT"
                || v["params"]["data"]["update_type"] == "s"
                || v["params"]["data"]["update_type"] == "SNAPSHOT"
        })
    }
    fn is_snapshot_control(&self, text: &str) -> bool {
        !self.is_data_message(text)
    }
    fn is_data_message(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            v["params"]["channel"] == self.channel && v["params"].get("data").is_some()
        })
    }
    fn subscribe_message(&self) -> String {
        json!({"jsonrpc":"2.0","id":self.id,"method":"subscribe","params":{"channel":self.channel}})
            .to_string()
    }
    fn unsubscribe_message(&self) -> String {
        json!({"jsonrpc":"2.0","id":self.id,"method":"unsubscribe","params":{"channel":self.channel}}).to_string()
    }
    fn matches(&self, text: &str) -> bool {
        let Ok(v) = serde_json::from_str::<Value>(text) else {
            return true;
        };
        v["params"]["channel"] == self.channel
            || v["id"] == self.id
            || v["method"] == "heartbeat"
            || v["method"] == "subscribed"
            || v.get("id").is_none()
                && (v.get("error").is_some() || v["params"].get("channel").is_none())
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
                if !e.is_gap() {
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
impl<T: WebSocketTransport, A: Sync> ParadexClient<T, A> {
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
                    account: None,
                    tokens: None,
                },
            )
            .await?;
        Ok(SubscriptionHandle {
            stream,
            ended: false,
        })
    }
}
impl<T: WebSocketTransport, A: Authentication> ParadexClient<T, A> {
    pub async fn subscribe_private(
        &self,
        subscription: Subscription,
    ) -> Result<SubscriptionHandle<T::Stream>, ClientError<Error>> {
        let channel = subscription.channel().map_err(ClientError::Exchange)?;
        self.auth
            .token()
            .bearer(false)
            .map_err(ClientError::Exchange)?;
        let stream = self
            .transport
            .subscribe(
                self.exchange.ws_url(),
                Wire {
                    channel,
                    id: IDS.fetch_add(1, Ordering::Relaxed),
                    account: Some(self.auth.account().into()),
                    tokens: Some(self.auth.token().clone()),
                },
            )
            .await?;
        Ok(SubscriptionHandle {
            stream,
            ended: false,
        })
    }
}
