//! Metric oracle and liquidity helpers backed by the native core.
use crate::{Result, http};
use serde_json::{Value, json};

pub fn evaluate(operation: &str, params: &Value) -> Result<Value> {
    http::call(44, json!({"operation":operation,"params":params}))
}

pub fn codebook() -> Result<Value> {
    evaluate("codebook", &json!({}))
}

macro_rules! helpers {
    ($($method:ident),*) => {$(
        pub fn $method(params: &Value) -> Result<Value> {
            evaluate(stringify!($method), params)
        }
    )*};
}
helpers!(
    create_uniform_bins,
    prepare_factory_create_pool_params,
    prepare_create_pool,
    prepare_liquidity_addition,
    liquidity_native_payment,
    router_multicall,
    native_payment,
    unwrap_output,
    prepare_external_swap,
    prepare_swap_with_fallback,
    addition_plan,
    weighted_plan,
    removal_plan,
    encode_spread,
    encode_u64x32,
    decode_u64x32,
    pack_oracle_slot,
    oracle_push_calldata,
    feed_id,
    slot_update_digest,
    pusher_consent_digest,
    sign_personal_digest,
    mul_div,
    price_to_q64,
    price_x64_to_e18,
    slippage_limit,
    percentage_removal,
    decayed_high_watermark,
    stop_loss_threshold,
    stop_loss_breached,
    seconds_until_stop_loss_clears,
    stop_loss_metrics,
    evaluate_oracle_stop_loss,
    extension_order,
    direction_bitmap,
    minimum_operational_liquidity
);
