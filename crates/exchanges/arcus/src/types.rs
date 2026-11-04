use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
pub type ExtraFields = serde_json::Map<String, Value>;
#[derive(Clone)]
pub struct Arcus {
    http: String,
    ws: String,
}
impl std::fmt::Debug for Arcus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Arcus").finish_non_exhaustive()
    }
}
impl Arcus {
    pub fn mainnet() -> Self {
        Self::custom("https://api.arcus.xyz", "wss://api.arcus.xyz/v1/ws")
    }
    pub fn testnet() -> Self {
        Self::custom(
            "https://api.testnet.arcus.xyz",
            "wss://api.testnet.arcus.xyz/v1/ws",
        )
    }
    pub fn custom(http: impl Into<String>, ws: impl Into<String>) -> Self {
        Self {
            http: http.into().trim_end_matches('/').into(),
            ws: ws.into(),
        }
    }
}
impl aux_rs_core::Exchange for Arcus {
    type Error = crate::Error;
    fn http_url(&self) -> &str {
        &self.http
    }
    fn ws_url(&self) -> &str {
        &self.ws
    }
}
pub(crate) fn nonempty(s: &str) -> Result<(), crate::Error> {
    if s.trim().is_empty() {
        Err(crate::Error::InvalidRequest("identifier cannot be empty"))
    } else {
        Ok(())
    }
}
pub fn address(s: &str) -> Result<String, crate::Error> {
    let value = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);
    if value.len() != 40 || !value.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(crate::Error::InvalidRequest("expected an Ethereum address"));
    }
    Ok(format!("0x{}", value.to_ascii_lowercase()))
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketInfo {
    pub market_id: u16,
    pub market_display_name: String,
    pub tick_size: String,
    pub step_size: String,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub status: String,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
impl MarketInfo {
    pub fn price_ticks(&self, s: &str) -> Result<u64, crate::Error> {
        aux_rs_core::decimal_units(s, &self.tick_size).map_err(crate::Error::InvalidRequest)
    }
    pub fn size_quantums(&self, s: &str) -> Result<u64, crate::Error> {
        aux_rs_core::decimal_units(s, &self.step_size).map_err(crate::Error::InvalidRequest)
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Markets {
    pub markets: Vec<MarketInfo>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub bids: Vec<[String; 2]>,
    pub asks: Vec<[String; 2]>,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub last_sequence_id: u64,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub global_sequence_id: u64,
    pub timestamp: i64,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub market_id: u16,
    pub market_display_name: String,
    pub side: String,
    pub price: String,
    pub size: String,
    pub trade_id: String,
    pub timestamp: i64,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub sequence_number: u64,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trades {
    pub trades: Vec<Trade>,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub total: u64,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub market_id: u16,
    pub market_display_name: String,
    pub side: String,
    pub size: String,
    pub average_entry_price: String,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Positions {
    pub positions: BTreeMap<String, Position>,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub total: u64,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub address: String,
    pub account_index: u8,
    pub equity: String,
    pub free_collateral: String,
    pub net_quote_balance: String,
    pub positions: BTreeMap<String, Position>,
    pub sequence_number: u64,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenOrders {
    pub orders: Vec<Value>,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub total: u64,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Side {
    Buy,
    Sell,
}
impl Side {
    pub(crate) fn native(self) -> u8 {
        match self {
            Self::Buy => 0,
            Self::Sell => 1,
        }
    }
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    Limit,
    Market,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    Gtt,
    Fok,
    Ioc,
    Alo,
}
impl TimeInForce {
    pub(crate) fn native(self) -> u8 {
        match self {
            Self::Gtt => 0,
            Self::Fok => 1,
            Self::Ioc => 2,
            Self::Alo => 3,
        }
    }
}
/// Caller-supplied native order fields. Identity and timestamps come from auth.
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OrderRequest {
    pub market_id: u16,
    pub order_side: Side,
    pub order_type: OrderType,
    pub quantity: String,
    pub price: String,
    pub time_in_force: TimeInForce,
    pub good_til_time: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub client_id: Option<String>,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub reduce_only: bool,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CancelOrderRequest {
    pub market_id: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub client_id: Option<String>,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub status: String,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub order_id: Option<String>,
    #[serde(default)]
    #[cfg_attr(feature = "typescript", ts(optional = nullable))]
    pub client_id: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(feature = "typescript", ts(skip))]
    pub extra: ExtraFields,
}
