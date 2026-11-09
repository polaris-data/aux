# Extended native API coverage

Contract examples are synthetic; live verification is recorded separately.

| Operation | Rust / Python method | TypeScript method |
| --- | --- | --- |
| `GET /api/v1/info/assets` | `get_assets` | `ExtendedClient.getAssets` |
| `GET /api/v1/info/assets/{asset}/price` | `get_asset_price` | `ExtendedClient.getAssetPrice` |
| `GET /api/v1/info/markets` | `get_markets` | `ExtendedClient.getMarkets` |
| `GET /api/v1/info/markets/stats` | `get_markets_stats` | `ExtendedClient.getMarketsStats` |
| `GET /api/v1/info/markets/{market}/stats` | `get_market_statistics` | `ExtendedClient.getMarketStatistics` |
| `GET /api/v1/info/markets/{market}/trades` | `get_market_trades` | `ExtendedClient.getMarketTrades` |
| `GET /api/v1/info/markets/{market}/orderbook` | `get_orderbook_snapshot` | `ExtendedClient.getOrderbookSnapshot` |
| `GET /api/v1/info/{market}/funding` | `get_funding_rates_history` | `ExtendedClient.getFundingRatesHistory` |
| `GET /api/v1/info/candles/{market}/trades` | `get_trading_candles` | `ExtendedClient.getTradingCandles` |
| `GET /api/v1/info/candles/{market}/mark-prices` | `get_mark_price_candles` | `ExtendedClient.getMarkPriceCandles` |
| `GET /api/v1/info/candles/{market}/index-prices` | `get_index_price_candles` | `ExtendedClient.getIndexPriceCandles` |
| `GET /api/v1/info/{market}/open-interests` | `get_open_interests` | `ExtendedClient.getOpenInterests` |
| `GET /api/v1/info/leaderboard` | `get_public_leaderboard` | `ExtendedClient.getPublicLeaderboard` |
| `GET /api/v1/info/referral/link` | `get_public_referral_link` | `ExtendedClient.getPublicReferralLink` |
| `GET /api/v1/info/builder/dashboard` | `get_builder_dashboard` | `ExtendedClient.getBuilderDashboard` |
| `GET /api/v1/info/vault/positions` | `get_vault_positions` | `ExtendedClient.getVaultPositions` |
| `GET /api/v1/info/vault/positions/history` | `get_vault_position_history` | `ExtendedClient.getVaultPositionHistory` |
| `GET /api/v1/info/vault/orders` | `get_vault_orders` | `ExtendedClient.getVaultOrders` |
| `GET /api/v1/info/vault/trades` | `get_vault_trades` | `ExtendedClient.getVaultTrades` |
| `GET /api/v1/info/vault/funding/history` | `get_vault_funding_history` | `ExtendedClient.getVaultFundingHistory` |
| `GET /api/v1/info/status` | `get_exchange_status` | `ExtendedClient.getExchangeStatus` |
| `GET /api/v1/info/settings` | `get_settings` | `ExtendedClient.getSettings` |
| `GET /api/v1/info/starknet` | `get_starknet_domain` | `ExtendedClient.getStarknetDomain` |
| `GET /api/v1/user/accounts` | `get_accounts` | `ExtendedClient.getAccounts` |
| `GET /api/v1/user/account/info` | `get_account` | `ExtendedClient.getAccount` |
| `PATCH /api/v1/user/account/info` | `update_account_info` | `ExtendedClient.updateAccountInfo` |
| `GET /api/v1/user/leverage` | `get_leverage` | `ExtendedClient.getLeverage` |
| `PATCH /api/v1/user/leverage` | `update_leverage` | `ExtendedClient.updateLeverage` |
| `GET /api/v1/user/account/api-keys` | `get_api_keys` | `ExtendedClient.getApiKeys` |
| `POST /api/v1/user/account/api-key` | `create_api_key` | `ExtendedClient.createApiKey` |
| `DELETE /api/v1/user/account/api-key/{id}` | `delete_api_key` | `ExtendedClient.deleteApiKey` |
| `GET /api/v1/user/balance` | `get_balance` | `ExtendedClient.getBalance` |
| `GET /api/v1/user/spot/balances` | `get_spot_balances` | `ExtendedClient.getSpotBalances` |
| `GET /api/v1/user/positions` | `get_positions` | `ExtendedClient.getPositions` |
| `GET /api/v1/user/positions/history` | `get_positions_history` | `ExtendedClient.getPositionsHistory` |
| `GET /api/v1/user/orders` | `get_open_orders` | `ExtendedClient.getOpenOrders` |
| `GET /api/v1/user/orders/{id}` | `get_order` | `ExtendedClient.getOrder` |
| `GET /api/v1/user/orders/history` | `get_orders_history` | `ExtendedClient.getOrdersHistory` |
| `GET /api/v1/user/orders/external/{externalId}` | `get_order_by_external_id` | `ExtendedClient.getOrderByExternalId` |
| `POST /api/v1/user/order` | `place_order` | `ExtendedClient.placeOrder` |
| `DELETE /api/v1/user/order` | `cancel_order_by_external_id` | `ExtendedClient.cancelOrderByExternalId` |
| `POST /api/v1/user/order/scaled` | `create_scaled_order` | `ExtendedClient.createScaledOrder` |
| `DELETE /api/v1/user/order/{id}` | `cancel_order` | `ExtendedClient.cancelOrder` |
| `POST /api/v1/user/order/massCancel` | `mass_cancel` | `ExtendedClient.massCancel` |
| `GET /api/v1/user/trades` | `get_trades` | `ExtendedClient.getTrades` |
| `GET /api/v1/user/funding/history` | `get_user_funding_history` | `ExtendedClient.getUserFundingHistory` |
| `POST /api/v1/user/config/funding-rate` | `update_funding_rate_config` | `ExtendedClient.updateFundingRateConfig` |
| `PATCH /api/v1/user/transfer` | `transfer_patch` | `ExtendedClient.transferPatch` |
| `POST /api/v1/user/transfer` | `transfer` | `ExtendedClient.transfer` |
| `POST /api/v1/user/transfer/onchain` | `post_user_transfer_onchain` | `ExtendedClient.postUserTransferOnchain` |
| `GET /api/v1/user/withdrawal` | `get_withdrawals` | `ExtendedClient.getWithdrawals` |
| `POST /api/v1/user/withdrawal` | `withdraw` | `ExtendedClient.withdraw` |
| `POST /api/v1/user/withdrawal/onchain` | `onchain_withdrawal` | `ExtendedClient.onchainWithdrawal` |
| `POST /api/v1/user/deposit` | `create_deposit` | `ExtendedClient.createDeposit` |
| `GET /api/v1/user/bridge/config` | `get_bridge_config` | `ExtendedClient.getBridgeConfig` |
| `GET /api/v1/user/bridge/quote` | `get_bridge_quote` | `ExtendedClient.getBridgeQuote` |
| `POST /api/v1/user/bridge/quote` | `commit_bridge_quote` | `ExtendedClient.commitBridgeQuote` |
| `POST /api/v1/user/wallet/validation` | `validate_wallet` | `ExtendedClient.validateWallet` |
| `GET /api/v1/user/wallet/validation/{id}` | `get_wallet_validation` | `ExtendedClient.getWalletValidation` |
| `GET /api/v1/user/fees` | `get_fees` | `ExtendedClient.getFees` |
| `GET /api/v1/user/fees/stats` | `get_trading_fee_stats` | `ExtendedClient.getTradingFeeStats` |
| `GET /api/v1/user/leaderboard` | `get_user_leaderboard` | `ExtendedClient.getUserLeaderboard` |
| `POST /api/v1/user/deadmanswitch` | `set_deadman_switch` | `ExtendedClient.setDeadmanSwitch` |
| `GET /api/v1/user/claim` | `get_claims` | `ExtendedClient.getClaims` |
| `POST /api/v1/user/claim` | `claim_testing_funds` | `ExtendedClient.claimTestingFunds` |
| `GET /api/v1/user/claim/{id}` | `get_claim` | `ExtendedClient.getClaim` |
| `GET /api/v1/user/nextclaim` | `get_next_claim_time` | `ExtendedClient.getNextClaimTime` |
| `GET /api/v1/user/rewards/partner` | `get_partner_rewards` | `ExtendedClient.getPartnerRewards` |
| `GET /api/v1/user/rewards/affiliate` | `get_affiliate_rewards` | `ExtendedClient.getAffiliateRewards` |
| `GET /api/v1/user/rewards/earned` | `get_earned_rewards` | `ExtendedClient.getEarnedRewards` |
| `GET /api/v1/user/rewards/leaderboard/stats` | `get_reward_leaderboard_stats` | `ExtendedClient.getRewardLeaderboardStats` |
| `GET /api/v1/user/rebates/csv` | `get_rebates_csv` | `ExtendedClient.getRebatesCsv` |
| `GET /api/v1/user/rebates/stats` | `get_rebate_stats` | `ExtendedClient.getRebateStats` |
| `GET /api/v1/user/referrals/status` | `get_referral_status` | `ExtendedClient.getReferralStatus` |
| `GET /api/v1/user/referrals/links` | `get_referral_links` | `ExtendedClient.getReferralLinks` |
| `POST /api/v1/user/referrals` | `create_referral_link` | `ExtendedClient.createReferralLink` |
| `PUT /api/v1/user/referrals` | `update_referral_link` | `ExtendedClient.updateReferralLink` |
| `POST /api/v1/user/referrals/use` | `use_referral_code` | `ExtendedClient.useReferralCode` |
| `GET /api/v1/user/referrals/rebates/period/csv` | `get_referral_rebates_by_period_csv` | `ExtendedClient.getReferralRebatesByPeriodCsv` |
| `GET /api/v1/user/referrals/rebates/user/csv` | `get_referral_rebates_by_user_csv` | `ExtendedClient.getReferralRebatesByUserCsv` |
| `GET /api/v1/user/referrals/rebates/subaffiliate/csv` | `get_referral_rebates_by_subaffiliate_csv` | `ExtendedClient.getReferralRebatesBySubaffiliateCsv` |
| `GET /api/v1/user/referrals/rebates/link/csv` | `get_referral_rebates_by_link_csv` | `ExtendedClient.getReferralRebatesByLinkCsv` |
| `GET /api/v1/user/referrals/dashboard` | `get_referral_dashboard` | `ExtendedClient.getReferralDashboard` |
| `GET /api/v1/user/affiliate` | `get_affiliate` | `ExtendedClient.getAffiliate` |
| `GET /api/v1/user/assetOperations` | `asset_operations` | `ExtendedClient.assetOperations` |
| `GET /api/v1/user/asset-operations` | `get_asset_operations` | `ExtendedClient.getAssetOperations` |
| `GET /api/v1/user/client/info` | `get_client` | `ExtendedClient.getClient` |
| `PUT /api/v1/user/client/info/walletChecked` | `mark_wallet_checked` | `ExtendedClient.markWalletChecked` |
| `GET /api/v1/user/export/csv/zip` | `export_csv_zip` | `ExtendedClient.exportCsvZip` |
| `GET /api/v1/user/wrapper` | `get_wrapper` | `ExtendedClient.getWrapper` |
| `POST /api/v1/user/order/rfq` | `place_rfq_order` | `ExtendedClient.placeRfqOrder` |
| `DELETE /api/v1/user/order/{order_id}` | `delete_user_order_order_id` | `ExtendedClient.deleteUserOrderOrderId` |
| `GET /api/v1/builder/trades` | `get_builder_trades` | `ExtendedClient.getBuilderTrades` |
| `GET /api/v1/info/assets/{asset_name}/price` | `get_info_assets_asset_name_price` | `ExtendedClient.getInfoAssetsAssetNamePrice` |
| `GET /api/v1/info/candles/{market}/{candle_type}` | `get_candles_history` | `ExtendedClient.getCandlesHistory` |
| `POST /api/v1/vault/user/deposits` | `deposit_to_vault` | `ExtendedClient.depositToVault` |
| `POST /api/v1/vault/user/withdrawals` | `withdraw_from_vault` | `ExtendedClient.withdrawFromVault` |
| `GET /api/v1/user/orders/{order_id}` | `get_order_by_id` | `ExtendedClient.getOrderById` |
| `GET /api/v1/user/orders/external/{external_id}` | `get_user_orders_external_external_id` | `ExtendedClient.getUserOrdersExternalExternalId` |
| `POST /auth/onboard` | `onboard_client` | `ExtendedClient.onboardClient` |
| `POST /auth/onboard/subaccount` | `onboard_subaccount` | `ExtendedClient.onboardSubaccount` |
| `GET /api/v1/info/candles/{market}/{candleType}` | `get_info_candles_market_candle_type` | `ExtendedClient.getInfoCandlesMarketCandleType` |
| `GET /api/v1/portfolio/charts/equities` | `get_account_equity_history` | `ExtendedClient.getAccountEquityHistory` |
| `GET /api/v1/portfolio/charts/pnl` | `get_account_pn_l_history` | `ExtendedClient.getAccountPnLHistory` |
| `GET /api/v1/portfolio/charts/pnl/percentage` | `get_account_pn_l_percentage_history` | `ExtendedClient.getAccountPnLPercentageHistory` |
| `GET /api/v1/portfolio/charts/pnl/cumulative` | `get_cumulative_account_pn_l_history` | `ExtendedClient.getCumulativeAccountPnLHistory` |
| `GET /api/v1/portfolio/charts/pnl/cumulative/percentage` | `get_cumulative_account_pn_l_percentage_history` | `ExtendedClient.getCumulativeAccountPnLPercentageHistory` |
| `GET /api/v1/portfolio/charts/vault-equities` | `get_account_vault_equity_history` | `ExtendedClient.getAccountVaultEquityHistory` |
| `GET /api/v1/portfolio/charts/max-drawdown` | `get_account_max_drawdown_history` | `ExtendedClient.getAccountMaxDrawdownHistory` |
| `GET /api/v1/portfolio/charts/funding` | `get_account_funding_chart` | `ExtendedClient.getAccountFundingChart` |
| `GET /api/v1/portfolio/accounts/summary` | `get_account_portfolio_summary` | `ExtendedClient.getAccountPortfolioSummary` |
| `GET /api/v1/portfolio/accounts/health` | `get_account_health` | `ExtendedClient.getAccountHealth` |
| `GET /api/v1/portfolio/accounts/performance` | `get_account_performance` | `ExtendedClient.getAccountPerformance` |
| `GET /api/v1/portfolio/funding/stats` | `get_account_funding_stats` | `ExtendedClient.getAccountFundingStats` |
| `GET /api/v1/portfolio/funding/history` | `get_account_funding_history` | `ExtendedClient.getAccountFundingHistory` |
| `GET /api/v1/vault/public/performance` | `get_vault_performance` | `ExtendedClient.getVaultPerformance` |
| `GET /api/v1/vault/public/summary` | `get_vault_summary` | `ExtendedClient.getVaultSummary` |
| `GET /api/v1/interest/info/rate-curves` | `get_interest_rate_curves_history` | `ExtendedClient.getInterestRateCurvesHistory` |
| `GET /api/v1/interest/info/latest-rate-curves` | `get_latest_interest_rate_curve` | `ExtendedClient.getLatestInterestRateCurve` |
| `GET /api/v1/interest/key-metrics` | `get_interest_key_metrics` | `ExtendedClient.getInterestKeyMetrics` |
| `GET /api/v1/interest/daily-metrics` | `get_interest_daily_metrics` | `ExtendedClient.getInterestDailyMetrics` |
| `GET /api/v1/interest/payment-chart` | `get_interest_payment_chart` | `ExtendedClient.getInterestPaymentChart` |
| `GET /api/v1/interest/payments` | `get_interest_payments_history` | `ExtendedClient.getInterestPaymentsHistory` |

## Native streams

| Contract | Rust/Python | Node |
| --- | --- | --- |
| `orderbooks` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `rfq_orderbooks` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `trades` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `funding` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `candles` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `mark_price` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `index_price` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `account` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `orderbooks_rpc` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `rfq_orderbooks_rpc` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `trades_rpc` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `funding_rpc` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `candles_rpc` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `mark_price_rpc` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `index_price_rpc` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |
| `account_rpc` | `ExtendedClient.subscribe` | `ExtendedClient.subscribe` |

## Signing and transaction helpers

| Rust | Python | Node | Test |
| --- | --- | --- | --- |
| `order_hash / limit_order_hash / transfer_hash / withdrawal_hash` | `ExtendedClient.settlement_hash` | `ExtendedClient.settlementHash` | `official_order_hash_and_signature_domain_separation` |
| `prepare_order` | `ExtendedClient.prepare_order` | `ExtendedClient.prepareOrder` | `exact_order_amounts_use_native_side_and_fee_rounding` |
| `sign_order` | `ExtendedClient.sign_order` | `ExtendedClient.signOrder` | `attached_and_position_tpsl_legs_are_signed_independently` |
| `onboarding_payload` | `ExtendedClient.onboarding_payload` | `ExtendedClient.onboardingPayload` | `official_wallet_onboarding_and_api_signatures` |
| `scale_decimal` | `ExtendedClient.scale_decimal` | `ExtendedClient.scaleDecimal` | `exact_order_amounts_use_native_side_and_fee_rounding` |
| `Signer::sign` | `ExtendedSigner.sign` | `ExtendedSigner.sign` | `official_order_hash_and_signature_domain_separation` |
| `key_derivation_payload` | `ExtendedClient.key_derivation_payload` | `ExtendedClient.keyDerivationPayload` | `official_wallet_onboarding_and_api_signatures` |
| `api_request_payload` | `ExtendedClient.api_request_payload` | `ExtendedClient.apiRequestPayload` | `official_wallet_onboarding_and_api_signatures` |
| `onboarding_l2_hash` | `ExtendedClient.onboarding_l2_hash` | `ExtendedClient.onboardingL2Hash` | `official_wallet_onboarding_and_api_signatures` |
| `LocalStarkSigner::from_wallet_signature` | `ExtendedSigner.from_wallet_signature` | `ExtendedSigner.fromWalletSignature` | `official_wallet_signature_key_derivation` |
| `stark_payload / Signer::sign` | `ExtendedSigner.sign_hash` | `ExtendedSigner.signHash` | `official_order_hash_and_signature_domain_separation` |
