use crate::*;
use aux_rs_core::{
    HandshakeHeaders, RequestOutcome, TransportError, TransportErrorKind, WebSocketStream,
    WebSocketSubscription, WebSocketTransport,
};
use serde_json::{Value, json};
use std::{
    future::Future,
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
};
static NEXT: AtomicU64 = AtomicU64::new(1);
fn protocol(message: &'static str) -> TransportError {
    TransportError::new(
        TransportErrorKind::Protocol,
        RequestOutcome::Unknown,
        message,
    )
}
fn authentication() -> TransportError {
    TransportError::new(
        TransportErrorKind::Authentication,
        RequestOutcome::NotSent,
        "native WebSocket authentication failed",
    )
}
pub trait StreamAuthentication: Clone + Send + Sync + 'static {
    fn stream_identity(&self) -> String;
    fn stream_headers(
        &self,
        _url: &str,
    ) -> impl Future<Output = Result<Vec<(String, String)>, Error>> + Send {
        async { Ok(vec![]) }
    }
    fn stream_credentials(&self, service: &str) -> Result<Option<Value>, Error>;
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Subscription {
    pub service: String,
    #[serde(default)]
    pub params: Value,
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Event(pub Value);
impl Event {
    pub fn payload<T: serde::de::DeserializeOwned>(&self) -> Result<T, Error> {
        Ok(serde_json::from_value(self.0.clone())?)
    }
}
pub struct SubscriptionHandle<S> {
    inner: S,
}
impl<S: WebSocketStream> SubscriptionHandle<S> {
    pub async fn next(&mut self) -> Option<Result<Event, aux_rs_core::Error<Error>>> {
        let raw = match self.inner.next().await? {
            Ok(s) => s,
            Err(e) => return Some(Err(e.into())),
        };
        if raw == "PONG" || raw == "PING" {
            return Some(Ok(Event(json!({"type":raw}))));
        }
        Some(
            serde_json::from_str(&raw)
                .map(Event)
                .map_err(|e| aux_rs_core::Error::Exchange(e.into())),
        )
    }
    pub async fn close(&mut self) -> Result<(), aux_rs_core::Error<Error>> {
        self.inner.close().await.map_err(Into::into)
    }
}
struct Spec<A> {
    id: u64,
    url: String,
    subscription: Subscription,
    auth: A,
    _credentials: Option<Value>,
    sequence: Mutex<Option<u64>>,
    sid: Mutex<Option<u64>>,
}
impl<A: StreamAuthentication> WebSocketSubscription for Spec<A> {
    fn connection_scope(&self) -> String {
        format!(
            "kalshi:{}:{}:{}",
            self.auth.stream_identity(),
            self.subscription.service,
            self.id
        )
    }
    fn key(&self) -> String {
        self.id.to_string()
    }
    fn handshake_headers(&self) -> HandshakeHeaders<'_> {
        Box::pin(async move {
            *self.sequence.lock().unwrap_or_else(|e| e.into_inner()) = None;
            *self.sid.lock().unwrap_or_else(|e| e.into_inner()) = None;
            self.auth
                .stream_headers(&self.url)
                .await
                .map_err(|_| authentication())
        })
    }

    fn subscribe_message(&self) -> String {
        let mut p = self.subscription.params.clone();
        p["channels"] = json!([self.subscription.params["channel"]]);
        p.as_object_mut()
            .expect("validated params")
            .remove("channel");
        json!({"id":self.id,"cmd":"subscribe","params":p}).to_string()
    }
    fn unsubscribe_message(&self) -> String {
        json!({"id":self.id,"cmd":"unsubscribe","params":{"sids":[*self.sid.lock().unwrap_or_else(|e|e.into_inner())]}}).to_string()
    }
    fn requires_fresh_snapshot(&self) -> bool {
        self.subscription.params["channel"] == "orderbook_delta"
    }
    fn is_initial_snapshot(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| v["type"] == "orderbook_snapshot")
    }
    fn is_snapshot_control(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            !matches!(
                v["type"].as_str(),
                Some("orderbook_snapshot" | "orderbook_delta")
            )
        })
    }
    fn is_data_message(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            !matches!(
                v["type"].as_str(),
                Some("subscribed" | "ok" | "error" | "unsubscribed")
            )
        })
    }

    fn matches(&self, _: &str) -> bool {
        true
    }
    fn validate_message(&self, text: &str) -> Result<(), TransportError> {
        if matches!(text, "PONG" | "PING") {
            return Ok(());
        }
        let v: Value =
            serde_json::from_str(text).map_err(|_| protocol("malformed native WebSocket JSON"))?;
        if v["type"] == "error" {
            return Err(protocol("Kalshi subscription rejected"));
        }
        if v["type"] == "subscribed" {
            *self.sid.lock().unwrap_or_else(|e| e.into_inner()) = v["msg"]["sid"].as_u64();
        }
        if let Some(seq) = v["seq"].as_u64() {
            let mut last = self.sequence.lock().unwrap_or_else(|e| e.into_inner());
            if last.is_some_and(|p| seq != p.saturating_add(1)) {
                return Err(TransportError::new(
                    TransportErrorKind::Disconnected,
                    RequestOutcome::Unknown,
                    "Kalshi sequence discontinuity; fresh snapshot required",
                ));
            }
            *last = Some(seq);
        }
        Ok(())
    }
}
macro_rules! subscriptions {
    ($client:ident) => {
        impl<T: WebSocketTransport, A: StreamAuthentication> $client<T, A> {
            pub async fn subscribe(
                &self,
                subscription: Subscription,
            ) -> Result<SubscriptionHandle<T::Stream>, aux_rs_core::Error<Error>> {
                if !self.transport.supports_handshake_headers() {
                    return Err(aux_rs_core::Error::Transport(TransportError::new(
                        TransportErrorKind::Configuration,
                        RequestOutcome::NotSent,
                        "custom WebSocket transport does not support handshake headers",
                    )));
                }
                let url = self
                    .exchange
                    .websocket
                    .get(&subscription.service)
                    .ok_or(aux_rs_core::Error::Exchange(Error::InvalidRequest(
                        "unknown WebSocket service",
                    )))?
                    .clone();
                let credentials = self
                    .auth
                    .stream_credentials(&subscription.service)
                    .map_err(aux_rs_core::Error::Exchange)?;
                let channel =
                    subscription.params["channel"]
                        .as_str()
                        .ok_or(aux_rs_core::Error::Exchange(Error::InvalidRequest(
                            "channel required",
                        )))?;
                let allowed: serde_json::Value =
                    serde_json::from_str(include_str!("../schema/websocket.json"))
                        .expect("pinned channels");
                if !allowed
                    .as_array()
                    .expect("inventory")
                    .iter()
                    .any(|v| v["channel"] == channel)
                {
                    return Err(aux_rs_core::Error::Exchange(Error::InvalidRequest(
                        "unknown Kalshi channel",
                    )));
                }
                if subscription.service.ends_with("warp")
                    && !matches!(channel, "fill" | "user_orders")
                {
                    return Err(aux_rs_core::Error::Exchange(Error::InvalidRequest(
                        "dedicated host only serves fill/user_orders",
                    )));
                }
                let spec = Spec {
                    id: NEXT.fetch_add(1, Ordering::Relaxed),
                    url: url.clone(),
                    subscription,
                    auth: self.auth.clone(),
                    _credentials: credentials,
                    sequence: Mutex::new(None),
                    sid: Mutex::new(None),
                };
                Ok(SubscriptionHandle {
                    inner: self.transport.subscribe(&url, spec).await?,
                })
            }
        }
    };
}
subscriptions!(KalshiClient);
subscriptions!(KalshiPerpsClient);

impl<S: RequestSigner + Clone> StreamAuthentication for KalshiAuth<S> {
    fn stream_identity(&self) -> String {
        self.identity()
    }
    async fn stream_headers(&self, u: &str) -> Result<Vec<(String, String)>, Error> {
        self.websocket_headers(u).await
    }
    fn stream_credentials(&self, _: &str) -> Result<Option<Value>, Error> {
        Ok(None)
    }
}
