use crate::prelude::*;
use crate::{convert, errors};
use aux_rs::hex;
use aux_rs::{hyperliquid as hl, lighter as lt};
use hl::Signer as _;
use lt::Signer as _;
use std::{collections::BTreeMap, sync::Arc};
use zeroize::Zeroizing;

pub fn hl_network(network: &str) -> Result<hl::Network> {
    match network {
        "testnet" => Ok(hl::Network::Testnet),
        "mainnet" => Ok(hl::Network::Mainnet),
        _ => Err(errors::ValidationError::new_err(
            "network must be 'testnet' or 'mainnet'",
        )),
    }
}
pub fn lt_chain(network: &str, chain_id: Option<u32>) -> Result<u32> {
    Ok(chain_id.unwrap_or(match hl_network(network)? {
        hl::Network::Testnet => 300,
        hl::Network::Mainnet => 304,
    }))
}
pub fn address(value: &str) -> Result<hl::Address> {
    value.parse().map_err(errors::hl_native)
}
pub fn options(
    expired_at: Option<i64>,
    attributes: Option<BTreeMap<u8, i64>>,
) -> Result<lt::TxOptions> {
    let mut options = match expired_at {
        Some(v) => lt::TxOptions::new(v),
        None => lt::TxOptions::short_lived().map_err(errors::lt_native)?,
    };
    options.attributes =
        lt::TxAttributes::new(attributes.unwrap_or_default()).map_err(errors::lt_native)?;
    Ok(options)
}
pub fn transaction(kind: &str, request: &Input) -> Result<lt::Transaction> {
    let tx = match kind {
        "create" => lt::Transaction::CreateOrder(convert::parse(request)?),
        "cancel" => lt::Transaction::CancelOrder(convert::parse(request)?),
        _ => {
            return Err(errors::ValidationError::new_err(
                "transaction kind must be 'create' or 'cancel'",
            ));
        }
    };
    tx.validate().map_err(errors::lt_native)?;
    Ok(tx)
}

#[derive(Clone)]
pub struct HyperliquidSigningPayload {
    inner: hl::SigningPayload,
}
impl HyperliquidSigningPayload {
    pub(crate) fn recover_address(&self, signature: &Input) -> Result<String> {
        let signature: hl::Signature = convert::parse(signature)?;
        let payload = self.inner.clone();
        context::evaluate(move || signature.recover_address(&payload))
            .map(|a| a.to_string())
            .map_err(|_| errors::SigningError::new_err("invalid Hyperliquid signature"))
    }
    pub(crate) fn digest(&self) -> Vec<u8> {
        self.inner.digest().to_vec()
    }
    pub(crate) fn network(&self) -> &'static str {
        match self.inner.network() {
            hl::Network::Mainnet => "mainnet",
            hl::Network::Testnet => "testnet",
        }
    }
    pub(crate) fn scheme(&self) -> &'static str {
        match self.inner.scheme() {
            hl::signing::SigningScheme::L1Action => "l1_action",
            hl::signing::SigningScheme::UserSignedAction => "user_signed_action",
        }
    }
}
#[derive(Clone)]
pub struct LighterSigningPayload {
    inner: lt::SigningPayload,
}
impl LighterSigningPayload {
    pub(crate) fn verify(&self, signature: &str, public_key: &str) -> Result<bool> {
        let signature = hex::decode(signature.strip_prefix("0x").unwrap_or(signature))
            .ok()
            .and_then(|v| v.try_into().ok())
            .ok_or_else(|| errors::SigningError::new_err("signature must encode 80 bytes"))?;
        let public_key = hex::decode(public_key.strip_prefix("0x").unwrap_or(public_key))
            .ok()
            .and_then(|v| v.try_into().ok())
            .ok_or_else(|| errors::SigningError::new_err("public key must encode 40 bytes"))?;
        let payload = self.inner.clone();
        context::evaluate(move || {
            lt::Signature::from_bytes(signature)?
                .verify(&payload, lt::PublicKey::from_bytes(public_key)?)
        })
        .map_err(errors::lt_native)?;
        Ok(true)
    }
    pub(crate) fn digest(&self) -> Vec<u8> {
        self.inner.digest().to_vec()
    }
    pub(crate) fn chain_id(&self) -> u32 {
        self.inner.chain_id()
    }
    pub(crate) fn account_index(&self) -> i64 {
        self.inner.account_index()
    }
    pub(crate) fn api_key_index(&self) -> u8 {
        self.inner.api_key_index()
    }
    pub(crate) fn purpose(&self) -> Result<Output> {
        convert::output(match self.inner.purpose() {
            lt::signing::SigningPurpose::Transaction { tx_type, nonce } => {
                serde_json::json!({"type":"transaction","tx_type":tx_type,"nonce":nonce})
            }
            lt::signing::SigningPurpose::AuthToken { deadline } => {
                serde_json::json!({"type":"auth_token","deadline":deadline})
            }
        })
    }
}

enum HlKind {
    Local(hl::LocalSigner),
    Remote {
        address: hl::Address,
        callback: Callback,
    },
}
#[derive(Clone)]
pub struct HlSigner(Arc<HlKind>);
impl hl::Signer for HlSigner {
    fn address(&self) -> hl::Address {
        match &*self.0 {
            HlKind::Local(s) => s.address(),
            HlKind::Remote { address, .. } => *address,
        }
    }
    async fn sign(&self, payload: hl::SigningPayload) -> Result<hl::Signature, hl::SignerError> {
        match &*self.0 {
            HlKind::Local(s) => s.sign(payload).await,
            HlKind::Remote { callback: cb, .. } => {
                let object = context::evaluate(|| {
                    convert::output(serde_json::json!({"digest":payload.digest(),"network":match payload.network(){hl::Network::Mainnet=>"mainnet",hl::Network::Testnet=>"testnet"},"scheme":match payload.scheme(){hl::signing::SigningScheme::L1Action=>"l1_action",_=>"user_signed_action"}}))
                })
                .map_err(|_| hl::SignerError::Unavailable)?;
                let value = callback(cb, object)
                    .await
                    .map_err(|_| hl::SignerError::Unavailable)?;
                context::evaluate(|| convert::parse(&value))
                    .map_err(|_| hl::SignerError::InvalidSignature)
            }
        }
    }
}
enum LtKind {
    Local(lt::LocalSigner),
    Remote {
        key: lt::PublicKey,
        callback: Callback,
    },
}
#[derive(Clone)]
pub struct LtSigner(Arc<LtKind>);
impl lt::Signer for LtSigner {
    fn public_key(&self) -> lt::PublicKey {
        match &*self.0 {
            LtKind::Local(s) => s.public_key(),
            LtKind::Remote { key, .. } => *key,
        }
    }
    async fn sign(&self, payload: &lt::SigningPayload) -> Result<lt::Signature, lt::Error> {
        match &*self.0 {
            LtKind::Local(s) => s.sign(payload).await,
            LtKind::Remote { callback: cb, .. } => {
                let object = context::evaluate(|| {
                    convert::output(serde_json::json!({"digest":payload.digest().as_slice(),"chainId":payload.chain_id(),"accountIndex":payload.account_index(),"apiKeyIndex":payload.api_key_index(),"purpose":match payload.purpose(){
                        lt::signing::SigningPurpose::Transaction{tx_type,nonce}=>serde_json::json!({"type":"transaction","tx_type":tx_type,"nonce":nonce}),
                        lt::signing::SigningPurpose::AuthToken{deadline}=>serde_json::json!({"type":"auth_token","deadline":deadline})
                    }}))
                })
                .map_err(|_| lt::Error::Signing("callback unavailable"))?;
                let value = callback(cb, object)
                    .await
                    .map_err(|_| lt::Error::Signing("callback failed"))?;
                let bytes = context::evaluate(|| convert::parse::<String>(&value))
                    .ok()
                    .and_then(|v| hex::decode(v.strip_prefix("0x").unwrap_or(&v)).ok())
                    .and_then(|v| v.try_into().ok())
                    .ok_or(lt::Error::InvalidSignature)?;
                lt::Signature::from_bytes(bytes)
            }
        }
    }
}

#[derive(Clone)]
pub struct HyperliquidSigner {
    pub inner: HlSigner,
}
impl HyperliquidSigner {
    pub(crate) fn new(private_key: String) -> Result<Self> {
        let private_key = Zeroizing::new(private_key);
        let signer = context::evaluate(move || private_key.parse())
            .map_err(|_| errors::SigningError::new_err("invalid Hyperliquid private key"))?;
        Ok(Self {
            inner: HlSigner(Arc::new(HlKind::Local(signer))),
        })
    }
    pub(crate) fn remote(address: &str, callback: Callback) -> Result<Self> {
        Ok(Self {
            inner: HlSigner(Arc::new(HlKind::Remote {
                address: self::address(address)?,
                callback,
            })),
        })
    }
    pub(crate) fn address(&self) -> String {
        self.inner.address().to_string()
    }
    pub(crate) async fn sign(&self, payload: &HyperliquidSigningPayload) -> Result<Output> {
        let payload = payload.inner.clone();
        let signer = self.inner.clone();
        convert::future_output(async move {
            let signature = signer
                .sign(payload.clone())
                .await
                .map_err(|_| errors::SigningError::new_err("Hyperliquid signing failed"))?;
            if signature.recover_address(&payload).ok() != Some(signer.address()) {
                return Err(errors::SigningError::new_err("signature identity mismatch"));
            }
            convert::output(signature)
        })
        .await
    }
    pub(crate) async fn sign_action(
        &self,
        action: &Input,
        nonce: u64,
        network: &str,
        vault_address: Option<&str>,
        expires_after: Option<u64>,
    ) -> Result<Output> {
        let action: hl::Action = convert::parse(action)?;
        action.validate().map_err(errors::hl_native)?;
        let network = hl_network(network)?;
        let vault = vault_address.map(address).transpose()?;
        let mut auth = hl::HyperliquidAuth::direct(self.inner.clone());
        if let Some(vault) = vault {
            auth = auth.with_vault(vault);
        }
        convert::future_output(async move {
            convert::output(
                auth.prepare(
                    action,
                    network,
                    hl::ExecutionOptions {
                        nonce: Some(nonce),
                        expires_after,
                    },
                )
                .await
                .map_err(errors::hl_native)?,
            )
        })
        .await
    }
    pub(crate) async fn sign_user_action(
        &self,
        action: &Input,
        network: &str,
        signature_chain_id: u64,
    ) -> Result<Output> {
        let action: hl::signing::UserSignedAction = convert::parse(action)?;
        let network = hl_network(network)?;
        let signer = self.inner.clone();
        convert::future_output(async move {
            let payload = action
                .signing_payload(network, signature_chain_id)
                .map_err(errors::hl_native)?;
            let signature = signer
                .sign(payload.clone())
                .await
                .map_err(|_| errors::SigningError::new_err("Hyperliquid signing failed"))?;
            if signature.recover_address(&payload).ok() != Some(signer.address()) {
                return Err(errors::SigningError::new_err("signature identity mismatch"));
            }
            convert::output(signature)
        })
        .await
    }
    pub(crate) fn __repr__(&self) -> String {
        format!("HyperliquidSigner(address='{}')", self.address())
    }
}
#[derive(Clone)]
pub struct LighterSigner {
    pub inner: LtSigner,
}
impl LighterSigner {
    pub(crate) fn new(private_key: String) -> Result<Self> {
        let signer =
            context::evaluate(move || lt::LocalSigner::from_hex(Zeroizing::new(private_key)))
                .map_err(errors::lt_native)?;
        Ok(Self {
            inner: LtSigner(Arc::new(LtKind::Local(signer))),
        })
    }
    pub(crate) fn remote(public_key: &str, callback: Callback) -> Result<Self> {
        let key = hex::decode(public_key.strip_prefix("0x").unwrap_or(public_key))
            .ok()
            .and_then(|v| v.try_into().ok())
            .ok_or_else(|| errors::ValidationError::new_err("public key must encode 40 bytes"))?;
        Ok(Self {
            inner: LtSigner(Arc::new(LtKind::Remote {
                key: lt::PublicKey::from_bytes(key).map_err(errors::lt_native)?,
                callback,
            })),
        })
    }
    pub(crate) fn public_key(&self) -> String {
        hex::encode(self.inner.public_key().as_bytes())
    }
    pub(crate) async fn sign(&self, payload: &LighterSigningPayload) -> Result<Output> {
        let payload = payload.inner.clone();
        let signer = self.inner.clone();
        convert::future_output(async move {
            let sig = signer.sign(&payload).await.map_err(errors::lt_native)?;
            sig.verify(&payload, signer.public_key())
                .map_err(errors::lt_native)?;
            Ok(hex::encode(sig.as_bytes()))
        })
        .await
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn sign_transaction(
        &self,
        kind: &str,
        request: &Input,
        account_index: i64,
        api_key_index: u8,
        nonce: i64,
        network: &str,
        chain_id: Option<u32>,
        expired_at: Option<i64>,
        attributes: Option<BTreeMap<u8, i64>>,
    ) -> Result<Output> {
        let tx = transaction(kind, request)?;
        let chain = lt_chain(network, chain_id)?;
        let options = options(expired_at, attributes)?;
        let signer = self.inner.clone();
        convert::future_output(async move {
            let prepared = tx
                .prepare(chain, account_index, api_key_index, nonce, options)
                .map_err(errors::lt_native)?;
            let signed = prepared.sign(&signer).await.map_err(errors::lt_native)?;
            let info: serde_json::Value = serde_json::from_str(signed.tx_info()).map_err(|_| {
                errors::DecodeError::new_err("native transaction conversion failed")
            })?;
            convert::output(
                serde_json::json!({"tx_type":signed.tx_type(),"tx_info":info,"digest":hex::encode(signed.signing_payload().digest())}),
            )
        }).await
    }
    pub(crate) async fn auth_token(
        &self,
        account_index: i64,
        api_key_index: u8,
        deadline: i64,
        network: &str,
        chain_id: Option<u32>,
    ) -> Result<Output> {
        let auth = lt::LighterAuth::new(
            self.inner.clone(),
            lt_chain(network, chain_id)?,
            account_index,
            api_key_index,
        )
        .map_err(errors::lt_native)?;
        convert::future_output(async move {
            Ok(auth
                .token(deadline)
                .await
                .map_err(errors::lt_native)?
                .as_str()
                .to_owned())
        })
        .await
    }
    pub(crate) fn __repr__(&self) -> String {
        format!("LighterSigner(public_key='{}')", self.public_key())
    }
}

pub fn hyperliquid_action_hash(
    action: &Input,
    nonce: u64,
    vault_address: Option<&str>,
    expires_after: Option<u64>,
) -> Result<String> {
    let action: hl::Action = convert::parse(action)?;
    action.validate().map_err(errors::hl_native)?;
    let vault = vault_address.map(address).transpose()?;
    context::evaluate(move || hl::signing::action_hash(&action, vault, nonce, expires_after))
        .map(hex::encode)
        .map_err(errors::hl_native)
}
pub fn validate_hyperliquid_order(order: &Input, sz_decimals: u32, is_spot: bool) -> Result<()> {
    let order: hl::Order = convert::parse(order)?;
    context::evaluate(move || order.validate_precision(sz_decimals, is_spot))
        .map_err(errors::hl_native)
}
pub fn scale_lighter_decimal(value: &str, decimals: u8) -> Result<u64> {
    lt::scale_decimal(value, decimals).map_err(errors::lt_native)
}

pub fn hyperliquid_signing_payload(
    action: &Input,
    nonce: u64,
    network: &str,
    vault_address: Option<&str>,
    expires_after: Option<u64>,
) -> Result<HyperliquidSigningPayload> {
    let action: hl::Action = convert::parse(action)?;
    action.validate().map_err(errors::hl_native)?;
    let network = hl_network(network)?;
    let vault = vault_address.map(address).transpose()?;
    let inner = context::evaluate(move || {
        hl::signing::l1_signing_payload(&action, vault, nonce, expires_after, network)
    })
    .map_err(errors::hl_native)?;
    Ok(HyperliquidSigningPayload { inner })
}
pub fn hyperliquid_user_signing_payload(
    action: &Input,
    network: &str,
    signature_chain_id: u64,
) -> Result<HyperliquidSigningPayload> {
    let action: hl::signing::UserSignedAction = convert::parse(action)?;
    let network = hl_network(network)?;
    let inner = context::evaluate(move || action.signing_payload(network, signature_chain_id))
        .map_err(errors::hl_native)?;
    Ok(HyperliquidSigningPayload { inner })
}
pub fn lighter_auth_signing_payload(
    account_index: i64,
    api_key_index: u8,
    deadline: i64,
    network: &str,
    chain_id: Option<u32>,
) -> Result<LighterSigningPayload> {
    let chain = lt_chain(network, chain_id)?;
    let inner = context::evaluate(move || {
        lt::auth_signing_payload(chain, account_index, api_key_index, deadline)
    })
    .map_err(errors::lt_native)?;
    Ok(LighterSigningPayload { inner })
}
#[allow(clippy::too_many_arguments)]
pub fn lighter_transaction_signing_payload(
    kind: &str,
    request: &Input,
    account_index: i64,
    api_key_index: u8,
    nonce: i64,
    network: &str,
    chain_id: Option<u32>,
    expired_at: Option<i64>,
    attributes: Option<BTreeMap<u8, i64>>,
) -> Result<LighterSigningPayload> {
    let tx = transaction(kind, request)?;
    let chain = lt_chain(network, chain_id)?;
    let options = options(expired_at, attributes)?;
    let inner = context::evaluate(move || {
        tx.prepare(chain, account_index, api_key_index, nonce, options)
            .map(|p| p.signing_payload().clone())
    })
    .map_err(errors::lt_native)?;
    Ok(LighterSigningPayload { inner })
}

/// Strongly typed thread-safe callbacks; JavaScript runs only on its owning thread.
pub type Callback = std::sync::Arc<
    napi::threadsafe_function::ThreadsafeFunction<
        String,
        napi::bindgen_prelude::Promise<String>,
        String,
        napi::Status,
        false,
        true,
    >,
>;
pub(crate) async fn callback(callback: &Callback, payload: Output) -> Result<Input> {
    let promise = callback
        .call_async_catch(convert::encode(payload)?)
        .await
        .map_err(|_| errors::SigningError::new_err("signer callback unavailable"))?;
    let text = promise
        .await
        .map_err(|_| errors::SigningError::new_err("signer callback rejected"))?;
    convert::input(&text)
}
