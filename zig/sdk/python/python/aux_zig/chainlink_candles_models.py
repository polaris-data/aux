"""ChainlinkCandles REST wire models generated from schema/chainlink_candles.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class AuthorizeRequest(TypedDict):
    login: str
    password: str

class NativeResponse(TypedDict):
    status: int
    headers: list[list[str]]
    data: Any

_SymbolInfoRequestOptional = TypedDict("_SymbolInfoRequestOptional", {'group': 'str | None'}, total=False)
class SymbolInfoRequest(_SymbolInfoRequestOptional):
    pass

class GroupsRequest(TypedDict):
    pass

_CandleHistoryRequestWireKeys = TypedDict("_CandleHistoryRequestWireKeys", {'from': 'int'})
class CandleHistoryRequest(_CandleHistoryRequestWireKeys):
    symbol: str
    resolution: str
    to: int

_CandleHistoryRowsRequestWireKeys = TypedDict("_CandleHistoryRowsRequestWireKeys", {'from': 'int'})
class CandleHistoryRowsRequest(_CandleHistoryRowsRequestWireKeys):
    symbol: str
    resolution: str
    to: int

