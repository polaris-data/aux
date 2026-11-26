"""Reya REST wire models generated from src/exchanges/reya/schema/reya.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'symbol': 'str | None', 'address': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

class MarketsRequest(TypedDict):
    pass

class MarketSummariesRequest(TypedDict):
    pass

class MarketSummaryRequest(TypedDict):
    symbol: str

_MarketExecutionsRequestOptional = TypedDict("_MarketExecutionsRequestOptional", {'startTime': 'int | None', 'endTime': 'int | None', 'type': 'str | None', 'limit': 'int | None', 'cursor': 'str | None'}, total=False)
class MarketExecutionsRequest(_MarketExecutionsRequestOptional):
    symbol: str

_WalletExecutionsRequestOptional = TypedDict("_WalletExecutionsRequestOptional", {'startTime': 'int | None', 'endTime': 'int | None', 'type': 'str | None', 'limit': 'int | None', 'cursor': 'str | None'}, total=False)
class WalletExecutionsRequest(_WalletExecutionsRequestOptional):
    address: str

MarketsResponse = Any
MarketSummariesResponse = Any
MarketSummaryResponse = Any
MarketExecutionsResponse = Any
WalletExecutionsResponse = Any
