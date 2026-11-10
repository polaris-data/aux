use crate::{
    convert, errors,
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{Exchange, StandardTransport, phoenix as v};
use pyo3::{prelude::*, types::PyAny};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use zeroize::Zeroizing;
mod operations;
fn native(e: aux_rs::Error<v::Error>) -> PyErr {
    errors::phoenix(e)
}
fn local(e: v::Error) -> PyErr {
    native(aux_rs::Error::Exchange(e))
}
fn input<T: DeserializeOwned>(value: Option<Bound<'_, PyAny>>) -> PyResult<T> {
    let value = value
        .as_ref()
        .map(convert::from_python)
        .transpose()?
        .unwrap_or(serde_json::json!({}));
    serde_json::from_value(value)
        .map_err(|_| errors::ValidationError::new_err("request does not match native schema"))
}
#[derive(Clone)]
struct Resources {
    public: v::PhoenixClient<StandardTransport, v::Credentials>,
}
impl state::ClientResources for Resources {
    fn transport(&self) -> &StandardTransport {
        self.public.transport()
    }
}
#[pyclass(module = "aux_py._native")]
pub struct PhoenixClient {
    state: Arc<ClientState<Resources>>,
}
#[pymethods]
impl PhoenixClient {
    #[new]
    #[pyo3(signature=(*,network="mainnet",http_url=None,ws_url=None,chain_id=None,api_key=None,access_token=None,expires_at=None,timeout_ms=10000,subscription_capacity=128))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        chain_id: Option<String>,
        api_key: Option<String>,
        access_token: Option<String>,
        expires_at: Option<u64>,
        timeout_ms: u64,
        subscription_capacity: usize,
    ) -> PyResult<Self> {
        let base = match network {
            "mainnet" => v::Phoenix::mainnet(),
            _ => return Err(errors::ValidationError::new_err("unsupported network")),
        };
        let custom = http_url.is_some() || ws_url.is_some();
        if custom && chain_id.is_none() {
            return Err(errors::ValidationError::new_err(
                "custom endpoints require explicit chain_id",
            ));
        }
        let exchange = if !custom && chain_id.is_none() {
            base
        } else {
            v::Phoenix::custom(
                http_url.unwrap_or_else(|| base.http_url().into()),
                ws_url.unwrap_or_else(|| base.ws_url().into()),
                chain_id.unwrap_or(base.chain_id),
            )
            .map_err(local)?
        };
        let config = aux_rs::TransportConfig {
            timeout: std::time::Duration::from_millis(timeout_ms),
            subscription_capacity,
            ..Default::default()
        };
        let public = state::build(py, || v::PhoenixClient::with_config(exchange, config))?
            .with_auth(
                v::Credentials::new(
                    api_key.map(Zeroizing::new),
                    access_token.map(Zeroizing::new),
                    expires_at,
                )
                .map_err(local)?,
            );
        Ok(Self {
            state: ClientState::new(Resources { public }),
        })
    }
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    fn set_credentials(&self, credentials: Bound<'_, PyAny>) -> PyResult<()> {
        let value = convert::from_python(&credentials)?;
        let key = value["api_key"]
            .as_str()
            .map(|s| Zeroizing::new(s.to_owned()));
        let token = value["access_token"]
            .as_str()
            .map(|s| Zeroizing::new(s.to_owned()));
        let c = v::Credentials::new(key, token, value["expires_at"].as_u64()).map_err(local)?;
        self.state.client()?.public.auth().replace(c);
        Ok(())
    }
    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        specification: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let specification: v::Subscription = convert::parse(&specification)?;
        let client = self.state.client()?.public;
        self.state.stream(py, async move {
            client
                .subscribe(specification)
                .await
                .map(NativeHandle::Phoenix)
                .map_err(native)
        })
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
}
impl Drop for PhoenixClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}

use v::Signer as _;
enum MessageKind {
    Local(v::LocalSigner),
    Remote {
        public_key: [u8; 32],
        callback: Py<PyAny>,
    },
}
#[derive(Clone)]
struct MessageSigner {
    inner: Arc<MessageKind>,
    locals: Option<pyo3_async_runtimes::TaskLocals>,
}
impl MessageSigner {
    fn scoped(&self, py: Python<'_>) -> PyResult<Self> {
        Ok(Self {
            inner: self.inner.clone(),
            locals: Some(pyo3_async_runtimes::tokio::get_current_locals(py)?),
        })
    }
}
impl v::Signer for MessageSigner {
    fn public_key(&self) -> [u8; 32] {
        match &*self.inner {
            MessageKind::Local(s) => s.public_key(),
            MessageKind::Remote { public_key, .. } => *public_key,
        }
    }
    async fn sign(&self, message: &[u8]) -> std::result::Result<[u8; 64], v::Error> {
        let signature = match &*self.inner {
            MessageKind::Local(s) => s.sign(message).await?,
            MessageKind::Remote { callback, .. } => {
                use base64::Engine;
                let value=convert::output(serde_json::json!({"message_base64":base64::engine::general_purpose::STANDARD.encode(message),"public_key":bs58::encode(self.public_key()).into_string(),"scheme":"phoenix_ed25519"})).map_err(|_|v::Error::Signing("callback payload conversion failed"))?;
                let result =
                    crate::signers::callback_with_locals(callback, value, self.locals.as_ref())
                        .await
                        .map_err(|_| v::Error::Signing("signer callback failed"))?;
                let text: String = Python::attach(|py| result.bind(py).extract())
                    .map_err(|_| v::Error::Signing("callback must return base58 signature"))?;
                bs58::decode(text)
                    .into_vec()
                    .map_err(|_| v::Error::Signing("invalid base58 signature"))?
                    .try_into()
                    .map_err(|_| v::Error::Signing("64-byte signature required"))?
            }
        };
        v::verify_signature(&self.public_key(), message, &signature)?;
        Ok(signature)
    }
}
#[pyclass(module = "aux_py._native", frozen)]
#[derive(Clone)]
pub struct PhoenixSigner {
    inner: MessageSigner,
}
#[pymethods]
impl PhoenixSigner {
    #[new]
    fn new(private_key: String) -> PyResult<Self> {
        Ok(Self {
            inner: MessageSigner {
                inner: Arc::new(MessageKind::Local(
                    v::LocalSigner::from_hex(Zeroizing::new(private_key)).map_err(local)?,
                )),
                locals: None,
            },
        })
    }
    #[staticmethod]
    fn remote(public_key: String, callback: Bound<'_, PyAny>) -> PyResult<Self> {
        if !callback.is_callable() {
            return Err(errors::ValidationError::new_err(
                "callback must be callable",
            ));
        }
        Ok(Self {
            inner: MessageSigner {
                inner: Arc::new(MessageKind::Remote {
                    public_key: v::public_key(&public_key).map_err(local)?,
                    callback: callback.unbind(),
                }),
                locals: None,
            },
        })
    }
    #[getter]
    fn public_key(&self) -> String {
        bs58::encode(self.inner.public_key()).into_string()
    }
    fn sign_message<'py>(
        &self,
        py: Python<'py>,
        message_base64: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        use base64::Engine;
        let message = base64::engine::general_purpose::STANDARD
            .decode(message_base64)
            .map_err(|_| errors::ValidationError::new_err("invalid base64 message"))?;
        let signer = self.inner.scoped(py)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let signature = signer.sign(&message).await.map_err(local)?;
            Ok(bs58::encode(signature).into_string())
        })
    }
    #[pyo3(signature=(prepared,signatures=None))]
    fn sign_transaction<'py>(
        &self,
        py: Python<'py>,
        prepared: Bound<'_, PyAny>,
        signatures: Option<Bound<'_, PyAny>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let prepared: v::PreparedTransaction = convert::parse(&prepared)?;
        let signatures: std::collections::BTreeMap<String, String> = input(signatures)?;
        let signer = self.inner.scoped(py)?;
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            convert::output(prepared.sign(&signer, signatures).await.map_err(local)?)
        })
    }
}

#[pymethods]
impl PhoenixClient {
    fn build_instruction(&self, name: &str, request: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        self.state.client()?;
        convert::output(v::build_instruction(name, convert::from_python(&request)?).map_err(local)?)
    }
    fn prepare_transaction(&self, request: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        self.state.client()?;
        let value = convert::from_python(&request)?;
        let instructions: Vec<v::instructions::types::Instruction> =
            serde_json::from_value(value["instructions"].clone())
                .map_err(|_| errors::ValidationError::new_err("instructions required"))?;
        let payer = value["fee_payer"]
            .as_str()
            .ok_or_else(|| errors::ValidationError::new_err("fee_payer required"))?;
        let hash = value["recent_blockhash"]
            .as_str()
            .ok_or_else(|| errors::ValidationError::new_err("recent_blockhash required"))?;
        let height = value["last_valid_block_height"]
            .as_u64()
            .ok_or_else(|| errors::ValidationError::new_err("last_valid_block_height required"))?;
        convert::output(if value["version"] == "v0" {
            let tables: Vec<v::LookupTable> = serde_json::from_value(
                value
                    .get("lookup_tables")
                    .cloned()
                    .unwrap_or_else(|| serde_json::json!([])),
            )
            .map_err(|_| errors::ValidationError::new_err("invalid lookup_tables"))?;
            v::PreparedTransaction::new_v0(&instructions, payer, hash, height, &tables)
                .map_err(local)?
        } else {
            v::PreparedTransaction::new(&instructions, payer, hash, height).map_err(local)?
        })
    }
    fn assemble_transaction(
        &self,
        prepared: Bound<'_, PyAny>,
        signatures: Bound<'_, PyAny>,
    ) -> PyResult<Py<PyAny>> {
        self.state.client()?;
        let prepared: v::PreparedTransaction = convert::parse(&prepared)?;
        let signatures = convert::parse(&signatures)?;
        convert::output(prepared.assemble(&signatures).map_err(local)?)
    }
    fn validate_server_transaction(
        &self,
        prepared: Bound<'_, PyAny>,
        transaction_base64: String,
    ) -> PyResult<()> {
        self.state.client()?;
        let prepared: v::PreparedTransaction = convert::parse(&prepared)?;
        prepared
            .validate_server_transaction(&transaction_base64)
            .map_err(local)
    }
}

#[derive(Clone)]
struct ChainResources {
    public: v::PhoenixChainClient<StandardTransport>,
}
impl state::ClientResources for ChainResources {
    fn transport(&self) -> &StandardTransport {
        self.public.transport()
    }
}
#[pyclass(module = "aux_py._native")]
pub struct PhoenixChainClient {
    state: Arc<ClientState<ChainResources>>,
}
#[pymethods]
impl PhoenixChainClient {
    #[new]
    #[pyo3(signature=(rpc_url,*,timeout_ms=10000))]
    fn new(py: Python<'_>, rpc_url: String, timeout_ms: u64) -> PyResult<Self> {
        let config = aux_rs::TransportConfig {
            timeout: std::time::Duration::from_millis(timeout_ms),
            ..Default::default()
        };
        let transport = state::build(py, || StandardTransport::new(config))?;
        let public = v::PhoenixChainClient::with_transport(&rpc_url, transport).map_err(local)?;
        Ok(Self {
            state: ClientState::new(ChainResources { public }),
        })
    }
    #[getter]
    fn closed(&self) -> bool {
        self.state.is_closed()
    }
    #[getter]
    fn pending(&self) -> PyResult<Py<PyAny>> {
        convert::output(self.state.client()?.public.pending())
    }
    fn clear_reconciled_submission(&self) -> PyResult<()> {
        self.state.client()?.public.clear_reconciled_submission();
        Ok(())
    }
    fn aclose<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.state.close(py)
    }
    fn latest_blockhash<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?.public;
        self.state.run(py, async move {
            client.latest_blockhash().await.map_err(native)
        })
    }
    fn block_height<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?.public;
        self.state.run(
            py,
            async move { client.block_height().await.map_err(native) },
        )
    }
    fn signature_status<'py>(
        &self,
        py: Python<'py>,
        signature: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?.public;
        self.state.run(py, async move {
            client.signature_status(&signature).await.map_err(native)
        })
    }
    fn transaction<'py>(&self, py: Python<'py>, signature: String) -> PyResult<Bound<'py, PyAny>> {
        let client = self.state.client()?.public;
        self.state.run(py, async move {
            client.transaction(&signature).await.map_err(native)
        })
    }
    fn simulate<'py>(
        &self,
        py: Python<'py>,
        signed: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let signed: v::SignedTransaction = convert::parse(&signed)?;
        let client = self.state.client()?.public;
        self.state.run(
            py,
            async move { client.simulate(&signed).await.map_err(native) },
        )
    }
    fn submit<'py>(
        &self,
        py: Python<'py>,
        signed: Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let signed: v::SignedTransaction = convert::parse(&signed)?;
        let client = self.state.client()?.public;
        self.state.run(
            py,
            async move { client.submit(signed).await.map_err(native) },
        )
    }
    fn prepare<'py>(
        &self,
        py: Python<'py>,
        instructions: Bound<'_, PyAny>,
        fee_payer: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let instructions: Vec<v::instructions::types::Instruction> = convert::parse(&instructions)?;
        let client = self.state.client()?.public;
        self.state.run(py, async move {
            client
                .prepare(&instructions, &fee_payer)
                .await
                .map_err(native)
        })
    }
}
impl Drop for PhoenixChainClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
