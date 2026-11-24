"""Phoenix REST wire models generated from src/exchanges/phoenix/schema/phoenix.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class AckBeforeTimestampBody(TypedDict):
    beforeTimestamp: str

class AckNotificationsBody(TypedDict):
    items: list[AckNotificationItem]

class AcknowledgeNotificationsParams(TypedDict):
    trader_pubkey: str
    body: AckNotificationsBody

class AcknowledgeNotificationsUpToTimestampParams(TypedDict):
    trader_pubkey: str
    body: AckBeforeTimestampBody

class ActivateInviteResponse(TypedDict):
    trader_pda: str

class ActivateInviteWithReferralRequest(TypedDict):
    authority: str
    referral_code: str

class ActivateReferralParams(TypedDict):
    body: ActivateInviteWithReferralRequest

_ActivateReferralTxRequestOptional = TypedDict("_ActivateReferralTxRequestOptional", {'trader_pda_index': 'int | Decimal | None', 'trader_subaccount_index': 'int | Decimal | None'}, total=False)
class ActivateReferralTxRequest(_ActivateReferralTxRequestOptional):
    recent_blockhash: str
    referral_code: str
    trader_authority: str
    transaction: str

_ActivateReferralTxResponseOptional = TypedDict("_ActivateReferralTxResponseOptional", {'signature': 'str | None'}, total=False)
class ActivateReferralTxResponse(_ActivateReferralTxResponseOptional):
    referral_code: str
    status: ActivateReferralTxStatus
    trader_pda: str

class ActivateReferralWithUserFundedTransactionParams(TypedDict):
    body: ActivateReferralTxRequest

class ApiAccountMeta(TypedDict):
    isSigner: bool
    isWritable: bool
    pubkey: str

_ApiCandleOptional = TypedDict("_ApiCandleOptional", {'externalSource': 'str | None', 'markClose': 'int | Decimal | str | None', 'markHigh': 'int | Decimal | str | None', 'markLow': 'int | Decimal | str | None', 'markOpen': 'int | Decimal | str | None', 'tradeCount': 'int | Decimal | None', 'volume': 'int | Decimal | None', 'volumeQuote': 'int | Decimal | None'}, total=False)
class ApiCandle(_ApiCandleOptional):
    close: int | Decimal | str
    high: int | Decimal | str
    low: int | Decimal | str
    open: int | Decimal | str
    time: int | Decimal | str

class ApiInstructionResponse(TypedDict):
    data: list[int | Decimal | str]
    keys: list[ApiAccountMeta]
    programId: str

class AuthResponse(TypedDict):
    access_token: str
    expires_in: int | Decimal | str
    pop_key: str
    refresh_expires_in: int | Decimal | str
    refresh_token: str
    token_type: str

class AuthoritySetView(TypedDict):
    adlAuthority: str
    backstopAuthority: str
    cancelAuthority: str
    marketAuthority: str
    oracleAuthority: str
    riskAuthority: str
    rootAuthority: str

class BuildAttachedConditionalOrderTransactionParams(TypedDict):
    body: PlaceAttachedConditionalOrderRequest

class BuildCancelConditionalOrderTransactionParams(TypedDict):
    body: CancelConditionalOrderRequest

class BuildCancelStopLossOrderTransactionParams(TypedDict):
    body: CancelStopLossOrderRequest

class BuildIsolatedLimitOrderTransactionParams(TypedDict):
    body: PlaceIsolatedLimitOrderRequest

class BuildIsolatedLimitOrderTransactionWithConditionalsParams(TypedDict):
    body: PlaceIsolatedLimitOrderWithConditionalsRequest

class BuildIsolatedLimitOrderTransactionWithLiquidationEstimateParams(TypedDict):
    body: PlaceIsolatedLimitOrderRequest

class BuildIsolatedMarketOrderTransactionParams(TypedDict):
    body: PlaceIsolatedMarketOrderRequest

class BuildIsolatedMarketOrderTransactionWithLiquidationEstimateParams(TypedDict):
    body: PlaceIsolatedMarketOrderRequest

class BuildPositionConditionalOrderTransactionParams(TypedDict):
    body: PlacePositionConditionalOrderRequest

_BuildRegisterIxsRequestOptional = TypedDict("_BuildRegisterIxsRequestOptional", {'maxPositions': 'int | Decimal | None'}, total=False)
class BuildRegisterIxsRequest(_BuildRegisterIxsRequestOptional):
    traderAuthority: str
    txFeePayer: str

class BuildRegisterIxsResponse(TypedDict):
    includeRegisterTrader: bool
    instructions: list[ApiInstructionResponse]
    maxPositions: int | Decimal | str
    traderOnboarder: str
    traderPda: str
    txFeePayer: str

class BuildRegisterTraderInstructionsParams(TypedDict):
    body: BuildRegisterIxsRequest

class BuildStopLossOrderTransactionParams(TypedDict):
    body: PlaceStopLossOrderRequest

_CancelConditionalOrderRequestOptional = TypedDict("_CancelConditionalOrderRequestOptional", {'isIsolated': 'bool | None', 'positionAuthority': 'str | None', 'traderSubaccountIndex': 'int | Decimal | None'}, total=False)
class CancelConditionalOrderRequest(_CancelConditionalOrderRequestOptional):
    authority: str
    conditionalOrderIndex: int | Decimal | str
    executionDirection: str
    symbol: str
    traderPdaIndex: int | Decimal | str

_CancelStopLossOrderRequestOptional = TypedDict("_CancelStopLossOrderRequestOptional", {'isIsolated': 'bool | None', 'positionAuthority': 'str | None', 'traderSubaccountIndex': 'int | Decimal | None'}, total=False)
class CancelStopLossOrderRequest(_CancelStopLossOrderRequestOptional):
    authority: str
    executionDirection: str
    symbol: str
    traderPdaIndex: int | Decimal | str

_CapabilityAccessViewOptional = TypedDict("_CapabilityAccessViewOptional", {'viaColdActivation': 'bool | None'}, total=False)
class CapabilityAccessView(_CapabilityAccessViewOptional):
    immediate: bool

class CheckWalletInviteStatusParams(TypedDict):
    wallet: str

_CheckWalletResponseOptional = TypedDict("_CheckWalletResponseOptional", {'invite_code_used': 'str | None', 'whitelisted_at': 'str | None'}, total=False)
class CheckWalletResponse(_CheckWalletResponseOptional):
    whitelisted: bool

class CollateralEvent(TypedDict):
    amount: int | Decimal | str
    collateralAfter: int | Decimal | str
    eventIndex: int | Decimal | str
    eventType: str
    slot: int | Decimal | str
    slotIndex: int | Decimal | str
    timestamp: str
    traderPdaIndex: int | Decimal | str
    traderSubaccountIndex: int | Decimal | str

_CollateralHistoryResponseOptional = TypedDict("_CollateralHistoryResponseOptional", {'nextCursor': 'str | None', 'prevCursor': 'str | None'}, total=False)
class CollateralHistoryResponse(_CollateralHistoryResponseOptional):
    data: list[CollateralEvent]
    hasMore: bool

class CommodityMarketCalendarResponse(TypedDict):
    calendar: CommodityMarketCalendarView
    loadedAt: str
    market: str
    rawJson: str

class CommodityMarketCalendarView(TypedDict):
    dateOverrides: dict[str, CommodityMarketDaySchedule]
    weeklySchedule: dict[str, CommodityMarketDaySchedule]

class CommodityMarketDaySchedule(TypedDict):
    sessions: list[CommodityMarketSessionRange]

class CommodityMarketSessionRange(TypedDict):
    end: str
    mode: str
    start: str

class CreateServiceLoginChallengeParams(TypedDict):
    body: ServiceChallengeRequest

class CreateWalletTransactionChallengeParams(TypedDict):
    body: WalletTransactionChallengeRequest

class ExchangeKeysView(TypedDict):
    activeTraderBuffer: list[str]
    canonicalMint: str
    currentAuthorities: AuthoritySetView
    globalConfig: str
    globalTraderIndex: list[str]
    globalVault: str
    pendingAuthorities: AuthoritySetView
    perpAssetMap: str
    withdrawQueue: str

_ExchangeLeverageTierOptional = TypedDict("_ExchangeLeverageTierOptional", {'limitOrderRiskFactorBps': 'int | Decimal | None'}, total=False)
class ExchangeLeverageTier(_ExchangeLeverageTierOptional):
    limitOrderRiskFactor: int | Decimal | str
    maxLeverage: int | Decimal | str
    maxSizeBaseLots: int | Decimal | str

_ExchangeMarketConfigOptional = TypedDict("_ExchangeMarketConfigOptional", {'commodityMetadata': 'Any | None', 'maxFundingRatePerIntervalPercentage': 'int | Decimal | str | None', 'metadata': 'Any | None', 'statsSnapshot': 'Any | None'}, total=False)
class ExchangeMarketConfig(_ExchangeMarketConfigOptional):
    assetId: int | Decimal | str
    baseLotsDecimals: int | Decimal | str
    fundingIntervalSeconds: int | Decimal | str
    fundingPeriodSeconds: int | Decimal | str
    isolatedOnly: bool
    leverageTiers: list[ExchangeLeverageTier]
    makerFee: int | Decimal | str
    marketPubkey: str
    marketStatus: MarketStatus
    maxFundingRatePerInterval: int | Decimal | str
    maxLiquidationSizeBaseLots: JsSafeU64
    openInterestCapBaseLots: JsSafeU64
    riskFactors: ExchangeRiskFactors
    splinePubkey: str
    symbol: str
    takerFee: int | Decimal | str
    tickSize: int | Decimal | str

_ExchangeMarketSnapshotOptional = TypedDict("_ExchangeMarketSnapshotOptional", {'commodityMetadata': 'Any | None', 'metadata': 'Any | None'}, total=False)
class ExchangeMarketSnapshot(_ExchangeMarketSnapshotOptional):
    assetId: int | Decimal | str
    baseLotsDecimals: int | Decimal | str
    fundingConfig: ExchangeWsFundingConfig
    isolatedOnly: bool
    leverageTiers: list[ExchangeWsLeverageTier]
    makerFee: int | Decimal | str
    markPriceParameters: ExchangeWsMarkPriceParameters
    marketPubkey: str
    marketStatus: MarketStatus
    maxLiquidationSizeBaseLots: JsSafeU64
    openInterestCapBaseLots: JsSafeU64
    riskFactors: ExchangeWsRiskFactors
    splinePubkey: str
    symbol: str
    takerFee: int | Decimal | str
    tickSize: int | Decimal | str

_ExchangeRiskFactorsOptional = TypedDict("_ExchangeRiskFactorsOptional", {'backstopBps': 'int | Decimal | None', 'cancelOrderBps': 'int | Decimal | None', 'highRiskBps': 'int | Decimal | None', 'maintenanceBps': 'int | Decimal | None', 'upnlBps': 'int | Decimal | None', 'upnlForWithdrawalsBps': 'int | Decimal | None'}, total=False)
class ExchangeRiskFactors(_ExchangeRiskFactorsOptional):
    backstop: int | Decimal | str
    cancelOrder: int | Decimal | str
    highRisk: int | Decimal | str
    maintenance: int | Decimal | str
    upnl: int | Decimal | str
    upnlForWithdrawals: int | Decimal | str

_ExchangeSnapshotViewOptional = TypedDict("_ExchangeSnapshotViewOptional", {'sequenceNumber': 'Any | None'}, total=False)
class ExchangeSnapshotView(_ExchangeSnapshotViewOptional):
    exchange: ExchangeStateSnapshot
    markets: list[ExchangeMarketSnapshot]
    slot: int | Decimal | str
    slotIndex: int | Decimal | str
    version: int | Decimal | str

_ExchangeStateSnapshotOptional = TypedDict("_ExchangeStateSnapshotOptional", {'withdrawalsAvailable': 'bool | None'}, total=False)
class ExchangeStateSnapshot(_ExchangeStateSnapshotOptional):
    active: bool
    activeTraderBuffer: list[str]
    canonicalMint: str
    currentAuthorities: AuthoritySetView
    exchangeStatusBits: int | Decimal | str
    exchangeStatusFeatures: list[str]
    gated: bool
    globalConfig: str
    globalTraderIndex: list[str]
    globalVault: str
    perpAssetMap: str
    programId: str
    usdcMint: str
    withdrawQueue: str

_ExchangeStatusViewOptional = TypedDict("_ExchangeStatusViewOptional", {'withdrawalsAvailable': 'bool | None'}, total=False)
class ExchangeStatusView(_ExchangeStatusViewOptional):
    active: bool
    gated: bool

class ExchangeView(TypedDict):
    keys: ExchangeKeysView
    markets: list[ExchangeMarketConfig]

class ExchangeWsFundingConfig(TypedDict):
    fundingIntervalSeconds: int | Decimal | str
    fundingPeriodSeconds: int | Decimal | str
    maxFundingRatePerInterval: int | Decimal | str

class ExchangeWsLeverageTier(TypedDict):
    limitOrderRiskFactor: int | Decimal | str
    maxLeverage: int | Decimal | str
    maxSizeBaseLots: JsSafeU64

_ExchangeWsMarkPriceParametersOptional = TypedDict("_ExchangeWsMarkPriceParametersOptional", {'bookHardStaleMultiplier': 'int | Decimal | str | None', 'oracleHardStaleMultiplier': 'int | Decimal | str | None'}, total=False)
class ExchangeWsMarkPriceParameters(_ExchangeWsMarkPriceParametersOptional):
    adjustedExchangeSpotPriceWeight: JsSafeU64
    bookPriceRadius: JsSafeU64
    bookPriceStaleThreshold: JsSafeU64
    bookPriceWeight: JsSafeU64
    commoditiesAfterHoursRadius: JsSafeU64
    commoditiesAfterHoursRadiusBps: JsSafeU64
    emaDiffRadius: JsSafeU64
    emaPeriodSlots: JsSafeU64
    exchangePerpPriceWeight: JsSafeU64
    minOracleResponses: int | Decimal | str
    oracleDivergenceRadius: int | Decimal | str
    perpPriceStaleThreshold: JsSafeU64
    riskActionPriceValidityRules: list[list[list[ExchangeWsValidationRule]]]
    spotPriceStaleThreshold: JsSafeU64

_ExchangeWsRiskFactorsOptional = TypedDict("_ExchangeWsRiskFactorsOptional", {'backstopBps': 'int | Decimal | None', 'cancelOrderBps': 'int | Decimal | None', 'highRiskBps': 'int | Decimal | None', 'maintenanceBps': 'int | Decimal | None', 'upnlBps': 'int | Decimal | None', 'upnlForWithdrawalsBps': 'int | Decimal | None'}, total=False)
class ExchangeWsRiskFactors(_ExchangeWsRiskFactorsOptional):
    backstop: int | Decimal | str
    cancelOrder: int | Decimal | str
    highRisk: int | Decimal | str
    maintenance: int | Decimal | str
    upnl: int | Decimal | str
    upnlForWithdrawals: int | Decimal | str

class FillRecord(TypedDict):
    baseQty: str
    instructionType: str
    marketSymbol: str
    price: str
    quoteQty: str
    timestamp: str
    transactionSignature: str

_FillsResponseOptional = TypedDict("_FillsResponseOptional", {'nextCursor': 'str | None', 'prevCursor': 'str | None'}, total=False)
class FillsResponse(_FillsResponseOptional):
    data: list[FillRecord]
    hasMore: bool

class FundingHourlyEvent(TypedDict):
    fundingPayment: str
    fundingRatePercentage: str
    positionSide: str
    positionSize: str
    symbol: str
    timestamp: str

_FundingHourlyHistoryResponseOptional = TypedDict("_FundingHourlyHistoryResponseOptional", {'nextCursor': 'str | None', 'prevCursor': 'str | None'}, total=False)
class FundingHourlyHistoryResponse(_FundingHourlyHistoryResponseOptional):
    events: list[FundingHourlyEvent]
    hasMore: bool

class FundingOverviewPoint(TypedDict):
    fundingAmountPerUnit: str
    fundingRate: str
    markPrice: str
    timestamp: str

class FundingOverviewResponse(TypedDict):
    series: list[FundingOverviewSeries]

class FundingOverviewSeries(TypedDict):
    marketId: int | Decimal | str
    points: list[FundingOverviewPoint]
    symbol: str

class FundingRateHistoryResponse(TypedDict):
    marketId: int | Decimal | str
    rates: list[FundingRatePoint]
    symbol: str

class FundingRatePoint(TypedDict):
    fundingRatePercentage: str
    timestamp: str

_GetCandlesParamsOptional = TypedDict("_GetCandlesParamsOptional", {'startTime': 'int | Decimal | str | None', 'endTime': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None', 'enableExternalSource': 'bool | None'}, total=False)
class GetCandlesParams(_GetCandlesParamsOptional):
    symbol: str
    timeframe: str

class GetCommodityMarketCalendarParams(TypedDict):
    pass

class GetExchangeKeysParams(TypedDict):
    pass

class GetExchangeParams(TypedDict):
    pass

class GetExchangeSnapshotParams(TypedDict):
    pass

class GetExchangeStatusParams(TypedDict):
    pass

_GetFundingOverviewParamsOptional = TypedDict("_GetFundingOverviewParamsOptional", {'startTime': 'int | Decimal | str | None', 'endTime': 'int | Decimal | str | None', 'perMarketLimit': 'int | Decimal | str | None'}, total=False)
class GetFundingOverviewParams(_GetFundingOverviewParamsOptional):
    pass

_GetFundingRateHistoryParamsOptional = TypedDict("_GetFundingRateHistoryParamsOptional", {'startTime': 'int | Decimal | str | None', 'endTime': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None'}, total=False)
class GetFundingRateHistoryParams(_GetFundingRateHistoryParamsOptional):
    symbol: str

class GetJwksParams(TypedDict):
    pass

class GetMarkPriceParams(TypedDict):
    symbol: str

class GetMarketCalendarByIdParams(TypedDict):
    market_calendar_id: str

class GetMarketCalendarParams(TypedDict):
    symbol: str

_GetMarketFillsParamsOptional = TypedDict("_GetMarketFillsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'startTime': 'int | Decimal | None', 'endTime': 'int | Decimal | None'}, total=False)
class GetMarketFillsParams(_GetMarketFillsParamsOptional):
    symbol: str

class GetMarketParams(TypedDict):
    symbol: str

_GetMarketStatsHistoryParamsOptional = TypedDict("_GetMarketStatsHistoryParamsOptional", {'start_time': 'str | None', 'end_time': 'str | None', 'limit': 'int | Decimal | None', 'timeframe': 'str | None'}, total=False)
class GetMarketStatsHistoryParams(_GetMarketStatsHistoryParamsOptional):
    symbol: str

class GetNextCommodityMarketTransitionParams(TypedDict):
    pass

class GetNextMarketCalendarTransitionParams(TypedDict):
    symbol: str

_GetNotificationsResponseOptional = TypedDict("_GetNotificationsResponseOptional", {'nextCursor': 'str | None'}, total=False)
class GetNotificationsResponse(_GetNotificationsResponseOptional):
    items: list[NotificationItem]

_GetOrderHistoryParamsOptional = TypedDict("_GetOrderHistoryParamsOptional", {'traderPdaIndex': 'int | Decimal | str | None', 'marketSymbol': 'str | None', 'cursor': 'str | None', 'privyId': 'str | None', 'orderStatus': 'str | None'}, total=False)
class GetOrderHistoryParams(_GetOrderHistoryParamsOptional):
    authority: str
    limit: int | Decimal | str

_GetOrderHistoryV2ParamsOptional = TypedDict("_GetOrderHistoryV2ParamsOptional", {'market_symbol': 'str | None', 'cursor': 'str | None'}, total=False)
class GetOrderHistoryV2Params(_GetOrderHistoryV2ParamsOptional):
    limit: int | Decimal | str
    trader_pubkey: str

_GetOrderbookParamsOptional = TypedDict("_GetOrderbookParamsOptional", {'include_splines': 'bool | None', 'bypass_execution_band': 'bool | None'}, total=False)
class GetOrderbookParams(_GetOrderbookParamsOptional):
    symbol: str

class GetReferralActivationPermissionParams(TypedDict):
    pass

_GetTradeHistoryParamsOptional = TypedDict("_GetTradeHistoryParamsOptional", {'pdaIndex': 'int | Decimal | str | None', 'marketSymbol': 'str | None', 'limit': 'int | Decimal | str | None', 'cursor': 'str | None'}, total=False)
class GetTradeHistoryParams(_GetTradeHistoryParamsOptional):
    authority: str

class GetTraderCapabilitiesParams(TypedDict):
    pass

_GetTraderCollateralHistoryParamsOptional = TypedDict("_GetTraderCollateralHistoryParamsOptional", {'pdaIndex': 'int | Decimal | str | None', 'nextCursor': 'str | None', 'prevCursor': 'str | None', 'cursor': 'str | None'}, total=False)
class GetTraderCollateralHistoryParams(_GetTraderCollateralHistoryParamsOptional):
    authority: str
    limit: int | Decimal | str

_GetTraderFundingHistoryParamsOptional = TypedDict("_GetTraderFundingHistoryParamsOptional", {'traderPdaIndex': 'int | Decimal | str | None', 'symbol': 'str | None', 'startTime': 'int | Decimal | str | None', 'endTime': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None', 'cursor': 'str | None', 'resolution': 'str | None'}, total=False)
class GetTraderFundingHistoryParams(_GetTraderFundingHistoryParamsOptional):
    authority: str

_GetTraderMarketPnLParamsOptional = TypedDict("_GetTraderMarketPnLParamsOptional", {'startTime': 'int | Decimal | str | None', 'endTime': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None', 'symbols': 'str | None'}, total=False)
class GetTraderMarketPnLParams(_GetTraderMarketPnLParamsOptional):
    trader_pubkey: str
    resolution: str

class GetTraderParams(TypedDict):
    pubkey: str

_GetTraderPnLParamsOptional = TypedDict("_GetTraderPnLParamsOptional", {'startTime': 'int | Decimal | str | None', 'endTime': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None'}, total=False)
class GetTraderPnLParams(_GetTraderPnLParamsOptional):
    trader_pubkey: str
    resolution: str

_GetTraderPortfolioValuesParamsOptional = TypedDict("_GetTraderPortfolioValuesParamsOptional", {'startTime': 'int | Decimal | str | None', 'endTime': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None'}, total=False)
class GetTraderPortfolioValuesParams(_GetTraderPortfolioValuesParamsOptional):
    trader_pubkey: str
    resolution: str

_GetTraderStateParamsOptional = TypedDict("_GetTraderStateParamsOptional", {'traderPdaIndex': 'int | Decimal | str | None'}, total=False)
class GetTraderStateParams(_GetTraderStateParamsOptional):
    authority_pubkey: str

_GetTraderTradeHistoryV2ParamsOptional = TypedDict("_GetTraderTradeHistoryV2ParamsOptional", {'market_symbol': 'str | None', 'trader_pda_index': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None', 'cursor': 'str | None', 'privy_id': 'str | None'}, total=False)
class GetTraderTradeHistoryV2Params(_GetTraderTradeHistoryV2ParamsOptional):
    trader_pubkey: str

_GetUserCollateralHistoryParamsOptional = TypedDict("_GetUserCollateralHistoryParamsOptional", {'nextCursor': 'str | None', 'prevCursor': 'str | None', 'cursor': 'str | None'}, total=False)
class GetUserCollateralHistoryParams(_GetUserCollateralHistoryParamsOptional):
    limit: int | Decimal | str
    user_pubkey: str

_GetUserHourlyFundingParamsOptional = TypedDict("_GetUserHourlyFundingParamsOptional", {'traderPdaIndex': 'int | Decimal | str | None', 'symbol': 'str | None', 'limit': 'int | Decimal | str | None', 'cursor': 'str | None'}, total=False)
class GetUserHourlyFundingParams(_GetUserHourlyFundingParamsOptional):
    user_pubkey: str

_GetUserLiquidationHistoryParamsOptional = TypedDict("_GetUserLiquidationHistoryParamsOptional", {'pdaIndex': 'int | Decimal | str | None', 'subaccountIndex': 'int | Decimal | None', 'symbol': 'str | None', 'limit': 'int | Decimal | str | None', 'cursor': 'str | None'}, total=False)
class GetUserLiquidationHistoryParams(_GetUserLiquidationHistoryParamsOptional):
    user_pubkey: str

_GetUserPnLParamsOptional = TypedDict("_GetUserPnLParamsOptional", {'startTime': 'int | Decimal | str | None', 'endTime': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None'}, total=False)
class GetUserPnLParams(_GetUserPnLParamsOptional):
    user_pubkey: str
    resolution: str

_GetUserTradeHistoryV2ParamsOptional = TypedDict("_GetUserTradeHistoryV2ParamsOptional", {'market_symbol': 'str | None', 'trader_pda_index': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None', 'cursor': 'str | None', 'privy_id': 'str | None'}, total=False)
class GetUserTradeHistoryV2Params(_GetUserTradeHistoryV2ParamsOptional):
    user_pubkey: str

_GetV1TradersByTraderPubkeyCollateralHistoryParamsOptional = TypedDict("_GetV1TradersByTraderPubkeyCollateralHistoryParamsOptional", {'nextCursor': 'str | None', 'prevCursor': 'str | None', 'cursor': 'str | None'}, total=False)
class GetV1TradersByTraderPubkeyCollateralHistoryParams(_GetV1TradersByTraderPubkeyCollateralHistoryParamsOptional):
    limit: int | Decimal | str
    trader_pubkey: str

class GetWalletLoginNonceParams(TypedDict):
    wallet_pubkey: str

_IntendedOrderOptional = TypedDict("_IntendedOrderOptional", {'clientOrderId': 'str | None', 'isConditionalOrder': 'bool | None', 'isStopLossDirection': 'bool | None', 'lastValidSlot': 'int | Decimal | None', 'matchLimit': 'int | Decimal | None', 'priceInTicks': 'int | Decimal | None', 'priceUsd': 'str | None', 'quoteLotBudget': 'int | Decimal | None'}, total=False)
class IntendedOrder(_IntendedOrderOptional):
    baseLots: int | Decimal | str
    baseQty: str
    cancelExisting: bool
    hasPriceLimit: bool
    isPostOnly: bool
    isReduceOnly: bool
    isStopLoss: bool
    isTakeOnly: bool
    orderFlags: int | Decimal | str
    orderKind: str
    side: str

class JwksResponse(TypedDict):
    keys: list[Any]

_LimitOrderOptional = TypedDict("_LimitOrderOptional", {'isConditionalOrder': 'bool | None', 'isStopLoss': 'bool | None', 'isStopLossDirection': 'bool | None'}, total=False)
class LimitOrder(_LimitOrderOptional):
    initialTradeSize: str
    isReduceOnly: bool
    marginFactor: str
    marginRequirement: str
    orderSequenceNumber: str
    price: str
    side: Side
    tradeSizeRemaining: str

class ListMarketsParams(TypedDict):
    pass

_ListTraderNotificationsParamsOptional = TypedDict("_ListTraderNotificationsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'unackedOnly': 'bool | None'}, total=False)
class ListTraderNotificationsParams(_ListTraderNotificationsParamsOptional):
    trader_pubkey: str

class LogInWithPrivyTokenParams(TypedDict):
    body: PrivyLoginRequest

class LogInWithServiceSignatureParams(TypedDict):
    body: ServiceLoginRequest

class LogInWithSignedWalletTransactionParams(TypedDict):
    body: WalletTransactionLoginRequest

class LogInWithWalletSignatureParams(TypedDict):
    body: WalletLoginRequest

class LogOutParams(TypedDict):
    pass

_MarkPriceResponseOptional = TypedDict("_MarkPriceResponseOptional", {'markPrice': 'Any | None'}, total=False)
class MarkPriceResponse(_MarkPriceResponseOptional):
    slot: int | Decimal | str
    slotIndex: int | Decimal | str
    symbol: str

_MarketCalendarRecordOptional = TypedDict("_MarketCalendarRecordOptional", {'calendar': 'Any | None'}, total=False)
class MarketCalendarRecord(_MarketCalendarRecordOptional):
    calendarUri: str
    contentSha256: str
    description: str
    kind: str
    loadedAt: str
    marketCalendarId: str
    rawJson: str
    s3Path: str
    updatedAt: str

_MarketCalendarResponseOptional = TypedDict("_MarketCalendarResponseOptional", {'calendar': 'Any | None'}, total=False)
class MarketCalendarResponse(_MarketCalendarResponseOptional):
    calendarUri: str
    contentSha256: str
    description: str
    kind: str
    loadedAt: str
    market: str
    marketCalendarId: str
    rawJson: str

_MarketStatsHistoryResponseOptional = TypedDict("_MarketStatsHistoryResponseOptional", {'timeframe': 'str | None'}, total=False)
class MarketStatsHistoryResponse(_MarketStatsHistoryResponseOptional):
    market_id: int | Decimal | str
    stats: list[MarketStatsPoint]
    symbol: str

_MarketStatsPointOptional = TypedDict("_MarketStatsPointOptional", {'total_maker_fees': 'int | Decimal | None', 'total_taker_fees': 'int | Decimal | None'}, total=False)
class MarketStatsPoint(_MarketStatsPointOptional):
    mark_price: int | Decimal | str
    open_interest: int | Decimal | str
    slot: int | Decimal | str
    spot_price: int | Decimal | str
    timestamp: str

_NextCommodityMarketTransitionOptional = TypedDict("_NextCommodityMarketTransitionOptional", {'nextMarketState': 'Any | None', 'utcNextTransition': 'str | None'}, total=False)
class NextCommodityMarketTransition(_NextCommodityMarketTransitionOptional):
    currentState: CommodityMarketStateView
    loadedAt: str
    market: str

_NextMarketCalendarTransitionOptional = TypedDict("_NextMarketCalendarTransitionOptional", {'nextMarketState': 'Any | None', 'utcNextTransition': 'str | None'}, total=False)
class NextMarketCalendarTransition(_NextMarketCalendarTransitionOptional):
    calendarUri: str
    currentState: MarketCalendarStateView
    loadedAt: str
    market: str
    marketCalendarId: str

_OrderbookViewOptional = TypedDict("_OrderbookViewOptional", {'mid': 'int | Decimal | None', 'splines': 'list[Spline] | None'}, total=False)
class OrderbookView(_OrderbookViewOptional):
    asks: list[list[int | Decimal | str]]
    bids: list[list[int | Decimal | str]]
    slot: int | Decimal | str
    symbol: str

_PaginatedResponseVecOrderHistoryItemOptional = TypedDict("_PaginatedResponseVecOrderHistoryItemOptional", {'nextCursor': 'str | None', 'prevCursor': 'str | None'}, total=False)
class PaginatedResponseVecOrderHistoryItem(_PaginatedResponseVecOrderHistoryItemOptional):
    data: list[PaginatedResponseVecOrderHistoryItemDataItem]
    hasMore: bool

_PaginatedResponseVecOrderHistoryItemDataItemOptional = TypedDict("_PaginatedResponseVecOrderHistoryItemDataItemOptional", {'completedAt': 'str | None', 'isConditionalOrder': 'bool | None', 'isStopLoss': 'bool | None', 'isStopLossDirection': 'bool | None', 'placedAt': 'str | None'}, total=False)
class PaginatedResponseVecOrderHistoryItemDataItem(_PaginatedResponseVecOrderHistoryItemDataItemOptional):
    baseQty: str
    filledBaseQty: str
    isReduceOnly: bool
    marketSymbol: str
    orderSequenceNumber: str
    price: str
    remainingBaseQty: str
    side: Side
    status: OrderStatus

_PaginatedResponseVecOrderHistoryV2ItemOptional = TypedDict("_PaginatedResponseVecOrderHistoryV2ItemOptional", {'nextCursor': 'str | None', 'prevCursor': 'str | None'}, total=False)
class PaginatedResponseVecOrderHistoryV2Item(_PaginatedResponseVecOrderHistoryV2ItemOptional):
    data: list[PaginatedResponseVecOrderHistoryV2ItemDataItem]
    hasMore: bool

_PaginatedResponseVecOrderHistoryV2ItemDataItemOptional = TypedDict("_PaginatedResponseVecOrderHistoryV2ItemDataItemOptional", {'currentOrderState': 'Any | None', 'fillBeforePlacement': 'Any | None', 'placedOrder': 'Any | None'}, total=False)
class PaginatedResponseVecOrderHistoryV2ItemDataItem(_PaginatedResponseVecOrderHistoryV2ItemDataItemOptional):
    createdAt: str
    eventIndex: int | Decimal | str
    instructionIndex: int | Decimal | str
    instructionType: str
    intended: IntendedOrder
    isConditionalOrder: bool
    isReduceOnly: bool
    isStopLoss: bool
    isStopLossDirection: bool
    marketSymbol: str
    orderType: str
    price: str
    slot: int | Decimal | str
    slotIndex: int | Decimal | str
    status: str
    traderId: int | Decimal | str
    traderPdaIndex: int | Decimal | str
    traderSubaccountIndex: int | Decimal | str
    userId: int | Decimal | str

_PaginatedResponseVecTradeHistoryV2ItemOptional = TypedDict("_PaginatedResponseVecTradeHistoryV2ItemOptional", {'nextCursor': 'str | None', 'prevCursor': 'str | None'}, total=False)
class PaginatedResponseVecTradeHistoryV2Item(_PaginatedResponseVecTradeHistoryV2ItemOptional):
    data: list[PaginatedResponseVecTradeHistoryV2ItemDataItem]
    hasMore: bool

_PaginatedResponseVecTradeHistoryV2ItemDataItemOptional = TypedDict("_PaginatedResponseVecTradeHistoryV2ItemDataItemOptional", {'fillId': 'str | None', 'orderSequenceNumber': 'int | Decimal | None', 'signature': 'str | None', 'splineSequenceNumber': 'int | Decimal | None'}, total=False)
class PaginatedResponseVecTradeHistoryV2ItemDataItem(_PaginatedResponseVecTradeHistoryV2ItemDataItemOptional):
    baseLotsAfter: str
    baseLotsBefore: str
    baseLotsDelta: str
    eventIndex: int | Decimal | str
    fees: str
    instructionIndex: int | Decimal | str
    instructionType: str
    liquidity: LiquidityRole
    marketSymbol: str
    price: str
    realizedPnl: str
    slot: int | Decimal | str
    slotIndex: int | Decimal | str
    subaccountIndex: int | Decimal | str
    timestamp: str
    tradeType: TradeType
    traderId: int | Decimal | str
    traderPdaIndex: int | Decimal | str
    userId: int | Decimal | str
    virtualQuoteLotsAfter: str
    virtualQuoteLotsBefore: str
    virtualQuoteLotsDelta: str

_PaginatedResponseVecUserLiquidationHistoryPointOptional = TypedDict("_PaginatedResponseVecUserLiquidationHistoryPointOptional", {'nextCursor': 'str | None', 'prevCursor': 'str | None'}, total=False)
class PaginatedResponseVecUserLiquidationHistoryPoint(_PaginatedResponseVecUserLiquidationHistoryPointOptional):
    data: list[PaginatedResponseVecUserLiquidationHistoryPointDataItem]
    hasMore: bool

_PaginatedResponseVecUserLiquidationHistoryPointDataItemOptional = TypedDict("_PaginatedResponseVecUserLiquidationHistoryPointDataItemOptional", {'atLossCloseValue': 'str | None', 'atLossCollateralChange': 'str | None', 'baseLotsFilled': 'str | None', 'caller': 'str | None', 'closedLong': 'str | None', 'closedShort': 'str | None', 'haircutRateBps': 'int | Decimal | None', 'inProfitAccount': 'str | None', 'inProfitCloseValue': 'str | None', 'inProfitCollateralChange': 'str | None', 'liquidatee': 'str | None', 'liquidateeCollateralChange': 'str | None', 'liquidator': 'str | None', 'liquidatorCollateralChange': 'str | None', 'positionClosed': 'bool | None', 'price': 'str | None', 'quoteLotsFilled': 'str | None', 'quoteSize': 'str | None', 'side': 'str | None', 'signature': 'str | None', 'size': 'str | None', 'subaccountIndex': 'int | Decimal | None'}, total=False)
class PaginatedResponseVecUserLiquidationHistoryPointDataItem(_PaginatedResponseVecUserLiquidationHistoryPointDataItemOptional):
    eventIndex: int | Decimal | str
    ixName: str
    kind: UserLiquidationHistoryKind
    market: str
    role: UserLiquidationHistoryRole
    slot: int | Decimal | str
    slotIndex: int | Decimal | str
    symbol: str
    timestamp: int | Decimal | str
    type: UserLiquidationHistoryType

_PlaceAttachedConditionalOrderRequestOptional = TypedDict("_PlaceAttachedConditionalOrderRequestOptional", {'feePayer': 'str | None', 'flightBuilderAuthority': 'str | None', 'flightFeeCollectorTrader': 'str | None', 'greaterTrigger': 'Any | None', 'isIsolated': 'bool | None', 'lessTrigger': 'Any | None', 'positionAuthority': 'str | None', 'traderSubaccountIndex': 'int | Decimal | None'}, total=False)
class PlaceAttachedConditionalOrderRequest(_PlaceAttachedConditionalOrderRequestOptional):
    authority: str
    orderPriceInTicks: int | Decimal | str
    orderSequenceNumber: str
    symbol: str
    traderPdaIndex: int | Decimal | str

_PlaceIsolatedLimitOrderEnhancedResponseOptional = TypedDict("_PlaceIsolatedLimitOrderEnhancedResponseOptional", {'estimatedLiquidationPriceUsd': 'int | Decimal | None'}, total=False)
class PlaceIsolatedLimitOrderEnhancedResponse(_PlaceIsolatedLimitOrderEnhancedResponseOptional):
    instructions: list[ApiInstructionResponse]

_PlaceIsolatedLimitOrderRequestOptional = TypedDict("_PlaceIsolatedLimitOrderRequestOptional", {'allowCrossAndIsolatedForAsset': 'bool | None', 'feePayer': 'str | None', 'flightBuilderAuthority': 'str | None', 'flightFeeCollectorTrader': 'str | None', 'isPostOnly': 'bool | None', 'isReduceOnly': 'bool | None', 'numBaseLots': 'int | Decimal | None', 'pdaIndex': 'int | Decimal | None', 'positionAuthority': 'str | None', 'price': 'int | Decimal | None', 'priceInTicks': 'int | Decimal | None', 'quantity': 'int | Decimal | None', 'skipTransferToParent': 'bool | None', 'slide': 'bool | None', 'tpSl': 'Any | None', 'transferAmount': 'int | Decimal | str | None'}, total=False)
class PlaceIsolatedLimitOrderRequest(_PlaceIsolatedLimitOrderRequestOptional):
    authority: str
    side: str
    symbol: str

_PlaceIsolatedLimitOrderWithConditionalsRequestOptional = TypedDict("_PlaceIsolatedLimitOrderWithConditionalsRequestOptional", {'allowCrossAndIsolatedForAsset': 'bool | None', 'feePayer': 'str | None', 'flightBuilderAuthority': 'str | None', 'flightFeeCollectorTrader': 'str | None', 'greaterTrigger': 'Any | None', 'isPostOnly': 'bool | None', 'lessTrigger': 'Any | None', 'numBaseLots': 'int | Decimal | None', 'pdaIndex': 'int | Decimal | None', 'positionAuthority': 'str | None', 'price': 'int | Decimal | None', 'priceInTicks': 'int | Decimal | None', 'quantity': 'int | Decimal | None', 'skipTransferToParent': 'bool | None', 'slide': 'bool | None', 'transferAmount': 'int | Decimal | str | None'}, total=False)
class PlaceIsolatedLimitOrderWithConditionalsRequest(_PlaceIsolatedLimitOrderWithConditionalsRequestOptional):
    authority: str
    side: str
    symbol: str

_PlaceIsolatedMarketOrderEnhancedResponseOptional = TypedDict("_PlaceIsolatedMarketOrderEnhancedResponseOptional", {'estimatedLiquidationPriceUsd': 'int | Decimal | None'}, total=False)
class PlaceIsolatedMarketOrderEnhancedResponse(_PlaceIsolatedMarketOrderEnhancedResponseOptional):
    instructions: list[ApiInstructionResponse]

_PlaceIsolatedMarketOrderRequestOptional = TypedDict("_PlaceIsolatedMarketOrderRequestOptional", {'allowCrossAndIsolatedForAsset': 'bool | None', 'feePayer': 'str | None', 'flightBuilderAuthority': 'str | None', 'flightFeeCollectorTrader': 'str | None', 'isReduceOnly': 'bool | None', 'maxPriceInTicks': 'int | Decimal | None', 'numBaseLots': 'int | Decimal | None', 'pdaIndex': 'int | Decimal | None', 'positionAuthority': 'str | None', 'quantity': 'int | Decimal | None', 'skipTransferToParent': 'bool | None', 'tpSl': 'Any | None', 'transferAmount': 'int | Decimal | str | None'}, total=False)
class PlaceIsolatedMarketOrderRequest(_PlaceIsolatedMarketOrderRequestOptional):
    authority: str
    side: str
    symbol: str

_PlacePositionConditionalOrderRequestOptional = TypedDict("_PlacePositionConditionalOrderRequestOptional", {'feePayer': 'str | None', 'flightBuilderAuthority': 'str | None', 'flightFeeCollectorTrader': 'str | None', 'greaterTrigger': 'Any | None', 'isIsolated': 'bool | None', 'lessTrigger': 'Any | None', 'numBaseLots': 'int | Decimal | None', 'positionAuthority': 'str | None', 'quantity': 'int | Decimal | None', 'sizePercent': 'int | Decimal | None', 'traderSubaccountIndex': 'int | Decimal | None'}, total=False)
class PlacePositionConditionalOrderRequest(_PlacePositionConditionalOrderRequestOptional):
    authority: str
    symbol: str
    traderPdaIndex: int | Decimal | str

class PnLPoint(TypedDict):
    cumulativeFundingPayment: int | Decimal | str
    cumulativePnl: int | Decimal | str
    cumulativeTakerFee: int | Decimal | str
    endTime: int | Decimal | str
    startTime: int | Decimal | str
    timestamp: int | Decimal | str
    unrealizedPnl: int | Decimal | str

_PortfolioValuePointOptional = TypedDict("_PortfolioValuePointOptional", {'positions': 'Any | None'}, total=False)
class PortfolioValuePoint(_PortfolioValuePointOptional):
    endTime: int | Decimal | str
    startTime: int | Decimal | str
    timestamp: int | Decimal | str
    value: int | Decimal | str

_PrivyLoginRequestOptional = TypedDict("_PrivyLoginRequestOptional", {'wallet_proof': 'Any | None', 'wallet_pubkey': 'str | None'}, total=False)
class PrivyLoginRequest(_PrivyLoginRequestOptional):
    privy_token: str

class ReferralActivationPermissionResponse(TypedDict):
    permission_account: str
    risk_authority: str
    trader_onboarder: str

class RefreshRequest(TypedDict):
    refresh_token: str

class RefreshSessionParams(TypedDict):
    body: RefreshRequest

_SendRegisterIxsRequestOptional = TypedDict("_SendRegisterIxsRequestOptional", {'maxPositions': 'int | Decimal | None', 'traderPdaIndex': 'int | Decimal | None', 'traderSubaccountIndex': 'int | Decimal | None'}, total=False)
class SendRegisterIxsRequest(_SendRegisterIxsRequestOptional):
    traderAuthority: str
    transaction: str
    txFeePayer: str

class SendRegisterIxsResponse(TypedDict):
    includeRegisterTrader: bool
    maxPositions: int | Decimal | str
    signature: str
    traderOnboarder: str
    traderPda: str
    txFeePayer: str

_ServiceChallengeRequestOptional = TypedDict("_ServiceChallengeRequestOptional", {'key_id': 'str | None'}, total=False)
class ServiceChallengeRequest(_ServiceChallengeRequestOptional):
    client_id: str

class ServiceChallengeResponse(TypedDict):
    expires_at: str
    key_id: str
    message: str
    nonce: str

_ServiceLoginRequestOptional = TypedDict("_ServiceLoginRequestOptional", {'key_id': 'str | None', 'nonce': 'str', 'signature': 'str'}, total=False)
class ServiceLoginRequest(_ServiceLoginRequestOptional):
    client_id: str
    timestamp: str

class Spline(TypedDict):
    askFilledAmount: str
    askRegions: list[TickRegion]
    bidFilledAmount: str
    bidRegions: list[TickRegion]
    midPrice: int | Decimal | str
    traderKey: str

class SubmitRegisterTraderTransactionParams(TypedDict):
    body: SendRegisterIxsRequest

class TickRegion(TypedDict):
    density: int | Decimal | str
    end: int | Decimal | str
    filledSize: str
    start: int | Decimal | str
    totalSize: str

class TraderCapabilitiesMetadata(TypedDict):
    capabilities: list[TraderCapabilityDescriptor]

class TraderCapabilitiesView(TypedDict):
    depositCollateral: CapabilityAccessView
    placeLimitOrder: CapabilityAccessView
    placeMarketOrder: CapabilityAccessView
    riskIncreasingTrade: CapabilityAccessView
    riskReducingTrade: CapabilityAccessView
    withdrawCollateral: CapabilityAccessView

_TraderCapabilityDescriptorOptional = TypedDict("_TraderCapabilityDescriptorOptional", {'deprecated': 'bool | None'}, total=False)
class TraderCapabilityDescriptor(_TraderCapabilityDescriptorOptional):
    description: str
    displayName: str
    key: str

class TraderFundingHistoryEvent(TypedDict):
    fundingPayment: str
    fundingRatePercentage: str
    positionSide: str
    positionSize: str
    symbol: str
    timestamp: str

_TraderFundingHistoryResponseOptional = TypedDict("_TraderFundingHistoryResponseOptional", {'nextCursor': 'str | None', 'prevCursor': 'str | None'}, total=False)
class TraderFundingHistoryResponse(_TraderFundingHistoryResponseOptional):
    events: list[TraderFundingHistoryEvent]
    hasMore: bool

class TraderMarketPnLPoint(TypedDict):
    baseLots: int | Decimal | str
    cumulativeFundingPayment: int | Decimal | str
    cumulativePnl: int | Decimal | str
    cumulativeTakerFee: int | Decimal | str
    endTime: int | Decimal | str
    markPrice: int | Decimal | str
    realizedPnl: int | Decimal | str
    startTime: int | Decimal | str
    timestamp: int | Decimal | str
    totalTakerFee: int | Decimal | str
    unrealizedPnl: int | Decimal | str
    virtualQuoteLots: int | Decimal | str

class TraderMarketPnLSeries(TypedDict):
    baseDecimals: int | Decimal | str
    marketId: int | Decimal | str
    points: list[TraderMarketPnLPoint]
    symbol: str
    tickSize: int | Decimal | str

_TraderPositionViewOptional = TypedDict("_TraderPositionViewOptional", {'stopLossPrice': 'str | None', 'takeProfitPrice': 'str | None'}, total=False)
class TraderPositionView(_TraderPositionViewOptional):
    accumulatedFunding: str
    backstopMargin: str
    discountedUnrealizedPnl: str
    entryPrice: str
    initialMargin: str
    limitOrderMargin: str
    liquidationPrice: str
    maintenanceMargin: str
    positionInitialMargin: str
    positionSize: str
    positionValue: str
    symbol: str
    unrealizedPnl: str
    unsettledFunding: str
    virtualQuotePosition: str

class TraderStateCapabilities(TypedDict):
    capabilities: TraderCapabilitiesView
    flags: TraderCapabilityFlags
    state: TraderActivityStateView

class TraderStateLimitOrderEvent(TypedDict):
    orders: list[TraderStateMarketLimitOrderEvent]
    symbol: str

_TraderStateMarketLimitOrderEventOptional = TypedDict("_TraderStateMarketLimitOrderEventOptional", {'change': 'Any | None', 'conditionalKind': 'str | None', 'isConditionalOrder': 'bool | None', 'isStopLoss': 'bool | None', 'isStopLossDirection': 'bool | None'}, total=False)
class TraderStateMarketLimitOrderEvent(_TraderStateMarketLimitOrderEventOptional):
    initialSizeLots: str
    orderSequenceNumber: str
    orderType: str
    priceTicks: str
    priceUsd: str
    reduceOnly: bool
    side: Side
    sizeRemainingLots: str
    status: str

class TraderStateSnapshot(TypedDict):
    capabilities: TraderStateCapabilities
    makerFeeOverrideMultiplier: int | Decimal | str
    subaccounts: list[TraderStateSubaccountSnapshot]
    takerFeeOverrideMultiplier: int | Decimal | str
    version: int | Decimal | str

class TraderStateSnapshotResponse(TypedDict):
    authority: str
    slot: int | Decimal | str
    slotIndex: int | Decimal | str
    snapshot: TraderStateSnapshot
    traderPdaIndex: int | Decimal | str

_TraderStateSubaccountSnapshotOptional = TypedDict("_TraderStateSubaccountSnapshotOptional", {'capabilities': 'Any | None', 'cooldownStatus': 'Any | None', 'orders': 'list[TraderStateLimitOrderEvent] | None', 'positions': 'list[TraderStatePositionSnapshot] | None', 'splines': 'list[TraderStateSplineSnapshot] | None', 'triggers': 'list[TraderStateTriggerSnapshot] | None'}, total=False)
class TraderStateSubaccountSnapshot(_TraderStateSubaccountSnapshotOptional):
    collateral: str
    sequence: int | Decimal | str
    subaccountIndex: int | Decimal | str

class TraderView(TypedDict):
    accumulatedFunding: str
    authority: str
    cancelMargin: str
    capabilities: TraderCapabilitiesView
    collateralBalance: str
    discountedUnrealizedPnl: str
    effectiveCollateral: str
    effectiveCollateralForWithdrawals: str
    flags: TraderCapabilityFlags
    initialMargin: str
    initialMarginForWithdrawals: str
    isInActiveTraders: bool
    lastDepositSlot: int | Decimal | str
    limitOrders: dict[str, list[LimitOrder]]
    maintenanceMargin: str
    makerFeeOverrideMultiplier: int | Decimal | str
    maxPositions: int | Decimal | str
    numMarketsWithSplines: int | Decimal | str
    portfolioValue: str
    positions: list[TraderPositionView]
    riskState: RiskState
    riskTier: RiskTier
    slot: int | Decimal | str
    slotIndex: int | Decimal | str
    state: TraderActivityStateView
    takerFeeOverrideMultiplier: int | Decimal | str
    traderKey: str
    traderPdaIndex: int | Decimal | str
    traderSubaccountIndex: int | Decimal | str
    unrealizedPnl: str
    unsettledFundingOwed: str

class ValidateInviteParams(TypedDict):
    body: ValidateInviteRequest

_ValidateInviteRequestOptional = TypedDict("_ValidateInviteRequestOptional", {'transaction_signature': 'str | None'}, total=False)
class ValidateInviteRequest(_ValidateInviteRequestOptional):
    code: str
    wallet_address: str

class ValidateInviteResponse(TypedDict):
    message: str
    success: bool
    whitelisted: bool

_WalletLoginRequestOptional = TypedDict("_WalletLoginRequestOptional", {'signature': 'str'}, total=False)
class WalletLoginRequest(_WalletLoginRequestOptional):
    nonce_id: str
    wallet_pubkey: str

class WalletNonceResponse(TypedDict):
    expires_at: str
    message: str
    nonce_id: str

class WalletTransactionChallengeRequest(TypedDict):
    wallet_pubkey: str

class WalletTransactionChallengeResponse(TypedDict):
    expires_at: str
    nonce_id: str
    unsigned_transaction: str

class WalletTransactionLoginRequest(TypedDict):
    nonce_id: str
    signed_transaction: str
    wallet_pubkey: str

AckNotificationItem = Any
ActivateReferralTxStatus = str
CommodityMarketStateView = str
ExchangeWsValidationRule = str
JsSafeU64 = int | Decimal | str
LiquidityRole = str
MarketCalendarStateView = str
MarketStatus = str
NotificationItem = Any
OrderStatus = str
PlaceStopLossOrderRequest = Any
RiskState = str
RiskTier = str
Side = str
TradeType = str
TraderActivityStateView = str
TraderCapabilityFlags = int | Decimal | str
TraderStatePositionSnapshot = Any
TraderStateSplineSnapshot = Any
TraderStateTriggerSnapshot = Any
UserLiquidationHistoryKind = str
UserLiquidationHistoryRole = str
UserLiquidationHistoryType = str
