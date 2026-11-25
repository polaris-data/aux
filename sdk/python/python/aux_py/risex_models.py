"""Risex REST wire models generated from src/exchanges/risex/schema/risex.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'market_ids': 'list[int] | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

class MarketsRequest(TypedDict):
    pass

_MarketsResponseDataMarketsItemConfigOptional = TypedDict("_MarketsResponseDataMarketsItemConfigOptional", {'name': 'str | None', 'step_size': 'str | None', 'step_price': 'str | None', 'min_order_size': 'str | None'}, total=False)
class MarketsResponseDataMarketsItemConfig(_MarketsResponseDataMarketsItemConfigOptional):
    pass

_MarketsResponseDataMarketsItemOptional = TypedDict("_MarketsResponseDataMarketsItemOptional", {'market_id': 'str | None', 'config': 'MarketsResponseDataMarketsItemConfig | None'}, total=False)
class MarketsResponseDataMarketsItem(_MarketsResponseDataMarketsItemOptional):
    pass

_MarketsResponseDataOptional = TypedDict("_MarketsResponseDataOptional", {'markets': 'list[MarketsResponseDataMarketsItem] | None'}, total=False)
class MarketsResponseData(_MarketsResponseDataOptional):
    pass

_MarketsResponseOptional = TypedDict("_MarketsResponseOptional", {'data': 'MarketsResponseData | None'}, total=False)
class MarketsResponse(_MarketsResponseOptional):
    pass

_OrderBookRequestOptional = TypedDict("_OrderBookRequestOptional", {'limit': 'int | None'}, total=False)
class OrderBookRequest(_OrderBookRequestOptional):
    market_id: int

_OrderBookResponseDataOptional = TypedDict("_OrderBookResponseDataOptional", {'bids': 'list[Any] | None', 'asks': 'list[Any] | None'}, total=False)
class OrderBookResponseData(_OrderBookResponseDataOptional):
    pass

_OrderBookResponseOptional = TypedDict("_OrderBookResponseOptional", {'data': 'OrderBookResponseData | None'}, total=False)
class OrderBookResponse(_OrderBookResponseOptional):
    pass

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'page': 'int | None', 'limit': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    market_id: int

_TradesResponseDataOptional = TypedDict("_TradesResponseDataOptional", {'market_id': 'str | None', 'trades': 'list[Any] | None'}, total=False)
class TradesResponseData(_TradesResponseDataOptional):
    pass

_TradesResponseOptional = TypedDict("_TradesResponseOptional", {'data': 'TradesResponseData | None'}, total=False)
class TradesResponse(_TradesResponseOptional):
    pass

class AccountRequest(TypedDict):
    account: str

_AccountResponseDataOptional = TypedDict("_AccountResponseDataOptional", {'balance': 'str | None'}, total=False)
class AccountResponseData(_AccountResponseDataOptional):
    pass

_AccountResponseOptional = TypedDict("_AccountResponseOptional", {'data': 'AccountResponseData | None'}, total=False)
class AccountResponse(_AccountResponseOptional):
    pass

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'market_id': 'int | None', 'page': 'int | None', 'page_size': 'int | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    account: str

_PositionsResponseOptional = TypedDict("_PositionsResponseOptional", {'data': 'PositionsResponseData | None'}, total=False)
class PositionsResponse(_PositionsResponseOptional):
    pass

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'limit': 'int | None', 'start_index': 'int | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    account: str
    market_id: int

_OpenOrdersResponseOptional = TypedDict("_OpenOrdersResponseOptional", {'data': 'OpenOrdersResponseData | None'}, total=False)
class OpenOrdersResponse(_OpenOrdersResponseOptional):
    pass

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'market_id': 'int | None', 'page': 'int | None', 'limit': 'int | None', 'order_ids': 'list[str] | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    account: str

_OrderHistoryResponseOptional = TypedDict("_OrderHistoryResponseOptional", {'data': 'OrderHistoryResponseData | None'}, total=False)
class OrderHistoryResponse(_OrderHistoryResponseOptional):
    pass

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'market_id': 'int | None', 'page': 'int | None', 'limit': 'int | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    account: str

_FillsResponseOptional = TypedDict("_FillsResponseOptional", {'data': 'FillsResponseData | None'}, total=False)
class FillsResponse(_FillsResponseOptional):
    pass

class SigningDomainRequest(TypedDict):
    pass

_SigningDomainResponseDataOptional = TypedDict("_SigningDomainResponseDataOptional", {'name': 'str | None', 'version': 'str | None', 'chain_id': 'str | None', 'verifying_contract': 'str | None'}, total=False)
class SigningDomainResponseData(_SigningDomainResponseDataOptional):
    pass

_SigningDomainResponseOptional = TypedDict("_SigningDomainResponseOptional", {'data': 'SigningDomainResponseData | None'}, total=False)
class SigningDomainResponse(_SigningDomainResponseOptional):
    pass

class SystemConfigRequest(TypedDict):
    pass

_SystemConfigResponseDataAddressesOptional = TypedDict("_SystemConfigResponseDataAddressesOptional", {'router': 'str | None'}, total=False)
class SystemConfigResponseDataAddresses(_SystemConfigResponseDataAddressesOptional):
    pass

_SystemConfigResponseDataOptional = TypedDict("_SystemConfigResponseDataOptional", {'addresses': 'SystemConfigResponseDataAddresses | None'}, total=False)
class SystemConfigResponseData(_SystemConfigResponseDataOptional):
    pass

_SystemConfigResponseOptional = TypedDict("_SystemConfigResponseOptional", {'data': 'SystemConfigResponseData | None'}, total=False)
class SystemConfigResponse(_SystemConfigResponseOptional):
    pass

class NonceStateRequest(TypedDict):
    account: str

_NonceStateResponseDataOptional = TypedDict("_NonceStateResponseDataOptional", {'nonce_anchor': 'str | None', 'current_bitmap_index': 'int | None', 'bitmap': 'str | None'}, total=False)
class NonceStateResponseData(_NonceStateResponseDataOptional):
    pass

_NonceStateResponseOptional = TypedDict("_NonceStateResponseOptional", {'data': 'NonceStateResponseData | None'}, total=False)
class NonceStateResponse(_NonceStateResponseOptional):
    pass

class AuthNonceRequest(TypedDict):
    pass

_AuthNonceResponseOptional = TypedDict("_AuthNonceResponseOptional", {'nonce': 'str | None'}, total=False)
class AuthNonceResponse(_AuthNonceResponseOptional):
    pass

_PlaceOrderBodyOptional = TypedDict("_PlaceOrderBodyOptional", {'time_in_force': 'str | None', 'client_order_id': 'str | None', 'post_only': 'bool | None', 'reduce_only': 'bool | None', 'stp_mode': 'str | None', 'ttl_units': 'int | None'}, total=False)
class PlaceOrderBody(_PlaceOrderBodyOptional):
    market_id: int
    size_steps: int
    price_ticks: int
    side: str
    order_type: str

class PlaceOrderRequest(TypedDict):
    body: PlaceOrderBody

_PlaceOrderResponseDataOptional = TypedDict("_PlaceOrderResponseDataOptional", {'tx_hash': 'str | None'}, total=False)
class PlaceOrderResponseData(_PlaceOrderResponseDataOptional):
    pass

_PlaceOrderResponseOptional = TypedDict("_PlaceOrderResponseOptional", {'data': 'PlaceOrderResponseData | None'}, total=False)
class PlaceOrderResponse(_PlaceOrderResponseOptional):
    pass

_CancelOrderBodyOptional = TypedDict("_CancelOrderBodyOptional", {'order_id': 'str | None', 'resting_order_id': 'str | None'}, total=False)
class CancelOrderBody(_CancelOrderBodyOptional):
    market_id: int

class CancelOrderRequest(TypedDict):
    body: CancelOrderBody

_CancelOrderResponseDataOptional = TypedDict("_CancelOrderResponseDataOptional", {'tx_hash': 'str | None'}, total=False)
class CancelOrderResponseData(_CancelOrderResponseDataOptional):
    pass

_CancelOrderResponseOptional = TypedDict("_CancelOrderResponseOptional", {'data': 'CancelOrderResponseData | None'}, total=False)
class CancelOrderResponse(_CancelOrderResponseOptional):
    pass

_PositionsResponseDataOptional = TypedDict("_PositionsResponseDataOptional", {'positions': 'list[Any] | None'}, total=False)
class PositionsResponseData(_PositionsResponseDataOptional):
    pass

_OpenOrdersResponseDataOptional = TypedDict("_OpenOrdersResponseDataOptional", {'orders': 'list[Any] | None'}, total=False)
class OpenOrdersResponseData(_OpenOrdersResponseDataOptional):
    pass

_OrderHistoryResponseDataOptional = TypedDict("_OrderHistoryResponseDataOptional", {'orders': 'list[Any] | None'}, total=False)
class OrderHistoryResponseData(_OrderHistoryResponseDataOptional):
    pass

_FillsResponseDataOptional = TypedDict("_FillsResponseDataOptional", {'trades': 'list[Any] | None'}, total=False)
class FillsResponseData(_FillsResponseDataOptional):
    pass

