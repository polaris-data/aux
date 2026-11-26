// Generated from src/exchanges/deribit/schema/deribit.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{DeribitClient, Error, Result};

pub type MarketsResponse = serde_json::Value;
pub type TickerResponse = serde_json::Value;
pub type TradesResponse = serde_json::Value;
pub type AccountResponse = serde_json::Value;
pub type PositionsResponse = serde_json::Value;
pub type OrderStatusResponse = serde_json::Value;
pub type OpenOrdersResponse = serde_json::Value;
pub type OrderHistoryResponse = serde_json::Value;
pub type FillsResponse = serde_json::Value;
pub type BuyResponse = serde_json::Value;
pub type SellResponse = serde_json::Value;
pub type AmendOrderResponse = serde_json::Value;
pub type CancelOrderResponse = serde_json::Value;
pub type CancelByLabelResponse = serde_json::Value;
pub type CancelAllOrdersResponse = serde_json::Value;
pub type CancelInstrumentOrdersResponse = serde_json::Value;
pub type CancelCurrencyOrdersResponse = serde_json::Value;
pub type CancelKindOrdersResponse = serde_json::Value;
pub type AuthenticateResponse = serde_json::Value;
pub type AmendByLabelResponse = serde_json::Value;
pub type CancelCurrencyPairOrdersResponse = serde_json::Value;
pub type CancelQuotesResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "instrument_name", skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<u64>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "expired", skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TickerRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "start_seq", skip_serializing_if = "Option::is_none")]
    pub start_seq: Option<u64>,
    #[serde(rename = "end_seq", skip_serializing_if = "Option::is_none")]
    pub end_seq: Option<u64>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[serde(rename = "start_timestamp", skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<u64>,
    #[serde(rename = "end_timestamp", skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<u64>,
    #[serde(rename = "sorting", skip_serializing_if = "Option::is_none")]
    pub sorting: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "extended", skip_serializing_if = "Option::is_none")]
    pub extended: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionsRequest {
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "subaccount_id", skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OpenOrdersRequest {
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderHistoryRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(rename = "include_old", skip_serializing_if = "Option::is_none")]
    pub include_old: Option<bool>,
    #[serde(rename = "include_unfilled", skip_serializing_if = "Option::is_none")]
    pub include_unfilled: Option<bool>,
    #[serde(rename = "historical", skip_serializing_if = "Option::is_none")]
    pub historical: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "start_seq", skip_serializing_if = "Option::is_none")]
    pub start_seq: Option<u64>,
    #[serde(rename = "end_seq", skip_serializing_if = "Option::is_none")]
    pub end_seq: Option<u64>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[serde(rename = "historical", skip_serializing_if = "Option::is_none")]
    pub historical: Option<bool>,
    #[serde(rename = "sorting", skip_serializing_if = "Option::is_none")]
    pub sorting: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuyRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "contracts", skip_serializing_if = "Option::is_none")]
    pub contracts: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "time_in_force", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "max_show", skip_serializing_if = "Option::is_none")]
    pub max_show: Option<String>,
    #[serde(rename = "post_only", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reject_post_only", skip_serializing_if = "Option::is_none")]
    pub reject_post_only: Option<bool>,
    #[serde(rename = "reduce_only", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "trigger_price", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "trigger_offset", skip_serializing_if = "Option::is_none")]
    pub trigger_offset: Option<String>,
    #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    #[serde(rename = "advanced", skip_serializing_if = "Option::is_none")]
    pub advanced: Option<String>,
    #[serde(rename = "mmp", skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    #[serde(rename = "valid_until", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SellRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "contracts", skip_serializing_if = "Option::is_none")]
    pub contracts: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "time_in_force", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "max_show", skip_serializing_if = "Option::is_none")]
    pub max_show: Option<String>,
    #[serde(rename = "post_only", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reject_post_only", skip_serializing_if = "Option::is_none")]
    pub reject_post_only: Option<bool>,
    #[serde(rename = "reduce_only", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "trigger_price", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "trigger_offset", skip_serializing_if = "Option::is_none")]
    pub trigger_offset: Option<String>,
    #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    #[serde(rename = "advanced", skip_serializing_if = "Option::is_none")]
    pub advanced: Option<String>,
    #[serde(rename = "mmp", skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    #[serde(rename = "valid_until", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendOrderRequest {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "contracts", skip_serializing_if = "Option::is_none")]
    pub contracts: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "post_only", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reject_post_only", skip_serializing_if = "Option::is_none")]
    pub reject_post_only: Option<bool>,
    #[serde(rename = "reduce_only", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "trigger_price", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "trigger_offset", skip_serializing_if = "Option::is_none")]
    pub trigger_offset: Option<String>,
    #[serde(rename = "advanced", skip_serializing_if = "Option::is_none")]
    pub advanced: Option<String>,
    #[serde(rename = "mmp", skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    #[serde(rename = "valid_until", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelByLabelRequest {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAllOrdersRequest {
    #[serde(rename = "detailed", skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    #[serde(rename = "freeze_quotes", skip_serializing_if = "Option::is_none")]
    pub freeze_quotes: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelInstrumentOrdersRequest {
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "detailed", skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    #[serde(rename = "include_combos", skip_serializing_if = "Option::is_none")]
    pub include_combos: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelCurrencyOrdersRequest {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "detailed", skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    #[serde(rename = "freeze_quotes", skip_serializing_if = "Option::is_none")]
    pub freeze_quotes: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelKindOrdersRequest {
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "detailed", skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AuthenticateRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendByLabelRequest {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "instrument_name")]
    pub instrument_name: String,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "contracts", skip_serializing_if = "Option::is_none")]
    pub contracts: Option<String>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "post_only", skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "reject_post_only", skip_serializing_if = "Option::is_none")]
    pub reject_post_only: Option<bool>,
    #[serde(rename = "advanced", skip_serializing_if = "Option::is_none")]
    pub advanced: Option<String>,
    #[serde(rename = "trigger_price", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    #[serde(rename = "mmp", skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    #[serde(rename = "valid_until", skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelCurrencyPairOrdersRequest {
    #[serde(rename = "currency_pair")]
    pub currency_pair: String,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "detailed", skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    #[serde(rename = "freeze_quotes", skip_serializing_if = "Option::is_none")]
    pub freeze_quotes: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelQuotesRequest {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "currency_pair")]
    pub currency_pair: String,
    #[serde(rename = "cancel_type")]
    pub cancel_type: String,
    #[serde(rename = "detailed", skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    #[serde(rename = "freeze_quotes", skip_serializing_if = "Option::is_none")]
    pub freeze_quotes: Option<bool>,
    #[serde(rename = "min_delta", skip_serializing_if = "Option::is_none")]
    pub min_delta: Option<String>,
    #[serde(rename = "max_delta", skip_serializing_if = "Option::is_none")]
    pub max_delta: Option<String>,
    #[serde(rename = "quote_set_id", skip_serializing_if = "Option::is_none")]
    pub quote_set_id: Option<String>,
    #[serde(rename = "instrument_name", skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedDeribitClient(DeribitClient);
impl DeribitClient {
    pub fn typed(self) -> TypedDeribitClient { TypedDeribitClient(self) }
}
impl TypedDeribitClient {
    pub fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn amend_by_label(&self, request: &AmendByLabelRequest) -> Result<AmendByLabelResponse> {
        serde_json::from_value(self.0.amend_by_label(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn authenticate(&self, request: &AuthenticateRequest) -> Result<AuthenticateResponse> {
        serde_json::from_value(self.0.authenticate(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn buy(&self, request: &BuyRequest) -> Result<BuyResponse> {
        serde_json::from_value(self.0.buy(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn cancel_all_orders(&self, request: &CancelAllOrdersRequest) -> Result<CancelAllOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn cancel_by_label(&self, request: &CancelByLabelRequest) -> Result<CancelByLabelResponse> {
        serde_json::from_value(self.0.cancel_by_label(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn cancel_currency_orders(&self, request: &CancelCurrencyOrdersRequest) -> Result<CancelCurrencyOrdersResponse> {
        serde_json::from_value(self.0.cancel_currency_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn cancel_currency_pair_orders(&self, request: &CancelCurrencyPairOrdersRequest) -> Result<CancelCurrencyPairOrdersResponse> {
        serde_json::from_value(self.0.cancel_currency_pair_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn cancel_instrument_orders(&self, request: &CancelInstrumentOrdersRequest) -> Result<CancelInstrumentOrdersResponse> {
        serde_json::from_value(self.0.cancel_instrument_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn cancel_kind_orders(&self, request: &CancelKindOrdersRequest) -> Result<CancelKindOrdersResponse> {
        serde_json::from_value(self.0.cancel_kind_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn cancel_quotes(&self, request: &CancelQuotesRequest) -> Result<CancelQuotesResponse> {
        serde_json::from_value(self.0.cancel_quotes(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn sell(&self, request: &SellRequest) -> Result<SellResponse> {
        serde_json::from_value(self.0.sell(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn ticker(&self, request: &TickerRequest) -> Result<TickerResponse> {
        serde_json::from_value(self.0.ticker(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedDeribitClient(crate::AsyncClient<DeribitClient>);
#[cfg(feature = "async")]
impl TypedDeribitClient {
    pub fn into_async(self) -> AsyncTypedDeribitClient { AsyncTypedDeribitClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedDeribitClient {
    pub async fn account(&self, request: &AccountRequest) -> Result<AccountResponse> {
        serde_json::from_value(self.0.account(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn amend_by_label(&self, request: &AmendByLabelRequest) -> Result<AmendByLabelResponse> {
        serde_json::from_value(self.0.amend_by_label(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn amend_order(&self, request: &AmendOrderRequest) -> Result<AmendOrderResponse> {
        serde_json::from_value(self.0.amend_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn authenticate(&self, request: &AuthenticateRequest) -> Result<AuthenticateResponse> {
        serde_json::from_value(self.0.authenticate(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn buy(&self, request: &BuyRequest) -> Result<BuyResponse> {
        serde_json::from_value(self.0.buy(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn cancel_all_orders(&self, request: &CancelAllOrdersRequest) -> Result<CancelAllOrdersResponse> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn cancel_by_label(&self, request: &CancelByLabelRequest) -> Result<CancelByLabelResponse> {
        serde_json::from_value(self.0.cancel_by_label(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn cancel_currency_orders(&self, request: &CancelCurrencyOrdersRequest) -> Result<CancelCurrencyOrdersResponse> {
        serde_json::from_value(self.0.cancel_currency_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn cancel_currency_pair_orders(&self, request: &CancelCurrencyPairOrdersRequest) -> Result<CancelCurrencyPairOrdersResponse> {
        serde_json::from_value(self.0.cancel_currency_pair_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn cancel_instrument_orders(&self, request: &CancelInstrumentOrdersRequest) -> Result<CancelInstrumentOrdersResponse> {
        serde_json::from_value(self.0.cancel_instrument_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn cancel_kind_orders(&self, request: &CancelKindOrdersRequest) -> Result<CancelKindOrdersResponse> {
        serde_json::from_value(self.0.cancel_kind_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        serde_json::from_value(self.0.cancel_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn cancel_quotes(&self, request: &CancelQuotesRequest) -> Result<CancelQuotesResponse> {
        serde_json::from_value(self.0.cancel_quotes(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn fills(&self, request: &FillsRequest) -> Result<FillsResponse> {
        serde_json::from_value(self.0.fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn open_orders(&self, request: &OpenOrdersRequest) -> Result<OpenOrdersResponse> {
        serde_json::from_value(self.0.open_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn order_history(&self, request: &OrderHistoryRequest) -> Result<OrderHistoryResponse> {
        serde_json::from_value(self.0.order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn order_status(&self, request: &OrderStatusRequest) -> Result<OrderStatusResponse> {
        serde_json::from_value(self.0.order_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn positions(&self, request: &PositionsRequest) -> Result<PositionsResponse> {
        serde_json::from_value(self.0.positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn sell(&self, request: &SellRequest) -> Result<SellResponse> {
        serde_json::from_value(self.0.sell(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn ticker(&self, request: &TickerRequest) -> Result<TickerResponse> {
        serde_json::from_value(self.0.ticker(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Deribit response model"))
    }
}
