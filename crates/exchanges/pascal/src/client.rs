use crate::*;
use aux_rs_core::{
    DefaultTransport, Error as ClientError, HttpMethod, HttpRequest, HttpResponse, HttpTransport,
    Unauthenticated,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{collections::BTreeMap, future::Future};
#[derive(Clone)]
pub struct Pascal {
    pub endpoints: BTreeMap<String, String>,
    pub websocket: BTreeMap<String, String>,
    pub network: String,
    pub deployment_id: u8,
}
impl std::fmt::Debug for Pascal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pascal")
            .field("network", &self.network)
            .finish_non_exhaustive()
    }
}
impl Pascal {
    pub fn mainnet() -> Self {
        Self {
            endpoints: [
                ("read".into(), "https://data.pascal.trade".into()),
                ("write".into(), "https://trade.pascal.trade".into()),
            ]
            .into(),
            websocket: [
                ("read".into(), "wss://data.pascal.trade/ws".into()),
                ("write".into(), "wss://trade.pascal.trade/ws".into()),
            ]
            .into(),
            network: "mainnet".into(),
            deployment_id: 3,
        }
    }
    pub fn custom(
        mut self,
        service: &str,
        http_url: &str,
        ws_url: Option<&str>,
    ) -> Result<Self, Error> {
        validate_url(http_url, false)?;
        if !self.endpoints.contains_key(service) {
            return Err(Error::InvalidRequest("unknown native service"));
        }
        self.endpoints
            .insert(service.into(), http_url.trim_end_matches('/').into());
        if let Some(ws) = ws_url {
            validate_url(ws, true)?;
            self.websocket.insert(service.into(), ws.into());
        }
        Ok(self)
    }
}
pub fn validate_url(s: &str, ws: bool) -> Result<(), Error> {
    let u = url::Url::parse(s).map_err(|_| Error::InvalidRequest("invalid endpoint"))?;
    if u.host_str().is_none()
        || !u.username().is_empty()
        || u.password().is_some()
        || u.query().is_some()
        || u.fragment().is_some()
        || !if ws {
            matches!(u.scheme(), "ws" | "wss")
        } else {
            matches!(u.scheme(), "http" | "https")
        }
    {
        return Err(Error::InvalidRequest(
            "invalid endpoint scheme or credentials/query/fragment",
        ));
    }
    Ok(())
}
#[derive(Clone, Deserialize, Serialize)]
pub struct Parameter {
    pub name: String,
    pub location: String,
    pub required: bool,
    #[serde(default = "yes")]
    pub explode: bool,
    #[serde(default)]
    pub style: String,
}
fn yes() -> bool {
    true
}
#[derive(Clone, Deserialize, Serialize)]
pub struct Operation {
    pub service: String,
    pub client: String,
    pub method: String,
    pub verb: String,
    pub path: String,
    pub auth: String,
    pub safe: bool,
    pub parameters: Vec<Parameter>,
    pub has_body: bool,
    #[serde(default)]
    pub fixture: String,
}
pub trait Authentication: Clone + Send + Sync + 'static {
    fn identity(&self) -> String;
    fn begin(&self, _: &Pascal, _: &Operation) -> Result<Submission, Error> {
        Ok(Submission::none())
    }
    fn prepare(
        &self,
        exchange: &Pascal,
        operation: &Operation,
        request: &mut HttpRequest,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
pub fn prepare_request(
    exchange: &Pascal,
    op: &Operation,
    params: Value,
) -> Result<HttpRequest, Error> {
    let mut fields = params
        .as_object()
        .cloned()
        .ok_or(Error::InvalidRequest("parameter object required"))?;
    let mut path = op.path.clone();
    let mut query = Vec::new();
    let mut headers = vec![("content-type".into(), "application/json".into())];
    for p in &op.parameters {
        let Some(v) = fields.remove(&p.name).filter(|v| !v.is_null()) else {
            if p.required {
                return Err(Error::InvalidRequest("missing required native parameter"));
            }
            continue;
        };
        let text = |x: &Value| -> Result<String, Error> {
            match x {
                Value::String(s) => Ok(s.clone()),
                Value::Bool(_) | Value::Number(_) => Ok(x.to_string()),
                _ => Err(Error::InvalidRequest("scalar parameter required")),
            }
        };
        match p.location.as_str() {
            "path" => {
                path = path.replace(
                    &format!("{{{}}}", p.name),
                    &aux_rs_core::url_component(&text(&v)?),
                );
            }
            "query" => {
                let values = if let Value::Array(xs) = v {
                    xs.iter().map(text).collect::<Result<Vec<_>, _>>()?
                } else {
                    vec![text(&v)?]
                };
                if p.explode {
                    for x in values {
                        query.push((p.name.clone(), x));
                    }
                } else {
                    query.push((p.name.clone(), values.join(",")));
                }
            }
            "header" => headers.push((p.name.clone(), text(&v)?)),
            _ => {
                return Err(Error::InvalidRequest(
                    "unsupported native parameter location",
                ));
            }
        }
    }
    if path.contains(['{', '}']) {
        return Err(Error::InvalidRequest("unresolved native path parameter"));
    }
    let origin = exchange
        .endpoints
        .get(&op.service)
        .ok_or(Error::InvalidRequest("service endpoint unavailable"))?;
    let mut url = format!("{}{path}", origin.trim_end_matches('/'));
    if !query.is_empty() {
        let mut q = url::form_urlencoded::Serializer::new(String::new());
        for (k, v) in query {
            q.append_pair(&k, &v);
        }
        url.push('?');
        url.push_str(&q.finish());
    }
    let body = fields.remove("body");
    if !fields.is_empty() {
        return Err(Error::InvalidRequest("unknown native request parameter"));
    }
    Ok(HttpRequest {
        method: match op.verb.as_str() {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            "PUT" => HttpMethod::Put,
            "PATCH" => HttpMethod::Patch,
            "DELETE" => HttpMethod::Delete,
            _ => return Err(Error::InvalidRequest("unknown HTTP verb")),
        },
        url,
        headers,
        body: body
            .map(|b| serde_json::to_vec(&b))
            .transpose()?
            .unwrap_or_default(),
    })
}
pub fn decode_response<R: DeserializeOwned>(response: HttpResponse) -> Result<R, Error> {
    if !(200..300).contains(&response.status) {
        return Err(Error::Api {
            status: response.status,
            body: serde_json::from_slice(&response.body).unwrap_or_else(|_| {
                Value::String(String::from_utf8_lossy(&response.body).into_owned())
            }),
        });
    }
    let value: Value = if response.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&response.body)?
    };
    if value.get("status").and_then(Value::as_str) == Some("error") {
        return Err(Error::Api {
            status: response.status,
            body: value,
        });
    }
    Ok(serde_json::from_value(value)?)
}
macro_rules! define_client {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name<T = DefaultTransport, A = Unauthenticated> {
            pub(crate) exchange: Pascal,
            pub(crate) transport: T,
            pub(crate) auth: A,
        }
        impl<T, A> std::fmt::Debug for $name<T, A> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }
        impl<T> $name<T> {
            pub fn with_transport(exchange: Pascal, transport: T) -> Self {
                Self {
                    exchange,
                    transport,
                    auth: Unauthenticated,
                }
            }
        }
        impl<T, A> $name<T, A> {
            pub fn with_auth<B>(self, auth: B) -> $name<T, B> {
                $name {
                    exchange: self.exchange,
                    transport: self.transport,
                    auth,
                }
            }
            pub fn transport(&self) -> &T {
                &self.transport
            }
            pub fn exchange(&self) -> &Pascal {
                &self.exchange
            }
        }
        #[cfg(any(feature = "http", feature = "ws"))]
        impl $name<aux_rs_core::StandardTransport> {
            pub fn mainnet() -> Result<Self, aux_rs_core::TransportError> {
                Self::with_config(Pascal::mainnet(), Default::default())
            }
            pub fn with_config(
                exchange: Pascal,
                config: aux_rs_core::TransportConfig,
            ) -> Result<Self, aux_rs_core::TransportError> {
                Ok(Self::with_transport(
                    exchange,
                    aux_rs_core::StandardTransport::new(config)?,
                ))
            }
        }
        impl<T: HttpTransport, A> $name<T, A> {
            pub(crate) async fn execute_public<R: DeserializeOwned>(
                &self,
                descriptor: &str,
                params: Value,
            ) -> Result<R, ClientError<Error>> {
                self.transport.ensure_open()?;
                let op: Operation = serde_json::from_str(descriptor)
                    .map_err(|e| ClientError::Exchange(e.into()))?;
                let request =
                    prepare_request(&self.exchange, &op, params).map_err(ClientError::Exchange)?;
                let response = if op.safe {
                    self.transport.execute_read(request).await?
                } else {
                    self.transport.execute(request).await?
                };
                decode_response(response).map_err(ClientError::Exchange)
            }
        }
        impl<T: HttpTransport, A: Authentication> $name<T, A> {
            pub(crate) async fn execute<R: DeserializeOwned>(
                &self,
                descriptor: &str,
                params: Value,
            ) -> Result<R, ClientError<Error>> {
                self.transport.ensure_open()?;
                let op: Operation = serde_json::from_str(descriptor)
                    .map_err(|e| ClientError::Exchange(e.into()))?;
                let mut request =
                    prepare_request(&self.exchange, &op, params).map_err(ClientError::Exchange)?;
                let mut submission = self
                    .auth
                    .begin(&self.exchange, &op)
                    .map_err(ClientError::Exchange)?;
                self.auth
                    .prepare(&self.exchange, &op, &mut request)
                    .await
                    .map_err(ClientError::Exchange)?;
                submission.sent();
                let response = match self.transport.execute(request).await {
                    Ok(r) => r,
                    Err(e) => {
                        if e.outcome == aux_rs_core::RequestOutcome::NotSent {
                            submission.finish();
                        }
                        return Err(e.into());
                    }
                };
                let rejected = (400..500).contains(&response.status) && response.status != 408;
                let decoded = decode_response(response);
                if rejected || decoded.is_ok() {
                    submission.finish();
                }
                decoded.map_err(ClientError::Exchange)
            }
        }
    };
}
define_client!(PascalClient);
