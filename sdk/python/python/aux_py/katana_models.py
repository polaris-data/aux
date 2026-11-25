"""Katana REST wire models generated from src/exchanges/katana/schema/katana.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'market': 'str | None', 'wallet': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

class MarketsRequest(TypedDict):
    pass

_MarketsResponseItemOptional = TypedDict("_MarketsResponseItemOptional", {'market': 'str | None', 'status': 'str | None'}, total=False)
class MarketsResponseItem(_MarketsResponseItemOptional):
    pass

_OrderBookRequestOptional = TypedDict("_OrderBookRequestOptional", {'level': 'int | None', 'limit': 'int | None'}, total=False)
class OrderBookRequest(_OrderBookRequestOptional):
    market: str

_OrderBookResponseOptional = TypedDict("_OrderBookResponseOptional", {'bids': 'list[Any] | None', 'asks': 'list[Any] | None'}, total=False)
class OrderBookResponse(_OrderBookResponseOptional):
    sequence: int

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    market: str

class AccountRequest(TypedDict):
    wallet: str

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'market': 'str | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    wallet: str

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'market': 'str | None', 'closed': 'bool | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    wallet: str

_OrderStatusRequestOptional = TypedDict("_OrderStatusRequestOptional", {'orderId': 'str | None'}, total=False)
class OrderStatusRequest(_OrderStatusRequestOptional):
    wallet: str

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'market': 'str | None', 'limit': 'int | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    wallet: str

class WsTokenRequest(TypedDict):
    wallet: str

_WsTokenResponseOptional = TypedDict("_WsTokenResponseOptional", {'token': 'str | None'}, total=False)
class WsTokenResponse(_WsTokenResponseOptional):
    pass

_PlaceOrderBodyOptional = TypedDict("_PlaceOrderBodyOptional", {'price': 'str | None', 'clientOrderId': 'str | None', 'timeInForce': 'str | None', 'selfTradePrevention': 'str | None', 'reduceOnly': 'bool | None'}, total=False)
class PlaceOrderBody(_PlaceOrderBodyOptional):
    wallet: str
    market: str
    type: str
    side: str
    quantity: str

class PlaceOrderRequest(TypedDict):
    body: PlaceOrderBody

_PlaceOrderResponseOptional = TypedDict("_PlaceOrderResponseOptional", {'orderId': 'str | None', 'status': 'str | None'}, total=False)
class PlaceOrderResponse(_PlaceOrderResponseOptional):
    pass

_CancelOrderBodyOptional = TypedDict("_CancelOrderBodyOptional", {'orderId': 'str | None'}, total=False)
class CancelOrderBody(_CancelOrderBodyOptional):
    wallet: str

class CancelOrderRequest(TypedDict):
    body: CancelOrderBody

_CancelOrderResponseItemOptional = TypedDict("_CancelOrderResponseItemOptional", {'orderId': 'str | None', 'status': 'str | None'}, total=False)
class CancelOrderResponseItem(_CancelOrderResponseItemOptional):
    pass

MarketsResponse = list[MarketsResponseItem]
TradesResponse = list[Any]
AccountResponse = list[Any]
PositionsResponse = list[Any]
OpenOrdersResponse = list[Any]
OrderStatusResponse = list[Any]
FillsResponse = list[Any]
CancelOrderResponse = list[CancelOrderResponseItem]
