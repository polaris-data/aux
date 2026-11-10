//! Node ownership glue for the Rust Dune client.
use crate::{
    convert, errors,
    prelude::*,
    state::{ClientResources, ClientState},
};
use aux_rs::{StandardTransport, dune as v};
use serde::de::DeserializeOwned;
use std::sync::Arc;
mod operations;
fn native(error: v::Error) -> errors::Error {
    errors::data(error)
}
fn input<T: DeserializeOwned>(value: Option<Input>) -> Result<T> {
    let value = value
        .as_ref()
        .map(convert::from_input)
        .transpose()?
        .unwrap_or(serde_json::json!({}));
    serde_json::from_value(value)
        .map_err(|_| errors::ValidationError::new_err("request does not match native Dune schema"))
}
impl ClientResources for v::DuneClient<StandardTransport> {
    fn transport(&self) -> &StandardTransport {
        self.transport()
    }
}
pub struct DuneClient {
    state: Arc<ClientState<v::DuneClient<StandardTransport>>>,
}
impl Drop for DuneClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
impl DuneClient {
    pub(crate) fn new(api_key: String, api_url: &str, timeout_ms: u64) -> Result<Self> {
        if timeout_ms == 0 {
            return Err(errors::ValidationError::new_err(
                "timeout_ms must be positive",
            ));
        }
        let client = napi::bindgen_prelude::within_runtime_if_available(|| {
            v::DuneClient::with_config(
                api_url,
                api_key,
                aux_rs::TransportConfig {
                    timeout: std::time::Duration::from_millis(timeout_ms),
                    ..Default::default()
                },
            )
        })
        .map_err(native)?;
        Ok(Self {
            state: ClientState::new(client),
        })
    }
    pub(crate) fn closed(&self) -> bool {
        self.state.is_closed()
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
    }
}
