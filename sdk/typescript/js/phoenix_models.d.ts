// Generated from src/exchanges/phoenix/schema/phoenix.json.
import type { HttpValue } from "./index";
export type AckNotificationItem = HttpValue;
export type ActivateReferralTxStatus = string;
export type CommodityMarketStateView = string;
export type ExchangeWsValidationRule = string;
export type JsSafeU64 = number | bigint | string;
export type LiquidityRole = string;
export type MarketCalendarStateView = string;
export type MarketStatus = string;
export type NotificationItem = HttpValue;
export type OrderStatus = string;
export type PlaceStopLossOrderRequest = HttpValue;
export type RiskState = string;
export type RiskTier = string;
export type Side = string;
export type TradeType = string;
export type TraderActivityStateView = string;
export type TraderCapabilityFlags = number | bigint | string;
export type TraderStatePositionSnapshot = HttpValue;
export type TraderStateSplineSnapshot = HttpValue;
export type TraderStateTriggerSnapshot = HttpValue;
export type UserLiquidationHistoryKind = string;
export type UserLiquidationHistoryRole = string;
export type UserLiquidationHistoryType = string;

export interface AckBeforeTimestampBody {
  [field: string]: unknown;
  "beforeTimestamp": string;
}

export interface AckNotificationsBody {
  [field: string]: unknown;
  "items": Array<AckNotificationItem>;
}

export interface AcknowledgeNotificationsParams {
  [field: string]: unknown;
  "trader_pubkey": string;
  "body": AckNotificationsBody;
}

export interface AcknowledgeNotificationsUpToTimestampParams {
  [field: string]: unknown;
  "trader_pubkey": string;
  "body": AckBeforeTimestampBody;
}

export interface ActivateInviteResponse {
  [field: string]: unknown;
  "trader_pda": string;
}

export interface ActivateInviteWithReferralRequest {
  [field: string]: unknown;
  "authority": string;
  "referral_code": string;
}

export interface ActivateReferralParams {
  [field: string]: unknown;
  "body": ActivateInviteWithReferralRequest;
}

export interface ActivateReferralTxRequest {
  [field: string]: unknown;
  "recent_blockhash": string;
  "referral_code": string;
  "trader_authority": string;
  "trader_pda_index"?: number | bigint | string | null;
  "trader_subaccount_index"?: number | bigint | string | null;
  "transaction": string;
}

export interface ActivateReferralTxResponse {
  [field: string]: unknown;
  "referral_code": string;
  "signature"?: string | null;
  "status": ActivateReferralTxStatus;
  "trader_pda": string;
}

export interface ActivateReferralWithUserFundedTransactionParams {
  [field: string]: unknown;
  "body": ActivateReferralTxRequest;
}

export interface ApiAccountMeta {
  [field: string]: unknown;
  "isSigner": boolean;
  "isWritable": boolean;
  "pubkey": string;
}

export interface ApiCandle {
  [field: string]: unknown;
  "close": number | bigint | string;
  "externalSource"?: string | null;
  "high": number | bigint | string;
  "low": number | bigint | string;
  "markClose"?: number | bigint | string | null;
  "markHigh"?: number | bigint | string | null;
  "markLow"?: number | bigint | string | null;
  "markOpen"?: number | bigint | string | null;
  "open": number | bigint | string;
  "time": number | bigint | string;
  "tradeCount"?: number | bigint | string | null;
  "volume"?: number | bigint | string | null;
  "volumeQuote"?: number | bigint | string | null;
}

export interface ApiInstructionResponse {
  [field: string]: unknown;
  "data": Array<number | bigint | string>;
  "keys": Array<ApiAccountMeta>;
  "programId": string;
}

export interface AuthResponse {
  [field: string]: unknown;
  "access_token": string;
  "expires_in": number | bigint | string;
  "pop_key": string;
  "refresh_expires_in": number | bigint | string;
  "refresh_token": string;
  "token_type": string;
}

export interface AuthoritySetView {
  [field: string]: unknown;
  "adlAuthority": string;
  "backstopAuthority": string;
  "cancelAuthority": string;
  "marketAuthority": string;
  "oracleAuthority": string;
  "riskAuthority": string;
  "rootAuthority": string;
}

export interface BuildAttachedConditionalOrderTransactionParams {
  [field: string]: unknown;
  "body": PlaceAttachedConditionalOrderRequest;
}

export interface BuildCancelConditionalOrderTransactionParams {
  [field: string]: unknown;
  "body": CancelConditionalOrderRequest;
}

export interface BuildCancelStopLossOrderTransactionParams {
  [field: string]: unknown;
  "body": CancelStopLossOrderRequest;
}

export interface BuildIsolatedLimitOrderTransactionParams {
  [field: string]: unknown;
  "body": PlaceIsolatedLimitOrderRequest;
}

export interface BuildIsolatedLimitOrderTransactionWithConditionalsParams {
  [field: string]: unknown;
  "body": PlaceIsolatedLimitOrderWithConditionalsRequest;
}

export interface BuildIsolatedLimitOrderTransactionWithLiquidationEstimateParams {
  [field: string]: unknown;
  "body": PlaceIsolatedLimitOrderRequest;
}

export interface BuildIsolatedMarketOrderTransactionParams {
  [field: string]: unknown;
  "body": PlaceIsolatedMarketOrderRequest;
}

export interface BuildIsolatedMarketOrderTransactionWithLiquidationEstimateParams {
  [field: string]: unknown;
  "body": PlaceIsolatedMarketOrderRequest;
}

export interface BuildPositionConditionalOrderTransactionParams {
  [field: string]: unknown;
  "body": PlacePositionConditionalOrderRequest;
}

export interface BuildRegisterIxsRequest {
  [field: string]: unknown;
  "maxPositions"?: number | bigint | string | null;
  "traderAuthority": string;
  "txFeePayer": string;
}

export interface BuildRegisterIxsResponse {
  [field: string]: unknown;
  "includeRegisterTrader": boolean;
  "instructions": Array<ApiInstructionResponse>;
  "maxPositions": number | bigint | string;
  "traderOnboarder": string;
  "traderPda": string;
  "txFeePayer": string;
}

export interface BuildRegisterTraderInstructionsParams {
  [field: string]: unknown;
  "body": BuildRegisterIxsRequest;
}

export interface BuildStopLossOrderTransactionParams {
  [field: string]: unknown;
  "body": PlaceStopLossOrderRequest;
}

export interface CancelConditionalOrderRequest {
  [field: string]: unknown;
  "authority": string;
  "conditionalOrderIndex": number | bigint | string;
  "executionDirection": string;
  "isIsolated"?: boolean | null;
  "positionAuthority"?: string | null;
  "symbol": string;
  "traderPdaIndex": number | bigint | string;
  "traderSubaccountIndex"?: number | bigint | string | null;
}

export interface CancelStopLossOrderRequest {
  [field: string]: unknown;
  "authority": string;
  "executionDirection": string;
  "isIsolated"?: boolean | null;
  "positionAuthority"?: string | null;
  "symbol": string;
  "traderPdaIndex": number | bigint | string;
  "traderSubaccountIndex"?: number | bigint | string | null;
}

export interface CapabilityAccessView {
  [field: string]: unknown;
  "immediate": boolean;
  "viaColdActivation"?: boolean | null;
}

export interface CheckWalletInviteStatusParams {
  [field: string]: unknown;
  "wallet": string;
}

export interface CheckWalletResponse {
  [field: string]: unknown;
  "invite_code_used"?: string | null;
  "whitelisted": boolean;
  "whitelisted_at"?: string | null;
}

export interface CollateralEvent {
  [field: string]: unknown;
  "amount": number | bigint | string;
  "collateralAfter": number | bigint | string;
  "eventIndex": number | bigint | string;
  "eventType": string;
  "slot": number | bigint | string;
  "slotIndex": number | bigint | string;
  "timestamp": string;
  "traderPdaIndex": number | bigint | string;
  "traderSubaccountIndex": number | bigint | string;
}

export interface CollateralHistoryResponse {
  [field: string]: unknown;
  "data": Array<CollateralEvent>;
  "hasMore": boolean;
  "nextCursor"?: string | null;
  "prevCursor"?: string | null;
}

export interface CommodityMarketCalendarResponse {
  [field: string]: unknown;
  "calendar": CommodityMarketCalendarView;
  "loadedAt": string;
  "market": string;
  "rawJson": string;
}

export interface CommodityMarketCalendarView {
  [field: string]: unknown;
  "dateOverrides": Record<string, CommodityMarketDaySchedule>;
  "weeklySchedule": Record<string, CommodityMarketDaySchedule>;
}

export interface CommodityMarketDaySchedule {
  [field: string]: unknown;
  "sessions": Array<CommodityMarketSessionRange>;
}

export interface CommodityMarketSessionRange {
  [field: string]: unknown;
  "end": string;
  "mode": string;
  "start": string;
}

export interface CreateServiceLoginChallengeParams {
  [field: string]: unknown;
  "body": ServiceChallengeRequest;
}

export interface CreateWalletTransactionChallengeParams {
  [field: string]: unknown;
  "body": WalletTransactionChallengeRequest;
}

export interface ExchangeKeysView {
  [field: string]: unknown;
  "activeTraderBuffer": Array<string>;
  "canonicalMint": string;
  "currentAuthorities": AuthoritySetView;
  "globalConfig": string;
  "globalTraderIndex": Array<string>;
  "globalVault": string;
  "pendingAuthorities": AuthoritySetView;
  "perpAssetMap": string;
  "withdrawQueue": string;
}

export interface ExchangeLeverageTier {
  [field: string]: unknown;
  "limitOrderRiskFactor": number | bigint | string;
  "limitOrderRiskFactorBps"?: number | bigint | string | null;
  "maxLeverage": number | bigint | string;
  "maxSizeBaseLots": number | bigint | string;
}

export interface ExchangeMarketConfig {
  [field: string]: unknown;
  "assetId": number | bigint | string;
  "baseLotsDecimals": number | bigint | string;
  "commodityMetadata"?: HttpValue | null;
  "fundingIntervalSeconds": number | bigint | string;
  "fundingPeriodSeconds": number | bigint | string;
  "isolatedOnly": boolean;
  "leverageTiers": Array<ExchangeLeverageTier>;
  "makerFee": number | bigint | string;
  "marketPubkey": string;
  "marketStatus": MarketStatus;
  "maxFundingRatePerInterval": number | bigint | string;
  "maxFundingRatePerIntervalPercentage"?: number | bigint | string | null;
  "maxLiquidationSizeBaseLots": JsSafeU64;
  "metadata"?: HttpValue | null;
  "openInterestCapBaseLots": JsSafeU64;
  "riskFactors": ExchangeRiskFactors;
  "splinePubkey": string;
  "statsSnapshot"?: HttpValue | null;
  "symbol": string;
  "takerFee": number | bigint | string;
  "tickSize": number | bigint | string;
}

export interface ExchangeMarketSnapshot {
  [field: string]: unknown;
  "assetId": number | bigint | string;
  "baseLotsDecimals": number | bigint | string;
  "commodityMetadata"?: HttpValue | null;
  "fundingConfig": ExchangeWsFundingConfig;
  "isolatedOnly": boolean;
  "leverageTiers": Array<ExchangeWsLeverageTier>;
  "makerFee": number | bigint | string;
  "markPriceParameters": ExchangeWsMarkPriceParameters;
  "marketPubkey": string;
  "marketStatus": MarketStatus;
  "maxLiquidationSizeBaseLots": JsSafeU64;
  "metadata"?: HttpValue | null;
  "openInterestCapBaseLots": JsSafeU64;
  "riskFactors": ExchangeWsRiskFactors;
  "splinePubkey": string;
  "symbol": string;
  "takerFee": number | bigint | string;
  "tickSize": number | bigint | string;
}

export interface ExchangeRiskFactors {
  [field: string]: unknown;
  "backstop": number | bigint | string;
  "backstopBps"?: number | bigint | string | null;
  "cancelOrder": number | bigint | string;
  "cancelOrderBps"?: number | bigint | string | null;
  "highRisk": number | bigint | string;
  "highRiskBps"?: number | bigint | string | null;
  "maintenance": number | bigint | string;
  "maintenanceBps"?: number | bigint | string | null;
  "upnl": number | bigint | string;
  "upnlBps"?: number | bigint | string | null;
  "upnlForWithdrawals": number | bigint | string;
  "upnlForWithdrawalsBps"?: number | bigint | string | null;
}

export interface ExchangeSnapshotView {
  [field: string]: unknown;
  "exchange": ExchangeStateSnapshot;
  "markets": Array<ExchangeMarketSnapshot>;
  "sequenceNumber"?: HttpValue | null;
  "slot": number | bigint | string;
  "slotIndex": number | bigint | string;
  "version": number | bigint | string;
}

export interface ExchangeStateSnapshot {
  [field: string]: unknown;
  "active": boolean;
  "activeTraderBuffer": Array<string>;
  "canonicalMint": string;
  "currentAuthorities": AuthoritySetView;
  "exchangeStatusBits": number | bigint | string;
  "exchangeStatusFeatures": Array<string>;
  "gated": boolean;
  "globalConfig": string;
  "globalTraderIndex": Array<string>;
  "globalVault": string;
  "perpAssetMap": string;
  "programId": string;
  "usdcMint": string;
  "withdrawQueue": string;
  "withdrawalsAvailable"?: boolean | null;
}

export interface ExchangeStatusView {
  [field: string]: unknown;
  "active": boolean;
  "gated": boolean;
  "withdrawalsAvailable"?: boolean | null;
}

export interface ExchangeView {
  [field: string]: unknown;
  "keys": ExchangeKeysView;
  "markets": Array<ExchangeMarketConfig>;
}

export interface ExchangeWsFundingConfig {
  [field: string]: unknown;
  "fundingIntervalSeconds": number | bigint | string;
  "fundingPeriodSeconds": number | bigint | string;
  "maxFundingRatePerInterval": number | bigint | string;
}

export interface ExchangeWsLeverageTier {
  [field: string]: unknown;
  "limitOrderRiskFactor": number | bigint | string;
  "maxLeverage": number | bigint | string;
  "maxSizeBaseLots": JsSafeU64;
}

export interface ExchangeWsMarkPriceParameters {
  [field: string]: unknown;
  "adjustedExchangeSpotPriceWeight": JsSafeU64;
  "bookHardStaleMultiplier"?: number | bigint | string | null;
  "bookPriceRadius": JsSafeU64;
  "bookPriceStaleThreshold": JsSafeU64;
  "bookPriceWeight": JsSafeU64;
  "commoditiesAfterHoursRadius": JsSafeU64;
  "commoditiesAfterHoursRadiusBps": JsSafeU64;
  "emaDiffRadius": JsSafeU64;
  "emaPeriodSlots": JsSafeU64;
  "exchangePerpPriceWeight": JsSafeU64;
  "minOracleResponses": number | bigint | string;
  "oracleDivergenceRadius": number | bigint | string;
  "oracleHardStaleMultiplier"?: number | bigint | string | null;
  "perpPriceStaleThreshold": JsSafeU64;
  "riskActionPriceValidityRules": Array<Array<Array<ExchangeWsValidationRule>>>;
  "spotPriceStaleThreshold": JsSafeU64;
}

export interface ExchangeWsRiskFactors {
  [field: string]: unknown;
  "backstop": number | bigint | string;
  "backstopBps"?: number | bigint | string | null;
  "cancelOrder": number | bigint | string;
  "cancelOrderBps"?: number | bigint | string | null;
  "highRisk": number | bigint | string;
  "highRiskBps"?: number | bigint | string | null;
  "maintenance": number | bigint | string;
  "maintenanceBps"?: number | bigint | string | null;
  "upnl": number | bigint | string;
  "upnlBps"?: number | bigint | string | null;
  "upnlForWithdrawals": number | bigint | string;
  "upnlForWithdrawalsBps"?: number | bigint | string | null;
}

export interface FillRecord {
  [field: string]: unknown;
  "baseQty": string;
  "instructionType": string;
  "marketSymbol": string;
  "price": string;
  "quoteQty": string;
  "timestamp": string;
  "transactionSignature": string;
}

export interface FillsResponse {
  [field: string]: unknown;
  "data": Array<FillRecord>;
  "hasMore": boolean;
  "nextCursor"?: string | null;
  "prevCursor"?: string | null;
}

export interface FundingHourlyEvent {
  [field: string]: unknown;
  "fundingPayment": string;
  "fundingRatePercentage": string;
  "positionSide": string;
  "positionSize": string;
  "symbol": string;
  "timestamp": string;
}

export interface FundingHourlyHistoryResponse {
  [field: string]: unknown;
  "events": Array<FundingHourlyEvent>;
  "hasMore": boolean;
  "nextCursor"?: string | null;
  "prevCursor"?: string | null;
}

export interface FundingOverviewPoint {
  [field: string]: unknown;
  "fundingAmountPerUnit": string;
  "fundingRate": string;
  "markPrice": string;
  "timestamp": string;
}

export interface FundingOverviewResponse {
  [field: string]: unknown;
  "series": Array<FundingOverviewSeries>;
}

export interface FundingOverviewSeries {
  [field: string]: unknown;
  "marketId": number | bigint | string;
  "points": Array<FundingOverviewPoint>;
  "symbol": string;
}

export interface FundingRateHistoryResponse {
  [field: string]: unknown;
  "marketId": number | bigint | string;
  "rates": Array<FundingRatePoint>;
  "symbol": string;
}

export interface FundingRatePoint {
  [field: string]: unknown;
  "fundingRatePercentage": string;
  "timestamp": string;
}

export interface GetCandlesParams {
  [field: string]: unknown;
  "symbol": string;
  "timeframe": string;
  "startTime"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "enableExternalSource"?: boolean | null;
}

export interface GetCommodityMarketCalendarParams {
  [field: string]: unknown;
}

export interface GetExchangeKeysParams {
  [field: string]: unknown;
}

export interface GetExchangeParams {
  [field: string]: unknown;
}

export interface GetExchangeSnapshotParams {
  [field: string]: unknown;
}

export interface GetExchangeStatusParams {
  [field: string]: unknown;
}

export interface GetFundingOverviewParams {
  [field: string]: unknown;
  "startTime"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
  "perMarketLimit"?: number | bigint | string | null;
}

export interface GetFundingRateHistoryParams {
  [field: string]: unknown;
  "symbol": string;
  "startTime"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetJwksParams {
  [field: string]: unknown;
}

export interface GetMarkPriceParams {
  [field: string]: unknown;
  "symbol": string;
}

export interface GetMarketCalendarByIdParams {
  [field: string]: unknown;
  "market_calendar_id": string;
}

export interface GetMarketCalendarParams {
  [field: string]: unknown;
  "symbol": string;
}

export interface GetMarketFillsParams {
  [field: string]: unknown;
  "symbol": string;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "startTime"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
}

export interface GetMarketParams {
  [field: string]: unknown;
  "symbol": string;
}

export interface GetMarketStatsHistoryParams {
  [field: string]: unknown;
  "symbol": string;
  "start_time"?: string | null;
  "end_time"?: string | null;
  "limit"?: number | bigint | string | null;
  "timeframe"?: string | null;
}

export interface GetNextCommodityMarketTransitionParams {
  [field: string]: unknown;
}

export interface GetNextMarketCalendarTransitionParams {
  [field: string]: unknown;
  "symbol": string;
}

export interface GetNotificationsResponse {
  [field: string]: unknown;
  "items": Array<NotificationItem>;
  "nextCursor"?: string | null;
}

export interface GetOrderHistoryParams {
  [field: string]: unknown;
  "authority": string;
  "traderPdaIndex"?: number | bigint | string | null;
  "marketSymbol"?: string | null;
  "limit": number | bigint | string;
  "cursor"?: string | null;
  "privyId"?: string | null;
  "orderStatus"?: string | null;
}

export interface GetOrderHistoryV2Params {
  [field: string]: unknown;
  "market_symbol"?: string | null;
  "limit": number | bigint | string;
  "cursor"?: string | null;
  "trader_pubkey": string;
}

export interface GetOrderbookParams {
  [field: string]: unknown;
  "symbol": string;
  "include_splines"?: boolean | null;
  "bypass_execution_band"?: boolean | null;
}

export interface GetReferralActivationPermissionParams {
  [field: string]: unknown;
}

export interface GetTradeHistoryParams {
  [field: string]: unknown;
  "authority": string;
  "pdaIndex"?: number | bigint | string | null;
  "marketSymbol"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetTraderCapabilitiesParams {
  [field: string]: unknown;
}

export interface GetTraderCollateralHistoryParams {
  [field: string]: unknown;
  "authority": string;
  "pdaIndex"?: number | bigint | string | null;
  "nextCursor"?: string | null;
  "prevCursor"?: string | null;
  "cursor"?: string | null;
  "limit": number | bigint | string;
}

export interface GetTraderFundingHistoryParams {
  [field: string]: unknown;
  "authority": string;
  "traderPdaIndex"?: number | bigint | string | null;
  "symbol"?: string | null;
  "startTime"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "resolution"?: string | null;
}

export interface GetTraderMarketPnLParams {
  [field: string]: unknown;
  "trader_pubkey": string;
  "resolution": string;
  "startTime"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "symbols"?: string | null;
}

export interface GetTraderParams {
  [field: string]: unknown;
  "pubkey": string;
}

export interface GetTraderPnLParams {
  [field: string]: unknown;
  "trader_pubkey": string;
  "resolution": string;
  "startTime"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetTraderPortfolioValuesParams {
  [field: string]: unknown;
  "trader_pubkey": string;
  "resolution": string;
  "startTime"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetTraderStateParams {
  [field: string]: unknown;
  "traderPdaIndex"?: number | bigint | string | null;
  "authority_pubkey": string;
}

export interface GetTraderTradeHistoryV2Params {
  [field: string]: unknown;
  "market_symbol"?: string | null;
  "trader_pda_index"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "privy_id"?: string | null;
  "trader_pubkey": string;
}

export interface GetUserCollateralHistoryParams {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "nextCursor"?: string | null;
  "prevCursor"?: string | null;
  "cursor"?: string | null;
  "user_pubkey": string;
}

export interface GetUserHourlyFundingParams {
  [field: string]: unknown;
  "user_pubkey": string;
  "traderPdaIndex"?: number | bigint | string | null;
  "symbol"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetUserLiquidationHistoryParams {
  [field: string]: unknown;
  "pdaIndex"?: number | bigint | string | null;
  "subaccountIndex"?: number | bigint | string | null;
  "symbol"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "user_pubkey": string;
}

export interface GetUserPnLParams {
  [field: string]: unknown;
  "user_pubkey": string;
  "resolution": string;
  "startTime"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetUserTradeHistoryV2Params {
  [field: string]: unknown;
  "market_symbol"?: string | null;
  "trader_pda_index"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "privy_id"?: string | null;
  "user_pubkey": string;
}

export interface GetV1TradersByTraderPubkeyCollateralHistoryParams {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "nextCursor"?: string | null;
  "prevCursor"?: string | null;
  "cursor"?: string | null;
  "trader_pubkey": string;
}

export interface GetWalletLoginNonceParams {
  [field: string]: unknown;
  "wallet_pubkey": string;
}

export interface IntendedOrder {
  [field: string]: unknown;
  "baseLots": number | bigint | string;
  "baseQty": string;
  "cancelExisting": boolean;
  "clientOrderId"?: string | null;
  "hasPriceLimit": boolean;
  "isConditionalOrder"?: boolean | null;
  "isPostOnly": boolean;
  "isReduceOnly": boolean;
  "isStopLoss": boolean;
  "isStopLossDirection"?: boolean | null;
  "isTakeOnly": boolean;
  "lastValidSlot"?: number | bigint | string | null;
  "matchLimit"?: number | bigint | string | null;
  "orderFlags": number | bigint | string;
  "orderKind": string;
  "priceInTicks"?: number | bigint | string | null;
  "priceUsd"?: string | null;
  "quoteLotBudget"?: number | bigint | string | null;
  "side": string;
}

export interface JwksResponse {
  [field: string]: unknown;
  "keys": Array<HttpValue>;
}

export interface LimitOrder {
  [field: string]: unknown;
  "initialTradeSize": string;
  "isConditionalOrder"?: boolean | null;
  "isReduceOnly": boolean;
  "isStopLoss"?: boolean | null;
  "isStopLossDirection"?: boolean | null;
  "marginFactor": string;
  "marginRequirement": string;
  "orderSequenceNumber": string;
  "price": string;
  "side": Side;
  "tradeSizeRemaining": string;
}

export interface ListMarketsParams {
  [field: string]: unknown;
}

export interface ListTraderNotificationsParams {
  [field: string]: unknown;
  "trader_pubkey": string;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "unackedOnly"?: boolean | null;
}

export interface LogInWithPrivyTokenParams {
  [field: string]: unknown;
  "body": PrivyLoginRequest;
}

export interface LogInWithServiceSignatureParams {
  [field: string]: unknown;
  "body": ServiceLoginRequest;
}

export interface LogInWithSignedWalletTransactionParams {
  [field: string]: unknown;
  "body": WalletTransactionLoginRequest;
}

export interface LogInWithWalletSignatureParams {
  [field: string]: unknown;
  "body": WalletLoginRequest;
}

export interface LogOutParams {
  [field: string]: unknown;
}

export interface MarkPriceResponse {
  [field: string]: unknown;
  "markPrice"?: HttpValue | null;
  "slot": number | bigint | string;
  "slotIndex": number | bigint | string;
  "symbol": string;
}

export interface MarketCalendarRecord {
  [field: string]: unknown;
  "calendar"?: HttpValue | null;
  "calendarUri": string;
  "contentSha256": string;
  "description": string;
  "kind": string;
  "loadedAt": string;
  "marketCalendarId": string;
  "rawJson": string;
  "s3Path": string;
  "updatedAt": string;
}

export interface MarketCalendarResponse {
  [field: string]: unknown;
  "calendar"?: HttpValue | null;
  "calendarUri": string;
  "contentSha256": string;
  "description": string;
  "kind": string;
  "loadedAt": string;
  "market": string;
  "marketCalendarId": string;
  "rawJson": string;
}

export interface MarketStatsHistoryResponse {
  [field: string]: unknown;
  "market_id": number | bigint | string;
  "stats": Array<MarketStatsPoint>;
  "symbol": string;
  "timeframe"?: string | null;
}

export interface MarketStatsPoint {
  [field: string]: unknown;
  "mark_price": number | bigint | string;
  "open_interest": number | bigint | string;
  "slot": number | bigint | string;
  "spot_price": number | bigint | string;
  "timestamp": string;
  "total_maker_fees"?: number | bigint | string | null;
  "total_taker_fees"?: number | bigint | string | null;
}

export interface NextCommodityMarketTransition {
  [field: string]: unknown;
  "currentState": CommodityMarketStateView;
  "loadedAt": string;
  "market": string;
  "nextMarketState"?: HttpValue | null;
  "utcNextTransition"?: string | null;
}

export interface NextMarketCalendarTransition {
  [field: string]: unknown;
  "calendarUri": string;
  "currentState": MarketCalendarStateView;
  "loadedAt": string;
  "market": string;
  "marketCalendarId": string;
  "nextMarketState"?: HttpValue | null;
  "utcNextTransition"?: string | null;
}

export interface OrderbookView {
  [field: string]: unknown;
  "asks": Array<[number | bigint | string, number | bigint | string]>;
  "bids": Array<[number | bigint | string, number | bigint | string]>;
  "mid"?: number | bigint | string | null;
  "slot": number | bigint | string;
  "splines"?: Array<Spline> | null;
  "symbol": string;
}

export interface PaginatedResponseVecOrderHistoryItem {
  [field: string]: unknown;
  "data": Array<PaginatedResponseVecOrderHistoryItemDataItem>;
  "hasMore": boolean;
  "nextCursor"?: string | null;
  "prevCursor"?: string | null;
}

export interface PaginatedResponseVecOrderHistoryItemDataItem {
  [field: string]: unknown;
  "baseQty": string;
  "completedAt"?: string | null;
  "filledBaseQty": string;
  "isConditionalOrder"?: boolean | null;
  "isReduceOnly": boolean;
  "isStopLoss"?: boolean | null;
  "isStopLossDirection"?: boolean | null;
  "marketSymbol": string;
  "orderSequenceNumber": string;
  "placedAt"?: string | null;
  "price": string;
  "remainingBaseQty": string;
  "side": Side;
  "status": OrderStatus;
}

export interface PaginatedResponseVecOrderHistoryV2Item {
  [field: string]: unknown;
  "data": Array<PaginatedResponseVecOrderHistoryV2ItemDataItem>;
  "hasMore": boolean;
  "nextCursor"?: string | null;
  "prevCursor"?: string | null;
}

export interface PaginatedResponseVecOrderHistoryV2ItemDataItem {
  [field: string]: unknown;
  "createdAt": string;
  "currentOrderState"?: HttpValue | null;
  "eventIndex": number | bigint | string;
  "fillBeforePlacement"?: HttpValue | null;
  "instructionIndex": number | bigint | string;
  "instructionType": string;
  "intended": IntendedOrder;
  "isConditionalOrder": boolean;
  "isReduceOnly": boolean;
  "isStopLoss": boolean;
  "isStopLossDirection": boolean;
  "marketSymbol": string;
  "orderType": string;
  "placedOrder"?: HttpValue | null;
  "price": string;
  "slot": number | bigint | string;
  "slotIndex": number | bigint | string;
  "status": string;
  "traderId": number | bigint | string;
  "traderPdaIndex": number | bigint | string;
  "traderSubaccountIndex": number | bigint | string;
  "userId": number | bigint | string;
}

export interface PaginatedResponseVecTradeHistoryV2Item {
  [field: string]: unknown;
  "data": Array<PaginatedResponseVecTradeHistoryV2ItemDataItem>;
  "hasMore": boolean;
  "nextCursor"?: string | null;
  "prevCursor"?: string | null;
}

export interface PaginatedResponseVecTradeHistoryV2ItemDataItem {
  [field: string]: unknown;
  "baseLotsAfter": string;
  "baseLotsBefore": string;
  "baseLotsDelta": string;
  "eventIndex": number | bigint | string;
  "fees": string;
  "fillId"?: string | null;
  "instructionIndex": number | bigint | string;
  "instructionType": string;
  "liquidity": LiquidityRole;
  "marketSymbol": string;
  "orderSequenceNumber"?: number | bigint | string | null;
  "price": string;
  "realizedPnl": string;
  "signature"?: string | null;
  "slot": number | bigint | string;
  "slotIndex": number | bigint | string;
  "splineSequenceNumber"?: number | bigint | string | null;
  "subaccountIndex": number | bigint | string;
  "timestamp": string;
  "tradeType": TradeType;
  "traderId": number | bigint | string;
  "traderPdaIndex": number | bigint | string;
  "userId": number | bigint | string;
  "virtualQuoteLotsAfter": string;
  "virtualQuoteLotsBefore": string;
  "virtualQuoteLotsDelta": string;
}

export interface PaginatedResponseVecUserLiquidationHistoryPoint {
  [field: string]: unknown;
  "data": Array<PaginatedResponseVecUserLiquidationHistoryPointDataItem>;
  "hasMore": boolean;
  "nextCursor"?: string | null;
  "prevCursor"?: string | null;
}

export interface PaginatedResponseVecUserLiquidationHistoryPointDataItem {
  [field: string]: unknown;
  "atLossCloseValue"?: string | null;
  "atLossCollateralChange"?: string | null;
  "baseLotsFilled"?: string | null;
  "caller"?: string | null;
  "closedLong"?: string | null;
  "closedShort"?: string | null;
  "eventIndex": number | bigint | string;
  "haircutRateBps"?: number | bigint | string | null;
  "inProfitAccount"?: string | null;
  "inProfitCloseValue"?: string | null;
  "inProfitCollateralChange"?: string | null;
  "ixName": string;
  "kind": UserLiquidationHistoryKind;
  "liquidatee"?: string | null;
  "liquidateeCollateralChange"?: string | null;
  "liquidator"?: string | null;
  "liquidatorCollateralChange"?: string | null;
  "market": string;
  "positionClosed"?: boolean | null;
  "price"?: string | null;
  "quoteLotsFilled"?: string | null;
  "quoteSize"?: string | null;
  "role": UserLiquidationHistoryRole;
  "side"?: string | null;
  "signature"?: string | null;
  "size"?: string | null;
  "slot": number | bigint | string;
  "slotIndex": number | bigint | string;
  "subaccountIndex"?: number | bigint | string | null;
  "symbol": string;
  "timestamp": number | bigint | string;
  "type": UserLiquidationHistoryType;
}

export interface PlaceAttachedConditionalOrderRequest {
  [field: string]: unknown;
  "authority": string;
  "feePayer"?: string | null;
  "flightBuilderAuthority"?: string | null;
  "flightFeeCollectorTrader"?: string | null;
  "greaterTrigger"?: HttpValue | null;
  "isIsolated"?: boolean | null;
  "lessTrigger"?: HttpValue | null;
  "orderPriceInTicks": number | bigint | string;
  "orderSequenceNumber": string;
  "positionAuthority"?: string | null;
  "symbol": string;
  "traderPdaIndex": number | bigint | string;
  "traderSubaccountIndex"?: number | bigint | string | null;
}

export interface PlaceIsolatedLimitOrderEnhancedResponse {
  [field: string]: unknown;
  "estimatedLiquidationPriceUsd"?: number | bigint | string | null;
  "instructions": Array<ApiInstructionResponse>;
}

export interface PlaceIsolatedLimitOrderRequest {
  [field: string]: unknown;
  "allowCrossAndIsolatedForAsset"?: boolean | null;
  "authority": string;
  "feePayer"?: string | null;
  "flightBuilderAuthority"?: string | null;
  "flightFeeCollectorTrader"?: string | null;
  "isPostOnly"?: boolean | null;
  "isReduceOnly"?: boolean | null;
  "numBaseLots"?: number | bigint | string | null;
  "pdaIndex"?: number | bigint | string | null;
  "positionAuthority"?: string | null;
  "price"?: number | bigint | string | null;
  "priceInTicks"?: number | bigint | string | null;
  "quantity"?: number | bigint | string | null;
  "side": string;
  "skipTransferToParent"?: boolean | null;
  "slide"?: boolean | null;
  "symbol": string;
  "tpSl"?: HttpValue | null;
  "transferAmount"?: number | bigint | string | null;
}

export interface PlaceIsolatedLimitOrderWithConditionalsRequest {
  [field: string]: unknown;
  "allowCrossAndIsolatedForAsset"?: boolean | null;
  "authority": string;
  "feePayer"?: string | null;
  "flightBuilderAuthority"?: string | null;
  "flightFeeCollectorTrader"?: string | null;
  "greaterTrigger"?: HttpValue | null;
  "isPostOnly"?: boolean | null;
  "lessTrigger"?: HttpValue | null;
  "numBaseLots"?: number | bigint | string | null;
  "pdaIndex"?: number | bigint | string | null;
  "positionAuthority"?: string | null;
  "price"?: number | bigint | string | null;
  "priceInTicks"?: number | bigint | string | null;
  "quantity"?: number | bigint | string | null;
  "side": string;
  "skipTransferToParent"?: boolean | null;
  "slide"?: boolean | null;
  "symbol": string;
  "transferAmount"?: number | bigint | string | null;
}

export interface PlaceIsolatedMarketOrderEnhancedResponse {
  [field: string]: unknown;
  "estimatedLiquidationPriceUsd"?: number | bigint | string | null;
  "instructions": Array<ApiInstructionResponse>;
}

export interface PlaceIsolatedMarketOrderRequest {
  [field: string]: unknown;
  "allowCrossAndIsolatedForAsset"?: boolean | null;
  "authority": string;
  "feePayer"?: string | null;
  "flightBuilderAuthority"?: string | null;
  "flightFeeCollectorTrader"?: string | null;
  "isReduceOnly"?: boolean | null;
  "maxPriceInTicks"?: number | bigint | string | null;
  "numBaseLots"?: number | bigint | string | null;
  "pdaIndex"?: number | bigint | string | null;
  "positionAuthority"?: string | null;
  "quantity"?: number | bigint | string | null;
  "side": string;
  "skipTransferToParent"?: boolean | null;
  "symbol": string;
  "tpSl"?: HttpValue | null;
  "transferAmount"?: number | bigint | string | null;
}

export interface PlacePositionConditionalOrderRequest {
  [field: string]: unknown;
  "authority": string;
  "feePayer"?: string | null;
  "flightBuilderAuthority"?: string | null;
  "flightFeeCollectorTrader"?: string | null;
  "greaterTrigger"?: HttpValue | null;
  "isIsolated"?: boolean | null;
  "lessTrigger"?: HttpValue | null;
  "numBaseLots"?: number | bigint | string | null;
  "positionAuthority"?: string | null;
  "quantity"?: number | bigint | string | null;
  "sizePercent"?: number | bigint | string | null;
  "symbol": string;
  "traderPdaIndex": number | bigint | string;
  "traderSubaccountIndex"?: number | bigint | string | null;
}

export interface PnLPoint {
  [field: string]: unknown;
  "cumulativeFundingPayment": number | bigint | string;
  "cumulativePnl": number | bigint | string;
  "cumulativeTakerFee": number | bigint | string;
  "endTime": number | bigint | string;
  "startTime": number | bigint | string;
  "timestamp": number | bigint | string;
  "unrealizedPnl": number | bigint | string;
}

export interface PortfolioValuePoint {
  [field: string]: unknown;
  "endTime": number | bigint | string;
  "positions"?: HttpValue | null;
  "startTime": number | bigint | string;
  "timestamp": number | bigint | string;
  "value": number | bigint | string;
}

export interface PrivyLoginRequest {
  [field: string]: unknown;
  "privy_token": string;
  "wallet_proof"?: HttpValue | null;
  "wallet_pubkey"?: string | null;
}

export interface ReferralActivationPermissionResponse {
  [field: string]: unknown;
  "permission_account": string;
  "risk_authority": string;
  "trader_onboarder": string;
}

export interface RefreshRequest {
  [field: string]: unknown;
  "refresh_token": string;
}

export interface RefreshSessionParams {
  [field: string]: unknown;
  "body": RefreshRequest;
}

export interface SendRegisterIxsRequest {
  [field: string]: unknown;
  "maxPositions"?: number | bigint | string | null;
  "traderAuthority": string;
  "traderPdaIndex"?: number | bigint | string | null;
  "traderSubaccountIndex"?: number | bigint | string | null;
  "transaction": string;
  "txFeePayer": string;
}

export interface SendRegisterIxsResponse {
  [field: string]: unknown;
  "includeRegisterTrader": boolean;
  "maxPositions": number | bigint | string;
  "signature": string;
  "traderOnboarder": string;
  "traderPda": string;
  "txFeePayer": string;
}

export interface ServiceChallengeRequest {
  [field: string]: unknown;
  "client_id": string;
  "key_id"?: string | null;
}

export interface ServiceChallengeResponse {
  [field: string]: unknown;
  "expires_at": string;
  "key_id": string;
  "message": string;
  "nonce": string;
}

export interface ServiceLoginRequest {
  [field: string]: unknown;
  "client_id": string;
  "key_id"?: string | null;
  "nonce"?: string;
  "signature"?: string;
  "timestamp": string;
}

export interface Spline {
  [field: string]: unknown;
  "askFilledAmount": string;
  "askRegions": Array<TickRegion>;
  "bidFilledAmount": string;
  "bidRegions": Array<TickRegion>;
  "midPrice": number | bigint | string;
  "traderKey": string;
}

export interface SubmitRegisterTraderTransactionParams {
  [field: string]: unknown;
  "body": SendRegisterIxsRequest;
}

export interface TickRegion {
  [field: string]: unknown;
  "density": number | bigint | string;
  "end": number | bigint | string;
  "filledSize": string;
  "start": number | bigint | string;
  "totalSize": string;
}

export interface TraderCapabilitiesMetadata {
  [field: string]: unknown;
  "capabilities": Array<TraderCapabilityDescriptor>;
}

export interface TraderCapabilitiesView {
  [field: string]: unknown;
  "depositCollateral": CapabilityAccessView;
  "placeLimitOrder": CapabilityAccessView;
  "placeMarketOrder": CapabilityAccessView;
  "riskIncreasingTrade": CapabilityAccessView;
  "riskReducingTrade": CapabilityAccessView;
  "withdrawCollateral": CapabilityAccessView;
}

export interface TraderCapabilityDescriptor {
  [field: string]: unknown;
  "deprecated"?: boolean | null;
  "description": string;
  "displayName": string;
  "key": string;
}

export interface TraderFundingHistoryEvent {
  [field: string]: unknown;
  "fundingPayment": string;
  "fundingRatePercentage": string;
  "positionSide": string;
  "positionSize": string;
  "symbol": string;
  "timestamp": string;
}

export interface TraderFundingHistoryResponse {
  [field: string]: unknown;
  "events": Array<TraderFundingHistoryEvent>;
  "hasMore": boolean;
  "nextCursor"?: string | null;
  "prevCursor"?: string | null;
}

export interface TraderMarketPnLPoint {
  [field: string]: unknown;
  "baseLots": number | bigint | string;
  "cumulativeFundingPayment": number | bigint | string;
  "cumulativePnl": number | bigint | string;
  "cumulativeTakerFee": number | bigint | string;
  "endTime": number | bigint | string;
  "markPrice": number | bigint | string;
  "realizedPnl": number | bigint | string;
  "startTime": number | bigint | string;
  "timestamp": number | bigint | string;
  "totalTakerFee": number | bigint | string;
  "unrealizedPnl": number | bigint | string;
  "virtualQuoteLots": number | bigint | string;
}

export interface TraderMarketPnLSeries {
  [field: string]: unknown;
  "baseDecimals": number | bigint | string;
  "marketId": number | bigint | string;
  "points": Array<TraderMarketPnLPoint>;
  "symbol": string;
  "tickSize": number | bigint | string;
}

export interface TraderPositionView {
  [field: string]: unknown;
  "accumulatedFunding": string;
  "backstopMargin": string;
  "discountedUnrealizedPnl": string;
  "entryPrice": string;
  "initialMargin": string;
  "limitOrderMargin": string;
  "liquidationPrice": string;
  "maintenanceMargin": string;
  "positionInitialMargin": string;
  "positionSize": string;
  "positionValue": string;
  "stopLossPrice"?: string | null;
  "symbol": string;
  "takeProfitPrice"?: string | null;
  "unrealizedPnl": string;
  "unsettledFunding": string;
  "virtualQuotePosition": string;
}

export interface TraderStateCapabilities {
  [field: string]: unknown;
  "capabilities": TraderCapabilitiesView;
  "flags": TraderCapabilityFlags;
  "state": TraderActivityStateView;
}

export interface TraderStateLimitOrderEvent {
  [field: string]: unknown;
  "orders": Array<TraderStateMarketLimitOrderEvent>;
  "symbol": string;
}

export interface TraderStateMarketLimitOrderEvent {
  [field: string]: unknown;
  "change"?: HttpValue | null;
  "conditionalKind"?: string | null;
  "initialSizeLots": string;
  "isConditionalOrder"?: boolean | null;
  "isStopLoss"?: boolean | null;
  "isStopLossDirection"?: boolean | null;
  "orderSequenceNumber": string;
  "orderType": string;
  "priceTicks": string;
  "priceUsd": string;
  "reduceOnly": boolean;
  "side": Side;
  "sizeRemainingLots": string;
  "status": string;
}

export interface TraderStateSnapshot {
  [field: string]: unknown;
  "capabilities": TraderStateCapabilities;
  "makerFeeOverrideMultiplier": number | bigint | string;
  "subaccounts": Array<TraderStateSubaccountSnapshot>;
  "takerFeeOverrideMultiplier": number | bigint | string;
  "version": number | bigint | string;
}

export interface TraderStateSnapshotResponse {
  [field: string]: unknown;
  "authority": string;
  "slot": number | bigint | string;
  "slotIndex": number | bigint | string;
  "snapshot": TraderStateSnapshot;
  "traderPdaIndex": number | bigint | string;
}

export interface TraderStateSubaccountSnapshot {
  [field: string]: unknown;
  "capabilities"?: HttpValue | null;
  "collateral": string;
  "cooldownStatus"?: HttpValue | null;
  "orders"?: Array<TraderStateLimitOrderEvent> | null;
  "positions"?: Array<TraderStatePositionSnapshot> | null;
  "sequence": number | bigint | string;
  "splines"?: Array<TraderStateSplineSnapshot> | null;
  "subaccountIndex": number | bigint | string;
  "triggers"?: Array<TraderStateTriggerSnapshot> | null;
}

export interface TraderView {
  [field: string]: unknown;
  "accumulatedFunding": string;
  "authority": string;
  "cancelMargin": string;
  "capabilities": TraderCapabilitiesView;
  "collateralBalance": string;
  "discountedUnrealizedPnl": string;
  "effectiveCollateral": string;
  "effectiveCollateralForWithdrawals": string;
  "flags": TraderCapabilityFlags;
  "initialMargin": string;
  "initialMarginForWithdrawals": string;
  "isInActiveTraders": boolean;
  "lastDepositSlot": number | bigint | string;
  "limitOrders": Record<string, Array<LimitOrder>>;
  "maintenanceMargin": string;
  "makerFeeOverrideMultiplier": number | bigint | string;
  "maxPositions": number | bigint | string;
  "numMarketsWithSplines": number | bigint | string;
  "portfolioValue": string;
  "positions": Array<TraderPositionView>;
  "riskState": RiskState;
  "riskTier": RiskTier;
  "slot": number | bigint | string;
  "slotIndex": number | bigint | string;
  "state": TraderActivityStateView;
  "takerFeeOverrideMultiplier": number | bigint | string;
  "traderKey": string;
  "traderPdaIndex": number | bigint | string;
  "traderSubaccountIndex": number | bigint | string;
  "unrealizedPnl": string;
  "unsettledFundingOwed": string;
}

export interface ValidateInviteParams {
  [field: string]: unknown;
  "body": ValidateInviteRequest;
}

export interface ValidateInviteRequest {
  [field: string]: unknown;
  "code": string;
  "transaction_signature"?: string | null;
  "wallet_address": string;
}

export interface ValidateInviteResponse {
  [field: string]: unknown;
  "message": string;
  "success": boolean;
  "whitelisted": boolean;
}

export interface WalletLoginRequest {
  [field: string]: unknown;
  "nonce_id": string;
  "signature"?: string;
  "wallet_pubkey": string;
}

export interface WalletNonceResponse {
  [field: string]: unknown;
  "expires_at": string;
  "message": string;
  "nonce_id": string;
}

export interface WalletTransactionChallengeRequest {
  [field: string]: unknown;
  "wallet_pubkey": string;
}

export interface WalletTransactionChallengeResponse {
  [field: string]: unknown;
  "expires_at": string;
  "nonce_id": string;
  "unsigned_transaction": string;
}

export interface WalletTransactionLoginRequest {
  [field: string]: unknown;
  "nonce_id": string;
  "signed_transaction": string;
  "wallet_pubkey": string;
}
