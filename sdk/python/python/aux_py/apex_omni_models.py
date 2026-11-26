"""ApexOmni REST wire models generated from src/exchanges/apex_omni/schema/apex_omni.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'symbol': 'str | None', 'depth': 'int | None', 'frequency': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

class MarketsRequest(TypedDict):
    pass

class TickerRequest(TypedDict):
    symbol: str

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    symbol: str

MarketsResponse = Any
TickerResponse = Any
TradesResponse = Any
