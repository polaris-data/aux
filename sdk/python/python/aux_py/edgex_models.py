"""Edgex REST wire models generated from src/exchanges/edgex/schema/edgex.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'contract_id': 'str | None', 'depth': 'int | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

class MarketsRequest(TypedDict):
    pass

_MarketsResponseDataOptional = TypedDict("_MarketsResponseDataOptional", {'contractList': 'list[Market] | None'}, total=False)
class MarketsResponseData(_MarketsResponseDataOptional):
    pass

_MarketsResponseOptional = TypedDict("_MarketsResponseOptional", {'data': 'MarketsResponseData | None'}, total=False)
class MarketsResponse(_MarketsResponseOptional):
    code: str

class OrderBookRequest(TypedDict):
    contractId: str
    level: int

_OrderBookResponseOptional = TypedDict("_OrderBookResponseOptional", {'data': 'list[Any] | None'}, total=False)
class OrderBookResponse(_OrderBookResponseOptional):
    code: str

class AccountRequest(TypedDict):
    accountId: str

_AccountResponseDataOptional = TypedDict("_AccountResponseDataOptional", {'collateralAssetList': 'list[Any] | None', 'positionAssetList': 'list[Any] | None'}, total=False)
class AccountResponseData(_AccountResponseDataOptional):
    pass

_AccountResponseOptional = TypedDict("_AccountResponseOptional", {'data': 'AccountResponseData | None'}, total=False)
class AccountResponse(_AccountResponseOptional):
    code: str

class PositionsRequest(TypedDict):
    accountId: str
    contractIdList: str

_PositionsResponseOptional = TypedDict("_PositionsResponseOptional", {'data': 'list[Any] | None'}, total=False)
class PositionsResponse(_PositionsResponseOptional):
    code: str

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'filterContractIdList': 'list[str] | None', 'size': 'str | None', 'offsetData': 'str | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    accountId: str

_OpenOrdersResponseDataOptional = TypedDict("_OpenOrdersResponseDataOptional", {'dataList': 'list[Any] | None'}, total=False)
class OpenOrdersResponseData(_OpenOrdersResponseDataOptional):
    pass

_OpenOrdersResponseOptional = TypedDict("_OpenOrdersResponseOptional", {'data': 'OpenOrdersResponseData | None'}, total=False)
class OpenOrdersResponse(_OpenOrdersResponseOptional):
    code: str

_OrderStatusRequestOptional = TypedDict("_OrderStatusRequestOptional", {'orderIdList': 'list[str] | None'}, total=False)
class OrderStatusRequest(_OrderStatusRequestOptional):
    accountId: str

_OrderStatusResponseOptional = TypedDict("_OrderStatusResponseOptional", {'data': 'list[Any] | None'}, total=False)
class OrderStatusResponse(_OrderStatusResponseOptional):
    code: str

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'filterContractIdList': 'list[str] | None', 'size': 'str | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    accountId: str

_FillsResponseDataOptional = TypedDict("_FillsResponseDataOptional", {'dataList': 'list[Any] | None'}, total=False)
class FillsResponseData(_FillsResponseDataOptional):
    pass

_FillsResponseOptional = TypedDict("_FillsResponseOptional", {'data': 'FillsResponseData | None'}, total=False)
class FillsResponse(_FillsResponseOptional):
    code: str

_PlaceOrderBodyOptional = TypedDict("_PlaceOrderBodyOptional", {'price': 'str | None', 'clientOrderId': 'str | None', 'timeInForce': 'str | None', 'price_bound': 'str | None', 'reduceOnly': 'bool | None'}, total=False)
class PlaceOrderBody(_PlaceOrderBodyOptional):
    accountId: str
    contractId: str
    side: str
    size: str
    type: str

class PlaceOrderRequest(TypedDict):
    body: PlaceOrderBody

_PlaceOrderResponseDataOptional = TypedDict("_PlaceOrderResponseDataOptional", {'orderId': 'str | None'}, total=False)
class PlaceOrderResponseData(_PlaceOrderResponseDataOptional):
    pass

_PlaceOrderResponseOptional = TypedDict("_PlaceOrderResponseOptional", {'data': 'PlaceOrderResponseData | None'}, total=False)
class PlaceOrderResponse(_PlaceOrderResponseOptional):
    code: str

class CancelOrderBody(TypedDict):
    accountId: str
    orderIdList: Any

class CancelOrderRequest(TypedDict):
    body: CancelOrderBody

_CancelOrderResponseDataOptional = TypedDict("_CancelOrderResponseDataOptional", {'cancelResultMap': 'dict[str, str] | None'}, total=False)
class CancelOrderResponseData(_CancelOrderResponseDataOptional):
    pass

_CancelOrderResponseOptional = TypedDict("_CancelOrderResponseOptional", {'data': 'CancelOrderResponseData | None'}, total=False)
class CancelOrderResponse(_CancelOrderResponseOptional):
    code: str

_MarketOptional = TypedDict("_MarketOptional", {'contractName': 'str | None', 'quoteCoinId': 'str | None', 'tickSize': 'str | None', 'stepSize': 'str | None', 'resolution': 'str | None'}, total=False)
class Market(_MarketOptional):
    contractId: str

_OrderByClientIdRequestOptional = TypedDict("_OrderByClientIdRequestOptional", {'clientOrderIdList': 'list[str] | None'}, total=False)
class OrderByClientIdRequest(_OrderByClientIdRequestOptional):
    accountId: str

