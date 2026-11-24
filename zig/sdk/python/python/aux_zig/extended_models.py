"""Extended REST wire models generated from schema/extended.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class AccountLeverage(TypedDict):
    market: str
    leverage: str

class AccountLeverageModel(TypedDict):
    market: str
    leverage: str

_AccountModelOptional = TypedDict("_AccountModelOptional", {'id': 'int | Decimal | str | None', 'bridgeStarknetAddress': 'str | None'}, total=False)
class AccountModel(_AccountModelOptional):
    description: str
    accountIndex: int | Decimal | str
    status: str
    l2Key: str
    l2Vault: int | Decimal | str

class AccountTradeModel(TypedDict):
    id: int | Decimal | str
    accountId: int | Decimal | str
    market: str
    orderId: int | Decimal | str
    side: OrderSide
    price: str
    qty: str
    value: str
    fee: str
    isTaker: bool
    tradeType: TradeType
    createdTime: int | Decimal | str

_AccountUpdateRequestOptional = TypedDict("_AccountUpdateRequestOptional", {'description': 'str | None'}, total=False)
class AccountUpdateRequest(_AccountUpdateRequestOptional):
    pass

class ApiKeyResponseModel(TypedDict):
    key: str

_ApiResponseOptional = TypedDict("_ApiResponseOptional", {'data': 'Any | None'}, total=False)
class ApiResponse(_ApiResponseOptional):
    ok: bool

class AssetModel(TypedDict):
    id: int | Decimal | str
    name: str
    symbol: str
    precision: int | Decimal | str
    isActive: bool
    isCollateral: bool
    starkexId: str
    starkexResolution: int | Decimal | str
    l1Id: str
    l1Resolution: int | Decimal | str
    version: int | Decimal | str
    type: AssetType
    canBeUsedAsCollateral: bool

_AssetOperationModelOptional = TypedDict("_AssetOperationModelOptional", {'counterpartyAccountId': 'int | Decimal | str | None', 'transactionHash': 'str | None'}, total=False)
class AssetOperationModel(_AssetOperationModelOptional):
    id: str
    type: AssetOperationType
    status: AssetOperationStatus
    amount: str
    fee: str
    asset: int | Decimal | str
    time: int | Decimal | str
    accountId: int | Decimal | str

_AssetOperationsParamsOptional = TypedDict("_AssetOperationsParamsOptional", {'id': 'str | None', 'type': 'str | None', 'status': 'str | None', 'startTime': 'int | Decimal | str | None', 'endTime': 'int | Decimal | str | None', 'cursor': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None'}, total=False)
class AssetOperationsParams(_AssetOperationsParamsOptional):
    pass

_AssetOperationsResultOptional = TypedDict("_AssetOperationsResultOptional", {'status': 'str | None', 'data': 'list[AssetOperationModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class AssetOperationsResult(_AssetOperationsResultOptional):
    pass

_BalanceModelOptional = TypedDict("_BalanceModelOptional", {'spotEquity': 'str | None', 'spotEquityForAvailableForTrade': 'str | None', 'collateralReservedForSpotOrders': 'str | None'}, total=False)
class BalanceModel(_BalanceModelOptional):
    collateralName: str
    balance: str
    equity: str
    availableForTrade: str
    availableForWithdrawal: str
    unrealisedPnl: str
    initialMargin: str
    marginRatio: str
    updatedTime: int | Decimal | str

class BridgesConfigModel(TypedDict):
    chains: list[ChainConfigModel]

_BuilderTradeModelOptional = TypedDict("_BuilderTradeModelOptional", {'makerId': 'int | Decimal | str | None', 'takerId': 'int | Decimal | str | None', 'makerBuilderId': 'int | Decimal | str | None', 'takerBuilderId': 'int | Decimal | str | None', 'makerFee': 'str | None', 'takerFee': 'str | None', 'makerBuilderFee': 'str | None', 'takerBuilderFee': 'str | None'}, total=False)
class BuilderTradeModel(_BuilderTradeModelOptional):
    id: int | Decimal | str
    time: int | Decimal | str
    volume: str

_CancelOrderByExternalIdParamsOptional = TypedDict("_CancelOrderByExternalIdParamsOptional", {'externalId': 'str | None'}, total=False)
class CancelOrderByExternalIdParams(_CancelOrderByExternalIdParamsOptional):
    pass

_CancelOrderByExternalIdResultOptional = TypedDict("_CancelOrderByExternalIdResultOptional", {'status': 'str | None', 'data': 'EmptyModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class CancelOrderByExternalIdResult(_CancelOrderByExternalIdResultOptional):
    pass

class CancelOrderParams(TypedDict):
    id: str

_CandleModelOptional = TypedDict("_CandleModelOptional", {'open': 'str | None', 'low': 'str | None', 'high': 'str | None', 'close': 'str | None', 'volume': 'str | None', 'timestamp': 'int | Decimal | str | None'}, total=False)
class CandleModel(_CandleModelOptional):
    pass

class ChainConfigModel(TypedDict):
    chain: str
    contractAddress: str

class ClaimResponseModel(TypedDict):
    id: int | Decimal | str

class ClaimTestingFundsParams(TypedDict):
    body: Any

_ClaimTestingFundsResultOptional = TypedDict("_ClaimTestingFundsResultOptional", {'status': 'str | None', 'data': 'ClaimResponseModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class ClaimTestingFundsResult(_ClaimTestingFundsResultOptional):
    pass

_ClientModelOptional = TypedDict("_ClientModelOptional", {'evmWalletAddress': 'Any | None', 'starknetWalletAddress': 'Any | None', 'referralLinkCode': 'Any | None'}, total=False)
class ClientModel(_ClientModelOptional):
    id: int | Decimal | str

_CollateralSlowWithdrawalOptional = TypedDict("_CollateralSlowWithdrawalOptional", {'positionId': 'int | Decimal | None'}, total=False)
class CollateralSlowWithdrawal(_CollateralSlowWithdrawalOptional):
    amount: str
    asset: str

_CommitBridgeQuoteParamsOptional = TypedDict("_CommitBridgeQuoteParamsOptional", {'id': 'str | None'}, total=False)
class CommitBridgeQuoteParams(_CommitBridgeQuoteParamsOptional):
    pass

_CommitBridgeQuoteResultOptional = TypedDict("_CommitBridgeQuoteResultOptional", {'status': 'str | None', 'data': 'EmptyModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class CommitBridgeQuoteResult(_CommitBridgeQuoteResultOptional):
    pass

_CreateApiKeyParamsWireKeys = TypedDict("_CreateApiKeyParamsWireKeys", {'X-X10-ACTIVE-ACCOUNT': 'str'})
class CreateApiKeyParams(_CreateApiKeyParamsWireKeys):
    L1_SIGNATURE: str
    L1_MESSAGE_TIME: str
    body: CreateApiKeyRequest

class CreateApiKeyRequest(TypedDict):
    description: str

_CreateApiKeyResultOptional = TypedDict("_CreateApiKeyResultOptional", {'status': 'str | None', 'data': 'ApiKeyResponseModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class CreateApiKeyResult(_CreateApiKeyResultOptional):
    pass

class CreateDepositParams(TypedDict):
    body: CreateDepositRequest

_CreateDepositRequestOptional = TypedDict("_CreateDepositRequestOptional", {'snTransactionHash': 'str | None', 'fee': 'str | None', 'bridgeId': 'str | None'}, total=False)
class CreateDepositRequest(_CreateDepositRequestOptional):
    accountId: int | Decimal
    assetId: str
    amount: str
    sourceChain: str

_CreateOrderRequestOptional = TypedDict("_CreateOrderRequestOptional", {'id': 'int | Decimal | None', 'price': 'str | None', 'expiryEpochMillis': 'int | Decimal | None', 'nonce': 'int | Decimal', 'reduceOnly': 'bool | None', 'postOnly': 'bool | None', 'trigger': 'OrderConditionalTrigger | None', 'tpSlType': 'OrderTpslType | None', 'twap': 'Twap | None'}, total=False)
class CreateOrderRequest(_CreateOrderRequestOptional):
    market: str
    type: OrderType
    side: OrderSide
    qty: str
    timeInForce: OrderTimeInForce
    fee: str

_CreateRFQOrderRequestOptional = TypedDict("_CreateRFQOrderRequestOptional", {'id': 'int | Decimal | None', 'price': 'str | None', 'expiryEpochMillis': 'int | Decimal | None', 'nonce': 'int | Decimal', 'reduceOnly': 'bool | None', 'postOnly': 'bool | None', 'trigger': 'OrderConditionalTrigger | None', 'tpSlType': 'OrderTpslType | None', 'twap': 'Twap | None', 'rfqStartPrice': 'str | None'}, total=False)
class CreateRFQOrderRequest(_CreateRFQOrderRequestOptional):
    market: str
    type: OrderType
    side: OrderSide
    qty: str
    timeInForce: OrderTimeInForce
    fee: str

_CreateReferralLinkCodeRequestOptional = TypedDict("_CreateReferralLinkCodeRequestOptional", {'description': 'str | None'}, total=False)
class CreateReferralLinkCodeRequest(_CreateReferralLinkCodeRequestOptional):
    code: str

class CreateReferralLinkParams(TypedDict):
    body: CreateReferralLinkCodeRequest

class CreateScaledOrderParams(TypedDict):
    body: CreateScaledOrderRequest

_CreateScaledOrderRequestOptional = TypedDict("_CreateScaledOrderRequestOptional", {'expiryEpochMillis': 'int | Decimal | None', 'reduceOnly': 'bool | None'}, total=False)
class CreateScaledOrderRequest(_CreateScaledOrderRequestOptional):
    market: str
    side: OrderSide
    totalQty: str
    numOrders: int | Decimal
    priceMin: str
    priceMax: str
    timeInForce: OrderTimeInForce

class DeleteApiKeyParams(TypedDict):
    id: str

class DeleteUserOrderOrderIdParams(TypedDict):
    order_id: str

_DeleteUserOrderOrderIdResultOptional = TypedDict("_DeleteUserOrderOrderIdResultOptional", {'status': 'str | None', 'data': 'EmptyModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class DeleteUserOrderOrderIdResult(_DeleteUserOrderOrderIdResultOptional):
    pass

class DepositRequestModel(TypedDict):
    fromAccountId: int | Decimal | str
    toAccountId: int | Decimal | str
    collateral: str
    shares: str
    settlement: LimitOrderSettlementModel

class DepositToVaultParams(TypedDict):
    body: DepositRequestModel

_DepositToVaultResultOptional = TypedDict("_DepositToVaultResultOptional", {'status': 'str | None', 'data': 'Any | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class DepositToVaultResult(_DepositToVaultResultOptional):
    pass

class EmptyModel(TypedDict):
    pass

_ExportCsvZipParamsOptional = TypedDict("_ExportCsvZipParamsOptional", {'accountId': 'int | Decimal | None'}, total=False)
_ExportCsvZipParamsWireKeys = TypedDict("_ExportCsvZipParamsWireKeys", {'type[]': 'list[ExportType]'})
class ExportCsvZipParams(_ExportCsvZipParamsWireKeys, _ExportCsvZipParamsOptional):
    pass

class FundingRateConfigRequest(TypedDict):
    market: str
    enabled: bool

_FundingRateModelOptional = TypedDict("_FundingRateModelOptional", {'market': 'str | None', 'fundingRate': 'str | None', 'timestamp': 'int | Decimal | str | None'}, total=False)
class FundingRateModel(_FundingRateModelOptional):
    pass

class GetAccountEquityHistoryParams(TypedDict):
    accountId: int | Decimal
    interval: str

_GetAccountEquityHistoryResultOptional = TypedDict("_GetAccountEquityHistoryResultOptional", {'status': 'str | None', 'data': 'list[GetAccountEquityHistoryResultDataItem] | None'}, total=False)
class GetAccountEquityHistoryResult(_GetAccountEquityHistoryResultOptional):
    pass

_GetAccountEquityHistoryResultDataItemOptional = TypedDict("_GetAccountEquityHistoryResultDataItemOptional", {'date': 'str | None', 'value': 'str | None'}, total=False)
class GetAccountEquityHistoryResultDataItem(_GetAccountEquityHistoryResultDataItemOptional):
    pass

_GetAccountFundingChartParamsOptional = TypedDict("_GetAccountFundingChartParamsOptional", {'market': 'list[str] | None'}, total=False)
class GetAccountFundingChartParams(_GetAccountFundingChartParamsOptional):
    accountId: list[int | Decimal]
    interval: str

_GetAccountFundingChartResultOptional = TypedDict("_GetAccountFundingChartResultOptional", {'status': 'str | None', 'data': 'list[GetAccountFundingChartResultDataItem] | None'}, total=False)
class GetAccountFundingChartResult(_GetAccountFundingChartResultOptional):
    pass

_GetAccountFundingChartResultDataItemOptional = TypedDict("_GetAccountFundingChartResultDataItemOptional", {'date': 'str | None', 'value': 'str | None'}, total=False)
class GetAccountFundingChartResultDataItem(_GetAccountFundingChartResultDataItemOptional):
    pass

_GetAccountFundingHistoryParamsOptional = TypedDict("_GetAccountFundingHistoryParamsOptional", {'market': 'list[str] | None', 'cursor': 'int | Decimal | None', 'limit': 'int | Decimal | None'}, total=False)
class GetAccountFundingHistoryParams(_GetAccountFundingHistoryParamsOptional):
    accountId: list[int | Decimal]
    interval: str

_GetAccountFundingHistoryResultOptional = TypedDict("_GetAccountFundingHistoryResultOptional", {'status': 'str | None', 'data': 'list[GetAccountFundingHistoryResultDataItem] | None', 'pagination': 'GetAccountFundingHistoryResultPagination | None'}, total=False)
class GetAccountFundingHistoryResult(_GetAccountFundingHistoryResultOptional):
    pass

_GetAccountFundingHistoryResultDataItemOptional = TypedDict("_GetAccountFundingHistoryResultDataItemOptional", {'id': 'int | Decimal | None', 'accountId': 'int | Decimal | None', 'market': 'str | None', 'side': 'str | None', 'size': 'str | None', 'value': 'str | None', 'markPrice': 'str | None', 'fundingFee': 'str | None', 'fundingRate': 'str | None', 'paidTime': 'int | Decimal | None'}, total=False)
class GetAccountFundingHistoryResultDataItem(_GetAccountFundingHistoryResultDataItemOptional):
    pass

_GetAccountFundingHistoryResultPaginationOptional = TypedDict("_GetAccountFundingHistoryResultPaginationOptional", {'cursor': 'int | Decimal | None', 'count': 'int | Decimal | None'}, total=False)
class GetAccountFundingHistoryResultPagination(_GetAccountFundingHistoryResultPaginationOptional):
    pass

_GetAccountFundingStatsParamsOptional = TypedDict("_GetAccountFundingStatsParamsOptional", {'market': 'list[str] | None'}, total=False)
class GetAccountFundingStatsParams(_GetAccountFundingStatsParamsOptional):
    accountId: list[int | Decimal]
    interval: str

_GetAccountFundingStatsResultOptional = TypedDict("_GetAccountFundingStatsResultOptional", {'status': 'str | None', 'data': 'GetAccountFundingStatsResultData | None'}, total=False)
class GetAccountFundingStatsResult(_GetAccountFundingStatsResultOptional):
    pass

_GetAccountFundingStatsResultDataOptional = TypedDict("_GetAccountFundingStatsResultDataOptional", {'totalFunding': 'str | None', 'fundingPerMarket': 'list[GetAccountFundingStatsResultDataFundingPerMarketItem] | None'}, total=False)
class GetAccountFundingStatsResultData(_GetAccountFundingStatsResultDataOptional):
    pass

_GetAccountFundingStatsResultDataFundingPerMarketItemOptional = TypedDict("_GetAccountFundingStatsResultDataFundingPerMarketItemOptional", {'market': 'str | None', 'netFundingFees': 'str | None', 'netExposure': 'str | None', 'fundingRate': 'str | None'}, total=False)
class GetAccountFundingStatsResultDataFundingPerMarketItem(_GetAccountFundingStatsResultDataFundingPerMarketItemOptional):
    pass

class GetAccountHealthParams(TypedDict):
    accountId: list[int | Decimal]

_GetAccountHealthResultOptional = TypedDict("_GetAccountHealthResultOptional", {'status': 'str | None', 'data': 'list[GetAccountHealthResultDataItem] | None'}, total=False)
class GetAccountHealthResult(_GetAccountHealthResultOptional):
    pass

_GetAccountHealthResultDataItemOptional = TypedDict("_GetAccountHealthResultDataItemOptional", {'accountId': 'int | Decimal | None', 'equity': 'str | None', 'unrealisedPnl': 'str | None', 'exposure': 'str | None', 'netExposure': 'str | None', 'leverage': 'str | None', 'initialMargin': 'str | None', 'marginRatio': 'str | None', 'availableForTrading': 'str | None', 'availableForSpotTrading': 'str | None', 'availableForWithdrawal': 'str | None', 'health': 'str | None'}, total=False)
class GetAccountHealthResultDataItem(_GetAccountHealthResultDataItemOptional):
    pass

class GetAccountMaxDrawdownHistoryParams(TypedDict):
    accountId: list[int | Decimal]
    interval: str

_GetAccountMaxDrawdownHistoryResultOptional = TypedDict("_GetAccountMaxDrawdownHistoryResultOptional", {'status': 'str | None', 'data': 'list[GetAccountMaxDrawdownHistoryResultDataItem] | None'}, total=False)
class GetAccountMaxDrawdownHistoryResult(_GetAccountMaxDrawdownHistoryResultOptional):
    pass

_GetAccountMaxDrawdownHistoryResultDataItemOptional = TypedDict("_GetAccountMaxDrawdownHistoryResultDataItemOptional", {'accountId': 'int | Decimal | None', 'data': 'list[GetAccountMaxDrawdownHistoryResultDataItemDataItem] | None'}, total=False)
class GetAccountMaxDrawdownHistoryResultDataItem(_GetAccountMaxDrawdownHistoryResultDataItemOptional):
    pass

_GetAccountMaxDrawdownHistoryResultDataItemDataItemOptional = TypedDict("_GetAccountMaxDrawdownHistoryResultDataItemDataItemOptional", {'date': 'str | None', 'value': 'str | None'}, total=False)
class GetAccountMaxDrawdownHistoryResultDataItemDataItem(_GetAccountMaxDrawdownHistoryResultDataItemDataItemOptional):
    pass

class GetAccountParams(TypedDict):
    pass

_GetAccountPerformanceParamsOptional = TypedDict("_GetAccountPerformanceParamsOptional", {'marketType': 'str | None'}, total=False)
class GetAccountPerformanceParams(_GetAccountPerformanceParamsOptional):
    accountId: list[int | Decimal]
    interval: str

_GetAccountPerformanceResultOptional = TypedDict("_GetAccountPerformanceResultOptional", {'status': 'str | None', 'data': 'list[GetAccountPerformanceResultDataItem] | None'}, total=False)
class GetAccountPerformanceResult(_GetAccountPerformanceResultOptional):
    pass

_GetAccountPerformanceResultDataItemOptional = TypedDict("_GetAccountPerformanceResultDataItemOptional", {'accountId': 'int | Decimal | None', 'realisedPnl': 'str | None', 'unrealisedPnl': 'str | None', 'totalPnl': 'str | None', 'totalPnlPercentage': 'str | None', 'profitFactor': 'str | None', 'maxDrawdown': 'str | None', 'winRate': 'str | None', 'tradingVolume': 'str | None', 'makerVolume': 'str | None', 'sharpeRatio': 'str | None'}, total=False)
class GetAccountPerformanceResultDataItem(_GetAccountPerformanceResultDataItemOptional):
    pass

_GetAccountPnLHistoryParamsOptional = TypedDict("_GetAccountPnLHistoryParamsOptional", {'instrumentType': 'str | None'}, total=False)
class GetAccountPnLHistoryParams(_GetAccountPnLHistoryParamsOptional):
    accountId: int | Decimal
    interval: str
    pnlType: str

_GetAccountPnLHistoryResultOptional = TypedDict("_GetAccountPnLHistoryResultOptional", {'status': 'str | None', 'data': 'list[GetAccountPnLHistoryResultDataItem] | None'}, total=False)
class GetAccountPnLHistoryResult(_GetAccountPnLHistoryResultOptional):
    pass

_GetAccountPnLHistoryResultDataItemOptional = TypedDict("_GetAccountPnLHistoryResultDataItemOptional", {'date': 'str | None', 'value': 'str | None'}, total=False)
class GetAccountPnLHistoryResultDataItem(_GetAccountPnLHistoryResultDataItemOptional):
    pass

_GetAccountPnLPercentageHistoryParamsOptional = TypedDict("_GetAccountPnLPercentageHistoryParamsOptional", {'priceMarket': 'list[str] | None', 'instrumentType': 'str | None'}, total=False)
class GetAccountPnLPercentageHistoryParams(_GetAccountPnLPercentageHistoryParamsOptional):
    accountId: list[int | Decimal]
    interval: str
    pnlType: str

_GetAccountPnLPercentageHistoryResultOptional = TypedDict("_GetAccountPnLPercentageHistoryResultOptional", {'status': 'str | None', 'data': 'GetAccountPnLPercentageHistoryResultData | None'}, total=False)
class GetAccountPnLPercentageHistoryResult(_GetAccountPnLPercentageHistoryResultOptional):
    pass

_GetAccountPnLPercentageHistoryResultDataOptional = TypedDict("_GetAccountPnLPercentageHistoryResultDataOptional", {'pnl': 'list[GetAccountPnLPercentageHistoryResultDataPnlItem] | None', 'priceChanges': 'GetAccountPnLPercentageHistoryResultDataPriceChanges | None'}, total=False)
class GetAccountPnLPercentageHistoryResultData(_GetAccountPnLPercentageHistoryResultDataOptional):
    pass

_GetAccountPnLPercentageHistoryResultDataPnlItemOptional = TypedDict("_GetAccountPnLPercentageHistoryResultDataPnlItemOptional", {'date': 'str | None', 'value': 'str | None'}, total=False)
class GetAccountPnLPercentageHistoryResultDataPnlItem(_GetAccountPnLPercentageHistoryResultDataPnlItemOptional):
    pass

_GetAccountPnLPercentageHistoryResultDataPriceChangesOptional = TypedDict("_GetAccountPnLPercentageHistoryResultDataPriceChangesOptional", {'BTC-USD': 'list[GetAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem] | None'}, total=False)
class GetAccountPnLPercentageHistoryResultDataPriceChanges(_GetAccountPnLPercentageHistoryResultDataPriceChangesOptional):
    pass

_GetAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItemOptional = TypedDict("_GetAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItemOptional", {'date': 'str | None', 'value': 'str | None'}, total=False)
class GetAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem(_GetAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItemOptional):
    pass

_GetAccountPortfolioSummaryParamsOptional = TypedDict("_GetAccountPortfolioSummaryParamsOptional", {'instrumentType': 'str | None'}, total=False)
class GetAccountPortfolioSummaryParams(_GetAccountPortfolioSummaryParamsOptional):
    accountId: list[int | Decimal]
    interval: str

_GetAccountPortfolioSummaryResultOptional = TypedDict("_GetAccountPortfolioSummaryResultOptional", {'status': 'str | None', 'data': 'GetAccountPortfolioSummaryResultData | None'}, total=False)
class GetAccountPortfolioSummaryResult(_GetAccountPortfolioSummaryResultOptional):
    pass

_GetAccountPortfolioSummaryResultDataOptional = TypedDict("_GetAccountPortfolioSummaryResultDataOptional", {'equity': 'str | None', 'accountValue': 'GetAccountPortfolioSummaryResultDataAccountValue | None', 'exposure': 'str | None', 'totalPnl': 'str | None', 'profitFactor': 'str | None', 'winRate': 'str | None', 'tradingVolume': 'str | None', 'makerVolume': 'str | None', 'sharpeRatio': 'str | None'}, total=False)
class GetAccountPortfolioSummaryResultData(_GetAccountPortfolioSummaryResultDataOptional):
    pass

_GetAccountPortfolioSummaryResultDataAccountValueOptional = TypedDict("_GetAccountPortfolioSummaryResultDataAccountValueOptional", {'accountNotionalValue': 'str | None', 'tradingEquity': 'str | None', 'vaultNotionalValue': 'str | None', 'spotNotionalValue': 'str | None'}, total=False)
class GetAccountPortfolioSummaryResultDataAccountValue(_GetAccountPortfolioSummaryResultDataAccountValueOptional):
    pass

_GetAccountResultOptional = TypedDict("_GetAccountResultOptional", {'status': 'str | None', 'data': 'AccountModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetAccountResult(_GetAccountResultOptional):
    pass

class GetAccountVaultEquityHistoryParams(TypedDict):
    accountId: list[int | Decimal]
    interval: str

_GetAccountVaultEquityHistoryResultOptional = TypedDict("_GetAccountVaultEquityHistoryResultOptional", {'status': 'str | None', 'data': 'list[GetAccountVaultEquityHistoryResultDataItem] | None'}, total=False)
class GetAccountVaultEquityHistoryResult(_GetAccountVaultEquityHistoryResultOptional):
    pass

_GetAccountVaultEquityHistoryResultDataItemOptional = TypedDict("_GetAccountVaultEquityHistoryResultDataItemOptional", {'date': 'str | None', 'value': 'str | None'}, total=False)
class GetAccountVaultEquityHistoryResultDataItem(_GetAccountVaultEquityHistoryResultDataItemOptional):
    pass

class GetAccountsParams(TypedDict):
    pass

_GetAccountsResultOptional = TypedDict("_GetAccountsResultOptional", {'status': 'str | None', 'data': 'list[AccountModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetAccountsResult(_GetAccountsResultOptional):
    pass

class GetAffiliateParams(TypedDict):
    pass

class GetAffiliateRewardsParams(TypedDict):
    pass

class GetApiKeysParams(TypedDict):
    pass

_GetAssetOperationsParamsOptional = TypedDict("_GetAssetOperationsParamsOptional", {'accountId[]': 'list[int | Decimal] | None', 'id': 'str | None', 'type[]': 'list[str] | None', 'status[]': 'list[str] | None', 'startTime': 'int | Decimal | None', 'endTime': 'int | Decimal | None', 'limit': 'int | Decimal | None', 'cursor': 'str | None'}, total=False)
class GetAssetOperationsParams(_GetAssetOperationsParamsOptional):
    pass

class GetAssetPriceParams(TypedDict):
    asset: str

_GetAssetsParamsOptional = TypedDict("_GetAssetsParamsOptional", {'type': 'AssetType | None', 'asset': 'list[str] | None', 'collateral': 'bool | None'}, total=False)
class GetAssetsParams(_GetAssetsParamsOptional):
    pass

_GetAssetsResultOptional = TypedDict("_GetAssetsResultOptional", {'status': 'str | None', 'data': 'list[AssetModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetAssetsResult(_GetAssetsResultOptional):
    pass

class GetBalanceParams(TypedDict):
    pass

_GetBalanceResultOptional = TypedDict("_GetBalanceResultOptional", {'status': 'str | None', 'data': 'BalanceModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetBalanceResult(_GetBalanceResultOptional):
    pass

class GetBridgeConfigParams(TypedDict):
    pass

_GetBridgeConfigResultOptional = TypedDict("_GetBridgeConfigResultOptional", {'status': 'str | None', 'data': 'BridgesConfigModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetBridgeConfigResult(_GetBridgeConfigResultOptional):
    pass

_GetBridgeQuoteParamsOptional = TypedDict("_GetBridgeQuoteParamsOptional", {'chainIn': 'str | None', 'chainOut': 'str | None', 'amount': 'str | None'}, total=False)
class GetBridgeQuoteParams(_GetBridgeQuoteParamsOptional):
    pass

_GetBridgeQuoteResultOptional = TypedDict("_GetBridgeQuoteResultOptional", {'status': 'str | None', 'data': 'QuoteModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetBridgeQuoteResult(_GetBridgeQuoteResultOptional):
    pass

class GetBuilderDashboardParams(TypedDict):
    pass

_GetBuilderTradesParamsOptional = TypedDict("_GetBuilderTradesParamsOptional", {'cursor': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None'}, total=False)
class GetBuilderTradesParams(_GetBuilderTradesParamsOptional):
    pass

_GetBuilderTradesResultOptional = TypedDict("_GetBuilderTradesResultOptional", {'status': 'str | None', 'data': 'list[BuilderTradeModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetBuilderTradesResult(_GetBuilderTradesResultOptional):
    pass

_GetCandlesHistoryParamsOptional = TypedDict("_GetCandlesHistoryParamsOptional", {'interval': 'str | None', 'limit': 'int | Decimal | str | None', 'endTime': 'str | None'}, total=False)
class GetCandlesHistoryParams(_GetCandlesHistoryParamsOptional):
    market: str
    candle_type: str

_GetCandlesHistoryResultOptional = TypedDict("_GetCandlesHistoryResultOptional", {'status': 'str | None', 'data': 'list[CandleModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetCandlesHistoryResult(_GetCandlesHistoryResultOptional):
    pass

class GetClaimParams(TypedDict):
    id: int | Decimal

_GetClaimsParamsOptional = TypedDict("_GetClaimsParamsOptional", {'startTime': 'int | Decimal | None', 'limit': 'int | Decimal | None'}, total=False)
class GetClaimsParams(_GetClaimsParamsOptional):
    pass

class GetClientParams(TypedDict):
    pass

_GetClientResultOptional = TypedDict("_GetClientResultOptional", {'status': 'str | None', 'data': 'ClientModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetClientResult(_GetClientResultOptional):
    pass

_GetCumulativeAccountPnLHistoryParamsOptional = TypedDict("_GetCumulativeAccountPnLHistoryParamsOptional", {'instrumentType': 'str | None'}, total=False)
class GetCumulativeAccountPnLHistoryParams(_GetCumulativeAccountPnLHistoryParamsOptional):
    accountId: int | Decimal
    interval: str
    pnlType: str

_GetCumulativeAccountPnLHistoryResultOptional = TypedDict("_GetCumulativeAccountPnLHistoryResultOptional", {'status': 'str | None', 'data': 'list[GetCumulativeAccountPnLHistoryResultDataItem] | None'}, total=False)
class GetCumulativeAccountPnLHistoryResult(_GetCumulativeAccountPnLHistoryResultOptional):
    pass

_GetCumulativeAccountPnLHistoryResultDataItemOptional = TypedDict("_GetCumulativeAccountPnLHistoryResultDataItemOptional", {'date': 'str | None', 'value': 'str | None'}, total=False)
class GetCumulativeAccountPnLHistoryResultDataItem(_GetCumulativeAccountPnLHistoryResultDataItemOptional):
    pass

_GetCumulativeAccountPnLPercentageHistoryParamsOptional = TypedDict("_GetCumulativeAccountPnLPercentageHistoryParamsOptional", {'priceMarket': 'list[str] | None', 'instrumentType': 'str | None'}, total=False)
class GetCumulativeAccountPnLPercentageHistoryParams(_GetCumulativeAccountPnLPercentageHistoryParamsOptional):
    accountId: list[int | Decimal]
    interval: str
    pnlType: str

_GetCumulativeAccountPnLPercentageHistoryResultOptional = TypedDict("_GetCumulativeAccountPnLPercentageHistoryResultOptional", {'status': 'str | None', 'data': 'GetCumulativeAccountPnLPercentageHistoryResultData | None'}, total=False)
class GetCumulativeAccountPnLPercentageHistoryResult(_GetCumulativeAccountPnLPercentageHistoryResultOptional):
    pass

_GetCumulativeAccountPnLPercentageHistoryResultDataOptional = TypedDict("_GetCumulativeAccountPnLPercentageHistoryResultDataOptional", {'pnl': 'list[GetCumulativeAccountPnLPercentageHistoryResultDataPnlItem] | None', 'priceChanges': 'GetCumulativeAccountPnLPercentageHistoryResultDataPriceChanges | None'}, total=False)
class GetCumulativeAccountPnLPercentageHistoryResultData(_GetCumulativeAccountPnLPercentageHistoryResultDataOptional):
    pass

_GetCumulativeAccountPnLPercentageHistoryResultDataPnlItemOptional = TypedDict("_GetCumulativeAccountPnLPercentageHistoryResultDataPnlItemOptional", {'date': 'str | None', 'value': 'str | None'}, total=False)
class GetCumulativeAccountPnLPercentageHistoryResultDataPnlItem(_GetCumulativeAccountPnLPercentageHistoryResultDataPnlItemOptional):
    pass

_GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesOptional = TypedDict("_GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesOptional", {'BTC-USD': 'list[GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem] | None'}, total=False)
class GetCumulativeAccountPnLPercentageHistoryResultDataPriceChanges(_GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesOptional):
    pass

_GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItemOptional = TypedDict("_GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItemOptional", {'date': 'str | None', 'value': 'str | None'}, total=False)
class GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem(_GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItemOptional):
    pass

class GetEarnedRewardsParams(TypedDict):
    pass

class GetExchangeStatusParams(TypedDict):
    pass

_GetFeesParamsOptional = TypedDict("_GetFeesParamsOptional", {'market': 'list[str] | None', 'builderId': 'int | Decimal | str | None'}, total=False)
class GetFeesParams(_GetFeesParamsOptional):
    pass

_GetFeesResultOptional = TypedDict("_GetFeesResultOptional", {'status': 'str | None', 'data': 'list[TradingFeeModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetFeesResult(_GetFeesResultOptional):
    pass

_GetFundingRatesHistoryParamsOptional = TypedDict("_GetFundingRatesHistoryParamsOptional", {'startTime': 'str | None', 'endTime': 'str | None', 'cursor': 'int | Decimal | None', 'limit': 'int | Decimal | None'}, total=False)
class GetFundingRatesHistoryParams(_GetFundingRatesHistoryParamsOptional):
    market: str

_GetFundingRatesHistoryResultOptional = TypedDict("_GetFundingRatesHistoryResultOptional", {'status': 'str | None', 'data': 'list[FundingRateModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetFundingRatesHistoryResult(_GetFundingRatesHistoryResultOptional):
    pass

_GetIndexPriceCandlesParamsOptional = TypedDict("_GetIndexPriceCandlesParamsOptional", {'limit': 'int | Decimal | None', 'endTime': 'int | Decimal | None'}, total=False)
class GetIndexPriceCandlesParams(_GetIndexPriceCandlesParamsOptional):
    market: str
    interval: str

class GetInfoAssetsAssetNamePriceParams(TypedDict):
    asset_name: str

_GetInfoAssetsAssetNamePriceResultOptional = TypedDict("_GetInfoAssetsAssetNamePriceResultOptional", {'status': 'str | None', 'data': 'str | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetInfoAssetsAssetNamePriceResult(_GetInfoAssetsAssetNamePriceResultOptional):
    pass

_GetInfoCandlesMarketCandleTypeParamsOptional = TypedDict("_GetInfoCandlesMarketCandleTypeParamsOptional", {'endTime': 'int | Decimal | None'}, total=False)
class GetInfoCandlesMarketCandleTypeParams(_GetInfoCandlesMarketCandleTypeParamsOptional):
    market: str
    candleType: str
    interval: str
    limit: int | Decimal

_GetInfoCandlesMarketCandleTypeResultOptional = TypedDict("_GetInfoCandlesMarketCandleTypeResultOptional", {'status': 'str | None', 'data': 'list[GetInfoCandlesMarketCandleTypeResultDataItem] | None'}, total=False)
class GetInfoCandlesMarketCandleTypeResult(_GetInfoCandlesMarketCandleTypeResultOptional):
    pass

_GetInfoCandlesMarketCandleTypeResultDataItemOptional = TypedDict("_GetInfoCandlesMarketCandleTypeResultDataItemOptional", {'o': 'str | None', 'l': 'str | None', 'h': 'str | None', 'c': 'str | None', 'v': 'str | None', 'T': 'int | Decimal | None'}, total=False)
class GetInfoCandlesMarketCandleTypeResultDataItem(_GetInfoCandlesMarketCandleTypeResultDataItemOptional):
    pass

class GetInterestDailyMetricsParams(TypedDict):
    accountId: list[int | Decimal]
    interval: str

_GetInterestDailyMetricsResultOptional = TypedDict("_GetInterestDailyMetricsResultOptional", {'status': 'str | None', 'data': 'GetInterestDailyMetricsResultData | None'}, total=False)
class GetInterestDailyMetricsResult(_GetInterestDailyMetricsResultOptional):
    pass

_GetInterestDailyMetricsResultDataOptional = TypedDict("_GetInterestDailyMetricsResultDataOptional", {'1000': 'list[GetInterestDailyMetricsResultData1000Item] | None'}, total=False)
class GetInterestDailyMetricsResultData(_GetInterestDailyMetricsResultDataOptional):
    pass

_GetInterestDailyMetricsResultData1000ItemOptional = TypedDict("_GetInterestDailyMetricsResultData1000ItemOptional", {'date': 'str | None', 'assetId': 'int | Decimal | None', 'interestPaid': 'str | None', 'averageBorrowed': 'str | None', 'annualizedInterestRate': 'str | None'}, total=False)
class GetInterestDailyMetricsResultData1000Item(_GetInterestDailyMetricsResultData1000ItemOptional):
    pass

class GetInterestKeyMetricsParams(TypedDict):
    accountId: list[int | Decimal]

_GetInterestKeyMetricsResultOptional = TypedDict("_GetInterestKeyMetricsResultOptional", {'status': 'str | None', 'data': 'GetInterestKeyMetricsResultData | None'}, total=False)
class GetInterestKeyMetricsResult(_GetInterestKeyMetricsResultOptional):
    pass

_GetInterestKeyMetricsResultDataOptional = TypedDict("_GetInterestKeyMetricsResultDataOptional", {'borrowed': 'str | None', 'currentInterestRate': 'str | None', 'estimatedNextPayment': 'str | None'}, total=False)
class GetInterestKeyMetricsResultData(_GetInterestKeyMetricsResultDataOptional):
    pass

_GetInterestPaymentChartParamsOptional = TypedDict("_GetInterestPaymentChartParamsOptional", {'bucket': 'str | None'}, total=False)
class GetInterestPaymentChartParams(_GetInterestPaymentChartParamsOptional):
    accountId: list[int | Decimal]
    interval: str

_GetInterestPaymentChartResultOptional = TypedDict("_GetInterestPaymentChartResultOptional", {'status': 'str | None', 'data': 'GetInterestPaymentChartResultData | None'}, total=False)
class GetInterestPaymentChartResult(_GetInterestPaymentChartResultOptional):
    pass

_GetInterestPaymentChartResultDataOptional = TypedDict("_GetInterestPaymentChartResultDataOptional", {'resolution': 'str | None', 'points': 'list[GetInterestPaymentChartResultDataPointsItem] | None'}, total=False)
class GetInterestPaymentChartResultData(_GetInterestPaymentChartResultDataOptional):
    pass

_GetInterestPaymentChartResultDataPointsItemOptional = TypedDict("_GetInterestPaymentChartResultDataPointsItemOptional", {'bucketStartTimestamp': 'int | Decimal | None', 'totalInterestPaid': 'str | None', 'averageBorrowed': 'str | None', 'averageApr': 'str | None', 'borrowingByCollateralAsset': 'list[GetInterestPaymentChartResultDataPointsItemBorrowingByCollateralAssetItem] | None'}, total=False)
class GetInterestPaymentChartResultDataPointsItem(_GetInterestPaymentChartResultDataPointsItemOptional):
    pass

_GetInterestPaymentChartResultDataPointsItemBorrowingByCollateralAssetItemOptional = TypedDict("_GetInterestPaymentChartResultDataPointsItemBorrowingByCollateralAssetItemOptional", {'assetId': 'int | Decimal | None', 'averageBorrowed': 'str | None', 'averageApr': 'str | None'}, total=False)
class GetInterestPaymentChartResultDataPointsItemBorrowingByCollateralAssetItem(_GetInterestPaymentChartResultDataPointsItemBorrowingByCollateralAssetItemOptional):
    pass

class GetInterestPaymentsHistoryParams(TypedDict):
    accountId: list[int | Decimal]
    interval: str

_GetInterestPaymentsHistoryResultOptional = TypedDict("_GetInterestPaymentsHistoryResultOptional", {'status': 'str | None', 'data': 'GetInterestPaymentsHistoryResultData | None'}, total=False)
class GetInterestPaymentsHistoryResult(_GetInterestPaymentsHistoryResultOptional):
    pass

_GetInterestPaymentsHistoryResultDataOptional = TypedDict("_GetInterestPaymentsHistoryResultDataOptional", {'items': 'list[GetInterestPaymentsHistoryResultDataItemsItem] | None'}, total=False)
class GetInterestPaymentsHistoryResultData(_GetInterestPaymentsHistoryResultDataOptional):
    pass

_GetInterestPaymentsHistoryResultDataItemsItemOptional = TypedDict("_GetInterestPaymentsHistoryResultDataItemsItemOptional", {'timestamp': 'int | Decimal | None', 'paymentAmount': 'str | None', 'borrowedAmount': 'str | None', 'effectiveOneHourRate': 'str | None', 'accountId': 'int | Decimal | None'}, total=False)
class GetInterestPaymentsHistoryResultDataItemsItem(_GetInterestPaymentsHistoryResultDataItemsItemOptional):
    pass

class GetInterestRateCurvesHistoryParams(TypedDict):
    interval: str

_GetInterestRateCurvesHistoryResultOptional = TypedDict("_GetInterestRateCurvesHistoryResultOptional", {'status': 'str | None', 'data': 'GetInterestRateCurvesHistoryResultData | None'}, total=False)
class GetInterestRateCurvesHistoryResult(_GetInterestRateCurvesHistoryResultOptional):
    pass

_GetInterestRateCurvesHistoryResultDataOptional = TypedDict("_GetInterestRateCurvesHistoryResultDataOptional", {'items': 'list[GetInterestRateCurvesHistoryResultDataItemsItem] | None'}, total=False)
class GetInterestRateCurvesHistoryResultData(_GetInterestRateCurvesHistoryResultDataOptional):
    pass

_GetInterestRateCurvesHistoryResultDataItemsItemOptional = TypedDict("_GetInterestRateCurvesHistoryResultDataItemsItemOptional", {'timestamp': 'int | Decimal | None', 'amountBorrowed': 'str | None', 'globalCurve': 'GetInterestRateCurvesHistoryResultDataItemsItemGlobalCurve | None', 'assetCurves': 'list[GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItem] | None'}, total=False)
class GetInterestRateCurvesHistoryResultDataItemsItem(_GetInterestRateCurvesHistoryResultDataItemsItemOptional):
    pass

_GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemOptional = TypedDict("_GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemOptional", {'assetId': 'int | Decimal | None', 'curve': 'GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemCurve | None', 'calculatedRate': 'str | None', 'amountBorrowed': 'str | None'}, total=False)
class GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItem(_GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemOptional):
    pass

_GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemCurveOptional = TypedDict("_GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemCurveOptional", {'currentUtilization': 'str | None', 'currentRate': 'str | None', 'targetUtilization': 'str | None', 'minRate': 'str | None', 'targetRate': 'str | None', 'maxRate': 'str | None'}, total=False)
class GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemCurve(_GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemCurveOptional):
    pass

_GetInterestRateCurvesHistoryResultDataItemsItemGlobalCurveOptional = TypedDict("_GetInterestRateCurvesHistoryResultDataItemsItemGlobalCurveOptional", {'currentUtilization': 'str | None', 'currentRate': 'str | None', 'targetUtilization': 'str | None', 'minRate': 'str | None', 'targetRate': 'str | None', 'maxRate': 'str | None'}, total=False)
class GetInterestRateCurvesHistoryResultDataItemsItemGlobalCurve(_GetInterestRateCurvesHistoryResultDataItemsItemGlobalCurveOptional):
    pass

class GetLatestInterestRateCurveParams(TypedDict):
    pass

_GetLatestInterestRateCurveResultOptional = TypedDict("_GetLatestInterestRateCurveResultOptional", {'status': 'str | None', 'data': 'GetLatestInterestRateCurveResultData | None'}, total=False)
class GetLatestInterestRateCurveResult(_GetLatestInterestRateCurveResultOptional):
    pass

_GetLatestInterestRateCurveResultDataOptional = TypedDict("_GetLatestInterestRateCurveResultDataOptional", {'item': 'GetLatestInterestRateCurveResultDataItem | None'}, total=False)
class GetLatestInterestRateCurveResultData(_GetLatestInterestRateCurveResultDataOptional):
    pass

_GetLatestInterestRateCurveResultDataItemOptional = TypedDict("_GetLatestInterestRateCurveResultDataItemOptional", {'timestamp': 'int | Decimal | None', 'amountBorrowed': 'str | None', 'globalCurve': 'GetLatestInterestRateCurveResultDataItemGlobalCurve | None', 'assetCurves': 'list[GetLatestInterestRateCurveResultDataItemAssetCurvesItem] | None'}, total=False)
class GetLatestInterestRateCurveResultDataItem(_GetLatestInterestRateCurveResultDataItemOptional):
    pass

_GetLatestInterestRateCurveResultDataItemAssetCurvesItemOptional = TypedDict("_GetLatestInterestRateCurveResultDataItemAssetCurvesItemOptional", {'assetId': 'int | Decimal | None', 'curve': 'GetLatestInterestRateCurveResultDataItemAssetCurvesItemCurve | None', 'calculatedRate': 'str | None', 'amountBorrowed': 'str | None'}, total=False)
class GetLatestInterestRateCurveResultDataItemAssetCurvesItem(_GetLatestInterestRateCurveResultDataItemAssetCurvesItemOptional):
    pass

_GetLatestInterestRateCurveResultDataItemAssetCurvesItemCurveOptional = TypedDict("_GetLatestInterestRateCurveResultDataItemAssetCurvesItemCurveOptional", {'currentUtilization': 'str | None', 'currentRate': 'str | None', 'targetUtilization': 'str | None', 'minRate': 'str | None', 'targetRate': 'str | None', 'maxRate': 'str | None'}, total=False)
class GetLatestInterestRateCurveResultDataItemAssetCurvesItemCurve(_GetLatestInterestRateCurveResultDataItemAssetCurvesItemCurveOptional):
    pass

_GetLatestInterestRateCurveResultDataItemGlobalCurveOptional = TypedDict("_GetLatestInterestRateCurveResultDataItemGlobalCurveOptional", {'currentUtilization': 'str | None', 'currentRate': 'str | None', 'targetUtilization': 'str | None', 'minRate': 'str | None', 'targetRate': 'str | None', 'maxRate': 'str | None'}, total=False)
class GetLatestInterestRateCurveResultDataItemGlobalCurve(_GetLatestInterestRateCurveResultDataItemGlobalCurveOptional):
    pass

_GetLeverageParamsOptional = TypedDict("_GetLeverageParamsOptional", {'market': 'list[str] | None'}, total=False)
class GetLeverageParams(_GetLeverageParamsOptional):
    pass

_GetLeverageResultOptional = TypedDict("_GetLeverageResultOptional", {'status': 'str | None', 'data': 'list[AccountLeverageModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetLeverageResult(_GetLeverageResultOptional):
    pass

_GetMarkPriceCandlesParamsOptional = TypedDict("_GetMarkPriceCandlesParamsOptional", {'limit': 'int | Decimal | None', 'endTime': 'int | Decimal | None'}, total=False)
class GetMarkPriceCandlesParams(_GetMarkPriceCandlesParamsOptional):
    market: str
    interval: str

class GetMarketStatisticsParams(TypedDict):
    market: str

_GetMarketStatisticsResultOptional = TypedDict("_GetMarketStatisticsResultOptional", {'status': 'str | None', 'data': 'MarketStatsModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetMarketStatisticsResult(_GetMarketStatisticsResultOptional):
    pass

class GetMarketTradesParams(TypedDict):
    market: str

_GetMarketsParamsOptional = TypedDict("_GetMarketsParamsOptional", {'market': 'list[str] | None'}, total=False)
class GetMarketsParams(_GetMarketsParamsOptional):
    pass

_GetMarketsResultOptional = TypedDict("_GetMarketsResultOptional", {'status': 'str | None', 'data': 'list[MarketModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetMarketsResult(_GetMarketsResultOptional):
    pass

_GetMarketsStatsParamsOptional = TypedDict("_GetMarketsStatsParamsOptional", {'market[]': 'list[str] | None'}, total=False)
class GetMarketsStatsParams(_GetMarketsStatsParamsOptional):
    pass

class GetNextClaimTimeParams(TypedDict):
    pass

_GetOpenInterestsParamsOptional = TypedDict("_GetOpenInterestsParamsOptional", {'interval': 'str | None', 'startTime': 'int | Decimal | None', 'endTime': 'int | Decimal | None', 'limit': 'int | Decimal | None'}, total=False)
class GetOpenInterestsParams(_GetOpenInterestsParamsOptional):
    market: str

_GetOpenOrdersParamsOptional = TypedDict("_GetOpenOrdersParamsOptional", {'market': 'list[str] | None', 'type': 'OrderType | None', 'side': 'OrderSide | None'}, total=False)
class GetOpenOrdersParams(_GetOpenOrdersParamsOptional):
    pass

_GetOpenOrdersResultOptional = TypedDict("_GetOpenOrdersResultOptional", {'status': 'str | None', 'data': 'list[OpenOrderModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetOpenOrdersResult(_GetOpenOrdersResultOptional):
    pass

class GetOrderByExternalIdParams(TypedDict):
    externalId: str

class GetOrderByIdParams(TypedDict):
    order_id: str

_GetOrderByIdResultOptional = TypedDict("_GetOrderByIdResultOptional", {'status': 'str | None', 'data': 'OpenOrderModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetOrderByIdResult(_GetOrderByIdResultOptional):
    pass

class GetOrderParams(TypedDict):
    id: str

class GetOrderbookSnapshotParams(TypedDict):
    market: str

_GetOrderbookSnapshotResultOptional = TypedDict("_GetOrderbookSnapshotResultOptional", {'status': 'str | None', 'data': 'OrderbookUpdateModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetOrderbookSnapshotResult(_GetOrderbookSnapshotResultOptional):
    pass

_GetOrdersHistoryParamsOptional = TypedDict("_GetOrdersHistoryParamsOptional", {'market': 'list[str] | None', 'type': 'OrderType | None', 'side': 'OrderSide | None', 'cursor': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None', 'sort': 'OrderSortBy | None', 'id': 'int | Decimal | None', 'externalId': 'str | None'}, total=False)
class GetOrdersHistoryParams(_GetOrdersHistoryParamsOptional):
    pass

_GetOrdersHistoryResultOptional = TypedDict("_GetOrdersHistoryResultOptional", {'status': 'str | None', 'data': 'list[OpenOrderModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetOrdersHistoryResult(_GetOrdersHistoryResultOptional):
    pass

class GetPartnerRewardsParams(TypedDict):
    pass

_GetPositionsHistoryParamsOptional = TypedDict("_GetPositionsHistoryParamsOptional", {'market': 'list[str] | None', 'side': 'PositionSide | None', 'cursor': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None'}, total=False)
class GetPositionsHistoryParams(_GetPositionsHistoryParamsOptional):
    pass

_GetPositionsHistoryResultOptional = TypedDict("_GetPositionsHistoryResultOptional", {'status': 'str | None', 'data': 'list[PositionHistoryModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetPositionsHistoryResult(_GetPositionsHistoryResultOptional):
    pass

_GetPositionsParamsOptional = TypedDict("_GetPositionsParamsOptional", {'market': 'list[str] | None', 'side': 'PositionSide | None'}, total=False)
class GetPositionsParams(_GetPositionsParamsOptional):
    pass

_GetPositionsResultOptional = TypedDict("_GetPositionsResultOptional", {'status': 'str | None', 'data': 'list[PositionModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetPositionsResult(_GetPositionsResultOptional):
    pass

class GetPublicLeaderboardParams(TypedDict):
    period: LeaderboardInterval

class GetPublicReferralLinkParams(TypedDict):
    id: str

class GetRebateStatsParams(TypedDict):
    pass

class GetRebatesCsvParams(TypedDict):
    pass

class GetReferralDashboardParams(TypedDict):
    period: ReferralDashboardTimeRange

class GetReferralLinksParams(TypedDict):
    pass

_GetReferralRebatesByLinkCsvParamsOptional = TypedDict("_GetReferralRebatesByLinkCsvParamsOptional", {'links[]': 'list[str] | None'}, total=False)
class GetReferralRebatesByLinkCsvParams(_GetReferralRebatesByLinkCsvParamsOptional):
    period: ReferralDashboardTimeRange

_GetReferralRebatesByPeriodCsvParamsOptional = TypedDict("_GetReferralRebatesByPeriodCsvParamsOptional", {'links[]': 'list[str] | None', 'subaffiliates[]': 'list[str] | None'}, total=False)
class GetReferralRebatesByPeriodCsvParams(_GetReferralRebatesByPeriodCsvParamsOptional):
    period: ReferralDashboardTimeRange
    granularity: ReferralDashboardGranularity

_GetReferralRebatesBySubaffiliateCsvParamsOptional = TypedDict("_GetReferralRebatesBySubaffiliateCsvParamsOptional", {'subaffiliates[]': 'list[str] | None'}, total=False)
class GetReferralRebatesBySubaffiliateCsvParams(_GetReferralRebatesBySubaffiliateCsvParamsOptional):
    period: ReferralDashboardTimeRange

_GetReferralRebatesByUserCsvParamsOptional = TypedDict("_GetReferralRebatesByUserCsvParamsOptional", {'links[]': 'list[str] | None', 'subaffiliates[]': 'list[str] | None'}, total=False)
class GetReferralRebatesByUserCsvParams(_GetReferralRebatesByUserCsvParamsOptional):
    period: ReferralDashboardTimeRange

class GetReferralStatusParams(TypedDict):
    pass

class GetRewardLeaderboardStatsParams(TypedDict):
    pass

class GetSettingsParams(TypedDict):
    pass

_GetSettingsResultOptional = TypedDict("_GetSettingsResultOptional", {'status': 'str | None', 'data': 'SettingsModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetSettingsResult(_GetSettingsResultOptional):
    pass

_GetSpotBalancesParamsOptional = TypedDict("_GetSpotBalancesParamsOptional", {'accountId[]': 'list[int | Decimal] | None', 'accountId': 'int | Decimal | None'}, total=False)
class GetSpotBalancesParams(_GetSpotBalancesParamsOptional):
    pass

_GetSpotBalancesResultOptional = TypedDict("_GetSpotBalancesResultOptional", {'status': 'str | None', 'data': 'list[SpotBalanceModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetSpotBalancesResult(_GetSpotBalancesResultOptional):
    pass

class GetStarknetDomainParams(TypedDict):
    pass

_GetTradesParamsOptional = TypedDict("_GetTradesParamsOptional", {'market': 'list[str] | None', 'side': 'OrderSide | None', 'type': 'TradeType | None', 'cursor': 'int | Decimal | str | None', 'limit': 'int | Decimal | str | None'}, total=False)
class GetTradesParams(_GetTradesParamsOptional):
    pass

_GetTradesResultOptional = TypedDict("_GetTradesResultOptional", {'status': 'str | None', 'data': 'list[AccountTradeModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetTradesResult(_GetTradesResultOptional):
    pass

_GetTradingCandlesParamsOptional = TypedDict("_GetTradingCandlesParamsOptional", {'limit': 'int | Decimal | None', 'endTime': 'int | Decimal | None'}, total=False)
class GetTradingCandlesParams(_GetTradingCandlesParamsOptional):
    market: str
    interval: str

class GetTradingFeeStatsParams(TypedDict):
    pass

_GetUserFundingHistoryParamsOptional = TypedDict("_GetUserFundingHistoryParamsOptional", {'market[]': 'list[str] | None', 'side': 'PositionSide | None', 'startTime': 'int | Decimal | None', 'cursor': 'str | None', 'limit': 'int | Decimal | None', 'market': 'str | None'}, total=False)
class GetUserFundingHistoryParams(_GetUserFundingHistoryParamsOptional):
    pass

class GetUserLeaderboardParams(TypedDict):
    period: LeaderboardInterval

class GetUserOrdersExternalExternalIdParams(TypedDict):
    external_id: str

_GetUserOrdersExternalExternalIdResultOptional = TypedDict("_GetUserOrdersExternalExternalIdResultOptional", {'status': 'str | None', 'data': 'list[OpenOrderModel] | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class GetUserOrdersExternalExternalIdResult(_GetUserOrdersExternalExternalIdResultOptional):
    pass

class GetVaultFundingHistoryParams(TypedDict):
    pass

class GetVaultOrdersParams(TypedDict):
    pass

class GetVaultPerformanceParams(TypedDict):
    interval: str

_GetVaultPerformanceResultOptional = TypedDict("_GetVaultPerformanceResultOptional", {'status': 'str | None', 'data': 'GetVaultPerformanceResultData | None'}, total=False)
class GetVaultPerformanceResult(_GetVaultPerformanceResultOptional):
    pass

_GetVaultPerformanceResultDataOptional = TypedDict("_GetVaultPerformanceResultDataOptional", {'totalPnl': 'str | None', 'aprPercentage': 'str | None', 'profitFactor': 'str | None', 'winRate': 'str | None', 'maxDrawdown': 'str | None', 'tradingVolume': 'str | None', 'sharpeRatio': 'str | None', 'aprBreakdown': 'GetVaultPerformanceResultDataAprBreakdown | None'}, total=False)
class GetVaultPerformanceResultData(_GetVaultPerformanceResultDataOptional):
    pass

_GetVaultPerformanceResultDataAprBreakdownOptional = TypedDict("_GetVaultPerformanceResultDataAprBreakdownOptional", {'baseApr': 'str | None', 'maxExtraApr': 'str | None'}, total=False)
class GetVaultPerformanceResultDataAprBreakdown(_GetVaultPerformanceResultDataAprBreakdownOptional):
    pass

class GetVaultPositionHistoryParams(TypedDict):
    pass

class GetVaultPositionsParams(TypedDict):
    pass

class GetVaultSummaryParams(TypedDict):
    pass

_GetVaultSummaryResultOptional = TypedDict("_GetVaultSummaryResultOptional", {'status': 'str | None', 'data': 'GetVaultSummaryResultData | None'}, total=False)
class GetVaultSummaryResult(_GetVaultSummaryResultOptional):
    pass

_GetVaultSummaryResultDataOptional = TypedDict("_GetVaultSummaryResultDataOptional", {'equity': 'str | None', 'walletBalance': 'str | None', 'lastMonthApr': 'str | None', 'exposure': 'str | None', 'numberOfDepositors': 'int | Decimal | None', 'ageDays': 'int | Decimal | None', 'profitShare': 'str | None', 'aprBreakdown': 'GetVaultSummaryResultDataAprBreakdown | None'}, total=False)
class GetVaultSummaryResultData(_GetVaultSummaryResultDataOptional):
    pass

_GetVaultSummaryResultDataAprBreakdownOptional = TypedDict("_GetVaultSummaryResultDataAprBreakdownOptional", {'baseApr': 'str | None', 'maxExtraApr': 'str | None'}, total=False)
class GetVaultSummaryResultDataAprBreakdown(_GetVaultSummaryResultDataAprBreakdownOptional):
    pass

class GetVaultTradesParams(TypedDict):
    pass

class GetWalletValidationParams(TypedDict):
    id: str

class GetWithdrawalsParams(TypedDict):
    pass

class GetWrapperParams(TypedDict):
    pass

class L2ConfigModel(TypedDict):
    type: str
    collateralId: str
    collateralResolution: int | Decimal | str
    syntheticId: str
    syntheticResolution: int | Decimal | str

class LimitOrderSettlementModel(TypedDict):
    baseAmount: int | Decimal | str
    quoteAmount: int | Decimal | str
    feeAmount: int | Decimal | str
    baseAssetId: str
    quoteAssetId: str
    feeAssetId: str
    expirationTimestamp: int | Decimal | str
    nonce: int | Decimal | str
    receiverPositionId: int | Decimal | str
    senderPositionId: int | Decimal | str
    signature: SettlementSignatureModel

class MarkWalletCheckedParams(TypedDict):
    passed: bool

class MarketModel(TypedDict):
    name: str
    type: MarketType
    assetName: str
    assetPrecision: int | Decimal | str
    collateralAssetName: str
    collateralAssetPrecision: int | Decimal | str
    active: bool
    isRfq: bool
    isOffHours: bool
    tradingHours: MarketTradingHours
    marketStats: MarketStatsModel
    tradingConfig: TradingConfigModel
    l2Config: L2ConfigModel

class MarketStatsModel(TypedDict):
    dailyVolume: str
    dailyVolumeBase: str
    dailyPriceChange: str
    dailyLow: str
    dailyHigh: str
    lastPrice: str
    askPrice: str
    bidPrice: str
    markPrice: str
    indexPrice: str
    fundingRate: str
    nextFundingRate: int | Decimal | str
    openInterest: str
    openInterestBase: str

class MassCancelParams(TypedDict):
    body: OrderCancelRequest

_MassCancelResultOptional = TypedDict("_MassCancelResultOptional", {'status': 'str | None', 'data': 'EmptyModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class MassCancelResult(_MassCancelResultOptional):
    pass

class OnChainTransfer(TypedDict):
    fromVault: str
    toVault: str
    amount: str
    transferredAsset: str

class OnboardClientParams(TypedDict):
    body: Any

_OnboardClientResultOptional = TypedDict("_OnboardClientResultOptional", {'status': 'str | None', 'data': 'OnboardedClientModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class OnboardClientResult(_OnboardClientResultOptional):
    pass

class OnboardSubaccountParams(TypedDict):
    L1_SIGNATURE: str
    L1_MESSAGE_TIME: str
    body: Any

_OnboardSubaccountResultOptional = TypedDict("_OnboardSubaccountResultOptional", {'status': 'str | None', 'data': 'AccountModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class OnboardSubaccountResult(_OnboardSubaccountResultOptional):
    pass

_OnboardedClientModelOptional = TypedDict("_OnboardedClientModelOptional", {'defaultAccount': 'AccountModel'}, total=False)
class OnboardedClientModel(_OnboardedClientModelOptional):
    l1Address: str

class OnchainWithdrawalParams(TypedDict):
    body: CollateralSlowWithdrawal

_OpenOrderModelOptional = TypedDict("_OpenOrderModelOptional", {'statusReason': 'OrderStatusReason | None', 'price': 'str | None', 'averagePrice': 'str | None', 'rfqStartPrice': 'str | None', 'filledQty': 'str | None', 'cancelledQty': 'str | None', 'chaseOrder': 'bool | None', 'payedFee': 'str | None', 'expiryTime': 'int | Decimal | str | None', 'tpSlType': 'OrderTpslType | None', 'takeProfit': 'OpenOrderTpslTriggerModel | None', 'stopLoss': 'OpenOrderTpslTriggerModel | None'}, total=False)
class OpenOrderModel(_OpenOrderModelOptional):
    id: int | Decimal | str
    accountId: int | Decimal | str
    externalId: str
    market: str
    type: OrderType
    side: OrderSide
    status: OrderStatus
    qty: str
    reduceOnly: bool
    postOnly: bool
    createdTime: int | Decimal | str
    updatedTime: int | Decimal | str
    timeInForce: TimeInForce

_OpenOrderTpslTriggerModelOptional = TypedDict("_OpenOrderTpslTriggerModelOptional", {'status': 'OrderStatus | None'}, total=False)
class OpenOrderTpslTriggerModel(_OpenOrderTpslTriggerModelOptional):
    triggerPrice: str
    triggerPriceType: OrderTriggerPriceType
    price: str
    priceType: OrderPriceType

_OrderCancelRequestOptional = TypedDict("_OrderCancelRequestOptional", {'market': 'str | None', 'side': 'OrderSide | None', 'ids': 'list[int | Decimal] | None', 'externalIds': 'list[str] | None'}, total=False)
class OrderCancelRequest(_OrderCancelRequestOptional):
    pass

_OrderConditionalTriggerOptional = TypedDict("_OrderConditionalTriggerOptional", {'triggerPrice': 'str | None', 'triggerType': 'str | None'}, total=False)
class OrderConditionalTrigger(_OrderConditionalTriggerOptional):
    pass

_OrderbookQuantityModelOptional = TypedDict("_OrderbookQuantityModelOptional", {'qty': 'str | None', 'price': 'str | None'}, total=False)
class OrderbookQuantityModel(_OrderbookQuantityModelOptional):
    pass

_OrderbookUpdateModelOptional = TypedDict("_OrderbookUpdateModelOptional", {'market': 'str | None', 'bid': 'list[OrderbookQuantityModel] | None', 'ask': 'list[OrderbookQuantityModel] | None'}, total=False)
class OrderbookUpdateModel(_OrderbookUpdateModelOptional):
    pass

class PlaceOrderParams(TypedDict):
    body: CreateOrderRequest

_PlaceOrderResultOptional = TypedDict("_PlaceOrderResultOptional", {'status': 'str | None', 'data': 'PlacedOrderModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class PlaceOrderResult(_PlaceOrderResultOptional):
    pass

class PlaceRfqOrderParams(TypedDict):
    body: CreateRFQOrderRequest

_PlaceRfqOrderResultOptional = TypedDict("_PlaceRfqOrderResultOptional", {'status': 'str | None', 'data': 'PlacedOrderModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class PlaceRfqOrderResult(_PlaceRfqOrderResultOptional):
    pass

class PlacedOrderModel(TypedDict):
    id: int | Decimal | str
    externalId: str

_PositionHistoryModelOptional = TypedDict("_PositionHistoryModelOptional", {'exitPrice': 'str | None', 'exitType': 'ExitType | None', 'closedTime': 'int | Decimal | str | None'}, total=False)
class PositionHistoryModel(_PositionHistoryModelOptional):
    id: int | Decimal | str
    accountId: int | Decimal | str
    market: str
    side: PositionSide
    size: str
    maxPositionSize: str
    leverage: str
    openPrice: str
    realisedPnl: str
    realisedPnlBreakdown: RealisedPnlBreakdownModel
    createdTime: int | Decimal | str

_PositionModelOptional = TypedDict("_PositionModelOptional", {'liquidationPrice': 'str | None', 'tpPrice': 'str | None', 'slPrice': 'str | None', 'adl': 'int | Decimal | str | None'}, total=False)
class PositionModel(_PositionModelOptional):
    id: int | Decimal | str
    accountId: int | Decimal | str
    market: str
    status: PositionStatus
    side: PositionSide
    leverage: str
    size: str
    value: str
    openPrice: str
    markPrice: str
    unrealisedPnl: str
    realisedPnl: str
    createdAt: int | Decimal | str
    updatedAt: int | Decimal | str

class PostUserTransferOnchainParams(TypedDict):
    body: OnChainTransfer

_PostUserTransferOnchainResultOptional = TypedDict("_PostUserTransferOnchainResultOptional", {'status': 'str | None', 'data': 'TransferResponseModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class PostUserTransferOnchainResult(_PostUserTransferOnchainResultOptional):
    pass

class QuoteModel(TypedDict):
    id: str
    fee: str

class RealisedPnlBreakdownModel(TypedDict):
    tradePnl: str
    fundingFees: str
    openFees: str
    closeFees: str

class RiskFactorConfigModel(TypedDict):
    upperBound: str
    riskFactor: str

class SetDeadmanSwitchParams(TypedDict):
    countdownTime: int | Decimal

class SettingsModel(TypedDict):
    starkExContractAddress: str

class SettlementSignatureModel(TypedDict):
    r: str
    s: str

_SpotBalanceModelOptional = TypedDict("_SpotBalanceModelOptional", {'availableToWithdraw': 'str | None', 'absolutePnl': 'str | None', 'pnlPercentage': 'str | None', 'averageEntryPrice': 'str | None'}, total=False)
class SpotBalanceModel(_SpotBalanceModelOptional):
    accountId: int | Decimal | str
    asset: str
    balance: str
    indexPrice: str
    notionalValue: str
    contributionFactor: str
    equityContribution: str
    updatedAt: int | Decimal | str

class TradingConfigModel(TypedDict):
    minOrderSize: str
    minOrderSizeChange: str
    minPriceChange: str
    maxMarketOrderValue: str
    maxLimitOrderValue: str
    maxPositionValue: str
    maxLeverage: str
    maxNumOrders: int | Decimal | str
    limitPriceCap: str
    limitPriceFloor: str
    riskFactorConfig: list[RiskFactorConfigModel]

class TradingFeeModel(TypedDict):
    market: str
    makerFeeRate: str
    takerFeeRate: str
    builderFeeRate: str

class TransferParams(TypedDict):
    body: TransferRequest

class TransferPatchParams(TypedDict):
    body: TransferRequest

_TransferRequestOptional = TypedDict("_TransferRequestOptional", {'settlement': 'dict[str, Any] | None'}, total=False)
class TransferRequest(_TransferRequestOptional):
    fromAccount: int | Decimal
    toAccount: int | Decimal
    amount: str
    transferredAsset: str

_TransferResponseModelOptional = TypedDict("_TransferResponseModelOptional", {'id': 'Any | None', 'hashCalculated': 'Any | None', 'starkExRepresentation': 'Any | None'}, total=False)
class TransferResponseModel(_TransferResponseModelOptional):
    validSignature: bool

_TwapOptional = TypedDict("_TwapOptional", {'numParts': 'int | Decimal | None', 'interval': 'int | Decimal | None'}, total=False)
class Twap(_TwapOptional):
    pass

class UpdateAccountInfoParams(TypedDict):
    body: AccountUpdateRequest

class UpdateFundingRateConfigParams(TypedDict):
    body: FundingRateConfigRequest

class UpdateLeverageParams(TypedDict):
    body: AccountLeverage

_UpdateLeverageResultOptional = TypedDict("_UpdateLeverageResultOptional", {'status': 'str | None', 'data': 'EmptyModel | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class UpdateLeverageResult(_UpdateLeverageResultOptional):
    pass

_UpdateReferralLinkCodeRequestOptional = TypedDict("_UpdateReferralLinkCodeRequestOptional", {'description': 'str | None'}, total=False)
class UpdateReferralLinkCodeRequest(_UpdateReferralLinkCodeRequestOptional):
    code: str

class UpdateReferralLinkParams(TypedDict):
    body: UpdateReferralLinkCodeRequest

class UseReferralCodeParams(TypedDict):
    body: UseReferralCodeRequest

class UseReferralCodeRequest(TypedDict):
    code: str

class ValidateWalletParams(TypedDict):
    body: WalletValidationRequest

class WalletValidationRequest(TypedDict):
    address: str

class WithdrawFromVaultParams(TypedDict):
    body: WithdrawRequestModel

_WithdrawFromVaultResultOptional = TypedDict("_WithdrawFromVaultResultOptional", {'status': 'str | None', 'data': 'Any | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class WithdrawFromVaultResult(_WithdrawFromVaultResultOptional):
    pass

class WithdrawParams(TypedDict):
    body: WithdrawalRequest

class WithdrawRequestModel(TypedDict):
    fromAccountId: int | Decimal | str
    toAccountId: int | Decimal | str
    collateral: str
    shares: str
    settlement: LimitOrderSettlementModel

_WithdrawResultOptional = TypedDict("_WithdrawResultOptional", {'status': 'str | None', 'data': 'int | Decimal | str | None', 'error': 'Any | None', 'pagination': 'dict[str, Any] | None'}, total=False)
class WithdrawResult(_WithdrawResultOptional):
    pass

_WithdrawalRequestOptional = TypedDict("_WithdrawalRequestOptional", {'nonce': 'int | Decimal'}, total=False)
class WithdrawalRequest(_WithdrawalRequestOptional):
    amount: str
    asset: str
    destination: str

AssetOperationStatus = str
AssetOperationType = str
AssetType = str
ExitType = str
ExportType = str
LeaderboardInterval = str
MarketTradingHours = str
MarketType = str
OrderPriceType = str
OrderSide = str
OrderSortBy = str
OrderStatus = str
OrderStatusReason = str
OrderTimeInForce = str
OrderTpslType = str
OrderTriggerPriceType = str
OrderType = str
PositionSide = str
PositionStatus = str
ReferralDashboardGranularity = str
ReferralDashboardTimeRange = str
TimeInForce = str
TradeType = str
