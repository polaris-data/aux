#![cfg(feature = "polymarket-perps-models")]
// Generated from schema/polymarket_perps.json.
#[rustfmt::skip]
mod fixtures {
    use aux_zig::polymarket_perps_models::*;
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
        for row in rows.iter().filter(|r| r["venue"] == "polymarket") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "perps.apply_referral" => { check::<ApplyReferralParams>(request); check::<Box<PerpsGenericAccepted>>(response); },
                "perps.approve_builder" => { check::<ApproveBuilderParams>(request); check::<Box<PerpsBuilderApproval>>(response); },
                "perps.cancel_all_orders" => { check::<CancelAllOrdersParams>(request); check::<Box<PerpsGenericAccepted>>(response); },
                "perps.cancel_chase" => { check::<CancelChaseParams>(request); check::<serde_json::Value>(response); },
                "perps.cancel_orders" => { check::<CancelOrdersParams>(request); check::<Vec<PerpsCancelResponse>>(response); },
                "perps.cancel_orders_by_coid" => { check::<CancelOrdersByCoidParams>(request); check::<Vec<PerpsCancelResponse>>(response); },
                "perps.cancel_twap" => { check::<CancelTwapParams>(request); check::<serde_json::Value>(response); },
                "perps.check_invite_code" => { check::<CheckInviteCodeParams>(request); check::<Box<PerpsInviteCheckResponse>>(response); },
                "perps.control_twap" => { check::<ControlTwapParams>(request); check::<serde_json::Value>(response); },
                "perps.create_account_invite" => { check::<CreateAccountInviteParams>(request); check::<Box<PerpsCreateInviteResponse>>(response); },
                "perps.create_chase" => { check::<CreateChaseParams>(request); check::<serde_json::Value>(response); },
                "perps.create_orders" => { check::<CreateOrdersParams>(request); check::<Vec<PerpsOrderResponse>>(response); },
                "perps.create_proxy" => { check::<CreateProxyParams>(request); check::<Box<PerpsProxyResponse>>(response); },
                "perps.create_twap" => { check::<CreateTwapParams>(request); check::<serde_json::Value>(response); },
                "perps.delete_proxy" => { check::<DeleteProxyParams>(request); check::<Box<PerpsGenericAccepted>>(response); },
                "perps.get_account_backstops" => { check::<GetAccountBackstopsParams>(request); check::<Box<PerpsAccountBackstops>>(response); },
                "perps.get_account_funding" => { check::<GetAccountFundingParams>(request); check::<Box<PerpsAccountFundingHistory>>(response); },
                "perps.get_account_limits" => { check::<GetAccountLimitsParams>(request); check::<Box<PerpsAccountLimits>>(response); },
                "perps.get_account_notifications" => { check::<GetAccountNotificationsParams>(request); check::<Box<PerpsNotificationsResponse>>(response); },
                "perps.get_account_referral" => { check::<GetAccountReferralParams>(request); check::<Box<PerpsAccountReferral>>(response); },
                "perps.get_account_rewards" => { check::<GetAccountRewardsParams>(request); check::<Box<PerpsAccountRewards>>(response); },
                "perps.get_account_stats" => { check::<GetAccountStatsParams>(request); check::<Box<PerpsAccountStats>>(response); },
                "perps.get_assets" => { check::<GetAssetsParams>(request); check::<Vec<PerpsAsset>>(response); },
                "perps.get_auto_cancel" => { check::<GetAutoCancelParams>(request); check::<Box<PerpsAutoCancelStatus>>(response); },
                "perps.get_balances" => { check::<GetBalancesParams>(request); check::<Vec<PerpsBalance>>(response); },
                "perps.get_bbo" => { check::<GetBboParams>(request); check::<Vec<PerpsBBO>>(response); },
                "perps.get_blp_enrollment" => { check::<GetBlpEnrollmentParams>(request); check::<Box<PerpsBlpEnrollment>>(response); },
                "perps.get_blp_liquidations" => { check::<GetBlpLiquidationsParams>(request); check::<Box<PerpsBlpLiquidations>>(response); },
                "perps.get_book" => { check::<GetBookParams>(request); check::<Box<PerpsBook>>(response); },
                "perps.get_builder" => { check::<GetBuilderParams>(request); check::<GetBuilderResponse>(response); },
                "perps.get_builder_approvals" => { check::<GetBuilderApprovalsParams>(request); check::<Box<PerpsBuilderApprovals>>(response); },
                "perps.get_builder_earnings" => { check::<GetBuilderEarningsParams>(request); check::<Box<PerpsBuilderEarnings>>(response); },
                "perps.get_builder_earnings_summary" => { check::<GetBuilderEarningsSummaryParams>(request); check::<Box<PerpsBuilderEarningsSummary>>(response); },
                "perps.get_chases" => { check::<GetChasesParams>(request); check::<serde_json::Value>(response); },
                "perps.get_competition" => { check::<GetCompetitionParams>(request); check::<Box<PerpsCompetitionDetail>>(response); },
                "perps.get_competition_account" => { check::<GetCompetitionAccountParams>(request); check::<Box<PerpsCompetitionAccount>>(response); },
                "perps.get_competition_leaderboard" => { check::<GetCompetitionLeaderboardParams>(request); check::<Box<PerpsCompetitionLeaderboard>>(response); },
                "perps.get_competition_widget" => { check::<GetCompetitionWidgetParams>(request); check::<Box<PerpsCompetitionWidget>>(response); },
                "perps.get_competitions" => { check::<GetCompetitionsParams>(request); check::<Box<PerpsCompetitionList>>(response); },
                "perps.get_config" => { check::<GetConfigParams>(request); check::<Vec<PerpsAccountConfig>>(response); },
                "perps.get_credentials" => { check::<GetCredentialsParams>(request); check::<Box<PerpsCredentials>>(response); },
                "perps.get_deposits" => { check::<GetDepositsParams>(request); check::<Box<PerpsDeposits>>(response); },
                "perps.get_equity_history" => { check::<GetEquityHistoryParams>(request); check::<Box<PerpsEquityHistory>>(response); },
                "perps.get_exchange" => { check::<GetExchangeParams>(request); check::<Box<PerpsExchange>>(response); },
                "perps.get_exchange_statistics" => { check::<GetExchangeStatisticsParams>(request); check::<Box<PerpsExchangeStatistics>>(response); },
                "perps.get_fills" => { check::<GetFillsParams>(request); check::<Box<PerpsAccountTrades>>(response); },
                "perps.get_funding_history" => { check::<GetFundingHistoryParams>(request); check::<Box<PerpsFundingHistory>>(response); },
                "perps.get_index" => { check::<GetIndexParams>(request); check::<Box<PerpsIndex>>(response); },
                "perps.get_info_fees" => { check::<GetInfoFeesParams>(request); check::<Box<PerpsFeesInfo>>(response); },
                "perps.get_instruments" => { check::<GetInstrumentsParams>(request); check::<Vec<PerpsInstrument>>(response); },
                "perps.get_internal_transfers" => { check::<GetInternalTransfersParams>(request); check::<Box<PerpsInternalTransfers>>(response); },
                "perps.get_klines" => { check::<GetKlinesParams>(request); check::<Box<PerpsKlinesResponse>>(response); },
                "perps.get_leaderboard" => { check::<GetLeaderboardParams>(request); check::<Box<PerpsLeaderboard>>(response); },
                "perps.get_limit_tiers" => { check::<GetLimitTiersParams>(request); check::<Vec<PerpsLimitTier>>(response); },
                "perps.get_mark_history" => { check::<GetMarkHistoryParams>(request); check::<Box<PerpsMarkHistoryResponse>>(response); },
                "perps.get_open_orders" => { check::<GetOpenOrdersParams>(request); check::<Vec<PerpsOrderData>>(response); },
                "perps.get_orders" => { check::<GetOrdersParams>(request); check::<Vec<PerpsOrderData>>(response); },
                "perps.get_ping" => { check::<GetPingParams>(request); check::<GetPingResponse>(response); },
                "perps.get_pnl_history" => { check::<GetPnlHistoryParams>(request); check::<Box<PerpsPnlHistory>>(response); },
                "perps.get_portfolio" => { check::<GetPortfolioParams>(request); check::<Box<PerpsPortfolio>>(response); },
                "perps.get_public_portfolio" => { check::<GetPublicPortfolioParams>(request); check::<Box<PerpsPublicPortfolio>>(response); },
                "perps.get_public_position_fills" => { check::<GetPublicPositionFillsParams>(request); check::<Box<PerpsAccountTrades>>(response); },
                "perps.get_public_position_snapshots" => { check::<GetPublicPositionSnapshotsParams>(request); check::<Box<PerpsPositionSnapshots>>(response); },
                "perps.get_registered" => { check::<GetRegisteredParams>(request); check::<Box<PerpsRegisteredResponse>>(response); },
                "perps.get_statistics" => { check::<GetStatisticsParams>(request); check::<Vec<PerpsStatistic>>(response); },
                "perps.get_tickers" => { check::<GetTickersParams>(request); check::<Vec<PerpsTicker>>(response); },
                "perps.get_time" => { check::<GetTimeParams>(request); check::<Box<PerpsTime>>(response); },
                "perps.get_trades" => { check::<GetTradesParams>(request); check::<Box<PerpsTrades>>(response); },
                "perps.get_twaps" => { check::<GetTwapsParams>(request); check::<serde_json::Value>(response); },
                "perps.get_withdrawals" => { check::<GetWithdrawalsParams>(request); check::<Box<PerpsWithdrawals>>(response); },
                "perps.internal_transfer" => { check::<InternalTransferParams>(request); check::<Box<PerpsInternalTransferAccepted>>(response); },
                "perps.mark_account_notifications_read" => { check::<MarkAccountNotificationsReadParams>(request); check::<Box<PerpsGenericAccepted>>(response); },
                "perps.modify_orders" => { check::<ModifyOrdersParams>(request); check::<Vec<PerpsModifyResponse>>(response); },
                "perps.modify_orders_by_coid" => { check::<ModifyOrdersByCoidParams>(request); check::<Vec<PerpsModifyResponse>>(response); },
                "perps.post_blp_enroll" => { check::<PostBlpEnrollParams>(request); check::<Box<PerpsGenericAccepted>>(response); },
                "perps.set_auto_cancel" => { check::<SetAutoCancelParams>(request); check::<Box<PerpsAutoCancelResponse>>(response); },
                "perps.update_builder" => { check::<UpdateBuilderParams>(request); check::<UpdateBuilderResponse>(response); },
                "perps.update_leverage" => { check::<UpdateLeverageParams>(request); check::<Box<PerpsLeverageResponse>>(response); },
                "perps.update_leverages" => { check::<UpdateLeveragesParams>(request); check::<Vec<PerpsLeverageBatchResponse>>(response); },
                "perps.update_margin" => { check::<UpdateMarginParams>(request); check::<Box<PerpsGenericResponse>>(response); },
                "perps.withdraw" => { check::<WithdrawParams>(request); check::<Box<PerpsWithdrawAccepted>>(response); },
                _ => continue,
            }
        }
    }
}
