"""PolymarketPerps REST wire models generated from src/exchanges/polymarket/schema/polymarket_perps.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class ApplyReferralParams(TypedDict):
    body: PerpsAccountReferralRequest

class ApplyReferralResponse(TypedDict):
    status: str

class ApproveBuilderParams(TypedDict):
    body: PerpsBuilderApprovalRequest

class ApproveBuilderResponse(TypedDict):
    trader: str
    builder: str
    max_fee_rate: str
    approval_version: int | Decimal
    timestamp: int | Decimal
    sequence: int | Decimal

_CancelAllOrdersParamsOptional = TypedDict("_CancelAllOrdersParamsOptional", {'body': 'PerpsCancelAllRequest | None'}, total=False)
class CancelAllOrdersParams(_CancelAllOrdersParamsOptional):
    pass

class CancelAllOrdersResponse(TypedDict):
    status: str

class CancelChaseParams(TypedDict):
    body: CancelChaseParamsBody

_CancelChaseParamsBodyOptional = TypedDict("_CancelChaseParamsBodyOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'exp': 'int | Decimal | None'}, total=False)
class CancelChaseParamsBody(_CancelChaseParamsBodyOptional):
    op: CancelChaseParamsBodyOp

class CancelChaseParamsBodyOp(TypedDict):
    type: str
    args: CancelChaseParamsBodyOpArgs

class CancelChaseParamsBodyOpArgs(TypedDict):
    chid: int | Decimal

_CancelOrdersByCoidParamsOptional = TypedDict("_CancelOrdersByCoidParamsOptional", {'body': 'PerpsCancelByCoidRequest | None'}, total=False)
class CancelOrdersByCoidParams(_CancelOrdersByCoidParamsOptional):
    pass

_CancelOrdersParamsOptional = TypedDict("_CancelOrdersParamsOptional", {'body': 'PerpsCancelRequest | None'}, total=False)
class CancelOrdersParams(_CancelOrdersParamsOptional):
    pass

class CancelTwapParams(TypedDict):
    body: CancelTwapParamsBody

_CancelTwapParamsBodyOptional = TypedDict("_CancelTwapParamsBodyOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'exp': 'int | Decimal | None'}, total=False)
class CancelTwapParamsBody(_CancelTwapParamsBodyOptional):
    op: CancelTwapParamsBodyOp

class CancelTwapParamsBodyOp(TypedDict):
    type: str
    args: CancelTwapParamsBodyOpArgs

class CancelTwapParamsBodyOpArgs(TypedDict):
    twid: int | Decimal

_CheckInviteCodeParamsOptional = TypedDict("_CheckInviteCodeParamsOptional", {'address': 'str | None'}, total=False)
class CheckInviteCodeParams(_CheckInviteCodeParamsOptional):
    code: str

_CheckInviteCodeResponseOptional = TypedDict("_CheckInviteCodeResponseOptional", {'error': 'str | None'}, total=False)
class CheckInviteCodeResponse(_CheckInviteCodeResponseOptional):
    valid: bool

class ControlTwapParams(TypedDict):
    body: ControlTwapParamsBody

_ControlTwapParamsBodyOptional = TypedDict("_ControlTwapParamsBodyOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'exp': 'int | Decimal | None'}, total=False)
class ControlTwapParamsBody(_ControlTwapParamsBodyOptional):
    op: ControlTwapParamsBodyOp

class ControlTwapParamsBodyOp(TypedDict):
    type: str
    args: ControlTwapParamsBodyOpArgs

class ControlTwapParamsBodyOpArgs(TypedDict):
    twid: int | Decimal
    act: str

_CreateAccountInviteParamsOptional = TypedDict("_CreateAccountInviteParamsOptional", {'body': 'PerpsCreateInviteRequest | None'}, total=False)
class CreateAccountInviteParams(_CreateAccountInviteParamsOptional):
    pass

class CreateAccountInviteResponse(TypedDict):
    status: str
    code: str
    referrals_available: int | Decimal
    cooldown_ms: int | Decimal | None

class CreateChaseParams(TypedDict):
    body: CreateChaseParamsBody

_CreateChaseParamsBodyOptional = TypedDict("_CreateChaseParamsBodyOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'exp': 'int | Decimal | None'}, total=False)
class CreateChaseParamsBody(_CreateChaseParamsBodyOptional):
    op: CreateChaseParamsBodyOp

class CreateChaseParamsBodyOp(TypedDict):
    type: str
    args: CreateChaseParamsBodyOpArgs

_CreateChaseParamsBodyOpArgsOptional = TypedDict("_CreateChaseParamsBodyOpArgsOptional", {'lim': 'str | None', 'max_dist': 'str | None', 'max_dist_bps': 'int | Decimal | None', 'po': 'bool | None', 'ro': 'bool | None', 'c': 'str | None'}, total=False)
class CreateChaseParamsBodyOpArgs(_CreateChaseParamsBodyOpArgsOptional):
    iid: int | Decimal
    buy: bool
    qty: str

_CreateOrdersParamsOptional = TypedDict("_CreateOrdersParamsOptional", {'body': 'PerpsOrderRequest | None'}, total=False)
class CreateOrdersParams(_CreateOrdersParamsOptional):
    pass

_CreateProxyParamsOptional = TypedDict("_CreateProxyParamsOptional", {'body': 'PerpsProxyRequest | None'}, total=False)
class CreateProxyParams(_CreateProxyParamsOptional):
    pass

class CreateProxyResponse(TypedDict):
    status: str
    secret: str

class CreateTwapParams(TypedDict):
    body: CreateTwapParamsBody

_CreateTwapParamsBodyOptional = TypedDict("_CreateTwapParamsBodyOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'exp': 'int | Decimal | None'}, total=False)
class CreateTwapParamsBody(_CreateTwapParamsBodyOptional):
    op: CreateTwapParamsBodyOp

class CreateTwapParamsBodyOp(TypedDict):
    type: str
    args: CreateTwapParamsBodyOpArgs

_CreateTwapParamsBodyOpArgsOptional = TypedDict("_CreateTwapParamsBodyOpArgsOptional", {'dur': 'int | Decimal | None', 'ivl': 'int | Decimal | None', 'rnd': 'bool | None', 'slip_bps': 'int | Decimal | None', 'min_px': 'str | None', 'max_px': 'str | None', 'ro': 'bool | None', 'c': 'str | None'}, total=False)
class CreateTwapParamsBodyOpArgs(_CreateTwapParamsBodyOpArgsOptional):
    iid: int | Decimal
    buy: bool
    qty: str

_DeleteProxyParamsOptional = TypedDict("_DeleteProxyParamsOptional", {'body': 'PerpsDeleteProxyRequest | None'}, total=False)
class DeleteProxyParams(_DeleteProxyParamsOptional):
    pass

class DeleteProxyResponse(TypedDict):
    status: str

_GetAccountBackstopsParamsOptional = TypedDict("_GetAccountBackstopsParamsOptional", {'start_timestamp': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetAccountBackstopsParams(_GetAccountBackstopsParamsOptional):
    pass

_GetAccountBackstopsResponseOptional = TypedDict("_GetAccountBackstopsResponseOptional", {'coverage': 'GetAccountBackstopsResponseCoverage | None'}, total=False)
class GetAccountBackstopsResponse(_GetAccountBackstopsResponseOptional):
    data: list[PerpsAccountBackstop]
    more: bool

class GetAccountBackstopsResponseCoverage(TypedDict):
    notes: list[str]

_GetAccountFundingParamsOptional = TypedDict("_GetAccountFundingParamsOptional", {'instrument_id': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None'}, total=False)
class GetAccountFundingParams(_GetAccountFundingParamsOptional):
    pass

class GetAccountFundingResponse(TypedDict):
    data: list[PerpsAccountFundingData]
    more: bool

class GetAccountLimitsParams(TypedDict):
    pass

class GetAccountLimitsResponse(TypedDict):
    actions_per_minute: int | Decimal
    actions_per_minute_limit: int | Decimal
    actions_burst: int | Decimal
    actions_burst_limit: int | Decimal
    open_orders: int | Decimal
    open_orders_limit: int | Decimal
    reset: int | Decimal
    messages_per_minute: int | Decimal

_GetAccountNotificationsParamsOptional = TypedDict("_GetAccountNotificationsParamsOptional", {'cursor': 'str | None', 'since_seq': 'int | Decimal | None', 'limit': 'int | Decimal | None'}, total=False)
class GetAccountNotificationsParams(_GetAccountNotificationsParamsOptional):
    pass

class GetAccountNotificationsResponse(TypedDict):
    items: list[PerpsNotificationItem]
    unread: int | Decimal
    durable_source_seq: int | Decimal
    has_more: bool
    next_cursor: str | None

class GetAccountReferralParams(TypedDict):
    pass

_GetAccountReferralResponseOptional = TypedDict("_GetAccountReferralResponseOptional", {'parent': 'str | None'}, total=False)
class GetAccountReferralResponse(_GetAccountReferralResponseOptional):
    code: str
    children_count: int | Decimal
    active_children_count: int | Decimal
    children_volume: str
    children_fees: str
    referrals_left: int | Decimal
    fee_share_rate: str

_GetAccountRewardsParamsOptional = TypedDict("_GetAccountRewardsParamsOptional", {'date': 'str | None', 'start_date': 'str | None', 'end_date': 'str | None', 'limit': 'int | Decimal | None'}, total=False)
class GetAccountRewardsParams(_GetAccountRewardsParamsOptional):
    pass

class GetAccountRewardsResponse(TypedDict):
    data: list[PerpsAccountReward]
    more: bool

class GetAccountStatsParams(TypedDict):
    pass

_GetAccountStatsResponseOptional = TypedDict("_GetAccountStatsResponseOptional", {'entity_maker_share_7d': 'str | None', 'entity_id': 'int | Decimal | None', 'entity_name': 'str | None'}, total=False)
class GetAccountStatsResponse(_GetAccountStatsResponseOptional):
    volume_7d: str
    taker_volume_7d: str
    maker_volume_7d: str
    account_maker_share_7d: str

class GetAssetsParams(TypedDict):
    pass

class GetAutoCancelParams(TypedDict):
    pass

class GetAutoCancelResponse(TypedDict):
    deadline: int | Decimal
    triggered: int | Decimal
    daily_limit: int | Decimal
    next_reset: int | Decimal

class GetBalancesParams(TypedDict):
    pass

_GetBboParamsOptional = TypedDict("_GetBboParamsOptional", {'instrument_id': 'int | Decimal | None'}, total=False)
class GetBboParams(_GetBboParamsOptional):
    pass

class GetBlpEnrollmentParams(TypedDict):
    pass

class GetBlpEnrollmentResponse(TypedDict):
    blp: bool
    instruments: list[PerpsBlpInstrument]

_GetBlpLiquidationsParamsOptional = TypedDict("_GetBlpLiquidationsParamsOptional", {'start_timestamp': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None', 'cursor': 'int | Decimal | None', 'sort': 'str | None'}, total=False)
class GetBlpLiquidationsParams(_GetBlpLiquidationsParamsOptional):
    pass

class GetBlpLiquidationsResponse(TypedDict):
    data: list[PerpsBlpLiquidation]
    more: bool

_GetBookParamsOptional = TypedDict("_GetBookParamsOptional", {'depth': 'int | Decimal | None'}, total=False)
class GetBookParams(_GetBookParamsOptional):
    instrument_id: int | Decimal

class GetBookResponse(TypedDict):
    instrument_id: int | Decimal
    bids: list[PerpsLevel]
    asks: list[PerpsLevel]
    timestamp: int | Decimal
    sequence: int | Decimal

_GetBuilderApprovalsParamsOptional = TypedDict("_GetBuilderApprovalsParamsOptional", {'builder': 'str | None'}, total=False)
class GetBuilderApprovalsParams(_GetBuilderApprovalsParamsOptional):
    pass

class GetBuilderApprovalsResponse(TypedDict):
    data: list[PerpsBuilderApproval]

_GetBuilderEarningsParamsOptional = TypedDict("_GetBuilderEarningsParamsOptional", {'start_timestamp': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None', 'as_of_sequence': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetBuilderEarningsParams(_GetBuilderEarningsParamsOptional):
    pass

_GetBuilderEarningsResponseOptional = TypedDict("_GetBuilderEarningsResponseOptional", {'cursor': 'str | None'}, total=False)
class GetBuilderEarningsResponse(_GetBuilderEarningsResponseOptional):
    data: list[PerpsBuilderEarning]
    more: bool
    start_timestamp: int | Decimal
    end_timestamp: int | Decimal
    as_of_sequence: int | Decimal

_GetBuilderEarningsSummaryParamsOptional = TypedDict("_GetBuilderEarningsSummaryParamsOptional", {'start_timestamp': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None', 'as_of_sequence': 'int | Decimal | None'}, total=False)
class GetBuilderEarningsSummaryParams(_GetBuilderEarningsSummaryParamsOptional):
    pass

class GetBuilderEarningsSummaryResponse(TypedDict):
    data: list[PerpsBuilderEarningsAsset]
    start_timestamp: int | Decimal
    end_timestamp: int | Decimal
    as_of_sequence: int | Decimal
    trader_count: int | Decimal
    active_approval_count: int | Decimal

class GetBuilderParams(TypedDict):
    address: str

class GetBuilderResponse(TypedDict):
    address: str
    registered: bool
    enabled: bool
    admission_enabled: bool
    max_fee_rate: str
    name: str | None

class GetChasesParams(TypedDict):
    pass

class GetCompetitionAccountParams(TypedDict):
    competition_id: int | Decimal
    address: str

_GetCompetitionAccountResponseOptional = TypedDict("_GetCompetitionAccountResponseOptional", {'updated_at': 'int | Decimal | None', 'rank': 'int | Decimal | None', 'first_fill_at': 'int | Decimal | None', 'prize': 'str | None', 'spots_from_prize_zone': 'int | Decimal | None', 'score_to_prize_zone': 'str | None', 'payout_tx_hash': 'str | None', 'paid_at': 'int | Decimal | None'}, total=False)
class GetCompetitionAccountResponse(_GetCompetitionAccountResponseOptional):
    competition_id: int | Decimal
    status: str
    account: str
    qualified: bool
    score: str
    pnl: str
    realized_pnl: str
    unrealized_pnl: str
    fees: str
    funding: str
    volume: str
    boosted_volume: str
    min_volume: str
    volume_to_qualify: str
    qualification_progress: str
    row_state: str
    locked_positions: list[PerpsCompetitionLockedPosition]

_GetCompetitionLeaderboardParamsOptional = TypedDict("_GetCompetitionLeaderboardParamsOptional", {'limit': 'int | Decimal | None', 'offset': 'int | Decimal | None'}, total=False)
class GetCompetitionLeaderboardParams(_GetCompetitionLeaderboardParamsOptional):
    competition_id: int | Decimal

_GetCompetitionLeaderboardResponseOptional = TypedDict("_GetCompetitionLeaderboardResponseOptional", {'updated_at': 'int | Decimal | None'}, total=False)
class GetCompetitionLeaderboardResponse(_GetCompetitionLeaderboardResponseOptional):
    competition_id: int | Decimal
    status: str
    total: int | Decimal
    offset: int | Decimal
    final_results: bool
    entries: list[PerpsCompetitionEntry]

class GetCompetitionParams(TypedDict):
    competition_id: int | Decimal

_GetCompetitionResponseOptional = TypedDict("_GetCompetitionResponseOptional", {'start_trading_instrument_id': 'int | Decimal | None', 'updated_at': 'int | Decimal | None', 'prize_cutoff_score': 'str | None'}, total=False)
class GetCompetitionResponse(_GetCompetitionResponseOptional):
    competition_id: int | Decimal
    number: int | Decimal
    name: str
    sector_label: str
    boost_description: str
    status: str
    start_time: int | Decimal
    end_time: int | Decimal
    all_markets: bool
    markets: list[PerpsCompetitionMarket]
    min_volume: str
    count_unrealized: bool
    prize_pool: str
    prize_asset_id: int | Decimal
    prize_tiers: list[PerpsCompetitionPrizeTier]
    paid_places: int | Decimal
    internal: bool
    ranked: int | Decimal
    participants: int | Decimal
    final_results: bool
    scoring_version: int | Decimal

_GetCompetitionWidgetParamsOptional = TypedDict("_GetCompetitionWidgetParamsOptional", {'address': 'str | None'}, total=False)
class GetCompetitionWidgetParams(_GetCompetitionWidgetParamsOptional):
    pass

_GetCompetitionWidgetResponseOptional = TypedDict("_GetCompetitionWidgetResponseOptional", {'competition_id': 'int | Decimal | None', 'number': 'int | Decimal | None', 'end_time': 'int | Decimal | None', 'rank': 'int | Decimal | None', 'qualified': 'bool | None', 'volume': 'str | None', 'min_volume': 'str | None', 'qualification_progress': 'str | None', 'spots_from_prize_zone': 'int | Decimal | None', 'updated_at': 'int | Decimal | None'}, total=False)
class GetCompetitionWidgetResponse(_GetCompetitionWidgetResponseOptional):
    timestamp: int | Decimal

_GetCompetitionsParamsOptional = TypedDict("_GetCompetitionsParamsOptional", {'address': 'str | None'}, total=False)
class GetCompetitionsParams(_GetCompetitionsParamsOptional):
    pass

_GetCompetitionsResponseOptional = TypedDict("_GetCompetitionsResponseOptional", {'live_competition_id': 'int | Decimal | None', 'landing_competition_id': 'int | Decimal | None'}, total=False)
class GetCompetitionsResponse(_GetCompetitionsResponseOptional):
    timestamp: int | Decimal
    competitions: list[PerpsCompetitionSummary]

_GetConfigParamsOptional = TypedDict("_GetConfigParamsOptional", {'instrument_id': 'int | Decimal | None'}, total=False)
class GetConfigParams(_GetConfigParamsOptional):
    pass

class GetCredentialsParams(TypedDict):
    pass

class GetCredentialsResponse(TypedDict):
    address: str
    keys: list[PerpsKey]

_GetDepositsParamsOptional = TypedDict("_GetDepositsParamsOptional", {'deposit_status': 'str | None', 'hash': 'str | None', 'start_timestamp': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None'}, total=False)
class GetDepositsParams(_GetDepositsParamsOptional):
    pass

class GetDepositsResponse(TypedDict):
    data: list[PerpsDeposit]
    more: bool

_GetEquityHistoryParamsOptional = TypedDict("_GetEquityHistoryParamsOptional", {'end_timestamp': 'int | Decimal | None'}, total=False)
class GetEquityHistoryParams(_GetEquityHistoryParamsOptional):
    interval: str
    start_timestamp: int | Decimal

class GetEquityHistoryResponse(TypedDict):
    data: list[list[Any]]
    more: bool

class GetExchangeParams(TypedDict):
    pass

class GetExchangeResponse(TypedDict):
    name: str
    version: str
    chain_id: int | Decimal
    contract: str
    cancel_only: bool
    engine_version: str

class GetExchangeStatisticsParams(TypedDict):
    start_timestamp: int | Decimal
    end_timestamp: int | Decimal

class GetExchangeStatisticsResponse(TypedDict):
    start_timestamp: int | Decimal
    end_timestamp: int | Decimal
    volume: str
    open_interest: str | None
    open_interest_timestamp: int | Decimal | None
    fees: str

_GetFillsParamsOptional = TypedDict("_GetFillsParamsOptional", {'instrument_id': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None', 'cursor': 'str | None', 'sort': 'str | None'}, total=False)
class GetFillsParams(_GetFillsParamsOptional):
    pass

_GetFillsResponseOptional = TypedDict("_GetFillsResponseOptional", {'cursor': 'str | None'}, total=False)
class GetFillsResponse(_GetFillsResponseOptional):
    data: list[PerpsAccountTradeData]
    more: bool

_GetFundingHistoryParamsOptional = TypedDict("_GetFundingHistoryParamsOptional", {'start_timestamp': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None'}, total=False)
class GetFundingHistoryParams(_GetFundingHistoryParamsOptional):
    instrument_id: int | Decimal

class GetFundingHistoryResponse(TypedDict):
    data: list[PerpsFundingRate]
    more: bool

class GetIndexParams(TypedDict):
    asset: str

class GetIndexResponse(TypedDict):
    asset: str
    index_price: str
    constituents: list[PerpsIndexConstituent]
    ts: int | Decimal

class GetInfoFeesParams(TypedDict):
    pass

class GetInfoFeesResponse(TypedDict):
    fee_schedule: list[PerpsFeeScheduleEntry]

_GetInstrumentsParamsOptional = TypedDict("_GetInstrumentsParamsOptional", {'instrument_id': 'int | Decimal | None', 'instrument_type': 'str | None', 'category': 'str | None'}, total=False)
class GetInstrumentsParams(_GetInstrumentsParamsOptional):
    pass

_GetInternalTransfersParamsOptional = TypedDict("_GetInternalTransfersParamsOptional", {'direction': 'str | None', 'start_timestamp': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None'}, total=False)
class GetInternalTransfersParams(_GetInternalTransfersParamsOptional):
    pass

class GetInternalTransfersResponse(TypedDict):
    data: list[PerpsInternalTransfer]
    more: bool

_GetKlinesParamsOptional = TypedDict("_GetKlinesParamsOptional", {'end_timestamp': 'int | Decimal | None'}, total=False)
class GetKlinesParams(_GetKlinesParamsOptional):
    instrument_id: int | Decimal
    interval: str
    start_timestamp: int | Decimal

class GetKlinesResponse(TypedDict):
    data: list[PerpsKline]
    more: bool

_GetLeaderboardParamsOptional = TypedDict("_GetLeaderboardParamsOptional", {'window': 'str | None', 'sort_by': 'str | None', 'limit': 'int | Decimal | None', 'offset': 'int | Decimal | None', 'address': 'str | None'}, total=False)
class GetLeaderboardParams(_GetLeaderboardParamsOptional):
    pass

_GetLeaderboardResponseOptional = TypedDict("_GetLeaderboardResponseOptional", {'account': 'PerpsLeaderboardAccount | None'}, total=False)
class GetLeaderboardResponse(_GetLeaderboardResponseOptional):
    window: str
    sort_by: str
    timestamp: int | Decimal
    total: int | Decimal
    entries: list[PerpsLeaderboardEntry]

class GetLimitTiersParams(TypedDict):
    pass

_GetMarkHistoryParamsOptional = TypedDict("_GetMarkHistoryParamsOptional", {'end_timestamp': 'int | Decimal | None'}, total=False)
class GetMarkHistoryParams(_GetMarkHistoryParamsOptional):
    instrument_id: int | Decimal
    interval: str
    start_timestamp: int | Decimal

class GetMarkHistoryResponse(TypedDict):
    data: list[PerpsMarkPoint]
    more: bool

_GetOpenOrdersParamsOptional = TypedDict("_GetOpenOrdersParamsOptional", {'instrument_id': 'int | Decimal | None'}, total=False)
class GetOpenOrdersParams(_GetOpenOrdersParamsOptional):
    pass

_GetOrdersParamsOptional = TypedDict("_GetOrdersParamsOptional", {'order_id': 'int | Decimal | None', 'client_order_id': 'str | None', 'instrument_id': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None'}, total=False)
class GetOrdersParams(_GetOrdersParamsOptional):
    pass

class GetPingParams(TypedDict):
    pass

class GetPingResponse(TypedDict):
    status: str

_GetPnlHistoryParamsOptional = TypedDict("_GetPnlHistoryParamsOptional", {'end_timestamp': 'int | Decimal | None'}, total=False)
class GetPnlHistoryParams(_GetPnlHistoryParamsOptional):
    interval: str
    start_timestamp: int | Decimal

class GetPnlHistoryResponse(TypedDict):
    data: list[list[Any]]
    more: bool

class GetPortfolioParams(TypedDict):
    pass

class GetPortfolioResponse(TypedDict):
    positions: list[PerpsPortfolioPosition]
    margin: PerpsMarginSummary
    withdrawable: str
    in_liquidation: bool
    fee_tier: int | Decimal
    timestamp: int | Decimal

class GetPublicPortfolioParams(TypedDict):
    address: str

class GetPublicPortfolioResponse(TypedDict):
    positions: list[PerpsPublicPortfolioPosition]
    equity: str
    timestamp: int | Decimal

_GetPublicPositionFillsParamsOptional = TypedDict("_GetPublicPositionFillsParamsOptional", {'cursor': 'str | None', 'sort': 'str | None'}, total=False)
class GetPublicPositionFillsParams(_GetPublicPositionFillsParamsOptional):
    address: str
    instrument_id: int | Decimal

_GetPublicPositionFillsResponseOptional = TypedDict("_GetPublicPositionFillsResponseOptional", {'cursor': 'str | None'}, total=False)
class GetPublicPositionFillsResponse(_GetPublicPositionFillsResponseOptional):
    data: list[PerpsAccountTradeData]
    more: bool

class GetPublicPositionSnapshotsParams(TypedDict):
    body: PerpsPositionSnapshotsRequest

class GetPublicPositionSnapshotsResponse(TypedDict):
    history_as_of_at: int | Decimal
    active: list[PerpsPositionSnapshotResult]
    history: list[PerpsPositionSnapshotResult]

class GetRegisteredParams(TypedDict):
    address: str

class GetRegisteredResponse(TypedDict):
    registered: bool

_GetStatisticsParamsOptional = TypedDict("_GetStatisticsParamsOptional", {'instrument_id': 'int | Decimal | None'}, total=False)
class GetStatisticsParams(_GetStatisticsParamsOptional):
    pass

_GetTickersParamsOptional = TypedDict("_GetTickersParamsOptional", {'instrument_id': 'int | Decimal | None'}, total=False)
class GetTickersParams(_GetTickersParamsOptional):
    pass

class GetTimeParams(TypedDict):
    pass

class GetTimeResponse(TypedDict):
    time: int | Decimal

_GetTradesParamsOptional = TypedDict("_GetTradesParamsOptional", {'start_timestamp': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None'}, total=False)
class GetTradesParams(_GetTradesParamsOptional):
    instrument_id: int | Decimal

class GetTradesResponse(TypedDict):
    data: list[PerpsTradeData]
    more: bool

class GetTwapsParams(TypedDict):
    pass

_GetWithdrawalsParamsOptional = TypedDict("_GetWithdrawalsParamsOptional", {'withdrawal_status': 'str | None', 'hash': 'str | None', 'start_timestamp': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None'}, total=False)
class GetWithdrawalsParams(_GetWithdrawalsParamsOptional):
    pass

class GetWithdrawalsResponse(TypedDict):
    data: list[PerpsWithdrawal]
    more: bool

_InternalTransferParamsOptional = TypedDict("_InternalTransferParamsOptional", {'body': 'PerpsInternalTransferRequest | None'}, total=False)
class InternalTransferParams(_InternalTransferParamsOptional):
    pass

class InternalTransferResponse(TypedDict):
    status: str
    transfer_id: int | Decimal

class MarkAccountNotificationsReadParams(TypedDict):
    body: PerpsNotificationsReadRequest

class MarkAccountNotificationsReadResponse(TypedDict):
    status: str

_ModifyOrdersByCoidParamsOptional = TypedDict("_ModifyOrdersByCoidParamsOptional", {'body': 'PerpsModifyByCoidRequest | None'}, total=False)
class ModifyOrdersByCoidParams(_ModifyOrdersByCoidParamsOptional):
    pass

_ModifyOrdersParamsOptional = TypedDict("_ModifyOrdersParamsOptional", {'body': 'PerpsModifyRequest | None'}, total=False)
class ModifyOrdersParams(_ModifyOrdersParamsOptional):
    pass

class PerpsAccountBackstop(TypedDict):
    id: int | Decimal
    cross: bool
    instrument_id: int | Decimal | None
    asset: str
    transferred: str
    cash_before: str
    cash_after: str
    timestamp: int | Decimal
    sequence: int | Decimal

_PerpsAccountBackstopsOptional = TypedDict("_PerpsAccountBackstopsOptional", {'coverage': 'PerpsAccountBackstopsCoverage | None'}, total=False)
class PerpsAccountBackstops(_PerpsAccountBackstopsOptional):
    data: list[PerpsAccountBackstop]
    more: bool

class PerpsAccountBackstopsCoverage(TypedDict):
    notes: list[str]

class PerpsAccountConfig(TypedDict):
    instrument_id: int | Decimal
    leverage: int | Decimal
    cross: bool

class PerpsAccountFundingData(TypedDict):
    id: int | Decimal
    instrument_id: int | Decimal
    size: str
    funding_rate: str
    funding_asset: str
    funding: str
    timestamp: int | Decimal

class PerpsAccountFundingHistory(TypedDict):
    data: list[PerpsAccountFundingData]
    more: bool

class PerpsAccountLimits(TypedDict):
    actions_per_minute: int | Decimal
    actions_per_minute_limit: int | Decimal
    actions_burst: int | Decimal
    actions_burst_limit: int | Decimal
    open_orders: int | Decimal
    open_orders_limit: int | Decimal
    reset: int | Decimal
    messages_per_minute: int | Decimal

_PerpsAccountReferralOptional = TypedDict("_PerpsAccountReferralOptional", {'parent': 'str | None'}, total=False)
class PerpsAccountReferral(_PerpsAccountReferralOptional):
    code: str
    children_count: int | Decimal
    active_children_count: int | Decimal
    children_volume: str
    children_fees: str
    referrals_left: int | Decimal
    fee_share_rate: str

class PerpsAccountReferralRequest(TypedDict):
    code: str

_PerpsAccountRewardOptional = TypedDict("_PerpsAccountRewardOptional", {'oi_rewards': 'PerpsAccountRewardOi | None'}, total=False)
class PerpsAccountReward(_PerpsAccountRewardOptional):
    date: str
    maker_share_7d: str
    reward_distributed: bool
    breakdown: list[PerpsAccountRewardInstrument]

class PerpsAccountRewardInstrument(TypedDict):
    instrument_id: int | Decimal
    reward_pool: str
    reward_amount: str

class PerpsAccountRewardOi(TypedDict):
    account_oi: str
    reward_amount: str

class PerpsAccountRewards(TypedDict):
    data: list[PerpsAccountReward]
    more: bool

_PerpsAccountStatsOptional = TypedDict("_PerpsAccountStatsOptional", {'entity_maker_share_7d': 'str | None', 'entity_id': 'int | Decimal | None', 'entity_name': 'str | None'}, total=False)
class PerpsAccountStats(_PerpsAccountStatsOptional):
    volume_7d: str
    taker_volume_7d: str
    maker_volume_7d: str
    account_maker_share_7d: str

_PerpsAccountTradeDataOptional = TypedDict("_PerpsAccountTradeDataOptional", {'builder': 'PerpsBuilder | None'}, total=False)
class PerpsAccountTradeData(_PerpsAccountTradeDataOptional):
    trade_id: int | Decimal
    order_id: int | Decimal
    instrument_id: int | Decimal
    side: str
    price: str
    quantity: str
    taker: bool
    fee: str
    fee_asset: str
    previous_size: str
    previous_entry_price: str
    pnl: str
    liquidation: bool
    adl: bool
    timestamp: int | Decimal
    hash: str
    builder_fee: str
    total_fee: str
    settlement: bool

_PerpsAccountTradesOptional = TypedDict("_PerpsAccountTradesOptional", {'cursor': 'str | None'}, total=False)
class PerpsAccountTrades(_PerpsAccountTradesOptional):
    data: list[PerpsAccountTradeData]
    more: bool

class PerpsAsset(TypedDict):
    asset: str
    address: str
    decimals: int | Decimal
    collateral_ratio: str
    withdrawal_fee: str

_PerpsAutoCancelRequestOptional = TypedDict("_PerpsAutoCancelRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None'}, total=False)
class PerpsAutoCancelRequest(_PerpsAutoCancelRequestOptional):
    op: PerpsOpAutoCancel

class PerpsAutoCancelResponse(TypedDict):
    status: str
    deadline: int | Decimal

class PerpsAutoCancelStatus(TypedDict):
    deadline: int | Decimal
    triggered: int | Decimal
    daily_limit: int | Decimal
    next_reset: int | Decimal

class PerpsBBO(TypedDict):
    instrument_id: int | Decimal
    bid_price: str
    bid_quantity: str
    ask_price: str
    ask_quantity: str
    timestamp: int | Decimal

class PerpsBalance(TypedDict):
    asset: str
    balance: str
    value: str

class PerpsBaseOp(TypedDict):
    sig: str
    salt: int | Decimal
    ts: int | Decimal

class PerpsBlpEnrollRequest(TypedDict):
    instrument_id: int | Decimal
    active: bool

class PerpsBlpEnrollment(TypedDict):
    blp: bool
    instruments: list[PerpsBlpInstrument]

class PerpsBlpInstrument(TypedDict):
    instrument_id: int | Decimal
    symbol: str

class PerpsBlpLiquidation(TypedDict):
    liquidation_id: int | Decimal
    timestamp: int | Decimal
    cross: bool
    margin: str
    trades: list[PerpsBlpLiquidationTrade]

class PerpsBlpLiquidationTrade(TypedDict):
    trade_id: int | Decimal
    instrument_id: int | Decimal
    side: str
    quantity: str
    price: str

class PerpsBlpLiquidations(TypedDict):
    data: list[PerpsBlpLiquidation]
    more: bool

class PerpsBook(TypedDict):
    instrument_id: int | Decimal
    bids: list[PerpsLevel]
    asks: list[PerpsLevel]
    timestamp: int | Decimal
    sequence: int | Decimal

class PerpsBuilder(TypedDict):
    address: str
    fee_rate: str

class PerpsBuilderApproval(TypedDict):
    trader: str
    builder: str
    max_fee_rate: str
    approval_version: int | Decimal
    timestamp: int | Decimal
    sequence: int | Decimal

_PerpsBuilderApprovalRequestOptional = TypedDict("_PerpsBuilderApprovalRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None'}, total=False)
class PerpsBuilderApprovalRequest(_PerpsBuilderApprovalRequestOptional):
    op: PerpsOpApproveBuilder

class PerpsBuilderApprovals(TypedDict):
    data: list[PerpsBuilderApproval]

_PerpsBuilderEarningOptional = TypedDict("_PerpsBuilderEarningOptional", {'client_order_id': 'str | None'}, total=False)
class PerpsBuilderEarning(_PerpsBuilderEarningOptional):
    earning_id: str
    trade_id: int | Decimal
    order_id: int | Decimal
    instrument_id: int | Decimal
    side: str
    timestamp: int | Decimal
    sequence: int | Decimal
    notional: str
    fee_asset: str
    builder_fee: str
    fee_rate: str
    trader: str
    buy: bool
    price: str
    quantity: str
    fee: str
    total_fee: str

_PerpsBuilderEarningsOptional = TypedDict("_PerpsBuilderEarningsOptional", {'cursor': 'str | None'}, total=False)
class PerpsBuilderEarnings(_PerpsBuilderEarningsOptional):
    data: list[PerpsBuilderEarning]
    more: bool
    start_timestamp: int | Decimal
    end_timestamp: int | Decimal
    as_of_sequence: int | Decimal

class PerpsBuilderEarningsAsset(TypedDict):
    fee_asset: str
    fill_count: int | Decimal
    notional: str
    builder_fee: str

class PerpsBuilderEarningsSummary(TypedDict):
    data: list[PerpsBuilderEarningsAsset]
    start_timestamp: int | Decimal
    end_timestamp: int | Decimal
    as_of_sequence: int | Decimal
    trader_count: int | Decimal
    active_approval_count: int | Decimal

class PerpsBuilderStatus(TypedDict):
    address: str
    registered: bool
    enabled: bool
    admission_enabled: bool
    max_fee_rate: str

_PerpsCancelAcceptedOptional = TypedDict("_PerpsCancelAcceptedOptional", {'coid': 'str | None'}, total=False)
class PerpsCancelAccepted(_PerpsCancelAcceptedOptional):
    status: str
    oid: int | Decimal
    ts: int | Decimal

_PerpsCancelAllRequestOptional = TypedDict("_PerpsCancelAllRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'exp': 'int | Decimal | None'}, total=False)
class PerpsCancelAllRequest(_PerpsCancelAllRequestOptional):
    op: PerpsOpCancelAll

_PerpsCancelByCoidRequestOptional = TypedDict("_PerpsCancelByCoidRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'exp': 'int | Decimal | None'}, total=False)
class PerpsCancelByCoidRequest(_PerpsCancelByCoidRequestOptional):
    op: PerpsOpCancelOrdersByCoid

_PerpsCancelRejectedOptional = TypedDict("_PerpsCancelRejectedOptional", {'oid': 'int | Decimal | None', 'coid': 'str | None'}, total=False)
class PerpsCancelRejected(_PerpsCancelRejectedOptional):
    status: str
    error: str
    ts: int | Decimal

_PerpsCancelRequestOptional = TypedDict("_PerpsCancelRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'exp': 'int | Decimal | None'}, total=False)
class PerpsCancelRequest(_PerpsCancelRequestOptional):
    op: PerpsOpCancelOrders

_PerpsCompetitionAccountOptional = TypedDict("_PerpsCompetitionAccountOptional", {'updated_at': 'int | Decimal | None', 'rank': 'int | Decimal | None', 'first_fill_at': 'int | Decimal | None', 'prize': 'str | None', 'spots_from_prize_zone': 'int | Decimal | None', 'score_to_prize_zone': 'str | None', 'payout_tx_hash': 'str | None', 'paid_at': 'int | Decimal | None'}, total=False)
class PerpsCompetitionAccount(_PerpsCompetitionAccountOptional):
    competition_id: int | Decimal
    status: str
    account: str
    qualified: bool
    score: str
    pnl: str
    realized_pnl: str
    unrealized_pnl: str
    fees: str
    funding: str
    volume: str
    boosted_volume: str
    min_volume: str
    volume_to_qualify: str
    qualification_progress: str
    row_state: str
    locked_positions: list[PerpsCompetitionLockedPosition]

_PerpsCompetitionDetailOptional = TypedDict("_PerpsCompetitionDetailOptional", {'start_trading_instrument_id': 'int | Decimal | None', 'updated_at': 'int | Decimal | None', 'prize_cutoff_score': 'str | None'}, total=False)
class PerpsCompetitionDetail(_PerpsCompetitionDetailOptional):
    competition_id: int | Decimal
    number: int | Decimal
    name: str
    sector_label: str
    boost_description: str
    status: str
    start_time: int | Decimal
    end_time: int | Decimal
    all_markets: bool
    markets: list[PerpsCompetitionMarket]
    min_volume: str
    count_unrealized: bool
    prize_pool: str
    prize_asset_id: int | Decimal
    prize_tiers: list[PerpsCompetitionPrizeTier]
    paid_places: int | Decimal
    internal: bool
    ranked: int | Decimal
    participants: int | Decimal
    final_results: bool
    scoring_version: int | Decimal

_PerpsCompetitionEntryOptional = TypedDict("_PerpsCompetitionEntryOptional", {'rank': 'int | Decimal | None', 'first_fill_at': 'int | Decimal | None', 'prize': 'str | None', 'payout_tx_hash': 'str | None', 'paid_at': 'int | Decimal | None'}, total=False)
class PerpsCompetitionEntry(_PerpsCompetitionEntryOptional):
    account: str
    score: str
    pnl: str
    realized_pnl: str
    unrealized_pnl: str
    volume: str
    boosted_volume: str
    row_state: str

_PerpsCompetitionLeaderboardOptional = TypedDict("_PerpsCompetitionLeaderboardOptional", {'updated_at': 'int | Decimal | None'}, total=False)
class PerpsCompetitionLeaderboard(_PerpsCompetitionLeaderboardOptional):
    competition_id: int | Decimal
    status: str
    total: int | Decimal
    offset: int | Decimal
    final_results: bool
    entries: list[PerpsCompetitionEntry]

_PerpsCompetitionListOptional = TypedDict("_PerpsCompetitionListOptional", {'live_competition_id': 'int | Decimal | None', 'landing_competition_id': 'int | Decimal | None'}, total=False)
class PerpsCompetitionList(_PerpsCompetitionListOptional):
    timestamp: int | Decimal
    competitions: list[PerpsCompetitionSummary]

_PerpsCompetitionLockedPositionOptional = TypedDict("_PerpsCompetitionLockedPositionOptional", {'released_at': 'int | Decimal | None'}, total=False)
class PerpsCompetitionLockedPosition(_PerpsCompetitionLockedPositionOptional):
    instrument_id: int | Decimal
    size_at_start: str

class PerpsCompetitionMarket(TypedDict):
    instrument_id: int | Decimal
    boost: str

class PerpsCompetitionPrizeTier(TypedDict):
    from_rank: int | Decimal
    to_rank: int | Decimal
    amount: str

_PerpsCompetitionReadError400Optional = TypedDict("_PerpsCompetitionReadError400Optional", {'arts': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'ref': 'str | None'}, total=False)
class PerpsCompetitionReadError400(_PerpsCompetitionReadError400Optional):
    status: str
    error: str

class PerpsCompetitionReadError404(TypedDict):
    status: str
    error: str

class PerpsCompetitionReadError429(TypedDict):
    status: str
    error: str

class PerpsCompetitionReadError500(TypedDict):
    status: str
    error: str

class PerpsCompetitionReadError503(TypedDict):
    status: str
    error: str

class PerpsCompetitionSummary(TypedDict):
    competition_id: int | Decimal
    number: int | Decimal
    name: str
    sector_label: str
    status: str
    start_time: int | Decimal
    end_time: int | Decimal
    prize_pool: str
    prize_asset_id: int | Decimal
    internal: bool

_PerpsCompetitionWidgetOptional = TypedDict("_PerpsCompetitionWidgetOptional", {'competition_id': 'int | Decimal | None', 'number': 'int | Decimal | None', 'end_time': 'int | Decimal | None', 'rank': 'int | Decimal | None', 'qualified': 'bool | None', 'volume': 'str | None', 'min_volume': 'str | None', 'qualification_progress': 'str | None', 'spots_from_prize_zone': 'int | Decimal | None', 'updated_at': 'int | Decimal | None'}, total=False)
class PerpsCompetitionWidget(_PerpsCompetitionWidgetOptional):
    timestamp: int | Decimal

class PerpsCreateInviteRequest(TypedDict):
    pass

class PerpsCreateInviteResponse(TypedDict):
    status: str
    code: str
    referrals_available: int | Decimal
    cooldown_ms: int | Decimal | None

_PerpsCreateOrderOptional = TypedDict("_PerpsCreateOrderOptional", {'p': 'str | None', 'tif': 'str | None', 'po': 'bool | None', 'ro': 'bool | None', 'c': 'str | None', 'tr': 'PerpsCreateOrderTr | None', 'builder': 'PerpsBuilder | None'}, total=False)
class PerpsCreateOrder(_PerpsCreateOrderOptional):
    iid: int | Decimal
    buy: bool
    qty: str

_PerpsCreateOrderTrOptional = TypedDict("_PerpsCreateOrderTrOptional", {'market': 'bool | None', 'trp': 'str | None', 'tpsl': 'str | None'}, total=False)
class PerpsCreateOrderTr(_PerpsCreateOrderTrOptional):
    pass

class PerpsCredentials(TypedDict):
    address: str
    keys: list[PerpsKey]

_PerpsDeleteProxyRequestOptional = TypedDict("_PerpsDeleteProxyRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None'}, total=False)
class PerpsDeleteProxyRequest(_PerpsDeleteProxyRequestOptional):
    op: PerpsOpDeleteProxy

_PerpsDepositOptional = TypedDict("_PerpsDepositOptional", {'confirmed_timestamp': 'int | Decimal | None'}, total=False)
_PerpsDepositWireKeys = TypedDict("_PerpsDepositWireKeys", {'from': 'str'})
class PerpsDeposit(_PerpsDepositWireKeys, _PerpsDepositOptional):
    hash: str
    asset: str
    amount: str
    to: str
    status: str
    confirmations: int | Decimal
    required_confirmations: int | Decimal
    created_timestamp: int | Decimal

class PerpsDeposits(TypedDict):
    data: list[PerpsDeposit]
    more: bool

class PerpsEquityHistory(TypedDict):
    data: list[list[Any]]
    more: bool

class PerpsError400(TypedDict):
    status: str
    error: str

class PerpsError401(TypedDict):
    status: str
    error: str

class PerpsError404(TypedDict):
    status: str
    error: str

class PerpsError408(TypedDict):
    status: str
    error: str

class PerpsError413(TypedDict):
    status: str
    error: str

class PerpsError429(TypedDict):
    status: str
    error: str

class PerpsError500(TypedDict):
    status: str
    error: str

class PerpsError503(TypedDict):
    status: str
    error: str

class PerpsExchange(TypedDict):
    name: str
    version: str
    chain_id: int | Decimal
    contract: str
    cancel_only: bool
    engine_version: str

class PerpsExchangeStatistics(TypedDict):
    start_timestamp: int | Decimal
    end_timestamp: int | Decimal
    volume: str
    open_interest: str | None
    open_interest_timestamp: int | Decimal | None
    fees: str

class PerpsFeeScheduleEntry(TypedDict):
    instrument_type: str
    category: str
    taker_fee_rate: str
    maker_fee_rate: str
    tiers: list[PerpsFeeTier]

class PerpsFeeTier(TypedDict):
    min_volume_30d: str
    taker_fee_rate: str
    maker_fee_rate: str

class PerpsFeesInfo(TypedDict):
    fee_schedule: list[PerpsFeeScheduleEntry]

class PerpsFundingHistory(TypedDict):
    data: list[PerpsFundingRate]
    more: bool

class PerpsFundingRate(TypedDict):
    funding_rate: str
    timestamp: int | Decimal

class PerpsGenericAccepted(TypedDict):
    status: str

class PerpsGenericRejected(TypedDict):
    status: str
    error: str

class PerpsIndex(TypedDict):
    asset: str
    index_price: str
    constituents: list[PerpsIndexConstituent]
    ts: int | Decimal

class PerpsIndexConstituent(TypedDict):
    source: str
    symbol: str
    weight: str
    price: str

_PerpsInstrumentOptional = TypedDict("_PerpsInstrumentOptional", {'display_symbol': 'str | None', 'settlement': 'PerpsInstrumentSettlement | None'}, total=False)
class PerpsInstrument(_PerpsInstrumentOptional):
    instrument_id: int | Decimal
    instrument_type: str
    category: str
    isolated_only: bool
    symbol: str
    base_asset: str
    quote_asset: str
    funding_interval: str
    quantity_decimals: int | Decimal
    price_decimals: int | Decimal
    price_bounds: str
    liquidation_fee: str
    max_order_count: int | Decimal
    min_notional: str
    max_market_notional: str
    max_limit_notional: str
    max_leverage: int | Decimal
    risk_tiers: list[PerpsRiskTier]
    ui_live_time: int | Decimal | None
    close_only: bool

class PerpsInstrumentSettlement(TypedDict):
    sequence: int | Decimal
    timestamp: int | Decimal
    price: str
    insurance_debit: str

_PerpsInternalTransferOptional = TypedDict("_PerpsInternalTransferOptional", {'label': 'str | None'}, total=False)
class PerpsInternalTransfer(_PerpsInternalTransferOptional):
    transfer_id: int | Decimal
    asset: str
    amount: str
    direction: str
    counterparty: str
    created_timestamp: int | Decimal

class PerpsInternalTransferAccepted(TypedDict):
    status: str
    transfer_id: int | Decimal

class PerpsInternalTransferRejected(TypedDict):
    status: str
    transfer_id: int | Decimal
    error: str

_PerpsInternalTransferRequestOptional = TypedDict("_PerpsInternalTransferRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'label': 'str | None'}, total=False)
class PerpsInternalTransferRequest(_PerpsInternalTransferRequestOptional):
    op: PerpsOpInternalTransfer

class PerpsInternalTransfers(TypedDict):
    data: list[PerpsInternalTransfer]
    more: bool

_PerpsInviteCheckResponseOptional = TypedDict("_PerpsInviteCheckResponseOptional", {'error': 'str | None'}, total=False)
class PerpsInviteCheckResponse(_PerpsInviteCheckResponseOptional):
    valid: bool

_PerpsKeyOptional = TypedDict("_PerpsKeyOptional", {'label': 'str | None'}, total=False)
class PerpsKey(_PerpsKeyOptional):
    proxy: str
    expiry: int | Decimal

class PerpsKlinesResponse(TypedDict):
    data: list[PerpsKline]
    more: bool

_PerpsLeaderboardOptional = TypedDict("_PerpsLeaderboardOptional", {'account': 'PerpsLeaderboardAccount | None'}, total=False)
class PerpsLeaderboard(_PerpsLeaderboardOptional):
    window: str
    sort_by: str
    timestamp: int | Decimal
    total: int | Decimal
    entries: list[PerpsLeaderboardEntry]

_PerpsLeaderboardAccountOptional = TypedDict("_PerpsLeaderboardAccountOptional", {'rank': 'int | Decimal | None'}, total=False)
class PerpsLeaderboardAccount(_PerpsLeaderboardAccountOptional):
    account: str
    pnl: str
    notional: str
    account_value: str
    roi: str

class PerpsLeaderboardEntry(TypedDict):
    rank: int | Decimal
    account: str
    pnl: str
    notional: str
    account_value: str
    roi: str

_PerpsLeverageBatchRequestOptional = TypedDict("_PerpsLeverageBatchRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None'}, total=False)
class PerpsLeverageBatchRequest(_PerpsLeverageBatchRequestOptional):
    op: PerpsOpUpdateLeverages

class PerpsLeverageRejected(TypedDict):
    status: str
    instrument_id: int | Decimal
    error: str

_PerpsLeverageRequestOptional = TypedDict("_PerpsLeverageRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None'}, total=False)
class PerpsLeverageRequest(_PerpsLeverageRequestOptional):
    op: PerpsOpUpdateLeverage

class PerpsLeverageResponse(TypedDict):
    status: str
    instrument_id: int | Decimal
    leverage: int | Decimal
    cross: bool

class PerpsLimitTier(TypedDict):
    min_volume_14d: str
    rate_per_minute_limit: int | Decimal
    rate_burst_limit: int | Decimal
    actions_per_minute_limit: int | Decimal
    actions_burst_limit: int | Decimal
    open_orders_limit: int | Decimal
    messages_per_minute: int | Decimal

_PerpsMarginRequestOptional = TypedDict("_PerpsMarginRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None'}, total=False)
class PerpsMarginRequest(_PerpsMarginRequestOptional):
    op: PerpsOpUpdateMargin

class PerpsMarginSummary(TypedDict):
    total_account_value: str
    available_order_margin: str
    total_initial_margin: str
    total_maintenance_margin: str
    total_position_value: str

class PerpsMarkHistoryResponse(TypedDict):
    data: list[PerpsMarkPoint]
    more: bool

class PerpsModifyAccepted(TypedDict):
    status: str
    order: PerpsOrder

_PerpsModifyByCoidRequestOptional = TypedDict("_PerpsModifyByCoidRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'exp': 'int | Decimal | None'}, total=False)
class PerpsModifyByCoidRequest(_PerpsModifyByCoidRequestOptional):
    op: PerpsOpModifyOrdersByCoid

class PerpsModifyOrder(TypedDict):
    oid: int | Decimal
    p: str
    qty: str

class PerpsModifyOrderByCoid(TypedDict):
    coid: str
    p: str
    qty: str

_PerpsModifyRejectedOptional = TypedDict("_PerpsModifyRejectedOptional", {'oid': 'int | Decimal | None', 'coid': 'str | None'}, total=False)
class PerpsModifyRejected(_PerpsModifyRejectedOptional):
    status: str
    error: str

_PerpsModifyRequestOptional = TypedDict("_PerpsModifyRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'exp': 'int | Decimal | None'}, total=False)
class PerpsModifyRequest(_PerpsModifyRequestOptional):
    op: PerpsOpModifyOrders

class PerpsNotificationItem(TypedDict):
    notification: dict[str, Any]
    read_at: int | Decimal | None
    ts: int | Decimal

class PerpsNotificationsResponse(TypedDict):
    items: list[PerpsNotificationItem]
    unread: int | Decimal
    durable_source_seq: int | Decimal
    has_more: bool
    next_cursor: str | None

class PerpsOpApproveBuilder(TypedDict):
    type: str
    args: PerpsOpApproveBuilderArgs

class PerpsOpApproveBuilderArgs(TypedDict):
    builder: str
    max_fee_rate: str
    approval_version: int | Decimal

class PerpsOpAutoCancel(TypedDict):
    type: str
    args: PerpsOpAutoCancelArgs

class PerpsOpAutoCancelArgs(TypedDict):
    time: int | Decimal

class PerpsOpCancelAll(TypedDict):
    type: str
    args: PerpsOpCancelAllArgs

_PerpsOpCancelAllArgsOptional = TypedDict("_PerpsOpCancelAllArgsOptional", {'iid': 'int | Decimal | None'}, total=False)
class PerpsOpCancelAllArgs(_PerpsOpCancelAllArgsOptional):
    pass

class PerpsOpCancelOrders(TypedDict):
    type: str
    args: list[int | Decimal]

class PerpsOpCancelOrdersByCoid(TypedDict):
    type: str
    args: list[str]

_PerpsOpCreateOrdersOptional = TypedDict("_PerpsOpCreateOrdersOptional", {'grp': 'str | None'}, total=False)
class PerpsOpCreateOrders(_PerpsOpCreateOrdersOptional):
    type: str
    args: list[PerpsCreateOrder]

class PerpsOpCreateProxy(TypedDict):
    type: str
    args: PerpsOpCreateProxyArgs

class PerpsOpCreateProxyArgs(TypedDict):
    owner: str
    proxy: str
    expiry: int | Decimal

class PerpsOpDeleteProxy(TypedDict):
    type: str
    args: PerpsOpDeleteProxyArgs

class PerpsOpDeleteProxyArgs(TypedDict):
    proxy: str

class PerpsOpInternalTransfer(TypedDict):
    type: str
    args: PerpsOpInternalTransferArgs

class PerpsOpInternalTransferArgs(TypedDict):
    account: str
    token: str
    amount: str
    to: str

class PerpsOpModifyOrders(TypedDict):
    type: str
    args: list[PerpsModifyOrder]

class PerpsOpModifyOrdersByCoid(TypedDict):
    type: str
    args: list[PerpsModifyOrderByCoid]

class PerpsOpUpdateLeverage(TypedDict):
    type: str
    args: PerpsOpUpdateLeverageArgs

class PerpsOpUpdateLeverageArgs(TypedDict):
    iid: int | Decimal
    lev: int | Decimal
    cross: bool

class PerpsOpUpdateLeverages(TypedDict):
    type: str
    args: list[PerpsOpUpdateLeveragesArgsItem]

class PerpsOpUpdateLeveragesArgsItem(TypedDict):
    iid: int | Decimal
    lev: int | Decimal
    cross: bool

class PerpsOpUpdateMargin(TypedDict):
    type: str
    args: PerpsOpUpdateMarginArgs

class PerpsOpUpdateMarginArgs(TypedDict):
    iid: int | Decimal
    amt: str

class PerpsOpWithdraw(TypedDict):
    type: str
    args: PerpsOpWithdrawArgs

class PerpsOpWithdrawArgs(TypedDict):
    account: str
    token: str
    amount: str
    to: str

_PerpsOrderOptional = TypedDict("_PerpsOrderOptional", {'coid': 'str | None', 'builder': 'PerpsBuilder | None'}, total=False)
class PerpsOrder(_PerpsOrderOptional):
    oid: int | Decimal
    iid: int | Decimal
    buy: bool
    p: str
    qty: str
    tif: str
    po: bool
    ro: bool
    rest: str
    fill: str
    cts: int | Decimal
    uts: int | Decimal
    status: str

_PerpsOrderAcceptedOptional = TypedDict("_PerpsOrderAcceptedOptional", {'coid': 'str | None', 'builder': 'PerpsBuilder | None'}, total=False)
class PerpsOrderAccepted(_PerpsOrderAcceptedOptional):
    status: str
    oid: int | Decimal

_PerpsOrderDataOptional = TypedDict("_PerpsOrderDataOptional", {'client_order_id': 'str | None', 'tpsl': 'PerpsTpSlOrderFields | None', 'builder': 'PerpsBuilder | None'}, total=False)
class PerpsOrderData(_PerpsOrderDataOptional):
    order_id: int | Decimal
    instrument_id: int | Decimal
    buy: bool
    price: str
    quantity: str
    tif: str
    post_only: bool
    ro: bool
    resting_quantity: str
    filled_quantity: str
    status: str
    created_timestamp: int | Decimal
    updated_timestamp: int | Decimal

_PerpsOrderRejectedOptional = TypedDict("_PerpsOrderRejectedOptional", {'oid': 'int | Decimal | None', 'coid': 'str | None'}, total=False)
class PerpsOrderRejected(_PerpsOrderRejectedOptional):
    status: str
    error: str

_PerpsOrderRequestOptional = TypedDict("_PerpsOrderRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'exp': 'int | Decimal | None'}, total=False)
class PerpsOrderRequest(_PerpsOrderRequestOptional):
    op: PerpsOpCreateOrders

class PerpsPnlHistory(TypedDict):
    data: list[list[Any]]
    more: bool

class PerpsPortfolio(TypedDict):
    positions: list[PerpsPortfolioPosition]
    margin: PerpsMarginSummary
    withdrawable: str
    in_liquidation: bool
    fee_tier: int | Decimal
    timestamp: int | Decimal

class PerpsPortfolioPosition(TypedDict):
    instrument_id: int | Decimal
    symbol: str
    size: str
    entry_price: str
    leverage: int | Decimal
    cross: bool
    initial_margin: str
    maintenance_margin: str
    position_value: str
    liquidation_price: str
    unrealized_pnl: str
    return_on_equity: str
    cumulative_funding: str

class PerpsPositionCardSnapshot(TypedDict):
    position_cycle_id: str
    side: str
    started_at: int | Decimal
    as_of_at: int | Decimal
    is_closed: bool
    size_after: str
    entry_price: str
    as_of_price: str
    pnl: str
    pnl_percent: str | None
    leverage: int | Decimal | None
    chart: PerpsPositionSnapshotChart

class PerpsPositionSnapshotCandle(TypedDict):
    position: int | Decimal
    open: int | Decimal
    high: int | Decimal
    low: int | Decimal
    close: int | Decimal

class PerpsPositionSnapshotChart(TypedDict):
    candles: list[PerpsPositionSnapshotCandle]
    markers: list[PerpsPositionSnapshotMarker]

class PerpsPositionSnapshotFill(TypedDict):
    instrument_id: int | Decimal
    trade_id: str
    timestamp: int | Decimal

class PerpsPositionSnapshotMarker(TypedDict):
    position: int | Decimal
    kind: str

_PerpsPositionSnapshotResultOptional = TypedDict("_PerpsPositionSnapshotResultOptional", {'instrument_id': 'int | Decimal | None', 'trade_id': 'str | None', 'snapshot': 'PerpsPositionCardSnapshot | None'}, total=False)
class PerpsPositionSnapshotResult(_PerpsPositionSnapshotResultOptional):
    status: str

class PerpsPositionSnapshots(TypedDict):
    history_as_of_at: int | Decimal
    active: list[PerpsPositionSnapshotResult]
    history: list[PerpsPositionSnapshotResult]

_PerpsPositionSnapshotsRequestOptional = TypedDict("_PerpsPositionSnapshotsRequestOptional", {'active_instrument_ids': 'list[int | Decimal] | None', 'history_fills': 'list[PerpsPositionSnapshotFill] | None'}, total=False)
class PerpsPositionSnapshotsRequest(_PerpsPositionSnapshotsRequestOptional):
    address: str

_PerpsProxyRequestOptional = TypedDict("_PerpsProxyRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None', 'label': 'str | None', 'code': 'str | None'}, total=False)
class PerpsProxyRequest(_PerpsProxyRequestOptional):
    op: PerpsOpCreateProxy

class PerpsProxyResponse(TypedDict):
    status: str
    secret: str

class PerpsPublicPortfolio(TypedDict):
    positions: list[PerpsPublicPortfolioPosition]
    equity: str
    timestamp: int | Decimal

class PerpsPublicPortfolioPosition(TypedDict):
    instrument_id: int | Decimal
    symbol: str
    size: str
    entry_price: str
    unrealized_pnl: str
    return_on_equity: str

class PerpsRegisteredResponse(TypedDict):
    registered: bool

class PerpsRiskTier(TypedDict):
    lower_bound: str
    max_leverage: int | Decimal

class PerpsStatistic(TypedDict):
    instrument_id: int | Decimal
    symbol: str
    volume: str
    open_price: str
    klines: PerpsKlines

class PerpsTicker(TypedDict):
    instrument_id: int | Decimal
    symbol: str
    index_price: str
    mark_price: str
    last_price: str
    mid_price: str
    open_interest: str
    funding_rate: str
    next_funding: int | Decimal
    timestamp: int | Decimal

class PerpsTickerData(TypedDict):
    instrument_id: int | Decimal
    symbol: str
    index_price: str
    mark_price: str
    last_price: str
    mid_price: str
    open_interest: str
    funding_rate: str
    next_funding: int | Decimal

class PerpsTime(TypedDict):
    time: int | Decimal

_PerpsTpSlOrderFieldsOptional = TypedDict("_PerpsTpSlOrderFieldsOptional", {'parent_oid': 'int | Decimal | None', 'armed_qty': 'str | None', 'slip_bps': 'int | Decimal | None'}, total=False)
class PerpsTpSlOrderFields(_PerpsTpSlOrderFieldsOptional):
    kind: str
    scope: str
    trp: str

class PerpsTradeData(TypedDict):
    trade_id: int | Decimal
    instrument_id: int | Decimal
    side: str
    price: str
    quantity: str
    timestamp: int | Decimal
    hash: str
    settlement: bool

class PerpsTrades(TypedDict):
    data: list[PerpsTradeData]
    more: bool

class PerpsWithdrawAccepted(TypedDict):
    status: str
    withdraw_id: int | Decimal

class PerpsWithdrawRejected(TypedDict):
    status: str
    withdraw_id: int | Decimal
    error: str

_PerpsWithdrawRequestOptional = TypedDict("_PerpsWithdrawRequestOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None'}, total=False)
class PerpsWithdrawRequest(_PerpsWithdrawRequestOptional):
    op: PerpsOpWithdraw

_PerpsWithdrawalOptional = TypedDict("_PerpsWithdrawalOptional", {'confirmed_timestamp': 'int | Decimal | None'}, total=False)
class PerpsWithdrawal(_PerpsWithdrawalOptional):
    withdraw_id: int | Decimal
    asset: str
    amount: str
    to: str
    fee: str
    status: str
    hash: str
    confirmations: int | Decimal
    required_confirmations: int | Decimal
    created_timestamp: int | Decimal

class PerpsWithdrawals(TypedDict):
    data: list[PerpsWithdrawal]
    more: bool

class PostBlpEnrollParams(TypedDict):
    body: PerpsBlpEnrollRequest

class PostBlpEnrollResponse(TypedDict):
    status: str

_SetAutoCancelParamsOptional = TypedDict("_SetAutoCancelParamsOptional", {'body': 'PerpsAutoCancelRequest | None'}, total=False)
class SetAutoCancelParams(_SetAutoCancelParamsOptional):
    pass

class SetAutoCancelResponse(TypedDict):
    status: str
    deadline: int | Decimal

class UpdateBuilderParams(TypedDict):
    body: UpdateBuilderParamsBody

_UpdateBuilderParamsBodyOptional = TypedDict("_UpdateBuilderParamsBodyOptional", {'sig': 'str | None', 'salt': 'int | Decimal | None', 'ts': 'int | Decimal | None'}, total=False)
class UpdateBuilderParamsBody(_UpdateBuilderParamsBodyOptional):
    op: UpdateBuilderParamsBodyOp

class UpdateBuilderParamsBodyOp(TypedDict):
    type: str
    args: UpdateBuilderParamsBodyOpArgs

class UpdateBuilderParamsBodyOpArgs(TypedDict):
    name: str

class UpdateBuilderResponse(TypedDict):
    address: str
    registered: bool
    enabled: bool
    admission_enabled: bool
    max_fee_rate: str
    name: str | None

_UpdateLeverageParamsOptional = TypedDict("_UpdateLeverageParamsOptional", {'body': 'PerpsLeverageRequest | None'}, total=False)
class UpdateLeverageParams(_UpdateLeverageParamsOptional):
    pass

class UpdateLeverageResponse(TypedDict):
    status: str
    instrument_id: int | Decimal
    leverage: int | Decimal
    cross: bool

_UpdateLeveragesParamsOptional = TypedDict("_UpdateLeveragesParamsOptional", {'body': 'PerpsLeverageBatchRequest | None'}, total=False)
class UpdateLeveragesParams(_UpdateLeveragesParamsOptional):
    pass

_UpdateMarginParamsOptional = TypedDict("_UpdateMarginParamsOptional", {'body': 'PerpsMarginRequest | None'}, total=False)
class UpdateMarginParams(_UpdateMarginParamsOptional):
    pass

_WithdrawParamsOptional = TypedDict("_WithdrawParamsOptional", {'body': 'PerpsWithdrawRequest | None'}, total=False)
class WithdrawParams(_WithdrawParamsOptional):
    pass

class WithdrawResponse(TypedDict):
    status: str
    withdraw_id: int | Decimal

CancelChaseResponse = Any
PerpsCancelResponse = Any
CancelTwapResponse = Any
ControlTwapResponse = Any
CreateChaseResponse = Any
PerpsOrderResponse = Any
CreateTwapResponse = Any
GetAssetsResponse = list[PerpsAsset]
GetBalancesResponse = list[PerpsBalance]
GetBboResponse = list[PerpsBBO]
GetChasesResponse = Any
GetConfigResponse = list[PerpsAccountConfig]
GetInstrumentsResponse = list[PerpsInstrument]
GetLimitTiersResponse = list[PerpsLimitTier]
GetOpenOrdersResponse = list[PerpsOrderData]
GetOrdersResponse = list[PerpsOrderData]
GetStatisticsResponse = list[PerpsStatistic]
GetTickersResponse = list[PerpsTicker]
GetTwapsResponse = Any
PerpsModifyResponse = Any
PerpsAccount = str
PerpsAccountBackstopsCursor = str
PerpsAccountMakerShare7d = str
PerpsAccountOi = str
PerpsAccountValue = str
PerpsActionsBurst = int | Decimal
PerpsActionsBurstLimit = int | Decimal
PerpsActionsPerMinute = int | Decimal
PerpsActionsPerMinuteLimit = int | Decimal
PerpsActiveChildrenCount = int | Decimal
PerpsAddress = str
PerpsAdl = bool
PerpsAmount = str
PerpsArts = int | Decimal
PerpsAssetField = str
PerpsAutoCancelDailyLimit = int | Decimal
PerpsAutoCancelDeadline = int | Decimal
PerpsAutoCancelNextReset = int | Decimal
PerpsAutoCancelTriggered = int | Decimal
PerpsAvailableOrderMargin = str
PerpsBackstopCashAfter = str
PerpsBackstopCashBefore = str
PerpsBackstopTransferred = str
PerpsBalanceField = str
PerpsBaseAsset = str
PerpsBestAskPrice = str
PerpsBestAskQuantity = str
PerpsBestBidPrice = str
PerpsBestBidQuantity = str
PerpsBlp = bool
PerpsBlpActive = bool
PerpsBlpCursor = int | Decimal
PerpsBlpSort = str
PerpsBuilderApprovalVersion = int | Decimal
PerpsBuilderFee = str
PerpsBuilderFeeRate = str
PerpsBuy = bool
PerpsCancelOnly = bool
PerpsCancelTs = int | Decimal
PerpsCategory = str
PerpsChainId = int | Decimal
PerpsChildrenCount = int | Decimal
PerpsChildrenFees = str
PerpsChildrenVolume = str
PerpsCloseOnly = bool
PerpsCode = str
PerpsCoid = str
PerpsCollateralRatio = str
PerpsCompetitionFirstFillAt = int | Decimal
PerpsCompetitionId = int | Decimal
PerpsCompetitionReadError = str
PerpsCompetitionReadTimestamp = int | Decimal
PerpsCompetitionRowState = str
PerpsCompetitionScore = str
PerpsCompetitionStatus = str
PerpsConfirmations = int | Decimal
PerpsConfirmedTimestamp = int | Decimal
PerpsContract = str
PerpsCounterparty = str
PerpsCreatedTimestamp = int | Decimal
PerpsCross = bool
PerpsCts = int | Decimal
PerpsCumulativeFunding = str
PerpsCursor = str
PerpsDate = str
PerpsDecimals = int | Decimal
PerpsDepositStatus = str
PerpsDepth = int | Decimal
PerpsDirection = str
PerpsDisplaySymbol = str
PerpsDurableSourceSeq = int | Decimal
PerpsEndDate = str
PerpsEndTimestamp = int | Decimal
PerpsEngineVersion = str
PerpsEntityId = int | Decimal
PerpsEntityMakerShare7d = str
PerpsEntityName = str
PerpsEntryPrice = str
PerpsEquity = str
PerpsError = str
PerpsExchangeFees = str
PerpsExchangeOpenInterest = str
PerpsExchangeVolume = str
PerpsExp = int | Decimal
PerpsExpiry = int | Decimal
PerpsFea = str
PerpsFee = str
PerpsFeeShareRate = str
PerpsFeeTierField = int | Decimal
PerpsFid = int | Decimal
PerpsFill = str
PerpsFillsCursor = str
PerpsFr = str
PerpsFrom = str
PerpsFua = str
PerpsFund = str
PerpsFundingInterval = str
PerpsFundingRateField = str
PerpsGenericResponse = Any
PerpsGrp = str
PerpsHash = str
PerpsHistorySequence = int | Decimal
PerpsIid = int | Decimal
PerpsInLiquidation = bool
PerpsIndexPrice = str
PerpsInitialMargin = str
PerpsInstrumentId = int | Decimal
PerpsInstrumentType = str
PerpsInterval = str
PerpsIsolatedOnly = bool
PerpsKind = str
PerpsKline = list[Any]
PerpsLabel = str
PerpsLastPrice = str
PerpsLev = int | Decimal
PerpsLevel = list[str]
PerpsLeverage = int | Decimal
PerpsLeverageBatchResponse = Any
PerpsLid = int | Decimal
PerpsLimit = int | Decimal
PerpsLiq = bool
PerpsLiquidationFee = str
PerpsLiquidationPrice = str
PerpsLowerBound = str
PerpsMaintenanceMarginAmount = str
PerpsMakerFeeRate = str
PerpsMakerShare7d = str
PerpsMakerVolume7d = str
PerpsMarginAmount = str
PerpsMarkPoint = list[Any]
PerpsMarkPrice = str
PerpsMarket = bool
PerpsMaxLeverage = int | Decimal
PerpsMaxLimitNotional = str
PerpsMaxMarketNotional = str
PerpsMaxOrderCount = int | Decimal
PerpsMessagesPerMinute = int | Decimal
PerpsMgn = str
PerpsMidPrice = str
PerpsMinNotional = str
PerpsMinVolume = str
PerpsMinVolume30d = str
PerpsMore = bool
PerpsName = str
PerpsNextFunding = int | Decimal
PerpsNotifCursor = str
PerpsNotifId = str
PerpsNotificationsReadRequest = Any
PerpsNotional = str
PerpsOffset = int | Decimal
PerpsOid = int | Decimal
PerpsOpAmount = str
PerpsOpenInterest = str
PerpsOpenInterestTimestamp = int | Decimal
PerpsOpenOrders = int | Decimal
PerpsOpenOrdersLimit = int | Decimal
PerpsOpenPrice = str
PerpsOrderId = int | Decimal
PerpsOwner = str
PerpsP = str
PerpsParentOid = int | Decimal
PerpsPep = str
PerpsPnl = str
PerpsPnlInterval = str
PerpsPo = bool
PerpsPositionSnapshotStatus = str
PerpsPositionValue = str
PerpsPrice = str
PerpsPriceBounds = str
PerpsPriceDecimals = int | Decimal
PerpsProxy = str
PerpsPsz = str
PerpsQty = str
PerpsQuantity = str
PerpsQuantityDecimals = int | Decimal
PerpsQuoteAsset = str
PerpsRank = int | Decimal
PerpsRateBurstLimit = int | Decimal
PerpsRatePerMinuteLimit = int | Decimal
PerpsRateReset = int | Decimal
PerpsReadAt = int | Decimal
PerpsRef = str
PerpsReferralsLeft = int | Decimal
PerpsRequiredConfirmations = int | Decimal
PerpsRest = str
PerpsReturnOnEquity = str
PerpsRewardAmount = str
PerpsRewardDistributed = bool
PerpsRewardPool = str
PerpsRo = bool
PerpsRoi = str
PerpsSalt = int | Decimal
PerpsSecret = str
PerpsSequence = int | Decimal
PerpsSettlementFill = bool
PerpsSettlementInsuranceDebit = str
PerpsSettlementPrice = str
PerpsSettlementSequence = int | Decimal
PerpsSettlementTimestamp = int | Decimal
PerpsSide = str
PerpsSig = str
PerpsSinceSeq = int | Decimal
PerpsSize = str
PerpsSlipBps = int | Decimal
PerpsSort = str
PerpsSortBy = str
PerpsSource = str
PerpsSt = str
PerpsStartDate = str
PerpsStartTimestamp = int | Decimal
PerpsSymbol = str
PerpsSz = str
PerpsTaker = bool
PerpsTakerFeeRate = str
PerpsTakerVolume7d = str
PerpsTid = int | Decimal
PerpsTif = str
PerpsTimeField = int | Decimal
PerpsTimestamp = int | Decimal
PerpsTo = str
PerpsToken = str
PerpsTotal = int | Decimal
PerpsTotalAccountValue = str
PerpsTotalFee = str
PerpsTotalInitialMargin = str
PerpsTotalMaintenanceMargin = str
PerpsTotalPositionValue = str
PerpsTpsl = str
PerpsTrScope = str
PerpsTransferId = int | Decimal
PerpsTrp = str
PerpsTs = int | Decimal
PerpsUiLiveTime = int | Decimal
PerpsUnread = int | Decimal
PerpsUnrealizedPnl = str
PerpsUpdateTimestamp = int | Decimal
PerpsUts = int | Decimal
PerpsValue = str
PerpsVersion = str
PerpsVolume = str
PerpsVolume7d = str
PerpsWeight = str
PerpsWindow = str
PerpsWithdrawId = int | Decimal
PerpsWithdrawable = str
PerpsWithdrawalFee = str
PerpsWithdrawalStatus = str
UpdateMarginResponse = Any
CancelOrdersByCoidResponse = list[PerpsCancelResponse]
CancelOrdersResponse = list[PerpsCancelResponse]
CreateOrdersResponse = list[PerpsOrderResponse]
ModifyOrdersByCoidResponse = list[PerpsModifyResponse]
ModifyOrdersResponse = list[PerpsModifyResponse]
PerpsKlines = list[PerpsKline]
UpdateLeveragesResponse = list[PerpsLeverageBatchResponse]
