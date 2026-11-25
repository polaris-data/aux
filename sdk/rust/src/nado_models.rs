// Generated from src/exchanges/nado/schema/nado.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{NadoClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookRequest {
    #[serde(rename = "product_id")]
    pub product_id: u64,
    #[serde(rename = "depth")]
    pub depth: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "ticker_id")]
    pub ticker_id: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(rename = "subaccount")]
    pub subaccount: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "subaccount")]
    pub subaccount: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "sender")]
    pub sender: String,
    #[serde(rename = "product_id")]
    pub product_id: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ContractsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    #[serde(rename = "body")]
    pub body: PlaceOrder,
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
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "subaccount", skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<String>,
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<u32>,
    #[serde(rename = "snapshot_timestamp", skip_serializing_if = "Option::is_none")]
    pub snapshot_timestamp: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrder {
    #[serde(rename = "product_id")]
    pub product_id: u32,
    #[serde(rename = "order")]
    pub order: Order,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrder {
    #[serde(rename = "tx")]
    pub tx: Cancellation,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GatewayReply {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(rename = "request_type", skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "sender")]
    pub sender: String,
    #[serde(rename = "priceX18")]
    pub price_x18: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "expiration")]
    pub expiration: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(rename = "appendix")]
    pub appendix: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Cancellation {
    #[serde(rename = "sender")]
    pub sender: String,
    #[serde(rename = "productIds")]
    pub product_ids: Vec<u32>,
    #[serde(rename = "digests")]
    pub digests: Vec<String>,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedNadoClient(NadoClient);
impl NadoClient {
    pub fn typed(self) -> TypedNadoClient { TypedNadoClient(self) }
}
impl TypedNadoClient {
    pub fn account(&self, request: &AccountRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub fn contracts(&self, request: &ContractsRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.contracts(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<Vec<serde_json::Value>> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedNadoClient(crate::AsyncClient<NadoClient>);
#[cfg(feature = "async")]
impl TypedNadoClient {
    pub fn into_async(self) -> AsyncTypedNadoClient { AsyncTypedNadoClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedNadoClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub async fn contracts(&self, request: &ContractsRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.contracts(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<GatewayReply> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<Vec<serde_json::Value>> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Nado response model"))
    }
}
