// Generated from src/exchanges/qfex/schema/qfex.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{QfexClient, Error, Result};

fn nullable_vec<'de,D:serde::Deserializer<'de>,T:Deserialize<'de>>(d:D) -> std::result::Result<Vec<T>,D::Error> { Ok(Option::<Vec<T>>::deserialize(d)?.unwrap_or_default()) }

#[derive(Clone, Serialize, Deserialize)]
pub struct Symbol {
    #[serde(rename = "clobPairId")]
    pub clob_pair_id: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "tick_size")]
    pub tick_size: String,
    #[serde(rename = "lot_size")]
    pub lot_size: String,
    #[serde(rename = "min_quantity", default)]
    pub min_quantity: String,
    #[serde(rename = "max_quantity", default)]
    pub max_quantity: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Refdata {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "data", default, deserialize_with = "nullable_vec")]
    pub data: Vec<Symbol>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Contracts {
    #[serde(rename = "data", default, deserialize_with = "nullable_vec")]
    pub data: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderBook {
    #[serde(rename = "ticker_id")]
    pub ticker_id: String,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "bids", default, deserialize_with = "nullable_vec")]
    pub bids: Vec<[String; 2]>,
    #[serde(rename = "asks", default, deserialize_with = "nullable_vec")]
    pub asks: Vec<[String; 2]>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "position")]
    pub position: serde_json::Number,
    #[serde(rename = "realised_pnl")]
    pub realised_pnl: serde_json::Number,
    #[serde(rename = "unrealised_pnl")]
    pub unrealised_pnl: serde_json::Number,
    #[serde(rename = "average_price")]
    pub average_price: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Balance {
    #[serde(rename = "available_balance")]
    pub available_balance: serde_json::Number,
    #[serde(rename = "deposit")]
    pub deposit: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PositionsBalance {
    #[serde(rename = "positions", default, deserialize_with = "nullable_vec")]
    pub positions: Vec<Position>,
    #[serde(rename = "balance")]
    pub balance: Balance,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefdataRequest {
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContractsRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderBookRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmptyRequest {
}

pub struct TypedQfexClient(QfexClient);
impl QfexClient {
    pub fn typed(self) -> TypedQfexClient { TypedQfexClient(self) }
}
impl TypedQfexClient {
    pub fn account_balances(&self, request: &EmptyRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.account_balances(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Qfex response model"))
    }
    pub fn contracts(&self, request: &ContractsRequest) -> Result<Contracts> {
        serde_json::from_value(self.0.contracts(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Qfex response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBook> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Qfex response model"))
    }
    pub fn positions(&self, request: &EmptyRequest) -> Result<PositionsBalance> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Qfex response model"))
    }
    pub fn refdata(&self, request: &RefdataRequest) -> Result<Refdata> {
        serde_json::from_value(self.0.refdata(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Qfex response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedQfexClient(crate::AsyncClient<QfexClient>);
#[cfg(feature = "async")]
impl TypedQfexClient {
    pub fn into_async(self) -> AsyncTypedQfexClient { AsyncTypedQfexClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedQfexClient {
    pub async fn account_balances(&self, request: &EmptyRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.account_balances(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Qfex response model"))
    }
    pub async fn contracts(&self, request: &ContractsRequest) -> Result<Contracts> {
        serde_json::from_value(self.0.contracts(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Qfex response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBook> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Qfex response model"))
    }
    pub async fn positions(&self, request: &EmptyRequest) -> Result<PositionsBalance> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Qfex response model"))
    }
    pub async fn refdata(&self, request: &RefdataRequest) -> Result<Refdata> {
        serde_json::from_value(self.0.refdata(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Qfex response model"))
    }
}
