// Generated from src/exchanges/perpl/schema/perpl.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{PerplClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "market_id", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsResponseChain {
    #[serde(rename = "chain_id", skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsResponse {
    #[serde(rename = "markets", skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<Market>>,
    #[serde(rename = "tokens", skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<serde_json::Value>>,
    #[serde(rename = "chain", skip_serializing_if = "Option::is_none")]
    pub chain: Option<MarketsResponseChain>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookRequest {
    #[serde(rename = "market_id")]
    pub market_id: u64,
    #[serde(rename = "levels", skip_serializing_if = "Option::is_none")]
    pub levels: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookResponse {
    #[serde(rename = "mt")]
    pub mt: u64,
    #[serde(rename = "bid", skip_serializing_if = "Option::is_none")]
    pub bid: Option<Vec<PriceLevel>>,
    #[serde(rename = "ask", skip_serializing_if = "Option::is_none")]
    pub ask: Option<Vec<PriceLevel>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountResponse {
    #[serde(rename = "mt")]
    pub mt: u64,
    #[serde(rename = "sn", skip_serializing_if = "Option::is_none")]
    pub sn: Option<u64>,
    #[serde(rename = "addr", skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(rename = "as", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<serde_json::Value>>,
    #[serde(rename = "sts", skip_serializing_if = "Option::is_none")]
    pub sts: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersResponse {
    #[serde(rename = "mt")]
    pub mt: u64,
    #[serde(rename = "sn", skip_serializing_if = "Option::is_none")]
    pub sn: Option<u64>,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub d: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsResponse {
    #[serde(rename = "mt")]
    pub mt: u64,
    #[serde(rename = "sn", skip_serializing_if = "Option::is_none")]
    pub sn: Option<u64>,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub d: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsResponse {
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub d: Option<Vec<serde_json::Value>>,
    #[serde(rename = "np", skip_serializing_if = "Option::is_none")]
    pub np: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryRequest {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryResponse {
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub d: Option<Vec<serde_json::Value>>,
    #[serde(rename = "np", skip_serializing_if = "Option::is_none")]
    pub np: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderBody {
    #[serde(rename = "rq")]
    pub rq: u64,
    #[serde(rename = "mkt")]
    pub mkt: u64,
    #[serde(rename = "acc")]
    pub acc: u64,
    #[serde(rename = "t")]
    pub t: u64,
    #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
    pub p: Option<u64>,
    #[serde(rename = "s")]
    pub s: u64,
    #[serde(rename = "fl")]
    pub fl: u64,
    #[serde(rename = "lv")]
    pub lv: u64,
    #[serde(rename = "lb")]
    pub lb: u64,
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
pub struct PlaceOrderResponseStatus {
    #[serde(rename = "code")]
    pub code: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderResponseStatusesItem {
    #[serde(rename = "code")]
    pub code: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderResponse {
    #[serde(rename = "mt")]
    pub mt: u64,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<PlaceOrderResponseStatus>,
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<PlaceOrderResponseStatusesItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderBody {
    #[serde(rename = "rq")]
    pub rq: u64,
    #[serde(rename = "mkt")]
    pub mkt: u64,
    #[serde(rename = "acc")]
    pub acc: u64,
    #[serde(rename = "t")]
    pub t: u64,
    #[serde(rename = "oid", skip_serializing_if = "Option::is_none")]
    pub oid: Option<u64>,
    #[serde(rename = "s")]
    pub s: u64,
    #[serde(rename = "fl")]
    pub fl: u64,
    #[serde(rename = "lv")]
    pub lv: u64,
    #[serde(rename = "lb")]
    pub lb: u64,
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
pub struct CancelOrderResponseStatus {
    #[serde(rename = "code")]
    pub code: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderResponseStatusesItem {
    #[serde(rename = "code")]
    pub code: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    #[serde(rename = "mt")]
    pub mt: u64,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<CancelOrderResponseStatus>,
    #[serde(rename = "statuses", skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<CancelOrderResponseStatusesItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Market {
    #[serde(rename = "id")]
    pub id: u64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "size_decimals", skip_serializing_if = "Option::is_none")]
    pub size_decimals: Option<u64>,
    #[serde(rename = "price_decimals", skip_serializing_if = "Option::is_none")]
    pub price_decimals: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PriceLevel {
    #[serde(rename = "p")]
    pub p: u64,
    #[serde(rename = "s")]
    pub s: u64,
    #[serde(rename = "o")]
    pub o: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedPerplClient(PerplClient);
impl PerplClient {
    pub fn typed(self) -> TypedPerplClient { TypedPerplClient(self) }
}
impl TypedPerplClient {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBookResponse> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedPerplClient(crate::AsyncClient<PerplClient>);
#[cfg(feature = "async")]
impl TypedPerplClient {
    pub fn into_async(self) -> AsyncTypedPerplClient { AsyncTypedPerplClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedPerplClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBookResponse> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub async fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Perpl response model"))
    }
}
