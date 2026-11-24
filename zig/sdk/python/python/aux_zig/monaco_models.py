"""Monaco REST wire models generated from schema/monaco.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_AccountBalanceOptional = TypedDict("_AccountBalanceOptional", {'token': 'str | None', 'symbol': 'str | None', 'decimals': 'int | None', 'availableBalance': 'str | None', 'lockedBalance': 'str | None', 'totalBalance': 'str | None', 'availableBalanceRaw': 'str | None', 'lockedBalanceRaw': 'str | None', 'assetId': 'str | None', 'totalBalanceRaw': 'str | None', 'wrappedNative': 'bool | None', 'version': 'int | None'}, total=False)
class AccountBalance(_AccountBalanceOptional):
    pass

_BatchCancelAllResponseOptional = TypedDict("_BatchCancelAllResponseOptional", {'totalRequested': 'int | None', 'totalCancelled': 'int | None', 'totalFailed': 'int | None', 'results': 'list[BatchCancelResult] | None'}, total=False)
class BatchCancelAllResponse(_BatchCancelAllResponseOptional):
    pass

_BatchCancelErrorOptional = TypedDict("_BatchCancelErrorOptional", {'code': 'str | None', 'message': 'str | None'}, total=False)
class BatchCancelError(_BatchCancelErrorOptional):
    pass

class BatchCancelOrdersRequest(TypedDict):
    orderIds: list[str]

_BatchCancelOrdersResponseOptional = TypedDict("_BatchCancelOrdersResponseOptional", {'totalRequested': 'int | None', 'totalCancelled': 'int | None', 'totalFailed': 'int | None', 'results': 'list[BatchCancelResult] | None'}, total=False)
class BatchCancelOrdersResponse(_BatchCancelOrdersResponseOptional):
    pass

_BatchCancelResultOptional = TypedDict("_BatchCancelResultOptional", {'orderId': 'str | None', 'cancelledAt': 'str | None', 'error': 'BatchCancelError | None'}, total=False)
class BatchCancelResult(_BatchCancelResultOptional):
    pass

_BatchCreateErrorOptional = TypedDict("_BatchCreateErrorOptional", {'code': 'str | None', 'message': 'str | None'}, total=False)
class BatchCreateError(_BatchCreateErrorOptional):
    pass

_BatchCreateOrderItemOptional = TypedDict("_BatchCreateOrderItemOptional", {'price': 'str | None', 'tradingMode': 'str | None', 'slippageToleranceBps': 'int | None', 'useMasterBalance': 'bool | None', 'expirationDate': 'str | None', 'timeInForce': 'str | None', 'marginAccountId': 'str | None', 'riskBucketId': 'str | None', 'riskBucketCollateral': 'str | None', 'positionSide': 'str | None', 'leverage': 'str | None', 'reduceOnly': 'bool | None', 'strategyKey': 'str | None', 'marginMode': 'str | None', 'postOnly': 'bool | None', 'clientOrderId': 'str | None', 'idempotencyKey': 'str | None', 'selfTradePreventionMode': 'str | None'}, total=False)
class BatchCreateOrderItem(_BatchCreateOrderItemOptional):
    tradingPairId: str
    orderType: str
    side: str
    quantity: str

class BatchCreateOrdersRequest(TypedDict):
    orders: list[BatchCreateOrderItem]

_BatchCreateOrdersResponseOptional = TypedDict("_BatchCreateOrdersResponseOptional", {'totalRequested': 'int | None', 'totalSucceeded': 'int | None', 'totalFailed': 'int | None', 'results': 'list[BatchCreateResult] | None'}, total=False)
class BatchCreateOrdersResponse(_BatchCreateOrdersResponseOptional):
    pass

_BatchCreateResultOptional = TypedDict("_BatchCreateResultOptional", {'orderId': 'str | None', 'matchResult': 'MatchResult | None', 'error': 'BatchCreateError | None'}, total=False)
class BatchCreateResult(_BatchCreateResultOptional):
    pass

_BatchReplaceErrorOptional = TypedDict("_BatchReplaceErrorOptional", {'code': 'str | None', 'message': 'str | None'}, total=False)
class BatchReplaceError(_BatchReplaceErrorOptional):
    pass

_BatchReplaceOrderItemOptional = TypedDict("_BatchReplaceOrderItemOptional", {'price': 'str | None', 'quantity': 'str | None', 'useMasterBalance': 'bool | None', 'postOnly': 'bool | None', 'clientOrderId': 'str | None', 'selfTradePreventionMode': 'str | None'}, total=False)
class BatchReplaceOrderItem(_BatchReplaceOrderItemOptional):
    orderId: str

class BatchReplaceOrdersRequest(TypedDict):
    orders: list[BatchReplaceOrderItem]

_BatchReplaceOrdersResponseOptional = TypedDict("_BatchReplaceOrdersResponseOptional", {'totalRequested': 'int | None', 'totalSucceeded': 'int | None', 'totalFailed': 'int | None', 'results': 'list[BatchReplaceResult] | None'}, total=False)
class BatchReplaceOrdersResponse(_BatchReplaceOrdersResponseOptional):
    pass

_BatchReplaceResultOptional = TypedDict("_BatchReplaceResultOptional", {'originalOrderId': 'str | None', 'newOrderId': 'str | None', 'updatedFields': 'UpdatedFields | None', 'matchResult': 'MatchResult | None', 'error': 'BatchReplaceError | None'}, total=False)
class BatchReplaceResult(_BatchReplaceResultOptional):
    pass

_CancelOrderRequestOptional = TypedDict("_CancelOrderRequestOptional", {'remainingQuantityTarget': 'str | None'}, total=False)
class CancelOrderRequest(_CancelOrderRequestOptional):
    orderId: str

_CancelOrderResponseOptional = TypedDict("_CancelOrderResponseOptional", {'orderId': 'str | None', 'status': 'str | None', 'message': 'str | None'}, total=False)
class CancelOrderResponse(_CancelOrderResponseOptional):
    pass

_ChallengeRequestOptional = TypedDict("_ChallengeRequestOptional", {'clientId': 'str | None', 'chainId': 'str | None'}, total=False)
class ChallengeRequest(_ChallengeRequestOptional):
    address: str
    sessionPublicKey: str

_ChallengeResponseOptional = TypedDict("_ChallengeResponseOptional", {'nonce': 'str | None', 'message': 'str | None', 'expiresAt': 'int | None'}, total=False)
class ChallengeResponse(_ChallengeResponseOptional):
    pass

_CreateOrderRequestOptional = TypedDict("_CreateOrderRequestOptional", {'price': 'str | None', 'tradingMode': 'str | None', 'slippageToleranceBps': 'int | None', 'useMasterBalance': 'bool | None', 'expirationDate': 'str | None', 'timeInForce': 'str | None', 'marginAccountId': 'str | None', 'riskBucketId': 'str | None', 'riskBucketCollateral': 'str | None', 'positionSide': 'str | None', 'leverage': 'str | None', 'reduceOnly': 'bool | None', 'takeProfit': 'ParentTpSlLeg | None', 'stopLoss': 'ParentTpSlLeg | None', 'strategyKey': 'str | None', 'marginMode': 'str | None', 'postOnly': 'bool | None', 'clientOrderId': 'str | None', 'idempotencyKey': 'str | None', 'selfTradePreventionMode': 'str | None', 'trailingStop': 'ParentTrailingStopLeg | None'}, total=False)
class CreateOrderRequest(_CreateOrderRequestOptional):
    tradingPairId: str
    orderType: str
    side: str
    quantity: str

_CreateOrderResponseOptional = TypedDict("_CreateOrderResponseOptional", {'orderId': 'str | None', 'status': 'str | None', 'message': 'str | None', 'matchResult': 'MatchResult | None', 'takeProfitOrderId': 'str | None', 'stopLossOrderId': 'str | None', 'marginAccountId': 'str | None', 'riskBucketId': 'str | None', 'strategyKey': 'str | None', 'delegationId': 'str | None', 'trailingStopOrderId': 'str | None'}, total=False)
class CreateOrderResponse(_CreateOrderResponseOptional):
    pass

_ExecutionPriceRangeOptional = TypedDict("_ExecutionPriceRangeOptional", {'bestPrice': 'str | None', 'worstPrice': 'str | None'}, total=False)
class ExecutionPriceRange(_ExecutionPriceRangeOptional):
    pass

_FundingRecordOptional = TypedDict("_FundingRecordOptional", {'tradingPairId': 'str | None', 'epoch': 'str | None', 'fundingRate': 'str | None', 'fundingDeltaPerUnit': 'str | None', 'cumulativeFundingPerUnit': 'str | None', 'referencePrice': 'str | None', 'sampleCount': 'int | None', 'windowStartedAt': 'str | None', 'windowClosedAt': 'str | None', 'settledAt': 'str | None', 'carried': 'bool | None'}, total=False)
class FundingRecord(_FundingRecordOptional):
    pass

_GetBalancesResponseOptional = TypedDict("_GetBalancesResponseOptional", {'balances': 'list[AccountBalance] | None', 'total': 'int | None', 'pageSize': 'int | None', 'totalPages': 'int | None', 'nextPageToken': 'str | None'}, total=False)
class GetBalancesResponse(_GetBalancesResponseOptional):
    pass

_GetFundingStateResponseOptional = TypedDict("_GetFundingStateResponseOptional", {'tradingPairId': 'str | None', 'currentFundingRate': 'str | None', 'estimatedNextFundingRate': 'str | None', 'nextFundingTime': 'str | None', 'fundingIntervalSeconds': 'int | None', 'lastFundingTime': 'str | None', 'updatedAt': 'str | None'}, total=False)
class GetFundingStateResponse(_GetFundingStateResponseOptional):
    pass

_GetIndexPriceResponseOptional = TypedDict("_GetIndexPriceResponseOptional", {'tradingPairId': 'str | None', 'indexPrice': 'str | None', 'components': 'list[IndexComponent] | None', 'updatedAt': 'str | None'}, total=False)
class GetIndexPriceResponse(_GetIndexPriceResponseOptional):
    pass

_GetMarginAccountSummaryResponseOptional = TypedDict("_GetMarginAccountSummaryResponseOptional", {'marginAccountId': 'str | None', 'equity': 'str | None', 'initialMarginRequired': 'str | None', 'maintenanceMarginRequired': 'str | None', 'freeCollateral': 'str | None', 'withdrawableCollateral': 'str | None', 'totalPositionNotional': 'str | None', 'unrealizedPnl': 'str | None', 'realizedPnl': 'str | None', 'updatedAt': 'str | None', 'label': 'str | None', 'tradingPairId': 'str | None', 'strategyKey': 'str | None', 'riskBucketId': 'str | None', 'marginMode': 'str | None', 'selectedTradingPairIds': 'list[str] | None', 'availableOrderCollateral': 'str | None', 'copyFollowId': 'str | None', 'copyLeaderHandle': 'str | None', 'riskBucketState': 'str | None', 'subaccountKey': 'str | None', 'marginAccountState': 'str | None', 'maxRemovableCollateral': 'str | None'}, total=False)
class GetMarginAccountSummaryResponse(_GetMarginAccountSummaryResponseOptional):
    pass

_GetMarkPriceResponseOptional = TypedDict("_GetMarkPriceResponseOptional", {'tradingPairId': 'str | None', 'markPrice': 'str | None', 'oracleProvider': 'str | None', 'oracleEpoch': 'str | None', 'updatedAt': 'str | None', 'regime': 'str | None'}, total=False)
class GetMarkPriceResponse(_GetMarkPriceResponseOptional):
    pass

_GetMarketMetadataResponseOptional = TypedDict("_GetMarketMetadataResponseOptional", {'baseIconUrl': 'str | None', 'high24h': 'str | None', 'lastPrice': 'str | None', 'lastPriceTimestamp': 'str | None', 'low24h': 'str | None', 'marketInitializationTimestamp': 'str | None', 'priceChange24h': 'str | None', 'priceChangePercent24h': 'str | None', 'quoteIconUrl': 'str | None', 'symbol': 'str | None', 'volume24h': 'str | None', 'minLeverage': 'str | None', 'maxLeverage': 'str | None', 'markPrice': 'str | None', 'indexPrice': 'str | None', 'totalBaseVolumeLtd': 'str | None', 'totalQuoteVolumeLtd': 'str | None', 'totalTradeCountLtd': 'int | None', 'quoteVolume24h': 'str | None'}, total=False)
class GetMarketMetadataResponse(_GetMarketMetadataResponseOptional):
    pass

_GetOrderResponseOptional = TypedDict("_GetOrderResponseOptional", {'id': 'str | None', 'tradingPairId': 'str | None', 'orderType': 'str | None', 'side': 'str | None', 'price': 'str | None', 'quantity': 'str | None', 'filledQuantity': 'str | None', 'averageFillPrice': 'str | None', 'status': 'str | None', 'tradingMode': 'str | None', 'timeInForce': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'expirationDate': 'str | None', 'applicationTakerFee': 'str | None', 'monacoTakerFee': 'str | None', 'monacoMakerRebate': 'str | None', 'totalTakerFees': 'str | None', 'takerTotalPayment': 'str | None', 'makerTotalReceipt': 'str | None', 'marginAccountId': 'str | None', 'positionSide': 'str | None', 'leverage': 'str | None', 'reduceOnly': 'bool | None', 'positionId': 'str | None', 'quantityRaw': 'str | None', 'filledQuantityRaw': 'str | None', 'remainingQuantity': 'str | None', 'remainingQuantityRaw': 'str | None', 'clientOrderId': 'str | None', 'cancelledAt': 'str | None', 'filledAt': 'str | None', 'expiredAt': 'str | None', 'submittedAt': 'str | None', 'acknowledgedAt': 'str | None', 'triggerPrice': 'str | None', 'quoteVolume': 'str | None', 'applicationId': 'str | None', 'parentOrderId': 'str | None', 'batchId': 'str | None', 'terminalReason': 'str | None', 'postOnly': 'bool | None', 'version': 'int | None', 'selfTradePreventionMode': 'str | None', 'conditionalOrderId': 'str | None', 'origin': 'str | None', 'placementTakerFeeBps': 'str | None', 'placementMakerFeeBps': 'str | None'}, total=False)
class GetOrderResponse(_GetOrderResponseOptional):
    pass

_GetOrderbookResponseOptional = TypedDict("_GetOrderbookResponseOptional", {'baseDecimals': 'int | None', 'baseToken': 'str | None', 'data': 'OrderbookData | None', 'eventType': 'str | None', 'symbol': 'str | None', 'tradingPairId': 'str | None', 'quoteDecimals': 'int | None', 'quoteToken': 'str | None', 'sequenceNumber': 'int | None', 'timestamp': 'str | None', 'tradingMode': 'str | None', 'bookSeq': 'str | None'}, total=False)
class GetOrderbookResponse(_GetOrderbookResponseOptional):
    pass

_GetTradesResponseOptional = TypedDict("_GetTradesResponseOptional", {'trades': 'list[PublicTrade] | None', 'pageSize': 'int | None', 'nextPageToken': 'str | None'}, total=False)
class GetTradesResponse(_GetTradesResponseOptional):
    pass

_GetTradingPairResponseOptional = TypedDict("_GetTradingPairResponseOptional", {'tradingPair': 'TradingPairData | None'}, total=False)
class GetTradingPairResponse(_GetTradingPairResponseOptional):
    pass

_IndexComponentOptional = TypedDict("_IndexComponentOptional", {'provider': 'str | None', 'price': 'str | None', 'weight': 'str | None', 'updatedAt': 'str | None'}, total=False)
class IndexComponent(_IndexComponentOptional):
    pass

_ListFundingHistoryResponseOptional = TypedDict("_ListFundingHistoryResponseOptional", {'records': 'list[FundingRecord] | None', 'total': 'int | None', 'pageSize': 'int | None', 'nextPageToken': 'str | None', 'totalCapped': 'bool | None'}, total=False)
class ListFundingHistoryResponse(_ListFundingHistoryResponseOptional):
    pass

_ListMarginAccountsResponseOptional = TypedDict("_ListMarginAccountsResponseOptional", {'accounts': 'list[MarginAccountSummary] | None', 'total': 'int | None', 'pageSize': 'int | None', 'nextPageToken': 'str | None'}, total=False)
class ListMarginAccountsResponse(_ListMarginAccountsResponseOptional):
    pass

_ListOrdersResponseOptional = TypedDict("_ListOrdersResponseOptional", {'orders': 'list[GetOrderResponse] | None', 'total': 'int | None', 'pageSize': 'int | None', 'totalPages': 'int | None', 'nextPageToken': 'str | None', 'totalCapped': 'bool | None'}, total=False)
class ListOrdersResponse(_ListOrdersResponseOptional):
    pass

_ListPositionsResponseOptional = TypedDict("_ListPositionsResponseOptional", {'positions': 'list[Position] | None', 'total': 'int | None', 'pageSize': 'int | None', 'nextPageToken': 'str | None', 'totalCapped': 'bool | None'}, total=False)
class ListPositionsResponse(_ListPositionsResponseOptional):
    pass

_ListTradingPairsResponseOptional = TypedDict("_ListTradingPairsResponseOptional", {'tradingPairs': 'list[TradingPairData] | None', 'page': 'int | None', 'pageSize': 'int | None', 'total': 'int | None', 'totalPages': 'int | None'}, total=False)
class ListTradingPairsResponse(_ListTradingPairsResponseOptional):
    pass

_MarginAccountSummaryOptional = TypedDict("_MarginAccountSummaryOptional", {'marginAccountId': 'str | None', 'equity': 'str | None', 'initialMarginRequired': 'str | None', 'maintenanceMarginRequired': 'str | None', 'freeCollateral': 'str | None', 'withdrawableCollateral': 'str | None', 'totalPositionNotional': 'str | None', 'unrealizedPnl': 'str | None', 'realizedPnl': 'str | None', 'updatedAt': 'str | None', 'label': 'str | None', 'tradingPairId': 'str | None', 'strategyKey': 'str | None', 'riskBucketId': 'str | None', 'marginMode': 'str | None', 'selectedTradingPairIds': 'list[str] | None', 'availableOrderCollateral': 'str | None', 'copyFollowId': 'str | None', 'copyLeaderHandle': 'str | None', 'riskBucketState': 'str | None', 'subaccountKey': 'str | None', 'marginAccountState': 'str | None', 'maxRemovableCollateral': 'str | None'}, total=False)
class MarginAccountSummary(_MarginAccountSummaryOptional):
    pass

_MatchResultOptional = TypedDict("_MatchResultOptional", {'tradesCount': 'int | None', 'totalFilled': 'str | None', 'remainingQuantity': 'str | None', 'averageFillPrice': 'str | None', 'status': 'str | None', 'actualSlippageBps': 'int | None', 'maxSlippageBps': 'int | None', 'executionPriceRange': 'ExecutionPriceRange | None'}, total=False)
class MatchResult(_MatchResultOptional):
    pass

_OrderbookDataOptional = TypedDict("_OrderbookDataOptional", {'bids': 'list[PriceLevel] | None', 'asks': 'list[PriceLevel] | None', 'bestBid': 'str | None', 'bestAsk': 'str | None', 'bidVolume': 'str | None', 'askVolume': 'str | None', 'priceChange': 'PriceChange | None'}, total=False)
class OrderbookData(_OrderbookDataOptional):
    pass

_ParentTpSlLegOptional = TypedDict("_ParentTpSlLegOptional", {'triggerPrice': 'str | None', 'orderType': 'str | None', 'limitPrice': 'str | None', 'timeInForce': 'str | None', 'slippageToleranceBps': 'int | None', 'expiresAt': 'str | None', 'quantity': 'str | None', 'closePosition': 'bool | None'}, total=False)
class ParentTpSlLeg(_ParentTpSlLegOptional):
    pass

_ParentTrailingStopLegOptional = TypedDict("_ParentTrailingStopLegOptional", {'trailBps': 'int | None', 'activationPrice': 'str | None', 'quantity': 'str | None', 'closePosition': 'bool | None', 'slippageToleranceBps': 'int | None', 'expiresAt': 'str | None'}, total=False)
class ParentTrailingStopLeg(_ParentTrailingStopLegOptional):
    pass

_PositionOptional = TypedDict("_PositionOptional", {'positionId': 'str | None', 'marginAccountId': 'str | None', 'tradingPairId': 'str | None', 'side': 'str | None', 'size': 'str | None', 'entryPrice': 'str | None', 'markPrice': 'str | None', 'indexPrice': 'str | None', 'unrealizedPnl': 'str | None', 'realizedPnl': 'str | None', 'isolatedMargin': 'str | None', 'leverage': 'str | None', 'maintenanceMarginRequired': 'str | None', 'initialMarginRequired': 'str | None', 'liquidationPrice': 'str | None', 'status': 'str | None', 'updatedAt': 'str | None', 'riskBucketId': 'str | None', 'marginMode': 'str | None', 'netRealizedPnl': 'str | None', 'realizedRoe': 'str | None', 'exitPrice': 'str | None', 'fundingPaid': 'str | None', 'feesPaid': 'str | None', 'cumFees': 'str | None', 'version': 'int | None', 'openedAt': 'str | None', 'lifetimeTotalsStatus': 'str | None'}, total=False)
class Position(_PositionOptional):
    pass

_PriceChangeOptional = TypedDict("_PriceChangeOptional", {'price24hAgo': 'str | None', 'priceChange24h': 'str | None', 'priceChangePercent24h': 'str | None'}, total=False)
class PriceChange(_PriceChangeOptional):
    pass

_PriceLevelOptional = TypedDict("_PriceLevelOptional", {'price': 'str | None', 'quantity': 'str | None', 'orderCount': 'int | None'}, total=False)
class PriceLevel(_PriceLevelOptional):
    pass

_PublicTradeOptional = TypedDict("_PublicTradeOptional", {'data': 'TradeData | None', 'eventType': 'str | None', 'tradingPairId': 'str | None', 'tradingMode': 'str | None'}, total=False)
class PublicTrade(_PublicTradeOptional):
    pass

_RefreshResponseOptional = TypedDict("_RefreshResponseOptional", {'expiresAt': 'int | None'}, total=False)
class RefreshResponse(_RefreshResponseOptional):
    pass

_ReplaceOrderRequestOptional = TypedDict("_ReplaceOrderRequestOptional", {'selfTradePreventionMode': 'str | None', 'price': 'str | None', 'quantity': 'str | None', 'useMasterBalance': 'bool | None', 'postOnly': 'bool | None', 'clientOrderId': 'str | None'}, total=False)
class ReplaceOrderRequest(_ReplaceOrderRequestOptional):
    pass

_ReplaceOrderResponseOptional = TypedDict("_ReplaceOrderResponseOptional", {'orderId': 'str | None', 'status': 'str | None', 'message': 'str | None', 'originalOrderId': 'str | None', 'updatedFields': 'UpdatedFields | None', 'matchResult': 'MatchResult | None'}, total=False)
class ReplaceOrderResponse(_ReplaceOrderResponseOptional):
    pass

_RevokeResponseOptional = TypedDict("_RevokeResponseOptional", {'message': 'str | None'}, total=False)
class RevokeResponse(_RevokeResponseOptional):
    pass

_TradeDataOptional = TypedDict("_TradeDataOptional", {'executedAt': 'str | None', 'makerSide': 'str | None', 'price': 'str | None', 'quantity': 'str | None', 'quantityRaw': 'str | None', 'tradeId': 'str | None', 'executionSource': 'str | None', 'makerAddress': 'str | None', 'takerAddress': 'str | None', 'takerFeeRaw': 'str | None', 'quoteVolumeRaw': 'str | None', 'takerApplicationId': 'str | None'}, total=False)
class TradeData(_TradeDataOptional):
    pass

_TradingPairDataOptional = TypedDict("_TradingPairDataOptional", {'baseAssetId': 'str | None', 'baseDecimals': 'int | None', 'baseIconUrl': 'str | None', 'baseToken': 'str | None', 'baseTokenContract': 'str | None', 'id': 'str | None', 'isActive': 'bool | None', 'makerFeeBps': 'int | None', 'marketType': 'str | None', 'maxOrderSize': 'str | None', 'minOrderSize': 'str | None', 'quoteAssetId': 'str | None', 'quoteDecimals': 'int | None', 'quoteIconUrl': 'str | None', 'quoteToken': 'str | None', 'quoteTokenContract': 'str | None', 'symbol': 'str | None', 'takerFeeBps': 'int | None', 'tickSize': 'str | None', 'minLeverage': 'str | None', 'maxLeverage': 'str | None', 'category': 'str | None', 'quantityStepSize': 'str | None', 'baseAssetName': 'str | None', 'quoteAssetName': 'str | None', 'isolatedOnly': 'bool | None'}, total=False)
class TradingPairData(_TradingPairDataOptional):
    pass

_UpdatedFieldsOptional = TypedDict("_UpdatedFieldsOptional", {'price': 'str | None', 'quantity': 'str | None'}, total=False)
class UpdatedFields(_UpdatedFieldsOptional):
    pass

_UserInfoOptional = TypedDict("_UserInfoOptional", {'id': 'str | None', 'address': 'str | None', 'username': 'str | None'}, total=False)
class UserInfo(_UserInfoOptional):
    pass

_VerifyRequestOptional = TypedDict("_VerifyRequestOptional", {'clientId': 'str | None', 'chainId': 'str | None', 'referralCode': 'str | None', 'inviteCode': 'str | None'}, total=False)
class VerifyRequest(_VerifyRequestOptional):
    address: str
    signature: str
    nonce: str
    sessionPublicKey: str

_VerifyResponseOptional = TypedDict("_VerifyResponseOptional", {'expiresAt': 'int | None', 'user': 'UserInfo | None', 'isNewUser': 'bool | None', 'referralApplied': 'bool | None', 'referralNotAppliedReason': 'str | None'}, total=False)
class VerifyResponse(_VerifyResponseOptional):
    pass

_TradingPairsRequestOptional = TypedDict("_TradingPairsRequestOptional", {'page': 'int | None', 'pageSize': 'int | None', 'marketType': 'str | None', 'baseToken': 'str | None', 'quoteToken': 'str | None', 'isActive': 'bool | None', 'category': 'str | None'}, total=False)
class TradingPairsRequest(_TradingPairsRequestOptional):
    pass

class TradingPairRequest(TypedDict):
    tradingPairId: str

class MarketMetadataRequest(TypedDict):
    tradingPairId: str

_OrderBookRequestOptional = TypedDict("_OrderBookRequestOptional", {'levels': 'int | None', 'tradingMode': 'str | None', 'magnitude': 'str | None', 'denomination': 'str | None'}, total=False)
class OrderBookRequest(_OrderBookRequestOptional):
    tradingPairId: str

_TradesRequestOptional = TypedDict("_TradesRequestOptional", {'pageSize': 'int | None', 'pageToken': 'str | None'}, total=False)
class TradesRequest(_TradesRequestOptional):
    tradingPairId: str

class MarkPriceRequest(TypedDict):
    tradingPairId: str

class IndexPriceRequest(TypedDict):
    tradingPairId: str

class FundingRequest(TypedDict):
    tradingPairId: str

_FundingHistoryRequestOptional = TypedDict("_FundingHistoryRequestOptional", {'startTime': 'int | None', 'endTime': 'int | None', 'pageSize': 'int | None', 'orderBy': 'str | None', 'pageToken': 'str | None'}, total=False)
class FundingHistoryRequest(_FundingHistoryRequestOptional):
    tradingPairId: str

_BalancesRequestOptional = TypedDict("_BalancesRequestOptional", {'pageSize': 'int | None', 'pageToken': 'str | None'}, total=False)
class BalancesRequest(_BalancesRequestOptional):
    pass

_ParentMarginAccountRequestOptional = TypedDict("_ParentMarginAccountRequestOptional", {'tradingPairId': 'str | None'}, total=False)
class ParentMarginAccountRequest(_ParentMarginAccountRequestOptional):
    pass

_MarginAccountsRequestOptional = TypedDict("_MarginAccountsRequestOptional", {'pageSize': 'int | None', 'state': 'str | None', 'tradingPairId': 'str | None', 'pageToken': 'str | None'}, total=False)
class MarginAccountsRequest(_MarginAccountsRequestOptional):
    pass

_PositionsRequestOptional = TypedDict("_PositionsRequestOptional", {'marginAccountId': 'str | None', 'tradingPairId': 'str | None', 'status': 'str | None', 'pageSize': 'int | None', 'pageToken': 'str | None'}, total=False)
class PositionsRequest(_PositionsRequestOptional):
    pass

_OrdersRequestOptional = TypedDict("_OrdersRequestOptional", {'pageSize': 'int | None', 'status': 'str | None', 'tradingPairId': 'str | None', 'tradingMode': 'str | None', 'marginAccountId': 'str | None', 'orderBy': 'str | None', 'pageToken': 'str | None'}, total=False)
class OrdersRequest(_OrdersRequestOptional):
    pass

class OrderRequest(TypedDict):
    orderId: str

class EmptyRequest(TypedDict):
    pass

class PlaceOrderParams(TypedDict):
    market: TradingPairData
    body: CreateOrderRequest

class ReplaceOrderParams(TypedDict):
    market: TradingPairData
    body: ReplaceOrderRequest
    orderId: str

class BatchCreateParams(TypedDict):
    markets: list[TradingPairData]
    body: BatchCreateOrdersRequest

class BatchReplaceParams(TypedDict):
    markets: list[TradingPairData]
    body: BatchReplaceOrdersRequest

_CancelAllParamsOptional = TypedDict("_CancelAllParamsOptional", {'tradingPairId': 'str | None'}, total=False)
class CancelAllParams(_CancelAllParamsOptional):
    pass

