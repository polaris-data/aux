//! Integer-only native packing and arithmetic. Rounding follows explicit native boundaries.
use crate::{Error, U256};
use alloy_primitives::U512;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinData {
    pub length_e6: u16,
    pub add_fee_buy_e6: u16,
    pub add_fee_sell_e6: u16,
}
pub fn pack_bin_data(bin: BinData) -> u64 {
    u64::from(bin.length_e6)
        | (u64::from(bin.add_fee_buy_e6) << 16)
        | (u64::from(bin.add_fee_sell_e6) << 32)
}
pub fn unpack_bin_data(word: u64) -> BinData {
    BinData {
        length_e6: word as u16,
        add_fee_buy_e6: (word >> 16usize) as u16,
        add_fee_sell_e6: (word >> 32usize) as u16,
    }
}
pub fn pack_bin_data_array(bins: &[BinData]) -> Vec<U256> {
    bins.chunks(5)
        .map(|chunk| {
            chunk.iter().enumerate().fold(U256::ZERO, |w, (i, b)| {
                w | (U256::from(pack_bin_data(*b)) << (i * 48))
            })
        })
        .collect()
}
pub fn unpack_bin_data_array(words: &[U256]) -> Vec<BinData> {
    let mut bins = Vec::new();
    for w in words {
        for i in 0..5 {
            let n: U256 = *w >> (i * 48);
            let b = unpack_bin_data(n.as_limbs()[0]);
            if b.length_e6 == 0 {
                return bins;
            }
            bins.push(b);
        }
    }
    bins
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinState {
    pub token0_balance_scaled: U256,
    pub token1_balance_scaled: U256,
    pub length_e6: u16,
    pub add_fee_buy_e6: u16,
    pub add_fee_sell_e6: u16,
}
pub fn encode_bin_state(state: &BinState) -> Result<U256, Error> {
    if state.token0_balance_scaled.bit_len() > 104 || state.token1_balance_scaled.bit_len() > 104 {
        return Err(Error::InvalidRequest("bin balance exceeds uint104"));
    }
    Ok(state.token0_balance_scaled
        | (state.token1_balance_scaled << 104)
        | (U256::from(state.length_e6) << 208)
        | (U256::from(state.add_fee_buy_e6) << 224)
        | (U256::from(state.add_fee_sell_e6) << 240))
}
pub fn decode_bin_state(word: U256) -> BinState {
    let mask = (U256::from(1) << 104) - U256::from(1);
    BinState {
        token0_balance_scaled: word & mask,
        token1_balance_scaled: (word >> 104usize) & mask,
        length_e6: ((word >> 208usize) & U256::from(65535)).to::<u16>(),
        add_fee_buy_e6: ((word >> 224usize) & U256::from(65535)).to::<u16>(),
        add_fee_sell_e6: ((word >> 240usize) & U256::from(65535)).to::<u16>(),
    }
}
pub fn unpack_two_uint128(word: U256) -> (u128, u128) {
    let l = word.as_limbs();
    (
        (u128::from(l[3]) << 64) | u128::from(l[2]),
        (u128::from(l[1]) << 64) | u128::from(l[0]),
    )
}
pub fn unpack_two_int128(word: U256) -> (i128, i128) {
    let (h, l) = unpack_two_uint128(word);
    (h as i128, l as i128)
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapDetails {
    pub exact_input: bool,
    pub bin_idx: i16,
    pub pos_in_bin: U256,
}
pub fn unpack_swap_details(word: U256) -> SwapDetails {
    SwapDetails {
        exact_input: word.bit(120),
        bin_idx: ((word >> 104usize) & U256::from(65535)).to::<u16>() as i16,
        pos_in_bin: word & ((U256::from(1) << 104) - U256::from(1)),
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Rounding {
    Down,
    Up,
    Exact,
}
pub fn mul_div(a: U256, b: U256, denominator: U256, rounding: Rounding) -> Result<U256, Error> {
    if denominator.is_zero() {
        return Err(Error::InvalidRequest("division by zero"));
    }
    let product = U512::from(a) * U512::from(b);
    let d = U512::from(denominator);
    let (mut q, r) = (product / d, product % d);
    if !r.is_zero() {
        match rounding {
            Rounding::Up => {
                q = q
                    .checked_add(U512::from(1))
                    .ok_or(Error::InvalidRequest("arithmetic overflow"))?
            }
            Rounding::Exact => {
                return Err(Error::InvalidRequest("value has unsupported precision"));
            }
            Rounding::Down => {}
        }
    }
    if q.bit_len() > 256 {
        return Err(Error::InvalidRequest("arithmetic exceeds uint256"));
    }
    let l = q.as_limbs();
    Ok(U256::from_limbs([l[0], l[1], l[2], l[3]]))
}
pub fn decimal_to_raw(value: &str, decimals: u8) -> Result<U256, Error> {
    if decimals > 77 {
        return Err(Error::InvalidRequest("decimal scale exceeds uint256"));
    }
    let mut parts = value.split('.');
    let whole = parts.next().unwrap_or_default();
    let fraction = parts.next().unwrap_or_default();
    if parts.next().is_some()
        || whole.is_empty()
        || !whole
            .bytes()
            .chain(fraction.bytes())
            .all(|b| b.is_ascii_digit())
    {
        return Err(Error::InvalidRequest("unsigned decimal string required"));
    }
    if fraction.len() > usize::from(decimals)
        && fraction[usize::from(decimals)..].bytes().any(|b| b != b'0')
    {
        return Err(Error::InvalidRequest("fractional native unit"));
    }
    let fraction = &fraction[..fraction.len().min(usize::from(decimals))];
    let digits = format!(
        "{whole}{fraction}{}",
        "0".repeat(usize::from(decimals) - fraction.len())
    );
    U256::from_str_radix(&digits, 10).map_err(|_| Error::InvalidRequest("scaled decimal overflow"))
}
pub fn price_to_q64(numerator: U256, denominator: U256, rounding: Rounding) -> Result<U256, Error> {
    mul_div(numerator, U256::from(1) << 64, denominator, rounding)
}
pub fn price_x64_to_e18(price: U256) -> Result<U256, Error> {
    if price.is_zero() {
        return Err(Error::InvalidRequest("positive price required"));
    }
    let p = mul_div(
        price,
        U256::from(1_000_000_000_000_000_000u64),
        U256::from(1) << 64,
        Rounding::Down,
    )?;
    if p.is_zero() {
        return Err(Error::InvalidRequest("price below E18 precision"));
    }
    Ok(p)
}
pub fn slippage_limit(amount: U256, bps: u16, exact_output: bool) -> Result<U256, Error> {
    if bps > 10_000 {
        return Err(Error::InvalidRequest("slippage exceeds 10000 bps"));
    }
    mul_div(
        amount,
        U256::from(if exact_output {
            10_000u32 + u32::from(bps)
        } else {
            10_000u32 - u32::from(bps)
        }),
        U256::from(10_000),
        if exact_output {
            Rounding::Up
        } else {
            Rounding::Down
        },
    )
}
pub fn direction_bitmap(directions: &[bool]) -> Result<U256, Error> {
    if directions.len() > 256 {
        return Err(Error::InvalidRequest("path exceeds bitmap width"));
    }
    Ok(directions.iter().enumerate().fold(U256::ZERO, |w, (i, b)| {
        if *b { w | (U256::from(1) << i) } else { w }
    }))
}
pub fn position_bin_key(owner: crate::Address, salt: U256, bin: i16) -> Result<crate::B256, Error> {
    if salt.bit_len() > 80 {
        return Err(Error::InvalidRequest("position salt exceeds uint80"));
    }
    let mut packed = Vec::with_capacity(32);
    packed.extend_from_slice(owner.as_slice());
    packed.extend_from_slice(&salt.to_be_bytes::<32>()[22..]);
    packed.extend_from_slice(&bin.to_be_bytes());
    Ok(crate::B256::from_slice(&packed))
}
