// Generated from src/exchanges/polymarket/schema/polymarket.json.
import type { HttpValue } from "./index";
export type BridgeAddress = string;
export type BuilderGatewayAcceptRfqResponse = HttpValue;
export type BuilderGatewayCreateRfqResponse = HttpValue;
export type BuilderGatewayGetRfqResponse = HttpValue;
export type ClobClobRewards = Record<string, HttpValue>;
export type ClobDeleteApiKeyResponse = string;
export type ClobDropNotificationsResponse = string;
export type ClobGetBooksGetResponse = Array<ClobOrderBookSummary>;
export type ClobGetBooksPostResponse = Array<ClobOrderBookSummary>;
export type ClobGetBuilderApiKeysResponse = Array<string>;
export type ClobGetCurrentRebatedFeesResponse = Array<ClobRebatedFees>;
export type ClobGetLastTradesPricesGetResponse = Array<ClobGetLastTradesPricesGetResponseValueItem>;
export type ClobGetLastTradesPricesPostResponse = Array<ClobGetLastTradesPricesPostResponseValueItem>;
export type ClobGetMarketsLiveActivityResponse = Array<ClobLiveActivityMarket>;
export type ClobGetMidpointsGetResponse = Record<string, string>;
export type ClobGetMidpointsPostResponse = Record<string, string>;
export type ClobGetNotificationsResponse = Array<ClobNotification>;
export type ClobGetOrdersScoringResponse = Record<string, boolean>;
export type ClobGetPricesGetResponse = Record<string, Record<string, number | bigint | string>>;
export type ClobGetPricesPostResponse = Record<string, Record<string, number | bigint | string>>;
export type ClobGetRewardPercentagesForUserResponse = Record<string, number | bigint | string>;
export type ClobGetSpreadsResponse = Record<string, string>;
export type ClobGetTimeResponse = number | bigint | string;
export type ClobGetTotalEarningsForUserForDayResponse = Array<ClobTotalUserEarning>;
export type ClobListSessionSignersResponse = HttpValue;
export type ClobOrdersScoringResponse = Record<string, boolean>;
export type ClobPostOrdersResponse = Array<ClobSendOrderResponse>;
export type ClobPostOrdersScoringResponse = Record<string, boolean>;
export type ClobRevokeBuilderApiKeyResponse = string;
export type CollateralReturnPlanResponse = HttpValue;
export type CollateralReturnSubmitResponse = HttpValue;
export type CombosRfqDirection = string;
export type CombosRfqRFQStatus = string;
export type CombosRfqSide = string;
export type CombosRfqSignatureType = number | bigint | string;
export type DataErrorCode = string;
export type DataGetLeaderboardResponse = HttpValue;
export type DataLeaderboardResponse = HttpValue;
export type DataTokenModule = string;
export type GammaGetAbridgedMarketsResponse = Array<GammaMarket>;
export type GammaGetCommentsByIdResponse = Array<GammaComment>;
export type GammaGetCommentsByUserAddressResponse = Array<GammaComment>;
export type GammaGetEventTagsResponse = Array<GammaTag>;
export type GammaGetGammaStatusResponse = HttpValue;
export type GammaGetMarketTagsResponse = Array<GammaTag>;
export type GammaGetMarketsInformationResponse = Array<GammaMarket>;
export type GammaGetRelatedTagsByIdResponse = Array<GammaRelatedTag>;
export type GammaGetRelatedTagsBySlugResponse = Array<GammaRelatedTag>;
export type GammaGetSportsMetadataResponse = Array<GammaSportsMetadata>;
export type GammaGetTagsRelatedToATagByIdResponse = Array<GammaTag>;
export type GammaGetTagsRelatedToATagBySlugResponse = Array<GammaTag>;
export type GammaListCommentsResponse = Array<GammaComment>;
export type GammaListEventCreatorsResponse = Array<GammaEventCreator>;
export type GammaListEventsResponse = Array<GammaEvent>;
export type GammaListMarketsResponse = Array<GammaMarket>;
export type GammaListSeriesResponse = Array<GammaSeries>;
export type GammaListSportEventsResultsResponse = Array<GammaEvent>;
export type GammaListTagsResponse = Array<GammaTag>;
export type GammaListTeamsResponse = Array<GammaTeam>;
export type RelayerAddress = string;
export type RelayerAuthorizationsResponse = HttpValue;
export type RelayerGetExecuteParamsResponse = HttpValue;
export type RelayerGetRelayerApiKeysResponse = Array<RelayerRelayerApiKey>;
export type RelayerGetTransactionResponse = Array<RelayerRelayerTransaction>;
export type RelayerGetTransactionsResponse = Array<RelayerRelayerTransaction>;
export type RelayerRevocationsResponse = HttpValue;

export interface BridgeDepositRequest {
  [field: string]: unknown;
  "address": string;
}

export interface BridgeDepositResponse {
  [field: string]: unknown;
  "address"?: BridgeDepositResponseAddress | null;
  "note"?: string | null;
}

export interface BridgeDepositResponseAddress {
  [field: string]: unknown;
  "evm"?: string | null;
  "svm"?: string | null;
  "btc"?: string | null;
  "tron"?: string | null;
}

export interface BridgeErrorResponse {
  [field: string]: unknown;
  "error": string;
}

export interface BridgeFeeBreakdown {
  [field: string]: unknown;
  "appFeeLabel"?: string | null;
  "appFeePercent"?: number | bigint | string | null;
  "appFeeUsd"?: number | bigint | string | null;
  "fillCostPercent"?: number | bigint | string | null;
  "fillCostUsd"?: number | bigint | string | null;
  "gasUsd"?: number | bigint | string | null;
  "maxSlippage"?: number | bigint | string | null;
  "minReceived"?: number | bigint | string | null;
  "swapImpact"?: number | bigint | string | null;
  "swapImpactUsd"?: number | bigint | string | null;
  "totalImpact"?: number | bigint | string | null;
  "totalImpactUsd"?: number | bigint | string | null;
}

export interface BridgeGetStatusAddressParams {
  [field: string]: unknown;
  "address": string;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "paginate"?: string | null;
}

export interface BridgeGetStatusAddressResponse {
  [field: string]: unknown;
  "transactions": Array<BridgeTransaction>;
  "nextCursor": string | null;
}

export interface BridgeGetSupportedAssetsParams {
  [field: string]: unknown;
}

export interface BridgeGetSupportedAssetsResponse {
  [field: string]: unknown;
  "supportedAssets"?: Array<BridgeSupportedAsset> | null;
}

export interface BridgePostDepositParams {
  [field: string]: unknown;
  "X-Builder-Code"?: string | null;
  "body": BridgeDepositRequest;
}

export interface BridgePostDepositResponse {
  [field: string]: unknown;
  "address"?: BridgePostDepositResponseAddress | null;
  "note"?: string | null;
}

export interface BridgePostDepositResponseAddress {
  [field: string]: unknown;
  "evm"?: string | null;
  "svm"?: string | null;
  "btc"?: string | null;
  "tron"?: string | null;
}

export interface BridgePostQuoteParams {
  [field: string]: unknown;
  "body": BridgeQuoteRequest;
}

export interface BridgePostQuoteResponse {
  [field: string]: unknown;
  "estCheckoutTimeMs"?: number | bigint | string | null;
  "estFeeBreakdown"?: BridgeFeeBreakdown | null;
  "estInputUsd"?: number | bigint | string | null;
  "estOutputUsd"?: number | bigint | string | null;
  "estToTokenBaseUnit"?: string | null;
  "quoteId"?: string | null;
}

export interface BridgePostWithdrawParams {
  [field: string]: unknown;
  "X-Builder-Code"?: string | null;
  "body": BridgeWithdrawalRequest;
}

export interface BridgePostWithdrawResponse {
  [field: string]: unknown;
  "address"?: BridgePostWithdrawResponseAddress | null;
  "note"?: string | null;
}

export interface BridgePostWithdrawResponseAddress {
  [field: string]: unknown;
  "evm"?: string | null;
  "svm"?: string | null;
  "btc"?: string | null;
  "tron"?: string | null;
}

export interface BridgeQuoteRequest {
  [field: string]: unknown;
  "fromAmountBaseUnit": string;
  "fromChainId": string;
  "fromTokenAddress": string;
  "recipientAddress": string;
  "toChainId": string;
  "toTokenAddress": string;
}

export interface BridgeQuoteResponse {
  [field: string]: unknown;
  "estCheckoutTimeMs"?: number | bigint | string | null;
  "estFeeBreakdown"?: BridgeFeeBreakdown | null;
  "estInputUsd"?: number | bigint | string | null;
  "estOutputUsd"?: number | bigint | string | null;
  "estToTokenBaseUnit"?: string | null;
  "quoteId"?: string | null;
}

export interface BridgeSupportedAsset {
  [field: string]: unknown;
  "chainId"?: string | null;
  "chainName"?: string | null;
  "token"?: BridgeToken | null;
  "minCheckoutUsd"?: number | bigint | string | null;
}

export interface BridgeSupportedAssetsResponse {
  [field: string]: unknown;
  "supportedAssets"?: Array<BridgeSupportedAsset> | null;
}

export interface BridgeToken {
  [field: string]: unknown;
  "name"?: string | null;
  "symbol"?: string | null;
  "address"?: string | null;
  "decimals"?: number | bigint | string | null;
}

export interface BridgeTransaction {
  [field: string]: unknown;
  "fromChainId"?: string | null;
  "fromTokenAddress"?: string | null;
  "fromAmountBaseUnit"?: string | null;
  "toChainId"?: string | null;
  "toTokenAddress"?: string | null;
  "status"?: string | null;
  "txHash"?: string | null;
  "createdTimeMs"?: number | bigint | string | null;
}

export interface BridgeTransactionStatusResponse {
  [field: string]: unknown;
  "transactions": Array<BridgeTransaction>;
  "nextCursor": string | null;
}

export interface BridgeWithdrawalRequest {
  [field: string]: unknown;
  "address": string;
  "toChainId": string;
  "toTokenAddress": string;
  "recipientAddr": string;
}

export interface BuilderGatewayAcceptRfqParams {
  [field: string]: unknown;
  "rfq_id": string;
  "body": BuilderGatewayAcceptRfqParamsBody;
}

export interface BuilderGatewayAcceptRfqParamsBody {
  [field: string]: unknown;
  "quote_id": string;
  "signed_order": Record<string, HttpValue>;
}

export interface BuilderGatewayCreateRfqParams {
  [field: string]: unknown;
  "body": BuilderGatewayCreateRfqParamsBody;
}

export interface BuilderGatewayCreateRfqParamsBody {
  [field: string]: unknown;
  "direction": string;
  "leg_position_ids": Array<string>;
  "maker_address": string;
  "requested_size": BuilderGatewayCreateRfqParamsBodyRequestedSize;
  "side": string;
  "signature_type": number | bigint | string;
  "signer_address": string;
}

export interface BuilderGatewayCreateRfqParamsBodyRequestedSize {
  [field: string]: unknown;
  "unit": string;
  "value_e6": string;
}

export interface BuilderGatewayGetRfqParams {
  [field: string]: unknown;
  "rfq_id": string;
}

export interface ClobApiKeyResponse {
  [field: string]: unknown;
  "apiKey": string;
  "secret": string;
  "passphrase": string;
}

export interface ClobApiKeysResponse {
  [field: string]: unknown;
  "apiKeys": Array<string>;
}

export interface ClobAssetEarning {
  [field: string]: unknown;
  "asset_address": string;
  "earnings": number | bigint | string;
  "asset_rate"?: number | bigint | string | null;
}

export interface ClobBalanceAllowanceResponse {
  [field: string]: unknown;
  "balance": string;
  "allowances": Record<string, string>;
}

export interface ClobBatchPricesHistoryRequest {
  [field: string]: unknown;
  "markets": Array<string>;
  "start_ts"?: number | bigint | string | null;
  "end_ts"?: number | bigint | string | null;
  "interval"?: string | null;
  "fidelity"?: number | bigint | string | null;
}

export interface ClobBatchPricesHistoryResponse {
  [field: string]: unknown;
  "history"?: Record<string, Array<ClobMarketPrice>> | null;
}

export interface ClobBookRequest {
  [field: string]: unknown;
  "token_id": string;
  "side"?: string | null;
}

export interface ClobBuilderApiKeyResponse {
  [field: string]: unknown;
  "key": string;
  "secret": string;
  "passphrase": string;
}

export interface ClobBuilderTrade {
  [field: string]: unknown;
  "id": string;
  "tradeType": string;
  "takerOrderHash": string;
  "builder": string;
  "market": string;
  "assetId": string;
  "side": string;
  "size": string;
  "sizeUsdc": string;
  "price": string;
  "status": string;
  "outcome": string;
  "outcomeIndex": number | bigint | string;
  "owner": string;
  "maker": string;
  "transactionHash": string;
  "matchTime": string;
  "bucketIndex": number | bigint | string;
  "fee": string;
  "feeUsdc": string;
  "err_msg"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
}

export interface ClobBuilderTradesResponse {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "next_cursor": string;
  "count": number | bigint | string;
  "data": Array<ClobBuilderTrade>;
}

export interface ClobCancelAllOrdersParams {
  [field: string]: unknown;
}

export interface ClobCancelAllOrdersResponse {
  [field: string]: unknown;
  "canceled": Array<string>;
  "not_canceled": Record<string, string>;
}

export interface ClobCancelMarketOrdersParams {
  [field: string]: unknown;
  "body": ClobOrderMarketCancelParams;
}

export interface ClobCancelMarketOrdersResponse {
  [field: string]: unknown;
  "canceled": Array<string>;
  "not_canceled": Record<string, string>;
}

export interface ClobCancelOrderParams {
  [field: string]: unknown;
  "body": ClobCancelOrderPayload;
}

export interface ClobCancelOrderPayload {
  [field: string]: unknown;
  "orderID": string;
}

export interface ClobCancelOrderResponse {
  [field: string]: unknown;
  "canceled": Array<string>;
  "not_canceled": Record<string, string>;
}

export interface ClobCancelOrdersParams {
  [field: string]: unknown;
  "body": Array<string>;
}

export interface ClobCancelOrdersResponse {
  [field: string]: unknown;
  "canceled": Array<string>;
  "not_canceled": Record<string, string>;
}

export interface ClobClobMarketDetails {
  [field: string]: unknown;
  "gst"?: string | null;
  "r"?: ClobClobRewards | null;
  "t"?: Array<ClobClobToken> | null;
  "mos"?: number | bigint | string | null;
  "mts"?: number | bigint | string | null;
  "mbf"?: number | bigint | string | null;
  "tbf"?: number | bigint | string | null;
  "rfqe"?: boolean | null;
  "itode"?: boolean | null;
  "ibce"?: boolean | null;
  "fd"?: ClobFeeDetails | null;
  "oas"?: number | bigint | string | null;
}

export interface ClobClobToken {
  [field: string]: unknown;
  "t"?: string | null;
  "o"?: string | null;
}

export interface ClobClosedOnlyResponse {
  [field: string]: unknown;
  "closed_only": boolean;
}

export interface ClobCreateApiKeyParams {
  [field: string]: unknown;
}

export interface ClobCreateApiKeyResponse {
  [field: string]: unknown;
  "apiKey": string;
  "secret": string;
  "passphrase": string;
}

export interface ClobCreateBuilderApiKeyParams {
  [field: string]: unknown;
}

export interface ClobCreateBuilderApiKeyResponse {
  [field: string]: unknown;
  "key": string;
  "secret": string;
  "passphrase": string;
}

export interface ClobCurrentReward {
  [field: string]: unknown;
  "condition_id": string;
  "rewards_max_spread"?: number | bigint | string | null;
  "rewards_min_size"?: number | bigint | string | null;
  "rewards_config"?: Array<ClobCurrentRewardConfig> | null;
  "sponsored_daily_rate"?: number | bigint | string | null;
  "sponsors_count"?: number | bigint | string | null;
  "native_daily_rate"?: number | bigint | string | null;
  "total_daily_rate"?: number | bigint | string | null;
}

export interface ClobCurrentRewardConfig {
  [field: string]: unknown;
  "id"?: number | bigint | string | null;
  "asset_address": string;
  "start_date": string;
  "end_date"?: string | null;
  "rate_per_day": number | bigint | string;
  "total_rewards"?: number | bigint | string | null;
}

export interface ClobDeleteApiKeyParams {
  [field: string]: unknown;
}

export interface ClobDeriveApiKeyParams {
  [field: string]: unknown;
}

export interface ClobDeriveApiKeyResponse {
  [field: string]: unknown;
  "apiKey": string;
  "secret": string;
  "passphrase": string;
}

export interface ClobDropNotificationsParams {
  [field: string]: unknown;
  "ids": string;
}

export interface ClobErrorResponse {
  [field: string]: unknown;
  "error": string;
  "code"?: string | null;
  "retry_after_seconds"?: number | bigint | string | null;
}

export interface ClobFeeDetails {
  [field: string]: unknown;
  "r"?: number | bigint | string | null;
  "e"?: number | bigint | string | null;
  "to"?: boolean | null;
}

export interface ClobFeeRate {
  [field: string]: unknown;
  "base_fee": number | bigint | string;
}

export interface ClobGetApiKeysParams {
  [field: string]: unknown;
}

export interface ClobGetApiKeysResponse {
  [field: string]: unknown;
  "apiKeys": Array<string>;
}

export interface ClobGetBalanceAllowanceParams {
  [field: string]: unknown;
  "asset_type": string;
  "token_id"?: string | null;
  "signature_type"?: number | bigint | string | null;
}

export interface ClobGetBalanceAllowanceResponse {
  [field: string]: unknown;
  "balance": string;
  "allowances": Record<string, string>;
}

export interface ClobGetBatchPricesHistoryParams {
  [field: string]: unknown;
  "body": ClobBatchPricesHistoryRequest;
}

export interface ClobGetBatchPricesHistoryResponse {
  [field: string]: unknown;
  "history"?: Record<string, Array<ClobMarketPrice>> | null;
}

export interface ClobGetBookParams {
  [field: string]: unknown;
  "token_id": string;
}

export interface ClobGetBookResponse {
  [field: string]: unknown;
  "market": string;
  "asset_id": string;
  "timestamp": string;
  "hash": string;
  "bids": Array<ClobOrderSummary>;
  "asks": Array<ClobOrderSummary>;
  "min_order_size": string;
  "tick_size": string;
  "neg_risk": boolean;
  "last_trade_price": string;
}

export interface ClobGetBooksGetParams {
  [field: string]: unknown;
  "token_ids": string;
}

export interface ClobGetBooksPostParams {
  [field: string]: unknown;
  "body": Array<ClobBookRequest>;
}

export interface ClobGetBuilderApiKeysParams {
  [field: string]: unknown;
}

export interface ClobGetBuilderTradesParams {
  [field: string]: unknown;
  "builder_code": string;
  "id"?: string | null;
  "market"?: string | null;
  "asset_id"?: string | null;
  "before"?: string | null;
  "after"?: string | null;
  "next_cursor"?: string | null;
}

export interface ClobGetBuilderTradesResponse {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "next_cursor": string;
  "count": number | bigint | string;
  "data": Array<ClobBuilderTrade>;
}

export interface ClobGetClobMarketInfoParams {
  [field: string]: unknown;
  "condition_id": string;
}

export interface ClobGetClobMarketInfoResponse {
  [field: string]: unknown;
  "gst"?: string | null;
  "r"?: ClobClobRewards | null;
  "t"?: Array<ClobClobToken> | null;
  "mos"?: number | bigint | string | null;
  "mts"?: number | bigint | string | null;
  "mbf"?: number | bigint | string | null;
  "tbf"?: number | bigint | string | null;
  "rfqe"?: boolean | null;
  "itode"?: boolean | null;
  "ibce"?: boolean | null;
  "fd"?: ClobFeeDetails | null;
  "oas"?: number | bigint | string | null;
}

export interface ClobGetClosedOnlyModeParams {
  [field: string]: unknown;
}

export interface ClobGetClosedOnlyModeResponse {
  [field: string]: unknown;
  "closed_only": boolean;
}

export interface ClobGetCurrentRebatedFeesParams {
  [field: string]: unknown;
  "date": string;
  "maker_address": string;
}

export interface ClobGetCurrentRewardsParams {
  [field: string]: unknown;
  "sponsored"?: boolean | null;
  "next_cursor"?: string | null;
}

export interface ClobGetCurrentRewardsResponse {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "count": number | bigint | string;
  "next_cursor": string;
  "data": Array<ClobCurrentReward>;
}

export interface ClobGetEarningsForUserForDayParams {
  [field: string]: unknown;
  "date": string;
  "signature_type"?: number | bigint | string | null;
  "maker_address"?: string | null;
  "sponsored"?: boolean | null;
  "next_cursor"?: string | null;
}

export interface ClobGetEarningsForUserForDayResponse {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "count": number | bigint | string;
  "next_cursor": string;
  "data": Array<ClobUserEarning>;
}

export interface ClobGetFeeRateByPathParams {
  [field: string]: unknown;
  "token_id": string;
}

export interface ClobGetFeeRateByPathResponse {
  [field: string]: unknown;
  "base_fee": number | bigint | string;
}

export interface ClobGetFeeRateParams {
  [field: string]: unknown;
  "token_id"?: string | null;
}

export interface ClobGetFeeRateResponse {
  [field: string]: unknown;
  "base_fee": number | bigint | string;
}

export interface ClobGetLastTradePriceParams {
  [field: string]: unknown;
  "token_id": string;
}

export interface ClobGetLastTradePriceResponse {
  [field: string]: unknown;
  "price": string;
  "side": string;
}

export interface ClobGetLastTradesPricesGetParams {
  [field: string]: unknown;
  "token_ids": string;
}

export interface ClobGetLastTradesPricesGetResponseItem {
  [field: string]: unknown;
  "token_id": string;
  "price": string;
  "side": string;
}

export interface ClobGetLastTradesPricesGetResponseValueItem {
  [field: string]: unknown;
  "token_id": string;
  "price": string;
  "side": string;
}

export interface ClobGetLastTradesPricesPostParams {
  [field: string]: unknown;
  "body": Array<ClobBookRequest>;
}

export interface ClobGetLastTradesPricesPostResponseItem {
  [field: string]: unknown;
  "token_id": string;
  "price": string;
  "side": string;
}

export interface ClobGetLastTradesPricesPostResponseValueItem {
  [field: string]: unknown;
  "token_id": string;
  "price": string;
  "side": string;
}

export interface ClobGetMarketByTokenParams {
  [field: string]: unknown;
  "token_id": string;
}

export interface ClobGetMarketByTokenResponse {
  [field: string]: unknown;
  "condition_id": string;
  "primary_token_id": string;
  "secondary_token_id": string;
}

export interface ClobGetMarketLiveActivityParams {
  [field: string]: unknown;
  "condition_id": string;
}

export interface ClobGetMarketLiveActivityResponse {
  [field: string]: unknown;
  "condition_id"?: string | null;
  "id"?: number | bigint | string | null;
  "question"?: string | null;
  "market_slug"?: string | null;
  "event_slug"?: string | null;
  "series_slug"?: string | null;
  "icon"?: string | null;
  "image"?: string | null;
  "tags"?: Array<string> | null;
}

export interface ClobGetMarketsLiveActivityParams {
  [field: string]: unknown;
  "body": Array<string>;
}

export interface ClobGetMidpointParams {
  [field: string]: unknown;
  "token_id": string;
}

export interface ClobGetMidpointResponse {
  [field: string]: unknown;
  "mid_price": string;
}

export interface ClobGetMidpointsGetParams {
  [field: string]: unknown;
  "token_ids": string;
}

export interface ClobGetMidpointsPostParams {
  [field: string]: unknown;
  "body": Array<ClobBookRequest>;
}

export interface ClobGetMultiMarketsParams {
  [field: string]: unknown;
  "q"?: string | null;
  "tag_slug"?: string | null;
  "event_id"?: string | null;
  "event_title"?: string | null;
  "order_by"?: string | null;
  "position"?: string | null;
  "min_volume_24hr"?: number | bigint | string | null;
  "max_volume_24hr"?: number | bigint | string | null;
  "min_spread"?: number | bigint | string | null;
  "max_spread"?: number | bigint | string | null;
  "min_price"?: number | bigint | string | null;
  "max_price"?: number | bigint | string | null;
  "next_cursor"?: string | null;
  "page_size"?: number | bigint | string | null;
}

export interface ClobGetMultiMarketsResponse {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "count": number | bigint | string;
  "next_cursor": string;
  "data": Array<ClobMultiMarketInfo>;
}

export interface ClobGetNegRiskByPathParams {
  [field: string]: unknown;
  "token_id": string;
}

export interface ClobGetNegRiskByPathResponse {
  [field: string]: unknown;
  "neg_risk": boolean;
}

export interface ClobGetNegRiskParams {
  [field: string]: unknown;
  "token_id"?: string | null;
}

export interface ClobGetNegRiskResponse {
  [field: string]: unknown;
  "neg_risk": boolean;
}

export interface ClobGetNotificationsParams {
  [field: string]: unknown;
  "signature_type": number | bigint | string;
}

export interface ClobGetOrderParams {
  [field: string]: unknown;
  "orderID": string;
}

export interface ClobGetOrderResponse {
  [field: string]: unknown;
  "id": string;
  "status": string;
  "owner": string;
  "maker_address": string;
  "market": string;
  "asset_id": string;
  "side": string;
  "original_size": string;
  "size_matched": string;
  "price": string;
  "outcome": string;
  "expiration": string;
  "order_type": string;
  "associate_trades"?: Array<string> | null;
  "created_at": number | bigint | string;
}

export interface ClobGetOrderScoringParams {
  [field: string]: unknown;
  "order_id": string;
}

export interface ClobGetOrderScoringResponse {
  [field: string]: unknown;
  "scoring": boolean;
}

export interface ClobGetOrdersParams {
  [field: string]: unknown;
  "id"?: string | null;
  "market"?: string | null;
  "asset_id"?: string | null;
  "next_cursor"?: string | null;
}

export interface ClobGetOrdersResponse {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "next_cursor": string;
  "count": number | bigint | string;
  "data": Array<ClobOpenOrder>;
}

export interface ClobGetOrdersScoringParams {
  [field: string]: unknown;
  "order_ids": Array<string>;
}

export interface ClobGetPriceParams {
  [field: string]: unknown;
  "token_id": string;
  "side": string;
}

export interface ClobGetPriceResponse {
  [field: string]: unknown;
  "price": number | bigint | string;
}

export interface ClobGetPricesGetParams {
  [field: string]: unknown;
  "token_ids": string;
  "sides": string;
}

export interface ClobGetPricesHistoryParams {
  [field: string]: unknown;
  "market": string;
  "startTs"?: number | bigint | string | null;
  "endTs"?: number | bigint | string | null;
  "interval"?: string | null;
  "fidelity"?: number | bigint | string | null;
}

export interface ClobGetPricesHistoryResponse {
  [field: string]: unknown;
  "history"?: Array<ClobMarketPrice> | null;
}

export interface ClobGetPricesPostParams {
  [field: string]: unknown;
  "body": Array<ClobBookRequest>;
}

export interface ClobGetRateLimitTierParams {
  [field: string]: unknown;
}

export interface ClobGetRateLimitTierResponse {
  [field: string]: unknown;
  "tier": string;
  "usdc_volume"?: string | null;
  "next_tier_threshold"?: number | bigint | string | null;
  "period": number | bigint | string;
  "tier_details": ClobRateLimitTierDetails;
}

export interface ClobGetRawRewardsForMarketParams {
  [field: string]: unknown;
  "condition_id": string;
  "sponsored"?: boolean | null;
  "next_cursor"?: string | null;
}

export interface ClobGetRawRewardsForMarketResponse {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "count": number | bigint | string;
  "next_cursor": string;
  "data": Array<ClobMarketReward>;
}

export interface ClobGetRewardPercentagesForUserParams {
  [field: string]: unknown;
  "signature_type"?: number | bigint | string | null;
  "maker_address"?: string | null;
}

export interface ClobGetSamplingMarketsParams {
  [field: string]: unknown;
  "next_cursor"?: string | null;
}

export interface ClobGetSamplingMarketsResponse {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "next_cursor"?: string | null;
  "count"?: number | bigint | string | null;
  "data"?: Array<ClobMarket> | null;
}

export interface ClobGetSamplingSimplifiedMarketsParams {
  [field: string]: unknown;
  "next_cursor"?: string | null;
}

export interface ClobGetSamplingSimplifiedMarketsResponse {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "next_cursor"?: string | null;
  "count"?: number | bigint | string | null;
  "data"?: Array<ClobSimplifiedMarket> | null;
}

export interface ClobGetSimplifiedMarketsParams {
  [field: string]: unknown;
  "next_cursor"?: string | null;
}

export interface ClobGetSimplifiedMarketsResponse {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "next_cursor"?: string | null;
  "count"?: number | bigint | string | null;
  "data"?: Array<ClobSimplifiedMarket> | null;
}

export interface ClobGetSpreadParams {
  [field: string]: unknown;
  "token_id": string;
}

export interface ClobGetSpreadResponse {
  [field: string]: unknown;
  "spread": string;
}

export interface ClobGetSpreadsParams {
  [field: string]: unknown;
  "body": Array<ClobBookRequest>;
}

export interface ClobGetTickSizeByPathParams {
  [field: string]: unknown;
  "token_id": string;
}

export interface ClobGetTickSizeByPathResponse {
  [field: string]: unknown;
  "minimum_tick_size": number | bigint | string;
}

export interface ClobGetTickSizeParams {
  [field: string]: unknown;
  "token_id"?: string | null;
}

export interface ClobGetTickSizeResponse {
  [field: string]: unknown;
  "minimum_tick_size": number | bigint | string;
}

export interface ClobGetTimeParams {
  [field: string]: unknown;
}

export interface ClobGetTotalEarningsForUserForDayParams {
  [field: string]: unknown;
  "date": string;
  "signature_type"?: number | bigint | string | null;
  "maker_address"?: string | null;
  "sponsored"?: boolean | null;
}

export interface ClobGetTradesParams {
  [field: string]: unknown;
  "id"?: string | null;
  "maker_address": string;
  "market"?: string | null;
  "asset_id"?: string | null;
  "before"?: string | null;
  "after"?: string | null;
  "next_cursor"?: string | null;
}

export interface ClobGetTradesResponse {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "next_cursor": string;
  "count": number | bigint | string;
  "data": Array<ClobTrade>;
}

export interface ClobGetUpdateBalanceAllowanceParams {
  [field: string]: unknown;
  "asset_type": string;
  "token_id"?: string | null;
  "signature_type"?: number | bigint | string | null;
}

export interface ClobGetUpdateBalanceAllowanceResponse {
  [field: string]: unknown;
  "balance": string;
  "allowances": Record<string, string>;
}

export interface ClobGetUserEarningsAndMarketsConfigParams {
  [field: string]: unknown;
  "date"?: string | null;
  "signature_type"?: number | bigint | string | null;
  "maker_address"?: string | null;
  "sponsored"?: boolean | null;
  "next_cursor"?: string | null;
  "page_size"?: number | bigint | string | null;
  "q"?: string | null;
  "tag_slug"?: string | null;
  "favorite_markets"?: boolean | null;
  "no_competition"?: boolean | null;
  "only_mergeable"?: boolean | null;
  "only_open_orders"?: boolean | null;
  "only_open_positions"?: boolean | null;
  "order_by"?: string | null;
  "position"?: string | null;
}

export interface ClobGetUserEarningsAndMarketsConfigResponse {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "count": number | bigint | string;
  "total_count"?: number | bigint | string | null;
  "next_cursor": string;
  "data": Array<ClobUserRewardsMarket>;
}

export interface ClobHeartbeatErrorResponse {
  [field: string]: unknown;
  "error_msg": string;
  "heartbeat_id": string;
}

export interface ClobHeartbeatRequest {
  [field: string]: unknown;
  "heartbeat_id": string;
}

export interface ClobHeartbeatResponse {
  [field: string]: unknown;
  "status": string;
}

export interface ClobHeartbeatV1Response {
  [field: string]: unknown;
  "heartbeat_id": string;
}

export interface ClobListSessionSignersParams {
  [field: string]: unknown;
}

export interface ClobLiveActivityMarket {
  [field: string]: unknown;
  "condition_id"?: string | null;
  "id"?: number | bigint | string | null;
  "question"?: string | null;
  "market_slug"?: string | null;
  "event_slug"?: string | null;
  "series_slug"?: string | null;
  "icon"?: string | null;
  "image"?: string | null;
  "tags"?: Array<string> | null;
}

export interface ClobMarket {
  [field: string]: unknown;
  "enable_order_book"?: boolean | null;
  "active"?: boolean | null;
  "closed"?: boolean | null;
  "archived"?: boolean | null;
  "accepting_orders"?: boolean | null;
  "accepting_order_timestamp"?: string | null;
  "minimum_order_size"?: number | bigint | string | null;
  "minimum_tick_size"?: number | bigint | string | null;
  "condition_id"?: string | null;
  "question_id"?: string | null;
  "question"?: string | null;
  "description"?: string | null;
  "market_slug"?: string | null;
  "end_date_iso"?: string | null;
  "game_start_time"?: string | null;
  "seconds_delay"?: number | bigint | string | null;
  "fpmm"?: string | null;
  "maker_base_fee"?: number | bigint | string | null;
  "taker_base_fee"?: number | bigint | string | null;
  "notifications_enabled"?: boolean | null;
  "neg_risk"?: boolean | null;
  "neg_risk_market_id"?: string | null;
  "neg_risk_request_id"?: string | null;
  "icon"?: string | null;
  "image"?: string | null;
  "rewards"?: ClobRewards | null;
  "is_50_50_outcome"?: boolean | null;
  "tokens"?: Array<ClobToken> | null;
  "tags"?: Array<string> | null;
}

export interface ClobMarketByTokenResponse {
  [field: string]: unknown;
  "condition_id": string;
  "primary_token_id": string;
  "secondary_token_id": string;
}

export interface ClobMarketPrice {
  [field: string]: unknown;
  "t"?: number | bigint | string | null;
  "p"?: number | bigint | string | null;
}

export interface ClobMarketReward {
  [field: string]: unknown;
  "condition_id": string;
  "question": string;
  "market_slug"?: string | null;
  "event_slug"?: string | null;
  "image"?: string | null;
  "rewards_max_spread"?: number | bigint | string | null;
  "rewards_min_size"?: number | bigint | string | null;
  "market_competitiveness"?: number | bigint | string | null;
  "tokens": Array<ClobRewardsToken>;
  "rewards_config"?: Array<ClobRewardsConfig> | null;
}

export interface ClobMultiMarketInfo {
  [field: string]: unknown;
  "condition_id": string;
  "event_id"?: string | null;
  "event_slug"?: string | null;
  "created_at"?: string | null;
  "group_item_title"?: string | null;
  "image"?: string | null;
  "market_competitiveness"?: number | bigint | string | null;
  "market_id": string;
  "market_slug"?: string | null;
  "one_day_price_change"?: number | bigint | string | null;
  "question": string;
  "rewards_max_spread"?: number | bigint | string | null;
  "rewards_min_size"?: number | bigint | string | null;
  "spread"?: number | bigint | string | null;
  "end_date"?: string | null;
  "tokens": Array<ClobRewardsToken>;
  "volume_24hr"?: number | bigint | string | null;
  "rewards_config"?: Array<ClobRewardsConfig> | null;
}

export interface ClobNegRisk {
  [field: string]: unknown;
  "neg_risk": boolean;
}

export interface ClobNotification {
  [field: string]: unknown;
  "id": number | bigint | string;
  "owner"?: string | null;
  "type": number | bigint | string;
  "payload": Record<string, HttpValue>;
  "timestamp": number | bigint | string;
}

export interface ClobOpenOrder {
  [field: string]: unknown;
  "id": string;
  "status": string;
  "owner": string;
  "maker_address": string;
  "market": string;
  "asset_id": string;
  "side": string;
  "original_size": string;
  "size_matched": string;
  "price": string;
  "outcome": string;
  "expiration": string;
  "order_type": string;
  "associate_trades"?: Array<string> | null;
  "created_at": number | bigint | string;
}

export interface ClobOrder {
  [field: string]: unknown;
  "maker": string;
  "signer": string;
  "tokenId": string;
  "makerAmount": string;
  "takerAmount": string;
  "side": string;
  "expiration": string;
  "timestamp": string;
  "metadata"?: string | null;
  "builder": string;
  "signature"?: string | null;
  "salt": number | bigint | string;
  "signatureType": number | bigint | string;
}

export interface ClobOrderBookSummary {
  [field: string]: unknown;
  "market": string;
  "asset_id": string;
  "timestamp": string;
  "hash": string;
  "bids": Array<ClobOrderSummary>;
  "asks": Array<ClobOrderSummary>;
  "min_order_size": string;
  "tick_size": string;
  "neg_risk": boolean;
  "last_trade_price": string;
}

export interface ClobOrderMarketCancelParams {
  [field: string]: unknown;
  "market": string;
  "asset_id": string;
}

export interface ClobOrderScoringResponse {
  [field: string]: unknown;
  "scoring": boolean;
}

export interface ClobOrderSummary {
  [field: string]: unknown;
  "price": string;
  "size": string;
}

export interface ClobOrdersResponse {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "next_cursor": string;
  "count": number | bigint | string;
  "data": Array<ClobOpenOrder>;
}

export interface ClobPaginatedCurrentReward {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "count": number | bigint | string;
  "next_cursor": string;
  "data": Array<ClobCurrentReward>;
}

export interface ClobPaginatedMarketReward {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "count": number | bigint | string;
  "next_cursor": string;
  "data": Array<ClobMarketReward>;
}

export interface ClobPaginatedMarkets {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "next_cursor"?: string | null;
  "count"?: number | bigint | string | null;
  "data"?: Array<ClobMarket> | null;
}

export interface ClobPaginatedMultiMarketInfo {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "count": number | bigint | string;
  "next_cursor": string;
  "data": Array<ClobMultiMarketInfo>;
}

export interface ClobPaginatedSimplifiedMarkets {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "next_cursor"?: string | null;
  "count"?: number | bigint | string | null;
  "data"?: Array<ClobSimplifiedMarket> | null;
}

export interface ClobPaginatedUserEarnings {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "count": number | bigint | string;
  "next_cursor": string;
  "data": Array<ClobUserEarning>;
}

export interface ClobPaginatedUserRewardsMarkets {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "count": number | bigint | string;
  "total_count"?: number | bigint | string | null;
  "next_cursor": string;
  "data": Array<ClobUserRewardsMarket>;
}

export interface ClobPostOrderParams {
  [field: string]: unknown;
  "body": ClobSendOrder;
}

export interface ClobPostOrderResponse {
  [field: string]: unknown;
  "success": boolean;
  "orderID": string;
  "status": string;
  "makingAmount"?: string | null;
  "takingAmount"?: string | null;
  "transactionsHashes"?: Array<string> | null;
  "tradeIDs"?: Array<string> | null;
  "errorMsg"?: string | null;
}

export interface ClobPostOrdersParams {
  [field: string]: unknown;
  "body": Array<ClobSendOrder>;
}

export interface ClobPostOrdersScoringParams {
  [field: string]: unknown;
  "body": Array<string>;
}

export interface ClobPricesHistoryResponse {
  [field: string]: unknown;
  "history"?: Array<ClobMarketPrice> | null;
}

export interface ClobRateLimitTierDetails {
  [field: string]: unknown;
  "name"?: string | null;
  "volume_threshold"?: number | bigint | string | null;
  "cancel_rate_per_sec"?: number | bigint | string | null;
  "order_rate_per_sec"?: number | bigint | string | null;
  "order_burst"?: number | bigint | string | null;
  "cancel_burst"?: number | bigint | string | null;
  "allow_negative"?: boolean | null;
}

export interface ClobRateLimitTierResponse {
  [field: string]: unknown;
  "tier": string;
  "usdc_volume"?: string | null;
  "next_tier_threshold"?: number | bigint | string | null;
  "period": number | bigint | string;
  "tier_details": ClobRateLimitTierDetails;
}

export interface ClobRebatedFees {
  [field: string]: unknown;
  "date": string;
  "condition_id": string;
  "asset_address": string;
  "maker_address": string;
  "rebated_fees_usdc": string;
}

export interface ClobRevokeBuilderApiKeyParams {
  [field: string]: unknown;
}

export interface ClobRewards {
  [field: string]: unknown;
  "rates"?: Array<ClobRewardsRatesItem> | null;
  "min_size"?: number | bigint | string | null;
  "max_spread"?: number | bigint | string | null;
}

export interface ClobRewardsConfig {
  [field: string]: unknown;
  "id"?: number | bigint | string | null;
  "asset_address": string;
  "start_date": string;
  "end_date"?: string | null;
  "rate_per_day": number | bigint | string;
  "total_rewards"?: number | bigint | string | null;
  "remaining_reward_amount"?: number | bigint | string | null;
  "total_days"?: number | bigint | string | null;
}

export interface ClobRewardsRatesItem {
  [field: string]: unknown;
  "asset_address"?: string | null;
  "rewards_daily_rate"?: number | bigint | string | null;
}

export interface ClobRewardsToken {
  [field: string]: unknown;
  "token_id": string;
  "outcome": string;
  "price"?: number | bigint | string | null;
}

export interface ClobSendHeartbeatParams {
  [field: string]: unknown;
}

export interface ClobSendHeartbeatResponse {
  [field: string]: unknown;
  "status": string;
}

export interface ClobSendHeartbeatV1Params {
  [field: string]: unknown;
  "body": ClobHeartbeatRequest;
}

export interface ClobSendHeartbeatV1Response {
  [field: string]: unknown;
  "heartbeat_id": string;
}

export interface ClobSendOrder {
  [field: string]: unknown;
  "order": ClobOrder;
  "owner": string;
  "orderType"?: string | null;
  "deferExec"?: boolean | null;
  "postOnly"?: boolean | null;
}

export interface ClobSendOrderResponse {
  [field: string]: unknown;
  "success": boolean;
  "orderID": string;
  "status": string;
  "makingAmount"?: string | null;
  "takingAmount"?: string | null;
  "transactionsHashes"?: Array<string> | null;
  "tradeIDs"?: Array<string> | null;
  "errorMsg"?: string | null;
}

export interface ClobSimplifiedMarket {
  [field: string]: unknown;
  "condition_id"?: string | null;
  "rewards"?: ClobRewards | null;
  "tokens"?: Array<ClobToken> | null;
  "active"?: boolean | null;
  "closed"?: boolean | null;
  "archived"?: boolean | null;
  "accepting_orders"?: boolean | null;
}

export interface ClobTickSize {
  [field: string]: unknown;
  "minimum_tick_size": number | bigint | string;
}

export interface ClobToken {
  [field: string]: unknown;
  "token_id"?: string | null;
  "outcome"?: string | null;
  "price"?: number | bigint | string | null;
  "winner"?: boolean | null;
}

export interface ClobTotalUserEarning {
  [field: string]: unknown;
  "date": string;
  "asset_address": string;
  "maker_address": string;
  "earnings": number | bigint | string;
  "native_earnings"?: number | bigint | string | null;
  "sponsored_earnings"?: number | bigint | string | null;
  "asset_rate": number | bigint | string;
}

export interface ClobTrade {
  [field: string]: unknown;
  "id": string;
  "taker_order_id": string;
  "market": string;
  "asset_id": string;
  "side": string;
  "size": string;
  "fee_rate_bps"?: string | null;
  "price": string;
  "status": string;
  "match_time": string;
  "match_time_nano"?: string | null;
  "last_update": string;
  "outcome": string;
  "bucket_index": number | bigint | string;
  "owner": string;
  "maker_address": string;
  "transaction_hash"?: string | null;
  "err_msg"?: string | null;
  "maker_orders"?: Array<ClobTradeMakerOrdersItem> | null;
  "trader_side": string;
}

export interface ClobTradeMakerOrdersItem {
  [field: string]: unknown;
  "order_id"?: string | null;
  "owner"?: string | null;
  "maker_address"?: string | null;
  "matched_amount"?: string | null;
  "price"?: string | null;
  "fee_rate_bps"?: string | null;
  "asset_id"?: string | null;
  "outcome"?: string | null;
  "side"?: string | null;
}

export interface ClobTradesResponse {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "next_cursor": string;
  "count": number | bigint | string;
  "data": Array<ClobTrade>;
}

export interface ClobUpdateBalanceAllowanceParams {
  [field: string]: unknown;
  "asset_type": string;
  "token_id"?: string | null;
  "signature_type"?: number | bigint | string | null;
}

export interface ClobUpdateBalanceAllowanceResponse {
  [field: string]: unknown;
}

export interface ClobUserEarning {
  [field: string]: unknown;
  "date": string;
  "condition_id": string;
  "asset_address": string;
  "maker_address": string;
  "earnings": number | bigint | string;
  "asset_rate"?: number | bigint | string | null;
}

export interface ClobUserRewardsMarket {
  [field: string]: unknown;
  "condition_id": string;
  "market_id": string;
  "event_id"?: string | null;
  "question": string;
  "market_slug"?: string | null;
  "event_slug"?: string | null;
  "image"?: string | null;
  "rewards_max_spread"?: number | bigint | string | null;
  "rewards_min_size"?: number | bigint | string | null;
  "volume_24hr"?: number | bigint | string | null;
  "spread"?: number | bigint | string | null;
  "market_competitiveness"?: number | bigint | string | null;
  "tokens": Array<ClobRewardsToken>;
  "rewards_config"?: Array<ClobCurrentRewardConfig> | null;
  "maker_address"?: string | null;
  "earning_percentage"?: number | bigint | string | null;
  "earnings"?: Array<ClobAssetEarning> | null;
}

export interface CollateralReturnPlanParams {
  [field: string]: unknown;
  "body": CollateralReturnPlanParamsBody;
}

export interface CollateralReturnPlanParamsBody {
  [field: string]: unknown;
  "wallet": string;
}

export interface CollateralReturnSubmitParams {
  [field: string]: unknown;
  "body": CollateralReturnSubmitParamsBody;
}

export interface CollateralReturnSubmitParamsBody {
  [field: string]: unknown;
  "plan_hash": string;
  "envelope": Record<string, HttpValue>;
}

export interface CombosRfqCancelMakerQuoteParams {
  [field: string]: unknown;
  "body": CombosRfqCancelQuoteRequest;
}

export interface CombosRfqCancelMakerQuoteResponse {
  [field: string]: unknown;
  "request": CombosRfqRFQRequest;
  "status": string;
  "competition_started_at"?: number | bigint | string | null;
  "competition_ends_at"?: number | bigint | string | null;
  "confirmation_started_at"?: number | bigint | string | null;
  "confirmation_ends_at"?: number | bigint | string | null;
  "quote_id"?: string | null;
  "bundle"?: CombosRfqFillBundle | null;
  "maker_confirmations"?: Array<CombosRfqMakerConfirmationSnapshot> | null;
}

export interface CombosRfqCancelQuoteRequest {
  [field: string]: unknown;
  "rfq_id": string;
  "quote_id": string;
  "signer_address": string;
  "maker_address": string;
  "signature_type": number | bigint | string;
}

export interface CombosRfqComboMarket {
  [field: string]: unknown;
  "id": string;
  "condition_id": string;
  "position_ids": Array<string>;
  "slug": string;
  "title": string;
  "outcomes": Array<string>;
  "outcome_prices": Array<string>;
  "image": string;
  "volume": number | bigint | string;
  "tags": Array<string>;
}

export interface CombosRfqComboMarketsResponse {
  [field: string]: unknown;
  "markets": Array<CombosRfqComboMarket>;
  "next_cursor": string | null;
}

export interface CombosRfqErrorResponse {
  [field: string]: unknown;
  "error": string;
}

export interface CombosRfqExchangeV3Order {
  [field: string]: unknown;
  "salt": string;
  "maker": string;
  "signer": string;
  "tokenId": string;
  "makerAmount": string;
  "takerAmount": string;
  "side": number | bigint | string;
  "signatureType": number | bigint | string;
  "timestamp": string;
  "metadata"?: string | null;
  "builder"?: string | null;
  "signature"?: string | null;
}

export interface CombosRfqExecutionHandoff {
  [field: string]: unknown;
  "execution_id": string;
  "request": CombosRfqRFQRequest;
  "quote_id": string;
  "bundle": CombosRfqFillBundle;
  "requester_acceptance": CombosRfqRequesterAcceptance;
  "maker_quotes": Array<CombosRfqQuote>;
  "reservations"?: Array<CombosRfqWalletReservation> | null;
  "ready_at"?: number | bigint | string | null;
}

export interface CombosRfqFillAllocation {
  [field: string]: unknown;
  "maker_quote_id": string;
  "signer_address": string;
  "maker_address": string;
  "size_e6": string;
  "price_e6": string;
  "received_at"?: number | bigint | string | null;
}

export interface CombosRfqFillBundle {
  [field: string]: unknown;
  "requested_shares_e6": string;
  "requested_notional_e6"?: string | null;
  "blended_price_e6": string;
  "allocations": Array<CombosRfqFillAllocation>;
}

export interface CombosRfqGetComboMarketsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "exclude"?: string | null;
}

export interface CombosRfqGetComboMarketsResponse {
  [field: string]: unknown;
  "markets": Array<CombosRfqComboMarket>;
  "next_cursor": string | null;
}

export interface CombosRfqMakerConfirmationRequest {
  [field: string]: unknown;
  "rfq_id": string;
  "quote_id": string;
  "signer_address": string;
  "maker_address": string;
  "signature_type": number | bigint | string;
  "decision": string;
}

export interface CombosRfqMakerConfirmationResult {
  [field: string]: unknown;
  "snapshot"?: CombosRfqRFQSnapshot | null;
  "execution"?: CombosRfqExecutionHandoff | null;
}

export interface CombosRfqMakerConfirmationSnapshot {
  [field: string]: unknown;
  "quote_id": string;
  "signer_address": string;
  "maker_address": string;
  "decision"?: string | null;
  "reason"?: string | null;
  "responded_at"?: number | bigint | string | null;
}

export interface CombosRfqQuote {
  [field: string]: unknown;
  "quote_id": string;
  "rfq_id": string;
  "auth_address"?: string | null;
  "signer_address": string;
  "maker_address": string;
  "signature_type": number | bigint | string;
  "price_e6": string;
  "size_e6": string;
  "valid_until"?: number | bigint | string | null;
  "signed_order": CombosRfqExchangeV3Order;
  "received_at"?: number | bigint | string | null;
}

export interface CombosRfqRFQRequest {
  [field: string]: unknown;
  "rfq_id": string;
  "auth_address"?: string | null;
  "signer_address"?: string | null;
  "maker_address"?: string | null;
  "signature_type"?: number | bigint | string | null;
  "requestor_public_id"?: string | null;
  "leg_position_ids": Array<string>;
  "condition_id"?: string | null;
  "yes_position_id"?: string | null;
  "no_position_id"?: string | null;
  "direction": string;
  "side": string;
  "requested_size"?: CombosRfqRequestedSize | null;
  "created_at"?: number | bigint | string | null;
}

export interface CombosRfqRFQSnapshot {
  [field: string]: unknown;
  "request": CombosRfqRFQRequest;
  "status": string;
  "competition_started_at"?: number | bigint | string | null;
  "competition_ends_at"?: number | bigint | string | null;
  "confirmation_started_at"?: number | bigint | string | null;
  "confirmation_ends_at"?: number | bigint | string | null;
  "quote_id"?: string | null;
  "bundle"?: CombosRfqFillBundle | null;
  "maker_confirmations"?: Array<CombosRfqMakerConfirmationSnapshot> | null;
}

export interface CombosRfqRequestedSize {
  [field: string]: unknown;
  "unit": string;
  "value_e6": string;
}

export interface CombosRfqRequesterAcceptance {
  [field: string]: unknown;
  "rfq_id": string;
  "quote_id": string;
  "auth_address"?: string | null;
  "signer_address"?: string | null;
  "maker_address"?: string | null;
  "signature_type"?: number | bigint | string | null;
  "signed_order": CombosRfqExchangeV3Order;
  "accepted_at"?: number | bigint | string | null;
}

export interface CombosRfqSubmitMakerConfirmationParams {
  [field: string]: unknown;
  "body": CombosRfqMakerConfirmationRequest;
}

export interface CombosRfqSubmitMakerConfirmationResponse {
  [field: string]: unknown;
  "snapshot"?: CombosRfqRFQSnapshot | null;
  "execution"?: CombosRfqExecutionHandoff | null;
}

export interface CombosRfqSubmitMakerQuoteParams {
  [field: string]: unknown;
  "body": CombosRfqQuote;
}

export interface CombosRfqSubmitMakerQuoteResponse {
  [field: string]: unknown;
  "request": CombosRfqRFQRequest;
  "status": string;
  "competition_started_at"?: number | bigint | string | null;
  "competition_ends_at"?: number | bigint | string | null;
  "confirmation_started_at"?: number | bigint | string | null;
  "confirmation_ends_at"?: number | bigint | string | null;
  "quote_id"?: string | null;
  "bundle"?: CombosRfqFillBundle | null;
  "maker_confirmations"?: Array<CombosRfqMakerConfirmationSnapshot> | null;
}

export interface CombosRfqWalletAssetDelta {
  [field: string]: unknown;
  "asset": string;
  "asset_id": string;
  "amount": string;
}

export interface CombosRfqWalletReservation {
  [field: string]: unknown;
  "action_id": string;
  "user": string;
  "wallet_nonce": number | bigint | string;
  "deltas": Array<CombosRfqWalletAssetDelta>;
}

export interface DataActivity {
  [field: string]: unknown;
  "bio": string;
  "condition_id": string;
  "event_slug": string;
  "icon": string;
  "is_combo"?: boolean | null;
  "name": string;
  "outcome": string;
  "outcome_index": number | bigint | string;
  "price": number | bigint | string;
  "profile_image": string;
  "profile_image_optimized": string;
  "proxy_wallet": string;
  "pseudonym": string;
  "side": string;
  "size": number | bigint | string;
  "slug": string;
  "timestamp": number | bigint | string;
  "title": string;
  "token_id": string;
  "transaction_hash": string;
  "type": string;
  "usdc_size": number | bigint | string;
}

export interface DataActivityPage {
  [field: string]: unknown;
  "data": Array<DataActivity>;
  "pagination": DataPagination;
}

export interface DataApprovalContract {
  [field: string]: unknown;
  "amount"?: string | null;
  "approved": boolean;
  "feature": string;
  "id": string;
  "spender": string;
  "standard": string;
  "token": string;
}

export interface DataApprovals {
  [field: string]: unknown;
  "address": string;
  "chain_id": number | bigint | string;
  "checked_at": string;
  "contracts": Array<DataApprovalContract>;
}

export interface DataBiggestWinner {
  [field: string]: unknown;
  "condition_id": string;
  "event_id": number | bigint | string;
  "event_slug": string;
  "event_title": string;
  "final_value": number | bigint | string;
  "initial_value": number | bigint | string;
  "kind": string;
  "pnl": number | bigint | string;
  "position_id": string;
  "profile_image": string;
  "resolved_at": number | bigint | string;
  "user_id": string;
  "user_name": string;
  "win_rank": number | bigint | string;
}

export interface DataBiggestWinnersPage {
  [field: string]: unknown;
  "data": Array<DataBiggestWinner>;
  "pagination": DataPagination;
}

export interface DataBuilderStanding {
  [field: string]: unknown;
  "active_users": number | bigint | string;
  "builder_code": string;
  "builder_name": string;
  "profile_image": string;
  "rank": number | bigint | string;
  "verified": boolean;
  "volume": number | bigint | string;
}

export interface DataBuilderVolumePoint {
  [field: string]: unknown;
  "active_users": number | bigint | string;
  "builder_code": string;
  "builder_name": string;
  "date": string;
  "profile_image": string;
  "rank": number | bigint | string;
  "verified": boolean;
  "volume": number | bigint | string;
}

export interface DataBuildersLeaderboardPage {
  [field: string]: unknown;
  "data": Array<DataBuilderStanding>;
  "pagination": DataPagination;
}

export interface DataComboActivity {
  [field: string]: unknown;
  "amount_usdc"?: number | bigint | string | null;
  "block_number": number | bigint | string;
  "combo_condition_id": string;
  "combo_position_id": string;
  "id": string;
  "legs": Array<DataComboLeg>;
  "payout_usdc"?: number | bigint | string | null;
  "proxy_wallet": string;
  "timestamp": number | bigint | string;
  "transaction_hash": string;
  "type": string;
}

export interface DataComboActivityPage {
  [field: string]: unknown;
  "data": Array<DataComboActivity>;
  "pagination": DataPagination;
}

export interface DataComboLeg {
  [field: string]: unknown;
  "leg_condition_id": string;
  "leg_current_price": number | bigint | string;
  "leg_index": number | bigint | string;
  "leg_outcome_index": number | bigint | string;
  "leg_outcome_label": string;
  "leg_position_id": string;
  "leg_resolved_at"?: string | null;
  "leg_status": string;
  "market": DataComboLegMarket;
}

export interface DataComboLegEvent {
  [field: string]: unknown;
  "event_id": string;
  "event_image": string;
  "event_slug": string;
  "event_title": string;
}

export interface DataComboLegMarket {
  [field: string]: unknown;
  "category": string;
  "end_date": string;
  "event": DataComboLegEvent;
  "group_item_title"?: string | null;
  "icon_url": string;
  "image_url": string;
  "line"?: number | bigint | string | null;
  "market_id": string;
  "outcome": string;
  "outcomes"?: Array<string> | null;
  "question"?: string | null;
  "slug": string;
  "sports_market_type"?: string | null;
  "subcategory": string;
  "tags": Array<string>;
  "title": string;
}

export interface DataComboPosition {
  [field: string]: unknown;
  "combo_condition_id": string;
  "combo_position_id": string;
  "current_size": number | bigint | string;
  "entry_avg_price_usdc": number | bigint | string;
  "entry_cost_usdc": number | bigint | string;
  "entry_fees_usdc": number | bigint | string;
  "first_entry_at": string;
  "first_entry_at_micros"?: number | bigint | string | null;
  "gross_entry_cost_usdc": number | bigint | string;
  "legs": Array<DataComboLeg>;
  "legs_pending": number | bigint | string;
  "legs_resolved": number | bigint | string;
  "legs_total": number | bigint | string;
  "outcome_index": number | bigint | string;
  "outcome_label": string;
  "proxy_wallet": string;
  "realized_payout_usdc": number | bigint | string;
  "redeemable": boolean;
  "resolved_at"?: string | null;
  "status": string;
  "updated_at": string;
  "updated_at_micros": number | bigint | string;
}

export interface DataComboPositionsPage {
  [field: string]: unknown;
  "data": Array<DataComboPosition>;
  "pagination": DataPagination;
}

export interface DataConditionVolume {
  [field: string]: unknown;
  "condition_id": string;
  "taker_volume": number | bigint | string;
}

export interface DataCursorLag {
  [field: string]: unknown;
  "behind_max": number | bigint | string;
  "block": number | bigint | string;
  "source": string;
}

export interface DataEnvelopeApprovals {
  [field: string]: unknown;
  "data": DataEnvelopeApprovalsData;
}

export interface DataEnvelopeApprovalsData {
  [field: string]: unknown;
  "address": string;
  "chain_id": number | bigint | string;
  "checked_at": string;
  "contracts": Array<DataApprovalContract>;
}

export interface DataEnvelopeLiveVolume {
  [field: string]: unknown;
  "data": DataEnvelopeLiveVolumeData;
}

export interface DataEnvelopeLiveVolumeData {
  [field: string]: unknown;
  "conditions": Array<DataConditionVolume>;
  "taker_volume_total": number | bigint | string;
}

export interface DataEnvelopeOptionLeaderboardUserEntry {
  [field: string]: unknown;
  "data": HttpValue;
}

export interface DataEnvelopeOptionUserStats {
  [field: string]: unknown;
  "data": HttpValue;
}

export interface DataEnvelopePortfolioValue {
  [field: string]: unknown;
  "data": DataEnvelopePortfolioValueData;
}

export interface DataEnvelopePortfolioValueData {
  [field: string]: unknown;
  "proxy_wallet": string;
  "value": number | bigint | string;
}

export interface DataEnvelopeServiceStatus {
  [field: string]: unknown;
  "data": DataEnvelopeServiceStatusData;
}

export interface DataEnvelopeServiceStatusData {
  [field: string]: unknown;
  "age_seconds": number | bigint | string;
  "computed_at": string;
  "ingestion": DataIngestionFreshness;
  "serving": DataServingFreshness;
}

export interface DataEnvelopeUserPnlSeries {
  [field: string]: unknown;
  "data": DataEnvelopeUserPnlSeriesData;
}

export interface DataEnvelopeUserPnlSeriesData {
  [field: string]: unknown;
  "fidelity": string;
  "interval": string;
  "points": Array<DataUserPnlPoint>;
  "proxy_wallet": string;
  "source_fidelity": string;
}

export interface DataEnvelopeUserVolume {
  [field: string]: unknown;
  "data": DataEnvelopeUserVolumeData;
}

export interface DataEnvelopeUserVolumeData {
  [field: string]: unknown;
  "trade_count": number | bigint | string;
  "volume": number | bigint | string;
  "volume_usdc": number | bigint | string;
}

export interface DataEnvelopeVecBuilderVolumePoint {
  [field: string]: unknown;
  "data": Array<DataEnvelopeVecBuilderVolumePointDataItem>;
}

export interface DataEnvelopeVecBuilderVolumePointDataItem {
  [field: string]: unknown;
  "active_users": number | bigint | string;
  "builder_code": string;
  "builder_name": string;
  "date": string;
  "profile_image": string;
  "rank": number | bigint | string;
  "verified": boolean;
  "volume": number | bigint | string;
}

export interface DataEnvelopeVecOpenInterest {
  [field: string]: unknown;
  "data": Array<DataEnvelopeVecOpenInterestDataItem>;
}

export interface DataEnvelopeVecOpenInterestDataItem {
  [field: string]: unknown;
  "condition_id": string;
  "value": number | bigint | string;
}

export interface DataEnvelopeVecResolution {
  [field: string]: unknown;
  "data": Array<DataEnvelopeVecResolutionDataItem>;
}

export interface DataEnvelopeVecResolutionDataItem {
  [field: string]: unknown;
  "condition_id"?: string | null;
  "expected_settlement_time"?: string | null;
  "extended_review": boolean;
  "last_update_timestamp": string;
  "log_index": string;
  "market_type"?: string | null;
  "new_version_q": boolean;
  "payouts"?: Array<number | bigint | string> | null;
  "price"?: string | null;
  "proposed_price"?: string | null;
  "question_id"?: string | null;
  "reporter"?: string | null;
  "reproposed_price"?: string | null;
  "resolution_source"?: string | null;
  "resolved_at"?: string | null;
  "resolved_block"?: number | bigint | string | null;
  "settlement_time_basis"?: string | null;
  "status": string;
  "transaction_hash": string;
  "was_arbitrated"?: boolean | null;
  "was_disputed": boolean;
}

export interface DataEnvelopeVecTokenRef {
  [field: string]: unknown;
  "data": Array<DataEnvelopeVecTokenRefDataItem>;
}

export interface DataEnvelopeVecTokenRefDataItem {
  [field: string]: unknown;
  "clob_index"?: number | bigint | string | null;
  "closed"?: boolean | null;
  "condition_id": string;
  "event_id"?: number | bigint | string | null;
  "event_slug"?: string | null;
  "final_price"?: number | bigint | string | null;
  "market_slug"?: string | null;
  "module": string;
  "neg_risk"?: boolean | null;
  "neg_risk_market_id"?: string | null;
  "opposite_token_id"?: string | null;
  "outcome"?: string | null;
  "outcome_index": number | bigint | string;
  "question_index"?: number | bigint | string | null;
  "resolved": boolean;
  "structural_condition_id"?: string | null;
  "title"?: string | null;
  "token_id": string;
}

export interface DataErrorResponse {
  [field: string]: unknown;
  "code": string;
  "error": string;
  "parameter"?: string | null;
  "retryable": boolean;
  "trace_id": string;
}

export interface DataGetActivityParams {
  [field: string]: unknown;
  "user"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "type"?: string | null;
  "condition"?: string | null;
  "event_id"?: string | null;
  "side"?: string | null;
  "start"?: number | bigint | string | null;
  "end"?: number | bigint | string | null;
  "sort_by"?: string | null;
  "sort_direction"?: string | null;
  "exclude_deposits_withdrawals"?: boolean | null;
}

export interface DataGetActivityResponse {
  [field: string]: unknown;
  "data": Array<DataActivity>;
  "pagination": DataPagination;
}

export interface DataGetApprovalsParams {
  [field: string]: unknown;
  "user"?: string | null;
}

export interface DataGetApprovalsResponse {
  [field: string]: unknown;
  "data": DataGetApprovalsResponseData;
}

export interface DataGetApprovalsResponseData {
  [field: string]: unknown;
  "address": string;
  "chain_id": number | bigint | string;
  "checked_at": string;
  "contracts": Array<DataApprovalContract>;
}

export interface DataGetBiggestWinnersParams {
  [field: string]: unknown;
  "time_period"?: string | null;
  "category"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface DataGetBiggestWinnersResponse {
  [field: string]: unknown;
  "data": Array<DataBiggestWinner>;
  "pagination": DataPagination;
}

export interface DataGetBuildersLeaderboardParams {
  [field: string]: unknown;
  "time_period"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface DataGetBuildersLeaderboardResponse {
  [field: string]: unknown;
  "data": Array<DataBuilderStanding>;
  "pagination": DataPagination;
}

export interface DataGetBuildersVolumeParams {
  [field: string]: unknown;
  "interval"?: string | null;
  "limit"?: number | bigint | string | null;
}

export interface DataGetBuildersVolumeResponse {
  [field: string]: unknown;
  "data": Array<DataGetBuildersVolumeResponseDataItem>;
}

export interface DataGetBuildersVolumeResponseDataItem {
  [field: string]: unknown;
  "active_users": number | bigint | string;
  "builder_code": string;
  "builder_name": string;
  "date": string;
  "profile_image": string;
  "rank": number | bigint | string;
  "verified": boolean;
  "volume": number | bigint | string;
}

export interface DataGetComboPositionsParams {
  [field: string]: unknown;
  "user"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "condition"?: string | null;
  "status"?: string | null;
  "sort_by"?: string | null;
  "sort_direction"?: string | null;
  "updated_after"?: number | bigint | string | null;
  "updated_before"?: number | bigint | string | null;
}

export interface DataGetComboPositionsResponse {
  [field: string]: unknown;
  "data": Array<DataComboPosition>;
  "pagination": DataPagination;
}

export interface DataGetCombosParams {
  [field: string]: unknown;
  "user"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "condition"?: string | null;
}

export interface DataGetCombosResponse {
  [field: string]: unknown;
  "data": Array<DataComboActivity>;
  "pagination": DataPagination;
}

export interface DataGetHoldersParams {
  [field: string]: unknown;
  "condition"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "min_balance"?: number | bigint | string | null;
  "include_pnl"?: boolean | null;
}

export interface DataGetHoldersResponse {
  [field: string]: unknown;
  "data": Array<DataMetaHolder>;
  "pagination": DataPagination;
}

export interface DataGetLeaderboardParams {
  [field: string]: unknown;
  "time_period"?: string | null;
  "category"?: string | null;
  "sort_by"?: string | null;
  "user"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface DataGetLiveVolumeParams {
  [field: string]: unknown;
  "event_id"?: string | null;
}

export interface DataGetLiveVolumeResponse {
  [field: string]: unknown;
  "data": DataGetLiveVolumeResponseData;
}

export interface DataGetLiveVolumeResponseData {
  [field: string]: unknown;
  "conditions": Array<DataConditionVolume>;
  "taker_volume_total": number | bigint | string;
}

export interface DataGetOiParams {
  [field: string]: unknown;
  "condition"?: string | null;
}

export interface DataGetOiResponse {
  [field: string]: unknown;
  "data": Array<DataGetOiResponseDataItem>;
}

export interface DataGetOiResponseDataItem {
  [field: string]: unknown;
  "condition_id": string;
  "value": number | bigint | string;
}

export interface DataGetPositionsParams {
  [field: string]: unknown;
  "user"?: string | null;
  "condition"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "status"?: string | null;
  "event_id"?: string | null;
  "title"?: string | null;
  "filter_type"?: string | null;
  "filter_amount"?: number | bigint | string | null;
  "include_archived"?: boolean | null;
  "sort_by"?: string | null;
  "start"?: number | bigint | string | null;
  "end"?: number | bigint | string | null;
  "sort_direction"?: string | null;
}

export interface DataGetPositionsResponse {
  [field: string]: unknown;
  "data": Array<DataPosition>;
  "pagination": DataPagination;
}

export interface DataGetPricesHistoryParams {
  [field: string]: unknown;
  "token_id"?: string | null;
  "start"?: number | bigint | string | null;
  "end"?: number | bigint | string | null;
  "interval"?: string | null;
  "bucket_seconds"?: number | bigint | string | null;
  "as_of"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface DataGetPricesHistoryResponse {
  [field: string]: unknown;
  "data": Array<DataPricePoint>;
  "pagination": DataPagination;
}

export interface DataGetResolutionsParams {
  [field: string]: unknown;
  "question_id"?: string | null;
  "condition"?: string | null;
  "event_id"?: string | null;
}

export interface DataGetResolutionsResponse {
  [field: string]: unknown;
  "data": Array<DataGetResolutionsResponseDataItem>;
}

export interface DataGetResolutionsResponseDataItem {
  [field: string]: unknown;
  "condition_id"?: string | null;
  "expected_settlement_time"?: string | null;
  "extended_review": boolean;
  "last_update_timestamp": string;
  "log_index": string;
  "market_type"?: string | null;
  "new_version_q": boolean;
  "payouts"?: Array<number | bigint | string> | null;
  "price"?: string | null;
  "proposed_price"?: string | null;
  "question_id"?: string | null;
  "reporter"?: string | null;
  "reproposed_price"?: string | null;
  "resolution_source"?: string | null;
  "resolved_at"?: string | null;
  "resolved_block"?: number | bigint | string | null;
  "settlement_time_basis"?: string | null;
  "status": string;
  "transaction_hash": string;
  "was_arbitrated"?: boolean | null;
  "was_disputed": boolean;
}

export interface DataGetStatusParams {
  [field: string]: unknown;
}

export interface DataGetStatusResponse {
  [field: string]: unknown;
  "data": DataGetStatusResponseData;
}

export interface DataGetStatusResponseData {
  [field: string]: unknown;
  "age_seconds": number | bigint | string;
  "computed_at": string;
  "ingestion": DataIngestionFreshness;
  "serving": DataServingFreshness;
}

export interface DataGetTokensParams {
  [field: string]: unknown;
  "token_id"?: string | null;
  "condition"?: string | null;
}

export interface DataGetTokensResponse {
  [field: string]: unknown;
  "data": Array<DataGetTokensResponseDataItem>;
}

export interface DataGetTokensResponseDataItem {
  [field: string]: unknown;
  "clob_index"?: number | bigint | string | null;
  "closed"?: boolean | null;
  "condition_id": string;
  "event_id"?: number | bigint | string | null;
  "event_slug"?: string | null;
  "final_price"?: number | bigint | string | null;
  "market_slug"?: string | null;
  "module": string;
  "neg_risk"?: boolean | null;
  "neg_risk_market_id"?: string | null;
  "opposite_token_id"?: string | null;
  "outcome"?: string | null;
  "outcome_index": number | bigint | string;
  "question_index"?: number | bigint | string | null;
  "resolved": boolean;
  "structural_condition_id"?: string | null;
  "title"?: string | null;
  "token_id": string;
}

export interface DataGetTradesParams {
  [field: string]: unknown;
  "user"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "taker_only"?: boolean | null;
  "filter_type"?: string | null;
  "filter_amount"?: number | bigint | string | null;
  "start"?: number | bigint | string | null;
  "end"?: number | bigint | string | null;
  "condition"?: string | null;
  "event_id"?: string | null;
  "side"?: string | null;
}

export interface DataGetTradesResponse {
  [field: string]: unknown;
  "data": Array<DataTrade>;
  "pagination": DataPagination;
}

export interface DataGetUserPnlV2Params {
  [field: string]: unknown;
  "user"?: string | null;
  "interval"?: string | null;
  "fidelity"?: string | null;
}

export interface DataGetUserPnlV2Response {
  [field: string]: unknown;
  "data": DataGetUserPnlV2ResponseData;
}

export interface DataGetUserPnlV2ResponseData {
  [field: string]: unknown;
  "fidelity": string;
  "interval": string;
  "points": Array<DataUserPnlPoint>;
  "proxy_wallet": string;
  "source_fidelity": string;
}

export interface DataGetUserStatsParams {
  [field: string]: unknown;
  "user"?: string | null;
}

export interface DataGetUserStatsResponse {
  [field: string]: unknown;
  "data": HttpValue;
}

export interface DataGetUserVolumeParams {
  [field: string]: unknown;
  "user"?: string | null;
  "start"?: number | bigint | string | null;
  "end"?: number | bigint | string | null;
}

export interface DataGetUserVolumeResponse {
  [field: string]: unknown;
  "data": DataGetUserVolumeResponseData;
}

export interface DataGetUserVolumeResponseData {
  [field: string]: unknown;
  "trade_count": number | bigint | string;
  "volume": number | bigint | string;
  "volume_usdc": number | bigint | string;
}

export interface DataGetValueParams {
  [field: string]: unknown;
  "user"?: string | null;
  "condition"?: string | null;
}

export interface DataGetValueResponse {
  [field: string]: unknown;
  "data": DataGetValueResponseData;
}

export interface DataGetValueResponseData {
  [field: string]: unknown;
  "proxy_wallet": string;
  "value": number | bigint | string;
}

export interface DataHolder {
  [field: string]: unknown;
  "amount": number | bigint | string;
  "avg_price"?: number | bigint | string | null;
  "bio": string;
  "current_price"?: number | bigint | string | null;
  "current_value"?: number | bigint | string | null;
  "display_username_public": boolean;
  "entry_cost_usdc"?: number | bigint | string | null;
  "name": string;
  "outcome_index": number | bigint | string;
  "profile_image": string;
  "profile_image_optimized": string;
  "proxy_wallet": string;
  "pseudonym": string;
  "realized_pnl"?: number | bigint | string | null;
  "token_id": string;
  "total_pnl"?: number | bigint | string | null;
  "unrealized_pnl"?: number | bigint | string | null;
  "verified": boolean;
}

export interface DataHoldersPage {
  [field: string]: unknown;
  "data": Array<DataMetaHolder>;
  "pagination": DataPagination;
}

export interface DataIngestionFreshness {
  [field: string]: unknown;
  "chain_id": number | bigint | string;
  "cursors": number | bigint | string;
  "lagging": Array<DataCursorLag>;
  "max_synced_block"?: number | bigint | string | null;
  "min_synced_block"?: number | bigint | string | null;
  "most_lagged"?: HttpValue | null;
  "network"?: string | null;
}

export interface DataLeaderboardEntry {
  [field: string]: unknown;
  "pnl": number | bigint | string;
  "profile_image": string;
  "rank": number | bigint | string;
  "user_id": string;
  "user_name": string;
  "verified": boolean;
  "volume": number | bigint | string;
  "x_username": string;
}

export interface DataLeaderboardPage {
  [field: string]: unknown;
  "data": Array<DataLeaderboardEntry>;
  "pagination": DataPagination;
}

export interface DataLeaderboardUserEntry {
  [field: string]: unknown;
  "pnl": number | bigint | string;
  "profile_image": string;
  "rank_pnl"?: number | bigint | string | null;
  "rank_volume"?: number | bigint | string | null;
  "user_id": string;
  "user_name": string;
  "verified": boolean;
  "volume": number | bigint | string;
  "x_username": string;
}

export interface DataLiveVolume {
  [field: string]: unknown;
  "conditions": Array<DataConditionVolume>;
  "taker_volume_total": number | bigint | string;
}

export interface DataMetaHolder {
  [field: string]: unknown;
  "holders": Array<DataHolder>;
  "token_id": string;
}

export interface DataOpenInterest {
  [field: string]: unknown;
  "condition_id": string;
  "value": number | bigint | string;
}

export interface DataPagination {
  [field: string]: unknown;
  "has_more": boolean;
  "limit": number | bigint | string;
  "next_cursor"?: string | null;
  "offset": number | bigint | string;
}

export interface DataPortfolioValue {
  [field: string]: unknown;
  "proxy_wallet": string;
  "value": number | bigint | string;
}

export interface DataPosition {
  [field: string]: unknown;
  "archived": boolean;
  "avg_price": number | bigint | string;
  "condition_id": string;
  "current_price": number | bigint | string;
  "current_size": number | bigint | string;
  "current_value": number | bigint | string;
  "end_date": string;
  "entry_cost_usdc": number | bigint | string;
  "entry_fees_usdc": number | bigint | string;
  "event_id": string;
  "event_slug": string;
  "first_entry_at": number | bigint | string;
  "icon": string;
  "last_event_at": number | bigint | string;
  "mergeable": boolean;
  "name": string;
  "negative_risk": boolean;
  "opposite_outcome": string;
  "opposite_token_id": string;
  "outcome": string;
  "outcome_index": number | bigint | string;
  "percent_pnl": number | bigint | string;
  "percent_realized_pnl": number | bigint | string;
  "profile_image": string;
  "proxy_wallet": string;
  "realized_pnl": number | bigint | string;
  "redeemable": boolean;
  "slug": string;
  "status": string;
  "title": string;
  "token_id": string;
  "total_cost_usdc": number | bigint | string;
  "total_pnl": number | bigint | string;
  "total_size": number | bigint | string;
  "unrealized_pnl": number | bigint | string;
  "verified": boolean;
}

export interface DataPositionsPage {
  [field: string]: unknown;
  "data": Array<DataPosition>;
  "pagination": DataPagination;
}

export interface DataPricePoint {
  [field: string]: unknown;
  "price": number | bigint | string;
  "resolution_seconds": number | bigint | string;
  "timestamp": number | bigint | string;
}

export interface DataPricesHistoryPage {
  [field: string]: unknown;
  "data": Array<DataPricePoint>;
  "pagination": DataPagination;
}

export interface DataResolution {
  [field: string]: unknown;
  "condition_id"?: string | null;
  "expected_settlement_time"?: string | null;
  "extended_review": boolean;
  "last_update_timestamp": string;
  "log_index": string;
  "market_type"?: string | null;
  "new_version_q": boolean;
  "payouts"?: Array<number | bigint | string> | null;
  "price"?: string | null;
  "proposed_price"?: string | null;
  "question_id"?: string | null;
  "reporter"?: string | null;
  "reproposed_price"?: string | null;
  "resolution_source"?: string | null;
  "resolved_at"?: string | null;
  "resolved_block"?: number | bigint | string | null;
  "settlement_time_basis"?: string | null;
  "status": string;
  "transaction_hash": string;
  "was_arbitrated"?: boolean | null;
  "was_disputed": boolean;
}

export interface DataServiceStatus {
  [field: string]: unknown;
  "age_seconds": number | bigint | string;
  "computed_at": string;
  "ingestion": DataIngestionFreshness;
  "serving": DataServingFreshness;
}

export interface DataServingFreshness {
  [field: string]: unknown;
  "lag_seconds"?: number | bigint | string | null;
  "mechanisms": Array<DataServingMechanism>;
  "worst"?: string | null;
}

export interface DataServingMechanism {
  [field: string]: unknown;
  "age_seconds": number | bigint | string;
  "blocks_behind"?: number | bigint | string | null;
  "name": string;
}

export interface DataTokenRef {
  [field: string]: unknown;
  "clob_index"?: number | bigint | string | null;
  "closed"?: boolean | null;
  "condition_id": string;
  "event_id"?: number | bigint | string | null;
  "event_slug"?: string | null;
  "final_price"?: number | bigint | string | null;
  "market_slug"?: string | null;
  "module": string;
  "neg_risk"?: boolean | null;
  "neg_risk_market_id"?: string | null;
  "opposite_token_id"?: string | null;
  "outcome"?: string | null;
  "outcome_index": number | bigint | string;
  "question_index"?: number | bigint | string | null;
  "resolved": boolean;
  "structural_condition_id"?: string | null;
  "title"?: string | null;
  "token_id": string;
}

export interface DataTrade {
  [field: string]: unknown;
  "bio": string;
  "condition_id": string;
  "event_slug": string;
  "icon": string;
  "name": string;
  "outcome": string;
  "outcome_index": number | bigint | string;
  "price": number | bigint | string;
  "profile_image": string;
  "profile_image_optimized": string;
  "proxy_wallet": string;
  "pseudonym": string;
  "side": string;
  "size": number | bigint | string;
  "slug": string;
  "timestamp": number | bigint | string;
  "title": string;
  "token_id": string;
  "transaction_hash": string;
}

export interface DataTradesPage {
  [field: string]: unknown;
  "data": Array<DataTrade>;
  "pagination": DataPagination;
}

export interface DataUserPnlPoint {
  [field: string]: unknown;
  "cashflow_net"?: number | bigint | string | null;
  "deposits"?: number | bigint | string | null;
  "economic_pnl"?: number | bigint | string | null;
  "fees"?: number | bigint | string | null;
  "fees_paid"?: number | bigint | string | null;
  "fees_refunded"?: number | bigint | string | null;
  "maker_rebate"?: number | bigint | string | null;
  "position_pnl"?: number | bigint | string | null;
  "realized_combo_pnl": number | bigint | string;
  "realized_lp_pnl": number | bigint | string;
  "realized_market_pnl": number | bigint | string;
  "realized_pnl": number | bigint | string;
  "referral_income"?: number | bigint | string | null;
  "reward_income"?: number | bigint | string | null;
  "settled_pnl"?: number | bigint | string | null;
  "source_block": number | bigint | string;
  "sponsored_income"?: number | bigint | string | null;
  "taker_rebate"?: number | bigint | string | null;
  "timestamp": number | bigint | string;
  "trade_count": number | bigint | string;
  "trade_pnl"?: number | bigint | string | null;
  "unrealized_pnl"?: number | bigint | string | null;
  "volume": number | bigint | string;
  "volume_usdc": number | bigint | string;
  "wallet_income"?: number | bigint | string | null;
  "withdrawals"?: number | bigint | string | null;
  "yield_income"?: number | bigint | string | null;
}

export interface DataUserPnlSeries {
  [field: string]: unknown;
  "fidelity": string;
  "interval": string;
  "points": Array<DataUserPnlPoint>;
  "proxy_wallet": string;
  "source_fidelity": string;
}

export interface DataUserStats {
  [field: string]: unknown;
  "all_time_pnl"?: HttpValue | null;
  "biggest_win": number | bigint | string;
  "join_date"?: number | bigint | string | null;
  "proxy_wallet": string;
  "trades": number | bigint | string;
  "views": number | bigint | string;
}

export interface DataUserVolume {
  [field: string]: unknown;
  "trade_count": number | bigint | string;
  "volume": number | bigint | string;
  "volume_usdc": number | bigint | string;
}

export interface GammaCategory {
  [field: string]: unknown;
  "id"?: string | null;
  "label"?: string | null;
  "parentCategory"?: string | null;
  "slug"?: string | null;
  "publishedAt"?: string | null;
  "createdBy"?: string | null;
  "updatedBy"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
}

export interface GammaChat {
  [field: string]: unknown;
  "id"?: string | null;
  "channelId"?: string | null;
  "channelName"?: string | null;
  "channelImage"?: string | null;
  "live"?: boolean | null;
  "startTime"?: string | null;
  "endTime"?: string | null;
}

export interface GammaCollection {
  [field: string]: unknown;
  "id"?: string | null;
  "ticker"?: string | null;
  "slug"?: string | null;
  "title"?: string | null;
  "subtitle"?: string | null;
  "collectionType"?: string | null;
  "description"?: string | null;
  "tags"?: string | null;
  "image"?: string | null;
  "icon"?: string | null;
  "headerImage"?: string | null;
  "layout"?: string | null;
  "active"?: boolean | null;
  "closed"?: boolean | null;
  "archived"?: boolean | null;
  "new"?: boolean | null;
  "featured"?: boolean | null;
  "restricted"?: boolean | null;
  "isTemplate"?: boolean | null;
  "templateVariables"?: string | null;
  "publishedAt"?: string | null;
  "createdBy"?: string | null;
  "updatedBy"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "commentsEnabled"?: boolean | null;
  "imageOptimized"?: GammaImageOptimization | null;
  "iconOptimized"?: GammaImageOptimization | null;
  "headerImageOptimized"?: GammaImageOptimization | null;
}

export interface GammaComment {
  [field: string]: unknown;
  "id"?: string | null;
  "body"?: string | null;
  "parentEntityType"?: string | null;
  "parentEntityID"?: number | bigint | string | null;
  "parentCommentID"?: string | null;
  "userAddress"?: string | null;
  "replyAddress"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "profile"?: GammaCommentProfile | null;
  "reactions"?: Array<GammaReaction> | null;
  "reportCount"?: number | bigint | string | null;
  "reactionCount"?: number | bigint | string | null;
}

export interface GammaCommentPosition {
  [field: string]: unknown;
  "tokenId"?: string | null;
  "positionSize"?: string | null;
}

export interface GammaCommentProfile {
  [field: string]: unknown;
  "name"?: string | null;
  "pseudonym"?: string | null;
  "displayUsernamePublic"?: boolean | null;
  "bio"?: string | null;
  "isMod"?: boolean | null;
  "isCreator"?: boolean | null;
  "proxyWallet"?: string | null;
  "baseAddress"?: string | null;
  "profileImage"?: string | null;
  "profileImageOptimized"?: GammaImageOptimization | null;
  "positions"?: Array<GammaCommentPosition> | null;
}

export interface GammaCount {
  [field: string]: unknown;
  "count"?: number | bigint | string | null;
}

export interface GammaEvent {
  [field: string]: unknown;
  "id"?: string | null;
  "ticker"?: string | null;
  "slug"?: string | null;
  "title"?: string | null;
  "subtitle"?: string | null;
  "description"?: string | null;
  "resolutionSource"?: string | null;
  "startDate"?: string | null;
  "creationDate"?: string | null;
  "endDate"?: string | null;
  "image"?: string | null;
  "icon"?: string | null;
  "active"?: boolean | null;
  "closed"?: boolean | null;
  "archived"?: boolean | null;
  "new"?: boolean | null;
  "featured"?: boolean | null;
  "restricted"?: boolean | null;
  "liquidity"?: number | bigint | string | null;
  "volume"?: number | bigint | string | null;
  "openInterest"?: number | bigint | string | null;
  "sortBy"?: string | null;
  "category"?: string | null;
  "subcategory"?: string | null;
  "isTemplate"?: boolean | null;
  "templateVariables"?: string | null;
  "published_at"?: string | null;
  "createdBy"?: string | null;
  "updatedBy"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "commentsEnabled"?: boolean | null;
  "competitive"?: number | bigint | string | null;
  "volume24hr"?: number | bigint | string | null;
  "volume1wk"?: number | bigint | string | null;
  "volume1mo"?: number | bigint | string | null;
  "volume1yr"?: number | bigint | string | null;
  "featuredImage"?: string | null;
  "disqusThread"?: string | null;
  "parentEvent"?: string | null;
  "enableOrderBook"?: boolean | null;
  "liquidityClob"?: number | bigint | string | null;
  "negRisk"?: boolean | null;
  "negRiskMarketID"?: string | null;
  "negRiskFeeBips"?: number | bigint | string | null;
  "commentCount"?: number | bigint | string | null;
  "imageOptimized"?: GammaImageOptimization | null;
  "iconOptimized"?: GammaImageOptimization | null;
  "featuredImageOptimized"?: GammaImageOptimization | null;
  "subEvents"?: Array<string> | null;
  "markets"?: Array<GammaMarket> | null;
  "series"?: Array<GammaSeries> | null;
  "categories"?: Array<GammaCategory> | null;
  "collections"?: Array<GammaCollection> | null;
  "tags"?: Array<GammaTag> | null;
  "cyom"?: boolean | null;
  "closedTime"?: string | null;
  "showAllOutcomes"?: boolean | null;
  "showMarketImages"?: boolean | null;
  "automaticallyResolved"?: boolean | null;
  "enableNegRisk"?: boolean | null;
  "automaticallyActive"?: boolean | null;
  "eventDate"?: string | null;
  "startTime"?: string | null;
  "eventWeek"?: number | bigint | string | null;
  "seriesSlug"?: string | null;
  "score"?: string | null;
  "elapsed"?: string | null;
  "period"?: string | null;
  "live"?: boolean | null;
  "ended"?: boolean | null;
  "finishedTimestamp"?: string | null;
  "gmpChartMode"?: string | null;
  "eventCreators"?: Array<GammaEventCreator> | null;
  "tweetCount"?: number | bigint | string | null;
  "chats"?: Array<GammaChat> | null;
  "featuredOrder"?: number | bigint | string | null;
  "estimateValue"?: boolean | null;
  "cantEstimate"?: boolean | null;
  "estimatedValue"?: string | null;
  "templates"?: Array<GammaTemplate> | null;
  "spreadsMainLine"?: number | bigint | string | null;
  "totalsMainLine"?: number | bigint | string | null;
  "carouselMap"?: string | null;
  "pendingDeployment"?: boolean | null;
  "deploying"?: boolean | null;
  "deployingTimestamp"?: string | null;
  "scheduledDeploymentTimestamp"?: string | null;
  "gameStatus"?: string | null;
}

export interface GammaEventCreator {
  [field: string]: unknown;
  "id"?: string | null;
  "creatorName"?: string | null;
  "creatorHandle"?: string | null;
  "creatorUrl"?: string | null;
  "creatorImage"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
}

export interface GammaEventTweetCount {
  [field: string]: unknown;
  "tweetCount"?: number | bigint | string | null;
}

export interface GammaEventsPagination {
  [field: string]: unknown;
  "data"?: Array<GammaEvent> | null;
  "pagination"?: GammaPagination | null;
}

export interface GammaFeeSchedule {
  [field: string]: unknown;
  "exponent"?: number | bigint | string | null;
  "rate"?: number | bigint | string | null;
  "takerOnly"?: boolean | null;
  "rebateRate"?: number | bigint | string | null;
}

export interface GammaGetAbridgedMarketsParams {
  [field: string]: unknown;
  "body": GammaMarketsInformationBody;
}

export interface GammaGetCommentsByIdParams {
  [field: string]: unknown;
  "id": number | bigint | string;
  "get_positions"?: boolean | null;
}

export interface GammaGetCommentsByUserAddressParams {
  [field: string]: unknown;
  "user_address": string;
  "limit"?: number | bigint | string | null;
  "offset"?: number | bigint | string | null;
  "order"?: string | null;
  "ascending"?: boolean | null;
}

export interface GammaGetEventBySlugParams {
  [field: string]: unknown;
  "slug": string;
  "include_chat"?: boolean | null;
  "include_template"?: boolean | null;
}

export interface GammaGetEventBySlugResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "ticker"?: string | null;
  "slug"?: string | null;
  "title"?: string | null;
  "subtitle"?: string | null;
  "description"?: string | null;
  "resolutionSource"?: string | null;
  "startDate"?: string | null;
  "creationDate"?: string | null;
  "endDate"?: string | null;
  "image"?: string | null;
  "icon"?: string | null;
  "active"?: boolean | null;
  "closed"?: boolean | null;
  "archived"?: boolean | null;
  "new"?: boolean | null;
  "featured"?: boolean | null;
  "restricted"?: boolean | null;
  "liquidity"?: number | bigint | string | null;
  "volume"?: number | bigint | string | null;
  "openInterest"?: number | bigint | string | null;
  "sortBy"?: string | null;
  "category"?: string | null;
  "subcategory"?: string | null;
  "isTemplate"?: boolean | null;
  "templateVariables"?: string | null;
  "published_at"?: string | null;
  "createdBy"?: string | null;
  "updatedBy"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "commentsEnabled"?: boolean | null;
  "competitive"?: number | bigint | string | null;
  "volume24hr"?: number | bigint | string | null;
  "volume1wk"?: number | bigint | string | null;
  "volume1mo"?: number | bigint | string | null;
  "volume1yr"?: number | bigint | string | null;
  "featuredImage"?: string | null;
  "disqusThread"?: string | null;
  "parentEvent"?: string | null;
  "enableOrderBook"?: boolean | null;
  "liquidityClob"?: number | bigint | string | null;
  "negRisk"?: boolean | null;
  "negRiskMarketID"?: string | null;
  "negRiskFeeBips"?: number | bigint | string | null;
  "commentCount"?: number | bigint | string | null;
  "imageOptimized"?: GammaImageOptimization | null;
  "iconOptimized"?: GammaImageOptimization | null;
  "featuredImageOptimized"?: GammaImageOptimization | null;
  "subEvents"?: Array<string> | null;
  "markets"?: Array<GammaMarket> | null;
  "series"?: Array<GammaSeries> | null;
  "categories"?: Array<GammaCategory> | null;
  "collections"?: Array<GammaCollection> | null;
  "tags"?: Array<GammaTag> | null;
  "cyom"?: boolean | null;
  "closedTime"?: string | null;
  "showAllOutcomes"?: boolean | null;
  "showMarketImages"?: boolean | null;
  "automaticallyResolved"?: boolean | null;
  "enableNegRisk"?: boolean | null;
  "automaticallyActive"?: boolean | null;
  "eventDate"?: string | null;
  "startTime"?: string | null;
  "eventWeek"?: number | bigint | string | null;
  "seriesSlug"?: string | null;
  "score"?: string | null;
  "elapsed"?: string | null;
  "period"?: string | null;
  "live"?: boolean | null;
  "ended"?: boolean | null;
  "finishedTimestamp"?: string | null;
  "gmpChartMode"?: string | null;
  "eventCreators"?: Array<GammaEventCreator> | null;
  "tweetCount"?: number | bigint | string | null;
  "chats"?: Array<GammaChat> | null;
  "featuredOrder"?: number | bigint | string | null;
  "estimateValue"?: boolean | null;
  "cantEstimate"?: boolean | null;
  "estimatedValue"?: string | null;
  "templates"?: Array<GammaTemplate> | null;
  "spreadsMainLine"?: number | bigint | string | null;
  "totalsMainLine"?: number | bigint | string | null;
  "carouselMap"?: string | null;
  "pendingDeployment"?: boolean | null;
  "deploying"?: boolean | null;
  "deployingTimestamp"?: string | null;
  "scheduledDeploymentTimestamp"?: string | null;
  "gameStatus"?: string | null;
}

export interface GammaGetEventCommentsCountParams {
  [field: string]: unknown;
  "id": number | bigint | string;
}

export interface GammaGetEventCommentsCountResponse {
  [field: string]: unknown;
  "count"?: number | bigint | string | null;
}

export interface GammaGetEventCreatorParams {
  [field: string]: unknown;
  "id": number | bigint | string;
}

export interface GammaGetEventCreatorResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "creatorName"?: string | null;
  "creatorHandle"?: string | null;
  "creatorUrl"?: string | null;
  "creatorImage"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
}

export interface GammaGetEventParams {
  [field: string]: unknown;
  "id": number | bigint | string;
  "include_chat"?: boolean | null;
  "include_template"?: boolean | null;
}

export interface GammaGetEventResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "ticker"?: string | null;
  "slug"?: string | null;
  "title"?: string | null;
  "subtitle"?: string | null;
  "description"?: string | null;
  "resolutionSource"?: string | null;
  "startDate"?: string | null;
  "creationDate"?: string | null;
  "endDate"?: string | null;
  "image"?: string | null;
  "icon"?: string | null;
  "active"?: boolean | null;
  "closed"?: boolean | null;
  "archived"?: boolean | null;
  "new"?: boolean | null;
  "featured"?: boolean | null;
  "restricted"?: boolean | null;
  "liquidity"?: number | bigint | string | null;
  "volume"?: number | bigint | string | null;
  "openInterest"?: number | bigint | string | null;
  "sortBy"?: string | null;
  "category"?: string | null;
  "subcategory"?: string | null;
  "isTemplate"?: boolean | null;
  "templateVariables"?: string | null;
  "published_at"?: string | null;
  "createdBy"?: string | null;
  "updatedBy"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "commentsEnabled"?: boolean | null;
  "competitive"?: number | bigint | string | null;
  "volume24hr"?: number | bigint | string | null;
  "volume1wk"?: number | bigint | string | null;
  "volume1mo"?: number | bigint | string | null;
  "volume1yr"?: number | bigint | string | null;
  "featuredImage"?: string | null;
  "disqusThread"?: string | null;
  "parentEvent"?: string | null;
  "enableOrderBook"?: boolean | null;
  "liquidityClob"?: number | bigint | string | null;
  "negRisk"?: boolean | null;
  "negRiskMarketID"?: string | null;
  "negRiskFeeBips"?: number | bigint | string | null;
  "commentCount"?: number | bigint | string | null;
  "imageOptimized"?: GammaImageOptimization | null;
  "iconOptimized"?: GammaImageOptimization | null;
  "featuredImageOptimized"?: GammaImageOptimization | null;
  "subEvents"?: Array<string> | null;
  "markets"?: Array<GammaMarket> | null;
  "series"?: Array<GammaSeries> | null;
  "categories"?: Array<GammaCategory> | null;
  "collections"?: Array<GammaCollection> | null;
  "tags"?: Array<GammaTag> | null;
  "cyom"?: boolean | null;
  "closedTime"?: string | null;
  "showAllOutcomes"?: boolean | null;
  "showMarketImages"?: boolean | null;
  "automaticallyResolved"?: boolean | null;
  "enableNegRisk"?: boolean | null;
  "automaticallyActive"?: boolean | null;
  "eventDate"?: string | null;
  "startTime"?: string | null;
  "eventWeek"?: number | bigint | string | null;
  "seriesSlug"?: string | null;
  "score"?: string | null;
  "elapsed"?: string | null;
  "period"?: string | null;
  "live"?: boolean | null;
  "ended"?: boolean | null;
  "finishedTimestamp"?: string | null;
  "gmpChartMode"?: string | null;
  "eventCreators"?: Array<GammaEventCreator> | null;
  "tweetCount"?: number | bigint | string | null;
  "chats"?: Array<GammaChat> | null;
  "featuredOrder"?: number | bigint | string | null;
  "estimateValue"?: boolean | null;
  "cantEstimate"?: boolean | null;
  "estimatedValue"?: string | null;
  "templates"?: Array<GammaTemplate> | null;
  "spreadsMainLine"?: number | bigint | string | null;
  "totalsMainLine"?: number | bigint | string | null;
  "carouselMap"?: string | null;
  "pendingDeployment"?: boolean | null;
  "deploying"?: boolean | null;
  "deployingTimestamp"?: string | null;
  "scheduledDeploymentTimestamp"?: string | null;
  "gameStatus"?: string | null;
}

export interface GammaGetEventTagsParams {
  [field: string]: unknown;
  "id": number | bigint | string;
}

export interface GammaGetEventTweetCountParams {
  [field: string]: unknown;
  "id": number | bigint | string;
}

export interface GammaGetEventTweetCountResponse {
  [field: string]: unknown;
  "tweetCount"?: number | bigint | string | null;
}

export interface GammaGetGammaStatusParams {
  [field: string]: unknown;
}

export interface GammaGetMarketBySlugParams {
  [field: string]: unknown;
  "slug": string;
  "include_tag"?: boolean | null;
}

export interface GammaGetMarketBySlugResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "question"?: string | null;
  "conditionId"?: string | null;
  "slug"?: string | null;
  "twitterCardImage"?: string | null;
  "resolutionSource"?: string | null;
  "endDate"?: string | null;
  "category"?: string | null;
  "liquidity"?: string | null;
  "sponsorName"?: string | null;
  "sponsorImage"?: string | null;
  "startDate"?: string | null;
  "xAxisValue"?: string | null;
  "yAxisValue"?: string | null;
  "denominationToken"?: string | null;
  "fee"?: string | null;
  "image"?: string | null;
  "icon"?: string | null;
  "lowerBound"?: string | null;
  "upperBound"?: string | null;
  "description"?: string | null;
  "outcomes"?: string | null;
  "outcomePrices"?: string | null;
  "volume"?: string | null;
  "active"?: boolean | null;
  "marketType"?: string | null;
  "formatType"?: string | null;
  "lowerBoundDate"?: string | null;
  "upperBoundDate"?: string | null;
  "closed"?: boolean | null;
  "createdBy"?: number | bigint | string | null;
  "updatedBy"?: number | bigint | string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "closedTime"?: string | null;
  "wideFormat"?: boolean | null;
  "new"?: boolean | null;
  "mailchimpTag"?: string | null;
  "featured"?: boolean | null;
  "archived"?: boolean | null;
  "resolvedBy"?: string | null;
  "restricted"?: boolean | null;
  "marketGroup"?: number | bigint | string | null;
  "groupItemTitle"?: string | null;
  "groupItemThreshold"?: string | null;
  "questionID"?: string | null;
  "umaEndDate"?: string | null;
  "enableOrderBook"?: boolean | null;
  "orderPriceMinTickSize"?: number | bigint | string | null;
  "orderMinSize"?: number | bigint | string | null;
  "umaResolutionStatus"?: string | null;
  "curationOrder"?: number | bigint | string | null;
  "volumeNum"?: number | bigint | string | null;
  "liquidityNum"?: number | bigint | string | null;
  "endDateIso"?: string | null;
  "startDateIso"?: string | null;
  "umaEndDateIso"?: string | null;
  "hasReviewedDates"?: boolean | null;
  "readyForCron"?: boolean | null;
  "commentsEnabled"?: boolean | null;
  "volume24hr"?: number | bigint | string | null;
  "volume1wk"?: number | bigint | string | null;
  "volume1mo"?: number | bigint | string | null;
  "volume1yr"?: number | bigint | string | null;
  "gameStartTime"?: string | null;
  "secondsDelay"?: number | bigint | string | null;
  "version"?: string | null;
  "clobTokenIds"?: string | null;
  "positionIds"?: Array<string> | null;
  "disqusThread"?: string | null;
  "shortOutcomes"?: string | null;
  "teamAID"?: string | null;
  "teamBID"?: string | null;
  "umaBond"?: string | null;
  "umaReward"?: string | null;
  "volume24hrClob"?: number | bigint | string | null;
  "volume1wkClob"?: number | bigint | string | null;
  "volume1moClob"?: number | bigint | string | null;
  "volume1yrClob"?: number | bigint | string | null;
  "volumeClob"?: number | bigint | string | null;
  "liquidityClob"?: number | bigint | string | null;
  "makerBaseFee"?: number | bigint | string | null;
  "takerBaseFee"?: number | bigint | string | null;
  "customLiveness"?: number | bigint | string | null;
  "acceptingOrders"?: boolean | null;
  "notificationsEnabled"?: boolean | null;
  "score"?: number | bigint | string | null;
  "imageOptimized"?: GammaImageOptimization | null;
  "iconOptimized"?: GammaImageOptimization | null;
  "events"?: Array<GammaEvent> | null;
  "categories"?: Array<GammaCategory> | null;
  "tags"?: Array<GammaTag> | null;
  "creator"?: string | null;
  "ready"?: boolean | null;
  "funded"?: boolean | null;
  "pastSlugs"?: string | null;
  "readyTimestamp"?: string | null;
  "fundedTimestamp"?: string | null;
  "acceptingOrdersTimestamp"?: string | null;
  "competitive"?: number | bigint | string | null;
  "rewardsMinSize"?: number | bigint | string | null;
  "rewardsMaxSpread"?: number | bigint | string | null;
  "spread"?: number | bigint | string | null;
  "automaticallyResolved"?: boolean | null;
  "oneDayPriceChange"?: number | bigint | string | null;
  "oneHourPriceChange"?: number | bigint | string | null;
  "oneWeekPriceChange"?: number | bigint | string | null;
  "oneMonthPriceChange"?: number | bigint | string | null;
  "oneYearPriceChange"?: number | bigint | string | null;
  "lastTradePrice"?: number | bigint | string | null;
  "bestBid"?: number | bigint | string | null;
  "bestAsk"?: number | bigint | string | null;
  "automaticallyActive"?: boolean | null;
  "clearBookOnStart"?: boolean | null;
  "chartColor"?: string | null;
  "seriesColor"?: string | null;
  "showGmpSeries"?: boolean | null;
  "showGmpOutcome"?: boolean | null;
  "manualActivation"?: boolean | null;
  "negRiskOther"?: boolean | null;
  "gameId"?: string | null;
  "groupItemRange"?: string | null;
  "sportsMarketType"?: string | null;
  "line"?: number | bigint | string | null;
  "umaResolutionStatuses"?: string | null;
  "pendingDeployment"?: boolean | null;
  "deploying"?: boolean | null;
  "deployingTimestamp"?: string | null;
  "scheduledDeploymentTimestamp"?: string | null;
  "rfqEnabled"?: boolean | null;
  "eventStartTime"?: string | null;
  "feesEnabled"?: boolean | null;
  "feeSchedule"?: GammaFeeSchedule | null;
}

export interface GammaGetMarketDescriptionParams {
  [field: string]: unknown;
  "id": number | bigint | string;
}

export interface GammaGetMarketDescriptionResponse {
  [field: string]: unknown;
  "description"?: string | null;
}

export interface GammaGetMarketParams {
  [field: string]: unknown;
  "id": number | bigint | string;
  "include_tag"?: boolean | null;
}

export interface GammaGetMarketResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "question"?: string | null;
  "conditionId"?: string | null;
  "slug"?: string | null;
  "twitterCardImage"?: string | null;
  "resolutionSource"?: string | null;
  "endDate"?: string | null;
  "category"?: string | null;
  "liquidity"?: string | null;
  "sponsorName"?: string | null;
  "sponsorImage"?: string | null;
  "startDate"?: string | null;
  "xAxisValue"?: string | null;
  "yAxisValue"?: string | null;
  "denominationToken"?: string | null;
  "fee"?: string | null;
  "image"?: string | null;
  "icon"?: string | null;
  "lowerBound"?: string | null;
  "upperBound"?: string | null;
  "description"?: string | null;
  "outcomes"?: string | null;
  "outcomePrices"?: string | null;
  "volume"?: string | null;
  "active"?: boolean | null;
  "marketType"?: string | null;
  "formatType"?: string | null;
  "lowerBoundDate"?: string | null;
  "upperBoundDate"?: string | null;
  "closed"?: boolean | null;
  "createdBy"?: number | bigint | string | null;
  "updatedBy"?: number | bigint | string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "closedTime"?: string | null;
  "wideFormat"?: boolean | null;
  "new"?: boolean | null;
  "mailchimpTag"?: string | null;
  "featured"?: boolean | null;
  "archived"?: boolean | null;
  "resolvedBy"?: string | null;
  "restricted"?: boolean | null;
  "marketGroup"?: number | bigint | string | null;
  "groupItemTitle"?: string | null;
  "groupItemThreshold"?: string | null;
  "questionID"?: string | null;
  "umaEndDate"?: string | null;
  "enableOrderBook"?: boolean | null;
  "orderPriceMinTickSize"?: number | bigint | string | null;
  "orderMinSize"?: number | bigint | string | null;
  "umaResolutionStatus"?: string | null;
  "curationOrder"?: number | bigint | string | null;
  "volumeNum"?: number | bigint | string | null;
  "liquidityNum"?: number | bigint | string | null;
  "endDateIso"?: string | null;
  "startDateIso"?: string | null;
  "umaEndDateIso"?: string | null;
  "hasReviewedDates"?: boolean | null;
  "readyForCron"?: boolean | null;
  "commentsEnabled"?: boolean | null;
  "volume24hr"?: number | bigint | string | null;
  "volume1wk"?: number | bigint | string | null;
  "volume1mo"?: number | bigint | string | null;
  "volume1yr"?: number | bigint | string | null;
  "gameStartTime"?: string | null;
  "secondsDelay"?: number | bigint | string | null;
  "version"?: string | null;
  "clobTokenIds"?: string | null;
  "positionIds"?: Array<string> | null;
  "disqusThread"?: string | null;
  "shortOutcomes"?: string | null;
  "teamAID"?: string | null;
  "teamBID"?: string | null;
  "umaBond"?: string | null;
  "umaReward"?: string | null;
  "volume24hrClob"?: number | bigint | string | null;
  "volume1wkClob"?: number | bigint | string | null;
  "volume1moClob"?: number | bigint | string | null;
  "volume1yrClob"?: number | bigint | string | null;
  "volumeClob"?: number | bigint | string | null;
  "liquidityClob"?: number | bigint | string | null;
  "makerBaseFee"?: number | bigint | string | null;
  "takerBaseFee"?: number | bigint | string | null;
  "customLiveness"?: number | bigint | string | null;
  "acceptingOrders"?: boolean | null;
  "notificationsEnabled"?: boolean | null;
  "score"?: number | bigint | string | null;
  "imageOptimized"?: GammaImageOptimization | null;
  "iconOptimized"?: GammaImageOptimization | null;
  "events"?: Array<GammaEvent> | null;
  "categories"?: Array<GammaCategory> | null;
  "tags"?: Array<GammaTag> | null;
  "creator"?: string | null;
  "ready"?: boolean | null;
  "funded"?: boolean | null;
  "pastSlugs"?: string | null;
  "readyTimestamp"?: string | null;
  "fundedTimestamp"?: string | null;
  "acceptingOrdersTimestamp"?: string | null;
  "competitive"?: number | bigint | string | null;
  "rewardsMinSize"?: number | bigint | string | null;
  "rewardsMaxSpread"?: number | bigint | string | null;
  "spread"?: number | bigint | string | null;
  "automaticallyResolved"?: boolean | null;
  "oneDayPriceChange"?: number | bigint | string | null;
  "oneHourPriceChange"?: number | bigint | string | null;
  "oneWeekPriceChange"?: number | bigint | string | null;
  "oneMonthPriceChange"?: number | bigint | string | null;
  "oneYearPriceChange"?: number | bigint | string | null;
  "lastTradePrice"?: number | bigint | string | null;
  "bestBid"?: number | bigint | string | null;
  "bestAsk"?: number | bigint | string | null;
  "automaticallyActive"?: boolean | null;
  "clearBookOnStart"?: boolean | null;
  "chartColor"?: string | null;
  "seriesColor"?: string | null;
  "showGmpSeries"?: boolean | null;
  "showGmpOutcome"?: boolean | null;
  "manualActivation"?: boolean | null;
  "negRiskOther"?: boolean | null;
  "gameId"?: string | null;
  "groupItemRange"?: string | null;
  "sportsMarketType"?: string | null;
  "line"?: number | bigint | string | null;
  "umaResolutionStatuses"?: string | null;
  "pendingDeployment"?: boolean | null;
  "deploying"?: boolean | null;
  "deployingTimestamp"?: string | null;
  "scheduledDeploymentTimestamp"?: string | null;
  "rfqEnabled"?: boolean | null;
  "eventStartTime"?: string | null;
  "feesEnabled"?: boolean | null;
  "feeSchedule"?: GammaFeeSchedule | null;
}

export interface GammaGetMarketTagsParams {
  [field: string]: unknown;
  "id": number | bigint | string;
}

export interface GammaGetMarketsInformationParams {
  [field: string]: unknown;
  "body": GammaMarketsInformationBody;
}

export interface GammaGetPublicProfileByUserAddressParams {
  [field: string]: unknown;
  "user_address": string;
}

export interface GammaGetPublicProfileByUserAddressResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "name"?: string | null;
  "user"?: number | bigint | string | null;
  "referral"?: string | null;
  "createdBy"?: number | bigint | string | null;
  "updatedBy"?: number | bigint | string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "utmSource"?: string | null;
  "utmMedium"?: string | null;
  "utmCampaign"?: string | null;
  "utmContent"?: string | null;
  "utmTerm"?: string | null;
  "walletActivated"?: boolean | null;
  "pseudonym"?: string | null;
  "displayUsernamePublic"?: boolean | null;
  "profileImage"?: string | null;
  "bio"?: string | null;
  "proxyWallet"?: string | null;
  "profileImageOptimized"?: GammaImageOptimization | null;
  "isCloseOnly"?: boolean | null;
  "isCertReq"?: boolean | null;
  "certReqDate"?: string | null;
}

export interface GammaGetPublicProfileParams {
  [field: string]: unknown;
  "address": string;
}

export interface GammaGetPublicProfileResponse {
  [field: string]: unknown;
  "createdAt"?: string | null;
  "proxyWallet"?: string | null;
  "profileImage"?: string | null;
  "displayUsernamePublic"?: boolean | null;
  "bio"?: string | null;
  "pseudonym"?: string | null;
  "name"?: string | null;
  "users"?: Array<GammaPublicProfileUser> | null;
  "xUsername"?: string | null;
  "verifiedBadge"?: boolean | null;
}

export interface GammaGetRelatedTagsByIdParams {
  [field: string]: unknown;
  "id": number | bigint | string;
  "omit_empty"?: boolean | null;
  "status"?: string | null;
}

export interface GammaGetRelatedTagsBySlugParams {
  [field: string]: unknown;
  "slug": string;
  "omit_empty"?: boolean | null;
  "status"?: string | null;
}

export interface GammaGetSeriesCommentsCountParams {
  [field: string]: unknown;
  "id": number | bigint | string;
}

export interface GammaGetSeriesCommentsCountResponse {
  [field: string]: unknown;
  "count"?: number | bigint | string | null;
}

export interface GammaGetSeriesParams {
  [field: string]: unknown;
  "id": number | bigint | string;
  "include_chat"?: boolean | null;
}

export interface GammaGetSeriesResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "ticker"?: string | null;
  "slug"?: string | null;
  "title"?: string | null;
  "subtitle"?: string | null;
  "seriesType"?: string | null;
  "recurrence"?: string | null;
  "description"?: string | null;
  "image"?: string | null;
  "icon"?: string | null;
  "layout"?: string | null;
  "active"?: boolean | null;
  "closed"?: boolean | null;
  "archived"?: boolean | null;
  "new"?: boolean | null;
  "featured"?: boolean | null;
  "restricted"?: boolean | null;
  "isTemplate"?: boolean | null;
  "templateVariables"?: boolean | null;
  "publishedAt"?: string | null;
  "createdBy"?: string | null;
  "updatedBy"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "commentsEnabled"?: boolean | null;
  "competitive"?: string | null;
  "volume24hr"?: number | bigint | string | null;
  "volume"?: number | bigint | string | null;
  "liquidity"?: number | bigint | string | null;
  "startDate"?: string | null;
  "pythTokenID"?: string | null;
  "cgAssetName"?: string | null;
  "score"?: number | bigint | string | null;
  "events"?: Array<GammaEvent> | null;
  "collections"?: Array<GammaCollection> | null;
  "categories"?: Array<GammaCategory> | null;
  "tags"?: Array<GammaTag> | null;
  "commentCount"?: number | bigint | string | null;
  "chats"?: Array<GammaChat> | null;
}

export interface GammaGetSeriesSummaryByIdParams {
  [field: string]: unknown;
  "id": number | bigint | string;
}

export interface GammaGetSeriesSummaryByIdResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "title"?: string | null;
  "slug"?: string | null;
  "eventDates"?: Array<string> | null;
  "eventWeeks"?: Array<number | bigint | string> | null;
  "earliest_open_week"?: number | bigint | string | null;
  "earliest_open_date"?: string | null;
}

export interface GammaGetSeriesSummaryBySlugParams {
  [field: string]: unknown;
  "slug": string;
}

export interface GammaGetSeriesSummaryBySlugResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "title"?: string | null;
  "slug"?: string | null;
  "eventDates"?: Array<string> | null;
  "eventWeeks"?: Array<number | bigint | string> | null;
  "earliest_open_week"?: number | bigint | string | null;
  "earliest_open_date"?: string | null;
}

export interface GammaGetSportsMarketTypesParams {
  [field: string]: unknown;
}

export interface GammaGetSportsMarketTypesResponse {
  [field: string]: unknown;
  "marketTypes"?: Array<string> | null;
}

export interface GammaGetSportsMetadataParams {
  [field: string]: unknown;
}

export interface GammaGetTagBySlugParams {
  [field: string]: unknown;
  "slug": string;
  "include_template"?: boolean | null;
}

export interface GammaGetTagBySlugResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "label"?: string | null;
  "slug"?: string | null;
  "forceShow"?: boolean | null;
  "publishedAt"?: string | null;
  "createdBy"?: number | bigint | string | null;
  "updatedBy"?: number | bigint | string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "forceHide"?: boolean | null;
  "isCarousel"?: boolean | null;
}

export interface GammaGetTagParams {
  [field: string]: unknown;
  "id": number | bigint | string;
  "include_template"?: boolean | null;
}

export interface GammaGetTagResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "label"?: string | null;
  "slug"?: string | null;
  "forceShow"?: boolean | null;
  "publishedAt"?: string | null;
  "createdBy"?: number | bigint | string | null;
  "updatedBy"?: number | bigint | string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "forceHide"?: boolean | null;
  "isCarousel"?: boolean | null;
}

export interface GammaGetTagsRelatedToATagByIdParams {
  [field: string]: unknown;
  "id": number | bigint | string;
  "omit_empty"?: boolean | null;
  "status"?: string | null;
}

export interface GammaGetTagsRelatedToATagBySlugParams {
  [field: string]: unknown;
  "slug": string;
  "omit_empty"?: boolean | null;
  "status"?: string | null;
}

export interface GammaGetTeamParams {
  [field: string]: unknown;
  "id": number | bigint | string;
}

export interface GammaGetTeamResponse {
  [field: string]: unknown;
  "id"?: number | bigint | string | null;
  "name"?: string | null;
  "league"?: string | null;
  "record"?: string | null;
  "logo"?: string | null;
  "abbreviation"?: string | null;
  "alias"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
}

export interface GammaImageOptimization {
  [field: string]: unknown;
  "id"?: string | null;
  "imageUrlSource"?: string | null;
  "imageUrlOptimized"?: string | null;
  "imageSizeKbSource"?: number | bigint | string | null;
  "imageSizeKbOptimized"?: number | bigint | string | null;
  "imageOptimizedComplete"?: boolean | null;
  "imageOptimizedLastUpdated"?: string | null;
  "relID"?: number | bigint | string | null;
  "field"?: string | null;
  "relname"?: string | null;
}

export interface GammaInternalError {
  [field: string]: unknown;
  "type"?: string | null;
  "error"?: string | null;
}

export interface GammaKeysetEventsResponse {
  [field: string]: unknown;
  "events"?: Array<GammaEvent> | null;
  "next_cursor"?: string | null;
}

export interface GammaKeysetMarketsResponse {
  [field: string]: unknown;
  "markets"?: Array<GammaMarket> | null;
  "next_cursor"?: string | null;
}

export interface GammaListCommentsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "offset"?: number | bigint | string | null;
  "order"?: string | null;
  "ascending"?: boolean | null;
  "parent_entity_type"?: string | null;
  "parent_entity_id"?: number | bigint | string | null;
  "get_positions"?: boolean | null;
  "holders_only"?: boolean | null;
}

export interface GammaListEventCreatorsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "offset"?: number | bigint | string | null;
  "order"?: string | null;
  "ascending"?: boolean | null;
  "creator_name"?: string | null;
  "creator_handle"?: string | null;
}

export interface GammaListEventsKeysetParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "order"?: string | null;
  "ascending"?: boolean | null;
  "after_cursor"?: string | null;
  "offset"?: number | bigint | string | null;
  "id"?: Array<number | bigint | string> | null;
  "slug"?: Array<string> | null;
  "closed"?: HttpValue | null;
  "live"?: boolean | null;
  "featured"?: boolean | null;
  "cyom"?: boolean | null;
  "title_search"?: string | null;
  "liquidity_min"?: number | bigint | string | null;
  "liquidity_max"?: number | bigint | string | null;
  "volume_min"?: number | bigint | string | null;
  "volume_max"?: number | bigint | string | null;
  "start_date_min"?: string | null;
  "start_date_max"?: string | null;
  "end_date_min"?: string | null;
  "end_date_max"?: string | null;
  "start_time_min"?: string | null;
  "start_time_max"?: string | null;
  "tag_id"?: Array<number | bigint | string> | null;
  "tag_slug"?: string | null;
  "exclude_tag_id"?: Array<number | bigint | string> | null;
  "related_tags"?: boolean | null;
  "tag_match"?: string | null;
  "series_id"?: Array<number | bigint | string> | null;
  "game_id"?: Array<number | bigint | string> | null;
  "event_date"?: string | null;
  "event_week"?: number | bigint | string | null;
  "featured_order"?: boolean | null;
  "recurrence"?: string | null;
  "created_by"?: Array<string> | null;
  "parent_event_id"?: number | bigint | string | null;
  "include_children"?: boolean | null;
  "partner_slug"?: string | null;
  "include_chat"?: boolean | null;
  "include_template"?: boolean | null;
  "include_best_lines"?: boolean | null;
  "locale"?: string | null;
}

export interface GammaListEventsKeysetResponse {
  [field: string]: unknown;
  "events"?: Array<GammaEvent> | null;
  "next_cursor"?: string | null;
}

export interface GammaListEventsPaginationParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "offset"?: number | bigint | string | null;
  "order"?: string | null;
  "ascending"?: boolean | null;
  "include_chat"?: boolean | null;
  "include_template"?: boolean | null;
  "recurrence"?: string | null;
}

export interface GammaListEventsPaginationResponse {
  [field: string]: unknown;
  "data"?: Array<GammaEvent> | null;
  "pagination"?: GammaPagination | null;
}

export interface GammaListEventsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "offset"?: number | bigint | string | null;
  "order"?: string | null;
  "ascending"?: boolean | null;
  "id"?: Array<number | bigint | string> | null;
  "tag_id"?: number | bigint | string | null;
  "exclude_tag_id"?: Array<number | bigint | string> | null;
  "slug"?: Array<string> | null;
  "tag_slug"?: string | null;
  "related_tags"?: boolean | null;
  "active"?: boolean | null;
  "archived"?: boolean | null;
  "featured"?: boolean | null;
  "cyom"?: boolean | null;
  "include_chat"?: boolean | null;
  "include_template"?: boolean | null;
  "recurrence"?: string | null;
  "closed"?: HttpValue | null;
  "liquidity_min"?: number | bigint | string | null;
  "liquidity_max"?: number | bigint | string | null;
  "volume_min"?: number | bigint | string | null;
  "volume_max"?: number | bigint | string | null;
  "start_date_min"?: string | null;
  "start_date_max"?: string | null;
  "end_date_min"?: string | null;
  "end_date_max"?: string | null;
}

export interface GammaListMarketsKeysetParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "order"?: string | null;
  "ascending"?: boolean | null;
  "after_cursor"?: string | null;
  "offset"?: number | bigint | string | null;
  "id"?: Array<number | bigint | string> | null;
  "slug"?: Array<string> | null;
  "closed"?: HttpValue | null;
  "decimalized"?: boolean | null;
  "clob_token_ids"?: Array<string> | null;
  "condition_ids"?: Array<string> | null;
  "question_ids"?: Array<string> | null;
  "liquidity_num_min"?: number | bigint | string | null;
  "liquidity_num_max"?: number | bigint | string | null;
  "volume_num_min"?: number | bigint | string | null;
  "volume_num_max"?: number | bigint | string | null;
  "start_date_min"?: string | null;
  "start_date_max"?: string | null;
  "end_date_min"?: string | null;
  "end_date_max"?: string | null;
  "tag_id"?: Array<number | bigint | string> | null;
  "related_tags"?: boolean | null;
  "tag_match"?: string | null;
  "cyom"?: boolean | null;
  "rfq_enabled"?: boolean | null;
  "uma_resolution_status"?: string | null;
  "game_id"?: string | null;
  "sports_market_types"?: Array<string> | null;
  "include_tag"?: boolean | null;
  "locale"?: string | null;
}

export interface GammaListMarketsKeysetResponse {
  [field: string]: unknown;
  "markets"?: Array<GammaMarket> | null;
  "next_cursor"?: string | null;
}

export interface GammaListMarketsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "offset"?: number | bigint | string | null;
  "order"?: string | null;
  "ascending"?: boolean | null;
  "id"?: Array<number | bigint | string> | null;
  "slug"?: Array<string> | null;
  "clob_token_ids"?: Array<string> | null;
  "condition_ids"?: Array<string> | null;
  "liquidity_num_min"?: number | bigint | string | null;
  "liquidity_num_max"?: number | bigint | string | null;
  "volume_num_min"?: number | bigint | string | null;
  "volume_num_max"?: number | bigint | string | null;
  "start_date_min"?: string | null;
  "start_date_max"?: string | null;
  "end_date_min"?: string | null;
  "end_date_max"?: string | null;
  "tag_id"?: number | bigint | string | null;
  "related_tags"?: boolean | null;
  "cyom"?: boolean | null;
  "uma_resolution_status"?: string | null;
  "game_id"?: string | null;
  "sports_market_types"?: Array<string> | null;
  "rewards_min_size"?: number | bigint | string | null;
  "question_ids"?: Array<string> | null;
  "include_tag"?: boolean | null;
  "closed"?: HttpValue | null;
}

export interface GammaListSeriesParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "offset"?: number | bigint | string | null;
  "order"?: string | null;
  "ascending"?: boolean | null;
  "slug"?: Array<string> | null;
  "categories_ids"?: Array<number | bigint | string> | null;
  "categories_labels"?: Array<string> | null;
  "closed"?: boolean | null;
  "include_chat"?: boolean | null;
  "recurrence"?: string | null;
  "exclude_events"?: boolean | null;
}

export interface GammaListSportEventsResultsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "offset"?: number | bigint | string | null;
  "order"?: string | null;
  "ascending"?: boolean | null;
}

export interface GammaListTagsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "offset"?: number | bigint | string | null;
  "order"?: string | null;
  "ascending"?: boolean | null;
  "include_template"?: boolean | null;
  "is_carousel"?: boolean | null;
}

export interface GammaListTeamsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "offset"?: number | bigint | string | null;
  "order"?: string | null;
  "ascending"?: boolean | null;
  "league"?: Array<string> | null;
  "name"?: Array<string> | null;
  "abbreviation"?: Array<string> | null;
}

export interface GammaMarket {
  [field: string]: unknown;
  "id"?: string | null;
  "question"?: string | null;
  "conditionId"?: string | null;
  "slug"?: string | null;
  "twitterCardImage"?: string | null;
  "resolutionSource"?: string | null;
  "endDate"?: string | null;
  "category"?: string | null;
  "liquidity"?: string | null;
  "sponsorName"?: string | null;
  "sponsorImage"?: string | null;
  "startDate"?: string | null;
  "xAxisValue"?: string | null;
  "yAxisValue"?: string | null;
  "denominationToken"?: string | null;
  "fee"?: string | null;
  "image"?: string | null;
  "icon"?: string | null;
  "lowerBound"?: string | null;
  "upperBound"?: string | null;
  "description"?: string | null;
  "outcomes"?: string | null;
  "outcomePrices"?: string | null;
  "volume"?: string | null;
  "active"?: boolean | null;
  "marketType"?: string | null;
  "formatType"?: string | null;
  "lowerBoundDate"?: string | null;
  "upperBoundDate"?: string | null;
  "closed"?: boolean | null;
  "createdBy"?: number | bigint | string | null;
  "updatedBy"?: number | bigint | string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "closedTime"?: string | null;
  "wideFormat"?: boolean | null;
  "new"?: boolean | null;
  "mailchimpTag"?: string | null;
  "featured"?: boolean | null;
  "archived"?: boolean | null;
  "resolvedBy"?: string | null;
  "restricted"?: boolean | null;
  "marketGroup"?: number | bigint | string | null;
  "groupItemTitle"?: string | null;
  "groupItemThreshold"?: string | null;
  "questionID"?: string | null;
  "umaEndDate"?: string | null;
  "enableOrderBook"?: boolean | null;
  "orderPriceMinTickSize"?: number | bigint | string | null;
  "orderMinSize"?: number | bigint | string | null;
  "umaResolutionStatus"?: string | null;
  "curationOrder"?: number | bigint | string | null;
  "volumeNum"?: number | bigint | string | null;
  "liquidityNum"?: number | bigint | string | null;
  "endDateIso"?: string | null;
  "startDateIso"?: string | null;
  "umaEndDateIso"?: string | null;
  "hasReviewedDates"?: boolean | null;
  "readyForCron"?: boolean | null;
  "commentsEnabled"?: boolean | null;
  "volume24hr"?: number | bigint | string | null;
  "volume1wk"?: number | bigint | string | null;
  "volume1mo"?: number | bigint | string | null;
  "volume1yr"?: number | bigint | string | null;
  "gameStartTime"?: string | null;
  "secondsDelay"?: number | bigint | string | null;
  "version"?: string | null;
  "clobTokenIds"?: string | null;
  "positionIds"?: Array<string> | null;
  "disqusThread"?: string | null;
  "shortOutcomes"?: string | null;
  "teamAID"?: string | null;
  "teamBID"?: string | null;
  "umaBond"?: string | null;
  "umaReward"?: string | null;
  "volume24hrClob"?: number | bigint | string | null;
  "volume1wkClob"?: number | bigint | string | null;
  "volume1moClob"?: number | bigint | string | null;
  "volume1yrClob"?: number | bigint | string | null;
  "volumeClob"?: number | bigint | string | null;
  "liquidityClob"?: number | bigint | string | null;
  "makerBaseFee"?: number | bigint | string | null;
  "takerBaseFee"?: number | bigint | string | null;
  "customLiveness"?: number | bigint | string | null;
  "acceptingOrders"?: boolean | null;
  "notificationsEnabled"?: boolean | null;
  "score"?: number | bigint | string | null;
  "imageOptimized"?: GammaImageOptimization | null;
  "iconOptimized"?: GammaImageOptimization | null;
  "events"?: Array<GammaEvent> | null;
  "categories"?: Array<GammaCategory> | null;
  "tags"?: Array<GammaTag> | null;
  "creator"?: string | null;
  "ready"?: boolean | null;
  "funded"?: boolean | null;
  "pastSlugs"?: string | null;
  "readyTimestamp"?: string | null;
  "fundedTimestamp"?: string | null;
  "acceptingOrdersTimestamp"?: string | null;
  "competitive"?: number | bigint | string | null;
  "rewardsMinSize"?: number | bigint | string | null;
  "rewardsMaxSpread"?: number | bigint | string | null;
  "spread"?: number | bigint | string | null;
  "automaticallyResolved"?: boolean | null;
  "oneDayPriceChange"?: number | bigint | string | null;
  "oneHourPriceChange"?: number | bigint | string | null;
  "oneWeekPriceChange"?: number | bigint | string | null;
  "oneMonthPriceChange"?: number | bigint | string | null;
  "oneYearPriceChange"?: number | bigint | string | null;
  "lastTradePrice"?: number | bigint | string | null;
  "bestBid"?: number | bigint | string | null;
  "bestAsk"?: number | bigint | string | null;
  "automaticallyActive"?: boolean | null;
  "clearBookOnStart"?: boolean | null;
  "chartColor"?: string | null;
  "seriesColor"?: string | null;
  "showGmpSeries"?: boolean | null;
  "showGmpOutcome"?: boolean | null;
  "manualActivation"?: boolean | null;
  "negRiskOther"?: boolean | null;
  "gameId"?: string | null;
  "groupItemRange"?: string | null;
  "sportsMarketType"?: string | null;
  "line"?: number | bigint | string | null;
  "umaResolutionStatuses"?: string | null;
  "pendingDeployment"?: boolean | null;
  "deploying"?: boolean | null;
  "deployingTimestamp"?: string | null;
  "scheduledDeploymentTimestamp"?: string | null;
  "rfqEnabled"?: boolean | null;
  "eventStartTime"?: string | null;
  "feesEnabled"?: boolean | null;
  "feeSchedule"?: GammaFeeSchedule | null;
}

export interface GammaMarketDescription {
  [field: string]: unknown;
  "description"?: string | null;
}

export interface GammaMarketsInformationBody {
  [field: string]: unknown;
  "id"?: Array<number | bigint | string> | null;
  "slug"?: Array<string> | null;
  "closed"?: boolean | null;
  "clobTokenIds"?: Array<string> | null;
  "conditionIds"?: Array<string> | null;
  "liquidityNumMin"?: number | bigint | string | null;
  "liquidityNumMax"?: number | bigint | string | null;
  "volumeNumMin"?: number | bigint | string | null;
  "volumeNumMax"?: number | bigint | string | null;
  "startDateMin"?: string | null;
  "startDateMax"?: string | null;
  "endDateMin"?: string | null;
  "endDateMax"?: string | null;
  "relatedTags"?: boolean | null;
  "tagId"?: number | bigint | string | null;
  "cyom"?: boolean | null;
  "umaResolutionStatus"?: string | null;
  "gameId"?: string | null;
  "sportsMarketTypes"?: Array<string> | null;
  "rewardsMinSize"?: number | bigint | string | null;
  "questionIds"?: Array<string> | null;
  "includeTags"?: boolean | null;
}

export interface GammaPagination {
  [field: string]: unknown;
  "hasMore"?: boolean | null;
  "totalResults"?: number | bigint | string | null;
}

export interface GammaProfile {
  [field: string]: unknown;
  "id"?: string | null;
  "name"?: string | null;
  "user"?: number | bigint | string | null;
  "referral"?: string | null;
  "createdBy"?: number | bigint | string | null;
  "updatedBy"?: number | bigint | string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "utmSource"?: string | null;
  "utmMedium"?: string | null;
  "utmCampaign"?: string | null;
  "utmContent"?: string | null;
  "utmTerm"?: string | null;
  "walletActivated"?: boolean | null;
  "pseudonym"?: string | null;
  "displayUsernamePublic"?: boolean | null;
  "profileImage"?: string | null;
  "bio"?: string | null;
  "proxyWallet"?: string | null;
  "profileImageOptimized"?: GammaImageOptimization | null;
  "isCloseOnly"?: boolean | null;
  "isCertReq"?: boolean | null;
  "certReqDate"?: string | null;
}

export interface GammaPublicProfileError {
  [field: string]: unknown;
  "type"?: string | null;
  "error"?: string | null;
}

export interface GammaPublicProfileResponse {
  [field: string]: unknown;
  "createdAt"?: string | null;
  "proxyWallet"?: string | null;
  "profileImage"?: string | null;
  "displayUsernamePublic"?: boolean | null;
  "bio"?: string | null;
  "pseudonym"?: string | null;
  "name"?: string | null;
  "users"?: Array<GammaPublicProfileUser> | null;
  "xUsername"?: string | null;
  "verifiedBadge"?: boolean | null;
}

export interface GammaPublicProfileUser {
  [field: string]: unknown;
  "id"?: string | null;
  "creator"?: boolean | null;
  "mod"?: boolean | null;
}

export interface GammaPublicSearchParams {
  [field: string]: unknown;
  "q": string;
  "cache"?: boolean | null;
  "events_status"?: string | null;
  "limit_per_type"?: number | bigint | string | null;
  "page"?: number | bigint | string | null;
  "events_tag"?: Array<string> | null;
  "keep_closed_markets"?: number | bigint | string | null;
  "sort"?: string | null;
  "ascending"?: boolean | null;
  "search_tags"?: boolean | null;
  "search_profiles"?: boolean | null;
  "recurrence"?: string | null;
  "exclude_tag_id"?: Array<number | bigint | string> | null;
  "optimized"?: boolean | null;
}

export interface GammaPublicSearchResponse {
  [field: string]: unknown;
  "events"?: Array<GammaEvent> | null;
  "tags"?: Array<GammaSearchTag> | null;
  "profiles"?: Array<GammaProfile> | null;
  "pagination"?: GammaPagination | null;
}

export interface GammaReaction {
  [field: string]: unknown;
  "id"?: string | null;
  "commentID"?: number | bigint | string | null;
  "reactionType"?: string | null;
  "icon"?: string | null;
  "userAddress"?: string | null;
  "createdAt"?: string | null;
  "profile"?: GammaCommentProfile | null;
}

export interface GammaRelatedTag {
  [field: string]: unknown;
  "id"?: string | null;
  "tagID"?: number | bigint | string | null;
  "relatedTagID"?: number | bigint | string | null;
  "rank"?: number | bigint | string | null;
}

export interface GammaSearch {
  [field: string]: unknown;
  "events"?: Array<GammaEvent> | null;
  "tags"?: Array<GammaSearchTag> | null;
  "profiles"?: Array<GammaProfile> | null;
  "pagination"?: GammaPagination | null;
}

export interface GammaSearchTag {
  [field: string]: unknown;
  "id"?: string | null;
  "label"?: string | null;
  "slug"?: string | null;
  "event_count"?: number | bigint | string | null;
}

export interface GammaSeries {
  [field: string]: unknown;
  "id"?: string | null;
  "ticker"?: string | null;
  "slug"?: string | null;
  "title"?: string | null;
  "subtitle"?: string | null;
  "seriesType"?: string | null;
  "recurrence"?: string | null;
  "description"?: string | null;
  "image"?: string | null;
  "icon"?: string | null;
  "layout"?: string | null;
  "active"?: boolean | null;
  "closed"?: boolean | null;
  "archived"?: boolean | null;
  "new"?: boolean | null;
  "featured"?: boolean | null;
  "restricted"?: boolean | null;
  "isTemplate"?: boolean | null;
  "templateVariables"?: boolean | null;
  "publishedAt"?: string | null;
  "createdBy"?: string | null;
  "updatedBy"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "commentsEnabled"?: boolean | null;
  "competitive"?: string | null;
  "volume24hr"?: number | bigint | string | null;
  "volume"?: number | bigint | string | null;
  "liquidity"?: number | bigint | string | null;
  "startDate"?: string | null;
  "pythTokenID"?: string | null;
  "cgAssetName"?: string | null;
  "score"?: number | bigint | string | null;
  "events"?: Array<GammaEvent> | null;
  "collections"?: Array<GammaCollection> | null;
  "categories"?: Array<GammaCategory> | null;
  "tags"?: Array<GammaTag> | null;
  "commentCount"?: number | bigint | string | null;
  "chats"?: Array<GammaChat> | null;
}

export interface GammaSeriesSummary {
  [field: string]: unknown;
  "id"?: string | null;
  "title"?: string | null;
  "slug"?: string | null;
  "eventDates"?: Array<string> | null;
  "eventWeeks"?: Array<number | bigint | string> | null;
  "earliest_open_week"?: number | bigint | string | null;
  "earliest_open_date"?: string | null;
}

export interface GammaServiceUnavailableError {
  [field: string]: unknown;
  "type"?: string | null;
  "error"?: string | null;
}

export interface GammaSportsMarketTypesResponse {
  [field: string]: unknown;
  "marketTypes"?: Array<string> | null;
}

export interface GammaSportsMetadata {
  [field: string]: unknown;
  "sport"?: string | null;
  "image"?: string | null;
  "resolution"?: string | null;
  "ordering"?: string | null;
  "tags"?: string | null;
  "series"?: string | null;
}

export interface GammaTag {
  [field: string]: unknown;
  "id"?: string | null;
  "label"?: string | null;
  "slug"?: string | null;
  "forceShow"?: boolean | null;
  "publishedAt"?: string | null;
  "createdBy"?: number | bigint | string | null;
  "updatedBy"?: number | bigint | string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
  "forceHide"?: boolean | null;
  "isCarousel"?: boolean | null;
}

export interface GammaTeam {
  [field: string]: unknown;
  "id"?: number | bigint | string | null;
  "name"?: string | null;
  "league"?: string | null;
  "record"?: string | null;
  "logo"?: string | null;
  "abbreviation"?: string | null;
  "alias"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
}

export interface GammaTemplate {
  [field: string]: unknown;
  "id"?: string | null;
  "eventTitle"?: string | null;
  "eventSlug"?: string | null;
  "eventImage"?: string | null;
  "marketTitle"?: string | null;
  "description"?: string | null;
  "resolutionSource"?: string | null;
  "negRisk"?: boolean | null;
  "sortBy"?: string | null;
  "showMarketImages"?: boolean | null;
  "seriesSlug"?: string | null;
  "outcomes"?: string | null;
}

export interface GammaValidationError {
  [field: string]: unknown;
  "type"?: string | null;
  "error"?: string | null;
}

export interface RelayerAuthorizationsParams {
  [field: string]: unknown;
  "Idempotency-Key": string;
  "body": RelayerAuthorizationsParamsBody;
}

export interface RelayerAuthorizationsParamsBody {
  [field: string]: unknown;
  "scopes": Array<string>;
  "validUntil": string;
  "deadline": string;
  "nonce"?: string;
  "sessionSignerAddress": string;
  "signature"?: string;
  "walletAddress": string;
}

export interface RelayerDeployedResponse {
  [field: string]: unknown;
  "deployed"?: boolean | null;
}

export interface RelayerErrorResponse {
  [field: string]: unknown;
  "error": string;
}

export interface RelayerGetDeployedParams {
  [field: string]: unknown;
  "address": string;
  "type"?: string | null;
}

export interface RelayerGetDeployedResponse {
  [field: string]: unknown;
  "deployed"?: boolean | null;
}

export interface RelayerGetExecuteParamsParams {
  [field: string]: unknown;
  "address": string;
  "type": string;
}

export interface RelayerGetNonceParams {
  [field: string]: unknown;
  "address": string;
  "type": string;
}

export interface RelayerGetNonceResponse {
  [field: string]: unknown;
  "nonce"?: string | null;
}

export interface RelayerGetRelayPayloadParams {
  [field: string]: unknown;
  "address": string;
  "type": string;
}

export interface RelayerGetRelayPayloadResponse {
  [field: string]: unknown;
  "address"?: string | null;
  "nonce"?: string | null;
}

export interface RelayerGetRelayerApiKeysParams {
  [field: string]: unknown;
  "RELAYER_API_KEY"?: string | null;
  "RELAYER_API_KEY_ADDRESS"?: string | null;
}

export interface RelayerGetTransactionParams {
  [field: string]: unknown;
  "id": string;
}

export interface RelayerGetTransactionsParams {
  [field: string]: unknown;
  "POLY_BUILDER_API_KEY"?: string | null;
  "POLY_BUILDER_TIMESTAMP"?: string | null;
  "POLY_BUILDER_PASSPHRASE"?: string | null;
  "POLY_BUILDER_SIGNATURE"?: string | null;
  "RELAYER_API_KEY"?: string | null;
  "RELAYER_API_KEY_ADDRESS"?: string | null;
}

export interface RelayerNonceResponse {
  [field: string]: unknown;
  "nonce"?: string | null;
}

export interface RelayerPostSubmitParams {
  [field: string]: unknown;
  "POLY_BUILDER_API_KEY"?: string | null;
  "POLY_BUILDER_TIMESTAMP"?: string | null;
  "POLY_BUILDER_PASSPHRASE"?: string | null;
  "POLY_BUILDER_SIGNATURE"?: string | null;
  "RELAYER_API_KEY"?: string | null;
  "RELAYER_API_KEY_ADDRESS"?: string | null;
  "body": RelayerSubmitRequest;
}

export interface RelayerPostSubmitResponse {
  [field: string]: unknown;
  "transactionID"?: string | null;
  "state"?: string | null;
}

export interface RelayerRelayPayloadResponse {
  [field: string]: unknown;
  "address"?: string | null;
  "nonce"?: string | null;
}

export interface RelayerRelayerApiKey {
  [field: string]: unknown;
  "apiKey"?: string | null;
  "address"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
}

export interface RelayerRelayerTransaction {
  [field: string]: unknown;
  "transactionID"?: string | null;
  "transactionHash"?: string | null;
  "from"?: string | null;
  "to"?: string | null;
  "proxyAddress"?: string | null;
  "data"?: string | null;
  "nonce"?: string | null;
  "value"?: string | null;
  "signature"?: string | null;
  "state"?: string | null;
  "type"?: string | null;
  "owner"?: string | null;
  "metadata"?: string | null;
  "createdAt"?: string | null;
  "updatedAt"?: string | null;
}

export interface RelayerRevocationsParams {
  [field: string]: unknown;
  "Idempotency-Key": string;
  "body": RelayerRevocationsParamsBody;
}

export interface RelayerRevocationsParamsBody {
  [field: string]: unknown;
  "deadline": string;
  "nonce"?: string;
  "sessionSignerAddress": string;
  "signature"?: string;
  "walletAddress": string;
}

export interface RelayerSignatureParams {
  [field: string]: unknown;
  "gasPrice"?: string | null;
  "operation"?: string | null;
  "safeTxnGas"?: string | null;
  "baseGas"?: string | null;
  "gasToken"?: string | null;
  "refundReceiver"?: string | null;
}

export interface RelayerSubmitRequest {
  [field: string]: unknown;
  "from": string;
  "to": string;
  "proxyWallet": string;
  "data": string;
  "nonce"?: string;
  "signature"?: string;
  "signatureParams": RelayerSignatureParams;
  "type": string;
}

export interface RelayerSubmitResponse {
  [field: string]: unknown;
  "transactionID"?: string | null;
  "state"?: string | null;
}
