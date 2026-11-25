"""Grvt REST wire models generated from src/exchanges/grvt/schema/grvt.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'instrument': 'str | None', 'sub_account_id': 'str | None', 'depth': 'int | None', 'rate': 'int | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

class InstrumentRequest(TypedDict):
    instrument: str

_InstrumentResponseResultOptional = TypedDict("_InstrumentResponseResultOptional", {'instrument': 'str | None', 'instrument_hash': 'str | None', 'base_decimals': 'int | None', 'kind': 'str | None', 'tick_size': 'str | None', 'min_size': 'str | None'}, total=False)
class InstrumentResponseResult(_InstrumentResponseResultOptional):
    pass

_InstrumentResponseOptional = TypedDict("_InstrumentResponseOptional", {'result': 'InstrumentResponseResult | None'}, total=False)
class InstrumentResponse(_InstrumentResponseOptional):
    pass

class MarketsBody(TypedDict):
    is_active: bool

class MarketsRequest(TypedDict):
    body: MarketsBody

_MarketsResponseOptional = TypedDict("_MarketsResponseOptional", {'result': 'list[InstrumentResponseResult] | None'}, total=False)
class MarketsResponse(_MarketsResponseOptional):
    pass

class OrderBookRequest(TypedDict):
    instrument: str
    depth: int

_OrderBookResponseResultOptional = TypedDict("_OrderBookResponseResultOptional", {'instrument': 'str | None', 'bids': 'list[Any] | None', 'asks': 'list[Any] | None'}, total=False)
class OrderBookResponseResult(_OrderBookResponseResultOptional):
    pass

_OrderBookResponseOptional = TypedDict("_OrderBookResponseOptional", {'result': 'OrderBookResponseResult | None'}, total=False)
class OrderBookResponse(_OrderBookResponseOptional):
    pass

class TradesRequest(TypedDict):
    instrument: str
    limit: int

_TradesResponseOptional = TypedDict("_TradesResponseOptional", {'result': 'list[Any] | None'}, total=False)
class TradesResponse(_TradesResponseOptional):
    pass

class LoginRequest(TypedDict):
    pass

_LoginResponseOptional = TypedDict("_LoginResponseOptional", {'status': 'str | None', 'location': 'str | None', 'sub_account_id': 'str | None'}, total=False)
class LoginResponse(_LoginResponseOptional):
    cookie: str
    account_id: str

class AccountRequest(TypedDict):
    sub_account_id: str

_AccountResponseResultOptional = TypedDict("_AccountResponseResultOptional", {'sub_account_id': 'str | None'}, total=False)
class AccountResponseResult(_AccountResponseResultOptional):
    pass

_AccountResponseOptional = TypedDict("_AccountResponseOptional", {'result': 'AccountResponseResult | None'}, total=False)
class AccountResponse(_AccountResponseOptional):
    pass

class PositionsRequest(TypedDict):
    sub_account_id: str

_PositionsResponseOptional = TypedDict("_PositionsResponseOptional", {'result': 'list[Any] | None'}, total=False)
class PositionsResponse(_PositionsResponseOptional):
    pass

class OpenOrdersRequest(TypedDict):
    sub_account_id: str

_OpenOrdersResponseOptional = TypedDict("_OpenOrdersResponseOptional", {'result': 'list[Any] | None'}, total=False)
class OpenOrdersResponse(_OpenOrdersResponseOptional):
    pass

_OrderStatusRequestOptional = TypedDict("_OrderStatusRequestOptional", {'order_id': 'str | None', 'client_order_id': 'str | None'}, total=False)
class OrderStatusRequest(_OrderStatusRequestOptional):
    sub_account_id: str

_OrderStatusResponseResultOptional = TypedDict("_OrderStatusResponseResultOptional", {'order_id': 'str | None'}, total=False)
class OrderStatusResponseResult(_OrderStatusResponseResultOptional):
    pass

_OrderStatusResponseOptional = TypedDict("_OrderStatusResponseOptional", {'result': 'OrderStatusResponseResult | None'}, total=False)
class OrderStatusResponse(_OrderStatusResponseOptional):
    pass

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'limit': 'int | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    sub_account_id: str

_FillsResponseOptional = TypedDict("_FillsResponseOptional", {'result': 'list[Any] | None'}, total=False)
class FillsResponse(_FillsResponseOptional):
    pass

_PlaceOrderBodyOptional = TypedDict("_PlaceOrderBodyOptional", {'time_in_force': 'str | None', 'post_only': 'bool | None', 'reduce_only': 'bool | None'}, total=False)
class PlaceOrderBody(_PlaceOrderBodyOptional):
    sub_account_id: str
    is_market: bool
    legs: list[OrderLeg]
    metadata: OrderMetadata

class PlaceOrderRequest(TypedDict):
    body: PlaceOrderBody

_PlaceOrderResponseResultOptional = TypedDict("_PlaceOrderResponseResultOptional", {'order_id': 'str | None'}, total=False)
class PlaceOrderResponseResult(_PlaceOrderResponseResultOptional):
    pass

_PlaceOrderResponseOptional = TypedDict("_PlaceOrderResponseOptional", {'result': 'PlaceOrderResponseResult | None'}, total=False)
class PlaceOrderResponse(_PlaceOrderResponseOptional):
    pass

_CancelOrderBodyOptional = TypedDict("_CancelOrderBodyOptional", {'order_id': 'str | None', 'client_order_id': 'str | None'}, total=False)
class CancelOrderBody(_CancelOrderBodyOptional):
    sub_account_id: str

class CancelOrderRequest(TypedDict):
    body: CancelOrderBody

_CancelOrderResponseResultOptional = TypedDict("_CancelOrderResponseResultOptional", {'ack': 'bool | None'}, total=False)
class CancelOrderResponseResult(_CancelOrderResponseResultOptional):
    pass

_CancelOrderResponseOptional = TypedDict("_CancelOrderResponseOptional", {'result': 'CancelOrderResponseResult | None'}, total=False)
class CancelOrderResponse(_CancelOrderResponseOptional):
    pass

class OrderLeg(TypedDict):
    instrument: str
    size: str
    limit_price: str
    is_buying_asset: bool

class OrderMetadata(TypedDict):
    client_order_id: str

