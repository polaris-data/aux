"""ChainlinkStreams REST wire models generated from src/data/chainlink/schema/chainlink_streams.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_DiscoveryRequestOptional = TypedDict("_DiscoveryRequestOptional", {'base_asset': 'str | None', 'quote_asset': 'str | None', 'asset_class': 'str | None', 'feed_type': 'str | None', 'attribute_type': 'str | None', 'status': 'str | None', 'network_type': 'str | None', 'created_after': 'str | None', 'created_before': 'str | None', 'expected_decommission_after': 'str | None', 'expected_decommission_before': 'str | None', 'decommissioned_after': 'str | None', 'decommissioned_before': 'str | None', 'hidden': 'bool | None'}, total=False)
class DiscoveryRequest(_DiscoveryRequestOptional):
    pass

class NativeResponse(TypedDict):
    status: int
    headers: list[list[str]]
    data: Any

class ReportRequest(TypedDict):
    feedID: str
    timestamp: int

class LatestReportRequest(TypedDict):
    feedID: str

class BulkReportsRequest(TypedDict):
    feedIDs: list[str]
    timestamp: int

_ReportPageRequestOptional = TypedDict("_ReportPageRequestOptional", {'limit': 'int | None'}, total=False)
class ReportPageRequest(_ReportPageRequestOptional):
    feedID: str
    startTimestamp: int

