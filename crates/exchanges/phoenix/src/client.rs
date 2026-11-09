use crate::*;
use aux_rs_core::{
    DefaultTransport, Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpResponse,
    HttpTransport, Unauthenticated,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
#[derive(Clone)]
pub struct Phoenix {
    http: String,
    ws: String,
    pub network: String,
    pub chain_id: String,
}
impl std::fmt::Debug for Phoenix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Phoenix")
            .field("network", &self.network)
            .finish_non_exhaustive()
    }
}
impl Phoenix {
    pub fn mainnet() -> Self {
        Self {
            http: "https://perp-api.phoenix.trade".into(),
            ws: "wss://perp-api.phoenix.trade/v1/ws".into(),
            network: "mainnet".into(),
            chain_id: "mainnet-beta".into(),
        }
    }

    pub fn custom(
        http: impl Into<String>,
        ws: impl Into<String>,
        chain_id: impl Into<String>,
    ) -> Result<Self, Error> {
        let http = http.into();
        let ws = ws.into();
        validate_url(&http, false)?;
        validate_url(&ws, true)?;
        Ok(Self {
            http: http.trim_end_matches('/').into(),
            ws,
            network: "custom".into(),
            chain_id: chain_id.into(),
        })
    }
}
impl Exchange for Phoenix {
    type Error = Error;
    fn http_url(&self) -> &str {
        &self.http
    }
    fn ws_url(&self) -> &str {
        &self.ws
    }
}
pub fn validate_url(value: &str, websocket: bool) -> Result<(), Error> {
    let u = url::Url::parse(value).map_err(|_| Error::InvalidRequest("invalid endpoint"))?;
    if u.host_str().is_none()
        || !u.username().is_empty()
        || u.password().is_some()
        || u.query().is_some()
        || u.fragment().is_some()
        || !if websocket {
            matches!(u.scheme(), "ws" | "wss")
        } else {
            matches!(u.scheme(), "http" | "https")
        }
    {
        return Err(Error::InvalidRequest(
            "invalid endpoint scheme, credentials, query or fragment",
        ));
    }
    Ok(())
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
    pub fn request<Q: Serialize>(&self, e: &Phoenix, request: Q) -> Result<HttpRequest, Error> {
        let mut fields = serde_json::to_value(request)?;
        let fields = fields
            .as_object_mut()
            .ok_or(Error::InvalidRequest("expected request object"))?;
        let mut path = self.path.to_owned();
        let mut query = Vec::new();
        let mut headers = vec![
            ("Content-Type".into(), "application/json".into()),
            ("User-Agent".into(), "aux-rs/0.0.0".into()),
        ];
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
                    if s.is_empty() || s == "." || s == ".." || s.contains(['/', '\\']) {
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
        let mut url = format!("{}{path}", e.http_url());
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
/// Native authenticated operations require an authenticated client state.
/// ```compile_fail
/// use aux_rs_phoenix::PhoenixClient;
/// fn unsigned<T:aux_rs_core::HttpTransport>(client:PhoenixClient<T>){let _=client.log_out(Default::default());}
/// ```
#[derive(Clone)]
pub struct PhoenixClient<T = DefaultTransport, A = Unauthenticated> {
    pub(crate) exchange: Phoenix,
    pub(crate) transport: T,
    pub(crate) auth: A,
}
impl<T, A> std::fmt::Debug for PhoenixClient<T, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhoenixClient").finish_non_exhaustive()
    }
}
impl<T> PhoenixClient<T> {
    pub fn with_transport(exchange: Phoenix, transport: T) -> Self {
        Self {
            exchange,
            transport,
            auth: Unauthenticated,
        }
    }
}
impl<T, A> PhoenixClient<T, A> {
    pub fn exchange(&self) -> &Phoenix {
        &self.exchange
    }
    pub fn transport(&self) -> &T {
        &self.transport
    }
    pub fn auth(&self) -> &A {
        &self.auth
    }
    pub fn with_auth<B>(self, auth: B) -> PhoenixClient<T, B> {
        PhoenixClient {
            exchange: self.exchange,
            transport: self.transport,
            auth,
        }
    }
}
#[cfg(any(feature = "http", feature = "ws"))]
impl PhoenixClient<aux_rs_core::StandardTransport> {
    pub fn new(exchange: Phoenix) -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(exchange, aux_rs_core::TransportConfig::default())
    }
    pub fn mainnet() -> Result<Self, aux_rs_core::TransportError> {
        Self::new(Phoenix::mainnet())
    }

    pub fn with_config(
        exchange: Phoenix,
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
impl<T: HttpTransport, A: PublicAuthentication> PhoenixClient<T, A> {
    pub async fn execute<Q: Serialize, R: DeserializeOwned>(
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
