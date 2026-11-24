// Generated from schema/hyperdash.json.
import type { HttpValue } from "./index";
export type CreateJobRequestBodyKind = "account_fills_summary";
export type GetAccountFillsRequestAggregate = "false" | "true";
export type GetAccountLedgerRequestInclude = "staking" | "all";
export type GetAccountOpenOrdersRequestAllDexes = "false" | "true";
export type GetAccountPnlChartRequestWindow = "day" | "week" | "month" | "alltime";
export type GetAccountPortfolioRequestAllDexes = "false" | "true";
export type GetAccountRealizedPnlRequestInterval = "5m" | "1h" | "1d";
export type GetAccountRealizedPnlRequestAnchor = "false" | "true";
export type GetAccountsLiquidationRiskRequestKind = "cross" | "coin_margined";
export type GetAccountsLiquidationRiskRequestSort = "risk" | "equity" | "notional" | "leverage";
export type GetActiveTradersRequestInterval = "1h" | "1d";
export type GetActiveTwapPressureRequestMinVolume = "0" | "1000000" | "10000000";
export type GetAloPriorityBarsRequestInterval = "1m" | "5m" | "15m" | "30m" | "1h" | "4h" | "1d";
export type GetAssetScoreLeaderboardRequestMinTrades = "1" | "5" | "10" | "25" | "50" | "100";
export type GetAssetScoreLeaderboardRequestMinLoss = "0" | "100" | "1000" | "10000" | "100000";
export type GetAssetScoreLeaderboardRequestMinPnl = "0" | "1000" | "10000" | "100000" | "1000000";
export type GetAssetScoreLeaderboardRequestMinAccountValue = "1000" | "10000" | "100000" | "1000000" | "10000000";
export type GetAssetScoreLeaderboardRequestMinCopyScore = "50" | "60" | "70" | "80" | "90";
export type GetAssistanceFundBuybacksRequestInterval = "1h" | "1d";
export type GetBoxTradersRequestMinNotional = "1000" | "10000" | "100000" | "1000000";
export type GetBridgeFlowsRequestInterval = "1h" | "1d";
export type GetCandlesRequestInterval = "1s" | "15s" | "30s" | "1m" | "3m" | "5m" | "15m" | "30m" | "1h" | "2h" | "4h" | "8h" | "12h" | "1d" | "3d" | "1w" | "1M";
export type GetCohortFlowsRequestInterval = "1h" | "1d";
export type GetCohortPositioningRequestCohortType = "all" | "size" | "pnl";
export type GetCohortPositioningMarketsRequestCohortType = "all" | "size" | "pnl";
export type GetCvdRequestInterval = "1h" | "1d";
export type GetExchangeFeesRequestBy = "segment" | "market";
export type GetExchangeFeesRequestInterval = "1h" | "1d";
export type GetFootprintRequestInterval = "1m" | "5m" | "15m" | "30m" | "1h" | "4h" | "1d";
export type GetFootprintRequestTier = "all" | "10k" | "100k";
export type GetFootprintActionsRequestInterval = "1m" | "5m" | "15m" | "1h" | "4h" | "1d";
export type GetFundingMatrixRequestInterval = "1h" | "1d";
export type GetFundingRatesRequestInterval = "1h" | "1d";
export type GetFundingScreenerRequestSort = "highest" | "lowest" | "abs";
export type GetFundingScreenerRequestMinOpenInterestUsd = "0" | "1000000" | "10000000";
export type GetFundingTopWalletsRequestRankBy = "paid" | "received" | "net";
export type GetHip3OracleBarsRequestInterval = "1m" | "5m" | "1h";
export type GetHlpRequestInterval = "1h" | "1d";
export type GetLendingReserveHistoryRequestInterval = "1h" | "1d";
export type GetLiquidationHeatmapRequestInterval = "1h" | "4h" | "1d";
export type GetLiquidationRiskLadderRequestKind = "cross" | "coin_margined";
export type GetLiquidationSeriesRequestInterval = "1h" | "1d";
export type GetMarketBetaRequestDays = "30" | "60" | "90";
export type GetMarketContextHistoryRequestInterval = "1m" | "5m" | "15m" | "30m" | "1h" | "4h" | "1d";
export type GetMarketCorrelationRequestInterval = "1d" | "1h";
export type GetMarketLeaderboardRequestWindow = "24h" | "7d" | "30d" | "ytd" | "alltime";
export type GetMarketLeaderboardRequestSort = "total" | "realized" | "unrealized" | "roi" | "realized_roi" | "unrealized_roi";
export type GetMarketPositionChangesRequestWindow = "300" | "900" | "3600" | "14400" | "86400";
export type GetMarketPositionsRequestSide = "all" | "long" | "short";
export type GetMarketPositionsRequestSort = "notional" | "entry_px" | "liquidation_px" | "unrealized_pnl" | "funding_pnl" | "account_value" | "address";
export type GetMarketPositionsRequestOrder = "desc" | "asc";
export type GetNewTradersRequestInterval = "1h" | "1d";
export type GetOpenInterestRequestInterval = "1m" | "5m" | "15m" | "30m" | "1h" | "4h" | "1d";
export type GetOpenInterestByCategoryRequestInterval = "1h" | "1d";
export type GetOpenInterestByDexRequestInterval = "1h" | "1d";
export type GetOpenInterestTotalRequestInterval = "1h" | "1d";
export type GetOutcomeActivityRequestInterval = "5m" | "1h" | "1d";
export type GetOutcomeHoldersRequestSort = "shares" | "notional";
export type GetOutcomeLeaderboardRequestHours = "24" | "168";
export type GetOutcomeProbabilityRequestInterval = "5m" | "1h" | "1d";
export type GetPnlLeaderboardRequestWindow = "alltime" | "24h" | "7d" | "30d" | "ytd";
export type GetPnlLeaderboardRequestCategory = "all" | "crypto" | "majors" | "memes" | "alts" | "equities" | "indices" | "commodities" | "fx";
export type GetPnlLeaderboardRequestSort = "total" | "realized" | "unrealized" | "roi" | "realized_roi" | "unrealized_roi";
export type GetPnlLeaderboardRequestMinAccountValue = "1000" | "10000" | "100000" | "1000000" | "10000000";
export type GetPnlLeaderboardRequestMaxAccountValue = "10000" | "100000" | "1000000" | "10000000" | "100000000";
export type GetPositioningRequestInterval = "1h" | "1d";
export type GetPriorityFeesRequestInterval = "1h" | "1d";
export type GetPriorityFeesBuilderShareRequestInterval = "1h" | "1d";
export type GetPriorityFeesByCoinRequestKind = "write" | "write_ioc" | "write_alo";
export type GetPriorityFeesTopCoinsRequestInterval = "1h" | "1d";
export type GetPriorityFeesTopCoinsRequestKind = "write" | "write_ioc" | "write_alo";
export type GetRealizedVolatilityRequestInterval = "1h" | "1d";
export type GetRevenueRequestInterval = "1h" | "1d";
export type GetSentimentRequestInterval = "1h" | "4h" | "1d";
export type GetSentimentScannerRequestInterval = "1h" | "4h" | "1d";
export type GetSentimentScannerRequestView = "markets" | "index";
export type GetSortedLeaderboardRequestWindow = "24h" | "7d" | "30d" | "90d" | "ytd" | "alltime";
export type GetSortedLeaderboardRequestSort = "net_pnl" | "volume" | "trade_count" | "win_rate" | "fees";
export type GetSortedLeaderboardRequestCategory = "all" | "crypto" | "majors" | "memes" | "alts" | "equities" | "indices" | "commodities" | "fx";
export type GetStakeHistoryRequestInterval = "1h" | "1d";
export type GetStakingFlowsRequestInterval = "1h" | "1d";
export type GetStopHeatmapRequestInterval = "1h" | "4h" | "1d";
export type GetTopTraderOpensRequestMinutes = "60" | "240" | "1440";
export type GetTradeBarsRequestInterval = "1m" | "5m" | "15m" | "30m" | "1h" | "4h" | "1d";
export type GetTraderDistributionRequestView = "size" | "volume" | "migration";
export type GetTraderDistributionRequestPeriod = "week" | "month";
export type GetTraderGroupRequestGroup = "copytraders" | "equities" | "btc" | "hype" | "cl";
export type GetUserGrowthRequestInterval = "1d" | "1w" | "1M";
export type GetValidatorRewardsRequestInterval = "1h" | "1d";
export type GetWalletCompletedTradesRequestSide = "long" | "short";
export type GetWalletCompletedTradesRequestResult = "win" | "loss";
export type GetWalletCompletedTradesRequestMinNotional = "1000" | "10000" | "100000" | "1000000";
export type GetWalletCompletedTradesRequestSort = "close_time" | "net_pnl" | "notional" | "duration" | "entry_px" | "exit_px" | "funding_pnl" | "coin" | "side";
export type GetWalletCompletedTradesRequestOrder = "desc" | "asc";
export type GetWalletHighlightsRequestSide = "long" | "short";
export type GetWalletLeaderboardRequestWindow = "alltime" | "24h" | "7d" | "30d" | "ytd";
export type GetWalletLeaderboardRequestCategory = "all" | "crypto" | "majors" | "memes" | "alts" | "equities" | "indices" | "commodities" | "fx";
export type GetWalletLeaderboardRequestSort = "total" | "realized" | "unrealized" | "roi" | "realized_roi" | "unrealized_roi";
export type GetWalletUpnlRankRequestCategory = "all" | "crypto" | "majors" | "memes" | "alts" | "equities" | "indices" | "commodities" | "fx";
export type GetWalletWindowStatsRequestWindow = "24h" | "7d" | "30d" | "90d" | "ytd" | "alltime";
export type ListCohortMembersRequestCohort = "small" | "medium" | "large" | "whale" | "apex";
export type ListCohortMembersRequestSort = "account_value" | "notional" | "upnl" | "upnl_loss";
export type ListCohortTopPositionsRequestDirection = "winners" | "losers";
export type ListFreshWalletPositionsRequestMaxAgeDays = "1" | "3" | "7";
export type ListFreshWalletPositionsRequestMinBalance = "0" | "10000" | "100000";
export type ListLargestOpensRequestMinNotional = "0" | "100000" | "1000000";
export type ListLargestPositionsRequestSide = "long" | "short";
export type ListMarketCatalogRequestCategory = "btc" | "eth_majors" | "hype" | "memes" | "equities" | "indices" | "commodities" | "fx" | "other";
export type ListMarketCatalogRequestSector = "majors" | "memes" | "alts" | "stocks" | "indices" | "commodities" | "fx";
export type ListMarketCatalogRequestAssetClass = "crypto" | "stocks" | "indices" | "commodities" | "fx";
export type ListOutcomeDefinitionsRequestKind = "outcomes" | "questions";
export type ListOutcomeMarketsRequestStatus = "active" | "settled" | "all";
export type ListPositionsNearLiquidationRequestMaxDistancePct = "1" | "2" | "5" | "10";
export type ListPositionsNearLiquidationRequestMinNotional = "0" | "100000" | "1000000";
export type ListPriorityFeeTopUsersRequestKind = "all" | "write" | "write_ioc" | "write_alo" | "read";
export type ListStakerChangesRequestSort = "gain" | "loss" | "abs";
export type ListTopUnrealizedPnlRequestCategory = "all" | "crypto" | "stocks" | "indices" | "commodities" | "fx" | "majors";
export type ListTopUnrealizedPositionsRequestSide = "winners" | "losers";
export type ListTopUnrealizedPositionsRequestMinNotional = "10000" | "100000" | "1000000" | "10000000";
export type ListUnrealizedPnlMoversRequestInterval = "1h" | "4h" | "1d" | "7d";
export type ListUnrealizedPnlMoversRequestDirection = "winners" | "losers";
export type ListWhaleDepositsRequestMinUsd = "100000" | "1000000" | "3000000" | "5000000";
export type OptionsIvRequestCoin = "BTC" | "ETH";
export type OptionsIvRequestMode = "latest" | "daily";
export type PortfolioLeaderboardRequestSort = "unrealizedPnl" | "unrealizedPnlPct" | "totalPnl" | "realizedPnlPct" | "totalPnlPct";
export type PortfolioLeaderboardRequestWindow = "day" | "week" | "month" | "alltime";
export type SnapshotAccountValuesRequestCollateral = "hyperliquid:USDC";
export type TraderClassStatsRequestTraderClass = "Scalper" | "Executioner" | "Toxic Taker" | "Sniper" | "Ronin";
export type TraderScreenerRequestSort = "pnlPerpDay" | "pnlPerpWeek" | "pnlPerpMonth" | "pnlPerpAllTime" | "winrate30d" | "winrate7d" | "winrateLifetime" | "totalTrades" | "accountValue" | "notional" | "sharpe30d" | "copyScore" | "copyScoreV2" | "maxDrawdownPerpWeek" | "maxDrawdownPerpMonth";
export type TraderScreenerRequestOrder = "desc" | "asc";
export type TraderScreenerRequestTimeframe = "24h" | "7d" | "30d" | "alltime";
export type TraderScreenerRequestMinAccountValue = "1000" | "10000" | "100000" | "1000000" | "10000000";
export type TraderScreenerRequestMinPnl = "0" | "10000" | "100000" | "1000000";
export type TraderScreenerRequestMinCopyScore = "50" | "70" | "90";
export type TraderScreenerRequestTag = "algo" | "scalp" | "intraday" | "swing" | "position";
export type TraderScreenerRequestMaxAccountValue = "10000" | "100000" | "1000000" | "10000000" | "100000000";
export type TraderScreenerRequestMaxPnl = "0" | "-1000" | "-10000" | "-100000" | "-1000000";
export type TraderScreenerRequestMaxCopyScore = "30" | "50" | "70" | "90";
export type TraderScreenerRequestWinrateWindow = "7d" | "30d" | "lifetime";
export type TraderScreenerRequestMinWinrate = "0.3" | "0.4" | "0.5" | "0.6" | "0.7" | "0.8" | "0.9";
export type TraderScreenerRequestMaxWinrate = "0.1" | "0.2" | "0.3" | "0.4" | "0.5" | "0.6";
export type TraderScreenerRequestMinSharpe = "0" | "1" | "2" | "3" | "5";
export type TraderScreenerRequestMaxSharpe = "0" | "-1" | "-2";
export type TraderScreenerRequestDrawdownWindow = "7d" | "30d";
export type TraderScreenerRequestMaxDrawdown = "0.05" | "0.1" | "0.2" | "0.3" | "0.5";
export type TraderScreenerRequestTradesWindow = "7d" | "30d" | "total";
export type TraderScreenerRequestMinTrades = "1" | "10" | "50" | "100" | "500" | "1000";
export type TraderScreenerRequestMaxTrades = "10" | "50" | "100" | "500" | "1000";
export type TraderScreenerRequestVerified = "true";
export type VenueShareRequestMode = "latest" | "daily" | "hourly";

export interface AccountPnl90dRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface AccountPnl90dResponseDataItem {
  [field: string]: unknown;
  "date"?: string | null;
  "pnl"?: string | null;
  "pnl_90d"?: string | null;
  "as_of"?: string | null;
}

export interface AccountPnl90dResponse {
  [field: string]: unknown;
  "data": Array<AccountPnl90dResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface AccountPositionHistoryRequest {
  "address": string;
  "times": string;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface AccountPositionHistoryResponseDataItem {
  [field: string]: unknown;
  "requested_time"?: number | bigint | null;
  "snapshot_time"?: number | bigint | null;
  "market"?: string | null;
  "size"?: string | null;
  "notional"?: string | null;
  "entry_px"?: string | null;
  "liquidation_px"?: string | null;
  "unrealized_pnl"?: string | null;
  "funding_pnl"?: string | null;
  "positions"?: number | bigint | null;
  "total_unrealized_pnl"?: string | null;
}

export interface AccountPositionHistoryResponse {
  [field: string]: unknown;
  "data": Array<AccountPositionHistoryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface BatchRecentTradesRequest {
  "addresses": string;
  "per_address"?: number | bigint | null;
  "market"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface BatchRecentTradesResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "coin"?: string | null;
  "direction"?: string | null;
  "net_pnl"?: string | null;
  "close_time"?: number | bigint | null;
}

export interface BatchRecentTradesResponse {
  [field: string]: unknown;
  "data": Array<BatchRecentTradesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface BuilderMembersRequest {
  "builder": string;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface BuilderMembersResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
}

export interface BuilderMembersResponse {
  [field: string]: unknown;
  "data": Array<BuilderMembersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface CancelJobRequest {
  "id": string;
}

export interface CancelJobResponseError {
  [field: string]: unknown;
  "code"?: string | null;
  "detail"?: string | null;
}

export interface CancelJobResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "kind"?: string | null;
  "params"?: Record<string, HttpValue> | null;
  "status"?: string | null;
  "created_at"?: string | null;
  "started_at"?: string | null;
  "finished_at"?: string | null;
  "expires_at"?: string | null;
  "result_rows"?: number | bigint | null;
  "compute_units"?: number | bigint | null;
  "result_url"?: string | null;
  "error"?: CancelJobResponseError | null;
}

export interface CreateJobRequestBody {
  "kind": CreateJobRequestBodyKind;
  "params": Record<string, HttpValue>;
}

export interface CreateJobRequest {
  "idempotency_key": string;
  "body": CreateJobRequestBody;
}

export interface CreateJobResponseError {
  [field: string]: unknown;
  "code"?: string | null;
  "detail"?: string | null;
}

export interface CreateJobResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "kind"?: string | null;
  "params"?: Record<string, HttpValue> | null;
  "status"?: string | null;
  "created_at"?: string | null;
  "started_at"?: string | null;
  "finished_at"?: string | null;
  "expires_at"?: string | null;
  "result_rows"?: number | bigint | null;
  "compute_units"?: number | bigint | null;
  "result_url"?: string | null;
  "error"?: CreateJobResponseError | null;
}

export interface EtfFlowsRequest {
  "asset": string;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface EtfFlowsResponseDataItem {
  [field: string]: unknown;
  "date"?: string | null;
  "fund"?: string | null;
  "source"?: string | null;
  "flow_usd"?: string | null;
  "span_days"?: number | bigint | null;
  "prev_date"?: string | null;
  "shares_outstanding"?: string | null;
  "nav_per_share"?: string | null;
  "net_assets"?: string | null;
  "coins_held"?: string | null;
  "fetched_at"?: number | bigint | null;
}

export interface EtfFlowsResponse {
  [field: string]: unknown;
  "data": Array<EtfFlowsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface FootprintAllWalletsRequest {
  "coin": string;
  "from": number | bigint;
  "to": number | bigint;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface FootprintAllWalletsResponseDataItem {
  [field: string]: unknown;
  "side"?: string | null;
  "address"?: string | null;
  "size"?: string | null;
  "notional"?: string | null;
  "fills"?: number | bigint | null;
  "taker_size"?: string | null;
  "taker_notional"?: string | null;
  "taker_fills"?: number | bigint | null;
  "side_wallets"?: number | bigint | null;
  "side_size"?: string | null;
  "side_notional"?: string | null;
  "side_fills"?: number | bigint | null;
  "side_taker_size"?: string | null;
  "side_taker_notional"?: string | null;
  "side_taker_fills"?: number | bigint | null;
}

export interface FootprintAllWalletsResponse {
  [field: string]: unknown;
  "data": Array<FootprintAllWalletsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountRequest {
  "address": string;
  "dex"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountResponseDataItem {
  [field: string]: unknown;
  "account_value"?: string | null;
  "total_notional"?: string | null;
  "total_raw_usd"?: string | null;
  "total_margin_used"?: string | null;
  "cross_account_value"?: string | null;
  "cross_maintenance_margin_used"?: string | null;
  "withdrawable"?: string | null;
  "positions"?: number | bigint | null;
  "as_of"?: number | bigint | null;
  "role"?: string | null;
  "master"?: string | null;
  "agent_of"?: string | null;
  "abstraction"?: string | null;
  "equity"?: string | null;
  "spot_value"?: string | null;
  "staked_value"?: string | null;
  "cross_total_notional"?: string | null;
  "cross_total_raw_usd"?: string | null;
  "cross_total_margin_used"?: string | null;
}

export interface GetAccountResponse {
  [field: string]: unknown;
  "data": Array<GetAccountResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountAgentsRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountAgentsResponseDataItem {
  [field: string]: unknown;
  "name"?: string | null;
  "agent"?: string | null;
  "valid_until"?: number | bigint | null;
}

export interface GetAccountAgentsResponse {
  [field: string]: unknown;
  "data": Array<GetAccountAgentsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountAllDexesRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountAllDexesResponseDataItem {
  [field: string]: unknown;
  "dex"?: string | null;
  "account_value"?: string | null;
  "total_notional"?: string | null;
  "total_margin_used"?: string | null;
  "withdrawable"?: string | null;
  "positions"?: number | bigint | null;
  "as_of"?: number | bigint | null;
  "total_raw_usd"?: string | null;
  "cross_account_value"?: string | null;
  "cross_total_notional"?: string | null;
  "cross_total_raw_usd"?: string | null;
  "cross_total_margin_used"?: string | null;
  "cross_maintenance_margin_used"?: string | null;
}

export interface GetAccountAllDexesResponse {
  [field: string]: unknown;
  "data": Array<GetAccountAllDexesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountApprovedBuildersRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountApprovedBuildersResponseDataItem {
  [field: string]: unknown;
  "builder"?: string | null;
}

export interface GetAccountApprovedBuildersResponse {
  [field: string]: unknown;
  "data": Array<GetAccountApprovedBuildersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountBorrowLendRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountBorrowLendResponseDataItem {
  [field: string]: unknown;
  "token"?: number | bigint | null;
  "borrow_value"?: string | null;
  "borrow_basis"?: string | null;
  "supply_value"?: string | null;
  "supply_basis"?: string | null;
}

export interface GetAccountBorrowLendResponse {
  [field: string]: unknown;
  "data": Array<GetAccountBorrowLendResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountBuilderFeeRequest {
  "address": string;
  "builder": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountBuilderFeeResponseDataItem {
  [field: string]: unknown;
  "builder"?: string | null;
  "max_fee_tenths_bp"?: number | bigint | null;
}

export interface GetAccountBuilderFeeResponse {
  [field: string]: unknown;
  "data": Array<GetAccountBuilderFeeResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountCoreWriterActionsRequest {
  "address": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountCoreWriterActionsResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "role"?: string | null;
  "user"?: string | null;
  "destination"?: string | null;
  "action_type"?: string | null;
  "nonce"?: number | bigint | null;
  "evm_tx_hash"?: string | null;
  "token"?: number | bigint | null;
  "wei"?: string | null;
  "wei_approx"?: string | null;
  "action"?: string | null;
  "block"?: number | bigint | null;
  "event_index"?: number | bigint | null;
}

export interface GetAccountCoreWriterActionsResponse {
  [field: string]: unknown;
  "data": Array<GetAccountCoreWriterActionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountFeesRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountFeesResponseDataItem {
  [field: string]: unknown;
  "taker_rate"?: string | null;
  "maker_rate"?: string | null;
  "spot_taker_rate"?: string | null;
  "spot_maker_rate"?: string | null;
  "referral_discount"?: string | null;
  "staking_discount"?: string | null;
  "volume_14d_taker"?: string | null;
  "volume_14d_maker"?: string | null;
  "staking_bps_of_max_supply"?: string | null;
  "daily_volume"?: string | null;
  "fee_schedule"?: string | null;
  "trial"?: string | null;
  "fee_trial_escrow"?: string | null;
  "next_trial_available_time"?: number | bigint | null;
  "staking_link"?: string | null;
}

export interface GetAccountFeesResponse {
  [field: string]: unknown;
  "data": Array<GetAccountFeesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountFillsRequest {
  "address": string;
  "limit"?: number | bigint | null;
  "aggregate"?: GetAccountFillsRequestAggregate | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountFillsResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "coin"?: string | null;
  "side"?: string | null;
  "px"?: string | null;
  "sz"?: string | null;
  "dir"?: string | null;
  "start_position"?: string | null;
  "closed_pnl"?: string | null;
  "fee"?: string | null;
  "fee_token"?: string | null;
  "crossed"?: boolean | null;
  "hash"?: string | null;
  "oid"?: string | null;
  "tid"?: string | null;
  "twap_id"?: string | null;
  "builder"?: string | null;
  "builder_fee"?: string | null;
  "cloid"?: string | null;
  "liquidated_user"?: string | null;
  "liquidation_mark_px"?: string | null;
  "liquidation_method"?: string | null;
  "deployer_fee"?: string | null;
  "priority_gas"?: string | null;
  "sub_builders"?: string | null;
  "tx_index"?: number | bigint | null;
  "block_number"?: number | bigint | null;
  "block_time"?: number | bigint | null;
}

export interface GetAccountFillsResponse {
  [field: string]: unknown;
  "data": Array<GetAccountFillsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountFundingRequest {
  "address": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountFundingResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "coin"?: string | null;
  "usdc"?: string | null;
  "size"?: string | null;
  "funding_rate"?: string | null;
  "hash"?: string | null;
  "samples"?: number | bigint | null;
}

export interface GetAccountFundingResponse {
  [field: string]: unknown;
  "data": Array<GetAccountFundingResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountL1ActionsRequest {
  "address": string;
  "action_type"?: string | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountL1ActionsResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "action_type"?: string | null;
  "ok"?: boolean | null;
  "error"?: string | null;
  "asset"?: number | bigint | null;
  "counterparty"?: string | null;
  "response_user"?: string | null;
  "vault_address"?: string | null;
  "multi_sig_user"?: string | null;
  "action"?: string | null;
  "block"?: number | bigint | null;
  "bundle_index"?: number | bigint | null;
  "action_index"?: number | bigint | null;
  "inner"?: boolean | null;
}

export interface GetAccountL1ActionsResponse {
  [field: string]: unknown;
  "data": Array<GetAccountL1ActionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountLeadingVaultsRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountLeadingVaultsResponseDataItem {
  [field: string]: unknown;
  "vault"?: string | null;
  "name"?: string | null;
}

export interface GetAccountLeadingVaultsResponse {
  [field: string]: unknown;
  "data": Array<GetAccountLeadingVaultsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountLedgerRequest {
  "address": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "include"?: GetAccountLedgerRequestInclude | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountLedgerResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "hash"?: string | null;
  "type"?: string | null;
  "usdc"?: string | null;
  "token"?: string | null;
  "amount"?: string | null;
  "usdc_value"?: string | null;
  "fee"?: string | null;
  "destination"?: string | null;
  "vault"?: string | null;
  "details"?: string | null;
}

export interface GetAccountLedgerResponse {
  [field: string]: unknown;
  "data": Array<GetAccountLedgerResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountMarketRequest {
  "address": string;
  "coin": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountMarketResponseDataItem {
  [field: string]: unknown;
  "leverage_type"?: string | null;
  "leverage"?: number | bigint | null;
  "max_buy_size"?: string | null;
  "max_sell_size"?: string | null;
  "available_to_buy"?: string | null;
  "available_to_sell"?: string | null;
  "mark_px"?: string | null;
  "leverage_raw_usd"?: string | null;
}

export interface GetAccountMarketResponse {
  [field: string]: unknown;
  "data": Array<GetAccountMarketResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountMultisigSignersRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountMultisigSignersResponseDataItem {
  [field: string]: unknown;
  "signer"?: string | null;
}

export interface GetAccountMultisigSignersResponse {
  [field: string]: unknown;
  "data": Array<GetAccountMultisigSignersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountOpenOrdersRequest {
  "address": string;
  "dex"?: string | null;
  "all_dexes"?: GetAccountOpenOrdersRequestAllDexes | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountOpenOrdersResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "side"?: string | null;
  "limit_px"?: string | null;
  "sz"?: string | null;
  "orig_sz"?: string | null;
  "oid"?: string | null;
  "time"?: number | bigint | null;
  "order_type"?: string | null;
  "tif"?: string | null;
  "reduce_only"?: boolean | null;
  "is_trigger"?: boolean | null;
  "trigger_px"?: string | null;
  "trigger_condition"?: string | null;
  "is_position_tpsl"?: boolean | null;
  "cloid"?: string | null;
  "children"?: string | null;
}

export interface GetAccountOpenOrdersResponse {
  [field: string]: unknown;
  "data": Array<GetAccountOpenOrdersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountOrderHistoryRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountOrderHistoryResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "side"?: string | null;
  "limit_px"?: string | null;
  "sz"?: string | null;
  "orig_sz"?: string | null;
  "oid"?: string | null;
  "time"?: number | bigint | null;
  "order_type"?: string | null;
  "tif"?: string | null;
  "reduce_only"?: boolean | null;
  "is_trigger"?: boolean | null;
  "trigger_px"?: string | null;
  "trigger_condition"?: string | null;
  "is_position_tpsl"?: boolean | null;
  "cloid"?: string | null;
  "children"?: string | null;
  "status"?: string | null;
  "status_time"?: number | bigint | null;
  "builder"?: string | null;
  "builder_fee_tenths_bp"?: number | bigint | null;
  "tx_index"?: number | bigint | null;
}

export interface GetAccountOrderHistoryResponse {
  [field: string]: unknown;
  "data": Array<GetAccountOrderHistoryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountOrderStatusRequest {
  "address": string;
  "oid": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountOrderStatusResponseDataItem {
  [field: string]: unknown;
  "lookup"?: string | null;
  "status"?: string | null;
  "status_time"?: number | bigint | null;
  "coin"?: string | null;
  "side"?: string | null;
  "limit_px"?: string | null;
  "sz"?: string | null;
  "orig_sz"?: string | null;
  "oid"?: string | null;
  "time"?: number | bigint | null;
  "order_type"?: string | null;
  "tif"?: string | null;
  "reduce_only"?: boolean | null;
  "is_trigger"?: boolean | null;
  "trigger_px"?: string | null;
  "trigger_condition"?: string | null;
  "is_position_tpsl"?: boolean | null;
  "cloid"?: string | null;
  "children"?: string | null;
}

export interface GetAccountOrderStatusResponse {
  [field: string]: unknown;
  "data": Array<GetAccountOrderStatusResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountPnlChartRequest {
  "address": string;
  "window"?: GetAccountPnlChartRequestWindow | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountPnlChartResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "account_value"?: string | null;
  "net_flows"?: string | null;
  "pnl"?: string | null;
}

export interface GetAccountPnlChartResponse {
  [field: string]: unknown;
  "data": Array<GetAccountPnlChartResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountPnlSummaryRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountPnlSummaryResponseDataItem {
  [field: string]: unknown;
  "total_pnl"?: string | null;
  "win_rate"?: string | null;
  "loss_rate"?: string | null;
  "total_trades"?: number | bigint | null;
  "total_fees"?: string | null;
  "total_funding"?: string | null;
  "volume_traded"?: string | null;
  "days_active"?: number | bigint | null;
  "account_age_days"?: number | bigint | null;
  "traded_pairs"?: number | bigint | null;
}

export interface GetAccountPnlSummaryResponse {
  [field: string]: unknown;
  "data": Array<GetAccountPnlSummaryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountPortfolioRequest {
  "address": string;
  "all_dexes"?: GetAccountPortfolioRequestAllDexes | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountPortfolioResponseDataItem {
  [field: string]: unknown;
  "kind"?: string | null;
  "dex"?: string | null;
  "coin"?: string | null;
  "size"?: string | null;
  "entry"?: string | null;
  "value"?: string | null;
  "unrealized_pnl"?: string | null;
  "margin_used"?: string | null;
  "liquidation_px"?: string | null;
  "leverage"?: number | bigint | null;
  "hold"?: string | null;
  "leverage_type"?: string | null;
  "max_leverage"?: number | bigint | null;
  "return_on_equity"?: string | null;
  "cum_funding_all_time"?: string | null;
  "cum_funding_since_open"?: string | null;
  "cum_funding_since_change"?: string | null;
  "token"?: number | bigint | null;
}

export interface GetAccountPortfolioResponse {
  [field: string]: unknown;
  "data": Array<GetAccountPortfolioResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountPortfolioDailyRequest {
  "address": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountPortfolioDailyResponseDataItem {
  [field: string]: unknown;
  "day"?: number | bigint | null;
  "close_time"?: number | bigint | null;
  "account_value"?: string | null;
  "daily_pnl"?: string | null;
  "perp_daily_pnl"?: string | null;
  "source"?: string | null;
}

export interface GetAccountPortfolioDailyResponse {
  [field: string]: unknown;
  "data": Array<GetAccountPortfolioDailyResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountPositionsRequest {
  "address": string;
  "dex"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountPositionsResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "size"?: string | null;
  "entry_px"?: string | null;
  "position_value"?: string | null;
  "unrealized_pnl"?: string | null;
  "return_on_equity"?: string | null;
  "liquidation_px"?: string | null;
  "margin_used"?: string | null;
  "max_leverage"?: number | bigint | null;
  "leverage_type"?: string | null;
  "leverage_value"?: number | bigint | null;
  "leverage_raw_usd"?: string | null;
  "cum_funding_all_time"?: string | null;
  "cum_funding_since_open"?: string | null;
  "cum_funding_since_change"?: string | null;
}

export interface GetAccountPositionsResponse {
  [field: string]: unknown;
  "data": Array<GetAccountPositionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountRateLimitRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountRateLimitResponseDataItem {
  [field: string]: unknown;
  "cum_volume"?: string | null;
  "requests_used"?: number | bigint | null;
  "requests_cap"?: number | bigint | null;
  "requests_surplus"?: number | bigint | null;
}

export interface GetAccountRateLimitResponse {
  [field: string]: unknown;
  "data": Array<GetAccountRateLimitResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountRealizedPnlRequest {
  "address": string;
  "interval"?: GetAccountRealizedPnlRequestInterval | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "anchor"?: GetAccountRealizedPnlRequestAnchor | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountRealizedPnlResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "realized_pnl"?: string | null;
  "anchored"?: boolean | null;
}

export interface GetAccountRealizedPnlResponse {
  [field: string]: unknown;
  "data": Array<GetAccountRealizedPnlResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountRiskRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountRiskResponseDataItem {
  [field: string]: unknown;
  "book"?: string | null;
  "dex"?: number | bigint | null;
  "coin"?: string | null;
  "notional"?: string | null;
  "mark_px"?: string | null;
  "maintenance_margin"?: string | null;
  "mm_rate"?: string | null;
  "isolated_margin"?: string | null;
  "liquidation_px"?: string | null;
  "margin_uncertain"?: boolean | null;
  "beta"?: string | null;
  "correlation"?: string | null;
  "beta_fallback"?: boolean | null;
  "btc_move"?: string | null;
}

export interface GetAccountRiskResponse {
  [field: string]: unknown;
  "data": Array<GetAccountRiskResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountSpotBalancesRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountSpotBalancesResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "token"?: number | bigint | null;
  "total"?: string | null;
  "hold"?: string | null;
  "entry_ntl"?: string | null;
  "available_after_maintenance"?: string | null;
}

export interface GetAccountSpotBalancesResponse {
  [field: string]: unknown;
  "data": Array<GetAccountSpotBalancesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountStakingRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountStakingResponseDataItem {
  [field: string]: unknown;
  "validator"?: string | null;
  "amount"?: string | null;
  "locked_until"?: number | bigint | null;
  "undelegated"?: string | null;
  "pending_withdrawal"?: string | null;
  "pending_withdrawals"?: number | bigint | null;
}

export interface GetAccountStakingResponse {
  [field: string]: unknown;
  "data": Array<GetAccountStakingResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountStakingHistoryRequest {
  "address": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountStakingHistoryResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "kind"?: string | null;
  "validator"?: string | null;
  "amount"?: string | null;
  "finalized"?: boolean | null;
  "hash"?: string | null;
  "block"?: number | bigint | null;
  "event_index"?: number | bigint | null;
}

export interface GetAccountStakingHistoryResponse {
  [field: string]: unknown;
  "data": Array<GetAccountStakingHistoryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountSubaccountHistoryRequest {
  "address": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountSubaccountHistoryResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "sub_account"?: string | null;
  "name"?: string | null;
  "hash"?: string | null;
  "block"?: number | bigint | null;
  "event_index"?: number | bigint | null;
}

export interface GetAccountSubaccountHistoryResponse {
  [field: string]: unknown;
  "data": Array<GetAccountSubaccountHistoryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountSubaccountsRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountSubaccountsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "name"?: string | null;
  "account_value"?: string | null;
  "withdrawable"?: string | null;
  "spot_balances"?: number | bigint | null;
  "equity"?: string | null;
  "spot_value"?: string | null;
  "staked_value"?: string | null;
}

export interface GetAccountSubaccountsResponse {
  [field: string]: unknown;
  "data": Array<GetAccountSubaccountsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountTradesRequest {
  "address": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountTradesResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "side"?: string | null;
  "open_time"?: number | bigint | null;
  "close_time"?: number | bigint | null;
  "duration_ms"?: number | bigint | null;
  "entry_px"?: string | null;
  "exit_px"?: string | null;
  "size"?: string | null;
  "max_size"?: string | null;
  "gross_pnl"?: string | null;
  "net_pnl"?: string | null;
  "fees"?: string | null;
  "funding_pnl"?: string | null;
  "fills"?: number | bigint | null;
  "maker_fills"?: number | bigint | null;
  "builder"?: string | null;
  "leverage"?: number | bigint | null;
  "is_cross"?: boolean | null;
  "tx_index"?: number | bigint | null;
  "native_realized_pnl"?: string | null;
  "fees_incomplete"?: boolean | null;
  "history_incomplete"?: boolean | null;
  "close_block_height"?: number | bigint | null;
}

export interface GetAccountTradesResponse {
  [field: string]: unknown;
  "data": Array<GetAccountTradesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountTwapFillsRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountTwapFillsResponseDataItem {
  [field: string]: unknown;
  "twap_id"?: number | bigint | null;
  "time"?: number | bigint | null;
  "coin"?: string | null;
  "side"?: string | null;
  "px"?: string | null;
  "sz"?: string | null;
  "fee"?: string | null;
  "closed_pnl"?: string | null;
  "hash"?: string | null;
  "tid"?: string | null;
  "oid"?: string | null;
  "dir"?: string | null;
  "start_position"?: string | null;
  "crossed"?: boolean | null;
  "fee_token"?: string | null;
  "builder"?: string | null;
  "builder_fee"?: string | null;
  "cloid"?: string | null;
  "tx_index"?: number | bigint | null;
}

export interface GetAccountTwapFillsResponse {
  [field: string]: unknown;
  "data": Array<GetAccountTwapFillsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountTwapHistoryRequest {
  "address": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountTwapHistoryResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "coin"?: string | null;
  "twap_id"?: number | bigint | null;
  "status"?: string | null;
  "error"?: string | null;
  "side"?: string | null;
  "sz"?: string | null;
  "executed_sz"?: string | null;
  "executed_ntl"?: string | null;
  "minutes"?: number | bigint | null;
  "reduce_only"?: boolean | null;
  "randomize"?: boolean | null;
  "created_time"?: number | bigint | null;
  "trigger"?: string | null;
  "stop_px"?: string | null;
  "block"?: number | bigint | null;
  "event_index"?: number | bigint | null;
}

export interface GetAccountTwapHistoryResponse {
  [field: string]: unknown;
  "data": Array<GetAccountTwapHistoryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountTwapsRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountTwapsResponseDataItem {
  [field: string]: unknown;
  "kind"?: string | null;
  "coin"?: string | null;
  "pair"?: string | null;
  "pair_id"?: string | null;
  "twap_id"?: string | null;
  "side"?: string | null;
  "total_sz"?: string | null;
  "executed_sz"?: string | null;
  "remaining_sz"?: string | null;
  "executed_notional"?: string | null;
  "progress_pct"?: string | null;
  "duration_s"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "reduce_only"?: boolean | null;
  "randomize"?: boolean | null;
  "next_slice_time"?: string | null;
  "slices_done"?: number | bigint | null;
  "waiting"?: boolean | null;
  "stop_px"?: string | null;
  "trigger_px"?: string | null;
  "trigger_above"?: boolean | null;
}

export interface GetAccountTwapsResponse {
  [field: string]: unknown;
  "data": Array<GetAccountTwapsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountValueHistoryRequest {
  "address": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountValueHistoryResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "account_value"?: string | null;
}

export interface GetAccountValueHistoryResponse {
  [field: string]: unknown;
  "data": Array<GetAccountValueHistoryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountVaultEquitiesRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountVaultEquitiesResponseDataItem {
  [field: string]: unknown;
  "vault"?: string | null;
  "equity"?: string | null;
  "locked_until"?: number | bigint | null;
}

export interface GetAccountVaultEquitiesResponse {
  [field: string]: unknown;
  "data": Array<GetAccountVaultEquitiesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountsBatchRequest {
  "addresses": string;
  "dex"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountsBatchResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "account_value"?: string | null;
  "total_notional"?: string | null;
  "total_margin_used"?: string | null;
  "cross_maintenance_margin_used"?: string | null;
  "withdrawable"?: string | null;
  "positions"?: number | bigint | null;
  "as_of"?: number | bigint | null;
  "equity"?: string | null;
  "spot_value"?: string | null;
  "staked_value"?: string | null;
}

export interface GetAccountsBatchResponse {
  [field: string]: unknown;
  "data": Array<GetAccountsBatchResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountsBatchPortfolioRequest {
  "addresses": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountsBatchPortfolioResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "abstraction"?: string | null;
  "account_value"?: string | null;
  "total_notional"?: string | null;
  "withdrawable"?: string | null;
  "maintenance_margin"?: string | null;
  "perp_positions"?: number | bigint | null;
  "spot_balances"?: number | bigint | null;
  "spot_usdc"?: string | null;
  "as_of"?: number | bigint | null;
  "equity"?: string | null;
  "spot_value"?: string | null;
  "staked_value"?: string | null;
}

export interface GetAccountsBatchPortfolioResponse {
  [field: string]: unknown;
  "data": Array<GetAccountsBatchPortfolioResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountsBatchStatsRequest {
  "addresses": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountsBatchStatsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "pnl_day"?: string | null;
  "pnl_week"?: string | null;
  "pnl_month"?: string | null;
  "pnl_alltime"?: string | null;
  "total_trades"?: number | bigint | null;
  "trades_7d"?: number | bigint | null;
  "trades_30d"?: number | bigint | null;
  "long_trades"?: number | bigint | null;
  "short_trades"?: number | bigint | null;
  "winning_trades"?: number | bigint | null;
  "losing_trades"?: number | bigint | null;
  "winrate"?: string | null;
  "winrate_7d"?: string | null;
  "winrate_30d"?: string | null;
  "winrate_90d"?: string | null;
  "last_trade_at"?: number | bigint | null;
  "last_fill_at"?: number | bigint | null;
  "account_value"?: string | null;
  "notional"?: string | null;
  "unrealized_pnl"?: string | null;
  "display_name"?: string | null;
  "twitter"?: string | null;
  "tag"?: string | null;
  "verified"?: boolean | null;
  "label"?: string | null;
  "sharpe_7d"?: string | null;
  "sharpe_30d"?: string | null;
  "sortino_7d"?: string | null;
  "sortino_30d"?: string | null;
  "sortino_90d"?: string | null;
  "max_drawdown_week"?: string | null;
  "max_drawdown_month"?: string | null;
  "max_drawdown_90d"?: string | null;
  "gpr_7d"?: string | null;
  "gpr_30d"?: string | null;
  "gpr_90d"?: string | null;
  "gpr_all_time"?: string | null;
  "copy_score"?: string | null;
  "copy_score_v2"?: string | null;
  "top_assets"?: string | null;
  "total_volume"?: string | null;
  "updated_at"?: number | bigint | null;
  "avatar"?: string | null;
}

export interface GetAccountsBatchStatsResponse {
  [field: string]: unknown;
  "data": Array<GetAccountsBatchStatsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAccountsLiquidationRiskRequest {
  "kind"?: GetAccountsLiquidationRiskRequestKind | null;
  "min_equity"?: string | null;
  "min_leverage"?: string | null;
  "sort"?: GetAccountsLiquidationRiskRequestSort | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAccountsLiquidationRiskResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "mode"?: string | null;
  "coin_margined"?: boolean | null;
  "equity"?: string | null;
  "cross_notional"?: string | null;
  "isolated_notional"?: string | null;
  "cross_leverage"?: string | null;
  "btc_delta"?: string | null;
  "maintenance_margin"?: string | null;
  "cushion"?: string | null;
  "collateral_hype_usd"?: string | null;
  "collateral_btc_usd"?: string | null;
  "btc_move"?: string | null;
  "btc_liquidation_px"?: string | null;
  "approximate"?: boolean | null;
  "margin_uncertain"?: boolean | null;
}

export interface GetAccountsLiquidationRiskResponse {
  [field: string]: unknown;
  "data": Array<GetAccountsLiquidationRiskResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetActiveTradersRequest {
  "scope"?: string | null;
  "interval"?: GetActiveTradersRequestInterval | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetActiveTradersResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "traders"?: number | bigint | null;
}

export interface GetActiveTradersResponse {
  [field: string]: unknown;
  "data": Array<GetActiveTradersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetActiveTwapPressureRequest {
  "market"?: string | null;
  "min_volume"?: GetActiveTwapPressureRequestMinVolume | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetActiveTwapPressureResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "buy_notional"?: string | null;
  "sell_notional"?: string | null;
  "net_notional"?: string | null;
  "twaps"?: number | bigint | null;
}

export interface GetActiveTwapPressureResponse {
  [field: string]: unknown;
  "data": Array<GetActiveTwapPressureResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAloPriorityRequest {
  "coin": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAloPriorityResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "oid"?: string | null;
  "user"?: string | null;
  "side"?: string | null;
  "px"?: string | null;
  "sz"?: string | null;
  "p"?: number | bigint | null;
  "tif"?: string | null;
  "fee_usd"?: string | null;
  "block"?: number | bigint | null;
  "bundle_index"?: number | bigint | null;
  "action_index"?: number | bigint | null;
  "order_index"?: number | bigint | null;
}

export interface GetAloPriorityResponse {
  [field: string]: unknown;
  "data": Array<GetAloPriorityResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAloPriorityBarsRequest {
  "coin": string;
  "interval"?: GetAloPriorityBarsRequestInterval | null;
  "payers"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAloPriorityBarsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "user"?: string | null;
  "side"?: string | null;
  "resting"?: number | bigint | null;
  "notional_usd"?: string | null;
  "fee_usd"?: string | null;
  "max_p"?: number | bigint | null;
  "bar_resting"?: number | bigint | null;
  "bar_fee_usd"?: string | null;
  "bar_payers"?: number | bigint | null;
}

export interface GetAloPriorityBarsResponse {
  [field: string]: unknown;
  "data": Array<GetAloPriorityBarsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAssetScoreLeaderboardRequest {
  "coin": string;
  "min_trades"?: GetAssetScoreLeaderboardRequestMinTrades | null;
  "min_loss"?: GetAssetScoreLeaderboardRequestMinLoss | null;
  "min_pnl"?: GetAssetScoreLeaderboardRequestMinPnl | null;
  "min_account_value"?: GetAssetScoreLeaderboardRequestMinAccountValue | null;
  "min_copy_score"?: GetAssetScoreLeaderboardRequestMinCopyScore | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAssetScoreLeaderboardResponseDataItem {
  [field: string]: unknown;
  "rank"?: number | bigint | null;
  "address"?: string | null;
  "asset_score"?: string | null;
  "net_pnl"?: string | null;
  "gains"?: string | null;
  "losses"?: string | null;
  "volume"?: string | null;
  "trades"?: number | bigint | null;
  "winning_trades"?: number | bigint | null;
  "losing_trades"?: number | bigint | null;
  "account_value"?: string | null;
  "copy_score"?: string | null;
  "display_name"?: string | null;
  "avatar"?: string | null;
  "twitter"?: string | null;
  "tag"?: string | null;
  "verified"?: boolean | null;
  "label"?: string | null;
  "last_trade_at"?: number | bigint | null;
  "updated_at"?: number | bigint | null;
  "total"?: number | bigint | null;
}

export interface GetAssetScoreLeaderboardResponse {
  [field: string]: unknown;
  "data": Array<GetAssetScoreLeaderboardResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetAssistanceFundBuybacksRequest {
  "interval"?: GetAssistanceFundBuybacksRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetAssistanceFundBuybacksResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "coin"?: string | null;
  "buy_size"?: string | null;
  "buy_notional"?: string | null;
  "sell_size"?: string | null;
  "sell_notional"?: string | null;
  "fills"?: number | bigint | null;
}

export interface GetAssistanceFundBuybacksResponse {
  [field: string]: unknown;
  "data": Array<GetAssistanceFundBuybacksResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetBarFlowsRequest {
  "coin": string;
  "bar_start": number | bigint;
  "minutes"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetBarFlowsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "side"?: string | null;
  "rank"?: number | bigint | null;
  "address"?: string | null;
  "notional"?: string | null;
  "fills"?: number | bigint | null;
  "badge"?: string | null;
  "bar_volume"?: string | null;
  "bar_trades"?: number | bigint | null;
}

export interface GetBarFlowsResponse {
  [field: string]: unknown;
  "data": Array<GetBarFlowsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetBoxTradersRequest {
  "coin": string;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "min_px"?: string | null;
  "max_px"?: string | null;
  "min_notional"?: GetBoxTradersRequestMinNotional | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetBoxTradersResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "address"?: string | null;
  "stance"?: string | null;
  "rank"?: number | bigint | null;
  "notional"?: string | null;
  "size"?: string | null;
  "avg_px"?: string | null;
  "fills"?: number | bigint | null;
  "badge"?: string | null;
}

export interface GetBoxTradersResponse {
  [field: string]: unknown;
  "data": Array<GetBoxTradersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetBridgeFlowsRequest {
  "interval"?: GetBridgeFlowsRequestInterval | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetBridgeFlowsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "route"?: string | null;
  "token"?: string | null;
  "direction"?: string | null;
  "amount"?: string | null;
  "fee"?: string | null;
  "transfers"?: number | bigint | null;
}

export interface GetBridgeFlowsResponse {
  [field: string]: unknown;
  "data": Array<GetBridgeFlowsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetBuilderSubBuilderSplitRequest {
  "builder": string;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetBuilderSubBuilderSplitResponseDataItem {
  [field: string]: unknown;
  "kind"?: string | null;
  "sub_builder"?: string | null;
  "fee_token"?: string | null;
  "orders"?: number | bigint | null;
  "fills"?: number | bigint | null;
  "notional"?: string | null;
  "fee"?: string | null;
  "fee_unreported_fills"?: number | bigint | null;
  "first_fill"?: number | bigint | null;
  "last_fill"?: number | bigint | null;
}

export interface GetBuilderSubBuilderSplitResponse {
  [field: string]: unknown;
  "data": Array<GetBuilderSubBuilderSplitResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetBuilderVolumeShareRequest {
  "days"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetBuilderVolumeShareResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "hip3_volume"?: string | null;
  "core_volume"?: string | null;
  "hip3_share_pct"?: string | null;
}

export interface GetBuilderVolumeShareResponse {
  [field: string]: unknown;
  "data": Array<GetBuilderVolumeShareResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetCandlesRequest {
  "coin": string;
  "interval"?: GetCandlesRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetCandlesResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "o"?: string | null;
  "h"?: string | null;
  "l"?: string | null;
  "c"?: string | null;
  "v"?: string | null;
  "q"?: string | null;
  "n"?: number | bigint | null;
  "liq_only"?: boolean | null;
}

export interface GetCandlesResponse {
  [field: string]: unknown;
  "data": Array<GetCandlesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetChangelogRequest {
  "since"?: number | bigint | null;
}

export interface GetChangelogResponse {
  [field: string]: unknown;
  "data": Array<Record<string, HttpValue>>;
}

export interface GetCohortFlowsRequest {
  "coin"?: string | null;
  "asset_class"?: string | null;
  "interval"?: GetCohortFlowsRequestInterval | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetCohortFlowsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "cohort"?: string | null;
  "taker_buy_notional"?: string | null;
  "taker_sell_notional"?: string | null;
  "taker_fills"?: number | bigint | null;
}

export interface GetCohortFlowsResponse {
  [field: string]: unknown;
  "data": Array<GetCohortFlowsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetCohortPositioningRequest {
  "coin"?: string | null;
  "cohort_type"?: GetCohortPositioningRequestCohortType | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetCohortPositioningResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "coin"?: string | null;
  "cohort_type"?: string | null;
  "cohort"?: string | null;
  "long_notional"?: string | null;
  "long_traders"?: number | bigint | null;
  "short_notional"?: string | null;
  "short_traders"?: number | bigint | null;
}

export interface GetCohortPositioningResponse {
  [field: string]: unknown;
  "data": Array<GetCohortPositioningResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetCohortPositioningMarketsRequest {
  "time"?: number | bigint | null;
  "cohort_type"?: GetCohortPositioningMarketsRequestCohortType | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetCohortPositioningMarketsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "coin"?: string | null;
  "cohort_type"?: string | null;
  "cohort"?: string | null;
  "long_notional"?: string | null;
  "long_traders"?: number | bigint | null;
  "short_notional"?: string | null;
  "short_traders"?: number | bigint | null;
}

export interface GetCohortPositioningMarketsResponse {
  [field: string]: unknown;
  "data": Array<GetCohortPositioningMarketsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetCohortTotalsRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetCohortTotalsResponseDataItem {
  [field: string]: unknown;
  "cohort"?: string | null;
  "wallets"?: number | bigint | null;
  "account_value"?: string | null;
  "wallets_with_positions"?: number | bigint | null;
  "unrealized_pnl"?: string | null;
  "profitable_wallets"?: number | bigint | null;
  "losing_wallets"?: number | bigint | null;
  "equity_snapshot_time"?: number | bigint | null;
  "positions_snapshot_time"?: number | bigint | null;
  "positions_complete"?: boolean | null;
}

export interface GetCohortTotalsResponse {
  [field: string]: unknown;
  "data": Array<GetCohortTotalsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetCvdRequest {
  "coin": string;
  "interval"?: GetCvdRequestInterval | null;
  "days"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetCvdResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "buy_notional"?: string | null;
  "sell_notional"?: string | null;
  "delta"?: string | null;
  "cvd"?: string | null;
}

export interface GetCvdResponse {
  [field: string]: unknown;
  "data": Array<GetCvdResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetDatasetsResponseDataItemEndpointsItem {
  [field: string]: unknown;
  "route"?: string | null;
  "summary"?: string | null;
  "scope"?: string | null;
  "max_range_ms"?: number | bigint | null;
  "max_limit"?: number | bigint | null;
  "pricing"?: string | null;
}

export interface GetDatasetsResponseDataItem {
  [field: string]: unknown;
  "id"?: string | null;
  "title"?: string | null;
  "description"?: string | null;
  "status"?: string | null;
  "coverage_start"?: string | null;
  "freshness"?: string | null;
  "gaps"?: Array<HttpValue> | null;
  "history_entitlement"?: string | null;
  "endpoints"?: Array<GetDatasetsResponseDataItemEndpointsItem> | null;
}

export interface GetDatasetsResponse {
  [field: string]: unknown;
  "data": Array<GetDatasetsResponseDataItem>;
}

export interface GetDexLimitsRequest {
  "dex": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetDexLimitsResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "oi_cap"?: string | null;
}

export interface GetDexLimitsResponse {
  [field: string]: unknown;
  "data": Array<GetDexLimitsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetDexStatusRequest {
  "dex": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetDexStatusResponseDataItem {
  [field: string]: unknown;
  "dex"?: string | null;
  "total_net_deposit"?: string | null;
}

export interface GetDexStatusResponse {
  [field: string]: unknown;
  "data": Array<GetDexStatusResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetEquitySessionGapsRequest {
  "coin": string;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetEquitySessionGapsResponseDataItem {
  [field: string]: unknown;
  "ny_date"?: string | null;
  "px_before_close"?: string | null;
  "px_at_open"?: string | null;
  "samples_before_close"?: number | bigint | null;
  "samples_after_open"?: number | bigint | null;
}

export interface GetEquitySessionGapsResponse {
  [field: string]: unknown;
  "data": Array<GetEquitySessionGapsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetEquitySessionSlotsRequest {
  "coin": string;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetEquitySessionSlotsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "volume"?: string | null;
  "trades"?: number | bigint | null;
  "liq_notional"?: string | null;
  "liquidations"?: number | bigint | null;
}

export interface GetEquitySessionSlotsResponse {
  [field: string]: unknown;
  "data": Array<GetEquitySessionSlotsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetEventMovesRequest {
  "coin": string;
  "times": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetEventMovesResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "px_before"?: string | null;
  "samples_before"?: number | bigint | null;
  "px_after"?: string | null;
  "samples_after"?: number | bigint | null;
}

export interface GetEventMovesResponse {
  [field: string]: unknown;
  "data": Array<GetEventMovesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetExchangeFeesRequest {
  "by"?: GetExchangeFeesRequestBy | null;
  "segment"?: string | null;
  "coin"?: string | null;
  "interval"?: GetExchangeFeesRequestInterval | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetExchangeFeesResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "key"?: string | null;
  "quote_token"?: string | null;
  "taker_buy_notional"?: string | null;
  "taker_sell_notional"?: string | null;
  "fees_paid"?: string | null;
  "maker_rebates"?: string | null;
  "builder_fees"?: string | null;
  "trades"?: number | bigint | null;
  "fills"?: number | bigint | null;
}

export interface GetExchangeFeesResponse {
  [field: string]: unknown;
  "data": Array<GetExchangeFeesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetExchangeStatusRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetExchangeStatusResponseDataItem {
  [field: string]: unknown;
  "special_statuses"?: string | null;
  "time"?: number | bigint | null;
}

export interface GetExchangeStatusResponse {
  [field: string]: unknown;
  "data": Array<GetExchangeStatusResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetFootprintRequest {
  "coin": string;
  "interval"?: GetFootprintRequestInterval | null;
  "row_size"?: string | null;
  "tier"?: GetFootprintRequestTier | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetFootprintResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "px"?: string | null;
  "buy_sz"?: string | null;
  "sell_sz"?: string | null;
  "buy_n"?: number | bigint | null;
  "sell_n"?: number | bigint | null;
  "final"?: boolean | null;
}

export interface GetFootprintResponse {
  [field: string]: unknown;
  "data": Array<GetFootprintResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetFootprintActionsRequest {
  "coin": string;
  "interval"?: GetFootprintActionsRequestInterval | null;
  "row_size"?: string | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetFootprintActionsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "px"?: string | null;
  "ol_taker_sz"?: string | null;
  "ol_maker_sz"?: string | null;
  "cl_taker_sz"?: string | null;
  "cl_maker_sz"?: string | null;
  "os_taker_sz"?: string | null;
  "os_maker_sz"?: string | null;
  "cs_taker_sz"?: string | null;
  "cs_maker_sz"?: string | null;
  "ol_taker_ntl"?: string | null;
  "ol_maker_ntl"?: string | null;
  "cl_taker_ntl"?: string | null;
  "cl_maker_ntl"?: string | null;
  "os_taker_ntl"?: string | null;
  "os_maker_ntl"?: string | null;
  "cs_taker_ntl"?: string | null;
  "cs_maker_ntl"?: string | null;
  "taker_legs"?: number | bigint | null;
  "maker_legs"?: number | bigint | null;
  "row_size"?: string | null;
}

export interface GetFootprintActionsResponse {
  [field: string]: unknown;
  "data": Array<GetFootprintActionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetFundingFlowsRequest {
  "coin"?: string | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetFundingFlowsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "side"?: string | null;
  "paid"?: string | null;
  "received"?: string | null;
  "payments"?: number | bigint | null;
  "payers"?: number | bigint | null;
  "earners"?: number | bigint | null;
}

export interface GetFundingFlowsResponse {
  [field: string]: unknown;
  "data": Array<GetFundingFlowsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetFundingMatrixRequest {
  "market"?: string | null;
  "top"?: number | bigint | null;
  "interval"?: GetFundingMatrixRequestInterval | null;
  "days"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetFundingMatrixResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "t"?: number | bigint | null;
  "funding_rate"?: string | null;
  "funding_apr_pct"?: string | null;
}

export interface GetFundingMatrixResponse {
  [field: string]: unknown;
  "data": Array<GetFundingMatrixResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetFundingRatesRequest {
  "coin": string;
  "interval"?: GetFundingRatesRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetFundingRatesResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "rate"?: string | null;
  "hours"?: number | bigint | null;
}

export interface GetFundingRatesResponse {
  [field: string]: unknown;
  "data": Array<GetFundingRatesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetFundingScreenerRequest {
  "sort"?: GetFundingScreenerRequestSort | null;
  "min_open_interest_usd"?: GetFundingScreenerRequestMinOpenInterestUsd | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetFundingScreenerResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "funding_rate"?: string | null;
  "funding_apr_pct"?: string | null;
  "mark_px"?: string | null;
  "open_interest_usd"?: string | null;
  "day_volume"?: string | null;
  "time"?: number | bigint | null;
}

export interface GetFundingScreenerResponse {
  [field: string]: unknown;
  "data": Array<GetFundingScreenerResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetFundingTopWalletsRequest {
  "coin": string;
  "rank_by"?: GetFundingTopWalletsRequestRankBy | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetFundingTopWalletsResponseDataItem {
  [field: string]: unknown;
  "rank"?: number | bigint | null;
  "address"?: string | null;
  "paid"?: string | null;
  "received"?: string | null;
  "net"?: string | null;
  "payments"?: number | bigint | null;
}

export interface GetFundingTopWalletsResponse {
  [field: string]: unknown;
  "data": Array<GetFundingTopWalletsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetHip3OracleBarsRequest {
  "coin": string;
  "interval"?: GetHip3OracleBarsRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetHip3OracleBarsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "oracle_open"?: string | null;
  "oracle_high"?: string | null;
  "oracle_low"?: string | null;
  "oracle_close"?: string | null;
  "mark_close"?: string | null;
  "external_perp_close"?: string | null;
  "oracle_daily_px"?: string | null;
  "updates"?: number | bigint | null;
  "fallback_updates"?: number | bigint | null;
}

export interface GetHip3OracleBarsResponse {
  [field: string]: unknown;
  "data": Array<GetHip3OracleBarsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetHlpRequest {
  "interval"?: GetHlpRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetHlpResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "strategy"?: string | null;
  "account_value"?: string | null;
  "pnl"?: string | null;
}

export interface GetHlpResponse {
  [field: string]: unknown;
  "data": Array<GetHlpResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetHypeHolderBandsRequest {
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetHypeHolderBandsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "band"?: string | null;
  "wallets"?: number | bigint | null;
  "held"?: string | null;
}

export interface GetHypeHolderBandsResponse {
  [field: string]: unknown;
  "data": Array<GetHypeHolderBandsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetHypeSupplyRequest {
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetHypeSupplyResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "circulating_supply"?: string | null;
  "total_supply"?: string | null;
  "max_supply"?: string | null;
  "future_emissions"?: string | null;
  "mark_px"?: string | null;
  "snapshot_time"?: number | bigint | null;
}

export interface GetHypeSupplyResponse {
  [field: string]: unknown;
  "data": Array<GetHypeSupplyResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetJobRequest {
  "id": string;
}

export interface GetJobResponseError {
  [field: string]: unknown;
  "code"?: string | null;
  "detail"?: string | null;
}

export interface GetJobResponse {
  [field: string]: unknown;
  "id"?: string | null;
  "kind"?: string | null;
  "params"?: Record<string, HttpValue> | null;
  "status"?: string | null;
  "created_at"?: string | null;
  "started_at"?: string | null;
  "finished_at"?: string | null;
  "expires_at"?: string | null;
  "result_rows"?: number | bigint | null;
  "compute_units"?: number | bigint | null;
  "result_url"?: string | null;
  "error"?: GetJobResponseError | null;
}

export interface GetJobResultRequest {
  "id": string;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
}

export interface GetJobResultResponse {
  [field: string]: unknown;
  "data"?: Array<Record<string, HttpValue>> | null;
  "has_more"?: boolean | null;
  "next_cursor"?: string | null;
  "meta"?: Record<string, HttpValue> | null;
}

export interface GetL4BookRequest {
  "coin": string;
  "band_bps"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetL4BookResponseDataItem {
  [field: string]: unknown;
  "side"?: string | null;
  "px"?: string | null;
  "sz"?: string | null;
  "orig_sz"?: string | null;
  "oid"?: string | null;
  "user"?: string | null;
  "time"?: number | bigint | null;
  "tif"?: string | null;
  "order_type"?: string | null;
  "reduce_only"?: boolean | null;
  "is_trigger"?: boolean | null;
  "trigger_px"?: string | null;
}

export interface GetL4BookResponse {
  [field: string]: unknown;
  "data": Array<GetL4BookResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetLendingReserveHistoryRequest {
  "token": number | bigint;
  "interval"?: GetLendingReserveHistoryRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetLendingReserveHistoryResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "snapshot_time"?: number | bigint | null;
  "token_name"?: string | null;
  "supply_apr"?: string | null;
  "borrow_apr"?: string | null;
  "utilization"?: string | null;
  "total_supplied"?: string | null;
  "total_borrowed"?: string | null;
  "available"?: string | null;
  "oracle_px"?: string | null;
  "ltv"?: string | null;
}

export interface GetLendingReserveHistoryResponse {
  [field: string]: unknown;
  "data": Array<GetLendingReserveHistoryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetLiquidationCascadesRequest {
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetLiquidationCascadesResponseDataItem {
  [field: string]: unknown;
  "day"?: number | bigint | null;
  "lead_coin"?: string | null;
  "peak_minute"?: number | bigint | null;
  "peak_minute_notional"?: string | null;
  "notional"?: string | null;
  "long_notional"?: string | null;
  "short_notional"?: string | null;
  "events"?: number | bigint | null;
}

export interface GetLiquidationCascadesResponse {
  [field: string]: unknown;
  "data": Array<GetLiquidationCascadesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetLiquidationHeatmapRequest {
  "coin": string;
  "interval"?: GetLiquidationHeatmapRequestInterval | null;
  "min_px"?: string | null;
  "max_px"?: string | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetLiquidationHeatmapResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "band_min"?: string | null;
  "band_max"?: string | null;
  "long_size"?: string | null;
  "short_size"?: string | null;
  "positions"?: number | bigint | null;
  "mark_px"?: string | null;
  "band_size"?: string | null;
}

export interface GetLiquidationHeatmapResponse {
  [field: string]: unknown;
  "data": Array<GetLiquidationHeatmapResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetLiquidationLevelTotalsRequest {
  "coin": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetLiquidationLevelTotalsResponseDataItem {
  [field: string]: unknown;
  "long_size"?: string | null;
  "long_positions"?: number | bigint | null;
  "short_size"?: string | null;
  "short_positions"?: number | bigint | null;
}

export interface GetLiquidationLevelTotalsResponse {
  [field: string]: unknown;
  "data": Array<GetLiquidationLevelTotalsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetLiquidationLevelsRequest {
  "coin": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetLiquidationLevelsResponseDataItem {
  [field: string]: unknown;
  "side"?: string | null;
  "px"?: string | null;
  "notional"?: string | null;
  "size"?: string | null;
  "positions"?: number | bigint | null;
}

export interface GetLiquidationLevelsResponse {
  [field: string]: unknown;
  "data": Array<GetLiquidationLevelsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetLiquidationRiskLadderRequest {
  "kind"?: GetLiquidationRiskLadderRequestKind | null;
  "min_equity"?: string | null;
  "min_leverage"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetLiquidationRiskLadderResponseDataItem {
  [field: string]: unknown;
  "from_pct"?: number | bigint | null;
  "to_pct"?: number | bigint | null;
  "accounts"?: number | bigint | null;
  "notional"?: string | null;
  "equity"?: string | null;
  "btc_px"?: string | null;
}

export interface GetLiquidationRiskLadderResponse {
  [field: string]: unknown;
  "data": Array<GetLiquidationRiskLadderResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetLiquidationSeriesRequest {
  "coin"?: string | null;
  "asset_class"?: string | null;
  "interval"?: GetLiquidationSeriesRequestInterval | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetLiquidationSeriesResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "side"?: string | null;
  "notional"?: string | null;
  "closed_pnl"?: string | null;
  "events"?: number | bigint | null;
}

export interface GetLiquidationSeriesResponse {
  [field: string]: unknown;
  "data": Array<GetLiquidationSeriesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetLiquidationTotalsByMarketRequest {
  "coin"?: string | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetLiquidationTotalsByMarketResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "events"?: number | bigint | null;
  "total_notional"?: string | null;
  "long_notional"?: string | null;
  "short_notional"?: string | null;
  "closed_pnl"?: string | null;
  "backstop_events"?: number | bigint | null;
  "largest_notional"?: string | null;
  "accounts"?: number | bigint | null;
  "trader_events"?: number | bigint | null;
  "trader_notional"?: string | null;
  "trader_long_notional"?: string | null;
  "trader_short_notional"?: string | null;
  "trader_largest_notional"?: string | null;
  "trader_accounts"?: number | bigint | null;
}

export interface GetLiquidationTotalsByMarketResponse {
  [field: string]: unknown;
  "data": Array<GetLiquidationTotalsByMarketResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetMarketBetaRequest {
  "days"?: GetMarketBetaRequestDays | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetMarketBetaResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "beta"?: string | null;
  "correlation"?: string | null;
  "hours"?: number | bigint | null;
}

export interface GetMarketBetaResponse {
  [field: string]: unknown;
  "data": Array<GetMarketBetaResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetMarketContextHistoryRequest {
  "coin": string;
  "interval"?: GetMarketContextHistoryRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetMarketContextHistoryResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "mark_open"?: string | null;
  "mark_high"?: string | null;
  "mark_low"?: string | null;
  "mark_close"?: string | null;
  "oracle_open"?: string | null;
  "oracle_high"?: string | null;
  "oracle_low"?: string | null;
  "oracle_close"?: string | null;
  "mid_close"?: string | null;
  "premium"?: string | null;
  "predicted_funding"?: string | null;
  "impact_bid"?: string | null;
  "impact_ask"?: string | null;
  "minutes"?: number | bigint | null;
}

export interface GetMarketContextHistoryResponse {
  [field: string]: unknown;
  "data": Array<GetMarketContextHistoryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetMarketCorrelationRequest {
  "coins": string;
  "interval"?: GetMarketCorrelationRequestInterval | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetMarketCorrelationResponseDataItem {
  [field: string]: unknown;
  "coin_a"?: string | null;
  "coin_b"?: string | null;
  "n"?: number | bigint | null;
  "correlation"?: string | null;
}

export interface GetMarketCorrelationResponse {
  [field: string]: unknown;
  "data": Array<GetMarketCorrelationResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetMarketLeaderboardRequest {
  "coin": string;
  "window"?: GetMarketLeaderboardRequestWindow | null;
  "sort"?: GetMarketLeaderboardRequestSort | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetMarketLeaderboardResponseDataItem {
  [field: string]: unknown;
  "rank"?: number | bigint | null;
  "address"?: string | null;
  "display_name"?: string | null;
  "net_pnl"?: string | null;
  "realized_pnl"?: string | null;
  "unrealized_pnl"?: string | null;
  "capital"?: string | null;
  "roi"?: string | null;
  "realized_roi"?: string | null;
  "unrealized_roi"?: string | null;
  "gross_pnl"?: string | null;
  "funding_pnl"?: string | null;
  "fees"?: string | null;
  "volume"?: string | null;
  "trades"?: number | bigint | null;
  "winning_trades"?: number | bigint | null;
  "losing_trades"?: number | bigint | null;
  "account_value"?: string | null;
  "account_value_as_of"?: number | bigint | null;
  "account_value_source"?: string | null;
  "tag"?: string | null;
  "last_trade_at"?: number | bigint | null;
  "label"?: string | null;
  "avatar"?: string | null;
  "twitter"?: string | null;
  "verified"?: boolean | null;
  "copy_score"?: string | null;
  "mark_quality"?: number | bigint | null;
  "window_start"?: number | bigint | null;
  "as_of"?: number | bigint | null;
}

export interface GetMarketLeaderboardResponse {
  [field: string]: unknown;
  "data": Array<GetMarketLeaderboardResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetMarketParticipationRequest {
  "coin"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetMarketParticipationResponseDataItem {
  [field: string]: unknown;
  "market"?: string | null;
  "positions"?: number | bigint | null;
  "longs"?: number | bigint | null;
  "shorts"?: number | bigint | null;
  "profitable"?: number | bigint | null;
  "losing"?: number | bigint | null;
  "long_notional"?: string | null;
  "short_notional"?: string | null;
  "unrealized_pnl"?: string | null;
  "snapshot_time"?: number | bigint | null;
}

export interface GetMarketParticipationResponse {
  [field: string]: unknown;
  "data": Array<GetMarketParticipationResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetMarketPositionChangesRequest {
  "coin": string;
  "window"?: GetMarketPositionChangesRequestWindow | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetMarketPositionChangesResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "previous"?: string | null;
  "current"?: string | null;
  "delta"?: string | null;
}

export interface GetMarketPositionChangesResponse {
  [field: string]: unknown;
  "data": Array<GetMarketPositionChangesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetMarketPositionsRequest {
  "coin": string;
  "side"?: GetMarketPositionsRequestSide | null;
  "sort"?: GetMarketPositionsRequestSort | null;
  "order"?: GetMarketPositionsRequestOrder | null;
  "min_notional"?: string | null;
  "max_notional"?: string | null;
  "min_entry_px"?: string | null;
  "max_entry_px"?: string | null;
  "min_liquidation_px"?: string | null;
  "max_liquidation_px"?: string | null;
  "min_unrealized_pnl"?: string | null;
  "max_unrealized_pnl"?: string | null;
  "min_account_value"?: string | null;
  "max_account_value"?: string | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetMarketPositionsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "side"?: string | null;
  "size"?: string | null;
  "notional"?: string | null;
  "entry_px"?: string | null;
  "liquidation_px"?: string | null;
  "unrealized_pnl"?: string | null;
  "funding_pnl"?: string | null;
  "account_value"?: string | null;
  "leverage"?: number | bigint | null;
  "leverage_type"?: string | null;
  "margin_uncertain"?: boolean | null;
}

export interface GetMarketPositionsResponse {
  [field: string]: unknown;
  "data": Array<GetMarketPositionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetMarketStopOrdersRequest {
  "coin": string;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetMarketStopOrdersResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "side"?: string | null;
  "trigger_px"?: string | null;
  "size"?: string | null;
  "notional"?: string | null;
  "take_profit"?: boolean | null;
  "position_sized"?: boolean | null;
}

export interface GetMarketStopOrdersResponse {
  [field: string]: unknown;
  "data": Array<GetMarketStopOrdersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetMarketTradesRequest {
  "coin": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetMarketTradesResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "tid"?: string | null;
  "side"?: string | null;
  "px"?: string | null;
  "sz"?: string | null;
  "hash"?: string | null;
  "taker"?: string | null;
  "maker"?: string | null;
  "taker_builder"?: string | null;
  "taker_builder_fee"?: string | null;
  "maker_builder"?: string | null;
  "maker_builder_fee"?: string | null;
}

export interface GetMarketTradesResponse {
  [field: string]: unknown;
  "data": Array<GetMarketTradesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetMarketTwapHistoryRequest {
  "coin": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetMarketTwapHistoryResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "user"?: string | null;
  "twap_id"?: number | bigint | null;
  "status"?: string | null;
  "error"?: string | null;
  "side"?: string | null;
  "sz"?: string | null;
  "executed_sz"?: string | null;
  "executed_ntl"?: string | null;
  "minutes"?: number | bigint | null;
  "reduce_only"?: boolean | null;
  "randomize"?: boolean | null;
  "created_time"?: number | bigint | null;
  "trigger"?: string | null;
  "stop_px"?: string | null;
  "block"?: number | bigint | null;
  "event_index"?: number | bigint | null;
}

export interface GetMarketTwapHistoryResponse {
  [field: string]: unknown;
  "data": Array<GetMarketTwapHistoryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetMarketTwapLevelsRequest {
  "coin": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetMarketTwapLevelsResponseDataItem {
  [field: string]: unknown;
  "px"?: string | null;
  "kind"?: string | null;
  "side"?: string | null;
  "when"?: string | null;
  "twaps"?: number | bigint | null;
  "remaining_sz"?: string | null;
  "remaining_notional"?: string | null;
  "flow_sz_per_min"?: string | null;
}

export interface GetMarketTwapLevelsResponse {
  [field: string]: unknown;
  "data": Array<GetMarketTwapLevelsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetMeResponseCredits {
  [field: string]: unknown;
  "included"?: number | bigint | null;
  "used"?: number | bigint | null;
  "reserved"?: number | bigint | null;
  "topup_balance"?: number | bigint | null;
  "remaining"?: number | bigint | null;
  "low"?: boolean | null;
}

export interface GetMeResponse {
  [field: string]: unknown;
  "scopes"?: Array<string> | null;
  "credits"?: GetMeResponseCredits | null;
}

export interface GetNetPositioningByMarketRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetNetPositioningByMarketResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "long_notional"?: string | null;
  "short_notional"?: string | null;
  "net_notional"?: string | null;
  "long_positions"?: number | bigint | null;
  "short_positions"?: number | bigint | null;
  "snapshot_time"?: number | bigint | null;
}

export interface GetNetPositioningByMarketResponse {
  [field: string]: unknown;
  "data": Array<GetNetPositioningByMarketResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetNetPositioningSummaryRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetNetPositioningSummaryResponseDataItem {
  [field: string]: unknown;
  "long_notional"?: string | null;
  "short_notional"?: string | null;
  "net_notional"?: string | null;
  "accounts"?: number | bigint | null;
  "snapshot_time"?: number | bigint | null;
}

export interface GetNetPositioningSummaryResponse {
  [field: string]: unknown;
  "data": Array<GetNetPositioningSummaryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetNewTradersRequest {
  "market"?: string | null;
  "interval"?: GetNewTradersRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetNewTradersResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "traders"?: number | bigint | null;
}

export interface GetNewTradersResponse {
  [field: string]: unknown;
  "data": Array<GetNewTradersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetOpenInterestRequest {
  "coin": string;
  "interval"?: GetOpenInterestRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetOpenInterestResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "open"?: string | null;
  "high"?: string | null;
  "low"?: string | null;
  "close"?: string | null;
  "minutes"?: number | bigint | null;
  "source_rank"?: number | bigint | null;
  "quality"?: number | bigint | null;
}

export interface GetOpenInterestResponse {
  [field: string]: unknown;
  "data": Array<GetOpenInterestResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetOpenInterestByCategoryRequest {
  "interval"?: GetOpenInterestByCategoryRequestInterval | null;
  "days"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetOpenInterestByCategoryResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "category"?: string | null;
  "open_interest_usd"?: string | null;
  "markets"?: number | bigint | null;
}

export interface GetOpenInterestByCategoryResponse {
  [field: string]: unknown;
  "data": Array<GetOpenInterestByCategoryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetOpenInterestByDexRequest {
  "interval"?: GetOpenInterestByDexRequestInterval | null;
  "days"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetOpenInterestByDexResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "dex"?: string | null;
  "open_interest_usd"?: string | null;
  "markets"?: number | bigint | null;
}

export interface GetOpenInterestByDexResponse {
  [field: string]: unknown;
  "data": Array<GetOpenInterestByDexResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetOpenInterestChangesRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetOpenInterestChangesResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "snapshot_time"?: number | bigint | null;
  "oi_usd"?: string | null;
  "mark_px"?: string | null;
  "oi_usd_24h"?: string | null;
  "oi_usd_7d"?: string | null;
  "oi_usd_30d"?: string | null;
}

export interface GetOpenInterestChangesResponse {
  [field: string]: unknown;
  "data": Array<GetOpenInterestChangesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetOpenInterestDailyRequest {
  "coin": string;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetOpenInterestDailyResponseDataItem {
  [field: string]: unknown;
  "day"?: number | bigint | null;
  "oi_usd"?: string | null;
  "mark_px"?: string | null;
  "snapshot_time"?: number | bigint | null;
}

export interface GetOpenInterestDailyResponse {
  [field: string]: unknown;
  "data": Array<GetOpenInterestDailyResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetOpenInterestTotalRequest {
  "market"?: string | null;
  "interval"?: GetOpenInterestTotalRequestInterval | null;
  "days"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetOpenInterestTotalResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "open_interest_usd"?: string | null;
  "markets"?: number | bigint | null;
}

export interface GetOpenInterestTotalResponse {
  [field: string]: unknown;
  "data": Array<GetOpenInterestTotalResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetOrderBookRequest {
  "coin": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetOrderBookResponseDataItem {
  [field: string]: unknown;
  "side"?: string | null;
  "level"?: number | bigint | null;
  "px"?: string | null;
  "sz"?: string | null;
  "orders"?: number | bigint | null;
}

export interface GetOrderBookResponse {
  [field: string]: unknown;
  "data": Array<GetOrderBookResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetOrderBookAtRequest {
  "coin": string;
  "time": number | bigint;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetOrderBookAtResponseDataItem {
  [field: string]: unknown;
  "side"?: string | null;
  "level"?: number | bigint | null;
  "px"?: string | null;
  "sz"?: string | null;
}

export interface GetOrderBookAtResponse {
  [field: string]: unknown;
  "data": Array<GetOrderBookAtResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetOutcomeActivityRequest {
  "outcome"?: number | bigint | null;
  "interval"?: GetOutcomeActivityRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetOutcomeActivityResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "taker_notional"?: string | null;
  "fees"?: string | null;
  "taker_fills"?: number | bigint | null;
  "traders"?: number | bigint | null;
}

export interface GetOutcomeActivityResponse {
  [field: string]: unknown;
  "data": Array<GetOutcomeActivityResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetOutcomeHoldersRequest {
  "outcome": number | bigint;
  "side"?: number | bigint | null;
  "sort"?: GetOutcomeHoldersRequestSort | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetOutcomeHoldersResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "side_index"?: number | bigint | null;
  "shares"?: string | null;
  "cost_basis"?: string | null;
  "avg_entry"?: string | null;
  "realized_pnl"?: string | null;
  "fills"?: number | bigint | null;
  "last_fill_at"?: number | bigint | null;
  "balance_status"?: string | null;
  "balance_checked_at"?: number | bigint | null;
  "mid_px"?: string | null;
  "notional"?: string | null;
  "unrealized_pnl"?: string | null;
  "display_name"?: string | null;
  "avatar"?: string | null;
  "verified"?: boolean | null;
  "share_fees"?: string | null;
  "account_value"?: string | null;
}

export interface GetOutcomeHoldersResponse {
  [field: string]: unknown;
  "data": Array<GetOutcomeHoldersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetOutcomeLeaderboardRequest {
  "hours"?: GetOutcomeLeaderboardRequestHours | null;
  "outcome"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetOutcomeLeaderboardResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "realized_pnl"?: string | null;
  "gross_pnl"?: string | null;
  "fees"?: string | null;
  "yes_notional"?: string | null;
  "fills"?: number | bigint | null;
  "markets"?: number | bigint | null;
  "last_trade_at"?: number | bigint | null;
  "share_fees"?: string | null;
  "taker_share_volume"?: string | null;
  "account_value"?: string | null;
  "display_name"?: string | null;
  "avatar"?: string | null;
  "twitter"?: string | null;
  "verified"?: boolean | null;
}

export interface GetOutcomeLeaderboardResponse {
  [field: string]: unknown;
  "data": Array<GetOutcomeLeaderboardResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetOutcomeProbabilityRequest {
  "outcome": number | bigint;
  "interval"?: GetOutcomeProbabilityRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetOutcomeProbabilityResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "mark_px"?: string | null;
  "mid_px"?: string | null;
  "day_volume"?: string | null;
  "last_time"?: number | bigint | null;
  "trade_px"?: string | null;
  "last_trade_time"?: number | bigint | null;
  "open_interest"?: string | null;
}

export interface GetOutcomeProbabilityResponse {
  [field: string]: unknown;
  "data": Array<GetOutcomeProbabilityResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetPnlLeaderboardRequest {
  "window"?: GetPnlLeaderboardRequestWindow | null;
  "category"?: GetPnlLeaderboardRequestCategory | null;
  "sort"?: GetPnlLeaderboardRequestSort | null;
  "min_trades"?: number | bigint | null;
  "min_account_value"?: GetPnlLeaderboardRequestMinAccountValue | null;
  "max_account_value"?: GetPnlLeaderboardRequestMaxAccountValue | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetPnlLeaderboardResponseDataItem {
  [field: string]: unknown;
  "rank"?: number | bigint | null;
  "address"?: string | null;
  "display_name"?: string | null;
  "tag"?: string | null;
  "net_pnl"?: string | null;
  "realized_pnl"?: string | null;
  "unrealized_pnl"?: string | null;
  "capital"?: string | null;
  "roi"?: string | null;
  "realized_roi"?: string | null;
  "unrealized_roi"?: string | null;
  "gross_pnl"?: string | null;
  "funding_pnl"?: string | null;
  "total_fees"?: string | null;
  "volume"?: string | null;
  "trades"?: number | bigint | null;
  "winning_trades"?: number | bigint | null;
  "losing_trades"?: number | bigint | null;
  "account_value"?: string | null;
  "account_value_as_of"?: number | bigint | null;
  "account_value_source"?: string | null;
  "last_trade_at"?: number | bigint | null;
  "label"?: string | null;
  "avatar"?: string | null;
  "twitter"?: string | null;
  "verified"?: boolean | null;
  "copy_score"?: string | null;
  "mark_quality"?: number | bigint | null;
  "window_start"?: number | bigint | null;
  "as_of"?: number | bigint | null;
}

export interface GetPnlLeaderboardResponse {
  [field: string]: unknown;
  "data": Array<GetPnlLeaderboardResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetPositionCountsRequest {
  "coin": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetPositionCountsResponseDataItem {
  [field: string]: unknown;
  "long_positions"?: number | bigint | null;
  "short_positions"?: number | bigint | null;
  "long_notional"?: string | null;
  "short_notional"?: string | null;
}

export interface GetPositionCountsResponse {
  [field: string]: unknown;
  "data": Array<GetPositionCountsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetPositioningRequest {
  "market"?: string | null;
  "interval"?: GetPositioningRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetPositioningResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "snapshot_time"?: number | bigint | null;
  "long_notional"?: string | null;
  "short_notional"?: string | null;
  "long_upnl"?: string | null;
  "short_upnl"?: string | null;
  "long_positions"?: number | bigint | null;
  "short_positions"?: number | bigint | null;
}

export interface GetPositioningResponse {
  [field: string]: unknown;
  "data": Array<GetPositioningResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetPriceChangesRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetPriceChangesResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "price"?: string | null;
  "price_source"?: string | null;
  "change_24h_pct"?: string | null;
  "change_7d_pct"?: string | null;
  "change_30d_pct"?: string | null;
  "mark_px"?: string | null;
  "px_24h"?: string | null;
  "px_7d"?: string | null;
  "px_30d"?: string | null;
  "day_volume"?: string | null;
  "open_interest"?: string | null;
}

export interface GetPriceChangesResponse {
  [field: string]: unknown;
  "data": Array<GetPriceChangesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetPriorityFeeSummaryRequest {
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetPriorityFeeSummaryResponseDataItem {
  [field: string]: unknown;
  "start"?: number | bigint | null;
  "end"?: number | bigint | null;
  "write_hype"?: string | null;
  "write_usd"?: string | null;
  "write_orders"?: number | bigint | null;
  "read_hype"?: string | null;
  "read_usd"?: string | null;
  "reads"?: number | bigint | null;
  "total_hype"?: string | null;
  "total_usd"?: string | null;
  "total_txs"?: number | bigint | null;
  "avg_write_usd"?: string | null;
  "avg_read_usd"?: string | null;
  "write_wallets"?: number | bigint | null;
  "read_wallets"?: number | bigint | null;
  "wallets"?: number | bigint | null;
  "usd_complete"?: boolean | null;
  "hype_complete"?: boolean | null;
  "write_ioc_hype"?: string | null;
  "write_ioc_usd"?: string | null;
  "write_ioc_orders"?: number | bigint | null;
  "write_ioc_fills"?: number | bigint | null;
  "write_ioc_unpriced_hype"?: string | null;
  "write_alo_hype_estimate"?: string | null;
  "write_alo_usd"?: string | null;
  "write_alo_orders"?: number | bigint | null;
  "write_alo_resting_notional_usd"?: string | null;
  "write_unpriced_hype"?: string | null;
  "unpriced_hype"?: string | null;
  "unconverted_usd"?: string | null;
  "hype_estimated"?: boolean | null;
  "live_from"?: number | bigint | null;
  "write_fills"?: number | bigint | null;
  "read_unpriced_hype"?: string | null;
  "avg_write_ioc_hype"?: string | null;
  "avg_write_ioc_usd"?: string | null;
  "avg_write_alo_hype_estimate"?: string | null;
  "avg_write_alo_usd"?: string | null;
  "avg_write_hype"?: string | null;
  "avg_read_hype"?: string | null;
  "write_ioc_wallets"?: number | bigint | null;
  "write_alo_wallets"?: number | bigint | null;
  "last_hour"?: number | bigint | null;
}

export interface GetPriorityFeeSummaryResponse {
  [field: string]: unknown;
  "data": Array<GetPriorityFeeSummaryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetPriorityFeesRequest {
  "interval"?: GetPriorityFeesRequestInterval | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetPriorityFeesResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "write_hype"?: string | null;
  "write_usd"?: string | null;
  "write_orders"?: number | bigint | null;
  "read_hype"?: string | null;
  "read_usd"?: string | null;
  "reads"?: number | bigint | null;
  "total_hype"?: string | null;
  "total_usd"?: string | null;
  "complete"?: boolean | null;
  "usd_complete"?: boolean | null;
  "hype_complete"?: boolean | null;
  "read_unpriced_hype"?: string | null;
  "total_txs"?: number | bigint | null;
  "write_ioc_hype"?: string | null;
  "write_ioc_usd"?: string | null;
  "write_ioc_orders"?: number | bigint | null;
  "write_ioc_fills"?: number | bigint | null;
  "write_ioc_unpriced_hype"?: string | null;
  "write_alo_hype_estimate"?: string | null;
  "write_alo_usd"?: string | null;
  "write_alo_orders"?: number | bigint | null;
  "write_alo_resting_notional_usd"?: string | null;
  "write_unpriced_hype"?: string | null;
  "unpriced_hype"?: string | null;
  "unconverted_usd"?: string | null;
  "hype_estimated"?: boolean | null;
  "live_from"?: number | bigint | null;
  "cumulative_write_ioc_hype"?: string | null;
  "cumulative_write_ioc_usd"?: string | null;
  "cumulative_write_alo_hype_estimate"?: string | null;
  "cumulative_write_alo_usd"?: string | null;
  "cumulative_write_hype"?: string | null;
  "cumulative_write_usd"?: string | null;
  "cumulative_read_hype"?: string | null;
  "cumulative_read_usd"?: string | null;
  "cumulative_hype"?: string | null;
  "cumulative_usd"?: string | null;
  "cumulative_unconverted_usd"?: string | null;
  "cumulative_unpriced_hype"?: string | null;
  "cumulative_hype_complete"?: boolean | null;
  "cumulative_hype_estimated"?: boolean | null;
  "cumulative_usd_complete"?: boolean | null;
}

export interface GetPriorityFeesResponse {
  [field: string]: unknown;
  "data": Array<GetPriorityFeesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetPriorityFeesBuilderShareRequest {
  "interval"?: GetPriorityFeesBuilderShareRequestInterval | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetPriorityFeesBuilderShareResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "write_usd"?: string | null;
  "hip3_write_usd"?: string | null;
  "hip3_share_usd"?: string | null;
  "write_hype"?: string | null;
  "hip3_write_hype"?: string | null;
  "hip3_share_hype"?: string | null;
  "usd_complete"?: boolean | null;
  "hype_complete"?: boolean | null;
  "write_ioc_hype"?: string | null;
  "write_ioc_usd"?: string | null;
  "write_ioc_orders"?: number | bigint | null;
  "write_ioc_fills"?: number | bigint | null;
  "write_ioc_unpriced_hype"?: string | null;
  "write_alo_hype_estimate"?: string | null;
  "write_alo_usd"?: string | null;
  "write_alo_orders"?: number | bigint | null;
  "write_alo_resting_notional_usd"?: string | null;
  "write_unpriced_hype"?: string | null;
  "unpriced_hype"?: string | null;
  "unconverted_usd"?: string | null;
  "hype_estimated"?: boolean | null;
  "live_from"?: number | bigint | null;
  "hip3_write_ioc_hype"?: string | null;
  "hip3_write_ioc_usd"?: string | null;
  "hip3_write_alo_hype_estimate"?: string | null;
  "hip3_write_alo_usd"?: string | null;
  "hip3_write_ioc_share_usd"?: string | null;
  "hip3_write_ioc_share_hype"?: string | null;
  "hip3_write_alo_share_usd"?: string | null;
  "hip3_write_alo_share_hype_estimate"?: string | null;
  "write_orders"?: number | bigint | null;
}

export interface GetPriorityFeesBuilderShareResponse {
  [field: string]: unknown;
  "data": Array<GetPriorityFeesBuilderShareResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetPriorityFeesByCoinRequest {
  "kind"?: GetPriorityFeesByCoinRequestKind | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetPriorityFeesByCoinResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "dex"?: string | null;
  "asset_class"?: string | null;
  "write_hype"?: string | null;
  "write_usd"?: string | null;
  "write_orders"?: number | bigint | null;
  "write_fills"?: number | bigint | null;
  "share_usd"?: string | null;
  "share_hype"?: string | null;
  "usd_complete"?: boolean | null;
  "hype_complete"?: boolean | null;
  "write_ioc_hype"?: string | null;
  "write_ioc_usd"?: string | null;
  "write_ioc_orders"?: number | bigint | null;
  "write_ioc_fills"?: number | bigint | null;
  "write_ioc_unpriced_hype"?: string | null;
  "write_alo_hype_estimate"?: string | null;
  "write_alo_usd"?: string | null;
  "write_alo_orders"?: number | bigint | null;
  "write_alo_resting_notional_usd"?: string | null;
  "write_unpriced_hype"?: string | null;
  "unpriced_hype"?: string | null;
  "unconverted_usd"?: string | null;
  "hype_estimated"?: boolean | null;
  "live_from"?: number | bigint | null;
  "share_hype_estimated"?: boolean | null;
  "share_hype_complete"?: boolean | null;
}

export interface GetPriorityFeesByCoinResponse {
  [field: string]: unknown;
  "data": Array<GetPriorityFeesByCoinResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetPriorityFeesTopCoinsRequest {
  "top"?: number | bigint | null;
  "interval"?: GetPriorityFeesTopCoinsRequestInterval | null;
  "kind"?: GetPriorityFeesTopCoinsRequestKind | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetPriorityFeesTopCoinsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "coin"?: string | null;
  "dex"?: string | null;
  "write_hype"?: string | null;
  "write_usd"?: string | null;
  "write_orders"?: number | bigint | null;
  "usd_complete"?: boolean | null;
  "hype_complete"?: boolean | null;
  "write_ioc_hype"?: string | null;
  "write_ioc_usd"?: string | null;
  "write_ioc_orders"?: number | bigint | null;
  "write_ioc_fills"?: number | bigint | null;
  "write_ioc_unpriced_hype"?: string | null;
  "write_alo_hype_estimate"?: string | null;
  "write_alo_usd"?: string | null;
  "write_alo_orders"?: number | bigint | null;
  "write_alo_resting_notional_usd"?: string | null;
  "write_unpriced_hype"?: string | null;
  "unpriced_hype"?: string | null;
  "unconverted_usd"?: string | null;
  "hype_estimated"?: boolean | null;
  "live_from"?: number | bigint | null;
}

export interface GetPriorityFeesTopCoinsResponse {
  [field: string]: unknown;
  "data": Array<GetPriorityFeesTopCoinsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetPriorityFillsRequest {
  "coin": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetPriorityFillsResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "user"?: string | null;
  "side"?: string | null;
  "tid"?: string | null;
  "oid"?: string | null;
  "px"?: string | null;
  "sz"?: string | null;
  "crossed"?: boolean | null;
  "priority_gas"?: string | null;
  "hash"?: string | null;
}

export interface GetPriorityFillsResponse {
  [field: string]: unknown;
  "data": Array<GetPriorityFillsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetQuoteRequest {
  "route": string;
  "range_ms"?: number | bigint | null;
  "query"?: Record<string, HttpValue> | null;
}

export interface GetQuoteResponse {
  [field: string]: unknown;
  "max_credits": number | bigint;
  "price_version": number | bigint;
}

export interface GetRealizedVolatilityRequest {
  "coin": string;
  "interval"?: GetRealizedVolatilityRequestInterval | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetRealizedVolatilityResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "vol_pct"?: string | null;
  "sum_sq_log_ret"?: string | null;
  "n_returns"?: number | bigint | null;
}

export interface GetRealizedVolatilityResponse {
  [field: string]: unknown;
  "data": Array<GetRealizedVolatilityResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetReferralRequest {
  "code"?: string | null;
  "referrer"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetReferralResponseDataItem {
  [field: string]: unknown;
  "referrer"?: string | null;
  "code"?: string | null;
  "referred_users"?: number | bigint | null;
  "snapshot_time"?: number | bigint | null;
}

export interface GetReferralResponse {
  [field: string]: unknown;
  "data": Array<GetReferralResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetReturnsMatrixRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetReturnsMatrixResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "px_now"?: string | null;
  "px_1h"?: string | null;
  "px_4h"?: string | null;
  "px_1d"?: string | null;
  "px_7d"?: string | null;
  "px_30d"?: string | null;
}

export interface GetReturnsMatrixResponse {
  [field: string]: unknown;
  "data": Array<GetReturnsMatrixResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetRevenueRequest {
  "market"?: string | null;
  "interval"?: GetRevenueRequestInterval | null;
  "days"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetRevenueResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "fees_usd"?: string | null;
  "volume_usd"?: string | null;
  "trades"?: number | bigint | null;
}

export interface GetRevenueResponse {
  [field: string]: unknown;
  "data": Array<GetRevenueResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetSentimentRequest {
  "coin": string;
  "interval"?: GetSentimentRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetSentimentResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "close"?: string | null;
  "ret"?: string | null;
  "cvd"?: string | null;
  "volume"?: string | null;
  "rv"?: string | null;
  "funding"?: string | null;
  "funding_start"?: string | null;
  "funding_end"?: string | null;
  "funding_base"?: string | null;
  "oi_usd"?: string | null;
  "oi_change_pct"?: string | null;
  "rv_change"?: string | null;
  "liq_long"?: string | null;
  "liq_short"?: string | null;
  "liq_long_next"?: string | null;
  "liq_short_next"?: string | null;
  "z_ret"?: string | null;
  "z_cvd"?: string | null;
  "z_funding"?: string | null;
  "z_oi"?: string | null;
  "z_vol"?: string | null;
  "z_funding_base"?: string | null;
}

export interface GetSentimentResponse {
  [field: string]: unknown;
  "data": Array<GetSentimentResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetSentimentEventsRequest {
  "coin": string;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetSentimentEventsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "close"?: string | null;
  "ret"?: string | null;
  "cvd"?: string | null;
  "volume"?: string | null;
  "rv"?: string | null;
  "funding"?: string | null;
  "funding_start"?: string | null;
  "funding_end"?: string | null;
  "funding_base"?: string | null;
  "oi_usd"?: string | null;
  "oi_change_pct"?: string | null;
  "rv_change"?: string | null;
  "liq_long"?: string | null;
  "liq_short"?: string | null;
  "liq_long_next"?: string | null;
  "liq_short_next"?: string | null;
  "z_ret"?: string | null;
  "z_cvd"?: string | null;
  "z_funding"?: string | null;
  "z_oi"?: string | null;
  "z_vol"?: string | null;
  "z_funding_base"?: string | null;
}

export interface GetSentimentEventsResponse {
  [field: string]: unknown;
  "data": Array<GetSentimentEventsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetSentimentScannerRequest {
  "interval"?: GetSentimentScannerRequestInterval | null;
  "view"?: GetSentimentScannerRequestView | null;
  "asset_class"?: string | null;
  "coins"?: string | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetSentimentScannerResponseDataItem {
  [field: string]: unknown;
  "market"?: string | null;
  "t"?: number | bigint | null;
  "z_ret"?: string | null;
  "z_cvd"?: string | null;
  "z_funding"?: string | null;
  "z_oi"?: string | null;
  "z_vol"?: string | null;
  "z_funding_base"?: string | null;
  "funding_change"?: string | null;
  "funding_end"?: string | null;
  "cvd"?: string | null;
  "volume"?: string | null;
  "rv"?: string | null;
  "px_change"?: string | null;
  "oi_change"?: string | null;
  "last_return"?: string | null;
}

export interface GetSentimentScannerResponse {
  [field: string]: unknown;
  "data": Array<GetSentimentScannerResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetSessionReturnsRequest {
  "market"?: string | null;
  "days"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetSessionReturnsResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "session"?: string | null;
  "return_pct"?: string | null;
  "hours"?: number | bigint | null;
}

export interface GetSessionReturnsResponse {
  [field: string]: unknown;
  "data": Array<GetSessionReturnsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetSortedLeaderboardRequest {
  "window"?: GetSortedLeaderboardRequestWindow | null;
  "sort"?: GetSortedLeaderboardRequestSort | null;
  "category"?: GetSortedLeaderboardRequestCategory | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetSortedLeaderboardResponseDataItem {
  [field: string]: unknown;
  "rank"?: number | bigint | null;
  "address"?: string | null;
  "net_pnl"?: string | null;
  "gross_pnl"?: string | null;
  "funding_pnl"?: string | null;
  "fees"?: string | null;
  "volume"?: string | null;
  "trades"?: number | bigint | null;
  "winning_trades"?: number | bigint | null;
  "losing_trades"?: number | bigint | null;
  "win_rate"?: string | null;
  "last_trade_at"?: number | bigint | null;
  "tag"?: string | null;
  "refreshed_at"?: number | bigint | null;
}

export interface GetSortedLeaderboardResponse {
  [field: string]: unknown;
  "data": Array<GetSortedLeaderboardResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetSpotTwapLevelsRequest {
  "token": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetSpotTwapLevelsResponseDataItem {
  [field: string]: unknown;
  "px"?: string | null;
  "kind"?: string | null;
  "side"?: string | null;
  "when"?: string | null;
  "pair"?: string | null;
  "twaps"?: number | bigint | null;
  "remaining_sz"?: string | null;
  "remaining_notional"?: string | null;
  "flow_sz_per_min"?: string | null;
}

export interface GetSpotTwapLevelsResponse {
  [field: string]: unknown;
  "data": Array<GetSpotTwapLevelsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetStakeHistoryRequest {
  "interval"?: GetStakeHistoryRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetStakeHistoryResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "total_stake"?: string | null;
  "active_stake"?: string | null;
  "validators"?: number | bigint | null;
  "snapshot_time"?: number | bigint | null;
}

export interface GetStakeHistoryResponse {
  [field: string]: unknown;
  "data": Array<GetStakeHistoryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetStakerBucketsRequest {
  "time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetStakerBucketsResponseDataItem {
  [field: string]: unknown;
  "bucket"?: string | null;
  "wallets"?: number | bigint | null;
  "stake_total"?: string | null;
}

export interface GetStakerBucketsResponse {
  [field: string]: unknown;
  "data": Array<GetStakerBucketsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetStakingFlowsRequest {
  "interval"?: GetStakingFlowsRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetStakingFlowsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "staked"?: string | null;
  "unstaked"?: string | null;
  "stake_transfers"?: number | bigint | null;
  "unstake_transfers"?: number | bigint | null;
  "stakers"?: number | bigint | null;
  "unstakers"?: number | bigint | null;
}

export interface GetStakingFlowsResponse {
  [field: string]: unknown;
  "data": Array<GetStakingFlowsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetStatusResponseDatasetsItem {
  [field: string]: unknown;
  "id"?: string | null;
  "complete_to"?: number | bigint | null;
  "lag_ms"?: number | bigint | null;
  "health"?: string | null;
}

export interface GetStatusResponse {
  [field: string]: unknown;
  "as_of": number | bigint;
  "datasets": Array<GetStatusResponseDatasetsItem>;
}

export interface GetStopHeatmapRequest {
  "coin": string;
  "interval"?: GetStopHeatmapRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetStopHeatmapResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "band_min"?: string | null;
  "band_max"?: string | null;
  "buy_size"?: string | null;
  "sell_size"?: string | null;
  "orders"?: number | bigint | null;
  "mark_px"?: string | null;
  "band_size"?: string | null;
}

export interface GetStopHeatmapResponse {
  [field: string]: unknown;
  "data": Array<GetStopHeatmapResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetStopLevelTotalsRequest {
  "coin": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetStopLevelTotalsResponseDataItem {
  [field: string]: unknown;
  "buy_size"?: string | null;
  "buy_orders"?: number | bigint | null;
  "sell_size"?: string | null;
  "sell_orders"?: number | bigint | null;
}

export interface GetStopLevelTotalsResponse {
  [field: string]: unknown;
  "data": Array<GetStopLevelTotalsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetStopLevelsRequest {
  "coin": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetStopLevelsResponseDataItem {
  [field: string]: unknown;
  "side"?: string | null;
  "px"?: string | null;
  "notional"?: string | null;
  "size"?: string | null;
  "orders"?: number | bigint | null;
}

export interface GetStopLevelsResponse {
  [field: string]: unknown;
  "data": Array<GetStopLevelsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetStopOrdersAtRequest {
  "coin": string;
  "time": number | bigint;
  "min_px"?: string | null;
  "max_px"?: string | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetStopOrdersAtResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "px"?: string | null;
  "size"?: string | null;
  "notional"?: string | null;
  "snapshot_time"?: number | bigint | null;
}

export interface GetStopOrdersAtResponse {
  [field: string]: unknown;
  "data": Array<GetStopOrdersAtResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetStopSummaryRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetStopSummaryResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "snapshot_time"?: number | bigint | null;
  "mark_px"?: string | null;
  "mark_time"?: number | bigint | null;
  "stop_notional"?: string | null;
  "tp_notional"?: string | null;
  "stop_notional_band"?: string | null;
  "tp_notional_band"?: string | null;
  "long_sl_band"?: string | null;
  "short_sl_band"?: string | null;
  "long_tp_band"?: string | null;
  "short_tp_band"?: string | null;
  "total_notional"?: string | null;
  "stop_levels"?: number | bigint | null;
  "tp_levels"?: number | bigint | null;
  "largest_level"?: string | null;
  "largest_level_px"?: string | null;
}

export interface GetStopSummaryResponse {
  [field: string]: unknown;
  "data": Array<GetStopSummaryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetTokenHolderBandsRequest {
  "token": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetTokenHolderBandsResponseDataItem {
  [field: string]: unknown;
  "period"?: string | null;
  "snapshot_time"?: number | bigint | null;
  "band"?: string | null;
  "wallets"?: number | bigint | null;
  "held"?: string | null;
}

export interface GetTokenHolderBandsResponse {
  [field: string]: unknown;
  "data": Array<GetTokenHolderBandsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetTokenSupplyRequest {
  "token_id": number | bigint;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetTokenSupplyResponseDataItem {
  [field: string]: unknown;
  "token_id"?: number | bigint | null;
  "name"?: string | null;
  "full_name"?: string | null;
  "sz_decimals"?: number | bigint | null;
  "wei_decimals"?: number | bigint | null;
  "evm_contract"?: string | null;
  "evm_extra_wei_decimals"?: number | bigint | null;
  "deployer"?: string | null;
  "deploy_time"?: number | bigint | null;
  "deploy_gas_wei"?: string | null;
  "deploy_gas_token"?: number | bigint | null;
  "deployer_trading_fee_share"?: string | null;
  "max_supply_wei"?: string | null;
  "balances_total_wei"?: string | null;
  "non_circulating_balance_wei"?: string | null;
  "non_circulating_users"?: number | bigint | null;
  "nc_spot_wei"?: string | null;
  "staking_delegated_wei"?: string | null;
  "snapshot_time"?: number | bigint | null;
}

export interface GetTokenSupplyResponse {
  [field: string]: unknown;
  "data": Array<GetTokenSupplyResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetTopLiquidationLevelsRequest {
  "coin": string;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetTopLiquidationLevelsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "side"?: string | null;
  "liquidation_px"?: string | null;
  "size"?: string | null;
}

export interface GetTopLiquidationLevelsResponse {
  [field: string]: unknown;
  "data": Array<GetTopLiquidationLevelsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetTopStopLevelsRequest {
  "coin": string;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetTopStopLevelsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "side"?: string | null;
  "trigger_px"?: string | null;
  "size"?: string | null;
}

export interface GetTopStopLevelsResponse {
  [field: string]: unknown;
  "data": Array<GetTopStopLevelsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetTopTraderConsensusRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetTopTraderConsensusResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "long_traders"?: number | bigint | null;
  "long_notional"?: string | null;
  "short_traders"?: number | bigint | null;
  "short_notional"?: string | null;
  "traders"?: number | bigint | null;
  "snapshot_time"?: number | bigint | null;
}

export interface GetTopTraderConsensusResponse {
  [field: string]: unknown;
  "data": Array<GetTopTraderConsensusResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetTopTraderOpensRequest {
  "minutes"?: GetTopTraderOpensRequestMinutes | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetTopTraderOpensResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "long_traders"?: number | bigint | null;
  "short_traders"?: number | bigint | null;
  "traders"?: number | bigint | null;
  "notional"?: string | null;
}

export interface GetTopTraderOpensResponse {
  [field: string]: unknown;
  "data": Array<GetTopTraderOpensResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetTradeBarsRequest {
  "coin": string;
  "interval"?: GetTradeBarsRequestInterval | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetTradeBarsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "open"?: string | null;
  "high"?: string | null;
  "low"?: string | null;
  "close"?: string | null;
  "buy_sz"?: string | null;
  "sell_sz"?: string | null;
  "buy_ntl"?: string | null;
  "sell_ntl"?: string | null;
  "buy_n"?: number | bigint | null;
  "sell_n"?: number | bigint | null;
  "liq_buy_sz"?: string | null;
  "liq_sell_sz"?: string | null;
  "liq_buy_ntl"?: string | null;
  "liq_sell_ntl"?: string | null;
  "liq_n"?: number | bigint | null;
  "liq_known"?: boolean | null;
  "final"?: boolean | null;
}

export interface GetTradeBarsResponse {
  [field: string]: unknown;
  "data": Array<GetTradeBarsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetTraderDistributionRequest {
  "view"?: GetTraderDistributionRequestView | null;
  "period"?: GetTraderDistributionRequestPeriod | null;
  "start"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetTraderDistributionResponseDataItem {
  [field: string]: unknown;
  "tier"?: string | null;
  "min_usd"?: string | null;
  "max_usd"?: string | null;
  "from_tier"?: string | null;
  "wallets"?: number | bigint | null;
  "total"?: string | null;
  "period_start"?: number | bigint | null;
  "period_end"?: number | bigint | null;
}

export interface GetTraderDistributionResponse {
  [field: string]: unknown;
  "data": Array<GetTraderDistributionResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetTraderGroupRequest {
  "group": GetTraderGroupRequestGroup;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetTraderGroupResponseDataItem {
  [field: string]: unknown;
  "rank"?: number | bigint | null;
  "address"?: string | null;
  "label"?: string | null;
  "synced_at"?: number | bigint | null;
}

export interface GetTraderGroupResponse {
  [field: string]: unknown;
  "data": Array<GetTraderGroupResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetTraderProfitabilityRequest {
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetTraderProfitabilityResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "cohort"?: string | null;
  "min_usd"?: string | null;
  "max_usd"?: string | null;
  "traders"?: number | bigint | null;
  "profitable"?: number | bigint | null;
  "unprofitable"?: number | bigint | null;
  "pnl_sum"?: string | null;
  "median_roi"?: string | null;
  "complete"?: boolean | null;
}

export interface GetTraderProfitabilityResponse {
  [field: string]: unknown;
  "data": Array<GetTraderProfitabilityResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetTwapPressureRequest {
  "market"?: string | null;
  "minutes"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetTwapPressureResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "buy_notional"?: string | null;
  "sell_notional"?: string | null;
  "net_notional"?: string | null;
  "twaps"?: number | bigint | null;
  "fills"?: number | bigint | null;
}

export interface GetTwapPressureResponse {
  [field: string]: unknown;
  "data": Array<GetTwapPressureResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetUsdcReserveRequest {
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetUsdcReserveResponseDataItem {
  [field: string]: unknown;
  "kind"?: string | null;
  "day"?: number | bigint | null;
  "balance"?: string | null;
  "snapshot_time"?: number | bigint | null;
  "read_time"?: number | bigint | null;
}

export interface GetUsdcReserveResponse {
  [field: string]: unknown;
  "data": Array<GetUsdcReserveResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetUsdcReserveIntervalsRequest {
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "assumed_rate_pct"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetUsdcReserveIntervalsResponseDataItem {
  [field: string]: unknown;
  "index"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "settlement_due"?: number | bigint | null;
  "readings"?: number | bigint | null;
  "readings_on_time"?: number | bigint | null;
  "accrued_at_assumed_rate"?: string | null;
  "settled"?: string | null;
  "settled_at"?: number | bigint | null;
  "implied_rate_pct"?: string | null;
}

export interface GetUsdcReserveIntervalsResponse {
  [field: string]: unknown;
  "data": Array<GetUsdcReserveIntervalsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetUsdcReserveSettlementsRequest {
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetUsdcReserveSettlementsResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "amount"?: string | null;
  "hash"?: string | null;
}

export interface GetUsdcReserveSettlementsResponse {
  [field: string]: unknown;
  "data": Array<GetUsdcReserveSettlementsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetUserGrowthRequest {
  "interval"?: GetUserGrowthRequestInterval | null;
  "days"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetUserGrowthResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "new_wallets"?: number | bigint | null;
  "total_wallets"?: number | bigint | null;
}

export interface GetUserGrowthResponse {
  [field: string]: unknown;
  "data": Array<GetUserGrowthResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetValidatorRegistryRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetValidatorRegistryResponseDataItem {
  [field: string]: unknown;
  "name"?: string | null;
  "description"?: string | null;
  "commission_bps"?: number | bigint | null;
  "delegations_disabled"?: boolean | null;
  "delegations_blocked"?: boolean | null;
  "total_delegated"?: string | null;
  "register_time"?: number | bigint | null;
  "last_manual_unjail_time"?: number | bigint | null;
  "last_signer_change_time"?: number | bigint | null;
  "snapshot_time"?: number | bigint | null;
}

export interface GetValidatorRegistryResponse {
  [field: string]: unknown;
  "data": Array<GetValidatorRegistryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetValidatorRewardsRequest {
  "interval"?: GetValidatorRewardsRequestInterval | null;
  "validator"?: string | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetValidatorRewardsResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "validator"?: string | null;
  "name"?: string | null;
  "reward"?: string | null;
  "events"?: number | bigint | null;
}

export interface GetValidatorRewardsResponse {
  [field: string]: unknown;
  "data": Array<GetValidatorRewardsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetValidatorsRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetValidatorsResponseDataItem {
  [field: string]: unknown;
  "name"?: string | null;
  "stake"?: string | null;
  "commission"?: string | null;
  "is_jailed"?: boolean | null;
  "is_active"?: boolean | null;
  "recent_blocks"?: number | bigint | null;
  "uptime_day"?: string | null;
  "uptime_week"?: string | null;
  "uptime_month"?: string | null;
  "apr_day"?: string | null;
  "apr_week"?: string | null;
  "apr_month"?: string | null;
  "snapshot_time"?: number | bigint | null;
  "unjailable_after"?: number | bigint | null;
  "samples_day"?: number | bigint | null;
  "samples_week"?: number | bigint | null;
  "samples_month"?: number | bigint | null;
  "source"?: string | null;
}

export interface GetValidatorsResponse {
  [field: string]: unknown;
  "data": Array<GetValidatorsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetVaultHistoryRequest {
  "vault": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetVaultHistoryResponseDataItem {
  [field: string]: unknown;
  "snapshot_time"?: number | bigint | null;
  "followers"?: number | bigint | null;
  "is_closed"?: boolean | null;
  "allow_deposits"?: boolean | null;
  "leader_commission"?: string | null;
}

export interface GetVaultHistoryResponse {
  [field: string]: unknown;
  "data": Array<GetVaultHistoryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletBuildersRequest {
  "address": string;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletBuildersResponseDataItem {
  [field: string]: unknown;
  "builder"?: string | null;
  "fee_token"?: string | null;
  "fills"?: number | bigint | null;
  "orders"?: number | bigint | null;
  "notional"?: string | null;
  "fees"?: string | null;
  "builder_fees"?: string | null;
  "fee_unreported_fills"?: number | bigint | null;
  "first_fill"?: number | bigint | null;
  "last_fill"?: number | bigint | null;
}

export interface GetWalletBuildersResponse {
  [field: string]: unknown;
  "data": Array<GetWalletBuildersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletCompletedTradesRequest {
  "address": string;
  "coin"?: string | null;
  "side"?: GetWalletCompletedTradesRequestSide | null;
  "result"?: GetWalletCompletedTradesRequestResult | null;
  "min_notional"?: GetWalletCompletedTradesRequestMinNotional | null;
  "sort"?: GetWalletCompletedTradesRequestSort | null;
  "order"?: GetWalletCompletedTradesRequestOrder | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletCompletedTradesResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "side"?: string | null;
  "open_time"?: number | bigint | null;
  "close_time"?: number | bigint | null;
  "duration_ms"?: number | bigint | null;
  "entry_px"?: string | null;
  "exit_px"?: string | null;
  "size"?: string | null;
  "max_size"?: string | null;
  "notional"?: string | null;
  "volume"?: string | null;
  "gross_pnl"?: string | null;
  "funding_pnl"?: string | null;
  "fees"?: string | null;
  "net_pnl"?: string | null;
  "fills"?: number | bigint | null;
  "maker_fills"?: number | bigint | null;
  "leverage"?: number | bigint | null;
  "is_cross"?: boolean | null;
  "builder"?: string | null;
  "tx_index"?: number | bigint | null;
}

export interface GetWalletCompletedTradesResponse {
  [field: string]: unknown;
  "data": Array<GetWalletCompletedTradesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletCopyActivityRequest {
  "address": string;
  "from"?: string | null;
  "to"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletCopyActivityResponseDataItem {
  [field: string]: unknown;
  "day"?: string | null;
  "fills"?: number | bigint | null;
  "orders_manual"?: number | bigint | null;
  "notional"?: string | null;
  "maker_notional"?: string | null;
  "fees_paid"?: string | null;
  "delay_cost"?: string | null;
  "priced_notional"?: string | null;
  "trips"?: number | bigint | null;
  "trip_notional"?: string | null;
  "trip_notional_under_1h"?: string | null;
  "computed_at"?: number | bigint | null;
}

export interface GetWalletCopyActivityResponse {
  [field: string]: unknown;
  "data": Array<GetWalletCopyActivityResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletCopyScoreRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletCopyScoreResponseDataItem {
  [field: string]: unknown;
  "as_of"?: string | null;
  "copy_score"?: string | null;
  "copy_score_raw"?: string | null;
  "quality"?: string | null;
  "gate"?: number | bigint | null;
  "consistency"?: string | null;
  "replicability"?: string | null;
  "activity"?: string | null;
  "skill"?: string | null;
  "drawdown_factor"?: string | null;
  "volatility_factor"?: string | null;
  "pnl_factor"?: string | null;
  "sortino_90d"?: string | null;
  "pnl_90d"?: string | null;
  "lifetime_pnl"?: string | null;
  "fills_90d"?: number | bigint | null;
  "orders_per_day"?: string | null;
  "maker_share"?: string | null;
  "short_hold_share"?: string | null;
  "algo"?: boolean | null;
  "delay_cost_90d"?: string | null;
  "fee_drag_90d"?: string | null;
  "round_trips_90d"?: number | bigint | null;
  "fill_days_90d"?: number | bigint | null;
  "weeks_with_pnl"?: number | bigint | null;
  "winning_week_share"?: string | null;
  "active_days_14d"?: number | bigint | null;
  "equity_p50"?: string | null;
  "max_drawdown_90d"?: string | null;
  "volatility_90d"?: string | null;
  "twr_drawdown_90d"?: string | null;
  "underwater"?: string | null;
  "sortino_weighted"?: string | null;
  "evidence_days"?: string | null;
  "record_days"?: number | bigint | null;
  "computed_at"?: string | null;
}

export interface GetWalletCopyScoreResponse {
  [field: string]: unknown;
  "data": Array<GetWalletCopyScoreResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletHighlightsRequest {
  "address": string;
  "coin"?: string | null;
  "side"?: GetWalletHighlightsRequestSide | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletHighlightsResponseDataItem {
  [field: string]: unknown;
  "kind"?: string | null;
  "rank"?: number | bigint | null;
  "coin"?: string | null;
  "side"?: string | null;
  "open_time"?: number | bigint | null;
  "close_time"?: number | bigint | null;
  "duration_s"?: string | null;
  "size"?: string | null;
  "entry_px"?: string | null;
  "exit_px"?: string | null;
  "volume"?: string | null;
  "gross_pnl"?: string | null;
  "funding_pnl"?: string | null;
  "fees"?: string | null;
  "net_pnl"?: string | null;
}

export interface GetWalletHighlightsResponse {
  [field: string]: unknown;
  "data": Array<GetWalletHighlightsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletLeaderboardRequest {
  "address": string;
  "window"?: GetWalletLeaderboardRequestWindow | null;
  "category"?: GetWalletLeaderboardRequestCategory | null;
  "sort"?: GetWalletLeaderboardRequestSort | null;
  "coin"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletLeaderboardResponseDataItem {
  [field: string]: unknown;
  "rank"?: number | bigint | null;
  "net_pnl"?: string | null;
  "realized_pnl"?: string | null;
  "unrealized_pnl"?: string | null;
  "capital"?: string | null;
  "roi"?: string | null;
  "realized_roi"?: string | null;
  "unrealized_roi"?: string | null;
  "display_name"?: string | null;
  "tag"?: string | null;
  "gross_pnl"?: string | null;
  "funding_pnl"?: string | null;
  "total_fees"?: string | null;
  "volume"?: string | null;
  "trades"?: number | bigint | null;
  "winning_trades"?: number | bigint | null;
  "losing_trades"?: number | bigint | null;
  "account_value"?: string | null;
  "last_trade_at"?: number | bigint | null;
  "label"?: string | null;
  "avatar"?: string | null;
  "twitter"?: string | null;
  "verified"?: boolean | null;
  "copy_score"?: string | null;
  "mark_quality"?: number | bigint | null;
  "window_start"?: number | bigint | null;
  "as_of"?: number | bigint | null;
}

export interface GetWalletLeaderboardResponse {
  [field: string]: unknown;
  "data": Array<GetWalletLeaderboardResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletMarketsTradedRequest {
  "address": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletMarketsTradedResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "volume"?: string | null;
  "gross_pnl"?: string | null;
  "net_pnl"?: string | null;
  "fees"?: string | null;
  "trades"?: number | bigint | null;
  "wins"?: number | bigint | null;
  "losses"?: number | bigint | null;
  "win_rate_pct"?: string | null;
  "last_trade_time"?: number | bigint | null;
}

export interface GetWalletMarketsTradedResponse {
  [field: string]: unknown;
  "data": Array<GetWalletMarketsTradedResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletOrderFillsRequest {
  "address": string;
  "coin"?: string | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletOrderFillsResponseDataItem {
  [field: string]: unknown;
  "oid"?: string | null;
  "coin"?: string | null;
  "time"?: number | bigint | null;
  "first_time"?: number | bigint | null;
  "start_position"?: string | null;
  "direction"?: string | null;
  "side"?: string | null;
  "size"?: string | null;
  "avg_px"?: string | null;
  "notional"?: string | null;
  "closed_pnl"?: string | null;
  "fees"?: string | null;
  "fill_count"?: number | bigint | null;
  "fills"?: string | null;
}

export interface GetWalletOrderFillsResponse {
  [field: string]: unknown;
  "data": Array<GetWalletOrderFillsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletOutcomesRequest {
  "address": string;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletOutcomesResponseDataItem {
  [field: string]: unknown;
  "outcome"?: string | null;
  "yes_shares"?: string | null;
  "no_shares"?: string | null;
  "yes_basis"?: string | null;
  "no_basis"?: string | null;
  "realized_pnl"?: string | null;
  "usdc_fees"?: string | null;
  "share_fees"?: string | null;
  "notional"?: string | null;
  "fills"?: number | bigint | null;
  "first_fill_time"?: number | bigint | null;
  "last_fill_time"?: number | bigint | null;
  "settled"?: boolean | null;
  "settle_fraction"?: string | null;
  "settlement_pnl"?: string | null;
  "net_pnl"?: string | null;
  "result"?: string | null;
  "balance_status"?: string | null;
  "balance_checked_at"?: number | bigint | null;
}

export interface GetWalletOutcomesResponse {
  [field: string]: unknown;
  "data": Array<GetWalletOutcomesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletPnlCalendarRequest {
  "address": string;
  "from"?: string | null;
  "to"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletPnlCalendarResponseDataItem {
  [field: string]: unknown;
  "date"?: string | null;
  "pnl"?: string | null;
  "pnl_pct"?: string | null;
}

export interface GetWalletPnlCalendarResponse {
  [field: string]: unknown;
  "data": Array<GetWalletPnlCalendarResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletProfilesRequest {
  "addresses": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletProfilesResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "display_name"?: string | null;
  "avatar"?: string | null;
  "twitter"?: string | null;
  "tag"?: string | null;
  "verified"?: boolean | null;
  "label"?: string | null;
  "updated_at"?: number | bigint | null;
}

export interface GetWalletProfilesResponse {
  [field: string]: unknown;
  "data": Array<GetWalletProfilesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletReviewRequest {
  "address": string;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletReviewResponseDataItem {
  [field: string]: unknown;
  "section"?: string | null;
  "key"?: string | null;
  "trades"?: number | bigint | null;
  "wins"?: number | bigint | null;
  "losses"?: number | bigint | null;
  "incomplete"?: number | bigint | null;
  "net_pnl"?: string | null;
  "gross_pnl"?: string | null;
  "funding_pnl"?: string | null;
  "fees"?: string | null;
  "volume"?: string | null;
  "best_trade"?: string | null;
  "worst_trade"?: string | null;
}

export interface GetWalletReviewResponse {
  [field: string]: unknown;
  "data": Array<GetWalletReviewResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletRiskMetricsRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletRiskMetricsResponseDataItem {
  [field: string]: unknown;
  "as_of"?: string | null;
  "sharpe_30d"?: string | null;
  "sortino_30d"?: string | null;
  "sharpe_90d"?: string | null;
  "sortino_90d"?: string | null;
  "sharpe_180d"?: string | null;
  "sortino_180d"?: string | null;
  "sharpe_365d"?: string | null;
  "sortino_365d"?: string | null;
  "max_drawdown_7d"?: string | null;
  "max_drawdown_30d"?: string | null;
  "max_drawdown_90d"?: string | null;
  "max_drawdown_180d"?: string | null;
  "max_drawdown_365d"?: string | null;
}

export interface GetWalletRiskMetricsResponse {
  [field: string]: unknown;
  "data": Array<GetWalletRiskMetricsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletSharpeRequest {
  "address": string;
  "from"?: string | null;
  "to"?: string | null;
  "min_active_days"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletSharpeResponseDataItem {
  [field: string]: unknown;
  "days"?: number | bigint | null;
  "active_days"?: number | bigint | null;
  "first_date"?: string | null;
  "last_date"?: string | null;
  "pnl"?: string | null;
  "sharpe"?: string | null;
  "sortino"?: string | null;
}

export interface GetWalletSharpeResponse {
  [field: string]: unknown;
  "data": Array<GetWalletSharpeResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletStakedHypeRequest {
  "address": string;
  "time": number | bigint;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletStakedHypeResponseDataItem {
  [field: string]: unknown;
  "hype"?: string | null;
}

export interface GetWalletStakedHypeResponse {
  [field: string]: unknown;
  "data": Array<GetWalletStakedHypeResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletStatsRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletStatsResponseDataItem {
  [field: string]: unknown;
  "trades"?: number | bigint | null;
  "first_trade_time"?: number | bigint | null;
  "last_trade_time"?: number | bigint | null;
  "pnl"?: string | null;
  "long_pnl"?: string | null;
  "short_pnl"?: string | null;
  "fees"?: string | null;
  "net_pnl"?: string | null;
  "avg_hold_minutes"?: string | null;
  "median_hold_minutes"?: string | null;
  "q1_hold_minutes"?: string | null;
  "q3_hold_minutes"?: string | null;
  "longest_win_streak"?: number | bigint | null;
}

export interface GetWalletStatsResponse {
  [field: string]: unknown;
  "data": Array<GetWalletStatsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletStatsByMarketRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletStatsByMarketResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "pnl"?: string | null;
  "fees"?: string | null;
  "net_pnl"?: string | null;
  "trades"?: number | bigint | null;
  "wins"?: number | bigint | null;
  "losses"?: number | bigint | null;
  "volume"?: string | null;
  "winning_pnl"?: string | null;
  "losing_pnl"?: string | null;
}

export interface GetWalletStatsByMarketResponse {
  [field: string]: unknown;
  "data": Array<GetWalletStatsByMarketResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletTradeCountsRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletTradeCountsResponseDataItem {
  [field: string]: unknown;
  "trades"?: number | bigint | null;
  "long_trades"?: number | bigint | null;
  "short_trades"?: number | bigint | null;
  "wins"?: number | bigint | null;
  "losses"?: number | bigint | null;
  "trades_7d"?: number | bigint | null;
  "trades_30d"?: number | bigint | null;
  "trades_90d"?: number | bigint | null;
  "trades_180d"?: number | bigint | null;
  "trades_365d"?: number | bigint | null;
  "wins_7d"?: number | bigint | null;
  "wins_30d"?: number | bigint | null;
  "wins_90d"?: number | bigint | null;
  "wins_180d"?: number | bigint | null;
  "wins_365d"?: number | bigint | null;
  "last_trade_time"?: number | bigint | null;
}

export interface GetWalletTradeCountsResponse {
  [field: string]: unknown;
  "data": Array<GetWalletTradeCountsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletUpnlRankRequest {
  "address": string;
  "category"?: GetWalletUpnlRankRequestCategory | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletUpnlRankResponseDataItem {
  [field: string]: unknown;
  "unrealized_pnl"?: string | null;
  "rank"?: number | bigint | null;
  "traders"?: number | bigint | null;
}

export interface GetWalletUpnlRankResponse {
  [field: string]: unknown;
  "data": Array<GetWalletUpnlRankResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletWindowStatsRequest {
  "address": string;
  "coin"?: string | null;
  "window"?: GetWalletWindowStatsRequestWindow | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletWindowStatsResponseDataItem {
  [field: string]: unknown;
  "net_pnl"?: string | null;
  "gross_pnl"?: string | null;
  "funding_pnl"?: string | null;
  "volume"?: string | null;
  "trades"?: number | bigint | null;
  "wins"?: number | bigint | null;
  "losses"?: number | bigint | null;
  "last_trade_at"?: string | null;
}

export interface GetWalletWindowStatsResponse {
  [field: string]: unknown;
  "data": Array<GetWalletWindowStatsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWalletYearSummaryRequest {
  "address": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWalletYearSummaryResponseDataItem {
  [field: string]: unknown;
  "year"?: number | bigint | null;
  "trader_class"?: string | null;
  "reason"?: string | null;
  "pnl"?: string | null;
  "volume"?: string | null;
  "trades"?: number | bigint | null;
  "pnl_percentile"?: string | null;
  "volume_percentile"?: string | null;
  "trades_percentile"?: string | null;
  "win_rate"?: string | null;
  "sharpe_30d"?: string | null;
  "taker_ratio"?: string | null;
  "trades_per_day"?: string | null;
  "median_hold_s"?: string | null;
  "computed_at"?: string | null;
}

export interface GetWalletYearSummaryResponse {
  [field: string]: unknown;
  "data": Array<GetWalletYearSummaryResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetWindowPricesRequest {
  "coins"?: string | null;
  "asset_class"?: string | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface GetWindowPricesResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "open"?: string | null;
  "high"?: string | null;
  "low"?: string | null;
  "close"?: string | null;
}

export interface GetWindowPricesResponse {
  [field: string]: unknown;
  "data": Array<GetWindowPricesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface LiquidationMinutesRequest {
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface LiquidationMinutesResponseDataItem {
  [field: string]: unknown;
  "t"?: number | bigint | null;
  "long_notional"?: string | null;
  "short_notional"?: string | null;
  "long_events"?: number | bigint | null;
  "short_events"?: number | bigint | null;
  "events"?: number | bigint | null;
}

export interface LiquidationMinutesResponse {
  [field: string]: unknown;
  "data": Array<LiquidationMinutesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface LiquidationMixRequest {
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface LiquidationMixResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "long_notional"?: string | null;
  "short_notional"?: string | null;
  "total_notional"?: string | null;
  "events"?: number | bigint | null;
}

export interface LiquidationMixResponse {
  [field: string]: unknown;
  "data": Array<LiquidationMixResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListAllTwapsRequest {
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListAllTwapsResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "address"?: string | null;
  "twap_id"?: string | null;
  "side"?: string | null;
  "total_sz"?: string | null;
  "executed_sz"?: string | null;
  "remaining_sz"?: string | null;
  "executed_notional"?: string | null;
  "progress_pct"?: string | null;
  "duration_s"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "reduce_only"?: boolean | null;
  "randomize"?: boolean | null;
  "next_slice_time"?: string | null;
  "slices_done"?: number | bigint | null;
  "waiting"?: boolean | null;
  "stop_px"?: string | null;
  "trigger_px"?: string | null;
  "trigger_above"?: boolean | null;
}

export interface ListAllTwapsResponse {
  [field: string]: unknown;
  "data": Array<ListAllTwapsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListCohortMembersRequest {
  "cohort": ListCohortMembersRequestCohort;
  "coin"?: string | null;
  "sort"?: ListCohortMembersRequestSort | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListCohortMembersResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "account_value"?: string | null;
  "positions"?: number | bigint | null;
  "market"?: string | null;
  "side"?: string | null;
  "size"?: string | null;
  "entry_px"?: string | null;
  "leverage_value"?: string | null;
  "leverage_mode"?: string | null;
  "notional"?: string | null;
  "unrealized_pnl"?: string | null;
  "snapshot_time"?: number | bigint | null;
}

export interface ListCohortMembersResponse {
  [field: string]: unknown;
  "data": Array<ListCohortMembersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListCohortTopPositionsRequest {
  "direction"?: ListCohortTopPositionsRequestDirection | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListCohortTopPositionsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "market"?: string | null;
  "side"?: string | null;
  "size"?: string | null;
  "notional"?: string | null;
  "entry_px"?: string | null;
  "liquidation_px"?: string | null;
  "unrealized_pnl"?: string | null;
  "account_value"?: string | null;
  "total_pnl"?: string | null;
  "opened_notional"?: string | null;
  "last_open_time"?: number | bigint | null;
  "first_open_time"?: number | bigint | null;
  "funding_pnl"?: string | null;
  "mark_px"?: string | null;
  "unrealized_pnl_pct"?: string | null;
  "leverage"?: string | null;
  "leverage_mode"?: string | null;
  "notional_pct_of_account"?: string | null;
  "snapshot_time"?: number | bigint | null;
}

export interface ListCohortTopPositionsResponse {
  [field: string]: unknown;
  "data": Array<ListCohortTopPositionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListDeployAuctionsRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListDeployAuctionsResponseDataItem {
  [field: string]: unknown;
  "kind"?: string | null;
  "start_time"?: number | bigint | null;
  "duration_s"?: number | bigint | null;
  "start_gas"?: string | null;
  "current_gas"?: string | null;
  "end_gas"?: string | null;
}

export interface ListDeployAuctionsResponse {
  [field: string]: unknown;
  "data": Array<ListDeployAuctionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListDexMarketsRequest {
  "dex": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListDexMarketsResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "oi_cap"?: string | null;
  "funding_multiplier"?: string | null;
  "funding_interest_rate"?: string | null;
  "funding_clamp"?: string | null;
}

export interface ListDexMarketsResponse {
  [field: string]: unknown;
  "data": Array<ListDexMarketsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListDexsRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListDexsResponseDataItem {
  [field: string]: unknown;
  "name"?: string | null;
  "full_name"?: string | null;
  "deployer"?: string | null;
  "oracle_updater"?: string | null;
  "fee_recipient"?: string | null;
  "markets"?: number | bigint | null;
  "sub_deployers"?: string | null;
}

export interface ListDexsResponse {
  [field: string]: unknown;
  "data": Array<ListDexsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListEvmCoreTransfersRequest {
  "action_type": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListEvmCoreTransfersResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "user"?: string | null;
  "destination"?: string | null;
  "action_type"?: string | null;
  "nonce"?: number | bigint | null;
  "evm_tx_hash"?: string | null;
  "token"?: number | bigint | null;
  "wei"?: string | null;
  "wei_approx"?: string | null;
  "block"?: number | bigint | null;
  "event_index"?: number | bigint | null;
}

export interface ListEvmCoreTransfersResponse {
  [field: string]: unknown;
  "data": Array<ListEvmCoreTransfersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListFreshWalletPositionsRequest {
  "max_age_days"?: ListFreshWalletPositionsRequestMaxAgeDays | null;
  "min_balance"?: ListFreshWalletPositionsRequestMinBalance | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListFreshWalletPositionsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "market"?: string | null;
  "side"?: string | null;
  "size"?: string | null;
  "notional"?: string | null;
  "entry_px"?: string | null;
  "liquidation_px"?: string | null;
  "unrealized_pnl"?: string | null;
  "account_value"?: string | null;
  "funding_pnl"?: string | null;
  "snapshot_time"?: number | bigint | null;
}

export interface ListFreshWalletPositionsResponse {
  [field: string]: unknown;
  "data": Array<ListFreshWalletPositionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListGossipAuctionsRequest {
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListGossipAuctionsResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "slot"?: number | bigint | null;
  "previous_winner_ip"?: string | null;
  "end_gas"?: string | null;
  "block"?: number | bigint | null;
  "event_index"?: number | bigint | null;
}

export interface ListGossipAuctionsResponse {
  [field: string]: unknown;
  "data": Array<ListGossipAuctionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListHip3OracleUpdatesRequest {
  "coin": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListHip3OracleUpdatesResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "dex"?: string | null;
  "update_class"?: string | null;
  "mark_px_input"?: string | null;
  "spot_px_input"?: string | null;
  "external_perp_px_input"?: string | null;
  "oracle_px"?: string | null;
  "oracle_daily_px"?: string | null;
  "mark_px"?: string | null;
  "external_perp_px"?: string | null;
  "block"?: number | bigint | null;
  "event_index"?: number | bigint | null;
}

export interface ListHip3OracleUpdatesResponse {
  [field: string]: unknown;
  "data": Array<ListHip3OracleUpdatesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListJobsResponseDataItemError {
  [field: string]: unknown;
  "code"?: string | null;
  "detail"?: string | null;
}

export interface ListJobsResponseDataItem {
  [field: string]: unknown;
  "id"?: string | null;
  "kind"?: string | null;
  "params"?: Record<string, HttpValue> | null;
  "status"?: string | null;
  "created_at"?: string | null;
  "started_at"?: string | null;
  "finished_at"?: string | null;
  "expires_at"?: string | null;
  "result_rows"?: number | bigint | null;
  "compute_units"?: number | bigint | null;
  "result_url"?: string | null;
  "error"?: ListJobsResponseDataItemError | null;
}

export interface ListJobsResponse {
  [field: string]: unknown;
  "data"?: Array<ListJobsResponseDataItem> | null;
}

export interface ListLargestOpensRequest {
  "market"?: string | null;
  "minutes"?: number | bigint | null;
  "min_notional"?: ListLargestOpensRequestMinNotional | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListLargestOpensResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "coin"?: string | null;
  "direction"?: string | null;
  "oid"?: string | null;
  "notional"?: string | null;
  "size"?: string | null;
  "avg_px"?: string | null;
  "fills"?: number | bigint | null;
  "first_fill_time"?: number | bigint | null;
  "last_fill_time"?: number | bigint | null;
}

export interface ListLargestOpensResponse {
  [field: string]: unknown;
  "data": Array<ListLargestOpensResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListLargestPositionsRequest {
  "market"?: string | null;
  "side"?: ListLargestPositionsRequestSide | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListLargestPositionsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "market"?: string | null;
  "side"?: string | null;
  "size"?: string | null;
  "notional"?: string | null;
  "entry_px"?: string | null;
  "liquidation_px"?: string | null;
  "unrealized_pnl"?: string | null;
  "account_value"?: string | null;
  "funding_pnl"?: string | null;
  "leverage"?: string | null;
  "leverage_mode"?: string | null;
  "snapshot_time"?: number | bigint | null;
}

export interface ListLargestPositionsResponse {
  [field: string]: unknown;
  "data": Array<ListLargestPositionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListLendingReservesRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListLendingReservesResponseDataItem {
  [field: string]: unknown;
  "token"?: string | null;
  "token_index"?: number | bigint | null;
  "supply_apr"?: string | null;
  "borrow_apr"?: string | null;
  "utilization"?: string | null;
  "total_supplied"?: string | null;
  "total_borrowed"?: string | null;
  "available"?: string | null;
  "oracle_px"?: string | null;
  "ltv"?: string | null;
}

export interface ListLendingReservesResponse {
  [field: string]: unknown;
  "data": Array<ListLendingReservesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListLiquidationsRequest {
  "coin"?: string | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListLiquidationsResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "coin"?: string | null;
  "tid"?: string | null;
  "side"?: string | null;
  "method"?: string | null;
  "user"?: string | null;
  "px"?: string | null;
  "sz"?: string | null;
  "notional"?: string | null;
  "closed_pnl"?: string | null;
  "mark_px"?: string | null;
  "hash"?: string | null;
  "start_position"?: string | null;
  "dir"?: string | null;
  "oid"?: string | null;
}

export interface ListLiquidationsResponse {
  [field: string]: unknown;
  "data": Array<ListLiquidationsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListMarginTablesRequest {
  "dex"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListMarginTablesResponseDataItem {
  [field: string]: unknown;
  "table_id"?: number | bigint | null;
  "description"?: string | null;
  "tier"?: number | bigint | null;
  "lower_bound"?: string | null;
  "max_leverage"?: number | bigint | null;
}

export interface ListMarginTablesResponse {
  [field: string]: unknown;
  "data": Array<ListMarginTablesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListMarketCatalogRequest {
  "coin"?: string | null;
  "category"?: ListMarketCatalogRequestCategory | null;
  "sector"?: ListMarketCatalogRequestSector | null;
  "asset_class"?: ListMarketCatalogRequestAssetClass | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListMarketCatalogResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "category"?: string | null;
  "sector"?: string | null;
  "asset_class"?: string | null;
  "display_name"?: string | null;
  "updated_at"?: number | bigint | null;
}

export interface ListMarketCatalogResponse {
  [field: string]: unknown;
  "data": Array<ListMarketCatalogResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListMarketTwapsRequest {
  "coin": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListMarketTwapsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "twap_id"?: string | null;
  "side"?: string | null;
  "total_sz"?: string | null;
  "executed_sz"?: string | null;
  "remaining_sz"?: string | null;
  "executed_notional"?: string | null;
  "progress_pct"?: string | null;
  "duration_s"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "reduce_only"?: boolean | null;
  "randomize"?: boolean | null;
  "next_slice_time"?: string | null;
  "slices_done"?: number | bigint | null;
  "waiting"?: boolean | null;
  "stop_px"?: string | null;
  "trigger_px"?: string | null;
  "trigger_above"?: boolean | null;
}

export interface ListMarketTwapsResponse {
  [field: string]: unknown;
  "data": Array<ListMarketTwapsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListMarketsRequest {
  "dex"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListMarketsResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "sz_decimals"?: number | bigint | null;
  "max_leverage"?: number | bigint | null;
  "mark_px"?: string | null;
  "oracle_px"?: string | null;
  "mid_px"?: string | null;
  "funding"?: string | null;
  "premium"?: string | null;
  "open_interest"?: string | null;
  "prev_day_px"?: string | null;
  "day_ntl_vlm"?: string | null;
  "day_base_vlm"?: string | null;
  "impact_bid_px"?: string | null;
  "impact_ask_px"?: string | null;
  "is_delisted"?: boolean | null;
  "margin_table_id"?: number | bigint | null;
  "only_isolated"?: boolean | null;
  "margin_mode"?: string | null;
  "growth_mode"?: string | null;
  "deployer_fee_scale"?: string | null;
  "last_fee_scale_change_time"?: number | bigint | null;
  "mark_px_estimated"?: boolean | null;
  "funding_unverified"?: boolean | null;
  "prev_day_px_seeded"?: boolean | null;
  "volume_inexact"?: boolean | null;
  "last_settled_funding"?: string | null;
  "last_settled_funding_time"?: number | bigint | null;
  "collateral_token"?: number | bigint | null;
  "at_oi_cap"?: boolean | null;
}

export interface ListMarketsResponse {
  [field: string]: unknown;
  "data": Array<ListMarketsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListMaxMarketOrderNotionalsRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListMaxMarketOrderNotionalsResponseDataItem {
  [field: string]: unknown;
  "max_leverage"?: number | bigint | null;
  "max_notional"?: string | null;
}

export interface ListMaxMarketOrderNotionalsResponse {
  [field: string]: unknown;
  "data": Array<ListMaxMarketOrderNotionalsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListMidsRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListMidsResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "mid"?: string | null;
}

export interface ListMidsResponse {
  [field: string]: unknown;
  "data": Array<ListMidsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListNetworkDeploysRequest {
  "action_type"?: string | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListNetworkDeploysResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "action_type"?: string | null;
  "deployer"?: string | null;
  "ok"?: boolean | null;
  "error"?: string | null;
  "asset"?: number | bigint | null;
  "action"?: string | null;
  "block"?: number | bigint | null;
  "bundle_index"?: number | bigint | null;
  "action_index"?: number | bigint | null;
  "inner"?: boolean | null;
}

export interface ListNetworkDeploysResponse {
  [field: string]: unknown;
  "data": Array<ListNetworkDeploysResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListOutcomeDefinitionsRequest {
  "kind"?: ListOutcomeDefinitionsRequestKind | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListOutcomeDefinitionsResponseDataItem {
  [field: string]: unknown;
  "outcome"?: number | bigint | null;
  "question"?: number | bigint | null;
  "name"?: string | null;
  "description"?: string | null;
  "side_names"?: string | null;
  "quote_token"?: string | null;
  "venue"?: string | null;
  "deployer_fee_scale"?: string | null;
  "fallback_outcome"?: number | bigint | null;
  "named_outcomes"?: string | null;
  "settled_named_outcomes"?: string | null;
}

export interface ListOutcomeDefinitionsResponse {
  [field: string]: unknown;
  "data": Array<ListOutcomeDefinitionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListOutcomeMarketsRequest {
  "status"?: ListOutcomeMarketsRequestStatus | null;
  "outcome_id"?: number | bigint | null;
  "question_id"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListOutcomeMarketsResponseDataItem {
  [field: string]: unknown;
  "outcome"?: number | bigint | null;
  "coin_yes"?: string | null;
  "coin_no"?: string | null;
  "question_id"?: number | bigint | null;
  "question_name"?: string | null;
  "is_fallback"?: boolean | null;
  "name"?: string | null;
  "description"?: string | null;
  "side0_name"?: string | null;
  "side1_name"?: string | null;
  "venue"?: string | null;
  "quote_token"?: string | null;
  "status"?: string | null;
  "first_seen_time"?: number | bigint | null;
  "expiry_time"?: number | bigint | null;
  "expiry_raw"?: string | null;
  "open_interest"?: string | null;
  "oi_as_of"?: number | bigint | null;
  "class"?: string | null;
  "underlying"?: string | null;
  "period"?: string | null;
  "target_price"?: string | null;
  "category"?: string | null;
  "sub_category"?: string | null;
  "deployer_fee_scale"?: string | null;
  "settle_fraction"?: string | null;
  "settle_details"?: string | null;
  "settled_at"?: number | bigint | null;
}

export interface ListOutcomeMarketsResponse {
  [field: string]: unknown;
  "data": Array<ListOutcomeMarketsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListOutcomeStatsRequest {
  "hours"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListOutcomeStatsResponseDataItem {
  [field: string]: unknown;
  "outcome"?: number | bigint | null;
  "yes_notional"?: string | null;
  "share_volume"?: string | null;
  "trades"?: number | bigint | null;
  "traders"?: number | bigint | null;
  "last_yes_px"?: string | null;
  "first_trade_at"?: number | bigint | null;
  "last_trade_at"?: number | bigint | null;
}

export interface ListOutcomeStatsResponse {
  [field: string]: unknown;
  "data": Array<ListOutcomeStatsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListOutcomeWhaleTradesRequest {
  "hours"?: number | bigint | null;
  "outcome"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListOutcomeWhaleTradesResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "coin"?: string | null;
  "outcome"?: number | bigint | null;
  "side_index"?: number | bigint | null;
  "yes_px"?: string | null;
  "yes_side"?: string | null;
  "size"?: string | null;
  "yes_notional"?: string | null;
  "address"?: string | null;
  "hash"?: string | null;
  "tid"?: string | null;
  "twap_id"?: string | null;
}

export interface ListOutcomeWhaleTradesResponse {
  [field: string]: unknown;
  "data": Array<ListOutcomeWhaleTradesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListPositionsNearLiquidationRequest {
  "market"?: string | null;
  "max_distance_pct"?: ListPositionsNearLiquidationRequestMaxDistancePct | null;
  "min_notional"?: ListPositionsNearLiquidationRequestMinNotional | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListPositionsNearLiquidationResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "market"?: string | null;
  "side"?: string | null;
  "size"?: string | null;
  "notional"?: string | null;
  "entry_px"?: string | null;
  "liquidation_px"?: string | null;
  "unrealized_pnl"?: string | null;
  "account_value"?: string | null;
  "mark_px"?: string | null;
  "mark_source"?: string | null;
  "mark_px_estimated"?: boolean | null;
  "distance_pct"?: string | null;
  "snapshot_time"?: number | bigint | null;
}

export interface ListPositionsNearLiquidationResponse {
  [field: string]: unknown;
  "data": Array<ListPositionsNearLiquidationResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListPriorityFeeTopUsersRequest {
  "days"?: number | bigint | null;
  "kind"?: ListPriorityFeeTopUsersRequestKind | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListPriorityFeeTopUsersResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "total_usd"?: string | null;
  "total_hype"?: string | null;
  "write_usd"?: string | null;
  "read_usd"?: string | null;
  "write_orders"?: number | bigint | null;
  "reads"?: number | bigint | null;
  "share_pct"?: string | null;
  "write_ioc_hype"?: string | null;
  "write_ioc_usd"?: string | null;
  "write_ioc_orders"?: number | bigint | null;
  "write_ioc_fills"?: number | bigint | null;
  "write_ioc_unpriced_hype"?: string | null;
  "write_alo_hype_estimate"?: string | null;
  "write_alo_usd"?: string | null;
  "write_alo_orders"?: number | bigint | null;
  "write_alo_resting_notional_usd"?: string | null;
  "write_unpriced_hype"?: string | null;
  "unpriced_hype"?: string | null;
  "unconverted_usd"?: string | null;
  "hype_estimated"?: boolean | null;
  "live_from"?: number | bigint | null;
  "write_fills"?: number | bigint | null;
  "read_hype"?: string | null;
  "read_unpriced_hype"?: string | null;
  "total_txs"?: number | bigint | null;
  "fee_hype"?: string | null;
  "fee_usd"?: string | null;
  "writes"?: number | bigint | null;
  "share_hype_pct"?: string | null;
  "share_hype_estimated"?: boolean | null;
  "share_hype_complete"?: boolean | null;
  "wallets"?: number | bigint | null;
  "usd_complete"?: boolean | null;
  "hype_complete"?: boolean | null;
  "write_hype"?: string | null;
}

export interface ListPriorityFeeTopUsersResponse {
  [field: string]: unknown;
  "data": Array<ListPriorityFeeTopUsersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListSpotMarketsRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListSpotMarketsResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "name"?: string | null;
  "base"?: string | null;
  "quote"?: string | null;
  "is_canonical"?: boolean | null;
  "index"?: number | bigint | null;
  "base_token_index"?: number | bigint | null;
  "quote_token_index"?: number | bigint | null;
  "mark_px"?: string | null;
  "mid_px"?: string | null;
  "prev_day_px"?: string | null;
  "day_ntl_vlm"?: string | null;
  "day_base_vlm"?: string | null;
  "circulating_supply"?: string | null;
  "total_supply"?: string | null;
  "mark_px_estimated"?: boolean | null;
  "day_stats_lagged"?: boolean | null;
  "prev_day_px_seeded"?: boolean | null;
  "volume_inexact"?: boolean | null;
  "book_pending"?: boolean | null;
  "supply_as_of"?: number | bigint | null;
  "supply_approx"?: boolean | null;
  "supply_unavailable"?: boolean | null;
}

export interface ListSpotMarketsResponse {
  [field: string]: unknown;
  "data": Array<ListSpotMarketsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListSpotTokensRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListSpotTokensResponseDataItem {
  [field: string]: unknown;
  "name"?: string | null;
  "index"?: number | bigint | null;
  "token_id"?: string | null;
  "full_name"?: string | null;
  "sz_decimals"?: number | bigint | null;
  "wei_decimals"?: number | bigint | null;
  "is_canonical"?: boolean | null;
  "evm_contract"?: string | null;
  "evm_extra_wei_decimals"?: number | bigint | null;
  "deployer_trading_fee_share"?: string | null;
  "deployer_label"?: string | null;
}

export interface ListSpotTokensResponse {
  [field: string]: unknown;
  "data": Array<ListSpotTokensResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListSpotTwapsRequest {
  "token": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListSpotTwapsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "twap_id"?: string | null;
  "pair"?: string | null;
  "pair_id"?: string | null;
  "side"?: string | null;
  "total_sz"?: string | null;
  "executed_sz"?: string | null;
  "remaining_sz"?: string | null;
  "executed_notional"?: string | null;
  "progress_pct"?: string | null;
  "duration_s"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "reduce_only"?: boolean | null;
  "randomize"?: boolean | null;
  "next_slice_time"?: string | null;
  "slices_done"?: number | bigint | null;
  "waiting"?: boolean | null;
  "stop_px"?: string | null;
  "trigger_px"?: string | null;
  "trigger_above"?: boolean | null;
}

export interface ListSpotTwapsResponse {
  [field: string]: unknown;
  "data": Array<ListSpotTwapsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListStakerChangesRequest {
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "sort"?: ListStakerChangesRequestSort | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListStakerChangesResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "stake_from"?: string | null;
  "stake_to"?: string | null;
  "change"?: string | null;
  "share_to"?: string | null;
}

export interface ListStakerChangesResponse {
  [field: string]: unknown;
  "data": Array<ListStakerChangesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListSubBuilderEarningsRequest {
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "builder"?: string | null;
  "sub_builder"?: string | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListSubBuilderEarningsResponseDataItem {
  [field: string]: unknown;
  "sub_builder"?: string | null;
  "fee_token"?: string | null;
  "builders"?: number | bigint | null;
  "orders"?: number | bigint | null;
  "fills"?: number | bigint | null;
  "notional"?: string | null;
  "earned"?: string | null;
  "fee_unreported_fills"?: number | bigint | null;
  "first_fill"?: number | bigint | null;
  "last_fill"?: number | bigint | null;
}

export interface ListSubBuilderEarningsResponse {
  [field: string]: unknown;
  "data": Array<ListSubBuilderEarningsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListTopLiquidationsRequest {
  "market"?: string | null;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListTopLiquidationsResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "coin"?: string | null;
  "side"?: string | null;
  "method"?: string | null;
  "address"?: string | null;
  "px"?: string | null;
  "size"?: string | null;
  "notional"?: string | null;
  "closed_pnl"?: string | null;
  "mark_px"?: string | null;
}

export interface ListTopLiquidationsResponse {
  [field: string]: unknown;
  "data": Array<ListTopLiquidationsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListTopStakersRequest {
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListTopStakersResponseDataItem {
  [field: string]: unknown;
  "rank"?: number | bigint | null;
  "address"?: string | null;
  "hype"?: string | null;
  "validators"?: number | bigint | null;
  "delegations"?: string | null;
}

export interface ListTopStakersResponse {
  [field: string]: unknown;
  "data": Array<ListTopStakersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListTopUnrealizedPnlRequest {
  "category"?: ListTopUnrealizedPnlRequestCategory | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListTopUnrealizedPnlResponseDataItem {
  [field: string]: unknown;
  "rank"?: number | bigint | null;
  "address"?: string | null;
  "unrealized_pnl"?: string | null;
}

export interface ListTopUnrealizedPnlResponse {
  [field: string]: unknown;
  "data": Array<ListTopUnrealizedPnlResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListTopUnrealizedPositionsRequest {
  "side"?: ListTopUnrealizedPositionsRequestSide | null;
  "market"?: string | null;
  "min_notional"?: ListTopUnrealizedPositionsRequestMinNotional | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListTopUnrealizedPositionsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "market"?: string | null;
  "side"?: string | null;
  "size"?: string | null;
  "notional"?: string | null;
  "entry_px"?: string | null;
  "liquidation_px"?: string | null;
  "unrealized_pnl"?: string | null;
  "funding_pnl"?: string | null;
  "account_value"?: string | null;
  "leverage_value"?: string | null;
  "leverage_mode"?: string | null;
  "snapshot_time"?: number | bigint | null;
}

export interface ListTopUnrealizedPositionsResponse {
  [field: string]: unknown;
  "data": Array<ListTopUnrealizedPositionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListUnrealizedPnlMoversRequest {
  "interval"?: ListUnrealizedPnlMoversRequestInterval | null;
  "direction"?: ListUnrealizedPnlMoversRequestDirection | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListUnrealizedPnlMoversResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "upnl_now"?: string | null;
  "upnl_then"?: string | null;
  "upnl_change"?: string | null;
  "notional"?: string | null;
  "open_positions"?: number | bigint | null;
  "account_value"?: string | null;
  "snapshot_time"?: number | bigint | null;
  "baseline_time"?: number | bigint | null;
}

export interface ListUnrealizedPnlMoversResponse {
  [field: string]: unknown;
  "data": Array<ListUnrealizedPnlMoversResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListValidatorDelegationsRequest {
  "validator": string;
  "limit"?: number | bigint | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListValidatorDelegationsResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "user"?: string | null;
  "kind"?: string | null;
  "amount"?: string | null;
  "hash"?: string | null;
  "block"?: number | bigint | null;
  "event_index"?: number | bigint | null;
}

export interface ListValidatorDelegationsResponse {
  [field: string]: unknown;
  "data": Array<ListValidatorDelegationsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListVaultFollowersRequest {
  "vault": string;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListVaultFollowersResponseDataItem {
  [field: string]: unknown;
  "follower"?: string | null;
  "ownership_fraction"?: string | null;
  "ownership_b"?: string | null;
  "net_transfers"?: string | null;
  "entry_time"?: number | bigint | null;
  "last_change_time"?: number | bigint | null;
  "snapshot_time"?: number | bigint | null;
}

export interface ListVaultFollowersResponse {
  [field: string]: unknown;
  "data": Array<ListVaultFollowersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListVaultsRequest {
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListVaultsResponseDataItem {
  [field: string]: unknown;
  "vault"?: string | null;
  "name"?: string | null;
  "description"?: string | null;
  "leader"?: string | null;
  "create_time"?: number | bigint | null;
  "leader_commission"?: string | null;
  "is_closed"?: boolean | null;
  "allow_deposits"?: boolean | null;
  "lockup_s"?: string | null;
  "always_close_on_withdraw"?: boolean | null;
  "relationship"?: string | null;
  "child_vaults"?: string | null;
  "followers"?: number | bigint | null;
  "snapshot_time"?: number | bigint | null;
}

export interface ListVaultsResponse {
  [field: string]: unknown;
  "data": Array<ListVaultsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface ListWhaleDepositsRequest {
  "hours"?: number | bigint | null;
  "min_usd"?: ListWhaleDepositsRequestMinUsd | null;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface ListWhaleDepositsResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "address"?: string | null;
  "usdc"?: string | null;
  "fee"?: string | null;
  "hash"?: string | null;
}

export interface ListWhaleDepositsResponse {
  [field: string]: unknown;
  "data": Array<ListWhaleDepositsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface LookupAloPriorityRequestBody {
  "oids": Array<string>;
}

export interface LookupAloPriorityRequest {
  "coin": string;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
  "body": LookupAloPriorityRequestBody;
}

export interface LookupAloPriorityResponseDataItem {
  [field: string]: unknown;
  "time"?: number | bigint | null;
  "oid"?: string | null;
  "user"?: string | null;
  "side"?: string | null;
  "px"?: string | null;
  "sz"?: string | null;
  "p"?: number | bigint | null;
  "tif"?: string | null;
  "fee_usd"?: string | null;
  "block"?: number | bigint | null;
  "bundle_index"?: number | bigint | null;
  "action_index"?: number | bigint | null;
  "order_index"?: number | bigint | null;
}

export interface LookupAloPriorityResponse {
  [field: string]: unknown;
  "data": Array<LookupAloPriorityResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface MarketPositionBookRequest {
  "coin": string;
  "at": number | bigint;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface MarketPositionBookResponseDataItem {
  [field: string]: unknown;
  "kind"?: string | null;
  "snapshot_time"?: number | bigint | null;
  "address"?: string | null;
  "size"?: string | null;
  "notional"?: string | null;
  "entry_px"?: string | null;
  "liquidation_px"?: string | null;
  "unrealized_pnl"?: string | null;
  "leverage"?: string | null;
  "still_open"?: boolean | null;
  "liquidated_since"?: boolean | null;
  "positions"?: number | bigint | null;
  "current_snapshot_time"?: number | bigint | null;
}

export interface MarketPositionBookResponse {
  [field: string]: unknown;
  "data": Array<MarketPositionBookResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface OptionsIvRequest {
  "coin": OptionsIvRequestCoin;
  "mode"?: OptionsIvRequestMode | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface OptionsIvResponseDataItem {
  [field: string]: unknown;
  "kind"?: string | null;
  "snapshot_time"?: number | bigint | null;
  "data"?: string | null;
}

export interface OptionsIvResponse {
  [field: string]: unknown;
  "data": Array<OptionsIvResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface PortfolioLeaderboardRequest {
  "sort"?: PortfolioLeaderboardRequestSort | null;
  "window"?: PortfolioLeaderboardRequestWindow | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface PortfolioLeaderboardResponseDataItem {
  [field: string]: unknown;
  "rank"?: number | bigint | null;
  "address"?: string | null;
  "display_name"?: string | null;
  "label"?: string | null;
  "tag"?: string | null;
  "account_value"?: string | null;
  "unrealized_pnl"?: string | null;
  "pnl_day"?: string | null;
  "pnl_week"?: string | null;
  "pnl_month"?: string | null;
  "pnl_alltime"?: string | null;
  "winrate"?: string | null;
  "total_trades"?: number | bigint | null;
  "total_volume"?: string | null;
  "copy_score"?: string | null;
  "top_assets"?: string | null;
  "last_trade_at"?: number | bigint | null;
  "verified"?: boolean | null;
  "sort_value"?: string | null;
  "updated_at"?: number | bigint | null;
}

export interface PortfolioLeaderboardResponse {
  [field: string]: unknown;
  "data": Array<PortfolioLeaderboardResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface PostAccountsBatchRequestBody {
  "addresses": Array<string>;
  "dex"?: string | null;
}

export interface PostAccountsBatchRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
  "body": PostAccountsBatchRequestBody;
}

export interface PostAccountsBatchResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "account_value"?: string | null;
  "total_notional"?: string | null;
  "total_margin_used"?: string | null;
  "cross_maintenance_margin_used"?: string | null;
  "withdrawable"?: string | null;
  "positions"?: number | bigint | null;
  "as_of"?: number | bigint | null;
  "equity"?: string | null;
  "spot_value"?: string | null;
  "staked_value"?: string | null;
}

export interface PostAccountsBatchResponse {
  [field: string]: unknown;
  "data": Array<PostAccountsBatchResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface PostAccountsBatchPortfolioRequestBody {
  "addresses": Array<string>;
}

export interface PostAccountsBatchPortfolioRequest {
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
  "body": PostAccountsBatchPortfolioRequestBody;
}

export interface PostAccountsBatchPortfolioResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "abstraction"?: string | null;
  "account_value"?: string | null;
  "total_notional"?: string | null;
  "withdrawable"?: string | null;
  "maintenance_margin"?: string | null;
  "perp_positions"?: number | bigint | null;
  "spot_balances"?: number | bigint | null;
  "spot_usdc"?: string | null;
  "as_of"?: number | bigint | null;
  "equity"?: string | null;
  "spot_value"?: string | null;
  "staked_value"?: string | null;
}

export interface PostAccountsBatchPortfolioResponse {
  [field: string]: unknown;
  "data": Array<PostAccountsBatchPortfolioResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface SearchWalletsRequest {
  "search": string;
  "limit"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface SearchWalletsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "display_name"?: string | null;
  "avatar"?: string | null;
  "twitter"?: string | null;
  "tag"?: string | null;
  "verified"?: boolean | null;
  "label"?: string | null;
  "match"?: number | bigint | null;
}

export interface SearchWalletsResponse {
  [field: string]: unknown;
  "data": Array<SearchWalletsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface SnapshotAccountValuesRequest {
  "collateral"?: SnapshotAccountValuesRequestCollateral | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface SnapshotAccountValuesResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "account_value"?: string | null;
  "long_notional"?: string | null;
  "short_notional"?: string | null;
  "unified"?: boolean | null;
  "mode"?: string | null;
}

export interface SnapshotAccountValuesResponse {
  [field: string]: unknown;
  "data": Array<SnapshotAccountValuesResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface SnapshotBorrowersRequest {
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface SnapshotBorrowersResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "mode"?: string | null;
  "borrowed_usd"?: string | null;
  "supplied_usd"?: string | null;
  "eligible_collateral_usd"?: string | null;
  "liquidation_value"?: string | null;
  "maintenance"?: string | null;
  "wallet_hype_usd"?: string | null;
  "wallet_btc_usd"?: string | null;
  "unpriced"?: boolean | null;
  "borrows"?: string | null;
  "supplies"?: string | null;
}

export interface SnapshotBorrowersResponse {
  [field: string]: unknown;
  "data": Array<SnapshotBorrowersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface SnapshotMarketPositionsRequest {
  "coin": string;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface SnapshotMarketPositionsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "coin"?: string | null;
  "dex"?: number | bigint | null;
  "size"?: string | null;
  "entry_notional"?: string | null;
  "entry_px"?: string | null;
  "mark_px"?: string | null;
  "liquidation_px"?: string | null;
  "leverage_type"?: string | null;
  "leverage"?: number | bigint | null;
  "isolated_margin"?: string | null;
  "cross_balance"?: string | null;
  "funding_accrued"?: string | null;
  "margin_table_id"?: number | bigint | null;
  "account_value"?: string | null;
  "spot_usdc"?: string | null;
  "abstraction"?: string | null;
  "updated_height"?: number | bigint | null;
  "margin_uncertain"?: boolean | null;
}

export interface SnapshotMarketPositionsResponse {
  [field: string]: unknown;
  "data": Array<SnapshotMarketPositionsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface SnapshotPortfolioMarginAccountsRequest {
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface SnapshotPortfolioMarginAccountsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
}

export interface SnapshotPortfolioMarginAccountsResponse {
  [field: string]: unknown;
  "data": Array<SnapshotPortfolioMarginAccountsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface SnapshotPositionedAccountsRequest {
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface SnapshotPositionedAccountsResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "dexes"?: string | null;
}

export interface SnapshotPositionedAccountsResponse {
  [field: string]: unknown;
  "data": Array<SnapshotPositionedAccountsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface SnapshotSpotHoldersRequest {
  "token": string;
  "min_balance"?: string | null;
  "max_balance"?: string | null;
  "min_value"?: string | null;
  "max_value"?: string | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface SnapshotSpotHoldersResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "balance"?: string | null;
  "value_usd"?: string | null;
}

export interface SnapshotSpotHoldersResponse {
  [field: string]: unknown;
  "data": Array<SnapshotSpotHoldersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface SnapshotTriggerOrdersRequest {
  "coin": string;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface SnapshotTriggerOrdersResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "coin"?: string | null;
  "side"?: string | null;
  "order_type"?: string | null;
  "trigger_condition"?: string | null;
  "trigger_px"?: string | null;
  "limit_px"?: string | null;
  "size"?: string | null;
  "orig_size"?: string | null;
  "oid"?: string | null;
  "time"?: number | bigint | null;
  "reduce_only"?: boolean | null;
  "is_position_tpsl"?: boolean | null;
  "tif"?: string | null;
}

export interface SnapshotTriggerOrdersResponse {
  [field: string]: unknown;
  "data": Array<SnapshotTriggerOrdersResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface TradedAssetsRequest {
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface TradedAssetsResponseDataItem {
  [field: string]: unknown;
  "coin"?: string | null;
  "volume"?: string | null;
  "trades"?: number | bigint | null;
  "traders"?: number | bigint | null;
  "first_trade_hour"?: number | bigint | null;
}

export interface TradedAssetsResponse {
  [field: string]: unknown;
  "data": Array<TradedAssetsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface TraderClassStatsRequest {
  "trader_class": TraderClassStatsRequestTraderClass;
  "year"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface TraderClassStatsResponseDataItem {
  [field: string]: unknown;
  "class"?: string | null;
  "traders"?: number | bigint | null;
  "total_volume"?: string | null;
  "total_pnl"?: string | null;
  "total_trades"?: number | bigint | null;
  "avg_winrate"?: string | null;
  "avg_sharpe_30d"?: string | null;
}

export interface TraderClassStatsResponse {
  [field: string]: unknown;
  "data": Array<TraderClassStatsResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface TraderCohortRequest {
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface TraderCohortResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "display_name"?: string | null;
  "label"?: string | null;
  "tag"?: string | null;
  "winrate"?: string | null;
  "winrate_7d"?: string | null;
  "winrate_30d"?: string | null;
  "winrate_90d"?: string | null;
  "trades_7d"?: number | bigint | null;
  "trades_30d"?: number | bigint | null;
  "total_trades"?: number | bigint | null;
  "long_trades"?: number | bigint | null;
  "short_trades"?: number | bigint | null;
  "winning_trades"?: number | bigint | null;
  "losing_trades"?: number | bigint | null;
  "pnl_day"?: string | null;
  "pnl_week"?: string | null;
  "pnl_month"?: string | null;
  "pnl_alltime"?: string | null;
  "account_value"?: string | null;
  "unrealized_pnl"?: string | null;
  "total_volume"?: string | null;
  "copy_score"?: string | null;
  "avg_duration_minutes"?: string | null;
  "median_duration_minutes"?: string | null;
  "longest_win_streak"?: number | bigint | null;
  "updated_at"?: number | bigint | null;
}

export interface TraderCohortResponse {
  [field: string]: unknown;
  "data": Array<TraderCohortResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface TraderProfitabilityWindowRequest {
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface TraderProfitabilityWindowResponseDataItem {
  [field: string]: unknown;
  "cohort"?: string | null;
  "min_usd"?: string | null;
  "max_usd"?: string | null;
  "traders"?: number | bigint | null;
  "profitable"?: number | bigint | null;
  "unprofitable"?: number | bigint | null;
  "pnl_sum"?: string | null;
  "median_roi"?: string | null;
  "roi_traders"?: number | bigint | null;
}

export interface TraderProfitabilityWindowResponse {
  [field: string]: unknown;
  "data": Array<TraderProfitabilityWindowResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface TraderScreenerRequest {
  "sort"?: TraderScreenerRequestSort | null;
  "order"?: TraderScreenerRequestOrder | null;
  "timeframe"?: TraderScreenerRequestTimeframe | null;
  "min_account_value"?: TraderScreenerRequestMinAccountValue | null;
  "min_pnl"?: TraderScreenerRequestMinPnl | null;
  "min_copy_score"?: TraderScreenerRequestMinCopyScore | null;
  "tag"?: TraderScreenerRequestTag | null;
  "max_account_value"?: TraderScreenerRequestMaxAccountValue | null;
  "max_pnl"?: TraderScreenerRequestMaxPnl | null;
  "max_copy_score"?: TraderScreenerRequestMaxCopyScore | null;
  "winrate_window"?: TraderScreenerRequestWinrateWindow | null;
  "min_winrate"?: TraderScreenerRequestMinWinrate | null;
  "max_winrate"?: TraderScreenerRequestMaxWinrate | null;
  "min_sharpe"?: TraderScreenerRequestMinSharpe | null;
  "max_sharpe"?: TraderScreenerRequestMaxSharpe | null;
  "drawdown_window"?: TraderScreenerRequestDrawdownWindow | null;
  "max_drawdown"?: TraderScreenerRequestMaxDrawdown | null;
  "trades_window"?: TraderScreenerRequestTradesWindow | null;
  "min_trades"?: TraderScreenerRequestMinTrades | null;
  "max_trades"?: TraderScreenerRequestMaxTrades | null;
  "verified"?: TraderScreenerRequestVerified | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface TraderScreenerResponseDataItem {
  [field: string]: unknown;
  "address"?: string | null;
  "rank"?: number | bigint | null;
  "pnl_day"?: string | null;
  "pnl_week"?: string | null;
  "pnl_month"?: string | null;
  "pnl_alltime"?: string | null;
  "total_trades"?: number | bigint | null;
  "trades_7d"?: number | bigint | null;
  "trades_30d"?: number | bigint | null;
  "long_trades"?: number | bigint | null;
  "short_trades"?: number | bigint | null;
  "winning_trades"?: number | bigint | null;
  "losing_trades"?: number | bigint | null;
  "winrate"?: string | null;
  "winrate_7d"?: string | null;
  "winrate_30d"?: string | null;
  "winrate_90d"?: string | null;
  "last_trade_at"?: number | bigint | null;
  "last_fill_at"?: number | bigint | null;
  "account_value"?: string | null;
  "notional"?: string | null;
  "unrealized_pnl"?: string | null;
  "display_name"?: string | null;
  "twitter"?: string | null;
  "tag"?: string | null;
  "verified"?: boolean | null;
  "label"?: string | null;
  "sharpe_7d"?: string | null;
  "sharpe_30d"?: string | null;
  "sortino_7d"?: string | null;
  "sortino_30d"?: string | null;
  "sortino_90d"?: string | null;
  "max_drawdown_week"?: string | null;
  "max_drawdown_month"?: string | null;
  "max_drawdown_90d"?: string | null;
  "gpr_7d"?: string | null;
  "gpr_30d"?: string | null;
  "gpr_90d"?: string | null;
  "gpr_all_time"?: string | null;
  "copy_score"?: string | null;
  "copy_score_v2"?: string | null;
  "top_assets"?: string | null;
  "total_volume"?: string | null;
  "updated_at"?: number | bigint | null;
  "avatar"?: string | null;
}

export interface TraderScreenerResponse {
  [field: string]: unknown;
  "data": Array<TraderScreenerResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface VenueShareRequest {
  "mode"?: VenueShareRequestMode | null;
  "venue"?: string | null;
  "start_time"?: number | bigint | null;
  "end_time"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
  "max_credits"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
}

export interface VenueShareResponseDataItem {
  [field: string]: unknown;
  "venue"?: string | null;
  "snapshot_time"?: number | bigint | null;
  "data"?: string | null;
  "name"?: string | null;
  "volume_usd"?: string | null;
  "oi_usd"?: string | null;
  "total_volume_usd"?: string | null;
  "total_oi_usd"?: string | null;
  "volume_share_pct"?: string | null;
  "oi_share_pct"?: string | null;
  "volume_rank"?: number | bigint | null;
  "oi_rank"?: number | bigint | null;
}

export interface VenueShareResponse {
  [field: string]: unknown;
  "data": Array<VenueShareResponseDataItem>;
  "has_more": boolean;
  "next_cursor": string | null;
  "meta": Record<string, HttpValue>;
}

export interface GetDatasetsRequest {
}

export interface GetMeRequest {
}

export interface GetStatusRequest {
}

export interface ListJobsRequest {
}

export interface ResponseMetadata {
  "request_id"?: string | null;
  "credits_charged"?: number | bigint | null;
  "credit_price_version"?: number | bigint | null;
  "credits_low"?: boolean | null;
  "problem_code"?: string | null;
  "retry_after"?: string | null;
  "has_more"?: boolean | null;
  "next_cursor"?: string | null;
  "as_of"?: number | bigint | null;
  "complete_to"?: number | bigint | null;
  "truncated"?: boolean | null;
  "max_rows"?: number | bigint | null;
  "incomplete"?: boolean | null;
}

export interface AccountPnl90dResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": AccountPnl90dResponse;
}

export interface TextResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": string;
}

export interface AccountPositionHistoryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": AccountPositionHistoryResponse;
}

export interface BatchRecentTradesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": BatchRecentTradesResponse;
}

export interface BuilderMembersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": BuilderMembersResponse;
}

export interface CancelJobResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": CancelJobResponse;
}

export interface CreateJobResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": CreateJobResponse;
}

export interface EtfFlowsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": EtfFlowsResponse;
}

export interface FootprintAllWalletsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": FootprintAllWalletsResponse;
}

export interface GetAccountResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountResponse;
}

export interface GetAccountAgentsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountAgentsResponse;
}

export interface GetAccountAllDexesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountAllDexesResponse;
}

export interface GetAccountApprovedBuildersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountApprovedBuildersResponse;
}

export interface GetAccountBorrowLendResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountBorrowLendResponse;
}

export interface GetAccountBuilderFeeResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountBuilderFeeResponse;
}

export interface GetAccountCoreWriterActionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountCoreWriterActionsResponse;
}

export interface GetAccountFeesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountFeesResponse;
}

export interface GetAccountFillsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountFillsResponse;
}

export interface GetAccountFundingResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountFundingResponse;
}

export interface GetAccountL1ActionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountL1ActionsResponse;
}

export interface GetAccountLeadingVaultsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountLeadingVaultsResponse;
}

export interface GetAccountLedgerResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountLedgerResponse;
}

export interface GetAccountMarketResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountMarketResponse;
}

export interface GetAccountMultisigSignersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountMultisigSignersResponse;
}

export interface GetAccountOpenOrdersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountOpenOrdersResponse;
}

export interface GetAccountOrderHistoryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountOrderHistoryResponse;
}

export interface GetAccountOrderStatusResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountOrderStatusResponse;
}

export interface GetAccountPnlChartResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountPnlChartResponse;
}

export interface GetAccountPnlSummaryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountPnlSummaryResponse;
}

export interface GetAccountPortfolioResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountPortfolioResponse;
}

export interface GetAccountPortfolioDailyResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountPortfolioDailyResponse;
}

export interface GetAccountPositionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountPositionsResponse;
}

export interface GetAccountRateLimitResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountRateLimitResponse;
}

export interface GetAccountRealizedPnlResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountRealizedPnlResponse;
}

export interface GetAccountRiskResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountRiskResponse;
}

export interface GetAccountSpotBalancesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountSpotBalancesResponse;
}

export interface GetAccountStakingResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountStakingResponse;
}

export interface GetAccountStakingHistoryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountStakingHistoryResponse;
}

export interface GetAccountSubaccountHistoryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountSubaccountHistoryResponse;
}

export interface GetAccountSubaccountsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountSubaccountsResponse;
}

export interface GetAccountTradesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountTradesResponse;
}

export interface GetAccountTwapFillsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountTwapFillsResponse;
}

export interface GetAccountTwapHistoryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountTwapHistoryResponse;
}

export interface GetAccountTwapsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountTwapsResponse;
}

export interface GetAccountValueHistoryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountValueHistoryResponse;
}

export interface GetAccountVaultEquitiesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountVaultEquitiesResponse;
}

export interface GetAccountsBatchResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountsBatchResponse;
}

export interface GetAccountsBatchPortfolioResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountsBatchPortfolioResponse;
}

export interface GetAccountsBatchStatsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountsBatchStatsResponse;
}

export interface GetAccountsLiquidationRiskResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAccountsLiquidationRiskResponse;
}

export interface GetActiveTradersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetActiveTradersResponse;
}

export interface GetActiveTwapPressureResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetActiveTwapPressureResponse;
}

export interface GetAloPriorityResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAloPriorityResponse;
}

export interface GetAloPriorityBarsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAloPriorityBarsResponse;
}

export interface GetAssetScoreLeaderboardResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAssetScoreLeaderboardResponse;
}

export interface GetAssistanceFundBuybacksResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetAssistanceFundBuybacksResponse;
}

export interface GetBarFlowsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetBarFlowsResponse;
}

export interface GetBoxTradersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetBoxTradersResponse;
}

export interface GetBridgeFlowsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetBridgeFlowsResponse;
}

export interface GetBuilderSubBuilderSplitResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetBuilderSubBuilderSplitResponse;
}

export interface GetBuilderVolumeShareResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetBuilderVolumeShareResponse;
}

export interface GetCandlesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetCandlesResponse;
}

export interface GetChangelogResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetChangelogResponse;
}

export interface GetCohortFlowsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetCohortFlowsResponse;
}

export interface GetCohortPositioningResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetCohortPositioningResponse;
}

export interface GetCohortPositioningMarketsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetCohortPositioningMarketsResponse;
}

export interface GetCohortTotalsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetCohortTotalsResponse;
}

export interface GetCvdResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetCvdResponse;
}

export interface GetDatasetsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetDatasetsResponse;
}

export interface GetDexLimitsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetDexLimitsResponse;
}

export interface GetDexStatusResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetDexStatusResponse;
}

export interface GetEquitySessionGapsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetEquitySessionGapsResponse;
}

export interface GetEquitySessionSlotsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetEquitySessionSlotsResponse;
}

export interface GetEventMovesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetEventMovesResponse;
}

export interface GetExchangeFeesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetExchangeFeesResponse;
}

export interface GetExchangeStatusResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetExchangeStatusResponse;
}

export interface GetFootprintResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetFootprintResponse;
}

export interface GetFootprintActionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetFootprintActionsResponse;
}

export interface GetFundingFlowsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetFundingFlowsResponse;
}

export interface GetFundingMatrixResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetFundingMatrixResponse;
}

export interface GetFundingRatesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetFundingRatesResponse;
}

export interface GetFundingScreenerResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetFundingScreenerResponse;
}

export interface GetFundingTopWalletsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetFundingTopWalletsResponse;
}

export interface GetHip3OracleBarsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetHip3OracleBarsResponse;
}

export interface GetHlpResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetHlpResponse;
}

export interface GetHypeHolderBandsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetHypeHolderBandsResponse;
}

export interface GetHypeSupplyResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetHypeSupplyResponse;
}

export interface GetJobResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetJobResponse;
}

export interface GetJobResultResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetJobResultResponse;
}

export interface GetL4BookResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetL4BookResponse;
}

export interface GetLendingReserveHistoryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetLendingReserveHistoryResponse;
}

export interface GetLiquidationCascadesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetLiquidationCascadesResponse;
}

export interface GetLiquidationHeatmapResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetLiquidationHeatmapResponse;
}

export interface GetLiquidationLevelTotalsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetLiquidationLevelTotalsResponse;
}

export interface GetLiquidationLevelsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetLiquidationLevelsResponse;
}

export interface GetLiquidationRiskLadderResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetLiquidationRiskLadderResponse;
}

export interface GetLiquidationSeriesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetLiquidationSeriesResponse;
}

export interface GetLiquidationTotalsByMarketResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetLiquidationTotalsByMarketResponse;
}

export interface GetMarketBetaResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetMarketBetaResponse;
}

export interface GetMarketContextHistoryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetMarketContextHistoryResponse;
}

export interface GetMarketCorrelationResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetMarketCorrelationResponse;
}

export interface GetMarketLeaderboardResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetMarketLeaderboardResponse;
}

export interface GetMarketParticipationResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetMarketParticipationResponse;
}

export interface GetMarketPositionChangesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetMarketPositionChangesResponse;
}

export interface GetMarketPositionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetMarketPositionsResponse;
}

export interface GetMarketStopOrdersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetMarketStopOrdersResponse;
}

export interface GetMarketTradesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetMarketTradesResponse;
}

export interface GetMarketTwapHistoryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetMarketTwapHistoryResponse;
}

export interface GetMarketTwapLevelsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetMarketTwapLevelsResponse;
}

export interface GetMeResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetMeResponse;
}

export interface GetNetPositioningByMarketResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetNetPositioningByMarketResponse;
}

export interface GetNetPositioningSummaryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetNetPositioningSummaryResponse;
}

export interface GetNewTradersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetNewTradersResponse;
}

export interface GetOpenInterestResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetOpenInterestResponse;
}

export interface GetOpenInterestByCategoryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetOpenInterestByCategoryResponse;
}

export interface GetOpenInterestByDexResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetOpenInterestByDexResponse;
}

export interface GetOpenInterestChangesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetOpenInterestChangesResponse;
}

export interface GetOpenInterestDailyResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetOpenInterestDailyResponse;
}

export interface GetOpenInterestTotalResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetOpenInterestTotalResponse;
}

export interface GetOrderBookResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetOrderBookResponse;
}

export interface GetOrderBookAtResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetOrderBookAtResponse;
}

export interface GetOutcomeActivityResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetOutcomeActivityResponse;
}

export interface GetOutcomeHoldersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetOutcomeHoldersResponse;
}

export interface GetOutcomeLeaderboardResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetOutcomeLeaderboardResponse;
}

export interface GetOutcomeProbabilityResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetOutcomeProbabilityResponse;
}

export interface GetPnlLeaderboardResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetPnlLeaderboardResponse;
}

export interface GetPositionCountsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetPositionCountsResponse;
}

export interface GetPositioningResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetPositioningResponse;
}

export interface GetPriceChangesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetPriceChangesResponse;
}

export interface GetPriorityFeeSummaryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetPriorityFeeSummaryResponse;
}

export interface GetPriorityFeesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetPriorityFeesResponse;
}

export interface GetPriorityFeesBuilderShareResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetPriorityFeesBuilderShareResponse;
}

export interface GetPriorityFeesByCoinResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetPriorityFeesByCoinResponse;
}

export interface GetPriorityFeesTopCoinsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetPriorityFeesTopCoinsResponse;
}

export interface GetPriorityFillsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetPriorityFillsResponse;
}

export interface GetQuoteResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetQuoteResponse;
}

export interface GetRealizedVolatilityResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetRealizedVolatilityResponse;
}

export interface GetReferralResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetReferralResponse;
}

export interface GetReturnsMatrixResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetReturnsMatrixResponse;
}

export interface GetRevenueResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetRevenueResponse;
}

export interface GetSentimentResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetSentimentResponse;
}

export interface GetSentimentEventsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetSentimentEventsResponse;
}

export interface GetSentimentScannerResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetSentimentScannerResponse;
}

export interface GetSessionReturnsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetSessionReturnsResponse;
}

export interface GetSortedLeaderboardResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetSortedLeaderboardResponse;
}

export interface GetSpotTwapLevelsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetSpotTwapLevelsResponse;
}

export interface GetStakeHistoryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetStakeHistoryResponse;
}

export interface GetStakerBucketsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetStakerBucketsResponse;
}

export interface GetStakingFlowsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetStakingFlowsResponse;
}

export interface GetStatusResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetStatusResponse;
}

export interface GetStopHeatmapResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetStopHeatmapResponse;
}

export interface GetStopLevelTotalsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetStopLevelTotalsResponse;
}

export interface GetStopLevelsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetStopLevelsResponse;
}

export interface GetStopOrdersAtResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetStopOrdersAtResponse;
}

export interface GetStopSummaryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetStopSummaryResponse;
}

export interface GetTokenHolderBandsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetTokenHolderBandsResponse;
}

export interface GetTokenSupplyResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetTokenSupplyResponse;
}

export interface GetTopLiquidationLevelsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetTopLiquidationLevelsResponse;
}

export interface GetTopStopLevelsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetTopStopLevelsResponse;
}

export interface GetTopTraderConsensusResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetTopTraderConsensusResponse;
}

export interface GetTopTraderOpensResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetTopTraderOpensResponse;
}

export interface GetTradeBarsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetTradeBarsResponse;
}

export interface GetTraderDistributionResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetTraderDistributionResponse;
}

export interface GetTraderGroupResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetTraderGroupResponse;
}

export interface GetTraderProfitabilityResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetTraderProfitabilityResponse;
}

export interface GetTwapPressureResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetTwapPressureResponse;
}

export interface GetUsdcReserveResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetUsdcReserveResponse;
}

export interface GetUsdcReserveIntervalsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetUsdcReserveIntervalsResponse;
}

export interface GetUsdcReserveSettlementsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetUsdcReserveSettlementsResponse;
}

export interface GetUserGrowthResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetUserGrowthResponse;
}

export interface GetValidatorRegistryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetValidatorRegistryResponse;
}

export interface GetValidatorRewardsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetValidatorRewardsResponse;
}

export interface GetValidatorsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetValidatorsResponse;
}

export interface GetVaultHistoryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetVaultHistoryResponse;
}

export interface GetWalletBuildersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletBuildersResponse;
}

export interface GetWalletCompletedTradesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletCompletedTradesResponse;
}

export interface GetWalletCopyActivityResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletCopyActivityResponse;
}

export interface GetWalletCopyScoreResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletCopyScoreResponse;
}

export interface GetWalletHighlightsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletHighlightsResponse;
}

export interface GetWalletLeaderboardResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletLeaderboardResponse;
}

export interface GetWalletMarketsTradedResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletMarketsTradedResponse;
}

export interface GetWalletOrderFillsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletOrderFillsResponse;
}

export interface GetWalletOutcomesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletOutcomesResponse;
}

export interface GetWalletPnlCalendarResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletPnlCalendarResponse;
}

export interface GetWalletProfilesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletProfilesResponse;
}

export interface GetWalletReviewResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletReviewResponse;
}

export interface GetWalletRiskMetricsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletRiskMetricsResponse;
}

export interface GetWalletSharpeResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletSharpeResponse;
}

export interface GetWalletStakedHypeResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletStakedHypeResponse;
}

export interface GetWalletStatsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletStatsResponse;
}

export interface GetWalletStatsByMarketResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletStatsByMarketResponse;
}

export interface GetWalletTradeCountsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletTradeCountsResponse;
}

export interface GetWalletUpnlRankResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletUpnlRankResponse;
}

export interface GetWalletWindowStatsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletWindowStatsResponse;
}

export interface GetWalletYearSummaryResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWalletYearSummaryResponse;
}

export interface GetWindowPricesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": GetWindowPricesResponse;
}

export interface LiquidationMinutesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": LiquidationMinutesResponse;
}

export interface LiquidationMixResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": LiquidationMixResponse;
}

export interface ListAllTwapsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListAllTwapsResponse;
}

export interface ListCohortMembersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListCohortMembersResponse;
}

export interface ListCohortTopPositionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListCohortTopPositionsResponse;
}

export interface ListDeployAuctionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListDeployAuctionsResponse;
}

export interface ListDexMarketsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListDexMarketsResponse;
}

export interface ListDexsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListDexsResponse;
}

export interface ListEvmCoreTransfersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListEvmCoreTransfersResponse;
}

export interface ListFreshWalletPositionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListFreshWalletPositionsResponse;
}

export interface ListGossipAuctionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListGossipAuctionsResponse;
}

export interface ListHip3OracleUpdatesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListHip3OracleUpdatesResponse;
}

export interface ListJobsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListJobsResponse;
}

export interface ListLargestOpensResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListLargestOpensResponse;
}

export interface ListLargestPositionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListLargestPositionsResponse;
}

export interface ListLendingReservesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListLendingReservesResponse;
}

export interface ListLiquidationsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListLiquidationsResponse;
}

export interface ListMarginTablesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListMarginTablesResponse;
}

export interface ListMarketCatalogResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListMarketCatalogResponse;
}

export interface ListMarketTwapsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListMarketTwapsResponse;
}

export interface ListMarketsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListMarketsResponse;
}

export interface ListMaxMarketOrderNotionalsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListMaxMarketOrderNotionalsResponse;
}

export interface ListMidsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListMidsResponse;
}

export interface ListNetworkDeploysResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListNetworkDeploysResponse;
}

export interface ListOutcomeDefinitionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListOutcomeDefinitionsResponse;
}

export interface ListOutcomeMarketsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListOutcomeMarketsResponse;
}

export interface ListOutcomeStatsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListOutcomeStatsResponse;
}

export interface ListOutcomeWhaleTradesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListOutcomeWhaleTradesResponse;
}

export interface ListPositionsNearLiquidationResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListPositionsNearLiquidationResponse;
}

export interface ListPriorityFeeTopUsersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListPriorityFeeTopUsersResponse;
}

export interface ListSpotMarketsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListSpotMarketsResponse;
}

export interface ListSpotTokensResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListSpotTokensResponse;
}

export interface ListSpotTwapsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListSpotTwapsResponse;
}

export interface ListStakerChangesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListStakerChangesResponse;
}

export interface ListSubBuilderEarningsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListSubBuilderEarningsResponse;
}

export interface ListTopLiquidationsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListTopLiquidationsResponse;
}

export interface ListTopStakersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListTopStakersResponse;
}

export interface ListTopUnrealizedPnlResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListTopUnrealizedPnlResponse;
}

export interface ListTopUnrealizedPositionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListTopUnrealizedPositionsResponse;
}

export interface ListUnrealizedPnlMoversResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListUnrealizedPnlMoversResponse;
}

export interface ListValidatorDelegationsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListValidatorDelegationsResponse;
}

export interface ListVaultFollowersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListVaultFollowersResponse;
}

export interface ListVaultsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListVaultsResponse;
}

export interface ListWhaleDepositsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": ListWhaleDepositsResponse;
}

export interface LookupAloPriorityResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": LookupAloPriorityResponse;
}

export interface MarketPositionBookResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": MarketPositionBookResponse;
}

export interface OptionsIvResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": OptionsIvResponse;
}

export interface PortfolioLeaderboardResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": PortfolioLeaderboardResponse;
}

export interface PostAccountsBatchResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": PostAccountsBatchResponse;
}

export interface PostAccountsBatchPortfolioResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": PostAccountsBatchPortfolioResponse;
}

export interface SearchWalletsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": SearchWalletsResponse;
}

export interface SnapshotAccountValuesResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": SnapshotAccountValuesResponse;
}

export interface SnapshotBorrowersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": SnapshotBorrowersResponse;
}

export interface SnapshotMarketPositionsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": SnapshotMarketPositionsResponse;
}

export interface SnapshotPortfolioMarginAccountsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": SnapshotPortfolioMarginAccountsResponse;
}

export interface SnapshotPositionedAccountsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": SnapshotPositionedAccountsResponse;
}

export interface SnapshotSpotHoldersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": SnapshotSpotHoldersResponse;
}

export interface SnapshotTriggerOrdersResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": SnapshotTriggerOrdersResponse;
}

export interface TradedAssetsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": TradedAssetsResponse;
}

export interface TraderClassStatsResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": TraderClassStatsResponse;
}

export interface TraderCohortResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": TraderCohortResponse;
}

export interface TraderProfitabilityWindowResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": TraderProfitabilityWindowResponse;
}

export interface TraderScreenerResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": TraderScreenerResponse;
}

export interface VenueShareResult {
  [field: string]: unknown;
  "status": number;
  "metadata": ResponseMetadata;
  "data": VenueShareResponse;
}
