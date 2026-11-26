"""DydxV4 REST wire models generated from src/exchanges/dydx_v4/schema/dydx_v4.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'ticker': 'str | None', 'id': 'str | None', 'address': 'str | None', 'subaccount_number': 'int | None', 'batched': 'bool | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

_MarketsRequestOptional = TypedDict("_MarketsRequestOptional", {'ticker': 'str | None', 'limit': 'int | None'}, total=False)
class MarketsRequest(_MarketsRequestOptional):
    pass

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None', 'createdBeforeOrAtHeight': 'int | None', 'createdBeforeOrAt': 'str | None', 'page': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    ticker: str

class AccountRequest(TypedDict):
    address: str
    subaccountNumber: str

class BalancesRequest(TypedDict):
    address: str
    subaccountNumber: int

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'status': 'str | None', 'limit': 'int | None', 'createdBeforeOrAtHeight': 'int | None', 'createdBeforeOrAt': 'str | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    address: str
    subaccountNumber: int

class OrderStatusRequest(TypedDict):
    orderId: str

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'ticker': 'str | None', 'side': 'str | None', 'type': 'str | None', 'limit': 'int | None', 'returnLatestOrders': 'bool | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    address: str
    subaccountNumber: int

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'ticker': 'str | None', 'side': 'str | None', 'type': 'str | None', 'status': 'str | None', 'limit': 'int | None', 'includeTypes': 'list[str] | None', 'excludeTypes': 'list[str] | None', 'goodTilBlockBeforeOrAt': 'int | None', 'goodTilBlockAfter': 'int | None', 'goodTilBlockTimeBeforeOrAt': 'str | None', 'goodTilBlockTimeAfter': 'str | None', 'returnLatestOrders': 'bool | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    address: str
    subaccountNumber: int

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'market': 'str | None', 'marketType': 'str | None', 'includeTypes': 'list[str] | None', 'excludeTypes': 'list[str] | None', 'limit': 'int | None', 'createdBeforeOrAtHeight': 'int | None', 'createdBeforeOrAt': 'str | None', 'page': 'int | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    address: str
    subaccountNumber: int

class TransactionRequest(TypedDict):
    hash: str

class PlaceOrderRequest(TypedDict):
    body: Any

class ReplaceOrderRequest(TypedDict):
    body: Any

class CancelOrderRequest(TypedDict):
    body: Any

class CancelOrdersRequest(TypedDict):
    body: Any

class PlaceOrdersRequest(TypedDict):
    body: Any

class CancelStatefulOrdersRequest(TypedDict):
    body: Any

MarketsResponse = Any
TradesResponse = Any
AccountResponse = Any
BalancesResponse = Any
PositionsResponse = Any
OrderStatusResponse = Any
OpenOrdersResponse = Any
OrderHistoryResponse = Any
FillsResponse = Any
TransactionResponse = Any
PlaceOrderResponse = Any
ReplaceOrderResponse = Any
CancelOrderResponse = Any
CancelOrdersResponse = Any
PlaceOrdersResponse = Any
CancelStatefulOrdersResponse = Any
