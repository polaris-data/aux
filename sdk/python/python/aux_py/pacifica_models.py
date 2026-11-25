"""Pacifica REST wire models generated from src/exchanges/pacifica/schema/pacifica.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class MarketsRequest(TypedDict):
    pass

_OrderBookRequestOptional = TypedDict("_OrderBookRequestOptional", {'agg_level': 'int | None'}, total=False)
class OrderBookRequest(_OrderBookRequestOptional):
    symbol: str

class TradesRequest(TypedDict):
    symbol: str

class AccountRequest(TypedDict):
    account: str

class PositionsRequest(TypedDict):
    account: str

class OpenOrdersRequest(TypedDict):
    account: str

class OrderStatusRequest(TypedDict):
    order_id: int

class PlaceOrderRequest(TypedDict):
    body: LimitOrder

class PlaceMarketOrderRequest(TypedDict):
    body: MarketOrder

class CancelOrderRequest(TypedDict):
    body: CancelOrder

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'symbol': 'str | None', 'account': 'str | None', 'agg_level': 'int | None'}, total=False)
class Subscription(_SubscriptionOptional):
    source: str

_LimitOrderOptional = TypedDict("_LimitOrderOptional", {'reduce_only': 'bool | None', 'client_order_id': 'str | None'}, total=False)
class LimitOrder(_LimitOrderOptional):
    symbol: str
    side: str
    amount: str
    price: str
    tif: str

_MarketOrderOptional = TypedDict("_MarketOrderOptional", {'reduce_only': 'bool | None', 'client_order_id': 'str | None'}, total=False)
class MarketOrder(_MarketOrderOptional):
    symbol: str
    side: str
    amount: str
    slippage_percent: str

_CancelOrderOptional = TypedDict("_CancelOrderOptional", {'order_id': 'int | None', 'client_order_id': 'str | None'}, total=False)
class CancelOrder(_CancelOrderOptional):
    symbol: str

_PacificaReplyOptional = TypedDict("_PacificaReplyOptional", {'data': 'Any | None', 'code': 'int | Decimal | None', 'error': 'Any | None'}, total=False)
class PacificaReply(_PacificaReplyOptional):
    success: bool

