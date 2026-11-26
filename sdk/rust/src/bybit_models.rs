// Generated from src/exchanges/bybit/schema/bybit.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{BybitClient, Error, Result};

pub type MarketsResponse = serde_json::Value;
pub type TickerResponse = serde_json::Value;
pub type TradesResponse = serde_json::Value;
pub type AccountResponse = serde_json::Value;
pub type BalancesResponse = serde_json::Value;
pub type PositionsResponse = serde_json::Value;
pub type OrderStatusResponse = serde_json::Value;
pub type OpenOrdersResponse = serde_json::Value;
pub type OrderHistoryResponse = serde_json::Value;
pub type FillsResponse = serde_json::Value;
pub type PlaceOrderResponse = serde_json::Value;
pub type AmendOrderResponse = serde_json::Value;
pub type CancelOrderResponse = serde_json::Value;
pub type PlaceOrdersResponse = serde_json::Value;
pub type AmendOrdersResponse = serde_json::Value;
pub type CancelOrdersResponse = serde_json::Value;
pub type CancelAllOrdersResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<u64>,
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "baseCoin", skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TickerRequest {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "baseCoin", skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(rename = "expDate", skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "baseCoin", skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(rename = "optionType", skip_serializing_if = "Option::is_none")]
    pub option_type: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BalancesRequest {
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "baseCoin", skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(rename = "settleCoin", skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "orderLinkId", skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "orderFilter", skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "baseCoin", skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(rename = "settleCoin", skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    #[serde(rename = "orderFilter", skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    #[serde(rename = "openOnly", skip_serializing_if = "Option::is_none")]
    pub open_only: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryRequest {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "baseCoin", skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(rename = "settleCoin", skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "orderLinkId", skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(rename = "orderFilter", skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    #[serde(rename = "orderStatus", skip_serializing_if = "Option::is_none")]
    pub order_status: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "orderLinkId", skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    #[serde(rename = "baseCoin", skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(rename = "execType", skip_serializing_if = "Option::is_none")]
    pub exec_type: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendOrderRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrdersRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendOrdersRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAllOrdersRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedBybitClient(BybitClient);
impl BybitClient {
    pub fn typed(self) -> TypedBybitClient { TypedBybitClient(self) }
}
impl TypedBybitClient {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn amend_orders(&self, request: &AmendOrdersRequest) -> Result<AmendOrdersResponse> {
        serde_json::from_value(self.0.amend_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn balances(&self, request: &BalancesRequest) -> Result<BalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn cancel_all_orders(&self, request: &CancelAllOrdersRequest) -> Result<CancelAllOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn cancel_orders(&self, request: &CancelOrdersRequest) -> Result<CancelOrdersResponse> {
        serde_json::from_value(self.0.cancel_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn place_orders(&self, request: &PlaceOrdersRequest) -> Result<PlaceOrdersResponse> {
        serde_json::from_value(self.0.place_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn ticker(&self, request: &TickerRequest) -> Result<TickerResponse> {
        serde_json::from_value(self.0.ticker(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedBybitClient(crate::AsyncClient<BybitClient>);
#[cfg(feature = "async")]
impl TypedBybitClient {
    pub fn into_async(self) -> AsyncTypedBybitClient { AsyncTypedBybitClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedBybitClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn amend_orders(&self, request: &AmendOrdersRequest) -> Result<AmendOrdersResponse> {
        serde_json::from_value(self.0.amend_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn balances(&self, request: &BalancesRequest) -> Result<BalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn cancel_all_orders(&self, request: &CancelAllOrdersRequest) -> Result<CancelAllOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn cancel_orders(&self, request: &CancelOrdersRequest) -> Result<CancelOrdersResponse> {
        serde_json::from_value(self.0.cancel_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn place_orders(&self, request: &PlaceOrdersRequest) -> Result<PlaceOrdersResponse> {
        serde_json::from_value(self.0.place_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn ticker(&self, request: &TickerRequest) -> Result<TickerResponse> {
        serde_json::from_value(self.0.ticker(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Bybit response model"))
    }
}
