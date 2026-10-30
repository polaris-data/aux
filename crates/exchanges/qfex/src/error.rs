use std::{error::Error as StdError, fmt};
/// Native response details are inspectable but excluded from formatted errors.
pub enum Error {
    InvalidRequest(&'static str),
    Decode(serde_json::Error),
    Api {
        status: Option<u16>,
        body: serde_json::Value,
    },
    Signing(&'static str),
    PendingOutcome,
}
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
 Self::InvalidRequest(s)=>write!(f,"invalid request: {s}"), Self::Decode(_)=>f.write_str("native response decoding failed"),
 Self::Api{status,..}=>write!(f,"native exchange error (HTTP {status:?})"), Self::Signing(s)=>write!(f,"signing failed: {s}"),
 Self::PendingOutcome=>f.write_str("an earlier command has an unresolved outcome; reconcile it before sending another command"),
 }
    }
}
impl StdError for Error {}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Decode(e)
    }
}
