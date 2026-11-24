// Generated from schema/kalshi_perps.json.
import type { HttpValue } from "./index";
export type CancelAllMarginOrdersResponse = HttpValue;
export type ClearingAssetClass = string;
export type ClearingDeleteMarginFcmApiKeyResponse = HttpValue;
export type ClearingDeleteMarginSubtraderGroupResponse = HttpValue;
export type ClearingUpdateMarginSubtraderGroupResponse = HttpValue;
export type DeleteCrossMarginExitTriggerByIdResponse = HttpValue;
export type DeleteCrossMarginExitTriggerResponse = HttpValue;
export type DeleteIsolatedMarginExitTriggerResponse = HttpValue;
export type GetMarginMarketSchedulesResponse = Record<string, PerpsMarginMarketScheduleDetails>;
export type PerpsBookSide = string;
export type PerpsExchangeIndex = number | bigint | string;
export type PerpsExchangeInstance = string;
export type PerpsExitTriggerKind = string;
export type PerpsExitTriggerLeg = string;
export type PerpsExitTriggerReason = string;
export type PerpsExitTriggerStatus = string;
export type PerpsFixedPointCount = string;
export type PerpsFixedPointDollars = string;
export type PerpsGetMarginMarketSchedulesResponse = Record<string, PerpsMarginMarketScheduleDetails>;
export type PerpsLastUpdateReason = string;
export type PerpsMarginMarketStatus = string;
export type PerpsOrderReason = string;
export type PerpsOrderSource = string;
export type PerpsPriceLevelDollarsCountFp = Array<string>;
export type PerpsSelfTradePreventionType = string;
export type PerpsSubaccountType = string;

export interface AmendMarginOrderParams {
  [field: string]: unknown;
  "order_id": string;
  "subaccount"?: number | bigint | string | null;
  "body": PerpsAmendMarginOrderRequest;
}

export interface AmendMarginOrderResponse {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "remaining_count"?: string | null;
  "fill_count"?: string | null;
  "average_fill_price"?: string | null;
  "average_fee_paid"?: string | null;
}

export interface ApplyMarginSubaccountTransferParams {
  [field: string]: unknown;
  "body": PerpsApplySubaccountTransferRequest;
}

export interface ApplyMarginSubaccountTransferResponse {
  [field: string]: unknown;
}

export interface CancelAllMarginOrdersParams {
  [field: string]: unknown;
  "subaccount"?: number | bigint | string | null;
}

export interface CancelMarginOrderParams {
  [field: string]: unknown;
  "order_id": string;
  "subaccount"?: number | bigint | string | null;
}

export interface CancelMarginOrderResponse {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "reduced_by": string;
}

export interface ClearingAssetClassFundingEstimate {
  [field: string]: unknown;
  "user_breakdown": ClearingFundingEstimate;
  "subtrader_breakdowns"?: Record<string, ClearingFundingEstimate> | null;
  "group_breakdowns"?: Record<string, ClearingFundingEstimate> | null;
  "omitted_subtrader_count": number | bigint | string;
  "omitted_group_count": number | bigint | string;
  "next_funding_time": string;
}

export interface ClearingAssetClassSettlementEstimate {
  [field: string]: unknown;
  "user_breakdown"?: ClearingSettlementEstimate | null;
  "subtrader_breakdowns"?: Record<string, ClearingSettlementEstimate> | null;
  "group_breakdowns"?: Record<string, ClearingSettlementEstimate> | null;
  "omitted_subtrader_count"?: number | bigint | string | null;
  "omitted_group_count"?: number | bigint | string | null;
  "next_runtime": string;
}

export interface ClearingCorporateActionPaymentDetail {
  [field: string]: unknown;
  "id": string;
  "type": string;
  "ca_id": string;
  "market_id": string;
  "market_ticker": string;
  "asset_class": string;
  "subtrader_id": string;
  "position_quantity_fp": string;
  "post_position_quantity_fp": string;
  "payment_amount_centicents": number | bigint | string;
  "payment_amount_fp": string;
  "payment_time": string | null;
  "settlement_execution_time": string;
  "pre_split_price_centicents"?: number | bigint | string | null;
  "post_split_price_centicents"?: number | bigint | string | null;
  "exchange_ts"?: string | null;
  "created_ts": string;
}

export interface ClearingCreateMarginFcmApiKeyParams {
  [field: string]: unknown;
  "body": ClearingCreateMarginFcmApiKeyRequest;
}

export interface ClearingCreateMarginFcmApiKeyRequest {
  [field: string]: unknown;
  "name": string;
  "public_key": string;
  "fcm_subtrader_id": string;
}

export interface ClearingCreateMarginFcmApiKeyResponse {
  [field: string]: unknown;
  "api_key_id": string;
  "warning"?: string | null;
}

export interface ClearingCreateMarginSubtraderGroupParams {
  [field: string]: unknown;
  "body": ClearingCreateMarginSubtraderGroupRequest;
}

export interface ClearingCreateMarginSubtraderGroupRequest {
  [field: string]: unknown;
  "subtrader_ids": Array<string>;
}

export interface ClearingCreateMarginSubtraderGroupResponse {
  [field: string]: unknown;
  "group_id": string;
}

export interface ClearingDeleteMarginFcmApiKeyParams {
  [field: string]: unknown;
  "api_key_id": string;
}

export interface ClearingDeleteMarginSubtraderGroupParams {
  [field: string]: unknown;
  "group_id": string;
}

export interface ClearingError {
  [field: string]: unknown;
  "code": string;
  "message": string;
  "details"?: string | null;
  "service"?: string | null;
}

export interface ClearingEstimatePortfolioMaintenanceMarginParams {
  [field: string]: unknown;
  "body": ClearingEstimatePortfolioMaintenanceMarginRequest;
}

export interface ClearingEstimatePortfolioMaintenanceMarginPosition {
  [field: string]: unknown;
  "market_ticker": string;
  "quantity": number | bigint | string;
  "price"?: string | null;
}

export interface ClearingEstimatePortfolioMaintenanceMarginRequest {
  [field: string]: unknown;
  "asset_class": string;
  "date"?: string | null;
  "clearing_type"?: string | null;
  "positions": Array<ClearingEstimatePortfolioMaintenanceMarginPosition>;
}

export interface ClearingEstimatePortfolioMaintenanceMarginResponse {
  [field: string]: unknown;
  "maintenance_margin_fp"?: string | null;
  "base_margin_fp"?: string | null;
  "hvar_fp"?: string | null;
  "apc_fp"?: string | null;
  "funding_addon_fp"?: string | null;
  "liquidation_addon_fp"?: string | null;
}

export interface ClearingFundingEstimate {
  [field: string]: unknown;
  "funding_amount_centicents": number | bigint | string;
  "funding_amount_fp": string;
  "positions"?: Record<string, ClearingMarketFundingEstimate> | null;
}

export interface ClearingFundingPaymentDetail {
  [field: string]: unknown;
  "id": string;
  "market_ticker": string;
  "subtrader_id": string;
  "funding_time": string;
  "position_quantity_fp": string;
  "notional_value_centicents": number | bigint | string;
  "notional_value_fp": string;
  "funding_amount_centicents": number | bigint | string;
  "funding_amount_fp": string;
}

export interface ClearingGenerateMarginFcmApiKeyParams {
  [field: string]: unknown;
  "body": ClearingGenerateMarginFcmApiKeyRequest;
}

export interface ClearingGenerateMarginFcmApiKeyRequest {
  [field: string]: unknown;
  "name": string;
  "fcm_subtrader_id": string;
}

export interface ClearingGenerateMarginFcmApiKeyResponse {
  [field: string]: unknown;
  "api_key_id": string;
  "private_key": string;
  "warning"?: string | null;
}

export interface ClearingGetActiveMarginObligationsParams {
  [field: string]: unknown;
}

export interface ClearingGetActiveMarginObligationsResponse {
  [field: string]: unknown;
  "obligations": Array<ClearingObligationEntry>;
}

export interface ClearingGetFundingEstimateByAssetClassParams {
  [field: string]: unknown;
}

export interface ClearingGetFundingEstimateByAssetClassResponse {
  [field: string]: unknown;
  "estimates": Record<string, ClearingAssetClassFundingEstimate>;
}

export interface ClearingGetGuarantyFundBalanceParams {
  [field: string]: unknown;
}

export interface ClearingGetGuarantyFundBalanceResponse {
  [field: string]: unknown;
  "user_id": string;
  "amount_centicents": number | bigint | string;
  "updated_ts": string;
}

export interface ClearingGetLargeTraderPositionsParams {
  [field: string]: unknown;
  "report_date"?: string | null;
}

export interface ClearingGetLargeTraderPositionsResponse {
  [field: string]: unknown;
  "user_id"?: string | null;
  "report_date"?: string | null;
  "positions"?: Array<ClearingGetLargeTraderPositionsResponsePositionsItem> | null;
}

export interface ClearingGetLargeTraderPositionsResponsePositionsItem {
  [field: string]: unknown;
  "market_ticker"?: string | null;
  "position"?: number | bigint | string | null;
  "exit_day"?: boolean | null;
  "reportable_threshold"?: number | bigint | string | null;
}

export interface ClearingGetMaintenanceMarginMetadataParams {
  [field: string]: unknown;
  "asset_class": string;
  "date": string;
}

export interface ClearingGetMaintenanceMarginMetadataResponse {
  [field: string]: unknown;
  "asset_class": string;
  "base_tail_percentile": number | bigint | string;
  "funding_tail_percentile": number | bigint | string;
  "matrices": ClearingMaintenanceMarginMatrices;
  "liquidation_configs": Array<ClearingMaintenanceMarginLiquidationConfig>;
  "subgroups": Array<Array<string>>;
}

export interface ClearingGetMarginFundingScheduleParams {
  [field: string]: unknown;
  "asset_class": string;
}

export interface ClearingGetMarginFundingScheduleResponse {
  [field: string]: unknown;
  "schedule": string;
}

export interface ClearingGetMarginReportsParams {
  [field: string]: unknown;
  "start_date": string;
  "end_date": string;
}

export interface ClearingGetMarginReportsResponse {
  [field: string]: unknown;
  "reports": Array<ClearingMarginReport>;
}

export interface ClearingGetMarginSubtraderGroupsParams {
  [field: string]: unknown;
}

export interface ClearingGetMarginSubtraderGroupsResponse {
  [field: string]: unknown;
  "groups": Array<ClearingMarginSubtraderGroup>;
}

export interface ClearingGetMemberFundingPaymentsParams {
  [field: string]: unknown;
  "funding_time": string;
  "market_ticker"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface ClearingGetMemberFundingPaymentsResponse {
  [field: string]: unknown;
  "payments": Array<ClearingMemberFundingPayment>;
  "cursor"?: string | null;
}

export interface ClearingGetObligationCorporateActionPaymentsParams {
  [field: string]: unknown;
  "obligation_id": string;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface ClearingGetObligationCorporateActionPaymentsResponse {
  [field: string]: unknown;
  "corporate_action_payments": Array<ClearingCorporateActionPaymentDetail>;
  "cursor"?: string | null;
}

export interface ClearingGetObligationFundingPaymentsParams {
  [field: string]: unknown;
  "obligation_id": string;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface ClearingGetObligationFundingPaymentsResponse {
  [field: string]: unknown;
  "funding_payments": Array<ClearingFundingPaymentDetail>;
  "cursor"?: string | null;
}

export interface ClearingGetObligationHistoryParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "settlement_start_ts"?: string | null;
  "settlement_end_ts"?: string | null;
}

export interface ClearingGetObligationHistoryResponse {
  [field: string]: unknown;
  "obligations": Array<ClearingObligationEntry>;
  "cursor"?: string | null;
}

export interface ClearingGetObligationMaintenanceMarginDetailsParams {
  [field: string]: unknown;
  "obligation_id": string;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface ClearingGetObligationMaintenanceMarginDetailsResponse {
  [field: string]: unknown;
  "maintenance_margin_details": Array<ClearingMaintenanceMarginDetail>;
  "cursor"?: string | null;
}

export interface ClearingGetObligationSettlementDetailsParams {
  [field: string]: unknown;
  "obligation_id": string;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
}

export interface ClearingGetObligationSettlementDetailsResponse {
  [field: string]: unknown;
  "settlement_details": Array<ClearingSettlementDetail>;
  "cursor"?: string | null;
}

export interface ClearingGetSettlementBalanceHistoryParams {
  [field: string]: unknown;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "start_ts"?: string | null;
  "end_ts"?: string | null;
}

export interface ClearingGetSettlementBalanceHistoryResponse {
  [field: string]: unknown;
  "entries": Array<ClearingSettlementBalanceHistoryEntry>;
  "cursor"?: string | null;
}

export interface ClearingGetSettlementBalanceParams {
  [field: string]: unknown;
}

export interface ClearingGetSettlementBalanceResponse {
  [field: string]: unknown;
  "user_id": string;
  "balance_available_centicents": number | bigint | string;
  "balance_available_fp": string;
  "locked_balance_centicents"?: number | bigint | string | null;
  "locked_balance_fp"?: string | null;
}

export interface ClearingGetSettlementBalanceWithdrawalParams {
  [field: string]: unknown;
  "id": string;
}

export interface ClearingGetSettlementBalanceWithdrawalResponse {
  [field: string]: unknown;
  "id": string;
  "amount": string;
  "status": string;
  "created_ts": string;
}

export interface ClearingGetSettlementEstimateByAssetClassParams {
  [field: string]: unknown;
}

export interface ClearingGetSettlementEstimateByAssetClassResponse {
  [field: string]: unknown;
  "estimates": Record<string, ClearingAssetClassSettlementEstimate>;
  "settlement_balance_centicents": number | bigint | string;
  "settlement_balance_fp": string;
}

export interface ClearingGetSettlementPricesParams {
  [field: string]: unknown;
  "asset_class": string;
  "settlement_time": string;
}

export interface ClearingGetSettlementPricesResponse {
  [field: string]: unknown;
  "settlement_prices": Record<string, number | bigint | string>;
}

export interface ClearingListMarginFcmApiKeysParams {
  [field: string]: unknown;
  "fcm_subtrader_id"?: string | null;
}

export interface ClearingListMarginFcmApiKeysResponse {
  [field: string]: unknown;
  "api_keys": Array<ClearingMarginFcmApiKey>;
}

export interface ClearingMaintenanceMarginDetail {
  [field: string]: unknown;
  "id": string;
  "subtrader_id": string;
  "margin_group_id"?: string | null;
  "maintenance_margin_centicents": number | bigint | string;
  "maintenance_margin_delta_centicents": number | bigint | string;
}

export interface ClearingMaintenanceMarginLiquidationConfig {
  [field: string]: unknown;
  "market_ticker": string;
  "market_impact_volatility": number | bigint | string;
  "market_impact_coefficient": number | bigint | string;
  "market_impact_exponent": number | bigint | string;
  "market_impact_forecasted_volume": number | bigint | string;
  "spread_rate": number | bigint | string;
}

export interface ClearingMaintenanceMarginMatrices {
  [field: string]: unknown;
  "hvar"?: ClearingMaintenanceMarginMatrix | null;
  "apc"?: ClearingMaintenanceMarginMatrix | null;
  "aug"?: ClearingMaintenanceMarginMatrix | null;
  "funding"?: ClearingMaintenanceMarginMatrix | null;
}

export interface ClearingMaintenanceMarginMatrix {
  [field: string]: unknown;
  "market_tickers": Array<string>;
  "returns": Array<Array<number | bigint | string>>;
}

export interface ClearingMarginFcmApiKey {
  [field: string]: unknown;
  "api_key_id": string;
  "name": string;
  "fcm_subtrader_id": string;
}

export interface ClearingMarginReport {
  [field: string]: unknown;
  "report_type": string;
  "url": string;
  "date": string;
  "created_ts": string;
  "is_end_of_day": boolean;
  "snapshot_ts"?: string | null;
}

export interface ClearingMarginSubtraderGroup {
  [field: string]: unknown;
  "group_id": string;
  "member_subtrader_ids": Array<string>;
}

export interface ClearingMarketFundingEstimate {
  [field: string]: unknown;
  "quantity_centicount": number | bigint | string;
  "funding_amount_centicents": number | bigint | string;
  "funding_amount_fp": string;
}

export interface ClearingMarketSettlementEstimate {
  [field: string]: unknown;
  "quantity_centicount": number | bigint | string;
  "variation_margin_centicents": number | bigint | string;
  "variation_margin_fp": string;
  "notional_value_centicents": number | bigint | string;
  "notional_value_fp": string;
  "session_avg_price_fp"?: string | null;
}

export interface ClearingMemberFundingPayment {
  [field: string]: unknown;
  "id": string;
  "market_ticker": string;
  "subtrader_id": string;
  "funding_time": string;
  "position_quantity_fp": string;
  "notional_value_centicents": number | bigint | string;
  "notional_value_fp": string;
  "funding_amount_centicents": number | bigint | string;
  "funding_amount_fp": string;
  "settlement_execution_time": string;
}

export interface ClearingObligationEntry {
  [field: string]: unknown;
  "id": string;
  "user_id": string;
  "amount_centicents": number | bigint | string;
  "amount_fp": string;
  "fees_centicents": number | bigint | string;
  "maintenance_margin_centicents": number | bigint | string;
  "pnl_centicents": number | bigint | string;
  "pnl_fp": string;
  "execution_time": string;
  "last_updated_ts": string;
  "asset_class": string;
  "receives": Array<ClearingObligationReceiveInfo>;
  "settlement_details": Array<ClearingSettlementDetail>;
  "maintenance_margin_details": Array<ClearingMaintenanceMarginDetail>;
  "funding_payments": Array<ClearingFundingPaymentDetail>;
  "corporate_action_payments": Array<ClearingCorporateActionPaymentDetail>;
  "corporate_action_payments_truncated"?: boolean | null;
  "settlement_details_truncated"?: boolean | null;
  "maintenance_margin_details_truncated"?: boolean | null;
  "funding_payments_truncated"?: boolean | null;
}

export interface ClearingObligationInfo {
  [field: string]: unknown;
  "id": string;
  "user_id": string;
  "amount_centicents": number | bigint | string;
  "amount_fp": string;
  "fees_centicents": number | bigint | string;
  "maintenance_margin_centicents": number | bigint | string;
  "pnl_centicents": number | bigint | string;
  "pnl_fp": string;
  "execution_time": string;
  "last_updated_ts": string;
  "asset_class": string;
}

export interface ClearingObligationReceiveInfo {
  [field: string]: unknown;
  "id": string;
  "type": string;
  "amount_centicents": number | bigint | string;
  "amount_fp": string;
  "external_reference": string;
  "created_ts": string;
}

export interface ClearingSettlementBalanceHistoryEntry {
  [field: string]: unknown;
  "balance_delta_centicents": number | bigint | string;
  "balance_delta_fp": string;
  "locked_balance_delta_centicents": number | bigint | string;
  "locked_balance_delta_fp": string;
  "reason": string;
  "business_transaction_id": string;
  "created_ts": string;
}

export interface ClearingSettlementDetail {
  [field: string]: unknown;
  "id": string;
  "market_ticker": string;
  "subtrader_id": string;
  "position_quantity_fp": string;
  "pnl_centicents": number | bigint | string;
  "pnl_fp": string;
  "total_fees_centicents": number | bigint | string;
  "total_amount_centicents": number | bigint | string;
  "total_amount_fp": string;
}

export interface ClearingSettlementEstimate {
  [field: string]: unknown;
  "variation_margin_centicents": number | bigint | string;
  "variation_margin_fp": string;
  "total_fees_centicents": number | bigint | string;
  "maintenance_margin_delta_centicents": number | bigint | string;
  "maintenance_margin_required_centicents": number | bigint | string;
  "total_amount_centicents": number | bigint | string;
  "total_amount_fp": string;
  "positions"?: Record<string, ClearingMarketSettlementEstimate> | null;
}

export interface ClearingUpdateMarginSubtraderGroupParams {
  [field: string]: unknown;
  "group_id": string;
  "body": ClearingUpdateMarginSubtraderGroupRequest;
}

export interface ClearingUpdateMarginSubtraderGroupRequest {
  [field: string]: unknown;
  "subtrader_ids": Array<string>;
}

export interface ClearingWithdrawSettlementBalanceParams {
  [field: string]: unknown;
  "body": ClearingWithdrawSettlementBalanceRequest;
}

export interface ClearingWithdrawSettlementBalanceRequest {
  [field: string]: unknown;
  "amount": string;
  "mfa_code": string;
}

export interface ClearingWithdrawSettlementBalanceResponse {
  [field: string]: unknown;
  "id": string;
}

export interface CreateMarginFcmSubtraderParams {
  [field: string]: unknown;
  "body": PerpsCreateMarginFCMSubtraderRequest;
}

export interface CreateMarginFcmSubtraderResponse {
  [field: string]: unknown;
  "subtrader_id": string;
}

export interface CreateMarginOrderGroupParams {
  [field: string]: unknown;
  "body": PerpsCreateOrderGroupRequest;
}

export interface CreateMarginOrderGroupResponse {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount": number | bigint | string;
  "exchange_index"?: number | bigint | string | null;
}

export interface CreateMarginOrderParams {
  [field: string]: unknown;
  "body": PerpsCreateMarginOrderRequest;
}

export interface CreateMarginOrderResponse {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "fill_count": string;
  "remaining_count": string;
  "average_fill_price"?: string | null;
  "average_fee_paid"?: string | null;
}

export interface CreateMarginSubaccountParams {
  [field: string]: unknown;
}

export interface CreateMarginSubaccountResponse {
  [field: string]: unknown;
  "subaccount_number": number | bigint | string;
}

export interface DecreaseMarginOrderParams {
  [field: string]: unknown;
  "order_id": string;
  "subaccount"?: number | bigint | string | null;
  "body": PerpsDecreaseMarginOrderRequest;
}

export interface DecreaseMarginOrderResponse {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "remaining_count": string;
}

export interface DeleteCrossMarginExitTriggerByIdParams {
  [field: string]: unknown;
  "ticker": string;
  "trigger_id": string;
  "subaccount"?: number | bigint | string | null;
}

export interface DeleteCrossMarginExitTriggerParams {
  [field: string]: unknown;
  "ticker": string;
  "subaccount"?: number | bigint | string | null;
  "kind"?: string | null;
}

export interface DeleteFcmNotionalRiskLimitParams {
  [field: string]: unknown;
}

export interface DeleteFcmNotionalRiskLimitResponse {
  [field: string]: unknown;
}

export interface DeleteFcmSubtraderRiskControlsParams {
  [field: string]: unknown;
  "subtrader_id": string;
  "market_ticker"?: string | null;
  "asset_class"?: string | null;
}

export interface DeleteFcmSubtraderRiskControlsResponse {
  [field: string]: unknown;
}

export interface DeleteIsolatedMarginExitTriggerParams {
  [field: string]: unknown;
  "ticker": string;
  "kind"?: string | null;
}

export interface DeleteMarginOrderGroupParams {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount"?: number | bigint | string | null;
}

export interface DeleteMarginOrderGroupResponse {
  [field: string]: unknown;
}

export interface GetCrossMarginExitTriggersParams {
  [field: string]: unknown;
  "ticker": string;
  "subaccount"?: number | bigint | string | null;
  "kind"?: string | null;
}

export interface GetCrossMarginExitTriggersResponse {
  [field: string]: unknown;
  "exit_triggers": Array<PerpsExitTrigger>;
}

export interface GetFcmSubtraderRiskControlsParams {
  [field: string]: unknown;
  "subtrader_id"?: string | null;
  "market_ticker"?: string | null;
  "asset_class"?: string | null;
}

export interface GetFcmSubtraderRiskControlsResponse {
  [field: string]: unknown;
  "risk_controls": Array<PerpsFCMSubtraderRiskControls>;
  "notional_limits": Array<PerpsFCMSubtraderNotionalRiskLimit>;
}

export interface GetIsolatedMarginExitTriggersParams {
  [field: string]: unknown;
  "ticker": string;
  "kind"?: string | null;
}

export interface GetIsolatedMarginExitTriggersResponse {
  [field: string]: unknown;
  "exit_triggers": Array<PerpsExitTrigger>;
}

export interface GetMarginBalanceParams {
  [field: string]: unknown;
  "compute_available_balance"?: boolean | null;
}

export interface GetMarginBalanceResponse {
  [field: string]: unknown;
  "subaccount_balances": Array<PerpsMarginSubaccountBalance>;
  "settled_funds": string;
}

export interface GetMarginCorporateActionsParams {
  [field: string]: unknown;
  "ticker"?: string | null;
  "market_id"?: string | null;
}

export interface GetMarginCorporateActionsResponse {
  [field: string]: unknown;
  "corporate_actions": Array<PerpsMarginCorporateAction>;
}

export interface GetMarginEnabledParams {
  [field: string]: unknown;
}

export interface GetMarginEnabledResponse {
  [field: string]: unknown;
  "enabled": boolean;
}

export interface GetMarginExchangeStatusParams {
  [field: string]: unknown;
}

export interface GetMarginExchangeStatusResponse {
  [field: string]: unknown;
  "exchange_active": boolean;
  "trading_active": boolean;
  "intra_exchange_transfers_active"?: boolean | null;
  "exchange_estimated_resume_time"?: string | null;
  "exchange_index_statuses"?: Array<PerpsExchangeIndexStatus> | null;
}

export interface GetMarginFeeTierRatesParams {
  [field: string]: unknown;
}

export interface GetMarginFeeTierRatesResponse {
  [field: string]: unknown;
  "fee_tier_rates": Array<PerpsMarginFeeTierRate>;
}

export interface GetMarginFeeTiersParams {
  [field: string]: unknown;
}

export interface GetMarginFeeTiersResponse {
  [field: string]: unknown;
  "maker_fee_rates": Record<string, number | bigint | string>;
  "taker_fee_rates": Record<string, number | bigint | string>;
}

export interface GetMarginFillsParams {
  [field: string]: unknown;
  "subaccount"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
}

export interface GetMarginFillsResponse {
  [field: string]: unknown;
  "fills": Array<PerpsMarginFill>;
  "cursor": string;
}

export interface GetMarginFundingHistoryParams {
  [field: string]: unknown;
  "ticker"?: string | null;
  "start_date": string;
  "end_date": string;
  "subaccount"?: number | bigint | string | null;
}

export interface GetMarginFundingHistoryResponse {
  [field: string]: unknown;
  "funding_history": Array<PerpsMarginFundingHistoryEntry>;
}

export interface GetMarginFundingRateEstimateParams {
  [field: string]: unknown;
  "ticker": string;
}

export interface GetMarginFundingRateEstimateResponse {
  [field: string]: unknown;
  "market_ticker"?: string | null;
  "computed_time"?: string | null;
  "funding_rate"?: number | bigint | string | null;
  "mark_price"?: string | null;
  "next_funding_time": string;
  "premium_index"?: string | null;
  "premium_index_ts"?: string | null;
}

export interface GetMarginHistoricalFundingRatesParams {
  [field: string]: unknown;
  "ticker"?: string | null;
  "start_ts"?: number | bigint | string | null;
  "end_ts"?: number | bigint | string | null;
}

export interface GetMarginHistoricalFundingRatesResponse {
  [field: string]: unknown;
  "funding_rates": Array<PerpsMarginFundingRate>;
}

export interface GetMarginMarketCandlesticksParams {
  [field: string]: unknown;
  "ticker": string;
  "start_ts": number | bigint | string;
  "end_ts": number | bigint | string;
  "period_interval": number | bigint | string;
  "include_latest_before_start"?: boolean | null;
}

export interface GetMarginMarketCandlesticksResponse {
  [field: string]: unknown;
  "ticker": string;
  "candlesticks": Array<PerpsMarginMarketCandlestick>;
}

export interface GetMarginMarketOrderbookParams {
  [field: string]: unknown;
  "ticker": string;
  "depth"?: number | bigint | string | null;
  "aggregation_tick_size"?: string | null;
}

export interface GetMarginMarketOrderbookResponse {
  [field: string]: unknown;
  "orderbook": PerpsMarginOrderbookCount;
}

export interface GetMarginMarketOrderbooksParams {
  [field: string]: unknown;
  "tickers": Array<string>;
}

export interface GetMarginMarketOrderbooksResponse {
  [field: string]: unknown;
  "orderbooks": Array<PerpsMarginMarketOrderbook>;
}

export interface GetMarginMarketParams {
  [field: string]: unknown;
  "ticker": string;
}

export interface GetMarginMarketResponse {
  [field: string]: unknown;
  "market": PerpsMarginMarket;
}

export interface GetMarginMarketSchedulesParams {
  [field: string]: unknown;
  "tickers": Array<string>;
}

export interface GetMarginMarketsParams {
  [field: string]: unknown;
  "status"?: string | null;
}

export interface GetMarginMarketsResponse {
  [field: string]: unknown;
  "markets": Array<PerpsMarginMarket>;
}

export interface GetMarginNotionalRiskLimitParams {
  [field: string]: unknown;
}

export interface GetMarginNotionalRiskLimitResponse {
  [field: string]: unknown;
  "default_notional_value_risk_limit": string;
  "notional_value_risk_limits_by_market_ticker": Record<string, string>;
  "total_current_usage": string;
  "current_usage_by_market_ticker": Record<string, string>;
  "member_notional_value_risk_limit"?: string | null;
  "effective_account_notional_value_risk_limit"?: string | null;
}

export interface GetMarginOrderGroupParams {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount"?: number | bigint | string | null;
}

export interface GetMarginOrderGroupResponse {
  [field: string]: unknown;
  "is_auto_cancel_enabled": boolean;
  "contracts_limit_fp"?: string | null;
  "orders": Array<string>;
  "exchange_index"?: number | bigint | string | null;
}

export interface GetMarginOrderGroupsParams {
  [field: string]: unknown;
  "subaccount"?: number | bigint | string | null;
}

export interface GetMarginOrderGroupsResponse {
  [field: string]: unknown;
  "order_groups"?: Array<PerpsOrderGroup> | null;
}

export interface GetMarginOrderParams {
  [field: string]: unknown;
  "order_id": string;
}

export interface GetMarginOrderResponse {
  [field: string]: unknown;
  "order": PerpsMarginOrder;
}

export interface GetMarginOrdersParams {
  [field: string]: unknown;
  "ticker"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
  "status"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "subaccount"?: number | bigint | string | null;
}

export interface GetMarginOrdersResponse {
  [field: string]: unknown;
  "orders": Array<PerpsMarginOrder>;
  "cursor": string;
}

export interface GetMarginPositionsParams {
  [field: string]: unknown;
  "subaccount"?: number | bigint | string | null;
  "ticker"?: string | null;
}

export interface GetMarginPositionsResponse {
  [field: string]: unknown;
  "positions": Array<PerpsMarginPosition>;
}

export interface GetMarginPremiumIndexParams {
  [field: string]: unknown;
  "ticker": string;
  "start_ts": number | bigint | string;
  "end_ts": number | bigint | string;
}

export interface GetMarginPremiumIndexResponse {
  [field: string]: unknown;
  "points": Array<PerpsMarginPremiumIndexPoint>;
}

export interface GetMarginRiskParametersParams {
  [field: string]: unknown;
}

export interface GetMarginRiskParametersResponse {
  [field: string]: unknown;
  "liquidation_margin_ratio_threshold": number | bigint | string;
  "queue_entry_margin_ratio_threshold": number | bigint | string;
  "initial_margin_multiplier": Record<string, number | bigint | string>;
}

export interface GetMarginRiskParams {
  [field: string]: unknown;
}

export interface GetMarginRiskResponse {
  [field: string]: unknown;
  "account_leverage"?: number | bigint | string | null;
  "total_position_notional": string;
  "total_maintenance_margin": string;
  "positions": Array<PerpsMarginRiskPosition>;
}

export interface GetMarginTradesParams {
  [field: string]: unknown;
  "ticker"?: string | null;
  "limit"?: number | bigint | string | null;
  "cursor"?: string | null;
  "min_ts"?: number | bigint | string | null;
  "max_ts"?: number | bigint | string | null;
  "is_block_trade"?: boolean | null;
}

export interface GetMarginTradesResponse {
  [field: string]: unknown;
  "trades": Array<PerpsMarginTrade>;
  "cursor": string;
}

export interface GetPerpsAccountApiLimitsParams {
  [field: string]: unknown;
}

export interface GetPerpsAccountApiLimitsResponse {
  [field: string]: unknown;
  "usage_tier": string;
  "read": PerpsBucketLimit;
  "write": PerpsBucketLimit;
  "grants": Array<PerpsApiUsageLevelGrant>;
}

export interface IntraExchangeInstanceTransferParams {
  [field: string]: unknown;
  "body": PerpsIntraExchangeInstanceTransferRequest;
}

export interface IntraExchangeInstanceTransferResponse {
  [field: string]: unknown;
  "transfer_id": string;
}

export interface PerpsAmendMarginOrderRequest {
  [field: string]: unknown;
  "ticker"?: string | null;
  "market_id"?: string | null;
  "side": string;
  "price": string;
  "count": string;
  "client_order_id"?: string | null;
  "updated_client_order_id"?: string | null;
  "expiration_time"?: number | bigint | string | null;
}

export interface PerpsAmendMarginOrderResponse {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "remaining_count"?: string | null;
  "fill_count"?: string | null;
  "average_fill_price"?: string | null;
  "average_fee_paid"?: string | null;
}

export interface PerpsApiUsageLevelGrant {
  [field: string]: unknown;
  "exchange_instance": string;
  "level": string;
  "expires_ts"?: number | bigint | string | null;
  "source": string;
}

export interface PerpsApplySubaccountTransferRequest {
  [field: string]: unknown;
  "client_transfer_id": string;
  "from_subaccount": number | bigint | string;
  "to_subaccount": number | bigint | string;
  "amount_cents": number | bigint | string;
  "exchange_index"?: number | bigint | string | null;
}

export interface PerpsApplySubaccountTransferResponse {
  [field: string]: unknown;
}

export interface PerpsBidAskDistributionHistorical {
  [field: string]: unknown;
  "open": string;
  "low": string;
  "high": string;
  "close": string;
}

export interface PerpsBucketLimit {
  [field: string]: unknown;
  "refill_rate": number | bigint | string;
  "bucket_capacity": number | bigint | string;
}

export interface PerpsCancelMarginOrderResponse {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "reduced_by": string;
}

export interface PerpsCreateMarginFCMSubtraderRequest {
  [field: string]: unknown;
  "subtrader_suffix": string;
  "require_category_cap": boolean;
}

export interface PerpsCreateMarginFCMSubtraderResponse {
  [field: string]: unknown;
  "subtrader_id": string;
}

export interface PerpsCreateMarginOrderRequest {
  [field: string]: unknown;
  "ticker"?: string | null;
  "market_id"?: string | null;
  "client_order_id": string;
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
  "market_version"?: number | bigint | string | null;
}

export interface PerpsCreateMarginOrderResponse {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "fill_count": string;
  "remaining_count": string;
  "average_fill_price"?: string | null;
  "average_fee_paid"?: string | null;
}

export interface PerpsCreateOrderGroupRequest {
  [field: string]: unknown;
  "subaccount"?: number | bigint | string | null;
  "contracts_limit"?: number | bigint | string | null;
  "contracts_limit_fp"?: string | null;
  "exchange_index"?: number | bigint | string | null;
}

export interface PerpsCreateOrderGroupResponse {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount": number | bigint | string;
  "exchange_index"?: number | bigint | string | null;
}

export interface PerpsCreateSubaccountResponse {
  [field: string]: unknown;
  "subaccount_number": number | bigint | string;
}

export interface PerpsDecreaseMarginOrderRequest {
  [field: string]: unknown;
  "reduce_by"?: string | null;
  "reduce_to"?: string | null;
}

export interface PerpsDecreaseMarginOrderResponse {
  [field: string]: unknown;
  "order_id": string;
  "client_order_id"?: string | null;
  "remaining_count": string;
}

export interface PerpsEmptyResponse {
  [field: string]: unknown;
}

export interface PerpsErrorResponse {
  [field: string]: unknown;
  "code"?: string | null;
  "message"?: string | null;
  "details"?: string | null;
}

export interface PerpsExchangeIndexStatus {
  [field: string]: unknown;
  "exchange_index": number | bigint | string;
  "description": string;
  "exchange_active": boolean;
  "trading_active": boolean;
  "intra_exchange_transfers_active": boolean;
}

export interface PerpsExchangeStatus {
  [field: string]: unknown;
  "exchange_active": boolean;
  "trading_active": boolean;
  "intra_exchange_transfers_active"?: boolean | null;
  "exchange_estimated_resume_time"?: string | null;
  "exchange_index_statuses"?: Array<PerpsExchangeIndexStatus> | null;
}

export interface PerpsExitTrigger {
  [field: string]: unknown;
  "id": string;
  "ticker": string;
  "kind": string;
  "status": string;
  "status_reason"?: string | null;
  "triggered_leg"?: string | null;
  "triggered_order_id"?: string | null;
  "anchor_order_id"?: string | null;
  "client_trigger_id"?: string | null;
  "stop_loss_price"?: string | null;
  "take_profit_price"?: string | null;
  "trail_amount"?: string | null;
  "trail_bps"?: number | bigint | string | null;
  "watermark_price"?: string | null;
  "effective_stop_price"?: string | null;
  "count": string;
  "filled_count": string;
  "created_time": string;
  "updated_time": string;
}

export interface PerpsFCMSubtraderNotionalRiskLimit {
  [field: string]: unknown;
  "subtrader_id": string;
  "market_ticker"?: string | null;
  "notional_value_risk_limit": string;
  "current_notional": string;
}

export interface PerpsFCMSubtraderRiskControls {
  [field: string]: unknown;
  "subtrader_id": string;
  "market_ticker"?: string | null;
  "asset_class"?: string | null;
  "im_cap": string;
  "current_im": string;
}

export interface PerpsGetAccountApiLimitsResponse {
  [field: string]: unknown;
  "usage_tier": string;
  "read": PerpsBucketLimit;
  "write": PerpsBucketLimit;
  "grants": Array<PerpsApiUsageLevelGrant>;
}

export interface PerpsGetExitTriggersResponse {
  [field: string]: unknown;
  "exit_triggers": Array<PerpsExitTrigger>;
}

export interface PerpsGetFCMSubtraderRiskControlsResponse {
  [field: string]: unknown;
  "risk_controls": Array<PerpsFCMSubtraderRiskControls>;
  "notional_limits": Array<PerpsFCMSubtraderNotionalRiskLimit>;
}

export interface PerpsGetMarginBalanceResponse {
  [field: string]: unknown;
  "subaccount_balances": Array<PerpsMarginSubaccountBalance>;
  "settled_funds": string;
}

export interface PerpsGetMarginCorporateActionsResponse {
  [field: string]: unknown;
  "corporate_actions": Array<PerpsMarginCorporateAction>;
}

export interface PerpsGetMarginFeeTierRatesResponse {
  [field: string]: unknown;
  "fee_tier_rates": Array<PerpsMarginFeeTierRate>;
}

export interface PerpsGetMarginFeeTiersResponse {
  [field: string]: unknown;
  "maker_fee_rates": Record<string, number | bigint | string>;
  "taker_fee_rates": Record<string, number | bigint | string>;
}

export interface PerpsGetMarginFillsResponse {
  [field: string]: unknown;
  "fills": Array<PerpsMarginFill>;
  "cursor": string;
}

export interface PerpsGetMarginFundingHistoryResponse {
  [field: string]: unknown;
  "funding_history": Array<PerpsMarginFundingHistoryEntry>;
}

export interface PerpsGetMarginFundingRateEstimateResponse {
  [field: string]: unknown;
  "market_ticker"?: string | null;
  "computed_time"?: string | null;
  "funding_rate"?: number | bigint | string | null;
  "mark_price"?: string | null;
  "next_funding_time": string;
  "premium_index"?: string | null;
  "premium_index_ts"?: string | null;
}

export interface PerpsGetMarginHistoricalFundingRatesResponse {
  [field: string]: unknown;
  "funding_rates": Array<PerpsMarginFundingRate>;
}

export interface PerpsGetMarginMarketCandlesticksResponse {
  [field: string]: unknown;
  "ticker": string;
  "candlesticks": Array<PerpsMarginMarketCandlestick>;
}

export interface PerpsGetMarginMarketOrderbooksResponse {
  [field: string]: unknown;
  "orderbooks": Array<PerpsMarginMarketOrderbook>;
}

export interface PerpsGetMarginMarketsResponse {
  [field: string]: unknown;
  "markets": Array<PerpsMarginMarket>;
}

export interface PerpsGetMarginOrderResponse {
  [field: string]: unknown;
  "order": PerpsMarginOrder;
}

export interface PerpsGetMarginOrdersResponse {
  [field: string]: unknown;
  "orders": Array<PerpsMarginOrder>;
  "cursor": string;
}

export interface PerpsGetMarginPositionsResponse {
  [field: string]: unknown;
  "positions": Array<PerpsMarginPosition>;
}

export interface PerpsGetMarginPremiumIndexResponse {
  [field: string]: unknown;
  "points": Array<PerpsMarginPremiumIndexPoint>;
}

export interface PerpsGetMarginRiskParametersResponse {
  [field: string]: unknown;
  "liquidation_margin_ratio_threshold": number | bigint | string;
  "queue_entry_margin_ratio_threshold": number | bigint | string;
  "initial_margin_multiplier": Record<string, number | bigint | string>;
}

export interface PerpsGetMarginRiskResponse {
  [field: string]: unknown;
  "account_leverage"?: number | bigint | string | null;
  "total_position_notional": string;
  "total_maintenance_margin": string;
  "positions": Array<PerpsMarginRiskPosition>;
}

export interface PerpsGetMarginTradesResponse {
  [field: string]: unknown;
  "trades": Array<PerpsMarginTrade>;
  "cursor": string;
}

export interface PerpsGetOrderGroupResponse {
  [field: string]: unknown;
  "is_auto_cancel_enabled": boolean;
  "contracts_limit_fp"?: string | null;
  "orders": Array<string>;
  "exchange_index"?: number | bigint | string | null;
}

export interface PerpsGetOrderGroupsResponse {
  [field: string]: unknown;
  "order_groups"?: Array<PerpsOrderGroup> | null;
}

export interface PerpsIntraExchangeInstanceTransferRequest {
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

export interface PerpsIntraExchangeInstanceTransferResponse {
  [field: string]: unknown;
  "transfer_id": string;
}

export interface PerpsMarginCorporateAction {
  [field: string]: unknown;
  "id": string;
  "market_id": string;
  "market_ticker": string;
  "type": string;
  "status": string;
  "updated_ts": string;
  "execution_time"?: string | null;
  "cash"?: PerpsMarginCorporateActionCashTerms | null;
  "split"?: PerpsMarginCorporateActionSplitTerms | null;
  "final_settlement"?: PerpsMarginCorporateActionFinalSettlementTerms | null;
  "ticker_change"?: PerpsMarginCorporateActionTickerChangeTerms | null;
}

export interface PerpsMarginCorporateActionCashTerms {
  [field: string]: unknown;
  "cash_per_share"?: string | null;
  "cash_per_contract"?: string | null;
  "ex_date"?: string | null;
}

export interface PerpsMarginCorporateActionFinalSettlementTerms {
  [field: string]: unknown;
  "reason": string;
  "settlement_price_per_share"?: string | null;
  "settlement_price_per_contract"?: string | null;
}

export interface PerpsMarginCorporateActionSplitTerms {
  [field: string]: unknown;
  "ratio_old"?: string | null;
  "ratio_new"?: string | null;
  "ex_date"?: string | null;
}

export interface PerpsMarginCorporateActionTickerChangeTerms {
  [field: string]: unknown;
  "old_ticker": string;
  "new_ticker": string;
}

export interface PerpsMarginEnabledResponse {
  [field: string]: unknown;
  "enabled": boolean;
}

export interface PerpsMarginFeeTierRate {
  [field: string]: unknown;
  "fee_schedule": string;
  "tier": number | bigint | string;
  "maker_fee_rate": number | bigint | string;
  "taker_fee_rate": number | bigint | string;
}

export interface PerpsMarginFill {
  [field: string]: unknown;
  "fill_id": string;
  "order_id": string;
  "is_taker": boolean;
  "side": string;
  "count": string;
  "created_time": string;
  "ticker": string;
  "price": string;
  "entry_price": string;
  "fees": string;
  "realized_pnl": string;
  "order_source"?: string | null;
}

export interface PerpsMarginFundingHistoryEntry {
  [field: string]: unknown;
  "market_ticker": string;
  "funding_time": string;
  "funding_rate": number | bigint | string;
  "mark_price": string;
  "funding_amount": string;
  "quantity": string;
  "subaccount_number": number | bigint | string | null;
  "subaccount_type": string;
}

export interface PerpsMarginFundingRate {
  [field: string]: unknown;
  "market_ticker": string;
  "funding_time": string;
  "funding_rate": number | bigint | string;
  "mark_price": string;
}

export interface PerpsMarginHolidayWindow {
  [field: string]: unknown;
  "from": string;
  "until": string;
  "reason"?: string | null;
}

export interface PerpsMarginMarket {
  [field: string]: unknown;
  "market_id"?: string | null;
  "ticker": string;
  "title": string;
  "market_version": number | bigint | string;
  "exchange_index": number | bigint | string;
  "contract_size": string;
  "underlying_multiplier": string;
  "tick_size": string;
  "status": string;
  "fractional_trading_enabled": boolean;
  "leverage_estimate"?: number | bigint | string | null;
  "leverage_estimates"?: Record<string, number | bigint | string> | null;
  "long_leverage_estimates"?: Record<string, number | bigint | string> | null;
  "short_leverage_estimates"?: Record<string, number | bigint | string> | null;
  "price"?: string | null;
  "volume"?: string | null;
  "volume_notional_value_dollars"?: string | null;
  "open_interest"?: string | null;
  "open_interest_notional_value_dollars"?: string | null;
  "volume_24h"?: string | null;
  "volume_24h_notional_value_dollars"?: string | null;
  "bid"?: string | null;
  "bid_size_fp"?: string | null;
  "ask"?: string | null;
  "ask_size_fp"?: string | null;
  "settlement_mark_price"?: PerpsTickerPrice | null;
  "liquidation_mark_price"?: PerpsTickerPrice | null;
  "reference_price"?: PerpsTickerPrice | null;
  "asset_class"?: string | null;
  "product_metadata"?: Record<string, HttpValue> | null;
  "schedule": PerpsMarginMarketSchedule;
  "funding_deadband": number | bigint | string;
  "funding_cap": number | bigint | string;
  "funding_limit_unit": string;
  "halt_sources": Array<string>;
  "funding_invalid_premium_handling": string;
}

export interface PerpsMarginMarketCandlestick {
  [field: string]: unknown;
  "end_period_ts": number | bigint | string;
  "bid": PerpsBidAskDistributionHistorical;
  "ask": PerpsBidAskDistributionHistorical;
  "price": PerpsPriceDistributionHistorical;
  "volume": string;
  "volume_notional_value_dollars": string;
  "open_interest": string;
  "open_interest_notional_value_dollars": string;
}

export interface PerpsMarginMarketOrderbook {
  [field: string]: unknown;
  "ticker": string;
  "orderbook": PerpsMarginOrderbookCount;
}

export interface PerpsMarginMarketResponse {
  [field: string]: unknown;
  "market": PerpsMarginMarket;
}

export interface PerpsMarginMarketSchedule {
  [field: string]: unknown;
  "is_open": boolean;
  "next_close_ts": number | bigint | string | null;
  "next_open_ts": number | bigint | string | null;
}

export interface PerpsMarginMarketScheduleDetails {
  [field: string]: unknown;
  "kalshi_trading_holidays": Array<PerpsMarginHolidayWindow>;
}

export interface PerpsMarginOrder {
  [field: string]: unknown;
  "order_id": string;
  "user_id": string;
  "client_order_id": string;
  "ticker": string;
  "side": string;
  "last_update_reason": string;
  "price": string;
  "fill_count": string;
  "remaining_count": string;
  "expiration_time"?: string | null;
  "created_time"?: string | null;
  "last_update_time"?: string | null;
  "self_trade_prevention_type"?: string | null;
  "cancel_order_on_pause"?: boolean | null;
  "order_group_id"?: string | null;
  "order_source"?: string | null;
  "order_reason"?: string | null;
}

export interface PerpsMarginOrderbookCount {
  [field: string]: unknown;
  "bids": Array<PerpsPriceLevelDollarsCountFp>;
  "asks": Array<PerpsPriceLevelDollarsCountFp>;
}

export interface PerpsMarginOrderbookResponse {
  [field: string]: unknown;
  "orderbook": PerpsMarginOrderbookCount;
}

export interface PerpsMarginPosition {
  [field: string]: unknown;
  "subaccount": number | bigint | string;
  "subaccount_type": string;
  "market_ticker": string;
  "position": string;
  "entry_price": string;
  "unrealized_pnl": string;
  "margin_used"?: string | null;
  "fees": string;
  "roe"?: number | bigint | string | null;
  "is_portfolio": boolean;
}

export interface PerpsMarginPremiumIndexPoint {
  [field: string]: unknown;
  "second_ts": string;
  "premium_index": string;
}

export interface PerpsMarginRiskPosition {
  [field: string]: unknown;
  "subaccount": number | bigint | string;
  "subaccount_type": string;
  "market_ticker": string;
  "position": string;
  "mark_price": string;
  "position_notional": string;
  "maintenance_margin_required"?: string | null;
  "position_leverage"?: number | bigint | string | null;
  "estimated_liquidation_price"?: string | null;
  "is_portfolio": boolean;
}

export interface PerpsMarginSubaccountBalance {
  [field: string]: unknown;
  "subaccount": number | bigint | string;
  "subaccount_type": string;
  "position_value": string;
  "account_equity": string;
  "maintenance_margin": string;
  "initial_margin": string;
  "resting_orders_margin": string;
  "available_balance": string;
}

export interface PerpsMarginTrade {
  [field: string]: unknown;
  "trade_id": string;
  "ticker": string;
  "count": string;
  "price": string;
  "created_time": string;
  "taker_side": string;
  "is_block_trade": boolean;
}

export interface PerpsNotionalRiskLimitResponse {
  [field: string]: unknown;
  "default_notional_value_risk_limit": string;
  "notional_value_risk_limits_by_market_ticker": Record<string, string>;
  "total_current_usage": string;
  "current_usage_by_market_ticker": Record<string, string>;
  "member_notional_value_risk_limit"?: string | null;
  "effective_account_notional_value_risk_limit"?: string | null;
}

export interface PerpsOrderGroup {
  [field: string]: unknown;
  "id": string;
  "contracts_limit_fp"?: string | null;
  "is_auto_cancel_enabled": boolean;
  "exchange_index"?: number | bigint | string | null;
}

export interface PerpsPriceDistributionHistorical {
  [field: string]: unknown;
  "open": string | null;
  "low": string | null;
  "high": string | null;
  "close": string | null;
  "mean": string | null;
  "previous": string | null;
}

export interface PerpsSetCrossExitTriggerRequest {
  [field: string]: unknown;
  "count"?: string | null;
  "anchor_order_id"?: string | null;
  "client_trigger_id"?: string | null;
  "kind"?: string | null;
  "stop_loss_price"?: string | null;
  "take_profit_price"?: string | null;
  "trail_amount"?: string | null;
  "trail_bps"?: number | bigint | string | null;
}

export interface PerpsSetIsolatedExitTriggerRequest {
  [field: string]: unknown;
  "kind"?: string | null;
  "stop_loss_price"?: string | null;
  "take_profit_price"?: string | null;
  "trail_amount"?: string | null;
  "trail_bps"?: number | bigint | string | null;
}

export interface PerpsTickerPrice {
  [field: string]: unknown;
  "price": string;
  "ts_ms": number | bigint | string;
}

export interface PerpsUpdateExitTriggerRequest {
  [field: string]: unknown;
  "stop_loss_price"?: string | null;
  "take_profit_price"?: string | null;
}

export interface PerpsUpdateFCMNotionalRiskLimitRequest {
  [field: string]: unknown;
  "notional_value_risk_limit": string;
}

export interface PerpsUpdateFCMSubtraderRiskControlsRequest {
  [field: string]: unknown;
  "subtrader_id": string;
  "market_ticker"?: string | null;
  "asset_class"?: string | null;
  "im_cap": string;
}

export interface PerpsUpdateOrderGroupLimitRequest {
  [field: string]: unknown;
  "contracts_limit"?: number | bigint | string | null;
  "contracts_limit_fp"?: string | null;
}

export interface ResetMarginOrderGroupParams {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount"?: number | bigint | string | null;
  "body"?: PerpsEmptyResponse | null;
}

export interface ResetMarginOrderGroupResponse {
  [field: string]: unknown;
}

export interface SetCrossMarginExitTriggerParams {
  [field: string]: unknown;
  "ticker": string;
  "subaccount"?: number | bigint | string | null;
  "body": PerpsSetCrossExitTriggerRequest;
}

export interface SetCrossMarginExitTriggerResponse {
  [field: string]: unknown;
  "id": string;
  "ticker": string;
  "kind": string;
  "status": string;
  "status_reason"?: string | null;
  "triggered_leg"?: string | null;
  "triggered_order_id"?: string | null;
  "anchor_order_id"?: string | null;
  "client_trigger_id"?: string | null;
  "stop_loss_price"?: string | null;
  "take_profit_price"?: string | null;
  "trail_amount"?: string | null;
  "trail_bps"?: number | bigint | string | null;
  "watermark_price"?: string | null;
  "effective_stop_price"?: string | null;
  "count": string;
  "filled_count": string;
  "created_time": string;
  "updated_time": string;
}

export interface SetIsolatedMarginExitTriggerParams {
  [field: string]: unknown;
  "ticker": string;
  "body": PerpsSetIsolatedExitTriggerRequest;
}

export interface SetIsolatedMarginExitTriggerResponse {
  [field: string]: unknown;
  "id": string;
  "ticker": string;
  "kind": string;
  "status": string;
  "status_reason"?: string | null;
  "triggered_leg"?: string | null;
  "triggered_order_id"?: string | null;
  "anchor_order_id"?: string | null;
  "client_trigger_id"?: string | null;
  "stop_loss_price"?: string | null;
  "take_profit_price"?: string | null;
  "trail_amount"?: string | null;
  "trail_bps"?: number | bigint | string | null;
  "watermark_price"?: string | null;
  "effective_stop_price"?: string | null;
  "count": string;
  "filled_count": string;
  "created_time": string;
  "updated_time": string;
}

export interface TriggerMarginOrderGroupParams {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount"?: number | bigint | string | null;
  "body"?: PerpsEmptyResponse | null;
}

export interface TriggerMarginOrderGroupResponse {
  [field: string]: unknown;
}

export interface UpdateCrossMarginExitTriggerParams {
  [field: string]: unknown;
  "ticker": string;
  "trigger_id": string;
  "subaccount"?: number | bigint | string | null;
  "body": PerpsUpdateExitTriggerRequest;
}

export interface UpdateCrossMarginExitTriggerResponse {
  [field: string]: unknown;
  "id": string;
  "ticker": string;
  "kind": string;
  "status": string;
  "status_reason"?: string | null;
  "triggered_leg"?: string | null;
  "triggered_order_id"?: string | null;
  "anchor_order_id"?: string | null;
  "client_trigger_id"?: string | null;
  "stop_loss_price"?: string | null;
  "take_profit_price"?: string | null;
  "trail_amount"?: string | null;
  "trail_bps"?: number | bigint | string | null;
  "watermark_price"?: string | null;
  "effective_stop_price"?: string | null;
  "count": string;
  "filled_count": string;
  "created_time": string;
  "updated_time": string;
}

export interface UpdateFcmNotionalRiskLimitParams {
  [field: string]: unknown;
  "body": PerpsUpdateFCMNotionalRiskLimitRequest;
}

export interface UpdateFcmNotionalRiskLimitResponse {
  [field: string]: unknown;
}

export interface UpdateFcmSubtraderRiskControlsParams {
  [field: string]: unknown;
  "body": PerpsUpdateFCMSubtraderRiskControlsRequest;
}

export interface UpdateFcmSubtraderRiskControlsResponse {
  [field: string]: unknown;
}

export interface UpdateMarginOrderGroupLimitParams {
  [field: string]: unknown;
  "order_group_id": string;
  "subaccount"?: number | bigint | string | null;
  "body": PerpsUpdateOrderGroupLimitRequest;
}

export interface UpdateMarginOrderGroupLimitResponse {
  [field: string]: unknown;
}
