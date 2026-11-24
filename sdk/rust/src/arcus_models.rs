// Generated from src/exchanges/arcus/schema/arcus.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{ArcusClient, Error, Result};

#[derive(Clone, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}
#[derive(Clone, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "MARKET")]
    Market,
}
#[derive(Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "GTT")]
    Gtt,
    #[serde(rename = "FOK")]
    Fok,
    #[serde(rename = "IOC")]
    Ioc,
    #[serde(rename = "ALO")]
    Alo,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketInfo {
    #[serde(rename = "marketId")]
    pub market_id: u16,
    #[serde(rename = "marketDisplayName")]
    pub market_display_name: String,
    #[serde(rename = "tickSize")]
    pub tick_size: String,
    #[serde(rename = "stepSize")]
    pub step_size: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Markets {
    #[serde(rename = "markets")]
    pub markets: Vec<MarketInfo>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBook {
    #[serde(rename = "bids")]
    pub bids: Vec<[String;2]>,
    #[serde(rename = "asks")]
    pub asks: Vec<[String;2]>,
    #[serde(rename = "lastSequenceId", default)]
    pub last_sequence_id: u64,
    #[serde(rename = "globalSequenceId", default)]
    pub global_sequence_id: u64,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Trade {
    #[serde(rename = "marketId")]
    pub market_id: u16,
    #[serde(rename = "marketDisplayName")]
    pub market_display_name: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "sequenceNumber", default)]
    pub sequence_number: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Trades {
    #[serde(rename = "trades")]
    pub trades: Vec<Trade>,
    #[serde(rename = "total", default)]
    pub total: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "marketId")]
    pub market_id: u16,
    #[serde(rename = "marketDisplayName")]
    pub market_display_name: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "averageEntryPrice")]
    pub average_entry_price: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Positions {
    #[serde(rename = "positions")]
    pub positions: std::collections::BTreeMap<String,Position>,
    #[serde(rename = "total", default)]
    pub total: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "accountIndex")]
    pub account_index: u8,
    #[serde(rename = "equity")]
    pub equity: String,
    #[serde(rename = "freeCollateral")]
    pub free_collateral: String,
    #[serde(rename = "netQuoteBalance")]
    pub net_quote_balance: String,
    #[serde(rename = "positions")]
    pub positions: std::collections::BTreeMap<String,Position>,
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrders {
    #[serde(rename = "orders")]
    pub orders: Vec<serde_json::Value>,
    #[serde(rename = "total", default)]
    pub total: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketsRequest {
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderBookRequest {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "nLevels")]
    pub n_levels: u16,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "accountIndex")]
    pub account_index: u8,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TradesRequest {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "limit")]
    pub limit: u16,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderRequest {
    #[serde(rename = "marketId")]
    pub market_id: u16,
    #[serde(rename = "orderSide")]
    pub order_side: Side,
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "goodTilTime")]
    pub good_til_time: String,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlaceOrderParams {
    #[serde(rename = "market")]
    pub market: MarketInfo,
    #[serde(rename = "body")]
    pub body: OrderRequest,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelOrderRequest {
    #[serde(rename = "marketId")]
    pub market_id: u16,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedArcusClient(ArcusClient);
impl ArcusClient {
    pub fn typed(self) -> TypedArcusClient { TypedArcusClient(self) }
}
impl TypedArcusClient {
    pub fn account(&self, request: &AccountRequest) -> Result<Account> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<OrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<Markets> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub fn open_orders(&self, request: &AccountRequest) -> Result<OpenOrders> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBook> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderParams) -> Result<OrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub fn positions(&self, request: &AccountRequest) -> Result<Positions> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<Trades> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedArcusClient(crate::AsyncClient<ArcusClient>);
#[cfg(feature = "async")]
impl TypedArcusClient {
    pub fn into_async(self) -> AsyncTypedArcusClient { AsyncTypedArcusClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedArcusClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<Account> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<OrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<Markets> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub async fn open_orders(&self, request: &AccountRequest) -> Result<OpenOrders> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBook> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderParams) -> Result<OrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub async fn positions(&self, request: &AccountRequest) -> Result<Positions> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<Trades> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Arcus response model"))
    }
}
