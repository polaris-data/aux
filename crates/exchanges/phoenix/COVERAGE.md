# Phoenix native API coverage

Contract examples are synthetic; live verification is recorded separately.

| Operation | Rust / Python method | TypeScript method |
| --- | --- | --- |
| `GET /v1/view/exchange` | `get_exchange` | `PhoenixClient.getExchange` |
| `GET /v1/view/exchange/keys` | `get_exchange_keys` | `PhoenixClient.getExchangeKeys` |
| `GET /v1/view/exchange/markets` | `list_markets` | `PhoenixClient.listMarkets` |
| `GET /v1/view/exchange/market/{symbol}` | `get_market` | `PhoenixClient.getMarket` |
| `GET /v1/view/exchange/status` | `get_exchange_status` | `PhoenixClient.getExchangeStatus` |
| `GET /v1/exchange/snapshot` | `get_exchange_snapshot` | `PhoenixClient.getExchangeSnapshot` |
| `POST /v1/exchange/build-register-ixs` | `build_register_trader_instructions` | `PhoenixClient.buildRegisterTraderInstructions` |
| `POST /v1/exchange/send-register-ixs` | `submit_register_trader_transaction` | `PhoenixClient.submitRegisterTraderTransaction` |
| `GET /v1/view/orderbook/{symbol}` | `get_orderbook` | `PhoenixClient.getOrderbook` |
| `GET /v1/market/{symbol}/mark-price` | `get_mark_price` | `PhoenixClient.getMarkPrice` |
| `GET /v1/market/{symbol}/stats` | `get_market_stats_history` | `PhoenixClient.getMarketStatsHistory` |
| `GET /v1/market/{symbol}/next-market-calendar-transition` | `get_next_market_calendar_transition` | `PhoenixClient.getNextMarketCalendarTransition` |
| `GET /v1/market/next-commodity-market-transition` | `get_next_commodity_market_transition` | `PhoenixClient.getNextCommodityMarketTransition` |
| `GET /v1/market/{symbol}/market-calendar` | `get_market_calendar` | `PhoenixClient.getMarketCalendar` |
| `GET /v1/market-calendar/{market_calendar_id}` | `get_market_calendar_by_id` | `PhoenixClient.getMarketCalendarById` |
| `GET /v1/market/commodity-calendar` | `get_commodity_market_calendar` | `PhoenixClient.getCommodityMarketCalendar` |
| `GET /v1/candles/{symbol}` | `get_candles` | `PhoenixClient.getCandles` |
| `GET /v1/view/trader/{pubkey}` | `get_trader` | `PhoenixClient.getTrader` |
| `GET /v1/view/trader-capabilities` | `get_trader_capabilities` | `PhoenixClient.getTraderCapabilities` |
| `GET /v1/trader/state/{authority_pubkey}` | `get_trader_state` | `PhoenixClient.getTraderState` |
| `GET /v1/users/{user_pubkey}/pnl` | `get_user_pn_l` | `PhoenixClient.getUserPnL` |
| `GET /v1/traders/{trader_pubkey}/portfolio-values` | `get_trader_portfolio_values` | `PhoenixClient.getTraderPortfolioValues` |
| `GET /v1/traders/{trader_pubkey}/pnl` | `get_trader_pn_l` | `PhoenixClient.getTraderPnL` |
| `GET /v1/traders/{trader_pubkey}/pnl/markets` | `get_trader_market_pn_l` | `PhoenixClient.getTraderMarketPnL` |
| `GET /v1/users/{user_pubkey}/collateral-history` | `get_user_collateral_history` | `PhoenixClient.getUserCollateralHistory` |
| `GET /v1/trader/{authority}/collateral-history` | `get_trader_collateral_history` | `PhoenixClient.getTraderCollateralHistory` |
| `GET /v1/traders/{trader_pubkey}/collateral-history` | `get_v1_traders_by_trader_pubkey_collateral_history` | `PhoenixClient.getV1TradersByTraderPubkeyCollateralHistory` |
| `GET /v1/users/{user_pubkey}/liquidation-history` | `get_user_liquidation_history` | `PhoenixClient.getUserLiquidationHistory` |
| `GET /v1/trader/{authority}/funding-history` | `get_trader_funding_history` | `PhoenixClient.getTraderFundingHistory` |
| `GET /v1/users/{user_pubkey}/funding-hourly` | `get_user_hourly_funding` | `PhoenixClient.getUserHourlyFunding` |
| `GET /v1/funding/{symbol}/rates` | `get_funding_rate_history` | `PhoenixClient.getFundingRateHistory` |
| `GET /v1/funding/overview` | `get_funding_overview` | `PhoenixClient.getFundingOverview` |
| `GET /v1/trader/{authority}/order-history` | `get_order_history` | `PhoenixClient.getOrderHistory` |
| `GET /v1/traders/{trader_pubkey}/orders_v2` | `get_order_history_v2` | `PhoenixClient.getOrderHistoryV2` |
| `GET /v1/trader/{authority}/trades-history` | `get_trade_history` | `PhoenixClient.getTradeHistory` |
| `GET /v1/users/{user_pubkey}/trades_v2` | `get_user_trade_history_v2` | `PhoenixClient.getUserTradeHistoryV2` |
| `GET /v1/traders/{trader_pubkey}/trades_v2` | `get_trader_trade_history_v2` | `PhoenixClient.getTraderTradeHistoryV2` |
| `GET /v1/trades/{symbol}/fills` | `get_market_fills` | `PhoenixClient.getMarketFills` |
| `GET /v1/traders/{trader_pubkey}/notifications` | `list_trader_notifications` | `PhoenixClient.listTraderNotifications` |
| `POST /v1/traders/{trader_pubkey}/notifications/ack/up-to` | `acknowledge_notifications_up_to_timestamp` | `PhoenixClient.acknowledgeNotificationsUpToTimestamp` |
| `POST /v1/traders/{trader_pubkey}/notifications/ack/notifications` | `acknowledge_notifications` | `PhoenixClient.acknowledgeNotifications` |
| `POST /v1/ix/place-isolated-limit-order` | `build_isolated_limit_order_transaction` | `PhoenixClient.buildIsolatedLimitOrderTransaction` |
| `POST /v1/ix/place-isolated-market-order` | `build_isolated_market_order_transaction` | `PhoenixClient.buildIsolatedMarketOrderTransaction` |
| `POST /v1/ix/cancel-conditional-order` | `build_cancel_conditional_order_transaction` | `PhoenixClient.buildCancelConditionalOrderTransaction` |
| `POST /v1/ix/place-stop-loss-order` | `build_stop_loss_order_transaction` | `PhoenixClient.buildStopLossOrderTransaction` |
| `POST /v1/ix/cancel-stop-loss-order` | `build_cancel_stop_loss_order_transaction` | `PhoenixClient.buildCancelStopLossOrderTransaction` |
| `POST /v1/ix/place-attached-conditional-order` | `build_attached_conditional_order_transaction` | `PhoenixClient.buildAttachedConditionalOrderTransaction` |
| `POST /v1/ix/place-position-conditional-order` | `build_position_conditional_order_transaction` | `PhoenixClient.buildPositionConditionalOrderTransaction` |
| `POST /v1/ix/place-isolated-limit-order-with-conditionals` | `build_isolated_limit_order_transaction_with_conditionals` | `PhoenixClient.buildIsolatedLimitOrderTransactionWithConditionals` |
| `POST /v1/ix/place-isolated-limit-order-enhanced` | `build_isolated_limit_order_transaction_with_liquidation_estimate` | `PhoenixClient.buildIsolatedLimitOrderTransactionWithLiquidationEstimate` |
| `POST /v1/ix/place-isolated-market-order-enhanced` | `build_isolated_market_order_transaction_with_liquidation_estimate` | `PhoenixClient.buildIsolatedMarketOrderTransactionWithLiquidationEstimate` |
| `POST /v1/invite/validate` | `validate_invite` | `PhoenixClient.validateInvite` |
| `GET /v1/invite/check/{wallet}` | `check_wallet_invite_status` | `PhoenixClient.checkWalletInviteStatus` |
| `POST /v1/referral/activate` | `activate_referral` | `PhoenixClient.activateReferral` |
| `GET /v1/referral/activation-permission` | `get_referral_activation_permission` | `PhoenixClient.getReferralActivationPermission` |
| `POST /v1/referral/activate-tx` | `activate_referral_with_user_funded_transaction` | `PhoenixClient.activateReferralWithUserFundedTransaction` |
| `POST /v1/auth/login/privy` | `log_in_with_privy_token` | `PhoenixClient.logInWithPrivyToken` |
| `POST /v1/auth/wallet/transaction-challenge` | `create_wallet_transaction_challenge` | `PhoenixClient.createWalletTransactionChallenge` |
| `POST /v1/auth/login/wallet/transaction` | `log_in_with_signed_wallet_transaction` | `PhoenixClient.logInWithSignedWalletTransaction` |
| `POST /v1/auth/login/service/challenge` | `create_service_login_challenge` | `PhoenixClient.createServiceLoginChallenge` |
| `POST /v1/auth/login/service` | `log_in_with_service_signature` | `PhoenixClient.logInWithServiceSignature` |
| `GET /v1/auth/nonce` | `get_wallet_login_nonce` | `PhoenixClient.getWalletLoginNonce` |
| `POST /v1/auth/login/wallet` | `log_in_with_wallet_signature` | `PhoenixClient.logInWithWalletSignature` |
| `POST /v1/auth/refresh` | `refresh_session` | `PhoenixClient.refreshSession` |
| `POST /v1/auth/logout` | `log_out` | `PhoenixClient.logOut` |
| `GET /v1/auth/jwks` | `get_jwks` | `PhoenixClient.getJwks` |

## Native streams

| Contract | Rust/Python | Node |
| --- | --- | --- |
| `allMids` | `PhoenixClient.subscribe` | `PhoenixClient.subscribe` |
| `exchange` | `PhoenixClient.subscribe` | `PhoenixClient.subscribe` |
| `fundingRate` | `PhoenixClient.subscribe` | `PhoenixClient.subscribe` |
| `orderbook` | `PhoenixClient.subscribe` | `PhoenixClient.subscribe` |
| `traderState` | `PhoenixClient.subscribe` | `PhoenixClient.subscribe` |
| `market` | `PhoenixClient.subscribe` | `PhoenixClient.subscribe` |
| `trades` | `PhoenixClient.subscribe` | `PhoenixClient.subscribe` |
| `candles` | `PhoenixClient.subscribe` | `PhoenixClient.subscribe` |
| `notifications` | `PhoenixClient.subscribe` | `PhoenixClient.subscribe` |

## Instruction builders

The binding method `build_instruction` (`buildInstruction` in Node) dispatches these typed Rust builders.

| Builder name | Native parameter type |
| --- | --- |
| `create_cancel_all_ix` | `instructions::cancel_orders::CancelAllParams` |
| `create_cancel_up_to_ix` | `instructions::cancel_orders::CancelUpToParams` |
| `create_cancel_orders_by_id_ix` | `instructions::cancel_orders::CancelOrdersByIdParams` |
| `create_cancel_stop_loss_ix` | `instructions::cancel_stop_loss::CancelStopLossParams` |
| `create_claim_fees_ix` | `instructions::claim_fees::ClaimFeesParams` |
| `create_create_conditional_orders_account_ix` | `instructions::conditional_order::CreateConditionalOrdersAccountParams` |
| `create_place_position_conditional_order_ix` | `instructions::conditional_order::PlacePositionConditionalOrderParams` |
| `create_place_attached_conditional_order_ix` | `instructions::conditional_order::PlaceAttachedConditionalOrderParams` |
| `create_place_limit_order_with_conditionals_ix` | `instructions::conditional_order::PlaceLimitOrderWithConditionalsParams` |
| `create_cancel_conditional_order_ix` | `instructions::conditional_order::CancelConditionalOrderParams` |
| `create_associated_token_account_idempotent_ix` | `{payer: Pubkey, owner: Pubkey, mint: Pubkey}` |
| `create_delegate_trader_ix` | `instructions::delegate_trader::DelegateTraderParams` |
| `create_deposit_funds_ix` | `instructions::deposit_funds::DepositFundsParams` |
| `create_ember_deposit_ix` | `instructions::ember_deposit::EmberDepositParams` |
| `create_ember_withdraw_ix` | `instructions::ember_withdraw::EmberWithdrawParams` |
| `create_onboard_trader_ix` | `instructions::flight::OnboardTraderParams` |
| `create_proxy_instruction_ix` | `instructions::flight::ProxyInstructionParams` |
| `create_register_builder_ix` | `instructions::flight::RegisterBuilderParams` |
| `create_update_fee_ix` | `instructions::flight::UpdateFeeParams` |
| `create_hawkeye_view_order_quote_ix` | `{accounts: HawkeyeOrderQuoteAccounts, order_packet: &OrderPacket, reference_price_ticks: u64}` |
| `create_hawkeye_view_margin_ix` | `instructions::hawkeye::HawkeyeTraderViewAccounts` |
| `create_hawkeye_view_margin_for_asset_ix` | `{accounts: HawkeyeTraderViewAccounts, asset_id: u32}` |
| `create_hawkeye_view_liquidation_price_ix` | `{accounts: HawkeyeTraderViewAccounts, asset_id: u32}` |
| `create_hawkeye_view_funding_ix` | `{accounts: HawkeyeTraderViewAccounts, asset_id: u32}` |
| `create_hawkeye_view_bbo_ix` | `instructions::hawkeye::HawkeyeBboViewAccounts` |
| `create_place_limit_order_ix` | `instructions::limit_order::LimitOrderParams` |
| `create_place_market_order_ix` | `instructions::market_order::MarketOrderParams` |
| `create_place_market_order_delegated_ix` | `instructions::market_order::MarketOrderDelegatedParams` |
| `create_place_multi_limit_order_ix` | `instructions::multi_limit_order::MultiLimitOrderParams` |
| `create_place_multi_limit_order_v2_ix` | `instructions::multi_limit_order::MultiLimitOrderParamsV2` |
| `create_realloc_trader_ix` | `{payer: Pubkey, trader_wallet: Pubkey, trader_account: Pubkey}` |
| `create_deposit_native_sol_ixs` | `{payer: Pubkey, trader_wallet: Pubkey, lamports: u64, sync: SyncNativeParams}` |
| `create_sync_native_ix` | `instructions::native_sol::SyncNativeParams` |
| `create_withdraw_native_sol_ix` | `instructions::native_sol::WithdrawNativeSolParams` |
| `create_transfer_native_sol_ix` | `instructions::native_sol::TransferNativeSolParams` |
| `create_transfer_native_sol_from_child_to_parent_ix` | `instructions::native_sol::TransferNativeSolFromChildToParentParams` |
| `create_swap_native_ix` | `instructions::native_sol::SwapNativeParams` |
| `create_onboard_trader_delegated_ix` | `instructions::onboard_trader_delegated::OnboardTraderDelegatedParams` |
| `create_place_stop_loss_ix` | `instructions::place_stop_loss::PlaceStopLossParams` |
| `create_register_trader_ix` | `instructions::register_trader::RegisterTraderParams` |
| `create_spl_approve_ix` | `instructions::spl_approve::SplApproveParams` |
| `create_register_spline_ix` | `instructions::spline::RegisterSplineParams` |
| `create_deactivate_spline_ix` | `instructions::spline::DeactivateSplineParams` |
| `create_update_spline_price_ix` | `{accounts: SplineUpdateAccounts, params: UpdateSplinePriceParams}` |
| `create_update_spline_price_with_ordering_ix` | `{accounts: SplineUpdateAccounts, params: UpdateSplinePriceParamsWithOrdering}` |
| `create_update_spline_price_with_metadata_ix` | `{accounts: SplineUpdateAccounts, params: UpdateSplinePriceWithMetadataParams}` |
| `create_update_spline_parameters_with_ordering_ix` | `{accounts: SplineUpdateAccounts, params: UpdateSplineParametersParamsWithOrdering}` |
| `create_update_spline_position_limits_config_ix` | `{accounts: SplineUpdateAccounts, params: UpdateSplinePositionLimitsConfigParams}` |
| `create_sync_spot_ix` | `instructions::spot::SyncSpotParams` |
| `create_withdraw_spot_ix` | `instructions::spot::WithdrawSpotParams` |
| `create_transfer_spot_ix` | `instructions::spot::TransferSpotParams` |
| `create_transfer_spot_from_child_to_parent_ix` | `instructions::spot::TransferSpotFromChildToParentParams` |
| `create_swap_spot_with_usdc_ix` | `instructions::spot::SwapSpotWithUsdcParams` |
| `create_swap_spot_with_sol_ix` | `instructions::spot::SwapSpotWithSolParams` |
| `create_swap_spot_with_spot_ix` | `instructions::spot::SwapSpotWithSpotParams` |
| `create_sync_parent_to_child_ix` | `instructions::sync_parent_to_child::SyncParentToChildParams` |
| `create_transfer_collateral_ix` | `instructions::transfer_collateral::TransferCollateralParams` |
| `create_withdraw_to_target_leverage_ix` | `instructions::transfer_collateral::WithdrawToTargetLeverageParams` |
| `create_transfer_collateral_child_to_parent_ix` | `instructions::transfer_collateral::TransferCollateralChildToParentParams` |
| `create_twap_account_ix` | `instructions::twap::CreateTwapAccountParams` |
| `create_place_twap_order_ix` | `instructions::twap::PlaceTwapOrderParams` |
| `create_execute_twap_order_ix` | `instructions::twap::ExecuteTwapOrderParams` |
| `create_cancel_twap_order_ix` | `instructions::twap::CancelTwapOrderParams` |
| `create_close_inactive_twap_account_ix` | `instructions::twap::CloseInactiveTwapAccountParams` |
| `create_uncross_crank_ix` | `instructions::uncross_crank::UncrossCrankParams` |
| `create_update_trader_preference_bits_ix` | `instructions::update_trader_preference_bits::UpdateTraderPreferenceBitsParams` |
| `create_withdraw_funds_ix` | `instructions::withdraw_funds::WithdrawFundsParams` |

## Signing and transaction helpers

| Rust | Python | Node | Test |
| --- | --- | --- | --- |
| `build_instruction` | `PhoenixClient.build_instruction` | `PhoenixClient.buildInstruction` | `every_instruction_dispatch_fixture` |
| `PreparedTransaction::{new,new_v0}` | `PhoenixClient.prepare_transaction` | `PhoenixClient.prepareTransaction` | `signatures_are_bound_to_reviewed_message_and_all_required_signers` |
| `PreparedTransaction::assemble` | `PhoenixClient.assemble_transaction` | `PhoenixClient.assembleTransaction` | `signatures_are_bound_to_reviewed_message_and_all_required_signers` |
| `PreparedTransaction::validate_server_transaction` | `PhoenixClient.validate_server_transaction` | `PhoenixClient.validateServerTransaction` | `signatures_are_bound_to_reviewed_message_and_all_required_signers` |
| `Signer::sign` | `PhoenixSigner.sign_message` | `PhoenixSigner.signMessage` | `signatures_are_bound_to_reviewed_message_and_all_required_signers` |
| `PreparedTransaction::sign` | `PhoenixSigner.sign_transaction` | `PhoenixSigner.signTransaction` | `versioned_messages_validate_lookup_compilation_and_signature_identity` |
| `PhoenixChainClient::latest_blockhash` | `PhoenixChainClient.latest_blockhash` | `PhoenixChainClient.latestBlockhash` | `rpc_preparation_simulation_and_reconciliation` |
| `PhoenixChainClient::block_height` | `PhoenixChainClient.block_height` | `PhoenixChainClient.blockHeight` | `rpc_preparation_simulation_and_reconciliation` |
| `PhoenixChainClient::prepare` | `PhoenixChainClient.prepare` | `PhoenixChainClient.prepare` | `rpc_preparation_simulation_and_reconciliation` |
| `PhoenixChainClient::simulate` | `PhoenixChainClient.simulate` | `PhoenixChainClient.simulate` | `rpc_preparation_simulation_and_reconciliation` |
| `PhoenixChainClient::submit` | `PhoenixChainClient.submit` | `PhoenixChainClient.submit` | `rpc_preparation_simulation_and_reconciliation` |
| `PhoenixChainClient::signature_status` | `PhoenixChainClient.signature_status` | `PhoenixChainClient.signatureStatus` | `rpc_preparation_simulation_and_reconciliation` |
| `PhoenixChainClient::transaction` | `PhoenixChainClient.transaction` | `PhoenixChainClient.transaction` | `rpc_preparation_simulation_and_reconciliation` |
| `PhoenixChainClient::clear_reconciled_submission` | `PhoenixChainClient.clear_reconciled_submission` | `PhoenixChainClient.clearReconciledSubmission` | `rpc_preparation_simulation_and_reconciliation` |
