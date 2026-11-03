//! Pool-creation parameter preparation and explicit unsigned call builders.
use crate::{
    Address, B256, Bytes, Error, PreparedCall, U256, encode_contract_call, liquidity, math,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub const DISABLED_BID_ASK_REF_ASYMMETRY_X8: u16 = u16::MAX;
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExtensionOrders {
    pub before_add_liquidity: U256,
    pub after_add_liquidity: U256,
    pub before_remove_liquidity: U256,
    pub after_remove_liquidity: U256,
    pub before_swap: U256,
    pub after_swap: U256,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreatePoolParams {
    pub token0: Address,
    pub token1: Address,
    pub price_provider: Address,
    pub admin: Address,
    #[serde(default)]
    pub extensions: Vec<Address>,
    #[serde(default)]
    pub extension_orders: ExtensionOrders,
    #[serde(default)]
    pub extension_init_data: Vec<Bytes>,
    #[serde(default)]
    pub price_provider_timelock: U256,
    pub non_negative_bins: Vec<U256>,
    pub negative_bins: Vec<U256>,
    pub initial_scaled_amount0_per_e18_shares: U256,
    pub initial_scaled_amount1_per_e18_shares: U256,
    pub minimal_operational_liquidity: U256,
    pub protocol_spread_fee_e6: u32,
    pub admin_spread_fee_e6: u32,
    pub protocol_notional_fee_e8: u32,
    pub admin_notional_fee_e8: u32,
    pub admin_fee_destination: Address,
    /// Exact percentage string, e.g. "-0.0125". Excess precision is rejected.
    pub cur_bin_dist_from_provided_price_percent: String,
    pub bid_ask_ref_asymmetry_x8: u16,
    /// An omitted salt is generated once using OS entropy and returned in the prepared result.
    #[serde(default)]
    pub salt: Option<B256>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FactoryPoolParameters {
    pub token0: Address,
    pub token1: Address,
    pub price_provider: Address,
    pub extensions: Vec<Address>,
    pub extension_orders: ExtensionOrders,
    pub extension_init_data: Vec<Bytes>,
    pub price_provider_timelock: U256,
    pub admin: Address,
    pub initial_scaled_amount0_per_e18_shares: U256,
    pub initial_scaled_amount1_per_e18_shares: U256,
    pub minimal_operational_liquidity: U256,
    pub protocol_spread_fee_e6: u32,
    pub admin_spread_fee_e6: u32,
    pub protocol_notional_fee_e8: u32,
    pub admin_notional_fee_e8: u32,
    pub admin_fee_destination: Address,
    pub cur_bin_dist_from_provided_price_e6: i32,
    pub non_negative_bin_data_array: Vec<U256>,
    pub negative_bin_data_array: Vec<U256>,
    pub salt: B256,
    pub bid_ask_ref_asymmetry_x8: u16,
}
fn distance(percent: &str) -> Result<i32, Error> {
    let (negative, magnitude) = percent
        .strip_prefix('-')
        .map_or((false, percent), |s| (true, s));
    let n = math::decimal_to_raw(magnitude, 4)?;
    let limit = if negative {
        1u32 << 23
    } else {
        (1u32 << 23) - 1
    };
    if n > U256::from(limit) {
        return Err(Error::InvalidRequest("pool price distance exceeds int24"));
    }
    let n = n.to::<i32>();
    Ok(if negative { -n } else { n })
}
pub fn prepare_factory_create_pool_params(
    input: &CreatePoolParams,
) -> Result<FactoryPoolParameters, Error> {
    if input.bid_ask_ref_asymmetry_x8 == 0 {
        return Err(Error::InvalidRequest(
            "bidAskRefAsymmetryX8 must be positive; use 65535 to disable",
        ));
    }
    let minimum = liquidity::minimum_operational_liquidity(
        input.initial_scaled_amount0_per_e18_shares,
        input.initial_scaled_amount1_per_e18_shares,
    )?;
    if input.minimal_operational_liquidity < minimum {
        return Err(Error::InvalidRequest(
            "minimalOperationalLiquidity is below the factory minimum",
        ));
    }
    if input.extensions.len() > 7 || input.extensions.len() != input.extension_init_data.len() {
        return Err(Error::InvalidRequest(
            "at most seven extensions with matching init data required",
        ));
    }
    let mut unique = std::collections::HashSet::new();
    if input
        .extensions
        .iter()
        .any(|a| a.is_zero() || !unique.insert(*a))
    {
        return Err(Error::InvalidRequest(
            "extensions must be nonzero and unique",
        ));
    }
    for order in [
        input.extension_orders.before_add_liquidity,
        input.extension_orders.after_add_liquidity,
        input.extension_orders.before_remove_liquidity,
        input.extension_orders.after_remove_liquidity,
        input.extension_orders.before_swap,
        input.extension_orders.after_swap,
    ] {
        if order.bit_len() > 21
            || (0usize..7).any(|i| {
                ((order >> (i * 3)) & U256::from(7)).to::<usize>() > input.extensions.len()
            })
        {
            return Err(Error::InvalidRequest(
                "extension orders reference an unconfigured slot",
            ));
        }
    }
    if [
        input.protocol_spread_fee_e6,
        input.admin_spread_fee_e6,
        input.protocol_notional_fee_e8,
        input.admin_notional_fee_e8,
    ]
    .iter()
    .any(|fee| *fee >= (1 << 24))
    {
        return Err(Error::InvalidRequest("pool fees exceed uint24"));
    }
    let distance = distance(&input.cur_bin_dist_from_provided_price_percent)?;
    let salt = match input.salt {
        Some(salt) => salt,
        None => {
            let mut bytes = [0; 32];
            getrandom::getrandom(&mut bytes)
                .map_err(|_| Error::InvalidRequest("OS entropy unavailable for pool salt"))?;
            B256::from(bytes)
        }
    };
    Ok(FactoryPoolParameters {
        token0: input.token0,
        token1: input.token1,
        price_provider: input.price_provider,
        extensions: input.extensions.clone(),
        extension_orders: input.extension_orders.clone(),
        extension_init_data: input.extension_init_data.clone(),
        price_provider_timelock: input.price_provider_timelock,
        admin: input.admin,
        initial_scaled_amount0_per_e18_shares: input.initial_scaled_amount0_per_e18_shares,
        initial_scaled_amount1_per_e18_shares: input.initial_scaled_amount1_per_e18_shares,
        minimal_operational_liquidity: input.minimal_operational_liquidity,
        protocol_spread_fee_e6: input.protocol_spread_fee_e6,
        admin_spread_fee_e6: input.admin_spread_fee_e6,
        protocol_notional_fee_e8: input.protocol_notional_fee_e8,
        admin_notional_fee_e8: input.admin_notional_fee_e8,
        admin_fee_destination: input.admin_fee_destination,
        cur_bin_dist_from_provided_price_e6: distance,
        non_negative_bin_data_array: input.non_negative_bins.clone(),
        negative_bin_data_array: input.negative_bins.clone(),
        salt,
        bid_ask_ref_asymmetry_x8: input.bid_ask_ref_asymmetry_x8,
    })
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PoolCreationRequest {
    /// Factory address, or ImmutableAppPoolHelper address when an override is supplied.
    pub target: Address,
    pub params: CreatePoolParams,
    #[serde(default)]
    pub helper_asymmetry_override: Option<u16>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreparedPoolCreation {
    pub params: FactoryPoolParameters,
    pub call: PreparedCall,
}
pub fn prepare_create_pool(request: &PoolCreationRequest) -> Result<PreparedPoolCreation, Error> {
    if request.target.is_zero() || request.helper_asymmetry_override == Some(0) {
        return Err(Error::InvalidRequest(
            "nonzero target and positive helper asymmetry override required",
        ));
    }
    let params = prepare_factory_create_pool_params(&request.params)?;
    let (contract, args) = match request.helper_asymmetry_override {
        Some(asymmetry) => ("ImmutableAppPoolHelper", json!([params, asymmetry])),
        None => ("MetricOmmPoolFactory", json!([params])),
    };
    let data = encode_contract_call(contract, "createPool", &args)?;
    Ok(PreparedPoolCreation {
        params,
        call: PreparedCall {
            to: request.target,
            data,
            value: U256::ZERO,
        },
    })
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UniformBinsRequest {
    pub count: u32,
    pub length_e6: u16,
    #[serde(default)]
    pub add_fee_buy_e6: u16,
    #[serde(default)]
    pub add_fee_sell_e6: u16,
}
pub fn create_uniform_bins(request: &UniformBinsRequest) -> Result<Vec<U256>, Error> {
    if request.count > 32768 || request.length_e6 == 0 {
        return Err(Error::InvalidRequest(
            "uniform bins require positive length and at most 32768 bins",
        ));
    }
    let bins = vec![
        math::BinData {
            length_e6: request.length_e6,
            add_fee_buy_e6: request.add_fee_buy_e6,
            add_fee_sell_e6: request.add_fee_sell_e6
        };
        request.count as usize
    ];
    Ok(math::pack_bin_data_array(&bins))
}
