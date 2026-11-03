use crate::{DatasetRequest, FeedRequest, FeedStream, MachineStream, ReplayRequest};
use aux_rs_core::{DefaultTransport, HttpMethod, StreamingHttpTransport, WebSocketConnector};
use aux_rs_data::{DecodedBody, Error, HttpClient, Params, Result, params, segment};
use serde_json::{Value, json};
use time::{
    Date, OffsetDateTime, PrimitiveDateTime, UtcOffset, format_description::well_known::Rfc3339,
};

#[derive(Clone, Debug)]
pub struct TardisClient<T = DefaultTransport> {
    http: HttpClient<T>,
    datasets: HttpClient<T>,
}
impl<T: Clone> TardisClient<T> {
    pub fn with_transport(api_url: &str, datasets_url: &str, transport: T) -> Result<Self> {
        Ok(Self {
            http: HttpClient::new(api_url, transport.clone())?,
            datasets: HttpClient::new(datasets_url, transport)?,
        })
    }
    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        let key = key.into();
        self.http = self.http.with_auth(format!("Bearer {key}"), key.clone());
        self.datasets = self.datasets.with_auth(format!("Bearer {key}"), key);
        self
    }
    pub fn transport(&self) -> &T {
        &self.http.transport
    }
}
impl<T: StreamingHttpTransport + Clone> TardisClient<T> {
    pub async fn exchanges(&self) -> Result<Value> {
        self.get("exchanges", Params::new()).await
    }
    pub async fn symbols(&self, parameters: Params) -> Result<Value> {
        self.http
            .json(
                self.http
                    .request(HttpMethod::Get, "symbols", &parameters, false)?,
            )
            .await
    }
    pub async fn api_key_info(&self) -> Result<Value> {
        self.get("api-key-info", Params::new()).await
    }
    pub async fn exchange_details(&self, exchange: &str) -> Result<Value> {
        self.get(&format!("exchanges/{}", segment(exchange)), Params::new())
            .await
    }
    pub async fn exchange_summary(&self, exchange: &str) -> Result<Value> {
        self.get(
            &format!("exchanges/{}/summary", segment(exchange)),
            Params::new(),
        )
        .await
    }
    pub async fn instruments(&self, exchange: &str, parameters: Params) -> Result<Value> {
        self.get(&format!("instruments/{}", segment(exchange)), parameters)
            .await
    }
    pub async fn instrument(&self, exchange: &str, symbol: &str) -> Result<Value> {
        self.get(
            &format!("instruments/{}/{}", segment(exchange), segment(symbol)),
            Params::new(),
        )
        .await
    }
    async fn get(&self, path: &str, parameters: Params) -> Result<Value> {
        self.http
            .json(
                self.http
                    .request(HttpMethod::Get, path, &parameters, true)?,
            )
            .await
    }
    pub async fn data_feeds(&self, request: FeedRequest) -> Result<FeedStream> {
        request.validate()?;
        let mut p = params(
            json!({"from":request.from,"offset":request.offset,"sliceSize":request.slice_size}),
        )?;
        if !request.filters.is_empty() {
            p.insert(
                "filters".into(),
                serde_json::to_value(request.filters)
                    .map_err(|_| Error::InvalidRequest("invalid filters"))?,
            );
        }
        let mut wire = self.http.request(
            HttpMethod::Get,
            &format!("data-feeds/{}", segment(&request.exchange)),
            &p,
            true,
        )?;
        wire.headers
            .push(("accept-encoding".into(), "gzip, zstd".into()));
        Ok(FeedStream::new(self.http.open(wire, true).await?, true))
    }
    pub fn replay(&self, request: ReplayRequest) -> Result<Replay<T>> {
        let from = parse_minute(&request.from)?;
        let to = parse_minute(&request.to)?;
        if from >= to {
            return Err(Error::InvalidRequest(
                "replay requires from before exclusive to",
            ));
        }
        let feed = FeedRequest {
            exchange: request.exchange,
            from: request.from,
            offset: 0,
            filters: request.filters,
            slice_size: 1,
        };
        feed.validate()?;
        Ok(Replay {
            client: self.clone(),
            feed,
            remaining: (to - from).whole_minutes() as u64,
            current: None,
            done: false,
            queued: std::collections::VecDeque::new(),
        })
    }
    pub async fn dataset(&self, request: DatasetRequest) -> Result<DecodedBody> {
        let date = Date::parse(
            &request.date,
            &time::format_description::parse_borrowed::<2>("[year]-[month]-[day]")
                .map_err(|_| Error::InvalidRequest("invalid date format"))?,
        )
        .map_err(|_| Error::InvalidRequest("dataset date must be YYYY-MM-DD"))?;
        let path = format!(
            "{}/{}/{:04}/{:02}/{:02}/{}.csv.gz",
            segment(&request.exchange),
            segment(&request.data_type),
            date.year(),
            u8::from(date.month()),
            date.day(),
            segment(&request.symbol)
        );
        let mut wire = self
            .datasets
            .request(HttpMethod::Get, &path, &Params::new(), true)?;
        if let Some(range) = request.range {
            let valid = range
                .strip_prefix("bytes=")
                .and_then(|s| s.split_once('-'))
                .is_some_and(|(start, end)| {
                    (!start.is_empty() || !end.is_empty())
                        && (start.is_empty() || start.parse::<u64>().is_ok())
                        && (end.is_empty() || end.parse::<u64>().is_ok())
                        && (start.is_empty()
                            || end.is_empty()
                            || start.parse::<u64>().ok() <= end.parse::<u64>().ok())
                });
            if !valid {
                return Err(Error::InvalidRequest(
                    "dataset range requires one native bytes=start-end range",
                ));
            }
            wire.headers.push(("range".into(), range));
        }
        // Files are returned verbatim, including their gzip container.
        self.datasets.open(wire, false).await
    }
    #[cfg(feature = "files")]
    pub async fn download_dataset(
        &self,
        request: DatasetRequest,
        path: impl AsRef<std::path::Path>,
    ) -> Result<u64> {
        aux_rs_data::download_to(self.dataset(request).await?, path).await
    }
}
fn parse_minute(value: &str) -> Result<OffsetDateTime> {
    let parsed = if value.len() == 10 {
        let fmt = time::format_description::parse_borrowed::<2>("[year]-[month]-[day]")
            .map_err(|_| Error::InvalidRequest("invalid date format"))?;
        Date::parse(value, &fmt)
            .map(|d| PrimitiveDateTime::new(d, time::Time::MIDNIGHT).assume_utc())
    } else {
        OffsetDateTime::parse(value, &Rfc3339)
    }
    .map_err(|_| Error::InvalidRequest("replay dates require ISO UTC minute boundaries"))?;
    if parsed.second() != 0 || parsed.nanosecond() != 0 || parsed.offset() != UtcOffset::UTC {
        return Err(Error::InvalidRequest(
            "replay dates require ISO UTC minute boundaries",
        ));
    }
    Ok(parsed)
}
/// Date-range replay preserves capture ordering. Empty minutes advance normally.
pub struct Replay<T> {
    client: TardisClient<T>,
    feed: FeedRequest,
    remaining: u64,
    current: Option<FeedStream>,
    done: bool,
    queued: std::collections::VecDeque<Result<Value>>,
}
impl<T: StreamingHttpTransport + Clone> Replay<T> {
    async fn read_one(&mut self) -> Option<Result<Value>> {
        if self.done {
            return None;
        }
        loop {
            if let Some(stream) = &mut self.current {
                match stream.next().await {
                    Some(Err(e)) => {
                        self.done = true;
                        return Some(Err(e));
                    }
                    Some(v) => return Some(v),
                    None => {
                        self.current = None;
                        self.feed.offset += 1;
                        self.remaining -= 1;
                    }
                }
            }
            if self.remaining == 0 {
                self.done = true;
                return None;
            }
            match self.client.data_feeds(self.feed.clone()).await {
                Ok(stream) => self.current = Some(stream),
                Err(e) => {
                    self.done = true;
                    return Some(Err(e));
                }
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
        aux_rs_data::check_batch_limit(limit)?;
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
        if let Some(stream) = &mut self.current {
            stream.close().await?;
        }
        self.current = None;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct TardisMachineClient<T = DefaultTransport> {
    http: HttpClient<T>,
    ws_url: Option<url::Url>,
}
impl<T> TardisMachineClient<T> {
    pub fn with_ws_url(mut self, url: &str) -> Result<Self> {
        let url = url::Url::parse(&format!("{}/", url.trim_end_matches('/')))
            .map_err(|_| Error::InvalidRequest("invalid Machine WebSocket URL"))?;
        if !matches!(url.scheme(), "ws" | "wss")
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(Error::InvalidRequest(
                "Machine WebSocket URL requires WS(S) without credentials, query or fragment",
            ));
        }
        self.ws_url = Some(url);
        Ok(self)
    }

    pub fn with_transport(url: &str, transport: T) -> Result<Self> {
        Ok(Self {
            http: HttpClient::new(url, transport)?,
            ws_url: None,
        })
    }
    pub fn transport(&self) -> &T {
        &self.http.transport
    }
    fn options_request(&self, path: &str, options: Value) -> Result<aux_rs_core::HttpRequest> {
        if !options.is_object() && !options.is_array() {
            return Err(Error::InvalidRequest(
                "Machine options require an object or array",
            ));
        }
        self.http.request(
            HttpMethod::Get,
            path,
            &params(json!({"options":options}))?,
            true,
        )
    }
}
impl<T: StreamingHttpTransport> TardisMachineClient<T> {
    pub async fn replay(&self, options: Value) -> Result<FeedStream> {
        let wire = self.options_request("replay", options)?;
        Ok(FeedStream::new(self.http.open(wire, true).await?, false))
    }
    pub async fn replay_normalized(&self, options: Value) -> Result<FeedStream> {
        let wire = self.options_request("replay-normalized", options)?;
        Ok(FeedStream::new(self.http.open(wire, true).await?, false))
    }
}
impl<T: WebSocketConnector> TardisMachineClient<T> {
    async fn socket(
        &self,
        path: &str,
        parameters: Params,
        messages: Vec<Value>,
    ) -> Result<MachineStream<T::Connection>> {
        let wire = self
            .http
            .request(HttpMethod::Get, path, &parameters, true)?;
        let mut url =
            url::Url::parse(&wire.url).map_err(|_| Error::InvalidRequest("invalid Machine URL"))?;
        let scheme = if url.scheme() == "https" { "wss" } else { "ws" };
        url.set_scheme(scheme)
            .map_err(|_| Error::InvalidRequest("invalid Machine scheme"))?;
        if let Some(base) = &self.ws_url {
            let mut target = base
                .join(path)
                .map_err(|_| Error::InvalidRequest("invalid Machine WebSocket path"))?;
            target.set_query(url.query());
            url = target;
        }
        let mut connection = self.http.transport.connect_read(url.as_str()).await?;
        for message in messages {
            connection_send(&mut connection, message.to_string()).await?;
        }
        Ok(MachineStream::new(connection))
    }
    pub async fn ws_replay(
        &self,
        exchange: &str,
        from: &str,
        to: &str,
        subscribe_messages: Vec<Value>,
        session: Option<&str>,
    ) -> Result<MachineStream<T::Connection>> {
        self.socket(
            "ws-replay",
            params(json!({"exchange":exchange,"from":from,"to":to,"session":session}))?,
            subscribe_messages,
        )
        .await
    }
    pub async fn ws_replay_normalized(
        &self,
        options: Value,
    ) -> Result<MachineStream<T::Connection>> {
        let wire = self.options_request("ws-replay-normalized", options)?;
        self.socket_from_wire(wire).await
    }
    pub async fn stream_normalized(&self, options: Value) -> Result<MachineStream<T::Connection>> {
        let wire = self.options_request("ws-stream-normalized", options)?;
        self.socket_from_wire(wire).await
    }
    async fn socket_from_wire(
        &self,
        wire: aux_rs_core::HttpRequest,
    ) -> Result<MachineStream<T::Connection>> {
        let url =
            url::Url::parse(&wire.url).map_err(|_| Error::InvalidRequest("invalid Machine URL"))?;
        let parameters = url
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), Value::String(v.into_owned())))
            .collect();
        let path = url.path().rsplit('/').next().unwrap_or_default();
        self.socket(path, parameters, Vec::new()).await
    }
}
async fn connection_send<C: aux_rs_core::WebSocketConnection>(
    connection: &mut C,
    text: String,
) -> Result<()> {
    connection.send(text).await?;
    Ok(())
}
#[cfg(any(feature = "http", feature = "ws"))]
impl TardisClient<aux_rs_core::StandardTransport> {
    pub fn new() -> Result<Self> {
        Self::with_config(aux_rs_core::TransportConfig::default())
    }
    pub fn with_config(config: aux_rs_core::TransportConfig) -> Result<Self> {
        Self::with_transport(
            "https://api.tardis.dev/v1",
            "https://datasets.tardis.dev/v1",
            aux_rs_core::StandardTransport::new(config)?,
        )
    }
}
#[cfg(any(feature = "http", feature = "ws"))]
impl TardisMachineClient<aux_rs_core::StandardTransport> {
    pub fn new(url: &str) -> Result<Self> {
        Self::with_transport(
            url,
            aux_rs_core::StandardTransport::new(aux_rs_core::TransportConfig::default())?,
        )
    }
}
