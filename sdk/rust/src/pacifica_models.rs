// Generated from src/exchanges/pacifica/schema/pacifica.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{PacificaClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "agg_level", skip_serializing_if = "Option::is_none")]
    pub agg_level: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "order_id")]
    pub order_id: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    #[serde(rename = "body")]
    pub body: LimitOrder,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceMarketOrderRequest {
    #[serde(rename = "body")]
    pub body: MarketOrder,
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
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "agg_level", skip_serializing_if = "Option::is_none")]
    pub agg_level: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LimitOrder {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "reduce_only", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "client_order_id", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "tif")]
    pub tif: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketOrder {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "reduce_only", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "client_order_id", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "slippage_percent")]
    pub slippage_percent: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrder {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    #[serde(rename = "client_order_id", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PacificaReply {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<serde_json::Number>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedPacificaClient(PacificaClient);
impl PacificaClient {
    pub fn typed(self) -> TypedPacificaClient { TypedPacificaClient(self) }
}
impl TypedPacificaClient {
    pub fn account(&self, request: &AccountRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub fn place_market_order(&self, request: &PlaceMarketOrderRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.place_market_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedPacificaClient(crate::AsyncClient<PacificaClient>);
#[cfg(feature = "async")]
impl TypedPacificaClient {
    pub fn into_async(self) -> AsyncTypedPacificaClient { AsyncTypedPacificaClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedPacificaClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub async fn place_market_order(&self, request: &PlaceMarketOrderRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.place_market_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<PacificaReply> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pacifica response model"))
    }
}
