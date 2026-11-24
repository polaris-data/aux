"""Metric REST wire models generated from src/exchanges/metric/schema/metric.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

from typing import Literal
TradeType = Literal["exact_in", "exact_out"]
_MetadataRequestOptional = TypedDict("_MetadataRequestOptional", {'count': 'int | None', 'offset': 'int | None', 'sortBy': 'str | None', 'sortOrder': 'str | None', 'include24h': 'bool | None', 'minTvlFiat': 'int | Decimal | None', 'tier': 'str | None', 'lockedPricingOnly': 'bool | None', 'depositAllowlistEnabled': 'bool | None', 'blacklistedOnly': 'bool | None', 'addresses': 'str | None', 'contract_version': 'str | None', 'fiat': 'str | None', 'chainIds': 'str | None'}, total=False)
class MetadataRequest(_MetadataRequestOptional):
    chainId: int

class Chain(TypedDict):
    type: str
    chainId: int
    name: str

class ChainsResponse(TypedDict):
    chains: list[Chain]

_PoolMetadataOptional = TypedDict("_PoolMetadataOptional", {'priceProviderAddress': 'str | None'}, total=False)
class PoolMetadata(_PoolMetadataOptional):
    poolAddress: str
    pair: str
    chain: str
    token0: str
    token1: str
    token0Decimals: int
    token1Decimals: int

_MetadataResponseOptional = TypedDict("_MetadataResponseOptional", {'nextOffset': 'int | None'}, total=False)
class MetadataResponse(_MetadataResponseOptional):
    data: list[PoolMetadata]
    total: int

class DepthLevel(TypedDict):
    binIdx: int
    price: str
    cumulativeVolume: str
    cumulativeInputVolume: str
    priceImpactE6: str

class Depth(TypedDict):
    bids: list[DepthLevel]
    asks: list[DepthLevel]

_BidAskResponseOptional = TypedDict("_BidAskResponseOptional", {'poolAddress': 'str | None', 'pair': 'str | None', 'bidAdj': 'str | None', 'askAdj': 'str | None', 'totalToken0Available': 'str | None', 'totalToken1Available': 'str | None', 'serverTs': 'int | None', 'depth': 'Depth | None'}, total=False)
class BidAskResponse(_BidAskResponseOptional):
    priceProviderStatus: str

_BatchBidAskRequestOptional = TypedDict("_BatchBidAskRequestOptional", {'depth': 'bool | None'}, total=False)
class BatchBidAskRequest(_BatchBidAskRequestOptional):
    chainId: int
    pools: list[str]

class BatchBidAskResponse(TypedDict):
    results: list[PoolQuote]

_PoolQuoteOptional = TypedDict("_PoolQuoteOptional", {'quote': 'BidAskResponse | None', 'error': 'PoolQuoteError | None'}, total=False)
class PoolQuote(_PoolQuoteOptional):
    poolAddress: str

class PoolQuoteError(TypedDict):
    status: int
    error: str

_RouteRequestOptional = TypedDict("_RouteRequestOptional", {'tradeType': 'TradeType | None', 'amountIn': 'str | None', 'amountOut': 'str | None', 'maxRoutes': 'int | None', 'poolAddress': 'str | None'}, total=False)
class RouteRequest(_RouteRequestOptional):
    chainId: int
    tokenIn: str
    tokenOut: str

class RouteLeg(TypedDict):
    tradeType: TradeType
    pair: str
    effectivePriceX64: str
    priceImpactE6: str
    poolAddress: str
    zeroForOne: bool
    tokenIn: str
    tokenOut: str
    amountIn: str
    amountOut: str
    priceLimitX64: str

class RouteOption(TypedDict):
    tradeType: TradeType
    routeType: str
    effectivePriceX64: str
    priceImpactE6: str
    amountIn: str
    amountOut: str
    legs: list[RouteLeg]

class RouteResponse(TypedDict):
    tokenIn: str
    tokenOut: str
    effectivePriceX64: str
    priceImpactBestRoute: str
    candidatesConsidered: int
    tradeType: TradeType
    amountIn: str
    amountOut: str
    routes: list[RouteOption]

_PammQuoteRequestOptional = TypedDict("_PammQuoteRequestOptional", {'disableCrossRegionSharing': 'bool | None', 'responseTimeoutMs': 'int | None', 'maxTimestamp': 'int | None', 'alwaysInclude': 'bool | None', 'quoteAddress': 'str | None', 'poolAddresses': 'list[str] | None', 'assetPairs': 'list[str] | None'}, total=False)
class PammQuoteRequest(_PammQuoteRequestOptional):
    chainId: int
    tx: str
    blockNumber: int
    replacementUuid: str
    replacementSeqNumber: int

_BuilderOutcomeOptional = TypedDict("_BuilderOutcomeOptional", {'pending': 'bool', 'error': 'str | None', 'bundleHash': 'str | None'}, total=False)
class BuilderOutcome(_BuilderOutcomeOptional):
    provider: str
    accepted: bool

class PammQuoteResponse(TypedDict):
    accepted: bool
    providers: list[BuilderOutcome]

class EmptyRequest(TypedDict):
    pass

_BidAskRequestOptional = TypedDict("_BidAskRequestOptional", {'depth': 'bool | None'}, total=False)
class BidAskRequest(_BidAskRequestOptional):
    chainId: int
    pool: str

_CrossChainMetadataRequestOptional = TypedDict("_CrossChainMetadataRequestOptional", {'count': 'int | None', 'offset': 'int | None', 'sortBy': 'str | None', 'sortOrder': 'str | None', 'include24h': 'bool | None', 'minTvlFiat': 'int | Decimal | None', 'tier': 'str | None', 'lockedPricingOnly': 'bool | None', 'depositAllowlistEnabled': 'bool | None', 'blacklistedOnly': 'bool | None', 'addresses': 'str | None', 'contract_version': 'str | None', 'fiat': 'str | None', 'chainIds': 'str | None'}, total=False)
class CrossChainMetadataRequest(_CrossChainMetadataRequestOptional):
    pass

FactorySettingsResponse = dict[str, Any]
