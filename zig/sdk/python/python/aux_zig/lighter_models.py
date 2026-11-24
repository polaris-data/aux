"""Lighter REST wire models generated from src/exchanges/lighter/schema/lighter.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

from typing import Literal
AccountSelector = Literal["index", "l1_address"]
class OrderBook(TypedDict):
    symbol: str
    market_id: int
    market_type: str
    status: str
    supported_size_decimals: int
    supported_price_decimals: int
    min_base_amount: str
    min_quote_amount: str

class OrderBooks(TypedDict):
    code: int
    order_books: list[OrderBook]

class SimpleOrder(TypedDict):
    order_index: int
    price: str
    remaining_base_amount: str

class OrderBookOrders(TypedDict):
    code: int
    total_asks: int
    asks: list[SimpleOrder]
    total_bids: int
    bids: list[SimpleOrder]

class Trade(TypedDict):
    trade_id: int
    market_id: int
    size: str
    price: str
    is_maker_ask: bool
    timestamp: int

class Trades(TypedDict):
    code: int
    trades: list[Trade]

class Position(TypedDict):
    market_id: int
    symbol: str
    sign: int
    position: str
    avg_entry_price: str
    unrealized_pnl: str
    realized_pnl: str
    liquidation_price: str

class Account(TypedDict):
    index: int
    account_index: int
    l1_address: str
    available_balance: str
    collateral: str
    positions: list[Position]

class Accounts(TypedDict):
    code: int
    total: int
    accounts: list[Account]

class Order(TypedDict):
    order_index: int
    client_order_index: int
    market_index: int
    owner_account_index: int
    price: str
    remaining_base_amount: str
    is_ask: bool
    status: str

class Orders(TypedDict):
    code: int
    orders: list[Order]

class NextNonce(TypedDict):
    code: int
    nonce: int

_OrderBooksRequestOptional = TypedDict("_OrderBooksRequestOptional", {'market_id': 'int | None'}, total=False)
class OrderBooksRequest(_OrderBooksRequestOptional):
    pass

class OrderBookOrdersRequest(TypedDict):
    market_id: int
    limit: int

class RecentTradesRequest(TypedDict):
    market_id: int
    limit: int

class NextNonceRequest(TypedDict):
    account_index: int
    api_key_index: int

_ActiveOrdersRequestOptional = TypedDict("_ActiveOrdersRequestOptional", {'account_index': 'int | None', 'market_id': 'int | None'}, total=False)
class ActiveOrdersRequest(_ActiveOrdersRequestOptional):
    pass

class AccountRequest(TypedDict):
    by: AccountSelector
    value: Any

