use crate::{DbnEvent, DbnStream};
use aux_rs_core::{ByteStream, TransportError};
use aux_rs_data::{DecodedBody, Error, MAX_RECORD_BYTES, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{collections::VecDeque, future::Future, sync::Arc};
use zeroize::Zeroizing;

pub(crate) fn validate_key(key: &str) -> Result<()> {
    if key.len() != 32 || !key.bytes().all(|b| b.is_ascii_graphic() && b != b'|') {
        return Err(Error::InvalidRequest(
            "Databento API key requires 32 printable ASCII characters",
        ));
    }
    Ok(())
}
#[derive(Clone)]
struct ApiKey(Arc<Zeroizing<String>>);
impl std::fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ApiKey([redacted])")
    }
}
/// Writes may be partially delivered if cancelled. Session methods quarantine
/// their writer before awaiting, and forbid subsequent commands after cancellation.
pub trait ByteWriter: Send {
    fn write(
        &mut self,
        bytes: &[u8],
    ) -> impl Future<Output = std::result::Result<(), TransportError>> + Send;
    fn close(&mut self) -> impl Future<Output = std::result::Result<(), TransportError>> + Send;
}
/// Dedicated, ordered connection with independent reading and writing. Readers
/// must release their connection on drop and provide bounded, cancellation-safe reads.
pub trait ByteConnector: Send + Sync {
    type Reader: ByteStream + 'static;
    type Writer: ByteWriter;
    fn connect(
        &self,
        gateway: &str,
    ) -> impl Future<Output = std::result::Result<(Self::Reader, Self::Writer), TransportError>> + Send;
}
pub type DefaultConnector = crate::TcpConnector;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionOptions {
    pub dataset: String,
    #[serde(default)]
    pub gateway: Option<String>,
    #[serde(default = "none")]
    pub compression: String,
    #[serde(default)]
    pub ts_out: bool,
    #[serde(default)]
    pub heartbeat_interval_s: Option<u16>,
    #[serde(default)]
    pub slow_reader_behavior: Option<String>,
}
fn none() -> String {
    "none".into()
}
impl SessionOptions {
    pub fn new(dataset: impl Into<String>) -> Self {
        Self {
            dataset: dataset.into(),
            gateway: None,
            compression: none(),
            ts_out: false,
            heartbeat_interval_s: None,
            slow_reader_behavior: None,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub schema: String,
    #[serde(default = "raw_symbol")]
    pub stype_in: String,
    pub symbols: Vec<String>,
    /// Native Unix nanoseconds, retained as an exact decimal string.
    #[serde(default)]
    pub start: Option<String>,
    #[serde(default)]
    pub snapshot: bool,
    #[serde(default)]
    pub id: Option<u32>,
}
fn raw_symbol() -> String {
    "raw_symbol".into()
}
#[derive(Clone, Debug)]
pub struct LiveClient<C = DefaultConnector> {
    key: ApiKey,
    connector: C,
}
impl<C> LiveClient<C> {
    pub fn with_connector(api_key: impl Into<String>, connector: C) -> Result<Self> {
        let key = api_key.into();
        validate_key(&key)?;
        Ok(Self {
            key: ApiKey(Arc::new(Zeroizing::new(key))),
            connector,
        })
    }
}
impl LiveClient<crate::TcpConnector> {
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::with_connector(api_key, crate::TcpConnector::default())
    }
}
impl<C: ByteConnector> LiveClient<C> {
    pub async fn connect(
        &self,
        options: SessionOptions,
    ) -> Result<LiveSession<C::Reader, C::Writer>> {
        field(&options.dataset)?;
        if !matches!(options.compression.as_str(), "none" | "zstd") {
            return Err(Error::InvalidRequest(
                "live compression must be none or zstd",
            ));
        }
        if options.heartbeat_interval_s == Some(0) {
            return Err(Error::InvalidRequest("heartbeat interval must be positive"));
        }
        if let Some(value) = &options.slow_reader_behavior
            && !matches!(value.as_str(), "warn" | "skip")
        {
            return Err(Error::InvalidRequest("invalid slow reader behavior"));
        }
        let gateway = options.gateway.clone().unwrap_or_else(|| {
            format!(
                "{}.lsg.databento.com:13000",
                options.dataset.replace('.', "-").to_ascii_lowercase()
            )
        });
        let (reader, mut writer) = self.connector.connect(&gateway).await?;
        let mut reader = Handshake {
            inner: reader,
            buffer: VecDeque::new(),
        };
        let _greeting = reader.line().await?;
        let challenge = reader.line().await?;
        let challenge = challenge
            .strip_prefix("cram=")
            .ok_or(Error::Decode("gateway did not send CRAM challenge"))?;
        field(challenge)?;
        let mut hash = Sha256::new();
        hash.update(challenge.as_bytes());
        hash.update(b"|");
        hash.update(self.key.0.as_bytes());
        let digest = hash.finalize();
        let response = digest
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        let key = self.key.0.as_str();
        let mut auth = format!(
            "auth={response}-{}|dataset={}|encoding=dbn|compression={}|ts_out={}|client=aux-rs/{}",
            &key[27..],
            options.dataset,
            options.compression,
            u8::from(options.ts_out),
            env!("CARGO_PKG_VERSION")
        );
        if let Some(seconds) = options.heartbeat_interval_s {
            auth.push_str(&format!("|heartbeat_interval_s={seconds}"));
        }
        if let Some(behavior) = &options.slow_reader_behavior {
            auth.push_str(&format!("|slow_reader_behavior={behavior}"));
        }
        auth.push('\n');
        writer.write(auth.as_bytes()).await?;
        let reply = reader.line().await?;
        let fields = reply
            .split('|')
            .filter_map(|s| s.split_once('='))
            .collect::<std::collections::HashMap<_, _>>();
        if fields.get("success") != Some(&"1") {
            let body =
                json!({"response":reply.replace(key,"[redacted]").replace(&response,"[redacted]")});
            return Err(Error::Authentication { body });
        }
        let session_id = fields
            .get("session_id")
            .copied()
            .unwrap_or_default()
            .to_owned();
        Ok(LiveSession {
            reader: Some(reader),
            writer,
            stream: None,
            compression: options.compression,
            poisoned: false,
            closed: false,
            session_id,
        })
    }
}
fn field(value: &str) -> Result<()> {
    if value.is_empty()
        || value
            .bytes()
            .any(|b| b == b'|' || b == b'\n' || b == b'\r' || b == 0)
    {
        return Err(Error::InvalidRequest("invalid live protocol field"));
    }
    Ok(())
}
struct Handshake<B> {
    inner: B,
    buffer: VecDeque<u8>,
}
impl<B: ByteStream> Handshake<B> {
    async fn line(&mut self) -> Result<String> {
        loop {
            if let Some(index) = self.buffer.iter().position(|b| *b == b'\n') {
                if index > 16 * 1024 {
                    return Err(Error::Decode("gateway control line exceeded limit"));
                }
                let bytes = self.buffer.drain(..=index).collect::<Vec<_>>();
                return String::from_utf8(bytes[..bytes.len() - 1].to_vec())
                    .map(|s| s.trim_end_matches('\r').to_owned())
                    .map_err(|_| Error::Decode("invalid gateway control text"));
            }
            if self.buffer.len() > 16 * 1024 {
                return Err(Error::Decode("gateway control line exceeded limit"));
            }
            match self.inner.next().await {
                Some(Ok(bytes)) => {
                    if bytes.len() > MAX_RECORD_BYTES {
                        return Err(Error::Decode("gateway chunk exceeded limit"));
                    }
                    self.buffer.extend(bytes)
                }
                Some(Err(e)) => return Err(e.into()),
                None => return Err(Error::Decode("gateway closed during authentication")),
            }
        }
    }
}
impl<B: ByteStream> ByteStream for Handshake<B> {
    async fn next(&mut self) -> Option<std::result::Result<Vec<u8>, TransportError>> {
        if !self.buffer.is_empty() {
            Some(Ok(self.buffer.drain(..).collect()))
        } else {
            self.inner.next().await
        }
    }
    async fn close(&mut self) -> std::result::Result<(), TransportError> {
        self.buffer.clear();
        self.inner.close().await
    }
}
pub struct LiveSession<R, W> {
    reader: Option<Handshake<R>>,
    writer: W,
    stream: Option<DbnStream>,
    compression: String,
    poisoned: bool,
    closed: bool,
    pub session_id: String,
}
impl<R: ByteStream + 'static, W: ByteWriter> LiveSession<R, W> {
    pub fn closed(&self) -> bool {
        self.closed || self.poisoned
    }
    async fn command(&mut self, text: &str) -> Result<()> {
        if self.closed || self.poisoned {
            return Err(Error::InvalidRequest(
                "live session is closed or has an interrupted command",
            ));
        }
        self.poisoned = true;
        self.writer.write(text.as_bytes()).await?;
        self.poisoned = false;
        Ok(())
    }
    pub async fn subscribe(&mut self, subscription: Subscription) -> Result<()> {
        self.subscription(subscription, false).await
    }
    pub async fn unsubscribe(&mut self, subscription: Subscription) -> Result<()> {
        self.subscription(subscription, true).await
    }
    async fn subscription(&mut self, sub: Subscription, unsubscribe: bool) -> Result<()> {
        field(&sub.schema)?;
        field(&sub.stype_in)?;
        if sub.symbols.is_empty() {
            return Err(Error::InvalidRequest("subscription requires symbols"));
        }
        for symbol in &sub.symbols {
            field(symbol)?;
            if symbol.contains(',') {
                return Err(Error::InvalidRequest(
                    "individual symbols cannot contain commas",
                ));
            }
        }
        if sub.snapshot && sub.start.is_some() {
            return Err(Error::InvalidRequest(
                "snapshot and replay start are mutually exclusive",
            ));
        }
        if sub
            .start
            .as_ref()
            .is_some_and(|s| s.parse::<u64>().is_err())
        {
            return Err(Error::InvalidRequest(
                "start requires unsigned Unix nanoseconds",
            ));
        }
        let mut command = String::new();
        for (index, chunk) in sub.symbols.chunks(500).enumerate() {
            let last = (index + 1) * 500 >= sub.symbols.len();
            if unsubscribe {
                command.push_str("unsubscribe|");
            }
            command.push_str(&format!(
                "schema={}|stype_in={}|symbols={}|",
                sub.schema,
                sub.stype_in,
                chunk.join(",")
            ));
            if !unsubscribe {
                command.push_str(&format!("snapshot={}|", u8::from(sub.snapshot)));
            }
            command.push_str(&format!("is_last={}", u8::from(last)));
            if !unsubscribe {
                if let Some(start) = &sub.start {
                    command.push_str(&format!("|start={start}"));
                }
                if let Some(id) = sub.id {
                    command.push_str(&format!("|id={id}"));
                }
            }
            command.push('\n');
        }
        self.command(&command).await
    }
    pub async fn start_session(&mut self) -> Result<()> {
        if self.stream.is_some() {
            return Err(Error::InvalidRequest("session already started"));
        }
        self.command("start_session\n").await?;
        let reader = self
            .reader
            .take()
            .ok_or(Error::InvalidRequest("live reader is closed"))?;
        self.stream = Some(DbnStream::new(DecodedBody::new(reader, &self.compression)?));
        Ok(())
    }
    pub fn metadata(&self) -> Option<&dbn::Metadata> {
        self.stream.as_ref().and_then(DbnStream::metadata)
    }
    pub async fn next(&mut self) -> Option<Result<DbnEvent>> {
        if self.closed || self.poisoned {
            return None;
        }
        let result = match &mut self.stream {
            Some(stream) => stream.next().await,
            None => return Some(Err(Error::InvalidRequest("start session before reading"))),
        };
        if result.is_none() || result.as_ref().is_some_and(Result::is_err) {
            self.closed = true;
            let _ = self.writer.close().await;
        }
        result
    }
    pub async fn read_batch(&mut self, limit: usize) -> Result<Vec<DbnEvent>> {
        if self.closed || self.poisoned {
            return Err(Error::InvalidRequest("live session is closed"));
        }
        let result = self
            .stream
            .as_mut()
            .ok_or(Error::InvalidRequest("start session before reading"))?
            .read_batch(limit)
            .await;
        if result.is_err() || result.as_ref().is_ok_and(|values| values.is_empty()) {
            self.closed = true;
        }
        result
    }
    pub async fn close(&mut self) -> Result<()> {
        self.closed = true;
        if let Some(reader) = &mut self.reader {
            reader.close().await?;
        }
        self.reader = None;
        if let Some(stream) = &mut self.stream {
            stream.close().await?;
        }
        self.writer.close().await?;
        Ok(())
    }
}
