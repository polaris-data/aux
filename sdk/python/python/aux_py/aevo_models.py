"""Aevo REST wire models generated from src/exchanges/aevo/schema/aevo.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'symbol': 'str | None', 'asset': 'str | None', 'instrument_type': 'str | None', 'topic': 'str | None', 'interval': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

_MarketsRequestOptional = TypedDict("_MarketsRequestOptional", {'asset': 'str | None', 'instrument_type': 'str | None'}, total=False)
class MarketsRequest(_MarketsRequestOptional):
    pass

class IndexRequest(TypedDict):
    asset: str

_StatisticsRequestOptional = TypedDict("_StatisticsRequestOptional", {'asset': 'str | None'}, total=False)
class StatisticsRequest(_StatisticsRequestOptional):
    pass

class AccountRequest(TypedDict):
    pass

class BalancesRequest(TypedDict):
    pass

class OpenOrdersRequest(TypedDict):
    pass

class PlaceOrderRequest(TypedDict):
    body: Any

class CancelOrderRequest(TypedDict):
    order_id: str

class CancelAllOrdersRequest(TypedDict):
    body: Any

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'instrument_name': 'str | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    pass

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'instrument_name': 'str | None', 'instrument_type': 'str | None', 'limit': 'int | None', 'offset': 'int | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    pass

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'instrument_name': 'str | None', 'instrument_type': 'str | None', 'limit': 'int | None', 'offset': 'int | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    pass

class OrderStatusRequest(TypedDict):
    order_id: str

class TradesRequest(TypedDict):
    instrument_name: str

class AmendOrderRequest(TypedDict):
    order_id: str
    body: Any

class CancelOrdersRequest(TypedDict):
    body: CancelOrdersBody

_CancelOrdersBodyOptional = TypedDict("_CancelOrdersBodyOptional", {'instrument_type': 'str | None'}, total=False)
class CancelOrdersBody(_CancelOrdersBodyOptional):
    order_ids: list[str]

MarketsResponse = Any
IndexResponse = Any
StatisticsResponse = Any
AccountResponse = Any
BalancesResponse = Any
OpenOrdersResponse = Any
PlaceOrderResponse = Any
CancelOrderResponse = Any
CancelAllOrdersResponse = Any
PositionsResponse = Any
OrderHistoryResponse = Any
FillsResponse = Any
OrderStatusResponse = Any
TradesResponse = Any
AmendOrderResponse = Any
CancelOrdersResponse = Any
