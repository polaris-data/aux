// Generated from src/exchanges/kalshi/schema/kalshi.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{KalshiClient, Error, Result};

pub type AcceptBlockTradeProposalResponse = serde_json::Value;
pub type AcceptQuoteResponse = serde_json::Value;
pub type AcceptRfqQuoteResponse = serde_json::Value;
pub type CancelAllOrdersResponse = serde_json::Value;
pub type ConfirmQuoteResponse = serde_json::Value;
pub type ConfirmRfqBatchQuoteResponse = serde_json::Value;
pub type ConfirmRfqQuoteResponse = serde_json::Value;
pub type DeleteApiKeyResponse = serde_json::Value;
pub type DeleteEventContractExitTriggerResponse = serde_json::Value;
pub type DeleteQuoteResponse = serde_json::Value;
pub type DeleteRfqBatchQuoteResponse = serde_json::Value;
pub type DeleteRfqBatchResponse = serde_json::Value;
pub type DeleteRfqQuoteResponse = serde_json::Value;
pub type DeleteRfqResponse = serde_json::Value;
pub type PredictionsApiKeyScope = String;
pub type PredictionsApiKeyType = String;
pub type PredictionsBookSide = String;
pub type PredictionsExchangeIndex = serde_json::Number;
pub type PredictionsExchangeInstance = String;
pub type PredictionsFeeType = String;
pub type PredictionsFixedPointCount = String;
pub type PredictionsFixedPointDollars = String;
pub type PredictionsIntraExchangeInstanceTransferStatus = String;
pub type PredictionsMarketIndexStatus = String;
pub type PredictionsOrderStatus = String;
pub type PredictionsOutcomeSide = String;
pub type PredictionsPriceLevelDollarsCountFp = Vec<String>;
pub type PredictionsRFQBatchQuoteStatus = String;
pub type PredictionsRFQBatchStatus = String;
pub type PredictionsRestingMarginReservation = String;
pub type PredictionsSelfTradePreventionType = String;
pub type PredictionsUserFilter = String;
pub type UpdateSubaccountNettingResponse = serde_json::Value;
pub type UpgradeAccountApiUsageLevelResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AcceptBlockTradeProposalParams {
    #[serde(rename = "block_trade_proposal_id")]
    pub block_trade_proposal_id: String,
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PredictionsAcceptBlockTradeProposalRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AcceptQuoteParams {
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "body")]
    pub body: Box<PredictionsAcceptQuoteRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AcceptRfqBatchParams {
    #[serde(rename = "rfq_batch_id")]
    pub rfq_batch_id: String,
    #[serde(rename = "body")]
    pub body: Box<PredictionsAcceptRFQBatchRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AcceptRfqBatchResponse {
    #[serde(rename = "accepted_side")]
    pub accepted_side: String,
    #[serde(rename = "quotes")]
    pub quotes: Vec<PredictionsQuote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AcceptRfqQuoteParams {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "body")]
    pub body: Box<PredictionsAcceptQuoteRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendOrderV2Params {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "body")]
    pub body: Box<PredictionsAmendOrderV2Request>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AmendOrderV2Response {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "remaining_count", default, skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<String>,
    #[serde(rename = "fill_count", default, skip_serializing_if = "Option::is_none")]
    pub fill_count: Option<String>,
    #[serde(rename = "average_fill_price", default, skip_serializing_if = "Option::is_none")]
    pub average_fill_price: Option<String>,
    #[serde(rename = "average_fee_paid", default, skip_serializing_if = "Option::is_none")]
    pub average_fee_paid: Option<String>,
    #[serde(rename = "ts_ms")]
    pub ts_ms: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for AmendOrderV2Response {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            client_order_id: Default::default(),
            remaining_count: Default::default(),
            fill_count: Default::default(),
            average_fill_price: Default::default(),
            average_fee_paid: Default::default(),
            ts_ms: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApplySubaccountTransferParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsApplySubaccountTransferRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApplySubaccountTransferResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchCancelOrdersV2Params {
    #[serde(rename = "body")]
    pub body: Box<PredictionsBatchCancelOrdersV2Request>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchCancelOrdersV2Response {
    #[serde(rename = "orders")]
    pub orders: Vec<BatchCancelOrdersV2ResponseOrdersItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchCancelOrdersV2ResponseOrdersItem {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "reduced_by")]
    pub reduced_by: String,
    #[serde(rename = "ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub ts_ms: Option<serde_json::Number>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<BatchCancelOrdersV2ResponseOrdersItemError>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchCancelOrdersV2ResponseOrdersItemError {
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateOrdersV2Params {
    #[serde(rename = "body")]
    pub body: Box<PredictionsBatchCreateOrdersV2Request>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateOrdersV2Response {
    #[serde(rename = "orders")]
    pub orders: Vec<BatchCreateOrdersV2ResponseOrdersItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateOrdersV2ResponseOrdersItem {
    #[serde(rename = "order_id", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "fill_count", default, skip_serializing_if = "Option::is_none")]
    pub fill_count: Option<String>,
    #[serde(rename = "remaining_count", default, skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<String>,
    #[serde(rename = "average_fill_price", default, skip_serializing_if = "Option::is_none")]
    pub average_fill_price: Option<String>,
    #[serde(rename = "average_fee_paid", default, skip_serializing_if = "Option::is_none")]
    pub average_fee_paid: Option<String>,
    #[serde(rename = "ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub ts_ms: Option<serde_json::Number>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<BatchCreateOrdersV2ResponseOrdersItemError>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateOrdersV2ResponseOrdersItemError {
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchGetMarketCandlesticksParams {
    #[serde(rename = "market_tickers")]
    pub market_tickers: String,
    #[serde(rename = "start_ts")]
    pub start_ts: serde_json::Number,
    #[serde(rename = "end_ts")]
    pub end_ts: serde_json::Number,
    #[serde(rename = "period_interval")]
    pub period_interval: serde_json::Number,
    #[serde(rename = "include_latest_before_start", default, skip_serializing_if = "Option::is_none")]
    pub include_latest_before_start: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for BatchGetMarketCandlesticksParams {
    fn default() -> Self {
        Self {
            market_tickers: Default::default(),
            start_ts: 0.into(),
            end_ts: 0.into(),
            period_interval: 0.into(),
            include_latest_before_start: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchGetMarketCandlesticksResponse {
    #[serde(rename = "markets")]
    pub markets: Vec<PredictionsMarketCandlesticksResponse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAllOrdersParams {
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrderV2Params {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelOrderV2Response {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "reduced_by")]
    pub reduced_by: String,
    #[serde(rename = "ts_ms")]
    pub ts_ms: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CancelOrderV2Response {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            client_order_id: Default::default(),
            reduced_by: Default::default(),
            ts_ms: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ConfirmQuoteParams {
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PredictionsEmptyResponse>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ConfirmRfqBatchQuoteParams {
    #[serde(rename = "rfq_batch_id")]
    pub rfq_batch_id: String,
    #[serde(rename = "batch_quote_id")]
    pub batch_quote_id: String,
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PredictionsEmptyResponse>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ConfirmRfqQuoteParams {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PredictionsEmptyResponse>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateApiKeyParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsCreateApiKeyRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    #[serde(rename = "warning", default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateFcmPositionTransferParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsCreateFCMPositionTransferRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateFcmPositionTransferResponse {
    #[serde(rename = "position_transfer_id")]
    pub position_transfer_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateFcmSubtraderParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsCreateFCMSubtraderRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateFcmSubtraderResponse {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateMarketInMultivariateEventCollectionParams {
    #[serde(rename = "collection_ticker")]
    pub collection_ticker: String,
    #[serde(rename = "body")]
    pub body: Box<PredictionsCreateMarketInMultivariateEventCollectionRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateMarketInMultivariateEventCollectionResponse {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Box<PredictionsMarket>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateOrderGroupParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsCreateOrderGroupRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateOrderGroupResponse {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount")]
    pub subaccount: serde_json::Number,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateOrderGroupResponse {
    fn default() -> Self {
        Self {
            order_group_id: Default::default(),
            subaccount: 0.into(),
            exchange_index: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateOrderV2Params {
    #[serde(rename = "body")]
    pub body: Box<PredictionsCreateOrderV2Request>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateOrderV2Response {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "fill_count")]
    pub fill_count: String,
    #[serde(rename = "remaining_count")]
    pub remaining_count: String,
    #[serde(rename = "average_fill_price", default, skip_serializing_if = "Option::is_none")]
    pub average_fill_price: Option<String>,
    #[serde(rename = "average_fee_paid", default, skip_serializing_if = "Option::is_none")]
    pub average_fee_paid: Option<String>,
    #[serde(rename = "ts_ms")]
    pub ts_ms: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateOrderV2Response {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            client_order_id: Default::default(),
            fill_count: Default::default(),
            remaining_count: Default::default(),
            average_fill_price: Default::default(),
            average_fee_paid: Default::default(),
            ts_ms: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateOrderWithExitTriggerParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsCreateOrderWithExitTriggerRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateOrderWithExitTriggerResponse {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "fill_count")]
    pub fill_count: String,
    #[serde(rename = "remaining_count")]
    pub remaining_count: String,
    #[serde(rename = "average_fill_price", default, skip_serializing_if = "Option::is_none")]
    pub average_fill_price: Option<String>,
    #[serde(rename = "average_fee_paid", default, skip_serializing_if = "Option::is_none")]
    pub average_fee_paid: Option<String>,
    #[serde(rename = "ts_ms")]
    pub ts_ms: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateOrderWithExitTriggerResponse {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            client_order_id: Default::default(),
            fill_count: Default::default(),
            remaining_count: Default::default(),
            average_fill_price: Default::default(),
            average_fee_paid: Default::default(),
            ts_ms: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateQuoteParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsCreateQuoteRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateQuoteResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateRfqBatchParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsCreateRFQBatchRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateRfqBatchQuoteParams {
    #[serde(rename = "rfq_batch_id")]
    pub rfq_batch_id: String,
    #[serde(rename = "body")]
    pub body: Box<PredictionsCreateRFQBatchQuoteRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateRfqBatchQuoteResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateRfqBatchResponse {
    #[serde(rename = "rfq_batch")]
    pub rfq_batch: Box<PredictionsRFQBatch>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateRfqParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsCreateRFQRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateRfqResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateSubaccountParams {
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PredictionsCreateSubaccountRequest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateSubaccountResponse {
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateSubaccountResponse {
    fn default() -> Self {
        Self {
            subaccount_number: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DecreaseOrderV2Params {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "body")]
    pub body: Box<PredictionsDecreaseOrderV2Request>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DecreaseOrderV2Response {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "remaining_count")]
    pub remaining_count: String,
    #[serde(rename = "ts_ms")]
    pub ts_ms: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for DecreaseOrderV2Response {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            client_order_id: Default::default(),
            remaining_count: Default::default(),
            ts_ms: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteApiKeyParams {
    #[serde(rename = "api_key")]
    pub api_key: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteEventContractExitTriggerParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteFcmEventContractDailyCapParams {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteFcmEventContractDailyCapResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteOrderGroupParams {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteOrderGroupResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteQuoteParams {
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteRfqBatchParams {
    #[serde(rename = "rfq_batch_id")]
    pub rfq_batch_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteRfqBatchQuoteParams {
    #[serde(rename = "rfq_batch_id")]
    pub rfq_batch_id: String,
    #[serde(rename = "batch_quote_id")]
    pub batch_quote_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteRfqParams {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteRfqQuoteParams {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GenerateApiKeyParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsGenerateApiKeyRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GenerateApiKeyResponse {
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    #[serde(rename = "key_type", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(rename = "private_key")]
    pub private_key: String,
    #[serde(rename = "warning", default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountApiLimitsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountApiLimitsResponse {
    #[serde(rename = "usage_tier")]
    pub usage_tier: String,
    #[serde(rename = "read")]
    pub read: Box<PredictionsBucketLimit>,
    #[serde(rename = "write")]
    pub write: Box<PredictionsBucketLimit>,
    #[serde(rename = "grants")]
    pub grants: Vec<PredictionsApiUsageLevelGrant>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountApiUsageLevelVolumeProgressParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountApiUsageLevelVolumeProgressResponse {
    #[serde(rename = "volume_progress")]
    pub volume_progress: Vec<PredictionsAccountApiUsageLevelVolumeProgress>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetAccountEndpointCostsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetAccountEndpointCostsResponse {
    #[serde(rename = "default_cost")]
    pub default_cost: serde_json::Number,
    #[serde(rename = "endpoint_costs")]
    pub endpoint_costs: Vec<PredictionsEndpointTokenCost>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetAccountEndpointCostsResponse {
    fn default() -> Self {
        Self {
            default_cost: 0.into(),
            endpoint_costs: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetApiKeysParams {
    #[serde(rename = "fcm_subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub fcm_subtrader_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetApiKeysResponse {
    #[serde(rename = "api_keys")]
    pub api_keys: Vec<PredictionsApiKey>,
    #[serde(rename = "api_key_region_expiration_ts", default, skip_serializing_if = "Option::is_none")]
    pub api_key_region_expiration_ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBalanceParams {
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetBalanceResponse {
    #[serde(rename = "balance")]
    pub balance: serde_json::Number,
    #[serde(rename = "balance_dollars")]
    pub balance_dollars: String,
    #[serde(rename = "portfolio_value")]
    pub portfolio_value: serde_json::Number,
    #[serde(rename = "updated_ts")]
    pub updated_ts: serde_json::Number,
    #[serde(rename = "balance_breakdown", default, skip_serializing_if = "Option::is_none")]
    pub balance_breakdown: Option<Vec<PredictionsIndexedBalance>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetBalanceResponse {
    fn default() -> Self {
        Self {
            balance: 0.into(),
            balance_dollars: Default::default(),
            portfolio_value: 0.into(),
            updated_ts: 0.into(),
            balance_breakdown: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBlockTradeProposalsParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetBlockTradeProposalsResponse {
    #[serde(rename = "block_trade_proposals")]
    pub block_trade_proposals: Vec<PredictionsBlockTradeProposal>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCommunicationsIdParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCommunicationsIdResponse {
    #[serde(rename = "communications_id")]
    pub communications_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCreditHistoryParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "min_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<serde_json::Number>,
    #[serde(rename = "max_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCreditHistoryResponse {
    #[serde(rename = "credits")]
    pub credits: Vec<PredictionsCreditHistory>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetDepositsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetDepositsResponse {
    #[serde(rename = "deposits")]
    pub deposits: Vec<PredictionsDeposit>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventContractExitTriggersParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventContractExitTriggersResponse {
    #[serde(rename = "exit_trigger", skip_serializing_if = "Option::is_none")]
    pub exit_trigger: Option<GetEventContractExitTriggersResponseExitTrigger>,
    #[serde(rename = "exit_triggers")]
    pub exit_triggers: Vec<PredictionsEventContractExitTrigger>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetEventContractExitTriggersResponseExitTrigger {
    #[serde(rename = "slippage", default, skip_serializing_if = "Option::is_none")]
    pub slippage: Option<Box<PredictionsEventContractExitSlippage>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: serde_json::Number,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "stop_loss_price", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "take_profit_price", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "status_reason", default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "triggered_order_id", default, skip_serializing_if = "Option::is_none")]
    pub triggered_order_id: Option<String>,
    #[serde(rename = "triggered_leg", default, skip_serializing_if = "Option::is_none")]
    pub triggered_leg: Option<String>,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "updated_time")]
    pub updated_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetEventContractExitTriggersResponseExitTrigger {
    fn default() -> Self {
        Self {
            slippage: Default::default(),
            id: Default::default(),
            ticker: Default::default(),
            side: Default::default(),
            subaccount_number: 0.into(),
            kind: Default::default(),
            stop_loss_price: Default::default(),
            take_profit_price: Default::default(),
            status: Default::default(),
            status_reason: Default::default(),
            triggered_order_id: Default::default(),
            triggered_leg: Default::default(),
            created_time: Default::default(),
            updated_time: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventFeeChangesParams {
    #[serde(rename = "event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventFeeChangesResponse {
    #[serde(rename = "event_fee_changes")]
    pub event_fee_changes: Vec<PredictionsEventFeeChange>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetEventForecastPercentilesHistoryParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    #[serde(rename = "percentiles")]
    pub percentiles: Vec<serde_json::Number>,
    #[serde(rename = "start_ts")]
    pub start_ts: serde_json::Number,
    #[serde(rename = "end_ts")]
    pub end_ts: serde_json::Number,
    #[serde(rename = "period_interval")]
    pub period_interval: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetEventForecastPercentilesHistoryParams {
    fn default() -> Self {
        Self {
            ticker: Default::default(),
            series_ticker: Default::default(),
            percentiles: Default::default(),
            start_ts: 0.into(),
            end_ts: 0.into(),
            period_interval: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventForecastPercentilesHistoryResponse {
    #[serde(rename = "forecast_history")]
    pub forecast_history: Vec<PredictionsForecastPercentilesPoint>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventLiveDataParams {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "range", default, skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventLiveDataResponse {
    #[serde(rename = "live_data")]
    pub live_data: Box<PredictionsEventLiveData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventMetadataParams {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventMetadataResponse {
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "featured_image_url", default, skip_serializing_if = "Option::is_none")]
    pub featured_image_url: Option<String>,
    #[serde(rename = "market_details")]
    pub market_details: Vec<PredictionsMarketMetadata>,
    #[serde(rename = "settlement_sources")]
    pub settlement_sources: Vec<PredictionsSettlementSource>,
    #[serde(rename = "competition", default, skip_serializing_if = "Option::is_none")]
    pub competition: Option<String>,
    #[serde(rename = "competition_scope", default, skip_serializing_if = "Option::is_none")]
    pub competition_scope: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventParams {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "exclude_markets", default, skip_serializing_if = "Option::is_none")]
    pub exclude_markets: Option<bool>,
    #[serde(rename = "with_nested_markets", default, skip_serializing_if = "Option::is_none")]
    pub with_nested_markets: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventResponse {
    #[serde(rename = "event")]
    pub event: Box<PredictionsEventData>,
    #[serde(rename = "markets")]
    pub markets: Vec<PredictionsMarket>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "with_nested_markets", default, skip_serializing_if = "Option::is_none")]
    pub with_nested_markets: Option<bool>,
    #[serde(rename = "with_milestones", default, skip_serializing_if = "Option::is_none")]
    pub with_milestones: Option<bool>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "series_ticker", default, skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(rename = "tickers", default, skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    #[serde(rename = "min_close_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_close_ts: Option<serde_json::Number>,
    #[serde(rename = "min_updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetEventsResponse {
    #[serde(rename = "events")]
    pub events: Vec<PredictionsEventData>,
    #[serde(rename = "milestones", default, skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Vec<PredictionsMilestone>>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetExchangeScheduleParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetExchangeScheduleResponse {
    #[serde(rename = "schedule")]
    pub schedule: Box<PredictionsSchedule>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetExchangeStatusParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetExchangeStatusResponse {
    #[serde(rename = "exchange_active")]
    pub exchange_active: bool,
    #[serde(rename = "trading_active")]
    pub trading_active: bool,
    #[serde(rename = "intra_exchange_transfers_active", default, skip_serializing_if = "Option::is_none")]
    pub intra_exchange_transfers_active: Option<bool>,
    #[serde(rename = "exchange_estimated_resume_time", default, skip_serializing_if = "Option::is_none")]
    pub exchange_estimated_resume_time: Option<String>,
    #[serde(rename = "exchange_index_statuses", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index_statuses: Option<Vec<PredictionsExchangeIndexStatus>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFcmEventContractDailyCapParams {
    #[serde(rename = "subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFcmEventContractDailyCapResponse {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "limit")]
    pub limit: String,
    #[serde(rename = "executed_utilization")]
    pub executed_utilization: String,
    #[serde(rename = "resting_order_utilization")]
    pub resting_order_utilization: String,
    #[serde(rename = "pending_order_utilization")]
    pub pending_order_utilization: String,
    #[serde(rename = "cap_date")]
    pub cap_date: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFcmFillsParams {
    #[serde(rename = "min_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<serde_json::Number>,
    #[serde(rename = "max_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFcmFillsResponse {
    #[serde(rename = "fills")]
    pub fills: Vec<PredictionsFcmFill>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFcmOrdersParams {
    #[serde(rename = "subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(rename = "client_order_ids", default, skip_serializing_if = "Option::is_none")]
    pub client_order_ids: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "min_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<serde_json::Number>,
    #[serde(rename = "max_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFcmOrdersResponse {
    #[serde(rename = "orders")]
    pub orders: Vec<PredictionsOrder>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFcmPositionsParams {
    #[serde(rename = "subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "count_filter", default, skip_serializing_if = "Option::is_none")]
    pub count_filter: Option<String>,
    #[serde(rename = "settlement_status", default, skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFcmPositionsResponse {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "market_positions")]
    pub market_positions: Vec<PredictionsMarketPosition>,
    #[serde(rename = "event_positions")]
    pub event_positions: Vec<PredictionsEventPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFcmSubtraderBlockedCategoriesParams {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFcmSubtraderBlockedCategoriesResponse {
    #[serde(rename = "categories")]
    pub categories: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFillsHistoricalParams {
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "min_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<serde_json::Number>,
    #[serde(rename = "max_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFillsHistoricalResponse {
    #[serde(rename = "fills")]
    pub fills: Vec<PredictionsFill>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFillsParams {
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "order_id", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "min_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<serde_json::Number>,
    #[serde(rename = "max_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFillsResponse {
    #[serde(rename = "fills")]
    pub fills: Vec<PredictionsFill>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFiltersForSportsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFiltersForSportsResponse {
    #[serde(rename = "filters_by_sports")]
    pub filters_by_sports: std::collections::BTreeMap<String,Box<PredictionsSportFilterDetails>>,
    #[serde(rename = "sport_ordering")]
    pub sport_ordering: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetGameStatsParams {
    #[serde(rename = "milestone_id")]
    pub milestone_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetGameStatsResponse {
    #[serde(rename = "pbp", default, skip_serializing_if = "Option::is_none")]
    pub pbp: Option<Box<PredictionsPlayByPlay>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetHistoricalCutoffParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetHistoricalCutoffResponse {
    #[serde(rename = "market_settled_ts")]
    pub market_settled_ts: String,
    #[serde(rename = "trades_created_ts")]
    pub trades_created_ts: String,
    #[serde(rename = "orders_updated_ts")]
    pub orders_updated_ts: String,
    #[serde(rename = "market_positions_last_updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub market_positions_last_updated_ts: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetHistoricalMarketParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetHistoricalMarketResponse {
    #[serde(rename = "market")]
    pub market: Box<PredictionsMarket>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetHistoricalMarketsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "tickers", default, skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    #[serde(rename = "event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "series_ticker", default, skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(rename = "mve_filter", default, skip_serializing_if = "Option::is_none")]
    pub mve_filter: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetHistoricalMarketsResponse {
    #[serde(rename = "markets")]
    pub markets: Vec<PredictionsMarket>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetHistoricalOrdersParams {
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "min_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<serde_json::Number>,
    #[serde(rename = "max_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetHistoricalOrdersResponse {
    #[serde(rename = "orders")]
    pub orders: Vec<PredictionsOrder>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetHistoricalPositionsParams {
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetHistoricalPositionsResponse {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "market_positions")]
    pub market_positions: Vec<PredictionsMarketPosition>,
    #[serde(rename = "event_positions")]
    pub event_positions: Vec<PredictionsEventPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetIncentiveProgramsParams {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "incentive_description", default, skip_serializing_if = "Option::is_none")]
    pub incentive_description: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetIncentiveProgramsResponse {
    #[serde(rename = "incentive_programs")]
    pub incentive_programs: Vec<PredictionsIncentiveProgram>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetIntraExchangeInstanceTransferParams {
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetIntraExchangeInstanceTransferResponse {
    #[serde(rename = "transfer")]
    pub transfer: Box<PredictionsIntraExchangeInstanceTransfer>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetIntraExchangeInstanceTransfersParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetIntraExchangeInstanceTransfersResponse {
    #[serde(rename = "transfers")]
    pub transfers: Vec<PredictionsIntraExchangeInstanceTransfer>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLiveDataByMilestoneParams {
    #[serde(rename = "milestone_id")]
    pub milestone_id: String,
    #[serde(rename = "include_player_stats", default, skip_serializing_if = "Option::is_none")]
    pub include_player_stats: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLiveDataByMilestoneResponse {
    #[serde(rename = "live_data")]
    pub live_data: Box<PredictionsLiveData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLiveDataParams {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "milestone_id")]
    pub milestone_id: String,
    #[serde(rename = "include_player_stats", default, skip_serializing_if = "Option::is_none")]
    pub include_player_stats: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLiveDataResponse {
    #[serde(rename = "live_data")]
    pub live_data: Box<PredictionsLiveData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLiveDatasParams {
    #[serde(rename = "milestone_ids")]
    pub milestone_ids: Vec<String>,
    #[serde(rename = "include_player_stats", default, skip_serializing_if = "Option::is_none")]
    pub include_player_stats: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetLiveDatasResponse {
    #[serde(rename = "live_datas")]
    pub live_datas: Vec<PredictionsLiveData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMarketCandlesticksByEventParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    #[serde(rename = "start_ts")]
    pub start_ts: serde_json::Number,
    #[serde(rename = "end_ts")]
    pub end_ts: serde_json::Number,
    #[serde(rename = "period_interval")]
    pub period_interval: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetMarketCandlesticksByEventParams {
    fn default() -> Self {
        Self {
            ticker: Default::default(),
            series_ticker: Default::default(),
            start_ts: 0.into(),
            end_ts: 0.into(),
            period_interval: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMarketCandlesticksByEventResponse {
    #[serde(rename = "market_tickers")]
    pub market_tickers: Vec<String>,
    #[serde(rename = "market_candlesticks")]
    pub market_candlesticks: Vec<Vec<PredictionsMarketCandlestick>>,
    #[serde(rename = "adjusted_end_ts")]
    pub adjusted_end_ts: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetMarketCandlesticksByEventResponse {
    fn default() -> Self {
        Self {
            market_tickers: Default::default(),
            market_candlesticks: Default::default(),
            adjusted_end_ts: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMarketCandlesticksHistoricalParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "start_ts")]
    pub start_ts: serde_json::Number,
    #[serde(rename = "end_ts")]
    pub end_ts: serde_json::Number,
    #[serde(rename = "period_interval")]
    pub period_interval: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetMarketCandlesticksHistoricalParams {
    fn default() -> Self {
        Self {
            ticker: Default::default(),
            start_ts: 0.into(),
            end_ts: 0.into(),
            period_interval: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketCandlesticksHistoricalResponse {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "candlesticks")]
    pub candlesticks: Vec<PredictionsMarketCandlestickHistorical>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMarketCandlesticksParams {
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "start_ts")]
    pub start_ts: serde_json::Number,
    #[serde(rename = "end_ts")]
    pub end_ts: serde_json::Number,
    #[serde(rename = "period_interval")]
    pub period_interval: serde_json::Number,
    #[serde(rename = "include_latest_before_start", default, skip_serializing_if = "Option::is_none")]
    pub include_latest_before_start: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetMarketCandlesticksParams {
    fn default() -> Self {
        Self {
            series_ticker: Default::default(),
            ticker: Default::default(),
            start_ts: 0.into(),
            end_ts: 0.into(),
            period_interval: 0.into(),
            include_latest_before_start: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketCandlesticksResponse {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "candlesticks")]
    pub candlesticks: Vec<PredictionsMarketCandlestick>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketOrderbookParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "depth", default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketOrderbookResponse {
    #[serde(rename = "orderbook_fp")]
    pub orderbook_fp: Box<PredictionsOrderbookCountFp>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketOrderbooksParams {
    #[serde(rename = "tickers")]
    pub tickers: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketOrderbooksResponse {
    #[serde(rename = "orderbooks")]
    pub orderbooks: Vec<PredictionsMarketOrderbookFp>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketResponse {
    #[serde(rename = "market")]
    pub market: Box<PredictionsMarket>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "series_ticker", default, skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(rename = "min_created_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_created_ts: Option<serde_json::Number>,
    #[serde(rename = "max_created_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_created_ts: Option<serde_json::Number>,
    #[serde(rename = "min_updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<serde_json::Number>,
    #[serde(rename = "max_updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_updated_ts: Option<serde_json::Number>,
    #[serde(rename = "max_close_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_close_ts: Option<serde_json::Number>,
    #[serde(rename = "min_close_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_close_ts: Option<serde_json::Number>,
    #[serde(rename = "min_settled_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_settled_ts: Option<serde_json::Number>,
    #[serde(rename = "max_settled_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_settled_ts: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tickers", default, skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    #[serde(rename = "mve_filter", default, skip_serializing_if = "Option::is_none")]
    pub mve_filter: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketsResponse {
    #[serde(rename = "markets")]
    pub markets: Vec<PredictionsMarket>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMilestoneParams {
    #[serde(rename = "milestone_id")]
    pub milestone_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMilestoneResponse {
    #[serde(rename = "milestone")]
    pub milestone: Box<PredictionsMilestone>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMilestonesParams {
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(rename = "minimum_start_date", default, skip_serializing_if = "Option::is_none")]
    pub minimum_start_date: Option<String>,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "competition", default, skip_serializing_if = "Option::is_none")]
    pub competition: Option<String>,
    #[serde(rename = "source_id", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "related_event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub related_event_ticker: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "min_updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetMilestonesParams {
    fn default() -> Self {
        Self {
            limit: 0.into(),
            minimum_start_date: Default::default(),
            category: Default::default(),
            competition: Default::default(),
            source_id: Default::default(),
            type_: Default::default(),
            related_event_ticker: Default::default(),
            cursor: Default::default(),
            min_updated_ts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMilestonesResponse {
    #[serde(rename = "milestones")]
    pub milestones: Vec<PredictionsMilestone>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMultivariateEventCollectionParams {
    #[serde(rename = "collection_ticker")]
    pub collection_ticker: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMultivariateEventCollectionResponse {
    #[serde(rename = "multivariate_contract")]
    pub multivariate_contract: Box<PredictionsMultivariateEventCollection>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMultivariateEventCollectionsParams {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "associated_event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub associated_event_ticker: Option<String>,
    #[serde(rename = "series_ticker", default, skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMultivariateEventCollectionsResponse {
    #[serde(rename = "multivariate_contracts")]
    pub multivariate_contracts: Vec<PredictionsMultivariateEventCollection>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMultivariateEventsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "series_ticker", default, skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(rename = "collection_ticker", default, skip_serializing_if = "Option::is_none")]
    pub collection_ticker: Option<String>,
    #[serde(rename = "tickers", default, skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    #[serde(rename = "with_nested_markets", default, skip_serializing_if = "Option::is_none")]
    pub with_nested_markets: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMultivariateEventsResponse {
    #[serde(rename = "events")]
    pub events: Vec<PredictionsEventData>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderGroupParams {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderGroupResponse {
    #[serde(rename = "is_auto_cancel_enabled")]
    pub is_auto_cancel_enabled: bool,
    #[serde(rename = "contracts_limit_fp", default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<String>,
    #[serde(rename = "orders")]
    pub orders: Vec<String>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderGroupsParams {
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderGroupsResponse {
    #[serde(rename = "order_groups", default, skip_serializing_if = "Option::is_none")]
    pub order_groups: Option<Vec<PredictionsOrderGroup>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderParams {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderQueuePositionParams {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderQueuePositionResponse {
    #[serde(rename = "queue_position_fp")]
    pub queue_position_fp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderQueuePositionsParams {
    #[serde(rename = "market_tickers", default, skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<String>,
    #[serde(rename = "event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderQueuePositionsResponse {
    #[serde(rename = "queue_positions")]
    pub queue_positions: Vec<PredictionsOrderQueuePosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderResponse {
    #[serde(rename = "order")]
    pub order: Box<PredictionsOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrdersParams {
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "min_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<serde_json::Number>,
    #[serde(rename = "max_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrdersResponse {
    #[serde(rename = "orders")]
    pub orders: Vec<PredictionsOrder>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPortfolioRestingOrderTotalValueParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetPortfolioRestingOrderTotalValueResponse {
    #[serde(rename = "total_resting_order_value")]
    pub total_resting_order_value: serde_json::Number,
    #[serde(rename = "resting_order_value_breakdown")]
    pub resting_order_value_breakdown: Vec<PredictionsIndexedBalance>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetPortfolioRestingOrderTotalValueResponse {
    fn default() -> Self {
        Self {
            total_resting_order_value: 0.into(),
            resting_order_value_breakdown: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPositionsParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "count_filter", default, skip_serializing_if = "Option::is_none")]
    pub count_filter: Option<String>,
    #[serde(rename = "settlement_status", default, skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<String>,
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPositionsResponse {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "market_positions")]
    pub market_positions: Vec<PredictionsMarketPosition>,
    #[serde(rename = "event_positions")]
    pub event_positions: Vec<PredictionsEventPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetQuoteParams {
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetQuoteResponse {
    #[serde(rename = "quote")]
    pub quote: Box<PredictionsQuote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetQuotesParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "min_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<serde_json::Number>,
    #[serde(rename = "max_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "quote_creator_user_id", default, skip_serializing_if = "Option::is_none")]
    pub quote_creator_user_id: Option<String>,
    #[serde(rename = "user_filter", default, skip_serializing_if = "Option::is_none")]
    pub user_filter: Option<String>,
    #[serde(rename = "rfq_user_filter", default, skip_serializing_if = "Option::is_none")]
    pub rfq_user_filter: Option<String>,
    #[serde(rename = "rfq_creator_user_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_creator_user_id: Option<String>,
    #[serde(rename = "rfq_creator_subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_creator_subtrader_id: Option<String>,
    #[serde(rename = "rfq_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetQuotesResponse {
    #[serde(rename = "quotes")]
    pub quotes: Vec<PredictionsQuote>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfQsParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "creator_user_id", default, skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    #[serde(rename = "user_filter", default, skip_serializing_if = "Option::is_none")]
    pub user_filter: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfQsResponse {
    #[serde(rename = "rfqs")]
    pub rfqs: Vec<PredictionsRFQ>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfqBatchBestQuotesParams {
    #[serde(rename = "rfq_batch_id")]
    pub rfq_batch_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfqBatchBestQuotesResponse {
    #[serde(rename = "best_quotes")]
    pub best_quotes: Box<PredictionsRFQBatchBestQuotes>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfqBatchParams {
    #[serde(rename = "rfq_batch_id")]
    pub rfq_batch_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfqBatchQuotesParams {
    #[serde(rename = "rfq_batch_id")]
    pub rfq_batch_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfqBatchQuotesResponse {
    #[serde(rename = "batch_quotes")]
    pub batch_quotes: Vec<PredictionsRFQBatchQuote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfqBatchResponse {
    #[serde(rename = "rfq_batch")]
    pub rfq_batch: Box<PredictionsRFQBatch>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfqBatchesParams {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "user_filter", default, skip_serializing_if = "Option::is_none")]
    pub user_filter: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfqBatchesResponse {
    #[serde(rename = "rfq_batches")]
    pub rfq_batches: Vec<PredictionsRFQBatch>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfqParams {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfqQuoteParams {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "quote_id")]
    pub quote_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfqQuoteResponse {
    #[serde(rename = "quote")]
    pub quote: Box<PredictionsQuote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetRfqResponse {
    #[serde(rename = "rfq")]
    pub rfq: Box<PredictionsRFQ>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSeriesFeeChangesParams {
    #[serde(rename = "series_ticker", default, skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(rename = "show_historical", default, skip_serializing_if = "Option::is_none")]
    pub show_historical: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSeriesFeeChangesResponse {
    #[serde(rename = "series_fee_change_arr")]
    pub series_fee_change_arr: Vec<PredictionsSeriesFeeChange>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSeriesListParams {
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(rename = "include_product_metadata", default, skip_serializing_if = "Option::is_none")]
    pub include_product_metadata: Option<bool>,
    #[serde(rename = "include_volume", default, skip_serializing_if = "Option::is_none")]
    pub include_volume: Option<bool>,
    #[serde(rename = "min_updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSeriesListResponse {
    #[serde(rename = "series")]
    pub series: Vec<PredictionsSeries>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSeriesParams {
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    #[serde(rename = "include_volume", default, skip_serializing_if = "Option::is_none")]
    pub include_volume: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSeriesResponse {
    #[serde(rename = "series")]
    pub series: Box<PredictionsSeries>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSettlementsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "min_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<serde_json::Number>,
    #[serde(rename = "max_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<serde_json::Number>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSettlementsResponse {
    #[serde(rename = "settlements")]
    pub settlements: Vec<PredictionsSettlement>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetStructuredTargetParams {
    #[serde(rename = "structured_target_id")]
    pub structured_target_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetStructuredTargetResponse {
    #[serde(rename = "structured_target", default, skip_serializing_if = "Option::is_none")]
    pub structured_target: Option<Box<PredictionsStructuredTarget>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetStructuredTargetsParams {
    #[serde(rename = "ids", default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "competition", default, skip_serializing_if = "Option::is_none")]
    pub competition: Option<String>,
    #[serde(rename = "page_size", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetStructuredTargetsResponse {
    #[serde(rename = "structured_targets", default, skip_serializing_if = "Option::is_none")]
    pub structured_targets: Option<Vec<PredictionsStructuredTarget>>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSubaccountBalancesParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSubaccountBalancesResponse {
    #[serde(rename = "subaccount_balances")]
    pub subaccount_balances: Vec<PredictionsSubaccountBalance>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSubaccountNettingParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSubaccountNettingResponse {
    #[serde(rename = "netting_configs")]
    pub netting_configs: Vec<PredictionsSubaccountNettingConfig>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSubaccountTransfersParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetSubaccountTransfersResponse {
    #[serde(rename = "transfers")]
    pub transfers: Vec<PredictionsSubaccountTransfer>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTagsForSeriesCategoriesParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTagsForSeriesCategoriesResponse {
    #[serde(rename = "tags_by_categories")]
    pub tags_by_categories: std::collections::BTreeMap<String,Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTargetBalanceAllocationParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTargetBalanceAllocationResponse {
    #[serde(rename = "allocations")]
    pub allocations: Vec<PredictionsTargetBalanceAllocation>,
    #[serde(rename = "resting_margin_reservation")]
    pub resting_margin_reservation: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTradesHistoricalParams {
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "min_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<serde_json::Number>,
    #[serde(rename = "max_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<serde_json::Number>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "is_block_trade", default, skip_serializing_if = "Option::is_none")]
    pub is_block_trade: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTradesHistoricalResponse {
    #[serde(rename = "trades")]
    pub trades: Vec<PredictionsTrade>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTradesParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "min_ts", default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<serde_json::Number>,
    #[serde(rename = "max_ts", default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<serde_json::Number>,
    #[serde(rename = "is_block_trade", default, skip_serializing_if = "Option::is_none")]
    pub is_block_trade: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTradesResponse {
    #[serde(rename = "trades")]
    pub trades: Vec<PredictionsTrade>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetUserDataTimestampParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetUserDataTimestampResponse {
    #[serde(rename = "as_of_time")]
    pub as_of_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWeatherIndexCalibrationsParams {
    #[serde(rename = "city")]
    pub city: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWeatherIndexCalibrationsResponse {
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "units")]
    pub units: String,
    #[serde(rename = "calibrations")]
    pub calibrations: Vec<PredictionsWeatherIndexCalibration>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWeatherIndexParams {
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<serde_json::Number>,
    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<serde_json::Number>,
    #[serde(rename = "last_sec", default, skip_serializing_if = "Option::is_none")]
    pub last_sec: Option<serde_json::Number>,
    #[serde(rename = "detailed", default, skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWeatherIndexResponse {
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "config_version", default, skip_serializing_if = "Option::is_none")]
    pub config_version: Option<String>,
    #[serde(rename = "units")]
    pub units: String,
    #[serde(rename = "timeseries")]
    pub timeseries: Vec<PredictionsWeatherIndexPoint>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWithdrawalsParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWithdrawalsResponse {
    #[serde(rename = "withdrawals")]
    pub withdrawals: Vec<PredictionsWithdrawal>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct IntraExchangeInstanceTransferParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsIntraExchangeInstanceTransferRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct IntraExchangeInstanceTransferResponse {
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListFcmSubtradersParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListFcmSubtradersResponse {
    #[serde(rename = "subtraders")]
    pub subtraders: Vec<PredictionsFCMSubtrader>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsAcceptBlockTradeProposalRequest {
    #[serde(rename = "subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsAcceptQuoteRequest {
    #[serde(rename = "accepted_side")]
    pub accepted_side: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsAcceptRFQBatchRequest {
    #[serde(rename = "accepted_side")]
    pub accepted_side: String,
    #[serde(rename = "min_bids")]
    pub min_bids: Vec<PredictionsRFQBatchAcceptLimit>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsAcceptRFQBatchResponse {
    #[serde(rename = "accepted_side")]
    pub accepted_side: String,
    #[serde(rename = "quotes")]
    pub quotes: Vec<PredictionsQuote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsAccountApiUsageLevelVolumeGoal {
    #[serde(rename = "level")]
    pub level: String,
    #[serde(rename = "earn_volume_goal_fp")]
    pub earn_volume_goal_fp: String,
    #[serde(rename = "keep_volume_goal_fp")]
    pub keep_volume_goal_fp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsAccountApiUsageLevelVolumeProgress {
    #[serde(rename = "computed_ts")]
    pub computed_ts: serde_json::Number,
    #[serde(rename = "trailing_30d_volume_fp")]
    pub trailing_30d_volume_fp: String,
    #[serde(rename = "goals")]
    pub goals: Vec<PredictionsAccountApiUsageLevelVolumeGoal>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsAccountApiUsageLevelVolumeProgress {
    fn default() -> Self {
        Self {
            computed_ts: 0.into(),
            trailing_30d_volume_fp: Default::default(),
            goals: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsAmendOrderRequest {
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "updated_client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub updated_client_order_id: Option<String>,
    #[serde(rename = "yes_price", default, skip_serializing_if = "Option::is_none")]
    pub yes_price: Option<serde_json::Number>,
    #[serde(rename = "no_price", default, skip_serializing_if = "Option::is_none")]
    pub no_price: Option<serde_json::Number>,
    #[serde(rename = "yes_price_dollars", default, skip_serializing_if = "Option::is_none")]
    pub yes_price_dollars: Option<String>,
    #[serde(rename = "no_price_dollars", default, skip_serializing_if = "Option::is_none")]
    pub no_price_dollars: Option<String>,
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<serde_json::Number>,
    #[serde(rename = "count_fp", default, skip_serializing_if = "Option::is_none")]
    pub count_fp: Option<String>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsAmendOrderResponse {
    #[serde(rename = "old_order")]
    pub old_order: Box<PredictionsOrder>,
    #[serde(rename = "order")]
    pub order: Box<PredictionsOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsAmendOrderV2Request {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "count")]
    pub count: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "updated_client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub updated_client_order_id: Option<String>,
    #[serde(rename = "expiration_time", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsAmendOrderV2Response {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "remaining_count", default, skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<String>,
    #[serde(rename = "fill_count", default, skip_serializing_if = "Option::is_none")]
    pub fill_count: Option<String>,
    #[serde(rename = "average_fill_price", default, skip_serializing_if = "Option::is_none")]
    pub average_fill_price: Option<String>,
    #[serde(rename = "average_fee_paid", default, skip_serializing_if = "Option::is_none")]
    pub average_fee_paid: Option<String>,
    #[serde(rename = "ts_ms")]
    pub ts_ms: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsAmendOrderV2Response {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            client_order_id: Default::default(),
            remaining_count: Default::default(),
            fill_count: Default::default(),
            average_fill_price: Default::default(),
            average_fee_paid: Default::default(),
            ts_ms: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsApiKey {
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "fcm_subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub fcm_subtrader_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsApiUsageLevelGrant {
    #[serde(rename = "exchange_instance")]
    pub exchange_instance: String,
    #[serde(rename = "level")]
    pub level: String,
    #[serde(rename = "expires_ts", default, skip_serializing_if = "Option::is_none")]
    pub expires_ts: Option<serde_json::Number>,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsApplySubaccountTransferRequest {
    #[serde(rename = "client_transfer_id")]
    pub client_transfer_id: String,
    #[serde(rename = "from_subaccount")]
    pub from_subaccount: serde_json::Number,
    #[serde(rename = "to_subaccount")]
    pub to_subaccount: serde_json::Number,
    #[serde(rename = "amount_cents")]
    pub amount_cents: serde_json::Number,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsApplySubaccountTransferRequest {
    fn default() -> Self {
        Self {
            client_transfer_id: Default::default(),
            from_subaccount: 0.into(),
            to_subaccount: 0.into(),
            amount_cents: 0.into(),
            exchange_index: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsApplySubaccountTransferResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsAssociatedEvent {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "is_yes_only")]
    pub is_yes_only: bool,
    #[serde(rename = "size_max", default, skip_serializing_if = "Option::is_none")]
    pub size_max: Option<serde_json::Number>,
    #[serde(rename = "size_min", default, skip_serializing_if = "Option::is_none")]
    pub size_min: Option<serde_json::Number>,
    #[serde(rename = "active_quoters")]
    pub active_quoters: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsAttachedEventContractExitTrigger {
    #[serde(rename = "slippage", default, skip_serializing_if = "Option::is_none")]
    pub slippage: Option<Box<PredictionsEventContractExitSlippage>>,
    #[serde(rename = "stop_loss_price", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "take_profit_price", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCancelOrdersIndividualResponse {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<PredictionsBatchCancelOrdersIndividualResponseOrder>,
    #[serde(rename = "reduced_by_fp")]
    pub reduced_by_fp: String,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PredictionsBatchCancelOrdersIndividualResponseError>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCancelOrdersIndividualResponseError {
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCancelOrdersIndividualResponseOrder {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "outcome_side")]
    pub outcome_side: String,
    #[serde(rename = "book_side")]
    pub book_side: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "yes_price_dollars")]
    pub yes_price_dollars: String,
    #[serde(rename = "no_price_dollars")]
    pub no_price_dollars: String,
    #[serde(rename = "fill_count_fp")]
    pub fill_count_fp: String,
    #[serde(rename = "remaining_count_fp")]
    pub remaining_count_fp: String,
    #[serde(rename = "initial_count_fp")]
    pub initial_count_fp: String,
    #[serde(rename = "taker_fill_cost_dollars")]
    pub taker_fill_cost_dollars: String,
    #[serde(rename = "maker_fill_cost_dollars")]
    pub maker_fill_cost_dollars: String,
    #[serde(rename = "taker_fees_dollars")]
    pub taker_fees_dollars: String,
    #[serde(rename = "maker_fees_dollars")]
    pub maker_fees_dollars: String,
    #[serde(rename = "expiration_time", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(rename = "created_time", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "last_update_time", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(rename = "self_trade_prevention_type", default, skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_type: Option<String>,
    #[serde(rename = "order_group_id", default, skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(rename = "cancel_order_on_pause", default, skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(rename = "subaccount_number", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_number: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCancelOrdersRequest {
    #[serde(rename = "ids", default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<PredictionsBatchCancelOrdersRequestOrder>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCancelOrdersRequestOrder {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCancelOrdersResponse {
    #[serde(rename = "orders")]
    pub orders: Vec<PredictionsBatchCancelOrdersIndividualResponse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCancelOrdersV2Request {
    #[serde(rename = "orders")]
    pub orders: Vec<PredictionsBatchCancelOrdersV2RequestOrdersItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCancelOrdersV2RequestOrdersItem {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCancelOrdersV2Response {
    #[serde(rename = "orders")]
    pub orders: Vec<PredictionsBatchCancelOrdersV2ResponseOrdersItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCancelOrdersV2ResponseOrdersItem {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "reduced_by")]
    pub reduced_by: String,
    #[serde(rename = "ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub ts_ms: Option<serde_json::Number>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PredictionsBatchCancelOrdersV2ResponseOrdersItemError>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCancelOrdersV2ResponseOrdersItemError {
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCreateOrdersIndividualResponse {
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<PredictionsBatchCreateOrdersIndividualResponseOrder>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PredictionsBatchCreateOrdersIndividualResponseError>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCreateOrdersIndividualResponseError {
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCreateOrdersIndividualResponseOrder {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "outcome_side")]
    pub outcome_side: String,
    #[serde(rename = "book_side")]
    pub book_side: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "yes_price_dollars")]
    pub yes_price_dollars: String,
    #[serde(rename = "no_price_dollars")]
    pub no_price_dollars: String,
    #[serde(rename = "fill_count_fp")]
    pub fill_count_fp: String,
    #[serde(rename = "remaining_count_fp")]
    pub remaining_count_fp: String,
    #[serde(rename = "initial_count_fp")]
    pub initial_count_fp: String,
    #[serde(rename = "taker_fill_cost_dollars")]
    pub taker_fill_cost_dollars: String,
    #[serde(rename = "maker_fill_cost_dollars")]
    pub maker_fill_cost_dollars: String,
    #[serde(rename = "taker_fees_dollars")]
    pub taker_fees_dollars: String,
    #[serde(rename = "maker_fees_dollars")]
    pub maker_fees_dollars: String,
    #[serde(rename = "expiration_time", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(rename = "created_time", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "last_update_time", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(rename = "self_trade_prevention_type", default, skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_type: Option<String>,
    #[serde(rename = "order_group_id", default, skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(rename = "cancel_order_on_pause", default, skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(rename = "subaccount_number", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_number: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCreateOrdersRequest {
    #[serde(rename = "orders")]
    pub orders: Vec<PredictionsCreateOrderRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCreateOrdersResponse {
    #[serde(rename = "orders")]
    pub orders: Vec<PredictionsBatchCreateOrdersIndividualResponse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCreateOrdersV2Request {
    #[serde(rename = "orders")]
    pub orders: Vec<PredictionsCreateOrderV2Request>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCreateOrdersV2Response {
    #[serde(rename = "orders")]
    pub orders: Vec<PredictionsBatchCreateOrdersV2ResponseOrdersItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCreateOrdersV2ResponseOrdersItem {
    #[serde(rename = "order_id", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "fill_count", default, skip_serializing_if = "Option::is_none")]
    pub fill_count: Option<String>,
    #[serde(rename = "remaining_count", default, skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<String>,
    #[serde(rename = "average_fill_price", default, skip_serializing_if = "Option::is_none")]
    pub average_fill_price: Option<String>,
    #[serde(rename = "average_fee_paid", default, skip_serializing_if = "Option::is_none")]
    pub average_fee_paid: Option<String>,
    #[serde(rename = "ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub ts_ms: Option<serde_json::Number>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<PredictionsBatchCreateOrdersV2ResponseOrdersItemError>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchCreateOrdersV2ResponseOrdersItemError {
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBatchGetMarketCandlesticksResponse {
    #[serde(rename = "markets")]
    pub markets: Vec<PredictionsMarketCandlesticksResponse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBidAskDistribution {
    #[serde(rename = "open_dollars")]
    pub open_dollars: String,
    #[serde(rename = "low_dollars")]
    pub low_dollars: String,
    #[serde(rename = "high_dollars")]
    pub high_dollars: String,
    #[serde(rename = "close_dollars")]
    pub close_dollars: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsBidAskDistributionHistorical {
    #[serde(rename = "open")]
    pub open: String,
    #[serde(rename = "low")]
    pub low: String,
    #[serde(rename = "high")]
    pub high: String,
    #[serde(rename = "close")]
    pub close: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsBlockTradeProposal {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "proposer_user_id")]
    pub proposer_user_id: String,
    #[serde(rename = "buyer_user_id")]
    pub buyer_user_id: String,
    #[serde(rename = "buyer_subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub buyer_subtrader_id: Option<String>,
    #[serde(rename = "seller_user_id")]
    pub seller_user_id: String,
    #[serde(rename = "seller_subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub seller_subtrader_id: Option<String>,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "price_centi_cents")]
    pub price_centi_cents: serde_json::Number,
    #[serde(rename = "centicount")]
    pub centicount: serde_json::Number,
    #[serde(rename = "maker_side")]
    pub maker_side: String,
    #[serde(rename = "expiration_ts")]
    pub expiration_ts: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(rename = "updated_ts")]
    pub updated_ts: String,
    #[serde(rename = "buyer_accepted")]
    pub buyer_accepted: bool,
    #[serde(rename = "seller_accepted")]
    pub seller_accepted: bool,
    #[serde(rename = "buyer_accepted_ts", default, skip_serializing_if = "Option::is_none")]
    pub buyer_accepted_ts: Option<String>,
    #[serde(rename = "seller_accepted_ts", default, skip_serializing_if = "Option::is_none")]
    pub seller_accepted_ts: Option<String>,
    #[serde(rename = "executed_ts", default, skip_serializing_if = "Option::is_none")]
    pub executed_ts: Option<String>,
    #[serde(rename = "buyer_order_id", default, skip_serializing_if = "Option::is_none")]
    pub buyer_order_id: Option<String>,
    #[serde(rename = "seller_order_id", default, skip_serializing_if = "Option::is_none")]
    pub seller_order_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsBlockTradeProposal {
    fn default() -> Self {
        Self {
            id: Default::default(),
            proposer_user_id: Default::default(),
            buyer_user_id: Default::default(),
            buyer_subtrader_id: Default::default(),
            seller_user_id: Default::default(),
            seller_subtrader_id: Default::default(),
            market_ticker: Default::default(),
            price_centi_cents: 0.into(),
            centicount: 0.into(),
            maker_side: Default::default(),
            expiration_ts: Default::default(),
            status: Default::default(),
            created_ts: Default::default(),
            updated_ts: Default::default(),
            buyer_accepted: Default::default(),
            seller_accepted: Default::default(),
            buyer_accepted_ts: Default::default(),
            seller_accepted_ts: Default::default(),
            executed_ts: Default::default(),
            buyer_order_id: Default::default(),
            seller_order_id: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsBucketLimit {
    #[serde(rename = "refill_rate")]
    pub refill_rate: serde_json::Number,
    #[serde(rename = "bucket_capacity")]
    pub bucket_capacity: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsBucketLimit {
    fn default() -> Self {
        Self {
            refill_rate: 0.into(),
            bucket_capacity: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsCancelOrderV2Response {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "reduced_by")]
    pub reduced_by: String,
    #[serde(rename = "ts_ms")]
    pub ts_ms: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsCancelOrderV2Response {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            client_order_id: Default::default(),
            reduced_by: Default::default(),
            ts_ms: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateApiKeyRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "scopes", default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "fcm_subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub fcm_subtrader_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateApiKeyResponse {
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    #[serde(rename = "warning", default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsCreateFCMPositionTransferRequest {
    #[serde(rename = "source_subtrader_id")]
    pub source_subtrader_id: String,
    #[serde(rename = "target_subtrader_id")]
    pub target_subtrader_id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "position_centicount")]
    pub position_centicount: serde_json::Number,
    #[serde(rename = "price_centicents")]
    pub price_centicents: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsCreateFCMPositionTransferRequest {
    fn default() -> Self {
        Self {
            source_subtrader_id: Default::default(),
            target_subtrader_id: Default::default(),
            market_ticker: Default::default(),
            position_centicount: 0.into(),
            price_centicents: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateFCMPositionTransferResponse {
    #[serde(rename = "position_transfer_id")]
    pub position_transfer_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateFCMSubtraderRequest {
    #[serde(rename = "subtrader_suffix")]
    pub subtrader_suffix: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateFCMSubtraderResponse {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateMarketInMultivariateEventCollectionRequest {
    #[serde(rename = "selected_markets")]
    pub selected_markets: Vec<PredictionsTickerPair>,
    #[serde(rename = "with_market_payload", default, skip_serializing_if = "Option::is_none")]
    pub with_market_payload: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateMarketInMultivariateEventCollectionResponse {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "market", default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Box<PredictionsMarket>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateOrderGroupRequest {
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "contracts_limit", default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit: Option<serde_json::Number>,
    #[serde(rename = "contracts_limit_fp", default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<String>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsCreateOrderGroupResponse {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount")]
    pub subaccount: serde_json::Number,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsCreateOrderGroupResponse {
    fn default() -> Self {
        Self {
            order_group_id: Default::default(),
            subaccount: 0.into(),
            exchange_index: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateOrderRequest {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<serde_json::Number>,
    #[serde(rename = "count_fp", default, skip_serializing_if = "Option::is_none")]
    pub count_fp: Option<String>,
    #[serde(rename = "yes_price", default, skip_serializing_if = "Option::is_none")]
    pub yes_price: Option<serde_json::Number>,
    #[serde(rename = "no_price", default, skip_serializing_if = "Option::is_none")]
    pub no_price: Option<serde_json::Number>,
    #[serde(rename = "yes_price_dollars", default, skip_serializing_if = "Option::is_none")]
    pub yes_price_dollars: Option<String>,
    #[serde(rename = "no_price_dollars", default, skip_serializing_if = "Option::is_none")]
    pub no_price_dollars: Option<String>,
    #[serde(rename = "expiration_ts", default, skip_serializing_if = "Option::is_none")]
    pub expiration_ts: Option<serde_json::Number>,
    #[serde(rename = "time_in_force", default, skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "buy_max_cost", default, skip_serializing_if = "Option::is_none")]
    pub buy_max_cost: Option<serde_json::Number>,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "sell_position_floor", default, skip_serializing_if = "Option::is_none")]
    pub sell_position_floor: Option<serde_json::Number>,
    #[serde(rename = "self_trade_prevention_type", default, skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_type: Option<String>,
    #[serde(rename = "order_group_id", default, skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(rename = "cancel_order_on_pause", default, skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateOrderResponse {
    #[serde(rename = "order")]
    pub order: Box<PredictionsOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateOrderV2Request {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "count")]
    pub count: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "expiration_time", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<serde_json::Number>,
    #[serde(rename = "time_in_force")]
    pub time_in_force: String,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "self_trade_prevention_type")]
    pub self_trade_prevention_type: String,
    #[serde(rename = "cancel_order_on_pause", default, skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "order_group_id", default, skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsCreateOrderV2Response {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "fill_count")]
    pub fill_count: String,
    #[serde(rename = "remaining_count")]
    pub remaining_count: String,
    #[serde(rename = "average_fill_price", default, skip_serializing_if = "Option::is_none")]
    pub average_fill_price: Option<String>,
    #[serde(rename = "average_fee_paid", default, skip_serializing_if = "Option::is_none")]
    pub average_fee_paid: Option<String>,
    #[serde(rename = "ts_ms")]
    pub ts_ms: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsCreateOrderV2Response {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            client_order_id: Default::default(),
            fill_count: Default::default(),
            remaining_count: Default::default(),
            average_fill_price: Default::default(),
            average_fee_paid: Default::default(),
            ts_ms: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateOrderWithExitTriggerRequest {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "count")]
    pub count: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "expiration_time", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<serde_json::Number>,
    #[serde(rename = "time_in_force")]
    pub time_in_force: String,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "self_trade_prevention_type")]
    pub self_trade_prevention_type: String,
    #[serde(rename = "cancel_order_on_pause", default, skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "order_group_id", default, skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(rename = "exit_trigger")]
    pub exit_trigger: Box<PredictionsAttachedEventContractExitTrigger>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateQuoteRequest {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "yes_bid")]
    pub yes_bid: String,
    #[serde(rename = "no_bid")]
    pub no_bid: String,
    #[serde(rename = "rest_remainder")]
    pub rest_remainder: bool,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateQuoteResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateRFQBatchChild {
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "selected_markets", default, skip_serializing_if = "Option::is_none")]
    pub selected_markets: Option<Vec<PredictionsTickerPair>>,
    #[serde(rename = "contracts_fp", default, skip_serializing_if = "Option::is_none")]
    pub contracts_fp: Option<String>,
    #[serde(rename = "target_cost_dollars", default, skip_serializing_if = "Option::is_none")]
    pub target_cost_dollars: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateRFQBatchQuoteRequest {
    #[serde(rename = "quotes")]
    pub quotes: Vec<PredictionsRFQBatchQuoteEntry>,
    #[serde(rename = "rest_remainder")]
    pub rest_remainder: bool,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateRFQBatchQuoteResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateRFQBatchRequest {
    #[serde(rename = "children")]
    pub children: Vec<PredictionsCreateRFQBatchChild>,
    #[serde(rename = "mve_collection_ticker", default, skip_serializing_if = "Option::is_none")]
    pub mve_collection_ticker: Option<String>,
    #[serde(rename = "contracts_fp", default, skip_serializing_if = "Option::is_none")]
    pub contracts_fp: Option<String>,
    #[serde(rename = "target_cost_dollars", default, skip_serializing_if = "Option::is_none")]
    pub target_cost_dollars: Option<String>,
    #[serde(rename = "target_cost_excludes_fees", default, skip_serializing_if = "Option::is_none")]
    pub target_cost_excludes_fees: Option<bool>,
    #[serde(rename = "rest_remainder", default, skip_serializing_if = "Option::is_none")]
    pub rest_remainder: Option<bool>,
    #[serde(rename = "replace_existing", default, skip_serializing_if = "Option::is_none")]
    pub replace_existing: Option<bool>,
    #[serde(rename = "obscure_creator_id", default, skip_serializing_if = "Option::is_none")]
    pub obscure_creator_id: Option<bool>,
    #[serde(rename = "client_batch_id", default, skip_serializing_if = "Option::is_none")]
    pub client_batch_id: Option<String>,
    #[serde(rename = "subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateRFQBatchResponse {
    #[serde(rename = "rfq_batch")]
    pub rfq_batch: Box<PredictionsRFQBatch>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateRFQRequest {
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "contracts", default, skip_serializing_if = "Option::is_none")]
    pub contracts: Option<serde_json::Number>,
    #[serde(rename = "contracts_fp", default, skip_serializing_if = "Option::is_none")]
    pub contracts_fp: Option<String>,
    #[serde(rename = "target_cost_centi_cents", default, skip_serializing_if = "Option::is_none")]
    pub target_cost_centi_cents: Option<serde_json::Number>,
    #[serde(rename = "target_cost_dollars", default, skip_serializing_if = "Option::is_none")]
    pub target_cost_dollars: Option<String>,
    #[serde(rename = "target_cost_excludes_fees", default, skip_serializing_if = "Option::is_none")]
    pub target_cost_excludes_fees: Option<bool>,
    #[serde(rename = "obscure_creator_id", default, skip_serializing_if = "Option::is_none")]
    pub obscure_creator_id: Option<bool>,
    #[serde(rename = "rest_remainder")]
    pub rest_remainder: bool,
    #[serde(rename = "replace_existing", default, skip_serializing_if = "Option::is_none")]
    pub replace_existing: Option<bool>,
    #[serde(rename = "subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateRFQResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreateSubaccountRequest {
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsCreateSubaccountResponse {
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsCreateSubaccountResponse {
    fn default() -> Self {
        Self {
            subaccount_number: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsCreditHistory {
    #[serde(rename = "credit_id")]
    pub credit_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "amount_dollars")]
    pub amount_dollars: String,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsDailySchedule {
    #[serde(rename = "open_time")]
    pub open_time: String,
    #[serde(rename = "close_time")]
    pub close_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsDecreaseOrderV2Request {
    #[serde(rename = "reduce_by", default, skip_serializing_if = "Option::is_none")]
    pub reduce_by: Option<String>,
    #[serde(rename = "reduce_to", default, skip_serializing_if = "Option::is_none")]
    pub reduce_to: Option<String>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsDecreaseOrderV2Response {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "remaining_count")]
    pub remaining_count: String,
    #[serde(rename = "ts_ms")]
    pub ts_ms: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsDecreaseOrderV2Response {
    fn default() -> Self {
        Self {
            order_id: Default::default(),
            client_order_id: Default::default(),
            remaining_count: Default::default(),
            ts_ms: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsDeposit {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "amount_cents")]
    pub amount_cents: serde_json::Number,
    #[serde(rename = "fee_cents")]
    pub fee_cents: serde_json::Number,
    #[serde(rename = "created_ts")]
    pub created_ts: serde_json::Number,
    #[serde(rename = "finalized_ts", default, skip_serializing_if = "Option::is_none")]
    pub finalized_ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsDeposit {
    fn default() -> Self {
        Self {
            id: Default::default(),
            status: Default::default(),
            type_: Default::default(),
            amount_cents: 0.into(),
            fee_cents: 0.into(),
            created_ts: 0.into(),
            finalized_ts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsEmptyResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsEndpointTokenCost {
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "cost")]
    pub cost: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsEndpointTokenCost {
    fn default() -> Self {
        Self {
            method: Default::default(),
            path: Default::default(),
            cost: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsErrorResponse {
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsEventContractExitSlippage {
    #[serde(rename = "unit")]
    pub unit: String,
    #[serde(rename = "value")]
    pub value: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsEventContractExitSlippage {
    fn default() -> Self {
        Self {
            unit: Default::default(),
            value: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsEventContractExitTrigger {
    #[serde(rename = "slippage", default, skip_serializing_if = "Option::is_none")]
    pub slippage: Option<Box<PredictionsEventContractExitSlippage>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: serde_json::Number,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "stop_loss_price", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "take_profit_price", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "status_reason", default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "triggered_order_id", default, skip_serializing_if = "Option::is_none")]
    pub triggered_order_id: Option<String>,
    #[serde(rename = "triggered_leg", default, skip_serializing_if = "Option::is_none")]
    pub triggered_leg: Option<String>,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "updated_time")]
    pub updated_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsEventContractExitTrigger {
    fn default() -> Self {
        Self {
            slippage: Default::default(),
            id: Default::default(),
            ticker: Default::default(),
            side: Default::default(),
            subaccount_number: 0.into(),
            kind: Default::default(),
            stop_loss_price: Default::default(),
            take_profit_price: Default::default(),
            status: Default::default(),
            status_reason: Default::default(),
            triggered_order_id: Default::default(),
            triggered_leg: Default::default(),
            created_time: Default::default(),
            updated_time: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsEventData {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    #[serde(rename = "sub_title")]
    pub sub_title: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "collateral_return_type")]
    pub collateral_return_type: String,
    #[serde(rename = "mutually_exclusive")]
    pub mutually_exclusive: bool,
    #[serde(rename = "category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "strike_date", default, skip_serializing_if = "Option::is_none")]
    pub strike_date: Option<String>,
    #[serde(rename = "strike_period", default, skip_serializing_if = "Option::is_none")]
    pub strike_period: Option<String>,
    #[serde(rename = "markets", default, skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<PredictionsMarket>>,
    #[serde(rename = "product_metadata", default, skip_serializing_if = "Option::is_none")]
    pub product_metadata: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(rename = "settlement_sources", skip_serializing_if = "Option::is_none")]
    pub settlement_sources: Option<Vec<PredictionsSettlementSource>>,
    #[serde(rename = "last_updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_ts: Option<String>,
    #[serde(rename = "fee_type_override", default, skip_serializing_if = "Option::is_none")]
    pub fee_type_override: Option<String>,
    #[serde(rename = "fee_multiplier_override", default, skip_serializing_if = "Option::is_none")]
    pub fee_multiplier_override: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsEventFeeChange {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    #[serde(rename = "fee_type_override", skip_serializing_if = "Option::is_none")]
    pub fee_type_override: Option<String>,
    #[serde(rename = "fee_multiplier_override", skip_serializing_if = "Option::is_none")]
    pub fee_multiplier_override: Option<serde_json::Number>,
    #[serde(rename = "scheduled_ts")]
    pub scheduled_ts: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsEventLiveData {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "details")]
    pub details: serde_json::Map<String,serde_json::Value>,
    #[serde(rename = "is_historical", default, skip_serializing_if = "Option::is_none")]
    pub is_historical: Option<bool>,
    #[serde(rename = "default_range", default, skip_serializing_if = "Option::is_none")]
    pub default_range: Option<String>,
    #[serde(rename = "range_options", default, skip_serializing_if = "Option::is_none")]
    pub range_options: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsEventPosition {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "total_cost_dollars")]
    pub total_cost_dollars: String,
    #[serde(rename = "total_cost_shares_fp")]
    pub total_cost_shares_fp: String,
    #[serde(rename = "event_exposure_dollars")]
    pub event_exposure_dollars: String,
    #[serde(rename = "realized_pnl_dollars")]
    pub realized_pnl_dollars: String,
    #[serde(rename = "fees_paid_dollars")]
    pub fees_paid_dollars: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsExchangeIndexStatus {
    #[serde(rename = "exchange_index")]
    pub exchange_index: serde_json::Number,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "exchange_active")]
    pub exchange_active: bool,
    #[serde(rename = "trading_active")]
    pub trading_active: bool,
    #[serde(rename = "intra_exchange_transfers_active")]
    pub intra_exchange_transfers_active: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsExchangeIndexStatus {
    fn default() -> Self {
        Self {
            exchange_index: 0.into(),
            description: Default::default(),
            exchange_active: Default::default(),
            trading_active: Default::default(),
            intra_exchange_transfers_active: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsExchangeStatus {
    #[serde(rename = "exchange_active")]
    pub exchange_active: bool,
    #[serde(rename = "trading_active")]
    pub trading_active: bool,
    #[serde(rename = "intra_exchange_transfers_active", default, skip_serializing_if = "Option::is_none")]
    pub intra_exchange_transfers_active: Option<bool>,
    #[serde(rename = "exchange_estimated_resume_time", default, skip_serializing_if = "Option::is_none")]
    pub exchange_estimated_resume_time: Option<String>,
    #[serde(rename = "exchange_index_statuses", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index_statuses: Option<Vec<PredictionsExchangeIndexStatus>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsFCMSubtrader {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "exchange_indices")]
    pub exchange_indices: Vec<serde_json::Number>,
    #[serde(rename = "trading_blocked")]
    pub trading_blocked: bool,
    #[serde(rename = "fcm_trading_blocked")]
    pub fcm_trading_blocked: bool,
    #[serde(rename = "propagation_pending")]
    pub propagation_pending: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsFcmFill {
    #[serde(rename = "fill_id")]
    pub fill_id: String,
    #[serde(rename = "exchange_index")]
    pub exchange_index: serde_json::Number,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "taker_outcome_side")]
    pub taker_outcome_side: String,
    #[serde(rename = "count_fp")]
    pub count_fp: String,
    #[serde(rename = "yes_price_dollars")]
    pub yes_price_dollars: String,
    #[serde(rename = "created_time", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "maker_order_id", default, skip_serializing_if = "Option::is_none")]
    pub maker_order_id: Option<String>,
    #[serde(rename = "maker_subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub maker_subtrader_id: Option<String>,
    #[serde(rename = "maker_fee_cost", default, skip_serializing_if = "Option::is_none")]
    pub maker_fee_cost: Option<String>,
    #[serde(rename = "taker_order_id", default, skip_serializing_if = "Option::is_none")]
    pub taker_order_id: Option<String>,
    #[serde(rename = "taker_subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub taker_subtrader_id: Option<String>,
    #[serde(rename = "taker_fee_cost", default, skip_serializing_if = "Option::is_none")]
    pub taker_fee_cost: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsFcmFill {
    fn default() -> Self {
        Self {
            fill_id: Default::default(),
            exchange_index: 0.into(),
            ticker: Default::default(),
            taker_outcome_side: Default::default(),
            count_fp: Default::default(),
            yes_price_dollars: Default::default(),
            created_time: Default::default(),
            maker_order_id: Default::default(),
            maker_subtrader_id: Default::default(),
            maker_fee_cost: Default::default(),
            taker_order_id: Default::default(),
            taker_subtrader_id: Default::default(),
            taker_fee_cost: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsFill {
    #[serde(rename = "fill_id")]
    pub fill_id: String,
    #[serde(rename = "exchange_index")]
    pub exchange_index: serde_json::Number,
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "outcome_side")]
    pub outcome_side: String,
    #[serde(rename = "book_side")]
    pub book_side: String,
    #[serde(rename = "count_fp")]
    pub count_fp: String,
    #[serde(rename = "yes_price_dollars")]
    pub yes_price_dollars: String,
    #[serde(rename = "no_price_dollars")]
    pub no_price_dollars: String,
    #[serde(rename = "is_taker")]
    pub is_taker: bool,
    #[serde(rename = "created_time", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "fee_cost")]
    pub fee_cost: String,
    #[serde(rename = "subaccount_number", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_number: Option<serde_json::Number>,
    #[serde(rename = "ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsFill {
    fn default() -> Self {
        Self {
            fill_id: Default::default(),
            exchange_index: 0.into(),
            trade_id: Default::default(),
            order_id: Default::default(),
            ticker: Default::default(),
            market_ticker: Default::default(),
            side: Default::default(),
            action: Default::default(),
            outcome_side: Default::default(),
            book_side: Default::default(),
            count_fp: Default::default(),
            yes_price_dollars: Default::default(),
            no_price_dollars: Default::default(),
            is_taker: Default::default(),
            created_time: Default::default(),
            fee_cost: Default::default(),
            subaccount_number: Default::default(),
            ts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsForecastPercentilesPoint {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "end_period_ts")]
    pub end_period_ts: serde_json::Number,
    #[serde(rename = "period_interval")]
    pub period_interval: serde_json::Number,
    #[serde(rename = "percentile_points")]
    pub percentile_points: Vec<PredictionsPercentilePoint>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsForecastPercentilesPoint {
    fn default() -> Self {
        Self {
            event_ticker: Default::default(),
            end_period_ts: 0.into(),
            period_interval: 0.into(),
            percentile_points: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGenerateApiKeyRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "key_type", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(rename = "scopes", default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "fcm_subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub fcm_subtrader_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGenerateApiKeyResponse {
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    #[serde(rename = "key_type", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(rename = "private_key")]
    pub private_key: String,
    #[serde(rename = "warning", default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetAccountApiLimitsResponse {
    #[serde(rename = "usage_tier")]
    pub usage_tier: String,
    #[serde(rename = "read")]
    pub read: Box<PredictionsBucketLimit>,
    #[serde(rename = "write")]
    pub write: Box<PredictionsBucketLimit>,
    #[serde(rename = "grants")]
    pub grants: Vec<PredictionsApiUsageLevelGrant>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetAccountApiUsageLevelVolumeProgressResponse {
    #[serde(rename = "volume_progress")]
    pub volume_progress: Vec<PredictionsAccountApiUsageLevelVolumeProgress>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsGetAccountEndpointCostsResponse {
    #[serde(rename = "default_cost")]
    pub default_cost: serde_json::Number,
    #[serde(rename = "endpoint_costs")]
    pub endpoint_costs: Vec<PredictionsEndpointTokenCost>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsGetAccountEndpointCostsResponse {
    fn default() -> Self {
        Self {
            default_cost: 0.into(),
            endpoint_costs: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetApiKeysResponse {
    #[serde(rename = "api_keys")]
    pub api_keys: Vec<PredictionsApiKey>,
    #[serde(rename = "api_key_region_expiration_ts", default, skip_serializing_if = "Option::is_none")]
    pub api_key_region_expiration_ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsGetBalanceResponse {
    #[serde(rename = "balance")]
    pub balance: serde_json::Number,
    #[serde(rename = "balance_dollars")]
    pub balance_dollars: String,
    #[serde(rename = "portfolio_value")]
    pub portfolio_value: serde_json::Number,
    #[serde(rename = "updated_ts")]
    pub updated_ts: serde_json::Number,
    #[serde(rename = "balance_breakdown", default, skip_serializing_if = "Option::is_none")]
    pub balance_breakdown: Option<Vec<PredictionsIndexedBalance>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsGetBalanceResponse {
    fn default() -> Self {
        Self {
            balance: 0.into(),
            balance_dollars: Default::default(),
            portfolio_value: 0.into(),
            updated_ts: 0.into(),
            balance_breakdown: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetBlockTradeProposalsResponse {
    #[serde(rename = "block_trade_proposals")]
    pub block_trade_proposals: Vec<PredictionsBlockTradeProposal>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetCommunicationsIDResponse {
    #[serde(rename = "communications_id")]
    pub communications_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetCreditHistoryResponse {
    #[serde(rename = "credits")]
    pub credits: Vec<PredictionsCreditHistory>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetDepositsResponse {
    #[serde(rename = "deposits")]
    pub deposits: Vec<PredictionsDeposit>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsGetEventCandlesticksResponse {
    #[serde(rename = "market_tickers")]
    pub market_tickers: Vec<String>,
    #[serde(rename = "market_candlesticks")]
    pub market_candlesticks: Vec<Vec<PredictionsMarketCandlestick>>,
    #[serde(rename = "adjusted_end_ts")]
    pub adjusted_end_ts: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsGetEventCandlesticksResponse {
    fn default() -> Self {
        Self {
            market_tickers: Default::default(),
            market_candlesticks: Default::default(),
            adjusted_end_ts: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetEventFeeChangesResponse {
    #[serde(rename = "event_fee_changes")]
    pub event_fee_changes: Vec<PredictionsEventFeeChange>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetEventForecastPercentilesHistoryResponse {
    #[serde(rename = "forecast_history")]
    pub forecast_history: Vec<PredictionsForecastPercentilesPoint>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetEventLiveDataResponse {
    #[serde(rename = "live_data")]
    pub live_data: Box<PredictionsEventLiveData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetEventMetadataResponse {
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "featured_image_url", default, skip_serializing_if = "Option::is_none")]
    pub featured_image_url: Option<String>,
    #[serde(rename = "market_details")]
    pub market_details: Vec<PredictionsMarketMetadata>,
    #[serde(rename = "settlement_sources")]
    pub settlement_sources: Vec<PredictionsSettlementSource>,
    #[serde(rename = "competition", default, skip_serializing_if = "Option::is_none")]
    pub competition: Option<String>,
    #[serde(rename = "competition_scope", default, skip_serializing_if = "Option::is_none")]
    pub competition_scope: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetEventResponse {
    #[serde(rename = "event")]
    pub event: Box<PredictionsEventData>,
    #[serde(rename = "markets")]
    pub markets: Vec<PredictionsMarket>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetEventsResponse {
    #[serde(rename = "events")]
    pub events: Vec<PredictionsEventData>,
    #[serde(rename = "milestones", default, skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Vec<PredictionsMilestone>>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetExchangeScheduleResponse {
    #[serde(rename = "schedule")]
    pub schedule: Box<PredictionsSchedule>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetFCMEventContractDailyCapResponse {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "limit")]
    pub limit: String,
    #[serde(rename = "executed_utilization")]
    pub executed_utilization: String,
    #[serde(rename = "resting_order_utilization")]
    pub resting_order_utilization: String,
    #[serde(rename = "pending_order_utilization")]
    pub pending_order_utilization: String,
    #[serde(rename = "cap_date")]
    pub cap_date: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetFCMSubtraderBlockedCategoriesResponse {
    #[serde(rename = "categories")]
    pub categories: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetFcmFillsResponse {
    #[serde(rename = "fills")]
    pub fills: Vec<PredictionsFcmFill>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetFillsResponse {
    #[serde(rename = "fills")]
    pub fills: Vec<PredictionsFill>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetFiltersBySportsResponse {
    #[serde(rename = "filters_by_sports")]
    pub filters_by_sports: std::collections::BTreeMap<String,Box<PredictionsSportFilterDetails>>,
    #[serde(rename = "sport_ordering")]
    pub sport_ordering: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetGameStatsResponse {
    #[serde(rename = "pbp", default, skip_serializing_if = "Option::is_none")]
    pub pbp: Option<Box<PredictionsPlayByPlay>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetHistoricalCutoffResponse {
    #[serde(rename = "market_settled_ts")]
    pub market_settled_ts: String,
    #[serde(rename = "trades_created_ts")]
    pub trades_created_ts: String,
    #[serde(rename = "orders_updated_ts")]
    pub orders_updated_ts: String,
    #[serde(rename = "market_positions_last_updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub market_positions_last_updated_ts: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetIncentiveProgramsResponse {
    #[serde(rename = "incentive_programs")]
    pub incentive_programs: Vec<PredictionsIncentiveProgram>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetIntraExchangeInstanceTransferResponse {
    #[serde(rename = "transfer")]
    pub transfer: Box<PredictionsIntraExchangeInstanceTransfer>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetIntraExchangeInstanceTransfersResponse {
    #[serde(rename = "transfers")]
    pub transfers: Vec<PredictionsIntraExchangeInstanceTransfer>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetLiveDataResponse {
    #[serde(rename = "live_data")]
    pub live_data: Box<PredictionsLiveData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetLiveDatasResponse {
    #[serde(rename = "live_datas")]
    pub live_datas: Vec<PredictionsLiveData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetMarketCandlesticksHistoricalResponse {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "candlesticks")]
    pub candlesticks: Vec<PredictionsMarketCandlestickHistorical>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetMarketCandlesticksResponse {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "candlesticks")]
    pub candlesticks: Vec<PredictionsMarketCandlestick>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsGetMarketIndexHistoryResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "last_tick_ms", default, skip_serializing_if = "Option::is_none")]
    pub last_tick_ms: Option<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "start_ms")]
    pub start_ms: serde_json::Number,
    #[serde(rename = "end_ms")]
    pub end_ms: serde_json::Number,
    #[serde(rename = "timeseries")]
    pub timeseries: Vec<PredictionsMarketIndexTimeseriesPoint>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsGetMarketIndexHistoryResponse {
    fn default() -> Self {
        Self {
            id: Default::default(),
            source: Default::default(),
            status: Default::default(),
            last_tick_ms: Default::default(),
            interval: Default::default(),
            start_ms: 0.into(),
            end_ms: 0.into(),
            timeseries: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsGetMarketIndexResponse {
    #[serde(rename = "index")]
    pub index: Box<PredictionsMarketIndex>,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Number>,
    #[serde(rename = "last_tick_ms", default, skip_serializing_if = "Option::is_none")]
    pub last_tick_ms: Option<serde_json::Number>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "start_ms")]
    pub start_ms: serde_json::Number,
    #[serde(rename = "end_ms")]
    pub end_ms: serde_json::Number,
    #[serde(rename = "timeseries")]
    pub timeseries: Vec<PredictionsMarketIndexTimeseriesPoint>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsGetMarketIndexResponse {
    fn default() -> Self {
        Self {
            index: Default::default(),
            source: Default::default(),
            status: Default::default(),
            value: Default::default(),
            last_tick_ms: Default::default(),
            interval: Default::default(),
            start_ms: 0.into(),
            end_ms: 0.into(),
            timeseries: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetMarketIndexesResponse {
    #[serde(rename = "indexes")]
    pub indexes: Vec<PredictionsMarketIndex>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetMarketOrderbookResponse {
    #[serde(rename = "orderbook_fp")]
    pub orderbook_fp: Box<PredictionsOrderbookCountFp>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetMarketOrderbooksResponse {
    #[serde(rename = "orderbooks")]
    pub orderbooks: Vec<PredictionsMarketOrderbookFp>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetMarketResponse {
    #[serde(rename = "market")]
    pub market: Box<PredictionsMarket>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetMarketsResponse {
    #[serde(rename = "markets")]
    pub markets: Vec<PredictionsMarket>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetMilestoneResponse {
    #[serde(rename = "milestone")]
    pub milestone: Box<PredictionsMilestone>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetMilestonesResponse {
    #[serde(rename = "milestones")]
    pub milestones: Vec<PredictionsMilestone>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetMultivariateEventCollectionResponse {
    #[serde(rename = "multivariate_contract")]
    pub multivariate_contract: Box<PredictionsMultivariateEventCollection>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetMultivariateEventCollectionsResponse {
    #[serde(rename = "multivariate_contracts")]
    pub multivariate_contracts: Vec<PredictionsMultivariateEventCollection>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetMultivariateEventsResponse {
    #[serde(rename = "events")]
    pub events: Vec<PredictionsEventData>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetOrderGroupResponse {
    #[serde(rename = "is_auto_cancel_enabled")]
    pub is_auto_cancel_enabled: bool,
    #[serde(rename = "contracts_limit_fp", default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<String>,
    #[serde(rename = "orders")]
    pub orders: Vec<String>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetOrderGroupsResponse {
    #[serde(rename = "order_groups", default, skip_serializing_if = "Option::is_none")]
    pub order_groups: Option<Vec<PredictionsOrderGroup>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetOrderQueuePositionResponse {
    #[serde(rename = "queue_position_fp")]
    pub queue_position_fp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetOrderQueuePositionsResponse {
    #[serde(rename = "queue_positions")]
    pub queue_positions: Vec<PredictionsOrderQueuePosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetOrderResponse {
    #[serde(rename = "order")]
    pub order: Box<PredictionsOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetOrdersResponse {
    #[serde(rename = "orders")]
    pub orders: Vec<PredictionsOrder>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsGetPortfolioRestingOrderTotalValueResponse {
    #[serde(rename = "total_resting_order_value")]
    pub total_resting_order_value: serde_json::Number,
    #[serde(rename = "resting_order_value_breakdown")]
    pub resting_order_value_breakdown: Vec<PredictionsIndexedBalance>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsGetPortfolioRestingOrderTotalValueResponse {
    fn default() -> Self {
        Self {
            total_resting_order_value: 0.into(),
            resting_order_value_breakdown: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetPositionsResponse {
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "market_positions")]
    pub market_positions: Vec<PredictionsMarketPosition>,
    #[serde(rename = "event_positions")]
    pub event_positions: Vec<PredictionsEventPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetQuoteResponse {
    #[serde(rename = "quote")]
    pub quote: Box<PredictionsQuote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetQuotesResponse {
    #[serde(rename = "quotes")]
    pub quotes: Vec<PredictionsQuote>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetRFQBatchBestQuotesResponse {
    #[serde(rename = "best_quotes")]
    pub best_quotes: Box<PredictionsRFQBatchBestQuotes>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetRFQBatchQuotesResponse {
    #[serde(rename = "batch_quotes")]
    pub batch_quotes: Vec<PredictionsRFQBatchQuote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetRFQBatchResponse {
    #[serde(rename = "rfq_batch")]
    pub rfq_batch: Box<PredictionsRFQBatch>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetRFQBatchesResponse {
    #[serde(rename = "rfq_batches")]
    pub rfq_batches: Vec<PredictionsRFQBatch>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetRFQResponse {
    #[serde(rename = "rfq")]
    pub rfq: Box<PredictionsRFQ>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetRFQsResponse {
    #[serde(rename = "rfqs")]
    pub rfqs: Vec<PredictionsRFQ>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetSeriesFeeChangesResponse {
    #[serde(rename = "series_fee_change_arr")]
    pub series_fee_change_arr: Vec<PredictionsSeriesFeeChange>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetSeriesListResponse {
    #[serde(rename = "series")]
    pub series: Vec<PredictionsSeries>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetSeriesResponse {
    #[serde(rename = "series")]
    pub series: Box<PredictionsSeries>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetSettlementsResponse {
    #[serde(rename = "settlements")]
    pub settlements: Vec<PredictionsSettlement>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetStructuredTargetResponse {
    #[serde(rename = "structured_target", default, skip_serializing_if = "Option::is_none")]
    pub structured_target: Option<Box<PredictionsStructuredTarget>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetStructuredTargetsResponse {
    #[serde(rename = "structured_targets", default, skip_serializing_if = "Option::is_none")]
    pub structured_targets: Option<Vec<PredictionsStructuredTarget>>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetSubaccountBalancesResponse {
    #[serde(rename = "subaccount_balances")]
    pub subaccount_balances: Vec<PredictionsSubaccountBalance>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetSubaccountNettingResponse {
    #[serde(rename = "netting_configs")]
    pub netting_configs: Vec<PredictionsSubaccountNettingConfig>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetSubaccountTransfersResponse {
    #[serde(rename = "transfers")]
    pub transfers: Vec<PredictionsSubaccountTransfer>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetTagsForSeriesCategoriesResponse {
    #[serde(rename = "tags_by_categories")]
    pub tags_by_categories: std::collections::BTreeMap<String,Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetTargetBalanceAllocationResponse {
    #[serde(rename = "allocations")]
    pub allocations: Vec<PredictionsTargetBalanceAllocation>,
    #[serde(rename = "resting_margin_reservation")]
    pub resting_margin_reservation: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetTradesResponse {
    #[serde(rename = "trades")]
    pub trades: Vec<PredictionsTrade>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetUserDataTimestampResponse {
    #[serde(rename = "as_of_time")]
    pub as_of_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetWeatherIndexCalibrationsResponse {
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "units")]
    pub units: String,
    #[serde(rename = "calibrations")]
    pub calibrations: Vec<PredictionsWeatherIndexCalibration>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetWeatherIndexResponse {
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "config_version", default, skip_serializing_if = "Option::is_none")]
    pub config_version: Option<String>,
    #[serde(rename = "units")]
    pub units: String,
    #[serde(rename = "timeseries")]
    pub timeseries: Vec<PredictionsWeatherIndexPoint>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsGetWithdrawalsResponse {
    #[serde(rename = "withdrawals")]
    pub withdrawals: Vec<PredictionsWithdrawal>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsIncentiveProgram {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "market_id")]
    pub market_id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "incentive_type")]
    pub incentive_type: String,
    #[serde(rename = "incentive_description")]
    pub incentive_description: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(rename = "period_reward")]
    pub period_reward: serde_json::Number,
    #[serde(rename = "paid_out")]
    pub paid_out: bool,
    #[serde(rename = "discount_factor_bps", default, skip_serializing_if = "Option::is_none")]
    pub discount_factor_bps: Option<serde_json::Number>,
    #[serde(rename = "target_size_fp", default, skip_serializing_if = "Option::is_none")]
    pub target_size_fp: Option<String>,
    #[serde(rename = "max_reward_per_account", default, skip_serializing_if = "Option::is_none")]
    pub max_reward_per_account: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsIncentiveProgram {
    fn default() -> Self {
        Self {
            id: Default::default(),
            market_id: Default::default(),
            market_ticker: Default::default(),
            incentive_type: Default::default(),
            incentive_description: Default::default(),
            start_date: Default::default(),
            end_date: Default::default(),
            period_reward: 0.into(),
            paid_out: Default::default(),
            discount_factor_bps: Default::default(),
            target_size_fp: Default::default(),
            max_reward_per_account: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsIndexedBalance {
    #[serde(rename = "exchange_index")]
    pub exchange_index: serde_json::Number,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsIndexedBalance {
    fn default() -> Self {
        Self {
            exchange_index: 0.into(),
            balance: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsIntraExchangeInstanceTransfer {
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    #[serde(rename = "client_transfer_id", default, skip_serializing_if = "Option::is_none")]
    pub client_transfer_id: Option<String>,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "destination")]
    pub destination: String,
    #[serde(rename = "source_exchange_shard")]
    pub source_exchange_shard: serde_json::Number,
    #[serde(rename = "destination_exchange_shard")]
    pub destination_exchange_shard: serde_json::Number,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "created_ts")]
    pub created_ts: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsIntraExchangeInstanceTransfer {
    fn default() -> Self {
        Self {
            transfer_id: Default::default(),
            client_transfer_id: Default::default(),
            source: Default::default(),
            destination: Default::default(),
            source_exchange_shard: 0.into(),
            destination_exchange_shard: 0.into(),
            amount: Default::default(),
            status: Default::default(),
            created_ts: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsIntraExchangeInstanceTransferRequest {
    #[serde(rename = "client_transfer_id", default, skip_serializing_if = "Option::is_none")]
    pub client_transfer_id: Option<String>,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "destination")]
    pub destination: String,
    #[serde(rename = "amount")]
    pub amount: serde_json::Number,
    #[serde(rename = "source_exchange_shard", default, skip_serializing_if = "Option::is_none")]
    pub source_exchange_shard: Option<serde_json::Number>,
    #[serde(rename = "destination_exchange_shard", default, skip_serializing_if = "Option::is_none")]
    pub destination_exchange_shard: Option<serde_json::Number>,
    #[serde(rename = "source_subaccount", default, skip_serializing_if = "Option::is_none")]
    pub source_subaccount: Option<serde_json::Number>,
    #[serde(rename = "destination_subaccount", default, skip_serializing_if = "Option::is_none")]
    pub destination_subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsIntraExchangeInstanceTransferRequest {
    fn default() -> Self {
        Self {
            client_transfer_id: Default::default(),
            source: Default::default(),
            destination: Default::default(),
            amount: 0.into(),
            source_exchange_shard: Default::default(),
            destination_exchange_shard: Default::default(),
            source_subaccount: Default::default(),
            destination_subaccount: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsIntraExchangeInstanceTransferResponse {
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsListFCMSubtradersResponse {
    #[serde(rename = "subtraders")]
    pub subtraders: Vec<PredictionsFCMSubtrader>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsLiveData {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "details")]
    pub details: serde_json::Map<String,serde_json::Value>,
    #[serde(rename = "milestone_id")]
    pub milestone_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsMaintenanceWindow {
    #[serde(rename = "start_datetime")]
    pub start_datetime: String,
    #[serde(rename = "end_datetime")]
    pub end_datetime: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsMarket {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "market_type")]
    pub market_type: String,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "subtitle", default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "yes_sub_title")]
    pub yes_sub_title: String,
    #[serde(rename = "no_sub_title")]
    pub no_sub_title: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "updated_time")]
    pub updated_time: String,
    #[serde(rename = "open_time")]
    pub open_time: String,
    #[serde(rename = "close_time")]
    pub close_time: String,
    #[serde(rename = "expected_expiration_time", default, skip_serializing_if = "Option::is_none")]
    pub expected_expiration_time: Option<String>,
    #[serde(rename = "expiration_time", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(rename = "latest_expiration_time")]
    pub latest_expiration_time: String,
    #[serde(rename = "settlement_timer_seconds")]
    pub settlement_timer_seconds: serde_json::Number,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "yes_bid_dollars")]
    pub yes_bid_dollars: String,
    #[serde(rename = "yes_bid_size_fp")]
    pub yes_bid_size_fp: String,
    #[serde(rename = "yes_ask_dollars")]
    pub yes_ask_dollars: String,
    #[serde(rename = "yes_ask_size_fp")]
    pub yes_ask_size_fp: String,
    #[serde(rename = "no_bid_dollars")]
    pub no_bid_dollars: String,
    #[serde(rename = "no_ask_dollars")]
    pub no_ask_dollars: String,
    #[serde(rename = "last_price_dollars")]
    pub last_price_dollars: String,
    #[serde(rename = "volume_fp")]
    pub volume_fp: String,
    #[serde(rename = "volume_24h_fp")]
    pub volume_24h_fp: String,
    #[serde(rename = "result")]
    pub result: String,
    #[serde(rename = "can_close_early")]
    pub can_close_early: bool,
    #[serde(rename = "open_interest_fp")]
    pub open_interest_fp: String,
    #[serde(rename = "notional_value_dollars")]
    pub notional_value_dollars: String,
    #[serde(rename = "previous_yes_bid_dollars")]
    pub previous_yes_bid_dollars: String,
    #[serde(rename = "previous_yes_ask_dollars")]
    pub previous_yes_ask_dollars: String,
    #[serde(rename = "previous_price_dollars")]
    pub previous_price_dollars: String,
    #[serde(rename = "settlement_value_dollars", default, skip_serializing_if = "Option::is_none")]
    pub settlement_value_dollars: Option<String>,
    #[serde(rename = "settlement_ts", default, skip_serializing_if = "Option::is_none")]
    pub settlement_ts: Option<String>,
    #[serde(rename = "settlement_bounds_type")]
    pub settlement_bounds_type: String,
    #[serde(rename = "settlement_floor_dollars", default, skip_serializing_if = "Option::is_none")]
    pub settlement_floor_dollars: Option<String>,
    #[serde(rename = "expiration_value")]
    pub expiration_value: String,
    #[serde(rename = "occurrence_datetime", default, skip_serializing_if = "Option::is_none")]
    pub occurrence_datetime: Option<String>,
    #[serde(rename = "fee_waiver_expiration_time", default, skip_serializing_if = "Option::is_none")]
    pub fee_waiver_expiration_time: Option<String>,
    #[serde(rename = "early_close_condition", default, skip_serializing_if = "Option::is_none")]
    pub early_close_condition: Option<String>,
    #[serde(rename = "strike_type", default, skip_serializing_if = "Option::is_none")]
    pub strike_type: Option<String>,
    #[serde(rename = "floor_strike", default, skip_serializing_if = "Option::is_none")]
    pub floor_strike: Option<serde_json::Number>,
    #[serde(rename = "cap_strike", default, skip_serializing_if = "Option::is_none")]
    pub cap_strike: Option<serde_json::Number>,
    #[serde(rename = "functional_strike", default, skip_serializing_if = "Option::is_none")]
    pub functional_strike: Option<String>,
    #[serde(rename = "custom_strike", default, skip_serializing_if = "Option::is_none")]
    pub custom_strike: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(rename = "rules_primary")]
    pub rules_primary: String,
    #[serde(rename = "rules_secondary")]
    pub rules_secondary: String,
    #[serde(rename = "mve_collection_ticker", default, skip_serializing_if = "Option::is_none")]
    pub mve_collection_ticker: Option<String>,
    #[serde(rename = "mve_selected_legs", default, skip_serializing_if = "Option::is_none")]
    pub mve_selected_legs: Option<Vec<PredictionsMveSelectedLeg>>,
    #[serde(rename = "primary_participant_key", default, skip_serializing_if = "Option::is_none")]
    pub primary_participant_key: Option<String>,
    #[serde(rename = "price_level_structure")]
    pub price_level_structure: String,
    #[serde(rename = "price_ranges")]
    pub price_ranges: Vec<PredictionsPriceRange>,
    #[serde(rename = "is_provisional", default, skip_serializing_if = "Option::is_none")]
    pub is_provisional: Option<bool>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsMarket {
    fn default() -> Self {
        Self {
            ticker: Default::default(),
            event_ticker: Default::default(),
            market_type: Default::default(),
            title: Default::default(),
            subtitle: Default::default(),
            yes_sub_title: Default::default(),
            no_sub_title: Default::default(),
            created_time: Default::default(),
            updated_time: Default::default(),
            open_time: Default::default(),
            close_time: Default::default(),
            expected_expiration_time: Default::default(),
            expiration_time: Default::default(),
            latest_expiration_time: Default::default(),
            settlement_timer_seconds: 0.into(),
            status: Default::default(),
            yes_bid_dollars: Default::default(),
            yes_bid_size_fp: Default::default(),
            yes_ask_dollars: Default::default(),
            yes_ask_size_fp: Default::default(),
            no_bid_dollars: Default::default(),
            no_ask_dollars: Default::default(),
            last_price_dollars: Default::default(),
            volume_fp: Default::default(),
            volume_24h_fp: Default::default(),
            result: Default::default(),
            can_close_early: Default::default(),
            open_interest_fp: Default::default(),
            notional_value_dollars: Default::default(),
            previous_yes_bid_dollars: Default::default(),
            previous_yes_ask_dollars: Default::default(),
            previous_price_dollars: Default::default(),
            settlement_value_dollars: Default::default(),
            settlement_ts: Default::default(),
            settlement_bounds_type: Default::default(),
            settlement_floor_dollars: Default::default(),
            expiration_value: Default::default(),
            occurrence_datetime: Default::default(),
            fee_waiver_expiration_time: Default::default(),
            early_close_condition: Default::default(),
            strike_type: Default::default(),
            floor_strike: Default::default(),
            cap_strike: Default::default(),
            functional_strike: Default::default(),
            custom_strike: Default::default(),
            rules_primary: Default::default(),
            rules_secondary: Default::default(),
            mve_collection_ticker: Default::default(),
            mve_selected_legs: Default::default(),
            primary_participant_key: Default::default(),
            price_level_structure: Default::default(),
            price_ranges: Default::default(),
            is_provisional: Default::default(),
            exchange_index: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsMarketCandlestick {
    #[serde(rename = "end_period_ts")]
    pub end_period_ts: serde_json::Number,
    #[serde(rename = "yes_bid")]
    pub yes_bid: Box<PredictionsBidAskDistribution>,
    #[serde(rename = "yes_ask")]
    pub yes_ask: Box<PredictionsBidAskDistribution>,
    #[serde(rename = "price")]
    pub price: Box<PredictionsPriceDistribution>,
    #[serde(rename = "volume_fp")]
    pub volume_fp: String,
    #[serde(rename = "open_interest_fp")]
    pub open_interest_fp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsMarketCandlestick {
    fn default() -> Self {
        Self {
            end_period_ts: 0.into(),
            yes_bid: Default::default(),
            yes_ask: Default::default(),
            price: Default::default(),
            volume_fp: Default::default(),
            open_interest_fp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsMarketCandlestickHistorical {
    #[serde(rename = "end_period_ts")]
    pub end_period_ts: serde_json::Number,
    #[serde(rename = "yes_bid")]
    pub yes_bid: Box<PredictionsBidAskDistributionHistorical>,
    #[serde(rename = "yes_ask")]
    pub yes_ask: Box<PredictionsBidAskDistributionHistorical>,
    #[serde(rename = "price")]
    pub price: Box<PredictionsPriceDistributionHistorical>,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "open_interest")]
    pub open_interest: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsMarketCandlestickHistorical {
    fn default() -> Self {
        Self {
            end_period_ts: 0.into(),
            yes_bid: Default::default(),
            yes_ask: Default::default(),
            price: Default::default(),
            volume: Default::default(),
            open_interest: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsMarketCandlesticksResponse {
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "candlesticks")]
    pub candlesticks: Vec<PredictionsMarketCandlestick>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsMarketIndex {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "family")]
    pub family: String,
    #[serde(rename = "gics_code", default, skip_serializing_if = "Option::is_none")]
    pub gics_code: Option<String>,
    #[serde(rename = "sector", default, skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "units")]
    pub units: String,
    #[serde(rename = "session")]
    pub session: Box<PredictionsMarketIndexSession>,
    #[serde(rename = "available")]
    pub available: bool,
    #[serde(rename = "latest", default, skip_serializing_if = "Option::is_none")]
    pub latest: Option<Box<PredictionsMarketIndexLatest>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsMarketIndexLatest {
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Number>,
    #[serde(rename = "last_tick_ms", default, skip_serializing_if = "Option::is_none")]
    pub last_tick_ms: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsMarketIndexSession {
    #[serde(rename = "timezone")]
    pub timezone: String,
    #[serde(rename = "open")]
    pub open: String,
    #[serde(rename = "close")]
    pub close: String,
    #[serde(rename = "days")]
    pub days: Vec<String>,
    #[serde(rename = "calendar", default, skip_serializing_if = "Option::is_none")]
    pub calendar: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsMarketIndexTimeseriesPoint {
    #[serde(rename = "t")]
    pub t: serde_json::Number,
    #[serde(rename = "v")]
    pub v: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsMarketIndexTimeseriesPoint {
    fn default() -> Self {
        Self {
            t: 0.into(),
            v: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsMarketMetadata {
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "color_code")]
    pub color_code: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsMarketOrderbookFp {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "orderbook_fp")]
    pub orderbook_fp: Box<PredictionsOrderbookCountFp>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsMarketPosition {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "exchange_index")]
    pub exchange_index: serde_json::Number,
    #[serde(rename = "total_traded_dollars")]
    pub total_traded_dollars: String,
    #[serde(rename = "position_fp")]
    pub position_fp: String,
    #[serde(rename = "market_exposure_dollars")]
    pub market_exposure_dollars: String,
    #[serde(rename = "realized_pnl_dollars")]
    pub realized_pnl_dollars: String,
    #[serde(rename = "fees_paid_dollars")]
    pub fees_paid_dollars: String,
    #[serde(rename = "last_updated_ts")]
    pub last_updated_ts: String,
    #[serde(rename = "netting_enabled")]
    pub netting_enabled: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsMarketPosition {
    fn default() -> Self {
        Self {
            ticker: Default::default(),
            exchange_index: 0.into(),
            total_traded_dollars: Default::default(),
            position_fp: Default::default(),
            market_exposure_dollars: Default::default(),
            realized_pnl_dollars: Default::default(),
            fees_paid_dollars: Default::default(),
            last_updated_ts: Default::default(),
            netting_enabled: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsMilestone {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "related_event_tickers")]
    pub related_event_tickers: Vec<String>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "notification_message")]
    pub notification_message: String,
    #[serde(rename = "source_id", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "source_ids", default, skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<std::collections::BTreeMap<String,String>>,
    #[serde(rename = "details")]
    pub details: serde_json::Map<String,serde_json::Value>,
    #[serde(rename = "primary_event_tickers")]
    pub primary_event_tickers: Vec<String>,
    #[serde(rename = "last_updated_ts")]
    pub last_updated_ts: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsMultivariateEventCollection {
    #[serde(rename = "collection_ticker")]
    pub collection_ticker: String,
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "open_date")]
    pub open_date: String,
    #[serde(rename = "close_date")]
    pub close_date: String,
    #[serde(rename = "associated_events")]
    pub associated_events: Vec<PredictionsAssociatedEvent>,
    #[serde(rename = "associated_event_tickers")]
    pub associated_event_tickers: Vec<String>,
    #[serde(rename = "is_ordered")]
    pub is_ordered: bool,
    #[serde(rename = "is_single_market_per_event")]
    pub is_single_market_per_event: bool,
    #[serde(rename = "is_all_yes")]
    pub is_all_yes: bool,
    #[serde(rename = "size_min")]
    pub size_min: serde_json::Number,
    #[serde(rename = "size_max")]
    pub size_max: serde_json::Number,
    #[serde(rename = "functional_description")]
    pub functional_description: String,
    #[serde(rename = "price_level_structure")]
    pub price_level_structure: String,
    #[serde(rename = "price_ranges")]
    pub price_ranges: Vec<PredictionsPriceRange>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsMultivariateEventCollection {
    fn default() -> Self {
        Self {
            collection_ticker: Default::default(),
            series_ticker: Default::default(),
            exchange_index: Default::default(),
            title: Default::default(),
            description: Default::default(),
            open_date: Default::default(),
            close_date: Default::default(),
            associated_events: Default::default(),
            associated_event_tickers: Default::default(),
            is_ordered: Default::default(),
            is_single_market_per_event: Default::default(),
            is_all_yes: Default::default(),
            size_min: 0.into(),
            size_max: 0.into(),
            functional_description: Default::default(),
            price_level_structure: Default::default(),
            price_ranges: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsMveSelectedLeg {
    #[serde(rename = "event_ticker", default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "yes_settlement_value_dollars", default, skip_serializing_if = "Option::is_none")]
    pub yes_settlement_value_dollars: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsOrder {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "outcome_side")]
    pub outcome_side: String,
    #[serde(rename = "book_side")]
    pub book_side: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "yes_price_dollars")]
    pub yes_price_dollars: String,
    #[serde(rename = "no_price_dollars")]
    pub no_price_dollars: String,
    #[serde(rename = "fill_count_fp")]
    pub fill_count_fp: String,
    #[serde(rename = "remaining_count_fp")]
    pub remaining_count_fp: String,
    #[serde(rename = "initial_count_fp")]
    pub initial_count_fp: String,
    #[serde(rename = "taker_fill_cost_dollars")]
    pub taker_fill_cost_dollars: String,
    #[serde(rename = "maker_fill_cost_dollars")]
    pub maker_fill_cost_dollars: String,
    #[serde(rename = "taker_fees_dollars")]
    pub taker_fees_dollars: String,
    #[serde(rename = "maker_fees_dollars")]
    pub maker_fees_dollars: String,
    #[serde(rename = "expiration_time", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(rename = "created_time", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "last_update_time", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(rename = "self_trade_prevention_type", default, skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_type: Option<String>,
    #[serde(rename = "order_group_id", default, skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(rename = "cancel_order_on_pause", default, skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(rename = "subaccount_number", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_number: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsOrderGroup {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "contracts_limit_fp", default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<String>,
    #[serde(rename = "is_auto_cancel_enabled")]
    pub is_auto_cancel_enabled: bool,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsOrderQueuePosition {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "queue_position_fp")]
    pub queue_position_fp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsOrderbookCountFp {
    #[serde(rename = "yes_dollars")]
    pub yes_dollars: Vec<PredictionsPriceLevelDollarsCountFp>,
    #[serde(rename = "no_dollars")]
    pub no_dollars: Vec<PredictionsPriceLevelDollarsCountFp>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsPercentilePoint {
    #[serde(rename = "percentile")]
    pub percentile: serde_json::Number,
    #[serde(rename = "raw_numerical_forecast")]
    pub raw_numerical_forecast: serde_json::Number,
    #[serde(rename = "numerical_forecast")]
    pub numerical_forecast: serde_json::Number,
    #[serde(rename = "formatted_forecast")]
    pub formatted_forecast: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsPercentilePoint {
    fn default() -> Self {
        Self {
            percentile: 0.into(),
            raw_numerical_forecast: 0.into(),
            numerical_forecast: 0.into(),
            formatted_forecast: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsPlayByPlay {
    #[serde(rename = "periods", default, skip_serializing_if = "Option::is_none")]
    pub periods: Option<Vec<PredictionsPlayByPlayPeriodsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsPlayByPlayPeriodsItem {
    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<serde_json::Map<String,serde_json::Value>>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsPriceDistribution {
    #[serde(rename = "open_dollars", default, skip_serializing_if = "Option::is_none")]
    pub open_dollars: Option<String>,
    #[serde(rename = "low_dollars", default, skip_serializing_if = "Option::is_none")]
    pub low_dollars: Option<String>,
    #[serde(rename = "high_dollars", default, skip_serializing_if = "Option::is_none")]
    pub high_dollars: Option<String>,
    #[serde(rename = "close_dollars", default, skip_serializing_if = "Option::is_none")]
    pub close_dollars: Option<String>,
    #[serde(rename = "mean_dollars", default, skip_serializing_if = "Option::is_none")]
    pub mean_dollars: Option<String>,
    #[serde(rename = "previous_dollars", default, skip_serializing_if = "Option::is_none")]
    pub previous_dollars: Option<String>,
    #[serde(rename = "min_dollars", default, skip_serializing_if = "Option::is_none")]
    pub min_dollars: Option<String>,
    #[serde(rename = "max_dollars", default, skip_serializing_if = "Option::is_none")]
    pub max_dollars: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsPriceDistributionHistorical {
    #[serde(rename = "open", skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,
    #[serde(rename = "low", skip_serializing_if = "Option::is_none")]
    pub low: Option<String>,
    #[serde(rename = "high", skip_serializing_if = "Option::is_none")]
    pub high: Option<String>,
    #[serde(rename = "close", skip_serializing_if = "Option::is_none")]
    pub close: Option<String>,
    #[serde(rename = "mean", skip_serializing_if = "Option::is_none")]
    pub mean: Option<String>,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsPriceRange {
    #[serde(rename = "start")]
    pub start: String,
    #[serde(rename = "end")]
    pub end: String,
    #[serde(rename = "step")]
    pub step: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsProposeBlockTradeRequest {
    #[serde(rename = "buyer_user_id")]
    pub buyer_user_id: String,
    #[serde(rename = "buyer_subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub buyer_subtrader_id: Option<String>,
    #[serde(rename = "buyer_subaccount", default, skip_serializing_if = "Option::is_none")]
    pub buyer_subaccount: Option<serde_json::Number>,
    #[serde(rename = "seller_user_id")]
    pub seller_user_id: String,
    #[serde(rename = "seller_subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub seller_subtrader_id: Option<String>,
    #[serde(rename = "seller_subaccount", default, skip_serializing_if = "Option::is_none")]
    pub seller_subaccount: Option<serde_json::Number>,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "price_centi_cents")]
    pub price_centi_cents: serde_json::Number,
    #[serde(rename = "centicount")]
    pub centicount: serde_json::Number,
    #[serde(rename = "maker_side")]
    pub maker_side: String,
    #[serde(rename = "expiration_ts")]
    pub expiration_ts: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsProposeBlockTradeRequest {
    fn default() -> Self {
        Self {
            buyer_user_id: Default::default(),
            buyer_subtrader_id: Default::default(),
            buyer_subaccount: Default::default(),
            seller_user_id: Default::default(),
            seller_subtrader_id: Default::default(),
            seller_subaccount: Default::default(),
            market_ticker: Default::default(),
            price_centi_cents: 0.into(),
            centicount: 0.into(),
            maker_side: Default::default(),
            expiration_ts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsProposeBlockTradeResponse {
    #[serde(rename = "block_trade_proposal_id")]
    pub block_trade_proposal_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsQuote {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "creator_id")]
    pub creator_id: String,
    #[serde(rename = "rfq_creator_id")]
    pub rfq_creator_id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "contracts_fp")]
    pub contracts_fp: String,
    #[serde(rename = "yes_bid_dollars")]
    pub yes_bid_dollars: String,
    #[serde(rename = "no_bid_dollars")]
    pub no_bid_dollars: String,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(rename = "updated_ts")]
    pub updated_ts: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "accepted_side", default, skip_serializing_if = "Option::is_none")]
    pub accepted_side: Option<String>,
    #[serde(rename = "accepted_ts", default, skip_serializing_if = "Option::is_none")]
    pub accepted_ts: Option<String>,
    #[serde(rename = "confirmed_ts", default, skip_serializing_if = "Option::is_none")]
    pub confirmed_ts: Option<String>,
    #[serde(rename = "executed_ts", default, skip_serializing_if = "Option::is_none")]
    pub executed_ts: Option<String>,
    #[serde(rename = "cancelled_ts", default, skip_serializing_if = "Option::is_none")]
    pub cancelled_ts: Option<String>,
    #[serde(rename = "rest_remainder", default, skip_serializing_if = "Option::is_none")]
    pub rest_remainder: Option<bool>,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "cancellation_reason", default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[serde(rename = "creator_user_id", default, skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    #[serde(rename = "rfq_creator_user_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_creator_user_id: Option<String>,
    #[serde(rename = "rfq_target_cost_dollars", default, skip_serializing_if = "Option::is_none")]
    pub rfq_target_cost_dollars: Option<String>,
    #[serde(rename = "target_cost_excludes_fees", default, skip_serializing_if = "Option::is_none")]
    pub target_cost_excludes_fees: Option<bool>,
    #[serde(rename = "rfq_creator_order_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_creator_order_id: Option<String>,
    #[serde(rename = "creator_order_id", default, skip_serializing_if = "Option::is_none")]
    pub creator_order_id: Option<String>,
    #[serde(rename = "creator_subaccount", default, skip_serializing_if = "Option::is_none")]
    pub creator_subaccount: Option<serde_json::Number>,
    #[serde(rename = "rfq_creator_subaccount", default, skip_serializing_if = "Option::is_none")]
    pub rfq_creator_subaccount: Option<serde_json::Number>,
    #[serde(rename = "yes_contracts_fp", default, skip_serializing_if = "Option::is_none")]
    pub yes_contracts_fp: Option<String>,
    #[serde(rename = "no_contracts_fp", default, skip_serializing_if = "Option::is_none")]
    pub no_contracts_fp: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsRFQ {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "creator_id")]
    pub creator_id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "contracts_fp")]
    pub contracts_fp: String,
    #[serde(rename = "target_cost_dollars", default, skip_serializing_if = "Option::is_none")]
    pub target_cost_dollars: Option<String>,
    #[serde(rename = "target_cost_excludes_fees", default, skip_serializing_if = "Option::is_none")]
    pub target_cost_excludes_fees: Option<bool>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(rename = "mve_collection_ticker", default, skip_serializing_if = "Option::is_none")]
    pub mve_collection_ticker: Option<String>,
    #[serde(rename = "mve_selected_legs", default, skip_serializing_if = "Option::is_none")]
    pub mve_selected_legs: Option<Vec<PredictionsMveSelectedLeg>>,
    #[serde(rename = "rest_remainder", default, skip_serializing_if = "Option::is_none")]
    pub rest_remainder: Option<bool>,
    #[serde(rename = "cancellation_reason", default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[serde(rename = "creator_user_id", default, skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    #[serde(rename = "creator_subaccount", default, skip_serializing_if = "Option::is_none")]
    pub creator_subaccount: Option<serde_json::Number>,
    #[serde(rename = "cancelled_ts", default, skip_serializing_if = "Option::is_none")]
    pub cancelled_ts: Option<String>,
    #[serde(rename = "updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub updated_ts: Option<String>,
    #[serde(rename = "rfq_batch_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_batch_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsRFQBatch {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "creator_id")]
    pub creator_id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(rename = "accepted_ts", default, skip_serializing_if = "Option::is_none")]
    pub accepted_ts: Option<String>,
    #[serde(rename = "closed_ts", default, skip_serializing_if = "Option::is_none")]
    pub closed_ts: Option<String>,
    #[serde(rename = "rfqs")]
    pub rfqs: Vec<PredictionsRFQ>,
    #[serde(rename = "updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub updated_ts: Option<String>,
    #[serde(rename = "close_reason", default, skip_serializing_if = "Option::is_none")]
    pub close_reason: Option<String>,
    #[serde(rename = "client_batch_id", default, skip_serializing_if = "Option::is_none")]
    pub client_batch_id: Option<String>,
    #[serde(rename = "creator_user_id", default, skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    #[serde(rename = "creator_subaccount", default, skip_serializing_if = "Option::is_none")]
    pub creator_subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsRFQBatchAcceptLimit {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "bid_dollars")]
    pub bid_dollars: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsRFQBatchBestQuote {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "yes_bid_dollars", default, skip_serializing_if = "Option::is_none")]
    pub yes_bid_dollars: Option<String>,
    #[serde(rename = "no_bid_dollars", default, skip_serializing_if = "Option::is_none")]
    pub no_bid_dollars: Option<String>,
    #[serde(rename = "yes_contracts_offered_fp", default, skip_serializing_if = "Option::is_none")]
    pub yes_contracts_offered_fp: Option<String>,
    #[serde(rename = "no_contracts_offered_fp", default, skip_serializing_if = "Option::is_none")]
    pub no_contracts_offered_fp: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsRFQBatchBestQuotes {
    #[serde(rename = "rfq_batch_id")]
    pub rfq_batch_id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "version")]
    pub version: serde_json::Number,
    #[serde(rename = "updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub updated_ts: Option<String>,
    #[serde(rename = "yes_complete")]
    pub yes_complete: bool,
    #[serde(rename = "no_complete")]
    pub no_complete: bool,
    #[serde(rename = "rfqs")]
    pub rfqs: Vec<PredictionsRFQBatchBestQuote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsRFQBatchBestQuotes {
    fn default() -> Self {
        Self {
            rfq_batch_id: Default::default(),
            status: Default::default(),
            version: 0.into(),
            updated_ts: Default::default(),
            yes_complete: Default::default(),
            no_complete: Default::default(),
            rfqs: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsRFQBatchQuote {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "rfq_batch_id")]
    pub rfq_batch_id: String,
    #[serde(rename = "creator_id")]
    pub creator_id: String,
    #[serde(rename = "rfq_creator_id")]
    pub rfq_creator_id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(rename = "updated_ts")]
    pub updated_ts: String,
    #[serde(rename = "accepted_side", default, skip_serializing_if = "Option::is_none")]
    pub accepted_side: Option<String>,
    #[serde(rename = "accepted_ts", default, skip_serializing_if = "Option::is_none")]
    pub accepted_ts: Option<String>,
    #[serde(rename = "confirmed_ts", default, skip_serializing_if = "Option::is_none")]
    pub confirmed_ts: Option<String>,
    #[serde(rename = "quotes")]
    pub quotes: Vec<PredictionsQuote>,
    #[serde(rename = "rest_remainder", default, skip_serializing_if = "Option::is_none")]
    pub rest_remainder: Option<bool>,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "creator_user_id", default, skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    #[serde(rename = "creator_subaccount", default, skip_serializing_if = "Option::is_none")]
    pub creator_subaccount: Option<serde_json::Number>,
    #[serde(rename = "rfq_creator_user_id", default, skip_serializing_if = "Option::is_none")]
    pub rfq_creator_user_id: Option<String>,
    #[serde(rename = "rfq_creator_subaccount", default, skip_serializing_if = "Option::is_none")]
    pub rfq_creator_subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsRFQBatchQuoteEntry {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "yes_bid")]
    pub yes_bid: String,
    #[serde(rename = "no_bid")]
    pub no_bid: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsSchedule {
    #[serde(rename = "standard_hours")]
    pub standard_hours: Vec<PredictionsWeeklySchedule>,
    #[serde(rename = "maintenance_windows")]
    pub maintenance_windows: Vec<PredictionsMaintenanceWindow>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsScopeList {
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsSeries {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "frequency")]
    pub frequency: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "categories")]
    pub categories: Vec<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "settlement_sources", skip_serializing_if = "Option::is_none")]
    pub settlement_sources: Option<Vec<PredictionsSettlementSource>>,
    #[serde(rename = "contract_url")]
    pub contract_url: String,
    #[serde(rename = "contract_terms_url")]
    pub contract_terms_url: String,
    #[serde(rename = "product_metadata", default, skip_serializing_if = "Option::is_none")]
    pub product_metadata: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(rename = "fee_type")]
    pub fee_type: String,
    #[serde(rename = "fee_multiplier")]
    pub fee_multiplier: serde_json::Number,
    #[serde(rename = "additional_prohibitions", skip_serializing_if = "Option::is_none")]
    pub additional_prohibitions: Option<Vec<String>>,
    #[serde(rename = "volume_fp", default, skip_serializing_if = "Option::is_none")]
    pub volume_fp: Option<String>,
    #[serde(rename = "last_updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_ts: Option<String>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsSeries {
    fn default() -> Self {
        Self {
            ticker: Default::default(),
            frequency: Default::default(),
            title: Default::default(),
            category: Default::default(),
            categories: Default::default(),
            tags: Default::default(),
            settlement_sources: Default::default(),
            contract_url: Default::default(),
            contract_terms_url: Default::default(),
            product_metadata: Default::default(),
            fee_type: Default::default(),
            fee_multiplier: 0.into(),
            additional_prohibitions: Default::default(),
            volume_fp: Default::default(),
            last_updated_ts: Default::default(),
            exchange_index: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsSeriesFeeChange {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    #[serde(rename = "fee_type")]
    pub fee_type: String,
    #[serde(rename = "fee_multiplier")]
    pub fee_multiplier: serde_json::Number,
    #[serde(rename = "scheduled_ts")]
    pub scheduled_ts: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsSeriesFeeChange {
    fn default() -> Self {
        Self {
            id: Default::default(),
            series_ticker: Default::default(),
            fee_type: Default::default(),
            fee_multiplier: 0.into(),
            scheduled_ts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsSetEventContractExitTriggerRequest {
    #[serde(rename = "slippage", default, skip_serializing_if = "Option::is_none")]
    pub slippage: Option<Box<PredictionsEventContractExitSlippage>>,
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "stop_loss_price", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "take_profit_price", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsSetTargetBalanceAllocationRequest {
    #[serde(rename = "allocations")]
    pub allocations: Vec<PredictionsTargetBalanceAllocationInput>,
    #[serde(rename = "resting_margin_reservation", default, skip_serializing_if = "Option::is_none")]
    pub resting_margin_reservation: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsSettlement {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "exchange_index")]
    pub exchange_index: serde_json::Number,
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "market_result")]
    pub market_result: String,
    #[serde(rename = "yes_count_fp")]
    pub yes_count_fp: String,
    #[serde(rename = "yes_total_cost_dollars")]
    pub yes_total_cost_dollars: String,
    #[serde(rename = "no_count_fp")]
    pub no_count_fp: String,
    #[serde(rename = "no_total_cost_dollars")]
    pub no_total_cost_dollars: String,
    #[serde(rename = "revenue")]
    pub revenue: serde_json::Number,
    #[serde(rename = "settled_time")]
    pub settled_time: String,
    #[serde(rename = "fee_cost")]
    pub fee_cost: String,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsSettlement {
    fn default() -> Self {
        Self {
            ticker: Default::default(),
            exchange_index: 0.into(),
            event_ticker: Default::default(),
            market_result: Default::default(),
            yes_count_fp: Default::default(),
            yes_total_cost_dollars: Default::default(),
            no_count_fp: Default::default(),
            no_total_cost_dollars: Default::default(),
            revenue: 0.into(),
            settled_time: Default::default(),
            fee_cost: Default::default(),
            value: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsSettlementSource {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsSportFilterDetails {
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    #[serde(rename = "competitions")]
    pub competitions: std::collections::BTreeMap<String,Box<PredictionsScopeList>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsStructuredTarget {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(rename = "source_id", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "source_ids", default, skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<std::collections::BTreeMap<String,String>>,
    #[serde(rename = "last_updated_ts", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_ts: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsSubaccountBalance {
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: serde_json::Number,
    #[serde(rename = "exchange_index")]
    pub exchange_index: serde_json::Number,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "updated_ts")]
    pub updated_ts: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsSubaccountBalance {
    fn default() -> Self {
        Self {
            subaccount_number: 0.into(),
            exchange_index: 0.into(),
            balance: Default::default(),
            updated_ts: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsSubaccountNettingConfig {
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: serde_json::Number,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "exchange_index")]
    pub exchange_index: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsSubaccountNettingConfig {
    fn default() -> Self {
        Self {
            subaccount_number: 0.into(),
            enabled: Default::default(),
            exchange_index: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsSubaccountTransfer {
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    #[serde(rename = "from_subaccount")]
    pub from_subaccount: serde_json::Number,
    #[serde(rename = "to_subaccount")]
    pub to_subaccount: serde_json::Number,
    #[serde(rename = "amount_cents")]
    pub amount_cents: serde_json::Number,
    #[serde(rename = "created_ts")]
    pub created_ts: serde_json::Number,
    #[serde(rename = "exchange_index")]
    pub exchange_index: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsSubaccountTransfer {
    fn default() -> Self {
        Self {
            transfer_id: Default::default(),
            from_subaccount: 0.into(),
            to_subaccount: 0.into(),
            amount_cents: 0.into(),
            created_ts: 0.into(),
            exchange_index: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsTargetBalanceAllocation {
    #[serde(rename = "exchange_index")]
    pub exchange_index: serde_json::Number,
    #[serde(rename = "percent")]
    pub percent: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsTargetBalanceAllocation {
    fn default() -> Self {
        Self {
            exchange_index: 0.into(),
            percent: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsTargetBalanceAllocationInput {
    #[serde(rename = "exchange_index")]
    pub exchange_index: serde_json::Number,
    #[serde(rename = "percent")]
    pub percent: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsTargetBalanceAllocationInput {
    fn default() -> Self {
        Self {
            exchange_index: 0.into(),
            percent: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsTickerPair {
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsTrade {
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "count_fp")]
    pub count_fp: String,
    #[serde(rename = "yes_price_dollars")]
    pub yes_price_dollars: String,
    #[serde(rename = "no_price_dollars")]
    pub no_price_dollars: String,
    #[serde(rename = "taker_side", default, skip_serializing_if = "Option::is_none")]
    pub taker_side: Option<String>,
    #[serde(rename = "taker_outcome_side")]
    pub taker_outcome_side: String,
    #[serde(rename = "taker_book_side")]
    pub taker_book_side: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "is_block_trade")]
    pub is_block_trade: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsUpdateFCMEventContractDailyCapRequest {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "limit")]
    pub limit: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsUpdateFCMSubtraderBlockedCategoriesRequest {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "blocked")]
    pub blocked: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsUpdateFCMSubtraderBlockedCategoriesResponse {
    #[serde(rename = "categories")]
    pub categories: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsUpdateOrderGroupLimitRequest {
    #[serde(rename = "contracts_limit", default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit: Option<serde_json::Number>,
    #[serde(rename = "contracts_limit_fp", default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsUpdateSubaccountNettingRequest {
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: serde_json::Number,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsUpdateSubaccountNettingRequest {
    fn default() -> Self {
        Self {
            subaccount_number: 0.into(),
            enabled: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsWeatherIndexCalibration {
    #[serde(rename = "config_version")]
    pub config_version: String,
    #[serde(rename = "published_at_ms", default, skip_serializing_if = "Option::is_none")]
    pub published_at_ms: Option<serde_json::Number>,
    #[serde(rename = "effective_at_ms")]
    pub effective_at_ms: serde_json::Number,
    #[serde(rename = "change_reason", default, skip_serializing_if = "Option::is_none")]
    pub change_reason: Option<String>,
    #[serde(rename = "calibration_window_start_ms", default, skip_serializing_if = "Option::is_none")]
    pub calibration_window_start_ms: Option<serde_json::Number>,
    #[serde(rename = "calibration_window_end_ms", default, skip_serializing_if = "Option::is_none")]
    pub calibration_window_end_ms: Option<serde_json::Number>,
    #[serde(rename = "city_reference_c")]
    pub city_reference_c: serde_json::Number,
    #[serde(rename = "stations")]
    pub stations: Vec<PredictionsWeatherIndexCalibrationStation>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsWeatherIndexCalibration {
    fn default() -> Self {
        Self {
            config_version: Default::default(),
            published_at_ms: Default::default(),
            effective_at_ms: 0.into(),
            change_reason: Default::default(),
            calibration_window_start_ms: Default::default(),
            calibration_window_end_ms: Default::default(),
            city_reference_c: 0.into(),
            stations: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsWeatherIndexCalibrationStation {
    #[serde(rename = "station_id")]
    pub station_id: String,
    #[serde(rename = "weight")]
    pub weight: serde_json::Number,
    #[serde(rename = "offset_c")]
    pub offset_c: serde_json::Number,
    #[serde(rename = "update_note", default, skip_serializing_if = "Option::is_none")]
    pub update_note: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsWeatherIndexCalibrationStation {
    fn default() -> Self {
        Self {
            station_id: Default::default(),
            weight: 0.into(),
            offset_c: 0.into(),
            update_note: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsWeatherIndexPoint {
    #[serde(rename = "t")]
    pub t: serde_json::Number,
    #[serde(rename = "v", default, skip_serializing_if = "Option::is_none")]
    pub v: Option<serde_json::Number>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "contributors", default, skip_serializing_if = "Option::is_none")]
    pub contributors: Option<serde_json::Number>,
    #[serde(rename = "receipt_basis", default, skip_serializing_if = "Option::is_none")]
    pub receipt_basis: Option<String>,
    #[serde(rename = "stations", default, skip_serializing_if = "Option::is_none")]
    pub stations: Option<Vec<PredictionsWeatherIndexStationReading>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsWeatherIndexPoint {
    fn default() -> Self {
        Self {
            t: 0.into(),
            v: Default::default(),
            status: Default::default(),
            contributors: Default::default(),
            receipt_basis: Default::default(),
            stations: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsWeatherIndexStationReading {
    #[serde(rename = "station_id")]
    pub station_id: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "temp_f", default, skip_serializing_if = "Option::is_none")]
    pub temp_f: Option<serde_json::Number>,
    #[serde(rename = "obs_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub obs_time_ms: Option<serde_json::Number>,
    #[serde(rename = "received_at_ms", default, skip_serializing_if = "Option::is_none")]
    pub received_at_ms: Option<serde_json::Number>,
    #[serde(rename = "primary_code", default, skip_serializing_if = "Option::is_none")]
    pub primary_code: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PredictionsWeeklySchedule {
    #[serde(rename = "start_time")]
    pub start_time: String,
    #[serde(rename = "end_time")]
    pub end_time: String,
    #[serde(rename = "monday")]
    pub monday: Vec<PredictionsDailySchedule>,
    #[serde(rename = "tuesday")]
    pub tuesday: Vec<PredictionsDailySchedule>,
    #[serde(rename = "wednesday")]
    pub wednesday: Vec<PredictionsDailySchedule>,
    #[serde(rename = "thursday")]
    pub thursday: Vec<PredictionsDailySchedule>,
    #[serde(rename = "friday")]
    pub friday: Vec<PredictionsDailySchedule>,
    #[serde(rename = "saturday")]
    pub saturday: Vec<PredictionsDailySchedule>,
    #[serde(rename = "sunday")]
    pub sunday: Vec<PredictionsDailySchedule>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PredictionsWithdrawal {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "amount_cents")]
    pub amount_cents: serde_json::Number,
    #[serde(rename = "fee_cents")]
    pub fee_cents: serde_json::Number,
    #[serde(rename = "created_ts")]
    pub created_ts: serde_json::Number,
    #[serde(rename = "finalized_ts", default, skip_serializing_if = "Option::is_none")]
    pub finalized_ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PredictionsWithdrawal {
    fn default() -> Self {
        Self {
            id: Default::default(),
            status: Default::default(),
            type_: Default::default(),
            amount_cents: 0.into(),
            fee_cents: 0.into(),
            created_ts: 0.into(),
            finalized_ts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ProposeBlockTradeParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsProposeBlockTradeRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ProposeBlockTradeResponse {
    #[serde(rename = "block_trade_proposal_id")]
    pub block_trade_proposal_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResetOrderGroupParams {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PredictionsEmptyResponse>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResetOrderGroupResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetEventContractExitTriggerParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "body")]
    pub body: Box<PredictionsSetEventContractExitTriggerRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SetEventContractExitTriggerResponse {
    #[serde(rename = "slippage", default, skip_serializing_if = "Option::is_none")]
    pub slippage: Option<Box<PredictionsEventContractExitSlippage>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: serde_json::Number,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "stop_loss_price", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "take_profit_price", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "status_reason", default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "triggered_order_id", default, skip_serializing_if = "Option::is_none")]
    pub triggered_order_id: Option<String>,
    #[serde(rename = "triggered_leg", default, skip_serializing_if = "Option::is_none")]
    pub triggered_leg: Option<String>,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "updated_time")]
    pub updated_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for SetEventContractExitTriggerResponse {
    fn default() -> Self {
        Self {
            slippage: Default::default(),
            id: Default::default(),
            ticker: Default::default(),
            side: Default::default(),
            subaccount_number: 0.into(),
            kind: Default::default(),
            stop_loss_price: Default::default(),
            take_profit_price: Default::default(),
            status: Default::default(),
            status_reason: Default::default(),
            triggered_order_id: Default::default(),
            triggered_leg: Default::default(),
            created_time: Default::default(),
            updated_time: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetTargetBalanceAllocationParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsSetTargetBalanceAllocationRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetTargetBalanceAllocationResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TriggerOrderGroupParams {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PredictionsEmptyResponse>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TriggerOrderGroupResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateFcmEventContractDailyCapParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsUpdateFCMEventContractDailyCapRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateFcmEventContractDailyCapResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateFcmSubtraderBlockedCategoriesParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsUpdateFCMSubtraderBlockedCategoriesRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateFcmSubtraderBlockedCategoriesResponse {
    #[serde(rename = "categories")]
    pub categories: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateOrderGroupLimitParams {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(rename = "body")]
    pub body: Box<PredictionsUpdateOrderGroupLimitRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateOrderGroupLimitResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateSubaccountNettingParams {
    #[serde(rename = "body")]
    pub body: Box<PredictionsUpdateSubaccountNettingRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpgradeAccountApiUsageLevelParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedKalshiClient(KalshiClient);
impl KalshiClient {
    pub fn typed(self) -> TypedKalshiClient { TypedKalshiClient(self) }
}
impl TypedKalshiClient {
    pub fn accept_block_trade_proposal(&self, request: &AcceptBlockTradeProposalParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.accept_block_trade_proposal(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn accept_quote(&self, request: &AcceptQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.accept_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn accept_rfq_batch(&self, request: &AcceptRfqBatchParams) -> Result<Box<PredictionsAcceptRFQBatchResponse>> {
        serde_json::from_value(self.0.accept_rfq_batch(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn accept_rfq_quote(&self, request: &AcceptRfqQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.accept_rfq_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn amend_order_v2(&self, request: &AmendOrderV2Params) -> Result<Box<PredictionsAmendOrderV2Response>> {
        serde_json::from_value(self.0.amend_order_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn apply_subaccount_transfer(&self, request: &ApplySubaccountTransferParams) -> Result<Box<PredictionsApplySubaccountTransferResponse>> {
        serde_json::from_value(self.0.apply_subaccount_transfer(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn batch_cancel_orders_v2(&self, request: &BatchCancelOrdersV2Params) -> Result<Box<PredictionsBatchCancelOrdersV2Response>> {
        serde_json::from_value(self.0.batch_cancel_orders_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn batch_create_orders_v2(&self, request: &BatchCreateOrdersV2Params) -> Result<Box<PredictionsBatchCreateOrdersV2Response>> {
        serde_json::from_value(self.0.batch_create_orders_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn batch_get_market_candlesticks(&self, request: &BatchGetMarketCandlesticksParams) -> Result<Box<PredictionsBatchGetMarketCandlesticksResponse>> {
        serde_json::from_value(self.0.batch_get_market_candlesticks(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn cancel_all_orders(&self, request: &CancelAllOrdersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn cancel_order_v2(&self, request: &CancelOrderV2Params) -> Result<Box<PredictionsCancelOrderV2Response>> {
        serde_json::from_value(self.0.cancel_order_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn confirm_quote(&self, request: &ConfirmQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.confirm_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn confirm_rfq_batch_quote(&self, request: &ConfirmRfqBatchQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.confirm_rfq_batch_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn confirm_rfq_quote(&self, request: &ConfirmRfqQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.confirm_rfq_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn create_api_key(&self, request: &CreateApiKeyParams) -> Result<Box<PredictionsCreateApiKeyResponse>> {
        serde_json::from_value(self.0.create_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn create_fcm_position_transfer(&self, request: &CreateFcmPositionTransferParams) -> Result<Box<PredictionsCreateFCMPositionTransferResponse>> {
        serde_json::from_value(self.0.create_fcm_position_transfer(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn create_fcm_subtrader(&self, request: &CreateFcmSubtraderParams) -> Result<Box<PredictionsCreateFCMSubtraderResponse>> {
        serde_json::from_value(self.0.create_fcm_subtrader(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn create_market_in_multivariate_event_collection(&self, request: &CreateMarketInMultivariateEventCollectionParams) -> Result<Box<PredictionsCreateMarketInMultivariateEventCollectionResponse>> {
        serde_json::from_value(self.0.create_market_in_multivariate_event_collection(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn create_order_group(&self, request: &CreateOrderGroupParams) -> Result<Box<PredictionsCreateOrderGroupResponse>> {
        serde_json::from_value(self.0.create_order_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn create_order_v2(&self, request: &CreateOrderV2Params) -> Result<Box<PredictionsCreateOrderV2Response>> {
        serde_json::from_value(self.0.create_order_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn create_order_with_exit_trigger(&self, request: &CreateOrderWithExitTriggerParams) -> Result<Box<PredictionsCreateOrderV2Response>> {
        serde_json::from_value(self.0.create_order_with_exit_trigger(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn create_quote(&self, request: &CreateQuoteParams) -> Result<Box<PredictionsCreateQuoteResponse>> {
        serde_json::from_value(self.0.create_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn create_rfq(&self, request: &CreateRfqParams) -> Result<Box<PredictionsCreateRFQResponse>> {
        serde_json::from_value(self.0.create_rfq(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn create_rfq_batch(&self, request: &CreateRfqBatchParams) -> Result<Box<PredictionsCreateRFQBatchResponse>> {
        serde_json::from_value(self.0.create_rfq_batch(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn create_rfq_batch_quote(&self, request: &CreateRfqBatchQuoteParams) -> Result<Box<PredictionsCreateRFQBatchQuoteResponse>> {
        serde_json::from_value(self.0.create_rfq_batch_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn create_subaccount(&self, request: &CreateSubaccountParams) -> Result<Box<PredictionsCreateSubaccountResponse>> {
        serde_json::from_value(self.0.create_subaccount(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn decrease_order_v2(&self, request: &DecreaseOrderV2Params) -> Result<Box<PredictionsDecreaseOrderV2Response>> {
        serde_json::from_value(self.0.decrease_order_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn delete_api_key(&self, request: &DeleteApiKeyParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn delete_event_contract_exit_trigger(&self, request: &DeleteEventContractExitTriggerParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_event_contract_exit_trigger(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn delete_fcm_event_contract_daily_cap(&self, request: &DeleteFcmEventContractDailyCapParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.delete_fcm_event_contract_daily_cap(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn delete_order_group(&self, request: &DeleteOrderGroupParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.delete_order_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn delete_quote(&self, request: &DeleteQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn delete_rfq(&self, request: &DeleteRfqParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_rfq(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn delete_rfq_batch(&self, request: &DeleteRfqBatchParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_rfq_batch(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn delete_rfq_batch_quote(&self, request: &DeleteRfqBatchQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_rfq_batch_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn delete_rfq_quote(&self, request: &DeleteRfqQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_rfq_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn generate_api_key(&self, request: &GenerateApiKeyParams) -> Result<Box<PredictionsGenerateApiKeyResponse>> {
        serde_json::from_value(self.0.generate_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_account_api_limits(&self, request: &GetAccountApiLimitsParams) -> Result<Box<PredictionsGetAccountApiLimitsResponse>> {
        serde_json::from_value(self.0.get_account_api_limits(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_account_api_usage_level_volume_progress(&self, request: &GetAccountApiUsageLevelVolumeProgressParams) -> Result<Box<PredictionsGetAccountApiUsageLevelVolumeProgressResponse>> {
        serde_json::from_value(self.0.get_account_api_usage_level_volume_progress(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_account_endpoint_costs(&self, request: &GetAccountEndpointCostsParams) -> Result<Box<PredictionsGetAccountEndpointCostsResponse>> {
        serde_json::from_value(self.0.get_account_endpoint_costs(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_api_keys(&self, request: &GetApiKeysParams) -> Result<Box<PredictionsGetApiKeysResponse>> {
        serde_json::from_value(self.0.get_api_keys(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_balance(&self, request: &GetBalanceParams) -> Result<Box<PredictionsGetBalanceResponse>> {
        serde_json::from_value(self.0.get_balance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_block_trade_proposals(&self, request: &GetBlockTradeProposalsParams) -> Result<Box<PredictionsGetBlockTradeProposalsResponse>> {
        serde_json::from_value(self.0.get_block_trade_proposals(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_communications_id(&self, request: &GetCommunicationsIdParams) -> Result<Box<PredictionsGetCommunicationsIDResponse>> {
        serde_json::from_value(self.0.get_communications_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_credit_history(&self, request: &GetCreditHistoryParams) -> Result<Box<PredictionsGetCreditHistoryResponse>> {
        serde_json::from_value(self.0.get_credit_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_deposits(&self, request: &GetDepositsParams) -> Result<Box<PredictionsGetDepositsResponse>> {
        serde_json::from_value(self.0.get_deposits(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_event(&self, request: &GetEventParams) -> Result<Box<PredictionsGetEventResponse>> {
        serde_json::from_value(self.0.get_event(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_event_contract_exit_triggers(&self, request: &GetEventContractExitTriggersParams) -> Result<GetEventContractExitTriggersResponse> {
        serde_json::from_value(self.0.get_event_contract_exit_triggers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_event_fee_changes(&self, request: &GetEventFeeChangesParams) -> Result<Box<PredictionsGetEventFeeChangesResponse>> {
        serde_json::from_value(self.0.get_event_fee_changes(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_event_forecast_percentiles_history(&self, request: &GetEventForecastPercentilesHistoryParams) -> Result<Box<PredictionsGetEventForecastPercentilesHistoryResponse>> {
        serde_json::from_value(self.0.get_event_forecast_percentiles_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_event_live_data(&self, request: &GetEventLiveDataParams) -> Result<Box<PredictionsGetEventLiveDataResponse>> {
        serde_json::from_value(self.0.get_event_live_data(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_event_metadata(&self, request: &GetEventMetadataParams) -> Result<Box<PredictionsGetEventMetadataResponse>> {
        serde_json::from_value(self.0.get_event_metadata(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_events(&self, request: &GetEventsParams) -> Result<Box<PredictionsGetEventsResponse>> {
        serde_json::from_value(self.0.get_events(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_exchange_schedule(&self, request: &GetExchangeScheduleParams) -> Result<Box<PredictionsGetExchangeScheduleResponse>> {
        serde_json::from_value(self.0.get_exchange_schedule(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_exchange_status(&self, request: &GetExchangeStatusParams) -> Result<Box<PredictionsExchangeStatus>> {
        serde_json::from_value(self.0.get_exchange_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_fcm_event_contract_daily_cap(&self, request: &GetFcmEventContractDailyCapParams) -> Result<Box<PredictionsGetFCMEventContractDailyCapResponse>> {
        serde_json::from_value(self.0.get_fcm_event_contract_daily_cap(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_fcm_fills(&self, request: &GetFcmFillsParams) -> Result<Box<PredictionsGetFcmFillsResponse>> {
        serde_json::from_value(self.0.get_fcm_fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_fcm_orders(&self, request: &GetFcmOrdersParams) -> Result<Box<PredictionsGetOrdersResponse>> {
        serde_json::from_value(self.0.get_fcm_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_fcm_positions(&self, request: &GetFcmPositionsParams) -> Result<Box<PredictionsGetPositionsResponse>> {
        serde_json::from_value(self.0.get_fcm_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_fcm_subtrader_blocked_categories(&self, request: &GetFcmSubtraderBlockedCategoriesParams) -> Result<Box<PredictionsGetFCMSubtraderBlockedCategoriesResponse>> {
        serde_json::from_value(self.0.get_fcm_subtrader_blocked_categories(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_fills(&self, request: &GetFillsParams) -> Result<Box<PredictionsGetFillsResponse>> {
        serde_json::from_value(self.0.get_fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_fills_historical(&self, request: &GetFillsHistoricalParams) -> Result<Box<PredictionsGetFillsResponse>> {
        serde_json::from_value(self.0.get_fills_historical(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_filters_for_sports(&self, request: &GetFiltersForSportsParams) -> Result<Box<PredictionsGetFiltersBySportsResponse>> {
        serde_json::from_value(self.0.get_filters_for_sports(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_game_stats(&self, request: &GetGameStatsParams) -> Result<Box<PredictionsGetGameStatsResponse>> {
        serde_json::from_value(self.0.get_game_stats(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_historical_cutoff(&self, request: &GetHistoricalCutoffParams) -> Result<Box<PredictionsGetHistoricalCutoffResponse>> {
        serde_json::from_value(self.0.get_historical_cutoff(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_historical_market(&self, request: &GetHistoricalMarketParams) -> Result<Box<PredictionsGetMarketResponse>> {
        serde_json::from_value(self.0.get_historical_market(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_historical_markets(&self, request: &GetHistoricalMarketsParams) -> Result<Box<PredictionsGetMarketsResponse>> {
        serde_json::from_value(self.0.get_historical_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_historical_orders(&self, request: &GetHistoricalOrdersParams) -> Result<Box<PredictionsGetOrdersResponse>> {
        serde_json::from_value(self.0.get_historical_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_historical_positions(&self, request: &GetHistoricalPositionsParams) -> Result<Box<PredictionsGetPositionsResponse>> {
        serde_json::from_value(self.0.get_historical_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_incentive_programs(&self, request: &GetIncentiveProgramsParams) -> Result<Box<PredictionsGetIncentiveProgramsResponse>> {
        serde_json::from_value(self.0.get_incentive_programs(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_intra_exchange_instance_transfer(&self, request: &GetIntraExchangeInstanceTransferParams) -> Result<Box<PredictionsGetIntraExchangeInstanceTransferResponse>> {
        serde_json::from_value(self.0.get_intra_exchange_instance_transfer(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_intra_exchange_instance_transfers(&self, request: &GetIntraExchangeInstanceTransfersParams) -> Result<Box<PredictionsGetIntraExchangeInstanceTransfersResponse>> {
        serde_json::from_value(self.0.get_intra_exchange_instance_transfers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_live_data(&self, request: &GetLiveDataParams) -> Result<Box<PredictionsGetLiveDataResponse>> {
        serde_json::from_value(self.0.get_live_data(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_live_data_by_milestone(&self, request: &GetLiveDataByMilestoneParams) -> Result<Box<PredictionsGetLiveDataResponse>> {
        serde_json::from_value(self.0.get_live_data_by_milestone(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_live_datas(&self, request: &GetLiveDatasParams) -> Result<Box<PredictionsGetLiveDatasResponse>> {
        serde_json::from_value(self.0.get_live_datas(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_market(&self, request: &GetMarketParams) -> Result<Box<PredictionsGetMarketResponse>> {
        serde_json::from_value(self.0.get_market(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_market_candlesticks(&self, request: &GetMarketCandlesticksParams) -> Result<Box<PredictionsGetMarketCandlesticksResponse>> {
        serde_json::from_value(self.0.get_market_candlesticks(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_market_candlesticks_by_event(&self, request: &GetMarketCandlesticksByEventParams) -> Result<Box<PredictionsGetEventCandlesticksResponse>> {
        serde_json::from_value(self.0.get_market_candlesticks_by_event(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_market_candlesticks_historical(&self, request: &GetMarketCandlesticksHistoricalParams) -> Result<Box<PredictionsGetMarketCandlesticksHistoricalResponse>> {
        serde_json::from_value(self.0.get_market_candlesticks_historical(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_market_orderbook(&self, request: &GetMarketOrderbookParams) -> Result<Box<PredictionsGetMarketOrderbookResponse>> {
        serde_json::from_value(self.0.get_market_orderbook(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_market_orderbooks(&self, request: &GetMarketOrderbooksParams) -> Result<Box<PredictionsGetMarketOrderbooksResponse>> {
        serde_json::from_value(self.0.get_market_orderbooks(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_markets(&self, request: &GetMarketsParams) -> Result<Box<PredictionsGetMarketsResponse>> {
        serde_json::from_value(self.0.get_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_milestone(&self, request: &GetMilestoneParams) -> Result<Box<PredictionsGetMilestoneResponse>> {
        serde_json::from_value(self.0.get_milestone(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_milestones(&self, request: &GetMilestonesParams) -> Result<Box<PredictionsGetMilestonesResponse>> {
        serde_json::from_value(self.0.get_milestones(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_multivariate_event_collection(&self, request: &GetMultivariateEventCollectionParams) -> Result<Box<PredictionsGetMultivariateEventCollectionResponse>> {
        serde_json::from_value(self.0.get_multivariate_event_collection(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_multivariate_event_collections(&self, request: &GetMultivariateEventCollectionsParams) -> Result<Box<PredictionsGetMultivariateEventCollectionsResponse>> {
        serde_json::from_value(self.0.get_multivariate_event_collections(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_multivariate_events(&self, request: &GetMultivariateEventsParams) -> Result<Box<PredictionsGetMultivariateEventsResponse>> {
        serde_json::from_value(self.0.get_multivariate_events(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_order(&self, request: &GetOrderParams) -> Result<Box<PredictionsGetOrderResponse>> {
        serde_json::from_value(self.0.get_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_order_group(&self, request: &GetOrderGroupParams) -> Result<Box<PredictionsGetOrderGroupResponse>> {
        serde_json::from_value(self.0.get_order_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_order_groups(&self, request: &GetOrderGroupsParams) -> Result<Box<PredictionsGetOrderGroupsResponse>> {
        serde_json::from_value(self.0.get_order_groups(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_order_queue_position(&self, request: &GetOrderQueuePositionParams) -> Result<Box<PredictionsGetOrderQueuePositionResponse>> {
        serde_json::from_value(self.0.get_order_queue_position(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_order_queue_positions(&self, request: &GetOrderQueuePositionsParams) -> Result<Box<PredictionsGetOrderQueuePositionsResponse>> {
        serde_json::from_value(self.0.get_order_queue_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_orders(&self, request: &GetOrdersParams) -> Result<Box<PredictionsGetOrdersResponse>> {
        serde_json::from_value(self.0.get_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_portfolio_resting_order_total_value(&self, request: &GetPortfolioRestingOrderTotalValueParams) -> Result<Box<PredictionsGetPortfolioRestingOrderTotalValueResponse>> {
        serde_json::from_value(self.0.get_portfolio_resting_order_total_value(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_positions(&self, request: &GetPositionsParams) -> Result<Box<PredictionsGetPositionsResponse>> {
        serde_json::from_value(self.0.get_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_quote(&self, request: &GetQuoteParams) -> Result<Box<PredictionsGetQuoteResponse>> {
        serde_json::from_value(self.0.get_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_quotes(&self, request: &GetQuotesParams) -> Result<Box<PredictionsGetQuotesResponse>> {
        serde_json::from_value(self.0.get_quotes(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_rf_qs(&self, request: &GetRfQsParams) -> Result<Box<PredictionsGetRFQsResponse>> {
        serde_json::from_value(self.0.get_rf_qs(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_rfq(&self, request: &GetRfqParams) -> Result<Box<PredictionsGetRFQResponse>> {
        serde_json::from_value(self.0.get_rfq(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_rfq_batch(&self, request: &GetRfqBatchParams) -> Result<Box<PredictionsGetRFQBatchResponse>> {
        serde_json::from_value(self.0.get_rfq_batch(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_rfq_batch_best_quotes(&self, request: &GetRfqBatchBestQuotesParams) -> Result<Box<PredictionsGetRFQBatchBestQuotesResponse>> {
        serde_json::from_value(self.0.get_rfq_batch_best_quotes(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_rfq_batch_quotes(&self, request: &GetRfqBatchQuotesParams) -> Result<Box<PredictionsGetRFQBatchQuotesResponse>> {
        serde_json::from_value(self.0.get_rfq_batch_quotes(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_rfq_batches(&self, request: &GetRfqBatchesParams) -> Result<Box<PredictionsGetRFQBatchesResponse>> {
        serde_json::from_value(self.0.get_rfq_batches(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_rfq_quote(&self, request: &GetRfqQuoteParams) -> Result<Box<PredictionsGetQuoteResponse>> {
        serde_json::from_value(self.0.get_rfq_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_series(&self, request: &GetSeriesParams) -> Result<Box<PredictionsGetSeriesResponse>> {
        serde_json::from_value(self.0.get_series(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_series_fee_changes(&self, request: &GetSeriesFeeChangesParams) -> Result<Box<PredictionsGetSeriesFeeChangesResponse>> {
        serde_json::from_value(self.0.get_series_fee_changes(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_series_list(&self, request: &GetSeriesListParams) -> Result<Box<PredictionsGetSeriesListResponse>> {
        serde_json::from_value(self.0.get_series_list(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_settlements(&self, request: &GetSettlementsParams) -> Result<Box<PredictionsGetSettlementsResponse>> {
        serde_json::from_value(self.0.get_settlements(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_structured_target(&self, request: &GetStructuredTargetParams) -> Result<Box<PredictionsGetStructuredTargetResponse>> {
        serde_json::from_value(self.0.get_structured_target(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_structured_targets(&self, request: &GetStructuredTargetsParams) -> Result<Box<PredictionsGetStructuredTargetsResponse>> {
        serde_json::from_value(self.0.get_structured_targets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_subaccount_balances(&self, request: &GetSubaccountBalancesParams) -> Result<Box<PredictionsGetSubaccountBalancesResponse>> {
        serde_json::from_value(self.0.get_subaccount_balances(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_subaccount_netting(&self, request: &GetSubaccountNettingParams) -> Result<Box<PredictionsGetSubaccountNettingResponse>> {
        serde_json::from_value(self.0.get_subaccount_netting(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_subaccount_transfers(&self, request: &GetSubaccountTransfersParams) -> Result<Box<PredictionsGetSubaccountTransfersResponse>> {
        serde_json::from_value(self.0.get_subaccount_transfers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_tags_for_series_categories(&self, request: &GetTagsForSeriesCategoriesParams) -> Result<Box<PredictionsGetTagsForSeriesCategoriesResponse>> {
        serde_json::from_value(self.0.get_tags_for_series_categories(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_target_balance_allocation(&self, request: &GetTargetBalanceAllocationParams) -> Result<Box<PredictionsGetTargetBalanceAllocationResponse>> {
        serde_json::from_value(self.0.get_target_balance_allocation(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_trades(&self, request: &GetTradesParams) -> Result<Box<PredictionsGetTradesResponse>> {
        serde_json::from_value(self.0.get_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_trades_historical(&self, request: &GetTradesHistoricalParams) -> Result<Box<PredictionsGetTradesResponse>> {
        serde_json::from_value(self.0.get_trades_historical(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_user_data_timestamp(&self, request: &GetUserDataTimestampParams) -> Result<Box<PredictionsGetUserDataTimestampResponse>> {
        serde_json::from_value(self.0.get_user_data_timestamp(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_weather_index(&self, request: &GetWeatherIndexParams) -> Result<Box<PredictionsGetWeatherIndexResponse>> {
        serde_json::from_value(self.0.get_weather_index(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_weather_index_calibrations(&self, request: &GetWeatherIndexCalibrationsParams) -> Result<Box<PredictionsGetWeatherIndexCalibrationsResponse>> {
        serde_json::from_value(self.0.get_weather_index_calibrations(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn get_withdrawals(&self, request: &GetWithdrawalsParams) -> Result<Box<PredictionsGetWithdrawalsResponse>> {
        serde_json::from_value(self.0.get_withdrawals(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn intra_exchange_instance_transfer(&self, request: &IntraExchangeInstanceTransferParams) -> Result<Box<PredictionsIntraExchangeInstanceTransferResponse>> {
        serde_json::from_value(self.0.intra_exchange_instance_transfer(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn list_fcm_subtraders(&self, request: &ListFcmSubtradersParams) -> Result<Box<PredictionsListFCMSubtradersResponse>> {
        serde_json::from_value(self.0.list_fcm_subtraders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn propose_block_trade(&self, request: &ProposeBlockTradeParams) -> Result<Box<PredictionsProposeBlockTradeResponse>> {
        serde_json::from_value(self.0.propose_block_trade(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn reset_order_group(&self, request: &ResetOrderGroupParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.reset_order_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn set_event_contract_exit_trigger(&self, request: &SetEventContractExitTriggerParams) -> Result<Box<PredictionsEventContractExitTrigger>> {
        serde_json::from_value(self.0.set_event_contract_exit_trigger(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn set_target_balance_allocation(&self, request: &SetTargetBalanceAllocationParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.set_target_balance_allocation(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn trigger_order_group(&self, request: &TriggerOrderGroupParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.trigger_order_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn update_fcm_event_contract_daily_cap(&self, request: &UpdateFcmEventContractDailyCapParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.update_fcm_event_contract_daily_cap(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn update_fcm_subtrader_blocked_categories(&self, request: &UpdateFcmSubtraderBlockedCategoriesParams) -> Result<Box<PredictionsUpdateFCMSubtraderBlockedCategoriesResponse>> {
        serde_json::from_value(self.0.update_fcm_subtrader_blocked_categories(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn update_order_group_limit(&self, request: &UpdateOrderGroupLimitParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.update_order_group_limit(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn update_subaccount_netting(&self, request: &UpdateSubaccountNettingParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.update_subaccount_netting(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub fn upgrade_account_api_usage_level(&self, request: &UpgradeAccountApiUsageLevelParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.upgrade_account_api_usage_level(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedKalshiClient(crate::AsyncClient<KalshiClient>);
#[cfg(feature = "async")]
impl TypedKalshiClient {
    pub fn into_async(self) -> AsyncTypedKalshiClient { AsyncTypedKalshiClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedKalshiClient {
    pub async fn accept_block_trade_proposal(&self, request: &AcceptBlockTradeProposalParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.accept_block_trade_proposal(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn accept_quote(&self, request: &AcceptQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.accept_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn accept_rfq_batch(&self, request: &AcceptRfqBatchParams) -> Result<Box<PredictionsAcceptRFQBatchResponse>> {
        serde_json::from_value(self.0.accept_rfq_batch(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn accept_rfq_quote(&self, request: &AcceptRfqQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.accept_rfq_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn amend_order_v2(&self, request: &AmendOrderV2Params) -> Result<Box<PredictionsAmendOrderV2Response>> {
        serde_json::from_value(self.0.amend_order_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn apply_subaccount_transfer(&self, request: &ApplySubaccountTransferParams) -> Result<Box<PredictionsApplySubaccountTransferResponse>> {
        serde_json::from_value(self.0.apply_subaccount_transfer(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn batch_cancel_orders_v2(&self, request: &BatchCancelOrdersV2Params) -> Result<Box<PredictionsBatchCancelOrdersV2Response>> {
        serde_json::from_value(self.0.batch_cancel_orders_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn batch_create_orders_v2(&self, request: &BatchCreateOrdersV2Params) -> Result<Box<PredictionsBatchCreateOrdersV2Response>> {
        serde_json::from_value(self.0.batch_create_orders_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn batch_get_market_candlesticks(&self, request: &BatchGetMarketCandlesticksParams) -> Result<Box<PredictionsBatchGetMarketCandlesticksResponse>> {
        serde_json::from_value(self.0.batch_get_market_candlesticks(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn cancel_all_orders(&self, request: &CancelAllOrdersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_all_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn cancel_order_v2(&self, request: &CancelOrderV2Params) -> Result<Box<PredictionsCancelOrderV2Response>> {
        serde_json::from_value(self.0.cancel_order_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn confirm_quote(&self, request: &ConfirmQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.confirm_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn confirm_rfq_batch_quote(&self, request: &ConfirmRfqBatchQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.confirm_rfq_batch_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn confirm_rfq_quote(&self, request: &ConfirmRfqQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.confirm_rfq_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn create_api_key(&self, request: &CreateApiKeyParams) -> Result<Box<PredictionsCreateApiKeyResponse>> {
        serde_json::from_value(self.0.create_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn create_fcm_position_transfer(&self, request: &CreateFcmPositionTransferParams) -> Result<Box<PredictionsCreateFCMPositionTransferResponse>> {
        serde_json::from_value(self.0.create_fcm_position_transfer(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn create_fcm_subtrader(&self, request: &CreateFcmSubtraderParams) -> Result<Box<PredictionsCreateFCMSubtraderResponse>> {
        serde_json::from_value(self.0.create_fcm_subtrader(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn create_market_in_multivariate_event_collection(&self, request: &CreateMarketInMultivariateEventCollectionParams) -> Result<Box<PredictionsCreateMarketInMultivariateEventCollectionResponse>> {
        serde_json::from_value(self.0.create_market_in_multivariate_event_collection(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn create_order_group(&self, request: &CreateOrderGroupParams) -> Result<Box<PredictionsCreateOrderGroupResponse>> {
        serde_json::from_value(self.0.create_order_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn create_order_v2(&self, request: &CreateOrderV2Params) -> Result<Box<PredictionsCreateOrderV2Response>> {
        serde_json::from_value(self.0.create_order_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn create_order_with_exit_trigger(&self, request: &CreateOrderWithExitTriggerParams) -> Result<Box<PredictionsCreateOrderV2Response>> {
        serde_json::from_value(self.0.create_order_with_exit_trigger(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn create_quote(&self, request: &CreateQuoteParams) -> Result<Box<PredictionsCreateQuoteResponse>> {
        serde_json::from_value(self.0.create_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn create_rfq(&self, request: &CreateRfqParams) -> Result<Box<PredictionsCreateRFQResponse>> {
        serde_json::from_value(self.0.create_rfq(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn create_rfq_batch(&self, request: &CreateRfqBatchParams) -> Result<Box<PredictionsCreateRFQBatchResponse>> {
        serde_json::from_value(self.0.create_rfq_batch(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn create_rfq_batch_quote(&self, request: &CreateRfqBatchQuoteParams) -> Result<Box<PredictionsCreateRFQBatchQuoteResponse>> {
        serde_json::from_value(self.0.create_rfq_batch_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn create_subaccount(&self, request: &CreateSubaccountParams) -> Result<Box<PredictionsCreateSubaccountResponse>> {
        serde_json::from_value(self.0.create_subaccount(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn decrease_order_v2(&self, request: &DecreaseOrderV2Params) -> Result<Box<PredictionsDecreaseOrderV2Response>> {
        serde_json::from_value(self.0.decrease_order_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn delete_api_key(&self, request: &DeleteApiKeyParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn delete_event_contract_exit_trigger(&self, request: &DeleteEventContractExitTriggerParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_event_contract_exit_trigger(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn delete_fcm_event_contract_daily_cap(&self, request: &DeleteFcmEventContractDailyCapParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.delete_fcm_event_contract_daily_cap(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn delete_order_group(&self, request: &DeleteOrderGroupParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.delete_order_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn delete_quote(&self, request: &DeleteQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn delete_rfq(&self, request: &DeleteRfqParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_rfq(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn delete_rfq_batch(&self, request: &DeleteRfqBatchParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_rfq_batch(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn delete_rfq_batch_quote(&self, request: &DeleteRfqBatchQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_rfq_batch_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn delete_rfq_quote(&self, request: &DeleteRfqQuoteParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_rfq_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn generate_api_key(&self, request: &GenerateApiKeyParams) -> Result<Box<PredictionsGenerateApiKeyResponse>> {
        serde_json::from_value(self.0.generate_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_account_api_limits(&self, request: &GetAccountApiLimitsParams) -> Result<Box<PredictionsGetAccountApiLimitsResponse>> {
        serde_json::from_value(self.0.get_account_api_limits(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_account_api_usage_level_volume_progress(&self, request: &GetAccountApiUsageLevelVolumeProgressParams) -> Result<Box<PredictionsGetAccountApiUsageLevelVolumeProgressResponse>> {
        serde_json::from_value(self.0.get_account_api_usage_level_volume_progress(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_account_endpoint_costs(&self, request: &GetAccountEndpointCostsParams) -> Result<Box<PredictionsGetAccountEndpointCostsResponse>> {
        serde_json::from_value(self.0.get_account_endpoint_costs(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_api_keys(&self, request: &GetApiKeysParams) -> Result<Box<PredictionsGetApiKeysResponse>> {
        serde_json::from_value(self.0.get_api_keys(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_balance(&self, request: &GetBalanceParams) -> Result<Box<PredictionsGetBalanceResponse>> {
        serde_json::from_value(self.0.get_balance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_block_trade_proposals(&self, request: &GetBlockTradeProposalsParams) -> Result<Box<PredictionsGetBlockTradeProposalsResponse>> {
        serde_json::from_value(self.0.get_block_trade_proposals(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_communications_id(&self, request: &GetCommunicationsIdParams) -> Result<Box<PredictionsGetCommunicationsIDResponse>> {
        serde_json::from_value(self.0.get_communications_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_credit_history(&self, request: &GetCreditHistoryParams) -> Result<Box<PredictionsGetCreditHistoryResponse>> {
        serde_json::from_value(self.0.get_credit_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_deposits(&self, request: &GetDepositsParams) -> Result<Box<PredictionsGetDepositsResponse>> {
        serde_json::from_value(self.0.get_deposits(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_event(&self, request: &GetEventParams) -> Result<Box<PredictionsGetEventResponse>> {
        serde_json::from_value(self.0.get_event(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_event_contract_exit_triggers(&self, request: &GetEventContractExitTriggersParams) -> Result<GetEventContractExitTriggersResponse> {
        serde_json::from_value(self.0.get_event_contract_exit_triggers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_event_fee_changes(&self, request: &GetEventFeeChangesParams) -> Result<Box<PredictionsGetEventFeeChangesResponse>> {
        serde_json::from_value(self.0.get_event_fee_changes(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_event_forecast_percentiles_history(&self, request: &GetEventForecastPercentilesHistoryParams) -> Result<Box<PredictionsGetEventForecastPercentilesHistoryResponse>> {
        serde_json::from_value(self.0.get_event_forecast_percentiles_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_event_live_data(&self, request: &GetEventLiveDataParams) -> Result<Box<PredictionsGetEventLiveDataResponse>> {
        serde_json::from_value(self.0.get_event_live_data(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_event_metadata(&self, request: &GetEventMetadataParams) -> Result<Box<PredictionsGetEventMetadataResponse>> {
        serde_json::from_value(self.0.get_event_metadata(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_events(&self, request: &GetEventsParams) -> Result<Box<PredictionsGetEventsResponse>> {
        serde_json::from_value(self.0.get_events(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_exchange_schedule(&self, request: &GetExchangeScheduleParams) -> Result<Box<PredictionsGetExchangeScheduleResponse>> {
        serde_json::from_value(self.0.get_exchange_schedule(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_exchange_status(&self, request: &GetExchangeStatusParams) -> Result<Box<PredictionsExchangeStatus>> {
        serde_json::from_value(self.0.get_exchange_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_fcm_event_contract_daily_cap(&self, request: &GetFcmEventContractDailyCapParams) -> Result<Box<PredictionsGetFCMEventContractDailyCapResponse>> {
        serde_json::from_value(self.0.get_fcm_event_contract_daily_cap(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_fcm_fills(&self, request: &GetFcmFillsParams) -> Result<Box<PredictionsGetFcmFillsResponse>> {
        serde_json::from_value(self.0.get_fcm_fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_fcm_orders(&self, request: &GetFcmOrdersParams) -> Result<Box<PredictionsGetOrdersResponse>> {
        serde_json::from_value(self.0.get_fcm_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_fcm_positions(&self, request: &GetFcmPositionsParams) -> Result<Box<PredictionsGetPositionsResponse>> {
        serde_json::from_value(self.0.get_fcm_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_fcm_subtrader_blocked_categories(&self, request: &GetFcmSubtraderBlockedCategoriesParams) -> Result<Box<PredictionsGetFCMSubtraderBlockedCategoriesResponse>> {
        serde_json::from_value(self.0.get_fcm_subtrader_blocked_categories(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_fills(&self, request: &GetFillsParams) -> Result<Box<PredictionsGetFillsResponse>> {
        serde_json::from_value(self.0.get_fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_fills_historical(&self, request: &GetFillsHistoricalParams) -> Result<Box<PredictionsGetFillsResponse>> {
        serde_json::from_value(self.0.get_fills_historical(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_filters_for_sports(&self, request: &GetFiltersForSportsParams) -> Result<Box<PredictionsGetFiltersBySportsResponse>> {
        serde_json::from_value(self.0.get_filters_for_sports(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_game_stats(&self, request: &GetGameStatsParams) -> Result<Box<PredictionsGetGameStatsResponse>> {
        serde_json::from_value(self.0.get_game_stats(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_historical_cutoff(&self, request: &GetHistoricalCutoffParams) -> Result<Box<PredictionsGetHistoricalCutoffResponse>> {
        serde_json::from_value(self.0.get_historical_cutoff(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_historical_market(&self, request: &GetHistoricalMarketParams) -> Result<Box<PredictionsGetMarketResponse>> {
        serde_json::from_value(self.0.get_historical_market(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_historical_markets(&self, request: &GetHistoricalMarketsParams) -> Result<Box<PredictionsGetMarketsResponse>> {
        serde_json::from_value(self.0.get_historical_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_historical_orders(&self, request: &GetHistoricalOrdersParams) -> Result<Box<PredictionsGetOrdersResponse>> {
        serde_json::from_value(self.0.get_historical_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_historical_positions(&self, request: &GetHistoricalPositionsParams) -> Result<Box<PredictionsGetPositionsResponse>> {
        serde_json::from_value(self.0.get_historical_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_incentive_programs(&self, request: &GetIncentiveProgramsParams) -> Result<Box<PredictionsGetIncentiveProgramsResponse>> {
        serde_json::from_value(self.0.get_incentive_programs(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_intra_exchange_instance_transfer(&self, request: &GetIntraExchangeInstanceTransferParams) -> Result<Box<PredictionsGetIntraExchangeInstanceTransferResponse>> {
        serde_json::from_value(self.0.get_intra_exchange_instance_transfer(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_intra_exchange_instance_transfers(&self, request: &GetIntraExchangeInstanceTransfersParams) -> Result<Box<PredictionsGetIntraExchangeInstanceTransfersResponse>> {
        serde_json::from_value(self.0.get_intra_exchange_instance_transfers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_live_data(&self, request: &GetLiveDataParams) -> Result<Box<PredictionsGetLiveDataResponse>> {
        serde_json::from_value(self.0.get_live_data(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_live_data_by_milestone(&self, request: &GetLiveDataByMilestoneParams) -> Result<Box<PredictionsGetLiveDataResponse>> {
        serde_json::from_value(self.0.get_live_data_by_milestone(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_live_datas(&self, request: &GetLiveDatasParams) -> Result<Box<PredictionsGetLiveDatasResponse>> {
        serde_json::from_value(self.0.get_live_datas(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_market(&self, request: &GetMarketParams) -> Result<Box<PredictionsGetMarketResponse>> {
        serde_json::from_value(self.0.get_market(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_market_candlesticks(&self, request: &GetMarketCandlesticksParams) -> Result<Box<PredictionsGetMarketCandlesticksResponse>> {
        serde_json::from_value(self.0.get_market_candlesticks(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_market_candlesticks_by_event(&self, request: &GetMarketCandlesticksByEventParams) -> Result<Box<PredictionsGetEventCandlesticksResponse>> {
        serde_json::from_value(self.0.get_market_candlesticks_by_event(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_market_candlesticks_historical(&self, request: &GetMarketCandlesticksHistoricalParams) -> Result<Box<PredictionsGetMarketCandlesticksHistoricalResponse>> {
        serde_json::from_value(self.0.get_market_candlesticks_historical(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_market_orderbook(&self, request: &GetMarketOrderbookParams) -> Result<Box<PredictionsGetMarketOrderbookResponse>> {
        serde_json::from_value(self.0.get_market_orderbook(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_market_orderbooks(&self, request: &GetMarketOrderbooksParams) -> Result<Box<PredictionsGetMarketOrderbooksResponse>> {
        serde_json::from_value(self.0.get_market_orderbooks(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_markets(&self, request: &GetMarketsParams) -> Result<Box<PredictionsGetMarketsResponse>> {
        serde_json::from_value(self.0.get_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_milestone(&self, request: &GetMilestoneParams) -> Result<Box<PredictionsGetMilestoneResponse>> {
        serde_json::from_value(self.0.get_milestone(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_milestones(&self, request: &GetMilestonesParams) -> Result<Box<PredictionsGetMilestonesResponse>> {
        serde_json::from_value(self.0.get_milestones(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_multivariate_event_collection(&self, request: &GetMultivariateEventCollectionParams) -> Result<Box<PredictionsGetMultivariateEventCollectionResponse>> {
        serde_json::from_value(self.0.get_multivariate_event_collection(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_multivariate_event_collections(&self, request: &GetMultivariateEventCollectionsParams) -> Result<Box<PredictionsGetMultivariateEventCollectionsResponse>> {
        serde_json::from_value(self.0.get_multivariate_event_collections(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_multivariate_events(&self, request: &GetMultivariateEventsParams) -> Result<Box<PredictionsGetMultivariateEventsResponse>> {
        serde_json::from_value(self.0.get_multivariate_events(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_order(&self, request: &GetOrderParams) -> Result<Box<PredictionsGetOrderResponse>> {
        serde_json::from_value(self.0.get_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_order_group(&self, request: &GetOrderGroupParams) -> Result<Box<PredictionsGetOrderGroupResponse>> {
        serde_json::from_value(self.0.get_order_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_order_groups(&self, request: &GetOrderGroupsParams) -> Result<Box<PredictionsGetOrderGroupsResponse>> {
        serde_json::from_value(self.0.get_order_groups(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_order_queue_position(&self, request: &GetOrderQueuePositionParams) -> Result<Box<PredictionsGetOrderQueuePositionResponse>> {
        serde_json::from_value(self.0.get_order_queue_position(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_order_queue_positions(&self, request: &GetOrderQueuePositionsParams) -> Result<Box<PredictionsGetOrderQueuePositionsResponse>> {
        serde_json::from_value(self.0.get_order_queue_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_orders(&self, request: &GetOrdersParams) -> Result<Box<PredictionsGetOrdersResponse>> {
        serde_json::from_value(self.0.get_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_portfolio_resting_order_total_value(&self, request: &GetPortfolioRestingOrderTotalValueParams) -> Result<Box<PredictionsGetPortfolioRestingOrderTotalValueResponse>> {
        serde_json::from_value(self.0.get_portfolio_resting_order_total_value(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_positions(&self, request: &GetPositionsParams) -> Result<Box<PredictionsGetPositionsResponse>> {
        serde_json::from_value(self.0.get_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_quote(&self, request: &GetQuoteParams) -> Result<Box<PredictionsGetQuoteResponse>> {
        serde_json::from_value(self.0.get_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_quotes(&self, request: &GetQuotesParams) -> Result<Box<PredictionsGetQuotesResponse>> {
        serde_json::from_value(self.0.get_quotes(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_rf_qs(&self, request: &GetRfQsParams) -> Result<Box<PredictionsGetRFQsResponse>> {
        serde_json::from_value(self.0.get_rf_qs(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_rfq(&self, request: &GetRfqParams) -> Result<Box<PredictionsGetRFQResponse>> {
        serde_json::from_value(self.0.get_rfq(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_rfq_batch(&self, request: &GetRfqBatchParams) -> Result<Box<PredictionsGetRFQBatchResponse>> {
        serde_json::from_value(self.0.get_rfq_batch(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_rfq_batch_best_quotes(&self, request: &GetRfqBatchBestQuotesParams) -> Result<Box<PredictionsGetRFQBatchBestQuotesResponse>> {
        serde_json::from_value(self.0.get_rfq_batch_best_quotes(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_rfq_batch_quotes(&self, request: &GetRfqBatchQuotesParams) -> Result<Box<PredictionsGetRFQBatchQuotesResponse>> {
        serde_json::from_value(self.0.get_rfq_batch_quotes(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_rfq_batches(&self, request: &GetRfqBatchesParams) -> Result<Box<PredictionsGetRFQBatchesResponse>> {
        serde_json::from_value(self.0.get_rfq_batches(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_rfq_quote(&self, request: &GetRfqQuoteParams) -> Result<Box<PredictionsGetQuoteResponse>> {
        serde_json::from_value(self.0.get_rfq_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_series(&self, request: &GetSeriesParams) -> Result<Box<PredictionsGetSeriesResponse>> {
        serde_json::from_value(self.0.get_series(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_series_fee_changes(&self, request: &GetSeriesFeeChangesParams) -> Result<Box<PredictionsGetSeriesFeeChangesResponse>> {
        serde_json::from_value(self.0.get_series_fee_changes(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_series_list(&self, request: &GetSeriesListParams) -> Result<Box<PredictionsGetSeriesListResponse>> {
        serde_json::from_value(self.0.get_series_list(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_settlements(&self, request: &GetSettlementsParams) -> Result<Box<PredictionsGetSettlementsResponse>> {
        serde_json::from_value(self.0.get_settlements(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_structured_target(&self, request: &GetStructuredTargetParams) -> Result<Box<PredictionsGetStructuredTargetResponse>> {
        serde_json::from_value(self.0.get_structured_target(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_structured_targets(&self, request: &GetStructuredTargetsParams) -> Result<Box<PredictionsGetStructuredTargetsResponse>> {
        serde_json::from_value(self.0.get_structured_targets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_subaccount_balances(&self, request: &GetSubaccountBalancesParams) -> Result<Box<PredictionsGetSubaccountBalancesResponse>> {
        serde_json::from_value(self.0.get_subaccount_balances(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_subaccount_netting(&self, request: &GetSubaccountNettingParams) -> Result<Box<PredictionsGetSubaccountNettingResponse>> {
        serde_json::from_value(self.0.get_subaccount_netting(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_subaccount_transfers(&self, request: &GetSubaccountTransfersParams) -> Result<Box<PredictionsGetSubaccountTransfersResponse>> {
        serde_json::from_value(self.0.get_subaccount_transfers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_tags_for_series_categories(&self, request: &GetTagsForSeriesCategoriesParams) -> Result<Box<PredictionsGetTagsForSeriesCategoriesResponse>> {
        serde_json::from_value(self.0.get_tags_for_series_categories(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_target_balance_allocation(&self, request: &GetTargetBalanceAllocationParams) -> Result<Box<PredictionsGetTargetBalanceAllocationResponse>> {
        serde_json::from_value(self.0.get_target_balance_allocation(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_trades(&self, request: &GetTradesParams) -> Result<Box<PredictionsGetTradesResponse>> {
        serde_json::from_value(self.0.get_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_trades_historical(&self, request: &GetTradesHistoricalParams) -> Result<Box<PredictionsGetTradesResponse>> {
        serde_json::from_value(self.0.get_trades_historical(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_user_data_timestamp(&self, request: &GetUserDataTimestampParams) -> Result<Box<PredictionsGetUserDataTimestampResponse>> {
        serde_json::from_value(self.0.get_user_data_timestamp(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_weather_index(&self, request: &GetWeatherIndexParams) -> Result<Box<PredictionsGetWeatherIndexResponse>> {
        serde_json::from_value(self.0.get_weather_index(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_weather_index_calibrations(&self, request: &GetWeatherIndexCalibrationsParams) -> Result<Box<PredictionsGetWeatherIndexCalibrationsResponse>> {
        serde_json::from_value(self.0.get_weather_index_calibrations(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn get_withdrawals(&self, request: &GetWithdrawalsParams) -> Result<Box<PredictionsGetWithdrawalsResponse>> {
        serde_json::from_value(self.0.get_withdrawals(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn intra_exchange_instance_transfer(&self, request: &IntraExchangeInstanceTransferParams) -> Result<Box<PredictionsIntraExchangeInstanceTransferResponse>> {
        serde_json::from_value(self.0.intra_exchange_instance_transfer(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn list_fcm_subtraders(&self, request: &ListFcmSubtradersParams) -> Result<Box<PredictionsListFCMSubtradersResponse>> {
        serde_json::from_value(self.0.list_fcm_subtraders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn propose_block_trade(&self, request: &ProposeBlockTradeParams) -> Result<Box<PredictionsProposeBlockTradeResponse>> {
        serde_json::from_value(self.0.propose_block_trade(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn reset_order_group(&self, request: &ResetOrderGroupParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.reset_order_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn set_event_contract_exit_trigger(&self, request: &SetEventContractExitTriggerParams) -> Result<Box<PredictionsEventContractExitTrigger>> {
        serde_json::from_value(self.0.set_event_contract_exit_trigger(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn set_target_balance_allocation(&self, request: &SetTargetBalanceAllocationParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.set_target_balance_allocation(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn trigger_order_group(&self, request: &TriggerOrderGroupParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.trigger_order_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn update_fcm_event_contract_daily_cap(&self, request: &UpdateFcmEventContractDailyCapParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.update_fcm_event_contract_daily_cap(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn update_fcm_subtrader_blocked_categories(&self, request: &UpdateFcmSubtraderBlockedCategoriesParams) -> Result<Box<PredictionsUpdateFCMSubtraderBlockedCategoriesResponse>> {
        serde_json::from_value(self.0.update_fcm_subtrader_blocked_categories(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn update_order_group_limit(&self, request: &UpdateOrderGroupLimitParams) -> Result<Box<PredictionsEmptyResponse>> {
        serde_json::from_value(self.0.update_order_group_limit(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn update_subaccount_netting(&self, request: &UpdateSubaccountNettingParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.update_subaccount_netting(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
    pub async fn upgrade_account_api_usage_level(&self, request: &UpgradeAccountApiUsageLevelParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.upgrade_account_api_usage_level(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kalshi response model"))
    }
}
