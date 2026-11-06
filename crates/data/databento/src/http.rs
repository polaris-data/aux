use crate::DbnStream;
use aux_rs_core::{DefaultTransport, HttpMethod, StreamingHttpTransport};
use aux_rs_data::{DecodedBody, Error, HttpClient, Lines, Params, Result, check_batch_limit};
use base64::Engine;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct HistoricalClient<T = DefaultTransport> {
    http: HttpClient<T>,
}
#[derive(Clone, Debug)]
pub struct ReferenceClient<T = DefaultTransport> {
    http: HttpClient<T>,
}
fn client<T>(url: &str, key: String, transport: T) -> Result<HttpClient<T>> {
    crate::validate_key(&key)?;
    let auth = base64::engine::general_purpose::STANDARD.encode(format!("{key}:"));
    Ok(HttpClient::new(url, transport)?.with_auth(format!("Basic {auth}"), key))
}
macro_rules! constructor {
    ($name:ident) => {
        impl<T> $name<T> {
            pub fn with_transport(
                api_url: &str,
                api_key: impl Into<String>,
                transport: T,
            ) -> Result<Self> {
                Ok(Self {
                    http: client(api_url, api_key.into(), transport)?,
                })
            }
            pub fn transport(&self) -> &T {
                &self.http.transport
            }
        }
        #[cfg(feature = "http")]
        impl $name<aux_rs_core::StandardTransport> {
            pub fn new(api_key: impl Into<String>) -> Result<Self> {
                Self::with_config(api_key, aux_rs_core::TransportConfig::default())
            }
            pub fn with_config(
                api_key: impl Into<String>,
                config: aux_rs_core::TransportConfig,
            ) -> Result<Self> {
                Self::with_transport(
                    "https://hist.databento.com/v0",
                    api_key,
                    aux_rs_core::StandardTransport::new(config)?,
                )
            }
        }
    };
}
constructor!(HistoricalClient);
constructor!(ReferenceClient);
macro_rules! json_methods {($($name:ident,$verb:ident,$path:literal;)+)=>{$(
    pub async fn $name(&self,parameters:Params)->Result<Value> {
        self.http.json(self.http.request(HttpMethod::$verb,$path,&parameters,false)?).await
    }
)+};}
impl<T: StreamingHttpTransport> HistoricalClient<T> {
    json_methods! {
        list_publishers,Get,"metadata.list_publishers";
        list_datasets,Get,"metadata.list_datasets";
        list_schemas,Get,"metadata.list_schemas";
        list_fields,Get,"metadata.list_fields";
        list_unit_prices,Get,"metadata.list_unit_prices";
        get_dataset_condition,Get,"metadata.get_dataset_condition";
        get_dataset_range,Get,"metadata.get_dataset_range";
        get_record_count,Post,"metadata.get_record_count";
        get_billable_size,Post,"metadata.get_billable_size";
        get_cost,Post,"metadata.get_cost";
        resolve,Post,"symbology.resolve";
        submit_job,Post,"batch.submit_job";
        list_jobs,Get,"batch.list_jobs";
        get_job_details,Get,"batch.get_job_details";
        list_files,Get,"batch.list_files";
    }
    /// Returns encoded timeseries bytes. Native `encoding` and `compression`
    /// parameters are respected, including CSV/JSON encodings.
    pub async fn timeseries(&self, parameters: Params) -> Result<DecodedBody> {
        let mut wire =
            self.http
                .request(HttpMethod::Post, "timeseries.get_range", &parameters, false)?;
        wire.headers.retain(|(k, _)| k != "accept");
        wire.headers
            .push(("accept".into(), "application/octet-stream".into()));
        self.http.open(wire, false).await
    }
    /// Incrementally decodes DBN metadata and records. HTTP compression is decoded
    /// separately from DBN's explicitly requested file compression.
    pub async fn get_range(&self, mut parameters: Params) -> Result<DbnStream> {
        parameters.insert("encoding".into(), Value::String("dbn".into()));
        parameters.insert("compression".into(), Value::String("none".into()));
        let mut wire =
            self.http
                .request(HttpMethod::Post, "timeseries.get_range", &parameters, false)?;
        wire.headers.retain(|(key, _)| key != "accept");
        wire.headers
            .push(("accept".into(), "application/octet-stream".into()));
        Ok(DbnStream::new(self.http.open(wire, true).await?))
    }
    /// Download a URL from `list_files`. Authentication is sent only to this
    /// client's origin; cross-origin storage URLs use their own signed query.
    pub async fn download(&self, url: &str) -> Result<DecodedBody> {
        let target =
            url::Url::parse(url).map_err(|_| Error::InvalidRequest("invalid download URL"))?;
        if !matches!(target.scheme(), "http" | "https")
            || !target.username().is_empty()
            || target.password().is_some()
            || target.fragment().is_some()
        {
            return Err(Error::InvalidRequest(
                "download URL requires HTTP(S) without user information or fragment",
            ));
        }
        let mut wire = self
            .http
            .request(HttpMethod::Get, "", &Params::new(), false)?;
        let source =
            url::Url::parse(&wire.url).map_err(|_| Error::InvalidRequest("invalid API URL"))?;
        if source.scheme() == "https" && target.scheme() != "https" {
            return Err(Error::InvalidRequest("insecure download URL"));
        }
        if source.origin() != target.origin() {
            wire.headers
                .retain(|(k, _)| !k.eq_ignore_ascii_case("authorization"));
        }
        wire.url = url.to_owned();
        self.http.open(wire, false).await
    }
    pub async fn download_to(&self, url: &str, path: impl AsRef<std::path::Path>) -> Result<u64> {
        aux_rs_data::download_to(self.download(url).await?, path).await
    }
}
impl<T: StreamingHttpTransport> ReferenceClient<T> {
    json_methods! {
        list_events,Get,"corporate_actions.list_events";
        list_enums,Get,"corporate_actions.list_enums";
    }
    async fn records(&self, path: &str, mut parameters: Params) -> Result<ReferenceStream> {
        parameters
            .entry("compression")
            .or_insert(Value::String("none".into()));
        let encoding = parameters
            .get("compression")
            .and_then(Value::as_str)
            .ok_or(Error::InvalidRequest("compression must be a string"))?;
        if !matches!(encoding, "none" | "zstd") {
            return Err(Error::InvalidRequest(
                "reference compression must be none or zstd",
            ));
        }
        let wire = self
            .http
            .request(HttpMethod::Post, path, &parameters, false)?;
        Ok(ReferenceStream::new(
            self.http.open_encoded(wire, encoding).await?,
        ))
    }
    pub async fn security_master_get_range(&self, parameters: Params) -> Result<ReferenceStream> {
        self.records("security_master.get_range", parameters).await
    }
    pub async fn security_master_get_last(&self, parameters: Params) -> Result<ReferenceStream> {
        self.records("security_master.get_last", parameters).await
    }
    pub async fn corporate_actions_get_range(&self, parameters: Params) -> Result<ReferenceStream> {
        self.records("corporate_actions.get_range", parameters)
            .await
    }
    pub async fn adjustment_factors_get_range(
        &self,
        parameters: Params,
    ) -> Result<ReferenceStream> {
        self.records("adjustment_factors.get_range", parameters)
            .await
    }
}
/// Unmodified reference-data JSON rows; numeric values remain exact.
pub struct ReferenceStream {
    lines: Lines,
    queued: std::collections::VecDeque<Result<Value>>,
    done: bool,
}
impl ReferenceStream {
    pub fn new(body: DecodedBody) -> Self {
        Self {
            lines: Lines::new(body),
            queued: std::collections::VecDeque::new(),
            done: false,
        }
    }
    async fn read_one(&mut self) -> Option<Result<Value>> {
        if self.done {
            return None;
        }
        match self.lines.next().await {
            Some(Ok(line)) => {
                let value = serde_json::from_slice(&line)
                    .map_err(|_| Error::Decode("invalid reference JSON row"));
                if value.is_err() {
                    self.done = true;
                    self.lines.stop();
                }
                Some(value)
            }
            Some(Err(e)) => {
                self.done = true;
                Some(Err(e))
            }
            None => {
                self.done = true;
                None
            }
        }
    }
    pub async fn next(&mut self) -> Option<Result<Value>> {
        if let Some(v) = self.queued.pop_front() {
            Some(v)
        } else {
            self.read_one().await
        }
    }
    pub async fn read_batch(&mut self, limit: usize) -> Result<Vec<Value>> {
        check_batch_limit(limit)?;
        while self.queued.len() < limit && !self.queued.back().is_some_and(Result::is_err) {
            match self.read_one().await {
                Some(v) => self.queued.push_back(v),
                None => break,
            }
        }
        let mut out = Vec::new();
        while out.len() < limit {
            if self.queued.front().is_some_and(Result::is_err) {
                if out.is_empty() {
                    return Err(self.queued.pop_front().unwrap().unwrap_err());
                }
                break;
            }
            match self.queued.pop_front() {
                Some(Ok(v)) => out.push(v),
                _ => break,
            }
        }
        Ok(out)
    }
    pub async fn close(&mut self) -> Result<()> {
        self.done = true;
        self.queued.clear();
        self.lines.close().await
    }
}
