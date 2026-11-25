"""Aster REST wire models generated from src/exchanges/aster/schema/aster.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class MarketsRequest(TypedDict):
    pass

_OrderBookRequestOptional = TypedDict("_OrderBookRequestOptional", {'limit': 'int | None'}, total=False)
class OrderBookRequest(_OrderBookRequestOptional):
    symbol: str

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    symbol: str

class AccountRequest(TypedDict):
    pass

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'symbol': 'str | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    pass

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'symbol': 'str | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    pass

_OrderStatusRequestOptional = TypedDict("_OrderStatusRequestOptional", {'orderId': 'int | None', 'origClientOrderId': 'str | None'}, total=False)
class OrderStatusRequest(_OrderStatusRequestOptional):
    symbol: str

class PlaceOrderRequest(TypedDict):
    body: Order

class CancelOrderRequest(TypedDict):
    body: CancelOrder

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'symbol': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

_OrderOptional = TypedDict("_OrderOptional", {'timeInForce': 'str | None', 'price': 'str | None', 'reduceOnly': 'str | None', 'positionSide': 'str | None', 'newClientOrderId': 'str | None'}, total=False)
class Order(_OrderOptional):
    symbol: str
    side: str
    type: str
    quantity: str

_CancelOrderOptional = TypedDict("_CancelOrderOptional", {'orderId': 'int | None', 'origClientOrderId': 'str | None'}, total=False)
class CancelOrder(_CancelOrderOptional):
    symbol: str

class Markets(TypedDict):
    symbols: list[Any]

class Orderbook(TypedDict):
    lastUpdateId: int
    bids: list[list[str]]
    asks: list[list[str]]

class Account(TypedDict):
    assets: list[Any]
    positions: list[Any]

class OrderReply(TypedDict):
    orderId: int

