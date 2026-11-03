use std::{error::Error as StdError, fmt};
/// Server details remain inspectable, but Debug/Display never include response bodies.
pub enum Error {
    InvalidRequest(&'static str),
    Decode(serde_json::Error),
    Api {
        status: Option<u16>,
        body: serde_json::Value,
    },
    Signing(&'static str),
    Authentication(&'static str),
    /// A write may have been accepted; retain its original payload/idempotency key.
    OutcomeUncertain {
        status: Option<u16>,
        body: Option<serde_json::Value>,
    },
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(s) => write!(f, "invalid Monaco request: {s}"),
            Self::Decode(_) => f.write_str("Monaco response decoding failed"),
            Self::Api { status, .. } => write!(f, "Monaco exchange error (HTTP {status:?})"),
            Self::Signing(s) => write!(f, "Monaco signing failed: {s}"),
            Self::Authentication(s) => write!(f, "Monaco authentication failed: {s}"),
            Self::OutcomeUncertain { .. } => f.write_str("Monaco submission outcome unknown; reconcile using the original payload and idempotency key"),
        }
    }
}
impl StdError for Error {}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Decode(e)
    }
}
