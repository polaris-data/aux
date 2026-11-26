"""Orderly REST wire models generated from src/exchanges/orderly/schema/orderly.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_MarketSummaryBodyOptional = TypedDict("_MarketSummaryBodyOptional", {'symbols': 'list[str] | None'}, total=False)
class MarketSummaryBody(_MarketSummaryBodyOptional):
    pass

_MarketTradesBodyOptional = TypedDict("_MarketTradesBodyOptional", {'limit': 'int | None'}, total=False)
class MarketTradesBody(_MarketTradesBodyOptional):
    symbol: str

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'symbol': 'str | None', 'topic': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

class MarketsRequest(TypedDict):
    pass

class TickerRequest(TypedDict):
    symbol: str

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    symbol: str

class MarketSummaryRequest(TypedDict):
    body: MarketSummaryBody

class MarketTradesRequest(TypedDict):
    body: MarketTradesBody

class AccountRequest(TypedDict):
    pass

_BalancesRequestOptional = TypedDict("_BalancesRequestOptional", {'all': 'bool | None'}, total=False)
class BalancesRequest(_BalancesRequestOptional):
    pass

class PositionsRequest(TypedDict):
    pass

class OrderStatusRequest(TypedDict):
    order_id: str

class OrderByClientIdRequest(TypedDict):
    client_order_id: str

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'symbol': 'str | None', 'side': 'str | None', 'order_type': 'str | None', 'page': 'int | None', 'size': 'int | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    pass

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'symbol': 'str | None', 'side': 'str | None', 'order_type': 'str | None', 'status': 'str | None', 'start_t': 'int | None', 'end_t': 'int | None', 'page': 'int | None', 'size': 'int | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    pass

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'symbol': 'str | None', 'start_t': 'int | None', 'end_t': 'int | None', 'page': 'int | None', 'size': 'int | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    pass

class PlaceOrderRequest(TypedDict):
    body: Any

class AmendOrderRequest(TypedDict):
    body: Any

_CancelOrderRequestOptional = TypedDict("_CancelOrderRequestOptional", {'order_id': 'str | None', 'client_order_id': 'str | None'}, total=False)
class CancelOrderRequest(_CancelOrderRequestOptional):
    symbol: str

class CancelByClientIdRequest(TypedDict):
    symbol: str
    client_order_id: str

class PlaceOrdersRequest(TypedDict):
    body: Any

class CancelOrdersRequest(TypedDict):
    symbol: str
    order_ids: str

class CancelOrdersByClientIdRequest(TypedDict):
    symbol: str
    client_order_ids: str

_CancelAllOrdersRequestOptional = TypedDict("_CancelAllOrdersRequestOptional", {'symbol': 'str | None'}, total=False)
class CancelAllOrdersRequest(_CancelAllOrdersRequestOptional):
    pass

class PlaceAlgoOrderRequest(TypedDict):
    body: Any

class AmendAlgoOrderRequest(TypedDict):
    body: Any

class CancelAlgoOrderRequest(TypedDict):
    symbol: str
    order_id: str

class CancelAlgoByClientIdRequest(TypedDict):
    symbol: str
    client_order_id: str

_CancelAllAlgoOrdersRequestOptional = TypedDict("_CancelAllAlgoOrdersRequestOptional", {'symbol': 'str | None'}, total=False)
class CancelAllAlgoOrdersRequest(_CancelAllAlgoOrdersRequestOptional):
    pass

class AlgoOrderStatusRequest(TypedDict):
    order_id: str

class AlgoOrderByClientIdRequest(TypedDict):
    client_order_id: str

_AlgoOrdersRequestOptional = TypedDict("_AlgoOrdersRequestOptional", {'symbol': 'str | None', 'status': 'str | None', 'page': 'int | None', 'size': 'int | None'}, total=False)
class AlgoOrdersRequest(_AlgoOrdersRequestOptional):
    pass

class MarketSummaryResponse(TypedDict):
    success: bool
    data: MarketSummaryData
    ts: int

_MarketSummaryDataOptional = TypedDict("_MarketSummaryDataOptional", {'total_24h_volume': 'str | None', 'total_open_interest': 'str | None'}, total=False)
class MarketSummaryData(_MarketSummaryDataOptional):
    markets: list[Any]

class MarketTradesResponse(TypedDict):
    success: bool
    data: MarketTradesData
    ts: int

_MarketTradesDataOptional = TypedDict("_MarketTradesDataOptional", {'next_cursor': 'Any | None'}, total=False)
class MarketTradesData(_MarketTradesDataOptional):
    rows: list[Any]

MarketsResponse = Any
TickerResponse = Any
TradesResponse = Any
AccountResponse = Any
BalancesResponse = Any
PositionsResponse = Any
OrderStatusResponse = Any
OrderByClientIdResponse = Any
OpenOrdersResponse = Any
OrderHistoryResponse = Any
FillsResponse = Any
PlaceOrderResponse = Any
AmendOrderResponse = Any
CancelOrderResponse = Any
CancelByClientIdResponse = Any
PlaceOrdersResponse = Any
CancelOrdersResponse = Any
CancelOrdersByClientIdResponse = Any
CancelAllOrdersResponse = Any
PlaceAlgoOrderResponse = Any
AmendAlgoOrderResponse = Any
CancelAlgoOrderResponse = Any
CancelAlgoByClientIdResponse = Any
CancelAllAlgoOrdersResponse = Any
AlgoOrderStatusResponse = Any
AlgoOrderByClientIdResponse = Any
AlgoOrdersResponse = Any
