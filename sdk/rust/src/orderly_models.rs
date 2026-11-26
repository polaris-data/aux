// Generated from src/exchanges/orderly/schema/orderly.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{OrderlyClient, Error, Result};

fn present<'de,D:serde::Deserializer<'de>,T:Deserialize<'de>>(d:D) -> std::result::Result<Option<T>,D::Error> { Deserialize::deserialize(d) }
fn optional_nullable<'de,D:serde::Deserializer<'de>,T:Deserialize<'de>>(d:D) -> std::result::Result<Option<Option<T>>,D::Error> { present(d).map(Some) }
pub type MarketsResponse = serde_json::Value;
pub type TickerResponse = serde_json::Value;
pub type TradesResponse = serde_json::Value;
pub type AccountResponse = serde_json::Value;
pub type BalancesResponse = serde_json::Value;
pub type PositionsResponse = serde_json::Value;
pub type OrderStatusResponse = serde_json::Value;
pub type OrderByClientIdResponse = serde_json::Value;
pub type OpenOrdersResponse = serde_json::Value;
pub type OrderHistoryResponse = serde_json::Value;
pub type FillsResponse = serde_json::Value;
pub type PlaceOrderResponse = serde_json::Value;
pub type AmendOrderResponse = serde_json::Value;
pub type CancelOrderResponse = serde_json::Value;
pub type CancelByClientIdResponse = serde_json::Value;
pub type PlaceOrdersResponse = serde_json::Value;
pub type CancelOrdersResponse = serde_json::Value;
pub type CancelOrdersByClientIdResponse = serde_json::Value;
pub type CancelAllOrdersResponse = serde_json::Value;
pub type PlaceAlgoOrderResponse = serde_json::Value;
pub type AmendAlgoOrderResponse = serde_json::Value;
pub type CancelAlgoOrderResponse = serde_json::Value;
pub type CancelAlgoByClientIdResponse = serde_json::Value;
pub type CancelAllAlgoOrdersResponse = serde_json::Value;
pub type AlgoOrderStatusResponse = serde_json::Value;
pub type AlgoOrderByClientIdResponse = serde_json::Value;
pub type AlgoOrdersResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSummaryBody {
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketTradesBody {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TickerRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
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
pub struct MarketSummaryRequest {
    #[serde(rename = "body")]
    pub body: MarketSummaryBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketTradesRequest {
    #[serde(rename = "body")]
    pub body: MarketTradesBody,
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
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderByClientIdRequest {
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "order_type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "order_type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "start_t", skip_serializing_if = "Option::is_none")]
    pub start_t: Option<u64>,
    #[serde(rename = "end_t", skip_serializing_if = "Option::is_none")]
    pub end_t: Option<u64>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "start_t", skip_serializing_if = "Option::is_none")]
    pub start_t: Option<u64>,
    #[serde(rename = "end_t", skip_serializing_if = "Option::is_none")]
    pub end_t: Option<u64>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
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
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "client_order_id", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelByClientIdRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
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
pub struct CancelOrdersRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "order_ids")]
    pub order_ids: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersByClientIdRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "client_order_ids")]
    pub client_order_ids: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAllOrdersRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAlgoOrderRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendAlgoOrderRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAlgoOrderRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAlgoByClientIdRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAllAlgoOrdersRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AlgoOrderStatusRequest {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AlgoOrderByClientIdRequest {
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AlgoOrdersRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSummaryResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "data")]
    pub data: MarketSummaryData,
    #[serde(rename = "ts")]
    pub ts: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSummaryData {
    #[serde(rename = "total_24h_volume", default, deserialize_with = "optional_nullable", skip_serializing_if = "Option::is_none")]
    pub total_24h_volume: Option<Option<String>>,
    #[serde(rename = "total_open_interest", default, deserialize_with = "optional_nullable", skip_serializing_if = "Option::is_none")]
    pub total_open_interest: Option<Option<String>>,
    #[serde(rename = "markets")]
    pub markets: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketTradesResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "data")]
    pub data: MarketTradesData,
    #[serde(rename = "ts")]
    pub ts: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketTradesData {
    #[serde(rename = "rows")]
    pub rows: Vec<serde_json::Value>,
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedOrderlyClient(OrderlyClient);
impl OrderlyClient {
    pub fn typed(self) -> TypedOrderlyClient { TypedOrderlyClient(self) }
}
impl TypedOrderlyClient {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn algo_order_by_client_id(&self, request: &AlgoOrderByClientIdRequest) -> Result<AlgoOrderByClientIdResponse> {
        serde_json::from_value(self.0.algo_order_by_client_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn algo_order_status(&self, request: &AlgoOrderStatusRequest) -> Result<AlgoOrderStatusResponse> {
        serde_json::from_value(self.0.algo_order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn algo_orders(&self, request: &AlgoOrdersRequest) -> Result<AlgoOrdersResponse> {
        serde_json::from_value(self.0.algo_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn amend_algo_order(&self, request: &AmendAlgoOrderRequest) -> Result<AmendAlgoOrderResponse> {
        serde_json::from_value(self.0.amend_algo_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn balances(&self, request: &BalancesRequest) -> Result<BalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn cancel_algo_by_client_id(&self, request: &CancelAlgoByClientIdRequest) -> Result<CancelAlgoByClientIdResponse> {
        serde_json::from_value(self.0.cancel_algo_by_client_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn cancel_algo_order(&self, request: &CancelAlgoOrderRequest) -> Result<CancelAlgoOrderResponse> {
        serde_json::from_value(self.0.cancel_algo_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn cancel_all_algo_orders(&self, request: &CancelAllAlgoOrdersRequest) -> Result<CancelAllAlgoOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_algo_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn cancel_all_orders(&self, request: &CancelAllOrdersRequest) -> Result<CancelAllOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn cancel_by_client_id(&self, request: &CancelByClientIdRequest) -> Result<CancelByClientIdResponse> {
        serde_json::from_value(self.0.cancel_by_client_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn cancel_orders(&self, request: &CancelOrdersRequest) -> Result<CancelOrdersResponse> {
        serde_json::from_value(self.0.cancel_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn cancel_orders_by_client_id(&self, request: &CancelOrdersByClientIdRequest) -> Result<CancelOrdersByClientIdResponse> {
        serde_json::from_value(self.0.cancel_orders_by_client_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn market_summary(&self, request: &MarketSummaryRequest) -> Result<MarketSummaryResponse> {
        serde_json::from_value(self.0.market_summary(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn market_trades(&self, request: &MarketTradesRequest) -> Result<MarketTradesResponse> {
        serde_json::from_value(self.0.market_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn order_by_client_id(&self, request: &OrderByClientIdRequest) -> Result<OrderByClientIdResponse> {
        serde_json::from_value(self.0.order_by_client_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn place_algo_order(&self, request: &PlaceAlgoOrderRequest) -> Result<PlaceAlgoOrderResponse> {
        serde_json::from_value(self.0.place_algo_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn place_orders(&self, request: &PlaceOrdersRequest) -> Result<PlaceOrdersResponse> {
        serde_json::from_value(self.0.place_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn ticker(&self, request: &TickerRequest) -> Result<TickerResponse> {
        serde_json::from_value(self.0.ticker(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedOrderlyClient(crate::AsyncClient<OrderlyClient>);
#[cfg(feature = "async")]
impl TypedOrderlyClient {
    pub fn into_async(self) -> AsyncTypedOrderlyClient { AsyncTypedOrderlyClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedOrderlyClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn algo_order_by_client_id(&self, request: &AlgoOrderByClientIdRequest) -> Result<AlgoOrderByClientIdResponse> {
        serde_json::from_value(self.0.algo_order_by_client_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn algo_order_status(&self, request: &AlgoOrderStatusRequest) -> Result<AlgoOrderStatusResponse> {
        serde_json::from_value(self.0.algo_order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn algo_orders(&self, request: &AlgoOrdersRequest) -> Result<AlgoOrdersResponse> {
        serde_json::from_value(self.0.algo_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn amend_algo_order(&self, request: &AmendAlgoOrderRequest) -> Result<AmendAlgoOrderResponse> {
        serde_json::from_value(self.0.amend_algo_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn balances(&self, request: &BalancesRequest) -> Result<BalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn cancel_algo_by_client_id(&self, request: &CancelAlgoByClientIdRequest) -> Result<CancelAlgoByClientIdResponse> {
        serde_json::from_value(self.0.cancel_algo_by_client_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn cancel_algo_order(&self, request: &CancelAlgoOrderRequest) -> Result<CancelAlgoOrderResponse> {
        serde_json::from_value(self.0.cancel_algo_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn cancel_all_algo_orders(&self, request: &CancelAllAlgoOrdersRequest) -> Result<CancelAllAlgoOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_algo_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn cancel_all_orders(&self, request: &CancelAllOrdersRequest) -> Result<CancelAllOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn cancel_by_client_id(&self, request: &CancelByClientIdRequest) -> Result<CancelByClientIdResponse> {
        serde_json::from_value(self.0.cancel_by_client_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn cancel_orders(&self, request: &CancelOrdersRequest) -> Result<CancelOrdersResponse> {
        serde_json::from_value(self.0.cancel_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn cancel_orders_by_client_id(&self, request: &CancelOrdersByClientIdRequest) -> Result<CancelOrdersByClientIdResponse> {
        serde_json::from_value(self.0.cancel_orders_by_client_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn market_summary(&self, request: &MarketSummaryRequest) -> Result<MarketSummaryResponse> {
        serde_json::from_value(self.0.market_summary(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn market_trades(&self, request: &MarketTradesRequest) -> Result<MarketTradesResponse> {
        serde_json::from_value(self.0.market_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn order_by_client_id(&self, request: &OrderByClientIdRequest) -> Result<OrderByClientIdResponse> {
        serde_json::from_value(self.0.order_by_client_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn place_algo_order(&self, request: &PlaceAlgoOrderRequest) -> Result<PlaceAlgoOrderResponse> {
        serde_json::from_value(self.0.place_algo_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn place_orders(&self, request: &PlaceOrdersRequest) -> Result<PlaceOrdersResponse> {
        serde_json::from_value(self.0.place_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn ticker(&self, request: &TickerRequest) -> Result<TickerResponse> {
        serde_json::from_value(self.0.ticker(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Orderly response model"))
    }
}
