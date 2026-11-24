#![cfg(feature = "kalshi-perps-models")]
// Generated from schema/kalshi_perps.json.
#[rustfmt::skip]
mod fixtures {
    use aux_zig::kalshi_perps_models::*;
    use serde::{de::DeserializeOwned, Serialize};
    use serde_json::Value;

    fn wire_eq(actual: &Value, wire: &Value) {
        match (actual, wire) {
            (Value::Object(a),Value::Object(b)) => {
                for (k,v) in b { if v.is_null() && !a.contains_key(k) { continue; } wire_eq(&a[k],v); }
                assert!(a.keys().all(|k| b.contains_key(k)));
            },
            (Value::Array(a),Value::Array(b)) => { assert_eq!(a.len(),b.len()); for (x,y) in a.iter().zip(b) { wire_eq(x,y); } },
            _ => assert_eq!(actual,wire),
        }
    }
    fn check<T: DeserializeOwned + Serialize>(v: &Value) {
        let model: T = serde_json::from_value(v.clone()).unwrap();
        wire_eq(&serde_json::to_value(model).unwrap(), v);
    }

    #[test]
    fn models_round_trip_exposed_rest_fixtures() {
        let rows: Vec<Value> = serde_json::from_str(include_str!("../../../tests/prediction-http.json")).unwrap();
        for row in rows.iter().filter(|r| r["venue"] == "kalshi") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "clearing.clearing_create_margin_fcm_api_key" => { check::<ClearingCreateMarginFcmApiKeyParams>(request); check::<Box<ClearingCreateMarginFcmApiKeyResponse>>(response); },
                "clearing.clearing_create_margin_subtrader_group" => { check::<ClearingCreateMarginSubtraderGroupParams>(request); check::<Box<ClearingCreateMarginSubtraderGroupResponse>>(response); },
                "clearing.clearing_delete_margin_fcm_api_key" => { check::<ClearingDeleteMarginFcmApiKeyParams>(request); check::<serde_json::Value>(response); },
                "clearing.clearing_delete_margin_subtrader_group" => { check::<ClearingDeleteMarginSubtraderGroupParams>(request); check::<serde_json::Value>(response); },
                "clearing.clearing_estimate_portfolio_maintenance_margin" => { check::<ClearingEstimatePortfolioMaintenanceMarginParams>(request); check::<Box<ClearingEstimatePortfolioMaintenanceMarginResponse>>(response); },
                "clearing.clearing_generate_margin_fcm_api_key" => { check::<ClearingGenerateMarginFcmApiKeyParams>(request); check::<Box<ClearingGenerateMarginFcmApiKeyResponse>>(response); },
                "clearing.clearing_get_active_margin_obligations" => { check::<ClearingGetActiveMarginObligationsParams>(request); check::<Box<ClearingGetActiveMarginObligationsResponse>>(response); },
                "clearing.clearing_get_funding_estimate_by_asset_class" => { check::<ClearingGetFundingEstimateByAssetClassParams>(request); check::<Box<ClearingGetFundingEstimateByAssetClassResponse>>(response); },
                "clearing.clearing_get_guaranty_fund_balance" => { check::<ClearingGetGuarantyFundBalanceParams>(request); check::<Box<ClearingGetGuarantyFundBalanceResponse>>(response); },
                "clearing.clearing_get_large_trader_positions" => { check::<ClearingGetLargeTraderPositionsParams>(request); check::<Box<ClearingGetLargeTraderPositionsResponse>>(response); },
                "clearing.clearing_get_maintenance_margin_metadata" => { check::<ClearingGetMaintenanceMarginMetadataParams>(request); check::<Box<ClearingGetMaintenanceMarginMetadataResponse>>(response); },
                "clearing.clearing_get_margin_funding_schedule" => { check::<ClearingGetMarginFundingScheduleParams>(request); check::<Box<ClearingGetMarginFundingScheduleResponse>>(response); },
                "clearing.clearing_get_margin_reports" => { check::<ClearingGetMarginReportsParams>(request); check::<Box<ClearingGetMarginReportsResponse>>(response); },
                "clearing.clearing_get_margin_subtrader_groups" => { check::<ClearingGetMarginSubtraderGroupsParams>(request); check::<Box<ClearingGetMarginSubtraderGroupsResponse>>(response); },
                "clearing.clearing_get_member_funding_payments" => { check::<ClearingGetMemberFundingPaymentsParams>(request); check::<Box<ClearingGetMemberFundingPaymentsResponse>>(response); },
                "clearing.clearing_get_obligation_corporate_action_payments" => { check::<ClearingGetObligationCorporateActionPaymentsParams>(request); check::<Box<ClearingGetObligationCorporateActionPaymentsResponse>>(response); },
                "clearing.clearing_get_obligation_funding_payments" => { check::<ClearingGetObligationFundingPaymentsParams>(request); check::<Box<ClearingGetObligationFundingPaymentsResponse>>(response); },
                "clearing.clearing_get_obligation_history" => { check::<ClearingGetObligationHistoryParams>(request); check::<Box<ClearingGetObligationHistoryResponse>>(response); },
                "clearing.clearing_get_obligation_maintenance_margin_details" => { check::<ClearingGetObligationMaintenanceMarginDetailsParams>(request); check::<Box<ClearingGetObligationMaintenanceMarginDetailsResponse>>(response); },
                "clearing.clearing_get_obligation_settlement_details" => { check::<ClearingGetObligationSettlementDetailsParams>(request); check::<Box<ClearingGetObligationSettlementDetailsResponse>>(response); },
                "clearing.clearing_get_settlement_balance" => { check::<ClearingGetSettlementBalanceParams>(request); check::<Box<ClearingGetSettlementBalanceResponse>>(response); },
                "clearing.clearing_get_settlement_balance_history" => { check::<ClearingGetSettlementBalanceHistoryParams>(request); check::<Box<ClearingGetSettlementBalanceHistoryResponse>>(response); },
                "clearing.clearing_get_settlement_balance_withdrawal" => { check::<ClearingGetSettlementBalanceWithdrawalParams>(request); check::<Box<ClearingGetSettlementBalanceWithdrawalResponse>>(response); },
                "clearing.clearing_get_settlement_estimate_by_asset_class" => { check::<ClearingGetSettlementEstimateByAssetClassParams>(request); check::<Box<ClearingGetSettlementEstimateByAssetClassResponse>>(response); },
                "clearing.clearing_get_settlement_prices" => { check::<ClearingGetSettlementPricesParams>(request); check::<Box<ClearingGetSettlementPricesResponse>>(response); },
                "clearing.clearing_list_margin_fcm_api_keys" => { check::<ClearingListMarginFcmApiKeysParams>(request); check::<Box<ClearingListMarginFcmApiKeysResponse>>(response); },
                "clearing.clearing_update_margin_subtrader_group" => { check::<ClearingUpdateMarginSubtraderGroupParams>(request); check::<serde_json::Value>(response); },
                "clearing.clearing_withdraw_settlement_balance" => { check::<ClearingWithdrawSettlementBalanceParams>(request); check::<Box<ClearingWithdrawSettlementBalanceResponse>>(response); },
                "perps.amend_margin_order" => { check::<AmendMarginOrderParams>(request); check::<Box<PerpsAmendMarginOrderResponse>>(response); },
                "perps.apply_margin_subaccount_transfer" => { check::<ApplyMarginSubaccountTransferParams>(request); check::<Box<PerpsApplySubaccountTransferResponse>>(response); },
                "perps.cancel_all_margin_orders" => { check::<CancelAllMarginOrdersParams>(request); check::<serde_json::Value>(response); },
                "perps.cancel_margin_order" => { check::<CancelMarginOrderParams>(request); check::<Box<PerpsCancelMarginOrderResponse>>(response); },
                "perps.create_margin_fcm_subtrader" => { check::<CreateMarginFcmSubtraderParams>(request); check::<Box<PerpsCreateMarginFCMSubtraderResponse>>(response); },
                "perps.create_margin_order" => { check::<CreateMarginOrderParams>(request); check::<Box<PerpsCreateMarginOrderResponse>>(response); },
                "perps.create_margin_order_group" => { check::<CreateMarginOrderGroupParams>(request); check::<Box<PerpsCreateOrderGroupResponse>>(response); },
                "perps.create_margin_subaccount" => { check::<CreateMarginSubaccountParams>(request); check::<Box<PerpsCreateSubaccountResponse>>(response); },
                "perps.decrease_margin_order" => { check::<DecreaseMarginOrderParams>(request); check::<Box<PerpsDecreaseMarginOrderResponse>>(response); },
                "perps.delete_cross_margin_exit_trigger" => { check::<DeleteCrossMarginExitTriggerParams>(request); check::<serde_json::Value>(response); },
                "perps.delete_cross_margin_exit_trigger_by_id" => { check::<DeleteCrossMarginExitTriggerByIdParams>(request); check::<serde_json::Value>(response); },
                "perps.delete_fcm_notional_risk_limit" => { check::<DeleteFcmNotionalRiskLimitParams>(request); check::<Box<PerpsEmptyResponse>>(response); },
                "perps.delete_fcm_subtrader_risk_controls" => { check::<DeleteFcmSubtraderRiskControlsParams>(request); check::<Box<PerpsEmptyResponse>>(response); },
                "perps.delete_isolated_margin_exit_trigger" => { check::<DeleteIsolatedMarginExitTriggerParams>(request); check::<serde_json::Value>(response); },
                "perps.delete_margin_order_group" => { check::<DeleteMarginOrderGroupParams>(request); check::<Box<PerpsEmptyResponse>>(response); },
                "perps.get_cross_margin_exit_triggers" => { check::<GetCrossMarginExitTriggersParams>(request); check::<Box<PerpsGetExitTriggersResponse>>(response); },
                "perps.get_fcm_subtrader_risk_controls" => { check::<GetFcmSubtraderRiskControlsParams>(request); check::<Box<PerpsGetFCMSubtraderRiskControlsResponse>>(response); },
                "perps.get_isolated_margin_exit_triggers" => { check::<GetIsolatedMarginExitTriggersParams>(request); check::<Box<PerpsGetExitTriggersResponse>>(response); },
                "perps.get_margin_balance" => { check::<GetMarginBalanceParams>(request); check::<Box<PerpsGetMarginBalanceResponse>>(response); },
                "perps.get_margin_corporate_actions" => { check::<GetMarginCorporateActionsParams>(request); check::<Box<PerpsGetMarginCorporateActionsResponse>>(response); },
                "perps.get_margin_enabled" => { check::<GetMarginEnabledParams>(request); check::<Box<PerpsMarginEnabledResponse>>(response); },
                "perps.get_margin_exchange_status" => { check::<GetMarginExchangeStatusParams>(request); check::<Box<PerpsExchangeStatus>>(response); },
                "perps.get_margin_fee_tier_rates" => { check::<GetMarginFeeTierRatesParams>(request); check::<Box<PerpsGetMarginFeeTierRatesResponse>>(response); },
                "perps.get_margin_fee_tiers" => { check::<GetMarginFeeTiersParams>(request); check::<Box<PerpsGetMarginFeeTiersResponse>>(response); },
                "perps.get_margin_fills" => { check::<GetMarginFillsParams>(request); check::<Box<PerpsGetMarginFillsResponse>>(response); },
                "perps.get_margin_funding_history" => { check::<GetMarginFundingHistoryParams>(request); check::<Box<PerpsGetMarginFundingHistoryResponse>>(response); },
                "perps.get_margin_funding_rate_estimate" => { check::<GetMarginFundingRateEstimateParams>(request); check::<Box<PerpsGetMarginFundingRateEstimateResponse>>(response); },
                "perps.get_margin_historical_funding_rates" => { check::<GetMarginHistoricalFundingRatesParams>(request); check::<Box<PerpsGetMarginHistoricalFundingRatesResponse>>(response); },
                "perps.get_margin_market" => { check::<GetMarginMarketParams>(request); check::<Box<PerpsMarginMarketResponse>>(response); },
                "perps.get_margin_market_candlesticks" => { check::<GetMarginMarketCandlesticksParams>(request); check::<Box<PerpsGetMarginMarketCandlesticksResponse>>(response); },
                "perps.get_margin_market_orderbook" => { check::<GetMarginMarketOrderbookParams>(request); check::<Box<PerpsMarginOrderbookResponse>>(response); },
                "perps.get_margin_market_orderbooks" => { check::<GetMarginMarketOrderbooksParams>(request); check::<Box<PerpsGetMarginMarketOrderbooksResponse>>(response); },
                "perps.get_margin_market_schedules" => { check::<GetMarginMarketSchedulesParams>(request); check::<Box<PerpsGetMarginMarketSchedulesResponse>>(response); },
                "perps.get_margin_markets" => { check::<GetMarginMarketsParams>(request); check::<Box<PerpsGetMarginMarketsResponse>>(response); },
                "perps.get_margin_notional_risk_limit" => { check::<GetMarginNotionalRiskLimitParams>(request); check::<Box<PerpsNotionalRiskLimitResponse>>(response); },
                "perps.get_margin_order" => { check::<GetMarginOrderParams>(request); check::<Box<PerpsGetMarginOrderResponse>>(response); },
                "perps.get_margin_order_group" => { check::<GetMarginOrderGroupParams>(request); check::<Box<PerpsGetOrderGroupResponse>>(response); },
                "perps.get_margin_order_groups" => { check::<GetMarginOrderGroupsParams>(request); check::<Box<PerpsGetOrderGroupsResponse>>(response); },
                "perps.get_margin_orders" => { check::<GetMarginOrdersParams>(request); check::<Box<PerpsGetMarginOrdersResponse>>(response); },
                "perps.get_margin_positions" => { check::<GetMarginPositionsParams>(request); check::<Box<PerpsGetMarginPositionsResponse>>(response); },
                "perps.get_margin_premium_index" => { check::<GetMarginPremiumIndexParams>(request); check::<Box<PerpsGetMarginPremiumIndexResponse>>(response); },
                "perps.get_margin_risk" => { check::<GetMarginRiskParams>(request); check::<Box<PerpsGetMarginRiskResponse>>(response); },
                "perps.get_margin_risk_parameters" => { check::<GetMarginRiskParametersParams>(request); check::<Box<PerpsGetMarginRiskParametersResponse>>(response); },
                "perps.get_margin_trades" => { check::<GetMarginTradesParams>(request); check::<Box<PerpsGetMarginTradesResponse>>(response); },
                "perps.get_perps_account_api_limits" => { check::<GetPerpsAccountApiLimitsParams>(request); check::<Box<PerpsGetAccountApiLimitsResponse>>(response); },
                "perps.intra_exchange_instance_transfer" => { check::<IntraExchangeInstanceTransferParams>(request); check::<Box<PerpsIntraExchangeInstanceTransferResponse>>(response); },
                "perps.reset_margin_order_group" => { check::<ResetMarginOrderGroupParams>(request); check::<Box<PerpsEmptyResponse>>(response); },
                "perps.set_cross_margin_exit_trigger" => { check::<SetCrossMarginExitTriggerParams>(request); check::<Box<PerpsExitTrigger>>(response); },
                "perps.set_isolated_margin_exit_trigger" => { check::<SetIsolatedMarginExitTriggerParams>(request); check::<Box<PerpsExitTrigger>>(response); },
                "perps.trigger_margin_order_group" => { check::<TriggerMarginOrderGroupParams>(request); check::<Box<PerpsEmptyResponse>>(response); },
                "perps.update_cross_margin_exit_trigger" => { check::<UpdateCrossMarginExitTriggerParams>(request); check::<Box<PerpsExitTrigger>>(response); },
                "perps.update_fcm_notional_risk_limit" => { check::<UpdateFcmNotionalRiskLimitParams>(request); check::<Box<PerpsEmptyResponse>>(response); },
                "perps.update_fcm_subtrader_risk_controls" => { check::<UpdateFcmSubtraderRiskControlsParams>(request); check::<Box<PerpsEmptyResponse>>(response); },
                "perps.update_margin_order_group_limit" => { check::<UpdateMarginOrderGroupLimitParams>(request); check::<Box<PerpsEmptyResponse>>(response); },
                _ => continue,
            }
        }
    }
}
