"""Derive REST wire models generated from schema/derive.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_AccountFeeInfoOptional = TypedDict("_AccountFeeInfoOptional", {'option_maker_fee': 'str | None', 'option_taker_fee': 'str | None', 'perp_maker_fee': 'str | None', 'perp_taker_fee': 'str | None', 'spot_maker_fee': 'str | None', 'spot_taker_fee': 'str | None'}, total=False)
class AccountFeeInfo(_AccountFeeInfoOptional):
    base_fee_discount: str
    rfq_maker_discount: str
    rfq_taker_discount: str

class Action(TypedDict):
    data: list[int | Decimal]
    expiry: int | Decimal
    module: Address
    nonce: int | Decimal
    owner: Address
    signer: Address
    subaccount_id: int | Decimal

class AggregatedOrdersResult(TypedDict):
    orders: list[Order]
    subaccount_id: int | Decimal

class AggregatedTriggerOrdersResult(TypedDict):
    orders: list[Order]
    subaccount_id: int | Decimal

class AssetEntry(TypedDict):
    address: str
    name: str
    universes: list[AssetUniverse]

_AssetUniverseOptional = TypedDict("_AssetUniverseOptional", {'risk_universe_name': 'str | None', 'srm_perp_margin_requirements': 'SrmPerpMarginRequirements | None'}, total=False)
class AssetUniverse(_AssetUniverseOptional):
    oi: OpenInterestStats
    risk_universe_id: int | Decimal

class AuctionBidEvent(TypedDict):
    amounts_liquidated: dict[str, str]
    bidder_id: int | Decimal
    cash_received: str
    discount_pnl: str
    operation_id: str
    percent_liquidated: str
    positions_realized_pnl: dict[str, str]
    positions_realized_pnl_excl_fees: dict[str, str]
    realized_pnl: str
    realized_pnl_excl_fees: str
    risk_universe_id: int | Decimal
    timestamp: int | Decimal
    tx_hash: str
    unit_price: str

_AuctionHistoryOptional = TypedDict("_AuctionHistoryOptional", {'end_timestamp': 'int | Decimal | None'}, total=False)
class AuctionHistory(_AuctionHistoryOptional):
    auction_id: str
    auction_type: AuctionType
    bids: list[AuctionBidEvent]
    fee: str
    risk_universe_id: int | Decimal
    start_timestamp: int | Decimal
    subaccount_id: int | Decimal
    tx_hash: str

_BurnSharesRequestOptional = TypedDict("_BurnSharesRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class BurnSharesRequest(_BurnSharesRequestOptional):
    request_id: VaultRequestId
    share_price: str
    subaccount_id: int | Decimal
    withdraw_hash: str

class CancelAlgoOrderRequest(TypedDict):
    order_id: str
    subaccount_id: int | Decimal

class CancelAllAlgoOrdersRequest(TypedDict):
    subaccount_id: int | Decimal

_CancelAllRequestOptional = TypedDict("_CancelAllRequestOptional", {'cancel_algo_orders': 'bool | None', 'cancel_trigger_orders': 'bool | None'}, total=False)
class CancelAllRequest(_CancelAllRequestOptional):
    subaccount_id: int | Decimal

class CancelAllTriggerOrdersRequest(TypedDict):
    subaccount_id: int | Decimal

_CancelBatchQuotesRequestOptional = TypedDict("_CancelBatchQuotesRequestOptional", {'label': 'str | None', 'nonce': 'str | None', 'quote_id': 'str | None', 'rfq_id': 'str | None'}, total=False)
class CancelBatchQuotesRequest(_CancelBatchQuotesRequestOptional):
    subaccount_id: int | Decimal

class CancelBatchResult(TypedDict):
    cancelled_ids: list[str]

_CancelBatchRfqsRequestOptional = TypedDict("_CancelBatchRfqsRequestOptional", {'label': 'str | None', 'nonce': 'str | None', 'rfq_id': 'str | None'}, total=False)
class CancelBatchRfqsRequest(_CancelBatchRfqsRequestOptional):
    subaccount_id: int | Decimal

class CancelBatchRfqsResponse(TypedDict):
    cancelled_ids: list[str]

class CancelByInstrumentRequest(TypedDict):
    instrument_name: str
    subaccount_id: int | Decimal

class CancelByInstrumentResponse(TypedDict):
    cancelled_orders: int | Decimal

_CancelByLabelRequestOptional = TypedDict("_CancelByLabelRequestOptional", {'instrument_name': 'str | None'}, total=False)
class CancelByLabelRequest(_CancelByLabelRequestOptional):
    label: str
    subaccount_id: int | Decimal

class CancelByLabelResponse(TypedDict):
    cancelled_orders: int | Decimal

_CancelByNonceRequestOptional = TypedDict("_CancelByNonceRequestOptional", {'nonce': 'str'}, total=False)
class CancelByNonceRequest(_CancelByNonceRequestOptional):
    instrument_name: str
    subaccount_id: int | Decimal

class CancelByNonceResponse(TypedDict):
    cancelled_orders: int | Decimal

class CancelOrderRequest(TypedDict):
    instrument_name: str
    order_id: str
    subaccount_id: int | Decimal

_CancelQuoteRequestOptional = TypedDict("_CancelQuoteRequestOptional", {'label': 'str | None', 'nonce': 'str | None', 'rfq_id': 'str | None'}, total=False)
class CancelQuoteRequest(_CancelQuoteRequestOptional):
    quote_id: str
    subaccount_id: int | Decimal

class CancelRfqRequest(TypedDict):
    rfq_id: str
    subaccount_id: int | Decimal

class CancelTriggerOrderRequest(TypedDict):
    order_id: str
    subaccount_id: int | Decimal

_CancelVaultRequestRequestOptional = TypedDict("_CancelVaultRequestRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class CancelVaultRequestRequest(_CancelVaultRequestRequestOptional):
    subaccount_id: int | Decimal
    vault_subaccount_id: int | Decimal

class ChangeSubaccountLabelRequest(TypedDict):
    label: str
    subaccount_id: int | Decimal

class Collateral(TypedDict):
    amount: str
    amount_step: str
    asset_name: str
    asset_type: str
    average_price: str
    average_price_excl_fees: str
    creation_timestamp: int | Decimal
    cumulative_interest: str
    currency: str
    delta: str
    delta_currency: str
    initial_margin: str
    maintenance_margin: str
    mark_price: str
    mark_value: str
    open_orders_margin: str
    pending_interest: str
    realized_pnl: str
    realized_pnl_excl_fees: str
    total_fees: str
    unrealized_pnl: str
    unrealized_pnl_excl_fees: str

_CreateOrderRequestOptional = TypedDict("_CreateOrderRequestOptional", {'algo_duration_sec': 'int | Decimal | None', 'algo_num_slices': 'int | Decimal | None', 'algo_type': 'AlgoType | None', 'client': 'str | None', 'extra_fee': 'str | None', 'is_atomic_signing': 'bool | None', 'label': 'str | None', 'mmp': 'bool | None', 'nonce': 'str', 'order_type': 'OrderType | None', 'reduce_only': 'bool | None', 'referral_code': 'str | None', 'reject_post_only': 'bool | None', 'reject_timestamp': 'int | Decimal | None', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'str', 'time_in_force': 'TimeInForce | None', 'trigger_price': 'str | None', 'trigger_price_type': 'TriggerPriceType | None', 'trigger_type': 'TriggerType | None'}, total=False)
class CreateOrderRequest(_CreateOrderRequestOptional):
    amount: str
    direction: Direction
    instrument_name: str
    limit_price: str
    max_fee: str
    subaccount_id: int | Decimal

_CreateVaultRequestOptional = TypedDict("_CreateVaultRequestOptional", {'benchmark_asset': 'Address | None', 'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class CreateVaultRequest(_CreateVaultRequestOptional):
    cooldown_sec: int | Decimal
    deposit_spot_asset: Address
    initial_deposit: str
    initial_share_price_usd: str
    management_fee_bps: int | Decimal
    manager_id: int | Decimal
    max_fee_usd: str
    max_slippage_bps: int | Decimal
    performance_fee_bps: int | Decimal
    subaccount_id: int | Decimal

_CurrencyOptional = TypedDict("_CurrencyOptional", {'option': 'AssetEntry | None', 'perp': 'AssetEntry | None', 'spot_price_24h': 'str | None'}, total=False)
class Currency(_CurrencyOptional):
    currency: str
    managers: list[UniverseManagers]
    market_type: MarketType
    spot: list[SpotAssetEntry]
    spot_price: str

class DailyTradingStatistics(TypedDict):
    c: str
    h: str
    l: str
    n: int | Decimal
    oi: str
    p: str
    pr: str
    v: str

class DecodeActionRequest(TypedDict):
    action_type: DecodableActionType
    encoded_data: str

class DecodeActionResponse(TypedDict):
    action_type: DecodableActionType
    decoded: Any

_DeleteSubaccountRequestOptional = TypedDict("_DeleteSubaccountRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'str'}, total=False)
class DeleteSubaccountRequest(_DeleteSubaccountRequestOptional):
    subaccount_id: int | Decimal

class DeleteSubaccountResponse(TypedDict):
    op_uuid: str
    operation_id: int | Decimal

_DepositBridgeOriginOptional = TypedDict("_DepositBridgeOriginOptional", {'source_address': 'str | None'}, total=False)
class DepositBridgeOrigin(_DepositBridgeOriginOptional):
    rail: str
    source_chain_id: int | Decimal
    source_tx_hash: str

_DepositEntryOptional = TypedDict("_DepositEntryOptional", {'action_id': 'int | Decimal | None', 'bridge_origin': 'DepositBridgeOrigin | None', 'fallback_error_code': 'int | Decimal | None', 'fallback_error_data': 'str | None', 'fallback_error_message': 'str | None', 'l1_sender': 'str | None', 'l1_tx_hash': 'str | None', 'tx_hash': 'str | None'}, total=False)
class DepositEntry(_DepositEntryOptional):
    amount: str
    asset: str
    batch_status: BatchStatus
    batch_uuid: str
    fee: str
    is_fallback: bool
    new_subaccount: bool
    operation_id: str
    subaccount_id: int | Decimal
    timestamp: int | Decimal
    wallet: str

class DepositHistoryResult(TypedDict):
    deposits: list[DepositEntry]

_EditSessionKeyRequestOptional = TypedDict("_EditSessionKeyRequestOptional", {'ip_whitelist': 'list[str] | None', 'label': 'str | None', 'offchain_scopes': 'list[str] | None'}, total=False)
class EditSessionKeyRequest(_EditSessionKeyRequestOptional):
    public_session_key: str
    wallet: str

class EmptyRequest(TypedDict):
    pass

_Erc20DetailsOptional = TypedDict("_Erc20DetailsOptional", {'underlying_erc20': 'str | None'}, total=False)
class Erc20Details(_Erc20DetailsOptional):
    decimals: int | Decimal

_ExecuteQuoteRequestOptional = TypedDict("_ExecuteQuoteRequestOptional", {'client': 'str | None', 'enable_taker_protection': 'bool | None', 'label': 'str | None', 'nonce': 'str', 'referral_code': 'str | None', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class ExecuteQuoteRequest(_ExecuteQuoteRequestOptional):
    direction: Direction
    legs: list[PricedLegParamsAndResponse]
    max_fee: str
    quote_id: str
    rfq_id: str
    subaccount_id: int | Decimal

_ExpirySettlementPriceOptional = TypedDict("_ExpirySettlementPriceOptional", {'price': 'str | None'}, total=False)
class ExpirySettlementPrice(_ExpirySettlementPriceOptional):
    expiry_date: str
    utc_expiry_sec: int | Decimal

class ExternalTransferActionDataResponse(TypedDict):
    amount: str
    asset: str
    max_fee_usd: str
    new_subaccount_manager: int | Decimal
    recipient_address: str
    sub_id: str
    to_subaccount_id: int | Decimal

_ForceBurnRequestOptional = TypedDict("_ForceBurnRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class ForceBurnRequest(_ForceBurnRequestOptional):
    holder: Address
    subaccount_id: int | Decimal

class ForwardFeedDataResponse(TypedDict):
    confidence: str
    currency: str
    deadline: int | Decimal
    expiry: int | Decimal
    fwd_diff: str
    signatures: OracleSignatureDataResponse
    spot_aggregate_latest: str
    spot_aggregate_start: str
    timestamp: int | Decimal

class ForwardedToken(TypedDict):
    bridge_type: str
    symbol: str
    token: str

class FundingFeedDataResponse(TypedDict):
    confidence: str
    currency: str
    deadline: int | Decimal
    funding_rate: str
    signatures: OracleSignatureDataResponse
    timestamp: int | Decimal

class FundingRateCandle(TypedDict):
    close: str
    currency: str
    funding_rate: str
    high: str
    low: str
    open: str
    risk_universe_id: int | Decimal
    timestamp: int | Decimal

class FundingRateHistoryResult(TypedDict):
    funding_rate_history: list[FundingRateCandle]

class GetAccountRequest(TypedDict):
    wallet: Address

class GetAlgoOrdersRequest(TypedDict):
    subaccount_id: int | Decimal

_GetAllInstrumentsRequestOptional = TypedDict("_GetAllInstrumentsRequestOptional", {'currency': 'str | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'risk_universe_id': 'int | Decimal | None'}, total=False)
class GetAllInstrumentsRequest(_GetAllInstrumentsRequestOptional):
    expired: bool
    instrument_type: AssetType

class GetAllInstrumentsResponse(TypedDict):
    instruments: list[Instrument]
    pagination: Pagination

class GetAllPortfoliosRequest(TypedDict):
    wallet: Address

class GetAllReferralCodesParams(TypedDict):
    pass

class GetCollateralsRequest(TypedDict):
    subaccount_id: int | Decimal

class GetCuratedVaultsRequest(TypedDict):
    wallet: str

class GetCurrencyRequest(TypedDict):
    currency: str

_GetDepositHistoryRequestOptional = TypedDict("_GetDepositHistoryRequestOptional", {'end_timestamp': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None', 'subaccount_id': 'int | Decimal | None', 'wallet': 'str | None'}, total=False)
class GetDepositHistoryRequest(_GetDepositHistoryRequestOptional):
    pass

_GetErc20TransferHistoryRequestOptional = TypedDict("_GetErc20TransferHistoryRequestOptional", {'end_timestamp': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None', 'subaccount_id': 'int | Decimal | None', 'wallet': 'str | None'}, total=False)
class GetErc20TransferHistoryRequest(_GetErc20TransferHistoryRequestOptional):
    pass

_GetFundingHistoryRequestOptional = TypedDict("_GetFundingHistoryRequestOptional", {'end_timestamp': 'int | Decimal | None', 'instrument_name': 'str | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None', 'subaccount_id': 'int | Decimal | None', 'wallet': 'str | None'}, total=False)
class GetFundingHistoryRequest(_GetFundingHistoryRequestOptional):
    pass

_GetFundingRateHistoryRequestOptional = TypedDict("_GetFundingRateHistoryRequestOptional", {'end_timestamp': 'int | Decimal | None', 'period': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None'}, total=False)
class GetFundingRateHistoryRequest(_GetFundingRateHistoryRequestOptional):
    instrument_name: str

class GetIndexChartDataRequest(TypedDict):
    currency: str
    end_timestamp: int | Decimal
    period: int | Decimal
    start_timestamp: int | Decimal

class GetInstrumentRequest(TypedDict):
    instrument_name: str

_GetInterestHistoryRequestOptional = TypedDict("_GetInterestHistoryRequestOptional", {'end_timestamp': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None', 'subaccount_id': 'int | Decimal | None', 'wallet': 'str | None'}, total=False)
class GetInterestHistoryRequest(_GetInterestHistoryRequestOptional):
    pass

_GetInterestRateHistoryRequestOptional = TypedDict("_GetInterestRateHistoryRequestOptional", {'end_timestamp': 'int | Decimal | None', 'period': 'int | Decimal | None', 'risk_universe_id': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None'}, total=False)
class GetInterestRateHistoryRequest(_GetInterestRateHistoryRequestOptional):
    currency: str

_GetLatestSignedFeedsRequestOptional = TypedDict("_GetLatestSignedFeedsRequestOptional", {'currency': 'str | None', 'expiry': 'int | Decimal | None'}, total=False)
class GetLatestSignedFeedsRequest(_GetLatestSignedFeedsRequestOptional):
    pass

class GetLatestSignedFeedsResponse(TypedDict):
    funding_data: dict[str, FundingFeedDataResponse]
    fwd_data: dict[str, dict[str, ForwardFeedDataResponse]]
    perp_data: dict[str, dict[str, PerpFeedDataResponse]]
    rate_data: dict[str, dict[str, RateFeedDataResponse]]
    spot_data: dict[str, SpotFeedDataResponse]
    vol_data: dict[str, dict[str, VolFeedDataResponse]]

_GetLiquidationHistoryRequestOptional = TypedDict("_GetLiquidationHistoryRequestOptional", {'end_timestamp': 'int | Decimal | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None', 'subaccount_id': 'int | Decimal | None'}, total=False)
class GetLiquidationHistoryRequest(_GetLiquidationHistoryRequestOptional):
    pass

class GetLiveBurnRequestsRequest(TypedDict):
    limit: int | Decimal
    subaccount_id: int | Decimal

class GetLiveIncidentsParams(TypedDict):
    pass

class GetLiveIncidentsResponse(TypedDict):
    incidents: list[IncidentResponse]

class GetLiveMintRequestsRequest(TypedDict):
    limit: int | Decimal
    subaccount_id: int | Decimal

class GetLiveVaultRequestsRequest(TypedDict):
    wallet: str

class GetMakerProgramScoresParams(TypedDict):
    epoch_start_timestamp: int | Decimal
    program_name: str

class GetMakerProgramScoresResponse(TypedDict):
    program: ProgramResponse
    scores: list[ScoreBreakdown]
    total_score: str
    total_volume: str

class GetMakerProgramsParams(TypedDict):
    pass

_GetMakerRfqHistoryRequestOptional = TypedDict("_GetMakerRfqHistoryRequestOptional", {'from_timestamp': 'int | Decimal | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'rfq_id': 'str | None', 'rfq_subaccount_id': 'int | Decimal | None', 'status': 'RFQStatus | None', 'to_timestamp': 'int | Decimal | None'}, total=False)
class GetMakerRfqHistoryRequest(_GetMakerRfqHistoryRequestOptional):
    subaccount_id: int | Decimal

class GetMarginResponse(TypedDict):
    is_valid_trade: bool
    post_initial_margin: str
    post_maintenance_margin: str
    pre_initial_margin: str
    pre_maintenance_margin: str
    subaccount_id: int | Decimal

_GetOnchainActionHistoryParamsOptional = TypedDict("_GetOnchainActionHistoryParamsOptional", {'action_type': 'int | Decimal | None', 'end_timestamp': 'int | Decimal | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None', 'wallet': 'str | None'}, total=False)
class GetOnchainActionHistoryParams(_GetOnchainActionHistoryParamsOptional):
    pass

class GetOnchainActionHistoryResponse(TypedDict):
    actions: list[OnchainActionHistoryEntry]
    pagination: Pagination

class GetOpenOrdersRequest(TypedDict):
    subaccount_id: int | Decimal

_GetOptionSettlementHistoryParamsOptional = TypedDict("_GetOptionSettlementHistoryParamsOptional", {'subaccount_id': 'int | Decimal | None', 'wallet': 'str | None'}, total=False)
class GetOptionSettlementHistoryParams(_GetOptionSettlementHistoryParamsOptional):
    pass

class GetOptionSettlementPricesRequest(TypedDict):
    currency: str

_GetOrderHistoryRequestOptional = TypedDict("_GetOrderHistoryRequestOptional", {'from_timestamp': 'int | Decimal | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'subaccount_id': 'int | Decimal | None', 'to_timestamp': 'int | Decimal | None', 'wallet': 'str | None'}, total=False)
class GetOrderHistoryRequest(_GetOrderHistoryRequestOptional):
    pass

class GetOrderRequest(TypedDict):
    order_id: str
    subaccount_id: int | Decimal

class GetPendingDepositsParams(TypedDict):
    wallet: str

class GetPendingDepositsResult(TypedDict):
    pending_deposits: list[PendingDepositEntry]
    wallet: str

class GetPerpImpactTwapRequest(TypedDict):
    currency: str
    end_time: int | Decimal
    start_time: int | Decimal

class GetPositionsRequest(TypedDict):
    subaccount_id: int | Decimal

_GetPublicTradeHistoryRequestOptional = TypedDict("_GetPublicTradeHistoryRequestOptional", {'batch_status': 'BatchStatus | None', 'currency': 'str | None', 'from_timestamp': 'int | Decimal | None', 'instrument_name': 'str | None', 'instrument_type': 'AssetType | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'subaccount_id': 'int | Decimal | None', 'to_timestamp': 'int | Decimal | None', 'trade_id': 'str | None'}, total=False)
class GetPublicTradeHistoryRequest(_GetPublicTradeHistoryRequestOptional):
    pass

_GetQuotesRequestOptional = TypedDict("_GetQuotesRequestOptional", {'from_timestamp': 'int | Decimal | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'quote_id': 'str | None', 'rfq_id': 'str | None', 'status': 'str | None', 'to_timestamp': 'int | Decimal | None'}, total=False)
class GetQuotesRequest(_GetQuotesRequestOptional):
    subaccount_id: int | Decimal

_GetReferralPerformanceParamsOptional = TypedDict("_GetReferralPerformanceParamsOptional", {'referral_code': 'str | None', 'wallet': 'str | None'}, total=False)
class GetReferralPerformanceParams(_GetReferralPerformanceParamsOptional):
    end_ms: int | Decimal
    start_ms: int | Decimal

class GetReferralPerformanceResult(TypedDict):
    fee_share_percentage: str
    referral_code: str
    rewards: dict[str, dict[str, dict[str, ReferralPerformanceByInstrumentType]]]
    stdrv_balance: str
    total_builder_fee_collected: str
    total_fee_rewards: str
    total_notional_volume: str
    total_referred_fees: str

_GetRfqsRequestOptional = TypedDict("_GetRfqsRequestOptional", {'from_timestamp': 'int | Decimal | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'rfq_id': 'str | None', 'status': 'str | None', 'to_timestamp': 'int | Decimal | None'}, total=False)
class GetRfqsRequest(_GetRfqsRequestOptional):
    subaccount_id: int | Decimal

class GetShareholderVaultsRequest(TypedDict):
    wallet: str

class GetSubaccountRequest(TypedDict):
    subaccount_id: int | Decimal

_GetSubaccountValueHistoryRequestOptional = TypedDict("_GetSubaccountValueHistoryRequestOptional", {'end_timestamp': 'int | Decimal | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'period': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None', 'subaccount_id': 'int | Decimal | None', 'wallet': 'str | None'}, total=False)
class GetSubaccountValueHistoryRequest(_GetSubaccountValueHistoryRequestOptional):
    pass

class GetSubaccountsRequest(TypedDict):
    wallet: Address

class GetTickerRequest(TypedDict):
    instrument_name: str

_GetTickersRequestOptional = TypedDict("_GetTickersRequestOptional", {'currency': 'str | None', 'expiry_date': 'int | Decimal | None'}, total=False)
class GetTickersRequest(_GetTickersRequestOptional):
    instrument_type: AssetType

class GetTickersResponse(TypedDict):
    tickers: dict[str, TickerSlimSnapshot]

_GetTradeHistoryRequestOptional = TypedDict("_GetTradeHistoryRequestOptional", {'from_timestamp': 'int | Decimal | None', 'instrument_name': 'str | None', 'order_id': 'str | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'quote_id': 'str | None', 'subaccount_id': 'int | Decimal | None', 'to_timestamp': 'int | Decimal | None', 'wallet': 'str | None'}, total=False)
class GetTradeHistoryRequest(_GetTradeHistoryRequestOptional):
    pass

class GetTradingviewChartDataRequest(TypedDict):
    end_timestamp: int | Decimal
    instrument_name: str
    period: int | Decimal
    start_timestamp: int | Decimal

class GetTransactionParams(TypedDict):
    op_uuid: str

_GetTransactionResultOptional = TypedDict("_GetTransactionResultOptional", {'error_log': 'str | None', 'status': 'BatchStatus | None', 'transaction_hash': 'str | None'}, total=False)
class GetTransactionResult(_GetTransactionResultOptional):
    data: str

class GetTriggerOrdersRequest(TypedDict):
    subaccount_id: int | Decimal

_GetVaultActionHistoryRequestOptional = TypedDict("_GetVaultActionHistoryRequestOptional", {'event_type': 'str | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None'}, total=False)
class GetVaultActionHistoryRequest(_GetVaultActionHistoryRequestOptional):
    subaccount_id: int | Decimal

_GetVaultPerformanceHistoryRequestOptional = TypedDict("_GetVaultPerformanceHistoryRequestOptional", {'from': 'int | Decimal | None', 'limit': 'int | Decimal | None', 'to': 'int | Decimal | None'}, total=False)
class GetVaultPerformanceHistoryRequest(_GetVaultPerformanceHistoryRequestOptional):
    resolution: PerformanceResolution
    subaccount_id: int | Decimal

class GetVaultRequest(TypedDict):
    subaccount_id: int | Decimal

_GetVaultRequestHistoryRequestOptional = TypedDict("_GetVaultRequestHistoryRequestOptional", {'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None'}, total=False)
class GetVaultRequestHistoryRequest(_GetVaultRequestHistoryRequestOptional):
    wallet: str

class GetVaultSharesRequest(TypedDict):
    wallet: str

_GetVaultsRequestOptional = TypedDict("_GetVaultsRequestOptional", {'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None'}, total=False)
class GetVaultsRequest(_GetVaultsRequestOptional):
    pass

_GetWalletsFromSessionKeyRequestOptional = TypedDict("_GetWalletsFromSessionKeyRequestOptional", {'scope': 'OffchainKeyScope | None'}, total=False)
class GetWalletsFromSessionKeyRequest(_GetWalletsFromSessionKeyRequestOptional):
    public_session_key: str

class GetWithdrawAddressesParams(TypedDict):
    wallet: Address

class GetWithdrawAddressesResult(TypedDict):
    withdraw_addresses: list[WithdrawAddressEntry]

_GetWithdrawalHistoryRequestOptional = TypedDict("_GetWithdrawalHistoryRequestOptional", {'end_timestamp': 'int | Decimal | None', 'start_timestamp': 'int | Decimal | None', 'subaccount_id': 'int | Decimal | None', 'wallet': 'str | None'}, total=False)
class GetWithdrawalHistoryRequest(_GetWithdrawalHistoryRequestOptional):
    pass

class IncidentResponse(TypedDict):
    creation_timestamp_sec: int | Decimal
    label: str
    message: str
    monitor_type: MonitorType
    severity: Severity

class IndexCandle(TypedDict):
    close_price: str
    high_price: str
    low_price: str
    open_price: str
    price: str
    timestamp: int | Decimal
    timestamp_bucket: int | Decimal

_InstrumentOptional = TypedDict("_InstrumentOptional", {'erc20_details': 'SpotPublicDetails | None', 'mark_price_fee_rate_cap': 'str | None', 'option_details': 'OptionDetails | None', 'perp_details': 'PerpDetails | None'}, total=False)
class Instrument(_InstrumentOptional):
    amount_step: str
    base_asset_address: str
    base_asset_sub_id: str
    base_currency: str
    base_fee: str
    fifo_min_allocation: str
    instrument_name: str
    instrument_type: AssetType
    is_active: bool
    maker_fee_rate: str
    maximum_amount: str
    minimum_amount: str
    pro_rata_amount_step: str
    pro_rata_fraction: str
    quote_currency: str
    risk_universe_id: int | Decimal
    scheduled_activation: int | Decimal
    scheduled_deactivation: int | Decimal
    taker_fee_rate: str
    tick_size: str

class InterestHistoryResult(TypedDict):
    events: list[InterestPayment]

class InterestPayment(TypedDict):
    interest: str
    subaccount_id: int | Decimal
    timestamp: int | Decimal

class InterestRateCandle(TypedDict):
    borrow_apy: Ohlc
    risk_universe_id: int | Decimal
    supply_apy: Ohlc
    timestamp: int | Decimal
    total_borrow: str
    total_supply: str

class InterestRateHistoryResult(TypedDict):
    interest_rate_history: list[InterestRateCandle]

class LegUnpricedParams(TypedDict):
    amount: str
    direction: Direction
    instrument_name: str

class LendingDetails(TypedDict):
    borrow_apy: str
    supply_apy: str
    total_borrow: str
    total_borrow_cap: str

class LiquidationHistoryResult(TypedDict):
    auctions: list[AuctionHistory]
    pagination: Pagination

_LiveAuctionOptional = TypedDict("_LiveAuctionOptional", {'currency': 'str | None'}, total=False)
class LiveAuction(_LiveAuctionOptional):
    estimated_bid_price: str
    estimated_discount_pnl: str
    estimated_mtm: str
    estimated_percent_bid: str
    margin_type: str
    min_price_limit: str
    risk_universe_id: int | Decimal
    subaccount_balances: dict[str, str]
    subaccount_id: int | Decimal
    timestamp: int | Decimal

class ManagerCollateral(TypedDict):
    address: str
    erc20: Erc20Details
    im_discount: str
    min_deposit_usd: str
    mm_discount: str
    name: str

_MarginWatchRequestOptional = TypedDict("_MarginWatchRequestOptional", {'is_delayed_liquidation': 'bool | None'}, total=False)
class MarginWatchRequest(_MarginWatchRequestOptional):
    subaccount_id: int | Decimal

_MintSharesRequestOptional = TypedDict("_MintSharesRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class MintSharesRequest(_MintSharesRequestOptional):
    deposit_hash: str
    request_id: VaultRequestId
    share_price: str
    subaccount_id: int | Decimal

class MmpConfigResult(TypedDict):
    currency: str
    is_frozen: bool
    mmp_amount_limit: str
    mmp_delta_limit: str
    mmp_frozen_time: int | Decimal
    mmp_interval: int | Decimal
    mmp_unfreeze_time: int | Decimal
    subaccount_id: int | Decimal

_MmpScopeRequestOptional = TypedDict("_MmpScopeRequestOptional", {'currency': 'str | None'}, total=False)
class MmpScopeRequest(_MmpScopeRequestOptional):
    subaccount_id: int | Decimal

class MultipleVaultRequestsResponse(TypedDict):
    requests: list[VaultRequestResponse]
    total: int | Decimal

class OffchainAckResponse(TypedDict):
    status: str

class Ohlc(TypedDict):
    close: str
    high: str
    low: str
    open: str

_OnchainActionHistoryEntryOptional = TypedDict("_OnchainActionHistoryEntryOptional", {'error_code': 'int | Decimal | None', 'error_data': 'str | None', 'error_message': 'str | None', 'fallback_at': 'int | Decimal | None', 'first_failed_at': 'int | Decimal | None', 'last_failed_at': 'int | Decimal | None', 'op_uuid': 'str | None', 'skip_reason': 'str | None', 'skip_reason_code': 'str | None', 'tx_hash': 'str | None'}, total=False)
class OnchainActionHistoryEntry(_OnchainActionHistoryEntryOptional):
    acc: str
    action_id: int | Decimal
    action_type: int | Decimal
    action_type_label: str
    block_number: int | Decimal
    data: str
    l1_sender: str
    queue: str
    status: str
    updated_at: int | Decimal

class OpenInterestStats(TypedDict):
    current_open_interest: str
    interest_cap: str

_OptionDetailsOptional = TypedDict("_OptionDetailsOptional", {'settlement_price': 'str | None'}, total=False)
class OptionDetails(_OptionDetailsOptional):
    expiry: int | Decimal
    index: str
    option_type: str
    strike: str

class OptionPricing(TypedDict):
    ai: str
    bi: str
    d: str
    df: str
    f: str
    g: str
    i: str
    m: str
    r: str
    t: str
    v: str

class OptionSettlementHistoryResponse(TypedDict):
    settlements: list[OptionSettlementResponse]

class OptionSettlementPricesResult(TypedDict):
    expiries: list[ExpirySettlementPrice]

class OptionSettlementResponse(TypedDict):
    amount: str
    expiry: int | Decimal
    instrument_name: str
    option_settlement_pnl: str | None
    option_settlement_pnl_excl_fees: str | None
    settlement_price: str
    settlement_value: str
    subaccount_id: int | Decimal

class OracleSignatureDataResponse(TypedDict):
    signatures: list[str]
    signers: list[Address]

_OrderOptional = TypedDict("_OrderOptional", {'algo_duration_sec': 'int | Decimal | None', 'algo_num_slices': 'int | Decimal | None', 'algo_slices_completed': 'int | Decimal | None', 'algo_type': 'AlgoType | None', 'cancel_reason': 'CancelReason | None', 'label': 'str | None', 'trigger_price_type': 'TriggerPriceType | None', 'trigger_reject_message': 'str | None', 'trigger_type': 'TriggerType | None'}, total=False)
class Order(_OrderOptional):
    amount: str
    average_price: str
    creation_timestamp: int | Decimal
    direction: Direction
    extra_fee: str
    filled_amount: str
    instrument_name: str
    is_transfer: bool
    last_update_timestamp: int | Decimal
    limit_price: str
    max_fee: str
    mmp: bool
    nonce: str
    order_fee: str
    order_id: str
    order_status: OrderStatus
    order_type: OrderType
    quote_id: str | None
    replaced_order_id: str | None
    signature: str
    signature_expiry_sec: int | Decimal
    signed_limit_price: str | None
    signer: str
    subaccount_id: int | Decimal
    time_in_force: TimeInForce
    trigger_price: str | None

class OrderActionDataResponse(TypedDict):
    asset_address: str
    asset_sub_id: str
    desired_amount: str
    is_bid: bool
    limit_price: str
    recipient_id: int | Decimal
    worst_fee: str

class OrderActionInputData(TypedDict):
    data: OrderActionDataResponse
    expiry: int | Decimal
    module: str
    nonce: str
    owner: str
    signer: str
    subaccount_id: int | Decimal

class OrderCreatedResponse(TypedDict):
    order: Order
    trades: list[Trade]

_OrderDebugResponseOptional = TypedDict("_OrderDebugResponseOptional", {'recovered_signer': 'str | None'}, total=False)
class OrderDebugResponse(_OrderDebugResponseOptional):
    action_hash: str
    action_typehash: str
    domain_separator: str
    encoded_data: str
    encoded_data_hashed: str
    expected_signer: str
    input_data: OrderActionInputData
    module: str
    owner: str
    typed_data_hash: str

_OrderQuoteRequestOptional = TypedDict("_OrderQuoteRequestOptional", {'client': 'str | None', 'extra_fee': 'str | None', 'is_atomic_signing': 'bool | None', 'label': 'str | None', 'mmp': 'bool | None', 'nonce': 'str', 'order_type': 'OrderType | None', 'reduce_only': 'bool | None', 'referral_code': 'str | None', 'reject_post_only': 'bool | None', 'reject_timestamp': 'int | Decimal | None', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'str', 'time_in_force': 'TimeInForce | None', 'trigger_price': 'str | None', 'trigger_price_type': 'TriggerPriceType | None', 'trigger_type': 'TriggerType | None'}, total=False)
class OrderQuoteRequest(_OrderQuoteRequestOptional):
    amount: str
    direction: Direction
    instrument_name: str
    limit_price: str
    max_fee: str
    subaccount_id: int | Decimal

_OrderQuoteResponseOptional = TypedDict("_OrderQuoteResponseOptional", {'invalid_reason': 'str | None', 'max_amount': 'str | None', 'post_liquidation_price': 'str | None'}, total=False)
class OrderQuoteResponse(_OrderQuoteResponseOptional):
    estimated_fee: str
    estimated_fill_amount: str
    estimated_fill_price: str
    estimated_order_status: OrderStatus
    estimated_realized_pnl: str
    estimated_realized_pnl_excl_fees: str
    is_valid: bool
    post_initial_margin: str
    pre_initial_margin: str
    suggested_max_fee: str

class PaginatedOrdersResult(TypedDict):
    orders: list[Order]
    pagination: Pagination
    subaccount_id: int | Decimal

class PaginatedTradesResult(TypedDict):
    pagination: Pagination
    subaccount_id: int | Decimal
    trades: list[TradeHistoryResponse]

class PaginatedVaultActionHistory(TypedDict):
    events: list[PublicVaultActionResponse]
    pagination: Pagination
    subaccount_id: int | Decimal

class PaginatedVaultRequestHistory(TypedDict):
    actions: list[VaultActionResponse]
    pagination: Pagination
    wallet: str

class Pagination(TypedDict):
    count: int | Decimal
    num_pages: int | Decimal

_PendingDepositBridgeOriginOptional = TypedDict("_PendingDepositBridgeOriginOptional", {'delivered_at_ms': 'int | Decimal | None', 'provider_transfer_id': 'str | None', 'receiving_amount': 'str | None', 'receiving_token': 'str | None', 'receiving_tx_hash': 'str | None', 'source_address': 'str | None'}, total=False)
class PendingDepositBridgeOrigin(_PendingDepositBridgeOriginOptional):
    bridge_id: str
    bridge_status: str
    deposit_address: str
    expected_amount: str
    expires_at_ms: int | Decimal
    rail: str
    source_chain_id: int | Decimal
    source_tx_hash: str
    substatus: str
    token: str

_PendingDepositEntryOptional = TypedDict("_PendingDepositEntryOptional", {'action_id': 'int | Decimal | None', 'amount': 'str | None', 'block_number': 'int | Decimal | None', 'bridge_to_l1': 'PendingDepositBridgeOrigin | None', 'log_index': 'int | Decimal | None', 'tx_hash': 'str | None'}, total=False)
class PendingDepositEntry(_PendingDepositEntryOptional):
    action_type: str
    asset: str
    deposit_type: str
    manager_id: int | Decimal
    status: str
    subaccount_id: int | Decimal
    timestamp: int | Decimal
    updated_at_ms: int | Decimal

_PerpDetailsOptional = TypedDict("_PerpDetailsOptional", {'srm_perp_margin_requirements': 'SrmPerpMarginRequirements | None'}, total=False)
class PerpDetails(_PerpDetailsOptional):
    aggregate_funding: str
    funding_rate: str
    index: str
    max_rate_per_hour: str
    min_rate_per_hour: str

class PerpFeedDataResponse(TypedDict):
    confidence: str
    currency: str
    deadline: int | Decimal
    signatures: OracleSignatureDataResponse
    spot_diff_value: str
    timestamp: int | Decimal
    type: str

class PerpImpactTwapResult(TypedDict):
    ask_impact_diff_twap: str
    bid_impact_diff_twap: str
    currency: str
    mid_price_diff_twap: str

_PerpSettlementEventResponseOptional = TypedDict("_PerpSettlementEventResponseOptional", {'tx_hash': 'str | None'}, total=False)
class PerpSettlementEventResponse(_PerpSettlementEventResponseOptional):
    batch_status: BatchStatus
    batch_uuid: str
    funding: str
    instrument_name: str
    pnl: str
    subaccount_id: int | Decimal
    timestamp: int | Decimal

class PerpSettlementHistoryResponse(TypedDict):
    events: list[PerpSettlementEventResponse]
    pagination: Pagination

_PollQuotesRequestOptional = TypedDict("_PollQuotesRequestOptional", {'from_timestamp': 'int | Decimal | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'quote_id': 'str | None', 'rfq_id': 'str | None', 'status': 'str | None', 'to_timestamp': 'int | Decimal | None'}, total=False)
class PollQuotesRequest(_PollQuotesRequestOptional):
    subaccount_id: int | Decimal

_PollRfqsRequestOptional = TypedDict("_PollRfqsRequestOptional", {'from_timestamp': 'int | Decimal | None', 'page': 'int | Decimal | None', 'page_size': 'int | Decimal | None', 'rfq_id': 'str | None', 'rfq_subaccount_id': 'int | Decimal | None', 'status': 'str | None', 'to_timestamp': 'int | Decimal | None'}, total=False)
class PollRfqsRequest(_PollRfqsRequestOptional):
    subaccount_id: int | Decimal

_PositionOptional = TypedDict("_PositionOptional", {'leverage': 'str | None', 'liquidation_price': 'str | None'}, total=False)
class Position(_PositionOptional):
    amount: str
    amount_step: str
    average_price: str
    average_price_excl_fees: str
    creation_timestamp: int | Decimal
    cumulative_funding: str
    delta: str
    gamma: str
    index_price: str
    initial_margin: str
    instrument_name: str
    instrument_type: AssetType
    maintenance_margin: str
    mark_price: str
    mark_value: str
    net_settlements: str
    open_orders_margin: str
    pending_funding: str
    realized_pnl: str
    realized_pnl_excl_fees: str
    theta: str
    total_fees: str
    unrealized_pnl: str
    unrealized_pnl_excl_fees: str
    vega: str

class PricedLegParamsAndResponse(TypedDict):
    amount: str
    direction: Direction
    instrument_name: str
    price: str

class PrivateChangeSubaccountLabelResponse(TypedDict):
    label: str
    subaccount_id: int | Decimal

_PrivateGetAccountResponseOptional = TypedDict("_PrivateGetAccountResponseOptional", {'creation_timestamp_sec': 'int | Decimal | None', 'referral_code': 'str | None'}, total=False)
class PrivateGetAccountResponse(_PrivateGetAccountResponseOptional):
    cancel_on_disconnect: bool
    fallback_subaccount_id: int | Decimal
    fee_info: AccountFeeInfo
    is_rfq_maker: bool
    per_endpoint_tps: dict[str, int | Decimal]
    subaccount_ids: list[int | Decimal]
    wallet: str
    websocket_matching_tps: int | Decimal
    websocket_non_matching_tps: int | Decimal
    websocket_option_tps: int | Decimal
    websocket_perp_tps: int | Decimal
    whitelisted_recipients: list[str]

class PrivateGetCollateralsResponse(TypedDict):
    collaterals: list[Collateral]
    subaccount_id: int | Decimal

_PrivateGetMarginRequestOptional = TypedDict("_PrivateGetMarginRequestOptional", {'is_delayed_liquidation': 'bool | None', 'simulated_collateral_changes': 'list[SimulatedCollateralRequest] | None', 'simulated_position_changes': 'list[SimulatedPositionRequest] | None'}, total=False)
class PrivateGetMarginRequest(_PrivateGetMarginRequestOptional):
    subaccount_id: int | Decimal

class PrivateGetPositionsResponse(TypedDict):
    positions: list[Position]
    subaccount_id: int | Decimal

class PrivateGetSubaccountsResponse(TypedDict):
    subaccount_ids: list[int | Decimal]
    wallet: str

_PrivateLiquidateRequestOptional = TypedDict("_PrivateLiquidateRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'str'}, total=False)
class PrivateLiquidateRequest(_PrivateLiquidateRequestOptional):
    liquidate_subaccount_id: int | Decimal
    percent_of_acc: str
    price_limit: str
    subaccount_id: int | Decimal

class PrivateLiquidateResponse(TypedDict):
    op_uuid: str
    operation_id: int | Decimal

class PrivateSessionKeysResponse(TypedDict):
    public_session_keys: list[SessionKey]

_PrivateSetSessionKeyResponseOptional = TypedDict("_PrivateSetSessionKeyResponseOptional", {'label': 'str | None'}, total=False)
class PrivateSetSessionKeyResponse(_PrivateSetSessionKeyResponseOptional):
    expiry_sec: int | Decimal
    ip_whitelist: list[str]
    offchain_scopes: list[str]
    protocol_scopes: list[str]
    public_session_key: str
    subaccount_ids: list[int | Decimal]

_PrivateTransferSpotExternalRequestOptional = TypedDict("_PrivateTransferSpotExternalRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'str'}, total=False)
class PrivateTransferSpotExternalRequest(_PrivateTransferSpotExternalRequestOptional):
    amount: str
    asset_name: str
    max_fee_usd: str
    new_subaccount_manager: int | Decimal
    recipient_address: str
    sub_id: int | Decimal
    subaccount_id: int | Decimal
    to_subaccount_id: int | Decimal

class PrivateTransferSpotExternalResponse(TypedDict):
    op_uuid: str
    operation_id: int | Decimal

_PrivateTransferSpotRequestOptional = TypedDict("_PrivateTransferSpotRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'str'}, total=False)
class PrivateTransferSpotRequest(_PrivateTransferSpotRequestOptional):
    amount: str
    asset_name: str
    max_fee_usd: str
    new_subaccount_manager: int | Decimal
    sub_id: int | Decimal
    subaccount_id: int | Decimal
    to_subaccount_id: int | Decimal

class PrivateTransferSpotResponse(TypedDict):
    op_uuid: str
    operation_id: int | Decimal

_PrivateWithdrawRequestOptional = TypedDict("_PrivateWithdrawRequestOptional", {'nonce': 'str', 'recipient': 'str | None', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'str'}, total=False)
class PrivateWithdrawRequest(_PrivateWithdrawRequestOptional):
    amount_in_underlying: str
    asset_name: str
    force_batch: bool
    max_fee_usd: str
    subaccount_id: int | Decimal

class PrivateWithdrawResponse(TypedDict):
    op_uuid: str
    operation_id: int | Decimal

class ProgramResponse(TypedDict):
    asset_types: list[str]
    currencies: list[str]
    end_timestamp: int | Decimal
    min_notional: str
    name: str
    rewards: dict[str, str]
    start_timestamp: int | Decimal

class ProtocolVault(TypedDict):
    closed: bool
    config: VaultConfig
    global_hwm: str
    last_fee_settled_at_sec: int | Decimal
    protocol_fee_share_bps: int | Decimal
    subaccount_id: int | Decimal
    total_shares: str

_PublicExecuteQuoteDebugRequestOptional = TypedDict("_PublicExecuteQuoteDebugRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class PublicExecuteQuoteDebugRequest(_PublicExecuteQuoteDebugRequestOptional):
    direction: Direction
    legs: list[PricedLegParamsAndResponse]
    max_fee: str
    quote_id: str
    rfq_id: str
    subaccount_id: int | Decimal

class PublicGetLiveAuctionsResponse(TypedDict):
    auctions: list[LiveAuction]

_PublicGetMarginRequestOptional = TypedDict("_PublicGetMarginRequestOptional", {'market': 'str | None', 'simulated_collateral_changes': 'list[SimulatedCollateralRequest] | None', 'simulated_position_changes': 'list[SimulatedPositionRequest] | None'}, total=False)
class PublicGetMarginRequest(_PublicGetMarginRequestOptional):
    margin_type: str
    simulated_collaterals: list[SimulatedCollateralRequest]
    simulated_positions: list[SimulatedPositionRequest]

class PublicGetSubaccountsRequest(TypedDict):
    wallet: Address

class PublicGetWalletRequest(TypedDict):
    subaccount_id: int | Decimal

class PublicGetWalletResponse(TypedDict):
    subaccount_id: int | Decimal
    wallet: str

class PublicGetWalletsFromSessionKeyResponse(TypedDict):
    wallets: list[str]

_PublicMarginWatchResponseOptional = TypedDict("_PublicMarginWatchResponseOptional", {'regular_margin_flagged_sec': 'int | Decimal | None'}, total=False)
class PublicMarginWatchResponse(_PublicMarginWatchResponseOptional):
    collaterals: list[Collateral]
    currency: list[str]
    has_delayed_liquidation: bool
    initial_margin: str
    is_delayed_liquidation: bool
    maintenance_margin: str
    margin_type: str
    positions: list[Position]
    risk_universe_id: int | Decimal
    subaccount_id: int | Decimal
    subaccount_value: str
    valuation_timestamp: int | Decimal

class PublicQuote(TypedDict):
    cancel_reason: RFQCancelReason
    creation_timestamp: int | Decimal
    direction: Direction
    fill_pct: str
    last_update_timestamp: int | Decimal
    legs: list[PricedLegParamsAndResponse]
    legs_hash: str
    liquidity_role: LiquidityRole
    quote_id: str
    rfq_id: str
    status: RFQStatus
    subaccount_id: int | Decimal
    wallet: Address

_PublicRfqOptional = TypedDict("_PublicRfqOptional", {'filled_direction': 'Direction | None'}, total=False)
class PublicRfq(_PublicRfqOptional):
    cancel_reason: RFQCancelReason
    creation_timestamp: int | Decimal
    fill_rate: str | None
    filled_pct: str
    last_update_timestamp: int | Decimal
    legs: list[LegUnpricedParams]
    partial_fill_step: str
    recent_fill_rate: str | None
    rfq_id: str
    status: RFQStatus
    subaccount_id: int | Decimal
    total_cost: str | None
    valid_until: int | Decimal
    wallet: Address

_PublicSendQuoteDebugRequestOptional = TypedDict("_PublicSendQuoteDebugRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class PublicSendQuoteDebugRequest(_PublicSendQuoteDebugRequestOptional):
    direction: Direction
    legs: list[PricedLegParamsAndResponse]
    max_fee: str
    rfq_id: str
    subaccount_id: int | Decimal

class PublicTradesResult(TypedDict):
    pagination: Pagination
    trades: list[SettledTrade]

class PublicVaultActionResponse(TypedDict):
    curator_shares_minted: str
    event_ts: int | Decimal
    event_type: str
    holder: str
    management_shares_minted: str
    nav: str
    new_high_water_mark: str
    old_high_water_mark: str
    operation_uuid: str
    performance_shares_minted: str
    protocol_shares_minted: str
    share_price: str
    shares_delta: str
    status: str
    subaccount_id: int | Decimal
    total_shares: str

_PublicWithdrawDebugRequestOptional = TypedDict("_PublicWithdrawDebugRequestOptional", {'nonce': 'str', 'recipient': 'str | None', 'signature_expiry_sec': 'int | Decimal', 'signer': 'str'}, total=False)
class PublicWithdrawDebugRequest(_PublicWithdrawDebugRequestOptional):
    amount_in_underlying: str
    asset_name: str
    force_batch: bool
    max_fee_usd: str
    subaccount_id: int | Decimal

_QuoteOptional = TypedDict("_QuoteOptional", {'batch_status': 'BatchStatus | None', 'tx_hash': 'str | None'}, total=False)
class Quote(_QuoteOptional):
    cancel_reason: RFQCancelReason
    creation_timestamp: int | Decimal
    direction: Direction
    extra_fee: str
    fee: str
    fill_pct: str
    is_transfer: bool
    label: str
    last_update_timestamp: int | Decimal
    legs: list[PricedLegParamsAndResponse]
    legs_hash: str
    liquidity_role: LiquidityRole
    max_fee: str
    mmp: bool
    nonce: str
    quote_id: str
    rfq_id: str
    signature_expiry_sec: int | Decimal
    status: RFQStatus
    subaccount_id: int | Decimal

class QuoteExecuteDebugResult(TypedDict):
    action_hash: str
    encoded_data: str
    encoded_data_hashed: str
    encoded_legs: str
    legs_hash: str
    typed_data_hash: str

class QuoteExecuteResponse(TypedDict):
    cancel_reason: RFQCancelReason
    creation_timestamp: int | Decimal
    direction: Direction
    extra_fee: str
    fee: str
    fill_pct: str
    is_transfer: bool
    label: str
    last_update_timestamp: int | Decimal
    legs: list[PricedLegParamsAndResponse]
    legs_hash: str
    liquidity_role: LiquidityRole
    max_fee: str
    mmp: bool
    nonce: str
    quote_id: str
    rfq_filled_pct: str
    rfq_id: str
    signature_expiry_sec: int | Decimal
    status: RFQStatus
    subaccount_id: int | Decimal

class QuoteGetResponse(TypedDict):
    pagination: Pagination
    quotes: list[Quote]

class QuotePollResponse(TypedDict):
    pagination: Pagination
    quotes: list[PublicQuote]

_QuoteReplaceResponseOptional = TypedDict("_QuoteReplaceResponseOptional", {'create_quote_error': 'RPCError | None', 'quote': 'Quote | None'}, total=False)
class QuoteReplaceResponse(_QuoteReplaceResponseOptional):
    cancelled_quote: Quote

class QuoteSendDebugResult(TypedDict):
    action_hash: str
    encoded_data: str
    encoded_data_hashed: str
    typed_data_hash: str

class RFQGetResponse(TypedDict):
    pagination: Pagination
    rfqs: list[Rfq]

class RFQPollResponse(TypedDict):
    pagination: Pagination
    rfqs: list[PublicRfq]

_RPCErrorOptional = TypedDict("_RPCErrorOptional", {'data': 'str | None'}, total=False)
class RPCError(_RPCErrorOptional):
    code: int | Decimal
    message: str

class RateFeedDataResponse(TypedDict):
    confidence: str
    currency: str
    deadline: int | Decimal
    expiry: int | Decimal
    rate: str
    signatures: OracleSignatureDataResponse
    timestamp: int | Decimal

class RateLimitInfo(TypedDict):
    consumedPoints: int | Decimal
    isFirstInDuration: bool
    msBeforeNext: int | Decimal
    remainingPoints: int | Decimal

_RateLimitResultOptional = TypedDict("_RateLimitResultOptional", {'remaining_connections': 'RateLimitInfo | None'}, total=False)
class RateLimitResult(_RateLimitResultOptional):
    remaining_matching: RateLimitInfo
    remaining_non_matching: RateLimitInfo
    remaining_per_endpoint: dict[str, RateLimitInfo]

class ReferralPerformanceByInstrumentType(TypedDict):
    builder_fee: str
    fee_reward: str
    notional_volume: str
    referred_fee: str
    unique_traders_referred: int | Decimal

_ReferrerOptional = TypedDict("_ReferrerOptional", {'receiving_wallet': 'str | None'}, total=False)
class Referrer(_ReferrerOptional):
    referral_code: str
    wallet: str

_RegisterBridgeDepositParamsOptional = TypedDict("_RegisterBridgeDepositParamsOptional", {'provider_transfer_id': 'str | None', 'tool': 'str | None'}, total=False)
class RegisterBridgeDepositParams(_RegisterBridgeDepositParamsOptional):
    bridge_token: str
    bridge_token_decimals: int | Decimal
    deposit_address: str
    expected_amount: str
    rail: str
    source_chain_id: int | Decimal
    source_token: str
    source_tx_hash: str
    wallet: str

_RegisterDepositAddressParamsOptional = TypedDict("_RegisterDepositAddressParamsOptional", {'manager_id': 'int | Decimal | None', 'subaccount_id': 'int | Decimal | None'}, total=False)
class RegisterDepositAddressParams(_RegisterDepositAddressParamsOptional):
    wallet: str

_RegisterDepositAddressResultOptional = TypedDict("_RegisterDepositAddressResultOptional", {'manager_id': 'int | Decimal | None', 'subaccount_id': 'int | Decimal | None'}, total=False)
class RegisterDepositAddressResult(_RegisterDepositAddressResultOptional):
    deposit_address: str
    wallet: str

class RegisterWithdrawAddressParams(TypedDict):
    chain_id: int | Decimal
    recipient: str
    wallet: str

class RegisterWithdrawAddressResult(TypedDict):
    chain_id: int | Decimal
    recipient: str
    recipient_display: str
    routes: list[ForwardedToken]
    withdraw_address: str

_RejectDepositRequestRequestOptional = TypedDict("_RejectDepositRequestRequestOptional", {'reason': 'str | None'}, total=False)
class RejectDepositRequestRequest(_RejectDepositRequestRequestOptional):
    request_id: VaultRequestId
    subaccount_id: int | Decimal

_ReplaceOrderRequestOptional = TypedDict("_ReplaceOrderRequestOptional", {'algo_duration_sec': 'int | Decimal | None', 'algo_num_slices': 'int | Decimal | None', 'algo_type': 'AlgoType | None', 'client': 'str | None', 'expected_filled_amount': 'str | None', 'extra_fee': 'str | None', 'is_atomic_signing': 'bool | None', 'label': 'str | None', 'mmp': 'bool | None', 'nonce': 'str', 'nonce_to_cancel': 'str | None', 'order_id_to_cancel': 'str | None', 'order_type': 'OrderType | None', 'reduce_only': 'bool | None', 'referral_code': 'str | None', 'reject_post_only': 'bool | None', 'reject_timestamp': 'int | Decimal | None', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'str', 'time_in_force': 'TimeInForce | None', 'trigger_price': 'str | None', 'trigger_price_type': 'TriggerPriceType | None', 'trigger_type': 'TriggerType | None'}, total=False)
class ReplaceOrderRequest(_ReplaceOrderRequestOptional):
    amount: str
    direction: Direction
    instrument_name: str
    limit_price: str
    max_fee: str
    subaccount_id: int | Decimal

_ReplaceOrderResponseOptional = TypedDict("_ReplaceOrderResponseOptional", {'create_order_error': 'RPCError | None', 'order': 'Order | None', 'trades': 'list[Trade] | None'}, total=False)
class ReplaceOrderResponse(_ReplaceOrderResponseOptional):
    cancelled_order: Order

_ReplaceQuoteRequestOptional = TypedDict("_ReplaceQuoteRequestOptional", {'client': 'str | None', 'extra_fee': 'str | None', 'label': 'str | None', 'mmp': 'bool | None', 'nonce': 'str', 'nonce_to_cancel': 'str | None', 'quote_id_to_cancel': 'str | None', 'referral_code': 'str | None', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class ReplaceQuoteRequest(_ReplaceQuoteRequestOptional):
    direction: Direction
    legs: list[PricedLegParamsAndResponse]
    max_fee: str
    rfq_id: str
    subaccount_id: int | Decimal

_RequestVaultDepositRequestOptional = TypedDict("_RequestVaultDepositRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class RequestVaultDepositRequest(_RequestVaultDepositRequestOptional):
    amount: str
    deposit_spot_asset: Address
    subaccount_id: int | Decimal
    vault_subaccount_id: int | Decimal

_RequestVaultWithdrawRequestOptional = TypedDict("_RequestVaultWithdrawRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class RequestVaultWithdrawRequest(_RequestVaultWithdrawRequestOptional):
    shares_to_burn: str
    subaccount_id: int | Decimal
    vault_subaccount_id: int | Decimal

_RfqOptional = TypedDict("_RfqOptional", {'counterparties': 'list[Address] | None', 'filled_direction': 'Direction | None'}, total=False)
class Rfq(_RfqOptional):
    ask_total_cost: str | None
    bid_total_cost: str | None
    cancel_reason: RFQCancelReason
    creation_timestamp: int | Decimal
    filled_pct: str
    label: str
    last_update_timestamp: int | Decimal
    legs: list[LegUnpricedParams]
    mark_total_cost: str | None
    max_total_cost: str | None
    min_total_cost: str | None
    partial_fill_step: str
    rfq_id: str
    status: RFQStatus
    subaccount_id: int | Decimal
    total_cost: str | None
    valid_until: int | Decimal
    wallet: Address

_RfqGetBestQuoteRequestOptional = TypedDict("_RfqGetBestQuoteRequestOptional", {'client': 'str | None', 'direction': 'Direction | None', 'extra_fee': 'str | None', 'legs': 'list[LegUnpricedParams] | None', 'rfq_id': 'str | None'}, total=False)
class RfqGetBestQuoteRequest(_RfqGetBestQuoteRequestOptional):
    subaccount_id: int | Decimal

_RfqGetBestQuoteResponseOptional = TypedDict("_RfqGetBestQuoteResponseOptional", {'best_quote': 'PublicQuote | None', 'invalid_reason': 'str | None'}, total=False)
class RfqGetBestQuoteResponse(_RfqGetBestQuoteResponseOptional):
    direction: Direction
    down_liquidation_price: str | None
    estimated_fee: str
    estimated_realized_pnl: str
    estimated_realized_pnl_excl_fees: str
    estimated_total_cost: str
    filled_pct: str
    is_valid: bool
    orderbook_total_cost: str | None
    post_initial_margin: str
    post_liquidation_price: str | None
    pre_initial_margin: str
    suggested_max_fee: str
    up_liquidation_price: str | None

class RfqLegResponse(TypedDict):
    amount: str
    asset_address: str
    asset_sub_id: str
    price: str

class RfqMakerActionDataResponse(TypedDict):
    max_fee: str
    trades: list[RfqLegResponse]

class RfqTakerActionDataResponse(TypedDict):
    max_fee: str
    order_hash: str

_RiskUniverseOptional = TypedDict("_RiskUniverseOptional", {'description': 'str | None', 'name': 'str | None'}, total=False)
class RiskUniverse(_RiskUniverseOptional):
    managers: list[RiskUniverseManager]
    risk_universe_id: int | Decimal
    security_module: SecurityModuleDetails

class RiskUniverseManager(TypedDict):
    collaterals: list[ManagerCollateral]
    instruments: list[str]
    manager_id: int | Decimal
    margin_type: MarginType

class ScoreBreakdown(TypedDict):
    coverage_score: str
    holder_boost: str
    quality_score: str
    total_score: str
    volume: str
    volume_multiplier: str
    wallet: str

class SecurityModuleDetails(TypedDict):
    cash_asset: str
    cash_currency: str
    subaccount_id: int | Decimal

_SendQuoteRequestOptional = TypedDict("_SendQuoteRequestOptional", {'client': 'str | None', 'extra_fee': 'str | None', 'label': 'str | None', 'mmp': 'bool | None', 'nonce': 'str', 'referral_code': 'str | None', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class SendQuoteRequest(_SendQuoteRequestOptional):
    direction: Direction
    legs: list[PricedLegParamsAndResponse]
    max_fee: str
    rfq_id: str
    subaccount_id: int | Decimal

_SendRfqRequestOptional = TypedDict("_SendRfqRequestOptional", {'client': 'str | None', 'counterparties': 'list[str] | None', 'extra_fee': 'str | None', 'label': 'str | None', 'max_total_cost': 'str | None', 'min_total_cost': 'str | None', 'partial_fill_step': 'str | None', 'referral_code': 'str | None'}, total=False)
class SendRfqRequest(_SendRfqRequestOptional):
    legs: list[LegUnpricedParams]
    subaccount_id: int | Decimal

class SessionKey(TypedDict):
    expiry_sec: int | Decimal
    ip_whitelist: list[str]
    label: str
    offchain_scopes: list[str]
    protocol_scopes: list[str]
    public_session_key: str
    registered_sec: int | Decimal
    subaccount_ids: list[int | Decimal]

class SessionKeysRequest(TypedDict):
    wallet: str

_SetMmpConfigRequestOptional = TypedDict("_SetMmpConfigRequestOptional", {'mmp_amount_limit': 'str | None', 'mmp_delta_limit': 'str | None'}, total=False)
class SetMmpConfigRequest(_SetMmpConfigRequestOptional):
    currency: str
    mmp_frozen_time: int | Decimal
    mmp_interval: int | Decimal
    subaccount_id: int | Decimal

class SetMmpConfigResponse(TypedDict):
    currency: str
    mmp_amount_limit: str
    mmp_delta_limit: str
    mmp_frozen_time: int | Decimal
    mmp_interval: int | Decimal
    subaccount_id: int | Decimal

class SetSessionKeyActionDataResponse(TypedDict):
    expiry_sec: int | Decimal
    scopes: list[str]
    session_key: str
    subaccounts: list[int | Decimal]

class SetSessionKeyActionInputData(TypedDict):
    data: SetSessionKeyActionDataResponse
    expiry: int | Decimal
    module: str
    nonce: str
    owner: str
    signer: str
    subaccount_id: int | Decimal

_SetSessionKeyDebugResponseOptional = TypedDict("_SetSessionKeyDebugResponseOptional", {'recovered_signer': 'str | None'}, total=False)
class SetSessionKeyDebugResponse(_SetSessionKeyDebugResponseOptional):
    action_hash: str
    action_typehash: str
    domain_separator: str
    encoded_data: str
    encoded_data_hashed: str
    expected_signer: str
    input_data: SetSessionKeyActionInputData
    module: str
    owner: str
    typed_data_hash: str

_SetSessionKeyRequestOptional = TypedDict("_SetSessionKeyRequestOptional", {'ip_whitelist': 'list[str] | None', 'label': 'str | None', 'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'str', 'subaccount_ids': 'list[int | Decimal] | None'}, total=False)
class SetSessionKeyRequest(_SetSessionKeyRequestOptional):
    expiry_sec: int | Decimal
    offchain_scopes: list[str]
    protocol_scopes: list[str]
    public_session_key: str
    wallet: str

_SettledTradeOptional = TypedDict("_SettledTradeOptional", {'batch_status': 'BatchStatus | None'}, total=False)
class SettledTrade(_SettledTradeOptional):
    direction: Direction
    expected_rebate: str
    extra_fee: str
    index_price: str
    instrument_name: str
    liquidity_role: LiquidityRole
    mark_price: str
    quote_id: str | None
    realized_pnl: str
    realized_pnl_excl_fees: str
    rfq_id: str | None
    subaccount_id: int | Decimal
    timestamp: int | Decimal
    trade_amount: str
    trade_fee: str
    trade_id: str
    trade_price: str
    tx_hash: str
    wallet: str

class SignedAction(TypedDict):
    action: Action
    signature: list[int | Decimal]

_SignedTransferQuoteRequestOptional = TypedDict("_SignedTransferQuoteRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'Address'}, total=False)
class SignedTransferQuoteRequest(_SignedTransferQuoteRequestOptional):
    direction: Direction
    legs: list[PricedLegParamsAndResponse]
    max_fee: str
    subaccount_id: int | Decimal

class SimulatedCollateralRequest(TypedDict):
    amount: str
    asset_name: str

_SimulatedPositionRequestOptional = TypedDict("_SimulatedPositionRequestOptional", {'entry_price': 'str | None'}, total=False)
class SimulatedPositionRequest(_SimulatedPositionRequestOptional):
    amount: str
    instrument_name: str

class SpotAssetEntry(TypedDict):
    address: str
    erc20: Erc20Details
    min_deposit_usd: str
    name: str
    universes: list[SpotUniverse]

_SpotFeedDataResponseOptional = TypedDict("_SpotFeedDataResponseOptional", {'feed_source_type': 'str | None'}, total=False)
class SpotFeedDataResponse(_SpotFeedDataResponseOptional):
    confidence: str
    currency: str
    deadline: int | Decimal
    price: str
    signatures: OracleSignatureDataResponse
    timestamp: int | Decimal

class SpotPublicDetails(TypedDict):
    borrow_index: str
    decimals: int | Decimal
    supply_index: str
    underlying_erc20_address: str

_SpotUniverseOptional = TypedDict("_SpotUniverseOptional", {'lending': 'LendingDetails | None'}, total=False)
class SpotUniverse(_SpotUniverseOptional):
    oi: OpenInterestStats
    pm2_im_discount: str
    pm2_mm_discount: str
    risk_universe_id: int | Decimal
    srm_im_discount: str
    srm_mm_discount: str

class SrmPerpMarginRequirements(TypedDict):
    im_perp_req: str
    max_leverage: str
    mm_perp_req: str

class Subaccount(TypedDict):
    collaterals: list[Collateral]
    collaterals_initial_margin: str
    collaterals_maintenance_margin: str
    collaterals_value: str
    currency: list[str]
    failed_to_fetch: bool
    initial_margin: str
    is_under_liquidation: bool
    label: str
    maintenance_margin: str
    manager_id: int | Decimal
    margin_type: str
    mm_credits: str
    open_orders: list[Order]
    open_orders_margin: str
    positions: list[Position]
    positions_initial_margin: str
    positions_maintenance_margin: str
    positions_value: str
    projected_margin_change: str
    risk_universe_id: int | Decimal
    subaccount_id: int | Decimal
    subaccount_value: str
    vault_deposit_holds: list[VaultDepositHold]

class SubaccountValueEntry(TypedDict):
    currency: str
    initial_margin: str
    maintenance_margin: str
    margin_type: str
    subaccount_id: int | Decimal
    subaccount_value: str
    timestamp: int | Decimal

class SubaccountValueHistoryResult(TypedDict):
    pagination: Pagination
    subaccount_value_history: list[SubaccountValueEntry]

_TickerSlimSnapshotOptional = TypedDict("_TickerSlimSnapshotOptional", {'f': 'str | None', 'option_pricing': 'OptionPricing | None'}, total=False)
class TickerSlimSnapshot(_TickerSlimSnapshotOptional):
    A: str
    B: str
    I: str
    M: str
    a: str
    b: str
    maxp: str
    minp: str
    stats: DailyTradingStatistics
    t: int | Decimal

_TradeOptional = TypedDict("_TradeOptional", {'batch_status': 'BatchStatus | None', 'label': 'str | None', 'tx_hash': 'str | None'}, total=False)
class Trade(_TradeOptional):
    direction: Direction
    expected_rebate: str
    extra_fee: str
    index_price: str
    instrument_name: str
    is_transfer: bool
    liquidity_role: LiquidityRole
    mark_price: str
    op_uuid: str
    order_id: str
    quote_id: str | None
    realized_pnl: str
    realized_pnl_excl_fees: str
    rfq_id: str | None
    subaccount_id: int | Decimal
    timestamp: int | Decimal
    trade_amount: str
    trade_fee: str
    trade_id: str
    trade_price: str

_TradeHistoryResponseOptional = TypedDict("_TradeHistoryResponseOptional", {'batch_status': 'BatchStatus | None', 'tx_hash': 'str | None'}, total=False)
class TradeHistoryResponse(_TradeHistoryResponseOptional):
    direction: Direction
    expected_rebate: str
    extra_fee: str
    index_price: str
    instrument_name: str
    is_transfer: bool
    label: str
    liquidity_role: LiquidityRole
    mark_price: str
    op_uuid: str
    order_id: str
    quote_id: str | None
    realized_pnl: str
    realized_pnl_excl_fees: str
    rfq_id: str | None
    subaccount_id: int | Decimal
    timestamp: int | Decimal
    trade_amount: str
    trade_fee: str
    trade_id: str
    trade_price: str

class TradingviewCandle(TypedDict):
    close_price: str
    high_price: str
    low_price: str
    open_price: str
    timestamp: int | Decimal
    timestamp_bucket: int | Decimal
    volume_contracts: str
    volume_usd: str

class TransferActionDataResponse(TypedDict):
    amount: str
    asset: str
    max_fee_usd: str
    new_subaccount_manager: int | Decimal
    sub_id: str
    to_subaccount_id: int | Decimal

_TransferEntryOptional = TypedDict("_TransferEntryOptional", {'tx_hash': 'str | None'}, total=False)
class TransferEntry(_TransferEntryOptional):
    amount: str
    asset: str
    batch_status: BatchStatus
    batch_uuid: str
    fee: str
    from_subaccount_id: int | Decimal
    from_wallet: str
    is_outgoing: bool
    operation_id: str
    timestamp: int | Decimal
    to_subaccount_id: int | Decimal
    to_wallet: str

class TransferHistoryResult(TypedDict):
    transfers: list[TransferEntry]

class TransferPositionsDebugResponse(TypedDict):
    maker_result: TransferPositionsMakerDebugResponse
    taker_result: TransferPositionsTakerDebugResponse

class TransferPositionsMakerActionInputData(TypedDict):
    data: RfqMakerActionDataResponse
    expiry: int | Decimal
    module: str
    nonce: str
    owner: str
    signer: str
    subaccount_id: int | Decimal

_TransferPositionsMakerDebugResponseOptional = TypedDict("_TransferPositionsMakerDebugResponseOptional", {'recovered_signer': 'str | None'}, total=False)
class TransferPositionsMakerDebugResponse(_TransferPositionsMakerDebugResponseOptional):
    action_hash: str
    action_typehash: str
    domain_separator: str
    encoded_data: str
    encoded_data_hashed: str
    expected_signer: str
    input_data: TransferPositionsMakerActionInputData
    module: str
    owner: str
    typed_data_hash: str

class TransferPositionsRequest(TypedDict):
    maker_params: SignedTransferQuoteRequest
    taker_params: SignedTransferQuoteRequest
    wallet: Address

class TransferPositionsResponse(TypedDict):
    maker_quote: Quote
    taker_quote: Quote

class TransferPositionsTakerActionInputData(TypedDict):
    data: RfqTakerActionDataResponse
    expiry: int | Decimal
    module: str
    nonce: str
    owner: str
    signer: str
    subaccount_id: int | Decimal

_TransferPositionsTakerDebugResponseOptional = TypedDict("_TransferPositionsTakerDebugResponseOptional", {'recovered_signer': 'str | None'}, total=False)
class TransferPositionsTakerDebugResponse(_TransferPositionsTakerDebugResponseOptional):
    action_hash: str
    action_typehash: str
    domain_separator: str
    encoded_data: str
    encoded_data_hashed: str
    expected_signer: str
    input_data: TransferPositionsTakerActionInputData
    module: str
    owner: str
    typed_data_hash: str

class TransferSpotActionInputData(TypedDict):
    data: TransferActionDataResponse
    expiry: int | Decimal
    module: str
    nonce: str
    owner: str
    signer: str
    subaccount_id: int | Decimal

_TransferSpotDebugResponseOptional = TypedDict("_TransferSpotDebugResponseOptional", {'recovered_signer': 'str | None'}, total=False)
class TransferSpotDebugResponse(_TransferSpotDebugResponseOptional):
    action_hash: str
    action_typehash: str
    domain_separator: str
    encoded_data: str
    encoded_data_hashed: str
    expected_signer: str
    input_data: TransferSpotActionInputData
    module: str
    owner: str
    typed_data_hash: str

class TransferSpotExternalActionInputData(TypedDict):
    data: ExternalTransferActionDataResponse
    expiry: int | Decimal
    module: str
    nonce: str
    owner: str
    signer: str
    subaccount_id: int | Decimal

_TransferSpotExternalDebugResponseOptional = TypedDict("_TransferSpotExternalDebugResponseOptional", {'recovered_signer': 'str | None'}, total=False)
class TransferSpotExternalDebugResponse(_TransferSpotExternalDebugResponseOptional):
    action_hash: str
    action_typehash: str
    domain_separator: str
    encoded_data: str
    encoded_data_hashed: str
    expected_signer: str
    input_data: TransferSpotExternalActionInputData
    module: str
    owner: str
    typed_data_hash: str

_UniverseManagersOptional = TypedDict("_UniverseManagersOptional", {'pm': 'int | Decimal | None', 'risk_universe_name': 'str | None', 'sm': 'int | Decimal | None'}, total=False)
class UniverseManagers(_UniverseManagersOptional):
    risk_universe_id: int | Decimal

_UpdateVaultInfoRequestOptional = TypedDict("_UpdateVaultInfoRequestOptional", {'description': 'str | None', 'mtm_cap': 'str | None', 'name': 'str | None', 'whitelist_only': 'bool | None'}, total=False)
class UpdateVaultInfoRequest(_UpdateVaultInfoRequestOptional):
    subaccount_id: int | Decimal

class UpdateWhitelistedRecipientsActionDataResponse(TypedDict):
    add: list[str]
    remove: list[str]

class UpdateWhitelistedRecipientsActionInputData(TypedDict):
    data: UpdateWhitelistedRecipientsActionDataResponse
    expiry: int | Decimal
    module: str
    nonce: str
    owner: str
    signer: str
    subaccount_id: int | Decimal

_UpdateWhitelistedRecipientsDebugResponseOptional = TypedDict("_UpdateWhitelistedRecipientsDebugResponseOptional", {'recovered_signer': 'str | None'}, total=False)
class UpdateWhitelistedRecipientsDebugResponse(_UpdateWhitelistedRecipientsDebugResponseOptional):
    action_hash: str
    action_typehash: str
    domain_separator: str
    encoded_data: str
    encoded_data_hashed: str
    expected_signer: str
    input_data: UpdateWhitelistedRecipientsActionInputData
    module: str
    owner: str
    typed_data_hash: str

_UpdateWhitelistedRecipientsRequestOptional = TypedDict("_UpdateWhitelistedRecipientsRequestOptional", {'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'str'}, total=False)
class UpdateWhitelistedRecipientsRequest(_UpdateWhitelistedRecipientsRequestOptional):
    add: list[str]
    remove: list[str]
    wallet: str

class UpdateWhitelistedRecipientsResponse(TypedDict):
    op_uuid: str
    operation_id: int | Decimal
    whitelisted_recipients: list[str]

_VaultOptional = TypedDict("_VaultOptional", {'benchmark_price': 'str | None', 'mtm_cap': 'str | None', 'nav_benchmark': 'str | None', 'nav_usd': 'str | None', 'simulated_share_price_usd': 'str | None'}, total=False)
class Vault(_VaultOptional):
    curator: Address
    curator_shares: str
    description: str
    name: str
    protocol: ProtocolVault
    whitelist_only: bool

class VaultActionResponse(TypedDict):
    after_shares: str
    amount: str
    before_shares: str
    creation_timestamp_ms: int | Decimal
    entry_price: str
    error_reason: str
    event_ts: int | Decimal
    event_type: str
    exit_price: str
    operation_id: int | Decimal
    operation_uuid: str
    share_price: str
    shares_delta: str
    shares_requested: str
    status: str
    user_action_hash: str
    vault_nonce: str
    vault_subaccount_id: int | Decimal
    wallet: str

class VaultCancelResponse(TypedDict):
    cancelled_request_ids: list[VaultRequestId]
    op_uuid: str
    operation_id: int | Decimal

_VaultConfigOptional = TypedDict("_VaultConfigOptional", {'benchmark_asset': 'Address | None'}, total=False)
class VaultConfig(_VaultConfigOptional):
    cooldown_sec: int | Decimal
    deposit_spot_asset: Address
    management_fee_bps: int | Decimal
    max_slippage_bps: int | Decimal
    performance_fee_bps: int | Decimal

class VaultCreateResponse(TypedDict):
    op_uuid: str
    operation_id: int | Decimal

class VaultDepositHold(TypedDict):
    amount: str
    asset_name: str
    currency: str
    vault_id: int | Decimal

class VaultForceBurnResponse(TypedDict):
    op_uuid: str
    operation_id: int | Decimal

class VaultIdsResponse(TypedDict):
    subaccount_ids: list[int | Decimal]

class VaultPerformanceHistoryResult(TypedDict):
    points: list[VaultPerformancePointResponse]
    resolution: PerformanceResolution
    subaccount_id: int | Decimal

class VaultPerformancePointResponse(TypedDict):
    benchmark_price: str | None
    curator_shares: str
    global_hwm: str
    nav: str | None
    nav_benchmark: str | None
    share_price: str
    total_shares: str
    ts: int | Decimal

class VaultRequestAckResponse(TypedDict):
    request_id: VaultRequestId

class VaultRequestId(TypedDict):
    vault_nonce: str
    vault_subaccount_id: int | Decimal
    wallet: Address

class VaultRequestResponse(TypedDict):
    creation_timestamp_ms: int | Decimal
    id: VaultRequestId
    signed_action: SignedAction
    subaccount_id: int | Decimal
    user_action_hash: str
    wallet: Address

class VaultSettleResponse(TypedDict):
    op_uuid: str
    operation_id: int | Decimal

class VaultShareEntryResponse(TypedDict):
    shares: str
    vault: Vault

class VaultSharesResponse(TypedDict):
    vaults: list[VaultShareEntryResponse]

class VaultsResponse(TypedDict):
    pagination: Pagination
    vaults: list[Vault]

class VolFeedDataResponse(TypedDict):
    confidence: str
    currency: str
    deadline: int | Decimal
    expiry: int | Decimal
    signatures: OracleSignatureDataResponse
    timestamp: int | Decimal
    vol_data: VolSVIParamDataResponse

class VolSVIParamDataResponse(TypedDict):
    SVI_a: str
    SVI_b: str
    SVI_fwd: str
    SVI_m: str
    SVI_refTau: str
    SVI_rho: str
    SVI_sigma: str

class WithdrawAddressEntry(TypedDict):
    address_kind: AddressKind
    chain_id: int | Decimal
    recipient: str
    recipient_display: str
    whitelisted: bool
    withdraw_address: str

_WithdrawalEntryOptional = TypedDict("_WithdrawalEntryOptional", {'forward': 'WithdrawalForward | None', 'tx_hash': 'str | None'}, total=False)
class WithdrawalEntry(_WithdrawalEntryOptional):
    amount: str
    asset: str
    batch_status: BatchStatus
    batch_uuid: str
    erc20_address: str
    fee: str
    operation_id: str
    recipient: str
    subaccount_id: int | Decimal
    timestamp: int | Decimal
    wallet: str

_WithdrawalForwardOptional = TypedDict("_WithdrawalForwardOptional", {'destination_tx': 'str | None', 'forward_tx': 'str | None', 'hold_reason': 'str | None', 'message_id': 'str | None', 'rail': 'str | None'}, total=False)
class WithdrawalForward(_WithdrawalForwardOptional):
    amount_forwarded: str | None
    chain_id: int | Decimal
    recipient: str
    status: WithdrawalForwardStatus

class WithdrawalHistoryResult(TypedDict):
    withdrawals: list[WithdrawalEntry]

_LoginRequestOptional = TypedDict("_LoginRequestOptional", {'signature': 'str | None', 'timestamp': 'int | Decimal | None', 'wallet': 'str | None'}, total=False)
class LoginRequest(_LoginRequestOptional):
    pass

_SetCancelOnDisconnectRequestOptional = TypedDict("_SetCancelOnDisconnectRequestOptional", {'enabled': 'bool | None', 'wallet': 'str | None'}, total=False)
class SetCancelOnDisconnectRequest(_SetCancelOnDisconnectRequestOptional):
    pass

class SubscribeParams(TypedDict):
    channels: list[str]

class SubscribeResult(TypedDict):
    current_subscriptions: list[str]
    status: dict[str, Any]

_UnsubscribeParamsOptional = TypedDict("_UnsubscribeParamsOptional", {'channels': 'list[str] | None'}, total=False)
class UnsubscribeParams(_UnsubscribeParamsOptional):
    pass

class UnsubscribeResult(TypedDict):
    remaining_subscriptions: list[str]
    status: dict[str, Any]

_AuctionDetailsSubaccountBalancesOptional = TypedDict("_AuctionDetailsSubaccountBalancesOptional", {'{key}': 'str | None'}, total=False)
class AuctionDetailsSubaccountBalances(_AuctionDetailsSubaccountBalancesOptional):
    pass

_AuctionDetailsOptional = TypedDict("_AuctionDetailsOptional", {'currency': 'str | None'}, total=False)
class AuctionDetails(_AuctionDetailsOptional):
    estimated_bid_price: str
    estimated_discount_pnl: str
    estimated_mtm: str
    estimated_percent_bid: str
    margin_type: str
    min_price_limit: str
    risk_universe_id: int | Decimal
    subaccount_balances: AuctionDetailsSubaccountBalances

_AuctionResultOptional = TypedDict("_AuctionResultOptional", {'details': 'AuctionDetails | None'}, total=False)
class AuctionResult(_AuctionResultOptional):
    state: AuctionStateType
    subaccount_id: int | Decimal
    timestamp: int | Decimal

class BalanceUpdate(TypedDict):
    name: str
    new_balance: str
    previous_balance: str
    update_type: BalanceUpdateType

_BestQuoteChannelResultOptional = TypedDict("_BestQuoteChannelResultOptional", {'error': 'RPCError | None', 'result': 'RfqGetBestQuoteResponse | None'}, total=False)
class BestQuoteChannelResult(_BestQuoteChannelResultOptional):
    rfq_id: str

class MarginWatchResult(TypedDict):
    collaterals: list[Any]
    currency: str
    initial_margin: str
    maintenance_margin: str
    margin_type: str
    positions: list[Any]
    subaccount_id: int | Decimal
    subaccount_value: str
    valuation_timestamp: int | Decimal

class OrderbookPayloadDoc(TypedDict):
    asks: list[list[str]]
    bids: list[list[str]]
    instrument_name: str
    publish_id: int | Decimal
    timestamp: int | Decimal

_PublicTradeOptional = TypedDict("_PublicTradeOptional", {'quote_id': 'str | None', 'rfq_id': 'str | None'}, total=False)
class PublicTrade(_PublicTradeOptional):
    direction: Direction
    expected_rebate: str
    extra_fee: str
    index_price: str
    instrument_name: str
    liquidity_role: LiquidityRole2
    mark_price: str
    realized_pnl: str
    realized_pnl_excl_fees: str
    subaccount_id: int | Decimal
    timestamp: int | Decimal
    trade_amount: str
    trade_fee: str
    trade_id: str
    trade_price: str
    wallet: str

_QuotePublishResultOptional = TypedDict("_QuotePublishResultOptional", {'batch_status': 'BatchStatus | None', 'cancel_reason': 'RFQCancelReason | None', 'nonce': 'str', 'signature': 'str', 'signature_expiry_sec': 'int | Decimal', 'signer': 'str', 'tx_hash': 'str | None'}, total=False)
class QuotePublishResult(_QuotePublishResultOptional):
    creation_timestamp: int | Decimal
    direction: Direction
    extra_fee: str
    fee: str
    fill_pct: str
    is_transfer: bool
    label: str
    last_update_timestamp: int | Decimal
    legs: list[PricedLegParamsAndResponse]
    legs_hash: str
    liquidity_role: LiquidityRole
    max_fee: str
    mmp: bool
    quote_id: str
    rfq_id: str
    status: RFQStatus
    subaccount_id: int | Decimal

class SpotFeedEntry(TypedDict):
    confidence: str
    confidence_prev_daily: str
    price: str
    price_prev_daily: str
    timestamp_prev_daily: int | Decimal

_SpotFeedPayloadFeedsOptional = TypedDict("_SpotFeedPayloadFeedsOptional", {'{key}': 'SpotFeedEntry | None'}, total=False)
class SpotFeedPayloadFeeds(_SpotFeedPayloadFeedsOptional):
    pass

class SpotFeedPayload(TypedDict):
    feeds: SpotFeedPayloadFeeds
    timestamp: int | Decimal

class TickerSlimPayload(TypedDict):
    instrument_ticker: TickerSlimSnapshot
    timestamp: int | Decimal

Address = str
AddressKind = str
AlgoType = str
AssetType = str
AuctionType = str
BatchStatus = str
CancelAllAlgoOrdersResponse = str
CancelAllResponse = str
CancelAllTriggerOrdersResponse = str
CancelReason = str
CancelRfqResponse = str
DecodableActionType = str
Direction = str
LiquidityRole = str
MarginType = str
MarketType = str
MonitorType = str
OffchainKeyScope = str
OrderStatus = str
OrderType = str
PerformanceResolution = str
RFQCancelReason = str
RFQStatus = str
ResetMmpResponse = str
Severity = str
TimeInForce = str
TriggerPriceType = str
TriggerType = str
WithdrawalForwardStatus = str
JsonRpcId = Any
SetCancelOnDisconnectResponse = str
AuctionStateType = str
BalanceUpdateType = str
LiquidityRole2 = str
