use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
pub type ExtraFields = serde_json::Map<String, Value>;
#[derive(Clone)]
pub struct Qfex {
    http: String,
    mds: String,
    trade: String,
}
impl std::fmt::Debug for Qfex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Qfex").finish_non_exhaustive()
    }
}
impl Qfex {
    pub fn mainnet() -> Self {
        Self::custom(
            "https://api.qfex.com",
            "wss://mds.qfex.com/",
            "wss://trade.qfex.com/",
        )
    }
    /// QFEX calls its separate test environment UAT.
    pub fn testnet() -> Self {
        Self::custom(
            "https://api.qfex.io",
            "wss://mds.qfex.io/",
            "wss://trade.qfex.io/",
        )
    }
    pub fn custom(
        http: impl Into<String>,
        mds: impl Into<String>,
        trade: impl Into<String>,
    ) -> Self {
        Self {
            http: http.into().trim_end_matches('/').into(),
            mds: mds.into(),
            trade: trade.into(),
        }
    }
    pub fn trade_url(&self) -> &str {
        &self.trade
    }
}
impl aux_rs_core::Exchange for Qfex {
    type Error = crate::Error;
    fn http_url(&self) -> &str {
        &self.http
    }
    fn ws_url(&self) -> &str {
        &self.mds
    }
}
pub(crate) fn nonempty(s: &str) -> Result<(), crate::Error> {
    if s.trim().is_empty() {
        Err(crate::Error::InvalidRequest("identifier cannot be empty"))
    } else {
        Ok(())
    }
}
pub(crate) fn nullable_vec<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> Result<Vec<T>, D::Error> {
    Ok(Option::<Vec<T>>::deserialize(d)?.unwrap_or_default())
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Symbol {
    #[serde(rename = "clobPairId")]
    pub clob_pair_id: String,
    pub symbol: String,
    pub tick_size: String,
    pub lot_size: String,
    #[serde(default)]
    pub min_quantity: String,
    #[serde(default)]
    pub max_quantity: String,
    #[serde(default)]
    pub status: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Refdata {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default, deserialize_with = "nullable_vec")]
    pub data: Vec<Symbol>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contracts {
    #[serde(default, deserialize_with = "nullable_vec")]
    pub data: Vec<Value>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderBook {
    pub ticker_id: String,
    pub timestamp: i64,
    #[serde(default, deserialize_with = "nullable_vec")]
    pub bids: Vec<[String; 2]>,
    #[serde(default, deserialize_with = "nullable_vec")]
    pub asks: Vec<[String; 2]>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub position: Number,
    pub realised_pnl: Number,
    pub unrealised_pnl: Number,
    pub average_price: Number,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Balance {
    pub available_balance: Number,
    pub deposit: Number,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PositionsBalance {
    #[serde(default, deserialize_with = "nullable_vec")]
    pub positions: Vec<Position>,
    pub balance: Balance,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Side {
    Buy,
    Sell,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    Limit,
    Market,
    Alo,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddOrder {
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub order_time_in_force: TimeInForce,
    pub quantity: Number,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Number>,
    pub client_order_id: String,
    #[serde(default)]
    pub reduce_only: u8,
}
impl AddOrder {
    pub fn validate(&self) -> Result<(), crate::Error> {
        nonempty(&self.symbol)?;
        nonempty(&self.client_order_id)?;
        if self.reduce_only > 1 {
            return Err(crate::Error::InvalidRequest(
                "reduce_only must be zero or one",
            ));
        }
        aux_rs_core::positive_decimal(&self.quantity.to_string())
            .map_err(crate::Error::InvalidRequest)?;
        if let Some(p) = &self.price {
            aux_rs_core::positive_decimal(&p.to_string()).map_err(crate::Error::InvalidRequest)?;
        }
        if !matches!(self.order_type, OrderType::Market) && self.price.is_none() {
            return Err(crate::Error::InvalidRequest("limit orders require a price"));
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancelOrderIdType {
    OrderId,
    ClientOrderId,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelOrder {
    pub symbol: String,
    pub order_id: String,
    pub cancel_order_id_type: CancelOrderIdType,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    #[serde(default)]
    pub order_id: String,
    pub symbol: String,
    pub status: String,
    #[serde(default)]
    pub client_order_id: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderBookUpdate {
    pub symbol: String,
    pub time: String,
    #[serde(default)]
    pub bid: Vec<[String; 2]>,
    #[serde(default)]
    pub ask: Vec<[String; 2]>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trade {
    pub symbol: String,
    pub trade_id: String,
    pub time: String,
    pub price: String,
    pub size: String,
    pub side: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserOrdersRequest {
    pub limit: u32,
    pub offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}
impl Default for UserOrdersRequest {
    fn default() -> Self {
        Self {
            limit: 100,
            offset: 0,
            symbol: None,
        }
    }
}
impl UserOrdersRequest {
    pub(crate) fn validate(&self) -> Result<(), crate::Error> {
        if self.limit == 0 {
            return Err(crate::Error::InvalidRequest("limit must be positive"));
        }
        if let Some(symbol) = &self.symbol {
            nonempty(symbol)?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserOrders {
    #[serde(default, deserialize_with = "nullable_vec")]
    pub orders: Vec<OrderResponse>,
    #[serde(default, deserialize_with = "nullable_vec")]
    pub twaps: Vec<Value>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}
