#![cfg(feature = "phoenix-models")]
// Generated from src/exchanges/phoenix/schema/phoenix.json.
#[rustfmt::skip]
mod fixtures {
    use aux_rs::phoenix_models::*;
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
        for row in rows.iter().filter(|r| r["venue"] == "phoenix") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "acknowledge_notifications" => { check::<AcknowledgeNotificationsParams>(request); check::<serde_json::Value>(response); },
                "acknowledge_notifications_up_to_timestamp" => { check::<AcknowledgeNotificationsUpToTimestampParams>(request); check::<serde_json::Value>(response); },
                "activate_referral" => { check::<ActivateReferralParams>(request); check::<ActivateInviteResponse>(response); },
                "activate_referral_with_user_funded_transaction" => { check::<ActivateReferralWithUserFundedTransactionParams>(request); check::<ActivateReferralTxResponse>(response); },
                "build_attached_conditional_order_transaction" => { check::<BuildAttachedConditionalOrderTransactionParams>(request); check::<Vec<ApiInstructionResponse>>(response); },
                "build_cancel_conditional_order_transaction" => { check::<BuildCancelConditionalOrderTransactionParams>(request); check::<Vec<ApiInstructionResponse>>(response); },
                "build_cancel_stop_loss_order_transaction" => { check::<BuildCancelStopLossOrderTransactionParams>(request); check::<Vec<ApiInstructionResponse>>(response); },
                "build_isolated_limit_order_transaction" => { check::<BuildIsolatedLimitOrderTransactionParams>(request); check::<Vec<ApiInstructionResponse>>(response); },
                "build_isolated_limit_order_transaction_with_conditionals" => { check::<BuildIsolatedLimitOrderTransactionWithConditionalsParams>(request); check::<Vec<ApiInstructionResponse>>(response); },
                "build_isolated_limit_order_transaction_with_liquidation_estimate" => { check::<BuildIsolatedLimitOrderTransactionWithLiquidationEstimateParams>(request); check::<PlaceIsolatedLimitOrderEnhancedResponse>(response); },
                "build_isolated_market_order_transaction" => { check::<BuildIsolatedMarketOrderTransactionParams>(request); check::<Vec<ApiInstructionResponse>>(response); },
                "build_isolated_market_order_transaction_with_liquidation_estimate" => { check::<BuildIsolatedMarketOrderTransactionWithLiquidationEstimateParams>(request); check::<PlaceIsolatedMarketOrderEnhancedResponse>(response); },
                "build_position_conditional_order_transaction" => { check::<BuildPositionConditionalOrderTransactionParams>(request); check::<Vec<ApiInstructionResponse>>(response); },
                "build_register_trader_instructions" => { check::<BuildRegisterTraderInstructionsParams>(request); check::<BuildRegisterIxsResponse>(response); },
                "build_stop_loss_order_transaction" => { check::<BuildStopLossOrderTransactionParams>(request); check::<Vec<ApiInstructionResponse>>(response); },
                "check_wallet_invite_status" => { check::<CheckWalletInviteStatusParams>(request); check::<CheckWalletResponse>(response); },
                "create_service_login_challenge" => { check::<CreateServiceLoginChallengeParams>(request); check::<ServiceChallengeResponse>(response); },
                "create_wallet_transaction_challenge" => { check::<CreateWalletTransactionChallengeParams>(request); check::<WalletTransactionChallengeResponse>(response); },
                "get_candles" => { check::<GetCandlesParams>(request); check::<Vec<ApiCandle>>(response); },
                "get_commodity_market_calendar" => { check::<GetCommodityMarketCalendarParams>(request); check::<CommodityMarketCalendarResponse>(response); },
                "get_exchange" => { check::<GetExchangeParams>(request); check::<ExchangeView>(response); },
                "get_exchange_keys" => { check::<GetExchangeKeysParams>(request); check::<ExchangeKeysView>(response); },
                "get_exchange_snapshot" => { check::<GetExchangeSnapshotParams>(request); check::<ExchangeSnapshotView>(response); },
                "get_exchange_status" => { check::<GetExchangeStatusParams>(request); check::<ExchangeStatusView>(response); },
                "get_funding_overview" => { check::<GetFundingOverviewParams>(request); check::<FundingOverviewResponse>(response); },
                "get_funding_rate_history" => { check::<GetFundingRateHistoryParams>(request); check::<FundingRateHistoryResponse>(response); },
                "get_jwks" => { check::<GetJwksParams>(request); check::<JwksResponse>(response); },
                "get_mark_price" => { check::<GetMarkPriceParams>(request); check::<MarkPriceResponse>(response); },
                "get_market" => { check::<GetMarketParams>(request); check::<ExchangeMarketConfig>(response); },
                "get_market_calendar" => { check::<GetMarketCalendarParams>(request); check::<MarketCalendarResponse>(response); },
                "get_market_calendar_by_id" => { check::<GetMarketCalendarByIdParams>(request); check::<MarketCalendarRecord>(response); },
                "get_market_fills" => { check::<GetMarketFillsParams>(request); check::<FillsResponse>(response); },
                "get_market_stats_history" => { check::<GetMarketStatsHistoryParams>(request); check::<MarketStatsHistoryResponse>(response); },
                "get_next_commodity_market_transition" => { check::<GetNextCommodityMarketTransitionParams>(request); check::<NextCommodityMarketTransition>(response); },
                "get_next_market_calendar_transition" => { check::<GetNextMarketCalendarTransitionParams>(request); check::<NextMarketCalendarTransition>(response); },
                "get_order_history" => { check::<GetOrderHistoryParams>(request); check::<PaginatedResponseVecOrderHistoryItem>(response); },
                "get_order_history_v2" => { check::<GetOrderHistoryV2Params>(request); check::<PaginatedResponseVecOrderHistoryV2Item>(response); },
                "get_orderbook" => { check::<GetOrderbookParams>(request); check::<OrderbookView>(response); },
                "get_referral_activation_permission" => { check::<GetReferralActivationPermissionParams>(request); check::<ReferralActivationPermissionResponse>(response); },
                "get_trade_history" => { check::<GetTradeHistoryParams>(request); check::<PaginatedResponseVecTradeHistoryV2Item>(response); },
                "get_trader" => { check::<GetTraderParams>(request); check::<TraderView>(response); },
                "get_trader_capabilities" => { check::<GetTraderCapabilitiesParams>(request); check::<TraderCapabilitiesMetadata>(response); },
                "get_trader_collateral_history" => { check::<GetTraderCollateralHistoryParams>(request); check::<CollateralHistoryResponse>(response); },
                "get_trader_funding_history" => { check::<GetTraderFundingHistoryParams>(request); check::<TraderFundingHistoryResponse>(response); },
                "get_trader_market_pn_l" => { check::<GetTraderMarketPnLParams>(request); check::<Vec<TraderMarketPnLSeries>>(response); },
                "get_trader_pn_l" => { check::<GetTraderPnLParams>(request); check::<Vec<PnLPoint>>(response); },
                "get_trader_portfolio_values" => { check::<GetTraderPortfolioValuesParams>(request); check::<Vec<PortfolioValuePoint>>(response); },
                "get_trader_state" => { check::<GetTraderStateParams>(request); check::<TraderStateSnapshotResponse>(response); },
                "get_trader_trade_history_v2" => { check::<GetTraderTradeHistoryV2Params>(request); check::<PaginatedResponseVecTradeHistoryV2Item>(response); },
                "get_user_collateral_history" => { check::<GetUserCollateralHistoryParams>(request); check::<CollateralHistoryResponse>(response); },
                "get_user_hourly_funding" => { check::<GetUserHourlyFundingParams>(request); check::<FundingHourlyHistoryResponse>(response); },
                "get_user_liquidation_history" => { check::<GetUserLiquidationHistoryParams>(request); check::<PaginatedResponseVecUserLiquidationHistoryPoint>(response); },
                "get_user_pn_l" => { check::<GetUserPnLParams>(request); check::<Vec<PnLPoint>>(response); },
                "get_user_trade_history_v2" => { check::<GetUserTradeHistoryV2Params>(request); check::<PaginatedResponseVecTradeHistoryV2Item>(response); },
                "get_v1_traders_by_trader_pubkey_collateral_history" => { check::<GetV1TradersByTraderPubkeyCollateralHistoryParams>(request); check::<CollateralHistoryResponse>(response); },
                "get_wallet_login_nonce" => { check::<GetWalletLoginNonceParams>(request); check::<WalletNonceResponse>(response); },
                "list_markets" => { check::<ListMarketsParams>(request); check::<Vec<ExchangeMarketConfig>>(response); },
                "list_trader_notifications" => { check::<ListTraderNotificationsParams>(request); check::<GetNotificationsResponse>(response); },
                "log_in_with_privy_token" => { check::<LogInWithPrivyTokenParams>(request); check::<AuthResponse>(response); },
                "log_in_with_service_signature" => { check::<LogInWithServiceSignatureParams>(request); check::<AuthResponse>(response); },
                "log_in_with_signed_wallet_transaction" => { check::<LogInWithSignedWalletTransactionParams>(request); check::<AuthResponse>(response); },
                "log_in_with_wallet_signature" => { check::<LogInWithWalletSignatureParams>(request); check::<AuthResponse>(response); },
                "log_out" => { check::<LogOutParams>(request); check::<serde_json::Value>(response); },
                "refresh_session" => { check::<RefreshSessionParams>(request); check::<AuthResponse>(response); },
                "submit_register_trader_transaction" => { check::<SubmitRegisterTraderTransactionParams>(request); check::<SendRegisterIxsResponse>(response); },
                "validate_invite" => { check::<ValidateInviteParams>(request); check::<ValidateInviteResponse>(response); },
                _ => continue,
            }
        }
    }
}
