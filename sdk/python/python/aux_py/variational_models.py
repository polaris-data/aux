"""Variational REST wire models generated from src/exchanges/variational/schema/variational.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class MarketStatsRequest(TypedDict):
    pass

class FundingLast(TypedDict):
    annualized_rate: str
    interval_rate: str

class FundingNext(TypedDict):
    annualized_rate: str
    interval_rate: str

class Funding(TypedDict):
    last: FundingLast
    next: FundingNext
    interval_seconds: int
    next_timestamp: int

class MarketQuotesBase(TypedDict):
    bid: str
    ask: str

class MarketQuotesSize1K(TypedDict):
    bid: str
    ask: str

class MarketQuotesSize100K(TypedDict):
    bid: str
    ask: str

class MarketQuotes(TypedDict):
    updated_at: str
    base: MarketQuotesBase | None
    size_1k: MarketQuotesSize1K | None
    size_100k: MarketQuotesSize100K | None

class MarketListingOpenInterest(TypedDict):
    long_open_interest: str
    short_open_interest: str

class MarketListing(TypedDict):
    ticker: str
    contract_type: str
    name: str
    mark_price: str
    volume_24h: str
    open_interest: MarketListingOpenInterest
    funding: Funding | None
    base_spread_bps: str
    quotes: MarketQuotes | None

class MarketStatsResponseLossRefund(TypedDict):
    pool_size: str
    refunded_24h: str

class MarketStatsResponse(TypedDict):
    total_volume_24h: str
    cumulative_volume: str
    tvl: str
    open_interest: str
    num_markets: int
    loss_refund: MarketStatsResponseLossRefund
    listings: list[MarketListing]

