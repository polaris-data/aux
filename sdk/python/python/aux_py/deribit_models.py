"""Deribit REST wire models generated from src/exchanges/deribit/schema/deribit.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'instrument_name': 'str | None', 'currency': 'str | None', 'kind': 'str | None', 'interval': 'str | None', 'depth': 'int | None', 'group': 'str | None', 'topic': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

_MarketsRequestOptional = TypedDict("_MarketsRequestOptional", {'kind': 'str | None', 'expired': 'bool | None'}, total=False)
class MarketsRequest(_MarketsRequestOptional):
    currency: str

class TickerRequest(TypedDict):
    instrument_name: str

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'start_seq': 'int | None', 'end_seq': 'int | None', 'count': 'int | None', 'start_timestamp': 'int | None', 'end_timestamp': 'int | None', 'sorting': 'str | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    instrument_name: str

_AccountRequestOptional = TypedDict("_AccountRequestOptional", {'extended': 'bool | None'}, total=False)
class AccountRequest(_AccountRequestOptional):
    currency: str

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'currency': 'str | None', 'kind': 'str | None', 'subaccount_id': 'int | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    pass

class OrderStatusRequest(TypedDict):
    order_id: str

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'kind': 'str | None', 'type': 'str | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    pass

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'count': 'int | None', 'offset': 'int | None', 'include_old': 'bool | None', 'include_unfilled': 'bool | None', 'historical': 'bool | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    instrument_name: str

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'start_seq': 'int | None', 'end_seq': 'int | None', 'count': 'int | None', 'historical': 'bool | None', 'sorting': 'str | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    instrument_name: str

_BuyRequestOptional = TypedDict("_BuyRequestOptional", {'amount': 'str | None', 'contracts': 'str | None', 'type': 'str | None', 'label': 'str | None', 'price': 'str | None', 'time_in_force': 'str | None', 'max_show': 'str | None', 'post_only': 'bool | None', 'reject_post_only': 'bool | None', 'reduce_only': 'bool | None', 'trigger_price': 'str | None', 'trigger_offset': 'str | None', 'trigger': 'str | None', 'advanced': 'str | None', 'mmp': 'bool | None', 'valid_until': 'int | None'}, total=False)
class BuyRequest(_BuyRequestOptional):
    instrument_name: str

_SellRequestOptional = TypedDict("_SellRequestOptional", {'amount': 'str | None', 'contracts': 'str | None', 'type': 'str | None', 'label': 'str | None', 'price': 'str | None', 'time_in_force': 'str | None', 'max_show': 'str | None', 'post_only': 'bool | None', 'reject_post_only': 'bool | None', 'reduce_only': 'bool | None', 'trigger_price': 'str | None', 'trigger_offset': 'str | None', 'trigger': 'str | None', 'advanced': 'str | None', 'mmp': 'bool | None', 'valid_until': 'int | None'}, total=False)
class SellRequest(_SellRequestOptional):
    instrument_name: str

_AmendOrderRequestOptional = TypedDict("_AmendOrderRequestOptional", {'amount': 'str | None', 'contracts': 'str | None', 'price': 'str | None', 'post_only': 'bool | None', 'reject_post_only': 'bool | None', 'reduce_only': 'bool | None', 'trigger_price': 'str | None', 'trigger_offset': 'str | None', 'advanced': 'str | None', 'mmp': 'bool | None', 'valid_until': 'int | None'}, total=False)
class AmendOrderRequest(_AmendOrderRequestOptional):
    order_id: str

class CancelOrderRequest(TypedDict):
    order_id: str

_CancelByLabelRequestOptional = TypedDict("_CancelByLabelRequestOptional", {'currency': 'str | None'}, total=False)
class CancelByLabelRequest(_CancelByLabelRequestOptional):
    label: str

_CancelAllOrdersRequestOptional = TypedDict("_CancelAllOrdersRequestOptional", {'detailed': 'bool | None', 'freeze_quotes': 'bool | None'}, total=False)
class CancelAllOrdersRequest(_CancelAllOrdersRequestOptional):
    pass

_CancelInstrumentOrdersRequestOptional = TypedDict("_CancelInstrumentOrdersRequestOptional", {'type': 'str | None', 'detailed': 'bool | None', 'include_combos': 'bool | None'}, total=False)
class CancelInstrumentOrdersRequest(_CancelInstrumentOrdersRequestOptional):
    instrument_name: str

_CancelCurrencyOrdersRequestOptional = TypedDict("_CancelCurrencyOrdersRequestOptional", {'kind': 'str | None', 'type': 'str | None', 'detailed': 'bool | None', 'freeze_quotes': 'bool | None'}, total=False)
class CancelCurrencyOrdersRequest(_CancelCurrencyOrdersRequestOptional):
    currency: str

_CancelKindOrdersRequestOptional = TypedDict("_CancelKindOrdersRequestOptional", {'kind': 'str | None', 'type': 'str | None', 'detailed': 'bool | None'}, total=False)
class CancelKindOrdersRequest(_CancelKindOrdersRequestOptional):
    pass

class AuthenticateRequest(TypedDict):
    pass

_AmendByLabelRequestOptional = TypedDict("_AmendByLabelRequestOptional", {'amount': 'str | None', 'contracts': 'str | None', 'price': 'str | None', 'post_only': 'bool | None', 'reduce_only': 'bool | None', 'reject_post_only': 'bool | None', 'advanced': 'str | None', 'trigger_price': 'str | None', 'mmp': 'bool | None', 'valid_until': 'int | None'}, total=False)
class AmendByLabelRequest(_AmendByLabelRequestOptional):
    label: str
    instrument_name: str

_CancelCurrencyPairOrdersRequestOptional = TypedDict("_CancelCurrencyPairOrdersRequestOptional", {'kind': 'str | None', 'type': 'str | None', 'detailed': 'bool | None', 'freeze_quotes': 'bool | None'}, total=False)
class CancelCurrencyPairOrdersRequest(_CancelCurrencyPairOrdersRequestOptional):
    currency_pair: str

_CancelQuotesRequestOptional = TypedDict("_CancelQuotesRequestOptional", {'detailed': 'bool | None', 'freeze_quotes': 'bool | None', 'min_delta': 'str | None', 'max_delta': 'str | None', 'quote_set_id': 'str | None', 'instrument_name': 'str | None', 'kind': 'str | None'}, total=False)
class CancelQuotesRequest(_CancelQuotesRequestOptional):
    currency: str
    currency_pair: str
    cancel_type: str

MarketsResponse = Any
TickerResponse = Any
TradesResponse = Any
AccountResponse = Any
PositionsResponse = Any
OrderStatusResponse = Any
OpenOrdersResponse = Any
OrderHistoryResponse = Any
FillsResponse = Any
BuyResponse = Any
SellResponse = Any
AmendOrderResponse = Any
CancelOrderResponse = Any
CancelByLabelResponse = Any
CancelAllOrdersResponse = Any
CancelInstrumentOrdersResponse = Any
CancelCurrencyOrdersResponse = Any
CancelKindOrdersResponse = Any
AuthenticateResponse = Any
AmendByLabelResponse = Any
CancelCurrencyPairOrdersResponse = Any
CancelQuotesResponse = Any
