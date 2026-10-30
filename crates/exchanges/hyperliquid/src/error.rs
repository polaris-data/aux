use std::{error::Error as StdError, fmt};

#[non_exhaustive]
pub enum Error {
    InvalidRequest(&'static str),
    Encode(serde_json::Error),
    Decode(serde_json::Error),
    /// Non-success HTTP status, retaining the exchange's response body.
    Http {
        status: u16,
        body: String,
    },
    /// A native error response or WebSocket error channel.
    Api {
        message: String,
    },
    Signing(crate::SignerError),
    NonceExhausted,
    Clock,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(message) => write!(f, "invalid Hyperliquid request: {message}"),
            Self::Encode(_) => f.write_str("Hyperliquid request encoding failed"),
            Self::Decode(_) => f.write_str("Hyperliquid response decoding failed"),
            Self::Http { status, .. } => write!(f, "Hyperliquid returned HTTP {status}"),
            Self::Api { .. } => f.write_str("Hyperliquid rejected request"),
            Self::Signing(error) => error.fmt(f),
            Self::NonceExhausted => f.write_str("Hyperliquid nonce counter exhausted"),
            Self::Clock => f.write_str("unable to read a valid Unix millisecond clock"),
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
        match self {
            Self::Signing(error) => Some(error),
            _ => None,
        }
    }
}
