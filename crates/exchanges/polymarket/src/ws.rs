use crate::*;
use aux_rs_core::{
    AuthenticationMessage, HandshakeHeaders, RequestOutcome, TransportError, TransportErrorKind,
    Unauthenticated, WebSocketStream, WebSocketSubscription, WebSocketTransport,
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
    fn stream_expires_at(&self) -> Option<std::time::SystemTime> {
        None
    }
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
    credentials: Option<Value>,
    sequence: Mutex<Option<u64>>,
    sid: Mutex<Option<u64>>,
}
impl<A: StreamAuthentication> WebSocketSubscription for Spec<A> {
    fn authentication_expires_at(&self) -> Option<std::time::SystemTime> {
        self.auth.stream_expires_at()
    }
    fn resubscribe_allowed(&self) -> bool {
        self.auth
            .stream_expires_at()
            .is_none_or(|t| std::time::SystemTime::now() < t)
    }
    fn connection_scope(&self) -> String {
        format!(
            "polymarket:{}:{}:{}",
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

    fn authentication_message(&self) -> AuthenticationMessage<'_> {
        Box::pin(async move {
            let Some(c) = &self.credentials else {
                return Ok(None);
            };
            Ok(match self.subscription.service.as_str() {
                "polybolt" => Some(json!({"op":"auth","auth":c}).to_string()),
                "perps" => {
                    Some(json!({"id":0,"req":"post","op":{"type":"auth","args":c}}).to_string())
                }
                "rfq" => Some(c.to_string()),
                _ => None,
            })
        })
    }
    fn authentication_result(&self, text: &str) -> Option<Result<(), TransportError>> {
        let v: Value = serde_json::from_str(text).ok()?;
        match self.subscription.service.as_str() {
            "polybolt" if v["op"] == "authed" => Some(Ok(())),
            "polybolt" if v["op"] == "error" => Some(Err(authentication())),
            "perps" if v["id"] == 0 => Some(if v["data"]["status"] == "ok" {
                Ok(())
            } else {
                Err(authentication())
            }),
            "rfq" if v["type"] == "auth" => Some(if v["success"] == true {
                Ok(())
            } else {
                Err(authentication())
            }),
            _ => None,
        }
    }
    fn subscribe_message(&self) -> String {
        let p = &self.subscription.params;
        match self.subscription.service.as_str(){"market"=>json!({"type":"market","assets_ids":p["assets_ids"],"custom_feature_enabled":p.get("custom_feature_enabled").unwrap_or(&json!(true))}).to_string(),"user"=>json!({"type":"user","markets":p.get("markets").unwrap_or(&json!([])),"auth":self.credentials}).to_string(),"polybolt"=>json!({"op":"subscribe","subscriptions":[p]}).to_string(),"perps"=>json!({"id":self.id,"req":"sub","chs":[p["channel"]]}).to_string(),"rtds"=>json!({"action":"subscribe","subscriptions":[p]}).to_string(),"sports"|"rfq"=>String::new(),_=>String::new()}
    }
    fn unsubscribe_message(&self) -> String {
        let p = &self.subscription.params;
        match self.subscription.service.as_str() {
            "market" => json!({"operation":"unsubscribe","assets_ids":p["assets_ids"]}).to_string(),
            "user" => json!({"operation":"unsubscribe","markets":p["markets"]}).to_string(),
            "polybolt" => json!({"op":"unsubscribe","subscriptions":[p]}).to_string(),
            "perps" => json!({"id":self.id,"req":"unsub","chs":[p["channel"]]}).to_string(),
            "rtds" => json!({"action":"unsubscribe","subscriptions":[p]}).to_string(),
            _ => String::new(),
        }
    }
    fn requires_fresh_snapshot(&self) -> bool {
        self.subscription.service == "market" || self.subscription.service == "polybolt"
    }
    fn is_initial_snapshot(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            let check = |x: &Value| x["event_type"] == "book" || x["snapshot"] == true;
            if let Value::Array(xs) = &v {
                xs.iter().any(check)
            } else {
                check(&v)
            }
        })
    }
    fn is_snapshot_control(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text)
            .is_ok_and(|v| v.get("op").is_some() || v.get("error").is_some())
            || text == "PONG"
    }
    fn is_data_message(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            v.is_array()
                || v.get("event_type").is_some()
                || v.get("payload").is_some()
                || v.get("ch").is_some()
        })
    }
    fn heartbeat_message(&self) -> Option<String> {
        match self.subscription.service.as_str() {
            "market" | "user" | "sports" | "rtds" => Some("PING".into()),
            "perps" => Some(json!({"req":"post","op":{"type":"ping"}}).to_string()),
            _ => None,
        }
    }

    fn close_error(&self, code: u16) -> Option<TransportError> {
        match code {
            4001 | 1008 => Some(authentication()),
            4002 | 4008 => Some(protocol("native WebSocket policy/slow-consumer close")),
            _ => None,
        }
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
        if v["op"] == "error" || v["event_type"] == "error" || v["type"] == "error" {
            return Err(protocol("Polymarket subscription rejected"));
        }
        if self.subscription.service == "polybolt" {
            if v["dropped"].as_u64().is_some_and(|x| x > 0) {
                return Err(TransportError::new(
                    TransportErrorKind::Disconnected,
                    RequestOutcome::Unknown,
                    "PolyBolt dropped updates",
                ));
            }
            if let Some(seq) = v["seq"].as_u64() {
                let mut last = self.sequence.lock().unwrap_or_else(|e| e.into_inner());
                if last.is_some_and(|p| seq != p.saturating_add(1)) {
                    return Err(TransportError::new(
                        TransportErrorKind::Disconnected,
                        RequestOutcome::Unknown,
                        "PolyBolt sequence discontinuity",
                    ));
                }
                *last = Some(seq);
            }
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
                if matches!(subscription.service.as_str(), "user" | "polybolt" | "rfq")
                    && credentials.is_none()
                {
                    return Err(aux_rs_core::Error::Exchange(Error::Authentication(
                        "WebSocket API credentials required",
                    )));
                }
                let spec = Spec {
                    id: NEXT.fetch_add(1, Ordering::Relaxed),
                    url: url.clone(),
                    subscription,
                    auth: self.auth.clone(),
                    credentials,
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
subscriptions!(PolymarketClient);
subscriptions!(PolymarketPerpsClient);

impl StreamAuthentication for Unauthenticated {
    fn stream_identity(&self) -> String {
        "public".into()
    }
    fn stream_credentials(&self, _: &str) -> Result<Option<Value>, Error> {
        Ok(None)
    }
}
impl<S: EvmSigner + Clone + 'static> StreamAuthentication for PolymarketAuth<S> {
    fn stream_identity(&self) -> String {
        self.identity()
    }
    fn stream_credentials(&self, service: &str) -> Result<Option<Value>, Error> {
        if matches!(service, "user" | "polybolt" | "rfq") {
            let c = self
                .api
                .as_ref()
                .ok_or(Error::Authentication("CLOB credentials required"))?
                .websocket_auth();
            if service == "rfq" {
                Ok(Some(
                    json!({"type":"auth","auth":c,"identity":{"signer_address":self.address,"maker_address":self.wallet,"signature_type":self.rfq_signature_type}}),
                ))
            } else {
                Ok(Some(c))
            }
        } else {
            Ok(None)
        }
    }
}
impl<S: EvmSigner + Clone + 'static> StreamAuthentication for PolymarketPerpsAuth<S> {
    fn stream_expires_at(&self) -> Option<std::time::SystemTime> {
        self.credentials
            .as_ref()?
            .expires_at_ms
            .and_then(|ms| std::time::UNIX_EPOCH.checked_add(std::time::Duration::from_millis(ms)))
    }
    fn stream_identity(&self) -> String {
        self.identity()
    }
    fn stream_credentials(&self, service: &str) -> Result<Option<Value>, Error> {
        if service == "perps" {
            self.credentials
                .as_ref()
                .map(PerpsCredentials::websocket_auth)
                .transpose()
        } else {
            Err(Error::Authentication(
                "perps credentials only authorize perps sockets",
            ))
        }
    }
}
