use crate::{
    Error, U256,
    math::{Rounding, mul_div},
};
use serde::{Deserialize, Serialize};
fn add(a: U256, b: U256) -> Result<U256, Error> {
    a.checked_add(b)
        .ok_or(Error::InvalidRequest("liquidity arithmetic overflow"))
}
fn wad() -> U256 {
    U256::from(1_000_000_000_000_000_000u64)
}
fn metric_max() -> U256 {
    (U256::from(1) << 104usize) - U256::from(1)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiquidityDeltas {
    pub bin_idxs: Vec<i16>,
    pub shares: Vec<U256>,
}
impl LiquidityDeltas {
    pub fn validate(&self) -> Result<(), Error> {
        if self.bin_idxs.is_empty() || self.bin_idxs.len() != self.shares.len() {
            return Err(Error::InvalidRequest(
                "liquidity bins and shares must have equal nonzero lengths",
            ));
        }
        let mut bins = std::collections::HashSet::new();
        if self.bin_idxs.iter().any(|b| !bins.insert(*b)) {
            return Err(Error::InvalidRequest("duplicate liquidity bins"));
        }
        Ok(())
    }
}
pub fn scale_weighted_deltas(
    weights: &LiquidityDeltas,
    probe_need0: U256,
    probe_need1: U256,
    max0: U256,
    max1: U256,
) -> Result<LiquidityDeltas, Error> {
    weights.validate()?;
    if probe_need0.is_zero() && probe_need1.is_zero() {
        return Err(Error::InvalidRequest(
            "liquidity probe requires a nonzero token need",
        ));
    }
    let count = U256::from(weights.bin_idxs.len());
    let adjusted = |cap: U256| if cap > count { cap - count } else { cap };
    let unlimited = U256::MAX - U256::from(1);
    let s0 = if probe_need0.is_zero() {
        unlimited
    } else {
        mul_div(adjusted(max0), wad(), probe_need0, Rounding::Down)?
    };
    let s1 = if probe_need1.is_zero() {
        unlimited
    } else {
        mul_div(adjusted(max1), wad(), probe_need1, Rounding::Down)?
    };
    let scale = s0.min(s1);
    let shares = weights
        .shares
        .iter()
        .map(|w| {
            let share = mul_div(*w, scale, wad(), Rounding::Down)?;
            if !w.is_zero() && share.is_zero() {
                return Err(Error::InvalidRequest("weighted shares round to zero"));
            }
            Ok(share)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(LiquidityDeltas {
        bin_idxs: weights.bin_idxs.clone(),
        shares,
    })
}
pub fn percentage_removal(shares: U256, bps: u16) -> Result<U256, Error> {
    if bps > 10_000 {
        return Err(Error::InvalidRequest(
            "removal percentage exceeds 10000 bps",
        ));
    }
    mul_div(shares, U256::from(bps), U256::from(10_000), Rounding::Down)
}
pub fn decayed_high_watermark(hwm: U256, rate: U256, seconds: u64) -> Result<U256, Error> {
    let factor = rate
        .checked_mul(U256::from(seconds))
        .ok_or(Error::InvalidRequest("decay arithmetic overflow"))?;
    if factor >= wad() {
        Ok(U256::ZERO)
    } else {
        Ok(hwm - mul_div(hwm, factor, wad(), Rounding::Down)?)
    }
}
pub fn stop_loss_threshold(hwm: U256, drawdown_e6: u32) -> Result<U256, Error> {
    if drawdown_e6 > 1_000_000 {
        return Err(Error::InvalidRequest("drawdown exceeds E6 unit"));
    }
    mul_div(
        hwm,
        U256::from(1_000_000 - drawdown_e6),
        U256::from(1_000_000),
        Rounding::Down,
    )
}
pub fn stop_loss_breached(metric: U256, hwm: U256, drawdown_e6: u32) -> Result<bool, Error> {
    Ok(drawdown_e6 != 0 && metric < stop_loss_threshold(hwm, drawdown_e6)?)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StopLossInput {
    pub bid_price_x64: U256,
    pub ask_price_x64: U256,
    pub token0_balance_scaled: U256,
    pub token1_balance_scaled: U256,
    pub total_shares: U256,
    pub minimal_operational_liquidity: U256,
}
pub fn stop_loss_metrics(input: &StopLossInput) -> Result<(U256, U256), Error> {
    if input.bid_price_x64.is_zero() || input.ask_price_x64.is_zero() {
        return Err(Error::InvalidRequest("positive oracle prices required"));
    }
    let shares = input
        .total_shares
        .max(input.minimal_operational_liquidity)
        .max(U256::from(1));
    let t0 = mul_div(input.token0_balance_scaled, wad(), shares, Rounding::Down)?;
    let t1 = mul_div(input.token1_balance_scaled, wad(), shares, Rounding::Down)?;
    let q64 = U256::from(1) << 64usize;
    Ok((
        add(t0, mul_div(t1, q64, input.bid_price_x64, Rounding::Down)?)?.min(metric_max()),
        add(mul_div(t0, input.ask_price_x64, q64, Rounding::Down)?, t1)?.min(metric_max()),
    ))
}
pub fn seconds_until_stop_loss_clears(
    metric: U256,
    hwm: U256,
    drawdown_e6: u32,
    rate: U256,
) -> Result<Option<u64>, Error> {
    if !stop_loss_breached(metric, hwm, drawdown_e6)? {
        return Ok(Some(0));
    }
    if rate.is_zero() {
        return Ok(None);
    }
    let max = mul_div(wad(), U256::from(1), rate, Rounding::Up)?;
    let mut hi =
        u64::try_from(max).map_err(|_| Error::InvalidRequest("decay duration exceeds u64"))?;
    let mut lo = 0;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if stop_loss_breached(metric, decayed_high_watermark(hwm, rate, mid)?, drawdown_e6)? {
            lo = mid + 1
        } else {
            hi = mid
        }
    }
    Ok(Some(lo))
}
fn isqrt(n: U256) -> U256 {
    if n < U256::from(2) {
        return n;
    }
    let mut x = U256::from(1) << n.bit_len().div_ceil(2);
    loop {
        let y = (x + n / x) >> 1usize;
        if y >= x {
            return x;
        }
        x = y;
    }
}
pub fn recommended_initial_scaled_amounts(price_e18: U256) -> Result<(U256, U256), Error> {
    if price_e18.is_zero() {
        return Err(Error::InvalidRequest("positive E18 price required"));
    }
    let k = metric_max() * U256::from(1_000_000);
    let a = isqrt(mul_div(k, wad(), price_e18, Rounding::Down)?);
    let b = isqrt(mul_div(k, price_e18, wad(), Rounding::Down)?);
    let max = U256::from(u128::MAX);
    if a.is_zero() || b.is_zero() || a >= max || b >= max {
        return Err(Error::InvalidRequest(
            "initial scaled amounts exceed factory bounds",
        ));
    }
    Ok((a, b))
}
pub fn minimum_operational_liquidity(amount0: U256, amount1: U256) -> Result<U256, Error> {
    let max = U256::from(u128::MAX);
    if amount0.is_zero() || amount1.is_zero() || amount0 >= max || amount1 >= max {
        return Err(Error::InvalidRequest(
            "initial amounts must be positive and below uint128.max",
        ));
    }
    Ok((U256::from(1_000_000) * wad() / amount0.min(amount1)).max(U256::from(1)))
}
pub fn extension_order(slots: &[u8]) -> Result<U256, Error> {
    if slots.len() > 7 || slots.iter().any(|s| *s > 7) {
        return Err(Error::InvalidRequest(
            "extension order requires at most seven 3-bit slots",
        ));
    }
    Ok(slots
        .iter()
        .enumerate()
        .fold(U256::ZERO, |w, (i, s)| w | (U256::from(*s) << (i * 3))))
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CursorInput {
    pub cur_bin_idx: i16,
    pub cur_pos_in_bin: U256,
    pub lowest_bin: i16,
    pub highest_bin: i16,
    pub bin_lengths_e6: Vec<u16>,
    pub distance_e12: U256,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CursorBounds {
    pub minimal_cur_bin: i16,
    pub minimal_position: U256,
    pub maximal_cur_bin: i16,
    pub maximal_position: U256,
}
pub fn liquidity_cursor_bounds(input: &CursorInput) -> Result<CursorBounds, Error> {
    if input.lowest_bin > input.highest_bin
        || input.bin_lengths_e6.len()
            != (i32::from(input.highest_bin) - i32::from(input.lowest_bin) + 1) as usize
        || input.cur_bin_idx < input.lowest_bin
        || input.cur_bin_idx > input.highest_bin
        || input.cur_pos_in_bin > metric_max()
    {
        return Err(Error::InvalidRequest(
            "invalid liquidity cursor or bin range",
        ));
    }
    let budget = input
        .distance_e12
        .checked_mul(metric_max())
        .ok_or(Error::InvalidRequest("cursor distance overflow"))?;
    let walk = |up: bool| -> Result<(i16, U256), Error> {
        let mut bin = input.cur_bin_idx;
        let mut pos = input.cur_pos_in_bin;
        let mut remaining = budget;
        loop {
            let length = U256::from(
                input.bin_lengths_e6[(i32::from(bin) - i32::from(input.lowest_bin)) as usize],
            ) * U256::from(1_000_000);
            let room = length * if up { metric_max() - pos } else { pos };
            if remaining <= room && !length.is_zero() {
                let step = remaining / length;
                return Ok((bin, if up { pos + step } else { pos - step }));
            }
            if bin
                == if up {
                    input.highest_bin
                } else {
                    input.lowest_bin
                }
            {
                return Ok((bin, if up { metric_max() } else { U256::ZERO }));
            }
            remaining -= room;
            bin = if up {
                bin.checked_add(1)
            } else {
                bin.checked_sub(1)
            }
            .ok_or(Error::InvalidRequest("cursor bin overflow"))?;
            pos = if up { U256::ZERO } else { metric_max() };
        }
    };
    let (minimal_cur_bin, minimal_position) = walk(false)?;
    let (maximal_cur_bin, maximal_position) = walk(true)?;
    Ok(CursorBounds {
        minimal_cur_bin,
        minimal_position,
        maximal_cur_bin,
        maximal_position,
    })
}
