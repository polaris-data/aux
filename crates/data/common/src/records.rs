//! Bounded HTTP record framing and cancellation-safe native JSON subscriptions.
use crate::{DecodedBody, Error, MAX_RECORD_BYTES, Result};
use aux_rs_core::{RequestOutcome, TransportError, TransportErrorKind, WebSocketStream};
use futures_util::{
    FutureExt,
    future::{BoxFuture, select_all},
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{HashSet, VecDeque};

trait Records: Send {
    fn next(&mut self) -> BoxFuture<'_, Option<Result<Value>>>;
    fn close(&mut self) -> BoxFuture<'_, Result<()>>;
}
struct Ws<S>(S);
impl<S: WebSocketStream> Records for Ws<S> {
    fn next(&mut self) -> BoxFuture<'_, Option<Result<Value>>> {
        async move {
            self.0.next().await.map(|r| {
                r.map_err(Error::from).and_then(|s| {
                    let value: Value = serde_json::from_str(&s)
                        .map_err(|_| Error::Decode("invalid WebSocket JSON"))?;
                    if !value.is_object() {
                        return Err(Error::Decode("expected a native event object"));
                    }
                    Ok(value)
                })
            })
        }
        .boxed()
    }
    fn close(&mut self) -> BoxFuture<'_, Result<()>> {
        async move { self.0.close().await.map_err(Error::from) }.boxed()
    }
}
/// Framing mode is protocol-specific; HTTP chunks are never assumed to be records.
#[derive(Clone, Copy)]
pub enum JsonFraming {
    Sse,
    Objects,
}
struct HttpRecords {
    body: DecodedBody,
    framing: JsonFraming,
    pending: Vec<u8>,
    event: Vec<u8>,
    first: bool,
    object: ObjectBoundary,
    skip_lf: bool,
}
impl HttpRecords {
    fn take_record(&mut self) -> Result<Option<Value>> {
        match self.framing {
            JsonFraming::Objects => self.take_object(),
            JsonFraming::Sse => loop {
                if self.skip_lf && !self.pending.is_empty() {
                    self.skip_lf = false;
                    if self.pending[0] == b'\n' {
                        self.pending.remove(0);
                    }
                }
                let Some(pos) = self.pending.iter().position(|b| matches!(b, b'\r' | b'\n')) else {
                    return Ok(None);
                };
                let mut line: Vec<u8> = self.pending.drain(..=pos).collect();
                self.skip_lf = line.pop() == Some(b'\r');
                if self.first {
                    self.first = false;
                    if line.starts_with(&[0xef, 0xbb, 0xbf]) {
                        line.drain(..3);
                    }
                }
                if line.is_empty() {
                    if self.event.is_empty() {
                        continue;
                    }
                    self.event.pop();
                    let value = serde_json::from_slice(&self.event)
                        .map_err(|_| Error::Decode("invalid SSE JSON event"))?;
                    self.event.clear();
                    return Ok(Some(value));
                }
                if let Some(mut data) = line.strip_prefix(b"data:") {
                    if data.first() == Some(&b' ') {
                        data = &data[1..];
                    }
                    if self.event.len() + data.len() + 1 > MAX_RECORD_BYTES {
                        return Err(Error::Decode("SSE event exceeded configured limit"));
                    }
                    self.event.extend_from_slice(data);
                    self.event.push(b'\n');
                }
                // Comments, event/id/retry fields are native framing, not JSON records.
            },
        }
    }
}
#[derive(Default)]
struct ObjectBoundary {
    offset: usize,
    depth: usize,
    in_string: bool,
    escape: bool,
}
impl HttpRecords {
    // Scan incrementally so fragmented large records are parsed only once.
    fn take_object(&mut self) -> Result<Option<Value>> {
        let scan = &mut self.object;
        for (offset, byte) in self.pending.iter().copied().enumerate().skip(scan.offset) {
            scan.offset = offset + 1;
            if scan.depth == 0 {
                if byte.is_ascii_whitespace() {
                    continue;
                }
                if byte != b'{' {
                    return Err(Error::Decode("expected a JSON stream object"));
                }
                scan.depth = 1;
            } else if scan.in_string {
                if scan.escape {
                    scan.escape = false;
                } else if byte == b'\\' {
                    scan.escape = true;
                } else if byte == b'"' {
                    scan.in_string = false;
                }
            } else {
                match byte {
                    b'"' => scan.in_string = true,
                    b'{' | b'[' => scan.depth += 1,
                    b'}' | b']' => scan.depth -= 1,
                    _ => {}
                }
                if scan.depth == 0 {
                    let value = serde_json::from_slice(&self.pending[..scan.offset])
                        .map_err(|_| Error::Decode("invalid JSON stream object"))?;
                    self.pending.drain(..scan.offset);
                    *scan = ObjectBoundary::default();
                    return Ok(Some(value));
                }
            }
        }
        if scan.depth == 0 {
            self.pending.clear();
            scan.offset = 0;
        }
        Ok(None)
    }
}
impl Records for HttpRecords {
    fn next(&mut self) -> BoxFuture<'_, Option<Result<Value>>> {
        async move {
            loop {
                match self.take_record() {
                    Ok(Some(value)) => return Some(Ok(value)),
                    Err(e) => return Some(Err(e)),
                    _ => {}
                }
                match self.body.next().await {
                    Some(Ok(bytes)) => {
                        if self.pending.len() + bytes.len() > MAX_RECORD_BYTES {
                            return Some(Err(Error::Decode(
                                "stream record exceeded configured limit",
                            )));
                        }
                        self.pending.extend(bytes);
                    }
                    Some(Err(e)) => return Some(Err(e)),
                    None => {
                        if !self.event.is_empty()
                            || self.pending.iter().any(|byte| !byte.is_ascii_whitespace())
                        {
                            return Some(Err(Error::Decode("truncated HTTP stream record")));
                        }
                        return Some(Err(Error::Transport(TransportError::new(
                            TransportErrorKind::Disconnected,
                            RequestOutcome::Unknown,
                            "HTTP data stream ended; reopen explicitly",
                        ))));
                    }
                }
            }
        }
        .boxed()
    }
    fn close(&mut self) -> BoxFuture<'_, Result<()>> {
        async move { self.body.close().await }.boxed()
    }
}
/// Owned subscription. No background readers; cancelling next() retains framing state.
pub struct JsonStream {
    sources: Vec<Box<dyn Records>>,
    healthy: Vec<bool>,
    seen: HashSet<[u8; 32]>,
    order: VecDeque<[u8; 32]>,
    deduplicate: bool,
    has_redundancy: bool,
    done: bool,
    next_source: usize,
}
impl JsonStream {
    pub fn http(body: DecodedBody, framing: JsonFraming) -> Self {
        Self::new(
            vec![Box::new(HttpRecords {
                body,
                framing,
                pending: Vec::new(),
                event: Vec::new(),
                first: true,
                object: ObjectBoundary::default(),
                skip_lf: false,
            })],
            false,
        )
    }
    pub fn websockets<S: WebSocketStream + 'static>(sources: Vec<S>, deduplicate: bool) -> Self {
        Self::new(
            sources
                .into_iter()
                .map(|s| Box::new(Ws(s)) as Box<dyn Records>)
                .collect(),
            deduplicate,
        )
    }
    fn new(sources: Vec<Box<dyn Records>>, deduplicate: bool) -> Self {
        let has_redundancy = sources.len() > 1;
        let healthy = vec![true; sources.len()];
        Self {
            sources,
            healthy,
            seen: HashSet::new(),
            order: VecDeque::new(),
            deduplicate,
            has_redundancy,
            done: false,
            next_source: 0,
        }
    }
    pub async fn next(&mut self) -> Option<Result<Value>> {
        if self.done {
            return None;
        }
        while !self.sources.is_empty() {
            let start = self.next_source % self.sources.len();
            let (before, after) = self.sources.split_at_mut(start);
            let reads: Vec<_> = after
                .iter_mut()
                .chain(before)
                .map(|source| source.next())
                .collect();
            let (event, offset, remaining) = select_all(reads).await;
            drop(remaining);
            let index = (start + offset) % self.sources.len();
            self.next_source = index + 1;
            match event {
                Some(Ok(value)) => {
                    let price_update =
                        value.get("type").and_then(Value::as_str) == Some("streamUpdated");
                    if !self.deduplicate || price_update {
                        self.healthy[index] = true;
                    }
                    if self.deduplicate && price_update {
                        let hash: [u8; 32] = Sha256::digest(value.to_string().as_bytes()).into();
                        if !self.seen.insert(hash) {
                            continue;
                        }
                        self.order.push_back(hash);
                        if self.order.len() > 4096 {
                            self.seen.remove(&self.order.pop_front().expect("nonempty"));
                        }
                    }
                    return Some(Ok(value));
                }
                Some(Err(Error::Transport(e))) if e.is_gap() => {
                    self.healthy[index] = false;
                    if !self.healthy.iter().any(|h| *h) {
                        return Some(Err(Error::Transport(e)));
                    }
                }
                Some(Err(Error::Transport(e)))
                    if self.has_redundancy
                        && self.sources.len() > 1
                        && matches!(
                            e.kind,
                            TransportErrorKind::Disconnected | TransportErrorKind::Timeout
                        ) =>
                {
                    let was_healthy = self.healthy.remove(index);
                    self.sources.remove(index);
                    if was_healthy && !self.healthy.iter().any(|h| *h) {
                        return Some(Err(Error::Transport(TransportError::new(
                            TransportErrorKind::Gap,
                            RequestOutcome::Unknown,
                            "all redundant sources lost continuity",
                        ))));
                    }
                }
                Some(Err(e)) => {
                    let _ = self.close().await;
                    return Some(Err(e));
                }
                None => {
                    let mut source = self.sources.remove(index);
                    let _ = source.close().await;
                    self.healthy.remove(index);
                    if self.sources.is_empty() {
                        let _ = self.close().await;
                        return Some(Err(Error::Transport(TransportError::new(
                            TransportErrorKind::Disconnected,
                            RequestOutcome::Unknown,
                            "all data WebSockets ended",
                        ))));
                    }
                }
            }
        }
        self.done = true;
        None
    }
    pub async fn close(&mut self) -> Result<()> {
        self.done = true;
        let mut first = None;
        for source in &mut self.sources {
            if let Err(e) = source.close().await {
                first.get_or_insert(e);
            }
        }
        self.sources.clear();
        self.healthy.clear();
        self.order.clear();
        self.seen.clear();
        first.map_or(Ok(()), Err)
    }
}
