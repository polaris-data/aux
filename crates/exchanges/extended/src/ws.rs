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
        if value.get("error").is_some_and(|e| !e.is_null()) || value["type"] == "ERROR" {
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
pub struct Subscription {
    pub channel: String,
    #[serde(default)]
    pub market: Option<String>,
    #[serde(default)]
    pub params: Value,
    /// Use Extended's v2 JSON-RPC subscription transport.
    #[serde(default)]
    pub rpc: bool,
}
impl Subscription {
    pub fn url(&self, exchange: &Extended) -> Result<String, Error> {
        if !self.params.is_null() && !self.params.is_object() {
            return Err(Error::InvalidRequest("stream params must be an object"));
        }
        if self.channel == "candles" && self.market.as_deref().is_none_or(str::is_empty) {
            return Err(Error::InvalidRequest("candle market required"));
        }
        let root = aux_rs_core::Exchange::ws_url(exchange);
        if self.rpc {
            return Ok(format!("{}/v2/rpc", root.trim_end_matches("/v1")));
        }
        let channel = match self.channel.as_str() {
            "orderbooks" => "orderbooks",
            "rfq_orderbooks" => "orderbooks/rfq",
            "trades" => "publicTrades",
            "funding" => "funding",
            "candles" => "candles",
            "mark_price" => "prices/mark",
            "index_price" => "prices/index",
            "account" => "account",
            _ => return Err(Error::InvalidRequest("unknown Extended channel")),
        };
        let mut url = format!("{root}/{channel}");
        if let Some(m) = &self.market {
            if m.is_empty() || m == "." || m == ".." || m.contains(['/', '\\']) {
                return Err(Error::InvalidRequest("invalid market"));
            }
            url.push('/');
            url.push_str(&aux_rs_core::url_component(m));
        }
        if self.channel == "candles" {
            let kind = self.params["candle_type"]
                .as_str()
                .ok_or(Error::InvalidRequest("candle_type required"))?;
            if !matches!(kind, "trades" | "mark-prices" | "index-prices") {
                return Err(Error::InvalidRequest("invalid candle type"));
            }
            url.push('/');
            url.push_str(kind);
        }
        let mut query = url::form_urlencoded::Serializer::new(String::new());
        if let Some(p) = self.params.as_object() {
            for (k, v) in p {
                if k == "candle_type" {
                    continue;
                }
                let text = match v {
                    Value::String(s) => s.clone(),
                    Value::Number(_) | Value::Bool(_) => v.to_string(),
                    _ => return Err(Error::InvalidRequest("scalar stream parameter required")),
                };
                query.append_pair(k, &text);
            }
        }
        let query = query.finish();
        if !query.is_empty() {
            url.push('?');
            url.push_str(&query)
        }
        Ok(url)
    }
    fn rpc_topic(&self) -> Result<String, Error> {
        let p = self.rpc_params()?;
        let selector = &p["selector"];
        let market = selector["market"].as_str().unwrap_or("all");
        Ok(match self.channel.as_str() {
            "orderbooks" | "rfq_orderbooks" => {
                let depth = if selector["depth"] == "1" { "1." } else { "" };
                let rfq = if self.channel == "rfq_orderbooks" {
                    ".rfq"
                } else {
                    ""
                };
                format!("orderbooks.{depth}{market}{rfq}")
            }
            "trades" => format!("trades.{market}"),
            "funding" => format!("funding-rates.{market}"),
            "mark_price" => format!("prices.mark.{market}"),
            "index_price" => format!("prices.index.{market}"),
            "candles" => format!(
                "candles.{}.{}.{}",
                selector["type"].as_str().unwrap_or("last"),
                market,
                selector["interval"].as_str().unwrap_or("")
            ),
            "account" => format!(
                "account.{}",
                selector["account"]
                    .as_str()
                    .ok_or(Error::InvalidRequest("RPC account selector required"))?
            ),
            _ => return Err(Error::InvalidRequest("unknown Extended RPC channel")),
        })
    }
    pub fn rpc_params(&self) -> Result<Value, Error> {
        let mut params = self.params.as_object().cloned().unwrap_or_default();
        let scope = match self.channel.as_str() {
            "orderbooks" | "rfq_orderbooks" => "orderbooks",
            "trades" => "trades",
            "funding" => "funding-rates",
            "candles" => "candles",
            "mark_price" | "index_price" => "prices",
            "account" => "account",
            _ => return Err(Error::InvalidRequest("unknown Extended RPC channel")),
        };
        let mut selector = params.remove("selector").unwrap_or_else(|| json!({}));
        if !selector.is_object() {
            return Err(Error::InvalidRequest("selector must be an object"));
        }
        if let Some(m) = &self.market {
            selector["market"] = json!(m)
        }
        if matches!(self.channel.as_str(), "orderbooks" | "rfq_orderbooks") {
            if selector.get("depth").is_none() {
                selector["depth"] = json!("full")
            }
            selector["rfqOnly"] = json!(self.channel == "rfq_orderbooks")
        }
        if self.channel == "mark_price" {
            selector["type"] = json!("mark")
        }
        if self.channel == "index_price" {
            selector["type"] = json!("index")
        }
        params.insert("scope".into(), json!(scope));
        params.insert("selector".into(), selector);
        Ok(Value::Object(params))
    }
}
pub fn decode_event(text: &str) -> Result<Value, Error> {
    Ok(serde_json::from_str(text)?)
}
struct Spec<A> {
    id: u64,
    subscription: Subscription,
    auth: A,
    sequence: Mutex<Option<u64>>,
}
impl<A: StreamAuthentication> WebSocketSubscription for Spec<A> {
    fn key(&self) -> String {
        self.id.to_string()
    }
    fn connection_scope(&self) -> String {
        format!("extended:{}:{}", self.auth.stream_identity(), self.id)
    }
    fn handshake_headers(&self) -> HandshakeHeaders<'_> {
        Box::pin(async move {
            *self.sequence.lock().unwrap_or_else(|e| e.into_inner()) = None;
            let mut h = self
                .auth
                .stream_headers(self.subscription.channel == "account")
                .map_err(|_| {
                    TransportError::new(
                        TransportErrorKind::Authentication,
                        RequestOutcome::NotSent,
                        "missing or expired stream credentials",
                    )
                })?;
            h.push(("User-Agent".into(), "aux-rs/0.0.0".into()));
            Ok(h)
        })
    }
    fn subscribe_message(&self) -> String {
        if !self.subscription.rpc {
            return String::new();
        }
        let mut p = self
            .subscription
            .rpc_params()
            .expect("validated subscription");
        if self.subscription.channel == "account"
            && let Ok(h) = self.auth.stream_headers(true)
            && let Some((_, key)) = h.into_iter().find(|(k, _)| k == "X-Api-Key")
        {
            p["apiKey"] = json!(key)
        }
        json!({"jsonrpc":"2.0","id":self.id.to_string(),"method":"subscribe","params":p})
            .to_string()
    }
    fn unsubscribe_message(&self) -> String {
        if self.subscription.rpc {
            let mut p = self.subscription.rpc_params().unwrap_or(Value::Null);
            if self.subscription.channel == "account"
                && let Ok(h) = self.auth.stream_headers(true)
                && let Some((_, key)) = h.into_iter().find(|(k, _)| k == "X-Api-Key")
            {
                p["apiKey"] = json!(key);
            }
            json!({"jsonrpc":"2.0","id":format!("close-{}",self.id),"method":"unsubscribe","params":p}).to_string()
        } else {
            String::new()
        }
    }
    fn requires_fresh_snapshot(&self) -> bool {
        matches!(
            self.subscription.channel.as_str(),
            "orderbooks" | "rfq_orderbooks"
        )
    }
    fn is_initial_snapshot(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            v["type"] == "SNAPSHOT"
                || v["type"] == "ORDERBOOK.SNAPSHOT"
                || v["type"] == "ORDERBOOKS.SNAPSHOT"
                || v["data"]["t"] == "SNAPSHOT"
                || v["data"]["isSnapshot"] == true
        })
    }
    fn is_snapshot_control(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| {
            v.get("id").is_some()
                || v.get("error").is_some_and(|e| !e.is_null())
                || v["type"] == "ERROR"
        })
    }
    fn is_data_message(&self, text: &str) -> bool {
        serde_json::from_str::<Value>(text).is_ok_and(|v| v.get("data").is_some())
    }
    fn matches(&self, text: &str) -> bool {
        if !self.subscription.rpc {
            return true;
        }
        let Ok(value) = serde_json::from_str::<Value>(text) else {
            return true;
        };
        if let Some(id) = value.get("id") {
            let id = id
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| id.to_string());
            return id == self.id.to_string()
                || id == format!("ping-{}", self.id)
                || id == format!("close-{}", self.id);
        }
        if let Some(topic) = value["subscription"].as_str() {
            return self
                .subscription
                .rpc_topic()
                .is_ok_and(|expected| expected == topic);
        }
        value.get("error").is_some_and(|e| !e.is_null()) || value["type"] == "ERROR"
    }
    fn heartbeat_message(&self) -> Option<String> {
        self.subscription.rpc.then(|| {
            json!({"jsonrpc":"2.0","id":format!("ping-{}",self.id),"method":"ping"}).to_string()
        })
    }
    fn validate_message(&self, text: &str) -> Result<(), TransportError> {
        let v: Value =
            serde_json::from_str(text).map_err(|_| protocol("malformed Extended frame"))?;
        if v.get("error").is_some_and(|e| !e.is_null()) {
            return Ok(());
        }
        if let Some(seq) = v["seq"].as_u64() {
            let mut prev = self.sequence.lock().unwrap_or_else(|e| e.into_inner());
            if prev.is_some_and(|p| p.checked_add(1) != Some(seq)) {
                return Err(gap());
            }
            *prev = Some(seq);
        }
        Ok(())
    }
}
impl<T: WebSocketTransport, A: StreamAuthentication> ExtendedClient<T, A> {
    pub async fn subscribe(
        &self,
        subscription: Subscription,
    ) -> Result<SubscriptionHandle<T::Stream>, aux_rs_core::Error<Error>> {
        if subscription.rpc {
            subscription
                .rpc_topic()
                .map_err(aux_rs_core::Error::Exchange)?;
        }
        let url = subscription
            .url(&self.exchange)
            .map_err(aux_rs_core::Error::Exchange)?;
        if subscription.rpc {
            subscription
                .rpc_params()
                .map_err(aux_rs_core::Error::Exchange)?;
        }
        let headers = self
            .auth
            .stream_headers(subscription.channel == "account")
            .map_err(aux_rs_core::Error::Exchange)?;
        if subscription.channel == "account" && !headers.iter().any(|(k, _)| k == "X-Api-Key") {
            return Err(aux_rs_core::Error::Exchange(Error::Authentication(
                "account streams require an API key",
            )));
        }
        if !self.transport.supports_handshake_headers() {
            return Err(aux_rs_core::Error::Exchange(Error::InvalidRequest(
                "Extended requires User-Agent handshake-header support",
            )));
        }
        let spec = Spec {
            id: NEXT.fetch_add(1, Ordering::Relaxed),
            subscription,
            auth: self.auth.clone(),
            sequence: Mutex::new(None),
        };
        Ok(SubscriptionHandle {
            inner: self.transport.subscribe(&url, spec).await?,
        })
    }
}
