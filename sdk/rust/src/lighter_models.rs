// Generated from src/exchanges/lighter/schema/lighter.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{LighterClient, Error, Result};

#[derive(Clone, Serialize, Deserialize)]
pub enum AccountSelector {
    #[serde(rename = "index")]
    Index,
    #[serde(rename = "l1_address")]
    L1Address,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderBook {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "market_id")]
    pub market_id: i64,
    #[serde(rename = "market_type")]
    pub market_type: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "supported_size_decimals")]
    pub supported_size_decimals: u8,
    #[serde(rename = "supported_price_decimals")]
    pub supported_price_decimals: u8,
    #[serde(rename = "min_base_amount")]
    pub min_base_amount: String,
    #[serde(rename = "min_quote_amount")]
    pub min_quote_amount: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderBooks {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "order_books")]
    pub order_books: Vec<OrderBook>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SimpleOrder {
    #[serde(rename = "order_index")]
    pub order_index: i64,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "remaining_base_amount")]
    pub remaining_base_amount: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderBookOrders {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "total_asks")]
    pub total_asks: i64,
    #[serde(rename = "asks")]
    pub asks: Vec<SimpleOrder>,
    #[serde(rename = "total_bids")]
    pub total_bids: i64,
    #[serde(rename = "bids")]
    pub bids: Vec<SimpleOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Trade {
    #[serde(rename = "trade_id")]
    pub trade_id: i64,
    #[serde(rename = "market_id")]
    pub market_id: i64,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "is_maker_ask")]
    pub is_maker_ask: bool,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Trades {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "trades")]
    pub trades: Vec<Trade>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "market_id")]
    pub market_id: i64,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "sign")]
    pub sign: i32,
    #[serde(rename = "position")]
    pub position: String,
    #[serde(rename = "avg_entry_price")]
    pub avg_entry_price: String,
    #[serde(rename = "unrealized_pnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: String,
    #[serde(rename = "liquidation_price")]
    pub liquidation_price: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "index")]
    pub index: i64,
    #[serde(rename = "account_index")]
    pub account_index: i64,
    #[serde(rename = "l1_address")]
    pub l1_address: String,
    #[serde(rename = "available_balance")]
    pub available_balance: String,
    #[serde(rename = "collateral")]
    pub collateral: String,
    #[serde(rename = "positions")]
    pub positions: Vec<Position>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Accounts {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(rename = "accounts")]
    pub accounts: Vec<Account>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "order_index")]
    pub order_index: i64,
    #[serde(rename = "client_order_index")]
    pub client_order_index: i64,
    #[serde(rename = "market_index")]
    pub market_index: i64,
    #[serde(rename = "owner_account_index")]
    pub owner_account_index: i64,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "remaining_base_amount")]
    pub remaining_base_amount: String,
    #[serde(rename = "is_ask")]
    pub is_ask: bool,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Orders {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "orders")]
    pub orders: Vec<Order>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NextNonce {
    #[serde(rename = "code")]
    pub code: i64,
    #[serde(rename = "nonce")]
    pub nonce: i64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderBooksRequest {
    #[serde(rename = "market_id", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderBookOrdersRequest {
    #[serde(rename = "market_id")]
    pub market_id: i64,
    #[serde(rename = "limit")]
    pub limit: u16,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecentTradesRequest {
    #[serde(rename = "market_id")]
    pub market_id: i64,
    #[serde(rename = "limit")]
    pub limit: u16,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NextNonceRequest {
    #[serde(rename = "account_index")]
    pub account_index: i64,
    #[serde(rename = "api_key_index")]
    pub api_key_index: u8,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActiveOrdersRequest {
    #[serde(rename = "account_index", skip_serializing_if = "Option::is_none")]
    pub account_index: Option<i64>,
    #[serde(rename = "market_id", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountRequest {
    #[serde(rename = "by")]
    pub by: AccountSelector,
    #[serde(rename = "value")]
    pub value: serde_json::Value,
}

pub struct TypedLighterClient(LighterClient);
impl LighterClient {
    pub fn typed(self) -> TypedLighterClient { TypedLighterClient(self) }
}
impl TypedLighterClient {
    pub fn account(&self, request: &AccountRequest) -> Result<Accounts> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Lighter response model"))
    }
    pub fn active_orders(&self, request: &ActiveOrdersRequest) -> Result<Orders> {
        serde_json::from_value(self.0.active_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Lighter response model"))
    }
    pub fn next_nonce(&self, request: &NextNonceRequest) -> Result<NextNonce> {
        serde_json::from_value(self.0.next_nonce(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Lighter response model"))
    }
    pub fn order_book_orders(&self, request: &OrderBookOrdersRequest) -> Result<OrderBookOrders> {
        serde_json::from_value(self.0.order_book_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Lighter response model"))
    }
    pub fn order_books(&self, request: &OrderBooksRequest) -> Result<OrderBooks> {
        serde_json::from_value(self.0.order_books(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Lighter response model"))
    }
    pub fn recent_trades(&self, request: &RecentTradesRequest) -> Result<Trades> {
        serde_json::from_value(self.0.recent_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Lighter response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedLighterClient(crate::AsyncClient<LighterClient>);
#[cfg(feature = "async")]
impl TypedLighterClient {
    pub fn into_async(self) -> AsyncTypedLighterClient { AsyncTypedLighterClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedLighterClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<Accounts> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Lighter response model"))
    }
    pub async fn active_orders(&self, request: &ActiveOrdersRequest) -> Result<Orders> {
        serde_json::from_value(self.0.active_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Lighter response model"))
    }
    pub async fn next_nonce(&self, request: &NextNonceRequest) -> Result<NextNonce> {
        serde_json::from_value(self.0.next_nonce(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Lighter response model"))
    }
    pub async fn order_book_orders(&self, request: &OrderBookOrdersRequest) -> Result<OrderBookOrders> {
        serde_json::from_value(self.0.order_book_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Lighter response model"))
    }
    pub async fn order_books(&self, request: &OrderBooksRequest) -> Result<OrderBooks> {
        serde_json::from_value(self.0.order_books(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Lighter response model"))
    }
    pub async fn recent_trades(&self, request: &RecentTradesRequest) -> Result<Trades> {
        serde_json::from_value(self.0.recent_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Lighter response model"))
    }
}
