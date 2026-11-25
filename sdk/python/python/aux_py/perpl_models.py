"""Perpl REST wire models generated from src/exchanges/perpl/schema/perpl.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'market_id': 'int | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

class MarketsRequest(TypedDict):
    pass

_MarketsResponseChainOptional = TypedDict("_MarketsResponseChainOptional", {'chain_id': 'int | None'}, total=False)
class MarketsResponseChain(_MarketsResponseChainOptional):
    pass

_MarketsResponseOptional = TypedDict("_MarketsResponseOptional", {'markets': 'list[Market] | None', 'tokens': 'list[Any] | None', 'chain': 'MarketsResponseChain | None'}, total=False)
class MarketsResponse(_MarketsResponseOptional):
    pass

_OrderBookRequestOptional = TypedDict("_OrderBookRequestOptional", {'levels': 'int | None'}, total=False)
class OrderBookRequest(_OrderBookRequestOptional):
    market_id: int

_OrderBookResponseOptional = TypedDict("_OrderBookResponseOptional", {'bid': 'list[PriceLevel] | None', 'ask': 'list[PriceLevel] | None'}, total=False)
class OrderBookResponse(_OrderBookResponseOptional):
    mt: int

class AccountRequest(TypedDict):
    pass

_AccountResponseOptional = TypedDict("_AccountResponseOptional", {'sn': 'int | None', 'addr': 'str | None', 'as': 'list[Any] | None', 'sts': 'list[Any] | None'}, total=False)
class AccountResponse(_AccountResponseOptional):
    mt: int

class OpenOrdersRequest(TypedDict):
    pass

_OpenOrdersResponseOptional = TypedDict("_OpenOrdersResponseOptional", {'sn': 'int | None', 'd': 'list[Any] | None'}, total=False)
class OpenOrdersResponse(_OpenOrdersResponseOptional):
    mt: int

class PositionsRequest(TypedDict):
    pass

_PositionsResponseOptional = TypedDict("_PositionsResponseOptional", {'sn': 'int | None', 'd': 'list[Any] | None'}, total=False)
class PositionsResponse(_PositionsResponseOptional):
    mt: int

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'count': 'int | None', 'page': 'str | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    pass

_FillsResponseOptional = TypedDict("_FillsResponseOptional", {'d': 'list[Any] | None', 'np': 'str | None'}, total=False)
class FillsResponse(_FillsResponseOptional):
    pass

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'count': 'int | None', 'page': 'str | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    pass

_OrderHistoryResponseOptional = TypedDict("_OrderHistoryResponseOptional", {'d': 'list[Any] | None', 'np': 'str | None'}, total=False)
class OrderHistoryResponse(_OrderHistoryResponseOptional):
    pass

_PlaceOrderBodyOptional = TypedDict("_PlaceOrderBodyOptional", {'p': 'int | None'}, total=False)
class PlaceOrderBody(_PlaceOrderBodyOptional):
    rq: int
    mkt: int
    acc: int
    t: int
    s: int
    fl: int
    lv: int
    lb: int

class PlaceOrderRequest(TypedDict):
    body: PlaceOrderBody

class PlaceOrderResponseStatus(TypedDict):
    code: int

class PlaceOrderResponseStatusesItem(TypedDict):
    code: int

_PlaceOrderResponseOptional = TypedDict("_PlaceOrderResponseOptional", {'status': 'PlaceOrderResponseStatus | None', 'statuses': 'list[PlaceOrderResponseStatusesItem] | None'}, total=False)
class PlaceOrderResponse(_PlaceOrderResponseOptional):
    mt: int

_CancelOrderBodyOptional = TypedDict("_CancelOrderBodyOptional", {'oid': 'int | None'}, total=False)
class CancelOrderBody(_CancelOrderBodyOptional):
    rq: int
    mkt: int
    acc: int
    t: int
    s: int
    fl: int
    lv: int
    lb: int

class CancelOrderRequest(TypedDict):
    body: CancelOrderBody

class CancelOrderResponseStatus(TypedDict):
    code: int

class CancelOrderResponseStatusesItem(TypedDict):
    code: int

_CancelOrderResponseOptional = TypedDict("_CancelOrderResponseOptional", {'status': 'CancelOrderResponseStatus | None', 'statuses': 'list[CancelOrderResponseStatusesItem] | None'}, total=False)
class CancelOrderResponse(_CancelOrderResponseOptional):
    mt: int

_MarketOptional = TypedDict("_MarketOptional", {'size_decimals': 'int | None', 'price_decimals': 'int | None'}, total=False)
class Market(_MarketOptional):
    id: int
    name: str

class PriceLevel(TypedDict):
    p: int
    s: int
    o: int

