use crate::prelude::*;
use crate::{
    convert, errors,
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{Exchange, StandardTransport, phoenix as v};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use zeroize::Zeroizing;
mod operations;
fn native(e: aux_rs::Error<v::Error>) -> crate::errors::Error {
    errors::phoenix(e)
}
fn local(e: v::Error) -> crate::errors::Error {
    native(aux_rs::Error::Exchange(e))
}
fn input<T: DeserializeOwned>(value: Option<Input>) -> Result<T> {
    let value = value
        .as_ref()
        .map(convert::from_input)
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
pub struct PhoenixClient {
    state: Arc<ClientState<Resources>>,
}
impl PhoenixClient {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        chain_id: Option<String>,
        api_key: Option<String>,
        access_token: Option<String>,
        expires_at: Option<u64>,
        timeout_ms: u64,
        subscription_capacity: usize,
    ) -> Result<Self> {
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
        let public = state::build(|| v::PhoenixClient::with_config(exchange, config))?.with_auth(
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
    pub(crate) fn closed(&self) -> bool {
        self.state.is_closed()
    }
    pub(crate) fn set_credentials(&self, credentials: Input) -> Result<()> {
        let value = convert::from_input(&credentials)?;
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
    pub(crate) async fn subscribe(&self, specification: Input) -> Result<Subscription> {
        let specification: v::Subscription = convert::parse(&specification)?;
        let client = self.state.client()?.public;
        self.state
            .stream(async move {
                client
                    .subscribe(specification)
                    .await
                    .map(NativeHandle::Phoenix)
                    .map_err(native)
            })
            .await
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
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
        callback: Callback,
    },
}
#[derive(Clone)]
struct MessageSigner {
    inner: Arc<MessageKind>,
}
impl MessageSigner {}
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
                let result = crate::signers::callback(callback, value)
                    .await
                    .map_err(|_| v::Error::Signing("signer callback failed"))?;
                let text: String = serde_json::from_value(
                    convert::from_input(&result)
                        .map_err(|_| v::Error::Signing("invalid callback signature"))?,
                )
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
#[derive(Clone)]
pub struct PhoenixSigner {
    inner: MessageSigner,
}
impl PhoenixSigner {
    pub(crate) fn new(private_key: String) -> Result<Self> {
        Ok(Self {
            inner: MessageSigner {
                inner: Arc::new(MessageKind::Local(
                    v::LocalSigner::from_hex(Zeroizing::new(private_key)).map_err(local)?,
                )),
            },
        })
    }
    pub(crate) fn remote(public_key: String, callback: Callback) -> Result<Self> {
        Ok(Self {
            inner: MessageSigner {
                inner: Arc::new(MessageKind::Remote {
                    public_key: v::public_key(&public_key).map_err(local)?,
                    callback,
                }),
            },
        })
    }
    pub(crate) fn public_key(&self) -> String {
        bs58::encode(self.inner.public_key()).into_string()
    }
    pub(crate) async fn sign_message(&self, message_base64: String) -> Result<String> {
        use base64::Engine;
        let message = base64::engine::general_purpose::STANDARD
            .decode(message_base64)
            .map_err(|_| errors::ValidationError::new_err("invalid base64 message"))?;
        let signature = self.inner.sign(&message).await.map_err(local)?;
        Ok(bs58::encode(signature).into_string())
    }
    pub(crate) async fn sign_transaction(
        &self,
        prepared: Input,
        signatures: Option<Input>,
    ) -> Result<Output> {
        let prepared: v::PreparedTransaction = convert::parse(&prepared)?;
        let signatures: std::collections::BTreeMap<String, String> = input(signatures)?;
        convert::output(
            prepared
                .sign(&self.inner, signatures)
                .await
                .map_err(local)?,
        )
    }
}

impl PhoenixClient {
    pub(crate) fn build_instruction(&self, name: &str, request: Input) -> Result<Output> {
        self.state.client()?;
        convert::output(v::build_instruction(name, convert::from_input(&request)?).map_err(local)?)
    }
    pub(crate) fn prepare_transaction(&self, request: Input) -> Result<Output> {
        self.state.client()?;
        let value = convert::from_input(&request)?;
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
    pub(crate) fn assemble_transaction(
        &self,
        prepared: Input,
        signatures: Input,
    ) -> Result<Output> {
        self.state.client()?;
        let prepared: v::PreparedTransaction = convert::parse(&prepared)?;
        let signatures = convert::parse(&signatures)?;
        convert::output(prepared.assemble(&signatures).map_err(local)?)
    }
    pub(crate) fn validate_server_transaction(
        &self,
        prepared: Input,
        transaction_base64: String,
    ) -> Result<()> {
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
pub struct PhoenixChainClient {
    state: Arc<ClientState<ChainResources>>,
}
impl PhoenixChainClient {
    pub(crate) fn new(rpc_url: String, timeout_ms: u64) -> Result<Self> {
        let config = aux_rs::TransportConfig {
            timeout: std::time::Duration::from_millis(timeout_ms),
            ..Default::default()
        };
        let transport = state::build(|| StandardTransport::new(config))?;
        let public = v::PhoenixChainClient::with_transport(&rpc_url, transport).map_err(local)?;
        Ok(Self {
            state: ClientState::new(ChainResources { public }),
        })
    }
    pub(crate) fn closed(&self) -> bool {
        self.state.is_closed()
    }
    pub(crate) fn pending(&self) -> Result<Output> {
        convert::output(self.state.client()?.public.pending())
    }
    pub(crate) fn clear_reconciled_submission(&self) -> Result<()> {
        self.state.client()?.public.clear_reconciled_submission();
        Ok(())
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
    }
    pub(crate) async fn latest_blockhash(&self) -> Result<Output> {
        let client = self.state.client()?.public;
        self.state
            .run(async move { client.latest_blockhash().await.map_err(native) })
            .await
    }
    pub(crate) async fn block_height(&self) -> Result<Output> {
        let client = self.state.client()?.public;
        self.state
            .run(async move { client.block_height().await.map_err(native) })
            .await
    }
    pub(crate) async fn signature_status(&self, signature: String) -> Result<Output> {
        let client = self.state.client()?.public;
        self.state
            .run(async move { client.signature_status(&signature).await.map_err(native) })
            .await
    }
    pub(crate) async fn transaction(&self, signature: String) -> Result<Output> {
        let client = self.state.client()?.public;
        self.state
            .run(async move { client.transaction(&signature).await.map_err(native) })
            .await
    }
    pub(crate) async fn simulate(&self, signed: Input) -> Result<Output> {
        let signed: v::SignedTransaction = convert::parse(&signed)?;
        let client = self.state.client()?.public;
        self.state
            .run(async move { client.simulate(&signed).await.map_err(native) })
            .await
    }
    pub(crate) async fn submit(&self, signed: Input) -> Result<Output> {
        let signed: v::SignedTransaction = convert::parse(&signed)?;
        let client = self.state.client()?.public;
        self.state
            .run(async move { client.submit(signed).await.map_err(native) })
            .await
    }
    pub(crate) async fn prepare(&self, instructions: Input, fee_payer: String) -> Result<Output> {
        let instructions: Vec<v::instructions::types::Instruction> = convert::parse(&instructions)?;
        let client = self.state.client()?.public;
        self.state
            .run(async move {
                client
                    .prepare(&instructions, &fee_payer)
                    .await
                    .map_err(native)
            })
            .await
    }
}
impl Drop for PhoenixChainClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
