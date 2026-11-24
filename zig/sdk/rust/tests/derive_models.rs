#![cfg(feature = "derive-models")]
// Generated from schema/derive.json.
#[rustfmt::skip]
mod fixtures {
    use aux_zig::derive_models::*;
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
        for row in rows.iter().filter(|r| r["venue"] == "derive") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "private_burn_vault_shares" => { check::<BurnSharesRequest>(request); check::<VaultSettleResponse>(response); },
                "private_cancel" => { check::<CancelOrderRequest>(request); check::<Order>(response); },
                "private_cancel_algo_order" => { check::<CancelAlgoOrderRequest>(request); check::<Order>(response); },
                "private_cancel_all" => { check::<CancelAllRequest>(request); check::<CancelAllResponse>(response); },
                "private_cancel_all_algo_orders" => { check::<CancelAllAlgoOrdersRequest>(request); check::<CancelAllAlgoOrdersResponse>(response); },
                "private_cancel_all_trigger_orders" => { check::<CancelAllTriggerOrdersRequest>(request); check::<CancelAllTriggerOrdersResponse>(response); },
                "private_cancel_all_vault_requests" => { check::<CancelVaultRequestRequest>(request); check::<VaultCancelResponse>(response); },
                "private_cancel_batch_quotes" => { check::<CancelBatchQuotesRequest>(request); check::<CancelBatchResult>(response); },
                "private_cancel_batch_rfqs" => { check::<CancelBatchRfqsRequest>(request); check::<CancelBatchRfqsResponse>(response); },
                "private_cancel_by_instrument" => { check::<CancelByInstrumentRequest>(request); check::<CancelByInstrumentResponse>(response); },
                "private_cancel_by_label" => { check::<CancelByLabelRequest>(request); check::<CancelByLabelResponse>(response); },
                "private_cancel_by_nonce" => { check::<CancelByNonceRequest>(request); check::<CancelByNonceResponse>(response); },
                "private_cancel_quote" => { check::<CancelQuoteRequest>(request); check::<Quote>(response); },
                "private_cancel_rfq" => { check::<CancelRfqRequest>(request); check::<CancelRfqResponse>(response); },
                "private_cancel_trigger_order" => { check::<CancelTriggerOrderRequest>(request); check::<Order>(response); },
                "private_change_subaccount_label" => { check::<ChangeSubaccountLabelRequest>(request); check::<PrivateChangeSubaccountLabelResponse>(response); },
                "private_create_vault" => { check::<CreateVaultRequest>(request); check::<VaultCreateResponse>(response); },
                "private_delete_subaccount" => { check::<DeleteSubaccountRequest>(request); check::<DeleteSubaccountResponse>(response); },
                "private_edit_session_key" => { check::<EditSessionKeyRequest>(request); check::<SessionKey>(response); },
                "private_execute_quote" => { check::<ExecuteQuoteRequest>(request); check::<QuoteExecuteResponse>(response); },
                "private_force_burn" => { check::<ForceBurnRequest>(request); check::<VaultForceBurnResponse>(response); },
                "private_get_account" => { check::<GetAccountRequest>(request); check::<PrivateGetAccountResponse>(response); },
                "private_get_algo_orders" => { check::<GetAlgoOrdersRequest>(request); check::<Vec<Order>>(response); },
                "private_get_all_portfolios" => { check::<GetAllPortfoliosRequest>(request); check::<Vec<Subaccount>>(response); },
                "private_get_collaterals" => { check::<GetCollateralsRequest>(request); check::<PrivateGetCollateralsResponse>(response); },
                "private_get_curated_vaults" => { check::<GetCuratedVaultsRequest>(request); check::<VaultIdsResponse>(response); },
                "private_get_deposit_history" => { check::<GetDepositHistoryRequest>(request); check::<DepositHistoryResult>(response); },
                "private_get_erc20_transfer_history" => { check::<GetErc20TransferHistoryRequest>(request); check::<TransferHistoryResult>(response); },
                "private_get_funding_history" => { check::<GetFundingHistoryRequest>(request); check::<PerpSettlementHistoryResponse>(response); },
                "private_get_interest_history" => { check::<GetInterestHistoryRequest>(request); check::<InterestHistoryResult>(response); },
                "private_get_live_burn_requests" => { check::<GetLiveBurnRequestsRequest>(request); check::<MultipleVaultRequestsResponse>(response); },
                "private_get_live_mint_requests" => { check::<GetLiveMintRequestsRequest>(request); check::<MultipleVaultRequestsResponse>(response); },
                "private_get_live_vault_requests" => { check::<GetLiveVaultRequestsRequest>(request); check::<MultipleVaultRequestsResponse>(response); },
                "private_get_maker_rfq_history" => { check::<GetMakerRfqHistoryRequest>(request); check::<RFQPollResponse>(response); },
                "private_get_margin" => { check::<PrivateGetMarginRequest>(request); check::<GetMarginResponse>(response); },
                "private_get_mmp_config" => { check::<MmpScopeRequest>(request); check::<Vec<MmpConfigResult>>(response); },
                "private_get_open_orders" => { check::<GetOpenOrdersRequest>(request); check::<AggregatedOrdersResult>(response); },
                "private_get_option_settlement_history" => { check::<GetOptionSettlementHistoryParams>(request); check::<OptionSettlementHistoryResponse>(response); },
                "private_get_order" => { check::<GetOrderRequest>(request); check::<Order>(response); },
                "private_get_order_history" => { check::<GetOrderHistoryRequest>(request); check::<PaginatedOrdersResult>(response); },
                "private_get_positions" => { check::<GetPositionsRequest>(request); check::<PrivateGetPositionsResponse>(response); },
                "private_get_quotes" => { check::<GetQuotesRequest>(request); check::<QuoteGetResponse>(response); },
                "private_get_rfqs" => { check::<GetRfqsRequest>(request); check::<RFQGetResponse>(response); },
                "private_get_shareholder_vaults" => { check::<GetShareholderVaultsRequest>(request); check::<VaultIdsResponse>(response); },
                "private_get_subaccount" => { check::<GetSubaccountRequest>(request); check::<Subaccount>(response); },
                "private_get_subaccount_value_history" => { check::<GetSubaccountValueHistoryRequest>(request); check::<SubaccountValueHistoryResult>(response); },
                "private_get_subaccounts" => { check::<GetSubaccountsRequest>(request); check::<PrivateGetSubaccountsResponse>(response); },
                "private_get_trade_history" => { check::<GetTradeHistoryRequest>(request); check::<PaginatedTradesResult>(response); },
                "private_get_trigger_orders" => { check::<GetTriggerOrdersRequest>(request); check::<AggregatedTriggerOrdersResult>(response); },
                "private_get_vault_request_history" => { check::<GetVaultRequestHistoryRequest>(request); check::<PaginatedVaultRequestHistory>(response); },
                "private_get_vault_shares" => { check::<GetVaultSharesRequest>(request); check::<VaultSharesResponse>(response); },
                "private_get_withdraw_addresses" => { check::<GetWithdrawAddressesParams>(request); check::<GetWithdrawAddressesResult>(response); },
                "private_get_withdrawal_history" => { check::<GetWithdrawalHistoryRequest>(request); check::<WithdrawalHistoryResult>(response); },
                "private_liquidate" => { check::<PrivateLiquidateRequest>(request); check::<PrivateLiquidateResponse>(response); },
                "private_mint_vault_shares" => { check::<MintSharesRequest>(request); check::<VaultSettleResponse>(response); },
                "private_order" => { check::<CreateOrderRequest>(request); check::<OrderCreatedResponse>(response); },
                "private_order_debug" => { check::<CreateOrderRequest>(request); check::<OrderDebugResponse>(response); },
                "private_order_quote" => { check::<OrderQuoteRequest>(request); check::<OrderQuoteResponse>(response); },
                "private_poll_quotes" => { check::<PollQuotesRequest>(request); check::<QuotePollResponse>(response); },
                "private_poll_rfqs" => { check::<PollRfqsRequest>(request); check::<RFQPollResponse>(response); },
                "private_register_bridge_deposit" => { check::<RegisterBridgeDepositParams>(request); check::<PendingDepositBridgeOrigin>(response); },
                "private_register_withdraw_address" => { check::<RegisterWithdrawAddressParams>(request); check::<RegisterWithdrawAddressResult>(response); },
                "private_reject_deposit_request" => { check::<RejectDepositRequestRequest>(request); check::<VaultRequestAckResponse>(response); },
                "private_replace" => { check::<ReplaceOrderRequest>(request); check::<ReplaceOrderResponse>(response); },
                "private_replace_quote" => { check::<ReplaceQuoteRequest>(request); check::<QuoteReplaceResponse>(response); },
                "private_request_vault_deposit" => { check::<RequestVaultDepositRequest>(request); check::<VaultRequestAckResponse>(response); },
                "private_request_vault_withdraw" => { check::<RequestVaultWithdrawRequest>(request); check::<VaultRequestAckResponse>(response); },
                "private_reset_mmp" => { check::<MmpScopeRequest>(request); check::<ResetMmpResponse>(response); },
                "private_rfq_get_best_quote" => { check::<RfqGetBestQuoteRequest>(request); check::<RfqGetBestQuoteResponse>(response); },
                "private_send_quote" => { check::<SendQuoteRequest>(request); check::<Quote>(response); },
                "private_send_rfq" => { check::<SendRfqRequest>(request); check::<Rfq>(response); },
                "private_session_keys" => { check::<SessionKeysRequest>(request); check::<PrivateSessionKeysResponse>(response); },
                "private_set_mmp_config" => { check::<SetMmpConfigRequest>(request); check::<SetMmpConfigResponse>(response); },
                "private_set_session_key" => { check::<SetSessionKeyRequest>(request); check::<PrivateSetSessionKeyResponse>(response); },
                "private_set_session_key_debug" => { check::<SetSessionKeyRequest>(request); check::<SetSessionKeyDebugResponse>(response); },
                "private_transfer_positions" => { check::<TransferPositionsRequest>(request); check::<TransferPositionsResponse>(response); },
                "private_transfer_positions_debug" => { check::<TransferPositionsRequest>(request); check::<TransferPositionsDebugResponse>(response); },
                "private_transfer_spot" => { check::<PrivateTransferSpotRequest>(request); check::<PrivateTransferSpotResponse>(response); },
                "private_transfer_spot_debug" => { check::<PrivateTransferSpotRequest>(request); check::<TransferSpotDebugResponse>(response); },
                "private_transfer_spot_external" => { check::<PrivateTransferSpotExternalRequest>(request); check::<PrivateTransferSpotExternalResponse>(response); },
                "private_transfer_spot_external_debug" => { check::<PrivateTransferSpotExternalRequest>(request); check::<TransferSpotExternalDebugResponse>(response); },
                "private_update_vault_info" => { check::<UpdateVaultInfoRequest>(request); check::<OffchainAckResponse>(response); },
                "private_update_whitelisted_recipients" => { check::<UpdateWhitelistedRecipientsRequest>(request); check::<UpdateWhitelistedRecipientsResponse>(response); },
                "private_update_whitelisted_recipients_debug" => { check::<UpdateWhitelistedRecipientsRequest>(request); check::<UpdateWhitelistedRecipientsDebugResponse>(response); },
                "private_withdraw" => { check::<PrivateWithdrawRequest>(request); check::<PrivateWithdrawResponse>(response); },
                "public_decode_action" => { check::<DecodeActionRequest>(request); check::<DecodeActionResponse>(response); },
                "public_execute_quote_debug" => { check::<PublicExecuteQuoteDebugRequest>(request); check::<QuoteExecuteDebugResult>(response); },
                "public_get_all_currencies" => { check::<EmptyRequest>(request); check::<Vec<Currency>>(response); },
                "public_get_all_instruments" => { check::<GetAllInstrumentsRequest>(request); check::<GetAllInstrumentsResponse>(response); },
                "public_get_all_live_instruments" => { check::<EmptyRequest>(request); check::<Vec<String>>(response); },
                "public_get_all_referral_codes" => { check::<GetAllReferralCodesParams>(request); check::<Vec<Referrer>>(response); },
                "public_get_currency" => { check::<GetCurrencyRequest>(request); check::<Currency>(response); },
                "public_get_funding_rate_history" => { check::<GetFundingRateHistoryRequest>(request); check::<FundingRateHistoryResult>(response); },
                "public_get_index_chart_data" => { check::<GetIndexChartDataRequest>(request); check::<Vec<IndexCandle>>(response); },
                "public_get_instrument" => { check::<GetInstrumentRequest>(request); check::<Instrument>(response); },
                "public_get_interest_rate_history" => { check::<GetInterestRateHistoryRequest>(request); check::<InterestRateHistoryResult>(response); },
                "public_get_latest_signed_feeds" => { check::<GetLatestSignedFeedsRequest>(request); check::<GetLatestSignedFeedsResponse>(response); },
                "public_get_liquidation_history" => { check::<GetLiquidationHistoryRequest>(request); check::<LiquidationHistoryResult>(response); },
                "public_get_live_auctions" => { check::<EmptyRequest>(request); check::<PublicGetLiveAuctionsResponse>(response); },
                "public_get_live_incidents" => { check::<GetLiveIncidentsParams>(request); check::<GetLiveIncidentsResponse>(response); },
                "public_get_maker_program_scores" => { check::<GetMakerProgramScoresParams>(request); check::<GetMakerProgramScoresResponse>(response); },
                "public_get_maker_programs" => { check::<GetMakerProgramsParams>(request); check::<Vec<ProgramResponse>>(response); },
                "public_get_margin" => { check::<PublicGetMarginRequest>(request); check::<GetMarginResponse>(response); },
                "public_get_onchain_action_history" => { check::<GetOnchainActionHistoryParams>(request); check::<GetOnchainActionHistoryResponse>(response); },
                "public_get_option_settlement_prices" => { check::<GetOptionSettlementPricesRequest>(request); check::<OptionSettlementPricesResult>(response); },
                "public_get_pending_deposits" => { check::<GetPendingDepositsParams>(request); check::<GetPendingDepositsResult>(response); },
                "public_get_perp_impact_twap" => { check::<GetPerpImpactTwapRequest>(request); check::<PerpImpactTwapResult>(response); },
                "public_get_rate_limits" => { check::<EmptyRequest>(request); check::<RateLimitResult>(response); },
                "public_get_referral_performance" => { check::<GetReferralPerformanceParams>(request); check::<GetReferralPerformanceResult>(response); },
                "public_get_risk_universes" => { check::<EmptyRequest>(request); check::<Vec<RiskUniverse>>(response); },
                "public_get_subaccounts" => { check::<PublicGetSubaccountsRequest>(request); check::<PrivateGetSubaccountsResponse>(response); },
                "public_get_ticker" => { check::<GetTickerRequest>(request); check::<TickerSlimSnapshot>(response); },
                "public_get_tickers" => { check::<GetTickersRequest>(request); check::<GetTickersResponse>(response); },
                "public_get_time" => { check::<EmptyRequest>(request); check::<serde_json::Number>(response); },
                "public_get_trade_history" => { check::<GetPublicTradeHistoryRequest>(request); check::<PublicTradesResult>(response); },
                "public_get_tradingview_chart_data" => { check::<GetTradingviewChartDataRequest>(request); check::<Vec<TradingviewCandle>>(response); },
                "public_get_transaction" => { check::<GetTransactionParams>(request); check::<GetTransactionResult>(response); },
                "public_get_vault" => { check::<GetVaultRequest>(request); check::<Vault>(response); },
                "public_get_vault_action_history" => { check::<GetVaultActionHistoryRequest>(request); check::<PaginatedVaultActionHistory>(response); },
                "public_get_vault_performance_history" => { check::<GetVaultPerformanceHistoryRequest>(request); check::<VaultPerformanceHistoryResult>(response); },
                "public_get_vaults" => { check::<GetVaultsRequest>(request); check::<VaultsResponse>(response); },
                "public_get_wallet" => { check::<PublicGetWalletRequest>(request); check::<PublicGetWalletResponse>(response); },
                "public_get_wallets_from_session_key" => { check::<GetWalletsFromSessionKeyRequest>(request); check::<PublicGetWalletsFromSessionKeyResponse>(response); },
                "public_margin_watch" => { check::<MarginWatchRequest>(request); check::<PublicMarginWatchResponse>(response); },
                "public_order_quote" => { check::<OrderQuoteRequest>(request); check::<OrderQuoteResponse>(response); },
                "public_register_deposit_address" => { check::<RegisterDepositAddressParams>(request); check::<RegisterDepositAddressResult>(response); },
                "public_send_quote_debug" => { check::<PublicSendQuoteDebugRequest>(request); check::<QuoteSendDebugResult>(response); },
                "public_withdraw_debug" => { check::<PublicWithdrawDebugRequest>(request); check::<serde_json::Value>(response); },
                _ => continue,
            }
        }
    }
}
