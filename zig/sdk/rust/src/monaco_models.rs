// Generated from src/exchanges/monaco/schema/monaco.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{MonacoClient, Error, Result};


#[derive(Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "decimals", skip_serializing_if = "Option::is_none")]
    pub decimals: Option<i64>,
    #[serde(rename = "availableBalance", skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<String>,
    #[serde(rename = "lockedBalance", skip_serializing_if = "Option::is_none")]
    pub locked_balance: Option<String>,
    #[serde(rename = "totalBalance", skip_serializing_if = "Option::is_none")]
    pub total_balance: Option<String>,
    #[serde(rename = "availableBalanceRaw", skip_serializing_if = "Option::is_none")]
    pub available_balance_raw: Option<String>,
    #[serde(rename = "lockedBalanceRaw", skip_serializing_if = "Option::is_none")]
    pub locked_balance_raw: Option<String>,
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "totalBalanceRaw", skip_serializing_if = "Option::is_none")]
    pub total_balance_raw: Option<String>,
    #[serde(rename = "wrappedNative", skip_serializing_if = "Option::is_none")]
    pub wrapped_native: Option<bool>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchCancelAllResponse {
    #[serde(rename = "totalRequested", skip_serializing_if = "Option::is_none")]
    pub total_requested: Option<i64>,
    #[serde(rename = "totalCancelled", skip_serializing_if = "Option::is_none")]
    pub total_cancelled: Option<i64>,
    #[serde(rename = "totalFailed", skip_serializing_if = "Option::is_none")]
    pub total_failed: Option<i64>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<BatchCancelResult>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchCancelError {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchCancelOrdersRequest {
    #[serde(rename = "orderIds")]
    pub order_ids: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchCancelOrdersResponse {
    #[serde(rename = "totalRequested", skip_serializing_if = "Option::is_none")]
    pub total_requested: Option<i64>,
    #[serde(rename = "totalCancelled", skip_serializing_if = "Option::is_none")]
    pub total_cancelled: Option<i64>,
    #[serde(rename = "totalFailed", skip_serializing_if = "Option::is_none")]
    pub total_failed: Option<i64>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<BatchCancelResult>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchCancelResult {
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "cancelledAt", skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<BatchCancelError>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchCreateError {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchCreateOrderItem {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "tradingMode", skip_serializing_if = "Option::is_none")]
    pub trading_mode: Option<String>,
    #[serde(rename = "slippageToleranceBps", skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance_bps: Option<i64>,
    #[serde(rename = "useMasterBalance", skip_serializing_if = "Option::is_none")]
    pub use_master_balance: Option<bool>,
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "marginAccountId", skip_serializing_if = "Option::is_none")]
    pub margin_account_id: Option<String>,
    #[serde(rename = "riskBucketId", skip_serializing_if = "Option::is_none")]
    pub risk_bucket_id: Option<String>,
    #[serde(rename = "riskBucketCollateral", skip_serializing_if = "Option::is_none")]
    pub risk_bucket_collateral: Option<String>,
    #[serde(rename = "positionSide", skip_serializing_if = "Option::is_none")]
    pub position_side: Option<String>,
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "strategyKey", skip_serializing_if = "Option::is_none")]
    pub strategy_key: Option<String>,
    #[serde(rename = "marginMode", skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "idempotencyKey", skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(rename = "selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchCreateOrdersRequest {
    #[serde(rename = "orders")]
    pub orders: Vec<BatchCreateOrderItem>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchCreateOrdersResponse {
    #[serde(rename = "totalRequested", skip_serializing_if = "Option::is_none")]
    pub total_requested: Option<i64>,
    #[serde(rename = "totalSucceeded", skip_serializing_if = "Option::is_none")]
    pub total_succeeded: Option<i64>,
    #[serde(rename = "totalFailed", skip_serializing_if = "Option::is_none")]
    pub total_failed: Option<i64>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<BatchCreateResult>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchCreateResult {
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "matchResult", skip_serializing_if = "Option::is_none")]
    pub match_result: Option<MatchResult>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<BatchCreateError>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchReplaceError {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchReplaceOrderItem {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "useMasterBalance", skip_serializing_if = "Option::is_none")]
    pub use_master_balance: Option<bool>,
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchReplaceOrdersRequest {
    #[serde(rename = "orders")]
    pub orders: Vec<BatchReplaceOrderItem>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchReplaceOrdersResponse {
    #[serde(rename = "totalRequested", skip_serializing_if = "Option::is_none")]
    pub total_requested: Option<i64>,
    #[serde(rename = "totalSucceeded", skip_serializing_if = "Option::is_none")]
    pub total_succeeded: Option<i64>,
    #[serde(rename = "totalFailed", skip_serializing_if = "Option::is_none")]
    pub total_failed: Option<i64>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<BatchReplaceResult>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchReplaceResult {
    #[serde(rename = "originalOrderId", skip_serializing_if = "Option::is_none")]
    pub original_order_id: Option<String>,
    #[serde(rename = "newOrderId", skip_serializing_if = "Option::is_none")]
    pub new_order_id: Option<String>,
    #[serde(rename = "updatedFields", skip_serializing_if = "Option::is_none")]
    pub updated_fields: Option<UpdatedFields>,
    #[serde(rename = "matchResult", skip_serializing_if = "Option::is_none")]
    pub match_result: Option<MatchResult>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<BatchReplaceError>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelOrderRequest {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "remainingQuantityTarget", skip_serializing_if = "Option::is_none")]
    pub remaining_quantity_target: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChallengeRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "chainId", skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(rename = "sessionPublicKey")]
    pub session_public_key: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ChallengeResponse {
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateOrderRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "tradingMode", skip_serializing_if = "Option::is_none")]
    pub trading_mode: Option<String>,
    #[serde(rename = "slippageToleranceBps", skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance_bps: Option<i64>,
    #[serde(rename = "useMasterBalance", skip_serializing_if = "Option::is_none")]
    pub use_master_balance: Option<bool>,
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "marginAccountId", skip_serializing_if = "Option::is_none")]
    pub margin_account_id: Option<String>,
    #[serde(rename = "riskBucketId", skip_serializing_if = "Option::is_none")]
    pub risk_bucket_id: Option<String>,
    #[serde(rename = "riskBucketCollateral", skip_serializing_if = "Option::is_none")]
    pub risk_bucket_collateral: Option<String>,
    #[serde(rename = "positionSide", skip_serializing_if = "Option::is_none")]
    pub position_side: Option<String>,
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "takeProfit", skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<ParentTpSlLeg>,
    #[serde(rename = "stopLoss", skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<ParentTpSlLeg>,
    #[serde(rename = "strategyKey", skip_serializing_if = "Option::is_none")]
    pub strategy_key: Option<String>,
    #[serde(rename = "marginMode", skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "idempotencyKey", skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(rename = "selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(rename = "trailingStop", skip_serializing_if = "Option::is_none")]
    pub trailing_stop: Option<ParentTrailingStopLeg>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "matchResult", skip_serializing_if = "Option::is_none")]
    pub match_result: Option<MatchResult>,
    #[serde(rename = "takeProfitOrderId", skip_serializing_if = "Option::is_none")]
    pub take_profit_order_id: Option<String>,
    #[serde(rename = "stopLossOrderId", skip_serializing_if = "Option::is_none")]
    pub stop_loss_order_id: Option<String>,
    #[serde(rename = "marginAccountId", skip_serializing_if = "Option::is_none")]
    pub margin_account_id: Option<String>,
    #[serde(rename = "riskBucketId", skip_serializing_if = "Option::is_none")]
    pub risk_bucket_id: Option<String>,
    #[serde(rename = "strategyKey", skip_serializing_if = "Option::is_none")]
    pub strategy_key: Option<String>,
    #[serde(rename = "delegationId", skip_serializing_if = "Option::is_none")]
    pub delegation_id: Option<String>,
    #[serde(rename = "trailingStopOrderId", skip_serializing_if = "Option::is_none")]
    pub trailing_stop_order_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExecutionPriceRange {
    #[serde(rename = "bestPrice", skip_serializing_if = "Option::is_none")]
    pub best_price: Option<String>,
    #[serde(rename = "worstPrice", skip_serializing_if = "Option::is_none")]
    pub worst_price: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FundingRecord {
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "epoch", skip_serializing_if = "Option::is_none")]
    pub epoch: Option<String>,
    #[serde(rename = "fundingRate", skip_serializing_if = "Option::is_none")]
    pub funding_rate: Option<String>,
    #[serde(rename = "fundingDeltaPerUnit", skip_serializing_if = "Option::is_none")]
    pub funding_delta_per_unit: Option<String>,
    #[serde(rename = "cumulativeFundingPerUnit", skip_serializing_if = "Option::is_none")]
    pub cumulative_funding_per_unit: Option<String>,
    #[serde(rename = "referencePrice", skip_serializing_if = "Option::is_none")]
    pub reference_price: Option<String>,
    #[serde(rename = "sampleCount", skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<u64>,
    #[serde(rename = "windowStartedAt", skip_serializing_if = "Option::is_none")]
    pub window_started_at: Option<String>,
    #[serde(rename = "windowClosedAt", skip_serializing_if = "Option::is_none")]
    pub window_closed_at: Option<String>,
    #[serde(rename = "settledAt", skip_serializing_if = "Option::is_none")]
    pub settled_at: Option<String>,
    #[serde(rename = "carried", skip_serializing_if = "Option::is_none")]
    pub carried: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetBalancesResponse {
    #[serde(rename = "balances", skip_serializing_if = "Option::is_none")]
    pub balances: Option<Vec<AccountBalance>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "totalPages", skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<u64>,
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetFundingStateResponse {
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "currentFundingRate", skip_serializing_if = "Option::is_none")]
    pub current_funding_rate: Option<String>,
    #[serde(rename = "estimatedNextFundingRate", skip_serializing_if = "Option::is_none")]
    pub estimated_next_funding_rate: Option<String>,
    #[serde(rename = "nextFundingTime", skip_serializing_if = "Option::is_none")]
    pub next_funding_time: Option<String>,
    #[serde(rename = "fundingIntervalSeconds", skip_serializing_if = "Option::is_none")]
    pub funding_interval_seconds: Option<u64>,
    #[serde(rename = "lastFundingTime", skip_serializing_if = "Option::is_none")]
    pub last_funding_time: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetIndexPriceResponse {
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "indexPrice", skip_serializing_if = "Option::is_none")]
    pub index_price: Option<String>,
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<IndexComponent>>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMarginAccountSummaryResponse {
    #[serde(rename = "marginAccountId", skip_serializing_if = "Option::is_none")]
    pub margin_account_id: Option<String>,
    #[serde(rename = "equity", skip_serializing_if = "Option::is_none")]
    pub equity: Option<String>,
    #[serde(rename = "initialMarginRequired", skip_serializing_if = "Option::is_none")]
    pub initial_margin_required: Option<String>,
    #[serde(rename = "maintenanceMarginRequired", skip_serializing_if = "Option::is_none")]
    pub maintenance_margin_required: Option<String>,
    #[serde(rename = "freeCollateral", skip_serializing_if = "Option::is_none")]
    pub free_collateral: Option<String>,
    #[serde(rename = "withdrawableCollateral", skip_serializing_if = "Option::is_none")]
    pub withdrawable_collateral: Option<String>,
    #[serde(rename = "totalPositionNotional", skip_serializing_if = "Option::is_none")]
    pub total_position_notional: Option<String>,
    #[serde(rename = "unrealizedPnl", skip_serializing_if = "Option::is_none")]
    pub unrealized_pnl: Option<String>,
    #[serde(rename = "realizedPnl", skip_serializing_if = "Option::is_none")]
    pub realized_pnl: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "strategyKey", skip_serializing_if = "Option::is_none")]
    pub strategy_key: Option<String>,
    #[serde(rename = "riskBucketId", skip_serializing_if = "Option::is_none")]
    pub risk_bucket_id: Option<String>,
    #[serde(rename = "marginMode", skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(rename = "selectedTradingPairIds", skip_serializing_if = "Option::is_none")]
    pub selected_trading_pair_ids: Option<Vec<String>>,
    #[serde(rename = "availableOrderCollateral", skip_serializing_if = "Option::is_none")]
    pub available_order_collateral: Option<String>,
    #[serde(rename = "copyFollowId", skip_serializing_if = "Option::is_none")]
    pub copy_follow_id: Option<String>,
    #[serde(rename = "copyLeaderHandle", skip_serializing_if = "Option::is_none")]
    pub copy_leader_handle: Option<String>,
    #[serde(rename = "riskBucketState", skip_serializing_if = "Option::is_none")]
    pub risk_bucket_state: Option<String>,
    #[serde(rename = "subaccountKey", skip_serializing_if = "Option::is_none")]
    pub subaccount_key: Option<String>,
    #[serde(rename = "marginAccountState", skip_serializing_if = "Option::is_none")]
    pub margin_account_state: Option<String>,
    #[serde(rename = "maxRemovableCollateral", skip_serializing_if = "Option::is_none")]
    pub max_removable_collateral: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMarkPriceResponse {
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "markPrice", skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "oracleProvider", skip_serializing_if = "Option::is_none")]
    pub oracle_provider: Option<String>,
    #[serde(rename = "oracleEpoch", skip_serializing_if = "Option::is_none")]
    pub oracle_epoch: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "regime", skip_serializing_if = "Option::is_none")]
    pub regime: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMarketMetadataResponse {
    #[serde(rename = "baseIconUrl", skip_serializing_if = "Option::is_none")]
    pub base_icon_url: Option<String>,
    #[serde(rename = "high24h", skip_serializing_if = "Option::is_none")]
    pub high24h: Option<String>,
    #[serde(rename = "lastPrice", skip_serializing_if = "Option::is_none")]
    pub last_price: Option<String>,
    #[serde(rename = "lastPriceTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_price_timestamp: Option<String>,
    #[serde(rename = "low24h", skip_serializing_if = "Option::is_none")]
    pub low24h: Option<String>,
    #[serde(rename = "marketInitializationTimestamp", skip_serializing_if = "Option::is_none")]
    pub market_initialization_timestamp: Option<String>,
    #[serde(rename = "priceChange24h", skip_serializing_if = "Option::is_none")]
    pub price_change24h: Option<String>,
    #[serde(rename = "priceChangePercent24h", skip_serializing_if = "Option::is_none")]
    pub price_change_percent24h: Option<String>,
    #[serde(rename = "quoteIconUrl", skip_serializing_if = "Option::is_none")]
    pub quote_icon_url: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "volume24h", skip_serializing_if = "Option::is_none")]
    pub volume24h: Option<String>,
    #[serde(rename = "minLeverage", skip_serializing_if = "Option::is_none")]
    pub min_leverage: Option<String>,
    #[serde(rename = "maxLeverage", skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<String>,
    #[serde(rename = "markPrice", skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "indexPrice", skip_serializing_if = "Option::is_none")]
    pub index_price: Option<String>,
    #[serde(rename = "totalBaseVolumeLtd", skip_serializing_if = "Option::is_none")]
    pub total_base_volume_ltd: Option<String>,
    #[serde(rename = "totalQuoteVolumeLtd", skip_serializing_if = "Option::is_none")]
    pub total_quote_volume_ltd: Option<String>,
    #[serde(rename = "totalTradeCountLtd", skip_serializing_if = "Option::is_none")]
    pub total_trade_count_ltd: Option<i64>,
    #[serde(rename = "quoteVolume24h", skip_serializing_if = "Option::is_none")]
    pub quote_volume24h: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetOrderResponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "orderType", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "filledQuantity", skip_serializing_if = "Option::is_none")]
    pub filled_quantity: Option<String>,
    #[serde(rename = "averageFillPrice", skip_serializing_if = "Option::is_none")]
    pub average_fill_price: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tradingMode", skip_serializing_if = "Option::is_none")]
    pub trading_mode: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "applicationTakerFee", skip_serializing_if = "Option::is_none")]
    pub application_taker_fee: Option<String>,
    #[serde(rename = "monacoTakerFee", skip_serializing_if = "Option::is_none")]
    pub monaco_taker_fee: Option<String>,
    #[serde(rename = "monacoMakerRebate", skip_serializing_if = "Option::is_none")]
    pub monaco_maker_rebate: Option<String>,
    #[serde(rename = "totalTakerFees", skip_serializing_if = "Option::is_none")]
    pub total_taker_fees: Option<String>,
    #[serde(rename = "takerTotalPayment", skip_serializing_if = "Option::is_none")]
    pub taker_total_payment: Option<String>,
    #[serde(rename = "makerTotalReceipt", skip_serializing_if = "Option::is_none")]
    pub maker_total_receipt: Option<String>,
    #[serde(rename = "marginAccountId", skip_serializing_if = "Option::is_none")]
    pub margin_account_id: Option<String>,
    #[serde(rename = "positionSide", skip_serializing_if = "Option::is_none")]
    pub position_side: Option<String>,
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "positionId", skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(rename = "quantityRaw", skip_serializing_if = "Option::is_none")]
    pub quantity_raw: Option<String>,
    #[serde(rename = "filledQuantityRaw", skip_serializing_if = "Option::is_none")]
    pub filled_quantity_raw: Option<String>,
    #[serde(rename = "remainingQuantity", skip_serializing_if = "Option::is_none")]
    pub remaining_quantity: Option<String>,
    #[serde(rename = "remainingQuantityRaw", skip_serializing_if = "Option::is_none")]
    pub remaining_quantity_raw: Option<String>,
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "cancelledAt", skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<String>,
    #[serde(rename = "filledAt", skip_serializing_if = "Option::is_none")]
    pub filled_at: Option<String>,
    #[serde(rename = "expiredAt", skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,
    #[serde(rename = "submittedAt", skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
    #[serde(rename = "acknowledgedAt", skip_serializing_if = "Option::is_none")]
    pub acknowledged_at: Option<String>,
    #[serde(rename = "triggerPrice", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "quoteVolume", skip_serializing_if = "Option::is_none")]
    pub quote_volume: Option<String>,
    #[serde(rename = "applicationId", skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "parentOrderId", skip_serializing_if = "Option::is_none")]
    pub parent_order_id: Option<String>,
    #[serde(rename = "batchId", skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    #[serde(rename = "terminalReason", skip_serializing_if = "Option::is_none")]
    pub terminal_reason: Option<String>,
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(rename = "conditionalOrderId", skip_serializing_if = "Option::is_none")]
    pub conditional_order_id: Option<String>,
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "placementTakerFeeBps", skip_serializing_if = "Option::is_none")]
    pub placement_taker_fee_bps: Option<String>,
    #[serde(rename = "placementMakerFeeBps", skip_serializing_if = "Option::is_none")]
    pub placement_maker_fee_bps: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetOrderbookResponse {
    #[serde(rename = "baseDecimals", skip_serializing_if = "Option::is_none")]
    pub base_decimals: Option<i64>,
    #[serde(rename = "baseToken", skip_serializing_if = "Option::is_none")]
    pub base_token: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<OrderbookData>,
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "quoteDecimals", skip_serializing_if = "Option::is_none")]
    pub quote_decimals: Option<i64>,
    #[serde(rename = "quoteToken", skip_serializing_if = "Option::is_none")]
    pub quote_token: Option<String>,
    #[serde(rename = "sequenceNumber", skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<u64>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "tradingMode", skip_serializing_if = "Option::is_none")]
    pub trading_mode: Option<String>,
    #[serde(rename = "bookSeq", skip_serializing_if = "Option::is_none")]
    pub book_seq: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetTradesResponse {
    #[serde(rename = "trades", skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<PublicTrade>>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetTradingPairResponse {
    #[serde(rename = "tradingPair", skip_serializing_if = "Option::is_none")]
    pub trading_pair: Option<TradingPairData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IndexComponent {
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListFundingHistoryResponse {
    #[serde(rename = "records", skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<FundingRecord>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "totalCapped", skip_serializing_if = "Option::is_none")]
    pub total_capped: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListMarginAccountsResponse {
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<MarginAccountSummary>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListOrdersResponse {
    #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<GetOrderResponse>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "totalPages", skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<u64>,
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "totalCapped", skip_serializing_if = "Option::is_none")]
    pub total_capped: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListPositionsResponse {
    #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<Position>>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(rename = "totalCapped", skip_serializing_if = "Option::is_none")]
    pub total_capped: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ListTradingPairsResponse {
    #[serde(rename = "tradingPairs", skip_serializing_if = "Option::is_none")]
    pub trading_pairs: Option<Vec<TradingPairData>>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    #[serde(rename = "totalPages", skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarginAccountSummary {
    #[serde(rename = "marginAccountId", skip_serializing_if = "Option::is_none")]
    pub margin_account_id: Option<String>,
    #[serde(rename = "equity", skip_serializing_if = "Option::is_none")]
    pub equity: Option<String>,
    #[serde(rename = "initialMarginRequired", skip_serializing_if = "Option::is_none")]
    pub initial_margin_required: Option<String>,
    #[serde(rename = "maintenanceMarginRequired", skip_serializing_if = "Option::is_none")]
    pub maintenance_margin_required: Option<String>,
    #[serde(rename = "freeCollateral", skip_serializing_if = "Option::is_none")]
    pub free_collateral: Option<String>,
    #[serde(rename = "withdrawableCollateral", skip_serializing_if = "Option::is_none")]
    pub withdrawable_collateral: Option<String>,
    #[serde(rename = "totalPositionNotional", skip_serializing_if = "Option::is_none")]
    pub total_position_notional: Option<String>,
    #[serde(rename = "unrealizedPnl", skip_serializing_if = "Option::is_none")]
    pub unrealized_pnl: Option<String>,
    #[serde(rename = "realizedPnl", skip_serializing_if = "Option::is_none")]
    pub realized_pnl: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "strategyKey", skip_serializing_if = "Option::is_none")]
    pub strategy_key: Option<String>,
    #[serde(rename = "riskBucketId", skip_serializing_if = "Option::is_none")]
    pub risk_bucket_id: Option<String>,
    #[serde(rename = "marginMode", skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(rename = "selectedTradingPairIds", skip_serializing_if = "Option::is_none")]
    pub selected_trading_pair_ids: Option<Vec<String>>,
    #[serde(rename = "availableOrderCollateral", skip_serializing_if = "Option::is_none")]
    pub available_order_collateral: Option<String>,
    #[serde(rename = "copyFollowId", skip_serializing_if = "Option::is_none")]
    pub copy_follow_id: Option<String>,
    #[serde(rename = "copyLeaderHandle", skip_serializing_if = "Option::is_none")]
    pub copy_leader_handle: Option<String>,
    #[serde(rename = "riskBucketState", skip_serializing_if = "Option::is_none")]
    pub risk_bucket_state: Option<String>,
    #[serde(rename = "subaccountKey", skip_serializing_if = "Option::is_none")]
    pub subaccount_key: Option<String>,
    #[serde(rename = "marginAccountState", skip_serializing_if = "Option::is_none")]
    pub margin_account_state: Option<String>,
    #[serde(rename = "maxRemovableCollateral", skip_serializing_if = "Option::is_none")]
    pub max_removable_collateral: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MatchResult {
    #[serde(rename = "tradesCount", skip_serializing_if = "Option::is_none")]
    pub trades_count: Option<i64>,
    #[serde(rename = "totalFilled", skip_serializing_if = "Option::is_none")]
    pub total_filled: Option<String>,
    #[serde(rename = "remainingQuantity", skip_serializing_if = "Option::is_none")]
    pub remaining_quantity: Option<String>,
    #[serde(rename = "averageFillPrice", skip_serializing_if = "Option::is_none")]
    pub average_fill_price: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "actualSlippageBps", skip_serializing_if = "Option::is_none")]
    pub actual_slippage_bps: Option<i64>,
    #[serde(rename = "maxSlippageBps", skip_serializing_if = "Option::is_none")]
    pub max_slippage_bps: Option<i64>,
    #[serde(rename = "executionPriceRange", skip_serializing_if = "Option::is_none")]
    pub execution_price_range: Option<ExecutionPriceRange>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderbookData {
    #[serde(rename = "bids", skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<PriceLevel>>,
    #[serde(rename = "asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<PriceLevel>>,
    #[serde(rename = "bestBid", skip_serializing_if = "Option::is_none")]
    pub best_bid: Option<String>,
    #[serde(rename = "bestAsk", skip_serializing_if = "Option::is_none")]
    pub best_ask: Option<String>,
    #[serde(rename = "bidVolume", skip_serializing_if = "Option::is_none")]
    pub bid_volume: Option<String>,
    #[serde(rename = "askVolume", skip_serializing_if = "Option::is_none")]
    pub ask_volume: Option<String>,
    #[serde(rename = "priceChange", skip_serializing_if = "Option::is_none")]
    pub price_change: Option<PriceChange>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParentTpSlLeg {
    #[serde(rename = "triggerPrice", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "orderType", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(rename = "limitPrice", skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<String>,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "slippageToleranceBps", skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance_bps: Option<i64>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "closePosition", skip_serializing_if = "Option::is_none")]
    pub close_position: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParentTrailingStopLeg {
    #[serde(rename = "trailBps", skip_serializing_if = "Option::is_none")]
    pub trail_bps: Option<u64>,
    #[serde(rename = "activationPrice", skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "closePosition", skip_serializing_if = "Option::is_none")]
    pub close_position: Option<bool>,
    #[serde(rename = "slippageToleranceBps", skip_serializing_if = "Option::is_none")]
    pub slippage_tolerance_bps: Option<i64>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "positionId", skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(rename = "marginAccountId", skip_serializing_if = "Option::is_none")]
    pub margin_account_id: Option<String>,
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "entryPrice", skip_serializing_if = "Option::is_none")]
    pub entry_price: Option<String>,
    #[serde(rename = "markPrice", skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "indexPrice", skip_serializing_if = "Option::is_none")]
    pub index_price: Option<String>,
    #[serde(rename = "unrealizedPnl", skip_serializing_if = "Option::is_none")]
    pub unrealized_pnl: Option<String>,
    #[serde(rename = "realizedPnl", skip_serializing_if = "Option::is_none")]
    pub realized_pnl: Option<String>,
    #[serde(rename = "isolatedMargin", skip_serializing_if = "Option::is_none")]
    pub isolated_margin: Option<String>,
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(rename = "maintenanceMarginRequired", skip_serializing_if = "Option::is_none")]
    pub maintenance_margin_required: Option<String>,
    #[serde(rename = "initialMarginRequired", skip_serializing_if = "Option::is_none")]
    pub initial_margin_required: Option<String>,
    #[serde(rename = "liquidationPrice", skip_serializing_if = "Option::is_none")]
    pub liquidation_price: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "riskBucketId", skip_serializing_if = "Option::is_none")]
    pub risk_bucket_id: Option<String>,
    #[serde(rename = "marginMode", skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(rename = "netRealizedPnl", skip_serializing_if = "Option::is_none")]
    pub net_realized_pnl: Option<String>,
    #[serde(rename = "realizedRoe", skip_serializing_if = "Option::is_none")]
    pub realized_roe: Option<String>,
    #[serde(rename = "exitPrice", skip_serializing_if = "Option::is_none")]
    pub exit_price: Option<String>,
    #[serde(rename = "fundingPaid", skip_serializing_if = "Option::is_none")]
    pub funding_paid: Option<String>,
    #[serde(rename = "feesPaid", skip_serializing_if = "Option::is_none")]
    pub fees_paid: Option<String>,
    #[serde(rename = "cumFees", skip_serializing_if = "Option::is_none")]
    pub cum_fees: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
    #[serde(rename = "openedAt", skip_serializing_if = "Option::is_none")]
    pub opened_at: Option<String>,
    #[serde(rename = "lifetimeTotalsStatus", skip_serializing_if = "Option::is_none")]
    pub lifetime_totals_status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PriceChange {
    #[serde(rename = "price24hAgo", skip_serializing_if = "Option::is_none")]
    pub price24h_ago: Option<String>,
    #[serde(rename = "priceChange24h", skip_serializing_if = "Option::is_none")]
    pub price_change24h: Option<String>,
    #[serde(rename = "priceChangePercent24h", skip_serializing_if = "Option::is_none")]
    pub price_change_percent24h: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "orderCount", skip_serializing_if = "Option::is_none")]
    pub order_count: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicTrade {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<TradeData>,
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "tradingMode", skip_serializing_if = "Option::is_none")]
    pub trading_mode: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RefreshResponse {
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplaceOrderRequest {
    #[serde(rename = "selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "useMasterBalance", skip_serializing_if = "Option::is_none")]
    pub use_master_balance: Option<bool>,
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReplaceOrderResponse {
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "originalOrderId", skip_serializing_if = "Option::is_none")]
    pub original_order_id: Option<String>,
    #[serde(rename = "updatedFields", skip_serializing_if = "Option::is_none")]
    pub updated_fields: Option<UpdatedFields>,
    #[serde(rename = "matchResult", skip_serializing_if = "Option::is_none")]
    pub match_result: Option<MatchResult>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RevokeResponse {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TradeData {
    #[serde(rename = "executedAt", skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
    #[serde(rename = "makerSide", skip_serializing_if = "Option::is_none")]
    pub maker_side: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "quantityRaw", skip_serializing_if = "Option::is_none")]
    pub quantity_raw: Option<String>,
    #[serde(rename = "tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename = "executionSource", skip_serializing_if = "Option::is_none")]
    pub execution_source: Option<String>,
    #[serde(rename = "makerAddress", skip_serializing_if = "Option::is_none")]
    pub maker_address: Option<String>,
    #[serde(rename = "takerAddress", skip_serializing_if = "Option::is_none")]
    pub taker_address: Option<String>,
    #[serde(rename = "takerFeeRaw", skip_serializing_if = "Option::is_none")]
    pub taker_fee_raw: Option<String>,
    #[serde(rename = "quoteVolumeRaw", skip_serializing_if = "Option::is_none")]
    pub quote_volume_raw: Option<String>,
    #[serde(rename = "takerApplicationId", skip_serializing_if = "Option::is_none")]
    pub taker_application_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TradingPairData {
    #[serde(rename = "baseAssetId", skip_serializing_if = "Option::is_none")]
    pub base_asset_id: Option<String>,
    #[serde(rename = "baseDecimals", skip_serializing_if = "Option::is_none")]
    pub base_decimals: Option<i64>,
    #[serde(rename = "baseIconUrl", skip_serializing_if = "Option::is_none")]
    pub base_icon_url: Option<String>,
    #[serde(rename = "baseToken", skip_serializing_if = "Option::is_none")]
    pub base_token: Option<String>,
    #[serde(rename = "baseTokenContract", skip_serializing_if = "Option::is_none")]
    pub base_token_contract: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "makerFeeBps", skip_serializing_if = "Option::is_none")]
    pub maker_fee_bps: Option<i64>,
    #[serde(rename = "marketType", skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(rename = "maxOrderSize", skip_serializing_if = "Option::is_none")]
    pub max_order_size: Option<String>,
    #[serde(rename = "minOrderSize", skip_serializing_if = "Option::is_none")]
    pub min_order_size: Option<String>,
    #[serde(rename = "quoteAssetId", skip_serializing_if = "Option::is_none")]
    pub quote_asset_id: Option<String>,
    #[serde(rename = "quoteDecimals", skip_serializing_if = "Option::is_none")]
    pub quote_decimals: Option<i64>,
    #[serde(rename = "quoteIconUrl", skip_serializing_if = "Option::is_none")]
    pub quote_icon_url: Option<String>,
    #[serde(rename = "quoteToken", skip_serializing_if = "Option::is_none")]
    pub quote_token: Option<String>,
    #[serde(rename = "quoteTokenContract", skip_serializing_if = "Option::is_none")]
    pub quote_token_contract: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "takerFeeBps", skip_serializing_if = "Option::is_none")]
    pub taker_fee_bps: Option<i64>,
    #[serde(rename = "tickSize", skip_serializing_if = "Option::is_none")]
    pub tick_size: Option<String>,
    #[serde(rename = "minLeverage", skip_serializing_if = "Option::is_none")]
    pub min_leverage: Option<String>,
    #[serde(rename = "maxLeverage", skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "quantityStepSize", skip_serializing_if = "Option::is_none")]
    pub quantity_step_size: Option<String>,
    #[serde(rename = "baseAssetName", skip_serializing_if = "Option::is_none")]
    pub base_asset_name: Option<String>,
    #[serde(rename = "quoteAssetName", skip_serializing_if = "Option::is_none")]
    pub quote_asset_name: Option<String>,
    #[serde(rename = "isolatedOnly", skip_serializing_if = "Option::is_none")]
    pub isolated_only: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdatedFields {
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerifyRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "chainId", skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(rename = "sessionPublicKey")]
    pub session_public_key: String,
    #[serde(rename = "referralCode", skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "inviteCode", skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VerifyResponse {
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<UserInfo>,
    #[serde(rename = "isNewUser", skip_serializing_if = "Option::is_none")]
    pub is_new_user: Option<bool>,
    #[serde(rename = "referralApplied", skip_serializing_if = "Option::is_none")]
    pub referral_applied: Option<bool>,
    #[serde(rename = "referralNotAppliedReason", skip_serializing_if = "Option::is_none")]
    pub referral_not_applied_reason: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TradingPairsRequest {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "marketType", skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(rename = "baseToken", skip_serializing_if = "Option::is_none")]
    pub base_token: Option<String>,
    #[serde(rename = "quoteToken", skip_serializing_if = "Option::is_none")]
    pub quote_token: Option<String>,
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TradingPairRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketMetadataRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderBookRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
    #[serde(rename = "levels", skip_serializing_if = "Option::is_none")]
    pub levels: Option<u64>,
    #[serde(rename = "tradingMode", skip_serializing_if = "Option::is_none")]
    pub trading_mode: Option<String>,
    #[serde(rename = "magnitude", skip_serializing_if = "Option::is_none")]
    pub magnitude: Option<String>,
    #[serde(rename = "denomination", skip_serializing_if = "Option::is_none")]
    pub denomination: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TradesRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "pageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarkPriceRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndexPriceRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FundingRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FundingHistoryRequest {
    #[serde(rename = "tradingPairId")]
    pub trading_pair_id: String,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "pageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BalancesRequest {
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "pageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParentMarginAccountRequest {
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarginAccountsRequest {
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "pageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PositionsRequest {
    #[serde(rename = "marginAccountId", skip_serializing_if = "Option::is_none")]
    pub margin_account_id: Option<String>,
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "pageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersRequest {
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
    #[serde(rename = "tradingMode", skip_serializing_if = "Option::is_none")]
    pub trading_mode: Option<String>,
    #[serde(rename = "marginAccountId", skip_serializing_if = "Option::is_none")]
    pub margin_account_id: Option<String>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "pageToken", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderRequest {
    #[serde(rename = "orderId")]
    pub order_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmptyRequest {
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceOrderParams {
    #[serde(rename = "market")]
    pub market: TradingPairData,
    #[serde(rename = "body")]
    pub body: CreateOrderRequest,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplaceOrderParams {
    #[serde(rename = "market")]
    pub market: TradingPairData,
    #[serde(rename = "body")]
    pub body: ReplaceOrderRequest,
    #[serde(rename = "orderId")]
    pub order_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchCreateParams {
    #[serde(rename = "markets")]
    pub markets: Vec<TradingPairData>,
    #[serde(rename = "body")]
    pub body: BatchCreateOrdersRequest,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchReplaceParams {
    #[serde(rename = "markets")]
    pub markets: Vec<TradingPairData>,
    #[serde(rename = "body")]
    pub body: BatchReplaceOrdersRequest,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelAllParams {
    #[serde(rename = "tradingPairId", skip_serializing_if = "Option::is_none")]
    pub trading_pair_id: Option<String>,
}

pub struct TypedMonacoClient(MonacoClient);
impl MonacoClient {
    pub fn typed(self) -> TypedMonacoClient { TypedMonacoClient(self) }
}
impl TypedMonacoClient {
    pub fn balances(&self, request: &BalancesRequest) -> Result<GetBalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn batch_cancel(&self, request: &BatchCancelOrdersRequest) -> Result<BatchCancelOrdersResponse> {
        serde_json::from_value(self.0.batch_cancel(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn batch_create(&self, request: &BatchCreateParams) -> Result<BatchCreateOrdersResponse> {
        serde_json::from_value(self.0.batch_create(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn batch_replace(&self, request: &BatchReplaceParams) -> Result<BatchReplaceOrdersResponse> {
        serde_json::from_value(self.0.batch_replace(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn cancel_all(&self, request: &CancelAllParams) -> Result<BatchCancelAllResponse> {
        serde_json::from_value(self.0.cancel_all(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn create_challenge(&self, request: &ChallengeRequest) -> Result<ChallengeResponse> {
        serde_json::from_value(self.0.create_challenge(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn funding(&self, request: &FundingRequest) -> Result<GetFundingStateResponse> {
        serde_json::from_value(self.0.funding(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn funding_history(&self, request: &FundingHistoryRequest) -> Result<ListFundingHistoryResponse> {
        serde_json::from_value(self.0.funding_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn index_price(&self, request: &IndexPriceRequest) -> Result<GetIndexPriceResponse> {
        serde_json::from_value(self.0.index_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn margin_accounts(&self, request: &MarginAccountsRequest) -> Result<ListMarginAccountsResponse> {
        serde_json::from_value(self.0.margin_accounts(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn mark_price(&self, request: &MarkPriceRequest) -> Result<GetMarkPriceResponse> {
        serde_json::from_value(self.0.mark_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn market_metadata(&self, request: &MarketMetadataRequest) -> Result<GetMarketMetadataResponse> {
        serde_json::from_value(self.0.market_metadata(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn order(&self, request: &OrderRequest) -> Result<GetOrderResponse> {
        serde_json::from_value(self.0.order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<GetOrderbookResponse> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn orders(&self, request: &OrdersRequest) -> Result<ListOrdersResponse> {
        serde_json::from_value(self.0.orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn parent_margin_account(&self, request: &ParentMarginAccountRequest) -> Result<GetMarginAccountSummaryResponse> {
        serde_json::from_value(self.0.parent_margin_account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderParams) -> Result<CreateOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<ListPositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn refresh_session(&self, request: &EmptyRequest) -> Result<RefreshResponse> {
        serde_json::from_value(self.0.refresh_session(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn replace_order(&self, request: &ReplaceOrderParams) -> Result<ReplaceOrderResponse> {
        serde_json::from_value(self.0.replace_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn revoke_session(&self, request: &EmptyRequest) -> Result<RevokeResponse> {
        serde_json::from_value(self.0.revoke_session(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<GetTradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn trading_pair(&self, request: &TradingPairRequest) -> Result<GetTradingPairResponse> {
        serde_json::from_value(self.0.trading_pair(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn trading_pairs(&self, request: &TradingPairsRequest) -> Result<ListTradingPairsResponse> {
        serde_json::from_value(self.0.trading_pairs(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub fn verify_signature(&self, request: &VerifyRequest) -> Result<VerifyResponse> {
        serde_json::from_value(self.0.verify_signature(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedMonacoClient(crate::AsyncClient<MonacoClient>);
#[cfg(feature = "async")]
impl TypedMonacoClient {
    pub fn into_async(self) -> AsyncTypedMonacoClient { AsyncTypedMonacoClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedMonacoClient {
    pub async fn balances(&self, request: &BalancesRequest) -> Result<GetBalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn batch_cancel(&self, request: &BatchCancelOrdersRequest) -> Result<BatchCancelOrdersResponse> {
        serde_json::from_value(self.0.batch_cancel(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn batch_create(&self, request: &BatchCreateParams) -> Result<BatchCreateOrdersResponse> {
        serde_json::from_value(self.0.batch_create(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn batch_replace(&self, request: &BatchReplaceParams) -> Result<BatchReplaceOrdersResponse> {
        serde_json::from_value(self.0.batch_replace(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn cancel_all(&self, request: &CancelAllParams) -> Result<BatchCancelAllResponse> {
        serde_json::from_value(self.0.cancel_all(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn create_challenge(&self, request: &ChallengeRequest) -> Result<ChallengeResponse> {
        serde_json::from_value(self.0.create_challenge(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn funding(&self, request: &FundingRequest) -> Result<GetFundingStateResponse> {
        serde_json::from_value(self.0.funding(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn funding_history(&self, request: &FundingHistoryRequest) -> Result<ListFundingHistoryResponse> {
        serde_json::from_value(self.0.funding_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn index_price(&self, request: &IndexPriceRequest) -> Result<GetIndexPriceResponse> {
        serde_json::from_value(self.0.index_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn margin_accounts(&self, request: &MarginAccountsRequest) -> Result<ListMarginAccountsResponse> {
        serde_json::from_value(self.0.margin_accounts(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn mark_price(&self, request: &MarkPriceRequest) -> Result<GetMarkPriceResponse> {
        serde_json::from_value(self.0.mark_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn market_metadata(&self, request: &MarketMetadataRequest) -> Result<GetMarketMetadataResponse> {
        serde_json::from_value(self.0.market_metadata(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn order(&self, request: &OrderRequest) -> Result<GetOrderResponse> {
        serde_json::from_value(self.0.order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<GetOrderbookResponse> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn orders(&self, request: &OrdersRequest) -> Result<ListOrdersResponse> {
        serde_json::from_value(self.0.orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn parent_margin_account(&self, request: &ParentMarginAccountRequest) -> Result<GetMarginAccountSummaryResponse> {
        serde_json::from_value(self.0.parent_margin_account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderParams) -> Result<CreateOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<ListPositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn refresh_session(&self, request: &EmptyRequest) -> Result<RefreshResponse> {
        serde_json::from_value(self.0.refresh_session(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn replace_order(&self, request: &ReplaceOrderParams) -> Result<ReplaceOrderResponse> {
        serde_json::from_value(self.0.replace_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn revoke_session(&self, request: &EmptyRequest) -> Result<RevokeResponse> {
        serde_json::from_value(self.0.revoke_session(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<GetTradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn trading_pair(&self, request: &TradingPairRequest) -> Result<GetTradingPairResponse> {
        serde_json::from_value(self.0.trading_pair(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn trading_pairs(&self, request: &TradingPairsRequest) -> Result<ListTradingPairsResponse> {
        serde_json::from_value(self.0.trading_pairs(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
    pub async fn verify_signature(&self, request: &VerifyRequest) -> Result<VerifyResponse> {
        serde_json::from_value(self.0.verify_signature(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Monaco response model"))
    }
}
