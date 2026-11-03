use crate::*;
use aux_rs_core::{
    DefaultTransport, Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpResponse,
    HttpTransport, Unauthenticated,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
#[derive(Clone)]
pub struct Derive {
    http: String,
    ws: String,
    pub chain_id: u64,
    pub matching: String,
}
impl std::fmt::Debug for Derive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Derive")
            .field("chain_id", &self.chain_id)
            .finish_non_exhaustive()
    }
}
impl Derive {
    pub fn mainnet() -> Self {
        Self::custom(
            "https://api.derive.xyz/v3",
            "wss://api.derive.xyz/v3/ws",
            1,
            "0xeB8d770ec18DB98Db922E9D83260A585b9F0DeAD",
        )
        .expect("valid native configuration")
    }
    pub fn testnet() -> Self {
        Self::custom(
            "https://testnet.api.derive.xyz/v3",
            "wss://testnet.api.derive.xyz/v3/ws",
            11155111,
            "0xeB8d770ec18DB98Db922E9D83260A585b9F0DeAD",
        )
        .expect("valid native configuration")
    }
    pub fn custom(
        http: impl Into<String>,
        ws: impl Into<String>,
        chain_id: u64,
        matching: impl Into<String>,
    ) -> Result<Self, Error> {
        let matching = matching.into();
        address(&matching)?;
        Ok(Self {
            http: http.into().trim_end_matches('/').into(),
            ws: ws.into(),
            chain_id,
            matching,
        })
    }
}
impl Exchange for Derive {
    type Error = Error;
    fn http_url(&self) -> &str {
        &self.http
    }
    fn ws_url(&self) -> &str {
        &self.ws
    }
}
/// Private methods are only available after attaching native authentication.
/// ```compile_fail
/// use aux_rs_derive::{DeriveClient, CreateOrderRequest};
/// fn unsigned<T:aux_rs_core::HttpTransport>(c:DeriveClient<T>) { let _=c.private_order(CreateOrderRequest::default()); }
/// ```
#[derive(Clone)]
pub struct DeriveClient<T = DefaultTransport, A = Unauthenticated> {
    pub(crate) exchange: Derive,
    pub(crate) transport: T,
    pub(crate) auth: A,
}
impl<T, A> std::fmt::Debug for DeriveClient<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeriveClient").finish_non_exhaustive()
    }
}
impl<T> DeriveClient<T> {
    pub fn with_transport(exchange: Derive, transport: T) -> Self {
        Self {
            exchange,
            transport,
            auth: Unauthenticated,
        }
    }
}
impl<T, A> DeriveClient<T, A> {
    pub fn exchange(&self) -> &Derive {
        &self.exchange
    }
    pub fn transport(&self) -> &T {
        &self.transport
    }
    pub fn auth(&self) -> &A {
        &self.auth
    }
    pub fn with_auth<B>(self, auth: B) -> DeriveClient<T, B> {
        DeriveClient {
            exchange: self.exchange,
            transport: self.transport,
            auth,
        }
    }
}
#[cfg(any(feature = "http", feature = "ws"))]
impl DeriveClient<aux_rs_core::StandardTransport> {
    pub fn new(exchange: Derive) -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(exchange, aux_rs_core::TransportConfig::default())
    }
    pub fn mainnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Derive::mainnet())
    }
    pub fn testnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Derive::testnet())
    }
    pub fn with_config(
        exchange: Derive,
        config: aux_rs_core::TransportConfig,
    ) -> Result<Self, aux_rs_core::TransportError> {
        Ok(Self::with_transport(
            exchange,
            aux_rs_core::StandardTransport::new(config)?,
        ))
    }
}
pub(crate) fn decode<R: DeserializeOwned>(response: HttpResponse) -> Result<R, Error> {
    let value: Value = match serde_json::from_slice(&response.body) {
        Ok(v) => v,
        Err(e) if (200..300).contains(&response.status) => return Err(Error::Decode(e)),
        Err(_) => Value::Null,
    };
    if !(200..300).contains(&response.status) || value.get("error").is_some_and(|e| !e.is_null()) {
        return Err(Error::Api {
            status: Some(response.status),
            body: value,
        });
    }
    let result = value
        .get("result")
        .cloned()
        .ok_or(Error::InvalidRequest("missing JSON-RPC result"))?;
    serde_json::from_value(result).map_err(Error::Decode)
}
fn safe_read(path: &str) -> bool {
    path.contains("/get_")
        || path.ends_with("_debug")
        || path.ends_with("/decode_action")
        || path.ends_with("/session_keys")
        || path.ends_with("/getRateLimits")
        || path.ends_with("/margin_watch")
        || path.ends_with("/poll_quotes")
        || path.ends_with("/poll_rfqs")
}
impl<T: HttpTransport, A: Sync> DeriveClient<T, A> {
    pub(crate) async fn execute_public<Q: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        request: Q,
    ) -> Result<R, ClientError<Error>> {
        self.transport.ensure_open()?;
        let body = serde_json::to_vec(&request)
            .map_err(Error::from)
            .map_err(ClientError::Exchange)?;
        let wire = HttpRequest {
            method: HttpMethod::Post,
            url: format!("{}{path}", self.exchange.http_url()),
            headers: vec![("Content-Type".into(), "application/json".into())],
            body,
        };
        let response = if safe_read(path) {
            self.transport.execute_read(wire).await?
        } else {
            self.transport.execute(wire).await?
        };
        decode(response).map_err(ClientError::Exchange)
    }
}
impl<T: HttpTransport, A: Authentication> DeriveClient<T, A> {
    pub(crate) async fn execute_authenticated<Q: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        request: Q,
    ) -> Result<R, ClientError<Error>> {
        self.transport.ensure_open()?;
        let mut body = serde_json::to_value(request)
            .map_err(Error::from)
            .map_err(ClientError::Exchange)?;
        self.prepare_action(path, &mut body).await?;
        let login = self
            .auth
            .login(&self.exchange)
            .await
            .map_err(ClientError::Exchange)?;
        let wire = HttpRequest {
            method: HttpMethod::Post,
            url: format!("{}{path}", self.exchange.http_url()),
            headers: vec![
                ("Content-Type".into(), "application/json".into()),
                (
                    "X-DeriveWallet".into(),
                    login["wallet"].as_str().unwrap_or_default().into(),
                ),
                (
                    "X-DeriveTimestamp".into(),
                    login["timestamp"].as_str().unwrap_or_default().into(),
                ),
                (
                    "X-DeriveSignature".into(),
                    login["signature"].as_str().unwrap_or_default().into(),
                ),
            ],
            body: serde_json::to_vec(&body)
                .map_err(Error::from)
                .map_err(ClientError::Exchange)?,
        };
        // Login proofs are fresh per request; never replay authenticated writes.
        decode(self.transport.execute(wire).await?).map_err(ClientError::Exchange)
    }
    pub(crate) async fn prepare_action(
        &self,
        path: &str,
        body: &mut Value,
    ) -> Result<(), ClientError<Error>> {
        crate::preparation::prepare_native(&mut &*self, &self.exchange, &self.auth, path, body)
            .await
    }
}
