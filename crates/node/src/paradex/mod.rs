use crate::prelude::*;
use crate::{
    convert, errors,
    state::{self, ClientState},
    subscription::NativeHandle,
};
use aux_rs::{Exchange, StandardTransport, paradex as v};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use v::Signer as _;
use zeroize::Zeroizing;
mod operations;
mod rpc;
pub use rpc::ParadexRpcSession;
fn native(error: aux_rs::Error<v::Error>) -> crate::errors::Error {
    errors::paradex(error)
}
fn local(error: v::Error) -> crate::errors::Error {
    native(aux_rs::Error::Exchange(error))
}
fn input<T: DeserializeOwned>(value: Option<Input>) -> Result<T> {
    let value = value
        .as_ref()
        .map(convert::from_input)
        .transpose()?
        .unwrap_or(serde_json::json!({}));
    serde_json::from_value(value).map_err(|_| {
        errors::ValidationError::new_err("request does not match the native Paradex schema")
    })
}
enum Kind {
    Stark(v::LocalStarkSigner),
    Evm(v::LocalEvmSigner),
    Remote {
        kind: v::SignerKind,
        public_key: String,
        callback: Callback,
    },
    None,
}
#[derive(Clone)]
pub(crate) struct Signer {
    inner: Arc<Kind>,
}
impl v::Signer for Signer {
    fn kind(&self) -> v::SignerKind {
        match &*self.inner {
            Kind::Stark(_) | Kind::None => v::SignerKind::Starknet,
            Kind::Evm(_) => v::SignerKind::Evm,
            Kind::Remote { kind, .. } => *kind,
        }
    }
    fn public_key(&self) -> String {
        match &*self.inner {
            Kind::Stark(s) => s.public_key(),
            Kind::Evm(s) => s.public_key(),
            Kind::Remote { public_key, .. } => public_key.clone(),
            Kind::None => String::new(),
        }
    }
    async fn sign(&self, payload: &v::SigningPayload) -> Result<Vec<String>, v::Error> {
        match &*self.inner {
            Kind::Stark(s) => s.sign(payload).await,
            Kind::Evm(s) => s.sign(payload).await,
            Kind::None => Err(v::Error::Signing("no signer attached")),
            Kind::Remote { callback, .. } => {
                let value=convert::output(serde_json::json!({"digest":format!("0x{}",aux_rs::hex::encode(payload.digest)),"scheme":match payload.scheme{v::SigningScheme::StarknetMessage=>"starknet_message",v::SigningScheme::EvmPersonal=>"evm_personal"},"typed_data":payload.typed_data,"personal_message":payload.personal_message})).map_err(|_|v::Error::Signing("callback payload conversion failed"))?;
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
pub struct ParadexSigner {
    inner: Signer,
}
impl ParadexSigner {
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
            _ => return Err(errors::ValidationError::new_err("invalid signing scheme")),
        };
        let payload = v::SigningPayload {
            digest,
            scheme,
            typed_data: value.get("typed_data").filter(|v| !v.is_null()).cloned(),
            personal_message: value["personal_message"].as_str().map(str::to_owned),
        };
        let signer = self.inner.clone();
        convert::future_output(async move { signer.sign(&payload).await.map_err(local) }).await
    }
    pub(crate) fn __repr__(&self) -> &'static str {
        "ParadexSigner()"
    }
}
#[derive(Clone)]
struct Resources {
    public: v::ParadexClient<StandardTransport>,
    auth: Option<v::ParadexAuth<Signer>>,
}
impl state::ClientResources for Resources {
    fn transport(&self) -> &StandardTransport {
        self.public.transport()
    }
}
impl Resources {
    pub(crate) fn authenticated(
        &self,
    ) -> Result<v::ParadexClient<StandardTransport, v::ParadexAuth<Signer>>> {
        let auth = self.auth.as_ref().ok_or_else(|| {
            errors::AuthenticationError::new_err("attach a Paradex account and signer or token")
        })?;
        Ok(self
            .public
            .clone()
            .with_auth(auth.with_signer(auth.signer().clone()).map_err(local)?))
    }
}
pub struct ParadexClient {
    state: Arc<ClientState<Resources>>,
}
impl ParadexClient {
    pub(crate) async fn open_rpc(&self) -> Result<ParadexRpcSession> {
        let client = self.state.client()?.authenticated()?;
        let state = self.state.clone();
        self.state
            .object(async move {
                let inner = client.open_rpc().await.map_err(native)?;
                context::evaluate(|| {
                    context::new(ParadexRpcSession {
                        inner: Arc::new(tokio::sync::Mutex::new(inner)),
                        state,
                        closing: tokio::sync::watch::channel(false).0,
                    })
                })
            })
            .await
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        network: &str,
        http_url: Option<String>,
        ws_url: Option<String>,
        chain_id: Option<String>,
        evm_chain_id: Option<u64>,
        siwe_domain: Option<String>,
        siwe_uri: Option<String>,
        signer: Option<&ParadexSigner>,
        account: Option<String>,
        subkey: Option<String>,
        token: Option<String>,
        read_only: bool,
        timeout_ms: u64,
        subscription_capacity: usize,
        read_retries: usize,
        reconnect_retries: usize,
        initial_backoff_ms: u64,
        max_backoff_ms: u64,
        heartbeat_interval_ms: u64,
        idle_timeout_ms: u64,
        max_message_bytes: usize,
        reconnect_stable_ms: u64,
    ) -> Result<Self> {
        let base = match network {
            "testnet" => v::Paradex::testnet(),
            "mainnet" => v::Paradex::mainnet(),
            _ => {
                return Err(errors::ValidationError::new_err(
                    "network must be testnet or mainnet",
                ));
            }
        };
        if signer.is_some() && (http_url.is_some() || ws_url.is_some()) && chain_id.is_none() {
            return Err(errors::ValidationError::new_err(
                "custom authenticated endpoints require chain_id",
            ));
        }
        if signer
            .as_ref()
            .is_some_and(|s| s.inner.kind() == v::SignerKind::Evm)
            && (http_url.is_some() || ws_url.is_some())
            && (evm_chain_id.is_none() || siwe_domain.is_none() || siwe_uri.is_none())
        {
            return Err(errors::ValidationError::new_err(
                "custom EVM authentication requires evm_chain_id, siwe_domain, and siwe_uri",
            ));
        }
        let exchange = v::Paradex::custom(
            http_url.unwrap_or_else(|| base.http_url().into()),
            ws_url.unwrap_or_else(|| base.ws_url().into()),
            chain_id.unwrap_or(base.chain_id),
            evm_chain_id.unwrap_or(base.evm_chain_id),
            siwe_domain.unwrap_or(base.siwe_domain),
            siwe_uri.unwrap_or(base.siwe_uri),
        );
        let config = state::config(
            timeout_ms,
            subscription_capacity,
            read_retries,
            reconnect_retries,
            initial_backoff_ms,
            max_backoff_ms,
            heartbeat_interval_ms,
            idle_timeout_ms,
            max_message_bytes,
            reconnect_stable_ms,
        )?;
        let public = state::build(|| v::ParadexClient::with_config(exchange, config))?;
        let auth = if signer.is_some() || token.is_some() || account.is_some() {
            let account = account.ok_or_else(|| {
                errors::ValidationError::new_err("account is required with signer or token")
            })?;
            let s = signer.map(|s| s.inner.clone()).unwrap_or(Signer {
                inner: Arc::new(Kind::None),
            });
            let auth =
                v::ParadexAuth::new(public.exchange(), &account, s, subkey).map_err(local)?;
            if let Some(token) = token {
                auth.tokens()
                    .set(Zeroizing::new(token), read_only)
                    .map_err(local)?;
            }
            Some(auth)
        } else {
            None
        };
        Ok(Self {
            state: ClientState::new(Resources { public, auth }),
        })
    }
    pub(crate) fn closed(&self) -> bool {
        self.state.is_closed()
    }
    pub(crate) fn authenticated(&self) -> Result<bool> {
        Ok(self.state.client()?.auth.is_some())
    }
    pub(crate) async fn authenticate(&self) -> Result<Output> {
        let client = self.state.client()?.authenticated()?;
        self.state
            .run(async move { client.authenticate().await.map_err(native) })
            .await
    }
    pub(crate) fn set_token(&self, token: String, read_only: bool) -> Result<()> {
        self.state
            .client()?
            .auth
            .as_ref()
            .ok_or_else(|| errors::AuthenticationError::new_err("configure an account first"))?
            .tokens()
            .set(Zeroizing::new(token), read_only)
            .map_err(local)
    }
    pub(crate) fn onboarding_typed_data(&self) -> Result<Output> {
        let client = self.state.client()?.public;
        convert::output(v::onboarding_typed_data(client.exchange()).map_err(local)?)
    }
    pub(crate) async fn sign_siwe(
        &self,
        statement: String,
        nonce: String,
        issued_at: String,
        expiration: Option<String>,
    ) -> Result<Output> {
        let client = self.state.client()?.authenticated()?;
        self.state
            .run(async move {
                client
                    .auth()
                    .sign_siwe(
                        client.exchange(),
                        &statement,
                        &nonce,
                        &issued_at,
                        expiration.as_deref(),
                    )
                    .await
                    .map_err(local)
            })
            .await
    }
    pub(crate) async fn evm_account_address(
        &self,
        address: String,
        class_hash: String,
    ) -> Result<Output> {
        self.state.client()?;
        self.state
            .run(async move { v::evm_account_address(&address, &class_hash).map_err(local) })
            .await
    }
    pub(crate) async fn sign_typed_data(&self, data: Input) -> Result<Output> {
        let data = convert::from_input(&data)?;
        let client = self.state.client()?.authenticated()?;
        self.state
            .run(async move {
                client
                    .auth()
                    .sign_typed_data(client.exchange(), data)
                    .await
                    .map_err(local)
            })
            .await
    }
    pub(crate) async fn block_signature(
        &self,
        nonce: String,
        expiration: u64,
        trades: Input,
        parent: Option<String>,
    ) -> Result<Output> {
        let trades = convert::from_input(&trades)?;
        let client = self.state.client()?.authenticated()?;
        self.state
            .run(async move {
                client
                    .auth()
                    .block_signature(
                        client.exchange(),
                        &nonce,
                        expiration,
                        &trades,
                        parent.as_deref(),
                    )
                    .await
                    .map_err(local)
            })
            .await
    }
    pub(crate) async fn subscribe(&self, specification: Input) -> Result<Subscription> {
        let spec: v::Subscription = convert::parse(&specification)?;
        spec.channel().map_err(local)?;
        if spec.is_private() {
            let client = self.state.client()?.authenticated()?;
            self.state
                .stream(async move {
                    client
                        .subscribe_private(spec)
                        .await
                        .map(NativeHandle::Paradex)
                        .map_err(native)
                })
                .await
        } else {
            let client = self.state.client()?.public;
            self.state
                .stream(async move {
                    client
                        .subscribe(spec)
                        .await
                        .map(NativeHandle::Paradex)
                        .map_err(native)
                })
                .await
        }
    }
    pub(crate) async fn aclose(&self) -> Result<()> {
        self.state.close().await
    }
    pub(crate) fn __repr__(&self) -> &'static str {
        "ParadexClient()"
    }
}
impl Drop for ParadexClient {
    fn drop(&mut self) {
        self.state.request_close();
    }
}
