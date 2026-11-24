use napi_derive::napi;

fn native_error(error: aux_zig::Error) -> napi::Error {
    use aux_zig::Error::*;
    let kind = match &error {
        Network => "TransportError",
        AuthenticationRequired => "AuthenticationError",
        ExchangeResponse(detail) => {
            return napi::Error::from_reason(format!(
                "[aux:ExchangeResponse] {}",
                serde_json::to_string(detail).unwrap_or_default()
            ));
        }
        SessionResponse(detail) => {
            return napi::Error::from_reason(format!(
                "[aux:SessionResponse] {}",
                serde_json::to_string(detail).unwrap_or_default()
            ));
        }
        ExchangeApi => "ExchangeError",
        StreamGap => "StreamGapError",
        PendingOutcome => "OutcomeUncertainError",
        InvalidRequest(_) => "ValidationError",
        Decode(_) => "DecodeError",
        OutOfMemory => "ConnectorError",
    };
    napi::Error::from_reason(format!("[aux:{kind}] {error}"))
}

#[napi]
pub fn invoke(op: u32, input: String) -> napi::Result<String> {
    aux_zig::native::invoke(op, input.as_bytes()).map_err(native_error)
}

pub struct HttpTask(u32, String);

impl napi::Task for HttpTask {
    type Output = String;
    type JsValue = String;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        invoke(self.0, self.1.clone())
    }

    fn resolve(&mut self, _: napi::Env, result: Self::Output) -> napi::Result<Self::JsValue> {
        Ok(result)
    }
}

#[napi]
pub fn execute_http(input: String) -> napi::bindgen_prelude::AsyncTask<HttpTask> {
    napi::bindgen_prelude::AsyncTask::new(HttpTask(12, input))
}

#[napi]
pub fn execute_rpc(input: String) -> napi::bindgen_prelude::AsyncTask<HttpTask> {
    napi::bindgen_prelude::AsyncTask::new(HttpTask(27, input))
}

#[napi]
pub struct EventDecoder(aux_zig::hermes::EventDecoder);

#[napi]
impl EventDecoder {
    #[napi(constructor)]
    pub fn new(limit: Option<u32>) -> napi::Result<Self> {
        aux_zig::hermes::EventDecoder::new(limit.unwrap_or(16 * 1024 * 1024) as usize)
            .map(Self)
            .map_err(native_error)
    }
    #[napi]
    pub fn feed(&mut self, chunk: napi::bindgen_prelude::Uint8Array) -> napi::Result<Vec<String>> {
        self.0.feed(&chunk).map_err(native_error)
    }
    #[napi]
    pub fn finish(&mut self) -> napi::Result<()> {
        self.0.finish().map_err(native_error)
    }
    #[napi]
    pub fn close(&mut self) {
        self.0.close();
    }
}

pub struct OpenStreamTask(String);
pub struct StreamCommandTask(aux_zig::PriceStream, String);
pub struct NextBytesTask(aux_zig::PriceStream);
pub struct NextStreamTask(aux_zig::PriceStream);

#[napi]
pub struct PriceStream(aux_zig::PriceStream);

impl napi::Task for OpenStreamTask {
    type Output = aux_zig::PriceStream;
    type JsValue = PriceStream;
    fn compute(&mut self) -> napi::Result<Self::Output> {
        aux_zig::PriceStream::open_json(&self.0).map_err(native_error)
    }
    fn resolve(&mut self, _: napi::Env, stream: Self::Output) -> napi::Result<PriceStream> {
        Ok(PriceStream(stream))
    }
}
impl napi::Task for StreamCommandTask {
    type Output = String;
    type JsValue = String;
    fn compute(&mut self) -> napi::Result<String> {
        self.0.command_json(&self.1).map_err(native_error)
    }
    fn resolve(&mut self, _: napi::Env, value: String) -> napi::Result<String> {
        Ok(value)
    }
}
impl napi::Task for NextBytesTask {
    type Output = Option<Vec<u8>>;
    type JsValue = Option<napi::bindgen_prelude::Buffer>;
    fn compute(&mut self) -> napi::Result<Self::Output> {
        self.0.next_bytes().map_err(native_error)
    }
    fn resolve(&mut self, _: napi::Env, value: Self::Output) -> napi::Result<Self::JsValue> {
        Ok(value.map(Into::into))
    }
}
impl napi::Task for NextStreamTask {
    type Output = Option<String>;
    type JsValue = Option<String>;
    fn compute(&mut self) -> napi::Result<Self::Output> {
        self.0.next_json().map_err(native_error)
    }
    fn resolve(&mut self, _: napi::Env, result: Self::Output) -> napi::Result<Self::JsValue> {
        Ok(result)
    }
}
#[napi]
pub fn open_stream(config: String) -> napi::bindgen_prelude::AsyncTask<OpenStreamTask> {
    napi::bindgen_prelude::AsyncTask::new(OpenStreamTask(config))
}
#[napi]
impl PriceStream {
    #[napi]
    pub fn command_json(
        &self,
        command: String,
    ) -> napi::bindgen_prelude::AsyncTask<StreamCommandTask> {
        napi::bindgen_prelude::AsyncTask::new(StreamCommandTask(self.0.clone(), command))
    }
    #[napi]
    pub fn next_bytes(&self) -> napi::bindgen_prelude::AsyncTask<NextBytesTask> {
        napi::bindgen_prelude::AsyncTask::new(NextBytesTask(self.0.clone()))
    }
    #[napi]
    pub fn next_json(&self) -> napi::bindgen_prelude::AsyncTask<NextStreamTask> {
        napi::bindgen_prelude::AsyncTask::new(NextStreamTask(self.0.clone()))
    }
    #[napi]
    pub fn close(&self) {
        self.0.close();
    }
}

#[napi]
pub fn execute_metric_chain(input: String) -> napi::bindgen_prelude::AsyncTask<HttpTask> {
    napi::bindgen_prelude::AsyncTask::new(HttpTask(51, input))
}

#[napi]
pub fn execute_phoenix_chain(input: String) -> napi::bindgen_prelude::AsyncTask<HttpTask> {
    napi::bindgen_prelude::AsyncTask::new(HttpTask(48, input))
}

#[napi]
pub fn execute_solana(input: String) -> napi::bindgen_prelude::AsyncTask<HttpTask> {
    napi::bindgen_prelude::AsyncTask::new(HttpTask(37, input))
}

#[napi]
pub fn open_dbn(data: napi::bindgen_prelude::Uint8Array) -> napi::Result<PriceStream> {
    aux_zig::PriceStream::open_dbn(&data)
        .map(PriceStream)
        .map_err(native_error)
}

#[napi]
pub fn execute_evm_chain(input: String) -> napi::bindgen_prelude::AsyncTask<HttpTask> {
    napi::bindgen_prelude::AsyncTask::new(HttpTask(38, input))
}
