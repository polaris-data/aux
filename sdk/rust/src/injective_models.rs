// Generated from src/exchanges/injective/schema/injective.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{InjectiveClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(rename = "marketStatus", skip_serializing_if = "Option::is_none")]
    pub market_status: Option<String>,
    #[serde(rename = "quoteDenom", skip_serializing_if = "Option::is_none")]
    pub quote_denom: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookRequest {
    #[serde(rename = "marketId")]
    pub market_id: String,
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "marketId")]
    pub market_id: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "subaccountId", skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(rename = "subaccountId")]
    pub subaccount_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "subaccountId")]
    pub subaccount_id: String,
    #[serde(rename = "marketId", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(rename = "skip", skip_serializing_if = "Option::is_none")]
    pub skip: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "subaccountId")]
    pub subaccount_id: String,
    #[serde(rename = "marketId", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(rename = "skip", skip_serializing_if = "Option::is_none")]
    pub skip: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
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
pub struct PendingRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    #[serde(rename = "body")]
    pub body: Order,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "body")]
    pub body: CancelOrder,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "market_id", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(rename = "subaccount_id", skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "market_id")]
    pub market_id: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: String,
    #[serde(rename = "gas_limit")]
    pub gas_limit: String,
    #[serde(rename = "fee_amount")]
    pub fee_amount: String,
    #[serde(rename = "client_order_id", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "quote_decimals")]
    pub quote_decimals: u8,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "margin")]
    pub margin: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "fee_recipient", skip_serializing_if = "Option::is_none")]
    pub fee_recipient: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrder {
    #[serde(rename = "market_id")]
    pub market_id: String,
    #[serde(rename = "subaccount_id")]
    pub subaccount_id: String,
    #[serde(rename = "gas_limit")]
    pub gas_limit: String,
    #[serde(rename = "fee_amount")]
    pub fee_amount: String,
    #[serde(rename = "client_order_id", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "order_hash", skip_serializing_if = "Option::is_none")]
    pub order_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Markets {
    #[serde(rename = "markets")]
    pub markets: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Trades {
    #[serde(rename = "trades")]
    pub trades: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Positions {
    #[serde(rename = "positions")]
    pub positions: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Orders {
    #[serde(rename = "orders")]
    pub orders: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Orderbook {
    #[serde(rename = "orderbook")]
    pub orderbook: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransactionResult {
    #[serde(rename = "tx_response")]
    pub tx_response: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedInjectiveClient(InjectiveClient);
impl InjectiveClient {
    pub fn typed(self) -> TypedInjectiveClient { TypedInjectiveClient(self) }
}
impl TypedInjectiveClient {
    pub fn account(&self, request: &AccountRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<TransactionResult> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<Markets> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<Orders> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<Orderbook> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub fn pending(&self, request: &PendingRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.pending(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<TransactionResult> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<Positions> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<Trades> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub fn transaction(&self, request: &TransactionRequest) -> Result<TransactionResult> {
        serde_json::from_value(self.0.transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedInjectiveClient(crate::AsyncClient<InjectiveClient>);
#[cfg(feature = "async")]
impl TypedInjectiveClient {
    pub fn into_async(self) -> AsyncTypedInjectiveClient { AsyncTypedInjectiveClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedInjectiveClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<TransactionResult> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<Markets> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<Orders> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<Orderbook> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub async fn pending(&self, request: &PendingRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.pending(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<TransactionResult> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<Positions> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<Trades> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
    pub async fn transaction(&self, request: &TransactionRequest) -> Result<TransactionResult> {
        serde_json::from_value(self.0.transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Injective response model"))
    }
}
