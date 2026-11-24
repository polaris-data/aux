use crate::{Error, Result};
use std::{ptr, slice};

#[repr(C)]
#[derive(Clone, Copy)]
struct Buffer {
    ptr: *mut u8,
    len: usize,
}
unsafe extern "C" {
    fn aux_v1_invoke_detailed(
        id: u32,
        ptr: *const u8,
        len: usize,
        dial: *const StreamDial,
        out: *mut Buffer,
    ) -> u32;
    fn aux_v1_result_free(buffer: Buffer);
}
struct Owned(Buffer);
impl Owned {
    fn empty() -> Self {
        Self(Buffer {
            ptr: ptr::null_mut(),
            len: 0,
        })
    }
    fn bytes(&self) -> Result<&[u8]> {
        if self.0.ptr.is_null() {
            return Err(Error::Decode("missing native output"));
        }
        // The buffer lives until this owner is dropped.
        Ok(unsafe { slice::from_raw_parts(self.0.ptr, self.0.len) })
    }
    fn text(&self) -> Result<String> {
        let bytes = self.bytes()?;
        std::str::from_utf8(bytes)
            .map(str::to_owned)
            .map_err(|_| Error::Decode("invalid native JSON"))
    }
}

fn status(code: u32) -> Error {
    match code {
        2 => Error::OutOfMemory,
        3 => Error::Network,
        4 => Error::ExchangeApi,
        5 => Error::AuthenticationRequired,
        7 => Error::PendingOutcome,
        8 | 9 => Error::StreamGap,
        _ => Error::InvalidRequest("invalid operation or protocol data"),
    }
}
fn response_status(code: u32, out: &Owned) -> Error {
    if code == 4
        && let Ok(bytes) = out.bytes()
        && let Ok(detail) = serde_json::from_slice(bytes)
    {
        return Error::ExchangeResponse(detail);
    }
    status(code)
}
impl Drop for Owned {
    fn drop(&mut self) {
        // SAFETY: The native allocator accepts empty outputs and owns this buffer.
        unsafe { aux_v1_result_free(self.0) };
    }
}

/// Shared language-binding entry point. Returns UTF-8 JSON with exact number tokens.
#[doc(hidden)]
pub fn invoke(id: u32, input: &[u8]) -> Result<String> {
    let mut out = Owned::empty();
    // SAFETY: Both borrowed buffers remain valid throughout the synchronous call.
    match unsafe {
        aux_v1_invoke_detailed(id, input.as_ptr(), input.len(), ptr::null(), &mut out.0)
    } {
        0 => out.text(),
        code => Err(response_status(code, &out)),
    }
}

use std::{
    ffi::c_void,
    ptr::NonNull,
    sync::{Arc, Mutex},
};

unsafe extern "C" {
    fn aux_v1_dbn_open(ptr: *const u8, len: usize, out: *mut *mut c_void) -> u32;
    fn aux_v1_stream_next_detailed(handle: *mut c_void, out: *mut Buffer) -> u32;
    fn aux_v1_stream_concurrent_commands(handle: *const c_void) -> u8;
    fn aux_v1_stream_command_detailed(
        handle: *mut c_void,
        input: *const u8,
        len: usize,
        out: *mut Buffer,
    ) -> u32;
    fn aux_v1_stream_cancel(handle: *mut c_void);
    fn aux_v1_stream_destroy(handle: *mut c_void);
}

#[repr(C)]
struct StreamDial {
    context: *const c_void,
    connect: unsafe extern "C" fn(*const c_void, *const u8, usize, u16, bool, *mut i32) -> u32,
}

struct StreamPolicy(Arc<dyn crate::http::Transport>);
impl StreamPolicy {
    fn dial(&self) -> StreamDial {
        StreamDial {
            context: ptr::from_ref(self).cast(),
            connect: connect_stream,
        }
    }
}

unsafe extern "C" fn connect_stream(
    policy: *const c_void,
    host: *const u8,
    len: usize,
    port: u16,
    tls: bool,
    fd: *mut i32,
) -> u32 {
    use std::os::fd::IntoRawFd;
    // Zig borrows host for the call and the stream owner retains the policy until destroy returns.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let host = std::str::from_utf8(unsafe { slice::from_raw_parts(host, len) })
            .map_err(|_| Error::InvalidRequest("invalid stream host"))?;
        unsafe { &*policy.cast::<StreamPolicy>() }
            .0
            .connect_stream(host, port, tls)
    }));
    match result {
        Ok(Ok(socket)) => {
            unsafe { fd.write(socket.into_raw_fd()) };
            0
        }
        Ok(Err(Error::InvalidRequest(_))) => 1,
        Ok(Err(Error::AuthenticationRequired)) => 5,
        _ => 3,
    }
}

unsafe extern "C" {
    fn aux_v1_stream_open_detailed(
        ptr: *const u8,
        len: usize,
        dial: *const StreamDial,
        out: *mut *mut c_void,
        detail: *mut Buffer,
    ) -> u32;

}

pub(crate) fn execute_with_transport(
    input: &serde_json::Value,
    transport: &Arc<dyn crate::http::Transport>,
) -> Result<serde_json::Value> {
    let policy = StreamPolicy(transport.clone());
    let config = input.to_string();
    let mut out = Owned::empty();
    match unsafe {
        aux_v1_invoke_detailed(
            12,
            config.as_ptr(),
            config.len(),
            &policy.dial(),
            &mut out.0,
        )
    } {
        0 => serde_json::from_slice(out.bytes()?)
            .map_err(|_| Error::Decode("invalid native response")),
        code => Err(response_status(code, &out)),
    }
}

struct Handle {
    _policy: Option<Box<StreamPolicy>>,
    ptr: NonNull<c_void>,
    reader: Mutex<()>,
    concurrent_commands: bool,
}
// SAFETY: Zig reads are serialized by reader. Its cancellation function uses an
// atomic flag and coordinates socket shutdown with the reader. The last Arc owns destruction.
unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { aux_v1_stream_destroy(self.ptr.as_ptr()) }
    }
}

/// Clones share one native stream. Closing any clone interrupts the pending read.
#[derive(Clone)]
pub struct PriceStream(Arc<Mutex<Option<Arc<Handle>>>>);
impl PriceStream {
    #[doc(hidden)]
    pub fn open_json(config: &str) -> Result<Self> {
        Self::open_with_transport(config, None)
    }
    pub(crate) fn open_with_transport(
        config: &str,
        transport: Option<&Arc<dyn crate::http::Transport>>,
    ) -> Result<Self> {
        // The heap address remains stable across Handle moves and across reconnects.
        let policy = transport.map(|t| Box::new(StreamPolicy(t.clone())));
        let dial = policy.as_ref().map(|p| p.dial());
        let mut ptr = ptr::null_mut();
        let mut detail = Owned::empty();
        let code = unsafe {
            aux_v1_stream_open_detailed(
                config.as_ptr(),
                config.len(),
                dial.as_ref().map_or(ptr::null(), |d| d),
                &mut ptr,
                &mut detail.0,
            )
        };
        if code != 0 {
            return Err(response_status(code, &detail));
        }
        Self::from_handle(code, ptr, policy)
    }
    pub fn open_dbn(input: &[u8]) -> Result<Self> {
        Self::from_bytes(input, aux_v1_dbn_open)
    }
    fn from_bytes(
        input: &[u8],
        open: unsafe extern "C" fn(*const u8, usize, *mut *mut c_void) -> u32,
    ) -> Result<Self> {
        let mut ptr = ptr::null_mut();
        // Both native constructors borrow this slice and initialize the output pointer.
        let code = unsafe { open(input.as_ptr(), input.len(), &mut ptr) };
        Self::from_handle(code, ptr, None)
    }
    fn from_handle(code: u32, ptr: *mut c_void, policy: Option<Box<StreamPolicy>>) -> Result<Self> {
        if code != 0 {
            return Err(status(code));
        }
        let ptr = NonNull::new(ptr).ok_or(Error::Decode("missing stream"))?;
        Ok(Self(Arc::new(Mutex::new(Some(Arc::new(Handle {
            ptr,
            reader: Mutex::new(()),
            concurrent_commands: unsafe { aux_v1_stream_concurrent_commands(ptr.as_ptr()) != 0 },
            _policy: policy,
        }))))))
    }
    pub fn open(
        params: &serde_json::Value,
        origin: Option<&str>,
        api_key: Option<&str>,
    ) -> Result<Self> {
        let mut config = serde_json::json!({"venue":"pyth_hermes","operation":"stream_price_updates","params":params});
        if let Some(v) = origin {
            config["origin"] = v.into();
        }
        if let Some(v) = api_key {
            config["api_key"] = v.into();
        }
        Self::open_json(&config.to_string())
    }
    #[doc(hidden)]
    pub fn command_json(&self, input: &str) -> Result<String> {
        let handle = self
            .0
            .lock()
            .unwrap()
            .clone()
            .ok_or(Error::InvalidRequest("session closed"))?;
        let _reader = (!handle.concurrent_commands).then(|| handle.reader.lock().unwrap());
        let mut out = Owned::empty();
        // The Arc owns the handle and the reader lock serializes commands and reads.
        match unsafe {
            aux_v1_stream_command_detailed(
                handle.ptr.as_ptr(),
                input.as_ptr(),
                input.len(),
                &mut out.0,
            )
        } {
            0 => out.text(),
            10 => Err(Error::SessionResponse(
                serde_json::from_slice(out.bytes()?)
                    .map_err(|_| Error::Decode("invalid session rejection"))?,
            )),
            code => Err(status(code)),
        }
    }
    pub fn next_json(&self) -> Result<Option<String>> {
        self.read_next(Owned::text)
    }
    pub fn next_bytes(&self) -> Result<Option<Vec<u8>>> {
        self.read_next(|out| out.bytes().map(<[u8]>::to_vec))
    }
    fn read_next<T>(&self, decode: impl FnOnce(&Owned) -> Result<T>) -> Result<Option<T>> {
        let Some(handle) = self.0.lock().unwrap().clone() else {
            return Ok(None);
        };
        let _reader = if handle.concurrent_commands {
            handle
                .reader
                .try_lock()
                .map_err(|_| Error::InvalidRequest("read already in progress"))?
        } else {
            handle.reader.lock().unwrap()
        };
        let mut out = Owned::empty();
        let code = unsafe { aux_v1_stream_next_detailed(handle.ptr.as_ptr(), &mut out.0) };
        match code {
            0 => decode(&out).map(Some),
            6 => {
                self.close();
                Ok(None)
            }
            8 => Err(Error::StreamGap),
            10 => {
                self.close();
                Err(Error::SessionResponse(
                    serde_json::from_slice(out.bytes()?)
                        .map_err(|_| Error::Decode("invalid subscription rejection"))?,
                ))
            }
            other => {
                self.close();
                Err(status(other))
            }
        }
    }
    pub fn next_event(&self) -> Result<Option<serde_json::Value>> {
        self.next_json()?
            .map(|text| {
                serde_json::from_str(&text).map_err(|_| Error::Decode("invalid stream JSON"))
            })
            .transpose()
    }
    pub fn close(&self) {
        if let Some(handle) = self.0.lock().unwrap().take() {
            unsafe { aux_v1_stream_cancel(handle.ptr.as_ptr()) };
        }
    }
}
