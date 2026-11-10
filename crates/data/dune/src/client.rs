use crate::{CsvResults, Error, Result, ResultOptions};
use aux_rs_core::{DefaultTransport, HttpMethod, HttpRequest, StreamingHttpTransport};
use aux_rs_data::{HttpClient, MAX_RECORD_BYTES, Params, header, params, segment};
use serde::{Serialize, de::DeserializeOwned};

pub const API_URL: &str = "https://api.dune.com/api/v1";
/// Single-request native client. Clones share their transport and credentials.
#[derive(Clone, Debug)]
pub struct DuneClient<T = DefaultTransport> {
    http: HttpClient<T>,
}
impl<T> DuneClient<T> {
    pub fn with_transport(api_url: &str, api_key: impl Into<String>, transport: T) -> Result<Self> {
        let key = api_key.into();
        if key.is_empty() || !key.bytes().all(|b| (0x21..=0x7e).contains(&b)) {
            return Err(Error::InvalidRequest(
                "Dune API key must be nonempty printable ASCII without spaces",
            ));
        }
        Ok(Self {
            http: HttpClient::new(api_url, transport)?.with_header_auth("X-Dune-API-Key", key),
        })
    }
    pub fn transport(&self) -> &T {
        &self.http.transport
    }
}
#[cfg(feature = "http")]
impl DuneClient<aux_rs_core::StandardTransport> {
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
impl<T: StreamingHttpTransport> DuneClient<T> {
    pub(crate) async fn get<R: DeserializeOwned>(
        &self,
        path: &str,
        options: ResultOptions,
    ) -> Result<R> {
        options.validate()?;
        let request = self
            .http
            .request(HttpMethod::Get, path, &native_params(&options)?, false)?;
        decode(self.http.json(request).await?)
    }
    pub(crate) async fn post<R: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<impl Serialize>,
    ) -> Result<R> {
        let mut request = self
            .http
            .request(HttpMethod::Post, path, &Params::new(), false)?;
        request
            .headers
            .retain(|(name, _)| !name.eq_ignore_ascii_case("content-type"));
        if let Some(body) = body {
            request.body = serde_json::to_vec(&body)
                .map_err(|_| Error::InvalidRequest("invalid Dune request"))?;
            request
                .headers
                .push(("content-type".into(), "application/json".into()));
        }
        decode(self.http.json(request).await?)
    }
    pub(crate) async fn csv(&self, path: &str, options: ResultOptions) -> Result<CsvResults> {
        options.validate()?;
        let mut request: HttpRequest =
            self.http
                .request(HttpMethod::Get, path, &native_params(&options)?, false)?;
        for (name, value) in &mut request.headers {
            if name.eq_ignore_ascii_case("accept") {
                *value = "text/csv".into();
            }
        }
        let (headers, body) = self.http.open_with_headers(request, true).await?;
        let next_offset = header(&headers, "X-Dune-Next-Offset")
            .map(|v| v.parse())
            .transpose()
            .map_err(|_| Error::Decode("invalid Dune pagination offset"))?;
        let next_uri = header(&headers, "X-Dune-Next-Uri").map(str::to_owned);
        let data = String::from_utf8(body.collect(MAX_RECORD_BYTES).await?)
            .map_err(|_| Error::Decode("invalid Dune CSV UTF-8"))?;
        Ok(CsvResults {
            data,
            next_offset,
            next_uri,
        })
    }
}
fn native_params(value: &impl Serialize) -> Result<Params> {
    params(
        serde_json::to_value(value)
            .map_err(|_| Error::InvalidRequest("invalid Dune parameters"))?,
    )
}
fn decode<R: DeserializeOwned>(value: serde_json::Value) -> Result<R> {
    serde_json::from_value(value).map_err(|_| Error::Decode("response does not match Dune schema"))
}
pub(crate) fn query_path(id: u64) -> Result<String> {
    if id == 0 {
        return Err(Error::InvalidRequest("query_id must be positive"));
    }
    Ok(format!("query/{id}"))
}
pub(crate) fn execution_path(id: &str) -> Result<String> {
    if id.is_empty() || id == "." || id == ".." || id.chars().any(char::is_whitespace) {
        return Err(Error::InvalidRequest(
            "execution_id must be nonempty without whitespace or dot segments",
        ));
    }
    Ok(format!("execution/{}", segment(id)))
}
impl ResultOptions {
    fn validate(&self) -> Result<()> {
        if self.sample_count.is_some()
            && (self.limit.is_some() || self.offset.is_some() || self.filters.is_some())
        {
            return Err(Error::InvalidRequest(
                "sample_count cannot be combined with limit, offset or filters",
            ));
        }
        Ok(())
    }
}
