use crate::*;
use aux_rs_core::{Error as ClientError, HttpTransport};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreparedCall {
    pub to: Address,
    pub data: Bytes,
    pub value: U256,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RoutePreparation {
    pub route: RouteResponse,
    pub router: Address,
    pub recipient: Address,
    pub deadline: U256,
    pub slippage_bps: u16,
    #[serde(default)]
    pub extension_datas: Option<Vec<Bytes>>,
    #[serde(default = "latest")]
    pub block: String,
}
fn latest() -> String {
    "latest".into()
}
impl<T: HttpTransport> MetricChainClient<T> {
    /// Map a single native route into executable calldata, checking directions against pool state.
    /// Split plans remain available in route(); request maxRoutes=1 for this SDK-compatible mapper.
    pub async fn prepare_route(
        &self,
        request: RoutePreparation,
    ) -> Result<PreparedCall, ClientError<Error>> {
        let local = ClientError::Exchange;
        let r = &request.route;
        if r.routes.len() != 1 {
            return Err(local(Error::InvalidRequest(
                "route mapper requires one route; request maxRoutes=1",
            )));
        }
        let option = &r.routes[0];
        if option.legs.is_empty() || option.legs.len() > 256 || option.trade_type != r.trade_type {
            return Err(local(Error::InvalidRequest(
                "invalid route legs or trade type",
            )));
        }
        let token_in = client::address(&r.token_in).map_err(local)?;
        let token_out = client::address(&r.token_out).map_err(local)?;
        if token_in == token_out
            || token_in.is_zero()
            || token_out.is_zero()
            || request.recipient.is_zero()
        {
            return Err(local(Error::InvalidRequest(
                "nonzero distinct tokens and recipient required",
            )));
        }
        let parse = |s: &str| {
            s.parse::<U256>()
                .map_err(|_| Error::InvalidRequest("invalid route raw amount"))
        };
        let input = parse(&option.amount_in).map_err(local)?;
        let output = parse(&option.amount_out).map_err(local)?;
        let exact_output = r.trade_type == TradeType::ExactOut;
        if input.is_zero()
            || output.is_zero()
            || if exact_output {
                parse(&r.amount_out).map_err(local)? != output
            } else {
                parse(&r.amount_in).map_err(local)? != input
            }
        {
            return Err(local(Error::InvalidRequest(
                "route mapper does not support partial fills",
            )));
        }
        let limit = math::slippage_limit(
            if exact_output { input } else { output },
            request.slippage_bps,
            exact_output,
        )
        .map_err(local)?;
        let exts = request
            .extension_datas
            .unwrap_or_else(|| vec![Bytes::new(); option.legs.len()]);
        if exts.len() != option.legs.len() {
            return Err(local(Error::InvalidRequest(
                "extensionDatas must match route legs",
            )));
        }
        let mut tokens = vec![token_in];
        let mut pools = Vec::new();
        let mut directions = Vec::new();
        for (i, leg) in option.legs.iter().enumerate() {
            let pool = client::address(&leg.pool_address).map_err(local)?;
            let data = encode_contract_call("MetricOmmPool", "getImmutables", &json!([]))
                .map_err(local)?;
            let result = self
                .eth_call(pool, data, None, U256::ZERO, &request.block)
                .await?;
            let output =
                decode_contract_output("MetricOmmPool", "getImmutables", &result).map_err(local)?;
            let token0 = output[0][1]
                .as_str()
                .ok_or(local(Error::InvalidRequest("pool response lacks token0")))?;
            let token1 = output[0][2]
                .as_str()
                .ok_or(local(Error::InvalidRequest("pool response lacks token1")))?;
            let token0 = client::address(token0).map_err(local)?;
            let token1 = client::address(token1).map_err(local)?;
            let current = *tokens.last().expect("initial token");
            let (direction, next) = if current == token0 {
                (true, token1)
            } else if current == token1 {
                (false, token0)
            } else {
                return Err(local(Error::InvalidRequest(
                    "route token is not in native pool",
                )));
            };
            if leg.trade_type != r.trade_type
                || client::address(&leg.token_in).map_err(local)? != current
                || client::address(&leg.token_out).map_err(local)? != next
                || leg.zero_for_one != direction
            {
                return Err(local(Error::InvalidRequest(
                    "route direction does not match native pool",
                )));
            }
            if i > 0
                && parse(&option.legs[i - 1].amount_out).map_err(local)?
                    != parse(&leg.amount_in).map_err(local)?
            {
                return Err(local(Error::InvalidRequest(
                    "route leg amounts do not connect",
                )));
            }
            pools.push(pool);
            directions.push(direction);
            tokens.push(next);
        }
        if *tokens.last().expect("nonempty path") != token_out {
            return Err(local(Error::InvalidRequest("route ends in another token")));
        }
        let mut params = json!({"recipient":request.recipient,"deadline":request.deadline});
        let method = if pools.len() == 1 {
            params["pool"] = json!(pools[0]);
            params["tokenIn"] = json!(token_in);
            params["tokenOut"] = json!(token_out);
            params["zeroForOne"] = json!(directions[0]);
            params["priceLimitX64"] = json!(option.legs[0].price_limit_x64);
            params["extensionData"] = json!(exts[0]);
            if exact_output {
                "exactOutputSingle"
            } else {
                "exactInputSingle"
            }
        } else {
            params["tokens"] = json!(tokens);
            params["pools"] = json!(pools);
            params["extensionDatas"] = json!(exts);
            params["zeroForOneBitMap"] = json!(math::direction_bitmap(&directions).map_err(local)?);
            if exact_output {
                "exactOutput"
            } else {
                "exactInput"
            }
        };
        if exact_output {
            params["amountOut"] = json!(output);
            params["amountInMaximum"] = json!(limit);
        } else {
            params["amountIn"] = json!(input);
            params["amountOutMinimum"] = json!(limit);
        }
        Ok(PreparedCall {
            to: request.router,
            data: encode_contract_call("MetricOmmSimpleRouter", method, &json!({"params":params}))
                .map_err(local)?,
            value: U256::ZERO,
        })
    }
}
pub fn router_multicall(
    router: Address,
    calls: &[Bytes],
    value: U256,
) -> Result<PreparedCall, Error> {
    Ok(PreparedCall {
        to: router,
        data: encode_contract_call("MetricOmmSimpleRouter", "multicall", &json!([calls]))?,
        value,
    })
}
pub(crate) fn swap_input(call: &PreparedCall) -> Result<(&'static str, serde_json::Value), Error> {
    for method in [
        "exactInputSingle",
        "exactOutputSingle",
        "exactInput",
        "exactOutput",
    ] {
        if let Ok(input) = decode_contract_input("MetricOmmSimpleRouter", method, &call.data) {
            return Ok((method, input));
        }
    }
    Err(Error::InvalidRequest(
        "native payment helpers require a native router swap call",
    ))
}
/// Native input follows the router's documented partial native-payment semantics.
pub fn native_payment(
    call: &PreparedCall,
    value: U256,
    wrapped_native: Address,
) -> Result<PreparedCall, Error> {
    let (method, args) = swap_input(call)?;
    let input = if method.ends_with("Single") {
        &args[0][1]
    } else {
        &args[0][0][0]
    };
    if input.as_str().and_then(|s| s.parse::<Address>().ok()) != Some(wrapped_native) {
        return Err(Error::InvalidRequest(
            "swap input is not the configured wrapped native token",
        ));
    }
    let refund = encode_contract_call("MetricOmmSimpleRouter", "refundETH", &json!([]))?;
    router_multicall(call.to, &[call.data.clone(), refund], value)
}
/// Sends wrapped-native swap output to the router and unwraps only after the swap succeeds.
pub fn unwrap_output(
    call: &PreparedCall,
    wrapped_native: Address,
    recipient: Address,
) -> Result<PreparedCall, Error> {
    let (method, mut args) = swap_input(call)?;
    let output = if method.ends_with("Single") {
        &args[0][2]
    } else {
        args[0][0]
            .as_array()
            .and_then(|a| a.last())
            .ok_or(Error::InvalidRequest("empty swap path"))?
    };
    if recipient.is_zero()
        || output.as_str().and_then(|s| s.parse::<Address>().ok()) != Some(wrapped_native)
    {
        return Err(Error::InvalidRequest(
            "swap output must match wrapped native and recipient must be nonzero",
        ));
    }
    let minimum = args[0][if method.starts_with("exactOutput") {
        4
    } else {
        5
    }]
    .clone();
    args[0][6] = json!(call.to);
    let swap = encode_contract_call("MetricOmmSimpleRouter", method, &args)?;
    let unwrap = encode_contract_call(
        "MetricOmmSimpleRouter",
        "unwrapWETH9",
        &json!([minimum, recipient]),
    )?;
    router_multicall(call.to, &[swap, unwrap], call.value)
}
