//! Python ownership and asyncio integration for the Hyperdash connector.
use crate::{
    convert, errors,
    state::{ClientResources, ClientState},
};
use aux_rs::{StandardTransport, TransportConfig, hyperdash as v};
use pyo3::prelude::*;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{sync::Arc, time::Duration};

fn native(error: v::Error) -> PyErr {
    match &error {
        v::Error::Transport(error) => errors::transport(error),
        v::Error::InvalidRequest(message) => errors::ValidationError::new_err(*message),
        v::Error::Decode(message) => errors::DecodeError::new_err(*message),
        v::Error::Io(_) => errors::ConnectorError::new_err(error.to_string()),
        _ => Python::attach(|py| -> PyResult<PyErr> {
            let mapped = if matches!(error, v::Error::Api(_)) {
                errors::DataVendorError::new_err(error.to_string())
            } else {
                errors::ConnectorError::new_err(error.to_string())
            };
            let value = mapped.value(py);
            value.setattr("kind", error.kind())?;
            match &error {
                v::Error::Api(error) => {
                    value.setattr("http_status", error.status)?;
                    value.setattr("metadata", convert::output(&error.metadata)?)?;
                    value.setattr("native", convert::output(&error.problem)?)?;
                    value.setattr(
                        "code",
                        error
                            .problem
                            .get("code")
                            .and_then(Value::as_str)
                            .or(error.metadata.problem_code.as_deref()),
                    )?;
                }
                v::Error::StreamClosed { code, reason } => {
                    value.setattr("close_code", *code)?;
                    value.setattr("close_reason", reason.as_deref())?;
                }
                v::Error::StreamError(problem) => {
                    value.setattr("native", convert::output(problem)?)?
                }
                _ => {}
            }
            Ok(mapped)
        })
        .unwrap_or_else(std::convert::identity),
    }
}

fn input<T: DeserializeOwned>(request: &Bound<'_, PyAny>) -> PyResult<T> {
    let value = convert::from_python_with_decimal(request)?;
    serde_json::from_value(value).map_err(|_| {
        errors::ValidationError::new_err("request does not match the Hyperdash schema")
    })
}

impl ClientResources for v::HyperdashClient<StandardTransport> {
    fn transport(&self) -> &StandardTransport {
        self.transport()
    }
}

#[pyclass(module = "aux_py._native", frozen)]
pub struct HyperdashClient {
    state: Arc<ClientState<v::HyperdashClient<StandardTransport>>>,
}

impl Drop for HyperdashClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}

#[pymethods]
impl HyperdashClient {
    #[new]
    #[pyo3(signature=(api_key, api_url, ws_url, timeout_ms, stream_capacity))]
    fn new(
        py: Python<'_>,
        api_key: Option<String>,
        api_url: String,
        ws_url: String,
        timeout_ms: u64,
        stream_capacity: usize,
    ) -> PyResult<Self> {
        let timeout = Duration::from_millis(timeout_ms);
        let client = py
            .detach(move || {
                let _runtime = pyo3_async_runtimes::tokio::get_runtime().enter();
                v::HyperdashClient::with_config(
                    v::HyperdashConfig {
                        api_key,
                        api_url,
                        ws_url,
                        stream_capacity,
                        stream_ready_timeout: timeout,
                    },
                    TransportConfig {
                        timeout,
                        ..Default::default()
                    },
                )
            })
            .map_err(native)?;
        Ok(Self {
            state: ClientState::new(client),
        })
    }

    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }

    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }

    #[pyo3(signature=(method, request=None))]
    fn request<'py>(
        &self,
        py: Python<'py>,
        method: String,
        request: Option<Bound<'py, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?;
        let request = request
            .as_ref()
            .map(convert::from_python_with_decimal)
            .transpose()?
            .unwrap_or(Value::Null);
        self.state.run(py, async move {
            client.dispatch(&method, request).await.map_err(native)
        })
    }

    fn open_stream<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?;
        self.state.object(py, async move {
            let stream = client.open_stream().await.map_err(native)?;
            Python::attach(|py| Ok(Py::new(py, HyperdashStream(stream))?.into_any()))
        })
    }
}

#[pyclass(module = "aux_py._native", frozen)]
pub struct HyperdashStream(v::HyperdashStream);

fn run<'py, T, F>(py: Python<'py>, future: F) -> PyResult<Bound<'py, PyAny>>
where
    T: Serialize + Send + 'static,
    F: Future<Output = v::Result<T>> + Send + 'static,
{
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        convert::output(future.await.map_err(native)?)
    })
}

#[pymethods]
impl HyperdashStream {
    #[getter]
    fn closed(&self) -> bool {
        self.0.closed()
    }

    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        request: Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let request = input(&request)?;
        let stream = self.0.clone();
        run(py, async move { stream.subscribe(request).await })
    }

    fn next<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.0.clone();
        run(py, async move { stream.next().await })
    }

    fn ack<'py>(
        &self,
        py: Python<'py>,
        sub: Bound<'py, PyAny>,
        batch: Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let sub = input(&sub)?;
        let batch = input(&batch)?;
        let stream = self.0.clone();
        run(py, async move { stream.ack(sub, batch).await })
    }

    fn unsubscribe<'py>(
        &self,
        py: Python<'py>,
        sub: Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let sub = input(&sub)?;
        let stream = self.0.clone();
        run(py, async move { stream.unsubscribe(sub).await })
    }

    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let stream = self.0.clone();
        run(py, async move {
            stream.close().await;
            Ok(())
        })
    }
}
