// Native subscription framing and continuity. Transport owns bounded recovery.
use crate::*;
use aux_rs_core::{
    HandshakeHeaders, RequestOutcome, TransportError, TransportErrorKind, WebSocketStream,
    WebSocketSubscription, WebSocketTransport,
};
use serde_json::{Value, json};
use std::sync::{
    Mutex,
    atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(1);
fn protocol(message: &'static str) -> TransportError {
    TransportError::new(
        TransportErrorKind::Protocol,
        RequestOutcome::Unknown,
        message,
    )
}
fn gap() -> TransportError {
    TransportError::new(
        TransportErrorKind::Disconnected,
        RequestOutcome::Unknown,
        "native stream sequence gap; fresh snapshot required",
    )
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
        let text = match self.inner.next().await? {
            Ok(t) => t,
            Err(e) => return Some(Err(e.into())),
        };
        let value = match decode_event(&text) {
            Ok(v) => v,
            Err(e) => return Some(Err(aux_rs_core::Error::Exchange(e))),
        };
        if value["type"] == "subscriptionError" || value["channel"] == "error" {
            let _ = self.inner.close().await;
            return Some(Err(aux_rs_core::Error::Exchange(Error::Api {
                status: None,
                body: value,
            })));
        }
        Some(Ok(Event(value)))
    }
    pub async fn close(&mut self) -> Result<(), aux_rs_core::Error<Error>> {
        self.inner.close().await.map_err(Into::into)
    }
}
pub trait StreamAuthentication: Clone + Send + Sync + 'static {
    fn stream_headers(&self, private: bool) -> Result<Vec<(String, String)>, Error>;
    fn stream_identity(&self) -> String;
}
impl StreamAuthentication for aux_rs_core::Unauthenticated {
    fn stream_headers(&self, private: bool) -> Result<Vec<(String, String)>, Error> {
        if private {
            Err(Error::Authentication("private stream requires credentials"))
        } else {
            Ok(vec![])
        }
    }
    fn stream_identity(&self) -> String {
        "public".into()
    }
}
impl StreamAuthentication for Credentials {
    fn stream_headers(&self, private: bool) -> Result<Vec<(String, String)>, Error> {
        self.headers(private)
    }
    fn stream_identity(&self) -> String {
        self.identity()
    }
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Subscription(pub Value);
impl Subscription {
    pub fn validate(&self) -> Result<(), Error> {
        let channel = self.0["channel"]
            .as_str()
            .ok_or(Error::InvalidRequest("channel required"))?;
        if !matches!(
            channel,
            "allMids"
                | "exchange"
                | "fundingRate"
                | "orderbook"
                | "traderState"
                | "market"
                | "trades"
                | "candles"
                | "notifications"
        ) {
            return Err(Error::InvalidRequest("unsupported Phoenix channel"));
        }
        if matches!(
            channel,
            "fundingRate" | "orderbook" | "market" | "trades" | "candles"
        ) && self.0["symbol"].as_str().is_none_or(str::is_empty)
        {
            return Err(Error::InvalidRequest("symbol required"));
        }
        if channel == "traderState" && self.0["authority"].as_str().is_none_or(str::is_empty) {
            return Err(Error::InvalidRequest("trader authority required"));
        }
        Ok(())
    }
}
pub fn decode_event(text: &str) -> Result<Value, Error> {
    use base64::Engine;
    let mut value: Value = serde_json::from_str(text)?;
    if value["messageType"] == "encodedSnapshot" {
        if value["encoding"] != "base64+zstd" {
            return Err(Error::InvalidRequest("unsupported snapshot encoding"));
        }
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(
                value["payload"]
                    .as_str()
                    .ok_or(Error::InvalidRequest("missing compressed snapshot"))?,
            )
            .map_err(|_| Error::InvalidRequest("invalid snapshot base64"))?;
        use std::io::Read;
        let decoder = zstd::stream::read::Decoder::new(bytes.as_slice())
            .map_err(|_| Error::InvalidRequest("invalid zstd snapshot"))?;
        let mut decoded = vec![];
        decoder
            .take(8 * 1024 * 1024 + 1)
            .read_to_end(&mut decoded)
            .map_err(|_| Error::InvalidRequest("invalid zstd snapshot"))?;
        if decoded.len() > 8 * 1024 * 1024 {
            return Err(Error::InvalidRequest("decompressed snapshot too large"));
        }
        let payload: Value = serde_json::from_slice(&decoded)?;
        value["decodedPayload"] = payload;
    }
    Ok(value)
}
struct Spec<A> {
    id: u64,
    subscription: Subscription,
    auth: A,
    sequence: Mutex<Option<u64>>,
}
impl<A: StreamAuthentication> WebSocketSubscription for Spec<A> {
    fn key(&self) -> String {
        self.subscription.0.to_string()
    }
    fn connection_scope(&self) -> String {
        format!("phoenix:{}:{}", self.auth.stream_identity(), self.id)
    }
    fn handshake_headers(&self) -> HandshakeHeaders<'_> {
        Box::pin(async move {
            *self.sequence.lock().unwrap_or_else(|e| e.into_inner()) = None;
            self.auth
                .stream_headers(self.subscription.0["channel"] == "notifications")
                .map_err(|_| {
                    TransportError::new(
                        TransportErrorKind::Authentication,
                        RequestOutcome::NotSent,
                        "stream credentials missing or expired",
                    )
                })
        })
    }
    fn subscribe_message(&self) -> String {
        json!({"type":"subscribe","subscription":self.subscription}).to_string()
    }
    fn unsubscribe_message(&self) -> String {
        json!({"type":"unsubscribe","subscription":self.subscription}).to_string()
    }
    fn requires_fresh_snapshot(&self) -> bool {
        matches!(
            self.subscription.0["channel"].as_str(),
            Some("exchange" | "traderState")
        )
    }
    fn is_initial_snapshot(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            matches!(
                v["messageType"].as_str(),
                Some("snapshot" | "encodedSnapshot")
            )
        })
    }
    fn is_snapshot_control(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            matches!(
                v["type"].as_str(),
                Some("subscriptionConfirmed" | "subscriptionError")
            ) || v["channel"] == "error"
        })
    }
    fn is_data_message(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text)
            .is_ok_and(|v| v.get("channel").is_some() && v["channel"] != "error")
    }
    fn matches(&self, _: &str) -> bool {
        true
    }
    fn validate_message(&self, text: &str) -> Result<(), TransportError> {
        let v: Value =
            serde_json::from_str(text).map_err(|_| protocol("malformed Phoenix frame"))?;
        if v["type"] == "subscriptionError" || v["channel"] == "error" {
            return Ok(());
        }
        let seq = v["sequenceNumber"]
            .as_u64()
            .or_else(|| v["sequenceNumber"].as_str().and_then(|s| s.parse().ok()));
        if let Some(seq) = seq {
            let mut prev = self.sequence.lock().unwrap_or_else(|e| e.into_inner());
            if prev.is_some_and(|p| p.checked_add(1) != Some(seq))
                && !self.is_initial_snapshot(text)
            {
                return Err(gap());
            }
            *prev = Some(seq);
        }
        Ok(())
    }
}
impl<T: WebSocketTransport, A: StreamAuthentication> PhoenixClient<T, A> {
    pub async fn subscribe(
        &self,
        subscription: Subscription,
    ) -> Result<SubscriptionHandle<T::Stream>, aux_rs_core::Error<Error>> {
        subscription
            .validate()
            .map_err(aux_rs_core::Error::Exchange)?;
        let private = subscription.0["channel"] == "notifications";
        self.auth
            .stream_headers(private)
            .map_err(aux_rs_core::Error::Exchange)?;
        if private && !self.transport.supports_handshake_headers() {
            return Err(aux_rs_core::Error::Exchange(Error::InvalidRequest(
                "private streams require handshake-header support",
            )));
        }
        let spec = Spec {
            id: NEXT.fetch_add(1, Ordering::Relaxed),
            subscription,
            auth: self.auth.clone(),
            sequence: Mutex::new(None),
        };
        Ok(SubscriptionHandle {
            inner: self
                .transport
                .subscribe(aux_rs_core::Exchange::ws_url(&self.exchange), spec)
                .await?,
        })
    }
}
