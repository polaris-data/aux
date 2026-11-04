//! Unsigned liquidity and external-fallback workflows. No network calls or submissions.
use crate::{
    Address, Bytes, Error, PreparedCall, U256, decode_contract_input, encode_contract_call,
    liquidity::{CursorBounds, LiquidityDeltas},
    native_payment, router_multicall, unwrap_output,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LiquidityAdditionKind {
    #[default]
    ExactShares,
    Weighted,
}
fn unconstrained_cursor() -> CursorBounds {
    CursorBounds {
        minimal_cur_bin: i16::MIN,
        minimal_position: U256::ZERO,
        maximal_cur_bin: i16::MAX,
        maximal_position: (U256::from(1) << 104usize) - U256::from(1),
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LiquidityAdditionRequest {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub liquidity_adder: Address,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub pool: Address,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(type = "(string) | null", optional = nullable))]
    pub owner: Option<Address>,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub salt: U256,
    pub deltas: LiquidityDeltas,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub max_amount_token0: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub max_amount_token1: U256,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub bin_position_bounds: Option<CursorBounds>,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub extension_data: Bytes,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub kind: LiquidityAdditionKind,
    /// Some(value) selects native payment and appends refundETH, including for zero contribution.
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(type = "(string) | null", optional = nullable))]
    pub native_value: Option<U256>,
}
fn liquidity_signature(kind: LiquidityAdditionKind, owner: bool) -> &'static str {
    match (kind, owner) {
        (LiquidityAdditionKind::ExactShares, false) => {
            "addLiquidityExactShares(address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)"
        }
        (LiquidityAdditionKind::ExactShares, true) => {
            "addLiquidityExactShares(address,address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)"
        }
        (LiquidityAdditionKind::Weighted, false) => {
            "addLiquidityWeighted(address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)"
        }
        (LiquidityAdditionKind::Weighted, true) => {
            "addLiquidityWeighted(address,address,uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)"
        }
    }
}
pub fn prepare_liquidity_addition(
    request: &LiquidityAdditionRequest,
) -> Result<PreparedCall, Error> {
    request.deltas.validate()?;
    let bounds = match &request.bin_position_bounds {
        Some(bounds) => bounds.clone(),
        None if matches!(request.kind, LiquidityAdditionKind::Weighted) => {
            return Err(Error::InvalidRequest(
                "weighted addition requires explicit cursor bounds",
            ));
        }
        None => unconstrained_cursor(),
    };
    let max104 = (U256::from(1) << 104usize) - U256::from(1);
    if request.liquidity_adder.is_zero()
        || request.pool.is_zero()
        || request.owner.is_some_and(|a| a.is_zero())
        || request.salt.bit_len() > 80
        || bounds.minimal_position > max104
        || bounds.maximal_position > max104
        || (bounds.minimal_cur_bin, bounds.minimal_position)
            > (bounds.maximal_cur_bin, bounds.maximal_position)
    {
        return Err(Error::InvalidRequest(
            "invalid liquidity target, owner, salt or cursor bounds",
        ));
    }
    let mut args = vec![json!(request.pool)];
    if let Some(owner) = request.owner {
        args.push(json!(owner));
    }
    args.extend([
        json!(request.salt),
        json!(request.deltas),
        json!(request.max_amount_token0),
        json!(request.max_amount_token1),
        json!(bounds),
        json!(request.extension_data),
    ]);
    let call = PreparedCall {
        to: request.liquidity_adder,
        data: encode_contract_call(
            "MetricOmmPoolLiquidityAdder",
            liquidity_signature(request.kind, request.owner.is_some()),
            &json!(args),
        )?,
        value: U256::ZERO,
    };
    match request.native_value {
        Some(value) => liquidity_native_payment(&call, value),
        None => Ok(call),
    }
}
/// Adds a refund to one canonical adder call. Wrapped-native leg and approvals remain native prerequisites.
pub fn liquidity_native_payment(call: &PreparedCall, value: U256) -> Result<PreparedCall, Error> {
    let mut valid = false;
    for kind in [
        LiquidityAdditionKind::ExactShares,
        LiquidityAdditionKind::Weighted,
    ] {
        for owner in [false, true] {
            valid |= decode_contract_input(
                "MetricOmmPoolLiquidityAdder",
                liquidity_signature(kind, owner),
                &call.data,
            )
            .is_ok();
        }
    }
    if !valid || call.to.is_zero() {
        return Err(Error::InvalidRequest(
            "native liquidity payment requires a canonical liquidity-adder call",
        ));
    }
    let refund = encode_contract_call("MetricOmmPoolLiquidityAdder", "refundETH", &json!([]))?;
    Ok(PreparedCall {
        to: call.to,
        data: encode_contract_call(
            "MetricOmmPoolLiquidityAdder",
            "multicall",
            &json!([[call.data.clone(), refund]]),
        )?,
        value,
    })
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum SwapPayment {
    #[default]
    Wrapped,
    NativeInput {
        #[serde(rename = "wrappedNative")]
        #[cfg_attr(feature = "typescript", ts(type = "string"))]
        wrapped_native: Address,
    },
    UnwrapOutput {
        #[serde(rename = "wrappedNative")]
        #[cfg_attr(feature = "typescript", ts(type = "string"))]
        wrapped_native: Address,
    },
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExternalSwapParams {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub token_in: Address,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub token_out: Address,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub amount_in_maximum: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub amount_out_minimum: U256,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub external_router: Address,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub external_router_calldata: Bytes,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub recipient: Address,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub deadline: U256,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExternalSwapRequest {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub router: Address,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub executor: Address,
    pub params: ExternalSwapParams,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub payment: SwapPayment,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub value: U256,
}
pub fn prepare_external_swap(request: &ExternalSwapRequest) -> Result<PreparedCall, Error> {
    let mut params = request.params.clone();
    if request.router.is_zero()
        || request.executor.is_zero()
        || params.external_router.is_zero()
        || params.token_in.is_zero()
        || params.token_out.is_zero()
        || params.token_in == params.token_out
        || params.recipient.is_zero()
    {
        return Err(Error::InvalidRequest(
            "external swap requires nonzero targets, distinct tokens and recipient",
        ));
    }
    let mut refund_native = false;
    let mut unwrap = None;
    match request.payment {
        SwapPayment::Wrapped => {
            if !request.value.is_zero() {
                return Err(Error::InvalidRequest(
                    "wrapped swap cannot carry native value",
                ));
            }
        }
        SwapPayment::NativeInput { wrapped_native } => {
            if wrapped_native.is_zero() || params.token_in != wrapped_native {
                return Err(Error::InvalidRequest(
                    "external swap input is not wrapped native",
                ));
            }
            refund_native = true;
        }
        SwapPayment::UnwrapOutput { wrapped_native } => {
            if wrapped_native.is_zero()
                || params.token_out != wrapped_native
                || !request.value.is_zero()
            {
                return Err(Error::InvalidRequest(
                    "external swap output is not wrapped native or carries native input",
                ));
            }
            unwrap = Some(params.recipient);
            params.recipient = request.router;
        }
    }
    let external = encode_contract_call(
        "MetricOmmSimpleRouter",
        "externalSwap",
        &json!([request.executor, false, refund_native, params]),
    )?;
    if refund_native {
        let refund = encode_contract_call("MetricOmmSimpleRouter", "refundETH", &json!([]))?;
        router_multicall(request.router, &[external, refund], request.value)
    } else if let Some(recipient) = unwrap {
        let unwrap = encode_contract_call(
            "MetricOmmSimpleRouter",
            "unwrapWETH9",
            &json!([params.amount_out_minimum, recipient]),
        )?;
        router_multicall(request.router, &[external, unwrap], request.value)
    } else {
        Ok(PreparedCall {
            to: request.router,
            data: external,
            value: request.value,
        })
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExternalSwapFallback {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub external_router: Address,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub external_router_calldata: Bytes,
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub executor: Address,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FallbackSwapRequest {
    /// Original, unwrapped exactInput/Output[Single] call. value is the native contribution.
    pub call: PreparedCall,
    pub fallback: ExternalSwapFallback,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub payment: SwapPayment,
}
pub fn prepare_swap_with_fallback(request: &FallbackSwapRequest) -> Result<PreparedCall, Error> {
    let call = &request.call;
    let (method, args) = crate::swap::swap_input(call)?;
    let params = &args[0];
    let address = |v: &serde_json::Value| {
        v.as_str()
            .and_then(|s| s.parse::<Address>().ok())
            .ok_or(Error::InvalidRequest("invalid decoded swap address"))
    };
    let uint = |v: &serde_json::Value| {
        v.as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| v.to_string())
            .parse::<U256>()
            .map_err(|_| Error::InvalidRequest("invalid decoded swap amount"))
    };
    let single = method.ends_with("Single");
    let (token_in, token_out) = if single {
        (address(&params[1])?, address(&params[2])?)
    } else {
        let tokens = params[0]
            .as_array()
            .ok_or(Error::InvalidRequest("swap path required"))?;
        let pools = params[1]
            .as_array()
            .ok_or(Error::InvalidRequest("swap pools required"))?;
        if tokens.len() < 2
            || tokens.len() != pools.len() + 1
            || params[2].as_array().map(Vec::len) != Some(pools.len())
        {
            return Err(Error::InvalidRequest(
                "swap path, pools and extensions do not match",
            ));
        }
        (
            address(&tokens[0])?,
            address(tokens.last().expect("nonempty path"))?,
        )
    };
    let exact_output = method.starts_with("exactOutput");
    let recipient = address(&params[6])?;
    let external = prepare_external_swap(&ExternalSwapRequest {
        router: call.to,
        executor: request.fallback.executor,
        params: ExternalSwapParams {
            token_in,
            token_out,
            amount_in_maximum: uint(&params[if exact_output { 5 } else { 4 }])?,
            amount_out_minimum: uint(&params[if exact_output { 4 } else { 5 }])?,
            external_router: request.fallback.external_router,
            external_router_calldata: request.fallback.external_router_calldata.clone(),
            recipient,
            deadline: uint(&params[7])?,
        },
        payment: request.payment.clone(),
        value: call.value,
    })?;
    let primary = match request.payment {
        SwapPayment::Wrapped => {
            router_multicall(call.to, std::slice::from_ref(&call.data), call.value)?
        }
        SwapPayment::NativeInput { wrapped_native } => {
            native_payment(call, call.value, wrapped_native)?
        }
        SwapPayment::UnwrapOutput { wrapped_native } => {
            unwrap_output(call, wrapped_native, recipient)?
        }
    };
    // Multicall provides atomic rollback of each branch before CONTINUE runs the fallback.
    let fallback = match request.payment {
        SwapPayment::Wrapped => router_multicall(call.to, &[external.data], call.value)?,
        _ => external,
    };
    let steps = json!([[primary.data, 1, 0], [fallback.data, 1, 2]]);
    Ok(PreparedCall {
        to: call.to,
        data: encode_contract_call("MetricOmmSimpleRouter", "sequence", &json!([steps]))?,
        value: call.value,
    })
}
