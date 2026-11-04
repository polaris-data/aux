use aux_rs_core::WebSocketConnection;
use aux_rs_data::{DecodedBody, Error, Lines, Result, check_batch_limit};
use serde_json::{Value, json};
use std::collections::VecDeque;

/// NDJSON events; timestamp-prefixed hosted records become native replay
/// envelopes. Recorded empty lines are explicit disconnect events.
pub struct FeedStream {
    lines: Lines,
    raw: bool,
    queued: VecDeque<Result<Value>>,
    done: bool,
}
impl FeedStream {
    pub fn new(body: DecodedBody, raw: bool) -> Self {
        Self {
            lines: Lines::new(body),
            raw,
            queued: VecDeque::new(),
            done: false,
        }
    }
    async fn read_one(&mut self) -> Option<Result<Value>> {
        if self.done {
            return None;
        }
        let line = match self.lines.next().await {
            None => {
                self.done = true;
                return None;
            }
            Some(Err(e)) => {
                self.done = true;
                return Some(Err(e));
            }
            Some(Ok(v)) => v,
        };
        let result = if line.is_empty() {
            Ok(json!({"type":"disconnect"}))
        } else if self.raw {
            match line.iter().position(|b| *b == b' ') {
                Some(pos) => String::from_utf8(line[..pos].to_vec())
                    .map_err(|_| Error::Decode("invalid local timestamp"))
                    .and_then(|ts| {
                        serde_json::from_slice::<Value>(&line[pos + 1..])
                            .map(|message| json!({"localTimestamp":ts,"message":message}))
                            .map_err(|_| Error::Decode("invalid raw feed JSON"))
                    }),
                None => Err(Error::Decode("raw feed record has no local timestamp")),
            }
        } else {
            serde_json::from_slice(&line).map_err(|_| Error::Decode("invalid feed JSON"))
        };
        if result.is_err() {
            self.done = true;
            self.lines.stop();
        }
        Some(result)
    }
    pub async fn next(&mut self) -> Option<Result<Value>> {
        if let Some(item) = self.queued.pop_front() {
            Some(item)
        } else {
            self.read_one().await
        }
    }
    /// Cancelled batch reads retain already decoded records for the next call.
    pub async fn read_batch(&mut self, limit: usize) -> Result<Vec<Value>> {
        check_batch_limit(limit)?;
        while self.queued.len() < limit && !self.queued.back().is_some_and(Result::is_err) {
            match self.read_one().await {
                Some(item) => self.queued.push_back(item),
                None => break,
            }
        }
        let mut values = Vec::new();
        while values.len() < limit {
            if self.queued.front().is_some_and(Result::is_err) {
                if values.is_empty() {
                    return Err(self
                        .queued
                        .pop_front()
                        .expect("front exists")
                        .expect_err("error checked"));
                }
                break;
            }
            match self.queued.pop_front() {
                Some(Ok(v)) => values.push(v),
                _ => break,
            }
        }
        Ok(values)
    }
    pub async fn close(&mut self) -> Result<()> {
        self.done = true;
        self.queued.clear();
        self.lines.close().await
    }
}
/// Dedicated Machine socket: no pooling, reconnect or implicit replay.
pub struct MachineStream<C> {
    connection: C,
    done: bool,
    queued: VecDeque<Result<Value>>,
}
impl<C: WebSocketConnection> MachineStream<C> {
    pub(crate) fn new(connection: C) -> Self {
        Self {
            connection,
            done: false,
            queued: VecDeque::new(),
        }
    }
    async fn read_one(&mut self) -> Option<Result<Value>> {
        if self.done {
            return None;
        }
        match self.connection.next().await {
            None => {
                self.done = true;
                None
            }
            Some(Err(e)) => {
                self.done = true;
                Some(Err(e.into()))
            }
            Some(Ok(text)) => {
                let value = serde_json::from_str(&text)
                    .map_err(|_| Error::Decode("invalid Machine WebSocket JSON"));
                if value.is_err() {
                    self.done = true;
                    let _ = self.connection.close().await;
                }
                Some(value)
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
    pub async fn send(&mut self, message: Value) -> Result<()> {
        if self.done {
            return Err(Error::InvalidRequest("Machine stream is closed"));
        }
        self.connection.send(message.to_string()).await?;
        Ok(())
    }
    pub async fn close(&mut self) -> Result<()> {
        self.done = true;
        self.queued.clear();
        self.connection.close().await?;
        Ok(())
    }
}
