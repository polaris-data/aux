//! The Zig archive owns all returned buffers and parser state. Borrowed inputs
//! live through the call and are never retained by Zig.
use std::{ffi::c_void, ptr::NonNull, slice};

use crate::{Error, Result};

const OK: u32 = 0;
const NEED_INPUT: u32 = 1;
const INVALID_REQUEST: u32 = 2;
const LIMIT: u32 = 5;
const TRUNCATED: u32 = 6;
const OUT_OF_MEMORY: u32 = 7;

#[repr(C)]
#[derive(Clone, Copy)]
struct Buffer {
    ptr: *mut u8,
    len: usize,
}

struct OwnedBuffer(Buffer);

impl OwnedBuffer {
    fn empty() -> Self {
        Self(Buffer {
            ptr: std::ptr::null_mut(),
            len: 0,
        })
    }

    fn bytes(&self) -> &[u8] {
        if self.0.len == 0 {
            return &[];
        }
        // SAFETY: A nonempty output contains len initialized bytes allocated by Zig.
        unsafe { slice::from_raw_parts(self.0.ptr, self.0.len) }
    }
}

impl Drop for OwnedBuffer {
    fn drop(&mut self) {
        // SAFETY: This is the sole owner and Zig also accepts an empty buffer.
        unsafe { aux_v1_hermes_buffer_free(self.0) };
    }
}

#[repr(C)]
struct View {
    ptr: *const u8,
    len: usize,
}

impl View {
    fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
        }
    }

    fn empty() -> Self {
        Self {
            ptr: std::ptr::null(),
            len: 0,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Operation {
    LatestPriceUpdates,
    StreamPriceUpdates,
    Live,
    Ready,
    PriceFeeds,
    PriceUpdatesAtTimestamp,
    LatestTwaps,
    LatestPublisherStakeCaps,
}

#[derive(Default)]
pub struct Request<'a> {
    pub ids: &'a [String],
    pub encoding: Option<&'a str>,
    pub parsed: Option<bool>,
    pub ignore_invalid_price_ids: Option<bool>,
    pub allow_unordered: Option<bool>,
    pub benchmarks_only: Option<bool>,
    pub asset_type: Option<&'a str>,
    pub query: Option<&'a str>,
    pub path_number: u64,
}

#[repr(C)]
struct RequestView {
    ids: *const View,
    ids_len: usize,
    encoding: View,
    parsed: u8,
    ignore_invalid_price_ids: u8,
    allow_unordered: u8,
    benchmarks_only: u8,
    asset_type: View,
    query: View,
    path_number: u64,
}

unsafe extern "C" {
    fn aux_v1_hermes_request(stream: u8, request: *const RequestView, out: *mut Buffer) -> u32;
    fn aux_v1_hermes_buffer_free(buffer: Buffer);
    fn aux_v1_hermes_sse_create(limit: usize) -> *mut c_void;
    fn aux_v1_hermes_sse_destroy(stream: *mut c_void);
    fn aux_v1_hermes_sse_push(stream: *mut c_void, ptr: *const u8, len: usize) -> u32;
    fn aux_v1_hermes_sse_next(stream: *mut c_void, out: *mut View) -> u32;
    fn aux_v1_hermes_sse_finish(stream: *const c_void) -> u32;
}

fn allocation_error() -> Error {
    Error::OutOfMemory
}

pub fn request(operation: Operation, request: Request<'_>) -> Result<String> {
    let text = |v: Option<&str>| v.map_or_else(View::empty, |s| View::from_bytes(s.as_bytes()));
    let flag = |flag: Option<bool>| flag.map_or(0, |value| u8::from(value) + 1);
    let ids: Vec<View> = request
        .ids
        .iter()
        .map(|id| View::from_bytes(id.as_bytes()))
        .collect();
    let request = RequestView {
        ids: ids.as_ptr(),
        ids_len: ids.len(),
        encoding: text(request.encoding),
        asset_type: text(request.asset_type),
        query: text(request.query),
        path_number: request.path_number,
        parsed: flag(request.parsed),
        ignore_invalid_price_ids: flag(request.ignore_invalid_price_ids),
        allow_unordered: flag(request.allow_unordered),
        benchmarks_only: flag(request.benchmarks_only),
    };
    let mut out = OwnedBuffer::empty();
    // SAFETY: The views stay live for the call and Zig does not retain them.
    match unsafe { aux_v1_hermes_request(operation as u8, &request, &mut out.0) } {
        OK => std::str::from_utf8(out.bytes())
            .map(str::to_owned)
            .map_err(|_| Error::Decode("invalid Hermes request path")),
        INVALID_REQUEST => Err(Error::InvalidRequest(
            "Hermes feed IDs must be 32-byte hexadecimal values",
        )),
        OUT_OF_MEMORY => Err(allocation_error()),
        _ => Err(Error::Decode("unexpected Hermes request status")),
    }
}

pub struct Sse(NonNull<c_void>);

// SAFETY: Zig's state has no thread affinity and mutation requires &mut self.
unsafe impl Send for Sse {}

impl Sse {
    pub fn new(limit: usize) -> Result<Self> {
        // SAFETY: Creation has no borrowed inputs and returns a unique handle.
        NonNull::new(unsafe { aux_v1_hermes_sse_create(limit) })
            .map(Self)
            .ok_or_else(allocation_error)
    }

    pub fn push(&mut self, bytes: &[u8]) -> Result<()> {
        // SAFETY: The stream is exclusively borrowed and Zig copies these bytes.
        match unsafe { aux_v1_hermes_sse_push(self.0.as_ptr(), bytes.as_ptr(), bytes.len()) } {
            OK => Ok(()),
            LIMIT => Err(Error::Decode("stream record exceeded configured limit")),
            OUT_OF_MEMORY => Err(allocation_error()),
            _ => Err(Error::Decode("unexpected Hermes stream status")),
        }
    }

    pub fn next_event(&mut self) -> Result<Option<&[u8]>> {
        let mut out = View::empty();
        // SAFETY: The stream and output are both exclusively borrowed.
        match unsafe { aux_v1_hermes_sse_next(self.0.as_ptr(), &mut out) } {
            OK => {
                let bytes = if out.len == 0 {
                    &[]
                } else {
                    // SAFETY: The event is valid until the next stream call.
                    unsafe { slice::from_raw_parts(out.ptr, out.len) }
                };
                Ok(Some(bytes))
            }
            NEED_INPUT => Ok(None),
            LIMIT => Err(Error::Decode("SSE event exceeded configured limit")),
            OUT_OF_MEMORY => Err(allocation_error()),
            _ => Err(Error::Decode("unexpected Hermes stream status")),
        }
    }

    pub fn finish(&self) -> Result<()> {
        // SAFETY: Finish reads the live handle without changing it.
        match unsafe { aux_v1_hermes_sse_finish(self.0.as_ptr()) } {
            OK => Ok(()),
            TRUNCATED => Err(Error::Decode("truncated HTTP stream record")),
            _ => Err(Error::Decode("unexpected Hermes stream status")),
        }
    }
}

impl Drop for Sse {
    fn drop(&mut self) {
        // SAFETY: This is the sole owning handle.
        unsafe { aux_v1_hermes_sse_destroy(self.0.as_ptr()) };
    }
}

/// Incremental SSE decoder. Feed arbitrary byte chunks and drain the returned
/// UTF-8 data fields. Closing the decoder releases the native state immediately.
pub struct EventDecoder(Option<Sse>);

impl EventDecoder {
    pub fn new(limit: usize) -> Result<Self> {
        if limit == 0 {
            return Err(Error::InvalidRequest("SSE limit must be positive"));
        }
        Ok(Self(Some(Sse::new(limit)?)))
    }
    pub fn feed(&mut self, bytes: &[u8]) -> Result<Vec<String>> {
        let stream = self.0.as_mut().ok_or(Error::Decode("stream closed"))?;
        let mut events = Vec::new();
        // A transport read may coalesce complete events beyond the per-record
        // cap. Always drain the parser before appending the next line.
        for chunk in bytes.split_inclusive(|b| matches!(b, b'\r' | b'\n')) {
            stream.push(chunk)?;
            while let Some(event) = stream.next_event()? {
                events.push(
                    std::str::from_utf8(event)
                        .map_err(|_| Error::Decode("invalid event UTF-8"))?
                        .to_owned(),
                );
            }
        }
        Ok(events)
    }
    pub fn finish(&mut self) -> Result<()> {
        self.0.take().map_or(Ok(()), |s| s.finish())
    }
    pub fn close(&mut self) {
        self.0.take();
    }
}
