"""Kalshi REST wire models generated from schema/kalshi.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_AcceptBlockTradeProposalParamsOptional = TypedDict("_AcceptBlockTradeProposalParamsOptional", {'body': 'PredictionsAcceptBlockTradeProposalRequest | None'}, total=False)
class AcceptBlockTradeProposalParams(_AcceptBlockTradeProposalParamsOptional):
    block_trade_proposal_id: str

class AcceptQuoteParams(TypedDict):
    quote_id: str
    body: PredictionsAcceptQuoteRequest

class AcceptRfqBatchParams(TypedDict):
    rfq_batch_id: str
    body: PredictionsAcceptRFQBatchRequest

class AcceptRfqBatchResponse(TypedDict):
    accepted_side: str
    quotes: list[PredictionsQuote]

class AcceptRfqQuoteParams(TypedDict):
    rfq_id: str
    quote_id: str
    body: PredictionsAcceptQuoteRequest

_AmendOrderV2ParamsOptional = TypedDict("_AmendOrderV2ParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class AmendOrderV2Params(_AmendOrderV2ParamsOptional):
    order_id: str
    body: PredictionsAmendOrderV2Request

_AmendOrderV2ResponseOptional = TypedDict("_AmendOrderV2ResponseOptional", {'client_order_id': 'str | None', 'remaining_count': 'str | None', 'fill_count': 'str | None', 'average_fill_price': 'str | None', 'average_fee_paid': 'str | None'}, total=False)
class AmendOrderV2Response(_AmendOrderV2ResponseOptional):
    order_id: str
    ts_ms: int | Decimal

class ApplySubaccountTransferParams(TypedDict):
    body: PredictionsApplySubaccountTransferRequest

class ApplySubaccountTransferResponse(TypedDict):
    pass

class BatchCancelOrdersV2Params(TypedDict):
    body: PredictionsBatchCancelOrdersV2Request

class BatchCancelOrdersV2Response(TypedDict):
    orders: list[BatchCancelOrdersV2ResponseOrdersItem]

_BatchCancelOrdersV2ResponseOrdersItemOptional = TypedDict("_BatchCancelOrdersV2ResponseOrdersItemOptional", {'client_order_id': 'str | None', 'ts_ms': 'int | Decimal | None', 'error': 'BatchCancelOrdersV2ResponseOrdersItemError | None'}, total=False)
class BatchCancelOrdersV2ResponseOrdersItem(_BatchCancelOrdersV2ResponseOrdersItemOptional):
    order_id: str
    reduced_by: str

_BatchCancelOrdersV2ResponseOrdersItemErrorOptional = TypedDict("_BatchCancelOrdersV2ResponseOrdersItemErrorOptional", {'code': 'str | None', 'message': 'str | None', 'details': 'str | None'}, total=False)
class BatchCancelOrdersV2ResponseOrdersItemError(_BatchCancelOrdersV2ResponseOrdersItemErrorOptional):
    pass

class BatchCreateOrdersV2Params(TypedDict):
    body: PredictionsBatchCreateOrdersV2Request

class BatchCreateOrdersV2Response(TypedDict):
    orders: list[BatchCreateOrdersV2ResponseOrdersItem]

_BatchCreateOrdersV2ResponseOrdersItemOptional = TypedDict("_BatchCreateOrdersV2ResponseOrdersItemOptional", {'order_id': 'str | None', 'client_order_id': 'str | None', 'fill_count': 'str | None', 'remaining_count': 'str | None', 'average_fill_price': 'str | None', 'average_fee_paid': 'str | None', 'ts_ms': 'int | Decimal | None', 'error': 'BatchCreateOrdersV2ResponseOrdersItemError | None'}, total=False)
class BatchCreateOrdersV2ResponseOrdersItem(_BatchCreateOrdersV2ResponseOrdersItemOptional):
    pass

_BatchCreateOrdersV2ResponseOrdersItemErrorOptional = TypedDict("_BatchCreateOrdersV2ResponseOrdersItemErrorOptional", {'code': 'str | None', 'message': 'str | None', 'details': 'str | None'}, total=False)
class BatchCreateOrdersV2ResponseOrdersItemError(_BatchCreateOrdersV2ResponseOrdersItemErrorOptional):
    pass

_BatchGetMarketCandlesticksParamsOptional = TypedDict("_BatchGetMarketCandlesticksParamsOptional", {'include_latest_before_start': 'bool | None'}, total=False)
class BatchGetMarketCandlesticksParams(_BatchGetMarketCandlesticksParamsOptional):
    market_tickers: str
    start_ts: int | Decimal
    end_ts: int | Decimal
    period_interval: int | Decimal

class BatchGetMarketCandlesticksResponse(TypedDict):
    markets: list[PredictionsMarketCandlesticksResponse]

_CancelAllOrdersParamsOptional = TypedDict("_CancelAllOrdersParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class CancelAllOrdersParams(_CancelAllOrdersParamsOptional):
    pass

_CancelOrderV2ParamsOptional = TypedDict("_CancelOrderV2ParamsOptional", {'subaccount': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None', 'market_ticker': 'str | None'}, total=False)
class CancelOrderV2Params(_CancelOrderV2ParamsOptional):
    order_id: str

_CancelOrderV2ResponseOptional = TypedDict("_CancelOrderV2ResponseOptional", {'client_order_id': 'str | None'}, total=False)
class CancelOrderV2Response(_CancelOrderV2ResponseOptional):
    order_id: str
    reduced_by: str
    ts_ms: int | Decimal

_ConfirmQuoteParamsOptional = TypedDict("_ConfirmQuoteParamsOptional", {'body': 'PredictionsEmptyResponse | None'}, total=False)
class ConfirmQuoteParams(_ConfirmQuoteParamsOptional):
    quote_id: str

_ConfirmRfqBatchQuoteParamsOptional = TypedDict("_ConfirmRfqBatchQuoteParamsOptional", {'body': 'PredictionsEmptyResponse | None'}, total=False)
class ConfirmRfqBatchQuoteParams(_ConfirmRfqBatchQuoteParamsOptional):
    rfq_batch_id: str
    batch_quote_id: str

_ConfirmRfqQuoteParamsOptional = TypedDict("_ConfirmRfqQuoteParamsOptional", {'body': 'PredictionsEmptyResponse | None'}, total=False)
class ConfirmRfqQuoteParams(_ConfirmRfqQuoteParamsOptional):
    rfq_id: str
    quote_id: str

class CreateApiKeyParams(TypedDict):
    body: PredictionsCreateApiKeyRequest

_CreateApiKeyResponseOptional = TypedDict("_CreateApiKeyResponseOptional", {'warning': 'str | None'}, total=False)
class CreateApiKeyResponse(_CreateApiKeyResponseOptional):
    api_key_id: str

class CreateFcmPositionTransferParams(TypedDict):
    body: PredictionsCreateFCMPositionTransferRequest

class CreateFcmPositionTransferResponse(TypedDict):
    position_transfer_id: str

class CreateFcmSubtraderParams(TypedDict):
    body: PredictionsCreateFCMSubtraderRequest

class CreateFcmSubtraderResponse(TypedDict):
    subtrader_id: str

class CreateMarketInMultivariateEventCollectionParams(TypedDict):
    collection_ticker: str
    body: PredictionsCreateMarketInMultivariateEventCollectionRequest

_CreateMarketInMultivariateEventCollectionResponseOptional = TypedDict("_CreateMarketInMultivariateEventCollectionResponseOptional", {'market': 'PredictionsMarket | None'}, total=False)
class CreateMarketInMultivariateEventCollectionResponse(_CreateMarketInMultivariateEventCollectionResponseOptional):
    event_ticker: str
    market_ticker: str

class CreateOrderGroupParams(TypedDict):
    body: PredictionsCreateOrderGroupRequest

_CreateOrderGroupResponseOptional = TypedDict("_CreateOrderGroupResponseOptional", {'exchange_index': 'int | Decimal | None'}, total=False)
class CreateOrderGroupResponse(_CreateOrderGroupResponseOptional):
    order_group_id: str
    subaccount: int | Decimal

class CreateOrderV2Params(TypedDict):
    body: PredictionsCreateOrderV2Request

_CreateOrderV2ResponseOptional = TypedDict("_CreateOrderV2ResponseOptional", {'client_order_id': 'str | None', 'average_fill_price': 'str | None', 'average_fee_paid': 'str | None'}, total=False)
class CreateOrderV2Response(_CreateOrderV2ResponseOptional):
    order_id: str
    fill_count: str
    remaining_count: str
    ts_ms: int | Decimal

class CreateOrderWithExitTriggerParams(TypedDict):
    body: PredictionsCreateOrderWithExitTriggerRequest

_CreateOrderWithExitTriggerResponseOptional = TypedDict("_CreateOrderWithExitTriggerResponseOptional", {'client_order_id': 'str | None', 'average_fill_price': 'str | None', 'average_fee_paid': 'str | None'}, total=False)
class CreateOrderWithExitTriggerResponse(_CreateOrderWithExitTriggerResponseOptional):
    order_id: str
    fill_count: str
    remaining_count: str
    ts_ms: int | Decimal

class CreateQuoteParams(TypedDict):
    body: PredictionsCreateQuoteRequest

class CreateQuoteResponse(TypedDict):
    id: str

class CreateRfqBatchParams(TypedDict):
    body: PredictionsCreateRFQBatchRequest

class CreateRfqBatchQuoteParams(TypedDict):
    rfq_batch_id: str
    body: PredictionsCreateRFQBatchQuoteRequest

class CreateRfqBatchQuoteResponse(TypedDict):
    id: str

class CreateRfqBatchResponse(TypedDict):
    rfq_batch: PredictionsRFQBatch

class CreateRfqParams(TypedDict):
    body: PredictionsCreateRFQRequest

class CreateRfqResponse(TypedDict):
    id: str

_CreateSubaccountParamsOptional = TypedDict("_CreateSubaccountParamsOptional", {'body': 'PredictionsCreateSubaccountRequest | None'}, total=False)
class CreateSubaccountParams(_CreateSubaccountParamsOptional):
    pass

class CreateSubaccountResponse(TypedDict):
    subaccount_number: int | Decimal

_DecreaseOrderV2ParamsOptional = TypedDict("_DecreaseOrderV2ParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class DecreaseOrderV2Params(_DecreaseOrderV2ParamsOptional):
    order_id: str
    body: PredictionsDecreaseOrderV2Request

_DecreaseOrderV2ResponseOptional = TypedDict("_DecreaseOrderV2ResponseOptional", {'client_order_id': 'str | None'}, total=False)
class DecreaseOrderV2Response(_DecreaseOrderV2ResponseOptional):
    order_id: str
    remaining_count: str
    ts_ms: int | Decimal

class DeleteApiKeyParams(TypedDict):
    api_key: str

_DeleteEventContractExitTriggerParamsOptional = TypedDict("_DeleteEventContractExitTriggerParamsOptional", {'subaccount': 'int | Decimal | None', 'kind': 'str | None'}, total=False)
class DeleteEventContractExitTriggerParams(_DeleteEventContractExitTriggerParamsOptional):
    ticker: str

class DeleteFcmEventContractDailyCapParams(TypedDict):
    subtrader_id: str

class DeleteFcmEventContractDailyCapResponse(TypedDict):
    pass

_DeleteOrderGroupParamsOptional = TypedDict("_DeleteOrderGroupParamsOptional", {'subaccount': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class DeleteOrderGroupParams(_DeleteOrderGroupParamsOptional):
    order_group_id: str

class DeleteOrderGroupResponse(TypedDict):
    pass

class DeleteQuoteParams(TypedDict):
    quote_id: str

class DeleteRfqBatchParams(TypedDict):
    rfq_batch_id: str

class DeleteRfqBatchQuoteParams(TypedDict):
    rfq_batch_id: str
    batch_quote_id: str

class DeleteRfqParams(TypedDict):
    rfq_id: str

class DeleteRfqQuoteParams(TypedDict):
    rfq_id: str
    quote_id: str

class GenerateApiKeyParams(TypedDict):
    body: PredictionsGenerateApiKeyRequest

_GenerateApiKeyResponseOptional = TypedDict("_GenerateApiKeyResponseOptional", {'key_type': 'str | None', 'warning': 'str | None'}, total=False)
class GenerateApiKeyResponse(_GenerateApiKeyResponseOptional):
    api_key_id: str
    private_key: str

class GetAccountApiLimitsParams(TypedDict):
    pass

class GetAccountApiLimitsResponse(TypedDict):
    usage_tier: str
    read: PredictionsBucketLimit
    write: PredictionsBucketLimit
    grants: list[PredictionsApiUsageLevelGrant]

class GetAccountApiUsageLevelVolumeProgressParams(TypedDict):
    pass

class GetAccountApiUsageLevelVolumeProgressResponse(TypedDict):
    volume_progress: list[PredictionsAccountApiUsageLevelVolumeProgress]

class GetAccountEndpointCostsParams(TypedDict):
    pass

class GetAccountEndpointCostsResponse(TypedDict):
    default_cost: int | Decimal
    endpoint_costs: list[PredictionsEndpointTokenCost]

_GetApiKeysParamsOptional = TypedDict("_GetApiKeysParamsOptional", {'fcm_subtrader_id': 'str | None'}, total=False)
class GetApiKeysParams(_GetApiKeysParamsOptional):
    pass

_GetApiKeysResponseOptional = TypedDict("_GetApiKeysResponseOptional", {'api_key_region_expiration_ts': 'int | Decimal | None'}, total=False)
class GetApiKeysResponse(_GetApiKeysResponseOptional):
    api_keys: list[PredictionsApiKey]

_GetBalanceParamsOptional = TypedDict("_GetBalanceParamsOptional", {'subaccount': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class GetBalanceParams(_GetBalanceParamsOptional):
    pass

_GetBalanceResponseOptional = TypedDict("_GetBalanceResponseOptional", {'balance_breakdown': 'list[PredictionsIndexedBalance] | None'}, total=False)
class GetBalanceResponse(_GetBalanceResponseOptional):
    balance: int | Decimal
    balance_dollars: str
    portfolio_value: int | Decimal
    updated_ts: int | Decimal

_GetBlockTradeProposalsParamsOptional = TypedDict("_GetBlockTradeProposalsParamsOptional", {'cursor': 'str | None', 'market_ticker': 'str | None', 'limit': 'int | Decimal | None', 'status': 'str | None'}, total=False)
class GetBlockTradeProposalsParams(_GetBlockTradeProposalsParamsOptional):
    pass

_GetBlockTradeProposalsResponseOptional = TypedDict("_GetBlockTradeProposalsResponseOptional", {'cursor': 'str | None'}, total=False)
class GetBlockTradeProposalsResponse(_GetBlockTradeProposalsResponseOptional):
    block_trade_proposals: list[PredictionsBlockTradeProposal]

class GetCommunicationsIdParams(TypedDict):
    pass

class GetCommunicationsIdResponse(TypedDict):
    communications_id: str

_GetCreditHistoryParamsOptional = TypedDict("_GetCreditHistoryParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None'}, total=False)
class GetCreditHistoryParams(_GetCreditHistoryParamsOptional):
    pass

class GetCreditHistoryResponse(TypedDict):
    credits: list[PredictionsCreditHistory]
    cursor: str

_GetDepositsParamsOptional = TypedDict("_GetDepositsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetDepositsParams(_GetDepositsParamsOptional):
    pass

_GetDepositsResponseOptional = TypedDict("_GetDepositsResponseOptional", {'cursor': 'str | None'}, total=False)
class GetDepositsResponse(_GetDepositsResponseOptional):
    deposits: list[PredictionsDeposit]

_GetEventContractExitTriggersParamsOptional = TypedDict("_GetEventContractExitTriggersParamsOptional", {'subaccount': 'int | Decimal | None', 'kind': 'str | None'}, total=False)
class GetEventContractExitTriggersParams(_GetEventContractExitTriggersParamsOptional):
    ticker: str

class GetEventContractExitTriggersResponse(TypedDict):
    exit_trigger: GetEventContractExitTriggersResponseExitTrigger | None
    exit_triggers: list[PredictionsEventContractExitTrigger]

_GetEventContractExitTriggersResponseExitTriggerOptional = TypedDict("_GetEventContractExitTriggersResponseExitTriggerOptional", {'slippage': 'PredictionsEventContractExitSlippage | None', 'stop_loss_price': 'str | None', 'take_profit_price': 'str | None', 'status_reason': 'str | None', 'triggered_order_id': 'str | None', 'triggered_leg': 'str | None'}, total=False)
class GetEventContractExitTriggersResponseExitTrigger(_GetEventContractExitTriggersResponseExitTriggerOptional):
    id: str
    ticker: str
    side: str
    subaccount_number: int | Decimal
    kind: str
    status: str
    created_time: str
    updated_time: str

_GetEventFeeChangesParamsOptional = TypedDict("_GetEventFeeChangesParamsOptional", {'event_ticker': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetEventFeeChangesParams(_GetEventFeeChangesParamsOptional):
    pass

class GetEventFeeChangesResponse(TypedDict):
    event_fee_changes: list[PredictionsEventFeeChange]
    cursor: str

class GetEventForecastPercentilesHistoryParams(TypedDict):
    ticker: str
    series_ticker: str
    percentiles: list[int | Decimal]
    start_ts: int | Decimal
    end_ts: int | Decimal
    period_interval: int | Decimal

class GetEventForecastPercentilesHistoryResponse(TypedDict):
    forecast_history: list[PredictionsForecastPercentilesPoint]

_GetEventLiveDataParamsOptional = TypedDict("_GetEventLiveDataParamsOptional", {'range': 'str | None'}, total=False)
class GetEventLiveDataParams(_GetEventLiveDataParamsOptional):
    event_ticker: str

class GetEventLiveDataResponse(TypedDict):
    live_data: PredictionsEventLiveData

class GetEventMetadataParams(TypedDict):
    event_ticker: str

_GetEventMetadataResponseOptional = TypedDict("_GetEventMetadataResponseOptional", {'featured_image_url': 'str | None', 'competition': 'str | None', 'competition_scope': 'str | None'}, total=False)
class GetEventMetadataResponse(_GetEventMetadataResponseOptional):
    image_url: str
    market_details: list[PredictionsMarketMetadata]
    settlement_sources: list[PredictionsSettlementSource]

_GetEventParamsOptional = TypedDict("_GetEventParamsOptional", {'exclude_markets': 'bool | None', 'with_nested_markets': 'bool | None'}, total=False)
class GetEventParams(_GetEventParamsOptional):
    event_ticker: str

class GetEventResponse(TypedDict):
    event: PredictionsEventData
    markets: list[PredictionsMarket]

_GetEventsParamsOptional = TypedDict("_GetEventsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'with_nested_markets': 'bool | None', 'with_milestones': 'bool | None', 'status': 'str | None', 'series_ticker': 'str | None', 'tickers': 'str | None', 'min_close_ts': 'int | Decimal | None', 'min_updated_ts': 'int | Decimal | None'}, total=False)
class GetEventsParams(_GetEventsParamsOptional):
    pass

_GetEventsResponseOptional = TypedDict("_GetEventsResponseOptional", {'milestones': 'list[PredictionsMilestone] | None'}, total=False)
class GetEventsResponse(_GetEventsResponseOptional):
    events: list[PredictionsEventData]
    cursor: str

class GetExchangeScheduleParams(TypedDict):
    pass

class GetExchangeScheduleResponse(TypedDict):
    schedule: PredictionsSchedule

class GetExchangeStatusParams(TypedDict):
    pass

_GetExchangeStatusResponseOptional = TypedDict("_GetExchangeStatusResponseOptional", {'intra_exchange_transfers_active': 'bool | None', 'exchange_estimated_resume_time': 'str | None', 'exchange_index_statuses': 'list[PredictionsExchangeIndexStatus] | None'}, total=False)
class GetExchangeStatusResponse(_GetExchangeStatusResponseOptional):
    exchange_active: bool
    trading_active: bool

_GetFcmEventContractDailyCapParamsOptional = TypedDict("_GetFcmEventContractDailyCapParamsOptional", {'subtrader_id': 'str | None'}, total=False)
class GetFcmEventContractDailyCapParams(_GetFcmEventContractDailyCapParamsOptional):
    pass

class GetFcmEventContractDailyCapResponse(TypedDict):
    subtrader_id: str
    limit: str
    executed_utilization: str
    resting_order_utilization: str
    pending_order_utilization: str
    cap_date: str

_GetFcmFillsParamsOptional = TypedDict("_GetFcmFillsParamsOptional", {'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetFcmFillsParams(_GetFcmFillsParamsOptional):
    pass

class GetFcmFillsResponse(TypedDict):
    fills: list[PredictionsFcmFill]
    cursor: str

_GetFcmOrdersParamsOptional = TypedDict("_GetFcmOrdersParamsOptional", {'subtrader_id': 'str | None', 'client_order_ids': 'str | None', 'cursor': 'str | None', 'event_ticker': 'str | None', 'ticker': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None', 'status': 'str | None', 'limit': 'int | Decimal | None'}, total=False)
class GetFcmOrdersParams(_GetFcmOrdersParamsOptional):
    pass

class GetFcmOrdersResponse(TypedDict):
    orders: list[PredictionsOrder]
    cursor: str

_GetFcmPositionsParamsOptional = TypedDict("_GetFcmPositionsParamsOptional", {'subtrader_id': 'str | None', 'ticker': 'str | None', 'event_ticker': 'str | None', 'count_filter': 'str | None', 'settlement_status': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetFcmPositionsParams(_GetFcmPositionsParamsOptional):
    pass

_GetFcmPositionsResponseOptional = TypedDict("_GetFcmPositionsResponseOptional", {'cursor': 'str | None'}, total=False)
class GetFcmPositionsResponse(_GetFcmPositionsResponseOptional):
    market_positions: list[PredictionsMarketPosition]
    event_positions: list[PredictionsEventPosition]

class GetFcmSubtraderBlockedCategoriesParams(TypedDict):
    subtrader_id: str

class GetFcmSubtraderBlockedCategoriesResponse(TypedDict):
    categories: list[str]

_GetFillsHistoricalParamsOptional = TypedDict("_GetFillsHistoricalParamsOptional", {'ticker': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'subaccount': 'int | Decimal | None'}, total=False)
class GetFillsHistoricalParams(_GetFillsHistoricalParamsOptional):
    pass

class GetFillsHistoricalResponse(TypedDict):
    fills: list[PredictionsFill]
    cursor: str

_GetFillsParamsOptional = TypedDict("_GetFillsParamsOptional", {'ticker': 'str | None', 'order_id': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'subaccount': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class GetFillsParams(_GetFillsParamsOptional):
    pass

class GetFillsResponse(TypedDict):
    fills: list[PredictionsFill]
    cursor: str

class GetFiltersForSportsParams(TypedDict):
    pass

class GetFiltersForSportsResponse(TypedDict):
    filters_by_sports: dict[str, PredictionsSportFilterDetails]
    sport_ordering: list[str]

class GetGameStatsParams(TypedDict):
    milestone_id: str

_GetGameStatsResponseOptional = TypedDict("_GetGameStatsResponseOptional", {'pbp': 'PredictionsPlayByPlay | None'}, total=False)
class GetGameStatsResponse(_GetGameStatsResponseOptional):
    pass

class GetHistoricalCutoffParams(TypedDict):
    pass

_GetHistoricalCutoffResponseOptional = TypedDict("_GetHistoricalCutoffResponseOptional", {'market_positions_last_updated_ts': 'str | None'}, total=False)
class GetHistoricalCutoffResponse(_GetHistoricalCutoffResponseOptional):
    market_settled_ts: str
    trades_created_ts: str
    orders_updated_ts: str

class GetHistoricalMarketParams(TypedDict):
    ticker: str

class GetHistoricalMarketResponse(TypedDict):
    market: PredictionsMarket

_GetHistoricalMarketsParamsOptional = TypedDict("_GetHistoricalMarketsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'tickers': 'str | None', 'event_ticker': 'str | None', 'series_ticker': 'str | None', 'mve_filter': 'str | None'}, total=False)
class GetHistoricalMarketsParams(_GetHistoricalMarketsParamsOptional):
    pass

class GetHistoricalMarketsResponse(TypedDict):
    markets: list[PredictionsMarket]
    cursor: str

_GetHistoricalOrdersParamsOptional = TypedDict("_GetHistoricalOrdersParamsOptional", {'ticker': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'subaccount': 'int | Decimal | None'}, total=False)
class GetHistoricalOrdersParams(_GetHistoricalOrdersParamsOptional):
    pass

class GetHistoricalOrdersResponse(TypedDict):
    orders: list[PredictionsOrder]
    cursor: str

_GetHistoricalPositionsParamsOptional = TypedDict("_GetHistoricalPositionsParamsOptional", {'ticker': 'str | None', 'event_ticker': 'str | None', 'subaccount': 'int | Decimal | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetHistoricalPositionsParams(_GetHistoricalPositionsParamsOptional):
    pass

_GetHistoricalPositionsResponseOptional = TypedDict("_GetHistoricalPositionsResponseOptional", {'cursor': 'str | None'}, total=False)
class GetHistoricalPositionsResponse(_GetHistoricalPositionsResponseOptional):
    market_positions: list[PredictionsMarketPosition]
    event_positions: list[PredictionsEventPosition]

_GetIncentiveProgramsParamsOptional = TypedDict("_GetIncentiveProgramsParamsOptional", {'status': 'str | None', 'type': 'str | None', 'incentive_description': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetIncentiveProgramsParams(_GetIncentiveProgramsParamsOptional):
    pass

_GetIncentiveProgramsResponseOptional = TypedDict("_GetIncentiveProgramsResponseOptional", {'next_cursor': 'str | None'}, total=False)
class GetIncentiveProgramsResponse(_GetIncentiveProgramsResponseOptional):
    incentive_programs: list[PredictionsIncentiveProgram]

class GetIntraExchangeInstanceTransferParams(TypedDict):
    transfer_id: str

class GetIntraExchangeInstanceTransferResponse(TypedDict):
    transfer: PredictionsIntraExchangeInstanceTransfer

_GetIntraExchangeInstanceTransfersParamsOptional = TypedDict("_GetIntraExchangeInstanceTransfersParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetIntraExchangeInstanceTransfersParams(_GetIntraExchangeInstanceTransfersParamsOptional):
    pass

_GetIntraExchangeInstanceTransfersResponseOptional = TypedDict("_GetIntraExchangeInstanceTransfersResponseOptional", {'cursor': 'str | None'}, total=False)
class GetIntraExchangeInstanceTransfersResponse(_GetIntraExchangeInstanceTransfersResponseOptional):
    transfers: list[PredictionsIntraExchangeInstanceTransfer]

_GetLiveDataByMilestoneParamsOptional = TypedDict("_GetLiveDataByMilestoneParamsOptional", {'include_player_stats': 'bool | None'}, total=False)
class GetLiveDataByMilestoneParams(_GetLiveDataByMilestoneParamsOptional):
    milestone_id: str

class GetLiveDataByMilestoneResponse(TypedDict):
    live_data: PredictionsLiveData

_GetLiveDataParamsOptional = TypedDict("_GetLiveDataParamsOptional", {'include_player_stats': 'bool | None'}, total=False)
class GetLiveDataParams(_GetLiveDataParamsOptional):
    type: str
    milestone_id: str

class GetLiveDataResponse(TypedDict):
    live_data: PredictionsLiveData

_GetLiveDatasParamsOptional = TypedDict("_GetLiveDatasParamsOptional", {'include_player_stats': 'bool | None'}, total=False)
class GetLiveDatasParams(_GetLiveDatasParamsOptional):
    milestone_ids: list[str]

class GetLiveDatasResponse(TypedDict):
    live_datas: list[PredictionsLiveData]

class GetMarketCandlesticksByEventParams(TypedDict):
    ticker: str
    series_ticker: str
    start_ts: int | Decimal
    end_ts: int | Decimal
    period_interval: int | Decimal

class GetMarketCandlesticksByEventResponse(TypedDict):
    market_tickers: list[str]
    market_candlesticks: list[list[PredictionsMarketCandlestick]]
    adjusted_end_ts: int | Decimal

class GetMarketCandlesticksHistoricalParams(TypedDict):
    ticker: str
    start_ts: int | Decimal
    end_ts: int | Decimal
    period_interval: int | Decimal

class GetMarketCandlesticksHistoricalResponse(TypedDict):
    ticker: str
    candlesticks: list[PredictionsMarketCandlestickHistorical]

_GetMarketCandlesticksParamsOptional = TypedDict("_GetMarketCandlesticksParamsOptional", {'include_latest_before_start': 'bool | None'}, total=False)
class GetMarketCandlesticksParams(_GetMarketCandlesticksParamsOptional):
    series_ticker: str
    ticker: str
    start_ts: int | Decimal
    end_ts: int | Decimal
    period_interval: int | Decimal

class GetMarketCandlesticksResponse(TypedDict):
    ticker: str
    candlesticks: list[PredictionsMarketCandlestick]

_GetMarketOrderbookParamsOptional = TypedDict("_GetMarketOrderbookParamsOptional", {'depth': 'int | Decimal | None'}, total=False)
class GetMarketOrderbookParams(_GetMarketOrderbookParamsOptional):
    ticker: str

class GetMarketOrderbookResponse(TypedDict):
    orderbook_fp: PredictionsOrderbookCountFp

class GetMarketOrderbooksParams(TypedDict):
    tickers: list[str]

class GetMarketOrderbooksResponse(TypedDict):
    orderbooks: list[PredictionsMarketOrderbookFp]

class GetMarketParams(TypedDict):
    ticker: str

class GetMarketResponse(TypedDict):
    market: PredictionsMarket

_GetMarketsParamsOptional = TypedDict("_GetMarketsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'event_ticker': 'str | None', 'series_ticker': 'str | None', 'min_created_ts': 'int | Decimal | None', 'max_created_ts': 'int | Decimal | None', 'min_updated_ts': 'int | Decimal | None', 'max_updated_ts': 'int | Decimal | None', 'max_close_ts': 'int | Decimal | None', 'min_close_ts': 'int | Decimal | None', 'min_settled_ts': 'int | Decimal | None', 'max_settled_ts': 'int | Decimal | None', 'status': 'str | None', 'tickers': 'str | None', 'mve_filter': 'str | None'}, total=False)
class GetMarketsParams(_GetMarketsParamsOptional):
    pass

class GetMarketsResponse(TypedDict):
    markets: list[PredictionsMarket]
    cursor: str

class GetMilestoneParams(TypedDict):
    milestone_id: str

class GetMilestoneResponse(TypedDict):
    milestone: PredictionsMilestone

_GetMilestonesParamsOptional = TypedDict("_GetMilestonesParamsOptional", {'minimum_start_date': 'str | None', 'category': 'str | None', 'competition': 'str | None', 'source_id': 'str | None', 'type': 'str | None', 'related_event_ticker': 'str | None', 'cursor': 'str | None', 'min_updated_ts': 'int | Decimal | None'}, total=False)
class GetMilestonesParams(_GetMilestonesParamsOptional):
    limit: int | Decimal

_GetMilestonesResponseOptional = TypedDict("_GetMilestonesResponseOptional", {'cursor': 'str | None'}, total=False)
class GetMilestonesResponse(_GetMilestonesResponseOptional):
    milestones: list[PredictionsMilestone]

class GetMultivariateEventCollectionParams(TypedDict):
    collection_ticker: str

class GetMultivariateEventCollectionResponse(TypedDict):
    multivariate_contract: PredictionsMultivariateEventCollection

_GetMultivariateEventCollectionsParamsOptional = TypedDict("_GetMultivariateEventCollectionsParamsOptional", {'status': 'str | None', 'associated_event_ticker': 'str | None', 'series_ticker': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetMultivariateEventCollectionsParams(_GetMultivariateEventCollectionsParamsOptional):
    pass

_GetMultivariateEventCollectionsResponseOptional = TypedDict("_GetMultivariateEventCollectionsResponseOptional", {'cursor': 'str | None'}, total=False)
class GetMultivariateEventCollectionsResponse(_GetMultivariateEventCollectionsResponseOptional):
    multivariate_contracts: list[PredictionsMultivariateEventCollection]

_GetMultivariateEventsParamsOptional = TypedDict("_GetMultivariateEventsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'series_ticker': 'str | None', 'collection_ticker': 'str | None', 'tickers': 'str | None', 'with_nested_markets': 'bool | None'}, total=False)
class GetMultivariateEventsParams(_GetMultivariateEventsParamsOptional):
    pass

class GetMultivariateEventsResponse(TypedDict):
    events: list[PredictionsEventData]
    cursor: str

_GetOrderGroupParamsOptional = TypedDict("_GetOrderGroupParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class GetOrderGroupParams(_GetOrderGroupParamsOptional):
    order_group_id: str

_GetOrderGroupResponseOptional = TypedDict("_GetOrderGroupResponseOptional", {'contracts_limit_fp': 'str | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class GetOrderGroupResponse(_GetOrderGroupResponseOptional):
    is_auto_cancel_enabled: bool
    orders: list[str]

_GetOrderGroupsParamsOptional = TypedDict("_GetOrderGroupsParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class GetOrderGroupsParams(_GetOrderGroupsParamsOptional):
    pass

_GetOrderGroupsResponseOptional = TypedDict("_GetOrderGroupsResponseOptional", {'order_groups': 'list[PredictionsOrderGroup] | None'}, total=False)
class GetOrderGroupsResponse(_GetOrderGroupsResponseOptional):
    pass

class GetOrderParams(TypedDict):
    order_id: str

class GetOrderQueuePositionParams(TypedDict):
    order_id: str

class GetOrderQueuePositionResponse(TypedDict):
    queue_position_fp: str

_GetOrderQueuePositionsParamsOptional = TypedDict("_GetOrderQueuePositionsParamsOptional", {'market_tickers': 'str | None', 'event_ticker': 'str | None', 'subaccount': 'int | Decimal | None'}, total=False)
class GetOrderQueuePositionsParams(_GetOrderQueuePositionsParamsOptional):
    pass

class GetOrderQueuePositionsResponse(TypedDict):
    queue_positions: list[PredictionsOrderQueuePosition]

class GetOrderResponse(TypedDict):
    order: PredictionsOrder

_GetOrdersParamsOptional = TypedDict("_GetOrdersParamsOptional", {'ticker': 'str | None', 'event_ticker': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None', 'status': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'subaccount': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class GetOrdersParams(_GetOrdersParamsOptional):
    pass

class GetOrdersResponse(TypedDict):
    orders: list[PredictionsOrder]
    cursor: str

class GetPortfolioRestingOrderTotalValueParams(TypedDict):
    pass

class GetPortfolioRestingOrderTotalValueResponse(TypedDict):
    total_resting_order_value: int | Decimal
    resting_order_value_breakdown: list[PredictionsIndexedBalance]

_GetPositionsParamsOptional = TypedDict("_GetPositionsParamsOptional", {'cursor': 'str | None', 'limit': 'int | Decimal | None', 'count_filter': 'str | None', 'settlement_status': 'str | None', 'ticker': 'str | None', 'event_ticker': 'str | None', 'subaccount': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class GetPositionsParams(_GetPositionsParamsOptional):
    pass

_GetPositionsResponseOptional = TypedDict("_GetPositionsResponseOptional", {'cursor': 'str | None'}, total=False)
class GetPositionsResponse(_GetPositionsResponseOptional):
    market_positions: list[PredictionsMarketPosition]
    event_positions: list[PredictionsEventPosition]

class GetQuoteParams(TypedDict):
    quote_id: str

class GetQuoteResponse(TypedDict):
    quote: PredictionsQuote

_GetQuotesParamsOptional = TypedDict("_GetQuotesParamsOptional", {'cursor': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None', 'limit': 'int | Decimal | None', 'status': 'str | None', 'quote_creator_user_id': 'str | None', 'user_filter': 'str | None', 'rfq_user_filter': 'str | None', 'rfq_creator_user_id': 'str | None', 'rfq_creator_subtrader_id': 'str | None', 'rfq_id': 'str | None'}, total=False)
class GetQuotesParams(_GetQuotesParamsOptional):
    pass

_GetQuotesResponseOptional = TypedDict("_GetQuotesResponseOptional", {'cursor': 'str | None'}, total=False)
class GetQuotesResponse(_GetQuotesResponseOptional):
    quotes: list[PredictionsQuote]

_GetRfQsParamsOptional = TypedDict("_GetRfQsParamsOptional", {'cursor': 'str | None', 'event_ticker': 'str | None', 'market_ticker': 'str | None', 'subaccount': 'int | Decimal | None', 'limit': 'int | Decimal | None', 'status': 'str | None', 'creator_user_id': 'str | None', 'user_filter': 'str | None'}, total=False)
class GetRfQsParams(_GetRfQsParamsOptional):
    pass

_GetRfQsResponseOptional = TypedDict("_GetRfQsResponseOptional", {'cursor': 'str | None'}, total=False)
class GetRfQsResponse(_GetRfQsResponseOptional):
    rfqs: list[PredictionsRFQ]

class GetRfqBatchBestQuotesParams(TypedDict):
    rfq_batch_id: str

class GetRfqBatchBestQuotesResponse(TypedDict):
    best_quotes: PredictionsRFQBatchBestQuotes

class GetRfqBatchParams(TypedDict):
    rfq_batch_id: str

class GetRfqBatchQuotesParams(TypedDict):
    rfq_batch_id: str

class GetRfqBatchQuotesResponse(TypedDict):
    batch_quotes: list[PredictionsRFQBatchQuote]

class GetRfqBatchResponse(TypedDict):
    rfq_batch: PredictionsRFQBatch

_GetRfqBatchesParamsOptional = TypedDict("_GetRfqBatchesParamsOptional", {'cursor': 'str | None', 'subaccount': 'int | Decimal | None', 'limit': 'int | Decimal | None', 'status': 'str | None', 'user_filter': 'str | None'}, total=False)
class GetRfqBatchesParams(_GetRfqBatchesParamsOptional):
    pass

_GetRfqBatchesResponseOptional = TypedDict("_GetRfqBatchesResponseOptional", {'cursor': 'str | None'}, total=False)
class GetRfqBatchesResponse(_GetRfqBatchesResponseOptional):
    rfq_batches: list[PredictionsRFQBatch]

class GetRfqParams(TypedDict):
    rfq_id: str

class GetRfqQuoteParams(TypedDict):
    rfq_id: str
    quote_id: str

class GetRfqQuoteResponse(TypedDict):
    quote: PredictionsQuote

class GetRfqResponse(TypedDict):
    rfq: PredictionsRFQ

_GetSeriesFeeChangesParamsOptional = TypedDict("_GetSeriesFeeChangesParamsOptional", {'series_ticker': 'str | None', 'show_historical': 'bool | None'}, total=False)
class GetSeriesFeeChangesParams(_GetSeriesFeeChangesParamsOptional):
    pass

class GetSeriesFeeChangesResponse(TypedDict):
    series_fee_change_arr: list[PredictionsSeriesFeeChange]

_GetSeriesListParamsOptional = TypedDict("_GetSeriesListParamsOptional", {'category': 'str | None', 'tags': 'str | None', 'include_product_metadata': 'bool | None', 'include_volume': 'bool | None', 'min_updated_ts': 'int | Decimal | None'}, total=False)
class GetSeriesListParams(_GetSeriesListParamsOptional):
    pass

class GetSeriesListResponse(TypedDict):
    series: list[PredictionsSeries]

_GetSeriesParamsOptional = TypedDict("_GetSeriesParamsOptional", {'include_volume': 'bool | None'}, total=False)
class GetSeriesParams(_GetSeriesParamsOptional):
    series_ticker: str

class GetSeriesResponse(TypedDict):
    series: PredictionsSeries

_GetSettlementsParamsOptional = TypedDict("_GetSettlementsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'ticker': 'str | None', 'event_ticker': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None', 'subaccount': 'int | Decimal | None'}, total=False)
class GetSettlementsParams(_GetSettlementsParamsOptional):
    pass

_GetSettlementsResponseOptional = TypedDict("_GetSettlementsResponseOptional", {'cursor': 'str | None'}, total=False)
class GetSettlementsResponse(_GetSettlementsResponseOptional):
    settlements: list[PredictionsSettlement]

class GetStructuredTargetParams(TypedDict):
    structured_target_id: str

_GetStructuredTargetResponseOptional = TypedDict("_GetStructuredTargetResponseOptional", {'structured_target': 'PredictionsStructuredTarget | None'}, total=False)
class GetStructuredTargetResponse(_GetStructuredTargetResponseOptional):
    pass

_GetStructuredTargetsParamsOptional = TypedDict("_GetStructuredTargetsParamsOptional", {'ids': 'list[str] | None', 'type': 'str | None', 'competition': 'str | None', 'page_size': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetStructuredTargetsParams(_GetStructuredTargetsParamsOptional):
    pass

_GetStructuredTargetsResponseOptional = TypedDict("_GetStructuredTargetsResponseOptional", {'structured_targets': 'list[PredictionsStructuredTarget] | None', 'cursor': 'str | None'}, total=False)
class GetStructuredTargetsResponse(_GetStructuredTargetsResponseOptional):
    pass

class GetSubaccountBalancesParams(TypedDict):
    pass

class GetSubaccountBalancesResponse(TypedDict):
    subaccount_balances: list[PredictionsSubaccountBalance]

class GetSubaccountNettingParams(TypedDict):
    pass

class GetSubaccountNettingResponse(TypedDict):
    netting_configs: list[PredictionsSubaccountNettingConfig]

_GetSubaccountTransfersParamsOptional = TypedDict("_GetSubaccountTransfersParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetSubaccountTransfersParams(_GetSubaccountTransfersParamsOptional):
    pass

_GetSubaccountTransfersResponseOptional = TypedDict("_GetSubaccountTransfersResponseOptional", {'cursor': 'str | None'}, total=False)
class GetSubaccountTransfersResponse(_GetSubaccountTransfersResponseOptional):
    transfers: list[PredictionsSubaccountTransfer]

class GetTagsForSeriesCategoriesParams(TypedDict):
    pass

class GetTagsForSeriesCategoriesResponse(TypedDict):
    tags_by_categories: dict[str, list[str]]

class GetTargetBalanceAllocationParams(TypedDict):
    pass

class GetTargetBalanceAllocationResponse(TypedDict):
    allocations: list[PredictionsTargetBalanceAllocation]
    resting_margin_reservation: str

_GetTradesHistoricalParamsOptional = TypedDict("_GetTradesHistoricalParamsOptional", {'ticker': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'is_block_trade': 'bool | None'}, total=False)
class GetTradesHistoricalParams(_GetTradesHistoricalParamsOptional):
    pass

class GetTradesHistoricalResponse(TypedDict):
    trades: list[PredictionsTrade]
    cursor: str

_GetTradesParamsOptional = TypedDict("_GetTradesParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'ticker': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None', 'is_block_trade': 'bool | None'}, total=False)
class GetTradesParams(_GetTradesParamsOptional):
    pass

class GetTradesResponse(TypedDict):
    trades: list[PredictionsTrade]
    cursor: str

class GetUserDataTimestampParams(TypedDict):
    pass

class GetUserDataTimestampResponse(TypedDict):
    as_of_time: str

class GetWeatherIndexCalibrationsParams(TypedDict):
    city: str

class GetWeatherIndexCalibrationsResponse(TypedDict):
    city: str
    units: str
    calibrations: list[PredictionsWeatherIndexCalibration]

_GetWeatherIndexParamsOptional = TypedDict("_GetWeatherIndexParamsOptional", {'from': 'int | Decimal | None', 'to': 'int | Decimal | None', 'last_sec': 'int | Decimal | None', 'detailed': 'bool | None'}, total=False)
class GetWeatherIndexParams(_GetWeatherIndexParamsOptional):
    city: str

_GetWeatherIndexResponseOptional = TypedDict("_GetWeatherIndexResponseOptional", {'config_version': 'str | None'}, total=False)
class GetWeatherIndexResponse(_GetWeatherIndexResponseOptional):
    city: str
    units: str
    timeseries: list[PredictionsWeatherIndexPoint]

_GetWithdrawalsParamsOptional = TypedDict("_GetWithdrawalsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetWithdrawalsParams(_GetWithdrawalsParamsOptional):
    pass

_GetWithdrawalsResponseOptional = TypedDict("_GetWithdrawalsResponseOptional", {'cursor': 'str | None'}, total=False)
class GetWithdrawalsResponse(_GetWithdrawalsResponseOptional):
    withdrawals: list[PredictionsWithdrawal]

class IntraExchangeInstanceTransferParams(TypedDict):
    body: PredictionsIntraExchangeInstanceTransferRequest

class IntraExchangeInstanceTransferResponse(TypedDict):
    transfer_id: str

class ListFcmSubtradersParams(TypedDict):
    pass

class ListFcmSubtradersResponse(TypedDict):
    subtraders: list[PredictionsFCMSubtrader]

_PredictionsAcceptBlockTradeProposalRequestOptional = TypedDict("_PredictionsAcceptBlockTradeProposalRequestOptional", {'subtrader_id': 'str | None', 'subaccount': 'int | Decimal | None'}, total=False)
class PredictionsAcceptBlockTradeProposalRequest(_PredictionsAcceptBlockTradeProposalRequestOptional):
    pass

class PredictionsAcceptQuoteRequest(TypedDict):
    accepted_side: str

_PredictionsAcceptRFQBatchRequestOptional = TypedDict("_PredictionsAcceptRFQBatchRequestOptional", {'client_order_id': 'str | None'}, total=False)
class PredictionsAcceptRFQBatchRequest(_PredictionsAcceptRFQBatchRequestOptional):
    accepted_side: str
    min_bids: list[PredictionsRFQBatchAcceptLimit]

class PredictionsAcceptRFQBatchResponse(TypedDict):
    accepted_side: str
    quotes: list[PredictionsQuote]

class PredictionsAccountApiUsageLevelVolumeGoal(TypedDict):
    level: str
    earn_volume_goal_fp: str
    keep_volume_goal_fp: str

class PredictionsAccountApiUsageLevelVolumeProgress(TypedDict):
    computed_ts: int | Decimal
    trailing_30d_volume_fp: str
    goals: list[PredictionsAccountApiUsageLevelVolumeGoal]

_PredictionsAmendOrderRequestOptional = TypedDict("_PredictionsAmendOrderRequestOptional", {'subaccount': 'int | Decimal | None', 'client_order_id': 'str | None', 'updated_client_order_id': 'str | None', 'yes_price': 'int | Decimal | None', 'no_price': 'int | Decimal | None', 'yes_price_dollars': 'str | None', 'no_price_dollars': 'str | None', 'count': 'int | Decimal | None', 'count_fp': 'str | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsAmendOrderRequest(_PredictionsAmendOrderRequestOptional):
    ticker: str
    side: str
    action: str

class PredictionsAmendOrderResponse(TypedDict):
    old_order: PredictionsOrder
    order: PredictionsOrder

_PredictionsAmendOrderV2RequestOptional = TypedDict("_PredictionsAmendOrderV2RequestOptional", {'client_order_id': 'str | None', 'updated_client_order_id': 'str | None', 'expiration_time': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsAmendOrderV2Request(_PredictionsAmendOrderV2RequestOptional):
    ticker: str
    side: str
    price: str
    count: str

_PredictionsAmendOrderV2ResponseOptional = TypedDict("_PredictionsAmendOrderV2ResponseOptional", {'client_order_id': 'str | None', 'remaining_count': 'str | None', 'fill_count': 'str | None', 'average_fill_price': 'str | None', 'average_fee_paid': 'str | None'}, total=False)
class PredictionsAmendOrderV2Response(_PredictionsAmendOrderV2ResponseOptional):
    order_id: str
    ts_ms: int | Decimal

_PredictionsApiKeyOptional = TypedDict("_PredictionsApiKeyOptional", {'subaccount': 'int | Decimal | None', 'fcm_subtrader_id': 'str | None'}, total=False)
class PredictionsApiKey(_PredictionsApiKeyOptional):
    api_key_id: str
    name: str
    scopes: list[str]

_PredictionsApiUsageLevelGrantOptional = TypedDict("_PredictionsApiUsageLevelGrantOptional", {'expires_ts': 'int | Decimal | None'}, total=False)
class PredictionsApiUsageLevelGrant(_PredictionsApiUsageLevelGrantOptional):
    exchange_instance: str
    level: str
    source: str

_PredictionsApplySubaccountTransferRequestOptional = TypedDict("_PredictionsApplySubaccountTransferRequestOptional", {'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsApplySubaccountTransferRequest(_PredictionsApplySubaccountTransferRequestOptional):
    client_transfer_id: str
    from_subaccount: int | Decimal
    to_subaccount: int | Decimal
    amount_cents: int | Decimal

class PredictionsApplySubaccountTransferResponse(TypedDict):
    pass

_PredictionsAssociatedEventOptional = TypedDict("_PredictionsAssociatedEventOptional", {'size_max': 'int | Decimal | None', 'size_min': 'int | Decimal | None'}, total=False)
class PredictionsAssociatedEvent(_PredictionsAssociatedEventOptional):
    ticker: str
    is_yes_only: bool
    active_quoters: list[str]

_PredictionsAttachedEventContractExitTriggerOptional = TypedDict("_PredictionsAttachedEventContractExitTriggerOptional", {'slippage': 'PredictionsEventContractExitSlippage | None', 'stop_loss_price': 'str | None', 'take_profit_price': 'str | None'}, total=False)
class PredictionsAttachedEventContractExitTrigger(_PredictionsAttachedEventContractExitTriggerOptional):
    pass

_PredictionsBatchCancelOrdersIndividualResponseOptional = TypedDict("_PredictionsBatchCancelOrdersIndividualResponseOptional", {'order': 'PredictionsBatchCancelOrdersIndividualResponseOrder | None', 'error': 'PredictionsBatchCancelOrdersIndividualResponseError | None'}, total=False)
class PredictionsBatchCancelOrdersIndividualResponse(_PredictionsBatchCancelOrdersIndividualResponseOptional):
    order_id: str
    reduced_by_fp: str

_PredictionsBatchCancelOrdersIndividualResponseErrorOptional = TypedDict("_PredictionsBatchCancelOrdersIndividualResponseErrorOptional", {'code': 'str | None', 'message': 'str | None', 'details': 'str | None'}, total=False)
class PredictionsBatchCancelOrdersIndividualResponseError(_PredictionsBatchCancelOrdersIndividualResponseErrorOptional):
    pass

_PredictionsBatchCancelOrdersIndividualResponseOrderOptional = TypedDict("_PredictionsBatchCancelOrdersIndividualResponseOrderOptional", {'side': 'str | None', 'action': 'str | None', 'expiration_time': 'str | None', 'created_time': 'str | None', 'last_update_time': 'str | None', 'self_trade_prevention_type': 'str | None', 'order_group_id': 'str | None', 'cancel_order_on_pause': 'bool | None', 'subaccount_number': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsBatchCancelOrdersIndividualResponseOrder(_PredictionsBatchCancelOrdersIndividualResponseOrderOptional):
    order_id: str
    user_id: str
    client_order_id: str
    ticker: str
    outcome_side: str
    book_side: str
    type: str
    status: str
    yes_price_dollars: str
    no_price_dollars: str
    fill_count_fp: str
    remaining_count_fp: str
    initial_count_fp: str
    taker_fill_cost_dollars: str
    maker_fill_cost_dollars: str
    taker_fees_dollars: str
    maker_fees_dollars: str

_PredictionsBatchCancelOrdersRequestOptional = TypedDict("_PredictionsBatchCancelOrdersRequestOptional", {'ids': 'list[str] | None', 'orders': 'list[PredictionsBatchCancelOrdersRequestOrder] | None'}, total=False)
class PredictionsBatchCancelOrdersRequest(_PredictionsBatchCancelOrdersRequestOptional):
    pass

_PredictionsBatchCancelOrdersRequestOrderOptional = TypedDict("_PredictionsBatchCancelOrdersRequestOrderOptional", {'subaccount': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None', 'market_ticker': 'str | None'}, total=False)
class PredictionsBatchCancelOrdersRequestOrder(_PredictionsBatchCancelOrdersRequestOrderOptional):
    order_id: str

class PredictionsBatchCancelOrdersResponse(TypedDict):
    orders: list[PredictionsBatchCancelOrdersIndividualResponse]

class PredictionsBatchCancelOrdersV2Request(TypedDict):
    orders: list[PredictionsBatchCancelOrdersV2RequestOrdersItem]

_PredictionsBatchCancelOrdersV2RequestOrdersItemOptional = TypedDict("_PredictionsBatchCancelOrdersV2RequestOrdersItemOptional", {'subaccount': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None', 'market_ticker': 'str | None'}, total=False)
class PredictionsBatchCancelOrdersV2RequestOrdersItem(_PredictionsBatchCancelOrdersV2RequestOrdersItemOptional):
    order_id: str

class PredictionsBatchCancelOrdersV2Response(TypedDict):
    orders: list[PredictionsBatchCancelOrdersV2ResponseOrdersItem]

_PredictionsBatchCancelOrdersV2ResponseOrdersItemOptional = TypedDict("_PredictionsBatchCancelOrdersV2ResponseOrdersItemOptional", {'client_order_id': 'str | None', 'ts_ms': 'int | Decimal | None', 'error': 'PredictionsBatchCancelOrdersV2ResponseOrdersItemError | None'}, total=False)
class PredictionsBatchCancelOrdersV2ResponseOrdersItem(_PredictionsBatchCancelOrdersV2ResponseOrdersItemOptional):
    order_id: str
    reduced_by: str

_PredictionsBatchCancelOrdersV2ResponseOrdersItemErrorOptional = TypedDict("_PredictionsBatchCancelOrdersV2ResponseOrdersItemErrorOptional", {'code': 'str | None', 'message': 'str | None', 'details': 'str | None'}, total=False)
class PredictionsBatchCancelOrdersV2ResponseOrdersItemError(_PredictionsBatchCancelOrdersV2ResponseOrdersItemErrorOptional):
    pass

_PredictionsBatchCreateOrdersIndividualResponseOptional = TypedDict("_PredictionsBatchCreateOrdersIndividualResponseOptional", {'client_order_id': 'str | None', 'order': 'PredictionsBatchCreateOrdersIndividualResponseOrder | None', 'error': 'PredictionsBatchCreateOrdersIndividualResponseError | None'}, total=False)
class PredictionsBatchCreateOrdersIndividualResponse(_PredictionsBatchCreateOrdersIndividualResponseOptional):
    pass

_PredictionsBatchCreateOrdersIndividualResponseErrorOptional = TypedDict("_PredictionsBatchCreateOrdersIndividualResponseErrorOptional", {'code': 'str | None', 'message': 'str | None', 'details': 'str | None'}, total=False)
class PredictionsBatchCreateOrdersIndividualResponseError(_PredictionsBatchCreateOrdersIndividualResponseErrorOptional):
    pass

_PredictionsBatchCreateOrdersIndividualResponseOrderOptional = TypedDict("_PredictionsBatchCreateOrdersIndividualResponseOrderOptional", {'side': 'str | None', 'action': 'str | None', 'expiration_time': 'str | None', 'created_time': 'str | None', 'last_update_time': 'str | None', 'self_trade_prevention_type': 'str | None', 'order_group_id': 'str | None', 'cancel_order_on_pause': 'bool | None', 'subaccount_number': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsBatchCreateOrdersIndividualResponseOrder(_PredictionsBatchCreateOrdersIndividualResponseOrderOptional):
    order_id: str
    user_id: str
    client_order_id: str
    ticker: str
    outcome_side: str
    book_side: str
    type: str
    status: str
    yes_price_dollars: str
    no_price_dollars: str
    fill_count_fp: str
    remaining_count_fp: str
    initial_count_fp: str
    taker_fill_cost_dollars: str
    maker_fill_cost_dollars: str
    taker_fees_dollars: str
    maker_fees_dollars: str

class PredictionsBatchCreateOrdersRequest(TypedDict):
    orders: list[PredictionsCreateOrderRequest]

class PredictionsBatchCreateOrdersResponse(TypedDict):
    orders: list[PredictionsBatchCreateOrdersIndividualResponse]

class PredictionsBatchCreateOrdersV2Request(TypedDict):
    orders: list[PredictionsCreateOrderV2Request]

class PredictionsBatchCreateOrdersV2Response(TypedDict):
    orders: list[PredictionsBatchCreateOrdersV2ResponseOrdersItem]

_PredictionsBatchCreateOrdersV2ResponseOrdersItemOptional = TypedDict("_PredictionsBatchCreateOrdersV2ResponseOrdersItemOptional", {'order_id': 'str | None', 'client_order_id': 'str | None', 'fill_count': 'str | None', 'remaining_count': 'str | None', 'average_fill_price': 'str | None', 'average_fee_paid': 'str | None', 'ts_ms': 'int | Decimal | None', 'error': 'PredictionsBatchCreateOrdersV2ResponseOrdersItemError | None'}, total=False)
class PredictionsBatchCreateOrdersV2ResponseOrdersItem(_PredictionsBatchCreateOrdersV2ResponseOrdersItemOptional):
    pass

_PredictionsBatchCreateOrdersV2ResponseOrdersItemErrorOptional = TypedDict("_PredictionsBatchCreateOrdersV2ResponseOrdersItemErrorOptional", {'code': 'str | None', 'message': 'str | None', 'details': 'str | None'}, total=False)
class PredictionsBatchCreateOrdersV2ResponseOrdersItemError(_PredictionsBatchCreateOrdersV2ResponseOrdersItemErrorOptional):
    pass

class PredictionsBatchGetMarketCandlesticksResponse(TypedDict):
    markets: list[PredictionsMarketCandlesticksResponse]

class PredictionsBidAskDistribution(TypedDict):
    open_dollars: str
    low_dollars: str
    high_dollars: str
    close_dollars: str

class PredictionsBidAskDistributionHistorical(TypedDict):
    open: str
    low: str
    high: str
    close: str

_PredictionsBlockTradeProposalOptional = TypedDict("_PredictionsBlockTradeProposalOptional", {'buyer_subtrader_id': 'str | None', 'seller_subtrader_id': 'str | None', 'buyer_accepted_ts': 'str | None', 'seller_accepted_ts': 'str | None', 'executed_ts': 'str | None', 'buyer_order_id': 'str | None', 'seller_order_id': 'str | None'}, total=False)
class PredictionsBlockTradeProposal(_PredictionsBlockTradeProposalOptional):
    id: str
    proposer_user_id: str
    buyer_user_id: str
    seller_user_id: str
    market_ticker: str
    price_centi_cents: int | Decimal
    centicount: int | Decimal
    maker_side: str
    expiration_ts: str
    status: str
    created_ts: str
    updated_ts: str
    buyer_accepted: bool
    seller_accepted: bool

class PredictionsBucketLimit(TypedDict):
    refill_rate: int | Decimal
    bucket_capacity: int | Decimal

_PredictionsCancelOrderV2ResponseOptional = TypedDict("_PredictionsCancelOrderV2ResponseOptional", {'client_order_id': 'str | None'}, total=False)
class PredictionsCancelOrderV2Response(_PredictionsCancelOrderV2ResponseOptional):
    order_id: str
    reduced_by: str
    ts_ms: int | Decimal

_PredictionsCreateApiKeyRequestOptional = TypedDict("_PredictionsCreateApiKeyRequestOptional", {'scopes': 'list[str] | None', 'subaccount': 'int | Decimal | None', 'fcm_subtrader_id': 'str | None'}, total=False)
class PredictionsCreateApiKeyRequest(_PredictionsCreateApiKeyRequestOptional):
    name: str
    public_key: str

_PredictionsCreateApiKeyResponseOptional = TypedDict("_PredictionsCreateApiKeyResponseOptional", {'warning': 'str | None'}, total=False)
class PredictionsCreateApiKeyResponse(_PredictionsCreateApiKeyResponseOptional):
    api_key_id: str

class PredictionsCreateFCMPositionTransferRequest(TypedDict):
    source_subtrader_id: str
    target_subtrader_id: str
    market_ticker: str
    position_centicount: int | Decimal
    price_centicents: int | Decimal

class PredictionsCreateFCMPositionTransferResponse(TypedDict):
    position_transfer_id: str

class PredictionsCreateFCMSubtraderRequest(TypedDict):
    subtrader_suffix: str

class PredictionsCreateFCMSubtraderResponse(TypedDict):
    subtrader_id: str

_PredictionsCreateMarketInMultivariateEventCollectionRequestOptional = TypedDict("_PredictionsCreateMarketInMultivariateEventCollectionRequestOptional", {'with_market_payload': 'bool | None'}, total=False)
class PredictionsCreateMarketInMultivariateEventCollectionRequest(_PredictionsCreateMarketInMultivariateEventCollectionRequestOptional):
    selected_markets: list[PredictionsTickerPair]

_PredictionsCreateMarketInMultivariateEventCollectionResponseOptional = TypedDict("_PredictionsCreateMarketInMultivariateEventCollectionResponseOptional", {'market': 'PredictionsMarket | None'}, total=False)
class PredictionsCreateMarketInMultivariateEventCollectionResponse(_PredictionsCreateMarketInMultivariateEventCollectionResponseOptional):
    event_ticker: str
    market_ticker: str

_PredictionsCreateOrderGroupRequestOptional = TypedDict("_PredictionsCreateOrderGroupRequestOptional", {'subaccount': 'int | Decimal | None', 'contracts_limit': 'int | Decimal | None', 'contracts_limit_fp': 'str | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsCreateOrderGroupRequest(_PredictionsCreateOrderGroupRequestOptional):
    pass

_PredictionsCreateOrderGroupResponseOptional = TypedDict("_PredictionsCreateOrderGroupResponseOptional", {'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsCreateOrderGroupResponse(_PredictionsCreateOrderGroupResponseOptional):
    order_group_id: str
    subaccount: int | Decimal

_PredictionsCreateOrderRequestOptional = TypedDict("_PredictionsCreateOrderRequestOptional", {'client_order_id': 'str | None', 'count': 'int | Decimal | None', 'count_fp': 'str | None', 'yes_price': 'int | Decimal | None', 'no_price': 'int | Decimal | None', 'yes_price_dollars': 'str | None', 'no_price_dollars': 'str | None', 'expiration_ts': 'int | Decimal | None', 'time_in_force': 'str | None', 'buy_max_cost': 'int | Decimal | None', 'post_only': 'bool | None', 'reduce_only': 'bool | None', 'sell_position_floor': 'int | Decimal | None', 'self_trade_prevention_type': 'str | None', 'order_group_id': 'str | None', 'cancel_order_on_pause': 'bool | None', 'subaccount': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsCreateOrderRequest(_PredictionsCreateOrderRequestOptional):
    ticker: str
    side: str
    action: str

class PredictionsCreateOrderResponse(TypedDict):
    order: PredictionsOrder

_PredictionsCreateOrderV2RequestOptional = TypedDict("_PredictionsCreateOrderV2RequestOptional", {'client_order_id': 'str | None', 'expiration_time': 'int | Decimal | None', 'post_only': 'bool | None', 'cancel_order_on_pause': 'bool | None', 'reduce_only': 'bool | None', 'subaccount': 'int | Decimal | None', 'order_group_id': 'str | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsCreateOrderV2Request(_PredictionsCreateOrderV2RequestOptional):
    ticker: str
    side: str
    count: str
    price: str
    time_in_force: str
    self_trade_prevention_type: str

_PredictionsCreateOrderV2ResponseOptional = TypedDict("_PredictionsCreateOrderV2ResponseOptional", {'client_order_id': 'str | None', 'average_fill_price': 'str | None', 'average_fee_paid': 'str | None'}, total=False)
class PredictionsCreateOrderV2Response(_PredictionsCreateOrderV2ResponseOptional):
    order_id: str
    fill_count: str
    remaining_count: str
    ts_ms: int | Decimal

_PredictionsCreateOrderWithExitTriggerRequestOptional = TypedDict("_PredictionsCreateOrderWithExitTriggerRequestOptional", {'client_order_id': 'str | None', 'expiration_time': 'int | Decimal | None', 'post_only': 'bool | None', 'cancel_order_on_pause': 'bool | None', 'reduce_only': 'bool | None', 'subaccount': 'int | Decimal | None', 'order_group_id': 'str | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsCreateOrderWithExitTriggerRequest(_PredictionsCreateOrderWithExitTriggerRequestOptional):
    ticker: str
    side: str
    count: str
    price: str
    time_in_force: str
    self_trade_prevention_type: str
    exit_trigger: PredictionsAttachedEventContractExitTrigger

_PredictionsCreateQuoteRequestOptional = TypedDict("_PredictionsCreateQuoteRequestOptional", {'post_only': 'bool | None', 'subaccount': 'int | Decimal | None'}, total=False)
class PredictionsCreateQuoteRequest(_PredictionsCreateQuoteRequestOptional):
    rfq_id: str
    yes_bid: str
    no_bid: str
    rest_remainder: bool

class PredictionsCreateQuoteResponse(TypedDict):
    id: str

_PredictionsCreateRFQBatchChildOptional = TypedDict("_PredictionsCreateRFQBatchChildOptional", {'market_ticker': 'str | None', 'selected_markets': 'list[PredictionsTickerPair] | None', 'contracts_fp': 'str | None', 'target_cost_dollars': 'str | None'}, total=False)
class PredictionsCreateRFQBatchChild(_PredictionsCreateRFQBatchChildOptional):
    pass

_PredictionsCreateRFQBatchQuoteRequestOptional = TypedDict("_PredictionsCreateRFQBatchQuoteRequestOptional", {'post_only': 'bool | None', 'subaccount': 'int | Decimal | None'}, total=False)
class PredictionsCreateRFQBatchQuoteRequest(_PredictionsCreateRFQBatchQuoteRequestOptional):
    quotes: list[PredictionsRFQBatchQuoteEntry]
    rest_remainder: bool

class PredictionsCreateRFQBatchQuoteResponse(TypedDict):
    id: str

_PredictionsCreateRFQBatchRequestOptional = TypedDict("_PredictionsCreateRFQBatchRequestOptional", {'mve_collection_ticker': 'str | None', 'contracts_fp': 'str | None', 'target_cost_dollars': 'str | None', 'target_cost_excludes_fees': 'bool | None', 'rest_remainder': 'bool | None', 'replace_existing': 'bool | None', 'obscure_creator_id': 'bool | None', 'client_batch_id': 'str | None', 'subtrader_id': 'str | None', 'subaccount': 'int | Decimal | None'}, total=False)
class PredictionsCreateRFQBatchRequest(_PredictionsCreateRFQBatchRequestOptional):
    children: list[PredictionsCreateRFQBatchChild]

class PredictionsCreateRFQBatchResponse(TypedDict):
    rfq_batch: PredictionsRFQBatch

_PredictionsCreateRFQRequestOptional = TypedDict("_PredictionsCreateRFQRequestOptional", {'contracts': 'int | Decimal | None', 'contracts_fp': 'str | None', 'target_cost_centi_cents': 'int | Decimal | None', 'target_cost_dollars': 'str | None', 'target_cost_excludes_fees': 'bool | None', 'obscure_creator_id': 'bool | None', 'replace_existing': 'bool | None', 'subtrader_id': 'str | None', 'subaccount': 'int | Decimal | None'}, total=False)
class PredictionsCreateRFQRequest(_PredictionsCreateRFQRequestOptional):
    market_ticker: str
    rest_remainder: bool

class PredictionsCreateRFQResponse(TypedDict):
    id: str

_PredictionsCreateSubaccountRequestOptional = TypedDict("_PredictionsCreateSubaccountRequestOptional", {'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsCreateSubaccountRequest(_PredictionsCreateSubaccountRequestOptional):
    pass

class PredictionsCreateSubaccountResponse(TypedDict):
    subaccount_number: int | Decimal

class PredictionsCreditHistory(TypedDict):
    credit_id: str
    type: str
    status: str
    amount_dollars: str
    reason: str
    created_time: str

class PredictionsDailySchedule(TypedDict):
    open_time: str
    close_time: str

_PredictionsDecreaseOrderV2RequestOptional = TypedDict("_PredictionsDecreaseOrderV2RequestOptional", {'reduce_by': 'str | None', 'reduce_to': 'str | None', 'exchange_index': 'int | Decimal | None', 'market_ticker': 'str | None'}, total=False)
class PredictionsDecreaseOrderV2Request(_PredictionsDecreaseOrderV2RequestOptional):
    pass

_PredictionsDecreaseOrderV2ResponseOptional = TypedDict("_PredictionsDecreaseOrderV2ResponseOptional", {'client_order_id': 'str | None'}, total=False)
class PredictionsDecreaseOrderV2Response(_PredictionsDecreaseOrderV2ResponseOptional):
    order_id: str
    remaining_count: str
    ts_ms: int | Decimal

_PredictionsDepositOptional = TypedDict("_PredictionsDepositOptional", {'finalized_ts': 'int | Decimal | None'}, total=False)
class PredictionsDeposit(_PredictionsDepositOptional):
    id: str
    status: str
    type: str
    amount_cents: int | Decimal
    fee_cents: int | Decimal
    created_ts: int | Decimal

class PredictionsEmptyResponse(TypedDict):
    pass

class PredictionsEndpointTokenCost(TypedDict):
    method: str
    path: str
    cost: int | Decimal

_PredictionsErrorResponseOptional = TypedDict("_PredictionsErrorResponseOptional", {'code': 'str | None', 'message': 'str | None', 'details': 'str | None'}, total=False)
class PredictionsErrorResponse(_PredictionsErrorResponseOptional):
    pass

class PredictionsEventContractExitSlippage(TypedDict):
    unit: str
    value: int | Decimal

_PredictionsEventContractExitTriggerOptional = TypedDict("_PredictionsEventContractExitTriggerOptional", {'slippage': 'PredictionsEventContractExitSlippage | None', 'stop_loss_price': 'str | None', 'take_profit_price': 'str | None', 'status_reason': 'str | None', 'triggered_order_id': 'str | None', 'triggered_leg': 'str | None'}, total=False)
class PredictionsEventContractExitTrigger(_PredictionsEventContractExitTriggerOptional):
    id: str
    ticker: str
    side: str
    subaccount_number: int | Decimal
    kind: str
    status: str
    created_time: str
    updated_time: str

_PredictionsEventDataOptional = TypedDict("_PredictionsEventDataOptional", {'category': 'str | None', 'strike_date': 'str | None', 'strike_period': 'str | None', 'markets': 'list[PredictionsMarket] | None', 'product_metadata': 'dict[str, Any] | None', 'last_updated_ts': 'str | None', 'fee_type_override': 'str | None', 'fee_multiplier_override': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsEventData(_PredictionsEventDataOptional):
    event_ticker: str
    series_ticker: str
    sub_title: str
    title: str
    collateral_return_type: str
    mutually_exclusive: bool
    settlement_sources: list[PredictionsSettlementSource] | None

class PredictionsEventFeeChange(TypedDict):
    id: str
    event_ticker: str
    series_ticker: str
    fee_type_override: str | None
    fee_multiplier_override: int | Decimal | None
    scheduled_ts: str

_PredictionsEventLiveDataOptional = TypedDict("_PredictionsEventLiveDataOptional", {'is_historical': 'bool | None', 'default_range': 'str | None', 'range_options': 'list[str] | None'}, total=False)
class PredictionsEventLiveData(_PredictionsEventLiveDataOptional):
    type: str
    details: dict[str, Any]

class PredictionsEventPosition(TypedDict):
    event_ticker: str
    total_cost_dollars: str
    total_cost_shares_fp: str
    event_exposure_dollars: str
    realized_pnl_dollars: str
    fees_paid_dollars: str

class PredictionsExchangeIndexStatus(TypedDict):
    exchange_index: int | Decimal
    description: str
    exchange_active: bool
    trading_active: bool
    intra_exchange_transfers_active: bool

_PredictionsExchangeStatusOptional = TypedDict("_PredictionsExchangeStatusOptional", {'intra_exchange_transfers_active': 'bool | None', 'exchange_estimated_resume_time': 'str | None', 'exchange_index_statuses': 'list[PredictionsExchangeIndexStatus] | None'}, total=False)
class PredictionsExchangeStatus(_PredictionsExchangeStatusOptional):
    exchange_active: bool
    trading_active: bool

class PredictionsFCMSubtrader(TypedDict):
    subtrader_id: str
    exchange_indices: list[int | Decimal]
    trading_blocked: bool
    fcm_trading_blocked: bool
    propagation_pending: bool

_PredictionsFcmFillOptional = TypedDict("_PredictionsFcmFillOptional", {'created_time': 'str | None', 'maker_order_id': 'str | None', 'maker_subtrader_id': 'str | None', 'maker_fee_cost': 'str | None', 'taker_order_id': 'str | None', 'taker_subtrader_id': 'str | None', 'taker_fee_cost': 'str | None'}, total=False)
class PredictionsFcmFill(_PredictionsFcmFillOptional):
    fill_id: str
    exchange_index: int | Decimal
    ticker: str
    taker_outcome_side: str
    count_fp: str
    yes_price_dollars: str

_PredictionsFillOptional = TypedDict("_PredictionsFillOptional", {'side': 'str | None', 'action': 'str | None', 'created_time': 'str | None', 'subaccount_number': 'int | Decimal | None', 'ts': 'int | Decimal | None'}, total=False)
class PredictionsFill(_PredictionsFillOptional):
    fill_id: str
    exchange_index: int | Decimal
    trade_id: str
    order_id: str
    ticker: str
    market_ticker: str
    outcome_side: str
    book_side: str
    count_fp: str
    yes_price_dollars: str
    no_price_dollars: str
    is_taker: bool
    fee_cost: str

class PredictionsForecastPercentilesPoint(TypedDict):
    event_ticker: str
    end_period_ts: int | Decimal
    period_interval: int | Decimal
    percentile_points: list[PredictionsPercentilePoint]

_PredictionsGenerateApiKeyRequestOptional = TypedDict("_PredictionsGenerateApiKeyRequestOptional", {'key_type': 'str | None', 'scopes': 'list[str] | None', 'subaccount': 'int | Decimal | None', 'fcm_subtrader_id': 'str | None'}, total=False)
class PredictionsGenerateApiKeyRequest(_PredictionsGenerateApiKeyRequestOptional):
    name: str

_PredictionsGenerateApiKeyResponseOptional = TypedDict("_PredictionsGenerateApiKeyResponseOptional", {'key_type': 'str | None', 'warning': 'str | None'}, total=False)
class PredictionsGenerateApiKeyResponse(_PredictionsGenerateApiKeyResponseOptional):
    api_key_id: str
    private_key: str

class PredictionsGetAccountApiLimitsResponse(TypedDict):
    usage_tier: str
    read: PredictionsBucketLimit
    write: PredictionsBucketLimit
    grants: list[PredictionsApiUsageLevelGrant]

class PredictionsGetAccountApiUsageLevelVolumeProgressResponse(TypedDict):
    volume_progress: list[PredictionsAccountApiUsageLevelVolumeProgress]

class PredictionsGetAccountEndpointCostsResponse(TypedDict):
    default_cost: int | Decimal
    endpoint_costs: list[PredictionsEndpointTokenCost]

_PredictionsGetApiKeysResponseOptional = TypedDict("_PredictionsGetApiKeysResponseOptional", {'api_key_region_expiration_ts': 'int | Decimal | None'}, total=False)
class PredictionsGetApiKeysResponse(_PredictionsGetApiKeysResponseOptional):
    api_keys: list[PredictionsApiKey]

_PredictionsGetBalanceResponseOptional = TypedDict("_PredictionsGetBalanceResponseOptional", {'balance_breakdown': 'list[PredictionsIndexedBalance] | None'}, total=False)
class PredictionsGetBalanceResponse(_PredictionsGetBalanceResponseOptional):
    balance: int | Decimal
    balance_dollars: str
    portfolio_value: int | Decimal
    updated_ts: int | Decimal

_PredictionsGetBlockTradeProposalsResponseOptional = TypedDict("_PredictionsGetBlockTradeProposalsResponseOptional", {'cursor': 'str | None'}, total=False)
class PredictionsGetBlockTradeProposalsResponse(_PredictionsGetBlockTradeProposalsResponseOptional):
    block_trade_proposals: list[PredictionsBlockTradeProposal]

class PredictionsGetCommunicationsIDResponse(TypedDict):
    communications_id: str

class PredictionsGetCreditHistoryResponse(TypedDict):
    credits: list[PredictionsCreditHistory]
    cursor: str

_PredictionsGetDepositsResponseOptional = TypedDict("_PredictionsGetDepositsResponseOptional", {'cursor': 'str | None'}, total=False)
class PredictionsGetDepositsResponse(_PredictionsGetDepositsResponseOptional):
    deposits: list[PredictionsDeposit]

class PredictionsGetEventCandlesticksResponse(TypedDict):
    market_tickers: list[str]
    market_candlesticks: list[list[PredictionsMarketCandlestick]]
    adjusted_end_ts: int | Decimal

class PredictionsGetEventFeeChangesResponse(TypedDict):
    event_fee_changes: list[PredictionsEventFeeChange]
    cursor: str

class PredictionsGetEventForecastPercentilesHistoryResponse(TypedDict):
    forecast_history: list[PredictionsForecastPercentilesPoint]

class PredictionsGetEventLiveDataResponse(TypedDict):
    live_data: PredictionsEventLiveData

_PredictionsGetEventMetadataResponseOptional = TypedDict("_PredictionsGetEventMetadataResponseOptional", {'featured_image_url': 'str | None', 'competition': 'str | None', 'competition_scope': 'str | None'}, total=False)
class PredictionsGetEventMetadataResponse(_PredictionsGetEventMetadataResponseOptional):
    image_url: str
    market_details: list[PredictionsMarketMetadata]
    settlement_sources: list[PredictionsSettlementSource]

class PredictionsGetEventResponse(TypedDict):
    event: PredictionsEventData
    markets: list[PredictionsMarket]

_PredictionsGetEventsResponseOptional = TypedDict("_PredictionsGetEventsResponseOptional", {'milestones': 'list[PredictionsMilestone] | None'}, total=False)
class PredictionsGetEventsResponse(_PredictionsGetEventsResponseOptional):
    events: list[PredictionsEventData]
    cursor: str

class PredictionsGetExchangeScheduleResponse(TypedDict):
    schedule: PredictionsSchedule

class PredictionsGetFCMEventContractDailyCapResponse(TypedDict):
    subtrader_id: str
    limit: str
    executed_utilization: str
    resting_order_utilization: str
    pending_order_utilization: str
    cap_date: str

class PredictionsGetFCMSubtraderBlockedCategoriesResponse(TypedDict):
    categories: list[str]

class PredictionsGetFcmFillsResponse(TypedDict):
    fills: list[PredictionsFcmFill]
    cursor: str

class PredictionsGetFillsResponse(TypedDict):
    fills: list[PredictionsFill]
    cursor: str

class PredictionsGetFiltersBySportsResponse(TypedDict):
    filters_by_sports: dict[str, PredictionsSportFilterDetails]
    sport_ordering: list[str]

_PredictionsGetGameStatsResponseOptional = TypedDict("_PredictionsGetGameStatsResponseOptional", {'pbp': 'PredictionsPlayByPlay | None'}, total=False)
class PredictionsGetGameStatsResponse(_PredictionsGetGameStatsResponseOptional):
    pass

_PredictionsGetHistoricalCutoffResponseOptional = TypedDict("_PredictionsGetHistoricalCutoffResponseOptional", {'market_positions_last_updated_ts': 'str | None'}, total=False)
class PredictionsGetHistoricalCutoffResponse(_PredictionsGetHistoricalCutoffResponseOptional):
    market_settled_ts: str
    trades_created_ts: str
    orders_updated_ts: str

_PredictionsGetIncentiveProgramsResponseOptional = TypedDict("_PredictionsGetIncentiveProgramsResponseOptional", {'next_cursor': 'str | None'}, total=False)
class PredictionsGetIncentiveProgramsResponse(_PredictionsGetIncentiveProgramsResponseOptional):
    incentive_programs: list[PredictionsIncentiveProgram]

class PredictionsGetIntraExchangeInstanceTransferResponse(TypedDict):
    transfer: PredictionsIntraExchangeInstanceTransfer

_PredictionsGetIntraExchangeInstanceTransfersResponseOptional = TypedDict("_PredictionsGetIntraExchangeInstanceTransfersResponseOptional", {'cursor': 'str | None'}, total=False)
class PredictionsGetIntraExchangeInstanceTransfersResponse(_PredictionsGetIntraExchangeInstanceTransfersResponseOptional):
    transfers: list[PredictionsIntraExchangeInstanceTransfer]

class PredictionsGetLiveDataResponse(TypedDict):
    live_data: PredictionsLiveData

class PredictionsGetLiveDatasResponse(TypedDict):
    live_datas: list[PredictionsLiveData]

class PredictionsGetMarketCandlesticksHistoricalResponse(TypedDict):
    ticker: str
    candlesticks: list[PredictionsMarketCandlestickHistorical]

class PredictionsGetMarketCandlesticksResponse(TypedDict):
    ticker: str
    candlesticks: list[PredictionsMarketCandlestick]

_PredictionsGetMarketIndexHistoryResponseOptional = TypedDict("_PredictionsGetMarketIndexHistoryResponseOptional", {'last_tick_ms': 'int | Decimal | None'}, total=False)
class PredictionsGetMarketIndexHistoryResponse(_PredictionsGetMarketIndexHistoryResponseOptional):
    id: str
    source: str
    status: str
    interval: str
    start_ms: int | Decimal
    end_ms: int | Decimal
    timeseries: list[PredictionsMarketIndexTimeseriesPoint]

_PredictionsGetMarketIndexResponseOptional = TypedDict("_PredictionsGetMarketIndexResponseOptional", {'value': 'int | Decimal | None', 'last_tick_ms': 'int | Decimal | None'}, total=False)
class PredictionsGetMarketIndexResponse(_PredictionsGetMarketIndexResponseOptional):
    index: PredictionsMarketIndex
    source: str
    status: str
    interval: str
    start_ms: int | Decimal
    end_ms: int | Decimal
    timeseries: list[PredictionsMarketIndexTimeseriesPoint]

class PredictionsGetMarketIndexesResponse(TypedDict):
    indexes: list[PredictionsMarketIndex]

class PredictionsGetMarketOrderbookResponse(TypedDict):
    orderbook_fp: PredictionsOrderbookCountFp

class PredictionsGetMarketOrderbooksResponse(TypedDict):
    orderbooks: list[PredictionsMarketOrderbookFp]

class PredictionsGetMarketResponse(TypedDict):
    market: PredictionsMarket

class PredictionsGetMarketsResponse(TypedDict):
    markets: list[PredictionsMarket]
    cursor: str

class PredictionsGetMilestoneResponse(TypedDict):
    milestone: PredictionsMilestone

_PredictionsGetMilestonesResponseOptional = TypedDict("_PredictionsGetMilestonesResponseOptional", {'cursor': 'str | None'}, total=False)
class PredictionsGetMilestonesResponse(_PredictionsGetMilestonesResponseOptional):
    milestones: list[PredictionsMilestone]

class PredictionsGetMultivariateEventCollectionResponse(TypedDict):
    multivariate_contract: PredictionsMultivariateEventCollection

_PredictionsGetMultivariateEventCollectionsResponseOptional = TypedDict("_PredictionsGetMultivariateEventCollectionsResponseOptional", {'cursor': 'str | None'}, total=False)
class PredictionsGetMultivariateEventCollectionsResponse(_PredictionsGetMultivariateEventCollectionsResponseOptional):
    multivariate_contracts: list[PredictionsMultivariateEventCollection]

class PredictionsGetMultivariateEventsResponse(TypedDict):
    events: list[PredictionsEventData]
    cursor: str

_PredictionsGetOrderGroupResponseOptional = TypedDict("_PredictionsGetOrderGroupResponseOptional", {'contracts_limit_fp': 'str | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsGetOrderGroupResponse(_PredictionsGetOrderGroupResponseOptional):
    is_auto_cancel_enabled: bool
    orders: list[str]

_PredictionsGetOrderGroupsResponseOptional = TypedDict("_PredictionsGetOrderGroupsResponseOptional", {'order_groups': 'list[PredictionsOrderGroup] | None'}, total=False)
class PredictionsGetOrderGroupsResponse(_PredictionsGetOrderGroupsResponseOptional):
    pass

class PredictionsGetOrderQueuePositionResponse(TypedDict):
    queue_position_fp: str

class PredictionsGetOrderQueuePositionsResponse(TypedDict):
    queue_positions: list[PredictionsOrderQueuePosition]

class PredictionsGetOrderResponse(TypedDict):
    order: PredictionsOrder

class PredictionsGetOrdersResponse(TypedDict):
    orders: list[PredictionsOrder]
    cursor: str

class PredictionsGetPortfolioRestingOrderTotalValueResponse(TypedDict):
    total_resting_order_value: int | Decimal
    resting_order_value_breakdown: list[PredictionsIndexedBalance]

_PredictionsGetPositionsResponseOptional = TypedDict("_PredictionsGetPositionsResponseOptional", {'cursor': 'str | None'}, total=False)
class PredictionsGetPositionsResponse(_PredictionsGetPositionsResponseOptional):
    market_positions: list[PredictionsMarketPosition]
    event_positions: list[PredictionsEventPosition]

class PredictionsGetQuoteResponse(TypedDict):
    quote: PredictionsQuote

_PredictionsGetQuotesResponseOptional = TypedDict("_PredictionsGetQuotesResponseOptional", {'cursor': 'str | None'}, total=False)
class PredictionsGetQuotesResponse(_PredictionsGetQuotesResponseOptional):
    quotes: list[PredictionsQuote]

class PredictionsGetRFQBatchBestQuotesResponse(TypedDict):
    best_quotes: PredictionsRFQBatchBestQuotes

class PredictionsGetRFQBatchQuotesResponse(TypedDict):
    batch_quotes: list[PredictionsRFQBatchQuote]

class PredictionsGetRFQBatchResponse(TypedDict):
    rfq_batch: PredictionsRFQBatch

_PredictionsGetRFQBatchesResponseOptional = TypedDict("_PredictionsGetRFQBatchesResponseOptional", {'cursor': 'str | None'}, total=False)
class PredictionsGetRFQBatchesResponse(_PredictionsGetRFQBatchesResponseOptional):
    rfq_batches: list[PredictionsRFQBatch]

class PredictionsGetRFQResponse(TypedDict):
    rfq: PredictionsRFQ

_PredictionsGetRFQsResponseOptional = TypedDict("_PredictionsGetRFQsResponseOptional", {'cursor': 'str | None'}, total=False)
class PredictionsGetRFQsResponse(_PredictionsGetRFQsResponseOptional):
    rfqs: list[PredictionsRFQ]

class PredictionsGetSeriesFeeChangesResponse(TypedDict):
    series_fee_change_arr: list[PredictionsSeriesFeeChange]

class PredictionsGetSeriesListResponse(TypedDict):
    series: list[PredictionsSeries]

class PredictionsGetSeriesResponse(TypedDict):
    series: PredictionsSeries

_PredictionsGetSettlementsResponseOptional = TypedDict("_PredictionsGetSettlementsResponseOptional", {'cursor': 'str | None'}, total=False)
class PredictionsGetSettlementsResponse(_PredictionsGetSettlementsResponseOptional):
    settlements: list[PredictionsSettlement]

_PredictionsGetStructuredTargetResponseOptional = TypedDict("_PredictionsGetStructuredTargetResponseOptional", {'structured_target': 'PredictionsStructuredTarget | None'}, total=False)
class PredictionsGetStructuredTargetResponse(_PredictionsGetStructuredTargetResponseOptional):
    pass

_PredictionsGetStructuredTargetsResponseOptional = TypedDict("_PredictionsGetStructuredTargetsResponseOptional", {'structured_targets': 'list[PredictionsStructuredTarget] | None', 'cursor': 'str | None'}, total=False)
class PredictionsGetStructuredTargetsResponse(_PredictionsGetStructuredTargetsResponseOptional):
    pass

class PredictionsGetSubaccountBalancesResponse(TypedDict):
    subaccount_balances: list[PredictionsSubaccountBalance]

class PredictionsGetSubaccountNettingResponse(TypedDict):
    netting_configs: list[PredictionsSubaccountNettingConfig]

_PredictionsGetSubaccountTransfersResponseOptional = TypedDict("_PredictionsGetSubaccountTransfersResponseOptional", {'cursor': 'str | None'}, total=False)
class PredictionsGetSubaccountTransfersResponse(_PredictionsGetSubaccountTransfersResponseOptional):
    transfers: list[PredictionsSubaccountTransfer]

class PredictionsGetTagsForSeriesCategoriesResponse(TypedDict):
    tags_by_categories: dict[str, list[str]]

class PredictionsGetTargetBalanceAllocationResponse(TypedDict):
    allocations: list[PredictionsTargetBalanceAllocation]
    resting_margin_reservation: str

class PredictionsGetTradesResponse(TypedDict):
    trades: list[PredictionsTrade]
    cursor: str

class PredictionsGetUserDataTimestampResponse(TypedDict):
    as_of_time: str

class PredictionsGetWeatherIndexCalibrationsResponse(TypedDict):
    city: str
    units: str
    calibrations: list[PredictionsWeatherIndexCalibration]

_PredictionsGetWeatherIndexResponseOptional = TypedDict("_PredictionsGetWeatherIndexResponseOptional", {'config_version': 'str | None'}, total=False)
class PredictionsGetWeatherIndexResponse(_PredictionsGetWeatherIndexResponseOptional):
    city: str
    units: str
    timeseries: list[PredictionsWeatherIndexPoint]

_PredictionsGetWithdrawalsResponseOptional = TypedDict("_PredictionsGetWithdrawalsResponseOptional", {'cursor': 'str | None'}, total=False)
class PredictionsGetWithdrawalsResponse(_PredictionsGetWithdrawalsResponseOptional):
    withdrawals: list[PredictionsWithdrawal]

_PredictionsIncentiveProgramOptional = TypedDict("_PredictionsIncentiveProgramOptional", {'discount_factor_bps': 'int | Decimal | None', 'target_size_fp': 'str | None', 'max_reward_per_account': 'int | Decimal | None'}, total=False)
class PredictionsIncentiveProgram(_PredictionsIncentiveProgramOptional):
    id: str
    market_id: str
    market_ticker: str
    incentive_type: str
    incentive_description: str
    start_date: str
    end_date: str
    period_reward: int | Decimal
    paid_out: bool

class PredictionsIndexedBalance(TypedDict):
    exchange_index: int | Decimal
    balance: str

_PredictionsIntraExchangeInstanceTransferOptional = TypedDict("_PredictionsIntraExchangeInstanceTransferOptional", {'client_transfer_id': 'str | None'}, total=False)
class PredictionsIntraExchangeInstanceTransfer(_PredictionsIntraExchangeInstanceTransferOptional):
    transfer_id: str
    source: str
    destination: str
    source_exchange_shard: int | Decimal
    destination_exchange_shard: int | Decimal
    amount: str
    status: str
    created_ts: int | Decimal

_PredictionsIntraExchangeInstanceTransferRequestOptional = TypedDict("_PredictionsIntraExchangeInstanceTransferRequestOptional", {'client_transfer_id': 'str | None', 'source_exchange_shard': 'int | Decimal | None', 'destination_exchange_shard': 'int | Decimal | None', 'source_subaccount': 'int | Decimal | None', 'destination_subaccount': 'int | Decimal | None'}, total=False)
class PredictionsIntraExchangeInstanceTransferRequest(_PredictionsIntraExchangeInstanceTransferRequestOptional):
    source: str
    destination: str
    amount: int | Decimal

class PredictionsIntraExchangeInstanceTransferResponse(TypedDict):
    transfer_id: str

class PredictionsListFCMSubtradersResponse(TypedDict):
    subtraders: list[PredictionsFCMSubtrader]

class PredictionsLiveData(TypedDict):
    type: str
    details: dict[str, Any]
    milestone_id: str

class PredictionsMaintenanceWindow(TypedDict):
    start_datetime: str
    end_datetime: str

_PredictionsMarketOptional = TypedDict("_PredictionsMarketOptional", {'title': 'str | None', 'subtitle': 'str | None', 'expected_expiration_time': 'str | None', 'expiration_time': 'str | None', 'settlement_value_dollars': 'str | None', 'settlement_ts': 'str | None', 'settlement_floor_dollars': 'str | None', 'occurrence_datetime': 'str | None', 'fee_waiver_expiration_time': 'str | None', 'early_close_condition': 'str | None', 'strike_type': 'str | None', 'floor_strike': 'int | Decimal | None', 'cap_strike': 'int | Decimal | None', 'functional_strike': 'str | None', 'custom_strike': 'dict[str, Any] | None', 'mve_collection_ticker': 'str | None', 'mve_selected_legs': 'list[PredictionsMveSelectedLeg] | None', 'primary_participant_key': 'str | None', 'is_provisional': 'bool | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsMarket(_PredictionsMarketOptional):
    ticker: str
    event_ticker: str
    market_type: str
    yes_sub_title: str
    no_sub_title: str
    created_time: str
    updated_time: str
    open_time: str
    close_time: str
    latest_expiration_time: str
    settlement_timer_seconds: int | Decimal
    status: str
    yes_bid_dollars: str
    yes_bid_size_fp: str
    yes_ask_dollars: str
    yes_ask_size_fp: str
    no_bid_dollars: str
    no_ask_dollars: str
    last_price_dollars: str
    volume_fp: str
    volume_24h_fp: str
    result: str
    can_close_early: bool
    open_interest_fp: str
    notional_value_dollars: str
    previous_yes_bid_dollars: str
    previous_yes_ask_dollars: str
    previous_price_dollars: str
    settlement_bounds_type: str
    expiration_value: str
    rules_primary: str
    rules_secondary: str
    price_level_structure: str
    price_ranges: list[PredictionsPriceRange]

class PredictionsMarketCandlestick(TypedDict):
    end_period_ts: int | Decimal
    yes_bid: PredictionsBidAskDistribution
    yes_ask: PredictionsBidAskDistribution
    price: PredictionsPriceDistribution
    volume_fp: str
    open_interest_fp: str

class PredictionsMarketCandlestickHistorical(TypedDict):
    end_period_ts: int | Decimal
    yes_bid: PredictionsBidAskDistributionHistorical
    yes_ask: PredictionsBidAskDistributionHistorical
    price: PredictionsPriceDistributionHistorical
    volume: str
    open_interest: str

class PredictionsMarketCandlesticksResponse(TypedDict):
    market_ticker: str
    candlesticks: list[PredictionsMarketCandlestick]

_PredictionsMarketIndexOptional = TypedDict("_PredictionsMarketIndexOptional", {'gics_code': 'str | None', 'sector': 'str | None', 'latest': 'PredictionsMarketIndexLatest | None'}, total=False)
class PredictionsMarketIndex(_PredictionsMarketIndexOptional):
    id: str
    name: str
    family: str
    currency: str
    units: str
    session: PredictionsMarketIndexSession
    available: bool

_PredictionsMarketIndexLatestOptional = TypedDict("_PredictionsMarketIndexLatestOptional", {'value': 'int | Decimal | None', 'last_tick_ms': 'int | Decimal | None'}, total=False)
class PredictionsMarketIndexLatest(_PredictionsMarketIndexLatestOptional):
    source: str
    status: str

_PredictionsMarketIndexSessionOptional = TypedDict("_PredictionsMarketIndexSessionOptional", {'calendar': 'str | None'}, total=False)
class PredictionsMarketIndexSession(_PredictionsMarketIndexSessionOptional):
    timezone: str
    open: str
    close: str
    days: list[str]

class PredictionsMarketIndexTimeseriesPoint(TypedDict):
    t: int | Decimal
    v: int | Decimal

class PredictionsMarketMetadata(TypedDict):
    market_ticker: str
    image_url: str
    color_code: str

class PredictionsMarketOrderbookFp(TypedDict):
    ticker: str
    orderbook_fp: PredictionsOrderbookCountFp

class PredictionsMarketPosition(TypedDict):
    ticker: str
    exchange_index: int | Decimal
    total_traded_dollars: str
    position_fp: str
    market_exposure_dollars: str
    realized_pnl_dollars: str
    fees_paid_dollars: str
    last_updated_ts: str
    netting_enabled: bool

_PredictionsMilestoneOptional = TypedDict("_PredictionsMilestoneOptional", {'end_date': 'str | None', 'source_id': 'str | None', 'source_ids': 'dict[str, str] | None'}, total=False)
class PredictionsMilestone(_PredictionsMilestoneOptional):
    id: str
    category: str
    type: str
    start_date: str
    related_event_tickers: list[str]
    title: str
    notification_message: str
    details: dict[str, Any]
    primary_event_tickers: list[str]
    last_updated_ts: str

_PredictionsMultivariateEventCollectionOptional = TypedDict("_PredictionsMultivariateEventCollectionOptional", {'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsMultivariateEventCollection(_PredictionsMultivariateEventCollectionOptional):
    collection_ticker: str
    series_ticker: str
    title: str
    description: str
    open_date: str
    close_date: str
    associated_events: list[PredictionsAssociatedEvent]
    associated_event_tickers: list[str]
    is_ordered: bool
    is_single_market_per_event: bool
    is_all_yes: bool
    size_min: int | Decimal
    size_max: int | Decimal
    functional_description: str
    price_level_structure: str
    price_ranges: list[PredictionsPriceRange]

_PredictionsMveSelectedLegOptional = TypedDict("_PredictionsMveSelectedLegOptional", {'event_ticker': 'str | None', 'market_ticker': 'str | None', 'side': 'str | None', 'yes_settlement_value_dollars': 'str | None'}, total=False)
class PredictionsMveSelectedLeg(_PredictionsMveSelectedLegOptional):
    pass

_PredictionsOrderOptional = TypedDict("_PredictionsOrderOptional", {'side': 'str | None', 'action': 'str | None', 'expiration_time': 'str | None', 'created_time': 'str | None', 'last_update_time': 'str | None', 'self_trade_prevention_type': 'str | None', 'order_group_id': 'str | None', 'cancel_order_on_pause': 'bool | None', 'subaccount_number': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsOrder(_PredictionsOrderOptional):
    order_id: str
    user_id: str
    client_order_id: str
    ticker: str
    outcome_side: str
    book_side: str
    type: str
    status: str
    yes_price_dollars: str
    no_price_dollars: str
    fill_count_fp: str
    remaining_count_fp: str
    initial_count_fp: str
    taker_fill_cost_dollars: str
    maker_fill_cost_dollars: str
    taker_fees_dollars: str
    maker_fees_dollars: str

_PredictionsOrderGroupOptional = TypedDict("_PredictionsOrderGroupOptional", {'contracts_limit_fp': 'str | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsOrderGroup(_PredictionsOrderGroupOptional):
    id: str
    is_auto_cancel_enabled: bool

class PredictionsOrderQueuePosition(TypedDict):
    order_id: str
    market_ticker: str
    queue_position_fp: str

class PredictionsOrderbookCountFp(TypedDict):
    yes_dollars: list[PredictionsPriceLevelDollarsCountFp]
    no_dollars: list[PredictionsPriceLevelDollarsCountFp]

class PredictionsPercentilePoint(TypedDict):
    percentile: int | Decimal
    raw_numerical_forecast: int | Decimal
    numerical_forecast: int | Decimal
    formatted_forecast: str

_PredictionsPlayByPlayOptional = TypedDict("_PredictionsPlayByPlayOptional", {'periods': 'list[PredictionsPlayByPlayPeriodsItem] | None'}, total=False)
class PredictionsPlayByPlay(_PredictionsPlayByPlayOptional):
    pass

_PredictionsPlayByPlayPeriodsItemOptional = TypedDict("_PredictionsPlayByPlayPeriodsItemOptional", {'events': 'list[dict[str, Any]] | None'}, total=False)
class PredictionsPlayByPlayPeriodsItem(_PredictionsPlayByPlayPeriodsItemOptional):
    pass

_PredictionsPriceDistributionOptional = TypedDict("_PredictionsPriceDistributionOptional", {'open_dollars': 'str | None', 'low_dollars': 'str | None', 'high_dollars': 'str | None', 'close_dollars': 'str | None', 'mean_dollars': 'str | None', 'previous_dollars': 'str | None', 'min_dollars': 'str | None', 'max_dollars': 'str | None'}, total=False)
class PredictionsPriceDistribution(_PredictionsPriceDistributionOptional):
    pass

class PredictionsPriceDistributionHistorical(TypedDict):
    open: str | None
    low: str | None
    high: str | None
    close: str | None
    mean: str | None
    previous: str | None

class PredictionsPriceRange(TypedDict):
    start: str
    end: str
    step: str

_PredictionsProposeBlockTradeRequestOptional = TypedDict("_PredictionsProposeBlockTradeRequestOptional", {'buyer_subtrader_id': 'str | None', 'buyer_subaccount': 'int | Decimal | None', 'seller_subtrader_id': 'str | None', 'seller_subaccount': 'int | Decimal | None'}, total=False)
class PredictionsProposeBlockTradeRequest(_PredictionsProposeBlockTradeRequestOptional):
    buyer_user_id: str
    seller_user_id: str
    market_ticker: str
    price_centi_cents: int | Decimal
    centicount: int | Decimal
    maker_side: str
    expiration_ts: str

class PredictionsProposeBlockTradeResponse(TypedDict):
    block_trade_proposal_id: str

_PredictionsQuoteOptional = TypedDict("_PredictionsQuoteOptional", {'accepted_side': 'str | None', 'accepted_ts': 'str | None', 'confirmed_ts': 'str | None', 'executed_ts': 'str | None', 'cancelled_ts': 'str | None', 'rest_remainder': 'bool | None', 'post_only': 'bool | None', 'cancellation_reason': 'str | None', 'creator_user_id': 'str | None', 'rfq_creator_user_id': 'str | None', 'rfq_target_cost_dollars': 'str | None', 'target_cost_excludes_fees': 'bool | None', 'rfq_creator_order_id': 'str | None', 'creator_order_id': 'str | None', 'creator_subaccount': 'int | Decimal | None', 'rfq_creator_subaccount': 'int | Decimal | None', 'yes_contracts_fp': 'str | None', 'no_contracts_fp': 'str | None'}, total=False)
class PredictionsQuote(_PredictionsQuoteOptional):
    id: str
    rfq_id: str
    creator_id: str
    rfq_creator_id: str
    market_ticker: str
    contracts_fp: str
    yes_bid_dollars: str
    no_bid_dollars: str
    created_ts: str
    updated_ts: str
    status: str

_PredictionsRFQOptional = TypedDict("_PredictionsRFQOptional", {'target_cost_dollars': 'str | None', 'target_cost_excludes_fees': 'bool | None', 'mve_collection_ticker': 'str | None', 'mve_selected_legs': 'list[PredictionsMveSelectedLeg] | None', 'rest_remainder': 'bool | None', 'cancellation_reason': 'str | None', 'creator_user_id': 'str | None', 'creator_subaccount': 'int | Decimal | None', 'cancelled_ts': 'str | None', 'updated_ts': 'str | None', 'rfq_batch_id': 'str | None'}, total=False)
class PredictionsRFQ(_PredictionsRFQOptional):
    id: str
    creator_id: str
    market_ticker: str
    contracts_fp: str
    status: str
    created_ts: str

_PredictionsRFQBatchOptional = TypedDict("_PredictionsRFQBatchOptional", {'accepted_ts': 'str | None', 'closed_ts': 'str | None', 'updated_ts': 'str | None', 'close_reason': 'str | None', 'client_batch_id': 'str | None', 'creator_user_id': 'str | None', 'creator_subaccount': 'int | Decimal | None'}, total=False)
class PredictionsRFQBatch(_PredictionsRFQBatchOptional):
    id: str
    creator_id: str
    status: str
    created_ts: str
    rfqs: list[PredictionsRFQ]

class PredictionsRFQBatchAcceptLimit(TypedDict):
    rfq_id: str
    bid_dollars: str

_PredictionsRFQBatchBestQuoteOptional = TypedDict("_PredictionsRFQBatchBestQuoteOptional", {'yes_bid_dollars': 'str | None', 'no_bid_dollars': 'str | None', 'yes_contracts_offered_fp': 'str | None', 'no_contracts_offered_fp': 'str | None'}, total=False)
class PredictionsRFQBatchBestQuote(_PredictionsRFQBatchBestQuoteOptional):
    rfq_id: str
    market_ticker: str

_PredictionsRFQBatchBestQuotesOptional = TypedDict("_PredictionsRFQBatchBestQuotesOptional", {'updated_ts': 'str | None'}, total=False)
class PredictionsRFQBatchBestQuotes(_PredictionsRFQBatchBestQuotesOptional):
    rfq_batch_id: str
    status: str
    version: int | Decimal
    yes_complete: bool
    no_complete: bool
    rfqs: list[PredictionsRFQBatchBestQuote]

_PredictionsRFQBatchQuoteOptional = TypedDict("_PredictionsRFQBatchQuoteOptional", {'accepted_side': 'str | None', 'accepted_ts': 'str | None', 'confirmed_ts': 'str | None', 'rest_remainder': 'bool | None', 'post_only': 'bool | None', 'creator_user_id': 'str | None', 'creator_subaccount': 'int | Decimal | None', 'rfq_creator_user_id': 'str | None', 'rfq_creator_subaccount': 'int | Decimal | None'}, total=False)
class PredictionsRFQBatchQuote(_PredictionsRFQBatchQuoteOptional):
    id: str
    rfq_batch_id: str
    creator_id: str
    rfq_creator_id: str
    status: str
    created_ts: str
    updated_ts: str
    quotes: list[PredictionsQuote]

class PredictionsRFQBatchQuoteEntry(TypedDict):
    rfq_id: str
    yes_bid: str
    no_bid: str

class PredictionsSchedule(TypedDict):
    standard_hours: list[PredictionsWeeklySchedule]
    maintenance_windows: list[PredictionsMaintenanceWindow]

class PredictionsScopeList(TypedDict):
    scopes: list[str]

_PredictionsSeriesOptional = TypedDict("_PredictionsSeriesOptional", {'product_metadata': 'dict[str, Any] | None', 'volume_fp': 'str | None', 'last_updated_ts': 'str | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PredictionsSeries(_PredictionsSeriesOptional):
    ticker: str
    frequency: str
    title: str
    category: str
    categories: list[str]
    tags: list[str] | None
    settlement_sources: list[PredictionsSettlementSource] | None
    contract_url: str
    contract_terms_url: str
    fee_type: str
    fee_multiplier: int | Decimal
    additional_prohibitions: list[str] | None

class PredictionsSeriesFeeChange(TypedDict):
    id: str
    series_ticker: str
    fee_type: str
    fee_multiplier: int | Decimal
    scheduled_ts: str

_PredictionsSetEventContractExitTriggerRequestOptional = TypedDict("_PredictionsSetEventContractExitTriggerRequestOptional", {'slippage': 'PredictionsEventContractExitSlippage | None', 'kind': 'str | None', 'stop_loss_price': 'str | None', 'take_profit_price': 'str | None'}, total=False)
class PredictionsSetEventContractExitTriggerRequest(_PredictionsSetEventContractExitTriggerRequestOptional):
    pass

_PredictionsSetTargetBalanceAllocationRequestOptional = TypedDict("_PredictionsSetTargetBalanceAllocationRequestOptional", {'resting_margin_reservation': 'str | None'}, total=False)
class PredictionsSetTargetBalanceAllocationRequest(_PredictionsSetTargetBalanceAllocationRequestOptional):
    allocations: list[PredictionsTargetBalanceAllocationInput]

_PredictionsSettlementOptional = TypedDict("_PredictionsSettlementOptional", {'value': 'int | Decimal | None'}, total=False)
class PredictionsSettlement(_PredictionsSettlementOptional):
    ticker: str
    exchange_index: int | Decimal
    event_ticker: str
    market_result: str
    yes_count_fp: str
    yes_total_cost_dollars: str
    no_count_fp: str
    no_total_cost_dollars: str
    revenue: int | Decimal
    settled_time: str
    fee_cost: str

_PredictionsSettlementSourceOptional = TypedDict("_PredictionsSettlementSourceOptional", {'name': 'str | None', 'url': 'str | None'}, total=False)
class PredictionsSettlementSource(_PredictionsSettlementSourceOptional):
    pass

class PredictionsSportFilterDetails(TypedDict):
    scopes: list[str]
    competitions: dict[str, PredictionsScopeList]

_PredictionsStructuredTargetOptional = TypedDict("_PredictionsStructuredTargetOptional", {'id': 'str | None', 'name': 'str | None', 'type': 'str | None', 'details': 'dict[str, Any] | None', 'source_id': 'str | None', 'source_ids': 'dict[str, str] | None', 'last_updated_ts': 'str | None'}, total=False)
class PredictionsStructuredTarget(_PredictionsStructuredTargetOptional):
    pass

class PredictionsSubaccountBalance(TypedDict):
    subaccount_number: int | Decimal
    exchange_index: int | Decimal
    balance: str
    updated_ts: int | Decimal

class PredictionsSubaccountNettingConfig(TypedDict):
    subaccount_number: int | Decimal
    enabled: bool
    exchange_index: int | Decimal

class PredictionsSubaccountTransfer(TypedDict):
    transfer_id: str
    from_subaccount: int | Decimal
    to_subaccount: int | Decimal
    amount_cents: int | Decimal
    created_ts: int | Decimal
    exchange_index: int | Decimal

class PredictionsTargetBalanceAllocation(TypedDict):
    exchange_index: int | Decimal
    percent: int | Decimal

class PredictionsTargetBalanceAllocationInput(TypedDict):
    exchange_index: int | Decimal
    percent: int | Decimal

class PredictionsTickerPair(TypedDict):
    market_ticker: str
    event_ticker: str
    side: str

_PredictionsTradeOptional = TypedDict("_PredictionsTradeOptional", {'taker_side': 'str | None'}, total=False)
class PredictionsTrade(_PredictionsTradeOptional):
    trade_id: str
    ticker: str
    count_fp: str
    yes_price_dollars: str
    no_price_dollars: str
    taker_outcome_side: str
    taker_book_side: str
    created_time: str
    is_block_trade: bool

class PredictionsUpdateFCMEventContractDailyCapRequest(TypedDict):
    subtrader_id: str
    limit: str

class PredictionsUpdateFCMSubtraderBlockedCategoriesRequest(TypedDict):
    subtrader_id: str
    category: str
    blocked: bool

class PredictionsUpdateFCMSubtraderBlockedCategoriesResponse(TypedDict):
    categories: list[str]

_PredictionsUpdateOrderGroupLimitRequestOptional = TypedDict("_PredictionsUpdateOrderGroupLimitRequestOptional", {'contracts_limit': 'int | Decimal | None', 'contracts_limit_fp': 'str | None'}, total=False)
class PredictionsUpdateOrderGroupLimitRequest(_PredictionsUpdateOrderGroupLimitRequestOptional):
    pass

class PredictionsUpdateSubaccountNettingRequest(TypedDict):
    subaccount_number: int | Decimal
    enabled: bool

_PredictionsWeatherIndexCalibrationOptional = TypedDict("_PredictionsWeatherIndexCalibrationOptional", {'published_at_ms': 'int | Decimal | None', 'change_reason': 'str | None', 'calibration_window_start_ms': 'int | Decimal | None', 'calibration_window_end_ms': 'int | Decimal | None'}, total=False)
class PredictionsWeatherIndexCalibration(_PredictionsWeatherIndexCalibrationOptional):
    config_version: str
    effective_at_ms: int | Decimal
    city_reference_c: int | Decimal
    stations: list[PredictionsWeatherIndexCalibrationStation]

_PredictionsWeatherIndexCalibrationStationOptional = TypedDict("_PredictionsWeatherIndexCalibrationStationOptional", {'update_note': 'str | None'}, total=False)
class PredictionsWeatherIndexCalibrationStation(_PredictionsWeatherIndexCalibrationStationOptional):
    station_id: str
    weight: int | Decimal
    offset_c: int | Decimal

_PredictionsWeatherIndexPointOptional = TypedDict("_PredictionsWeatherIndexPointOptional", {'v': 'int | Decimal | None', 'contributors': 'int | Decimal | None', 'receipt_basis': 'str | None', 'stations': 'list[PredictionsWeatherIndexStationReading] | None'}, total=False)
class PredictionsWeatherIndexPoint(_PredictionsWeatherIndexPointOptional):
    t: int | Decimal
    status: str

_PredictionsWeatherIndexStationReadingOptional = TypedDict("_PredictionsWeatherIndexStationReadingOptional", {'source': 'str | None', 'temp_f': 'int | Decimal | None', 'obs_time_ms': 'int | Decimal | None', 'received_at_ms': 'int | Decimal | None', 'primary_code': 'str | None'}, total=False)
class PredictionsWeatherIndexStationReading(_PredictionsWeatherIndexStationReadingOptional):
    station_id: str
    code: str

class PredictionsWeeklySchedule(TypedDict):
    start_time: str
    end_time: str
    monday: list[PredictionsDailySchedule]
    tuesday: list[PredictionsDailySchedule]
    wednesday: list[PredictionsDailySchedule]
    thursday: list[PredictionsDailySchedule]
    friday: list[PredictionsDailySchedule]
    saturday: list[PredictionsDailySchedule]
    sunday: list[PredictionsDailySchedule]

_PredictionsWithdrawalOptional = TypedDict("_PredictionsWithdrawalOptional", {'finalized_ts': 'int | Decimal | None'}, total=False)
class PredictionsWithdrawal(_PredictionsWithdrawalOptional):
    id: str
    status: str
    type: str
    amount_cents: int | Decimal
    fee_cents: int | Decimal
    created_ts: int | Decimal

class ProposeBlockTradeParams(TypedDict):
    body: PredictionsProposeBlockTradeRequest

class ProposeBlockTradeResponse(TypedDict):
    block_trade_proposal_id: str

_ResetOrderGroupParamsOptional = TypedDict("_ResetOrderGroupParamsOptional", {'subaccount': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None', 'body': 'PredictionsEmptyResponse | None'}, total=False)
class ResetOrderGroupParams(_ResetOrderGroupParamsOptional):
    order_group_id: str

class ResetOrderGroupResponse(TypedDict):
    pass

_SetEventContractExitTriggerParamsOptional = TypedDict("_SetEventContractExitTriggerParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class SetEventContractExitTriggerParams(_SetEventContractExitTriggerParamsOptional):
    ticker: str
    body: PredictionsSetEventContractExitTriggerRequest

_SetEventContractExitTriggerResponseOptional = TypedDict("_SetEventContractExitTriggerResponseOptional", {'slippage': 'PredictionsEventContractExitSlippage | None', 'stop_loss_price': 'str | None', 'take_profit_price': 'str | None', 'status_reason': 'str | None', 'triggered_order_id': 'str | None', 'triggered_leg': 'str | None'}, total=False)
class SetEventContractExitTriggerResponse(_SetEventContractExitTriggerResponseOptional):
    id: str
    ticker: str
    side: str
    subaccount_number: int | Decimal
    kind: str
    status: str
    created_time: str
    updated_time: str

class SetTargetBalanceAllocationParams(TypedDict):
    body: PredictionsSetTargetBalanceAllocationRequest

class SetTargetBalanceAllocationResponse(TypedDict):
    pass

_TriggerOrderGroupParamsOptional = TypedDict("_TriggerOrderGroupParamsOptional", {'subaccount': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None', 'body': 'PredictionsEmptyResponse | None'}, total=False)
class TriggerOrderGroupParams(_TriggerOrderGroupParamsOptional):
    order_group_id: str

class TriggerOrderGroupResponse(TypedDict):
    pass

class UpdateFcmEventContractDailyCapParams(TypedDict):
    body: PredictionsUpdateFCMEventContractDailyCapRequest

class UpdateFcmEventContractDailyCapResponse(TypedDict):
    pass

class UpdateFcmSubtraderBlockedCategoriesParams(TypedDict):
    body: PredictionsUpdateFCMSubtraderBlockedCategoriesRequest

class UpdateFcmSubtraderBlockedCategoriesResponse(TypedDict):
    categories: list[str]

_UpdateOrderGroupLimitParamsOptional = TypedDict("_UpdateOrderGroupLimitParamsOptional", {'subaccount': 'int | Decimal | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class UpdateOrderGroupLimitParams(_UpdateOrderGroupLimitParamsOptional):
    order_group_id: str
    body: PredictionsUpdateOrderGroupLimitRequest

class UpdateOrderGroupLimitResponse(TypedDict):
    pass

class UpdateSubaccountNettingParams(TypedDict):
    body: PredictionsUpdateSubaccountNettingRequest

class UpgradeAccountApiUsageLevelParams(TypedDict):
    pass

AcceptBlockTradeProposalResponse = Any
AcceptQuoteResponse = Any
AcceptRfqQuoteResponse = Any
CancelAllOrdersResponse = Any
ConfirmQuoteResponse = Any
ConfirmRfqBatchQuoteResponse = Any
ConfirmRfqQuoteResponse = Any
DeleteApiKeyResponse = Any
DeleteEventContractExitTriggerResponse = Any
DeleteQuoteResponse = Any
DeleteRfqBatchQuoteResponse = Any
DeleteRfqBatchResponse = Any
DeleteRfqQuoteResponse = Any
DeleteRfqResponse = Any
PredictionsApiKeyScope = str
PredictionsApiKeyType = str
PredictionsBookSide = str
PredictionsExchangeIndex = int | Decimal
PredictionsExchangeInstance = str
PredictionsFeeType = str
PredictionsFixedPointCount = str
PredictionsFixedPointDollars = str
PredictionsIntraExchangeInstanceTransferStatus = str
PredictionsMarketIndexStatus = str
PredictionsOrderStatus = str
PredictionsOutcomeSide = str
PredictionsPriceLevelDollarsCountFp = list[str]
PredictionsRFQBatchQuoteStatus = str
PredictionsRFQBatchStatus = str
PredictionsRestingMarginReservation = str
PredictionsSelfTradePreventionType = str
PredictionsUserFilter = str
UpdateSubaccountNettingResponse = Any
UpgradeAccountApiUsageLevelResponse = Any
