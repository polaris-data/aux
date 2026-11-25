// Generated from src/exchanges/aster/schema/aster.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{AsterClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
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
pub struct AccountRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
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
    #[serde(rename = "body")]
    pub body: CancelOrder,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<String>,
    #[serde(rename = "positionSide", skip_serializing_if = "Option::is_none")]
    pub position_side: Option<String>,
    #[serde(rename = "newClientOrderId", skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrder {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(rename = "origClientOrderId", skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Markets {
    #[serde(rename = "symbols")]
    pub symbols: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Orderbook {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    #[serde(rename = "bids")]
    pub bids: Vec<Vec<String>>,
    #[serde(rename = "asks")]
    pub asks: Vec<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "assets")]
    pub assets: Vec<serde_json::Value>,
    #[serde(rename = "positions")]
    pub positions: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderReply {
    #[serde(rename = "orderId")]
    pub order_id: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedAsterClient(AsterClient);
impl AsterClient {
    pub fn typed(self) -> TypedAsterClient { TypedAsterClient(self) }
}
impl TypedAsterClient {
    pub fn account(&self, request: &AccountRequest) -> Result<Account> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<OrderReply> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<Markets> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<Vec<serde_json::Value>> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<Orderbook> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderReply> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<OrderReply> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<Vec<serde_json::Value>> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<Vec<serde_json::Value>> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedAsterClient(crate::AsyncClient<AsterClient>);
#[cfg(feature = "async")]
impl TypedAsterClient {
    pub fn into_async(self) -> AsyncTypedAsterClient { AsyncTypedAsterClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedAsterClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<Account> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<OrderReply> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<Markets> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<Vec<serde_json::Value>> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<Orderbook> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderReply> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<OrderReply> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<Vec<serde_json::Value>> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<Vec<serde_json::Value>> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Aster response model"))
    }
}
