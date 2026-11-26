"""Bybit REST wire models generated from src/exchanges/bybit/schema/bybit.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'symbol': 'str | None', 'category': 'str | None', 'depth': 'int | None', 'topic': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

_MarketsRequestOptional = TypedDict("_MarketsRequestOptional", {'symbol': 'str | None', 'status': 'str | None', 'baseCoin': 'str | None', 'limit': 'int | None', 'cursor': 'str | None'}, total=False)
class MarketsRequest(_MarketsRequestOptional):
    category: str

_TickerRequestOptional = TypedDict("_TickerRequestOptional", {'symbol': 'str | None', 'baseCoin': 'str | None', 'expDate': 'str | None'}, total=False)
class TickerRequest(_TickerRequestOptional):
    category: str

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'symbol': 'str | None', 'baseCoin': 'str | None', 'optionType': 'str | None', 'limit': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    category: str

class AccountRequest(TypedDict):
    pass

_BalancesRequestOptional = TypedDict("_BalancesRequestOptional", {'coin': 'str | None'}, total=False)
class BalancesRequest(_BalancesRequestOptional):
    accountType: str

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'symbol': 'str | None', 'baseCoin': 'str | None', 'settleCoin': 'str | None', 'limit': 'int | None', 'cursor': 'str | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    category: str

_OrderStatusRequestOptional = TypedDict("_OrderStatusRequestOptional", {'orderId': 'str | None', 'orderLinkId': 'str | None', 'symbol': 'str | None', 'orderFilter': 'str | None'}, total=False)
class OrderStatusRequest(_OrderStatusRequestOptional):
    category: str

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'symbol': 'str | None', 'baseCoin': 'str | None', 'settleCoin': 'str | None', 'orderFilter': 'str | None', 'openOnly': 'int | None', 'limit': 'int | None', 'cursor': 'str | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    category: str

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'symbol': 'str | None', 'baseCoin': 'str | None', 'settleCoin': 'str | None', 'orderId': 'str | None', 'orderLinkId': 'str | None', 'orderFilter': 'str | None', 'orderStatus': 'str | None', 'startTime': 'int | None', 'endTime': 'int | None', 'limit': 'int | None', 'cursor': 'str | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    category: str

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'symbol': 'str | None', 'orderId': 'str | None', 'orderLinkId': 'str | None', 'baseCoin': 'str | None', 'startTime': 'int | None', 'endTime': 'int | None', 'execType': 'str | None', 'limit': 'int | None', 'cursor': 'str | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    category: str

class PlaceOrderRequest(TypedDict):
    body: Any

class AmendOrderRequest(TypedDict):
    body: Any

class CancelOrderRequest(TypedDict):
    body: Any

class PlaceOrdersRequest(TypedDict):
    body: Any

class AmendOrdersRequest(TypedDict):
    body: Any

class CancelOrdersRequest(TypedDict):
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
AmendOrderResponse = Any
CancelOrderResponse = Any
PlaceOrdersResponse = Any
AmendOrdersResponse = Any
CancelOrdersResponse = Any
CancelAllOrdersResponse = Any
