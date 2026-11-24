// Generated from src/exchanges/kalshi/schema/kalshi_perps.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{KalshiPerpsClient, Error, Result};

pub type CancelAllMarginOrdersResponse = serde_json::Value;
pub type ClearingAssetClass = String;
pub type ClearingDeleteMarginFcmApiKeyResponse = serde_json::Value;
pub type ClearingDeleteMarginSubtraderGroupResponse = serde_json::Value;
pub type ClearingUpdateMarginSubtraderGroupResponse = serde_json::Value;
pub type DeleteCrossMarginExitTriggerByIdResponse = serde_json::Value;
pub type DeleteCrossMarginExitTriggerResponse = serde_json::Value;
pub type DeleteIsolatedMarginExitTriggerResponse = serde_json::Value;
pub type GetMarginMarketSchedulesResponse = std::collections::BTreeMap<String,Box<PerpsMarginMarketScheduleDetails>>;
pub type PerpsBookSide = String;
pub type PerpsExchangeIndex = serde_json::Number;
pub type PerpsExchangeInstance = String;
pub type PerpsExitTriggerKind = String;
pub type PerpsExitTriggerLeg = String;
pub type PerpsExitTriggerReason = String;
pub type PerpsExitTriggerStatus = String;
pub type PerpsFixedPointCount = String;
pub type PerpsFixedPointDollars = String;
pub type PerpsGetMarginMarketSchedulesResponse = std::collections::BTreeMap<String,Box<PerpsMarginMarketScheduleDetails>>;
pub type PerpsLastUpdateReason = String;
pub type PerpsMarginMarketStatus = String;
pub type PerpsOrderReason = String;
pub type PerpsOrderSource = String;
pub type PerpsPriceLevelDollarsCountFp = Vec<String>;
pub type PerpsSelfTradePreventionType = String;
pub type PerpsSubaccountType = String;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendMarginOrderParams {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "body")]
    pub body: Box<PerpsAmendMarginOrderRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AmendMarginOrderResponse {
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
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApplyMarginSubaccountTransferParams {
    #[serde(rename = "body")]
    pub body: Box<PerpsApplySubaccountTransferRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApplyMarginSubaccountTransferResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelAllMarginOrdersParams {
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelMarginOrderParams {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelMarginOrderResponse {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "reduced_by")]
    pub reduced_by: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingAssetClassFundingEstimate {
    #[serde(rename = "user_breakdown")]
    pub user_breakdown: Box<ClearingFundingEstimate>,
    #[serde(rename = "subtrader_breakdowns", default, skip_serializing_if = "Option::is_none")]
    pub subtrader_breakdowns: Option<std::collections::BTreeMap<String,Box<ClearingFundingEstimate>>>,
    #[serde(rename = "group_breakdowns", default, skip_serializing_if = "Option::is_none")]
    pub group_breakdowns: Option<std::collections::BTreeMap<String,Box<ClearingFundingEstimate>>>,
    #[serde(rename = "omitted_subtrader_count")]
    pub omitted_subtrader_count: serde_json::Number,
    #[serde(rename = "omitted_group_count")]
    pub omitted_group_count: serde_json::Number,
    #[serde(rename = "next_funding_time")]
    pub next_funding_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingAssetClassFundingEstimate {
    fn default() -> Self {
        Self {
            user_breakdown: Default::default(),
            subtrader_breakdowns: Default::default(),
            group_breakdowns: Default::default(),
            omitted_subtrader_count: 0.into(),
            omitted_group_count: 0.into(),
            next_funding_time: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingAssetClassSettlementEstimate {
    #[serde(rename = "user_breakdown", default, skip_serializing_if = "Option::is_none")]
    pub user_breakdown: Option<Box<ClearingSettlementEstimate>>,
    #[serde(rename = "subtrader_breakdowns", default, skip_serializing_if = "Option::is_none")]
    pub subtrader_breakdowns: Option<std::collections::BTreeMap<String,Box<ClearingSettlementEstimate>>>,
    #[serde(rename = "group_breakdowns", default, skip_serializing_if = "Option::is_none")]
    pub group_breakdowns: Option<std::collections::BTreeMap<String,Box<ClearingSettlementEstimate>>>,
    #[serde(rename = "omitted_subtrader_count", default, skip_serializing_if = "Option::is_none")]
    pub omitted_subtrader_count: Option<serde_json::Number>,
    #[serde(rename = "omitted_group_count", default, skip_serializing_if = "Option::is_none")]
    pub omitted_group_count: Option<serde_json::Number>,
    #[serde(rename = "next_runtime")]
    pub next_runtime: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingCorporateActionPaymentDetail {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "ca_id")]
    pub ca_id: String,
    #[serde(rename = "market_id")]
    pub market_id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "asset_class")]
    pub asset_class: String,
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "position_quantity_fp")]
    pub position_quantity_fp: String,
    #[serde(rename = "post_position_quantity_fp")]
    pub post_position_quantity_fp: String,
    #[serde(rename = "payment_amount_centicents")]
    pub payment_amount_centicents: serde_json::Number,
    #[serde(rename = "payment_amount_fp")]
    pub payment_amount_fp: String,
    #[serde(rename = "payment_time", skip_serializing_if = "Option::is_none")]
    pub payment_time: Option<String>,
    #[serde(rename = "settlement_execution_time")]
    pub settlement_execution_time: String,
    #[serde(rename = "pre_split_price_centicents", default, skip_serializing_if = "Option::is_none")]
    pub pre_split_price_centicents: Option<serde_json::Number>,
    #[serde(rename = "post_split_price_centicents", default, skip_serializing_if = "Option::is_none")]
    pub post_split_price_centicents: Option<serde_json::Number>,
    #[serde(rename = "exchange_ts", default, skip_serializing_if = "Option::is_none")]
    pub exchange_ts: Option<String>,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingCorporateActionPaymentDetail {
    fn default() -> Self {
        Self {
            id: Default::default(),
            type_: Default::default(),
            ca_id: Default::default(),
            market_id: Default::default(),
            market_ticker: Default::default(),
            asset_class: Default::default(),
            subtrader_id: Default::default(),
            position_quantity_fp: Default::default(),
            post_position_quantity_fp: Default::default(),
            payment_amount_centicents: 0.into(),
            payment_amount_fp: Default::default(),
            payment_time: Default::default(),
            settlement_execution_time: Default::default(),
            pre_split_price_centicents: Default::default(),
            post_split_price_centicents: Default::default(),
            exchange_ts: Default::default(),
            created_ts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingCreateMarginFcmApiKeyParams {
    #[serde(rename = "body")]
    pub body: Box<ClearingCreateMarginFcmApiKeyRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingCreateMarginFcmApiKeyRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "fcm_subtrader_id")]
    pub fcm_subtrader_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingCreateMarginFcmApiKeyResponse {
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    #[serde(rename = "warning", default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingCreateMarginSubtraderGroupParams {
    #[serde(rename = "body")]
    pub body: Box<ClearingCreateMarginSubtraderGroupRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingCreateMarginSubtraderGroupRequest {
    #[serde(rename = "subtrader_ids")]
    pub subtrader_ids: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingCreateMarginSubtraderGroupResponse {
    #[serde(rename = "group_id")]
    pub group_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingDeleteMarginFcmApiKeyParams {
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingDeleteMarginSubtraderGroupParams {
    #[serde(rename = "group_id")]
    pub group_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingError {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "service", default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingEstimatePortfolioMaintenanceMarginParams {
    #[serde(rename = "body")]
    pub body: Box<ClearingEstimatePortfolioMaintenanceMarginRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingEstimatePortfolioMaintenanceMarginPosition {
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "quantity")]
    pub quantity: serde_json::Number,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingEstimatePortfolioMaintenanceMarginPosition {
    fn default() -> Self {
        Self {
            market_ticker: Default::default(),
            quantity: 0.into(),
            price: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingEstimatePortfolioMaintenanceMarginRequest {
    #[serde(rename = "asset_class")]
    pub asset_class: String,
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "clearing_type", default, skip_serializing_if = "Option::is_none")]
    pub clearing_type: Option<String>,
    #[serde(rename = "positions")]
    pub positions: Vec<ClearingEstimatePortfolioMaintenanceMarginPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingEstimatePortfolioMaintenanceMarginResponse {
    #[serde(rename = "maintenance_margin_fp", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_margin_fp: Option<String>,
    #[serde(rename = "base_margin_fp", default, skip_serializing_if = "Option::is_none")]
    pub base_margin_fp: Option<String>,
    #[serde(rename = "hvar_fp", default, skip_serializing_if = "Option::is_none")]
    pub hvar_fp: Option<String>,
    #[serde(rename = "apc_fp", default, skip_serializing_if = "Option::is_none")]
    pub apc_fp: Option<String>,
    #[serde(rename = "funding_addon_fp", default, skip_serializing_if = "Option::is_none")]
    pub funding_addon_fp: Option<String>,
    #[serde(rename = "liquidation_addon_fp", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_addon_fp: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingFundingEstimate {
    #[serde(rename = "funding_amount_centicents")]
    pub funding_amount_centicents: serde_json::Number,
    #[serde(rename = "funding_amount_fp")]
    pub funding_amount_fp: String,
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<std::collections::BTreeMap<String,Box<ClearingMarketFundingEstimate>>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingFundingEstimate {
    fn default() -> Self {
        Self {
            funding_amount_centicents: 0.into(),
            funding_amount_fp: Default::default(),
            positions: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingFundingPaymentDetail {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "funding_time")]
    pub funding_time: String,
    #[serde(rename = "position_quantity_fp")]
    pub position_quantity_fp: String,
    #[serde(rename = "notional_value_centicents")]
    pub notional_value_centicents: serde_json::Number,
    #[serde(rename = "notional_value_fp")]
    pub notional_value_fp: String,
    #[serde(rename = "funding_amount_centicents")]
    pub funding_amount_centicents: serde_json::Number,
    #[serde(rename = "funding_amount_fp")]
    pub funding_amount_fp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingFundingPaymentDetail {
    fn default() -> Self {
        Self {
            id: Default::default(),
            market_ticker: Default::default(),
            subtrader_id: Default::default(),
            funding_time: Default::default(),
            position_quantity_fp: Default::default(),
            notional_value_centicents: 0.into(),
            notional_value_fp: Default::default(),
            funding_amount_centicents: 0.into(),
            funding_amount_fp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGenerateMarginFcmApiKeyParams {
    #[serde(rename = "body")]
    pub body: Box<ClearingGenerateMarginFcmApiKeyRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGenerateMarginFcmApiKeyRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "fcm_subtrader_id")]
    pub fcm_subtrader_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGenerateMarginFcmApiKeyResponse {
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    #[serde(rename = "private_key")]
    pub private_key: String,
    #[serde(rename = "warning", default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetActiveMarginObligationsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetActiveMarginObligationsResponse {
    #[serde(rename = "obligations")]
    pub obligations: Vec<ClearingObligationEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetFundingEstimateByAssetClassParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetFundingEstimateByAssetClassResponse {
    #[serde(rename = "estimates")]
    pub estimates: std::collections::BTreeMap<String,Box<ClearingAssetClassFundingEstimate>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetGuarantyFundBalanceParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingGetGuarantyFundBalanceResponse {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "amount_centicents")]
    pub amount_centicents: serde_json::Number,
    #[serde(rename = "updated_ts")]
    pub updated_ts: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingGetGuarantyFundBalanceResponse {
    fn default() -> Self {
        Self {
            user_id: Default::default(),
            amount_centicents: 0.into(),
            updated_ts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetLargeTraderPositionsParams {
    #[serde(rename = "report_date", default, skip_serializing_if = "Option::is_none")]
    pub report_date: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetLargeTraderPositionsResponse {
    #[serde(rename = "user_id", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "report_date", default, skip_serializing_if = "Option::is_none")]
    pub report_date: Option<String>,
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<ClearingGetLargeTraderPositionsResponsePositionsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetLargeTraderPositionsResponsePositionsItem {
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "position", default, skip_serializing_if = "Option::is_none")]
    pub position: Option<serde_json::Number>,
    #[serde(rename = "exit_day", default, skip_serializing_if = "Option::is_none")]
    pub exit_day: Option<bool>,
    #[serde(rename = "reportable_threshold", default, skip_serializing_if = "Option::is_none")]
    pub reportable_threshold: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetMaintenanceMarginMetadataParams {
    #[serde(rename = "asset_class")]
    pub asset_class: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingGetMaintenanceMarginMetadataResponse {
    #[serde(rename = "asset_class")]
    pub asset_class: String,
    #[serde(rename = "base_tail_percentile")]
    pub base_tail_percentile: serde_json::Number,
    #[serde(rename = "funding_tail_percentile")]
    pub funding_tail_percentile: serde_json::Number,
    #[serde(rename = "matrices")]
    pub matrices: Box<ClearingMaintenanceMarginMatrices>,
    #[serde(rename = "liquidation_configs")]
    pub liquidation_configs: Vec<ClearingMaintenanceMarginLiquidationConfig>,
    #[serde(rename = "subgroups")]
    pub subgroups: Vec<Vec<String>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingGetMaintenanceMarginMetadataResponse {
    fn default() -> Self {
        Self {
            asset_class: Default::default(),
            base_tail_percentile: 0.into(),
            funding_tail_percentile: 0.into(),
            matrices: Default::default(),
            liquidation_configs: Default::default(),
            subgroups: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetMarginFundingScheduleParams {
    #[serde(rename = "asset_class")]
    pub asset_class: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetMarginFundingScheduleResponse {
    #[serde(rename = "schedule")]
    pub schedule: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetMarginReportsParams {
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetMarginReportsResponse {
    #[serde(rename = "reports")]
    pub reports: Vec<ClearingMarginReport>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetMarginSubtraderGroupsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetMarginSubtraderGroupsResponse {
    #[serde(rename = "groups")]
    pub groups: Vec<ClearingMarginSubtraderGroup>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetMemberFundingPaymentsParams {
    #[serde(rename = "funding_time")]
    pub funding_time: String,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetMemberFundingPaymentsResponse {
    #[serde(rename = "payments")]
    pub payments: Vec<ClearingMemberFundingPayment>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetObligationCorporateActionPaymentsParams {
    #[serde(rename = "obligation_id")]
    pub obligation_id: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetObligationCorporateActionPaymentsResponse {
    #[serde(rename = "corporate_action_payments")]
    pub corporate_action_payments: Vec<ClearingCorporateActionPaymentDetail>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetObligationFundingPaymentsParams {
    #[serde(rename = "obligation_id")]
    pub obligation_id: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetObligationFundingPaymentsResponse {
    #[serde(rename = "funding_payments")]
    pub funding_payments: Vec<ClearingFundingPaymentDetail>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetObligationHistoryParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "settlement_start_ts", default, skip_serializing_if = "Option::is_none")]
    pub settlement_start_ts: Option<String>,
    #[serde(rename = "settlement_end_ts", default, skip_serializing_if = "Option::is_none")]
    pub settlement_end_ts: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetObligationHistoryResponse {
    #[serde(rename = "obligations")]
    pub obligations: Vec<ClearingObligationEntry>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetObligationMaintenanceMarginDetailsParams {
    #[serde(rename = "obligation_id")]
    pub obligation_id: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetObligationMaintenanceMarginDetailsResponse {
    #[serde(rename = "maintenance_margin_details")]
    pub maintenance_margin_details: Vec<ClearingMaintenanceMarginDetail>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetObligationSettlementDetailsParams {
    #[serde(rename = "obligation_id")]
    pub obligation_id: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetObligationSettlementDetailsResponse {
    #[serde(rename = "settlement_details")]
    pub settlement_details: Vec<ClearingSettlementDetail>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetSettlementBalanceHistoryParams {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "start_ts", default, skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<String>,
    #[serde(rename = "end_ts", default, skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetSettlementBalanceHistoryResponse {
    #[serde(rename = "entries")]
    pub entries: Vec<ClearingSettlementBalanceHistoryEntry>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetSettlementBalanceParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingGetSettlementBalanceResponse {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "balance_available_centicents")]
    pub balance_available_centicents: serde_json::Number,
    #[serde(rename = "balance_available_fp")]
    pub balance_available_fp: String,
    #[serde(rename = "locked_balance_centicents", default, skip_serializing_if = "Option::is_none")]
    pub locked_balance_centicents: Option<serde_json::Number>,
    #[serde(rename = "locked_balance_fp", default, skip_serializing_if = "Option::is_none")]
    pub locked_balance_fp: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingGetSettlementBalanceResponse {
    fn default() -> Self {
        Self {
            user_id: Default::default(),
            balance_available_centicents: 0.into(),
            balance_available_fp: Default::default(),
            locked_balance_centicents: Default::default(),
            locked_balance_fp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetSettlementBalanceWithdrawalParams {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetSettlementBalanceWithdrawalResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetSettlementEstimateByAssetClassParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingGetSettlementEstimateByAssetClassResponse {
    #[serde(rename = "estimates")]
    pub estimates: std::collections::BTreeMap<String,Box<ClearingAssetClassSettlementEstimate>>,
    #[serde(rename = "settlement_balance_centicents")]
    pub settlement_balance_centicents: serde_json::Number,
    #[serde(rename = "settlement_balance_fp")]
    pub settlement_balance_fp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingGetSettlementEstimateByAssetClassResponse {
    fn default() -> Self {
        Self {
            estimates: Default::default(),
            settlement_balance_centicents: 0.into(),
            settlement_balance_fp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetSettlementPricesParams {
    #[serde(rename = "asset_class")]
    pub asset_class: String,
    #[serde(rename = "settlement_time")]
    pub settlement_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingGetSettlementPricesResponse {
    #[serde(rename = "settlement_prices")]
    pub settlement_prices: std::collections::BTreeMap<String,serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingListMarginFcmApiKeysParams {
    #[serde(rename = "fcm_subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub fcm_subtrader_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingListMarginFcmApiKeysResponse {
    #[serde(rename = "api_keys")]
    pub api_keys: Vec<ClearingMarginFcmApiKey>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingMaintenanceMarginDetail {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "margin_group_id", default, skip_serializing_if = "Option::is_none")]
    pub margin_group_id: Option<String>,
    #[serde(rename = "maintenance_margin_centicents")]
    pub maintenance_margin_centicents: serde_json::Number,
    #[serde(rename = "maintenance_margin_delta_centicents")]
    pub maintenance_margin_delta_centicents: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingMaintenanceMarginDetail {
    fn default() -> Self {
        Self {
            id: Default::default(),
            subtrader_id: Default::default(),
            margin_group_id: Default::default(),
            maintenance_margin_centicents: 0.into(),
            maintenance_margin_delta_centicents: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingMaintenanceMarginLiquidationConfig {
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "market_impact_volatility")]
    pub market_impact_volatility: serde_json::Number,
    #[serde(rename = "market_impact_coefficient")]
    pub market_impact_coefficient: serde_json::Number,
    #[serde(rename = "market_impact_exponent")]
    pub market_impact_exponent: serde_json::Number,
    #[serde(rename = "market_impact_forecasted_volume")]
    pub market_impact_forecasted_volume: serde_json::Number,
    #[serde(rename = "spread_rate")]
    pub spread_rate: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingMaintenanceMarginLiquidationConfig {
    fn default() -> Self {
        Self {
            market_ticker: Default::default(),
            market_impact_volatility: 0.into(),
            market_impact_coefficient: 0.into(),
            market_impact_exponent: 0.into(),
            market_impact_forecasted_volume: 0.into(),
            spread_rate: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingMaintenanceMarginMatrices {
    #[serde(rename = "hvar", default, skip_serializing_if = "Option::is_none")]
    pub hvar: Option<Box<ClearingMaintenanceMarginMatrix>>,
    #[serde(rename = "apc", default, skip_serializing_if = "Option::is_none")]
    pub apc: Option<Box<ClearingMaintenanceMarginMatrix>>,
    #[serde(rename = "aug", default, skip_serializing_if = "Option::is_none")]
    pub aug: Option<Box<ClearingMaintenanceMarginMatrix>>,
    #[serde(rename = "funding", default, skip_serializing_if = "Option::is_none")]
    pub funding: Option<Box<ClearingMaintenanceMarginMatrix>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingMaintenanceMarginMatrix {
    #[serde(rename = "market_tickers")]
    pub market_tickers: Vec<String>,
    #[serde(rename = "returns")]
    pub returns: Vec<Vec<serde_json::Number>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingMarginFcmApiKey {
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "fcm_subtrader_id")]
    pub fcm_subtrader_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingMarginReport {
    #[serde(rename = "report_type")]
    pub report_type: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(rename = "is_end_of_day")]
    pub is_end_of_day: bool,
    #[serde(rename = "snapshot_ts", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_ts: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingMarginSubtraderGroup {
    #[serde(rename = "group_id")]
    pub group_id: String,
    #[serde(rename = "member_subtrader_ids")]
    pub member_subtrader_ids: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingMarketFundingEstimate {
    #[serde(rename = "quantity_centicount")]
    pub quantity_centicount: serde_json::Number,
    #[serde(rename = "funding_amount_centicents")]
    pub funding_amount_centicents: serde_json::Number,
    #[serde(rename = "funding_amount_fp")]
    pub funding_amount_fp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingMarketFundingEstimate {
    fn default() -> Self {
        Self {
            quantity_centicount: 0.into(),
            funding_amount_centicents: 0.into(),
            funding_amount_fp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingMarketSettlementEstimate {
    #[serde(rename = "quantity_centicount")]
    pub quantity_centicount: serde_json::Number,
    #[serde(rename = "variation_margin_centicents")]
    pub variation_margin_centicents: serde_json::Number,
    #[serde(rename = "variation_margin_fp")]
    pub variation_margin_fp: String,
    #[serde(rename = "notional_value_centicents")]
    pub notional_value_centicents: serde_json::Number,
    #[serde(rename = "notional_value_fp")]
    pub notional_value_fp: String,
    #[serde(rename = "session_avg_price_fp", default, skip_serializing_if = "Option::is_none")]
    pub session_avg_price_fp: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingMarketSettlementEstimate {
    fn default() -> Self {
        Self {
            quantity_centicount: 0.into(),
            variation_margin_centicents: 0.into(),
            variation_margin_fp: Default::default(),
            notional_value_centicents: 0.into(),
            notional_value_fp: Default::default(),
            session_avg_price_fp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingMemberFundingPayment {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "funding_time")]
    pub funding_time: String,
    #[serde(rename = "position_quantity_fp")]
    pub position_quantity_fp: String,
    #[serde(rename = "notional_value_centicents")]
    pub notional_value_centicents: serde_json::Number,
    #[serde(rename = "notional_value_fp")]
    pub notional_value_fp: String,
    #[serde(rename = "funding_amount_centicents")]
    pub funding_amount_centicents: serde_json::Number,
    #[serde(rename = "funding_amount_fp")]
    pub funding_amount_fp: String,
    #[serde(rename = "settlement_execution_time")]
    pub settlement_execution_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingMemberFundingPayment {
    fn default() -> Self {
        Self {
            id: Default::default(),
            market_ticker: Default::default(),
            subtrader_id: Default::default(),
            funding_time: Default::default(),
            position_quantity_fp: Default::default(),
            notional_value_centicents: 0.into(),
            notional_value_fp: Default::default(),
            funding_amount_centicents: 0.into(),
            funding_amount_fp: Default::default(),
            settlement_execution_time: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingObligationEntry {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "amount_centicents")]
    pub amount_centicents: serde_json::Number,
    #[serde(rename = "amount_fp")]
    pub amount_fp: String,
    #[serde(rename = "fees_centicents")]
    pub fees_centicents: serde_json::Number,
    #[serde(rename = "maintenance_margin_centicents")]
    pub maintenance_margin_centicents: serde_json::Number,
    #[serde(rename = "pnl_centicents")]
    pub pnl_centicents: serde_json::Number,
    #[serde(rename = "pnl_fp")]
    pub pnl_fp: String,
    #[serde(rename = "execution_time")]
    pub execution_time: String,
    #[serde(rename = "last_updated_ts")]
    pub last_updated_ts: String,
    #[serde(rename = "asset_class")]
    pub asset_class: String,
    #[serde(rename = "receives")]
    pub receives: Vec<ClearingObligationReceiveInfo>,
    #[serde(rename = "settlement_details")]
    pub settlement_details: Vec<ClearingSettlementDetail>,
    #[serde(rename = "maintenance_margin_details")]
    pub maintenance_margin_details: Vec<ClearingMaintenanceMarginDetail>,
    #[serde(rename = "funding_payments")]
    pub funding_payments: Vec<ClearingFundingPaymentDetail>,
    #[serde(rename = "corporate_action_payments")]
    pub corporate_action_payments: Vec<ClearingCorporateActionPaymentDetail>,
    #[serde(rename = "corporate_action_payments_truncated", default, skip_serializing_if = "Option::is_none")]
    pub corporate_action_payments_truncated: Option<bool>,
    #[serde(rename = "settlement_details_truncated", default, skip_serializing_if = "Option::is_none")]
    pub settlement_details_truncated: Option<bool>,
    #[serde(rename = "maintenance_margin_details_truncated", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_margin_details_truncated: Option<bool>,
    #[serde(rename = "funding_payments_truncated", default, skip_serializing_if = "Option::is_none")]
    pub funding_payments_truncated: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingObligationEntry {
    fn default() -> Self {
        Self {
            id: Default::default(),
            user_id: Default::default(),
            amount_centicents: 0.into(),
            amount_fp: Default::default(),
            fees_centicents: 0.into(),
            maintenance_margin_centicents: 0.into(),
            pnl_centicents: 0.into(),
            pnl_fp: Default::default(),
            execution_time: Default::default(),
            last_updated_ts: Default::default(),
            asset_class: Default::default(),
            receives: Default::default(),
            settlement_details: Default::default(),
            maintenance_margin_details: Default::default(),
            funding_payments: Default::default(),
            corporate_action_payments: Default::default(),
            corporate_action_payments_truncated: Default::default(),
            settlement_details_truncated: Default::default(),
            maintenance_margin_details_truncated: Default::default(),
            funding_payments_truncated: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingObligationInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "amount_centicents")]
    pub amount_centicents: serde_json::Number,
    #[serde(rename = "amount_fp")]
    pub amount_fp: String,
    #[serde(rename = "fees_centicents")]
    pub fees_centicents: serde_json::Number,
    #[serde(rename = "maintenance_margin_centicents")]
    pub maintenance_margin_centicents: serde_json::Number,
    #[serde(rename = "pnl_centicents")]
    pub pnl_centicents: serde_json::Number,
    #[serde(rename = "pnl_fp")]
    pub pnl_fp: String,
    #[serde(rename = "execution_time")]
    pub execution_time: String,
    #[serde(rename = "last_updated_ts")]
    pub last_updated_ts: String,
    #[serde(rename = "asset_class")]
    pub asset_class: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingObligationInfo {
    fn default() -> Self {
        Self {
            id: Default::default(),
            user_id: Default::default(),
            amount_centicents: 0.into(),
            amount_fp: Default::default(),
            fees_centicents: 0.into(),
            maintenance_margin_centicents: 0.into(),
            pnl_centicents: 0.into(),
            pnl_fp: Default::default(),
            execution_time: Default::default(),
            last_updated_ts: Default::default(),
            asset_class: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingObligationReceiveInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "amount_centicents")]
    pub amount_centicents: serde_json::Number,
    #[serde(rename = "amount_fp")]
    pub amount_fp: String,
    #[serde(rename = "external_reference")]
    pub external_reference: String,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingObligationReceiveInfo {
    fn default() -> Self {
        Self {
            id: Default::default(),
            type_: Default::default(),
            amount_centicents: 0.into(),
            amount_fp: Default::default(),
            external_reference: Default::default(),
            created_ts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingSettlementBalanceHistoryEntry {
    #[serde(rename = "balance_delta_centicents")]
    pub balance_delta_centicents: serde_json::Number,
    #[serde(rename = "balance_delta_fp")]
    pub balance_delta_fp: String,
    #[serde(rename = "locked_balance_delta_centicents")]
    pub locked_balance_delta_centicents: serde_json::Number,
    #[serde(rename = "locked_balance_delta_fp")]
    pub locked_balance_delta_fp: String,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "business_transaction_id")]
    pub business_transaction_id: String,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingSettlementBalanceHistoryEntry {
    fn default() -> Self {
        Self {
            balance_delta_centicents: 0.into(),
            balance_delta_fp: Default::default(),
            locked_balance_delta_centicents: 0.into(),
            locked_balance_delta_fp: Default::default(),
            reason: Default::default(),
            business_transaction_id: Default::default(),
            created_ts: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingSettlementDetail {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "position_quantity_fp")]
    pub position_quantity_fp: String,
    #[serde(rename = "pnl_centicents")]
    pub pnl_centicents: serde_json::Number,
    #[serde(rename = "pnl_fp")]
    pub pnl_fp: String,
    #[serde(rename = "total_fees_centicents")]
    pub total_fees_centicents: serde_json::Number,
    #[serde(rename = "total_amount_centicents")]
    pub total_amount_centicents: serde_json::Number,
    #[serde(rename = "total_amount_fp")]
    pub total_amount_fp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingSettlementDetail {
    fn default() -> Self {
        Self {
            id: Default::default(),
            market_ticker: Default::default(),
            subtrader_id: Default::default(),
            position_quantity_fp: Default::default(),
            pnl_centicents: 0.into(),
            pnl_fp: Default::default(),
            total_fees_centicents: 0.into(),
            total_amount_centicents: 0.into(),
            total_amount_fp: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClearingSettlementEstimate {
    #[serde(rename = "variation_margin_centicents")]
    pub variation_margin_centicents: serde_json::Number,
    #[serde(rename = "variation_margin_fp")]
    pub variation_margin_fp: String,
    #[serde(rename = "total_fees_centicents")]
    pub total_fees_centicents: serde_json::Number,
    #[serde(rename = "maintenance_margin_delta_centicents")]
    pub maintenance_margin_delta_centicents: serde_json::Number,
    #[serde(rename = "maintenance_margin_required_centicents")]
    pub maintenance_margin_required_centicents: serde_json::Number,
    #[serde(rename = "total_amount_centicents")]
    pub total_amount_centicents: serde_json::Number,
    #[serde(rename = "total_amount_fp")]
    pub total_amount_fp: String,
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<std::collections::BTreeMap<String,Box<ClearingMarketSettlementEstimate>>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ClearingSettlementEstimate {
    fn default() -> Self {
        Self {
            variation_margin_centicents: 0.into(),
            variation_margin_fp: Default::default(),
            total_fees_centicents: 0.into(),
            maintenance_margin_delta_centicents: 0.into(),
            maintenance_margin_required_centicents: 0.into(),
            total_amount_centicents: 0.into(),
            total_amount_fp: Default::default(),
            positions: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingUpdateMarginSubtraderGroupParams {
    #[serde(rename = "group_id")]
    pub group_id: String,
    #[serde(rename = "body")]
    pub body: Box<ClearingUpdateMarginSubtraderGroupRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingUpdateMarginSubtraderGroupRequest {
    #[serde(rename = "subtrader_ids")]
    pub subtrader_ids: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingWithdrawSettlementBalanceParams {
    #[serde(rename = "body")]
    pub body: Box<ClearingWithdrawSettlementBalanceRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingWithdrawSettlementBalanceRequest {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "mfa_code")]
    pub mfa_code: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClearingWithdrawSettlementBalanceResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateMarginFcmSubtraderParams {
    #[serde(rename = "body")]
    pub body: Box<PerpsCreateMarginFCMSubtraderRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateMarginFcmSubtraderResponse {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateMarginOrderGroupParams {
    #[serde(rename = "body")]
    pub body: Box<PerpsCreateOrderGroupRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateMarginOrderGroupResponse {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount")]
    pub subaccount: serde_json::Number,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateMarginOrderGroupResponse {
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
pub struct CreateMarginOrderParams {
    #[serde(rename = "body")]
    pub body: Box<PerpsCreateMarginOrderRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateMarginOrderResponse {
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
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateMarginSubaccountParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateMarginSubaccountResponse {
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for CreateMarginSubaccountResponse {
    fn default() -> Self {
        Self {
            subaccount_number: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DecreaseMarginOrderParams {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "body")]
    pub body: Box<PerpsDecreaseMarginOrderRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DecreaseMarginOrderResponse {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "remaining_count")]
    pub remaining_count: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteCrossMarginExitTriggerByIdParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "trigger_id")]
    pub trigger_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteCrossMarginExitTriggerParams {
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
pub struct DeleteFcmNotionalRiskLimitParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteFcmNotionalRiskLimitResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteFcmSubtraderRiskControlsParams {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "asset_class", default, skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteFcmSubtraderRiskControlsResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteIsolatedMarginExitTriggerParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteMarginOrderGroupParams {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DeleteMarginOrderGroupResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCrossMarginExitTriggersParams {
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
pub struct GetCrossMarginExitTriggersResponse {
    #[serde(rename = "exit_triggers")]
    pub exit_triggers: Vec<PerpsExitTrigger>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFcmSubtraderRiskControlsParams {
    #[serde(rename = "subtrader_id", default, skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "asset_class", default, skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFcmSubtraderRiskControlsResponse {
    #[serde(rename = "risk_controls")]
    pub risk_controls: Vec<PerpsFCMSubtraderRiskControls>,
    #[serde(rename = "notional_limits")]
    pub notional_limits: Vec<PerpsFCMSubtraderNotionalRiskLimit>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetIsolatedMarginExitTriggersParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetIsolatedMarginExitTriggersResponse {
    #[serde(rename = "exit_triggers")]
    pub exit_triggers: Vec<PerpsExitTrigger>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginBalanceParams {
    #[serde(rename = "compute_available_balance", default, skip_serializing_if = "Option::is_none")]
    pub compute_available_balance: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginBalanceResponse {
    #[serde(rename = "subaccount_balances")]
    pub subaccount_balances: Vec<PerpsMarginSubaccountBalance>,
    #[serde(rename = "settled_funds")]
    pub settled_funds: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginCorporateActionsParams {
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "market_id", default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginCorporateActionsResponse {
    #[serde(rename = "corporate_actions")]
    pub corporate_actions: Vec<PerpsMarginCorporateAction>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginEnabledParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginEnabledResponse {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginExchangeStatusParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginExchangeStatusResponse {
    #[serde(rename = "exchange_active")]
    pub exchange_active: bool,
    #[serde(rename = "trading_active")]
    pub trading_active: bool,
    #[serde(rename = "intra_exchange_transfers_active", default, skip_serializing_if = "Option::is_none")]
    pub intra_exchange_transfers_active: Option<bool>,
    #[serde(rename = "exchange_estimated_resume_time", default, skip_serializing_if = "Option::is_none")]
    pub exchange_estimated_resume_time: Option<String>,
    #[serde(rename = "exchange_index_statuses", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index_statuses: Option<Vec<PerpsExchangeIndexStatus>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginFeeTierRatesParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginFeeTierRatesResponse {
    #[serde(rename = "fee_tier_rates")]
    pub fee_tier_rates: Vec<PerpsMarginFeeTierRate>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginFeeTiersParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginFeeTiersResponse {
    #[serde(rename = "maker_fee_rates")]
    pub maker_fee_rates: std::collections::BTreeMap<String,serde_json::Number>,
    #[serde(rename = "taker_fee_rates")]
    pub taker_fee_rates: std::collections::BTreeMap<String,serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginFillsParams {
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
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
pub struct GetMarginFillsResponse {
    #[serde(rename = "fills")]
    pub fills: Vec<PerpsMarginFill>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginFundingHistoryParams {
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginFundingHistoryResponse {
    #[serde(rename = "funding_history")]
    pub funding_history: Vec<PerpsMarginFundingHistoryEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginFundingRateEstimateParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginFundingRateEstimateResponse {
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "computed_time", default, skip_serializing_if = "Option::is_none")]
    pub computed_time: Option<String>,
    #[serde(rename = "funding_rate", default, skip_serializing_if = "Option::is_none")]
    pub funding_rate: Option<serde_json::Number>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "next_funding_time")]
    pub next_funding_time: String,
    #[serde(rename = "premium_index", default, skip_serializing_if = "Option::is_none")]
    pub premium_index: Option<String>,
    #[serde(rename = "premium_index_ts", default, skip_serializing_if = "Option::is_none")]
    pub premium_index_ts: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginHistoricalFundingRatesParams {
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "start_ts", default, skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<serde_json::Number>,
    #[serde(rename = "end_ts", default, skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginHistoricalFundingRatesResponse {
    #[serde(rename = "funding_rates")]
    pub funding_rates: Vec<PerpsMarginFundingRate>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMarginMarketCandlesticksParams {
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
impl Default for GetMarginMarketCandlesticksParams {
    fn default() -> Self {
        Self {
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
pub struct GetMarginMarketCandlesticksResponse {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "candlesticks")]
    pub candlesticks: Vec<PerpsMarginMarketCandlestick>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginMarketOrderbookParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "depth", default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<serde_json::Number>,
    #[serde(rename = "aggregation_tick_size", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_tick_size: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginMarketOrderbookResponse {
    #[serde(rename = "orderbook")]
    pub orderbook: Box<PerpsMarginOrderbookCount>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginMarketOrderbooksParams {
    #[serde(rename = "tickers")]
    pub tickers: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginMarketOrderbooksResponse {
    #[serde(rename = "orderbooks")]
    pub orderbooks: Vec<PerpsMarginMarketOrderbook>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginMarketParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginMarketResponse {
    #[serde(rename = "market")]
    pub market: Box<PerpsMarginMarket>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginMarketSchedulesParams {
    #[serde(rename = "tickers")]
    pub tickers: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginMarketsParams {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginMarketsResponse {
    #[serde(rename = "markets")]
    pub markets: Vec<PerpsMarginMarket>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginNotionalRiskLimitParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginNotionalRiskLimitResponse {
    #[serde(rename = "default_notional_value_risk_limit")]
    pub default_notional_value_risk_limit: String,
    #[serde(rename = "notional_value_risk_limits_by_market_ticker")]
    pub notional_value_risk_limits_by_market_ticker: std::collections::BTreeMap<String,String>,
    #[serde(rename = "total_current_usage")]
    pub total_current_usage: String,
    #[serde(rename = "current_usage_by_market_ticker")]
    pub current_usage_by_market_ticker: std::collections::BTreeMap<String,String>,
    #[serde(rename = "member_notional_value_risk_limit", default, skip_serializing_if = "Option::is_none")]
    pub member_notional_value_risk_limit: Option<String>,
    #[serde(rename = "effective_account_notional_value_risk_limit", default, skip_serializing_if = "Option::is_none")]
    pub effective_account_notional_value_risk_limit: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginOrderGroupParams {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginOrderGroupResponse {
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
pub struct GetMarginOrderGroupsParams {
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginOrderGroupsResponse {
    #[serde(rename = "order_groups", default, skip_serializing_if = "Option::is_none")]
    pub order_groups: Option<Vec<PerpsOrderGroup>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginOrderParams {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginOrderResponse {
    #[serde(rename = "order")]
    pub order: Box<PerpsMarginOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginOrdersParams {
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
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginOrdersResponse {
    #[serde(rename = "orders")]
    pub orders: Vec<PerpsMarginOrder>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginPositionsParams {
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginPositionsResponse {
    #[serde(rename = "positions")]
    pub positions: Vec<PerpsMarginPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMarginPremiumIndexParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "start_ts")]
    pub start_ts: serde_json::Number,
    #[serde(rename = "end_ts")]
    pub end_ts: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetMarginPremiumIndexParams {
    fn default() -> Self {
        Self {
            ticker: Default::default(),
            start_ts: 0.into(),
            end_ts: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginPremiumIndexResponse {
    #[serde(rename = "points")]
    pub points: Vec<PerpsMarginPremiumIndexPoint>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginRiskParametersParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetMarginRiskParametersResponse {
    #[serde(rename = "liquidation_margin_ratio_threshold")]
    pub liquidation_margin_ratio_threshold: serde_json::Number,
    #[serde(rename = "queue_entry_margin_ratio_threshold")]
    pub queue_entry_margin_ratio_threshold: serde_json::Number,
    #[serde(rename = "initial_margin_multiplier")]
    pub initial_margin_multiplier: std::collections::BTreeMap<String,serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for GetMarginRiskParametersResponse {
    fn default() -> Self {
        Self {
            liquidation_margin_ratio_threshold: 0.into(),
            queue_entry_margin_ratio_threshold: 0.into(),
            initial_margin_multiplier: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginRiskParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginRiskResponse {
    #[serde(rename = "account_leverage", default, skip_serializing_if = "Option::is_none")]
    pub account_leverage: Option<serde_json::Number>,
    #[serde(rename = "total_position_notional")]
    pub total_position_notional: String,
    #[serde(rename = "total_maintenance_margin")]
    pub total_maintenance_margin: String,
    #[serde(rename = "positions")]
    pub positions: Vec<PerpsMarginRiskPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarginTradesParams {
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
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
pub struct GetMarginTradesResponse {
    #[serde(rename = "trades")]
    pub trades: Vec<PerpsMarginTrade>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPerpsAccountApiLimitsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetPerpsAccountApiLimitsResponse {
    #[serde(rename = "usage_tier")]
    pub usage_tier: String,
    #[serde(rename = "read")]
    pub read: Box<PerpsBucketLimit>,
    #[serde(rename = "write")]
    pub write: Box<PerpsBucketLimit>,
    #[serde(rename = "grants")]
    pub grants: Vec<PerpsApiUsageLevelGrant>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct IntraExchangeInstanceTransferParams {
    #[serde(rename = "body")]
    pub body: Box<PerpsIntraExchangeInstanceTransferRequest>,
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
pub struct PerpsAmendMarginOrderRequest {
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "market_id", default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
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
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsAmendMarginOrderResponse {
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
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsApiUsageLevelGrant {
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
pub struct PerpsApplySubaccountTransferRequest {
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
impl Default for PerpsApplySubaccountTransferRequest {
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
pub struct PerpsApplySubaccountTransferResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsBidAskDistributionHistorical {
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
pub struct PerpsBucketLimit {
    #[serde(rename = "refill_rate")]
    pub refill_rate: serde_json::Number,
    #[serde(rename = "bucket_capacity")]
    pub bucket_capacity: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsBucketLimit {
    fn default() -> Self {
        Self {
            refill_rate: 0.into(),
            bucket_capacity: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCancelMarginOrderResponse {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "reduced_by")]
    pub reduced_by: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCreateMarginFCMSubtraderRequest {
    #[serde(rename = "subtrader_suffix")]
    pub subtrader_suffix: String,
    #[serde(rename = "require_category_cap")]
    pub require_category_cap: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCreateMarginFCMSubtraderResponse {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCreateMarginOrderRequest {
    #[serde(rename = "ticker", default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "market_id", default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
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
    #[serde(rename = "market_version", default, skip_serializing_if = "Option::is_none")]
    pub market_version: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCreateMarginOrderResponse {
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
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsCreateOrderGroupRequest {
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
pub struct PerpsCreateOrderGroupResponse {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount")]
    pub subaccount: serde_json::Number,
    #[serde(rename = "exchange_index", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCreateOrderGroupResponse {
    fn default() -> Self {
        Self {
            order_group_id: Default::default(),
            subaccount: 0.into(),
            exchange_index: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsCreateSubaccountResponse {
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsCreateSubaccountResponse {
    fn default() -> Self {
        Self {
            subaccount_number: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsDecreaseMarginOrderRequest {
    #[serde(rename = "reduce_by", default, skip_serializing_if = "Option::is_none")]
    pub reduce_by: Option<String>,
    #[serde(rename = "reduce_to", default, skip_serializing_if = "Option::is_none")]
    pub reduce_to: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsDecreaseMarginOrderResponse {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "remaining_count")]
    pub remaining_count: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsEmptyResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsErrorResponse {
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
pub struct PerpsExchangeIndexStatus {
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
impl Default for PerpsExchangeIndexStatus {
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
pub struct PerpsExchangeStatus {
    #[serde(rename = "exchange_active")]
    pub exchange_active: bool,
    #[serde(rename = "trading_active")]
    pub trading_active: bool,
    #[serde(rename = "intra_exchange_transfers_active", default, skip_serializing_if = "Option::is_none")]
    pub intra_exchange_transfers_active: Option<bool>,
    #[serde(rename = "exchange_estimated_resume_time", default, skip_serializing_if = "Option::is_none")]
    pub exchange_estimated_resume_time: Option<String>,
    #[serde(rename = "exchange_index_statuses", default, skip_serializing_if = "Option::is_none")]
    pub exchange_index_statuses: Option<Vec<PerpsExchangeIndexStatus>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsExitTrigger {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "status_reason", default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "triggered_leg", default, skip_serializing_if = "Option::is_none")]
    pub triggered_leg: Option<String>,
    #[serde(rename = "triggered_order_id", default, skip_serializing_if = "Option::is_none")]
    pub triggered_order_id: Option<String>,
    #[serde(rename = "anchor_order_id", default, skip_serializing_if = "Option::is_none")]
    pub anchor_order_id: Option<String>,
    #[serde(rename = "client_trigger_id", default, skip_serializing_if = "Option::is_none")]
    pub client_trigger_id: Option<String>,
    #[serde(rename = "stop_loss_price", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "take_profit_price", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(rename = "trail_amount", default, skip_serializing_if = "Option::is_none")]
    pub trail_amount: Option<String>,
    #[serde(rename = "trail_bps", default, skip_serializing_if = "Option::is_none")]
    pub trail_bps: Option<serde_json::Number>,
    #[serde(rename = "watermark_price", default, skip_serializing_if = "Option::is_none")]
    pub watermark_price: Option<String>,
    #[serde(rename = "effective_stop_price", default, skip_serializing_if = "Option::is_none")]
    pub effective_stop_price: Option<String>,
    #[serde(rename = "count")]
    pub count: String,
    #[serde(rename = "filled_count")]
    pub filled_count: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "updated_time")]
    pub updated_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsFCMSubtraderNotionalRiskLimit {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "notional_value_risk_limit")]
    pub notional_value_risk_limit: String,
    #[serde(rename = "current_notional")]
    pub current_notional: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsFCMSubtraderRiskControls {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "asset_class", default, skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<String>,
    #[serde(rename = "im_cap")]
    pub im_cap: String,
    #[serde(rename = "current_im")]
    pub current_im: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetAccountApiLimitsResponse {
    #[serde(rename = "usage_tier")]
    pub usage_tier: String,
    #[serde(rename = "read")]
    pub read: Box<PerpsBucketLimit>,
    #[serde(rename = "write")]
    pub write: Box<PerpsBucketLimit>,
    #[serde(rename = "grants")]
    pub grants: Vec<PerpsApiUsageLevelGrant>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetExitTriggersResponse {
    #[serde(rename = "exit_triggers")]
    pub exit_triggers: Vec<PerpsExitTrigger>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetFCMSubtraderRiskControlsResponse {
    #[serde(rename = "risk_controls")]
    pub risk_controls: Vec<PerpsFCMSubtraderRiskControls>,
    #[serde(rename = "notional_limits")]
    pub notional_limits: Vec<PerpsFCMSubtraderNotionalRiskLimit>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginBalanceResponse {
    #[serde(rename = "subaccount_balances")]
    pub subaccount_balances: Vec<PerpsMarginSubaccountBalance>,
    #[serde(rename = "settled_funds")]
    pub settled_funds: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginCorporateActionsResponse {
    #[serde(rename = "corporate_actions")]
    pub corporate_actions: Vec<PerpsMarginCorporateAction>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginFeeTierRatesResponse {
    #[serde(rename = "fee_tier_rates")]
    pub fee_tier_rates: Vec<PerpsMarginFeeTierRate>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginFeeTiersResponse {
    #[serde(rename = "maker_fee_rates")]
    pub maker_fee_rates: std::collections::BTreeMap<String,serde_json::Number>,
    #[serde(rename = "taker_fee_rates")]
    pub taker_fee_rates: std::collections::BTreeMap<String,serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginFillsResponse {
    #[serde(rename = "fills")]
    pub fills: Vec<PerpsMarginFill>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginFundingHistoryResponse {
    #[serde(rename = "funding_history")]
    pub funding_history: Vec<PerpsMarginFundingHistoryEntry>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginFundingRateEstimateResponse {
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "computed_time", default, skip_serializing_if = "Option::is_none")]
    pub computed_time: Option<String>,
    #[serde(rename = "funding_rate", default, skip_serializing_if = "Option::is_none")]
    pub funding_rate: Option<serde_json::Number>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "next_funding_time")]
    pub next_funding_time: String,
    #[serde(rename = "premium_index", default, skip_serializing_if = "Option::is_none")]
    pub premium_index: Option<String>,
    #[serde(rename = "premium_index_ts", default, skip_serializing_if = "Option::is_none")]
    pub premium_index_ts: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginHistoricalFundingRatesResponse {
    #[serde(rename = "funding_rates")]
    pub funding_rates: Vec<PerpsMarginFundingRate>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginMarketCandlesticksResponse {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "candlesticks")]
    pub candlesticks: Vec<PerpsMarginMarketCandlestick>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginMarketOrderbooksResponse {
    #[serde(rename = "orderbooks")]
    pub orderbooks: Vec<PerpsMarginMarketOrderbook>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginMarketsResponse {
    #[serde(rename = "markets")]
    pub markets: Vec<PerpsMarginMarket>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginOrderResponse {
    #[serde(rename = "order")]
    pub order: Box<PerpsMarginOrder>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginOrdersResponse {
    #[serde(rename = "orders")]
    pub orders: Vec<PerpsMarginOrder>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginPositionsResponse {
    #[serde(rename = "positions")]
    pub positions: Vec<PerpsMarginPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginPremiumIndexResponse {
    #[serde(rename = "points")]
    pub points: Vec<PerpsMarginPremiumIndexPoint>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsGetMarginRiskParametersResponse {
    #[serde(rename = "liquidation_margin_ratio_threshold")]
    pub liquidation_margin_ratio_threshold: serde_json::Number,
    #[serde(rename = "queue_entry_margin_ratio_threshold")]
    pub queue_entry_margin_ratio_threshold: serde_json::Number,
    #[serde(rename = "initial_margin_multiplier")]
    pub initial_margin_multiplier: std::collections::BTreeMap<String,serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsGetMarginRiskParametersResponse {
    fn default() -> Self {
        Self {
            liquidation_margin_ratio_threshold: 0.into(),
            queue_entry_margin_ratio_threshold: 0.into(),
            initial_margin_multiplier: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginRiskResponse {
    #[serde(rename = "account_leverage", default, skip_serializing_if = "Option::is_none")]
    pub account_leverage: Option<serde_json::Number>,
    #[serde(rename = "total_position_notional")]
    pub total_position_notional: String,
    #[serde(rename = "total_maintenance_margin")]
    pub total_maintenance_margin: String,
    #[serde(rename = "positions")]
    pub positions: Vec<PerpsMarginRiskPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetMarginTradesResponse {
    #[serde(rename = "trades")]
    pub trades: Vec<PerpsMarginTrade>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsGetOrderGroupResponse {
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
pub struct PerpsGetOrderGroupsResponse {
    #[serde(rename = "order_groups", default, skip_serializing_if = "Option::is_none")]
    pub order_groups: Option<Vec<PerpsOrderGroup>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsIntraExchangeInstanceTransferRequest {
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
impl Default for PerpsIntraExchangeInstanceTransferRequest {
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
pub struct PerpsIntraExchangeInstanceTransferResponse {
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginCorporateAction {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "market_id")]
    pub market_id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "updated_ts")]
    pub updated_ts: String,
    #[serde(rename = "execution_time", default, skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<String>,
    #[serde(rename = "cash", default, skip_serializing_if = "Option::is_none")]
    pub cash: Option<Box<PerpsMarginCorporateActionCashTerms>>,
    #[serde(rename = "split", default, skip_serializing_if = "Option::is_none")]
    pub split: Option<Box<PerpsMarginCorporateActionSplitTerms>>,
    #[serde(rename = "final_settlement", default, skip_serializing_if = "Option::is_none")]
    pub final_settlement: Option<Box<PerpsMarginCorporateActionFinalSettlementTerms>>,
    #[serde(rename = "ticker_change", default, skip_serializing_if = "Option::is_none")]
    pub ticker_change: Option<Box<PerpsMarginCorporateActionTickerChangeTerms>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginCorporateActionCashTerms {
    #[serde(rename = "cash_per_share", default, skip_serializing_if = "Option::is_none")]
    pub cash_per_share: Option<String>,
    #[serde(rename = "cash_per_contract", default, skip_serializing_if = "Option::is_none")]
    pub cash_per_contract: Option<String>,
    #[serde(rename = "ex_date", default, skip_serializing_if = "Option::is_none")]
    pub ex_date: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginCorporateActionFinalSettlementTerms {
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "settlement_price_per_share", default, skip_serializing_if = "Option::is_none")]
    pub settlement_price_per_share: Option<String>,
    #[serde(rename = "settlement_price_per_contract", default, skip_serializing_if = "Option::is_none")]
    pub settlement_price_per_contract: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginCorporateActionSplitTerms {
    #[serde(rename = "ratio_old", default, skip_serializing_if = "Option::is_none")]
    pub ratio_old: Option<String>,
    #[serde(rename = "ratio_new", default, skip_serializing_if = "Option::is_none")]
    pub ratio_new: Option<String>,
    #[serde(rename = "ex_date", default, skip_serializing_if = "Option::is_none")]
    pub ex_date: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginCorporateActionTickerChangeTerms {
    #[serde(rename = "old_ticker")]
    pub old_ticker: String,
    #[serde(rename = "new_ticker")]
    pub new_ticker: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginEnabledResponse {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsMarginFeeTierRate {
    #[serde(rename = "fee_schedule")]
    pub fee_schedule: String,
    #[serde(rename = "tier")]
    pub tier: serde_json::Number,
    #[serde(rename = "maker_fee_rate")]
    pub maker_fee_rate: serde_json::Number,
    #[serde(rename = "taker_fee_rate")]
    pub taker_fee_rate: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsMarginFeeTierRate {
    fn default() -> Self {
        Self {
            fee_schedule: Default::default(),
            tier: 0.into(),
            maker_fee_rate: 0.into(),
            taker_fee_rate: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginFill {
    #[serde(rename = "fill_id")]
    pub fill_id: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "is_taker")]
    pub is_taker: bool,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "count")]
    pub count: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "entry_price")]
    pub entry_price: String,
    #[serde(rename = "fees")]
    pub fees: String,
    #[serde(rename = "realized_pnl")]
    pub realized_pnl: String,
    #[serde(rename = "order_source", default, skip_serializing_if = "Option::is_none")]
    pub order_source: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsMarginFundingHistoryEntry {
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "funding_time")]
    pub funding_time: String,
    #[serde(rename = "funding_rate")]
    pub funding_rate: serde_json::Number,
    #[serde(rename = "mark_price")]
    pub mark_price: String,
    #[serde(rename = "funding_amount")]
    pub funding_amount: String,
    #[serde(rename = "quantity")]
    pub quantity: String,
    #[serde(rename = "subaccount_number", skip_serializing_if = "Option::is_none")]
    pub subaccount_number: Option<serde_json::Number>,
    #[serde(rename = "subaccount_type")]
    pub subaccount_type: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsMarginFundingHistoryEntry {
    fn default() -> Self {
        Self {
            market_ticker: Default::default(),
            funding_time: Default::default(),
            funding_rate: 0.into(),
            mark_price: Default::default(),
            funding_amount: Default::default(),
            quantity: Default::default(),
            subaccount_number: Default::default(),
            subaccount_type: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsMarginFundingRate {
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "funding_time")]
    pub funding_time: String,
    #[serde(rename = "funding_rate")]
    pub funding_rate: serde_json::Number,
    #[serde(rename = "mark_price")]
    pub mark_price: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsMarginFundingRate {
    fn default() -> Self {
        Self {
            market_ticker: Default::default(),
            funding_time: Default::default(),
            funding_rate: 0.into(),
            mark_price: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginHolidayWindow {
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "until")]
    pub until: String,
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsMarginMarket {
    #[serde(rename = "market_id", default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "market_version")]
    pub market_version: serde_json::Number,
    #[serde(rename = "exchange_index")]
    pub exchange_index: serde_json::Number,
    #[serde(rename = "contract_size")]
    pub contract_size: String,
    #[serde(rename = "underlying_multiplier")]
    pub underlying_multiplier: String,
    #[serde(rename = "tick_size")]
    pub tick_size: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "fractional_trading_enabled")]
    pub fractional_trading_enabled: bool,
    #[serde(rename = "leverage_estimate", default, skip_serializing_if = "Option::is_none")]
    pub leverage_estimate: Option<serde_json::Number>,
    #[serde(rename = "leverage_estimates", default, skip_serializing_if = "Option::is_none")]
    pub leverage_estimates: Option<std::collections::BTreeMap<String,serde_json::Number>>,
    #[serde(rename = "long_leverage_estimates", default, skip_serializing_if = "Option::is_none")]
    pub long_leverage_estimates: Option<std::collections::BTreeMap<String,serde_json::Number>>,
    #[serde(rename = "short_leverage_estimates", default, skip_serializing_if = "Option::is_none")]
    pub short_leverage_estimates: Option<std::collections::BTreeMap<String,serde_json::Number>>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(rename = "volume_notional_value_dollars", default, skip_serializing_if = "Option::is_none")]
    pub volume_notional_value_dollars: Option<String>,
    #[serde(rename = "open_interest", default, skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<String>,
    #[serde(rename = "open_interest_notional_value_dollars", default, skip_serializing_if = "Option::is_none")]
    pub open_interest_notional_value_dollars: Option<String>,
    #[serde(rename = "volume_24h", default, skip_serializing_if = "Option::is_none")]
    pub volume_24h: Option<String>,
    #[serde(rename = "volume_24h_notional_value_dollars", default, skip_serializing_if = "Option::is_none")]
    pub volume_24h_notional_value_dollars: Option<String>,
    #[serde(rename = "bid", default, skip_serializing_if = "Option::is_none")]
    pub bid: Option<String>,
    #[serde(rename = "bid_size_fp", default, skip_serializing_if = "Option::is_none")]
    pub bid_size_fp: Option<String>,
    #[serde(rename = "ask", default, skip_serializing_if = "Option::is_none")]
    pub ask: Option<String>,
    #[serde(rename = "ask_size_fp", default, skip_serializing_if = "Option::is_none")]
    pub ask_size_fp: Option<String>,
    #[serde(rename = "settlement_mark_price", default, skip_serializing_if = "Option::is_none")]
    pub settlement_mark_price: Option<Box<PerpsTickerPrice>>,
    #[serde(rename = "liquidation_mark_price", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_mark_price: Option<Box<PerpsTickerPrice>>,
    #[serde(rename = "reference_price", default, skip_serializing_if = "Option::is_none")]
    pub reference_price: Option<Box<PerpsTickerPrice>>,
    #[serde(rename = "asset_class", default, skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<String>,
    #[serde(rename = "product_metadata", default, skip_serializing_if = "Option::is_none")]
    pub product_metadata: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(rename = "schedule")]
    pub schedule: Box<PerpsMarginMarketSchedule>,
    #[serde(rename = "funding_deadband")]
    pub funding_deadband: serde_json::Number,
    #[serde(rename = "funding_cap")]
    pub funding_cap: serde_json::Number,
    #[serde(rename = "funding_limit_unit")]
    pub funding_limit_unit: String,
    #[serde(rename = "halt_sources")]
    pub halt_sources: Vec<String>,
    #[serde(rename = "funding_invalid_premium_handling")]
    pub funding_invalid_premium_handling: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsMarginMarket {
    fn default() -> Self {
        Self {
            market_id: Default::default(),
            ticker: Default::default(),
            title: Default::default(),
            market_version: 0.into(),
            exchange_index: 0.into(),
            contract_size: Default::default(),
            underlying_multiplier: Default::default(),
            tick_size: Default::default(),
            status: Default::default(),
            fractional_trading_enabled: Default::default(),
            leverage_estimate: Default::default(),
            leverage_estimates: Default::default(),
            long_leverage_estimates: Default::default(),
            short_leverage_estimates: Default::default(),
            price: Default::default(),
            volume: Default::default(),
            volume_notional_value_dollars: Default::default(),
            open_interest: Default::default(),
            open_interest_notional_value_dollars: Default::default(),
            volume_24h: Default::default(),
            volume_24h_notional_value_dollars: Default::default(),
            bid: Default::default(),
            bid_size_fp: Default::default(),
            ask: Default::default(),
            ask_size_fp: Default::default(),
            settlement_mark_price: Default::default(),
            liquidation_mark_price: Default::default(),
            reference_price: Default::default(),
            asset_class: Default::default(),
            product_metadata: Default::default(),
            schedule: Default::default(),
            funding_deadband: 0.into(),
            funding_cap: 0.into(),
            funding_limit_unit: Default::default(),
            halt_sources: Default::default(),
            funding_invalid_premium_handling: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsMarginMarketCandlestick {
    #[serde(rename = "end_period_ts")]
    pub end_period_ts: serde_json::Number,
    #[serde(rename = "bid")]
    pub bid: Box<PerpsBidAskDistributionHistorical>,
    #[serde(rename = "ask")]
    pub ask: Box<PerpsBidAskDistributionHistorical>,
    #[serde(rename = "price")]
    pub price: Box<PerpsPriceDistributionHistorical>,
    #[serde(rename = "volume")]
    pub volume: String,
    #[serde(rename = "volume_notional_value_dollars")]
    pub volume_notional_value_dollars: String,
    #[serde(rename = "open_interest")]
    pub open_interest: String,
    #[serde(rename = "open_interest_notional_value_dollars")]
    pub open_interest_notional_value_dollars: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsMarginMarketCandlestick {
    fn default() -> Self {
        Self {
            end_period_ts: 0.into(),
            bid: Default::default(),
            ask: Default::default(),
            price: Default::default(),
            volume: Default::default(),
            volume_notional_value_dollars: Default::default(),
            open_interest: Default::default(),
            open_interest_notional_value_dollars: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginMarketOrderbook {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "orderbook")]
    pub orderbook: Box<PerpsMarginOrderbookCount>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginMarketResponse {
    #[serde(rename = "market")]
    pub market: Box<PerpsMarginMarket>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginMarketSchedule {
    #[serde(rename = "is_open")]
    pub is_open: bool,
    #[serde(rename = "next_close_ts", skip_serializing_if = "Option::is_none")]
    pub next_close_ts: Option<serde_json::Number>,
    #[serde(rename = "next_open_ts", skip_serializing_if = "Option::is_none")]
    pub next_open_ts: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginMarketScheduleDetails {
    #[serde(rename = "kalshi_trading_holidays")]
    pub kalshi_trading_holidays: Vec<PerpsMarginHolidayWindow>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginOrder {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "last_update_reason")]
    pub last_update_reason: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "fill_count")]
    pub fill_count: String,
    #[serde(rename = "remaining_count")]
    pub remaining_count: String,
    #[serde(rename = "expiration_time", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(rename = "created_time", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "last_update_time", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(rename = "self_trade_prevention_type", default, skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_type: Option<String>,
    #[serde(rename = "cancel_order_on_pause", default, skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(rename = "order_group_id", default, skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(rename = "order_source", default, skip_serializing_if = "Option::is_none")]
    pub order_source: Option<String>,
    #[serde(rename = "order_reason", default, skip_serializing_if = "Option::is_none")]
    pub order_reason: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginOrderbookCount {
    #[serde(rename = "bids")]
    pub bids: Vec<PerpsPriceLevelDollarsCountFp>,
    #[serde(rename = "asks")]
    pub asks: Vec<PerpsPriceLevelDollarsCountFp>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginOrderbookResponse {
    #[serde(rename = "orderbook")]
    pub orderbook: Box<PerpsMarginOrderbookCount>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsMarginPosition {
    #[serde(rename = "subaccount")]
    pub subaccount: serde_json::Number,
    #[serde(rename = "subaccount_type")]
    pub subaccount_type: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "position")]
    pub position: String,
    #[serde(rename = "entry_price")]
    pub entry_price: String,
    #[serde(rename = "unrealized_pnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "margin_used", default, skip_serializing_if = "Option::is_none")]
    pub margin_used: Option<String>,
    #[serde(rename = "fees")]
    pub fees: String,
    #[serde(rename = "roe", default, skip_serializing_if = "Option::is_none")]
    pub roe: Option<serde_json::Number>,
    #[serde(rename = "is_portfolio")]
    pub is_portfolio: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsMarginPosition {
    fn default() -> Self {
        Self {
            subaccount: 0.into(),
            subaccount_type: Default::default(),
            market_ticker: Default::default(),
            position: Default::default(),
            entry_price: Default::default(),
            unrealized_pnl: Default::default(),
            margin_used: Default::default(),
            fees: Default::default(),
            roe: Default::default(),
            is_portfolio: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginPremiumIndexPoint {
    #[serde(rename = "second_ts")]
    pub second_ts: String,
    #[serde(rename = "premium_index")]
    pub premium_index: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsMarginRiskPosition {
    #[serde(rename = "subaccount")]
    pub subaccount: serde_json::Number,
    #[serde(rename = "subaccount_type")]
    pub subaccount_type: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "position")]
    pub position: String,
    #[serde(rename = "mark_price")]
    pub mark_price: String,
    #[serde(rename = "position_notional")]
    pub position_notional: String,
    #[serde(rename = "maintenance_margin_required", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_margin_required: Option<String>,
    #[serde(rename = "position_leverage", default, skip_serializing_if = "Option::is_none")]
    pub position_leverage: Option<serde_json::Number>,
    #[serde(rename = "estimated_liquidation_price", default, skip_serializing_if = "Option::is_none")]
    pub estimated_liquidation_price: Option<String>,
    #[serde(rename = "is_portfolio")]
    pub is_portfolio: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsMarginRiskPosition {
    fn default() -> Self {
        Self {
            subaccount: 0.into(),
            subaccount_type: Default::default(),
            market_ticker: Default::default(),
            position: Default::default(),
            mark_price: Default::default(),
            position_notional: Default::default(),
            maintenance_margin_required: Default::default(),
            position_leverage: Default::default(),
            estimated_liquidation_price: Default::default(),
            is_portfolio: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsMarginSubaccountBalance {
    #[serde(rename = "subaccount")]
    pub subaccount: serde_json::Number,
    #[serde(rename = "subaccount_type")]
    pub subaccount_type: String,
    #[serde(rename = "position_value")]
    pub position_value: String,
    #[serde(rename = "account_equity")]
    pub account_equity: String,
    #[serde(rename = "maintenance_margin")]
    pub maintenance_margin: String,
    #[serde(rename = "initial_margin")]
    pub initial_margin: String,
    #[serde(rename = "resting_orders_margin")]
    pub resting_orders_margin: String,
    #[serde(rename = "available_balance")]
    pub available_balance: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsMarginSubaccountBalance {
    fn default() -> Self {
        Self {
            subaccount: 0.into(),
            subaccount_type: Default::default(),
            position_value: Default::default(),
            account_equity: Default::default(),
            maintenance_margin: Default::default(),
            initial_margin: Default::default(),
            resting_orders_margin: Default::default(),
            available_balance: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsMarginTrade {
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "count")]
    pub count: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "taker_side")]
    pub taker_side: String,
    #[serde(rename = "is_block_trade")]
    pub is_block_trade: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsNotionalRiskLimitResponse {
    #[serde(rename = "default_notional_value_risk_limit")]
    pub default_notional_value_risk_limit: String,
    #[serde(rename = "notional_value_risk_limits_by_market_ticker")]
    pub notional_value_risk_limits_by_market_ticker: std::collections::BTreeMap<String,String>,
    #[serde(rename = "total_current_usage")]
    pub total_current_usage: String,
    #[serde(rename = "current_usage_by_market_ticker")]
    pub current_usage_by_market_ticker: std::collections::BTreeMap<String,String>,
    #[serde(rename = "member_notional_value_risk_limit", default, skip_serializing_if = "Option::is_none")]
    pub member_notional_value_risk_limit: Option<String>,
    #[serde(rename = "effective_account_notional_value_risk_limit", default, skip_serializing_if = "Option::is_none")]
    pub effective_account_notional_value_risk_limit: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsOrderGroup {
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
pub struct PerpsPriceDistributionHistorical {
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
pub struct PerpsSetCrossExitTriggerRequest {
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    #[serde(rename = "anchor_order_id", default, skip_serializing_if = "Option::is_none")]
    pub anchor_order_id: Option<String>,
    #[serde(rename = "client_trigger_id", default, skip_serializing_if = "Option::is_none")]
    pub client_trigger_id: Option<String>,
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "stop_loss_price", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "take_profit_price", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(rename = "trail_amount", default, skip_serializing_if = "Option::is_none")]
    pub trail_amount: Option<String>,
    #[serde(rename = "trail_bps", default, skip_serializing_if = "Option::is_none")]
    pub trail_bps: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsSetIsolatedExitTriggerRequest {
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "stop_loss_price", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "take_profit_price", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(rename = "trail_amount", default, skip_serializing_if = "Option::is_none")]
    pub trail_amount: Option<String>,
    #[serde(rename = "trail_bps", default, skip_serializing_if = "Option::is_none")]
    pub trail_bps: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PerpsTickerPrice {
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "ts_ms")]
    pub ts_ms: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for PerpsTickerPrice {
    fn default() -> Self {
        Self {
            price: Default::default(),
            ts_ms: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsUpdateExitTriggerRequest {
    #[serde(rename = "stop_loss_price", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "take_profit_price", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsUpdateFCMNotionalRiskLimitRequest {
    #[serde(rename = "notional_value_risk_limit")]
    pub notional_value_risk_limit: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsUpdateFCMSubtraderRiskControlsRequest {
    #[serde(rename = "subtrader_id")]
    pub subtrader_id: String,
    #[serde(rename = "market_ticker", default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "asset_class", default, skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<String>,
    #[serde(rename = "im_cap")]
    pub im_cap: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PerpsUpdateOrderGroupLimitRequest {
    #[serde(rename = "contracts_limit", default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit: Option<serde_json::Number>,
    #[serde(rename = "contracts_limit_fp", default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResetMarginOrderGroupParams {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsEmptyResponse>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResetMarginOrderGroupResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetCrossMarginExitTriggerParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "body")]
    pub body: Box<PerpsSetCrossExitTriggerRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetCrossMarginExitTriggerResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "status_reason", default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "triggered_leg", default, skip_serializing_if = "Option::is_none")]
    pub triggered_leg: Option<String>,
    #[serde(rename = "triggered_order_id", default, skip_serializing_if = "Option::is_none")]
    pub triggered_order_id: Option<String>,
    #[serde(rename = "anchor_order_id", default, skip_serializing_if = "Option::is_none")]
    pub anchor_order_id: Option<String>,
    #[serde(rename = "client_trigger_id", default, skip_serializing_if = "Option::is_none")]
    pub client_trigger_id: Option<String>,
    #[serde(rename = "stop_loss_price", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "take_profit_price", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(rename = "trail_amount", default, skip_serializing_if = "Option::is_none")]
    pub trail_amount: Option<String>,
    #[serde(rename = "trail_bps", default, skip_serializing_if = "Option::is_none")]
    pub trail_bps: Option<serde_json::Number>,
    #[serde(rename = "watermark_price", default, skip_serializing_if = "Option::is_none")]
    pub watermark_price: Option<String>,
    #[serde(rename = "effective_stop_price", default, skip_serializing_if = "Option::is_none")]
    pub effective_stop_price: Option<String>,
    #[serde(rename = "count")]
    pub count: String,
    #[serde(rename = "filled_count")]
    pub filled_count: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "updated_time")]
    pub updated_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetIsolatedMarginExitTriggerParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "body")]
    pub body: Box<PerpsSetIsolatedExitTriggerRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetIsolatedMarginExitTriggerResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "status_reason", default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "triggered_leg", default, skip_serializing_if = "Option::is_none")]
    pub triggered_leg: Option<String>,
    #[serde(rename = "triggered_order_id", default, skip_serializing_if = "Option::is_none")]
    pub triggered_order_id: Option<String>,
    #[serde(rename = "anchor_order_id", default, skip_serializing_if = "Option::is_none")]
    pub anchor_order_id: Option<String>,
    #[serde(rename = "client_trigger_id", default, skip_serializing_if = "Option::is_none")]
    pub client_trigger_id: Option<String>,
    #[serde(rename = "stop_loss_price", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "take_profit_price", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(rename = "trail_amount", default, skip_serializing_if = "Option::is_none")]
    pub trail_amount: Option<String>,
    #[serde(rename = "trail_bps", default, skip_serializing_if = "Option::is_none")]
    pub trail_bps: Option<serde_json::Number>,
    #[serde(rename = "watermark_price", default, skip_serializing_if = "Option::is_none")]
    pub watermark_price: Option<String>,
    #[serde(rename = "effective_stop_price", default, skip_serializing_if = "Option::is_none")]
    pub effective_stop_price: Option<String>,
    #[serde(rename = "count")]
    pub count: String,
    #[serde(rename = "filled_count")]
    pub filled_count: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "updated_time")]
    pub updated_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TriggerMarginOrderGroupParams {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<PerpsEmptyResponse>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TriggerMarginOrderGroupResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateCrossMarginExitTriggerParams {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "trigger_id")]
    pub trigger_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "body")]
    pub body: Box<PerpsUpdateExitTriggerRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateCrossMarginExitTriggerResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "status_reason", default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "triggered_leg", default, skip_serializing_if = "Option::is_none")]
    pub triggered_leg: Option<String>,
    #[serde(rename = "triggered_order_id", default, skip_serializing_if = "Option::is_none")]
    pub triggered_order_id: Option<String>,
    #[serde(rename = "anchor_order_id", default, skip_serializing_if = "Option::is_none")]
    pub anchor_order_id: Option<String>,
    #[serde(rename = "client_trigger_id", default, skip_serializing_if = "Option::is_none")]
    pub client_trigger_id: Option<String>,
    #[serde(rename = "stop_loss_price", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "take_profit_price", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(rename = "trail_amount", default, skip_serializing_if = "Option::is_none")]
    pub trail_amount: Option<String>,
    #[serde(rename = "trail_bps", default, skip_serializing_if = "Option::is_none")]
    pub trail_bps: Option<serde_json::Number>,
    #[serde(rename = "watermark_price", default, skip_serializing_if = "Option::is_none")]
    pub watermark_price: Option<String>,
    #[serde(rename = "effective_stop_price", default, skip_serializing_if = "Option::is_none")]
    pub effective_stop_price: Option<String>,
    #[serde(rename = "count")]
    pub count: String,
    #[serde(rename = "filled_count")]
    pub filled_count: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "updated_time")]
    pub updated_time: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateFcmNotionalRiskLimitParams {
    #[serde(rename = "body")]
    pub body: Box<PerpsUpdateFCMNotionalRiskLimitRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateFcmNotionalRiskLimitResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateFcmSubtraderRiskControlsParams {
    #[serde(rename = "body")]
    pub body: Box<PerpsUpdateFCMSubtraderRiskControlsRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateFcmSubtraderRiskControlsResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateMarginOrderGroupLimitParams {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount", default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<serde_json::Number>,
    #[serde(rename = "body")]
    pub body: Box<PerpsUpdateOrderGroupLimitRequest>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpdateMarginOrderGroupLimitResponse {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedKalshiPerpsClient(KalshiPerpsClient);
impl KalshiPerpsClient {
    pub fn typed(self) -> TypedKalshiPerpsClient { TypedKalshiPerpsClient(self) }
}
impl TypedKalshiPerpsClient {
    pub fn clearing_create_margin_fcm_api_key(&self, request: &ClearingCreateMarginFcmApiKeyParams) -> Result<Box<ClearingCreateMarginFcmApiKeyResponse>> {
        serde_json::from_value(self.0.clearing_create_margin_fcm_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_create_margin_subtrader_group(&self, request: &ClearingCreateMarginSubtraderGroupParams) -> Result<Box<ClearingCreateMarginSubtraderGroupResponse>> {
        serde_json::from_value(self.0.clearing_create_margin_subtrader_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_delete_margin_fcm_api_key(&self, request: &ClearingDeleteMarginFcmApiKeyParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.clearing_delete_margin_fcm_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_delete_margin_subtrader_group(&self, request: &ClearingDeleteMarginSubtraderGroupParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.clearing_delete_margin_subtrader_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_estimate_portfolio_maintenance_margin(&self, request: &ClearingEstimatePortfolioMaintenanceMarginParams) -> Result<Box<ClearingEstimatePortfolioMaintenanceMarginResponse>> {
        serde_json::from_value(self.0.clearing_estimate_portfolio_maintenance_margin(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_generate_margin_fcm_api_key(&self, request: &ClearingGenerateMarginFcmApiKeyParams) -> Result<Box<ClearingGenerateMarginFcmApiKeyResponse>> {
        serde_json::from_value(self.0.clearing_generate_margin_fcm_api_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_active_margin_obligations(&self, request: &ClearingGetActiveMarginObligationsParams) -> Result<Box<ClearingGetActiveMarginObligationsResponse>> {
        serde_json::from_value(self.0.clearing_get_active_margin_obligations(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_funding_estimate_by_asset_class(&self, request: &ClearingGetFundingEstimateByAssetClassParams) -> Result<Box<ClearingGetFundingEstimateByAssetClassResponse>> {
        serde_json::from_value(self.0.clearing_get_funding_estimate_by_asset_class(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_guaranty_fund_balance(&self, request: &ClearingGetGuarantyFundBalanceParams) -> Result<Box<ClearingGetGuarantyFundBalanceResponse>> {
        serde_json::from_value(self.0.clearing_get_guaranty_fund_balance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_large_trader_positions(&self, request: &ClearingGetLargeTraderPositionsParams) -> Result<Box<ClearingGetLargeTraderPositionsResponse>> {
        serde_json::from_value(self.0.clearing_get_large_trader_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_maintenance_margin_metadata(&self, request: &ClearingGetMaintenanceMarginMetadataParams) -> Result<Box<ClearingGetMaintenanceMarginMetadataResponse>> {
        serde_json::from_value(self.0.clearing_get_maintenance_margin_metadata(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_margin_funding_schedule(&self, request: &ClearingGetMarginFundingScheduleParams) -> Result<Box<ClearingGetMarginFundingScheduleResponse>> {
        serde_json::from_value(self.0.clearing_get_margin_funding_schedule(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_margin_reports(&self, request: &ClearingGetMarginReportsParams) -> Result<Box<ClearingGetMarginReportsResponse>> {
        serde_json::from_value(self.0.clearing_get_margin_reports(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_margin_subtrader_groups(&self, request: &ClearingGetMarginSubtraderGroupsParams) -> Result<Box<ClearingGetMarginSubtraderGroupsResponse>> {
        serde_json::from_value(self.0.clearing_get_margin_subtrader_groups(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_member_funding_payments(&self, request: &ClearingGetMemberFundingPaymentsParams) -> Result<Box<ClearingGetMemberFundingPaymentsResponse>> {
        serde_json::from_value(self.0.clearing_get_member_funding_payments(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_obligation_corporate_action_payments(&self, request: &ClearingGetObligationCorporateActionPaymentsParams) -> Result<Box<ClearingGetObligationCorporateActionPaymentsResponse>> {
        serde_json::from_value(self.0.clearing_get_obligation_corporate_action_payments(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_obligation_funding_payments(&self, request: &ClearingGetObligationFundingPaymentsParams) -> Result<Box<ClearingGetObligationFundingPaymentsResponse>> {
        serde_json::from_value(self.0.clearing_get_obligation_funding_payments(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_obligation_history(&self, request: &ClearingGetObligationHistoryParams) -> Result<Box<ClearingGetObligationHistoryResponse>> {
        serde_json::from_value(self.0.clearing_get_obligation_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_obligation_maintenance_margin_details(&self, request: &ClearingGetObligationMaintenanceMarginDetailsParams) -> Result<Box<ClearingGetObligationMaintenanceMarginDetailsResponse>> {
        serde_json::from_value(self.0.clearing_get_obligation_maintenance_margin_details(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_obligation_settlement_details(&self, request: &ClearingGetObligationSettlementDetailsParams) -> Result<Box<ClearingGetObligationSettlementDetailsResponse>> {
        serde_json::from_value(self.0.clearing_get_obligation_settlement_details(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_settlement_balance(&self, request: &ClearingGetSettlementBalanceParams) -> Result<Box<ClearingGetSettlementBalanceResponse>> {
        serde_json::from_value(self.0.clearing_get_settlement_balance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_settlement_balance_history(&self, request: &ClearingGetSettlementBalanceHistoryParams) -> Result<Box<ClearingGetSettlementBalanceHistoryResponse>> {
        serde_json::from_value(self.0.clearing_get_settlement_balance_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_settlement_balance_withdrawal(&self, request: &ClearingGetSettlementBalanceWithdrawalParams) -> Result<Box<ClearingGetSettlementBalanceWithdrawalResponse>> {
        serde_json::from_value(self.0.clearing_get_settlement_balance_withdrawal(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_settlement_estimate_by_asset_class(&self, request: &ClearingGetSettlementEstimateByAssetClassParams) -> Result<Box<ClearingGetSettlementEstimateByAssetClassResponse>> {
        serde_json::from_value(self.0.clearing_get_settlement_estimate_by_asset_class(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_get_settlement_prices(&self, request: &ClearingGetSettlementPricesParams) -> Result<Box<ClearingGetSettlementPricesResponse>> {
        serde_json::from_value(self.0.clearing_get_settlement_prices(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_list_margin_fcm_api_keys(&self, request: &ClearingListMarginFcmApiKeysParams) -> Result<Box<ClearingListMarginFcmApiKeysResponse>> {
        serde_json::from_value(self.0.clearing_list_margin_fcm_api_keys(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_update_margin_subtrader_group(&self, request: &ClearingUpdateMarginSubtraderGroupParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.clearing_update_margin_subtrader_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn clearing_withdraw_settlement_balance(&self, request: &ClearingWithdrawSettlementBalanceParams) -> Result<Box<ClearingWithdrawSettlementBalanceResponse>> {
        serde_json::from_value(self.0.clearing_withdraw_settlement_balance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn amend_margin_order(&self, request: &AmendMarginOrderParams) -> Result<Box<PerpsAmendMarginOrderResponse>> {
        serde_json::from_value(self.0.amend_margin_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn apply_margin_subaccount_transfer(&self, request: &ApplyMarginSubaccountTransferParams) -> Result<Box<PerpsApplySubaccountTransferResponse>> {
        serde_json::from_value(self.0.apply_margin_subaccount_transfer(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn cancel_all_margin_orders(&self, request: &CancelAllMarginOrdersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_all_margin_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn cancel_margin_order(&self, request: &CancelMarginOrderParams) -> Result<Box<PerpsCancelMarginOrderResponse>> {
        serde_json::from_value(self.0.cancel_margin_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn create_margin_fcm_subtrader(&self, request: &CreateMarginFcmSubtraderParams) -> Result<Box<PerpsCreateMarginFCMSubtraderResponse>> {
        serde_json::from_value(self.0.create_margin_fcm_subtrader(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn create_margin_order(&self, request: &CreateMarginOrderParams) -> Result<Box<PerpsCreateMarginOrderResponse>> {
        serde_json::from_value(self.0.create_margin_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn create_margin_order_group(&self, request: &CreateMarginOrderGroupParams) -> Result<Box<PerpsCreateOrderGroupResponse>> {
        serde_json::from_value(self.0.create_margin_order_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn create_margin_subaccount(&self, request: &CreateMarginSubaccountParams) -> Result<Box<PerpsCreateSubaccountResponse>> {
        serde_json::from_value(self.0.create_margin_subaccount(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn decrease_margin_order(&self, request: &DecreaseMarginOrderParams) -> Result<Box<PerpsDecreaseMarginOrderResponse>> {
        serde_json::from_value(self.0.decrease_margin_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn delete_cross_margin_exit_trigger(&self, request: &DeleteCrossMarginExitTriggerParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_cross_margin_exit_trigger(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn delete_cross_margin_exit_trigger_by_id(&self, request: &DeleteCrossMarginExitTriggerByIdParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_cross_margin_exit_trigger_by_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn delete_fcm_notional_risk_limit(&self, request: &DeleteFcmNotionalRiskLimitParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.delete_fcm_notional_risk_limit(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn delete_fcm_subtrader_risk_controls(&self, request: &DeleteFcmSubtraderRiskControlsParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.delete_fcm_subtrader_risk_controls(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn delete_isolated_margin_exit_trigger(&self, request: &DeleteIsolatedMarginExitTriggerParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_isolated_margin_exit_trigger(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn delete_margin_order_group(&self, request: &DeleteMarginOrderGroupParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.delete_margin_order_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_cross_margin_exit_triggers(&self, request: &GetCrossMarginExitTriggersParams) -> Result<Box<PerpsGetExitTriggersResponse>> {
        serde_json::from_value(self.0.get_cross_margin_exit_triggers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_fcm_subtrader_risk_controls(&self, request: &GetFcmSubtraderRiskControlsParams) -> Result<Box<PerpsGetFCMSubtraderRiskControlsResponse>> {
        serde_json::from_value(self.0.get_fcm_subtrader_risk_controls(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_isolated_margin_exit_triggers(&self, request: &GetIsolatedMarginExitTriggersParams) -> Result<Box<PerpsGetExitTriggersResponse>> {
        serde_json::from_value(self.0.get_isolated_margin_exit_triggers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_balance(&self, request: &GetMarginBalanceParams) -> Result<Box<PerpsGetMarginBalanceResponse>> {
        serde_json::from_value(self.0.get_margin_balance(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_corporate_actions(&self, request: &GetMarginCorporateActionsParams) -> Result<Box<PerpsGetMarginCorporateActionsResponse>> {
        serde_json::from_value(self.0.get_margin_corporate_actions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_enabled(&self, request: &GetMarginEnabledParams) -> Result<Box<PerpsMarginEnabledResponse>> {
        serde_json::from_value(self.0.get_margin_enabled(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_exchange_status(&self, request: &GetMarginExchangeStatusParams) -> Result<Box<PerpsExchangeStatus>> {
        serde_json::from_value(self.0.get_margin_exchange_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_fee_tier_rates(&self, request: &GetMarginFeeTierRatesParams) -> Result<Box<PerpsGetMarginFeeTierRatesResponse>> {
        serde_json::from_value(self.0.get_margin_fee_tier_rates(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_fee_tiers(&self, request: &GetMarginFeeTiersParams) -> Result<Box<PerpsGetMarginFeeTiersResponse>> {
        serde_json::from_value(self.0.get_margin_fee_tiers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_fills(&self, request: &GetMarginFillsParams) -> Result<Box<PerpsGetMarginFillsResponse>> {
        serde_json::from_value(self.0.get_margin_fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_funding_history(&self, request: &GetMarginFundingHistoryParams) -> Result<Box<PerpsGetMarginFundingHistoryResponse>> {
        serde_json::from_value(self.0.get_margin_funding_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_funding_rate_estimate(&self, request: &GetMarginFundingRateEstimateParams) -> Result<Box<PerpsGetMarginFundingRateEstimateResponse>> {
        serde_json::from_value(self.0.get_margin_funding_rate_estimate(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_historical_funding_rates(&self, request: &GetMarginHistoricalFundingRatesParams) -> Result<Box<PerpsGetMarginHistoricalFundingRatesResponse>> {
        serde_json::from_value(self.0.get_margin_historical_funding_rates(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_market(&self, request: &GetMarginMarketParams) -> Result<Box<PerpsMarginMarketResponse>> {
        serde_json::from_value(self.0.get_margin_market(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_market_candlesticks(&self, request: &GetMarginMarketCandlesticksParams) -> Result<Box<PerpsGetMarginMarketCandlesticksResponse>> {
        serde_json::from_value(self.0.get_margin_market_candlesticks(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_market_orderbook(&self, request: &GetMarginMarketOrderbookParams) -> Result<Box<PerpsMarginOrderbookResponse>> {
        serde_json::from_value(self.0.get_margin_market_orderbook(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_market_orderbooks(&self, request: &GetMarginMarketOrderbooksParams) -> Result<Box<PerpsGetMarginMarketOrderbooksResponse>> {
        serde_json::from_value(self.0.get_margin_market_orderbooks(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_market_schedules(&self, request: &GetMarginMarketSchedulesParams) -> Result<Box<PerpsGetMarginMarketSchedulesResponse>> {
        serde_json::from_value(self.0.get_margin_market_schedules(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_markets(&self, request: &GetMarginMarketsParams) -> Result<Box<PerpsGetMarginMarketsResponse>> {
        serde_json::from_value(self.0.get_margin_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_notional_risk_limit(&self, request: &GetMarginNotionalRiskLimitParams) -> Result<Box<PerpsNotionalRiskLimitResponse>> {
        serde_json::from_value(self.0.get_margin_notional_risk_limit(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_order(&self, request: &GetMarginOrderParams) -> Result<Box<PerpsGetMarginOrderResponse>> {
        serde_json::from_value(self.0.get_margin_order(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_order_group(&self, request: &GetMarginOrderGroupParams) -> Result<Box<PerpsGetOrderGroupResponse>> {
        serde_json::from_value(self.0.get_margin_order_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_order_groups(&self, request: &GetMarginOrderGroupsParams) -> Result<Box<PerpsGetOrderGroupsResponse>> {
        serde_json::from_value(self.0.get_margin_order_groups(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_orders(&self, request: &GetMarginOrdersParams) -> Result<Box<PerpsGetMarginOrdersResponse>> {
        serde_json::from_value(self.0.get_margin_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_positions(&self, request: &GetMarginPositionsParams) -> Result<Box<PerpsGetMarginPositionsResponse>> {
        serde_json::from_value(self.0.get_margin_positions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_premium_index(&self, request: &GetMarginPremiumIndexParams) -> Result<Box<PerpsGetMarginPremiumIndexResponse>> {
        serde_json::from_value(self.0.get_margin_premium_index(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_risk(&self, request: &GetMarginRiskParams) -> Result<Box<PerpsGetMarginRiskResponse>> {
        serde_json::from_value(self.0.get_margin_risk(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_risk_parameters(&self, request: &GetMarginRiskParametersParams) -> Result<Box<PerpsGetMarginRiskParametersResponse>> {
        serde_json::from_value(self.0.get_margin_risk_parameters(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_margin_trades(&self, request: &GetMarginTradesParams) -> Result<Box<PerpsGetMarginTradesResponse>> {
        serde_json::from_value(self.0.get_margin_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn get_perps_account_api_limits(&self, request: &GetPerpsAccountApiLimitsParams) -> Result<Box<PerpsGetAccountApiLimitsResponse>> {
        serde_json::from_value(self.0.get_perps_account_api_limits(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn intra_exchange_instance_transfer(&self, request: &IntraExchangeInstanceTransferParams) -> Result<Box<PerpsIntraExchangeInstanceTransferResponse>> {
        serde_json::from_value(self.0.intra_exchange_instance_transfer(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn reset_margin_order_group(&self, request: &ResetMarginOrderGroupParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.reset_margin_order_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn set_cross_margin_exit_trigger(&self, request: &SetCrossMarginExitTriggerParams) -> Result<Box<PerpsExitTrigger>> {
        serde_json::from_value(self.0.set_cross_margin_exit_trigger(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn set_isolated_margin_exit_trigger(&self, request: &SetIsolatedMarginExitTriggerParams) -> Result<Box<PerpsExitTrigger>> {
        serde_json::from_value(self.0.set_isolated_margin_exit_trigger(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn trigger_margin_order_group(&self, request: &TriggerMarginOrderGroupParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.trigger_margin_order_group(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn update_cross_margin_exit_trigger(&self, request: &UpdateCrossMarginExitTriggerParams) -> Result<Box<PerpsExitTrigger>> {
        serde_json::from_value(self.0.update_cross_margin_exit_trigger(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn update_fcm_notional_risk_limit(&self, request: &UpdateFcmNotionalRiskLimitParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.update_fcm_notional_risk_limit(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn update_fcm_subtrader_risk_controls(&self, request: &UpdateFcmSubtraderRiskControlsParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.update_fcm_subtrader_risk_controls(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub fn update_margin_order_group_limit(&self, request: &UpdateMarginOrderGroupLimitParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.update_margin_order_group_limit(&json!(request))?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedKalshiPerpsClient(crate::AsyncClient<KalshiPerpsClient>);
#[cfg(feature = "async")]
impl TypedKalshiPerpsClient {
    pub fn into_async(self) -> AsyncTypedKalshiPerpsClient { AsyncTypedKalshiPerpsClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedKalshiPerpsClient {
    pub async fn amend_margin_order(&self, request: &AmendMarginOrderParams) -> Result<Box<PerpsAmendMarginOrderResponse>> {
        serde_json::from_value(self.0.amend_margin_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn apply_margin_subaccount_transfer(&self, request: &ApplyMarginSubaccountTransferParams) -> Result<Box<PerpsApplySubaccountTransferResponse>> {
        serde_json::from_value(self.0.apply_margin_subaccount_transfer(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn cancel_all_margin_orders(&self, request: &CancelAllMarginOrdersParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.cancel_all_margin_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn cancel_margin_order(&self, request: &CancelMarginOrderParams) -> Result<Box<PerpsCancelMarginOrderResponse>> {
        serde_json::from_value(self.0.cancel_margin_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_create_margin_fcm_api_key(&self, request: &ClearingCreateMarginFcmApiKeyParams) -> Result<Box<ClearingCreateMarginFcmApiKeyResponse>> {
        serde_json::from_value(self.0.clearing_create_margin_fcm_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_create_margin_subtrader_group(&self, request: &ClearingCreateMarginSubtraderGroupParams) -> Result<Box<ClearingCreateMarginSubtraderGroupResponse>> {
        serde_json::from_value(self.0.clearing_create_margin_subtrader_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_delete_margin_fcm_api_key(&self, request: &ClearingDeleteMarginFcmApiKeyParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.clearing_delete_margin_fcm_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_delete_margin_subtrader_group(&self, request: &ClearingDeleteMarginSubtraderGroupParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.clearing_delete_margin_subtrader_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_estimate_portfolio_maintenance_margin(&self, request: &ClearingEstimatePortfolioMaintenanceMarginParams) -> Result<Box<ClearingEstimatePortfolioMaintenanceMarginResponse>> {
        serde_json::from_value(self.0.clearing_estimate_portfolio_maintenance_margin(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_generate_margin_fcm_api_key(&self, request: &ClearingGenerateMarginFcmApiKeyParams) -> Result<Box<ClearingGenerateMarginFcmApiKeyResponse>> {
        serde_json::from_value(self.0.clearing_generate_margin_fcm_api_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_active_margin_obligations(&self, request: &ClearingGetActiveMarginObligationsParams) -> Result<Box<ClearingGetActiveMarginObligationsResponse>> {
        serde_json::from_value(self.0.clearing_get_active_margin_obligations(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_funding_estimate_by_asset_class(&self, request: &ClearingGetFundingEstimateByAssetClassParams) -> Result<Box<ClearingGetFundingEstimateByAssetClassResponse>> {
        serde_json::from_value(self.0.clearing_get_funding_estimate_by_asset_class(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_guaranty_fund_balance(&self, request: &ClearingGetGuarantyFundBalanceParams) -> Result<Box<ClearingGetGuarantyFundBalanceResponse>> {
        serde_json::from_value(self.0.clearing_get_guaranty_fund_balance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_large_trader_positions(&self, request: &ClearingGetLargeTraderPositionsParams) -> Result<Box<ClearingGetLargeTraderPositionsResponse>> {
        serde_json::from_value(self.0.clearing_get_large_trader_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_maintenance_margin_metadata(&self, request: &ClearingGetMaintenanceMarginMetadataParams) -> Result<Box<ClearingGetMaintenanceMarginMetadataResponse>> {
        serde_json::from_value(self.0.clearing_get_maintenance_margin_metadata(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_margin_funding_schedule(&self, request: &ClearingGetMarginFundingScheduleParams) -> Result<Box<ClearingGetMarginFundingScheduleResponse>> {
        serde_json::from_value(self.0.clearing_get_margin_funding_schedule(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_margin_reports(&self, request: &ClearingGetMarginReportsParams) -> Result<Box<ClearingGetMarginReportsResponse>> {
        serde_json::from_value(self.0.clearing_get_margin_reports(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_margin_subtrader_groups(&self, request: &ClearingGetMarginSubtraderGroupsParams) -> Result<Box<ClearingGetMarginSubtraderGroupsResponse>> {
        serde_json::from_value(self.0.clearing_get_margin_subtrader_groups(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_member_funding_payments(&self, request: &ClearingGetMemberFundingPaymentsParams) -> Result<Box<ClearingGetMemberFundingPaymentsResponse>> {
        serde_json::from_value(self.0.clearing_get_member_funding_payments(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_obligation_corporate_action_payments(&self, request: &ClearingGetObligationCorporateActionPaymentsParams) -> Result<Box<ClearingGetObligationCorporateActionPaymentsResponse>> {
        serde_json::from_value(self.0.clearing_get_obligation_corporate_action_payments(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_obligation_funding_payments(&self, request: &ClearingGetObligationFundingPaymentsParams) -> Result<Box<ClearingGetObligationFundingPaymentsResponse>> {
        serde_json::from_value(self.0.clearing_get_obligation_funding_payments(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_obligation_history(&self, request: &ClearingGetObligationHistoryParams) -> Result<Box<ClearingGetObligationHistoryResponse>> {
        serde_json::from_value(self.0.clearing_get_obligation_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_obligation_maintenance_margin_details(&self, request: &ClearingGetObligationMaintenanceMarginDetailsParams) -> Result<Box<ClearingGetObligationMaintenanceMarginDetailsResponse>> {
        serde_json::from_value(self.0.clearing_get_obligation_maintenance_margin_details(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_obligation_settlement_details(&self, request: &ClearingGetObligationSettlementDetailsParams) -> Result<Box<ClearingGetObligationSettlementDetailsResponse>> {
        serde_json::from_value(self.0.clearing_get_obligation_settlement_details(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_settlement_balance(&self, request: &ClearingGetSettlementBalanceParams) -> Result<Box<ClearingGetSettlementBalanceResponse>> {
        serde_json::from_value(self.0.clearing_get_settlement_balance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_settlement_balance_history(&self, request: &ClearingGetSettlementBalanceHistoryParams) -> Result<Box<ClearingGetSettlementBalanceHistoryResponse>> {
        serde_json::from_value(self.0.clearing_get_settlement_balance_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_settlement_balance_withdrawal(&self, request: &ClearingGetSettlementBalanceWithdrawalParams) -> Result<Box<ClearingGetSettlementBalanceWithdrawalResponse>> {
        serde_json::from_value(self.0.clearing_get_settlement_balance_withdrawal(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_settlement_estimate_by_asset_class(&self, request: &ClearingGetSettlementEstimateByAssetClassParams) -> Result<Box<ClearingGetSettlementEstimateByAssetClassResponse>> {
        serde_json::from_value(self.0.clearing_get_settlement_estimate_by_asset_class(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_get_settlement_prices(&self, request: &ClearingGetSettlementPricesParams) -> Result<Box<ClearingGetSettlementPricesResponse>> {
        serde_json::from_value(self.0.clearing_get_settlement_prices(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_list_margin_fcm_api_keys(&self, request: &ClearingListMarginFcmApiKeysParams) -> Result<Box<ClearingListMarginFcmApiKeysResponse>> {
        serde_json::from_value(self.0.clearing_list_margin_fcm_api_keys(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_update_margin_subtrader_group(&self, request: &ClearingUpdateMarginSubtraderGroupParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.clearing_update_margin_subtrader_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn clearing_withdraw_settlement_balance(&self, request: &ClearingWithdrawSettlementBalanceParams) -> Result<Box<ClearingWithdrawSettlementBalanceResponse>> {
        serde_json::from_value(self.0.clearing_withdraw_settlement_balance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn create_margin_fcm_subtrader(&self, request: &CreateMarginFcmSubtraderParams) -> Result<Box<PerpsCreateMarginFCMSubtraderResponse>> {
        serde_json::from_value(self.0.create_margin_fcm_subtrader(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn create_margin_order(&self, request: &CreateMarginOrderParams) -> Result<Box<PerpsCreateMarginOrderResponse>> {
        serde_json::from_value(self.0.create_margin_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn create_margin_order_group(&self, request: &CreateMarginOrderGroupParams) -> Result<Box<PerpsCreateOrderGroupResponse>> {
        serde_json::from_value(self.0.create_margin_order_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn create_margin_subaccount(&self, request: &CreateMarginSubaccountParams) -> Result<Box<PerpsCreateSubaccountResponse>> {
        serde_json::from_value(self.0.create_margin_subaccount(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn decrease_margin_order(&self, request: &DecreaseMarginOrderParams) -> Result<Box<PerpsDecreaseMarginOrderResponse>> {
        serde_json::from_value(self.0.decrease_margin_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn delete_cross_margin_exit_trigger(&self, request: &DeleteCrossMarginExitTriggerParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_cross_margin_exit_trigger(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn delete_cross_margin_exit_trigger_by_id(&self, request: &DeleteCrossMarginExitTriggerByIdParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_cross_margin_exit_trigger_by_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn delete_fcm_notional_risk_limit(&self, request: &DeleteFcmNotionalRiskLimitParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.delete_fcm_notional_risk_limit(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn delete_fcm_subtrader_risk_controls(&self, request: &DeleteFcmSubtraderRiskControlsParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.delete_fcm_subtrader_risk_controls(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn delete_isolated_margin_exit_trigger(&self, request: &DeleteIsolatedMarginExitTriggerParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.delete_isolated_margin_exit_trigger(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn delete_margin_order_group(&self, request: &DeleteMarginOrderGroupParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.delete_margin_order_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_cross_margin_exit_triggers(&self, request: &GetCrossMarginExitTriggersParams) -> Result<Box<PerpsGetExitTriggersResponse>> {
        serde_json::from_value(self.0.get_cross_margin_exit_triggers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_fcm_subtrader_risk_controls(&self, request: &GetFcmSubtraderRiskControlsParams) -> Result<Box<PerpsGetFCMSubtraderRiskControlsResponse>> {
        serde_json::from_value(self.0.get_fcm_subtrader_risk_controls(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_isolated_margin_exit_triggers(&self, request: &GetIsolatedMarginExitTriggersParams) -> Result<Box<PerpsGetExitTriggersResponse>> {
        serde_json::from_value(self.0.get_isolated_margin_exit_triggers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_balance(&self, request: &GetMarginBalanceParams) -> Result<Box<PerpsGetMarginBalanceResponse>> {
        serde_json::from_value(self.0.get_margin_balance(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_corporate_actions(&self, request: &GetMarginCorporateActionsParams) -> Result<Box<PerpsGetMarginCorporateActionsResponse>> {
        serde_json::from_value(self.0.get_margin_corporate_actions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_enabled(&self, request: &GetMarginEnabledParams) -> Result<Box<PerpsMarginEnabledResponse>> {
        serde_json::from_value(self.0.get_margin_enabled(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_exchange_status(&self, request: &GetMarginExchangeStatusParams) -> Result<Box<PerpsExchangeStatus>> {
        serde_json::from_value(self.0.get_margin_exchange_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_fee_tier_rates(&self, request: &GetMarginFeeTierRatesParams) -> Result<Box<PerpsGetMarginFeeTierRatesResponse>> {
        serde_json::from_value(self.0.get_margin_fee_tier_rates(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_fee_tiers(&self, request: &GetMarginFeeTiersParams) -> Result<Box<PerpsGetMarginFeeTiersResponse>> {
        serde_json::from_value(self.0.get_margin_fee_tiers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_fills(&self, request: &GetMarginFillsParams) -> Result<Box<PerpsGetMarginFillsResponse>> {
        serde_json::from_value(self.0.get_margin_fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_funding_history(&self, request: &GetMarginFundingHistoryParams) -> Result<Box<PerpsGetMarginFundingHistoryResponse>> {
        serde_json::from_value(self.0.get_margin_funding_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_funding_rate_estimate(&self, request: &GetMarginFundingRateEstimateParams) -> Result<Box<PerpsGetMarginFundingRateEstimateResponse>> {
        serde_json::from_value(self.0.get_margin_funding_rate_estimate(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_historical_funding_rates(&self, request: &GetMarginHistoricalFundingRatesParams) -> Result<Box<PerpsGetMarginHistoricalFundingRatesResponse>> {
        serde_json::from_value(self.0.get_margin_historical_funding_rates(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_market(&self, request: &GetMarginMarketParams) -> Result<Box<PerpsMarginMarketResponse>> {
        serde_json::from_value(self.0.get_margin_market(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_market_candlesticks(&self, request: &GetMarginMarketCandlesticksParams) -> Result<Box<PerpsGetMarginMarketCandlesticksResponse>> {
        serde_json::from_value(self.0.get_margin_market_candlesticks(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_market_orderbook(&self, request: &GetMarginMarketOrderbookParams) -> Result<Box<PerpsMarginOrderbookResponse>> {
        serde_json::from_value(self.0.get_margin_market_orderbook(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_market_orderbooks(&self, request: &GetMarginMarketOrderbooksParams) -> Result<Box<PerpsGetMarginMarketOrderbooksResponse>> {
        serde_json::from_value(self.0.get_margin_market_orderbooks(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_market_schedules(&self, request: &GetMarginMarketSchedulesParams) -> Result<Box<PerpsGetMarginMarketSchedulesResponse>> {
        serde_json::from_value(self.0.get_margin_market_schedules(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_markets(&self, request: &GetMarginMarketsParams) -> Result<Box<PerpsGetMarginMarketsResponse>> {
        serde_json::from_value(self.0.get_margin_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_notional_risk_limit(&self, request: &GetMarginNotionalRiskLimitParams) -> Result<Box<PerpsNotionalRiskLimitResponse>> {
        serde_json::from_value(self.0.get_margin_notional_risk_limit(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_order(&self, request: &GetMarginOrderParams) -> Result<Box<PerpsGetMarginOrderResponse>> {
        serde_json::from_value(self.0.get_margin_order(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_order_group(&self, request: &GetMarginOrderGroupParams) -> Result<Box<PerpsGetOrderGroupResponse>> {
        serde_json::from_value(self.0.get_margin_order_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_order_groups(&self, request: &GetMarginOrderGroupsParams) -> Result<Box<PerpsGetOrderGroupsResponse>> {
        serde_json::from_value(self.0.get_margin_order_groups(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_orders(&self, request: &GetMarginOrdersParams) -> Result<Box<PerpsGetMarginOrdersResponse>> {
        serde_json::from_value(self.0.get_margin_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_positions(&self, request: &GetMarginPositionsParams) -> Result<Box<PerpsGetMarginPositionsResponse>> {
        serde_json::from_value(self.0.get_margin_positions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_premium_index(&self, request: &GetMarginPremiumIndexParams) -> Result<Box<PerpsGetMarginPremiumIndexResponse>> {
        serde_json::from_value(self.0.get_margin_premium_index(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_risk(&self, request: &GetMarginRiskParams) -> Result<Box<PerpsGetMarginRiskResponse>> {
        serde_json::from_value(self.0.get_margin_risk(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_risk_parameters(&self, request: &GetMarginRiskParametersParams) -> Result<Box<PerpsGetMarginRiskParametersResponse>> {
        serde_json::from_value(self.0.get_margin_risk_parameters(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_margin_trades(&self, request: &GetMarginTradesParams) -> Result<Box<PerpsGetMarginTradesResponse>> {
        serde_json::from_value(self.0.get_margin_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn get_perps_account_api_limits(&self, request: &GetPerpsAccountApiLimitsParams) -> Result<Box<PerpsGetAccountApiLimitsResponse>> {
        serde_json::from_value(self.0.get_perps_account_api_limits(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn intra_exchange_instance_transfer(&self, request: &IntraExchangeInstanceTransferParams) -> Result<Box<PerpsIntraExchangeInstanceTransferResponse>> {
        serde_json::from_value(self.0.intra_exchange_instance_transfer(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn reset_margin_order_group(&self, request: &ResetMarginOrderGroupParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.reset_margin_order_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn set_cross_margin_exit_trigger(&self, request: &SetCrossMarginExitTriggerParams) -> Result<Box<PerpsExitTrigger>> {
        serde_json::from_value(self.0.set_cross_margin_exit_trigger(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn set_isolated_margin_exit_trigger(&self, request: &SetIsolatedMarginExitTriggerParams) -> Result<Box<PerpsExitTrigger>> {
        serde_json::from_value(self.0.set_isolated_margin_exit_trigger(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn trigger_margin_order_group(&self, request: &TriggerMarginOrderGroupParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.trigger_margin_order_group(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn update_cross_margin_exit_trigger(&self, request: &UpdateCrossMarginExitTriggerParams) -> Result<Box<PerpsExitTrigger>> {
        serde_json::from_value(self.0.update_cross_margin_exit_trigger(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn update_fcm_notional_risk_limit(&self, request: &UpdateFcmNotionalRiskLimitParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.update_fcm_notional_risk_limit(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn update_fcm_subtrader_risk_controls(&self, request: &UpdateFcmSubtraderRiskControlsParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.update_fcm_subtrader_risk_controls(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
    pub async fn update_margin_order_group_limit(&self, request: &UpdateMarginOrderGroupLimitParams) -> Result<Box<PerpsEmptyResponse>> {
        serde_json::from_value(self.0.update_margin_order_group_limit(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid KalshiPerps response model"))
    }
}
