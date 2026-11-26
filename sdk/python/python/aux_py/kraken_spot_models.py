"""KrakenSpot REST wire models generated from src/exchanges/kraken_spot/schema/kraken_spot.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'symbol': 'str | None', 'symbols': 'list[str] | None', 'depth': 'int | None', 'snapshot': 'bool | None', 'topic': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

_MarketsRequestOptional = TypedDict("_MarketsRequestOptional", {'pair': 'str | None', 'info': 'str | None', 'country_code': 'str | None', 'execution_venue': 'str | None'}, total=False)
class MarketsRequest(_MarketsRequestOptional):
    pass

_TickerRequestOptional = TypedDict("_TickerRequestOptional", {'pair': 'str | None', 'execution_venue': 'str | None'}, total=False)
class TickerRequest(_TickerRequestOptional):
    pass

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'since': 'str | None', 'count': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    pair: str

_AccountRequestOptional = TypedDict("_AccountRequestOptional", {'asset': 'str | None'}, total=False)
class AccountRequest(_AccountRequestOptional):
    pass

class BalancesRequest(TypedDict):
    pass

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'txid': 'str | None', 'docalcs': 'bool | None', 'consolidation': 'str | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    pass

_OrderStatusRequestOptional = TypedDict("_OrderStatusRequestOptional", {'trades': 'bool | None', 'userref': 'int | None', 'consolidate_taker': 'bool | None'}, total=False)
class OrderStatusRequest(_OrderStatusRequestOptional):
    txid: str

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'trades': 'bool | None', 'userref': 'int | None', 'cl_ord_id': 'str | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    pass

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'trades': 'bool | None', 'userref': 'int | None', 'start': 'str | None', 'end': 'str | None', 'ofs': 'int | None', 'closetime': 'str | None', 'consolidate_taker': 'bool | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    pass

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'type': 'str | None', 'trades': 'bool | None', 'start': 'str | None', 'end': 'str | None', 'ofs': 'int | None', 'consolidate_taker': 'bool | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    pass

class WebsocketTokenRequest(TypedDict):
    pass

_PlaceOrderRequestOptional = TypedDict("_PlaceOrderRequestOptional", {'price': 'str | None', 'price2': 'str | None', 'leverage': 'str | None', 'oflags': 'str | None', 'starttm': 'str | None', 'expiretm': 'str | None', 'timeinforce': 'str | None', 'userref': 'int | None', 'cl_ord_id': 'str | None', 'validate': 'bool | None', 'reduce_only': 'bool | None', 'deadline': 'str | None', 'stptype': 'str | None', 'close_ordertype': 'str | None', 'close_price': 'str | None', 'close_price2': 'str | None'}, total=False)
class PlaceOrderRequest(_PlaceOrderRequestOptional):
    pair: str
    type: str
    ordertype: str
    volume: str

_AmendOrderRequestOptional = TypedDict("_AmendOrderRequestOptional", {'txid': 'str | None', 'cl_ord_id': 'str | None', 'order_qty': 'str | None', 'display_qty': 'str | None', 'limit_price': 'str | None', 'limit_price_type': 'str | None', 'trigger_price': 'str | None', 'trigger_price_type': 'str | None', 'post_only': 'bool | None', 'deadline': 'str | None'}, total=False)
class AmendOrderRequest(_AmendOrderRequestOptional):
    pass

_CancelOrderRequestOptional = TypedDict("_CancelOrderRequestOptional", {'txid': 'str | None', 'cl_ord_id': 'str | None'}, total=False)
class CancelOrderRequest(_CancelOrderRequestOptional):
    pass

class CancelAllOrdersRequest(TypedDict):
    pass

class PlaceOrdersRequest(TypedDict):
    body: Any

class CancelOrdersRequest(TypedDict):
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
WebsocketTokenResponse = Any
PlaceOrderResponse = Any
AmendOrderResponse = Any
CancelOrderResponse = Any
CancelAllOrdersResponse = Any
PlaceOrdersResponse = Any
CancelOrdersResponse = Any
