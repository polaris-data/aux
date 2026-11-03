//! Native Starknet SNIP-12 (both revisions) and EVM account signatures.
use crate::{Error, Paradex};
use aux_rs_core::hex;
use k256::ecdsa::{RecoveryId, SigningKey, VerifyingKey};
use serde_json::{Value, json};
use sha3::{Digest, Keccak256};
use starknet_core::types::{Felt, TypedData};
use std::{future::Future, sync::Arc};
use zeroize::Zeroizing;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignerKind {
    Starknet,
    Evm,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SigningScheme {
    StarknetMessage,
    EvmPersonal,
}
#[derive(Clone, Debug)]
pub struct SigningPayload {
    pub digest: [u8; 32],
    pub scheme: SigningScheme,
    pub typed_data: Option<Value>,
    pub personal_message: Option<String>,
}
pub trait Signer: Send + Sync {
    fn kind(&self) -> SignerKind;
    fn public_key(&self) -> String;
    fn sign(
        &self,
        payload: &SigningPayload,
    ) -> impl Future<Output = Result<Vec<String>, Error>> + Send;
}
pub fn felt(value: &str) -> Result<Felt, Error> {
    if value.starts_with("0x") {
        Felt::from_hex(value)
    } else {
        Felt::from_dec_str(value)
    }
    .map_err(|_| Error::InvalidRequest("invalid Starknet field element"))
}
pub fn short_string(value: &str) -> Result<Felt, Error> {
    starknet_core::utils::cairo_short_string_to_felt(value)
        .map_err(|_| Error::InvalidRequest("invalid Cairo short string"))
}
fn keccak(bytes: impl AsRef<[u8]>) -> [u8; 32] {
    Keccak256::digest(bytes.as_ref()).into()
}
pub fn personal_hash(message: &[u8]) -> [u8; 32] {
    let mut b = format!("\x19Ethereum Signed Message:\n{}", message.len()).into_bytes();
    b.extend(message);
    keccak(b)
}
#[derive(Clone)]
pub struct LocalStarkSigner {
    key: Arc<Zeroizing<[u8; 32]>>,
    public: Felt,
}
impl std::fmt::Debug for LocalStarkSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalStarkSigner").finish_non_exhaustive()
    }
}
impl LocalStarkSigner {
    pub fn from_hex(value: Zeroizing<String>) -> Result<Self, Error> {
        let key = felt(&value)?;
        if key == Felt::ZERO
            || key
                >= Felt::from_raw([
                    369010039416812937,
                    9,
                    1143265896874747514,
                    8939893405601011193,
                ])
        {
            return Err(Error::Signing("invalid Stark private key"));
        }
        let public = starknet_crypto::get_public_key(&key);
        Ok(Self {
            key: Arc::new(Zeroizing::new(key.to_bytes_be())),
            public,
        })
    }
}
impl Signer for LocalStarkSigner {
    fn kind(&self) -> SignerKind {
        SignerKind::Starknet
    }
    fn public_key(&self) -> String {
        format!("{:#x}", self.public)
    }
    async fn sign(&self, payload: &SigningPayload) -> Result<Vec<String>, Error> {
        if payload.scheme != SigningScheme::StarknetMessage {
            return Err(Error::Signing("Stark key cannot sign an EVM message"));
        }
        let key = Felt::from_bytes_be(&self.key);
        let digest = Felt::from_bytes_be(&payload.digest);
        // Match paradex-py's pinned starknet-crypto-py reference entropy seed.
        // RFC6979 derives the ephemeral scalar; no caller or RNG supplies it.
        let k = starknet_crypto::rfc6979_generate_k(&digest, &key, Some(&Felt::from(32u8)));
        let sig = starknet_crypto::sign(&key, &digest, &k)
            .map_err(|_| Error::Signing("Stark ECDSA failed"))?;
        Ok(vec![sig.r.to_string(), sig.s.to_string()])
    }
}
#[derive(Clone)]
pub struct LocalEvmSigner {
    key: Arc<Zeroizing<[u8; 32]>>,
    address: [u8; 20],
}
impl std::fmt::Debug for LocalEvmSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalEvmSigner").finish_non_exhaustive()
    }
}
impl LocalEvmSigner {
    pub fn from_hex(value: Zeroizing<String>) -> Result<Self, Error> {
        let mut bytes = Zeroizing::new([0; 32]);
        hex::decode_to_slice(value.strip_prefix("0x").unwrap_or(&value), &mut *bytes)
            .map_err(|_| Error::Signing("invalid EVM signing key"))?;
        let key = SigningKey::from_slice(&*bytes)
            .map_err(|_| Error::Signing("invalid EVM signing key"))?;
        let hash = keccak(&key.verifying_key().to_encoded_point(false).as_bytes()[1..]);
        let mut address = [0; 20];
        address.copy_from_slice(&hash[12..]);
        Ok(Self {
            key: Arc::new(bytes),
            address,
        })
    }
    pub fn uncompressed_public_key(&self) -> Result<String, Error> {
        let key = SigningKey::from_slice(&**self.key)
            .map_err(|_| Error::Signing("invalid EVM signing key"))?;
        Ok(format!(
            "0x{}",
            hex::encode(key.verifying_key().to_encoded_point(false).as_bytes())
        ))
    }
}
impl Signer for LocalEvmSigner {
    fn kind(&self) -> SignerKind {
        SignerKind::Evm
    }
    fn public_key(&self) -> String {
        format!("0x{}", hex::encode(self.address))
    }
    async fn sign(&self, payload: &SigningPayload) -> Result<Vec<String>, Error> {
        let digest = if payload.scheme == SigningScheme::EvmPersonal {
            payload.digest
        } else {
            personal_hash(&payload.digest)
        };
        let key = SigningKey::from_slice(&**self.key)
            .map_err(|_| Error::Signing("invalid EVM signing key"))?;
        let (sig, v) = key
            .sign_prehash_recoverable(&digest)
            .map_err(|_| Error::Signing("EVM signing failed"))?;
        let raw = sig.to_bytes();
        if payload.scheme == SigningScheme::EvmPersonal {
            let mut out = raw.to_vec();
            out.push(v.to_byte() + 27);
            return Ok(vec![format!("0x{}", hex::encode(out))]);
        }
        Ok(vec![
            "1".into(),
            "3".into(),
            felt(&self.public_key())?.to_string(),
            u128::from_be_bytes(
                raw[16..32]
                    .try_into()
                    .map_err(|_| Error::Signing("invalid signature"))?,
            )
            .to_string(),
            u128::from_be_bytes(
                raw[..16]
                    .try_into()
                    .map_err(|_| Error::Signing("invalid signature"))?,
            )
            .to_string(),
            u128::from_be_bytes(
                raw[48..64]
                    .try_into()
                    .map_err(|_| Error::Signing("invalid signature"))?,
            )
            .to_string(),
            u128::from_be_bytes(
                raw[32..48]
                    .try_into()
                    .map_err(|_| Error::Signing("invalid signature"))?,
            )
            .to_string(),
            v.to_byte().to_string(),
        ])
    }
}
#[derive(Clone, Copy, Debug)]
pub struct NoSigner;
impl Signer for NoSigner {
    fn kind(&self) -> SignerKind {
        SignerKind::Starknet
    }
    fn public_key(&self) -> String {
        String::new()
    }
    async fn sign(&self, _: &SigningPayload) -> Result<Vec<String>, Error> {
        Err(Error::Signing("no signer attached"))
    }
}
pub fn verify_signature<S: Signer>(
    signer: &S,
    payload: &SigningPayload,
    signature: &[String],
) -> Result<(), Error> {
    if signer.kind() == SignerKind::Starknet {
        if payload.scheme != SigningScheme::StarknetMessage || signature.len() != 2 {
            return Err(Error::Signing("invalid Stark signature shape"));
        }
        let valid = starknet_crypto::verify(
            &felt(&signer.public_key())?,
            &Felt::from_bytes_be(&payload.digest),
            &felt(&signature[0])?,
            &felt(&signature[1])?,
        )
        .map_err(|_| Error::Signing("invalid Stark signature"))?;
        if !valid {
            return Err(Error::Signing("signature does not match signer"));
        }
        return Ok(());
    }
    let mut bytes = [0u8; 65];
    if payload.scheme == SigningScheme::EvmPersonal {
        if signature.len() != 1 {
            return Err(Error::Signing("invalid EVM signature shape"));
        }
        hex::decode_to_slice(
            signature[0].strip_prefix("0x").unwrap_or(&signature[0]),
            &mut bytes,
        )
        .map_err(|_| Error::Signing("invalid EVM signature"))?;
        bytes[64] = bytes[64]
            .checked_sub(27)
            .ok_or(Error::Signing("invalid recovery id"))?;
    } else {
        if signature.len() != 8
            || signature[0] != "1"
            || signature[1] != "3"
            || felt(&signature[2])? != felt(&signer.public_key())?
        {
            return Err(Error::Signing("invalid Argent EVM signature"));
        }
        for (index, start) in [(3, 16), (4, 0), (5, 48), (6, 32)] {
            let n = signature[index]
                .parse::<u128>()
                .map_err(|_| Error::Signing("invalid EVM signature limb"))?;
            bytes[start..start + 16].copy_from_slice(&n.to_be_bytes());
        }
        bytes[64] = signature[7]
            .parse()
            .map_err(|_| Error::Signing("invalid recovery id"))?;
    }
    let sig = k256::ecdsa::Signature::from_slice(&bytes[..64])
        .map_err(|_| Error::Signing("invalid EVM signature"))?;
    if sig.normalize_s().is_some() {
        return Err(Error::Signing("noncanonical EVM signature"));
    }
    let digest = if payload.scheme == SigningScheme::EvmPersonal {
        payload.digest
    } else {
        personal_hash(&payload.digest)
    };
    let id = RecoveryId::from_byte(bytes[64]).ok_or(Error::Signing("invalid recovery id"))?;
    let key = VerifyingKey::recover_from_prehash(&digest, &sig, id)
        .map_err(|_| Error::Signing("invalid EVM signature"))?;
    if felt(&format!(
        "0x{}",
        hex::encode(&keccak(&key.to_encoded_point(false).as_bytes()[1..])[12..])
    ))? != felt(&signer.public_key())?
    {
        return Err(Error::Signing("signature does not match signer"));
    }
    Ok(())
}
pub fn typed_payload(account: &str, data: Value) -> Result<SigningPayload, Error> {
    let revision = data["domain"].get("revision");
    let hash = if revision.is_none_or(|v| v.as_str() == Some("0") || v.as_u64() == Some(0)) {
        crate::legacy::message_hash(account, &data)?
    } else {
        let typed: TypedData = serde_json::from_value(data.clone())
            .map_err(|_| Error::Signing("invalid SNIP-12 typed data"))?;
        crate::revision_one::message_hash(&typed, felt(account)?)?
    };
    Ok(SigningPayload {
        digest: hash.to_bytes_be(),
        scheme: SigningScheme::StarknetMessage,
        typed_data: Some(data),
        personal_message: None,
    })
}

pub fn legacy_typed_data(
    e: &Paradex,
    primary: &str,
    fields: Value,
    message: Value,
) -> Result<Value, Error> {
    let mut types = json!({"StarkNetDomain":[{"name":"name","type":"felt"},{"name":"chainId","type":"felt"},{"name":"version","type":"felt"}]});
    types[primary] = fields;
    Ok(
        json!({"types":types,"primaryType":primary,"domain":{"name":"Paradex","chainId":format!("{:#x}",short_string(&e.chain_id)?),"version":"1"},"message":message}),
    )
}
pub fn auth_typed_data(e: &Paradex, timestamp: u64, expiration: u64) -> Result<Value, Error> {
    legacy_typed_data(
        e,
        "Request",
        json!([{"name":"method","type":"felt"},{"name":"path","type":"felt"},{"name":"body","type":"felt"},{"name":"timestamp","type":"felt"},{"name":"expiration","type":"felt"}]),
        json!({"method":"POST","path":"/v1/auth","body":"","timestamp":timestamp,"expiration":expiration}),
    )
}
pub fn onboarding_typed_data(e: &Paradex) -> Result<Value, Error> {
    legacy_typed_data(
        e,
        "Constant",
        json!([{"name":"action","type":"felt"}]),
        json!({"action":"Onboarding"}),
    )
}
/// Scale exact non-negative decimals to Paraclear's eight places. Never truncate.
pub fn chain_decimal(value: &str) -> Result<String, Error> {
    let (whole, fraction) = value.split_once('.').unwrap_or((value, ""));
    if whole.is_empty()
        || !whole.bytes().all(|b| b.is_ascii_digit())
        || !fraction.bytes().all(|b| b.is_ascii_digit())
        || fraction.len() > 8
    {
        return Err(Error::InvalidRequest(
            "Paraclear amount must be non-negative with at most 8 decimal places",
        ));
    }
    let mut s = format!("{whole}{fraction}{}", "0".repeat(8 - fraction.len()));
    s = s.trim_start_matches('0').to_owned();
    if s.is_empty() {
        s = "0".into();
    }
    felt(&s)?;
    Ok(s)
}
pub fn order_typed_data(
    e: &Paradex,
    order: &Value,
    modify_id: Option<&str>,
) -> Result<Value, Error> {
    let text = |k: &str| {
        order[k]
            .as_str()
            .ok_or(Error::InvalidRequest("missing order signing field"))
    };
    let kind = text("type")?;
    let side = match text("side")? {
        "BUY" => "1",
        "SELL" => "2",
        _ => return Err(Error::InvalidRequest("invalid order side")),
    };
    let price = if kind == "MARKET" {
        "0".into()
    } else {
        chain_decimal(text("price")?)?
    };
    let mut fields = json!([{"name":"timestamp","type":"felt"},{"name":"market","type":"felt"},{"name":"side","type":"felt"},{"name":"orderType","type":"felt"},{"name":"size","type":"felt"},{"name":"price","type":"felt"}]);
    let mut message = json!({"timestamp":order["signature_timestamp"].to_string(),"market":text("market")?,"side":side,"orderType":kind,"size":chain_decimal(text("size")?)?,"price":price});
    if let Some(id) = modify_id {
        fields
            .as_array_mut()
            .ok_or(Error::InvalidRequest("invalid signing fields"))?
            .push(json!({"name":"id","type":"felt"}));
        message["id"] = json!(id);
    }
    legacy_typed_data(
        e,
        if modify_id.is_some() {
            "ModifyOrder"
        } else {
            "Order"
        },
        fields,
        message,
    )
}
/// Native revision-1 block signing, with canonical market ordering and merkle leaves.
pub fn block_typed_data(
    e: &Paradex,
    nonce: &str,
    expiration: u64,
    trades: &Value,
    parent: Option<&str>,
) -> Result<Value, Error> {
    let object = trades
        .as_object()
        .ok_or(Error::InvalidRequest("expected market-keyed block trades"))?;
    let mut markets: Vec<_> = object.iter().collect();
    markets.sort_by_key(|(k, _)| *k);
    let zero_order = || json!({"account":"0","side":"0","type":"0","size":"0","price":"0"});
    let order = |o: &Value| -> Result<Value, Error> {
        if o.is_null() {
            return Ok(zero_order());
        }
        Ok(
            json!({"account":o["account"].as_str().filter(|s|!s.is_empty()).unwrap_or("0"),"side":match o["side"].as_str(){Some("BUY")=>"1",Some("SELL")=>"2",None|Some("")=>"0",_=>return Err(Error::InvalidRequest("invalid block side"))},"type":o["type"].as_str().filter(|s|!s.is_empty()).unwrap_or("0"),"size":chain_decimal(o["size"].as_str().unwrap_or("0"))?,"price":chain_decimal(o["price"].as_str().unwrap_or("0"))?}),
        )
    };
    let mut leaves = Vec::new();
    for (market, t) in markets {
        let mut leaf = json!({"market":market,"maker_order":order(if parent.is_some(){&t["offerer_order"]}else{&t["maker_order"]})?,"taker_order":if parent.is_some(){zero_order()}else{order(&t["taker_order"])?}});
        for key in [
            "price",
            "size",
            "min_size",
            "max_size",
            "min_price",
            "max_price",
            "oracle_tolerance",
        ] {
            let value = t.get(key).or_else(|| t["trade_constraints"].get(key));
            leaf[key] = json!(chain_decimal(value.and_then(Value::as_str).unwrap_or("0"))?);
        }
        leaves.push(leaf);
    }
    let primary = if parent.is_some() {
        "BlockTradeOffer"
    } else {
        "BlockTrade"
    };
    let mut primary_fields = json!([{"name":"version","type":"shortstring"},{"name":"nonce","type":"felt"},{"name":"expiration","type":"felt"}]);
    if parent.is_some() {
        primary_fields
            .as_array_mut()
            .ok_or(Error::InvalidRequest("invalid signing fields"))?
            .push(json!({"name":"block_trade_id","type":"felt"}));
    }
    primary_fields
        .as_array_mut()
        .ok_or(Error::InvalidRequest("invalid signing fields"))?
        .push(json!({"name":"trades","type":"merkletree","contains":"Trade"}));
    let mut types = json!({"StarknetDomain":[{"name":"name","type":"shortstring"},{"name":"version","type":"shortstring"},{"name":"chainId","type":"shortstring"},{"name":"revision","type":"shortstring"}],"Trade":[{"name":"market","type":"shortstring"},{"name":"price","type":"felt"},{"name":"size","type":"felt"},{"name":"maker_order","type":"BlockTradeOrder"},{"name":"taker_order","type":"BlockTradeOrder"},{"name":"min_size","type":"felt"},{"name":"max_size","type":"felt"},{"name":"min_price","type":"felt"},{"name":"max_price","type":"felt"},{"name":"oracle_tolerance","type":"felt"}],"BlockTradeOrder":[{"name":"account","type":"felt"},{"name":"side","type":"shortstring"},{"name":"type","type":"shortstring"},{"name":"size","type":"felt"},{"name":"price","type":"felt"}]});
    types[primary] = primary_fields;
    let mut message =
        json!({"version":"2","nonce":nonce,"expiration":expiration.to_string(),"trades":leaves});
    if let Some(id) = parent {
        message["block_trade_id"] = json!(id);
    }
    Ok(
        json!({"types":types,"primaryType":primary,"domain":{"name":"Paradex","chainId":format!("{:#x}",short_string(&e.chain_id)?),"version":"1","revision":"1"},"message":message}),
    )
}
