// Generated from src/exchanges/kraken_spot/schema/kraken_spot.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{KrakenSpotClient, Error, Result};

pub type MarketsResponse = serde_json::Value;
pub type TickerResponse = serde_json::Value;
pub type TradesResponse = serde_json::Value;
pub type AccountResponse = serde_json::Value;
pub type BalancesResponse = serde_json::Value;
pub type PositionsResponse = serde_json::Value;
pub type OrderStatusResponse = serde_json::Value;
pub type OpenOrdersResponse = serde_json::Value;
pub type OrderHistoryResponse = serde_json::Value;
pub type FillsResponse = serde_json::Value;
pub type WebsocketTokenResponse = serde_json::Value;
pub type PlaceOrderResponse = serde_json::Value;
pub type AmendOrderResponse = serde_json::Value;
pub type CancelOrderResponse = serde_json::Value;
pub type CancelAllOrdersResponse = serde_json::Value;
pub type PlaceOrdersResponse = serde_json::Value;
pub type CancelOrdersResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "symbols", skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<u64>,
    #[serde(rename = "snapshot", skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "execution_venue", skip_serializing_if = "Option::is_none")]
    pub execution_venue: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TickerRequest {
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    #[serde(rename = "execution_venue", skip_serializing_if = "Option::is_none")]
    pub execution_venue: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "pair")]
    pub pair: String,
    #[serde(rename = "since", skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BalancesRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "txid", skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    #[serde(rename = "docalcs", skip_serializing_if = "Option::is_none")]
    pub docalcs: Option<bool>,
    #[serde(rename = "consolidation", skip_serializing_if = "Option::is_none")]
    pub consolidation: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "txid")]
    pub txid: String,
    #[serde(rename = "trades", skip_serializing_if = "Option::is_none")]
    pub trades: Option<bool>,
    #[serde(rename = "userref", skip_serializing_if = "Option::is_none")]
    pub userref: Option<u64>,
    #[serde(rename = "consolidate_taker", skip_serializing_if = "Option::is_none")]
    pub consolidate_taker: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "trades", skip_serializing_if = "Option::is_none")]
    pub trades: Option<bool>,
    #[serde(rename = "userref", skip_serializing_if = "Option::is_none")]
    pub userref: Option<u64>,
    #[serde(rename = "cl_ord_id", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryRequest {
    #[serde(rename = "trades", skip_serializing_if = "Option::is_none")]
    pub trades: Option<bool>,
    #[serde(rename = "userref", skip_serializing_if = "Option::is_none")]
    pub userref: Option<u64>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "ofs", skip_serializing_if = "Option::is_none")]
    pub ofs: Option<u64>,
    #[serde(rename = "closetime", skip_serializing_if = "Option::is_none")]
    pub closetime: Option<String>,
    #[serde(rename = "consolidate_taker", skip_serializing_if = "Option::is_none")]
    pub consolidate_taker: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "trades", skip_serializing_if = "Option::is_none")]
    pub trades: Option<bool>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "ofs", skip_serializing_if = "Option::is_none")]
    pub ofs: Option<u64>,
    #[serde(rename = "consolidate_taker", skip_serializing_if = "Option::is_none")]
    pub consolidate_taker: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WebsocketTokenRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    #[serde(rename = "pair")]
    pub pair: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "ordertype")]
    pub ordertype: String,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "price2", skip_serializing_if = "Option::is_none")]
    pub price2: Option<String>,
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(rename = "oflags", skip_serializing_if = "Option::is_none")]
    pub oflags: Option<String>,
    #[serde(rename = "starttm", skip_serializing_if = "Option::is_none")]
    pub starttm: Option<String>,
    #[serde(rename = "expiretm", skip_serializing_if = "Option::is_none")]
    pub expiretm: Option<String>,
    #[serde(rename = "timeinforce", skip_serializing_if = "Option::is_none")]
    pub timeinforce: Option<String>,
    #[serde(rename = "userref", skip_serializing_if = "Option::is_none")]
    pub userref: Option<u64>,
    #[serde(rename = "cl_ord_id", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(rename = "validate", skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
    #[serde(rename = "reduce_only", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(rename = "stptype", skip_serializing_if = "Option::is_none")]
    pub stptype: Option<String>,
    #[serde(rename = "close_ordertype", skip_serializing_if = "Option::is_none")]
    pub close_ordertype: Option<String>,
    #[serde(rename = "close_price", skip_serializing_if = "Option::is_none")]
    pub close_price: Option<String>,
    #[serde(rename = "close_price2", skip_serializing_if = "Option::is_none")]
    pub close_price2: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendOrderRequest {
    #[serde(rename = "txid", skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    #[serde(rename = "cl_ord_id", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(rename = "order_qty", skip_serializing_if = "Option::is_none")]
    pub order_qty: Option<String>,
    #[serde(rename = "display_qty", skip_serializing_if = "Option::is_none")]
    pub display_qty: Option<String>,
    #[serde(rename = "limit_price", skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<String>,
    #[serde(rename = "limit_price_type", skip_serializing_if = "Option::is_none")]
    pub limit_price_type: Option<String>,
    #[serde(rename = "trigger_price", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "trigger_price_type", skip_serializing_if = "Option::is_none")]
    pub trigger_price_type: Option<String>,
    #[serde(rename = "post_only", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "txid", skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    #[serde(rename = "cl_ord_id", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAllOrdersRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrdersRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedKrakenSpotClient(KrakenSpotClient);
impl KrakenSpotClient {
    pub fn typed(self) -> TypedKrakenSpotClient { TypedKrakenSpotClient(self) }
}
impl TypedKrakenSpotClient {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn balances(&self, request: &BalancesRequest) -> Result<BalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn cancel_all_orders(&self, request: &CancelAllOrdersRequest) -> Result<CancelAllOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn cancel_orders(&self, request: &CancelOrdersRequest) -> Result<CancelOrdersResponse> {
        serde_json::from_value(self.0.cancel_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn place_orders(&self, request: &PlaceOrdersRequest) -> Result<PlaceOrdersResponse> {
        serde_json::from_value(self.0.place_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn ticker(&self, request: &TickerRequest) -> Result<TickerResponse> {
        serde_json::from_value(self.0.ticker(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub fn websocket_token(&self, request: &WebsocketTokenRequest) -> Result<WebsocketTokenResponse> {
        serde_json::from_value(self.0.websocket_token(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedKrakenSpotClient(crate::AsyncClient<KrakenSpotClient>);
#[cfg(feature = "async")]
impl TypedKrakenSpotClient {
    pub fn into_async(self) -> AsyncTypedKrakenSpotClient { AsyncTypedKrakenSpotClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedKrakenSpotClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn balances(&self, request: &BalancesRequest) -> Result<BalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn cancel_all_orders(&self, request: &CancelAllOrdersRequest) -> Result<CancelAllOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn cancel_orders(&self, request: &CancelOrdersRequest) -> Result<CancelOrdersResponse> {
        serde_json::from_value(self.0.cancel_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn place_orders(&self, request: &PlaceOrdersRequest) -> Result<PlaceOrdersResponse> {
        serde_json::from_value(self.0.place_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn ticker(&self, request: &TickerRequest) -> Result<TickerResponse> {
        serde_json::from_value(self.0.ticker(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
    pub async fn websocket_token(&self, request: &WebsocketTokenRequest) -> Result<WebsocketTokenResponse> {
        serde_json::from_value(self.0.websocket_token(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KrakenSpot response model"))
    }
}
