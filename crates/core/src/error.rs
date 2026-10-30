use std::{error::Error as StdError, fmt};

/// Whether a failed transport operation may have reached the server.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequestOutcome {
    NotSent,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TransportErrorKind {
    Configuration,
    Connection,
    Timeout,
    Disconnected,
    Protocol,
    Overflow,
    ResponseTooLarge,
    /// Recoverable stream discontinuity. The next poll may yield fresh data.
    Gap,
    Closed,
    Authentication,
}

/// A transport failure without request bodies, credentials, or endpoint URLs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransportError {
    pub kind: TransportErrorKind,
    pub outcome: RequestOutcome,
    message: String,
}

impl TransportError {
    pub fn is_gap(&self) -> bool {
        self.kind == TransportErrorKind::Gap
    }
    /// Custom transports should also avoid including credentials in `message`.
    pub fn new(
        kind: TransportErrorKind,
        outcome: RequestOutcome,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            outcome,
            message: message.into(),
        }
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:?}, {:?})", self.message, self.kind, self.outcome)
    }
}

impl StdError for TransportError {}

/// Native exchange errors remain typed instead of being flattened to strings.
#[derive(Debug)]
pub enum Error<E> {
    Transport(TransportError),
    Exchange(E),
}

impl<E: fmt::Display> fmt::Display for Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(error) => error.fmt(f),
            Self::Exchange(error) => error.fmt(f),
        }
    }
}

impl<E: StdError + 'static> StdError for Error<E> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(match self {
            Self::Transport(error) => error,
            Self::Exchange(error) => error,
        })
    }
}

impl<E> From<TransportError> for Error<E> {
    fn from(error: TransportError) -> Self {
        Self::Transport(error)
    }
}
