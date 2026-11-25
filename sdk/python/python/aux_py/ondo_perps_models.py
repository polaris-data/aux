"""OndoPerps REST wire models generated from src/exchanges/ondo_perps/schema/ondo_perps.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_MarketsRequestOptional = TypedDict("_MarketsRequestOptional", {'sparkline': 'bool | None'}, total=False)
class MarketsRequest(_MarketsRequestOptional):
    pass

_OrderBookRequestOptional = TypedDict("_OrderBookRequestOptional", {'depth': 'int | None'}, total=False)
class OrderBookRequest(_OrderBookRequestOptional):
    market: str

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None', 'cursor': 'str | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    market: str

class AccountRequest(TypedDict):
    pass

class PositionsRequest(TypedDict):
    pass

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'market': 'str | None', 'limit': 'int | None', 'cursor': 'str | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    pass

class OrderStatusRequest(TypedDict):
    orderID: str

class PlaceOrderRequest(TypedDict):
    body: Order

class CancelOrderRequest(TypedDict):
    orderID: str

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'markets': 'list[str] | None', 'numPastTrades': 'int | None', 'depthLevels': 'Any | None', 'limit': 'int | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

_OrderOptional = TypedDict("_OrderOptional", {'price': 'str | None', 'clientOrderId': 'str | None', 'reduceOnly': 'bool | None', 'postOnly': 'bool | None', 'builderCode': 'str | None'}, total=False)
class Order(_OrderOptional):
    market: str
    side: str
    type: str
    size: str

_OndoReplyOptional = TypedDict("_OndoReplyOptional", {'result': 'Any | None', 'error': 'Any | None'}, total=False)
class OndoReply(_OndoReplyOptional):
    success: bool

