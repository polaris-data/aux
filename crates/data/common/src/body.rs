use crate::{Error, Result};
use aux_rs_core::{ByteStream, TransportError};
use futures_util::{
    TryStreamExt,
    io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader},
};
use std::{io, pin::Pin};

pub const MAX_RECORD_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_BATCH_RECORDS: usize = 4096;
/// Decompression uses a fixed output chunk and reads all gzip/zstd members.
/// The owned reader retains partial decoder state across cancelled polls.
pub struct DecodedBody {
    reader: Pin<Box<dyn AsyncRead + Send>>,
    closed: bool,
}
impl DecodedBody {
    pub fn new<B: ByteStream + 'static>(body: B, encoding: &str) -> Result<Self> {
        let stream = futures_util::stream::unfold(body, |mut body| async move {
            body.next()
                .await
                .map(|item| (item.map_err(io::Error::other), body))
        });
        let reader = BufReader::new(Box::pin(stream).into_async_read());
        let reader: Pin<Box<dyn AsyncRead + Send>> = match encoding.trim() {
            "" | "identity" | "none" => Box::pin(reader),
            "gzip" => {
                let mut decoder = async_compression::futures::bufread::GzipDecoder::new(reader);
                decoder.multiple_members(true);
                Box::pin(decoder)
            }
            "zstd" => {
                let mut decoder = async_compression::futures::bufread::ZstdDecoder::new(reader);
                decoder.multiple_members(true);
                Box::pin(decoder)
            }
            _ => return Err(Error::Decode("unsupported content encoding")),
        };
        Ok(Self {
            reader,
            closed: false,
        })
    }
    pub fn reader(&mut self) -> &mut Pin<Box<dyn AsyncRead + Send>> {
        &mut self.reader
    }
    pub fn stop(&mut self) {
        self.closed = true;
        self.reader = Box::pin(futures_util::io::empty());
    }
    pub async fn next(&mut self) -> Option<Result<Vec<u8>>> {
        if self.closed {
            return None;
        }
        let mut bytes = vec![0; 64 * 1024];
        match self.reader.read(&mut bytes).await {
            Ok(0) => {
                self.stop();
                None
            }
            Ok(n) => {
                bytes.truncate(n);
                Some(Ok(bytes))
            }
            Err(e) => {
                self.stop();
                Some(Err(e.into()))
            }
        }
    }
    pub async fn close(&mut self) -> Result<()> {
        self.stop();
        Ok(())
    }
    pub async fn collect(mut self, limit: usize) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        while let Some(chunk) = self.next().await {
            let chunk = chunk?;
            if chunk.len() > limit.saturating_sub(bytes.len()) {
                return Err(Error::Decode("response exceeded configured limit"));
            }
            bytes.extend(chunk);
        }
        Ok(bytes)
    }
}
/// Bounded line parsing preserves blank lines and final unterminated lines.
/// Partial input is owned by this object, so cancellation cannot discard bytes.
pub struct Lines {
    reader: BufReader<Pin<Box<dyn AsyncRead + Send>>>,
    pending: Vec<u8>,
    done: bool,
}
impl Lines {
    pub fn new(body: DecodedBody) -> Self {
        Self {
            reader: BufReader::new(body.reader),
            pending: Vec::new(),
            done: false,
        }
    }
    pub async fn next(&mut self) -> Option<Result<Vec<u8>>> {
        if self.done {
            return None;
        }
        loop {
            let available = match self.reader.fill_buf().await {
                Ok(v) => v,
                Err(e) => {
                    self.stop();
                    return Some(Err(e.into()));
                }
            };
            if available.is_empty() {
                self.done = true;
                return if self.pending.is_empty() {
                    None
                } else {
                    Some(Ok(std::mem::take(&mut self.pending)))
                };
            }
            let newline = available.iter().position(|b| *b == b'\n');
            let n = newline.map_or(available.len(), |n| n + 1);
            if n > MAX_RECORD_BYTES.saturating_sub(self.pending.len()) {
                self.stop();
                return Some(Err(Error::Decode("line exceeded record limit")));
            }
            self.pending.extend_from_slice(&available[..n]);
            self.reader.consume_unpin(n);
            if newline.is_some() {
                self.pending.pop();
                if self.pending.last() == Some(&b'\r') {
                    self.pending.pop();
                }
                return Some(Ok(std::mem::take(&mut self.pending)));
            }
        }
    }
    pub fn stop(&mut self) {
        self.done = true;
        self.pending.clear();
        self.reader = BufReader::new(Box::pin(futures_util::io::empty()));
    }
    pub async fn close(&mut self) -> Result<()> {
        self.stop();
        Ok(())
    }
}
/// Adapter for callers with an owned in-memory fixture or preloaded response.
pub struct BytesBody(pub Option<Vec<u8>>);
impl ByteStream for BytesBody {
    async fn next(&mut self) -> Option<std::result::Result<Vec<u8>, TransportError>> {
        self.0.take().map(Ok)
    }
    async fn close(&mut self) -> std::result::Result<(), TransportError> {
        self.0 = None;
        Ok(())
    }
}
pub fn check_batch_limit(limit: usize) -> Result<()> {
    if limit == 0 || limit > MAX_BATCH_RECORDS {
        Err(Error::InvalidRequest(
            "batch size must be between 1 and 4096",
        ))
    } else {
        Ok(())
    }
}
