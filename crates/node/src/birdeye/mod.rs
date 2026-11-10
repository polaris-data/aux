//! Node ownership glue for the Rust Birdeye client.
use crate::{
    convert, errors,
    prelude::*,
    state::{ClientResources, ClientState},
};
use aux_rs::{StandardTransport, birdeye as v};
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
    serde_json::from_value(value).map_err(|_| {
        errors::ValidationError::new_err("request does not match native Birdeye schema")
    })
}
impl ClientResources for v::BirdeyeClient<StandardTransport> {
    fn transport(&self) -> &StandardTransport {
        self.transport()
    }
}
pub struct BirdeyeClient {
    state: Arc<ClientState<v::BirdeyeClient<StandardTransport>>>,
}
impl Drop for BirdeyeClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
impl BirdeyeClient {
    pub(crate) fn new(
        api_key: String,
        api_url: &str,
        timeout_ms: u64,
        chain: String,
    ) -> Result<Self> {
        if timeout_ms == 0 {
            return Err(errors::ValidationError::new_err(
                "timeout_ms must be positive",
            ));
        }
        let client = napi::bindgen_prelude::within_runtime_if_available(|| {
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
    pub(crate) fn closed(&self) -> bool {
        self.state.is_closed()
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
    }
}
