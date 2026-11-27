// Generated from src/exchanges/binance_spot/schema/binance_spot.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{BinanceSpotClient, Error, Result};

pub type MarketsResponse = serde_json::Value;
pub type TickerResponse = serde_json::Value;
pub type TradesResponse = serde_json::Value;
pub type AggTradesResponse = serde_json::Value;
pub type AccountResponse = serde_json::Value;
pub type OrderStatusResponse = serde_json::Value;
pub type OpenOrdersResponse = serde_json::Value;
pub type OrderHistoryResponse = serde_json::Value;
pub type FillsResponse = serde_json::Value;
pub type PlaceOrderResponse = serde_json::Value;
pub type CancelOrderResponse = serde_json::Value;
pub type CancelAllOrdersResponse = serde_json::Value;
pub type CancelReplaceOrderResponse = serde_json::Value;
pub type AmendOrderResponse = serde_json::Value;
pub type PlaceOcoResponse = serde_json::Value;
pub type PlaceOtoResponse = serde_json::Value;
pub type PlaceOtocoResponse = serde_json::Value;
pub type PlaceOpoResponse = serde_json::Value;
pub type PlaceOpocoResponse = serde_json::Value;
pub type CancelOrderListResponse = serde_json::Value;
pub type OrderListStatusResponse = serde_json::Value;
pub type OrderListHistoryResponse = serde_json::Value;
pub type OpenOrderListsResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<u64>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DepthRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    pub symbol_status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DepthResponse {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    #[serde(rename = "bids")]
    pub bids: Vec<[String; 2]>,
    #[serde(rename = "asks")]
    pub asks: Vec<[String; 2]>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "showPermissionSets", skip_serializing_if = "Option::is_none")]
    pub show_permission_sets: Option<bool>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    pub symbol_status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TickerRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "symbolStatus", skip_serializing_if = "Option::is_none")]
    pub symbol_status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AggTradesRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(rename = "omitZeroBalances", skip_serializing_if = "Option::is_none")]
    pub omit_zero_balances: Option<bool>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<u64>,
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<u64>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<u64>,
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<String>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<String>,
    #[serde(rename = "selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(rename = "pegPriceType", skip_serializing_if = "Option::is_none")]
    pub peg_price_type: Option<String>,
    #[serde(rename = "pegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub peg_offset_value: Option<u64>,
    #[serde(rename = "pegOffsetType", skip_serializing_if = "Option::is_none")]
    pub peg_offset_type: Option<String>,
    #[serde(rename = "quoteNotionalLimit", skip_serializing_if = "Option::is_none")]
    pub quote_notional_limit: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,
    #[serde(rename = "cancelRestrictions", skip_serializing_if = "Option::is_none")]
    pub cancel_restrictions: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAllOrdersRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelReplaceOrderRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "cancelReplaceMode")]
    pub cancel_replace_mode: String,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "quoteOrderQty", skip_serializing_if = "Option::is_none")]
    pub quote_order_qty: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "cancelNewClientOrderId", skip_serializing_if = "Option::is_none")]
    pub cancel_new_client_order_id: Option<String>,
    #[serde(rename = "cancelOrigClientOrderId", skip_serializing_if = "Option::is_none")]
    pub cancel_orig_client_order_id: Option<String>,
    #[serde(rename = "cancelOrderId", skip_serializing_if = "Option::is_none")]
    pub cancel_order_id: Option<u64>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,
    #[serde(rename = "strategyId", skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<u64>,
    #[serde(rename = "strategyType", skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<u64>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    #[serde(rename = "trailingDelta", skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<u64>,
    #[serde(rename = "icebergQty", skip_serializing_if = "Option::is_none")]
    pub iceberg_qty: Option<String>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<String>,
    #[serde(rename = "selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(rename = "cancelRestrictions", skip_serializing_if = "Option::is_none")]
    pub cancel_restrictions: Option<String>,
    #[serde(rename = "orderRateLimitExceededMode", skip_serializing_if = "Option::is_none")]
    pub order_rate_limit_exceeded_mode: Option<String>,
    #[serde(rename = "pegPriceType", skip_serializing_if = "Option::is_none")]
    pub peg_price_type: Option<String>,
    #[serde(rename = "pegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub peg_offset_value: Option<u64>,
    #[serde(rename = "pegOffsetType", skip_serializing_if = "Option::is_none")]
    pub peg_offset_type: Option<String>,
    #[serde(rename = "quoteNotionalLimit", skip_serializing_if = "Option::is_none")]
    pub quote_notional_limit: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendOrderRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "newQty")]
    pub new_qty: String,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOcoRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "aboveType")]
    pub above_type: String,
    #[serde(rename = "aboveClientOrderId", skip_serializing_if = "Option::is_none")]
    pub above_client_order_id: Option<String>,
    #[serde(rename = "aboveIcebergQty", skip_serializing_if = "Option::is_none")]
    pub above_iceberg_qty: Option<String>,
    #[serde(rename = "abovePrice", skip_serializing_if = "Option::is_none")]
    pub above_price: Option<String>,
    #[serde(rename = "aboveStopPrice", skip_serializing_if = "Option::is_none")]
    pub above_stop_price: Option<String>,
    #[serde(rename = "aboveTrailingDelta", skip_serializing_if = "Option::is_none")]
    pub above_trailing_delta: Option<u64>,
    #[serde(rename = "aboveTimeInForce", skip_serializing_if = "Option::is_none")]
    pub above_time_in_force: Option<String>,
    #[serde(rename = "aboveStrategyId", skip_serializing_if = "Option::is_none")]
    pub above_strategy_id: Option<u64>,
    #[serde(rename = "aboveStrategyType", skip_serializing_if = "Option::is_none")]
    pub above_strategy_type: Option<u64>,
    #[serde(rename = "abovePegPriceType", skip_serializing_if = "Option::is_none")]
    pub above_peg_price_type: Option<String>,
    #[serde(rename = "abovePegOffsetType", skip_serializing_if = "Option::is_none")]
    pub above_peg_offset_type: Option<String>,
    #[serde(rename = "abovePegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub above_peg_offset_value: Option<u64>,
    #[serde(rename = "belowType")]
    pub below_type: String,
    #[serde(rename = "belowClientOrderId", skip_serializing_if = "Option::is_none")]
    pub below_client_order_id: Option<String>,
    #[serde(rename = "belowIcebergQty", skip_serializing_if = "Option::is_none")]
    pub below_iceberg_qty: Option<String>,
    #[serde(rename = "belowPrice", skip_serializing_if = "Option::is_none")]
    pub below_price: Option<String>,
    #[serde(rename = "belowStopPrice", skip_serializing_if = "Option::is_none")]
    pub below_stop_price: Option<String>,
    #[serde(rename = "belowTrailingDelta", skip_serializing_if = "Option::is_none")]
    pub below_trailing_delta: Option<u64>,
    #[serde(rename = "belowTimeInForce", skip_serializing_if = "Option::is_none")]
    pub below_time_in_force: Option<String>,
    #[serde(rename = "belowStrategyId", skip_serializing_if = "Option::is_none")]
    pub below_strategy_id: Option<u64>,
    #[serde(rename = "belowStrategyType", skip_serializing_if = "Option::is_none")]
    pub below_strategy_type: Option<u64>,
    #[serde(rename = "belowPegPriceType", skip_serializing_if = "Option::is_none")]
    pub below_peg_price_type: Option<String>,
    #[serde(rename = "belowPegOffsetType", skip_serializing_if = "Option::is_none")]
    pub below_peg_offset_type: Option<String>,
    #[serde(rename = "belowPegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub below_peg_offset_value: Option<u64>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<String>,
    #[serde(rename = "selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(rename = "aboveQuoteNotionalLimit", skip_serializing_if = "Option::is_none")]
    pub above_quote_notional_limit: Option<String>,
    #[serde(rename = "belowQuoteNotionalLimit", skip_serializing_if = "Option::is_none")]
    pub below_quote_notional_limit: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOtoRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<String>,
    #[serde(rename = "selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(rename = "workingType")]
    pub working_type: String,
    #[serde(rename = "workingSide")]
    pub working_side: String,
    #[serde(rename = "workingClientOrderId", skip_serializing_if = "Option::is_none")]
    pub working_client_order_id: Option<String>,
    #[serde(rename = "workingPrice")]
    pub working_price: String,
    #[serde(rename = "workingQuantity")]
    pub working_quantity: String,
    #[serde(rename = "workingIcebergQty", skip_serializing_if = "Option::is_none")]
    pub working_iceberg_qty: Option<String>,
    #[serde(rename = "workingTimeInForce", skip_serializing_if = "Option::is_none")]
    pub working_time_in_force: Option<String>,
    #[serde(rename = "workingStrategyId", skip_serializing_if = "Option::is_none")]
    pub working_strategy_id: Option<u64>,
    #[serde(rename = "workingStrategyType", skip_serializing_if = "Option::is_none")]
    pub working_strategy_type: Option<u64>,
    #[serde(rename = "pendingType")]
    pub pending_type: String,
    #[serde(rename = "workingPegPriceType", skip_serializing_if = "Option::is_none")]
    pub working_peg_price_type: Option<String>,
    #[serde(rename = "workingPegOffsetType", skip_serializing_if = "Option::is_none")]
    pub working_peg_offset_type: Option<String>,
    #[serde(rename = "workingPegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub working_peg_offset_value: Option<u64>,
    #[serde(rename = "pendingSide")]
    pub pending_side: String,
    #[serde(rename = "pendingClientOrderId", skip_serializing_if = "Option::is_none")]
    pub pending_client_order_id: Option<String>,
    #[serde(rename = "pendingPrice", skip_serializing_if = "Option::is_none")]
    pub pending_price: Option<String>,
    #[serde(rename = "pendingStopPrice", skip_serializing_if = "Option::is_none")]
    pub pending_stop_price: Option<String>,
    #[serde(rename = "pendingTrailingDelta", skip_serializing_if = "Option::is_none")]
    pub pending_trailing_delta: Option<u64>,
    #[serde(rename = "pendingQuantity")]
    pub pending_quantity: String,
    #[serde(rename = "pendingIcebergQty", skip_serializing_if = "Option::is_none")]
    pub pending_iceberg_qty: Option<String>,
    #[serde(rename = "pendingTimeInForce", skip_serializing_if = "Option::is_none")]
    pub pending_time_in_force: Option<String>,
    #[serde(rename = "pendingStrategyId", skip_serializing_if = "Option::is_none")]
    pub pending_strategy_id: Option<u64>,
    #[serde(rename = "pendingStrategyType", skip_serializing_if = "Option::is_none")]
    pub pending_strategy_type: Option<u64>,
    #[serde(rename = "pendingPegPriceType", skip_serializing_if = "Option::is_none")]
    pub pending_peg_price_type: Option<String>,
    #[serde(rename = "pendingPegOffsetType", skip_serializing_if = "Option::is_none")]
    pub pending_peg_offset_type: Option<String>,
    #[serde(rename = "pendingPegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub pending_peg_offset_value: Option<u64>,
    #[serde(rename = "workingQuoteNotionalLimit", skip_serializing_if = "Option::is_none")]
    pub working_quote_notional_limit: Option<String>,
    #[serde(rename = "pendingQuoteNotionalLimit", skip_serializing_if = "Option::is_none")]
    pub pending_quote_notional_limit: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOtocoRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<String>,
    #[serde(rename = "selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(rename = "workingType")]
    pub working_type: String,
    #[serde(rename = "workingSide")]
    pub working_side: String,
    #[serde(rename = "workingClientOrderId", skip_serializing_if = "Option::is_none")]
    pub working_client_order_id: Option<String>,
    #[serde(rename = "workingPrice")]
    pub working_price: String,
    #[serde(rename = "workingQuantity")]
    pub working_quantity: String,
    #[serde(rename = "workingIcebergQty", skip_serializing_if = "Option::is_none")]
    pub working_iceberg_qty: Option<String>,
    #[serde(rename = "workingTimeInForce", skip_serializing_if = "Option::is_none")]
    pub working_time_in_force: Option<String>,
    #[serde(rename = "workingStrategyId", skip_serializing_if = "Option::is_none")]
    pub working_strategy_id: Option<u64>,
    #[serde(rename = "workingStrategyType", skip_serializing_if = "Option::is_none")]
    pub working_strategy_type: Option<u64>,
    #[serde(rename = "workingPegPriceType", skip_serializing_if = "Option::is_none")]
    pub working_peg_price_type: Option<String>,
    #[serde(rename = "workingPegOffsetType", skip_serializing_if = "Option::is_none")]
    pub working_peg_offset_type: Option<String>,
    #[serde(rename = "workingPegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub working_peg_offset_value: Option<u64>,
    #[serde(rename = "pendingSide")]
    pub pending_side: String,
    #[serde(rename = "pendingQuantity")]
    pub pending_quantity: String,
    #[serde(rename = "pendingAboveType")]
    pub pending_above_type: String,
    #[serde(rename = "pendingAboveClientOrderId", skip_serializing_if = "Option::is_none")]
    pub pending_above_client_order_id: Option<String>,
    #[serde(rename = "pendingAbovePrice", skip_serializing_if = "Option::is_none")]
    pub pending_above_price: Option<String>,
    #[serde(rename = "pendingAboveStopPrice", skip_serializing_if = "Option::is_none")]
    pub pending_above_stop_price: Option<String>,
    #[serde(rename = "pendingAboveTrailingDelta", skip_serializing_if = "Option::is_none")]
    pub pending_above_trailing_delta: Option<u64>,
    #[serde(rename = "pendingAboveIcebergQty", skip_serializing_if = "Option::is_none")]
    pub pending_above_iceberg_qty: Option<String>,
    #[serde(rename = "pendingAboveTimeInForce", skip_serializing_if = "Option::is_none")]
    pub pending_above_time_in_force: Option<String>,
    #[serde(rename = "pendingAboveStrategyId", skip_serializing_if = "Option::is_none")]
    pub pending_above_strategy_id: Option<u64>,
    #[serde(rename = "pendingAboveStrategyType", skip_serializing_if = "Option::is_none")]
    pub pending_above_strategy_type: Option<u64>,
    #[serde(rename = "pendingAbovePegPriceType", skip_serializing_if = "Option::is_none")]
    pub pending_above_peg_price_type: Option<String>,
    #[serde(rename = "pendingAbovePegOffsetType", skip_serializing_if = "Option::is_none")]
    pub pending_above_peg_offset_type: Option<String>,
    #[serde(rename = "pendingAbovePegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub pending_above_peg_offset_value: Option<u64>,
    #[serde(rename = "pendingBelowType", skip_serializing_if = "Option::is_none")]
    pub pending_below_type: Option<String>,
    #[serde(rename = "pendingBelowClientOrderId", skip_serializing_if = "Option::is_none")]
    pub pending_below_client_order_id: Option<String>,
    #[serde(rename = "pendingBelowPrice", skip_serializing_if = "Option::is_none")]
    pub pending_below_price: Option<String>,
    #[serde(rename = "pendingBelowStopPrice", skip_serializing_if = "Option::is_none")]
    pub pending_below_stop_price: Option<String>,
    #[serde(rename = "pendingBelowTrailingDelta", skip_serializing_if = "Option::is_none")]
    pub pending_below_trailing_delta: Option<u64>,
    #[serde(rename = "pendingBelowIcebergQty", skip_serializing_if = "Option::is_none")]
    pub pending_below_iceberg_qty: Option<String>,
    #[serde(rename = "pendingBelowTimeInForce", skip_serializing_if = "Option::is_none")]
    pub pending_below_time_in_force: Option<String>,
    #[serde(rename = "pendingBelowStrategyId", skip_serializing_if = "Option::is_none")]
    pub pending_below_strategy_id: Option<u64>,
    #[serde(rename = "pendingBelowStrategyType", skip_serializing_if = "Option::is_none")]
    pub pending_below_strategy_type: Option<u64>,
    #[serde(rename = "pendingBelowPegPriceType", skip_serializing_if = "Option::is_none")]
    pub pending_below_peg_price_type: Option<String>,
    #[serde(rename = "pendingBelowPegOffsetType", skip_serializing_if = "Option::is_none")]
    pub pending_below_peg_offset_type: Option<String>,
    #[serde(rename = "pendingBelowPegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub pending_below_peg_offset_value: Option<u64>,
    #[serde(rename = "workingQuoteNotionalLimit", skip_serializing_if = "Option::is_none")]
    pub working_quote_notional_limit: Option<String>,
    #[serde(rename = "pendingAboveQuoteNotionalLimit", skip_serializing_if = "Option::is_none")]
    pub pending_above_quote_notional_limit: Option<String>,
    #[serde(rename = "pendingBelowQuoteNotionalLimit", skip_serializing_if = "Option::is_none")]
    pub pending_below_quote_notional_limit: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOpoRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<String>,
    #[serde(rename = "selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(rename = "workingType")]
    pub working_type: String,
    #[serde(rename = "workingSide")]
    pub working_side: String,
    #[serde(rename = "workingClientOrderId", skip_serializing_if = "Option::is_none")]
    pub working_client_order_id: Option<String>,
    #[serde(rename = "workingPrice")]
    pub working_price: String,
    #[serde(rename = "workingQuantity")]
    pub working_quantity: String,
    #[serde(rename = "workingIcebergQty", skip_serializing_if = "Option::is_none")]
    pub working_iceberg_qty: Option<String>,
    #[serde(rename = "workingTimeInForce", skip_serializing_if = "Option::is_none")]
    pub working_time_in_force: Option<String>,
    #[serde(rename = "workingStrategyId", skip_serializing_if = "Option::is_none")]
    pub working_strategy_id: Option<u64>,
    #[serde(rename = "workingStrategyType", skip_serializing_if = "Option::is_none")]
    pub working_strategy_type: Option<u64>,
    #[serde(rename = "workingPegPriceType", skip_serializing_if = "Option::is_none")]
    pub working_peg_price_type: Option<String>,
    #[serde(rename = "workingPegOffsetType", skip_serializing_if = "Option::is_none")]
    pub working_peg_offset_type: Option<String>,
    #[serde(rename = "workingPegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub working_peg_offset_value: Option<u64>,
    #[serde(rename = "pendingType")]
    pub pending_type: String,
    #[serde(rename = "pendingSide")]
    pub pending_side: String,
    #[serde(rename = "pendingClientOrderId", skip_serializing_if = "Option::is_none")]
    pub pending_client_order_id: Option<String>,
    #[serde(rename = "pendingPrice", skip_serializing_if = "Option::is_none")]
    pub pending_price: Option<String>,
    #[serde(rename = "pendingStopPrice", skip_serializing_if = "Option::is_none")]
    pub pending_stop_price: Option<String>,
    #[serde(rename = "pendingTrailingDelta", skip_serializing_if = "Option::is_none")]
    pub pending_trailing_delta: Option<u64>,
    #[serde(rename = "pendingIcebergQty", skip_serializing_if = "Option::is_none")]
    pub pending_iceberg_qty: Option<String>,
    #[serde(rename = "pendingTimeInForce", skip_serializing_if = "Option::is_none")]
    pub pending_time_in_force: Option<String>,
    #[serde(rename = "pendingStrategyId", skip_serializing_if = "Option::is_none")]
    pub pending_strategy_id: Option<u64>,
    #[serde(rename = "pendingStrategyType", skip_serializing_if = "Option::is_none")]
    pub pending_strategy_type: Option<u64>,
    #[serde(rename = "pendingPegPriceType", skip_serializing_if = "Option::is_none")]
    pub pending_peg_price_type: Option<String>,
    #[serde(rename = "pendingPegOffsetType", skip_serializing_if = "Option::is_none")]
    pub pending_peg_offset_type: Option<String>,
    #[serde(rename = "pendingPegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub pending_peg_offset_value: Option<u64>,
    #[serde(rename = "workingQuoteNotionalLimit", skip_serializing_if = "Option::is_none")]
    pub working_quote_notional_limit: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOpocoRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<String>,
    #[serde(rename = "selfTradePreventionMode", skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<String>,
    #[serde(rename = "workingType")]
    pub working_type: String,
    #[serde(rename = "workingSide")]
    pub working_side: String,
    #[serde(rename = "workingClientOrderId", skip_serializing_if = "Option::is_none")]
    pub working_client_order_id: Option<String>,
    #[serde(rename = "workingPrice")]
    pub working_price: String,
    #[serde(rename = "workingQuantity")]
    pub working_quantity: String,
    #[serde(rename = "workingIcebergQty", skip_serializing_if = "Option::is_none")]
    pub working_iceberg_qty: Option<String>,
    #[serde(rename = "workingTimeInForce", skip_serializing_if = "Option::is_none")]
    pub working_time_in_force: Option<String>,
    #[serde(rename = "workingStrategyId", skip_serializing_if = "Option::is_none")]
    pub working_strategy_id: Option<u64>,
    #[serde(rename = "workingStrategyType", skip_serializing_if = "Option::is_none")]
    pub working_strategy_type: Option<u64>,
    #[serde(rename = "workingPegPriceType", skip_serializing_if = "Option::is_none")]
    pub working_peg_price_type: Option<String>,
    #[serde(rename = "workingPegOffsetType", skip_serializing_if = "Option::is_none")]
    pub working_peg_offset_type: Option<String>,
    #[serde(rename = "workingPegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub working_peg_offset_value: Option<u64>,
    #[serde(rename = "pendingSide")]
    pub pending_side: String,
    #[serde(rename = "pendingAboveType")]
    pub pending_above_type: String,
    #[serde(rename = "pendingAboveClientOrderId", skip_serializing_if = "Option::is_none")]
    pub pending_above_client_order_id: Option<String>,
    #[serde(rename = "pendingAbovePrice", skip_serializing_if = "Option::is_none")]
    pub pending_above_price: Option<String>,
    #[serde(rename = "pendingAboveStopPrice", skip_serializing_if = "Option::is_none")]
    pub pending_above_stop_price: Option<String>,
    #[serde(rename = "pendingAboveTrailingDelta", skip_serializing_if = "Option::is_none")]
    pub pending_above_trailing_delta: Option<u64>,
    #[serde(rename = "pendingAboveIcebergQty", skip_serializing_if = "Option::is_none")]
    pub pending_above_iceberg_qty: Option<String>,
    #[serde(rename = "pendingAboveTimeInForce", skip_serializing_if = "Option::is_none")]
    pub pending_above_time_in_force: Option<String>,
    #[serde(rename = "pendingAboveStrategyId", skip_serializing_if = "Option::is_none")]
    pub pending_above_strategy_id: Option<u64>,
    #[serde(rename = "pendingAboveStrategyType", skip_serializing_if = "Option::is_none")]
    pub pending_above_strategy_type: Option<u64>,
    #[serde(rename = "pendingAbovePegPriceType", skip_serializing_if = "Option::is_none")]
    pub pending_above_peg_price_type: Option<String>,
    #[serde(rename = "pendingAbovePegOffsetType", skip_serializing_if = "Option::is_none")]
    pub pending_above_peg_offset_type: Option<String>,
    #[serde(rename = "pendingAbovePegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub pending_above_peg_offset_value: Option<u64>,
    #[serde(rename = "pendingBelowType", skip_serializing_if = "Option::is_none")]
    pub pending_below_type: Option<String>,
    #[serde(rename = "pendingBelowClientOrderId", skip_serializing_if = "Option::is_none")]
    pub pending_below_client_order_id: Option<String>,
    #[serde(rename = "pendingBelowPrice", skip_serializing_if = "Option::is_none")]
    pub pending_below_price: Option<String>,
    #[serde(rename = "pendingBelowStopPrice", skip_serializing_if = "Option::is_none")]
    pub pending_below_stop_price: Option<String>,
    #[serde(rename = "pendingBelowTrailingDelta", skip_serializing_if = "Option::is_none")]
    pub pending_below_trailing_delta: Option<u64>,
    #[serde(rename = "pendingBelowIcebergQty", skip_serializing_if = "Option::is_none")]
    pub pending_below_iceberg_qty: Option<String>,
    #[serde(rename = "pendingBelowTimeInForce", skip_serializing_if = "Option::is_none")]
    pub pending_below_time_in_force: Option<String>,
    #[serde(rename = "pendingBelowStrategyId", skip_serializing_if = "Option::is_none")]
    pub pending_below_strategy_id: Option<u64>,
    #[serde(rename = "pendingBelowStrategyType", skip_serializing_if = "Option::is_none")]
    pub pending_below_strategy_type: Option<u64>,
    #[serde(rename = "pendingBelowPegPriceType", skip_serializing_if = "Option::is_none")]
    pub pending_below_peg_price_type: Option<String>,
    #[serde(rename = "pendingBelowPegOffsetType", skip_serializing_if = "Option::is_none")]
    pub pending_below_peg_offset_type: Option<String>,
    #[serde(rename = "pendingBelowPegOffsetValue", skip_serializing_if = "Option::is_none")]
    pub pending_below_peg_offset_value: Option<u64>,
    #[serde(rename = "workingQuoteNotionalLimit", skip_serializing_if = "Option::is_none")]
    pub working_quote_notional_limit: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderListRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "orderListId", skip_serializing_if = "Option::is_none")]
    pub order_list_id: Option<u64>,
    #[serde(rename = "listClientOrderId", skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderListStatusRequest {
    #[serde(rename = "orderListId", skip_serializing_if = "Option::is_none")]
    pub order_list_id: Option<u64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderListHistoryRequest {
    #[serde(rename = "fromId", skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrderListsRequest {
    #[serde(rename = "recvWindow", skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedBinanceSpotClient(BinanceSpotClient);
impl BinanceSpotClient {
    pub fn typed(self) -> TypedBinanceSpotClient { TypedBinanceSpotClient(self) }
}
impl TypedBinanceSpotClient {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn agg_trades(&self, request: &AggTradesRequest) -> Result<AggTradesResponse> {
        serde_json::from_value(self.0.agg_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn cancel_all_orders(&self, request: &CancelAllOrdersRequest) -> Result<CancelAllOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn cancel_order_list(&self, request: &CancelOrderListRequest) -> Result<CancelOrderListResponse> {
        serde_json::from_value(self.0.cancel_order_list(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn cancel_replace_order(&self, request: &CancelReplaceOrderRequest) -> Result<CancelReplaceOrderResponse> {
        serde_json::from_value(self.0.cancel_replace_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn depth(&self, request: &DepthRequest) -> Result<DepthResponse> {
        serde_json::from_value(self.0.depth(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn open_order_lists(&self, request: &OpenOrderListsRequest) -> Result<OpenOrderListsResponse> {
        serde_json::from_value(self.0.open_order_lists(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn order_list_history(&self, request: &OrderListHistoryRequest) -> Result<OrderListHistoryResponse> {
        serde_json::from_value(self.0.order_list_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn order_list_status(&self, request: &OrderListStatusRequest) -> Result<OrderListStatusResponse> {
        serde_json::from_value(self.0.order_list_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn place_oco(&self, request: &PlaceOcoRequest) -> Result<PlaceOcoResponse> {
        serde_json::from_value(self.0.place_oco(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn place_opo(&self, request: &PlaceOpoRequest) -> Result<PlaceOpoResponse> {
        serde_json::from_value(self.0.place_opo(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn place_opoco(&self, request: &PlaceOpocoRequest) -> Result<PlaceOpocoResponse> {
        serde_json::from_value(self.0.place_opoco(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn place_oto(&self, request: &PlaceOtoRequest) -> Result<PlaceOtoResponse> {
        serde_json::from_value(self.0.place_oto(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn place_otoco(&self, request: &PlaceOtocoRequest) -> Result<PlaceOtocoResponse> {
        serde_json::from_value(self.0.place_otoco(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn ticker(&self, request: &TickerRequest) -> Result<TickerResponse> {
        serde_json::from_value(self.0.ticker(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedBinanceSpotClient(crate::AsyncClient<BinanceSpotClient>);
#[cfg(feature = "async")]
impl TypedBinanceSpotClient {
    pub fn into_async(self) -> AsyncTypedBinanceSpotClient { AsyncTypedBinanceSpotClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedBinanceSpotClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn agg_trades(&self, request: &AggTradesRequest) -> Result<AggTradesResponse> {
        serde_json::from_value(self.0.agg_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn cancel_all_orders(&self, request: &CancelAllOrdersRequest) -> Result<CancelAllOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn cancel_order_list(&self, request: &CancelOrderListRequest) -> Result<CancelOrderListResponse> {
        serde_json::from_value(self.0.cancel_order_list(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn cancel_replace_order(&self, request: &CancelReplaceOrderRequest) -> Result<CancelReplaceOrderResponse> {
        serde_json::from_value(self.0.cancel_replace_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn depth(&self, request: &DepthRequest) -> Result<DepthResponse> {
        serde_json::from_value(self.0.depth(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn open_order_lists(&self, request: &OpenOrderListsRequest) -> Result<OpenOrderListsResponse> {
        serde_json::from_value(self.0.open_order_lists(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn order_list_history(&self, request: &OrderListHistoryRequest) -> Result<OrderListHistoryResponse> {
        serde_json::from_value(self.0.order_list_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn order_list_status(&self, request: &OrderListStatusRequest) -> Result<OrderListStatusResponse> {
        serde_json::from_value(self.0.order_list_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn place_oco(&self, request: &PlaceOcoRequest) -> Result<PlaceOcoResponse> {
        serde_json::from_value(self.0.place_oco(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn place_opo(&self, request: &PlaceOpoRequest) -> Result<PlaceOpoResponse> {
        serde_json::from_value(self.0.place_opo(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn place_opoco(&self, request: &PlaceOpocoRequest) -> Result<PlaceOpocoResponse> {
        serde_json::from_value(self.0.place_opoco(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn place_oto(&self, request: &PlaceOtoRequest) -> Result<PlaceOtoResponse> {
        serde_json::from_value(self.0.place_oto(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn place_otoco(&self, request: &PlaceOtocoRequest) -> Result<PlaceOtocoResponse> {
        serde_json::from_value(self.0.place_otoco(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn ticker(&self, request: &TickerRequest) -> Result<TickerResponse> {
        serde_json::from_value(self.0.ticker(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid BinanceSpot response model"))
    }
}
