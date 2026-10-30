//! Native signing checked against the pinned official Python reference.
//! EIP-712 encoding is protocol logic; K256 and SHA3 supply cryptography.

use crate::{Address, Error, Network};
use aux_rs_core::hex;
use k256::ecdsa::{RecoveryId, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::{fmt, future::Future, str::FromStr};
use zeroize::Zeroizing;

pub const REFERENCE_REVISION: &str = "2fdb18f9517675ea03695a0962bd19eece9c83f0";
pub const USER_SIGNATURE_CHAIN_ID: u64 = 421_614;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignerError {
    InvalidKey,
    Unavailable,
    InvalidSignature,
    AddressMismatch,
}
impl fmt::Display for SignerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidKey => "invalid Hyperliquid signing key",
            Self::Unavailable => "Hyperliquid signer unavailable",
            Self::InvalidSignature => "invalid Hyperliquid signature",
            Self::AddressMismatch => "signature does not match the configured signer",
        })
    }
}
impl std::error::Error for SignerError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SigningScheme {
    L1Action,
    UserSignedAction,
}

/// Immutable native context passed to local or remote signers.
#[derive(Clone, Debug)]
pub struct SigningPayload {
    digest: [u8; 32],
    network: Network,
    scheme: SigningScheme,
}
impl SigningPayload {
    pub fn digest(&self) -> &[u8; 32] {
        &self.digest
    }
    pub fn network(&self) -> Network {
        self.network
    }
    pub fn scheme(&self) -> SigningScheme {
        self.scheme
    }
}

pub trait Signer: Send + Sync {
    fn address(&self) -> Address;
    fn sign(
        &self,
        payload: SigningPayload,
    ) -> impl Future<Output = Result<Signature, SignerError>> + Send;
}

/// K256 zeroizes its key storage on drop. Debug never formats private keys.
pub struct LocalSigner {
    key: SigningKey,
    address: Address,
}
impl fmt::Debug for LocalSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocalSigner")
            .field("address", &self.address)
            .finish_non_exhaustive()
    }
}
impl FromStr for LocalSigner {
    type Err = SignerError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value
            .strip_prefix("0x")
            .or_else(|| value.strip_prefix("0X"))
            .unwrap_or(value);
        let mut bytes = Zeroizing::new([0u8; 32]);
        hex::decode_to_slice(value, bytes.as_mut()).map_err(|_| SignerError::InvalidKey)?;
        let key = SigningKey::from_slice(bytes.as_ref()).map_err(|_| SignerError::InvalidKey)?;
        let address = address_of(key.verifying_key());
        Ok(Self { key, address })
    }
}
impl Signer for LocalSigner {
    fn address(&self) -> Address {
        self.address
    }
    async fn sign(&self, payload: SigningPayload) -> Result<Signature, SignerError> {
        let (signature, recovery) = self
            .key
            .sign_prehash_recoverable(payload.digest())
            .map_err(|_| SignerError::Unavailable)?;
        if recovery.to_byte() > 1 {
            return Err(SignerError::InvalidSignature);
        }
        let bytes = signature.to_bytes();
        Ok(Signature {
            r: scalar_hex(&bytes[..32]),
            s: scalar_hex(&bytes[32..]),
            v: recovery.to_byte() + 27,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    pub r: String,
    pub s: String,
    pub v: u8,
}
impl Signature {
    pub fn recover_address(&self, payload: &SigningPayload) -> Result<Address, SignerError> {
        let mut bytes = [0u8; 64];
        decode_scalar(&self.r, &mut bytes[..32])?;
        decode_scalar(&self.s, &mut bytes[32..])?;
        let signature = k256::ecdsa::Signature::from_slice(&bytes)
            .map_err(|_| SignerError::InvalidSignature)?;
        if signature.normalize_s().is_some() {
            return Err(SignerError::InvalidSignature);
        }
        let recovery = self
            .v
            .checked_sub(27)
            .filter(|v| *v <= 1)
            .and_then(RecoveryId::from_byte)
            .ok_or(SignerError::InvalidSignature)?;
        let key = VerifyingKey::recover_from_prehash(payload.digest(), &signature, recovery)
            .map_err(|_| SignerError::InvalidSignature)?;
        Ok(address_of(&key))
    }
}
fn address_of(key: &VerifyingKey) -> Address {
    let hash = keccak(&key.to_encoded_point(false).as_bytes()[1..]);
    Address::from_bytes(hash[12..].try_into().expect("20-byte hash suffix"))
}
fn scalar_hex(bytes: &[u8]) -> String {
    let encoded = hex::encode(bytes);
    format!("0x{}", encoded.trim_start_matches('0'))
}
fn decode_scalar(value: &str, output: &mut [u8]) -> Result<(), SignerError> {
    let value = value
        .strip_prefix("0x")
        .ok_or(SignerError::InvalidSignature)?;
    if value.is_empty() || value.len() > 64 {
        return Err(SignerError::InvalidSignature);
    }
    hex::decode_to_slice(format!("{value:0>64}"), output).map_err(|_| SignerError::InvalidSignature)
}

pub fn keccak(bytes: &[u8]) -> [u8; 32] {
    Keccak256::digest(bytes).into()
}

/// Pass typed actions, not sorted JSON maps: MessagePack field order is signed.
pub fn action_hash<A: Serialize>(
    action: &A,
    vault: Option<Address>,
    nonce: u64,
    expires_after: Option<u64>,
) -> Result<[u8; 32], Error> {
    let mut bytes = rmp_serde::to_vec_named(action)
        .map_err(|_| Error::InvalidRequest("unable to encode native action as MessagePack"))?;
    bytes.extend_from_slice(&nonce.to_be_bytes());
    match vault {
        None => bytes.push(0),
        Some(address) => {
            bytes.push(1);
            bytes.extend_from_slice(address.as_bytes());
        }
    }
    if let Some(expiry) = expires_after {
        bytes.push(0);
        bytes.extend_from_slice(&expiry.to_be_bytes());
    }
    Ok(keccak(&bytes))
}
pub fn l1_signing_payload<A: Serialize>(
    action: &A,
    vault: Option<Address>,
    nonce: u64,
    expires_after: Option<u64>,
    network: Network,
) -> Result<SigningPayload, Error> {
    let connection_id = action_hash(action, vault, nonce, expires_after)?;
    let source = match network {
        Network::Mainnet => "a",
        Network::Testnet => "b",
    };
    let agent = hash_words(&[
        keccak(b"Agent(string source,bytes32 connectionId)"),
        keccak(source.as_bytes()),
        connection_id,
    ]);
    Ok(SigningPayload {
        digest: eip712_digest("Exchange", 1337, agent),
        network,
        scheme: SigningScheme::L1Action,
    })
}

/// Offline primitives for the second native signing scheme. This SDK does not
/// submit transfers or agent approvals; those workflows remain outside v1.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum UserSignedAction {
    UsdSend {
        destination: Address,
        amount: String,
        time: u64,
    },
    ApproveAgent {
        agent_address: Address,
        agent_name: String,
        nonce: u64,
    },
}
impl UserSignedAction {
    pub fn signing_payload(
        &self,
        network: Network,
        signature_chain_id: u64,
    ) -> Result<SigningPayload, Error> {
        let chain = keccak(network.as_str().as_bytes());
        let body = match self {
            Self::UsdSend { destination, amount, time } => {
                crate::trading::validate_decimal(amount)?;
                hash_words(&[
                    keccak(b"HyperliquidTransaction:UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)"),
                    chain, keccak(destination.to_string().as_bytes()), keccak(amount.as_bytes()), word(*time),
                ])
            }
            Self::ApproveAgent { agent_address, agent_name, nonce } => hash_words(&[
                keccak(b"HyperliquidTransaction:ApproveAgent(string hyperliquidChain,address agentAddress,string agentName,uint64 nonce)"),
                chain, address_word(*agent_address), keccak(agent_name.as_bytes()), word(*nonce),
            ]),
        };
        Ok(SigningPayload {
            digest: eip712_digest("HyperliquidSignTransaction", signature_chain_id, body),
            network,
            scheme: SigningScheme::UserSignedAction,
        })
    }
}
fn word(value: u64) -> [u8; 32] {
    let mut bytes = [0; 32];
    bytes[24..].copy_from_slice(&value.to_be_bytes());
    bytes
}
fn address_word(address: Address) -> [u8; 32] {
    let mut bytes = [0; 32];
    bytes[12..].copy_from_slice(address.as_bytes());
    bytes
}
fn hash_words(words: &[[u8; 32]]) -> [u8; 32] {
    keccak(&words.concat())
}
fn eip712_digest(name: &str, chain_id: u64, body: [u8; 32]) -> [u8; 32] {
    let domain = hash_words(&[
        keccak(
            b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)",
        ),
        keccak(name.as_bytes()),
        keccak(b"1"),
        word(chain_id),
        [0; 32],
    ]);
    let mut bytes = Vec::with_capacity(66);
    bytes.extend_from_slice(&[0x19, 0x01]);
    bytes.extend_from_slice(&domain);
    bytes.extend_from_slice(&body);
    keccak(&bytes)
}
