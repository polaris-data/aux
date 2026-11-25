"""Decibel REST wire models generated from src/exchanges/decibel/schema/decibel.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'market': 'str | None', 'account': 'str | None', 'aggregation_level': 'int | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

class MarketsRequest(TypedDict):
    pass

_MarketsResponseItemOptional = TypedDict("_MarketsResponseItemOptional", {'market_addr': 'str | None', 'market_name': 'str | None', 'asset_type': 'str | None', 'px_decimals': 'int | None', 'sz_decimals': 'int | None', 'tick_size': 'int | None', 'lot_size': 'int | None', 'min_size': 'int | None'}, total=False)
class MarketsResponseItem(_MarketsResponseItemOptional):
    pass

class OrderBookRequest(TypedDict):
    market: str

_OrderBookResponseOptional = TypedDict("_OrderBookResponseOptional", {'bids': 'list[Any] | None', 'asks': 'list[Any] | None', 'market': 'str | None', 'unix_ms': 'int | None'}, total=False)
class OrderBookResponse(_OrderBookResponseOptional):
    pass

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None', 'offset': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    market: str

_TradesResponseOptional = TypedDict("_TradesResponseOptional", {'items': 'list[Any] | None', 'total_count': 'int | None'}, total=False)
class TradesResponse(_TradesResponseOptional):
    pass

class AccountRequest(TypedDict):
    account: str

_AccountResponseOptional = TypedDict("_AccountResponseOptional", {'perp_equity_balance': 'int | Decimal | None'}, total=False)
class AccountResponse(_AccountResponseOptional):
    pass

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'market_address': 'str | None', 'limit': 'int | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    account: str

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'asset_type': 'str | None', 'limit': 'int | None', 'offset': 'int | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    account: str

_OpenOrdersResponseOptional = TypedDict("_OpenOrdersResponseOptional", {'items': 'list[Any] | None', 'total_count': 'int | None'}, total=False)
class OpenOrdersResponse(_OpenOrdersResponseOptional):
    pass

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'market': 'str | None', 'limit': 'int | None', 'offset': 'int | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    account: str

_OrderHistoryResponseOptional = TypedDict("_OrderHistoryResponseOptional", {'items': 'list[Any] | None', 'total_count': 'int | None'}, total=False)
class OrderHistoryResponse(_OrderHistoryResponseOptional):
    pass

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'market': 'str | None', 'limit': 'int | None', 'offset': 'int | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    account: str

_FillsResponseOptional = TypedDict("_FillsResponseOptional", {'items': 'list[Any] | None', 'total_count': 'int | None'}, total=False)
class FillsResponse(_FillsResponseOptional):
    pass

class LedgerRequest(TypedDict):
    pass

_LedgerResponseOptional = TypedDict("_LedgerResponseOptional", {'chain_id': 'int | None'}, total=False)
class LedgerResponse(_LedgerResponseOptional):
    pass

class GasPriceRequest(TypedDict):
    pass

_GasPriceResponseOptional = TypedDict("_GasPriceResponseOptional", {'gas_estimate': 'int | None'}, total=False)
class GasPriceResponse(_GasPriceResponseOptional):
    pass

class TransactionRequest(TypedDict):
    hash: str

_TransactionResponseOptional = TypedDict("_TransactionResponseOptional", {'hash': 'str | None', 'type': 'str | None'}, total=False)
class TransactionResponse(_TransactionResponseOptional):
    pass

_PlaceOrderBodyOptional = TypedDict("_PlaceOrderBodyOptional", {'price': 'str | None', 'time_in_force': 'int | None', 'client_order_id': 'str | None', 'reduce_only': 'bool | None'}, total=False)
class PlaceOrderBody(_PlaceOrderBodyOptional):
    market: str
    size: str
    is_buy: bool

class PlaceOrderRequest(TypedDict):
    body: PlaceOrderBody

_PlaceOrderResponseOptional = TypedDict("_PlaceOrderResponseOptional", {'hash': 'str | None', 'type': 'str | None'}, total=False)
class PlaceOrderResponse(_PlaceOrderResponseOptional):
    pass

_CancelOrderBodyOptional = TypedDict("_CancelOrderBodyOptional", {'order_id': 'str | None'}, total=False)
class CancelOrderBody(_CancelOrderBodyOptional):
    market: str

class CancelOrderRequest(TypedDict):
    body: CancelOrderBody

_CancelOrderResponseOptional = TypedDict("_CancelOrderResponseOptional", {'hash': 'str | None', 'type': 'str | None'}, total=False)
class CancelOrderResponse(_CancelOrderResponseOptional):
    pass

_PositionOptional = TypedDict("_PositionOptional", {'market': 'str | None', 'user': 'str | None', 'size': 'int | Decimal | None', 'entry_price': 'int | Decimal | None', 'estimated_liquidation_price': 'int | Decimal | None', 'is_isolated': 'bool | None'}, total=False)
class Position(_PositionOptional):
    pass

MarketsResponse = list[MarketsResponseItem]
PositionsResponse = list[Position]
