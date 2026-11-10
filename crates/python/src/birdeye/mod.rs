//! Ownership and asyncio glue; protocol behavior lives in aux-rs-birdeye.
use crate::{
    convert, errors,
    state::{ClientResources, ClientState},
};
use aux_rs::{StandardTransport, birdeye as v};
use pyo3::prelude::*;
use serde::de::DeserializeOwned;
use std::sync::Arc;
mod operations;
fn native(error: v::Error) -> PyErr {
    errors::data(error)
}
fn input<T: DeserializeOwned>(value: Option<Bound<'_, PyAny>>) -> PyResult<T> {
    let value = value
        .as_ref()
        .map(convert::from_python_with_decimal)
        .transpose()?
        .unwrap_or(serde_json::json!({}));
    serde_json::from_value(value).map_err(|_| {
        errors::ValidationError::new_err("request does not match native Birdeye schema")
    })
}
impl ClientResources for v::BirdeyeClient<StandardTransport> {
    fn transport(&self) -> &StandardTransport {
        self.transport()
    }
}
#[pyclass(module = "aux_py._native", frozen)]
pub struct BirdeyeClient {
    state: Arc<ClientState<v::BirdeyeClient<StandardTransport>>>,
}
impl Drop for BirdeyeClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
#[pymethods]
impl BirdeyeClient {
    #[new]
    #[pyo3(signature=(api_key, *, api_url="https://public-api.birdeye.so", timeout_ms=10000, chain="solana"))]
    fn new(
        py: Python<'_>,
        api_key: String,
        api_url: &str,
        timeout_ms: u64,
        chain: String,
    ) -> PyResult<Self> {
        if timeout_ms == 0 {
            return Err(errors::ValidationError::new_err(
                "timeout_ms must be positive",
            ));
        }
        let client = py
            .detach(move || {
                let _entered = pyo3_async_runtimes::tokio::get_runtime().enter();
                v::BirdeyeClient::with_config(
                    api_url,
                    api_key,
                    aux_rs::TransportConfig {
                        timeout: std::time::Duration::from_millis(timeout_ms),
                        ..Default::default()
                    },
                )?
                .with_chain(chain)
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
}
