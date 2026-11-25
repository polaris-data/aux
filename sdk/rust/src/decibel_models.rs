// Generated from src/exchanges/decibel/schema/decibel.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{DecibelClient, Error, Result};

pub type MarketsResponse = Vec<MarketsResponseItem>;
pub type PositionsResponse = Vec<Position>;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "aggregation_level", skip_serializing_if = "Option::is_none")]
    pub aggregation_level: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsResponseItem {
    #[serde(rename = "market_addr", skip_serializing_if = "Option::is_none")]
    pub market_addr: Option<String>,
    #[serde(rename = "market_name", skip_serializing_if = "Option::is_none")]
    pub market_name: Option<String>,
    #[serde(rename = "asset_type", skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    #[serde(rename = "px_decimals", skip_serializing_if = "Option::is_none")]
    pub px_decimals: Option<u64>,
    #[serde(rename = "sz_decimals", skip_serializing_if = "Option::is_none")]
    pub sz_decimals: Option<u64>,
    #[serde(rename = "tick_size", skip_serializing_if = "Option::is_none")]
    pub tick_size: Option<u64>,
    #[serde(rename = "lot_size", skip_serializing_if = "Option::is_none")]
    pub lot_size: Option<u64>,
    #[serde(rename = "min_size", skip_serializing_if = "Option::is_none")]
    pub min_size: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookRequest {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookResponse {
    #[serde(rename = "bids", skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<serde_json::Value>>,
    #[serde(rename = "asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<serde_json::Value>>,
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "unix_ms", skip_serializing_if = "Option::is_none")]
    pub unix_ms: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesResponse {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<serde_json::Value>>,
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
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
pub struct AccountResponse {
    #[serde(rename = "perp_equity_balance", skip_serializing_if = "Option::is_none")]
    pub perp_equity_balance: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "market_address", skip_serializing_if = "Option::is_none")]
    pub market_address: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "asset_type", skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersResponse {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<serde_json::Value>>,
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryRequest {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryResponse {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<serde_json::Value>>,
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsResponse {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<serde_json::Value>>,
    #[serde(rename = "total_count", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LedgerRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LedgerResponse {
    #[serde(rename = "chain_id", skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GasPriceRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GasPriceResponse {
    #[serde(rename = "gas_estimate", skip_serializing_if = "Option::is_none")]
    pub gas_estimate: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransactionRequest {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransactionResponse {
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderBody {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "is_buy")]
    pub is_buy: bool,
    #[serde(rename = "time_in_force", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<u64>,
    #[serde(rename = "client_order_id", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "reduce_only", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
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
pub struct PlaceOrderResponse {
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderBody {
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
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
pub struct CancelOrderResponse {
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<serde_json::Number>,
    #[serde(rename = "entry_price", skip_serializing_if = "Option::is_none")]
    pub entry_price: Option<serde_json::Number>,
    #[serde(rename = "estimated_liquidation_price", skip_serializing_if = "Option::is_none")]
    pub estimated_liquidation_price: Option<serde_json::Number>,
    #[serde(rename = "is_isolated", skip_serializing_if = "Option::is_none")]
    pub is_isolated: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedDecibelClient(DecibelClient);
impl DecibelClient {
    pub fn typed(self) -> TypedDecibelClient { TypedDecibelClient(self) }
}
impl TypedDecibelClient {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub fn gas_price(&self, request: &GasPriceRequest) -> Result<GasPriceResponse> {
        serde_json::from_value(self.0.gas_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub fn ledger(&self, request: &LedgerRequest) -> Result<LedgerResponse> {
        serde_json::from_value(self.0.ledger(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBookResponse> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub fn transaction(&self, request: &TransactionRequest) -> Result<TransactionResponse> {
        serde_json::from_value(self.0.transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedDecibelClient(crate::AsyncClient<DecibelClient>);
#[cfg(feature = "async")]
impl TypedDecibelClient {
    pub fn into_async(self) -> AsyncTypedDecibelClient { AsyncTypedDecibelClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedDecibelClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub async fn gas_price(&self, request: &GasPriceRequest) -> Result<GasPriceResponse> {
        serde_json::from_value(self.0.gas_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub async fn ledger(&self, request: &LedgerRequest) -> Result<LedgerResponse> {
        serde_json::from_value(self.0.ledger(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBookResponse> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub async fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
    pub async fn transaction(&self, request: &TransactionRequest) -> Result<TransactionResponse> {
        serde_json::from_value(self.0.transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Decibel response model"))
    }
}
