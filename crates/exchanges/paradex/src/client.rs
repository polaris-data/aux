use crate::*;
use aux_rs_core::{
    DefaultTransport, Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpResponse,
    HttpTransport, Unauthenticated,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
#[derive(Clone)]
pub struct Paradex {
    http: String,
    ws: String,
    pub chain_id: String,
    pub evm_chain_id: u64,
    pub siwe_domain: String,
    pub siwe_uri: String,
}
impl std::fmt::Debug for Paradex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Paradex")
            .field("chain_id", &self.chain_id)
            .finish_non_exhaustive()
    }
}
impl Paradex {
    pub fn mainnet() -> Self {
        Self::custom(
            "https://api.prod.paradex.trade",
            "wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=3",
            "PRIVATE_SN_PARACLEAR_MAINNET",
            1,
            "app.paradex.trade",
            "https://app.paradex.trade",
        )
    }
    pub fn testnet() -> Self {
        Self::custom(
            "https://api.testnet.paradex.trade",
            "wss://ws.api.testnet.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=3",
            "PRIVATE_SN_PARACLEAR_TESTNET",
            11155111,
            "app.testnet.paradex.trade",
            "https://app.testnet.paradex.trade",
        )
    }
    /// HTTP URL is the origin, without /v1. v2 routes use the same origin.
    pub fn custom(
        http: impl Into<String>,
        ws: impl Into<String>,
        chain_id: impl Into<String>,
        evm_chain_id: u64,
        siwe_domain: impl Into<String>,
        siwe_uri: impl Into<String>,
    ) -> Self {
        let mut ws = ws.into();
        if let Ok(mut url) = url::Url::parse(&ws) {
            let params: Vec<_> = url
                .query_pairs()
                .filter(|(k, _)| k != "sbeSchemaId" && k != "sbeSchemaVersion")
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect();
            url.set_query(None);
            url.query_pairs_mut()
                .extend_pairs(params)
                .append_pair("sbeSchemaId", "1")
                .append_pair("sbeSchemaVersion", "3");
            ws = url.to_string();
        }
        Self {
            http: http.into().trim_end_matches('/').into(),
            ws,
            chain_id: chain_id.into(),
            evm_chain_id,
            siwe_domain: siwe_domain.into(),
            siwe_uri: siwe_uri.into(),
        }
    }
    pub fn rpc_url(&self) -> Result<String, Error> {
        let mut url = url::Url::parse(&self.ws)
            .map_err(|_| Error::InvalidRequest("invalid WebSocket URL"))?;
        let params: Vec<_> = url
            .query_pairs()
            .filter(|(k, _)| k != "sbeSchemaId" && k != "sbeSchemaVersion")
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        url.set_query(None);
        if !params.is_empty() {
            url.query_pairs_mut().extend_pairs(params);
        }
        Ok(url.to_string())
    }
}
impl Exchange for Paradex {
    type Error = Error;
    fn http_url(&self) -> &str {
        &self.http
    }
    fn ws_url(&self) -> &str {
        &self.ws
    }
}
#[derive(Clone, Copy)]
pub enum Location {
    Path,
    Query,
    Header,
    Body,
}
#[derive(Clone, Copy)]
pub struct Parameter {
    pub name: &'static str,
    pub location: Location,
    pub required: bool,
    pub collection: &'static str,
}
pub struct Operation {
    pub method: HttpMethod,
    pub path: &'static str,
    pub authenticated: bool,
    pub safe_read: bool,
    pub parameters: &'static [Parameter],
}
impl Operation {
    pub fn request<Q: Serialize>(&self, e: &Paradex, request: Q) -> Result<HttpRequest, Error> {
        let mut fields = serde_json::to_value(request)?;
        let fields = fields
            .as_object_mut()
            .ok_or(Error::InvalidRequest("expected request object"))?;
        let mut path = self.path.to_owned();
        let mut query = Vec::new();
        let mut headers = vec![("Content-Type".into(), "application/json".into())];
        let mut body = Value::Null;
        for p in self.parameters {
            let Some(value) = fields.remove(p.name).filter(|v| !v.is_null()) else {
                if p.required {
                    return Err(Error::InvalidRequest("missing required request parameter"));
                }
                continue;
            };
            match p.location {
                Location::Body => body = value,
                Location::Header => {
                    let s = scalar(&value)?;
                    if s.contains(['\r', '\n']) {
                        return Err(Error::InvalidRequest("invalid header value"));
                    }
                    headers.push((p.name.into(), s));
                }
                Location::Path => {
                    let s = scalar(&value)?;
                    if s.is_empty() {
                        return Err(Error::InvalidRequest("empty path parameter"));
                    }
                    path =
                        path.replace(&format!("{{{}}}", p.name), &aux_rs_core::url_component(&s));
                }
                Location::Query => {
                    if let Some(xs) = value.as_array() {
                        let xs = xs.iter().map(scalar).collect::<Result<Vec<_>, _>>()?;
                        if p.collection == "multi" {
                            query.extend(xs.into_iter().map(|v| (p.name.to_owned(), v)));
                        } else {
                            let separator = match p.collection {
                                "ssv" => " ",
                                "tsv" => "\t",
                                "pipes" => "|",
                                _ => ",",
                            };
                            query.push((p.name.to_owned(), xs.join(separator)));
                        }
                    } else {
                        query.push((p.name.into(), scalar(&value)?));
                    }
                }
            }
        }
        // Extra caller fields are native body additions, not silently discarded.
        if !fields.is_empty() {
            if let Value::Object(ref mut b) = body {
                b.extend(fields.clone());
            } else {
                return Err(Error::InvalidRequest(
                    "unknown top-level parameter; body fields belong under body",
                ));
            }
        }
        let prefix = if path.starts_with("/v2/") {
            String::new()
        } else {
            "/v1".into()
        };
        let mut url = format!("{}{prefix}{path}", e.http_url());
        if !query.is_empty() {
            url.push('?');
            url.push_str(
                &query
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "{}={}",
                            aux_rs_core::url_component(k),
                            aux_rs_core::url_component(v)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("&"),
            );
        }
        Ok(HttpRequest {
            method: self.method,
            url,
            headers,
            body: if body.is_null() {
                vec![]
            } else {
                serde_json::to_vec(&body)?
            },
        })
    }
}
fn scalar(v: &Value) -> Result<String, Error> {
    match v {
        Value::String(s) => Ok(s.clone()),
        Value::Number(n) => Ok(n.to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        _ => Err(Error::InvalidRequest("expected scalar request parameter")),
    }
}
/// Native operations remain unavailable on an unauthenticated Rust client.
/// ```compile_fail
/// use aux_rs_paradex::{ParadexClient, OrdersNewParams};
/// fn unsigned<T:aux_rs_core::HttpTransport>(c:ParadexClient<T>) { let _=c.orders_new(OrdersNewParams::default()); }
/// ```
#[derive(Clone)]
pub struct ParadexClient<T = DefaultTransport, A = Unauthenticated> {
    pub(crate) exchange: Paradex,
    pub(crate) transport: T,
    pub(crate) auth: A,
}
impl<T, A> std::fmt::Debug for ParadexClient<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParadexClient").finish_non_exhaustive()
    }
}
impl<T> ParadexClient<T> {
    pub fn with_transport(exchange: Paradex, transport: T) -> Self {
        Self {
            exchange,
            transport,
            auth: Unauthenticated,
        }
    }
}
impl<T, A> ParadexClient<T, A> {
    pub fn exchange(&self) -> &Paradex {
        &self.exchange
    }
    pub fn transport(&self) -> &T {
        &self.transport
    }
    pub fn auth(&self) -> &A {
        &self.auth
    }
    pub fn with_auth<B>(self, auth: B) -> ParadexClient<T, B> {
        ParadexClient {
            exchange: self.exchange,
            transport: self.transport,
            auth,
        }
    }
}
#[cfg(any(feature = "http", feature = "ws"))]
impl ParadexClient<aux_rs_core::StandardTransport> {
    pub fn new(exchange: Paradex) -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(exchange, aux_rs_core::TransportConfig::default())
    }
    pub fn mainnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Paradex::mainnet())
    }
    pub fn testnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Paradex::testnet())
    }
    pub fn with_config(
        exchange: Paradex,
        config: aux_rs_core::TransportConfig,
    ) -> Result<Self, aux_rs_core::TransportError> {
        Ok(Self::with_transport(
            exchange,
            aux_rs_core::StandardTransport::new(config)?,
        ))
    }
}
pub(crate) fn decode<R: DeserializeOwned>(response: HttpResponse) -> Result<R, Error> {
    let value: Value = if response.body.is_empty() && (200..300).contains(&response.status) {
        Value::Null
    } else {
        match serde_json::from_slice(&response.body) {
            Ok(v) => v,
            Err(e) if (200..300).contains(&response.status) => return Err(Error::Decode(e)),
            Err(_) => Value::Null,
        }
    };
    if !(200..300).contains(&response.status) || value.get("error").is_some_and(|e| !e.is_null()) {
        return Err(Error::Api {
            status: Some(response.status),
            body: value,
        });
    }
    serde_json::from_value(value).map_err(Error::Decode)
}
impl<T: HttpTransport, A: Sync> ParadexClient<T, A> {
    pub(crate) async fn execute_public<Q: Serialize, R: DeserializeOwned>(
        &self,
        operation: &Operation,
        request: Q,
    ) -> Result<R, ClientError<Error>> {
        self.transport.ensure_open()?;
        let wire = operation
            .request(&self.exchange, request)
            .map_err(ClientError::Exchange)?;
        let response = if operation.safe_read {
            self.transport.execute_read(wire).await?
        } else {
            self.transport.execute(wire).await?
        };
        decode(response).map_err(ClientError::Exchange)
    }
}
impl<T: HttpTransport, A: Authentication> ParadexClient<T, A> {
    pub(crate) async fn execute_authenticated<Q: Serialize, R: DeserializeOwned>(
        &self,
        operation: &Operation,
        request: Q,
    ) -> Result<R, ClientError<Error>> {
        self.transport.ensure_open()?;
        let mut wire = operation
            .request(&self.exchange, request)
            .map_err(ClientError::Exchange)?;
        self.auth
            .prepare(&self.exchange, operation, &mut wire)
            .await
            .map_err(ClientError::Exchange)?;
        let response = if operation.safe_read {
            self.transport.execute_read(wire).await?
        } else {
            self.transport.execute(wire).await?
        };
        decode(response).map_err(ClientError::Exchange)
    }
}
