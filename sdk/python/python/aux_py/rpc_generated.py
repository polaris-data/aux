"""Generated from the native RPC command inventory."""
from . import _RpcSession

class DeriveRpcSession(_RpcSession):
    async def private_burn_vault_shares(self, request=None):
        return await self._command("private_burn_vault_shares", {} if request is None else request)

    async def private_cancel(self, request=None):
        return await self._command("private_cancel", {} if request is None else request)

    async def private_cancel_algo_order(self, request=None):
        return await self._command("private_cancel_algo_order", {} if request is None else request)

    async def private_cancel_all(self, request=None):
        return await self._command("private_cancel_all", {} if request is None else request)

    async def private_cancel_all_algo_orders(self, request=None):
        return await self._command("private_cancel_all_algo_orders", {} if request is None else request)

    async def private_cancel_all_trigger_orders(self, request=None):
        return await self._command("private_cancel_all_trigger_orders", {} if request is None else request)

    async def private_cancel_all_vault_requests(self, request=None):
        return await self._command("private_cancel_all_vault_requests", {} if request is None else request)

    async def private_cancel_batch_quotes(self, request=None):
        return await self._command("private_cancel_batch_quotes", {} if request is None else request)

    async def private_cancel_batch_rfqs(self, request=None):
        return await self._command("private_cancel_batch_rfqs", {} if request is None else request)

    async def private_cancel_by_instrument(self, request=None):
        return await self._command("private_cancel_by_instrument", {} if request is None else request)

    async def private_cancel_by_label(self, request=None):
        return await self._command("private_cancel_by_label", {} if request is None else request)

    async def private_cancel_by_nonce(self, request=None):
        return await self._command("private_cancel_by_nonce", {} if request is None else request)

    async def private_cancel_quote(self, request=None):
        return await self._command("private_cancel_quote", {} if request is None else request)

    async def private_cancel_rfq(self, request=None):
        return await self._command("private_cancel_rfq", {} if request is None else request)

    async def private_cancel_trigger_order(self, request=None):
        return await self._command("private_cancel_trigger_order", {} if request is None else request)

    async def private_change_subaccount_label(self, request=None):
        return await self._command("private_change_subaccount_label", {} if request is None else request)

    async def private_create_vault(self, request=None):
        return await self._command("private_create_vault", {} if request is None else request)

    async def private_delete_subaccount(self, request=None):
        return await self._command("private_delete_subaccount", {} if request is None else request)

    async def private_edit_session_key(self, request=None):
        return await self._command("private_edit_session_key", {} if request is None else request)

    async def private_execute_quote(self, request=None):
        return await self._command("private_execute_quote", {} if request is None else request)

    async def private_force_burn(self, request=None):
        return await self._command("private_force_burn", {} if request is None else request)

    async def private_get_account(self, request=None):
        return await self._command("private_get_account", {} if request is None else request)

    async def private_get_algo_orders(self, request=None):
        return await self._command("private_get_algo_orders", {} if request is None else request)

    async def private_get_all_portfolios(self, request=None):
        return await self._command("private_get_all_portfolios", {} if request is None else request)

    async def private_get_collaterals(self, request=None):
        return await self._command("private_get_collaterals", {} if request is None else request)

    async def private_get_curated_vaults(self, request=None):
        return await self._command("private_get_curated_vaults", {} if request is None else request)

    async def private_get_deposit_history(self, request=None):
        return await self._command("private_get_deposit_history", {} if request is None else request)

    async def private_get_erc20_transfer_history(self, request=None):
        return await self._command("private_get_erc20_transfer_history", {} if request is None else request)

    async def private_get_funding_history(self, request=None):
        return await self._command("private_get_funding_history", {} if request is None else request)

    async def private_get_interest_history(self, request=None):
        return await self._command("private_get_interest_history", {} if request is None else request)

    async def private_get_live_burn_requests(self, request=None):
        return await self._command("private_get_live_burn_requests", {} if request is None else request)

    async def private_get_live_mint_requests(self, request=None):
        return await self._command("private_get_live_mint_requests", {} if request is None else request)

    async def private_get_live_vault_requests(self, request=None):
        return await self._command("private_get_live_vault_requests", {} if request is None else request)

    async def private_get_maker_rfq_history(self, request=None):
        return await self._command("private_get_maker_rfq_history", {} if request is None else request)

    async def private_get_margin(self, request=None):
        return await self._command("private_get_margin", {} if request is None else request)

    async def private_get_mmp_config(self, request=None):
        return await self._command("private_get_mmp_config", {} if request is None else request)

    async def private_get_open_orders(self, request=None):
        return await self._command("private_get_open_orders", {} if request is None else request)

    async def private_get_option_settlement_history(self, request=None):
        return await self._command("private_get_option_settlement_history", {} if request is None else request)

    async def private_get_order(self, request=None):
        return await self._command("private_get_order", {} if request is None else request)

    async def private_get_order_history(self, request=None):
        return await self._command("private_get_order_history", {} if request is None else request)

    async def private_get_positions(self, request=None):
        return await self._command("private_get_positions", {} if request is None else request)

    async def private_get_quotes(self, request=None):
        return await self._command("private_get_quotes", {} if request is None else request)

    async def private_get_rfqs(self, request=None):
        return await self._command("private_get_rfqs", {} if request is None else request)

    async def private_get_shareholder_vaults(self, request=None):
        return await self._command("private_get_shareholder_vaults", {} if request is None else request)

    async def private_get_subaccount(self, request=None):
        return await self._command("private_get_subaccount", {} if request is None else request)

    async def private_get_subaccount_value_history(self, request=None):
        return await self._command("private_get_subaccount_value_history", {} if request is None else request)

    async def private_get_subaccounts(self, request=None):
        return await self._command("private_get_subaccounts", {} if request is None else request)

    async def private_get_trade_history(self, request=None):
        return await self._command("private_get_trade_history", {} if request is None else request)

    async def private_get_trigger_orders(self, request=None):
        return await self._command("private_get_trigger_orders", {} if request is None else request)

    async def private_get_vault_request_history(self, request=None):
        return await self._command("private_get_vault_request_history", {} if request is None else request)

    async def private_get_vault_shares(self, request=None):
        return await self._command("private_get_vault_shares", {} if request is None else request)

    async def private_get_withdraw_addresses(self, request=None):
        return await self._command("private_get_withdraw_addresses", {} if request is None else request)

    async def private_get_withdrawal_history(self, request=None):
        return await self._command("private_get_withdrawal_history", {} if request is None else request)

    async def private_liquidate(self, request=None):
        return await self._command("private_liquidate", {} if request is None else request)

    async def private_mint_vault_shares(self, request=None):
        return await self._command("private_mint_vault_shares", {} if request is None else request)

    async def private_order(self, request=None):
        return await self._command("private_order", {} if request is None else request)

    async def private_order_debug(self, request=None):
        return await self._command("private_order_debug", {} if request is None else request)

    async def private_order_quote(self, request=None):
        return await self._command("private_order_quote", {} if request is None else request)

    async def private_poll_quotes(self, request=None):
        return await self._command("private_poll_quotes", {} if request is None else request)

    async def private_poll_rfqs(self, request=None):
        return await self._command("private_poll_rfqs", {} if request is None else request)

    async def private_register_bridge_deposit(self, request=None):
        return await self._command("private_register_bridge_deposit", {} if request is None else request)

    async def private_register_withdraw_address(self, request=None):
        return await self._command("private_register_withdraw_address", {} if request is None else request)

    async def private_reject_deposit_request(self, request=None):
        return await self._command("private_reject_deposit_request", {} if request is None else request)

    async def private_replace(self, request=None):
        return await self._command("private_replace", {} if request is None else request)

    async def private_replace_quote(self, request=None):
        return await self._command("private_replace_quote", {} if request is None else request)

    async def private_request_vault_deposit(self, request=None):
        return await self._command("private_request_vault_deposit", {} if request is None else request)

    async def private_request_vault_withdraw(self, request=None):
        return await self._command("private_request_vault_withdraw", {} if request is None else request)

    async def private_reset_mmp(self, request=None):
        return await self._command("private_reset_mmp", {} if request is None else request)

    async def private_rfq_get_best_quote(self, request=None):
        return await self._command("private_rfq_get_best_quote", {} if request is None else request)

    async def private_send_quote(self, request=None):
        return await self._command("private_send_quote", {} if request is None else request)

    async def private_send_rfq(self, request=None):
        return await self._command("private_send_rfq", {} if request is None else request)

    async def private_session_keys(self, request=None):
        return await self._command("private_session_keys", {} if request is None else request)

    async def private_set_mmp_config(self, request=None):
        return await self._command("private_set_mmp_config", {} if request is None else request)

    async def private_set_session_key(self, request=None):
        return await self._command("private_set_session_key", {} if request is None else request)

    async def private_set_session_key_debug(self, request=None):
        return await self._command("private_set_session_key_debug", {} if request is None else request)

    async def private_transfer_positions(self, request=None):
        return await self._command("private_transfer_positions", {} if request is None else request)

    async def private_transfer_positions_debug(self, request=None):
        return await self._command("private_transfer_positions_debug", {} if request is None else request)

    async def private_transfer_spot(self, request=None):
        return await self._command("private_transfer_spot", {} if request is None else request)

    async def private_transfer_spot_debug(self, request=None):
        return await self._command("private_transfer_spot_debug", {} if request is None else request)

    async def private_transfer_spot_external(self, request=None):
        return await self._command("private_transfer_spot_external", {} if request is None else request)

    async def private_transfer_spot_external_debug(self, request=None):
        return await self._command("private_transfer_spot_external_debug", {} if request is None else request)

    async def private_update_vault_info(self, request=None):
        return await self._command("private_update_vault_info", {} if request is None else request)

    async def private_update_whitelisted_recipients(self, request=None):
        return await self._command("private_update_whitelisted_recipients", {} if request is None else request)

    async def private_update_whitelisted_recipients_debug(self, request=None):
        return await self._command("private_update_whitelisted_recipients_debug", {} if request is None else request)

    async def private_withdraw(self, request=None):
        return await self._command("private_withdraw", {} if request is None else request)

    async def public_decode_action(self, request=None):
        return await self._command("public_decode_action", {} if request is None else request)

    async def public_execute_quote_debug(self, request=None):
        return await self._command("public_execute_quote_debug", {} if request is None else request)

    async def public_get_all_currencies(self, request=None):
        return await self._command("public_get_all_currencies", {} if request is None else request)

    async def public_get_all_instruments(self, request=None):
        return await self._command("public_get_all_instruments", {} if request is None else request)

    async def public_get_all_live_instruments(self, request=None):
        return await self._command("public_get_all_live_instruments", {} if request is None else request)

    async def public_get_all_referral_codes(self, request=None):
        return await self._command("public_get_all_referral_codes", {} if request is None else request)

    async def public_get_currency(self, request=None):
        return await self._command("public_get_currency", {} if request is None else request)

    async def public_get_funding_rate_history(self, request=None):
        return await self._command("public_get_funding_rate_history", {} if request is None else request)

    async def public_get_index_chart_data(self, request=None):
        return await self._command("public_get_index_chart_data", {} if request is None else request)

    async def public_get_instrument(self, request=None):
        return await self._command("public_get_instrument", {} if request is None else request)

    async def public_get_interest_rate_history(self, request=None):
        return await self._command("public_get_interest_rate_history", {} if request is None else request)

    async def public_get_latest_signed_feeds(self, request=None):
        return await self._command("public_get_latest_signed_feeds", {} if request is None else request)

    async def public_get_liquidation_history(self, request=None):
        return await self._command("public_get_liquidation_history", {} if request is None else request)

    async def public_get_live_auctions(self, request=None):
        return await self._command("public_get_live_auctions", {} if request is None else request)

    async def public_get_live_incidents(self, request=None):
        return await self._command("public_get_live_incidents", {} if request is None else request)

    async def public_get_maker_program_scores(self, request=None):
        return await self._command("public_get_maker_program_scores", {} if request is None else request)

    async def public_get_maker_programs(self, request=None):
        return await self._command("public_get_maker_programs", {} if request is None else request)

    async def public_get_margin(self, request=None):
        return await self._command("public_get_margin", {} if request is None else request)

    async def public_get_onchain_action_history(self, request=None):
        return await self._command("public_get_onchain_action_history", {} if request is None else request)

    async def public_get_option_settlement_prices(self, request=None):
        return await self._command("public_get_option_settlement_prices", {} if request is None else request)

    async def public_get_pending_deposits(self, request=None):
        return await self._command("public_get_pending_deposits", {} if request is None else request)

    async def public_get_perp_impact_twap(self, request=None):
        return await self._command("public_get_perp_impact_twap", {} if request is None else request)

    async def public_get_rate_limits(self, request=None):
        return await self._command("public_get_rate_limits", {} if request is None else request)

    async def public_get_referral_performance(self, request=None):
        return await self._command("public_get_referral_performance", {} if request is None else request)

    async def public_get_risk_universes(self, request=None):
        return await self._command("public_get_risk_universes", {} if request is None else request)

    async def public_get_subaccounts(self, request=None):
        return await self._command("public_get_subaccounts", {} if request is None else request)

    async def public_get_ticker(self, request=None):
        return await self._command("public_get_ticker", {} if request is None else request)

    async def public_get_tickers(self, request=None):
        return await self._command("public_get_tickers", {} if request is None else request)

    async def public_get_time(self, request=None):
        return await self._command("public_get_time", {} if request is None else request)

    async def public_get_trade_history(self, request=None):
        return await self._command("public_get_trade_history", {} if request is None else request)

    async def public_get_tradingview_chart_data(self, request=None):
        return await self._command("public_get_tradingview_chart_data", {} if request is None else request)

    async def public_get_transaction(self, request=None):
        return await self._command("public_get_transaction", {} if request is None else request)

    async def public_get_vault(self, request=None):
        return await self._command("public_get_vault", {} if request is None else request)

    async def public_get_vault_action_history(self, request=None):
        return await self._command("public_get_vault_action_history", {} if request is None else request)

    async def public_get_vault_performance_history(self, request=None):
        return await self._command("public_get_vault_performance_history", {} if request is None else request)

    async def public_get_vaults(self, request=None):
        return await self._command("public_get_vaults", {} if request is None else request)

    async def public_get_wallet(self, request=None):
        return await self._command("public_get_wallet", {} if request is None else request)

    async def public_get_wallets_from_session_key(self, request=None):
        return await self._command("public_get_wallets_from_session_key", {} if request is None else request)

    async def public_margin_watch(self, request=None):
        return await self._command("public_margin_watch", {} if request is None else request)

    async def public_order_quote(self, request=None):
        return await self._command("public_order_quote", {} if request is None else request)

    async def public_register_deposit_address(self, request=None):
        return await self._command("public_register_deposit_address", {} if request is None else request)

    async def public_send_quote_debug(self, request=None):
        return await self._command("public_send_quote_debug", {} if request is None else request)

    async def public_withdraw_debug(self, request=None):
        return await self._command("public_withdraw_debug", {} if request is None else request)

    async def set_cancel_on_disconnect(self, request=None):
        return await self._command("set_cancel_on_disconnect", {} if request is None else request)

    async def subscribe_channels(self, request=None):
        return await self._command("subscribe_channels", {} if request is None else request)

    async def unsubscribe_channels(self, request=None):
        return await self._command("unsubscribe_channels", {} if request is None else request)

class PolymarketPerpsRpcSession(_RpcSession):
    async def create_orders(self, request):
        return await self._command("create_orders", request)

    async def modify_orders(self, request):
        return await self._command("modify_orders", request)

    async def modify_orders_coid(self, request):
        return await self._command("modify_orders_coid", request)

    async def cancel_orders(self, request):
        return await self._command("cancel_orders", request)

    async def cancel_orders_coid(self, request):
        return await self._command("cancel_orders_coid", request)

    async def cancel_all(self, request):
        return await self._command("cancel_all", request)

    async def auto_cancel(self, request):
        return await self._command("auto_cancel", request)

    async def update_leverage(self, request):
        return await self._command("update_leverage", request)

    async def update_leverages(self, request):
        return await self._command("update_leverages", request)

    async def update_margin(self, request):
        return await self._command("update_margin", request)

    async def subscribe(self, channels):
        return await self._command("subscribe", channels)

    async def unsubscribe(self, channels):
        return await self._command("unsubscribe", channels)

    async def ping(self):
        return await self._command("ping")

class PolymarketRfqRpcSession(_RpcSession):
    async def quote(self, request):
        return await self._command("quote", request)

    async def cancel_quote(self, request):
        return await self._command("cancel_quote", request)

    async def last_look(self, request):
        return await self._command("last_look", request)

class AevoRpcSession(_RpcSession):
    async def place_order(self, request):
        return await self._command("place_order", request)

    async def amend_order(self, request):
        return await self._command("amend_order", request)

    async def cancel_order(self, request):
        return await self._command("cancel_order", request)

    async def cancel_all_orders(self, request):
        return await self._command("cancel_all_orders", request)

class BinanceSpotRpcSession(_RpcSession):
    async def place_order(self, request):
        return await self._command("place_order", request)

    async def amend_order(self, request):
        return await self._command("amend_order", request)

    async def cancel_order(self, request):
        return await self._command("cancel_order", request)

    async def cancel_replace_order(self, request):
        return await self._command("cancel_replace_order", request)

    async def cancel_all_orders(self, request):
        return await self._command("cancel_all_orders", request)

    async def place_oco(self, request):
        return await self._command("place_oco", request)

    async def place_oto(self, request):
        return await self._command("place_oto", request)

    async def place_otoco(self, request):
        return await self._command("place_otoco", request)

    async def place_opo(self, request):
        return await self._command("place_opo", request)

    async def place_opoco(self, request):
        return await self._command("place_opoco", request)

    async def cancel_order_list(self, request):
        return await self._command("cancel_order_list", request)

    async def order_status(self, request):
        return await self._command("order_status", request)

    async def open_orders(self, request):
        return await self._command("open_orders", request)

    async def order_history(self, request):
        return await self._command("order_history", request)

    async def order_list_status(self, request):
        return await self._command("order_list_status", request)

    async def open_order_lists(self, request):
        return await self._command("open_order_lists", request)

    async def fills(self, request):
        return await self._command("fills", request)

class BybitRpcSession(_RpcSession):
    async def place_order(self, request):
        return await self._command("place_order", request)

    async def amend_order(self, request):
        return await self._command("amend_order", request)

    async def cancel_order(self, request):
        return await self._command("cancel_order", request)

    async def place_orders(self, request):
        return await self._command("place_orders", request)

    async def amend_orders(self, request):
        return await self._command("amend_orders", request)

    async def cancel_orders(self, request):
        return await self._command("cancel_orders", request)

class DeribitRpcSession(_RpcSession):
    async def buy(self, request):
        return await self._command("buy", request)

    async def sell(self, request):
        return await self._command("sell", request)

    async def amend_order(self, request):
        return await self._command("amend_order", request)

    async def amend_by_label(self, request):
        return await self._command("amend_by_label", request)

    async def cancel_order(self, request):
        return await self._command("cancel_order", request)

    async def cancel_by_label(self, request):
        return await self._command("cancel_by_label", request)

    async def cancel_all_orders(self, request):
        return await self._command("cancel_all_orders", request)

    async def cancel_instrument_orders(self, request):
        return await self._command("cancel_instrument_orders", request)

    async def cancel_currency_orders(self, request):
        return await self._command("cancel_currency_orders", request)

    async def cancel_kind_orders(self, request):
        return await self._command("cancel_kind_orders", request)

    async def cancel_currency_pair_orders(self, request):
        return await self._command("cancel_currency_pair_orders", request)

    async def mass_quote(self, request):
        return await self._command("mass_quote", request)

    async def cancel_quotes(self, request):
        return await self._command("cancel_quotes", request)

    async def order_status(self, request):
        return await self._command("order_status", request)

    async def open_orders(self, request):
        return await self._command("open_orders", request)

class HibachiRpcSession(_RpcSession):
    async def place_order(self, request):
        return await self._command("place_order", request)

    async def amend_order(self, request):
        return await self._command("amend_order", request)

    async def cancel_order(self, request):
        return await self._command("cancel_order", request)

    async def order_status(self, request):
        return await self._command("order_status", request)

    async def open_orders(self, request):
        return await self._command("open_orders", request)

    async def cancel_all_orders(self, request):
        return await self._command("cancel_all_orders", request)

    async def place_orders(self, request):
        return await self._command("place_orders", request)

class KrakenSpotRpcSession(_RpcSession):
    async def place_order(self, request):
        return await self._command("place_order", request)

    async def amend_order(self, request):
        return await self._command("amend_order", request)

    async def cancel_order(self, request):
        return await self._command("cancel_order", request)

    async def cancel_all_orders(self, request):
        return await self._command("cancel_all_orders", request)

    async def place_orders(self, request):
        return await self._command("place_orders", request)

    async def cancel_orders(self, request):
        return await self._command("cancel_orders", request)

class OkxRpcSession(_RpcSession):
    async def place_order(self, request):
        return await self._command("place_order", request)

    async def amend_order(self, request):
        return await self._command("amend_order", request)

    async def cancel_order(self, request):
        return await self._command("cancel_order", request)

    async def place_orders(self, request):
        return await self._command("place_orders", request)

    async def amend_orders(self, request):
        return await self._command("amend_orders", request)

    async def cancel_orders(self, request):
        return await self._command("cancel_orders", request)

EXCHANGE_SESSIONS = {
    'aevo': AevoRpcSession,
    'binance_spot': BinanceSpotRpcSession,
    'bybit': BybitRpcSession,
    'deribit': DeribitRpcSession,
    'hibachi': HibachiRpcSession,
    'kraken_spot': KrakenSpotRpcSession,
    'okx': OkxRpcSession,
}
