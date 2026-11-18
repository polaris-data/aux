//! Python ownership and conversion for the native oracle clients.
use crate::{
    convert, errors,
    state::{ClientResources, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{StandardTransport, TransportConfig, chainlink, pyth};
use pyo3::prelude::*;
use serde::de::DeserializeOwned;
use std::{sync::Arc, time::Duration};
mod operations;

fn input<T: DeserializeOwned>(value: Option<Bound<'_, PyAny>>) -> PyResult<T> {
    let value = value
        .as_ref()
        .map(convert::from_python_with_decimal)
        .transpose()?
        .unwrap_or(serde_json::json!({}));
    serde_json::from_value(value).map_err(|_| {
        errors::ValidationError::new_err("request does not match native oracle schema")
    })
}
fn transport_config(timeout: u64) -> pyth::Result<TransportConfig> {
    let config = TransportConfig {
        timeout: Duration::from_millis(timeout),
        ..Default::default()
    };
    config.validate()?;
    Ok(config)
}
fn build<T: Send>(py: Python<'_>, f: impl FnOnce() -> pyth::Result<T> + Send) -> PyResult<T> {
    py.detach(move || {
        let _entered = pyo3_async_runtimes::tokio::get_runtime().enter();
        f()
    })
    .map_err(errors::data)
}
macro_rules! client {
    ($vendor:ident, $name:ident) => {
        impl ClientResources for $vendor::$name<StandardTransport> {
            fn transport(&self) -> &StandardTransport {
                self.transport()
            }
        }
        #[pyclass(module = "aux_py._native", frozen)]
        pub struct $name {
            state: Arc<ClientState<$vendor::$name<StandardTransport>>>,
        }
        impl Drop for $name {
            fn drop(&mut self) {
                self.state.request_close();
            }
        }
    };
}
client!(pyth, PythProClient);
client!(pyth, PythHermesClient);
client!(chainlink, ChainlinkDataStreamsClient);
client!(chainlink, ChainlinkCandlestickClient);
