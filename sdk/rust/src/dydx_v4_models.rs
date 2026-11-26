// Generated from src/exchanges/dydx_v4/schema/dydx_v4.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{DydxV4Client, Error, Result};

pub type MarketsResponse = serde_json::Value;
pub type TradesResponse = serde_json::Value;
pub type AccountResponse = serde_json::Value;
pub type BalancesResponse = serde_json::Value;
pub type PositionsResponse = serde_json::Value;
pub type OrderStatusResponse = serde_json::Value;
pub type OpenOrdersResponse = serde_json::Value;
pub type OrderHistoryResponse = serde_json::Value;
pub type FillsResponse = serde_json::Value;
pub type TransactionResponse = serde_json::Value;
pub type PlaceOrderResponse = serde_json::Value;
pub type ReplaceOrderResponse = serde_json::Value;
pub type CancelOrderResponse = serde_json::Value;
pub type CancelOrdersResponse = serde_json::Value;
pub type PlaceOrdersResponse = serde_json::Value;
pub type CancelStatefulOrdersResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "subaccount_number", skip_serializing_if = "Option::is_none")]
    pub subaccount_number: Option<u64>,
    #[serde(rename = "batched", skip_serializing_if = "Option::is_none")]
    pub batched: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "createdBeforeOrAtHeight", skip_serializing_if = "Option::is_none")]
    pub created_before_or_at_height: Option<u64>,
    #[serde(rename = "createdBeforeOrAt", skip_serializing_if = "Option::is_none")]
    pub created_before_or_at: Option<String>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "subaccountNumber")]
    pub subaccount_number: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BalancesRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "subaccountNumber")]
    pub subaccount_number: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "subaccountNumber")]
    pub subaccount_number: u64,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "createdBeforeOrAtHeight", skip_serializing_if = "Option::is_none")]
    pub created_before_or_at_height: Option<u64>,
    #[serde(rename = "createdBeforeOrAt", skip_serializing_if = "Option::is_none")]
    pub created_before_or_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "subaccountNumber")]
    pub subaccount_number: u64,
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "returnLatestOrders", skip_serializing_if = "Option::is_none")]
    pub return_latest_orders: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "subaccountNumber")]
    pub subaccount_number: u64,
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "includeTypes", skip_serializing_if = "Option::is_none")]
    pub include_types: Option<Vec<String>>,
    #[serde(rename = "excludeTypes", skip_serializing_if = "Option::is_none")]
    pub exclude_types: Option<Vec<String>>,
    #[serde(rename = "goodTilBlockBeforeOrAt", skip_serializing_if = "Option::is_none")]
    pub good_til_block_before_or_at: Option<u64>,
    #[serde(rename = "goodTilBlockAfter", skip_serializing_if = "Option::is_none")]
    pub good_til_block_after: Option<u64>,
    #[serde(rename = "goodTilBlockTimeBeforeOrAt", skip_serializing_if = "Option::is_none")]
    pub good_til_block_time_before_or_at: Option<String>,
    #[serde(rename = "goodTilBlockTimeAfter", skip_serializing_if = "Option::is_none")]
    pub good_til_block_time_after: Option<String>,
    #[serde(rename = "returnLatestOrders", skip_serializing_if = "Option::is_none")]
    pub return_latest_orders: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "subaccountNumber")]
    pub subaccount_number: u64,
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "marketType", skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(rename = "includeTypes", skip_serializing_if = "Option::is_none")]
    pub include_types: Option<Vec<String>>,
    #[serde(rename = "excludeTypes", skip_serializing_if = "Option::is_none")]
    pub exclude_types: Option<Vec<String>>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "createdBeforeOrAtHeight", skip_serializing_if = "Option::is_none")]
    pub created_before_or_at_height: Option<u64>,
    #[serde(rename = "createdBeforeOrAt", skip_serializing_if = "Option::is_none")]
    pub created_before_or_at: Option<String>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransactionRequest {
    #[serde(rename = "hash")]
    pub hash: String,
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
pub struct ReplaceOrderRequest {
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
pub struct CancelOrdersRequest {
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
pub struct CancelStatefulOrdersRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedDydxV4Client(DydxV4Client);
impl DydxV4Client {
    pub fn typed(self) -> TypedDydxV4Client { TypedDydxV4Client(self) }
}
impl TypedDydxV4Client {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn balances(&self, request: &BalancesRequest) -> Result<BalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn cancel_orders(&self, request: &CancelOrdersRequest) -> Result<CancelOrdersResponse> {
        serde_json::from_value(self.0.cancel_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn cancel_stateful_orders(&self, request: &CancelStatefulOrdersRequest) -> Result<CancelStatefulOrdersResponse> {
        serde_json::from_value(self.0.cancel_stateful_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn place_orders(&self, request: &PlaceOrdersRequest) -> Result<PlaceOrdersResponse> {
        serde_json::from_value(self.0.place_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn replace_order(&self, request: &ReplaceOrderRequest) -> Result<ReplaceOrderResponse> {
        serde_json::from_value(self.0.replace_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub fn transaction(&self, request: &TransactionRequest) -> Result<TransactionResponse> {
        serde_json::from_value(self.0.transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedDydxV4Client(crate::AsyncClient<DydxV4Client>);
#[cfg(feature = "async")]
impl TypedDydxV4Client {
    pub fn into_async(self) -> AsyncTypedDydxV4Client { AsyncTypedDydxV4Client(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedDydxV4Client {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn balances(&self, request: &BalancesRequest) -> Result<BalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn cancel_orders(&self, request: &CancelOrdersRequest) -> Result<CancelOrdersResponse> {
        serde_json::from_value(self.0.cancel_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn cancel_stateful_orders(&self, request: &CancelStatefulOrdersRequest) -> Result<CancelStatefulOrdersResponse> {
        serde_json::from_value(self.0.cancel_stateful_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn place_orders(&self, request: &PlaceOrdersRequest) -> Result<PlaceOrdersResponse> {
        serde_json::from_value(self.0.place_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn replace_order(&self, request: &ReplaceOrderRequest) -> Result<ReplaceOrderResponse> {
        serde_json::from_value(self.0.replace_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
    pub async fn transaction(&self, request: &TransactionRequest) -> Result<TransactionResponse> {
        serde_json::from_value(self.0.transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DydxV4 response model"))
    }
}
