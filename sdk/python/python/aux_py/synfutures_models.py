"""Synfutures REST wire models generated from src/exchanges/synfutures/schema/synfutures.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_VirtualTradesRequestOptional = TypedDict("_VirtualTradesRequestOptional", {'last': 'str | None', 'first': 'int | None'}, total=False)
class VirtualTradesRequest(_VirtualTradesRequestOptional):
    since: int
    until: int

_CandlesRequestOptional = TypedDict("_CandlesRequestOptional", {'last': 'str | None', 'first': 'int | None'}, total=False)
class CandlesRequest(_CandlesRequestOptional):
    since: int
    until: int
    interval: str
    amm_ids: list[str]

GraphResponse = Any
