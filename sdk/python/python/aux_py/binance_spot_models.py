"""BinanceSpot REST wire models generated from src/exchanges/binance_spot/schema/binance_spot.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_SubscriptionOptional = TypedDict("_SubscriptionOptional", {'symbol': 'str | None', 'depth': 'int | None', 'interval': 'str | None', 'topic': 'str | None'}, total=False)
class Subscription(_SubscriptionOptional):
    channel: str

_DepthRequestOptional = TypedDict("_DepthRequestOptional", {'limit': 'int | None', 'symbolStatus': 'str | None'}, total=False)
class DepthRequest(_DepthRequestOptional):
    symbol: str

class DepthResponse(TypedDict):
    lastUpdateId: int
    bids: list[list[str]]
    asks: list[list[str]]

_MarketsRequestOptional = TypedDict("_MarketsRequestOptional", {'symbol': 'str | None', 'symbols': 'list[str] | None', 'permissions': 'list[str] | None', 'showPermissionSets': 'bool | None', 'symbolStatus': 'str | None'}, total=False)
class MarketsRequest(_MarketsRequestOptional):
    pass

_TickerRequestOptional = TypedDict("_TickerRequestOptional", {'symbol': 'str | None', 'symbols': 'list[str] | None', 'type': 'str | None', 'symbolStatus': 'str | None'}, total=False)
class TickerRequest(_TickerRequestOptional):
    pass

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'limit': 'int | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    symbol: str

_AggTradesRequestOptional = TypedDict("_AggTradesRequestOptional", {'fromId': 'int | None', 'startTime': 'int | None', 'endTime': 'int | None', 'limit': 'int | None'}, total=False)
class AggTradesRequest(_AggTradesRequestOptional):
    symbol: str

_AccountRequestOptional = TypedDict("_AccountRequestOptional", {'omitZeroBalances': 'bool | None', 'recvWindow': 'str | None'}, total=False)
class AccountRequest(_AccountRequestOptional):
    pass

_OrderStatusRequestOptional = TypedDict("_OrderStatusRequestOptional", {'orderId': 'int | None', 'origClientOrderId': 'str | None', 'recvWindow': 'str | None'}, total=False)
class OrderStatusRequest(_OrderStatusRequestOptional):
    symbol: str

_OpenOrdersRequestOptional = TypedDict("_OpenOrdersRequestOptional", {'symbol': 'str | None', 'recvWindow': 'str | None'}, total=False)
class OpenOrdersRequest(_OpenOrdersRequestOptional):
    pass

_OrderHistoryRequestOptional = TypedDict("_OrderHistoryRequestOptional", {'orderId': 'int | None', 'startTime': 'int | None', 'endTime': 'int | None', 'limit': 'int | None', 'recvWindow': 'str | None'}, total=False)
class OrderHistoryRequest(_OrderHistoryRequestOptional):
    symbol: str

_FillsRequestOptional = TypedDict("_FillsRequestOptional", {'orderId': 'int | None', 'startTime': 'int | None', 'endTime': 'int | None', 'fromId': 'int | None', 'limit': 'int | None', 'recvWindow': 'str | None'}, total=False)
class FillsRequest(_FillsRequestOptional):
    symbol: str

_PlaceOrderRequestOptional = TypedDict("_PlaceOrderRequestOptional", {'timeInForce': 'str | None', 'quantity': 'str | None', 'quoteOrderQty': 'str | None', 'price': 'str | None', 'newClientOrderId': 'str | None', 'strategyId': 'int | None', 'strategyType': 'int | None', 'stopPrice': 'str | None', 'trailingDelta': 'int | None', 'icebergQty': 'str | None', 'newOrderRespType': 'str | None', 'selfTradePreventionMode': 'str | None', 'pegPriceType': 'str | None', 'pegOffsetValue': 'int | None', 'pegOffsetType': 'str | None', 'quoteNotionalLimit': 'str | None', 'recvWindow': 'str | None'}, total=False)
class PlaceOrderRequest(_PlaceOrderRequestOptional):
    symbol: str
    side: str
    type: str

_CancelOrderRequestOptional = TypedDict("_CancelOrderRequestOptional", {'orderId': 'int | None', 'origClientOrderId': 'str | None', 'newClientOrderId': 'str | None', 'cancelRestrictions': 'str | None', 'recvWindow': 'str | None'}, total=False)
class CancelOrderRequest(_CancelOrderRequestOptional):
    symbol: str

_CancelAllOrdersRequestOptional = TypedDict("_CancelAllOrdersRequestOptional", {'recvWindow': 'str | None'}, total=False)
class CancelAllOrdersRequest(_CancelAllOrdersRequestOptional):
    symbol: str

_CancelReplaceOrderRequestOptional = TypedDict("_CancelReplaceOrderRequestOptional", {'timeInForce': 'str | None', 'quantity': 'str | None', 'quoteOrderQty': 'str | None', 'price': 'str | None', 'cancelNewClientOrderId': 'str | None', 'cancelOrigClientOrderId': 'str | None', 'cancelOrderId': 'int | None', 'newClientOrderId': 'str | None', 'strategyId': 'int | None', 'strategyType': 'int | None', 'stopPrice': 'str | None', 'trailingDelta': 'int | None', 'icebergQty': 'str | None', 'newOrderRespType': 'str | None', 'selfTradePreventionMode': 'str | None', 'cancelRestrictions': 'str | None', 'orderRateLimitExceededMode': 'str | None', 'pegPriceType': 'str | None', 'pegOffsetValue': 'int | None', 'pegOffsetType': 'str | None', 'quoteNotionalLimit': 'str | None', 'recvWindow': 'str | None'}, total=False)
class CancelReplaceOrderRequest(_CancelReplaceOrderRequestOptional):
    symbol: str
    side: str
    type: str
    cancelReplaceMode: str

_AmendOrderRequestOptional = TypedDict("_AmendOrderRequestOptional", {'orderId': 'int | None', 'origClientOrderId': 'str | None', 'newClientOrderId': 'str | None', 'recvWindow': 'str | None'}, total=False)
class AmendOrderRequest(_AmendOrderRequestOptional):
    symbol: str
    newQty: str

_PlaceOcoRequestOptional = TypedDict("_PlaceOcoRequestOptional", {'listClientOrderId': 'str | None', 'aboveClientOrderId': 'str | None', 'aboveIcebergQty': 'str | None', 'abovePrice': 'str | None', 'aboveStopPrice': 'str | None', 'aboveTrailingDelta': 'int | None', 'aboveTimeInForce': 'str | None', 'aboveStrategyId': 'int | None', 'aboveStrategyType': 'int | None', 'abovePegPriceType': 'str | None', 'abovePegOffsetType': 'str | None', 'abovePegOffsetValue': 'int | None', 'belowClientOrderId': 'str | None', 'belowIcebergQty': 'str | None', 'belowPrice': 'str | None', 'belowStopPrice': 'str | None', 'belowTrailingDelta': 'int | None', 'belowTimeInForce': 'str | None', 'belowStrategyId': 'int | None', 'belowStrategyType': 'int | None', 'belowPegPriceType': 'str | None', 'belowPegOffsetType': 'str | None', 'belowPegOffsetValue': 'int | None', 'newOrderRespType': 'str | None', 'selfTradePreventionMode': 'str | None', 'aboveQuoteNotionalLimit': 'str | None', 'belowQuoteNotionalLimit': 'str | None', 'recvWindow': 'str | None'}, total=False)
class PlaceOcoRequest(_PlaceOcoRequestOptional):
    symbol: str
    side: str
    quantity: str
    aboveType: str
    belowType: str

_PlaceOtoRequestOptional = TypedDict("_PlaceOtoRequestOptional", {'listClientOrderId': 'str | None', 'newOrderRespType': 'str | None', 'selfTradePreventionMode': 'str | None', 'workingClientOrderId': 'str | None', 'workingIcebergQty': 'str | None', 'workingTimeInForce': 'str | None', 'workingStrategyId': 'int | None', 'workingStrategyType': 'int | None', 'workingPegPriceType': 'str | None', 'workingPegOffsetType': 'str | None', 'workingPegOffsetValue': 'int | None', 'pendingClientOrderId': 'str | None', 'pendingPrice': 'str | None', 'pendingStopPrice': 'str | None', 'pendingTrailingDelta': 'int | None', 'pendingIcebergQty': 'str | None', 'pendingTimeInForce': 'str | None', 'pendingStrategyId': 'int | None', 'pendingStrategyType': 'int | None', 'pendingPegPriceType': 'str | None', 'pendingPegOffsetType': 'str | None', 'pendingPegOffsetValue': 'int | None', 'workingQuoteNotionalLimit': 'str | None', 'pendingQuoteNotionalLimit': 'str | None', 'recvWindow': 'str | None'}, total=False)
class PlaceOtoRequest(_PlaceOtoRequestOptional):
    symbol: str
    workingType: str
    workingSide: str
    workingPrice: str
    workingQuantity: str
    pendingType: str
    pendingSide: str
    pendingQuantity: str

_PlaceOtocoRequestOptional = TypedDict("_PlaceOtocoRequestOptional", {'listClientOrderId': 'str | None', 'newOrderRespType': 'str | None', 'selfTradePreventionMode': 'str | None', 'workingClientOrderId': 'str | None', 'workingIcebergQty': 'str | None', 'workingTimeInForce': 'str | None', 'workingStrategyId': 'int | None', 'workingStrategyType': 'int | None', 'workingPegPriceType': 'str | None', 'workingPegOffsetType': 'str | None', 'workingPegOffsetValue': 'int | None', 'pendingAboveClientOrderId': 'str | None', 'pendingAbovePrice': 'str | None', 'pendingAboveStopPrice': 'str | None', 'pendingAboveTrailingDelta': 'int | None', 'pendingAboveIcebergQty': 'str | None', 'pendingAboveTimeInForce': 'str | None', 'pendingAboveStrategyId': 'int | None', 'pendingAboveStrategyType': 'int | None', 'pendingAbovePegPriceType': 'str | None', 'pendingAbovePegOffsetType': 'str | None', 'pendingAbovePegOffsetValue': 'int | None', 'pendingBelowType': 'str | None', 'pendingBelowClientOrderId': 'str | None', 'pendingBelowPrice': 'str | None', 'pendingBelowStopPrice': 'str | None', 'pendingBelowTrailingDelta': 'int | None', 'pendingBelowIcebergQty': 'str | None', 'pendingBelowTimeInForce': 'str | None', 'pendingBelowStrategyId': 'int | None', 'pendingBelowStrategyType': 'int | None', 'pendingBelowPegPriceType': 'str | None', 'pendingBelowPegOffsetType': 'str | None', 'pendingBelowPegOffsetValue': 'int | None', 'workingQuoteNotionalLimit': 'str | None', 'pendingAboveQuoteNotionalLimit': 'str | None', 'pendingBelowQuoteNotionalLimit': 'str | None', 'recvWindow': 'str | None'}, total=False)
class PlaceOtocoRequest(_PlaceOtocoRequestOptional):
    symbol: str
    workingType: str
    workingSide: str
    workingPrice: str
    workingQuantity: str
    pendingSide: str
    pendingQuantity: str
    pendingAboveType: str

_PlaceOpoRequestOptional = TypedDict("_PlaceOpoRequestOptional", {'listClientOrderId': 'str | None', 'newOrderRespType': 'str | None', 'selfTradePreventionMode': 'str | None', 'workingClientOrderId': 'str | None', 'workingIcebergQty': 'str | None', 'workingTimeInForce': 'str | None', 'workingStrategyId': 'int | None', 'workingStrategyType': 'int | None', 'workingPegPriceType': 'str | None', 'workingPegOffsetType': 'str | None', 'workingPegOffsetValue': 'int | None', 'pendingClientOrderId': 'str | None', 'pendingPrice': 'str | None', 'pendingStopPrice': 'str | None', 'pendingTrailingDelta': 'int | None', 'pendingIcebergQty': 'str | None', 'pendingTimeInForce': 'str | None', 'pendingStrategyId': 'int | None', 'pendingStrategyType': 'int | None', 'pendingPegPriceType': 'str | None', 'pendingPegOffsetType': 'str | None', 'pendingPegOffsetValue': 'int | None', 'workingQuoteNotionalLimit': 'str | None', 'recvWindow': 'str | None'}, total=False)
class PlaceOpoRequest(_PlaceOpoRequestOptional):
    symbol: str
    workingType: str
    workingSide: str
    workingPrice: str
    workingQuantity: str
    pendingType: str
    pendingSide: str

_PlaceOpocoRequestOptional = TypedDict("_PlaceOpocoRequestOptional", {'listClientOrderId': 'str | None', 'newOrderRespType': 'str | None', 'selfTradePreventionMode': 'str | None', 'workingClientOrderId': 'str | None', 'workingIcebergQty': 'str | None', 'workingTimeInForce': 'str | None', 'workingStrategyId': 'int | None', 'workingStrategyType': 'int | None', 'workingPegPriceType': 'str | None', 'workingPegOffsetType': 'str | None', 'workingPegOffsetValue': 'int | None', 'pendingAboveClientOrderId': 'str | None', 'pendingAbovePrice': 'str | None', 'pendingAboveStopPrice': 'str | None', 'pendingAboveTrailingDelta': 'int | None', 'pendingAboveIcebergQty': 'str | None', 'pendingAboveTimeInForce': 'str | None', 'pendingAboveStrategyId': 'int | None', 'pendingAboveStrategyType': 'int | None', 'pendingAbovePegPriceType': 'str | None', 'pendingAbovePegOffsetType': 'str | None', 'pendingAbovePegOffsetValue': 'int | None', 'pendingBelowType': 'str | None', 'pendingBelowClientOrderId': 'str | None', 'pendingBelowPrice': 'str | None', 'pendingBelowStopPrice': 'str | None', 'pendingBelowTrailingDelta': 'int | None', 'pendingBelowIcebergQty': 'str | None', 'pendingBelowTimeInForce': 'str | None', 'pendingBelowStrategyId': 'int | None', 'pendingBelowStrategyType': 'int | None', 'pendingBelowPegPriceType': 'str | None', 'pendingBelowPegOffsetType': 'str | None', 'pendingBelowPegOffsetValue': 'int | None', 'workingQuoteNotionalLimit': 'str | None', 'recvWindow': 'str | None'}, total=False)
class PlaceOpocoRequest(_PlaceOpocoRequestOptional):
    symbol: str
    workingType: str
    workingSide: str
    workingPrice: str
    workingQuantity: str
    pendingSide: str
    pendingAboveType: str

_CancelOrderListRequestOptional = TypedDict("_CancelOrderListRequestOptional", {'orderListId': 'int | None', 'listClientOrderId': 'str | None', 'newClientOrderId': 'str | None', 'recvWindow': 'str | None'}, total=False)
class CancelOrderListRequest(_CancelOrderListRequestOptional):
    symbol: str

_OrderListStatusRequestOptional = TypedDict("_OrderListStatusRequestOptional", {'orderListId': 'int | None', 'origClientOrderId': 'str | None', 'recvWindow': 'str | None'}, total=False)
class OrderListStatusRequest(_OrderListStatusRequestOptional):
    pass

_OrderListHistoryRequestOptional = TypedDict("_OrderListHistoryRequestOptional", {'fromId': 'int | None', 'startTime': 'int | None', 'endTime': 'int | None', 'limit': 'int | None', 'recvWindow': 'str | None'}, total=False)
class OrderListHistoryRequest(_OrderListHistoryRequestOptional):
    pass

_OpenOrderListsRequestOptional = TypedDict("_OpenOrderListsRequestOptional", {'recvWindow': 'str | None'}, total=False)
class OpenOrderListsRequest(_OpenOrderListsRequestOptional):
    pass

MarketsResponse = Any
TickerResponse = Any
TradesResponse = Any
AggTradesResponse = Any
AccountResponse = Any
OrderStatusResponse = Any
OpenOrdersResponse = Any
OrderHistoryResponse = Any
FillsResponse = Any
PlaceOrderResponse = Any
CancelOrderResponse = Any
CancelAllOrdersResponse = Any
CancelReplaceOrderResponse = Any
AmendOrderResponse = Any
PlaceOcoResponse = Any
PlaceOtoResponse = Any
PlaceOtocoResponse = Any
PlaceOpoResponse = Any
PlaceOpocoResponse = Any
CancelOrderListResponse = Any
OrderListStatusResponse = Any
OrderListHistoryResponse = Any
OpenOrderListsResponse = Any
