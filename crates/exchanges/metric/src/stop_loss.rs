//! Combined per-bin stop-loss evaluation matching the pinned official SDK.
use crate::{
    Error, U256,
    liquidity::{
        StopLossInput, seconds_until_stop_loss_clears, stop_loss_breached, stop_loss_metrics,
        stop_loss_threshold,
    },
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StopLossBinInput {
    pub bin_idx: i16,
    pub token0_balance_scaled: U256,
    pub token1_balance_scaled: U256,
    pub total_shares: U256,
    /// Current, already decayed watermark from the same observation as the balances.
    pub high_watermark0: U256,
    pub high_watermark1: U256,
}
fn yes() -> bool {
    true
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StopLossEvaluationRequest {
    pub bins: Vec<StopLossBinInput>,
    pub bid_price_x64: U256,
    pub ask_price_x64: U256,
    pub drawdown_e6: u32,
    pub decay_per_second_e18: U256,
    pub minimal_operational_liquidity: U256,
    #[serde(default = "yes")]
    pub skip_empty_bins: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLossBinStatus {
    pub bin_idx: i16,
    pub metric_token0: U256,
    pub metric_token1: U256,
    pub high_watermark0: U256,
    pub high_watermark1: U256,
    pub threshold0: U256,
    pub threshold1: U256,
    pub breached_token0: bool,
    pub breached_token1: bool,
    pub blocks_zero_for_one: bool,
    pub blocks_one_for_zero: bool,
    pub seconds_until_zero_for_one_resumes: Option<u64>,
    pub seconds_until_one_for_zero_resumes: Option<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLossEvaluation {
    pub enabled: bool,
    pub bins: Vec<StopLossBinStatus>,
    pub breached_bins: Vec<StopLossBinStatus>,
    pub is_any_bin_stop_loss_breached: bool,
    pub seconds_until_zero_for_one_resumes: Option<u64>,
    pub seconds_until_one_for_zero_resumes: Option<u64>,
}
pub fn evaluate_oracle_stop_loss(
    request: &StopLossEvaluationRequest,
) -> Result<StopLossEvaluation, Error> {
    if request.drawdown_e6 > 1_000_000 {
        return Err(Error::InvalidRequest("drawdown exceeds E6 unit"));
    }
    let enabled = request.drawdown_e6 > 0;
    let mut bins = Vec::new();
    for bin in &request.bins {
        if request.skip_empty_bins && bin.total_shares.is_zero() {
            continue;
        }
        let (metric_token0, metric_token1) = if enabled {
            stop_loss_metrics(&StopLossInput {
                bid_price_x64: request.bid_price_x64,
                ask_price_x64: request.ask_price_x64,
                token0_balance_scaled: bin.token0_balance_scaled,
                token1_balance_scaled: bin.token1_balance_scaled,
                total_shares: bin.total_shares,
                minimal_operational_liquidity: request.minimal_operational_liquidity,
            })?
        } else {
            (U256::ZERO, U256::ZERO)
        };
        let breached0 =
            stop_loss_breached(metric_token0, bin.high_watermark0, request.drawdown_e6)?;
        let breached1 =
            stop_loss_breached(metric_token1, bin.high_watermark1, request.drawdown_e6)?;
        bins.push(StopLossBinStatus {
            bin_idx: bin.bin_idx,
            metric_token0,
            metric_token1,
            high_watermark0: bin.high_watermark0,
            high_watermark1: bin.high_watermark1,
            threshold0: if enabled {
                stop_loss_threshold(bin.high_watermark0, request.drawdown_e6)?
            } else {
                U256::ZERO
            },
            threshold1: if enabled {
                stop_loss_threshold(bin.high_watermark1, request.drawdown_e6)?
            } else {
                U256::ZERO
            },
            breached_token0: breached0,
            breached_token1: breached1,
            blocks_zero_for_one: breached0,
            blocks_one_for_zero: breached1,
            seconds_until_zero_for_one_resumes: seconds_until_stop_loss_clears(
                metric_token0,
                bin.high_watermark0,
                request.drawdown_e6,
                request.decay_per_second_e18,
            )?,
            seconds_until_one_for_zero_resumes: seconds_until_stop_loss_clears(
                metric_token1,
                bin.high_watermark1,
                request.drawdown_e6,
                request.decay_per_second_e18,
            )?,
        });
    }
    let breached_bins: Vec<_> = bins
        .iter()
        .filter(|b| b.breached_token0 || b.breached_token1)
        .cloned()
        .collect();
    let resume0 = breached_bins
        .iter()
        .filter(|b| b.breached_token0)
        .try_fold(0, |max, b| {
            b.seconds_until_zero_for_one_resumes.map(|s| max.max(s))
        });
    let resume1 = breached_bins
        .iter()
        .filter(|b| b.breached_token1)
        .try_fold(0, |max, b| {
            b.seconds_until_one_for_zero_resumes.map(|s| max.max(s))
        });
    Ok(StopLossEvaluation {
        enabled,
        bins,
        is_any_bin_stop_loss_breached: !breached_bins.is_empty(),
        breached_bins,
        seconds_until_zero_for_one_resumes: resume0,
        seconds_until_one_for_zero_resumes: resume1,
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLossConfig {
    pub drawdown_e6: u32,
    pub decay_per_second_e18: U256,
    pub timelock: u64,
    pub initialized: bool,
}
fn latest() -> String {
    "latest".into()
}
fn batch_limit() -> u32 {
    512
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StopLossStatusRequest {
    pub pool: crate::Address,
    pub extension: crate::Address,
    pub data_provider: crate::Address,
    pub bid_price_x64: U256,
    pub ask_price_x64: U256,
    #[serde(default = "latest")]
    pub block: String,
    /// Explicit upper bound on the pool range to read; no partial status is returned.
    #[serde(default = "batch_limit")]
    pub max_bins: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLossStatus {
    #[serde(flatten)]
    pub evaluation: StopLossEvaluation,
    pub config: StopLossConfig,
    pub bid_price_x64: U256,
    pub ask_price_x64: U256,
    pub cur_bin_idx: i16,
    pub lowest_bin: i16,
    pub highest_bin: i16,
    pub minimal_operational_liquidity: U256,
    pub block: String,
}
impl<T: aux_rs_core::HttpTransport> crate::MetricChainClient<T> {
    /// Native pool/config/bin/watermark reads anchored to one block, then combined evaluation.
    pub async fn oracle_stop_loss_status(
        &self,
        request: StopLossStatusRequest,
    ) -> Result<StopLossStatus, aux_rs_core::Error<Error>> {
        use aux_rs_core::Error as ClientError;
        use serde_json::{Value, json};
        let local = ClientError::Exchange;
        if request.pool.is_zero()
            || request.extension.is_zero()
            || request.data_provider.is_zero()
            || request.bid_price_x64.is_zero()
            || request.bid_price_x64 >= request.ask_price_x64
            || request.max_bins == 0
            || request.max_bins > 65536
        {
            return Err(local(Error::InvalidRequest(
                "invalid stop-loss targets, bid/ask or read bound",
            )));
        }
        let block = match request.block.as_str() {
            "latest" => format!("0x{:x}", self.block_number().await?),
            "safe" | "finalized" => self
                .block(&request.block)
                .await?
                .and_then(|b| b["number"].as_str().map(str::to_owned))
                .ok_or(local(Error::InvalidRequest(
                    "stop-loss block not available",
                )))?,
            "pending" => {
                return Err(local(Error::InvalidRequest(
                    "stop-loss status requires a committed block",
                )));
            }
            other => other.to_owned(),
        };
        let block_ref = &block;
        let read = |contract: &'static str,
                    function: &'static str,
                    to: crate::Address,
                    args: Value| async move {
            let data = crate::encode_contract_call(contract, function, &args).map_err(local)?;
            let raw = self.eth_call(to, data, None, U256::ZERO, block_ref).await?;
            crate::decode_contract_output(contract, function, &raw).map_err(local)
        };
        let uint = |v: &Value| {
            v.as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| v.to_string())
                .parse::<U256>()
                .map_err(|_| local(Error::InvalidRequest("invalid native stop-loss integer")))
        };
        let index = |v: &Value| {
            v.as_i64()
                .and_then(|n| i16::try_from(n).ok())
                .ok_or(local(Error::InvalidRequest("invalid native stop-loss bin")))
        };
        let imm = read("MetricOmmPool", "getImmutables", request.pool, json!([])).await?;
        let config = read(
            "OracleValueStopLossExtension",
            "oracleStopLossConfig",
            request.extension,
            json!([request.pool]),
        )
        .await?;
        let slot = read(
            "MetricOmmPoolDataProvider",
            "slot0",
            request.data_provider,
            json!([request.pool]),
        )
        .await?;
        let lowest_bin = index(&imm[0][9])?;
        let highest_bin = index(&imm[0][10])?;
        let count = i32::from(highest_bin) - i32::from(lowest_bin) + 1;
        if count <= 0 || count as u32 > request.max_bins {
            return Err(local(Error::InvalidRequest(
                "pool range exceeds the stop-loss read bound",
            )));
        }
        let config = StopLossConfig {
            drawdown_e6: u32::try_from(uint(&config[0])?)
                .map_err(|_| local(Error::InvalidRequest("invalid stop-loss drawdown")))?,
            decay_per_second_e18: uint(&config[1])?,
            timelock: u64::try_from(uint(&config[2])?)
                .map_err(|_| local(Error::InvalidRequest("invalid stop-loss timelock")))?,
            initialized: config[3].as_bool().ok_or(local(Error::InvalidRequest(
                "invalid stop-loss initialization state",
            )))?,
        };
        let bins: Vec<i16> = (i32::from(lowest_bin)..=i32::from(highest_bin))
            .map(|i| i as i16)
            .collect();
        let mut inputs = Vec::new();
        for batch in bins.chunks(512) {
            let packed = read(
                "MetricOmmPoolDataProvider",
                "binStates",
                request.data_provider,
                json!([request.pool, batch]),
            )
            .await?;
            let shares = read(
                "MetricOmmPoolDataProvider",
                "binTotalShares(address,int16[])",
                request.data_provider,
                json!([request.pool, batch]),
            )
            .await?;
            let packed = packed[0]
                .as_array()
                .ok_or(local(Error::InvalidRequest("missing stop-loss bin states")))?;
            let shares = shares[0]
                .as_array()
                .ok_or(local(Error::InvalidRequest("missing stop-loss bin shares")))?;
            if packed.len() != batch.len() || shares.len() != batch.len() {
                return Err(local(Error::InvalidRequest(
                    "stop-loss bin response count mismatch",
                )));
            }
            for ((bin_idx, packed), shares) in batch.iter().zip(packed).zip(shares) {
                let total_shares = uint(shares)?;
                if total_shares.is_zero() {
                    continue;
                }
                let state = crate::math::decode_bin_state(uint(packed)?);
                let hwm = read(
                    "OracleValueStopLossExtension",
                    "currentHighWatermarks",
                    request.extension,
                    json!([request.pool, bin_idx]),
                )
                .await?;
                inputs.push(StopLossBinInput {
                    bin_idx: *bin_idx,
                    token0_balance_scaled: state.token0_balance_scaled,
                    token1_balance_scaled: state.token1_balance_scaled,
                    total_shares,
                    high_watermark0: uint(&hwm[0])?,
                    high_watermark1: uint(&hwm[1])?,
                });
            }
        }
        let minimal_operational_liquidity = uint(&imm[0][7])?;
        let evaluation = evaluate_oracle_stop_loss(&StopLossEvaluationRequest {
            bins: inputs,
            bid_price_x64: request.bid_price_x64,
            ask_price_x64: request.ask_price_x64,
            drawdown_e6: config.drawdown_e6,
            decay_per_second_e18: config.decay_per_second_e18,
            minimal_operational_liquidity,
            skip_empty_bins: true,
        })
        .map_err(local)?;
        Ok(StopLossStatus {
            evaluation,
            config,
            bid_price_x64: request.bid_price_x64,
            ask_price_x64: request.ask_price_x64,
            cur_bin_idx: index(&slot[1])?,
            lowest_bin,
            highest_bin,
            minimal_operational_liquidity,
            block,
        })
    }
}
