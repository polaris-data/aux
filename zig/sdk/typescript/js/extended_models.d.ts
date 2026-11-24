// Generated from schema/extended.json.
import type { HttpValue } from "./index";
export type AssetOperationStatus = string;
export type AssetOperationType = string;
export type AssetType = string;
export type ExitType = string;
export type ExportType = string;
export type LeaderboardInterval = string;
export type MarketTradingHours = string;
export type MarketType = string;
export type OrderPriceType = string;
export type OrderSide = string;
export type OrderSortBy = string;
export type OrderStatus = string;
export type OrderStatusReason = string;
export type OrderTimeInForce = string;
export type OrderTpslType = string;
export type OrderTriggerPriceType = string;
export type OrderType = string;
export type PositionSide = string;
export type PositionStatus = string;
export type ReferralDashboardGranularity = string;
export type ReferralDashboardTimeRange = string;
export type TimeInForce = string;
export type TradeType = string;

export interface AccountLeverage {
  [field: string]: unknown;
  "market": string;
  "leverage": string;
}

export interface AccountLeverageModel {
  [field: string]: unknown;
  "market": string;
  "leverage": string;
}

export interface AccountModel {
  [field: string]: unknown;
  "id"?: number | bigint | string | null;
  "description": string;
  "accountIndex": number | bigint | string;
  "status": string;
  "l2Key": string;
  "l2Vault": number | bigint | string;
  "bridgeStarknetAddress"?: string | null;
}

export interface AccountTradeModel {
  [field: string]: unknown;
  "id": number | bigint | string;
  "accountId": number | bigint | string;
  "market": string;
  "orderId": number | bigint | string;
  "side": OrderSide;
  "price": string;
  "qty": string;
  "value": string;
  "fee": string;
  "isTaker": boolean;
  "tradeType": TradeType;
  "createdTime": number | bigint | string;
}

export interface AccountUpdateRequest {
  [field: string]: unknown;
  "description"?: string | null;
}

export interface ApiKeyResponseModel {
  [field: string]: unknown;
  "key": string;
}

export interface ApiResponse {
  [field: string]: unknown;
  "ok": boolean;
  "data"?: HttpValue | null;
}

export interface AssetModel {
  [field: string]: unknown;
  "id": number | bigint | string;
  "name": string;
  "symbol": string;
  "precision": number | bigint | string;
  "isActive": boolean;
  "isCollateral": boolean;
  "starkexId": string;
  "starkexResolution": number | bigint | string;
  "l1Id": string;
  "l1Resolution": number | bigint | string;
  "version": number | bigint | string;
  "type": AssetType;
  "canBeUsedAsCollateral": boolean;
}

export interface AssetOperationModel {
  [field: string]: unknown;
  "id": string;
  "type": AssetOperationType;
  "status": AssetOperationStatus;
  "amount": string;
  "fee": string;
  "asset": number | bigint | string;
  "time": number | bigint | string;
  "accountId": number | bigint | string;
  "counterpartyAccountId"?: number | bigint | string | null;
  "transactionHash"?: string | null;
}

export interface AssetOperationsParams {
  [field: string]: unknown;
  "id"?: string | null;
  "type"?: string | null;
  "status"?: string | null;
  "startTime"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
  "cursor"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface AssetOperationsResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<AssetOperationModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface BalanceModel {
  [field: string]: unknown;
  "collateralName": string;
  "balance": string;
  "equity": string;
  "availableForTrade": string;
  "availableForWithdrawal": string;
  "unrealisedPnl": string;
  "initialMargin": string;
  "marginRatio": string;
  "updatedTime": number | bigint | string;
  "spotEquity"?: string | null;
  "spotEquityForAvailableForTrade"?: string | null;
  "collateralReservedForSpotOrders"?: string | null;
}

export interface BridgesConfigModel {
  [field: string]: unknown;
  "chains": Array<ChainConfigModel>;
}

export interface BuilderTradeModel {
  [field: string]: unknown;
  "id": number | bigint | string;
  "time": number | bigint | string;
  "volume": string;
  "makerId"?: number | bigint | string | null;
  "takerId"?: number | bigint | string | null;
  "makerBuilderId"?: number | bigint | string | null;
  "takerBuilderId"?: number | bigint | string | null;
  "makerFee"?: string | null;
  "takerFee"?: string | null;
  "makerBuilderFee"?: string | null;
  "takerBuilderFee"?: string | null;
}

export interface CancelOrderByExternalIdParams {
  [field: string]: unknown;
  "externalId"?: string | null;
}

export interface CancelOrderByExternalIdResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: EmptyModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface CancelOrderParams {
  [field: string]: unknown;
  "id": string;
}

export interface CandleModel {
  [field: string]: unknown;
  "open"?: string | null;
  "low"?: string | null;
  "high"?: string | null;
  "close"?: string | null;
  "volume"?: string | null;
  "timestamp"?: number | bigint | string | null;
}

export interface ChainConfigModel {
  [field: string]: unknown;
  "chain": string;
  "contractAddress": string;
}

export interface ClaimResponseModel {
  [field: string]: unknown;
  "id": number | bigint | string;
}

export interface ClaimTestingFundsParams {
  [field: string]: unknown;
  "body": HttpValue;
}

export interface ClaimTestingFundsResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: ClaimResponseModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface ClientModel {
  [field: string]: unknown;
  "id": number | bigint | string;
  "evmWalletAddress"?: HttpValue | null;
  "starknetWalletAddress"?: HttpValue | null;
  "referralLinkCode"?: HttpValue | null;
}

export interface CollateralSlowWithdrawal {
  [field: string]: unknown;
  "amount": string;
  "asset": string;
  "positionId"?: number | bigint | string | null;
}

export interface CommitBridgeQuoteParams {
  [field: string]: unknown;
  "id"?: string | null;
}

export interface CommitBridgeQuoteResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: EmptyModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface CreateApiKeyParams {
  [field: string]: unknown;
  "L1_SIGNATURE": string;
  "L1_MESSAGE_TIME": string;
  "X-X10-ACTIVE-ACCOUNT": string;
  "body": CreateApiKeyRequest;
}

export interface CreateApiKeyRequest {
  [field: string]: unknown;
  "description": string;
}

export interface CreateApiKeyResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: ApiKeyResponseModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface CreateDepositParams {
  [field: string]: unknown;
  "body": CreateDepositRequest;
}

export interface CreateDepositRequest {
  [field: string]: unknown;
  "accountId": number | bigint | string;
  "assetId": string;
  "amount": string;
  "sourceChain": string;
  "snTransactionHash"?: string | null;
  "fee"?: string | null;
  "bridgeId"?: string | null;
}

export interface CreateOrderRequest {
  [field: string]: unknown;
  "id"?: number | bigint | string | null;
  "market": string;
  "type": OrderType;
  "side": OrderSide;
  "qty": string;
  "price"?: string | null;
  "timeInForce": OrderTimeInForce;
  "expiryEpochMillis"?: number | bigint | string | null;
  "fee": string;
  "nonce"?: number | bigint | string;
  "reduceOnly"?: boolean | null;
  "postOnly"?: boolean | null;
  "trigger"?: OrderConditionalTrigger | null;
  "tpSlType"?: OrderTpslType | null;
  "twap"?: Twap | null;
}

export interface CreateRFQOrderRequest {
  [field: string]: unknown;
  "id"?: number | bigint | string | null;
  "market": string;
  "type": OrderType;
  "side": OrderSide;
  "qty": string;
  "price"?: string | null;
  "timeInForce": OrderTimeInForce;
  "expiryEpochMillis"?: number | bigint | string | null;
  "fee": string;
  "nonce"?: number | bigint | string;
  "reduceOnly"?: boolean | null;
  "postOnly"?: boolean | null;
  "trigger"?: OrderConditionalTrigger | null;
  "tpSlType"?: OrderTpslType | null;
  "twap"?: Twap | null;
  "rfqStartPrice"?: string | null;
}

export interface CreateReferralLinkCodeRequest {
  [field: string]: unknown;
  "code": string;
  "description"?: string | null;
}

export interface CreateReferralLinkParams {
  [field: string]: unknown;
  "body": CreateReferralLinkCodeRequest;
}

export interface CreateScaledOrderParams {
  [field: string]: unknown;
  "body": CreateScaledOrderRequest;
}

export interface CreateScaledOrderRequest {
  [field: string]: unknown;
  "market": string;
  "side": OrderSide;
  "totalQty": string;
  "numOrders": number | bigint | string;
  "priceMin": string;
  "priceMax": string;
  "timeInForce": OrderTimeInForce;
  "expiryEpochMillis"?: number | bigint | string | null;
  "reduceOnly"?: boolean | null;
}

export interface DeleteApiKeyParams {
  [field: string]: unknown;
  "id": string;
}

export interface DeleteUserOrderOrderIdParams {
  [field: string]: unknown;
  "order_id": string;
}

export interface DeleteUserOrderOrderIdResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: EmptyModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface DepositRequestModel {
  [field: string]: unknown;
  "fromAccountId": number | bigint | string;
  "toAccountId": number | bigint | string;
  "collateral": string;
  "shares": string;
  "settlement": LimitOrderSettlementModel;
}

export interface DepositToVaultParams {
  [field: string]: unknown;
  "body": DepositRequestModel;
}

export interface DepositToVaultResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: HttpValue | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface EmptyModel {
  [field: string]: unknown;
}

export interface ExportCsvZipParams {
  [field: string]: unknown;
  "type[]": Array<ExportType>;
  "accountId"?: number | bigint | string | null;
}

export interface FundingRateConfigRequest {
  [field: string]: unknown;
  "market": string;
  "enabled": boolean;
}

export interface FundingRateModel {
  [field: string]: unknown;
  "market"?: string | null;
  "fundingRate"?: string | null;
  "timestamp"?: number | bigint | string | null;
}

export interface GetAccountEquityHistoryParams {
  [field: string]: unknown;
  "accountId": number | bigint | string;
  "interval": string;
}

export interface GetAccountEquityHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<GetAccountEquityHistoryResultDataItem> | null;
}

export interface GetAccountEquityHistoryResultDataItem {
  [field: string]: unknown;
  "date"?: string | null;
  "value"?: string | null;
}

export interface GetAccountFundingChartParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
  "interval": string;
  "market"?: Array<string> | null;
}

export interface GetAccountFundingChartResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<GetAccountFundingChartResultDataItem> | null;
}

export interface GetAccountFundingChartResultDataItem {
  [field: string]: unknown;
  "date"?: string | null;
  "value"?: string | null;
}

export interface GetAccountFundingHistoryParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
  "interval": string;
  "market"?: Array<string> | null;
  "cursor"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetAccountFundingHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<GetAccountFundingHistoryResultDataItem> | null;
  "pagination"?: GetAccountFundingHistoryResultPagination | null;
}

export interface GetAccountFundingHistoryResultDataItem {
  [field: string]: unknown;
  "id"?: number | bigint | string | null;
  "accountId"?: number | bigint | string | null;
  "market"?: string | null;
  "side"?: string | null;
  "size"?: string | null;
  "value"?: string | null;
  "markPrice"?: string | null;
  "fundingFee"?: string | null;
  "fundingRate"?: string | null;
  "paidTime"?: number | bigint | string | null;
}

export interface GetAccountFundingHistoryResultPagination {
  [field: string]: unknown;
  "cursor"?: number | bigint | string | null;
  "count"?: number | bigint | string | null;
}

export interface GetAccountFundingStatsParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
  "interval": string;
  "market"?: Array<string> | null;
}

export interface GetAccountFundingStatsResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GetAccountFundingStatsResultData | null;
}

export interface GetAccountFundingStatsResultData {
  [field: string]: unknown;
  "totalFunding"?: string | null;
  "fundingPerMarket"?: Array<GetAccountFundingStatsResultDataFundingPerMarketItem> | null;
}

export interface GetAccountFundingStatsResultDataFundingPerMarketItem {
  [field: string]: unknown;
  "market"?: string | null;
  "netFundingFees"?: string | null;
  "netExposure"?: string | null;
  "fundingRate"?: string | null;
}

export interface GetAccountHealthParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
}

export interface GetAccountHealthResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<GetAccountHealthResultDataItem> | null;
}

export interface GetAccountHealthResultDataItem {
  [field: string]: unknown;
  "accountId"?: number | bigint | string | null;
  "equity"?: string | null;
  "unrealisedPnl"?: string | null;
  "exposure"?: string | null;
  "netExposure"?: string | null;
  "leverage"?: string | null;
  "initialMargin"?: string | null;
  "marginRatio"?: string | null;
  "availableForTrading"?: string | null;
  "availableForSpotTrading"?: string | null;
  "availableForWithdrawal"?: string | null;
  "health"?: string | null;
}

export interface GetAccountMaxDrawdownHistoryParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
  "interval": string;
}

export interface GetAccountMaxDrawdownHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<GetAccountMaxDrawdownHistoryResultDataItem> | null;
}

export interface GetAccountMaxDrawdownHistoryResultDataItem {
  [field: string]: unknown;
  "accountId"?: number | bigint | string | null;
  "data"?: Array<GetAccountMaxDrawdownHistoryResultDataItemDataItem> | null;
}

export interface GetAccountMaxDrawdownHistoryResultDataItemDataItem {
  [field: string]: unknown;
  "date"?: string | null;
  "value"?: string | null;
}

export interface GetAccountParams {
  [field: string]: unknown;
}

export interface GetAccountPerformanceParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
  "interval": string;
  "marketType"?: string | null;
}

export interface GetAccountPerformanceResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<GetAccountPerformanceResultDataItem> | null;
}

export interface GetAccountPerformanceResultDataItem {
  [field: string]: unknown;
  "accountId"?: number | bigint | string | null;
  "realisedPnl"?: string | null;
  "unrealisedPnl"?: string | null;
  "totalPnl"?: string | null;
  "totalPnlPercentage"?: string | null;
  "profitFactor"?: string | null;
  "maxDrawdown"?: string | null;
  "winRate"?: string | null;
  "tradingVolume"?: string | null;
  "makerVolume"?: string | null;
  "sharpeRatio"?: string | null;
}

export interface GetAccountPnLHistoryParams {
  [field: string]: unknown;
  "accountId": number | bigint | string;
  "interval": string;
  "pnlType": string;
  "instrumentType"?: string | null;
}

export interface GetAccountPnLHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<GetAccountPnLHistoryResultDataItem> | null;
}

export interface GetAccountPnLHistoryResultDataItem {
  [field: string]: unknown;
  "date"?: string | null;
  "value"?: string | null;
}

export interface GetAccountPnLPercentageHistoryParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
  "interval": string;
  "pnlType": string;
  "priceMarket"?: Array<string> | null;
  "instrumentType"?: string | null;
}

export interface GetAccountPnLPercentageHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GetAccountPnLPercentageHistoryResultData | null;
}

export interface GetAccountPnLPercentageHistoryResultData {
  [field: string]: unknown;
  "pnl"?: Array<GetAccountPnLPercentageHistoryResultDataPnlItem> | null;
  "priceChanges"?: GetAccountPnLPercentageHistoryResultDataPriceChanges | null;
}

export interface GetAccountPnLPercentageHistoryResultDataPnlItem {
  [field: string]: unknown;
  "date"?: string | null;
  "value"?: string | null;
}

export interface GetAccountPnLPercentageHistoryResultDataPriceChanges {
  [field: string]: unknown;
  "BTC-USD"?: Array<GetAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem> | null;
}

export interface GetAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem {
  [field: string]: unknown;
  "date"?: string | null;
  "value"?: string | null;
}

export interface GetAccountPortfolioSummaryParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
  "interval": string;
  "instrumentType"?: string | null;
}

export interface GetAccountPortfolioSummaryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GetAccountPortfolioSummaryResultData | null;
}

export interface GetAccountPortfolioSummaryResultData {
  [field: string]: unknown;
  "equity"?: string | null;
  "accountValue"?: GetAccountPortfolioSummaryResultDataAccountValue | null;
  "exposure"?: string | null;
  "totalPnl"?: string | null;
  "profitFactor"?: string | null;
  "winRate"?: string | null;
  "tradingVolume"?: string | null;
  "makerVolume"?: string | null;
  "sharpeRatio"?: string | null;
}

export interface GetAccountPortfolioSummaryResultDataAccountValue {
  [field: string]: unknown;
  "accountNotionalValue"?: string | null;
  "tradingEquity"?: string | null;
  "vaultNotionalValue"?: string | null;
  "spotNotionalValue"?: string | null;
}

export interface GetAccountResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: AccountModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetAccountVaultEquityHistoryParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
  "interval": string;
}

export interface GetAccountVaultEquityHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<GetAccountVaultEquityHistoryResultDataItem> | null;
}

export interface GetAccountVaultEquityHistoryResultDataItem {
  [field: string]: unknown;
  "date"?: string | null;
  "value"?: string | null;
}

export interface GetAccountsParams {
  [field: string]: unknown;
}

export interface GetAccountsResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<AccountModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetAffiliateParams {
  [field: string]: unknown;
}

export interface GetAffiliateRewardsParams {
  [field: string]: unknown;
}

export interface GetApiKeysParams {
  [field: string]: unknown;
}

export interface GetAssetOperationsParams {
  [field: string]: unknown;
  "accountId[]"?: Array<number | bigint | string> | null;
  "id"?: string | null;
  "type[]"?: Array<string> | null;
  "status[]"?: Array<string> | null;
  "startTime"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetAssetPriceParams {
  [field: string]: unknown;
  "asset": string;
}

export interface GetAssetsParams {
  [field: string]: unknown;
  "type"?: AssetType | null;
  "asset"?: Array<string> | null;
  "collateral"?: boolean | null;
}

export interface GetAssetsResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<AssetModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetBalanceParams {
  [field: string]: unknown;
}

export interface GetBalanceResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: BalanceModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetBridgeConfigParams {
  [field: string]: unknown;
}

export interface GetBridgeConfigResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: BridgesConfigModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetBridgeQuoteParams {
  [field: string]: unknown;
  "chainIn"?: string | null;
  "chainOut"?: string | null;
  "amount"?: string | null;
}

export interface GetBridgeQuoteResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: QuoteModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetBuilderDashboardParams {
  [field: string]: unknown;
}

export interface GetBuilderTradesParams {
  [field: string]: unknown;
  "cursor"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetBuilderTradesResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<BuilderTradeModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetCandlesHistoryParams {
  [field: string]: unknown;
  "market": string;
  "candle_type": string;
  "interval"?: string | null;
  "limit"?: number | bigint | string | null;
  "endTime"?: string | null;
}

export interface GetCandlesHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<CandleModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetClaimParams {
  [field: string]: unknown;
  "id": number | bigint | string;
}

export interface GetClaimsParams {
  [field: string]: unknown;
  "startTime"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetClientParams {
  [field: string]: unknown;
}

export interface GetClientResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: ClientModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetCumulativeAccountPnLHistoryParams {
  [field: string]: unknown;
  "accountId": number | bigint | string;
  "interval": string;
  "pnlType": string;
  "instrumentType"?: string | null;
}

export interface GetCumulativeAccountPnLHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<GetCumulativeAccountPnLHistoryResultDataItem> | null;
}

export interface GetCumulativeAccountPnLHistoryResultDataItem {
  [field: string]: unknown;
  "date"?: string | null;
  "value"?: string | null;
}

export interface GetCumulativeAccountPnLPercentageHistoryParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
  "interval": string;
  "pnlType": string;
  "priceMarket"?: Array<string> | null;
  "instrumentType"?: string | null;
}

export interface GetCumulativeAccountPnLPercentageHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GetCumulativeAccountPnLPercentageHistoryResultData | null;
}

export interface GetCumulativeAccountPnLPercentageHistoryResultData {
  [field: string]: unknown;
  "pnl"?: Array<GetCumulativeAccountPnLPercentageHistoryResultDataPnlItem> | null;
  "priceChanges"?: GetCumulativeAccountPnLPercentageHistoryResultDataPriceChanges | null;
}

export interface GetCumulativeAccountPnLPercentageHistoryResultDataPnlItem {
  [field: string]: unknown;
  "date"?: string | null;
  "value"?: string | null;
}

export interface GetCumulativeAccountPnLPercentageHistoryResultDataPriceChanges {
  [field: string]: unknown;
  "BTC-USD"?: Array<GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem> | null;
}

export interface GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem {
  [field: string]: unknown;
  "date"?: string | null;
  "value"?: string | null;
}

export interface GetEarnedRewardsParams {
  [field: string]: unknown;
}

export interface GetExchangeStatusParams {
  [field: string]: unknown;
}

export interface GetFeesParams {
  [field: string]: unknown;
  "market"?: Array<string> | null;
  "builderId"?: number | bigint | string | null;
}

export interface GetFeesResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<TradingFeeModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetFundingRatesHistoryParams {
  [field: string]: unknown;
  "market": string;
  "startTime"?: string | null;
  "endTime"?: string | null;
  "cursor"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetFundingRatesHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<FundingRateModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetIndexPriceCandlesParams {
  [field: string]: unknown;
  "market": string;
  "interval": string;
  "limit"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
}

export interface GetInfoAssetsAssetNamePriceParams {
  [field: string]: unknown;
  "asset_name": string;
}

export interface GetInfoAssetsAssetNamePriceResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: string | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetInfoCandlesMarketCandleTypeParams {
  [field: string]: unknown;
  "market": string;
  "candleType": string;
  "interval": string;
  "limit": number | bigint | string;
  "endTime"?: number | bigint | string | null;
}

export interface GetInfoCandlesMarketCandleTypeResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<GetInfoCandlesMarketCandleTypeResultDataItem> | null;
}

export interface GetInfoCandlesMarketCandleTypeResultDataItem {
  [field: string]: unknown;
  "o"?: string | null;
  "l"?: string | null;
  "h"?: string | null;
  "c"?: string | null;
  "v"?: string | null;
  "T"?: number | bigint | string | null;
}

export interface GetInterestDailyMetricsParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
  "interval": string;
}

export interface GetInterestDailyMetricsResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GetInterestDailyMetricsResultData | null;
}

export interface GetInterestDailyMetricsResultData {
  [field: string]: unknown;
  "1000"?: Array<GetInterestDailyMetricsResultData1000Item> | null;
}

export interface GetInterestDailyMetricsResultData1000Item {
  [field: string]: unknown;
  "date"?: string | null;
  "assetId"?: number | bigint | string | null;
  "interestPaid"?: string | null;
  "averageBorrowed"?: string | null;
  "annualizedInterestRate"?: string | null;
}

export interface GetInterestKeyMetricsParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
}

export interface GetInterestKeyMetricsResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GetInterestKeyMetricsResultData | null;
}

export interface GetInterestKeyMetricsResultData {
  [field: string]: unknown;
  "borrowed"?: string | null;
  "currentInterestRate"?: string | null;
  "estimatedNextPayment"?: string | null;
}

export interface GetInterestPaymentChartParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
  "interval": string;
  "bucket"?: string | null;
}

export interface GetInterestPaymentChartResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GetInterestPaymentChartResultData | null;
}

export interface GetInterestPaymentChartResultData {
  [field: string]: unknown;
  "resolution"?: string | null;
  "points"?: Array<GetInterestPaymentChartResultDataPointsItem> | null;
}

export interface GetInterestPaymentChartResultDataPointsItem {
  [field: string]: unknown;
  "bucketStartTimestamp"?: number | bigint | string | null;
  "totalInterestPaid"?: string | null;
  "averageBorrowed"?: string | null;
  "averageApr"?: string | null;
  "borrowingByCollateralAsset"?: Array<GetInterestPaymentChartResultDataPointsItemBorrowingByCollateralAssetItem> | null;
}

export interface GetInterestPaymentChartResultDataPointsItemBorrowingByCollateralAssetItem {
  [field: string]: unknown;
  "assetId"?: number | bigint | string | null;
  "averageBorrowed"?: string | null;
  "averageApr"?: string | null;
}

export interface GetInterestPaymentsHistoryParams {
  [field: string]: unknown;
  "accountId": Array<number | bigint | string>;
  "interval": string;
}

export interface GetInterestPaymentsHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GetInterestPaymentsHistoryResultData | null;
}

export interface GetInterestPaymentsHistoryResultData {
  [field: string]: unknown;
  "items"?: Array<GetInterestPaymentsHistoryResultDataItemsItem> | null;
}

export interface GetInterestPaymentsHistoryResultDataItemsItem {
  [field: string]: unknown;
  "timestamp"?: number | bigint | string | null;
  "paymentAmount"?: string | null;
  "borrowedAmount"?: string | null;
  "effectiveOneHourRate"?: string | null;
  "accountId"?: number | bigint | string | null;
}

export interface GetInterestRateCurvesHistoryParams {
  [field: string]: unknown;
  "interval": string;
}

export interface GetInterestRateCurvesHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GetInterestRateCurvesHistoryResultData | null;
}

export interface GetInterestRateCurvesHistoryResultData {
  [field: string]: unknown;
  "items"?: Array<GetInterestRateCurvesHistoryResultDataItemsItem> | null;
}

export interface GetInterestRateCurvesHistoryResultDataItemsItem {
  [field: string]: unknown;
  "timestamp"?: number | bigint | string | null;
  "amountBorrowed"?: string | null;
  "globalCurve"?: GetInterestRateCurvesHistoryResultDataItemsItemGlobalCurve | null;
  "assetCurves"?: Array<GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItem> | null;
}

export interface GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItem {
  [field: string]: unknown;
  "assetId"?: number | bigint | string | null;
  "curve"?: GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemCurve | null;
  "calculatedRate"?: string | null;
  "amountBorrowed"?: string | null;
}

export interface GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemCurve {
  [field: string]: unknown;
  "currentUtilization"?: string | null;
  "currentRate"?: string | null;
  "targetUtilization"?: string | null;
  "minRate"?: string | null;
  "targetRate"?: string | null;
  "maxRate"?: string | null;
}

export interface GetInterestRateCurvesHistoryResultDataItemsItemGlobalCurve {
  [field: string]: unknown;
  "currentUtilization"?: string | null;
  "currentRate"?: string | null;
  "targetUtilization"?: string | null;
  "minRate"?: string | null;
  "targetRate"?: string | null;
  "maxRate"?: string | null;
}

export interface GetLatestInterestRateCurveParams {
  [field: string]: unknown;
}

export interface GetLatestInterestRateCurveResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GetLatestInterestRateCurveResultData | null;
}

export interface GetLatestInterestRateCurveResultData {
  [field: string]: unknown;
  "item"?: GetLatestInterestRateCurveResultDataItem | null;
}

export interface GetLatestInterestRateCurveResultDataItem {
  [field: string]: unknown;
  "timestamp"?: number | bigint | string | null;
  "amountBorrowed"?: string | null;
  "globalCurve"?: GetLatestInterestRateCurveResultDataItemGlobalCurve | null;
  "assetCurves"?: Array<GetLatestInterestRateCurveResultDataItemAssetCurvesItem> | null;
}

export interface GetLatestInterestRateCurveResultDataItemAssetCurvesItem {
  [field: string]: unknown;
  "assetId"?: number | bigint | string | null;
  "curve"?: GetLatestInterestRateCurveResultDataItemAssetCurvesItemCurve | null;
  "calculatedRate"?: string | null;
  "amountBorrowed"?: string | null;
}

export interface GetLatestInterestRateCurveResultDataItemAssetCurvesItemCurve {
  [field: string]: unknown;
  "currentUtilization"?: string | null;
  "currentRate"?: string | null;
  "targetUtilization"?: string | null;
  "minRate"?: string | null;
  "targetRate"?: string | null;
  "maxRate"?: string | null;
}

export interface GetLatestInterestRateCurveResultDataItemGlobalCurve {
  [field: string]: unknown;
  "currentUtilization"?: string | null;
  "currentRate"?: string | null;
  "targetUtilization"?: string | null;
  "minRate"?: string | null;
  "targetRate"?: string | null;
  "maxRate"?: string | null;
}

export interface GetLeverageParams {
  [field: string]: unknown;
  "market"?: Array<string> | null;
}

export interface GetLeverageResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<AccountLeverageModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetMarkPriceCandlesParams {
  [field: string]: unknown;
  "market": string;
  "interval": string;
  "limit"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
}

export interface GetMarketStatisticsParams {
  [field: string]: unknown;
  "market": string;
}

export interface GetMarketStatisticsResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: MarketStatsModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetMarketTradesParams {
  [field: string]: unknown;
  "market": string;
}

export interface GetMarketsParams {
  [field: string]: unknown;
  "market"?: Array<string> | null;
}

export interface GetMarketsResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<MarketModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetMarketsStatsParams {
  [field: string]: unknown;
  "market[]"?: Array<string> | null;
}

export interface GetNextClaimTimeParams {
  [field: string]: unknown;
}

export interface GetOpenInterestsParams {
  [field: string]: unknown;
  "market": string;
  "interval"?: string | null;
  "startTime"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetOpenOrdersParams {
  [field: string]: unknown;
  "market"?: Array<string> | null;
  "type"?: OrderType | null;
  "side"?: OrderSide | null;
}

export interface GetOpenOrdersResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<OpenOrderModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetOrderByExternalIdParams {
  [field: string]: unknown;
  "externalId": string;
}

export interface GetOrderByIdParams {
  [field: string]: unknown;
  "order_id": string;
}

export interface GetOrderByIdResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: OpenOrderModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetOrderParams {
  [field: string]: unknown;
  "id": string;
}

export interface GetOrderbookSnapshotParams {
  [field: string]: unknown;
  "market": string;
}

export interface GetOrderbookSnapshotResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: OrderbookUpdateModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetOrdersHistoryParams {
  [field: string]: unknown;
  "market"?: Array<string> | null;
  "type"?: OrderType | null;
  "side"?: OrderSide | null;
  "cursor"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "sort"?: OrderSortBy | null;
  "id"?: number | bigint | string | null;
  "externalId"?: string | null;
}

export interface GetOrdersHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<OpenOrderModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetPartnerRewardsParams {
  [field: string]: unknown;
}

export interface GetPositionsHistoryParams {
  [field: string]: unknown;
  "market"?: Array<string> | null;
  "side"?: PositionSide | null;
  "cursor"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetPositionsHistoryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<PositionHistoryModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetPositionsParams {
  [field: string]: unknown;
  "market"?: Array<string> | null;
  "side"?: PositionSide | null;
}

export interface GetPositionsResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<PositionModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetPublicLeaderboardParams {
  [field: string]: unknown;
  "period": LeaderboardInterval;
}

export interface GetPublicReferralLinkParams {
  [field: string]: unknown;
  "id": string;
}

export interface GetRebateStatsParams {
  [field: string]: unknown;
}

export interface GetRebatesCsvParams {
  [field: string]: unknown;
}

export interface GetReferralDashboardParams {
  [field: string]: unknown;
  "period": ReferralDashboardTimeRange;
}

export interface GetReferralLinksParams {
  [field: string]: unknown;
}

export interface GetReferralRebatesByLinkCsvParams {
  [field: string]: unknown;
  "period": ReferralDashboardTimeRange;
  "links[]"?: Array<string> | null;
}

export interface GetReferralRebatesByPeriodCsvParams {
  [field: string]: unknown;
  "period": ReferralDashboardTimeRange;
  "granularity": ReferralDashboardGranularity;
  "links[]"?: Array<string> | null;
  "subaffiliates[]"?: Array<string> | null;
}

export interface GetReferralRebatesBySubaffiliateCsvParams {
  [field: string]: unknown;
  "period": ReferralDashboardTimeRange;
  "subaffiliates[]"?: Array<string> | null;
}

export interface GetReferralRebatesByUserCsvParams {
  [field: string]: unknown;
  "period": ReferralDashboardTimeRange;
  "links[]"?: Array<string> | null;
  "subaffiliates[]"?: Array<string> | null;
}

export interface GetReferralStatusParams {
  [field: string]: unknown;
}

export interface GetRewardLeaderboardStatsParams {
  [field: string]: unknown;
}

export interface GetSettingsParams {
  [field: string]: unknown;
}

export interface GetSettingsResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: SettingsModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetSpotBalancesParams {
  [field: string]: unknown;
  "accountId[]"?: Array<number | bigint | string> | null;
  "accountId"?: number | bigint | string | null;
}

export interface GetSpotBalancesResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<SpotBalanceModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetStarknetDomainParams {
  [field: string]: unknown;
}

export interface GetTradesParams {
  [field: string]: unknown;
  "market"?: Array<string> | null;
  "side"?: OrderSide | null;
  "type"?: TradeType | null;
  "cursor"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetTradesResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<AccountTradeModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetTradingCandlesParams {
  [field: string]: unknown;
  "market": string;
  "interval": string;
  "limit"?: number | bigint | string | null;
  "endTime"?: number | bigint | string | null;
}

export interface GetTradingFeeStatsParams {
  [field: string]: unknown;
}

export interface GetUserFundingHistoryParams {
  [field: string]: unknown;
  "market[]"?: Array<string> | null;
  "side"?: PositionSide | null;
  "startTime"?: number | bigint | string | null;
  "cursor"?: string | null;
  "limit"?: number | bigint | string | null;
  "market"?: string | null;
}

export interface GetUserLeaderboardParams {
  [field: string]: unknown;
  "period": LeaderboardInterval;
}

export interface GetUserOrdersExternalExternalIdParams {
  [field: string]: unknown;
  "external_id": string;
}

export interface GetUserOrdersExternalExternalIdResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<OpenOrderModel> | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface GetVaultFundingHistoryParams {
  [field: string]: unknown;
}

export interface GetVaultOrdersParams {
  [field: string]: unknown;
}

export interface GetVaultPerformanceParams {
  [field: string]: unknown;
  "interval": string;
}

export interface GetVaultPerformanceResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GetVaultPerformanceResultData | null;
}

export interface GetVaultPerformanceResultData {
  [field: string]: unknown;
  "totalPnl"?: string | null;
  "aprPercentage"?: string | null;
  "profitFactor"?: string | null;
  "winRate"?: string | null;
  "maxDrawdown"?: string | null;
  "tradingVolume"?: string | null;
  "sharpeRatio"?: string | null;
  "aprBreakdown"?: GetVaultPerformanceResultDataAprBreakdown | null;
}

export interface GetVaultPerformanceResultDataAprBreakdown {
  [field: string]: unknown;
  "baseApr"?: string | null;
  "maxExtraApr"?: string | null;
}

export interface GetVaultPositionHistoryParams {
  [field: string]: unknown;
}

export interface GetVaultPositionsParams {
  [field: string]: unknown;
}

export interface GetVaultSummaryParams {
  [field: string]: unknown;
}

export interface GetVaultSummaryResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GetVaultSummaryResultData | null;
}

export interface GetVaultSummaryResultData {
  [field: string]: unknown;
  "equity"?: string | null;
  "walletBalance"?: string | null;
  "lastMonthApr"?: string | null;
  "exposure"?: string | null;
  "numberOfDepositors"?: number | bigint | string | null;
  "ageDays"?: number | bigint | string | null;
  "profitShare"?: string | null;
  "aprBreakdown"?: GetVaultSummaryResultDataAprBreakdown | null;
}

export interface GetVaultSummaryResultDataAprBreakdown {
  [field: string]: unknown;
  "baseApr"?: string | null;
  "maxExtraApr"?: string | null;
}

export interface GetVaultTradesParams {
  [field: string]: unknown;
}

export interface GetWalletValidationParams {
  [field: string]: unknown;
  "id": string;
}

export interface GetWithdrawalsParams {
  [field: string]: unknown;
}

export interface GetWrapperParams {
  [field: string]: unknown;
}

export interface L2ConfigModel {
  [field: string]: unknown;
  "type": string;
  "collateralId": string;
  "collateralResolution": number | bigint | string;
  "syntheticId": string;
  "syntheticResolution": number | bigint | string;
}

export interface LimitOrderSettlementModel {
  [field: string]: unknown;
  "baseAmount": number | bigint | string;
  "quoteAmount": number | bigint | string;
  "feeAmount": number | bigint | string;
  "baseAssetId": string;
  "quoteAssetId": string;
  "feeAssetId": string;
  "expirationTimestamp": number | bigint | string;
  "nonce": number | bigint | string;
  "receiverPositionId": number | bigint | string;
  "senderPositionId": number | bigint | string;
  "signature": SettlementSignatureModel;
}

export interface MarkWalletCheckedParams {
  [field: string]: unknown;
  "passed": boolean;
}

export interface MarketModel {
  [field: string]: unknown;
  "name": string;
  "type": MarketType;
  "assetName": string;
  "assetPrecision": number | bigint | string;
  "collateralAssetName": string;
  "collateralAssetPrecision": number | bigint | string;
  "active": boolean;
  "isRfq": boolean;
  "isOffHours": boolean;
  "tradingHours": MarketTradingHours;
  "marketStats": MarketStatsModel;
  "tradingConfig": TradingConfigModel;
  "l2Config": L2ConfigModel;
}

export interface MarketStatsModel {
  [field: string]: unknown;
  "dailyVolume": string;
  "dailyVolumeBase": string;
  "dailyPriceChange": string;
  "dailyLow": string;
  "dailyHigh": string;
  "lastPrice": string;
  "askPrice": string;
  "bidPrice": string;
  "markPrice": string;
  "indexPrice": string;
  "fundingRate": string;
  "nextFundingRate": number | bigint | string;
  "openInterest": string;
  "openInterestBase": string;
}

export interface MassCancelParams {
  [field: string]: unknown;
  "body": OrderCancelRequest;
}

export interface MassCancelResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: EmptyModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface OnChainTransfer {
  [field: string]: unknown;
  "fromVault": string;
  "toVault": string;
  "amount": string;
  "transferredAsset": string;
}

export interface OnboardClientParams {
  [field: string]: unknown;
  "body": HttpValue;
}

export interface OnboardClientResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: OnboardedClientModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface OnboardSubaccountParams {
  [field: string]: unknown;
  "L1_SIGNATURE": string;
  "L1_MESSAGE_TIME": string;
  "body": HttpValue;
}

export interface OnboardSubaccountResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: AccountModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface OnboardedClientModel {
  [field: string]: unknown;
  "l1Address": string;
  "defaultAccount"?: AccountModel;
}

export interface OnchainWithdrawalParams {
  [field: string]: unknown;
  "body": CollateralSlowWithdrawal;
}

export interface OpenOrderModel {
  [field: string]: unknown;
  "id": number | bigint | string;
  "accountId": number | bigint | string;
  "externalId": string;
  "market": string;
  "type": OrderType;
  "side": OrderSide;
  "status": OrderStatus;
  "statusReason"?: OrderStatusReason | null;
  "price"?: string | null;
  "averagePrice"?: string | null;
  "rfqStartPrice"?: string | null;
  "qty": string;
  "filledQty"?: string | null;
  "cancelledQty"?: string | null;
  "reduceOnly": boolean;
  "postOnly": boolean;
  "chaseOrder"?: boolean | null;
  "payedFee"?: string | null;
  "createdTime": number | bigint | string;
  "updatedTime": number | bigint | string;
  "expiryTime"?: number | bigint | string | null;
  "timeInForce": TimeInForce;
  "tpSlType"?: OrderTpslType | null;
  "takeProfit"?: OpenOrderTpslTriggerModel | null;
  "stopLoss"?: OpenOrderTpslTriggerModel | null;
}

export interface OpenOrderTpslTriggerModel {
  [field: string]: unknown;
  "triggerPrice": string;
  "triggerPriceType": OrderTriggerPriceType;
  "price": string;
  "priceType": OrderPriceType;
  "status"?: OrderStatus | null;
}

export interface OrderCancelRequest {
  [field: string]: unknown;
  "market"?: string | null;
  "side"?: OrderSide | null;
  "ids"?: Array<number | bigint | string> | null;
  "externalIds"?: Array<string> | null;
}

export interface OrderConditionalTrigger {
  [field: string]: unknown;
  "triggerPrice"?: string | null;
  "triggerType"?: string | null;
}

export interface OrderbookQuantityModel {
  [field: string]: unknown;
  "qty"?: string | null;
  "price"?: string | null;
}

export interface OrderbookUpdateModel {
  [field: string]: unknown;
  "market"?: string | null;
  "bid"?: Array<OrderbookQuantityModel> | null;
  "ask"?: Array<OrderbookQuantityModel> | null;
}

export interface PlaceOrderParams {
  [field: string]: unknown;
  "body": CreateOrderRequest;
}

export interface PlaceOrderResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: PlacedOrderModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface PlaceRfqOrderParams {
  [field: string]: unknown;
  "body": CreateRFQOrderRequest;
}

export interface PlaceRfqOrderResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: PlacedOrderModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface PlacedOrderModel {
  [field: string]: unknown;
  "id": number | bigint | string;
  "externalId": string;
}

export interface PositionHistoryModel {
  [field: string]: unknown;
  "id": number | bigint | string;
  "accountId": number | bigint | string;
  "market": string;
  "side": PositionSide;
  "size": string;
  "maxPositionSize": string;
  "leverage": string;
  "openPrice": string;
  "exitPrice"?: string | null;
  "realisedPnl": string;
  "realisedPnlBreakdown": RealisedPnlBreakdownModel;
  "createdTime": number | bigint | string;
  "exitType"?: ExitType | null;
  "closedTime"?: number | bigint | string | null;
}

export interface PositionModel {
  [field: string]: unknown;
  "id": number | bigint | string;
  "accountId": number | bigint | string;
  "market": string;
  "status": PositionStatus;
  "side": PositionSide;
  "leverage": string;
  "size": string;
  "value": string;
  "openPrice": string;
  "markPrice": string;
  "liquidationPrice"?: string | null;
  "unrealisedPnl": string;
  "realisedPnl": string;
  "tpPrice"?: string | null;
  "slPrice"?: string | null;
  "adl"?: number | bigint | string | null;
  "createdAt": number | bigint | string;
  "updatedAt": number | bigint | string;
}

export interface PostUserTransferOnchainParams {
  [field: string]: unknown;
  "body": OnChainTransfer;
}

export interface PostUserTransferOnchainResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: TransferResponseModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface QuoteModel {
  [field: string]: unknown;
  "id": string;
  "fee": string;
}

export interface RealisedPnlBreakdownModel {
  [field: string]: unknown;
  "tradePnl": string;
  "fundingFees": string;
  "openFees": string;
  "closeFees": string;
}

export interface RiskFactorConfigModel {
  [field: string]: unknown;
  "upperBound": string;
  "riskFactor": string;
}

export interface SetDeadmanSwitchParams {
  [field: string]: unknown;
  "countdownTime": number | bigint | string;
}

export interface SettingsModel {
  [field: string]: unknown;
  "starkExContractAddress": string;
}

export interface SettlementSignatureModel {
  [field: string]: unknown;
  "r": string;
  "s": string;
}

export interface SpotBalanceModel {
  [field: string]: unknown;
  "accountId": number | bigint | string;
  "asset": string;
  "balance": string;
  "indexPrice": string;
  "notionalValue": string;
  "contributionFactor": string;
  "equityContribution": string;
  "availableToWithdraw"?: string | null;
  "absolutePnl"?: string | null;
  "pnlPercentage"?: string | null;
  "averageEntryPrice"?: string | null;
  "updatedAt": number | bigint | string;
}

export interface TradingConfigModel {
  [field: string]: unknown;
  "minOrderSize": string;
  "minOrderSizeChange": string;
  "minPriceChange": string;
  "maxMarketOrderValue": string;
  "maxLimitOrderValue": string;
  "maxPositionValue": string;
  "maxLeverage": string;
  "maxNumOrders": number | bigint | string;
  "limitPriceCap": string;
  "limitPriceFloor": string;
  "riskFactorConfig": Array<RiskFactorConfigModel>;
}

export interface TradingFeeModel {
  [field: string]: unknown;
  "market": string;
  "makerFeeRate": string;
  "takerFeeRate": string;
  "builderFeeRate": string;
}

export interface TransferParams {
  [field: string]: unknown;
  "body": TransferRequest;
}

export interface TransferPatchParams {
  [field: string]: unknown;
  "body": TransferRequest;
}

export interface TransferRequest {
  [field: string]: unknown;
  "fromAccount": number | bigint | string;
  "toAccount": number | bigint | string;
  "amount": string;
  "transferredAsset": string;
  "settlement"?: Record<string, HttpValue> | null;
}

export interface TransferResponseModel {
  [field: string]: unknown;
  "validSignature": boolean;
  "id"?: HttpValue | null;
  "hashCalculated"?: HttpValue | null;
  "starkExRepresentation"?: HttpValue | null;
}

export interface Twap {
  [field: string]: unknown;
  "numParts"?: number | bigint | string | null;
  "interval"?: number | bigint | string | null;
}

export interface UpdateAccountInfoParams {
  [field: string]: unknown;
  "body": AccountUpdateRequest;
}

export interface UpdateFundingRateConfigParams {
  [field: string]: unknown;
  "body": FundingRateConfigRequest;
}

export interface UpdateLeverageParams {
  [field: string]: unknown;
  "body": AccountLeverage;
}

export interface UpdateLeverageResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: EmptyModel | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface UpdateReferralLinkCodeRequest {
  [field: string]: unknown;
  "code": string;
  "description"?: string | null;
}

export interface UpdateReferralLinkParams {
  [field: string]: unknown;
  "body": UpdateReferralLinkCodeRequest;
}

export interface UseReferralCodeParams {
  [field: string]: unknown;
  "body": UseReferralCodeRequest;
}

export interface UseReferralCodeRequest {
  [field: string]: unknown;
  "code": string;
}

export interface ValidateWalletParams {
  [field: string]: unknown;
  "body": WalletValidationRequest;
}

export interface WalletValidationRequest {
  [field: string]: unknown;
  "address": string;
}

export interface WithdrawFromVaultParams {
  [field: string]: unknown;
  "body": WithdrawRequestModel;
}

export interface WithdrawFromVaultResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: HttpValue | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface WithdrawParams {
  [field: string]: unknown;
  "body": WithdrawalRequest;
}

export interface WithdrawRequestModel {
  [field: string]: unknown;
  "fromAccountId": number | bigint | string;
  "toAccountId": number | bigint | string;
  "collateral": string;
  "shares": string;
  "settlement": LimitOrderSettlementModel;
}

export interface WithdrawResult {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: number | bigint | string | null;
  "error"?: HttpValue | null;
  "pagination"?: Record<string, HttpValue> | null;
}

export interface WithdrawalRequest {
  [field: string]: unknown;
  "amount": string;
  "asset": string;
  "destination": string;
  "nonce"?: number | bigint | string;
}
