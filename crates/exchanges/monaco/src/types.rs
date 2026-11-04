//! Native Monaco REST schemas, scoped to this adapter; provenance in SIGNING.md.
use serde::{Deserialize, Serialize};
pub type ExtraFields = serde_json::Map<String, serde_json::Value>;
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    #[serde(rename = "token", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub token: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub symbol: Option<String>,
    #[serde(rename = "decimals", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub decimals: Option<i64>,
    #[serde(
        rename = "availableBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub available_balance: Option<String>,
    #[serde(
        rename = "lockedBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub locked_balance: Option<String>,
    #[serde(
        rename = "totalBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_balance: Option<String>,
    #[serde(
        rename = "availableBalanceRaw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub available_balance_raw: Option<String>,
    #[serde(
        rename = "lockedBalanceRaw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub locked_balance_raw: Option<String>,
    #[serde(rename = "assetId", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub asset_id: Option<String>,
    #[serde(
        rename = "totalBalanceRaw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_balance_raw: Option<String>,
    #[serde(
        rename = "wrappedNative",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wrapped_native: Option<bool>,
    #[serde(rename = "version", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub version: Option<u64>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelAllResponse {
    #[serde(
        rename = "totalRequested",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_requested: Option<i64>,
    #[serde(
        rename = "totalCancelled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_cancelled: Option<i64>,
    #[serde(
        rename = "totalFailed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_failed: Option<i64>,
    #[serde(rename = "results", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub results: Option<Vec<BatchCancelResult>>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelError {
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub code: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub message: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BatchCancelOrdersRequest {
    #[serde(rename = "orderIds")]
    pub order_ids: Vec<String>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrdersResponse {
    #[serde(
        rename = "totalRequested",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_requested: Option<i64>,
    #[serde(
        rename = "totalCancelled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_cancelled: Option<i64>,
    #[serde(
        rename = "totalFailed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_failed: Option<i64>,
    #[serde(rename = "results", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub results: Option<Vec<BatchCancelResult>>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelResult {
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub order_id: Option<String>,
    #[serde(
        rename = "cancelledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelled_at: Option<String>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub error: Option<BatchCancelError>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateError {
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub code: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub message: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BatchCreateOrderItem {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub price: Option<String>,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(
        rename = "tradingMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_mode: Option<String>,
    #[serde(
        rename = "slippageToleranceBps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub slippage_tolerance_bps: Option<i64>,
    #[serde(
        rename = "useMasterBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_master_balance: Option<bool>,
    #[serde(
        rename = "expirationDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_date: Option<String>,
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    #[serde(
        rename = "marginAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_account_id: Option<String>,
    #[serde(
        rename = "riskBucketId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub risk_bucket_id: Option<String>,
    #[serde(
        rename = "riskBucketCollateral",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub risk_bucket_collateral: Option<String>,
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub leverage: Option<String>,
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    #[serde(
        rename = "strategyKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_key: Option<String>,
    #[serde(
        rename = "marginMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_mode: Option<String>,
    #[serde(rename = "postOnly", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub post_only: Option<bool>,
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<String>,
    #[serde(
        rename = "idempotencyKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub idempotency_key: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BatchCreateOrdersRequest {
    #[serde(rename = "orders")]
    pub orders: Vec<BatchCreateOrderItem>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateOrdersResponse {
    #[serde(
        rename = "totalRequested",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_requested: Option<i64>,
    #[serde(
        rename = "totalSucceeded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_succeeded: Option<i64>,
    #[serde(
        rename = "totalFailed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_failed: Option<i64>,
    #[serde(rename = "results", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub results: Option<Vec<BatchCreateResult>>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCreateResult {
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub order_id: Option<String>,
    #[serde(
        rename = "matchResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub match_result: Option<MatchResult>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub error: Option<BatchCreateError>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchReplaceError {
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub code: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub message: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BatchReplaceOrderItem {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub price: Option<String>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub quantity: Option<String>,
    #[serde(
        rename = "useMasterBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_master_balance: Option<bool>,
    #[serde(rename = "postOnly", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub post_only: Option<bool>,
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BatchReplaceOrdersRequest {
    #[serde(rename = "orders")]
    pub orders: Vec<BatchReplaceOrderItem>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchReplaceOrdersResponse {
    #[serde(
        rename = "totalRequested",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_requested: Option<i64>,
    #[serde(
        rename = "totalSucceeded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_succeeded: Option<i64>,
    #[serde(
        rename = "totalFailed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_failed: Option<i64>,
    #[serde(rename = "results", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub results: Option<Vec<BatchReplaceResult>>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchReplaceResult {
    #[serde(
        rename = "originalOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_order_id: Option<String>,
    #[serde(
        rename = "newOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub new_order_id: Option<String>,
    #[serde(
        rename = "updatedFields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_fields: Option<UpdatedFields>,
    #[serde(
        rename = "matchResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub match_result: Option<MatchResult>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub error: Option<BatchReplaceError>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CancelOrderRequest {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(
        rename = "remainingQuantityTarget",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remaining_quantity_target: Option<String>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub order_id: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub status: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub message: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChallengeRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub client_id: Option<String>,
    #[serde(rename = "chainId", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub chain_id: Option<String>,
    #[serde(rename = "sessionPublicKey")]
    pub session_public_key: String,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeResponse {
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub nonce: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub message: Option<String>,
    #[serde(rename = "expiresAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub expires_at: Option<i64>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateOrderRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub price: Option<String>,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(
        rename = "tradingMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_mode: Option<String>,
    #[serde(
        rename = "slippageToleranceBps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub slippage_tolerance_bps: Option<i64>,
    #[serde(
        rename = "useMasterBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_master_balance: Option<bool>,
    #[serde(
        rename = "expirationDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_date: Option<String>,
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    #[serde(
        rename = "marginAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_account_id: Option<String>,
    #[serde(
        rename = "riskBucketId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub risk_bucket_id: Option<String>,
    #[serde(
        rename = "riskBucketCollateral",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub risk_bucket_collateral: Option<String>,
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub leverage: Option<String>,
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    #[serde(
        rename = "takeProfit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit: Option<ParentTpSlLeg>,
    #[serde(rename = "stopLoss", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub stop_loss: Option<ParentTpSlLeg>,
    #[serde(
        rename = "strategyKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_key: Option<String>,
    #[serde(
        rename = "marginMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_mode: Option<String>,
    #[serde(rename = "postOnly", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub post_only: Option<bool>,
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<String>,
    #[serde(
        rename = "idempotencyKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub idempotency_key: Option<String>,
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(
        rename = "trailingStop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop: Option<ParentTrailingStopLeg>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResponse {
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub order_id: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub status: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub message: Option<String>,
    #[serde(
        rename = "matchResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub match_result: Option<MatchResult>,
    #[serde(
        rename = "takeProfitOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_id: Option<String>,
    #[serde(
        rename = "stopLossOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_id: Option<String>,
    #[serde(
        rename = "marginAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_account_id: Option<String>,
    #[serde(
        rename = "riskBucketId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub risk_bucket_id: Option<String>,
    #[serde(
        rename = "strategyKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_key: Option<String>,
    #[serde(
        rename = "delegationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delegation_id: Option<String>,
    #[serde(
        rename = "trailingStopOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_order_id: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPriceRange {
    #[serde(rename = "bestPrice", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub best_price: Option<String>,
    #[serde(
        rename = "worstPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub worst_price: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRecord {
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "epoch", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub epoch: Option<String>,
    #[serde(
        rename = "fundingRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_rate: Option<String>,
    #[serde(
        rename = "fundingDeltaPerUnit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_delta_per_unit: Option<String>,
    #[serde(
        rename = "cumulativeFundingPerUnit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cumulative_funding_per_unit: Option<String>,
    #[serde(
        rename = "referencePrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_price: Option<String>,
    #[serde(
        rename = "sampleCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sample_count: Option<u64>,
    #[serde(
        rename = "windowStartedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub window_started_at: Option<String>,
    #[serde(
        rename = "windowClosedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub window_closed_at: Option<String>,
    #[serde(rename = "settledAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub settled_at: Option<String>,
    #[serde(rename = "carried", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub carried: Option<bool>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalancesResponse {
    #[serde(rename = "balances", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub balances: Option<Vec<AccountBalance>>,
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub total: Option<u64>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(
        rename = "totalPages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_pages: Option<u64>,
    #[serde(
        rename = "nextPageToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page_token: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingStateResponse {
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(
        rename = "currentFundingRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_funding_rate: Option<String>,
    #[serde(
        rename = "estimatedNextFundingRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_next_funding_rate: Option<String>,
    #[serde(
        rename = "nextFundingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_funding_time: Option<String>,
    #[serde(
        rename = "fundingIntervalSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_interval_seconds: Option<u64>,
    #[serde(
        rename = "lastFundingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_funding_time: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexPriceResponse {
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(
        rename = "indexPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_price: Option<String>,
    #[serde(
        rename = "components",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub components: Option<Vec<IndexComponent>>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarginAccountSummaryResponse {
    #[serde(
        rename = "marginAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_account_id: Option<String>,
    #[serde(rename = "equity", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub equity: Option<String>,
    #[serde(
        rename = "initialMarginRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_margin_required: Option<String>,
    #[serde(
        rename = "maintenanceMarginRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_margin_required: Option<String>,
    #[serde(
        rename = "freeCollateral",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub free_collateral: Option<String>,
    #[serde(
        rename = "withdrawableCollateral",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawable_collateral: Option<String>,
    #[serde(
        rename = "totalPositionNotional",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_position_notional: Option<String>,
    #[serde(
        rename = "unrealizedPnl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_pnl: Option<String>,
    #[serde(
        rename = "realizedPnl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub realized_pnl: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub updated_at: Option<String>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub label: Option<String>,
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(
        rename = "strategyKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_key: Option<String>,
    #[serde(
        rename = "riskBucketId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub risk_bucket_id: Option<String>,
    #[serde(
        rename = "marginMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_mode: Option<String>,
    #[serde(
        rename = "selectedTradingPairIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub selected_trading_pair_ids: Option<Vec<String>>,
    #[serde(
        rename = "availableOrderCollateral",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub available_order_collateral: Option<String>,
    #[serde(
        rename = "copyFollowId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_follow_id: Option<String>,
    #[serde(
        rename = "copyLeaderHandle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_leader_handle: Option<String>,
    #[serde(
        rename = "riskBucketState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub risk_bucket_state: Option<String>,
    #[serde(
        rename = "subaccountKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subaccount_key: Option<String>,
    #[serde(
        rename = "marginAccountState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_account_state: Option<String>,
    #[serde(
        rename = "maxRemovableCollateral",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_removable_collateral: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarkPriceResponse {
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "markPrice", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub mark_price: Option<String>,
    #[serde(
        rename = "oracleProvider",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub oracle_provider: Option<String>,
    #[serde(
        rename = "oracleEpoch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub oracle_epoch: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub updated_at: Option<String>,
    #[serde(rename = "regime", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub regime: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarketMetadataResponse {
    #[serde(
        rename = "baseIconUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_icon_url: Option<String>,
    #[serde(rename = "high24h", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub high24h: Option<String>,
    #[serde(rename = "lastPrice", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub last_price: Option<String>,
    #[serde(
        rename = "lastPriceTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_price_timestamp: Option<String>,
    #[serde(rename = "low24h", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub low24h: Option<String>,
    #[serde(
        rename = "marketInitializationTimestamp",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub market_initialization_timestamp: Option<String>,
    #[serde(
        rename = "priceChange24h",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_change24h: Option<String>,
    #[serde(
        rename = "priceChangePercent24h",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_change_percent24h: Option<String>,
    #[serde(
        rename = "quoteIconUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_icon_url: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub symbol: Option<String>,
    #[serde(rename = "volume24h", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub volume24h: Option<String>,
    #[serde(
        rename = "minLeverage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_leverage: Option<String>,
    #[serde(
        rename = "maxLeverage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_leverage: Option<String>,
    #[serde(rename = "markPrice", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub mark_price: Option<String>,
    #[serde(
        rename = "indexPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_price: Option<String>,
    #[serde(
        rename = "totalBaseVolumeLtd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_base_volume_ltd: Option<String>,
    #[serde(
        rename = "totalQuoteVolumeLtd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_quote_volume_ltd: Option<String>,
    #[serde(
        rename = "totalTradeCountLtd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_trade_count_ltd: Option<i64>,
    #[serde(
        rename = "quoteVolume24h",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_volume24h: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderResponse {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub id: Option<String>,
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub order_type: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub side: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub price: Option<String>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub quantity: Option<String>,
    #[serde(
        rename = "filledQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_quantity: Option<String>,
    #[serde(
        rename = "averageFillPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub average_fill_price: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub status: Option<String>,
    #[serde(
        rename = "tradingMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_mode: Option<String>,
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub updated_at: Option<String>,
    #[serde(
        rename = "expirationDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration_date: Option<String>,
    #[serde(
        rename = "applicationTakerFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_taker_fee: Option<String>,
    #[serde(
        rename = "monacoTakerFee",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub monaco_taker_fee: Option<String>,
    #[serde(
        rename = "monacoMakerRebate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub monaco_maker_rebate: Option<String>,
    #[serde(
        rename = "totalTakerFees",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_taker_fees: Option<String>,
    #[serde(
        rename = "takerTotalPayment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_total_payment: Option<String>,
    #[serde(
        rename = "makerTotalReceipt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maker_total_receipt: Option<String>,
    #[serde(
        rename = "marginAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_account_id: Option<String>,
    #[serde(
        rename = "positionSide",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_side: Option<String>,
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub leverage: Option<String>,
    #[serde(
        rename = "reduceOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reduce_only: Option<bool>,
    #[serde(
        rename = "positionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_id: Option<String>,
    #[serde(
        rename = "quantityRaw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_raw: Option<String>,
    #[serde(
        rename = "filledQuantityRaw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filled_quantity_raw: Option<String>,
    #[serde(
        rename = "remainingQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remaining_quantity: Option<String>,
    #[serde(
        rename = "remainingQuantityRaw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remaining_quantity_raw: Option<String>,
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<String>,
    #[serde(
        rename = "cancelledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelled_at: Option<String>,
    #[serde(rename = "filledAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub filled_at: Option<String>,
    #[serde(rename = "expiredAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub expired_at: Option<String>,
    #[serde(
        rename = "submittedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub submitted_at: Option<String>,
    #[serde(
        rename = "acknowledgedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub acknowledged_at: Option<String>,
    #[serde(
        rename = "triggerPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_price: Option<String>,
    #[serde(
        rename = "quoteVolume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_volume: Option<String>,
    #[serde(
        rename = "applicationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_id: Option<String>,
    #[serde(
        rename = "parentOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_order_id: Option<String>,
    #[serde(rename = "batchId", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub batch_id: Option<String>,
    #[serde(
        rename = "terminalReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub terminal_reason: Option<String>,
    #[serde(rename = "postOnly", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub post_only: Option<bool>,
    #[serde(rename = "version", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub version: Option<i64>,
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(
        rename = "conditionalOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conditional_order_id: Option<String>,
    #[serde(rename = "origin", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub origin: Option<String>,
    #[serde(
        rename = "placementTakerFeeBps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub placement_taker_fee_bps: Option<String>,
    #[serde(
        rename = "placementMakerFeeBps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub placement_maker_fee_bps: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderbookResponse {
    #[serde(
        rename = "baseDecimals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_decimals: Option<i64>,
    #[serde(rename = "baseToken", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub base_token: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub data: Option<OrderbookData>,
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub event_type: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub symbol: Option<String>,
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(
        rename = "quoteDecimals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_decimals: Option<i64>,
    #[serde(
        rename = "quoteToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_token: Option<String>,
    #[serde(
        rename = "sequenceNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sequence_number: Option<u64>,
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub timestamp: Option<String>,
    #[serde(
        rename = "tradingMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_mode: Option<String>,
    #[serde(rename = "bookSeq", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub book_seq: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradesResponse {
    #[serde(rename = "trades", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub trades: Option<Vec<PublicTrade>>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(
        rename = "nextPageToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page_token: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradingPairResponse {
    #[serde(
        rename = "tradingPair",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair: Option<TradingPairData>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexComponent {
    #[serde(rename = "provider", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub provider: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub price: Option<String>,
    #[serde(rename = "weight", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub weight: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFundingHistoryResponse {
    #[serde(rename = "records", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub records: Option<Vec<FundingRecord>>,
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub total: Option<u64>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(
        rename = "nextPageToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page_token: Option<String>,
    #[serde(
        rename = "totalCapped",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_capped: Option<bool>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMarginAccountsResponse {
    #[serde(rename = "accounts", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub accounts: Option<Vec<MarginAccountSummary>>,
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub total: Option<u64>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(
        rename = "nextPageToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page_token: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListOrdersResponse {
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub orders: Option<Vec<GetOrderResponse>>,
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub total: Option<u64>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(
        rename = "totalPages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_pages: Option<u64>,
    #[serde(
        rename = "nextPageToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page_token: Option<String>,
    #[serde(
        rename = "totalCapped",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_capped: Option<bool>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPositionsResponse {
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub positions: Option<Vec<Position>>,
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub total: Option<u64>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(
        rename = "nextPageToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_page_token: Option<String>,
    #[serde(
        rename = "totalCapped",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_capped: Option<bool>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTradingPairsResponse {
    #[serde(
        rename = "tradingPairs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pairs: Option<Vec<TradingPairData>>,
    #[serde(rename = "page", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page: Option<u64>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub page_size: Option<u64>,
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub total: Option<u64>,
    #[serde(
        rename = "totalPages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_pages: Option<u64>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginAccountSummary {
    #[serde(
        rename = "marginAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_account_id: Option<String>,
    #[serde(rename = "equity", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub equity: Option<String>,
    #[serde(
        rename = "initialMarginRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_margin_required: Option<String>,
    #[serde(
        rename = "maintenanceMarginRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_margin_required: Option<String>,
    #[serde(
        rename = "freeCollateral",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub free_collateral: Option<String>,
    #[serde(
        rename = "withdrawableCollateral",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawable_collateral: Option<String>,
    #[serde(
        rename = "totalPositionNotional",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_position_notional: Option<String>,
    #[serde(
        rename = "unrealizedPnl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_pnl: Option<String>,
    #[serde(
        rename = "realizedPnl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub realized_pnl: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub updated_at: Option<String>,
    #[serde(rename = "label", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub label: Option<String>,
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(
        rename = "strategyKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strategy_key: Option<String>,
    #[serde(
        rename = "riskBucketId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub risk_bucket_id: Option<String>,
    #[serde(
        rename = "marginMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_mode: Option<String>,
    #[serde(
        rename = "selectedTradingPairIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub selected_trading_pair_ids: Option<Vec<String>>,
    #[serde(
        rename = "availableOrderCollateral",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub available_order_collateral: Option<String>,
    #[serde(
        rename = "copyFollowId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_follow_id: Option<String>,
    #[serde(
        rename = "copyLeaderHandle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_leader_handle: Option<String>,
    #[serde(
        rename = "riskBucketState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub risk_bucket_state: Option<String>,
    #[serde(
        rename = "subaccountKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subaccount_key: Option<String>,
    #[serde(
        rename = "marginAccountState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_account_state: Option<String>,
    #[serde(
        rename = "maxRemovableCollateral",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_removable_collateral: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchResult {
    #[serde(
        rename = "tradesCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trades_count: Option<i64>,
    #[serde(
        rename = "totalFilled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_filled: Option<String>,
    #[serde(
        rename = "remainingQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remaining_quantity: Option<String>,
    #[serde(
        rename = "averageFillPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub average_fill_price: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub status: Option<String>,
    #[serde(
        rename = "actualSlippageBps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_slippage_bps: Option<i64>,
    #[serde(
        rename = "maxSlippageBps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_slippage_bps: Option<i64>,
    #[serde(
        rename = "executionPriceRange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_price_range: Option<ExecutionPriceRange>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookData {
    #[serde(rename = "bids", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub bids: Option<Vec<PriceLevel>>,
    #[serde(rename = "asks", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub asks: Option<Vec<PriceLevel>>,
    #[serde(rename = "bestBid", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub best_bid: Option<String>,
    #[serde(rename = "bestAsk", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub best_ask: Option<String>,
    #[serde(rename = "bidVolume", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub bid_volume: Option<String>,
    #[serde(rename = "askVolume", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub ask_volume: Option<String>,
    #[serde(
        rename = "priceChange",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_change: Option<PriceChange>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentTpSlLeg {
    #[serde(
        rename = "triggerPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_price: Option<String>,
    #[serde(rename = "orderType", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub order_type: Option<String>,
    #[serde(
        rename = "limitPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_price: Option<String>,
    #[serde(
        rename = "timeInForce",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_in_force: Option<String>,
    #[serde(
        rename = "slippageToleranceBps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub slippage_tolerance_bps: Option<i64>,
    #[serde(rename = "expiresAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub expires_at: Option<String>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub quantity: Option<String>,
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentTrailingStopLeg {
    #[serde(rename = "trailBps", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub trail_bps: Option<u64>,
    #[serde(
        rename = "activationPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub activation_price: Option<String>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub quantity: Option<String>,
    #[serde(
        rename = "closePosition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub close_position: Option<bool>,
    #[serde(
        rename = "slippageToleranceBps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub slippage_tolerance_bps: Option<i64>,
    #[serde(rename = "expiresAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub expires_at: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde(
        rename = "positionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub position_id: Option<String>,
    #[serde(
        rename = "marginAccountId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_account_id: Option<String>,
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub side: Option<String>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub size: Option<String>,
    #[serde(
        rename = "entryPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub entry_price: Option<String>,
    #[serde(rename = "markPrice", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub mark_price: Option<String>,
    #[serde(
        rename = "indexPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_price: Option<String>,
    #[serde(
        rename = "unrealizedPnl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unrealized_pnl: Option<String>,
    #[serde(
        rename = "realizedPnl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub realized_pnl: Option<String>,
    #[serde(
        rename = "isolatedMargin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub isolated_margin: Option<String>,
    #[serde(rename = "leverage", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub leverage: Option<String>,
    #[serde(
        rename = "maintenanceMarginRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_margin_required: Option<String>,
    #[serde(
        rename = "initialMarginRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_margin_required: Option<String>,
    #[serde(
        rename = "liquidationPrice",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub liquidation_price: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub status: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub updated_at: Option<String>,
    #[serde(
        rename = "riskBucketId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub risk_bucket_id: Option<String>,
    #[serde(
        rename = "marginMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_mode: Option<String>,
    #[serde(
        rename = "netRealizedPnl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub net_realized_pnl: Option<String>,
    #[serde(
        rename = "realizedRoe",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub realized_roe: Option<String>,
    #[serde(rename = "exitPrice", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub exit_price: Option<String>,
    #[serde(
        rename = "fundingPaid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub funding_paid: Option<String>,
    #[serde(rename = "feesPaid", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub fees_paid: Option<String>,
    #[serde(rename = "cumFees", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub cum_fees: Option<String>,
    #[serde(rename = "version", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub version: Option<u64>,
    #[serde(rename = "openedAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub opened_at: Option<String>,
    #[serde(
        rename = "lifetimeTotalsStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lifetime_totals_status: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceChange {
    #[serde(
        rename = "price24hAgo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price24h_ago: Option<String>,
    #[serde(
        rename = "priceChange24h",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_change24h: Option<String>,
    #[serde(
        rename = "priceChangePercent24h",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub price_change_percent24h: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceLevel {
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub price: Option<String>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub quantity: Option<String>,
    #[serde(
        rename = "orderCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_count: Option<u64>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTrade {
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub data: Option<TradeData>,
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub event_type: Option<String>,
    #[serde(
        rename = "tradingPairId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_pair_id: Option<String>,
    #[serde(
        rename = "tradingMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trading_mode: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RefreshRequest {}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResponse {
    #[serde(rename = "expiresAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub expires_at: Option<i64>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplaceOrderRequest {
    #[serde(
        rename = "selfTradePreventionMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub price: Option<String>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub quantity: Option<String>,
    #[serde(
        rename = "useMasterBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_master_balance: Option<bool>,
    #[serde(rename = "postOnly", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub post_only: Option<bool>,
    #[serde(
        rename = "clientOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_order_id: Option<String>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceOrderResponse {
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub order_id: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub status: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub message: Option<String>,
    #[serde(
        rename = "originalOrderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_order_id: Option<String>,
    #[serde(
        rename = "updatedFields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_fields: Option<UpdatedFields>,
    #[serde(
        rename = "matchResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub match_result: Option<MatchResult>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RevokeRequest {}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeResponse {
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub message: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeData {
    #[serde(
        rename = "executedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub executed_at: Option<String>,
    #[serde(rename = "makerSide", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub maker_side: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub price: Option<String>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub quantity: Option<String>,
    #[serde(
        rename = "quantityRaw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_raw: Option<String>,
    #[serde(rename = "tradeId", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub trade_id: Option<String>,
    #[serde(
        rename = "executionSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub execution_source: Option<String>,
    #[serde(
        rename = "makerAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maker_address: Option<String>,
    #[serde(
        rename = "takerAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_address: Option<String>,
    #[serde(
        rename = "takerFeeRaw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_fee_raw: Option<String>,
    #[serde(
        rename = "quoteVolumeRaw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_volume_raw: Option<String>,
    #[serde(
        rename = "takerApplicationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_application_id: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingPairData {
    #[serde(
        rename = "baseAssetId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_asset_id: Option<String>,
    #[serde(
        rename = "baseDecimals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_decimals: Option<i64>,
    #[serde(
        rename = "baseIconUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_icon_url: Option<String>,
    #[serde(rename = "baseToken", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub base_token: Option<String>,
    #[serde(
        rename = "baseTokenContract",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_token_contract: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub id: Option<String>,
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub is_active: Option<bool>,
    #[serde(
        rename = "makerFeeBps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maker_fee_bps: Option<i64>,
    #[serde(
        rename = "marketType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub market_type: Option<String>,
    #[serde(
        rename = "maxOrderSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_order_size: Option<String>,
    #[serde(
        rename = "minOrderSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_order_size: Option<String>,
    #[serde(
        rename = "quoteAssetId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset_id: Option<String>,
    #[serde(
        rename = "quoteDecimals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_decimals: Option<i64>,
    #[serde(
        rename = "quoteIconUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_icon_url: Option<String>,
    #[serde(
        rename = "quoteToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_token: Option<String>,
    #[serde(
        rename = "quoteTokenContract",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_token_contract: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub symbol: Option<String>,
    #[serde(
        rename = "takerFeeBps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub taker_fee_bps: Option<i64>,
    #[serde(rename = "tickSize", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub tick_size: Option<String>,
    #[serde(
        rename = "minLeverage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_leverage: Option<String>,
    #[serde(
        rename = "maxLeverage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_leverage: Option<String>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub category: Option<String>,
    #[serde(
        rename = "quantityStepSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_step_size: Option<String>,
    #[serde(
        rename = "baseAssetName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base_asset_name: Option<String>,
    #[serde(
        rename = "quoteAssetName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_asset_name: Option<String>,
    #[serde(
        rename = "isolatedOnly",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub isolated_only: Option<bool>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedFields {
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub price: Option<String>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub quantity: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub id: Option<String>,
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub address: Option<String>,
    #[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub username: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VerifyRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub client_id: Option<String>,
    #[serde(rename = "chainId", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub chain_id: Option<String>,
    #[serde(rename = "sessionPublicKey")]
    pub session_public_key: String,
    #[serde(
        rename = "referralCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub referral_code: Option<String>,
    #[serde(
        rename = "inviteCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invite_code: Option<String>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyResponse {
    #[serde(rename = "expiresAt", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub expires_at: Option<i64>,
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub user: Option<UserInfo>,
    #[serde(rename = "isNewUser", default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub is_new_user: Option<bool>,
    #[serde(
        rename = "referralApplied",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub referral_applied: Option<bool>,
    #[serde(
        rename = "referralNotAppliedReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub referral_not_applied_reason: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
