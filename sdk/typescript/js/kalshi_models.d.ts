// Generated from src/exchanges/kalshi/schema/kalshi.json.
import type { HttpValue } from "./index";
export type AcceptBlockTradeProposalResponse = HttpValue;
export type AcceptQuoteResponse = HttpValue;
export type AcceptRfqQuoteResponse = HttpValue;
export type CancelAllOrdersResponse = HttpValue;
export type ConfirmQuoteResponse = HttpValue;
export type ConfirmRfqBatchQuoteResponse = HttpValue;
export type ConfirmRfqQuoteResponse = HttpValue;
export type DeleteApiKeyResponse = HttpValue;
export type DeleteEventContractExitTriggerResponse = HttpValue;
export type DeleteQuoteResponse = HttpValue;
export type DeleteRfqBatchQuoteResponse = HttpValue;
export type DeleteRfqBatchResponse = HttpValue;
export type DeleteRfqQuoteResponse = HttpValue;
export type DeleteRfqResponse = HttpValue;
export type PredictionsApiKeyScope = string;
export type PredictionsApiKeyType = string;
export type PredictionsBookSide = string;
export type PredictionsExchangeIndex = number | bigint | string;
export type PredictionsExchangeInstance = string;
export type PredictionsFeeType = string;
export type PredictionsFixedPointCount = string;
export type PredictionsFixedPointDollars = string;
export type PredictionsIntraExchangeInstanceTransferStatus = string;
export type PredictionsMarketIndexStatus = string;
export type PredictionsOrderStatus = string;
export type PredictionsOutcomeSide = string;
export type PredictionsPriceLevelDollarsCountFp = Array<string>;
export type PredictionsRFQBatchQuoteStatus = string;
export type PredictionsRFQBatchStatus = string;
export type PredictionsRestingMarginReservation = string;
export type PredictionsSelfTradePreventionType = string;
export type PredictionsUserFilter = string;
export type UpdateSubaccountNettingResponse = HttpValue;
export type UpgradeAccountApiUsageLevelResponse = HttpValue;

export interface AcceptBlockTradeProposalParams {
  [field: string]: unknown;
  "block_trade_proposal_id": string;
  "body"?: PredictionsAcceptBlockTradeProposalRequest | null;
}

export interface AcceptQuoteParams {
  [field: string]: unknown;
  "quote_id": string;
  "body": PredictionsAcceptQuoteRequest;
}

export interface AcceptRfqBatchParams {
  [field: string]: unknown;
  "rfq_batch_id": string;
  "body": PredictionsAcceptRFQBatchRequest;
}

export interface AcceptRfqBatchResponse {
  [field: string]: unknown;
  "accepted_side": string;
  "quotes": Array<PredictionsQuote>;
}

export interface AcceptRfqQuoteParams {
  [field: string]: unknown;
  "rfq_id": string;
  "quote_id": string;
  "body": PredictionsAcceptQuoteRequest;
}

export interface AmendOrderV2Params {
  [field: string]: unknown;
  "order_id": string;
  "subaccount"?: number | bigint | string | null;
  "body": PredictionsAmendOrderV2Request;
}

export interface AmendOrderV2Response {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "remaining_count"?: string | null;
  "fill_count"?: string | null;
  "average_fill_price"?: string | null;
  "average_fee_paid"?: string | null;
  "ts_ms": number | bigint | string;
}

export interface ApplySubaccountTransferParams {
  [field: string]: unknown;
  "body": PredictionsApplySubaccountTransferRequest;
}

export interface ApplySubaccountTransferResponse {
  [field: string]: unknown;
}

export interface BatchCancelOrdersV2Params {
  [field: string]: unknown;
  "body": PredictionsBatchCancelOrdersV2Request;
}

export interface BatchCancelOrdersV2Response {
  [field: string]: unknown;
  "orders": Array<BatchCancelOrdersV2ResponseOrdersItem>;
}

export interface BatchCancelOrdersV2ResponseOrdersItem {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "reduced_by": string;
  "ts_ms"?: number | bigint | string | null;
  "error"?: BatchCancelOrdersV2ResponseOrdersItemError | null;
}

export interface BatchCancelOrdersV2ResponseOrdersItemError {
  [field: string]: unknown;
  "code"?: string | null;
  "message"?: string | null;
  "details"?: string | null;
}

export interface BatchCreateOrdersV2Params {
  [field: string]: unknown;
  "body": PredictionsBatchCreateOrdersV2Request;
}

export interface BatchCreateOrdersV2Response {
  [field: string]: unknown;
  "orders": Array<BatchCreateOrdersV2ResponseOrdersItem>;
}

export interface BatchCreateOrdersV2ResponseOrdersItem {
  [field: string]: unknown;
  "order_id"?: string | null;
  "client_order_id"?: string | null;
  "fill_count"?: string | null;
  "remaining_count"?: string | null;
  "average_fill_price"?: string | null;
  "average_fee_paid"?: string | null;
  "ts_ms"?: number | bigint | string | null;
  "error"?: BatchCreateOrdersV2ResponseOrdersItemError | null;
}

export interface BatchCreateOrdersV2ResponseOrdersItemError {
  [field: string]: unknown;
  "code"?: string | null;
  "message"?: string | null;
  "details"?: string | null;
}

export interface BatchGetMarketCandlesticksParams {
  [field: string]: unknown;
  "market_tickers": string;
  "start_ts": number | bigint | string;
  "end_ts": number | bigint | string;
  "period_interval": number | bigint | string;
  "include_latest_before_start"?: boolean | null;
}

export interface BatchGetMarketCandlesticksResponse {
  [field: string]: unknown;
  "markets": Array<PredictionsMarketCandlesticksResponse>;
}

export interface CancelAllOrdersParams {
  [field: string]: unknown;
  "subaccount"?: number | bigint | string | null;
}

export interface CancelOrderV2Params {
  [field: string]: unknown;
  "order_id": string;
  "subaccount"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
  "market_ticker"?: string | null;
}

export interface CancelOrderV2Response {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "reduced_by": string;
  "ts_ms": number | bigint | string;
}

export interface ConfirmQuoteParams {
  [field: string]: unknown;
  "quote_id": string;
  "body"?: PredictionsEmptyResponse | null;
}

export interface ConfirmRfqBatchQuoteParams {
  [field: string]: unknown;
  "rfq_batch_id": string;
  "batch_quote_id": string;
  "body"?: PredictionsEmptyResponse | null;
}

export interface ConfirmRfqQuoteParams {
  [field: string]: unknown;
  "rfq_id": string;
  "quote_id": string;
  "body"?: PredictionsEmptyResponse | null;
}

export interface CreateApiKeyParams {
  [field: string]: unknown;
  "body": PredictionsCreateApiKeyRequest;
}

export interface CreateApiKeyResponse {
  [field: string]: unknown;
  "api_key_id": string;
  "warning"?: string | null;
}

export interface CreateFcmPositionTransferParams {
  [field: string]: unknown;
  "body": PredictionsCreateFCMPositionTransferRequest;
}

export interface CreateFcmPositionTransferResponse {
  [field: string]: unknown;
  "position_transfer_id": string;
}

export interface CreateFcmSubtraderParams {
  [field: string]: unknown;
  "body": PredictionsCreateFCMSubtraderRequest;
}

export interface CreateFcmSubtraderResponse {
  [field: string]: unknown;
  "subtrader_id": string;
}

export interface CreateMarketInMultivariateEventCollectionParams {
  [field: string]: unknown;
  "collection_ticker": string;
  "body": PredictionsCreateMarketInMultivariateEventCollectionRequest;
}

export interface CreateMarketInMultivariateEventCollectionResponse {
  [field: string]: unknown;
  "event_ticker": string;
  "market_ticker": string;
  "market"?: PredictionsMarket | null;
}

export interface CreateOrderGroupParams {
  [field: string]: unknown;
  "body": PredictionsCreateOrderGroupRequest;
}

export interface CreateOrderGroupResponse {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount": number | bigint | string;
  "exchange_index"?: number | bigint | string | null;
}

export interface CreateOrderV2Params {
  [field: string]: unknown;
  "body": PredictionsCreateOrderV2Request;
}

export interface CreateOrderV2Response {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "fill_count": string;
  "remaining_count": string;
  "average_fill_price"?: string | null;
  "average_fee_paid"?: string | null;
  "ts_ms": number | bigint | string;
}

export interface CreateOrderWithExitTriggerParams {
  [field: string]: unknown;
  "body": PredictionsCreateOrderWithExitTriggerRequest;
}

export interface CreateOrderWithExitTriggerResponse {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "fill_count": string;
  "remaining_count": string;
  "average_fill_price"?: string | null;
  "average_fee_paid"?: string | null;
  "ts_ms": number | bigint | string;
}

export interface CreateQuoteParams {
  [field: string]: unknown;
  "body": PredictionsCreateQuoteRequest;
}

export interface CreateQuoteResponse {
  [field: string]: unknown;
  "id": string;
}

export interface CreateRfqBatchParams {
  [field: string]: unknown;
  "body": PredictionsCreateRFQBatchRequest;
}

export interface CreateRfqBatchQuoteParams {
  [field: string]: unknown;
  "rfq_batch_id": string;
  "body": PredictionsCreateRFQBatchQuoteRequest;
}

export interface CreateRfqBatchQuoteResponse {
  [field: string]: unknown;
  "id": string;
}

export interface CreateRfqBatchResponse {
  [field: string]: unknown;
  "rfq_batch": PredictionsRFQBatch;
}

export interface CreateRfqParams {
  [field: string]: unknown;
  "body": PredictionsCreateRFQRequest;
}

export interface CreateRfqResponse {
  [field: string]: unknown;
  "id": string;
}

export interface CreateSubaccountParams {
  [field: string]: unknown;
  "body"?: PredictionsCreateSubaccountRequest | null;
}

export interface CreateSubaccountResponse {
  [field: string]: unknown;
  "subaccount_number": number | bigint | string;
}

export interface DecreaseOrderV2Params {
  [field: string]: unknown;
  "order_id": string;
  "subaccount"?: number | bigint | string | null;
  "body": PredictionsDecreaseOrderV2Request;
}

export interface DecreaseOrderV2Response {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "remaining_count": string;
  "ts_ms": number | bigint | string;
}

export interface DeleteApiKeyParams {
  [field: string]: unknown;
  "api_key": string;
}

export interface DeleteEventContractExitTriggerParams {
  [field: string]: unknown;
  "ticker": string;
  "subaccount"?: number | bigint | string | null;
  "kind"?: string | null;
}

export interface DeleteFcmEventContractDailyCapParams {
  [field: string]: unknown;
  "subtrader_id": string;
}

export interface DeleteFcmEventContractDailyCapResponse {
  [field: string]: unknown;
}

export interface DeleteOrderGroupParams {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface DeleteOrderGroupResponse {
  [field: string]: unknown;
}

export interface DeleteQuoteParams {
  [field: string]: unknown;
  "quote_id": string;
}

export interface DeleteRfqBatchParams {
  [field: string]: unknown;
  "rfq_batch_id": string;
}

export interface DeleteRfqBatchQuoteParams {
  [field: string]: unknown;
  "rfq_batch_id": string;
  "batch_quote_id": string;
}

export interface DeleteRfqParams {
  [field: string]: unknown;
  "rfq_id": string;
}

export interface DeleteRfqQuoteParams {
  [field: string]: unknown;
  "rfq_id": string;
  "quote_id": string;
}

export interface GenerateApiKeyParams {
  [field: string]: unknown;
  "body": PredictionsGenerateApiKeyRequest;
}

export interface GenerateApiKeyResponse {
  [field: string]: unknown;
  "api_key_id": string;
  "key_type"?: string | null;
  "private_key": string;
  "warning"?: string | null;
}

export interface GetAccountApiLimitsParams {
  [field: string]: unknown;
}

export interface GetAccountApiLimitsResponse {
  [field: string]: unknown;
  "usage_tier": string;
  "read": PredictionsBucketLimit;
  "write": PredictionsBucketLimit;
  "grants": Array<PredictionsApiUsageLevelGrant>;
}

export interface GetAccountApiUsageLevelVolumeProgressParams {
  [field: string]: unknown;
}

export interface GetAccountApiUsageLevelVolumeProgressResponse {
  [field: string]: unknown;
  "volume_progress": Array<PredictionsAccountApiUsageLevelVolumeProgress>;
}

export interface GetAccountEndpointCostsParams {
  [field: string]: unknown;
}

export interface GetAccountEndpointCostsResponse {
  [field: string]: unknown;
  "default_cost": number | bigint | string;
  "endpoint_costs": Array<PredictionsEndpointTokenCost>;
}

export interface GetApiKeysParams {
  [field: string]: unknown;
  "fcm_subtrader_id"?: string | null;
}

export interface GetApiKeysResponse {
  [field: string]: unknown;
  "api_keys": Array<PredictionsApiKey>;
  "api_key_region_expiration_ts"?: number | bigint | string | null;
}

export interface GetBalanceParams {
  [field: string]: unknown;
  "subaccount"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface GetBalanceResponse {
  [field: string]: unknown;
  "balance": number | bigint | string;
  "balance_dollars": string;
  "portfolio_value": number | bigint | string;
  "updated_ts": number | bigint | string;
  "balance_breakdown"?: Array<PredictionsIndexedBalance> | null;
}

export interface GetBlockTradeProposalsParams {
  [field: string]: unknown;
  "cursor"?: string | null;
  "market_ticker"?: string | null;
  "limit"?: number | bigint | string | null;
  "status"?: string | null;
}

export interface GetBlockTradeProposalsResponse {
  [field: string]: unknown;
  "block_trade_proposals": Array<PredictionsBlockTradeProposal>;
  "cursor"?: string | null;
}

export interface GetCommunicationsIdParams {
  [field: string]: unknown;
}

export interface GetCommunicationsIdResponse {
  [field: string]: unknown;
  "communications_id": string;
}

export interface GetCreditHistoryParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
}

export interface GetCreditHistoryResponse {
  [field: string]: unknown;
  "credits": Array<PredictionsCreditHistory>;
  "cursor": string;
}

export interface GetDepositsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetDepositsResponse {
  [field: string]: unknown;
  "deposits": Array<PredictionsDeposit>;
  "cursor"?: string | null;
}

export interface GetEventContractExitTriggersParams {
  [field: string]: unknown;
  "ticker": string;
  "subaccount"?: number | bigint | string | null;
  "kind"?: string | null;
}

export interface GetEventContractExitTriggersResponse {
  [field: string]: unknown;
  "exit_trigger": GetEventContractExitTriggersResponseExitTrigger | null;
  "exit_triggers": Array<PredictionsEventContractExitTrigger>;
}

export interface GetEventContractExitTriggersResponseExitTrigger {
  [field: string]: unknown;
  "slippage"?: PredictionsEventContractExitSlippage | null;
  "id": string;
  "ticker": string;
  "side": string;
  "subaccount_number": number | bigint | string;
  "kind": string;
  "stop_loss_price"?: string | null;
  "take_profit_price"?: string | null;
  "status": string;
  "status_reason"?: string | null;
  "triggered_order_id"?: string | null;
  "triggered_leg"?: string | null;
  "created_time": string;
  "updated_time": string;
}

export interface GetEventFeeChangesParams {
  [field: string]: unknown;
  "event_ticker"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetEventFeeChangesResponse {
  [field: string]: unknown;
  "event_fee_changes": Array<PredictionsEventFeeChange>;
  "cursor": string;
}

export interface GetEventForecastPercentilesHistoryParams {
  [field: string]: unknown;
  "ticker": string;
  "series_ticker": string;
  "percentiles": Array<number | bigint | string>;
  "start_ts": number | bigint | string;
  "end_ts": number | bigint | string;
  "period_interval": number | bigint | string;
}

export interface GetEventForecastPercentilesHistoryResponse {
  [field: string]: unknown;
  "forecast_history": Array<PredictionsForecastPercentilesPoint>;
}

export interface GetEventLiveDataParams {
  [field: string]: unknown;
  "event_ticker": string;
  "range"?: string | null;
}

export interface GetEventLiveDataResponse {
  [field: string]: unknown;
  "live_data": PredictionsEventLiveData;
}

export interface GetEventMetadataParams {
  [field: string]: unknown;
  "event_ticker": string;
}

export interface GetEventMetadataResponse {
  [field: string]: unknown;
  "image_url": string;
  "featured_image_url"?: string | null;
  "market_details": Array<PredictionsMarketMetadata>;
  "settlement_sources": Array<PredictionsSettlementSource>;
  "competition"?: string | null;
  "competition_scope"?: string | null;
}

export interface GetEventParams {
  [field: string]: unknown;
  "event_ticker": string;
  "exclude_markets"?: boolean | null;
  "with_nested_markets"?: boolean | null;
}

export interface GetEventResponse {
  [field: string]: unknown;
  "event": PredictionsEventData;
  "markets": Array<PredictionsMarket>;
}

export interface GetEventsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "with_nested_markets"?: boolean | null;
  "with_milestones"?: boolean | null;
  "status"?: string | null;
  "series_ticker"?: string | null;
  "tickers"?: string | null;
  "min_close_ts"?: number | bigint | string | null;
  "min_updated_ts"?: number | bigint | string | null;
}

export interface GetEventsResponse {
  [field: string]: unknown;
  "events": Array<PredictionsEventData>;
  "milestones"?: Array<PredictionsMilestone> | null;
  "cursor": string;
}

export interface GetExchangeScheduleParams {
  [field: string]: unknown;
}

export interface GetExchangeScheduleResponse {
  [field: string]: unknown;
  "schedule": PredictionsSchedule;
}

export interface GetExchangeStatusParams {
  [field: string]: unknown;
}

export interface GetExchangeStatusResponse {
  [field: string]: unknown;
  "exchange_active": boolean;
  "trading_active": boolean;
  "intra_exchange_transfers_active"?: boolean | null;
  "exchange_estimated_resume_time"?: string | null;
  "exchange_index_statuses"?: Array<PredictionsExchangeIndexStatus> | null;
}

export interface GetFcmEventContractDailyCapParams {
  [field: string]: unknown;
  "subtrader_id"?: string | null;
}

export interface GetFcmEventContractDailyCapResponse {
  [field: string]: unknown;
  "subtrader_id": string;
  "limit": string;
  "executed_utilization": string;
  "resting_order_utilization": string;
  "pending_order_utilization": string;
  "cap_date": string;
}

export interface GetFcmFillsParams {
  [field: string]: unknown;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetFcmFillsResponse {
  [field: string]: unknown;
  "fills": Array<PredictionsFcmFill>;
  "cursor": string;
}

export interface GetFcmOrdersParams {
  [field: string]: unknown;
  "subtrader_id"?: string | null;
  "client_order_ids"?: string | null;
  "cursor"?: string | null;
  "event_ticker"?: string | null;
  "ticker"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
  "status"?: string | null;
  "limit"?: number | bigint | string | null;
}

export interface GetFcmOrdersResponse {
  [field: string]: unknown;
  "orders": Array<PredictionsOrder>;
  "cursor": string;
}

export interface GetFcmPositionsParams {
  [field: string]: unknown;
  "subtrader_id"?: string | null;
  "ticker"?: string | null;
  "event_ticker"?: string | null;
  "count_filter"?: string | null;
  "settlement_status"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetFcmPositionsResponse {
  [field: string]: unknown;
  "cursor"?: string | null;
  "market_positions": Array<PredictionsMarketPosition>;
  "event_positions": Array<PredictionsEventPosition>;
}

export interface GetFcmSubtraderBlockedCategoriesParams {
  [field: string]: unknown;
  "subtrader_id": string;
}

export interface GetFcmSubtraderBlockedCategoriesResponse {
  [field: string]: unknown;
  "categories": Array<string>;
}

export interface GetFillsHistoricalParams {
  [field: string]: unknown;
  "ticker"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "subaccount"?: number | bigint | string | null;
}

export interface GetFillsHistoricalResponse {
  [field: string]: unknown;
  "fills": Array<PredictionsFill>;
  "cursor": string;
}

export interface GetFillsParams {
  [field: string]: unknown;
  "ticker"?: string | null;
  "order_id"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "subaccount"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface GetFillsResponse {
  [field: string]: unknown;
  "fills": Array<PredictionsFill>;
  "cursor": string;
}

export interface GetFiltersForSportsParams {
  [field: string]: unknown;
}

export interface GetFiltersForSportsResponse {
  [field: string]: unknown;
  "filters_by_sports": Record<string, PredictionsSportFilterDetails>;
  "sport_ordering": Array<string>;
}

export interface GetGameStatsParams {
  [field: string]: unknown;
  "milestone_id": string;
}

export interface GetGameStatsResponse {
  [field: string]: unknown;
  "pbp"?: PredictionsPlayByPlay | null;
}

export interface GetHistoricalCutoffParams {
  [field: string]: unknown;
}

export interface GetHistoricalCutoffResponse {
  [field: string]: unknown;
  "market_settled_ts": string;
  "trades_created_ts": string;
  "orders_updated_ts": string;
  "market_positions_last_updated_ts"?: string | null;
}

export interface GetHistoricalMarketParams {
  [field: string]: unknown;
  "ticker": string;
}

export interface GetHistoricalMarketResponse {
  [field: string]: unknown;
  "market": PredictionsMarket;
}

export interface GetHistoricalMarketsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "tickers"?: string | null;
  "event_ticker"?: string | null;
  "series_ticker"?: string | null;
  "mve_filter"?: string | null;
}

export interface GetHistoricalMarketsResponse {
  [field: string]: unknown;
  "markets": Array<PredictionsMarket>;
  "cursor": string;
}

export interface GetHistoricalOrdersParams {
  [field: string]: unknown;
  "ticker"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "subaccount"?: number | bigint | string | null;
}

export interface GetHistoricalOrdersResponse {
  [field: string]: unknown;
  "orders": Array<PredictionsOrder>;
  "cursor": string;
}

export interface GetHistoricalPositionsParams {
  [field: string]: unknown;
  "ticker"?: string | null;
  "event_ticker"?: string | null;
  "subaccount"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetHistoricalPositionsResponse {
  [field: string]: unknown;
  "cursor"?: string | null;
  "market_positions": Array<PredictionsMarketPosition>;
  "event_positions": Array<PredictionsEventPosition>;
}

export interface GetIncentiveProgramsParams {
  [field: string]: unknown;
  "status"?: string | null;
  "type"?: string | null;
  "incentive_description"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetIncentiveProgramsResponse {
  [field: string]: unknown;
  "incentive_programs": Array<PredictionsIncentiveProgram>;
  "next_cursor"?: string | null;
}

export interface GetIntraExchangeInstanceTransferParams {
  [field: string]: unknown;
  "transfer_id": string;
}

export interface GetIntraExchangeInstanceTransferResponse {
  [field: string]: unknown;
  "transfer": PredictionsIntraExchangeInstanceTransfer;
}

export interface GetIntraExchangeInstanceTransfersParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetIntraExchangeInstanceTransfersResponse {
  [field: string]: unknown;
  "transfers": Array<PredictionsIntraExchangeInstanceTransfer>;
  "cursor"?: string | null;
}

export interface GetLiveDataByMilestoneParams {
  [field: string]: unknown;
  "milestone_id": string;
  "include_player_stats"?: boolean | null;
}

export interface GetLiveDataByMilestoneResponse {
  [field: string]: unknown;
  "live_data": PredictionsLiveData;
}

export interface GetLiveDataParams {
  [field: string]: unknown;
  "type": string;
  "milestone_id": string;
  "include_player_stats"?: boolean | null;
}

export interface GetLiveDataResponse {
  [field: string]: unknown;
  "live_data": PredictionsLiveData;
}

export interface GetLiveDatasParams {
  [field: string]: unknown;
  "milestone_ids": Array<string>;
  "include_player_stats"?: boolean | null;
}

export interface GetLiveDatasResponse {
  [field: string]: unknown;
  "live_datas": Array<PredictionsLiveData>;
}

export interface GetMarketCandlesticksByEventParams {
  [field: string]: unknown;
  "ticker": string;
  "series_ticker": string;
  "start_ts": number | bigint | string;
  "end_ts": number | bigint | string;
  "period_interval": number | bigint | string;
}

export interface GetMarketCandlesticksByEventResponse {
  [field: string]: unknown;
  "market_tickers": Array<string>;
  "market_candlesticks": Array<Array<PredictionsMarketCandlestick>>;
  "adjusted_end_ts": number | bigint | string;
}

export interface GetMarketCandlesticksHistoricalParams {
  [field: string]: unknown;
  "ticker": string;
  "start_ts": number | bigint | string;
  "end_ts": number | bigint | string;
  "period_interval": number | bigint | string;
}

export interface GetMarketCandlesticksHistoricalResponse {
  [field: string]: unknown;
  "ticker": string;
  "candlesticks": Array<PredictionsMarketCandlestickHistorical>;
}

export interface GetMarketCandlesticksParams {
  [field: string]: unknown;
  "series_ticker": string;
  "ticker": string;
  "start_ts": number | bigint | string;
  "end_ts": number | bigint | string;
  "period_interval": number | bigint | string;
  "include_latest_before_start"?: boolean | null;
}

export interface GetMarketCandlesticksResponse {
  [field: string]: unknown;
  "ticker": string;
  "candlesticks": Array<PredictionsMarketCandlestick>;
}

export interface GetMarketOrderbookParams {
  [field: string]: unknown;
  "ticker": string;
  "depth"?: number | bigint | string | null;
}

export interface GetMarketOrderbookResponse {
  [field: string]: unknown;
  "orderbook_fp": PredictionsOrderbookCountFp;
}

export interface GetMarketOrderbooksParams {
  [field: string]: unknown;
  "tickers": Array<string>;
}

export interface GetMarketOrderbooksResponse {
  [field: string]: unknown;
  "orderbooks": Array<PredictionsMarketOrderbookFp>;
}

export interface GetMarketParams {
  [field: string]: unknown;
  "ticker": string;
}

export interface GetMarketResponse {
  [field: string]: unknown;
  "market": PredictionsMarket;
}

export interface GetMarketsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "event_ticker"?: string | null;
  "series_ticker"?: string | null;
  "min_created_ts"?: number | bigint | string | null;
  "max_created_ts"?: number | bigint | string | null;
  "min_updated_ts"?: number | bigint | string | null;
  "max_updated_ts"?: number | bigint | string | null;
  "max_close_ts"?: number | bigint | string | null;
  "min_close_ts"?: number | bigint | string | null;
  "min_settled_ts"?: number | bigint | string | null;
  "max_settled_ts"?: number | bigint | string | null;
  "status"?: string | null;
  "tickers"?: string | null;
  "mve_filter"?: string | null;
}

export interface GetMarketsResponse {
  [field: string]: unknown;
  "markets": Array<PredictionsMarket>;
  "cursor": string;
}

export interface GetMilestoneParams {
  [field: string]: unknown;
  "milestone_id": string;
}

export interface GetMilestoneResponse {
  [field: string]: unknown;
  "milestone": PredictionsMilestone;
}

export interface GetMilestonesParams {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "minimum_start_date"?: string | null;
  "category"?: string | null;
  "competition"?: string | null;
  "source_id"?: string | null;
  "type"?: string | null;
  "related_event_ticker"?: string | null;
  "cursor"?: string | null;
  "min_updated_ts"?: number | bigint | string | null;
}

export interface GetMilestonesResponse {
  [field: string]: unknown;
  "milestones": Array<PredictionsMilestone>;
  "cursor"?: string | null;
}

export interface GetMultivariateEventCollectionParams {
  [field: string]: unknown;
  "collection_ticker": string;
}

export interface GetMultivariateEventCollectionResponse {
  [field: string]: unknown;
  "multivariate_contract": PredictionsMultivariateEventCollection;
}

export interface GetMultivariateEventCollectionsParams {
  [field: string]: unknown;
  "status"?: string | null;
  "associated_event_ticker"?: string | null;
  "series_ticker"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetMultivariateEventCollectionsResponse {
  [field: string]: unknown;
  "multivariate_contracts": Array<PredictionsMultivariateEventCollection>;
  "cursor"?: string | null;
}

export interface GetMultivariateEventsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "series_ticker"?: string | null;
  "collection_ticker"?: string | null;
  "tickers"?: string | null;
  "with_nested_markets"?: boolean | null;
}

export interface GetMultivariateEventsResponse {
  [field: string]: unknown;
  "events": Array<PredictionsEventData>;
  "cursor": string;
}

export interface GetOrderGroupParams {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount"?: number | bigint | string | null;
}

export interface GetOrderGroupResponse {
  [field: string]: unknown;
  "is_auto_cancel_enabled": boolean;
  "contracts_limit_fp"?: string | null;
  "orders": Array<string>;
  "exchange_index"?: number | bigint | string | null;
}

export interface GetOrderGroupsParams {
  [field: string]: unknown;
  "subaccount"?: number | bigint | string | null;
}

export interface GetOrderGroupsResponse {
  [field: string]: unknown;
  "order_groups"?: Array<PredictionsOrderGroup> | null;
}

export interface GetOrderParams {
  [field: string]: unknown;
  "order_id": string;
}

export interface GetOrderQueuePositionParams {
  [field: string]: unknown;
  "order_id": string;
}

export interface GetOrderQueuePositionResponse {
  [field: string]: unknown;
  "queue_position_fp": string;
}

export interface GetOrderQueuePositionsParams {
  [field: string]: unknown;
  "market_tickers"?: string | null;
  "event_ticker"?: string | null;
  "subaccount"?: number | bigint | string | null;
}

export interface GetOrderQueuePositionsResponse {
  [field: string]: unknown;
  "queue_positions": Array<PredictionsOrderQueuePosition>;
}

export interface GetOrderResponse {
  [field: string]: unknown;
  "order": PredictionsOrder;
}

export interface GetOrdersParams {
  [field: string]: unknown;
  "ticker"?: string | null;
  "event_ticker"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
  "status"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "subaccount"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface GetOrdersResponse {
  [field: string]: unknown;
  "orders": Array<PredictionsOrder>;
  "cursor": string;
}

export interface GetPortfolioRestingOrderTotalValueParams {
  [field: string]: unknown;
}

export interface GetPortfolioRestingOrderTotalValueResponse {
  [field: string]: unknown;
  "total_resting_order_value": number | bigint | string;
  "resting_order_value_breakdown": Array<PredictionsIndexedBalance>;
}

export interface GetPositionsParams {
  [field: string]: unknown;
  "cursor"?: string | null;
  "limit"?: number | bigint | string | null;
  "count_filter"?: string | null;
  "settlement_status"?: string | null;
  "ticker"?: string | null;
  "event_ticker"?: string | null;
  "subaccount"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface GetPositionsResponse {
  [field: string]: unknown;
  "cursor"?: string | null;
  "market_positions": Array<PredictionsMarketPosition>;
  "event_positions": Array<PredictionsEventPosition>;
}

export interface GetQuoteParams {
  [field: string]: unknown;
  "quote_id": string;
}

export interface GetQuoteResponse {
  [field: string]: unknown;
  "quote": PredictionsQuote;
}

export interface GetQuotesParams {
  [field: string]: unknown;
  "cursor"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "status"?: string | null;
  "quote_creator_user_id"?: string | null;
  "user_filter"?: string | null;
  "rfq_user_filter"?: string | null;
  "rfq_creator_user_id"?: string | null;
  "rfq_creator_subtrader_id"?: string | null;
  "rfq_id"?: string | null;
}

export interface GetQuotesResponse {
  [field: string]: unknown;
  "quotes": Array<PredictionsQuote>;
  "cursor"?: string | null;
}

export interface GetRfQsParams {
  [field: string]: unknown;
  "cursor"?: string | null;
  "event_ticker"?: string | null;
  "market_ticker"?: string | null;
  "subaccount"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "status"?: string | null;
  "creator_user_id"?: string | null;
  "user_filter"?: string | null;
}

export interface GetRfQsResponse {
  [field: string]: unknown;
  "rfqs": Array<PredictionsRFQ>;
  "cursor"?: string | null;
}

export interface GetRfqBatchBestQuotesParams {
  [field: string]: unknown;
  "rfq_batch_id": string;
}

export interface GetRfqBatchBestQuotesResponse {
  [field: string]: unknown;
  "best_quotes": PredictionsRFQBatchBestQuotes;
}

export interface GetRfqBatchParams {
  [field: string]: unknown;
  "rfq_batch_id": string;
}

export interface GetRfqBatchQuotesParams {
  [field: string]: unknown;
  "rfq_batch_id": string;
}

export interface GetRfqBatchQuotesResponse {
  [field: string]: unknown;
  "batch_quotes": Array<PredictionsRFQBatchQuote>;
}

export interface GetRfqBatchResponse {
  [field: string]: unknown;
  "rfq_batch": PredictionsRFQBatch;
}

export interface GetRfqBatchesParams {
  [field: string]: unknown;
  "cursor"?: string | null;
  "subaccount"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "status"?: string | null;
  "user_filter"?: string | null;
}

export interface GetRfqBatchesResponse {
  [field: string]: unknown;
  "rfq_batches": Array<PredictionsRFQBatch>;
  "cursor"?: string | null;
}

export interface GetRfqParams {
  [field: string]: unknown;
  "rfq_id": string;
}

export interface GetRfqQuoteParams {
  [field: string]: unknown;
  "rfq_id": string;
  "quote_id": string;
}

export interface GetRfqQuoteResponse {
  [field: string]: unknown;
  "quote": PredictionsQuote;
}

export interface GetRfqResponse {
  [field: string]: unknown;
  "rfq": PredictionsRFQ;
}

export interface GetSeriesFeeChangesParams {
  [field: string]: unknown;
  "series_ticker"?: string | null;
  "show_historical"?: boolean | null;
}

export interface GetSeriesFeeChangesResponse {
  [field: string]: unknown;
  "series_fee_change_arr": Array<PredictionsSeriesFeeChange>;
}

export interface GetSeriesListParams {
  [field: string]: unknown;
  "category"?: string | null;
  "tags"?: string | null;
  "include_product_metadata"?: boolean | null;
  "include_volume"?: boolean | null;
  "min_updated_ts"?: number | bigint | string | null;
}

export interface GetSeriesListResponse {
  [field: string]: unknown;
  "series": Array<PredictionsSeries>;
}

export interface GetSeriesParams {
  [field: string]: unknown;
  "series_ticker": string;
  "include_volume"?: boolean | null;
}

export interface GetSeriesResponse {
  [field: string]: unknown;
  "series": PredictionsSeries;
}

export interface GetSettlementsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "ticker"?: string | null;
  "event_ticker"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
  "subaccount"?: number | bigint | string | null;
}

export interface GetSettlementsResponse {
  [field: string]: unknown;
  "settlements": Array<PredictionsSettlement>;
  "cursor"?: string | null;
}

export interface GetStructuredTargetParams {
  [field: string]: unknown;
  "structured_target_id": string;
}

export interface GetStructuredTargetResponse {
  [field: string]: unknown;
  "structured_target"?: PredictionsStructuredTarget | null;
}

export interface GetStructuredTargetsParams {
  [field: string]: unknown;
  "ids"?: Array<string> | null;
  "type"?: string | null;
  "competition"?: string | null;
  "page_size"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetStructuredTargetsResponse {
  [field: string]: unknown;
  "structured_targets"?: Array<PredictionsStructuredTarget> | null;
  "cursor"?: string | null;
}

export interface GetSubaccountBalancesParams {
  [field: string]: unknown;
}

export interface GetSubaccountBalancesResponse {
  [field: string]: unknown;
  "subaccount_balances": Array<PredictionsSubaccountBalance>;
}

export interface GetSubaccountNettingParams {
  [field: string]: unknown;
}

export interface GetSubaccountNettingResponse {
  [field: string]: unknown;
  "netting_configs": Array<PredictionsSubaccountNettingConfig>;
}

export interface GetSubaccountTransfersParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetSubaccountTransfersResponse {
  [field: string]: unknown;
  "transfers": Array<PredictionsSubaccountTransfer>;
  "cursor"?: string | null;
}

export interface GetTagsForSeriesCategoriesParams {
  [field: string]: unknown;
}

export interface GetTagsForSeriesCategoriesResponse {
  [field: string]: unknown;
  "tags_by_categories": Record<string, Array<string>>;
}

export interface GetTargetBalanceAllocationParams {
  [field: string]: unknown;
}

export interface GetTargetBalanceAllocationResponse {
  [field: string]: unknown;
  "allocations": Array<PredictionsTargetBalanceAllocation>;
  "resting_margin_reservation": string;
}

export interface GetTradesHistoricalParams {
  [field: string]: unknown;
  "ticker"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "is_block_trade"?: boolean | null;
}

export interface GetTradesHistoricalResponse {
  [field: string]: unknown;
  "trades": Array<PredictionsTrade>;
  "cursor": string;
}

export interface GetTradesParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "ticker"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
  "is_block_trade"?: boolean | null;
}

export interface GetTradesResponse {
  [field: string]: unknown;
  "trades": Array<PredictionsTrade>;
  "cursor": string;
}

export interface GetUserDataTimestampParams {
  [field: string]: unknown;
}

export interface GetUserDataTimestampResponse {
  [field: string]: unknown;
  "as_of_time": string;
}

export interface GetWeatherIndexCalibrationsParams {
  [field: string]: unknown;
  "city": string;
}

export interface GetWeatherIndexCalibrationsResponse {
  [field: string]: unknown;
  "city": string;
  "units": string;
  "calibrations": Array<PredictionsWeatherIndexCalibration>;
}

export interface GetWeatherIndexParams {
  [field: string]: unknown;
  "city": string;
  "from"?: number | bigint | string | null;
  "to"?: number | bigint | string | null;
  "last_sec"?: number | bigint | string | null;
  "detailed"?: boolean | null;
}

export interface GetWeatherIndexResponse {
  [field: string]: unknown;
  "city": string;
  "config_version"?: string | null;
  "units": string;
  "timeseries": Array<PredictionsWeatherIndexPoint>;
}

export interface GetWithdrawalsParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface GetWithdrawalsResponse {
  [field: string]: unknown;
  "withdrawals": Array<PredictionsWithdrawal>;
  "cursor"?: string | null;
}

export interface IntraExchangeInstanceTransferParams {
  [field: string]: unknown;
  "body": PredictionsIntraExchangeInstanceTransferRequest;
}

export interface IntraExchangeInstanceTransferResponse {
  [field: string]: unknown;
  "transfer_id": string;
}

export interface ListFcmSubtradersParams {
  [field: string]: unknown;
}

export interface ListFcmSubtradersResponse {
  [field: string]: unknown;
  "subtraders": Array<PredictionsFCMSubtrader>;
}

export interface PredictionsAcceptBlockTradeProposalRequest {
  [field: string]: unknown;
  "subtrader_id"?: string | null;
  "subaccount"?: number | bigint | string | null;
}

export interface PredictionsAcceptQuoteRequest {
  [field: string]: unknown;
  "accepted_side": string;
}

export interface PredictionsAcceptRFQBatchRequest {
  [field: string]: unknown;
  "accepted_side": string;
  "min_bids": Array<PredictionsRFQBatchAcceptLimit>;
  "client_order_id"?: string | null;
}

export interface PredictionsAcceptRFQBatchResponse {
  [field: string]: unknown;
  "accepted_side": string;
  "quotes": Array<PredictionsQuote>;
}

export interface PredictionsAccountApiUsageLevelVolumeGoal {
  [field: string]: unknown;
  "level": string;
  "earn_volume_goal_fp": string;
  "keep_volume_goal_fp": string;
}

export interface PredictionsAccountApiUsageLevelVolumeProgress {
  [field: string]: unknown;
  "computed_ts": number | bigint | string;
  "trailing_30d_volume_fp": string;
  "goals": Array<PredictionsAccountApiUsageLevelVolumeGoal>;
}

export interface PredictionsAmendOrderRequest {
  [field: string]: unknown;
  "subaccount"?: number | bigint | string | null;
  "ticker": string;
  "side": string;
  "action": string;
  "client_order_id"?: string | null;
  "updated_client_order_id"?: string | null;
  "yes_price"?: number | bigint | string | null;
  "no_price"?: number | bigint | string | null;
  "yes_price_dollars"?: string | null;
  "no_price_dollars"?: string | null;
  "count"?: number | bigint | string | null;
  "count_fp"?: string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsAmendOrderResponse {
  [field: string]: unknown;
  "old_order": PredictionsOrder;
  "order": PredictionsOrder;
}

export interface PredictionsAmendOrderV2Request {
  [field: string]: unknown;
  "ticker": string;
  "side": string;
  "price": string;
  "count": string;
  "client_order_id"?: string | null;
  "updated_client_order_id"?: string | null;
  "expiration_time"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsAmendOrderV2Response {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "remaining_count"?: string | null;
  "fill_count"?: string | null;
  "average_fill_price"?: string | null;
  "average_fee_paid"?: string | null;
  "ts_ms": number | bigint | string;
}

export interface PredictionsApiKey {
  [field: string]: unknown;
  "api_key_id": string;
  "name": string;
  "scopes": Array<string>;
  "subaccount"?: number | bigint | string | null;
  "fcm_subtrader_id"?: string | null;
}

export interface PredictionsApiUsageLevelGrant {
  [field: string]: unknown;
  "exchange_instance": string;
  "level": string;
  "expires_ts"?: number | bigint | string | null;
  "source": string;
}

export interface PredictionsApplySubaccountTransferRequest {
  [field: string]: unknown;
  "client_transfer_id": string;
  "from_subaccount": number | bigint | string;
  "to_subaccount": number | bigint | string;
  "amount_cents": number | bigint | string;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsApplySubaccountTransferResponse {
  [field: string]: unknown;
}

export interface PredictionsAssociatedEvent {
  [field: string]: unknown;
  "ticker": string;
  "is_yes_only": boolean;
  "size_max"?: number | bigint | string | null;
  "size_min"?: number | bigint | string | null;
  "active_quoters": Array<string>;
}

export interface PredictionsAttachedEventContractExitTrigger {
  [field: string]: unknown;
  "slippage"?: PredictionsEventContractExitSlippage | null;
  "stop_loss_price"?: string | null;
  "take_profit_price"?: string | null;
}

export interface PredictionsBatchCancelOrdersIndividualResponse {
  [field: string]: unknown;
  "order_id": string;
  "order"?: PredictionsBatchCancelOrdersIndividualResponseOrder | null;
  "reduced_by_fp": string;
  "error"?: PredictionsBatchCancelOrdersIndividualResponseError | null;
}

export interface PredictionsBatchCancelOrdersIndividualResponseError {
  [field: string]: unknown;
  "code"?: string | null;
  "message"?: string | null;
  "details"?: string | null;
}

export interface PredictionsBatchCancelOrdersIndividualResponseOrder {
  [field: string]: unknown;
  "order_id": string;
  "user_id": string;
  "client_order_id": string;
  "ticker": string;
  "side"?: string | null;
  "action"?: string | null;
  "outcome_side": string;
  "book_side": string;
  "type": string;
  "status": string;
  "yes_price_dollars": string;
  "no_price_dollars": string;
  "fill_count_fp": string;
  "remaining_count_fp": string;
  "initial_count_fp": string;
  "taker_fill_cost_dollars": string;
  "maker_fill_cost_dollars": string;
  "taker_fees_dollars": string;
  "maker_fees_dollars": string;
  "expiration_time"?: string | null;
  "created_time"?: string | null;
  "last_update_time"?: string | null;
  "self_trade_prevention_type"?: string | null;
  "order_group_id"?: string | null;
  "cancel_order_on_pause"?: boolean | null;
  "subaccount_number"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsBatchCancelOrdersRequest {
  [field: string]: unknown;
  "ids"?: Array<string> | null;
  "orders"?: Array<PredictionsBatchCancelOrdersRequestOrder> | null;
}

export interface PredictionsBatchCancelOrdersRequestOrder {
  [field: string]: unknown;
  "order_id": string;
  "subaccount"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
  "market_ticker"?: string | null;
}

export interface PredictionsBatchCancelOrdersResponse {
  [field: string]: unknown;
  "orders": Array<PredictionsBatchCancelOrdersIndividualResponse>;
}

export interface PredictionsBatchCancelOrdersV2Request {
  [field: string]: unknown;
  "orders": Array<PredictionsBatchCancelOrdersV2RequestOrdersItem>;
}

export interface PredictionsBatchCancelOrdersV2RequestOrdersItem {
  [field: string]: unknown;
  "order_id": string;
  "subaccount"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
  "market_ticker"?: string | null;
}

export interface PredictionsBatchCancelOrdersV2Response {
  [field: string]: unknown;
  "orders": Array<PredictionsBatchCancelOrdersV2ResponseOrdersItem>;
}

export interface PredictionsBatchCancelOrdersV2ResponseOrdersItem {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "reduced_by": string;
  "ts_ms"?: number | bigint | string | null;
  "error"?: PredictionsBatchCancelOrdersV2ResponseOrdersItemError | null;
}

export interface PredictionsBatchCancelOrdersV2ResponseOrdersItemError {
  [field: string]: unknown;
  "code"?: string | null;
  "message"?: string | null;
  "details"?: string | null;
}

export interface PredictionsBatchCreateOrdersIndividualResponse {
  [field: string]: unknown;
  "client_order_id"?: string | null;
  "order"?: PredictionsBatchCreateOrdersIndividualResponseOrder | null;
  "error"?: PredictionsBatchCreateOrdersIndividualResponseError | null;
}

export interface PredictionsBatchCreateOrdersIndividualResponseError {
  [field: string]: unknown;
  "code"?: string | null;
  "message"?: string | null;
  "details"?: string | null;
}

export interface PredictionsBatchCreateOrdersIndividualResponseOrder {
  [field: string]: unknown;
  "order_id": string;
  "user_id": string;
  "client_order_id": string;
  "ticker": string;
  "side"?: string | null;
  "action"?: string | null;
  "outcome_side": string;
  "book_side": string;
  "type": string;
  "status": string;
  "yes_price_dollars": string;
  "no_price_dollars": string;
  "fill_count_fp": string;
  "remaining_count_fp": string;
  "initial_count_fp": string;
  "taker_fill_cost_dollars": string;
  "maker_fill_cost_dollars": string;
  "taker_fees_dollars": string;
  "maker_fees_dollars": string;
  "expiration_time"?: string | null;
  "created_time"?: string | null;
  "last_update_time"?: string | null;
  "self_trade_prevention_type"?: string | null;
  "order_group_id"?: string | null;
  "cancel_order_on_pause"?: boolean | null;
  "subaccount_number"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsBatchCreateOrdersRequest {
  [field: string]: unknown;
  "orders": Array<PredictionsCreateOrderRequest>;
}

export interface PredictionsBatchCreateOrdersResponse {
  [field: string]: unknown;
  "orders": Array<PredictionsBatchCreateOrdersIndividualResponse>;
}

export interface PredictionsBatchCreateOrdersV2Request {
  [field: string]: unknown;
  "orders": Array<PredictionsCreateOrderV2Request>;
}

export interface PredictionsBatchCreateOrdersV2Response {
  [field: string]: unknown;
  "orders": Array<PredictionsBatchCreateOrdersV2ResponseOrdersItem>;
}

export interface PredictionsBatchCreateOrdersV2ResponseOrdersItem {
  [field: string]: unknown;
  "order_id"?: string | null;
  "client_order_id"?: string | null;
  "fill_count"?: string | null;
  "remaining_count"?: string | null;
  "average_fill_price"?: string | null;
  "average_fee_paid"?: string | null;
  "ts_ms"?: number | bigint | string | null;
  "error"?: PredictionsBatchCreateOrdersV2ResponseOrdersItemError | null;
}

export interface PredictionsBatchCreateOrdersV2ResponseOrdersItemError {
  [field: string]: unknown;
  "code"?: string | null;
  "message"?: string | null;
  "details"?: string | null;
}

export interface PredictionsBatchGetMarketCandlesticksResponse {
  [field: string]: unknown;
  "markets": Array<PredictionsMarketCandlesticksResponse>;
}

export interface PredictionsBidAskDistribution {
  [field: string]: unknown;
  "open_dollars": string;
  "low_dollars": string;
  "high_dollars": string;
  "close_dollars": string;
}

export interface PredictionsBidAskDistributionHistorical {
  [field: string]: unknown;
  "open": string;
  "low": string;
  "high": string;
  "close": string;
}

export interface PredictionsBlockTradeProposal {
  [field: string]: unknown;
  "id": string;
  "proposer_user_id": string;
  "buyer_user_id": string;
  "buyer_subtrader_id"?: string | null;
  "seller_user_id": string;
  "seller_subtrader_id"?: string | null;
  "market_ticker": string;
  "price_centi_cents": number | bigint | string;
  "centicount": number | bigint | string;
  "maker_side": string;
  "expiration_ts": string;
  "status": string;
  "created_ts": string;
  "updated_ts": string;
  "buyer_accepted": boolean;
  "seller_accepted": boolean;
  "buyer_accepted_ts"?: string | null;
  "seller_accepted_ts"?: string | null;
  "executed_ts"?: string | null;
  "buyer_order_id"?: string | null;
  "seller_order_id"?: string | null;
}

export interface PredictionsBucketLimit {
  [field: string]: unknown;
  "refill_rate": number | bigint | string;
  "bucket_capacity": number | bigint | string;
}

export interface PredictionsCancelOrderV2Response {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "reduced_by": string;
  "ts_ms": number | bigint | string;
}

export interface PredictionsCreateApiKeyRequest {
  [field: string]: unknown;
  "name": string;
  "public_key": string;
  "scopes"?: Array<string> | null;
  "subaccount"?: number | bigint | string | null;
  "fcm_subtrader_id"?: string | null;
}

export interface PredictionsCreateApiKeyResponse {
  [field: string]: unknown;
  "api_key_id": string;
  "warning"?: string | null;
}

export interface PredictionsCreateFCMPositionTransferRequest {
  [field: string]: unknown;
  "source_subtrader_id": string;
  "target_subtrader_id": string;
  "market_ticker": string;
  "position_centicount": number | bigint | string;
  "price_centicents": number | bigint | string;
}

export interface PredictionsCreateFCMPositionTransferResponse {
  [field: string]: unknown;
  "position_transfer_id": string;
}

export interface PredictionsCreateFCMSubtraderRequest {
  [field: string]: unknown;
  "subtrader_suffix": string;
}

export interface PredictionsCreateFCMSubtraderResponse {
  [field: string]: unknown;
  "subtrader_id": string;
}

export interface PredictionsCreateMarketInMultivariateEventCollectionRequest {
  [field: string]: unknown;
  "selected_markets": Array<PredictionsTickerPair>;
  "with_market_payload"?: boolean | null;
}

export interface PredictionsCreateMarketInMultivariateEventCollectionResponse {
  [field: string]: unknown;
  "event_ticker": string;
  "market_ticker": string;
  "market"?: PredictionsMarket | null;
}

export interface PredictionsCreateOrderGroupRequest {
  [field: string]: unknown;
  "subaccount"?: number | bigint | string | null;
  "contracts_limit"?: number | bigint | string | null;
  "contracts_limit_fp"?: string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsCreateOrderGroupResponse {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount": number | bigint | string;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsCreateOrderRequest {
  [field: string]: unknown;
  "ticker": string;
  "client_order_id"?: string | null;
  "side": string;
  "action": string;
  "count"?: number | bigint | string | null;
  "count_fp"?: string | null;
  "yes_price"?: number | bigint | string | null;
  "no_price"?: number | bigint | string | null;
  "yes_price_dollars"?: string | null;
  "no_price_dollars"?: string | null;
  "expiration_ts"?: number | bigint | string | null;
  "time_in_force"?: string | null;
  "buy_max_cost"?: number | bigint | string | null;
  "post_only"?: boolean | null;
  "reduce_only"?: boolean | null;
  "sell_position_floor"?: number | bigint | string | null;
  "self_trade_prevention_type"?: string | null;
  "order_group_id"?: string | null;
  "cancel_order_on_pause"?: boolean | null;
  "subaccount"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsCreateOrderResponse {
  [field: string]: unknown;
  "order": PredictionsOrder;
}

export interface PredictionsCreateOrderV2Request {
  [field: string]: unknown;
  "ticker": string;
  "client_order_id"?: string | null;
  "side": string;
  "count": string;
  "price": string;
  "expiration_time"?: number | bigint | string | null;
  "time_in_force": string;
  "post_only"?: boolean | null;
  "self_trade_prevention_type": string;
  "cancel_order_on_pause"?: boolean | null;
  "reduce_only"?: boolean | null;
  "subaccount"?: number | bigint | string | null;
  "order_group_id"?: string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsCreateOrderV2Response {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "fill_count": string;
  "remaining_count": string;
  "average_fill_price"?: string | null;
  "average_fee_paid"?: string | null;
  "ts_ms": number | bigint | string;
}

export interface PredictionsCreateOrderWithExitTriggerRequest {
  [field: string]: unknown;
  "ticker": string;
  "client_order_id"?: string | null;
  "side": string;
  "count": string;
  "price": string;
  "expiration_time"?: number | bigint | string | null;
  "time_in_force": string;
  "post_only"?: boolean | null;
  "self_trade_prevention_type": string;
  "cancel_order_on_pause"?: boolean | null;
  "reduce_only"?: boolean | null;
  "subaccount"?: number | bigint | string | null;
  "order_group_id"?: string | null;
  "exchange_index"?: number | bigint | string | null;
  "exit_trigger": PredictionsAttachedEventContractExitTrigger;
}

export interface PredictionsCreateQuoteRequest {
  [field: string]: unknown;
  "rfq_id": string;
  "yes_bid": string;
  "no_bid": string;
  "rest_remainder": boolean;
  "post_only"?: boolean | null;
  "subaccount"?: number | bigint | string | null;
}

export interface PredictionsCreateQuoteResponse {
  [field: string]: unknown;
  "id": string;
}

export interface PredictionsCreateRFQBatchChild {
  [field: string]: unknown;
  "market_ticker"?: string | null;
  "selected_markets"?: Array<PredictionsTickerPair> | null;
  "contracts_fp"?: string | null;
  "target_cost_dollars"?: string | null;
}

export interface PredictionsCreateRFQBatchQuoteRequest {
  [field: string]: unknown;
  "quotes": Array<PredictionsRFQBatchQuoteEntry>;
  "rest_remainder": boolean;
  "post_only"?: boolean | null;
  "subaccount"?: number | bigint | string | null;
}

export interface PredictionsCreateRFQBatchQuoteResponse {
  [field: string]: unknown;
  "id": string;
}

export interface PredictionsCreateRFQBatchRequest {
  [field: string]: unknown;
  "children": Array<PredictionsCreateRFQBatchChild>;
  "mve_collection_ticker"?: string | null;
  "contracts_fp"?: string | null;
  "target_cost_dollars"?: string | null;
  "target_cost_excludes_fees"?: boolean | null;
  "rest_remainder"?: boolean | null;
  "replace_existing"?: boolean | null;
  "obscure_creator_id"?: boolean | null;
  "client_batch_id"?: string | null;
  "subtrader_id"?: string | null;
  "subaccount"?: number | bigint | string | null;
}

export interface PredictionsCreateRFQBatchResponse {
  [field: string]: unknown;
  "rfq_batch": PredictionsRFQBatch;
}

export interface PredictionsCreateRFQRequest {
  [field: string]: unknown;
  "market_ticker": string;
  "contracts"?: number | bigint | string | null;
  "contracts_fp"?: string | null;
  "target_cost_centi_cents"?: number | bigint | string | null;
  "target_cost_dollars"?: string | null;
  "target_cost_excludes_fees"?: boolean | null;
  "obscure_creator_id"?: boolean | null;
  "rest_remainder": boolean;
  "replace_existing"?: boolean | null;
  "subtrader_id"?: string | null;
  "subaccount"?: number | bigint | string | null;
}

export interface PredictionsCreateRFQResponse {
  [field: string]: unknown;
  "id": string;
}

export interface PredictionsCreateSubaccountRequest {
  [field: string]: unknown;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsCreateSubaccountResponse {
  [field: string]: unknown;
  "subaccount_number": number | bigint | string;
}

export interface PredictionsCreditHistory {
  [field: string]: unknown;
  "credit_id": string;
  "type": string;
  "status": string;
  "amount_dollars": string;
  "reason": string;
  "created_time": string;
}

export interface PredictionsDailySchedule {
  [field: string]: unknown;
  "open_time": string;
  "close_time": string;
}

export interface PredictionsDecreaseOrderV2Request {
  [field: string]: unknown;
  "reduce_by"?: string | null;
  "reduce_to"?: string | null;
  "exchange_index"?: number | bigint | string | null;
  "market_ticker"?: string | null;
}

export interface PredictionsDecreaseOrderV2Response {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "remaining_count": string;
  "ts_ms": number | bigint | string;
}

export interface PredictionsDeposit {
  [field: string]: unknown;
  "id": string;
  "status": string;
  "type": string;
  "amount_cents": number | bigint | string;
  "fee_cents": number | bigint | string;
  "created_ts": number | bigint | string;
  "finalized_ts"?: number | bigint | string | null;
}

export interface PredictionsEmptyResponse {
  [field: string]: unknown;
}

export interface PredictionsEndpointTokenCost {
  [field: string]: unknown;
  "method": string;
  "path": string;
  "cost": number | bigint | string;
}

export interface PredictionsErrorResponse {
  [field: string]: unknown;
  "code"?: string | null;
  "message"?: string | null;
  "details"?: string | null;
}

export interface PredictionsEventContractExitSlippage {
  [field: string]: unknown;
  "unit": string;
  "value": number | bigint | string;
}

export interface PredictionsEventContractExitTrigger {
  [field: string]: unknown;
  "slippage"?: PredictionsEventContractExitSlippage | null;
  "id": string;
  "ticker": string;
  "side": string;
  "subaccount_number": number | bigint | string;
  "kind": string;
  "stop_loss_price"?: string | null;
  "take_profit_price"?: string | null;
  "status": string;
  "status_reason"?: string | null;
  "triggered_order_id"?: string | null;
  "triggered_leg"?: string | null;
  "created_time": string;
  "updated_time": string;
}

export interface PredictionsEventData {
  [field: string]: unknown;
  "event_ticker": string;
  "series_ticker": string;
  "sub_title": string;
  "title": string;
  "collateral_return_type": string;
  "mutually_exclusive": boolean;
  "category"?: string | null;
  "strike_date"?: string | null;
  "strike_period"?: string | null;
  "markets"?: Array<PredictionsMarket> | null;
  "product_metadata"?: Record<string, HttpValue> | null;
  "settlement_sources": Array<PredictionsSettlementSource> | null;
  "last_updated_ts"?: string | null;
  "fee_type_override"?: string | null;
  "fee_multiplier_override"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsEventFeeChange {
  [field: string]: unknown;
  "id": string;
  "event_ticker": string;
  "series_ticker": string;
  "fee_type_override": string | null;
  "fee_multiplier_override": number | bigint | string | null;
  "scheduled_ts": string;
}

export interface PredictionsEventLiveData {
  [field: string]: unknown;
  "type": string;
  "details": Record<string, HttpValue>;
  "is_historical"?: boolean | null;
  "default_range"?: string | null;
  "range_options"?: Array<string> | null;
}

export interface PredictionsEventPosition {
  [field: string]: unknown;
  "event_ticker": string;
  "total_cost_dollars": string;
  "total_cost_shares_fp": string;
  "event_exposure_dollars": string;
  "realized_pnl_dollars": string;
  "fees_paid_dollars": string;
}

export interface PredictionsExchangeIndexStatus {
  [field: string]: unknown;
  "exchange_index": number | bigint | string;
  "description": string;
  "exchange_active": boolean;
  "trading_active": boolean;
  "intra_exchange_transfers_active": boolean;
}

export interface PredictionsExchangeStatus {
  [field: string]: unknown;
  "exchange_active": boolean;
  "trading_active": boolean;
  "intra_exchange_transfers_active"?: boolean | null;
  "exchange_estimated_resume_time"?: string | null;
  "exchange_index_statuses"?: Array<PredictionsExchangeIndexStatus> | null;
}

export interface PredictionsFCMSubtrader {
  [field: string]: unknown;
  "subtrader_id": string;
  "exchange_indices": Array<number | bigint | string>;
  "trading_blocked": boolean;
  "fcm_trading_blocked": boolean;
  "propagation_pending": boolean;
}

export interface PredictionsFcmFill {
  [field: string]: unknown;
  "fill_id": string;
  "exchange_index": number | bigint | string;
  "ticker": string;
  "taker_outcome_side": string;
  "count_fp": string;
  "yes_price_dollars": string;
  "created_time"?: string | null;
  "maker_order_id"?: string | null;
  "maker_subtrader_id"?: string | null;
  "maker_fee_cost"?: string | null;
  "taker_order_id"?: string | null;
  "taker_subtrader_id"?: string | null;
  "taker_fee_cost"?: string | null;
}

export interface PredictionsFill {
  [field: string]: unknown;
  "fill_id": string;
  "exchange_index": number | bigint | string;
  "trade_id": string;
  "order_id": string;
  "ticker": string;
  "market_ticker": string;
  "side"?: string | null;
  "action"?: string | null;
  "outcome_side": string;
  "book_side": string;
  "count_fp": string;
  "yes_price_dollars": string;
  "no_price_dollars": string;
  "is_taker": boolean;
  "created_time"?: string | null;
  "fee_cost": string;
  "subaccount_number"?: number | bigint | string | null;
  "ts"?: number | bigint | string | null;
}

export interface PredictionsForecastPercentilesPoint {
  [field: string]: unknown;
  "event_ticker": string;
  "end_period_ts": number | bigint | string;
  "period_interval": number | bigint | string;
  "percentile_points": Array<PredictionsPercentilePoint>;
}

export interface PredictionsGenerateApiKeyRequest {
  [field: string]: unknown;
  "name": string;
  "key_type"?: string | null;
  "scopes"?: Array<string> | null;
  "subaccount"?: number | bigint | string | null;
  "fcm_subtrader_id"?: string | null;
}

export interface PredictionsGenerateApiKeyResponse {
  [field: string]: unknown;
  "api_key_id": string;
  "key_type"?: string | null;
  "private_key": string;
  "warning"?: string | null;
}

export interface PredictionsGetAccountApiLimitsResponse {
  [field: string]: unknown;
  "usage_tier": string;
  "read": PredictionsBucketLimit;
  "write": PredictionsBucketLimit;
  "grants": Array<PredictionsApiUsageLevelGrant>;
}

export interface PredictionsGetAccountApiUsageLevelVolumeProgressResponse {
  [field: string]: unknown;
  "volume_progress": Array<PredictionsAccountApiUsageLevelVolumeProgress>;
}

export interface PredictionsGetAccountEndpointCostsResponse {
  [field: string]: unknown;
  "default_cost": number | bigint | string;
  "endpoint_costs": Array<PredictionsEndpointTokenCost>;
}

export interface PredictionsGetApiKeysResponse {
  [field: string]: unknown;
  "api_keys": Array<PredictionsApiKey>;
  "api_key_region_expiration_ts"?: number | bigint | string | null;
}

export interface PredictionsGetBalanceResponse {
  [field: string]: unknown;
  "balance": number | bigint | string;
  "balance_dollars": string;
  "portfolio_value": number | bigint | string;
  "updated_ts": number | bigint | string;
  "balance_breakdown"?: Array<PredictionsIndexedBalance> | null;
}

export interface PredictionsGetBlockTradeProposalsResponse {
  [field: string]: unknown;
  "block_trade_proposals": Array<PredictionsBlockTradeProposal>;
  "cursor"?: string | null;
}

export interface PredictionsGetCommunicationsIDResponse {
  [field: string]: unknown;
  "communications_id": string;
}

export interface PredictionsGetCreditHistoryResponse {
  [field: string]: unknown;
  "credits": Array<PredictionsCreditHistory>;
  "cursor": string;
}

export interface PredictionsGetDepositsResponse {
  [field: string]: unknown;
  "deposits": Array<PredictionsDeposit>;
  "cursor"?: string | null;
}

export interface PredictionsGetEventCandlesticksResponse {
  [field: string]: unknown;
  "market_tickers": Array<string>;
  "market_candlesticks": Array<Array<PredictionsMarketCandlestick>>;
  "adjusted_end_ts": number | bigint | string;
}

export interface PredictionsGetEventFeeChangesResponse {
  [field: string]: unknown;
  "event_fee_changes": Array<PredictionsEventFeeChange>;
  "cursor": string;
}

export interface PredictionsGetEventForecastPercentilesHistoryResponse {
  [field: string]: unknown;
  "forecast_history": Array<PredictionsForecastPercentilesPoint>;
}

export interface PredictionsGetEventLiveDataResponse {
  [field: string]: unknown;
  "live_data": PredictionsEventLiveData;
}

export interface PredictionsGetEventMetadataResponse {
  [field: string]: unknown;
  "image_url": string;
  "featured_image_url"?: string | null;
  "market_details": Array<PredictionsMarketMetadata>;
  "settlement_sources": Array<PredictionsSettlementSource>;
  "competition"?: string | null;
  "competition_scope"?: string | null;
}

export interface PredictionsGetEventResponse {
  [field: string]: unknown;
  "event": PredictionsEventData;
  "markets": Array<PredictionsMarket>;
}

export interface PredictionsGetEventsResponse {
  [field: string]: unknown;
  "events": Array<PredictionsEventData>;
  "milestones"?: Array<PredictionsMilestone> | null;
  "cursor": string;
}

export interface PredictionsGetExchangeScheduleResponse {
  [field: string]: unknown;
  "schedule": PredictionsSchedule;
}

export interface PredictionsGetFCMEventContractDailyCapResponse {
  [field: string]: unknown;
  "subtrader_id": string;
  "limit": string;
  "executed_utilization": string;
  "resting_order_utilization": string;
  "pending_order_utilization": string;
  "cap_date": string;
}

export interface PredictionsGetFCMSubtraderBlockedCategoriesResponse {
  [field: string]: unknown;
  "categories": Array<string>;
}

export interface PredictionsGetFcmFillsResponse {
  [field: string]: unknown;
  "fills": Array<PredictionsFcmFill>;
  "cursor": string;
}

export interface PredictionsGetFillsResponse {
  [field: string]: unknown;
  "fills": Array<PredictionsFill>;
  "cursor": string;
}

export interface PredictionsGetFiltersBySportsResponse {
  [field: string]: unknown;
  "filters_by_sports": Record<string, PredictionsSportFilterDetails>;
  "sport_ordering": Array<string>;
}

export interface PredictionsGetGameStatsResponse {
  [field: string]: unknown;
  "pbp"?: PredictionsPlayByPlay | null;
}

export interface PredictionsGetHistoricalCutoffResponse {
  [field: string]: unknown;
  "market_settled_ts": string;
  "trades_created_ts": string;
  "orders_updated_ts": string;
  "market_positions_last_updated_ts"?: string | null;
}

export interface PredictionsGetIncentiveProgramsResponse {
  [field: string]: unknown;
  "incentive_programs": Array<PredictionsIncentiveProgram>;
  "next_cursor"?: string | null;
}

export interface PredictionsGetIntraExchangeInstanceTransferResponse {
  [field: string]: unknown;
  "transfer": PredictionsIntraExchangeInstanceTransfer;
}

export interface PredictionsGetIntraExchangeInstanceTransfersResponse {
  [field: string]: unknown;
  "transfers": Array<PredictionsIntraExchangeInstanceTransfer>;
  "cursor"?: string | null;
}

export interface PredictionsGetLiveDataResponse {
  [field: string]: unknown;
  "live_data": PredictionsLiveData;
}

export interface PredictionsGetLiveDatasResponse {
  [field: string]: unknown;
  "live_datas": Array<PredictionsLiveData>;
}

export interface PredictionsGetMarketCandlesticksHistoricalResponse {
  [field: string]: unknown;
  "ticker": string;
  "candlesticks": Array<PredictionsMarketCandlestickHistorical>;
}

export interface PredictionsGetMarketCandlesticksResponse {
  [field: string]: unknown;
  "ticker": string;
  "candlesticks": Array<PredictionsMarketCandlestick>;
}

export interface PredictionsGetMarketIndexHistoryResponse {
  [field: string]: unknown;
  "id": string;
  "source": string;
  "status": string;
  "last_tick_ms"?: number | bigint | string | null;
  "interval": string;
  "start_ms": number | bigint | string;
  "end_ms": number | bigint | string;
  "timeseries": Array<PredictionsMarketIndexTimeseriesPoint>;
}

export interface PredictionsGetMarketIndexResponse {
  [field: string]: unknown;
  "index": PredictionsMarketIndex;
  "source": string;
  "status": string;
  "value"?: number | bigint | string | null;
  "last_tick_ms"?: number | bigint | string | null;
  "interval": string;
  "start_ms": number | bigint | string;
  "end_ms": number | bigint | string;
  "timeseries": Array<PredictionsMarketIndexTimeseriesPoint>;
}

export interface PredictionsGetMarketIndexesResponse {
  [field: string]: unknown;
  "indexes": Array<PredictionsMarketIndex>;
}

export interface PredictionsGetMarketOrderbookResponse {
  [field: string]: unknown;
  "orderbook_fp": PredictionsOrderbookCountFp;
}

export interface PredictionsGetMarketOrderbooksResponse {
  [field: string]: unknown;
  "orderbooks": Array<PredictionsMarketOrderbookFp>;
}

export interface PredictionsGetMarketResponse {
  [field: string]: unknown;
  "market": PredictionsMarket;
}

export interface PredictionsGetMarketsResponse {
  [field: string]: unknown;
  "markets": Array<PredictionsMarket>;
  "cursor": string;
}

export interface PredictionsGetMilestoneResponse {
  [field: string]: unknown;
  "milestone": PredictionsMilestone;
}

export interface PredictionsGetMilestonesResponse {
  [field: string]: unknown;
  "milestones": Array<PredictionsMilestone>;
  "cursor"?: string | null;
}

export interface PredictionsGetMultivariateEventCollectionResponse {
  [field: string]: unknown;
  "multivariate_contract": PredictionsMultivariateEventCollection;
}

export interface PredictionsGetMultivariateEventCollectionsResponse {
  [field: string]: unknown;
  "multivariate_contracts": Array<PredictionsMultivariateEventCollection>;
  "cursor"?: string | null;
}

export interface PredictionsGetMultivariateEventsResponse {
  [field: string]: unknown;
  "events": Array<PredictionsEventData>;
  "cursor": string;
}

export interface PredictionsGetOrderGroupResponse {
  [field: string]: unknown;
  "is_auto_cancel_enabled": boolean;
  "contracts_limit_fp"?: string | null;
  "orders": Array<string>;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsGetOrderGroupsResponse {
  [field: string]: unknown;
  "order_groups"?: Array<PredictionsOrderGroup> | null;
}

export interface PredictionsGetOrderQueuePositionResponse {
  [field: string]: unknown;
  "queue_position_fp": string;
}

export interface PredictionsGetOrderQueuePositionsResponse {
  [field: string]: unknown;
  "queue_positions": Array<PredictionsOrderQueuePosition>;
}

export interface PredictionsGetOrderResponse {
  [field: string]: unknown;
  "order": PredictionsOrder;
}

export interface PredictionsGetOrdersResponse {
  [field: string]: unknown;
  "orders": Array<PredictionsOrder>;
  "cursor": string;
}

export interface PredictionsGetPortfolioRestingOrderTotalValueResponse {
  [field: string]: unknown;
  "total_resting_order_value": number | bigint | string;
  "resting_order_value_breakdown": Array<PredictionsIndexedBalance>;
}

export interface PredictionsGetPositionsResponse {
  [field: string]: unknown;
  "cursor"?: string | null;
  "market_positions": Array<PredictionsMarketPosition>;
  "event_positions": Array<PredictionsEventPosition>;
}

export interface PredictionsGetQuoteResponse {
  [field: string]: unknown;
  "quote": PredictionsQuote;
}

export interface PredictionsGetQuotesResponse {
  [field: string]: unknown;
  "quotes": Array<PredictionsQuote>;
  "cursor"?: string | null;
}

export interface PredictionsGetRFQBatchBestQuotesResponse {
  [field: string]: unknown;
  "best_quotes": PredictionsRFQBatchBestQuotes;
}

export interface PredictionsGetRFQBatchQuotesResponse {
  [field: string]: unknown;
  "batch_quotes": Array<PredictionsRFQBatchQuote>;
}

export interface PredictionsGetRFQBatchResponse {
  [field: string]: unknown;
  "rfq_batch": PredictionsRFQBatch;
}

export interface PredictionsGetRFQBatchesResponse {
  [field: string]: unknown;
  "rfq_batches": Array<PredictionsRFQBatch>;
  "cursor"?: string | null;
}

export interface PredictionsGetRFQResponse {
  [field: string]: unknown;
  "rfq": PredictionsRFQ;
}

export interface PredictionsGetRFQsResponse {
  [field: string]: unknown;
  "rfqs": Array<PredictionsRFQ>;
  "cursor"?: string | null;
}

export interface PredictionsGetSeriesFeeChangesResponse {
  [field: string]: unknown;
  "series_fee_change_arr": Array<PredictionsSeriesFeeChange>;
}

export interface PredictionsGetSeriesListResponse {
  [field: string]: unknown;
  "series": Array<PredictionsSeries>;
}

export interface PredictionsGetSeriesResponse {
  [field: string]: unknown;
  "series": PredictionsSeries;
}

export interface PredictionsGetSettlementsResponse {
  [field: string]: unknown;
  "settlements": Array<PredictionsSettlement>;
  "cursor"?: string | null;
}

export interface PredictionsGetStructuredTargetResponse {
  [field: string]: unknown;
  "structured_target"?: PredictionsStructuredTarget | null;
}

export interface PredictionsGetStructuredTargetsResponse {
  [field: string]: unknown;
  "structured_targets"?: Array<PredictionsStructuredTarget> | null;
  "cursor"?: string | null;
}

export interface PredictionsGetSubaccountBalancesResponse {
  [field: string]: unknown;
  "subaccount_balances": Array<PredictionsSubaccountBalance>;
}

export interface PredictionsGetSubaccountNettingResponse {
  [field: string]: unknown;
  "netting_configs": Array<PredictionsSubaccountNettingConfig>;
}

export interface PredictionsGetSubaccountTransfersResponse {
  [field: string]: unknown;
  "transfers": Array<PredictionsSubaccountTransfer>;
  "cursor"?: string | null;
}

export interface PredictionsGetTagsForSeriesCategoriesResponse {
  [field: string]: unknown;
  "tags_by_categories": Record<string, Array<string>>;
}

export interface PredictionsGetTargetBalanceAllocationResponse {
  [field: string]: unknown;
  "allocations": Array<PredictionsTargetBalanceAllocation>;
  "resting_margin_reservation": string;
}

export interface PredictionsGetTradesResponse {
  [field: string]: unknown;
  "trades": Array<PredictionsTrade>;
  "cursor": string;
}

export interface PredictionsGetUserDataTimestampResponse {
  [field: string]: unknown;
  "as_of_time": string;
}

export interface PredictionsGetWeatherIndexCalibrationsResponse {
  [field: string]: unknown;
  "city": string;
  "units": string;
  "calibrations": Array<PredictionsWeatherIndexCalibration>;
}

export interface PredictionsGetWeatherIndexResponse {
  [field: string]: unknown;
  "city": string;
  "config_version"?: string | null;
  "units": string;
  "timeseries": Array<PredictionsWeatherIndexPoint>;
}

export interface PredictionsGetWithdrawalsResponse {
  [field: string]: unknown;
  "withdrawals": Array<PredictionsWithdrawal>;
  "cursor"?: string | null;
}

export interface PredictionsIncentiveProgram {
  [field: string]: unknown;
  "id": string;
  "market_id": string;
  "market_ticker": string;
  "incentive_type": string;
  "incentive_description": string;
  "start_date": string;
  "end_date": string;
  "period_reward": number | bigint | string;
  "paid_out": boolean;
  "discount_factor_bps"?: number | bigint | string | null;
  "target_size_fp"?: string | null;
  "max_reward_per_account"?: number | bigint | string | null;
}

export interface PredictionsIndexedBalance {
  [field: string]: unknown;
  "exchange_index": number | bigint | string;
  "balance": string;
}

export interface PredictionsIntraExchangeInstanceTransfer {
  [field: string]: unknown;
  "transfer_id": string;
  "client_transfer_id"?: string | null;
  "source": string;
  "destination": string;
  "source_exchange_shard": number | bigint | string;
  "destination_exchange_shard": number | bigint | string;
  "amount": string;
  "status": string;
  "created_ts": number | bigint | string;
}

export interface PredictionsIntraExchangeInstanceTransferRequest {
  [field: string]: unknown;
  "client_transfer_id"?: string | null;
  "source": string;
  "destination": string;
  "amount": number | bigint | string;
  "source_exchange_shard"?: number | bigint | string | null;
  "destination_exchange_shard"?: number | bigint | string | null;
  "source_subaccount"?: number | bigint | string | null;
  "destination_subaccount"?: number | bigint | string | null;
}

export interface PredictionsIntraExchangeInstanceTransferResponse {
  [field: string]: unknown;
  "transfer_id": string;
}

export interface PredictionsListFCMSubtradersResponse {
  [field: string]: unknown;
  "subtraders": Array<PredictionsFCMSubtrader>;
}

export interface PredictionsLiveData {
  [field: string]: unknown;
  "type": string;
  "details": Record<string, HttpValue>;
  "milestone_id": string;
}

export interface PredictionsMaintenanceWindow {
  [field: string]: unknown;
  "start_datetime": string;
  "end_datetime": string;
}

export interface PredictionsMarket {
  [field: string]: unknown;
  "ticker": string;
  "event_ticker": string;
  "market_type": string;
  "title"?: string | null;
  "subtitle"?: string | null;
  "yes_sub_title": string;
  "no_sub_title": string;
  "created_time": string;
  "updated_time": string;
  "open_time": string;
  "close_time": string;
  "expected_expiration_time"?: string | null;
  "expiration_time"?: string | null;
  "latest_expiration_time": string;
  "settlement_timer_seconds": number | bigint | string;
  "status": string;
  "yes_bid_dollars": string;
  "yes_bid_size_fp": string;
  "yes_ask_dollars": string;
  "yes_ask_size_fp": string;
  "no_bid_dollars": string;
  "no_ask_dollars": string;
  "last_price_dollars": string;
  "volume_fp": string;
  "volume_24h_fp": string;
  "result": string;
  "can_close_early": boolean;
  "open_interest_fp": string;
  "notional_value_dollars": string;
  "previous_yes_bid_dollars": string;
  "previous_yes_ask_dollars": string;
  "previous_price_dollars": string;
  "settlement_value_dollars"?: string | null;
  "settlement_ts"?: string | null;
  "settlement_bounds_type": string;
  "settlement_floor_dollars"?: string | null;
  "expiration_value": string;
  "occurrence_datetime"?: string | null;
  "fee_waiver_expiration_time"?: string | null;
  "early_close_condition"?: string | null;
  "strike_type"?: string | null;
  "floor_strike"?: number | bigint | string | null;
  "cap_strike"?: number | bigint | string | null;
  "functional_strike"?: string | null;
  "custom_strike"?: Record<string, HttpValue> | null;
  "rules_primary": string;
  "rules_secondary": string;
  "mve_collection_ticker"?: string | null;
  "mve_selected_legs"?: Array<PredictionsMveSelectedLeg> | null;
  "primary_participant_key"?: string | null;
  "price_level_structure": string;
  "price_ranges": Array<PredictionsPriceRange>;
  "is_provisional"?: boolean | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsMarketCandlestick {
  [field: string]: unknown;
  "end_period_ts": number | bigint | string;
  "yes_bid": PredictionsBidAskDistribution;
  "yes_ask": PredictionsBidAskDistribution;
  "price": PredictionsPriceDistribution;
  "volume_fp": string;
  "open_interest_fp": string;
}

export interface PredictionsMarketCandlestickHistorical {
  [field: string]: unknown;
  "end_period_ts": number | bigint | string;
  "yes_bid": PredictionsBidAskDistributionHistorical;
  "yes_ask": PredictionsBidAskDistributionHistorical;
  "price": PredictionsPriceDistributionHistorical;
  "volume": string;
  "open_interest": string;
}

export interface PredictionsMarketCandlesticksResponse {
  [field: string]: unknown;
  "market_ticker": string;
  "candlesticks": Array<PredictionsMarketCandlestick>;
}

export interface PredictionsMarketIndex {
  [field: string]: unknown;
  "id": string;
  "name": string;
  "family": string;
  "gics_code"?: string | null;
  "sector"?: string | null;
  "currency": string;
  "units": string;
  "session": PredictionsMarketIndexSession;
  "available": boolean;
  "latest"?: PredictionsMarketIndexLatest | null;
}

export interface PredictionsMarketIndexLatest {
  [field: string]: unknown;
  "source": string;
  "status": string;
  "value"?: number | bigint | string | null;
  "last_tick_ms"?: number | bigint | string | null;
}

export interface PredictionsMarketIndexSession {
  [field: string]: unknown;
  "timezone": string;
  "open": string;
  "close": string;
  "days": Array<string>;
  "calendar"?: string | null;
}

export interface PredictionsMarketIndexTimeseriesPoint {
  [field: string]: unknown;
  "t": number | bigint | string;
  "v": number | bigint | string;
}

export interface PredictionsMarketMetadata {
  [field: string]: unknown;
  "market_ticker": string;
  "image_url": string;
  "color_code": string;
}

export interface PredictionsMarketOrderbookFp {
  [field: string]: unknown;
  "ticker": string;
  "orderbook_fp": PredictionsOrderbookCountFp;
}

export interface PredictionsMarketPosition {
  [field: string]: unknown;
  "ticker": string;
  "exchange_index": number | bigint | string;
  "total_traded_dollars": string;
  "position_fp": string;
  "market_exposure_dollars": string;
  "realized_pnl_dollars": string;
  "fees_paid_dollars": string;
  "last_updated_ts": string;
  "netting_enabled": boolean;
}

export interface PredictionsMilestone {
  [field: string]: unknown;
  "id": string;
  "category": string;
  "type": string;
  "start_date": string;
  "end_date"?: string | null;
  "related_event_tickers": Array<string>;
  "title": string;
  "notification_message": string;
  "source_id"?: string | null;
  "source_ids"?: Record<string, string> | null;
  "details": Record<string, HttpValue>;
  "primary_event_tickers": Array<string>;
  "last_updated_ts": string;
}

export interface PredictionsMultivariateEventCollection {
  [field: string]: unknown;
  "collection_ticker": string;
  "series_ticker": string;
  "exchange_index"?: number | bigint | string | null;
  "title": string;
  "description": string;
  "open_date": string;
  "close_date": string;
  "associated_events": Array<PredictionsAssociatedEvent>;
  "associated_event_tickers": Array<string>;
  "is_ordered": boolean;
  "is_single_market_per_event": boolean;
  "is_all_yes": boolean;
  "size_min": number | bigint | string;
  "size_max": number | bigint | string;
  "functional_description": string;
  "price_level_structure": string;
  "price_ranges": Array<PredictionsPriceRange>;
}

export interface PredictionsMveSelectedLeg {
  [field: string]: unknown;
  "event_ticker"?: string | null;
  "market_ticker"?: string | null;
  "side"?: string | null;
  "yes_settlement_value_dollars"?: string | null;
}

export interface PredictionsOrder {
  [field: string]: unknown;
  "order_id": string;
  "user_id": string;
  "client_order_id": string;
  "ticker": string;
  "side"?: string | null;
  "action"?: string | null;
  "outcome_side": string;
  "book_side": string;
  "type": string;
  "status": string;
  "yes_price_dollars": string;
  "no_price_dollars": string;
  "fill_count_fp": string;
  "remaining_count_fp": string;
  "initial_count_fp": string;
  "taker_fill_cost_dollars": string;
  "maker_fill_cost_dollars": string;
  "taker_fees_dollars": string;
  "maker_fees_dollars": string;
  "expiration_time"?: string | null;
  "created_time"?: string | null;
  "last_update_time"?: string | null;
  "self_trade_prevention_type"?: string | null;
  "order_group_id"?: string | null;
  "cancel_order_on_pause"?: boolean | null;
  "subaccount_number"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsOrderGroup {
  [field: string]: unknown;
  "id": string;
  "contracts_limit_fp"?: string | null;
  "is_auto_cancel_enabled": boolean;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsOrderQueuePosition {
  [field: string]: unknown;
  "order_id": string;
  "market_ticker": string;
  "queue_position_fp": string;
}

export interface PredictionsOrderbookCountFp {
  [field: string]: unknown;
  "yes_dollars": Array<PredictionsPriceLevelDollarsCountFp>;
  "no_dollars": Array<PredictionsPriceLevelDollarsCountFp>;
}

export interface PredictionsPercentilePoint {
  [field: string]: unknown;
  "percentile": number | bigint | string;
  "raw_numerical_forecast": number | bigint | string;
  "numerical_forecast": number | bigint | string;
  "formatted_forecast": string;
}

export interface PredictionsPlayByPlay {
  [field: string]: unknown;
  "periods"?: Array<PredictionsPlayByPlayPeriodsItem> | null;
}

export interface PredictionsPlayByPlayPeriodsItem {
  [field: string]: unknown;
  "events"?: Array<Record<string, HttpValue>> | null;
}

export interface PredictionsPriceDistribution {
  [field: string]: unknown;
  "open_dollars"?: string | null;
  "low_dollars"?: string | null;
  "high_dollars"?: string | null;
  "close_dollars"?: string | null;
  "mean_dollars"?: string | null;
  "previous_dollars"?: string | null;
  "min_dollars"?: string | null;
  "max_dollars"?: string | null;
}

export interface PredictionsPriceDistributionHistorical {
  [field: string]: unknown;
  "open": string | null;
  "low": string | null;
  "high": string | null;
  "close": string | null;
  "mean": string | null;
  "previous": string | null;
}

export interface PredictionsPriceRange {
  [field: string]: unknown;
  "start": string;
  "end": string;
  "step": string;
}

export interface PredictionsProposeBlockTradeRequest {
  [field: string]: unknown;
  "buyer_user_id": string;
  "buyer_subtrader_id"?: string | null;
  "buyer_subaccount"?: number | bigint | string | null;
  "seller_user_id": string;
  "seller_subtrader_id"?: string | null;
  "seller_subaccount"?: number | bigint | string | null;
  "market_ticker": string;
  "price_centi_cents": number | bigint | string;
  "centicount": number | bigint | string;
  "maker_side": string;
  "expiration_ts": string;
}

export interface PredictionsProposeBlockTradeResponse {
  [field: string]: unknown;
  "block_trade_proposal_id": string;
}

export interface PredictionsQuote {
  [field: string]: unknown;
  "id": string;
  "rfq_id": string;
  "creator_id": string;
  "rfq_creator_id": string;
  "market_ticker": string;
  "contracts_fp": string;
  "yes_bid_dollars": string;
  "no_bid_dollars": string;
  "created_ts": string;
  "updated_ts": string;
  "status": string;
  "accepted_side"?: string | null;
  "accepted_ts"?: string | null;
  "confirmed_ts"?: string | null;
  "executed_ts"?: string | null;
  "cancelled_ts"?: string | null;
  "rest_remainder"?: boolean | null;
  "post_only"?: boolean | null;
  "cancellation_reason"?: string | null;
  "creator_user_id"?: string | null;
  "rfq_creator_user_id"?: string | null;
  "rfq_target_cost_dollars"?: string | null;
  "target_cost_excludes_fees"?: boolean | null;
  "rfq_creator_order_id"?: string | null;
  "creator_order_id"?: string | null;
  "creator_subaccount"?: number | bigint | string | null;
  "rfq_creator_subaccount"?: number | bigint | string | null;
  "yes_contracts_fp"?: string | null;
  "no_contracts_fp"?: string | null;
}

export interface PredictionsRFQ {
  [field: string]: unknown;
  "id": string;
  "creator_id": string;
  "market_ticker": string;
  "contracts_fp": string;
  "target_cost_dollars"?: string | null;
  "target_cost_excludes_fees"?: boolean | null;
  "status": string;
  "created_ts": string;
  "mve_collection_ticker"?: string | null;
  "mve_selected_legs"?: Array<PredictionsMveSelectedLeg> | null;
  "rest_remainder"?: boolean | null;
  "cancellation_reason"?: string | null;
  "creator_user_id"?: string | null;
  "creator_subaccount"?: number | bigint | string | null;
  "cancelled_ts"?: string | null;
  "updated_ts"?: string | null;
  "rfq_batch_id"?: string | null;
}

export interface PredictionsRFQBatch {
  [field: string]: unknown;
  "id": string;
  "creator_id": string;
  "status": string;
  "created_ts": string;
  "accepted_ts"?: string | null;
  "closed_ts"?: string | null;
  "rfqs": Array<PredictionsRFQ>;
  "updated_ts"?: string | null;
  "close_reason"?: string | null;
  "client_batch_id"?: string | null;
  "creator_user_id"?: string | null;
  "creator_subaccount"?: number | bigint | string | null;
}

export interface PredictionsRFQBatchAcceptLimit {
  [field: string]: unknown;
  "rfq_id": string;
  "bid_dollars": string;
}

export interface PredictionsRFQBatchBestQuote {
  [field: string]: unknown;
  "rfq_id": string;
  "market_ticker": string;
  "yes_bid_dollars"?: string | null;
  "no_bid_dollars"?: string | null;
  "yes_contracts_offered_fp"?: string | null;
  "no_contracts_offered_fp"?: string | null;
}

export interface PredictionsRFQBatchBestQuotes {
  [field: string]: unknown;
  "rfq_batch_id": string;
  "status": string;
  "version": number | bigint | string;
  "updated_ts"?: string | null;
  "yes_complete": boolean;
  "no_complete": boolean;
  "rfqs": Array<PredictionsRFQBatchBestQuote>;
}

export interface PredictionsRFQBatchQuote {
  [field: string]: unknown;
  "id": string;
  "rfq_batch_id": string;
  "creator_id": string;
  "rfq_creator_id": string;
  "status": string;
  "created_ts": string;
  "updated_ts": string;
  "accepted_side"?: string | null;
  "accepted_ts"?: string | null;
  "confirmed_ts"?: string | null;
  "quotes": Array<PredictionsQuote>;
  "rest_remainder"?: boolean | null;
  "post_only"?: boolean | null;
  "creator_user_id"?: string | null;
  "creator_subaccount"?: number | bigint | string | null;
  "rfq_creator_user_id"?: string | null;
  "rfq_creator_subaccount"?: number | bigint | string | null;
}

export interface PredictionsRFQBatchQuoteEntry {
  [field: string]: unknown;
  "rfq_id": string;
  "yes_bid": string;
  "no_bid": string;
}

export interface PredictionsSchedule {
  [field: string]: unknown;
  "standard_hours": Array<PredictionsWeeklySchedule>;
  "maintenance_windows": Array<PredictionsMaintenanceWindow>;
}

export interface PredictionsScopeList {
  [field: string]: unknown;
  "scopes": Array<string>;
}

export interface PredictionsSeries {
  [field: string]: unknown;
  "ticker": string;
  "frequency": string;
  "title": string;
  "category": string;
  "categories": Array<string>;
  "tags": Array<string> | null;
  "settlement_sources": Array<PredictionsSettlementSource> | null;
  "contract_url": string;
  "contract_terms_url": string;
  "product_metadata"?: Record<string, HttpValue> | null;
  "fee_type": string;
  "fee_multiplier": number | bigint | string;
  "additional_prohibitions": Array<string> | null;
  "volume_fp"?: string | null;
  "last_updated_ts"?: string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface PredictionsSeriesFeeChange {
  [field: string]: unknown;
  "id": string;
  "series_ticker": string;
  "fee_type": string;
  "fee_multiplier": number | bigint | string;
  "scheduled_ts": string;
}

export interface PredictionsSetEventContractExitTriggerRequest {
  [field: string]: unknown;
  "slippage"?: PredictionsEventContractExitSlippage | null;
  "kind"?: string | null;
  "stop_loss_price"?: string | null;
  "take_profit_price"?: string | null;
}

export interface PredictionsSetTargetBalanceAllocationRequest {
  [field: string]: unknown;
  "allocations": Array<PredictionsTargetBalanceAllocationInput>;
  "resting_margin_reservation"?: string | null;
}

export interface PredictionsSettlement {
  [field: string]: unknown;
  "ticker": string;
  "exchange_index": number | bigint | string;
  "event_ticker": string;
  "market_result": string;
  "yes_count_fp": string;
  "yes_total_cost_dollars": string;
  "no_count_fp": string;
  "no_total_cost_dollars": string;
  "revenue": number | bigint | string;
  "settled_time": string;
  "fee_cost": string;
  "value"?: number | bigint | string | null;
}

export interface PredictionsSettlementSource {
  [field: string]: unknown;
  "name"?: string | null;
  "url"?: string | null;
}

export interface PredictionsSportFilterDetails {
  [field: string]: unknown;
  "scopes": Array<string>;
  "competitions": Record<string, PredictionsScopeList>;
}

export interface PredictionsStructuredTarget {
  [field: string]: unknown;
  "id"?: string | null;
  "name"?: string | null;
  "type"?: string | null;
  "details"?: Record<string, HttpValue> | null;
  "source_id"?: string | null;
  "source_ids"?: Record<string, string> | null;
  "last_updated_ts"?: string | null;
}

export interface PredictionsSubaccountBalance {
  [field: string]: unknown;
  "subaccount_number": number | bigint | string;
  "exchange_index": number | bigint | string;
  "balance": string;
  "updated_ts": number | bigint | string;
}

export interface PredictionsSubaccountNettingConfig {
  [field: string]: unknown;
  "subaccount_number": number | bigint | string;
  "enabled": boolean;
  "exchange_index": number | bigint | string;
}

export interface PredictionsSubaccountTransfer {
  [field: string]: unknown;
  "transfer_id": string;
  "from_subaccount": number | bigint | string;
  "to_subaccount": number | bigint | string;
  "amount_cents": number | bigint | string;
  "created_ts": number | bigint | string;
  "exchange_index": number | bigint | string;
}

export interface PredictionsTargetBalanceAllocation {
  [field: string]: unknown;
  "exchange_index": number | bigint | string;
  "percent": number | bigint | string;
}

export interface PredictionsTargetBalanceAllocationInput {
  [field: string]: unknown;
  "exchange_index": number | bigint | string;
  "percent": number | bigint | string;
}

export interface PredictionsTickerPair {
  [field: string]: unknown;
  "market_ticker": string;
  "event_ticker": string;
  "side": string;
}

export interface PredictionsTrade {
  [field: string]: unknown;
  "trade_id": string;
  "ticker": string;
  "count_fp": string;
  "yes_price_dollars": string;
  "no_price_dollars": string;
  "taker_side"?: string | null;
  "taker_outcome_side": string;
  "taker_book_side": string;
  "created_time": string;
  "is_block_trade": boolean;
}

export interface PredictionsUpdateFCMEventContractDailyCapRequest {
  [field: string]: unknown;
  "subtrader_id": string;
  "limit": string;
}

export interface PredictionsUpdateFCMSubtraderBlockedCategoriesRequest {
  [field: string]: unknown;
  "subtrader_id": string;
  "category": string;
  "blocked": boolean;
}

export interface PredictionsUpdateFCMSubtraderBlockedCategoriesResponse {
  [field: string]: unknown;
  "categories": Array<string>;
}

export interface PredictionsUpdateOrderGroupLimitRequest {
  [field: string]: unknown;
  "contracts_limit"?: number | bigint | string | null;
  "contracts_limit_fp"?: string | null;
}

export interface PredictionsUpdateSubaccountNettingRequest {
  [field: string]: unknown;
  "subaccount_number": number | bigint | string;
  "enabled": boolean;
}

export interface PredictionsWeatherIndexCalibration {
  [field: string]: unknown;
  "config_version": string;
  "published_at_ms"?: number | bigint | string | null;
  "effective_at_ms": number | bigint | string;
  "change_reason"?: string | null;
  "calibration_window_start_ms"?: number | bigint | string | null;
  "calibration_window_end_ms"?: number | bigint | string | null;
  "city_reference_c": number | bigint | string;
  "stations": Array<PredictionsWeatherIndexCalibrationStation>;
}

export interface PredictionsWeatherIndexCalibrationStation {
  [field: string]: unknown;
  "station_id": string;
  "weight": number | bigint | string;
  "offset_c": number | bigint | string;
  "update_note"?: string | null;
}

export interface PredictionsWeatherIndexPoint {
  [field: string]: unknown;
  "t": number | bigint | string;
  "v"?: number | bigint | string | null;
  "status": string;
  "contributors"?: number | bigint | string | null;
  "receipt_basis"?: string | null;
  "stations"?: Array<PredictionsWeatherIndexStationReading> | null;
}

export interface PredictionsWeatherIndexStationReading {
  [field: string]: unknown;
  "station_id": string;
  "code": string;
  "source"?: string | null;
  "temp_f"?: number | bigint | string | null;
  "obs_time_ms"?: number | bigint | string | null;
  "received_at_ms"?: number | bigint | string | null;
  "primary_code"?: string | null;
}

export interface PredictionsWeeklySchedule {
  [field: string]: unknown;
  "start_time": string;
  "end_time": string;
  "monday": Array<PredictionsDailySchedule>;
  "tuesday": Array<PredictionsDailySchedule>;
  "wednesday": Array<PredictionsDailySchedule>;
  "thursday": Array<PredictionsDailySchedule>;
  "friday": Array<PredictionsDailySchedule>;
  "saturday": Array<PredictionsDailySchedule>;
  "sunday": Array<PredictionsDailySchedule>;
}

export interface PredictionsWithdrawal {
  [field: string]: unknown;
  "id": string;
  "status": string;
  "type": string;
  "amount_cents": number | bigint | string;
  "fee_cents": number | bigint | string;
  "created_ts": number | bigint | string;
  "finalized_ts"?: number | bigint | string | null;
}

export interface ProposeBlockTradeParams {
  [field: string]: unknown;
  "body": PredictionsProposeBlockTradeRequest;
}

export interface ProposeBlockTradeResponse {
  [field: string]: unknown;
  "block_trade_proposal_id": string;
}

export interface ResetOrderGroupParams {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
  "body"?: PredictionsEmptyResponse | null;
}

export interface ResetOrderGroupResponse {
  [field: string]: unknown;
}

export interface SetEventContractExitTriggerParams {
  [field: string]: unknown;
  "ticker": string;
  "subaccount"?: number | bigint | string | null;
  "body": PredictionsSetEventContractExitTriggerRequest;
}

export interface SetEventContractExitTriggerResponse {
  [field: string]: unknown;
  "slippage"?: PredictionsEventContractExitSlippage | null;
  "id": string;
  "ticker": string;
  "side": string;
  "subaccount_number": number | bigint | string;
  "kind": string;
  "stop_loss_price"?: string | null;
  "take_profit_price"?: string | null;
  "status": string;
  "status_reason"?: string | null;
  "triggered_order_id"?: string | null;
  "triggered_leg"?: string | null;
  "created_time": string;
  "updated_time": string;
}

export interface SetTargetBalanceAllocationParams {
  [field: string]: unknown;
  "body": PredictionsSetTargetBalanceAllocationRequest;
}

export interface SetTargetBalanceAllocationResponse {
  [field: string]: unknown;
}

export interface TriggerOrderGroupParams {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
  "body"?: PredictionsEmptyResponse | null;
}

export interface TriggerOrderGroupResponse {
  [field: string]: unknown;
}

export interface UpdateFcmEventContractDailyCapParams {
  [field: string]: unknown;
  "body": PredictionsUpdateFCMEventContractDailyCapRequest;
}

export interface UpdateFcmEventContractDailyCapResponse {
  [field: string]: unknown;
}

export interface UpdateFcmSubtraderBlockedCategoriesParams {
  [field: string]: unknown;
  "body": PredictionsUpdateFCMSubtraderBlockedCategoriesRequest;
}

export interface UpdateFcmSubtraderBlockedCategoriesResponse {
  [field: string]: unknown;
  "categories": Array<string>;
}

export interface UpdateOrderGroupLimitParams {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount"?: number | bigint | string | null;
  "exchange_index"?: number | bigint | string | null;
  "body": PredictionsUpdateOrderGroupLimitRequest;
}

export interface UpdateOrderGroupLimitResponse {
  [field: string]: unknown;
}

export interface UpdateSubaccountNettingParams {
  [field: string]: unknown;
  "body": PredictionsUpdateSubaccountNettingRequest;
}

export interface UpgradeAccountApiUsageLevelParams {
  [field: string]: unknown;
}
