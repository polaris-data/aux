"""Paradex REST wire models generated from src/exchanges/paradex/schema/paradex.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

class ActivateSubkeyParams(TypedDict):
    body: RequestsActivateSubkey

class ArchiveSubAccountParams(TypedDict):
    address: str

class BlockOffersCancelParams(TypedDict):
    block_trade_id: str
    offer_id: str

class BlockOffersCreateParams(TypedDict):
    block_trade_id: str
    body: RequestsBlockOfferRequest

class BlockOffersExecuteParams(TypedDict):
    block_trade_id: str
    offer_id: str
    body: RequestsBlockExecuteRequest

class BlockOffersGetParams(TypedDict):
    block_trade_id: str
    offer_id: str

class BlockOffersListParams(TypedDict):
    block_trade_id: str

class BlockTradesCancelParams(TypedDict):
    block_trade_id: str

class BlockTradesCreateParams(TypedDict):
    body: RequestsBlockTradeRequest

class BlockTradesExecuteParams(TypedDict):
    block_trade_id: str
    body: RequestsBlockExecuteRequest

class BlockTradesGetParams(TypedDict):
    block_trade_id: str

_BlockTradesListParamsOptional = TypedDict("_BlockTradesListParamsOptional", {'status': 'str | None', 'market': 'str | None'}, total=False)
class BlockTradesListParams(_BlockTradesListParamsOptional):
    pass

class CancelAlgoOrderParams(TypedDict):
    algo_id: str

class CreateAlgoOrderParams(TypedDict):
    body: RequestsAlgoOrderRequest

class CreateDepositAddressParams(TypedDict):
    body: RequestsCreateDepositAddressReq

class CreateTokenParams(TypedDict):
    body: RequestsCreateToken

class CreateXpTransferV2Params(TypedDict):
    body: RequestsCreateXPTransferRequest

class DeleteAccountMmpParams(TypedDict):
    base_asset: str

_EvmAuthParamsOptional = TypedDict("_EvmAuthParamsOptional", {'PARADEX-AUTHORIZE-ISOLATED-MARKETS': 'str | None'}, total=False)
_EvmAuthParamsWireKeys = TypedDict("_EvmAuthParamsWireKeys", {'PARADEX-STARKNET-ACCOUNT': 'str', 'PARADEX-EVM-SIGNATURE': 'str', 'PARADEX-SIWE-MESSAGE': 'str'})
class EvmAuthParams(_EvmAuthParamsWireKeys, _EvmAuthParamsOptional):
    pass

_EvmOnboardingParamsWireKeys = TypedDict("_EvmOnboardingParamsWireKeys", {'PARADEX-STARKNET-ACCOUNT': 'str', 'PARADEX-EVM-SIGNATURE': 'str', 'PARADEX-SIWE-MESSAGE': 'str'})
class EvmOnboardingParams(_EvmOnboardingParamsWireKeys):
    body: RequestsOnboarding

class GetAccountHistoryParams(TypedDict):
    type: str

class GetAccountMarginParams(TypedDict):
    market: str

_GetAccountMmpParamsOptional = TypedDict("_GetAccountMmpParamsOptional", {'base_asset': 'str | None'}, total=False)
class GetAccountMmpParams(_GetAccountMmpParamsOptional):
    pass

_GetAccountParamsOptional = TypedDict("_GetAccountParamsOptional", {'subaccount_address': 'str | None'}, total=False)
class GetAccountParams(_GetAccountParamsOptional):
    pass

class GetAccountProfileParams(TypedDict):
    pass

class GetAccountSettingsParams(TypedDict):
    pass

class GetAccountsInfoParams(TypedDict):
    pass

class GetAccountsParams(TypedDict):
    pass

class GetAlgoOrderByIdParams(TypedDict):
    algo_id: str

_GetAlgoOrdersHistoryParamsOptional = TypedDict("_GetAlgoOrdersHistoryParamsOptional", {'client_id': 'str | None', 'cursor': 'str | None', 'end_at': 'int | Decimal | None', 'market': 'str | None', 'page_size': 'int | Decimal | None', 'side': 'str | None', 'start_at': 'int | Decimal | None', 'status': 'str | None', 'type': 'str | None'}, total=False)
class GetAlgoOrdersHistoryParams(_GetAlgoOrdersHistoryParamsOptional):
    pass

_GetAnnouncementsParamsOptional = TypedDict("_GetAnnouncementsParamsOptional", {'cursor': 'str | None', 'end_at': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'start_at': 'int | Decimal | None'}, total=False)
class GetAnnouncementsParams(_GetAnnouncementsParamsOptional):
    pass

class GetBalanceParams(TypedDict):
    pass

class GetBboInteractiveParams(TypedDict):
    market: str

class GetBboParams(TypedDict):
    market: str

class GetComplianceActionsParams(TypedDict):
    pass

class GetDepositAddressParams(TypedDict):
    id: str

class GetDimeConfigParams(TypedDict):
    pass

_GetFillsParamsOptional = TypedDict("_GetFillsParamsOptional", {'asset_kind': 'str | None', 'cursor': 'str | None', 'end_at': 'int | Decimal | None', 'market': 'str | None', 'page_size': 'int | Decimal | None', 'start_at': 'int | Decimal | None'}, total=False)
class GetFillsParams(_GetFillsParamsOptional):
    pass

_GetFundingDataParamsOptional = TypedDict("_GetFundingDataParamsOptional", {'cursor': 'str | None', 'end_at': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'start_at': 'int | Decimal | None'}, total=False)
class GetFundingDataParams(_GetFundingDataParamsOptional):
    market: str

_GetFundingParamsOptional = TypedDict("_GetFundingParamsOptional", {'cursor': 'str | None', 'end_at': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'start_at': 'int | Decimal | None'}, total=False)
class GetFundingParams(_GetFundingParamsOptional):
    market: str

class GetGlobalStakingBalanceHistoryParams(TypedDict):
    resolution: str

class GetImpactPriceParams(TypedDict):
    market: str
    size: int | Decimal

class GetInsuranceFundParams(TypedDict):
    pass

_GetJwtParamsOptional = TypedDict("_GetJwtParamsOptional", {'PARADEX-SIGNATURE-EXPIRATION': 'str | None', 'PARADEX-AUTHORIZE-ISOLATED-MARKETS': 'str | None'}, total=False)
_GetJwtParamsWireKeys = TypedDict("_GetJwtParamsWireKeys", {'PARADEX-STARKNET-ACCOUNT': 'str', 'PARADEX-STARKNET-SIGNATURE': 'str', 'PARADEX-TIMESTAMP': 'str'})
class GetJwtParams(_GetJwtParamsWireKeys, _GetJwtParamsOptional):
    pass

_GetLiquidationsParamsOptional = TypedDict("_GetLiquidationsParamsOptional", {'from': 'int | Decimal | None', 'to': 'int | Decimal | None'}, total=False)
class GetLiquidationsParams(_GetLiquidationsParamsOptional):
    pass

_GetMarginConfigParamsOptional = TypedDict("_GetMarginConfigParamsOptional", {'base_asset': 'str | None'}, total=False)
class GetMarginConfigParams(_GetMarginConfigParamsOptional):
    pass

class GetMarketSettlementPriceParams(TypedDict):
    market: str

_GetMarketsHistoryParamsOptional = TypedDict("_GetMarketsHistoryParamsOptional", {'market': 'str | None'}, total=False)
class GetMarketsHistoryParams(_GetMarketsHistoryParamsOptional):
    pass

_GetMarketsParamsOptional = TypedDict("_GetMarketsParamsOptional", {'market': 'str | None', 'asset_kind': 'str | None', 'base_asset': 'str | None'}, total=False)
class GetMarketsParams(_GetMarketsParamsOptional):
    pass

_GetMarketsSummaryParamsOptional = TypedDict("_GetMarketsSummaryParamsOptional", {'asset_kind': 'str | None', 'base_asset': 'str | None', 'end': 'int | Decimal | None', 'start': 'int | Decimal | None'}, total=False)
class GetMarketsSummaryParams(_GetMarketsSummaryParamsOptional):
    market: str

_GetOnboardingParamsOptional = TypedDict("_GetOnboardingParamsOptional", {'public_key': 'str | None', 'eth_address': 'str | None', 'vault_operator_index': 'int | Decimal | None'}, total=False)
class GetOnboardingParams(_GetOnboardingParamsOptional):
    account_signer_type: str

class GetOpenAlgoOrdersParams(TypedDict):
    pass

_GetOpenOrdersParamsOptional = TypedDict("_GetOpenOrdersParamsOptional", {'market': 'str | None'}, total=False)
class GetOpenOrdersParams(_GetOpenOrdersParamsOptional):
    pass

_GetOrderbookInteractiveParamsOptional = TypedDict("_GetOrderbookInteractiveParamsOptional", {'depth': 'int | Decimal | None', 'price_tick': 'str | None'}, total=False)
class GetOrderbookInteractiveParams(_GetOrderbookInteractiveParamsOptional):
    market: str

_GetOrderbookParamsOptional = TypedDict("_GetOrderbookParamsOptional", {'depth': 'int | Decimal | None', 'price_tick': 'str | None'}, total=False)
class GetOrderbookParams(_GetOrderbookParamsOptional):
    market: str

_GetOrdersParamsOptional = TypedDict("_GetOrdersParamsOptional", {'client_id': 'str | None', 'cursor': 'str | None', 'end_at': 'int | Decimal | None', 'market': 'str | None', 'page_size': 'int | Decimal | None', 'side': 'str | None', 'start_at': 'int | Decimal | None', 'status': 'str | None', 'type': 'str | None'}, total=False)
class GetOrdersParams(_GetOrdersParamsOptional):
    pass

_GetPortfolioMarginConfigParamsOptional = TypedDict("_GetPortfolioMarginConfigParamsOptional", {'base_asset': 'str | None'}, total=False)
class GetPortfolioMarginConfigParams(_GetPortfolioMarginConfigParamsOptional):
    pass

class GetPositionsParams(TypedDict):
    pass

_GetPublicXpTransfersV2ParamsOptional = TypedDict("_GetPublicXpTransfersV2ParamsOptional", {'start_at': 'int | Decimal | None', 'end_at': 'int | Decimal | None', 'season': 'str | None', 'cursor': 'str | None', 'page_size': 'int | Decimal | None'}, total=False)
class GetPublicXpTransfersV2Params(_GetPublicXpTransfersV2ParamsOptional):
    pass

class GetReferralQrCodeParams(TypedDict):
    pass

class GetReferralsConfigParams(TypedDict):
    pass

class GetReferralsSummaryParams(TypedDict):
    pass

class GetStakingBalanceHistoryParams(TypedDict):
    resolution: str

class GetStakingConfigParams(TypedDict):
    pass

class GetStakingHistoryParams(TypedDict):
    pass

class GetStakingSummaryParams(TypedDict):
    pass

class GetSubAccountsParams(TypedDict):
    pass

_GetSubkeyJwtParamsOptional = TypedDict("_GetSubkeyJwtParamsOptional", {'PARADEX-SIGNATURE-EXPIRATION': 'str | None', 'PARADEX-AUTHORIZE-ISOLATED-MARKETS': 'str | None'}, total=False)
_GetSubkeyJwtParamsWireKeys = TypedDict("_GetSubkeyJwtParamsWireKeys", {'PARADEX-STARKNET-ACCOUNT': 'str', 'PARADEX-STARKNET-SIGNATURE': 'str', 'PARADEX-TIMESTAMP': 'str'})
class GetSubkeyJwtParams(_GetSubkeyJwtParamsWireKeys, _GetSubkeyJwtParamsOptional):
    public_key: str

class GetSubkeyParams(TypedDict):
    public_key: str

class GetSystemConfigParams(TypedDict):
    pass

class GetSystemStateParams(TypedDict):
    pass

class GetSystemStatsParams(TypedDict):
    pass

class GetSystemTimeParams(TypedDict):
    pass

class GetTransferFeeConfigV2Params(TypedDict):
    pass

_GetTransfersParamsOptional = TypedDict("_GetTransfersParamsOptional", {'cursor': 'str | None', 'end_at': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'start_at': 'int | Decimal | None', 'status': 'str | None'}, total=False)
class GetTransfersParams(_GetTransfersParamsOptional):
    pass

class GetVolumeTiersParams(TypedDict):
    pass

_GetXpBalanceV2ParamsOptional = TypedDict("_GetXpBalanceV2ParamsOptional", {'season': 'str | None'}, total=False)
class GetXpBalanceV2Params(_GetXpBalanceV2ParamsOptional):
    pass

class GetXpTransferByIdV2Params(TypedDict):
    transfer_id: str

_GetXpTransfersV2ParamsOptional = TypedDict("_GetXpTransfersV2ParamsOptional", {'role': 'str | None', 'season': 'str | None', 'cursor': 'str | None', 'page_size': 'int | Decimal | None'}, total=False)
class GetXpTransfersV2Params(_GetXpTransfersV2ParamsOptional):
    pass

class JwksParams(TypedDict):
    pass

class ListDepositNetworksParams(TypedDict):
    pass

_ListSubkeysParamsOptional = TypedDict("_ListSubkeysParamsOptional", {'with_revoked': 'bool | None'}, total=False)
class ListSubkeysParams(_ListSubkeysParamsOptional):
    pass

_ListTokensParamsOptional = TypedDict("_ListTokensParamsOptional", {'kind': 'str | None', 'with_invalid': 'bool | None'}, total=False)
class ListTokensParams(_ListTokensParamsOptional):
    pass

_MarketsKlinesParamsOptional = TypedDict("_MarketsKlinesParamsOptional", {'price_kind': 'str | None'}, total=False)
class MarketsKlinesParams(_MarketsKlinesParamsOptional):
    end_at: int | Decimal
    resolution: str
    start_at: int | Decimal
    symbol: str

_Messagesv1NftPriceOptional = TypedDict("_Messagesv1NftPriceOptional", {'currency': 'str | None', 'decimals': 'int | Decimal | None', 'value': 'str | None'}, total=False)
class Messagesv1NftPrice(_Messagesv1NftPriceOptional):
    pass

_OnboardingParamsOptional = TypedDict("_OnboardingParamsOptional", {'PARADEX-PARENT-SIWE-MESSAGE': 'str | None', 'PARADEX-PARENT-SIGNATURE': 'str | None'}, total=False)
_OnboardingParamsWireKeys = TypedDict("_OnboardingParamsWireKeys", {'PARADEX-ETHEREUM-ACCOUNT': 'str', 'PARADEX-STARKNET-ACCOUNT': 'str', 'PARADEX-STARKNET-SIGNATURE': 'str'})
class OnboardingParams(_OnboardingParamsWireKeys, _OnboardingParamsOptional):
    body: RequestsOnboarding

_OrdersCancelAllParamsOptional = TypedDict("_OrdersCancelAllParamsOptional", {'market': 'str | None'}, total=False)
class OrdersCancelAllParams(_OrdersCancelAllParamsOptional):
    pass

class OrdersCancelBatchParams(TypedDict):
    body: RequestsCancelOrderBatchRequest

_OrdersCancelByClientIdParamsOptional = TypedDict("_OrdersCancelByClientIdParamsOptional", {'market': 'str | None'}, total=False)
class OrdersCancelByClientIdParams(_OrdersCancelByClientIdParamsOptional):
    client_id: str

class OrdersCancelParams(TypedDict):
    order_id: str

class OrdersGetByClientIdParams(TypedDict):
    client_id: str

class OrdersGetParams(TypedDict):
    order_id: str

class OrdersModifyParams(TypedDict):
    order_id: str
    body: RequestsModifyOrderRequest

class OrdersNewBatchParams(TypedDict):
    body: list[RequestsOrderRequest]

class OrdersNewParams(TypedDict):
    body: RequestsOrderRequest

class PostAccountTerminalTokenParams(TypedDict):
    pass

class RecordComplianceActionParams(TypedDict):
    body: RequestsRecordComplianceActionRequest

class RefreshAccountInventoryParams(TypedDict):
    pass

class RegisterSubkeyParams(TypedDict):
    body: RequestsCreateSubkey

_RequestsAccountMarginRequestOptional = TypedDict("_RequestsAccountMarginRequestOptional", {'on_behalf_of_account': 'str | None'}, total=False)
class RequestsAccountMarginRequest(_RequestsAccountMarginRequestOptional):
    leverage: int | Decimal
    margin_type: str

_RequestsAccountMmpConfigRequestOptional = TypedDict("_RequestsAccountMmpConfigRequestOptional", {'delta_limit': 'str | None', 'protect_block_trades': 'bool | None', 'size_limit': 'str | None', 'vega_limit': 'str | None'}, total=False)
class RequestsAccountMmpConfigRequest(_RequestsAccountMmpConfigRequestOptional):
    frozen_time_ms: int | Decimal
    interval_ms: int | Decimal

_RequestsAccountMmpResetRequestOptional = TypedDict("_RequestsAccountMmpResetRequestOptional", {'base_asset': 'str | None'}, total=False)
class RequestsAccountMmpResetRequest(_RequestsAccountMmpResetRequestOptional):
    pass

_RequestsActivateSubkeyOptional = TypedDict("_RequestsActivateSubkeyOptional", {'name': 'str | None', 'signature': 'list[str]'}, total=False)
class RequestsActivateSubkey(_RequestsActivateSubkeyOptional):
    account_id: str
    public_key: str
    timestamp: int | Decimal

_RequestsActivateSubkeyResponseOptional = TypedDict("_RequestsActivateSubkeyResponseOptional", {'encrypted_key': 'str | None'}, total=False)
class RequestsActivateSubkeyResponse(_RequestsActivateSubkeyResponseOptional):
    pass

_RequestsAlgoOrderRequestOptional = TypedDict("_RequestsAlgoOrderRequestOptional", {'dime_discount': 'bool | None', 'frequency': 'int | Decimal | None', 'on_behalf_of_account': 'str | None', 'recv_window': 'int | Decimal | None', 'signature': 'str', 'signature_timestamp': 'int | Decimal'}, total=False)
class RequestsAlgoOrderRequest(_RequestsAlgoOrderRequestOptional):
    algo_type: str
    duration_seconds: int | Decimal
    market: str
    side: ResponsesOrderSide
    size: str
    type: ResponsesOrderType

_RequestsBlockExecuteRequestOptional = TypedDict("_RequestsBlockExecuteRequestOptional", {'selected_offers': 'list[str] | None'}, total=False)
class RequestsBlockExecuteRequest(_RequestsBlockExecuteRequestOptional):
    execution_nonce: str
    signatures: dict[str, ResponsesBlockTradeSignature]

class RequestsBlockOfferInfo(TypedDict):
    offerer_order: ResponsesBlockTradeOrder
    price: str
    size: str

_RequestsBlockOfferRequestOptional = TypedDict("_RequestsBlockOfferRequestOptional", {'nonce': 'str', 'signature': 'ResponsesBlockTradeSignature'}, total=False)
class RequestsBlockOfferRequest(_RequestsBlockOfferRequestOptional):
    offering_account: str
    trades: dict[str, RequestsBlockOfferInfo]

_RequestsBlockTradeInfoOptional = TypedDict("_RequestsBlockTradeInfoOptional", {'maker_order': 'ResponsesBlockTradeOrder | None', 'price': 'str | None', 'size': 'str | None', 'taker_order': 'ResponsesBlockTradeOrder | None', 'trade_constraints': 'ResponsesBlockTradeConstraints | None'}, total=False)
class RequestsBlockTradeInfo(_RequestsBlockTradeInfoOptional):
    pass

_RequestsBlockTradeRequestOptional = TypedDict("_RequestsBlockTradeRequestOptional", {'nonce': 'str'}, total=False)
class RequestsBlockTradeRequest(_RequestsBlockTradeRequestOptional):
    block_expiration: int | Decimal
    required_signers: list[str]
    signatures: dict[str, ResponsesBlockTradeSignature]
    trades: dict[str, RequestsBlockTradeInfo]

_RequestsCancelOrderBatchRequestOptional = TypedDict("_RequestsCancelOrderBatchRequestOptional", {'client_order_ids': 'list[str] | None', 'order_ids': 'list[str] | None'}, total=False)
class RequestsCancelOrderBatchRequest(_RequestsCancelOrderBatchRequestOptional):
    pass

_RequestsCreateDepositAddressReqOptional = TypedDict("_RequestsCreateDepositAddressReqOptional", {'token': 'str | None'}, total=False)
class RequestsCreateDepositAddressReq(_RequestsCreateDepositAddressReqOptional):
    network: str

_RequestsCreateSubkeyOptional = TypedDict("_RequestsCreateSubkeyOptional", {'allowed_cidrs': 'list[str] | None', 'encrypted_key': 'str | None', 'eph_public_key': 'str | None', 'evm_signature': 'str | None', 'signature': 'list[str] | None', 'signature_expiry': 'int | Decimal | None', 'signature_timestamp': 'int | Decimal | None', 'siwe_message': 'str | None', 'state': 'str | None'}, total=False)
class RequestsCreateSubkey(_RequestsCreateSubkeyOptional):
    name: str
    public_key: str

_RequestsCreateTokenOptional = TypedDict("_RequestsCreateTokenOptional", {'allowed_cidrs': 'list[str] | None'}, total=False)
class RequestsCreateToken(_RequestsCreateTokenOptional):
    expiry_duration: int | Decimal
    name: str
    token_type: str

_RequestsCreateVaultOptional = TypedDict("_RequestsCreateVaultOptional", {'curator_name': 'str | None', 'deposit_tx_signature': 'str | None', 'description': 'str | None', 'lockup_period': 'int | Decimal | None', 'max_tvl': 'int | Decimal | None', 'name': 'str | None', 'operator_eth_address': 'str | None', 'operator_index': 'int | Decimal | None', 'operator_signer_type': 'str | None', 'profit_share': 'int | Decimal | None', 'public_key': 'str | None', 'show_balances': 'bool | None', 'show_positions': 'bool | None', 'strategy_description': 'str | None'}, total=False)
class RequestsCreateVault(_RequestsCreateVaultOptional):
    pass

_RequestsCreateXPTransferRequestOptional = TypedDict("_RequestsCreateXPTransferRequestOptional", {'is_private': 'bool | None', 'season': 'str | None'}, total=False)
class RequestsCreateXPTransferRequest(_RequestsCreateXPTransferRequestOptional):
    amount: int | Decimal
    recipient: str

_RequestsModifyOrderRequestOptional = TypedDict("_RequestsModifyOrderRequestOptional", {'on_behalf_of_account': 'str | None', 'signature': 'str', 'signature_timestamp': 'int | Decimal', 'trigger_price': 'str | None'}, total=False)
class RequestsModifyOrderRequest(_RequestsModifyOrderRequestOptional):
    id: str
    market: str
    price: str
    side: str
    size: str
    type: str

_RequestsOnboardingOptional = TypedDict("_RequestsOnboardingOptional", {'marketing_code': 'str | None', 'portfolio_margin': 'bool | None', 'public_key': 'str | None', 'referral_code': 'str | None', 'utm': 'RequestsUtm | None'}, total=False)
class RequestsOnboarding(_RequestsOnboardingOptional):
    pass

_RequestsOrderRequestOptional = TypedDict("_RequestsOrderRequestOptional", {'client_id': 'str | None', 'dime_discount': 'bool | None', 'flags': 'list[ResponsesOrderFlag] | None', 'on_behalf_of_account': 'str | None', 'recv_window': 'int | Decimal | None', 'signature': 'str', 'signature_timestamp': 'int | Decimal', 'stp': 'str | None', 'trigger_price': 'str | None', 'vwap_price': 'str | None'}, total=False)
class RequestsOrderRequest(_RequestsOrderRequestOptional):
    instruction: ResponsesOrderInstruction
    market: str
    price: str
    side: ResponsesOrderSide
    size: str
    type: ResponsesOrderType

class RequestsRecordComplianceActionRequest(TypedDict):
    action_type: str
    action_version: str

_RequestsRfqCreateRequestOptional = TypedDict("_RequestsRfqCreateRequestOptional", {'counterparties': 'list[str] | None', 'is_anonymous': 'bool | None', 'label': 'str | None', 'max_slippage': 'str | None', 'side': 'str | None', 'strategy': 'str | None'}, total=False)
class RequestsRfqCreateRequest(_RequestsRfqCreateRequestOptional):
    legs: list[RequestsRfqLeg]
    quantity: str

_RequestsRfqExecuteRequestOptional = TypedDict("_RequestsRfqExecuteRequestOptional", {'label': 'str | None'}, total=False)
class RequestsRfqExecuteRequest(_RequestsRfqExecuteRequestOptional):
    price: str
    quantity: str
    side: str

_RequestsRfqLegOptional = TypedDict("_RequestsRfqLegOptional", {'price': 'str | None'}, total=False)
class RequestsRfqLeg(_RequestsRfqLegOptional):
    market: str
    ratio: str
    side: str

class RequestsUpdateAccountMaxSlippageRequest(TypedDict):
    max_slippage: str

_RequestsUpdateAccountProfileRequestOptional = TypedDict("_RequestsUpdateAccountProfileRequestOptional", {'ai_agent_url': 'str | None', 'announcements': 'bool | None', 'fill_sound': 'bool | None', 'fills': 'bool | None', 'is_username_private': 'bool | None', 'max_slippage': 'str | None', 'orders': 'bool | None', 'referral_code': 'str | None', 'referred_by': 'str | None', 'size_currency_display': 'str | None', 'tap_share_rate': 'str | None', 'trading_value_display': 'str | None', 'transfers': 'bool | None', 'username': 'str | None', 'xp_share_rate': 'str | None'}, total=False)
class RequestsUpdateAccountProfileRequest(_RequestsUpdateAccountProfileRequestOptional):
    pass

_RequestsUpdateAllowedCIDRsOptional = TypedDict("_RequestsUpdateAllowedCIDRsOptional", {'allowed_cidrs': 'list[str] | None'}, total=False)
class RequestsUpdateAllowedCIDRs(_RequestsUpdateAllowedCIDRsOptional):
    pass

_RequestsUpdateNotificationPreferencesRequestOptional = TypedDict("_RequestsUpdateNotificationPreferencesRequestOptional", {'announcements': 'bool | None', 'fill_sound': 'bool | None', 'fills': 'bool | None', 'orders': 'bool | None', 'transfers': 'bool | None'}, total=False)
class RequestsUpdateNotificationPreferencesRequest(_RequestsUpdateNotificationPreferencesRequestOptional):
    pass

class RequestsUpdateSizeCurrencyDisplayRequest(TypedDict):
    size_currency_display: str

class RequestsUpdateSubkey(TypedDict):
    name: str

class RequestsUpdateSubkeyBlockTradeSigning(TypedDict):
    enabled: bool

class RequestsUpdateTradingValueDisplayRequest(TypedDict):
    trading_value_display: str

_RequestsUtmOptional = TypedDict("_RequestsUtmOptional", {'campaign': 'str | None', 'source': 'str | None', 'type': 'str | None'}, total=False)
class RequestsUtm(_RequestsUtmOptional):
    pass

_ResetAccountMmpParamsOptional = TypedDict("_ResetAccountMmpParamsOptional", {'body': 'RequestsAccountMmpResetRequest | None'}, total=False)
class ResetAccountMmpParams(_ResetAccountMmpParamsOptional):
    pass

_ResponsesAccountAnalyticsOptional = TypedDict("_ResponsesAccountAnalyticsOptional", {'account_address': 'str | None', 'annualized_volatility': 'ResponsesWindowedMetrics | None', 'apr': 'ResponsesWindowedMetrics | None', 'apr_simple_itd': 'str | None', 'cum_return': 'str | None', 'cumulative_pnl_series': 'list[ResponsesAccountDailyPnlPoint] | None', 'cumulative_pnl_usd': 'str | None', 'end_value': 'str | None', 'hourly_cumulative_pnl_series': 'list[ResponsesAccountHourlyPnlPoint] | None', 'max_dd': 'ResponsesWindowedMetrics | None', 'max_drawdown_to_date': 'str | None', 'monthly_pnl_last_12m': 'list[ResponsesAccountMonthlyPnlEntry] | None', 'pnl': 'ResponsesWindowedMetrics | None', 'return': 'ResponsesWindowedMetrics | None', 'sharpe': 'ResponsesWindowedMetrics | None', 'sharpe_annualized_itd': 'str | None', 'strategy_name': 'str | None', 'vol_annualized_itd': 'str | None'}, total=False)
class ResponsesAccountAnalytics(_ResponsesAccountAnalyticsOptional):
    pass

_ResponsesAccountDailyPnlPointOptional = TypedDict("_ResponsesAccountDailyPnlPointOptional", {'day': 'str | None', 'value': 'str | None'}, total=False)
class ResponsesAccountDailyPnlPoint(_ResponsesAccountDailyPnlPointOptional):
    pass

_ResponsesAccountHistoricalDataRespOptional = TypedDict("_ResponsesAccountHistoricalDataRespOptional", {'data': 'list[int | Decimal] | None', 'timestamps': 'list[int | Decimal] | None'}, total=False)
class ResponsesAccountHistoricalDataResp(_ResponsesAccountHistoricalDataRespOptional):
    pass

_ResponsesAccountHourlyPnlPointOptional = TypedDict("_ResponsesAccountHourlyPnlPointOptional", {'at': 'str | None', 'value': 'str | None'}, total=False)
class ResponsesAccountHourlyPnlPoint(_ResponsesAccountHourlyPnlPointOptional):
    pass

_ResponsesAccountInfoResponseOptional = TypedDict("_ResponsesAccountInfoResponseOptional", {'account': 'str | None', 'archived_at': 'int | Decimal | None', 'created_at': 'int | Decimal | None', 'derivation_path': 'str | None', 'fees': 'ResponsesFees | None', 'has_ecosystem_nft': 'bool | None', 'isolated_market': 'str | None', 'kind': 'ResponsesAccountKind | None', 'parent_account': 'str | None', 'public_key': 'str | None', 'username': 'str | None', 'volume_tiers': 'dict[str, ResponsesVolumeTierKindInfo] | None'}, total=False)
class ResponsesAccountInfoResponse(_ResponsesAccountInfoResponseOptional):
    pass

_ResponsesAccountMarginEntryOptional = TypedDict("_ResponsesAccountMarginEntryOptional", {'isolated_margin_leverage': 'int | Decimal | None', 'leverage': 'int | Decimal | None', 'margin_type': 'str | None', 'market': 'str | None'}, total=False)
class ResponsesAccountMarginEntry(_ResponsesAccountMarginEntryOptional):
    pass

_ResponsesAccountMmpConfigRespOptional = TypedDict("_ResponsesAccountMmpConfigRespOptional", {'base_asset': 'str | None', 'delta_limit': 'str | None', 'frozen_time_ms': 'int | Decimal | None', 'frozen_until': 'int | Decimal | None', 'interval_ms': 'int | Decimal | None', 'is_frozen': 'bool | None', 'protect_block_trades': 'bool | None', 'size_limit': 'str | None', 'updated_at': 'int | Decimal | None', 'vega_limit': 'str | None', 'window_delta': 'str | None', 'window_size': 'str | None', 'window_vega': 'str | None'}, total=False)
class ResponsesAccountMmpConfigResp(_ResponsesAccountMmpConfigRespOptional):
    pass

_ResponsesAccountMmpResetRespOptional = TypedDict("_ResponsesAccountMmpResetRespOptional", {'results': 'list[ResponsesAccountMmpResetResult] | None'}, total=False)
class ResponsesAccountMmpResetResp(_ResponsesAccountMmpResetRespOptional):
    pass

_ResponsesAccountMmpResetResultOptional = TypedDict("_ResponsesAccountMmpResetResultOptional", {'base_asset': 'str | None', 'reset_at': 'int | Decimal | None'}, total=False)
class ResponsesAccountMmpResetResult(_ResponsesAccountMmpResetResultOptional):
    pass

_ResponsesAccountMonthlyPnlEntryOptional = TypedDict("_ResponsesAccountMonthlyPnlEntryOptional", {'month': 'str | None', 'month_return_canonical': 'str | None'}, total=False)
class ResponsesAccountMonthlyPnlEntry(_ResponsesAccountMonthlyPnlEntryOptional):
    pass

_ResponsesAccountProfileRespOptional = TypedDict("_ResponsesAccountProfileRespOptional", {'ai_agent_url': 'str | None', 'discord': 'ResponsesDiscordProfile | None', 'is_username_private': 'bool | None', 'last_seen_notification': 'int | Decimal | None', 'market_max_slippage': 'dict[str, str] | None', 'marketed_by': 'str | None', 'nfts': 'list[ResponsesNft] | None', 'notifications': 'ResponsesNotificationPreferencesResp | None', 'referral': 'ResponsesReferralConfigResp | None', 'referral_code': 'str | None', 'referred_by': 'str | None', 'size_currency_display': 'str | None', 'tap_share_rate': 'ResponsesShareRateResp | None', 'tap_status': 'str | None', 'twitter': 'ResponsesTwitterProfile | None', 'twitter_following': 'dict[str, bool] | None', 'username': 'str | None', 'xp_share_rate': 'ResponsesShareRateResp | None'}, total=False)
class ResponsesAccountProfileResp(_ResponsesAccountProfileRespOptional):
    pass

_ResponsesAccountSettingsRespOptional = TypedDict("_ResponsesAccountSettingsRespOptional", {'trading_value_display': 'str | None'}, total=False)
class ResponsesAccountSettingsResp(_ResponsesAccountSettingsRespOptional):
    pass

_ResponsesAccountSummaryResponseOptional = TypedDict("_ResponsesAccountSummaryResponseOptional", {'account': 'str | None', 'account_value': 'str | None', 'free_collateral': 'str | None', 'initial_margin_requirement': 'str | None', 'last_seen_notification': 'int | Decimal | None', 'maintenance_margin_requirement': 'str | None', 'margin_cushion': 'str | None', 'seq_no': 'int | Decimal | None', 'settlement_asset': 'str | None', 'status': 'str | None', 'total_collateral': 'str | None', 'updated_at': 'int | Decimal | None', 'withdrawal_lock': 'ResponsesWithdrawalLockResponse | None', 'withdrawal_lock_rejection': 'ResponsesWithdrawalLockRejectionResponse | None'}, total=False)
class ResponsesAccountSummaryResponse(_ResponsesAccountSummaryResponseOptional):
    pass

_ResponsesAlgoOrderRespOptional = TypedDict("_ResponsesAlgoOrderRespOptional", {'account': 'str | None', 'algo_params': 'dict[str, str] | None', 'algo_type': 'ResponsesAlgoType | None', 'avg_fill_price': 'str | None', 'cancel_reason': 'str | None', 'created_at': 'int | Decimal | None', 'end_at': 'int | Decimal | None', 'id': 'str | None', 'last_updated_at': 'int | Decimal | None', 'market': 'str | None', 'remaining_size': 'str | None', 'side': 'ResponsesOrderSide | None', 'size': 'str | None', 'status': 'ResponsesOrderStatus | None'}, total=False)
class ResponsesAlgoOrderResp(_ResponsesAlgoOrderRespOptional):
    pass

_ResponsesApiTokenOptional = TypedDict("_ResponsesApiTokenOptional", {'allowed_cidrs': 'list[str] | None', 'created_at': 'int | Decimal | None', 'expiry_at': 'int | Decimal | None', 'kind': 'str | None', 'last_updated_at': 'int | Decimal | None', 'lookup_id': 'str | None', 'name': 'str | None', 'revoked_at': 'int | Decimal | None', 'token_id': 'str | None'}, total=False)
class ResponsesApiToken(_ResponsesApiTokenOptional):
    pass

_ResponsesAskBidArrayOptional = TypedDict("_ResponsesAskBidArrayOptional", {'asks': 'list[list[str]] | None', 'best_ask_api': 'list[str] | None', 'best_ask_interactive': 'list[str] | None', 'best_bid_api': 'list[str] | None', 'best_bid_interactive': 'list[str] | None', 'bids': 'list[list[str]] | None', 'last_updated_at': 'int | Decimal | None', 'market': 'str | None', 'seq_no': 'int | Decimal | None'}, total=False)
class ResponsesAskBidArray(_ResponsesAskBidArrayOptional):
    pass

_ResponsesAuthRespOptional = TypedDict("_ResponsesAuthRespOptional", {'jwt_token': 'str | None'}, total=False)
class ResponsesAuthResp(_ResponsesAuthRespOptional):
    pass

_ResponsesBBORespOptional = TypedDict("_ResponsesBBORespOptional", {'ask': 'str | None', 'ask_size': 'str | None', 'bid': 'str | None', 'bid_size': 'str | None', 'last_updated_at': 'int | Decimal | None', 'market': 'str | None', 'seq_no': 'int | Decimal | None'}, total=False)
class ResponsesBBOResp(_ResponsesBBORespOptional):
    pass

_ResponsesBatchResponseOptional = TypedDict("_ResponsesBatchResponseOptional", {'errors': 'list[ResponsesErrorResponse] | None', 'orders': 'list[ResponsesOrderResp] | None'}, total=False)
class ResponsesBatchResponse(_ResponsesBatchResponseOptional):
    pass

_ResponsesBlockTradeConstraintsOptional = TypedDict("_ResponsesBlockTradeConstraintsOptional", {'max_price': 'str | None', 'max_size': 'str | None', 'min_price': 'str | None', 'min_size': 'str | None'}, total=False)
class ResponsesBlockTradeConstraints(_ResponsesBlockTradeConstraintsOptional):
    pass

_ResponsesBlockTradeDetailFullResponseOptional = TypedDict("_ResponsesBlockTradeDetailFullResponseOptional", {'block_expiration': 'int | Decimal | None', 'block_id': 'str | None', 'block_type': 'ResponsesBlockTradeType | None', 'created_at': 'int | Decimal | None', 'failure_reason': 'str | None', 'initiator': 'str | None', 'last_updated_at': 'int | Decimal | None', 'nonce': 'str | None', 'parent_block_id': 'str | None', 'required_signers': 'list[str] | None', 'signatures': 'dict[str, ResponsesBlockTradeSignature] | None', 'status': 'ResponsesBlockTradeStatus | None', 'trades': 'dict[str, ResponsesBlockTradeDetailResponse] | None'}, total=False)
class ResponsesBlockTradeDetailFullResponse(_ResponsesBlockTradeDetailFullResponseOptional):
    pass

_ResponsesBlockTradeDetailResponseOptional = TypedDict("_ResponsesBlockTradeDetailResponseOptional", {'executed_at': 'int | Decimal | None', 'failure_reason': 'str | None', 'maker_account': 'str | None', 'maker_fill': 'ResponsesFillResult | None', 'maker_order': 'ResponsesBlockTradeOrder | None', 'market': 'str | None', 'price': 'str | None', 'size': 'str | None', 'taker_account': 'str | None', 'taker_fill': 'ResponsesFillResult | None', 'taker_order': 'ResponsesBlockTradeOrder | None', 'trade_constraints': 'ResponsesBlockTradeConstraints | None', 'trade_id': 'str | None'}, total=False)
class ResponsesBlockTradeDetailResponse(_ResponsesBlockTradeDetailResponseOptional):
    pass

_ResponsesBlockTradeOrderOptional = TypedDict("_ResponsesBlockTradeOrderOptional", {'account': 'str | None', 'client_id': 'str | None', 'flags': 'list[ResponsesOrderFlag] | None', 'market': 'str | None', 'price': 'str | None', 'signature': 'str | None', 'signature_timestamp': 'int | Decimal | None', 'size': 'str | None'}, total=False)
class ResponsesBlockTradeOrder(_ResponsesBlockTradeOrderOptional):
    side: ResponsesOrderSide
    type: ResponsesOrderType

_ResponsesBlockTradeSignatureOptional = TypedDict("_ResponsesBlockTradeSignatureOptional", {'signer_public_key': 'str | None'}, total=False)
class ResponsesBlockTradeSignature(_ResponsesBlockTradeSignatureOptional):
    nonce: str
    signature_data: str
    signature_expiration: int | Decimal
    signature_timestamp: int | Decimal
    signature_type: str
    signer_account: str

_ResponsesBridgedTokenOptional = TypedDict("_ResponsesBridgedTokenOptional", {'decimals': 'int | Decimal | None', 'l1_bridge_address': 'str | None', 'l1_token_address': 'str | None', 'l2_bridge_address': 'str | None', 'l2_token_address': 'str | None', 'name': 'str | None', 'symbol': 'str | None'}, total=False)
class ResponsesBridgedToken(_ResponsesBridgedTokenOptional):
    pass

_ResponsesCancelOrderBatchResponseOptional = TypedDict("_ResponsesCancelOrderBatchResponseOptional", {'results': 'list[ResponsesCancelOrderResult] | None'}, total=False)
class ResponsesCancelOrderBatchResponse(_ResponsesCancelOrderBatchResponseOptional):
    pass

_ResponsesCancelOrderResultOptional = TypedDict("_ResponsesCancelOrderResultOptional", {'account': 'str | None', 'client_id': 'str | None', 'id': 'str | None', 'market': 'str | None', 'status': 'str | None'}, total=False)
class ResponsesCancelOrderResult(_ResponsesCancelOrderResultOptional):
    pass

_ResponsesCreateTokenResponseOptional = TypedDict("_ResponsesCreateTokenResponseOptional", {'api_token': 'ResponsesApiToken | None', 'token': 'str | None'}, total=False)
class ResponsesCreateTokenResponse(_ResponsesCreateTokenResponseOptional):
    pass

_ResponsesCreateTransferResponseOptional = TypedDict("_ResponsesCreateTransferResponseOptional", {'transfer': 'ResponsesXPTransfer | None'}, total=False)
class ResponsesCreateTransferResponse(_ResponsesCreateTransferResponseOptional):
    pass

_ResponsesDepositAddressDetailsRespOptional = TypedDict("_ResponsesDepositAddressDetailsRespOptional", {'account': 'str | None', 'address': 'str | None', 'amount_credited': 'str | None', 'amount_received': 'str | None', 'created_at': 'int | Decimal | None', 'destination_tx_hash': 'str | None', 'expires_at': 'int | Decimal | None', 'fail_reason': 'str | None', 'fee': 'str | None', 'id': 'str | None', 'network': 'str | None', 'source_tx_hash': 'str | None', 'status': 'ResponsesDepositAddressStatus | None', 'token': 'str | None'}, total=False)
class ResponsesDepositAddressDetailsResp(_ResponsesDepositAddressDetailsRespOptional):
    pass

_ResponsesDepositAddressRespOptional = TypedDict("_ResponsesDepositAddressRespOptional", {'account': 'str | None', 'address': 'str | None', 'avg_completion_time': 'str | None', 'created_at': 'int | Decimal | None', 'expires_at': 'int | Decimal | None', 'id': 'str | None', 'max_amount': 'str | None', 'min_amount': 'str | None', 'network': 'str | None', 'status': 'ResponsesDepositAddressStatus | None', 'token': 'str | None'}, total=False)
class ResponsesDepositAddressResp(_ResponsesDepositAddressRespOptional):
    pass

_ResponsesDepositNetworkOptional = TypedDict("_ResponsesDepositNetworkOptional", {'max_amount': 'str | None', 'min_amount': 'str | None', 'network': 'str | None', 'token': 'str | None'}, total=False)
class ResponsesDepositNetwork(_ResponsesDepositNetworkOptional):
    pass

_ResponsesDepositNetworksRespOptional = TypedDict("_ResponsesDepositNetworksRespOptional", {'results': 'list[ResponsesDepositNetwork] | None'}, total=False)
class ResponsesDepositNetworksResp(_ResponsesDepositNetworksRespOptional):
    pass

_ResponsesDimeAllocationRespOptional = TypedDict("_ResponsesDimeAllocationRespOptional", {'addresses': 'list[str] | None', 'balance': 'str | None', 'category': 'str | None'}, total=False)
class ResponsesDimeAllocationResp(_ResponsesDimeAllocationRespOptional):
    pass

_ResponsesDimeConfigResponseOptional = TypedDict("_ResponsesDimeConfigResponseOptional", {'allocations': 'list[ResponsesDimeAllocationResp] | None', 'circulating_supply': 'str | None', 'staked_supply': 'str | None', 'total_supply': 'str | None'}, total=False)
class ResponsesDimeConfigResponse(_ResponsesDimeConfigResponseOptional):
    pass

_ResponsesDiscordProfileOptional = TypedDict("_ResponsesDiscordProfileOptional", {'id': 'str | None', 'image_url': 'str | None', 'username': 'str | None'}, total=False)
class ResponsesDiscordProfile(_ResponsesDiscordProfileOptional):
    pass

_ResponsesErrorResponseOptional = TypedDict("_ResponsesErrorResponseOptional", {'error': 'ResponsesErrorCode | None', 'message': 'str | None'}, total=False)
class ResponsesErrorResponse(_ResponsesErrorResponseOptional):
    pass

_ResponsesFeesOptional = TypedDict("_ResponsesFeesOptional", {'commission_rate': 'str | None', 'dated_option_maker_rate': 'str | None', 'dated_option_taker_rate': 'str | None', 'discount_rate': 'str | None', 'maker_rate': 'str | None', 'perp_option_maker_rate': 'str | None', 'perp_option_taker_rate': 'str | None', 'spot_maker_rate': 'str | None', 'spot_taker_rate': 'str | None', 'taker_rate': 'str | None'}, total=False)
class ResponsesFees(_ResponsesFeesOptional):
    pass

_ResponsesFillResultOptional = TypedDict("_ResponsesFillResultOptional", {'account': 'str | None', 'client_id': 'str | None', 'created_at': 'int | Decimal | None', 'fee': 'str | None', 'fee_currency': 'str | None', 'fill_type': 'ResponsesFillType | None', 'flags': 'list[ResponsesFillFlag] | None', 'id': 'str | None', 'leverage': 'str | None', 'liquidity': 'ResponsesTraderRole | None', 'market': 'str | None', 'order_id': 'str | None', 'orderbook_seq_no': 'int | Decimal | None', 'price': 'str | None', 'realized_funding': 'str | None', 'realized_pnl': 'str | None', 'remaining_size': 'str | None', 'side': 'ResponsesOrderSide | None', 'size': 'str | None', 'underlying_price': 'str | None'}, total=False)
class ResponsesFillResult(_ResponsesFillResultOptional):
    pass

_ResponsesGetAccountMarginConfigsRespOptional = TypedDict("_ResponsesGetAccountMarginConfigsRespOptional", {'account': 'str | None', 'configs': 'list[ResponsesAccountMarginEntry] | None', 'margin_methodology': 'str | None'}, total=False)
class ResponsesGetAccountMarginConfigsResp(_ResponsesGetAccountMarginConfigsRespOptional):
    pass

_ResponsesGetAccountMmpConfigsRespOptional = TypedDict("_ResponsesGetAccountMmpConfigsRespOptional", {'account': 'str | None', 'enabled': 'bool | None', 'results': 'list[ResponsesAccountMmpConfigResp] | None'}, total=False)
class ResponsesGetAccountMmpConfigsResp(_ResponsesGetAccountMmpConfigsRespOptional):
    pass

_ResponsesGetAccountsInfoResponseOptional = TypedDict("_ResponsesGetAccountsInfoResponseOptional", {'results': 'list[ResponsesAccountInfoResponse] | None'}, total=False)
class ResponsesGetAccountsInfoResponse(_ResponsesGetAccountsInfoResponseOptional):
    pass

_ResponsesGetOnboardingRespOptional = TypedDict("_ResponsesGetOnboardingRespOptional", {'account_signer_type': 'str | None', 'address': 'str | None', 'derivation_info': 'ResponsesOnboardingDerivationInfo | None', 'exists': 'bool | None'}, total=False)
class ResponsesGetOnboardingResp(_ResponsesGetOnboardingRespOptional):
    pass

_ResponsesGetSubAccountsResponseOptional = TypedDict("_ResponsesGetSubAccountsResponseOptional", {'results': 'list[ResponsesAccountInfoResponse] | None'}, total=False)
class ResponsesGetSubAccountsResponse(_ResponsesGetSubAccountsResponseOptional):
    pass

_ResponsesGetXPBalanceResponseV2Optional = TypedDict("_ResponsesGetXPBalanceResponseV2Optional", {'earned_xp': 'int | Decimal | None', 'transferrable_xp': 'int | Decimal | None', 'xp_delta': 'int | Decimal | None'}, total=False)
class ResponsesGetXPBalanceResponseV2(_ResponsesGetXPBalanceResponseV2Optional):
    pass

_ResponsesImpactPriceRespOptional = TypedDict("_ResponsesImpactPriceRespOptional", {'impact_price': 'str | None', 'market': 'str | None', 'side': 'str | None', 'size': 'str | None'}, total=False)
class ResponsesImpactPriceResp(_ResponsesImpactPriceRespOptional):
    pass

_ResponsesInsuranceAccountRespOptional = TypedDict("_ResponsesInsuranceAccountRespOptional", {'account': 'str | None', 'account_value': 'str | None', 'settlement_asset': 'str | None', 'updated_at': 'int | Decimal | None'}, total=False)
class ResponsesInsuranceAccountResp(_ResponsesInsuranceAccountRespOptional):
    pass

_ResponsesJwkKeyOptional = TypedDict("_ResponsesJwkKeyOptional", {'alg': 'str | None', 'crv': 'str | None', 'kid': 'str | None', 'kty': 'str | None', 'use': 'str | None', 'x': 'str | None', 'y': 'str | None'}, total=False)
class ResponsesJwkKey(_ResponsesJwkKeyOptional):
    pass

_ResponsesJwksResponseOptional = TypedDict("_ResponsesJwksResponseOptional", {'keys': 'list[ResponsesJwkKey] | None'}, total=False)
class ResponsesJwksResponse(_ResponsesJwksResponseOptional):
    pass

_ResponsesMarketEventOptional = TypedDict("_ResponsesMarketEventOptional", {'changed_fields': 'ResponsesMarketHistoryFields | None', 'event_type': 'ResponsesMarketEventType | None', 'symbol': 'str | None', 'updated_at': 'int | Decimal | None'}, total=False)
class ResponsesMarketEvent(_ResponsesMarketEventOptional):
    pass

_ResponsesMarketHistoryFieldsOptional = TypedDict("_ResponsesMarketHistoryFieldsOptional", {'asset_kind': 'str | None', 'base_currency': 'str | None', 'clamp_rate': 'int | Decimal | None', 'delta1_cross_margin_params': 'Any | None', 'expiry_at': 'int | Decimal | None', 'fee_config': 'Any | None', 'funding_multiplier': 'int | Decimal | None', 'funding_period_hours': 'int | Decimal | None', 'interest_rate': 'int | Decimal | None', 'market_kind': 'str | None', 'max_funding_rate': 'int | Decimal | None', 'max_funding_rate_change': 'int | Decimal | None', 'max_open_orders': 'int | Decimal | None', 'max_order_size': 'str | None', 'max_slippage': 'int | Decimal | None', 'max_tob_spread': 'int | Decimal | None', 'min_notional': 'str | None', 'open_at': 'int | Decimal | None', 'option_cross_margin_params': 'Any | None', 'option_type': 'str | None', 'oracle_ewma_factor': 'int | Decimal | None', 'order_size_increment': 'str | None', 'position_limit': 'str | None', 'price_bands_width': 'int | Decimal | None', 'price_tick_size': 'str | None', 'quote_currency': 'str | None', 'settlement_currency': 'str | None', 'settlement_price': 'int | Decimal | None', 'strike_price': 'str | None', 'underlying_twap_price': 'int | Decimal | None'}, total=False)
class ResponsesMarketHistoryFields(_ResponsesMarketHistoryFieldsOptional):
    pass

_ResponsesMarketHistoryResponseOptional = TypedDict("_ResponsesMarketHistoryResponseOptional", {'results': 'list[ResponsesMarketEvent] | None'}, total=False)
class ResponsesMarketHistoryResponse(_ResponsesMarketHistoryResponseOptional):
    pass

_ResponsesMarketSettlementRespOptional = TypedDict("_ResponsesMarketSettlementRespOptional", {'expiry_at': 'int | Decimal | None', 'settlement_price': 'str | None', 'symbol': 'str | None', 'underlying_twap_price': 'str | None'}, total=False)
class ResponsesMarketSettlementResp(_ResponsesMarketSettlementRespOptional):
    pass

_ResponsesNftOptional = TypedDict("_ResponsesNftOptional", {'collection_address': 'str | None', 'collection_name': 'str | None', 'id': 'str | None', 'image_url': 'str | None', 'name': 'str | None', 'price': 'Messagesv1NftPrice | None'}, total=False)
class ResponsesNft(_ResponsesNftOptional):
    pass

_ResponsesNotificationPreferencesRespOptional = TypedDict("_ResponsesNotificationPreferencesRespOptional", {'announcements': 'bool | None', 'fill_sound': 'bool | None', 'fills': 'bool | None', 'orders': 'bool | None', 'transfers': 'bool | None'}, total=False)
class ResponsesNotificationPreferencesResp(_ResponsesNotificationPreferencesRespOptional):
    pass

_ResponsesOnboardingDerivationInfoOptional = TypedDict("_ResponsesOnboardingDerivationInfoOptional", {'class_hash': 'str | None', 'proxy_class_hash': 'str | None'}, total=False)
class ResponsesOnboardingDerivationInfo(_ResponsesOnboardingDerivationInfoOptional):
    pass

_ResponsesOrderRespOptional = TypedDict("_ResponsesOrderRespOptional", {'account': 'str | None', 'avg_fill_price': 'str | None', 'cancel_reason': 'str | None', 'client_id': 'str | None', 'created_at': 'int | Decimal | None', 'flags': 'list[ResponsesOrderFlag] | None', 'id': 'str | None', 'instruction': 'ResponsesOrderInstruction | None', 'last_updated_at': 'int | Decimal | None', 'market': 'str | None', 'price': 'str | None', 'published_at': 'int | Decimal | None', 'received_at': 'int | Decimal | None', 'remaining_size': 'str | None', 'request_info': 'ResponsesRequestInfo | None', 'seq_no': 'int | Decimal | None', 'side': 'ResponsesOrderSide | None', 'size': 'str | None', 'status': 'ResponsesOrderStatus | None', 'stp': 'ResponsesSTPMode | None', 'timestamp': 'int | Decimal | None', 'trigger_price': 'str | None', 'type': 'ResponsesOrderType | None'}, total=False)
class ResponsesOrderResp(_ResponsesOrderRespOptional):
    pass

_ResponsesParadigmEnableResponseOptional = TypedDict("_ResponsesParadigmEnableResponseOptional", {'account': 'str | None', 'status': 'str | None', 'subkey_public_key': 'str | None'}, total=False)
class ResponsesParadigmEnableResponse(_ResponsesParadigmEnableResponseOptional):
    pass

_ResponsesReferralConfigRespOptional = TypedDict("_ResponsesReferralConfigRespOptional", {'commission_rate': 'str | None', 'commission_volume_cap': 'str | None', 'discount_rate': 'str | None', 'discount_volume_cap': 'str | None', 'minimum_volume': 'str | None', 'name': 'str | None', 'points_bonus_rate': 'str | None', 'points_bonus_volume_cap': 'str | None', 'referral_type': 'str | None'}, total=False)
class ResponsesReferralConfigResp(_ResponsesReferralConfigRespOptional):
    pass

_ResponsesRefreshInventoryRespOptional = TypedDict("_ResponsesRefreshInventoryRespOptional", {'nfts': 'list[ResponsesNft] | None'}, total=False)
class ResponsesRefreshInventoryResp(_ResponsesRefreshInventoryRespOptional):
    pass

_ResponsesRequestInfoOptional = TypedDict("_ResponsesRequestInfoOptional", {'id': 'str | None', 'message': 'str | None', 'request_type': 'str | None', 'status': 'str | None'}, total=False)
class ResponsesRequestInfo(_ResponsesRequestInfoOptional):
    pass

_ResponsesRevokeSubkeyResponseOptional = TypedDict("_ResponsesRevokeSubkeyResponseOptional", {'public_key': 'str | None'}, total=False)
class ResponsesRevokeSubkeyResponse(_ResponsesRevokeSubkeyResponseOptional):
    pass

_ResponsesRevokeTokenResponseOptional = TypedDict("_ResponsesRevokeTokenResponseOptional", {'lookup_id': 'str | None'}, total=False)
class ResponsesRevokeTokenResponse(_ResponsesRevokeTokenResponseOptional):
    pass

_ResponsesRfqAcceptedResponseOptional = TypedDict("_ResponsesRfqAcceptedResponseOptional", {'accepted_at': 'int | Decimal | None', 'request_id': 'str | None'}, total=False)
class ResponsesRfqAcceptedResponse(_ResponsesRfqAcceptedResponseOptional):
    pass

_ResponsesRfqBboLegResultOptional = TypedDict("_ResponsesRfqBboLegResultOptional", {'best_ask_iv': 'str | None', 'best_ask_price': 'str | None', 'best_ask_quantity': 'str | None', 'best_bid_iv': 'str | None', 'best_bid_price': 'str | None', 'best_bid_quantity': 'str | None', 'mark_price': 'str | None', 'mark_price_iv': 'str | None', 'market': 'str | None'}, total=False)
class ResponsesRfqBboLegResult(_ResponsesRfqBboLegResultOptional):
    pass

_ResponsesRfqBboResultOptional = TypedDict("_ResponsesRfqBboResultOptional", {'best_ask_price': 'str | None', 'best_ask_quantity': 'str | None', 'best_bid_price': 'str | None', 'best_bid_quantity': 'str | None', 'created_at': 'int | Decimal | None', 'legs': 'list[ResponsesRfqBboLegResult] | None', 'mark_price': 'str | None', 'max_price': 'str | None', 'min_price': 'str | None', 'rfq_id': 'str | None'}, total=False)
class ResponsesRfqBboResult(_ResponsesRfqBboResultOptional):
    pass

_ResponsesRfqDraftResultOptional = TypedDict("_ResponsesRfqDraftResultOptional", {'created_at': 'int | Decimal | None', 'id': 'str | None', 'legs': 'list[ResponsesRfqLegResult] | None', 'quantity': 'str | None', 'strategy': 'str | None', 'updated_at': 'int | Decimal | None'}, total=False)
class ResponsesRfqDraftResult(_ResponsesRfqDraftResultOptional):
    pass

_ResponsesRfqLegResultOptional = TypedDict("_ResponsesRfqLegResultOptional", {'market': 'str | None', 'price': 'str | None', 'ratio': 'str | None', 'side': 'str | None'}, total=False)
class ResponsesRfqLegResult(_ResponsesRfqLegResultOptional):
    pass

_ResponsesRfqMarketInfoOptional = TypedDict("_ResponsesRfqMarketInfoOptional", {'symbol': 'str | None', 'venue_size_increment': 'str | None'}, total=False)
class ResponsesRfqMarketInfo(_ResponsesRfqMarketInfoOptional):
    pass

_ResponsesRfqMarketsResultOptional = TypedDict("_ResponsesRfqMarketsResultOptional", {'markets': 'list[str] | None', 'markets_info': 'list[ResponsesRfqMarketInfo] | None'}, total=False)
class ResponsesRfqMarketsResult(_ResponsesRfqMarketsResultOptional):
    pass

_ResponsesRfqPlatformStateResultOptional = TypedDict("_ResponsesRfqPlatformStateResultOptional", {'next_platform_state': 'ResponsesRfqVenueState | None', 'next_platform_state_at': 'int | Decimal | None', 'platform_state': 'ResponsesRfqVenueState | None', 'state': 'ResponsesRfqVenueState | None', 'venue_state': 'ResponsesRfqVenueState | None'}, total=False)
class ResponsesRfqPlatformStateResult(_ResponsesRfqPlatformStateResultOptional):
    pass

_ResponsesRfqResultOptional = TypedDict("_ResponsesRfqResultOptional", {'closed_reason': 'str | None', 'counterparties': 'list[str] | None', 'created_at': 'int | Decimal | None', 'expires_at': 'int | Decimal | None', 'filled_quantity': 'str | None', 'id': 'str | None', 'label': 'str | None', 'legs': 'list[ResponsesRfqLegResult] | None', 'mark_price': 'str | None', 'quantity': 'str | None', 'state': 'str | None'}, total=False)
class ResponsesRfqResult(_ResponsesRfqResultOptional):
    pass

_ResponsesRfqResultsOptional = TypedDict("_ResponsesRfqResultsOptional", {'next': 'str | None', 'results': 'list[ResponsesRfqResult] | None'}, total=False)
class ResponsesRfqResults(_ResponsesRfqResultsOptional):
    pass

_ResponsesShareRateRespOptional = TypedDict("_ResponsesShareRateRespOptional", {'last_updated_at': 'int | Decimal | None', 'share_rate': 'str | None'}, total=False)
class ResponsesShareRateResp(_ResponsesShareRateRespOptional):
    pass

_ResponsesStakingConfigResponseOptional = TypedDict("_ResponsesStakingConfigResponseOptional", {'dime_fee_discount_pct': 'str | None', 'tiers': 'list[ResponsesStakingTierResp] | None'}, total=False)
class ResponsesStakingConfigResponse(_ResponsesStakingConfigResponseOptional):
    pass

_ResponsesStakingSummaryResponseOptional = TypedDict("_ResponsesStakingSummaryResponseOptional", {'account': 'str | None', 'active_stakes': 'int | Decimal | None', 'staking_tier': 'str | None', 'total_staked': 'str | None'}, total=False)
class ResponsesStakingSummaryResponse(_ResponsesStakingSummaryResponseOptional):
    pass

_ResponsesStakingTierRespOptional = TypedDict("_ResponsesStakingTierRespOptional", {'fee_rates': 'dict[str, str] | None', 'min_staked': 'str | None', 'tier_name': 'str | None', 'xp_multiplier': 'str | None'}, total=False)
class ResponsesStakingTierResp(_ResponsesStakingTierRespOptional):
    pass

_ResponsesStrategyOptional = TypedDict("_ResponsesStrategyOptional", {'address': 'str | None', 'name': 'str | None', 'visibility': 'ResponsesVisibility | None'}, total=False)
class ResponsesStrategy(_ResponsesStrategyOptional):
    pass

_ResponsesSubkeyOptional = TypedDict("_ResponsesSubkeyOptional", {'account_id': 'str | None', 'allowed_cidrs': 'list[str] | None', 'block_trade_signing_enabled': 'bool | None', 'created_at': 'int | Decimal | None', 'id': 'int | Decimal | None', 'name': 'str | None', 'public_key': 'str | None', 'revoked_at': 'int | Decimal | None', 'state': 'str | None', 'updated_at': 'int | Decimal | None'}, total=False)
class ResponsesSubkey(_ResponsesSubkeyOptional):
    pass

_ResponsesSystemConfigResponseOptional = TypedDict("_ResponsesSystemConfigResponseOptional", {'assets_limit_per_account': 'int | Decimal | None', 'block_explorer_url': 'str | None', 'bridged_tokens': 'list[ResponsesBridgedToken] | None', 'dime_staking_contract_address': 'str | None', 'environment': 'str | None', 'l1_chain_id': 'str | None', 'l1_core_contract_address': 'str | None', 'l1_operator_address': 'str | None', 'l1_relayer_address': 'str | None', 'l2_relayer_address': 'str | None', 'liquidation_fee': 'str | None', 'oracle_address': 'str | None', 'paraclear_account_hash': 'str | None', 'paraclear_account_proxy_hash': 'str | None', 'paraclear_address': 'str | None', 'paraclear_decimals': 'int | Decimal | None', 'paraclear_evm_account_hash': 'str | None', 'partial_liquidation_buffer': 'str | None', 'partial_liquidation_share_increment': 'str | None', 'rfq_default_max_slippage': 'str | None', 'starknet_chain_id': 'str | None', 'starknet_fullnode_rpc_base_url': 'str | None', 'starknet_fullnode_rpc_url': 'str | None', 'starknet_gateway_url': 'str | None', 'universal_deployer_address': 'str | None'}, total=False)
class ResponsesSystemConfigResponse(_ResponsesSystemConfigResponseOptional):
    pass

_ResponsesSystemStateResponseOptional = TypedDict("_ResponsesSystemStateResponseOptional", {'status': 'ResponsesSystemStatus | None'}, total=False)
class ResponsesSystemStateResponse(_ResponsesSystemStateResponseOptional):
    pass

_ResponsesSystemStatsResponseOptional = TypedDict("_ResponsesSystemStatsResponseOptional", {'lifetime_volume': 'str | None'}, total=False)
class ResponsesSystemStatsResponse(_ResponsesSystemStatsResponseOptional):
    pass

_ResponsesSystemTimeResponseOptional = TypedDict("_ResponsesSystemTimeResponseOptional", {'server_time': 'str | None'}, total=False)
class ResponsesSystemTimeResponse(_ResponsesSystemTimeResponseOptional):
    pass

_ResponsesTwitterProfileOptional = TypedDict("_ResponsesTwitterProfileOptional", {'id': 'str | None', 'image_url': 'str | None', 'username': 'str | None'}, total=False)
class ResponsesTwitterProfile(_ResponsesTwitterProfileOptional):
    pass

_ResponsesUpdateAccountMarginConfigRespOptional = TypedDict("_ResponsesUpdateAccountMarginConfigRespOptional", {'account': 'str | None', 'leverage': 'int | Decimal | None', 'margin_type': 'str | None', 'market': 'str | None'}, total=False)
class ResponsesUpdateAccountMarginConfigResp(_ResponsesUpdateAccountMarginConfigRespOptional):
    pass

_ResponsesVaultAnalyticsOptional = TypedDict("_ResponsesVaultAnalyticsOptional", {'address': 'str | None', 'annualized_volatility': 'ResponsesWindowedMetrics | None', 'apr': 'ResponsesWindowedMetrics | None', 'as_of': 'str | None', 'best_period_return': 'ResponsesWindowedMetrics | None', 'max_drawdown': 'ResponsesWindowedMetrics | None', 'monthly_pnl_last_12m': 'list[ResponsesVaultMonthlyPnlEntry] | None', 'pct_positive': 'ResponsesWindowedMetrics | None', 'pnl': 'ResponsesWindowedMetrics | None', 'recovery_hours': 'ResponsesWindowedMetrics | None', 'sharpe_ratio': 'ResponsesWindowedMetrics | None', 'top_holders': 'list[ResponsesVaultTopHolder] | None', 'worst_period_return': 'ResponsesWindowedMetrics | None'}, total=False)
class ResponsesVaultAnalytics(_ResponsesVaultAnalyticsOptional):
    pass

_ResponsesVaultAnalyticsRespOptional = TypedDict("_ResponsesVaultAnalyticsRespOptional", {'address': 'str | None', 'as_of': 'str | None', 'substrategies': 'list[ResponsesAccountAnalytics] | None', 'vault_level': 'ResponsesVaultAnalytics | None'}, total=False)
class ResponsesVaultAnalyticsResp(_ResponsesVaultAnalyticsRespOptional):
    pass

_ResponsesVaultMonthlyPnlEntryOptional = TypedDict("_ResponsesVaultMonthlyPnlEntryOptional", {'month': 'str | None', 'month_return_pct': 'str | None'}, total=False)
class ResponsesVaultMonthlyPnlEntry(_ResponsesVaultMonthlyPnlEntryOptional):
    pass

_ResponsesVaultRespOptional = TypedDict("_ResponsesVaultRespOptional", {'address': 'str | None', 'created_at': 'int | Decimal | None', 'curator_name': 'str | None', 'description': 'str | None', 'kind': 'ResponsesVaultKind | None', 'last_updated_at': 'int | Decimal | None', 'lockup_period': 'int | Decimal | None', 'max_tvl': 'int | Decimal | None', 'name': 'str | None', 'operator_account': 'str | None', 'owner_account': 'str | None', 'profit_share': 'int | Decimal | None', 'status': 'ResponsesVaultStatus | None', 'strategies': 'list[ResponsesStrategy] | None', 'strategy_description': 'str | None', 'token_address': 'str | None', 'visibility': 'ResponsesVisibility | None'}, total=False)
class ResponsesVaultResp(_ResponsesVaultRespOptional):
    pass

_ResponsesVaultTopHolderOptional = TypedDict("_ResponsesVaultTopHolderOptional", {'percentage': 'str | None', 'rank': 'int | Decimal | None', 'value_usdc': 'str | None'}, total=False)
class ResponsesVaultTopHolder(_ResponsesVaultTopHolderOptional):
    pass

_ResponsesVaultsConfigResponseOptional = TypedDict("_ResponsesVaultsConfigResponseOptional", {'max_lockup_period_days': 'str | None', 'max_profit_share_percentage': 'str | None', 'min_initial_deposit': 'str | None', 'min_lockup_period_days': 'str | None', 'min_owner_share_percentage': 'str | None', 'profit_share_period': 'str | None', 'vault_factory_address': 'str | None'}, total=False)
class ResponsesVaultsConfigResponse(_ResponsesVaultsConfigResponseOptional):
    pass

_ResponsesVisibilityOptional = TypedDict("_ResponsesVisibilityOptional", {'balances': 'bool | None', 'positions': 'bool | None'}, total=False)
class ResponsesVisibility(_ResponsesVisibilityOptional):
    pass

_ResponsesVolumeTierKindInfoOptional = TypedDict("_ResponsesVolumeTierKindInfoOptional", {'tier': 'str | None', 'volume': 'str | None'}, total=False)
class ResponsesVolumeTierKindInfo(_ResponsesVolumeTierKindInfoOptional):
    pass

_ResponsesWindowedMetricsOptional = TypedDict("_ResponsesWindowedMetricsOptional", {'window_1y': 'str | None', 'window_24h': 'str | None', 'window_30d': 'str | None', 'window_7d': 'str | None', 'window_90d': 'str | None', 'window_all': 'str | None'}, total=False)
class ResponsesWindowedMetrics(_ResponsesWindowedMetricsOptional):
    pass

_ResponsesWithdrawalLockRejectionResponseOptional = TypedDict("_ResponsesWithdrawalLockRejectionResponseOptional", {'amount': 'str | None', 'reason': 'str | None', 'rejected_at': 'int | Decimal | None', 'request_id': 'str | None'}, total=False)
class ResponsesWithdrawalLockRejectionResponse(_ResponsesWithdrawalLockRejectionResponseOptional):
    pass

_ResponsesWithdrawalLockResponseOptional = TypedDict("_ResponsesWithdrawalLockResponseOptional", {'amount': 'str | None', 'expires_at': 'int | Decimal | None', 'request_id': 'str | None', 'status': 'str | None'}, total=False)
class ResponsesWithdrawalLockResponse(_ResponsesWithdrawalLockResponseOptional):
    pass

_ResponsesXPTransferOptional = TypedDict("_ResponsesXPTransferOptional", {'amount': 'int | Decimal | None', 'created_at': 'int | Decimal | None', 'fee': 'int | Decimal | None', 'id': 'str | None', 'is_private': 'bool | None', 'recipient': 'str | None', 'season': 'str | None', 'sender': 'str | None'}, total=False)
class ResponsesXPTransfer(_ResponsesXPTransferOptional):
    pass

class RevokeSubkeyParams(TypedDict):
    public_key: str

class RevokeTokenParams(TypedDict):
    lookup_id: str

class RfqBboParams(TypedDict):
    rfq_id: str

class RfqCreateParams(TypedDict):
    body: RequestsRfqCreateRequest

class RfqDeleteParams(TypedDict):
    rfq_id: str

class RfqDraftCreateParams(TypedDict):
    body: RequestsRfqCreateRequest

class RfqDraftDeleteParams(TypedDict):
    draft_id: str

class RfqDraftListParams(TypedDict):
    pass

class RfqEnableParams(TypedDict):
    pass

class RfqExecuteParams(TypedDict):
    rfq_id: str
    body: RequestsRfqExecuteRequest

_RfqListParamsOptional = TypedDict("_RfqListParamsOptional", {'state': 'str | None', 'cursor': 'str | None', 'page_size': 'int | Decimal | None'}, total=False)
class RfqListParams(_RfqListParamsOptional):
    pass

class RfqMarketsParams(TypedDict):
    pass

class RfqPlatformStateParams(TypedDict):
    pass

_ServicesTransferFeeConfigResponseOptional = TypedDict("_ServicesTransferFeeConfigResponseOptional", {'fee_percentage': 'int | Decimal | None'}, total=False)
class ServicesTransferFeeConfigResponse(_ServicesTransferFeeConfigResponseOptional):
    pass

_TradebustsParamsOptional = TypedDict("_TradebustsParamsOptional", {'cursor': 'str | None', 'end_at': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'start_at': 'int | Decimal | None'}, total=False)
class TradebustsParams(_TradebustsParamsOptional):
    pass

_TradesParamsOptional = TypedDict("_TradesParamsOptional", {'base_asset': 'str | None', 'cursor': 'str | None', 'end_at': 'int | Decimal | None', 'market': 'str | None', 'page_size': 'int | Decimal | None', 'start_at': 'int | Decimal | None', 'trade_type': 'str | None'}, total=False)
class TradesParams(_TradesParamsOptional):
    pass

_TransactionsParamsOptional = TypedDict("_TransactionsParamsOptional", {'cursor': 'str | None', 'end_at': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'start_at': 'int | Decimal | None'}, total=False)
class TransactionsParams(_TransactionsParamsOptional):
    pass

class UpdateAccountMarketMaxSlippageParams(TypedDict):
    market: str
    body: RequestsUpdateAccountMaxSlippageRequest

class UpdateAccountMaxSlippageParams(TypedDict):
    body: UpdateAccountMaxSlippageParamsBody

class UpdateAccountMaxSlippageParamsBody(TypedDict):
    max_slippage: str

class UpdateAccountNotificationPreferencesParams(TypedDict):
    body: RequestsUpdateNotificationPreferencesRequest

class UpdateAccountProfileParams(TypedDict):
    body: RequestsUpdateAccountProfileRequest

class UpdateAccountProfileReferralCodeParams(TypedDict):
    pass

class UpdateAccountProfileUsernameParams(TypedDict):
    pass

class UpdateAccountReferrerParams(TypedDict):
    pass

class UpdateAccountSizeCurrencyDisplayParams(TypedDict):
    body: RequestsUpdateSizeCurrencyDisplayRequest

class UpdateAccountTradingValueDisplayParams(TypedDict):
    body: RequestsUpdateTradingValueDisplayRequest

class UpdateLastSeenParams(TypedDict):
    pass

class UpdateSubkeyAllowedCidrsParams(TypedDict):
    public_key: str
    body: RequestsUpdateAllowedCIDRs

class UpdateSubkeyBlockTradeSigningParams(TypedDict):
    public_key: str
    body: RequestsUpdateSubkeyBlockTradeSigning

class UpdateSubkeyParams(TypedDict):
    public_key: str
    body: RequestsUpdateSubkey

class UpdateTokenAllowedCidrsParams(TypedDict):
    lookup_id: str
    body: RequestsUpdateAllowedCIDRs

class UpsertAccountMarginParams(TypedDict):
    market: str
    body: RequestsAccountMarginRequest

class UpsertAccountMmpParams(TypedDict):
    base_asset: str
    body: RequestsAccountMmpConfigRequest

_VaultsCreateParamsWireKeys = TypedDict("_VaultsCreateParamsWireKeys", {'PARADEX-PARENT-ACCOUNT': 'str', 'PARADEX-STARKNET-SIGNATURE': 'str', 'PARADEX-STARKNET-ACCOUNT': 'str', 'PARADEX-STARKNET-ACCOUNT-DERIVATION-PATH': 'str'})
class VaultsCreateParams(_VaultsCreateParamsWireKeys):
    body: RequestsCreateVault

_VaultsGetAccountSummaryParamsOptional = TypedDict("_VaultsGetAccountSummaryParamsOptional", {'address': 'str | None'}, total=False)
class VaultsGetAccountSummaryParams(_VaultsGetAccountSummaryParamsOptional):
    pass

class VaultsGetAnalyticsParams(TypedDict):
    address: str

_VaultsGetBalanceParamsOptional = TypedDict("_VaultsGetBalanceParamsOptional", {'strategy': 'str | None'}, total=False)
class VaultsGetBalanceParams(_VaultsGetBalanceParamsOptional):
    address: str

class VaultsGetConfigParams(TypedDict):
    pass

_VaultsGetHistoricalDataParamsOptional = TypedDict("_VaultsGetHistoricalDataParamsOptional", {'strategy': 'str | None'}, total=False)
class VaultsGetHistoricalDataParams(_VaultsGetHistoricalDataParamsOptional):
    address: str
    type: str
    resolution: str

class VaultsGetMineParams(TypedDict):
    pass

_VaultsGetParamsOptional = TypedDict("_VaultsGetParamsOptional", {'address': 'str | None'}, total=False)
class VaultsGetParams(_VaultsGetParamsOptional):
    pass

_VaultsGetPositionsParamsOptional = TypedDict("_VaultsGetPositionsParamsOptional", {'strategy': 'str | None'}, total=False)
class VaultsGetPositionsParams(_VaultsGetPositionsParamsOptional):
    address: str

_VaultsGetSummaryParamsOptional = TypedDict("_VaultsGetSummaryParamsOptional", {'address': 'str | None'}, total=False)
class VaultsGetSummaryParams(_VaultsGetSummaryParamsOptional):
    pass

class VaultsGetTransfersParams(TypedDict):
    address: str

ResponsesAccountKind = str
ResponsesAlgoType = str
ResponsesBlockTradeStatus = str
ResponsesBlockTradeType = str
ResponsesDepositAddressStatus = str
ResponsesErrorCode = str
ResponsesFillFlag = str
ResponsesFillType = str
ResponsesMarketEventType = str
ResponsesOrderFlag = str
ResponsesOrderInstruction = str
ResponsesOrderSide = str
ResponsesOrderStatus = str
ResponsesOrderType = str
ResponsesRfqVenueState = str
ResponsesSTPMode = str
ResponsesSystemStatus = str
ResponsesTraderRole = str
ResponsesVaultKind = str
ResponsesVaultStatus = str
