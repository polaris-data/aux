// Generated from src/exchanges/risex/schema/risex.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{RisexClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "market_ids", skip_serializing_if = "Option::is_none")]
    pub market_ids: Option<Vec<u64>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsResponseDataMarketsItemConfig {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "step_size", skip_serializing_if = "Option::is_none")]
    pub step_size: Option<String>,
    #[serde(rename = "step_price", skip_serializing_if = "Option::is_none")]
    pub step_price: Option<String>,
    #[serde(rename = "min_order_size", skip_serializing_if = "Option::is_none")]
    pub min_order_size: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsResponseDataMarketsItem {
    #[serde(rename = "market_id", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<MarketsResponseDataMarketsItemConfig>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsResponseData {
    #[serde(rename = "markets", skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<MarketsResponseDataMarketsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<MarketsResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookRequest {
    #[serde(rename = "market_id")]
    pub market_id: u64,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookResponseData {
    #[serde(rename = "bids", skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<serde_json::Value>>,
    #[serde(rename = "asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBookResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<OrderBookResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "market_id")]
    pub market_id: u64,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesResponseData {
    #[serde(rename = "market_id", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(rename = "trades", skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<TradesResponseData>,
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
pub struct AccountResponseData {
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<AccountResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "market_id", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<u64>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<PositionsResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "market_id")]
    pub market_id: u64,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "start_index", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<OpenOrdersResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryRequest {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "market_id", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<u64>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "order_ids", skip_serializing_if = "Option::is_none")]
    pub order_ids: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<OrderHistoryResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "market_id", skip_serializing_if = "Option::is_none")]
    pub market_id: Option<u64>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<FillsResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SigningDomainRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SigningDomainResponseData {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "chain_id", skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(rename = "verifying_contract", skip_serializing_if = "Option::is_none")]
    pub verifying_contract: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SigningDomainResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<SigningDomainResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SystemConfigRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SystemConfigResponseDataAddresses {
    #[serde(rename = "router", skip_serializing_if = "Option::is_none")]
    pub router: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SystemConfigResponseData {
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<SystemConfigResponseDataAddresses>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SystemConfigResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<SystemConfigResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct NonceStateRequest {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct NonceStateResponseData {
    #[serde(rename = "nonce_anchor", skip_serializing_if = "Option::is_none")]
    pub nonce_anchor: Option<String>,
    #[serde(rename = "current_bitmap_index", skip_serializing_if = "Option::is_none")]
    pub current_bitmap_index: Option<u64>,
    #[serde(rename = "bitmap", skip_serializing_if = "Option::is_none")]
    pub bitmap: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct NonceStateResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<NonceStateResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AuthNonceRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AuthNonceResponse {
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderBody {
    #[serde(rename = "market_id")]
    pub market_id: u64,
    #[serde(rename = "size_steps")]
    pub size_steps: u64,
    #[serde(rename = "price_ticks")]
    pub price_ticks: u64,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "order_type")]
    pub order_type: String,
    #[serde(rename = "time_in_force", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "client_order_id", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "post_only", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "stp_mode", skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<String>,
    #[serde(rename = "ttl_units", skip_serializing_if = "Option::is_none")]
    pub ttl_units: Option<u16>,
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
pub struct PlaceOrderResponseData {
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<PlaceOrderResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderBody {
    #[serde(rename = "market_id")]
    pub market_id: u64,
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "resting_order_id", skip_serializing_if = "Option::is_none")]
    pub resting_order_id: Option<String>,
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
pub struct CancelOrderResponseData {
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<CancelOrderResponseData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsResponseData {
    #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersResponseData {
    #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryResponseData {
    #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsResponseData {
    #[serde(rename = "trades", skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedRisexClient(RisexClient);
impl RisexClient {
    pub fn typed(self) -> TypedRisexClient { TypedRisexClient(self) }
}
impl TypedRisexClient {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn auth_nonce(&self, request: &AuthNonceRequest) -> Result<AuthNonceResponse> {
        serde_json::from_value(self.0.auth_nonce(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn nonce_state(&self, request: &NonceStateRequest) -> Result<NonceStateResponse> {
        serde_json::from_value(self.0.nonce_state(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBookResponse> {
        serde_json::from_value(self.0.order_book(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn signing_domain(&self, request: &SigningDomainRequest) -> Result<SigningDomainResponse> {
        serde_json::from_value(self.0.signing_domain(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn system_config(&self, request: &SystemConfigRequest) -> Result<SystemConfigResponse> {
        serde_json::from_value(self.0.system_config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedRisexClient(crate::AsyncClient<RisexClient>);
#[cfg(feature = "async")]
impl TypedRisexClient {
    pub fn into_async(self) -> AsyncTypedRisexClient { AsyncTypedRisexClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedRisexClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn auth_nonce(&self, request: &AuthNonceRequest) -> Result<AuthNonceResponse> {
        serde_json::from_value(self.0.auth_nonce(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn nonce_state(&self, request: &NonceStateRequest) -> Result<NonceStateResponse> {
        serde_json::from_value(self.0.nonce_state(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn order_book(&self, request: &OrderBookRequest) -> Result<OrderBookResponse> {
        serde_json::from_value(self.0.order_book(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn signing_domain(&self, request: &SigningDomainRequest) -> Result<SigningDomainResponse> {
        serde_json::from_value(self.0.signing_domain(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn system_config(&self, request: &SystemConfigRequest) -> Result<SystemConfigResponse> {
        serde_json::from_value(self.0.system_config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Risex response model"))
    }
}
