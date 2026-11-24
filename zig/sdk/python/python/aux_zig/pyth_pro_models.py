"""PythPro REST wire models generated from schema/pyth_pro.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class LiveRequest(TypedDict):
    pass

class HealthResponse(TypedDict):
    status: int
    headers: list[list[str]]
    data: str

class ReadyRequest(TypedDict):
    pass

_SymbolsRequestOptional = TypedDict("_SymbolsRequestOptional", {'query': 'str | None', 'asset_type': 'list[str] | None', 'instrument_type': 'list[str] | None', 'symbol_chain_id': 'list[str] | None', 'entitled_only': 'bool | None'}, total=False)
class SymbolsRequest(_SymbolsRequestOptional):
    pass

class NativeResponse(TypedDict):
    status: int
    headers: list[list[str]]
    data: Any

class ConfigRequest(TypedDict):
    channel: str

_SearchRequestOptional = TypedDict("_SearchRequestOptional", {'type': 'str | None', 'exchange': 'str | None', 'limit': 'int | None'}, total=False)
class SearchRequest(_SearchRequestOptional):
    channel: str
    query: str

class ResolveSymbolRequest(TypedDict):
    channel: str
    symbol: str

_HistoryRequestOptional = TypedDict("_HistoryRequestOptional", {'split_adjusted': 'bool | None'}, total=False)
_HistoryRequestWireKeys = TypedDict("_HistoryRequestWireKeys", {'from': 'int'})
class HistoryRequest(_HistoryRequestWireKeys, _HistoryRequestOptional):
    channel: str
    symbol: str
    to: int
    resolution: str

_PriceRequestOptional = TypedDict("_PriceRequestOptional", {'split_adjusted': 'bool | None'}, total=False)
class PriceRequest(_PriceRequestOptional):
    channel: str
    ids: list[int]
    timestamp: int

_PriceRangeRequestOptional = TypedDict("_PriceRangeRequestOptional", {'limit': 'int | None', 'after': 'str | None', 'split_adjusted': 'bool | None'}, total=False)
class PriceRangeRequest(_PriceRangeRequestOptional):
    channel: str
    ids: list[int]
    start_timestamp: int
    end_timestamp: int

_PublisherUpdatesRequestOptional = TypedDict("_PublisherUpdatesRequestOptional", {'price_feed_id': 'list[int] | None', 'min_timestamp': 'str | None', 'max_timestamp': 'str | None', 'status': 'str | None', 'status_reason': 'str | None', 'status_reason_unset': 'bool | None', 'limit': 'int | None'}, total=False)
class PublisherUpdatesRequest(_PublisherUpdatesRequestOptional):
    sort: str

_LatestPriceRequestOptional = TypedDict("_LatestPriceRequestOptional", {'priceFeedIds': 'list[int] | None', 'properties': 'list[str] | None', 'parsed': 'bool | None', 'jsonBinaryEncoding': 'str | None', 'symbols': 'list[str] | None'}, total=False)
class LatestPriceRequest(_LatestPriceRequestOptional):
    channel: str
    formats: list[str]

_PriceAtTimestampRequestOptional = TypedDict("_PriceAtTimestampRequestOptional", {'priceFeedIds': 'list[int] | None', 'properties': 'list[str] | None', 'parsed': 'bool | None', 'jsonBinaryEncoding': 'str | None', 'symbols': 'list[str] | None'}, total=False)
class PriceAtTimestampRequest(_PriceAtTimestampRequestOptional):
    channel: str
    formats: list[str]
    timestamp: int

class ReducePriceRequest(TypedDict):
    payload: Any
    priceFeedIds: list[int]

class PublisherStakeCapsRequest(TypedDict):
    pass

class GuardianSetUpgradeRequest(TypedDict):
    pass

