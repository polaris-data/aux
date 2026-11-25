"""Paragon REST wire models generated from src/exchanges/paragon/schema/paragon.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class MarketsRequest(TypedDict):
    pass

class OrderBookRequest(TypedDict):
    coin: str

class TradesRequest(TypedDict):
    coin: str

class AccountRequest(TypedDict):
    user: str

class PositionsRequest(TypedDict):
    user: str

class OpenOrdersRequest(TypedDict):
    user: str

class OrderStatusRequest(TypedDict):
    user: str
    coin: str
    oid: Any

_PlaceOrderRequestOptional = TypedDict("_PlaceOrderRequestOptional", {'options': 'Any | None'}, total=False)
class PlaceOrderRequest(_PlaceOrderRequestOptional):
    coin: str
    body: Order

_CancelOrderRequestOptional = TypedDict("_CancelOrderRequestOptional", {'options': 'Any | None'}, total=False)
class CancelOrderRequest(_CancelOrderRequestOptional):
    coin: str
    body: CancelOrder

class PerpDexsRequest(TypedDict):
    pass

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'coin': 'str | None', 'user': 'str | None', 'aggregateByTime': 'bool | None'}, total=False)
class Subscription(_SubscriptionOptional):
    type: str

_OrderOptional = TypedDict("_OrderOptional", {'c': 'str | None'}, total=False)
class Order(_OrderOptional):
    b: bool
    p: str
    s: str
    r: bool
    t: Limit

_CancelOrderOptional = TypedDict("_CancelOrderOptional", {'o': 'int | None', 'cloid': 'str | None'}, total=False)
class CancelOrder(_CancelOrderOptional):
    pass

class Markets(TypedDict):
    universe: list[Market]

class Account(TypedDict):
    assetPositions: list[Any]

class Orderbook(TypedDict):
    coin: str
    levels: list[list[BookLevel]]

_MarketOptional = TypedDict("_MarketOptional", {'szDecimals': 'int | None', 'maxLeverage': 'int | None', 'isDelisted': 'bool | None'}, total=False)
class Market(_MarketOptional):
    name: str

class BookLevel(TypedDict):
    px: str
    sz: str
    n: int

class Limit(TypedDict):
    limit: TimeInForce

class TimeInForce(TypedDict):
    tif: str

