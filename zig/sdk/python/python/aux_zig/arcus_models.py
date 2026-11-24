"""Arcus REST wire models generated from src/exchanges/arcus/schema/arcus.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

from typing import Literal
Side = Literal["BUY", "SELL"]
OrderType = Literal["LIMIT", "MARKET"]
TimeInForce = Literal["GTT", "FOK", "IOC", "ALO"]
_MarketInfoOptional = TypedDict("_MarketInfoOptional", {'status': 'str'}, total=False)
class MarketInfo(_MarketInfoOptional):
    marketId: int
    marketDisplayName: str
    tickSize: str
    stepSize: str

class Markets(TypedDict):
    markets: list[MarketInfo]

_OrderBookOptional = TypedDict("_OrderBookOptional", {'lastSequenceId': 'int', 'globalSequenceId': 'int'}, total=False)
class OrderBook(_OrderBookOptional):
    bids: list[list[str]]
    asks: list[list[str]]
    timestamp: int

_TradeOptional = TypedDict("_TradeOptional", {'sequenceNumber': 'int'}, total=False)
class Trade(_TradeOptional):
    marketId: int
    marketDisplayName: str
    side: str
    price: str
    size: str
    tradeId: str
    timestamp: int

_TradesOptional = TypedDict("_TradesOptional", {'total': 'int'}, total=False)
class Trades(_TradesOptional):
    trades: list[Trade]

class Position(TypedDict):
    marketId: int
    marketDisplayName: str
    side: str
    size: str
    averageEntryPrice: str

_PositionsOptional = TypedDict("_PositionsOptional", {'total': 'int'}, total=False)
class Positions(_PositionsOptional):
    positions: dict[str, Position]

class Account(TypedDict):
    address: str
    accountIndex: int
    equity: str
    freeCollateral: str
    netQuoteBalance: str
    positions: dict[str, Position]
    sequenceNumber: int

_OpenOrdersOptional = TypedDict("_OpenOrdersOptional", {'total': 'int'}, total=False)
class OpenOrders(_OpenOrdersOptional):
    orders: list[Any]

_MarketsRequestOptional = TypedDict("_MarketsRequestOptional", {'market': 'str | None'}, total=False)
class MarketsRequest(_MarketsRequestOptional):
    pass

class OrderBookRequest(TypedDict):
    market: str
    nLevels: int

class AccountRequest(TypedDict):
    address: str
    accountIndex: int

class TradesRequest(TypedDict):
    market: str
    limit: int

_OrderRequestOptional = TypedDict("_OrderRequestOptional", {'clientId': 'str | None', 'reduceOnly': 'bool | None'}, total=False)
class OrderRequest(_OrderRequestOptional):
    marketId: int
    orderSide: Side
    orderType: OrderType
    quantity: str
    price: str
    timeInForce: TimeInForce
    goodTilTime: str

class PlaceOrderParams(TypedDict):
    market: MarketInfo
    body: OrderRequest

_CancelOrderRequestOptional = TypedDict("_CancelOrderRequestOptional", {'orderId': 'str | None', 'clientId': 'str | None'}, total=False)
class CancelOrderRequest(_CancelOrderRequestOptional):
    marketId: int

_OrderResponseOptional = TypedDict("_OrderResponseOptional", {'orderId': 'str | None', 'clientId': 'str | None'}, total=False)
class OrderResponse(_OrderResponseOptional):
    status: str

