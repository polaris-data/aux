# Paradex API coverage

Generated from the pinned specification and documentation union. See [provenance](reference/provenance.json) and [verification](VERIFICATION.md). Fixtures are synthetic offline contracts; coverage does not imply funded verification.

142 REST operations; 30 WebSocket entries.

| REST operation | Rust method | Python method | Fixture / test |
| --- | --- | --- | --- |
| `GET /account` | `ParadexClient::get_account` | `aux_py.ParadexClient.get_account` | [contract_get_account](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /account/compliance` | `ParadexClient::get_compliance_actions` | `aux_py.ParadexClient.get_compliance_actions` | [contract_get_compliance_actions](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/compliance` | `ParadexClient::record_compliance_action` | `aux_py.ParadexClient.record_compliance_action` | [contract_record_compliance_action](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /account/history` | `ParadexClient::get_account_history` | `aux_py.ParadexClient.get_account_history` | [contract_get_account_history](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /account/info` | `ParadexClient::get_accounts_info` | `aux_py.ParadexClient.get_accounts_info` | [contract_get_accounts_info](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /account/keys/subkeys` | `ParadexClient::list_subkeys` | `aux_py.ParadexClient.list_subkeys` | [contract_list_subkeys](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/keys/subkeys` | `ParadexClient::register_subkey` | `aux_py.ParadexClient.register_subkey` | [contract_register_subkey](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/keys/subkeys/activate` | `ParadexClient::activate_subkey` | `aux_py.ParadexClient.activate_subkey` | [contract_activate_subkey](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /account/keys/subkeys/{public_key}` | `ParadexClient::get_subkey` | `aux_py.ParadexClient.get_subkey` | [contract_get_subkey](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `PUT /account/keys/subkeys/{public_key}` | `ParadexClient::update_subkey` | `aux_py.ParadexClient.update_subkey` | [contract_update_subkey](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /account/keys/subkeys/{public_key}` | `ParadexClient::revoke_subkey` | `aux_py.ParadexClient.revoke_subkey` | [contract_revoke_subkey](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `PUT /account/keys/subkeys/{public_key}/allowed-cidrs` | `ParadexClient::update_subkey_allowed_cidrs` | `aux_py.ParadexClient.update_subkey_allowed_cidrs` | [contract_update_subkey_allowed_cidrs](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `PUT /account/keys/subkeys/{public_key}/block-trade-signing` | `ParadexClient::update_subkey_block_trade_signing` | `aux_py.ParadexClient.update_subkey_block_trade_signing` | [contract_update_subkey_block_trade_signing](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /account/margin` | `ParadexClient::get_account_margin` | `aux_py.ParadexClient.get_account_margin` | [contract_get_account_margin](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/margin/{market}` | `ParadexClient::upsert_account_margin` | `aux_py.ParadexClient.upsert_account_margin` | [contract_upsert_account_margin](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /account/mmp` | `ParadexClient::get_account_mmp` | `aux_py.ParadexClient.get_account_mmp` | [contract_get_account_mmp](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/mmp/reset` | `ParadexClient::reset_account_mmp` | `aux_py.ParadexClient.reset_account_mmp` | [contract_reset_account_mmp](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/mmp/{base_asset}` | `ParadexClient::upsert_account_mmp` | `aux_py.ParadexClient.upsert_account_mmp` | [contract_upsert_account_mmp](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /account/mmp/{base_asset}` | `ParadexClient::delete_account_mmp` | `aux_py.ParadexClient.delete_account_mmp` | [contract_delete_account_mmp](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/paradigm/enable` | `ParadexClient::rfq_enable` | `aux_py.ParadexClient.rfq_enable` | [contract_rfq_enable](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /account/profile` | `ParadexClient::get_account_profile` | `aux_py.ParadexClient.get_account_profile` | [contract_get_account_profile](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `PUT /account/profile` | `ParadexClient::update_account_profile` | `aux_py.ParadexClient.update_account_profile` | [contract_update_account_profile](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/profile/market_max_slippage/{market}` | `ParadexClient::update_account_market_max_slippage` | `aux_py.ParadexClient.update_account_market_max_slippage` | [contract_update_account_market_max_slippage](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/profile/notifications` | `ParadexClient::update_account_notification_preferences` | `aux_py.ParadexClient.update_account_notification_preferences` | [contract_update_account_notification_preferences](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/profile/notifications/last_seen` | `ParadexClient::update_last_seen` | `aux_py.ParadexClient.update_last_seen` | [contract_update_last_seen](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/profile/referral_code` | `ParadexClient::update_account_profile_referral_code` | `aux_py.ParadexClient.update_account_profile_referral_code` | [contract_update_account_profile_referral_code](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/profile/refresh_inventory` | `ParadexClient::refresh_account_inventory` | `aux_py.ParadexClient.refresh_account_inventory` | [contract_refresh_account_inventory](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/profile/size_currency_display` | `ParadexClient::update_account_size_currency_display` | `aux_py.ParadexClient.update_account_size_currency_display` | [contract_update_account_size_currency_display](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/profile/username` | `ParadexClient::update_account_profile_username` | `aux_py.ParadexClient.update_account_profile_username` | [contract_update_account_profile_username](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/referrer` | `ParadexClient::update_account_referrer` | `aux_py.ParadexClient.update_account_referrer` | [contract_update_account_referrer](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /account/settings` | `ParadexClient::get_account_settings` | `aux_py.ParadexClient.get_account_settings` | [contract_get_account_settings](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/settings/trading_value_display` | `ParadexClient::update_account_trading_value_display` | `aux_py.ParadexClient.update_account_trading_value_display` | [contract_update_account_trading_value_display](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /account/subaccounts` | `ParadexClient::get_sub_accounts` | `aux_py.ParadexClient.get_sub_accounts` | [contract_get_sub_accounts](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /account/subaccounts/{address}` | `ParadexClient::archive_sub_account` | `aux_py.ParadexClient.archive_sub_account` | [contract_archive_sub_account](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /account/summary` | `ParadexClient::get_accounts` | `aux_py.ParadexClient.get_accounts` | [contract_get_accounts](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/terminal-token` | `ParadexClient::post_account_terminal_token` | `aux_py.ParadexClient.post_account_terminal_token` | [contract_post_account_terminal_token](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /account/tokens` | `ParadexClient::list_tokens` | `aux_py.ParadexClient.list_tokens` | [contract_list_tokens](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/tokens` | `ParadexClient::create_token` | `aux_py.ParadexClient.create_token` | [contract_create_token](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /account/tokens/{lookup_id}` | `ParadexClient::revoke_token` | `aux_py.ParadexClient.revoke_token` | [contract_revoke_token](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `PUT /account/tokens/{lookup_id}/allowed-cidrs` | `ParadexClient::update_token_allowed_cidrs` | `aux_py.ParadexClient.update_token_allowed_cidrs` | [contract_update_token_allowed_cidrs](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /algo/orders` | `ParadexClient::get_open_algo_orders` | `aux_py.ParadexClient.get_open_algo_orders` | [contract_get_open_algo_orders](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /algo/orders` | `ParadexClient::create_algo_order` | `aux_py.ParadexClient.create_algo_order` | [contract_create_algo_order](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /algo/orders-history` | `ParadexClient::get_algo_orders_history` | `aux_py.ParadexClient.get_algo_orders_history` | [contract_get_algo_orders_history](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /algo/orders/{algo_id}` | `ParadexClient::get_algo_order_by_id` | `aux_py.ParadexClient.get_algo_order_by_id` | [contract_get_algo_order_by_id](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /algo/orders/{algo_id}` | `ParadexClient::cancel_algo_order` | `aux_py.ParadexClient.cancel_algo_order` | [contract_cancel_algo_order](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /auth` | `ParadexClient::get_jwt` | `aux_py.ParadexClient.get_jwt` | [contract_get_jwt](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /balance` | `ParadexClient::get_balance` | `aux_py.ParadexClient.get_balance` | [contract_get_balance](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /bbo/{market}` | `ParadexClient::get_bbo` | `aux_py.ParadexClient.get_bbo` | [contract_get_bbo](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /bbo/{market}/interactive` | `ParadexClient::get_bbo_interactive` | `aux_py.ParadexClient.get_bbo_interactive` | [contract_get_bbo_interactive](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /block-trades` | `ParadexClient::block_trades_list` | `aux_py.ParadexClient.block_trades_list` | [contract_block_trades_list](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /block-trades` | `ParadexClient::block_trades_create` | `aux_py.ParadexClient.block_trades_create` | [contract_block_trades_create](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /block-trades/{block_trade_id}` | `ParadexClient::block_trades_get` | `aux_py.ParadexClient.block_trades_get` | [contract_block_trades_get](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /block-trades/{block_trade_id}` | `ParadexClient::block_trades_cancel` | `aux_py.ParadexClient.block_trades_cancel` | [contract_block_trades_cancel](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /block-trades/{block_trade_id}/execute` | `ParadexClient::block_trades_execute` | `aux_py.ParadexClient.block_trades_execute` | [contract_block_trades_execute](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /block-trades/{block_trade_id}/offers` | `ParadexClient::block_offers_list` | `aux_py.ParadexClient.block_offers_list` | [contract_block_offers_list](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /block-trades/{block_trade_id}/offers` | `ParadexClient::block_offers_create` | `aux_py.ParadexClient.block_offers_create` | [contract_block_offers_create](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /block-trades/{block_trade_id}/offers/{offer_id}` | `ParadexClient::block_offers_get` | `aux_py.ParadexClient.block_offers_get` | [contract_block_offers_get](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /block-trades/{block_trade_id}/offers/{offer_id}` | `ParadexClient::block_offers_cancel` | `aux_py.ParadexClient.block_offers_cancel` | [contract_block_offers_cancel](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /block-trades/{block_trade_id}/offers/{offer_id}/execute` | `ParadexClient::block_offers_execute` | `aux_py.ParadexClient.block_offers_execute` | [contract_block_offers_execute](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /deposit-addresses` | `ParadexClient::create_deposit_address` | `aux_py.ParadexClient.create_deposit_address` | [contract_create_deposit_address](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /deposit-addresses/networks` | `ParadexClient::list_deposit_networks` | `aux_py.ParadexClient.list_deposit_networks` | [contract_list_deposit_networks](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /deposit-addresses/{id}` | `ParadexClient::get_deposit_address` | `aux_py.ParadexClient.get_deposit_address` | [contract_get_deposit_address](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /dime/config` | `ParadexClient::get_dime_config` | `aux_py.ParadexClient.get_dime_config` | [contract_get_dime_config](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /fills` | `ParadexClient::get_fills` | `aux_py.ParadexClient.get_fills` | [contract_get_fills](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /funding/data` | `ParadexClient::get_funding_data` | `aux_py.ParadexClient.get_funding_data` | [contract_get_funding_data](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /funding/payments` | `ParadexClient::get_funding` | `aux_py.ParadexClient.get_funding` | [contract_get_funding](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /insurance` | `ParadexClient::get_insurance_fund` | `aux_py.ParadexClient.get_insurance_fund` | [contract_get_insurance_fund](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /jwks.json` | `ParadexClient::jwks` | `aux_py.ParadexClient.jwks` | [contract_jwks](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /liquidations` | `ParadexClient::get_liquidations` | `aux_py.ParadexClient.get_liquidations` | [contract_get_liquidations](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /markets` | `ParadexClient::get_markets` | `aux_py.ParadexClient.get_markets` | [contract_get_markets](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /markets/history` | `ParadexClient::get_markets_history` | `aux_py.ParadexClient.get_markets_history` | [contract_get_markets_history](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /markets/klines` | `ParadexClient::markets_klines` | `aux_py.ParadexClient.markets_klines` | [contract_markets_klines](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /markets/settlement-price` | `ParadexClient::get_market_settlement_price` | `aux_py.ParadexClient.get_market_settlement_price` | [contract_get_market_settlement_price](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /markets/summary` | `ParadexClient::get_markets_summary` | `aux_py.ParadexClient.get_markets_summary` | [contract_get_markets_summary](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /onboarding` | `ParadexClient::get_onboarding` | `aux_py.ParadexClient.get_onboarding` | [contract_get_onboarding](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /onboarding` | `ParadexClient::onboarding` | `aux_py.ParadexClient.onboarding` | [contract_onboarding](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /orderbook/{market}` | `ParadexClient::get_orderbook` | `aux_py.ParadexClient.get_orderbook` | [contract_get_orderbook](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /orderbook/{market}/impact-price` | `ParadexClient::get_impact_price` | `aux_py.ParadexClient.get_impact_price` | [contract_get_impact_price](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /orderbook/{market}/interactive` | `ParadexClient::get_orderbook_interactive` | `aux_py.ParadexClient.get_orderbook_interactive` | [contract_get_orderbook_interactive](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /orders` | `ParadexClient::get_open_orders` | `aux_py.ParadexClient.get_open_orders` | [contract_get_open_orders](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /orders` | `ParadexClient::orders_new` | `aux_py.ParadexClient.orders_new` | [contract_orders_new](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /orders` | `ParadexClient::orders_cancel_all` | `aux_py.ParadexClient.orders_cancel_all` | [contract_orders_cancel_all](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /orders-history` | `ParadexClient::get_orders` | `aux_py.ParadexClient.get_orders` | [contract_get_orders](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /orders/batch` | `ParadexClient::orders_new_batch` | `aux_py.ParadexClient.orders_new_batch` | [contract_orders_new_batch](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /orders/batch` | `ParadexClient::orders_cancel_batch` | `aux_py.ParadexClient.orders_cancel_batch` | [contract_orders_cancel_batch](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /orders/by_client_id/{client_id}` | `ParadexClient::orders_get_by_client_id` | `aux_py.ParadexClient.orders_get_by_client_id` | [contract_orders_get_by_client_id](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /orders/by_client_id/{client_id}` | `ParadexClient::orders_cancel_by_client_id` | `aux_py.ParadexClient.orders_cancel_by_client_id` | [contract_orders_cancel_by_client_id](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /orders/{order_id}` | `ParadexClient::orders_get` | `aux_py.ParadexClient.orders_get` | [contract_orders_get](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `PUT /orders/{order_id}` | `ParadexClient::orders_modify` | `aux_py.ParadexClient.orders_modify` | [contract_orders_modify](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /orders/{order_id}` | `ParadexClient::orders_cancel` | `aux_py.ParadexClient.orders_cancel` | [contract_orders_cancel](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /positions` | `ParadexClient::get_positions` | `aux_py.ParadexClient.get_positions` | [contract_get_positions](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /referrals/qr-code` | `ParadexClient::get_referral_qr_code` | `aux_py.ParadexClient.get_referral_qr_code` | [contract_get_referral_qr_code](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /referrals/summary` | `ParadexClient::get_referrals_summary` | `aux_py.ParadexClient.get_referrals_summary` | [contract_get_referrals_summary](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /rfqs` | `ParadexClient::rfq_list` | `aux_py.ParadexClient.rfq_list` | [contract_rfq_list](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /rfqs` | `ParadexClient::rfq_create` | `aux_py.ParadexClient.rfq_create` | [contract_rfq_create](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /rfqs/drafts` | `ParadexClient::rfq_draft_list` | `aux_py.ParadexClient.rfq_draft_list` | [contract_rfq_draft_list](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /rfqs/drafts` | `ParadexClient::rfq_draft_create` | `aux_py.ParadexClient.rfq_draft_create` | [contract_rfq_draft_create](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /rfqs/drafts/{draft_id}` | `ParadexClient::rfq_draft_delete` | `aux_py.ParadexClient.rfq_draft_delete` | [contract_rfq_draft_delete](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /rfqs/markets` | `ParadexClient::rfq_markets` | `aux_py.ParadexClient.rfq_markets` | [contract_rfq_markets](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /rfqs/platform-state` | `ParadexClient::rfq_platform_state` | `aux_py.ParadexClient.rfq_platform_state` | [contract_rfq_platform_state](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `DELETE /rfqs/{rfq_id}` | `ParadexClient::rfq_delete` | `aux_py.ParadexClient.rfq_delete` | [contract_rfq_delete](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /rfqs/{rfq_id}/bbo` | `ParadexClient::rfq_bbo` | `aux_py.ParadexClient.rfq_bbo` | [contract_rfq_bbo](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /rfqs/{rfq_id}/execute` | `ParadexClient::rfq_execute` | `aux_py.ParadexClient.rfq_execute` | [contract_rfq_execute](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /staking/balance/history` | `ParadexClient::get_staking_balance_history` | `aux_py.ParadexClient.get_staking_balance_history` | [contract_get_staking_balance_history](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /staking/balance/history/global` | `ParadexClient::get_global_staking_balance_history` | `aux_py.ParadexClient.get_global_staking_balance_history` | [contract_get_global_staking_balance_history](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /staking/config` | `ParadexClient::get_staking_config` | `aux_py.ParadexClient.get_staking_config` | [contract_get_staking_config](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /staking/history` | `ParadexClient::get_staking_history` | `aux_py.ParadexClient.get_staking_history` | [contract_get_staking_history](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /staking/summary` | `ParadexClient::get_staking_summary` | `aux_py.ParadexClient.get_staking_summary` | [contract_get_staking_summary](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /system/announcements` | `ParadexClient::get_announcements` | `aux_py.ParadexClient.get_announcements` | [contract_get_announcements](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /system/config` | `ParadexClient::get_system_config` | `aux_py.ParadexClient.get_system_config` | [contract_get_system_config](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /system/margin-config` | `ParadexClient::get_margin_config` | `aux_py.ParadexClient.get_margin_config` | [contract_get_margin_config](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /system/portfolio-margin-config` | `ParadexClient::get_portfolio_margin_config` | `aux_py.ParadexClient.get_portfolio_margin_config` | [contract_get_portfolio_margin_config](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /system/state` | `ParadexClient::get_system_state` | `aux_py.ParadexClient.get_system_state` | [contract_get_system_state](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /system/stats` | `ParadexClient::get_system_stats` | `aux_py.ParadexClient.get_system_stats` | [contract_get_system_stats](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /system/time` | `ParadexClient::get_system_time` | `aux_py.ParadexClient.get_system_time` | [contract_get_system_time](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /system/volume-tiers` | `ParadexClient::get_volume_tiers` | `aux_py.ParadexClient.get_volume_tiers` | [contract_get_volume_tiers](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /tradebusts` | `ParadexClient::tradebusts` | `aux_py.ParadexClient.tradebusts` | [contract_tradebusts](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /trades` | `ParadexClient::trades` | `aux_py.ParadexClient.trades` | [contract_trades](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /transactions` | `ParadexClient::transactions` | `aux_py.ParadexClient.transactions` | [contract_transactions](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /transfers` | `ParadexClient::get_transfers` | `aux_py.ParadexClient.get_transfers` | [contract_get_transfers](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /v2/auth` | `ParadexClient::evm_auth` | `aux_py.ParadexClient.evm_auth` | [contract_evm_auth](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /v2/onboarding` | `ParadexClient::evm_onboarding` | `aux_py.ParadexClient.evm_onboarding` | [contract_evm_onboarding](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /vaults` | `ParadexClient::vaults_get` | `aux_py.ParadexClient.vaults_get` | [contract_vaults_get](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /vaults` | `ParadexClient::vaults_create` | `aux_py.ParadexClient.vaults_create` | [contract_vaults_create](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /vaults/account-summary` | `ParadexClient::vaults_get_account_summary` | `aux_py.ParadexClient.vaults_get_account_summary` | [contract_vaults_get_account_summary](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /vaults/analytics` | `ParadexClient::vaults_get_analytics` | `aux_py.ParadexClient.vaults_get_analytics` | [contract_vaults_get_analytics](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /vaults/balance` | `ParadexClient::vaults_get_balance` | `aux_py.ParadexClient.vaults_get_balance` | [contract_vaults_get_balance](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /vaults/config` | `ParadexClient::vaults_get_config` | `aux_py.ParadexClient.vaults_get_config` | [contract_vaults_get_config](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /vaults/history` | `ParadexClient::vaults_get_historical_data` | `aux_py.ParadexClient.vaults_get_historical_data` | [contract_vaults_get_historical_data](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /vaults/mine` | `ParadexClient::vaults_get_mine` | `aux_py.ParadexClient.vaults_get_mine` | [contract_vaults_get_mine](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /vaults/positions` | `ParadexClient::vaults_get_positions` | `aux_py.ParadexClient.vaults_get_positions` | [contract_vaults_get_positions](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /vaults/summary` | `ParadexClient::vaults_get_summary` | `aux_py.ParadexClient.vaults_get_summary` | [contract_vaults_get_summary](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /vaults/transfers` | `ParadexClient::vaults_get_transfers` | `aux_py.ParadexClient.vaults_get_transfers` | [contract_vaults_get_transfers](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /xp/account-balance` | `ParadexClient::get_xp_balance_v2` | `aux_py.ParadexClient.get_xp_balance_v2` | [contract_get_xp_balance_v2](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /xp/fee-config` | `ParadexClient::get_transfer_fee_config_v2` | `aux_py.ParadexClient.get_transfer_fee_config_v2` | [contract_get_transfer_fee_config_v2](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /xp/public-transfers` | `ParadexClient::get_public_xp_transfers_v2` | `aux_py.ParadexClient.get_public_xp_transfers_v2` | [contract_get_public_xp_transfers_v2](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /xp/transfer` | `ParadexClient::create_xp_transfer_v2` | `aux_py.ParadexClient.create_xp_transfer_v2` | [contract_create_xp_transfer_v2](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /xp/transfer/{transfer_id}` | `ParadexClient::get_xp_transfer_by_id_v2` | `aux_py.ParadexClient.get_xp_transfer_by_id_v2` | [contract_get_xp_transfer_by_id_v2](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /xp/transfers` | `ParadexClient::get_xp_transfers_v2` | `aux_py.ParadexClient.get_xp_transfers_v2` | [contract_get_xp_transfers_v2](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /auth/{public_key}` | `ParadexClient::get_subkey_jwt` | `aux_py.ParadexClient.get_subkey_jwt` | [contract_get_subkey_jwt](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `GET /referrals/config` | `ParadexClient::get_referrals_config` | `aux_py.ParadexClient.get_referrals_config` | [contract_get_referrals_config](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |
| `POST /account/profile/max_slippage` | `ParadexClient::update_account_max_slippage` | `aux_py.ParadexClient.update_account_max_slippage` | [contract_update_account_max_slippage](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |

| WebSocket command/channel | Rust API | Python API | Fixture / test |
| --- | --- | --- | --- |
| `account` | `Subscription::Account` | `ParadexClient.subscribe` | [websocket_account](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `auth` | `ParadexClient::open_rpc` | `ParadexClient.open_rpc` | [websocket_auth](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `balance_events` | `Subscription::BalanceEvents` | `ParadexClient.subscribe` | [websocket_balance_events](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `bbo.{market_symbol}` | `Subscription::Bbo` | `ParadexClient.subscribe` | [websocket_bbo_market_symbol](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `fills.{market_symbol}` | `Subscription::Fills` | `ParadexClient.subscribe` | [websocket_fills_market_symbol](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `funding_data.{market_symbol}` | `Subscription::FundingData` | `ParadexClient.subscribe` | [websocket_funding_data_market_symbol](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `funding_payments.{market_symbol}` | `Subscription::FundingPayments` | `ParadexClient.subscribe` | [websocket_funding_payments_market_symbol](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `markets[.{market_symbol}]` | `Subscription::Markets` | `ParadexClient.subscribe` | [websocket_markets_market_symbol](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `markets_summary[.{market_symbol}]` | `Subscription::MarketsSummary` | `ParadexClient.subscribe` | [websocket_markets_summary_market_symbol](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `mmp` | `Subscription::Mmp` | `ParadexClient.subscribe` | [websocket_mmp](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `modify_subscription` | `RpcSession::modify_subscription` | `ParadexRpcSession.modify_subscription` | [websocket_modify_subscription](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `order_book.{market_symbol}.{feed_type}@15@{refresh_rate}[@{price_tick}]` | `Subscription::OrderBook` | `ParadexClient.subscribe` | [websocket_order_book_market_symbol_feed_type_15_refresh_rate_price_tick](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `order.cancel_all` | `RpcSession::cancel_all` | `ParadexRpcSession.cancel_all` | [websocket_order_cancel_all](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `order.cancel_batch` | `RpcSession::cancel_batch` | `ParadexRpcSession.cancel_batch` | [websocket_order_cancel_batch](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `order.cancel_on_disconnect` | `RpcSession::set_cancel_on_disconnect` | `ParadexRpcSession.set_cancel_on_disconnect` | [websocket_order_cancel_on_disconnect](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `order.cancel` | `RpcSession::cancel_order` | `ParadexRpcSession.cancel_order` | [websocket_order_cancel](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `order.create_batch` | `RpcSession::create_batch` | `ParadexRpcSession.create_batch` | [websocket_order_create_batch](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `order.create` | `RpcSession::create_order` | `ParadexRpcSession.create_order` | [websocket_order_create](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `order.modify` | `RpcSession::modify_order` | `ParadexRpcSession.modify_order` | [websocket_order_modify](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `orders.{market_symbol}` | `Subscription::Orders` | `ParadexClient.subscribe` | [websocket_orders_market_symbol](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `positions` | `Subscription::Positions` | `ParadexClient.subscribe` | [websocket_positions](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `rfq[.{rfq_id}]` | `Subscription::Rfq` | `ParadexClient.subscribe` | [websocket_rfq_rfq_id](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `subscribe` | `RpcSession::subscribe_channel` | `ParadexRpcSession.subscribe_channel` | [websocket_subscribe](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `tradebusts` | `Subscription::Tradebusts` | `ParadexClient.subscribe` | [websocket_tradebusts](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `trades.{market_symbol}` | `Subscription::Trades` | `ParadexClient.subscribe` | [websocket_trades_market_symbol](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `transaction` | `Subscription::Transaction` | `ParadexClient.subscribe` | [websocket_transaction](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `transfers` | `Subscription::Transfers` | `ParadexClient.subscribe` | [websocket_transfers](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `unsubscribe` | `RpcSession::unsubscribe_channel` | `ParadexRpcSession.unsubscribe_channel` | [websocket_unsubscribe](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `funding_rate_comparison.ALL@1000ms` | `Subscription::FundingRateComparison` | `ParadexClient.subscribe` | [websocket_funding_rate_comparison](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |
| `block_trades` | `Subscription::BlockTrades` | `ParadexClient.subscribe` | [websocket_block_trades](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |

| SBE template (schema 1:3) | Rust event | Fixture |
| --- | --- | --- |
| 1 | `sbe::TradeEvent` | [frame](tests/fixtures/sbe.json) |
| 2 | `sbe::BboEvent` | [frame](tests/fixtures/sbe.json) |
| 3 | `sbe::BookEvent` | [frame](tests/fixtures/sbe.json) |
| 4 | `sbe::MarketSummaryEvent` | [frame](tests/fixtures/sbe.json) |
| 5 | `sbe::FundingDataEvent` | [frame](tests/fixtures/sbe.json) |
| 6 | `sbe::FundingRateComparisonEvent` | [frame](tests/fixtures/sbe.json) |
| 20 | `sbe::OrderEvent` | [frame](tests/fixtures/sbe.json) |
| 21 | `sbe::FillEvent` | [frame](tests/fixtures/sbe.json) |
| 22 | `sbe::PositionEvent` | [frame](tests/fixtures/sbe.json) |
| 23 | `sbe::AccountEvent` | [frame](tests/fixtures/sbe.json) |
| 40 | `sbe::HeartbeatEvent` | [frame](tests/fixtures/sbe.json) |
| 41 | `sbe::SubscribedEvent` | [frame](tests/fixtures/sbe.json) |
