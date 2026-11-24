// Generated from src/exchanges/derive/schema/derive.json.
import type { HttpValue } from "./index";
export type Address = string;
export type AddressKind = string;
export type AlgoType = string;
export type AssetType = string;
export type AuctionType = string;
export type BatchStatus = string;
export type CancelAllAlgoOrdersResponse = string;
export type CancelAllResponse = string;
export type CancelAllTriggerOrdersResponse = string;
export type CancelReason = string;
export type CancelRfqResponse = string;
export type DecodableActionType = string;
export type Direction = string;
export type LiquidityRole = string;
export type MarginType = string;
export type MarketType = string;
export type MonitorType = string;
export type OffchainKeyScope = string;
export type OrderStatus = string;
export type OrderType = string;
export type PerformanceResolution = string;
export type RFQCancelReason = string;
export type RFQStatus = string;
export type ResetMmpResponse = string;
export type Severity = string;
export type TimeInForce = string;
export type TriggerPriceType = string;
export type TriggerType = string;
export type WithdrawalForwardStatus = string;
export type JsonRpcId = HttpValue;
export type SetCancelOnDisconnectResponse = string;
export type AuctionStateType = string;
export type BalanceUpdateType = string;
export type LiquidityRole2 = string;

export interface AccountFeeInfo {
  [field: string]: unknown;
  "base_fee_discount": string;
  "option_maker_fee"?: string | null;
  "option_taker_fee"?: string | null;
  "perp_maker_fee"?: string | null;
  "perp_taker_fee"?: string | null;
  "rfq_maker_discount": string;
  "rfq_taker_discount": string;
  "spot_maker_fee"?: string | null;
  "spot_taker_fee"?: string | null;
}

export interface Action {
  [field: string]: unknown;
  "data": Array<number | bigint | string>;
  "expiry": number | bigint | string;
  "module": Address;
  "nonce": number | bigint | string;
  "owner": Address;
  "signer": Address;
  "subaccount_id": number | bigint | string;
}

export interface AggregatedOrdersResult {
  [field: string]: unknown;
  "orders": Array<Order>;
  "subaccount_id": number | bigint | string;
}

export interface AggregatedTriggerOrdersResult {
  [field: string]: unknown;
  "orders": Array<Order>;
  "subaccount_id": number | bigint | string;
}

export interface AssetEntry {
  [field: string]: unknown;
  "address": string;
  "name": string;
  "universes": Array<AssetUniverse>;
}

export interface AssetUniverse {
  [field: string]: unknown;
  "oi": OpenInterestStats;
  "risk_universe_id": number | bigint | string;
  "risk_universe_name"?: string | null;
  "srm_perp_margin_requirements"?: SrmPerpMarginRequirements | null;
}

export interface AuctionBidEvent {
  [field: string]: unknown;
  "amounts_liquidated": Record<string, string>;
  "bidder_id": number | bigint | string;
  "cash_received": string;
  "discount_pnl": string;
  "operation_id": string;
  "percent_liquidated": string;
  "positions_realized_pnl": Record<string, string>;
  "positions_realized_pnl_excl_fees": Record<string, string>;
  "realized_pnl": string;
  "realized_pnl_excl_fees": string;
  "risk_universe_id": number | bigint | string;
  "timestamp": number | bigint | string;
  "tx_hash": string;
  "unit_price": string;
}

export interface AuctionHistory {
  [field: string]: unknown;
  "auction_id": string;
  "auction_type": AuctionType;
  "bids": Array<AuctionBidEvent>;
  "end_timestamp"?: number | bigint | string | null;
  "fee": string;
  "risk_universe_id": number | bigint | string;
  "start_timestamp": number | bigint | string;
  "subaccount_id": number | bigint | string;
  "tx_hash": string;
}

export interface BurnSharesRequest {
  [field: string]: unknown;
  "nonce"?: string;
  "request_id": VaultRequestId;
  "share_price": string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
  "withdraw_hash": string;
}

export interface CancelAlgoOrderRequest {
  [field: string]: unknown;
  "order_id": string;
  "subaccount_id": number | bigint | string;
}

export interface CancelAllAlgoOrdersRequest {
  [field: string]: unknown;
  "subaccount_id": number | bigint | string;
}

export interface CancelAllRequest {
  [field: string]: unknown;
  "cancel_algo_orders"?: boolean | null;
  "cancel_trigger_orders"?: boolean | null;
  "subaccount_id": number | bigint | string;
}

export interface CancelAllTriggerOrdersRequest {
  [field: string]: unknown;
  "subaccount_id": number | bigint | string;
}

export interface CancelBatchQuotesRequest {
  [field: string]: unknown;
  "label"?: string | null;
  "nonce"?: string | null;
  "quote_id"?: string | null;
  "rfq_id"?: string | null;
  "subaccount_id": number | bigint | string;
}

export interface CancelBatchResult {
  [field: string]: unknown;
  "cancelled_ids": Array<string>;
}

export interface CancelBatchRfqsRequest {
  [field: string]: unknown;
  "label"?: string | null;
  "nonce"?: string | null;
  "rfq_id"?: string | null;
  "subaccount_id": number | bigint | string;
}

export interface CancelBatchRfqsResponse {
  [field: string]: unknown;
  "cancelled_ids": Array<string>;
}

export interface CancelByInstrumentRequest {
  [field: string]: unknown;
  "instrument_name": string;
  "subaccount_id": number | bigint | string;
}

export interface CancelByInstrumentResponse {
  [field: string]: unknown;
  "cancelled_orders": number | bigint | string;
}

export interface CancelByLabelRequest {
  [field: string]: unknown;
  "instrument_name"?: string | null;
  "label": string;
  "subaccount_id": number | bigint | string;
}

export interface CancelByLabelResponse {
  [field: string]: unknown;
  "cancelled_orders": number | bigint | string;
}

export interface CancelByNonceRequest {
  [field: string]: unknown;
  "instrument_name": string;
  "nonce"?: string;
  "subaccount_id": number | bigint | string;
}

export interface CancelByNonceResponse {
  [field: string]: unknown;
  "cancelled_orders": number | bigint | string;
}

export interface CancelOrderRequest {
  [field: string]: unknown;
  "instrument_name": string;
  "order_id": string;
  "subaccount_id": number | bigint | string;
}

export interface CancelQuoteRequest {
  [field: string]: unknown;
  "label"?: string | null;
  "nonce"?: string | null;
  "quote_id": string;
  "rfq_id"?: string | null;
  "subaccount_id": number | bigint | string;
}

export interface CancelRfqRequest {
  [field: string]: unknown;
  "rfq_id": string;
  "subaccount_id": number | bigint | string;
}

export interface CancelTriggerOrderRequest {
  [field: string]: unknown;
  "order_id": string;
  "subaccount_id": number | bigint | string;
}

export interface CancelVaultRequestRequest {
  [field: string]: unknown;
  "nonce"?: string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
  "vault_subaccount_id": number | bigint | string;
}

export interface ChangeSubaccountLabelRequest {
  [field: string]: unknown;
  "label": string;
  "subaccount_id": number | bigint | string;
}

export interface Collateral {
  [field: string]: unknown;
  "amount": string;
  "amount_step": string;
  "asset_name": string;
  "asset_type": string;
  "average_price": string;
  "average_price_excl_fees": string;
  "creation_timestamp": number | bigint | string;
  "cumulative_interest": string;
  "currency": string;
  "delta": string;
  "delta_currency": string;
  "initial_margin": string;
  "maintenance_margin": string;
  "mark_price": string;
  "mark_value": string;
  "open_orders_margin": string;
  "pending_interest": string;
  "realized_pnl": string;
  "realized_pnl_excl_fees": string;
  "total_fees": string;
  "unrealized_pnl": string;
  "unrealized_pnl_excl_fees": string;
}

export interface CreateOrderRequest {
  [field: string]: unknown;
  "algo_duration_sec"?: number | bigint | string | null;
  "algo_num_slices"?: number | bigint | string | null;
  "algo_type"?: AlgoType | null;
  "amount": string;
  "client"?: string | null;
  "direction": Direction;
  "extra_fee"?: string | null;
  "instrument_name": string;
  "is_atomic_signing"?: boolean | null;
  "label"?: string | null;
  "limit_price": string;
  "max_fee": string;
  "mmp"?: boolean | null;
  "nonce"?: string;
  "order_type"?: OrderType | null;
  "reduce_only"?: boolean | null;
  "referral_code"?: string | null;
  "reject_post_only"?: boolean | null;
  "reject_timestamp"?: number | bigint | string | null;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: string;
  "subaccount_id": number | bigint | string;
  "time_in_force"?: TimeInForce | null;
  "trigger_price"?: string | null;
  "trigger_price_type"?: TriggerPriceType | null;
  "trigger_type"?: TriggerType | null;
}

export interface CreateVaultRequest {
  [field: string]: unknown;
  "benchmark_asset"?: Address | null;
  "cooldown_sec": number | bigint | string;
  "deposit_spot_asset": Address;
  "initial_deposit": string;
  "initial_share_price_usd": string;
  "management_fee_bps": number | bigint | string;
  "manager_id": number | bigint | string;
  "max_fee_usd": string;
  "max_slippage_bps": number | bigint | string;
  "nonce"?: string;
  "performance_fee_bps": number | bigint | string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
}

export interface Currency {
  [field: string]: unknown;
  "currency": string;
  "managers": Array<UniverseManagers>;
  "market_type": MarketType;
  "option"?: AssetEntry | null;
  "perp"?: AssetEntry | null;
  "spot": Array<SpotAssetEntry>;
  "spot_price": string;
  "spot_price_24h"?: string | null;
}

export interface DailyTradingStatistics {
  [field: string]: unknown;
  "c": string;
  "h": string;
  "l": string;
  "n": number | bigint | string;
  "oi": string;
  "p": string;
  "pr": string;
  "v": string;
}

export interface DecodeActionRequest {
  [field: string]: unknown;
  "action_type": DecodableActionType;
  "encoded_data": string;
}

export interface DecodeActionResponse {
  [field: string]: unknown;
  "action_type": DecodableActionType;
  "decoded": HttpValue;
}

export interface DeleteSubaccountRequest {
  [field: string]: unknown;
  "nonce"?: string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: string;
  "subaccount_id": number | bigint | string;
}

export interface DeleteSubaccountResponse {
  [field: string]: unknown;
  "op_uuid": string;
  "operation_id": number | bigint | string;
}

export interface DepositBridgeOrigin {
  [field: string]: unknown;
  "rail": string;
  "source_address"?: string | null;
  "source_chain_id": number | bigint | string;
  "source_tx_hash": string;
}

export interface DepositEntry {
  [field: string]: unknown;
  "action_id"?: number | bigint | string | null;
  "amount": string;
  "asset": string;
  "batch_status": BatchStatus;
  "batch_uuid": string;
  "bridge_origin"?: DepositBridgeOrigin | null;
  "fallback_error_code"?: number | bigint | string | null;
  "fallback_error_data"?: string | null;
  "fallback_error_message"?: string | null;
  "fee": string;
  "is_fallback": boolean;
  "l1_sender"?: string | null;
  "l1_tx_hash"?: string | null;
  "new_subaccount": boolean;
  "operation_id": string;
  "subaccount_id": number | bigint | string;
  "timestamp": number | bigint | string;
  "tx_hash"?: string | null;
  "wallet": string;
}

export interface DepositHistoryResult {
  [field: string]: unknown;
  "deposits": Array<DepositEntry>;
}

export interface EditSessionKeyRequest {
  [field: string]: unknown;
  "ip_whitelist"?: Array<string> | null;
  "label"?: string | null;
  "offchain_scopes"?: Array<string> | null;
  "public_session_key": string;
  "wallet": string;
}

export interface EmptyRequest {
  [field: string]: unknown;
}

export interface Erc20Details {
  [field: string]: unknown;
  "decimals": number | bigint | string;
  "underlying_erc20"?: string | null;
}

export interface ExecuteQuoteRequest {
  [field: string]: unknown;
  "client"?: string | null;
  "direction": Direction;
  "enable_taker_protection"?: boolean | null;
  "label"?: string | null;
  "legs": Array<PricedLegParamsAndResponse>;
  "max_fee": string;
  "nonce"?: string;
  "quote_id": string;
  "referral_code"?: string | null;
  "rfq_id": string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
}

export interface ExpirySettlementPrice {
  [field: string]: unknown;
  "expiry_date": string;
  "price"?: string | null;
  "utc_expiry_sec": number | bigint | string;
}

export interface ExternalTransferActionDataResponse {
  [field: string]: unknown;
  "amount": string;
  "asset": string;
  "max_fee_usd": string;
  "new_subaccount_manager": number | bigint | string;
  "recipient_address": string;
  "sub_id": string;
  "to_subaccount_id": number | bigint | string;
}

export interface ForceBurnRequest {
  [field: string]: unknown;
  "holder": Address;
  "nonce"?: string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
}

export interface ForwardFeedDataResponse {
  [field: string]: unknown;
  "confidence": string;
  "currency": string;
  "deadline": number | bigint | string;
  "expiry": number | bigint | string;
  "fwd_diff": string;
  "signatures": OracleSignatureDataResponse;
  "spot_aggregate_latest": string;
  "spot_aggregate_start": string;
  "timestamp": number | bigint | string;
}

export interface ForwardedToken {
  [field: string]: unknown;
  "bridge_type": string;
  "symbol": string;
  "token": string;
}

export interface FundingFeedDataResponse {
  [field: string]: unknown;
  "confidence": string;
  "currency": string;
  "deadline": number | bigint | string;
  "funding_rate": string;
  "signatures": OracleSignatureDataResponse;
  "timestamp": number | bigint | string;
}

export interface FundingRateCandle {
  [field: string]: unknown;
  "close": string;
  "currency": string;
  "funding_rate": string;
  "high": string;
  "low": string;
  "open": string;
  "risk_universe_id": number | bigint | string;
  "timestamp": number | bigint | string;
}

export interface FundingRateHistoryResult {
  [field: string]: unknown;
  "funding_rate_history": Array<FundingRateCandle>;
}

export interface GetAccountRequest {
  [field: string]: unknown;
  "wallet": Address;
}

export interface GetAlgoOrdersRequest {
  [field: string]: unknown;
  "subaccount_id": number | bigint | string;
}

export interface GetAllInstrumentsRequest {
  [field: string]: unknown;
  "currency"?: string | null;
  "expired": boolean;
  "instrument_type": AssetType;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "risk_universe_id"?: number | bigint | string | null;
}

export interface GetAllInstrumentsResponse {
  [field: string]: unknown;
  "instruments": Array<Instrument>;
  "pagination": Pagination;
}

export interface GetAllPortfoliosRequest {
  [field: string]: unknown;
  "wallet": Address;
}

export interface GetAllReferralCodesParams {
  [field: string]: unknown;
}

export interface GetCollateralsRequest {
  [field: string]: unknown;
  "subaccount_id": number | bigint | string;
}

export interface GetCuratedVaultsRequest {
  [field: string]: unknown;
  "wallet": string;
}

export interface GetCurrencyRequest {
  [field: string]: unknown;
  "currency": string;
}

export interface GetDepositHistoryRequest {
  [field: string]: unknown;
  "end_timestamp"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
  "subaccount_id"?: number | bigint | string | null;
  "wallet"?: string | null;
}

export interface GetErc20TransferHistoryRequest {
  [field: string]: unknown;
  "end_timestamp"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
  "subaccount_id"?: number | bigint | string | null;
  "wallet"?: string | null;
}

export interface GetFundingHistoryRequest {
  [field: string]: unknown;
  "end_timestamp"?: number | bigint | string | null;
  "instrument_name"?: string | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
  "subaccount_id"?: number | bigint | string | null;
  "wallet"?: string | null;
}

export interface GetFundingRateHistoryRequest {
  [field: string]: unknown;
  "end_timestamp"?: number | bigint | string | null;
  "instrument_name": string;
  "period"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
}

export interface GetIndexChartDataRequest {
  [field: string]: unknown;
  "currency": string;
  "end_timestamp": number | bigint | string;
  "period": number | bigint | string;
  "start_timestamp": number | bigint | string;
}

export interface GetInstrumentRequest {
  [field: string]: unknown;
  "instrument_name": string;
}

export interface GetInterestHistoryRequest {
  [field: string]: unknown;
  "end_timestamp"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
  "subaccount_id"?: number | bigint | string | null;
  "wallet"?: string | null;
}

export interface GetInterestRateHistoryRequest {
  [field: string]: unknown;
  "currency": string;
  "end_timestamp"?: number | bigint | string | null;
  "period"?: number | bigint | string | null;
  "risk_universe_id"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
}

export interface GetLatestSignedFeedsRequest {
  [field: string]: unknown;
  "currency"?: string | null;
  "expiry"?: number | bigint | string | null;
}

export interface GetLatestSignedFeedsResponse {
  [field: string]: unknown;
  "funding_data": Record<string, FundingFeedDataResponse>;
  "fwd_data": Record<string, Record<string, ForwardFeedDataResponse>>;
  "perp_data": Record<string, Record<string, PerpFeedDataResponse>>;
  "rate_data": Record<string, Record<string, RateFeedDataResponse>>;
  "spot_data": Record<string, SpotFeedDataResponse>;
  "vol_data": Record<string, Record<string, VolFeedDataResponse>>;
}

export interface GetLiquidationHistoryRequest {
  [field: string]: unknown;
  "end_timestamp"?: number | bigint | string | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
  "subaccount_id"?: number | bigint | string | null;
}

export interface GetLiveBurnRequestsRequest {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "subaccount_id": number | bigint | string;
}

export interface GetLiveIncidentsParams {
  [field: string]: unknown;
}

export interface GetLiveIncidentsResponse {
  [field: string]: unknown;
  "incidents": Array<IncidentResponse>;
}

export interface GetLiveMintRequestsRequest {
  [field: string]: unknown;
  "limit": number | bigint | string;
  "subaccount_id": number | bigint | string;
}

export interface GetLiveVaultRequestsRequest {
  [field: string]: unknown;
  "wallet": string;
}

export interface GetMakerProgramScoresParams {
  [field: string]: unknown;
  "epoch_start_timestamp": number | bigint | string;
  "program_name": string;
}

export interface GetMakerProgramScoresResponse {
  [field: string]: unknown;
  "program": ProgramResponse;
  "scores": Array<ScoreBreakdown>;
  "total_score": string;
  "total_volume": string;
}

export interface GetMakerProgramsParams {
  [field: string]: unknown;
}

export interface GetMakerRfqHistoryRequest {
  [field: string]: unknown;
  "from_timestamp"?: number | bigint | string | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "rfq_id"?: string | null;
  "rfq_subaccount_id"?: number | bigint | string | null;
  "status"?: RFQStatus | null;
  "subaccount_id": number | bigint | string;
  "to_timestamp"?: number | bigint | string | null;
}

export interface GetMarginResponse {
  [field: string]: unknown;
  "is_valid_trade": boolean;
  "post_initial_margin": string;
  "post_maintenance_margin": string;
  "pre_initial_margin": string;
  "pre_maintenance_margin": string;
  "subaccount_id": number | bigint | string;
}

export interface GetOnchainActionHistoryParams {
  [field: string]: unknown;
  "action_type"?: number | bigint | string | null;
  "end_timestamp"?: number | bigint | string | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
  "wallet"?: string | null;
}

export interface GetOnchainActionHistoryResponse {
  [field: string]: unknown;
  "actions": Array<OnchainActionHistoryEntry>;
  "pagination": Pagination;
}

export interface GetOpenOrdersRequest {
  [field: string]: unknown;
  "subaccount_id": number | bigint | string;
}

export interface GetOptionSettlementHistoryParams {
  [field: string]: unknown;
  "subaccount_id"?: number | bigint | string | null;
  "wallet"?: string | null;
}

export interface GetOptionSettlementPricesRequest {
  [field: string]: unknown;
  "currency": string;
}

export interface GetOrderHistoryRequest {
  [field: string]: unknown;
  "from_timestamp"?: number | bigint | string | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "subaccount_id"?: number | bigint | string | null;
  "to_timestamp"?: number | bigint | string | null;
  "wallet"?: string | null;
}

export interface GetOrderRequest {
  [field: string]: unknown;
  "order_id": string;
  "subaccount_id": number | bigint | string;
}

export interface GetPendingDepositsParams {
  [field: string]: unknown;
  "wallet": string;
}

export interface GetPendingDepositsResult {
  [field: string]: unknown;
  "pending_deposits": Array<PendingDepositEntry>;
  "wallet": string;
}

export interface GetPerpImpactTwapRequest {
  [field: string]: unknown;
  "currency": string;
  "end_time": number | bigint | string;
  "start_time": number | bigint | string;
}

export interface GetPositionsRequest {
  [field: string]: unknown;
  "subaccount_id": number | bigint | string;
}

export interface GetPublicTradeHistoryRequest {
  [field: string]: unknown;
  "batch_status"?: BatchStatus | null;
  "currency"?: string | null;
  "from_timestamp"?: number | bigint | string | null;
  "instrument_name"?: string | null;
  "instrument_type"?: AssetType | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "subaccount_id"?: number | bigint | string | null;
  "to_timestamp"?: number | bigint | string | null;
  "trade_id"?: string | null;
}

export interface GetQuotesRequest {
  [field: string]: unknown;
  "from_timestamp"?: number | bigint | string | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "quote_id"?: string | null;
  "rfq_id"?: string | null;
  "status"?: string | null;
  "subaccount_id": number | bigint | string;
  "to_timestamp"?: number | bigint | string | null;
}

export interface GetReferralPerformanceParams {
  [field: string]: unknown;
  "end_ms": number | bigint | string;
  "referral_code"?: string | null;
  "start_ms": number | bigint | string;
  "wallet"?: string | null;
}

export interface GetReferralPerformanceResult {
  [field: string]: unknown;
  "fee_share_percentage": string;
  "referral_code": string;
  "rewards": Record<string, Record<string, Record<string, ReferralPerformanceByInstrumentType>>>;
  "stdrv_balance": string;
  "total_builder_fee_collected": string;
  "total_fee_rewards": string;
  "total_notional_volume": string;
  "total_referred_fees": string;
}

export interface GetRfqsRequest {
  [field: string]: unknown;
  "from_timestamp"?: number | bigint | string | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "rfq_id"?: string | null;
  "status"?: string | null;
  "subaccount_id": number | bigint | string;
  "to_timestamp"?: number | bigint | string | null;
}

export interface GetShareholderVaultsRequest {
  [field: string]: unknown;
  "wallet": string;
}

export interface GetSubaccountRequest {
  [field: string]: unknown;
  "subaccount_id": number | bigint | string;
}

export interface GetSubaccountValueHistoryRequest {
  [field: string]: unknown;
  "end_timestamp"?: number | bigint | string | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "period"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
  "subaccount_id"?: number | bigint | string | null;
  "wallet"?: string | null;
}

export interface GetSubaccountsRequest {
  [field: string]: unknown;
  "wallet": Address;
}

export interface GetTickerRequest {
  [field: string]: unknown;
  "instrument_name": string;
}

export interface GetTickersRequest {
  [field: string]: unknown;
  "currency"?: string | null;
  "expiry_date"?: number | bigint | string | null;
  "instrument_type": AssetType;
}

export interface GetTickersResponse {
  [field: string]: unknown;
  "tickers": Record<string, TickerSlimSnapshot>;
}

export interface GetTradeHistoryRequest {
  [field: string]: unknown;
  "from_timestamp"?: number | bigint | string | null;
  "instrument_name"?: string | null;
  "order_id"?: string | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "quote_id"?: string | null;
  "subaccount_id"?: number | bigint | string | null;
  "to_timestamp"?: number | bigint | string | null;
  "wallet"?: string | null;
}

export interface GetTradingviewChartDataRequest {
  [field: string]: unknown;
  "end_timestamp": number | bigint | string;
  "instrument_name": string;
  "period": number | bigint | string;
  "start_timestamp": number | bigint | string;
}

export interface GetTransactionParams {
  [field: string]: unknown;
  "op_uuid": string;
}

export interface GetTransactionResult {
  [field: string]: unknown;
  "data": string;
  "error_log"?: string | null;
  "status"?: BatchStatus | null;
  "transaction_hash"?: string | null;
}

export interface GetTriggerOrdersRequest {
  [field: string]: unknown;
  "subaccount_id": number | bigint | string;
}

export interface GetVaultActionHistoryRequest {
  [field: string]: unknown;
  "event_type"?: string | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "subaccount_id": number | bigint | string;
}

export interface GetVaultPerformanceHistoryRequest {
  [field: string]: unknown;
  "from"?: number | bigint | string | null;
  "limit"?: number | bigint | string | null;
  "resolution": PerformanceResolution;
  "subaccount_id": number | bigint | string;
  "to"?: number | bigint | string | null;
}

export interface GetVaultRequest {
  [field: string]: unknown;
  "subaccount_id": number | bigint | string;
}

export interface GetVaultRequestHistoryRequest {
  [field: string]: unknown;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "wallet": string;
}

export interface GetVaultSharesRequest {
  [field: string]: unknown;
  "wallet": string;
}

export interface GetVaultsRequest {
  [field: string]: unknown;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
}

export interface GetWalletsFromSessionKeyRequest {
  [field: string]: unknown;
  "public_session_key": string;
  "scope"?: OffchainKeyScope | null;
}

export interface GetWithdrawAddressesParams {
  [field: string]: unknown;
  "wallet": Address;
}

export interface GetWithdrawAddressesResult {
  [field: string]: unknown;
  "withdraw_addresses": Array<WithdrawAddressEntry>;
}

export interface GetWithdrawalHistoryRequest {
  [field: string]: unknown;
  "end_timestamp"?: number | bigint | string | null;
  "start_timestamp"?: number | bigint | string | null;
  "subaccount_id"?: number | bigint | string | null;
  "wallet"?: string | null;
}

export interface IncidentResponse {
  [field: string]: unknown;
  "creation_timestamp_sec": number | bigint | string;
  "label": string;
  "message": string;
  "monitor_type": MonitorType;
  "severity": Severity;
}

export interface IndexCandle {
  [field: string]: unknown;
  "close_price": string;
  "high_price": string;
  "low_price": string;
  "open_price": string;
  "price": string;
  "timestamp": number | bigint | string;
  "timestamp_bucket": number | bigint | string;
}

export interface Instrument {
  [field: string]: unknown;
  "amount_step": string;
  "base_asset_address": string;
  "base_asset_sub_id": string;
  "base_currency": string;
  "base_fee": string;
  "erc20_details"?: SpotPublicDetails | null;
  "fifo_min_allocation": string;
  "instrument_name": string;
  "instrument_type": AssetType;
  "is_active": boolean;
  "maker_fee_rate": string;
  "mark_price_fee_rate_cap"?: string | null;
  "maximum_amount": string;
  "minimum_amount": string;
  "option_details"?: OptionDetails | null;
  "perp_details"?: PerpDetails | null;
  "pro_rata_amount_step": string;
  "pro_rata_fraction": string;
  "quote_currency": string;
  "risk_universe_id": number | bigint | string;
  "scheduled_activation": number | bigint | string;
  "scheduled_deactivation": number | bigint | string;
  "taker_fee_rate": string;
  "tick_size": string;
}

export interface InterestHistoryResult {
  [field: string]: unknown;
  "events": Array<InterestPayment>;
}

export interface InterestPayment {
  [field: string]: unknown;
  "interest": string;
  "subaccount_id": number | bigint | string;
  "timestamp": number | bigint | string;
}

export interface InterestRateCandle {
  [field: string]: unknown;
  "borrow_apy": Ohlc;
  "risk_universe_id": number | bigint | string;
  "supply_apy": Ohlc;
  "timestamp": number | bigint | string;
  "total_borrow": string;
  "total_supply": string;
}

export interface InterestRateHistoryResult {
  [field: string]: unknown;
  "interest_rate_history": Array<InterestRateCandle>;
}

export interface LegUnpricedParams {
  [field: string]: unknown;
  "amount": string;
  "direction": Direction;
  "instrument_name": string;
}

export interface LendingDetails {
  [field: string]: unknown;
  "borrow_apy": string;
  "supply_apy": string;
  "total_borrow": string;
  "total_borrow_cap": string;
}

export interface LiquidationHistoryResult {
  [field: string]: unknown;
  "auctions": Array<AuctionHistory>;
  "pagination": Pagination;
}

export interface LiveAuction {
  [field: string]: unknown;
  "currency"?: string | null;
  "estimated_bid_price": string;
  "estimated_discount_pnl": string;
  "estimated_mtm": string;
  "estimated_percent_bid": string;
  "margin_type": string;
  "min_price_limit": string;
  "risk_universe_id": number | bigint | string;
  "subaccount_balances": Record<string, string>;
  "subaccount_id": number | bigint | string;
  "timestamp": number | bigint | string;
}

export interface ManagerCollateral {
  [field: string]: unknown;
  "address": string;
  "erc20": Erc20Details;
  "im_discount": string;
  "min_deposit_usd": string;
  "mm_discount": string;
  "name": string;
}

export interface MarginWatchRequest {
  [field: string]: unknown;
  "is_delayed_liquidation"?: boolean | null;
  "subaccount_id": number | bigint | string;
}

export interface MintSharesRequest {
  [field: string]: unknown;
  "deposit_hash": string;
  "nonce"?: string;
  "request_id": VaultRequestId;
  "share_price": string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
}

export interface MmpConfigResult {
  [field: string]: unknown;
  "currency": string;
  "is_frozen": boolean;
  "mmp_amount_limit": string;
  "mmp_delta_limit": string;
  "mmp_frozen_time": number | bigint | string;
  "mmp_interval": number | bigint | string;
  "mmp_unfreeze_time": number | bigint | string;
  "subaccount_id": number | bigint | string;
}

export interface MmpScopeRequest {
  [field: string]: unknown;
  "currency"?: string | null;
  "subaccount_id": number | bigint | string;
}

export interface MultipleVaultRequestsResponse {
  [field: string]: unknown;
  "requests": Array<VaultRequestResponse>;
  "total": number | bigint | string;
}

export interface OffchainAckResponse {
  [field: string]: unknown;
  "status": string;
}

export interface Ohlc {
  [field: string]: unknown;
  "close": string;
  "high": string;
  "low": string;
  "open": string;
}

export interface OnchainActionHistoryEntry {
  [field: string]: unknown;
  "acc": string;
  "action_id": number | bigint | string;
  "action_type": number | bigint | string;
  "action_type_label": string;
  "block_number": number | bigint | string;
  "data": string;
  "error_code"?: number | bigint | string | null;
  "error_data"?: string | null;
  "error_message"?: string | null;
  "fallback_at"?: number | bigint | string | null;
  "first_failed_at"?: number | bigint | string | null;
  "l1_sender": string;
  "last_failed_at"?: number | bigint | string | null;
  "op_uuid"?: string | null;
  "queue": string;
  "skip_reason"?: string | null;
  "skip_reason_code"?: string | null;
  "status": string;
  "tx_hash"?: string | null;
  "updated_at": number | bigint | string;
}

export interface OpenInterestStats {
  [field: string]: unknown;
  "current_open_interest": string;
  "interest_cap": string;
}

export interface OptionDetails {
  [field: string]: unknown;
  "expiry": number | bigint | string;
  "index": string;
  "option_type": string;
  "settlement_price"?: string | null;
  "strike": string;
}

export interface OptionPricing {
  [field: string]: unknown;
  "ai": string;
  "bi": string;
  "d": string;
  "df": string;
  "f": string;
  "g": string;
  "i": string;
  "m": string;
  "r": string;
  "t": string;
  "v": string;
}

export interface OptionSettlementHistoryResponse {
  [field: string]: unknown;
  "settlements": Array<OptionSettlementResponse>;
}

export interface OptionSettlementPricesResult {
  [field: string]: unknown;
  "expiries": Array<ExpirySettlementPrice>;
}

export interface OptionSettlementResponse {
  [field: string]: unknown;
  "amount": string;
  "expiry": number | bigint | string;
  "instrument_name": string;
  "option_settlement_pnl": string | null;
  "option_settlement_pnl_excl_fees": string | null;
  "settlement_price": string;
  "settlement_value": string;
  "subaccount_id": number | bigint | string;
}

export interface OracleSignatureDataResponse {
  [field: string]: unknown;
  "signatures": Array<string>;
  "signers": Array<Address>;
}

export interface Order {
  [field: string]: unknown;
  "algo_duration_sec"?: number | bigint | string | null;
  "algo_num_slices"?: number | bigint | string | null;
  "algo_slices_completed"?: number | bigint | string | null;
  "algo_type"?: AlgoType | null;
  "amount": string;
  "average_price": string;
  "cancel_reason"?: CancelReason | null;
  "creation_timestamp": number | bigint | string;
  "direction": Direction;
  "extra_fee": string;
  "filled_amount": string;
  "instrument_name": string;
  "is_transfer": boolean;
  "label"?: string | null;
  "last_update_timestamp": number | bigint | string;
  "limit_price": string;
  "max_fee": string;
  "mmp": boolean;
  "nonce": string;
  "order_fee": string;
  "order_id": string;
  "order_status": OrderStatus;
  "order_type": OrderType;
  "quote_id": string | null;
  "replaced_order_id": string | null;
  "signature": string;
  "signature_expiry_sec": number | bigint | string;
  "signed_limit_price": string | null;
  "signer": string;
  "subaccount_id": number | bigint | string;
  "time_in_force": TimeInForce;
  "trigger_price": string | null;
  "trigger_price_type"?: TriggerPriceType | null;
  "trigger_reject_message"?: string | null;
  "trigger_type"?: TriggerType | null;
}

export interface OrderActionDataResponse {
  [field: string]: unknown;
  "asset_address": string;
  "asset_sub_id": string;
  "desired_amount": string;
  "is_bid": boolean;
  "limit_price": string;
  "recipient_id": number | bigint | string;
  "worst_fee": string;
}

export interface OrderActionInputData {
  [field: string]: unknown;
  "data": OrderActionDataResponse;
  "expiry": number | bigint | string;
  "module": string;
  "nonce": string;
  "owner": string;
  "signer": string;
  "subaccount_id": number | bigint | string;
}

export interface OrderCreatedResponse {
  [field: string]: unknown;
  "order": Order;
  "trades": Array<Trade>;
}

export interface OrderDebugResponse {
  [field: string]: unknown;
  "action_hash": string;
  "action_typehash": string;
  "domain_separator": string;
  "encoded_data": string;
  "encoded_data_hashed": string;
  "expected_signer": string;
  "input_data": OrderActionInputData;
  "module": string;
  "owner": string;
  "recovered_signer"?: string | null;
  "typed_data_hash": string;
}

export interface OrderQuoteRequest {
  [field: string]: unknown;
  "amount": string;
  "client"?: string | null;
  "direction": Direction;
  "extra_fee"?: string | null;
  "instrument_name": string;
  "is_atomic_signing"?: boolean | null;
  "label"?: string | null;
  "limit_price": string;
  "max_fee": string;
  "mmp"?: boolean | null;
  "nonce"?: string;
  "order_type"?: OrderType | null;
  "reduce_only"?: boolean | null;
  "referral_code"?: string | null;
  "reject_post_only"?: boolean | null;
  "reject_timestamp"?: number | bigint | string | null;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: string;
  "subaccount_id": number | bigint | string;
  "time_in_force"?: TimeInForce | null;
  "trigger_price"?: string | null;
  "trigger_price_type"?: TriggerPriceType | null;
  "trigger_type"?: TriggerType | null;
}

export interface OrderQuoteResponse {
  [field: string]: unknown;
  "estimated_fee": string;
  "estimated_fill_amount": string;
  "estimated_fill_price": string;
  "estimated_order_status": OrderStatus;
  "estimated_realized_pnl": string;
  "estimated_realized_pnl_excl_fees": string;
  "invalid_reason"?: string | null;
  "is_valid": boolean;
  "max_amount"?: string | null;
  "post_initial_margin": string;
  "post_liquidation_price"?: string | null;
  "pre_initial_margin": string;
  "suggested_max_fee": string;
}

export interface PaginatedOrdersResult {
  [field: string]: unknown;
  "orders": Array<Order>;
  "pagination": Pagination;
  "subaccount_id": number | bigint | string;
}

export interface PaginatedTradesResult {
  [field: string]: unknown;
  "pagination": Pagination;
  "subaccount_id": number | bigint | string;
  "trades": Array<TradeHistoryResponse>;
}

export interface PaginatedVaultActionHistory {
  [field: string]: unknown;
  "events": Array<PublicVaultActionResponse>;
  "pagination": Pagination;
  "subaccount_id": number | bigint | string;
}

export interface PaginatedVaultRequestHistory {
  [field: string]: unknown;
  "actions": Array<VaultActionResponse>;
  "pagination": Pagination;
  "wallet": string;
}

export interface Pagination {
  [field: string]: unknown;
  "count": number | bigint | string;
  "num_pages": number | bigint | string;
}

export interface PendingDepositBridgeOrigin {
  [field: string]: unknown;
  "bridge_id": string;
  "bridge_status": string;
  "delivered_at_ms"?: number | bigint | string | null;
  "deposit_address": string;
  "expected_amount": string;
  "expires_at_ms": number | bigint | string;
  "provider_transfer_id"?: string | null;
  "rail": string;
  "receiving_amount"?: string | null;
  "receiving_token"?: string | null;
  "receiving_tx_hash"?: string | null;
  "source_address"?: string | null;
  "source_chain_id": number | bigint | string;
  "source_tx_hash": string;
  "substatus": string;
  "token": string;
}

export interface PendingDepositEntry {
  [field: string]: unknown;
  "action_id"?: number | bigint | string | null;
  "action_type": string;
  "amount"?: string | null;
  "asset": string;
  "block_number"?: number | bigint | string | null;
  "bridge_to_l1"?: PendingDepositBridgeOrigin | null;
  "deposit_type": string;
  "log_index"?: number | bigint | string | null;
  "manager_id": number | bigint | string;
  "status": string;
  "subaccount_id": number | bigint | string;
  "timestamp": number | bigint | string;
  "tx_hash"?: string | null;
  "updated_at_ms": number | bigint | string;
}

export interface PerpDetails {
  [field: string]: unknown;
  "aggregate_funding": string;
  "funding_rate": string;
  "index": string;
  "max_rate_per_hour": string;
  "min_rate_per_hour": string;
  "srm_perp_margin_requirements"?: SrmPerpMarginRequirements | null;
}

export interface PerpFeedDataResponse {
  [field: string]: unknown;
  "confidence": string;
  "currency": string;
  "deadline": number | bigint | string;
  "signatures": OracleSignatureDataResponse;
  "spot_diff_value": string;
  "timestamp": number | bigint | string;
  "type": string;
}

export interface PerpImpactTwapResult {
  [field: string]: unknown;
  "ask_impact_diff_twap": string;
  "bid_impact_diff_twap": string;
  "currency": string;
  "mid_price_diff_twap": string;
}

export interface PerpSettlementEventResponse {
  [field: string]: unknown;
  "batch_status": BatchStatus;
  "batch_uuid": string;
  "funding": string;
  "instrument_name": string;
  "pnl": string;
  "subaccount_id": number | bigint | string;
  "timestamp": number | bigint | string;
  "tx_hash"?: string | null;
}

export interface PerpSettlementHistoryResponse {
  [field: string]: unknown;
  "events": Array<PerpSettlementEventResponse>;
  "pagination": Pagination;
}

export interface PollQuotesRequest {
  [field: string]: unknown;
  "from_timestamp"?: number | bigint | string | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "quote_id"?: string | null;
  "rfq_id"?: string | null;
  "status"?: string | null;
  "subaccount_id": number | bigint | string;
  "to_timestamp"?: number | bigint | string | null;
}

export interface PollRfqsRequest {
  [field: string]: unknown;
  "from_timestamp"?: number | bigint | string | null;
  "page"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "rfq_id"?: string | null;
  "rfq_subaccount_id"?: number | bigint | string | null;
  "status"?: string | null;
  "subaccount_id": number | bigint | string;
  "to_timestamp"?: number | bigint | string | null;
}

export interface Position {
  [field: string]: unknown;
  "amount": string;
  "amount_step": string;
  "average_price": string;
  "average_price_excl_fees": string;
  "creation_timestamp": number | bigint | string;
  "cumulative_funding": string;
  "delta": string;
  "gamma": string;
  "index_price": string;
  "initial_margin": string;
  "instrument_name": string;
  "instrument_type": AssetType;
  "leverage"?: string | null;
  "liquidation_price"?: string | null;
  "maintenance_margin": string;
  "mark_price": string;
  "mark_value": string;
  "net_settlements": string;
  "open_orders_margin": string;
  "pending_funding": string;
  "realized_pnl": string;
  "realized_pnl_excl_fees": string;
  "theta": string;
  "total_fees": string;
  "unrealized_pnl": string;
  "unrealized_pnl_excl_fees": string;
  "vega": string;
}

export interface PricedLegParamsAndResponse {
  [field: string]: unknown;
  "amount": string;
  "direction": Direction;
  "instrument_name": string;
  "price": string;
}

export interface PrivateChangeSubaccountLabelResponse {
  [field: string]: unknown;
  "label": string;
  "subaccount_id": number | bigint | string;
}

export interface PrivateGetAccountResponse {
  [field: string]: unknown;
  "cancel_on_disconnect": boolean;
  "creation_timestamp_sec"?: number | bigint | string | null;
  "fallback_subaccount_id": number | bigint | string;
  "fee_info": AccountFeeInfo;
  "is_rfq_maker": boolean;
  "per_endpoint_tps": Record<string, number | bigint | string>;
  "referral_code"?: string | null;
  "subaccount_ids": Array<number | bigint | string>;
  "wallet": string;
  "websocket_matching_tps": number | bigint | string;
  "websocket_non_matching_tps": number | bigint | string;
  "websocket_option_tps": number | bigint | string;
  "websocket_perp_tps": number | bigint | string;
  "whitelisted_recipients": Array<string>;
}

export interface PrivateGetCollateralsResponse {
  [field: string]: unknown;
  "collaterals": Array<Collateral>;
  "subaccount_id": number | bigint | string;
}

export interface PrivateGetMarginRequest {
  [field: string]: unknown;
  "is_delayed_liquidation"?: boolean | null;
  "simulated_collateral_changes"?: Array<SimulatedCollateralRequest> | null;
  "simulated_position_changes"?: Array<SimulatedPositionRequest> | null;
  "subaccount_id": number | bigint | string;
}

export interface PrivateGetPositionsResponse {
  [field: string]: unknown;
  "positions": Array<Position>;
  "subaccount_id": number | bigint | string;
}

export interface PrivateGetSubaccountsResponse {
  [field: string]: unknown;
  "subaccount_ids": Array<number | bigint | string>;
  "wallet": string;
}

export interface PrivateLiquidateRequest {
  [field: string]: unknown;
  "liquidate_subaccount_id": number | bigint | string;
  "nonce"?: string;
  "percent_of_acc": string;
  "price_limit": string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: string;
  "subaccount_id": number | bigint | string;
}

export interface PrivateLiquidateResponse {
  [field: string]: unknown;
  "op_uuid": string;
  "operation_id": number | bigint | string;
}

export interface PrivateSessionKeysResponse {
  [field: string]: unknown;
  "public_session_keys": Array<SessionKey>;
}

export interface PrivateSetSessionKeyResponse {
  [field: string]: unknown;
  "expiry_sec": number | bigint | string;
  "ip_whitelist": Array<string>;
  "label"?: string | null;
  "offchain_scopes": Array<string>;
  "protocol_scopes": Array<string>;
  "public_session_key": string;
  "subaccount_ids": Array<number | bigint | string>;
}

export interface PrivateTransferSpotExternalRequest {
  [field: string]: unknown;
  "amount": string;
  "asset_name": string;
  "max_fee_usd": string;
  "new_subaccount_manager": number | bigint | string;
  "nonce"?: string;
  "recipient_address": string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: string;
  "sub_id": number | bigint | string;
  "subaccount_id": number | bigint | string;
  "to_subaccount_id": number | bigint | string;
}

export interface PrivateTransferSpotExternalResponse {
  [field: string]: unknown;
  "op_uuid": string;
  "operation_id": number | bigint | string;
}

export interface PrivateTransferSpotRequest {
  [field: string]: unknown;
  "amount": string;
  "asset_name": string;
  "max_fee_usd": string;
  "new_subaccount_manager": number | bigint | string;
  "nonce"?: string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: string;
  "sub_id": number | bigint | string;
  "subaccount_id": number | bigint | string;
  "to_subaccount_id": number | bigint | string;
}

export interface PrivateTransferSpotResponse {
  [field: string]: unknown;
  "op_uuid": string;
  "operation_id": number | bigint | string;
}

export interface PrivateWithdrawRequest {
  [field: string]: unknown;
  "amount_in_underlying": string;
  "asset_name": string;
  "force_batch": boolean;
  "max_fee_usd": string;
  "nonce"?: string;
  "recipient"?: string | null;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: string;
  "subaccount_id": number | bigint | string;
}

export interface PrivateWithdrawResponse {
  [field: string]: unknown;
  "op_uuid": string;
  "operation_id": number | bigint | string;
}

export interface ProgramResponse {
  [field: string]: unknown;
  "asset_types": Array<string>;
  "currencies": Array<string>;
  "end_timestamp": number | bigint | string;
  "min_notional": string;
  "name": string;
  "rewards": Record<string, string>;
  "start_timestamp": number | bigint | string;
}

export interface ProtocolVault {
  [field: string]: unknown;
  "closed": boolean;
  "config": VaultConfig;
  "global_hwm": string;
  "last_fee_settled_at_sec": number | bigint | string;
  "protocol_fee_share_bps": number | bigint | string;
  "subaccount_id": number | bigint | string;
  "total_shares": string;
}

export interface PublicExecuteQuoteDebugRequest {
  [field: string]: unknown;
  "direction": Direction;
  "legs": Array<PricedLegParamsAndResponse>;
  "max_fee": string;
  "nonce"?: string;
  "quote_id": string;
  "rfq_id": string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
}

export interface PublicGetLiveAuctionsResponse {
  [field: string]: unknown;
  "auctions": Array<LiveAuction>;
}

export interface PublicGetMarginRequest {
  [field: string]: unknown;
  "margin_type": string;
  "market"?: string | null;
  "simulated_collateral_changes"?: Array<SimulatedCollateralRequest> | null;
  "simulated_collaterals": Array<SimulatedCollateralRequest>;
  "simulated_position_changes"?: Array<SimulatedPositionRequest> | null;
  "simulated_positions": Array<SimulatedPositionRequest>;
}

export interface PublicGetSubaccountsRequest {
  [field: string]: unknown;
  "wallet": Address;
}

export interface PublicGetWalletRequest {
  [field: string]: unknown;
  "subaccount_id": number | bigint | string;
}

export interface PublicGetWalletResponse {
  [field: string]: unknown;
  "subaccount_id": number | bigint | string;
  "wallet": string;
}

export interface PublicGetWalletsFromSessionKeyResponse {
  [field: string]: unknown;
  "wallets": Array<string>;
}

export interface PublicMarginWatchResponse {
  [field: string]: unknown;
  "collaterals": Array<Collateral>;
  "currency": Array<string>;
  "has_delayed_liquidation": boolean;
  "initial_margin": string;
  "is_delayed_liquidation": boolean;
  "maintenance_margin": string;
  "margin_type": string;
  "positions": Array<Position>;
  "regular_margin_flagged_sec"?: number | bigint | string | null;
  "risk_universe_id": number | bigint | string;
  "subaccount_id": number | bigint | string;
  "subaccount_value": string;
  "valuation_timestamp": number | bigint | string;
}

export interface PublicQuote {
  [field: string]: unknown;
  "cancel_reason": RFQCancelReason;
  "creation_timestamp": number | bigint | string;
  "direction": Direction;
  "fill_pct": string;
  "last_update_timestamp": number | bigint | string;
  "legs": Array<PricedLegParamsAndResponse>;
  "legs_hash": string;
  "liquidity_role": LiquidityRole;
  "quote_id": string;
  "rfq_id": string;
  "status": RFQStatus;
  "subaccount_id": number | bigint | string;
  "wallet": Address;
}

export interface PublicRfq {
  [field: string]: unknown;
  "cancel_reason": RFQCancelReason;
  "creation_timestamp": number | bigint | string;
  "fill_rate": string | null;
  "filled_direction"?: Direction | null;
  "filled_pct": string;
  "last_update_timestamp": number | bigint | string;
  "legs": Array<LegUnpricedParams>;
  "partial_fill_step": string;
  "recent_fill_rate": string | null;
  "rfq_id": string;
  "status": RFQStatus;
  "subaccount_id": number | bigint | string;
  "total_cost": string | null;
  "valid_until": number | bigint | string;
  "wallet": Address;
}

export interface PublicSendQuoteDebugRequest {
  [field: string]: unknown;
  "direction": Direction;
  "legs": Array<PricedLegParamsAndResponse>;
  "max_fee": string;
  "nonce"?: string;
  "rfq_id": string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
}

export interface PublicTradesResult {
  [field: string]: unknown;
  "pagination": Pagination;
  "trades": Array<SettledTrade>;
}

export interface PublicVaultActionResponse {
  [field: string]: unknown;
  "curator_shares_minted": string;
  "event_ts": number | bigint | string;
  "event_type": string;
  "holder": string;
  "management_shares_minted": string;
  "nav": string;
  "new_high_water_mark": string;
  "old_high_water_mark": string;
  "operation_uuid": string;
  "performance_shares_minted": string;
  "protocol_shares_minted": string;
  "share_price": string;
  "shares_delta": string;
  "status": string;
  "subaccount_id": number | bigint | string;
  "total_shares": string;
}

export interface PublicWithdrawDebugRequest {
  [field: string]: unknown;
  "amount_in_underlying": string;
  "asset_name": string;
  "force_batch": boolean;
  "max_fee_usd": string;
  "nonce"?: string;
  "recipient"?: string | null;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: string;
  "subaccount_id": number | bigint | string;
}

export interface Quote {
  [field: string]: unknown;
  "batch_status"?: BatchStatus | null;
  "cancel_reason": RFQCancelReason;
  "creation_timestamp": number | bigint | string;
  "direction": Direction;
  "extra_fee": string;
  "fee": string;
  "fill_pct": string;
  "is_transfer": boolean;
  "label": string;
  "last_update_timestamp": number | bigint | string;
  "legs": Array<PricedLegParamsAndResponse>;
  "legs_hash": string;
  "liquidity_role": LiquidityRole;
  "max_fee": string;
  "mmp": boolean;
  "nonce": string;
  "quote_id": string;
  "rfq_id": string;
  "signature_expiry_sec": number | bigint | string;
  "status": RFQStatus;
  "subaccount_id": number | bigint | string;
  "tx_hash"?: string | null;
}

export interface QuoteExecuteDebugResult {
  [field: string]: unknown;
  "action_hash": string;
  "encoded_data": string;
  "encoded_data_hashed": string;
  "encoded_legs": string;
  "legs_hash": string;
  "typed_data_hash": string;
}

export interface QuoteExecuteResponse {
  [field: string]: unknown;
  "cancel_reason": RFQCancelReason;
  "creation_timestamp": number | bigint | string;
  "direction": Direction;
  "extra_fee": string;
  "fee": string;
  "fill_pct": string;
  "is_transfer": boolean;
  "label": string;
  "last_update_timestamp": number | bigint | string;
  "legs": Array<PricedLegParamsAndResponse>;
  "legs_hash": string;
  "liquidity_role": LiquidityRole;
  "max_fee": string;
  "mmp": boolean;
  "nonce": string;
  "quote_id": string;
  "rfq_filled_pct": string;
  "rfq_id": string;
  "signature_expiry_sec": number | bigint | string;
  "status": RFQStatus;
  "subaccount_id": number | bigint | string;
}

export interface QuoteGetResponse {
  [field: string]: unknown;
  "pagination": Pagination;
  "quotes": Array<Quote>;
}

export interface QuotePollResponse {
  [field: string]: unknown;
  "pagination": Pagination;
  "quotes": Array<PublicQuote>;
}

export interface QuoteReplaceResponse {
  [field: string]: unknown;
  "cancelled_quote": Quote;
  "create_quote_error"?: RPCError | null;
  "quote"?: Quote | null;
}

export interface QuoteSendDebugResult {
  [field: string]: unknown;
  "action_hash": string;
  "encoded_data": string;
  "encoded_data_hashed": string;
  "typed_data_hash": string;
}

export interface RFQGetResponse {
  [field: string]: unknown;
  "pagination": Pagination;
  "rfqs": Array<Rfq>;
}

export interface RFQPollResponse {
  [field: string]: unknown;
  "pagination": Pagination;
  "rfqs": Array<PublicRfq>;
}

export interface RPCError {
  [field: string]: unknown;
  "code": number | bigint | string;
  "data"?: string | null;
  "message": string;
}

export interface RateFeedDataResponse {
  [field: string]: unknown;
  "confidence": string;
  "currency": string;
  "deadline": number | bigint | string;
  "expiry": number | bigint | string;
  "rate": string;
  "signatures": OracleSignatureDataResponse;
  "timestamp": number | bigint | string;
}

export interface RateLimitInfo {
  [field: string]: unknown;
  "consumedPoints": number | bigint | string;
  "isFirstInDuration": boolean;
  "msBeforeNext": number | bigint | string;
  "remainingPoints": number | bigint | string;
}

export interface RateLimitResult {
  [field: string]: unknown;
  "remaining_connections"?: RateLimitInfo | null;
  "remaining_matching": RateLimitInfo;
  "remaining_non_matching": RateLimitInfo;
  "remaining_per_endpoint": Record<string, RateLimitInfo>;
}

export interface ReferralPerformanceByInstrumentType {
  [field: string]: unknown;
  "builder_fee": string;
  "fee_reward": string;
  "notional_volume": string;
  "referred_fee": string;
  "unique_traders_referred": number | bigint | string;
}

export interface Referrer {
  [field: string]: unknown;
  "receiving_wallet"?: string | null;
  "referral_code": string;
  "wallet": string;
}

export interface RegisterBridgeDepositParams {
  [field: string]: unknown;
  "bridge_token": string;
  "bridge_token_decimals": number | bigint | string;
  "deposit_address": string;
  "expected_amount": string;
  "provider_transfer_id"?: string | null;
  "rail": string;
  "source_chain_id": number | bigint | string;
  "source_token": string;
  "source_tx_hash": string;
  "tool"?: string | null;
  "wallet": string;
}

export interface RegisterDepositAddressParams {
  [field: string]: unknown;
  "manager_id"?: number | bigint | string | null;
  "subaccount_id"?: number | bigint | string | null;
  "wallet": string;
}

export interface RegisterDepositAddressResult {
  [field: string]: unknown;
  "deposit_address": string;
  "manager_id"?: number | bigint | string | null;
  "subaccount_id"?: number | bigint | string | null;
  "wallet": string;
}

export interface RegisterWithdrawAddressParams {
  [field: string]: unknown;
  "chain_id": number | bigint | string;
  "recipient": string;
  "wallet": string;
}

export interface RegisterWithdrawAddressResult {
  [field: string]: unknown;
  "chain_id": number | bigint | string;
  "recipient": string;
  "recipient_display": string;
  "routes": Array<ForwardedToken>;
  "withdraw_address": string;
}

export interface RejectDepositRequestRequest {
  [field: string]: unknown;
  "reason"?: string | null;
  "request_id": VaultRequestId;
  "subaccount_id": number | bigint | string;
}

export interface ReplaceOrderRequest {
  [field: string]: unknown;
  "algo_duration_sec"?: number | bigint | string | null;
  "algo_num_slices"?: number | bigint | string | null;
  "algo_type"?: AlgoType | null;
  "amount": string;
  "client"?: string | null;
  "direction": Direction;
  "expected_filled_amount"?: string | null;
  "extra_fee"?: string | null;
  "instrument_name": string;
  "is_atomic_signing"?: boolean | null;
  "label"?: string | null;
  "limit_price": string;
  "max_fee": string;
  "mmp"?: boolean | null;
  "nonce"?: string;
  "nonce_to_cancel"?: string | null;
  "order_id_to_cancel"?: string | null;
  "order_type"?: OrderType | null;
  "reduce_only"?: boolean | null;
  "referral_code"?: string | null;
  "reject_post_only"?: boolean | null;
  "reject_timestamp"?: number | bigint | string | null;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: string;
  "subaccount_id": number | bigint | string;
  "time_in_force"?: TimeInForce | null;
  "trigger_price"?: string | null;
  "trigger_price_type"?: TriggerPriceType | null;
  "trigger_type"?: TriggerType | null;
}

export interface ReplaceOrderResponse {
  [field: string]: unknown;
  "cancelled_order": Order;
  "create_order_error"?: RPCError | null;
  "order"?: Order | null;
  "trades"?: Array<Trade> | null;
}

export interface ReplaceQuoteRequest {
  [field: string]: unknown;
  "client"?: string | null;
  "direction": Direction;
  "extra_fee"?: string | null;
  "label"?: string | null;
  "legs": Array<PricedLegParamsAndResponse>;
  "max_fee": string;
  "mmp"?: boolean | null;
  "nonce"?: string;
  "nonce_to_cancel"?: string | null;
  "quote_id_to_cancel"?: string | null;
  "referral_code"?: string | null;
  "rfq_id": string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
}

export interface RequestVaultDepositRequest {
  [field: string]: unknown;
  "amount": string;
  "deposit_spot_asset": Address;
  "nonce"?: string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
  "vault_subaccount_id": number | bigint | string;
}

export interface RequestVaultWithdrawRequest {
  [field: string]: unknown;
  "nonce"?: string;
  "shares_to_burn": string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
  "vault_subaccount_id": number | bigint | string;
}

export interface Rfq {
  [field: string]: unknown;
  "ask_total_cost": string | null;
  "bid_total_cost": string | null;
  "cancel_reason": RFQCancelReason;
  "counterparties"?: Array<Address> | null;
  "creation_timestamp": number | bigint | string;
  "filled_direction"?: Direction | null;
  "filled_pct": string;
  "label": string;
  "last_update_timestamp": number | bigint | string;
  "legs": Array<LegUnpricedParams>;
  "mark_total_cost": string | null;
  "max_total_cost": string | null;
  "min_total_cost": string | null;
  "partial_fill_step": string;
  "rfq_id": string;
  "status": RFQStatus;
  "subaccount_id": number | bigint | string;
  "total_cost": string | null;
  "valid_until": number | bigint | string;
  "wallet": Address;
}

export interface RfqGetBestQuoteRequest {
  [field: string]: unknown;
  "client"?: string | null;
  "direction"?: Direction | null;
  "extra_fee"?: string | null;
  "legs"?: Array<LegUnpricedParams> | null;
  "rfq_id"?: string | null;
  "subaccount_id": number | bigint | string;
}

export interface RfqGetBestQuoteResponse {
  [field: string]: unknown;
  "best_quote"?: PublicQuote | null;
  "direction": Direction;
  "down_liquidation_price": string | null;
  "estimated_fee": string;
  "estimated_realized_pnl": string;
  "estimated_realized_pnl_excl_fees": string;
  "estimated_total_cost": string;
  "filled_pct": string;
  "invalid_reason"?: string | null;
  "is_valid": boolean;
  "orderbook_total_cost": string | null;
  "post_initial_margin": string;
  "post_liquidation_price": string | null;
  "pre_initial_margin": string;
  "suggested_max_fee": string;
  "up_liquidation_price": string | null;
}

export interface RfqLegResponse {
  [field: string]: unknown;
  "amount": string;
  "asset_address": string;
  "asset_sub_id": string;
  "price": string;
}

export interface RfqMakerActionDataResponse {
  [field: string]: unknown;
  "max_fee": string;
  "trades": Array<RfqLegResponse>;
}

export interface RfqTakerActionDataResponse {
  [field: string]: unknown;
  "max_fee": string;
  "order_hash": string;
}

export interface RiskUniverse {
  [field: string]: unknown;
  "description"?: string | null;
  "managers": Array<RiskUniverseManager>;
  "name"?: string | null;
  "risk_universe_id": number | bigint | string;
  "security_module": SecurityModuleDetails;
}

export interface RiskUniverseManager {
  [field: string]: unknown;
  "collaterals": Array<ManagerCollateral>;
  "instruments": Array<string>;
  "manager_id": number | bigint | string;
  "margin_type": MarginType;
}

export interface ScoreBreakdown {
  [field: string]: unknown;
  "coverage_score": string;
  "holder_boost": string;
  "quality_score": string;
  "total_score": string;
  "volume": string;
  "volume_multiplier": string;
  "wallet": string;
}

export interface SecurityModuleDetails {
  [field: string]: unknown;
  "cash_asset": string;
  "cash_currency": string;
  "subaccount_id": number | bigint | string;
}

export interface SendQuoteRequest {
  [field: string]: unknown;
  "client"?: string | null;
  "direction": Direction;
  "extra_fee"?: string | null;
  "label"?: string | null;
  "legs": Array<PricedLegParamsAndResponse>;
  "max_fee": string;
  "mmp"?: boolean | null;
  "nonce"?: string;
  "referral_code"?: string | null;
  "rfq_id": string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
}

export interface SendRfqRequest {
  [field: string]: unknown;
  "client"?: string | null;
  "counterparties"?: Array<string> | null;
  "extra_fee"?: string | null;
  "label"?: string | null;
  "legs": Array<LegUnpricedParams>;
  "max_total_cost"?: string | null;
  "min_total_cost"?: string | null;
  "partial_fill_step"?: string | null;
  "referral_code"?: string | null;
  "subaccount_id": number | bigint | string;
}

export interface SessionKey {
  [field: string]: unknown;
  "expiry_sec": number | bigint | string;
  "ip_whitelist": Array<string>;
  "label": string;
  "offchain_scopes": Array<string>;
  "protocol_scopes": Array<string>;
  "public_session_key": string;
  "registered_sec": number | bigint | string;
  "subaccount_ids": Array<number | bigint | string>;
}

export interface SessionKeysRequest {
  [field: string]: unknown;
  "wallet": string;
}

export interface SetMmpConfigRequest {
  [field: string]: unknown;
  "currency": string;
  "mmp_amount_limit"?: string | null;
  "mmp_delta_limit"?: string | null;
  "mmp_frozen_time": number | bigint | string;
  "mmp_interval": number | bigint | string;
  "subaccount_id": number | bigint | string;
}

export interface SetMmpConfigResponse {
  [field: string]: unknown;
  "currency": string;
  "mmp_amount_limit": string;
  "mmp_delta_limit": string;
  "mmp_frozen_time": number | bigint | string;
  "mmp_interval": number | bigint | string;
  "subaccount_id": number | bigint | string;
}

export interface SetSessionKeyActionDataResponse {
  [field: string]: unknown;
  "expiry_sec": number | bigint | string;
  "scopes": Array<string>;
  "session_key": string;
  "subaccounts": Array<number | bigint | string>;
}

export interface SetSessionKeyActionInputData {
  [field: string]: unknown;
  "data": SetSessionKeyActionDataResponse;
  "expiry": number | bigint | string;
  "module": string;
  "nonce": string;
  "owner": string;
  "signer": string;
  "subaccount_id": number | bigint | string;
}

export interface SetSessionKeyDebugResponse {
  [field: string]: unknown;
  "action_hash": string;
  "action_typehash": string;
  "domain_separator": string;
  "encoded_data": string;
  "encoded_data_hashed": string;
  "expected_signer": string;
  "input_data": SetSessionKeyActionInputData;
  "module": string;
  "owner": string;
  "recovered_signer"?: string | null;
  "typed_data_hash": string;
}

export interface SetSessionKeyRequest {
  [field: string]: unknown;
  "expiry_sec": number | bigint | string;
  "ip_whitelist"?: Array<string> | null;
  "label"?: string | null;
  "nonce"?: string;
  "offchain_scopes": Array<string>;
  "protocol_scopes": Array<string>;
  "public_session_key": string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: string;
  "subaccount_ids"?: Array<number | bigint | string> | null;
  "wallet": string;
}

export interface SettledTrade {
  [field: string]: unknown;
  "batch_status"?: BatchStatus | null;
  "direction": Direction;
  "expected_rebate": string;
  "extra_fee": string;
  "index_price": string;
  "instrument_name": string;
  "liquidity_role": LiquidityRole;
  "mark_price": string;
  "quote_id": string | null;
  "realized_pnl": string;
  "realized_pnl_excl_fees": string;
  "rfq_id": string | null;
  "subaccount_id": number | bigint | string;
  "timestamp": number | bigint | string;
  "trade_amount": string;
  "trade_fee": string;
  "trade_id": string;
  "trade_price": string;
  "tx_hash": string;
  "wallet": string;
}

export interface SignedAction {
  [field: string]: unknown;
  "action": Action;
  "signature": Array<number | bigint | string>;
}

export interface SignedTransferQuoteRequest {
  [field: string]: unknown;
  "direction": Direction;
  "legs": Array<PricedLegParamsAndResponse>;
  "max_fee": string;
  "nonce"?: string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: Address;
  "subaccount_id": number | bigint | string;
}

export interface SimulatedCollateralRequest {
  [field: string]: unknown;
  "amount": string;
  "asset_name": string;
}

export interface SimulatedPositionRequest {
  [field: string]: unknown;
  "amount": string;
  "entry_price"?: string | null;
  "instrument_name": string;
}

export interface SpotAssetEntry {
  [field: string]: unknown;
  "address": string;
  "erc20": Erc20Details;
  "min_deposit_usd": string;
  "name": string;
  "universes": Array<SpotUniverse>;
}

export interface SpotFeedDataResponse {
  [field: string]: unknown;
  "confidence": string;
  "currency": string;
  "deadline": number | bigint | string;
  "feed_source_type"?: string | null;
  "price": string;
  "signatures": OracleSignatureDataResponse;
  "timestamp": number | bigint | string;
}

export interface SpotPublicDetails {
  [field: string]: unknown;
  "borrow_index": string;
  "decimals": number | bigint | string;
  "supply_index": string;
  "underlying_erc20_address": string;
}

export interface SpotUniverse {
  [field: string]: unknown;
  "lending"?: LendingDetails | null;
  "oi": OpenInterestStats;
  "pm2_im_discount": string;
  "pm2_mm_discount": string;
  "risk_universe_id": number | bigint | string;
  "srm_im_discount": string;
  "srm_mm_discount": string;
}

export interface SrmPerpMarginRequirements {
  [field: string]: unknown;
  "im_perp_req": string;
  "max_leverage": string;
  "mm_perp_req": string;
}

export interface Subaccount {
  [field: string]: unknown;
  "collaterals": Array<Collateral>;
  "collaterals_initial_margin": string;
  "collaterals_maintenance_margin": string;
  "collaterals_value": string;
  "currency": Array<string>;
  "failed_to_fetch": boolean;
  "initial_margin": string;
  "is_under_liquidation": boolean;
  "label": string;
  "maintenance_margin": string;
  "manager_id": number | bigint | string;
  "margin_type": string;
  "mm_credits": string;
  "open_orders": Array<Order>;
  "open_orders_margin": string;
  "positions": Array<Position>;
  "positions_initial_margin": string;
  "positions_maintenance_margin": string;
  "positions_value": string;
  "projected_margin_change": string;
  "risk_universe_id": number | bigint | string;
  "subaccount_id": number | bigint | string;
  "subaccount_value": string;
  "vault_deposit_holds": Array<VaultDepositHold>;
}

export interface SubaccountValueEntry {
  [field: string]: unknown;
  "currency": string;
  "initial_margin": string;
  "maintenance_margin": string;
  "margin_type": string;
  "subaccount_id": number | bigint | string;
  "subaccount_value": string;
  "timestamp": number | bigint | string;
}

export interface SubaccountValueHistoryResult {
  [field: string]: unknown;
  "pagination": Pagination;
  "subaccount_value_history": Array<SubaccountValueEntry>;
}

export interface TickerSlimSnapshot {
  [field: string]: unknown;
  "A": string;
  "B": string;
  "I": string;
  "M": string;
  "a": string;
  "b": string;
  "f"?: string | null;
  "maxp": string;
  "minp": string;
  "option_pricing"?: OptionPricing | null;
  "stats": DailyTradingStatistics;
  "t": number | bigint | string;
}

export interface Trade {
  [field: string]: unknown;
  "batch_status"?: BatchStatus | null;
  "direction": Direction;
  "expected_rebate": string;
  "extra_fee": string;
  "index_price": string;
  "instrument_name": string;
  "is_transfer": boolean;
  "label"?: string | null;
  "liquidity_role": LiquidityRole;
  "mark_price": string;
  "op_uuid": string;
  "order_id": string;
  "quote_id": string | null;
  "realized_pnl": string;
  "realized_pnl_excl_fees": string;
  "rfq_id": string | null;
  "subaccount_id": number | bigint | string;
  "timestamp": number | bigint | string;
  "trade_amount": string;
  "trade_fee": string;
  "trade_id": string;
  "trade_price": string;
  "tx_hash"?: string | null;
}

export interface TradeHistoryResponse {
  [field: string]: unknown;
  "batch_status"?: BatchStatus | null;
  "direction": Direction;
  "expected_rebate": string;
  "extra_fee": string;
  "index_price": string;
  "instrument_name": string;
  "is_transfer": boolean;
  "label": string;
  "liquidity_role": LiquidityRole;
  "mark_price": string;
  "op_uuid": string;
  "order_id": string;
  "quote_id": string | null;
  "realized_pnl": string;
  "realized_pnl_excl_fees": string;
  "rfq_id": string | null;
  "subaccount_id": number | bigint | string;
  "timestamp": number | bigint | string;
  "trade_amount": string;
  "trade_fee": string;
  "trade_id": string;
  "trade_price": string;
  "tx_hash"?: string | null;
}

export interface TradingviewCandle {
  [field: string]: unknown;
  "close_price": string;
  "high_price": string;
  "low_price": string;
  "open_price": string;
  "timestamp": number | bigint | string;
  "timestamp_bucket": number | bigint | string;
  "volume_contracts": string;
  "volume_usd": string;
}

export interface TransferActionDataResponse {
  [field: string]: unknown;
  "amount": string;
  "asset": string;
  "max_fee_usd": string;
  "new_subaccount_manager": number | bigint | string;
  "sub_id": string;
  "to_subaccount_id": number | bigint | string;
}

export interface TransferEntry {
  [field: string]: unknown;
  "amount": string;
  "asset": string;
  "batch_status": BatchStatus;
  "batch_uuid": string;
  "fee": string;
  "from_subaccount_id": number | bigint | string;
  "from_wallet": string;
  "is_outgoing": boolean;
  "operation_id": string;
  "timestamp": number | bigint | string;
  "to_subaccount_id": number | bigint | string;
  "to_wallet": string;
  "tx_hash"?: string | null;
}

export interface TransferHistoryResult {
  [field: string]: unknown;
  "transfers": Array<TransferEntry>;
}

export interface TransferPositionsDebugResponse {
  [field: string]: unknown;
  "maker_result": TransferPositionsMakerDebugResponse;
  "taker_result": TransferPositionsTakerDebugResponse;
}

export interface TransferPositionsMakerActionInputData {
  [field: string]: unknown;
  "data": RfqMakerActionDataResponse;
  "expiry": number | bigint | string;
  "module": string;
  "nonce": string;
  "owner": string;
  "signer": string;
  "subaccount_id": number | bigint | string;
}

export interface TransferPositionsMakerDebugResponse {
  [field: string]: unknown;
  "action_hash": string;
  "action_typehash": string;
  "domain_separator": string;
  "encoded_data": string;
  "encoded_data_hashed": string;
  "expected_signer": string;
  "input_data": TransferPositionsMakerActionInputData;
  "module": string;
  "owner": string;
  "recovered_signer"?: string | null;
  "typed_data_hash": string;
}

export interface TransferPositionsRequest {
  [field: string]: unknown;
  "maker_params": SignedTransferQuoteRequest;
  "taker_params": SignedTransferQuoteRequest;
  "wallet": Address;
}

export interface TransferPositionsResponse {
  [field: string]: unknown;
  "maker_quote": Quote;
  "taker_quote": Quote;
}

export interface TransferPositionsTakerActionInputData {
  [field: string]: unknown;
  "data": RfqTakerActionDataResponse;
  "expiry": number | bigint | string;
  "module": string;
  "nonce": string;
  "owner": string;
  "signer": string;
  "subaccount_id": number | bigint | string;
}

export interface TransferPositionsTakerDebugResponse {
  [field: string]: unknown;
  "action_hash": string;
  "action_typehash": string;
  "domain_separator": string;
  "encoded_data": string;
  "encoded_data_hashed": string;
  "expected_signer": string;
  "input_data": TransferPositionsTakerActionInputData;
  "module": string;
  "owner": string;
  "recovered_signer"?: string | null;
  "typed_data_hash": string;
}

export interface TransferSpotActionInputData {
  [field: string]: unknown;
  "data": TransferActionDataResponse;
  "expiry": number | bigint | string;
  "module": string;
  "nonce": string;
  "owner": string;
  "signer": string;
  "subaccount_id": number | bigint | string;
}

export interface TransferSpotDebugResponse {
  [field: string]: unknown;
  "action_hash": string;
  "action_typehash": string;
  "domain_separator": string;
  "encoded_data": string;
  "encoded_data_hashed": string;
  "expected_signer": string;
  "input_data": TransferSpotActionInputData;
  "module": string;
  "owner": string;
  "recovered_signer"?: string | null;
  "typed_data_hash": string;
}

export interface TransferSpotExternalActionInputData {
  [field: string]: unknown;
  "data": ExternalTransferActionDataResponse;
  "expiry": number | bigint | string;
  "module": string;
  "nonce": string;
  "owner": string;
  "signer": string;
  "subaccount_id": number | bigint | string;
}

export interface TransferSpotExternalDebugResponse {
  [field: string]: unknown;
  "action_hash": string;
  "action_typehash": string;
  "domain_separator": string;
  "encoded_data": string;
  "encoded_data_hashed": string;
  "expected_signer": string;
  "input_data": TransferSpotExternalActionInputData;
  "module": string;
  "owner": string;
  "recovered_signer"?: string | null;
  "typed_data_hash": string;
}

export interface UniverseManagers {
  [field: string]: unknown;
  "pm"?: number | bigint | string | null;
  "risk_universe_id": number | bigint | string;
  "risk_universe_name"?: string | null;
  "sm"?: number | bigint | string | null;
}

export interface UpdateVaultInfoRequest {
  [field: string]: unknown;
  "description"?: string | null;
  "mtm_cap"?: string | null;
  "name"?: string | null;
  "subaccount_id": number | bigint | string;
  "whitelist_only"?: boolean | null;
}

export interface UpdateWhitelistedRecipientsActionDataResponse {
  [field: string]: unknown;
  "add": Array<string>;
  "remove": Array<string>;
}

export interface UpdateWhitelistedRecipientsActionInputData {
  [field: string]: unknown;
  "data": UpdateWhitelistedRecipientsActionDataResponse;
  "expiry": number | bigint | string;
  "module": string;
  "nonce": string;
  "owner": string;
  "signer": string;
  "subaccount_id": number | bigint | string;
}

export interface UpdateWhitelistedRecipientsDebugResponse {
  [field: string]: unknown;
  "action_hash": string;
  "action_typehash": string;
  "domain_separator": string;
  "encoded_data": string;
  "encoded_data_hashed": string;
  "expected_signer": string;
  "input_data": UpdateWhitelistedRecipientsActionInputData;
  "module": string;
  "owner": string;
  "recovered_signer"?: string | null;
  "typed_data_hash": string;
}

export interface UpdateWhitelistedRecipientsRequest {
  [field: string]: unknown;
  "add": Array<string>;
  "nonce"?: string;
  "remove": Array<string>;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: string;
  "wallet": string;
}

export interface UpdateWhitelistedRecipientsResponse {
  [field: string]: unknown;
  "op_uuid": string;
  "operation_id": number | bigint | string;
  "whitelisted_recipients": Array<string>;
}

export interface Vault {
  [field: string]: unknown;
  "benchmark_price"?: string | null;
  "curator": Address;
  "curator_shares": string;
  "description": string;
  "mtm_cap"?: string | null;
  "name": string;
  "nav_benchmark"?: string | null;
  "nav_usd"?: string | null;
  "protocol": ProtocolVault;
  "simulated_share_price_usd"?: string | null;
  "whitelist_only": boolean;
}

export interface VaultActionResponse {
  [field: string]: unknown;
  "after_shares": string;
  "amount": string;
  "before_shares": string;
  "creation_timestamp_ms": number | bigint | string;
  "entry_price": string;
  "error_reason": string;
  "event_ts": number | bigint | string;
  "event_type": string;
  "exit_price": string;
  "operation_id": number | bigint | string;
  "operation_uuid": string;
  "share_price": string;
  "shares_delta": string;
  "shares_requested": string;
  "status": string;
  "user_action_hash": string;
  "vault_nonce": string;
  "vault_subaccount_id": number | bigint | string;
  "wallet": string;
}

export interface VaultCancelResponse {
  [field: string]: unknown;
  "cancelled_request_ids": Array<VaultRequestId>;
  "op_uuid": string;
  "operation_id": number | bigint | string;
}

export interface VaultConfig {
  [field: string]: unknown;
  "benchmark_asset"?: Address | null;
  "cooldown_sec": number | bigint | string;
  "deposit_spot_asset": Address;
  "management_fee_bps": number | bigint | string;
  "max_slippage_bps": number | bigint | string;
  "performance_fee_bps": number | bigint | string;
}

export interface VaultCreateResponse {
  [field: string]: unknown;
  "op_uuid": string;
  "operation_id": number | bigint | string;
}

export interface VaultDepositHold {
  [field: string]: unknown;
  "amount": string;
  "asset_name": string;
  "currency": string;
  "vault_id": number | bigint | string;
}

export interface VaultForceBurnResponse {
  [field: string]: unknown;
  "op_uuid": string;
  "operation_id": number | bigint | string;
}

export interface VaultIdsResponse {
  [field: string]: unknown;
  "subaccount_ids": Array<number | bigint | string>;
}

export interface VaultPerformanceHistoryResult {
  [field: string]: unknown;
  "points": Array<VaultPerformancePointResponse>;
  "resolution": PerformanceResolution;
  "subaccount_id": number | bigint | string;
}

export interface VaultPerformancePointResponse {
  [field: string]: unknown;
  "benchmark_price": string | null;
  "curator_shares": string;
  "global_hwm": string;
  "nav": string | null;
  "nav_benchmark": string | null;
  "share_price": string;
  "total_shares": string;
  "ts": number | bigint | string;
}

export interface VaultRequestAckResponse {
  [field: string]: unknown;
  "request_id": VaultRequestId;
}

export interface VaultRequestId {
  [field: string]: unknown;
  "vault_nonce": string;
  "vault_subaccount_id": number | bigint | string;
  "wallet": Address;
}

export interface VaultRequestResponse {
  [field: string]: unknown;
  "creation_timestamp_ms": number | bigint | string;
  "id": VaultRequestId;
  "signed_action": SignedAction;
  "subaccount_id": number | bigint | string;
  "user_action_hash": string;
  "wallet": Address;
}

export interface VaultSettleResponse {
  [field: string]: unknown;
  "op_uuid": string;
  "operation_id": number | bigint | string;
}

export interface VaultShareEntryResponse {
  [field: string]: unknown;
  "shares": string;
  "vault": Vault;
}

export interface VaultSharesResponse {
  [field: string]: unknown;
  "vaults": Array<VaultShareEntryResponse>;
}

export interface VaultsResponse {
  [field: string]: unknown;
  "pagination": Pagination;
  "vaults": Array<Vault>;
}

export interface VolFeedDataResponse {
  [field: string]: unknown;
  "confidence": string;
  "currency": string;
  "deadline": number | bigint | string;
  "expiry": number | bigint | string;
  "signatures": OracleSignatureDataResponse;
  "timestamp": number | bigint | string;
  "vol_data": VolSVIParamDataResponse;
}

export interface VolSVIParamDataResponse {
  [field: string]: unknown;
  "SVI_a": string;
  "SVI_b": string;
  "SVI_fwd": string;
  "SVI_m": string;
  "SVI_refTau": string;
  "SVI_rho": string;
  "SVI_sigma": string;
}

export interface WithdrawAddressEntry {
  [field: string]: unknown;
  "address_kind": AddressKind;
  "chain_id": number | bigint | string;
  "recipient": string;
  "recipient_display": string;
  "whitelisted": boolean;
  "withdraw_address": string;
}

export interface WithdrawalEntry {
  [field: string]: unknown;
  "amount": string;
  "asset": string;
  "batch_status": BatchStatus;
  "batch_uuid": string;
  "erc20_address": string;
  "fee": string;
  "forward"?: WithdrawalForward | null;
  "operation_id": string;
  "recipient": string;
  "subaccount_id": number | bigint | string;
  "timestamp": number | bigint | string;
  "tx_hash"?: string | null;
  "wallet": string;
}

export interface WithdrawalForward {
  [field: string]: unknown;
  "amount_forwarded": string | null;
  "chain_id": number | bigint | string;
  "destination_tx"?: string | null;
  "forward_tx"?: string | null;
  "hold_reason"?: string | null;
  "message_id"?: string | null;
  "rail"?: string | null;
  "recipient": string;
  "status": WithdrawalForwardStatus;
}

export interface WithdrawalHistoryResult {
  [field: string]: unknown;
  "withdrawals": Array<WithdrawalEntry>;
}

export interface LoginRequest {
  [field: string]: unknown;
  "signature"?: string | null;
  "timestamp"?: number | bigint | string | null;
  "wallet"?: string | null;
}

export interface SetCancelOnDisconnectRequest {
  [field: string]: unknown;
  "enabled"?: boolean | null;
  "wallet"?: string | null;
}

export interface SubscribeParams {
  [field: string]: unknown;
  "channels": Array<string>;
}

export interface SubscribeResult {
  [field: string]: unknown;
  "current_subscriptions": Array<string>;
  "status": Record<string, HttpValue>;
}

export interface UnsubscribeParams {
  [field: string]: unknown;
  "channels"?: Array<string> | null;
}

export interface UnsubscribeResult {
  [field: string]: unknown;
  "remaining_subscriptions": Array<string>;
  "status": Record<string, HttpValue>;
}

export interface AuctionDetailsSubaccountBalances {
  [field: string]: unknown;
  "{key}"?: string | null;
}

export interface AuctionDetails {
  [field: string]: unknown;
  "currency"?: string | null;
  "estimated_bid_price": string;
  "estimated_discount_pnl": string;
  "estimated_mtm": string;
  "estimated_percent_bid": string;
  "margin_type": string;
  "min_price_limit": string;
  "risk_universe_id": number | bigint | string;
  "subaccount_balances": AuctionDetailsSubaccountBalances;
}

export interface AuctionResult {
  [field: string]: unknown;
  "details"?: AuctionDetails | null;
  "state": AuctionStateType;
  "subaccount_id": number | bigint | string;
  "timestamp": number | bigint | string;
}

export interface BalanceUpdate {
  [field: string]: unknown;
  "name": string;
  "new_balance": string;
  "previous_balance": string;
  "update_type": BalanceUpdateType;
}

export interface BestQuoteChannelResult {
  [field: string]: unknown;
  "error"?: RPCError | null;
  "result"?: RfqGetBestQuoteResponse | null;
  "rfq_id": string;
}

export interface MarginWatchResult {
  [field: string]: unknown;
  "collaterals": Array<HttpValue>;
  "currency": string;
  "initial_margin": string;
  "maintenance_margin": string;
  "margin_type": string;
  "positions": Array<HttpValue>;
  "subaccount_id": number | bigint | string;
  "subaccount_value": string;
  "valuation_timestamp": number | bigint | string;
}

export interface OrderbookPayloadDoc {
  [field: string]: unknown;
  "asks": Array<Array<string>>;
  "bids": Array<Array<string>>;
  "instrument_name": string;
  "publish_id": number | bigint | string;
  "timestamp": number | bigint | string;
}

export interface PublicTrade {
  [field: string]: unknown;
  "direction": Direction;
  "expected_rebate": string;
  "extra_fee": string;
  "index_price": string;
  "instrument_name": string;
  "liquidity_role": LiquidityRole2;
  "mark_price": string;
  "quote_id"?: string | null;
  "realized_pnl": string;
  "realized_pnl_excl_fees": string;
  "rfq_id"?: string | null;
  "subaccount_id": number | bigint | string;
  "timestamp": number | bigint | string;
  "trade_amount": string;
  "trade_fee": string;
  "trade_id": string;
  "trade_price": string;
  "wallet": string;
}

export interface QuotePublishResult {
  [field: string]: unknown;
  "batch_status"?: BatchStatus | null;
  "cancel_reason"?: RFQCancelReason | null;
  "creation_timestamp": number | bigint | string;
  "direction": Direction;
  "extra_fee": string;
  "fee": string;
  "fill_pct": string;
  "is_transfer": boolean;
  "label": string;
  "last_update_timestamp": number | bigint | string;
  "legs": Array<PricedLegParamsAndResponse>;
  "legs_hash": string;
  "liquidity_role": LiquidityRole;
  "max_fee": string;
  "mmp": boolean;
  "nonce"?: string;
  "quote_id": string;
  "rfq_id": string;
  "signature"?: string;
  "signature_expiry_sec"?: number | bigint | string;
  "signer"?: string;
  "status": RFQStatus;
  "subaccount_id": number | bigint | string;
  "tx_hash"?: string | null;
}

export interface SpotFeedEntry {
  [field: string]: unknown;
  "confidence": string;
  "confidence_prev_daily": string;
  "price": string;
  "price_prev_daily": string;
  "timestamp_prev_daily": number | bigint | string;
}

export interface SpotFeedPayloadFeeds {
  [field: string]: unknown;
  "{key}"?: SpotFeedEntry | null;
}

export interface SpotFeedPayload {
  [field: string]: unknown;
  "feeds": SpotFeedPayloadFeeds;
  "timestamp": number | bigint | string;
}

export interface TickerSlimPayload {
  [field: string]: unknown;
  "instrument_ticker": TickerSlimSnapshot;
  "timestamp": number | bigint | string;
}
