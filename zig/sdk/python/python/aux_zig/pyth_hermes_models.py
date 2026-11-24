"""PythHermes REST wire models generated from schema/pyth_hermes.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class HermesLiveRequest(TypedDict):
    pass

class HealthResponse(TypedDict):
    status: int
    headers: list[list[str]]
    data: str

class HermesReadyRequest(TypedDict):
    pass

_PriceFeedsRequestOptional = TypedDict("_PriceFeedsRequestOptional", {'query': 'str | None', 'asset_type': 'str | None'}, total=False)
class PriceFeedsRequest(_PriceFeedsRequestOptional):
    pass

class NativeResponse(TypedDict):
    status: int
    headers: list[list[str]]
    data: Any

_LatestPriceUpdatesRequestOptional = TypedDict("_LatestPriceUpdatesRequestOptional", {'encoding': 'str | None', 'parsed': 'bool | None', 'ignore_invalid_price_ids': 'bool | None'}, total=False)
class LatestPriceUpdatesRequest(_LatestPriceUpdatesRequestOptional):
    ids: list[str]

_PriceUpdatesAtTimestampRequestOptional = TypedDict("_PriceUpdatesAtTimestampRequestOptional", {'encoding': 'str | None', 'parsed': 'bool | None', 'ignore_invalid_price_ids': 'bool | None'}, total=False)
class PriceUpdatesAtTimestampRequest(_PriceUpdatesAtTimestampRequestOptional):
    publish_time: int
    ids: list[str]

class LatestTwapsRequest(TypedDict):
    window_seconds: int

_LatestPublisherStakeCapsRequestOptional = TypedDict("_LatestPublisherStakeCapsRequestOptional", {'encoding': 'str | None', 'parsed': 'bool | None'}, total=False)
class LatestPublisherStakeCapsRequest(_LatestPublisherStakeCapsRequestOptional):
    pass

