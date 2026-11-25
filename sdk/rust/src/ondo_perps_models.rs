// Generated from src/exchanges/ondo_perps/schema/ondo_perps.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{OndoPerpsClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(rename = "sparkline", skip_serializing_if = "Option::is_none")]
    pub sparkline: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookRequest {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "orderID")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    #[serde(rename = "body")]
    pub body: Order,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "orderID")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "markets", skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<String>>,
    #[serde(rename = "numPastTrades", skip_serializing_if = "Option::is_none")]
    pub num_past_trades: Option<u64>,
    #[serde(rename = "depthLevels", skip_serializing_if = "Option::is_none")]
    pub depth_levels: Option<serde_json::Value>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "postOnly", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "builderCode", skip_serializing_if = "Option::is_none")]
    pub builder_code: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OndoReply {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedOndoPerpsClient(OndoPerpsClient);
impl OndoPerpsClient {
    pub fn typed(self) -> TypedOndoPerpsClient { TypedOndoPerpsClient(self) }
}
impl TypedOndoPerpsClient {
    pub fn account(&self, request: &AccountRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedOndoPerpsClient(crate::AsyncClient<OndoPerpsClient>);
#[cfg(feature = "async")]
impl TypedOndoPerpsClient {
    pub fn into_async(self) -> AsyncTypedOndoPerpsClient { AsyncTypedOndoPerpsClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedOndoPerpsClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<OndoReply> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid OndoPerps response model"))
    }
}
