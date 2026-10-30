use aux_rs::{
    Error as NativeError, RequestOutcome, TransportErrorKind, hyperliquid as hl, lighter as lt,
};
use pyo3::{create_exception, exceptions::PyException, prelude::*};

pub enum ClientError {
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
            | Self::Lighter(NativeError::Transport(e)) => e.is_gap(),
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
        | ClientError::Lighter(NativeError::Transport(e)) => transport(e),
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
        NonceUncertainError
    );
    Ok(())
}
