use crate::Error;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
pub type ExtraFields = BTreeMap<String, serde_json::Value>;

/// A strict, unsigned fixed-point conversion. Rejects excess precision,
/// exponents, signs and overflow; never rounds or passes through f64.
pub fn scale_decimal(value: &str, decimals: u8) -> Result<u64, Error> {
    if decimals > 19 {
        return Err(Error::InvalidRequest("unsupported decimal scale"));
    }
    let (whole, frac) = value.split_once('.').unwrap_or((value, ""));
    if whole.is_empty()
        || !whole.bytes().all(|v| v.is_ascii_digit())
        || !frac.bytes().all(|v| v.is_ascii_digit())
        || frac.len() > decimals as usize
        || (value.contains('.') && frac.is_empty())
    {
        return Err(Error::InvalidRequest(
            "invalid unsigned decimal or excess precision",
        ));
    }
    let mut digits = whole
        .bytes()
        .chain(frac.bytes())
        .chain(std::iter::repeat_n(b'0', decimals as usize - frac.len()));
    digits
        .try_fold(0u64, |n, d| {
            n.checked_mul(10)
                .and_then(|n| n.checked_add((d - b'0') as u64))
        })
        .ok_or(Error::InvalidRequest("scaled decimal overflow"))
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderBook {
    pub symbol: String,
    pub market_id: i64,
    pub market_type: String,
    pub status: String,
    pub supported_size_decimals: u8,
    pub supported_price_decimals: u8,
    pub min_base_amount: String,
    pub min_quote_amount: String,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
impl OrderBook {
    pub fn scale_size(&self, value: &str) -> Result<i64, Error> {
        i64::try_from(scale_decimal(value, self.supported_size_decimals)?)
            .map_err(|_| Error::InvalidRequest("size overflow"))
    }
    pub fn scale_price(&self, value: &str) -> Result<u32, Error> {
        u32::try_from(scale_decimal(value, self.supported_price_decimals)?)
            .map_err(|_| Error::InvalidRequest("price overflow"))
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderBooks {
    pub code: i64,
    pub order_books: Vec<OrderBook>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimpleOrder {
    pub order_index: i64,
    pub price: String,
    pub remaining_base_amount: String,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderBookOrders {
    pub code: i64,
    pub total_asks: i64,
    pub asks: Vec<SimpleOrder>,
    pub total_bids: i64,
    pub bids: Vec<SimpleOrder>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Trade {
    pub trade_id: i64,
    pub market_id: i64,
    pub size: String,
    pub price: String,
    pub is_maker_ask: bool,
    pub timestamp: i64,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Trades {
    pub code: i64,
    pub trades: Vec<Trade>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Position {
    pub market_id: i64,
    pub symbol: String,
    pub sign: i32,
    pub position: String,
    pub avg_entry_price: String,
    pub unrealized_pnl: String,
    pub realized_pnl: String,
    pub liquidation_price: String,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub index: i64,
    pub account_index: i64,
    pub l1_address: String,
    pub available_balance: String,
    pub collateral: String,
    pub positions: Vec<Position>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Accounts {
    pub code: i64,
    pub total: i64,
    pub accounts: Vec<Account>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Order {
    pub order_index: i64,
    pub client_order_index: i64,
    pub market_index: i64,
    pub owner_account_index: i64,
    pub price: String,
    pub remaining_base_amount: String,
    pub is_ask: bool,
    pub status: String,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Orders {
    pub code: i64,
    pub orders: Vec<Order>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NextNonce {
    pub code: i64,
    pub nonce: i64,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
