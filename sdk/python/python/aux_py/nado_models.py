"""Nado REST wire models generated from src/exchanges/nado/schema/nado.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class MarketsRequest(TypedDict):
    pass

class OrderBookRequest(TypedDict):
    product_id: int
    depth: int

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    ticker_id: str

class AccountRequest(TypedDict):
    subaccount: str

class PositionsRequest(TypedDict):
    subaccount: str

class OpenOrdersRequest(TypedDict):
    sender: str
    product_id: int

class ContractsRequest(TypedDict):
    pass

class OrderStatusRequest(TypedDict):
    body: Any

class PlaceOrderRequest(TypedDict):
    body: PlaceOrder

class CancelOrderRequest(TypedDict):
    body: CancelOrder

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'subaccount': 'str | None', 'product_id': 'int | None', 'snapshot_timestamp': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    type: str

_PlaceOrderOptional = TypedDict("_PlaceOrderOptional", {'id': 'int | None'}, total=False)
class PlaceOrder(_PlaceOrderOptional):
    product_id: int
    order: Order

class CancelOrder(TypedDict):
    tx: Cancellation

_GatewayReplyOptional = TypedDict("_GatewayReplyOptional", {'data': 'Any | None', 'error': 'Any | None', 'request_type': 'str | None'}, total=False)
class GatewayReply(_GatewayReplyOptional):
    status: str

class Order(TypedDict):
    sender: str
    priceX18: str
    amount: str
    expiration: str
    nonce: str
    appendix: str

class Cancellation(TypedDict):
    sender: str
    productIds: list[int]
    digests: list[str]
    nonce: str

