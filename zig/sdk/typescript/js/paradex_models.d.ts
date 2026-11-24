// Generated from schema/paradex.json.
import type { HttpValue } from "./index";
export type ResponsesAccountKind = string;
export type ResponsesAlgoType = string;
export type ResponsesBlockTradeStatus = string;
export type ResponsesBlockTradeType = string;
export type ResponsesDepositAddressStatus = string;
export type ResponsesErrorCode = string;
export type ResponsesFillFlag = string;
export type ResponsesFillType = string;
export type ResponsesMarketEventType = string;
export type ResponsesOrderFlag = string;
export type ResponsesOrderInstruction = string;
export type ResponsesOrderSide = string;
export type ResponsesOrderStatus = string;
export type ResponsesOrderType = string;
export type ResponsesRfqVenueState = string;
export type ResponsesSTPMode = string;
export type ResponsesSystemStatus = string;
export type ResponsesTraderRole = string;
export type ResponsesVaultKind = string;
export type ResponsesVaultStatus = string;

export interface ActivateSubkeyParams {
  [field: string]: unknown;
  "body": RequestsActivateSubkey;
}

export interface ArchiveSubAccountParams {
  [field: string]: unknown;
  "address": string;
}

export interface BlockOffersCancelParams {
  [field: string]: unknown;
  "block_trade_id": string;
  "offer_id": string;
}

export interface BlockOffersCreateParams {
  [field: string]: unknown;
  "block_trade_id": string;
  "body": RequestsBlockOfferRequest;
}

export interface BlockOffersExecuteParams {
  [field: string]: unknown;
  "block_trade_id": string;
  "offer_id": string;
  "body": RequestsBlockExecuteRequest;
}

export interface BlockOffersGetParams {
  [field: string]: unknown;
  "block_trade_id": string;
  "offer_id": string;
}

export interface BlockOffersListParams {
  [field: string]: unknown;
  "block_trade_id": string;
}

export interface BlockTradesCancelParams {
  [field: string]: unknown;
  "block_trade_id": string;
}

export interface BlockTradesCreateParams {
  [field: string]: unknown;
  "body": RequestsBlockTradeRequest;
}

export interface BlockTradesExecuteParams {
  [field: string]: unknown;
  "block_trade_id": string;
  "body": RequestsBlockExecuteRequest;
}

export interface BlockTradesGetParams {
  [field: string]: unknown;
  "block_trade_id": string;
}

export interface BlockTradesListParams {
  [field: string]: unknown;
  "status"?: string | null;
  "market"?: string | null;
}

export interface CancelAlgoOrderParams {
  [field: string]: unknown;
  "algo_id": string;
}

export interface CreateAlgoOrderParams {
  [field: string]: unknown;
  "body": RequestsAlgoOrderRequest;
}

export interface CreateDepositAddressParams {
  [field: string]: unknown;
  "body": RequestsCreateDepositAddressReq;
}

export interface CreateTokenParams {
  [field: string]: unknown;
  "body": RequestsCreateToken;
}

export interface CreateXpTransferV2Params {
  [field: string]: unknown;
  "body": RequestsCreateXPTransferRequest;
}

export interface DeleteAccountMmpParams {
  [field: string]: unknown;
  "base_asset": string;
}

export interface EvmAuthParams {
  [field: string]: unknown;
  "PARADEX-STARKNET-ACCOUNT": string;
  "PARADEX-EVM-SIGNATURE": string;
  "PARADEX-SIWE-MESSAGE": string;
  "PARADEX-AUTHORIZE-ISOLATED-MARKETS"?: string | null;
}

export interface EvmOnboardingParams {
  [field: string]: unknown;
  "PARADEX-STARKNET-ACCOUNT": string;
  "PARADEX-EVM-SIGNATURE": string;
  "PARADEX-SIWE-MESSAGE": string;
  "body": RequestsOnboarding;
}

export interface GetAccountHistoryParams {
  [field: string]: unknown;
  "type": string;
}

export interface GetAccountMarginParams {
  [field: string]: unknown;
  "market": string;
}

export interface GetAccountMmpParams {
  [field: string]: unknown;
  "base_asset"?: string | null;
}

export interface GetAccountParams {
  [field: string]: unknown;
  "subaccount_address"?: string | null;
}

export interface GetAccountProfileParams {
  [field: string]: unknown;
}

export interface GetAccountSettingsParams {
  [field: string]: unknown;
}

export interface GetAccountsInfoParams {
  [field: string]: unknown;
}

export interface GetAccountsParams {
  [field: string]: unknown;
}

export interface GetAlgoOrderByIdParams {
  [field: string]: unknown;
  "algo_id": string;
}

export interface GetAlgoOrdersHistoryParams {
  [field: string]: unknown;
  "client_id"?: string | null;
  "cursor"?: string | null;
  "end_at"?: number | bigint | string | null;
  "market"?: string | null;
  "page_size"?: number | bigint | string | null;
  "side"?: string | null;
  "start_at"?: number | bigint | string | null;
  "status"?: string | null;
  "type"?: string | null;
}

export interface GetAnnouncementsParams {
  [field: string]: unknown;
  "cursor"?: string | null;
  "end_at"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "start_at"?: number | bigint | string | null;
}

export interface GetBalanceParams {
  [field: string]: unknown;
}

export interface GetBboInteractiveParams {
  [field: string]: unknown;
  "market": string;
}

export interface GetBboParams {
  [field: string]: unknown;
  "market": string;
}

export interface GetComplianceActionsParams {
  [field: string]: unknown;
}

export interface GetDepositAddressParams {
  [field: string]: unknown;
  "id": string;
}

export interface GetDimeConfigParams {
  [field: string]: unknown;
}

export interface GetFillsParams {
  [field: string]: unknown;
  "asset_kind"?: string | null;
  "cursor"?: string | null;
  "end_at"?: number | bigint | string | null;
  "market"?: string | null;
  "page_size"?: number | bigint | string | null;
  "start_at"?: number | bigint | string | null;
}

export interface GetFundingDataParams {
  [field: string]: unknown;
  "cursor"?: string | null;
  "end_at"?: number | bigint | string | null;
  "market": string;
  "page_size"?: number | bigint | string | null;
  "start_at"?: number | bigint | string | null;
}

export interface GetFundingParams {
  [field: string]: unknown;
  "cursor"?: string | null;
  "end_at"?: number | bigint | string | null;
  "market": string;
  "page_size"?: number | bigint | string | null;
  "start_at"?: number | bigint | string | null;
}

export interface GetGlobalStakingBalanceHistoryParams {
  [field: string]: unknown;
  "resolution": string;
}

export interface GetImpactPriceParams {
  [field: string]: unknown;
  "market": string;
  "size": number | bigint | string;
}

export interface GetInsuranceFundParams {
  [field: string]: unknown;
}

export interface GetJwtParams {
  [field: string]: unknown;
  "PARADEX-STARKNET-ACCOUNT": string;
  "PARADEX-STARKNET-SIGNATURE": string;
  "PARADEX-TIMESTAMP": string;
  "PARADEX-SIGNATURE-EXPIRATION"?: string | null;
  "PARADEX-AUTHORIZE-ISOLATED-MARKETS"?: string | null;
}

export interface GetLiquidationsParams {
  [field: string]: unknown;
  "from"?: number | bigint | string | null;
  "to"?: number | bigint | string | null;
}

export interface GetMarginConfigParams {
  [field: string]: unknown;
  "base_asset"?: string | null;
}

export interface GetMarketSettlementPriceParams {
  [field: string]: unknown;
  "market": string;
}

export interface GetMarketsHistoryParams {
  [field: string]: unknown;
  "market"?: string | null;
}

export interface GetMarketsParams {
  [field: string]: unknown;
  "market"?: string | null;
  "asset_kind"?: string | null;
  "base_asset"?: string | null;
}

export interface GetMarketsSummaryParams {
  [field: string]: unknown;
  "asset_kind"?: string | null;
  "base_asset"?: string | null;
  "end"?: number | bigint | string | null;
  "market": string;
  "start"?: number | bigint | string | null;
}

export interface GetOnboardingParams {
  [field: string]: unknown;
  "public_key"?: string | null;
  "eth_address"?: string | null;
  "account_signer_type": string;
  "vault_operator_index"?: number | bigint | string | null;
}

export interface GetOpenAlgoOrdersParams {
  [field: string]: unknown;
}

export interface GetOpenOrdersParams {
  [field: string]: unknown;
  "market"?: string | null;
}

export interface GetOrderbookInteractiveParams {
  [field: string]: unknown;
  "market": string;
  "depth"?: number | bigint | string | null;
  "price_tick"?: string | null;
}

export interface GetOrderbookParams {
  [field: string]: unknown;
  "market": string;
  "depth"?: number | bigint | string | null;
  "price_tick"?: string | null;
}

export interface GetOrdersParams {
  [field: string]: unknown;
  "client_id"?: string | null;
  "cursor"?: string | null;
  "end_at"?: number | bigint | string | null;
  "market"?: string | null;
  "page_size"?: number | bigint | string | null;
  "side"?: string | null;
  "start_at"?: number | bigint | string | null;
  "status"?: string | null;
  "type"?: string | null;
}

export interface GetPortfolioMarginConfigParams {
  [field: string]: unknown;
  "base_asset"?: string | null;
}

export interface GetPositionsParams {
  [field: string]: unknown;
}

export interface GetPublicXpTransfersV2Params {
  [field: string]: unknown;
  "start_at"?: number | bigint | string | null;
  "end_at"?: number | bigint | string | null;
  "season"?: string | null;
  "cursor"?: string | null;
  "page_size"?: number | bigint | string | null;
}

export interface GetReferralQrCodeParams {
  [field: string]: unknown;
}

export interface GetReferralsConfigParams {
  [field: string]: unknown;
}

export interface GetReferralsSummaryParams {
  [field: string]: unknown;
}

export interface GetStakingBalanceHistoryParams {
  [field: string]: unknown;
  "resolution": string;
}

export interface GetStakingConfigParams {
  [field: string]: unknown;
}

export interface GetStakingHistoryParams {
  [field: string]: unknown;
}

export interface GetStakingSummaryParams {
  [field: string]: unknown;
}

export interface GetSubAccountsParams {
  [field: string]: unknown;
}

export interface GetSubkeyJwtParams {
  [field: string]: unknown;
  "public_key": string;
  "PARADEX-STARKNET-ACCOUNT": string;
  "PARADEX-STARKNET-SIGNATURE": string;
  "PARADEX-TIMESTAMP": string;
  "PARADEX-SIGNATURE-EXPIRATION"?: string | null;
  "PARADEX-AUTHORIZE-ISOLATED-MARKETS"?: string | null;
}

export interface GetSubkeyParams {
  [field: string]: unknown;
  "public_key": string;
}

export interface GetSystemConfigParams {
  [field: string]: unknown;
}

export interface GetSystemStateParams {
  [field: string]: unknown;
}

export interface GetSystemStatsParams {
  [field: string]: unknown;
}

export interface GetSystemTimeParams {
  [field: string]: unknown;
}

export interface GetTransferFeeConfigV2Params {
  [field: string]: unknown;
}

export interface GetTransfersParams {
  [field: string]: unknown;
  "cursor"?: string | null;
  "end_at"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "start_at"?: number | bigint | string | null;
  "status"?: string | null;
}

export interface GetVolumeTiersParams {
  [field: string]: unknown;
}

export interface GetXpBalanceV2Params {
  [field: string]: unknown;
  "season"?: string | null;
}

export interface GetXpTransferByIdV2Params {
  [field: string]: unknown;
  "transfer_id": string;
}

export interface GetXpTransfersV2Params {
  [field: string]: unknown;
  "role"?: string | null;
  "season"?: string | null;
  "cursor"?: string | null;
  "page_size"?: number | bigint | string | null;
}

export interface JwksParams {
  [field: string]: unknown;
}

export interface ListDepositNetworksParams {
  [field: string]: unknown;
}

export interface ListSubkeysParams {
  [field: string]: unknown;
  "with_revoked"?: boolean | null;
}

export interface ListTokensParams {
  [field: string]: unknown;
  "kind"?: string | null;
  "with_invalid"?: boolean | null;
}

export interface MarketsKlinesParams {
  [field: string]: unknown;
  "end_at": number | bigint | string;
  "price_kind"?: string | null;
  "resolution": string;
  "start_at": number | bigint | string;
  "symbol": string;
}

export interface Messagesv1NftPrice {
  [field: string]: unknown;
  "currency"?: string | null;
  "decimals"?: number | bigint | string | null;
  "value"?: string | null;
}

export interface OnboardingParams {
  [field: string]: unknown;
  "PARADEX-ETHEREUM-ACCOUNT": string;
  "PARADEX-STARKNET-ACCOUNT": string;
  "PARADEX-STARKNET-SIGNATURE": string;
  "PARADEX-PARENT-SIWE-MESSAGE"?: string | null;
  "PARADEX-PARENT-SIGNATURE"?: string | null;
  "body": RequestsOnboarding;
}

export interface OrdersCancelAllParams {
  [field: string]: unknown;
  "market"?: string | null;
}

export interface OrdersCancelBatchParams {
  [field: string]: unknown;
  "body": RequestsCancelOrderBatchRequest;
}

export interface OrdersCancelByClientIdParams {
  [field: string]: unknown;
  "client_id": string;
  "market"?: string | null;
}

export interface OrdersCancelParams {
  [field: string]: unknown;
  "order_id": string;
}

export interface OrdersGetByClientIdParams {
  [field: string]: unknown;
  "client_id": string;
}

export interface OrdersGetParams {
  [field: string]: unknown;
  "order_id": string;
}

export interface OrdersModifyParams {
  [field: string]: unknown;
  "order_id": string;
  "body": RequestsModifyOrderRequest;
}

export interface OrdersNewBatchParams {
  [field: string]: unknown;
  "body": Array<RequestsOrderRequest>;
}

export interface OrdersNewParams {
  [field: string]: unknown;
  "body": RequestsOrderRequest;
}

export interface PostAccountTerminalTokenParams {
  [field: string]: unknown;
}

export interface RecordComplianceActionParams {
  [field: string]: unknown;
  "body": RequestsRecordComplianceActionRequest;
}

export interface RefreshAccountInventoryParams {
  [field: string]: unknown;
}

export interface RegisterSubkeyParams {
  [field: string]: unknown;
  "body": RequestsCreateSubkey;
}

export interface RequestsAccountMarginRequest {
  [field: string]: unknown;
  "leverage": number | bigint | string;
  "margin_type": string;
  "on_behalf_of_account"?: string | null;
}

export interface RequestsAccountMmpConfigRequest {
  [field: string]: unknown;
  "delta_limit"?: string | null;
  "frozen_time_ms": number | bigint | string;
  "interval_ms": number | bigint | string;
  "protect_block_trades"?: boolean | null;
  "size_limit"?: string | null;
  "vega_limit"?: string | null;
}

export interface RequestsAccountMmpResetRequest {
  [field: string]: unknown;
  "base_asset"?: string | null;
}

export interface RequestsActivateSubkey {
  [field: string]: unknown;
  "account_id": string;
  "name"?: string | null;
  "public_key": string;
  "signature"?: Array<string>;
  "timestamp": number | bigint | string;
}

export interface RequestsActivateSubkeyResponse {
  [field: string]: unknown;
  "encrypted_key"?: string | null;
}

export interface RequestsAlgoOrderRequest {
  [field: string]: unknown;
  "algo_type": string;
  "dime_discount"?: boolean | null;
  "duration_seconds": number | bigint | string;
  "frequency"?: number | bigint | string | null;
  "market": string;
  "on_behalf_of_account"?: string | null;
  "recv_window"?: number | bigint | string | null;
  "side": ResponsesOrderSide;
  "signature"?: string;
  "signature_timestamp"?: number | bigint | string;
  "size": string;
  "type": ResponsesOrderType;
}

export interface RequestsBlockExecuteRequest {
  [field: string]: unknown;
  "execution_nonce": string;
  "selected_offers"?: Array<string> | null;
  "signatures": Record<string, ResponsesBlockTradeSignature>;
}

export interface RequestsBlockOfferInfo {
  [field: string]: unknown;
  "offerer_order": ResponsesBlockTradeOrder;
  "price": string;
  "size": string;
}

export interface RequestsBlockOfferRequest {
  [field: string]: unknown;
  "nonce"?: string;
  "offering_account": string;
  "signature"?: ResponsesBlockTradeSignature;
  "trades": Record<string, RequestsBlockOfferInfo>;
}

export interface RequestsBlockTradeInfo {
  [field: string]: unknown;
  "maker_order"?: ResponsesBlockTradeOrder | null;
  "price"?: string | null;
  "size"?: string | null;
  "taker_order"?: ResponsesBlockTradeOrder | null;
  "trade_constraints"?: ResponsesBlockTradeConstraints | null;
}

export interface RequestsBlockTradeRequest {
  [field: string]: unknown;
  "block_expiration": number | bigint | string;
  "nonce"?: string;
  "required_signers": Array<string>;
  "signatures": Record<string, ResponsesBlockTradeSignature>;
  "trades": Record<string, RequestsBlockTradeInfo>;
}

export interface RequestsCancelOrderBatchRequest {
  [field: string]: unknown;
  "client_order_ids"?: Array<string> | null;
  "order_ids"?: Array<string> | null;
}

export interface RequestsCreateDepositAddressReq {
  [field: string]: unknown;
  "network": string;
  "token"?: string | null;
}

export interface RequestsCreateSubkey {
  [field: string]: unknown;
  "allowed_cidrs"?: Array<string> | null;
  "encrypted_key"?: string | null;
  "eph_public_key"?: string | null;
  "evm_signature"?: string | null;
  "name": string;
  "public_key": string;
  "signature"?: Array<string> | null;
  "signature_expiry"?: number | bigint | string | null;
  "signature_timestamp"?: number | bigint | string | null;
  "siwe_message"?: string | null;
  "state"?: string | null;
}

export interface RequestsCreateToken {
  [field: string]: unknown;
  "allowed_cidrs"?: Array<string> | null;
  "expiry_duration": number | bigint | string;
  "name": string;
  "token_type": string;
}

export interface RequestsCreateVault {
  [field: string]: unknown;
  "curator_name"?: string | null;
  "deposit_tx_signature"?: string | null;
  "description"?: string | null;
  "lockup_period"?: number | bigint | string | null;
  "max_tvl"?: number | bigint | string | null;
  "name"?: string | null;
  "operator_eth_address"?: string | null;
  "operator_index"?: number | bigint | string | null;
  "operator_signer_type"?: string | null;
  "profit_share"?: number | bigint | string | null;
  "public_key"?: string | null;
  "show_balances"?: boolean | null;
  "show_positions"?: boolean | null;
  "strategy_description"?: string | null;
}

export interface RequestsCreateXPTransferRequest {
  [field: string]: unknown;
  "amount": number | bigint | string;
  "is_private"?: boolean | null;
  "recipient": string;
  "season"?: string | null;
}

export interface RequestsModifyOrderRequest {
  [field: string]: unknown;
  "id": string;
  "market": string;
  "on_behalf_of_account"?: string | null;
  "price": string;
  "side": string;
  "signature"?: string;
  "signature_timestamp"?: number | bigint | string;
  "size": string;
  "trigger_price"?: string | null;
  "type": string;
}

export interface RequestsOnboarding {
  [field: string]: unknown;
  "marketing_code"?: string | null;
  "portfolio_margin"?: boolean | null;
  "public_key"?: string | null;
  "referral_code"?: string | null;
  "utm"?: RequestsUtm | null;
}

export interface RequestsOrderRequest {
  [field: string]: unknown;
  "client_id"?: string | null;
  "dime_discount"?: boolean | null;
  "flags"?: Array<ResponsesOrderFlag> | null;
  "instruction": ResponsesOrderInstruction;
  "market": string;
  "on_behalf_of_account"?: string | null;
  "price": string;
  "recv_window"?: number | bigint | string | null;
  "side": ResponsesOrderSide;
  "signature"?: string;
  "signature_timestamp"?: number | bigint | string;
  "size": string;
  "stp"?: string | null;
  "trigger_price"?: string | null;
  "type": ResponsesOrderType;
  "vwap_price"?: string | null;
}

export interface RequestsRecordComplianceActionRequest {
  [field: string]: unknown;
  "action_type": string;
  "action_version": string;
}

export interface RequestsRfqCreateRequest {
  [field: string]: unknown;
  "counterparties"?: Array<string> | null;
  "is_anonymous"?: boolean | null;
  "label"?: string | null;
  "legs": Array<RequestsRfqLeg>;
  "max_slippage"?: string | null;
  "quantity": string;
  "side"?: string | null;
  "strategy"?: string | null;
}

export interface RequestsRfqExecuteRequest {
  [field: string]: unknown;
  "label"?: string | null;
  "price": string;
  "quantity": string;
  "side": string;
}

export interface RequestsRfqLeg {
  [field: string]: unknown;
  "market": string;
  "price"?: string | null;
  "ratio": string;
  "side": string;
}

export interface RequestsUpdateAccountMaxSlippageRequest {
  [field: string]: unknown;
  "max_slippage": string;
}

export interface RequestsUpdateAccountProfileRequest {
  [field: string]: unknown;
  "ai_agent_url"?: string | null;
  "announcements"?: boolean | null;
  "fill_sound"?: boolean | null;
  "fills"?: boolean | null;
  "is_username_private"?: boolean | null;
  "max_slippage"?: string | null;
  "orders"?: boolean | null;
  "referral_code"?: string | null;
  "referred_by"?: string | null;
  "size_currency_display"?: string | null;
  "tap_share_rate"?: string | null;
  "trading_value_display"?: string | null;
  "transfers"?: boolean | null;
  "username"?: string | null;
  "xp_share_rate"?: string | null;
}

export interface RequestsUpdateAllowedCIDRs {
  [field: string]: unknown;
  "allowed_cidrs"?: Array<string> | null;
}

export interface RequestsUpdateNotificationPreferencesRequest {
  [field: string]: unknown;
  "announcements"?: boolean | null;
  "fill_sound"?: boolean | null;
  "fills"?: boolean | null;
  "orders"?: boolean | null;
  "transfers"?: boolean | null;
}

export interface RequestsUpdateSizeCurrencyDisplayRequest {
  [field: string]: unknown;
  "size_currency_display": string;
}

export interface RequestsUpdateSubkey {
  [field: string]: unknown;
  "name": string;
}

export interface RequestsUpdateSubkeyBlockTradeSigning {
  [field: string]: unknown;
  "enabled": boolean;
}

export interface RequestsUpdateTradingValueDisplayRequest {
  [field: string]: unknown;
  "trading_value_display": string;
}

export interface RequestsUtm {
  [field: string]: unknown;
  "campaign"?: string | null;
  "source"?: string | null;
  "type"?: string | null;
}

export interface ResetAccountMmpParams {
  [field: string]: unknown;
  "body"?: RequestsAccountMmpResetRequest | null;
}

export interface ResponsesAccountAnalytics {
  [field: string]: unknown;
  "account_address"?: string | null;
  "annualized_volatility"?: ResponsesWindowedMetrics | null;
  "apr"?: ResponsesWindowedMetrics | null;
  "apr_simple_itd"?: string | null;
  "cum_return"?: string | null;
  "cumulative_pnl_series"?: Array<ResponsesAccountDailyPnlPoint> | null;
  "cumulative_pnl_usd"?: string | null;
  "end_value"?: string | null;
  "hourly_cumulative_pnl_series"?: Array<ResponsesAccountHourlyPnlPoint> | null;
  "max_dd"?: ResponsesWindowedMetrics | null;
  "max_drawdown_to_date"?: string | null;
  "monthly_pnl_last_12m"?: Array<ResponsesAccountMonthlyPnlEntry> | null;
  "pnl"?: ResponsesWindowedMetrics | null;
  "return"?: ResponsesWindowedMetrics | null;
  "sharpe"?: ResponsesWindowedMetrics | null;
  "sharpe_annualized_itd"?: string | null;
  "strategy_name"?: string | null;
  "vol_annualized_itd"?: string | null;
}

export interface ResponsesAccountDailyPnlPoint {
  [field: string]: unknown;
  "day"?: string | null;
  "value"?: string | null;
}

export interface ResponsesAccountHistoricalDataResp {
  [field: string]: unknown;
  "data"?: Array<number | bigint | string> | null;
  "timestamps"?: Array<number | bigint | string> | null;
}

export interface ResponsesAccountHourlyPnlPoint {
  [field: string]: unknown;
  "at"?: string | null;
  "value"?: string | null;
}

export interface ResponsesAccountInfoResponse {
  [field: string]: unknown;
  "account"?: string | null;
  "archived_at"?: number | bigint | string | null;
  "created_at"?: number | bigint | string | null;
  "derivation_path"?: string | null;
  "fees"?: ResponsesFees | null;
  "has_ecosystem_nft"?: boolean | null;
  "isolated_market"?: string | null;
  "kind"?: ResponsesAccountKind | null;
  "parent_account"?: string | null;
  "public_key"?: string | null;
  "username"?: string | null;
  "volume_tiers"?: Record<string, ResponsesVolumeTierKindInfo> | null;
}

export interface ResponsesAccountMarginEntry {
  [field: string]: unknown;
  "isolated_margin_leverage"?: number | bigint | string | null;
  "leverage"?: number | bigint | string | null;
  "margin_type"?: string | null;
  "market"?: string | null;
}

export interface ResponsesAccountMmpConfigResp {
  [field: string]: unknown;
  "base_asset"?: string | null;
  "delta_limit"?: string | null;
  "frozen_time_ms"?: number | bigint | string | null;
  "frozen_until"?: number | bigint | string | null;
  "interval_ms"?: number | bigint | string | null;
  "is_frozen"?: boolean | null;
  "protect_block_trades"?: boolean | null;
  "size_limit"?: string | null;
  "updated_at"?: number | bigint | string | null;
  "vega_limit"?: string | null;
  "window_delta"?: string | null;
  "window_size"?: string | null;
  "window_vega"?: string | null;
}

export interface ResponsesAccountMmpResetResp {
  [field: string]: unknown;
  "results"?: Array<ResponsesAccountMmpResetResult> | null;
}

export interface ResponsesAccountMmpResetResult {
  [field: string]: unknown;
  "base_asset"?: string | null;
  "reset_at"?: number | bigint | string | null;
}

export interface ResponsesAccountMonthlyPnlEntry {
  [field: string]: unknown;
  "month"?: string | null;
  "month_return_canonical"?: string | null;
}

export interface ResponsesAccountProfileResp {
  [field: string]: unknown;
  "ai_agent_url"?: string | null;
  "discord"?: ResponsesDiscordProfile | null;
  "is_username_private"?: boolean | null;
  "last_seen_notification"?: number | bigint | string | null;
  "market_max_slippage"?: Record<string, string> | null;
  "marketed_by"?: string | null;
  "nfts"?: Array<ResponsesNft> | null;
  "notifications"?: ResponsesNotificationPreferencesResp | null;
  "referral"?: ResponsesReferralConfigResp | null;
  "referral_code"?: string | null;
  "referred_by"?: string | null;
  "size_currency_display"?: string | null;
  "tap_share_rate"?: ResponsesShareRateResp | null;
  "tap_status"?: string | null;
  "twitter"?: ResponsesTwitterProfile | null;
  "twitter_following"?: Record<string, boolean> | null;
  "username"?: string | null;
  "xp_share_rate"?: ResponsesShareRateResp | null;
}

export interface ResponsesAccountSettingsResp {
  [field: string]: unknown;
  "trading_value_display"?: string | null;
}

export interface ResponsesAccountSummaryResponse {
  [field: string]: unknown;
  "account"?: string | null;
  "account_value"?: string | null;
  "free_collateral"?: string | null;
  "initial_margin_requirement"?: string | null;
  "last_seen_notification"?: number | bigint | string | null;
  "maintenance_margin_requirement"?: string | null;
  "margin_cushion"?: string | null;
  "seq_no"?: number | bigint | string | null;
  "settlement_asset"?: string | null;
  "status"?: string | null;
  "total_collateral"?: string | null;
  "updated_at"?: number | bigint | string | null;
  "withdrawal_lock"?: ResponsesWithdrawalLockResponse | null;
  "withdrawal_lock_rejection"?: ResponsesWithdrawalLockRejectionResponse | null;
}

export interface ResponsesAlgoOrderResp {
  [field: string]: unknown;
  "account"?: string | null;
  "algo_params"?: Record<string, string> | null;
  "algo_type"?: ResponsesAlgoType | null;
  "avg_fill_price"?: string | null;
  "cancel_reason"?: string | null;
  "created_at"?: number | bigint | string | null;
  "end_at"?: number | bigint | string | null;
  "id"?: string | null;
  "last_updated_at"?: number | bigint | string | null;
  "market"?: string | null;
  "remaining_size"?: string | null;
  "side"?: ResponsesOrderSide | null;
  "size"?: string | null;
  "status"?: ResponsesOrderStatus | null;
}

export interface ResponsesApiToken {
  [field: string]: unknown;
  "allowed_cidrs"?: Array<string> | null;
  "created_at"?: number | bigint | string | null;
  "expiry_at"?: number | bigint | string | null;
  "kind"?: string | null;
  "last_updated_at"?: number | bigint | string | null;
  "lookup_id"?: string | null;
  "name"?: string | null;
  "revoked_at"?: number | bigint | string | null;
  "token_id"?: string | null;
}

export interface ResponsesAskBidArray {
  [field: string]: unknown;
  "asks"?: Array<Array<string>> | null;
  "best_ask_api"?: Array<string> | null;
  "best_ask_interactive"?: Array<string> | null;
  "best_bid_api"?: Array<string> | null;
  "best_bid_interactive"?: Array<string> | null;
  "bids"?: Array<Array<string>> | null;
  "last_updated_at"?: number | bigint | string | null;
  "market"?: string | null;
  "seq_no"?: number | bigint | string | null;
}

export interface ResponsesAuthResp {
  [field: string]: unknown;
  "jwt_token"?: string | null;
}

export interface ResponsesBBOResp {
  [field: string]: unknown;
  "ask"?: string | null;
  "ask_size"?: string | null;
  "bid"?: string | null;
  "bid_size"?: string | null;
  "last_updated_at"?: number | bigint | string | null;
  "market"?: string | null;
  "seq_no"?: number | bigint | string | null;
}

export interface ResponsesBatchResponse {
  [field: string]: unknown;
  "errors"?: Array<ResponsesErrorResponse> | null;
  "orders"?: Array<ResponsesOrderResp> | null;
}

export interface ResponsesBlockTradeConstraints {
  [field: string]: unknown;
  "max_price"?: string | null;
  "max_size"?: string | null;
  "min_price"?: string | null;
  "min_size"?: string | null;
}

export interface ResponsesBlockTradeDetailFullResponse {
  [field: string]: unknown;
  "block_expiration"?: number | bigint | string | null;
  "block_id"?: string | null;
  "block_type"?: ResponsesBlockTradeType | null;
  "created_at"?: number | bigint | string | null;
  "failure_reason"?: string | null;
  "initiator"?: string | null;
  "last_updated_at"?: number | bigint | string | null;
  "nonce"?: string | null;
  "parent_block_id"?: string | null;
  "required_signers"?: Array<string> | null;
  "signatures"?: Record<string, ResponsesBlockTradeSignature> | null;
  "status"?: ResponsesBlockTradeStatus | null;
  "trades"?: Record<string, ResponsesBlockTradeDetailResponse> | null;
}

export interface ResponsesBlockTradeDetailResponse {
  [field: string]: unknown;
  "executed_at"?: number | bigint | string | null;
  "failure_reason"?: string | null;
  "maker_account"?: string | null;
  "maker_fill"?: ResponsesFillResult | null;
  "maker_order"?: ResponsesBlockTradeOrder | null;
  "market"?: string | null;
  "price"?: string | null;
  "size"?: string | null;
  "taker_account"?: string | null;
  "taker_fill"?: ResponsesFillResult | null;
  "taker_order"?: ResponsesBlockTradeOrder | null;
  "trade_constraints"?: ResponsesBlockTradeConstraints | null;
  "trade_id"?: string | null;
}

export interface ResponsesBlockTradeOrder {
  [field: string]: unknown;
  "account"?: string | null;
  "client_id"?: string | null;
  "flags"?: Array<ResponsesOrderFlag> | null;
  "market"?: string | null;
  "price"?: string | null;
  "side": ResponsesOrderSide;
  "signature"?: string | null;
  "signature_timestamp"?: number | bigint | string | null;
  "size"?: string | null;
  "type": ResponsesOrderType;
}

export interface ResponsesBlockTradeSignature {
  [field: string]: unknown;
  "nonce": string;
  "signature_data": string;
  "signature_expiration": number | bigint | string;
  "signature_timestamp": number | bigint | string;
  "signature_type": string;
  "signer_account": string;
  "signer_public_key"?: string | null;
}

export interface ResponsesBridgedToken {
  [field: string]: unknown;
  "decimals"?: number | bigint | string | null;
  "l1_bridge_address"?: string | null;
  "l1_token_address"?: string | null;
  "l2_bridge_address"?: string | null;
  "l2_token_address"?: string | null;
  "name"?: string | null;
  "symbol"?: string | null;
}

export interface ResponsesCancelOrderBatchResponse {
  [field: string]: unknown;
  "results"?: Array<ResponsesCancelOrderResult> | null;
}

export interface ResponsesCancelOrderResult {
  [field: string]: unknown;
  "account"?: string | null;
  "client_id"?: string | null;
  "id"?: string | null;
  "market"?: string | null;
  "status"?: string | null;
}

export interface ResponsesCreateTokenResponse {
  [field: string]: unknown;
  "api_token"?: ResponsesApiToken | null;
  "token"?: string | null;
}

export interface ResponsesCreateTransferResponse {
  [field: string]: unknown;
  "transfer"?: ResponsesXPTransfer | null;
}

export interface ResponsesDepositAddressDetailsResp {
  [field: string]: unknown;
  "account"?: string | null;
  "address"?: string | null;
  "amount_credited"?: string | null;
  "amount_received"?: string | null;
  "created_at"?: number | bigint | string | null;
  "destination_tx_hash"?: string | null;
  "expires_at"?: number | bigint | string | null;
  "fail_reason"?: string | null;
  "fee"?: string | null;
  "id"?: string | null;
  "network"?: string | null;
  "source_tx_hash"?: string | null;
  "status"?: ResponsesDepositAddressStatus | null;
  "token"?: string | null;
}

export interface ResponsesDepositAddressResp {
  [field: string]: unknown;
  "account"?: string | null;
  "address"?: string | null;
  "avg_completion_time"?: string | null;
  "created_at"?: number | bigint | string | null;
  "expires_at"?: number | bigint | string | null;
  "id"?: string | null;
  "max_amount"?: string | null;
  "min_amount"?: string | null;
  "network"?: string | null;
  "status"?: ResponsesDepositAddressStatus | null;
  "token"?: string | null;
}

export interface ResponsesDepositNetwork {
  [field: string]: unknown;
  "max_amount"?: string | null;
  "min_amount"?: string | null;
  "network"?: string | null;
  "token"?: string | null;
}

export interface ResponsesDepositNetworksResp {
  [field: string]: unknown;
  "results"?: Array<ResponsesDepositNetwork> | null;
}

export interface ResponsesDimeAllocationResp {
  [field: string]: unknown;
  "addresses"?: Array<string> | null;
  "balance"?: string | null;
  "category"?: string | null;
}

export interface ResponsesDimeConfigResponse {
  [field: string]: unknown;
  "allocations"?: Array<ResponsesDimeAllocationResp> | null;
  "circulating_supply"?: string | null;
  "staked_supply"?: string | null;
  "total_supply"?: string | null;
}

export interface ResponsesDiscordProfile {
  [field: string]: unknown;
  "id"?: string | null;
  "image_url"?: string | null;
  "username"?: string | null;
}

export interface ResponsesErrorResponse {
  [field: string]: unknown;
  "error"?: ResponsesErrorCode | null;
  "message"?: string | null;
}

export interface ResponsesFees {
  [field: string]: unknown;
  "commission_rate"?: string | null;
  "dated_option_maker_rate"?: string | null;
  "dated_option_taker_rate"?: string | null;
  "discount_rate"?: string | null;
  "maker_rate"?: string | null;
  "perp_option_maker_rate"?: string | null;
  "perp_option_taker_rate"?: string | null;
  "spot_maker_rate"?: string | null;
  "spot_taker_rate"?: string | null;
  "taker_rate"?: string | null;
}

export interface ResponsesFillResult {
  [field: string]: unknown;
  "account"?: string | null;
  "client_id"?: string | null;
  "created_at"?: number | bigint | string | null;
  "fee"?: string | null;
  "fee_currency"?: string | null;
  "fill_type"?: ResponsesFillType | null;
  "flags"?: Array<ResponsesFillFlag> | null;
  "id"?: string | null;
  "leverage"?: string | null;
  "liquidity"?: ResponsesTraderRole | null;
  "market"?: string | null;
  "order_id"?: string | null;
  "orderbook_seq_no"?: number | bigint | string | null;
  "price"?: string | null;
  "realized_funding"?: string | null;
  "realized_pnl"?: string | null;
  "remaining_size"?: string | null;
  "side"?: ResponsesOrderSide | null;
  "size"?: string | null;
  "underlying_price"?: string | null;
}

export interface ResponsesGetAccountMarginConfigsResp {
  [field: string]: unknown;
  "account"?: string | null;
  "configs"?: Array<ResponsesAccountMarginEntry> | null;
  "margin_methodology"?: string | null;
}

export interface ResponsesGetAccountMmpConfigsResp {
  [field: string]: unknown;
  "account"?: string | null;
  "enabled"?: boolean | null;
  "results"?: Array<ResponsesAccountMmpConfigResp> | null;
}

export interface ResponsesGetAccountsInfoResponse {
  [field: string]: unknown;
  "results"?: Array<ResponsesAccountInfoResponse> | null;
}

export interface ResponsesGetOnboardingResp {
  [field: string]: unknown;
  "account_signer_type"?: string | null;
  "address"?: string | null;
  "derivation_info"?: ResponsesOnboardingDerivationInfo | null;
  "exists"?: boolean | null;
}

export interface ResponsesGetSubAccountsResponse {
  [field: string]: unknown;
  "results"?: Array<ResponsesAccountInfoResponse> | null;
}

export interface ResponsesGetXPBalanceResponseV2 {
  [field: string]: unknown;
  "earned_xp"?: number | bigint | string | null;
  "transferrable_xp"?: number | bigint | string | null;
  "xp_delta"?: number | bigint | string | null;
}

export interface ResponsesImpactPriceResp {
  [field: string]: unknown;
  "impact_price"?: string | null;
  "market"?: string | null;
  "side"?: string | null;
  "size"?: string | null;
}

export interface ResponsesInsuranceAccountResp {
  [field: string]: unknown;
  "account"?: string | null;
  "account_value"?: string | null;
  "settlement_asset"?: string | null;
  "updated_at"?: number | bigint | string | null;
}

export interface ResponsesJwkKey {
  [field: string]: unknown;
  "alg"?: string | null;
  "crv"?: string | null;
  "kid"?: string | null;
  "kty"?: string | null;
  "use"?: string | null;
  "x"?: string | null;
  "y"?: string | null;
}

export interface ResponsesJwksResponse {
  [field: string]: unknown;
  "keys"?: Array<ResponsesJwkKey> | null;
}

export interface ResponsesMarketEvent {
  [field: string]: unknown;
  "changed_fields"?: ResponsesMarketHistoryFields | null;
  "event_type"?: ResponsesMarketEventType | null;
  "symbol"?: string | null;
  "updated_at"?: number | bigint | string | null;
}

export interface ResponsesMarketHistoryFields {
  [field: string]: unknown;
  "asset_kind"?: string | null;
  "base_currency"?: string | null;
  "clamp_rate"?: number | bigint | string | null;
  "delta1_cross_margin_params"?: HttpValue | null;
  "expiry_at"?: number | bigint | string | null;
  "fee_config"?: HttpValue | null;
  "funding_multiplier"?: number | bigint | string | null;
  "funding_period_hours"?: number | bigint | string | null;
  "interest_rate"?: number | bigint | string | null;
  "market_kind"?: string | null;
  "max_funding_rate"?: number | bigint | string | null;
  "max_funding_rate_change"?: number | bigint | string | null;
  "max_open_orders"?: number | bigint | string | null;
  "max_order_size"?: string | null;
  "max_slippage"?: number | bigint | string | null;
  "max_tob_spread"?: number | bigint | string | null;
  "min_notional"?: string | null;
  "open_at"?: number | bigint | string | null;
  "option_cross_margin_params"?: HttpValue | null;
  "option_type"?: string | null;
  "oracle_ewma_factor"?: number | bigint | string | null;
  "order_size_increment"?: string | null;
  "position_limit"?: string | null;
  "price_bands_width"?: number | bigint | string | null;
  "price_tick_size"?: string | null;
  "quote_currency"?: string | null;
  "settlement_currency"?: string | null;
  "settlement_price"?: number | bigint | string | null;
  "strike_price"?: string | null;
  "underlying_twap_price"?: number | bigint | string | null;
}

export interface ResponsesMarketHistoryResponse {
  [field: string]: unknown;
  "results"?: Array<ResponsesMarketEvent> | null;
}

export interface ResponsesMarketSettlementResp {
  [field: string]: unknown;
  "expiry_at"?: number | bigint | string | null;
  "settlement_price"?: string | null;
  "symbol"?: string | null;
  "underlying_twap_price"?: string | null;
}

export interface ResponsesNft {
  [field: string]: unknown;
  "collection_address"?: string | null;
  "collection_name"?: string | null;
  "id"?: string | null;
  "image_url"?: string | null;
  "name"?: string | null;
  "price"?: Messagesv1NftPrice | null;
}

export interface ResponsesNotificationPreferencesResp {
  [field: string]: unknown;
  "announcements"?: boolean | null;
  "fill_sound"?: boolean | null;
  "fills"?: boolean | null;
  "orders"?: boolean | null;
  "transfers"?: boolean | null;
}

export interface ResponsesOnboardingDerivationInfo {
  [field: string]: unknown;
  "class_hash"?: string | null;
  "proxy_class_hash"?: string | null;
}

export interface ResponsesOrderResp {
  [field: string]: unknown;
  "account"?: string | null;
  "avg_fill_price"?: string | null;
  "cancel_reason"?: string | null;
  "client_id"?: string | null;
  "created_at"?: number | bigint | string | null;
  "flags"?: Array<ResponsesOrderFlag> | null;
  "id"?: string | null;
  "instruction"?: ResponsesOrderInstruction | null;
  "last_updated_at"?: number | bigint | string | null;
  "market"?: string | null;
  "price"?: string | null;
  "published_at"?: number | bigint | string | null;
  "received_at"?: number | bigint | string | null;
  "remaining_size"?: string | null;
  "request_info"?: ResponsesRequestInfo | null;
  "seq_no"?: number | bigint | string | null;
  "side"?: ResponsesOrderSide | null;
  "size"?: string | null;
  "status"?: ResponsesOrderStatus | null;
  "stp"?: ResponsesSTPMode | null;
  "timestamp"?: number | bigint | string | null;
  "trigger_price"?: string | null;
  "type"?: ResponsesOrderType | null;
}

export interface ResponsesParadigmEnableResponse {
  [field: string]: unknown;
  "account"?: string | null;
  "status"?: string | null;
  "subkey_public_key"?: string | null;
}

export interface ResponsesReferralConfigResp {
  [field: string]: unknown;
  "commission_rate"?: string | null;
  "commission_volume_cap"?: string | null;
  "discount_rate"?: string | null;
  "discount_volume_cap"?: string | null;
  "minimum_volume"?: string | null;
  "name"?: string | null;
  "points_bonus_rate"?: string | null;
  "points_bonus_volume_cap"?: string | null;
  "referral_type"?: string | null;
}

export interface ResponsesRefreshInventoryResp {
  [field: string]: unknown;
  "nfts"?: Array<ResponsesNft> | null;
}

export interface ResponsesRequestInfo {
  [field: string]: unknown;
  "id"?: string | null;
  "message"?: string | null;
  "request_type"?: string | null;
  "status"?: string | null;
}

export interface ResponsesRevokeSubkeyResponse {
  [field: string]: unknown;
  "public_key"?: string | null;
}

export interface ResponsesRevokeTokenResponse {
  [field: string]: unknown;
  "lookup_id"?: string | null;
}

export interface ResponsesRfqAcceptedResponse {
  [field: string]: unknown;
  "accepted_at"?: number | bigint | string | null;
  "request_id"?: string | null;
}

export interface ResponsesRfqBboLegResult {
  [field: string]: unknown;
  "best_ask_iv"?: string | null;
  "best_ask_price"?: string | null;
  "best_ask_quantity"?: string | null;
  "best_bid_iv"?: string | null;
  "best_bid_price"?: string | null;
  "best_bid_quantity"?: string | null;
  "mark_price"?: string | null;
  "mark_price_iv"?: string | null;
  "market"?: string | null;
}

export interface ResponsesRfqBboResult {
  [field: string]: unknown;
  "best_ask_price"?: string | null;
  "best_ask_quantity"?: string | null;
  "best_bid_price"?: string | null;
  "best_bid_quantity"?: string | null;
  "created_at"?: number | bigint | string | null;
  "legs"?: Array<ResponsesRfqBboLegResult> | null;
  "mark_price"?: string | null;
  "max_price"?: string | null;
  "min_price"?: string | null;
  "rfq_id"?: string | null;
}

export interface ResponsesRfqDraftResult {
  [field: string]: unknown;
  "created_at"?: number | bigint | string | null;
  "id"?: string | null;
  "legs"?: Array<ResponsesRfqLegResult> | null;
  "quantity"?: string | null;
  "strategy"?: string | null;
  "updated_at"?: number | bigint | string | null;
}

export interface ResponsesRfqLegResult {
  [field: string]: unknown;
  "market"?: string | null;
  "price"?: string | null;
  "ratio"?: string | null;
  "side"?: string | null;
}

export interface ResponsesRfqMarketInfo {
  [field: string]: unknown;
  "symbol"?: string | null;
  "venue_size_increment"?: string | null;
}

export interface ResponsesRfqMarketsResult {
  [field: string]: unknown;
  "markets"?: Array<string> | null;
  "markets_info"?: Array<ResponsesRfqMarketInfo> | null;
}

export interface ResponsesRfqPlatformStateResult {
  [field: string]: unknown;
  "next_platform_state"?: ResponsesRfqVenueState | null;
  "next_platform_state_at"?: number | bigint | string | null;
  "platform_state"?: ResponsesRfqVenueState | null;
  "state"?: ResponsesRfqVenueState | null;
  "venue_state"?: ResponsesRfqVenueState | null;
}

export interface ResponsesRfqResult {
  [field: string]: unknown;
  "closed_reason"?: string | null;
  "counterparties"?: Array<string> | null;
  "created_at"?: number | bigint | string | null;
  "expires_at"?: number | bigint | string | null;
  "filled_quantity"?: string | null;
  "id"?: string | null;
  "label"?: string | null;
  "legs"?: Array<ResponsesRfqLegResult> | null;
  "mark_price"?: string | null;
  "quantity"?: string | null;
  "state"?: string | null;
}

export interface ResponsesRfqResults {
  [field: string]: unknown;
  "next"?: string | null;
  "results"?: Array<ResponsesRfqResult> | null;
}

export interface ResponsesShareRateResp {
  [field: string]: unknown;
  "last_updated_at"?: number | bigint | string | null;
  "share_rate"?: string | null;
}

export interface ResponsesStakingConfigResponse {
  [field: string]: unknown;
  "dime_fee_discount_pct"?: string | null;
  "tiers"?: Array<ResponsesStakingTierResp> | null;
}

export interface ResponsesStakingSummaryResponse {
  [field: string]: unknown;
  "account"?: string | null;
  "active_stakes"?: number | bigint | string | null;
  "staking_tier"?: string | null;
  "total_staked"?: string | null;
}

export interface ResponsesStakingTierResp {
  [field: string]: unknown;
  "fee_rates"?: Record<string, string> | null;
  "min_staked"?: string | null;
  "tier_name"?: string | null;
  "xp_multiplier"?: string | null;
}

export interface ResponsesStrategy {
  [field: string]: unknown;
  "address"?: string | null;
  "name"?: string | null;
  "visibility"?: ResponsesVisibility | null;
}

export interface ResponsesSubkey {
  [field: string]: unknown;
  "account_id"?: string | null;
  "allowed_cidrs"?: Array<string> | null;
  "block_trade_signing_enabled"?: boolean | null;
  "created_at"?: number | bigint | string | null;
  "id"?: number | bigint | string | null;
  "name"?: string | null;
  "public_key"?: string | null;
  "revoked_at"?: number | bigint | string | null;
  "state"?: string | null;
  "updated_at"?: number | bigint | string | null;
}

export interface ResponsesSystemConfigResponse {
  [field: string]: unknown;
  "assets_limit_per_account"?: number | bigint | string | null;
  "block_explorer_url"?: string | null;
  "bridged_tokens"?: Array<ResponsesBridgedToken> | null;
  "dime_staking_contract_address"?: string | null;
  "environment"?: string | null;
  "l1_chain_id"?: string | null;
  "l1_core_contract_address"?: string | null;
  "l1_operator_address"?: string | null;
  "l1_relayer_address"?: string | null;
  "l2_relayer_address"?: string | null;
  "liquidation_fee"?: string | null;
  "oracle_address"?: string | null;
  "paraclear_account_hash"?: string | null;
  "paraclear_account_proxy_hash"?: string | null;
  "paraclear_address"?: string | null;
  "paraclear_decimals"?: number | bigint | string | null;
  "paraclear_evm_account_hash"?: string | null;
  "partial_liquidation_buffer"?: string | null;
  "partial_liquidation_share_increment"?: string | null;
  "rfq_default_max_slippage"?: string | null;
  "starknet_chain_id"?: string | null;
  "starknet_fullnode_rpc_base_url"?: string | null;
  "starknet_fullnode_rpc_url"?: string | null;
  "starknet_gateway_url"?: string | null;
  "universal_deployer_address"?: string | null;
}

export interface ResponsesSystemStateResponse {
  [field: string]: unknown;
  "status"?: ResponsesSystemStatus | null;
}

export interface ResponsesSystemStatsResponse {
  [field: string]: unknown;
  "lifetime_volume"?: string | null;
}

export interface ResponsesSystemTimeResponse {
  [field: string]: unknown;
  "server_time"?: string | null;
}

export interface ResponsesTwitterProfile {
  [field: string]: unknown;
  "id"?: string | null;
  "image_url"?: string | null;
  "username"?: string | null;
}

export interface ResponsesUpdateAccountMarginConfigResp {
  [field: string]: unknown;
  "account"?: string | null;
  "leverage"?: number | bigint | string | null;
  "margin_type"?: string | null;
  "market"?: string | null;
}

export interface ResponsesVaultAnalytics {
  [field: string]: unknown;
  "address"?: string | null;
  "annualized_volatility"?: ResponsesWindowedMetrics | null;
  "apr"?: ResponsesWindowedMetrics | null;
  "as_of"?: string | null;
  "best_period_return"?: ResponsesWindowedMetrics | null;
  "max_drawdown"?: ResponsesWindowedMetrics | null;
  "monthly_pnl_last_12m"?: Array<ResponsesVaultMonthlyPnlEntry> | null;
  "pct_positive"?: ResponsesWindowedMetrics | null;
  "pnl"?: ResponsesWindowedMetrics | null;
  "recovery_hours"?: ResponsesWindowedMetrics | null;
  "sharpe_ratio"?: ResponsesWindowedMetrics | null;
  "top_holders"?: Array<ResponsesVaultTopHolder> | null;
  "worst_period_return"?: ResponsesWindowedMetrics | null;
}

export interface ResponsesVaultAnalyticsResp {
  [field: string]: unknown;
  "address"?: string | null;
  "as_of"?: string | null;
  "substrategies"?: Array<ResponsesAccountAnalytics> | null;
  "vault_level"?: ResponsesVaultAnalytics | null;
}

export interface ResponsesVaultMonthlyPnlEntry {
  [field: string]: unknown;
  "month"?: string | null;
  "month_return_pct"?: string | null;
}

export interface ResponsesVaultResp {
  [field: string]: unknown;
  "address"?: string | null;
  "created_at"?: number | bigint | string | null;
  "curator_name"?: string | null;
  "description"?: string | null;
  "kind"?: ResponsesVaultKind | null;
  "last_updated_at"?: number | bigint | string | null;
  "lockup_period"?: number | bigint | string | null;
  "max_tvl"?: number | bigint | string | null;
  "name"?: string | null;
  "operator_account"?: string | null;
  "owner_account"?: string | null;
  "profit_share"?: number | bigint | string | null;
  "status"?: ResponsesVaultStatus | null;
  "strategies"?: Array<ResponsesStrategy> | null;
  "strategy_description"?: string | null;
  "token_address"?: string | null;
  "visibility"?: ResponsesVisibility | null;
}

export interface ResponsesVaultTopHolder {
  [field: string]: unknown;
  "percentage"?: string | null;
  "rank"?: number | bigint | string | null;
  "value_usdc"?: string | null;
}

export interface ResponsesVaultsConfigResponse {
  [field: string]: unknown;
  "max_lockup_period_days"?: string | null;
  "max_profit_share_percentage"?: string | null;
  "min_initial_deposit"?: string | null;
  "min_lockup_period_days"?: string | null;
  "min_owner_share_percentage"?: string | null;
  "profit_share_period"?: string | null;
  "vault_factory_address"?: string | null;
}

export interface ResponsesVisibility {
  [field: string]: unknown;
  "balances"?: boolean | null;
  "positions"?: boolean | null;
}

export interface ResponsesVolumeTierKindInfo {
  [field: string]: unknown;
  "tier"?: string | null;
  "volume"?: string | null;
}

export interface ResponsesWindowedMetrics {
  [field: string]: unknown;
  "window_1y"?: string | null;
  "window_24h"?: string | null;
  "window_30d"?: string | null;
  "window_7d"?: string | null;
  "window_90d"?: string | null;
  "window_all"?: string | null;
}

export interface ResponsesWithdrawalLockRejectionResponse {
  [field: string]: unknown;
  "amount"?: string | null;
  "reason"?: string | null;
  "rejected_at"?: number | bigint | string | null;
  "request_id"?: string | null;
}

export interface ResponsesWithdrawalLockResponse {
  [field: string]: unknown;
  "amount"?: string | null;
  "expires_at"?: number | bigint | string | null;
  "request_id"?: string | null;
  "status"?: string | null;
}

export interface ResponsesXPTransfer {
  [field: string]: unknown;
  "amount"?: number | bigint | string | null;
  "created_at"?: number | bigint | string | null;
  "fee"?: number | bigint | string | null;
  "id"?: string | null;
  "is_private"?: boolean | null;
  "recipient"?: string | null;
  "season"?: string | null;
  "sender"?: string | null;
}

export interface RevokeSubkeyParams {
  [field: string]: unknown;
  "public_key": string;
}

export interface RevokeTokenParams {
  [field: string]: unknown;
  "lookup_id": string;
}

export interface RfqBboParams {
  [field: string]: unknown;
  "rfq_id": string;
}

export interface RfqCreateParams {
  [field: string]: unknown;
  "body": RequestsRfqCreateRequest;
}

export interface RfqDeleteParams {
  [field: string]: unknown;
  "rfq_id": string;
}

export interface RfqDraftCreateParams {
  [field: string]: unknown;
  "body": RequestsRfqCreateRequest;
}

export interface RfqDraftDeleteParams {
  [field: string]: unknown;
  "draft_id": string;
}

export interface RfqDraftListParams {
  [field: string]: unknown;
}

export interface RfqEnableParams {
  [field: string]: unknown;
}

export interface RfqExecuteParams {
  [field: string]: unknown;
  "rfq_id": string;
  "body": RequestsRfqExecuteRequest;
}

export interface RfqListParams {
  [field: string]: unknown;
  "state"?: string | null;
  "cursor"?: string | null;
  "page_size"?: number | bigint | string | null;
}

export interface RfqMarketsParams {
  [field: string]: unknown;
}

export interface RfqPlatformStateParams {
  [field: string]: unknown;
}

export interface ServicesTransferFeeConfigResponse {
  [field: string]: unknown;
  "fee_percentage"?: number | bigint | string | null;
}

export interface TradebustsParams {
  [field: string]: unknown;
  "cursor"?: string | null;
  "end_at"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "start_at"?: number | bigint | string | null;
}

export interface TradesParams {
  [field: string]: unknown;
  "base_asset"?: string | null;
  "cursor"?: string | null;
  "end_at"?: number | bigint | string | null;
  "market"?: string | null;
  "page_size"?: number | bigint | string | null;
  "start_at"?: number | bigint | string | null;
  "trade_type"?: string | null;
}

export interface TransactionsParams {
  [field: string]: unknown;
  "cursor"?: string | null;
  "end_at"?: number | bigint | string | null;
  "page_size"?: number | bigint | string | null;
  "start_at"?: number | bigint | string | null;
}

export interface UpdateAccountMarketMaxSlippageParams {
  [field: string]: unknown;
  "market": string;
  "body": RequestsUpdateAccountMaxSlippageRequest;
}

export interface UpdateAccountMaxSlippageParams {
  [field: string]: unknown;
  "body": UpdateAccountMaxSlippageParamsBody;
}

export interface UpdateAccountMaxSlippageParamsBody {
  [field: string]: unknown;
  "max_slippage": string;
}

export interface UpdateAccountNotificationPreferencesParams {
  [field: string]: unknown;
  "body": RequestsUpdateNotificationPreferencesRequest;
}

export interface UpdateAccountProfileParams {
  [field: string]: unknown;
  "body": RequestsUpdateAccountProfileRequest;
}

export interface UpdateAccountProfileReferralCodeParams {
  [field: string]: unknown;
}

export interface UpdateAccountProfileUsernameParams {
  [field: string]: unknown;
}

export interface UpdateAccountReferrerParams {
  [field: string]: unknown;
}

export interface UpdateAccountSizeCurrencyDisplayParams {
  [field: string]: unknown;
  "body": RequestsUpdateSizeCurrencyDisplayRequest;
}

export interface UpdateAccountTradingValueDisplayParams {
  [field: string]: unknown;
  "body": RequestsUpdateTradingValueDisplayRequest;
}

export interface UpdateLastSeenParams {
  [field: string]: unknown;
}

export interface UpdateSubkeyAllowedCidrsParams {
  [field: string]: unknown;
  "public_key": string;
  "body": RequestsUpdateAllowedCIDRs;
}

export interface UpdateSubkeyBlockTradeSigningParams {
  [field: string]: unknown;
  "public_key": string;
  "body": RequestsUpdateSubkeyBlockTradeSigning;
}

export interface UpdateSubkeyParams {
  [field: string]: unknown;
  "public_key": string;
  "body": RequestsUpdateSubkey;
}

export interface UpdateTokenAllowedCidrsParams {
  [field: string]: unknown;
  "lookup_id": string;
  "body": RequestsUpdateAllowedCIDRs;
}

export interface UpsertAccountMarginParams {
  [field: string]: unknown;
  "market": string;
  "body": RequestsAccountMarginRequest;
}

export interface UpsertAccountMmpParams {
  [field: string]: unknown;
  "base_asset": string;
  "body": RequestsAccountMmpConfigRequest;
}

export interface VaultsCreateParams {
  [field: string]: unknown;
  "PARADEX-PARENT-ACCOUNT": string;
  "PARADEX-STARKNET-SIGNATURE": string;
  "PARADEX-STARKNET-ACCOUNT": string;
  "PARADEX-STARKNET-ACCOUNT-DERIVATION-PATH": string;
  "body": RequestsCreateVault;
}

export interface VaultsGetAccountSummaryParams {
  [field: string]: unknown;
  "address"?: string | null;
}

export interface VaultsGetAnalyticsParams {
  [field: string]: unknown;
  "address": string;
}

export interface VaultsGetBalanceParams {
  [field: string]: unknown;
  "address": string;
  "strategy"?: string | null;
}

export interface VaultsGetConfigParams {
  [field: string]: unknown;
}

export interface VaultsGetHistoricalDataParams {
  [field: string]: unknown;
  "address": string;
  "strategy"?: string | null;
  "type": string;
  "resolution": string;
}

export interface VaultsGetMineParams {
  [field: string]: unknown;
}

export interface VaultsGetParams {
  [field: string]: unknown;
  "address"?: string | null;
}

export interface VaultsGetPositionsParams {
  [field: string]: unknown;
  "address": string;
  "strategy"?: string | null;
}

export interface VaultsGetSummaryParams {
  [field: string]: unknown;
  "address"?: string | null;
}

export interface VaultsGetTransfersParams {
  [field: string]: unknown;
  "address": string;
}
