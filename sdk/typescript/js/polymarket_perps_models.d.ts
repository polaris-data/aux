// Generated from src/exchanges/polymarket/schema/polymarket_perps.json.
import type { HttpValue } from "./index";
export type CancelChaseResponse = HttpValue;
export type PerpsCancelResponse = HttpValue;
export type CancelTwapResponse = HttpValue;
export type ControlTwapResponse = HttpValue;
export type CreateChaseResponse = HttpValue;
export type PerpsOrderResponse = HttpValue;
export type CreateTwapResponse = HttpValue;
export type GetAssetsResponse = Array<PerpsAsset>;
export type GetBalancesResponse = Array<PerpsBalance>;
export type GetBboResponse = Array<PerpsBBO>;
export type GetChasesResponse = HttpValue;
export type GetConfigResponse = Array<PerpsAccountConfig>;
export type GetInstrumentsResponse = Array<PerpsInstrument>;
export type GetLimitTiersResponse = Array<PerpsLimitTier>;
export type GetOpenOrdersResponse = Array<PerpsOrderData>;
export type GetOrdersResponse = Array<PerpsOrderData>;
export type GetStatisticsResponse = Array<PerpsStatistic>;
export type GetTickersResponse = Array<PerpsTicker>;
export type GetTwapsResponse = HttpValue;
export type PerpsModifyResponse = HttpValue;
export type PerpsAccount = string;
export type PerpsAccountBackstopsCursor = string;
export type PerpsAccountMakerShare7d = string;
export type PerpsAccountOi = string;
export type PerpsAccountValue = string;
export type PerpsActionsBurst = number | bigint | string;
export type PerpsActionsBurstLimit = number | bigint | string;
export type PerpsActionsPerMinute = number | bigint | string;
export type PerpsActionsPerMinuteLimit = number | bigint | string;
export type PerpsActiveChildrenCount = number | bigint | string;
export type PerpsAddress = string;
export type PerpsAdl = boolean;
export type PerpsAmount = string;
export type PerpsArts = number | bigint | string;
export type PerpsAssetField = string;
export type PerpsAutoCancelDailyLimit = number | bigint | string;
export type PerpsAutoCancelDeadline = number | bigint | string;
export type PerpsAutoCancelNextReset = number | bigint | string;
export type PerpsAutoCancelTriggered = number | bigint | string;
export type PerpsAvailableOrderMargin = string;
export type PerpsBackstopCashAfter = string;
export type PerpsBackstopCashBefore = string;
export type PerpsBackstopTransferred = string;
export type PerpsBalanceField = string;
export type PerpsBaseAsset = string;
export type PerpsBestAskPrice = string;
export type PerpsBestAskQuantity = string;
export type PerpsBestBidPrice = string;
export type PerpsBestBidQuantity = string;
export type PerpsBlp = boolean;
export type PerpsBlpActive = boolean;
export type PerpsBlpCursor = number | bigint | string;
export type PerpsBlpSort = string;
export type PerpsBuilderApprovalVersion = number | bigint | string;
export type PerpsBuilderFee = string;
export type PerpsBuilderFeeRate = string;
export type PerpsBuy = boolean;
export type PerpsCancelOnly = boolean;
export type PerpsCancelTs = number | bigint | string;
export type PerpsCategory = string;
export type PerpsChainId = number | bigint | string;
export type PerpsChildrenCount = number | bigint | string;
export type PerpsChildrenFees = string;
export type PerpsChildrenVolume = string;
export type PerpsCloseOnly = boolean;
export type PerpsCode = string;
export type PerpsCoid = string;
export type PerpsCollateralRatio = string;
export type PerpsCompetitionFirstFillAt = number | bigint | string;
export type PerpsCompetitionId = number | bigint | string;
export type PerpsCompetitionReadError = string;
export type PerpsCompetitionReadTimestamp = number | bigint | string;
export type PerpsCompetitionRowState = string;
export type PerpsCompetitionScore = string;
export type PerpsCompetitionStatus = string;
export type PerpsConfirmations = number | bigint | string;
export type PerpsConfirmedTimestamp = number | bigint | string;
export type PerpsContract = string;
export type PerpsCounterparty = string;
export type PerpsCreatedTimestamp = number | bigint | string;
export type PerpsCross = boolean;
export type PerpsCts = number | bigint | string;
export type PerpsCumulativeFunding = string;
export type PerpsCursor = string;
export type PerpsDate = string;
export type PerpsDecimals = number | bigint | string;
export type PerpsDepositStatus = string;
export type PerpsDepth = number | bigint | string;
export type PerpsDirection = string;
export type PerpsDisplaySymbol = string;
export type PerpsDurableSourceSeq = number | bigint | string;
export type PerpsEndDate = string;
export type PerpsEndTimestamp = number | bigint | string;
export type PerpsEngineVersion = string;
export type PerpsEntityId = number | bigint | string;
export type PerpsEntityMakerShare7d = string;
export type PerpsEntityName = string;
export type PerpsEntryPrice = string;
export type PerpsEquity = string;
export type PerpsError = string;
export type PerpsExchangeFees = string;
export type PerpsExchangeOpenInterest = string;
export type PerpsExchangeVolume = string;
export type PerpsExp = number | bigint | string;
export type PerpsExpiry = number | bigint | string;
export type PerpsFea = string;
export type PerpsFee = string;
export type PerpsFeeShareRate = string;
export type PerpsFeeTierField = number | bigint | string;
export type PerpsFid = number | bigint | string;
export type PerpsFill = string;
export type PerpsFillsCursor = string;
export type PerpsFr = string;
export type PerpsFrom = string;
export type PerpsFua = string;
export type PerpsFund = string;
export type PerpsFundingInterval = string;
export type PerpsFundingRateField = string;
export type PerpsGenericResponse = HttpValue;
export type PerpsGrp = string;
export type PerpsHash = string;
export type PerpsHistorySequence = number | bigint | string;
export type PerpsIid = number | bigint | string;
export type PerpsInLiquidation = boolean;
export type PerpsIndexPrice = string;
export type PerpsInitialMargin = string;
export type PerpsInstrumentId = number | bigint | string;
export type PerpsInstrumentType = string;
export type PerpsInterval = string;
export type PerpsIsolatedOnly = boolean;
export type PerpsKind = string;
export type PerpsKline = Array<HttpValue>;
export type PerpsLabel = string;
export type PerpsLastPrice = string;
export type PerpsLev = number | bigint | string;
export type PerpsLevel = Array<string>;
export type PerpsLeverage = number | bigint | string;
export type PerpsLeverageBatchResponse = HttpValue;
export type PerpsLid = number | bigint | string;
export type PerpsLimit = number | bigint | string;
export type PerpsLiq = boolean;
export type PerpsLiquidationFee = string;
export type PerpsLiquidationPrice = string;
export type PerpsLowerBound = string;
export type PerpsMaintenanceMarginAmount = string;
export type PerpsMakerFeeRate = string;
export type PerpsMakerShare7d = string;
export type PerpsMakerVolume7d = string;
export type PerpsMarginAmount = string;
export type PerpsMarkPoint = Array<HttpValue>;
export type PerpsMarkPrice = string;
export type PerpsMarket = boolean;
export type PerpsMaxLeverage = number | bigint | string;
export type PerpsMaxLimitNotional = string;
export type PerpsMaxMarketNotional = string;
export type PerpsMaxOrderCount = number | bigint | string;
export type PerpsMessagesPerMinute = number | bigint | string;
export type PerpsMgn = string;
export type PerpsMidPrice = string;
export type PerpsMinNotional = string;
export type PerpsMinVolume = string;
export type PerpsMinVolume30d = string;
export type PerpsMore = boolean;
export type PerpsName = string;
export type PerpsNextFunding = number | bigint | string;
export type PerpsNotifCursor = string;
export type PerpsNotifId = string;
export type PerpsNotificationsReadRequest = HttpValue;
export type PerpsNotional = string;
export type PerpsOffset = number | bigint | string;
export type PerpsOid = number | bigint | string;
export type PerpsOpAmount = string;
export type PerpsOpenInterest = string;
export type PerpsOpenInterestTimestamp = number | bigint | string;
export type PerpsOpenOrders = number | bigint | string;
export type PerpsOpenOrdersLimit = number | bigint | string;
export type PerpsOpenPrice = string;
export type PerpsOrderId = number | bigint | string;
export type PerpsOwner = string;
export type PerpsP = string;
export type PerpsParentOid = number | bigint | string;
export type PerpsPep = string;
export type PerpsPnl = string;
export type PerpsPnlInterval = string;
export type PerpsPo = boolean;
export type PerpsPositionSnapshotStatus = string;
export type PerpsPositionValue = string;
export type PerpsPrice = string;
export type PerpsPriceBounds = string;
export type PerpsPriceDecimals = number | bigint | string;
export type PerpsProxy = string;
export type PerpsPsz = string;
export type PerpsQty = string;
export type PerpsQuantity = string;
export type PerpsQuantityDecimals = number | bigint | string;
export type PerpsQuoteAsset = string;
export type PerpsRank = number | bigint | string;
export type PerpsRateBurstLimit = number | bigint | string;
export type PerpsRatePerMinuteLimit = number | bigint | string;
export type PerpsRateReset = number | bigint | string;
export type PerpsReadAt = number | bigint | string;
export type PerpsRef = string;
export type PerpsReferralsLeft = number | bigint | string;
export type PerpsRequiredConfirmations = number | bigint | string;
export type PerpsRest = string;
export type PerpsReturnOnEquity = string;
export type PerpsRewardAmount = string;
export type PerpsRewardDistributed = boolean;
export type PerpsRewardPool = string;
export type PerpsRo = boolean;
export type PerpsRoi = string;
export type PerpsSalt = number | bigint | string;
export type PerpsSecret = string;
export type PerpsSequence = number | bigint | string;
export type PerpsSettlementFill = boolean;
export type PerpsSettlementInsuranceDebit = string;
export type PerpsSettlementPrice = string;
export type PerpsSettlementSequence = number | bigint | string;
export type PerpsSettlementTimestamp = number | bigint | string;
export type PerpsSide = string;
export type PerpsSig = string;
export type PerpsSinceSeq = number | bigint | string;
export type PerpsSize = string;
export type PerpsSlipBps = number | bigint | string;
export type PerpsSort = string;
export type PerpsSortBy = string;
export type PerpsSource = string;
export type PerpsSt = string;
export type PerpsStartDate = string;
export type PerpsStartTimestamp = number | bigint | string;
export type PerpsSymbol = string;
export type PerpsSz = string;
export type PerpsTaker = boolean;
export type PerpsTakerFeeRate = string;
export type PerpsTakerVolume7d = string;
export type PerpsTid = number | bigint | string;
export type PerpsTif = string;
export type PerpsTimeField = number | bigint | string;
export type PerpsTimestamp = number | bigint | string;
export type PerpsTo = string;
export type PerpsToken = string;
export type PerpsTotal = number | bigint | string;
export type PerpsTotalAccountValue = string;
export type PerpsTotalFee = string;
export type PerpsTotalInitialMargin = string;
export type PerpsTotalMaintenanceMargin = string;
export type PerpsTotalPositionValue = string;
export type PerpsTpsl = string;
export type PerpsTrScope = string;
export type PerpsTransferId = number | bigint | string;
export type PerpsTrp = string;
export type PerpsTs = number | bigint | string;
export type PerpsUiLiveTime = number | bigint | string;
export type PerpsUnread = number | bigint | string;
export type PerpsUnrealizedPnl = string;
export type PerpsUpdateTimestamp = number | bigint | string;
export type PerpsUts = number | bigint | string;
export type PerpsValue = string;
export type PerpsVersion = string;
export type PerpsVolume = string;
export type PerpsVolume7d = string;
export type PerpsWeight = string;
export type PerpsWindow = string;
export type PerpsWithdrawId = number | bigint | string;
export type PerpsWithdrawable = string;
export type PerpsWithdrawalFee = string;
export type PerpsWithdrawalStatus = string;
export type UpdateMarginResponse = HttpValue;
export type CancelOrdersByCoidResponse = Array<PerpsCancelResponse>;
export type CancelOrdersResponse = Array<PerpsCancelResponse>;
export type CreateOrdersResponse = Array<PerpsOrderResponse>;
export type ModifyOrdersByCoidResponse = Array<PerpsModifyResponse>;
export type ModifyOrdersResponse = Array<PerpsModifyResponse>;
export type PerpsKlines = Array<PerpsKline>;
export type UpdateLeveragesResponse = Array<PerpsLeverageBatchResponse>;

export interface ApplyReferralParams {
  [field: string]: unknown;
  "body": PerpsAccountReferralRequest;
}

export interface ApplyReferralResponse {
  [field: string]: unknown;
  "status": string;
}

export interface ApproveBuilderParams {
  [field: string]: unknown;
  "body": PerpsBuilderApprovalRequest;
}

export interface ApproveBuilderResponse {
  [field: string]: unknown;
  "trader": string;
  "builder": string;
  "max_fee_rate": string;
  "approval_version": number | bigint | string;
  "timestamp": number | bigint | string;
  "sequence": number | bigint | string;
}

export interface CancelAllOrdersParams {
  [field: string]: unknown;
  "body"?: PerpsCancelAllRequest | null;
}

export interface CancelAllOrdersResponse {
  [field: string]: unknown;
  "status": string;
}

export interface CancelChaseParams {
  [field: string]: unknown;
  "body": CancelChaseParamsBody;
}

export interface CancelChaseParamsBody {
  [field: string]: unknown;
  "op": CancelChaseParamsBodyOp;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "exp"?: number | bigint | string | null;
}

export interface CancelChaseParamsBodyOp {
  [field: string]: unknown;
  "type": string;
  "args": CancelChaseParamsBodyOpArgs;
}

export interface CancelChaseParamsBodyOpArgs {
  [field: string]: unknown;
  "chid": number | bigint | string;
}

export interface CancelOrdersByCoidParams {
  [field: string]: unknown;
  "body"?: PerpsCancelByCoidRequest | null;
}

export interface CancelOrdersParams {
  [field: string]: unknown;
  "body"?: PerpsCancelRequest | null;
}

export interface CancelTwapParams {
  [field: string]: unknown;
  "body": CancelTwapParamsBody;
}

export interface CancelTwapParamsBody {
  [field: string]: unknown;
  "op": CancelTwapParamsBodyOp;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "exp"?: number | bigint | string | null;
}

export interface CancelTwapParamsBodyOp {
  [field: string]: unknown;
  "type": string;
  "args": CancelTwapParamsBodyOpArgs;
}

export interface CancelTwapParamsBodyOpArgs {
  [field: string]: unknown;
  "twid": number | bigint | string;
}

export interface CheckInviteCodeParams {
  [field: string]: unknown;
  "code": string;
  "address"?: string | null;
}

export interface CheckInviteCodeResponse {
  [field: string]: unknown;
  "valid": boolean;
  "error"?: string | null;
}

export interface ControlTwapParams {
  [field: string]: unknown;
  "body": ControlTwapParamsBody;
}

export interface ControlTwapParamsBody {
  [field: string]: unknown;
  "op": ControlTwapParamsBodyOp;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "exp"?: number | bigint | string | null;
}

export interface ControlTwapParamsBodyOp {
  [field: string]: unknown;
  "type": string;
  "args": ControlTwapParamsBodyOpArgs;
}

export interface ControlTwapParamsBodyOpArgs {
  [field: string]: unknown;
  "twid": number | bigint | string;
  "act": string;
}

export interface CreateAccountInviteParams {
  [field: string]: unknown;
  "body"?: PerpsCreateInviteRequest | null;
}

export interface CreateAccountInviteResponse {
  [field: string]: unknown;
  "status": string;
  "code": string;
  "referrals_available": number | bigint | string;
  "cooldown_ms": number | bigint | string | null;
}

export interface CreateChaseParams {
  [field: string]: unknown;
  "body": CreateChaseParamsBody;
}

export interface CreateChaseParamsBody {
  [field: string]: unknown;
  "op": CreateChaseParamsBodyOp;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "exp"?: number | bigint | string | null;
}

export interface CreateChaseParamsBodyOp {
  [field: string]: unknown;
  "type": string;
  "args": CreateChaseParamsBodyOpArgs;
}

export interface CreateChaseParamsBodyOpArgs {
  [field: string]: unknown;
  "iid": number | bigint | string;
  "buy": boolean;
  "qty": string;
  "lim"?: string | null;
  "max_dist"?: string | null;
  "max_dist_bps"?: number | bigint | string | null;
  "po"?: boolean | null;
  "ro"?: boolean | null;
  "c"?: string | null;
}

export interface CreateOrdersParams {
  [field: string]: unknown;
  "body"?: PerpsOrderRequest | null;
}

export interface CreateProxyParams {
  [field: string]: unknown;
  "body"?: PerpsProxyRequest | null;
}

export interface CreateProxyResponse {
  [field: string]: unknown;
  "status": string;
  "secret": string;
}

export interface CreateTwapParams {
  [field: string]: unknown;
  "body": CreateTwapParamsBody;
}

export interface CreateTwapParamsBody {
  [field: string]: unknown;
  "op": CreateTwapParamsBodyOp;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "exp"?: number | bigint | string | null;
}

export interface CreateTwapParamsBodyOp {
  [field: string]: unknown;
  "type": string;
  "args": CreateTwapParamsBodyOpArgs;
}

export interface CreateTwapParamsBodyOpArgs {
  [field: string]: unknown;
  "iid": number | bigint | string;
  "buy": boolean;
  "qty": string;
  "dur"?: number | bigint | string | null;
  "ivl"?: number | bigint | string | null;
  "rnd"?: boolean | null;
  "slip_bps"?: number | bigint | string | null;
  "min_px"?: string | null;
  "max_px"?: string | null;
  "ro"?: boolean | null;
  "c"?: string | null;
}

export interface DeleteProxyParams {
  [field: string]: unknown;
  "body"?: PerpsDeleteProxyRequest | null;
}

export interface DeleteProxyResponse {
  [field: string]: unknown;
  "status": string;
}

export interface GetAccountBackstopsParams {
  [field: string]: unknown;
  "start_timestamp"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetAccountBackstopsResponse {
  [field: string]: unknown;
  "data": Array<PerpsAccountBackstop>;
  "more": boolean;
  "coverage"?: GetAccountBackstopsResponseCoverage | null;
}

export interface GetAccountBackstopsResponseCoverage {
  [field: string]: unknown;
  "notes": Array<string>;
}

export interface GetAccountFundingParams {
  [field: string]: unknown;
  "instrument_id"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
}

export interface GetAccountFundingResponse {
  [field: string]: unknown;
  "data": Array<PerpsAccountFundingData>;
  "more": boolean;
}

export interface GetAccountLimitsParams {
  [field: string]: unknown;
}

export interface GetAccountLimitsResponse {
  [field: string]: unknown;
  "actions_per_minute": number | bigint | string;
  "actions_per_minute_limit": number | bigint | string;
  "actions_burst": number | bigint | string;
  "actions_burst_limit": number | bigint | string;
  "open_orders": number | bigint | string;
  "open_orders_limit": number | bigint | string;
  "reset": number | bigint | string;
  "messages_per_minute": number | bigint | string;
}

export interface GetAccountNotificationsParams {
  [field: string]: unknown;
  "cursor"?: string | null;
  "since_seq"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetAccountNotificationsResponse {
  [field: string]: unknown;
  "items": Array<PerpsNotificationItem>;
  "unread": number | bigint | string;
  "durable_source_seq": number | bigint | string;
  "has_more": boolean;
  "next_cursor": string | null;
}

export interface GetAccountReferralParams {
  [field: string]: unknown;
}

export interface GetAccountReferralResponse {
  [field: string]: unknown;
  "code": string;
  "parent"?: string | null;
  "children_count": number | bigint | string;
  "active_children_count": number | bigint | string;
  "children_volume": string;
  "children_fees": string;
  "referrals_left": number | bigint | string;
  "fee_share_rate": string;
}

export interface GetAccountRewardsParams {
  [field: string]: unknown;
  "date"?: string | null;
  "start_date"?: string | null;
  "end_date"?: string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetAccountRewardsResponse {
  [field: string]: unknown;
  "data": Array<PerpsAccountReward>;
  "more": boolean;
}

export interface GetAccountStatsParams {
  [field: string]: unknown;
}

export interface GetAccountStatsResponse {
  [field: string]: unknown;
  "volume_7d": string;
  "taker_volume_7d": string;
  "maker_volume_7d": string;
  "account_maker_share_7d": string;
  "entity_maker_share_7d"?: string | null;
  "entity_id"?: number | bigint | string | null;
  "entity_name"?: string | null;
}

export interface GetAssetsParams {
  [field: string]: unknown;
}

export interface GetAutoCancelParams {
  [field: string]: unknown;
}

export interface GetAutoCancelResponse {
  [field: string]: unknown;
  "deadline": number | bigint | string;
  "triggered": number | bigint | string;
  "daily_limit": number | bigint | string;
  "next_reset": number | bigint | string;
}

export interface GetBalancesParams {
  [field: string]: unknown;
}

export interface GetBboParams {
  [field: string]: unknown;
  "instrument_id"?: number | bigint | string | null;
}

export interface GetBlpEnrollmentParams {
  [field: string]: unknown;
}

export interface GetBlpEnrollmentResponse {
  [field: string]: unknown;
  "blp": boolean;
  "instruments": Array<PerpsBlpInstrument>;
}

export interface GetBlpLiquidationsParams {
  [field: string]: unknown;
  "start_timestamp"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
  "cursor"?: number | bigint | string | null;
  "sort"?: string | null;
}

export interface GetBlpLiquidationsResponse {
  [field: string]: unknown;
  "data": Array<PerpsBlpLiquidation>;
  "more": boolean;
}

export interface GetBookParams {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "depth"?: number | bigint | string | null;
}

export interface GetBookResponse {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "bids": Array<PerpsLevel>;
  "asks": Array<PerpsLevel>;
  "timestamp": number | bigint | string;
  "sequence": number | bigint | string;
}

export interface GetBuilderApprovalsParams {
  [field: string]: unknown;
  "builder"?: string | null;
}

export interface GetBuilderApprovalsResponse {
  [field: string]: unknown;
  "data": Array<PerpsBuilderApproval>;
}

export interface GetBuilderEarningsParams {
  [field: string]: unknown;
  "start_timestamp"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
  "as_of_sequence"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetBuilderEarningsResponse {
  [field: string]: unknown;
  "data": Array<PerpsBuilderEarning>;
  "more": boolean;
  "cursor"?: string | null;
  "start_timestamp": number | bigint | string;
  "end_timestamp": number | bigint | string;
  "as_of_sequence": number | bigint | string;
}

export interface GetBuilderEarningsSummaryParams {
  [field: string]: unknown;
  "start_timestamp"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
  "as_of_sequence"?: number | bigint | string | null;
}

export interface GetBuilderEarningsSummaryResponse {
  [field: string]: unknown;
  "data": Array<PerpsBuilderEarningsAsset>;
  "start_timestamp": number | bigint | string;
  "end_timestamp": number | bigint | string;
  "as_of_sequence": number | bigint | string;
  "trader_count": number | bigint | string;
  "active_approval_count": number | bigint | string;
}

export interface GetBuilderParams {
  [field: string]: unknown;
  "address": string;
}

export interface GetBuilderResponse {
  [field: string]: unknown;
  "address": string;
  "registered": boolean;
  "enabled": boolean;
  "admission_enabled": boolean;
  "max_fee_rate": string;
  "name": string | null;
}

export interface GetChasesParams {
  [field: string]: unknown;
}

export interface GetCompetitionAccountParams {
  [field: string]: unknown;
  "competition_id": number | bigint | string;
  "address": string;
}

export interface GetCompetitionAccountResponse {
  [field: string]: unknown;
  "competition_id": number | bigint | string;
  "status": string;
  "updated_at"?: number | bigint | string | null;
  "account": string;
  "rank"?: number | bigint | string | null;
  "qualified": boolean;
  "score": string;
  "pnl": string;
  "realized_pnl": string;
  "unrealized_pnl": string;
  "fees": string;
  "funding": string;
  "volume": string;
  "boosted_volume": string;
  "first_fill_at"?: number | bigint | string | null;
  "min_volume": string;
  "volume_to_qualify": string;
  "qualification_progress": string;
  "prize"?: string | null;
  "spots_from_prize_zone"?: number | bigint | string | null;
  "score_to_prize_zone"?: string | null;
  "row_state": string;
  "payout_tx_hash"?: string | null;
  "paid_at"?: number | bigint | string | null;
  "locked_positions": Array<PerpsCompetitionLockedPosition>;
}

export interface GetCompetitionLeaderboardParams {
  [field: string]: unknown;
  "competition_id": number | bigint | string;
  "limit"?: number | bigint | string | null;
  "offset"?: number | bigint | string | null;
}

export interface GetCompetitionLeaderboardResponse {
  [field: string]: unknown;
  "competition_id": number | bigint | string;
  "status": string;
  "updated_at"?: number | bigint | string | null;
  "total": number | bigint | string;
  "offset": number | bigint | string;
  "final_results": boolean;
  "entries": Array<PerpsCompetitionEntry>;
}

export interface GetCompetitionParams {
  [field: string]: unknown;
  "competition_id": number | bigint | string;
}

export interface GetCompetitionResponse {
  [field: string]: unknown;
  "competition_id": number | bigint | string;
  "number": number | bigint | string;
  "name": string;
  "sector_label": string;
  "boost_description": string;
  "status": string;
  "start_time": number | bigint | string;
  "end_time": number | bigint | string;
  "all_markets": boolean;
  "markets": Array<PerpsCompetitionMarket>;
  "min_volume": string;
  "count_unrealized": boolean;
  "prize_pool": string;
  "prize_asset_id": number | bigint | string;
  "prize_tiers": Array<PerpsCompetitionPrizeTier>;
  "paid_places": number | bigint | string;
  "internal": boolean;
  "start_trading_instrument_id"?: number | bigint | string | null;
  "updated_at"?: number | bigint | string | null;
  "ranked": number | bigint | string;
  "participants": number | bigint | string;
  "prize_cutoff_score"?: string | null;
  "final_results": boolean;
  "scoring_version": number | bigint | string;
}

export interface GetCompetitionWidgetParams {
  [field: string]: unknown;
  "address"?: string | null;
}

export interface GetCompetitionWidgetResponse {
  [field: string]: unknown;
  "timestamp": number | bigint | string;
  "competition_id"?: number | bigint | string | null;
  "number"?: number | bigint | string | null;
  "end_time"?: number | bigint | string | null;
  "rank"?: number | bigint | string | null;
  "qualified"?: boolean | null;
  "volume"?: string | null;
  "min_volume"?: string | null;
  "qualification_progress"?: string | null;
  "spots_from_prize_zone"?: number | bigint | string | null;
  "updated_at"?: number | bigint | string | null;
}

export interface GetCompetitionsParams {
  [field: string]: unknown;
  "address"?: string | null;
}

export interface GetCompetitionsResponse {
  [field: string]: unknown;
  "timestamp": number | bigint | string;
  "live_competition_id"?: number | bigint | string | null;
  "landing_competition_id"?: number | bigint | string | null;
  "competitions": Array<PerpsCompetitionSummary>;
}

export interface GetConfigParams {
  [field: string]: unknown;
  "instrument_id"?: number | bigint | string | null;
}

export interface GetCredentialsParams {
  [field: string]: unknown;
}

export interface GetCredentialsResponse {
  [field: string]: unknown;
  "address": string;
  "keys": Array<PerpsKey>;
}

export interface GetDepositsParams {
  [field: string]: unknown;
  "deposit_status"?: string | null;
  "hash"?: string | null;
  "start_timestamp"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
}

export interface GetDepositsResponse {
  [field: string]: unknown;
  "data": Array<PerpsDeposit>;
  "more": boolean;
}

export interface GetEquityHistoryParams {
  [field: string]: unknown;
  "interval": string;
  "start_timestamp": number | bigint | string;
  "end_timestamp"?: number | bigint | string | null;
}

export interface GetEquityHistoryResponse {
  [field: string]: unknown;
  "data": Array<Array<HttpValue>>;
  "more": boolean;
}

export interface GetExchangeParams {
  [field: string]: unknown;
}

export interface GetExchangeResponse {
  [field: string]: unknown;
  "name": string;
  "version": string;
  "chain_id": number | bigint | string;
  "contract": string;
  "cancel_only": boolean;
  "engine_version": string;
}

export interface GetExchangeStatisticsParams {
  [field: string]: unknown;
  "start_timestamp": number | bigint | string;
  "end_timestamp": number | bigint | string;
}

export interface GetExchangeStatisticsResponse {
  [field: string]: unknown;
  "start_timestamp": number | bigint | string;
  "end_timestamp": number | bigint | string;
  "volume": string;
  "open_interest": string | null;
  "open_interest_timestamp": number | bigint | string | null;
  "fees": string;
}

export interface GetFillsParams {
  [field: string]: unknown;
  "instrument_id"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
  "cursor"?: string | null;
  "sort"?: string | null;
}

export interface GetFillsResponse {
  [field: string]: unknown;
  "data": Array<PerpsAccountTradeData>;
  "more": boolean;
  "cursor"?: string | null;
}

export interface GetFundingHistoryParams {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "start_timestamp"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
}

export interface GetFundingHistoryResponse {
  [field: string]: unknown;
  "data": Array<PerpsFundingRate>;
  "more": boolean;
}

export interface GetIndexParams {
  [field: string]: unknown;
  "asset": string;
}

export interface GetIndexResponse {
  [field: string]: unknown;
  "asset": string;
  "index_price": string;
  "constituents": Array<PerpsIndexConstituent>;
  "ts": number | bigint | string;
}

export interface GetInfoFeesParams {
  [field: string]: unknown;
}

export interface GetInfoFeesResponse {
  [field: string]: unknown;
  "fee_schedule": Array<PerpsFeeScheduleEntry>;
}

export interface GetInstrumentsParams {
  [field: string]: unknown;
  "instrument_id"?: number | bigint | string | null;
  "instrument_type"?: string | null;
  "category"?: string | null;
}

export interface GetInternalTransfersParams {
  [field: string]: unknown;
  "direction"?: string | null;
  "start_timestamp"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
}

export interface GetInternalTransfersResponse {
  [field: string]: unknown;
  "data": Array<PerpsInternalTransfer>;
  "more": boolean;
}

export interface GetKlinesParams {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "interval": string;
  "start_timestamp": number | bigint | string;
  "end_timestamp"?: number | bigint | string | null;
}

export interface GetKlinesResponse {
  [field: string]: unknown;
  "data": Array<PerpsKline>;
  "more": boolean;
}

export interface GetLeaderboardParams {
  [field: string]: unknown;
  "window"?: string | null;
  "sort_by"?: string | null;
  "limit"?: number | bigint | string | null;
  "offset"?: number | bigint | string | null;
  "address"?: string | null;
}

export interface GetLeaderboardResponse {
  [field: string]: unknown;
  "window": string;
  "sort_by": string;
  "timestamp": number | bigint | string;
  "total": number | bigint | string;
  "entries": Array<PerpsLeaderboardEntry>;
  "account"?: PerpsLeaderboardAccount | null;
}

export interface GetLimitTiersParams {
  [field: string]: unknown;
}

export interface GetMarkHistoryParams {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "interval": string;
  "start_timestamp": number | bigint | string;
  "end_timestamp"?: number | bigint | string | null;
}

export interface GetMarkHistoryResponse {
  [field: string]: unknown;
  "data": Array<PerpsMarkPoint>;
  "more": boolean;
}

export interface GetOpenOrdersParams {
  [field: string]: unknown;
  "instrument_id"?: number | bigint | string | null;
}

export interface GetOrdersParams {
  [field: string]: unknown;
  "order_id"?: number | bigint | string | null;
  "client_order_id"?: string | null;
  "instrument_id"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
}

export interface GetPingParams {
  [field: string]: unknown;
}

export interface GetPingResponse {
  [field: string]: unknown;
  "status": string;
}

export interface GetPnlHistoryParams {
  [field: string]: unknown;
  "interval": string;
  "start_timestamp": number | bigint | string;
  "end_timestamp"?: number | bigint | string | null;
}

export interface GetPnlHistoryResponse {
  [field: string]: unknown;
  "data": Array<Array<HttpValue>>;
  "more": boolean;
}

export interface GetPortfolioParams {
  [field: string]: unknown;
}

export interface GetPortfolioResponse {
  [field: string]: unknown;
  "positions": Array<PerpsPortfolioPosition>;
  "margin": PerpsMarginSummary;
  "withdrawable": string;
  "in_liquidation": boolean;
  "fee_tier": number | bigint | string;
  "timestamp": number | bigint | string;
}

export interface GetPublicPortfolioParams {
  [field: string]: unknown;
  "address": string;
}

export interface GetPublicPortfolioResponse {
  [field: string]: unknown;
  "positions": Array<PerpsPublicPortfolioPosition>;
  "equity": string;
  "timestamp": number | bigint | string;
}

export interface GetPublicPositionFillsParams {
  [field: string]: unknown;
  "address": string;
  "instrument_id": number | bigint | string;
  "cursor"?: string | null;
  "sort"?: string | null;
}

export interface GetPublicPositionFillsResponse {
  [field: string]: unknown;
  "data": Array<PerpsAccountTradeData>;
  "more": boolean;
  "cursor"?: string | null;
}

export interface GetPublicPositionSnapshotsParams {
  [field: string]: unknown;
  "body": PerpsPositionSnapshotsRequest;
}

export interface GetPublicPositionSnapshotsResponse {
  [field: string]: unknown;
  "history_as_of_at": number | bigint | string;
  "active": Array<PerpsPositionSnapshotResult>;
  "history": Array<PerpsPositionSnapshotResult>;
}

export interface GetRegisteredParams {
  [field: string]: unknown;
  "address": string;
}

export interface GetRegisteredResponse {
  [field: string]: unknown;
  "registered": boolean;
}

export interface GetStatisticsParams {
  [field: string]: unknown;
  "instrument_id"?: number | bigint | string | null;
}

export interface GetTickersParams {
  [field: string]: unknown;
  "instrument_id"?: number | bigint | string | null;
}

export interface GetTimeParams {
  [field: string]: unknown;
}

export interface GetTimeResponse {
  [field: string]: unknown;
  "time": number | bigint | string;
}

export interface GetTradesParams {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "start_timestamp"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
}

export interface GetTradesResponse {
  [field: string]: unknown;
  "data": Array<PerpsTradeData>;
  "more": boolean;
}

export interface GetTwapsParams {
  [field: string]: unknown;
}

export interface GetWithdrawalsParams {
  [field: string]: unknown;
  "withdrawal_status"?: string | null;
  "hash"?: string | null;
  "start_timestamp"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
}

export interface GetWithdrawalsResponse {
  [field: string]: unknown;
  "data": Array<PerpsWithdrawal>;
  "more": boolean;
}

export interface InternalTransferParams {
  [field: string]: unknown;
  "body"?: PerpsInternalTransferRequest | null;
}

export interface InternalTransferResponse {
  [field: string]: unknown;
  "status": string;
  "transfer_id": number | bigint | string;
}

export interface MarkAccountNotificationsReadParams {
  [field: string]: unknown;
  "body": PerpsNotificationsReadRequest;
}

export interface MarkAccountNotificationsReadResponse {
  [field: string]: unknown;
  "status": string;
}

export interface ModifyOrdersByCoidParams {
  [field: string]: unknown;
  "body"?: PerpsModifyByCoidRequest | null;
}

export interface ModifyOrdersParams {
  [field: string]: unknown;
  "body"?: PerpsModifyRequest | null;
}

export interface PerpsAccountBackstop {
  [field: string]: unknown;
  "id": number | bigint | string;
  "cross": boolean;
  "instrument_id": number | bigint | string | null;
  "asset": string;
  "transferred": string;
  "cash_before": string;
  "cash_after": string;
  "timestamp": number | bigint | string;
  "sequence": number | bigint | string;
}

export interface PerpsAccountBackstops {
  [field: string]: unknown;
  "data": Array<PerpsAccountBackstop>;
  "more": boolean;
  "coverage"?: PerpsAccountBackstopsCoverage | null;
}

export interface PerpsAccountBackstopsCoverage {
  [field: string]: unknown;
  "notes": Array<string>;
}

export interface PerpsAccountConfig {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "leverage": number | bigint | string;
  "cross": boolean;
}

export interface PerpsAccountFundingData {
  [field: string]: unknown;
  "id": number | bigint | string;
  "instrument_id": number | bigint | string;
  "size": string;
  "funding_rate": string;
  "funding_asset": string;
  "funding": string;
  "timestamp": number | bigint | string;
}

export interface PerpsAccountFundingHistory {
  [field: string]: unknown;
  "data": Array<PerpsAccountFundingData>;
  "more": boolean;
}

export interface PerpsAccountLimits {
  [field: string]: unknown;
  "actions_per_minute": number | bigint | string;
  "actions_per_minute_limit": number | bigint | string;
  "actions_burst": number | bigint | string;
  "actions_burst_limit": number | bigint | string;
  "open_orders": number | bigint | string;
  "open_orders_limit": number | bigint | string;
  "reset": number | bigint | string;
  "messages_per_minute": number | bigint | string;
}

export interface PerpsAccountReferral {
  [field: string]: unknown;
  "code": string;
  "parent"?: string | null;
  "children_count": number | bigint | string;
  "active_children_count": number | bigint | string;
  "children_volume": string;
  "children_fees": string;
  "referrals_left": number | bigint | string;
  "fee_share_rate": string;
}

export interface PerpsAccountReferralRequest {
  [field: string]: unknown;
  "code": string;
}

export interface PerpsAccountReward {
  [field: string]: unknown;
  "date": string;
  "maker_share_7d": string;
  "reward_distributed": boolean;
  "breakdown": Array<PerpsAccountRewardInstrument>;
  "oi_rewards"?: PerpsAccountRewardOi | null;
}

export interface PerpsAccountRewardInstrument {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "reward_pool": string;
  "reward_amount": string;
}

export interface PerpsAccountRewardOi {
  [field: string]: unknown;
  "account_oi": string;
  "reward_amount": string;
}

export interface PerpsAccountRewards {
  [field: string]: unknown;
  "data": Array<PerpsAccountReward>;
  "more": boolean;
}

export interface PerpsAccountStats {
  [field: string]: unknown;
  "volume_7d": string;
  "taker_volume_7d": string;
  "maker_volume_7d": string;
  "account_maker_share_7d": string;
  "entity_maker_share_7d"?: string | null;
  "entity_id"?: number | bigint | string | null;
  "entity_name"?: string | null;
}

export interface PerpsAccountTradeData {
  [field: string]: unknown;
  "trade_id": number | bigint | string;
  "order_id": number | bigint | string;
  "instrument_id": number | bigint | string;
  "side": string;
  "price": string;
  "quantity": string;
  "taker": boolean;
  "fee": string;
  "fee_asset": string;
  "previous_size": string;
  "previous_entry_price": string;
  "pnl": string;
  "liquidation": boolean;
  "adl": boolean;
  "timestamp": number | bigint | string;
  "hash": string;
  "builder_fee": string;
  "total_fee": string;
  "builder"?: PerpsBuilder | null;
  "settlement": boolean;
}

export interface PerpsAccountTrades {
  [field: string]: unknown;
  "data": Array<PerpsAccountTradeData>;
  "more": boolean;
  "cursor"?: string | null;
}

export interface PerpsAsset {
  [field: string]: unknown;
  "asset": string;
  "address": string;
  "decimals": number | bigint | string;
  "collateral_ratio": string;
  "withdrawal_fee": string;
}

export interface PerpsAutoCancelRequest {
  [field: string]: unknown;
  "op": PerpsOpAutoCancel;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
}

export interface PerpsAutoCancelResponse {
  [field: string]: unknown;
  "status": string;
  "deadline": number | bigint | string;
}

export interface PerpsAutoCancelStatus {
  [field: string]: unknown;
  "deadline": number | bigint | string;
  "triggered": number | bigint | string;
  "daily_limit": number | bigint | string;
  "next_reset": number | bigint | string;
}

export interface PerpsBBO {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "bid_price": string;
  "bid_quantity": string;
  "ask_price": string;
  "ask_quantity": string;
  "timestamp": number | bigint | string;
}

export interface PerpsBalance {
  [field: string]: unknown;
  "asset": string;
  "balance": string;
  "value": string;
}

export interface PerpsBaseOp {
  [field: string]: unknown;
  "sig": string;
  "salt": number | bigint | string;
  "ts": number | bigint | string;
}

export interface PerpsBlpEnrollRequest {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "active": boolean;
}

export interface PerpsBlpEnrollment {
  [field: string]: unknown;
  "blp": boolean;
  "instruments": Array<PerpsBlpInstrument>;
}

export interface PerpsBlpInstrument {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "symbol": string;
}

export interface PerpsBlpLiquidation {
  [field: string]: unknown;
  "liquidation_id": number | bigint | string;
  "timestamp": number | bigint | string;
  "cross": boolean;
  "margin": string;
  "trades": Array<PerpsBlpLiquidationTrade>;
}

export interface PerpsBlpLiquidationTrade {
  [field: string]: unknown;
  "trade_id": number | bigint | string;
  "instrument_id": number | bigint | string;
  "side": string;
  "quantity": string;
  "price": string;
}

export interface PerpsBlpLiquidations {
  [field: string]: unknown;
  "data": Array<PerpsBlpLiquidation>;
  "more": boolean;
}

export interface PerpsBook {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "bids": Array<PerpsLevel>;
  "asks": Array<PerpsLevel>;
  "timestamp": number | bigint | string;
  "sequence": number | bigint | string;
}

export interface PerpsBuilder {
  [field: string]: unknown;
  "address": string;
  "fee_rate": string;
}

export interface PerpsBuilderApproval {
  [field: string]: unknown;
  "trader": string;
  "builder": string;
  "max_fee_rate": string;
  "approval_version": number | bigint | string;
  "timestamp": number | bigint | string;
  "sequence": number | bigint | string;
}

export interface PerpsBuilderApprovalRequest {
  [field: string]: unknown;
  "op": PerpsOpApproveBuilder;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
}

export interface PerpsBuilderApprovals {
  [field: string]: unknown;
  "data": Array<PerpsBuilderApproval>;
}

export interface PerpsBuilderEarning {
  [field: string]: unknown;
  "earning_id": string;
  "trade_id": number | bigint | string;
  "order_id": number | bigint | string;
  "instrument_id": number | bigint | string;
  "side": string;
  "timestamp": number | bigint | string;
  "sequence": number | bigint | string;
  "notional": string;
  "fee_asset": string;
  "builder_fee": string;
  "fee_rate": string;
  "trader": string;
  "buy": boolean;
  "price": string;
  "quantity": string;
  "client_order_id"?: string | null;
  "fee": string;
  "total_fee": string;
}

export interface PerpsBuilderEarnings {
  [field: string]: unknown;
  "data": Array<PerpsBuilderEarning>;
  "more": boolean;
  "cursor"?: string | null;
  "start_timestamp": number | bigint | string;
  "end_timestamp": number | bigint | string;
  "as_of_sequence": number | bigint | string;
}

export interface PerpsBuilderEarningsAsset {
  [field: string]: unknown;
  "fee_asset": string;
  "fill_count": number | bigint | string;
  "notional": string;
  "builder_fee": string;
}

export interface PerpsBuilderEarningsSummary {
  [field: string]: unknown;
  "data": Array<PerpsBuilderEarningsAsset>;
  "start_timestamp": number | bigint | string;
  "end_timestamp": number | bigint | string;
  "as_of_sequence": number | bigint | string;
  "trader_count": number | bigint | string;
  "active_approval_count": number | bigint | string;
}

export interface PerpsBuilderStatus {
  [field: string]: unknown;
  "address": string;
  "registered": boolean;
  "enabled": boolean;
  "admission_enabled": boolean;
  "max_fee_rate": string;
}

export interface PerpsCancelAccepted {
  [field: string]: unknown;
  "status": string;
  "oid": number | bigint | string;
  "coid"?: string | null;
  "ts": number | bigint | string;
}

export interface PerpsCancelAllRequest {
  [field: string]: unknown;
  "op": PerpsOpCancelAll;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "exp"?: number | bigint | string | null;
}

export interface PerpsCancelByCoidRequest {
  [field: string]: unknown;
  "op": PerpsOpCancelOrdersByCoid;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "exp"?: number | bigint | string | null;
}

export interface PerpsCancelRejected {
  [field: string]: unknown;
  "status": string;
  "oid"?: number | bigint | string | null;
  "coid"?: string | null;
  "error": string;
  "ts": number | bigint | string;
}

export interface PerpsCancelRequest {
  [field: string]: unknown;
  "op": PerpsOpCancelOrders;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "exp"?: number | bigint | string | null;
}

export interface PerpsCompetitionAccount {
  [field: string]: unknown;
  "competition_id": number | bigint | string;
  "status": string;
  "updated_at"?: number | bigint | string | null;
  "account": string;
  "rank"?: number | bigint | string | null;
  "qualified": boolean;
  "score": string;
  "pnl": string;
  "realized_pnl": string;
  "unrealized_pnl": string;
  "fees": string;
  "funding": string;
  "volume": string;
  "boosted_volume": string;
  "first_fill_at"?: number | bigint | string | null;
  "min_volume": string;
  "volume_to_qualify": string;
  "qualification_progress": string;
  "prize"?: string | null;
  "spots_from_prize_zone"?: number | bigint | string | null;
  "score_to_prize_zone"?: string | null;
  "row_state": string;
  "payout_tx_hash"?: string | null;
  "paid_at"?: number | bigint | string | null;
  "locked_positions": Array<PerpsCompetitionLockedPosition>;
}

export interface PerpsCompetitionDetail {
  [field: string]: unknown;
  "competition_id": number | bigint | string;
  "number": number | bigint | string;
  "name": string;
  "sector_label": string;
  "boost_description": string;
  "status": string;
  "start_time": number | bigint | string;
  "end_time": number | bigint | string;
  "all_markets": boolean;
  "markets": Array<PerpsCompetitionMarket>;
  "min_volume": string;
  "count_unrealized": boolean;
  "prize_pool": string;
  "prize_asset_id": number | bigint | string;
  "prize_tiers": Array<PerpsCompetitionPrizeTier>;
  "paid_places": number | bigint | string;
  "internal": boolean;
  "start_trading_instrument_id"?: number | bigint | string | null;
  "updated_at"?: number | bigint | string | null;
  "ranked": number | bigint | string;
  "participants": number | bigint | string;
  "prize_cutoff_score"?: string | null;
  "final_results": boolean;
  "scoring_version": number | bigint | string;
}

export interface PerpsCompetitionEntry {
  [field: string]: unknown;
  "rank"?: number | bigint | string | null;
  "account": string;
  "score": string;
  "pnl": string;
  "realized_pnl": string;
  "unrealized_pnl": string;
  "volume": string;
  "boosted_volume": string;
  "first_fill_at"?: number | bigint | string | null;
  "prize"?: string | null;
  "row_state": string;
  "payout_tx_hash"?: string | null;
  "paid_at"?: number | bigint | string | null;
}

export interface PerpsCompetitionLeaderboard {
  [field: string]: unknown;
  "competition_id": number | bigint | string;
  "status": string;
  "updated_at"?: number | bigint | string | null;
  "total": number | bigint | string;
  "offset": number | bigint | string;
  "final_results": boolean;
  "entries": Array<PerpsCompetitionEntry>;
}

export interface PerpsCompetitionList {
  [field: string]: unknown;
  "timestamp": number | bigint | string;
  "live_competition_id"?: number | bigint | string | null;
  "landing_competition_id"?: number | bigint | string | null;
  "competitions": Array<PerpsCompetitionSummary>;
}

export interface PerpsCompetitionLockedPosition {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "size_at_start": string;
  "released_at"?: number | bigint | string | null;
}

export interface PerpsCompetitionMarket {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "boost": string;
}

export interface PerpsCompetitionPrizeTier {
  [field: string]: unknown;
  "from_rank": number | bigint | string;
  "to_rank": number | bigint | string;
  "amount": string;
}

export interface PerpsCompetitionReadError400 {
  [field: string]: unknown;
  "status": string;
  "error": string;
  "arts"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "ref"?: string | null;
}

export interface PerpsCompetitionReadError404 {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsCompetitionReadError429 {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsCompetitionReadError500 {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsCompetitionReadError503 {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsCompetitionSummary {
  [field: string]: unknown;
  "competition_id": number | bigint | string;
  "number": number | bigint | string;
  "name": string;
  "sector_label": string;
  "status": string;
  "start_time": number | bigint | string;
  "end_time": number | bigint | string;
  "prize_pool": string;
  "prize_asset_id": number | bigint | string;
  "internal": boolean;
}

export interface PerpsCompetitionWidget {
  [field: string]: unknown;
  "timestamp": number | bigint | string;
  "competition_id"?: number | bigint | string | null;
  "number"?: number | bigint | string | null;
  "end_time"?: number | bigint | string | null;
  "rank"?: number | bigint | string | null;
  "qualified"?: boolean | null;
  "volume"?: string | null;
  "min_volume"?: string | null;
  "qualification_progress"?: string | null;
  "spots_from_prize_zone"?: number | bigint | string | null;
  "updated_at"?: number | bigint | string | null;
}

export interface PerpsCreateInviteRequest {
  [field: string]: unknown;
}

export interface PerpsCreateInviteResponse {
  [field: string]: unknown;
  "status": string;
  "code": string;
  "referrals_available": number | bigint | string;
  "cooldown_ms": number | bigint | string | null;
}

export interface PerpsCreateOrder {
  [field: string]: unknown;
  "iid": number | bigint | string;
  "buy": boolean;
  "p"?: string | null;
  "qty": string;
  "tif"?: string | null;
  "po"?: boolean | null;
  "ro"?: boolean | null;
  "c"?: string | null;
  "tr"?: PerpsCreateOrderTr | null;
  "builder"?: PerpsBuilder | null;
}

export interface PerpsCreateOrderTr {
  [field: string]: unknown;
  "market"?: boolean | null;
  "trp"?: string | null;
  "tpsl"?: string | null;
}

export interface PerpsCredentials {
  [field: string]: unknown;
  "address": string;
  "keys": Array<PerpsKey>;
}

export interface PerpsDeleteProxyRequest {
  [field: string]: unknown;
  "op": PerpsOpDeleteProxy;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
}

export interface PerpsDeposit {
  [field: string]: unknown;
  "hash": string;
  "asset": string;
  "amount": string;
  "from": string;
  "to": string;
  "status": string;
  "confirmations": number | bigint | string;
  "required_confirmations": number | bigint | string;
  "created_timestamp": number | bigint | string;
  "confirmed_timestamp"?: number | bigint | string | null;
}

export interface PerpsDeposits {
  [field: string]: unknown;
  "data": Array<PerpsDeposit>;
  "more": boolean;
}

export interface PerpsEquityHistory {
  [field: string]: unknown;
  "data": Array<Array<HttpValue>>;
  "more": boolean;
}

export interface PerpsError400 {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsError401 {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsError404 {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsError408 {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsError413 {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsError429 {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsError500 {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsError503 {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsExchange {
  [field: string]: unknown;
  "name": string;
  "version": string;
  "chain_id": number | bigint | string;
  "contract": string;
  "cancel_only": boolean;
  "engine_version": string;
}

export interface PerpsExchangeStatistics {
  [field: string]: unknown;
  "start_timestamp": number | bigint | string;
  "end_timestamp": number | bigint | string;
  "volume": string;
  "open_interest": string | null;
  "open_interest_timestamp": number | bigint | string | null;
  "fees": string;
}

export interface PerpsFeeScheduleEntry {
  [field: string]: unknown;
  "instrument_type": string;
  "category": string;
  "taker_fee_rate": string;
  "maker_fee_rate": string;
  "tiers": Array<PerpsFeeTier>;
}

export interface PerpsFeeTier {
  [field: string]: unknown;
  "min_volume_30d": string;
  "taker_fee_rate": string;
  "maker_fee_rate": string;
}

export interface PerpsFeesInfo {
  [field: string]: unknown;
  "fee_schedule": Array<PerpsFeeScheduleEntry>;
}

export interface PerpsFundingHistory {
  [field: string]: unknown;
  "data": Array<PerpsFundingRate>;
  "more": boolean;
}

export interface PerpsFundingRate {
  [field: string]: unknown;
  "funding_rate": string;
  "timestamp": number | bigint | string;
}

export interface PerpsGenericAccepted {
  [field: string]: unknown;
  "status": string;
}

export interface PerpsGenericRejected {
  [field: string]: unknown;
  "status": string;
  "error": string;
}

export interface PerpsIndex {
  [field: string]: unknown;
  "asset": string;
  "index_price": string;
  "constituents": Array<PerpsIndexConstituent>;
  "ts": number | bigint | string;
}

export interface PerpsIndexConstituent {
  [field: string]: unknown;
  "source": string;
  "symbol": string;
  "weight": string;
  "price": string;
}

export interface PerpsInstrument {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "instrument_type": string;
  "category": string;
  "isolated_only": boolean;
  "symbol": string;
  "display_symbol"?: string | null;
  "base_asset": string;
  "quote_asset": string;
  "funding_interval": string;
  "quantity_decimals": number | bigint | string;
  "price_decimals": number | bigint | string;
  "price_bounds": string;
  "liquidation_fee": string;
  "max_order_count": number | bigint | string;
  "min_notional": string;
  "max_market_notional": string;
  "max_limit_notional": string;
  "max_leverage": number | bigint | string;
  "risk_tiers": Array<PerpsRiskTier>;
  "ui_live_time": number | bigint | string | null;
  "close_only": boolean;
  "settlement"?: PerpsInstrumentSettlement | null;
}

export interface PerpsInstrumentSettlement {
  [field: string]: unknown;
  "sequence": number | bigint | string;
  "timestamp": number | bigint | string;
  "price": string;
  "insurance_debit": string;
}

export interface PerpsInternalTransfer {
  [field: string]: unknown;
  "transfer_id": number | bigint | string;
  "asset": string;
  "amount": string;
  "direction": string;
  "counterparty": string;
  "label"?: string | null;
  "created_timestamp": number | bigint | string;
}

export interface PerpsInternalTransferAccepted {
  [field: string]: unknown;
  "status": string;
  "transfer_id": number | bigint | string;
}

export interface PerpsInternalTransferRejected {
  [field: string]: unknown;
  "status": string;
  "transfer_id": number | bigint | string;
  "error": string;
}

export interface PerpsInternalTransferRequest {
  [field: string]: unknown;
  "op": PerpsOpInternalTransfer;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "label"?: string | null;
}

export interface PerpsInternalTransfers {
  [field: string]: unknown;
  "data": Array<PerpsInternalTransfer>;
  "more": boolean;
}

export interface PerpsInviteCheckResponse {
  [field: string]: unknown;
  "valid": boolean;
  "error"?: string | null;
}

export interface PerpsKey {
  [field: string]: unknown;
  "proxy": string;
  "label"?: string | null;
  "expiry": number | bigint | string;
}

export interface PerpsKlinesResponse {
  [field: string]: unknown;
  "data": Array<PerpsKline>;
  "more": boolean;
}

export interface PerpsLeaderboard {
  [field: string]: unknown;
  "window": string;
  "sort_by": string;
  "timestamp": number | bigint | string;
  "total": number | bigint | string;
  "entries": Array<PerpsLeaderboardEntry>;
  "account"?: PerpsLeaderboardAccount | null;
}

export interface PerpsLeaderboardAccount {
  [field: string]: unknown;
  "rank"?: number | bigint | string | null;
  "account": string;
  "pnl": string;
  "notional": string;
  "account_value": string;
  "roi": string;
}

export interface PerpsLeaderboardEntry {
  [field: string]: unknown;
  "rank": number | bigint | string;
  "account": string;
  "pnl": string;
  "notional": string;
  "account_value": string;
  "roi": string;
}

export interface PerpsLeverageBatchRequest {
  [field: string]: unknown;
  "op": PerpsOpUpdateLeverages;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
}

export interface PerpsLeverageRejected {
  [field: string]: unknown;
  "status": string;
  "instrument_id": number | bigint | string;
  "error": string;
}

export interface PerpsLeverageRequest {
  [field: string]: unknown;
  "op": PerpsOpUpdateLeverage;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
}

export interface PerpsLeverageResponse {
  [field: string]: unknown;
  "status": string;
  "instrument_id": number | bigint | string;
  "leverage": number | bigint | string;
  "cross": boolean;
}

export interface PerpsLimitTier {
  [field: string]: unknown;
  "min_volume_14d": string;
  "rate_per_minute_limit": number | bigint | string;
  "rate_burst_limit": number | bigint | string;
  "actions_per_minute_limit": number | bigint | string;
  "actions_burst_limit": number | bigint | string;
  "open_orders_limit": number | bigint | string;
  "messages_per_minute": number | bigint | string;
}

export interface PerpsMarginRequest {
  [field: string]: unknown;
  "op": PerpsOpUpdateMargin;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
}

export interface PerpsMarginSummary {
  [field: string]: unknown;
  "total_account_value": string;
  "available_order_margin": string;
  "total_initial_margin": string;
  "total_maintenance_margin": string;
  "total_position_value": string;
}

export interface PerpsMarkHistoryResponse {
  [field: string]: unknown;
  "data": Array<PerpsMarkPoint>;
  "more": boolean;
}

export interface PerpsModifyAccepted {
  [field: string]: unknown;
  "status": string;
  "order": PerpsOrder;
}

export interface PerpsModifyByCoidRequest {
  [field: string]: unknown;
  "op": PerpsOpModifyOrdersByCoid;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "exp"?: number | bigint | string | null;
}

export interface PerpsModifyOrder {
  [field: string]: unknown;
  "oid": number | bigint | string;
  "p": string;
  "qty": string;
}

export interface PerpsModifyOrderByCoid {
  [field: string]: unknown;
  "coid": string;
  "p": string;
  "qty": string;
}

export interface PerpsModifyRejected {
  [field: string]: unknown;
  "status": string;
  "oid"?: number | bigint | string | null;
  "coid"?: string | null;
  "error": string;
}

export interface PerpsModifyRequest {
  [field: string]: unknown;
  "op": PerpsOpModifyOrders;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "exp"?: number | bigint | string | null;
}

export interface PerpsNotificationItem {
  [field: string]: unknown;
  "notification": Record<string, HttpValue>;
  "read_at": number | bigint | string | null;
  "ts": number | bigint | string;
}

export interface PerpsNotificationsResponse {
  [field: string]: unknown;
  "items": Array<PerpsNotificationItem>;
  "unread": number | bigint | string;
  "durable_source_seq": number | bigint | string;
  "has_more": boolean;
  "next_cursor": string | null;
}

export interface PerpsOpApproveBuilder {
  [field: string]: unknown;
  "type": string;
  "args": PerpsOpApproveBuilderArgs;
}

export interface PerpsOpApproveBuilderArgs {
  [field: string]: unknown;
  "builder": string;
  "max_fee_rate": string;
  "approval_version": number | bigint | string;
}

export interface PerpsOpAutoCancel {
  [field: string]: unknown;
  "type": string;
  "args": PerpsOpAutoCancelArgs;
}

export interface PerpsOpAutoCancelArgs {
  [field: string]: unknown;
  "time": number | bigint | string;
}

export interface PerpsOpCancelAll {
  [field: string]: unknown;
  "type": string;
  "args": PerpsOpCancelAllArgs;
}

export interface PerpsOpCancelAllArgs {
  [field: string]: unknown;
  "iid"?: number | bigint | string | null;
}

export interface PerpsOpCancelOrders {
  [field: string]: unknown;
  "type": string;
  "args": Array<number | bigint | string>;
}

export interface PerpsOpCancelOrdersByCoid {
  [field: string]: unknown;
  "type": string;
  "args": Array<string>;
}

export interface PerpsOpCreateOrders {
  [field: string]: unknown;
  "type": string;
  "args": Array<PerpsCreateOrder>;
  "grp"?: string | null;
}

export interface PerpsOpCreateProxy {
  [field: string]: unknown;
  "type": string;
  "args": PerpsOpCreateProxyArgs;
}

export interface PerpsOpCreateProxyArgs {
  [field: string]: unknown;
  "owner": string;
  "proxy": string;
  "expiry": number | bigint | string;
}

export interface PerpsOpDeleteProxy {
  [field: string]: unknown;
  "type": string;
  "args": PerpsOpDeleteProxyArgs;
}

export interface PerpsOpDeleteProxyArgs {
  [field: string]: unknown;
  "proxy": string;
}

export interface PerpsOpInternalTransfer {
  [field: string]: unknown;
  "type": string;
  "args": PerpsOpInternalTransferArgs;
}

export interface PerpsOpInternalTransferArgs {
  [field: string]: unknown;
  "account": string;
  "token": string;
  "amount": string;
  "to": string;
}

export interface PerpsOpModifyOrders {
  [field: string]: unknown;
  "type": string;
  "args": Array<PerpsModifyOrder>;
}

export interface PerpsOpModifyOrdersByCoid {
  [field: string]: unknown;
  "type": string;
  "args": Array<PerpsModifyOrderByCoid>;
}

export interface PerpsOpUpdateLeverage {
  [field: string]: unknown;
  "type": string;
  "args": PerpsOpUpdateLeverageArgs;
}

export interface PerpsOpUpdateLeverageArgs {
  [field: string]: unknown;
  "iid": number | bigint | string;
  "lev": number | bigint | string;
  "cross": boolean;
}

export interface PerpsOpUpdateLeverages {
  [field: string]: unknown;
  "type": string;
  "args": Array<PerpsOpUpdateLeveragesArgsItem>;
}

export interface PerpsOpUpdateLeveragesArgsItem {
  [field: string]: unknown;
  "iid": number | bigint | string;
  "lev": number | bigint | string;
  "cross": boolean;
}

export interface PerpsOpUpdateMargin {
  [field: string]: unknown;
  "type": string;
  "args": PerpsOpUpdateMarginArgs;
}

export interface PerpsOpUpdateMarginArgs {
  [field: string]: unknown;
  "iid": number | bigint | string;
  "amt": string;
}

export interface PerpsOpWithdraw {
  [field: string]: unknown;
  "type": string;
  "args": PerpsOpWithdrawArgs;
}

export interface PerpsOpWithdrawArgs {
  [field: string]: unknown;
  "account": string;
  "token": string;
  "amount": string;
  "to": string;
}

export interface PerpsOrder {
  [field: string]: unknown;
  "oid": number | bigint | string;
  "iid": number | bigint | string;
  "buy": boolean;
  "p": string;
  "qty": string;
  "tif": string;
  "po": boolean;
  "ro": boolean;
  "rest": string;
  "fill": string;
  "cts": number | bigint | string;
  "uts": number | bigint | string;
  "status": string;
  "coid"?: string | null;
  "builder"?: PerpsBuilder | null;
}

export interface PerpsOrderAccepted {
  [field: string]: unknown;
  "status": string;
  "oid": number | bigint | string;
  "coid"?: string | null;
  "builder"?: PerpsBuilder | null;
}

export interface PerpsOrderData {
  [field: string]: unknown;
  "order_id": number | bigint | string;
  "instrument_id": number | bigint | string;
  "buy": boolean;
  "price": string;
  "quantity": string;
  "tif": string;
  "post_only": boolean;
  "ro": boolean;
  "resting_quantity": string;
  "filled_quantity": string;
  "status": string;
  "created_timestamp": number | bigint | string;
  "updated_timestamp": number | bigint | string;
  "client_order_id"?: string | null;
  "tpsl"?: PerpsTpSlOrderFields | null;
  "builder"?: PerpsBuilder | null;
}

export interface PerpsOrderRejected {
  [field: string]: unknown;
  "status": string;
  "error": string;
  "oid"?: number | bigint | string | null;
  "coid"?: string | null;
}

export interface PerpsOrderRequest {
  [field: string]: unknown;
  "op": PerpsOpCreateOrders;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "exp"?: number | bigint | string | null;
}

export interface PerpsPnlHistory {
  [field: string]: unknown;
  "data": Array<Array<HttpValue>>;
  "more": boolean;
}

export interface PerpsPortfolio {
  [field: string]: unknown;
  "positions": Array<PerpsPortfolioPosition>;
  "margin": PerpsMarginSummary;
  "withdrawable": string;
  "in_liquidation": boolean;
  "fee_tier": number | bigint | string;
  "timestamp": number | bigint | string;
}

export interface PerpsPortfolioPosition {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "symbol": string;
  "size": string;
  "entry_price": string;
  "leverage": number | bigint | string;
  "cross": boolean;
  "initial_margin": string;
  "maintenance_margin": string;
  "position_value": string;
  "liquidation_price": string;
  "unrealized_pnl": string;
  "return_on_equity": string;
  "cumulative_funding": string;
}

export interface PerpsPositionCardSnapshot {
  [field: string]: unknown;
  "position_cycle_id": string;
  "side": string;
  "started_at": number | bigint | string;
  "as_of_at": number | bigint | string;
  "is_closed": boolean;
  "size_after": string;
  "entry_price": string;
  "as_of_price": string;
  "pnl": string;
  "pnl_percent": string | null;
  "leverage": number | bigint | string | null;
  "chart": PerpsPositionSnapshotChart;
}

export interface PerpsPositionSnapshotCandle {
  [field: string]: unknown;
  "position": number | bigint | string;
  "open": number | bigint | string;
  "high": number | bigint | string;
  "low": number | bigint | string;
  "close": number | bigint | string;
}

export interface PerpsPositionSnapshotChart {
  [field: string]: unknown;
  "candles": Array<PerpsPositionSnapshotCandle>;
  "markers": Array<PerpsPositionSnapshotMarker>;
}

export interface PerpsPositionSnapshotFill {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "trade_id": string;
  "timestamp": number | bigint | string;
}

export interface PerpsPositionSnapshotMarker {
  [field: string]: unknown;
  "position": number | bigint | string;
  "kind": string;
}

export interface PerpsPositionSnapshotResult {
  [field: string]: unknown;
  "instrument_id"?: number | bigint | string | null;
  "trade_id"?: string | null;
  "status": string;
  "snapshot"?: PerpsPositionCardSnapshot | null;
}

export interface PerpsPositionSnapshots {
  [field: string]: unknown;
  "history_as_of_at": number | bigint | string;
  "active": Array<PerpsPositionSnapshotResult>;
  "history": Array<PerpsPositionSnapshotResult>;
}

export interface PerpsPositionSnapshotsRequest {
  [field: string]: unknown;
  "address": string;
  "active_instrument_ids"?: Array<number | bigint | string> | null;
  "history_fills"?: Array<PerpsPositionSnapshotFill> | null;
}

export interface PerpsProxyRequest {
  [field: string]: unknown;
  "op": PerpsOpCreateProxy;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
  "label"?: string | null;
  "code"?: string | null;
}

export interface PerpsProxyResponse {
  [field: string]: unknown;
  "status": string;
  "secret": string;
}

export interface PerpsPublicPortfolio {
  [field: string]: unknown;
  "positions": Array<PerpsPublicPortfolioPosition>;
  "equity": string;
  "timestamp": number | bigint | string;
}

export interface PerpsPublicPortfolioPosition {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "symbol": string;
  "size": string;
  "entry_price": string;
  "unrealized_pnl": string;
  "return_on_equity": string;
}

export interface PerpsRegisteredResponse {
  [field: string]: unknown;
  "registered": boolean;
}

export interface PerpsRiskTier {
  [field: string]: unknown;
  "lower_bound": string;
  "max_leverage": number | bigint | string;
}

export interface PerpsStatistic {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "symbol": string;
  "volume": string;
  "open_price": string;
  "klines": PerpsKlines;
}

export interface PerpsTicker {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "symbol": string;
  "index_price": string;
  "mark_price": string;
  "last_price": string;
  "mid_price": string;
  "open_interest": string;
  "funding_rate": string;
  "next_funding": number | bigint | string;
  "timestamp": number | bigint | string;
}

export interface PerpsTickerData {
  [field: string]: unknown;
  "instrument_id": number | bigint | string;
  "symbol": string;
  "index_price": string;
  "mark_price": string;
  "last_price": string;
  "mid_price": string;
  "open_interest": string;
  "funding_rate": string;
  "next_funding": number | bigint | string;
}

export interface PerpsTime {
  [field: string]: unknown;
  "time": number | bigint | string;
}

export interface PerpsTpSlOrderFields {
  [field: string]: unknown;
  "kind": string;
  "scope": string;
  "trp": string;
  "parent_oid"?: number | bigint | string | null;
  "armed_qty"?: string | null;
  "slip_bps"?: number | bigint | string | null;
}

export interface PerpsTradeData {
  [field: string]: unknown;
  "trade_id": number | bigint | string;
  "instrument_id": number | bigint | string;
  "side": string;
  "price": string;
  "quantity": string;
  "timestamp": number | bigint | string;
  "hash": string;
  "settlement": boolean;
}

export interface PerpsTrades {
  [field: string]: unknown;
  "data": Array<PerpsTradeData>;
  "more": boolean;
}

export interface PerpsWithdrawAccepted {
  [field: string]: unknown;
  "status": string;
  "withdraw_id": number | bigint | string;
}

export interface PerpsWithdrawRejected {
  [field: string]: unknown;
  "status": string;
  "withdraw_id": number | bigint | string;
  "error": string;
}

export interface PerpsWithdrawRequest {
  [field: string]: unknown;
  "op": PerpsOpWithdraw;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
}

export interface PerpsWithdrawal {
  [field: string]: unknown;
  "withdraw_id": number | bigint | string;
  "asset": string;
  "amount": string;
  "to": string;
  "fee": string;
  "status": string;
  "hash": string;
  "confirmations": number | bigint | string;
  "required_confirmations": number | bigint | string;
  "created_timestamp": number | bigint | string;
  "confirmed_timestamp"?: number | bigint | string | null;
}

export interface PerpsWithdrawals {
  [field: string]: unknown;
  "data": Array<PerpsWithdrawal>;
  "more": boolean;
}

export interface PostBlpEnrollParams {
  [field: string]: unknown;
  "body": PerpsBlpEnrollRequest;
}

export interface PostBlpEnrollResponse {
  [field: string]: unknown;
  "status": string;
}

export interface SetAutoCancelParams {
  [field: string]: unknown;
  "body"?: PerpsAutoCancelRequest | null;
}

export interface SetAutoCancelResponse {
  [field: string]: unknown;
  "status": string;
  "deadline": number | bigint | string;
}

export interface UpdateBuilderParams {
  [field: string]: unknown;
  "body": UpdateBuilderParamsBody;
}

export interface UpdateBuilderParamsBody {
  [field: string]: unknown;
  "op": UpdateBuilderParamsBodyOp;
  "sig"?: string | null;
  "salt"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
}

export interface UpdateBuilderParamsBodyOp {
  [field: string]: unknown;
  "type": string;
  "args": UpdateBuilderParamsBodyOpArgs;
}

export interface UpdateBuilderParamsBodyOpArgs {
  [field: string]: unknown;
  "name": string;
}

export interface UpdateBuilderResponse {
  [field: string]: unknown;
  "address": string;
  "registered": boolean;
  "enabled": boolean;
  "admission_enabled": boolean;
  "max_fee_rate": string;
  "name": string | null;
}

export interface UpdateLeverageParams {
  [field: string]: unknown;
  "body"?: PerpsLeverageRequest | null;
}

export interface UpdateLeverageResponse {
  [field: string]: unknown;
  "status": string;
  "instrument_id": number | bigint | string;
  "leverage": number | bigint | string;
  "cross": boolean;
}

export interface UpdateLeveragesParams {
  [field: string]: unknown;
  "body"?: PerpsLeverageBatchRequest | null;
}

export interface UpdateMarginParams {
  [field: string]: unknown;
  "body"?: PerpsMarginRequest | null;
}

export interface WithdrawParams {
  [field: string]: unknown;
  "body"?: PerpsWithdrawRequest | null;
}

export interface WithdrawResponse {
  [field: string]: unknown;
  "status": string;
  "withdraw_id": number | bigint | string;
}
