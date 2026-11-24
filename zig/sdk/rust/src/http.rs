//! Native exchange HTTP operations and caller-supplied transport support.
use crate::{Error, Result, native::invoke};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::sync::Arc;
#[cfg(feature = "rustls")]
pub use tokio_rustls::rustls;

#[derive(Clone)]
pub(crate) enum Policy {
    Application(Arc<dyn Transport>),
    #[cfg(feature = "rustls")]
    Tls(Arc<dyn Transport>),
}
impl Policy {
    pub(crate) fn connection(&self) -> &Arc<dyn Transport> {
        match self {
            Self::Application(t) => t,
            #[cfg(feature = "rustls")]
            Self::Tls(t) => t,
        }
    }
    pub(crate) fn execute(&self, args: Value) -> Result<Value> {
        match self {
            Self::Application(t) => with_transport(t.as_ref(), args),
            #[cfg(feature = "rustls")]
            Self::Tls(t) => crate::native::execute_with_transport(&args, t),
        }
    }
    #[cfg(feature = "rustls")]
    pub(crate) fn tls(config: rustls::ClientConfig) -> Self {
        Self::Tls(Arc::new(crate::tls::RustlsPolicy::new(config)))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: BTreeMap<String, String>,
    pub body: Value,
    /// The caller must sign this request before dispatch.
    pub authenticated: bool,
}

pub(crate) fn call<T: serde::de::DeserializeOwned>(id: u32, args: Value) -> Result<T> {
    let result = invoke(id, args.to_string().as_bytes())?;
    serde_json::from_str(&result).map_err(|_| Error::Decode("invalid native response"))
}

pub fn prepare(venue: &str, operation: &str, params: &Value) -> Result<Request> {
    call(
        10,
        json!({ "venue": venue, "operation": operation, "params": params }),
    )
}

/// Decode a wire response from a caller-supplied transport.
pub fn decode(venue: &str, status: u16, body: &str) -> Result<Value> {
    call(
        11,
        json!({ "venue": venue, "status": status, "body": body }),
    )
}

/// Perform one synchronous public operation. An optional origin selects a custom server.
pub fn execute(
    venue: &str,
    operation: &str,
    params: &Value,
    origin: Option<&str>,
) -> Result<Value> {
    execute_with_key(venue, operation, params, origin, None)
}

/// Supply the venue's API key for operations using header authentication.
pub fn execute_with_key(
    venue: &str,
    operation: &str,
    params: &Value,
    origin: Option<&str>,
    api_key: Option<&str>,
) -> Result<Value> {
    execute_with_credentials(venue, operation, params, origin, api_key, None)
}

pub fn execute_with_credentials(
    venue: &str,
    operation: &str,
    params: &Value,
    origin: Option<&str>,
    api_key: Option<&str>,
    api_secret: Option<&str>,
) -> Result<Value> {
    call(
        12,
        config(venue, operation, params, origin, api_key, api_secret),
    )
}

/// An outbound unary request after native encoding and authentication.
/// Header values may contain credentials. Body strings are the exact wire bytes.
#[derive(Deserialize)]
pub struct OutboundRequest {
    pub method: String,
    pub url: String,
    pub headers: BTreeMap<String, String>,
    pub body: Option<String>,
}

/// The transport supplies the decompressed UTF-8 body and received header pairs.
pub struct TransportResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

/// Implement this with your own HTTP client when the application owns TLS policy.
pub trait Transport: Send + Sync {
    fn send(&self, request: OutboundRequest) -> Result<TransportResponse>;

    /// Supply a connected blocking socket for Zig to frame HTTP/1.1 or WebSocket data.
    /// For TLS, authenticate the named host and apply the application client identity,
    /// then forward decrypted data over a socket pair. Ownership of the returned end
    /// passes to the SDK. Its shutdown must release any forwarding resources.
    fn connect_stream(&self, _host: &str, _port: u16, _tls: bool) -> Result<std::os::fd::OwnedFd> {
        Err(Error::InvalidRequest(
            "streaming requires a streaming transport",
        ))
    }
}

impl<F> Transport for F
where
    F: Fn(OutboundRequest) -> Result<TransportResponse> + Send + Sync,
{
    fn send(&self, request: OutboundRequest) -> Result<TransportResponse> {
        self(request)
    }
}

pub(crate) fn with_transport(transport: &dyn Transport, mut args: Value) -> Result<Value> {
    if args["venue"] == "derive" {
        let jobs: BTreeMap<String, Value> = call(28, args.clone())?;
        let mut metadata = serde_json::Map::new();
        for (key, job) in jobs {
            metadata.insert(key, send_prepared(transport, job)?);
        }
        args["metadata"] = Value::Object(metadata);
    }
    send_prepared(transport, args)
}
fn send_prepared(transport: &dyn Transport, args: Value) -> Result<Value> {
    let venue = args["venue"].clone();
    let key = args.get("api_key").cloned();
    let operation = args["operation"].clone();
    let session = if venue == "monaco" && operation == "refresh_session" {
        args.get("auth").cloned()
    } else {
        None
    };
    let is_pascal_write = venue == "pascal"
        && args["operation"]
            .as_str()
            .is_some_and(|op| op.starts_with("write."));
    let (request, mut reservation) = if is_pascal_write {
        (
            call(20, args.clone())?,
            call::<Option<Value>>(33, args)?.map(WalletReservation),
        )
    } else {
        (call(20, args)?, None)
    };
    // Once a caller's transport accepts an order, a lost response leaves its outcome unknown.
    let monaco_write = venue == "monaco"
        && matches!(
            operation.as_str(),
            Some(
                "place_order"
                    | "replace_order"
                    | "cancel_order"
                    | "batch_create"
                    | "batch_replace"
                    | "batch_cancel"
                    | "cancel_all"
            )
        );
    let response = transport.send(request).map_err(|error| match error {
        Error::Network if monaco_write => Error::PendingOutcome,
        other => other,
    })?;
    let rejected = (400..500).contains(&response.status) && response.status != 408;
    let mut input = json!({"venue":venue,"operation":operation,"status":response.status,"headers":response.headers,"body":response.body});
    if let Some(auth) = session {
        input["auth"] = auth;
    }
    if let Some(key) = key {
        input["api_key"] = key;
    }
    let result = call(21, input);
    if let Some(r) = reservation.as_mut() {
        r.0["confirmed"] = (rejected || result.is_ok()).into();
    }
    result
}

// Drop also releases a reservation if an application-provided transport unwinds.
struct WalletReservation(Value);
impl Drop for WalletReservation {
    fn drop(&mut self) {
        if self.0.get("confirmed").is_none() {
            self.0["confirmed"] = false.into();
        }
        let _ = call::<Value>(34, self.0.take());
    }
}

pub(crate) fn config(
    venue: &str,
    operation: &str,
    params: &Value,
    origin: Option<&str>,
    api_key: Option<&str>,
    api_secret: Option<&str>,
) -> Value {
    let mut args = json!({ "venue": venue, "operation": operation, "params": params });
    if let Some(secret) = api_secret {
        args["api_secret"] = json!(secret);
    }
    if let Some(key) = api_key {
        args["api_key"] = json!(key);
    }
    if let Some(origin) = origin {
        args["origin"] = json!(origin);
    }
    args
}
