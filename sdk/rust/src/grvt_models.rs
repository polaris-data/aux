// Generated from src/exchanges/grvt/schema/grvt.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{GrvtClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
    #[serde(rename = "sub_account_id", skip_serializing_if = "Option::is_none")]
    pub sub_account_id: Option<String>,
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<u64>,
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct InstrumentRequest {
    #[serde(rename = "instrument")]
    pub instrument: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct InstrumentResponseResult {
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
    #[serde(rename = "instrument_hash", skip_serializing_if = "Option::is_none")]
    pub instrument_hash: Option<String>,
    #[serde(rename = "base_decimals", skip_serializing_if = "Option::is_none")]
    pub base_decimals: Option<i8>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "tick_size", skip_serializing_if = "Option::is_none")]
    pub tick_size: Option<String>,
    #[serde(rename = "min_size", skip_serializing_if = "Option::is_none")]
    pub min_size: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct InstrumentResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<InstrumentResponseResult>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsBody {
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(rename = "body")]
    pub body: MarketsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<InstrumentResponseResult>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookRequest {
    #[serde(rename = "instrument")]
    pub instrument: String,
    #[serde(rename = "depth")]
    pub depth: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookResponseResult {
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
    #[serde(rename = "bids", skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<serde_json::Value>>,
    #[serde(rename = "asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<OrderBookResponseResult>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "instrument")]
    pub instrument: String,
    #[serde(rename = "limit")]
    pub limit: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LoginRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "sub_account_id", skip_serializing_if = "Option::is_none")]
    pub sub_account_id: Option<String>,
    #[serde(rename = "cookie")]
    pub cookie: String,
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(rename = "sub_account_id")]
    pub sub_account_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountResponseResult {
    #[serde(rename = "sub_account_id", skip_serializing_if = "Option::is_none")]
    pub sub_account_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<AccountResponseResult>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "sub_account_id")]
    pub sub_account_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "sub_account_id")]
    pub sub_account_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "sub_account_id")]
    pub sub_account_id: String,
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "client_order_id", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusResponseResult {
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<OrderStatusResponseResult>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "sub_account_id")]
    pub sub_account_id: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderBody {
    #[serde(rename = "sub_account_id")]
    pub sub_account_id: String,
    #[serde(rename = "is_market")]
    pub is_market: bool,
    #[serde(rename = "time_in_force", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "post_only", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "legs")]
    pub legs: Vec<OrderLeg>,
    #[serde(rename = "metadata")]
    pub metadata: OrderMetadata,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    #[serde(rename = "body")]
    pub body: PlaceOrderBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderResponseResult {
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<PlaceOrderResponseResult>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderBody {
    #[serde(rename = "sub_account_id")]
    pub sub_account_id: String,
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "client_order_id", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "body")]
    pub body: CancelOrderBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderResponseResult {
    #[serde(rename = "ack", skip_serializing_if = "Option::is_none")]
    pub ack: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<CancelOrderResponseResult>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderLeg {
    #[serde(rename = "instrument")]
    pub instrument: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "limit_price")]
    pub limit_price: String,
    #[serde(rename = "is_buying_asset")]
    pub is_buying_asset: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderMetadata {
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedGrvtClient(GrvtClient);
impl GrvtClient {
    pub fn typed(self) -> TypedGrvtClient { TypedGrvtClient(self) }
}
impl TypedGrvtClient {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub fn instrument(&self, request: &InstrumentRequest) -> Result<InstrumentResponse> {
        serde_json::from_value(self.0.instrument(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub fn login(&self, request: &LoginRequest) -> Result<LoginResponse> {
        serde_json::from_value(self.0.login(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBookResponse> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedGrvtClient(crate::AsyncClient<GrvtClient>);
#[cfg(feature = "async")]
impl TypedGrvtClient {
    pub fn into_async(self) -> AsyncTypedGrvtClient { AsyncTypedGrvtClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedGrvtClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub async fn instrument(&self, request: &InstrumentRequest) -> Result<InstrumentResponse> {
        serde_json::from_value(self.0.instrument(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub async fn login(&self, request: &LoginRequest) -> Result<LoginResponse> {
        serde_json::from_value(self.0.login(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBookResponse> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Grvt response model"))
    }
}
