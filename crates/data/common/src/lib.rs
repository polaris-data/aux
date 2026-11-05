//! Bounded streaming support and Tokio-backed file downloads. No vendor protocols live here.
mod body;
mod files;
mod http;
use aux_rs_core::TransportError;
pub use body::*;
pub use files::*;
pub use http::*;
use serde_json::Value;

pub type Result<T> = std::result::Result<T, Error>;
/// Native rejection details are accessible explicitly, never included in diagnostics.
pub enum Error {
    Transport(TransportError),
    Api { status: u16, body: Value },
    Authentication { body: Value },
    InvalidRequest(&'static str),
    Decode(&'static str),
    Io(std::io::Error),
}
impl Error {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Transport(_) => "transport",
            Self::Api { status: 401, .. } | Self::Authentication { .. } => "authentication",
            Self::Api { status: 403, .. } => "entitlement",
            Self::Api { status: 429, .. } => "rate_limit",
            Self::Api { .. } => "vendor",
            Self::InvalidRequest(_) => "validation",
            Self::Decode(_) => "decode",
            Self::Io(_) => "io",
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transport(e) => e.fmt(f),
            Self::Api { status, .. } => write!(f, "data vendor returned HTTP {status}"),
            Self::Authentication { .. } => f.write_str("data vendor authentication failed"),
            Self::InvalidRequest(s) | Self::Decode(s) => f.write_str(s),
            Self::Io(_) => f.write_str("data stream I/O failed"),
        }
    }
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Transport(e) => Some(e),
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}
impl From<TransportError> for Error {
    fn from(e: TransportError) -> Self {
        Self::Transport(e)
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        match e.get_ref().and_then(|e| e.downcast_ref::<TransportError>()) {
            Some(e) => Self::Transport(e.clone()),
            None if matches!(
                e.kind(),
                std::io::ErrorKind::InvalidData | std::io::ErrorKind::UnexpectedEof
            ) =>
            {
                Self::Decode("invalid or truncated encoded data")
            }
            None => Self::Io(e),
        }
    }
}
/// Native parameter object. Strings, exact JSON numbers, arrays and booleans are
/// retained until the vendor's query/form encoding is applied.
pub type Params = serde_json::Map<String, Value>;
pub fn params(value: Value) -> Result<Params> {
    value
        .as_object()
        .cloned()
        .ok_or(Error::InvalidRequest("expected a native parameter object"))
}
