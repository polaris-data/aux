"""Backpack REST wire models generated from src/exchanges/backpack/schema/backpack.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'symbol': 'str | None', 'interval': 'str | None', 'topic': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

class MarketsRequest(TypedDict):
    pass

class TickerRequest(TypedDict):
    symbol: str

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    symbol: str

class AccountRequest(TypedDict):
    pass

class BalancesRequest(TypedDict):
    pass

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'symbol': 'str | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    pass

_OrderStatusRequestOptional = TypedDict("_OrderStatusRequestOptional", {'orderId': 'str | None', 'clientId': 'int | None'}, total=False)
class OrderStatusRequest(_OrderStatusRequestOptional):
    symbol: str

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'symbol': 'str | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    pass

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'symbol': 'str | None', 'orderId': 'str | None', 'clientId': 'int | None', 'limit': 'int | None', 'offset': 'int | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    pass

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'symbol': 'str | None', 'orderId': 'str | None', 'tradeId': 'int | None', 'limit': 'int | None', 'offset': 'int | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    pass

class PlaceOrderRequest(TypedDict):
    body: Any

class PlaceOrdersRequest(TypedDict):
    body: list[Any]

class CancelOrderRequest(TypedDict):
    body: Any

class CancelAllOrdersRequest(TypedDict):
    body: Any

MarketsResponse = Any
TickerResponse = Any
TradesResponse = Any
AccountResponse = Any
BalancesResponse = Any
PositionsResponse = Any
OrderStatusResponse = Any
OpenOrdersResponse = Any
OrderHistoryResponse = Any
FillsResponse = Any
PlaceOrderResponse = Any
PlaceOrdersResponse = Any
CancelOrderResponse = Any
CancelAllOrdersResponse = Any
