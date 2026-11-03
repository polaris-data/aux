//! CompressedOracleV1 packet and signature helpers, pinned to Metric's MIT pusher reference.
use crate::{Address, B256, Bytes, Error, EvmSigner, Signature, U256, math::Rounding};
use alloy_sol_types::SolValue;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub fn codebook() -> &'static [u16] {
    static TABLE: OnceLock<Vec<u16>> = OnceLock::new();
    TABLE.get_or_init(|| {
        serde_json::from_str(include_str!("../schema/codebook.json")).expect("pinned codebook JSON")
    })
}
pub fn encode_spread(units: u16) -> Result<u8, Error> {
    if units > 10_000 {
        return Err(Error::InvalidRequest(
            "spread exceeds 10000 native 0.01-bps units",
        ));
    }
    Ok(codebook().partition_point(|value| *value < units) as u8)
}
pub fn decode_u64x32(word: u32) -> u64 {
    u64::from(word & ((1 << 27) - 1)) << (word >> 27)
}
pub fn encode_u64x32(value: u64, rounding: Rounding) -> Result<u32, Error> {
    const MAX: u64 = ((1u64 << 27) - 1) << 31;
    if value > MAX {
        return Err(Error::InvalidRequest("price exceeds U64x32 range"));
    }
    let mut m = value;
    let mut e = 0;
    while m >= (1 << 27) {
        m >>= 1;
        e += 1;
    }
    if decode_u64x32((e << 27) | m as u32) != value {
        match rounding {
            Rounding::Exact => {
                return Err(Error::InvalidRequest(
                    "price is not exactly representable in U64x32",
                ));
            }
            Rounding::Up => {
                return Err(Error::InvalidRequest(
                    "native U64x32 encoding only supports truncation or exact values",
                ));
            }
            Rounding::Down => {}
        }
    }
    Ok((e << 27) | m as u32)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OracleLane {
    #[serde(deserialize_with = "exact_u64")]
    pub price: u64,
    pub spread0: u16,
    pub spread1: u16,
}
fn exact_u64<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Exact {
        Integer(u64),
        String(String),
    }
    match Exact::deserialize(deserializer)? {
        Exact::Integer(n) => Ok(n),
        Exact::String(s) => s
            .parse()
            .map_err(|_| serde::de::Error::custom("exact uint64 price required")),
    }
}
pub fn pack_oracle_slot(
    lanes: &[Option<OracleLane>; 4],
    timestamp_ms: u64,
    slot_id: u8,
    rounding: Rounding,
) -> Result<U256, Error> {
    if timestamp_ms >= (1u64 << 56) {
        return Err(Error::InvalidRequest("oracle timestamp exceeds uint56"));
    }
    let mut word = (U256::from(timestamp_ms) << 8usize) | U256::from(slot_id);
    for (i, lane) in lanes.iter().enumerate() {
        let bits = if let Some(lane) = lane {
            (u64::from(encode_u64x32(lane.price, rounding)?) << 16)
                | (u64::from(encode_spread(lane.spread0)?) << 8)
                | u64::from(encode_spread(lane.spread1)?)
        } else {
            65535
        };
        word |= U256::from(bits) << (208 - i * 48);
    }
    Ok(word)
}
pub fn oracle_push_calldata(words: &[U256]) -> Result<Bytes, Error> {
    if words.is_empty() {
        return Err(Error::InvalidRequest(
            "at least one oracle slot is required",
        ));
    }
    let mut bytes = vec![0];
    for word in words {
        bytes.extend_from_slice(&word.to_be_bytes::<32>());
    }
    Ok(bytes.into())
}
pub fn feed_id(creator: Address, chain_id: U256, slot: u8, position: u8) -> Result<B256, Error> {
    if creator.is_zero() || chain_id.bit_len() > 80 || position > 3 {
        return Err(Error::InvalidRequest(
            "invalid oracle namespace coordinates",
        ));
    }
    let raw = (U256::from_be_slice(creator.as_slice()) << 96usize)
        | (chain_id << 16usize)
        | (U256::from(slot) << 8usize)
        | U256::from(position);
    Ok(B256::from(raw.to_be_bytes::<32>()))
}
pub fn slot_update_digest(chain_id: U256, oracle: Address, creator: Address, word: U256) -> B256 {
    alloy_primitives::keccak256((chain_id, oracle, creator, word).abi_encode())
}
pub fn pusher_consent_digest(
    chain_id: U256,
    oracle: Address,
    deadline: U256,
    pusher: Address,
    creator: Address,
) -> B256 {
    alloy_primitives::keccak256((chain_id, oracle, deadline, pusher, creator).abi_encode())
}
pub async fn sign_personal_digest<S: EvmSigner>(
    signer: &S,
    digest: B256,
) -> Result<Signature, Error> {
    let mut bytes = b"\x19Ethereum Signed Message:\n32".to_vec();
    bytes.extend_from_slice(digest.as_slice());
    let hash = alloy_primitives::keccak256(bytes);
    let sig = signer.sign_digest(hash).await?;
    if sig
        .recover_address_from_prehash(&hash)
        .map_err(|_| Error::Signing("invalid external oracle signature"))?
        != signer.address()
    {
        return Err(Error::Signing("oracle signature does not match signer"));
    }
    Ok(sig)
}
