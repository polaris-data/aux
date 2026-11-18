use crate::*;
use aux_rs_core::{DefaultTransport, HttpMethod, StreamingHttpTransport};
use aux_rs_data::{HttpClient, Params, params, segment};
use serde::Serialize;
pub const PRO_API_URL: &str = "https://pyth.dourolabs.app/v1/";
pub const ROUTER_URL: &str = "https://pyth-lazer.dourolabs.app/";
pub const HERMES_URL: &str = "https://hermes.pyth.network/";
pub fn default_ws_urls() -> Vec<String> {
    (0..3)
        .map(|n| format!("wss://pyth-lazer-{n}.dourolabs.app/v1/stream"))
        .collect()
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PythConfig {
    pub api_url: String,
    pub router_url: String,
    pub hermes_url: String,
    pub ws_urls: Vec<String>,
    pub api_key: Option<String>,
    pub publisher_token: Option<String>,
}
impl Default for PythConfig {
    fn default() -> Self {
        Self {
            api_url: PRO_API_URL.into(),
            router_url: ROUTER_URL.into(),
            hermes_url: HERMES_URL.into(),
            ws_urls: default_ws_urls(),
            api_key: None,
            publisher_token: None,
        }
    }
}
#[derive(Clone)]
pub struct PythProClient<T = DefaultTransport> {
    pub(crate) http: HttpClient<T>,
    router: HttpClient<T>,
    publisher: HttpClient<T>,
    pub(crate) urls: Vec<String>,
    pub(crate) token: Option<std::sync::Arc<zeroize::Zeroizing<String>>>,
}
#[derive(Clone)]
pub struct PythHermesClient<T = DefaultTransport> {
    http: HttpClient<T>,
}
fn http<T>(url: &str, token: Option<&str>, transport: T) -> Result<HttpClient<T>> {
    let client = HttpClient::new(url, transport)?;
    match token {
        Some(key) => {
            if key.is_empty() || !key.bytes().all(|b| (0x21..=0x7e).contains(&b)) {
                return Err(Error::InvalidRequest("invalid Pyth credential"));
            }
            Ok(client.with_auth(format!("Bearer {key}"), key.into()))
        }
        None => Ok(client),
    }
}
impl<T: Clone> PythProClient<T> {
    pub fn with_transport(config: PythConfig, transport: T) -> Result<Self> {
        if config.ws_urls.is_empty() {
            return Err(Error::InvalidRequest(
                "at least one Pyth WebSocket endpoint is required",
            ));
        }
        for url in &config.ws_urls {
            validate_ws_url(url)?;
        }
        Ok(Self {
            http: http(
                &config.api_url,
                config.api_key.as_deref(),
                transport.clone(),
            )?,
            router: http(
                &config.router_url,
                config.api_key.as_deref(),
                transport.clone(),
            )?,
            publisher: http(
                &config.api_url,
                config.publisher_token.as_deref(),
                transport,
            )?,
            urls: config.ws_urls,
            token: config
                .api_key
                .map(|s| std::sync::Arc::new(zeroize::Zeroizing::new(s))),
        })
    }
}
impl<T> PythProClient<T> {
    pub fn transport(&self) -> &T {
        &self.http.transport
    }
}
impl<T> PythHermesClient<T> {
    pub fn with_transport(config: PythConfig, transport: T) -> Result<Self> {
        Ok(Self {
            http: http(&config.hermes_url, config.api_key.as_deref(), transport)?,
        })
    }
    pub fn transport(&self) -> &T {
        &self.http.transport
    }
}
#[cfg(any(feature = "http", feature = "ws"))]
impl PythProClient<aux_rs_core::StandardTransport> {
    pub fn new(config: PythConfig) -> Result<Self> {
        Self::with_config(config, Default::default())
    }
    pub fn with_config(
        config: PythConfig,
        transport: aux_rs_core::TransportConfig,
    ) -> Result<Self> {
        Self::with_transport(config, aux_rs_core::StandardTransport::new(transport)?)
    }
}
#[cfg(feature = "http")]
impl PythHermesClient<aux_rs_core::StandardTransport> {
    pub fn new(config: PythConfig) -> Result<Self> {
        Self::with_config(config, Default::default())
    }
    pub fn with_config(
        config: PythConfig,
        transport: aux_rs_core::TransportConfig,
    ) -> Result<Self> {
        Self::with_transport(config, aux_rs_core::StandardTransport::new(transport)?)
    }
}
pub(crate) fn validate_ws_url(value: &str) -> Result<()> {
    let url = url::Url::parse(value).map_err(|_| Error::InvalidRequest("invalid WebSocket URL"))?;
    if !matches!(url.scheme(), "ws" | "wss")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err(Error::InvalidRequest("invalid WebSocket URL"));
    }
    Ok(())
}
pub(crate) fn prepare(
    path: &str,
    request: impl Serialize,
    hermes: bool,
) -> Result<(String, Params)> {
    let mut p = params(
        serde_json::to_value(request).map_err(|_| Error::InvalidRequest("invalid Pyth request"))?,
    )?;
    let mut path = path.to_owned();
    for key in ["channel", "publish_time", "window_seconds"] {
        if path.contains(&format!("{{{key}}}")) {
            let value = p
                .remove(key)
                .ok_or(Error::InvalidRequest("missing Pyth path parameter"))?;
            let value = value
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| value.to_string());
            if value.is_empty() || value == "." || value == ".." {
                return Err(Error::InvalidRequest("invalid Pyth path parameter"));
            }
            path = path.replace(&format!("{{{key}}}"), &segment(&value));
        }
    }
    if hermes
        && let Some(ids) = p.get("ids").and_then(serde_json::Value::as_array)
        && (ids.is_empty()
            || ids.iter().any(|id| {
                id.as_str().is_none_or(|id| {
                    let s = id.strip_prefix("0x").unwrap_or(id);
                    s.len() != 64 || !s.bytes().all(|c| c.is_ascii_hexdigit())
                })
            }))
    {
        return Err(Error::InvalidRequest(
            "Hermes feed IDs must be 32-byte hexadecimal values",
        ));
    }
    Ok((path, p))
}
fn make_request<T>(
    http: &HttpClient<T>,
    path: &str,
    p: &Params,
    kind: &str,
    hermes: bool,
) -> Result<aux_rs_core::HttpRequest> {
    let empty = Params::new();
    let query = if kind == "post" { &empty } else { p };
    let mut req = http.request(HttpMethod::Get, path, query, false)?;
    if hermes && let Some(ids) = p.get("ids").and_then(serde_json::Value::as_array) {
        let mut url = http.url(path)?;
        for (k, v) in aux_rs_data::encode_params(p, false)? {
            if k != "ids" {
                url.query_pairs_mut().append_pair(&k, &v);
            }
        }
        for id in ids {
            url.query_pairs_mut()
                .append_pair("ids[]", id.as_str().expect("validated IDs"));
        }
        req.url = url.into();
    }
    if kind == "post" {
        req.method = HttpMethod::Post;
        req.url = http.url(path)?.into();
        req.body = serde_json::to_vec(p).map_err(|_| Error::InvalidRequest("invalid Pyth JSON"))?;
        req.headers
            .push(("content-type".into(), "application/json".into()));
    }
    Ok(req)
}
async fn request<T: StreamingHttpTransport>(
    http: &HttpClient<T>,
    path: &str,
    p: Params,
    kind: &str,
    hermes: bool,
) -> Result<NativeResponse> {
    let req = make_request(http, path, &p, kind, hermes)?;
    if kind == "text" {
        let response = http.send(req).await?;
        let status = response.status;
        let encoding = aux_rs_data::header(&response.headers, "content-encoding").unwrap_or("");
        let bytes = aux_rs_data::DecodedBody::new(response.body, encoding)?
            .collect(aux_rs_data::MAX_RECORD_BYTES)
            .await?;
        if !(200..300).contains(&status) {
            return Err(Error::Api {
                status,
                body: http.redact(serde_json::Value::String(
                    String::from_utf8_lossy(&bytes).into(),
                )),
            });
        }
        return Ok(NativeResponse {
            status,
            headers: Vec::new(),
            data: serde_json::Value::String(
                String::from_utf8(bytes).map_err(|_| Error::Decode("invalid health response"))?,
            ),
        });
    }
    let response = http.json_response(req).await?;
    if response.data.get("s").and_then(serde_json::Value::as_str) == Some("error") {
        return Err(Error::Api {
            status: response.status,
            body: http.redact(response.data),
        });
    }
    Ok(response)
}
impl<T: StreamingHttpTransport> PythProClient<T> {
    pub(crate) async fn execute(
        &self,
        path: &str,
        request_value: impl Serialize,
        kind: &str,
    ) -> Result<NativeResponse> {
        let (path, p) = prepare(path, request_value, false)?;
        let http = match kind {
            "post" | "router_get" => &self.router,
            "publisher" => &self.publisher,
            _ => &self.http,
        };
        request(http, &path, p, kind, false).await
    }
}
impl<T: StreamingHttpTransport> PythHermesClient<T> {
    pub(crate) async fn execute(
        &self,
        path: &str,
        request_value: impl Serialize,
        kind: &str,
    ) -> Result<NativeResponse> {
        let (path, p) = prepare(path, request_value, true)?;
        request(&self.http, &path, p, kind, true).await
    }
    pub(crate) async fn stream(
        &self,
        path: &str,
        request_value: impl Serialize,
    ) -> Result<JsonStream> {
        let (path, p) = prepare(path, request_value, true)?;
        let mut req = make_request(&self.http, &path, &p, "get", true)?;
        req.headers
            .retain(|(k, _)| !k.eq_ignore_ascii_case("accept"));
        req.headers
            .push(("accept".into(), "text/event-stream".into()));
        Ok(JsonStream::http(
            self.http.open(req, true).await?,
            aux_rs_data::JsonFraming::Sse,
        ))
    }
}
