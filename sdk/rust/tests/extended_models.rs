#![cfg(feature = "extended-models")]
// Generated from src/exchanges/extended/schema/extended.json.
#[rustfmt::skip]
mod fixtures {
    use aux_rs::extended_models::*;
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
        let rows: Vec<Value> = serde_json::from_str(include_str!("../../../tests/http.json")).unwrap();
        for row in rows.iter().filter(|r| r["venue"] == "extended") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "asset_operations" => { check::<AssetOperationsParams>(request); check::<AssetOperationsResult>(response); },
                "cancel_order" => { check::<CancelOrderParams>(request); check::<serde_json::Value>(response); },
                "cancel_order_by_external_id" => { check::<CancelOrderByExternalIdParams>(request); check::<CancelOrderByExternalIdResult>(response); },
                "claim_testing_funds" => { check::<ClaimTestingFundsParams>(request); check::<ClaimTestingFundsResult>(response); },
                "commit_bridge_quote" => { check::<CommitBridgeQuoteParams>(request); check::<CommitBridgeQuoteResult>(response); },
                "create_api_key" => { check::<CreateApiKeyParams>(request); check::<CreateApiKeyResult>(response); },
                "create_deposit" => { check::<CreateDepositParams>(request); check::<ApiResponse>(response); },
                "create_referral_link" => { check::<CreateReferralLinkParams>(request); check::<ApiResponse>(response); },
                "create_scaled_order" => { check::<CreateScaledOrderParams>(request); check::<serde_json::Value>(response); },
                "delete_api_key" => { check::<DeleteApiKeyParams>(request); check::<ApiResponse>(response); },
                "delete_user_order_order_id" => { check::<DeleteUserOrderOrderIdParams>(request); check::<DeleteUserOrderOrderIdResult>(response); },
                "deposit_to_vault" => { check::<DepositToVaultParams>(request); check::<DepositToVaultResult>(response); },
                "export_csv_zip" => { check::<ExportCsvZipParams>(request); check::<serde_json::Value>(response); },
                "get_account" => { check::<GetAccountParams>(request); check::<GetAccountResult>(response); },
                "get_account_equity_history" => { check::<GetAccountEquityHistoryParams>(request); check::<GetAccountEquityHistoryResult>(response); },
                "get_account_funding_chart" => { check::<GetAccountFundingChartParams>(request); check::<GetAccountFundingChartResult>(response); },
                "get_account_funding_history" => { check::<GetAccountFundingHistoryParams>(request); check::<GetAccountFundingHistoryResult>(response); },
                "get_account_funding_stats" => { check::<GetAccountFundingStatsParams>(request); check::<GetAccountFundingStatsResult>(response); },
                "get_account_health" => { check::<GetAccountHealthParams>(request); check::<GetAccountHealthResult>(response); },
                "get_account_max_drawdown_history" => { check::<GetAccountMaxDrawdownHistoryParams>(request); check::<GetAccountMaxDrawdownHistoryResult>(response); },
                "get_account_performance" => { check::<GetAccountPerformanceParams>(request); check::<GetAccountPerformanceResult>(response); },
                "get_account_pn_l_history" => { check::<GetAccountPnLHistoryParams>(request); check::<GetAccountPnLHistoryResult>(response); },
                "get_account_pn_l_percentage_history" => { check::<GetAccountPnLPercentageHistoryParams>(request); check::<GetAccountPnLPercentageHistoryResult>(response); },
                "get_account_portfolio_summary" => { check::<GetAccountPortfolioSummaryParams>(request); check::<GetAccountPortfolioSummaryResult>(response); },
                "get_account_vault_equity_history" => { check::<GetAccountVaultEquityHistoryParams>(request); check::<GetAccountVaultEquityHistoryResult>(response); },
                "get_accounts" => { check::<GetAccountsParams>(request); check::<GetAccountsResult>(response); },
                "get_affiliate" => { check::<GetAffiliateParams>(request); check::<ApiResponse>(response); },
                "get_affiliate_rewards" => { check::<GetAffiliateRewardsParams>(request); check::<ApiResponse>(response); },
                "get_api_keys" => { check::<GetApiKeysParams>(request); check::<serde_json::Value>(response); },
                "get_asset_operations" => { check::<GetAssetOperationsParams>(request); check::<serde_json::Value>(response); },
                "get_asset_price" => { check::<GetAssetPriceParams>(request); check::<serde_json::Value>(response); },
                "get_assets" => { check::<GetAssetsParams>(request); check::<GetAssetsResult>(response); },
                "get_balance" => { check::<GetBalanceParams>(request); check::<GetBalanceResult>(response); },
                "get_bridge_config" => { check::<GetBridgeConfigParams>(request); check::<GetBridgeConfigResult>(response); },
                "get_bridge_quote" => { check::<GetBridgeQuoteParams>(request); check::<GetBridgeQuoteResult>(response); },
                "get_builder_dashboard" => { check::<GetBuilderDashboardParams>(request); check::<serde_json::Value>(response); },
                "get_builder_trades" => { check::<GetBuilderTradesParams>(request); check::<GetBuilderTradesResult>(response); },
                "get_candles_history" => { check::<GetCandlesHistoryParams>(request); check::<GetCandlesHistoryResult>(response); },
                "get_claim" => { check::<GetClaimParams>(request); check::<serde_json::Value>(response); },
                "get_claims" => { check::<GetClaimsParams>(request); check::<serde_json::Value>(response); },
                "get_client" => { check::<GetClientParams>(request); check::<GetClientResult>(response); },
                "get_cumulative_account_pn_l_history" => { check::<GetCumulativeAccountPnLHistoryParams>(request); check::<GetCumulativeAccountPnLHistoryResult>(response); },
                "get_cumulative_account_pn_l_percentage_history" => { check::<GetCumulativeAccountPnLPercentageHistoryParams>(request); check::<GetCumulativeAccountPnLPercentageHistoryResult>(response); },
                "get_earned_rewards" => { check::<GetEarnedRewardsParams>(request); check::<ApiResponse>(response); },
                "get_exchange_status" => { check::<GetExchangeStatusParams>(request); check::<serde_json::Value>(response); },
                "get_fees" => { check::<GetFeesParams>(request); check::<GetFeesResult>(response); },
                "get_funding_rates_history" => { check::<GetFundingRatesHistoryParams>(request); check::<GetFundingRatesHistoryResult>(response); },
                "get_index_price_candles" => { check::<GetIndexPriceCandlesParams>(request); check::<serde_json::Value>(response); },
                "get_info_assets_asset_name_price" => { check::<GetInfoAssetsAssetNamePriceParams>(request); check::<GetInfoAssetsAssetNamePriceResult>(response); },
                "get_info_candles_market_candle_type" => { check::<GetInfoCandlesMarketCandleTypeParams>(request); check::<GetInfoCandlesMarketCandleTypeResult>(response); },
                "get_interest_daily_metrics" => { check::<GetInterestDailyMetricsParams>(request); check::<GetInterestDailyMetricsResult>(response); },
                "get_interest_key_metrics" => { check::<GetInterestKeyMetricsParams>(request); check::<GetInterestKeyMetricsResult>(response); },
                "get_interest_payment_chart" => { check::<GetInterestPaymentChartParams>(request); check::<GetInterestPaymentChartResult>(response); },
                "get_interest_payments_history" => { check::<GetInterestPaymentsHistoryParams>(request); check::<GetInterestPaymentsHistoryResult>(response); },
                "get_interest_rate_curves_history" => { check::<GetInterestRateCurvesHistoryParams>(request); check::<GetInterestRateCurvesHistoryResult>(response); },
                "get_latest_interest_rate_curve" => { check::<GetLatestInterestRateCurveParams>(request); check::<GetLatestInterestRateCurveResult>(response); },
                "get_leverage" => { check::<GetLeverageParams>(request); check::<GetLeverageResult>(response); },
                "get_mark_price_candles" => { check::<GetMarkPriceCandlesParams>(request); check::<serde_json::Value>(response); },
                "get_market_statistics" => { check::<GetMarketStatisticsParams>(request); check::<GetMarketStatisticsResult>(response); },
                "get_market_trades" => { check::<GetMarketTradesParams>(request); check::<serde_json::Value>(response); },
                "get_markets" => { check::<GetMarketsParams>(request); check::<GetMarketsResult>(response); },
                "get_markets_stats" => { check::<GetMarketsStatsParams>(request); check::<serde_json::Value>(response); },
                "get_next_claim_time" => { check::<GetNextClaimTimeParams>(request); check::<serde_json::Value>(response); },
                "get_open_interests" => { check::<GetOpenInterestsParams>(request); check::<serde_json::Value>(response); },
                "get_open_orders" => { check::<GetOpenOrdersParams>(request); check::<GetOpenOrdersResult>(response); },
                "get_order" => { check::<GetOrderParams>(request); check::<serde_json::Value>(response); },
                "get_order_by_external_id" => { check::<GetOrderByExternalIdParams>(request); check::<serde_json::Value>(response); },
                "get_order_by_id" => { check::<GetOrderByIdParams>(request); check::<GetOrderByIdResult>(response); },
                "get_orderbook_snapshot" => { check::<GetOrderbookSnapshotParams>(request); check::<GetOrderbookSnapshotResult>(response); },
                "get_orders_history" => { check::<GetOrdersHistoryParams>(request); check::<GetOrdersHistoryResult>(response); },
                "get_partner_rewards" => { check::<GetPartnerRewardsParams>(request); check::<ApiResponse>(response); },
                "get_positions" => { check::<GetPositionsParams>(request); check::<GetPositionsResult>(response); },
                "get_positions_history" => { check::<GetPositionsHistoryParams>(request); check::<GetPositionsHistoryResult>(response); },
                "get_public_leaderboard" => { check::<GetPublicLeaderboardParams>(request); check::<serde_json::Value>(response); },
                "get_public_referral_link" => { check::<GetPublicReferralLinkParams>(request); check::<ApiResponse>(response); },
                "get_rebate_stats" => { check::<GetRebateStatsParams>(request); check::<ApiResponse>(response); },
                "get_rebates_csv" => { check::<GetRebatesCsvParams>(request); check::<serde_json::Value>(response); },
                "get_referral_dashboard" => { check::<GetReferralDashboardParams>(request); check::<ApiResponse>(response); },
                "get_referral_links" => { check::<GetReferralLinksParams>(request); check::<ApiResponse>(response); },
                "get_referral_rebates_by_link_csv" => { check::<GetReferralRebatesByLinkCsvParams>(request); check::<serde_json::Value>(response); },
                "get_referral_rebates_by_period_csv" => { check::<GetReferralRebatesByPeriodCsvParams>(request); check::<serde_json::Value>(response); },
                "get_referral_rebates_by_subaffiliate_csv" => { check::<GetReferralRebatesBySubaffiliateCsvParams>(request); check::<serde_json::Value>(response); },
                "get_referral_rebates_by_user_csv" => { check::<GetReferralRebatesByUserCsvParams>(request); check::<serde_json::Value>(response); },
                "get_referral_status" => { check::<GetReferralStatusParams>(request); check::<ApiResponse>(response); },
                "get_reward_leaderboard_stats" => { check::<GetRewardLeaderboardStatsParams>(request); check::<ApiResponse>(response); },
                "get_settings" => { check::<GetSettingsParams>(request); check::<GetSettingsResult>(response); },
                "get_spot_balances" => { check::<GetSpotBalancesParams>(request); check::<GetSpotBalancesResult>(response); },
                "get_starknet_domain" => { check::<GetStarknetDomainParams>(request); check::<serde_json::Value>(response); },
                "get_trades" => { check::<GetTradesParams>(request); check::<GetTradesResult>(response); },
                "get_trading_candles" => { check::<GetTradingCandlesParams>(request); check::<serde_json::Value>(response); },
                "get_trading_fee_stats" => { check::<GetTradingFeeStatsParams>(request); check::<serde_json::Value>(response); },
                "get_user_funding_history" => { check::<GetUserFundingHistoryParams>(request); check::<serde_json::Value>(response); },
                "get_user_leaderboard" => { check::<GetUserLeaderboardParams>(request); check::<ApiResponse>(response); },
                "get_user_orders_external_external_id" => { check::<GetUserOrdersExternalExternalIdParams>(request); check::<GetUserOrdersExternalExternalIdResult>(response); },
                "get_vault_funding_history" => { check::<GetVaultFundingHistoryParams>(request); check::<serde_json::Value>(response); },
                "get_vault_orders" => { check::<GetVaultOrdersParams>(request); check::<serde_json::Value>(response); },
                "get_vault_performance" => { check::<GetVaultPerformanceParams>(request); check::<GetVaultPerformanceResult>(response); },
                "get_vault_position_history" => { check::<GetVaultPositionHistoryParams>(request); check::<serde_json::Value>(response); },
                "get_vault_positions" => { check::<GetVaultPositionsParams>(request); check::<serde_json::Value>(response); },
                "get_vault_summary" => { check::<GetVaultSummaryParams>(request); check::<GetVaultSummaryResult>(response); },
                "get_vault_trades" => { check::<GetVaultTradesParams>(request); check::<serde_json::Value>(response); },
                "get_wallet_validation" => { check::<GetWalletValidationParams>(request); check::<serde_json::Value>(response); },
                "get_withdrawals" => { check::<GetWithdrawalsParams>(request); check::<serde_json::Value>(response); },
                "get_wrapper" => { check::<GetWrapperParams>(request); check::<ApiResponse>(response); },
                "mark_wallet_checked" => { check::<MarkWalletCheckedParams>(request); check::<serde_json::Value>(response); },
                "mass_cancel" => { check::<MassCancelParams>(request); check::<MassCancelResult>(response); },
                "onboard_client" => { check::<OnboardClientParams>(request); check::<OnboardClientResult>(response); },
                "onboard_subaccount" => { check::<OnboardSubaccountParams>(request); check::<OnboardSubaccountResult>(response); },
                "onchain_withdrawal" => { check::<OnchainWithdrawalParams>(request); check::<ApiResponse>(response); },
                "place_order" => { check::<PlaceOrderParams>(request); check::<PlaceOrderResult>(response); },
                "place_rfq_order" => { check::<PlaceRfqOrderParams>(request); check::<PlaceRfqOrderResult>(response); },
                "post_user_transfer_onchain" => { check::<PostUserTransferOnchainParams>(request); check::<PostUserTransferOnchainResult>(response); },
                "set_deadman_switch" => { check::<SetDeadmanSwitchParams>(request); check::<serde_json::Value>(response); },
                "transfer" => { check::<TransferParams>(request); check::<ApiResponse>(response); },
                "transfer_patch" => { check::<TransferPatchParams>(request); check::<ApiResponse>(response); },
                "update_account_info" => { check::<UpdateAccountInfoParams>(request); check::<ApiResponse>(response); },
                "update_funding_rate_config" => { check::<UpdateFundingRateConfigParams>(request); check::<ApiResponse>(response); },
                "update_leverage" => { check::<UpdateLeverageParams>(request); check::<UpdateLeverageResult>(response); },
                "update_referral_link" => { check::<UpdateReferralLinkParams>(request); check::<ApiResponse>(response); },
                "use_referral_code" => { check::<UseReferralCodeParams>(request); check::<ApiResponse>(response); },
                "validate_wallet" => { check::<ValidateWalletParams>(request); check::<serde_json::Value>(response); },
                "withdraw" => { check::<WithdrawParams>(request); check::<WithdrawResult>(response); },
                "withdraw_from_vault" => { check::<WithdrawFromVaultParams>(request); check::<WithdrawFromVaultResult>(response); },
                _ => continue,
            }
        }
    }
}
