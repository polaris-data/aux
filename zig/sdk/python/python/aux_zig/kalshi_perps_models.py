"""KalshiPerps REST wire models generated from schema/kalshi_perps.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_AmendMarginOrderParamsOptional = TypedDict("_AmendMarginOrderParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class AmendMarginOrderParams(_AmendMarginOrderParamsOptional):
    order_id: str
    body: PerpsAmendMarginOrderRequest

_AmendMarginOrderResponseOptional = TypedDict("_AmendMarginOrderResponseOptional", {'client_order_id': 'str | None', 'remaining_count': 'str | None', 'fill_count': 'str | None', 'average_fill_price': 'str | None', 'average_fee_paid': 'str | None'}, total=False)
class AmendMarginOrderResponse(_AmendMarginOrderResponseOptional):
    order_id: str

class ApplyMarginSubaccountTransferParams(TypedDict):
    body: PerpsApplySubaccountTransferRequest

class ApplyMarginSubaccountTransferResponse(TypedDict):
    pass

_CancelAllMarginOrdersParamsOptional = TypedDict("_CancelAllMarginOrdersParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class CancelAllMarginOrdersParams(_CancelAllMarginOrdersParamsOptional):
    pass

_CancelMarginOrderParamsOptional = TypedDict("_CancelMarginOrderParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class CancelMarginOrderParams(_CancelMarginOrderParamsOptional):
    order_id: str

_CancelMarginOrderResponseOptional = TypedDict("_CancelMarginOrderResponseOptional", {'client_order_id': 'str | None'}, total=False)
class CancelMarginOrderResponse(_CancelMarginOrderResponseOptional):
    order_id: str
    reduced_by: str

_ClearingAssetClassFundingEstimateOptional = TypedDict("_ClearingAssetClassFundingEstimateOptional", {'subtrader_breakdowns': 'dict[str, ClearingFundingEstimate] | None', 'group_breakdowns': 'dict[str, ClearingFundingEstimate] | None'}, total=False)
class ClearingAssetClassFundingEstimate(_ClearingAssetClassFundingEstimateOptional):
    user_breakdown: ClearingFundingEstimate
    omitted_subtrader_count: int | Decimal
    omitted_group_count: int | Decimal
    next_funding_time: str

_ClearingAssetClassSettlementEstimateOptional = TypedDict("_ClearingAssetClassSettlementEstimateOptional", {'user_breakdown': 'ClearingSettlementEstimate | None', 'subtrader_breakdowns': 'dict[str, ClearingSettlementEstimate] | None', 'group_breakdowns': 'dict[str, ClearingSettlementEstimate] | None', 'omitted_subtrader_count': 'int | Decimal | None', 'omitted_group_count': 'int | Decimal | None'}, total=False)
class ClearingAssetClassSettlementEstimate(_ClearingAssetClassSettlementEstimateOptional):
    next_runtime: str

_ClearingCorporateActionPaymentDetailOptional = TypedDict("_ClearingCorporateActionPaymentDetailOptional", {'pre_split_price_centicents': 'int | Decimal | None', 'post_split_price_centicents': 'int | Decimal | None', 'exchange_ts': 'str | None'}, total=False)
class ClearingCorporateActionPaymentDetail(_ClearingCorporateActionPaymentDetailOptional):
    id: str
    type: str
    ca_id: str
    market_id: str
    market_ticker: str
    asset_class: str
    subtrader_id: str
    position_quantity_fp: str
    post_position_quantity_fp: str
    payment_amount_centicents: int | Decimal
    payment_amount_fp: str
    payment_time: str | None
    settlement_execution_time: str
    created_ts: str

class ClearingCreateMarginFcmApiKeyParams(TypedDict):
    body: ClearingCreateMarginFcmApiKeyRequest

class ClearingCreateMarginFcmApiKeyRequest(TypedDict):
    name: str
    public_key: str
    fcm_subtrader_id: str

_ClearingCreateMarginFcmApiKeyResponseOptional = TypedDict("_ClearingCreateMarginFcmApiKeyResponseOptional", {'warning': 'str | None'}, total=False)
class ClearingCreateMarginFcmApiKeyResponse(_ClearingCreateMarginFcmApiKeyResponseOptional):
    api_key_id: str

class ClearingCreateMarginSubtraderGroupParams(TypedDict):
    body: ClearingCreateMarginSubtraderGroupRequest

class ClearingCreateMarginSubtraderGroupRequest(TypedDict):
    subtrader_ids: list[str]

class ClearingCreateMarginSubtraderGroupResponse(TypedDict):
    group_id: str

class ClearingDeleteMarginFcmApiKeyParams(TypedDict):
    api_key_id: str

class ClearingDeleteMarginSubtraderGroupParams(TypedDict):
    group_id: str

_ClearingErrorOptional = TypedDict("_ClearingErrorOptional", {'details': 'str | None', 'service': 'str | None'}, total=False)
class ClearingError(_ClearingErrorOptional):
    code: str
    message: str

class ClearingEstimatePortfolioMaintenanceMarginParams(TypedDict):
    body: ClearingEstimatePortfolioMaintenanceMarginRequest

_ClearingEstimatePortfolioMaintenanceMarginPositionOptional = TypedDict("_ClearingEstimatePortfolioMaintenanceMarginPositionOptional", {'price': 'str | None'}, total=False)
class ClearingEstimatePortfolioMaintenanceMarginPosition(_ClearingEstimatePortfolioMaintenanceMarginPositionOptional):
    market_ticker: str
    quantity: int | Decimal

_ClearingEstimatePortfolioMaintenanceMarginRequestOptional = TypedDict("_ClearingEstimatePortfolioMaintenanceMarginRequestOptional", {'date': 'str | None', 'clearing_type': 'str | None'}, total=False)
class ClearingEstimatePortfolioMaintenanceMarginRequest(_ClearingEstimatePortfolioMaintenanceMarginRequestOptional):
    asset_class: str
    positions: list[ClearingEstimatePortfolioMaintenanceMarginPosition]

_ClearingEstimatePortfolioMaintenanceMarginResponseOptional = TypedDict("_ClearingEstimatePortfolioMaintenanceMarginResponseOptional", {'maintenance_margin_fp': 'str | None', 'base_margin_fp': 'str | None', 'hvar_fp': 'str | None', 'apc_fp': 'str | None', 'funding_addon_fp': 'str | None', 'liquidation_addon_fp': 'str | None'}, total=False)
class ClearingEstimatePortfolioMaintenanceMarginResponse(_ClearingEstimatePortfolioMaintenanceMarginResponseOptional):
    pass

_ClearingFundingEstimateOptional = TypedDict("_ClearingFundingEstimateOptional", {'positions': 'dict[str, ClearingMarketFundingEstimate] | None'}, total=False)
class ClearingFundingEstimate(_ClearingFundingEstimateOptional):
    funding_amount_centicents: int | Decimal
    funding_amount_fp: str

class ClearingFundingPaymentDetail(TypedDict):
    id: str
    market_ticker: str
    subtrader_id: str
    funding_time: str
    position_quantity_fp: str
    notional_value_centicents: int | Decimal
    notional_value_fp: str
    funding_amount_centicents: int | Decimal
    funding_amount_fp: str

class ClearingGenerateMarginFcmApiKeyParams(TypedDict):
    body: ClearingGenerateMarginFcmApiKeyRequest

class ClearingGenerateMarginFcmApiKeyRequest(TypedDict):
    name: str
    fcm_subtrader_id: str

_ClearingGenerateMarginFcmApiKeyResponseOptional = TypedDict("_ClearingGenerateMarginFcmApiKeyResponseOptional", {'warning': 'str | None'}, total=False)
class ClearingGenerateMarginFcmApiKeyResponse(_ClearingGenerateMarginFcmApiKeyResponseOptional):
    api_key_id: str
    private_key: str

class ClearingGetActiveMarginObligationsParams(TypedDict):
    pass

class ClearingGetActiveMarginObligationsResponse(TypedDict):
    obligations: list[ClearingObligationEntry]

class ClearingGetFundingEstimateByAssetClassParams(TypedDict):
    pass

class ClearingGetFundingEstimateByAssetClassResponse(TypedDict):
    estimates: dict[str, ClearingAssetClassFundingEstimate]

class ClearingGetGuarantyFundBalanceParams(TypedDict):
    pass

class ClearingGetGuarantyFundBalanceResponse(TypedDict):
    user_id: str
    amount_centicents: int | Decimal
    updated_ts: str

_ClearingGetLargeTraderPositionsParamsOptional = TypedDict("_ClearingGetLargeTraderPositionsParamsOptional", {'report_date': 'str | None'}, total=False)
class ClearingGetLargeTraderPositionsParams(_ClearingGetLargeTraderPositionsParamsOptional):
    pass

_ClearingGetLargeTraderPositionsResponseOptional = TypedDict("_ClearingGetLargeTraderPositionsResponseOptional", {'user_id': 'str | None', 'report_date': 'str | None', 'positions': 'list[ClearingGetLargeTraderPositionsResponsePositionsItem] | None'}, total=False)
class ClearingGetLargeTraderPositionsResponse(_ClearingGetLargeTraderPositionsResponseOptional):
    pass

_ClearingGetLargeTraderPositionsResponsePositionsItemOptional = TypedDict("_ClearingGetLargeTraderPositionsResponsePositionsItemOptional", {'market_ticker': 'str | None', 'position': 'int | Decimal | None', 'exit_day': 'bool | None', 'reportable_threshold': 'int | Decimal | None'}, total=False)
class ClearingGetLargeTraderPositionsResponsePositionsItem(_ClearingGetLargeTraderPositionsResponsePositionsItemOptional):
    pass

class ClearingGetMaintenanceMarginMetadataParams(TypedDict):
    asset_class: str
    date: str

class ClearingGetMaintenanceMarginMetadataResponse(TypedDict):
    asset_class: str
    base_tail_percentile: int | Decimal
    funding_tail_percentile: int | Decimal
    matrices: ClearingMaintenanceMarginMatrices
    liquidation_configs: list[ClearingMaintenanceMarginLiquidationConfig]
    subgroups: list[list[str]]

class ClearingGetMarginFundingScheduleParams(TypedDict):
    asset_class: str

class ClearingGetMarginFundingScheduleResponse(TypedDict):
    schedule: str

class ClearingGetMarginReportsParams(TypedDict):
    start_date: str
    end_date: str

class ClearingGetMarginReportsResponse(TypedDict):
    reports: list[ClearingMarginReport]

class ClearingGetMarginSubtraderGroupsParams(TypedDict):
    pass

class ClearingGetMarginSubtraderGroupsResponse(TypedDict):
    groups: list[ClearingMarginSubtraderGroup]

_ClearingGetMemberFundingPaymentsParamsOptional = TypedDict("_ClearingGetMemberFundingPaymentsParamsOptional", {'market_ticker': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class ClearingGetMemberFundingPaymentsParams(_ClearingGetMemberFundingPaymentsParamsOptional):
    funding_time: str

_ClearingGetMemberFundingPaymentsResponseOptional = TypedDict("_ClearingGetMemberFundingPaymentsResponseOptional", {'cursor': 'str | None'}, total=False)
class ClearingGetMemberFundingPaymentsResponse(_ClearingGetMemberFundingPaymentsResponseOptional):
    payments: list[ClearingMemberFundingPayment]

_ClearingGetObligationCorporateActionPaymentsParamsOptional = TypedDict("_ClearingGetObligationCorporateActionPaymentsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class ClearingGetObligationCorporateActionPaymentsParams(_ClearingGetObligationCorporateActionPaymentsParamsOptional):
    obligation_id: str

_ClearingGetObligationCorporateActionPaymentsResponseOptional = TypedDict("_ClearingGetObligationCorporateActionPaymentsResponseOptional", {'cursor': 'str | None'}, total=False)
class ClearingGetObligationCorporateActionPaymentsResponse(_ClearingGetObligationCorporateActionPaymentsResponseOptional):
    corporate_action_payments: list[ClearingCorporateActionPaymentDetail]

_ClearingGetObligationFundingPaymentsParamsOptional = TypedDict("_ClearingGetObligationFundingPaymentsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class ClearingGetObligationFundingPaymentsParams(_ClearingGetObligationFundingPaymentsParamsOptional):
    obligation_id: str

_ClearingGetObligationFundingPaymentsResponseOptional = TypedDict("_ClearingGetObligationFundingPaymentsResponseOptional", {'cursor': 'str | None'}, total=False)
class ClearingGetObligationFundingPaymentsResponse(_ClearingGetObligationFundingPaymentsResponseOptional):
    funding_payments: list[ClearingFundingPaymentDetail]

_ClearingGetObligationHistoryParamsOptional = TypedDict("_ClearingGetObligationHistoryParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'settlement_start_ts': 'str | None', 'settlement_end_ts': 'str | None'}, total=False)
class ClearingGetObligationHistoryParams(_ClearingGetObligationHistoryParamsOptional):
    pass

_ClearingGetObligationHistoryResponseOptional = TypedDict("_ClearingGetObligationHistoryResponseOptional", {'cursor': 'str | None'}, total=False)
class ClearingGetObligationHistoryResponse(_ClearingGetObligationHistoryResponseOptional):
    obligations: list[ClearingObligationEntry]

_ClearingGetObligationMaintenanceMarginDetailsParamsOptional = TypedDict("_ClearingGetObligationMaintenanceMarginDetailsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class ClearingGetObligationMaintenanceMarginDetailsParams(_ClearingGetObligationMaintenanceMarginDetailsParamsOptional):
    obligation_id: str

_ClearingGetObligationMaintenanceMarginDetailsResponseOptional = TypedDict("_ClearingGetObligationMaintenanceMarginDetailsResponseOptional", {'cursor': 'str | None'}, total=False)
class ClearingGetObligationMaintenanceMarginDetailsResponse(_ClearingGetObligationMaintenanceMarginDetailsResponseOptional):
    maintenance_margin_details: list[ClearingMaintenanceMarginDetail]

_ClearingGetObligationSettlementDetailsParamsOptional = TypedDict("_ClearingGetObligationSettlementDetailsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class ClearingGetObligationSettlementDetailsParams(_ClearingGetObligationSettlementDetailsParamsOptional):
    obligation_id: str

_ClearingGetObligationSettlementDetailsResponseOptional = TypedDict("_ClearingGetObligationSettlementDetailsResponseOptional", {'cursor': 'str | None'}, total=False)
class ClearingGetObligationSettlementDetailsResponse(_ClearingGetObligationSettlementDetailsResponseOptional):
    settlement_details: list[ClearingSettlementDetail]

_ClearingGetSettlementBalanceHistoryParamsOptional = TypedDict("_ClearingGetSettlementBalanceHistoryParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'start_ts': 'str | None', 'end_ts': 'str | None'}, total=False)
class ClearingGetSettlementBalanceHistoryParams(_ClearingGetSettlementBalanceHistoryParamsOptional):
    pass

_ClearingGetSettlementBalanceHistoryResponseOptional = TypedDict("_ClearingGetSettlementBalanceHistoryResponseOptional", {'cursor': 'str | None'}, total=False)
class ClearingGetSettlementBalanceHistoryResponse(_ClearingGetSettlementBalanceHistoryResponseOptional):
    entries: list[ClearingSettlementBalanceHistoryEntry]

class ClearingGetSettlementBalanceParams(TypedDict):
    pass

_ClearingGetSettlementBalanceResponseOptional = TypedDict("_ClearingGetSettlementBalanceResponseOptional", {'locked_balance_centicents': 'int | Decimal | None', 'locked_balance_fp': 'str | None'}, total=False)
class ClearingGetSettlementBalanceResponse(_ClearingGetSettlementBalanceResponseOptional):
    user_id: str
    balance_available_centicents: int | Decimal
    balance_available_fp: str

class ClearingGetSettlementBalanceWithdrawalParams(TypedDict):
    id: str

class ClearingGetSettlementBalanceWithdrawalResponse(TypedDict):
    id: str
    amount: str
    status: str
    created_ts: str

class ClearingGetSettlementEstimateByAssetClassParams(TypedDict):
    pass

class ClearingGetSettlementEstimateByAssetClassResponse(TypedDict):
    estimates: dict[str, ClearingAssetClassSettlementEstimate]
    settlement_balance_centicents: int | Decimal
    settlement_balance_fp: str

class ClearingGetSettlementPricesParams(TypedDict):
    asset_class: str
    settlement_time: str

class ClearingGetSettlementPricesResponse(TypedDict):
    settlement_prices: dict[str, int | Decimal]

_ClearingListMarginFcmApiKeysParamsOptional = TypedDict("_ClearingListMarginFcmApiKeysParamsOptional", {'fcm_subtrader_id': 'str | None'}, total=False)
class ClearingListMarginFcmApiKeysParams(_ClearingListMarginFcmApiKeysParamsOptional):
    pass

class ClearingListMarginFcmApiKeysResponse(TypedDict):
    api_keys: list[ClearingMarginFcmApiKey]

_ClearingMaintenanceMarginDetailOptional = TypedDict("_ClearingMaintenanceMarginDetailOptional", {'margin_group_id': 'str | None'}, total=False)
class ClearingMaintenanceMarginDetail(_ClearingMaintenanceMarginDetailOptional):
    id: str
    subtrader_id: str
    maintenance_margin_centicents: int | Decimal
    maintenance_margin_delta_centicents: int | Decimal

class ClearingMaintenanceMarginLiquidationConfig(TypedDict):
    market_ticker: str
    market_impact_volatility: int | Decimal
    market_impact_coefficient: int | Decimal
    market_impact_exponent: int | Decimal
    market_impact_forecasted_volume: int | Decimal
    spread_rate: int | Decimal

_ClearingMaintenanceMarginMatricesOptional = TypedDict("_ClearingMaintenanceMarginMatricesOptional", {'hvar': 'ClearingMaintenanceMarginMatrix | None', 'apc': 'ClearingMaintenanceMarginMatrix | None', 'aug': 'ClearingMaintenanceMarginMatrix | None', 'funding': 'ClearingMaintenanceMarginMatrix | None'}, total=False)
class ClearingMaintenanceMarginMatrices(_ClearingMaintenanceMarginMatricesOptional):
    pass

class ClearingMaintenanceMarginMatrix(TypedDict):
    market_tickers: list[str]
    returns: list[list[int | Decimal]]

class ClearingMarginFcmApiKey(TypedDict):
    api_key_id: str
    name: str
    fcm_subtrader_id: str

_ClearingMarginReportOptional = TypedDict("_ClearingMarginReportOptional", {'snapshot_ts': 'str | None'}, total=False)
class ClearingMarginReport(_ClearingMarginReportOptional):
    report_type: str
    url: str
    date: str
    created_ts: str
    is_end_of_day: bool

class ClearingMarginSubtraderGroup(TypedDict):
    group_id: str
    member_subtrader_ids: list[str]

class ClearingMarketFundingEstimate(TypedDict):
    quantity_centicount: int | Decimal
    funding_amount_centicents: int | Decimal
    funding_amount_fp: str

_ClearingMarketSettlementEstimateOptional = TypedDict("_ClearingMarketSettlementEstimateOptional", {'session_avg_price_fp': 'str | None'}, total=False)
class ClearingMarketSettlementEstimate(_ClearingMarketSettlementEstimateOptional):
    quantity_centicount: int | Decimal
    variation_margin_centicents: int | Decimal
    variation_margin_fp: str
    notional_value_centicents: int | Decimal
    notional_value_fp: str

class ClearingMemberFundingPayment(TypedDict):
    id: str
    market_ticker: str
    subtrader_id: str
    funding_time: str
    position_quantity_fp: str
    notional_value_centicents: int | Decimal
    notional_value_fp: str
    funding_amount_centicents: int | Decimal
    funding_amount_fp: str
    settlement_execution_time: str

_ClearingObligationEntryOptional = TypedDict("_ClearingObligationEntryOptional", {'corporate_action_payments_truncated': 'bool | None', 'settlement_details_truncated': 'bool | None', 'maintenance_margin_details_truncated': 'bool | None', 'funding_payments_truncated': 'bool | None'}, total=False)
class ClearingObligationEntry(_ClearingObligationEntryOptional):
    id: str
    user_id: str
    amount_centicents: int | Decimal
    amount_fp: str
    fees_centicents: int | Decimal
    maintenance_margin_centicents: int | Decimal
    pnl_centicents: int | Decimal
    pnl_fp: str
    execution_time: str
    last_updated_ts: str
    asset_class: str
    receives: list[ClearingObligationReceiveInfo]
    settlement_details: list[ClearingSettlementDetail]
    maintenance_margin_details: list[ClearingMaintenanceMarginDetail]
    funding_payments: list[ClearingFundingPaymentDetail]
    corporate_action_payments: list[ClearingCorporateActionPaymentDetail]

class ClearingObligationInfo(TypedDict):
    id: str
    user_id: str
    amount_centicents: int | Decimal
    amount_fp: str
    fees_centicents: int | Decimal
    maintenance_margin_centicents: int | Decimal
    pnl_centicents: int | Decimal
    pnl_fp: str
    execution_time: str
    last_updated_ts: str
    asset_class: str

class ClearingObligationReceiveInfo(TypedDict):
    id: str
    type: str
    amount_centicents: int | Decimal
    amount_fp: str
    external_reference: str
    created_ts: str

class ClearingSettlementBalanceHistoryEntry(TypedDict):
    balance_delta_centicents: int | Decimal
    balance_delta_fp: str
    locked_balance_delta_centicents: int | Decimal
    locked_balance_delta_fp: str
    reason: str
    business_transaction_id: str
    created_ts: str

class ClearingSettlementDetail(TypedDict):
    id: str
    market_ticker: str
    subtrader_id: str
    position_quantity_fp: str
    pnl_centicents: int | Decimal
    pnl_fp: str
    total_fees_centicents: int | Decimal
    total_amount_centicents: int | Decimal
    total_amount_fp: str

_ClearingSettlementEstimateOptional = TypedDict("_ClearingSettlementEstimateOptional", {'positions': 'dict[str, ClearingMarketSettlementEstimate] | None'}, total=False)
class ClearingSettlementEstimate(_ClearingSettlementEstimateOptional):
    variation_margin_centicents: int | Decimal
    variation_margin_fp: str
    total_fees_centicents: int | Decimal
    maintenance_margin_delta_centicents: int | Decimal
    maintenance_margin_required_centicents: int | Decimal
    total_amount_centicents: int | Decimal
    total_amount_fp: str

class ClearingUpdateMarginSubtraderGroupParams(TypedDict):
    group_id: str
    body: ClearingUpdateMarginSubtraderGroupRequest

class ClearingUpdateMarginSubtraderGroupRequest(TypedDict):
    subtrader_ids: list[str]

class ClearingWithdrawSettlementBalanceParams(TypedDict):
    body: ClearingWithdrawSettlementBalanceRequest

class ClearingWithdrawSettlementBalanceRequest(TypedDict):
    amount: str
    mfa_code: str

class ClearingWithdrawSettlementBalanceResponse(TypedDict):
    id: str

class CreateMarginFcmSubtraderParams(TypedDict):
    body: PerpsCreateMarginFCMSubtraderRequest

class CreateMarginFcmSubtraderResponse(TypedDict):
    subtrader_id: str

class CreateMarginOrderGroupParams(TypedDict):
    body: PerpsCreateOrderGroupRequest

_CreateMarginOrderGroupResponseOptional = TypedDict("_CreateMarginOrderGroupResponseOptional", {'exchange_index': 'int | Decimal | None'}, total=False)
class CreateMarginOrderGroupResponse(_CreateMarginOrderGroupResponseOptional):
    order_group_id: str
    subaccount: int | Decimal

class CreateMarginOrderParams(TypedDict):
    body: PerpsCreateMarginOrderRequest

_CreateMarginOrderResponseOptional = TypedDict("_CreateMarginOrderResponseOptional", {'client_order_id': 'str | None', 'average_fill_price': 'str | None', 'average_fee_paid': 'str | None'}, total=False)
class CreateMarginOrderResponse(_CreateMarginOrderResponseOptional):
    order_id: str
    fill_count: str
    remaining_count: str

class CreateMarginSubaccountParams(TypedDict):
    pass

class CreateMarginSubaccountResponse(TypedDict):
    subaccount_number: int | Decimal

_DecreaseMarginOrderParamsOptional = TypedDict("_DecreaseMarginOrderParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class DecreaseMarginOrderParams(_DecreaseMarginOrderParamsOptional):
    order_id: str
    body: PerpsDecreaseMarginOrderRequest

_DecreaseMarginOrderResponseOptional = TypedDict("_DecreaseMarginOrderResponseOptional", {'client_order_id': 'str | None'}, total=False)
class DecreaseMarginOrderResponse(_DecreaseMarginOrderResponseOptional):
    order_id: str
    remaining_count: str

_DeleteCrossMarginExitTriggerByIdParamsOptional = TypedDict("_DeleteCrossMarginExitTriggerByIdParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class DeleteCrossMarginExitTriggerByIdParams(_DeleteCrossMarginExitTriggerByIdParamsOptional):
    ticker: str
    trigger_id: str

_DeleteCrossMarginExitTriggerParamsOptional = TypedDict("_DeleteCrossMarginExitTriggerParamsOptional", {'subaccount': 'int | Decimal | None', 'kind': 'str | None'}, total=False)
class DeleteCrossMarginExitTriggerParams(_DeleteCrossMarginExitTriggerParamsOptional):
    ticker: str

class DeleteFcmNotionalRiskLimitParams(TypedDict):
    pass

class DeleteFcmNotionalRiskLimitResponse(TypedDict):
    pass

_DeleteFcmSubtraderRiskControlsParamsOptional = TypedDict("_DeleteFcmSubtraderRiskControlsParamsOptional", {'market_ticker': 'str | None', 'asset_class': 'str | None'}, total=False)
class DeleteFcmSubtraderRiskControlsParams(_DeleteFcmSubtraderRiskControlsParamsOptional):
    subtrader_id: str

class DeleteFcmSubtraderRiskControlsResponse(TypedDict):
    pass

_DeleteIsolatedMarginExitTriggerParamsOptional = TypedDict("_DeleteIsolatedMarginExitTriggerParamsOptional", {'kind': 'str | None'}, total=False)
class DeleteIsolatedMarginExitTriggerParams(_DeleteIsolatedMarginExitTriggerParamsOptional):
    ticker: str

_DeleteMarginOrderGroupParamsOptional = TypedDict("_DeleteMarginOrderGroupParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class DeleteMarginOrderGroupParams(_DeleteMarginOrderGroupParamsOptional):
    order_group_id: str

class DeleteMarginOrderGroupResponse(TypedDict):
    pass

_GetCrossMarginExitTriggersParamsOptional = TypedDict("_GetCrossMarginExitTriggersParamsOptional", {'subaccount': 'int | Decimal | None', 'kind': 'str | None'}, total=False)
class GetCrossMarginExitTriggersParams(_GetCrossMarginExitTriggersParamsOptional):
    ticker: str

class GetCrossMarginExitTriggersResponse(TypedDict):
    exit_triggers: list[PerpsExitTrigger]

_GetFcmSubtraderRiskControlsParamsOptional = TypedDict("_GetFcmSubtraderRiskControlsParamsOptional", {'subtrader_id': 'str | None', 'market_ticker': 'str | None', 'asset_class': 'str | None'}, total=False)
class GetFcmSubtraderRiskControlsParams(_GetFcmSubtraderRiskControlsParamsOptional):
    pass

class GetFcmSubtraderRiskControlsResponse(TypedDict):
    risk_controls: list[PerpsFCMSubtraderRiskControls]
    notional_limits: list[PerpsFCMSubtraderNotionalRiskLimit]

_GetIsolatedMarginExitTriggersParamsOptional = TypedDict("_GetIsolatedMarginExitTriggersParamsOptional", {'kind': 'str | None'}, total=False)
class GetIsolatedMarginExitTriggersParams(_GetIsolatedMarginExitTriggersParamsOptional):
    ticker: str

class GetIsolatedMarginExitTriggersResponse(TypedDict):
    exit_triggers: list[PerpsExitTrigger]

_GetMarginBalanceParamsOptional = TypedDict("_GetMarginBalanceParamsOptional", {'compute_available_balance': 'bool | None'}, total=False)
class GetMarginBalanceParams(_GetMarginBalanceParamsOptional):
    pass

class GetMarginBalanceResponse(TypedDict):
    subaccount_balances: list[PerpsMarginSubaccountBalance]
    settled_funds: str

_GetMarginCorporateActionsParamsOptional = TypedDict("_GetMarginCorporateActionsParamsOptional", {'ticker': 'str | None', 'market_id': 'str | None'}, total=False)
class GetMarginCorporateActionsParams(_GetMarginCorporateActionsParamsOptional):
    pass

class GetMarginCorporateActionsResponse(TypedDict):
    corporate_actions: list[PerpsMarginCorporateAction]

class GetMarginEnabledParams(TypedDict):
    pass

class GetMarginEnabledResponse(TypedDict):
    enabled: bool

class GetMarginExchangeStatusParams(TypedDict):
    pass

_GetMarginExchangeStatusResponseOptional = TypedDict("_GetMarginExchangeStatusResponseOptional", {'intra_exchange_transfers_active': 'bool | None', 'exchange_estimated_resume_time': 'str | None', 'exchange_index_statuses': 'list[PerpsExchangeIndexStatus] | None'}, total=False)
class GetMarginExchangeStatusResponse(_GetMarginExchangeStatusResponseOptional):
    exchange_active: bool
    trading_active: bool

class GetMarginFeeTierRatesParams(TypedDict):
    pass

class GetMarginFeeTierRatesResponse(TypedDict):
    fee_tier_rates: list[PerpsMarginFeeTierRate]

class GetMarginFeeTiersParams(TypedDict):
    pass

class GetMarginFeeTiersResponse(TypedDict):
    maker_fee_rates: dict[str, int | Decimal]
    taker_fee_rates: dict[str, int | Decimal]

_GetMarginFillsParamsOptional = TypedDict("_GetMarginFillsParamsOptional", {'subaccount': 'int | Decimal | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None'}, total=False)
class GetMarginFillsParams(_GetMarginFillsParamsOptional):
    pass

class GetMarginFillsResponse(TypedDict):
    fills: list[PerpsMarginFill]
    cursor: str

_GetMarginFundingHistoryParamsOptional = TypedDict("_GetMarginFundingHistoryParamsOptional", {'ticker': 'str | None', 'subaccount': 'int | Decimal | None'}, total=False)
class GetMarginFundingHistoryParams(_GetMarginFundingHistoryParamsOptional):
    start_date: str
    end_date: str

class GetMarginFundingHistoryResponse(TypedDict):
    funding_history: list[PerpsMarginFundingHistoryEntry]

class GetMarginFundingRateEstimateParams(TypedDict):
    ticker: str

_GetMarginFundingRateEstimateResponseOptional = TypedDict("_GetMarginFundingRateEstimateResponseOptional", {'market_ticker': 'str | None', 'computed_time': 'str | None', 'funding_rate': 'int | Decimal | None', 'mark_price': 'str | None', 'premium_index': 'str | None', 'premium_index_ts': 'str | None'}, total=False)
class GetMarginFundingRateEstimateResponse(_GetMarginFundingRateEstimateResponseOptional):
    next_funding_time: str

_GetMarginHistoricalFundingRatesParamsOptional = TypedDict("_GetMarginHistoricalFundingRatesParamsOptional", {'ticker': 'str | None', 'start_ts': 'int | Decimal | None', 'end_ts': 'int | Decimal | None'}, total=False)
class GetMarginHistoricalFundingRatesParams(_GetMarginHistoricalFundingRatesParamsOptional):
    pass

class GetMarginHistoricalFundingRatesResponse(TypedDict):
    funding_rates: list[PerpsMarginFundingRate]

_GetMarginMarketCandlesticksParamsOptional = TypedDict("_GetMarginMarketCandlesticksParamsOptional", {'include_latest_before_start': 'bool | None'}, total=False)
class GetMarginMarketCandlesticksParams(_GetMarginMarketCandlesticksParamsOptional):
    ticker: str
    start_ts: int | Decimal
    end_ts: int | Decimal
    period_interval: int | Decimal

class GetMarginMarketCandlesticksResponse(TypedDict):
    ticker: str
    candlesticks: list[PerpsMarginMarketCandlestick]

_GetMarginMarketOrderbookParamsOptional = TypedDict("_GetMarginMarketOrderbookParamsOptional", {'depth': 'int | Decimal | None', 'aggregation_tick_size': 'str | None'}, total=False)
class GetMarginMarketOrderbookParams(_GetMarginMarketOrderbookParamsOptional):
    ticker: str

class GetMarginMarketOrderbookResponse(TypedDict):
    orderbook: PerpsMarginOrderbookCount

class GetMarginMarketOrderbooksParams(TypedDict):
    tickers: list[str]

class GetMarginMarketOrderbooksResponse(TypedDict):
    orderbooks: list[PerpsMarginMarketOrderbook]

class GetMarginMarketParams(TypedDict):
    ticker: str

class GetMarginMarketResponse(TypedDict):
    market: PerpsMarginMarket

class GetMarginMarketSchedulesParams(TypedDict):
    tickers: list[str]

_GetMarginMarketsParamsOptional = TypedDict("_GetMarginMarketsParamsOptional", {'status': 'str | None'}, total=False)
class GetMarginMarketsParams(_GetMarginMarketsParamsOptional):
    pass

class GetMarginMarketsResponse(TypedDict):
    markets: list[PerpsMarginMarket]

class GetMarginNotionalRiskLimitParams(TypedDict):
    pass

_GetMarginNotionalRiskLimitResponseOptional = TypedDict("_GetMarginNotionalRiskLimitResponseOptional", {'member_notional_value_risk_limit': 'str | None', 'effective_account_notional_value_risk_limit': 'str | None'}, total=False)
class GetMarginNotionalRiskLimitResponse(_GetMarginNotionalRiskLimitResponseOptional):
    default_notional_value_risk_limit: str
    notional_value_risk_limits_by_market_ticker: dict[str, str]
    total_current_usage: str
    current_usage_by_market_ticker: dict[str, str]

_GetMarginOrderGroupParamsOptional = TypedDict("_GetMarginOrderGroupParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class GetMarginOrderGroupParams(_GetMarginOrderGroupParamsOptional):
    order_group_id: str

_GetMarginOrderGroupResponseOptional = TypedDict("_GetMarginOrderGroupResponseOptional", {'contracts_limit_fp': 'str | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class GetMarginOrderGroupResponse(_GetMarginOrderGroupResponseOptional):
    is_auto_cancel_enabled: bool
    orders: list[str]

_GetMarginOrderGroupsParamsOptional = TypedDict("_GetMarginOrderGroupsParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class GetMarginOrderGroupsParams(_GetMarginOrderGroupsParamsOptional):
    pass

_GetMarginOrderGroupsResponseOptional = TypedDict("_GetMarginOrderGroupsResponseOptional", {'order_groups': 'list[PerpsOrderGroup] | None'}, total=False)
class GetMarginOrderGroupsResponse(_GetMarginOrderGroupsResponseOptional):
    pass

class GetMarginOrderParams(TypedDict):
    order_id: str

class GetMarginOrderResponse(TypedDict):
    order: PerpsMarginOrder

_GetMarginOrdersParamsOptional = TypedDict("_GetMarginOrdersParamsOptional", {'ticker': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None', 'status': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'subaccount': 'int | Decimal | None'}, total=False)
class GetMarginOrdersParams(_GetMarginOrdersParamsOptional):
    pass

class GetMarginOrdersResponse(TypedDict):
    orders: list[PerpsMarginOrder]
    cursor: str

_GetMarginPositionsParamsOptional = TypedDict("_GetMarginPositionsParamsOptional", {'subaccount': 'int | Decimal | None', 'ticker': 'str | None'}, total=False)
class GetMarginPositionsParams(_GetMarginPositionsParamsOptional):
    pass

class GetMarginPositionsResponse(TypedDict):
    positions: list[PerpsMarginPosition]

class GetMarginPremiumIndexParams(TypedDict):
    ticker: str
    start_ts: int | Decimal
    end_ts: int | Decimal

class GetMarginPremiumIndexResponse(TypedDict):
    points: list[PerpsMarginPremiumIndexPoint]

class GetMarginRiskParametersParams(TypedDict):
    pass

class GetMarginRiskParametersResponse(TypedDict):
    liquidation_margin_ratio_threshold: int | Decimal
    queue_entry_margin_ratio_threshold: int | Decimal
    initial_margin_multiplier: dict[str, int | Decimal]

class GetMarginRiskParams(TypedDict):
    pass

_GetMarginRiskResponseOptional = TypedDict("_GetMarginRiskResponseOptional", {'account_leverage': 'int | Decimal | None'}, total=False)
class GetMarginRiskResponse(_GetMarginRiskResponseOptional):
    total_position_notional: str
    total_maintenance_margin: str
    positions: list[PerpsMarginRiskPosition]

_GetMarginTradesParamsOptional = TypedDict("_GetMarginTradesParamsOptional", {'ticker': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'min_ts': 'int | Decimal | None', 'max_ts': 'int | Decimal | None', 'is_block_trade': 'bool | None'}, total=False)
class GetMarginTradesParams(_GetMarginTradesParamsOptional):
    pass

class GetMarginTradesResponse(TypedDict):
    trades: list[PerpsMarginTrade]
    cursor: str

class GetPerpsAccountApiLimitsParams(TypedDict):
    pass

class GetPerpsAccountApiLimitsResponse(TypedDict):
    usage_tier: str
    read: PerpsBucketLimit
    write: PerpsBucketLimit
    grants: list[PerpsApiUsageLevelGrant]

class IntraExchangeInstanceTransferParams(TypedDict):
    body: PerpsIntraExchangeInstanceTransferRequest

class IntraExchangeInstanceTransferResponse(TypedDict):
    transfer_id: str

_PerpsAmendMarginOrderRequestOptional = TypedDict("_PerpsAmendMarginOrderRequestOptional", {'ticker': 'str | None', 'market_id': 'str | None', 'client_order_id': 'str | None', 'updated_client_order_id': 'str | None', 'expiration_time': 'int | Decimal | None'}, total=False)
class PerpsAmendMarginOrderRequest(_PerpsAmendMarginOrderRequestOptional):
    side: str
    price: str
    count: str

_PerpsAmendMarginOrderResponseOptional = TypedDict("_PerpsAmendMarginOrderResponseOptional", {'client_order_id': 'str | None', 'remaining_count': 'str | None', 'fill_count': 'str | None', 'average_fill_price': 'str | None', 'average_fee_paid': 'str | None'}, total=False)
class PerpsAmendMarginOrderResponse(_PerpsAmendMarginOrderResponseOptional):
    order_id: str

_PerpsApiUsageLevelGrantOptional = TypedDict("_PerpsApiUsageLevelGrantOptional", {'expires_ts': 'int | Decimal | None'}, total=False)
class PerpsApiUsageLevelGrant(_PerpsApiUsageLevelGrantOptional):
    exchange_instance: str
    level: str
    source: str

_PerpsApplySubaccountTransferRequestOptional = TypedDict("_PerpsApplySubaccountTransferRequestOptional", {'exchange_index': 'int | Decimal | None'}, total=False)
class PerpsApplySubaccountTransferRequest(_PerpsApplySubaccountTransferRequestOptional):
    client_transfer_id: str
    from_subaccount: int | Decimal
    to_subaccount: int | Decimal
    amount_cents: int | Decimal

class PerpsApplySubaccountTransferResponse(TypedDict):
    pass

class PerpsBidAskDistributionHistorical(TypedDict):
    open: str
    low: str
    high: str
    close: str

class PerpsBucketLimit(TypedDict):
    refill_rate: int | Decimal
    bucket_capacity: int | Decimal

_PerpsCancelMarginOrderResponseOptional = TypedDict("_PerpsCancelMarginOrderResponseOptional", {'client_order_id': 'str | None'}, total=False)
class PerpsCancelMarginOrderResponse(_PerpsCancelMarginOrderResponseOptional):
    order_id: str
    reduced_by: str

class PerpsCreateMarginFCMSubtraderRequest(TypedDict):
    subtrader_suffix: str
    require_category_cap: bool

class PerpsCreateMarginFCMSubtraderResponse(TypedDict):
    subtrader_id: str

_PerpsCreateMarginOrderRequestOptional = TypedDict("_PerpsCreateMarginOrderRequestOptional", {'ticker': 'str | None', 'market_id': 'str | None', 'expiration_time': 'int | Decimal | None', 'post_only': 'bool | None', 'cancel_order_on_pause': 'bool | None', 'reduce_only': 'bool | None', 'subaccount': 'int | Decimal | None', 'order_group_id': 'str | None', 'market_version': 'int | Decimal | None'}, total=False)
class PerpsCreateMarginOrderRequest(_PerpsCreateMarginOrderRequestOptional):
    client_order_id: str
    side: str
    count: str
    price: str
    time_in_force: str
    self_trade_prevention_type: str

_PerpsCreateMarginOrderResponseOptional = TypedDict("_PerpsCreateMarginOrderResponseOptional", {'client_order_id': 'str | None', 'average_fill_price': 'str | None', 'average_fee_paid': 'str | None'}, total=False)
class PerpsCreateMarginOrderResponse(_PerpsCreateMarginOrderResponseOptional):
    order_id: str
    fill_count: str
    remaining_count: str

_PerpsCreateOrderGroupRequestOptional = TypedDict("_PerpsCreateOrderGroupRequestOptional", {'subaccount': 'int | Decimal | None', 'contracts_limit': 'int | Decimal | None', 'contracts_limit_fp': 'str | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PerpsCreateOrderGroupRequest(_PerpsCreateOrderGroupRequestOptional):
    pass

_PerpsCreateOrderGroupResponseOptional = TypedDict("_PerpsCreateOrderGroupResponseOptional", {'exchange_index': 'int | Decimal | None'}, total=False)
class PerpsCreateOrderGroupResponse(_PerpsCreateOrderGroupResponseOptional):
    order_group_id: str
    subaccount: int | Decimal

class PerpsCreateSubaccountResponse(TypedDict):
    subaccount_number: int | Decimal

_PerpsDecreaseMarginOrderRequestOptional = TypedDict("_PerpsDecreaseMarginOrderRequestOptional", {'reduce_by': 'str | None', 'reduce_to': 'str | None'}, total=False)
class PerpsDecreaseMarginOrderRequest(_PerpsDecreaseMarginOrderRequestOptional):
    pass

_PerpsDecreaseMarginOrderResponseOptional = TypedDict("_PerpsDecreaseMarginOrderResponseOptional", {'client_order_id': 'str | None'}, total=False)
class PerpsDecreaseMarginOrderResponse(_PerpsDecreaseMarginOrderResponseOptional):
    order_id: str
    remaining_count: str

class PerpsEmptyResponse(TypedDict):
    pass

_PerpsErrorResponseOptional = TypedDict("_PerpsErrorResponseOptional", {'code': 'str | None', 'message': 'str | None', 'details': 'str | None'}, total=False)
class PerpsErrorResponse(_PerpsErrorResponseOptional):
    pass

class PerpsExchangeIndexStatus(TypedDict):
    exchange_index: int | Decimal
    description: str
    exchange_active: bool
    trading_active: bool
    intra_exchange_transfers_active: bool

_PerpsExchangeStatusOptional = TypedDict("_PerpsExchangeStatusOptional", {'intra_exchange_transfers_active': 'bool | None', 'exchange_estimated_resume_time': 'str | None', 'exchange_index_statuses': 'list[PerpsExchangeIndexStatus] | None'}, total=False)
class PerpsExchangeStatus(_PerpsExchangeStatusOptional):
    exchange_active: bool
    trading_active: bool

_PerpsExitTriggerOptional = TypedDict("_PerpsExitTriggerOptional", {'status_reason': 'str | None', 'triggered_leg': 'str | None', 'triggered_order_id': 'str | None', 'anchor_order_id': 'str | None', 'client_trigger_id': 'str | None', 'stop_loss_price': 'str | None', 'take_profit_price': 'str | None', 'trail_amount': 'str | None', 'trail_bps': 'int | Decimal | None', 'watermark_price': 'str | None', 'effective_stop_price': 'str | None'}, total=False)
class PerpsExitTrigger(_PerpsExitTriggerOptional):
    id: str
    ticker: str
    kind: str
    status: str
    count: str
    filled_count: str
    created_time: str
    updated_time: str

_PerpsFCMSubtraderNotionalRiskLimitOptional = TypedDict("_PerpsFCMSubtraderNotionalRiskLimitOptional", {'market_ticker': 'str | None'}, total=False)
class PerpsFCMSubtraderNotionalRiskLimit(_PerpsFCMSubtraderNotionalRiskLimitOptional):
    subtrader_id: str
    notional_value_risk_limit: str
    current_notional: str

_PerpsFCMSubtraderRiskControlsOptional = TypedDict("_PerpsFCMSubtraderRiskControlsOptional", {'market_ticker': 'str | None', 'asset_class': 'str | None'}, total=False)
class PerpsFCMSubtraderRiskControls(_PerpsFCMSubtraderRiskControlsOptional):
    subtrader_id: str
    im_cap: str
    current_im: str

class PerpsGetAccountApiLimitsResponse(TypedDict):
    usage_tier: str
    read: PerpsBucketLimit
    write: PerpsBucketLimit
    grants: list[PerpsApiUsageLevelGrant]

class PerpsGetExitTriggersResponse(TypedDict):
    exit_triggers: list[PerpsExitTrigger]

class PerpsGetFCMSubtraderRiskControlsResponse(TypedDict):
    risk_controls: list[PerpsFCMSubtraderRiskControls]
    notional_limits: list[PerpsFCMSubtraderNotionalRiskLimit]

class PerpsGetMarginBalanceResponse(TypedDict):
    subaccount_balances: list[PerpsMarginSubaccountBalance]
    settled_funds: str

class PerpsGetMarginCorporateActionsResponse(TypedDict):
    corporate_actions: list[PerpsMarginCorporateAction]

class PerpsGetMarginFeeTierRatesResponse(TypedDict):
    fee_tier_rates: list[PerpsMarginFeeTierRate]

class PerpsGetMarginFeeTiersResponse(TypedDict):
    maker_fee_rates: dict[str, int | Decimal]
    taker_fee_rates: dict[str, int | Decimal]

class PerpsGetMarginFillsResponse(TypedDict):
    fills: list[PerpsMarginFill]
    cursor: str

class PerpsGetMarginFundingHistoryResponse(TypedDict):
    funding_history: list[PerpsMarginFundingHistoryEntry]

_PerpsGetMarginFundingRateEstimateResponseOptional = TypedDict("_PerpsGetMarginFundingRateEstimateResponseOptional", {'market_ticker': 'str | None', 'computed_time': 'str | None', 'funding_rate': 'int | Decimal | None', 'mark_price': 'str | None', 'premium_index': 'str | None', 'premium_index_ts': 'str | None'}, total=False)
class PerpsGetMarginFundingRateEstimateResponse(_PerpsGetMarginFundingRateEstimateResponseOptional):
    next_funding_time: str

class PerpsGetMarginHistoricalFundingRatesResponse(TypedDict):
    funding_rates: list[PerpsMarginFundingRate]

class PerpsGetMarginMarketCandlesticksResponse(TypedDict):
    ticker: str
    candlesticks: list[PerpsMarginMarketCandlestick]

class PerpsGetMarginMarketOrderbooksResponse(TypedDict):
    orderbooks: list[PerpsMarginMarketOrderbook]

class PerpsGetMarginMarketsResponse(TypedDict):
    markets: list[PerpsMarginMarket]

class PerpsGetMarginOrderResponse(TypedDict):
    order: PerpsMarginOrder

class PerpsGetMarginOrdersResponse(TypedDict):
    orders: list[PerpsMarginOrder]
    cursor: str

class PerpsGetMarginPositionsResponse(TypedDict):
    positions: list[PerpsMarginPosition]

class PerpsGetMarginPremiumIndexResponse(TypedDict):
    points: list[PerpsMarginPremiumIndexPoint]

class PerpsGetMarginRiskParametersResponse(TypedDict):
    liquidation_margin_ratio_threshold: int | Decimal
    queue_entry_margin_ratio_threshold: int | Decimal
    initial_margin_multiplier: dict[str, int | Decimal]

_PerpsGetMarginRiskResponseOptional = TypedDict("_PerpsGetMarginRiskResponseOptional", {'account_leverage': 'int | Decimal | None'}, total=False)
class PerpsGetMarginRiskResponse(_PerpsGetMarginRiskResponseOptional):
    total_position_notional: str
    total_maintenance_margin: str
    positions: list[PerpsMarginRiskPosition]

class PerpsGetMarginTradesResponse(TypedDict):
    trades: list[PerpsMarginTrade]
    cursor: str

_PerpsGetOrderGroupResponseOptional = TypedDict("_PerpsGetOrderGroupResponseOptional", {'contracts_limit_fp': 'str | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PerpsGetOrderGroupResponse(_PerpsGetOrderGroupResponseOptional):
    is_auto_cancel_enabled: bool
    orders: list[str]

_PerpsGetOrderGroupsResponseOptional = TypedDict("_PerpsGetOrderGroupsResponseOptional", {'order_groups': 'list[PerpsOrderGroup] | None'}, total=False)
class PerpsGetOrderGroupsResponse(_PerpsGetOrderGroupsResponseOptional):
    pass

_PerpsIntraExchangeInstanceTransferRequestOptional = TypedDict("_PerpsIntraExchangeInstanceTransferRequestOptional", {'client_transfer_id': 'str | None', 'source_exchange_shard': 'int | Decimal | None', 'destination_exchange_shard': 'int | Decimal | None', 'source_subaccount': 'int | Decimal | None', 'destination_subaccount': 'int | Decimal | None'}, total=False)
class PerpsIntraExchangeInstanceTransferRequest(_PerpsIntraExchangeInstanceTransferRequestOptional):
    source: str
    destination: str
    amount: int | Decimal

class PerpsIntraExchangeInstanceTransferResponse(TypedDict):
    transfer_id: str

_PerpsMarginCorporateActionOptional = TypedDict("_PerpsMarginCorporateActionOptional", {'execution_time': 'str | None', 'cash': 'PerpsMarginCorporateActionCashTerms | None', 'split': 'PerpsMarginCorporateActionSplitTerms | None', 'final_settlement': 'PerpsMarginCorporateActionFinalSettlementTerms | None', 'ticker_change': 'PerpsMarginCorporateActionTickerChangeTerms | None'}, total=False)
class PerpsMarginCorporateAction(_PerpsMarginCorporateActionOptional):
    id: str
    market_id: str
    market_ticker: str
    type: str
    status: str
    updated_ts: str

_PerpsMarginCorporateActionCashTermsOptional = TypedDict("_PerpsMarginCorporateActionCashTermsOptional", {'cash_per_share': 'str | None', 'cash_per_contract': 'str | None', 'ex_date': 'str | None'}, total=False)
class PerpsMarginCorporateActionCashTerms(_PerpsMarginCorporateActionCashTermsOptional):
    pass

_PerpsMarginCorporateActionFinalSettlementTermsOptional = TypedDict("_PerpsMarginCorporateActionFinalSettlementTermsOptional", {'settlement_price_per_share': 'str | None', 'settlement_price_per_contract': 'str | None'}, total=False)
class PerpsMarginCorporateActionFinalSettlementTerms(_PerpsMarginCorporateActionFinalSettlementTermsOptional):
    reason: str

_PerpsMarginCorporateActionSplitTermsOptional = TypedDict("_PerpsMarginCorporateActionSplitTermsOptional", {'ratio_old': 'str | None', 'ratio_new': 'str | None', 'ex_date': 'str | None'}, total=False)
class PerpsMarginCorporateActionSplitTerms(_PerpsMarginCorporateActionSplitTermsOptional):
    pass

class PerpsMarginCorporateActionTickerChangeTerms(TypedDict):
    old_ticker: str
    new_ticker: str

class PerpsMarginEnabledResponse(TypedDict):
    enabled: bool

class PerpsMarginFeeTierRate(TypedDict):
    fee_schedule: str
    tier: int | Decimal
    maker_fee_rate: int | Decimal
    taker_fee_rate: int | Decimal

_PerpsMarginFillOptional = TypedDict("_PerpsMarginFillOptional", {'order_source': 'str | None'}, total=False)
class PerpsMarginFill(_PerpsMarginFillOptional):
    fill_id: str
    order_id: str
    is_taker: bool
    side: str
    count: str
    created_time: str
    ticker: str
    price: str
    entry_price: str
    fees: str
    realized_pnl: str

class PerpsMarginFundingHistoryEntry(TypedDict):
    market_ticker: str
    funding_time: str
    funding_rate: int | Decimal
    mark_price: str
    funding_amount: str
    quantity: str
    subaccount_number: int | Decimal | None
    subaccount_type: str

class PerpsMarginFundingRate(TypedDict):
    market_ticker: str
    funding_time: str
    funding_rate: int | Decimal
    mark_price: str

_PerpsMarginHolidayWindowOptional = TypedDict("_PerpsMarginHolidayWindowOptional", {'reason': 'str | None'}, total=False)
_PerpsMarginHolidayWindowWireKeys = TypedDict("_PerpsMarginHolidayWindowWireKeys", {'from': 'str'})
class PerpsMarginHolidayWindow(_PerpsMarginHolidayWindowWireKeys, _PerpsMarginHolidayWindowOptional):
    until: str

_PerpsMarginMarketOptional = TypedDict("_PerpsMarginMarketOptional", {'market_id': 'str | None', 'leverage_estimate': 'int | Decimal | None', 'leverage_estimates': 'dict[str, int | Decimal] | None', 'long_leverage_estimates': 'dict[str, int | Decimal] | None', 'short_leverage_estimates': 'dict[str, int | Decimal] | None', 'price': 'str | None', 'volume': 'str | None', 'volume_notional_value_dollars': 'str | None', 'open_interest': 'str | None', 'open_interest_notional_value_dollars': 'str | None', 'volume_24h': 'str | None', 'volume_24h_notional_value_dollars': 'str | None', 'bid': 'str | None', 'bid_size_fp': 'str | None', 'ask': 'str | None', 'ask_size_fp': 'str | None', 'settlement_mark_price': 'PerpsTickerPrice | None', 'liquidation_mark_price': 'PerpsTickerPrice | None', 'reference_price': 'PerpsTickerPrice | None', 'asset_class': 'str | None', 'product_metadata': 'dict[str, Any] | None'}, total=False)
class PerpsMarginMarket(_PerpsMarginMarketOptional):
    ticker: str
    title: str
    market_version: int | Decimal
    exchange_index: int | Decimal
    contract_size: str
    underlying_multiplier: str
    tick_size: str
    status: str
    fractional_trading_enabled: bool
    schedule: PerpsMarginMarketSchedule
    funding_deadband: int | Decimal
    funding_cap: int | Decimal
    funding_limit_unit: str
    halt_sources: list[str]
    funding_invalid_premium_handling: str

class PerpsMarginMarketCandlestick(TypedDict):
    end_period_ts: int | Decimal
    bid: PerpsBidAskDistributionHistorical
    ask: PerpsBidAskDistributionHistorical
    price: PerpsPriceDistributionHistorical
    volume: str
    volume_notional_value_dollars: str
    open_interest: str
    open_interest_notional_value_dollars: str

class PerpsMarginMarketOrderbook(TypedDict):
    ticker: str
    orderbook: PerpsMarginOrderbookCount

class PerpsMarginMarketResponse(TypedDict):
    market: PerpsMarginMarket

class PerpsMarginMarketSchedule(TypedDict):
    is_open: bool
    next_close_ts: int | Decimal | None
    next_open_ts: int | Decimal | None

class PerpsMarginMarketScheduleDetails(TypedDict):
    kalshi_trading_holidays: list[PerpsMarginHolidayWindow]

_PerpsMarginOrderOptional = TypedDict("_PerpsMarginOrderOptional", {'expiration_time': 'str | None', 'created_time': 'str | None', 'last_update_time': 'str | None', 'self_trade_prevention_type': 'str | None', 'cancel_order_on_pause': 'bool | None', 'order_group_id': 'str | None', 'order_source': 'str | None', 'order_reason': 'str | None'}, total=False)
class PerpsMarginOrder(_PerpsMarginOrderOptional):
    order_id: str
    user_id: str
    client_order_id: str
    ticker: str
    side: str
    last_update_reason: str
    price: str
    fill_count: str
    remaining_count: str

class PerpsMarginOrderbookCount(TypedDict):
    bids: list[PerpsPriceLevelDollarsCountFp]
    asks: list[PerpsPriceLevelDollarsCountFp]

class PerpsMarginOrderbookResponse(TypedDict):
    orderbook: PerpsMarginOrderbookCount

_PerpsMarginPositionOptional = TypedDict("_PerpsMarginPositionOptional", {'margin_used': 'str | None', 'roe': 'int | Decimal | None'}, total=False)
class PerpsMarginPosition(_PerpsMarginPositionOptional):
    subaccount: int | Decimal
    subaccount_type: str
    market_ticker: str
    position: str
    entry_price: str
    unrealized_pnl: str
    fees: str
    is_portfolio: bool

class PerpsMarginPremiumIndexPoint(TypedDict):
    second_ts: str
    premium_index: str

_PerpsMarginRiskPositionOptional = TypedDict("_PerpsMarginRiskPositionOptional", {'maintenance_margin_required': 'str | None', 'position_leverage': 'int | Decimal | None', 'estimated_liquidation_price': 'str | None'}, total=False)
class PerpsMarginRiskPosition(_PerpsMarginRiskPositionOptional):
    subaccount: int | Decimal
    subaccount_type: str
    market_ticker: str
    position: str
    mark_price: str
    position_notional: str
    is_portfolio: bool

class PerpsMarginSubaccountBalance(TypedDict):
    subaccount: int | Decimal
    subaccount_type: str
    position_value: str
    account_equity: str
    maintenance_margin: str
    initial_margin: str
    resting_orders_margin: str
    available_balance: str

class PerpsMarginTrade(TypedDict):
    trade_id: str
    ticker: str
    count: str
    price: str
    created_time: str
    taker_side: str
    is_block_trade: bool

_PerpsNotionalRiskLimitResponseOptional = TypedDict("_PerpsNotionalRiskLimitResponseOptional", {'member_notional_value_risk_limit': 'str | None', 'effective_account_notional_value_risk_limit': 'str | None'}, total=False)
class PerpsNotionalRiskLimitResponse(_PerpsNotionalRiskLimitResponseOptional):
    default_notional_value_risk_limit: str
    notional_value_risk_limits_by_market_ticker: dict[str, str]
    total_current_usage: str
    current_usage_by_market_ticker: dict[str, str]

_PerpsOrderGroupOptional = TypedDict("_PerpsOrderGroupOptional", {'contracts_limit_fp': 'str | None', 'exchange_index': 'int | Decimal | None'}, total=False)
class PerpsOrderGroup(_PerpsOrderGroupOptional):
    id: str
    is_auto_cancel_enabled: bool

class PerpsPriceDistributionHistorical(TypedDict):
    open: str | None
    low: str | None
    high: str | None
    close: str | None
    mean: str | None
    previous: str | None

_PerpsSetCrossExitTriggerRequestOptional = TypedDict("_PerpsSetCrossExitTriggerRequestOptional", {'count': 'str | None', 'anchor_order_id': 'str | None', 'client_trigger_id': 'str | None', 'kind': 'str | None', 'stop_loss_price': 'str | None', 'take_profit_price': 'str | None', 'trail_amount': 'str | None', 'trail_bps': 'int | Decimal | None'}, total=False)
class PerpsSetCrossExitTriggerRequest(_PerpsSetCrossExitTriggerRequestOptional):
    pass

_PerpsSetIsolatedExitTriggerRequestOptional = TypedDict("_PerpsSetIsolatedExitTriggerRequestOptional", {'kind': 'str | None', 'stop_loss_price': 'str | None', 'take_profit_price': 'str | None', 'trail_amount': 'str | None', 'trail_bps': 'int | Decimal | None'}, total=False)
class PerpsSetIsolatedExitTriggerRequest(_PerpsSetIsolatedExitTriggerRequestOptional):
    pass

class PerpsTickerPrice(TypedDict):
    price: str
    ts_ms: int | Decimal

_PerpsUpdateExitTriggerRequestOptional = TypedDict("_PerpsUpdateExitTriggerRequestOptional", {'stop_loss_price': 'str | None', 'take_profit_price': 'str | None'}, total=False)
class PerpsUpdateExitTriggerRequest(_PerpsUpdateExitTriggerRequestOptional):
    pass

class PerpsUpdateFCMNotionalRiskLimitRequest(TypedDict):
    notional_value_risk_limit: str

_PerpsUpdateFCMSubtraderRiskControlsRequestOptional = TypedDict("_PerpsUpdateFCMSubtraderRiskControlsRequestOptional", {'market_ticker': 'str | None', 'asset_class': 'str | None'}, total=False)
class PerpsUpdateFCMSubtraderRiskControlsRequest(_PerpsUpdateFCMSubtraderRiskControlsRequestOptional):
    subtrader_id: str
    im_cap: str

_PerpsUpdateOrderGroupLimitRequestOptional = TypedDict("_PerpsUpdateOrderGroupLimitRequestOptional", {'contracts_limit': 'int | Decimal | None', 'contracts_limit_fp': 'str | None'}, total=False)
class PerpsUpdateOrderGroupLimitRequest(_PerpsUpdateOrderGroupLimitRequestOptional):
    pass

_ResetMarginOrderGroupParamsOptional = TypedDict("_ResetMarginOrderGroupParamsOptional", {'subaccount': 'int | Decimal | None', 'body': 'PerpsEmptyResponse | None'}, total=False)
class ResetMarginOrderGroupParams(_ResetMarginOrderGroupParamsOptional):
    order_group_id: str

class ResetMarginOrderGroupResponse(TypedDict):
    pass

_SetCrossMarginExitTriggerParamsOptional = TypedDict("_SetCrossMarginExitTriggerParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class SetCrossMarginExitTriggerParams(_SetCrossMarginExitTriggerParamsOptional):
    ticker: str
    body: PerpsSetCrossExitTriggerRequest

_SetCrossMarginExitTriggerResponseOptional = TypedDict("_SetCrossMarginExitTriggerResponseOptional", {'status_reason': 'str | None', 'triggered_leg': 'str | None', 'triggered_order_id': 'str | None', 'anchor_order_id': 'str | None', 'client_trigger_id': 'str | None', 'stop_loss_price': 'str | None', 'take_profit_price': 'str | None', 'trail_amount': 'str | None', 'trail_bps': 'int | Decimal | None', 'watermark_price': 'str | None', 'effective_stop_price': 'str | None'}, total=False)
class SetCrossMarginExitTriggerResponse(_SetCrossMarginExitTriggerResponseOptional):
    id: str
    ticker: str
    kind: str
    status: str
    count: str
    filled_count: str
    created_time: str
    updated_time: str

class SetIsolatedMarginExitTriggerParams(TypedDict):
    ticker: str
    body: PerpsSetIsolatedExitTriggerRequest

_SetIsolatedMarginExitTriggerResponseOptional = TypedDict("_SetIsolatedMarginExitTriggerResponseOptional", {'status_reason': 'str | None', 'triggered_leg': 'str | None', 'triggered_order_id': 'str | None', 'anchor_order_id': 'str | None', 'client_trigger_id': 'str | None', 'stop_loss_price': 'str | None', 'take_profit_price': 'str | None', 'trail_amount': 'str | None', 'trail_bps': 'int | Decimal | None', 'watermark_price': 'str | None', 'effective_stop_price': 'str | None'}, total=False)
class SetIsolatedMarginExitTriggerResponse(_SetIsolatedMarginExitTriggerResponseOptional):
    id: str
    ticker: str
    kind: str
    status: str
    count: str
    filled_count: str
    created_time: str
    updated_time: str

_TriggerMarginOrderGroupParamsOptional = TypedDict("_TriggerMarginOrderGroupParamsOptional", {'subaccount': 'int | Decimal | None', 'body': 'PerpsEmptyResponse | None'}, total=False)
class TriggerMarginOrderGroupParams(_TriggerMarginOrderGroupParamsOptional):
    order_group_id: str

class TriggerMarginOrderGroupResponse(TypedDict):
    pass

_UpdateCrossMarginExitTriggerParamsOptional = TypedDict("_UpdateCrossMarginExitTriggerParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class UpdateCrossMarginExitTriggerParams(_UpdateCrossMarginExitTriggerParamsOptional):
    ticker: str
    trigger_id: str
    body: PerpsUpdateExitTriggerRequest

_UpdateCrossMarginExitTriggerResponseOptional = TypedDict("_UpdateCrossMarginExitTriggerResponseOptional", {'status_reason': 'str | None', 'triggered_leg': 'str | None', 'triggered_order_id': 'str | None', 'anchor_order_id': 'str | None', 'client_trigger_id': 'str | None', 'stop_loss_price': 'str | None', 'take_profit_price': 'str | None', 'trail_amount': 'str | None', 'trail_bps': 'int | Decimal | None', 'watermark_price': 'str | None', 'effective_stop_price': 'str | None'}, total=False)
class UpdateCrossMarginExitTriggerResponse(_UpdateCrossMarginExitTriggerResponseOptional):
    id: str
    ticker: str
    kind: str
    status: str
    count: str
    filled_count: str
    created_time: str
    updated_time: str

class UpdateFcmNotionalRiskLimitParams(TypedDict):
    body: PerpsUpdateFCMNotionalRiskLimitRequest

class UpdateFcmNotionalRiskLimitResponse(TypedDict):
    pass

class UpdateFcmSubtraderRiskControlsParams(TypedDict):
    body: PerpsUpdateFCMSubtraderRiskControlsRequest

class UpdateFcmSubtraderRiskControlsResponse(TypedDict):
    pass

_UpdateMarginOrderGroupLimitParamsOptional = TypedDict("_UpdateMarginOrderGroupLimitParamsOptional", {'subaccount': 'int | Decimal | None'}, total=False)
class UpdateMarginOrderGroupLimitParams(_UpdateMarginOrderGroupLimitParamsOptional):
    order_group_id: str
    body: PerpsUpdateOrderGroupLimitRequest

class UpdateMarginOrderGroupLimitResponse(TypedDict):
    pass

CancelAllMarginOrdersResponse = Any
ClearingAssetClass = str
ClearingDeleteMarginFcmApiKeyResponse = Any
ClearingDeleteMarginSubtraderGroupResponse = Any
ClearingUpdateMarginSubtraderGroupResponse = Any
DeleteCrossMarginExitTriggerByIdResponse = Any
DeleteCrossMarginExitTriggerResponse = Any
DeleteIsolatedMarginExitTriggerResponse = Any
GetMarginMarketSchedulesResponse = dict[str, PerpsMarginMarketScheduleDetails]
PerpsBookSide = str
PerpsExchangeIndex = int | Decimal
PerpsExchangeInstance = str
PerpsExitTriggerKind = str
PerpsExitTriggerLeg = str
PerpsExitTriggerReason = str
PerpsExitTriggerStatus = str
PerpsFixedPointCount = str
PerpsFixedPointDollars = str
PerpsGetMarginMarketSchedulesResponse = dict[str, PerpsMarginMarketScheduleDetails]
PerpsLastUpdateReason = str
PerpsMarginMarketStatus = str
PerpsOrderReason = str
PerpsOrderSource = str
PerpsPriceLevelDollarsCountFp = list[str]
PerpsSelfTradePreventionType = str
PerpsSubaccountType = str
