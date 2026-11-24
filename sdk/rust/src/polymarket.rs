//! Market resolution and unsigned wallet authorizations using the native core.
use crate::{Result, http};
use serde_json::{Value, json};

pub fn evaluate<T: serde::de::DeserializeOwned>(operation: &str, params: &Value) -> Result<T> {
    http::call(46, json!({"operation":operation,"params":params}))
}

pub fn market_asset_id(market: &Value, outcome: &str) -> Result<String> {
    evaluate(
        "market_asset_id",
        &json!({"market": market, "outcome": outcome}),
    )
}

pub fn market_order_domain(market: &Value) -> Result<Value> {
    evaluate("market_order_domain", &json!({"market": market}))
}

pub fn typed_data_hash(payload: &Value) -> Result<String> {
    evaluate("typed_data_hash", &json!({"payload":payload}))
}
pub fn order_typed_data(order: &Value, domain: &Value) -> Result<Value> {
    evaluate("order_typed_data", &json!({"order":order,"domain":domain}))
}
pub fn deployments() -> Result<Value> {
    evaluate("deployments", &json!({}))
}

pub fn wallet_auth_typed_data(
    chain_id: u64,
    address: &str,
    timestamp: u64,
    nonce: u64,
) -> Result<Value> {
    evaluate(
        "wallet_auth_typed_data",
        &json!({"chain_id": chain_id, "address": address, "timestamp": timestamp, "nonce": nonce}),
    )
}

pub fn perps_op_typed_data(
    chain_id: u64,
    operation: &Value,
    salt: u64,
    timestamp: u64,
) -> Result<Value> {
    evaluate(
        "perps_op_typed_data",
        &json!({"chain_id": chain_id, "operation": operation, "salt": salt, "timestamp": timestamp}),
    )
}

pub fn deposit_wallet_batch_typed_data(
    chain_id: u64,
    wallet: &str,
    nonce: &str,
    deadline: &str,
    calls: &Value,
) -> Result<Value> {
    evaluate(
        "deposit_wallet_batch_typed_data",
        &json!({"chain_id": chain_id, "wallet": wallet, "nonce": nonce, "deadline": deadline, "calls": calls}),
    )
}

pub fn wrap_session_signature(session_signer: &str, signature: &str) -> Result<String> {
    evaluate(
        "wrap_session_signature",
        &json!({"session_signer": session_signer, "signature": signature}),
    )
}
