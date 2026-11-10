use crate::{ApiResponse, Error, Result};
use aux_rs_core::{DefaultTransport, HttpMethod, StreamingHttpTransport};
use aux_rs_data::{HttpClient, params};
use serde::Serialize;

pub const API_URL: &str = "https://public-api.birdeye.so";
/// Native Birdeye data client. Clones share credentials and transport.
#[derive(Clone, Debug)]
pub struct BirdeyeClient<T = DefaultTransport> {
    http: HttpClient<T>,
    chain: String,
}
impl<T> BirdeyeClient<T> {
    pub fn with_transport(api_url: &str, api_key: impl Into<String>, transport: T) -> Result<Self> {
        let key = api_key.into();
        validate_header(&key)?;
        Ok(Self {
            http: HttpClient::new(api_url, transport)?.with_header_auth("X-API-KEY", key),
            chain: "solana".into(),
        })
    }
    /// Chain names are open strings so newly supported chains remain usable.
    pub fn with_chain(mut self, chain: impl Into<String>) -> Result<Self> {
        let chain = chain.into();
        validate_header(&chain)?;
        self.chain = chain;
        Ok(self)
    }
    pub fn transport(&self) -> &T {
        &self.http.transport
    }
}
#[cfg(feature = "http")]
impl BirdeyeClient<aux_rs_core::StandardTransport> {
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::with_config(API_URL, api_key, aux_rs_core::TransportConfig::default())
    }
    pub fn with_config(
        api_url: &str,
        api_key: impl Into<String>,
        config: aux_rs_core::TransportConfig,
    ) -> Result<Self> {
        Self::with_transport(
            api_url,
            api_key,
            aux_rs_core::StandardTransport::new(config)?,
        )
    }
}
impl<T: StreamingHttpTransport> BirdeyeClient<T> {
    pub(crate) async fn request(
        &self,
        method: HttpMethod,
        path: &str,
        parameters: impl Serialize,
    ) -> Result<ApiResponse> {
        let mut parameters = params(
            serde_json::to_value(parameters)
                .map_err(|_| Error::InvalidRequest("invalid Birdeye parameters"))?,
        )?;
        let body = parameters.remove("body");
        // Birdeye POST routes also accept URL query parameters: never form-encode them.
        let mut request = self
            .http
            .request(HttpMethod::Get, path, &parameters, false)?;
        request.method = method;
        request.headers.push(("x-chain".into(), self.chain.clone()));
        if let Some(body) = body {
            request.body = serde_json::to_vec(&body)
                .map_err(|_| Error::InvalidRequest("invalid Birdeye body"))?;
            request
                .headers
                .push(("content-type".into(), "application/json".into()));
        }
        let value = self.http.json(request).await?;
        match value.get("success").and_then(serde_json::Value::as_bool) {
            Some(false) => Err(Error::Api {
                status: 200,
                body: self.http.redact(value),
            }),
            Some(true) => serde_json::from_value(value)
                .map_err(|_| Error::Decode("response does not match Birdeye schema")),
            None => Err(Error::Decode(
                "Birdeye response is missing a boolean success flag",
            )),
        }
    }
}
fn validate_header(value: &str) -> Result<()> {
    if value.is_empty() || !value.bytes().all(|b| (0x21..=0x7e).contains(&b)) {
        Err(Error::InvalidRequest(
            "Birdeye API key and chain must be nonempty printable ASCII without spaces",
        ))
    } else {
        Ok(())
    }
}
