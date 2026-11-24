"""Qfex REST wire models generated from schema/qfex.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SymbolOptional = TypedDict("_SymbolOptional", {'min_quantity': 'str', 'max_quantity': 'str', 'status': 'str'}, total=False)
class Symbol(_SymbolOptional):
    clobPairId: str
    symbol: str
    tick_size: str
    lot_size: str

_RefdataOptional = TypedDict("_RefdataOptional", {'data': 'list[Symbol]'}, total=False)
class Refdata(_RefdataOptional):
    type: str

_ContractsOptional = TypedDict("_ContractsOptional", {'data': 'list[Any]'}, total=False)
class Contracts(_ContractsOptional):
    pass

_OrderBookOptional = TypedDict("_OrderBookOptional", {'bids': 'list[list[str]]', 'asks': 'list[list[str]]'}, total=False)
class OrderBook(_OrderBookOptional):
    ticker_id: str
    timestamp: int

class Position(TypedDict):
    symbol: str
    position: int | Decimal
    realised_pnl: int | Decimal
    unrealised_pnl: int | Decimal
    average_price: int | Decimal

class Balance(TypedDict):
    available_balance: int | Decimal
    deposit: int | Decimal

_PositionsBalanceOptional = TypedDict("_PositionsBalanceOptional", {'positions': 'list[Position]'}, total=False)
class PositionsBalance(_PositionsBalanceOptional):
    balance: Balance

_RefdataRequestOptional = TypedDict("_RefdataRequestOptional", {'ticker': 'str | None'}, total=False)
class RefdataRequest(_RefdataRequestOptional):
    pass

_ContractsRequestOptional = TypedDict("_ContractsRequestOptional", {'symbol': 'str | None'}, total=False)
class ContractsRequest(_ContractsRequestOptional):
    pass

class OrderBookRequest(TypedDict):
    symbol: str

class EmptyRequest(TypedDict):
    pass

