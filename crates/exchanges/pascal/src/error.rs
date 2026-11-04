use std::{error::Error as StdError, fmt};
pub enum Error {
    InvalidRequest(&'static str),
    Decode(serde_json::Error),
    Api {
        status: u16,
        body: serde_json::Value,
    },
    Authentication(&'static str),
    Signing(&'static str),
    PendingOutcome,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(s) => write!(f, "invalid native request: {s}"),
            Self::Decode(_) => f.write_str("native response decoding failed"),
            Self::Api { status, .. } => write!(f, "native API error (HTTP {status})"),
            Self::Authentication(s) => write!(f, "authentication failed: {s}"),
            Self::Signing(s) => write!(f, "signing failed: {s}"),
            Self::PendingOutcome => {
                f.write_str("reconcile the pending submission before another write")
            }
        }
    }
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        if let Self::Decode(e) = self {
            Some(e)
        } else {
            None
        }
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Decode(e)
    }
}
