"""Pascal REST wire models generated from schema/pascal.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class AccountBioParams(TypedDict):
    owner: str

_AccountBioResponseOptional = TypedDict("_AccountBioResponseOptional", {'status': 'str | None', 'data': 'AccountBioResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class AccountBioResponse(_AccountBioResponseOptional):
    pass

_AccountBioResponseDataOptional = TypedDict("_AccountBioResponseDataOptional", {'account_bio': 'str | None'}, total=False)
class AccountBioResponseData(_AccountBioResponseDataOptional):
    pass

_AccountStateSnapshotParamsOptional = TypedDict("_AccountStateSnapshotParamsOptional", {'symbol': 'str | None', 'exclude_orders': 'bool | None'}, total=False)
class AccountStateSnapshotParams(_AccountStateSnapshotParamsOptional):
    owner: str

_AccountStateSnapshotResponseOptional = TypedDict("_AccountStateSnapshotResponseOptional", {'status': 'str | None', 'data': 'AccountStateSnapshotResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class AccountStateSnapshotResponse(_AccountStateSnapshotResponseOptional):
    pass

_AccountStateSnapshotResponseDataOptional = TypedDict("_AccountStateSnapshotResponseDataOptional", {'has_deposited': 'bool | None', 'username': 'str | None', 'twitter_account_connection': 'AccountStateSnapshotResponseDataTwitterAccountConnection | None', 'collateral_usd': 'str | None', 'rewards': 'AccountStateSnapshotResponseDataRewards | None', 'lifetime_stats': 'AccountStateSnapshotResponseDataLifetimeStats | None', 'orders': 'list[AccountStateSnapshotResponseDataOrdersItem] | None', 'positions': 'list[AccountStateSnapshotResponseDataPositionsItem] | None', 'pending_deposits': 'list[AccountStateSnapshotResponseDataPendingDepositsItem] | None', 'pending_withdrawals': 'list[AccountStateSnapshotResponseDataPendingWithdrawalsItem] | None'}, total=False)
class AccountStateSnapshotResponseData(_AccountStateSnapshotResponseDataOptional):
    pass

_AccountStateSnapshotResponseDataLifetimeStatsOptional = TypedDict("_AccountStateSnapshotResponseDataLifetimeStatsOptional", {'maker': 'AccountStateSnapshotResponseDataLifetimeStatsMaker | None', 'taker': 'AccountStateSnapshotResponseDataLifetimeStatsTaker | None'}, total=False)
class AccountStateSnapshotResponseDataLifetimeStats(_AccountStateSnapshotResponseDataLifetimeStatsOptional):
    pass

_AccountStateSnapshotResponseDataLifetimeStatsMakerOptional = TypedDict("_AccountStateSnapshotResponseDataLifetimeStatsMakerOptional", {'volume_contracts': 'str | None', 'weighted_volume': 'str | None', 'volume_notional_usd': 'str | None', 'fees_paid_usd': 'str | None'}, total=False)
class AccountStateSnapshotResponseDataLifetimeStatsMaker(_AccountStateSnapshotResponseDataLifetimeStatsMakerOptional):
    pass

_AccountStateSnapshotResponseDataLifetimeStatsTakerOptional = TypedDict("_AccountStateSnapshotResponseDataLifetimeStatsTakerOptional", {'volume_contracts': 'str | None', 'weighted_volume': 'str | None', 'volume_notional_usd': 'str | None', 'fees_paid_usd': 'str | None'}, total=False)
class AccountStateSnapshotResponseDataLifetimeStatsTaker(_AccountStateSnapshotResponseDataLifetimeStatsTakerOptional):
    pass

_AccountStateSnapshotResponseDataOrdersItemOptional = TypedDict("_AccountStateSnapshotResponseDataOrdersItemOptional", {'id': 'str | None', 'client_order_id': 'str | None', 'symbol': 'str | None', 'side': 'str | None', 'price': 'str | None', 'size_original': 'str | None', 'size_remaining': 'str | None', 'size_filled': 'str | None', 'notional_filled': 'str | None', 'tif': 'str | None', 'type': 'str | None', 'post_only': 'bool | None', 'reduce_only': 'bool | None', 'seq': 'str | None', 'update_seq': 'str | None', 'place_ts_ms': 'str | None', 'update_ts_ms': 'str | None'}, total=False)
class AccountStateSnapshotResponseDataOrdersItem(_AccountStateSnapshotResponseDataOrdersItemOptional):
    pass

_AccountStateSnapshotResponseDataPendingDepositsItemOptional = TypedDict("_AccountStateSnapshotResponseDataPendingDepositsItemOptional", {'deposit_id': 'str | None', 'owner': 'str | None', 'amount': 'str | None', 'transfer_status': 'str | None'}, total=False)
class AccountStateSnapshotResponseDataPendingDepositsItem(_AccountStateSnapshotResponseDataPendingDepositsItemOptional):
    pass

_AccountStateSnapshotResponseDataPendingWithdrawalsItemOptional = TypedDict("_AccountStateSnapshotResponseDataPendingWithdrawalsItemOptional", {'owner': 'str | None', 'status': 'str | None', 'amount': 'str | None', 'destination_authority': 'str | None'}, total=False)
class AccountStateSnapshotResponseDataPendingWithdrawalsItem(_AccountStateSnapshotResponseDataPendingWithdrawalsItemOptional):
    pass

_AccountStateSnapshotResponseDataPositionsItemOptional = TypedDict("_AccountStateSnapshotResponseDataPositionsItemOptional", {'symbol': 'str | None', 'size': 'str | None', 'cumulative_increasing_contracts': 'str | None', 'cumulative_increasing_notional_usd': 'str | None', 'cumulative_reducing_contracts': 'str | None', 'cumulative_reducing_notional_usd': 'str | None', 'cumulative_fees_usd': 'str | None', 'realized_pnl_usd': 'str | None', 'unrealized_pnl_usd': 'str | None', 'mark_price': 'str | None', 'average_entry_price': 'str | None', 'seq': 'str | None', 'update_seq': 'str | None', 'open_ts_ms': 'str | None', 'update_ts_ms': 'str | None'}, total=False)
class AccountStateSnapshotResponseDataPositionsItem(_AccountStateSnapshotResponseDataPositionsItemOptional):
    pass

_AccountStateSnapshotResponseDataRewardsOptional = TypedDict("_AccountStateSnapshotResponseDataRewardsOptional", {'referral_rewards_received_usd': 'str | None', 'builder_fees_received_usd': 'str | None', 'user_rewards_received_usd': 'str | None'}, total=False)
class AccountStateSnapshotResponseDataRewards(_AccountStateSnapshotResponseDataRewardsOptional):
    pass

_AccountStateSnapshotResponseDataTwitterAccountConnectionOptional = TypedDict("_AccountStateSnapshotResponseDataTwitterAccountConnectionOptional", {'twitter_user_id': 'str | None', 'twitter_handle': 'str | None', 'connect_seq': 'str | None'}, total=False)
class AccountStateSnapshotResponseDataTwitterAccountConnection(_AccountStateSnapshotResponseDataTwitterAccountConnectionOptional):
    pass

class ApplyAReferralCodeParams(TypedDict):
    body: ApplyAReferralCodeParamsBody

_ApplyAReferralCodeParamsBodyOptional = TypedDict("_ApplyAReferralCodeParamsBodyOptional", {'auth': 'dict[str, Any] | None'}, total=False)
class ApplyAReferralCodeParamsBody(_ApplyAReferralCodeParamsBodyOptional):
    referral_code: str

_ApplyAReferralCodeResponseOptional = TypedDict("_ApplyAReferralCodeResponseOptional", {'status': 'str | None', 'data': 'dict[str, Any] | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class ApplyAReferralCodeResponse(_ApplyAReferralCodeResponseOptional):
    pass

_BatchMarkPriceCandlesParamsOptional = TypedDict("_BatchMarkPriceCandlesParamsOptional", {'start_time_ms': 'int | Decimal | None', 'count_back': 'int | Decimal | None'}, total=False)
class BatchMarkPriceCandlesParams(_BatchMarkPriceCandlesParamsOptional):
    symbols: str
    interval: str
    end_time_ms: int | Decimal

_BatchMarkPriceCandlesResponseOptional = TypedDict("_BatchMarkPriceCandlesResponseOptional", {'status': 'str | None', 'data': 'BatchMarkPriceCandlesResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class BatchMarkPriceCandlesResponse(_BatchMarkPriceCandlesResponseOptional):
    pass

_BatchMarkPriceCandlesResponseDataOptional = TypedDict("_BatchMarkPriceCandlesResponseDataOptional", {'items': 'list[BatchMarkPriceCandlesResponseDataItemsItem] | None'}, total=False)
class BatchMarkPriceCandlesResponseData(_BatchMarkPriceCandlesResponseDataOptional):
    pass

_BatchMarkPriceCandlesResponseDataItemsItemOptional = TypedDict("_BatchMarkPriceCandlesResponseDataItemsItemOptional", {'symbol': 'str | None', 'candles': 'list[BatchMarkPriceCandlesResponseDataItemsItemCandlesItem] | None'}, total=False)
class BatchMarkPriceCandlesResponseDataItemsItem(_BatchMarkPriceCandlesResponseDataItemsItemOptional):
    pass

_BatchMarkPriceCandlesResponseDataItemsItemCandlesItemOptional = TypedDict("_BatchMarkPriceCandlesResponseDataItemsItemCandlesItemOptional", {'time': 'str | None', 'open': 'str | None', 'high': 'str | None', 'low': 'str | None', 'close': 'str | None'}, total=False)
class BatchMarkPriceCandlesResponseDataItemsItemCandlesItem(_BatchMarkPriceCandlesResponseDataItemsItemCandlesItemOptional):
    pass

_BatchTradesParamsOptional = TypedDict("_BatchTradesParamsOptional", {'limit': 'int | Decimal | None'}, total=False)
class BatchTradesParams(_BatchTradesParamsOptional):
    symbols: str

_BatchTradesResponseOptional = TypedDict("_BatchTradesResponseOptional", {'status': 'str | None', 'data': 'BatchTradesResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class BatchTradesResponse(_BatchTradesResponseOptional):
    pass

_BatchTradesResponseDataOptional = TypedDict("_BatchTradesResponseDataOptional", {'items': 'list[BatchTradesResponseDataItemsItem] | None'}, total=False)
class BatchTradesResponseData(_BatchTradesResponseDataOptional):
    pass

_BatchTradesResponseDataItemsItemOptional = TypedDict("_BatchTradesResponseDataItemsItemOptional", {'symbol': 'str | None', 'trades': 'list[BatchTradesResponseDataItemsItemTradesItem] | None'}, total=False)
class BatchTradesResponseDataItemsItem(_BatchTradesResponseDataItemsItemOptional):
    pass

_BatchTradesResponseDataItemsItemTradesItemOptional = TypedDict("_BatchTradesResponseDataItemsItemTradesItemOptional", {'trade_id': 'str | None', 'symbol': 'str | None', 'taker_side': 'str | None', 'price': 'str | None', 'size': 'str | None', 'maker': 'str | None', 'taker': 'str | None', 'maker_reduce_only': 'bool | None', 'taker_reduce_only': 'bool | None', 'maker_position_size_prev': 'str | None', 'taker_position_size_prev': 'str | None', 'seq': 'str | None', 'trade_ts_ms': 'str | None'}, total=False)
class BatchTradesResponseDataItemsItemTradesItem(_BatchTradesResponseDataItemsItemTradesItemOptional):
    pass

class BuilderStatsParams(TypedDict):
    owner: str

_BuilderStatsResponseOptional = TypedDict("_BuilderStatsResponseOptional", {'status': 'str | None', 'data': 'BuilderStatsResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class BuilderStatsResponse(_BuilderStatsResponseOptional):
    pass

_BuilderStatsResponseDataOptional = TypedDict("_BuilderStatsResponseDataOptional", {'builder_volume_contracts': 'str | None', 'builder_volume_notional_usd': 'str | None', 'builder_fee_base_usd': 'str | None', 'builder_fees_received_usd': 'str | None'}, total=False)
class BuilderStatsResponseData(_BuilderStatsResponseDataOptional):
    pass

class CancelOrdersParams(TypedDict):
    body: list[CancelOrdersParamsBodyItem]

_CancelOrdersParamsBodyItemOptional = TypedDict("_CancelOrdersParamsBodyItemOptional", {'auth': 'dict[str, Any] | None'}, total=False)
class CancelOrdersParamsBodyItem(_CancelOrdersParamsBodyItemOptional):
    client_order_id: str
    order_id: str

_CancelOrdersResponseItemOptional = TypedDict("_CancelOrdersResponseItemOptional", {'status': 'str | None', 'data': 'CancelOrdersResponseItemData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class CancelOrdersResponseItem(_CancelOrdersResponseItemOptional):
    pass

_CancelOrdersResponseItemDataOptional = TypedDict("_CancelOrdersResponseItemDataOptional", {'id': 'str | None', 'client_order_id': 'str | None', 'symbol': 'str | None', 'side': 'str | None', 'price': 'str | None', 'size_original': 'str | None', 'size_remaining': 'str | None', 'size_filled': 'str | None', 'notional_filled': 'str | None', 'tif': 'str | None', 'type': 'str | None', 'post_only': 'bool | None', 'reduce_only': 'bool | None', 'seq': 'str | None', 'update_seq': 'str | None', 'place_ts_ms': 'str | None', 'update_ts_ms': 'str | None'}, total=False)
class CancelOrdersResponseItemData(_CancelOrdersResponseItemDataOptional):
    pass

_CancelOrdersResponseValueItemOptional = TypedDict("_CancelOrdersResponseValueItemOptional", {'status': 'str | None', 'data': 'CancelOrdersResponseValueItemData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class CancelOrdersResponseValueItem(_CancelOrdersResponseValueItemOptional):
    pass

_CancelOrdersResponseValueItemDataOptional = TypedDict("_CancelOrdersResponseValueItemDataOptional", {'id': 'str | None', 'client_order_id': 'str | None', 'symbol': 'str | None', 'side': 'str | None', 'price': 'str | None', 'size_original': 'str | None', 'size_remaining': 'str | None', 'size_filled': 'str | None', 'notional_filled': 'str | None', 'tif': 'str | None', 'type': 'str | None', 'post_only': 'bool | None', 'reduce_only': 'bool | None', 'seq': 'str | None', 'update_seq': 'str | None', 'place_ts_ms': 'str | None', 'update_ts_ms': 'str | None'}, total=False)
class CancelOrdersResponseValueItemData(_CancelOrdersResponseValueItemDataOptional):
    pass

class ClaimACustomReferralCodeParams(TypedDict):
    body: ClaimACustomReferralCodeParamsBody

_ClaimACustomReferralCodeParamsBodyOptional = TypedDict("_ClaimACustomReferralCodeParamsBodyOptional", {'auth': 'dict[str, Any] | None'}, total=False)
class ClaimACustomReferralCodeParamsBody(_ClaimACustomReferralCodeParamsBodyOptional):
    referral_code: str

_ClaimACustomReferralCodeResponseOptional = TypedDict("_ClaimACustomReferralCodeResponseOptional", {'status': 'str | None', 'data': 'ClaimACustomReferralCodeResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class ClaimACustomReferralCodeResponse(_ClaimACustomReferralCodeResponseOptional):
    pass

_ClaimACustomReferralCodeResponseDataOptional = TypedDict("_ClaimACustomReferralCodeResponseDataOptional", {'custom_referral_code': 'str | None'}, total=False)
class ClaimACustomReferralCodeResponseData(_ClaimACustomReferralCodeResponseDataOptional):
    pass

_ClosedPositionHistoryParamsOptional = TypedDict("_ClosedPositionHistoryParamsOptional", {'symbols': 'str | None', 'order_by': 'str | None', 'sort_order': 'str | None', 'before_cursor': 'str | None', 'limit': 'int | Decimal | None'}, total=False)
class ClosedPositionHistoryParams(_ClosedPositionHistoryParamsOptional):
    owner: str

_ClosedPositionHistoryResponseOptional = TypedDict("_ClosedPositionHistoryResponseOptional", {'status': 'str | None', 'data': 'ClosedPositionHistoryResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class ClosedPositionHistoryResponse(_ClosedPositionHistoryResponseOptional):
    pass

_ClosedPositionHistoryResponseDataOptional = TypedDict("_ClosedPositionHistoryResponseDataOptional", {'items': 'list[ClosedPositionHistoryResponseDataItemsItem] | None', 'next_cursor': 'str | None'}, total=False)
class ClosedPositionHistoryResponseData(_ClosedPositionHistoryResponseDataOptional):
    pass

_ClosedPositionHistoryResponseDataItemsItemOptional = TypedDict("_ClosedPositionHistoryResponseDataItemsItemOptional", {'symbol': 'str | None', 'position_side': 'str | None', 'position_instance_count': 'str | None', 'held_until_resolution': 'bool | None', 'realized_pnl_usd': 'str | None', 'cumulative_increasing_contracts': 'str | None', 'cumulative_increasing_notional_usd': 'str | None', 'cumulative_reducing_notional_usd': 'str | None', 'average_increasing_price': 'str | None', 'average_reducing_price': 'str | None', 'open_seq': 'str | None', 'open_ts_ms': 'str | None', 'close_seq': 'str | None', 'close_ts_ms': 'str | None'}, total=False)
class ClosedPositionHistoryResponseDataItemsItem(_ClosedPositionHistoryResponseDataItemsItemOptional):
    pass

class CreateATradingKeyParams(TypedDict):
    body: CreateATradingKeyParamsBody

_CreateATradingKeyParamsBodyOptional = TypedDict("_CreateATradingKeyParamsBodyOptional", {'auth': 'dict[str, Any] | None'}, total=False)
class CreateATradingKeyParamsBody(_CreateATradingKeyParamsBodyOptional):
    trading_key: str
    name: str
    expiration_ts_ms: str

_CreateATradingKeyResponseOptional = TypedDict("_CreateATradingKeyResponseOptional", {'status': 'str | None', 'data': 'CreateATradingKeyResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class CreateATradingKeyResponse(_CreateATradingKeyResponseOptional):
    pass

_CreateATradingKeyResponseDataOptional = TypedDict("_CreateATradingKeyResponseDataOptional", {'trading_key': 'str | None', 'name': 'str | None', 'expiration_ts_ms': 'str | None', 'open_order_count': 'str | None'}, total=False)
class CreateATradingKeyResponseData(_CreateATradingKeyResponseDataOptional):
    pass

class DepositAddressParams(TypedDict):
    owner: str

_DepositAddressResponseOptional = TypedDict("_DepositAddressResponseOptional", {'status': 'str | None', 'data': 'DepositAddressResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class DepositAddressResponse(_DepositAddressResponseOptional):
    pass

_DepositAddressResponseDataOptional = TypedDict("_DepositAddressResponseDataOptional", {'owner': 'str | None', 'deposit_address': 'str | None', 'registered': 'bool | None'}, total=False)
class DepositAddressResponseData(_DepositAddressResponseDataOptional):
    pass

_FillHistoryParamsOptional = TypedDict("_FillHistoryParamsOptional", {'symbols': 'str | None', 'before_cursor': 'str | None', 'at_or_before_seq': 'str | None', 'limit': 'int | Decimal | None'}, total=False)
class FillHistoryParams(_FillHistoryParamsOptional):
    owner: str

_FillHistoryResponseOptional = TypedDict("_FillHistoryResponseOptional", {'status': 'str | None', 'data': 'FillHistoryResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class FillHistoryResponse(_FillHistoryResponseOptional):
    pass

_FillHistoryResponseDataOptional = TypedDict("_FillHistoryResponseDataOptional", {'items': 'list[FillHistoryResponseDataItemsItem] | None'}, total=False)
class FillHistoryResponseData(_FillHistoryResponseDataOptional):
    pass

_FillHistoryResponseDataItemsItemOptional = TypedDict("_FillHistoryResponseDataItemsItemOptional", {'trade_id': 'str | None', 'symbol': 'str | None', 'side': 'str | None', 'reduce_only': 'bool | None', 'fill_price': 'str | None', 'fill_size': 'str | None', 'liquidity': 'str | None', 'counterparty': 'str | None', 'order_id': 'str | None', 'client_order_id': 'str | None', 'order_progress': 'FillHistoryResponseDataItemsItemOrderProgress | None', 'fee_usd': 'str | None', 'collateral_change_usd': 'str | None', 'position_size_prev': 'str | None', 'realized_pnl_usd': 'str | None', 'position_open_seq': 'str | None', 'position_open_ts_ms': 'str | None', 'seq': 'str | None', 'trade_ts_ms': 'str | None'}, total=False)
class FillHistoryResponseDataItemsItem(_FillHistoryResponseDataItemsItemOptional):
    pass

_FillHistoryResponseDataItemsItemOrderProgressOptional = TypedDict("_FillHistoryResponseDataItemsItemOrderProgressOptional", {'size_original': 'str | None', 'size_filled': 'str | None', 'size_remaining': 'str | None'}, total=False)
class FillHistoryResponseDataItemsItemOrderProgress(_FillHistoryResponseDataItemsItemOrderProgressOptional):
    pass

class GenerateAReferralCodeParams(TypedDict):
    body: GenerateAReferralCodeParamsBody

_GenerateAReferralCodeParamsBodyOptional = TypedDict("_GenerateAReferralCodeParamsBodyOptional", {'auth': 'dict[str, Any] | None'}, total=False)
class GenerateAReferralCodeParamsBody(_GenerateAReferralCodeParamsBodyOptional):
    pass

_GenerateAReferralCodeResponseOptional = TypedDict("_GenerateAReferralCodeResponseOptional", {'status': 'str | None', 'data': 'GenerateAReferralCodeResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class GenerateAReferralCodeResponse(_GenerateAReferralCodeResponseOptional):
    pass

_GenerateAReferralCodeResponseDataOptional = TypedDict("_GenerateAReferralCodeResponseDataOptional", {'generated_referral_code': 'str | None'}, total=False)
class GenerateAReferralCodeResponseData(_GenerateAReferralCodeResponseDataOptional):
    pass

class ListMarketsParams(TypedDict):
    pass

_ListMarketsResponseOptional = TypedDict("_ListMarketsResponseOptional", {'status': 'str | None', 'data': 'list[ListMarketsResponseDataItem] | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class ListMarketsResponse(_ListMarketsResponseOptional):
    pass

_ListMarketsResponseDataItemOptional = TypedDict("_ListMarketsResponseDataItemOptional", {'symbol': 'str | None', 'taker_fee_rate': 'str | None', 'maker_rebate_share': 'str | None', 'tick_size_min': 'str | None', 'tick_sig_figs': 'int | Decimal | None', 'is_mece': 'bool | None', 'display_attributes': 'ListMarketsResponseDataItemDisplayAttributes | None', 'listing_ts_ms': 'str | None', 'rules_url': 'str | None', 'stats': 'ListMarketsResponseDataItemStats | None', 'mark_price': 'str | None', 'best_bid': 'str | None', 'best_ask': 'str | None', 'open_interest': 'str | None', 'long_interest': 'str | None', 'short_interest': 'str | None'}, total=False)
class ListMarketsResponseDataItem(_ListMarketsResponseDataItemOptional):
    pass

_ListMarketsResponseDataItemDisplayAttributesOptional = TypedDict("_ListMarketsResponseDataItemDisplayAttributesOptional", {'event_description': 'str | None', 'expected_resolution_time_ms': 'str | None', 'market_description': 'str | None', 'reverse_description': 'str | None', 'sort_order': 'str | None', 'tags': 'list[str] | None', 'topic': 'ListMarketsResponseDataItemDisplayAttributesTopic | None'}, total=False)
class ListMarketsResponseDataItemDisplayAttributes(_ListMarketsResponseDataItemDisplayAttributesOptional):
    pass

_ListMarketsResponseDataItemDisplayAttributesTopicOptional = TypedDict("_ListMarketsResponseDataItemDisplayAttributesTopicOptional", {'id': 'str | None', 'description': 'str | None'}, total=False)
class ListMarketsResponseDataItemDisplayAttributesTopic(_ListMarketsResponseDataItemDisplayAttributesTopicOptional):
    pass

_ListMarketsResponseDataItemStatsOptional = TypedDict("_ListMarketsResponseDataItemStatsOptional", {'last_24h': 'ListMarketsResponseDataItemStatsLast24h | None', 'all_time': 'ListMarketsResponseDataItemStatsAllTime | None'}, total=False)
class ListMarketsResponseDataItemStats(_ListMarketsResponseDataItemStatsOptional):
    pass

_ListMarketsResponseDataItemStatsAllTimeOptional = TypedDict("_ListMarketsResponseDataItemStatsAllTimeOptional", {'taker_buy_volume': 'str | None', 'taker_sell_volume': 'str | None', 'taker_buy_volume_notional': 'str | None', 'taker_sell_volume_notional': 'str | None', 'taker_buy_trade_count': 'str | None', 'taker_sell_trade_count': 'str | None', 'window_end_ts_ms_exclusive': 'str | None'}, total=False)
class ListMarketsResponseDataItemStatsAllTime(_ListMarketsResponseDataItemStatsAllTimeOptional):
    pass

_ListMarketsResponseDataItemStatsLast24hOptional = TypedDict("_ListMarketsResponseDataItemStatsLast24hOptional", {'mark_price_start': 'str | None', 'taker_buy_volume': 'str | None', 'taker_sell_volume': 'str | None', 'taker_buy_volume_notional': 'str | None', 'taker_sell_volume_notional': 'str | None', 'taker_buy_trade_count': 'str | None', 'taker_sell_trade_count': 'str | None', 'window_end_ts_ms_exclusive': 'str | None'}, total=False)
class ListMarketsResponseDataItemStatsLast24h(_ListMarketsResponseDataItemStatsLast24hOptional):
    pass

_MarkPriceCandlesParamsOptional = TypedDict("_MarkPriceCandlesParamsOptional", {'start_time_ms': 'int | Decimal | None', 'count_back': 'int | Decimal | None'}, total=False)
class MarkPriceCandlesParams(_MarkPriceCandlesParamsOptional):
    symbol: str
    interval: str
    end_time_ms: int | Decimal

_MarkPriceCandlesResponseOptional = TypedDict("_MarkPriceCandlesResponseOptional", {'status': 'str | None', 'data': 'list[MarkPriceCandlesResponseDataItem] | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class MarkPriceCandlesResponse(_MarkPriceCandlesResponseOptional):
    pass

_MarkPriceCandlesResponseDataItemOptional = TypedDict("_MarkPriceCandlesResponseDataItemOptional", {'time': 'str | None', 'open': 'str | None', 'high': 'str | None', 'low': 'str | None', 'close': 'str | None'}, total=False)
class MarkPriceCandlesResponseDataItem(_MarkPriceCandlesResponseDataItemOptional):
    pass

_MarketPnLMoversParamsOptional = TypedDict("_MarketPnLMoversParamsOptional", {'limit': 'int | Decimal | None', 'symbols': 'str | None', 'order_by': 'str | None', 'sort_order': 'str | None'}, total=False)
class MarketPnLMoversParams(_MarketPnLMoversParamsOptional):
    window: str
    owner: str

_MarketPnLMoversResponseOptional = TypedDict("_MarketPnLMoversResponseOptional", {'status': 'str | None', 'data': 'MarketPnLMoversResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class MarketPnLMoversResponse(_MarketPnLMoversResponseOptional):
    pass

_MarketPnLMoversResponseDataOptional = TypedDict("_MarketPnLMoversResponseDataOptional", {'owner': 'str | None', 'window': 'str | None', 'start_ms': 'str | None', 'as_of_ms': 'str | None', 'total_markets': 'str | None', 'total_pnl_change_usd': 'str | None', 'entries': 'list[MarketPnLMoversResponseDataEntriesItem] | None'}, total=False)
class MarketPnLMoversResponseData(_MarketPnLMoversResponseDataOptional):
    pass

_MarketPnLMoversResponseDataEntriesItemOptional = TypedDict("_MarketPnLMoversResponseDataEntriesItemOptional", {'market_id': 'str | None', 'symbol': 'str | None', 'pnl_change_usd': 'str | None', 'mark_price': 'str | None', 'mark_price_start': 'str | None', 'position_size_start': 'str | None', 'position_size': 'str | None', 'volume_contracts': 'str | None', 'resolved': 'bool | None'}, total=False)
class MarketPnLMoversResponseDataEntriesItem(_MarketPnLMoversResponseDataEntriesItemOptional):
    pass

class MarketSearchParams(TypedDict):
    pass

_MarketSearchResponseOptional = TypedDict("_MarketSearchResponseOptional", {'status': 'str | None', 'data': 'list[MarketSearchResponseDataItem] | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class MarketSearchResponse(_MarketSearchResponseOptional):
    pass

_MarketSearchResponseDataItemOptional = TypedDict("_MarketSearchResponseDataItemOptional", {'symbol': 'str | None', 'taker_fee_rate': 'str | None', 'maker_rebate_share': 'str | None', 'tick_size_min': 'str | None', 'tick_sig_figs': 'int | Decimal | None', 'is_mece': 'bool | None', 'display_attributes': 'MarketSearchResponseDataItemDisplayAttributes | None', 'listing_ts_ms': 'str | None', 'rules_url': 'str | None', 'stats': 'MarketSearchResponseDataItemStats | None', 'mark_price': 'str | None', 'best_bid': 'str | None', 'best_ask': 'str | None', 'open_interest': 'str | None', 'long_interest': 'str | None', 'short_interest': 'str | None'}, total=False)
class MarketSearchResponseDataItem(_MarketSearchResponseDataItemOptional):
    pass

_MarketSearchResponseDataItemDisplayAttributesOptional = TypedDict("_MarketSearchResponseDataItemDisplayAttributesOptional", {'event_description': 'str | None', 'expected_resolution_time_ms': 'str | None', 'market_description': 'str | None', 'reverse_description': 'str | None', 'sort_order': 'str | None', 'tags': 'list[str] | None', 'topic': 'MarketSearchResponseDataItemDisplayAttributesTopic | None'}, total=False)
class MarketSearchResponseDataItemDisplayAttributes(_MarketSearchResponseDataItemDisplayAttributesOptional):
    pass

_MarketSearchResponseDataItemDisplayAttributesTopicOptional = TypedDict("_MarketSearchResponseDataItemDisplayAttributesTopicOptional", {'id': 'str | None', 'description': 'str | None'}, total=False)
class MarketSearchResponseDataItemDisplayAttributesTopic(_MarketSearchResponseDataItemDisplayAttributesTopicOptional):
    pass

_MarketSearchResponseDataItemStatsOptional = TypedDict("_MarketSearchResponseDataItemStatsOptional", {'last_24h': 'MarketSearchResponseDataItemStatsLast24h | None', 'all_time': 'MarketSearchResponseDataItemStatsAllTime | None'}, total=False)
class MarketSearchResponseDataItemStats(_MarketSearchResponseDataItemStatsOptional):
    pass

_MarketSearchResponseDataItemStatsAllTimeOptional = TypedDict("_MarketSearchResponseDataItemStatsAllTimeOptional", {'taker_buy_volume': 'str | None', 'taker_sell_volume': 'str | None', 'taker_buy_volume_notional': 'str | None', 'taker_sell_volume_notional': 'str | None', 'taker_buy_trade_count': 'str | None', 'taker_sell_trade_count': 'str | None', 'window_end_ts_ms_exclusive': 'str | None'}, total=False)
class MarketSearchResponseDataItemStatsAllTime(_MarketSearchResponseDataItemStatsAllTimeOptional):
    pass

_MarketSearchResponseDataItemStatsLast24hOptional = TypedDict("_MarketSearchResponseDataItemStatsLast24hOptional", {'mark_price_start': 'str | None', 'taker_buy_volume': 'str | None', 'taker_sell_volume': 'str | None', 'taker_buy_volume_notional': 'str | None', 'taker_sell_volume_notional': 'str | None', 'taker_buy_trade_count': 'str | None', 'taker_sell_trade_count': 'str | None', 'window_end_ts_ms_exclusive': 'str | None'}, total=False)
class MarketSearchResponseDataItemStatsLast24h(_MarketSearchResponseDataItemStatsLast24hOptional):
    pass

class MarketTextSearchParams(TypedDict):
    pass

_MarketTextSearchResponseOptional = TypedDict("_MarketTextSearchResponseOptional", {'status': 'str | None', 'data': 'list[MarketTextSearchResponseDataItem] | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class MarketTextSearchResponse(_MarketTextSearchResponseOptional):
    pass

_MarketTextSearchResponseDataItemOptional = TypedDict("_MarketTextSearchResponseDataItemOptional", {'symbol': 'str | None', 'taker_fee_rate': 'str | None', 'maker_rebate_share': 'str | None', 'tick_size_min': 'str | None', 'tick_sig_figs': 'int | Decimal | None', 'is_mece': 'bool | None', 'display_attributes': 'MarketTextSearchResponseDataItemDisplayAttributes | None', 'listing_ts_ms': 'str | None', 'rules_url': 'str | None', 'stats': 'MarketTextSearchResponseDataItemStats | None', 'mark_price': 'str | None', 'best_bid': 'str | None', 'best_ask': 'str | None', 'open_interest': 'str | None', 'long_interest': 'str | None', 'short_interest': 'str | None'}, total=False)
class MarketTextSearchResponseDataItem(_MarketTextSearchResponseDataItemOptional):
    pass

_MarketTextSearchResponseDataItemDisplayAttributesOptional = TypedDict("_MarketTextSearchResponseDataItemDisplayAttributesOptional", {'event_description': 'str | None', 'expected_resolution_time_ms': 'str | None', 'market_description': 'str | None', 'reverse_description': 'str | None', 'sort_order': 'str | None', 'tags': 'list[str] | None', 'topic': 'MarketTextSearchResponseDataItemDisplayAttributesTopic | None'}, total=False)
class MarketTextSearchResponseDataItemDisplayAttributes(_MarketTextSearchResponseDataItemDisplayAttributesOptional):
    pass

_MarketTextSearchResponseDataItemDisplayAttributesTopicOptional = TypedDict("_MarketTextSearchResponseDataItemDisplayAttributesTopicOptional", {'id': 'str | None', 'description': 'str | None'}, total=False)
class MarketTextSearchResponseDataItemDisplayAttributesTopic(_MarketTextSearchResponseDataItemDisplayAttributesTopicOptional):
    pass

_MarketTextSearchResponseDataItemStatsOptional = TypedDict("_MarketTextSearchResponseDataItemStatsOptional", {'last_24h': 'MarketTextSearchResponseDataItemStatsLast24h | None', 'all_time': 'MarketTextSearchResponseDataItemStatsAllTime | None'}, total=False)
class MarketTextSearchResponseDataItemStats(_MarketTextSearchResponseDataItemStatsOptional):
    pass

_MarketTextSearchResponseDataItemStatsAllTimeOptional = TypedDict("_MarketTextSearchResponseDataItemStatsAllTimeOptional", {'taker_buy_volume': 'str | None', 'taker_sell_volume': 'str | None', 'taker_buy_volume_notional': 'str | None', 'taker_sell_volume_notional': 'str | None', 'taker_buy_trade_count': 'str | None', 'taker_sell_trade_count': 'str | None', 'window_end_ts_ms_exclusive': 'str | None'}, total=False)
class MarketTextSearchResponseDataItemStatsAllTime(_MarketTextSearchResponseDataItemStatsAllTimeOptional):
    pass

_MarketTextSearchResponseDataItemStatsLast24hOptional = TypedDict("_MarketTextSearchResponseDataItemStatsLast24hOptional", {'mark_price_start': 'str | None', 'taker_buy_volume': 'str | None', 'taker_sell_volume': 'str | None', 'taker_buy_volume_notional': 'str | None', 'taker_sell_volume_notional': 'str | None', 'taker_buy_trade_count': 'str | None', 'taker_sell_trade_count': 'str | None', 'window_end_ts_ms_exclusive': 'str | None'}, total=False)
class MarketTextSearchResponseDataItemStatsLast24h(_MarketTextSearchResponseDataItemStatsLast24hOptional):
    pass

class MassCancelOrdersParams(TypedDict):
    body: list[MassCancelOrdersParamsBodyItem]

_MassCancelOrdersParamsBodyItemOptional = TypedDict("_MassCancelOrdersParamsBodyItemOptional", {'auth': 'dict[str, Any] | None'}, total=False)
class MassCancelOrdersParamsBodyItem(_MassCancelOrdersParamsBodyItemOptional):
    symbol: str

_MassCancelOrdersResponseItemOptional = TypedDict("_MassCancelOrdersResponseItemOptional", {'status': 'str | None', 'data': 'MassCancelOrdersResponseItemData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class MassCancelOrdersResponseItem(_MassCancelOrdersResponseItemOptional):
    pass

_MassCancelOrdersResponseItemDataOptional = TypedDict("_MassCancelOrdersResponseItemDataOptional", {'orders': 'list[MassCancelOrdersResponseItemDataOrdersItem] | None'}, total=False)
class MassCancelOrdersResponseItemData(_MassCancelOrdersResponseItemDataOptional):
    pass

_MassCancelOrdersResponseItemDataOrdersItemOptional = TypedDict("_MassCancelOrdersResponseItemDataOrdersItemOptional", {'id': 'str | None', 'client_order_id': 'str | None', 'symbol': 'str | None', 'side': 'str | None', 'price': 'str | None', 'size_original': 'str | None', 'size_remaining': 'str | None', 'size_filled': 'str | None', 'notional_filled': 'str | None', 'tif': 'str | None', 'type': 'str | None', 'post_only': 'bool | None', 'reduce_only': 'bool | None', 'seq': 'str | None', 'update_seq': 'str | None', 'place_ts_ms': 'str | None', 'update_ts_ms': 'str | None'}, total=False)
class MassCancelOrdersResponseItemDataOrdersItem(_MassCancelOrdersResponseItemDataOrdersItemOptional):
    pass

_MassCancelOrdersResponseValueItemOptional = TypedDict("_MassCancelOrdersResponseValueItemOptional", {'status': 'str | None', 'data': 'MassCancelOrdersResponseValueItemData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class MassCancelOrdersResponseValueItem(_MassCancelOrdersResponseValueItemOptional):
    pass

_MassCancelOrdersResponseValueItemDataOptional = TypedDict("_MassCancelOrdersResponseValueItemDataOptional", {'orders': 'list[MassCancelOrdersResponseValueItemDataOrdersItem] | None'}, total=False)
class MassCancelOrdersResponseValueItemData(_MassCancelOrdersResponseValueItemDataOptional):
    pass

_MassCancelOrdersResponseValueItemDataOrdersItemOptional = TypedDict("_MassCancelOrdersResponseValueItemDataOrdersItemOptional", {'id': 'str | None', 'client_order_id': 'str | None', 'symbol': 'str | None', 'side': 'str | None', 'price': 'str | None', 'size_original': 'str | None', 'size_remaining': 'str | None', 'size_filled': 'str | None', 'notional_filled': 'str | None', 'tif': 'str | None', 'type': 'str | None', 'post_only': 'bool | None', 'reduce_only': 'bool | None', 'seq': 'str | None', 'update_seq': 'str | None', 'place_ts_ms': 'str | None', 'update_ts_ms': 'str | None'}, total=False)
class MassCancelOrdersResponseValueItemDataOrdersItem(_MassCancelOrdersResponseValueItemDataOrdersItemOptional):
    pass

class OrderBooksParams(TypedDict):
    symbols: str

_OrderBooksResponseOptional = TypedDict("_OrderBooksResponseOptional", {'status': 'str | None', 'data': 'OrderBooksResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class OrderBooksResponse(_OrderBooksResponseOptional):
    pass

_OrderBooksResponseDataOptional = TypedDict("_OrderBooksResponseDataOptional", {'books': 'OrderBooksResponseDataBooks | None'}, total=False)
class OrderBooksResponseData(_OrderBooksResponseDataOptional):
    pass

_OrderBooksResponseDataBooksOptional = TypedDict("_OrderBooksResponseDataBooksOptional", {'SIM_EVENT_1.MARKET_1': 'OrderBooksResponseDataBooksSIMEVENT1MARKET1 | None', 'SIM_EVENT_1.MARKET_2': 'OrderBooksResponseDataBooksSIMEVENT1MARKET2 | None'}, total=False)
class OrderBooksResponseDataBooks(_OrderBooksResponseDataBooksOptional):
    pass

_OrderBooksResponseDataBooksSIMEVENT1MARKET1Optional = TypedDict("_OrderBooksResponseDataBooksSIMEVENT1MARKET1Optional", {'asks': 'list[list[str]] | None', 'bids': 'list[list[str]] | None', 'spec': 'OrderBooksResponseDataBooksSIMEVENT1MARKET1Spec | None'}, total=False)
class OrderBooksResponseDataBooksSIMEVENT1MARKET1(_OrderBooksResponseDataBooksSIMEVENT1MARKET1Optional):
    pass

_OrderBooksResponseDataBooksSIMEVENT1MARKET1SpecOptional = TypedDict("_OrderBooksResponseDataBooksSIMEVENT1MARKET1SpecOptional", {'symbol': 'str | None', 'taker_fee_rate': 'str | None', 'maker_rebate_share': 'str | None', 'tick_size_min': 'str | None', 'tick_sig_figs': 'int | Decimal | None'}, total=False)
class OrderBooksResponseDataBooksSIMEVENT1MARKET1Spec(_OrderBooksResponseDataBooksSIMEVENT1MARKET1SpecOptional):
    pass

_OrderBooksResponseDataBooksSIMEVENT1MARKET2Optional = TypedDict("_OrderBooksResponseDataBooksSIMEVENT1MARKET2Optional", {'asks': 'list[list[str]] | None', 'bids': 'list[list[str]] | None'}, total=False)
class OrderBooksResponseDataBooksSIMEVENT1MARKET2(_OrderBooksResponseDataBooksSIMEVENT1MARKET2Optional):
    pass

class PlaceAndReplaceOrdersParams(TypedDict):
    body: list[PlaceAndReplaceOrdersParamsBodyItem]

_PlaceAndReplaceOrdersParamsBodyItemOptional = TypedDict("_PlaceAndReplaceOrdersParamsBodyItemOptional", {'replace_client_order_id': 'str | None', 'tif': 'str | None', 'post_only': 'bool | None', 'reduce_only': 'bool | None', 'allow_missing_replace': 'bool | None', 'replace_with_open_size': 'bool | None', 'expires_ts_ms': 'str | None', 'builder_info': 'str | None', 'auth': 'dict[str, Any] | None'}, total=False)
class PlaceAndReplaceOrdersParamsBodyItem(_PlaceAndReplaceOrdersParamsBodyItemOptional):
    client_order_id: str
    symbol: str
    side: str
    price: str
    size: str

_PlaceAndReplaceOrdersResponseItemOptional = TypedDict("_PlaceAndReplaceOrdersResponseItemOptional", {'status': 'str | None', 'data': 'PlaceAndReplaceOrdersResponseItemData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class PlaceAndReplaceOrdersResponseItem(_PlaceAndReplaceOrdersResponseItemOptional):
    pass

_PlaceAndReplaceOrdersResponseItemDataOptional = TypedDict("_PlaceAndReplaceOrdersResponseItemDataOptional", {'order': 'PlaceAndReplaceOrdersResponseItemDataOrder | None', 'fills': 'list[PlaceAndReplaceOrdersResponseItemDataFillsItem] | None'}, total=False)
class PlaceAndReplaceOrdersResponseItemData(_PlaceAndReplaceOrdersResponseItemDataOptional):
    pass

_PlaceAndReplaceOrdersResponseItemDataFillsItemOptional = TypedDict("_PlaceAndReplaceOrdersResponseItemDataFillsItemOptional", {'trade_id': 'str | None', 'symbol': 'str | None', 'side': 'str | None', 'reduce_only': 'bool | None', 'fill_price': 'str | None', 'fill_size': 'str | None', 'liquidity': 'str | None', 'counterparty': 'str | None', 'order_id': 'str | None', 'client_order_id': 'str | None', 'order_progress': 'PlaceAndReplaceOrdersResponseItemDataFillsItemOrderProgress | None', 'fee_usd': 'str | None', 'collateral_change_usd': 'str | None', 'position_size_prev': 'str | None', 'realized_pnl_usd': 'str | None', 'seq': 'str | None', 'trade_ts_ms': 'str | None'}, total=False)
class PlaceAndReplaceOrdersResponseItemDataFillsItem(_PlaceAndReplaceOrdersResponseItemDataFillsItemOptional):
    pass

_PlaceAndReplaceOrdersResponseItemDataFillsItemOrderProgressOptional = TypedDict("_PlaceAndReplaceOrdersResponseItemDataFillsItemOrderProgressOptional", {'size_original': 'str | None', 'size_filled': 'str | None', 'size_remaining': 'str | None'}, total=False)
class PlaceAndReplaceOrdersResponseItemDataFillsItemOrderProgress(_PlaceAndReplaceOrdersResponseItemDataFillsItemOrderProgressOptional):
    pass

_PlaceAndReplaceOrdersResponseItemDataOrderOptional = TypedDict("_PlaceAndReplaceOrdersResponseItemDataOrderOptional", {'id': 'str | None', 'client_order_id': 'str | None', 'symbol': 'str | None', 'side': 'str | None', 'price': 'str | None', 'size_original': 'str | None', 'size_remaining': 'str | None', 'size_filled': 'str | None', 'notional_filled': 'str | None', 'tif': 'str | None', 'type': 'str | None', 'post_only': 'bool | None', 'reduce_only': 'bool | None', 'seq': 'str | None', 'update_seq': 'str | None', 'place_ts_ms': 'str | None', 'update_ts_ms': 'str | None'}, total=False)
class PlaceAndReplaceOrdersResponseItemDataOrder(_PlaceAndReplaceOrdersResponseItemDataOrderOptional):
    pass

_PlaceAndReplaceOrdersResponseValueItemOptional = TypedDict("_PlaceAndReplaceOrdersResponseValueItemOptional", {'status': 'str | None', 'data': 'PlaceAndReplaceOrdersResponseValueItemData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class PlaceAndReplaceOrdersResponseValueItem(_PlaceAndReplaceOrdersResponseValueItemOptional):
    pass

_PlaceAndReplaceOrdersResponseValueItemDataOptional = TypedDict("_PlaceAndReplaceOrdersResponseValueItemDataOptional", {'order': 'PlaceAndReplaceOrdersResponseValueItemDataOrder | None', 'fills': 'list[PlaceAndReplaceOrdersResponseValueItemDataFillsItem] | None'}, total=False)
class PlaceAndReplaceOrdersResponseValueItemData(_PlaceAndReplaceOrdersResponseValueItemDataOptional):
    pass

_PlaceAndReplaceOrdersResponseValueItemDataFillsItemOptional = TypedDict("_PlaceAndReplaceOrdersResponseValueItemDataFillsItemOptional", {'trade_id': 'str | None', 'symbol': 'str | None', 'side': 'str | None', 'reduce_only': 'bool | None', 'fill_price': 'str | None', 'fill_size': 'str | None', 'liquidity': 'str | None', 'counterparty': 'str | None', 'order_id': 'str | None', 'client_order_id': 'str | None', 'order_progress': 'PlaceAndReplaceOrdersResponseValueItemDataFillsItemOrderProgress | None', 'fee_usd': 'str | None', 'collateral_change_usd': 'str | None', 'position_size_prev': 'str | None', 'realized_pnl_usd': 'str | None', 'seq': 'str | None', 'trade_ts_ms': 'str | None'}, total=False)
class PlaceAndReplaceOrdersResponseValueItemDataFillsItem(_PlaceAndReplaceOrdersResponseValueItemDataFillsItemOptional):
    pass

_PlaceAndReplaceOrdersResponseValueItemDataFillsItemOrderProgressOptional = TypedDict("_PlaceAndReplaceOrdersResponseValueItemDataFillsItemOrderProgressOptional", {'size_original': 'str | None', 'size_filled': 'str | None', 'size_remaining': 'str | None'}, total=False)
class PlaceAndReplaceOrdersResponseValueItemDataFillsItemOrderProgress(_PlaceAndReplaceOrdersResponseValueItemDataFillsItemOrderProgressOptional):
    pass

_PlaceAndReplaceOrdersResponseValueItemDataOrderOptional = TypedDict("_PlaceAndReplaceOrdersResponseValueItemDataOrderOptional", {'id': 'str | None', 'client_order_id': 'str | None', 'symbol': 'str | None', 'side': 'str | None', 'price': 'str | None', 'size_original': 'str | None', 'size_remaining': 'str | None', 'size_filled': 'str | None', 'notional_filled': 'str | None', 'tif': 'str | None', 'type': 'str | None', 'post_only': 'bool | None', 'reduce_only': 'bool | None', 'seq': 'str | None', 'update_seq': 'str | None', 'place_ts_ms': 'str | None', 'update_ts_ms': 'str | None'}, total=False)
class PlaceAndReplaceOrdersResponseValueItemDataOrder(_PlaceAndReplaceOrdersResponseValueItemDataOrderOptional):
    pass

class PnLHistoryParams(TypedDict):
    owner: str

_PnLHistoryResponseOptional = TypedDict("_PnLHistoryResponseOptional", {'status': 'str | None', 'data': 'PnLHistoryResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class PnLHistoryResponse(_PnLHistoryResponseOptional):
    pass

_PnLHistoryResponseDataOptional = TypedDict("_PnLHistoryResponseDataOptional", {'items': 'list[PnLHistoryResponseDataItemsItem] | None'}, total=False)
class PnLHistoryResponseData(_PnLHistoryResponseDataOptional):
    pass

_PnLHistoryResponseDataItemsItemOptional = TypedDict("_PnLHistoryResponseDataItemsItemOptional", {'timeframe': 'str | None', 'data': 'PnLHistoryResponseDataItemsItemData | None'}, total=False)
class PnLHistoryResponseDataItemsItem(_PnLHistoryResponseDataItemsItemOptional):
    pass

_PnLHistoryResponseDataItemsItemDataOptional = TypedDict("_PnLHistoryResponseDataItemsItemDataOptional", {'account_value_history': 'list[list[str]] | None', 'total_pnl_history': 'list[list[str]] | None', 'trading_pnl_history': 'list[list[str]] | None', 'referral_rewards_received_history': 'list[list[str]] | None', 'builder_fees_received_history': 'list[list[str]] | None', 'user_rewards_received_history': 'list[list[str]] | None'}, total=False)
class PnLHistoryResponseDataItemsItemData(_PnLHistoryResponseDataItemsItemDataOptional):
    pass

_PositionResolutionHistoryParamsOptional = TypedDict("_PositionResolutionHistoryParamsOptional", {'symbols': 'str | None', 'before_cursor': 'str | None', 'at_or_before_seq': 'str | None', 'limit': 'int | Decimal | None'}, total=False)
class PositionResolutionHistoryParams(_PositionResolutionHistoryParamsOptional):
    owner: str

_PositionResolutionHistoryResponseOptional = TypedDict("_PositionResolutionHistoryResponseOptional", {'status': 'str | None', 'data': 'PositionResolutionHistoryResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class PositionResolutionHistoryResponse(_PositionResolutionHistoryResponseOptional):
    pass

_PositionResolutionHistoryResponseDataOptional = TypedDict("_PositionResolutionHistoryResponseDataOptional", {'items': 'list[PositionResolutionHistoryResponseDataItemsItem] | None'}, total=False)
class PositionResolutionHistoryResponseData(_PositionResolutionHistoryResponseDataOptional):
    pass

_PositionResolutionHistoryResponseDataItemsItemOptional = TypedDict("_PositionResolutionHistoryResponseDataItemsItemOptional", {'symbol': 'str | None', 'resolution': 'str | None', 'size': 'str | None', 'collateral_change_usd': 'str | None', 'realized_pnl_usd': 'str | None', 'position_open_seq': 'str | None', 'position_open_ts_ms': 'str | None', 'seq': 'str | None', 'position_resolution_ts_ms': 'str | None'}, total=False)
class PositionResolutionHistoryResponseDataItemsItem(_PositionResolutionHistoryResponseDataItemsItemOptional):
    pass

class ReferralCodeParams(TypedDict):
    referral_code: str

_ReferralCodeResponseOptional = TypedDict("_ReferralCodeResponseOptional", {'status': 'str | None', 'data': 'ReferralCodeResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class ReferralCodeResponse(_ReferralCodeResponseOptional):
    pass

_ReferralCodeResponseDataOptional = TypedDict("_ReferralCodeResponseDataOptional", {'exists': 'bool | None', 'taker_fee_discount_share': 'str | None', 'fee_savings_limit_usd': 'str | None'}, total=False)
class ReferralCodeResponseData(_ReferralCodeResponseDataOptional):
    pass

class ReferralsParams(TypedDict):
    owner: str

_ReferralsResponseOptional = TypedDict("_ReferralsResponseOptional", {'status': 'str | None', 'data': 'ReferralsResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class ReferralsResponse(_ReferralsResponseOptional):
    pass

_ReferralsResponseDataOptional = TypedDict("_ReferralsResponseDataOptional", {'referral_reward_terms': 'ReferralsResponseDataReferralRewardTerms | None', 'reward_paid_total_usd': 'str | None', 'referral_count_total': 'int | Decimal | None', 'referrals': 'list[ReferralsResponseDataReferralsItem] | None', 'referrer': 'ReferralsResponseDataReferrer | None', 'generated_referral_code': 'str | None', 'custom_referral_code': 'str | None', 'custom_referral_code_revoked': 'bool | None', 'can_generate_referral_code': 'bool | None', 'can_claim_custom_referral_code': 'bool | None', 'can_apply_referral_code': 'bool | None'}, total=False)
class ReferralsResponseData(_ReferralsResponseDataOptional):
    pass

_ReferralsResponseDataReferralRewardTermsOptional = TypedDict("_ReferralsResponseDataReferralRewardTermsOptional", {'referral_reward_share': 'str | None', 'per_referred_owner_reward_paid_limit_usd': 'str | None'}, total=False)
class ReferralsResponseDataReferralRewardTerms(_ReferralsResponseDataReferralRewardTermsOptional):
    pass

_ReferralsResponseDataReferralsItemOptional = TypedDict("_ReferralsResponseDataReferralsItemOptional", {'seq': 'str | None', 'referred_owner': 'str | None', 'created_ts_ms': 'str | None', 'referral_reward_share': 'str | None', 'reward_paid_usd': 'str | None', 'reward_paid_limit_usd': 'str | None'}, total=False)
class ReferralsResponseDataReferralsItem(_ReferralsResponseDataReferralsItemOptional):
    pass

_ReferralsResponseDataReferrerOptional = TypedDict("_ReferralsResponseDataReferrerOptional", {'owner': 'str | None', 'username': 'str | None', 'referral_code': 'str | None'}, total=False)
class ReferralsResponseDataReferrer(_ReferralsResponseDataReferrerOptional):
    pass

class RegisterADepositAddressParams(TypedDict):
    body: RegisterADepositAddressParamsBody

_RegisterADepositAddressParamsBodyOptional = TypedDict("_RegisterADepositAddressParamsBodyOptional", {'referral_code': 'str | None', 'auth': 'dict[str, Any] | None'}, total=False)
class RegisterADepositAddressParamsBody(_RegisterADepositAddressParamsBodyOptional):
    owner: str

_RegisterADepositAddressResponseOptional = TypedDict("_RegisterADepositAddressResponseOptional", {'status': 'str | None', 'data': 'RegisterADepositAddressResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class RegisterADepositAddressResponse(_RegisterADepositAddressResponseOptional):
    pass

_RegisterADepositAddressResponseDataOptional = TypedDict("_RegisterADepositAddressResponseDataOptional", {'owner': 'str | None', 'deposit_address': 'str | None', 'registered': 'bool | None'}, total=False)
class RegisterADepositAddressResponseData(_RegisterADepositAddressResponseDataOptional):
    pass

class ResolvedMarketsParams(TypedDict):
    lookback: str
    order_by: str
    limit: int | Decimal

_ResolvedMarketsResponseOptional = TypedDict("_ResolvedMarketsResponseOptional", {'status': 'str | None', 'data': 'list[ResolvedMarketsResponseDataItem] | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class ResolvedMarketsResponse(_ResolvedMarketsResponseOptional):
    pass

_ResolvedMarketsResponseDataItemOptional = TypedDict("_ResolvedMarketsResponseDataItemOptional", {'symbol': 'str | None', 'taker_fee_rate': 'str | None', 'maker_rebate_share': 'str | None', 'tick_size_min': 'str | None', 'tick_sig_figs': 'int | Decimal | None', 'is_mece': 'bool | None', 'display_attributes': 'ResolvedMarketsResponseDataItemDisplayAttributes | None', 'listing_ts_ms': 'str | None', 'rules_url': 'str | None', 'stats': 'ResolvedMarketsResponseDataItemStats | None', 'mark_price': 'str | None', 'open_interest': 'str | None', 'long_interest': 'str | None', 'short_interest': 'str | None', 'resolution': 'ResolvedMarketsResponseDataItemResolution | None'}, total=False)
class ResolvedMarketsResponseDataItem(_ResolvedMarketsResponseDataItemOptional):
    pass

_ResolvedMarketsResponseDataItemDisplayAttributesOptional = TypedDict("_ResolvedMarketsResponseDataItemDisplayAttributesOptional", {'event_description': 'str | None', 'expected_resolution_time_ms': 'str | None', 'market_description': 'str | None', 'reverse_description': 'str | None', 'sort_order': 'str | None', 'tags': 'list[str] | None', 'topic': 'ResolvedMarketsResponseDataItemDisplayAttributesTopic | None'}, total=False)
class ResolvedMarketsResponseDataItemDisplayAttributes(_ResolvedMarketsResponseDataItemDisplayAttributesOptional):
    pass

_ResolvedMarketsResponseDataItemDisplayAttributesTopicOptional = TypedDict("_ResolvedMarketsResponseDataItemDisplayAttributesTopicOptional", {'id': 'str | None', 'description': 'str | None'}, total=False)
class ResolvedMarketsResponseDataItemDisplayAttributesTopic(_ResolvedMarketsResponseDataItemDisplayAttributesTopicOptional):
    pass

_ResolvedMarketsResponseDataItemResolutionOptional = TypedDict("_ResolvedMarketsResponseDataItemResolutionOptional", {'price': 'str | None', 'seq': 'str | None', 'market_resolution_ts_ms': 'str | None'}, total=False)
class ResolvedMarketsResponseDataItemResolution(_ResolvedMarketsResponseDataItemResolutionOptional):
    pass

_ResolvedMarketsResponseDataItemStatsOptional = TypedDict("_ResolvedMarketsResponseDataItemStatsOptional", {'last_24h': 'ResolvedMarketsResponseDataItemStatsLast24h | None', 'all_time': 'ResolvedMarketsResponseDataItemStatsAllTime | None'}, total=False)
class ResolvedMarketsResponseDataItemStats(_ResolvedMarketsResponseDataItemStatsOptional):
    pass

_ResolvedMarketsResponseDataItemStatsAllTimeOptional = TypedDict("_ResolvedMarketsResponseDataItemStatsAllTimeOptional", {'taker_buy_volume': 'str | None', 'taker_sell_volume': 'str | None', 'taker_buy_volume_notional': 'str | None', 'taker_sell_volume_notional': 'str | None', 'taker_buy_trade_count': 'str | None', 'taker_sell_trade_count': 'str | None', 'window_end_ts_ms_exclusive': 'str | None'}, total=False)
class ResolvedMarketsResponseDataItemStatsAllTime(_ResolvedMarketsResponseDataItemStatsAllTimeOptional):
    pass

_ResolvedMarketsResponseDataItemStatsLast24hOptional = TypedDict("_ResolvedMarketsResponseDataItemStatsLast24hOptional", {'mark_price_start': 'str | None', 'taker_buy_volume': 'str | None', 'taker_sell_volume': 'str | None', 'taker_buy_volume_notional': 'str | None', 'taker_sell_volume_notional': 'str | None', 'taker_buy_trade_count': 'str | None', 'taker_sell_trade_count': 'str | None', 'window_end_ts_ms_exclusive': 'str | None'}, total=False)
class ResolvedMarketsResponseDataItemStatsLast24h(_ResolvedMarketsResponseDataItemStatsLast24hOptional):
    pass

class RevokeATradingKeyParams(TypedDict):
    body: RevokeATradingKeyParamsBody

_RevokeATradingKeyParamsBodyOptional = TypedDict("_RevokeATradingKeyParamsBodyOptional", {'auth': 'dict[str, Any] | None'}, total=False)
class RevokeATradingKeyParamsBody(_RevokeATradingKeyParamsBodyOptional):
    trading_key: str

_RevokeATradingKeyResponseOptional = TypedDict("_RevokeATradingKeyResponseOptional", {'status': 'str | None', 'data': 'RevokeATradingKeyResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class RevokeATradingKeyResponse(_RevokeATradingKeyResponseOptional):
    pass

_RevokeATradingKeyResponseDataOptional = TypedDict("_RevokeATradingKeyResponseDataOptional", {'trading_key': 'str | None'}, total=False)
class RevokeATradingKeyResponseData(_RevokeATradingKeyResponseDataOptional):
    pass

class ServerTimeParams(TypedDict):
    pass

_ServerTimeResponseOptional = TypedDict("_ServerTimeResponseOptional", {'status': 'str | None', 'data': 'ServerTimeResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class ServerTimeResponse(_ServerTimeResponseOptional):
    pass

_ServerTimeResponseDataOptional = TypedDict("_ServerTimeResponseDataOptional", {'server_time_ms': 'str | None'}, total=False)
class ServerTimeResponseData(_ServerTimeResponseDataOptional):
    pass

class SetAUsernameParams(TypedDict):
    body: SetAUsernameParamsBody

_SetAUsernameParamsBodyOptional = TypedDict("_SetAUsernameParamsBodyOptional", {'auth': 'dict[str, Any] | None'}, total=False)
class SetAUsernameParamsBody(_SetAUsernameParamsBodyOptional):
    username: str

_SetAUsernameResponseOptional = TypedDict("_SetAUsernameResponseOptional", {'status': 'str | None', 'data': 'SetAUsernameResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class SetAUsernameResponse(_SetAUsernameResponseOptional):
    pass

_SetAUsernameResponseDataOptional = TypedDict("_SetAUsernameResponseDataOptional", {'username': 'str | None'}, total=False)
class SetAUsernameResponseData(_SetAUsernameResponseDataOptional):
    pass

class SetAnAccountBioParams(TypedDict):
    body: SetAnAccountBioParamsBody

_SetAnAccountBioParamsBodyOptional = TypedDict("_SetAnAccountBioParamsBodyOptional", {'auth': 'dict[str, Any] | None'}, total=False)
class SetAnAccountBioParamsBody(_SetAnAccountBioParamsBodyOptional):
    account_bio: str

_SetAnAccountBioResponseOptional = TypedDict("_SetAnAccountBioResponseOptional", {'status': 'str | None', 'data': 'SetAnAccountBioResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class SetAnAccountBioResponse(_SetAnAccountBioResponseOptional):
    pass

_SetAnAccountBioResponseDataOptional = TypedDict("_SetAnAccountBioResponseDataOptional", {'account_bio': 'str | None'}, total=False)
class SetAnAccountBioResponseData(_SetAnAccountBioResponseDataOptional):
    pass

class TakerDiscountTermsParams(TypedDict):
    owner: str

_TakerDiscountTermsResponseOptional = TypedDict("_TakerDiscountTermsResponseOptional", {'status': 'str | None', 'data': 'TakerDiscountTermsResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class TakerDiscountTermsResponse(_TakerDiscountTermsResponseOptional):
    pass

_TakerDiscountTermsResponseDataOptional = TypedDict("_TakerDiscountTermsResponseDataOptional", {'taker_discount_terms': 'TakerDiscountTermsResponseDataTakerDiscountTerms | None'}, total=False)
class TakerDiscountTermsResponseData(_TakerDiscountTermsResponseDataOptional):
    pass

_TakerDiscountTermsResponseDataTakerDiscountTermsOptional = TypedDict("_TakerDiscountTermsResponseDataTakerDiscountTermsOptional", {'taker_fee_discount_share': 'str | None', 'created_ts_ms': 'str | None', 'fee_savings_limit_usd': 'str | None', 'fee_savings_usd': 'str | None'}, total=False)
class TakerDiscountTermsResponseDataTakerDiscountTerms(_TakerDiscountTermsResponseDataTakerDiscountTermsOptional):
    pass

_TopHoldersParamsOptional = TypedDict("_TopHoldersParamsOptional", {'limit': 'int | Decimal | None'}, total=False)
class TopHoldersParams(_TopHoldersParamsOptional):
    symbols: str

_TopHoldersResponseOptional = TypedDict("_TopHoldersResponseOptional", {'status': 'str | None', 'data': 'TopHoldersResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class TopHoldersResponse(_TopHoldersResponseOptional):
    pass

_TopHoldersResponseDataOptional = TypedDict("_TopHoldersResponseDataOptional", {'holders': 'TopHoldersResponseDataHolders | None'}, total=False)
class TopHoldersResponseData(_TopHoldersResponseDataOptional):
    pass

_TopHoldersResponseDataHoldersOptional = TypedDict("_TopHoldersResponseDataHoldersOptional", {'SIM_EVENT_1.MARKET_1': 'TopHoldersResponseDataHoldersSIMEVENT1MARKET1 | None', 'SIM_EVENT_1.MARKET_2': 'TopHoldersResponseDataHoldersSIMEVENT1MARKET2 | None'}, total=False)
class TopHoldersResponseDataHolders(_TopHoldersResponseDataHoldersOptional):
    pass

_TopHoldersResponseDataHoldersSIMEVENT1MARKET1Optional = TypedDict("_TopHoldersResponseDataHoldersSIMEVENT1MARKET1Optional", {'longs': 'list[TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItem] | None', 'shorts': 'list[Any] | None'}, total=False)
class TopHoldersResponseDataHoldersSIMEVENT1MARKET1(_TopHoldersResponseDataHoldersSIMEVENT1MARKET1Optional):
    pass

_TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItemOptional = TypedDict("_TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItemOptional", {'owner': 'str | None', 'position': 'TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItemPosition | None'}, total=False)
class TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItem(_TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItemOptional):
    pass

_TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItemPositionOptional = TypedDict("_TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItemPositionOptional", {'symbol': 'str | None', 'size': 'str | None', 'cumulative_increasing_contracts': 'str | None', 'cumulative_increasing_notional_usd': 'str | None', 'cumulative_reducing_contracts': 'str | None', 'cumulative_reducing_notional_usd': 'str | None', 'cumulative_fees_usd': 'str | None', 'realized_pnl_usd': 'str | None', 'unrealized_pnl_usd': 'str | None', 'mark_price': 'str | None', 'average_entry_price': 'str | None', 'seq': 'str | None', 'update_seq': 'str | None', 'open_ts_ms': 'str | None', 'update_ts_ms': 'str | None'}, total=False)
class TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItemPosition(_TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItemPositionOptional):
    pass

_TopHoldersResponseDataHoldersSIMEVENT1MARKET2Optional = TypedDict("_TopHoldersResponseDataHoldersSIMEVENT1MARKET2Optional", {'longs': 'list[Any] | None', 'shorts': 'list[TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItem] | None'}, total=False)
class TopHoldersResponseDataHoldersSIMEVENT1MARKET2(_TopHoldersResponseDataHoldersSIMEVENT1MARKET2Optional):
    pass

_TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItemOptional = TypedDict("_TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItemOptional", {'owner': 'str | None', 'position': 'TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItemPosition | None'}, total=False)
class TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItem(_TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItemOptional):
    pass

_TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItemPositionOptional = TypedDict("_TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItemPositionOptional", {'symbol': 'str | None', 'size': 'str | None', 'cumulative_increasing_contracts': 'str | None', 'cumulative_increasing_notional_usd': 'str | None', 'cumulative_reducing_contracts': 'str | None', 'cumulative_reducing_notional_usd': 'str | None', 'cumulative_fees_usd': 'str | None', 'realized_pnl_usd': 'str | None', 'unrealized_pnl_usd': 'str | None', 'mark_price': 'str | None', 'average_entry_price': 'str | None', 'seq': 'str | None', 'update_seq': 'str | None', 'open_ts_ms': 'str | None', 'update_ts_ms': 'str | None'}, total=False)
class TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItemPosition(_TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItemPositionOptional):
    pass

_TradeCandlesParamsOptional = TypedDict("_TradeCandlesParamsOptional", {'start_time_ms': 'int | Decimal | None', 'count_back': 'int | Decimal | None'}, total=False)
class TradeCandlesParams(_TradeCandlesParamsOptional):
    symbol: str
    interval: str
    end_time_ms: int | Decimal

_TradeCandlesResponseOptional = TypedDict("_TradeCandlesResponseOptional", {'status': 'str | None', 'data': 'list[TradeCandlesResponseDataItem] | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class TradeCandlesResponse(_TradeCandlesResponseOptional):
    pass

_TradeCandlesResponseDataItemOptional = TypedDict("_TradeCandlesResponseDataItemOptional", {'time': 'str | None', 'open': 'str | None', 'high': 'str | None', 'low': 'str | None', 'close': 'str | None', 'volume': 'str | None', 'taker_buy_volume': 'str | None', 'taker_sell_volume': 'str | None', 'volume_notional': 'str | None', 'taker_buy_volume_notional': 'str | None', 'taker_sell_volume_notional': 'str | None', 'trade_count': 'str | None', 'taker_buy_count': 'str | None', 'taker_sell_count': 'str | None'}, total=False)
class TradeCandlesResponseDataItem(_TradeCandlesResponseDataItemOptional):
    pass

_TradesParamsOptional = TypedDict("_TradesParamsOptional", {'before_cursor': 'str | None', 'at_or_before_seq': 'str | None', 'limit': 'int | Decimal | None'}, total=False)
class TradesParams(_TradesParamsOptional):
    symbols: str

_TradesResponseOptional = TypedDict("_TradesResponseOptional", {'status': 'str | None', 'data': 'TradesResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class TradesResponse(_TradesResponseOptional):
    pass

_TradesResponseDataOptional = TypedDict("_TradesResponseDataOptional", {'items': 'list[TradesResponseDataItemsItem] | None', 'next_cursor': 'str | None'}, total=False)
class TradesResponseData(_TradesResponseDataOptional):
    pass

_TradesResponseDataItemsItemOptional = TypedDict("_TradesResponseDataItemsItemOptional", {'trade_id': 'str | None', 'symbol': 'str | None', 'taker_side': 'str | None', 'price': 'str | None', 'size': 'str | None', 'maker': 'str | None', 'taker': 'str | None', 'maker_reduce_only': 'bool | None', 'taker_reduce_only': 'bool | None', 'maker_position_size_prev': 'str | None', 'taker_position_size_prev': 'str | None', 'seq': 'str | None', 'trade_ts_ms': 'str | None'}, total=False)
class TradesResponseDataItemsItem(_TradesResponseDataItemsItemOptional):
    pass

class TradingKeyAccountsParams(TypedDict):
    trading_key_pubkey: str

_TradingKeyAccountsResponseOptional = TypedDict("_TradingKeyAccountsResponseOptional", {'status': 'str | None', 'data': 'list[str] | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class TradingKeyAccountsResponse(_TradingKeyAccountsResponseOptional):
    pass

class TradingKeysParams(TypedDict):
    owner: str

_TradingKeysResponseOptional = TypedDict("_TradingKeysResponseOptional", {'status': 'str | None', 'data': 'list[TradingKeysResponseDataItem] | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class TradingKeysResponse(_TradingKeysResponseOptional):
    pass

_TradingKeysResponseDataItemOptional = TypedDict("_TradingKeysResponseDataItemOptional", {'trading_key': 'str | None', 'name': 'str | None', 'expiration_ts_ms': 'str | None', 'open_order_count': 'str | None'}, total=False)
class TradingKeysResponseDataItem(_TradingKeysResponseDataItemOptional):
    pass

_TransferHistoryParamsOptional = TypedDict("_TransferHistoryParamsOptional", {'before_cursor': 'str | None', 'at_or_before_seq': 'str | None', 'limit': 'int | Decimal | None'}, total=False)
class TransferHistoryParams(_TransferHistoryParamsOptional):
    owner: str

_TransferHistoryResponseOptional = TypedDict("_TransferHistoryResponseOptional", {'status': 'str | None', 'data': 'TransferHistoryResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class TransferHistoryResponse(_TransferHistoryResponseOptional):
    pass

_TransferHistoryResponseDataOptional = TypedDict("_TransferHistoryResponseDataOptional", {'items': 'list[TransferHistoryResponseDataItemsItem] | None'}, total=False)
class TransferHistoryResponseData(_TransferHistoryResponseDataOptional):
    pass

_TransferHistoryResponseDataItemsItemOptional = TypedDict("_TransferHistoryResponseDataItemsItemOptional", {'type': 'str | None', 'status': 'str | None', 'collateral_change_usd': 'str | None', 'fee_usd': 'str | None', 'net_usd': 'str | None', 'seq': 'str | None', 'transfer_ts_ms': 'str | None'}, total=False)
class TransferHistoryResponseDataItemsItem(_TransferHistoryResponseDataItemsItemOptional):
    pass

class UsernamesParams(TypedDict):
    owners: list[str]

_UsernamesResponseOptional = TypedDict("_UsernamesResponseOptional", {'status': 'str | None', 'data': 'UsernamesResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class UsernamesResponse(_UsernamesResponseOptional):
    pass

_UsernamesResponseDataOptional = TypedDict("_UsernamesResponseDataOptional", {'usernames': 'UsernamesResponseDataUsernames | None'}, total=False)
class UsernamesResponseData(_UsernamesResponseDataOptional):
    pass

_UsernamesResponseDataUsernamesOptional = TypedDict("_UsernamesResponseDataUsernamesOptional", {'GmaDrppBC7P5ARKV8g3djiwP89vz1jLK23V2GBjuAEGB': 'str | None'}, total=False)
class UsernamesResponseDataUsernames(_UsernamesResponseDataUsernamesOptional):
    pass

class WithdrawCollateralParams(TypedDict):
    body: WithdrawCollateralParamsBody

_WithdrawCollateralParamsBodyOptional = TypedDict("_WithdrawCollateralParamsBodyOptional", {'auth': 'dict[str, Any] | None'}, total=False)
class WithdrawCollateralParamsBody(_WithdrawCollateralParamsBodyOptional):
    amount: str
    destination_authority: str
    destination_token_account: str

_WithdrawCollateralResponseOptional = TypedDict("_WithdrawCollateralResponseOptional", {'status': 'str | None', 'data': 'WithdrawCollateralResponseData | None', 'seq': 'str | None', 'state_time_ms': 'str | None', 'server_time_ms': 'str | None'}, total=False)
class WithdrawCollateralResponse(_WithdrawCollateralResponseOptional):
    pass

_WithdrawCollateralResponseDataOptional = TypedDict("_WithdrawCollateralResponseDataOptional", {'owner': 'str | None', 'amount': 'str | None', 'destination_authority': 'str | None', 'destination_token_account': 'str | None'}, total=False)
class WithdrawCollateralResponseData(_WithdrawCollateralResponseDataOptional):
    pass

CancelOrdersResponse = list[CancelOrdersResponseValueItem]
MassCancelOrdersResponse = list[MassCancelOrdersResponseValueItem]
PlaceAndReplaceOrdersResponse = list[PlaceAndReplaceOrdersResponseValueItem]
