// Generated from src/exchanges/paragon/schema/paragon.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{ParagonClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookRequest {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(rename = "user")]
    pub user: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "user")]
    pub user: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "user")]
    pub user: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "user")]
    pub user: String,
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "oid")]
    pub oid: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "body")]
    pub body: Order,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "body")]
    pub body: CancelOrder,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpDexsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "coin", skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "aggregateByTime", skip_serializing_if = "Option::is_none")]
    pub aggregate_by_time: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "b")]
    pub b: bool,
    #[serde(rename = "p")]
    pub p: String,
    #[serde(rename = "s")]
    pub s: String,
    #[serde(rename = "r")]
    pub r: bool,
    #[serde(rename = "t")]
    pub t: Limit,
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrder {
    #[serde(rename = "o", skip_serializing_if = "Option::is_none")]
    pub o: Option<u64>,
    #[serde(rename = "cloid", skip_serializing_if = "Option::is_none")]
    pub cloid: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Markets {
    #[serde(rename = "universe")]
    pub universe: Vec<Market>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "assetPositions")]
    pub asset_positions: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Orderbook {
    #[serde(rename = "coin")]
    pub coin: String,
    #[serde(rename = "levels")]
    pub levels: Vec<Vec<BookLevel>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Market {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "szDecimals", skip_serializing_if = "Option::is_none")]
    pub sz_decimals: Option<u32>,
    #[serde(rename = "maxLeverage", skip_serializing_if = "Option::is_none")]
    pub max_leverage: Option<u32>,
    #[serde(rename = "isDelisted", skip_serializing_if = "Option::is_none")]
    pub is_delisted: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BookLevel {
    #[serde(rename = "px")]
    pub px: String,
    #[serde(rename = "sz")]
    pub sz: String,
    #[serde(rename = "n")]
    pub n: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Limit {
    #[serde(rename = "limit")]
    pub limit: TimeInForce,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TimeInForce {
    #[serde(rename = "tif")]
    pub tif: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedParagonClient(ParagonClient);
impl ParagonClient {
    pub fn typed(self) -> TypedParagonClient { TypedParagonClient(self) }
}
impl TypedParagonClient {
    pub fn account(&self, request: &AccountRequest) -> Result<Account> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<Markets> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<Vec<serde_json::Value>> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<Orderbook> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub fn perp_dexs(&self, request: &PerpDexsRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.perp_dexs(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<Account> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<Vec<serde_json::Value>> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedParagonClient(crate::AsyncClient<ParagonClient>);
#[cfg(feature = "async")]
impl TypedParagonClient {
    pub fn into_async(self) -> AsyncTypedParagonClient { AsyncTypedParagonClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedParagonClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<Account> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<Markets> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<Vec<serde_json::Value>> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<Orderbook> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub async fn perp_dexs(&self, request: &PerpDexsRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.perp_dexs(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<Account> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<Vec<serde_json::Value>> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Paragon response model"))
    }
}
