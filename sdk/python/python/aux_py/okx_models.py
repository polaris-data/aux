"""Okx REST wire models generated from src/exchanges/okx/schema/okx.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'instId': 'str | None', 'instType': 'str | None', 'instFamily': 'str | None', 'topic': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

_MarketsRequestOptional = TypedDict("_MarketsRequestOptional", {'instFamily': 'str | None', 'instId': 'str | None'}, total=False)
class MarketsRequest(_MarketsRequestOptional):
    instType: str

class TickerRequest(TypedDict):
    instId: str

_TickersRequestOptional = TypedDict("_TickersRequestOptional", {'instFamily': 'str | None'}, total=False)
class TickersRequest(_TickersRequestOptional):
    instType: str

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    instId: str

class AccountRequest(TypedDict):
    pass

_BalancesRequestOptional = TypedDict("_BalancesRequestOptional", {'ccy': 'str | None'}, total=False)
class BalancesRequest(_BalancesRequestOptional):
    pass

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'instType': 'str | None', 'instId': 'str | None', 'posId': 'str | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    pass

_OrderStatusRequestOptional = TypedDict("_OrderStatusRequestOptional", {'ordId': 'str | None', 'clOrdId': 'str | None'}, total=False)
class OrderStatusRequest(_OrderStatusRequestOptional):
    instId: str

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'instType': 'str | None', 'instFamily': 'str | None', 'instId': 'str | None', 'ordType': 'str | None', 'state': 'str | None', 'after': 'str | None', 'before': 'str | None', 'limit': 'int | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    pass

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'instFamily': 'str | None', 'instId': 'str | None', 'ordType': 'str | None', 'state': 'str | None', 'after': 'str | None', 'before': 'str | None', 'begin': 'int | None', 'end': 'int | None', 'limit': 'int | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    instType: str

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'instType': 'str | None', 'instFamily': 'str | None', 'instId': 'str | None', 'ordId': 'str | None', 'subType': 'str | None', 'after': 'str | None', 'before': 'str | None', 'begin': 'int | None', 'end': 'int | None', 'limit': 'int | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    pass

class PlaceOrderRequest(TypedDict):
    body: Any

class AmendOrderRequest(TypedDict):
    body: Any

class CancelOrderRequest(TypedDict):
    body: Any

class PlaceOrdersRequest(TypedDict):
    body: list[Any]

class AmendOrdersRequest(TypedDict):
    body: list[Any]

class CancelOrdersRequest(TypedDict):
    body: list[Any]

class PlaceAlgoOrderRequest(TypedDict):
    body: Any

class AmendAlgoOrderRequest(TypedDict):
    body: Any

class CancelAlgoOrdersRequest(TypedDict):
    body: list[Any]

class CancelAdvanceAlgoOrdersRequest(TypedDict):
    body: list[Any]

_AlgoOrderStatusRequestOptional = TypedDict("_AlgoOrderStatusRequestOptional", {'algoId': 'str | None', 'algoClOrdId': 'str | None'}, total=False)
class AlgoOrderStatusRequest(_AlgoOrderStatusRequestOptional):
    pass

_OpenAlgoOrdersRequestOptional = TypedDict("_OpenAlgoOrdersRequestOptional", {'algoId': 'str | None', 'instType': 'str | None', 'instId': 'str | None', 'after': 'str | None', 'before': 'str | None', 'limit': 'int | None'}, total=False)
class OpenAlgoOrdersRequest(_OpenAlgoOrdersRequestOptional):
    ordType: str

_AlgoOrderHistoryRequestOptional = TypedDict("_AlgoOrderHistoryRequestOptional", {'state': 'str | None', 'algoId': 'str | None', 'instType': 'str | None', 'instId': 'str | None', 'after': 'str | None', 'before': 'str | None', 'limit': 'int | None'}, total=False)
class AlgoOrderHistoryRequest(_AlgoOrderHistoryRequestOptional):
    ordType: str

MarketsResponse = Any
TickerResponse = Any
TickersResponse = Any
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
PlaceAlgoOrderResponse = Any
AmendAlgoOrderResponse = Any
CancelAlgoOrdersResponse = Any
CancelAdvanceAlgoOrdersResponse = Any
AlgoOrderStatusResponse = Any
OpenAlgoOrdersResponse = Any
AlgoOrderHistoryResponse = Any
