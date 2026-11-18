use crate::*;
use aux_rs_core::{
    HandshakeHeaders, RequestOutcome, TransportError, TransportErrorKind, WebSocketSubscription,
    WebSocketTransport,
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::{Value, json};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
static NEXT_SCOPE: AtomicU64 = AtomicU64::new(0);
use zeroize::Zeroizing;
struct Spec {
    request: Option<Value>,
    scope: u64,
    token: Arc<Zeroizing<String>>,
}
impl WebSocketSubscription for Spec {
    fn handshake_headers(&self) -> HandshakeHeaders<'_> {
        Box::pin(async move {
            Ok(vec![(
                "Authorization".into(),
                format!("Bearer {}", self.token.as_str()),
            )])
        })
    }
    fn connection_scope(&self) -> String {
        format!("pyth:{}", self.scope)
    }
    fn key(&self) -> String {
        self.scope.to_string()
    }
    fn subscribe_message(&self) -> String {
        let Some(mut r) = self.request.clone() else {
            return String::new();
        };
        r["type"] = json!("subscribe");
        r.to_string()
    }
    fn unsubscribe_message(&self) -> String {
        self.request.as_ref().map_or_else(String::new, |r| {
            json!({"type":"unsubscribe","subscriptionId":r["subscriptionId"]}).to_string()
        })
    }
    fn matches(&self, text: &str) -> bool {
        let Some(request) = &self.request else {
            return true;
        };
        serde_json::from_str::<Value>(text).map_or(true, |v| {
            v.get("subscriptionId")
                .is_none_or(|s| s == &request["subscriptionId"])
        })
    }
    fn is_data_message(&self, text: &str) -> bool {
        self.request.is_none()
            || serde_json::from_str::<Value>(text).is_ok_and(|v| v["type"] == "streamUpdated")
    }
    fn decode_binary_message(&self, bytes: &[u8]) -> std::result::Result<String, TransportError> {
        decode_binary(bytes).ok_or_else(|| {
            TransportError::new(
                TransportErrorKind::Protocol,
                RequestOutcome::Unknown,
                "invalid Pyth binary update",
            )
        })
    }
    fn validate_message(&self, text: &str) -> std::result::Result<(), TransportError> {
        let v: Value = serde_json::from_str(text).map_err(|_| {
            TransportError::new(
                TransportErrorKind::Protocol,
                RequestOutcome::Unknown,
                "invalid Pyth stream JSON",
            )
        })?;
        if !v.is_object() || (self.request.is_some() && v["type"].as_str().is_none()) {
            return Err(TransportError::new(
                TransportErrorKind::Protocol,
                RequestOutcome::Unknown,
                "invalid Pyth stream event",
            ));
        }
        if v["type"]
            .as_str()
            .is_some_and(|s| s.to_ascii_lowercase().contains("error"))
        {
            return Err(TransportError::new(
                TransportErrorKind::Protocol,
                RequestOutcome::Unknown,
                "Pyth subscription rejected",
            ));
        }
        Ok(())
    }
}
impl<T: WebSocketTransport> PythProClient<T>
where
    T::Stream: 'static,
{
    pub(crate) async fn subscribe_native(&self, request: SubscribeRequest) -> Result<JsonStream> {
        if request.price_feed_ids.as_ref().is_none_or(Vec::is_empty)
            && request.symbols.as_ref().is_none_or(Vec::is_empty)
        {
            return Err(Error::InvalidRequest(
                "at least one feed ID or symbol is required",
            ));
        }
        let request = serde_json::to_value(request)
            .map_err(|_| Error::InvalidRequest("invalid Pyth subscription"))?;
        self.open_websockets(&self.urls, Some(request)).await
    }

    pub(crate) async fn merkle_native(&self) -> Result<JsonStream> {
        let urls = self
            .urls
            .iter()
            .map(|endpoint| {
                let mut url = url::Url::parse(endpoint)
                    .map_err(|_| Error::InvalidRequest("invalid Pyth WebSocket endpoint"))?;
                url.set_path("/v1/merkle/root/stream");
                Ok(url.into())
            })
            .collect::<Result<Vec<String>>>()?;
        self.open_websockets(&urls, None).await
    }

    async fn open_websockets(&self, urls: &[String], request: Option<Value>) -> Result<JsonStream> {
        if !self.http.transport.supports_handshake_headers() {
            return Err(Error::InvalidRequest(
                "Pyth WebSocket transport must support handshake headers",
            ));
        }
        let token = self.token.clone().ok_or(Error::InvalidRequest(
            "Pyth API key is required for streaming",
        ))?;
        let scope = NEXT_SCOPE.fetch_add(1, Ordering::Relaxed);
        let results = futures_util::future::join_all(urls.iter().map(|url| {
            self.http.transport.subscribe(
                url,
                Spec {
                    request: request.clone(),
                    token: token.clone(),
                    scope,
                },
            )
        }))
        .await;
        let mut streams = Vec::new();
        let mut first = None;
        for result in results {
            match result {
                Ok(s) => streams.push(s),
                Err(e) => {
                    first.get_or_insert(e);
                }
            }
        }
        // Subscription setup must be complete; otherwise callers would believe all configured sources are active.
        if let Some(e) = first {
            use aux_rs_core::WebSocketStream;
            for stream in &mut streams {
                let _ = stream.close().await;
            }
            return Err(e.into());
        }
        Ok(JsonStream::websockets(streams, request.is_some()))
    }
}

// The Pyth binary router envelope carries length-prefixed format messages.
// Preserve all bytes, including new formats, and decode the parsed JSON entry.
fn decode_binary(bytes: &[u8]) -> Option<String> {
    let (magic, rest) = bytes.split_at_checked(4)?;
    if u32::from_le_bytes(magic.try_into().ok()?) != 461_928_307 {
        return None;
    }
    let (subscription, mut rest) = rest.split_at_checked(8)?;
    let subscription = u64::from_be_bytes(subscription.try_into().ok()?);
    let mut update = json!({"type":"streamUpdated","subscriptionId":subscription,"binary":STANDARD.encode(bytes)});
    if rest.is_empty() {
        return None;
    }
    while !rest.is_empty() {
        let (size, tail) = rest.split_at_checked(2)?;
        let size = u16::from_be_bytes(size.try_into().ok()?) as usize;
        let (message, tail) = tail.split_at_checked(size)?;
        let (magic, body) = message.split_at_checked(4)?;
        let name = match u32::from_le_bytes(magic.try_into().ok()?) {
            3_302_625_434 => {
                update["parsed"] = serde_json::from_slice(body).ok()?;
                rest = tail;
                continue;
            }
            2_593_727_018 => "evm",
            2_182_742_457 => "solana",
            1_296_547_300 => "leEcdsa",
            1_499_680_012 => "leUnsigned",
            _ => {
                rest = tail;
                continue;
            }
        };
        update[name] = json!({"encoding":"base64","data":STANDARD.encode(message)});
        rest = tail;
    }
    Some(update.to_string())
}
