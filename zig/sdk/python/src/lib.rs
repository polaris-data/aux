use pyo3::{
    exceptions::{PyMemoryError, PyValueError},
    prelude::*,
};

pyo3::create_exception!(aux_zig, ConnectorError, PyValueError);
pyo3::create_exception!(aux_zig, AuthenticationError, ConnectorError);
pyo3::create_exception!(aux_zig, TransportError, ConnectorError);
pyo3::create_exception!(aux_zig, ExchangeError, ConnectorError);
pyo3::create_exception!(aux_zig, DecodeError, ConnectorError);
pyo3::create_exception!(aux_zig, ValidationError, ConnectorError);
pyo3::create_exception!(aux_zig, StreamGapError, TransportError);
pyo3::create_exception!(aux_zig, OutcomeUncertainError, ConnectorError);

fn native_error(error: aux_zig::Error) -> PyErr {
    use aux_zig::Error::*;
    let message = error.to_string();
    match error {
        OutOfMemory => PyMemoryError::new_err(message),
        AuthenticationRequired => AuthenticationError::new_err(message),
        Network => TransportError::new_err(message),
        ExchangeResponse(detail) => exchange_error(
            message,
            detail.venue,
            Some(detail.status),
            detail.body.to_string(),
        ),
        SessionResponse(detail) => {
            exchange_error(message, detail.venue, None, detail.body.to_string())
        }
        ExchangeApi => ExchangeError::new_err(message),
        Decode(_) => DecodeError::new_err(message),
        InvalidRequest(_) => ValidationError::new_err(message),
        StreamGap => StreamGapError::new_err(message),
        PendingOutcome => OutcomeUncertainError::new_err(message),
    }
}

fn exchange_error(message: String, venue: String, status: Option<u16>, body: String) -> PyErr {
    Python::attach(|py| {
        let err = ExchangeError::new_err(message);
        let attach = || -> PyResult<()> {
            let instance = err.value(py);
            instance.setattr("venue", venue)?;
            if let Some(code) = status {
                instance.setattr("status", code)?;
            }
            let kwargs = pyo3::types::PyDict::new(py);
            kwargs.set_item("parse_float", py.import("decimal")?.getattr("Decimal")?)?;
            let body = py
                .import("json")?
                .getattr("loads")?
                .call((body,), Some(&kwargs))?;
            instance.setattr("body", body)
        };
        match attach() {
            Ok(()) => err,
            Err(other) => other,
        }
    })
}

#[pyfunction]
fn invoke(py: Python<'_>, op: u32, input: &str) -> PyResult<String> {
    py.detach(|| aux_zig::native::invoke(op, input.as_bytes()))
        .map_err(native_error)
}

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("ConnectorError", module.py().get_type::<ConnectorError>())?;
    module.add(
        "AuthenticationError",
        module.py().get_type::<AuthenticationError>(),
    )?;
    module.add("StreamGapError", module.py().get_type::<StreamGapError>())?;
    module.add("TransportError", module.py().get_type::<TransportError>())?;
    module.add("ExchangeError", module.py().get_type::<ExchangeError>())?;
    module.add("DecodeError", module.py().get_type::<DecodeError>())?;
    module.add("ValidationError", module.py().get_type::<ValidationError>())?;
    module.add(
        "OutcomeUncertainError",
        module.py().get_type::<OutcomeUncertainError>(),
    )?;
    module.add_class::<EventDecoder>()?;
    module.add_class::<PriceStream>()?;
    module.add_function(wrap_pyfunction!(invoke, module)?)
}

#[pyclass]
struct EventDecoder(std::sync::Mutex<aux_zig::hermes::EventDecoder>);

#[pymethods]
impl EventDecoder {
    #[new]
    #[pyo3(signature=(limit=16 * 1024 * 1024))]
    fn new(limit: usize) -> PyResult<Self> {
        aux_zig::hermes::EventDecoder::new(limit)
            .map(|s| Self(s.into()))
            .map_err(native_error)
    }
    fn feed(&mut self, chunk: &[u8]) -> PyResult<Vec<String>> {
        self.0.get_mut().unwrap().feed(chunk).map_err(native_error)
    }
    fn finish(&mut self) -> PyResult<()> {
        self.0.get_mut().unwrap().finish().map_err(native_error)
    }
    fn close(&mut self) {
        self.0.get_mut().unwrap().close();
    }
}

#[pyclass]
struct PriceStream(aux_zig::PriceStream);
#[pymethods]
impl PriceStream {
    #[staticmethod]
    fn open_dbn(py: Python<'_>, input: &[u8]) -> PyResult<Self> {
        py.detach(|| aux_zig::PriceStream::open_dbn(input))
            .map(Self)
            .map_err(native_error)
    }
    #[staticmethod]
    fn open(py: Python<'_>, config: &str) -> PyResult<Self> {
        py.detach(|| aux_zig::PriceStream::open_json(config))
            .map(Self)
            .map_err(native_error)
    }
    fn command_json(&self, py: Python<'_>, config: &str) -> PyResult<String> {
        py.detach(|| self.0.command_json(config))
            .map_err(native_error)
    }
    fn next_bytes<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, pyo3::types::PyBytes>>> {
        py.detach(|| self.0.next_bytes())
            .map(|data| data.map(|v| pyo3::types::PyBytes::new(py, &v)))
            .map_err(native_error)
    }
    fn next_json(&self, py: Python<'_>) -> PyResult<Option<String>> {
        py.detach(|| self.0.next_json()).map_err(native_error)
    }
    fn close(&self) {
        self.0.close();
    }
}
