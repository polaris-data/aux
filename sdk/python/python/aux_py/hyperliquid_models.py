"""Hyperliquid REST wire models generated from src/exchanges/hyperliquid/schema/hyperliquid.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

from typing import Literal
Tif = Literal["Alo", "Ioc", "Gtc"]
Tpsl = Literal["tp", "sl"]
NamedGrouping = Literal["na", "normalTpsl", "positionTpsl"]
CrossKind = Literal["cross"]
IsolatedKind = Literal["isolated"]
OkStatus = Literal["ok"]
ErrStatus = Literal["err"]
_MetaOptional = TypedDict("_MetaOptional", {'marginTables': 'list[tuple[int, MarginTable]]'}, total=False)
class Meta(_MetaOptional):
    universe: list[Asset]

_AssetOptional = TypedDict("_AssetOptional", {'onlyIsolated': 'bool | None', 'isDelisted': 'bool | None', 'marginTableId': 'int | None', 'marginMode': 'str | None'}, total=False)
class Asset(_AssetOptional):
    name: str
    szDecimals: int
    maxLeverage: int

class MarginTable(TypedDict):
    description: str
    marginTiers: list[MarginTier]

class MarginTier(TypedDict):
    lowerBound: str
    maxLeverage: int

class L2Book(TypedDict):
    coin: str
    time: int
    levels: list[list[Level]]

class Level(TypedDict):
    px: str
    sz: str
    n: int

_MetaRequestOptional = TypedDict("_MetaRequestOptional", {'dex': 'str | None'}, total=False)
class MetaRequest(_MetaRequestOptional):
    pass

_L2BookRequestOptional = TypedDict("_L2BookRequestOptional", {'nSigFigs': 'int | None', 'mantissa': 'int | None'}, total=False)
class L2BookRequest(_L2BookRequestOptional):
    coin: str

_OrderOptional = TypedDict("_OrderOptional", {'c': 'Cloid | None'}, total=False)
class Order(_OrderOptional):
    a: int
    b: bool
    p: str
    s: str
    r: bool
    t: OrderType

class SubBuilder(TypedDict):
    b: Address
    s: int

_BuilderFeeOptional = TypedDict("_BuilderFeeOptional", {'s': 'list[SubBuilder] | None'}, total=False)
class BuilderFee(_BuilderFeeOptional):
    b: Address
    f: int

_OrderRequestOptional = TypedDict("_OrderRequestOptional", {'grouping': 'Grouping | None', 'builder': 'BuilderFee | None', 'options': 'ExecutionOptions | None'}, total=False)
class OrderRequest(_OrderRequestOptional):
    orders: list[Order]

class Cancel(TypedDict):
    a: int
    o: int

_CancelRequestOptional = TypedDict("_CancelRequestOptional", {'options': 'ExecutionOptions | None'}, total=False)
class CancelRequest(_CancelRequestOptional):
    cancels: list[Cancel]

class CancelByCloid(TypedDict):
    asset: int
    cloid: Cloid

_CancelByCloidRequestOptional = TypedDict("_CancelByCloidRequestOptional", {'options': 'ExecutionOptions | None'}, total=False)
class CancelByCloidRequest(_CancelByCloidRequestOptional):
    cancels: list[CancelByCloid]

_ExecutionOptionsOptional = TypedDict("_ExecutionOptionsOptional", {'nonce': 'int | None', 'expires_after': 'int | None'}, total=False)
class ExecutionOptions(_ExecutionOptionsOptional):
    pass

class BatchResponse(TypedDict):
    type: str
    data: BatchData

class BatchData(TypedDict):
    statuses: list[ItemStatus]

_RestingOrderOptional = TypedDict("_RestingOrderOptional", {'cloid': 'Cloid | None'}, total=False)
class RestingOrder(_RestingOrderOptional):
    oid: int

_FilledOrderOptional = TypedDict("_FilledOrderOptional", {'cloid': 'Cloid | None'}, total=False)
class FilledOrder(_FilledOrderOptional):
    totalSz: str
    avgPx: str
    oid: int

_UserStateRequestOptional = TypedDict("_UserStateRequestOptional", {'dex': 'str | None'}, total=False)
class UserStateRequest(_UserStateRequestOptional):
    user: Address

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'dex': 'str | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    user: Address

_UserStateOptional = TypedDict("_UserStateOptional", {'time': 'int | None'}, total=False)
class UserState(_UserStateOptional):
    assetPositions: list[AssetPosition]
    crossMaintenanceMarginUsed: str
    crossMarginSummary: MarginSummary
    marginSummary: MarginSummary
    withdrawable: str

class MarginSummary(TypedDict):
    accountValue: str
    totalMarginUsed: str
    totalNtlPos: str
    totalRawUsd: str

class AssetPosition(TypedDict):
    type: str
    position: Position

_PositionOptional = TypedDict("_PositionOptional", {'entryPx': 'str | None', 'liquidationPx': 'str | None'}, total=False)
class Position(_PositionOptional):
    coin: str
    cumFunding: CumulativeFunding
    leverage: Leverage
    marginUsed: str
    maxLeverage: int
    positionValue: str
    returnOnEquity: str
    szi: str
    unrealizedPnl: str

class CumulativeFunding(TypedDict):
    allTime: str
    sinceChange: str
    sinceOpen: str

_OpenOrderOptional = TypedDict("_OpenOrderOptional", {'cloid': 'Cloid | None', 'origSz': 'str | None'}, total=False)
class OpenOrder(_OpenOrderOptional):
    coin: str
    limitPx: str
    oid: int
    side: str
    sz: str
    timestamp: int

class LimitDetails(TypedDict):
    tif: Tif

class TriggerDetails(TypedDict):
    isMarket: bool
    triggerPx: str
    tpsl: Tpsl

class LimitOrder(TypedDict):
    limit: LimitDetails

class TriggerOrder(TypedDict):
    trigger: TriggerDetails

class PriorityGrouping(TypedDict):
    p: int

class RestingStatus(TypedDict):
    resting: RestingOrder

class FilledStatus(TypedDict):
    filled: FilledOrder

class ErrorStatus(TypedDict):
    error: str

class OkResponse(TypedDict):
    status: OkStatus
    response: BatchResponse

class ErrResponse(TypedDict):
    status: ErrStatus
    response: str

class CrossLeverage(TypedDict):
    type: CrossKind
    value: int

class IsolatedLeverage(TypedDict):
    type: IsolatedKind
    value: int
    rawUsd: str

OrderType = LimitOrder | TriggerOrder
Grouping = NamedGrouping | PriorityGrouping
ItemStatus = RestingStatus | FilledStatus | ErrorStatus | str | Any
TradingResponse = OkResponse | ErrResponse
Leverage = CrossLeverage | IsolatedLeverage
Cloid = str
Address = str
