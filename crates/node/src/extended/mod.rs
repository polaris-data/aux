use crate::prelude::*;
use crate::{
    convert, errors,
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{Exchange, StandardTransport, extended as v};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use zeroize::Zeroizing;
mod operations;
use v::Signer as _;
fn native(e: aux_rs::Error<v::Error>) -> crate::errors::Error {
    errors::extended(e)
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
    public: v::ExtendedClient<StandardTransport, v::Credentials>,
}
impl state::ClientResources for Resources {
    fn transport(&self) -> &StandardTransport {
        self.public.transport()
    }
}
pub struct ExtendedClient {
    state: Arc<ClientState<Resources>>,
}
impl ExtendedClient {
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
            "mainnet" => v::Extended::mainnet(),
            "testnet" => v::Extended::testnet(),
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
            v::Extended::custom(
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
        let public = state::build(|| v::ExtendedClient::with_config(exchange, config))?.with_auth(
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
                    .map(NativeHandle::Extended)
                    .map_err(native)
            })
            .await
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
    }
}
impl Drop for ExtendedClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}

enum Kind {
    Stark(v::LocalStarkSigner),
    Evm(v::LocalEvmSigner),
    Remote {
        kind: v::SignerKind,
        public_key: String,
        callback: Callback,
    },
}
#[derive(Clone)]
pub(crate) struct Signer {
    inner: Arc<Kind>,
}
impl v::Signer for Signer {
    fn kind(&self) -> v::SignerKind {
        match &*self.inner {
            Kind::Stark(_) => v::SignerKind::Starknet,
            Kind::Evm(_) => v::SignerKind::Evm,
            Kind::Remote { kind, .. } => *kind,
        }
    }
    fn public_key(&self) -> String {
        match &*self.inner {
            Kind::Stark(s) => s.public_key(),
            Kind::Evm(s) => s.public_key(),
            Kind::Remote { public_key, .. } => public_key.clone(),
        }
    }
    async fn sign(&self, payload: &v::SigningPayload) -> Result<Vec<String>, v::Error> {
        match &*self.inner {
            Kind::Stark(s) => s.sign(payload).await,
            Kind::Evm(s) => s.sign(payload).await,
            Kind::Remote { callback, .. } => {
                let value=convert::output(serde_json::json!({"digest":format!("0x{}",aux_rs::hex::encode(payload.digest)),"scheme":match payload.scheme{v::SigningScheme::StarknetMessage=>"starknet_message",v::SigningScheme::EvmPersonal=>"evm_personal",v::SigningScheme::EvmTyped=>"evm_typed"},"typed_data":payload.typed_data,"personal_message":payload.personal_message})).map_err(|_|v::Error::Signing("callback payload conversion failed"))?;
                let result = crate::signers::callback(callback, value)
                    .await
                    .map_err(|_| v::Error::Signing("signer callback failed"))?;
                context::evaluate(|| {
                    let value = convert::from_input(&result)
                        .map_err(|_| v::Error::Signing("invalid callback signature"))?;
                    match value {
                        serde_json::Value::String(s) => Ok(vec![s]),
                        serde_json::Value::Array(xs) => xs
                            .into_iter()
                            .map(|x| match x {
                                serde_json::Value::String(s) => Ok(s),
                                serde_json::Value::Number(n) => Ok(n.to_string()),
                                _ => Err(v::Error::Signing("invalid callback signature")),
                            })
                            .collect(),
                        _ => Err(v::Error::Signing(
                            "callback must return a signature string or list",
                        )),
                    }
                })
            }
        }
    }
}
#[derive(Clone)]
pub struct ExtendedSigner {
    inner: Signer,
}
impl ExtendedSigner {
    pub(crate) fn new(private_key: String, kind: &str) -> Result<Self> {
        let key = Zeroizing::new(private_key);
        let inner = match kind {
            "starknet" => Kind::Stark(v::LocalStarkSigner::from_hex(key).map_err(local)?),
            "evm" => Kind::Evm(v::LocalEvmSigner::from_hex(key).map_err(local)?),
            _ => {
                return Err(errors::ValidationError::new_err(
                    "kind must be starknet or evm",
                ));
            }
        };
        Ok(Self {
            inner: Signer {
                inner: Arc::new(inner),
            },
        })
    }
    pub(crate) fn remote(public_key: &str, callback: Callback, kind: &str) -> Result<Self> {
        let public_key = format!("{:#x}", v::felt(public_key).map_err(local)?);
        let kind = match kind {
            "starknet" => v::SignerKind::Starknet,
            "evm" => v::SignerKind::Evm,
            _ => {
                return Err(errors::ValidationError::new_err(
                    "kind must be starknet or evm",
                ));
            }
        };
        Ok(Self {
            inner: Signer {
                inner: Arc::new(Kind::Remote {
                    kind,
                    public_key,
                    callback,
                }),
            },
        })
    }
    pub(crate) fn public_key(&self) -> String {
        self.inner.public_key()
    }
    pub(crate) async fn sign(&self, payload: Input) -> Result<Output> {
        let value = convert::from_input(&payload)?;
        let mut digest = [0; 32];
        let text = value["digest"]
            .as_str()
            .ok_or_else(|| errors::ValidationError::new_err("digest is required"))?;
        aux_rs::hex::decode_to_slice(text.strip_prefix("0x").unwrap_or(text), &mut digest)
            .map_err(|_| errors::ValidationError::new_err("invalid digest"))?;
        let scheme = match value["scheme"].as_str() {
            Some("starknet_message") => v::SigningScheme::StarknetMessage,
            Some("evm_personal") => v::SigningScheme::EvmPersonal,
            Some("evm_typed") => v::SigningScheme::EvmTyped,
            _ => return Err(errors::ValidationError::new_err("invalid signing scheme")),
        };
        let payload = v::SigningPayload {
            digest,
            scheme,
            typed_data: value.get("typed_data").filter(|v| !v.is_null()).cloned(),
            personal_message: value["personal_message"].as_str().map(str::to_owned),
        };
        let signer = self.inner.clone();
        convert::future_output(async move {
            let signature = signer.sign(&payload).await.map_err(local)?;
            v::verify_signature(&signer, &payload, &signature).map_err(local)?;
            Ok(signature)
        })
        .await
    }
    pub(crate) fn __repr__(&self) -> &'static str {
        "ExtendedSigner()"
    }
}

impl ExtendedClient {
    pub(crate) fn settlement_hash(
        &self,
        kind: &str,
        public_key: &str,
        request: Input,
    ) -> Result<String> {
        let client = self.state.client()?.public;
        let value = convert::from_input(&request)?;
        let hash = match kind {
            "order" => v::order_hash(
                client.exchange(),
                public_key,
                &serde_json::from_value(value)
                    .map_err(v::Error::from)
                    .map_err(local)?,
            ),
            "limit_order" => v::limit_order_hash(
                client.exchange(),
                public_key,
                &serde_json::from_value(value)
                    .map_err(v::Error::from)
                    .map_err(local)?,
            ),
            "transfer" => v::transfer_hash(
                client.exchange(),
                public_key,
                &serde_json::from_value(value)
                    .map_err(v::Error::from)
                    .map_err(local)?,
            ),
            "withdrawal" => v::withdrawal_hash(
                client.exchange(),
                public_key,
                &serde_json::from_value(value)
                    .map_err(v::Error::from)
                    .map_err(local)?,
            ),
            _ => {
                return Err(errors::ValidationError::new_err(
                    "unsupported settlement kind",
                ));
            }
        };
        hash.map_err(local)
    }
    pub(crate) fn prepare_order(
        &self,
        public_key: &str,
        order: Input,
        context: Input,
    ) -> Result<Output> {
        let client = self.state.client()?.public;
        let context = convert::parse(&context)?;
        convert::output(
            v::prepare_order(
                client.exchange(),
                public_key,
                convert::from_input(&order)?,
                &context,
            )
            .map_err(local)?,
        )
    }
    pub(crate) async fn sign_order(
        &self,
        signer: &ExtendedSigner,
        order: Input,
        context: Input,
    ) -> Result<Output> {
        let client = self.state.client()?.public;
        let order = convert::from_input(&order)?;
        let context = convert::parse(&context)?;
        let signer = signer.inner.clone();
        self.state
            .run(async move {
                v::sign_order(client.exchange(), &signer, order, &context)
                    .await
                    .map_err(local)
            })
            .await
    }
    pub(crate) fn onboarding_payload(&self, registration: Input) -> Result<Output> {
        let client = self.state.client()?.public;
        let payload =
            v::onboarding_payload(client.exchange(), &convert::from_input(&registration)?)
                .map_err(local)?;
        convert::output(
            serde_json::json!({"digest":format!("0x{}",aux_rs::hex::encode(payload.digest)),"scheme":"evm_typed","typed_data":payload.typed_data}),
        )
    }
    pub(crate) fn scale_decimal(&self, value: &str, decimals: u32) -> Result<i64> {
        self.state.client()?;
        v::scale_decimal(value, decimals).map_err(local)
    }
}

impl ExtendedSigner {
    pub(crate) fn from_wallet_signature(signature: String) -> Result<Self> {
        Ok(Self {
            inner: Signer {
                inner: Arc::new(Kind::Stark(
                    v::LocalStarkSigner::from_wallet_signature(Zeroizing::new(signature))
                        .map_err(local)?,
                )),
            },
        })
    }
}
impl ExtendedClient {
    pub(crate) fn key_derivation_payload(&self, registration: Input) -> Result<Output> {
        let client = self.state.client()?.public;
        let p = v::key_derivation_payload(client.exchange(), &convert::from_input(&registration)?)
            .map_err(local)?;
        convert::output(
            serde_json::json!({"digest":format!("0x{}",aux_rs::hex::encode(p.digest)),"scheme":"evm_typed","typed_data":p.typed_data}),
        )
    }
    pub(crate) fn api_request_payload(&self, path: &str, timestamp: &str) -> Result<Output> {
        self.state.client()?;
        let p = v::api_request_payload(path, timestamp).map_err(local)?;
        convert::output(
            serde_json::json!({"digest":format!("0x{}",aux_rs::hex::encode(p.digest)),"scheme":"evm_personal","personal_message":p.personal_message}),
        )
    }
    pub(crate) fn onboarding_l2_hash(&self, wallet: &str, public_key: &str) -> Result<String> {
        self.state.client()?;
        v::onboarding_l2_hash(wallet, public_key).map_err(local)
    }
}

impl ExtendedSigner {
    pub(crate) async fn sign_hash(&self, hash: String) -> Result<Output> {
        convert::future_output(async move { v::sign_hash(&self.inner, &hash).await.map_err(local) })
            .await
    }
}
