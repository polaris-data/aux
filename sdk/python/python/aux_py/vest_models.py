"""Vest REST wire models generated from src/exchanges/vest/schema/vest.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_ExchangeInfoRequestOptional = TypedDict("_ExchangeInfoRequestOptional", {'account_group': 'int | None'}, total=False)
class ExchangeInfoRequest(_ExchangeInfoRequestOptional):
    pass

ExchangeInfoResponse = Any
