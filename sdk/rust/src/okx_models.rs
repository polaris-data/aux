// Generated from src/exchanges/okx/schema/okx.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{OkxClient, Error, Result};

pub type MarketsResponse = serde_json::Value;
pub type TickerResponse = serde_json::Value;
pub type TickersResponse = serde_json::Value;
pub type TradesResponse = serde_json::Value;
pub type AccountResponse = serde_json::Value;
pub type BalancesResponse = serde_json::Value;
pub type PositionsResponse = serde_json::Value;
pub type OrderStatusResponse = serde_json::Value;
pub type OpenOrdersResponse = serde_json::Value;
pub type OrderHistoryResponse = serde_json::Value;
pub type FillsResponse = serde_json::Value;
pub type PlaceOrderResponse = serde_json::Value;
pub type AmendOrderResponse = serde_json::Value;
pub type CancelOrderResponse = serde_json::Value;
pub type PlaceOrdersResponse = serde_json::Value;
pub type AmendOrdersResponse = serde_json::Value;
pub type CancelOrdersResponse = serde_json::Value;
pub type PlaceAlgoOrderResponse = serde_json::Value;
pub type AmendAlgoOrderResponse = serde_json::Value;
pub type CancelAlgoOrdersResponse = serde_json::Value;
pub type CancelAdvanceAlgoOrdersResponse = serde_json::Value;
pub type AlgoOrderStatusResponse = serde_json::Value;
pub type OpenAlgoOrdersResponse = serde_json::Value;
pub type AlgoOrderHistoryResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(rename = "instType")]
    pub inst_type: String,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TickerRequest {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TickersRequest {
    #[serde(rename = "instType")]
    pub inst_type: String,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BalancesRequest {
    #[serde(rename = "ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename = "posId", skip_serializing_if = "Option::is_none")]
    pub pos_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename = "ordType", skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryRequest {
    #[serde(rename = "instType")]
    pub inst_type: String,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename = "ordType", skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<u64>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(rename = "subType", skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(rename = "begin", skip_serializing_if = "Option::is_none")]
    pub begin: Option<u64>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendOrderRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceOrdersRequest {
    #[serde(rename = "body")]
    pub body: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendOrdersRequest {
    #[serde(rename = "body")]
    pub body: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersRequest {
    #[serde(rename = "body")]
    pub body: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAlgoOrderRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendAlgoOrderRequest {
    #[serde(rename = "body")]
    pub body: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAlgoOrdersRequest {
    #[serde(rename = "body")]
    pub body: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAdvanceAlgoOrdersRequest {
    #[serde(rename = "body")]
    pub body: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AlgoOrderStatusRequest {
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(rename = "algoClOrdId", skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenAlgoOrdersRequest {
    #[serde(rename = "ordType")]
    pub ord_type: String,
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AlgoOrderHistoryRequest {
    #[serde(rename = "ordType")]
    pub ord_type: String,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedOkxClient(OkxClient);
impl OkxClient {
    pub fn typed(self) -> TypedOkxClient { TypedOkxClient(self) }
}
impl TypedOkxClient {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn algo_order_history(&self, request: &AlgoOrderHistoryRequest) -> Result<AlgoOrderHistoryResponse> {
        serde_json::from_value(self.0.algo_order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn algo_order_status(&self, request: &AlgoOrderStatusRequest) -> Result<AlgoOrderStatusResponse> {
        serde_json::from_value(self.0.algo_order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn amend_algo_order(&self, request: &AmendAlgoOrderRequest) -> Result<AmendAlgoOrderResponse> {
        serde_json::from_value(self.0.amend_algo_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn amend_orders(&self, request: &AmendOrdersRequest) -> Result<AmendOrdersResponse> {
        serde_json::from_value(self.0.amend_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn balances(&self, request: &BalancesRequest) -> Result<BalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn cancel_advance_algo_orders(&self, request: &CancelAdvanceAlgoOrdersRequest) -> Result<CancelAdvanceAlgoOrdersResponse> {
        serde_json::from_value(self.0.cancel_advance_algo_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn cancel_algo_orders(&self, request: &CancelAlgoOrdersRequest) -> Result<CancelAlgoOrdersResponse> {
        serde_json::from_value(self.0.cancel_algo_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn cancel_orders(&self, request: &CancelOrdersRequest) -> Result<CancelOrdersResponse> {
        serde_json::from_value(self.0.cancel_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn open_algo_orders(&self, request: &OpenAlgoOrdersRequest) -> Result<OpenAlgoOrdersResponse> {
        serde_json::from_value(self.0.open_algo_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn place_algo_order(&self, request: &PlaceAlgoOrderRequest) -> Result<PlaceAlgoOrderResponse> {
        serde_json::from_value(self.0.place_algo_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn place_orders(&self, request: &PlaceOrdersRequest) -> Result<PlaceOrdersResponse> {
        serde_json::from_value(self.0.place_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn ticker(&self, request: &TickerRequest) -> Result<TickerResponse> {
        serde_json::from_value(self.0.ticker(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn tickers(&self, request: &TickersRequest) -> Result<TickersResponse> {
        serde_json::from_value(self.0.tickers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedOkxClient(crate::AsyncClient<OkxClient>);
#[cfg(feature = "async")]
impl TypedOkxClient {
    pub fn into_async(self) -> AsyncTypedOkxClient { AsyncTypedOkxClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedOkxClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn algo_order_history(&self, request: &AlgoOrderHistoryRequest) -> Result<AlgoOrderHistoryResponse> {
        serde_json::from_value(self.0.algo_order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn algo_order_status(&self, request: &AlgoOrderStatusRequest) -> Result<AlgoOrderStatusResponse> {
        serde_json::from_value(self.0.algo_order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn amend_algo_order(&self, request: &AmendAlgoOrderRequest) -> Result<AmendAlgoOrderResponse> {
        serde_json::from_value(self.0.amend_algo_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn amend_orders(&self, request: &AmendOrdersRequest) -> Result<AmendOrdersResponse> {
        serde_json::from_value(self.0.amend_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn balances(&self, request: &BalancesRequest) -> Result<BalancesResponse> {
        serde_json::from_value(self.0.balances(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn cancel_advance_algo_orders(&self, request: &CancelAdvanceAlgoOrdersRequest) -> Result<CancelAdvanceAlgoOrdersResponse> {
        serde_json::from_value(self.0.cancel_advance_algo_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn cancel_algo_orders(&self, request: &CancelAlgoOrdersRequest) -> Result<CancelAlgoOrdersResponse> {
        serde_json::from_value(self.0.cancel_algo_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn cancel_orders(&self, request: &CancelOrdersRequest) -> Result<CancelOrdersResponse> {
        serde_json::from_value(self.0.cancel_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn open_algo_orders(&self, request: &OpenAlgoOrdersRequest) -> Result<OpenAlgoOrdersResponse> {
        serde_json::from_value(self.0.open_algo_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn place_algo_order(&self, request: &PlaceAlgoOrderRequest) -> Result<PlaceAlgoOrderResponse> {
        serde_json::from_value(self.0.place_algo_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn place_order(&self, request: &PlaceOrderRequest) -> Result<PlaceOrderResponse> {
        serde_json::from_value(self.0.place_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn place_orders(&self, request: &PlaceOrdersRequest) -> Result<PlaceOrdersResponse> {
        serde_json::from_value(self.0.place_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn ticker(&self, request: &TickerRequest) -> Result<TickerResponse> {
        serde_json::from_value(self.0.ticker(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn tickers(&self, request: &TickersRequest) -> Result<TickersResponse> {
        serde_json::from_value(self.0.tickers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Okx response model"))
    }
}
