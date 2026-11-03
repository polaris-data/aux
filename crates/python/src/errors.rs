use aux_rs::{
    Error as NativeError, RequestOutcome, TransportErrorKind, hyperliquid as hl, lighter as lt,
};
use pyo3::{create_exception, exceptions::PyException, prelude::*};

pub enum ClientError {
    Derive(NativeError<aux_rs::derive::Error>),
    Paradex(NativeError<aux_rs::paradex::Error>),
    Monaco(NativeError<aux_rs::monaco::Error>),
    Qfex(NativeError<aux_rs::qfex::Error>),
    Arcus(NativeError<aux_rs::arcus::Error>),
    Hyperliquid(NativeError<hl::Error>),
    Lighter(NativeError<lt::Error>),
    Conversion,
}
create_exception!(aux_py, ConnectorError, PyException);
create_exception!(aux_py, TransportError, ConnectorError);
create_exception!(aux_py, GapError, TransportError);

impl ClientError {
    pub fn is_gap(&self) -> bool {
        match self {
            Self::Hyperliquid(NativeError::Transport(e))
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
create_exception!(aux_py, ExchangeError, ConnectorError);
create_exception!(aux_py, DecodeError, ConnectorError);
create_exception!(aux_py, ValidationError, ConnectorError);
create_exception!(aux_py, ClosedError, ConnectorError);
create_exception!(aux_py, BusyError, ConnectorError);
create_exception!(aux_py, AuthenticationError, ConnectorError);
create_exception!(aux_py, SigningError, ConnectorError);
create_exception!(aux_py, NonceUncertainError, ConnectorError);
create_exception!(aux_py, OutcomeUncertainError, ConnectorError);

pub fn transport(native: &aux_rs::TransportError) -> PyErr {
    Python::attach(|py| -> PyResult<PyErr> {
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
        error.value(py).setattr("kind", kind)?;
        error.value(py).setattr(
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
) -> PyErr {
    Python::attach(|py| -> PyResult<PyErr> {
        // Raw server text may echo credentials; never include it in args/repr.
        let error = ExchangeError::new_err(format!("{venue} rejected request"));
        error.value(py).setattr("venue", venue)?;
        error.value(py).setattr("http_status", status)?;
        error.value(py).setattr("body", body)?;
        error.value(py).setattr("native_message", message)?;
        error.value(py).setattr("code", code)?;
        error.value(py).setattr(
            "extra",
            crate::convert::output(extra.cloned().unwrap_or_default())?,
        )?;
        Ok(error)
    })
    .unwrap_or_else(std::convert::identity)
}
pub fn hl(error: NativeError<hl::Error>) -> PyErr {
    map_error(&ClientError::Hyperliquid(error))
}
pub fn lt(error: NativeError<lt::Error>) -> PyErr {
    map_error(&ClientError::Lighter(error))
}
pub fn hl_native(error: hl::Error) -> PyErr {
    hl(NativeError::Exchange(error))
}
pub fn lt_native(error: lt::Error) -> PyErr {
    lt(NativeError::Exchange(error))
}
pub fn map_error(error: &ClientError) -> PyErr {
    match error {
        ClientError::Hyperliquid(NativeError::Transport(e))
        | ClientError::Derive(NativeError::Transport(e))
        | ClientError::Paradex(NativeError::Transport(e))
        | ClientError::Lighter(NativeError::Transport(e))
        | ClientError::Qfex(NativeError::Transport(e))
        | ClientError::Arcus(NativeError::Transport(e))
        | ClientError::Monaco(NativeError::Transport(e)) => transport(e),
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
            aux_rs::monaco::Error::OutcomeUncertain {status,body} => Python::attach(|py| -> PyResult<PyErr> {
                let error=OutcomeUncertainError::new_err("Monaco submission outcome unknown; retain the original payload and idempotency key");
                error.value(py).setattr("http_status",status)?;
                error.value(py).setattr("outcome","unknown")?;
                error.value(py).setattr("body",crate::convert::output(body)?)?;
                Ok(error)
            }).unwrap_or_else(std::convert::identity),
            aux_rs::monaco::Error::Api {status,body} => Python::attach(|py| -> PyResult<PyErr> {
                let error=ExchangeError::new_err("Monaco rejected request");
                error.value(py).setattr("venue","Monaco")?;
                error.value(py).setattr("http_status",status)?;
                error.value(py).setattr("body",crate::convert::output(body)?)?;
                error.value(py).setattr("code",crate::convert::output(body.get("code"))?)?;
                error.value(py).setattr("native_message",crate::convert::output(body.get("message"))?)?;
                Ok(error)
            }).unwrap_or_else(std::convert::identity),
        },
        ClientError::Conversion => {
            DecodeError::new_err("unexpected or unrepresentable native stream event")
        }
    }
}
pub fn add_to_module(module: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = module.py();
    macro_rules! add {($($name:ident),+)=>{$(module.add(stringify!($name),py.get_type::<$name>())?;)+}}
    add!(
        ConnectorError,
        GapError,
        TransportError,
        ExchangeError,
        DecodeError,
        ValidationError,
        ClosedError,
        BusyError,
        AuthenticationError,
        SigningError,
        NonceUncertainError,
        OutcomeUncertainError
    );
    Ok(())
}

pub fn qfex(error: NativeError<aux_rs::qfex::Error>) -> PyErr {
    map_error(&ClientError::Qfex(error))
}

pub fn arcus(error: NativeError<aux_rs::arcus::Error>) -> PyErr {
    map_error(&ClientError::Arcus(error))
}

pub fn monaco(error: NativeError<aux_rs::monaco::Error>) -> PyErr {
    map_error(&ClientError::Monaco(error))
}
pub fn derive(error: NativeError<aux_rs::derive::Error>) -> PyErr {
    map_error(&ClientError::Derive(error))
}
pub fn paradex(error: NativeError<aux_rs::paradex::Error>) -> PyErr {
    map_error(&ClientError::Paradex(error))
}
fn native_body(venue: &str, status: Option<u16>, body: &serde_json::Value) -> PyErr {
    let native = body.get("error").unwrap_or(body);
    let code = native.get("code").and_then(serde_json::Value::as_i64);
    let message = native
        .get("message")
        .or_else(|| body.get("message"))
        .and_then(serde_json::Value::as_str);
    let error = exchange(venue, status, None, message, code, None);
    Python::attach(|py| {
        if let Ok(value) = crate::convert::output(body) {
            let _ = error.value(py).setattr("native", value);
        }
        if let Some(code) = native.as_str() {
            let _ = error.value(py).setattr("code", code);
        }
    });
    error
}
