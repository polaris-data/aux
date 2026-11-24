// Generated from schema/monaco.json.
import type { HttpValue } from "./index";

export interface AccountBalance {
  [field: string]: unknown;
  "token"?: string | null;
  "symbol"?: string | null;
  "decimals"?: number | bigint | null;
  "availableBalance"?: string | null;
  "lockedBalance"?: string | null;
  "totalBalance"?: string | null;
  "availableBalanceRaw"?: string | null;
  "lockedBalanceRaw"?: string | null;
  "assetId"?: string | null;
  "totalBalanceRaw"?: string | null;
  "wrappedNative"?: boolean | null;
  "version"?: number | bigint | null;
}

export interface BatchCancelAllResponse {
  [field: string]: unknown;
  "totalRequested"?: number | bigint | null;
  "totalCancelled"?: number | bigint | null;
  "totalFailed"?: number | bigint | null;
  "results"?: Array<BatchCancelResult> | null;
}

export interface BatchCancelError {
  [field: string]: unknown;
  "code"?: string | null;
  "message"?: string | null;
}

export interface BatchCancelOrdersRequest {
  "orderIds": Array<string>;
}

export interface BatchCancelOrdersResponse {
  [field: string]: unknown;
  "totalRequested"?: number | bigint | null;
  "totalCancelled"?: number | bigint | null;
  "totalFailed"?: number | bigint | null;
  "results"?: Array<BatchCancelResult> | null;
}

export interface BatchCancelResult {
  [field: string]: unknown;
  "orderId"?: string | null;
  "cancelledAt"?: string | null;
  "error"?: BatchCancelError | null;
}

export interface BatchCreateError {
  [field: string]: unknown;
  "code"?: string | null;
  "message"?: string | null;
}

export interface BatchCreateOrderItem {
  "tradingPairId": string;
  "orderType": string;
  "side": string;
  "price"?: string | null;
  "quantity": string;
  "tradingMode"?: string | null;
  "slippageToleranceBps"?: number | bigint | null;
  "useMasterBalance"?: boolean | null;
  "expirationDate"?: string | null;
  "timeInForce"?: string | null;
  "marginAccountId"?: string | null;
  "riskBucketId"?: string | null;
  "riskBucketCollateral"?: string | null;
  "positionSide"?: string | null;
  "leverage"?: string | null;
  "reduceOnly"?: boolean | null;
  "strategyKey"?: string | null;
  "marginMode"?: string | null;
  "postOnly"?: boolean | null;
  "clientOrderId"?: string | null;
  "idempotencyKey"?: string | null;
  "selfTradePreventionMode"?: string | null;
}

export interface BatchCreateOrdersRequest {
  "orders": Array<BatchCreateOrderItem>;
}

export interface BatchCreateOrdersResponse {
  [field: string]: unknown;
  "totalRequested"?: number | bigint | null;
  "totalSucceeded"?: number | bigint | null;
  "totalFailed"?: number | bigint | null;
  "results"?: Array<BatchCreateResult> | null;
}

export interface BatchCreateResult {
  [field: string]: unknown;
  "orderId"?: string | null;
  "matchResult"?: MatchResult | null;
  "error"?: BatchCreateError | null;
}

export interface BatchReplaceError {
  [field: string]: unknown;
  "code"?: string | null;
  "message"?: string | null;
}

export interface BatchReplaceOrderItem {
  "orderId": string;
  "price"?: string | null;
  "quantity"?: string | null;
  "useMasterBalance"?: boolean | null;
  "postOnly"?: boolean | null;
  "clientOrderId"?: string | null;
  "selfTradePreventionMode"?: string | null;
}

export interface BatchReplaceOrdersRequest {
  "orders": Array<BatchReplaceOrderItem>;
}

export interface BatchReplaceOrdersResponse {
  [field: string]: unknown;
  "totalRequested"?: number | bigint | null;
  "totalSucceeded"?: number | bigint | null;
  "totalFailed"?: number | bigint | null;
  "results"?: Array<BatchReplaceResult> | null;
}

export interface BatchReplaceResult {
  [field: string]: unknown;
  "originalOrderId"?: string | null;
  "newOrderId"?: string | null;
  "updatedFields"?: UpdatedFields | null;
  "matchResult"?: MatchResult | null;
  "error"?: BatchReplaceError | null;
}

export interface CancelOrderRequest {
  "orderId": string;
  "remainingQuantityTarget"?: string | null;
}

export interface CancelOrderResponse {
  [field: string]: unknown;
  "orderId"?: string | null;
  "status"?: string | null;
  "message"?: string | null;
}

export interface ChallengeRequest {
  "address": string;
  "clientId"?: string | null;
  "chainId"?: string | null;
  "sessionPublicKey": string;
}

export interface ChallengeResponse {
  [field: string]: unknown;
  "nonce"?: string | null;
  "message"?: string | null;
  "expiresAt"?: number | bigint | null;
}

export interface CreateOrderRequest {
  "tradingPairId": string;
  "orderType": string;
  "side": string;
  "price"?: string | null;
  "quantity": string;
  "tradingMode"?: string | null;
  "slippageToleranceBps"?: number | bigint | null;
  "useMasterBalance"?: boolean | null;
  "expirationDate"?: string | null;
  "timeInForce"?: string | null;
  "marginAccountId"?: string | null;
  "riskBucketId"?: string | null;
  "riskBucketCollateral"?: string | null;
  "positionSide"?: string | null;
  "leverage"?: string | null;
  "reduceOnly"?: boolean | null;
  "takeProfit"?: ParentTpSlLeg | null;
  "stopLoss"?: ParentTpSlLeg | null;
  "strategyKey"?: string | null;
  "marginMode"?: string | null;
  "postOnly"?: boolean | null;
  "clientOrderId"?: string | null;
  "idempotencyKey"?: string | null;
  "selfTradePreventionMode"?: string | null;
  "trailingStop"?: ParentTrailingStopLeg | null;
}

export interface CreateOrderResponse {
  [field: string]: unknown;
  "orderId"?: string | null;
  "status"?: string | null;
  "message"?: string | null;
  "matchResult"?: MatchResult | null;
  "takeProfitOrderId"?: string | null;
  "stopLossOrderId"?: string | null;
  "marginAccountId"?: string | null;
  "riskBucketId"?: string | null;
  "strategyKey"?: string | null;
  "delegationId"?: string | null;
  "trailingStopOrderId"?: string | null;
}

export interface ExecutionPriceRange {
  [field: string]: unknown;
  "bestPrice"?: string | null;
  "worstPrice"?: string | null;
}

export interface FundingRecord {
  [field: string]: unknown;
  "tradingPairId"?: string | null;
  "epoch"?: string | null;
  "fundingRate"?: string | null;
  "fundingDeltaPerUnit"?: string | null;
  "cumulativeFundingPerUnit"?: string | null;
  "referencePrice"?: string | null;
  "sampleCount"?: number | bigint | null;
  "windowStartedAt"?: string | null;
  "windowClosedAt"?: string | null;
  "settledAt"?: string | null;
  "carried"?: boolean | null;
}

export interface GetBalancesResponse {
  [field: string]: unknown;
  "balances"?: Array<AccountBalance> | null;
  "total"?: number | bigint | null;
  "pageSize"?: number | bigint | null;
  "totalPages"?: number | bigint | null;
  "nextPageToken"?: string | null;
}

export interface GetFundingStateResponse {
  [field: string]: unknown;
  "tradingPairId"?: string | null;
  "currentFundingRate"?: string | null;
  "estimatedNextFundingRate"?: string | null;
  "nextFundingTime"?: string | null;
  "fundingIntervalSeconds"?: number | bigint | null;
  "lastFundingTime"?: string | null;
  "updatedAt"?: string | null;
}

export interface GetIndexPriceResponse {
  [field: string]: unknown;
  "tradingPairId"?: string | null;
  "indexPrice"?: string | null;
  "components"?: Array<IndexComponent> | null;
  "updatedAt"?: string | null;
}

export interface GetMarginAccountSummaryResponse {
  [field: string]: unknown;
  "marginAccountId"?: string | null;
  "equity"?: string | null;
  "initialMarginRequired"?: string | null;
  "maintenanceMarginRequired"?: string | null;
  "freeCollateral"?: string | null;
  "withdrawableCollateral"?: string | null;
  "totalPositionNotional"?: string | null;
  "unrealizedPnl"?: string | null;
  "realizedPnl"?: string | null;
  "updatedAt"?: string | null;
  "label"?: string | null;
  "tradingPairId"?: string | null;
  "strategyKey"?: string | null;
  "riskBucketId"?: string | null;
  "marginMode"?: string | null;
  "selectedTradingPairIds"?: Array<string> | null;
  "availableOrderCollateral"?: string | null;
  "copyFollowId"?: string | null;
  "copyLeaderHandle"?: string | null;
  "riskBucketState"?: string | null;
  "subaccountKey"?: string | null;
  "marginAccountState"?: string | null;
  "maxRemovableCollateral"?: string | null;
}

export interface GetMarkPriceResponse {
  [field: string]: unknown;
  "tradingPairId"?: string | null;
  "markPrice"?: string | null;
  "oracleProvider"?: string | null;
  "oracleEpoch"?: string | null;
  "updatedAt"?: string | null;
  "regime"?: string | null;
}

export interface GetMarketMetadataResponse {
  [field: string]: unknown;
  "baseIconUrl"?: string | null;
  "high24h"?: string | null;
  "lastPrice"?: string | null;
  "lastPriceTimestamp"?: string | null;
  "low24h"?: string | null;
  "marketInitializationTimestamp"?: string | null;
  "priceChange24h"?: string | null;
  "priceChangePercent24h"?: string | null;
  "quoteIconUrl"?: string | null;
  "symbol"?: string | null;
  "volume24h"?: string | null;
  "minLeverage"?: string | null;
  "maxLeverage"?: string | null;
  "markPrice"?: string | null;
  "indexPrice"?: string | null;
  "totalBaseVolumeLtd"?: string | null;
  "totalQuoteVolumeLtd"?: string | null;
  "totalTradeCountLtd"?: number | bigint | null;
  "quoteVolume24h"?: string | null;
}

export interface GetOrderResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "tradingPairId"?: string | null;
  "orderType"?: string | null;
  "side"?: string | null;
  "price"?: string | null;
  "quantity"?: string | null;
  "filledQuantity"?: string | null;
  "averageFillPrice"?: string | null;
  "status"?: string | null;
  "tradingMode"?: string | null;
  "timeInForce"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "expirationDate"?: string | null;
  "applicationTakerFee"?: string | null;
  "monacoTakerFee"?: string | null;
  "monacoMakerRebate"?: string | null;
  "totalTakerFees"?: string | null;
  "takerTotalPayment"?: string | null;
  "makerTotalReceipt"?: string | null;
  "marginAccountId"?: string | null;
  "positionSide"?: string | null;
  "leverage"?: string | null;
  "reduceOnly"?: boolean | null;
  "positionId"?: string | null;
  "quantityRaw"?: string | null;
  "filledQuantityRaw"?: string | null;
  "remainingQuantity"?: string | null;
  "remainingQuantityRaw"?: string | null;
  "clientOrderId"?: string | null;
  "cancelledAt"?: string | null;
  "filledAt"?: string | null;
  "expiredAt"?: string | null;
  "submittedAt"?: string | null;
  "acknowledgedAt"?: string | null;
  "triggerPrice"?: string | null;
  "quoteVolume"?: string | null;
  "applicationId"?: string | null;
  "parentOrderId"?: string | null;
  "batchId"?: string | null;
  "terminalReason"?: string | null;
  "postOnly"?: boolean | null;
  "version"?: number | bigint | null;
  "selfTradePreventionMode"?: string | null;
  "conditionalOrderId"?: string | null;
  "origin"?: string | null;
  "placementTakerFeeBps"?: string | null;
  "placementMakerFeeBps"?: string | null;
}

export interface GetOrderbookResponse {
  [field: string]: unknown;
  "baseDecimals"?: number | bigint | null;
  "baseToken"?: string | null;
  "data"?: OrderbookData | null;
  "eventType"?: string | null;
  "symbol"?: string | null;
  "tradingPairId"?: string | null;
  "quoteDecimals"?: number | bigint | null;
  "quoteToken"?: string | null;
  "sequenceNumber"?: number | bigint | null;
  "timestamp"?: string | null;
  "tradingMode"?: string | null;
  "bookSeq"?: string | null;
}

export interface GetTradesResponse {
  [field: string]: unknown;
  "trades"?: Array<PublicTrade> | null;
  "pageSize"?: number | bigint | null;
  "nextPageToken"?: string | null;
}

export interface GetTradingPairResponse {
  [field: string]: unknown;
  "tradingPair"?: TradingPairData | null;
}

export interface IndexComponent {
  [field: string]: unknown;
  "provider"?: string | null;
  "price"?: string | null;
  "weight"?: string | null;
  "updatedAt"?: string | null;
}

export interface ListFundingHistoryResponse {
  [field: string]: unknown;
  "records"?: Array<FundingRecord> | null;
  "total"?: number | bigint | null;
  "pageSize"?: number | bigint | null;
  "nextPageToken"?: string | null;
  "totalCapped"?: boolean | null;
}

export interface ListMarginAccountsResponse {
  [field: string]: unknown;
  "accounts"?: Array<MarginAccountSummary> | null;
  "total"?: number | bigint | null;
  "pageSize"?: number | bigint | null;
  "nextPageToken"?: string | null;
}

export interface ListOrdersResponse {
  [field: string]: unknown;
  "orders"?: Array<GetOrderResponse> | null;
  "total"?: number | bigint | null;
  "pageSize"?: number | bigint | null;
  "totalPages"?: number | bigint | null;
  "nextPageToken"?: string | null;
  "totalCapped"?: boolean | null;
}

export interface ListPositionsResponse {
  [field: string]: unknown;
  "positions"?: Array<Position> | null;
  "total"?: number | bigint | null;
  "pageSize"?: number | bigint | null;
  "nextPageToken"?: string | null;
  "totalCapped"?: boolean | null;
}

export interface ListTradingPairsResponse {
  [field: string]: unknown;
  "tradingPairs"?: Array<TradingPairData> | null;
  "page"?: number | bigint | null;
  "pageSize"?: number | bigint | null;
  "total"?: number | bigint | null;
  "totalPages"?: number | bigint | null;
}

export interface MarginAccountSummary {
  [field: string]: unknown;
  "marginAccountId"?: string | null;
  "equity"?: string | null;
  "initialMarginRequired"?: string | null;
  "maintenanceMarginRequired"?: string | null;
  "freeCollateral"?: string | null;
  "withdrawableCollateral"?: string | null;
  "totalPositionNotional"?: string | null;
  "unrealizedPnl"?: string | null;
  "realizedPnl"?: string | null;
  "updatedAt"?: string | null;
  "label"?: string | null;
  "tradingPairId"?: string | null;
  "strategyKey"?: string | null;
  "riskBucketId"?: string | null;
  "marginMode"?: string | null;
  "selectedTradingPairIds"?: Array<string> | null;
  "availableOrderCollateral"?: string | null;
  "copyFollowId"?: string | null;
  "copyLeaderHandle"?: string | null;
  "riskBucketState"?: string | null;
  "subaccountKey"?: string | null;
  "marginAccountState"?: string | null;
  "maxRemovableCollateral"?: string | null;
}

export interface MatchResult {
  [field: string]: unknown;
  "tradesCount"?: number | bigint | null;
  "totalFilled"?: string | null;
  "remainingQuantity"?: string | null;
  "averageFillPrice"?: string | null;
  "status"?: string | null;
  "actualSlippageBps"?: number | bigint | null;
  "maxSlippageBps"?: number | bigint | null;
  "executionPriceRange"?: ExecutionPriceRange | null;
}

export interface OrderbookData {
  [field: string]: unknown;
  "bids"?: Array<PriceLevel> | null;
  "asks"?: Array<PriceLevel> | null;
  "bestBid"?: string | null;
  "bestAsk"?: string | null;
  "bidVolume"?: string | null;
  "askVolume"?: string | null;
  "priceChange"?: PriceChange | null;
}

export interface ParentTpSlLeg {
  [field: string]: unknown;
  "triggerPrice"?: string | null;
  "orderType"?: string | null;
  "limitPrice"?: string | null;
  "timeInForce"?: string | null;
  "slippageToleranceBps"?: number | bigint | null;
  "expiresAt"?: string | null;
  "quantity"?: string | null;
  "closePosition"?: boolean | null;
}

export interface ParentTrailingStopLeg {
  [field: string]: unknown;
  "trailBps"?: number | bigint | null;
  "activationPrice"?: string | null;
  "quantity"?: string | null;
  "closePosition"?: boolean | null;
  "slippageToleranceBps"?: number | bigint | null;
  "expiresAt"?: string | null;
}

export interface Position {
  [field: string]: unknown;
  "positionId"?: string | null;
  "marginAccountId"?: string | null;
  "tradingPairId"?: string | null;
  "side"?: string | null;
  "size"?: string | null;
  "entryPrice"?: string | null;
  "markPrice"?: string | null;
  "indexPrice"?: string | null;
  "unrealizedPnl"?: string | null;
  "realizedPnl"?: string | null;
  "isolatedMargin"?: string | null;
  "leverage"?: string | null;
  "maintenanceMarginRequired"?: string | null;
  "initialMarginRequired"?: string | null;
  "liquidationPrice"?: string | null;
  "status"?: string | null;
  "updatedAt"?: string | null;
  "riskBucketId"?: string | null;
  "marginMode"?: string | null;
  "netRealizedPnl"?: string | null;
  "realizedRoe"?: string | null;
  "exitPrice"?: string | null;
  "fundingPaid"?: string | null;
  "feesPaid"?: string | null;
  "cumFees"?: string | null;
  "version"?: number | bigint | null;
  "openedAt"?: string | null;
  "lifetimeTotalsStatus"?: string | null;
}

export interface PriceChange {
  [field: string]: unknown;
  "price24hAgo"?: string | null;
  "priceChange24h"?: string | null;
  "priceChangePercent24h"?: string | null;
}

export interface PriceLevel {
  [field: string]: unknown;
  "price"?: string | null;
  "quantity"?: string | null;
  "orderCount"?: number | bigint | null;
}

export interface PublicTrade {
  [field: string]: unknown;
  "data"?: TradeData | null;
  "eventType"?: string | null;
  "tradingPairId"?: string | null;
  "tradingMode"?: string | null;
}

export interface RefreshResponse {
  [field: string]: unknown;
  "expiresAt"?: number | bigint | null;
}

export interface ReplaceOrderRequest {
  "selfTradePreventionMode"?: string | null;
  "price"?: string | null;
  "quantity"?: string | null;
  "useMasterBalance"?: boolean | null;
  "postOnly"?: boolean | null;
  "clientOrderId"?: string | null;
}

export interface ReplaceOrderResponse {
  [field: string]: unknown;
  "orderId"?: string | null;
  "status"?: string | null;
  "message"?: string | null;
  "originalOrderId"?: string | null;
  "updatedFields"?: UpdatedFields | null;
  "matchResult"?: MatchResult | null;
}

export interface RevokeResponse {
  [field: string]: unknown;
  "message"?: string | null;
}

export interface TradeData {
  [field: string]: unknown;
  "executedAt"?: string | null;
  "makerSide"?: string | null;
  "price"?: string | null;
  "quantity"?: string | null;
  "quantityRaw"?: string | null;
  "tradeId"?: string | null;
  "executionSource"?: string | null;
  "makerAddress"?: string | null;
  "takerAddress"?: string | null;
  "takerFeeRaw"?: string | null;
  "quoteVolumeRaw"?: string | null;
  "takerApplicationId"?: string | null;
}

export interface TradingPairData {
  [field: string]: unknown;
  "baseAssetId"?: string | null;
  "baseDecimals"?: number | bigint | null;
  "baseIconUrl"?: string | null;
  "baseToken"?: string | null;
  "baseTokenContract"?: string | null;
  "id"?: string | null;
  "isActive"?: boolean | null;
  "makerFeeBps"?: number | bigint | null;
  "marketType"?: string | null;
  "maxOrderSize"?: string | null;
  "minOrderSize"?: string | null;
  "quoteAssetId"?: string | null;
  "quoteDecimals"?: number | bigint | null;
  "quoteIconUrl"?: string | null;
  "quoteToken"?: string | null;
  "quoteTokenContract"?: string | null;
  "symbol"?: string | null;
  "takerFeeBps"?: number | bigint | null;
  "tickSize"?: string | null;
  "minLeverage"?: string | null;
  "maxLeverage"?: string | null;
  "category"?: string | null;
  "quantityStepSize"?: string | null;
  "baseAssetName"?: string | null;
  "quoteAssetName"?: string | null;
  "isolatedOnly"?: boolean | null;
}

export interface UpdatedFields {
  [field: string]: unknown;
  "price"?: string | null;
  "quantity"?: string | null;
}

export interface UserInfo {
  [field: string]: unknown;
  "id"?: string | null;
  "address"?: string | null;
  "username"?: string | null;
}

export interface VerifyRequest {
  "address": string;
  "signature": string;
  "nonce": string;
  "clientId"?: string | null;
  "chainId"?: string | null;
  "sessionPublicKey": string;
  "referralCode"?: string | null;
  "inviteCode"?: string | null;
}

export interface VerifyResponse {
  [field: string]: unknown;
  "expiresAt"?: number | bigint | null;
  "user"?: UserInfo | null;
  "isNewUser"?: boolean | null;
  "referralApplied"?: boolean | null;
  "referralNotAppliedReason"?: string | null;
}

export interface TradingPairsRequest {
  "page"?: number | bigint | null;
  "pageSize"?: number | bigint | null;
  "marketType"?: string | null;
  "baseToken"?: string | null;
  "quoteToken"?: string | null;
  "isActive"?: boolean | null;
  "category"?: string | null;
}

export interface TradingPairRequest {
  "tradingPairId": string;
}

export interface MarketMetadataRequest {
  "tradingPairId": string;
}

export interface OrderBookRequest {
  "tradingPairId": string;
  "levels"?: number | bigint | null;
  "tradingMode"?: string | null;
  "magnitude"?: string | null;
  "denomination"?: string | null;
}

export interface TradesRequest {
  "tradingPairId": string;
  "pageSize"?: number | bigint | null;
  "pageToken"?: string | null;
}

export interface MarkPriceRequest {
  "tradingPairId": string;
}

export interface IndexPriceRequest {
  "tradingPairId": string;
}

export interface FundingRequest {
  "tradingPairId": string;
}

export interface FundingHistoryRequest {
  "tradingPairId": string;
  "startTime"?: number | bigint | null;
  "endTime"?: number | bigint | null;
  "pageSize"?: number | bigint | null;
  "orderBy"?: string | null;
  "pageToken"?: string | null;
}

export interface BalancesRequest {
  "pageSize"?: number | bigint | null;
  "pageToken"?: string | null;
}

export interface ParentMarginAccountRequest {
  "tradingPairId"?: string | null;
}

export interface MarginAccountsRequest {
  "pageSize"?: number | bigint | null;
  "state"?: string | null;
  "tradingPairId"?: string | null;
  "pageToken"?: string | null;
}

export interface PositionsRequest {
  "marginAccountId"?: string | null;
  "tradingPairId"?: string | null;
  "status"?: string | null;
  "pageSize"?: number | bigint | null;
  "pageToken"?: string | null;
}

export interface OrdersRequest {
  "pageSize"?: number | bigint | null;
  "status"?: string | null;
  "tradingPairId"?: string | null;
  "tradingMode"?: string | null;
  "marginAccountId"?: string | null;
  "orderBy"?: string | null;
  "pageToken"?: string | null;
}

export interface OrderRequest {
  "orderId": string;
}

export interface EmptyRequest {
}

export interface PlaceOrderParams {
  "market": TradingPairData;
  "body": CreateOrderRequest;
}

export interface ReplaceOrderParams {
  "market": TradingPairData;
  "body": ReplaceOrderRequest;
  "orderId": string;
}

export interface BatchCreateParams {
  "markets": Array<TradingPairData>;
  "body": BatchCreateOrdersRequest;
}

export interface BatchReplaceParams {
  "markets": Array<TradingPairData>;
  "body": BatchReplaceOrdersRequest;
}

export interface CancelAllParams {
  "tradingPairId"?: string | null;
}
