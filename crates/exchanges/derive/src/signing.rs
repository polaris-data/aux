//! EIP-191 login and EIP-712 actions; no exchange SDK is used at runtime.
use crate::Error;
use aux_rs_core::hex;
use k256::ecdsa::{RecoveryId, SigningKey, VerifyingKey};
use sha3::{Digest, Keccak256};
use std::{future::Future, sync::Arc};
use zeroize::Zeroizing;

pub fn keccak(bytes: impl AsRef<[u8]>) -> [u8; 32] {
    Keccak256::digest(bytes.as_ref()).into()
}
pub fn address(value: &str) -> Result<[u8; 20], Error> {
    let mut result = [0; 20];
    hex::decode_to_slice(value.strip_prefix("0x").unwrap_or(value), &mut result)
        .map_err(|_| Error::InvalidRequest("expected a 20-byte Ethereum address"))?;
    Ok(result)
}
pub fn personal_hash(message: &[u8]) -> [u8; 32] {
    let mut bytes = format!("\x19Ethereum Signed Message:\n{}", message.len()).into_bytes();
    bytes.extend_from_slice(message);
    keccak(bytes)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SigningScheme {
    Login,
    Action,
}
#[derive(Clone, Debug)]
pub struct SigningPayload {
    pub digest: [u8; 32],
    pub chain_id: u64,
    pub scheme: SigningScheme,
}
pub trait Signer: Send + Sync {
    fn address(&self) -> [u8; 20];
    fn sign(
        &self,
        payload: &SigningPayload,
    ) -> impl Future<Output = Result<[u8; 65], Error>> + Send;
}
#[derive(Clone)]
pub struct LocalSigner {
    key: Arc<Zeroizing<[u8; 32]>>,
    address: [u8; 20],
}
impl std::fmt::Debug for LocalSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalSigner").finish_non_exhaustive()
    }
}
impl LocalSigner {
    pub fn from_hex(value: Zeroizing<String>) -> Result<Self, Error> {
        let mut bytes = Zeroizing::new([0; 32]);
        hex::decode_to_slice(value.strip_prefix("0x").unwrap_or(&value), &mut *bytes)
            .map_err(|_| Error::Signing("invalid signing key"))?;
        let key =
            SigningKey::from_slice(&*bytes).map_err(|_| Error::Signing("invalid signing key"))?;
        let public = key.verifying_key().to_encoded_point(false);
        let hash = keccak(&public.as_bytes()[1..]);
        let mut address = [0; 20];
        address.copy_from_slice(&hash[12..]);
        Ok(Self {
            key: Arc::new(bytes),
            address,
        })
    }
}
impl Signer for LocalSigner {
    fn address(&self) -> [u8; 20] {
        self.address
    }
    async fn sign(&self, payload: &SigningPayload) -> Result<[u8; 65], Error> {
        let key = SigningKey::from_slice(&**self.key)
            .map_err(|_| Error::Signing("invalid signing key"))?;
        let (signature, id) = key
            .sign_prehash_recoverable(&payload.digest)
            .map_err(|_| Error::Signing("ECDSA signing failed"))?;
        let mut result = [0; 65];
        result[..64].copy_from_slice(&signature.to_bytes());
        result[64] = id.to_byte() + 27;
        Ok(result)
    }
}
pub fn verify_signature(
    signer: [u8; 20],
    digest: &[u8; 32],
    signature: &[u8; 65],
) -> Result<(), Error> {
    let sig = k256::ecdsa::Signature::from_slice(&signature[..64])
        .map_err(|_| Error::Signing("invalid signature"))?;
    if sig.normalize_s().is_some() {
        return Err(Error::Signing("noncanonical ECDSA signature"));
    }
    let v = signature[64]
        .checked_sub(27)
        .ok_or(Error::Signing("signature recovery id must be 27 or 28"))?;
    let id = RecoveryId::from_byte(v).ok_or(Error::Signing("invalid recovery id"))?;
    let key = VerifyingKey::recover_from_prehash(digest, &sig, id)
        .map_err(|_| Error::Signing("invalid signature"))?;
    if keccak(&key.to_encoded_point(false).as_bytes()[1..])[12..] != signer {
        return Err(Error::Signing("signature does not match signer"));
    }
    Ok(())
}
/// Encode an unsigned decimal integer without a float or a bounded machine integer.
pub fn uint_word(value: &str) -> Result<[u8; 32], Error> {
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err(Error::InvalidRequest("expected unsigned decimal integer"));
    }
    let mut out = [0u8; 32];
    for digit in value.bytes() {
        let mut carry = u16::from(digit - b'0');
        for byte in out.iter_mut().rev() {
            let v = u16::from(*byte) * 10 + carry;
            *byte = v as u8;
            carry = v >> 8;
        }
        if carry != 0 {
            return Err(Error::InvalidRequest("integer exceeds uint256"));
        }
    }
    Ok(out)
}
pub fn decimal_word(value: &str, decimals: usize, signed: bool) -> Result<[u8; 32], Error> {
    let negative = value.starts_with('-');
    let raw = if negative { &value[1..] } else { value };
    if negative && !signed {
        return Err(Error::InvalidRequest("negative unsigned amount"));
    }
    let (whole, fraction) = raw.split_once('.').unwrap_or((raw, ""));
    if whole.is_empty()
        || !whole.bytes().all(|b| b.is_ascii_digit())
        || !fraction.bytes().all(|b| b.is_ascii_digit())
        || fraction.len() > decimals
    {
        return Err(Error::InvalidRequest("invalid decimal precision"));
    }
    let mut digits = String::from(whole);
    digits.push_str(fraction);
    digits.extend(std::iter::repeat_n('0', decimals - fraction.len()));
    let mut word = uint_word(&digits)?;
    if signed && word[0] & 0x80 != 0 {
        return Err(Error::InvalidRequest("amount exceeds int256"));
    }
    if negative {
        for b in &mut word {
            *b = !*b;
        }
        for b in word.iter_mut().rev() {
            let (v, carry) = b.overflowing_add(1);
            *b = v;
            if !carry {
                break;
            }
        }
    }
    Ok(word)
}
/// ABI values for every module, including dynamic arrays and tuples.
#[derive(Clone, Debug)]
pub enum AbiValue {
    Word([u8; 32]),
    Bytes(Vec<u8>),
    Array(Vec<AbiValue>),
    Tuple(Vec<AbiValue>),
}
impl AbiValue {
    pub fn uint(value: impl ToString) -> Result<Self, Error> {
        Ok(Self::Word(uint_word(&value.to_string())?))
    }
    pub fn address(value: &str) -> Result<Self, Error> {
        let mut word = [0; 32];
        word[12..].copy_from_slice(&address(value)?);
        Ok(Self::Word(word))
    }
    pub fn decimal(value: &str, signed: bool) -> Result<Self, Error> {
        Ok(Self::Word(decimal_word(value, 18, signed)?))
    }
    fn dynamic(&self) -> bool {
        match self {
            Self::Word(_) => false,
            Self::Tuple(xs) => xs.iter().any(Self::dynamic),
            _ => true,
        }
    }
    fn static_size(&self) -> usize {
        match self {
            Self::Tuple(xs) if !self.dynamic() => xs.iter().map(Self::static_size).sum(),
            _ => 32,
        }
    }
    fn encode(&self) -> Result<Vec<u8>, Error> {
        match self {
            Self::Word(w) => Ok(w.to_vec()),
            Self::Tuple(xs) => abi_encode(xs),
            Self::Array(xs) => {
                let mut out = uint_word(&xs.len().to_string())?.to_vec();
                out.extend(abi_encode(xs)?);
                Ok(out)
            }
            Self::Bytes(bytes) => {
                let mut out = uint_word(&bytes.len().to_string())?.to_vec();
                out.extend(bytes);
                out.resize(32 + bytes.len().div_ceil(32) * 32, 0);
                Ok(out)
            }
        }
    }
}
pub fn abi_encode(values: &[AbiValue]) -> Result<Vec<u8>, Error> {
    let head_len: usize = values.iter().map(AbiValue::static_size).sum();
    let mut head = Vec::new();
    let mut tail = Vec::new();
    for value in values {
        if value.dynamic() {
            head.extend(uint_word(&(head_len + tail.len()).to_string())?);
            tail.extend(value.encode()?);
        } else {
            head.extend(value.encode()?);
        }
    }
    head.extend(tail);
    Ok(head)
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActionPayload {
    pub subaccount_id: String,
    pub nonce: String,
    pub module: String,
    pub encoded_data: Vec<u8>,
    pub expiry: u64,
    pub owner: String,
    pub signer: String,
}
impl ActionPayload {
    pub fn digest(&self, chain_id: u64, matching: &str) -> Result<[u8; 32], Error> {
        let domain=keccak(abi_encode(&[
            AbiValue::Word(keccak(b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)")),
            AbiValue::Word(keccak(b"Matching")),AbiValue::Word(keccak(b"1.0")),AbiValue::uint(chain_id)?,AbiValue::address(matching)?,
        ])?);
        let action=keccak(abi_encode(&[
            AbiValue::Word(keccak(b"Action(uint256 subaccountId,uint256 nonce,address module,bytes data,uint256 expiry,address owner,address signer)")),
            AbiValue::uint(&self.subaccount_id)?,AbiValue::uint(&self.nonce)?,AbiValue::address(&self.module)?,AbiValue::Word(keccak(&self.encoded_data)),AbiValue::uint(self.expiry)?,AbiValue::address(&self.owner)?,AbiValue::address(&self.signer)?,
        ])?);
        let mut bytes = vec![0x19, 0x01];
        bytes.extend(domain);
        bytes.extend(action);
        Ok(keccak(bytes))
    }
}
