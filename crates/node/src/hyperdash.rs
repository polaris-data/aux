//! Node ownership and conversion for the Hyperdash connector.
use crate::{
    convert, errors,
    state::{ClientResources, ClientState},
};
use aux_rs::{StandardTransport, TransportConfig, hyperdash as v};
use napi_derive::napi;
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;
use std::{sync::Arc, time::Duration};

fn native(error: v::Error) -> errors::Error {
    let mapped = match &error {
        v::Error::Transport(error) => return errors::transport(error),
        v::Error::InvalidRequest(message) => return errors::ValidationError::new_err(*message),
        v::Error::Decode(message) => return errors::DecodeError::new_err(*message),
        v::Error::Api(_) => errors::DataVendorError::new_err(error.to_string()),
        _ => errors::ConnectorError::new_err(error.to_string()),
    };
    let _ = mapped.setattr("kind", error.kind());
    match &error {
        v::Error::Api(error) => {
            let _ = mapped.setattr("http_status", error.status);
            let _ = mapped.setattr("metadata", &error.metadata);
            let _ = mapped.setattr("native", &error.problem);
            let code = error
                .problem
                .get("code")
                .and_then(Value::as_str)
                .or(error.metadata.problem_code.as_deref());
            let _ = mapped.setattr("code", code);
        }
        v::Error::StreamClosed { code, reason } => {
            let _ = mapped.setattr("close_code", code);
            let _ = mapped.setattr("close_reason", reason);
        }
        v::Error::StreamError(problem) => {
            let _ = mapped.setattr("native", problem);
        }
        _ => {}
    }
    mapped
}

fn input<T: DeserializeOwned>(text: &str) -> napi::Result<T> {
    serde_json::from_str(text).map_err(|_| {
        errors::ValidationError::new_err("request does not match the Hyperdash schema").into_napi()
    })
}

impl ClientResources for v::HyperdashClient<StandardTransport> {
    fn transport(&self) -> &StandardTransport {
        self.transport()
    }
}

#[derive(Deserialize)]
struct Options {
    api_key: Option<String>,
    api_url: Option<String>,
    ws_url: Option<String>,
    timeout_ms: Option<u64>,
    stream_capacity: Option<usize>,
}

#[napi(js_name = "HyperdashClient")]
pub struct HyperdashClient {
    state: Arc<ClientState<v::HyperdashClient<StandardTransport>>>,
}

impl Drop for HyperdashClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}

#[napi]
impl HyperdashClient {
    #[napi(constructor)]
    pub fn new(options: String) -> napi::Result<Self> {
        let options: Options = input(&options)?;
        let mut config = v::HyperdashConfig {
            api_key: options.api_key,
            ..Default::default()
        };
        if let Some(api_url) = options.api_url {
            config.api_url = api_url;
        }
        if let Some(ws_url) = options.ws_url {
            config.ws_url = ws_url;
        }
        if let Some(stream_capacity) = options.stream_capacity {
            config.stream_capacity = stream_capacity;
        }
        if let Some(timeout_ms) = options.timeout_ms {
            config.stream_ready_timeout = Duration::from_millis(timeout_ms);
        }
        let transport = TransportConfig {
            timeout: config.stream_ready_timeout,
            ..Default::default()
        };
        let client = napi::bindgen_prelude::within_runtime_if_available(|| {
            v::HyperdashClient::with_config(config, transport)
        })
        .map_err(|error| native(error).into_napi())?;
        Ok(Self {
            state: ClientState::new(client),
        })
    }

    #[napi(getter)]
    pub fn closed(&self) -> bool {
        self.state.is_closed()
    }

    #[napi]
    pub async fn request(&self, method: String, request: String) -> napi::Result<String> {
        let client = self.state.client().map_err(errors::Error::into_napi)?;
        let request = input(&request)?;
        let response = self
            .state
            .object(async move { client.dispatch(&method, request).await.map_err(native) })
            .await
            .map_err(errors::Error::into_napi)?;
        convert::encode(response).map_err(errors::Error::into_napi)
    }

    #[napi]
    pub async fn open_stream(&self) -> napi::Result<HyperdashStream> {
        let client = self.state.client().map_err(errors::Error::into_napi)?;
        self.state
            .object(async move {
                client
                    .open_stream()
                    .await
                    .map(HyperdashStream)
                    .map_err(native)
            })
            .await
            .map_err(errors::Error::into_napi)
    }

    #[napi]
    pub async fn close(&self) -> napi::Result<()> {
        self.state.close().await.map_err(errors::Error::into_napi)
    }
}

#[napi(js_name = "HyperdashStream")]
pub struct HyperdashStream(v::HyperdashStream);

#[derive(Deserialize)]
struct Ack {
    sub: v::StreamId,
    batch: v::StreamId,
}

#[napi]
impl HyperdashStream {
    #[napi(getter)]
    pub fn closed(&self) -> bool {
        self.0.closed()
    }

    #[napi]
    pub async fn subscribe(&self, request: String) -> napi::Result<()> {
        self.0
            .subscribe(input(&request)?)
            .await
            .map_err(|error| native(error).into_napi())
    }

    #[napi]
    pub async fn next(&self) -> napi::Result<String> {
        let message = self
            .0
            .next()
            .await
            .map_err(|error| native(error).into_napi())?;
        convert::encode(message).map_err(errors::Error::into_napi)
    }

    #[napi]
    pub async fn ack(&self, request: String) -> napi::Result<()> {
        let Ack { sub, batch } = input(&request)?;
        self.0
            .ack(sub, batch)
            .await
            .map_err(|error| native(error).into_napi())
    }

    #[napi]
    pub async fn unsubscribe(&self, sub: String) -> napi::Result<()> {
        self.0
            .unsubscribe(input(&sub)?)
            .await
            .map_err(|error| native(error).into_napi())
    }

    #[napi]
    pub async fn close(&self) {
        self.0.close().await;
    }
}
