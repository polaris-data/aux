use std::{error::Error as StdError, fmt};

/// Native codes and unknown error fields survive decoding. Debug/Display omit
/// server text and bodies, which can echo a submitted credential.
#[non_exhaustive]
pub enum Error {
    InvalidRequest(&'static str),
    Encode(serde_json::Error),
    Decode(serde_json::Error),
    Api {
        code: i64,
        message: Option<String>,
        extra: crate::ExtraFields,
        http_status: Option<u16>,
    },
    Http {
        status: u16,
        code: Option<i64>,
    },
    Signing(&'static str),
    InvalidSignature,
    NonceUncertain,
    NetworkMismatch,
    Subscription(&'static str),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(s) => write!(f, "invalid Lighter request: {s}"),
            Self::Encode(_) => f.write_str("Lighter request encoding failed"),
            Self::Decode(_) => f.write_str("Lighter response decoding failed"),
            Self::Api { code, .. } => write!(f, "Lighter rejected request (code {code})"),
            Self::Http { status, .. } => write!(f, "Lighter HTTP failure ({status})"),
            Self::Signing(s) => write!(f, "Lighter signing failed: {s}"),
            Self::InvalidSignature => f.write_str("invalid Lighter signature"),
            Self::NonceUncertain => f.write_str(
                "Lighter nonce outcome uncertain; reconcile and explicitly resynchronize",
            ),
            Self::NetworkMismatch => f.write_str("Lighter authentication chain context mismatch"),
            Self::Subscription(s) => write!(f, "Lighter subscription: {s}"),
        }
    }
}
impl StdError for Error {}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
