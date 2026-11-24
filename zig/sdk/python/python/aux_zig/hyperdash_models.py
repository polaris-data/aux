"""Hyperdash REST wire models generated from src/data/hyperdash/schema/hyperdash.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

from typing import Literal
CreateJobRequestBodyKind = Literal["account_fills_summary"]
GetAccountFillsRequestAggregate = Literal["false", "true"]
GetAccountLedgerRequestInclude = Literal["staking", "all"]
GetAccountOpenOrdersRequestAllDexes = Literal["false", "true"]
GetAccountPnlChartRequestWindow = Literal["day", "week", "month", "alltime"]
GetAccountPortfolioRequestAllDexes = Literal["false", "true"]
GetAccountRealizedPnlRequestInterval = Literal["5m", "1h", "1d"]
GetAccountRealizedPnlRequestAnchor = Literal["false", "true"]
GetAccountsLiquidationRiskRequestKind = Literal["cross", "coin_margined"]
GetAccountsLiquidationRiskRequestSort = Literal["risk", "equity", "notional", "leverage"]
GetActiveTradersRequestInterval = Literal["1h", "1d"]
GetActiveTwapPressureRequestMinVolume = Literal["0", "1000000", "10000000"]
GetAloPriorityBarsRequestInterval = Literal["1m", "5m", "15m", "30m", "1h", "4h", "1d"]
GetAssetScoreLeaderboardRequestMinTrades = Literal["1", "5", "10", "25", "50", "100"]
GetAssetScoreLeaderboardRequestMinLoss = Literal["0", "100", "1000", "10000", "100000"]
GetAssetScoreLeaderboardRequestMinPnl = Literal["0", "1000", "10000", "100000", "1000000"]
GetAssetScoreLeaderboardRequestMinAccountValue = Literal["1000", "10000", "100000", "1000000", "10000000"]
GetAssetScoreLeaderboardRequestMinCopyScore = Literal["50", "60", "70", "80", "90"]
GetAssistanceFundBuybacksRequestInterval = Literal["1h", "1d"]
GetBoxTradersRequestMinNotional = Literal["1000", "10000", "100000", "1000000"]
GetBridgeFlowsRequestInterval = Literal["1h", "1d"]
GetCandlesRequestInterval = Literal["1s", "15s", "30s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "8h", "12h", "1d", "3d", "1w", "1M"]
GetCohortFlowsRequestInterval = Literal["1h", "1d"]
GetCohortPositioningRequestCohortType = Literal["all", "size", "pnl"]
GetCohortPositioningMarketsRequestCohortType = Literal["all", "size", "pnl"]
GetCvdRequestInterval = Literal["1h", "1d"]
GetExchangeFeesRequestBy = Literal["segment", "market"]
GetExchangeFeesRequestInterval = Literal["1h", "1d"]
GetFootprintRequestInterval = Literal["1m", "5m", "15m", "30m", "1h", "4h", "1d"]
GetFootprintRequestTier = Literal["all", "10k", "100k"]
GetFootprintActionsRequestInterval = Literal["1m", "5m", "15m", "1h", "4h", "1d"]
GetFundingMatrixRequestInterval = Literal["1h", "1d"]
GetFundingRatesRequestInterval = Literal["1h", "1d"]
GetFundingScreenerRequestSort = Literal["highest", "lowest", "abs"]
GetFundingScreenerRequestMinOpenInterestUsd = Literal["0", "1000000", "10000000"]
GetFundingTopWalletsRequestRankBy = Literal["paid", "received", "net"]
GetHip3OracleBarsRequestInterval = Literal["1m", "5m", "1h"]
GetHlpRequestInterval = Literal["1h", "1d"]
GetLendingReserveHistoryRequestInterval = Literal["1h", "1d"]
GetLiquidationHeatmapRequestInterval = Literal["1h", "4h", "1d"]
GetLiquidationRiskLadderRequestKind = Literal["cross", "coin_margined"]
GetLiquidationSeriesRequestInterval = Literal["1h", "1d"]
GetMarketBetaRequestDays = Literal["30", "60", "90"]
GetMarketContextHistoryRequestInterval = Literal["1m", "5m", "15m", "30m", "1h", "4h", "1d"]
GetMarketCorrelationRequestInterval = Literal["1d", "1h"]
GetMarketLeaderboardRequestWindow = Literal["24h", "7d", "30d", "ytd", "alltime"]
GetMarketLeaderboardRequestSort = Literal["total", "realized", "unrealized", "roi", "realized_roi", "unrealized_roi"]
GetMarketPositionChangesRequestWindow = Literal["300", "900", "3600", "14400", "86400"]
GetMarketPositionsRequestSide = Literal["all", "long", "short"]
GetMarketPositionsRequestSort = Literal["notional", "entry_px", "liquidation_px", "unrealized_pnl", "funding_pnl", "account_value", "address"]
GetMarketPositionsRequestOrder = Literal["desc", "asc"]
GetNewTradersRequestInterval = Literal["1h", "1d"]
GetOpenInterestRequestInterval = Literal["1m", "5m", "15m", "30m", "1h", "4h", "1d"]
GetOpenInterestByCategoryRequestInterval = Literal["1h", "1d"]
GetOpenInterestByDexRequestInterval = Literal["1h", "1d"]
GetOpenInterestTotalRequestInterval = Literal["1h", "1d"]
GetOutcomeActivityRequestInterval = Literal["5m", "1h", "1d"]
GetOutcomeHoldersRequestSort = Literal["shares", "notional"]
GetOutcomeLeaderboardRequestHours = Literal["24", "168"]
GetOutcomeProbabilityRequestInterval = Literal["5m", "1h", "1d"]
GetPnlLeaderboardRequestWindow = Literal["alltime", "24h", "7d", "30d", "ytd"]
GetPnlLeaderboardRequestCategory = Literal["all", "crypto", "majors", "memes", "alts", "equities", "indices", "commodities", "fx"]
GetPnlLeaderboardRequestSort = Literal["total", "realized", "unrealized", "roi", "realized_roi", "unrealized_roi"]
GetPnlLeaderboardRequestMinAccountValue = Literal["1000", "10000", "100000", "1000000", "10000000"]
GetPnlLeaderboardRequestMaxAccountValue = Literal["10000", "100000", "1000000", "10000000", "100000000"]
GetPositioningRequestInterval = Literal["1h", "1d"]
GetPriorityFeesRequestInterval = Literal["1h", "1d"]
GetPriorityFeesBuilderShareRequestInterval = Literal["1h", "1d"]
GetPriorityFeesByCoinRequestKind = Literal["write", "write_ioc", "write_alo"]
GetPriorityFeesTopCoinsRequestInterval = Literal["1h", "1d"]
GetPriorityFeesTopCoinsRequestKind = Literal["write", "write_ioc", "write_alo"]
GetRealizedVolatilityRequestInterval = Literal["1h", "1d"]
GetRevenueRequestInterval = Literal["1h", "1d"]
GetSentimentRequestInterval = Literal["1h", "4h", "1d"]
GetSentimentScannerRequestInterval = Literal["1h", "4h", "1d"]
GetSentimentScannerRequestView = Literal["markets", "index"]
GetSortedLeaderboardRequestWindow = Literal["24h", "7d", "30d", "90d", "ytd", "alltime"]
GetSortedLeaderboardRequestSort = Literal["net_pnl", "volume", "trade_count", "win_rate", "fees"]
GetSortedLeaderboardRequestCategory = Literal["all", "crypto", "majors", "memes", "alts", "equities", "indices", "commodities", "fx"]
GetStakeHistoryRequestInterval = Literal["1h", "1d"]
GetStakingFlowsRequestInterval = Literal["1h", "1d"]
GetStopHeatmapRequestInterval = Literal["1h", "4h", "1d"]
GetTopTraderOpensRequestMinutes = Literal["60", "240", "1440"]
GetTradeBarsRequestInterval = Literal["1m", "5m", "15m", "30m", "1h", "4h", "1d"]
GetTraderDistributionRequestView = Literal["size", "volume", "migration"]
GetTraderDistributionRequestPeriod = Literal["week", "month"]
GetTraderGroupRequestGroup = Literal["copytraders", "equities", "btc", "hype", "cl"]
GetUserGrowthRequestInterval = Literal["1d", "1w", "1M"]
GetValidatorRewardsRequestInterval = Literal["1h", "1d"]
GetWalletCompletedTradesRequestSide = Literal["long", "short"]
GetWalletCompletedTradesRequestResult = Literal["win", "loss"]
GetWalletCompletedTradesRequestMinNotional = Literal["1000", "10000", "100000", "1000000"]
GetWalletCompletedTradesRequestSort = Literal["close_time", "net_pnl", "notional", "duration", "entry_px", "exit_px", "funding_pnl", "coin", "side"]
GetWalletCompletedTradesRequestOrder = Literal["desc", "asc"]
GetWalletHighlightsRequestSide = Literal["long", "short"]
GetWalletLeaderboardRequestWindow = Literal["alltime", "24h", "7d", "30d", "ytd"]
GetWalletLeaderboardRequestCategory = Literal["all", "crypto", "majors", "memes", "alts", "equities", "indices", "commodities", "fx"]
GetWalletLeaderboardRequestSort = Literal["total", "realized", "unrealized", "roi", "realized_roi", "unrealized_roi"]
GetWalletUpnlRankRequestCategory = Literal["all", "crypto", "majors", "memes", "alts", "equities", "indices", "commodities", "fx"]
GetWalletWindowStatsRequestWindow = Literal["24h", "7d", "30d", "90d", "ytd", "alltime"]
ListCohortMembersRequestCohort = Literal["small", "medium", "large", "whale", "apex"]
ListCohortMembersRequestSort = Literal["account_value", "notional", "upnl", "upnl_loss"]
ListCohortTopPositionsRequestDirection = Literal["winners", "losers"]
ListFreshWalletPositionsRequestMaxAgeDays = Literal["1", "3", "7"]
ListFreshWalletPositionsRequestMinBalance = Literal["0", "10000", "100000"]
ListLargestOpensRequestMinNotional = Literal["0", "100000", "1000000"]
ListLargestPositionsRequestSide = Literal["long", "short"]
ListMarketCatalogRequestCategory = Literal["btc", "eth_majors", "hype", "memes", "equities", "indices", "commodities", "fx", "other"]
ListMarketCatalogRequestSector = Literal["majors", "memes", "alts", "stocks", "indices", "commodities", "fx"]
ListMarketCatalogRequestAssetClass = Literal["crypto", "stocks", "indices", "commodities", "fx"]
ListOutcomeDefinitionsRequestKind = Literal["outcomes", "questions"]
ListOutcomeMarketsRequestStatus = Literal["active", "settled", "all"]
ListPositionsNearLiquidationRequestMaxDistancePct = Literal["1", "2", "5", "10"]
ListPositionsNearLiquidationRequestMinNotional = Literal["0", "100000", "1000000"]
ListPriorityFeeTopUsersRequestKind = Literal["all", "write", "write_ioc", "write_alo", "read"]
ListStakerChangesRequestSort = Literal["gain", "loss", "abs"]
ListTopUnrealizedPnlRequestCategory = Literal["all", "crypto", "stocks", "indices", "commodities", "fx", "majors"]
ListTopUnrealizedPositionsRequestSide = Literal["winners", "losers"]
ListTopUnrealizedPositionsRequestMinNotional = Literal["10000", "100000", "1000000", "10000000"]
ListUnrealizedPnlMoversRequestInterval = Literal["1h", "4h", "1d", "7d"]
ListUnrealizedPnlMoversRequestDirection = Literal["winners", "losers"]
ListWhaleDepositsRequestMinUsd = Literal["100000", "1000000", "3000000", "5000000"]
OptionsIvRequestCoin = Literal["BTC", "ETH"]
OptionsIvRequestMode = Literal["latest", "daily"]
PortfolioLeaderboardRequestSort = Literal["unrealizedPnl", "unrealizedPnlPct", "totalPnl", "realizedPnlPct", "totalPnlPct"]
PortfolioLeaderboardRequestWindow = Literal["day", "week", "month", "alltime"]
SnapshotAccountValuesRequestCollateral = Literal["hyperliquid:USDC"]
TraderClassStatsRequestTraderClass = Literal["Scalper", "Executioner", "Toxic Taker", "Sniper", "Ronin"]
TraderScreenerRequestSort = Literal["pnlPerpDay", "pnlPerpWeek", "pnlPerpMonth", "pnlPerpAllTime", "winrate30d", "winrate7d", "winrateLifetime", "totalTrades", "accountValue", "notional", "sharpe30d", "copyScore", "copyScoreV2", "maxDrawdownPerpWeek", "maxDrawdownPerpMonth"]
TraderScreenerRequestOrder = Literal["desc", "asc"]
TraderScreenerRequestTimeframe = Literal["24h", "7d", "30d", "alltime"]
TraderScreenerRequestMinAccountValue = Literal["1000", "10000", "100000", "1000000", "10000000"]
TraderScreenerRequestMinPnl = Literal["0", "10000", "100000", "1000000"]
TraderScreenerRequestMinCopyScore = Literal["50", "70", "90"]
TraderScreenerRequestTag = Literal["algo", "scalp", "intraday", "swing", "position"]
TraderScreenerRequestMaxAccountValue = Literal["10000", "100000", "1000000", "10000000", "100000000"]
TraderScreenerRequestMaxPnl = Literal["0", "-1000", "-10000", "-100000", "-1000000"]
TraderScreenerRequestMaxCopyScore = Literal["30", "50", "70", "90"]
TraderScreenerRequestWinrateWindow = Literal["7d", "30d", "lifetime"]
TraderScreenerRequestMinWinrate = Literal["0.3", "0.4", "0.5", "0.6", "0.7", "0.8", "0.9"]
TraderScreenerRequestMaxWinrate = Literal["0.1", "0.2", "0.3", "0.4", "0.5", "0.6"]
TraderScreenerRequestMinSharpe = Literal["0", "1", "2", "3", "5"]
TraderScreenerRequestMaxSharpe = Literal["0", "-1", "-2"]
TraderScreenerRequestDrawdownWindow = Literal["7d", "30d"]
TraderScreenerRequestMaxDrawdown = Literal["0.05", "0.1", "0.2", "0.3", "0.5"]
TraderScreenerRequestTradesWindow = Literal["7d", "30d", "total"]
TraderScreenerRequestMinTrades = Literal["1", "10", "50", "100", "500", "1000"]
TraderScreenerRequestMaxTrades = Literal["10", "50", "100", "500", "1000"]
TraderScreenerRequestVerified = Literal["true"]
VenueShareRequestMode = Literal["latest", "daily", "hourly"]
_AccountPnl90dRequestOptional = TypedDict("_AccountPnl90dRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class AccountPnl90dRequest(_AccountPnl90dRequestOptional):
    address: str

_AccountPnl90dResponseDataItemOptional = TypedDict("_AccountPnl90dResponseDataItemOptional", {'date': 'str | None', 'pnl': 'str | None', 'pnl_90d': 'str | None', 'as_of': 'str | None'}, total=False)
class AccountPnl90dResponseDataItem(_AccountPnl90dResponseDataItemOptional):
    pass

class AccountPnl90dResponse(TypedDict):
    data: list[AccountPnl90dResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_AccountPositionHistoryRequestOptional = TypedDict("_AccountPositionHistoryRequestOptional", {'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class AccountPositionHistoryRequest(_AccountPositionHistoryRequestOptional):
    address: str
    times: str

_AccountPositionHistoryResponseDataItemOptional = TypedDict("_AccountPositionHistoryResponseDataItemOptional", {'requested_time': 'int | None', 'snapshot_time': 'int | None', 'market': 'str | None', 'size': 'str | None', 'notional': 'str | None', 'entry_px': 'str | None', 'liquidation_px': 'str | None', 'unrealized_pnl': 'str | None', 'funding_pnl': 'str | None', 'positions': 'int | None', 'total_unrealized_pnl': 'str | None'}, total=False)
class AccountPositionHistoryResponseDataItem(_AccountPositionHistoryResponseDataItemOptional):
    pass

class AccountPositionHistoryResponse(TypedDict):
    data: list[AccountPositionHistoryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_BatchRecentTradesRequestOptional = TypedDict("_BatchRecentTradesRequestOptional", {'per_address': 'int | None', 'market': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class BatchRecentTradesRequest(_BatchRecentTradesRequestOptional):
    addresses: str

_BatchRecentTradesResponseDataItemOptional = TypedDict("_BatchRecentTradesResponseDataItemOptional", {'address': 'str | None', 'coin': 'str | None', 'direction': 'str | None', 'net_pnl': 'str | None', 'close_time': 'int | None'}, total=False)
class BatchRecentTradesResponseDataItem(_BatchRecentTradesResponseDataItemOptional):
    pass

class BatchRecentTradesResponse(TypedDict):
    data: list[BatchRecentTradesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_BuilderMembersRequestOptional = TypedDict("_BuilderMembersRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class BuilderMembersRequest(_BuilderMembersRequestOptional):
    builder: str

_BuilderMembersResponseDataItemOptional = TypedDict("_BuilderMembersResponseDataItemOptional", {'address': 'str | None'}, total=False)
class BuilderMembersResponseDataItem(_BuilderMembersResponseDataItemOptional):
    pass

class BuilderMembersResponse(TypedDict):
    data: list[BuilderMembersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

class CancelJobRequest(TypedDict):
    id: str

_CancelJobResponseErrorOptional = TypedDict("_CancelJobResponseErrorOptional", {'code': 'str | None', 'detail': 'str | None'}, total=False)
class CancelJobResponseError(_CancelJobResponseErrorOptional):
    pass

_CancelJobResponseOptional = TypedDict("_CancelJobResponseOptional", {'id': 'str | None', 'kind': 'str | None', 'params': 'dict[str, Any] | None', 'status': 'str | None', 'created_at': 'str | None', 'started_at': 'str | None', 'finished_at': 'str | None', 'expires_at': 'str | None', 'result_rows': 'int | None', 'compute_units': 'int | None', 'result_url': 'str | None', 'error': 'CancelJobResponseError | None'}, total=False)
class CancelJobResponse(_CancelJobResponseOptional):
    pass

class CreateJobRequestBody(TypedDict):
    kind: CreateJobRequestBodyKind
    params: dict[str, Any]

class CreateJobRequest(TypedDict):
    idempotency_key: str
    body: CreateJobRequestBody

_CreateJobResponseErrorOptional = TypedDict("_CreateJobResponseErrorOptional", {'code': 'str | None', 'detail': 'str | None'}, total=False)
class CreateJobResponseError(_CreateJobResponseErrorOptional):
    pass

_CreateJobResponseOptional = TypedDict("_CreateJobResponseOptional", {'id': 'str | None', 'kind': 'str | None', 'params': 'dict[str, Any] | None', 'status': 'str | None', 'created_at': 'str | None', 'started_at': 'str | None', 'finished_at': 'str | None', 'expires_at': 'str | None', 'result_rows': 'int | None', 'compute_units': 'int | None', 'result_url': 'str | None', 'error': 'CreateJobResponseError | None'}, total=False)
class CreateJobResponse(_CreateJobResponseOptional):
    pass

_EtfFlowsRequestOptional = TypedDict("_EtfFlowsRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class EtfFlowsRequest(_EtfFlowsRequestOptional):
    asset: str

_EtfFlowsResponseDataItemOptional = TypedDict("_EtfFlowsResponseDataItemOptional", {'date': 'str | None', 'fund': 'str | None', 'source': 'str | None', 'flow_usd': 'str | None', 'span_days': 'int | None', 'prev_date': 'str | None', 'shares_outstanding': 'str | None', 'nav_per_share': 'str | None', 'net_assets': 'str | None', 'coins_held': 'str | None', 'fetched_at': 'int | None'}, total=False)
class EtfFlowsResponseDataItem(_EtfFlowsResponseDataItemOptional):
    pass

class EtfFlowsResponse(TypedDict):
    data: list[EtfFlowsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_FootprintAllWalletsRequestOptional = TypedDict("_FootprintAllWalletsRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
_FootprintAllWalletsRequestWireKeys = TypedDict("_FootprintAllWalletsRequestWireKeys", {'from': 'int'})
class FootprintAllWalletsRequest(_FootprintAllWalletsRequestWireKeys, _FootprintAllWalletsRequestOptional):
    coin: str
    to: int

_FootprintAllWalletsResponseDataItemOptional = TypedDict("_FootprintAllWalletsResponseDataItemOptional", {'side': 'str | None', 'address': 'str | None', 'size': 'str | None', 'notional': 'str | None', 'fills': 'int | None', 'taker_size': 'str | None', 'taker_notional': 'str | None', 'taker_fills': 'int | None', 'side_wallets': 'int | None', 'side_size': 'str | None', 'side_notional': 'str | None', 'side_fills': 'int | None', 'side_taker_size': 'str | None', 'side_taker_notional': 'str | None', 'side_taker_fills': 'int | None'}, total=False)
class FootprintAllWalletsResponseDataItem(_FootprintAllWalletsResponseDataItemOptional):
    pass

class FootprintAllWalletsResponse(TypedDict):
    data: list[FootprintAllWalletsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountRequestOptional = TypedDict("_GetAccountRequestOptional", {'dex': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountRequest(_GetAccountRequestOptional):
    address: str

_GetAccountResponseDataItemOptional = TypedDict("_GetAccountResponseDataItemOptional", {'account_value': 'str | None', 'total_notional': 'str | None', 'total_raw_usd': 'str | None', 'total_margin_used': 'str | None', 'cross_account_value': 'str | None', 'cross_maintenance_margin_used': 'str | None', 'withdrawable': 'str | None', 'positions': 'int | None', 'as_of': 'int | None', 'role': 'str | None', 'master': 'str | None', 'agent_of': 'str | None', 'abstraction': 'str | None', 'equity': 'str | None', 'spot_value': 'str | None', 'staked_value': 'str | None', 'cross_total_notional': 'str | None', 'cross_total_raw_usd': 'str | None', 'cross_total_margin_used': 'str | None'}, total=False)
class GetAccountResponseDataItem(_GetAccountResponseDataItemOptional):
    pass

class GetAccountResponse(TypedDict):
    data: list[GetAccountResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountAgentsRequestOptional = TypedDict("_GetAccountAgentsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountAgentsRequest(_GetAccountAgentsRequestOptional):
    address: str

_GetAccountAgentsResponseDataItemOptional = TypedDict("_GetAccountAgentsResponseDataItemOptional", {'name': 'str | None', 'agent': 'str | None', 'valid_until': 'int | None'}, total=False)
class GetAccountAgentsResponseDataItem(_GetAccountAgentsResponseDataItemOptional):
    pass

class GetAccountAgentsResponse(TypedDict):
    data: list[GetAccountAgentsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountAllDexesRequestOptional = TypedDict("_GetAccountAllDexesRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountAllDexesRequest(_GetAccountAllDexesRequestOptional):
    address: str

_GetAccountAllDexesResponseDataItemOptional = TypedDict("_GetAccountAllDexesResponseDataItemOptional", {'dex': 'str | None', 'account_value': 'str | None', 'total_notional': 'str | None', 'total_margin_used': 'str | None', 'withdrawable': 'str | None', 'positions': 'int | None', 'as_of': 'int | None', 'total_raw_usd': 'str | None', 'cross_account_value': 'str | None', 'cross_total_notional': 'str | None', 'cross_total_raw_usd': 'str | None', 'cross_total_margin_used': 'str | None', 'cross_maintenance_margin_used': 'str | None'}, total=False)
class GetAccountAllDexesResponseDataItem(_GetAccountAllDexesResponseDataItemOptional):
    pass

class GetAccountAllDexesResponse(TypedDict):
    data: list[GetAccountAllDexesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountApprovedBuildersRequestOptional = TypedDict("_GetAccountApprovedBuildersRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountApprovedBuildersRequest(_GetAccountApprovedBuildersRequestOptional):
    address: str

_GetAccountApprovedBuildersResponseDataItemOptional = TypedDict("_GetAccountApprovedBuildersResponseDataItemOptional", {'builder': 'str | None'}, total=False)
class GetAccountApprovedBuildersResponseDataItem(_GetAccountApprovedBuildersResponseDataItemOptional):
    pass

class GetAccountApprovedBuildersResponse(TypedDict):
    data: list[GetAccountApprovedBuildersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountBorrowLendRequestOptional = TypedDict("_GetAccountBorrowLendRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountBorrowLendRequest(_GetAccountBorrowLendRequestOptional):
    address: str

_GetAccountBorrowLendResponseDataItemOptional = TypedDict("_GetAccountBorrowLendResponseDataItemOptional", {'token': 'int | None', 'borrow_value': 'str | None', 'borrow_basis': 'str | None', 'supply_value': 'str | None', 'supply_basis': 'str | None'}, total=False)
class GetAccountBorrowLendResponseDataItem(_GetAccountBorrowLendResponseDataItemOptional):
    pass

class GetAccountBorrowLendResponse(TypedDict):
    data: list[GetAccountBorrowLendResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountBuilderFeeRequestOptional = TypedDict("_GetAccountBuilderFeeRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountBuilderFeeRequest(_GetAccountBuilderFeeRequestOptional):
    address: str
    builder: str

_GetAccountBuilderFeeResponseDataItemOptional = TypedDict("_GetAccountBuilderFeeResponseDataItemOptional", {'builder': 'str | None', 'max_fee_tenths_bp': 'int | None'}, total=False)
class GetAccountBuilderFeeResponseDataItem(_GetAccountBuilderFeeResponseDataItemOptional):
    pass

class GetAccountBuilderFeeResponse(TypedDict):
    data: list[GetAccountBuilderFeeResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountCoreWriterActionsRequestOptional = TypedDict("_GetAccountCoreWriterActionsRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountCoreWriterActionsRequest(_GetAccountCoreWriterActionsRequestOptional):
    address: str

_GetAccountCoreWriterActionsResponseDataItemOptional = TypedDict("_GetAccountCoreWriterActionsResponseDataItemOptional", {'time': 'int | None', 'role': 'str | None', 'user': 'str | None', 'destination': 'str | None', 'action_type': 'str | None', 'nonce': 'int | None', 'evm_tx_hash': 'str | None', 'token': 'int | None', 'wei': 'str | None', 'wei_approx': 'str | None', 'action': 'str | None', 'block': 'int | None', 'event_index': 'int | None'}, total=False)
class GetAccountCoreWriterActionsResponseDataItem(_GetAccountCoreWriterActionsResponseDataItemOptional):
    pass

class GetAccountCoreWriterActionsResponse(TypedDict):
    data: list[GetAccountCoreWriterActionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountFeesRequestOptional = TypedDict("_GetAccountFeesRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountFeesRequest(_GetAccountFeesRequestOptional):
    address: str

_GetAccountFeesResponseDataItemOptional = TypedDict("_GetAccountFeesResponseDataItemOptional", {'taker_rate': 'str | None', 'maker_rate': 'str | None', 'spot_taker_rate': 'str | None', 'spot_maker_rate': 'str | None', 'referral_discount': 'str | None', 'staking_discount': 'str | None', 'volume_14d_taker': 'str | None', 'volume_14d_maker': 'str | None', 'staking_bps_of_max_supply': 'str | None', 'daily_volume': 'str | None', 'fee_schedule': 'str | None', 'trial': 'str | None', 'fee_trial_escrow': 'str | None', 'next_trial_available_time': 'int | None', 'staking_link': 'str | None'}, total=False)
class GetAccountFeesResponseDataItem(_GetAccountFeesResponseDataItemOptional):
    pass

class GetAccountFeesResponse(TypedDict):
    data: list[GetAccountFeesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountFillsRequestOptional = TypedDict("_GetAccountFillsRequestOptional", {'limit': 'int | None', 'aggregate': 'GetAccountFillsRequestAggregate | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountFillsRequest(_GetAccountFillsRequestOptional):
    address: str

_GetAccountFillsResponseDataItemOptional = TypedDict("_GetAccountFillsResponseDataItemOptional", {'time': 'int | None', 'coin': 'str | None', 'side': 'str | None', 'px': 'str | None', 'sz': 'str | None', 'dir': 'str | None', 'start_position': 'str | None', 'closed_pnl': 'str | None', 'fee': 'str | None', 'fee_token': 'str | None', 'crossed': 'bool | None', 'hash': 'str | None', 'oid': 'str | None', 'tid': 'str | None', 'twap_id': 'str | None', 'builder': 'str | None', 'builder_fee': 'str | None', 'cloid': 'str | None', 'liquidated_user': 'str | None', 'liquidation_mark_px': 'str | None', 'liquidation_method': 'str | None', 'deployer_fee': 'str | None', 'priority_gas': 'str | None', 'sub_builders': 'str | None', 'tx_index': 'int | None', 'block_number': 'int | None', 'block_time': 'int | None'}, total=False)
class GetAccountFillsResponseDataItem(_GetAccountFillsResponseDataItemOptional):
    pass

class GetAccountFillsResponse(TypedDict):
    data: list[GetAccountFillsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountFundingRequestOptional = TypedDict("_GetAccountFundingRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountFundingRequest(_GetAccountFundingRequestOptional):
    address: str

_GetAccountFundingResponseDataItemOptional = TypedDict("_GetAccountFundingResponseDataItemOptional", {'time': 'int | None', 'coin': 'str | None', 'usdc': 'str | None', 'size': 'str | None', 'funding_rate': 'str | None', 'hash': 'str | None', 'samples': 'int | None'}, total=False)
class GetAccountFundingResponseDataItem(_GetAccountFundingResponseDataItemOptional):
    pass

class GetAccountFundingResponse(TypedDict):
    data: list[GetAccountFundingResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountL1ActionsRequestOptional = TypedDict("_GetAccountL1ActionsRequestOptional", {'action_type': 'str | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountL1ActionsRequest(_GetAccountL1ActionsRequestOptional):
    address: str

_GetAccountL1ActionsResponseDataItemOptional = TypedDict("_GetAccountL1ActionsResponseDataItemOptional", {'time': 'int | None', 'action_type': 'str | None', 'ok': 'bool | None', 'error': 'str | None', 'asset': 'int | None', 'counterparty': 'str | None', 'response_user': 'str | None', 'vault_address': 'str | None', 'multi_sig_user': 'str | None', 'action': 'str | None', 'block': 'int | None', 'bundle_index': 'int | None', 'action_index': 'int | None', 'inner': 'bool | None'}, total=False)
class GetAccountL1ActionsResponseDataItem(_GetAccountL1ActionsResponseDataItemOptional):
    pass

class GetAccountL1ActionsResponse(TypedDict):
    data: list[GetAccountL1ActionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountLeadingVaultsRequestOptional = TypedDict("_GetAccountLeadingVaultsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountLeadingVaultsRequest(_GetAccountLeadingVaultsRequestOptional):
    address: str

_GetAccountLeadingVaultsResponseDataItemOptional = TypedDict("_GetAccountLeadingVaultsResponseDataItemOptional", {'vault': 'str | None', 'name': 'str | None'}, total=False)
class GetAccountLeadingVaultsResponseDataItem(_GetAccountLeadingVaultsResponseDataItemOptional):
    pass

class GetAccountLeadingVaultsResponse(TypedDict):
    data: list[GetAccountLeadingVaultsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountLedgerRequestOptional = TypedDict("_GetAccountLedgerRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'include': 'GetAccountLedgerRequestInclude | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountLedgerRequest(_GetAccountLedgerRequestOptional):
    address: str

_GetAccountLedgerResponseDataItemOptional = TypedDict("_GetAccountLedgerResponseDataItemOptional", {'time': 'int | None', 'hash': 'str | None', 'type': 'str | None', 'usdc': 'str | None', 'token': 'str | None', 'amount': 'str | None', 'usdc_value': 'str | None', 'fee': 'str | None', 'destination': 'str | None', 'vault': 'str | None', 'details': 'str | None'}, total=False)
class GetAccountLedgerResponseDataItem(_GetAccountLedgerResponseDataItemOptional):
    pass

class GetAccountLedgerResponse(TypedDict):
    data: list[GetAccountLedgerResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountMarketRequestOptional = TypedDict("_GetAccountMarketRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountMarketRequest(_GetAccountMarketRequestOptional):
    address: str
    coin: str

_GetAccountMarketResponseDataItemOptional = TypedDict("_GetAccountMarketResponseDataItemOptional", {'leverage_type': 'str | None', 'leverage': 'int | None', 'max_buy_size': 'str | None', 'max_sell_size': 'str | None', 'available_to_buy': 'str | None', 'available_to_sell': 'str | None', 'mark_px': 'str | None', 'leverage_raw_usd': 'str | None'}, total=False)
class GetAccountMarketResponseDataItem(_GetAccountMarketResponseDataItemOptional):
    pass

class GetAccountMarketResponse(TypedDict):
    data: list[GetAccountMarketResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountMultisigSignersRequestOptional = TypedDict("_GetAccountMultisigSignersRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountMultisigSignersRequest(_GetAccountMultisigSignersRequestOptional):
    address: str

_GetAccountMultisigSignersResponseDataItemOptional = TypedDict("_GetAccountMultisigSignersResponseDataItemOptional", {'signer': 'str | None'}, total=False)
class GetAccountMultisigSignersResponseDataItem(_GetAccountMultisigSignersResponseDataItemOptional):
    pass

class GetAccountMultisigSignersResponse(TypedDict):
    data: list[GetAccountMultisigSignersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountOpenOrdersRequestOptional = TypedDict("_GetAccountOpenOrdersRequestOptional", {'dex': 'str | None', 'all_dexes': 'GetAccountOpenOrdersRequestAllDexes | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountOpenOrdersRequest(_GetAccountOpenOrdersRequestOptional):
    address: str

_GetAccountOpenOrdersResponseDataItemOptional = TypedDict("_GetAccountOpenOrdersResponseDataItemOptional", {'coin': 'str | None', 'side': 'str | None', 'limit_px': 'str | None', 'sz': 'str | None', 'orig_sz': 'str | None', 'oid': 'str | None', 'time': 'int | None', 'order_type': 'str | None', 'tif': 'str | None', 'reduce_only': 'bool | None', 'is_trigger': 'bool | None', 'trigger_px': 'str | None', 'trigger_condition': 'str | None', 'is_position_tpsl': 'bool | None', 'cloid': 'str | None', 'children': 'str | None'}, total=False)
class GetAccountOpenOrdersResponseDataItem(_GetAccountOpenOrdersResponseDataItemOptional):
    pass

class GetAccountOpenOrdersResponse(TypedDict):
    data: list[GetAccountOpenOrdersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountOrderHistoryRequestOptional = TypedDict("_GetAccountOrderHistoryRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountOrderHistoryRequest(_GetAccountOrderHistoryRequestOptional):
    address: str

_GetAccountOrderHistoryResponseDataItemOptional = TypedDict("_GetAccountOrderHistoryResponseDataItemOptional", {'coin': 'str | None', 'side': 'str | None', 'limit_px': 'str | None', 'sz': 'str | None', 'orig_sz': 'str | None', 'oid': 'str | None', 'time': 'int | None', 'order_type': 'str | None', 'tif': 'str | None', 'reduce_only': 'bool | None', 'is_trigger': 'bool | None', 'trigger_px': 'str | None', 'trigger_condition': 'str | None', 'is_position_tpsl': 'bool | None', 'cloid': 'str | None', 'children': 'str | None', 'status': 'str | None', 'status_time': 'int | None', 'builder': 'str | None', 'builder_fee_tenths_bp': 'int | None', 'tx_index': 'int | None'}, total=False)
class GetAccountOrderHistoryResponseDataItem(_GetAccountOrderHistoryResponseDataItemOptional):
    pass

class GetAccountOrderHistoryResponse(TypedDict):
    data: list[GetAccountOrderHistoryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountOrderStatusRequestOptional = TypedDict("_GetAccountOrderStatusRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountOrderStatusRequest(_GetAccountOrderStatusRequestOptional):
    address: str
    oid: str

_GetAccountOrderStatusResponseDataItemOptional = TypedDict("_GetAccountOrderStatusResponseDataItemOptional", {'lookup': 'str | None', 'status': 'str | None', 'status_time': 'int | None', 'coin': 'str | None', 'side': 'str | None', 'limit_px': 'str | None', 'sz': 'str | None', 'orig_sz': 'str | None', 'oid': 'str | None', 'time': 'int | None', 'order_type': 'str | None', 'tif': 'str | None', 'reduce_only': 'bool | None', 'is_trigger': 'bool | None', 'trigger_px': 'str | None', 'trigger_condition': 'str | None', 'is_position_tpsl': 'bool | None', 'cloid': 'str | None', 'children': 'str | None'}, total=False)
class GetAccountOrderStatusResponseDataItem(_GetAccountOrderStatusResponseDataItemOptional):
    pass

class GetAccountOrderStatusResponse(TypedDict):
    data: list[GetAccountOrderStatusResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountPnlChartRequestOptional = TypedDict("_GetAccountPnlChartRequestOptional", {'window': 'GetAccountPnlChartRequestWindow | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountPnlChartRequest(_GetAccountPnlChartRequestOptional):
    address: str

_GetAccountPnlChartResponseDataItemOptional = TypedDict("_GetAccountPnlChartResponseDataItemOptional", {'t': 'int | None', 'account_value': 'str | None', 'net_flows': 'str | None', 'pnl': 'str | None'}, total=False)
class GetAccountPnlChartResponseDataItem(_GetAccountPnlChartResponseDataItemOptional):
    pass

class GetAccountPnlChartResponse(TypedDict):
    data: list[GetAccountPnlChartResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountPnlSummaryRequestOptional = TypedDict("_GetAccountPnlSummaryRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountPnlSummaryRequest(_GetAccountPnlSummaryRequestOptional):
    address: str

_GetAccountPnlSummaryResponseDataItemOptional = TypedDict("_GetAccountPnlSummaryResponseDataItemOptional", {'total_pnl': 'str | None', 'win_rate': 'str | None', 'loss_rate': 'str | None', 'total_trades': 'int | None', 'total_fees': 'str | None', 'total_funding': 'str | None', 'volume_traded': 'str | None', 'days_active': 'int | None', 'account_age_days': 'int | None', 'traded_pairs': 'int | None'}, total=False)
class GetAccountPnlSummaryResponseDataItem(_GetAccountPnlSummaryResponseDataItemOptional):
    pass

class GetAccountPnlSummaryResponse(TypedDict):
    data: list[GetAccountPnlSummaryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountPortfolioRequestOptional = TypedDict("_GetAccountPortfolioRequestOptional", {'all_dexes': 'GetAccountPortfolioRequestAllDexes | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountPortfolioRequest(_GetAccountPortfolioRequestOptional):
    address: str

_GetAccountPortfolioResponseDataItemOptional = TypedDict("_GetAccountPortfolioResponseDataItemOptional", {'kind': 'str | None', 'dex': 'str | None', 'coin': 'str | None', 'size': 'str | None', 'entry': 'str | None', 'value': 'str | None', 'unrealized_pnl': 'str | None', 'margin_used': 'str | None', 'liquidation_px': 'str | None', 'leverage': 'int | None', 'hold': 'str | None', 'leverage_type': 'str | None', 'max_leverage': 'int | None', 'return_on_equity': 'str | None', 'cum_funding_all_time': 'str | None', 'cum_funding_since_open': 'str | None', 'cum_funding_since_change': 'str | None', 'token': 'int | None'}, total=False)
class GetAccountPortfolioResponseDataItem(_GetAccountPortfolioResponseDataItemOptional):
    pass

class GetAccountPortfolioResponse(TypedDict):
    data: list[GetAccountPortfolioResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountPortfolioDailyRequestOptional = TypedDict("_GetAccountPortfolioDailyRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountPortfolioDailyRequest(_GetAccountPortfolioDailyRequestOptional):
    address: str

_GetAccountPortfolioDailyResponseDataItemOptional = TypedDict("_GetAccountPortfolioDailyResponseDataItemOptional", {'day': 'int | None', 'close_time': 'int | None', 'account_value': 'str | None', 'daily_pnl': 'str | None', 'perp_daily_pnl': 'str | None', 'source': 'str | None'}, total=False)
class GetAccountPortfolioDailyResponseDataItem(_GetAccountPortfolioDailyResponseDataItemOptional):
    pass

class GetAccountPortfolioDailyResponse(TypedDict):
    data: list[GetAccountPortfolioDailyResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountPositionsRequestOptional = TypedDict("_GetAccountPositionsRequestOptional", {'dex': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountPositionsRequest(_GetAccountPositionsRequestOptional):
    address: str

_GetAccountPositionsResponseDataItemOptional = TypedDict("_GetAccountPositionsResponseDataItemOptional", {'coin': 'str | None', 'size': 'str | None', 'entry_px': 'str | None', 'position_value': 'str | None', 'unrealized_pnl': 'str | None', 'return_on_equity': 'str | None', 'liquidation_px': 'str | None', 'margin_used': 'str | None', 'max_leverage': 'int | None', 'leverage_type': 'str | None', 'leverage_value': 'int | None', 'leverage_raw_usd': 'str | None', 'cum_funding_all_time': 'str | None', 'cum_funding_since_open': 'str | None', 'cum_funding_since_change': 'str | None'}, total=False)
class GetAccountPositionsResponseDataItem(_GetAccountPositionsResponseDataItemOptional):
    pass

class GetAccountPositionsResponse(TypedDict):
    data: list[GetAccountPositionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountRateLimitRequestOptional = TypedDict("_GetAccountRateLimitRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountRateLimitRequest(_GetAccountRateLimitRequestOptional):
    address: str

_GetAccountRateLimitResponseDataItemOptional = TypedDict("_GetAccountRateLimitResponseDataItemOptional", {'cum_volume': 'str | None', 'requests_used': 'int | None', 'requests_cap': 'int | None', 'requests_surplus': 'int | None'}, total=False)
class GetAccountRateLimitResponseDataItem(_GetAccountRateLimitResponseDataItemOptional):
    pass

class GetAccountRateLimitResponse(TypedDict):
    data: list[GetAccountRateLimitResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountRealizedPnlRequestOptional = TypedDict("_GetAccountRealizedPnlRequestOptional", {'interval': 'GetAccountRealizedPnlRequestInterval | None', 'start_time': 'int | None', 'end_time': 'int | None', 'anchor': 'GetAccountRealizedPnlRequestAnchor | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountRealizedPnlRequest(_GetAccountRealizedPnlRequestOptional):
    address: str

_GetAccountRealizedPnlResponseDataItemOptional = TypedDict("_GetAccountRealizedPnlResponseDataItemOptional", {'t': 'int | None', 'realized_pnl': 'str | None', 'anchored': 'bool | None'}, total=False)
class GetAccountRealizedPnlResponseDataItem(_GetAccountRealizedPnlResponseDataItemOptional):
    pass

class GetAccountRealizedPnlResponse(TypedDict):
    data: list[GetAccountRealizedPnlResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountRiskRequestOptional = TypedDict("_GetAccountRiskRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountRiskRequest(_GetAccountRiskRequestOptional):
    address: str

_GetAccountRiskResponseDataItemOptional = TypedDict("_GetAccountRiskResponseDataItemOptional", {'book': 'str | None', 'dex': 'int | None', 'coin': 'str | None', 'notional': 'str | None', 'mark_px': 'str | None', 'maintenance_margin': 'str | None', 'mm_rate': 'str | None', 'isolated_margin': 'str | None', 'liquidation_px': 'str | None', 'margin_uncertain': 'bool | None', 'beta': 'str | None', 'correlation': 'str | None', 'beta_fallback': 'bool | None', 'btc_move': 'str | None'}, total=False)
class GetAccountRiskResponseDataItem(_GetAccountRiskResponseDataItemOptional):
    pass

class GetAccountRiskResponse(TypedDict):
    data: list[GetAccountRiskResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountSpotBalancesRequestOptional = TypedDict("_GetAccountSpotBalancesRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountSpotBalancesRequest(_GetAccountSpotBalancesRequestOptional):
    address: str

_GetAccountSpotBalancesResponseDataItemOptional = TypedDict("_GetAccountSpotBalancesResponseDataItemOptional", {'coin': 'str | None', 'token': 'int | None', 'total': 'str | None', 'hold': 'str | None', 'entry_ntl': 'str | None', 'available_after_maintenance': 'str | None'}, total=False)
class GetAccountSpotBalancesResponseDataItem(_GetAccountSpotBalancesResponseDataItemOptional):
    pass

class GetAccountSpotBalancesResponse(TypedDict):
    data: list[GetAccountSpotBalancesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountStakingRequestOptional = TypedDict("_GetAccountStakingRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountStakingRequest(_GetAccountStakingRequestOptional):
    address: str

_GetAccountStakingResponseDataItemOptional = TypedDict("_GetAccountStakingResponseDataItemOptional", {'validator': 'str | None', 'amount': 'str | None', 'locked_until': 'int | None', 'undelegated': 'str | None', 'pending_withdrawal': 'str | None', 'pending_withdrawals': 'int | None'}, total=False)
class GetAccountStakingResponseDataItem(_GetAccountStakingResponseDataItemOptional):
    pass

class GetAccountStakingResponse(TypedDict):
    data: list[GetAccountStakingResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountStakingHistoryRequestOptional = TypedDict("_GetAccountStakingHistoryRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountStakingHistoryRequest(_GetAccountStakingHistoryRequestOptional):
    address: str

_GetAccountStakingHistoryResponseDataItemOptional = TypedDict("_GetAccountStakingHistoryResponseDataItemOptional", {'time': 'int | None', 'kind': 'str | None', 'validator': 'str | None', 'amount': 'str | None', 'finalized': 'bool | None', 'hash': 'str | None', 'block': 'int | None', 'event_index': 'int | None'}, total=False)
class GetAccountStakingHistoryResponseDataItem(_GetAccountStakingHistoryResponseDataItemOptional):
    pass

class GetAccountStakingHistoryResponse(TypedDict):
    data: list[GetAccountStakingHistoryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountSubaccountHistoryRequestOptional = TypedDict("_GetAccountSubaccountHistoryRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountSubaccountHistoryRequest(_GetAccountSubaccountHistoryRequestOptional):
    address: str

_GetAccountSubaccountHistoryResponseDataItemOptional = TypedDict("_GetAccountSubaccountHistoryResponseDataItemOptional", {'time': 'int | None', 'sub_account': 'str | None', 'name': 'str | None', 'hash': 'str | None', 'block': 'int | None', 'event_index': 'int | None'}, total=False)
class GetAccountSubaccountHistoryResponseDataItem(_GetAccountSubaccountHistoryResponseDataItemOptional):
    pass

class GetAccountSubaccountHistoryResponse(TypedDict):
    data: list[GetAccountSubaccountHistoryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountSubaccountsRequestOptional = TypedDict("_GetAccountSubaccountsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountSubaccountsRequest(_GetAccountSubaccountsRequestOptional):
    address: str

_GetAccountSubaccountsResponseDataItemOptional = TypedDict("_GetAccountSubaccountsResponseDataItemOptional", {'address': 'str | None', 'name': 'str | None', 'account_value': 'str | None', 'withdrawable': 'str | None', 'spot_balances': 'int | None', 'equity': 'str | None', 'spot_value': 'str | None', 'staked_value': 'str | None'}, total=False)
class GetAccountSubaccountsResponseDataItem(_GetAccountSubaccountsResponseDataItemOptional):
    pass

class GetAccountSubaccountsResponse(TypedDict):
    data: list[GetAccountSubaccountsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountTradesRequestOptional = TypedDict("_GetAccountTradesRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountTradesRequest(_GetAccountTradesRequestOptional):
    address: str

_GetAccountTradesResponseDataItemOptional = TypedDict("_GetAccountTradesResponseDataItemOptional", {'coin': 'str | None', 'side': 'str | None', 'open_time': 'int | None', 'close_time': 'int | None', 'duration_ms': 'int | None', 'entry_px': 'str | None', 'exit_px': 'str | None', 'size': 'str | None', 'max_size': 'str | None', 'gross_pnl': 'str | None', 'net_pnl': 'str | None', 'fees': 'str | None', 'funding_pnl': 'str | None', 'fills': 'int | None', 'maker_fills': 'int | None', 'builder': 'str | None', 'leverage': 'int | None', 'is_cross': 'bool | None', 'tx_index': 'int | None', 'native_realized_pnl': 'str | None', 'fees_incomplete': 'bool | None', 'history_incomplete': 'bool | None', 'close_block_height': 'int | None'}, total=False)
class GetAccountTradesResponseDataItem(_GetAccountTradesResponseDataItemOptional):
    pass

class GetAccountTradesResponse(TypedDict):
    data: list[GetAccountTradesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountTwapFillsRequestOptional = TypedDict("_GetAccountTwapFillsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountTwapFillsRequest(_GetAccountTwapFillsRequestOptional):
    address: str

_GetAccountTwapFillsResponseDataItemOptional = TypedDict("_GetAccountTwapFillsResponseDataItemOptional", {'twap_id': 'int | None', 'time': 'int | None', 'coin': 'str | None', 'side': 'str | None', 'px': 'str | None', 'sz': 'str | None', 'fee': 'str | None', 'closed_pnl': 'str | None', 'hash': 'str | None', 'tid': 'str | None', 'oid': 'str | None', 'dir': 'str | None', 'start_position': 'str | None', 'crossed': 'bool | None', 'fee_token': 'str | None', 'builder': 'str | None', 'builder_fee': 'str | None', 'cloid': 'str | None', 'tx_index': 'int | None'}, total=False)
class GetAccountTwapFillsResponseDataItem(_GetAccountTwapFillsResponseDataItemOptional):
    pass

class GetAccountTwapFillsResponse(TypedDict):
    data: list[GetAccountTwapFillsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountTwapHistoryRequestOptional = TypedDict("_GetAccountTwapHistoryRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountTwapHistoryRequest(_GetAccountTwapHistoryRequestOptional):
    address: str

_GetAccountTwapHistoryResponseDataItemOptional = TypedDict("_GetAccountTwapHistoryResponseDataItemOptional", {'time': 'int | None', 'coin': 'str | None', 'twap_id': 'int | None', 'status': 'str | None', 'error': 'str | None', 'side': 'str | None', 'sz': 'str | None', 'executed_sz': 'str | None', 'executed_ntl': 'str | None', 'minutes': 'int | None', 'reduce_only': 'bool | None', 'randomize': 'bool | None', 'created_time': 'int | None', 'trigger': 'str | None', 'stop_px': 'str | None', 'block': 'int | None', 'event_index': 'int | None'}, total=False)
class GetAccountTwapHistoryResponseDataItem(_GetAccountTwapHistoryResponseDataItemOptional):
    pass

class GetAccountTwapHistoryResponse(TypedDict):
    data: list[GetAccountTwapHistoryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountTwapsRequestOptional = TypedDict("_GetAccountTwapsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountTwapsRequest(_GetAccountTwapsRequestOptional):
    address: str

_GetAccountTwapsResponseDataItemOptional = TypedDict("_GetAccountTwapsResponseDataItemOptional", {'kind': 'str | None', 'coin': 'str | None', 'pair': 'str | None', 'pair_id': 'str | None', 'twap_id': 'str | None', 'side': 'str | None', 'total_sz': 'str | None', 'executed_sz': 'str | None', 'remaining_sz': 'str | None', 'executed_notional': 'str | None', 'progress_pct': 'str | None', 'duration_s': 'int | None', 'start_time': 'int | None', 'reduce_only': 'bool | None', 'randomize': 'bool | None', 'next_slice_time': 'str | None', 'slices_done': 'int | None', 'waiting': 'bool | None', 'stop_px': 'str | None', 'trigger_px': 'str | None', 'trigger_above': 'bool | None'}, total=False)
class GetAccountTwapsResponseDataItem(_GetAccountTwapsResponseDataItemOptional):
    pass

class GetAccountTwapsResponse(TypedDict):
    data: list[GetAccountTwapsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountValueHistoryRequestOptional = TypedDict("_GetAccountValueHistoryRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountValueHistoryRequest(_GetAccountValueHistoryRequestOptional):
    address: str

_GetAccountValueHistoryResponseDataItemOptional = TypedDict("_GetAccountValueHistoryResponseDataItemOptional", {'time': 'int | None', 'account_value': 'str | None'}, total=False)
class GetAccountValueHistoryResponseDataItem(_GetAccountValueHistoryResponseDataItemOptional):
    pass

class GetAccountValueHistoryResponse(TypedDict):
    data: list[GetAccountValueHistoryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountVaultEquitiesRequestOptional = TypedDict("_GetAccountVaultEquitiesRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountVaultEquitiesRequest(_GetAccountVaultEquitiesRequestOptional):
    address: str

_GetAccountVaultEquitiesResponseDataItemOptional = TypedDict("_GetAccountVaultEquitiesResponseDataItemOptional", {'vault': 'str | None', 'equity': 'str | None', 'locked_until': 'int | None'}, total=False)
class GetAccountVaultEquitiesResponseDataItem(_GetAccountVaultEquitiesResponseDataItemOptional):
    pass

class GetAccountVaultEquitiesResponse(TypedDict):
    data: list[GetAccountVaultEquitiesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountsBatchRequestOptional = TypedDict("_GetAccountsBatchRequestOptional", {'dex': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountsBatchRequest(_GetAccountsBatchRequestOptional):
    addresses: str

_GetAccountsBatchResponseDataItemOptional = TypedDict("_GetAccountsBatchResponseDataItemOptional", {'address': 'str | None', 'account_value': 'str | None', 'total_notional': 'str | None', 'total_margin_used': 'str | None', 'cross_maintenance_margin_used': 'str | None', 'withdrawable': 'str | None', 'positions': 'int | None', 'as_of': 'int | None', 'equity': 'str | None', 'spot_value': 'str | None', 'staked_value': 'str | None'}, total=False)
class GetAccountsBatchResponseDataItem(_GetAccountsBatchResponseDataItemOptional):
    pass

class GetAccountsBatchResponse(TypedDict):
    data: list[GetAccountsBatchResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountsBatchPortfolioRequestOptional = TypedDict("_GetAccountsBatchPortfolioRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountsBatchPortfolioRequest(_GetAccountsBatchPortfolioRequestOptional):
    addresses: str

_GetAccountsBatchPortfolioResponseDataItemOptional = TypedDict("_GetAccountsBatchPortfolioResponseDataItemOptional", {'address': 'str | None', 'abstraction': 'str | None', 'account_value': 'str | None', 'total_notional': 'str | None', 'withdrawable': 'str | None', 'maintenance_margin': 'str | None', 'perp_positions': 'int | None', 'spot_balances': 'int | None', 'spot_usdc': 'str | None', 'as_of': 'int | None', 'equity': 'str | None', 'spot_value': 'str | None', 'staked_value': 'str | None'}, total=False)
class GetAccountsBatchPortfolioResponseDataItem(_GetAccountsBatchPortfolioResponseDataItemOptional):
    pass

class GetAccountsBatchPortfolioResponse(TypedDict):
    data: list[GetAccountsBatchPortfolioResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountsBatchStatsRequestOptional = TypedDict("_GetAccountsBatchStatsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountsBatchStatsRequest(_GetAccountsBatchStatsRequestOptional):
    addresses: str

_GetAccountsBatchStatsResponseDataItemOptional = TypedDict("_GetAccountsBatchStatsResponseDataItemOptional", {'address': 'str | None', 'pnl_day': 'str | None', 'pnl_week': 'str | None', 'pnl_month': 'str | None', 'pnl_alltime': 'str | None', 'total_trades': 'int | None', 'trades_7d': 'int | None', 'trades_30d': 'int | None', 'long_trades': 'int | None', 'short_trades': 'int | None', 'winning_trades': 'int | None', 'losing_trades': 'int | None', 'winrate': 'str | None', 'winrate_7d': 'str | None', 'winrate_30d': 'str | None', 'winrate_90d': 'str | None', 'last_trade_at': 'int | None', 'last_fill_at': 'int | None', 'account_value': 'str | None', 'notional': 'str | None', 'unrealized_pnl': 'str | None', 'display_name': 'str | None', 'twitter': 'str | None', 'tag': 'str | None', 'verified': 'bool | None', 'label': 'str | None', 'sharpe_7d': 'str | None', 'sharpe_30d': 'str | None', 'sortino_7d': 'str | None', 'sortino_30d': 'str | None', 'sortino_90d': 'str | None', 'max_drawdown_week': 'str | None', 'max_drawdown_month': 'str | None', 'max_drawdown_90d': 'str | None', 'gpr_7d': 'str | None', 'gpr_30d': 'str | None', 'gpr_90d': 'str | None', 'gpr_all_time': 'str | None', 'copy_score': 'str | None', 'copy_score_v2': 'str | None', 'top_assets': 'str | None', 'total_volume': 'str | None', 'updated_at': 'int | None', 'avatar': 'str | None'}, total=False)
class GetAccountsBatchStatsResponseDataItem(_GetAccountsBatchStatsResponseDataItemOptional):
    pass

class GetAccountsBatchStatsResponse(TypedDict):
    data: list[GetAccountsBatchStatsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAccountsLiquidationRiskRequestOptional = TypedDict("_GetAccountsLiquidationRiskRequestOptional", {'kind': 'GetAccountsLiquidationRiskRequestKind | None', 'min_equity': 'str | None', 'min_leverage': 'str | None', 'sort': 'GetAccountsLiquidationRiskRequestSort | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAccountsLiquidationRiskRequest(_GetAccountsLiquidationRiskRequestOptional):
    pass

_GetAccountsLiquidationRiskResponseDataItemOptional = TypedDict("_GetAccountsLiquidationRiskResponseDataItemOptional", {'address': 'str | None', 'mode': 'str | None', 'coin_margined': 'bool | None', 'equity': 'str | None', 'cross_notional': 'str | None', 'isolated_notional': 'str | None', 'cross_leverage': 'str | None', 'btc_delta': 'str | None', 'maintenance_margin': 'str | None', 'cushion': 'str | None', 'collateral_hype_usd': 'str | None', 'collateral_btc_usd': 'str | None', 'btc_move': 'str | None', 'btc_liquidation_px': 'str | None', 'approximate': 'bool | None', 'margin_uncertain': 'bool | None'}, total=False)
class GetAccountsLiquidationRiskResponseDataItem(_GetAccountsLiquidationRiskResponseDataItemOptional):
    pass

class GetAccountsLiquidationRiskResponse(TypedDict):
    data: list[GetAccountsLiquidationRiskResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetActiveTradersRequestOptional = TypedDict("_GetActiveTradersRequestOptional", {'scope': 'str | None', 'interval': 'GetActiveTradersRequestInterval | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetActiveTradersRequest(_GetActiveTradersRequestOptional):
    pass

_GetActiveTradersResponseDataItemOptional = TypedDict("_GetActiveTradersResponseDataItemOptional", {'t': 'int | None', 'traders': 'int | None'}, total=False)
class GetActiveTradersResponseDataItem(_GetActiveTradersResponseDataItemOptional):
    pass

class GetActiveTradersResponse(TypedDict):
    data: list[GetActiveTradersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetActiveTwapPressureRequestOptional = TypedDict("_GetActiveTwapPressureRequestOptional", {'market': 'str | None', 'min_volume': 'GetActiveTwapPressureRequestMinVolume | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetActiveTwapPressureRequest(_GetActiveTwapPressureRequestOptional):
    pass

_GetActiveTwapPressureResponseDataItemOptional = TypedDict("_GetActiveTwapPressureResponseDataItemOptional", {'coin': 'str | None', 'buy_notional': 'str | None', 'sell_notional': 'str | None', 'net_notional': 'str | None', 'twaps': 'int | None'}, total=False)
class GetActiveTwapPressureResponseDataItem(_GetActiveTwapPressureResponseDataItemOptional):
    pass

class GetActiveTwapPressureResponse(TypedDict):
    data: list[GetActiveTwapPressureResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAloPriorityRequestOptional = TypedDict("_GetAloPriorityRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAloPriorityRequest(_GetAloPriorityRequestOptional):
    coin: str

_GetAloPriorityResponseDataItemOptional = TypedDict("_GetAloPriorityResponseDataItemOptional", {'time': 'int | None', 'oid': 'str | None', 'user': 'str | None', 'side': 'str | None', 'px': 'str | None', 'sz': 'str | None', 'p': 'int | None', 'tif': 'str | None', 'fee_usd': 'str | None', 'block': 'int | None', 'bundle_index': 'int | None', 'action_index': 'int | None', 'order_index': 'int | None'}, total=False)
class GetAloPriorityResponseDataItem(_GetAloPriorityResponseDataItemOptional):
    pass

class GetAloPriorityResponse(TypedDict):
    data: list[GetAloPriorityResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAloPriorityBarsRequestOptional = TypedDict("_GetAloPriorityBarsRequestOptional", {'interval': 'GetAloPriorityBarsRequestInterval | None', 'payers': 'int | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAloPriorityBarsRequest(_GetAloPriorityBarsRequestOptional):
    coin: str

_GetAloPriorityBarsResponseDataItemOptional = TypedDict("_GetAloPriorityBarsResponseDataItemOptional", {'t': 'int | None', 'user': 'str | None', 'side': 'str | None', 'resting': 'int | None', 'notional_usd': 'str | None', 'fee_usd': 'str | None', 'max_p': 'int | None', 'bar_resting': 'int | None', 'bar_fee_usd': 'str | None', 'bar_payers': 'int | None'}, total=False)
class GetAloPriorityBarsResponseDataItem(_GetAloPriorityBarsResponseDataItemOptional):
    pass

class GetAloPriorityBarsResponse(TypedDict):
    data: list[GetAloPriorityBarsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAssetScoreLeaderboardRequestOptional = TypedDict("_GetAssetScoreLeaderboardRequestOptional", {'min_trades': 'GetAssetScoreLeaderboardRequestMinTrades | None', 'min_loss': 'GetAssetScoreLeaderboardRequestMinLoss | None', 'min_pnl': 'GetAssetScoreLeaderboardRequestMinPnl | None', 'min_account_value': 'GetAssetScoreLeaderboardRequestMinAccountValue | None', 'min_copy_score': 'GetAssetScoreLeaderboardRequestMinCopyScore | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAssetScoreLeaderboardRequest(_GetAssetScoreLeaderboardRequestOptional):
    coin: str

_GetAssetScoreLeaderboardResponseDataItemOptional = TypedDict("_GetAssetScoreLeaderboardResponseDataItemOptional", {'rank': 'int | None', 'address': 'str | None', 'asset_score': 'str | None', 'net_pnl': 'str | None', 'gains': 'str | None', 'losses': 'str | None', 'volume': 'str | None', 'trades': 'int | None', 'winning_trades': 'int | None', 'losing_trades': 'int | None', 'account_value': 'str | None', 'copy_score': 'str | None', 'display_name': 'str | None', 'avatar': 'str | None', 'twitter': 'str | None', 'tag': 'str | None', 'verified': 'bool | None', 'label': 'str | None', 'last_trade_at': 'int | None', 'updated_at': 'int | None', 'total': 'int | None'}, total=False)
class GetAssetScoreLeaderboardResponseDataItem(_GetAssetScoreLeaderboardResponseDataItemOptional):
    pass

class GetAssetScoreLeaderboardResponse(TypedDict):
    data: list[GetAssetScoreLeaderboardResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetAssistanceFundBuybacksRequestOptional = TypedDict("_GetAssistanceFundBuybacksRequestOptional", {'interval': 'GetAssistanceFundBuybacksRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetAssistanceFundBuybacksRequest(_GetAssistanceFundBuybacksRequestOptional):
    pass

_GetAssistanceFundBuybacksResponseDataItemOptional = TypedDict("_GetAssistanceFundBuybacksResponseDataItemOptional", {'t': 'int | None', 'coin': 'str | None', 'buy_size': 'str | None', 'buy_notional': 'str | None', 'sell_size': 'str | None', 'sell_notional': 'str | None', 'fills': 'int | None'}, total=False)
class GetAssistanceFundBuybacksResponseDataItem(_GetAssistanceFundBuybacksResponseDataItemOptional):
    pass

class GetAssistanceFundBuybacksResponse(TypedDict):
    data: list[GetAssistanceFundBuybacksResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetBarFlowsRequestOptional = TypedDict("_GetBarFlowsRequestOptional", {'minutes': 'int | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetBarFlowsRequest(_GetBarFlowsRequestOptional):
    coin: str
    bar_start: int

_GetBarFlowsResponseDataItemOptional = TypedDict("_GetBarFlowsResponseDataItemOptional", {'t': 'int | None', 'side': 'str | None', 'rank': 'int | None', 'address': 'str | None', 'notional': 'str | None', 'fills': 'int | None', 'badge': 'str | None', 'bar_volume': 'str | None', 'bar_trades': 'int | None'}, total=False)
class GetBarFlowsResponseDataItem(_GetBarFlowsResponseDataItemOptional):
    pass

class GetBarFlowsResponse(TypedDict):
    data: list[GetBarFlowsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetBoxTradersRequestOptional = TypedDict("_GetBoxTradersRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'min_px': 'str | None', 'max_px': 'str | None', 'min_notional': 'GetBoxTradersRequestMinNotional | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetBoxTradersRequest(_GetBoxTradersRequestOptional):
    coin: str

_GetBoxTradersResponseDataItemOptional = TypedDict("_GetBoxTradersResponseDataItemOptional", {'t': 'int | None', 'address': 'str | None', 'stance': 'str | None', 'rank': 'int | None', 'notional': 'str | None', 'size': 'str | None', 'avg_px': 'str | None', 'fills': 'int | None', 'badge': 'str | None'}, total=False)
class GetBoxTradersResponseDataItem(_GetBoxTradersResponseDataItemOptional):
    pass

class GetBoxTradersResponse(TypedDict):
    data: list[GetBoxTradersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetBridgeFlowsRequestOptional = TypedDict("_GetBridgeFlowsRequestOptional", {'interval': 'GetBridgeFlowsRequestInterval | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetBridgeFlowsRequest(_GetBridgeFlowsRequestOptional):
    pass

_GetBridgeFlowsResponseDataItemOptional = TypedDict("_GetBridgeFlowsResponseDataItemOptional", {'t': 'int | None', 'route': 'str | None', 'token': 'str | None', 'direction': 'str | None', 'amount': 'str | None', 'fee': 'str | None', 'transfers': 'int | None'}, total=False)
class GetBridgeFlowsResponseDataItem(_GetBridgeFlowsResponseDataItemOptional):
    pass

class GetBridgeFlowsResponse(TypedDict):
    data: list[GetBridgeFlowsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetBuilderSubBuilderSplitRequestOptional = TypedDict("_GetBuilderSubBuilderSplitRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetBuilderSubBuilderSplitRequest(_GetBuilderSubBuilderSplitRequestOptional):
    builder: str

_GetBuilderSubBuilderSplitResponseDataItemOptional = TypedDict("_GetBuilderSubBuilderSplitResponseDataItemOptional", {'kind': 'str | None', 'sub_builder': 'str | None', 'fee_token': 'str | None', 'orders': 'int | None', 'fills': 'int | None', 'notional': 'str | None', 'fee': 'str | None', 'fee_unreported_fills': 'int | None', 'first_fill': 'int | None', 'last_fill': 'int | None'}, total=False)
class GetBuilderSubBuilderSplitResponseDataItem(_GetBuilderSubBuilderSplitResponseDataItemOptional):
    pass

class GetBuilderSubBuilderSplitResponse(TypedDict):
    data: list[GetBuilderSubBuilderSplitResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetBuilderVolumeShareRequestOptional = TypedDict("_GetBuilderVolumeShareRequestOptional", {'days': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetBuilderVolumeShareRequest(_GetBuilderVolumeShareRequestOptional):
    pass

_GetBuilderVolumeShareResponseDataItemOptional = TypedDict("_GetBuilderVolumeShareResponseDataItemOptional", {'t': 'int | None', 'hip3_volume': 'str | None', 'core_volume': 'str | None', 'hip3_share_pct': 'str | None'}, total=False)
class GetBuilderVolumeShareResponseDataItem(_GetBuilderVolumeShareResponseDataItemOptional):
    pass

class GetBuilderVolumeShareResponse(TypedDict):
    data: list[GetBuilderVolumeShareResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetCandlesRequestOptional = TypedDict("_GetCandlesRequestOptional", {'interval': 'GetCandlesRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetCandlesRequest(_GetCandlesRequestOptional):
    coin: str

_GetCandlesResponseDataItemOptional = TypedDict("_GetCandlesResponseDataItemOptional", {'t': 'int | None', 'o': 'str | None', 'h': 'str | None', 'l': 'str | None', 'c': 'str | None', 'v': 'str | None', 'q': 'str | None', 'n': 'int | None', 'liq_only': 'bool | None'}, total=False)
class GetCandlesResponseDataItem(_GetCandlesResponseDataItemOptional):
    pass

class GetCandlesResponse(TypedDict):
    data: list[GetCandlesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetChangelogRequestOptional = TypedDict("_GetChangelogRequestOptional", {'since': 'int | None'}, total=False)
class GetChangelogRequest(_GetChangelogRequestOptional):
    pass

class GetChangelogResponse(TypedDict):
    data: list[dict[str, Any]]

_GetCohortFlowsRequestOptional = TypedDict("_GetCohortFlowsRequestOptional", {'coin': 'str | None', 'asset_class': 'str | None', 'interval': 'GetCohortFlowsRequestInterval | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetCohortFlowsRequest(_GetCohortFlowsRequestOptional):
    pass

_GetCohortFlowsResponseDataItemOptional = TypedDict("_GetCohortFlowsResponseDataItemOptional", {'t': 'int | None', 'cohort': 'str | None', 'taker_buy_notional': 'str | None', 'taker_sell_notional': 'str | None', 'taker_fills': 'int | None'}, total=False)
class GetCohortFlowsResponseDataItem(_GetCohortFlowsResponseDataItemOptional):
    pass

class GetCohortFlowsResponse(TypedDict):
    data: list[GetCohortFlowsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetCohortPositioningRequestOptional = TypedDict("_GetCohortPositioningRequestOptional", {'coin': 'str | None', 'cohort_type': 'GetCohortPositioningRequestCohortType | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetCohortPositioningRequest(_GetCohortPositioningRequestOptional):
    pass

_GetCohortPositioningResponseDataItemOptional = TypedDict("_GetCohortPositioningResponseDataItemOptional", {'t': 'int | None', 'coin': 'str | None', 'cohort_type': 'str | None', 'cohort': 'str | None', 'long_notional': 'str | None', 'long_traders': 'int | None', 'short_notional': 'str | None', 'short_traders': 'int | None'}, total=False)
class GetCohortPositioningResponseDataItem(_GetCohortPositioningResponseDataItemOptional):
    pass

class GetCohortPositioningResponse(TypedDict):
    data: list[GetCohortPositioningResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetCohortPositioningMarketsRequestOptional = TypedDict("_GetCohortPositioningMarketsRequestOptional", {'time': 'int | None', 'cohort_type': 'GetCohortPositioningMarketsRequestCohortType | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetCohortPositioningMarketsRequest(_GetCohortPositioningMarketsRequestOptional):
    pass

_GetCohortPositioningMarketsResponseDataItemOptional = TypedDict("_GetCohortPositioningMarketsResponseDataItemOptional", {'t': 'int | None', 'coin': 'str | None', 'cohort_type': 'str | None', 'cohort': 'str | None', 'long_notional': 'str | None', 'long_traders': 'int | None', 'short_notional': 'str | None', 'short_traders': 'int | None'}, total=False)
class GetCohortPositioningMarketsResponseDataItem(_GetCohortPositioningMarketsResponseDataItemOptional):
    pass

class GetCohortPositioningMarketsResponse(TypedDict):
    data: list[GetCohortPositioningMarketsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetCohortTotalsRequestOptional = TypedDict("_GetCohortTotalsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetCohortTotalsRequest(_GetCohortTotalsRequestOptional):
    pass

_GetCohortTotalsResponseDataItemOptional = TypedDict("_GetCohortTotalsResponseDataItemOptional", {'cohort': 'str | None', 'wallets': 'int | None', 'account_value': 'str | None', 'wallets_with_positions': 'int | None', 'unrealized_pnl': 'str | None', 'profitable_wallets': 'int | None', 'losing_wallets': 'int | None', 'equity_snapshot_time': 'int | None', 'positions_snapshot_time': 'int | None', 'positions_complete': 'bool | None'}, total=False)
class GetCohortTotalsResponseDataItem(_GetCohortTotalsResponseDataItemOptional):
    pass

class GetCohortTotalsResponse(TypedDict):
    data: list[GetCohortTotalsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetCvdRequestOptional = TypedDict("_GetCvdRequestOptional", {'interval': 'GetCvdRequestInterval | None', 'days': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetCvdRequest(_GetCvdRequestOptional):
    coin: str

_GetCvdResponseDataItemOptional = TypedDict("_GetCvdResponseDataItemOptional", {'t': 'int | None', 'buy_notional': 'str | None', 'sell_notional': 'str | None', 'delta': 'str | None', 'cvd': 'str | None'}, total=False)
class GetCvdResponseDataItem(_GetCvdResponseDataItemOptional):
    pass

class GetCvdResponse(TypedDict):
    data: list[GetCvdResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetDatasetsResponseDataItemEndpointsItemOptional = TypedDict("_GetDatasetsResponseDataItemEndpointsItemOptional", {'route': 'str | None', 'summary': 'str | None', 'scope': 'str | None', 'max_range_ms': 'int | None', 'max_limit': 'int | None', 'pricing': 'str | None'}, total=False)
class GetDatasetsResponseDataItemEndpointsItem(_GetDatasetsResponseDataItemEndpointsItemOptional):
    pass

_GetDatasetsResponseDataItemOptional = TypedDict("_GetDatasetsResponseDataItemOptional", {'id': 'str | None', 'title': 'str | None', 'description': 'str | None', 'status': 'str | None', 'coverage_start': 'str | None', 'freshness': 'str | None', 'gaps': 'list[Any] | None', 'history_entitlement': 'str | None', 'endpoints': 'list[GetDatasetsResponseDataItemEndpointsItem] | None'}, total=False)
class GetDatasetsResponseDataItem(_GetDatasetsResponseDataItemOptional):
    pass

class GetDatasetsResponse(TypedDict):
    data: list[GetDatasetsResponseDataItem]

_GetDexLimitsRequestOptional = TypedDict("_GetDexLimitsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetDexLimitsRequest(_GetDexLimitsRequestOptional):
    dex: str

_GetDexLimitsResponseDataItemOptional = TypedDict("_GetDexLimitsResponseDataItemOptional", {'coin': 'str | None', 'oi_cap': 'str | None'}, total=False)
class GetDexLimitsResponseDataItem(_GetDexLimitsResponseDataItemOptional):
    pass

class GetDexLimitsResponse(TypedDict):
    data: list[GetDexLimitsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetDexStatusRequestOptional = TypedDict("_GetDexStatusRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetDexStatusRequest(_GetDexStatusRequestOptional):
    dex: str

_GetDexStatusResponseDataItemOptional = TypedDict("_GetDexStatusResponseDataItemOptional", {'dex': 'str | None', 'total_net_deposit': 'str | None'}, total=False)
class GetDexStatusResponseDataItem(_GetDexStatusResponseDataItemOptional):
    pass

class GetDexStatusResponse(TypedDict):
    data: list[GetDexStatusResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetEquitySessionGapsRequestOptional = TypedDict("_GetEquitySessionGapsRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetEquitySessionGapsRequest(_GetEquitySessionGapsRequestOptional):
    coin: str

_GetEquitySessionGapsResponseDataItemOptional = TypedDict("_GetEquitySessionGapsResponseDataItemOptional", {'ny_date': 'str | None', 'px_before_close': 'str | None', 'px_at_open': 'str | None', 'samples_before_close': 'int | None', 'samples_after_open': 'int | None'}, total=False)
class GetEquitySessionGapsResponseDataItem(_GetEquitySessionGapsResponseDataItemOptional):
    pass

class GetEquitySessionGapsResponse(TypedDict):
    data: list[GetEquitySessionGapsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetEquitySessionSlotsRequestOptional = TypedDict("_GetEquitySessionSlotsRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetEquitySessionSlotsRequest(_GetEquitySessionSlotsRequestOptional):
    coin: str

_GetEquitySessionSlotsResponseDataItemOptional = TypedDict("_GetEquitySessionSlotsResponseDataItemOptional", {'t': 'int | None', 'volume': 'str | None', 'trades': 'int | None', 'liq_notional': 'str | None', 'liquidations': 'int | None'}, total=False)
class GetEquitySessionSlotsResponseDataItem(_GetEquitySessionSlotsResponseDataItemOptional):
    pass

class GetEquitySessionSlotsResponse(TypedDict):
    data: list[GetEquitySessionSlotsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetEventMovesRequestOptional = TypedDict("_GetEventMovesRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetEventMovesRequest(_GetEventMovesRequestOptional):
    coin: str
    times: str

_GetEventMovesResponseDataItemOptional = TypedDict("_GetEventMovesResponseDataItemOptional", {'time': 'int | None', 'px_before': 'str | None', 'samples_before': 'int | None', 'px_after': 'str | None', 'samples_after': 'int | None'}, total=False)
class GetEventMovesResponseDataItem(_GetEventMovesResponseDataItemOptional):
    pass

class GetEventMovesResponse(TypedDict):
    data: list[GetEventMovesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetExchangeFeesRequestOptional = TypedDict("_GetExchangeFeesRequestOptional", {'by': 'GetExchangeFeesRequestBy | None', 'segment': 'str | None', 'coin': 'str | None', 'interval': 'GetExchangeFeesRequestInterval | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetExchangeFeesRequest(_GetExchangeFeesRequestOptional):
    pass

_GetExchangeFeesResponseDataItemOptional = TypedDict("_GetExchangeFeesResponseDataItemOptional", {'t': 'int | None', 'key': 'str | None', 'quote_token': 'str | None', 'taker_buy_notional': 'str | None', 'taker_sell_notional': 'str | None', 'fees_paid': 'str | None', 'maker_rebates': 'str | None', 'builder_fees': 'str | None', 'trades': 'int | None', 'fills': 'int | None'}, total=False)
class GetExchangeFeesResponseDataItem(_GetExchangeFeesResponseDataItemOptional):
    pass

class GetExchangeFeesResponse(TypedDict):
    data: list[GetExchangeFeesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetExchangeStatusRequestOptional = TypedDict("_GetExchangeStatusRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetExchangeStatusRequest(_GetExchangeStatusRequestOptional):
    pass

_GetExchangeStatusResponseDataItemOptional = TypedDict("_GetExchangeStatusResponseDataItemOptional", {'special_statuses': 'str | None', 'time': 'int | None'}, total=False)
class GetExchangeStatusResponseDataItem(_GetExchangeStatusResponseDataItemOptional):
    pass

class GetExchangeStatusResponse(TypedDict):
    data: list[GetExchangeStatusResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetFootprintRequestOptional = TypedDict("_GetFootprintRequestOptional", {'interval': 'GetFootprintRequestInterval | None', 'row_size': 'str | None', 'tier': 'GetFootprintRequestTier | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetFootprintRequest(_GetFootprintRequestOptional):
    coin: str

_GetFootprintResponseDataItemOptional = TypedDict("_GetFootprintResponseDataItemOptional", {'t': 'int | None', 'px': 'str | None', 'buy_sz': 'str | None', 'sell_sz': 'str | None', 'buy_n': 'int | None', 'sell_n': 'int | None', 'final': 'bool | None'}, total=False)
class GetFootprintResponseDataItem(_GetFootprintResponseDataItemOptional):
    pass

class GetFootprintResponse(TypedDict):
    data: list[GetFootprintResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetFootprintActionsRequestOptional = TypedDict("_GetFootprintActionsRequestOptional", {'interval': 'GetFootprintActionsRequestInterval | None', 'row_size': 'str | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetFootprintActionsRequest(_GetFootprintActionsRequestOptional):
    coin: str

_GetFootprintActionsResponseDataItemOptional = TypedDict("_GetFootprintActionsResponseDataItemOptional", {'t': 'int | None', 'px': 'str | None', 'ol_taker_sz': 'str | None', 'ol_maker_sz': 'str | None', 'cl_taker_sz': 'str | None', 'cl_maker_sz': 'str | None', 'os_taker_sz': 'str | None', 'os_maker_sz': 'str | None', 'cs_taker_sz': 'str | None', 'cs_maker_sz': 'str | None', 'ol_taker_ntl': 'str | None', 'ol_maker_ntl': 'str | None', 'cl_taker_ntl': 'str | None', 'cl_maker_ntl': 'str | None', 'os_taker_ntl': 'str | None', 'os_maker_ntl': 'str | None', 'cs_taker_ntl': 'str | None', 'cs_maker_ntl': 'str | None', 'taker_legs': 'int | None', 'maker_legs': 'int | None', 'row_size': 'str | None'}, total=False)
class GetFootprintActionsResponseDataItem(_GetFootprintActionsResponseDataItemOptional):
    pass

class GetFootprintActionsResponse(TypedDict):
    data: list[GetFootprintActionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetFundingFlowsRequestOptional = TypedDict("_GetFundingFlowsRequestOptional", {'coin': 'str | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetFundingFlowsRequest(_GetFundingFlowsRequestOptional):
    pass

_GetFundingFlowsResponseDataItemOptional = TypedDict("_GetFundingFlowsResponseDataItemOptional", {'t': 'int | None', 'side': 'str | None', 'paid': 'str | None', 'received': 'str | None', 'payments': 'int | None', 'payers': 'int | None', 'earners': 'int | None'}, total=False)
class GetFundingFlowsResponseDataItem(_GetFundingFlowsResponseDataItemOptional):
    pass

class GetFundingFlowsResponse(TypedDict):
    data: list[GetFundingFlowsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetFundingMatrixRequestOptional = TypedDict("_GetFundingMatrixRequestOptional", {'market': 'str | None', 'top': 'int | None', 'interval': 'GetFundingMatrixRequestInterval | None', 'days': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetFundingMatrixRequest(_GetFundingMatrixRequestOptional):
    pass

_GetFundingMatrixResponseDataItemOptional = TypedDict("_GetFundingMatrixResponseDataItemOptional", {'coin': 'str | None', 't': 'int | None', 'funding_rate': 'str | None', 'funding_apr_pct': 'str | None'}, total=False)
class GetFundingMatrixResponseDataItem(_GetFundingMatrixResponseDataItemOptional):
    pass

class GetFundingMatrixResponse(TypedDict):
    data: list[GetFundingMatrixResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetFundingRatesRequestOptional = TypedDict("_GetFundingRatesRequestOptional", {'interval': 'GetFundingRatesRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetFundingRatesRequest(_GetFundingRatesRequestOptional):
    coin: str

_GetFundingRatesResponseDataItemOptional = TypedDict("_GetFundingRatesResponseDataItemOptional", {'t': 'int | None', 'rate': 'str | None', 'hours': 'int | None'}, total=False)
class GetFundingRatesResponseDataItem(_GetFundingRatesResponseDataItemOptional):
    pass

class GetFundingRatesResponse(TypedDict):
    data: list[GetFundingRatesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetFundingScreenerRequestOptional = TypedDict("_GetFundingScreenerRequestOptional", {'sort': 'GetFundingScreenerRequestSort | None', 'min_open_interest_usd': 'GetFundingScreenerRequestMinOpenInterestUsd | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetFundingScreenerRequest(_GetFundingScreenerRequestOptional):
    pass

_GetFundingScreenerResponseDataItemOptional = TypedDict("_GetFundingScreenerResponseDataItemOptional", {'coin': 'str | None', 'funding_rate': 'str | None', 'funding_apr_pct': 'str | None', 'mark_px': 'str | None', 'open_interest_usd': 'str | None', 'day_volume': 'str | None', 'time': 'int | None'}, total=False)
class GetFundingScreenerResponseDataItem(_GetFundingScreenerResponseDataItemOptional):
    pass

class GetFundingScreenerResponse(TypedDict):
    data: list[GetFundingScreenerResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetFundingTopWalletsRequestOptional = TypedDict("_GetFundingTopWalletsRequestOptional", {'rank_by': 'GetFundingTopWalletsRequestRankBy | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetFundingTopWalletsRequest(_GetFundingTopWalletsRequestOptional):
    coin: str

_GetFundingTopWalletsResponseDataItemOptional = TypedDict("_GetFundingTopWalletsResponseDataItemOptional", {'rank': 'int | None', 'address': 'str | None', 'paid': 'str | None', 'received': 'str | None', 'net': 'str | None', 'payments': 'int | None'}, total=False)
class GetFundingTopWalletsResponseDataItem(_GetFundingTopWalletsResponseDataItemOptional):
    pass

class GetFundingTopWalletsResponse(TypedDict):
    data: list[GetFundingTopWalletsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetHip3OracleBarsRequestOptional = TypedDict("_GetHip3OracleBarsRequestOptional", {'interval': 'GetHip3OracleBarsRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetHip3OracleBarsRequest(_GetHip3OracleBarsRequestOptional):
    coin: str

_GetHip3OracleBarsResponseDataItemOptional = TypedDict("_GetHip3OracleBarsResponseDataItemOptional", {'t': 'int | None', 'oracle_open': 'str | None', 'oracle_high': 'str | None', 'oracle_low': 'str | None', 'oracle_close': 'str | None', 'mark_close': 'str | None', 'external_perp_close': 'str | None', 'oracle_daily_px': 'str | None', 'updates': 'int | None', 'fallback_updates': 'int | None'}, total=False)
class GetHip3OracleBarsResponseDataItem(_GetHip3OracleBarsResponseDataItemOptional):
    pass

class GetHip3OracleBarsResponse(TypedDict):
    data: list[GetHip3OracleBarsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetHlpRequestOptional = TypedDict("_GetHlpRequestOptional", {'interval': 'GetHlpRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetHlpRequest(_GetHlpRequestOptional):
    pass

_GetHlpResponseDataItemOptional = TypedDict("_GetHlpResponseDataItemOptional", {'t': 'int | None', 'strategy': 'str | None', 'account_value': 'str | None', 'pnl': 'str | None'}, total=False)
class GetHlpResponseDataItem(_GetHlpResponseDataItemOptional):
    pass

class GetHlpResponse(TypedDict):
    data: list[GetHlpResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetHypeHolderBandsRequestOptional = TypedDict("_GetHypeHolderBandsRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetHypeHolderBandsRequest(_GetHypeHolderBandsRequestOptional):
    pass

_GetHypeHolderBandsResponseDataItemOptional = TypedDict("_GetHypeHolderBandsResponseDataItemOptional", {'t': 'int | None', 'band': 'str | None', 'wallets': 'int | None', 'held': 'str | None'}, total=False)
class GetHypeHolderBandsResponseDataItem(_GetHypeHolderBandsResponseDataItemOptional):
    pass

class GetHypeHolderBandsResponse(TypedDict):
    data: list[GetHypeHolderBandsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetHypeSupplyRequestOptional = TypedDict("_GetHypeSupplyRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetHypeSupplyRequest(_GetHypeSupplyRequestOptional):
    pass

_GetHypeSupplyResponseDataItemOptional = TypedDict("_GetHypeSupplyResponseDataItemOptional", {'t': 'int | None', 'circulating_supply': 'str | None', 'total_supply': 'str | None', 'max_supply': 'str | None', 'future_emissions': 'str | None', 'mark_px': 'str | None', 'snapshot_time': 'int | None'}, total=False)
class GetHypeSupplyResponseDataItem(_GetHypeSupplyResponseDataItemOptional):
    pass

class GetHypeSupplyResponse(TypedDict):
    data: list[GetHypeSupplyResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

class GetJobRequest(TypedDict):
    id: str

_GetJobResponseErrorOptional = TypedDict("_GetJobResponseErrorOptional", {'code': 'str | None', 'detail': 'str | None'}, total=False)
class GetJobResponseError(_GetJobResponseErrorOptional):
    pass

_GetJobResponseOptional = TypedDict("_GetJobResponseOptional", {'id': 'str | None', 'kind': 'str | None', 'params': 'dict[str, Any] | None', 'status': 'str | None', 'created_at': 'str | None', 'started_at': 'str | None', 'finished_at': 'str | None', 'expires_at': 'str | None', 'result_rows': 'int | None', 'compute_units': 'int | None', 'result_url': 'str | None', 'error': 'GetJobResponseError | None'}, total=False)
class GetJobResponse(_GetJobResponseOptional):
    pass

_GetJobResultRequestOptional = TypedDict("_GetJobResultRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None'}, total=False)
class GetJobResultRequest(_GetJobResultRequestOptional):
    id: str

_GetJobResultResponseOptional = TypedDict("_GetJobResultResponseOptional", {'data': 'list[dict[str, Any]] | None', 'has_more': 'bool | None', 'next_cursor': 'str | None', 'meta': 'dict[str, Any] | None'}, total=False)
class GetJobResultResponse(_GetJobResultResponseOptional):
    pass

_GetL4BookRequestOptional = TypedDict("_GetL4BookRequestOptional", {'band_bps': 'int | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetL4BookRequest(_GetL4BookRequestOptional):
    coin: str

_GetL4BookResponseDataItemOptional = TypedDict("_GetL4BookResponseDataItemOptional", {'side': 'str | None', 'px': 'str | None', 'sz': 'str | None', 'orig_sz': 'str | None', 'oid': 'str | None', 'user': 'str | None', 'time': 'int | None', 'tif': 'str | None', 'order_type': 'str | None', 'reduce_only': 'bool | None', 'is_trigger': 'bool | None', 'trigger_px': 'str | None'}, total=False)
class GetL4BookResponseDataItem(_GetL4BookResponseDataItemOptional):
    pass

class GetL4BookResponse(TypedDict):
    data: list[GetL4BookResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetLendingReserveHistoryRequestOptional = TypedDict("_GetLendingReserveHistoryRequestOptional", {'interval': 'GetLendingReserveHistoryRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetLendingReserveHistoryRequest(_GetLendingReserveHistoryRequestOptional):
    token: int

_GetLendingReserveHistoryResponseDataItemOptional = TypedDict("_GetLendingReserveHistoryResponseDataItemOptional", {'t': 'int | None', 'snapshot_time': 'int | None', 'token_name': 'str | None', 'supply_apr': 'str | None', 'borrow_apr': 'str | None', 'utilization': 'str | None', 'total_supplied': 'str | None', 'total_borrowed': 'str | None', 'available': 'str | None', 'oracle_px': 'str | None', 'ltv': 'str | None'}, total=False)
class GetLendingReserveHistoryResponseDataItem(_GetLendingReserveHistoryResponseDataItemOptional):
    pass

class GetLendingReserveHistoryResponse(TypedDict):
    data: list[GetLendingReserveHistoryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetLiquidationCascadesRequestOptional = TypedDict("_GetLiquidationCascadesRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetLiquidationCascadesRequest(_GetLiquidationCascadesRequestOptional):
    pass

_GetLiquidationCascadesResponseDataItemOptional = TypedDict("_GetLiquidationCascadesResponseDataItemOptional", {'day': 'int | None', 'lead_coin': 'str | None', 'peak_minute': 'int | None', 'peak_minute_notional': 'str | None', 'notional': 'str | None', 'long_notional': 'str | None', 'short_notional': 'str | None', 'events': 'int | None'}, total=False)
class GetLiquidationCascadesResponseDataItem(_GetLiquidationCascadesResponseDataItemOptional):
    pass

class GetLiquidationCascadesResponse(TypedDict):
    data: list[GetLiquidationCascadesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetLiquidationHeatmapRequestOptional = TypedDict("_GetLiquidationHeatmapRequestOptional", {'interval': 'GetLiquidationHeatmapRequestInterval | None', 'min_px': 'str | None', 'max_px': 'str | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetLiquidationHeatmapRequest(_GetLiquidationHeatmapRequestOptional):
    coin: str

_GetLiquidationHeatmapResponseDataItemOptional = TypedDict("_GetLiquidationHeatmapResponseDataItemOptional", {'t': 'int | None', 'band_min': 'str | None', 'band_max': 'str | None', 'long_size': 'str | None', 'short_size': 'str | None', 'positions': 'int | None', 'mark_px': 'str | None', 'band_size': 'str | None'}, total=False)
class GetLiquidationHeatmapResponseDataItem(_GetLiquidationHeatmapResponseDataItemOptional):
    pass

class GetLiquidationHeatmapResponse(TypedDict):
    data: list[GetLiquidationHeatmapResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetLiquidationLevelTotalsRequestOptional = TypedDict("_GetLiquidationLevelTotalsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetLiquidationLevelTotalsRequest(_GetLiquidationLevelTotalsRequestOptional):
    coin: str

_GetLiquidationLevelTotalsResponseDataItemOptional = TypedDict("_GetLiquidationLevelTotalsResponseDataItemOptional", {'long_size': 'str | None', 'long_positions': 'int | None', 'short_size': 'str | None', 'short_positions': 'int | None'}, total=False)
class GetLiquidationLevelTotalsResponseDataItem(_GetLiquidationLevelTotalsResponseDataItemOptional):
    pass

class GetLiquidationLevelTotalsResponse(TypedDict):
    data: list[GetLiquidationLevelTotalsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetLiquidationLevelsRequestOptional = TypedDict("_GetLiquidationLevelsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetLiquidationLevelsRequest(_GetLiquidationLevelsRequestOptional):
    coin: str

_GetLiquidationLevelsResponseDataItemOptional = TypedDict("_GetLiquidationLevelsResponseDataItemOptional", {'side': 'str | None', 'px': 'str | None', 'notional': 'str | None', 'size': 'str | None', 'positions': 'int | None'}, total=False)
class GetLiquidationLevelsResponseDataItem(_GetLiquidationLevelsResponseDataItemOptional):
    pass

class GetLiquidationLevelsResponse(TypedDict):
    data: list[GetLiquidationLevelsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetLiquidationRiskLadderRequestOptional = TypedDict("_GetLiquidationRiskLadderRequestOptional", {'kind': 'GetLiquidationRiskLadderRequestKind | None', 'min_equity': 'str | None', 'min_leverage': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetLiquidationRiskLadderRequest(_GetLiquidationRiskLadderRequestOptional):
    pass

_GetLiquidationRiskLadderResponseDataItemOptional = TypedDict("_GetLiquidationRiskLadderResponseDataItemOptional", {'from_pct': 'int | None', 'to_pct': 'int | None', 'accounts': 'int | None', 'notional': 'str | None', 'equity': 'str | None', 'btc_px': 'str | None'}, total=False)
class GetLiquidationRiskLadderResponseDataItem(_GetLiquidationRiskLadderResponseDataItemOptional):
    pass

class GetLiquidationRiskLadderResponse(TypedDict):
    data: list[GetLiquidationRiskLadderResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetLiquidationSeriesRequestOptional = TypedDict("_GetLiquidationSeriesRequestOptional", {'coin': 'str | None', 'asset_class': 'str | None', 'interval': 'GetLiquidationSeriesRequestInterval | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetLiquidationSeriesRequest(_GetLiquidationSeriesRequestOptional):
    pass

_GetLiquidationSeriesResponseDataItemOptional = TypedDict("_GetLiquidationSeriesResponseDataItemOptional", {'t': 'int | None', 'side': 'str | None', 'notional': 'str | None', 'closed_pnl': 'str | None', 'events': 'int | None'}, total=False)
class GetLiquidationSeriesResponseDataItem(_GetLiquidationSeriesResponseDataItemOptional):
    pass

class GetLiquidationSeriesResponse(TypedDict):
    data: list[GetLiquidationSeriesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetLiquidationTotalsByMarketRequestOptional = TypedDict("_GetLiquidationTotalsByMarketRequestOptional", {'coin': 'str | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetLiquidationTotalsByMarketRequest(_GetLiquidationTotalsByMarketRequestOptional):
    pass

_GetLiquidationTotalsByMarketResponseDataItemOptional = TypedDict("_GetLiquidationTotalsByMarketResponseDataItemOptional", {'coin': 'str | None', 'events': 'int | None', 'total_notional': 'str | None', 'long_notional': 'str | None', 'short_notional': 'str | None', 'closed_pnl': 'str | None', 'backstop_events': 'int | None', 'largest_notional': 'str | None', 'accounts': 'int | None', 'trader_events': 'int | None', 'trader_notional': 'str | None', 'trader_long_notional': 'str | None', 'trader_short_notional': 'str | None', 'trader_largest_notional': 'str | None', 'trader_accounts': 'int | None'}, total=False)
class GetLiquidationTotalsByMarketResponseDataItem(_GetLiquidationTotalsByMarketResponseDataItemOptional):
    pass

class GetLiquidationTotalsByMarketResponse(TypedDict):
    data: list[GetLiquidationTotalsByMarketResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetMarketBetaRequestOptional = TypedDict("_GetMarketBetaRequestOptional", {'days': 'GetMarketBetaRequestDays | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetMarketBetaRequest(_GetMarketBetaRequestOptional):
    pass

_GetMarketBetaResponseDataItemOptional = TypedDict("_GetMarketBetaResponseDataItemOptional", {'coin': 'str | None', 'beta': 'str | None', 'correlation': 'str | None', 'hours': 'int | None'}, total=False)
class GetMarketBetaResponseDataItem(_GetMarketBetaResponseDataItemOptional):
    pass

class GetMarketBetaResponse(TypedDict):
    data: list[GetMarketBetaResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetMarketContextHistoryRequestOptional = TypedDict("_GetMarketContextHistoryRequestOptional", {'interval': 'GetMarketContextHistoryRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetMarketContextHistoryRequest(_GetMarketContextHistoryRequestOptional):
    coin: str

_GetMarketContextHistoryResponseDataItemOptional = TypedDict("_GetMarketContextHistoryResponseDataItemOptional", {'t': 'int | None', 'mark_open': 'str | None', 'mark_high': 'str | None', 'mark_low': 'str | None', 'mark_close': 'str | None', 'oracle_open': 'str | None', 'oracle_high': 'str | None', 'oracle_low': 'str | None', 'oracle_close': 'str | None', 'mid_close': 'str | None', 'premium': 'str | None', 'predicted_funding': 'str | None', 'impact_bid': 'str | None', 'impact_ask': 'str | None', 'minutes': 'int | None'}, total=False)
class GetMarketContextHistoryResponseDataItem(_GetMarketContextHistoryResponseDataItemOptional):
    pass

class GetMarketContextHistoryResponse(TypedDict):
    data: list[GetMarketContextHistoryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetMarketCorrelationRequestOptional = TypedDict("_GetMarketCorrelationRequestOptional", {'interval': 'GetMarketCorrelationRequestInterval | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetMarketCorrelationRequest(_GetMarketCorrelationRequestOptional):
    coins: str

_GetMarketCorrelationResponseDataItemOptional = TypedDict("_GetMarketCorrelationResponseDataItemOptional", {'coin_a': 'str | None', 'coin_b': 'str | None', 'n': 'int | None', 'correlation': 'str | None'}, total=False)
class GetMarketCorrelationResponseDataItem(_GetMarketCorrelationResponseDataItemOptional):
    pass

class GetMarketCorrelationResponse(TypedDict):
    data: list[GetMarketCorrelationResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetMarketLeaderboardRequestOptional = TypedDict("_GetMarketLeaderboardRequestOptional", {'window': 'GetMarketLeaderboardRequestWindow | None', 'sort': 'GetMarketLeaderboardRequestSort | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetMarketLeaderboardRequest(_GetMarketLeaderboardRequestOptional):
    coin: str

_GetMarketLeaderboardResponseDataItemOptional = TypedDict("_GetMarketLeaderboardResponseDataItemOptional", {'rank': 'int | None', 'address': 'str | None', 'display_name': 'str | None', 'net_pnl': 'str | None', 'realized_pnl': 'str | None', 'unrealized_pnl': 'str | None', 'capital': 'str | None', 'roi': 'str | None', 'realized_roi': 'str | None', 'unrealized_roi': 'str | None', 'gross_pnl': 'str | None', 'funding_pnl': 'str | None', 'fees': 'str | None', 'volume': 'str | None', 'trades': 'int | None', 'winning_trades': 'int | None', 'losing_trades': 'int | None', 'account_value': 'str | None', 'account_value_as_of': 'int | None', 'account_value_source': 'str | None', 'tag': 'str | None', 'last_trade_at': 'int | None', 'label': 'str | None', 'avatar': 'str | None', 'twitter': 'str | None', 'verified': 'bool | None', 'copy_score': 'str | None', 'mark_quality': 'int | None', 'window_start': 'int | None', 'as_of': 'int | None'}, total=False)
class GetMarketLeaderboardResponseDataItem(_GetMarketLeaderboardResponseDataItemOptional):
    pass

class GetMarketLeaderboardResponse(TypedDict):
    data: list[GetMarketLeaderboardResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetMarketParticipationRequestOptional = TypedDict("_GetMarketParticipationRequestOptional", {'coin': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetMarketParticipationRequest(_GetMarketParticipationRequestOptional):
    pass

_GetMarketParticipationResponseDataItemOptional = TypedDict("_GetMarketParticipationResponseDataItemOptional", {'market': 'str | None', 'positions': 'int | None', 'longs': 'int | None', 'shorts': 'int | None', 'profitable': 'int | None', 'losing': 'int | None', 'long_notional': 'str | None', 'short_notional': 'str | None', 'unrealized_pnl': 'str | None', 'snapshot_time': 'int | None'}, total=False)
class GetMarketParticipationResponseDataItem(_GetMarketParticipationResponseDataItemOptional):
    pass

class GetMarketParticipationResponse(TypedDict):
    data: list[GetMarketParticipationResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetMarketPositionChangesRequestOptional = TypedDict("_GetMarketPositionChangesRequestOptional", {'window': 'GetMarketPositionChangesRequestWindow | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetMarketPositionChangesRequest(_GetMarketPositionChangesRequestOptional):
    coin: str

_GetMarketPositionChangesResponseDataItemOptional = TypedDict("_GetMarketPositionChangesResponseDataItemOptional", {'address': 'str | None', 'previous': 'str | None', 'current': 'str | None', 'delta': 'str | None'}, total=False)
class GetMarketPositionChangesResponseDataItem(_GetMarketPositionChangesResponseDataItemOptional):
    pass

class GetMarketPositionChangesResponse(TypedDict):
    data: list[GetMarketPositionChangesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetMarketPositionsRequestOptional = TypedDict("_GetMarketPositionsRequestOptional", {'side': 'GetMarketPositionsRequestSide | None', 'sort': 'GetMarketPositionsRequestSort | None', 'order': 'GetMarketPositionsRequestOrder | None', 'min_notional': 'str | None', 'max_notional': 'str | None', 'min_entry_px': 'str | None', 'max_entry_px': 'str | None', 'min_liquidation_px': 'str | None', 'max_liquidation_px': 'str | None', 'min_unrealized_pnl': 'str | None', 'max_unrealized_pnl': 'str | None', 'min_account_value': 'str | None', 'max_account_value': 'str | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetMarketPositionsRequest(_GetMarketPositionsRequestOptional):
    coin: str

_GetMarketPositionsResponseDataItemOptional = TypedDict("_GetMarketPositionsResponseDataItemOptional", {'address': 'str | None', 'side': 'str | None', 'size': 'str | None', 'notional': 'str | None', 'entry_px': 'str | None', 'liquidation_px': 'str | None', 'unrealized_pnl': 'str | None', 'funding_pnl': 'str | None', 'account_value': 'str | None', 'leverage': 'int | None', 'leverage_type': 'str | None', 'margin_uncertain': 'bool | None'}, total=False)
class GetMarketPositionsResponseDataItem(_GetMarketPositionsResponseDataItemOptional):
    pass

class GetMarketPositionsResponse(TypedDict):
    data: list[GetMarketPositionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetMarketStopOrdersRequestOptional = TypedDict("_GetMarketStopOrdersRequestOptional", {'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetMarketStopOrdersRequest(_GetMarketStopOrdersRequestOptional):
    coin: str

_GetMarketStopOrdersResponseDataItemOptional = TypedDict("_GetMarketStopOrdersResponseDataItemOptional", {'address': 'str | None', 'side': 'str | None', 'trigger_px': 'str | None', 'size': 'str | None', 'notional': 'str | None', 'take_profit': 'bool | None', 'position_sized': 'bool | None'}, total=False)
class GetMarketStopOrdersResponseDataItem(_GetMarketStopOrdersResponseDataItemOptional):
    pass

class GetMarketStopOrdersResponse(TypedDict):
    data: list[GetMarketStopOrdersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetMarketTradesRequestOptional = TypedDict("_GetMarketTradesRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetMarketTradesRequest(_GetMarketTradesRequestOptional):
    coin: str

_GetMarketTradesResponseDataItemOptional = TypedDict("_GetMarketTradesResponseDataItemOptional", {'time': 'int | None', 'tid': 'str | None', 'side': 'str | None', 'px': 'str | None', 'sz': 'str | None', 'hash': 'str | None', 'taker': 'str | None', 'maker': 'str | None', 'taker_builder': 'str | None', 'taker_builder_fee': 'str | None', 'maker_builder': 'str | None', 'maker_builder_fee': 'str | None'}, total=False)
class GetMarketTradesResponseDataItem(_GetMarketTradesResponseDataItemOptional):
    pass

class GetMarketTradesResponse(TypedDict):
    data: list[GetMarketTradesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetMarketTwapHistoryRequestOptional = TypedDict("_GetMarketTwapHistoryRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetMarketTwapHistoryRequest(_GetMarketTwapHistoryRequestOptional):
    coin: str

_GetMarketTwapHistoryResponseDataItemOptional = TypedDict("_GetMarketTwapHistoryResponseDataItemOptional", {'time': 'int | None', 'user': 'str | None', 'twap_id': 'int | None', 'status': 'str | None', 'error': 'str | None', 'side': 'str | None', 'sz': 'str | None', 'executed_sz': 'str | None', 'executed_ntl': 'str | None', 'minutes': 'int | None', 'reduce_only': 'bool | None', 'randomize': 'bool | None', 'created_time': 'int | None', 'trigger': 'str | None', 'stop_px': 'str | None', 'block': 'int | None', 'event_index': 'int | None'}, total=False)
class GetMarketTwapHistoryResponseDataItem(_GetMarketTwapHistoryResponseDataItemOptional):
    pass

class GetMarketTwapHistoryResponse(TypedDict):
    data: list[GetMarketTwapHistoryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetMarketTwapLevelsRequestOptional = TypedDict("_GetMarketTwapLevelsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetMarketTwapLevelsRequest(_GetMarketTwapLevelsRequestOptional):
    coin: str

_GetMarketTwapLevelsResponseDataItemOptional = TypedDict("_GetMarketTwapLevelsResponseDataItemOptional", {'px': 'str | None', 'kind': 'str | None', 'side': 'str | None', 'when': 'str | None', 'twaps': 'int | None', 'remaining_sz': 'str | None', 'remaining_notional': 'str | None', 'flow_sz_per_min': 'str | None'}, total=False)
class GetMarketTwapLevelsResponseDataItem(_GetMarketTwapLevelsResponseDataItemOptional):
    pass

class GetMarketTwapLevelsResponse(TypedDict):
    data: list[GetMarketTwapLevelsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetMeResponseCreditsOptional = TypedDict("_GetMeResponseCreditsOptional", {'included': 'int | None', 'used': 'int | None', 'reserved': 'int | None', 'topup_balance': 'int | None', 'remaining': 'int | None', 'low': 'bool | None'}, total=False)
class GetMeResponseCredits(_GetMeResponseCreditsOptional):
    pass

_GetMeResponseOptional = TypedDict("_GetMeResponseOptional", {'scopes': 'list[str] | None', 'credits': 'GetMeResponseCredits | None'}, total=False)
class GetMeResponse(_GetMeResponseOptional):
    pass

_GetNetPositioningByMarketRequestOptional = TypedDict("_GetNetPositioningByMarketRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetNetPositioningByMarketRequest(_GetNetPositioningByMarketRequestOptional):
    pass

_GetNetPositioningByMarketResponseDataItemOptional = TypedDict("_GetNetPositioningByMarketResponseDataItemOptional", {'coin': 'str | None', 'long_notional': 'str | None', 'short_notional': 'str | None', 'net_notional': 'str | None', 'long_positions': 'int | None', 'short_positions': 'int | None', 'snapshot_time': 'int | None'}, total=False)
class GetNetPositioningByMarketResponseDataItem(_GetNetPositioningByMarketResponseDataItemOptional):
    pass

class GetNetPositioningByMarketResponse(TypedDict):
    data: list[GetNetPositioningByMarketResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetNetPositioningSummaryRequestOptional = TypedDict("_GetNetPositioningSummaryRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetNetPositioningSummaryRequest(_GetNetPositioningSummaryRequestOptional):
    pass

_GetNetPositioningSummaryResponseDataItemOptional = TypedDict("_GetNetPositioningSummaryResponseDataItemOptional", {'long_notional': 'str | None', 'short_notional': 'str | None', 'net_notional': 'str | None', 'accounts': 'int | None', 'snapshot_time': 'int | None'}, total=False)
class GetNetPositioningSummaryResponseDataItem(_GetNetPositioningSummaryResponseDataItemOptional):
    pass

class GetNetPositioningSummaryResponse(TypedDict):
    data: list[GetNetPositioningSummaryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetNewTradersRequestOptional = TypedDict("_GetNewTradersRequestOptional", {'market': 'str | None', 'interval': 'GetNewTradersRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetNewTradersRequest(_GetNewTradersRequestOptional):
    pass

_GetNewTradersResponseDataItemOptional = TypedDict("_GetNewTradersResponseDataItemOptional", {'t': 'int | None', 'traders': 'int | None'}, total=False)
class GetNewTradersResponseDataItem(_GetNewTradersResponseDataItemOptional):
    pass

class GetNewTradersResponse(TypedDict):
    data: list[GetNewTradersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetOpenInterestRequestOptional = TypedDict("_GetOpenInterestRequestOptional", {'interval': 'GetOpenInterestRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetOpenInterestRequest(_GetOpenInterestRequestOptional):
    coin: str

_GetOpenInterestResponseDataItemOptional = TypedDict("_GetOpenInterestResponseDataItemOptional", {'t': 'int | None', 'open': 'str | None', 'high': 'str | None', 'low': 'str | None', 'close': 'str | None', 'minutes': 'int | None', 'source_rank': 'int | None', 'quality': 'int | None'}, total=False)
class GetOpenInterestResponseDataItem(_GetOpenInterestResponseDataItemOptional):
    pass

class GetOpenInterestResponse(TypedDict):
    data: list[GetOpenInterestResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetOpenInterestByCategoryRequestOptional = TypedDict("_GetOpenInterestByCategoryRequestOptional", {'interval': 'GetOpenInterestByCategoryRequestInterval | None', 'days': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetOpenInterestByCategoryRequest(_GetOpenInterestByCategoryRequestOptional):
    pass

_GetOpenInterestByCategoryResponseDataItemOptional = TypedDict("_GetOpenInterestByCategoryResponseDataItemOptional", {'t': 'int | None', 'category': 'str | None', 'open_interest_usd': 'str | None', 'markets': 'int | None'}, total=False)
class GetOpenInterestByCategoryResponseDataItem(_GetOpenInterestByCategoryResponseDataItemOptional):
    pass

class GetOpenInterestByCategoryResponse(TypedDict):
    data: list[GetOpenInterestByCategoryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetOpenInterestByDexRequestOptional = TypedDict("_GetOpenInterestByDexRequestOptional", {'interval': 'GetOpenInterestByDexRequestInterval | None', 'days': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetOpenInterestByDexRequest(_GetOpenInterestByDexRequestOptional):
    pass

_GetOpenInterestByDexResponseDataItemOptional = TypedDict("_GetOpenInterestByDexResponseDataItemOptional", {'t': 'int | None', 'dex': 'str | None', 'open_interest_usd': 'str | None', 'markets': 'int | None'}, total=False)
class GetOpenInterestByDexResponseDataItem(_GetOpenInterestByDexResponseDataItemOptional):
    pass

class GetOpenInterestByDexResponse(TypedDict):
    data: list[GetOpenInterestByDexResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetOpenInterestChangesRequestOptional = TypedDict("_GetOpenInterestChangesRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetOpenInterestChangesRequest(_GetOpenInterestChangesRequestOptional):
    pass

_GetOpenInterestChangesResponseDataItemOptional = TypedDict("_GetOpenInterestChangesResponseDataItemOptional", {'coin': 'str | None', 'snapshot_time': 'int | None', 'oi_usd': 'str | None', 'mark_px': 'str | None', 'oi_usd_24h': 'str | None', 'oi_usd_7d': 'str | None', 'oi_usd_30d': 'str | None'}, total=False)
class GetOpenInterestChangesResponseDataItem(_GetOpenInterestChangesResponseDataItemOptional):
    pass

class GetOpenInterestChangesResponse(TypedDict):
    data: list[GetOpenInterestChangesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetOpenInterestDailyRequestOptional = TypedDict("_GetOpenInterestDailyRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetOpenInterestDailyRequest(_GetOpenInterestDailyRequestOptional):
    coin: str

_GetOpenInterestDailyResponseDataItemOptional = TypedDict("_GetOpenInterestDailyResponseDataItemOptional", {'day': 'int | None', 'oi_usd': 'str | None', 'mark_px': 'str | None', 'snapshot_time': 'int | None'}, total=False)
class GetOpenInterestDailyResponseDataItem(_GetOpenInterestDailyResponseDataItemOptional):
    pass

class GetOpenInterestDailyResponse(TypedDict):
    data: list[GetOpenInterestDailyResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetOpenInterestTotalRequestOptional = TypedDict("_GetOpenInterestTotalRequestOptional", {'market': 'str | None', 'interval': 'GetOpenInterestTotalRequestInterval | None', 'days': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetOpenInterestTotalRequest(_GetOpenInterestTotalRequestOptional):
    pass

_GetOpenInterestTotalResponseDataItemOptional = TypedDict("_GetOpenInterestTotalResponseDataItemOptional", {'t': 'int | None', 'open_interest_usd': 'str | None', 'markets': 'int | None'}, total=False)
class GetOpenInterestTotalResponseDataItem(_GetOpenInterestTotalResponseDataItemOptional):
    pass

class GetOpenInterestTotalResponse(TypedDict):
    data: list[GetOpenInterestTotalResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetOrderBookRequestOptional = TypedDict("_GetOrderBookRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetOrderBookRequest(_GetOrderBookRequestOptional):
    coin: str

_GetOrderBookResponseDataItemOptional = TypedDict("_GetOrderBookResponseDataItemOptional", {'side': 'str | None', 'level': 'int | None', 'px': 'str | None', 'sz': 'str | None', 'orders': 'int | None'}, total=False)
class GetOrderBookResponseDataItem(_GetOrderBookResponseDataItemOptional):
    pass

class GetOrderBookResponse(TypedDict):
    data: list[GetOrderBookResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetOrderBookAtRequestOptional = TypedDict("_GetOrderBookAtRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetOrderBookAtRequest(_GetOrderBookAtRequestOptional):
    coin: str
    time: int

_GetOrderBookAtResponseDataItemOptional = TypedDict("_GetOrderBookAtResponseDataItemOptional", {'side': 'str | None', 'level': 'int | None', 'px': 'str | None', 'sz': 'str | None'}, total=False)
class GetOrderBookAtResponseDataItem(_GetOrderBookAtResponseDataItemOptional):
    pass

class GetOrderBookAtResponse(TypedDict):
    data: list[GetOrderBookAtResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetOutcomeActivityRequestOptional = TypedDict("_GetOutcomeActivityRequestOptional", {'outcome': 'int | None', 'interval': 'GetOutcomeActivityRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetOutcomeActivityRequest(_GetOutcomeActivityRequestOptional):
    pass

_GetOutcomeActivityResponseDataItemOptional = TypedDict("_GetOutcomeActivityResponseDataItemOptional", {'t': 'int | None', 'taker_notional': 'str | None', 'fees': 'str | None', 'taker_fills': 'int | None', 'traders': 'int | None'}, total=False)
class GetOutcomeActivityResponseDataItem(_GetOutcomeActivityResponseDataItemOptional):
    pass

class GetOutcomeActivityResponse(TypedDict):
    data: list[GetOutcomeActivityResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetOutcomeHoldersRequestOptional = TypedDict("_GetOutcomeHoldersRequestOptional", {'side': 'int | None', 'sort': 'GetOutcomeHoldersRequestSort | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetOutcomeHoldersRequest(_GetOutcomeHoldersRequestOptional):
    outcome: int

_GetOutcomeHoldersResponseDataItemOptional = TypedDict("_GetOutcomeHoldersResponseDataItemOptional", {'address': 'str | None', 'side_index': 'int | None', 'shares': 'str | None', 'cost_basis': 'str | None', 'avg_entry': 'str | None', 'realized_pnl': 'str | None', 'fills': 'int | None', 'last_fill_at': 'int | None', 'balance_status': 'str | None', 'balance_checked_at': 'int | None', 'mid_px': 'str | None', 'notional': 'str | None', 'unrealized_pnl': 'str | None', 'display_name': 'str | None', 'avatar': 'str | None', 'verified': 'bool | None', 'share_fees': 'str | None', 'account_value': 'str | None'}, total=False)
class GetOutcomeHoldersResponseDataItem(_GetOutcomeHoldersResponseDataItemOptional):
    pass

class GetOutcomeHoldersResponse(TypedDict):
    data: list[GetOutcomeHoldersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetOutcomeLeaderboardRequestOptional = TypedDict("_GetOutcomeLeaderboardRequestOptional", {'hours': 'GetOutcomeLeaderboardRequestHours | None', 'outcome': 'int | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetOutcomeLeaderboardRequest(_GetOutcomeLeaderboardRequestOptional):
    pass

_GetOutcomeLeaderboardResponseDataItemOptional = TypedDict("_GetOutcomeLeaderboardResponseDataItemOptional", {'address': 'str | None', 'realized_pnl': 'str | None', 'gross_pnl': 'str | None', 'fees': 'str | None', 'yes_notional': 'str | None', 'fills': 'int | None', 'markets': 'int | None', 'last_trade_at': 'int | None', 'share_fees': 'str | None', 'taker_share_volume': 'str | None', 'account_value': 'str | None', 'display_name': 'str | None', 'avatar': 'str | None', 'twitter': 'str | None', 'verified': 'bool | None'}, total=False)
class GetOutcomeLeaderboardResponseDataItem(_GetOutcomeLeaderboardResponseDataItemOptional):
    pass

class GetOutcomeLeaderboardResponse(TypedDict):
    data: list[GetOutcomeLeaderboardResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetOutcomeProbabilityRequestOptional = TypedDict("_GetOutcomeProbabilityRequestOptional", {'interval': 'GetOutcomeProbabilityRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetOutcomeProbabilityRequest(_GetOutcomeProbabilityRequestOptional):
    outcome: int

_GetOutcomeProbabilityResponseDataItemOptional = TypedDict("_GetOutcomeProbabilityResponseDataItemOptional", {'t': 'int | None', 'mark_px': 'str | None', 'mid_px': 'str | None', 'day_volume': 'str | None', 'last_time': 'int | None', 'trade_px': 'str | None', 'last_trade_time': 'int | None', 'open_interest': 'str | None'}, total=False)
class GetOutcomeProbabilityResponseDataItem(_GetOutcomeProbabilityResponseDataItemOptional):
    pass

class GetOutcomeProbabilityResponse(TypedDict):
    data: list[GetOutcomeProbabilityResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetPnlLeaderboardRequestOptional = TypedDict("_GetPnlLeaderboardRequestOptional", {'window': 'GetPnlLeaderboardRequestWindow | None', 'category': 'GetPnlLeaderboardRequestCategory | None', 'sort': 'GetPnlLeaderboardRequestSort | None', 'min_trades': 'int | None', 'min_account_value': 'GetPnlLeaderboardRequestMinAccountValue | None', 'max_account_value': 'GetPnlLeaderboardRequestMaxAccountValue | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetPnlLeaderboardRequest(_GetPnlLeaderboardRequestOptional):
    pass

_GetPnlLeaderboardResponseDataItemOptional = TypedDict("_GetPnlLeaderboardResponseDataItemOptional", {'rank': 'int | None', 'address': 'str | None', 'display_name': 'str | None', 'tag': 'str | None', 'net_pnl': 'str | None', 'realized_pnl': 'str | None', 'unrealized_pnl': 'str | None', 'capital': 'str | None', 'roi': 'str | None', 'realized_roi': 'str | None', 'unrealized_roi': 'str | None', 'gross_pnl': 'str | None', 'funding_pnl': 'str | None', 'total_fees': 'str | None', 'volume': 'str | None', 'trades': 'int | None', 'winning_trades': 'int | None', 'losing_trades': 'int | None', 'account_value': 'str | None', 'account_value_as_of': 'int | None', 'account_value_source': 'str | None', 'last_trade_at': 'int | None', 'label': 'str | None', 'avatar': 'str | None', 'twitter': 'str | None', 'verified': 'bool | None', 'copy_score': 'str | None', 'mark_quality': 'int | None', 'window_start': 'int | None', 'as_of': 'int | None'}, total=False)
class GetPnlLeaderboardResponseDataItem(_GetPnlLeaderboardResponseDataItemOptional):
    pass

class GetPnlLeaderboardResponse(TypedDict):
    data: list[GetPnlLeaderboardResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetPositionCountsRequestOptional = TypedDict("_GetPositionCountsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetPositionCountsRequest(_GetPositionCountsRequestOptional):
    coin: str

_GetPositionCountsResponseDataItemOptional = TypedDict("_GetPositionCountsResponseDataItemOptional", {'long_positions': 'int | None', 'short_positions': 'int | None', 'long_notional': 'str | None', 'short_notional': 'str | None'}, total=False)
class GetPositionCountsResponseDataItem(_GetPositionCountsResponseDataItemOptional):
    pass

class GetPositionCountsResponse(TypedDict):
    data: list[GetPositionCountsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetPositioningRequestOptional = TypedDict("_GetPositioningRequestOptional", {'market': 'str | None', 'interval': 'GetPositioningRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetPositioningRequest(_GetPositioningRequestOptional):
    pass

_GetPositioningResponseDataItemOptional = TypedDict("_GetPositioningResponseDataItemOptional", {'t': 'int | None', 'snapshot_time': 'int | None', 'long_notional': 'str | None', 'short_notional': 'str | None', 'long_upnl': 'str | None', 'short_upnl': 'str | None', 'long_positions': 'int | None', 'short_positions': 'int | None'}, total=False)
class GetPositioningResponseDataItem(_GetPositioningResponseDataItemOptional):
    pass

class GetPositioningResponse(TypedDict):
    data: list[GetPositioningResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetPriceChangesRequestOptional = TypedDict("_GetPriceChangesRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetPriceChangesRequest(_GetPriceChangesRequestOptional):
    pass

_GetPriceChangesResponseDataItemOptional = TypedDict("_GetPriceChangesResponseDataItemOptional", {'coin': 'str | None', 'price': 'str | None', 'price_source': 'str | None', 'change_24h_pct': 'str | None', 'change_7d_pct': 'str | None', 'change_30d_pct': 'str | None', 'mark_px': 'str | None', 'px_24h': 'str | None', 'px_7d': 'str | None', 'px_30d': 'str | None', 'day_volume': 'str | None', 'open_interest': 'str | None'}, total=False)
class GetPriceChangesResponseDataItem(_GetPriceChangesResponseDataItemOptional):
    pass

class GetPriceChangesResponse(TypedDict):
    data: list[GetPriceChangesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetPriorityFeeSummaryRequestOptional = TypedDict("_GetPriorityFeeSummaryRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetPriorityFeeSummaryRequest(_GetPriorityFeeSummaryRequestOptional):
    pass

_GetPriorityFeeSummaryResponseDataItemOptional = TypedDict("_GetPriorityFeeSummaryResponseDataItemOptional", {'start': 'int | None', 'end': 'int | None', 'write_hype': 'str | None', 'write_usd': 'str | None', 'write_orders': 'int | None', 'read_hype': 'str | None', 'read_usd': 'str | None', 'reads': 'int | None', 'total_hype': 'str | None', 'total_usd': 'str | None', 'total_txs': 'int | None', 'avg_write_usd': 'str | None', 'avg_read_usd': 'str | None', 'write_wallets': 'int | None', 'read_wallets': 'int | None', 'wallets': 'int | None', 'usd_complete': 'bool | None', 'hype_complete': 'bool | None', 'write_ioc_hype': 'str | None', 'write_ioc_usd': 'str | None', 'write_ioc_orders': 'int | None', 'write_ioc_fills': 'int | None', 'write_ioc_unpriced_hype': 'str | None', 'write_alo_hype_estimate': 'str | None', 'write_alo_usd': 'str | None', 'write_alo_orders': 'int | None', 'write_alo_resting_notional_usd': 'str | None', 'write_unpriced_hype': 'str | None', 'unpriced_hype': 'str | None', 'unconverted_usd': 'str | None', 'hype_estimated': 'bool | None', 'live_from': 'int | None', 'write_fills': 'int | None', 'read_unpriced_hype': 'str | None', 'avg_write_ioc_hype': 'str | None', 'avg_write_ioc_usd': 'str | None', 'avg_write_alo_hype_estimate': 'str | None', 'avg_write_alo_usd': 'str | None', 'avg_write_hype': 'str | None', 'avg_read_hype': 'str | None', 'write_ioc_wallets': 'int | None', 'write_alo_wallets': 'int | None', 'last_hour': 'int | None'}, total=False)
class GetPriorityFeeSummaryResponseDataItem(_GetPriorityFeeSummaryResponseDataItemOptional):
    pass

class GetPriorityFeeSummaryResponse(TypedDict):
    data: list[GetPriorityFeeSummaryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetPriorityFeesRequestOptional = TypedDict("_GetPriorityFeesRequestOptional", {'interval': 'GetPriorityFeesRequestInterval | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetPriorityFeesRequest(_GetPriorityFeesRequestOptional):
    pass

_GetPriorityFeesResponseDataItemOptional = TypedDict("_GetPriorityFeesResponseDataItemOptional", {'t': 'int | None', 'write_hype': 'str | None', 'write_usd': 'str | None', 'write_orders': 'int | None', 'read_hype': 'str | None', 'read_usd': 'str | None', 'reads': 'int | None', 'total_hype': 'str | None', 'total_usd': 'str | None', 'complete': 'bool | None', 'usd_complete': 'bool | None', 'hype_complete': 'bool | None', 'read_unpriced_hype': 'str | None', 'total_txs': 'int | None', 'write_ioc_hype': 'str | None', 'write_ioc_usd': 'str | None', 'write_ioc_orders': 'int | None', 'write_ioc_fills': 'int | None', 'write_ioc_unpriced_hype': 'str | None', 'write_alo_hype_estimate': 'str | None', 'write_alo_usd': 'str | None', 'write_alo_orders': 'int | None', 'write_alo_resting_notional_usd': 'str | None', 'write_unpriced_hype': 'str | None', 'unpriced_hype': 'str | None', 'unconverted_usd': 'str | None', 'hype_estimated': 'bool | None', 'live_from': 'int | None', 'cumulative_write_ioc_hype': 'str | None', 'cumulative_write_ioc_usd': 'str | None', 'cumulative_write_alo_hype_estimate': 'str | None', 'cumulative_write_alo_usd': 'str | None', 'cumulative_write_hype': 'str | None', 'cumulative_write_usd': 'str | None', 'cumulative_read_hype': 'str | None', 'cumulative_read_usd': 'str | None', 'cumulative_hype': 'str | None', 'cumulative_usd': 'str | None', 'cumulative_unconverted_usd': 'str | None', 'cumulative_unpriced_hype': 'str | None', 'cumulative_hype_complete': 'bool | None', 'cumulative_hype_estimated': 'bool | None', 'cumulative_usd_complete': 'bool | None'}, total=False)
class GetPriorityFeesResponseDataItem(_GetPriorityFeesResponseDataItemOptional):
    pass

class GetPriorityFeesResponse(TypedDict):
    data: list[GetPriorityFeesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetPriorityFeesBuilderShareRequestOptional = TypedDict("_GetPriorityFeesBuilderShareRequestOptional", {'interval': 'GetPriorityFeesBuilderShareRequestInterval | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetPriorityFeesBuilderShareRequest(_GetPriorityFeesBuilderShareRequestOptional):
    pass

_GetPriorityFeesBuilderShareResponseDataItemOptional = TypedDict("_GetPriorityFeesBuilderShareResponseDataItemOptional", {'t': 'int | None', 'write_usd': 'str | None', 'hip3_write_usd': 'str | None', 'hip3_share_usd': 'str | None', 'write_hype': 'str | None', 'hip3_write_hype': 'str | None', 'hip3_share_hype': 'str | None', 'usd_complete': 'bool | None', 'hype_complete': 'bool | None', 'write_ioc_hype': 'str | None', 'write_ioc_usd': 'str | None', 'write_ioc_orders': 'int | None', 'write_ioc_fills': 'int | None', 'write_ioc_unpriced_hype': 'str | None', 'write_alo_hype_estimate': 'str | None', 'write_alo_usd': 'str | None', 'write_alo_orders': 'int | None', 'write_alo_resting_notional_usd': 'str | None', 'write_unpriced_hype': 'str | None', 'unpriced_hype': 'str | None', 'unconverted_usd': 'str | None', 'hype_estimated': 'bool | None', 'live_from': 'int | None', 'hip3_write_ioc_hype': 'str | None', 'hip3_write_ioc_usd': 'str | None', 'hip3_write_alo_hype_estimate': 'str | None', 'hip3_write_alo_usd': 'str | None', 'hip3_write_ioc_share_usd': 'str | None', 'hip3_write_ioc_share_hype': 'str | None', 'hip3_write_alo_share_usd': 'str | None', 'hip3_write_alo_share_hype_estimate': 'str | None', 'write_orders': 'int | None'}, total=False)
class GetPriorityFeesBuilderShareResponseDataItem(_GetPriorityFeesBuilderShareResponseDataItemOptional):
    pass

class GetPriorityFeesBuilderShareResponse(TypedDict):
    data: list[GetPriorityFeesBuilderShareResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetPriorityFeesByCoinRequestOptional = TypedDict("_GetPriorityFeesByCoinRequestOptional", {'kind': 'GetPriorityFeesByCoinRequestKind | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetPriorityFeesByCoinRequest(_GetPriorityFeesByCoinRequestOptional):
    pass

_GetPriorityFeesByCoinResponseDataItemOptional = TypedDict("_GetPriorityFeesByCoinResponseDataItemOptional", {'coin': 'str | None', 'dex': 'str | None', 'asset_class': 'str | None', 'write_hype': 'str | None', 'write_usd': 'str | None', 'write_orders': 'int | None', 'write_fills': 'int | None', 'share_usd': 'str | None', 'share_hype': 'str | None', 'usd_complete': 'bool | None', 'hype_complete': 'bool | None', 'write_ioc_hype': 'str | None', 'write_ioc_usd': 'str | None', 'write_ioc_orders': 'int | None', 'write_ioc_fills': 'int | None', 'write_ioc_unpriced_hype': 'str | None', 'write_alo_hype_estimate': 'str | None', 'write_alo_usd': 'str | None', 'write_alo_orders': 'int | None', 'write_alo_resting_notional_usd': 'str | None', 'write_unpriced_hype': 'str | None', 'unpriced_hype': 'str | None', 'unconverted_usd': 'str | None', 'hype_estimated': 'bool | None', 'live_from': 'int | None', 'share_hype_estimated': 'bool | None', 'share_hype_complete': 'bool | None'}, total=False)
class GetPriorityFeesByCoinResponseDataItem(_GetPriorityFeesByCoinResponseDataItemOptional):
    pass

class GetPriorityFeesByCoinResponse(TypedDict):
    data: list[GetPriorityFeesByCoinResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetPriorityFeesTopCoinsRequestOptional = TypedDict("_GetPriorityFeesTopCoinsRequestOptional", {'top': 'int | None', 'interval': 'GetPriorityFeesTopCoinsRequestInterval | None', 'kind': 'GetPriorityFeesTopCoinsRequestKind | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetPriorityFeesTopCoinsRequest(_GetPriorityFeesTopCoinsRequestOptional):
    pass

_GetPriorityFeesTopCoinsResponseDataItemOptional = TypedDict("_GetPriorityFeesTopCoinsResponseDataItemOptional", {'t': 'int | None', 'coin': 'str | None', 'dex': 'str | None', 'write_hype': 'str | None', 'write_usd': 'str | None', 'write_orders': 'int | None', 'usd_complete': 'bool | None', 'hype_complete': 'bool | None', 'write_ioc_hype': 'str | None', 'write_ioc_usd': 'str | None', 'write_ioc_orders': 'int | None', 'write_ioc_fills': 'int | None', 'write_ioc_unpriced_hype': 'str | None', 'write_alo_hype_estimate': 'str | None', 'write_alo_usd': 'str | None', 'write_alo_orders': 'int | None', 'write_alo_resting_notional_usd': 'str | None', 'write_unpriced_hype': 'str | None', 'unpriced_hype': 'str | None', 'unconverted_usd': 'str | None', 'hype_estimated': 'bool | None', 'live_from': 'int | None'}, total=False)
class GetPriorityFeesTopCoinsResponseDataItem(_GetPriorityFeesTopCoinsResponseDataItemOptional):
    pass

class GetPriorityFeesTopCoinsResponse(TypedDict):
    data: list[GetPriorityFeesTopCoinsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetPriorityFillsRequestOptional = TypedDict("_GetPriorityFillsRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetPriorityFillsRequest(_GetPriorityFillsRequestOptional):
    coin: str

_GetPriorityFillsResponseDataItemOptional = TypedDict("_GetPriorityFillsResponseDataItemOptional", {'time': 'int | None', 'user': 'str | None', 'side': 'str | None', 'tid': 'str | None', 'oid': 'str | None', 'px': 'str | None', 'sz': 'str | None', 'crossed': 'bool | None', 'priority_gas': 'str | None', 'hash': 'str | None'}, total=False)
class GetPriorityFillsResponseDataItem(_GetPriorityFillsResponseDataItemOptional):
    pass

class GetPriorityFillsResponse(TypedDict):
    data: list[GetPriorityFillsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetQuoteRequestOptional = TypedDict("_GetQuoteRequestOptional", {'range_ms': 'int | None', 'query': 'dict[str, Any] | None'}, total=False)
class GetQuoteRequest(_GetQuoteRequestOptional):
    route: str

class GetQuoteResponse(TypedDict):
    max_credits: int
    price_version: int

_GetRealizedVolatilityRequestOptional = TypedDict("_GetRealizedVolatilityRequestOptional", {'interval': 'GetRealizedVolatilityRequestInterval | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetRealizedVolatilityRequest(_GetRealizedVolatilityRequestOptional):
    coin: str

_GetRealizedVolatilityResponseDataItemOptional = TypedDict("_GetRealizedVolatilityResponseDataItemOptional", {'t': 'int | None', 'vol_pct': 'str | None', 'sum_sq_log_ret': 'str | None', 'n_returns': 'int | None'}, total=False)
class GetRealizedVolatilityResponseDataItem(_GetRealizedVolatilityResponseDataItemOptional):
    pass

class GetRealizedVolatilityResponse(TypedDict):
    data: list[GetRealizedVolatilityResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetReferralRequestOptional = TypedDict("_GetReferralRequestOptional", {'code': 'str | None', 'referrer': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetReferralRequest(_GetReferralRequestOptional):
    pass

_GetReferralResponseDataItemOptional = TypedDict("_GetReferralResponseDataItemOptional", {'referrer': 'str | None', 'code': 'str | None', 'referred_users': 'int | None', 'snapshot_time': 'int | None'}, total=False)
class GetReferralResponseDataItem(_GetReferralResponseDataItemOptional):
    pass

class GetReferralResponse(TypedDict):
    data: list[GetReferralResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetReturnsMatrixRequestOptional = TypedDict("_GetReturnsMatrixRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetReturnsMatrixRequest(_GetReturnsMatrixRequestOptional):
    pass

_GetReturnsMatrixResponseDataItemOptional = TypedDict("_GetReturnsMatrixResponseDataItemOptional", {'coin': 'str | None', 'px_now': 'str | None', 'px_1h': 'str | None', 'px_4h': 'str | None', 'px_1d': 'str | None', 'px_7d': 'str | None', 'px_30d': 'str | None'}, total=False)
class GetReturnsMatrixResponseDataItem(_GetReturnsMatrixResponseDataItemOptional):
    pass

class GetReturnsMatrixResponse(TypedDict):
    data: list[GetReturnsMatrixResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetRevenueRequestOptional = TypedDict("_GetRevenueRequestOptional", {'market': 'str | None', 'interval': 'GetRevenueRequestInterval | None', 'days': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetRevenueRequest(_GetRevenueRequestOptional):
    pass

_GetRevenueResponseDataItemOptional = TypedDict("_GetRevenueResponseDataItemOptional", {'t': 'int | None', 'fees_usd': 'str | None', 'volume_usd': 'str | None', 'trades': 'int | None'}, total=False)
class GetRevenueResponseDataItem(_GetRevenueResponseDataItemOptional):
    pass

class GetRevenueResponse(TypedDict):
    data: list[GetRevenueResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetSentimentRequestOptional = TypedDict("_GetSentimentRequestOptional", {'interval': 'GetSentimentRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetSentimentRequest(_GetSentimentRequestOptional):
    coin: str

_GetSentimentResponseDataItemOptional = TypedDict("_GetSentimentResponseDataItemOptional", {'t': 'int | None', 'close': 'str | None', 'ret': 'str | None', 'cvd': 'str | None', 'volume': 'str | None', 'rv': 'str | None', 'funding': 'str | None', 'funding_start': 'str | None', 'funding_end': 'str | None', 'funding_base': 'str | None', 'oi_usd': 'str | None', 'oi_change_pct': 'str | None', 'rv_change': 'str | None', 'liq_long': 'str | None', 'liq_short': 'str | None', 'liq_long_next': 'str | None', 'liq_short_next': 'str | None', 'z_ret': 'str | None', 'z_cvd': 'str | None', 'z_funding': 'str | None', 'z_oi': 'str | None', 'z_vol': 'str | None', 'z_funding_base': 'str | None'}, total=False)
class GetSentimentResponseDataItem(_GetSentimentResponseDataItemOptional):
    pass

class GetSentimentResponse(TypedDict):
    data: list[GetSentimentResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetSentimentEventsRequestOptional = TypedDict("_GetSentimentEventsRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetSentimentEventsRequest(_GetSentimentEventsRequestOptional):
    coin: str

_GetSentimentEventsResponseDataItemOptional = TypedDict("_GetSentimentEventsResponseDataItemOptional", {'t': 'int | None', 'close': 'str | None', 'ret': 'str | None', 'cvd': 'str | None', 'volume': 'str | None', 'rv': 'str | None', 'funding': 'str | None', 'funding_start': 'str | None', 'funding_end': 'str | None', 'funding_base': 'str | None', 'oi_usd': 'str | None', 'oi_change_pct': 'str | None', 'rv_change': 'str | None', 'liq_long': 'str | None', 'liq_short': 'str | None', 'liq_long_next': 'str | None', 'liq_short_next': 'str | None', 'z_ret': 'str | None', 'z_cvd': 'str | None', 'z_funding': 'str | None', 'z_oi': 'str | None', 'z_vol': 'str | None', 'z_funding_base': 'str | None'}, total=False)
class GetSentimentEventsResponseDataItem(_GetSentimentEventsResponseDataItemOptional):
    pass

class GetSentimentEventsResponse(TypedDict):
    data: list[GetSentimentEventsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetSentimentScannerRequestOptional = TypedDict("_GetSentimentScannerRequestOptional", {'interval': 'GetSentimentScannerRequestInterval | None', 'view': 'GetSentimentScannerRequestView | None', 'asset_class': 'str | None', 'coins': 'str | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetSentimentScannerRequest(_GetSentimentScannerRequestOptional):
    pass

_GetSentimentScannerResponseDataItemOptional = TypedDict("_GetSentimentScannerResponseDataItemOptional", {'market': 'str | None', 't': 'int | None', 'z_ret': 'str | None', 'z_cvd': 'str | None', 'z_funding': 'str | None', 'z_oi': 'str | None', 'z_vol': 'str | None', 'z_funding_base': 'str | None', 'funding_change': 'str | None', 'funding_end': 'str | None', 'cvd': 'str | None', 'volume': 'str | None', 'rv': 'str | None', 'px_change': 'str | None', 'oi_change': 'str | None', 'last_return': 'str | None'}, total=False)
class GetSentimentScannerResponseDataItem(_GetSentimentScannerResponseDataItemOptional):
    pass

class GetSentimentScannerResponse(TypedDict):
    data: list[GetSentimentScannerResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetSessionReturnsRequestOptional = TypedDict("_GetSessionReturnsRequestOptional", {'market': 'str | None', 'days': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetSessionReturnsRequest(_GetSessionReturnsRequestOptional):
    pass

_GetSessionReturnsResponseDataItemOptional = TypedDict("_GetSessionReturnsResponseDataItemOptional", {'coin': 'str | None', 'session': 'str | None', 'return_pct': 'str | None', 'hours': 'int | None'}, total=False)
class GetSessionReturnsResponseDataItem(_GetSessionReturnsResponseDataItemOptional):
    pass

class GetSessionReturnsResponse(TypedDict):
    data: list[GetSessionReturnsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetSortedLeaderboardRequestOptional = TypedDict("_GetSortedLeaderboardRequestOptional", {'window': 'GetSortedLeaderboardRequestWindow | None', 'sort': 'GetSortedLeaderboardRequestSort | None', 'category': 'GetSortedLeaderboardRequestCategory | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetSortedLeaderboardRequest(_GetSortedLeaderboardRequestOptional):
    pass

_GetSortedLeaderboardResponseDataItemOptional = TypedDict("_GetSortedLeaderboardResponseDataItemOptional", {'rank': 'int | None', 'address': 'str | None', 'net_pnl': 'str | None', 'gross_pnl': 'str | None', 'funding_pnl': 'str | None', 'fees': 'str | None', 'volume': 'str | None', 'trades': 'int | None', 'winning_trades': 'int | None', 'losing_trades': 'int | None', 'win_rate': 'str | None', 'last_trade_at': 'int | None', 'tag': 'str | None', 'refreshed_at': 'int | None'}, total=False)
class GetSortedLeaderboardResponseDataItem(_GetSortedLeaderboardResponseDataItemOptional):
    pass

class GetSortedLeaderboardResponse(TypedDict):
    data: list[GetSortedLeaderboardResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetSpotTwapLevelsRequestOptional = TypedDict("_GetSpotTwapLevelsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetSpotTwapLevelsRequest(_GetSpotTwapLevelsRequestOptional):
    token: str

_GetSpotTwapLevelsResponseDataItemOptional = TypedDict("_GetSpotTwapLevelsResponseDataItemOptional", {'px': 'str | None', 'kind': 'str | None', 'side': 'str | None', 'when': 'str | None', 'pair': 'str | None', 'twaps': 'int | None', 'remaining_sz': 'str | None', 'remaining_notional': 'str | None', 'flow_sz_per_min': 'str | None'}, total=False)
class GetSpotTwapLevelsResponseDataItem(_GetSpotTwapLevelsResponseDataItemOptional):
    pass

class GetSpotTwapLevelsResponse(TypedDict):
    data: list[GetSpotTwapLevelsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetStakeHistoryRequestOptional = TypedDict("_GetStakeHistoryRequestOptional", {'interval': 'GetStakeHistoryRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetStakeHistoryRequest(_GetStakeHistoryRequestOptional):
    pass

_GetStakeHistoryResponseDataItemOptional = TypedDict("_GetStakeHistoryResponseDataItemOptional", {'t': 'int | None', 'total_stake': 'str | None', 'active_stake': 'str | None', 'validators': 'int | None', 'snapshot_time': 'int | None'}, total=False)
class GetStakeHistoryResponseDataItem(_GetStakeHistoryResponseDataItemOptional):
    pass

class GetStakeHistoryResponse(TypedDict):
    data: list[GetStakeHistoryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetStakerBucketsRequestOptional = TypedDict("_GetStakerBucketsRequestOptional", {'time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetStakerBucketsRequest(_GetStakerBucketsRequestOptional):
    pass

_GetStakerBucketsResponseDataItemOptional = TypedDict("_GetStakerBucketsResponseDataItemOptional", {'bucket': 'str | None', 'wallets': 'int | None', 'stake_total': 'str | None'}, total=False)
class GetStakerBucketsResponseDataItem(_GetStakerBucketsResponseDataItemOptional):
    pass

class GetStakerBucketsResponse(TypedDict):
    data: list[GetStakerBucketsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetStakingFlowsRequestOptional = TypedDict("_GetStakingFlowsRequestOptional", {'interval': 'GetStakingFlowsRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetStakingFlowsRequest(_GetStakingFlowsRequestOptional):
    pass

_GetStakingFlowsResponseDataItemOptional = TypedDict("_GetStakingFlowsResponseDataItemOptional", {'t': 'int | None', 'staked': 'str | None', 'unstaked': 'str | None', 'stake_transfers': 'int | None', 'unstake_transfers': 'int | None', 'stakers': 'int | None', 'unstakers': 'int | None'}, total=False)
class GetStakingFlowsResponseDataItem(_GetStakingFlowsResponseDataItemOptional):
    pass

class GetStakingFlowsResponse(TypedDict):
    data: list[GetStakingFlowsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetStatusResponseDatasetsItemOptional = TypedDict("_GetStatusResponseDatasetsItemOptional", {'id': 'str | None', 'complete_to': 'int | None', 'lag_ms': 'int | None', 'health': 'str | None'}, total=False)
class GetStatusResponseDatasetsItem(_GetStatusResponseDatasetsItemOptional):
    pass

class GetStatusResponse(TypedDict):
    as_of: int
    datasets: list[GetStatusResponseDatasetsItem]

_GetStopHeatmapRequestOptional = TypedDict("_GetStopHeatmapRequestOptional", {'interval': 'GetStopHeatmapRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetStopHeatmapRequest(_GetStopHeatmapRequestOptional):
    coin: str

_GetStopHeatmapResponseDataItemOptional = TypedDict("_GetStopHeatmapResponseDataItemOptional", {'t': 'int | None', 'band_min': 'str | None', 'band_max': 'str | None', 'buy_size': 'str | None', 'sell_size': 'str | None', 'orders': 'int | None', 'mark_px': 'str | None', 'band_size': 'str | None'}, total=False)
class GetStopHeatmapResponseDataItem(_GetStopHeatmapResponseDataItemOptional):
    pass

class GetStopHeatmapResponse(TypedDict):
    data: list[GetStopHeatmapResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetStopLevelTotalsRequestOptional = TypedDict("_GetStopLevelTotalsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetStopLevelTotalsRequest(_GetStopLevelTotalsRequestOptional):
    coin: str

_GetStopLevelTotalsResponseDataItemOptional = TypedDict("_GetStopLevelTotalsResponseDataItemOptional", {'buy_size': 'str | None', 'buy_orders': 'int | None', 'sell_size': 'str | None', 'sell_orders': 'int | None'}, total=False)
class GetStopLevelTotalsResponseDataItem(_GetStopLevelTotalsResponseDataItemOptional):
    pass

class GetStopLevelTotalsResponse(TypedDict):
    data: list[GetStopLevelTotalsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetStopLevelsRequestOptional = TypedDict("_GetStopLevelsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetStopLevelsRequest(_GetStopLevelsRequestOptional):
    coin: str

_GetStopLevelsResponseDataItemOptional = TypedDict("_GetStopLevelsResponseDataItemOptional", {'side': 'str | None', 'px': 'str | None', 'notional': 'str | None', 'size': 'str | None', 'orders': 'int | None'}, total=False)
class GetStopLevelsResponseDataItem(_GetStopLevelsResponseDataItemOptional):
    pass

class GetStopLevelsResponse(TypedDict):
    data: list[GetStopLevelsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetStopOrdersAtRequestOptional = TypedDict("_GetStopOrdersAtRequestOptional", {'min_px': 'str | None', 'max_px': 'str | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetStopOrdersAtRequest(_GetStopOrdersAtRequestOptional):
    coin: str
    time: int

_GetStopOrdersAtResponseDataItemOptional = TypedDict("_GetStopOrdersAtResponseDataItemOptional", {'address': 'str | None', 'px': 'str | None', 'size': 'str | None', 'notional': 'str | None', 'snapshot_time': 'int | None'}, total=False)
class GetStopOrdersAtResponseDataItem(_GetStopOrdersAtResponseDataItemOptional):
    pass

class GetStopOrdersAtResponse(TypedDict):
    data: list[GetStopOrdersAtResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetStopSummaryRequestOptional = TypedDict("_GetStopSummaryRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetStopSummaryRequest(_GetStopSummaryRequestOptional):
    pass

_GetStopSummaryResponseDataItemOptional = TypedDict("_GetStopSummaryResponseDataItemOptional", {'coin': 'str | None', 'snapshot_time': 'int | None', 'mark_px': 'str | None', 'mark_time': 'int | None', 'stop_notional': 'str | None', 'tp_notional': 'str | None', 'stop_notional_band': 'str | None', 'tp_notional_band': 'str | None', 'long_sl_band': 'str | None', 'short_sl_band': 'str | None', 'long_tp_band': 'str | None', 'short_tp_band': 'str | None', 'total_notional': 'str | None', 'stop_levels': 'int | None', 'tp_levels': 'int | None', 'largest_level': 'str | None', 'largest_level_px': 'str | None'}, total=False)
class GetStopSummaryResponseDataItem(_GetStopSummaryResponseDataItemOptional):
    pass

class GetStopSummaryResponse(TypedDict):
    data: list[GetStopSummaryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetTokenHolderBandsRequestOptional = TypedDict("_GetTokenHolderBandsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetTokenHolderBandsRequest(_GetTokenHolderBandsRequestOptional):
    token: str

_GetTokenHolderBandsResponseDataItemOptional = TypedDict("_GetTokenHolderBandsResponseDataItemOptional", {'period': 'str | None', 'snapshot_time': 'int | None', 'band': 'str | None', 'wallets': 'int | None', 'held': 'str | None'}, total=False)
class GetTokenHolderBandsResponseDataItem(_GetTokenHolderBandsResponseDataItemOptional):
    pass

class GetTokenHolderBandsResponse(TypedDict):
    data: list[GetTokenHolderBandsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetTokenSupplyRequestOptional = TypedDict("_GetTokenSupplyRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetTokenSupplyRequest(_GetTokenSupplyRequestOptional):
    token_id: int

_GetTokenSupplyResponseDataItemOptional = TypedDict("_GetTokenSupplyResponseDataItemOptional", {'token_id': 'int | None', 'name': 'str | None', 'full_name': 'str | None', 'sz_decimals': 'int | None', 'wei_decimals': 'int | None', 'evm_contract': 'str | None', 'evm_extra_wei_decimals': 'int | None', 'deployer': 'str | None', 'deploy_time': 'int | None', 'deploy_gas_wei': 'str | None', 'deploy_gas_token': 'int | None', 'deployer_trading_fee_share': 'str | None', 'max_supply_wei': 'str | None', 'balances_total_wei': 'str | None', 'non_circulating_balance_wei': 'str | None', 'non_circulating_users': 'int | None', 'nc_spot_wei': 'str | None', 'staking_delegated_wei': 'str | None', 'snapshot_time': 'int | None'}, total=False)
class GetTokenSupplyResponseDataItem(_GetTokenSupplyResponseDataItemOptional):
    pass

class GetTokenSupplyResponse(TypedDict):
    data: list[GetTokenSupplyResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetTopLiquidationLevelsRequestOptional = TypedDict("_GetTopLiquidationLevelsRequestOptional", {'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetTopLiquidationLevelsRequest(_GetTopLiquidationLevelsRequestOptional):
    coin: str

_GetTopLiquidationLevelsResponseDataItemOptional = TypedDict("_GetTopLiquidationLevelsResponseDataItemOptional", {'address': 'str | None', 'side': 'str | None', 'liquidation_px': 'str | None', 'size': 'str | None'}, total=False)
class GetTopLiquidationLevelsResponseDataItem(_GetTopLiquidationLevelsResponseDataItemOptional):
    pass

class GetTopLiquidationLevelsResponse(TypedDict):
    data: list[GetTopLiquidationLevelsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetTopStopLevelsRequestOptional = TypedDict("_GetTopStopLevelsRequestOptional", {'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetTopStopLevelsRequest(_GetTopStopLevelsRequestOptional):
    coin: str

_GetTopStopLevelsResponseDataItemOptional = TypedDict("_GetTopStopLevelsResponseDataItemOptional", {'address': 'str | None', 'side': 'str | None', 'trigger_px': 'str | None', 'size': 'str | None'}, total=False)
class GetTopStopLevelsResponseDataItem(_GetTopStopLevelsResponseDataItemOptional):
    pass

class GetTopStopLevelsResponse(TypedDict):
    data: list[GetTopStopLevelsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetTopTraderConsensusRequestOptional = TypedDict("_GetTopTraderConsensusRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetTopTraderConsensusRequest(_GetTopTraderConsensusRequestOptional):
    pass

_GetTopTraderConsensusResponseDataItemOptional = TypedDict("_GetTopTraderConsensusResponseDataItemOptional", {'coin': 'str | None', 'long_traders': 'int | None', 'long_notional': 'str | None', 'short_traders': 'int | None', 'short_notional': 'str | None', 'traders': 'int | None', 'snapshot_time': 'int | None'}, total=False)
class GetTopTraderConsensusResponseDataItem(_GetTopTraderConsensusResponseDataItemOptional):
    pass

class GetTopTraderConsensusResponse(TypedDict):
    data: list[GetTopTraderConsensusResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetTopTraderOpensRequestOptional = TypedDict("_GetTopTraderOpensRequestOptional", {'minutes': 'GetTopTraderOpensRequestMinutes | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetTopTraderOpensRequest(_GetTopTraderOpensRequestOptional):
    pass

_GetTopTraderOpensResponseDataItemOptional = TypedDict("_GetTopTraderOpensResponseDataItemOptional", {'coin': 'str | None', 'long_traders': 'int | None', 'short_traders': 'int | None', 'traders': 'int | None', 'notional': 'str | None'}, total=False)
class GetTopTraderOpensResponseDataItem(_GetTopTraderOpensResponseDataItemOptional):
    pass

class GetTopTraderOpensResponse(TypedDict):
    data: list[GetTopTraderOpensResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetTradeBarsRequestOptional = TypedDict("_GetTradeBarsRequestOptional", {'interval': 'GetTradeBarsRequestInterval | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetTradeBarsRequest(_GetTradeBarsRequestOptional):
    coin: str

_GetTradeBarsResponseDataItemOptional = TypedDict("_GetTradeBarsResponseDataItemOptional", {'t': 'int | None', 'open': 'str | None', 'high': 'str | None', 'low': 'str | None', 'close': 'str | None', 'buy_sz': 'str | None', 'sell_sz': 'str | None', 'buy_ntl': 'str | None', 'sell_ntl': 'str | None', 'buy_n': 'int | None', 'sell_n': 'int | None', 'liq_buy_sz': 'str | None', 'liq_sell_sz': 'str | None', 'liq_buy_ntl': 'str | None', 'liq_sell_ntl': 'str | None', 'liq_n': 'int | None', 'liq_known': 'bool | None', 'final': 'bool | None'}, total=False)
class GetTradeBarsResponseDataItem(_GetTradeBarsResponseDataItemOptional):
    pass

class GetTradeBarsResponse(TypedDict):
    data: list[GetTradeBarsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetTraderDistributionRequestOptional = TypedDict("_GetTraderDistributionRequestOptional", {'view': 'GetTraderDistributionRequestView | None', 'period': 'GetTraderDistributionRequestPeriod | None', 'start': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetTraderDistributionRequest(_GetTraderDistributionRequestOptional):
    pass

_GetTraderDistributionResponseDataItemOptional = TypedDict("_GetTraderDistributionResponseDataItemOptional", {'tier': 'str | None', 'min_usd': 'str | None', 'max_usd': 'str | None', 'from_tier': 'str | None', 'wallets': 'int | None', 'total': 'str | None', 'period_start': 'int | None', 'period_end': 'int | None'}, total=False)
class GetTraderDistributionResponseDataItem(_GetTraderDistributionResponseDataItemOptional):
    pass

class GetTraderDistributionResponse(TypedDict):
    data: list[GetTraderDistributionResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetTraderGroupRequestOptional = TypedDict("_GetTraderGroupRequestOptional", {'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetTraderGroupRequest(_GetTraderGroupRequestOptional):
    group: GetTraderGroupRequestGroup

_GetTraderGroupResponseDataItemOptional = TypedDict("_GetTraderGroupResponseDataItemOptional", {'rank': 'int | None', 'address': 'str | None', 'label': 'str | None', 'synced_at': 'int | None'}, total=False)
class GetTraderGroupResponseDataItem(_GetTraderGroupResponseDataItemOptional):
    pass

class GetTraderGroupResponse(TypedDict):
    data: list[GetTraderGroupResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetTraderProfitabilityRequestOptional = TypedDict("_GetTraderProfitabilityRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetTraderProfitabilityRequest(_GetTraderProfitabilityRequestOptional):
    pass

_GetTraderProfitabilityResponseDataItemOptional = TypedDict("_GetTraderProfitabilityResponseDataItemOptional", {'t': 'int | None', 'cohort': 'str | None', 'min_usd': 'str | None', 'max_usd': 'str | None', 'traders': 'int | None', 'profitable': 'int | None', 'unprofitable': 'int | None', 'pnl_sum': 'str | None', 'median_roi': 'str | None', 'complete': 'bool | None'}, total=False)
class GetTraderProfitabilityResponseDataItem(_GetTraderProfitabilityResponseDataItemOptional):
    pass

class GetTraderProfitabilityResponse(TypedDict):
    data: list[GetTraderProfitabilityResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetTwapPressureRequestOptional = TypedDict("_GetTwapPressureRequestOptional", {'market': 'str | None', 'minutes': 'int | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetTwapPressureRequest(_GetTwapPressureRequestOptional):
    pass

_GetTwapPressureResponseDataItemOptional = TypedDict("_GetTwapPressureResponseDataItemOptional", {'coin': 'str | None', 'buy_notional': 'str | None', 'sell_notional': 'str | None', 'net_notional': 'str | None', 'twaps': 'int | None', 'fills': 'int | None'}, total=False)
class GetTwapPressureResponseDataItem(_GetTwapPressureResponseDataItemOptional):
    pass

class GetTwapPressureResponse(TypedDict):
    data: list[GetTwapPressureResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetUsdcReserveRequestOptional = TypedDict("_GetUsdcReserveRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetUsdcReserveRequest(_GetUsdcReserveRequestOptional):
    pass

_GetUsdcReserveResponseDataItemOptional = TypedDict("_GetUsdcReserveResponseDataItemOptional", {'kind': 'str | None', 'day': 'int | None', 'balance': 'str | None', 'snapshot_time': 'int | None', 'read_time': 'int | None'}, total=False)
class GetUsdcReserveResponseDataItem(_GetUsdcReserveResponseDataItemOptional):
    pass

class GetUsdcReserveResponse(TypedDict):
    data: list[GetUsdcReserveResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetUsdcReserveIntervalsRequestOptional = TypedDict("_GetUsdcReserveIntervalsRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'assumed_rate_pct': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetUsdcReserveIntervalsRequest(_GetUsdcReserveIntervalsRequestOptional):
    pass

_GetUsdcReserveIntervalsResponseDataItemOptional = TypedDict("_GetUsdcReserveIntervalsResponseDataItemOptional", {'index': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'settlement_due': 'int | None', 'readings': 'int | None', 'readings_on_time': 'int | None', 'accrued_at_assumed_rate': 'str | None', 'settled': 'str | None', 'settled_at': 'int | None', 'implied_rate_pct': 'str | None'}, total=False)
class GetUsdcReserveIntervalsResponseDataItem(_GetUsdcReserveIntervalsResponseDataItemOptional):
    pass

class GetUsdcReserveIntervalsResponse(TypedDict):
    data: list[GetUsdcReserveIntervalsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetUsdcReserveSettlementsRequestOptional = TypedDict("_GetUsdcReserveSettlementsRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetUsdcReserveSettlementsRequest(_GetUsdcReserveSettlementsRequestOptional):
    pass

_GetUsdcReserveSettlementsResponseDataItemOptional = TypedDict("_GetUsdcReserveSettlementsResponseDataItemOptional", {'time': 'int | None', 'amount': 'str | None', 'hash': 'str | None'}, total=False)
class GetUsdcReserveSettlementsResponseDataItem(_GetUsdcReserveSettlementsResponseDataItemOptional):
    pass

class GetUsdcReserveSettlementsResponse(TypedDict):
    data: list[GetUsdcReserveSettlementsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetUserGrowthRequestOptional = TypedDict("_GetUserGrowthRequestOptional", {'interval': 'GetUserGrowthRequestInterval | None', 'days': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetUserGrowthRequest(_GetUserGrowthRequestOptional):
    pass

_GetUserGrowthResponseDataItemOptional = TypedDict("_GetUserGrowthResponseDataItemOptional", {'t': 'int | None', 'new_wallets': 'int | None', 'total_wallets': 'int | None'}, total=False)
class GetUserGrowthResponseDataItem(_GetUserGrowthResponseDataItemOptional):
    pass

class GetUserGrowthResponse(TypedDict):
    data: list[GetUserGrowthResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetValidatorRegistryRequestOptional = TypedDict("_GetValidatorRegistryRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetValidatorRegistryRequest(_GetValidatorRegistryRequestOptional):
    pass

_GetValidatorRegistryResponseDataItemOptional = TypedDict("_GetValidatorRegistryResponseDataItemOptional", {'name': 'str | None', 'description': 'str | None', 'commission_bps': 'int | None', 'delegations_disabled': 'bool | None', 'delegations_blocked': 'bool | None', 'total_delegated': 'str | None', 'register_time': 'int | None', 'last_manual_unjail_time': 'int | None', 'last_signer_change_time': 'int | None', 'snapshot_time': 'int | None'}, total=False)
class GetValidatorRegistryResponseDataItem(_GetValidatorRegistryResponseDataItemOptional):
    pass

class GetValidatorRegistryResponse(TypedDict):
    data: list[GetValidatorRegistryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetValidatorRewardsRequestOptional = TypedDict("_GetValidatorRewardsRequestOptional", {'interval': 'GetValidatorRewardsRequestInterval | None', 'validator': 'str | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetValidatorRewardsRequest(_GetValidatorRewardsRequestOptional):
    pass

_GetValidatorRewardsResponseDataItemOptional = TypedDict("_GetValidatorRewardsResponseDataItemOptional", {'t': 'int | None', 'validator': 'str | None', 'name': 'str | None', 'reward': 'str | None', 'events': 'int | None'}, total=False)
class GetValidatorRewardsResponseDataItem(_GetValidatorRewardsResponseDataItemOptional):
    pass

class GetValidatorRewardsResponse(TypedDict):
    data: list[GetValidatorRewardsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetValidatorsRequestOptional = TypedDict("_GetValidatorsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetValidatorsRequest(_GetValidatorsRequestOptional):
    pass

_GetValidatorsResponseDataItemOptional = TypedDict("_GetValidatorsResponseDataItemOptional", {'name': 'str | None', 'stake': 'str | None', 'commission': 'str | None', 'is_jailed': 'bool | None', 'is_active': 'bool | None', 'recent_blocks': 'int | None', 'uptime_day': 'str | None', 'uptime_week': 'str | None', 'uptime_month': 'str | None', 'apr_day': 'str | None', 'apr_week': 'str | None', 'apr_month': 'str | None', 'snapshot_time': 'int | None', 'unjailable_after': 'int | None', 'samples_day': 'int | None', 'samples_week': 'int | None', 'samples_month': 'int | None', 'source': 'str | None'}, total=False)
class GetValidatorsResponseDataItem(_GetValidatorsResponseDataItemOptional):
    pass

class GetValidatorsResponse(TypedDict):
    data: list[GetValidatorsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetVaultHistoryRequestOptional = TypedDict("_GetVaultHistoryRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetVaultHistoryRequest(_GetVaultHistoryRequestOptional):
    vault: str

_GetVaultHistoryResponseDataItemOptional = TypedDict("_GetVaultHistoryResponseDataItemOptional", {'snapshot_time': 'int | None', 'followers': 'int | None', 'is_closed': 'bool | None', 'allow_deposits': 'bool | None', 'leader_commission': 'str | None'}, total=False)
class GetVaultHistoryResponseDataItem(_GetVaultHistoryResponseDataItemOptional):
    pass

class GetVaultHistoryResponse(TypedDict):
    data: list[GetVaultHistoryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletBuildersRequestOptional = TypedDict("_GetWalletBuildersRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletBuildersRequest(_GetWalletBuildersRequestOptional):
    address: str

_GetWalletBuildersResponseDataItemOptional = TypedDict("_GetWalletBuildersResponseDataItemOptional", {'builder': 'str | None', 'fee_token': 'str | None', 'fills': 'int | None', 'orders': 'int | None', 'notional': 'str | None', 'fees': 'str | None', 'builder_fees': 'str | None', 'fee_unreported_fills': 'int | None', 'first_fill': 'int | None', 'last_fill': 'int | None'}, total=False)
class GetWalletBuildersResponseDataItem(_GetWalletBuildersResponseDataItemOptional):
    pass

class GetWalletBuildersResponse(TypedDict):
    data: list[GetWalletBuildersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletCompletedTradesRequestOptional = TypedDict("_GetWalletCompletedTradesRequestOptional", {'coin': 'str | None', 'side': 'GetWalletCompletedTradesRequestSide | None', 'result': 'GetWalletCompletedTradesRequestResult | None', 'min_notional': 'GetWalletCompletedTradesRequestMinNotional | None', 'sort': 'GetWalletCompletedTradesRequestSort | None', 'order': 'GetWalletCompletedTradesRequestOrder | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletCompletedTradesRequest(_GetWalletCompletedTradesRequestOptional):
    address: str

_GetWalletCompletedTradesResponseDataItemOptional = TypedDict("_GetWalletCompletedTradesResponseDataItemOptional", {'coin': 'str | None', 'side': 'str | None', 'open_time': 'int | None', 'close_time': 'int | None', 'duration_ms': 'int | None', 'entry_px': 'str | None', 'exit_px': 'str | None', 'size': 'str | None', 'max_size': 'str | None', 'notional': 'str | None', 'volume': 'str | None', 'gross_pnl': 'str | None', 'funding_pnl': 'str | None', 'fees': 'str | None', 'net_pnl': 'str | None', 'fills': 'int | None', 'maker_fills': 'int | None', 'leverage': 'int | None', 'is_cross': 'bool | None', 'builder': 'str | None', 'tx_index': 'int | None'}, total=False)
class GetWalletCompletedTradesResponseDataItem(_GetWalletCompletedTradesResponseDataItemOptional):
    pass

class GetWalletCompletedTradesResponse(TypedDict):
    data: list[GetWalletCompletedTradesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletCopyActivityRequestOptional = TypedDict("_GetWalletCopyActivityRequestOptional", {'from': 'str | None', 'to': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletCopyActivityRequest(_GetWalletCopyActivityRequestOptional):
    address: str

_GetWalletCopyActivityResponseDataItemOptional = TypedDict("_GetWalletCopyActivityResponseDataItemOptional", {'day': 'str | None', 'fills': 'int | None', 'orders_manual': 'int | None', 'notional': 'str | None', 'maker_notional': 'str | None', 'fees_paid': 'str | None', 'delay_cost': 'str | None', 'priced_notional': 'str | None', 'trips': 'int | None', 'trip_notional': 'str | None', 'trip_notional_under_1h': 'str | None', 'computed_at': 'int | None'}, total=False)
class GetWalletCopyActivityResponseDataItem(_GetWalletCopyActivityResponseDataItemOptional):
    pass

class GetWalletCopyActivityResponse(TypedDict):
    data: list[GetWalletCopyActivityResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletCopyScoreRequestOptional = TypedDict("_GetWalletCopyScoreRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletCopyScoreRequest(_GetWalletCopyScoreRequestOptional):
    address: str

_GetWalletCopyScoreResponseDataItemOptional = TypedDict("_GetWalletCopyScoreResponseDataItemOptional", {'as_of': 'str | None', 'copy_score': 'str | None', 'copy_score_raw': 'str | None', 'quality': 'str | None', 'gate': 'int | None', 'consistency': 'str | None', 'replicability': 'str | None', 'activity': 'str | None', 'skill': 'str | None', 'drawdown_factor': 'str | None', 'volatility_factor': 'str | None', 'pnl_factor': 'str | None', 'sortino_90d': 'str | None', 'pnl_90d': 'str | None', 'lifetime_pnl': 'str | None', 'fills_90d': 'int | None', 'orders_per_day': 'str | None', 'maker_share': 'str | None', 'short_hold_share': 'str | None', 'algo': 'bool | None', 'delay_cost_90d': 'str | None', 'fee_drag_90d': 'str | None', 'round_trips_90d': 'int | None', 'fill_days_90d': 'int | None', 'weeks_with_pnl': 'int | None', 'winning_week_share': 'str | None', 'active_days_14d': 'int | None', 'equity_p50': 'str | None', 'max_drawdown_90d': 'str | None', 'volatility_90d': 'str | None', 'twr_drawdown_90d': 'str | None', 'underwater': 'str | None', 'sortino_weighted': 'str | None', 'evidence_days': 'str | None', 'record_days': 'int | None', 'computed_at': 'str | None'}, total=False)
class GetWalletCopyScoreResponseDataItem(_GetWalletCopyScoreResponseDataItemOptional):
    pass

class GetWalletCopyScoreResponse(TypedDict):
    data: list[GetWalletCopyScoreResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletHighlightsRequestOptional = TypedDict("_GetWalletHighlightsRequestOptional", {'coin': 'str | None', 'side': 'GetWalletHighlightsRequestSide | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletHighlightsRequest(_GetWalletHighlightsRequestOptional):
    address: str

_GetWalletHighlightsResponseDataItemOptional = TypedDict("_GetWalletHighlightsResponseDataItemOptional", {'kind': 'str | None', 'rank': 'int | None', 'coin': 'str | None', 'side': 'str | None', 'open_time': 'int | None', 'close_time': 'int | None', 'duration_s': 'str | None', 'size': 'str | None', 'entry_px': 'str | None', 'exit_px': 'str | None', 'volume': 'str | None', 'gross_pnl': 'str | None', 'funding_pnl': 'str | None', 'fees': 'str | None', 'net_pnl': 'str | None'}, total=False)
class GetWalletHighlightsResponseDataItem(_GetWalletHighlightsResponseDataItemOptional):
    pass

class GetWalletHighlightsResponse(TypedDict):
    data: list[GetWalletHighlightsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletLeaderboardRequestOptional = TypedDict("_GetWalletLeaderboardRequestOptional", {'window': 'GetWalletLeaderboardRequestWindow | None', 'category': 'GetWalletLeaderboardRequestCategory | None', 'sort': 'GetWalletLeaderboardRequestSort | None', 'coin': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletLeaderboardRequest(_GetWalletLeaderboardRequestOptional):
    address: str

_GetWalletLeaderboardResponseDataItemOptional = TypedDict("_GetWalletLeaderboardResponseDataItemOptional", {'rank': 'int | None', 'net_pnl': 'str | None', 'realized_pnl': 'str | None', 'unrealized_pnl': 'str | None', 'capital': 'str | None', 'roi': 'str | None', 'realized_roi': 'str | None', 'unrealized_roi': 'str | None', 'display_name': 'str | None', 'tag': 'str | None', 'gross_pnl': 'str | None', 'funding_pnl': 'str | None', 'total_fees': 'str | None', 'volume': 'str | None', 'trades': 'int | None', 'winning_trades': 'int | None', 'losing_trades': 'int | None', 'account_value': 'str | None', 'last_trade_at': 'int | None', 'label': 'str | None', 'avatar': 'str | None', 'twitter': 'str | None', 'verified': 'bool | None', 'copy_score': 'str | None', 'mark_quality': 'int | None', 'window_start': 'int | None', 'as_of': 'int | None'}, total=False)
class GetWalletLeaderboardResponseDataItem(_GetWalletLeaderboardResponseDataItemOptional):
    pass

class GetWalletLeaderboardResponse(TypedDict):
    data: list[GetWalletLeaderboardResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletMarketsTradedRequestOptional = TypedDict("_GetWalletMarketsTradedRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletMarketsTradedRequest(_GetWalletMarketsTradedRequestOptional):
    address: str

_GetWalletMarketsTradedResponseDataItemOptional = TypedDict("_GetWalletMarketsTradedResponseDataItemOptional", {'coin': 'str | None', 'volume': 'str | None', 'gross_pnl': 'str | None', 'net_pnl': 'str | None', 'fees': 'str | None', 'trades': 'int | None', 'wins': 'int | None', 'losses': 'int | None', 'win_rate_pct': 'str | None', 'last_trade_time': 'int | None'}, total=False)
class GetWalletMarketsTradedResponseDataItem(_GetWalletMarketsTradedResponseDataItemOptional):
    pass

class GetWalletMarketsTradedResponse(TypedDict):
    data: list[GetWalletMarketsTradedResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletOrderFillsRequestOptional = TypedDict("_GetWalletOrderFillsRequestOptional", {'coin': 'str | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletOrderFillsRequest(_GetWalletOrderFillsRequestOptional):
    address: str

_GetWalletOrderFillsResponseDataItemOptional = TypedDict("_GetWalletOrderFillsResponseDataItemOptional", {'oid': 'str | None', 'coin': 'str | None', 'time': 'int | None', 'first_time': 'int | None', 'start_position': 'str | None', 'direction': 'str | None', 'side': 'str | None', 'size': 'str | None', 'avg_px': 'str | None', 'notional': 'str | None', 'closed_pnl': 'str | None', 'fees': 'str | None', 'fill_count': 'int | None', 'fills': 'str | None'}, total=False)
class GetWalletOrderFillsResponseDataItem(_GetWalletOrderFillsResponseDataItemOptional):
    pass

class GetWalletOrderFillsResponse(TypedDict):
    data: list[GetWalletOrderFillsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletOutcomesRequestOptional = TypedDict("_GetWalletOutcomesRequestOptional", {'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletOutcomesRequest(_GetWalletOutcomesRequestOptional):
    address: str

_GetWalletOutcomesResponseDataItemOptional = TypedDict("_GetWalletOutcomesResponseDataItemOptional", {'outcome': 'str | None', 'yes_shares': 'str | None', 'no_shares': 'str | None', 'yes_basis': 'str | None', 'no_basis': 'str | None', 'realized_pnl': 'str | None', 'usdc_fees': 'str | None', 'share_fees': 'str | None', 'notional': 'str | None', 'fills': 'int | None', 'first_fill_time': 'int | None', 'last_fill_time': 'int | None', 'settled': 'bool | None', 'settle_fraction': 'str | None', 'settlement_pnl': 'str | None', 'net_pnl': 'str | None', 'result': 'str | None', 'balance_status': 'str | None', 'balance_checked_at': 'int | None'}, total=False)
class GetWalletOutcomesResponseDataItem(_GetWalletOutcomesResponseDataItemOptional):
    pass

class GetWalletOutcomesResponse(TypedDict):
    data: list[GetWalletOutcomesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletPnlCalendarRequestOptional = TypedDict("_GetWalletPnlCalendarRequestOptional", {'from': 'str | None', 'to': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletPnlCalendarRequest(_GetWalletPnlCalendarRequestOptional):
    address: str

_GetWalletPnlCalendarResponseDataItemOptional = TypedDict("_GetWalletPnlCalendarResponseDataItemOptional", {'date': 'str | None', 'pnl': 'str | None', 'pnl_pct': 'str | None'}, total=False)
class GetWalletPnlCalendarResponseDataItem(_GetWalletPnlCalendarResponseDataItemOptional):
    pass

class GetWalletPnlCalendarResponse(TypedDict):
    data: list[GetWalletPnlCalendarResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletProfilesRequestOptional = TypedDict("_GetWalletProfilesRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletProfilesRequest(_GetWalletProfilesRequestOptional):
    addresses: str

_GetWalletProfilesResponseDataItemOptional = TypedDict("_GetWalletProfilesResponseDataItemOptional", {'address': 'str | None', 'display_name': 'str | None', 'avatar': 'str | None', 'twitter': 'str | None', 'tag': 'str | None', 'verified': 'bool | None', 'label': 'str | None', 'updated_at': 'int | None'}, total=False)
class GetWalletProfilesResponseDataItem(_GetWalletProfilesResponseDataItemOptional):
    pass

class GetWalletProfilesResponse(TypedDict):
    data: list[GetWalletProfilesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletReviewRequestOptional = TypedDict("_GetWalletReviewRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletReviewRequest(_GetWalletReviewRequestOptional):
    address: str

_GetWalletReviewResponseDataItemOptional = TypedDict("_GetWalletReviewResponseDataItemOptional", {'section': 'str | None', 'key': 'str | None', 'trades': 'int | None', 'wins': 'int | None', 'losses': 'int | None', 'incomplete': 'int | None', 'net_pnl': 'str | None', 'gross_pnl': 'str | None', 'funding_pnl': 'str | None', 'fees': 'str | None', 'volume': 'str | None', 'best_trade': 'str | None', 'worst_trade': 'str | None'}, total=False)
class GetWalletReviewResponseDataItem(_GetWalletReviewResponseDataItemOptional):
    pass

class GetWalletReviewResponse(TypedDict):
    data: list[GetWalletReviewResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletRiskMetricsRequestOptional = TypedDict("_GetWalletRiskMetricsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletRiskMetricsRequest(_GetWalletRiskMetricsRequestOptional):
    address: str

_GetWalletRiskMetricsResponseDataItemOptional = TypedDict("_GetWalletRiskMetricsResponseDataItemOptional", {'as_of': 'str | None', 'sharpe_30d': 'str | None', 'sortino_30d': 'str | None', 'sharpe_90d': 'str | None', 'sortino_90d': 'str | None', 'sharpe_180d': 'str | None', 'sortino_180d': 'str | None', 'sharpe_365d': 'str | None', 'sortino_365d': 'str | None', 'max_drawdown_7d': 'str | None', 'max_drawdown_30d': 'str | None', 'max_drawdown_90d': 'str | None', 'max_drawdown_180d': 'str | None', 'max_drawdown_365d': 'str | None'}, total=False)
class GetWalletRiskMetricsResponseDataItem(_GetWalletRiskMetricsResponseDataItemOptional):
    pass

class GetWalletRiskMetricsResponse(TypedDict):
    data: list[GetWalletRiskMetricsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletSharpeRequestOptional = TypedDict("_GetWalletSharpeRequestOptional", {'from': 'str | None', 'to': 'str | None', 'min_active_days': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletSharpeRequest(_GetWalletSharpeRequestOptional):
    address: str

_GetWalletSharpeResponseDataItemOptional = TypedDict("_GetWalletSharpeResponseDataItemOptional", {'days': 'int | None', 'active_days': 'int | None', 'first_date': 'str | None', 'last_date': 'str | None', 'pnl': 'str | None', 'sharpe': 'str | None', 'sortino': 'str | None'}, total=False)
class GetWalletSharpeResponseDataItem(_GetWalletSharpeResponseDataItemOptional):
    pass

class GetWalletSharpeResponse(TypedDict):
    data: list[GetWalletSharpeResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletStakedHypeRequestOptional = TypedDict("_GetWalletStakedHypeRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletStakedHypeRequest(_GetWalletStakedHypeRequestOptional):
    address: str
    time: int

_GetWalletStakedHypeResponseDataItemOptional = TypedDict("_GetWalletStakedHypeResponseDataItemOptional", {'hype': 'str | None'}, total=False)
class GetWalletStakedHypeResponseDataItem(_GetWalletStakedHypeResponseDataItemOptional):
    pass

class GetWalletStakedHypeResponse(TypedDict):
    data: list[GetWalletStakedHypeResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletStatsRequestOptional = TypedDict("_GetWalletStatsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletStatsRequest(_GetWalletStatsRequestOptional):
    address: str

_GetWalletStatsResponseDataItemOptional = TypedDict("_GetWalletStatsResponseDataItemOptional", {'trades': 'int | None', 'first_trade_time': 'int | None', 'last_trade_time': 'int | None', 'pnl': 'str | None', 'long_pnl': 'str | None', 'short_pnl': 'str | None', 'fees': 'str | None', 'net_pnl': 'str | None', 'avg_hold_minutes': 'str | None', 'median_hold_minutes': 'str | None', 'q1_hold_minutes': 'str | None', 'q3_hold_minutes': 'str | None', 'longest_win_streak': 'int | None'}, total=False)
class GetWalletStatsResponseDataItem(_GetWalletStatsResponseDataItemOptional):
    pass

class GetWalletStatsResponse(TypedDict):
    data: list[GetWalletStatsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletStatsByMarketRequestOptional = TypedDict("_GetWalletStatsByMarketRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletStatsByMarketRequest(_GetWalletStatsByMarketRequestOptional):
    address: str

_GetWalletStatsByMarketResponseDataItemOptional = TypedDict("_GetWalletStatsByMarketResponseDataItemOptional", {'coin': 'str | None', 'pnl': 'str | None', 'fees': 'str | None', 'net_pnl': 'str | None', 'trades': 'int | None', 'wins': 'int | None', 'losses': 'int | None', 'volume': 'str | None', 'winning_pnl': 'str | None', 'losing_pnl': 'str | None'}, total=False)
class GetWalletStatsByMarketResponseDataItem(_GetWalletStatsByMarketResponseDataItemOptional):
    pass

class GetWalletStatsByMarketResponse(TypedDict):
    data: list[GetWalletStatsByMarketResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletTradeCountsRequestOptional = TypedDict("_GetWalletTradeCountsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletTradeCountsRequest(_GetWalletTradeCountsRequestOptional):
    address: str

_GetWalletTradeCountsResponseDataItemOptional = TypedDict("_GetWalletTradeCountsResponseDataItemOptional", {'trades': 'int | None', 'long_trades': 'int | None', 'short_trades': 'int | None', 'wins': 'int | None', 'losses': 'int | None', 'trades_7d': 'int | None', 'trades_30d': 'int | None', 'trades_90d': 'int | None', 'trades_180d': 'int | None', 'trades_365d': 'int | None', 'wins_7d': 'int | None', 'wins_30d': 'int | None', 'wins_90d': 'int | None', 'wins_180d': 'int | None', 'wins_365d': 'int | None', 'last_trade_time': 'int | None'}, total=False)
class GetWalletTradeCountsResponseDataItem(_GetWalletTradeCountsResponseDataItemOptional):
    pass

class GetWalletTradeCountsResponse(TypedDict):
    data: list[GetWalletTradeCountsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletUpnlRankRequestOptional = TypedDict("_GetWalletUpnlRankRequestOptional", {'category': 'GetWalletUpnlRankRequestCategory | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletUpnlRankRequest(_GetWalletUpnlRankRequestOptional):
    address: str

_GetWalletUpnlRankResponseDataItemOptional = TypedDict("_GetWalletUpnlRankResponseDataItemOptional", {'unrealized_pnl': 'str | None', 'rank': 'int | None', 'traders': 'int | None'}, total=False)
class GetWalletUpnlRankResponseDataItem(_GetWalletUpnlRankResponseDataItemOptional):
    pass

class GetWalletUpnlRankResponse(TypedDict):
    data: list[GetWalletUpnlRankResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletWindowStatsRequestOptional = TypedDict("_GetWalletWindowStatsRequestOptional", {'coin': 'str | None', 'window': 'GetWalletWindowStatsRequestWindow | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletWindowStatsRequest(_GetWalletWindowStatsRequestOptional):
    address: str

_GetWalletWindowStatsResponseDataItemOptional = TypedDict("_GetWalletWindowStatsResponseDataItemOptional", {'net_pnl': 'str | None', 'gross_pnl': 'str | None', 'funding_pnl': 'str | None', 'volume': 'str | None', 'trades': 'int | None', 'wins': 'int | None', 'losses': 'int | None', 'last_trade_at': 'str | None'}, total=False)
class GetWalletWindowStatsResponseDataItem(_GetWalletWindowStatsResponseDataItemOptional):
    pass

class GetWalletWindowStatsResponse(TypedDict):
    data: list[GetWalletWindowStatsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWalletYearSummaryRequestOptional = TypedDict("_GetWalletYearSummaryRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWalletYearSummaryRequest(_GetWalletYearSummaryRequestOptional):
    address: str

_GetWalletYearSummaryResponseDataItemOptional = TypedDict("_GetWalletYearSummaryResponseDataItemOptional", {'year': 'int | None', 'trader_class': 'str | None', 'reason': 'str | None', 'pnl': 'str | None', 'volume': 'str | None', 'trades': 'int | None', 'pnl_percentile': 'str | None', 'volume_percentile': 'str | None', 'trades_percentile': 'str | None', 'win_rate': 'str | None', 'sharpe_30d': 'str | None', 'taker_ratio': 'str | None', 'trades_per_day': 'str | None', 'median_hold_s': 'str | None', 'computed_at': 'str | None'}, total=False)
class GetWalletYearSummaryResponseDataItem(_GetWalletYearSummaryResponseDataItemOptional):
    pass

class GetWalletYearSummaryResponse(TypedDict):
    data: list[GetWalletYearSummaryResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_GetWindowPricesRequestOptional = TypedDict("_GetWindowPricesRequestOptional", {'coins': 'str | None', 'asset_class': 'str | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class GetWindowPricesRequest(_GetWindowPricesRequestOptional):
    pass

_GetWindowPricesResponseDataItemOptional = TypedDict("_GetWindowPricesResponseDataItemOptional", {'coin': 'str | None', 'open': 'str | None', 'high': 'str | None', 'low': 'str | None', 'close': 'str | None'}, total=False)
class GetWindowPricesResponseDataItem(_GetWindowPricesResponseDataItemOptional):
    pass

class GetWindowPricesResponse(TypedDict):
    data: list[GetWindowPricesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_LiquidationMinutesRequestOptional = TypedDict("_LiquidationMinutesRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class LiquidationMinutesRequest(_LiquidationMinutesRequestOptional):
    pass

_LiquidationMinutesResponseDataItemOptional = TypedDict("_LiquidationMinutesResponseDataItemOptional", {'t': 'int | None', 'long_notional': 'str | None', 'short_notional': 'str | None', 'long_events': 'int | None', 'short_events': 'int | None', 'events': 'int | None'}, total=False)
class LiquidationMinutesResponseDataItem(_LiquidationMinutesResponseDataItemOptional):
    pass

class LiquidationMinutesResponse(TypedDict):
    data: list[LiquidationMinutesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_LiquidationMixRequestOptional = TypedDict("_LiquidationMixRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class LiquidationMixRequest(_LiquidationMixRequestOptional):
    pass

_LiquidationMixResponseDataItemOptional = TypedDict("_LiquidationMixResponseDataItemOptional", {'coin': 'str | None', 'long_notional': 'str | None', 'short_notional': 'str | None', 'total_notional': 'str | None', 'events': 'int | None'}, total=False)
class LiquidationMixResponseDataItem(_LiquidationMixResponseDataItemOptional):
    pass

class LiquidationMixResponse(TypedDict):
    data: list[LiquidationMixResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListAllTwapsRequestOptional = TypedDict("_ListAllTwapsRequestOptional", {'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListAllTwapsRequest(_ListAllTwapsRequestOptional):
    pass

_ListAllTwapsResponseDataItemOptional = TypedDict("_ListAllTwapsResponseDataItemOptional", {'coin': 'str | None', 'address': 'str | None', 'twap_id': 'str | None', 'side': 'str | None', 'total_sz': 'str | None', 'executed_sz': 'str | None', 'remaining_sz': 'str | None', 'executed_notional': 'str | None', 'progress_pct': 'str | None', 'duration_s': 'int | None', 'start_time': 'int | None', 'reduce_only': 'bool | None', 'randomize': 'bool | None', 'next_slice_time': 'str | None', 'slices_done': 'int | None', 'waiting': 'bool | None', 'stop_px': 'str | None', 'trigger_px': 'str | None', 'trigger_above': 'bool | None'}, total=False)
class ListAllTwapsResponseDataItem(_ListAllTwapsResponseDataItemOptional):
    pass

class ListAllTwapsResponse(TypedDict):
    data: list[ListAllTwapsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListCohortMembersRequestOptional = TypedDict("_ListCohortMembersRequestOptional", {'coin': 'str | None', 'sort': 'ListCohortMembersRequestSort | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListCohortMembersRequest(_ListCohortMembersRequestOptional):
    cohort: ListCohortMembersRequestCohort

_ListCohortMembersResponseDataItemOptional = TypedDict("_ListCohortMembersResponseDataItemOptional", {'address': 'str | None', 'account_value': 'str | None', 'positions': 'int | None', 'market': 'str | None', 'side': 'str | None', 'size': 'str | None', 'entry_px': 'str | None', 'leverage_value': 'str | None', 'leverage_mode': 'str | None', 'notional': 'str | None', 'unrealized_pnl': 'str | None', 'snapshot_time': 'int | None'}, total=False)
class ListCohortMembersResponseDataItem(_ListCohortMembersResponseDataItemOptional):
    pass

class ListCohortMembersResponse(TypedDict):
    data: list[ListCohortMembersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListCohortTopPositionsRequestOptional = TypedDict("_ListCohortTopPositionsRequestOptional", {'direction': 'ListCohortTopPositionsRequestDirection | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListCohortTopPositionsRequest(_ListCohortTopPositionsRequestOptional):
    pass

_ListCohortTopPositionsResponseDataItemOptional = TypedDict("_ListCohortTopPositionsResponseDataItemOptional", {'address': 'str | None', 'market': 'str | None', 'side': 'str | None', 'size': 'str | None', 'notional': 'str | None', 'entry_px': 'str | None', 'liquidation_px': 'str | None', 'unrealized_pnl': 'str | None', 'account_value': 'str | None', 'total_pnl': 'str | None', 'opened_notional': 'str | None', 'last_open_time': 'int | None', 'first_open_time': 'int | None', 'funding_pnl': 'str | None', 'mark_px': 'str | None', 'unrealized_pnl_pct': 'str | None', 'leverage': 'str | None', 'leverage_mode': 'str | None', 'notional_pct_of_account': 'str | None', 'snapshot_time': 'int | None'}, total=False)
class ListCohortTopPositionsResponseDataItem(_ListCohortTopPositionsResponseDataItemOptional):
    pass

class ListCohortTopPositionsResponse(TypedDict):
    data: list[ListCohortTopPositionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListDeployAuctionsRequestOptional = TypedDict("_ListDeployAuctionsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListDeployAuctionsRequest(_ListDeployAuctionsRequestOptional):
    pass

_ListDeployAuctionsResponseDataItemOptional = TypedDict("_ListDeployAuctionsResponseDataItemOptional", {'kind': 'str | None', 'start_time': 'int | None', 'duration_s': 'int | None', 'start_gas': 'str | None', 'current_gas': 'str | None', 'end_gas': 'str | None'}, total=False)
class ListDeployAuctionsResponseDataItem(_ListDeployAuctionsResponseDataItemOptional):
    pass

class ListDeployAuctionsResponse(TypedDict):
    data: list[ListDeployAuctionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListDexMarketsRequestOptional = TypedDict("_ListDexMarketsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListDexMarketsRequest(_ListDexMarketsRequestOptional):
    dex: str

_ListDexMarketsResponseDataItemOptional = TypedDict("_ListDexMarketsResponseDataItemOptional", {'coin': 'str | None', 'oi_cap': 'str | None', 'funding_multiplier': 'str | None', 'funding_interest_rate': 'str | None', 'funding_clamp': 'str | None'}, total=False)
class ListDexMarketsResponseDataItem(_ListDexMarketsResponseDataItemOptional):
    pass

class ListDexMarketsResponse(TypedDict):
    data: list[ListDexMarketsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListDexsRequestOptional = TypedDict("_ListDexsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListDexsRequest(_ListDexsRequestOptional):
    pass

_ListDexsResponseDataItemOptional = TypedDict("_ListDexsResponseDataItemOptional", {'name': 'str | None', 'full_name': 'str | None', 'deployer': 'str | None', 'oracle_updater': 'str | None', 'fee_recipient': 'str | None', 'markets': 'int | None', 'sub_deployers': 'str | None'}, total=False)
class ListDexsResponseDataItem(_ListDexsResponseDataItemOptional):
    pass

class ListDexsResponse(TypedDict):
    data: list[ListDexsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListEvmCoreTransfersRequestOptional = TypedDict("_ListEvmCoreTransfersRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListEvmCoreTransfersRequest(_ListEvmCoreTransfersRequestOptional):
    action_type: str

_ListEvmCoreTransfersResponseDataItemOptional = TypedDict("_ListEvmCoreTransfersResponseDataItemOptional", {'time': 'int | None', 'user': 'str | None', 'destination': 'str | None', 'action_type': 'str | None', 'nonce': 'int | None', 'evm_tx_hash': 'str | None', 'token': 'int | None', 'wei': 'str | None', 'wei_approx': 'str | None', 'block': 'int | None', 'event_index': 'int | None'}, total=False)
class ListEvmCoreTransfersResponseDataItem(_ListEvmCoreTransfersResponseDataItemOptional):
    pass

class ListEvmCoreTransfersResponse(TypedDict):
    data: list[ListEvmCoreTransfersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListFreshWalletPositionsRequestOptional = TypedDict("_ListFreshWalletPositionsRequestOptional", {'max_age_days': 'ListFreshWalletPositionsRequestMaxAgeDays | None', 'min_balance': 'ListFreshWalletPositionsRequestMinBalance | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListFreshWalletPositionsRequest(_ListFreshWalletPositionsRequestOptional):
    pass

_ListFreshWalletPositionsResponseDataItemOptional = TypedDict("_ListFreshWalletPositionsResponseDataItemOptional", {'address': 'str | None', 'market': 'str | None', 'side': 'str | None', 'size': 'str | None', 'notional': 'str | None', 'entry_px': 'str | None', 'liquidation_px': 'str | None', 'unrealized_pnl': 'str | None', 'account_value': 'str | None', 'funding_pnl': 'str | None', 'snapshot_time': 'int | None'}, total=False)
class ListFreshWalletPositionsResponseDataItem(_ListFreshWalletPositionsResponseDataItemOptional):
    pass

class ListFreshWalletPositionsResponse(TypedDict):
    data: list[ListFreshWalletPositionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListGossipAuctionsRequestOptional = TypedDict("_ListGossipAuctionsRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListGossipAuctionsRequest(_ListGossipAuctionsRequestOptional):
    pass

_ListGossipAuctionsResponseDataItemOptional = TypedDict("_ListGossipAuctionsResponseDataItemOptional", {'time': 'int | None', 'slot': 'int | None', 'previous_winner_ip': 'str | None', 'end_gas': 'str | None', 'block': 'int | None', 'event_index': 'int | None'}, total=False)
class ListGossipAuctionsResponseDataItem(_ListGossipAuctionsResponseDataItemOptional):
    pass

class ListGossipAuctionsResponse(TypedDict):
    data: list[ListGossipAuctionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListHip3OracleUpdatesRequestOptional = TypedDict("_ListHip3OracleUpdatesRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListHip3OracleUpdatesRequest(_ListHip3OracleUpdatesRequestOptional):
    coin: str

_ListHip3OracleUpdatesResponseDataItemOptional = TypedDict("_ListHip3OracleUpdatesResponseDataItemOptional", {'time': 'int | None', 'dex': 'str | None', 'update_class': 'str | None', 'mark_px_input': 'str | None', 'spot_px_input': 'str | None', 'external_perp_px_input': 'str | None', 'oracle_px': 'str | None', 'oracle_daily_px': 'str | None', 'mark_px': 'str | None', 'external_perp_px': 'str | None', 'block': 'int | None', 'event_index': 'int | None'}, total=False)
class ListHip3OracleUpdatesResponseDataItem(_ListHip3OracleUpdatesResponseDataItemOptional):
    pass

class ListHip3OracleUpdatesResponse(TypedDict):
    data: list[ListHip3OracleUpdatesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListJobsResponseDataItemErrorOptional = TypedDict("_ListJobsResponseDataItemErrorOptional", {'code': 'str | None', 'detail': 'str | None'}, total=False)
class ListJobsResponseDataItemError(_ListJobsResponseDataItemErrorOptional):
    pass

_ListJobsResponseDataItemOptional = TypedDict("_ListJobsResponseDataItemOptional", {'id': 'str | None', 'kind': 'str | None', 'params': 'dict[str, Any] | None', 'status': 'str | None', 'created_at': 'str | None', 'started_at': 'str | None', 'finished_at': 'str | None', 'expires_at': 'str | None', 'result_rows': 'int | None', 'compute_units': 'int | None', 'result_url': 'str | None', 'error': 'ListJobsResponseDataItemError | None'}, total=False)
class ListJobsResponseDataItem(_ListJobsResponseDataItemOptional):
    pass

_ListJobsResponseOptional = TypedDict("_ListJobsResponseOptional", {'data': 'list[ListJobsResponseDataItem] | None'}, total=False)
class ListJobsResponse(_ListJobsResponseOptional):
    pass

_ListLargestOpensRequestOptional = TypedDict("_ListLargestOpensRequestOptional", {'market': 'str | None', 'minutes': 'int | None', 'min_notional': 'ListLargestOpensRequestMinNotional | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListLargestOpensRequest(_ListLargestOpensRequestOptional):
    pass

_ListLargestOpensResponseDataItemOptional = TypedDict("_ListLargestOpensResponseDataItemOptional", {'address': 'str | None', 'coin': 'str | None', 'direction': 'str | None', 'oid': 'str | None', 'notional': 'str | None', 'size': 'str | None', 'avg_px': 'str | None', 'fills': 'int | None', 'first_fill_time': 'int | None', 'last_fill_time': 'int | None'}, total=False)
class ListLargestOpensResponseDataItem(_ListLargestOpensResponseDataItemOptional):
    pass

class ListLargestOpensResponse(TypedDict):
    data: list[ListLargestOpensResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListLargestPositionsRequestOptional = TypedDict("_ListLargestPositionsRequestOptional", {'market': 'str | None', 'side': 'ListLargestPositionsRequestSide | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListLargestPositionsRequest(_ListLargestPositionsRequestOptional):
    pass

_ListLargestPositionsResponseDataItemOptional = TypedDict("_ListLargestPositionsResponseDataItemOptional", {'address': 'str | None', 'market': 'str | None', 'side': 'str | None', 'size': 'str | None', 'notional': 'str | None', 'entry_px': 'str | None', 'liquidation_px': 'str | None', 'unrealized_pnl': 'str | None', 'account_value': 'str | None', 'funding_pnl': 'str | None', 'leverage': 'str | None', 'leverage_mode': 'str | None', 'snapshot_time': 'int | None'}, total=False)
class ListLargestPositionsResponseDataItem(_ListLargestPositionsResponseDataItemOptional):
    pass

class ListLargestPositionsResponse(TypedDict):
    data: list[ListLargestPositionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListLendingReservesRequestOptional = TypedDict("_ListLendingReservesRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListLendingReservesRequest(_ListLendingReservesRequestOptional):
    pass

_ListLendingReservesResponseDataItemOptional = TypedDict("_ListLendingReservesResponseDataItemOptional", {'token': 'str | None', 'token_index': 'int | None', 'supply_apr': 'str | None', 'borrow_apr': 'str | None', 'utilization': 'str | None', 'total_supplied': 'str | None', 'total_borrowed': 'str | None', 'available': 'str | None', 'oracle_px': 'str | None', 'ltv': 'str | None'}, total=False)
class ListLendingReservesResponseDataItem(_ListLendingReservesResponseDataItemOptional):
    pass

class ListLendingReservesResponse(TypedDict):
    data: list[ListLendingReservesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListLiquidationsRequestOptional = TypedDict("_ListLiquidationsRequestOptional", {'coin': 'str | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListLiquidationsRequest(_ListLiquidationsRequestOptional):
    pass

_ListLiquidationsResponseDataItemOptional = TypedDict("_ListLiquidationsResponseDataItemOptional", {'time': 'int | None', 'coin': 'str | None', 'tid': 'str | None', 'side': 'str | None', 'method': 'str | None', 'user': 'str | None', 'px': 'str | None', 'sz': 'str | None', 'notional': 'str | None', 'closed_pnl': 'str | None', 'mark_px': 'str | None', 'hash': 'str | None', 'start_position': 'str | None', 'dir': 'str | None', 'oid': 'str | None'}, total=False)
class ListLiquidationsResponseDataItem(_ListLiquidationsResponseDataItemOptional):
    pass

class ListLiquidationsResponse(TypedDict):
    data: list[ListLiquidationsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListMarginTablesRequestOptional = TypedDict("_ListMarginTablesRequestOptional", {'dex': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListMarginTablesRequest(_ListMarginTablesRequestOptional):
    pass

_ListMarginTablesResponseDataItemOptional = TypedDict("_ListMarginTablesResponseDataItemOptional", {'table_id': 'int | None', 'description': 'str | None', 'tier': 'int | None', 'lower_bound': 'str | None', 'max_leverage': 'int | None'}, total=False)
class ListMarginTablesResponseDataItem(_ListMarginTablesResponseDataItemOptional):
    pass

class ListMarginTablesResponse(TypedDict):
    data: list[ListMarginTablesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListMarketCatalogRequestOptional = TypedDict("_ListMarketCatalogRequestOptional", {'coin': 'str | None', 'category': 'ListMarketCatalogRequestCategory | None', 'sector': 'ListMarketCatalogRequestSector | None', 'asset_class': 'ListMarketCatalogRequestAssetClass | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListMarketCatalogRequest(_ListMarketCatalogRequestOptional):
    pass

_ListMarketCatalogResponseDataItemOptional = TypedDict("_ListMarketCatalogResponseDataItemOptional", {'coin': 'str | None', 'category': 'str | None', 'sector': 'str | None', 'asset_class': 'str | None', 'display_name': 'str | None', 'updated_at': 'int | None'}, total=False)
class ListMarketCatalogResponseDataItem(_ListMarketCatalogResponseDataItemOptional):
    pass

class ListMarketCatalogResponse(TypedDict):
    data: list[ListMarketCatalogResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListMarketTwapsRequestOptional = TypedDict("_ListMarketTwapsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListMarketTwapsRequest(_ListMarketTwapsRequestOptional):
    coin: str

_ListMarketTwapsResponseDataItemOptional = TypedDict("_ListMarketTwapsResponseDataItemOptional", {'address': 'str | None', 'twap_id': 'str | None', 'side': 'str | None', 'total_sz': 'str | None', 'executed_sz': 'str | None', 'remaining_sz': 'str | None', 'executed_notional': 'str | None', 'progress_pct': 'str | None', 'duration_s': 'int | None', 'start_time': 'int | None', 'reduce_only': 'bool | None', 'randomize': 'bool | None', 'next_slice_time': 'str | None', 'slices_done': 'int | None', 'waiting': 'bool | None', 'stop_px': 'str | None', 'trigger_px': 'str | None', 'trigger_above': 'bool | None'}, total=False)
class ListMarketTwapsResponseDataItem(_ListMarketTwapsResponseDataItemOptional):
    pass

class ListMarketTwapsResponse(TypedDict):
    data: list[ListMarketTwapsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListMarketsRequestOptional = TypedDict("_ListMarketsRequestOptional", {'dex': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListMarketsRequest(_ListMarketsRequestOptional):
    pass

_ListMarketsResponseDataItemOptional = TypedDict("_ListMarketsResponseDataItemOptional", {'coin': 'str | None', 'sz_decimals': 'int | None', 'max_leverage': 'int | None', 'mark_px': 'str | None', 'oracle_px': 'str | None', 'mid_px': 'str | None', 'funding': 'str | None', 'premium': 'str | None', 'open_interest': 'str | None', 'prev_day_px': 'str | None', 'day_ntl_vlm': 'str | None', 'day_base_vlm': 'str | None', 'impact_bid_px': 'str | None', 'impact_ask_px': 'str | None', 'is_delisted': 'bool | None', 'margin_table_id': 'int | None', 'only_isolated': 'bool | None', 'margin_mode': 'str | None', 'growth_mode': 'str | None', 'deployer_fee_scale': 'str | None', 'last_fee_scale_change_time': 'int | None', 'mark_px_estimated': 'bool | None', 'funding_unverified': 'bool | None', 'prev_day_px_seeded': 'bool | None', 'volume_inexact': 'bool | None', 'last_settled_funding': 'str | None', 'last_settled_funding_time': 'int | None', 'collateral_token': 'int | None', 'at_oi_cap': 'bool | None'}, total=False)
class ListMarketsResponseDataItem(_ListMarketsResponseDataItemOptional):
    pass

class ListMarketsResponse(TypedDict):
    data: list[ListMarketsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListMaxMarketOrderNotionalsRequestOptional = TypedDict("_ListMaxMarketOrderNotionalsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListMaxMarketOrderNotionalsRequest(_ListMaxMarketOrderNotionalsRequestOptional):
    pass

_ListMaxMarketOrderNotionalsResponseDataItemOptional = TypedDict("_ListMaxMarketOrderNotionalsResponseDataItemOptional", {'max_leverage': 'int | None', 'max_notional': 'str | None'}, total=False)
class ListMaxMarketOrderNotionalsResponseDataItem(_ListMaxMarketOrderNotionalsResponseDataItemOptional):
    pass

class ListMaxMarketOrderNotionalsResponse(TypedDict):
    data: list[ListMaxMarketOrderNotionalsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListMidsRequestOptional = TypedDict("_ListMidsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListMidsRequest(_ListMidsRequestOptional):
    pass

_ListMidsResponseDataItemOptional = TypedDict("_ListMidsResponseDataItemOptional", {'coin': 'str | None', 'mid': 'str | None'}, total=False)
class ListMidsResponseDataItem(_ListMidsResponseDataItemOptional):
    pass

class ListMidsResponse(TypedDict):
    data: list[ListMidsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListNetworkDeploysRequestOptional = TypedDict("_ListNetworkDeploysRequestOptional", {'action_type': 'str | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListNetworkDeploysRequest(_ListNetworkDeploysRequestOptional):
    pass

_ListNetworkDeploysResponseDataItemOptional = TypedDict("_ListNetworkDeploysResponseDataItemOptional", {'time': 'int | None', 'action_type': 'str | None', 'deployer': 'str | None', 'ok': 'bool | None', 'error': 'str | None', 'asset': 'int | None', 'action': 'str | None', 'block': 'int | None', 'bundle_index': 'int | None', 'action_index': 'int | None', 'inner': 'bool | None'}, total=False)
class ListNetworkDeploysResponseDataItem(_ListNetworkDeploysResponseDataItemOptional):
    pass

class ListNetworkDeploysResponse(TypedDict):
    data: list[ListNetworkDeploysResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListOutcomeDefinitionsRequestOptional = TypedDict("_ListOutcomeDefinitionsRequestOptional", {'kind': 'ListOutcomeDefinitionsRequestKind | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListOutcomeDefinitionsRequest(_ListOutcomeDefinitionsRequestOptional):
    pass

_ListOutcomeDefinitionsResponseDataItemOptional = TypedDict("_ListOutcomeDefinitionsResponseDataItemOptional", {'outcome': 'int | None', 'question': 'int | None', 'name': 'str | None', 'description': 'str | None', 'side_names': 'str | None', 'quote_token': 'str | None', 'venue': 'str | None', 'deployer_fee_scale': 'str | None', 'fallback_outcome': 'int | None', 'named_outcomes': 'str | None', 'settled_named_outcomes': 'str | None'}, total=False)
class ListOutcomeDefinitionsResponseDataItem(_ListOutcomeDefinitionsResponseDataItemOptional):
    pass

class ListOutcomeDefinitionsResponse(TypedDict):
    data: list[ListOutcomeDefinitionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListOutcomeMarketsRequestOptional = TypedDict("_ListOutcomeMarketsRequestOptional", {'status': 'ListOutcomeMarketsRequestStatus | None', 'outcome_id': 'int | None', 'question_id': 'int | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListOutcomeMarketsRequest(_ListOutcomeMarketsRequestOptional):
    pass

_ListOutcomeMarketsResponseDataItemOptional = TypedDict("_ListOutcomeMarketsResponseDataItemOptional", {'outcome': 'int | None', 'coin_yes': 'str | None', 'coin_no': 'str | None', 'question_id': 'int | None', 'question_name': 'str | None', 'is_fallback': 'bool | None', 'name': 'str | None', 'description': 'str | None', 'side0_name': 'str | None', 'side1_name': 'str | None', 'venue': 'str | None', 'quote_token': 'str | None', 'status': 'str | None', 'first_seen_time': 'int | None', 'expiry_time': 'int | None', 'expiry_raw': 'str | None', 'open_interest': 'str | None', 'oi_as_of': 'int | None', 'class': 'str | None', 'underlying': 'str | None', 'period': 'str | None', 'target_price': 'str | None', 'category': 'str | None', 'sub_category': 'str | None', 'deployer_fee_scale': 'str | None', 'settle_fraction': 'str | None', 'settle_details': 'str | None', 'settled_at': 'int | None'}, total=False)
class ListOutcomeMarketsResponseDataItem(_ListOutcomeMarketsResponseDataItemOptional):
    pass

class ListOutcomeMarketsResponse(TypedDict):
    data: list[ListOutcomeMarketsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListOutcomeStatsRequestOptional = TypedDict("_ListOutcomeStatsRequestOptional", {'hours': 'int | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListOutcomeStatsRequest(_ListOutcomeStatsRequestOptional):
    pass

_ListOutcomeStatsResponseDataItemOptional = TypedDict("_ListOutcomeStatsResponseDataItemOptional", {'outcome': 'int | None', 'yes_notional': 'str | None', 'share_volume': 'str | None', 'trades': 'int | None', 'traders': 'int | None', 'last_yes_px': 'str | None', 'first_trade_at': 'int | None', 'last_trade_at': 'int | None'}, total=False)
class ListOutcomeStatsResponseDataItem(_ListOutcomeStatsResponseDataItemOptional):
    pass

class ListOutcomeStatsResponse(TypedDict):
    data: list[ListOutcomeStatsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListOutcomeWhaleTradesRequestOptional = TypedDict("_ListOutcomeWhaleTradesRequestOptional", {'hours': 'int | None', 'outcome': 'int | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListOutcomeWhaleTradesRequest(_ListOutcomeWhaleTradesRequestOptional):
    pass

_ListOutcomeWhaleTradesResponseDataItemOptional = TypedDict("_ListOutcomeWhaleTradesResponseDataItemOptional", {'time': 'int | None', 'coin': 'str | None', 'outcome': 'int | None', 'side_index': 'int | None', 'yes_px': 'str | None', 'yes_side': 'str | None', 'size': 'str | None', 'yes_notional': 'str | None', 'address': 'str | None', 'hash': 'str | None', 'tid': 'str | None', 'twap_id': 'str | None'}, total=False)
class ListOutcomeWhaleTradesResponseDataItem(_ListOutcomeWhaleTradesResponseDataItemOptional):
    pass

class ListOutcomeWhaleTradesResponse(TypedDict):
    data: list[ListOutcomeWhaleTradesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListPositionsNearLiquidationRequestOptional = TypedDict("_ListPositionsNearLiquidationRequestOptional", {'market': 'str | None', 'max_distance_pct': 'ListPositionsNearLiquidationRequestMaxDistancePct | None', 'min_notional': 'ListPositionsNearLiquidationRequestMinNotional | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListPositionsNearLiquidationRequest(_ListPositionsNearLiquidationRequestOptional):
    pass

_ListPositionsNearLiquidationResponseDataItemOptional = TypedDict("_ListPositionsNearLiquidationResponseDataItemOptional", {'address': 'str | None', 'market': 'str | None', 'side': 'str | None', 'size': 'str | None', 'notional': 'str | None', 'entry_px': 'str | None', 'liquidation_px': 'str | None', 'unrealized_pnl': 'str | None', 'account_value': 'str | None', 'mark_px': 'str | None', 'mark_source': 'str | None', 'mark_px_estimated': 'bool | None', 'distance_pct': 'str | None', 'snapshot_time': 'int | None'}, total=False)
class ListPositionsNearLiquidationResponseDataItem(_ListPositionsNearLiquidationResponseDataItemOptional):
    pass

class ListPositionsNearLiquidationResponse(TypedDict):
    data: list[ListPositionsNearLiquidationResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListPriorityFeeTopUsersRequestOptional = TypedDict("_ListPriorityFeeTopUsersRequestOptional", {'days': 'int | None', 'kind': 'ListPriorityFeeTopUsersRequestKind | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListPriorityFeeTopUsersRequest(_ListPriorityFeeTopUsersRequestOptional):
    pass

_ListPriorityFeeTopUsersResponseDataItemOptional = TypedDict("_ListPriorityFeeTopUsersResponseDataItemOptional", {'address': 'str | None', 'total_usd': 'str | None', 'total_hype': 'str | None', 'write_usd': 'str | None', 'read_usd': 'str | None', 'write_orders': 'int | None', 'reads': 'int | None', 'share_pct': 'str | None', 'write_ioc_hype': 'str | None', 'write_ioc_usd': 'str | None', 'write_ioc_orders': 'int | None', 'write_ioc_fills': 'int | None', 'write_ioc_unpriced_hype': 'str | None', 'write_alo_hype_estimate': 'str | None', 'write_alo_usd': 'str | None', 'write_alo_orders': 'int | None', 'write_alo_resting_notional_usd': 'str | None', 'write_unpriced_hype': 'str | None', 'unpriced_hype': 'str | None', 'unconverted_usd': 'str | None', 'hype_estimated': 'bool | None', 'live_from': 'int | None', 'write_fills': 'int | None', 'read_hype': 'str | None', 'read_unpriced_hype': 'str | None', 'total_txs': 'int | None', 'fee_hype': 'str | None', 'fee_usd': 'str | None', 'writes': 'int | None', 'share_hype_pct': 'str | None', 'share_hype_estimated': 'bool | None', 'share_hype_complete': 'bool | None', 'wallets': 'int | None', 'usd_complete': 'bool | None', 'hype_complete': 'bool | None', 'write_hype': 'str | None'}, total=False)
class ListPriorityFeeTopUsersResponseDataItem(_ListPriorityFeeTopUsersResponseDataItemOptional):
    pass

class ListPriorityFeeTopUsersResponse(TypedDict):
    data: list[ListPriorityFeeTopUsersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListSpotMarketsRequestOptional = TypedDict("_ListSpotMarketsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListSpotMarketsRequest(_ListSpotMarketsRequestOptional):
    pass

_ListSpotMarketsResponseDataItemOptional = TypedDict("_ListSpotMarketsResponseDataItemOptional", {'coin': 'str | None', 'name': 'str | None', 'base': 'str | None', 'quote': 'str | None', 'is_canonical': 'bool | None', 'index': 'int | None', 'base_token_index': 'int | None', 'quote_token_index': 'int | None', 'mark_px': 'str | None', 'mid_px': 'str | None', 'prev_day_px': 'str | None', 'day_ntl_vlm': 'str | None', 'day_base_vlm': 'str | None', 'circulating_supply': 'str | None', 'total_supply': 'str | None', 'mark_px_estimated': 'bool | None', 'day_stats_lagged': 'bool | None', 'prev_day_px_seeded': 'bool | None', 'volume_inexact': 'bool | None', 'book_pending': 'bool | None', 'supply_as_of': 'int | None', 'supply_approx': 'bool | None', 'supply_unavailable': 'bool | None'}, total=False)
class ListSpotMarketsResponseDataItem(_ListSpotMarketsResponseDataItemOptional):
    pass

class ListSpotMarketsResponse(TypedDict):
    data: list[ListSpotMarketsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListSpotTokensRequestOptional = TypedDict("_ListSpotTokensRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListSpotTokensRequest(_ListSpotTokensRequestOptional):
    pass

_ListSpotTokensResponseDataItemOptional = TypedDict("_ListSpotTokensResponseDataItemOptional", {'name': 'str | None', 'index': 'int | None', 'token_id': 'str | None', 'full_name': 'str | None', 'sz_decimals': 'int | None', 'wei_decimals': 'int | None', 'is_canonical': 'bool | None', 'evm_contract': 'str | None', 'evm_extra_wei_decimals': 'int | None', 'deployer_trading_fee_share': 'str | None', 'deployer_label': 'str | None'}, total=False)
class ListSpotTokensResponseDataItem(_ListSpotTokensResponseDataItemOptional):
    pass

class ListSpotTokensResponse(TypedDict):
    data: list[ListSpotTokensResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListSpotTwapsRequestOptional = TypedDict("_ListSpotTwapsRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListSpotTwapsRequest(_ListSpotTwapsRequestOptional):
    token: str

_ListSpotTwapsResponseDataItemOptional = TypedDict("_ListSpotTwapsResponseDataItemOptional", {'address': 'str | None', 'twap_id': 'str | None', 'pair': 'str | None', 'pair_id': 'str | None', 'side': 'str | None', 'total_sz': 'str | None', 'executed_sz': 'str | None', 'remaining_sz': 'str | None', 'executed_notional': 'str | None', 'progress_pct': 'str | None', 'duration_s': 'int | None', 'start_time': 'int | None', 'reduce_only': 'bool | None', 'randomize': 'bool | None', 'next_slice_time': 'str | None', 'slices_done': 'int | None', 'waiting': 'bool | None', 'stop_px': 'str | None', 'trigger_px': 'str | None', 'trigger_above': 'bool | None'}, total=False)
class ListSpotTwapsResponseDataItem(_ListSpotTwapsResponseDataItemOptional):
    pass

class ListSpotTwapsResponse(TypedDict):
    data: list[ListSpotTwapsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListStakerChangesRequestOptional = TypedDict("_ListStakerChangesRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'sort': 'ListStakerChangesRequestSort | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListStakerChangesRequest(_ListStakerChangesRequestOptional):
    pass

_ListStakerChangesResponseDataItemOptional = TypedDict("_ListStakerChangesResponseDataItemOptional", {'address': 'str | None', 'stake_from': 'str | None', 'stake_to': 'str | None', 'change': 'str | None', 'share_to': 'str | None'}, total=False)
class ListStakerChangesResponseDataItem(_ListStakerChangesResponseDataItemOptional):
    pass

class ListStakerChangesResponse(TypedDict):
    data: list[ListStakerChangesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListSubBuilderEarningsRequestOptional = TypedDict("_ListSubBuilderEarningsRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'builder': 'str | None', 'sub_builder': 'str | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListSubBuilderEarningsRequest(_ListSubBuilderEarningsRequestOptional):
    pass

_ListSubBuilderEarningsResponseDataItemOptional = TypedDict("_ListSubBuilderEarningsResponseDataItemOptional", {'sub_builder': 'str | None', 'fee_token': 'str | None', 'builders': 'int | None', 'orders': 'int | None', 'fills': 'int | None', 'notional': 'str | None', 'earned': 'str | None', 'fee_unreported_fills': 'int | None', 'first_fill': 'int | None', 'last_fill': 'int | None'}, total=False)
class ListSubBuilderEarningsResponseDataItem(_ListSubBuilderEarningsResponseDataItemOptional):
    pass

class ListSubBuilderEarningsResponse(TypedDict):
    data: list[ListSubBuilderEarningsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListTopLiquidationsRequestOptional = TypedDict("_ListTopLiquidationsRequestOptional", {'market': 'str | None', 'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListTopLiquidationsRequest(_ListTopLiquidationsRequestOptional):
    pass

_ListTopLiquidationsResponseDataItemOptional = TypedDict("_ListTopLiquidationsResponseDataItemOptional", {'time': 'int | None', 'coin': 'str | None', 'side': 'str | None', 'method': 'str | None', 'address': 'str | None', 'px': 'str | None', 'size': 'str | None', 'notional': 'str | None', 'closed_pnl': 'str | None', 'mark_px': 'str | None'}, total=False)
class ListTopLiquidationsResponseDataItem(_ListTopLiquidationsResponseDataItemOptional):
    pass

class ListTopLiquidationsResponse(TypedDict):
    data: list[ListTopLiquidationsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListTopStakersRequestOptional = TypedDict("_ListTopStakersRequestOptional", {'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListTopStakersRequest(_ListTopStakersRequestOptional):
    pass

_ListTopStakersResponseDataItemOptional = TypedDict("_ListTopStakersResponseDataItemOptional", {'rank': 'int | None', 'address': 'str | None', 'hype': 'str | None', 'validators': 'int | None', 'delegations': 'str | None'}, total=False)
class ListTopStakersResponseDataItem(_ListTopStakersResponseDataItemOptional):
    pass

class ListTopStakersResponse(TypedDict):
    data: list[ListTopStakersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListTopUnrealizedPnlRequestOptional = TypedDict("_ListTopUnrealizedPnlRequestOptional", {'category': 'ListTopUnrealizedPnlRequestCategory | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListTopUnrealizedPnlRequest(_ListTopUnrealizedPnlRequestOptional):
    pass

_ListTopUnrealizedPnlResponseDataItemOptional = TypedDict("_ListTopUnrealizedPnlResponseDataItemOptional", {'rank': 'int | None', 'address': 'str | None', 'unrealized_pnl': 'str | None'}, total=False)
class ListTopUnrealizedPnlResponseDataItem(_ListTopUnrealizedPnlResponseDataItemOptional):
    pass

class ListTopUnrealizedPnlResponse(TypedDict):
    data: list[ListTopUnrealizedPnlResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListTopUnrealizedPositionsRequestOptional = TypedDict("_ListTopUnrealizedPositionsRequestOptional", {'side': 'ListTopUnrealizedPositionsRequestSide | None', 'market': 'str | None', 'min_notional': 'ListTopUnrealizedPositionsRequestMinNotional | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListTopUnrealizedPositionsRequest(_ListTopUnrealizedPositionsRequestOptional):
    pass

_ListTopUnrealizedPositionsResponseDataItemOptional = TypedDict("_ListTopUnrealizedPositionsResponseDataItemOptional", {'address': 'str | None', 'market': 'str | None', 'side': 'str | None', 'size': 'str | None', 'notional': 'str | None', 'entry_px': 'str | None', 'liquidation_px': 'str | None', 'unrealized_pnl': 'str | None', 'funding_pnl': 'str | None', 'account_value': 'str | None', 'leverage_value': 'str | None', 'leverage_mode': 'str | None', 'snapshot_time': 'int | None'}, total=False)
class ListTopUnrealizedPositionsResponseDataItem(_ListTopUnrealizedPositionsResponseDataItemOptional):
    pass

class ListTopUnrealizedPositionsResponse(TypedDict):
    data: list[ListTopUnrealizedPositionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListUnrealizedPnlMoversRequestOptional = TypedDict("_ListUnrealizedPnlMoversRequestOptional", {'interval': 'ListUnrealizedPnlMoversRequestInterval | None', 'direction': 'ListUnrealizedPnlMoversRequestDirection | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListUnrealizedPnlMoversRequest(_ListUnrealizedPnlMoversRequestOptional):
    pass

_ListUnrealizedPnlMoversResponseDataItemOptional = TypedDict("_ListUnrealizedPnlMoversResponseDataItemOptional", {'address': 'str | None', 'upnl_now': 'str | None', 'upnl_then': 'str | None', 'upnl_change': 'str | None', 'notional': 'str | None', 'open_positions': 'int | None', 'account_value': 'str | None', 'snapshot_time': 'int | None', 'baseline_time': 'int | None'}, total=False)
class ListUnrealizedPnlMoversResponseDataItem(_ListUnrealizedPnlMoversResponseDataItemOptional):
    pass

class ListUnrealizedPnlMoversResponse(TypedDict):
    data: list[ListUnrealizedPnlMoversResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListValidatorDelegationsRequestOptional = TypedDict("_ListValidatorDelegationsRequestOptional", {'limit': 'int | None', 'start_time': 'int | None', 'end_time': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListValidatorDelegationsRequest(_ListValidatorDelegationsRequestOptional):
    validator: str

_ListValidatorDelegationsResponseDataItemOptional = TypedDict("_ListValidatorDelegationsResponseDataItemOptional", {'time': 'int | None', 'user': 'str | None', 'kind': 'str | None', 'amount': 'str | None', 'hash': 'str | None', 'block': 'int | None', 'event_index': 'int | None'}, total=False)
class ListValidatorDelegationsResponseDataItem(_ListValidatorDelegationsResponseDataItemOptional):
    pass

class ListValidatorDelegationsResponse(TypedDict):
    data: list[ListValidatorDelegationsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListVaultFollowersRequestOptional = TypedDict("_ListVaultFollowersRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListVaultFollowersRequest(_ListVaultFollowersRequestOptional):
    vault: str

_ListVaultFollowersResponseDataItemOptional = TypedDict("_ListVaultFollowersResponseDataItemOptional", {'follower': 'str | None', 'ownership_fraction': 'str | None', 'ownership_b': 'str | None', 'net_transfers': 'str | None', 'entry_time': 'int | None', 'last_change_time': 'int | None', 'snapshot_time': 'int | None'}, total=False)
class ListVaultFollowersResponseDataItem(_ListVaultFollowersResponseDataItemOptional):
    pass

class ListVaultFollowersResponse(TypedDict):
    data: list[ListVaultFollowersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListVaultsRequestOptional = TypedDict("_ListVaultsRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListVaultsRequest(_ListVaultsRequestOptional):
    pass

_ListVaultsResponseDataItemOptional = TypedDict("_ListVaultsResponseDataItemOptional", {'vault': 'str | None', 'name': 'str | None', 'description': 'str | None', 'leader': 'str | None', 'create_time': 'int | None', 'leader_commission': 'str | None', 'is_closed': 'bool | None', 'allow_deposits': 'bool | None', 'lockup_s': 'str | None', 'always_close_on_withdraw': 'bool | None', 'relationship': 'str | None', 'child_vaults': 'str | None', 'followers': 'int | None', 'snapshot_time': 'int | None'}, total=False)
class ListVaultsResponseDataItem(_ListVaultsResponseDataItemOptional):
    pass

class ListVaultsResponse(TypedDict):
    data: list[ListVaultsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_ListWhaleDepositsRequestOptional = TypedDict("_ListWhaleDepositsRequestOptional", {'hours': 'int | None', 'min_usd': 'ListWhaleDepositsRequestMinUsd | None', 'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class ListWhaleDepositsRequest(_ListWhaleDepositsRequestOptional):
    pass

_ListWhaleDepositsResponseDataItemOptional = TypedDict("_ListWhaleDepositsResponseDataItemOptional", {'time': 'int | None', 'address': 'str | None', 'usdc': 'str | None', 'fee': 'str | None', 'hash': 'str | None'}, total=False)
class ListWhaleDepositsResponseDataItem(_ListWhaleDepositsResponseDataItemOptional):
    pass

class ListWhaleDepositsResponse(TypedDict):
    data: list[ListWhaleDepositsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

class LookupAloPriorityRequestBody(TypedDict):
    oids: list[str]

_LookupAloPriorityRequestOptional = TypedDict("_LookupAloPriorityRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class LookupAloPriorityRequest(_LookupAloPriorityRequestOptional):
    coin: str
    body: LookupAloPriorityRequestBody

_LookupAloPriorityResponseDataItemOptional = TypedDict("_LookupAloPriorityResponseDataItemOptional", {'time': 'int | None', 'oid': 'str | None', 'user': 'str | None', 'side': 'str | None', 'px': 'str | None', 'sz': 'str | None', 'p': 'int | None', 'tif': 'str | None', 'fee_usd': 'str | None', 'block': 'int | None', 'bundle_index': 'int | None', 'action_index': 'int | None', 'order_index': 'int | None'}, total=False)
class LookupAloPriorityResponseDataItem(_LookupAloPriorityResponseDataItemOptional):
    pass

class LookupAloPriorityResponse(TypedDict):
    data: list[LookupAloPriorityResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_MarketPositionBookRequestOptional = TypedDict("_MarketPositionBookRequestOptional", {'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class MarketPositionBookRequest(_MarketPositionBookRequestOptional):
    coin: str
    at: int

_MarketPositionBookResponseDataItemOptional = TypedDict("_MarketPositionBookResponseDataItemOptional", {'kind': 'str | None', 'snapshot_time': 'int | None', 'address': 'str | None', 'size': 'str | None', 'notional': 'str | None', 'entry_px': 'str | None', 'liquidation_px': 'str | None', 'unrealized_pnl': 'str | None', 'leverage': 'str | None', 'still_open': 'bool | None', 'liquidated_since': 'bool | None', 'positions': 'int | None', 'current_snapshot_time': 'int | None'}, total=False)
class MarketPositionBookResponseDataItem(_MarketPositionBookResponseDataItemOptional):
    pass

class MarketPositionBookResponse(TypedDict):
    data: list[MarketPositionBookResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_OptionsIvRequestOptional = TypedDict("_OptionsIvRequestOptional", {'mode': 'OptionsIvRequestMode | None', 'start_time': 'int | None', 'end_time': 'int | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class OptionsIvRequest(_OptionsIvRequestOptional):
    coin: OptionsIvRequestCoin

_OptionsIvResponseDataItemOptional = TypedDict("_OptionsIvResponseDataItemOptional", {'kind': 'str | None', 'snapshot_time': 'int | None', 'data': 'str | None'}, total=False)
class OptionsIvResponseDataItem(_OptionsIvResponseDataItemOptional):
    pass

class OptionsIvResponse(TypedDict):
    data: list[OptionsIvResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_PortfolioLeaderboardRequestOptional = TypedDict("_PortfolioLeaderboardRequestOptional", {'sort': 'PortfolioLeaderboardRequestSort | None', 'window': 'PortfolioLeaderboardRequestWindow | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class PortfolioLeaderboardRequest(_PortfolioLeaderboardRequestOptional):
    pass

_PortfolioLeaderboardResponseDataItemOptional = TypedDict("_PortfolioLeaderboardResponseDataItemOptional", {'rank': 'int | None', 'address': 'str | None', 'display_name': 'str | None', 'label': 'str | None', 'tag': 'str | None', 'account_value': 'str | None', 'unrealized_pnl': 'str | None', 'pnl_day': 'str | None', 'pnl_week': 'str | None', 'pnl_month': 'str | None', 'pnl_alltime': 'str | None', 'winrate': 'str | None', 'total_trades': 'int | None', 'total_volume': 'str | None', 'copy_score': 'str | None', 'top_assets': 'str | None', 'last_trade_at': 'int | None', 'verified': 'bool | None', 'sort_value': 'str | None', 'updated_at': 'int | None'}, total=False)
class PortfolioLeaderboardResponseDataItem(_PortfolioLeaderboardResponseDataItemOptional):
    pass

class PortfolioLeaderboardResponse(TypedDict):
    data: list[PortfolioLeaderboardResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_PostAccountsBatchRequestBodyOptional = TypedDict("_PostAccountsBatchRequestBodyOptional", {'dex': 'str | None'}, total=False)
class PostAccountsBatchRequestBody(_PostAccountsBatchRequestBodyOptional):
    addresses: list[str]

_PostAccountsBatchRequestOptional = TypedDict("_PostAccountsBatchRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class PostAccountsBatchRequest(_PostAccountsBatchRequestOptional):
    body: PostAccountsBatchRequestBody

_PostAccountsBatchResponseDataItemOptional = TypedDict("_PostAccountsBatchResponseDataItemOptional", {'address': 'str | None', 'account_value': 'str | None', 'total_notional': 'str | None', 'total_margin_used': 'str | None', 'cross_maintenance_margin_used': 'str | None', 'withdrawable': 'str | None', 'positions': 'int | None', 'as_of': 'int | None', 'equity': 'str | None', 'spot_value': 'str | None', 'staked_value': 'str | None'}, total=False)
class PostAccountsBatchResponseDataItem(_PostAccountsBatchResponseDataItemOptional):
    pass

class PostAccountsBatchResponse(TypedDict):
    data: list[PostAccountsBatchResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

class PostAccountsBatchPortfolioRequestBody(TypedDict):
    addresses: list[str]

_PostAccountsBatchPortfolioRequestOptional = TypedDict("_PostAccountsBatchPortfolioRequestOptional", {'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class PostAccountsBatchPortfolioRequest(_PostAccountsBatchPortfolioRequestOptional):
    body: PostAccountsBatchPortfolioRequestBody

_PostAccountsBatchPortfolioResponseDataItemOptional = TypedDict("_PostAccountsBatchPortfolioResponseDataItemOptional", {'address': 'str | None', 'abstraction': 'str | None', 'account_value': 'str | None', 'total_notional': 'str | None', 'withdrawable': 'str | None', 'maintenance_margin': 'str | None', 'perp_positions': 'int | None', 'spot_balances': 'int | None', 'spot_usdc': 'str | None', 'as_of': 'int | None', 'equity': 'str | None', 'spot_value': 'str | None', 'staked_value': 'str | None'}, total=False)
class PostAccountsBatchPortfolioResponseDataItem(_PostAccountsBatchPortfolioResponseDataItemOptional):
    pass

class PostAccountsBatchPortfolioResponse(TypedDict):
    data: list[PostAccountsBatchPortfolioResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_SearchWalletsRequestOptional = TypedDict("_SearchWalletsRequestOptional", {'limit': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class SearchWalletsRequest(_SearchWalletsRequestOptional):
    search: str

_SearchWalletsResponseDataItemOptional = TypedDict("_SearchWalletsResponseDataItemOptional", {'address': 'str | None', 'display_name': 'str | None', 'avatar': 'str | None', 'twitter': 'str | None', 'tag': 'str | None', 'verified': 'bool | None', 'label': 'str | None', 'match': 'int | None'}, total=False)
class SearchWalletsResponseDataItem(_SearchWalletsResponseDataItemOptional):
    pass

class SearchWalletsResponse(TypedDict):
    data: list[SearchWalletsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_SnapshotAccountValuesRequestOptional = TypedDict("_SnapshotAccountValuesRequestOptional", {'collateral': 'SnapshotAccountValuesRequestCollateral | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class SnapshotAccountValuesRequest(_SnapshotAccountValuesRequestOptional):
    pass

_SnapshotAccountValuesResponseDataItemOptional = TypedDict("_SnapshotAccountValuesResponseDataItemOptional", {'address': 'str | None', 'account_value': 'str | None', 'long_notional': 'str | None', 'short_notional': 'str | None', 'unified': 'bool | None', 'mode': 'str | None'}, total=False)
class SnapshotAccountValuesResponseDataItem(_SnapshotAccountValuesResponseDataItemOptional):
    pass

class SnapshotAccountValuesResponse(TypedDict):
    data: list[SnapshotAccountValuesResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_SnapshotBorrowersRequestOptional = TypedDict("_SnapshotBorrowersRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class SnapshotBorrowersRequest(_SnapshotBorrowersRequestOptional):
    pass

_SnapshotBorrowersResponseDataItemOptional = TypedDict("_SnapshotBorrowersResponseDataItemOptional", {'address': 'str | None', 'mode': 'str | None', 'borrowed_usd': 'str | None', 'supplied_usd': 'str | None', 'eligible_collateral_usd': 'str | None', 'liquidation_value': 'str | None', 'maintenance': 'str | None', 'wallet_hype_usd': 'str | None', 'wallet_btc_usd': 'str | None', 'unpriced': 'bool | None', 'borrows': 'str | None', 'supplies': 'str | None'}, total=False)
class SnapshotBorrowersResponseDataItem(_SnapshotBorrowersResponseDataItemOptional):
    pass

class SnapshotBorrowersResponse(TypedDict):
    data: list[SnapshotBorrowersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_SnapshotMarketPositionsRequestOptional = TypedDict("_SnapshotMarketPositionsRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class SnapshotMarketPositionsRequest(_SnapshotMarketPositionsRequestOptional):
    coin: str

_SnapshotMarketPositionsResponseDataItemOptional = TypedDict("_SnapshotMarketPositionsResponseDataItemOptional", {'address': 'str | None', 'coin': 'str | None', 'dex': 'int | None', 'size': 'str | None', 'entry_notional': 'str | None', 'entry_px': 'str | None', 'mark_px': 'str | None', 'liquidation_px': 'str | None', 'leverage_type': 'str | None', 'leverage': 'int | None', 'isolated_margin': 'str | None', 'cross_balance': 'str | None', 'funding_accrued': 'str | None', 'margin_table_id': 'int | None', 'account_value': 'str | None', 'spot_usdc': 'str | None', 'abstraction': 'str | None', 'updated_height': 'int | None', 'margin_uncertain': 'bool | None'}, total=False)
class SnapshotMarketPositionsResponseDataItem(_SnapshotMarketPositionsResponseDataItemOptional):
    pass

class SnapshotMarketPositionsResponse(TypedDict):
    data: list[SnapshotMarketPositionsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_SnapshotPortfolioMarginAccountsRequestOptional = TypedDict("_SnapshotPortfolioMarginAccountsRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class SnapshotPortfolioMarginAccountsRequest(_SnapshotPortfolioMarginAccountsRequestOptional):
    pass

_SnapshotPortfolioMarginAccountsResponseDataItemOptional = TypedDict("_SnapshotPortfolioMarginAccountsResponseDataItemOptional", {'address': 'str | None'}, total=False)
class SnapshotPortfolioMarginAccountsResponseDataItem(_SnapshotPortfolioMarginAccountsResponseDataItemOptional):
    pass

class SnapshotPortfolioMarginAccountsResponse(TypedDict):
    data: list[SnapshotPortfolioMarginAccountsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_SnapshotPositionedAccountsRequestOptional = TypedDict("_SnapshotPositionedAccountsRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class SnapshotPositionedAccountsRequest(_SnapshotPositionedAccountsRequestOptional):
    pass

_SnapshotPositionedAccountsResponseDataItemOptional = TypedDict("_SnapshotPositionedAccountsResponseDataItemOptional", {'address': 'str | None', 'dexes': 'str | None'}, total=False)
class SnapshotPositionedAccountsResponseDataItem(_SnapshotPositionedAccountsResponseDataItemOptional):
    pass

class SnapshotPositionedAccountsResponse(TypedDict):
    data: list[SnapshotPositionedAccountsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_SnapshotSpotHoldersRequestOptional = TypedDict("_SnapshotSpotHoldersRequestOptional", {'min_balance': 'str | None', 'max_balance': 'str | None', 'min_value': 'str | None', 'max_value': 'str | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class SnapshotSpotHoldersRequest(_SnapshotSpotHoldersRequestOptional):
    token: str

_SnapshotSpotHoldersResponseDataItemOptional = TypedDict("_SnapshotSpotHoldersResponseDataItemOptional", {'address': 'str | None', 'balance': 'str | None', 'value_usd': 'str | None'}, total=False)
class SnapshotSpotHoldersResponseDataItem(_SnapshotSpotHoldersResponseDataItemOptional):
    pass

class SnapshotSpotHoldersResponse(TypedDict):
    data: list[SnapshotSpotHoldersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_SnapshotTriggerOrdersRequestOptional = TypedDict("_SnapshotTriggerOrdersRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class SnapshotTriggerOrdersRequest(_SnapshotTriggerOrdersRequestOptional):
    coin: str

_SnapshotTriggerOrdersResponseDataItemOptional = TypedDict("_SnapshotTriggerOrdersResponseDataItemOptional", {'address': 'str | None', 'coin': 'str | None', 'side': 'str | None', 'order_type': 'str | None', 'trigger_condition': 'str | None', 'trigger_px': 'str | None', 'limit_px': 'str | None', 'size': 'str | None', 'orig_size': 'str | None', 'oid': 'str | None', 'time': 'int | None', 'reduce_only': 'bool | None', 'is_position_tpsl': 'bool | None', 'tif': 'str | None'}, total=False)
class SnapshotTriggerOrdersResponseDataItem(_SnapshotTriggerOrdersResponseDataItemOptional):
    pass

class SnapshotTriggerOrdersResponse(TypedDict):
    data: list[SnapshotTriggerOrdersResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_TradedAssetsRequestOptional = TypedDict("_TradedAssetsRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class TradedAssetsRequest(_TradedAssetsRequestOptional):
    pass

_TradedAssetsResponseDataItemOptional = TypedDict("_TradedAssetsResponseDataItemOptional", {'coin': 'str | None', 'volume': 'str | None', 'trades': 'int | None', 'traders': 'int | None', 'first_trade_hour': 'int | None'}, total=False)
class TradedAssetsResponseDataItem(_TradedAssetsResponseDataItemOptional):
    pass

class TradedAssetsResponse(TypedDict):
    data: list[TradedAssetsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_TraderClassStatsRequestOptional = TypedDict("_TraderClassStatsRequestOptional", {'year': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class TraderClassStatsRequest(_TraderClassStatsRequestOptional):
    trader_class: TraderClassStatsRequestTraderClass

_TraderClassStatsResponseDataItemOptional = TypedDict("_TraderClassStatsResponseDataItemOptional", {'class': 'str | None', 'traders': 'int | None', 'total_volume': 'str | None', 'total_pnl': 'str | None', 'total_trades': 'int | None', 'avg_winrate': 'str | None', 'avg_sharpe_30d': 'str | None'}, total=False)
class TraderClassStatsResponseDataItem(_TraderClassStatsResponseDataItemOptional):
    pass

class TraderClassStatsResponse(TypedDict):
    data: list[TraderClassStatsResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_TraderCohortRequestOptional = TypedDict("_TraderCohortRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class TraderCohortRequest(_TraderCohortRequestOptional):
    pass

_TraderCohortResponseDataItemOptional = TypedDict("_TraderCohortResponseDataItemOptional", {'address': 'str | None', 'display_name': 'str | None', 'label': 'str | None', 'tag': 'str | None', 'winrate': 'str | None', 'winrate_7d': 'str | None', 'winrate_30d': 'str | None', 'winrate_90d': 'str | None', 'trades_7d': 'int | None', 'trades_30d': 'int | None', 'total_trades': 'int | None', 'long_trades': 'int | None', 'short_trades': 'int | None', 'winning_trades': 'int | None', 'losing_trades': 'int | None', 'pnl_day': 'str | None', 'pnl_week': 'str | None', 'pnl_month': 'str | None', 'pnl_alltime': 'str | None', 'account_value': 'str | None', 'unrealized_pnl': 'str | None', 'total_volume': 'str | None', 'copy_score': 'str | None', 'avg_duration_minutes': 'str | None', 'median_duration_minutes': 'str | None', 'longest_win_streak': 'int | None', 'updated_at': 'int | None'}, total=False)
class TraderCohortResponseDataItem(_TraderCohortResponseDataItemOptional):
    pass

class TraderCohortResponse(TypedDict):
    data: list[TraderCohortResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_TraderProfitabilityWindowRequestOptional = TypedDict("_TraderProfitabilityWindowRequestOptional", {'start_time': 'int | None', 'end_time': 'int | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class TraderProfitabilityWindowRequest(_TraderProfitabilityWindowRequestOptional):
    pass

_TraderProfitabilityWindowResponseDataItemOptional = TypedDict("_TraderProfitabilityWindowResponseDataItemOptional", {'cohort': 'str | None', 'min_usd': 'str | None', 'max_usd': 'str | None', 'traders': 'int | None', 'profitable': 'int | None', 'unprofitable': 'int | None', 'pnl_sum': 'str | None', 'median_roi': 'str | None', 'roi_traders': 'int | None'}, total=False)
class TraderProfitabilityWindowResponseDataItem(_TraderProfitabilityWindowResponseDataItemOptional):
    pass

class TraderProfitabilityWindowResponse(TypedDict):
    data: list[TraderProfitabilityWindowResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_TraderScreenerRequestOptional = TypedDict("_TraderScreenerRequestOptional", {'sort': 'TraderScreenerRequestSort | None', 'order': 'TraderScreenerRequestOrder | None', 'timeframe': 'TraderScreenerRequestTimeframe | None', 'min_account_value': 'TraderScreenerRequestMinAccountValue | None', 'min_pnl': 'TraderScreenerRequestMinPnl | None', 'min_copy_score': 'TraderScreenerRequestMinCopyScore | None', 'tag': 'TraderScreenerRequestTag | None', 'max_account_value': 'TraderScreenerRequestMaxAccountValue | None', 'max_pnl': 'TraderScreenerRequestMaxPnl | None', 'max_copy_score': 'TraderScreenerRequestMaxCopyScore | None', 'winrate_window': 'TraderScreenerRequestWinrateWindow | None', 'min_winrate': 'TraderScreenerRequestMinWinrate | None', 'max_winrate': 'TraderScreenerRequestMaxWinrate | None', 'min_sharpe': 'TraderScreenerRequestMinSharpe | None', 'max_sharpe': 'TraderScreenerRequestMaxSharpe | None', 'drawdown_window': 'TraderScreenerRequestDrawdownWindow | None', 'max_drawdown': 'TraderScreenerRequestMaxDrawdown | None', 'trades_window': 'TraderScreenerRequestTradesWindow | None', 'min_trades': 'TraderScreenerRequestMinTrades | None', 'max_trades': 'TraderScreenerRequestMaxTrades | None', 'verified': 'TraderScreenerRequestVerified | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class TraderScreenerRequest(_TraderScreenerRequestOptional):
    pass

_TraderScreenerResponseDataItemOptional = TypedDict("_TraderScreenerResponseDataItemOptional", {'address': 'str | None', 'rank': 'int | None', 'pnl_day': 'str | None', 'pnl_week': 'str | None', 'pnl_month': 'str | None', 'pnl_alltime': 'str | None', 'total_trades': 'int | None', 'trades_7d': 'int | None', 'trades_30d': 'int | None', 'long_trades': 'int | None', 'short_trades': 'int | None', 'winning_trades': 'int | None', 'losing_trades': 'int | None', 'winrate': 'str | None', 'winrate_7d': 'str | None', 'winrate_30d': 'str | None', 'winrate_90d': 'str | None', 'last_trade_at': 'int | None', 'last_fill_at': 'int | None', 'account_value': 'str | None', 'notional': 'str | None', 'unrealized_pnl': 'str | None', 'display_name': 'str | None', 'twitter': 'str | None', 'tag': 'str | None', 'verified': 'bool | None', 'label': 'str | None', 'sharpe_7d': 'str | None', 'sharpe_30d': 'str | None', 'sortino_7d': 'str | None', 'sortino_30d': 'str | None', 'sortino_90d': 'str | None', 'max_drawdown_week': 'str | None', 'max_drawdown_month': 'str | None', 'max_drawdown_90d': 'str | None', 'gpr_7d': 'str | None', 'gpr_30d': 'str | None', 'gpr_90d': 'str | None', 'gpr_all_time': 'str | None', 'copy_score': 'str | None', 'copy_score_v2': 'str | None', 'top_assets': 'str | None', 'total_volume': 'str | None', 'updated_at': 'int | None', 'avatar': 'str | None'}, total=False)
class TraderScreenerResponseDataItem(_TraderScreenerResponseDataItemOptional):
    pass

class TraderScreenerResponse(TypedDict):
    data: list[TraderScreenerResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

_VenueShareRequestOptional = TypedDict("_VenueShareRequestOptional", {'mode': 'VenueShareRequestMode | None', 'venue': 'str | None', 'start_time': 'int | None', 'end_time': 'int | None', 'limit': 'int | None', 'cursor': 'str | None', 'max_credits': 'int | None', 'credit_price_version': 'int | None'}, total=False)
class VenueShareRequest(_VenueShareRequestOptional):
    pass

_VenueShareResponseDataItemOptional = TypedDict("_VenueShareResponseDataItemOptional", {'venue': 'str | None', 'snapshot_time': 'int | None', 'data': 'str | None', 'name': 'str | None', 'volume_usd': 'str | None', 'oi_usd': 'str | None', 'total_volume_usd': 'str | None', 'total_oi_usd': 'str | None', 'volume_share_pct': 'str | None', 'oi_share_pct': 'str | None', 'volume_rank': 'int | None', 'oi_rank': 'int | None'}, total=False)
class VenueShareResponseDataItem(_VenueShareResponseDataItemOptional):
    pass

class VenueShareResponse(TypedDict):
    data: list[VenueShareResponseDataItem]
    has_more: bool
    next_cursor: str | None
    meta: dict[str, Any]

class GetDatasetsRequest(TypedDict):
    pass

class GetMeRequest(TypedDict):
    pass

class GetStatusRequest(TypedDict):
    pass

class ListJobsRequest(TypedDict):
    pass

_ResponseMetadataOptional = TypedDict("_ResponseMetadataOptional", {'request_id': 'str | None', 'credits_charged': 'int | None', 'credit_price_version': 'int | None', 'credits_low': 'bool | None', 'problem_code': 'str | None', 'retry_after': 'str | None', 'has_more': 'bool | None', 'next_cursor': 'str | None', 'as_of': 'int | None', 'complete_to': 'int | None', 'truncated': 'bool | None', 'max_rows': 'int | None', 'incomplete': 'bool | None'}, total=False)
class ResponseMetadata(_ResponseMetadataOptional):
    pass

class AccountPnl90dResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: AccountPnl90dResponse

class TextResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: str

class AccountPositionHistoryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: AccountPositionHistoryResponse

class BatchRecentTradesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: BatchRecentTradesResponse

class BuilderMembersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: BuilderMembersResponse

class CancelJobResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: CancelJobResponse

class CreateJobResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: CreateJobResponse

class EtfFlowsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: EtfFlowsResponse

class FootprintAllWalletsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: FootprintAllWalletsResponse

class GetAccountResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountResponse

class GetAccountAgentsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountAgentsResponse

class GetAccountAllDexesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountAllDexesResponse

class GetAccountApprovedBuildersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountApprovedBuildersResponse

class GetAccountBorrowLendResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountBorrowLendResponse

class GetAccountBuilderFeeResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountBuilderFeeResponse

class GetAccountCoreWriterActionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountCoreWriterActionsResponse

class GetAccountFeesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountFeesResponse

class GetAccountFillsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountFillsResponse

class GetAccountFundingResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountFundingResponse

class GetAccountL1ActionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountL1ActionsResponse

class GetAccountLeadingVaultsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountLeadingVaultsResponse

class GetAccountLedgerResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountLedgerResponse

class GetAccountMarketResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountMarketResponse

class GetAccountMultisigSignersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountMultisigSignersResponse

class GetAccountOpenOrdersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountOpenOrdersResponse

class GetAccountOrderHistoryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountOrderHistoryResponse

class GetAccountOrderStatusResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountOrderStatusResponse

class GetAccountPnlChartResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountPnlChartResponse

class GetAccountPnlSummaryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountPnlSummaryResponse

class GetAccountPortfolioResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountPortfolioResponse

class GetAccountPortfolioDailyResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountPortfolioDailyResponse

class GetAccountPositionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountPositionsResponse

class GetAccountRateLimitResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountRateLimitResponse

class GetAccountRealizedPnlResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountRealizedPnlResponse

class GetAccountRiskResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountRiskResponse

class GetAccountSpotBalancesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountSpotBalancesResponse

class GetAccountStakingResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountStakingResponse

class GetAccountStakingHistoryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountStakingHistoryResponse

class GetAccountSubaccountHistoryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountSubaccountHistoryResponse

class GetAccountSubaccountsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountSubaccountsResponse

class GetAccountTradesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountTradesResponse

class GetAccountTwapFillsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountTwapFillsResponse

class GetAccountTwapHistoryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountTwapHistoryResponse

class GetAccountTwapsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountTwapsResponse

class GetAccountValueHistoryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountValueHistoryResponse

class GetAccountVaultEquitiesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountVaultEquitiesResponse

class GetAccountsBatchResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountsBatchResponse

class GetAccountsBatchPortfolioResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountsBatchPortfolioResponse

class GetAccountsBatchStatsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountsBatchStatsResponse

class GetAccountsLiquidationRiskResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAccountsLiquidationRiskResponse

class GetActiveTradersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetActiveTradersResponse

class GetActiveTwapPressureResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetActiveTwapPressureResponse

class GetAloPriorityResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAloPriorityResponse

class GetAloPriorityBarsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAloPriorityBarsResponse

class GetAssetScoreLeaderboardResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAssetScoreLeaderboardResponse

class GetAssistanceFundBuybacksResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetAssistanceFundBuybacksResponse

class GetBarFlowsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetBarFlowsResponse

class GetBoxTradersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetBoxTradersResponse

class GetBridgeFlowsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetBridgeFlowsResponse

class GetBuilderSubBuilderSplitResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetBuilderSubBuilderSplitResponse

class GetBuilderVolumeShareResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetBuilderVolumeShareResponse

class GetCandlesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetCandlesResponse

class GetChangelogResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetChangelogResponse

class GetCohortFlowsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetCohortFlowsResponse

class GetCohortPositioningResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetCohortPositioningResponse

class GetCohortPositioningMarketsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetCohortPositioningMarketsResponse

class GetCohortTotalsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetCohortTotalsResponse

class GetCvdResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetCvdResponse

class GetDatasetsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetDatasetsResponse

class GetDexLimitsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetDexLimitsResponse

class GetDexStatusResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetDexStatusResponse

class GetEquitySessionGapsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetEquitySessionGapsResponse

class GetEquitySessionSlotsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetEquitySessionSlotsResponse

class GetEventMovesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetEventMovesResponse

class GetExchangeFeesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetExchangeFeesResponse

class GetExchangeStatusResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetExchangeStatusResponse

class GetFootprintResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetFootprintResponse

class GetFootprintActionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetFootprintActionsResponse

class GetFundingFlowsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetFundingFlowsResponse

class GetFundingMatrixResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetFundingMatrixResponse

class GetFundingRatesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetFundingRatesResponse

class GetFundingScreenerResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetFundingScreenerResponse

class GetFundingTopWalletsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetFundingTopWalletsResponse

class GetHip3OracleBarsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetHip3OracleBarsResponse

class GetHlpResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetHlpResponse

class GetHypeHolderBandsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetHypeHolderBandsResponse

class GetHypeSupplyResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetHypeSupplyResponse

class GetJobResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetJobResponse

class GetJobResultResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetJobResultResponse

class GetL4BookResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetL4BookResponse

class GetLendingReserveHistoryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetLendingReserveHistoryResponse

class GetLiquidationCascadesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetLiquidationCascadesResponse

class GetLiquidationHeatmapResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetLiquidationHeatmapResponse

class GetLiquidationLevelTotalsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetLiquidationLevelTotalsResponse

class GetLiquidationLevelsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetLiquidationLevelsResponse

class GetLiquidationRiskLadderResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetLiquidationRiskLadderResponse

class GetLiquidationSeriesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetLiquidationSeriesResponse

class GetLiquidationTotalsByMarketResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetLiquidationTotalsByMarketResponse

class GetMarketBetaResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetMarketBetaResponse

class GetMarketContextHistoryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetMarketContextHistoryResponse

class GetMarketCorrelationResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetMarketCorrelationResponse

class GetMarketLeaderboardResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetMarketLeaderboardResponse

class GetMarketParticipationResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetMarketParticipationResponse

class GetMarketPositionChangesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetMarketPositionChangesResponse

class GetMarketPositionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetMarketPositionsResponse

class GetMarketStopOrdersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetMarketStopOrdersResponse

class GetMarketTradesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetMarketTradesResponse

class GetMarketTwapHistoryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetMarketTwapHistoryResponse

class GetMarketTwapLevelsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetMarketTwapLevelsResponse

class GetMeResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetMeResponse

class GetNetPositioningByMarketResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetNetPositioningByMarketResponse

class GetNetPositioningSummaryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetNetPositioningSummaryResponse

class GetNewTradersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetNewTradersResponse

class GetOpenInterestResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetOpenInterestResponse

class GetOpenInterestByCategoryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetOpenInterestByCategoryResponse

class GetOpenInterestByDexResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetOpenInterestByDexResponse

class GetOpenInterestChangesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetOpenInterestChangesResponse

class GetOpenInterestDailyResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetOpenInterestDailyResponse

class GetOpenInterestTotalResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetOpenInterestTotalResponse

class GetOrderBookResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetOrderBookResponse

class GetOrderBookAtResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetOrderBookAtResponse

class GetOutcomeActivityResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetOutcomeActivityResponse

class GetOutcomeHoldersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetOutcomeHoldersResponse

class GetOutcomeLeaderboardResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetOutcomeLeaderboardResponse

class GetOutcomeProbabilityResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetOutcomeProbabilityResponse

class GetPnlLeaderboardResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetPnlLeaderboardResponse

class GetPositionCountsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetPositionCountsResponse

class GetPositioningResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetPositioningResponse

class GetPriceChangesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetPriceChangesResponse

class GetPriorityFeeSummaryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetPriorityFeeSummaryResponse

class GetPriorityFeesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetPriorityFeesResponse

class GetPriorityFeesBuilderShareResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetPriorityFeesBuilderShareResponse

class GetPriorityFeesByCoinResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetPriorityFeesByCoinResponse

class GetPriorityFeesTopCoinsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetPriorityFeesTopCoinsResponse

class GetPriorityFillsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetPriorityFillsResponse

class GetQuoteResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetQuoteResponse

class GetRealizedVolatilityResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetRealizedVolatilityResponse

class GetReferralResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetReferralResponse

class GetReturnsMatrixResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetReturnsMatrixResponse

class GetRevenueResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetRevenueResponse

class GetSentimentResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetSentimentResponse

class GetSentimentEventsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetSentimentEventsResponse

class GetSentimentScannerResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetSentimentScannerResponse

class GetSessionReturnsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetSessionReturnsResponse

class GetSortedLeaderboardResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetSortedLeaderboardResponse

class GetSpotTwapLevelsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetSpotTwapLevelsResponse

class GetStakeHistoryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetStakeHistoryResponse

class GetStakerBucketsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetStakerBucketsResponse

class GetStakingFlowsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetStakingFlowsResponse

class GetStatusResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetStatusResponse

class GetStopHeatmapResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetStopHeatmapResponse

class GetStopLevelTotalsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetStopLevelTotalsResponse

class GetStopLevelsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetStopLevelsResponse

class GetStopOrdersAtResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetStopOrdersAtResponse

class GetStopSummaryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetStopSummaryResponse

class GetTokenHolderBandsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetTokenHolderBandsResponse

class GetTokenSupplyResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetTokenSupplyResponse

class GetTopLiquidationLevelsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetTopLiquidationLevelsResponse

class GetTopStopLevelsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetTopStopLevelsResponse

class GetTopTraderConsensusResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetTopTraderConsensusResponse

class GetTopTraderOpensResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetTopTraderOpensResponse

class GetTradeBarsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetTradeBarsResponse

class GetTraderDistributionResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetTraderDistributionResponse

class GetTraderGroupResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetTraderGroupResponse

class GetTraderProfitabilityResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetTraderProfitabilityResponse

class GetTwapPressureResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetTwapPressureResponse

class GetUsdcReserveResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetUsdcReserveResponse

class GetUsdcReserveIntervalsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetUsdcReserveIntervalsResponse

class GetUsdcReserveSettlementsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetUsdcReserveSettlementsResponse

class GetUserGrowthResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetUserGrowthResponse

class GetValidatorRegistryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetValidatorRegistryResponse

class GetValidatorRewardsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetValidatorRewardsResponse

class GetValidatorsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetValidatorsResponse

class GetVaultHistoryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetVaultHistoryResponse

class GetWalletBuildersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletBuildersResponse

class GetWalletCompletedTradesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletCompletedTradesResponse

class GetWalletCopyActivityResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletCopyActivityResponse

class GetWalletCopyScoreResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletCopyScoreResponse

class GetWalletHighlightsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletHighlightsResponse

class GetWalletLeaderboardResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletLeaderboardResponse

class GetWalletMarketsTradedResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletMarketsTradedResponse

class GetWalletOrderFillsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletOrderFillsResponse

class GetWalletOutcomesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletOutcomesResponse

class GetWalletPnlCalendarResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletPnlCalendarResponse

class GetWalletProfilesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletProfilesResponse

class GetWalletReviewResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletReviewResponse

class GetWalletRiskMetricsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletRiskMetricsResponse

class GetWalletSharpeResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletSharpeResponse

class GetWalletStakedHypeResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletStakedHypeResponse

class GetWalletStatsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletStatsResponse

class GetWalletStatsByMarketResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletStatsByMarketResponse

class GetWalletTradeCountsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletTradeCountsResponse

class GetWalletUpnlRankResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletUpnlRankResponse

class GetWalletWindowStatsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletWindowStatsResponse

class GetWalletYearSummaryResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWalletYearSummaryResponse

class GetWindowPricesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: GetWindowPricesResponse

class LiquidationMinutesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: LiquidationMinutesResponse

class LiquidationMixResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: LiquidationMixResponse

class ListAllTwapsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListAllTwapsResponse

class ListCohortMembersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListCohortMembersResponse

class ListCohortTopPositionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListCohortTopPositionsResponse

class ListDeployAuctionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListDeployAuctionsResponse

class ListDexMarketsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListDexMarketsResponse

class ListDexsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListDexsResponse

class ListEvmCoreTransfersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListEvmCoreTransfersResponse

class ListFreshWalletPositionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListFreshWalletPositionsResponse

class ListGossipAuctionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListGossipAuctionsResponse

class ListHip3OracleUpdatesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListHip3OracleUpdatesResponse

class ListJobsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListJobsResponse

class ListLargestOpensResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListLargestOpensResponse

class ListLargestPositionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListLargestPositionsResponse

class ListLendingReservesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListLendingReservesResponse

class ListLiquidationsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListLiquidationsResponse

class ListMarginTablesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListMarginTablesResponse

class ListMarketCatalogResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListMarketCatalogResponse

class ListMarketTwapsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListMarketTwapsResponse

class ListMarketsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListMarketsResponse

class ListMaxMarketOrderNotionalsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListMaxMarketOrderNotionalsResponse

class ListMidsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListMidsResponse

class ListNetworkDeploysResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListNetworkDeploysResponse

class ListOutcomeDefinitionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListOutcomeDefinitionsResponse

class ListOutcomeMarketsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListOutcomeMarketsResponse

class ListOutcomeStatsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListOutcomeStatsResponse

class ListOutcomeWhaleTradesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListOutcomeWhaleTradesResponse

class ListPositionsNearLiquidationResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListPositionsNearLiquidationResponse

class ListPriorityFeeTopUsersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListPriorityFeeTopUsersResponse

class ListSpotMarketsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListSpotMarketsResponse

class ListSpotTokensResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListSpotTokensResponse

class ListSpotTwapsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListSpotTwapsResponse

class ListStakerChangesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListStakerChangesResponse

class ListSubBuilderEarningsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListSubBuilderEarningsResponse

class ListTopLiquidationsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListTopLiquidationsResponse

class ListTopStakersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListTopStakersResponse

class ListTopUnrealizedPnlResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListTopUnrealizedPnlResponse

class ListTopUnrealizedPositionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListTopUnrealizedPositionsResponse

class ListUnrealizedPnlMoversResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListUnrealizedPnlMoversResponse

class ListValidatorDelegationsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListValidatorDelegationsResponse

class ListVaultFollowersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListVaultFollowersResponse

class ListVaultsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListVaultsResponse

class ListWhaleDepositsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: ListWhaleDepositsResponse

class LookupAloPriorityResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: LookupAloPriorityResponse

class MarketPositionBookResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: MarketPositionBookResponse

class OptionsIvResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: OptionsIvResponse

class PortfolioLeaderboardResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: PortfolioLeaderboardResponse

class PostAccountsBatchResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: PostAccountsBatchResponse

class PostAccountsBatchPortfolioResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: PostAccountsBatchPortfolioResponse

class SearchWalletsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: SearchWalletsResponse

class SnapshotAccountValuesResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: SnapshotAccountValuesResponse

class SnapshotBorrowersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: SnapshotBorrowersResponse

class SnapshotMarketPositionsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: SnapshotMarketPositionsResponse

class SnapshotPortfolioMarginAccountsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: SnapshotPortfolioMarginAccountsResponse

class SnapshotPositionedAccountsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: SnapshotPositionedAccountsResponse

class SnapshotSpotHoldersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: SnapshotSpotHoldersResponse

class SnapshotTriggerOrdersResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: SnapshotTriggerOrdersResponse

class TradedAssetsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: TradedAssetsResponse

class TraderClassStatsResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: TraderClassStatsResponse

class TraderCohortResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: TraderCohortResponse

class TraderProfitabilityWindowResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: TraderProfitabilityWindowResponse

class TraderScreenerResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: TraderScreenerResponse

class VenueShareResult(TypedDict):
    status: int
    metadata: ResponseMetadata
    data: VenueShareResponse

