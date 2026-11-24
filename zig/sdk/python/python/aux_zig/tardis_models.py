"""Tardis REST wire models generated from schema/tardis.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class EmptyRequest(TypedDict):
    pass

class ExchangeRequest(TypedDict):
    exchange: str

class InstrumentRequest(TypedDict):
    exchange: str
    symbol: str

class InstrumentsRequest(TypedDict):
    exchange: str

Params = dict[str, Any]
