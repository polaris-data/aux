//! Snapshot-based liquidity planning matching the pinned SDK's share and cap arithmetic.
use crate::{
    Address, Error, MetricChainClient, U256,
    liquidity::{LiquidityDeltas, percentage_removal},
    math::{Rounding, decode_bin_state},
};
use alloy_primitives::U512;
use aux_rs_core::{Error as ClientError, HttpTransport};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashSet};

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiquidityBin {
    pub bin: i16,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub token0_balance_scaled: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub token1_balance_scaled: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub total_shares: U256,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiquiditySnapshot {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub token0_scale_multiplier: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub token1_scale_multiplier: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub initial_scaled_token0_per_share_e18: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub initial_scaled_token1_per_share_e18: U256,
    pub lowest_bin: i16,
    pub highest_bin: i16,
    pub cur_bin_idx: i16,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub cur_pos_in_bin: U256,
    pub bins: Vec<LiquidityBin>,
    pub block: String,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BinTarget {
    pub bin: i16,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub target_value_in_token: U256,
    pub target_value_in_token0: bool,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AdditionRequest {
    pub snapshot: LiquiditySnapshot,
    pub targets: Vec<BinTarget>,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub price_e18: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub salt: U256,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub buffer_bps: u32,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionPlan {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub salt: U256,
    pub deltas: LiquidityDeltas,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub max_amount_token0: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub max_amount_token1: U256,
}
fn wad() -> U256 {
    U256::from(1_000_000_000_000_000_000u64)
}
fn max104() -> U256 {
    (U256::from(1) << 104usize) - U256::from(1)
}
fn product(values: &[U256]) -> Result<U512, Error> {
    values.iter().try_fold(U512::from(1), |p, n| {
        p.checked_mul(U512::from(*n)).ok_or(Error::InvalidRequest(
            "liquidity intermediate exceeds 512 bits",
        ))
    })
}
fn ratio(top: &[U256], bottom: &[U256], rounding: Rounding) -> Result<U256, Error> {
    let a = product(top)?;
    let b = product(bottom)?;
    if b.is_zero() {
        return Err(Error::InvalidRequest("zero liquidity denominator"));
    }
    let (mut q, r) = (a / b, a % b);
    if !r.is_zero() {
        match rounding {
            Rounding::Up => {
                q = q
                    .checked_add(U512::from(1))
                    .ok_or(Error::InvalidRequest("liquidity overflow"))?
            }
            Rounding::Down => {}
            Rounding::Exact => {
                return Err(Error::InvalidRequest("unsupported liquidity precision"));
            }
        }
    }
    if q.bit_len() > 256 {
        return Err(Error::InvalidRequest("liquidity value exceeds uint256"));
    }
    let l = q.as_limbs();
    Ok(U256::from_limbs([l[0], l[1], l[2], l[3]]))
}
fn add(a: U256, b: U256) -> Result<U256, Error> {
    a.checked_add(b)
        .ok_or(Error::InvalidRequest("liquidity sum overflow"))
}
impl LiquiditySnapshot {
    fn validate(&self) -> Result<BTreeMap<i16, &LiquidityBin>, Error> {
        if self.lowest_bin > self.highest_bin
            || self.cur_bin_idx < self.lowest_bin
            || self.cur_bin_idx > self.highest_bin
            || self.cur_pos_in_bin > max104()
            || self.token0_scale_multiplier.is_zero()
            || self.token1_scale_multiplier.is_zero()
            || self.initial_scaled_token0_per_share_e18.is_zero()
            || self.initial_scaled_token1_per_share_e18.is_zero()
        {
            return Err(Error::InvalidRequest(
                "invalid liquidity snapshot or scaling",
            ));
        }
        let mut bins = BTreeMap::new();
        for bin in &self.bins {
            if bin.bin < self.lowest_bin
                || bin.bin > self.highest_bin
                || bins.insert(bin.bin, bin).is_some()
                || bin.token0_balance_scaled > max104()
                || bin.token1_balance_scaled > max104()
            {
                return Err(Error::InvalidRequest("invalid or duplicate snapshot bin"));
            }
        }
        Ok(bins)
    }
}
pub fn addition_plan(request: &AdditionRequest) -> Result<AdditionPlan, Error> {
    let s = &request.snapshot;
    let states = s.validate()?;
    if request.price_e18.is_zero() || request.salt.bit_len() > 80 {
        return Err(Error::InvalidRequest(
            "positive price and uint80 salt required",
        ));
    }
    let mut targets = BTreeMap::new();
    for target in &request.targets {
        if target.target_value_in_token.is_zero()
            || !states.contains_key(&target.bin)
            || targets.insert(target.bin, target).is_some()
        {
            return Err(Error::InvalidRequest(
                "target requires a positive amount, a snapshot bin and a unique index",
            ));
        }
    }
    let mut shares_out = Vec::new();
    let mut bins_out = Vec::new();
    let mut cap0 = U256::ZERO;
    let mut cap1 = U256::ZERO;
    for (index, target) in targets {
        let bin = states[&index];
        let value = target.target_value_in_token;
        let (a0, a1) = if target.target_value_in_token0 {
            (
                value,
                ratio(
                    &[value, s.token0_scale_multiplier, request.price_e18],
                    &[s.token1_scale_multiplier, wad()],
                    Rounding::Down,
                )?,
            )
        } else {
            (
                ratio(
                    &[value, s.token1_scale_multiplier, wad()],
                    &[s.token0_scale_multiplier, request.price_e18],
                    Rounding::Down,
                )?,
                value,
            )
        };
        let target0 = product(&[a0, s.token0_scale_multiplier])?;
        let target1 = product(&[a1, s.token1_scale_multiplier])?;
        let narrow = |n: U512| -> Result<U256, Error> {
            if n.bit_len() > 256 {
                return Err(Error::InvalidRequest("scaled target overflow"));
            }
            let l = n.as_limbs();
            Ok(U256::from_limbs([l[0], l[1], l[2], l[3]]))
        };
        let (t0, t1) = (narrow(target0)?, narrow(target1)?);
        let below = index < s.cur_bin_idx || index == s.cur_bin_idx && s.cur_pos_in_bin == max104();
        let above = index > s.cur_bin_idx || index == s.cur_bin_idx && s.cur_pos_in_bin.is_zero();
        let shares = if below {
            if t1.is_zero() {
                return Err(Error::InvalidRequest("target rounds to zero token1"));
            }
            if !bin.total_shares.is_zero() {
                ratio(
                    &[t1, bin.total_shares],
                    &[bin.token1_balance_scaled],
                    Rounding::Up,
                )?
            } else {
                ratio(
                    &[t1, wad()],
                    &[s.initial_scaled_token1_per_share_e18],
                    Rounding::Up,
                )?
            }
        } else if above {
            if t0.is_zero() {
                return Err(Error::InvalidRequest("target rounds to zero token0"));
            }
            if !bin.total_shares.is_zero() {
                ratio(
                    &[t0, bin.total_shares],
                    &[bin.token0_balance_scaled],
                    Rounding::Up,
                )?
            } else {
                ratio(
                    &[t0, wad()],
                    &[s.initial_scaled_token0_per_share_e18],
                    Rounding::Up,
                )?
            }
        } else if !bin.total_shares.is_zero() {
            let total = add(
                bin.token1_balance_scaled,
                ratio(
                    &[bin.token0_balance_scaled, request.price_e18],
                    &[wad()],
                    Rounding::Down,
                )?,
            )?;
            ratio(&[t1, bin.total_shares], &[total], Rounding::Down)?
        } else {
            let part0 = max104() - s.cur_pos_in_bin;
            let part1 = s.cur_pos_in_bin;
            let value0 = ratio(
                &[
                    s.initial_scaled_token0_per_share_e18,
                    part0,
                    request.price_e18,
                ],
                &[wad()],
                Rounding::Down,
            )?;
            let value1 = ratio(
                &[s.initial_scaled_token1_per_share_e18, part1],
                &[U256::from(1)],
                Rounding::Down,
            )?;
            let total = add(value0, value1)? / max104();
            ratio(&[t1, wad()], &[total], Rounding::Down)?
        };
        let shares = shares.max(U256::from(1));
        if shares > ((U256::from(1) << 103usize) - U256::from(1)) {
            return Err(Error::InvalidRequest("shares exceed int104 max"));
        }
        let (amount0, amount1) = if !bin.total_shares.is_zero() {
            (
                ratio(
                    &[shares, bin.token0_balance_scaled],
                    &[bin.total_shares],
                    Rounding::Up,
                )?,
                ratio(
                    &[shares, bin.token1_balance_scaled],
                    &[bin.total_shares],
                    Rounding::Up,
                )?,
            )
        } else if below {
            (
                U256::ZERO,
                ratio(
                    &[shares, s.initial_scaled_token1_per_share_e18],
                    &[wad()],
                    Rounding::Up,
                )?,
            )
        } else if above {
            (
                ratio(
                    &[shares, s.initial_scaled_token0_per_share_e18],
                    &[wad()],
                    Rounding::Up,
                )?,
                U256::ZERO,
            )
        } else {
            (
                ratio(
                    &[
                        shares,
                        s.initial_scaled_token0_per_share_e18,
                        max104() - s.cur_pos_in_bin,
                    ],
                    &[wad(), max104()],
                    Rounding::Up,
                )?,
                ratio(
                    &[
                        shares,
                        s.initial_scaled_token1_per_share_e18,
                        s.cur_pos_in_bin,
                    ],
                    &[wad(), max104()],
                    Rounding::Up,
                )?,
            )
        };
        cap0 = add(
            cap0,
            ratio(&[amount0], &[s.token0_scale_multiplier], Rounding::Up)?,
        )?;
        cap1 = add(
            cap1,
            ratio(&[amount1], &[s.token1_scale_multiplier], Rounding::Up)?,
        )?;
        if cap0 > U256::from(i128::MAX as u128) || cap1 > U256::from(i128::MAX as u128) {
            return Err(Error::InvalidRequest("liquidity caps exceed int128 max"));
        }
        bins_out.push(index);
        shares_out.push(shares);
    }
    let buffer = U256::from(10_000u64 + u64::from(request.buffer_bps));
    cap0 = ratio(&[cap0, buffer], &[U256::from(10_000)], Rounding::Down)?;
    cap1 = ratio(&[cap1, buffer], &[U256::from(10_000)], Rounding::Down)?;
    if cap0 > U256::from(i128::MAX as u128) || cap1 > U256::from(i128::MAX as u128) {
        return Err(Error::InvalidRequest("buffered caps exceed int128 max"));
    }
    Ok(AdditionPlan {
        salt: request.salt,
        deltas: LiquidityDeltas {
            bin_idxs: bins_out,
            shares: shares_out,
        },
        max_amount_token0: cap0,
        max_amount_token1: cap1,
    })
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WeightedRequest {
    pub snapshot: LiquiditySnapshot,
    pub bins: Vec<i16>,
    #[cfg_attr(feature = "typescript", ts(type = "Array<string>"))]
    pub weights: Vec<U256>,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub price_e18: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub salt: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub max_amount_token0: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub max_amount_token1: U256,
}
pub fn weighted_plan(request: &WeightedRequest) -> Result<AdditionPlan, Error> {
    let s = &request.snapshot;
    s.validate()?;
    if request.bins.is_empty()
        || request.bins.len() != request.weights.len()
        || request.weights.iter().any(U256::is_zero)
        || request.price_e18.is_zero()
    {
        return Err(Error::InvalidRequest(
            "positive weights matching nonempty bins and a positive price required",
        ));
    }
    let needs0 = request
        .bins
        .iter()
        .any(|bin| *bin > s.cur_bin_idx || *bin == s.cur_bin_idx && s.cur_pos_in_bin < max104());
    let needs1 = request
        .bins
        .iter()
        .any(|bin| *bin < s.cur_bin_idx || *bin == s.cur_bin_idx && !s.cur_pos_in_bin.is_zero());
    if needs0 && request.max_amount_token0.is_zero()
        || needs1 && request.max_amount_token1.is_zero()
    {
        return Err(Error::InvalidRequest(
            "selected bins require positive caps for each needed token",
        ));
    }
    let unit0 = ratio(
        &[s.token0_scale_multiplier, request.price_e18],
        &[U256::from(1)],
        Rounding::Down,
    )?;
    let unit1 = ratio(
        &[s.token1_scale_multiplier, wad()],
        &[U256::from(1)],
        Rounding::Down,
    )?;
    let use0 = needs0
        && (!needs1
            || product(&[request.max_amount_token0, unit0])?
                <= product(&[request.max_amount_token1, unit1])?);
    let minimum = *request
        .weights
        .iter()
        .min()
        .expect("validated nonempty weights");
    let units = request
        .weights
        .iter()
        .map(|w| ratio(&[*w, U256::from(1_000_000)], &[minimum], Rounding::Up))
        .collect::<Result<Vec<_>, _>>()?;
    let total = units.iter().try_fold(U256::ZERO, |sum, w| add(sum, *w))?;
    let minimum_target = if use0 && needs1 {
        ratio(&[unit1], &[unit0], Rounding::Up)?
    } else if !use0 && needs0 {
        ratio(&[unit0], &[unit1], Rounding::Up)?
    } else {
        U256::from(1)
    };
    let budget = if use0 {
        request.max_amount_token0
    } else {
        request.max_amount_token1
    };
    let calibration = budget.max(ratio(
        &[total, minimum_target],
        &[U256::from(1)],
        Rounding::Down,
    )?);
    let targets = request
        .bins
        .iter()
        .zip(units)
        .map(|(bin, w)| {
            Ok(BinTarget {
                bin: *bin,
                target_value_in_token: ratio(&[calibration, w], &[total], Rounding::Down)?,
                target_value_in_token0: use0,
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let plan = addition_plan(&AdditionRequest {
        snapshot: s.clone(),
        targets,
        price_e18: request.price_e18,
        salt: request.salt,
        buffer_bps: 0,
    })?;
    Ok(AdditionPlan {
        max_amount_token0: request.max_amount_token0,
        max_amount_token1: request.max_amount_token1,
        ..plan
    })
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemovalSpec {
    pub bin: i16,
    #[cfg_attr(feature = "typescript", ts(type = "(string) | null", optional = nullable))]
    pub shares_to_remove: Option<U256>,
    pub bps: Option<u16>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PositionShares {
    pub bin: i16,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub shares: U256,
}
pub fn removal_plan(
    positions: &[PositionShares],
    specs: &[RemovalSpec],
) -> Result<LiquidityDeltas, Error> {
    let positions: BTreeMap<_, _> = positions.iter().map(|p| (p.bin, p.shares)).collect();
    let mut seen = HashSet::new();
    let mut out = BTreeMap::new();
    for spec in specs {
        if !seen.insert(spec.bin) {
            return Err(Error::InvalidRequest("duplicate removal bin"));
        }
        let current = positions.get(&spec.bin).copied().unwrap_or_default();
        let shares = match (spec.shares_to_remove, spec.bps) {
            (Some(shares), None) => shares.min(current),
            (None, Some(bps)) if bps > 0 => percentage_removal(current, bps)?,
            _ => {
                return Err(Error::InvalidRequest(
                    "supply either positive bps or explicit shares per removal bin",
                ));
            }
        };
        if !shares.is_zero() {
            out.insert(spec.bin, shares);
        }
    }
    Ok(LiquidityDeltas {
        bin_idxs: out.keys().copied().collect(),
        shares: out.values().copied().collect(),
    })
}

fn uint(v: &Value) -> Result<U256, Error> {
    v.as_str()
        .map(str::to_string)
        .unwrap_or_else(|| v.to_string())
        .parse()
        .map_err(|_| Error::InvalidRequest("invalid chain integer"))
}
impl<T: HttpTransport> MetricChainClient<T> {
    /// Read every planning input at one concrete block, without keeping application state.
    pub async fn liquidity_snapshot(
        &self,
        pool: Address,
        lens: Address,
        bins: &[i16],
        block: &str,
    ) -> Result<LiquiditySnapshot, ClientError<Error>> {
        let local = ClientError::Exchange;
        if bins.len() > 512 {
            return Err(local(Error::InvalidRequest(
                "snapshot batch is limited to 512 bins",
            )));
        }
        let block = if block == "latest" {
            format!("0x{:x}", self.block_number().await?)
        } else {
            block.to_string()
        };
        let block_ref = &block;
        let read = |contract: &'static str, function: &'static str, args: Value| async move {
            let data = crate::encode_contract_call(contract, function, &args).map_err(local)?;
            let raw = self
                .eth_call(
                    if contract == "MetricOmmPool" {
                        pool
                    } else {
                        lens
                    },
                    data,
                    None,
                    U256::ZERO,
                    block_ref,
                )
                .await?;
            crate::decode_contract_output(contract, function, &raw).map_err(local)
        };
        let imm = read("MetricOmmPool", "getImmutables", json!([])).await?;
        let slot = read("MetricOmmPoolDataProvider", "slot0", json!([pool])).await?;
        let packed = read(
            "MetricOmmPoolDataProvider",
            "binStates",
            json!([pool, bins]),
        )
        .await?;
        let shares = read(
            "MetricOmmPoolDataProvider",
            "binTotalShares(address,int16[])",
            json!([pool, bins]),
        )
        .await?;
        let packed = packed[0]
            .as_array()
            .ok_or(local(Error::InvalidRequest("missing bin states")))?;
        let shares = shares[0]
            .as_array()
            .ok_or(local(Error::InvalidRequest("missing bin shares")))?;
        if packed.len() != bins.len() || shares.len() != bins.len() {
            return Err(local(Error::InvalidRequest("bin response count mismatch")));
        }
        let states = bins
            .iter()
            .zip(packed)
            .zip(shares)
            .map(|((bin, packed), shares)| {
                let decoded = decode_bin_state(uint(packed)?);
                Ok(LiquidityBin {
                    bin: *bin,
                    token0_balance_scaled: decoded.token0_balance_scaled,
                    token1_balance_scaled: decoded.token1_balance_scaled,
                    total_shares: uint(shares)?,
                })
            })
            .collect::<Result<Vec<_>, Error>>()
            .map_err(local)?;
        let int16 = |v: &Value| {
            v.as_i64()
                .and_then(|n| i16::try_from(n).ok())
                .ok_or(local(Error::InvalidRequest("invalid native bin index")))
        };
        let snapshot = LiquiditySnapshot {
            token0_scale_multiplier: uint(&imm[0][3]).map_err(local)?,
            token1_scale_multiplier: uint(&imm[0][4]).map_err(local)?,
            initial_scaled_token0_per_share_e18: uint(&imm[0][5]).map_err(local)?,
            initial_scaled_token1_per_share_e18: uint(&imm[0][6]).map_err(local)?,
            lowest_bin: int16(&imm[0][9])?,
            highest_bin: int16(&imm[0][10])?,
            cur_bin_idx: int16(&slot[1])?,
            cur_pos_in_bin: uint(&slot[2]).map_err(local)?,
            bins: states,
            block,
        };
        snapshot.validate().map_err(local)?;
        Ok(snapshot)
    }
}
