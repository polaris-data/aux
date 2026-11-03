use aux_rs_data::{DecodedBody, Error, MAX_RECORD_BYTES, Result, check_batch_limit};
use dbn::{
    decode::dbn::fsm::{DbnFsm, ProcessResult},
    encode::EncodeRecordRef,
};
use futures_util::io::AsyncReadExt;
use serde_json::{Value, json};
use std::collections::VecDeque;

/// Owned native DBN record, including a possible trailing output timestamp.
#[derive(Debug, Clone)]
pub struct DbnRecord {
    pub record: dbn::RecordBuf,
    pub ts_out: Option<u64>,
}
impl DbnRecord {
    pub fn to_json(&self) -> Result<Value> {
        let mut bytes = Vec::new();
        let mut encoder = dbn::encode::json::Encoder::new(&mut bytes, false, false, false);
        let result = if let Some(ts_out) = self.ts_out {
            encoder.encode_record_ref_with_ts_out(self.record.as_rec_ref(), ts_out)
        } else {
            encoder.encode_record_ref(self.record.as_rec_ref())
        };
        result.map_err(|_| Error::Decode("unsupported DBN record layout"))?;
        serde_json::from_slice(&bytes).map_err(|_| Error::Decode("invalid DBN JSON encoding"))
    }
}
// Records stay inline to avoid a heap allocation for every market event.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum DbnEvent {
    Metadata(Box<dbn::Metadata>),
    Record(DbnRecord),
}
impl DbnEvent {
    pub fn to_json(&self) -> Result<Value> {
        match self {
            Self::Metadata(m) => {
                let mut bytes = Vec::new();
                dbn::encode::json::Encoder::new(&mut bytes, false, false, false)
                    .encode_metadata(m)
                    .map_err(|_| Error::Decode("invalid DBN metadata"))?;
                let value: Value = serde_json::from_slice(&bytes)
                    .map_err(|_| Error::Decode("invalid DBN metadata JSON"))?;
                Ok(json!({"type":"metadata","metadata":value}))
            }
            Self::Record(r) => r.to_json(),
        }
    }
}
/// Incremental sans-I/O DBN decoding. Uses the official codec's v1-v3 upgrade
/// policy; unsupported versions and truncated metadata/records fail explicitly.
pub struct DbnStream {
    body: DecodedBody,
    fsm: DbnFsm,
    metadata: Option<dbn::Metadata>,
    done: bool,
    queued: VecDeque<Result<DbnEvent>>,
    checked_prelude: bool,
}
impl DbnStream {
    pub fn new(body: DecodedBody) -> Self {
        Self {
            body,
            fsm: DbnFsm::builder()
                .build()
                .expect("default DBN configuration is valid"),
            metadata: None,
            done: false,
            queued: VecDeque::new(),
            checked_prelude: false,
        }
    }
    pub fn metadata(&self) -> Option<&dbn::Metadata> {
        self.metadata.as_ref()
    }
    async fn read_one(&mut self) -> Option<Result<DbnEvent>> {
        if self.done {
            return None;
        }
        loop {
            // Validate the untrusted allocation size before the upstream codec
            // reads its prelude and grows the metadata buffer.
            if !self.checked_prelude && self.fsm.data().len() >= 8 {
                let data = self.fsm.data();
                if u32::from_le_bytes(data[4..8].try_into().expect("four bytes")) as usize
                    > MAX_RECORD_BYTES
                {
                    return self.fail(Error::Decode("DBN metadata exceeded configured limit"));
                }
                self.checked_prelude = true;
            }
            match self.fsm.process() {
                ProcessResult::Metadata(m) => {
                    self.metadata = Some(m.clone());
                    return Some(Ok(DbnEvent::Metadata(Box::new(m))));
                }
                ProcessResult::Record(()) => {
                    let reference = self.fsm.last_record().expect("codec returned a record");
                    let ts_out = if self.metadata.as_ref().is_some_and(|m| m.ts_out) {
                        let bytes = reference.as_ref();
                        if bytes.len() < 8 {
                            return self.fail(Error::Decode("missing DBN output timestamp"));
                        }
                        Some(u64::from_le_bytes(
                            bytes[bytes.len() - 8..].try_into().expect("eight bytes"),
                        ))
                    } else {
                        None
                    };
                    let record = dbn::RecordBuf::try_from(reference)
                        .map_err(|_| Error::Decode("invalid owned DBN record"));
                    return match record {
                        Ok(record) => Some(Ok(DbnEvent::Record(DbnRecord { record, ts_out }))),
                        Err(e) => self.fail(e),
                    };
                }
                ProcessResult::Err(_) => {
                    return self.fail(Error::Decode("invalid or unsupported DBN stream"));
                }
                ProcessResult::ReadMore(n) => {
                    if n > MAX_RECORD_BYTES || self.fsm.data().len() > MAX_RECORD_BYTES {
                        return self.fail(Error::Decode("DBN metadata exceeded configured limit"));
                    }
                    match self.body.reader().read(self.fsm.space()).await {
                        Ok(0) => {
                            if self.metadata.is_none() || !self.fsm.data().is_empty() {
                                return self.fail(Error::Decode("truncated DBN stream"));
                            }
                            self.done = true;
                            self.body.stop();
                            return None;
                        }
                        Ok(n) => self.fsm.fill(n),
                        Err(e) => return self.fail(e.into()),
                    }
                }
            }
        }
    }
    fn fail(&mut self, error: Error) -> Option<Result<DbnEvent>> {
        self.done = true;
        self.body.stop();
        Some(Err(error))
    }
    pub async fn next(&mut self) -> Option<Result<DbnEvent>> {
        if let Some(v) = self.queued.pop_front() {
            Some(v)
        } else {
            self.read_one().await
        }
    }
    pub async fn read_batch(&mut self, limit: usize) -> Result<Vec<DbnEvent>> {
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
        self.body.close().await
    }
}
