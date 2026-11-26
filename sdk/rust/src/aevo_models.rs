// Generated from src/exchanges/aevo/schema/aevo.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{AevoClient, Error, Result};

pub type MarketsResponse = serde_json::Value;
pub type IndexResponse = serde_json::Value;
pub type StatisticsResponse = serde_json::Value;
pub type AccountResponse = serde_json::Value;
pub type BalancesResponse = serde_json::Value;
pub type OpenOrdersResponse = serde_json::Value;
pub type PlaceOrderResponse = serde_json::Value;
pub type CancelOrderResponse = serde_json::Value;
pub type CancelAllOrdersResponse = serde_json::Value;
pub type PositionsResponse = serde_json::Value;
pub type OrderHistoryResponse = serde_json::Value;
pub type FillsResponse = serde_json::Value;
pub type OrderStatusResponse = serde_json::Value;
pub type TradesResponse = serde_json::Value;
pub type AmendOrderResponse = serde_json::Value;
pub type CancelOrdersResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "instrument_type", skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "instrument_type", skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct IndexRequest {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct StatisticsRequest {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
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
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
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
pub struct CancelOrderRequest {
    #[serde(rename = "order_id")]
    pub order_id: String,
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

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "instrument_name", skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryRequest {
    #[serde(rename = "instrument_name", skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    #[serde(rename = "instrument_type", skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "instrument_name", skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    #[serde(rename = "instrument_type", skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
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
pub struct TradesRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendOrderRequest {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersRequest {
    #[serde(rename = "body")]
    pub body: CancelOrdersBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersBody {
    #[serde(rename = "order_ids")]
    pub order_ids: Vec<String>,
    #[serde(rename = "instrument_type", skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedAevoClient(AevoClient);
impl AevoClient {
    pub fn typed(self) -> TypedAevoClient { TypedAevoClient(self) }
}
impl TypedAevoClient {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn balances(&self, request: &BalancesRequest) -> Result<BalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn cancel_all_orders(&self, request: &CancelAllOrdersRequest) -> Result<CancelAllOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn cancel_orders(&self, request: &CancelOrdersRequest) -> Result<CancelOrdersResponse> {
        serde_json::from_value(self.0.cancel_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn index(&self, request: &IndexRequest) -> Result<IndexResponse> {
        serde_json::from_value(self.0.index(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn statistics(&self, request: &StatisticsRequest) -> Result<StatisticsResponse> {
        serde_json::from_value(self.0.statistics(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedAevoClient(crate::AsyncClient<AevoClient>);
#[cfg(feature = "async")]
impl TypedAevoClient {
    pub fn into_async(self) -> AsyncTypedAevoClient { AsyncTypedAevoClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedAevoClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn balances(&self, request: &BalancesRequest) -> Result<BalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn cancel_all_orders(&self, request: &CancelAllOrdersRequest) -> Result<CancelAllOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn cancel_orders(&self, request: &CancelOrdersRequest) -> Result<CancelOrdersResponse> {
        serde_json::from_value(self.0.cancel_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn index(&self, request: &IndexRequest) -> Result<IndexResponse> {
        serde_json::from_value(self.0.index(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn statistics(&self, request: &StatisticsRequest) -> Result<StatisticsResponse> {
        serde_json::from_value(self.0.statistics(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aevo response model"))
    }
}
