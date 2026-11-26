"""Hibachi REST wire models generated from src/exchanges/hibachi/schema/hibachi.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'symbol': 'str | None', 'accountId': 'int | None', 'interval': 'str | None', 'topic': 'str | None', 'depth': 'int | None', 'granularity': 'Any | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

class MarketsRequest(TypedDict):
    pass

class TickerRequest(TypedDict):
    symbol: str

class PricesRequest(TypedDict):
    symbol: str

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    symbol: str

class AccountRequest(TypedDict):
    accountId: int

class BalancesRequest(TypedDict):
    accountId: int

class PositionsRequest(TypedDict):
    accountId: int

_OrderStatusRequestOptional = TypedDict("_OrderStatusRequestOptional", {'orderId': 'str | None', 'clientId': 'str | None', 'nonce': 'int | None'}, total=False)
class OrderStatusRequest(_OrderStatusRequestOptional):
    accountId: int

class OpenOrdersRequest(TypedDict):
    accountId: int

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'cursorOrderId': 'str | None', 'endTime': 'int | None', 'limit': 'int | None', 'marketSymbols': 'list[str] | None', 'side': 'str | None', 'startTime': 'int | None', 'status': 'str | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    accountId: int

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'contractId': 'int | None', 'endTime': 'int | None', 'limit': 'int | None', 'marketSymbols': 'list[str] | None', 'maxJournalId': 'str | None', 'orderType': 'str | None', 'page': 'int | None', 'side': 'str | None', 'startTime': 'int | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    accountId: int

class PlaceOrderRequest(TypedDict):
    body: Any

class AmendOrderRequest(TypedDict):
    body: Any

class CancelOrderRequest(TypedDict):
    body: Any

class CancelAllOrdersRequest(TypedDict):
    body: Any

class PlaceOrdersRequest(TypedDict):
    body: Any

MarketsResponse = Any
TickerResponse = Any
PricesResponse = Any
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
CancelAllOrdersResponse = Any
PlaceOrdersResponse = Any
