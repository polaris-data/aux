//! Databento Live TCP sessions. Reads and subscription changes share one connection.
use crate::{EventStream, Result, encoding, http::call};
use serde_json::{Value, json};

pub struct LiveClient {
    key: String,
    timeout_ms: u64,
    idle_timeout_ms: u64,
}
impl LiveClient {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            timeout_ms: 10000,
            idle_timeout_ms: 60000,
        }
    }
    pub fn with_timeouts(mut self, timeout_ms: u64, idle_timeout_ms: u64) -> Result<Self> {
        if timeout_ms == 0 || idle_timeout_ms == 0 {
            return Err(crate::Error::InvalidRequest(
                "Databento timeouts require positive milliseconds",
            ));
        }
        self.timeout_ms = timeout_ms;
        self.idle_timeout_ms = idle_timeout_ms;
        Ok(self)
    }
    pub fn connect(&self, options: &Value) -> Result<LiveSession> {
        let config = json!({"venue":"databento","operation":"connect","api_key":self.key,"params":options,"timeout_ms":self.timeout_ms,"idle_timeout_ms":self.idle_timeout_ms});
        let stream = EventStream::open_json(&config.to_string())?;
        let id = stream.command_json(r#"{"operation":"session_id"}"#)?;
        let session_id = serde_json::from_str(&id)
            .map_err(|_| crate::Error::Decode("invalid live session ID"))?;
        Ok(LiveSession { stream, session_id })
    }
}
pub struct LiveSession {
    stream: EventStream,
    pub session_id: String,
}
impl LiveSession {
    fn command(&self, input: Value) -> Result<Value> {
        serde_json::from_str(&self.stream.command_json(&input.to_string())?)
            .map_err(|_| crate::Error::Decode("invalid session response"))
    }
    pub fn subscribe(&self, subscription: &Value) -> Result<()> {
        self.command(json!({"operation":"subscribe","subscription":subscription}))
            .map(|_| ())
    }
    pub fn unsubscribe(&self, subscription: &Value) -> Result<()> {
        self.command(json!({"operation":"unsubscribe","subscription":subscription}))
            .map(|_| ())
    }
    pub fn start_session(&self) -> Result<()> {
        self.command(json!({"operation":"start_session"}))
            .map(|_| ())
    }
    pub fn metadata(&self) -> Result<Value> {
        self.command(json!({"operation":"metadata"}))
    }
    pub fn next_event(&self) -> Result<Option<Value>> {
        self.stream.next_event()
    }
    pub fn read_batch(&self, limit: usize) -> Result<Vec<Value>> {
        let raw = self
            .stream
            .command_json(&json!({"operation":"read_batch","limit":limit}).to_string())?;
        serde_json::from_str(&raw).map_err(|_| crate::Error::Decode("invalid DBN batch"))
    }
    pub fn close(&self) {
        self.stream.close();
    }
}

/// Read DBN v1–v3 metadata followed by each record, including from Zstd-compressed files.
pub fn decode_file(input: &[u8]) -> Result<impl Iterator<Item = Result<Value>>> {
    let stream = crate::PriceStream::open_dbn(input)?;
    Ok(std::iter::from_fn(move || stream.next_event().transpose()))
}
/// Decode one complete DBN v3 wire record, including its record header.
pub fn decode_record(record: &[u8], ts_out: bool) -> Result<Value> {
    call(
        18,
        json!({"data":encoding::hex_encode(record),"ts_out":ts_out}),
    )
}

pub struct DataStream(pub(crate) EventStream);
impl DataStream {
    pub fn next_event(&self) -> Result<Option<Value>> {
        self.0.next_event()
    }
    pub fn read_batch(&self, limit: usize) -> Result<Vec<Value>> {
        let raw = self
            .0
            .command_json(&json!({"operation":"read_batch","limit":limit}).to_string())?;
        serde_json::from_str(&raw).map_err(|_| crate::Error::Decode("invalid record batch"))
    }
    pub fn close(&self) {
        self.0.close();
    }
}

pub struct ByteStream(pub(crate) EventStream);
impl ByteStream {
    pub fn next_chunk(&self) -> Result<Option<Vec<u8>>> {
        self.0.next_bytes()
    }
    pub fn close(&self) {
        self.0.close();
    }
}
