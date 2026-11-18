use crate::*;
use aux_rs_core::{DefaultTransport, HttpMethod, HttpRequest, StreamingHttpTransport};
use aux_rs_data::{HttpClient, params};
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    Mainnet,
    Testnet,
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ChainlinkConfig {
    pub network: Network,
    pub api_url: Option<String>,
    pub ws_url: Option<String>,
    pub candles_url: Option<String>,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub token: Option<String>,
}
impl ChainlinkConfig {
    pub fn new(network: Network) -> Self {
        Self {
            network,
            api_url: None,
            ws_url: None,
            candles_url: None,
            api_key: None,
            api_secret: None,
            token: None,
        }
    }
    pub fn api_url(&self) -> String {
        self.api_url.clone().unwrap_or_else(|| {
            match self.network {
                Network::Mainnet => "https://api.dataengine.chain.link/",
                Network::Testnet => "https://api.testnet-dataengine.chain.link/",
            }
            .into()
        })
    }
    pub fn ws_url(&self) -> String {
        self.ws_url.clone().unwrap_or_else(|| {
            match self.network {
                Network::Mainnet => "wss://ws.dataengine.chain.link/",
                Network::Testnet => "wss://ws.testnet-dataengine.chain.link/",
            }
            .into()
        })
    }
    pub fn candles_url(&self) -> String {
        self.candles_url.clone().unwrap_or_else(|| {
            match self.network {
                Network::Mainnet => "https://priceapi.dataengine.chain.link/",
                Network::Testnet => "https://priceapi.testnet-dataengine.chain.link/",
            }
            .into()
        })
    }
}
#[derive(Clone)]
pub struct Credentials {
    key: Arc<Zeroizing<String>>,
    secret: Arc<Zeroizing<String>>,
}
impl Credentials {
    pub fn new(key: String, secret: String) -> Result<Self> {
        validate_credential(&key)?;
        validate_credential(&secret)?;
        Ok(Self {
            key: Arc::new(Zeroizing::new(key)),
            secret: Arc::new(Zeroizing::new(secret)),
        })
    }
    /// Sign the final URL's encoded path and query, never a reconstructed parameter map.
    pub fn headers_at(
        &self,
        method: &str,
        url: &str,
        body: &[u8],
        timestamp_ms: u64,
    ) -> Result<Vec<(String, String)>> {
        let url = url::Url::parse(url).map_err(|_| Error::InvalidRequest("invalid signing URL"))?;
        let path = match url.query() {
            Some(q) => format!("{}?{q}", url.path()),
            None => url.path().into(),
        };
        let digest = format!("{:x}", Sha256::digest(body));
        let text = format!(
            "{method} {path} {digest} {} {timestamp_ms}",
            self.key.as_str()
        );
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes())
            .map_err(|_| Error::InvalidRequest("invalid HMAC secret"))?;
        mac.update(text.as_bytes());
        let signature = mac
            .finalize()
            .into_bytes()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        Ok(vec![
            ("Authorization".into(), self.key.as_str().into()),
            ("X-Authorization-Timestamp".into(), timestamp_ms.to_string()),
            ("X-Authorization-Signature-SHA256".into(), signature),
        ])
    }
    pub(crate) fn headers(&self, url: &str) -> Result<Vec<(String, String)>> {
        self.headers_at("GET", url, &[], now_ms()?)
    }
    pub(crate) fn scope(&self) -> String {
        format!(
            "chainlink:{:x}:{:x}",
            Sha256::digest(self.key.as_bytes()),
            Sha256::digest(self.secret.as_bytes())
        )
    }
}
fn now_ms() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::InvalidRequest("system clock precedes Unix epoch"))?
        .as_millis()
        .try_into()
        .map_err(|_| Error::InvalidRequest("system clock overflow"))
}
fn validate_credential(s: &str) -> Result<()> {
    if s.is_empty() || !s.bytes().all(|b| (0x21..=0x7e).contains(&b)) {
        return Err(Error::InvalidRequest("invalid Chainlink credential"));
    }
    Ok(())
}
#[derive(Clone)]
pub struct ChainlinkDataStreamsClient<T = DefaultTransport> {
    pub(crate) http: HttpClient<T>,
    pub(crate) ws_url: String,
    pub(crate) credentials: Option<Credentials>,
}
#[derive(Clone)]
pub struct ChainlinkCandlestickClient<T = DefaultTransport> {
    http: HttpClient<T>,
}
impl<T> ChainlinkDataStreamsClient<T> {
    pub fn with_transport(config: ChainlinkConfig, transport: T) -> Result<Self> {
        let ws_url = config.ws_url();
        let url = url::Url::parse(&ws_url)
            .map_err(|_| Error::InvalidRequest("invalid Chainlink WebSocket URL"))?;
        if !matches!(url.scheme(), "ws" | "wss")
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.fragment().is_some()
            || url.query().is_some()
        {
            return Err(Error::InvalidRequest("invalid Chainlink WebSocket URL"));
        }
        let mut http = HttpClient::new(&config.api_url(), transport)?.without_redirects();
        let credentials = match (config.api_key, config.api_secret) {
            (Some(k), Some(s)) => {
                http = http.with_secret(k.clone()).with_secret(s.clone());
                Some(Credentials::new(k, s)?)
            }
            (None, None) => None,
            _ => {
                return Err(Error::InvalidRequest(
                    "both Chainlink API key and secret are required",
                ));
            }
        };
        Ok(Self {
            http,
            ws_url,
            credentials,
        })
    }
    pub fn transport(&self) -> &T {
        &self.http.transport
    }
    pub fn decode_report(&self, full_report: &str) -> Result<DecodedReport> {
        decode_report(full_report)
    }
}
impl<T> ChainlinkCandlestickClient<T> {
    pub fn with_transport(config: ChainlinkConfig, transport: T) -> Result<Self> {
        let mut client = Self {
            http: HttpClient::new(&config.candles_url(), transport)?.without_redirects(),
        };
        if let Some(token) = config.token {
            client = client.with_token(token)?;
        }
        Ok(client)
    }
    pub fn with_token(mut self, token: String) -> Result<Self> {
        validate_credential(&token)?;
        self.http = self.http.with_auth(format!("Bearer {token}"), token);
        Ok(self)
    }
    pub fn transport(&self) -> &T {
        &self.http.transport
    }
}
#[cfg(any(feature = "http", feature = "ws"))]
impl ChainlinkDataStreamsClient<aux_rs_core::StandardTransport> {
    pub fn new(config: ChainlinkConfig) -> Result<Self> {
        Self::with_config(config, Default::default())
    }
    pub fn with_config(
        config: ChainlinkConfig,
        transport: aux_rs_core::TransportConfig,
    ) -> Result<Self> {
        Self::with_transport(config, aux_rs_core::StandardTransport::new(transport)?)
    }
}
#[cfg(feature = "http")]
impl ChainlinkCandlestickClient<aux_rs_core::StandardTransport> {
    pub fn new(config: ChainlinkConfig) -> Result<Self> {
        Self::with_config(config, Default::default())
    }
    pub fn with_config(
        config: ChainlinkConfig,
        transport: aux_rs_core::TransportConfig,
    ) -> Result<Self> {
        Self::with_transport(config, aux_rs_core::StandardTransport::new(transport)?)
    }
}
fn validate_request(p: &serde_json::Map<String, serde_json::Value>) -> Result<()> {
    for name in ["feedID", "feedIDs", "feedId"] {
        if let Some(v) = p.get(name).filter(|v| !v.is_null()) {
            let ids: Vec<_> = match v {
                serde_json::Value::Array(a) => a.iter().collect(),
                v => vec![v],
            };
            if ids.is_empty()
                || ids.iter().any(|v| {
                    v.as_str().is_none_or(|s| {
                        let s = s.strip_prefix("0x").unwrap_or(s);
                        s.len() != 64 || !s.bytes().all(|b| b.is_ascii_hexdigit())
                    })
                })
            {
                return Err(Error::InvalidRequest(
                    "Chainlink feed IDs must be 32-byte hexadecimal values",
                ));
            }
        }
    }
    Ok(())
}
impl<T: StreamingHttpTransport> ChainlinkDataStreamsClient<T> {
    pub(crate) async fn execute(
        &self,
        path: &str,
        request: impl Serialize,
        kind: &str,
    ) -> Result<NativeResponse> {
        let p = params(
            serde_json::to_value(request)
                .map_err(|_| Error::InvalidRequest("invalid Chainlink request"))?,
        )?;
        validate_request(&p)?;
        let mut req = self.http.request(HttpMethod::Get, path, &p, false)?;
        let require_auth = kind != "discovery"
            || p.get("hidden").and_then(serde_json::Value::as_bool) == Some(true);
        if let Some(auth) = &self.credentials {
            req.headers.extend(auth.headers(&req.url)?);
        } else if require_auth {
            return Err(Error::InvalidRequest("Chainlink API credentials required"));
        }
        self.http.json_response(req).await
    }
}
impl<T: StreamingHttpTransport + Clone> ChainlinkCandlestickClient<T> {
    fn request(&self, path: &str, request: impl Serialize, form: bool) -> Result<HttpRequest> {
        let p = params(
            serde_json::to_value(request)
                .map_err(|_| Error::InvalidRequest("invalid Candlestick request"))?,
        )?;
        validate_request(&p)?;
        if path.ends_with("streaming")
            && p.get("symbol").is_none_or(serde_json::Value::is_null)
            && p.get("feedId").is_none_or(serde_json::Value::is_null)
        {
            return Err(Error::InvalidRequest("symbol or feedId is required"));
        }
        self.http.request(
            if form {
                HttpMethod::Post
            } else {
                HttpMethod::Get
            },
            path,
            &p,
            false,
        )
    }
    pub(crate) async fn execute(
        &self,
        path: &str,
        request: impl Serialize,
        kind: &str,
    ) -> Result<NativeResponse> {
        let value = serde_json::to_value(request)
            .map_err(|_| Error::InvalidRequest("invalid Candlestick request"))?;
        // Authorization credentials can be echoed by errors; register them only on this request's owner.
        let mut http = self.http.clone();
        if kind == "form" {
            for name in ["login", "password"] {
                if let Some(s) = value.get(name).and_then(serde_json::Value::as_str) {
                    http = http.with_secret(s.into());
                }
            }
        }
        let mut req = self.request(path, value, kind == "form")?;
        if kind == "form" {
            req.headers
                .retain(|(key, _)| !key.eq_ignore_ascii_case("authorization"));
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
    pub(crate) async fn stream(&self, path: &str, request: impl Serialize) -> Result<JsonStream> {
        let req = self.request(path, request, false)?;
        Ok(JsonStream::http(
            self.http.open(req, true).await?,
            aux_rs_data::JsonFraming::Objects,
        ))
    }
}
