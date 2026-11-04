use std::{error::Error as StdError, fmt};

/// Native details are inspectable, but formatted errors never include payloads or credentials.
pub enum Error {
    InvalidRequest(&'static str),
    Decode(serde_json::Error),
    Api {
        status: u16,
        body: serde_json::Value,
    },
    Rpc {
        code: i64,
        message: String,
        data: Option<serde_json::Value>,
    },
    Abi(String),
    Signing(&'static str),
    Authentication(&'static str),
    PendingOutcome,
    UncertainSubmission {
        hash: alloy_primitives::B256,
        cause: Box<Error>,
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
            Self::InvalidRequest(s) => write!(f, "invalid native request: {s}"),
            Self::Decode(_) => f.write_str("native response decoding failed"),
            Self::Api { status, .. } => write!(f, "native API error (HTTP {status})"),
            Self::Rpc { code, .. } => write!(f, "native chain RPC error ({code})"),
            Self::Abi(_) => f.write_str("native ABI encoding/decoding failed"),
            Self::Signing(s) => write!(f, "native signing failed: {s}"),
            Self::Authentication(s) => write!(f, "native authentication failed: {s}"),
            Self::PendingOutcome => f.write_str(
                "reconcile the pending native transaction before allocating another nonce",
            ),
            Self::UncertainSubmission { .. } => {
                f.write_str("native transaction may have been submitted; reconcile its hash")
            }
        }
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Decode(e) => Some(e),
            Self::UncertainSubmission { cause, .. } => Some(cause.as_ref()),
            _ => None,
        }
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Decode(e)
    }
}
