use crate::*;
use aux_rs_core::{
    AuthenticationChanged, AuthenticationMessage, Error as ClientError, Exchange, RequestOutcome,
    TransportError, TransportErrorKind, WebSocketStream, WebSocketSubscription, WebSocketTransport,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{future::Future, pin::Pin, sync::Arc, time::SystemTime};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "snake_case",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum Subscription {
    Trades {
        trading_pair_id: String,
    },
    Orderbook {
        trading_pair_id: String,
        mode: String,
        #[serde(default)]
        magnitude: Option<String>,
        #[serde(default)]
        denomination: Option<String>,
    },
    Depth {
        trading_pair_id: String,
        mode: String,
    },
    Bbo {
        trading_pair_id: String,
        mode: String,
    },
}
impl Subscription {
    pub fn channel(&self) -> Result<String, Error> {
        let (kind, id, mode) = match self {
            Self::Trades { trading_pair_id } => ("trades", trading_pair_id, None),
            Self::Orderbook {
                trading_pair_id,
                mode,
                ..
            } => ("orderbook", trading_pair_id, Some(mode)),
            Self::Depth {
                trading_pair_id,
                mode,
            } => ("depth", trading_pair_id, Some(mode)),
            Self::Bbo {
                trading_pair_id,
                mode,
            } => ("bbo", trading_pair_id, Some(mode)),
        };
        if id != "*" || kind != "trades" {
            uuid(id)?;
        }
        if let Some(mode) = mode {
            one_of(mode, &["SPOT", "MARGIN"])?;
        }
        let mut channel = format!("{kind}:{id}");
        if let Some(mode) = mode {
            channel.push(':');
            channel.push_str(mode);
        }
        if let Self::Orderbook {
            magnitude,
            denomination,
            ..
        } = self
        {
            if magnitude.is_some() || denomination.is_some() {
                let m = magnitude.as_deref().unwrap_or("*");
                one_of(
                    m,
                    &[
                        "*", "0.000001", "0.00001", "0.0001", "0.001", "0.01", "0.1", "1", "10",
                        "100", "1000", "10000",
                    ],
                )?;
                channel.push(':');
                channel.push_str(m);
            }
            if let Some(d) = denomination {
                one_of(d, &["base", "quote", "*"])?;
                channel.push(':');
                channel.push_str(d);
            }
        }
        Ok(canonical(channel))
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "snake_case",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum AccountSubscription {
    Orders {
        #[serde(default)]
        trading_pair_id: Option<String>,
        #[serde(default)]
        mode: Option<String>,
    },
    Balances,
    Positions {
        #[serde(default)]
        trading_pair_id: Option<String>,
    },
    Account,
}
fn canonical(channel: String) -> String {
    channel
        .split(':')
        .map(|part| match part {
            "SPOT" | "MARGIN" => part.to_owned(),
            _ => part.to_ascii_lowercase(),
        })
        .collect::<Vec<_>>()
        .join(":")
}
impl AccountSubscription {
    pub fn channel(&self) -> Result<String, Error> {
        let result = match self {
            Self::Balances => Ok("balances".into()),
            Self::Account => Ok("account".into()),
            Self::Positions { trading_pair_id } => {
                if let Some(id) = trading_pair_id {
                    uuid(id)?;
                    Ok(format!("positions:{id}"))
                } else {
                    Ok("positions".into())
                }
            }
            Self::Orders {
                trading_pair_id,
                mode,
            } => {
                if let Some(id) = trading_pair_id {
                    uuid(id)?;
                }
                if let Some(m) = mode {
                    one_of(m, &["SPOT", "MARGIN"])?;
                }
                Ok(match (trading_pair_id, mode) {
                    (None, None) => "orders".into(),
                    (Some(id), None) => format!("orders:{id}"),
                    (id, Some(m)) => format!("orders:{}:{m}", id.as_deref().unwrap_or("*")),
                })
            }
        };
        result.map(canonical)
    }
}
trait StreamAuth: Send + Sync {
    fn active(&self) -> bool;
    fn reject(&self, body: Value);
    fn failure(&self) -> Option<Value>;
    fn expires(&self) -> SystemTime;
    fn changed(&self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
    fn message(&self) -> AuthenticationMessage<'_>;
}
impl<S: SessionSigner + 'static> StreamAuth for MonacoAuth<S> {
    fn active(&self) -> bool {
        self.is_active()
    }
    fn reject(&self, body: Value) {
        self.reject(body)
    }
    fn failure(&self) -> Option<Value> {
        self.failure()
    }
    fn expires(&self) -> SystemTime {
        self.expires_at()
    }
    fn changed(&self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        self.changed()
    }
    fn message(&self) -> AuthenticationMessage<'_> {
        Box::pin(async move {
            let timestamp = unix_millis().map_err(|_| auth_error())?;
            let public = aux_rs_core::hex::encode(self.signer.public_key());
            let signature = self
                .sign_checked(format!("WS-AUTH\n{public}\n{timestamp}").as_bytes())
                .await
                .map_err(|_| auth_error())?;
            Ok(Some(json!({"type":"Authenticate","public_key":public,"timestamp":timestamp,"signature":aux_rs_core::hex::encode(signature)}).to_string()))
        })
    }
}
fn auth_error() -> TransportError {
    TransportError::new(
        TransportErrorKind::Authentication,
        RequestOutcome::NotSent,
        "Monaco session expired, revoked or authentication failed",
    )
}
fn gap() -> ClientError<Error> {
    ClientError::Transport(TransportError::new(
        TransportErrorKind::Gap,
        RequestOutcome::Unknown,
        "Monaco baseline unavailable or stream sequence lost; reconcile a fresh snapshot",
    ))
}
struct WireSubscription {
    channel: String,
    auth: Option<Arc<dyn StreamAuth>>,
    scope: String,
    snapshot: bool,
}
fn matches_channel(pattern: &str, actual: &str) -> bool {
    let wanted: Vec<_> = pattern.split(':').collect();
    let actual: Vec<_> = actual.split(':').collect();
    wanted.len() <= actual.len() && wanted.iter().zip(actual).all(|(w, a)| *w == "*" || *w == a)
}
impl WireSubscription {
    // The engine can keep publishing after a failed account/history baseline.
    // Signal the gap and allow callers to reconcile REST with those live frames.
    // Sequenced market-data channels instead wait for the server's fresh snapshot.
    fn baseline_must_be_refetched(&self, v: &Value) -> bool {
        v["type"] == "Error"
            && v["code"] == "SNAPSHOT_UNAVAILABLE"
            && !self.channel.starts_with("depth:")
            && !self.channel.starts_with("bbo:")
    }
}
impl WebSocketSubscription for WireSubscription {
    fn authentication_message(&self) -> AuthenticationMessage<'_> {
        match &self.auth {
            Some(a) => a.message(),
            None => Box::pin(std::future::ready(Ok(None))),
        }
    }
    fn authentication_result(&self, text: &str) -> Option<Result<(), TransportError>> {
        let v: Value = match serde_json::from_str(text) {
            Ok(v) => v,
            Err(_) => {
                return Some(Err(TransportError::new(
                    TransportErrorKind::Protocol,
                    RequestOutcome::Unknown,
                    "invalid Monaco authentication response",
                )));
            }
        };
        match v["type"].as_str() {
            Some("Authenticated") => Some(Ok(())),
            Some("Error") => Some(Err(auth_error())),
            _ => None,
        }
    }
    fn authentication_expires_at(&self) -> Option<SystemTime> {
        self.auth.as_ref().map(|a| a.expires())
    }
    fn authentication_changed(&self) -> Option<AuthenticationChanged<'_>> {
        self.auth.as_ref().map(|a| a.changed())
    }
    fn resubscribe_allowed(&self) -> bool {
        self.auth.as_ref().is_none_or(|a| a.active())
    }
    fn connection_scope(&self) -> String {
        self.scope.clone()
    }
    fn key(&self) -> String {
        self.channel.clone()
    }
    fn subscribe_message(&self) -> String {
        json!({"type":"Subscribe","channels":[self.channel]}).to_string()
    }
    fn unsubscribe_message(&self) -> String {
        json!({"type":"Unsubscribe","channels":[self.channel]}).to_string()
    }
    fn requires_fresh_snapshot(&self) -> bool {
        self.snapshot
    }
    fn is_snapshot_control(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            if self.baseline_must_be_refetched(&v) {
                return false;
            }
            matches!(
                v["type"].as_str(),
                Some("Subscribed" | "Unsubscribed" | "Error" | "Authenticated" | "Pong")
            )
        })
    }
    fn is_initial_snapshot(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text)
            .is_ok_and(|v| v["type"] == "Snapshot" || self.baseline_must_be_refetched(&v))
    }
    fn is_data_message(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text)
            .is_ok_and(|v| matches!(v["type"].as_str(), Some("Event" | "Snapshot")))
    }
    fn heartbeat_message(&self) -> Option<String> {
        Some(json!({"type":"Ping"}).to_string())
    }
    fn response_message(&self, text: &str) -> Option<String> {
        serde_json::from_str::<Value>(text)
            .ok()
            .filter(|v| v["type"] == "Ping")
            .map(|_| json!({"type":"Pong"}).to_string())
    }
    fn matches(&self, text: &str) -> bool {
        let Ok(v) = serde_json::from_str::<Value>(text) else {
            return true;
        };
        if v["type"] == "Error"
            && matches!(
                v["code"].as_str(),
                Some(
                    "AUTH_FAILED"
                        | "SESSION_INVALID"
                        | "ACCOUNT_DEACTIVATED"
                        | "ACCOUNT_RESTRICTED"
                )
            )
            && let Some(auth) = &self.auth
        {
            auth.reject(v.clone());
        }
        if matches!(v["type"].as_str(), Some("Authenticated" | "Ping" | "Pong")) {
            return false;
        }
        if let Some(c) = v.get("channel").and_then(Value::as_str) {
            return matches_channel(&self.channel, c);
        }
        if let Some(cs) = v.get("channels").and_then(Value::as_array) {
            return cs.iter().any(|c| c.as_str() == Some(&self.channel));
        }
        v["type"] == "Error"
    }
}
/// Native envelopes retain snake_case stream data and unknown fields unchanged.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
impl Event {
    pub fn decode(text: &str) -> Result<Self, Error> {
        serde_json::from_str(text).map_err(Error::Decode)
    }
}
pub struct SubscriptionHandle<S> {
    stream: S,
    auth: Option<Arc<dyn StreamAuth>>,
    pending: Option<Event>,
    book_seq: Option<u64>,
    depth: bool,
    ended: bool,
}
impl<S: WebSocketStream> SubscriptionHandle<S> {
    pub async fn next(&mut self) -> Option<Result<Event, ClientError<Error>>> {
        if self.ended {
            return None;
        }
        if self.auth.as_ref().is_some_and(|a| !a.active()) {
            self.ended = true;
            let _ = self.stream.close().await;
            if let Some(body) = self.auth.as_ref().and_then(|a| a.failure()) {
                return Some(Err(ClientError::Exchange(Error::Api {
                    status: None,
                    body,
                })));
            }
            return Some(Err(ClientError::Transport(auth_error())));
        }
        if let Some(event) = self.pending.take() {
            return Some(Ok(event));
        }
        let raw = match self.stream.next().await? {
            Ok(s) => s,
            Err(e) => {
                self.book_seq = None;
                if !e.is_gap() {
                    self.ended = true;
                    let _ = self.stream.close().await;
                }
                if let Some(body) = self.auth.as_ref().and_then(|a| a.failure()) {
                    return Some(Err(ClientError::Exchange(Error::Api {
                        status: None,
                        body,
                    })));
                }
                return Some(Err(ClientError::Transport(e)));
            }
        };
        let event = match Event::decode(&raw) {
            Ok(e) => e,
            Err(e) => {
                self.ended = true;
                let _ = self.stream.close().await;
                return Some(Err(ClientError::Exchange(e)));
            }
        };
        if event.kind == "Error" {
            if event.extra.get("code").and_then(Value::as_str) == Some("SNAPSHOT_UNAVAILABLE") {
                self.book_seq = None;
                self.pending = Some(event);
                return Some(Err(gap()));
            }
            self.ended = true;
            let _ = self.stream.close().await;
            return Some(Err(ClientError::Exchange(Error::Api {
                status: None,
                body: serde_json::from_str(&raw).unwrap_or(Value::Null),
            })));
        }
        if self.depth
            && let Some(data) = &event.data
        {
            let seq = |key: &str| {
                data.get(key).and_then(|v| {
                    v.as_u64()
                        .or_else(|| v.as_str().and_then(|s| s.parse::<u64>().ok()))
                })
            };
            if event.kind == "Snapshot" {
                self.book_seq = seq("book_seq");
            } else if event.kind == "Event" {
                let previous = seq("prev_book_seq");
                let current = seq("book_seq");
                if self.book_seq.is_none()
                    || previous.is_none()
                    || current.is_none()
                    || previous > self.book_seq
                {
                    self.book_seq = None;
                    self.ended = true;
                    let _ = self.stream.close().await;
                    return Some(Err(gap()));
                }
                self.book_seq = self.book_seq.max(current);
            }
        }
        Some(Ok(event))
    }
    pub async fn close(&mut self) -> Result<(), ClientError<Error>> {
        self.pending = None;
        self.ended = true;
        self.stream.close().await.map_err(ClientError::Transport)
    }
}
impl<T: WebSocketTransport, A: Sync> MonacoClient<T, A> {
    pub async fn subscribe(
        &self,
        subscription: Subscription,
    ) -> Result<SubscriptionHandle<T::Stream>, ClientError<Error>> {
        let channel = subscription.channel().map_err(ClientError::Exchange)?;
        let snapshot = channel != "trades:*";
        let depth = matches!(subscription, Subscription::Depth { .. });
        let stream = self
            .transport
            .subscribe(
                self.exchange.ws_url(),
                WireSubscription {
                    channel,
                    auth: None,
                    scope: String::new(),
                    snapshot,
                },
            )
            .await?;
        Ok(SubscriptionHandle {
            stream,
            auth: None,
            pending: None,
            book_seq: None,
            depth,
            ended: false,
        })
    }
}
impl<T: WebSocketTransport, S: SessionSigner + 'static> MonacoClient<T, MonacoAuth<S>> {
    pub async fn subscribe_account(
        &self,
        subscription: AccountSubscription,
    ) -> Result<SubscriptionHandle<T::Stream>, ClientError<Error>> {
        self.auth
            .check(&self.exchange)
            .map_err(ClientError::Exchange)?;
        let channel = subscription.channel().map_err(ClientError::Exchange)?;
        let info = self.auth.session_info();
        let scope = json!([
            "monaco",
            self.exchange.http_url(),
            info.chain_id,
            info.client_id,
            info.address,
            info.session_public_key,
            self.auth.session_id()
        ])
        .to_string();
        let auth: Arc<dyn StreamAuth> = Arc::new(self.auth.clone());
        let stream = self
            .transport
            .subscribe(
                self.exchange.ws_url(),
                WireSubscription {
                    channel,
                    auth: Some(auth.clone()),
                    scope,
                    snapshot: true,
                },
            )
            .await?;
        Ok(SubscriptionHandle {
            stream,
            auth: Some(auth),
            pending: None,
            book_seq: None,
            depth: false,
            ended: false,
        })
    }
}
