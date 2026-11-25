"""Injective REST wire models generated from src/exchanges/injective/schema/injective.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_MarketsRequestOptional = TypedDict("_MarketsRequestOptional", {'marketStatus': 'str | None', 'quoteDenom': 'str | None'}, total=False)
class MarketsRequest(_MarketsRequestOptional):
    pass

_OrderBookRequestOptional = TypedDict("_OrderBookRequestOptional", {'depth': 'int | None'}, total=False)
class OrderBookRequest(_OrderBookRequestOptional):
    marketId: str

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None', 'subaccountId': 'str | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    marketId: str

class AccountRequest(TypedDict):
    subaccountId: str

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'marketId': 'str | None', 'skip': 'int | None', 'limit': 'int | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    subaccountId: str

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'marketId': 'str | None', 'skip': 'int | None', 'limit': 'int | None', 'cid': 'str | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    subaccountId: str

class TransactionRequest(TypedDict):
    hash: str

class PendingRequest(TypedDict):
    pass

class PlaceOrderRequest(TypedDict):
    body: Order

class CancelOrderRequest(TypedDict):
    body: CancelOrder

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'market_id': 'str | None', 'subaccount_id': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

_OrderOptional = TypedDict("_OrderOptional", {'client_order_id': 'str | None', 'fee_recipient': 'str | None'}, total=False)
class Order(_OrderOptional):
    market_id: str
    subaccount_id: str
    gas_limit: str
    fee_amount: str
    quote_decimals: int
    price: str
    quantity: str
    margin: str
    side: str
    kind: str

_CancelOrderOptional = TypedDict("_CancelOrderOptional", {'client_order_id': 'str | None', 'order_hash': 'str | None'}, total=False)
class CancelOrder(_CancelOrderOptional):
    market_id: str
    subaccount_id: str
    gas_limit: str
    fee_amount: str

class Markets(TypedDict):
    markets: list[Any]

class Trades(TypedDict):
    trades: list[Any]

class Positions(TypedDict):
    positions: list[Any]

class Orders(TypedDict):
    orders: list[Any]

class Orderbook(TypedDict):
    orderbook: Any

class TransactionResult(TypedDict):
    tx_response: Any

