"""Kuru REST wire models generated from src/exchanges/kuru/schema/kuru.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'marketAddress': 'str | None', 'feedEpoch': 'int | None', 'seq': 'int | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

_MarketsRequestOptional = TypedDict("_MarketsRequestOptional", {'status': 'str | None', 'limit': 'int | None', 'cursor': 'str | None'}, total=False)
class MarketsRequest(_MarketsRequestOptional):
    pass

class MarketRequest(TypedDict):
    marketAddress: str

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None', 'cursor': 'str | None', 'from': 'int | None', 'to': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    marketAddress: str

MarketsResponse = Any
MarketResponse = Any
TradesResponse = Any
