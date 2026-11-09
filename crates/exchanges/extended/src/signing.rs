//! Extended Stark settlement hashes and separate EVM wallet signatures.
use crate::{Error, Extended};
use aux_rs_core::hex;
use k256::ecdsa::{RecoveryId, SigningKey, VerifyingKey};
use serde_json::{Value, json};
use sha3::{Digest, Keccak256};
use starknet_core::types::Felt;
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
    EvmTyped,
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
        if digest.to_bytes_be() != payload.digest {
            return Err(Error::Signing("noncanonical Stark digest"));
        }
        // Match Extended fast-stark-crypto RFC6979 with no entropy seed.
        // RFC6979 derives the ephemeral scalar; no caller or RNG supplies it.
        let k = starknet_crypto::rfc6979_generate_k(&digest, &key, None);
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
        if payload.scheme == SigningScheme::StarknetMessage {
            return Err(Error::Signing("EVM key cannot sign a Stark settlement"));
        }
        let digest = if matches!(
            payload.scheme,
            SigningScheme::EvmPersonal | SigningScheme::EvmTyped
        ) {
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
        if matches!(
            payload.scheme,
            SigningScheme::EvmPersonal | SigningScheme::EvmTyped
        ) {
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
        if Felt::from_bytes_be(&payload.digest).to_bytes_be() != payload.digest {
            return Err(Error::Signing("noncanonical Stark digest"));
        }
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
    if matches!(
        payload.scheme,
        SigningScheme::EvmPersonal | SigningScheme::EvmTyped
    ) {
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
    let digest = if matches!(
        payload.scheme,
        SigningScheme::EvmPersonal | SigningScheme::EvmTyped
    ) {
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
/// A validated, explicit signing domain. Arc production keeps SN_MAIN for compatibility.
pub fn settlement_domain(exchange: &Extended) -> Result<crate::settlement::StarknetDomain, Error> {
    short_string(&exchange.chain_id)?;
    Ok(crate::settlement::StarknetDomain {
        name: "Perpetuals".into(),
        version: "v0".into(),
        chain_id: exchange.chain_id.clone(),
        revision: 1,
    })
}
pub fn order_hash(
    exchange: &Extended,
    public_key: &str,
    order: &crate::settlement::Order,
) -> Result<String, Error> {
    use crate::settlement::OffChainMessage;
    Ok(format!(
        "{:#x}",
        order
            .message_hash(&settlement_domain(exchange)?, felt(public_key)?)
            .map_err(|_| Error::Signing("order hash failed"))?
    ))
}
pub fn limit_order_hash(
    exchange: &Extended,
    public_key: &str,
    order: &crate::settlement::LimitOrder,
) -> Result<String, Error> {
    use crate::settlement::OffChainMessage;
    Ok(format!(
        "{:#x}",
        order
            .message_hash(&settlement_domain(exchange)?, felt(public_key)?)
            .map_err(|_| Error::Signing("limit-order hash failed"))?
    ))
}
pub fn transfer_hash(
    exchange: &Extended,
    public_key: &str,
    transfer: &crate::settlement::TransferArgs,
) -> Result<String, Error> {
    use crate::settlement::OffChainMessage;
    Ok(format!(
        "{:#x}",
        transfer
            .message_hash(&settlement_domain(exchange)?, felt(public_key)?)
            .map_err(|_| Error::Signing("transfer hash failed"))?
    ))
}
pub fn withdrawal_hash(
    exchange: &Extended,
    public_key: &str,
    withdrawal: &crate::settlement::WithdrawalArgs,
) -> Result<String, Error> {
    use crate::settlement::OffChainMessage;
    Ok(format!(
        "{:#x}",
        withdrawal
            .message_hash(&settlement_domain(exchange)?, felt(public_key)?)
            .map_err(|_| Error::Signing("withdrawal hash failed"))?
    ))
}
pub fn stark_payload(hash: &str) -> Result<SigningPayload, Error> {
    Ok(SigningPayload {
        digest: felt(hash)?.to_bytes_be(),
        scheme: SigningScheme::StarknetMessage,
        typed_data: None,
        personal_message: None,
    })
}
pub async fn sign_hash<S: Signer>(signer: &S, hash: &str) -> Result<Value, Error> {
    let payload = stark_payload(hash)?;
    let sig = signer.sign(&payload).await?;
    verify_signature(signer, &payload, &sig)?;
    Ok(json!({"r":format!("{:#x}",felt(&sig[0])?),"s":format!("{:#x}",felt(&sig[1])?)}))
}
/// Exact conversion only. Callers choose native signed amounts and explicit rounding beforehand.
pub fn scale_decimal(value: &str, decimals: u32) -> Result<i64, Error> {
    if decimals > 18 {
        return Err(Error::InvalidRequest("unsupported asset precision"));
    }
    let (negative, value) = value
        .strip_prefix('-')
        .map_or((false, value), |v| (true, v));
    let mut parts = value.split('.');
    let whole = parts.next().unwrap_or("");
    let fraction = parts.next().unwrap_or("");
    if parts.next().is_some()
        || whole.is_empty()
        || !whole
            .bytes()
            .chain(fraction.bytes())
            .all(|c| c.is_ascii_digit())
        || fraction.len() > decimals as usize
    {
        return Err(Error::InvalidRequest(
            "invalid decimal or excessive precision",
        ));
    }
    let amount = format!(
        "{whole}{fraction}{}",
        "0".repeat(decimals as usize - fraction.len())
    )
    .parse::<i128>()
    .map_err(|_| Error::InvalidRequest("amount overflow"))?;
    i64::try_from(if negative { -amount } else { amount })
        .map_err(|_| Error::InvalidRequest("native amount exceeds i64"))
}
/// Wallet registration typed-data digest. Terms acceptance must be supplied explicitly.
pub fn onboarding_payload(
    exchange: &Extended,
    registration: &Value,
) -> Result<SigningPayload, Error> {
    let domain = if exchange.network == "testnet" {
        "starknet.sepolia.extended.exchange"
    } else if exchange.network == "mainnet" {
        "extended.exchange"
    } else {
        registration["signingDomain"]
            .as_str()
            .ok_or(Error::InvalidRequest(
                "custom onboarding signingDomain required",
            ))?
    };
    let text = |k: &str| {
        registration[k]
            .as_str()
            .ok_or(Error::InvalidRequest("missing registration field"))
    };
    let index = registration["accountIndex"]
        .as_i64()
        .filter(|v| *v >= 0 && *v <= 127)
        .ok_or(Error::InvalidRequest("invalid account index"))?;
    let accepted = registration["tosAccepted"]
        .as_bool()
        .ok_or(Error::InvalidRequest("explicit tosAccepted required"))?;
    let mut wallet = [0; 32];
    let mut addr = [0; 20];
    hex::decode_to_slice(
        text("wallet")?
            .strip_prefix("0x")
            .unwrap_or(text("wallet")?),
        &mut addr,
    )
    .map_err(|_| Error::InvalidRequest("invalid wallet"))?;
    wallet[12..].copy_from_slice(&addr);
    let mut d = keccak(b"EIP712Domain(string name)").to_vec();
    d.extend(keccak(domain));
    let domain_hash = keccak(d);
    let mut m=keccak(b"AccountRegistration(int8 accountIndex,address wallet,bool tosAccepted,string time,string action,string host)").to_vec();
    let mut word = [0; 32];
    word[31] = index as u8;
    m.extend(word);
    m.extend(wallet);
    word[31] = u8::from(accepted);
    m.extend(word);
    for key in ["time", "action", "host"] {
        m.extend(keccak(text(key)?))
    }
    let mut bytes = vec![0x19, 0x01];
    bytes.extend(domain_hash);
    bytes.extend(keccak(m));
    Ok(SigningPayload {
        digest: keccak(bytes),
        scheme: SigningScheme::EvmTyped,
        typed_data: Some(
            json!({"domain":{"name":domain},"primaryType":"AccountRegistration","types":{"EIP712Domain":[{"name":"name","type":"string"}],"AccountRegistration":[{"name":"accountIndex","type":"int8"},{"name":"wallet","type":"address"},{"name":"tosAccepted","type":"bool"},{"name":"time","type":"string"},{"name":"action","type":"string"},{"name":"host","type":"string"}]},"message":{"accountIndex":index,"wallet":registration["wallet"],"tosAccepted":accepted,"time":registration["time"],"action":registration["action"],"host":registration["host"]}}),
        ),
        personal_message: None,
    })
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OrderContext {
    pub synthetic_asset_id: String,
    pub collateral_asset_id: String,
    pub synthetic_resolution: u64,
    pub collateral_resolution: u64,
    pub collateral_position: u32,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub max_position_value: Option<String>,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub quantity_precision: Option<u32>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PreparedOrder {
    pub hash: String,
    pub body: Value,
}
#[derive(Clone)]
struct Decimal {
    coefficient: num_bigint::BigInt,
    scale: u32,
}
impl Decimal {
    fn parse(s: &str) -> Result<Self, Error> {
        if s.len() > 128 {
            return Err(Error::InvalidRequest("decimal too large"));
        }
        let mut pieces = s.split('.');
        let whole = pieces.next().unwrap_or("");
        let fraction = pieces.next().unwrap_or("");
        if whole.is_empty()
            || pieces.next().is_some()
            || !whole
                .bytes()
                .chain(fraction.bytes())
                .all(|c| c.is_ascii_digit())
            || fraction.len() > 38
        {
            return Err(Error::InvalidRequest("unsigned fixed decimal required"));
        }
        Ok(Self {
            coefficient: format!("{whole}{fraction}")
                .parse()
                .map_err(|_| Error::InvalidRequest("invalid decimal"))?,
            scale: fraction.len() as u32,
        })
    }
    fn mul(&self, other: &Self) -> Self {
        Self {
            coefficient: &self.coefficient * &other.coefficient,
            scale: self.scale + other.scale,
        }
    }
    fn add(&self, other: &Self) -> Self {
        let scale = self.scale.max(other.scale);
        Self {
            coefficient: &self.coefficient * num_bigint::BigInt::from(10u8).pow(scale - self.scale)
                + &other.coefficient * num_bigint::BigInt::from(10u8).pow(scale - other.scale),
            scale,
        }
    }
    fn native(&self, resolution: u64, up: bool, exact: bool) -> Result<u64, Error> {
        if resolution == 0 {
            return Err(Error::InvalidRequest("asset resolution must be positive"));
        }
        let n = &self.coefficient * resolution;
        let d = num_bigint::BigInt::from(10u8).pow(self.scale);
        let q = &n / &d;
        let rem = &n % &d;
        if exact && rem != num_bigint::BigInt::from(0) {
            return Err(Error::InvalidRequest("quantity exceeds native precision"));
        }
        let q = if up && rem != num_bigint::BigInt::from(0) {
            q + 1
        } else {
            q
        };
        q.to_string()
            .parse()
            .map_err(|_| Error::InvalidRequest("native amount overflow"))
    }
}
/// Builds the native settlement hash and preserves the supplied order envelope.
/// Derived quote and fee rounding follows the official SDK; quantity precision is exact.
pub fn prepare_order(
    exchange: &Extended,
    public_key: &str,
    mut body: Value,
    context: &OrderContext,
) -> Result<PreparedOrder, Error> {
    let text = |k: &str| {
        body[k]
            .as_str()
            .ok_or(Error::InvalidRequest("native string order field required"))
    };
    let qty = Decimal::parse(text("qty")?)?;
    let price = Decimal::parse(text("price")?)?;
    let fee = Decimal::parse(text("fee")?)?;
    let builder = Decimal::parse(body["builderFee"].as_str().unwrap_or("0"))?;
    let buy = match text("side")? {
        "BUY" => true,
        "SELL" => false,
        _ => return Err(Error::InvalidRequest("BUY or SELL side required")),
    };
    let base = qty.native(context.synthetic_resolution, buy, true)?;
    let quote = qty
        .mul(&price)
        .native(context.collateral_resolution, buy, false)?;
    let max_fee = qty.mul(&price).mul(&fee.add(&builder)).native(
        context.collateral_resolution,
        true,
        false,
    )?;
    if (base == 0 || quote == 0) && body["type"] != "TPSL" {
        return Err(Error::InvalidRequest(
            "positive quantity and price required",
        ));
    }
    let base = i64::try_from(base).map_err(|_| Error::InvalidRequest("base amount exceeds i64"))?;
    let quote =
        i64::try_from(quote).map_err(|_| Error::InvalidRequest("quote amount exceeds i64"))?;
    let expiry = body["expiryEpochMillis"]
        .as_u64()
        .ok_or(Error::InvalidRequest("expiryEpochMillis required"))?;
    let expiration = expiry
        .div_ceil(1000)
        .checked_add(14 * 86400)
        .ok_or(Error::InvalidRequest("expiration overflow"))?;
    let salt = felt(text("nonce")?)?;
    use crate::settlement::{AssetId, Order, PositionId, Timestamp};
    let order = Order {
        position_id: PositionId {
            value: context.collateral_position,
        },
        base_asset_id: AssetId {
            value: felt(&context.synthetic_asset_id)?,
        },
        base_amount: if buy { base } else { -base },
        quote_asset_id: AssetId {
            value: felt(&context.collateral_asset_id)?,
        },
        quote_amount: if buy { -quote } else { quote },
        fee_asset_id: AssetId {
            value: felt(&context.collateral_asset_id)?,
        },
        fee_amount: max_fee,
        expiration: Timestamp {
            seconds: expiration,
        },
        salt,
    };
    let hash = order_hash(exchange, public_key, &order)?;
    if body.get("id").is_none() {
        body["id"] = json!(felt(&hash)?.to_string())
    }
    body["debuggingAmounts"] = json!({"collateralAmount":order.quote_amount.to_string(),"syntheticAmount":order.base_amount.to_string(),"feeAmount":order.fee_amount.to_string()});
    body["settlement"] = json!({"starkKey":format!("{:#x}",felt(public_key)?),"collateralPosition":context.collateral_position.to_string()});
    Ok(PreparedOrder { hash, body })
}
pub async fn sign_order<S: Signer>(
    exchange: &Extended,
    signer: &S,
    body: Value,
    context: &OrderContext,
) -> Result<Value, Error> {
    let original = body.clone();
    let mut prepared = prepare_order(exchange, &signer.public_key(), body, context)?;
    if original["type"] == "TPSL" {
        prepared.body["settlement"] = Value::Null;
    } else {
        prepared.body["settlement"]["signature"] = sign_hash(signer, &prepared.hash).await?;
    }
    for key in ["takeProfit", "stopLoss"] {
        if original.get(key).is_none_or(Value::is_null) {
            continue;
        }
        let mut leg = original[key].clone();
        let mut order = original.clone();
        order["type"] = json!("LIMIT");
        order["side"] = if original["type"] == "TPSL" {
            original["side"].clone()
        } else if original["side"] == "BUY" {
            json!("SELL")
        } else {
            json!("BUY")
        };
        order["price"] = leg["price"].clone();
        order
            .as_object_mut()
            .ok_or(Error::InvalidRequest("order object required"))?
            .remove("id");
        if original["tpSlType"] == "POSITION" {
            let value = Decimal::parse(context.max_position_value.as_deref().ok_or(
                Error::InvalidRequest("position TPSL requires max_position_value"),
            )?)?;
            let precision = context.quantity_precision.ok_or(Error::InvalidRequest(
                "position TPSL requires quantity_precision",
            ))?;
            if precision > 18 {
                return Err(Error::InvalidRequest("unsupported TPSL quantity precision"));
            }
            let price = Decimal::parse(
                leg["price"]
                    .as_str()
                    .ok_or(Error::InvalidRequest("leg price required"))?,
            )?;
            if price.coefficient == num_bigint::BigInt::from(0) {
                return Err(Error::InvalidRequest("positive leg price required"));
            }
            let units = value.coefficient
                * 50u8
                * num_bigint::BigInt::from(10u8).pow(price.scale + precision)
                / (price.coefficient * num_bigint::BigInt::from(10u8).pow(value.scale));
            let mut digits = units.to_string();
            while digits.len() <= precision as usize {
                digits.insert(0, '0');
            }
            if precision > 0 {
                digits.insert(digits.len() - precision as usize, '.');
            }
            order["qty"] = json!(digits);
        } else if original["tpSlType"] != "ORDER" {
            return Err(Error::InvalidRequest(
                "explicit tpSlType required for conditional legs",
            ));
        }
        let leg_prepared = prepare_order(exchange, &signer.public_key(), order, context)?;
        leg["settlement"] = leg_prepared.body["settlement"].clone();
        leg["settlement"]["signature"] = sign_hash(signer, &leg_prepared.hash).await?;
        leg["debuggingAmounts"] = leg_prepared.body["debuggingAmounts"].clone();
        prepared.body[key] = leg;
    }
    Ok(prepared.body)
}
impl LocalStarkSigner {
    /// Derive the native Stark key from the exact EIP-712 AccountCreation signature.
    pub fn from_wallet_signature(signature: Zeroizing<String>) -> Result<Self, Error> {
        use num_bigint::BigUint;
        use sha2::{Digest, Sha256};
        let text = signature.strip_prefix("0x").unwrap_or(&signature);
        let mut bytes = Zeroizing::new([0; 65]);
        hex::decode_to_slice(text, &mut *bytes)
            .map_err(|_| Error::Signing("65-byte wallet signature required"))?;
        let seed = BigUint::from_bytes_be(&bytes[..32]);
        let two = BigUint::from(1u8) << 256usize;
        let order = BigUint::parse_bytes(
            b"3618502788666131213697322783095070105526743751716087489154079457884512865583",
            10,
        )
        .expect("constant curve order");
        let limit = &two - (&two % &order);
        for index in 0u64..1024 {
            let mut input = seed.to_bytes_be();
            let count = BigUint::from(index).to_bytes_be();
            input.extend(count);
            let digest = Sha256::digest(input);
            let key = BigUint::from_bytes_be(&digest);
            if key < limit {
                let key = key % &order;
                return Self::from_hex(Zeroizing::new(format!("0x{}", key.to_str_radix(16))));
            }
        }
        Err(Error::Signing("key grinding limit exhausted"))
    }
}
pub fn onboarding_l2_hash(wallet: &str, public_key: &str) -> Result<String, Error> {
    Ok(format!(
        "{:#x}",
        starknet_crypto::pedersen_hash(&felt(wallet)?, &felt(public_key)?)
    ))
}
pub fn api_request_payload(path: &str, timestamp: &str) -> Result<SigningPayload, Error> {
    if !path.starts_with('/') || path.contains(['\r', '\n']) || timestamp.contains(['\r', '\n']) {
        return Err(Error::InvalidRequest(
            "invalid signed API path or timestamp",
        ));
    }
    let message = format!("{path}@{timestamp}");
    Ok(SigningPayload {
        digest: personal_hash(message.as_bytes()),
        scheme: SigningScheme::EvmPersonal,
        typed_data: None,
        personal_message: Some(message),
    })
}
pub fn key_derivation_payload(
    exchange: &Extended,
    registration: &Value,
) -> Result<SigningPayload, Error> {
    let domain = if exchange.network == "testnet" {
        "starknet.sepolia.extended.exchange"
    } else if exchange.network == "mainnet" {
        "extended.exchange"
    } else {
        registration["signingDomain"]
            .as_str()
            .ok_or(Error::InvalidRequest("custom signingDomain required"))?
    };
    let index = registration["accountIndex"]
        .as_u64()
        .filter(|n| *n <= 127)
        .ok_or(Error::InvalidRequest("accountIndex must be 0..127"))?;
    let accepted = registration["tosAccepted"]
        .as_bool()
        .ok_or(Error::InvalidRequest("explicit tosAccepted required"))?;
    let wallet = registration["wallet"]
        .as_str()
        .ok_or(Error::InvalidRequest("wallet required"))?;
    let mut address = [0; 20];
    hex::decode_to_slice(wallet.strip_prefix("0x").unwrap_or(wallet), &mut address)
        .map_err(|_| Error::InvalidRequest("invalid EVM wallet"))?;
    let mut domain_data = keccak(b"EIP712Domain(string name)").to_vec();
    domain_data.extend(keccak(domain));
    let mut data =
        keccak(b"AccountCreation(int8 accountIndex,address wallet,bool tosAccepted)").to_vec();
    let mut word = [0; 32];
    word[31] = index as u8;
    data.extend(word);
    word = [0; 32];
    word[12..].copy_from_slice(&address);
    data.extend(word);
    word = [0; 32];
    word[31] = u8::from(accepted);
    data.extend(word);
    let mut digest = vec![0x19, 0x01];
    digest.extend(keccak(domain_data));
    digest.extend(keccak(data));
    Ok(SigningPayload {
        digest: keccak(digest),
        scheme: SigningScheme::EvmTyped,
        typed_data: Some(
            json!({"domain":{"name":domain},"primaryType":"AccountCreation","types":{"EIP712Domain":[{"name":"name","type":"string"}],"AccountCreation":[{"name":"accountIndex","type":"int8"},{"name":"wallet","type":"address"},{"name":"tosAccepted","type":"bool"}]},"message":{"accountIndex":index,"wallet":wallet,"tosAccepted":accepted}}),
        ),
        personal_message: None,
    })
}
