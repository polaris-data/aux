"""Polymarket REST wire models generated from src/exchanges/polymarket/schema/polymarket.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class BridgeDepositRequest(TypedDict):
    address: str

_BridgeDepositResponseOptional = TypedDict("_BridgeDepositResponseOptional", {'address': 'BridgeDepositResponseAddress | None', 'note': 'str | None'}, total=False)
class BridgeDepositResponse(_BridgeDepositResponseOptional):
    pass

_BridgeDepositResponseAddressOptional = TypedDict("_BridgeDepositResponseAddressOptional", {'evm': 'str | None', 'svm': 'str | None', 'btc': 'str | None', 'tron': 'str | None'}, total=False)
class BridgeDepositResponseAddress(_BridgeDepositResponseAddressOptional):
    pass

class BridgeErrorResponse(TypedDict):
    error: str

_BridgeFeeBreakdownOptional = TypedDict("_BridgeFeeBreakdownOptional", {'appFeeLabel': 'str | None', 'appFeePercent': 'int | Decimal | None', 'appFeeUsd': 'int | Decimal | None', 'fillCostPercent': 'int | Decimal | None', 'fillCostUsd': 'int | Decimal | None', 'gasUsd': 'int | Decimal | None', 'maxSlippage': 'int | Decimal | None', 'minReceived': 'int | Decimal | None', 'swapImpact': 'int | Decimal | None', 'swapImpactUsd': 'int | Decimal | None', 'totalImpact': 'int | Decimal | None', 'totalImpactUsd': 'int | Decimal | None'}, total=False)
class BridgeFeeBreakdown(_BridgeFeeBreakdownOptional):
    pass

_BridgeGetStatusAddressParamsOptional = TypedDict("_BridgeGetStatusAddressParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'paginate': 'str | None'}, total=False)
class BridgeGetStatusAddressParams(_BridgeGetStatusAddressParamsOptional):
    address: str

class BridgeGetStatusAddressResponse(TypedDict):
    transactions: list[BridgeTransaction]
    nextCursor: str | None

class BridgeGetSupportedAssetsParams(TypedDict):
    pass

_BridgeGetSupportedAssetsResponseOptional = TypedDict("_BridgeGetSupportedAssetsResponseOptional", {'supportedAssets': 'list[BridgeSupportedAsset] | None'}, total=False)
class BridgeGetSupportedAssetsResponse(_BridgeGetSupportedAssetsResponseOptional):
    pass

_BridgePostDepositParamsOptional = TypedDict("_BridgePostDepositParamsOptional", {'X-Builder-Code': 'str | None'}, total=False)
class BridgePostDepositParams(_BridgePostDepositParamsOptional):
    body: BridgeDepositRequest

_BridgePostDepositResponseOptional = TypedDict("_BridgePostDepositResponseOptional", {'address': 'BridgePostDepositResponseAddress | None', 'note': 'str | None'}, total=False)
class BridgePostDepositResponse(_BridgePostDepositResponseOptional):
    pass

_BridgePostDepositResponseAddressOptional = TypedDict("_BridgePostDepositResponseAddressOptional", {'evm': 'str | None', 'svm': 'str | None', 'btc': 'str | None', 'tron': 'str | None'}, total=False)
class BridgePostDepositResponseAddress(_BridgePostDepositResponseAddressOptional):
    pass

class BridgePostQuoteParams(TypedDict):
    body: BridgeQuoteRequest

_BridgePostQuoteResponseOptional = TypedDict("_BridgePostQuoteResponseOptional", {'estCheckoutTimeMs': 'int | Decimal | None', 'estFeeBreakdown': 'BridgeFeeBreakdown | None', 'estInputUsd': 'int | Decimal | None', 'estOutputUsd': 'int | Decimal | None', 'estToTokenBaseUnit': 'str | None', 'quoteId': 'str | None'}, total=False)
class BridgePostQuoteResponse(_BridgePostQuoteResponseOptional):
    pass

_BridgePostWithdrawParamsOptional = TypedDict("_BridgePostWithdrawParamsOptional", {'X-Builder-Code': 'str | None'}, total=False)
class BridgePostWithdrawParams(_BridgePostWithdrawParamsOptional):
    body: BridgeWithdrawalRequest

_BridgePostWithdrawResponseOptional = TypedDict("_BridgePostWithdrawResponseOptional", {'address': 'BridgePostWithdrawResponseAddress | None', 'note': 'str | None'}, total=False)
class BridgePostWithdrawResponse(_BridgePostWithdrawResponseOptional):
    pass

_BridgePostWithdrawResponseAddressOptional = TypedDict("_BridgePostWithdrawResponseAddressOptional", {'evm': 'str | None', 'svm': 'str | None', 'btc': 'str | None', 'tron': 'str | None'}, total=False)
class BridgePostWithdrawResponseAddress(_BridgePostWithdrawResponseAddressOptional):
    pass

class BridgeQuoteRequest(TypedDict):
    fromAmountBaseUnit: str
    fromChainId: str
    fromTokenAddress: str
    recipientAddress: str
    toChainId: str
    toTokenAddress: str

_BridgeQuoteResponseOptional = TypedDict("_BridgeQuoteResponseOptional", {'estCheckoutTimeMs': 'int | Decimal | None', 'estFeeBreakdown': 'BridgeFeeBreakdown | None', 'estInputUsd': 'int | Decimal | None', 'estOutputUsd': 'int | Decimal | None', 'estToTokenBaseUnit': 'str | None', 'quoteId': 'str | None'}, total=False)
class BridgeQuoteResponse(_BridgeQuoteResponseOptional):
    pass

_BridgeSupportedAssetOptional = TypedDict("_BridgeSupportedAssetOptional", {'chainId': 'str | None', 'chainName': 'str | None', 'token': 'BridgeToken | None', 'minCheckoutUsd': 'int | Decimal | None'}, total=False)
class BridgeSupportedAsset(_BridgeSupportedAssetOptional):
    pass

_BridgeSupportedAssetsResponseOptional = TypedDict("_BridgeSupportedAssetsResponseOptional", {'supportedAssets': 'list[BridgeSupportedAsset] | None'}, total=False)
class BridgeSupportedAssetsResponse(_BridgeSupportedAssetsResponseOptional):
    pass

_BridgeTokenOptional = TypedDict("_BridgeTokenOptional", {'name': 'str | None', 'symbol': 'str | None', 'address': 'str | None', 'decimals': 'int | Decimal | None'}, total=False)
class BridgeToken(_BridgeTokenOptional):
    pass

_BridgeTransactionOptional = TypedDict("_BridgeTransactionOptional", {'fromChainId': 'str | None', 'fromTokenAddress': 'str | None', 'fromAmountBaseUnit': 'str | None', 'toChainId': 'str | None', 'toTokenAddress': 'str | None', 'status': 'str | None', 'txHash': 'str | None', 'createdTimeMs': 'int | Decimal | None'}, total=False)
class BridgeTransaction(_BridgeTransactionOptional):
    pass

class BridgeTransactionStatusResponse(TypedDict):
    transactions: list[BridgeTransaction]
    nextCursor: str | None

class BridgeWithdrawalRequest(TypedDict):
    address: str
    toChainId: str
    toTokenAddress: str
    recipientAddr: str

class BuilderGatewayAcceptRfqParams(TypedDict):
    rfq_id: str
    body: BuilderGatewayAcceptRfqParamsBody

class BuilderGatewayAcceptRfqParamsBody(TypedDict):
    quote_id: str
    signed_order: dict[str, Any]

class BuilderGatewayCreateRfqParams(TypedDict):
    body: BuilderGatewayCreateRfqParamsBody

class BuilderGatewayCreateRfqParamsBody(TypedDict):
    direction: str
    leg_position_ids: list[str]
    maker_address: str
    requested_size: BuilderGatewayCreateRfqParamsBodyRequestedSize
    side: str
    signature_type: int | Decimal
    signer_address: str

class BuilderGatewayCreateRfqParamsBodyRequestedSize(TypedDict):
    unit: str
    value_e6: str

class BuilderGatewayGetRfqParams(TypedDict):
    rfq_id: str

class ClobApiKeyResponse(TypedDict):
    apiKey: str
    secret: str
    passphrase: str

class ClobApiKeysResponse(TypedDict):
    apiKeys: list[str]

_ClobAssetEarningOptional = TypedDict("_ClobAssetEarningOptional", {'asset_rate': 'int | Decimal | None'}, total=False)
class ClobAssetEarning(_ClobAssetEarningOptional):
    asset_address: str
    earnings: int | Decimal

class ClobBalanceAllowanceResponse(TypedDict):
    balance: str
    allowances: dict[str, str]

_ClobBatchPricesHistoryRequestOptional = TypedDict("_ClobBatchPricesHistoryRequestOptional", {'start_ts': 'int | Decimal | None', 'end_ts': 'int | Decimal | None', 'interval': 'str | None', 'fidelity': 'int | Decimal | None'}, total=False)
class ClobBatchPricesHistoryRequest(_ClobBatchPricesHistoryRequestOptional):
    markets: list[str]

_ClobBatchPricesHistoryResponseOptional = TypedDict("_ClobBatchPricesHistoryResponseOptional", {'history': 'dict[str, list[ClobMarketPrice]] | None'}, total=False)
class ClobBatchPricesHistoryResponse(_ClobBatchPricesHistoryResponseOptional):
    pass

_ClobBookRequestOptional = TypedDict("_ClobBookRequestOptional", {'side': 'str | None'}, total=False)
class ClobBookRequest(_ClobBookRequestOptional):
    token_id: str

class ClobBuilderApiKeyResponse(TypedDict):
    key: str
    secret: str
    passphrase: str

_ClobBuilderTradeOptional = TypedDict("_ClobBuilderTradeOptional", {'err_msg': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None'}, total=False)
class ClobBuilderTrade(_ClobBuilderTradeOptional):
    id: str
    tradeType: str
    takerOrderHash: str
    builder: str
    market: str
    assetId: str
    side: str
    size: str
    sizeUsdc: str
    price: str
    status: str
    outcome: str
    outcomeIndex: int | Decimal
    owner: str
    maker: str
    transactionHash: str
    matchTime: str
    bucketIndex: int | Decimal
    fee: str
    feeUsdc: str

class ClobBuilderTradesResponse(TypedDict):
    limit: int | Decimal
    next_cursor: str
    count: int | Decimal
    data: list[ClobBuilderTrade]

class ClobCancelAllOrdersParams(TypedDict):
    pass

class ClobCancelAllOrdersResponse(TypedDict):
    canceled: list[str]
    not_canceled: dict[str, str]

class ClobCancelMarketOrdersParams(TypedDict):
    body: ClobOrderMarketCancelParams

class ClobCancelMarketOrdersResponse(TypedDict):
    canceled: list[str]
    not_canceled: dict[str, str]

class ClobCancelOrderParams(TypedDict):
    body: ClobCancelOrderPayload

class ClobCancelOrderPayload(TypedDict):
    orderID: str

class ClobCancelOrderResponse(TypedDict):
    canceled: list[str]
    not_canceled: dict[str, str]

class ClobCancelOrdersParams(TypedDict):
    body: list[str]

class ClobCancelOrdersResponse(TypedDict):
    canceled: list[str]
    not_canceled: dict[str, str]

_ClobClobMarketDetailsOptional = TypedDict("_ClobClobMarketDetailsOptional", {'gst': 'str | None', 'r': 'ClobClobRewards | None', 't': 'list[ClobClobToken] | None', 'mos': 'int | Decimal | None', 'mts': 'int | Decimal | None', 'mbf': 'int | Decimal | None', 'tbf': 'int | Decimal | None', 'rfqe': 'bool | None', 'itode': 'bool | None', 'ibce': 'bool | None', 'fd': 'ClobFeeDetails | None', 'oas': 'int | Decimal | None'}, total=False)
class ClobClobMarketDetails(_ClobClobMarketDetailsOptional):
    pass

_ClobClobTokenOptional = TypedDict("_ClobClobTokenOptional", {'t': 'str | None', 'o': 'str | None'}, total=False)
class ClobClobToken(_ClobClobTokenOptional):
    pass

class ClobClosedOnlyResponse(TypedDict):
    closed_only: bool

class ClobCreateApiKeyParams(TypedDict):
    pass

class ClobCreateApiKeyResponse(TypedDict):
    apiKey: str
    secret: str
    passphrase: str

class ClobCreateBuilderApiKeyParams(TypedDict):
    pass

class ClobCreateBuilderApiKeyResponse(TypedDict):
    key: str
    secret: str
    passphrase: str

_ClobCurrentRewardOptional = TypedDict("_ClobCurrentRewardOptional", {'rewards_max_spread': 'int | Decimal | None', 'rewards_min_size': 'int | Decimal | None', 'rewards_config': 'list[ClobCurrentRewardConfig] | None', 'sponsored_daily_rate': 'int | Decimal | None', 'sponsors_count': 'int | Decimal | None', 'native_daily_rate': 'int | Decimal | None', 'total_daily_rate': 'int | Decimal | None'}, total=False)
class ClobCurrentReward(_ClobCurrentRewardOptional):
    condition_id: str

_ClobCurrentRewardConfigOptional = TypedDict("_ClobCurrentRewardConfigOptional", {'id': 'int | Decimal | None', 'end_date': 'str | None', 'total_rewards': 'int | Decimal | None'}, total=False)
class ClobCurrentRewardConfig(_ClobCurrentRewardConfigOptional):
    asset_address: str
    start_date: str
    rate_per_day: int | Decimal

class ClobDeleteApiKeyParams(TypedDict):
    pass

class ClobDeriveApiKeyParams(TypedDict):
    pass

class ClobDeriveApiKeyResponse(TypedDict):
    apiKey: str
    secret: str
    passphrase: str

class ClobDropNotificationsParams(TypedDict):
    ids: str

_ClobErrorResponseOptional = TypedDict("_ClobErrorResponseOptional", {'code': 'str | None', 'retry_after_seconds': 'int | Decimal | None'}, total=False)
class ClobErrorResponse(_ClobErrorResponseOptional):
    error: str

_ClobFeeDetailsOptional = TypedDict("_ClobFeeDetailsOptional", {'r': 'int | Decimal | None', 'e': 'int | Decimal | None', 'to': 'bool | None'}, total=False)
class ClobFeeDetails(_ClobFeeDetailsOptional):
    pass

class ClobFeeRate(TypedDict):
    base_fee: int | Decimal

class ClobGetApiKeysParams(TypedDict):
    pass

class ClobGetApiKeysResponse(TypedDict):
    apiKeys: list[str]

_ClobGetBalanceAllowanceParamsOptional = TypedDict("_ClobGetBalanceAllowanceParamsOptional", {'token_id': 'str | None', 'signature_type': 'int | Decimal | None'}, total=False)
class ClobGetBalanceAllowanceParams(_ClobGetBalanceAllowanceParamsOptional):
    asset_type: str

class ClobGetBalanceAllowanceResponse(TypedDict):
    balance: str
    allowances: dict[str, str]

class ClobGetBatchPricesHistoryParams(TypedDict):
    body: ClobBatchPricesHistoryRequest

_ClobGetBatchPricesHistoryResponseOptional = TypedDict("_ClobGetBatchPricesHistoryResponseOptional", {'history': 'dict[str, list[ClobMarketPrice]] | None'}, total=False)
class ClobGetBatchPricesHistoryResponse(_ClobGetBatchPricesHistoryResponseOptional):
    pass

class ClobGetBookParams(TypedDict):
    token_id: str

class ClobGetBookResponse(TypedDict):
    market: str
    asset_id: str
    timestamp: str
    hash: str
    bids: list[ClobOrderSummary]
    asks: list[ClobOrderSummary]
    min_order_size: str
    tick_size: str
    neg_risk: bool
    last_trade_price: str

class ClobGetBooksGetParams(TypedDict):
    token_ids: str

class ClobGetBooksPostParams(TypedDict):
    body: list[ClobBookRequest]

class ClobGetBuilderApiKeysParams(TypedDict):
    pass

_ClobGetBuilderTradesParamsOptional = TypedDict("_ClobGetBuilderTradesParamsOptional", {'id': 'str | None', 'market': 'str | None', 'asset_id': 'str | None', 'before': 'str | None', 'after': 'str | None', 'next_cursor': 'str | None'}, total=False)
class ClobGetBuilderTradesParams(_ClobGetBuilderTradesParamsOptional):
    builder_code: str

class ClobGetBuilderTradesResponse(TypedDict):
    limit: int | Decimal
    next_cursor: str
    count: int | Decimal
    data: list[ClobBuilderTrade]

class ClobGetClobMarketInfoParams(TypedDict):
    condition_id: str

_ClobGetClobMarketInfoResponseOptional = TypedDict("_ClobGetClobMarketInfoResponseOptional", {'gst': 'str | None', 'r': 'ClobClobRewards | None', 't': 'list[ClobClobToken] | None', 'mos': 'int | Decimal | None', 'mts': 'int | Decimal | None', 'mbf': 'int | Decimal | None', 'tbf': 'int | Decimal | None', 'rfqe': 'bool | None', 'itode': 'bool | None', 'ibce': 'bool | None', 'fd': 'ClobFeeDetails | None', 'oas': 'int | Decimal | None'}, total=False)
class ClobGetClobMarketInfoResponse(_ClobGetClobMarketInfoResponseOptional):
    pass

class ClobGetClosedOnlyModeParams(TypedDict):
    pass

class ClobGetClosedOnlyModeResponse(TypedDict):
    closed_only: bool

class ClobGetCurrentRebatedFeesParams(TypedDict):
    date: str
    maker_address: str

_ClobGetCurrentRewardsParamsOptional = TypedDict("_ClobGetCurrentRewardsParamsOptional", {'sponsored': 'bool | None', 'next_cursor': 'str | None'}, total=False)
class ClobGetCurrentRewardsParams(_ClobGetCurrentRewardsParamsOptional):
    pass

class ClobGetCurrentRewardsResponse(TypedDict):
    limit: int | Decimal
    count: int | Decimal
    next_cursor: str
    data: list[ClobCurrentReward]

_ClobGetEarningsForUserForDayParamsOptional = TypedDict("_ClobGetEarningsForUserForDayParamsOptional", {'signature_type': 'int | Decimal | None', 'maker_address': 'str | None', 'sponsored': 'bool | None', 'next_cursor': 'str | None'}, total=False)
class ClobGetEarningsForUserForDayParams(_ClobGetEarningsForUserForDayParamsOptional):
    date: str

class ClobGetEarningsForUserForDayResponse(TypedDict):
    limit: int | Decimal
    count: int | Decimal
    next_cursor: str
    data: list[ClobUserEarning]

class ClobGetFeeRateByPathParams(TypedDict):
    token_id: str

class ClobGetFeeRateByPathResponse(TypedDict):
    base_fee: int | Decimal

_ClobGetFeeRateParamsOptional = TypedDict("_ClobGetFeeRateParamsOptional", {'token_id': 'str | None'}, total=False)
class ClobGetFeeRateParams(_ClobGetFeeRateParamsOptional):
    pass

class ClobGetFeeRateResponse(TypedDict):
    base_fee: int | Decimal

class ClobGetLastTradePriceParams(TypedDict):
    token_id: str

class ClobGetLastTradePriceResponse(TypedDict):
    price: str
    side: str

class ClobGetLastTradesPricesGetParams(TypedDict):
    token_ids: str

class ClobGetLastTradesPricesGetResponseItem(TypedDict):
    token_id: str
    price: str
    side: str

class ClobGetLastTradesPricesGetResponseValueItem(TypedDict):
    token_id: str
    price: str
    side: str

class ClobGetLastTradesPricesPostParams(TypedDict):
    body: list[ClobBookRequest]

class ClobGetLastTradesPricesPostResponseItem(TypedDict):
    token_id: str
    price: str
    side: str

class ClobGetLastTradesPricesPostResponseValueItem(TypedDict):
    token_id: str
    price: str
    side: str

class ClobGetMarketByTokenParams(TypedDict):
    token_id: str

class ClobGetMarketByTokenResponse(TypedDict):
    condition_id: str
    primary_token_id: str
    secondary_token_id: str

class ClobGetMarketLiveActivityParams(TypedDict):
    condition_id: str

_ClobGetMarketLiveActivityResponseOptional = TypedDict("_ClobGetMarketLiveActivityResponseOptional", {'condition_id': 'str | None', 'id': 'int | Decimal | None', 'question': 'str | None', 'market_slug': 'str | None', 'event_slug': 'str | None', 'series_slug': 'str | None', 'icon': 'str | None', 'image': 'str | None', 'tags': 'list[str] | None'}, total=False)
class ClobGetMarketLiveActivityResponse(_ClobGetMarketLiveActivityResponseOptional):
    pass

class ClobGetMarketsLiveActivityParams(TypedDict):
    body: list[str]

class ClobGetMidpointParams(TypedDict):
    token_id: str

class ClobGetMidpointResponse(TypedDict):
    mid_price: str

class ClobGetMidpointsGetParams(TypedDict):
    token_ids: str

class ClobGetMidpointsPostParams(TypedDict):
    body: list[ClobBookRequest]

_ClobGetMultiMarketsParamsOptional = TypedDict("_ClobGetMultiMarketsParamsOptional", {'q': 'str | None', 'tag_slug': 'str | None', 'event_id': 'str | None', 'event_title': 'str | None', 'order_by': 'str | None', 'position': 'str | None', 'min_volume_24hr': 'int | Decimal | None', 'max_volume_24hr': 'int | Decimal | None', 'min_spread': 'int | Decimal | None', 'max_spread': 'int | Decimal | None', 'min_price': 'int | Decimal | None', 'max_price': 'int | Decimal | None', 'next_cursor': 'str | None', 'page_size': 'int | Decimal | None'}, total=False)
class ClobGetMultiMarketsParams(_ClobGetMultiMarketsParamsOptional):
    pass

class ClobGetMultiMarketsResponse(TypedDict):
    limit: int | Decimal
    count: int | Decimal
    next_cursor: str
    data: list[ClobMultiMarketInfo]

class ClobGetNegRiskByPathParams(TypedDict):
    token_id: str

class ClobGetNegRiskByPathResponse(TypedDict):
    neg_risk: bool

_ClobGetNegRiskParamsOptional = TypedDict("_ClobGetNegRiskParamsOptional", {'token_id': 'str | None'}, total=False)
class ClobGetNegRiskParams(_ClobGetNegRiskParamsOptional):
    pass

class ClobGetNegRiskResponse(TypedDict):
    neg_risk: bool

class ClobGetNotificationsParams(TypedDict):
    signature_type: int | Decimal

class ClobGetOrderParams(TypedDict):
    orderID: str

_ClobGetOrderResponseOptional = TypedDict("_ClobGetOrderResponseOptional", {'associate_trades': 'list[str] | None'}, total=False)
class ClobGetOrderResponse(_ClobGetOrderResponseOptional):
    id: str
    status: str
    owner: str
    maker_address: str
    market: str
    asset_id: str
    side: str
    original_size: str
    size_matched: str
    price: str
    outcome: str
    expiration: str
    order_type: str
    created_at: int | Decimal

class ClobGetOrderScoringParams(TypedDict):
    order_id: str

class ClobGetOrderScoringResponse(TypedDict):
    scoring: bool

_ClobGetOrdersParamsOptional = TypedDict("_ClobGetOrdersParamsOptional", {'id': 'str | None', 'market': 'str | None', 'asset_id': 'str | None', 'next_cursor': 'str | None'}, total=False)
class ClobGetOrdersParams(_ClobGetOrdersParamsOptional):
    pass

class ClobGetOrdersResponse(TypedDict):
    limit: int | Decimal
    next_cursor: str
    count: int | Decimal
    data: list[ClobOpenOrder]

class ClobGetOrdersScoringParams(TypedDict):
    order_ids: list[str]

class ClobGetPriceParams(TypedDict):
    token_id: str
    side: str

class ClobGetPriceResponse(TypedDict):
    price: int | Decimal

class ClobGetPricesGetParams(TypedDict):
    token_ids: str
    sides: str

_ClobGetPricesHistoryParamsOptional = TypedDict("_ClobGetPricesHistoryParamsOptional", {'startTs': 'int | Decimal | None', 'endTs': 'int | Decimal | None', 'interval': 'str | None', 'fidelity': 'int | Decimal | None'}, total=False)
class ClobGetPricesHistoryParams(_ClobGetPricesHistoryParamsOptional):
    market: str

_ClobGetPricesHistoryResponseOptional = TypedDict("_ClobGetPricesHistoryResponseOptional", {'history': 'list[ClobMarketPrice] | None'}, total=False)
class ClobGetPricesHistoryResponse(_ClobGetPricesHistoryResponseOptional):
    pass

class ClobGetPricesPostParams(TypedDict):
    body: list[ClobBookRequest]

class ClobGetRateLimitTierParams(TypedDict):
    pass

_ClobGetRateLimitTierResponseOptional = TypedDict("_ClobGetRateLimitTierResponseOptional", {'usdc_volume': 'str | None', 'next_tier_threshold': 'int | Decimal | None'}, total=False)
class ClobGetRateLimitTierResponse(_ClobGetRateLimitTierResponseOptional):
    tier: str
    period: int | Decimal
    tier_details: ClobRateLimitTierDetails

_ClobGetRawRewardsForMarketParamsOptional = TypedDict("_ClobGetRawRewardsForMarketParamsOptional", {'sponsored': 'bool | None', 'next_cursor': 'str | None'}, total=False)
class ClobGetRawRewardsForMarketParams(_ClobGetRawRewardsForMarketParamsOptional):
    condition_id: str

class ClobGetRawRewardsForMarketResponse(TypedDict):
    limit: int | Decimal
    count: int | Decimal
    next_cursor: str
    data: list[ClobMarketReward]

_ClobGetRewardPercentagesForUserParamsOptional = TypedDict("_ClobGetRewardPercentagesForUserParamsOptional", {'signature_type': 'int | Decimal | None', 'maker_address': 'str | None'}, total=False)
class ClobGetRewardPercentagesForUserParams(_ClobGetRewardPercentagesForUserParamsOptional):
    pass

_ClobGetSamplingMarketsParamsOptional = TypedDict("_ClobGetSamplingMarketsParamsOptional", {'next_cursor': 'str | None'}, total=False)
class ClobGetSamplingMarketsParams(_ClobGetSamplingMarketsParamsOptional):
    pass

_ClobGetSamplingMarketsResponseOptional = TypedDict("_ClobGetSamplingMarketsResponseOptional", {'limit': 'int | Decimal | None', 'next_cursor': 'str | None', 'count': 'int | Decimal | None', 'data': 'list[ClobMarket] | None'}, total=False)
class ClobGetSamplingMarketsResponse(_ClobGetSamplingMarketsResponseOptional):
    pass

_ClobGetSamplingSimplifiedMarketsParamsOptional = TypedDict("_ClobGetSamplingSimplifiedMarketsParamsOptional", {'next_cursor': 'str | None'}, total=False)
class ClobGetSamplingSimplifiedMarketsParams(_ClobGetSamplingSimplifiedMarketsParamsOptional):
    pass

_ClobGetSamplingSimplifiedMarketsResponseOptional = TypedDict("_ClobGetSamplingSimplifiedMarketsResponseOptional", {'limit': 'int | Decimal | None', 'next_cursor': 'str | None', 'count': 'int | Decimal | None', 'data': 'list[ClobSimplifiedMarket] | None'}, total=False)
class ClobGetSamplingSimplifiedMarketsResponse(_ClobGetSamplingSimplifiedMarketsResponseOptional):
    pass

_ClobGetSimplifiedMarketsParamsOptional = TypedDict("_ClobGetSimplifiedMarketsParamsOptional", {'next_cursor': 'str | None'}, total=False)
class ClobGetSimplifiedMarketsParams(_ClobGetSimplifiedMarketsParamsOptional):
    pass

_ClobGetSimplifiedMarketsResponseOptional = TypedDict("_ClobGetSimplifiedMarketsResponseOptional", {'limit': 'int | Decimal | None', 'next_cursor': 'str | None', 'count': 'int | Decimal | None', 'data': 'list[ClobSimplifiedMarket] | None'}, total=False)
class ClobGetSimplifiedMarketsResponse(_ClobGetSimplifiedMarketsResponseOptional):
    pass

class ClobGetSpreadParams(TypedDict):
    token_id: str

class ClobGetSpreadResponse(TypedDict):
    spread: str

class ClobGetSpreadsParams(TypedDict):
    body: list[ClobBookRequest]

class ClobGetTickSizeByPathParams(TypedDict):
    token_id: str

class ClobGetTickSizeByPathResponse(TypedDict):
    minimum_tick_size: int | Decimal

_ClobGetTickSizeParamsOptional = TypedDict("_ClobGetTickSizeParamsOptional", {'token_id': 'str | None'}, total=False)
class ClobGetTickSizeParams(_ClobGetTickSizeParamsOptional):
    pass

class ClobGetTickSizeResponse(TypedDict):
    minimum_tick_size: int | Decimal

class ClobGetTimeParams(TypedDict):
    pass

_ClobGetTotalEarningsForUserForDayParamsOptional = TypedDict("_ClobGetTotalEarningsForUserForDayParamsOptional", {'signature_type': 'int | Decimal | None', 'maker_address': 'str | None', 'sponsored': 'bool | None'}, total=False)
class ClobGetTotalEarningsForUserForDayParams(_ClobGetTotalEarningsForUserForDayParamsOptional):
    date: str

_ClobGetTradesParamsOptional = TypedDict("_ClobGetTradesParamsOptional", {'id': 'str | None', 'market': 'str | None', 'asset_id': 'str | None', 'before': 'str | None', 'after': 'str | None', 'next_cursor': 'str | None'}, total=False)
class ClobGetTradesParams(_ClobGetTradesParamsOptional):
    maker_address: str

class ClobGetTradesResponse(TypedDict):
    limit: int | Decimal
    next_cursor: str
    count: int | Decimal
    data: list[ClobTrade]

_ClobGetUpdateBalanceAllowanceParamsOptional = TypedDict("_ClobGetUpdateBalanceAllowanceParamsOptional", {'token_id': 'str | None', 'signature_type': 'int | Decimal | None'}, total=False)
class ClobGetUpdateBalanceAllowanceParams(_ClobGetUpdateBalanceAllowanceParamsOptional):
    asset_type: str

class ClobGetUpdateBalanceAllowanceResponse(TypedDict):
    balance: str
    allowances: dict[str, str]

_ClobGetUserEarningsAndMarketsConfigParamsOptional = TypedDict("_ClobGetUserEarningsAndMarketsConfigParamsOptional", {'date': 'str | None', 'signature_type': 'int | Decimal | None', 'maker_address': 'str | None', 'sponsored': 'bool | None', 'next_cursor': 'str | None', 'page_size': 'int | Decimal | None', 'q': 'str | None', 'tag_slug': 'str | None', 'favorite_markets': 'bool | None', 'no_competition': 'bool | None', 'only_mergeable': 'bool | None', 'only_open_orders': 'bool | None', 'only_open_positions': 'bool | None', 'order_by': 'str | None', 'position': 'str | None'}, total=False)
class ClobGetUserEarningsAndMarketsConfigParams(_ClobGetUserEarningsAndMarketsConfigParamsOptional):
    pass

_ClobGetUserEarningsAndMarketsConfigResponseOptional = TypedDict("_ClobGetUserEarningsAndMarketsConfigResponseOptional", {'total_count': 'int | Decimal | None'}, total=False)
class ClobGetUserEarningsAndMarketsConfigResponse(_ClobGetUserEarningsAndMarketsConfigResponseOptional):
    limit: int | Decimal
    count: int | Decimal
    next_cursor: str
    data: list[ClobUserRewardsMarket]

class ClobHeartbeatErrorResponse(TypedDict):
    error_msg: str
    heartbeat_id: str

class ClobHeartbeatRequest(TypedDict):
    heartbeat_id: str

class ClobHeartbeatResponse(TypedDict):
    status: str

class ClobHeartbeatV1Response(TypedDict):
    heartbeat_id: str

class ClobListSessionSignersParams(TypedDict):
    pass

_ClobLiveActivityMarketOptional = TypedDict("_ClobLiveActivityMarketOptional", {'condition_id': 'str | None', 'id': 'int | Decimal | None', 'question': 'str | None', 'market_slug': 'str | None', 'event_slug': 'str | None', 'series_slug': 'str | None', 'icon': 'str | None', 'image': 'str | None', 'tags': 'list[str] | None'}, total=False)
class ClobLiveActivityMarket(_ClobLiveActivityMarketOptional):
    pass

_ClobMarketOptional = TypedDict("_ClobMarketOptional", {'enable_order_book': 'bool | None', 'active': 'bool | None', 'closed': 'bool | None', 'archived': 'bool | None', 'accepting_orders': 'bool | None', 'accepting_order_timestamp': 'str | None', 'minimum_order_size': 'int | Decimal | None', 'minimum_tick_size': 'int | Decimal | None', 'condition_id': 'str | None', 'question_id': 'str | None', 'question': 'str | None', 'description': 'str | None', 'market_slug': 'str | None', 'end_date_iso': 'str | None', 'game_start_time': 'str | None', 'seconds_delay': 'int | Decimal | None', 'fpmm': 'str | None', 'maker_base_fee': 'int | Decimal | None', 'taker_base_fee': 'int | Decimal | None', 'notifications_enabled': 'bool | None', 'neg_risk': 'bool | None', 'neg_risk_market_id': 'str | None', 'neg_risk_request_id': 'str | None', 'icon': 'str | None', 'image': 'str | None', 'rewards': 'ClobRewards | None', 'is_50_50_outcome': 'bool | None', 'tokens': 'list[ClobToken] | None', 'tags': 'list[str] | None'}, total=False)
class ClobMarket(_ClobMarketOptional):
    pass

class ClobMarketByTokenResponse(TypedDict):
    condition_id: str
    primary_token_id: str
    secondary_token_id: str

_ClobMarketPriceOptional = TypedDict("_ClobMarketPriceOptional", {'t': 'int | Decimal | None', 'p': 'int | Decimal | None'}, total=False)
class ClobMarketPrice(_ClobMarketPriceOptional):
    pass

_ClobMarketRewardOptional = TypedDict("_ClobMarketRewardOptional", {'market_slug': 'str | None', 'event_slug': 'str | None', 'image': 'str | None', 'rewards_max_spread': 'int | Decimal | None', 'rewards_min_size': 'int | Decimal | None', 'market_competitiveness': 'int | Decimal | None', 'rewards_config': 'list[ClobRewardsConfig] | None'}, total=False)
class ClobMarketReward(_ClobMarketRewardOptional):
    condition_id: str
    question: str
    tokens: list[ClobRewardsToken]

_ClobMultiMarketInfoOptional = TypedDict("_ClobMultiMarketInfoOptional", {'event_id': 'str | None', 'event_slug': 'str | None', 'created_at': 'str | None', 'group_item_title': 'str | None', 'image': 'str | None', 'market_competitiveness': 'int | Decimal | None', 'market_slug': 'str | None', 'one_day_price_change': 'int | Decimal | None', 'rewards_max_spread': 'int | Decimal | None', 'rewards_min_size': 'int | Decimal | None', 'spread': 'int | Decimal | None', 'end_date': 'str | None', 'volume_24hr': 'int | Decimal | None', 'rewards_config': 'list[ClobRewardsConfig] | None'}, total=False)
class ClobMultiMarketInfo(_ClobMultiMarketInfoOptional):
    condition_id: str
    market_id: str
    question: str
    tokens: list[ClobRewardsToken]

class ClobNegRisk(TypedDict):
    neg_risk: bool

_ClobNotificationOptional = TypedDict("_ClobNotificationOptional", {'owner': 'str | None'}, total=False)
class ClobNotification(_ClobNotificationOptional):
    id: int | Decimal
    type: int | Decimal
    payload: dict[str, Any]
    timestamp: int | Decimal

_ClobOpenOrderOptional = TypedDict("_ClobOpenOrderOptional", {'associate_trades': 'list[str] | None'}, total=False)
class ClobOpenOrder(_ClobOpenOrderOptional):
    id: str
    status: str
    owner: str
    maker_address: str
    market: str
    asset_id: str
    side: str
    original_size: str
    size_matched: str
    price: str
    outcome: str
    expiration: str
    order_type: str
    created_at: int | Decimal

_ClobOrderOptional = TypedDict("_ClobOrderOptional", {'metadata': 'str | None', 'signature': 'str | None'}, total=False)
class ClobOrder(_ClobOrderOptional):
    maker: str
    signer: str
    tokenId: str
    makerAmount: str
    takerAmount: str
    side: str
    expiration: str
    timestamp: str
    builder: str
    salt: int | Decimal
    signatureType: int | Decimal

class ClobOrderBookSummary(TypedDict):
    market: str
    asset_id: str
    timestamp: str
    hash: str
    bids: list[ClobOrderSummary]
    asks: list[ClobOrderSummary]
    min_order_size: str
    tick_size: str
    neg_risk: bool
    last_trade_price: str

class ClobOrderMarketCancelParams(TypedDict):
    market: str
    asset_id: str

class ClobOrderScoringResponse(TypedDict):
    scoring: bool

class ClobOrderSummary(TypedDict):
    price: str
    size: str

class ClobOrdersResponse(TypedDict):
    limit: int | Decimal
    next_cursor: str
    count: int | Decimal
    data: list[ClobOpenOrder]

class ClobPaginatedCurrentReward(TypedDict):
    limit: int | Decimal
    count: int | Decimal
    next_cursor: str
    data: list[ClobCurrentReward]

class ClobPaginatedMarketReward(TypedDict):
    limit: int | Decimal
    count: int | Decimal
    next_cursor: str
    data: list[ClobMarketReward]

_ClobPaginatedMarketsOptional = TypedDict("_ClobPaginatedMarketsOptional", {'limit': 'int | Decimal | None', 'next_cursor': 'str | None', 'count': 'int | Decimal | None', 'data': 'list[ClobMarket] | None'}, total=False)
class ClobPaginatedMarkets(_ClobPaginatedMarketsOptional):
    pass

class ClobPaginatedMultiMarketInfo(TypedDict):
    limit: int | Decimal
    count: int | Decimal
    next_cursor: str
    data: list[ClobMultiMarketInfo]

_ClobPaginatedSimplifiedMarketsOptional = TypedDict("_ClobPaginatedSimplifiedMarketsOptional", {'limit': 'int | Decimal | None', 'next_cursor': 'str | None', 'count': 'int | Decimal | None', 'data': 'list[ClobSimplifiedMarket] | None'}, total=False)
class ClobPaginatedSimplifiedMarkets(_ClobPaginatedSimplifiedMarketsOptional):
    pass

class ClobPaginatedUserEarnings(TypedDict):
    limit: int | Decimal
    count: int | Decimal
    next_cursor: str
    data: list[ClobUserEarning]

_ClobPaginatedUserRewardsMarketsOptional = TypedDict("_ClobPaginatedUserRewardsMarketsOptional", {'total_count': 'int | Decimal | None'}, total=False)
class ClobPaginatedUserRewardsMarkets(_ClobPaginatedUserRewardsMarketsOptional):
    limit: int | Decimal
    count: int | Decimal
    next_cursor: str
    data: list[ClobUserRewardsMarket]

class ClobPostOrderParams(TypedDict):
    body: ClobSendOrder

_ClobPostOrderResponseOptional = TypedDict("_ClobPostOrderResponseOptional", {'makingAmount': 'str | None', 'takingAmount': 'str | None', 'transactionsHashes': 'list[str] | None', 'tradeIDs': 'list[str] | None', 'errorMsg': 'str | None'}, total=False)
class ClobPostOrderResponse(_ClobPostOrderResponseOptional):
    success: bool
    orderID: str
    status: str

class ClobPostOrdersParams(TypedDict):
    body: list[ClobSendOrder]

class ClobPostOrdersScoringParams(TypedDict):
    body: list[str]

_ClobPricesHistoryResponseOptional = TypedDict("_ClobPricesHistoryResponseOptional", {'history': 'list[ClobMarketPrice] | None'}, total=False)
class ClobPricesHistoryResponse(_ClobPricesHistoryResponseOptional):
    pass

_ClobRateLimitTierDetailsOptional = TypedDict("_ClobRateLimitTierDetailsOptional", {'name': 'str | None', 'volume_threshold': 'int | Decimal | None', 'cancel_rate_per_sec': 'int | Decimal | None', 'order_rate_per_sec': 'int | Decimal | None', 'order_burst': 'int | Decimal | None', 'cancel_burst': 'int | Decimal | None', 'allow_negative': 'bool | None'}, total=False)
class ClobRateLimitTierDetails(_ClobRateLimitTierDetailsOptional):
    pass

_ClobRateLimitTierResponseOptional = TypedDict("_ClobRateLimitTierResponseOptional", {'usdc_volume': 'str | None', 'next_tier_threshold': 'int | Decimal | None'}, total=False)
class ClobRateLimitTierResponse(_ClobRateLimitTierResponseOptional):
    tier: str
    period: int | Decimal
    tier_details: ClobRateLimitTierDetails

class ClobRebatedFees(TypedDict):
    date: str
    condition_id: str
    asset_address: str
    maker_address: str
    rebated_fees_usdc: str

class ClobRevokeBuilderApiKeyParams(TypedDict):
    pass

_ClobRewardsOptional = TypedDict("_ClobRewardsOptional", {'rates': 'list[ClobRewardsRatesItem] | None', 'min_size': 'int | Decimal | None', 'max_spread': 'int | Decimal | None'}, total=False)
class ClobRewards(_ClobRewardsOptional):
    pass

_ClobRewardsConfigOptional = TypedDict("_ClobRewardsConfigOptional", {'id': 'int | Decimal | None', 'end_date': 'str | None', 'total_rewards': 'int | Decimal | None', 'remaining_reward_amount': 'int | Decimal | None', 'total_days': 'int | Decimal | None'}, total=False)
class ClobRewardsConfig(_ClobRewardsConfigOptional):
    asset_address: str
    start_date: str
    rate_per_day: int | Decimal

_ClobRewardsRatesItemOptional = TypedDict("_ClobRewardsRatesItemOptional", {'asset_address': 'str | None', 'rewards_daily_rate': 'int | Decimal | None'}, total=False)
class ClobRewardsRatesItem(_ClobRewardsRatesItemOptional):
    pass

_ClobRewardsTokenOptional = TypedDict("_ClobRewardsTokenOptional", {'price': 'int | Decimal | None'}, total=False)
class ClobRewardsToken(_ClobRewardsTokenOptional):
    token_id: str
    outcome: str

class ClobSendHeartbeatParams(TypedDict):
    pass

class ClobSendHeartbeatResponse(TypedDict):
    status: str

class ClobSendHeartbeatV1Params(TypedDict):
    body: ClobHeartbeatRequest

class ClobSendHeartbeatV1Response(TypedDict):
    heartbeat_id: str

_ClobSendOrderOptional = TypedDict("_ClobSendOrderOptional", {'orderType': 'str | None', 'deferExec': 'bool | None', 'postOnly': 'bool | None'}, total=False)
class ClobSendOrder(_ClobSendOrderOptional):
    order: ClobOrder
    owner: str

_ClobSendOrderResponseOptional = TypedDict("_ClobSendOrderResponseOptional", {'makingAmount': 'str | None', 'takingAmount': 'str | None', 'transactionsHashes': 'list[str] | None', 'tradeIDs': 'list[str] | None', 'errorMsg': 'str | None'}, total=False)
class ClobSendOrderResponse(_ClobSendOrderResponseOptional):
    success: bool
    orderID: str
    status: str

_ClobSimplifiedMarketOptional = TypedDict("_ClobSimplifiedMarketOptional", {'condition_id': 'str | None', 'rewards': 'ClobRewards | None', 'tokens': 'list[ClobToken] | None', 'active': 'bool | None', 'closed': 'bool | None', 'archived': 'bool | None', 'accepting_orders': 'bool | None'}, total=False)
class ClobSimplifiedMarket(_ClobSimplifiedMarketOptional):
    pass

class ClobTickSize(TypedDict):
    minimum_tick_size: int | Decimal

_ClobTokenOptional = TypedDict("_ClobTokenOptional", {'token_id': 'str | None', 'outcome': 'str | None', 'price': 'int | Decimal | None', 'winner': 'bool | None'}, total=False)
class ClobToken(_ClobTokenOptional):
    pass

_ClobTotalUserEarningOptional = TypedDict("_ClobTotalUserEarningOptional", {'native_earnings': 'int | Decimal | None', 'sponsored_earnings': 'int | Decimal | None'}, total=False)
class ClobTotalUserEarning(_ClobTotalUserEarningOptional):
    date: str
    asset_address: str
    maker_address: str
    earnings: int | Decimal
    asset_rate: int | Decimal

_ClobTradeOptional = TypedDict("_ClobTradeOptional", {'fee_rate_bps': 'str | None', 'match_time_nano': 'str | None', 'transaction_hash': 'str | None', 'err_msg': 'str | None', 'maker_orders': 'list[ClobTradeMakerOrdersItem] | None'}, total=False)
class ClobTrade(_ClobTradeOptional):
    id: str
    taker_order_id: str
    market: str
    asset_id: str
    side: str
    size: str
    price: str
    status: str
    match_time: str
    last_update: str
    outcome: str
    bucket_index: int | Decimal
    owner: str
    maker_address: str
    trader_side: str

_ClobTradeMakerOrdersItemOptional = TypedDict("_ClobTradeMakerOrdersItemOptional", {'order_id': 'str | None', 'owner': 'str | None', 'maker_address': 'str | None', 'matched_amount': 'str | None', 'price': 'str | None', 'fee_rate_bps': 'str | None', 'asset_id': 'str | None', 'outcome': 'str | None', 'side': 'str | None'}, total=False)
class ClobTradeMakerOrdersItem(_ClobTradeMakerOrdersItemOptional):
    pass

class ClobTradesResponse(TypedDict):
    limit: int | Decimal
    next_cursor: str
    count: int | Decimal
    data: list[ClobTrade]

_ClobUpdateBalanceAllowanceParamsOptional = TypedDict("_ClobUpdateBalanceAllowanceParamsOptional", {'token_id': 'str | None', 'signature_type': 'int | Decimal | None'}, total=False)
class ClobUpdateBalanceAllowanceParams(_ClobUpdateBalanceAllowanceParamsOptional):
    asset_type: str

class ClobUpdateBalanceAllowanceResponse(TypedDict):
    pass

_ClobUserEarningOptional = TypedDict("_ClobUserEarningOptional", {'asset_rate': 'int | Decimal | None'}, total=False)
class ClobUserEarning(_ClobUserEarningOptional):
    date: str
    condition_id: str
    asset_address: str
    maker_address: str
    earnings: int | Decimal

_ClobUserRewardsMarketOptional = TypedDict("_ClobUserRewardsMarketOptional", {'event_id': 'str | None', 'market_slug': 'str | None', 'event_slug': 'str | None', 'image': 'str | None', 'rewards_max_spread': 'int | Decimal | None', 'rewards_min_size': 'int | Decimal | None', 'volume_24hr': 'int | Decimal | None', 'spread': 'int | Decimal | None', 'market_competitiveness': 'int | Decimal | None', 'rewards_config': 'list[ClobCurrentRewardConfig] | None', 'maker_address': 'str | None', 'earning_percentage': 'int | Decimal | None', 'earnings': 'list[ClobAssetEarning] | None'}, total=False)
class ClobUserRewardsMarket(_ClobUserRewardsMarketOptional):
    condition_id: str
    market_id: str
    question: str
    tokens: list[ClobRewardsToken]

class CollateralReturnPlanParams(TypedDict):
    body: CollateralReturnPlanParamsBody

class CollateralReturnPlanParamsBody(TypedDict):
    wallet: str

class CollateralReturnSubmitParams(TypedDict):
    body: CollateralReturnSubmitParamsBody

class CollateralReturnSubmitParamsBody(TypedDict):
    plan_hash: str
    envelope: dict[str, Any]

class CombosRfqCancelMakerQuoteParams(TypedDict):
    body: CombosRfqCancelQuoteRequest

_CombosRfqCancelMakerQuoteResponseOptional = TypedDict("_CombosRfqCancelMakerQuoteResponseOptional", {'competition_started_at': 'int | Decimal | None', 'competition_ends_at': 'int | Decimal | None', 'confirmation_started_at': 'int | Decimal | None', 'confirmation_ends_at': 'int | Decimal | None', 'quote_id': 'str | None', 'bundle': 'CombosRfqFillBundle | None', 'maker_confirmations': 'list[CombosRfqMakerConfirmationSnapshot] | None'}, total=False)
class CombosRfqCancelMakerQuoteResponse(_CombosRfqCancelMakerQuoteResponseOptional):
    request: CombosRfqRFQRequest
    status: str

class CombosRfqCancelQuoteRequest(TypedDict):
    rfq_id: str
    quote_id: str
    signer_address: str
    maker_address: str
    signature_type: int | Decimal

class CombosRfqComboMarket(TypedDict):
    id: str
    condition_id: str
    position_ids: list[str]
    slug: str
    title: str
    outcomes: list[str]
    outcome_prices: list[str]
    image: str
    volume: int | Decimal
    tags: list[str]

class CombosRfqComboMarketsResponse(TypedDict):
    markets: list[CombosRfqComboMarket]
    next_cursor: str | None

class CombosRfqErrorResponse(TypedDict):
    error: str

_CombosRfqExchangeV3OrderOptional = TypedDict("_CombosRfqExchangeV3OrderOptional", {'metadata': 'str | None', 'builder': 'str | None', 'signature': 'str | None'}, total=False)
class CombosRfqExchangeV3Order(_CombosRfqExchangeV3OrderOptional):
    salt: str
    maker: str
    signer: str
    tokenId: str
    makerAmount: str
    takerAmount: str
    side: int | Decimal
    signatureType: int | Decimal
    timestamp: str

_CombosRfqExecutionHandoffOptional = TypedDict("_CombosRfqExecutionHandoffOptional", {'reservations': 'list[CombosRfqWalletReservation] | None', 'ready_at': 'int | Decimal | None'}, total=False)
class CombosRfqExecutionHandoff(_CombosRfqExecutionHandoffOptional):
    execution_id: str
    request: CombosRfqRFQRequest
    quote_id: str
    bundle: CombosRfqFillBundle
    requester_acceptance: CombosRfqRequesterAcceptance
    maker_quotes: list[CombosRfqQuote]

_CombosRfqFillAllocationOptional = TypedDict("_CombosRfqFillAllocationOptional", {'received_at': 'int | Decimal | None'}, total=False)
class CombosRfqFillAllocation(_CombosRfqFillAllocationOptional):
    maker_quote_id: str
    signer_address: str
    maker_address: str
    size_e6: str
    price_e6: str

_CombosRfqFillBundleOptional = TypedDict("_CombosRfqFillBundleOptional", {'requested_notional_e6': 'str | None'}, total=False)
class CombosRfqFillBundle(_CombosRfqFillBundleOptional):
    requested_shares_e6: str
    blended_price_e6: str
    allocations: list[CombosRfqFillAllocation]

_CombosRfqGetComboMarketsParamsOptional = TypedDict("_CombosRfqGetComboMarketsParamsOptional", {'limit': 'int | Decimal | None', 'cursor': 'str | None', 'exclude': 'str | None'}, total=False)
class CombosRfqGetComboMarketsParams(_CombosRfqGetComboMarketsParamsOptional):
    pass

class CombosRfqGetComboMarketsResponse(TypedDict):
    markets: list[CombosRfqComboMarket]
    next_cursor: str | None

class CombosRfqMakerConfirmationRequest(TypedDict):
    rfq_id: str
    quote_id: str
    signer_address: str
    maker_address: str
    signature_type: int | Decimal
    decision: str

_CombosRfqMakerConfirmationResultOptional = TypedDict("_CombosRfqMakerConfirmationResultOptional", {'snapshot': 'CombosRfqRFQSnapshot | None', 'execution': 'CombosRfqExecutionHandoff | None'}, total=False)
class CombosRfqMakerConfirmationResult(_CombosRfqMakerConfirmationResultOptional):
    pass

_CombosRfqMakerConfirmationSnapshotOptional = TypedDict("_CombosRfqMakerConfirmationSnapshotOptional", {'decision': 'str | None', 'reason': 'str | None', 'responded_at': 'int | Decimal | None'}, total=False)
class CombosRfqMakerConfirmationSnapshot(_CombosRfqMakerConfirmationSnapshotOptional):
    quote_id: str
    signer_address: str
    maker_address: str

_CombosRfqQuoteOptional = TypedDict("_CombosRfqQuoteOptional", {'auth_address': 'str | None', 'valid_until': 'int | Decimal | None', 'received_at': 'int | Decimal | None'}, total=False)
class CombosRfqQuote(_CombosRfqQuoteOptional):
    quote_id: str
    rfq_id: str
    signer_address: str
    maker_address: str
    signature_type: int | Decimal
    price_e6: str
    size_e6: str
    signed_order: CombosRfqExchangeV3Order

_CombosRfqRFQRequestOptional = TypedDict("_CombosRfqRFQRequestOptional", {'auth_address': 'str | None', 'signer_address': 'str | None', 'maker_address': 'str | None', 'signature_type': 'int | Decimal | None', 'requestor_public_id': 'str | None', 'condition_id': 'str | None', 'yes_position_id': 'str | None', 'no_position_id': 'str | None', 'requested_size': 'CombosRfqRequestedSize | None', 'created_at': 'int | Decimal | None'}, total=False)
class CombosRfqRFQRequest(_CombosRfqRFQRequestOptional):
    rfq_id: str
    leg_position_ids: list[str]
    direction: str
    side: str

_CombosRfqRFQSnapshotOptional = TypedDict("_CombosRfqRFQSnapshotOptional", {'competition_started_at': 'int | Decimal | None', 'competition_ends_at': 'int | Decimal | None', 'confirmation_started_at': 'int | Decimal | None', 'confirmation_ends_at': 'int | Decimal | None', 'quote_id': 'str | None', 'bundle': 'CombosRfqFillBundle | None', 'maker_confirmations': 'list[CombosRfqMakerConfirmationSnapshot] | None'}, total=False)
class CombosRfqRFQSnapshot(_CombosRfqRFQSnapshotOptional):
    request: CombosRfqRFQRequest
    status: str

class CombosRfqRequestedSize(TypedDict):
    unit: str
    value_e6: str

_CombosRfqRequesterAcceptanceOptional = TypedDict("_CombosRfqRequesterAcceptanceOptional", {'auth_address': 'str | None', 'signer_address': 'str | None', 'maker_address': 'str | None', 'signature_type': 'int | Decimal | None', 'accepted_at': 'int | Decimal | None'}, total=False)
class CombosRfqRequesterAcceptance(_CombosRfqRequesterAcceptanceOptional):
    rfq_id: str
    quote_id: str
    signed_order: CombosRfqExchangeV3Order

class CombosRfqSubmitMakerConfirmationParams(TypedDict):
    body: CombosRfqMakerConfirmationRequest

_CombosRfqSubmitMakerConfirmationResponseOptional = TypedDict("_CombosRfqSubmitMakerConfirmationResponseOptional", {'snapshot': 'CombosRfqRFQSnapshot | None', 'execution': 'CombosRfqExecutionHandoff | None'}, total=False)
class CombosRfqSubmitMakerConfirmationResponse(_CombosRfqSubmitMakerConfirmationResponseOptional):
    pass

class CombosRfqSubmitMakerQuoteParams(TypedDict):
    body: CombosRfqQuote

_CombosRfqSubmitMakerQuoteResponseOptional = TypedDict("_CombosRfqSubmitMakerQuoteResponseOptional", {'competition_started_at': 'int | Decimal | None', 'competition_ends_at': 'int | Decimal | None', 'confirmation_started_at': 'int | Decimal | None', 'confirmation_ends_at': 'int | Decimal | None', 'quote_id': 'str | None', 'bundle': 'CombosRfqFillBundle | None', 'maker_confirmations': 'list[CombosRfqMakerConfirmationSnapshot] | None'}, total=False)
class CombosRfqSubmitMakerQuoteResponse(_CombosRfqSubmitMakerQuoteResponseOptional):
    request: CombosRfqRFQRequest
    status: str

class CombosRfqWalletAssetDelta(TypedDict):
    asset: str
    asset_id: str
    amount: str

class CombosRfqWalletReservation(TypedDict):
    action_id: str
    user: str
    wallet_nonce: int | Decimal
    deltas: list[CombosRfqWalletAssetDelta]

_DataActivityOptional = TypedDict("_DataActivityOptional", {'is_combo': 'bool | None'}, total=False)
class DataActivity(_DataActivityOptional):
    bio: str
    condition_id: str
    event_slug: str
    icon: str
    name: str
    outcome: str
    outcome_index: int | Decimal
    price: int | Decimal
    profile_image: str
    profile_image_optimized: str
    proxy_wallet: str
    pseudonym: str
    side: str
    size: int | Decimal
    slug: str
    timestamp: int | Decimal
    title: str
    token_id: str
    transaction_hash: str
    type: str
    usdc_size: int | Decimal

class DataActivityPage(TypedDict):
    data: list[DataActivity]
    pagination: DataPagination

_DataApprovalContractOptional = TypedDict("_DataApprovalContractOptional", {'amount': 'str | None'}, total=False)
class DataApprovalContract(_DataApprovalContractOptional):
    approved: bool
    feature: str
    id: str
    spender: str
    standard: str
    token: str

class DataApprovals(TypedDict):
    address: str
    chain_id: int | Decimal
    checked_at: str
    contracts: list[DataApprovalContract]

class DataBiggestWinner(TypedDict):
    condition_id: str
    event_id: int | Decimal
    event_slug: str
    event_title: str
    final_value: int | Decimal
    initial_value: int | Decimal
    kind: str
    pnl: int | Decimal
    position_id: str
    profile_image: str
    resolved_at: int | Decimal
    user_id: str
    user_name: str
    win_rank: int | Decimal

class DataBiggestWinnersPage(TypedDict):
    data: list[DataBiggestWinner]
    pagination: DataPagination

class DataBuilderStanding(TypedDict):
    active_users: int | Decimal
    builder_code: str
    builder_name: str
    profile_image: str
    rank: int | Decimal
    verified: bool
    volume: int | Decimal

class DataBuilderVolumePoint(TypedDict):
    active_users: int | Decimal
    builder_code: str
    builder_name: str
    date: str
    profile_image: str
    rank: int | Decimal
    verified: bool
    volume: int | Decimal

class DataBuildersLeaderboardPage(TypedDict):
    data: list[DataBuilderStanding]
    pagination: DataPagination

_DataComboActivityOptional = TypedDict("_DataComboActivityOptional", {'amount_usdc': 'int | Decimal | None', 'payout_usdc': 'int | Decimal | None'}, total=False)
class DataComboActivity(_DataComboActivityOptional):
    block_number: int | Decimal
    combo_condition_id: str
    combo_position_id: str
    id: str
    legs: list[DataComboLeg]
    proxy_wallet: str
    timestamp: int | Decimal
    transaction_hash: str
    type: str

class DataComboActivityPage(TypedDict):
    data: list[DataComboActivity]
    pagination: DataPagination

_DataComboLegOptional = TypedDict("_DataComboLegOptional", {'leg_resolved_at': 'str | None'}, total=False)
class DataComboLeg(_DataComboLegOptional):
    leg_condition_id: str
    leg_current_price: int | Decimal
    leg_index: int | Decimal
    leg_outcome_index: int | Decimal
    leg_outcome_label: str
    leg_position_id: str
    leg_status: str
    market: DataComboLegMarket

class DataComboLegEvent(TypedDict):
    event_id: str
    event_image: str
    event_slug: str
    event_title: str

_DataComboLegMarketOptional = TypedDict("_DataComboLegMarketOptional", {'group_item_title': 'str | None', 'line': 'int | Decimal | None', 'outcomes': 'list[str] | None', 'question': 'str | None', 'sports_market_type': 'str | None'}, total=False)
class DataComboLegMarket(_DataComboLegMarketOptional):
    category: str
    end_date: str
    event: DataComboLegEvent
    icon_url: str
    image_url: str
    market_id: str
    outcome: str
    slug: str
    subcategory: str
    tags: list[str]
    title: str

_DataComboPositionOptional = TypedDict("_DataComboPositionOptional", {'first_entry_at_micros': 'int | Decimal | None', 'resolved_at': 'str | None'}, total=False)
class DataComboPosition(_DataComboPositionOptional):
    combo_condition_id: str
    combo_position_id: str
    current_size: int | Decimal
    entry_avg_price_usdc: int | Decimal
    entry_cost_usdc: int | Decimal
    entry_fees_usdc: int | Decimal
    first_entry_at: str
    gross_entry_cost_usdc: int | Decimal
    legs: list[DataComboLeg]
    legs_pending: int | Decimal
    legs_resolved: int | Decimal
    legs_total: int | Decimal
    outcome_index: int | Decimal
    outcome_label: str
    proxy_wallet: str
    realized_payout_usdc: int | Decimal
    redeemable: bool
    status: str
    updated_at: str
    updated_at_micros: int | Decimal

class DataComboPositionsPage(TypedDict):
    data: list[DataComboPosition]
    pagination: DataPagination

class DataConditionVolume(TypedDict):
    condition_id: str
    taker_volume: int | Decimal

class DataCursorLag(TypedDict):
    behind_max: int | Decimal
    block: int | Decimal
    source: str

class DataEnvelopeApprovals(TypedDict):
    data: DataEnvelopeApprovalsData

class DataEnvelopeApprovalsData(TypedDict):
    address: str
    chain_id: int | Decimal
    checked_at: str
    contracts: list[DataApprovalContract]

class DataEnvelopeLiveVolume(TypedDict):
    data: DataEnvelopeLiveVolumeData

class DataEnvelopeLiveVolumeData(TypedDict):
    conditions: list[DataConditionVolume]
    taker_volume_total: int | Decimal

class DataEnvelopeOptionLeaderboardUserEntry(TypedDict):
    data: Any

class DataEnvelopeOptionUserStats(TypedDict):
    data: Any

class DataEnvelopePortfolioValue(TypedDict):
    data: DataEnvelopePortfolioValueData

class DataEnvelopePortfolioValueData(TypedDict):
    proxy_wallet: str
    value: int | Decimal

class DataEnvelopeServiceStatus(TypedDict):
    data: DataEnvelopeServiceStatusData

class DataEnvelopeServiceStatusData(TypedDict):
    age_seconds: int | Decimal
    computed_at: str
    ingestion: DataIngestionFreshness
    serving: DataServingFreshness

class DataEnvelopeUserPnlSeries(TypedDict):
    data: DataEnvelopeUserPnlSeriesData

class DataEnvelopeUserPnlSeriesData(TypedDict):
    fidelity: str
    interval: str
    points: list[DataUserPnlPoint]
    proxy_wallet: str
    source_fidelity: str

class DataEnvelopeUserVolume(TypedDict):
    data: DataEnvelopeUserVolumeData

class DataEnvelopeUserVolumeData(TypedDict):
    trade_count: int | Decimal
    volume: int | Decimal
    volume_usdc: int | Decimal

class DataEnvelopeVecBuilderVolumePoint(TypedDict):
    data: list[DataEnvelopeVecBuilderVolumePointDataItem]

class DataEnvelopeVecBuilderVolumePointDataItem(TypedDict):
    active_users: int | Decimal
    builder_code: str
    builder_name: str
    date: str
    profile_image: str
    rank: int | Decimal
    verified: bool
    volume: int | Decimal

class DataEnvelopeVecOpenInterest(TypedDict):
    data: list[DataEnvelopeVecOpenInterestDataItem]

class DataEnvelopeVecOpenInterestDataItem(TypedDict):
    condition_id: str
    value: int | Decimal

class DataEnvelopeVecResolution(TypedDict):
    data: list[DataEnvelopeVecResolutionDataItem]

_DataEnvelopeVecResolutionDataItemOptional = TypedDict("_DataEnvelopeVecResolutionDataItemOptional", {'condition_id': 'str | None', 'expected_settlement_time': 'str | None', 'market_type': 'str | None', 'payouts': 'list[int | Decimal] | None', 'price': 'str | None', 'proposed_price': 'str | None', 'question_id': 'str | None', 'reporter': 'str | None', 'reproposed_price': 'str | None', 'resolution_source': 'str | None', 'resolved_at': 'str | None', 'resolved_block': 'int | Decimal | None', 'settlement_time_basis': 'str | None', 'was_arbitrated': 'bool | None'}, total=False)
class DataEnvelopeVecResolutionDataItem(_DataEnvelopeVecResolutionDataItemOptional):
    extended_review: bool
    last_update_timestamp: str
    log_index: str
    new_version_q: bool
    status: str
    transaction_hash: str
    was_disputed: bool

class DataEnvelopeVecTokenRef(TypedDict):
    data: list[DataEnvelopeVecTokenRefDataItem]

_DataEnvelopeVecTokenRefDataItemOptional = TypedDict("_DataEnvelopeVecTokenRefDataItemOptional", {'clob_index': 'int | Decimal | None', 'closed': 'bool | None', 'event_id': 'int | Decimal | None', 'event_slug': 'str | None', 'final_price': 'int | Decimal | None', 'market_slug': 'str | None', 'neg_risk': 'bool | None', 'neg_risk_market_id': 'str | None', 'opposite_token_id': 'str | None', 'outcome': 'str | None', 'question_index': 'int | Decimal | None', 'structural_condition_id': 'str | None', 'title': 'str | None'}, total=False)
class DataEnvelopeVecTokenRefDataItem(_DataEnvelopeVecTokenRefDataItemOptional):
    condition_id: str
    module: str
    outcome_index: int | Decimal
    resolved: bool
    token_id: str

_DataErrorResponseOptional = TypedDict("_DataErrorResponseOptional", {'parameter': 'str | None'}, total=False)
class DataErrorResponse(_DataErrorResponseOptional):
    code: str
    error: str
    retryable: bool
    trace_id: str

_DataGetActivityParamsOptional = TypedDict("_DataGetActivityParamsOptional", {'user': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'type': 'str | None', 'condition': 'str | None', 'event_id': 'str | None', 'side': 'str | None', 'start': 'int | Decimal | None', 'end': 'int | Decimal | None', 'sort_by': 'str | None', 'sort_direction': 'str | None', 'exclude_deposits_withdrawals': 'bool | None'}, total=False)
class DataGetActivityParams(_DataGetActivityParamsOptional):
    pass

class DataGetActivityResponse(TypedDict):
    data: list[DataActivity]
    pagination: DataPagination

_DataGetApprovalsParamsOptional = TypedDict("_DataGetApprovalsParamsOptional", {'user': 'str | None'}, total=False)
class DataGetApprovalsParams(_DataGetApprovalsParamsOptional):
    pass

class DataGetApprovalsResponse(TypedDict):
    data: DataGetApprovalsResponseData

class DataGetApprovalsResponseData(TypedDict):
    address: str
    chain_id: int | Decimal
    checked_at: str
    contracts: list[DataApprovalContract]

_DataGetBiggestWinnersParamsOptional = TypedDict("_DataGetBiggestWinnersParamsOptional", {'time_period': 'str | None', 'category': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class DataGetBiggestWinnersParams(_DataGetBiggestWinnersParamsOptional):
    pass

class DataGetBiggestWinnersResponse(TypedDict):
    data: list[DataBiggestWinner]
    pagination: DataPagination

_DataGetBuildersLeaderboardParamsOptional = TypedDict("_DataGetBuildersLeaderboardParamsOptional", {'time_period': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class DataGetBuildersLeaderboardParams(_DataGetBuildersLeaderboardParamsOptional):
    pass

class DataGetBuildersLeaderboardResponse(TypedDict):
    data: list[DataBuilderStanding]
    pagination: DataPagination

_DataGetBuildersVolumeParamsOptional = TypedDict("_DataGetBuildersVolumeParamsOptional", {'interval': 'str | None', 'limit': 'int | Decimal | None'}, total=False)
class DataGetBuildersVolumeParams(_DataGetBuildersVolumeParamsOptional):
    pass

class DataGetBuildersVolumeResponse(TypedDict):
    data: list[DataGetBuildersVolumeResponseDataItem]

class DataGetBuildersVolumeResponseDataItem(TypedDict):
    active_users: int | Decimal
    builder_code: str
    builder_name: str
    date: str
    profile_image: str
    rank: int | Decimal
    verified: bool
    volume: int | Decimal

_DataGetComboPositionsParamsOptional = TypedDict("_DataGetComboPositionsParamsOptional", {'user': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'condition': 'str | None', 'status': 'str | None', 'sort_by': 'str | None', 'sort_direction': 'str | None', 'updated_after': 'int | Decimal | None', 'updated_before': 'int | Decimal | None'}, total=False)
class DataGetComboPositionsParams(_DataGetComboPositionsParamsOptional):
    pass

class DataGetComboPositionsResponse(TypedDict):
    data: list[DataComboPosition]
    pagination: DataPagination

_DataGetCombosParamsOptional = TypedDict("_DataGetCombosParamsOptional", {'user': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'condition': 'str | None'}, total=False)
class DataGetCombosParams(_DataGetCombosParamsOptional):
    pass

class DataGetCombosResponse(TypedDict):
    data: list[DataComboActivity]
    pagination: DataPagination

_DataGetHoldersParamsOptional = TypedDict("_DataGetHoldersParamsOptional", {'condition': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'min_balance': 'int | Decimal | None', 'include_pnl': 'bool | None'}, total=False)
class DataGetHoldersParams(_DataGetHoldersParamsOptional):
    pass

class DataGetHoldersResponse(TypedDict):
    data: list[DataMetaHolder]
    pagination: DataPagination

_DataGetLeaderboardParamsOptional = TypedDict("_DataGetLeaderboardParamsOptional", {'time_period': 'str | None', 'category': 'str | None', 'sort_by': 'str | None', 'user': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class DataGetLeaderboardParams(_DataGetLeaderboardParamsOptional):
    pass

_DataGetLiveVolumeParamsOptional = TypedDict("_DataGetLiveVolumeParamsOptional", {'event_id': 'str | None'}, total=False)
class DataGetLiveVolumeParams(_DataGetLiveVolumeParamsOptional):
    pass

class DataGetLiveVolumeResponse(TypedDict):
    data: DataGetLiveVolumeResponseData

class DataGetLiveVolumeResponseData(TypedDict):
    conditions: list[DataConditionVolume]
    taker_volume_total: int | Decimal

_DataGetOiParamsOptional = TypedDict("_DataGetOiParamsOptional", {'condition': 'str | None'}, total=False)
class DataGetOiParams(_DataGetOiParamsOptional):
    pass

class DataGetOiResponse(TypedDict):
    data: list[DataGetOiResponseDataItem]

class DataGetOiResponseDataItem(TypedDict):
    condition_id: str
    value: int | Decimal

_DataGetPositionsParamsOptional = TypedDict("_DataGetPositionsParamsOptional", {'user': 'str | None', 'condition': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'status': 'str | None', 'event_id': 'str | None', 'title': 'str | None', 'filter_type': 'str | None', 'filter_amount': 'int | Decimal | None', 'include_archived': 'bool | None', 'sort_by': 'str | None', 'start': 'int | Decimal | None', 'end': 'int | Decimal | None', 'sort_direction': 'str | None'}, total=False)
class DataGetPositionsParams(_DataGetPositionsParamsOptional):
    pass

class DataGetPositionsResponse(TypedDict):
    data: list[DataPosition]
    pagination: DataPagination

_DataGetPricesHistoryParamsOptional = TypedDict("_DataGetPricesHistoryParamsOptional", {'token_id': 'str | None', 'start': 'int | Decimal | None', 'end': 'int | Decimal | None', 'interval': 'str | None', 'bucket_seconds': 'int | Decimal | None', 'as_of': 'int | Decimal | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class DataGetPricesHistoryParams(_DataGetPricesHistoryParamsOptional):
    pass

class DataGetPricesHistoryResponse(TypedDict):
    data: list[DataPricePoint]
    pagination: DataPagination

_DataGetResolutionsParamsOptional = TypedDict("_DataGetResolutionsParamsOptional", {'question_id': 'str | None', 'condition': 'str | None', 'event_id': 'str | None'}, total=False)
class DataGetResolutionsParams(_DataGetResolutionsParamsOptional):
    pass

class DataGetResolutionsResponse(TypedDict):
    data: list[DataGetResolutionsResponseDataItem]

_DataGetResolutionsResponseDataItemOptional = TypedDict("_DataGetResolutionsResponseDataItemOptional", {'condition_id': 'str | None', 'expected_settlement_time': 'str | None', 'market_type': 'str | None', 'payouts': 'list[int | Decimal] | None', 'price': 'str | None', 'proposed_price': 'str | None', 'question_id': 'str | None', 'reporter': 'str | None', 'reproposed_price': 'str | None', 'resolution_source': 'str | None', 'resolved_at': 'str | None', 'resolved_block': 'int | Decimal | None', 'settlement_time_basis': 'str | None', 'was_arbitrated': 'bool | None'}, total=False)
class DataGetResolutionsResponseDataItem(_DataGetResolutionsResponseDataItemOptional):
    extended_review: bool
    last_update_timestamp: str
    log_index: str
    new_version_q: bool
    status: str
    transaction_hash: str
    was_disputed: bool

class DataGetStatusParams(TypedDict):
    pass

class DataGetStatusResponse(TypedDict):
    data: DataGetStatusResponseData

class DataGetStatusResponseData(TypedDict):
    age_seconds: int | Decimal
    computed_at: str
    ingestion: DataIngestionFreshness
    serving: DataServingFreshness

_DataGetTokensParamsOptional = TypedDict("_DataGetTokensParamsOptional", {'token_id': 'str | None', 'condition': 'str | None'}, total=False)
class DataGetTokensParams(_DataGetTokensParamsOptional):
    pass

class DataGetTokensResponse(TypedDict):
    data: list[DataGetTokensResponseDataItem]

_DataGetTokensResponseDataItemOptional = TypedDict("_DataGetTokensResponseDataItemOptional", {'clob_index': 'int | Decimal | None', 'closed': 'bool | None', 'event_id': 'int | Decimal | None', 'event_slug': 'str | None', 'final_price': 'int | Decimal | None', 'market_slug': 'str | None', 'neg_risk': 'bool | None', 'neg_risk_market_id': 'str | None', 'opposite_token_id': 'str | None', 'outcome': 'str | None', 'question_index': 'int | Decimal | None', 'structural_condition_id': 'str | None', 'title': 'str | None'}, total=False)
class DataGetTokensResponseDataItem(_DataGetTokensResponseDataItemOptional):
    condition_id: str
    module: str
    outcome_index: int | Decimal
    resolved: bool
    token_id: str

_DataGetTradesParamsOptional = TypedDict("_DataGetTradesParamsOptional", {'user': 'str | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None', 'taker_only': 'bool | None', 'filter_type': 'str | None', 'filter_amount': 'int | Decimal | None', 'start': 'int | Decimal | None', 'end': 'int | Decimal | None', 'condition': 'str | None', 'event_id': 'str | None', 'side': 'str | None'}, total=False)
class DataGetTradesParams(_DataGetTradesParamsOptional):
    pass

class DataGetTradesResponse(TypedDict):
    data: list[DataTrade]
    pagination: DataPagination

_DataGetUserPnlV2ParamsOptional = TypedDict("_DataGetUserPnlV2ParamsOptional", {'user': 'str | None', 'interval': 'str | None', 'fidelity': 'str | None'}, total=False)
class DataGetUserPnlV2Params(_DataGetUserPnlV2ParamsOptional):
    pass

class DataGetUserPnlV2Response(TypedDict):
    data: DataGetUserPnlV2ResponseData

class DataGetUserPnlV2ResponseData(TypedDict):
    fidelity: str
    interval: str
    points: list[DataUserPnlPoint]
    proxy_wallet: str
    source_fidelity: str

_DataGetUserStatsParamsOptional = TypedDict("_DataGetUserStatsParamsOptional", {'user': 'str | None'}, total=False)
class DataGetUserStatsParams(_DataGetUserStatsParamsOptional):
    pass

class DataGetUserStatsResponse(TypedDict):
    data: Any

_DataGetUserVolumeParamsOptional = TypedDict("_DataGetUserVolumeParamsOptional", {'user': 'str | None', 'start': 'int | Decimal | None', 'end': 'int | Decimal | None'}, total=False)
class DataGetUserVolumeParams(_DataGetUserVolumeParamsOptional):
    pass

class DataGetUserVolumeResponse(TypedDict):
    data: DataGetUserVolumeResponseData

class DataGetUserVolumeResponseData(TypedDict):
    trade_count: int | Decimal
    volume: int | Decimal
    volume_usdc: int | Decimal

_DataGetValueParamsOptional = TypedDict("_DataGetValueParamsOptional", {'user': 'str | None', 'condition': 'str | None'}, total=False)
class DataGetValueParams(_DataGetValueParamsOptional):
    pass

class DataGetValueResponse(TypedDict):
    data: DataGetValueResponseData

class DataGetValueResponseData(TypedDict):
    proxy_wallet: str
    value: int | Decimal

_DataHolderOptional = TypedDict("_DataHolderOptional", {'avg_price': 'int | Decimal | None', 'current_price': 'int | Decimal | None', 'current_value': 'int | Decimal | None', 'entry_cost_usdc': 'int | Decimal | None', 'realized_pnl': 'int | Decimal | None', 'total_pnl': 'int | Decimal | None', 'unrealized_pnl': 'int | Decimal | None'}, total=False)
class DataHolder(_DataHolderOptional):
    amount: int | Decimal
    bio: str
    display_username_public: bool
    name: str
    outcome_index: int | Decimal
    profile_image: str
    profile_image_optimized: str
    proxy_wallet: str
    pseudonym: str
    token_id: str
    verified: bool

class DataHoldersPage(TypedDict):
    data: list[DataMetaHolder]
    pagination: DataPagination

_DataIngestionFreshnessOptional = TypedDict("_DataIngestionFreshnessOptional", {'max_synced_block': 'int | Decimal | None', 'min_synced_block': 'int | Decimal | None', 'most_lagged': 'Any | None', 'network': 'str | None'}, total=False)
class DataIngestionFreshness(_DataIngestionFreshnessOptional):
    chain_id: int | Decimal
    cursors: int | Decimal
    lagging: list[DataCursorLag]

class DataLeaderboardEntry(TypedDict):
    pnl: int | Decimal
    profile_image: str
    rank: int | Decimal
    user_id: str
    user_name: str
    verified: bool
    volume: int | Decimal
    x_username: str

class DataLeaderboardPage(TypedDict):
    data: list[DataLeaderboardEntry]
    pagination: DataPagination

_DataLeaderboardUserEntryOptional = TypedDict("_DataLeaderboardUserEntryOptional", {'rank_pnl': 'int | Decimal | None', 'rank_volume': 'int | Decimal | None'}, total=False)
class DataLeaderboardUserEntry(_DataLeaderboardUserEntryOptional):
    pnl: int | Decimal
    profile_image: str
    user_id: str
    user_name: str
    verified: bool
    volume: int | Decimal
    x_username: str

class DataLiveVolume(TypedDict):
    conditions: list[DataConditionVolume]
    taker_volume_total: int | Decimal

class DataMetaHolder(TypedDict):
    holders: list[DataHolder]
    token_id: str

class DataOpenInterest(TypedDict):
    condition_id: str
    value: int | Decimal

_DataPaginationOptional = TypedDict("_DataPaginationOptional", {'next_cursor': 'str | None'}, total=False)
class DataPagination(_DataPaginationOptional):
    has_more: bool
    limit: int | Decimal
    offset: int | Decimal

class DataPortfolioValue(TypedDict):
    proxy_wallet: str
    value: int | Decimal

class DataPosition(TypedDict):
    archived: bool
    avg_price: int | Decimal
    condition_id: str
    current_price: int | Decimal
    current_size: int | Decimal
    current_value: int | Decimal
    end_date: str
    entry_cost_usdc: int | Decimal
    entry_fees_usdc: int | Decimal
    event_id: str
    event_slug: str
    first_entry_at: int | Decimal
    icon: str
    last_event_at: int | Decimal
    mergeable: bool
    name: str
    negative_risk: bool
    opposite_outcome: str
    opposite_token_id: str
    outcome: str
    outcome_index: int | Decimal
    percent_pnl: int | Decimal
    percent_realized_pnl: int | Decimal
    profile_image: str
    proxy_wallet: str
    realized_pnl: int | Decimal
    redeemable: bool
    slug: str
    status: str
    title: str
    token_id: str
    total_cost_usdc: int | Decimal
    total_pnl: int | Decimal
    total_size: int | Decimal
    unrealized_pnl: int | Decimal
    verified: bool

class DataPositionsPage(TypedDict):
    data: list[DataPosition]
    pagination: DataPagination

class DataPricePoint(TypedDict):
    price: int | Decimal
    resolution_seconds: int | Decimal
    timestamp: int | Decimal

class DataPricesHistoryPage(TypedDict):
    data: list[DataPricePoint]
    pagination: DataPagination

_DataResolutionOptional = TypedDict("_DataResolutionOptional", {'condition_id': 'str | None', 'expected_settlement_time': 'str | None', 'market_type': 'str | None', 'payouts': 'list[int | Decimal] | None', 'price': 'str | None', 'proposed_price': 'str | None', 'question_id': 'str | None', 'reporter': 'str | None', 'reproposed_price': 'str | None', 'resolution_source': 'str | None', 'resolved_at': 'str | None', 'resolved_block': 'int | Decimal | None', 'settlement_time_basis': 'str | None', 'was_arbitrated': 'bool | None'}, total=False)
class DataResolution(_DataResolutionOptional):
    extended_review: bool
    last_update_timestamp: str
    log_index: str
    new_version_q: bool
    status: str
    transaction_hash: str
    was_disputed: bool

class DataServiceStatus(TypedDict):
    age_seconds: int | Decimal
    computed_at: str
    ingestion: DataIngestionFreshness
    serving: DataServingFreshness

_DataServingFreshnessOptional = TypedDict("_DataServingFreshnessOptional", {'lag_seconds': 'int | Decimal | None', 'worst': 'str | None'}, total=False)
class DataServingFreshness(_DataServingFreshnessOptional):
    mechanisms: list[DataServingMechanism]

_DataServingMechanismOptional = TypedDict("_DataServingMechanismOptional", {'blocks_behind': 'int | Decimal | None'}, total=False)
class DataServingMechanism(_DataServingMechanismOptional):
    age_seconds: int | Decimal
    name: str

_DataTokenRefOptional = TypedDict("_DataTokenRefOptional", {'clob_index': 'int | Decimal | None', 'closed': 'bool | None', 'event_id': 'int | Decimal | None', 'event_slug': 'str | None', 'final_price': 'int | Decimal | None', 'market_slug': 'str | None', 'neg_risk': 'bool | None', 'neg_risk_market_id': 'str | None', 'opposite_token_id': 'str | None', 'outcome': 'str | None', 'question_index': 'int | Decimal | None', 'structural_condition_id': 'str | None', 'title': 'str | None'}, total=False)
class DataTokenRef(_DataTokenRefOptional):
    condition_id: str
    module: str
    outcome_index: int | Decimal
    resolved: bool
    token_id: str

class DataTrade(TypedDict):
    bio: str
    condition_id: str
    event_slug: str
    icon: str
    name: str
    outcome: str
    outcome_index: int | Decimal
    price: int | Decimal
    profile_image: str
    profile_image_optimized: str
    proxy_wallet: str
    pseudonym: str
    side: str
    size: int | Decimal
    slug: str
    timestamp: int | Decimal
    title: str
    token_id: str
    transaction_hash: str

class DataTradesPage(TypedDict):
    data: list[DataTrade]
    pagination: DataPagination

_DataUserPnlPointOptional = TypedDict("_DataUserPnlPointOptional", {'cashflow_net': 'int | Decimal | None', 'deposits': 'int | Decimal | None', 'economic_pnl': 'int | Decimal | None', 'fees': 'int | Decimal | None', 'fees_paid': 'int | Decimal | None', 'fees_refunded': 'int | Decimal | None', 'maker_rebate': 'int | Decimal | None', 'position_pnl': 'int | Decimal | None', 'referral_income': 'int | Decimal | None', 'reward_income': 'int | Decimal | None', 'settled_pnl': 'int | Decimal | None', 'sponsored_income': 'int | Decimal | None', 'taker_rebate': 'int | Decimal | None', 'trade_pnl': 'int | Decimal | None', 'unrealized_pnl': 'int | Decimal | None', 'wallet_income': 'int | Decimal | None', 'withdrawals': 'int | Decimal | None', 'yield_income': 'int | Decimal | None'}, total=False)
class DataUserPnlPoint(_DataUserPnlPointOptional):
    realized_combo_pnl: int | Decimal
    realized_lp_pnl: int | Decimal
    realized_market_pnl: int | Decimal
    realized_pnl: int | Decimal
    source_block: int | Decimal
    timestamp: int | Decimal
    trade_count: int | Decimal
    volume: int | Decimal
    volume_usdc: int | Decimal

class DataUserPnlSeries(TypedDict):
    fidelity: str
    interval: str
    points: list[DataUserPnlPoint]
    proxy_wallet: str
    source_fidelity: str

_DataUserStatsOptional = TypedDict("_DataUserStatsOptional", {'all_time_pnl': 'Any | None', 'join_date': 'int | Decimal | None'}, total=False)
class DataUserStats(_DataUserStatsOptional):
    biggest_win: int | Decimal
    proxy_wallet: str
    trades: int | Decimal
    views: int | Decimal

class DataUserVolume(TypedDict):
    trade_count: int | Decimal
    volume: int | Decimal
    volume_usdc: int | Decimal

_GammaCategoryOptional = TypedDict("_GammaCategoryOptional", {'id': 'str | None', 'label': 'str | None', 'parentCategory': 'str | None', 'slug': 'str | None', 'publishedAt': 'str | None', 'createdBy': 'str | None', 'updatedBy': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None'}, total=False)
class GammaCategory(_GammaCategoryOptional):
    pass

_GammaChatOptional = TypedDict("_GammaChatOptional", {'id': 'str | None', 'channelId': 'str | None', 'channelName': 'str | None', 'channelImage': 'str | None', 'live': 'bool | None', 'startTime': 'str | None', 'endTime': 'str | None'}, total=False)
class GammaChat(_GammaChatOptional):
    pass

_GammaCollectionOptional = TypedDict("_GammaCollectionOptional", {'id': 'str | None', 'ticker': 'str | None', 'slug': 'str | None', 'title': 'str | None', 'subtitle': 'str | None', 'collectionType': 'str | None', 'description': 'str | None', 'tags': 'str | None', 'image': 'str | None', 'icon': 'str | None', 'headerImage': 'str | None', 'layout': 'str | None', 'active': 'bool | None', 'closed': 'bool | None', 'archived': 'bool | None', 'new': 'bool | None', 'featured': 'bool | None', 'restricted': 'bool | None', 'isTemplate': 'bool | None', 'templateVariables': 'str | None', 'publishedAt': 'str | None', 'createdBy': 'str | None', 'updatedBy': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'commentsEnabled': 'bool | None', 'imageOptimized': 'GammaImageOptimization | None', 'iconOptimized': 'GammaImageOptimization | None', 'headerImageOptimized': 'GammaImageOptimization | None'}, total=False)
class GammaCollection(_GammaCollectionOptional):
    pass

_GammaCommentOptional = TypedDict("_GammaCommentOptional", {'id': 'str | None', 'body': 'str | None', 'parentEntityType': 'str | None', 'parentEntityID': 'int | Decimal | None', 'parentCommentID': 'str | None', 'userAddress': 'str | None', 'replyAddress': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'profile': 'GammaCommentProfile | None', 'reactions': 'list[GammaReaction] | None', 'reportCount': 'int | Decimal | None', 'reactionCount': 'int | Decimal | None'}, total=False)
class GammaComment(_GammaCommentOptional):
    pass

_GammaCommentPositionOptional = TypedDict("_GammaCommentPositionOptional", {'tokenId': 'str | None', 'positionSize': 'str | None'}, total=False)
class GammaCommentPosition(_GammaCommentPositionOptional):
    pass

_GammaCommentProfileOptional = TypedDict("_GammaCommentProfileOptional", {'name': 'str | None', 'pseudonym': 'str | None', 'displayUsernamePublic': 'bool | None', 'bio': 'str | None', 'isMod': 'bool | None', 'isCreator': 'bool | None', 'proxyWallet': 'str | None', 'baseAddress': 'str | None', 'profileImage': 'str | None', 'profileImageOptimized': 'GammaImageOptimization | None', 'positions': 'list[GammaCommentPosition] | None'}, total=False)
class GammaCommentProfile(_GammaCommentProfileOptional):
    pass

_GammaCountOptional = TypedDict("_GammaCountOptional", {'count': 'int | Decimal | None'}, total=False)
class GammaCount(_GammaCountOptional):
    pass

_GammaEventOptional = TypedDict("_GammaEventOptional", {'id': 'str | None', 'ticker': 'str | None', 'slug': 'str | None', 'title': 'str | None', 'subtitle': 'str | None', 'description': 'str | None', 'resolutionSource': 'str | None', 'startDate': 'str | None', 'creationDate': 'str | None', 'endDate': 'str | None', 'image': 'str | None', 'icon': 'str | None', 'active': 'bool | None', 'closed': 'bool | None', 'archived': 'bool | None', 'new': 'bool | None', 'featured': 'bool | None', 'restricted': 'bool | None', 'liquidity': 'int | Decimal | None', 'volume': 'int | Decimal | None', 'openInterest': 'int | Decimal | None', 'sortBy': 'str | None', 'category': 'str | None', 'subcategory': 'str | None', 'isTemplate': 'bool | None', 'templateVariables': 'str | None', 'published_at': 'str | None', 'createdBy': 'str | None', 'updatedBy': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'commentsEnabled': 'bool | None', 'competitive': 'int | Decimal | None', 'volume24hr': 'int | Decimal | None', 'volume1wk': 'int | Decimal | None', 'volume1mo': 'int | Decimal | None', 'volume1yr': 'int | Decimal | None', 'featuredImage': 'str | None', 'disqusThread': 'str | None', 'parentEvent': 'str | None', 'enableOrderBook': 'bool | None', 'liquidityClob': 'int | Decimal | None', 'negRisk': 'bool | None', 'negRiskMarketID': 'str | None', 'negRiskFeeBips': 'int | Decimal | None', 'commentCount': 'int | Decimal | None', 'imageOptimized': 'GammaImageOptimization | None', 'iconOptimized': 'GammaImageOptimization | None', 'featuredImageOptimized': 'GammaImageOptimization | None', 'subEvents': 'list[str] | None', 'markets': 'list[GammaMarket] | None', 'series': 'list[GammaSeries] | None', 'categories': 'list[GammaCategory] | None', 'collections': 'list[GammaCollection] | None', 'tags': 'list[GammaTag] | None', 'cyom': 'bool | None', 'closedTime': 'str | None', 'showAllOutcomes': 'bool | None', 'showMarketImages': 'bool | None', 'automaticallyResolved': 'bool | None', 'enableNegRisk': 'bool | None', 'automaticallyActive': 'bool | None', 'eventDate': 'str | None', 'startTime': 'str | None', 'eventWeek': 'int | Decimal | None', 'seriesSlug': 'str | None', 'score': 'str | None', 'elapsed': 'str | None', 'period': 'str | None', 'live': 'bool | None', 'ended': 'bool | None', 'finishedTimestamp': 'str | None', 'gmpChartMode': 'str | None', 'eventCreators': 'list[GammaEventCreator] | None', 'tweetCount': 'int | Decimal | None', 'chats': 'list[GammaChat] | None', 'featuredOrder': 'int | Decimal | None', 'estimateValue': 'bool | None', 'cantEstimate': 'bool | None', 'estimatedValue': 'str | None', 'templates': 'list[GammaTemplate] | None', 'spreadsMainLine': 'int | Decimal | None', 'totalsMainLine': 'int | Decimal | None', 'carouselMap': 'str | None', 'pendingDeployment': 'bool | None', 'deploying': 'bool | None', 'deployingTimestamp': 'str | None', 'scheduledDeploymentTimestamp': 'str | None', 'gameStatus': 'str | None'}, total=False)
class GammaEvent(_GammaEventOptional):
    pass

_GammaEventCreatorOptional = TypedDict("_GammaEventCreatorOptional", {'id': 'str | None', 'creatorName': 'str | None', 'creatorHandle': 'str | None', 'creatorUrl': 'str | None', 'creatorImage': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None'}, total=False)
class GammaEventCreator(_GammaEventCreatorOptional):
    pass

_GammaEventTweetCountOptional = TypedDict("_GammaEventTweetCountOptional", {'tweetCount': 'int | Decimal | None'}, total=False)
class GammaEventTweetCount(_GammaEventTweetCountOptional):
    pass

_GammaEventsPaginationOptional = TypedDict("_GammaEventsPaginationOptional", {'data': 'list[GammaEvent] | None', 'pagination': 'GammaPagination | None'}, total=False)
class GammaEventsPagination(_GammaEventsPaginationOptional):
    pass

_GammaFeeScheduleOptional = TypedDict("_GammaFeeScheduleOptional", {'exponent': 'int | Decimal | None', 'rate': 'int | Decimal | None', 'takerOnly': 'bool | None', 'rebateRate': 'int | Decimal | None'}, total=False)
class GammaFeeSchedule(_GammaFeeScheduleOptional):
    pass

class GammaGetAbridgedMarketsParams(TypedDict):
    body: GammaMarketsInformationBody

_GammaGetCommentsByIdParamsOptional = TypedDict("_GammaGetCommentsByIdParamsOptional", {'get_positions': 'bool | None'}, total=False)
class GammaGetCommentsByIdParams(_GammaGetCommentsByIdParamsOptional):
    id: int | Decimal

_GammaGetCommentsByUserAddressParamsOptional = TypedDict("_GammaGetCommentsByUserAddressParamsOptional", {'limit': 'int | Decimal | None', 'offset': 'int | Decimal | None', 'order': 'str | None', 'ascending': 'bool | None'}, total=False)
class GammaGetCommentsByUserAddressParams(_GammaGetCommentsByUserAddressParamsOptional):
    user_address: str

_GammaGetEventBySlugParamsOptional = TypedDict("_GammaGetEventBySlugParamsOptional", {'include_chat': 'bool | None', 'include_template': 'bool | None'}, total=False)
class GammaGetEventBySlugParams(_GammaGetEventBySlugParamsOptional):
    slug: str

_GammaGetEventBySlugResponseOptional = TypedDict("_GammaGetEventBySlugResponseOptional", {'id': 'str | None', 'ticker': 'str | None', 'slug': 'str | None', 'title': 'str | None', 'subtitle': 'str | None', 'description': 'str | None', 'resolutionSource': 'str | None', 'startDate': 'str | None', 'creationDate': 'str | None', 'endDate': 'str | None', 'image': 'str | None', 'icon': 'str | None', 'active': 'bool | None', 'closed': 'bool | None', 'archived': 'bool | None', 'new': 'bool | None', 'featured': 'bool | None', 'restricted': 'bool | None', 'liquidity': 'int | Decimal | None', 'volume': 'int | Decimal | None', 'openInterest': 'int | Decimal | None', 'sortBy': 'str | None', 'category': 'str | None', 'subcategory': 'str | None', 'isTemplate': 'bool | None', 'templateVariables': 'str | None', 'published_at': 'str | None', 'createdBy': 'str | None', 'updatedBy': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'commentsEnabled': 'bool | None', 'competitive': 'int | Decimal | None', 'volume24hr': 'int | Decimal | None', 'volume1wk': 'int | Decimal | None', 'volume1mo': 'int | Decimal | None', 'volume1yr': 'int | Decimal | None', 'featuredImage': 'str | None', 'disqusThread': 'str | None', 'parentEvent': 'str | None', 'enableOrderBook': 'bool | None', 'liquidityClob': 'int | Decimal | None', 'negRisk': 'bool | None', 'negRiskMarketID': 'str | None', 'negRiskFeeBips': 'int | Decimal | None', 'commentCount': 'int | Decimal | None', 'imageOptimized': 'GammaImageOptimization | None', 'iconOptimized': 'GammaImageOptimization | None', 'featuredImageOptimized': 'GammaImageOptimization | None', 'subEvents': 'list[str] | None', 'markets': 'list[GammaMarket] | None', 'series': 'list[GammaSeries] | None', 'categories': 'list[GammaCategory] | None', 'collections': 'list[GammaCollection] | None', 'tags': 'list[GammaTag] | None', 'cyom': 'bool | None', 'closedTime': 'str | None', 'showAllOutcomes': 'bool | None', 'showMarketImages': 'bool | None', 'automaticallyResolved': 'bool | None', 'enableNegRisk': 'bool | None', 'automaticallyActive': 'bool | None', 'eventDate': 'str | None', 'startTime': 'str | None', 'eventWeek': 'int | Decimal | None', 'seriesSlug': 'str | None', 'score': 'str | None', 'elapsed': 'str | None', 'period': 'str | None', 'live': 'bool | None', 'ended': 'bool | None', 'finishedTimestamp': 'str | None', 'gmpChartMode': 'str | None', 'eventCreators': 'list[GammaEventCreator] | None', 'tweetCount': 'int | Decimal | None', 'chats': 'list[GammaChat] | None', 'featuredOrder': 'int | Decimal | None', 'estimateValue': 'bool | None', 'cantEstimate': 'bool | None', 'estimatedValue': 'str | None', 'templates': 'list[GammaTemplate] | None', 'spreadsMainLine': 'int | Decimal | None', 'totalsMainLine': 'int | Decimal | None', 'carouselMap': 'str | None', 'pendingDeployment': 'bool | None', 'deploying': 'bool | None', 'deployingTimestamp': 'str | None', 'scheduledDeploymentTimestamp': 'str | None', 'gameStatus': 'str | None'}, total=False)
class GammaGetEventBySlugResponse(_GammaGetEventBySlugResponseOptional):
    pass

class GammaGetEventCommentsCountParams(TypedDict):
    id: int | Decimal

_GammaGetEventCommentsCountResponseOptional = TypedDict("_GammaGetEventCommentsCountResponseOptional", {'count': 'int | Decimal | None'}, total=False)
class GammaGetEventCommentsCountResponse(_GammaGetEventCommentsCountResponseOptional):
    pass

class GammaGetEventCreatorParams(TypedDict):
    id: int | Decimal

_GammaGetEventCreatorResponseOptional = TypedDict("_GammaGetEventCreatorResponseOptional", {'id': 'str | None', 'creatorName': 'str | None', 'creatorHandle': 'str | None', 'creatorUrl': 'str | None', 'creatorImage': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None'}, total=False)
class GammaGetEventCreatorResponse(_GammaGetEventCreatorResponseOptional):
    pass

_GammaGetEventParamsOptional = TypedDict("_GammaGetEventParamsOptional", {'include_chat': 'bool | None', 'include_template': 'bool | None'}, total=False)
class GammaGetEventParams(_GammaGetEventParamsOptional):
    id: int | Decimal

_GammaGetEventResponseOptional = TypedDict("_GammaGetEventResponseOptional", {'id': 'str | None', 'ticker': 'str | None', 'slug': 'str | None', 'title': 'str | None', 'subtitle': 'str | None', 'description': 'str | None', 'resolutionSource': 'str | None', 'startDate': 'str | None', 'creationDate': 'str | None', 'endDate': 'str | None', 'image': 'str | None', 'icon': 'str | None', 'active': 'bool | None', 'closed': 'bool | None', 'archived': 'bool | None', 'new': 'bool | None', 'featured': 'bool | None', 'restricted': 'bool | None', 'liquidity': 'int | Decimal | None', 'volume': 'int | Decimal | None', 'openInterest': 'int | Decimal | None', 'sortBy': 'str | None', 'category': 'str | None', 'subcategory': 'str | None', 'isTemplate': 'bool | None', 'templateVariables': 'str | None', 'published_at': 'str | None', 'createdBy': 'str | None', 'updatedBy': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'commentsEnabled': 'bool | None', 'competitive': 'int | Decimal | None', 'volume24hr': 'int | Decimal | None', 'volume1wk': 'int | Decimal | None', 'volume1mo': 'int | Decimal | None', 'volume1yr': 'int | Decimal | None', 'featuredImage': 'str | None', 'disqusThread': 'str | None', 'parentEvent': 'str | None', 'enableOrderBook': 'bool | None', 'liquidityClob': 'int | Decimal | None', 'negRisk': 'bool | None', 'negRiskMarketID': 'str | None', 'negRiskFeeBips': 'int | Decimal | None', 'commentCount': 'int | Decimal | None', 'imageOptimized': 'GammaImageOptimization | None', 'iconOptimized': 'GammaImageOptimization | None', 'featuredImageOptimized': 'GammaImageOptimization | None', 'subEvents': 'list[str] | None', 'markets': 'list[GammaMarket] | None', 'series': 'list[GammaSeries] | None', 'categories': 'list[GammaCategory] | None', 'collections': 'list[GammaCollection] | None', 'tags': 'list[GammaTag] | None', 'cyom': 'bool | None', 'closedTime': 'str | None', 'showAllOutcomes': 'bool | None', 'showMarketImages': 'bool | None', 'automaticallyResolved': 'bool | None', 'enableNegRisk': 'bool | None', 'automaticallyActive': 'bool | None', 'eventDate': 'str | None', 'startTime': 'str | None', 'eventWeek': 'int | Decimal | None', 'seriesSlug': 'str | None', 'score': 'str | None', 'elapsed': 'str | None', 'period': 'str | None', 'live': 'bool | None', 'ended': 'bool | None', 'finishedTimestamp': 'str | None', 'gmpChartMode': 'str | None', 'eventCreators': 'list[GammaEventCreator] | None', 'tweetCount': 'int | Decimal | None', 'chats': 'list[GammaChat] | None', 'featuredOrder': 'int | Decimal | None', 'estimateValue': 'bool | None', 'cantEstimate': 'bool | None', 'estimatedValue': 'str | None', 'templates': 'list[GammaTemplate] | None', 'spreadsMainLine': 'int | Decimal | None', 'totalsMainLine': 'int | Decimal | None', 'carouselMap': 'str | None', 'pendingDeployment': 'bool | None', 'deploying': 'bool | None', 'deployingTimestamp': 'str | None', 'scheduledDeploymentTimestamp': 'str | None', 'gameStatus': 'str | None'}, total=False)
class GammaGetEventResponse(_GammaGetEventResponseOptional):
    pass

class GammaGetEventTagsParams(TypedDict):
    id: int | Decimal

class GammaGetEventTweetCountParams(TypedDict):
    id: int | Decimal

_GammaGetEventTweetCountResponseOptional = TypedDict("_GammaGetEventTweetCountResponseOptional", {'tweetCount': 'int | Decimal | None'}, total=False)
class GammaGetEventTweetCountResponse(_GammaGetEventTweetCountResponseOptional):
    pass

class GammaGetGammaStatusParams(TypedDict):
    pass

_GammaGetMarketBySlugParamsOptional = TypedDict("_GammaGetMarketBySlugParamsOptional", {'include_tag': 'bool | None'}, total=False)
class GammaGetMarketBySlugParams(_GammaGetMarketBySlugParamsOptional):
    slug: str

_GammaGetMarketBySlugResponseOptional = TypedDict("_GammaGetMarketBySlugResponseOptional", {'id': 'str | None', 'question': 'str | None', 'conditionId': 'str | None', 'slug': 'str | None', 'twitterCardImage': 'str | None', 'resolutionSource': 'str | None', 'endDate': 'str | None', 'category': 'str | None', 'liquidity': 'str | None', 'sponsorName': 'str | None', 'sponsorImage': 'str | None', 'startDate': 'str | None', 'xAxisValue': 'str | None', 'yAxisValue': 'str | None', 'denominationToken': 'str | None', 'fee': 'str | None', 'image': 'str | None', 'icon': 'str | None', 'lowerBound': 'str | None', 'upperBound': 'str | None', 'description': 'str | None', 'outcomes': 'str | None', 'outcomePrices': 'str | None', 'volume': 'str | None', 'active': 'bool | None', 'marketType': 'str | None', 'formatType': 'str | None', 'lowerBoundDate': 'str | None', 'upperBoundDate': 'str | None', 'closed': 'bool | None', 'createdBy': 'int | Decimal | None', 'updatedBy': 'int | Decimal | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'closedTime': 'str | None', 'wideFormat': 'bool | None', 'new': 'bool | None', 'mailchimpTag': 'str | None', 'featured': 'bool | None', 'archived': 'bool | None', 'resolvedBy': 'str | None', 'restricted': 'bool | None', 'marketGroup': 'int | Decimal | None', 'groupItemTitle': 'str | None', 'groupItemThreshold': 'str | None', 'questionID': 'str | None', 'umaEndDate': 'str | None', 'enableOrderBook': 'bool | None', 'orderPriceMinTickSize': 'int | Decimal | None', 'orderMinSize': 'int | Decimal | None', 'umaResolutionStatus': 'str | None', 'curationOrder': 'int | Decimal | None', 'volumeNum': 'int | Decimal | None', 'liquidityNum': 'int | Decimal | None', 'endDateIso': 'str | None', 'startDateIso': 'str | None', 'umaEndDateIso': 'str | None', 'hasReviewedDates': 'bool | None', 'readyForCron': 'bool | None', 'commentsEnabled': 'bool | None', 'volume24hr': 'int | Decimal | None', 'volume1wk': 'int | Decimal | None', 'volume1mo': 'int | Decimal | None', 'volume1yr': 'int | Decimal | None', 'gameStartTime': 'str | None', 'secondsDelay': 'int | Decimal | None', 'version': 'str | None', 'clobTokenIds': 'str | None', 'positionIds': 'list[str] | None', 'disqusThread': 'str | None', 'shortOutcomes': 'str | None', 'teamAID': 'str | None', 'teamBID': 'str | None', 'umaBond': 'str | None', 'umaReward': 'str | None', 'volume24hrClob': 'int | Decimal | None', 'volume1wkClob': 'int | Decimal | None', 'volume1moClob': 'int | Decimal | None', 'volume1yrClob': 'int | Decimal | None', 'volumeClob': 'int | Decimal | None', 'liquidityClob': 'int | Decimal | None', 'makerBaseFee': 'int | Decimal | None', 'takerBaseFee': 'int | Decimal | None', 'customLiveness': 'int | Decimal | None', 'acceptingOrders': 'bool | None', 'notificationsEnabled': 'bool | None', 'score': 'int | Decimal | None', 'imageOptimized': 'GammaImageOptimization | None', 'iconOptimized': 'GammaImageOptimization | None', 'events': 'list[GammaEvent] | None', 'categories': 'list[GammaCategory] | None', 'tags': 'list[GammaTag] | None', 'creator': 'str | None', 'ready': 'bool | None', 'funded': 'bool | None', 'pastSlugs': 'str | None', 'readyTimestamp': 'str | None', 'fundedTimestamp': 'str | None', 'acceptingOrdersTimestamp': 'str | None', 'competitive': 'int | Decimal | None', 'rewardsMinSize': 'int | Decimal | None', 'rewardsMaxSpread': 'int | Decimal | None', 'spread': 'int | Decimal | None', 'automaticallyResolved': 'bool | None', 'oneDayPriceChange': 'int | Decimal | None', 'oneHourPriceChange': 'int | Decimal | None', 'oneWeekPriceChange': 'int | Decimal | None', 'oneMonthPriceChange': 'int | Decimal | None', 'oneYearPriceChange': 'int | Decimal | None', 'lastTradePrice': 'int | Decimal | None', 'bestBid': 'int | Decimal | None', 'bestAsk': 'int | Decimal | None', 'automaticallyActive': 'bool | None', 'clearBookOnStart': 'bool | None', 'chartColor': 'str | None', 'seriesColor': 'str | None', 'showGmpSeries': 'bool | None', 'showGmpOutcome': 'bool | None', 'manualActivation': 'bool | None', 'negRiskOther': 'bool | None', 'gameId': 'str | None', 'groupItemRange': 'str | None', 'sportsMarketType': 'str | None', 'line': 'int | Decimal | None', 'umaResolutionStatuses': 'str | None', 'pendingDeployment': 'bool | None', 'deploying': 'bool | None', 'deployingTimestamp': 'str | None', 'scheduledDeploymentTimestamp': 'str | None', 'rfqEnabled': 'bool | None', 'eventStartTime': 'str | None', 'feesEnabled': 'bool | None', 'feeSchedule': 'GammaFeeSchedule | None'}, total=False)
class GammaGetMarketBySlugResponse(_GammaGetMarketBySlugResponseOptional):
    pass

class GammaGetMarketDescriptionParams(TypedDict):
    id: int | Decimal

_GammaGetMarketDescriptionResponseOptional = TypedDict("_GammaGetMarketDescriptionResponseOptional", {'description': 'str | None'}, total=False)
class GammaGetMarketDescriptionResponse(_GammaGetMarketDescriptionResponseOptional):
    pass

_GammaGetMarketParamsOptional = TypedDict("_GammaGetMarketParamsOptional", {'include_tag': 'bool | None'}, total=False)
class GammaGetMarketParams(_GammaGetMarketParamsOptional):
    id: int | Decimal

_GammaGetMarketResponseOptional = TypedDict("_GammaGetMarketResponseOptional", {'id': 'str | None', 'question': 'str | None', 'conditionId': 'str | None', 'slug': 'str | None', 'twitterCardImage': 'str | None', 'resolutionSource': 'str | None', 'endDate': 'str | None', 'category': 'str | None', 'liquidity': 'str | None', 'sponsorName': 'str | None', 'sponsorImage': 'str | None', 'startDate': 'str | None', 'xAxisValue': 'str | None', 'yAxisValue': 'str | None', 'denominationToken': 'str | None', 'fee': 'str | None', 'image': 'str | None', 'icon': 'str | None', 'lowerBound': 'str | None', 'upperBound': 'str | None', 'description': 'str | None', 'outcomes': 'str | None', 'outcomePrices': 'str | None', 'volume': 'str | None', 'active': 'bool | None', 'marketType': 'str | None', 'formatType': 'str | None', 'lowerBoundDate': 'str | None', 'upperBoundDate': 'str | None', 'closed': 'bool | None', 'createdBy': 'int | Decimal | None', 'updatedBy': 'int | Decimal | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'closedTime': 'str | None', 'wideFormat': 'bool | None', 'new': 'bool | None', 'mailchimpTag': 'str | None', 'featured': 'bool | None', 'archived': 'bool | None', 'resolvedBy': 'str | None', 'restricted': 'bool | None', 'marketGroup': 'int | Decimal | None', 'groupItemTitle': 'str | None', 'groupItemThreshold': 'str | None', 'questionID': 'str | None', 'umaEndDate': 'str | None', 'enableOrderBook': 'bool | None', 'orderPriceMinTickSize': 'int | Decimal | None', 'orderMinSize': 'int | Decimal | None', 'umaResolutionStatus': 'str | None', 'curationOrder': 'int | Decimal | None', 'volumeNum': 'int | Decimal | None', 'liquidityNum': 'int | Decimal | None', 'endDateIso': 'str | None', 'startDateIso': 'str | None', 'umaEndDateIso': 'str | None', 'hasReviewedDates': 'bool | None', 'readyForCron': 'bool | None', 'commentsEnabled': 'bool | None', 'volume24hr': 'int | Decimal | None', 'volume1wk': 'int | Decimal | None', 'volume1mo': 'int | Decimal | None', 'volume1yr': 'int | Decimal | None', 'gameStartTime': 'str | None', 'secondsDelay': 'int | Decimal | None', 'version': 'str | None', 'clobTokenIds': 'str | None', 'positionIds': 'list[str] | None', 'disqusThread': 'str | None', 'shortOutcomes': 'str | None', 'teamAID': 'str | None', 'teamBID': 'str | None', 'umaBond': 'str | None', 'umaReward': 'str | None', 'volume24hrClob': 'int | Decimal | None', 'volume1wkClob': 'int | Decimal | None', 'volume1moClob': 'int | Decimal | None', 'volume1yrClob': 'int | Decimal | None', 'volumeClob': 'int | Decimal | None', 'liquidityClob': 'int | Decimal | None', 'makerBaseFee': 'int | Decimal | None', 'takerBaseFee': 'int | Decimal | None', 'customLiveness': 'int | Decimal | None', 'acceptingOrders': 'bool | None', 'notificationsEnabled': 'bool | None', 'score': 'int | Decimal | None', 'imageOptimized': 'GammaImageOptimization | None', 'iconOptimized': 'GammaImageOptimization | None', 'events': 'list[GammaEvent] | None', 'categories': 'list[GammaCategory] | None', 'tags': 'list[GammaTag] | None', 'creator': 'str | None', 'ready': 'bool | None', 'funded': 'bool | None', 'pastSlugs': 'str | None', 'readyTimestamp': 'str | None', 'fundedTimestamp': 'str | None', 'acceptingOrdersTimestamp': 'str | None', 'competitive': 'int | Decimal | None', 'rewardsMinSize': 'int | Decimal | None', 'rewardsMaxSpread': 'int | Decimal | None', 'spread': 'int | Decimal | None', 'automaticallyResolved': 'bool | None', 'oneDayPriceChange': 'int | Decimal | None', 'oneHourPriceChange': 'int | Decimal | None', 'oneWeekPriceChange': 'int | Decimal | None', 'oneMonthPriceChange': 'int | Decimal | None', 'oneYearPriceChange': 'int | Decimal | None', 'lastTradePrice': 'int | Decimal | None', 'bestBid': 'int | Decimal | None', 'bestAsk': 'int | Decimal | None', 'automaticallyActive': 'bool | None', 'clearBookOnStart': 'bool | None', 'chartColor': 'str | None', 'seriesColor': 'str | None', 'showGmpSeries': 'bool | None', 'showGmpOutcome': 'bool | None', 'manualActivation': 'bool | None', 'negRiskOther': 'bool | None', 'gameId': 'str | None', 'groupItemRange': 'str | None', 'sportsMarketType': 'str | None', 'line': 'int | Decimal | None', 'umaResolutionStatuses': 'str | None', 'pendingDeployment': 'bool | None', 'deploying': 'bool | None', 'deployingTimestamp': 'str | None', 'scheduledDeploymentTimestamp': 'str | None', 'rfqEnabled': 'bool | None', 'eventStartTime': 'str | None', 'feesEnabled': 'bool | None', 'feeSchedule': 'GammaFeeSchedule | None'}, total=False)
class GammaGetMarketResponse(_GammaGetMarketResponseOptional):
    pass

class GammaGetMarketTagsParams(TypedDict):
    id: int | Decimal

class GammaGetMarketsInformationParams(TypedDict):
    body: GammaMarketsInformationBody

class GammaGetPublicProfileByUserAddressParams(TypedDict):
    user_address: str

_GammaGetPublicProfileByUserAddressResponseOptional = TypedDict("_GammaGetPublicProfileByUserAddressResponseOptional", {'id': 'str | None', 'name': 'str | None', 'user': 'int | Decimal | None', 'referral': 'str | None', 'createdBy': 'int | Decimal | None', 'updatedBy': 'int | Decimal | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'utmSource': 'str | None', 'utmMedium': 'str | None', 'utmCampaign': 'str | None', 'utmContent': 'str | None', 'utmTerm': 'str | None', 'walletActivated': 'bool | None', 'pseudonym': 'str | None', 'displayUsernamePublic': 'bool | None', 'profileImage': 'str | None', 'bio': 'str | None', 'proxyWallet': 'str | None', 'profileImageOptimized': 'GammaImageOptimization | None', 'isCloseOnly': 'bool | None', 'isCertReq': 'bool | None', 'certReqDate': 'str | None'}, total=False)
class GammaGetPublicProfileByUserAddressResponse(_GammaGetPublicProfileByUserAddressResponseOptional):
    pass

class GammaGetPublicProfileParams(TypedDict):
    address: str

_GammaGetPublicProfileResponseOptional = TypedDict("_GammaGetPublicProfileResponseOptional", {'createdAt': 'str | None', 'proxyWallet': 'str | None', 'profileImage': 'str | None', 'displayUsernamePublic': 'bool | None', 'bio': 'str | None', 'pseudonym': 'str | None', 'name': 'str | None', 'users': 'list[GammaPublicProfileUser] | None', 'xUsername': 'str | None', 'verifiedBadge': 'bool | None'}, total=False)
class GammaGetPublicProfileResponse(_GammaGetPublicProfileResponseOptional):
    pass

_GammaGetRelatedTagsByIdParamsOptional = TypedDict("_GammaGetRelatedTagsByIdParamsOptional", {'omit_empty': 'bool | None', 'status': 'str | None'}, total=False)
class GammaGetRelatedTagsByIdParams(_GammaGetRelatedTagsByIdParamsOptional):
    id: int | Decimal

_GammaGetRelatedTagsBySlugParamsOptional = TypedDict("_GammaGetRelatedTagsBySlugParamsOptional", {'omit_empty': 'bool | None', 'status': 'str | None'}, total=False)
class GammaGetRelatedTagsBySlugParams(_GammaGetRelatedTagsBySlugParamsOptional):
    slug: str

class GammaGetSeriesCommentsCountParams(TypedDict):
    id: int | Decimal

_GammaGetSeriesCommentsCountResponseOptional = TypedDict("_GammaGetSeriesCommentsCountResponseOptional", {'count': 'int | Decimal | None'}, total=False)
class GammaGetSeriesCommentsCountResponse(_GammaGetSeriesCommentsCountResponseOptional):
    pass

_GammaGetSeriesParamsOptional = TypedDict("_GammaGetSeriesParamsOptional", {'include_chat': 'bool | None'}, total=False)
class GammaGetSeriesParams(_GammaGetSeriesParamsOptional):
    id: int | Decimal

_GammaGetSeriesResponseOptional = TypedDict("_GammaGetSeriesResponseOptional", {'id': 'str | None', 'ticker': 'str | None', 'slug': 'str | None', 'title': 'str | None', 'subtitle': 'str | None', 'seriesType': 'str | None', 'recurrence': 'str | None', 'description': 'str | None', 'image': 'str | None', 'icon': 'str | None', 'layout': 'str | None', 'active': 'bool | None', 'closed': 'bool | None', 'archived': 'bool | None', 'new': 'bool | None', 'featured': 'bool | None', 'restricted': 'bool | None', 'isTemplate': 'bool | None', 'templateVariables': 'bool | None', 'publishedAt': 'str | None', 'createdBy': 'str | None', 'updatedBy': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'commentsEnabled': 'bool | None', 'competitive': 'str | None', 'volume24hr': 'int | Decimal | None', 'volume': 'int | Decimal | None', 'liquidity': 'int | Decimal | None', 'startDate': 'str | None', 'pythTokenID': 'str | None', 'cgAssetName': 'str | None', 'score': 'int | Decimal | None', 'events': 'list[GammaEvent] | None', 'collections': 'list[GammaCollection] | None', 'categories': 'list[GammaCategory] | None', 'tags': 'list[GammaTag] | None', 'commentCount': 'int | Decimal | None', 'chats': 'list[GammaChat] | None'}, total=False)
class GammaGetSeriesResponse(_GammaGetSeriesResponseOptional):
    pass

class GammaGetSeriesSummaryByIdParams(TypedDict):
    id: int | Decimal

_GammaGetSeriesSummaryByIdResponseOptional = TypedDict("_GammaGetSeriesSummaryByIdResponseOptional", {'id': 'str | None', 'title': 'str | None', 'slug': 'str | None', 'eventDates': 'list[str] | None', 'eventWeeks': 'list[int | Decimal] | None', 'earliest_open_week': 'int | Decimal | None', 'earliest_open_date': 'str | None'}, total=False)
class GammaGetSeriesSummaryByIdResponse(_GammaGetSeriesSummaryByIdResponseOptional):
    pass

class GammaGetSeriesSummaryBySlugParams(TypedDict):
    slug: str

_GammaGetSeriesSummaryBySlugResponseOptional = TypedDict("_GammaGetSeriesSummaryBySlugResponseOptional", {'id': 'str | None', 'title': 'str | None', 'slug': 'str | None', 'eventDates': 'list[str] | None', 'eventWeeks': 'list[int | Decimal] | None', 'earliest_open_week': 'int | Decimal | None', 'earliest_open_date': 'str | None'}, total=False)
class GammaGetSeriesSummaryBySlugResponse(_GammaGetSeriesSummaryBySlugResponseOptional):
    pass

class GammaGetSportsMarketTypesParams(TypedDict):
    pass

_GammaGetSportsMarketTypesResponseOptional = TypedDict("_GammaGetSportsMarketTypesResponseOptional", {'marketTypes': 'list[str] | None'}, total=False)
class GammaGetSportsMarketTypesResponse(_GammaGetSportsMarketTypesResponseOptional):
    pass

class GammaGetSportsMetadataParams(TypedDict):
    pass

_GammaGetTagBySlugParamsOptional = TypedDict("_GammaGetTagBySlugParamsOptional", {'include_template': 'bool | None'}, total=False)
class GammaGetTagBySlugParams(_GammaGetTagBySlugParamsOptional):
    slug: str

_GammaGetTagBySlugResponseOptional = TypedDict("_GammaGetTagBySlugResponseOptional", {'id': 'str | None', 'label': 'str | None', 'slug': 'str | None', 'forceShow': 'bool | None', 'publishedAt': 'str | None', 'createdBy': 'int | Decimal | None', 'updatedBy': 'int | Decimal | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'forceHide': 'bool | None', 'isCarousel': 'bool | None'}, total=False)
class GammaGetTagBySlugResponse(_GammaGetTagBySlugResponseOptional):
    pass

_GammaGetTagParamsOptional = TypedDict("_GammaGetTagParamsOptional", {'include_template': 'bool | None'}, total=False)
class GammaGetTagParams(_GammaGetTagParamsOptional):
    id: int | Decimal

_GammaGetTagResponseOptional = TypedDict("_GammaGetTagResponseOptional", {'id': 'str | None', 'label': 'str | None', 'slug': 'str | None', 'forceShow': 'bool | None', 'publishedAt': 'str | None', 'createdBy': 'int | Decimal | None', 'updatedBy': 'int | Decimal | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'forceHide': 'bool | None', 'isCarousel': 'bool | None'}, total=False)
class GammaGetTagResponse(_GammaGetTagResponseOptional):
    pass

_GammaGetTagsRelatedToATagByIdParamsOptional = TypedDict("_GammaGetTagsRelatedToATagByIdParamsOptional", {'omit_empty': 'bool | None', 'status': 'str | None'}, total=False)
class GammaGetTagsRelatedToATagByIdParams(_GammaGetTagsRelatedToATagByIdParamsOptional):
    id: int | Decimal

_GammaGetTagsRelatedToATagBySlugParamsOptional = TypedDict("_GammaGetTagsRelatedToATagBySlugParamsOptional", {'omit_empty': 'bool | None', 'status': 'str | None'}, total=False)
class GammaGetTagsRelatedToATagBySlugParams(_GammaGetTagsRelatedToATagBySlugParamsOptional):
    slug: str

class GammaGetTeamParams(TypedDict):
    id: int | Decimal

_GammaGetTeamResponseOptional = TypedDict("_GammaGetTeamResponseOptional", {'id': 'int | Decimal | None', 'name': 'str | None', 'league': 'str | None', 'record': 'str | None', 'logo': 'str | None', 'abbreviation': 'str | None', 'alias': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None'}, total=False)
class GammaGetTeamResponse(_GammaGetTeamResponseOptional):
    pass

_GammaImageOptimizationOptional = TypedDict("_GammaImageOptimizationOptional", {'id': 'str | None', 'imageUrlSource': 'str | None', 'imageUrlOptimized': 'str | None', 'imageSizeKbSource': 'int | Decimal | None', 'imageSizeKbOptimized': 'int | Decimal | None', 'imageOptimizedComplete': 'bool | None', 'imageOptimizedLastUpdated': 'str | None', 'relID': 'int | Decimal | None', 'field': 'str | None', 'relname': 'str | None'}, total=False)
class GammaImageOptimization(_GammaImageOptimizationOptional):
    pass

_GammaInternalErrorOptional = TypedDict("_GammaInternalErrorOptional", {'type': 'str | None', 'error': 'str | None'}, total=False)
class GammaInternalError(_GammaInternalErrorOptional):
    pass

_GammaKeysetEventsResponseOptional = TypedDict("_GammaKeysetEventsResponseOptional", {'events': 'list[GammaEvent] | None', 'next_cursor': 'str | None'}, total=False)
class GammaKeysetEventsResponse(_GammaKeysetEventsResponseOptional):
    pass

_GammaKeysetMarketsResponseOptional = TypedDict("_GammaKeysetMarketsResponseOptional", {'markets': 'list[GammaMarket] | None', 'next_cursor': 'str | None'}, total=False)
class GammaKeysetMarketsResponse(_GammaKeysetMarketsResponseOptional):
    pass

_GammaListCommentsParamsOptional = TypedDict("_GammaListCommentsParamsOptional", {'limit': 'int | Decimal | None', 'offset': 'int | Decimal | None', 'order': 'str | None', 'ascending': 'bool | None', 'parent_entity_type': 'str | None', 'parent_entity_id': 'int | Decimal | None', 'get_positions': 'bool | None', 'holders_only': 'bool | None'}, total=False)
class GammaListCommentsParams(_GammaListCommentsParamsOptional):
    pass

_GammaListEventCreatorsParamsOptional = TypedDict("_GammaListEventCreatorsParamsOptional", {'limit': 'int | Decimal | None', 'offset': 'int | Decimal | None', 'order': 'str | None', 'ascending': 'bool | None', 'creator_name': 'str | None', 'creator_handle': 'str | None'}, total=False)
class GammaListEventCreatorsParams(_GammaListEventCreatorsParamsOptional):
    pass

_GammaListEventsKeysetParamsOptional = TypedDict("_GammaListEventsKeysetParamsOptional", {'limit': 'int | Decimal | None', 'order': 'str | None', 'ascending': 'bool | None', 'after_cursor': 'str | None', 'offset': 'int | Decimal | None', 'id': 'list[int | Decimal] | None', 'slug': 'list[str] | None', 'closed': 'Any | None', 'live': 'bool | None', 'featured': 'bool | None', 'cyom': 'bool | None', 'title_search': 'str | None', 'liquidity_min': 'int | Decimal | None', 'liquidity_max': 'int | Decimal | None', 'volume_min': 'int | Decimal | None', 'volume_max': 'int | Decimal | None', 'start_date_min': 'str | None', 'start_date_max': 'str | None', 'end_date_min': 'str | None', 'end_date_max': 'str | None', 'start_time_min': 'str | None', 'start_time_max': 'str | None', 'tag_id': 'list[int | Decimal] | None', 'tag_slug': 'str | None', 'exclude_tag_id': 'list[int | Decimal] | None', 'related_tags': 'bool | None', 'tag_match': 'str | None', 'series_id': 'list[int | Decimal] | None', 'game_id': 'list[int | Decimal] | None', 'event_date': 'str | None', 'event_week': 'int | Decimal | None', 'featured_order': 'bool | None', 'recurrence': 'str | None', 'created_by': 'list[str] | None', 'parent_event_id': 'int | Decimal | None', 'include_children': 'bool | None', 'partner_slug': 'str | None', 'include_chat': 'bool | None', 'include_template': 'bool | None', 'include_best_lines': 'bool | None', 'locale': 'str | None'}, total=False)
class GammaListEventsKeysetParams(_GammaListEventsKeysetParamsOptional):
    pass

_GammaListEventsKeysetResponseOptional = TypedDict("_GammaListEventsKeysetResponseOptional", {'events': 'list[GammaEvent] | None', 'next_cursor': 'str | None'}, total=False)
class GammaListEventsKeysetResponse(_GammaListEventsKeysetResponseOptional):
    pass

_GammaListEventsPaginationParamsOptional = TypedDict("_GammaListEventsPaginationParamsOptional", {'limit': 'int | Decimal | None', 'offset': 'int | Decimal | None', 'order': 'str | None', 'ascending': 'bool | None', 'include_chat': 'bool | None', 'include_template': 'bool | None', 'recurrence': 'str | None'}, total=False)
class GammaListEventsPaginationParams(_GammaListEventsPaginationParamsOptional):
    pass

_GammaListEventsPaginationResponseOptional = TypedDict("_GammaListEventsPaginationResponseOptional", {'data': 'list[GammaEvent] | None', 'pagination': 'GammaPagination | None'}, total=False)
class GammaListEventsPaginationResponse(_GammaListEventsPaginationResponseOptional):
    pass

_GammaListEventsParamsOptional = TypedDict("_GammaListEventsParamsOptional", {'limit': 'int | Decimal | None', 'offset': 'int | Decimal | None', 'order': 'str | None', 'ascending': 'bool | None', 'id': 'list[int | Decimal] | None', 'tag_id': 'int | Decimal | None', 'exclude_tag_id': 'list[int | Decimal] | None', 'slug': 'list[str] | None', 'tag_slug': 'str | None', 'related_tags': 'bool | None', 'active': 'bool | None', 'archived': 'bool | None', 'featured': 'bool | None', 'cyom': 'bool | None', 'include_chat': 'bool | None', 'include_template': 'bool | None', 'recurrence': 'str | None', 'closed': 'Any | None', 'liquidity_min': 'int | Decimal | None', 'liquidity_max': 'int | Decimal | None', 'volume_min': 'int | Decimal | None', 'volume_max': 'int | Decimal | None', 'start_date_min': 'str | None', 'start_date_max': 'str | None', 'end_date_min': 'str | None', 'end_date_max': 'str | None'}, total=False)
class GammaListEventsParams(_GammaListEventsParamsOptional):
    pass

_GammaListMarketsKeysetParamsOptional = TypedDict("_GammaListMarketsKeysetParamsOptional", {'limit': 'int | Decimal | None', 'order': 'str | None', 'ascending': 'bool | None', 'after_cursor': 'str | None', 'offset': 'int | Decimal | None', 'id': 'list[int | Decimal] | None', 'slug': 'list[str] | None', 'closed': 'Any | None', 'decimalized': 'bool | None', 'clob_token_ids': 'list[str] | None', 'condition_ids': 'list[str] | None', 'question_ids': 'list[str] | None', 'liquidity_num_min': 'int | Decimal | None', 'liquidity_num_max': 'int | Decimal | None', 'volume_num_min': 'int | Decimal | None', 'volume_num_max': 'int | Decimal | None', 'start_date_min': 'str | None', 'start_date_max': 'str | None', 'end_date_min': 'str | None', 'end_date_max': 'str | None', 'tag_id': 'list[int | Decimal] | None', 'related_tags': 'bool | None', 'tag_match': 'str | None', 'cyom': 'bool | None', 'rfq_enabled': 'bool | None', 'uma_resolution_status': 'str | None', 'game_id': 'str | None', 'sports_market_types': 'list[str] | None', 'include_tag': 'bool | None', 'locale': 'str | None'}, total=False)
class GammaListMarketsKeysetParams(_GammaListMarketsKeysetParamsOptional):
    pass

_GammaListMarketsKeysetResponseOptional = TypedDict("_GammaListMarketsKeysetResponseOptional", {'markets': 'list[GammaMarket] | None', 'next_cursor': 'str | None'}, total=False)
class GammaListMarketsKeysetResponse(_GammaListMarketsKeysetResponseOptional):
    pass

_GammaListMarketsParamsOptional = TypedDict("_GammaListMarketsParamsOptional", {'limit': 'int | Decimal | None', 'offset': 'int | Decimal | None', 'order': 'str | None', 'ascending': 'bool | None', 'id': 'list[int | Decimal] | None', 'slug': 'list[str] | None', 'clob_token_ids': 'list[str] | None', 'condition_ids': 'list[str] | None', 'liquidity_num_min': 'int | Decimal | None', 'liquidity_num_max': 'int | Decimal | None', 'volume_num_min': 'int | Decimal | None', 'volume_num_max': 'int | Decimal | None', 'start_date_min': 'str | None', 'start_date_max': 'str | None', 'end_date_min': 'str | None', 'end_date_max': 'str | None', 'tag_id': 'int | Decimal | None', 'related_tags': 'bool | None', 'cyom': 'bool | None', 'uma_resolution_status': 'str | None', 'game_id': 'str | None', 'sports_market_types': 'list[str] | None', 'rewards_min_size': 'int | Decimal | None', 'question_ids': 'list[str] | None', 'include_tag': 'bool | None', 'closed': 'Any | None'}, total=False)
class GammaListMarketsParams(_GammaListMarketsParamsOptional):
    pass

_GammaListSeriesParamsOptional = TypedDict("_GammaListSeriesParamsOptional", {'limit': 'int | Decimal | None', 'offset': 'int | Decimal | None', 'order': 'str | None', 'ascending': 'bool | None', 'slug': 'list[str] | None', 'categories_ids': 'list[int | Decimal] | None', 'categories_labels': 'list[str] | None', 'closed': 'bool | None', 'include_chat': 'bool | None', 'recurrence': 'str | None', 'exclude_events': 'bool | None'}, total=False)
class GammaListSeriesParams(_GammaListSeriesParamsOptional):
    pass

_GammaListSportEventsResultsParamsOptional = TypedDict("_GammaListSportEventsResultsParamsOptional", {'limit': 'int | Decimal | None', 'offset': 'int | Decimal | None', 'order': 'str | None', 'ascending': 'bool | None'}, total=False)
class GammaListSportEventsResultsParams(_GammaListSportEventsResultsParamsOptional):
    pass

_GammaListTagsParamsOptional = TypedDict("_GammaListTagsParamsOptional", {'limit': 'int | Decimal | None', 'offset': 'int | Decimal | None', 'order': 'str | None', 'ascending': 'bool | None', 'include_template': 'bool | None', 'is_carousel': 'bool | None'}, total=False)
class GammaListTagsParams(_GammaListTagsParamsOptional):
    pass

_GammaListTeamsParamsOptional = TypedDict("_GammaListTeamsParamsOptional", {'limit': 'int | Decimal | None', 'offset': 'int | Decimal | None', 'order': 'str | None', 'ascending': 'bool | None', 'league': 'list[str] | None', 'name': 'list[str] | None', 'abbreviation': 'list[str] | None'}, total=False)
class GammaListTeamsParams(_GammaListTeamsParamsOptional):
    pass

_GammaMarketOptional = TypedDict("_GammaMarketOptional", {'id': 'str | None', 'question': 'str | None', 'conditionId': 'str | None', 'slug': 'str | None', 'twitterCardImage': 'str | None', 'resolutionSource': 'str | None', 'endDate': 'str | None', 'category': 'str | None', 'liquidity': 'str | None', 'sponsorName': 'str | None', 'sponsorImage': 'str | None', 'startDate': 'str | None', 'xAxisValue': 'str | None', 'yAxisValue': 'str | None', 'denominationToken': 'str | None', 'fee': 'str | None', 'image': 'str | None', 'icon': 'str | None', 'lowerBound': 'str | None', 'upperBound': 'str | None', 'description': 'str | None', 'outcomes': 'str | None', 'outcomePrices': 'str | None', 'volume': 'str | None', 'active': 'bool | None', 'marketType': 'str | None', 'formatType': 'str | None', 'lowerBoundDate': 'str | None', 'upperBoundDate': 'str | None', 'closed': 'bool | None', 'createdBy': 'int | Decimal | None', 'updatedBy': 'int | Decimal | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'closedTime': 'str | None', 'wideFormat': 'bool | None', 'new': 'bool | None', 'mailchimpTag': 'str | None', 'featured': 'bool | None', 'archived': 'bool | None', 'resolvedBy': 'str | None', 'restricted': 'bool | None', 'marketGroup': 'int | Decimal | None', 'groupItemTitle': 'str | None', 'groupItemThreshold': 'str | None', 'questionID': 'str | None', 'umaEndDate': 'str | None', 'enableOrderBook': 'bool | None', 'orderPriceMinTickSize': 'int | Decimal | None', 'orderMinSize': 'int | Decimal | None', 'umaResolutionStatus': 'str | None', 'curationOrder': 'int | Decimal | None', 'volumeNum': 'int | Decimal | None', 'liquidityNum': 'int | Decimal | None', 'endDateIso': 'str | None', 'startDateIso': 'str | None', 'umaEndDateIso': 'str | None', 'hasReviewedDates': 'bool | None', 'readyForCron': 'bool | None', 'commentsEnabled': 'bool | None', 'volume24hr': 'int | Decimal | None', 'volume1wk': 'int | Decimal | None', 'volume1mo': 'int | Decimal | None', 'volume1yr': 'int | Decimal | None', 'gameStartTime': 'str | None', 'secondsDelay': 'int | Decimal | None', 'version': 'str | None', 'clobTokenIds': 'str | None', 'positionIds': 'list[str] | None', 'disqusThread': 'str | None', 'shortOutcomes': 'str | None', 'teamAID': 'str | None', 'teamBID': 'str | None', 'umaBond': 'str | None', 'umaReward': 'str | None', 'volume24hrClob': 'int | Decimal | None', 'volume1wkClob': 'int | Decimal | None', 'volume1moClob': 'int | Decimal | None', 'volume1yrClob': 'int | Decimal | None', 'volumeClob': 'int | Decimal | None', 'liquidityClob': 'int | Decimal | None', 'makerBaseFee': 'int | Decimal | None', 'takerBaseFee': 'int | Decimal | None', 'customLiveness': 'int | Decimal | None', 'acceptingOrders': 'bool | None', 'notificationsEnabled': 'bool | None', 'score': 'int | Decimal | None', 'imageOptimized': 'GammaImageOptimization | None', 'iconOptimized': 'GammaImageOptimization | None', 'events': 'list[GammaEvent] | None', 'categories': 'list[GammaCategory] | None', 'tags': 'list[GammaTag] | None', 'creator': 'str | None', 'ready': 'bool | None', 'funded': 'bool | None', 'pastSlugs': 'str | None', 'readyTimestamp': 'str | None', 'fundedTimestamp': 'str | None', 'acceptingOrdersTimestamp': 'str | None', 'competitive': 'int | Decimal | None', 'rewardsMinSize': 'int | Decimal | None', 'rewardsMaxSpread': 'int | Decimal | None', 'spread': 'int | Decimal | None', 'automaticallyResolved': 'bool | None', 'oneDayPriceChange': 'int | Decimal | None', 'oneHourPriceChange': 'int | Decimal | None', 'oneWeekPriceChange': 'int | Decimal | None', 'oneMonthPriceChange': 'int | Decimal | None', 'oneYearPriceChange': 'int | Decimal | None', 'lastTradePrice': 'int | Decimal | None', 'bestBid': 'int | Decimal | None', 'bestAsk': 'int | Decimal | None', 'automaticallyActive': 'bool | None', 'clearBookOnStart': 'bool | None', 'chartColor': 'str | None', 'seriesColor': 'str | None', 'showGmpSeries': 'bool | None', 'showGmpOutcome': 'bool | None', 'manualActivation': 'bool | None', 'negRiskOther': 'bool | None', 'gameId': 'str | None', 'groupItemRange': 'str | None', 'sportsMarketType': 'str | None', 'line': 'int | Decimal | None', 'umaResolutionStatuses': 'str | None', 'pendingDeployment': 'bool | None', 'deploying': 'bool | None', 'deployingTimestamp': 'str | None', 'scheduledDeploymentTimestamp': 'str | None', 'rfqEnabled': 'bool | None', 'eventStartTime': 'str | None', 'feesEnabled': 'bool | None', 'feeSchedule': 'GammaFeeSchedule | None'}, total=False)
class GammaMarket(_GammaMarketOptional):
    pass

_GammaMarketDescriptionOptional = TypedDict("_GammaMarketDescriptionOptional", {'description': 'str | None'}, total=False)
class GammaMarketDescription(_GammaMarketDescriptionOptional):
    pass

_GammaMarketsInformationBodyOptional = TypedDict("_GammaMarketsInformationBodyOptional", {'id': 'list[int | Decimal] | None', 'slug': 'list[str] | None', 'closed': 'bool | None', 'clobTokenIds': 'list[str] | None', 'conditionIds': 'list[str] | None', 'liquidityNumMin': 'int | Decimal | None', 'liquidityNumMax': 'int | Decimal | None', 'volumeNumMin': 'int | Decimal | None', 'volumeNumMax': 'int | Decimal | None', 'startDateMin': 'str | None', 'startDateMax': 'str | None', 'endDateMin': 'str | None', 'endDateMax': 'str | None', 'relatedTags': 'bool | None', 'tagId': 'int | Decimal | None', 'cyom': 'bool | None', 'umaResolutionStatus': 'str | None', 'gameId': 'str | None', 'sportsMarketTypes': 'list[str] | None', 'rewardsMinSize': 'int | Decimal | None', 'questionIds': 'list[str] | None', 'includeTags': 'bool | None'}, total=False)
class GammaMarketsInformationBody(_GammaMarketsInformationBodyOptional):
    pass

_GammaPaginationOptional = TypedDict("_GammaPaginationOptional", {'hasMore': 'bool | None', 'totalResults': 'int | Decimal | None'}, total=False)
class GammaPagination(_GammaPaginationOptional):
    pass

_GammaProfileOptional = TypedDict("_GammaProfileOptional", {'id': 'str | None', 'name': 'str | None', 'user': 'int | Decimal | None', 'referral': 'str | None', 'createdBy': 'int | Decimal | None', 'updatedBy': 'int | Decimal | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'utmSource': 'str | None', 'utmMedium': 'str | None', 'utmCampaign': 'str | None', 'utmContent': 'str | None', 'utmTerm': 'str | None', 'walletActivated': 'bool | None', 'pseudonym': 'str | None', 'displayUsernamePublic': 'bool | None', 'profileImage': 'str | None', 'bio': 'str | None', 'proxyWallet': 'str | None', 'profileImageOptimized': 'GammaImageOptimization | None', 'isCloseOnly': 'bool | None', 'isCertReq': 'bool | None', 'certReqDate': 'str | None'}, total=False)
class GammaProfile(_GammaProfileOptional):
    pass

_GammaPublicProfileErrorOptional = TypedDict("_GammaPublicProfileErrorOptional", {'type': 'str | None', 'error': 'str | None'}, total=False)
class GammaPublicProfileError(_GammaPublicProfileErrorOptional):
    pass

_GammaPublicProfileResponseOptional = TypedDict("_GammaPublicProfileResponseOptional", {'createdAt': 'str | None', 'proxyWallet': 'str | None', 'profileImage': 'str | None', 'displayUsernamePublic': 'bool | None', 'bio': 'str | None', 'pseudonym': 'str | None', 'name': 'str | None', 'users': 'list[GammaPublicProfileUser] | None', 'xUsername': 'str | None', 'verifiedBadge': 'bool | None'}, total=False)
class GammaPublicProfileResponse(_GammaPublicProfileResponseOptional):
    pass

_GammaPublicProfileUserOptional = TypedDict("_GammaPublicProfileUserOptional", {'id': 'str | None', 'creator': 'bool | None', 'mod': 'bool | None'}, total=False)
class GammaPublicProfileUser(_GammaPublicProfileUserOptional):
    pass

_GammaPublicSearchParamsOptional = TypedDict("_GammaPublicSearchParamsOptional", {'cache': 'bool | None', 'events_status': 'str | None', 'limit_per_type': 'int | Decimal | None', 'page': 'int | Decimal | None', 'events_tag': 'list[str] | None', 'keep_closed_markets': 'int | Decimal | None', 'sort': 'str | None', 'ascending': 'bool | None', 'search_tags': 'bool | None', 'search_profiles': 'bool | None', 'recurrence': 'str | None', 'exclude_tag_id': 'list[int | Decimal] | None', 'optimized': 'bool | None'}, total=False)
class GammaPublicSearchParams(_GammaPublicSearchParamsOptional):
    q: str

_GammaPublicSearchResponseOptional = TypedDict("_GammaPublicSearchResponseOptional", {'events': 'list[GammaEvent] | None', 'tags': 'list[GammaSearchTag] | None', 'profiles': 'list[GammaProfile] | None', 'pagination': 'GammaPagination | None'}, total=False)
class GammaPublicSearchResponse(_GammaPublicSearchResponseOptional):
    pass

_GammaReactionOptional = TypedDict("_GammaReactionOptional", {'id': 'str | None', 'commentID': 'int | Decimal | None', 'reactionType': 'str | None', 'icon': 'str | None', 'userAddress': 'str | None', 'createdAt': 'str | None', 'profile': 'GammaCommentProfile | None'}, total=False)
class GammaReaction(_GammaReactionOptional):
    pass

_GammaRelatedTagOptional = TypedDict("_GammaRelatedTagOptional", {'id': 'str | None', 'tagID': 'int | Decimal | None', 'relatedTagID': 'int | Decimal | None', 'rank': 'int | Decimal | None'}, total=False)
class GammaRelatedTag(_GammaRelatedTagOptional):
    pass

_GammaSearchOptional = TypedDict("_GammaSearchOptional", {'events': 'list[GammaEvent] | None', 'tags': 'list[GammaSearchTag] | None', 'profiles': 'list[GammaProfile] | None', 'pagination': 'GammaPagination | None'}, total=False)
class GammaSearch(_GammaSearchOptional):
    pass

_GammaSearchTagOptional = TypedDict("_GammaSearchTagOptional", {'id': 'str | None', 'label': 'str | None', 'slug': 'str | None', 'event_count': 'int | Decimal | None'}, total=False)
class GammaSearchTag(_GammaSearchTagOptional):
    pass

_GammaSeriesOptional = TypedDict("_GammaSeriesOptional", {'id': 'str | None', 'ticker': 'str | None', 'slug': 'str | None', 'title': 'str | None', 'subtitle': 'str | None', 'seriesType': 'str | None', 'recurrence': 'str | None', 'description': 'str | None', 'image': 'str | None', 'icon': 'str | None', 'layout': 'str | None', 'active': 'bool | None', 'closed': 'bool | None', 'archived': 'bool | None', 'new': 'bool | None', 'featured': 'bool | None', 'restricted': 'bool | None', 'isTemplate': 'bool | None', 'templateVariables': 'bool | None', 'publishedAt': 'str | None', 'createdBy': 'str | None', 'updatedBy': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'commentsEnabled': 'bool | None', 'competitive': 'str | None', 'volume24hr': 'int | Decimal | None', 'volume': 'int | Decimal | None', 'liquidity': 'int | Decimal | None', 'startDate': 'str | None', 'pythTokenID': 'str | None', 'cgAssetName': 'str | None', 'score': 'int | Decimal | None', 'events': 'list[GammaEvent] | None', 'collections': 'list[GammaCollection] | None', 'categories': 'list[GammaCategory] | None', 'tags': 'list[GammaTag] | None', 'commentCount': 'int | Decimal | None', 'chats': 'list[GammaChat] | None'}, total=False)
class GammaSeries(_GammaSeriesOptional):
    pass

_GammaSeriesSummaryOptional = TypedDict("_GammaSeriesSummaryOptional", {'id': 'str | None', 'title': 'str | None', 'slug': 'str | None', 'eventDates': 'list[str] | None', 'eventWeeks': 'list[int | Decimal] | None', 'earliest_open_week': 'int | Decimal | None', 'earliest_open_date': 'str | None'}, total=False)
class GammaSeriesSummary(_GammaSeriesSummaryOptional):
    pass

_GammaServiceUnavailableErrorOptional = TypedDict("_GammaServiceUnavailableErrorOptional", {'type': 'str | None', 'error': 'str | None'}, total=False)
class GammaServiceUnavailableError(_GammaServiceUnavailableErrorOptional):
    pass

_GammaSportsMarketTypesResponseOptional = TypedDict("_GammaSportsMarketTypesResponseOptional", {'marketTypes': 'list[str] | None'}, total=False)
class GammaSportsMarketTypesResponse(_GammaSportsMarketTypesResponseOptional):
    pass

_GammaSportsMetadataOptional = TypedDict("_GammaSportsMetadataOptional", {'sport': 'str | None', 'image': 'str | None', 'resolution': 'str | None', 'ordering': 'str | None', 'tags': 'str | None', 'series': 'str | None'}, total=False)
class GammaSportsMetadata(_GammaSportsMetadataOptional):
    pass

_GammaTagOptional = TypedDict("_GammaTagOptional", {'id': 'str | None', 'label': 'str | None', 'slug': 'str | None', 'forceShow': 'bool | None', 'publishedAt': 'str | None', 'createdBy': 'int | Decimal | None', 'updatedBy': 'int | Decimal | None', 'createdAt': 'str | None', 'updatedAt': 'str | None', 'forceHide': 'bool | None', 'isCarousel': 'bool | None'}, total=False)
class GammaTag(_GammaTagOptional):
    pass

_GammaTeamOptional = TypedDict("_GammaTeamOptional", {'id': 'int | Decimal | None', 'name': 'str | None', 'league': 'str | None', 'record': 'str | None', 'logo': 'str | None', 'abbreviation': 'str | None', 'alias': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None'}, total=False)
class GammaTeam(_GammaTeamOptional):
    pass

_GammaTemplateOptional = TypedDict("_GammaTemplateOptional", {'id': 'str | None', 'eventTitle': 'str | None', 'eventSlug': 'str | None', 'eventImage': 'str | None', 'marketTitle': 'str | None', 'description': 'str | None', 'resolutionSource': 'str | None', 'negRisk': 'bool | None', 'sortBy': 'str | None', 'showMarketImages': 'bool | None', 'seriesSlug': 'str | None', 'outcomes': 'str | None'}, total=False)
class GammaTemplate(_GammaTemplateOptional):
    pass

_GammaValidationErrorOptional = TypedDict("_GammaValidationErrorOptional", {'type': 'str | None', 'error': 'str | None'}, total=False)
class GammaValidationError(_GammaValidationErrorOptional):
    pass

_RelayerAuthorizationsParamsWireKeys = TypedDict("_RelayerAuthorizationsParamsWireKeys", {'Idempotency-Key': 'str'})
class RelayerAuthorizationsParams(_RelayerAuthorizationsParamsWireKeys):
    body: RelayerAuthorizationsParamsBody

_RelayerAuthorizationsParamsBodyOptional = TypedDict("_RelayerAuthorizationsParamsBodyOptional", {'nonce': 'str', 'signature': 'str'}, total=False)
class RelayerAuthorizationsParamsBody(_RelayerAuthorizationsParamsBodyOptional):
    scopes: list[str]
    validUntil: str
    deadline: str
    sessionSignerAddress: str
    walletAddress: str

_RelayerDeployedResponseOptional = TypedDict("_RelayerDeployedResponseOptional", {'deployed': 'bool | None'}, total=False)
class RelayerDeployedResponse(_RelayerDeployedResponseOptional):
    pass

class RelayerErrorResponse(TypedDict):
    error: str

_RelayerGetDeployedParamsOptional = TypedDict("_RelayerGetDeployedParamsOptional", {'type': 'str | None'}, total=False)
class RelayerGetDeployedParams(_RelayerGetDeployedParamsOptional):
    address: str

_RelayerGetDeployedResponseOptional = TypedDict("_RelayerGetDeployedResponseOptional", {'deployed': 'bool | None'}, total=False)
class RelayerGetDeployedResponse(_RelayerGetDeployedResponseOptional):
    pass

class RelayerGetExecuteParamsParams(TypedDict):
    address: str
    type: str

class RelayerGetNonceParams(TypedDict):
    address: str
    type: str

_RelayerGetNonceResponseOptional = TypedDict("_RelayerGetNonceResponseOptional", {'nonce': 'str | None'}, total=False)
class RelayerGetNonceResponse(_RelayerGetNonceResponseOptional):
    pass

class RelayerGetRelayPayloadParams(TypedDict):
    address: str
    type: str

_RelayerGetRelayPayloadResponseOptional = TypedDict("_RelayerGetRelayPayloadResponseOptional", {'address': 'str | None', 'nonce': 'str | None'}, total=False)
class RelayerGetRelayPayloadResponse(_RelayerGetRelayPayloadResponseOptional):
    pass

_RelayerGetRelayerApiKeysParamsOptional = TypedDict("_RelayerGetRelayerApiKeysParamsOptional", {'RELAYER_API_KEY': 'str | None', 'RELAYER_API_KEY_ADDRESS': 'str | None'}, total=False)
class RelayerGetRelayerApiKeysParams(_RelayerGetRelayerApiKeysParamsOptional):
    pass

class RelayerGetTransactionParams(TypedDict):
    id: str

_RelayerGetTransactionsParamsOptional = TypedDict("_RelayerGetTransactionsParamsOptional", {'POLY_BUILDER_API_KEY': 'str | None', 'POLY_BUILDER_TIMESTAMP': 'str | None', 'POLY_BUILDER_PASSPHRASE': 'str | None', 'POLY_BUILDER_SIGNATURE': 'str | None', 'RELAYER_API_KEY': 'str | None', 'RELAYER_API_KEY_ADDRESS': 'str | None'}, total=False)
class RelayerGetTransactionsParams(_RelayerGetTransactionsParamsOptional):
    pass

_RelayerNonceResponseOptional = TypedDict("_RelayerNonceResponseOptional", {'nonce': 'str | None'}, total=False)
class RelayerNonceResponse(_RelayerNonceResponseOptional):
    pass

_RelayerPostSubmitParamsOptional = TypedDict("_RelayerPostSubmitParamsOptional", {'POLY_BUILDER_API_KEY': 'str | None', 'POLY_BUILDER_TIMESTAMP': 'str | None', 'POLY_BUILDER_PASSPHRASE': 'str | None', 'POLY_BUILDER_SIGNATURE': 'str | None', 'RELAYER_API_KEY': 'str | None', 'RELAYER_API_KEY_ADDRESS': 'str | None'}, total=False)
class RelayerPostSubmitParams(_RelayerPostSubmitParamsOptional):
    body: RelayerSubmitRequest

_RelayerPostSubmitResponseOptional = TypedDict("_RelayerPostSubmitResponseOptional", {'transactionID': 'str | None', 'state': 'str | None'}, total=False)
class RelayerPostSubmitResponse(_RelayerPostSubmitResponseOptional):
    pass

_RelayerRelayPayloadResponseOptional = TypedDict("_RelayerRelayPayloadResponseOptional", {'address': 'str | None', 'nonce': 'str | None'}, total=False)
class RelayerRelayPayloadResponse(_RelayerRelayPayloadResponseOptional):
    pass

_RelayerRelayerApiKeyOptional = TypedDict("_RelayerRelayerApiKeyOptional", {'apiKey': 'str | None', 'address': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None'}, total=False)
class RelayerRelayerApiKey(_RelayerRelayerApiKeyOptional):
    pass

_RelayerRelayerTransactionOptional = TypedDict("_RelayerRelayerTransactionOptional", {'transactionID': 'str | None', 'transactionHash': 'str | None', 'from': 'str | None', 'to': 'str | None', 'proxyAddress': 'str | None', 'data': 'str | None', 'nonce': 'str | None', 'value': 'str | None', 'signature': 'str | None', 'state': 'str | None', 'type': 'str | None', 'owner': 'str | None', 'metadata': 'str | None', 'createdAt': 'str | None', 'updatedAt': 'str | None'}, total=False)
class RelayerRelayerTransaction(_RelayerRelayerTransactionOptional):
    pass

_RelayerRevocationsParamsWireKeys = TypedDict("_RelayerRevocationsParamsWireKeys", {'Idempotency-Key': 'str'})
class RelayerRevocationsParams(_RelayerRevocationsParamsWireKeys):
    body: RelayerRevocationsParamsBody

_RelayerRevocationsParamsBodyOptional = TypedDict("_RelayerRevocationsParamsBodyOptional", {'nonce': 'str', 'signature': 'str'}, total=False)
class RelayerRevocationsParamsBody(_RelayerRevocationsParamsBodyOptional):
    deadline: str
    sessionSignerAddress: str
    walletAddress: str

_RelayerSignatureParamsOptional = TypedDict("_RelayerSignatureParamsOptional", {'gasPrice': 'str | None', 'operation': 'str | None', 'safeTxnGas': 'str | None', 'baseGas': 'str | None', 'gasToken': 'str | None', 'refundReceiver': 'str | None'}, total=False)
class RelayerSignatureParams(_RelayerSignatureParamsOptional):
    pass

_RelayerSubmitRequestOptional = TypedDict("_RelayerSubmitRequestOptional", {'nonce': 'str', 'signature': 'str'}, total=False)
_RelayerSubmitRequestWireKeys = TypedDict("_RelayerSubmitRequestWireKeys", {'from': 'str'})
class RelayerSubmitRequest(_RelayerSubmitRequestWireKeys, _RelayerSubmitRequestOptional):
    to: str
    proxyWallet: str
    data: str
    signatureParams: RelayerSignatureParams
    type: str

_RelayerSubmitResponseOptional = TypedDict("_RelayerSubmitResponseOptional", {'transactionID': 'str | None', 'state': 'str | None'}, total=False)
class RelayerSubmitResponse(_RelayerSubmitResponseOptional):
    pass

BridgeAddress = str
BuilderGatewayAcceptRfqResponse = Any
BuilderGatewayCreateRfqResponse = Any
BuilderGatewayGetRfqResponse = Any
ClobClobRewards = dict[str, Any]
ClobDeleteApiKeyResponse = str
ClobDropNotificationsResponse = str
ClobGetBooksGetResponse = list[ClobOrderBookSummary]
ClobGetBooksPostResponse = list[ClobOrderBookSummary]
ClobGetBuilderApiKeysResponse = list[str]
ClobGetCurrentRebatedFeesResponse = list[ClobRebatedFees]
ClobGetLastTradesPricesGetResponse = list[ClobGetLastTradesPricesGetResponseValueItem]
ClobGetLastTradesPricesPostResponse = list[ClobGetLastTradesPricesPostResponseValueItem]
ClobGetMarketsLiveActivityResponse = list[ClobLiveActivityMarket]
ClobGetMidpointsGetResponse = dict[str, str]
ClobGetMidpointsPostResponse = dict[str, str]
ClobGetNotificationsResponse = list[ClobNotification]
ClobGetOrdersScoringResponse = dict[str, bool]
ClobGetPricesGetResponse = dict[str, dict[str, int | Decimal]]
ClobGetPricesPostResponse = dict[str, dict[str, int | Decimal]]
ClobGetRewardPercentagesForUserResponse = dict[str, int | Decimal]
ClobGetSpreadsResponse = dict[str, str]
ClobGetTimeResponse = int | Decimal
ClobGetTotalEarningsForUserForDayResponse = list[ClobTotalUserEarning]
ClobListSessionSignersResponse = Any
ClobOrdersScoringResponse = dict[str, bool]
ClobPostOrdersResponse = list[ClobSendOrderResponse]
ClobPostOrdersScoringResponse = dict[str, bool]
ClobRevokeBuilderApiKeyResponse = str
CollateralReturnPlanResponse = Any
CollateralReturnSubmitResponse = Any
CombosRfqDirection = str
CombosRfqRFQStatus = str
CombosRfqSide = str
CombosRfqSignatureType = int | Decimal
DataErrorCode = str
DataGetLeaderboardResponse = Any
DataLeaderboardResponse = Any
DataTokenModule = str
GammaGetAbridgedMarketsResponse = list[GammaMarket]
GammaGetCommentsByIdResponse = list[GammaComment]
GammaGetCommentsByUserAddressResponse = list[GammaComment]
GammaGetEventTagsResponse = list[GammaTag]
GammaGetGammaStatusResponse = Any
GammaGetMarketTagsResponse = list[GammaTag]
GammaGetMarketsInformationResponse = list[GammaMarket]
GammaGetRelatedTagsByIdResponse = list[GammaRelatedTag]
GammaGetRelatedTagsBySlugResponse = list[GammaRelatedTag]
GammaGetSportsMetadataResponse = list[GammaSportsMetadata]
GammaGetTagsRelatedToATagByIdResponse = list[GammaTag]
GammaGetTagsRelatedToATagBySlugResponse = list[GammaTag]
GammaListCommentsResponse = list[GammaComment]
GammaListEventCreatorsResponse = list[GammaEventCreator]
GammaListEventsResponse = list[GammaEvent]
GammaListMarketsResponse = list[GammaMarket]
GammaListSeriesResponse = list[GammaSeries]
GammaListSportEventsResultsResponse = list[GammaEvent]
GammaListTagsResponse = list[GammaTag]
GammaListTeamsResponse = list[GammaTeam]
RelayerAddress = str
RelayerAuthorizationsResponse = Any
RelayerGetExecuteParamsResponse = Any
RelayerGetRelayerApiKeysResponse = list[RelayerRelayerApiKey]
RelayerGetTransactionResponse = list[RelayerRelayerTransaction]
RelayerGetTransactionsResponse = list[RelayerRelayerTransaction]
RelayerRevocationsResponse = Any
