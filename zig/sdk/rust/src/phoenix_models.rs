// Generated from schema/phoenix.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{PhoenixClient, Error, Result};

pub use crate::model_number::NativeNumber;
pub type AckNotificationItem = serde_json::Value;
pub type ActivateReferralTxStatus = String;
pub type CommodityMarketStateView = String;
pub type ExchangeWsValidationRule = String;
pub type JsSafeU64 = NativeNumber;
pub type LiquidityRole = String;
pub type MarketCalendarStateView = String;
pub type MarketStatus = String;
pub type NotificationItem = serde_json::Value;
pub type OrderStatus = String;
pub type PlaceStopLossOrderRequest = serde_json::Value;
pub type RiskState = String;
pub type RiskTier = String;
pub type Side = String;
pub type TradeType = String;
pub type TraderActivityStateView = String;
pub type TraderCapabilityFlags = NativeNumber;
pub type TraderStatePositionSnapshot = serde_json::Value;
pub type TraderStateSplineSnapshot = serde_json::Value;
pub type TraderStateTriggerSnapshot = serde_json::Value;
pub type UserLiquidationHistoryKind = String;
pub type UserLiquidationHistoryRole = String;
pub type UserLiquidationHistoryType = String;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AckBeforeTimestampBody {
    #[serde(rename = "beforeTimestamp")]
    pub before_timestamp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AckNotificationsBody {
    #[serde(rename = "items")]
    pub items: Vec<AckNotificationItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AcknowledgeNotificationsParams {
    #[serde(rename = "trader_pubkey")]
    pub trader_pubkey: String,
    #[serde(rename = "body")]
    pub body: AckNotificationsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AcknowledgeNotificationsUpToTimestampParams {
    #[serde(rename = "trader_pubkey")]
    pub trader_pubkey: String,
    #[serde(rename = "body")]
    pub body: AckBeforeTimestampBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ActivateInviteResponse {
    #[serde(rename = "trader_pda")]
    pub trader_pda: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ActivateInviteWithReferralRequest {
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "referral_code")]
    pub referral_code: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ActivateReferralParams {
    #[serde(rename = "body")]
    pub body: ActivateInviteWithReferralRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ActivateReferralTxRequest {
    #[serde(rename = "recent_blockhash")]
    pub recent_blockhash: String,
    #[serde(rename = "referral_code")]
    pub referral_code: String,
    #[serde(rename = "trader_authority")]
    pub trader_authority: String,
    #[serde(rename = "trader_pda_index", default, skip_serializing_if = "Option::is_none")]
    pub trader_pda_index: Option<serde_json::Number>,
    #[serde(rename = "trader_subaccount_index", default, skip_serializing_if = "Option::is_none")]
    pub trader_subaccount_index: Option<serde_json::Number>,
    #[serde(rename = "transaction")]
    pub transaction: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ActivateReferralTxResponse {
    #[serde(rename = "referral_code")]
    pub referral_code: String,
    #[serde(rename = "signature", default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "status")]
    pub status: ActivateReferralTxStatus,
    #[serde(rename = "trader_pda")]
    pub trader_pda: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ActivateReferralWithUserFundedTransactionParams {
    #[serde(rename = "body")]
    pub body: ActivateReferralTxRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApiAccountMeta {
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
    #[serde(rename = "isWritable")]
    pub is_writable: bool,
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApiCandle {
    #[serde(rename = "close")]
    pub close: NativeNumber,
    #[serde(rename = "externalSource", default, skip_serializing_if = "Option::is_none")]
    pub external_source: Option<String>,
    #[serde(rename = "high")]
    pub high: NativeNumber,
    #[serde(rename = "low")]
    pub low: NativeNumber,
    #[serde(rename = "markClose", default, skip_serializing_if = "Option::is_none")]
    pub mark_close: Option<NativeNumber>,
    #[serde(rename = "markHigh", default, skip_serializing_if = "Option::is_none")]
    pub mark_high: Option<NativeNumber>,
    #[serde(rename = "markLow", default, skip_serializing_if = "Option::is_none")]
    pub mark_low: Option<NativeNumber>,
    #[serde(rename = "markOpen", default, skip_serializing_if = "Option::is_none")]
    pub mark_open: Option<NativeNumber>,
    #[serde(rename = "open")]
    pub open: NativeNumber,
    #[serde(rename = "time")]
    pub time: NativeNumber,
    #[serde(rename = "tradeCount", default, skip_serializing_if = "Option::is_none")]
    pub trade_count: Option<serde_json::Number>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<serde_json::Number>,
    #[serde(rename = "volumeQuote", default, skip_serializing_if = "Option::is_none")]
    pub volume_quote: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApiInstructionResponse {
    #[serde(rename = "data")]
    pub data: Vec<NativeNumber>,
    #[serde(rename = "keys")]
    pub keys: Vec<ApiAccountMeta>,
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AuthResponse {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: NativeNumber,
    #[serde(rename = "pop_key")]
    pub pop_key: String,
    #[serde(rename = "refresh_expires_in")]
    pub refresh_expires_in: NativeNumber,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AuthoritySetView {
    #[serde(rename = "adlAuthority")]
    pub adl_authority: String,
    #[serde(rename = "backstopAuthority")]
    pub backstop_authority: String,
    #[serde(rename = "cancelAuthority")]
    pub cancel_authority: String,
    #[serde(rename = "marketAuthority")]
    pub market_authority: String,
    #[serde(rename = "oracleAuthority")]
    pub oracle_authority: String,
    #[serde(rename = "riskAuthority")]
    pub risk_authority: String,
    #[serde(rename = "rootAuthority")]
    pub root_authority: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildAttachedConditionalOrderTransactionParams {
    #[serde(rename = "body")]
    pub body: PlaceAttachedConditionalOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildCancelConditionalOrderTransactionParams {
    #[serde(rename = "body")]
    pub body: CancelConditionalOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildCancelStopLossOrderTransactionParams {
    #[serde(rename = "body")]
    pub body: CancelStopLossOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildIsolatedLimitOrderTransactionParams {
    #[serde(rename = "body")]
    pub body: PlaceIsolatedLimitOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildIsolatedLimitOrderTransactionWithConditionalsParams {
    #[serde(rename = "body")]
    pub body: PlaceIsolatedLimitOrderWithConditionalsRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildIsolatedLimitOrderTransactionWithLiquidationEstimateParams {
    #[serde(rename = "body")]
    pub body: PlaceIsolatedLimitOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildIsolatedMarketOrderTransactionParams {
    #[serde(rename = "body")]
    pub body: PlaceIsolatedMarketOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildIsolatedMarketOrderTransactionWithLiquidationEstimateParams {
    #[serde(rename = "body")]
    pub body: PlaceIsolatedMarketOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildPositionConditionalOrderTransactionParams {
    #[serde(rename = "body")]
    pub body: PlacePositionConditionalOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildRegisterIxsRequest {
    #[serde(rename = "maxPositions", default, skip_serializing_if = "Option::is_none")]
    pub max_positions: Option<serde_json::Number>,
    #[serde(rename = "traderAuthority")]
    pub trader_authority: String,
    #[serde(rename = "txFeePayer")]
    pub tx_fee_payer: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildRegisterIxsResponse {
    #[serde(rename = "includeRegisterTrader")]
    pub include_register_trader: bool,
    #[serde(rename = "instructions")]
    pub instructions: Vec<ApiInstructionResponse>,
    #[serde(rename = "maxPositions")]
    pub max_positions: NativeNumber,
    #[serde(rename = "traderOnboarder")]
    pub trader_onboarder: String,
    #[serde(rename = "traderPda")]
    pub trader_pda: String,
    #[serde(rename = "txFeePayer")]
    pub tx_fee_payer: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildRegisterTraderInstructionsParams {
    #[serde(rename = "body")]
    pub body: BuildRegisterIxsRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuildStopLossOrderTransactionParams {
    #[serde(rename = "body")]
    pub body: PlaceStopLossOrderRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelConditionalOrderRequest {
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "conditionalOrderIndex")]
    pub conditional_order_index: NativeNumber,
    #[serde(rename = "executionDirection")]
    pub execution_direction: String,
    #[serde(rename = "isIsolated", default, skip_serializing_if = "Option::is_none")]
    pub is_isolated: Option<bool>,
    #[serde(rename = "positionAuthority", default, skip_serializing_if = "Option::is_none")]
    pub position_authority: Option<String>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "traderPdaIndex")]
    pub trader_pda_index: NativeNumber,
    #[serde(rename = "traderSubaccountIndex", default, skip_serializing_if = "Option::is_none")]
    pub trader_subaccount_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelStopLossOrderRequest {
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "executionDirection")]
    pub execution_direction: String,
    #[serde(rename = "isIsolated", default, skip_serializing_if = "Option::is_none")]
    pub is_isolated: Option<bool>,
    #[serde(rename = "positionAuthority", default, skip_serializing_if = "Option::is_none")]
    pub position_authority: Option<String>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "traderPdaIndex")]
    pub trader_pda_index: NativeNumber,
    #[serde(rename = "traderSubaccountIndex", default, skip_serializing_if = "Option::is_none")]
    pub trader_subaccount_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CapabilityAccessView {
    #[serde(rename = "immediate")]
    pub immediate: bool,
    #[serde(rename = "viaColdActivation", default, skip_serializing_if = "Option::is_none")]
    pub via_cold_activation: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CheckWalletInviteStatusParams {
    #[serde(rename = "wallet")]
    pub wallet: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CheckWalletResponse {
    #[serde(rename = "invite_code_used", default, skip_serializing_if = "Option::is_none")]
    pub invite_code_used: Option<String>,
    #[serde(rename = "whitelisted")]
    pub whitelisted: bool,
    #[serde(rename = "whitelisted_at", default, skip_serializing_if = "Option::is_none")]
    pub whitelisted_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CollateralEvent {
    #[serde(rename = "amount")]
    pub amount: NativeNumber,
    #[serde(rename = "collateralAfter")]
    pub collateral_after: NativeNumber,
    #[serde(rename = "eventIndex")]
    pub event_index: NativeNumber,
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "slot")]
    pub slot: NativeNumber,
    #[serde(rename = "slotIndex")]
    pub slot_index: NativeNumber,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "traderPdaIndex")]
    pub trader_pda_index: NativeNumber,
    #[serde(rename = "traderSubaccountIndex")]
    pub trader_subaccount_index: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CollateralHistoryResponse {
    #[serde(rename = "data")]
    pub data: Vec<CollateralEvent>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "nextCursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "prevCursor", default, skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CommodityMarketCalendarResponse {
    #[serde(rename = "calendar")]
    pub calendar: CommodityMarketCalendarView,
    #[serde(rename = "loadedAt")]
    pub loaded_at: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "rawJson")]
    pub raw_json: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CommodityMarketCalendarView {
    #[serde(rename = "dateOverrides")]
    pub date_overrides: std::collections::BTreeMap<String,CommodityMarketDaySchedule>,
    #[serde(rename = "weeklySchedule")]
    pub weekly_schedule: std::collections::BTreeMap<String,CommodityMarketDaySchedule>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CommodityMarketDaySchedule {
    #[serde(rename = "sessions")]
    pub sessions: Vec<CommodityMarketSessionRange>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CommodityMarketSessionRange {
    #[serde(rename = "end")]
    pub end: String,
    #[serde(rename = "mode")]
    pub mode: String,
    #[serde(rename = "start")]
    pub start: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateServiceLoginChallengeParams {
    #[serde(rename = "body")]
    pub body: ServiceChallengeRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateWalletTransactionChallengeParams {
    #[serde(rename = "body")]
    pub body: WalletTransactionChallengeRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeKeysView {
    #[serde(rename = "activeTraderBuffer")]
    pub active_trader_buffer: Vec<String>,
    #[serde(rename = "canonicalMint")]
    pub canonical_mint: String,
    #[serde(rename = "currentAuthorities")]
    pub current_authorities: AuthoritySetView,
    #[serde(rename = "globalConfig")]
    pub global_config: String,
    #[serde(rename = "globalTraderIndex")]
    pub global_trader_index: Vec<String>,
    #[serde(rename = "globalVault")]
    pub global_vault: String,
    #[serde(rename = "pendingAuthorities")]
    pub pending_authorities: AuthoritySetView,
    #[serde(rename = "perpAssetMap")]
    pub perp_asset_map: String,
    #[serde(rename = "withdrawQueue")]
    pub withdraw_queue: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeLeverageTier {
    #[serde(rename = "limitOrderRiskFactor")]
    pub limit_order_risk_factor: NativeNumber,
    #[serde(rename = "limitOrderRiskFactorBps", default, skip_serializing_if = "Option::is_none")]
    pub limit_order_risk_factor_bps: Option<serde_json::Number>,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: NativeNumber,
    #[serde(rename = "maxSizeBaseLots")]
    pub max_size_base_lots: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeMarketConfig {
    #[serde(rename = "assetId")]
    pub asset_id: NativeNumber,
    #[serde(rename = "baseLotsDecimals")]
    pub base_lots_decimals: NativeNumber,
    #[serde(rename = "commodityMetadata", default, skip_serializing_if = "Option::is_none")]
    pub commodity_metadata: Option<serde_json::Value>,
    #[serde(rename = "fundingIntervalSeconds")]
    pub funding_interval_seconds: NativeNumber,
    #[serde(rename = "fundingPeriodSeconds")]
    pub funding_period_seconds: NativeNumber,
    #[serde(rename = "isolatedOnly")]
    pub isolated_only: bool,
    #[serde(rename = "leverageTiers")]
    pub leverage_tiers: Vec<ExchangeLeverageTier>,
    #[serde(rename = "makerFee")]
    pub maker_fee: NativeNumber,
    #[serde(rename = "marketPubkey")]
    pub market_pubkey: String,
    #[serde(rename = "marketStatus")]
    pub market_status: MarketStatus,
    #[serde(rename = "maxFundingRatePerInterval")]
    pub max_funding_rate_per_interval: NativeNumber,
    #[serde(rename = "maxFundingRatePerIntervalPercentage", default, skip_serializing_if = "Option::is_none")]
    pub max_funding_rate_per_interval_percentage: Option<NativeNumber>,
    #[serde(rename = "maxLiquidationSizeBaseLots")]
    pub max_liquidation_size_base_lots: JsSafeU64,
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "openInterestCapBaseLots")]
    pub open_interest_cap_base_lots: JsSafeU64,
    #[serde(rename = "riskFactors")]
    pub risk_factors: ExchangeRiskFactors,
    #[serde(rename = "splinePubkey")]
    pub spline_pubkey: String,
    #[serde(rename = "statsSnapshot", default, skip_serializing_if = "Option::is_none")]
    pub stats_snapshot: Option<serde_json::Value>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "takerFee")]
    pub taker_fee: NativeNumber,
    #[serde(rename = "tickSize")]
    pub tick_size: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeMarketSnapshot {
    #[serde(rename = "assetId")]
    pub asset_id: NativeNumber,
    #[serde(rename = "baseLotsDecimals")]
    pub base_lots_decimals: NativeNumber,
    #[serde(rename = "commodityMetadata", default, skip_serializing_if = "Option::is_none")]
    pub commodity_metadata: Option<serde_json::Value>,
    #[serde(rename = "fundingConfig")]
    pub funding_config: ExchangeWsFundingConfig,
    #[serde(rename = "isolatedOnly")]
    pub isolated_only: bool,
    #[serde(rename = "leverageTiers")]
    pub leverage_tiers: Vec<ExchangeWsLeverageTier>,
    #[serde(rename = "makerFee")]
    pub maker_fee: NativeNumber,
    #[serde(rename = "markPriceParameters")]
    pub mark_price_parameters: ExchangeWsMarkPriceParameters,
    #[serde(rename = "marketPubkey")]
    pub market_pubkey: String,
    #[serde(rename = "marketStatus")]
    pub market_status: MarketStatus,
    #[serde(rename = "maxLiquidationSizeBaseLots")]
    pub max_liquidation_size_base_lots: JsSafeU64,
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "openInterestCapBaseLots")]
    pub open_interest_cap_base_lots: JsSafeU64,
    #[serde(rename = "riskFactors")]
    pub risk_factors: ExchangeWsRiskFactors,
    #[serde(rename = "splinePubkey")]
    pub spline_pubkey: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "takerFee")]
    pub taker_fee: NativeNumber,
    #[serde(rename = "tickSize")]
    pub tick_size: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeRiskFactors {
    #[serde(rename = "backstop")]
    pub backstop: NativeNumber,
    #[serde(rename = "backstopBps", default, skip_serializing_if = "Option::is_none")]
    pub backstop_bps: Option<serde_json::Number>,
    #[serde(rename = "cancelOrder")]
    pub cancel_order: NativeNumber,
    #[serde(rename = "cancelOrderBps", default, skip_serializing_if = "Option::is_none")]
    pub cancel_order_bps: Option<serde_json::Number>,
    #[serde(rename = "highRisk")]
    pub high_risk: NativeNumber,
    #[serde(rename = "highRiskBps", default, skip_serializing_if = "Option::is_none")]
    pub high_risk_bps: Option<serde_json::Number>,
    #[serde(rename = "maintenance")]
    pub maintenance: NativeNumber,
    #[serde(rename = "maintenanceBps", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_bps: Option<serde_json::Number>,
    #[serde(rename = "upnl")]
    pub upnl: NativeNumber,
    #[serde(rename = "upnlBps", default, skip_serializing_if = "Option::is_none")]
    pub upnl_bps: Option<serde_json::Number>,
    #[serde(rename = "upnlForWithdrawals")]
    pub upnl_for_withdrawals: NativeNumber,
    #[serde(rename = "upnlForWithdrawalsBps", default, skip_serializing_if = "Option::is_none")]
    pub upnl_for_withdrawals_bps: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeSnapshotView {
    #[serde(rename = "exchange")]
    pub exchange: ExchangeStateSnapshot,
    #[serde(rename = "markets")]
    pub markets: Vec<ExchangeMarketSnapshot>,
    #[serde(rename = "sequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<serde_json::Value>,
    #[serde(rename = "slot")]
    pub slot: NativeNumber,
    #[serde(rename = "slotIndex")]
    pub slot_index: NativeNumber,
    #[serde(rename = "version")]
    pub version: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeStateSnapshot {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "activeTraderBuffer")]
    pub active_trader_buffer: Vec<String>,
    #[serde(rename = "canonicalMint")]
    pub canonical_mint: String,
    #[serde(rename = "currentAuthorities")]
    pub current_authorities: AuthoritySetView,
    #[serde(rename = "exchangeStatusBits")]
    pub exchange_status_bits: NativeNumber,
    #[serde(rename = "exchangeStatusFeatures")]
    pub exchange_status_features: Vec<String>,
    #[serde(rename = "gated")]
    pub gated: bool,
    #[serde(rename = "globalConfig")]
    pub global_config: String,
    #[serde(rename = "globalTraderIndex")]
    pub global_trader_index: Vec<String>,
    #[serde(rename = "globalVault")]
    pub global_vault: String,
    #[serde(rename = "perpAssetMap")]
    pub perp_asset_map: String,
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(rename = "usdcMint")]
    pub usdc_mint: String,
    #[serde(rename = "withdrawQueue")]
    pub withdraw_queue: String,
    #[serde(rename = "withdrawalsAvailable", default, skip_serializing_if = "Option::is_none")]
    pub withdrawals_available: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeStatusView {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "gated")]
    pub gated: bool,
    #[serde(rename = "withdrawalsAvailable", default, skip_serializing_if = "Option::is_none")]
    pub withdrawals_available: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeView {
    #[serde(rename = "keys")]
    pub keys: ExchangeKeysView,
    #[serde(rename = "markets")]
    pub markets: Vec<ExchangeMarketConfig>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeWsFundingConfig {
    #[serde(rename = "fundingIntervalSeconds")]
    pub funding_interval_seconds: NativeNumber,
    #[serde(rename = "fundingPeriodSeconds")]
    pub funding_period_seconds: NativeNumber,
    #[serde(rename = "maxFundingRatePerInterval")]
    pub max_funding_rate_per_interval: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeWsLeverageTier {
    #[serde(rename = "limitOrderRiskFactor")]
    pub limit_order_risk_factor: NativeNumber,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: NativeNumber,
    #[serde(rename = "maxSizeBaseLots")]
    pub max_size_base_lots: JsSafeU64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeWsMarkPriceParameters {
    #[serde(rename = "adjustedExchangeSpotPriceWeight")]
    pub adjusted_exchange_spot_price_weight: JsSafeU64,
    #[serde(rename = "bookHardStaleMultiplier", default, skip_serializing_if = "Option::is_none")]
    pub book_hard_stale_multiplier: Option<NativeNumber>,
    #[serde(rename = "bookPriceRadius")]
    pub book_price_radius: JsSafeU64,
    #[serde(rename = "bookPriceStaleThreshold")]
    pub book_price_stale_threshold: JsSafeU64,
    #[serde(rename = "bookPriceWeight")]
    pub book_price_weight: JsSafeU64,
    #[serde(rename = "commoditiesAfterHoursRadius")]
    pub commodities_after_hours_radius: JsSafeU64,
    #[serde(rename = "commoditiesAfterHoursRadiusBps")]
    pub commodities_after_hours_radius_bps: JsSafeU64,
    #[serde(rename = "emaDiffRadius")]
    pub ema_diff_radius: JsSafeU64,
    #[serde(rename = "emaPeriodSlots")]
    pub ema_period_slots: JsSafeU64,
    #[serde(rename = "exchangePerpPriceWeight")]
    pub exchange_perp_price_weight: JsSafeU64,
    #[serde(rename = "minOracleResponses")]
    pub min_oracle_responses: NativeNumber,
    #[serde(rename = "oracleDivergenceRadius")]
    pub oracle_divergence_radius: NativeNumber,
    #[serde(rename = "oracleHardStaleMultiplier", default, skip_serializing_if = "Option::is_none")]
    pub oracle_hard_stale_multiplier: Option<NativeNumber>,
    #[serde(rename = "perpPriceStaleThreshold")]
    pub perp_price_stale_threshold: JsSafeU64,
    #[serde(rename = "riskActionPriceValidityRules")]
    pub risk_action_price_validity_rules: Vec<Vec<Vec<ExchangeWsValidationRule>>>,
    #[serde(rename = "spotPriceStaleThreshold")]
    pub spot_price_stale_threshold: JsSafeU64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeWsRiskFactors {
    #[serde(rename = "backstop")]
    pub backstop: NativeNumber,
    #[serde(rename = "backstopBps", default, skip_serializing_if = "Option::is_none")]
    pub backstop_bps: Option<serde_json::Number>,
    #[serde(rename = "cancelOrder")]
    pub cancel_order: NativeNumber,
    #[serde(rename = "cancelOrderBps", default, skip_serializing_if = "Option::is_none")]
    pub cancel_order_bps: Option<serde_json::Number>,
    #[serde(rename = "highRisk")]
    pub high_risk: NativeNumber,
    #[serde(rename = "highRiskBps", default, skip_serializing_if = "Option::is_none")]
    pub high_risk_bps: Option<serde_json::Number>,
    #[serde(rename = "maintenance")]
    pub maintenance: NativeNumber,
    #[serde(rename = "maintenanceBps", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_bps: Option<serde_json::Number>,
    #[serde(rename = "upnl")]
    pub upnl: NativeNumber,
    #[serde(rename = "upnlBps", default, skip_serializing_if = "Option::is_none")]
    pub upnl_bps: Option<serde_json::Number>,
    #[serde(rename = "upnlForWithdrawals")]
    pub upnl_for_withdrawals: NativeNumber,
    #[serde(rename = "upnlForWithdrawalsBps", default, skip_serializing_if = "Option::is_none")]
    pub upnl_for_withdrawals_bps: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillRecord {
    #[serde(rename = "baseQty")]
    pub base_qty: String,
    #[serde(rename = "instructionType")]
    pub instruction_type: String,
    #[serde(rename = "marketSymbol")]
    pub market_symbol: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "quoteQty")]
    pub quote_qty: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "transactionSignature")]
    pub transaction_signature: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillsResponse {
    #[serde(rename = "data")]
    pub data: Vec<FillRecord>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "nextCursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "prevCursor", default, skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundingHourlyEvent {
    #[serde(rename = "fundingPayment")]
    pub funding_payment: String,
    #[serde(rename = "fundingRatePercentage")]
    pub funding_rate_percentage: String,
    #[serde(rename = "positionSide")]
    pub position_side: String,
    #[serde(rename = "positionSize")]
    pub position_size: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundingHourlyHistoryResponse {
    #[serde(rename = "events")]
    pub events: Vec<FundingHourlyEvent>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "nextCursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "prevCursor", default, skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundingOverviewPoint {
    #[serde(rename = "fundingAmountPerUnit")]
    pub funding_amount_per_unit: String,
    #[serde(rename = "fundingRate")]
    pub funding_rate: String,
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundingOverviewResponse {
    #[serde(rename = "series")]
    pub series: Vec<FundingOverviewSeries>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundingOverviewSeries {
    #[serde(rename = "marketId")]
    pub market_id: NativeNumber,
    #[serde(rename = "points")]
    pub points: Vec<FundingOverviewPoint>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundingRateHistoryResponse {
    #[serde(rename = "marketId")]
    pub market_id: NativeNumber,
    #[serde(rename = "rates")]
    pub rates: Vec<FundingRatePoint>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundingRatePoint {
    #[serde(rename = "fundingRatePercentage")]
    pub funding_rate_percentage: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCandlesParams {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "timeframe")]
    pub timeframe: String,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<NativeNumber>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(rename = "enableExternalSource", default, skip_serializing_if = "Option::is_none")]
    pub enable_external_source: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetCommodityMarketCalendarParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetExchangeKeysParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetExchangeParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetExchangeSnapshotParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetExchangeStatusParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFundingOverviewParams {
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<NativeNumber>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NativeNumber>,
    #[serde(rename = "perMarketLimit", default, skip_serializing_if = "Option::is_none")]
    pub per_market_limit: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetFundingRateHistoryParams {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<NativeNumber>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetJwksParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarkPriceParams {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketCalendarByIdParams {
    #[serde(rename = "market_calendar_id")]
    pub market_calendar_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketCalendarParams {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketFillsParams {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<serde_json::Number>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketParams {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetMarketStatsHistoryParams {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "start_time", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "end_time", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "timeframe", default, skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetNextCommodityMarketTransitionParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetNextMarketCalendarTransitionParams {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetNotificationsResponse {
    #[serde(rename = "items")]
    pub items: Vec<NotificationItem>,
    #[serde(rename = "nextCursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderHistoryParams {
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "traderPdaIndex", default, skip_serializing_if = "Option::is_none")]
    pub trader_pda_index: Option<NativeNumber>,
    #[serde(rename = "marketSymbol", default, skip_serializing_if = "Option::is_none")]
    pub market_symbol: Option<String>,
    #[serde(rename = "limit")]
    pub limit: NativeNumber,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "privyId", default, skip_serializing_if = "Option::is_none")]
    pub privy_id: Option<String>,
    #[serde(rename = "orderStatus", default, skip_serializing_if = "Option::is_none")]
    pub order_status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderHistoryV2Params {
    #[serde(rename = "market_symbol", default, skip_serializing_if = "Option::is_none")]
    pub market_symbol: Option<String>,
    #[serde(rename = "limit")]
    pub limit: NativeNumber,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "trader_pubkey")]
    pub trader_pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetOrderbookParams {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "include_splines", default, skip_serializing_if = "Option::is_none")]
    pub include_splines: Option<bool>,
    #[serde(rename = "bypass_execution_band", default, skip_serializing_if = "Option::is_none")]
    pub bypass_execution_band: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetReferralActivationPermissionParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTradeHistoryParams {
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "pdaIndex", default, skip_serializing_if = "Option::is_none")]
    pub pda_index: Option<NativeNumber>,
    #[serde(rename = "marketSymbol", default, skip_serializing_if = "Option::is_none")]
    pub market_symbol: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTraderCapabilitiesParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTraderCollateralHistoryParams {
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "pdaIndex", default, skip_serializing_if = "Option::is_none")]
    pub pda_index: Option<NativeNumber>,
    #[serde(rename = "nextCursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "prevCursor", default, skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "limit")]
    pub limit: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTraderFundingHistoryParams {
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "traderPdaIndex", default, skip_serializing_if = "Option::is_none")]
    pub trader_pda_index: Option<NativeNumber>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<NativeNumber>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "resolution", default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTraderMarketPnLParams {
    #[serde(rename = "trader_pubkey")]
    pub trader_pubkey: String,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<NativeNumber>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTraderParams {
    #[serde(rename = "pubkey")]
    pub pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTraderPnLParams {
    #[serde(rename = "trader_pubkey")]
    pub trader_pubkey: String,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<NativeNumber>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTraderPortfolioValuesParams {
    #[serde(rename = "trader_pubkey")]
    pub trader_pubkey: String,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<NativeNumber>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTraderStateParams {
    #[serde(rename = "traderPdaIndex", default, skip_serializing_if = "Option::is_none")]
    pub trader_pda_index: Option<NativeNumber>,
    #[serde(rename = "authority_pubkey")]
    pub authority_pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetTraderTradeHistoryV2Params {
    #[serde(rename = "market_symbol", default, skip_serializing_if = "Option::is_none")]
    pub market_symbol: Option<String>,
    #[serde(rename = "trader_pda_index", default, skip_serializing_if = "Option::is_none")]
    pub trader_pda_index: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "privy_id", default, skip_serializing_if = "Option::is_none")]
    pub privy_id: Option<String>,
    #[serde(rename = "trader_pubkey")]
    pub trader_pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetUserCollateralHistoryParams {
    #[serde(rename = "limit")]
    pub limit: NativeNumber,
    #[serde(rename = "nextCursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "prevCursor", default, skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "user_pubkey")]
    pub user_pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetUserHourlyFundingParams {
    #[serde(rename = "user_pubkey")]
    pub user_pubkey: String,
    #[serde(rename = "traderPdaIndex", default, skip_serializing_if = "Option::is_none")]
    pub trader_pda_index: Option<NativeNumber>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetUserLiquidationHistoryParams {
    #[serde(rename = "pdaIndex", default, skip_serializing_if = "Option::is_none")]
    pub pda_index: Option<NativeNumber>,
    #[serde(rename = "subaccountIndex", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_index: Option<serde_json::Number>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "user_pubkey")]
    pub user_pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetUserPnLParams {
    #[serde(rename = "user_pubkey")]
    pub user_pubkey: String,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<NativeNumber>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetUserTradeHistoryV2Params {
    #[serde(rename = "market_symbol", default, skip_serializing_if = "Option::is_none")]
    pub market_symbol: Option<String>,
    #[serde(rename = "trader_pda_index", default, skip_serializing_if = "Option::is_none")]
    pub trader_pda_index: Option<NativeNumber>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<NativeNumber>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "privy_id", default, skip_serializing_if = "Option::is_none")]
    pub privy_id: Option<String>,
    #[serde(rename = "user_pubkey")]
    pub user_pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetV1TradersByTraderPubkeyCollateralHistoryParams {
    #[serde(rename = "limit")]
    pub limit: NativeNumber,
    #[serde(rename = "nextCursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "prevCursor", default, skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "trader_pubkey")]
    pub trader_pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GetWalletLoginNonceParams {
    #[serde(rename = "wallet_pubkey")]
    pub wallet_pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct IntendedOrder {
    #[serde(rename = "baseLots")]
    pub base_lots: NativeNumber,
    #[serde(rename = "baseQty")]
    pub base_qty: String,
    #[serde(rename = "cancelExisting")]
    pub cancel_existing: bool,
    #[serde(rename = "clientOrderId", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "hasPriceLimit")]
    pub has_price_limit: bool,
    #[serde(rename = "isConditionalOrder", default, skip_serializing_if = "Option::is_none")]
    pub is_conditional_order: Option<bool>,
    #[serde(rename = "isPostOnly")]
    pub is_post_only: bool,
    #[serde(rename = "isReduceOnly")]
    pub is_reduce_only: bool,
    #[serde(rename = "isStopLoss")]
    pub is_stop_loss: bool,
    #[serde(rename = "isStopLossDirection", default, skip_serializing_if = "Option::is_none")]
    pub is_stop_loss_direction: Option<bool>,
    #[serde(rename = "isTakeOnly")]
    pub is_take_only: bool,
    #[serde(rename = "lastValidSlot", default, skip_serializing_if = "Option::is_none")]
    pub last_valid_slot: Option<serde_json::Number>,
    #[serde(rename = "matchLimit", default, skip_serializing_if = "Option::is_none")]
    pub match_limit: Option<serde_json::Number>,
    #[serde(rename = "orderFlags")]
    pub order_flags: NativeNumber,
    #[serde(rename = "orderKind")]
    pub order_kind: String,
    #[serde(rename = "priceInTicks", default, skip_serializing_if = "Option::is_none")]
    pub price_in_ticks: Option<serde_json::Number>,
    #[serde(rename = "priceUsd", default, skip_serializing_if = "Option::is_none")]
    pub price_usd: Option<String>,
    #[serde(rename = "quoteLotBudget", default, skip_serializing_if = "Option::is_none")]
    pub quote_lot_budget: Option<serde_json::Number>,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct JwksResponse {
    #[serde(rename = "keys")]
    pub keys: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LimitOrder {
    #[serde(rename = "initialTradeSize")]
    pub initial_trade_size: String,
    #[serde(rename = "isConditionalOrder", default, skip_serializing_if = "Option::is_none")]
    pub is_conditional_order: Option<bool>,
    #[serde(rename = "isReduceOnly")]
    pub is_reduce_only: bool,
    #[serde(rename = "isStopLoss", default, skip_serializing_if = "Option::is_none")]
    pub is_stop_loss: Option<bool>,
    #[serde(rename = "isStopLossDirection", default, skip_serializing_if = "Option::is_none")]
    pub is_stop_loss_direction: Option<bool>,
    #[serde(rename = "marginFactor")]
    pub margin_factor: String,
    #[serde(rename = "marginRequirement")]
    pub margin_requirement: String,
    #[serde(rename = "orderSequenceNumber")]
    pub order_sequence_number: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "side")]
    pub side: Side,
    #[serde(rename = "tradeSizeRemaining")]
    pub trade_size_remaining: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListMarketsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListTraderNotificationsParams {
    #[serde(rename = "trader_pubkey")]
    pub trader_pubkey: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "cursor", default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "unackedOnly", default, skip_serializing_if = "Option::is_none")]
    pub unacked_only: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LogInWithPrivyTokenParams {
    #[serde(rename = "body")]
    pub body: PrivyLoginRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LogInWithServiceSignatureParams {
    #[serde(rename = "body")]
    pub body: ServiceLoginRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LogInWithSignedWalletTransactionParams {
    #[serde(rename = "body")]
    pub body: WalletTransactionLoginRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LogInWithWalletSignatureParams {
    #[serde(rename = "body")]
    pub body: WalletLoginRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LogOutParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarkPriceResponse {
    #[serde(rename = "markPrice", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<serde_json::Value>,
    #[serde(rename = "slot")]
    pub slot: NativeNumber,
    #[serde(rename = "slotIndex")]
    pub slot_index: NativeNumber,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketCalendarRecord {
    #[serde(rename = "calendar", default, skip_serializing_if = "Option::is_none")]
    pub calendar: Option<serde_json::Value>,
    #[serde(rename = "calendarUri")]
    pub calendar_uri: String,
    #[serde(rename = "contentSha256")]
    pub content_sha256: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "loadedAt")]
    pub loaded_at: String,
    #[serde(rename = "marketCalendarId")]
    pub market_calendar_id: String,
    #[serde(rename = "rawJson")]
    pub raw_json: String,
    #[serde(rename = "s3Path")]
    pub s3_path: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketCalendarResponse {
    #[serde(rename = "calendar", default, skip_serializing_if = "Option::is_none")]
    pub calendar: Option<serde_json::Value>,
    #[serde(rename = "calendarUri")]
    pub calendar_uri: String,
    #[serde(rename = "contentSha256")]
    pub content_sha256: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "loadedAt")]
    pub loaded_at: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "marketCalendarId")]
    pub market_calendar_id: String,
    #[serde(rename = "rawJson")]
    pub raw_json: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketStatsHistoryResponse {
    #[serde(rename = "market_id")]
    pub market_id: NativeNumber,
    #[serde(rename = "stats")]
    pub stats: Vec<MarketStatsPoint>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "timeframe", default, skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketStatsPoint {
    #[serde(rename = "mark_price")]
    pub mark_price: NativeNumber,
    #[serde(rename = "open_interest")]
    pub open_interest: NativeNumber,
    #[serde(rename = "slot")]
    pub slot: NativeNumber,
    #[serde(rename = "spot_price")]
    pub spot_price: NativeNumber,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "total_maker_fees", default, skip_serializing_if = "Option::is_none")]
    pub total_maker_fees: Option<serde_json::Number>,
    #[serde(rename = "total_taker_fees", default, skip_serializing_if = "Option::is_none")]
    pub total_taker_fees: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct NextCommodityMarketTransition {
    #[serde(rename = "currentState")]
    pub current_state: CommodityMarketStateView,
    #[serde(rename = "loadedAt")]
    pub loaded_at: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "nextMarketState", default, skip_serializing_if = "Option::is_none")]
    pub next_market_state: Option<serde_json::Value>,
    #[serde(rename = "utcNextTransition", default, skip_serializing_if = "Option::is_none")]
    pub utc_next_transition: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct NextMarketCalendarTransition {
    #[serde(rename = "calendarUri")]
    pub calendar_uri: String,
    #[serde(rename = "currentState")]
    pub current_state: MarketCalendarStateView,
    #[serde(rename = "loadedAt")]
    pub loaded_at: String,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "marketCalendarId")]
    pub market_calendar_id: String,
    #[serde(rename = "nextMarketState", default, skip_serializing_if = "Option::is_none")]
    pub next_market_state: Option<serde_json::Value>,
    #[serde(rename = "utcNextTransition", default, skip_serializing_if = "Option::is_none")]
    pub utc_next_transition: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderbookView {
    #[serde(rename = "asks")]
    pub asks: Vec<[NativeNumber;2]>,
    #[serde(rename = "bids")]
    pub bids: Vec<[NativeNumber;2]>,
    #[serde(rename = "mid", default, skip_serializing_if = "Option::is_none")]
    pub mid: Option<serde_json::Number>,
    #[serde(rename = "slot")]
    pub slot: NativeNumber,
    #[serde(rename = "splines", default, skip_serializing_if = "Option::is_none")]
    pub splines: Option<Vec<Spline>>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PaginatedResponseVecOrderHistoryItem {
    #[serde(rename = "data")]
    pub data: Vec<PaginatedResponseVecOrderHistoryItemDataItem>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "nextCursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "prevCursor", default, skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PaginatedResponseVecOrderHistoryItemDataItem {
    #[serde(rename = "baseQty")]
    pub base_qty: String,
    #[serde(rename = "completedAt", default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "filledBaseQty")]
    pub filled_base_qty: String,
    #[serde(rename = "isConditionalOrder", default, skip_serializing_if = "Option::is_none")]
    pub is_conditional_order: Option<bool>,
    #[serde(rename = "isReduceOnly")]
    pub is_reduce_only: bool,
    #[serde(rename = "isStopLoss", default, skip_serializing_if = "Option::is_none")]
    pub is_stop_loss: Option<bool>,
    #[serde(rename = "isStopLossDirection", default, skip_serializing_if = "Option::is_none")]
    pub is_stop_loss_direction: Option<bool>,
    #[serde(rename = "marketSymbol")]
    pub market_symbol: String,
    #[serde(rename = "orderSequenceNumber")]
    pub order_sequence_number: String,
    #[serde(rename = "placedAt", default, skip_serializing_if = "Option::is_none")]
    pub placed_at: Option<String>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "remainingBaseQty")]
    pub remaining_base_qty: String,
    #[serde(rename = "side")]
    pub side: Side,
    #[serde(rename = "status")]
    pub status: OrderStatus,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PaginatedResponseVecOrderHistoryV2Item {
    #[serde(rename = "data")]
    pub data: Vec<PaginatedResponseVecOrderHistoryV2ItemDataItem>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "nextCursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "prevCursor", default, skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PaginatedResponseVecOrderHistoryV2ItemDataItem {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "currentOrderState", default, skip_serializing_if = "Option::is_none")]
    pub current_order_state: Option<serde_json::Value>,
    #[serde(rename = "eventIndex")]
    pub event_index: NativeNumber,
    #[serde(rename = "fillBeforePlacement", default, skip_serializing_if = "Option::is_none")]
    pub fill_before_placement: Option<serde_json::Value>,
    #[serde(rename = "instructionIndex")]
    pub instruction_index: NativeNumber,
    #[serde(rename = "instructionType")]
    pub instruction_type: String,
    #[serde(rename = "intended")]
    pub intended: IntendedOrder,
    #[serde(rename = "isConditionalOrder")]
    pub is_conditional_order: bool,
    #[serde(rename = "isReduceOnly")]
    pub is_reduce_only: bool,
    #[serde(rename = "isStopLoss")]
    pub is_stop_loss: bool,
    #[serde(rename = "isStopLossDirection")]
    pub is_stop_loss_direction: bool,
    #[serde(rename = "marketSymbol")]
    pub market_symbol: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "placedOrder", default, skip_serializing_if = "Option::is_none")]
    pub placed_order: Option<serde_json::Value>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "slot")]
    pub slot: NativeNumber,
    #[serde(rename = "slotIndex")]
    pub slot_index: NativeNumber,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "traderId")]
    pub trader_id: NativeNumber,
    #[serde(rename = "traderPdaIndex")]
    pub trader_pda_index: NativeNumber,
    #[serde(rename = "traderSubaccountIndex")]
    pub trader_subaccount_index: NativeNumber,
    #[serde(rename = "userId")]
    pub user_id: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PaginatedResponseVecTradeHistoryV2Item {
    #[serde(rename = "data")]
    pub data: Vec<PaginatedResponseVecTradeHistoryV2ItemDataItem>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "nextCursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "prevCursor", default, skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PaginatedResponseVecTradeHistoryV2ItemDataItem {
    #[serde(rename = "baseLotsAfter")]
    pub base_lots_after: String,
    #[serde(rename = "baseLotsBefore")]
    pub base_lots_before: String,
    #[serde(rename = "baseLotsDelta")]
    pub base_lots_delta: String,
    #[serde(rename = "eventIndex")]
    pub event_index: NativeNumber,
    #[serde(rename = "fees")]
    pub fees: String,
    #[serde(rename = "fillId", default, skip_serializing_if = "Option::is_none")]
    pub fill_id: Option<String>,
    #[serde(rename = "instructionIndex")]
    pub instruction_index: NativeNumber,
    #[serde(rename = "instructionType")]
    pub instruction_type: String,
    #[serde(rename = "liquidity")]
    pub liquidity: LiquidityRole,
    #[serde(rename = "marketSymbol")]
    pub market_symbol: String,
    #[serde(rename = "orderSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub order_sequence_number: Option<serde_json::Number>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: String,
    #[serde(rename = "signature", default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "slot")]
    pub slot: NativeNumber,
    #[serde(rename = "slotIndex")]
    pub slot_index: NativeNumber,
    #[serde(rename = "splineSequenceNumber", default, skip_serializing_if = "Option::is_none")]
    pub spline_sequence_number: Option<serde_json::Number>,
    #[serde(rename = "subaccountIndex")]
    pub subaccount_index: NativeNumber,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "tradeType")]
    pub trade_type: TradeType,
    #[serde(rename = "traderId")]
    pub trader_id: NativeNumber,
    #[serde(rename = "traderPdaIndex")]
    pub trader_pda_index: NativeNumber,
    #[serde(rename = "userId")]
    pub user_id: NativeNumber,
    #[serde(rename = "virtualQuoteLotsAfter")]
    pub virtual_quote_lots_after: String,
    #[serde(rename = "virtualQuoteLotsBefore")]
    pub virtual_quote_lots_before: String,
    #[serde(rename = "virtualQuoteLotsDelta")]
    pub virtual_quote_lots_delta: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PaginatedResponseVecUserLiquidationHistoryPoint {
    #[serde(rename = "data")]
    pub data: Vec<PaginatedResponseVecUserLiquidationHistoryPointDataItem>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "nextCursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "prevCursor", default, skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PaginatedResponseVecUserLiquidationHistoryPointDataItem {
    #[serde(rename = "atLossCloseValue", default, skip_serializing_if = "Option::is_none")]
    pub at_loss_close_value: Option<String>,
    #[serde(rename = "atLossCollateralChange", default, skip_serializing_if = "Option::is_none")]
    pub at_loss_collateral_change: Option<String>,
    #[serde(rename = "baseLotsFilled", default, skip_serializing_if = "Option::is_none")]
    pub base_lots_filled: Option<String>,
    #[serde(rename = "caller", default, skip_serializing_if = "Option::is_none")]
    pub caller: Option<String>,
    #[serde(rename = "closedLong", default, skip_serializing_if = "Option::is_none")]
    pub closed_long: Option<String>,
    #[serde(rename = "closedShort", default, skip_serializing_if = "Option::is_none")]
    pub closed_short: Option<String>,
    #[serde(rename = "eventIndex")]
    pub event_index: NativeNumber,
    #[serde(rename = "haircutRateBps", default, skip_serializing_if = "Option::is_none")]
    pub haircut_rate_bps: Option<serde_json::Number>,
    #[serde(rename = "inProfitAccount", default, skip_serializing_if = "Option::is_none")]
    pub in_profit_account: Option<String>,
    #[serde(rename = "inProfitCloseValue", default, skip_serializing_if = "Option::is_none")]
    pub in_profit_close_value: Option<String>,
    #[serde(rename = "inProfitCollateralChange", default, skip_serializing_if = "Option::is_none")]
    pub in_profit_collateral_change: Option<String>,
    #[serde(rename = "ixName")]
    pub ix_name: String,
    #[serde(rename = "kind")]
    pub kind: UserLiquidationHistoryKind,
    #[serde(rename = "liquidatee", default, skip_serializing_if = "Option::is_none")]
    pub liquidatee: Option<String>,
    #[serde(rename = "liquidateeCollateralChange", default, skip_serializing_if = "Option::is_none")]
    pub liquidatee_collateral_change: Option<String>,
    #[serde(rename = "liquidator", default, skip_serializing_if = "Option::is_none")]
    pub liquidator: Option<String>,
    #[serde(rename = "liquidatorCollateralChange", default, skip_serializing_if = "Option::is_none")]
    pub liquidator_collateral_change: Option<String>,
    #[serde(rename = "market")]
    pub market: String,
    #[serde(rename = "positionClosed", default, skip_serializing_if = "Option::is_none")]
    pub position_closed: Option<bool>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "quoteLotsFilled", default, skip_serializing_if = "Option::is_none")]
    pub quote_lots_filled: Option<String>,
    #[serde(rename = "quoteSize", default, skip_serializing_if = "Option::is_none")]
    pub quote_size: Option<String>,
    #[serde(rename = "role")]
    pub role: UserLiquidationHistoryRole,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "signature", default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "slot")]
    pub slot: NativeNumber,
    #[serde(rename = "slotIndex")]
    pub slot_index: NativeNumber,
    #[serde(rename = "subaccountIndex", default, skip_serializing_if = "Option::is_none")]
    pub subaccount_index: Option<serde_json::Number>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "timestamp")]
    pub timestamp: NativeNumber,
    #[serde(rename = "type")]
    pub type_: UserLiquidationHistoryType,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAttachedConditionalOrderRequest {
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "feePayer", default, skip_serializing_if = "Option::is_none")]
    pub fee_payer: Option<String>,
    #[serde(rename = "flightBuilderAuthority", default, skip_serializing_if = "Option::is_none")]
    pub flight_builder_authority: Option<String>,
    #[serde(rename = "flightFeeCollectorTrader", default, skip_serializing_if = "Option::is_none")]
    pub flight_fee_collector_trader: Option<String>,
    #[serde(rename = "greaterTrigger", default, skip_serializing_if = "Option::is_none")]
    pub greater_trigger: Option<serde_json::Value>,
    #[serde(rename = "isIsolated", default, skip_serializing_if = "Option::is_none")]
    pub is_isolated: Option<bool>,
    #[serde(rename = "lessTrigger", default, skip_serializing_if = "Option::is_none")]
    pub less_trigger: Option<serde_json::Value>,
    #[serde(rename = "orderPriceInTicks")]
    pub order_price_in_ticks: NativeNumber,
    #[serde(rename = "orderSequenceNumber")]
    pub order_sequence_number: String,
    #[serde(rename = "positionAuthority", default, skip_serializing_if = "Option::is_none")]
    pub position_authority: Option<String>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "traderPdaIndex")]
    pub trader_pda_index: NativeNumber,
    #[serde(rename = "traderSubaccountIndex", default, skip_serializing_if = "Option::is_none")]
    pub trader_subaccount_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceIsolatedLimitOrderEnhancedResponse {
    #[serde(rename = "estimatedLiquidationPriceUsd", default, skip_serializing_if = "Option::is_none")]
    pub estimated_liquidation_price_usd: Option<serde_json::Number>,
    #[serde(rename = "instructions")]
    pub instructions: Vec<ApiInstructionResponse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceIsolatedLimitOrderRequest {
    #[serde(rename = "allowCrossAndIsolatedForAsset", default, skip_serializing_if = "Option::is_none")]
    pub allow_cross_and_isolated_for_asset: Option<bool>,
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "feePayer", default, skip_serializing_if = "Option::is_none")]
    pub fee_payer: Option<String>,
    #[serde(rename = "flightBuilderAuthority", default, skip_serializing_if = "Option::is_none")]
    pub flight_builder_authority: Option<String>,
    #[serde(rename = "flightFeeCollectorTrader", default, skip_serializing_if = "Option::is_none")]
    pub flight_fee_collector_trader: Option<String>,
    #[serde(rename = "isPostOnly", default, skip_serializing_if = "Option::is_none")]
    pub is_post_only: Option<bool>,
    #[serde(rename = "isReduceOnly", default, skip_serializing_if = "Option::is_none")]
    pub is_reduce_only: Option<bool>,
    #[serde(rename = "numBaseLots", default, skip_serializing_if = "Option::is_none")]
    pub num_base_lots: Option<serde_json::Number>,
    #[serde(rename = "pdaIndex", default, skip_serializing_if = "Option::is_none")]
    pub pda_index: Option<serde_json::Number>,
    #[serde(rename = "positionAuthority", default, skip_serializing_if = "Option::is_none")]
    pub position_authority: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Number>,
    #[serde(rename = "priceInTicks", default, skip_serializing_if = "Option::is_none")]
    pub price_in_ticks: Option<serde_json::Number>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<serde_json::Number>,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "skipTransferToParent", default, skip_serializing_if = "Option::is_none")]
    pub skip_transfer_to_parent: Option<bool>,
    #[serde(rename = "slide", default, skip_serializing_if = "Option::is_none")]
    pub slide: Option<bool>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "tpSl", default, skip_serializing_if = "Option::is_none")]
    pub tp_sl: Option<serde_json::Value>,
    #[serde(rename = "transferAmount", default, skip_serializing_if = "Option::is_none")]
    pub transfer_amount: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceIsolatedLimitOrderWithConditionalsRequest {
    #[serde(rename = "allowCrossAndIsolatedForAsset", default, skip_serializing_if = "Option::is_none")]
    pub allow_cross_and_isolated_for_asset: Option<bool>,
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "feePayer", default, skip_serializing_if = "Option::is_none")]
    pub fee_payer: Option<String>,
    #[serde(rename = "flightBuilderAuthority", default, skip_serializing_if = "Option::is_none")]
    pub flight_builder_authority: Option<String>,
    #[serde(rename = "flightFeeCollectorTrader", default, skip_serializing_if = "Option::is_none")]
    pub flight_fee_collector_trader: Option<String>,
    #[serde(rename = "greaterTrigger", default, skip_serializing_if = "Option::is_none")]
    pub greater_trigger: Option<serde_json::Value>,
    #[serde(rename = "isPostOnly", default, skip_serializing_if = "Option::is_none")]
    pub is_post_only: Option<bool>,
    #[serde(rename = "lessTrigger", default, skip_serializing_if = "Option::is_none")]
    pub less_trigger: Option<serde_json::Value>,
    #[serde(rename = "numBaseLots", default, skip_serializing_if = "Option::is_none")]
    pub num_base_lots: Option<serde_json::Number>,
    #[serde(rename = "pdaIndex", default, skip_serializing_if = "Option::is_none")]
    pub pda_index: Option<serde_json::Number>,
    #[serde(rename = "positionAuthority", default, skip_serializing_if = "Option::is_none")]
    pub position_authority: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Number>,
    #[serde(rename = "priceInTicks", default, skip_serializing_if = "Option::is_none")]
    pub price_in_ticks: Option<serde_json::Number>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<serde_json::Number>,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "skipTransferToParent", default, skip_serializing_if = "Option::is_none")]
    pub skip_transfer_to_parent: Option<bool>,
    #[serde(rename = "slide", default, skip_serializing_if = "Option::is_none")]
    pub slide: Option<bool>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "transferAmount", default, skip_serializing_if = "Option::is_none")]
    pub transfer_amount: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceIsolatedMarketOrderEnhancedResponse {
    #[serde(rename = "estimatedLiquidationPriceUsd", default, skip_serializing_if = "Option::is_none")]
    pub estimated_liquidation_price_usd: Option<serde_json::Number>,
    #[serde(rename = "instructions")]
    pub instructions: Vec<ApiInstructionResponse>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceIsolatedMarketOrderRequest {
    #[serde(rename = "allowCrossAndIsolatedForAsset", default, skip_serializing_if = "Option::is_none")]
    pub allow_cross_and_isolated_for_asset: Option<bool>,
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "feePayer", default, skip_serializing_if = "Option::is_none")]
    pub fee_payer: Option<String>,
    #[serde(rename = "flightBuilderAuthority", default, skip_serializing_if = "Option::is_none")]
    pub flight_builder_authority: Option<String>,
    #[serde(rename = "flightFeeCollectorTrader", default, skip_serializing_if = "Option::is_none")]
    pub flight_fee_collector_trader: Option<String>,
    #[serde(rename = "isReduceOnly", default, skip_serializing_if = "Option::is_none")]
    pub is_reduce_only: Option<bool>,
    #[serde(rename = "maxPriceInTicks", default, skip_serializing_if = "Option::is_none")]
    pub max_price_in_ticks: Option<serde_json::Number>,
    #[serde(rename = "numBaseLots", default, skip_serializing_if = "Option::is_none")]
    pub num_base_lots: Option<serde_json::Number>,
    #[serde(rename = "pdaIndex", default, skip_serializing_if = "Option::is_none")]
    pub pda_index: Option<serde_json::Number>,
    #[serde(rename = "positionAuthority", default, skip_serializing_if = "Option::is_none")]
    pub position_authority: Option<String>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<serde_json::Number>,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "skipTransferToParent", default, skip_serializing_if = "Option::is_none")]
    pub skip_transfer_to_parent: Option<bool>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "tpSl", default, skip_serializing_if = "Option::is_none")]
    pub tp_sl: Option<serde_json::Value>,
    #[serde(rename = "transferAmount", default, skip_serializing_if = "Option::is_none")]
    pub transfer_amount: Option<NativeNumber>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlacePositionConditionalOrderRequest {
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "feePayer", default, skip_serializing_if = "Option::is_none")]
    pub fee_payer: Option<String>,
    #[serde(rename = "flightBuilderAuthority", default, skip_serializing_if = "Option::is_none")]
    pub flight_builder_authority: Option<String>,
    #[serde(rename = "flightFeeCollectorTrader", default, skip_serializing_if = "Option::is_none")]
    pub flight_fee_collector_trader: Option<String>,
    #[serde(rename = "greaterTrigger", default, skip_serializing_if = "Option::is_none")]
    pub greater_trigger: Option<serde_json::Value>,
    #[serde(rename = "isIsolated", default, skip_serializing_if = "Option::is_none")]
    pub is_isolated: Option<bool>,
    #[serde(rename = "lessTrigger", default, skip_serializing_if = "Option::is_none")]
    pub less_trigger: Option<serde_json::Value>,
    #[serde(rename = "numBaseLots", default, skip_serializing_if = "Option::is_none")]
    pub num_base_lots: Option<serde_json::Number>,
    #[serde(rename = "positionAuthority", default, skip_serializing_if = "Option::is_none")]
    pub position_authority: Option<String>,
    #[serde(rename = "quantity", default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<serde_json::Number>,
    #[serde(rename = "sizePercent", default, skip_serializing_if = "Option::is_none")]
    pub size_percent: Option<serde_json::Number>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "traderPdaIndex")]
    pub trader_pda_index: NativeNumber,
    #[serde(rename = "traderSubaccountIndex", default, skip_serializing_if = "Option::is_none")]
    pub trader_subaccount_index: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PnLPoint {
    #[serde(rename = "cumulativeFundingPayment")]
    pub cumulative_funding_payment: NativeNumber,
    #[serde(rename = "cumulativePnl")]
    pub cumulative_pnl: NativeNumber,
    #[serde(rename = "cumulativeTakerFee")]
    pub cumulative_taker_fee: NativeNumber,
    #[serde(rename = "endTime")]
    pub end_time: NativeNumber,
    #[serde(rename = "startTime")]
    pub start_time: NativeNumber,
    #[serde(rename = "timestamp")]
    pub timestamp: NativeNumber,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PortfolioValuePoint {
    #[serde(rename = "endTime")]
    pub end_time: NativeNumber,
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<serde_json::Value>,
    #[serde(rename = "startTime")]
    pub start_time: NativeNumber,
    #[serde(rename = "timestamp")]
    pub timestamp: NativeNumber,
    #[serde(rename = "value")]
    pub value: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PrivyLoginRequest {
    #[serde(rename = "privy_token")]
    pub privy_token: String,
    #[serde(rename = "wallet_proof", default, skip_serializing_if = "Option::is_none")]
    pub wallet_proof: Option<serde_json::Value>,
    #[serde(rename = "wallet_pubkey", default, skip_serializing_if = "Option::is_none")]
    pub wallet_pubkey: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReferralActivationPermissionResponse {
    #[serde(rename = "permission_account")]
    pub permission_account: String,
    #[serde(rename = "risk_authority")]
    pub risk_authority: String,
    #[serde(rename = "trader_onboarder")]
    pub trader_onboarder: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RefreshRequest {
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RefreshSessionParams {
    #[serde(rename = "body")]
    pub body: RefreshRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SendRegisterIxsRequest {
    #[serde(rename = "maxPositions", default, skip_serializing_if = "Option::is_none")]
    pub max_positions: Option<serde_json::Number>,
    #[serde(rename = "traderAuthority")]
    pub trader_authority: String,
    #[serde(rename = "traderPdaIndex", default, skip_serializing_if = "Option::is_none")]
    pub trader_pda_index: Option<serde_json::Number>,
    #[serde(rename = "traderSubaccountIndex", default, skip_serializing_if = "Option::is_none")]
    pub trader_subaccount_index: Option<serde_json::Number>,
    #[serde(rename = "transaction")]
    pub transaction: String,
    #[serde(rename = "txFeePayer")]
    pub tx_fee_payer: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SendRegisterIxsResponse {
    #[serde(rename = "includeRegisterTrader")]
    pub include_register_trader: bool,
    #[serde(rename = "maxPositions")]
    pub max_positions: NativeNumber,
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(rename = "traderOnboarder")]
    pub trader_onboarder: String,
    #[serde(rename = "traderPda")]
    pub trader_pda: String,
    #[serde(rename = "txFeePayer")]
    pub tx_fee_payer: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ServiceChallengeRequest {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "key_id", default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ServiceChallengeResponse {
    #[serde(rename = "expires_at")]
    pub expires_at: String,
    #[serde(rename = "key_id")]
    pub key_id: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "nonce")]
    pub nonce: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ServiceLoginRequest {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "key_id", default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "nonce", default)]
    pub nonce: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Spline {
    #[serde(rename = "askFilledAmount")]
    pub ask_filled_amount: String,
    #[serde(rename = "askRegions")]
    pub ask_regions: Vec<TickRegion>,
    #[serde(rename = "bidFilledAmount")]
    pub bid_filled_amount: String,
    #[serde(rename = "bidRegions")]
    pub bid_regions: Vec<TickRegion>,
    #[serde(rename = "midPrice")]
    pub mid_price: NativeNumber,
    #[serde(rename = "traderKey")]
    pub trader_key: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SubmitRegisterTraderTransactionParams {
    #[serde(rename = "body")]
    pub body: SendRegisterIxsRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TickRegion {
    #[serde(rename = "density")]
    pub density: NativeNumber,
    #[serde(rename = "end")]
    pub end: NativeNumber,
    #[serde(rename = "filledSize")]
    pub filled_size: String,
    #[serde(rename = "start")]
    pub start: NativeNumber,
    #[serde(rename = "totalSize")]
    pub total_size: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderCapabilitiesMetadata {
    #[serde(rename = "capabilities")]
    pub capabilities: Vec<TraderCapabilityDescriptor>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderCapabilitiesView {
    #[serde(rename = "depositCollateral")]
    pub deposit_collateral: CapabilityAccessView,
    #[serde(rename = "placeLimitOrder")]
    pub place_limit_order: CapabilityAccessView,
    #[serde(rename = "placeMarketOrder")]
    pub place_market_order: CapabilityAccessView,
    #[serde(rename = "riskIncreasingTrade")]
    pub risk_increasing_trade: CapabilityAccessView,
    #[serde(rename = "riskReducingTrade")]
    pub risk_reducing_trade: CapabilityAccessView,
    #[serde(rename = "withdrawCollateral")]
    pub withdraw_collateral: CapabilityAccessView,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderCapabilityDescriptor {
    #[serde(rename = "deprecated", default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderFundingHistoryEvent {
    #[serde(rename = "fundingPayment")]
    pub funding_payment: String,
    #[serde(rename = "fundingRatePercentage")]
    pub funding_rate_percentage: String,
    #[serde(rename = "positionSide")]
    pub position_side: String,
    #[serde(rename = "positionSize")]
    pub position_size: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderFundingHistoryResponse {
    #[serde(rename = "events")]
    pub events: Vec<TraderFundingHistoryEvent>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "nextCursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "prevCursor", default, skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderMarketPnLPoint {
    #[serde(rename = "baseLots")]
    pub base_lots: NativeNumber,
    #[serde(rename = "cumulativeFundingPayment")]
    pub cumulative_funding_payment: NativeNumber,
    #[serde(rename = "cumulativePnl")]
    pub cumulative_pnl: NativeNumber,
    #[serde(rename = "cumulativeTakerFee")]
    pub cumulative_taker_fee: NativeNumber,
    #[serde(rename = "endTime")]
    pub end_time: NativeNumber,
    #[serde(rename = "markPrice")]
    pub mark_price: NativeNumber,
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: NativeNumber,
    #[serde(rename = "startTime")]
    pub start_time: NativeNumber,
    #[serde(rename = "timestamp")]
    pub timestamp: NativeNumber,
    #[serde(rename = "totalTakerFee")]
    pub total_taker_fee: NativeNumber,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: NativeNumber,
    #[serde(rename = "virtualQuoteLots")]
    pub virtual_quote_lots: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderMarketPnLSeries {
    #[serde(rename = "baseDecimals")]
    pub base_decimals: NativeNumber,
    #[serde(rename = "marketId")]
    pub market_id: NativeNumber,
    #[serde(rename = "points")]
    pub points: Vec<TraderMarketPnLPoint>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "tickSize")]
    pub tick_size: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderPositionView {
    #[serde(rename = "accumulatedFunding")]
    pub accumulated_funding: String,
    #[serde(rename = "backstopMargin")]
    pub backstop_margin: String,
    #[serde(rename = "discountedUnrealizedPnl")]
    pub discounted_unrealized_pnl: String,
    #[serde(rename = "entryPrice")]
    pub entry_price: String,
    #[serde(rename = "initialMargin")]
    pub initial_margin: String,
    #[serde(rename = "limitOrderMargin")]
    pub limit_order_margin: String,
    #[serde(rename = "liquidationPrice")]
    pub liquidation_price: String,
    #[serde(rename = "maintenanceMargin")]
    pub maintenance_margin: String,
    #[serde(rename = "positionInitialMargin")]
    pub position_initial_margin: String,
    #[serde(rename = "positionSize")]
    pub position_size: String,
    #[serde(rename = "positionValue")]
    pub position_value: String,
    #[serde(rename = "stopLossPrice", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_price: Option<String>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "takeProfitPrice", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_price: Option<String>,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "unsettledFunding")]
    pub unsettled_funding: String,
    #[serde(rename = "virtualQuotePosition")]
    pub virtual_quote_position: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderStateCapabilities {
    #[serde(rename = "capabilities")]
    pub capabilities: TraderCapabilitiesView,
    #[serde(rename = "flags")]
    pub flags: TraderCapabilityFlags,
    #[serde(rename = "state")]
    pub state: TraderActivityStateView,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderStateLimitOrderEvent {
    #[serde(rename = "orders")]
    pub orders: Vec<TraderStateMarketLimitOrderEvent>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderStateMarketLimitOrderEvent {
    #[serde(rename = "change", default, skip_serializing_if = "Option::is_none")]
    pub change: Option<serde_json::Value>,
    #[serde(rename = "conditionalKind", default, skip_serializing_if = "Option::is_none")]
    pub conditional_kind: Option<String>,
    #[serde(rename = "initialSizeLots")]
    pub initial_size_lots: String,
    #[serde(rename = "isConditionalOrder", default, skip_serializing_if = "Option::is_none")]
    pub is_conditional_order: Option<bool>,
    #[serde(rename = "isStopLoss", default, skip_serializing_if = "Option::is_none")]
    pub is_stop_loss: Option<bool>,
    #[serde(rename = "isStopLossDirection", default, skip_serializing_if = "Option::is_none")]
    pub is_stop_loss_direction: Option<bool>,
    #[serde(rename = "orderSequenceNumber")]
    pub order_sequence_number: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "priceTicks")]
    pub price_ticks: String,
    #[serde(rename = "priceUsd")]
    pub price_usd: String,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "side")]
    pub side: Side,
    #[serde(rename = "sizeRemainingLots")]
    pub size_remaining_lots: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderStateSnapshot {
    #[serde(rename = "capabilities")]
    pub capabilities: TraderStateCapabilities,
    #[serde(rename = "makerFeeOverrideMultiplier")]
    pub maker_fee_override_multiplier: NativeNumber,
    #[serde(rename = "subaccounts")]
    pub subaccounts: Vec<TraderStateSubaccountSnapshot>,
    #[serde(rename = "takerFeeOverrideMultiplier")]
    pub taker_fee_override_multiplier: NativeNumber,
    #[serde(rename = "version")]
    pub version: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderStateSnapshotResponse {
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "slot")]
    pub slot: NativeNumber,
    #[serde(rename = "slotIndex")]
    pub slot_index: NativeNumber,
    #[serde(rename = "snapshot")]
    pub snapshot: TraderStateSnapshot,
    #[serde(rename = "traderPdaIndex")]
    pub trader_pda_index: NativeNumber,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderStateSubaccountSnapshot {
    #[serde(rename = "capabilities", default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[serde(rename = "collateral")]
    pub collateral: String,
    #[serde(rename = "cooldownStatus", default, skip_serializing_if = "Option::is_none")]
    pub cooldown_status: Option<serde_json::Value>,
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<TraderStateLimitOrderEvent>>,
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<TraderStatePositionSnapshot>>,
    #[serde(rename = "sequence")]
    pub sequence: NativeNumber,
    #[serde(rename = "splines", default, skip_serializing_if = "Option::is_none")]
    pub splines: Option<Vec<TraderStateSplineSnapshot>>,
    #[serde(rename = "subaccountIndex")]
    pub subaccount_index: NativeNumber,
    #[serde(rename = "triggers", default, skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<TraderStateTriggerSnapshot>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TraderView {
    #[serde(rename = "accumulatedFunding")]
    pub accumulated_funding: String,
    #[serde(rename = "authority")]
    pub authority: String,
    #[serde(rename = "cancelMargin")]
    pub cancel_margin: String,
    #[serde(rename = "capabilities")]
    pub capabilities: TraderCapabilitiesView,
    #[serde(rename = "collateralBalance")]
    pub collateral_balance: String,
    #[serde(rename = "discountedUnrealizedPnl")]
    pub discounted_unrealized_pnl: String,
    #[serde(rename = "effectiveCollateral")]
    pub effective_collateral: String,
    #[serde(rename = "effectiveCollateralForWithdrawals")]
    pub effective_collateral_for_withdrawals: String,
    #[serde(rename = "flags")]
    pub flags: TraderCapabilityFlags,
    #[serde(rename = "initialMargin")]
    pub initial_margin: String,
    #[serde(rename = "initialMarginForWithdrawals")]
    pub initial_margin_for_withdrawals: String,
    #[serde(rename = "isInActiveTraders")]
    pub is_in_active_traders: bool,
    #[serde(rename = "lastDepositSlot")]
    pub last_deposit_slot: NativeNumber,
    #[serde(rename = "limitOrders")]
    pub limit_orders: std::collections::BTreeMap<String,Vec<LimitOrder>>,
    #[serde(rename = "maintenanceMargin")]
    pub maintenance_margin: String,
    #[serde(rename = "makerFeeOverrideMultiplier")]
    pub maker_fee_override_multiplier: NativeNumber,
    #[serde(rename = "maxPositions")]
    pub max_positions: NativeNumber,
    #[serde(rename = "numMarketsWithSplines")]
    pub num_markets_with_splines: NativeNumber,
    #[serde(rename = "portfolioValue")]
    pub portfolio_value: String,
    #[serde(rename = "positions")]
    pub positions: Vec<TraderPositionView>,
    #[serde(rename = "riskState")]
    pub risk_state: RiskState,
    #[serde(rename = "riskTier")]
    pub risk_tier: RiskTier,
    #[serde(rename = "slot")]
    pub slot: NativeNumber,
    #[serde(rename = "slotIndex")]
    pub slot_index: NativeNumber,
    #[serde(rename = "state")]
    pub state: TraderActivityStateView,
    #[serde(rename = "takerFeeOverrideMultiplier")]
    pub taker_fee_override_multiplier: NativeNumber,
    #[serde(rename = "traderKey")]
    pub trader_key: String,
    #[serde(rename = "traderPdaIndex")]
    pub trader_pda_index: NativeNumber,
    #[serde(rename = "traderSubaccountIndex")]
    pub trader_subaccount_index: NativeNumber,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: String,
    #[serde(rename = "unsettledFundingOwed")]
    pub unsettled_funding_owed: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ValidateInviteParams {
    #[serde(rename = "body")]
    pub body: ValidateInviteRequest,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ValidateInviteRequest {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "transaction_signature", default, skip_serializing_if = "Option::is_none")]
    pub transaction_signature: Option<String>,
    #[serde(rename = "wallet_address")]
    pub wallet_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ValidateInviteResponse {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "whitelisted")]
    pub whitelisted: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WalletLoginRequest {
    #[serde(rename = "nonce_id")]
    pub nonce_id: String,
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "wallet_pubkey")]
    pub wallet_pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WalletNonceResponse {
    #[serde(rename = "expires_at")]
    pub expires_at: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "nonce_id")]
    pub nonce_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WalletTransactionChallengeRequest {
    #[serde(rename = "wallet_pubkey")]
    pub wallet_pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WalletTransactionChallengeResponse {
    #[serde(rename = "expires_at")]
    pub expires_at: String,
    #[serde(rename = "nonce_id")]
    pub nonce_id: String,
    #[serde(rename = "unsigned_transaction")]
    pub unsigned_transaction: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WalletTransactionLoginRequest {
    #[serde(rename = "nonce_id")]
    pub nonce_id: String,
    #[serde(rename = "signed_transaction")]
    pub signed_transaction: String,
    #[serde(rename = "wallet_pubkey")]
    pub wallet_pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedPhoenixClient(PhoenixClient);
impl PhoenixClient {
    pub fn typed(self) -> TypedPhoenixClient { TypedPhoenixClient(self) }
}
impl TypedPhoenixClient {
    pub fn acknowledge_notifications(&self, request: &AcknowledgeNotificationsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.acknowledge_notifications(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn acknowledge_notifications_up_to_timestamp(&self, request: &AcknowledgeNotificationsUpToTimestampParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.acknowledge_notifications_up_to_timestamp(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn activate_referral(&self, request: &ActivateReferralParams) -> Result<ActivateInviteResponse> {
        serde_json::from_value(self.0.activate_referral(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn activate_referral_with_user_funded_transaction(&self, request: &ActivateReferralWithUserFundedTransactionParams) -> Result<ActivateReferralTxResponse> {
        serde_json::from_value(self.0.activate_referral_with_user_funded_transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn build_attached_conditional_order_transaction(&self, request: &BuildAttachedConditionalOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_attached_conditional_order_transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn build_cancel_conditional_order_transaction(&self, request: &BuildCancelConditionalOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_cancel_conditional_order_transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn build_cancel_stop_loss_order_transaction(&self, request: &BuildCancelStopLossOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_cancel_stop_loss_order_transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn build_isolated_limit_order_transaction(&self, request: &BuildIsolatedLimitOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_isolated_limit_order_transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn build_isolated_limit_order_transaction_with_conditionals(&self, request: &BuildIsolatedLimitOrderTransactionWithConditionalsParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_isolated_limit_order_transaction_with_conditionals(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn build_isolated_limit_order_transaction_with_liquidation_estimate(&self, request: &BuildIsolatedLimitOrderTransactionWithLiquidationEstimateParams) -> Result<PlaceIsolatedLimitOrderEnhancedResponse> {
        serde_json::from_value(self.0.build_isolated_limit_order_transaction_with_liquidation_estimate(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn build_isolated_market_order_transaction(&self, request: &BuildIsolatedMarketOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_isolated_market_order_transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn build_isolated_market_order_transaction_with_liquidation_estimate(&self, request: &BuildIsolatedMarketOrderTransactionWithLiquidationEstimateParams) -> Result<PlaceIsolatedMarketOrderEnhancedResponse> {
        serde_json::from_value(self.0.build_isolated_market_order_transaction_with_liquidation_estimate(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn build_position_conditional_order_transaction(&self, request: &BuildPositionConditionalOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_position_conditional_order_transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn build_register_trader_instructions(&self, request: &BuildRegisterTraderInstructionsParams) -> Result<BuildRegisterIxsResponse> {
        serde_json::from_value(self.0.build_register_trader_instructions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn build_stop_loss_order_transaction(&self, request: &BuildStopLossOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_stop_loss_order_transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn check_wallet_invite_status(&self, request: &CheckWalletInviteStatusParams) -> Result<CheckWalletResponse> {
        serde_json::from_value(self.0.check_wallet_invite_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn create_service_login_challenge(&self, request: &CreateServiceLoginChallengeParams) -> Result<ServiceChallengeResponse> {
        serde_json::from_value(self.0.create_service_login_challenge(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn create_wallet_transaction_challenge(&self, request: &CreateWalletTransactionChallengeParams) -> Result<WalletTransactionChallengeResponse> {
        serde_json::from_value(self.0.create_wallet_transaction_challenge(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_candles(&self, request: &GetCandlesParams) -> Result<Vec<ApiCandle>> {
        serde_json::from_value(self.0.get_candles(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_commodity_market_calendar(&self, request: &GetCommodityMarketCalendarParams) -> Result<CommodityMarketCalendarResponse> {
        serde_json::from_value(self.0.get_commodity_market_calendar(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_exchange(&self, request: &GetExchangeParams) -> Result<ExchangeView> {
        serde_json::from_value(self.0.get_exchange(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_exchange_keys(&self, request: &GetExchangeKeysParams) -> Result<ExchangeKeysView> {
        serde_json::from_value(self.0.get_exchange_keys(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_exchange_snapshot(&self, request: &GetExchangeSnapshotParams) -> Result<ExchangeSnapshotView> {
        serde_json::from_value(self.0.get_exchange_snapshot(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_exchange_status(&self, request: &GetExchangeStatusParams) -> Result<ExchangeStatusView> {
        serde_json::from_value(self.0.get_exchange_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_funding_overview(&self, request: &GetFundingOverviewParams) -> Result<FundingOverviewResponse> {
        serde_json::from_value(self.0.get_funding_overview(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_funding_rate_history(&self, request: &GetFundingRateHistoryParams) -> Result<FundingRateHistoryResponse> {
        serde_json::from_value(self.0.get_funding_rate_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_jwks(&self, request: &GetJwksParams) -> Result<JwksResponse> {
        serde_json::from_value(self.0.get_jwks(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_mark_price(&self, request: &GetMarkPriceParams) -> Result<MarkPriceResponse> {
        serde_json::from_value(self.0.get_mark_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_market(&self, request: &GetMarketParams) -> Result<ExchangeMarketConfig> {
        serde_json::from_value(self.0.get_market(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_market_calendar(&self, request: &GetMarketCalendarParams) -> Result<MarketCalendarResponse> {
        serde_json::from_value(self.0.get_market_calendar(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_market_calendar_by_id(&self, request: &GetMarketCalendarByIdParams) -> Result<MarketCalendarRecord> {
        serde_json::from_value(self.0.get_market_calendar_by_id(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_market_fills(&self, request: &GetMarketFillsParams) -> Result<FillsResponse> {
        serde_json::from_value(self.0.get_market_fills(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_market_stats_history(&self, request: &GetMarketStatsHistoryParams) -> Result<MarketStatsHistoryResponse> {
        serde_json::from_value(self.0.get_market_stats_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_next_commodity_market_transition(&self, request: &GetNextCommodityMarketTransitionParams) -> Result<NextCommodityMarketTransition> {
        serde_json::from_value(self.0.get_next_commodity_market_transition(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_next_market_calendar_transition(&self, request: &GetNextMarketCalendarTransitionParams) -> Result<NextMarketCalendarTransition> {
        serde_json::from_value(self.0.get_next_market_calendar_transition(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_order_history(&self, request: &GetOrderHistoryParams) -> Result<PaginatedResponseVecOrderHistoryItem> {
        serde_json::from_value(self.0.get_order_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_order_history_v2(&self, request: &GetOrderHistoryV2Params) -> Result<PaginatedResponseVecOrderHistoryV2Item> {
        serde_json::from_value(self.0.get_order_history_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_orderbook(&self, request: &GetOrderbookParams) -> Result<OrderbookView> {
        serde_json::from_value(self.0.get_orderbook(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_referral_activation_permission(&self, request: &GetReferralActivationPermissionParams) -> Result<ReferralActivationPermissionResponse> {
        serde_json::from_value(self.0.get_referral_activation_permission(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_trade_history(&self, request: &GetTradeHistoryParams) -> Result<PaginatedResponseVecTradeHistoryV2Item> {
        serde_json::from_value(self.0.get_trade_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_trader(&self, request: &GetTraderParams) -> Result<TraderView> {
        serde_json::from_value(self.0.get_trader(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_trader_capabilities(&self, request: &GetTraderCapabilitiesParams) -> Result<TraderCapabilitiesMetadata> {
        serde_json::from_value(self.0.get_trader_capabilities(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_trader_collateral_history(&self, request: &GetTraderCollateralHistoryParams) -> Result<CollateralHistoryResponse> {
        serde_json::from_value(self.0.get_trader_collateral_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_trader_funding_history(&self, request: &GetTraderFundingHistoryParams) -> Result<TraderFundingHistoryResponse> {
        serde_json::from_value(self.0.get_trader_funding_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_trader_market_pn_l(&self, request: &GetTraderMarketPnLParams) -> Result<Vec<TraderMarketPnLSeries>> {
        serde_json::from_value(self.0.get_trader_market_pn_l(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_trader_pn_l(&self, request: &GetTraderPnLParams) -> Result<Vec<PnLPoint>> {
        serde_json::from_value(self.0.get_trader_pn_l(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_trader_portfolio_values(&self, request: &GetTraderPortfolioValuesParams) -> Result<Vec<PortfolioValuePoint>> {
        serde_json::from_value(self.0.get_trader_portfolio_values(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_trader_state(&self, request: &GetTraderStateParams) -> Result<TraderStateSnapshotResponse> {
        serde_json::from_value(self.0.get_trader_state(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_trader_trade_history_v2(&self, request: &GetTraderTradeHistoryV2Params) -> Result<PaginatedResponseVecTradeHistoryV2Item> {
        serde_json::from_value(self.0.get_trader_trade_history_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_user_collateral_history(&self, request: &GetUserCollateralHistoryParams) -> Result<CollateralHistoryResponse> {
        serde_json::from_value(self.0.get_user_collateral_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_user_hourly_funding(&self, request: &GetUserHourlyFundingParams) -> Result<FundingHourlyHistoryResponse> {
        serde_json::from_value(self.0.get_user_hourly_funding(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_user_liquidation_history(&self, request: &GetUserLiquidationHistoryParams) -> Result<PaginatedResponseVecUserLiquidationHistoryPoint> {
        serde_json::from_value(self.0.get_user_liquidation_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_user_pn_l(&self, request: &GetUserPnLParams) -> Result<Vec<PnLPoint>> {
        serde_json::from_value(self.0.get_user_pn_l(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_user_trade_history_v2(&self, request: &GetUserTradeHistoryV2Params) -> Result<PaginatedResponseVecTradeHistoryV2Item> {
        serde_json::from_value(self.0.get_user_trade_history_v2(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_v1_traders_by_trader_pubkey_collateral_history(&self, request: &GetV1TradersByTraderPubkeyCollateralHistoryParams) -> Result<CollateralHistoryResponse> {
        serde_json::from_value(self.0.get_v1_traders_by_trader_pubkey_collateral_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn get_wallet_login_nonce(&self, request: &GetWalletLoginNonceParams) -> Result<WalletNonceResponse> {
        serde_json::from_value(self.0.get_wallet_login_nonce(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn list_markets(&self, request: &ListMarketsParams) -> Result<Vec<ExchangeMarketConfig>> {
        serde_json::from_value(self.0.list_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn list_trader_notifications(&self, request: &ListTraderNotificationsParams) -> Result<GetNotificationsResponse> {
        serde_json::from_value(self.0.list_trader_notifications(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn log_in_with_privy_token(&self, request: &LogInWithPrivyTokenParams) -> Result<AuthResponse> {
        serde_json::from_value(self.0.log_in_with_privy_token(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn log_in_with_service_signature(&self, request: &LogInWithServiceSignatureParams) -> Result<AuthResponse> {
        serde_json::from_value(self.0.log_in_with_service_signature(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn log_in_with_signed_wallet_transaction(&self, request: &LogInWithSignedWalletTransactionParams) -> Result<AuthResponse> {
        serde_json::from_value(self.0.log_in_with_signed_wallet_transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn log_in_with_wallet_signature(&self, request: &LogInWithWalletSignatureParams) -> Result<AuthResponse> {
        serde_json::from_value(self.0.log_in_with_wallet_signature(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn log_out(&self, request: &LogOutParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.log_out(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn refresh_session(&self, request: &RefreshSessionParams) -> Result<AuthResponse> {
        serde_json::from_value(self.0.refresh_session(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn submit_register_trader_transaction(&self, request: &SubmitRegisterTraderTransactionParams) -> Result<SendRegisterIxsResponse> {
        serde_json::from_value(self.0.submit_register_trader_transaction(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub fn validate_invite(&self, request: &ValidateInviteParams) -> Result<ValidateInviteResponse> {
        serde_json::from_value(self.0.validate_invite(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedPhoenixClient(crate::AsyncClient<PhoenixClient>);
#[cfg(feature = "async")]
impl TypedPhoenixClient {
    pub fn into_async(self) -> AsyncTypedPhoenixClient { AsyncTypedPhoenixClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedPhoenixClient {
    pub async fn acknowledge_notifications(&self, request: &AcknowledgeNotificationsParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.acknowledge_notifications(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn acknowledge_notifications_up_to_timestamp(&self, request: &AcknowledgeNotificationsUpToTimestampParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.acknowledge_notifications_up_to_timestamp(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn activate_referral(&self, request: &ActivateReferralParams) -> Result<ActivateInviteResponse> {
        serde_json::from_value(self.0.activate_referral(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn activate_referral_with_user_funded_transaction(&self, request: &ActivateReferralWithUserFundedTransactionParams) -> Result<ActivateReferralTxResponse> {
        serde_json::from_value(self.0.activate_referral_with_user_funded_transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn build_attached_conditional_order_transaction(&self, request: &BuildAttachedConditionalOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_attached_conditional_order_transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn build_cancel_conditional_order_transaction(&self, request: &BuildCancelConditionalOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_cancel_conditional_order_transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn build_cancel_stop_loss_order_transaction(&self, request: &BuildCancelStopLossOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_cancel_stop_loss_order_transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn build_isolated_limit_order_transaction(&self, request: &BuildIsolatedLimitOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_isolated_limit_order_transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn build_isolated_limit_order_transaction_with_conditionals(&self, request: &BuildIsolatedLimitOrderTransactionWithConditionalsParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_isolated_limit_order_transaction_with_conditionals(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn build_isolated_limit_order_transaction_with_liquidation_estimate(&self, request: &BuildIsolatedLimitOrderTransactionWithLiquidationEstimateParams) -> Result<PlaceIsolatedLimitOrderEnhancedResponse> {
        serde_json::from_value(self.0.build_isolated_limit_order_transaction_with_liquidation_estimate(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn build_isolated_market_order_transaction(&self, request: &BuildIsolatedMarketOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_isolated_market_order_transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn build_isolated_market_order_transaction_with_liquidation_estimate(&self, request: &BuildIsolatedMarketOrderTransactionWithLiquidationEstimateParams) -> Result<PlaceIsolatedMarketOrderEnhancedResponse> {
        serde_json::from_value(self.0.build_isolated_market_order_transaction_with_liquidation_estimate(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn build_position_conditional_order_transaction(&self, request: &BuildPositionConditionalOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_position_conditional_order_transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn build_register_trader_instructions(&self, request: &BuildRegisterTraderInstructionsParams) -> Result<BuildRegisterIxsResponse> {
        serde_json::from_value(self.0.build_register_trader_instructions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn build_stop_loss_order_transaction(&self, request: &BuildStopLossOrderTransactionParams) -> Result<Vec<ApiInstructionResponse>> {
        serde_json::from_value(self.0.build_stop_loss_order_transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn check_wallet_invite_status(&self, request: &CheckWalletInviteStatusParams) -> Result<CheckWalletResponse> {
        serde_json::from_value(self.0.check_wallet_invite_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn create_service_login_challenge(&self, request: &CreateServiceLoginChallengeParams) -> Result<ServiceChallengeResponse> {
        serde_json::from_value(self.0.create_service_login_challenge(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn create_wallet_transaction_challenge(&self, request: &CreateWalletTransactionChallengeParams) -> Result<WalletTransactionChallengeResponse> {
        serde_json::from_value(self.0.create_wallet_transaction_challenge(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_candles(&self, request: &GetCandlesParams) -> Result<Vec<ApiCandle>> {
        serde_json::from_value(self.0.get_candles(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_commodity_market_calendar(&self, request: &GetCommodityMarketCalendarParams) -> Result<CommodityMarketCalendarResponse> {
        serde_json::from_value(self.0.get_commodity_market_calendar(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_exchange(&self, request: &GetExchangeParams) -> Result<ExchangeView> {
        serde_json::from_value(self.0.get_exchange(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_exchange_keys(&self, request: &GetExchangeKeysParams) -> Result<ExchangeKeysView> {
        serde_json::from_value(self.0.get_exchange_keys(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_exchange_snapshot(&self, request: &GetExchangeSnapshotParams) -> Result<ExchangeSnapshotView> {
        serde_json::from_value(self.0.get_exchange_snapshot(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_exchange_status(&self, request: &GetExchangeStatusParams) -> Result<ExchangeStatusView> {
        serde_json::from_value(self.0.get_exchange_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_funding_overview(&self, request: &GetFundingOverviewParams) -> Result<FundingOverviewResponse> {
        serde_json::from_value(self.0.get_funding_overview(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_funding_rate_history(&self, request: &GetFundingRateHistoryParams) -> Result<FundingRateHistoryResponse> {
        serde_json::from_value(self.0.get_funding_rate_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_jwks(&self, request: &GetJwksParams) -> Result<JwksResponse> {
        serde_json::from_value(self.0.get_jwks(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_mark_price(&self, request: &GetMarkPriceParams) -> Result<MarkPriceResponse> {
        serde_json::from_value(self.0.get_mark_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_market(&self, request: &GetMarketParams) -> Result<ExchangeMarketConfig> {
        serde_json::from_value(self.0.get_market(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_market_calendar(&self, request: &GetMarketCalendarParams) -> Result<MarketCalendarResponse> {
        serde_json::from_value(self.0.get_market_calendar(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_market_calendar_by_id(&self, request: &GetMarketCalendarByIdParams) -> Result<MarketCalendarRecord> {
        serde_json::from_value(self.0.get_market_calendar_by_id(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_market_fills(&self, request: &GetMarketFillsParams) -> Result<FillsResponse> {
        serde_json::from_value(self.0.get_market_fills(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_market_stats_history(&self, request: &GetMarketStatsHistoryParams) -> Result<MarketStatsHistoryResponse> {
        serde_json::from_value(self.0.get_market_stats_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_next_commodity_market_transition(&self, request: &GetNextCommodityMarketTransitionParams) -> Result<NextCommodityMarketTransition> {
        serde_json::from_value(self.0.get_next_commodity_market_transition(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_next_market_calendar_transition(&self, request: &GetNextMarketCalendarTransitionParams) -> Result<NextMarketCalendarTransition> {
        serde_json::from_value(self.0.get_next_market_calendar_transition(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_order_history(&self, request: &GetOrderHistoryParams) -> Result<PaginatedResponseVecOrderHistoryItem> {
        serde_json::from_value(self.0.get_order_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_order_history_v2(&self, request: &GetOrderHistoryV2Params) -> Result<PaginatedResponseVecOrderHistoryV2Item> {
        serde_json::from_value(self.0.get_order_history_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_orderbook(&self, request: &GetOrderbookParams) -> Result<OrderbookView> {
        serde_json::from_value(self.0.get_orderbook(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_referral_activation_permission(&self, request: &GetReferralActivationPermissionParams) -> Result<ReferralActivationPermissionResponse> {
        serde_json::from_value(self.0.get_referral_activation_permission(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_trade_history(&self, request: &GetTradeHistoryParams) -> Result<PaginatedResponseVecTradeHistoryV2Item> {
        serde_json::from_value(self.0.get_trade_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_trader(&self, request: &GetTraderParams) -> Result<TraderView> {
        serde_json::from_value(self.0.get_trader(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_trader_capabilities(&self, request: &GetTraderCapabilitiesParams) -> Result<TraderCapabilitiesMetadata> {
        serde_json::from_value(self.0.get_trader_capabilities(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_trader_collateral_history(&self, request: &GetTraderCollateralHistoryParams) -> Result<CollateralHistoryResponse> {
        serde_json::from_value(self.0.get_trader_collateral_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_trader_funding_history(&self, request: &GetTraderFundingHistoryParams) -> Result<TraderFundingHistoryResponse> {
        serde_json::from_value(self.0.get_trader_funding_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_trader_market_pn_l(&self, request: &GetTraderMarketPnLParams) -> Result<Vec<TraderMarketPnLSeries>> {
        serde_json::from_value(self.0.get_trader_market_pn_l(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_trader_pn_l(&self, request: &GetTraderPnLParams) -> Result<Vec<PnLPoint>> {
        serde_json::from_value(self.0.get_trader_pn_l(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_trader_portfolio_values(&self, request: &GetTraderPortfolioValuesParams) -> Result<Vec<PortfolioValuePoint>> {
        serde_json::from_value(self.0.get_trader_portfolio_values(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_trader_state(&self, request: &GetTraderStateParams) -> Result<TraderStateSnapshotResponse> {
        serde_json::from_value(self.0.get_trader_state(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_trader_trade_history_v2(&self, request: &GetTraderTradeHistoryV2Params) -> Result<PaginatedResponseVecTradeHistoryV2Item> {
        serde_json::from_value(self.0.get_trader_trade_history_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_user_collateral_history(&self, request: &GetUserCollateralHistoryParams) -> Result<CollateralHistoryResponse> {
        serde_json::from_value(self.0.get_user_collateral_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_user_hourly_funding(&self, request: &GetUserHourlyFundingParams) -> Result<FundingHourlyHistoryResponse> {
        serde_json::from_value(self.0.get_user_hourly_funding(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_user_liquidation_history(&self, request: &GetUserLiquidationHistoryParams) -> Result<PaginatedResponseVecUserLiquidationHistoryPoint> {
        serde_json::from_value(self.0.get_user_liquidation_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_user_pn_l(&self, request: &GetUserPnLParams) -> Result<Vec<PnLPoint>> {
        serde_json::from_value(self.0.get_user_pn_l(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_user_trade_history_v2(&self, request: &GetUserTradeHistoryV2Params) -> Result<PaginatedResponseVecTradeHistoryV2Item> {
        serde_json::from_value(self.0.get_user_trade_history_v2(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_v1_traders_by_trader_pubkey_collateral_history(&self, request: &GetV1TradersByTraderPubkeyCollateralHistoryParams) -> Result<CollateralHistoryResponse> {
        serde_json::from_value(self.0.get_v1_traders_by_trader_pubkey_collateral_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn get_wallet_login_nonce(&self, request: &GetWalletLoginNonceParams) -> Result<WalletNonceResponse> {
        serde_json::from_value(self.0.get_wallet_login_nonce(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn list_markets(&self, request: &ListMarketsParams) -> Result<Vec<ExchangeMarketConfig>> {
        serde_json::from_value(self.0.list_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn list_trader_notifications(&self, request: &ListTraderNotificationsParams) -> Result<GetNotificationsResponse> {
        serde_json::from_value(self.0.list_trader_notifications(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn log_in_with_privy_token(&self, request: &LogInWithPrivyTokenParams) -> Result<AuthResponse> {
        serde_json::from_value(self.0.log_in_with_privy_token(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn log_in_with_service_signature(&self, request: &LogInWithServiceSignatureParams) -> Result<AuthResponse> {
        serde_json::from_value(self.0.log_in_with_service_signature(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn log_in_with_signed_wallet_transaction(&self, request: &LogInWithSignedWalletTransactionParams) -> Result<AuthResponse> {
        serde_json::from_value(self.0.log_in_with_signed_wallet_transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn log_in_with_wallet_signature(&self, request: &LogInWithWalletSignatureParams) -> Result<AuthResponse> {
        serde_json::from_value(self.0.log_in_with_wallet_signature(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn log_out(&self, request: &LogOutParams) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.log_out(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn refresh_session(&self, request: &RefreshSessionParams) -> Result<AuthResponse> {
        serde_json::from_value(self.0.refresh_session(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn submit_register_trader_transaction(&self, request: &SubmitRegisterTraderTransactionParams) -> Result<SendRegisterIxsResponse> {
        serde_json::from_value(self.0.submit_register_trader_transaction(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
    pub async fn validate_invite(&self, request: &ValidateInviteParams) -> Result<ValidateInviteResponse> {
        serde_json::from_value(self.0.validate_invite(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Phoenix response model"))
    }
}
