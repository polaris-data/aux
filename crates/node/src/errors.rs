//! Error categories and submission metadata are preserved across the native boundary.
use serde::Serialize;
use std::sync::{Arc, Mutex};
#[derive(Clone, Debug)]
pub struct Error {
    name: &'static str,
    message: String,
    fields: Arc<Mutex<std::collections::BTreeMap<String, serde_value::Value>>>,
}
impl Error {
    pub fn value(&self) -> &Self {
        self
    }
    pub fn setattr(&self, name: &str, value: impl Serialize) -> crate::prelude::Result<()> {
        self.fields
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(name.into(), crate::convert::output(value)?);
        Ok(())
    }
    pub fn into_napi(self) -> napi::Error {
        let message =
            crate::convert::encode(&self).unwrap_or_else(|_| "error conversion failed".into());
        napi::Error::from_reason(format!("AUX_ERROR:{message}"))
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
impl std::error::Error for Error {}
impl Serialize for Error {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let fields = self.fields.lock().unwrap_or_else(|e| e.into_inner());
        let mut m = s.serialize_map(Some(fields.len() + 2))?;
        m.serialize_entry("name", self.name)?;
        m.serialize_entry("message", &self.message)?;
        for (k, v) in fields.iter() {
            m.serialize_entry(k, v)?;
        }
        m.end()
    }
}
macro_rules! error_type {
    ($name:ident, $base:ident) => {
        pub struct $name;
        impl $name {
            pub fn new_err(message: impl ToString) -> Error {
                Error {
                    name: stringify!($name),
                    message: message.to_string(),
                    fields: Arc::new(Mutex::new(Default::default())),
                }
            }
        }
    };
}
use crate::prelude::*;
use crate::{context, errors::Error as NativeBindingError};
use aux_rs::{
    Error as NativeError, RequestOutcome, TransportErrorKind, hyperliquid as hl, lighter as lt,
};

pub enum ClientError {
    Metric(NativeError<aux_rs::metric::Error>),
    Derive(NativeError<aux_rs::derive::Error>),
    Paradex(NativeError<aux_rs::paradex::Error>),
    Monaco(NativeError<aux_rs::monaco::Error>),
    Qfex(NativeError<aux_rs::qfex::Error>),
    Arcus(NativeError<aux_rs::arcus::Error>),
    Hyperliquid(NativeError<hl::Error>),
    Lighter(NativeError<lt::Error>),
    Conversion,
}
error_type!(ConnectorError, ConnectorError);
error_type!(TransportError, ConnectorError);
error_type!(GapError, TransportError);

impl ClientError {
    pub fn is_gap(&self) -> bool {
        match self {
            Self::Metric(NativeError::Transport(e))
            | Self::Hyperliquid(NativeError::Transport(e))
            | Self::Derive(NativeError::Transport(e))
            | Self::Paradex(NativeError::Transport(e))
            | Self::Lighter(NativeError::Transport(e))
            | Self::Qfex(NativeError::Transport(e))
            | Self::Arcus(NativeError::Transport(e))
            | Self::Monaco(NativeError::Transport(e)) => e.is_gap(),
            _ => false,
        }
    }
}
error_type!(ExchangeError, ConnectorError);
error_type!(DecodeError, ConnectorError);
error_type!(ValidationError, ConnectorError);
error_type!(ClosedError, ConnectorError);
error_type!(BusyError, ConnectorError);
error_type!(AuthenticationError, ConnectorError);
error_type!(SigningError, ConnectorError);
error_type!(NonceUncertainError, ConnectorError);
error_type!(OutcomeUncertainError, ConnectorError);

pub fn transport(native: &aux_rs::TransportError) -> NativeBindingError {
    context::evaluate(|| -> Result<NativeBindingError> {
        let error = if native.is_gap() {
            GapError::new_err(native.to_string())
        } else if native.kind == TransportErrorKind::Authentication {
            AuthenticationError::new_err(native.to_string())
        } else {
            TransportError::new_err(native.to_string())
        };
        let kind = match native.kind {
            TransportErrorKind::Configuration => "configuration",
            TransportErrorKind::Connection => "connection",
            TransportErrorKind::Timeout => "timeout",
            TransportErrorKind::Disconnected => "disconnected",
            TransportErrorKind::Protocol => "protocol",
            TransportErrorKind::Overflow => "overflow",
            TransportErrorKind::ResponseTooLarge => "response_too_large",
            TransportErrorKind::Gap => "gap",
            TransportErrorKind::Closed => "closed",
            TransportErrorKind::Authentication => "authentication",
            _ => "unknown",
        };
        error.value().setattr("kind", kind)?;
        error.value().setattr(
            "outcome",
            match native.outcome {
                RequestOutcome::NotSent => "not_sent",
                RequestOutcome::Unknown => "unknown",
            },
        )?;
        Ok(error)
    })
    .unwrap_or_else(std::convert::identity)
}
fn exchange(
    venue: &str,
    status: Option<u16>,
    body: Option<&str>,
    message: Option<&str>,
    code: Option<i64>,
    extra: Option<&lt::ExtraFields>,
) -> NativeBindingError {
    context::evaluate(|| -> Result<NativeBindingError> {
        // Raw server text may echo credentials; never include it in args/repr.
        let error = ExchangeError::new_err(format!("{venue} rejected request"));
        error.value().setattr("venue", venue)?;
        error.value().setattr("http_status", status)?;
        error.value().setattr("body", body)?;
        error.value().setattr("native_message", message)?;
        error.value().setattr("code", code)?;
        error.value().setattr(
            "extra",
            crate::convert::output(extra.cloned().unwrap_or_default())?,
        )?;
        Ok(error)
    })
    .unwrap_or_else(std::convert::identity)
}
pub fn hl(error: NativeError<hl::Error>) -> NativeBindingError {
    map_error(&ClientError::Hyperliquid(error))
}
pub fn lt(error: NativeError<lt::Error>) -> NativeBindingError {
    map_error(&ClientError::Lighter(error))
}
pub fn hl_native(error: hl::Error) -> NativeBindingError {
    hl(NativeError::Exchange(error))
}
pub fn lt_native(error: lt::Error) -> NativeBindingError {
    lt(NativeError::Exchange(error))
}
pub fn map_error(error: &ClientError) -> NativeBindingError {
    match error {
        ClientError::Metric(NativeError::Transport(e))
        | ClientError::Hyperliquid(NativeError::Transport(e))
        | ClientError::Derive(NativeError::Transport(e))
        | ClientError::Paradex(NativeError::Transport(e))
        | ClientError::Lighter(NativeError::Transport(e))
        | ClientError::Qfex(NativeError::Transport(e))
        | ClientError::Arcus(NativeError::Transport(e))
        | ClientError::Monaco(NativeError::Transport(e)) => transport(e),
        ClientError::Metric(NativeError::Exchange(e)) => metric_native(e),
        ClientError::Derive(NativeError::Exchange(e)) => match e {
            aux_rs::derive::Error::InvalidRequest(s)=>ValidationError::new_err(*s),
            aux_rs::derive::Error::Decode(_)=>DecodeError::new_err("Derive decoding failed"),
            aux_rs::derive::Error::Signing(_)=>SigningError::new_err("Derive signing failed"),
            aux_rs::derive::Error::Authentication(s)=>AuthenticationError::new_err(*s),
            aux_rs::derive::Error::PendingOutcome=>OutcomeUncertainError::new_err("Derive command outcome is unresolved; reconcile before another command"),
            aux_rs::derive::Error::Api{status,body}=>native_body("Derive",*status,body),
        },
        ClientError::Paradex(NativeError::Exchange(e)) => match e {
            aux_rs::paradex::Error::InvalidRequest(s)=>ValidationError::new_err(*s),
            aux_rs::paradex::Error::Decode(_)=>DecodeError::new_err("Paradex decoding failed"),
            aux_rs::paradex::Error::Signing(_)=>SigningError::new_err("Paradex signing failed"),
            aux_rs::paradex::Error::Authentication(s)=>AuthenticationError::new_err(*s),
            aux_rs::paradex::Error::PendingOutcome=>OutcomeUncertainError::new_err("Paradex command outcome is unresolved; reconcile before another command"),
            aux_rs::paradex::Error::Api{status,body}=>native_body("Paradex",*status,body),
        },
        ClientError::Hyperliquid(NativeError::Exchange(e)) => match e {
            hl::Error::Http { status, body } => {
                exchange("Hyperliquid", Some(*status), Some(body), None, None, None)
            }
            hl::Error::Api { message } => {
                exchange("Hyperliquid", None, None, Some(message), None, None)
            }
            hl::Error::InvalidRequest(message) => ValidationError::new_err(*message),
            hl::Error::Signing(_) => SigningError::new_err("Hyperliquid signing failed"),
            hl::Error::Decode(_) | hl::Error::Encode(_) => {
                DecodeError::new_err("Hyperliquid native encoding/decoding failed")
            }
            _ => ConnectorError::new_err("Hyperliquid operation failed"),
        },
        ClientError::Lighter(NativeError::Exchange(e)) => match e {
            lt::Error::Api {
                code,
                message,
                extra,
                http_status,
            } => exchange(
                "Lighter",
                *http_status,
                None,
                message.as_deref(),
                Some(*code),
                Some(extra),
            ),
            lt::Error::Http { status, code } => {
                exchange("Lighter", Some(*status), None, None, *code, None)
            }
            lt::Error::InvalidRequest(message) => ValidationError::new_err(*message),
            lt::Error::Signing(_) | lt::Error::InvalidSignature => {
                SigningError::new_err("Lighter signing failed")
            }
            lt::Error::NonceUncertain => NonceUncertainError::new_err(
                "Lighter nonce outcome uncertain; reconcile pending transactions before explicit resync_nonce()",
            ),
            lt::Error::NetworkMismatch => {
                AuthenticationError::new_err("Lighter authentication context mismatch")
            }
            lt::Error::Decode(_) | lt::Error::Encode(_) => {
                DecodeError::new_err("Lighter native encoding/decoding failed")
            }
            _ => ConnectorError::new_err("Lighter operation failed"),
        },
        ClientError::Qfex(NativeError::Exchange(e)) => match e {
            aux_rs::qfex::Error::InvalidRequest(message) => ValidationError::new_err(*message),
            aux_rs::qfex::Error::Decode(_) => DecodeError::new_err("QFEX decoding failed"),
            aux_rs::qfex::Error::Signing(_) => SigningError::new_err("QFEX signing failed"),
            aux_rs::qfex::Error::PendingOutcome => OutcomeUncertainError::new_err(
                "QFEX command outcome is unresolved; reconcile before another command",
            ),
            aux_rs::qfex::Error::Api { status, body } => exchange(
                "QFEX",
                *status,
                Some(&body.to_string()),
                body.get("error")
                    .or_else(|| body.get("message"))
                    .and_then(serde_json::Value::as_str),
                None,
                body.as_object()
                    .map(|m| {
                        m.iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect::<lt::ExtraFields>()
                    })
                    .as_ref(),
            ),
        },
        ClientError::Arcus(NativeError::Exchange(e)) => match e {
            aux_rs::arcus::Error::InvalidRequest(message) => ValidationError::new_err(*message),
            aux_rs::arcus::Error::Decode(_) => DecodeError::new_err("Arcus decoding failed"),
            aux_rs::arcus::Error::Signing(_) => SigningError::new_err("Arcus signing failed"),
            aux_rs::arcus::Error::PendingOutcome => OutcomeUncertainError::new_err(
                "Arcus command outcome is unresolved; reconcile before another command",
            ),
            aux_rs::arcus::Error::Api { status, body } => exchange(
                "Arcus",
                *status,
                Some(&body.to_string()),
                body.get("error")
                    .or_else(|| body.get("message"))
                    .and_then(serde_json::Value::as_str),
                None,
                body.as_object()
                    .map(|m| {
                        m.iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect::<lt::ExtraFields>()
                    })
                    .as_ref(),
            ),
        },
        ClientError::Monaco(NativeError::Exchange(e)) => match e {
            aux_rs::monaco::Error::InvalidRequest(m) => ValidationError::new_err(*m),
            aux_rs::monaco::Error::Decode(_) => DecodeError::new_err("Monaco decoding failed"),
            aux_rs::monaco::Error::Signing(_) => SigningError::new_err("Monaco signing failed"),
            aux_rs::monaco::Error::Authentication(m) => AuthenticationError::new_err(*m),
            aux_rs::monaco::Error::OutcomeUncertain {status,body} => context::evaluate(|| -> Result<NativeBindingError> {
                let error=OutcomeUncertainError::new_err("Monaco submission outcome unknown; retain the original payload and idempotency key");
                error.value().setattr("http_status",status)?;
                error.value().setattr("outcome","unknown")?;
                error.value().setattr("body",crate::convert::output(body)?)?;
                Ok(error)
            }).unwrap_or_else(std::convert::identity),
            aux_rs::monaco::Error::Api {status,body} => context::evaluate(|| -> Result<NativeBindingError> {
                let error=ExchangeError::new_err("Monaco rejected request");
                error.value().setattr("venue","Monaco")?;
                error.value().setattr("http_status",status)?;
                error.value().setattr("body",crate::convert::output(body)?)?;
                error.value().setattr("code",crate::convert::output(body.get("code"))?)?;
                error.value().setattr("native_message",crate::convert::output(body.get("message"))?)?;
                Ok(error)
            }).unwrap_or_else(std::convert::identity),
        },
        ClientError::Conversion => {
            DecodeError::new_err("unexpected or unrepresentable native stream event")
        }
    }
}
pub fn qfex(error: NativeError<aux_rs::qfex::Error>) -> NativeBindingError {
    map_error(&ClientError::Qfex(error))
}

pub fn arcus(error: NativeError<aux_rs::arcus::Error>) -> NativeBindingError {
    map_error(&ClientError::Arcus(error))
}

pub fn monaco(error: NativeError<aux_rs::monaco::Error>) -> NativeBindingError {
    map_error(&ClientError::Monaco(error))
}
pub fn derive(error: NativeError<aux_rs::derive::Error>) -> NativeBindingError {
    map_error(&ClientError::Derive(error))
}
pub fn paradex(error: NativeError<aux_rs::paradex::Error>) -> NativeBindingError {
    map_error(&ClientError::Paradex(error))
}
fn native_body(venue: &str, status: Option<u16>, body: &serde_json::Value) -> NativeBindingError {
    let native = body.get("error").unwrap_or(body);
    let code = native.get("code").and_then(serde_json::Value::as_i64);
    let message = native
        .get("message")
        .or_else(|| body.get("message"))
        .and_then(serde_json::Value::as_str);
    let error = exchange(venue, status, None, message, code, None);
    context::evaluate(|| {
        if let Ok(value) = crate::convert::output(body) {
            let _ = error.value().setattr("native", value);
        }
        if let Some(code) = native.as_str() {
            let _ = error.value().setattr("code", code);
        }
    });
    error
}

pub fn metric(error: NativeError<aux_rs::metric::Error>) -> NativeBindingError {
    map_error(&ClientError::Metric(error))
}

fn metric_native(e: &aux_rs::metric::Error) -> NativeBindingError {
    match e {
        aux_rs::metric::Error::InvalidRequest(s) => ValidationError::new_err(*s),
        aux_rs::metric::Error::Decode(_) | aux_rs::metric::Error::Abi(_) => {
            DecodeError::new_err("Metric codec failed")
        }
        aux_rs::metric::Error::Signing(_) => SigningError::new_err("Metric signing failed"),
        aux_rs::metric::Error::Authentication(s) => AuthenticationError::new_err(*s),
        aux_rs::metric::Error::PendingOutcome => {
            OutcomeUncertainError::new_err("Metric submission outcome is unresolved")
        }
        aux_rs::metric::Error::Api { status, body } => native_body("Metric", Some(*status), body),
        aux_rs::metric::Error::Rpc {
            code,
            message,
            data,
        } => native_body(
            "Metric",
            None,
            &serde_json::json!({"code":code,"message":message,"data":data}),
        ),
        aux_rs::metric::Error::UncertainSubmission { hash, cause } => {
            context::evaluate(|| -> Result<NativeBindingError> {
                let error = OutcomeUncertainError::new_err(
                    "Metric submission response is uncertain; reconcile the transaction hash",
                );
                error.value().setattr("hash", format!("{hash:#x}"))?;
                error.value().setattr("outcome", "unknown")?;
                error
                    .value()
                    .setattr("cause", metric_native(cause).value())?;
                Ok(error)
            })
            .unwrap_or_else(std::convert::identity)
        }
    }
}
