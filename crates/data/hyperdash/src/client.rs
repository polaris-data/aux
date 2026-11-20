use aux_rs_core::{DefaultTransport, HttpMethod, StreamingHttpTransport, TransportError};
use aux_rs_data::{DecodedBody, HttpClient, header, segment};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Map, Value};
use std::{fmt, sync::Arc};
use url::Url;
use zeroize::Zeroizing;

pub const API_URL: &str = "https://data.hyperdash.com";
pub const WS_URL: &str = "wss://data.hyperdash.com/v1/stream";
const MIB: usize = 1024 * 1024;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct HyperdashConfig {
    pub api_url: String,
    pub ws_url: String,
    pub api_key: Option<String>,
    /// Time to receive the initial ready frame after the WebSocket upgrade.
    pub stream_ready_timeout: std::time::Duration,
    /// Maximum ready-to-read messages and queued commands per stream.
    pub stream_capacity: usize,
}

impl Default for HyperdashConfig {
    fn default() -> Self {
        Self {
            api_url: API_URL.into(),
            ws_url: WS_URL.into(),
            api_key: None,
            stream_ready_timeout: std::time::Duration::from_secs(10),
            stream_capacity: 64,
        }
    }
}

impl fmt::Debug for HyperdashConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HyperdashConfig").finish_non_exhaustive()
    }
}

#[derive(Clone)]
pub struct HyperdashClient<T = DefaultTransport> {
    pub(crate) http: HttpClient<T>,
    pub(crate) ws_url: Url,
    pub(crate) authorization: Option<Arc<Zeroizing<String>>>,
    pub(crate) ready_timeout: std::time::Duration,
    pub(crate) stream_capacity: usize,
}

impl<T> fmt::Debug for HyperdashClient<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HyperdashClient").finish_non_exhaustive()
    }
}

fn endpoint(input: &str, schemes: [&str; 2]) -> Result<Url> {
    let url = Url::parse(input).map_err(|_| Error::InvalidRequest("invalid Hyperdash URL"))?;
    if !schemes.contains(&url.scheme())
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || url.query().is_some()
    {
        return Err(Error::InvalidRequest("invalid Hyperdash URL"));
    }
    Ok(url)
}

impl<T> HyperdashClient<T> {
    pub fn with_transport(config: HyperdashConfig, transport: T) -> Result<Self> {
        let api_url = endpoint(&config.api_url, ["http", "https"])?;
        let ws_url = endpoint(&config.ws_url, ["ws", "wss"])?;
        if config.stream_capacity == 0
            || config.stream_capacity > tokio::sync::Semaphore::MAX_PERMITS
            || config.stream_ready_timeout.is_zero()
        {
            return Err(Error::InvalidRequest(
                "invalid Hyperdash stream capacity or timeout",
            ));
        }
        let http = HttpClient::new(api_url.as_str(), transport)
            .map_err(Error::from_data)?
            .without_redirects();
        let authorization = config
            .api_key
            .map(Zeroizing::new)
            .map(|key| {
                if key.is_empty() || !key.bytes().all(|byte| (0x21..=0x7e).contains(&byte)) {
                    return Err(Error::InvalidRequest("invalid Hyperdash API key"));
                }
                Ok(Arc::new(Zeroizing::new(format!("Bearer {}", key.as_str()))))
            })
            .transpose()?;
        Ok(Self {
            http,
            ws_url,
            authorization,
            ready_timeout: config.stream_ready_timeout,
            stream_capacity: config.stream_capacity,
        })
    }
}

impl<T> HyperdashClient<T> {
    pub fn transport(&self) -> &T {
        &self.http.transport
    }
}

#[cfg(any(feature = "http", feature = "ws"))]
impl HyperdashClient<aux_rs_core::StandardTransport> {
    pub fn new(api_key: Option<String>) -> Result<Self> {
        Self::with_config(
            HyperdashConfig {
                api_key,
                ..Default::default()
            },
            Default::default(),
        )
    }

    pub fn with_config(
        config: HyperdashConfig,
        transport: aux_rs_core::TransportConfig,
    ) -> Result<Self> {
        Self::with_transport(config, aux_rs_core::StandardTransport::new(transport)?)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResponseFormat {
    Json,
    Csv,
    Ndjson,
}

impl ResponseFormat {
    fn accept(self) -> &'static str {
        match self {
            Self::Json => "application/json",
            Self::Csv => "text/csv",
            Self::Ndjson => "application/x-ndjson",
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct ResponseMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits_charged: Option<i128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_price_version: Option<i128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits_low: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of: Option<i128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_to: Option<i128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rows: Option<i128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete: Option<bool>,
}

impl ResponseMetadata {
    fn read(headers: &[(String, String)], authorization: Option<&str>) -> Self {
        let raw = |name| header(headers, name);
        let string = |name| {
            raw(name).map(|value| match authorization {
                Some(authorization) => redact_text(value, authorization),
                None => value.to_owned(),
            })
        };
        let integer = |name| raw(name).and_then(|value| value.parse().ok());
        let boolean = |name| raw(name).and_then(|value| value.parse().ok());
        Self {
            request_id: string("X-Request-Id"),
            credits_charged: integer("X-Credits-Charged"),
            credit_price_version: integer("X-Credit-Price-Version"),
            credits_low: boolean("X-Credits-Low"),
            problem_code: string("X-Problem-Code"),
            retry_after: string("Retry-After"),
            has_more: boolean("X-Has-More"),
            next_cursor: string("X-Next-Cursor"),
            as_of: integer("X-As-Of"),
            complete_to: integer("X-Complete-To"),
            truncated: boolean("X-Truncated"),
            max_rows: integer("X-Max-Rows"),
            incomplete: boolean("X-Incomplete"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HyperdashResponse<T> {
    pub status: u16,
    pub metadata: ResponseMetadata,
    pub data: T,
}

pub struct ApiError {
    pub status: u16,
    pub metadata: ResponseMetadata,
    /// Native RFC 9457 problem details, or the body as text for non-JSON failures.
    pub problem: Value,
}

pub enum Error {
    Transport(TransportError),
    Api(Box<ApiError>),
    InvalidRequest(&'static str),
    Decode(&'static str),
    StreamError(Value),
    StreamClosed {
        code: Option<u16>,
        reason: Option<String>,
    },
    Io(std::io::Error),
}

impl Error {
    pub(crate) fn from_data(error: aux_rs_data::Error) -> Self {
        match error {
            aux_rs_data::Error::Transport(error) => Self::Transport(error),
            aux_rs_data::Error::InvalidRequest(error) => Self::InvalidRequest(error),
            aux_rs_data::Error::Decode(error) => Self::Decode(error),
            aux_rs_data::Error::Io(error) => Self::Io(error),
            _ => Self::Decode("unexpected data transport error"),
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Self::Transport(_) => "transport",
            Self::Api(error) => match error.status {
                401 => "authentication",
                403 => "entitlement",
                429 => "rate_limit",
                _ => "vendor",
            },
            Self::InvalidRequest(_) => "validation",
            Self::Decode(_) => "decode",
            Self::StreamError(_) => "stream_error",
            Self::StreamClosed { .. } => "stream_closed",
            Self::Io(_) => "io",
        }
    }
}

impl From<TransportError> for Error {
    fn from(error: TransportError) -> Self {
        Self::Transport(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(error) => error.fmt(f),
            Self::Api(error) => write!(f, "Hyperdash returned HTTP {}", error.status),
            Self::InvalidRequest(message) | Self::Decode(message) => f.write_str(message),
            Self::StreamError(_) => f.write_str("Hyperdash rejected the WebSocket session"),
            Self::StreamClosed {
                code: Some(code), ..
            } => write!(f, "Hyperdash WebSocket closed ({code})"),
            Self::StreamClosed { .. } => f.write_str("Hyperdash WebSocket closed"),
            Self::Io(_) => f.write_str("Hyperdash I/O failed"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Error {}

pub(crate) struct Operation {
    pub method: HttpMethod,
    pub path: &'static str,
    pub auth: bool,
    pub path_fields: &'static [&'static str],
    pub query_fields: &'static [&'static str],
    pub headers: &'static [(&'static str, &'static str)],
    pub batch: bool,
    pub body_limit: usize,
    pub extra_query: bool,
}

impl<T: StreamingHttpTransport> HyperdashClient<T> {
    pub(crate) async fn request<R: DeserializeOwned>(
        &self,
        operation: &Operation,
        request: impl Serialize,
    ) -> Result<HyperdashResponse<R>> {
        let (status, metadata, payload) =
            self.send(operation, request, ResponseFormat::Json).await?;
        let data = serde_json::from_slice(&payload)
            .map_err(|_| Error::Decode("response does not match the Hyperdash schema"))?;
        Ok(HyperdashResponse {
            status,
            metadata,
            data,
        })
    }

    pub(crate) async fn text(
        &self,
        operation: &Operation,
        request: impl Serialize,
        format: ResponseFormat,
    ) -> Result<HyperdashResponse<String>> {
        let (status, metadata, payload) = self.send(operation, request, format).await?;
        let data = String::from_utf8(payload)
            .map_err(|_| Error::Decode("invalid Hyperdash response UTF-8"))?;
        Ok(HyperdashResponse {
            status,
            metadata,
            data,
        })
    }

    async fn send(
        &self,
        operation: &Operation,
        request: impl Serialize,
        format: ResponseFormat,
    ) -> Result<(u16, ResponseMetadata, Vec<u8>)> {
        if operation.auth && self.authorization.is_none() {
            return Err(Error::InvalidRequest("Hyperdash API key is required"));
        }
        let fields = match serde_json::to_value(request) {
            Ok(Value::Object(fields)) => fields,
            _ => return Err(Error::InvalidRequest("invalid Hyperdash request")),
        };
        let http = &self.http;
        let mut path = operation.path.trim_start_matches('/').to_owned();
        for &name in operation.path_fields {
            let value = fields
                .get(name)
                .ok_or(Error::InvalidRequest("missing Hyperdash path parameter"))?;
            let value = scalar(value)?;
            if value.is_empty() || value == "." || value == ".." {
                return Err(Error::InvalidRequest("invalid Hyperdash path parameter"));
            }
            path = path.replace(&format!("{{{name}}}"), &segment(&value));
        }
        let mut request = http
            .request(operation.method, &path, &Map::new(), false)
            .map_err(Error::from_data)?;
        let mut url = Url::parse(&request.url)
            .map_err(|_| Error::InvalidRequest("invalid Hyperdash request URL"))?;
        for &name in operation.query_fields {
            if let Some(value) = fields.get(name).filter(|value| !value.is_null()) {
                url.query_pairs_mut().append_pair(name, &scalar(value)?);
            }
        }
        if operation.extra_query
            && let Some(extra) = fields.get("query").and_then(Value::as_object)
        {
            if extra.contains_key("route") || extra.contains_key("range_ms") {
                return Err(Error::InvalidRequest("duplicate Hyperdash quote parameter"));
            }
            if fields.get("range_ms").is_some_and(|value| !value.is_null())
                && extra.contains_key("start_time")
            {
                return Err(Error::InvalidRequest(
                    "range_ms cannot be combined with start_time",
                ));
            }
            for (name, value) in extra {
                url.query_pairs_mut().append_pair(name, &scalar(value)?);
            }
        }
        request.url = url.into();
        request
            .headers
            .retain(|(key, _)| !matches!(key.as_str(), "accept" | "content-type"));
        request
            .headers
            .push(("accept".into(), format.accept().into()));
        if operation.auth
            && let Some(authorization) = &self.authorization
        {
            request
                .headers
                .push(("authorization".into(), authorization.as_str().into()));
        }
        for &(field, header_name) in operation.headers {
            if let Some(value) = fields.get(field).filter(|value| !value.is_null()) {
                let value = scalar(value)?;
                if value.is_empty()
                    || !value.bytes().all(|byte| (0x20..=0x7e).contains(&byte))
                    || (field == "idempotency_key" && value.len() > 255)
                {
                    return Err(Error::InvalidRequest("invalid Hyperdash request header"));
                }
                request.headers.push((header_name.into(), value));
            }
        }
        if let Some(body) = fields.get("body") {
            request.body = serde_json::to_vec(body)
                .map_err(|_| Error::InvalidRequest("invalid Hyperdash request body"))?;
            if request.body.len() > operation.body_limit {
                return Err(Error::InvalidRequest(
                    "Hyperdash request body exceeded limit",
                ));
            }
            request
                .headers
                .push(("content-type".into(), "application/json".into()));
        }
        let response = http.send(request).await.map_err(Error::from_data)?;
        let status = response.status;
        let metadata = ResponseMetadata::read(
            &response.headers,
            self.authorization.as_ref().map(|value| value.as_str()),
        );
        let limit = if operation.batch { 16 * MIB } else { 2 * MIB };
        let encoding = header(&response.headers, "content-encoding").unwrap_or("");
        let bytes = DecodedBody::new(response.body, encoding)
            .map_err(Error::from_data)?
            .collect(limit)
            .await
            .map_err(Error::from_data)?;
        if !(200..300).contains(&status) {
            let problem = serde_json::from_slice(&bytes)
                .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()));
            return Err(Error::Api(Box::new(ApiError {
                status,
                metadata,
                problem: match &self.authorization {
                    Some(authorization) => redact(problem, authorization),
                    None => problem,
                },
            })));
        }
        Ok((status, metadata, bytes))
    }
}

fn scalar(value: &Value) -> Result<String> {
    match value {
        Value::String(value) => Ok(value.clone()),
        Value::Number(_) | Value::Bool(_) => Ok(value.to_string()),
        _ => Err(Error::InvalidRequest(
            "expected a scalar Hyperdash parameter",
        )),
    }
}

pub(crate) fn redact_text(text: &str, authorization: &str) -> String {
    let result = text.replace(authorization, "[redacted]");
    match authorization.strip_prefix("Bearer ") {
        Some(key) => result.replace(key, "[redacted]"),
        None => result,
    }
}

pub(crate) fn redact(value: Value, authorization: &str) -> Value {
    match value {
        Value::String(text) => Value::String(redact_text(&text, authorization)),
        Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(|value| redact(value, authorization))
                .collect(),
        ),
        Value::Object(values) => Value::Object(
            values
                .into_iter()
                .map(|(name, value)| {
                    (
                        redact_text(&name, authorization),
                        redact(value, authorization),
                    )
                })
                .collect(),
        ),
        value => value,
    }
}
