// Generated from src/exchanges/pascal/schema/pascal.json.
import type { HttpValue } from "./index";
export type CancelOrdersResponse = Array<CancelOrdersResponseValueItem>;
export type MassCancelOrdersResponse = Array<MassCancelOrdersResponseValueItem>;
export type PlaceAndReplaceOrdersResponse = Array<PlaceAndReplaceOrdersResponseValueItem>;

export interface AccountBioParams {
  [field: string]: unknown;
  "owner": string;
}

export interface AccountBioResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: AccountBioResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface AccountBioResponseData {
  [field: string]: unknown;
  "account_bio"?: string | null;
}

export interface AccountStateSnapshotParams {
  [field: string]: unknown;
  "symbol"?: string | null;
  "exclude_orders"?: boolean | null;
  "owner": string;
}

export interface AccountStateSnapshotResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: AccountStateSnapshotResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface AccountStateSnapshotResponseData {
  [field: string]: unknown;
  "has_deposited"?: boolean | null;
  "username"?: string | null;
  "twitter_account_connection"?: AccountStateSnapshotResponseDataTwitterAccountConnection | null;
  "collateral_usd"?: string | null;
  "rewards"?: AccountStateSnapshotResponseDataRewards | null;
  "lifetime_stats"?: AccountStateSnapshotResponseDataLifetimeStats | null;
  "orders"?: Array<AccountStateSnapshotResponseDataOrdersItem> | null;
  "positions"?: Array<AccountStateSnapshotResponseDataPositionsItem> | null;
  "pending_deposits"?: Array<AccountStateSnapshotResponseDataPendingDepositsItem> | null;
  "pending_withdrawals"?: Array<AccountStateSnapshotResponseDataPendingWithdrawalsItem> | null;
}

export interface AccountStateSnapshotResponseDataLifetimeStats {
  [field: string]: unknown;
  "maker"?: AccountStateSnapshotResponseDataLifetimeStatsMaker | null;
  "taker"?: AccountStateSnapshotResponseDataLifetimeStatsTaker | null;
}

export interface AccountStateSnapshotResponseDataLifetimeStatsMaker {
  [field: string]: unknown;
  "volume_contracts"?: string | null;
  "weighted_volume"?: string | null;
  "volume_notional_usd"?: string | null;
  "fees_paid_usd"?: string | null;
}

export interface AccountStateSnapshotResponseDataLifetimeStatsTaker {
  [field: string]: unknown;
  "volume_contracts"?: string | null;
  "weighted_volume"?: string | null;
  "volume_notional_usd"?: string | null;
  "fees_paid_usd"?: string | null;
}

export interface AccountStateSnapshotResponseDataOrdersItem {
  [field: string]: unknown;
  "id"?: string | null;
  "client_order_id"?: string | null;
  "symbol"?: string | null;
  "side"?: string | null;
  "price"?: string | null;
  "size_original"?: string | null;
  "size_remaining"?: string | null;
  "size_filled"?: string | null;
  "notional_filled"?: string | null;
  "tif"?: string | null;
  "type"?: string | null;
  "post_only"?: boolean | null;
  "reduce_only"?: boolean | null;
  "seq"?: string | null;
  "update_seq"?: string | null;
  "place_ts_ms"?: string | null;
  "update_ts_ms"?: string | null;
}

export interface AccountStateSnapshotResponseDataPendingDepositsItem {
  [field: string]: unknown;
  "deposit_id"?: string | null;
  "owner"?: string | null;
  "amount"?: string | null;
  "transfer_status"?: string | null;
}

export interface AccountStateSnapshotResponseDataPendingWithdrawalsItem {
  [field: string]: unknown;
  "owner"?: string | null;
  "status"?: string | null;
  "amount"?: string | null;
  "destination_authority"?: string | null;
}

export interface AccountStateSnapshotResponseDataPositionsItem {
  [field: string]: unknown;
  "symbol"?: string | null;
  "size"?: string | null;
  "cumulative_increasing_contracts"?: string | null;
  "cumulative_increasing_notional_usd"?: string | null;
  "cumulative_reducing_contracts"?: string | null;
  "cumulative_reducing_notional_usd"?: string | null;
  "cumulative_fees_usd"?: string | null;
  "realized_pnl_usd"?: string | null;
  "unrealized_pnl_usd"?: string | null;
  "mark_price"?: string | null;
  "average_entry_price"?: string | null;
  "seq"?: string | null;
  "update_seq"?: string | null;
  "open_ts_ms"?: string | null;
  "update_ts_ms"?: string | null;
}

export interface AccountStateSnapshotResponseDataRewards {
  [field: string]: unknown;
  "referral_rewards_received_usd"?: string | null;
  "builder_fees_received_usd"?: string | null;
  "user_rewards_received_usd"?: string | null;
}

export interface AccountStateSnapshotResponseDataTwitterAccountConnection {
  [field: string]: unknown;
  "twitter_user_id"?: string | null;
  "twitter_handle"?: string | null;
  "connect_seq"?: string | null;
}

export interface ApplyAReferralCodeParams {
  [field: string]: unknown;
  "body": ApplyAReferralCodeParamsBody;
}

export interface ApplyAReferralCodeParamsBody {
  [field: string]: unknown;
  "referral_code": string;
  "auth"?: Record<string, HttpValue> | null;
}

export interface ApplyAReferralCodeResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Record<string, HttpValue> | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface BatchMarkPriceCandlesParams {
  [field: string]: unknown;
  "symbols": string;
  "interval": string;
  "end_time_ms": number | bigint | string;
  "start_time_ms"?: number | bigint | string | null;
  "count_back"?: number | bigint | string | null;
}

export interface BatchMarkPriceCandlesResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: BatchMarkPriceCandlesResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface BatchMarkPriceCandlesResponseData {
  [field: string]: unknown;
  "items"?: Array<BatchMarkPriceCandlesResponseDataItemsItem> | null;
}

export interface BatchMarkPriceCandlesResponseDataItemsItem {
  [field: string]: unknown;
  "symbol"?: string | null;
  "candles"?: Array<BatchMarkPriceCandlesResponseDataItemsItemCandlesItem> | null;
}

export interface BatchMarkPriceCandlesResponseDataItemsItemCandlesItem {
  [field: string]: unknown;
  "time"?: string | null;
  "open"?: string | null;
  "high"?: string | null;
  "low"?: string | null;
  "close"?: string | null;
}

export interface BatchTradesParams {
  [field: string]: unknown;
  "symbols": string;
  "limit"?: number | bigint | string | null;
}

export interface BatchTradesResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: BatchTradesResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface BatchTradesResponseData {
  [field: string]: unknown;
  "items"?: Array<BatchTradesResponseDataItemsItem> | null;
}

export interface BatchTradesResponseDataItemsItem {
  [field: string]: unknown;
  "symbol"?: string | null;
  "trades"?: Array<BatchTradesResponseDataItemsItemTradesItem> | null;
}

export interface BatchTradesResponseDataItemsItemTradesItem {
  [field: string]: unknown;
  "trade_id"?: string | null;
  "symbol"?: string | null;
  "taker_side"?: string | null;
  "price"?: string | null;
  "size"?: string | null;
  "maker"?: string | null;
  "taker"?: string | null;
  "maker_reduce_only"?: boolean | null;
  "taker_reduce_only"?: boolean | null;
  "maker_position_size_prev"?: string | null;
  "taker_position_size_prev"?: string | null;
  "seq"?: string | null;
  "trade_ts_ms"?: string | null;
}

export interface BuilderStatsParams {
  [field: string]: unknown;
  "owner": string;
}

export interface BuilderStatsResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: BuilderStatsResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface BuilderStatsResponseData {
  [field: string]: unknown;
  "builder_volume_contracts"?: string | null;
  "builder_volume_notional_usd"?: string | null;
  "builder_fee_base_usd"?: string | null;
  "builder_fees_received_usd"?: string | null;
}

export interface CancelOrdersParams {
  [field: string]: unknown;
  "body": Array<CancelOrdersParamsBodyItem>;
}

export interface CancelOrdersParamsBodyItem {
  [field: string]: unknown;
  "client_order_id": string;
  "order_id": string;
  "auth"?: Record<string, HttpValue> | null;
}

export interface CancelOrdersResponseItem {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: CancelOrdersResponseItemData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface CancelOrdersResponseItemData {
  [field: string]: unknown;
  "id"?: string | null;
  "client_order_id"?: string | null;
  "symbol"?: string | null;
  "side"?: string | null;
  "price"?: string | null;
  "size_original"?: string | null;
  "size_remaining"?: string | null;
  "size_filled"?: string | null;
  "notional_filled"?: string | null;
  "tif"?: string | null;
  "type"?: string | null;
  "post_only"?: boolean | null;
  "reduce_only"?: boolean | null;
  "seq"?: string | null;
  "update_seq"?: string | null;
  "place_ts_ms"?: string | null;
  "update_ts_ms"?: string | null;
}

export interface CancelOrdersResponseValueItem {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: CancelOrdersResponseValueItemData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface CancelOrdersResponseValueItemData {
  [field: string]: unknown;
  "id"?: string | null;
  "client_order_id"?: string | null;
  "symbol"?: string | null;
  "side"?: string | null;
  "price"?: string | null;
  "size_original"?: string | null;
  "size_remaining"?: string | null;
  "size_filled"?: string | null;
  "notional_filled"?: string | null;
  "tif"?: string | null;
  "type"?: string | null;
  "post_only"?: boolean | null;
  "reduce_only"?: boolean | null;
  "seq"?: string | null;
  "update_seq"?: string | null;
  "place_ts_ms"?: string | null;
  "update_ts_ms"?: string | null;
}

export interface ClaimACustomReferralCodeParams {
  [field: string]: unknown;
  "body": ClaimACustomReferralCodeParamsBody;
}

export interface ClaimACustomReferralCodeParamsBody {
  [field: string]: unknown;
  "referral_code": string;
  "auth"?: Record<string, HttpValue> | null;
}

export interface ClaimACustomReferralCodeResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: ClaimACustomReferralCodeResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface ClaimACustomReferralCodeResponseData {
  [field: string]: unknown;
  "custom_referral_code"?: string | null;
}

export interface ClosedPositionHistoryParams {
  [field: string]: unknown;
  "symbols"?: string | null;
  "order_by"?: string | null;
  "sort_order"?: string | null;
  "before_cursor"?: string | null;
  "limit"?: number | bigint | string | null;
  "owner": string;
}

export interface ClosedPositionHistoryResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: ClosedPositionHistoryResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface ClosedPositionHistoryResponseData {
  [field: string]: unknown;
  "items"?: Array<ClosedPositionHistoryResponseDataItemsItem> | null;
  "next_cursor"?: string | null;
}

export interface ClosedPositionHistoryResponseDataItemsItem {
  [field: string]: unknown;
  "symbol"?: string | null;
  "position_side"?: string | null;
  "position_instance_count"?: string | null;
  "held_until_resolution"?: boolean | null;
  "realized_pnl_usd"?: string | null;
  "cumulative_increasing_contracts"?: string | null;
  "cumulative_increasing_notional_usd"?: string | null;
  "cumulative_reducing_notional_usd"?: string | null;
  "average_increasing_price"?: string | null;
  "average_reducing_price"?: string | null;
  "open_seq"?: string | null;
  "open_ts_ms"?: string | null;
  "close_seq"?: string | null;
  "close_ts_ms"?: string | null;
}

export interface CreateATradingKeyParams {
  [field: string]: unknown;
  "body": CreateATradingKeyParamsBody;
}

export interface CreateATradingKeyParamsBody {
  [field: string]: unknown;
  "trading_key": string;
  "name": string;
  "expiration_ts_ms": string;
  "auth"?: Record<string, HttpValue> | null;
}

export interface CreateATradingKeyResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: CreateATradingKeyResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface CreateATradingKeyResponseData {
  [field: string]: unknown;
  "trading_key"?: string | null;
  "name"?: string | null;
  "expiration_ts_ms"?: string | null;
  "open_order_count"?: string | null;
}

export interface DepositAddressParams {
  [field: string]: unknown;
  "owner": string;
}

export interface DepositAddressResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: DepositAddressResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface DepositAddressResponseData {
  [field: string]: unknown;
  "owner"?: string | null;
  "deposit_address"?: string | null;
  "registered"?: boolean | null;
}

export interface FillHistoryParams {
  [field: string]: unknown;
  "symbols"?: string | null;
  "before_cursor"?: string | null;
  "at_or_before_seq"?: string | null;
  "limit"?: number | bigint | string | null;
  "owner": string;
}

export interface FillHistoryResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: FillHistoryResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface FillHistoryResponseData {
  [field: string]: unknown;
  "items"?: Array<FillHistoryResponseDataItemsItem> | null;
}

export interface FillHistoryResponseDataItemsItem {
  [field: string]: unknown;
  "trade_id"?: string | null;
  "symbol"?: string | null;
  "side"?: string | null;
  "reduce_only"?: boolean | null;
  "fill_price"?: string | null;
  "fill_size"?: string | null;
  "liquidity"?: string | null;
  "counterparty"?: string | null;
  "order_id"?: string | null;
  "client_order_id"?: string | null;
  "order_progress"?: FillHistoryResponseDataItemsItemOrderProgress | null;
  "fee_usd"?: string | null;
  "collateral_change_usd"?: string | null;
  "position_size_prev"?: string | null;
  "realized_pnl_usd"?: string | null;
  "position_open_seq"?: string | null;
  "position_open_ts_ms"?: string | null;
  "seq"?: string | null;
  "trade_ts_ms"?: string | null;
}

export interface FillHistoryResponseDataItemsItemOrderProgress {
  [field: string]: unknown;
  "size_original"?: string | null;
  "size_filled"?: string | null;
  "size_remaining"?: string | null;
}

export interface GenerateAReferralCodeParams {
  [field: string]: unknown;
  "body": GenerateAReferralCodeParamsBody;
}

export interface GenerateAReferralCodeParamsBody {
  [field: string]: unknown;
  "auth"?: Record<string, HttpValue> | null;
}

export interface GenerateAReferralCodeResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: GenerateAReferralCodeResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface GenerateAReferralCodeResponseData {
  [field: string]: unknown;
  "generated_referral_code"?: string | null;
}

export interface ListMarketsParams {
  [field: string]: unknown;
}

export interface ListMarketsResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<ListMarketsResponseDataItem> | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface ListMarketsResponseDataItem {
  [field: string]: unknown;
  "symbol"?: string | null;
  "taker_fee_rate"?: string | null;
  "maker_rebate_share"?: string | null;
  "tick_size_min"?: string | null;
  "tick_sig_figs"?: number | bigint | string | null;
  "is_mece"?: boolean | null;
  "display_attributes"?: ListMarketsResponseDataItemDisplayAttributes | null;
  "listing_ts_ms"?: string | null;
  "rules_url"?: string | null;
  "stats"?: ListMarketsResponseDataItemStats | null;
  "mark_price"?: string | null;
  "best_bid"?: string | null;
  "best_ask"?: string | null;
  "open_interest"?: string | null;
  "long_interest"?: string | null;
  "short_interest"?: string | null;
}

export interface ListMarketsResponseDataItemDisplayAttributes {
  [field: string]: unknown;
  "event_description"?: string | null;
  "expected_resolution_time_ms"?: string | null;
  "market_description"?: string | null;
  "reverse_description"?: string | null;
  "sort_order"?: string | null;
  "tags"?: Array<string> | null;
  "topic"?: ListMarketsResponseDataItemDisplayAttributesTopic | null;
}

export interface ListMarketsResponseDataItemDisplayAttributesTopic {
  [field: string]: unknown;
  "id"?: string | null;
  "description"?: string | null;
}

export interface ListMarketsResponseDataItemStats {
  [field: string]: unknown;
  "last_24h"?: ListMarketsResponseDataItemStatsLast24h | null;
  "all_time"?: ListMarketsResponseDataItemStatsAllTime | null;
}

export interface ListMarketsResponseDataItemStatsAllTime {
  [field: string]: unknown;
  "taker_buy_volume"?: string | null;
  "taker_sell_volume"?: string | null;
  "taker_buy_volume_notional"?: string | null;
  "taker_sell_volume_notional"?: string | null;
  "taker_buy_trade_count"?: string | null;
  "taker_sell_trade_count"?: string | null;
  "window_end_ts_ms_exclusive"?: string | null;
}

export interface ListMarketsResponseDataItemStatsLast24h {
  [field: string]: unknown;
  "mark_price_start"?: string | null;
  "taker_buy_volume"?: string | null;
  "taker_sell_volume"?: string | null;
  "taker_buy_volume_notional"?: string | null;
  "taker_sell_volume_notional"?: string | null;
  "taker_buy_trade_count"?: string | null;
  "taker_sell_trade_count"?: string | null;
  "window_end_ts_ms_exclusive"?: string | null;
}

export interface MarkPriceCandlesParams {
  [field: string]: unknown;
  "symbol": string;
  "interval": string;
  "end_time_ms": number | bigint | string;
  "start_time_ms"?: number | bigint | string | null;
  "count_back"?: number | bigint | string | null;
}

export interface MarkPriceCandlesResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<MarkPriceCandlesResponseDataItem> | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface MarkPriceCandlesResponseDataItem {
  [field: string]: unknown;
  "time"?: string | null;
  "open"?: string | null;
  "high"?: string | null;
  "low"?: string | null;
  "close"?: string | null;
}

export interface MarketPnLMoversParams {
  [field: string]: unknown;
  "window": string;
  "limit"?: number | bigint | string | null;
  "symbols"?: string | null;
  "order_by"?: string | null;
  "sort_order"?: string | null;
  "owner": string;
}

export interface MarketPnLMoversResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: MarketPnLMoversResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface MarketPnLMoversResponseData {
  [field: string]: unknown;
  "owner"?: string | null;
  "window"?: string | null;
  "start_ms"?: string | null;
  "as_of_ms"?: string | null;
  "total_markets"?: string | null;
  "total_pnl_change_usd"?: string | null;
  "entries"?: Array<MarketPnLMoversResponseDataEntriesItem> | null;
}

export interface MarketPnLMoversResponseDataEntriesItem {
  [field: string]: unknown;
  "market_id"?: string | null;
  "symbol"?: string | null;
  "pnl_change_usd"?: string | null;
  "mark_price"?: string | null;
  "mark_price_start"?: string | null;
  "position_size_start"?: string | null;
  "position_size"?: string | null;
  "volume_contracts"?: string | null;
  "resolved"?: boolean | null;
}

export interface MarketSearchParams {
  [field: string]: unknown;
}

export interface MarketSearchResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<MarketSearchResponseDataItem> | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface MarketSearchResponseDataItem {
  [field: string]: unknown;
  "symbol"?: string | null;
  "taker_fee_rate"?: string | null;
  "maker_rebate_share"?: string | null;
  "tick_size_min"?: string | null;
  "tick_sig_figs"?: number | bigint | string | null;
  "is_mece"?: boolean | null;
  "display_attributes"?: MarketSearchResponseDataItemDisplayAttributes | null;
  "listing_ts_ms"?: string | null;
  "rules_url"?: string | null;
  "stats"?: MarketSearchResponseDataItemStats | null;
  "mark_price"?: string | null;
  "best_bid"?: string | null;
  "best_ask"?: string | null;
  "open_interest"?: string | null;
  "long_interest"?: string | null;
  "short_interest"?: string | null;
}

export interface MarketSearchResponseDataItemDisplayAttributes {
  [field: string]: unknown;
  "event_description"?: string | null;
  "expected_resolution_time_ms"?: string | null;
  "market_description"?: string | null;
  "reverse_description"?: string | null;
  "sort_order"?: string | null;
  "tags"?: Array<string> | null;
  "topic"?: MarketSearchResponseDataItemDisplayAttributesTopic | null;
}

export interface MarketSearchResponseDataItemDisplayAttributesTopic {
  [field: string]: unknown;
  "id"?: string | null;
  "description"?: string | null;
}

export interface MarketSearchResponseDataItemStats {
  [field: string]: unknown;
  "last_24h"?: MarketSearchResponseDataItemStatsLast24h | null;
  "all_time"?: MarketSearchResponseDataItemStatsAllTime | null;
}

export interface MarketSearchResponseDataItemStatsAllTime {
  [field: string]: unknown;
  "taker_buy_volume"?: string | null;
  "taker_sell_volume"?: string | null;
  "taker_buy_volume_notional"?: string | null;
  "taker_sell_volume_notional"?: string | null;
  "taker_buy_trade_count"?: string | null;
  "taker_sell_trade_count"?: string | null;
  "window_end_ts_ms_exclusive"?: string | null;
}

export interface MarketSearchResponseDataItemStatsLast24h {
  [field: string]: unknown;
  "mark_price_start"?: string | null;
  "taker_buy_volume"?: string | null;
  "taker_sell_volume"?: string | null;
  "taker_buy_volume_notional"?: string | null;
  "taker_sell_volume_notional"?: string | null;
  "taker_buy_trade_count"?: string | null;
  "taker_sell_trade_count"?: string | null;
  "window_end_ts_ms_exclusive"?: string | null;
}

export interface MarketTextSearchParams {
  [field: string]: unknown;
}

export interface MarketTextSearchResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<MarketTextSearchResponseDataItem> | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface MarketTextSearchResponseDataItem {
  [field: string]: unknown;
  "symbol"?: string | null;
  "taker_fee_rate"?: string | null;
  "maker_rebate_share"?: string | null;
  "tick_size_min"?: string | null;
  "tick_sig_figs"?: number | bigint | string | null;
  "is_mece"?: boolean | null;
  "display_attributes"?: MarketTextSearchResponseDataItemDisplayAttributes | null;
  "listing_ts_ms"?: string | null;
  "rules_url"?: string | null;
  "stats"?: MarketTextSearchResponseDataItemStats | null;
  "mark_price"?: string | null;
  "best_bid"?: string | null;
  "best_ask"?: string | null;
  "open_interest"?: string | null;
  "long_interest"?: string | null;
  "short_interest"?: string | null;
}

export interface MarketTextSearchResponseDataItemDisplayAttributes {
  [field: string]: unknown;
  "event_description"?: string | null;
  "expected_resolution_time_ms"?: string | null;
  "market_description"?: string | null;
  "reverse_description"?: string | null;
  "sort_order"?: string | null;
  "tags"?: Array<string> | null;
  "topic"?: MarketTextSearchResponseDataItemDisplayAttributesTopic | null;
}

export interface MarketTextSearchResponseDataItemDisplayAttributesTopic {
  [field: string]: unknown;
  "id"?: string | null;
  "description"?: string | null;
}

export interface MarketTextSearchResponseDataItemStats {
  [field: string]: unknown;
  "last_24h"?: MarketTextSearchResponseDataItemStatsLast24h | null;
  "all_time"?: MarketTextSearchResponseDataItemStatsAllTime | null;
}

export interface MarketTextSearchResponseDataItemStatsAllTime {
  [field: string]: unknown;
  "taker_buy_volume"?: string | null;
  "taker_sell_volume"?: string | null;
  "taker_buy_volume_notional"?: string | null;
  "taker_sell_volume_notional"?: string | null;
  "taker_buy_trade_count"?: string | null;
  "taker_sell_trade_count"?: string | null;
  "window_end_ts_ms_exclusive"?: string | null;
}

export interface MarketTextSearchResponseDataItemStatsLast24h {
  [field: string]: unknown;
  "mark_price_start"?: string | null;
  "taker_buy_volume"?: string | null;
  "taker_sell_volume"?: string | null;
  "taker_buy_volume_notional"?: string | null;
  "taker_sell_volume_notional"?: string | null;
  "taker_buy_trade_count"?: string | null;
  "taker_sell_trade_count"?: string | null;
  "window_end_ts_ms_exclusive"?: string | null;
}

export interface MassCancelOrdersParams {
  [field: string]: unknown;
  "body": Array<MassCancelOrdersParamsBodyItem>;
}

export interface MassCancelOrdersParamsBodyItem {
  [field: string]: unknown;
  "symbol": string;
  "auth"?: Record<string, HttpValue> | null;
}

export interface MassCancelOrdersResponseItem {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: MassCancelOrdersResponseItemData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface MassCancelOrdersResponseItemData {
  [field: string]: unknown;
  "orders"?: Array<MassCancelOrdersResponseItemDataOrdersItem> | null;
}

export interface MassCancelOrdersResponseItemDataOrdersItem {
  [field: string]: unknown;
  "id"?: string | null;
  "client_order_id"?: string | null;
  "symbol"?: string | null;
  "side"?: string | null;
  "price"?: string | null;
  "size_original"?: string | null;
  "size_remaining"?: string | null;
  "size_filled"?: string | null;
  "notional_filled"?: string | null;
  "tif"?: string | null;
  "type"?: string | null;
  "post_only"?: boolean | null;
  "reduce_only"?: boolean | null;
  "seq"?: string | null;
  "update_seq"?: string | null;
  "place_ts_ms"?: string | null;
  "update_ts_ms"?: string | null;
}

export interface MassCancelOrdersResponseValueItem {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: MassCancelOrdersResponseValueItemData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface MassCancelOrdersResponseValueItemData {
  [field: string]: unknown;
  "orders"?: Array<MassCancelOrdersResponseValueItemDataOrdersItem> | null;
}

export interface MassCancelOrdersResponseValueItemDataOrdersItem {
  [field: string]: unknown;
  "id"?: string | null;
  "client_order_id"?: string | null;
  "symbol"?: string | null;
  "side"?: string | null;
  "price"?: string | null;
  "size_original"?: string | null;
  "size_remaining"?: string | null;
  "size_filled"?: string | null;
  "notional_filled"?: string | null;
  "tif"?: string | null;
  "type"?: string | null;
  "post_only"?: boolean | null;
  "reduce_only"?: boolean | null;
  "seq"?: string | null;
  "update_seq"?: string | null;
  "place_ts_ms"?: string | null;
  "update_ts_ms"?: string | null;
}

export interface OrderBooksParams {
  [field: string]: unknown;
  "symbols": string;
}

export interface OrderBooksResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: OrderBooksResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface OrderBooksResponseData {
  [field: string]: unknown;
  "books"?: OrderBooksResponseDataBooks | null;
}

export interface OrderBooksResponseDataBooks {
  [field: string]: unknown;
  "SIM_EVENT_1.MARKET_1"?: OrderBooksResponseDataBooksSIMEVENT1MARKET1 | null;
  "SIM_EVENT_1.MARKET_2"?: OrderBooksResponseDataBooksSIMEVENT1MARKET2 | null;
}

export interface OrderBooksResponseDataBooksSIMEVENT1MARKET1 {
  [field: string]: unknown;
  "asks"?: Array<Array<string>> | null;
  "bids"?: Array<Array<string>> | null;
  "spec"?: OrderBooksResponseDataBooksSIMEVENT1MARKET1Spec | null;
}

export interface OrderBooksResponseDataBooksSIMEVENT1MARKET1Spec {
  [field: string]: unknown;
  "symbol"?: string | null;
  "taker_fee_rate"?: string | null;
  "maker_rebate_share"?: string | null;
  "tick_size_min"?: string | null;
  "tick_sig_figs"?: number | bigint | string | null;
}

export interface OrderBooksResponseDataBooksSIMEVENT1MARKET2 {
  [field: string]: unknown;
  "asks"?: Array<Array<string>> | null;
  "bids"?: Array<Array<string>> | null;
}

export interface PlaceAndReplaceOrdersParams {
  [field: string]: unknown;
  "body": Array<PlaceAndReplaceOrdersParamsBodyItem>;
}

export interface PlaceAndReplaceOrdersParamsBodyItem {
  [field: string]: unknown;
  "client_order_id": string;
  "replace_client_order_id"?: string | null;
  "symbol": string;
  "side": string;
  "price": string;
  "size": string;
  "tif"?: string | null;
  "post_only"?: boolean | null;
  "reduce_only"?: boolean | null;
  "allow_missing_replace"?: boolean | null;
  "replace_with_open_size"?: boolean | null;
  "expires_ts_ms"?: string | null;
  "builder_info"?: string | null;
  "auth"?: Record<string, HttpValue> | null;
}

export interface PlaceAndReplaceOrdersResponseItem {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: PlaceAndReplaceOrdersResponseItemData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface PlaceAndReplaceOrdersResponseItemData {
  [field: string]: unknown;
  "order"?: PlaceAndReplaceOrdersResponseItemDataOrder | null;
  "fills"?: Array<PlaceAndReplaceOrdersResponseItemDataFillsItem> | null;
}

export interface PlaceAndReplaceOrdersResponseItemDataFillsItem {
  [field: string]: unknown;
  "trade_id"?: string | null;
  "symbol"?: string | null;
  "side"?: string | null;
  "reduce_only"?: boolean | null;
  "fill_price"?: string | null;
  "fill_size"?: string | null;
  "liquidity"?: string | null;
  "counterparty"?: string | null;
  "order_id"?: string | null;
  "client_order_id"?: string | null;
  "order_progress"?: PlaceAndReplaceOrdersResponseItemDataFillsItemOrderProgress | null;
  "fee_usd"?: string | null;
  "collateral_change_usd"?: string | null;
  "position_size_prev"?: string | null;
  "realized_pnl_usd"?: string | null;
  "seq"?: string | null;
  "trade_ts_ms"?: string | null;
}

export interface PlaceAndReplaceOrdersResponseItemDataFillsItemOrderProgress {
  [field: string]: unknown;
  "size_original"?: string | null;
  "size_filled"?: string | null;
  "size_remaining"?: string | null;
}

export interface PlaceAndReplaceOrdersResponseItemDataOrder {
  [field: string]: unknown;
  "id"?: string | null;
  "client_order_id"?: string | null;
  "symbol"?: string | null;
  "side"?: string | null;
  "price"?: string | null;
  "size_original"?: string | null;
  "size_remaining"?: string | null;
  "size_filled"?: string | null;
  "notional_filled"?: string | null;
  "tif"?: string | null;
  "type"?: string | null;
  "post_only"?: boolean | null;
  "reduce_only"?: boolean | null;
  "seq"?: string | null;
  "update_seq"?: string | null;
  "place_ts_ms"?: string | null;
  "update_ts_ms"?: string | null;
}

export interface PlaceAndReplaceOrdersResponseValueItem {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: PlaceAndReplaceOrdersResponseValueItemData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface PlaceAndReplaceOrdersResponseValueItemData {
  [field: string]: unknown;
  "order"?: PlaceAndReplaceOrdersResponseValueItemDataOrder | null;
  "fills"?: Array<PlaceAndReplaceOrdersResponseValueItemDataFillsItem> | null;
}

export interface PlaceAndReplaceOrdersResponseValueItemDataFillsItem {
  [field: string]: unknown;
  "trade_id"?: string | null;
  "symbol"?: string | null;
  "side"?: string | null;
  "reduce_only"?: boolean | null;
  "fill_price"?: string | null;
  "fill_size"?: string | null;
  "liquidity"?: string | null;
  "counterparty"?: string | null;
  "order_id"?: string | null;
  "client_order_id"?: string | null;
  "order_progress"?: PlaceAndReplaceOrdersResponseValueItemDataFillsItemOrderProgress | null;
  "fee_usd"?: string | null;
  "collateral_change_usd"?: string | null;
  "position_size_prev"?: string | null;
  "realized_pnl_usd"?: string | null;
  "seq"?: string | null;
  "trade_ts_ms"?: string | null;
}

export interface PlaceAndReplaceOrdersResponseValueItemDataFillsItemOrderProgress {
  [field: string]: unknown;
  "size_original"?: string | null;
  "size_filled"?: string | null;
  "size_remaining"?: string | null;
}

export interface PlaceAndReplaceOrdersResponseValueItemDataOrder {
  [field: string]: unknown;
  "id"?: string | null;
  "client_order_id"?: string | null;
  "symbol"?: string | null;
  "side"?: string | null;
  "price"?: string | null;
  "size_original"?: string | null;
  "size_remaining"?: string | null;
  "size_filled"?: string | null;
  "notional_filled"?: string | null;
  "tif"?: string | null;
  "type"?: string | null;
  "post_only"?: boolean | null;
  "reduce_only"?: boolean | null;
  "seq"?: string | null;
  "update_seq"?: string | null;
  "place_ts_ms"?: string | null;
  "update_ts_ms"?: string | null;
}

export interface PnLHistoryParams {
  [field: string]: unknown;
  "owner": string;
}

export interface PnLHistoryResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: PnLHistoryResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface PnLHistoryResponseData {
  [field: string]: unknown;
  "items"?: Array<PnLHistoryResponseDataItemsItem> | null;
}

export interface PnLHistoryResponseDataItemsItem {
  [field: string]: unknown;
  "timeframe"?: string | null;
  "data"?: PnLHistoryResponseDataItemsItemData | null;
}

export interface PnLHistoryResponseDataItemsItemData {
  [field: string]: unknown;
  "account_value_history"?: Array<Array<string>> | null;
  "total_pnl_history"?: Array<Array<string>> | null;
  "trading_pnl_history"?: Array<Array<string>> | null;
  "referral_rewards_received_history"?: Array<Array<string>> | null;
  "builder_fees_received_history"?: Array<Array<string>> | null;
  "user_rewards_received_history"?: Array<Array<string>> | null;
}

export interface PositionResolutionHistoryParams {
  [field: string]: unknown;
  "symbols"?: string | null;
  "before_cursor"?: string | null;
  "at_or_before_seq"?: string | null;
  "limit"?: number | bigint | string | null;
  "owner": string;
}

export interface PositionResolutionHistoryResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: PositionResolutionHistoryResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface PositionResolutionHistoryResponseData {
  [field: string]: unknown;
  "items"?: Array<PositionResolutionHistoryResponseDataItemsItem> | null;
}

export interface PositionResolutionHistoryResponseDataItemsItem {
  [field: string]: unknown;
  "symbol"?: string | null;
  "resolution"?: string | null;
  "size"?: string | null;
  "collateral_change_usd"?: string | null;
  "realized_pnl_usd"?: string | null;
  "position_open_seq"?: string | null;
  "position_open_ts_ms"?: string | null;
  "seq"?: string | null;
  "position_resolution_ts_ms"?: string | null;
}

export interface ReferralCodeParams {
  [field: string]: unknown;
  "referral_code": string;
}

export interface ReferralCodeResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: ReferralCodeResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface ReferralCodeResponseData {
  [field: string]: unknown;
  "exists"?: boolean | null;
  "taker_fee_discount_share"?: string | null;
  "fee_savings_limit_usd"?: string | null;
}

export interface ReferralsParams {
  [field: string]: unknown;
  "owner": string;
}

export interface ReferralsResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: ReferralsResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface ReferralsResponseData {
  [field: string]: unknown;
  "referral_reward_terms"?: ReferralsResponseDataReferralRewardTerms | null;
  "reward_paid_total_usd"?: string | null;
  "referral_count_total"?: number | bigint | string | null;
  "referrals"?: Array<ReferralsResponseDataReferralsItem> | null;
  "referrer"?: ReferralsResponseDataReferrer | null;
  "generated_referral_code"?: string | null;
  "custom_referral_code"?: string | null;
  "custom_referral_code_revoked"?: boolean | null;
  "can_generate_referral_code"?: boolean | null;
  "can_claim_custom_referral_code"?: boolean | null;
  "can_apply_referral_code"?: boolean | null;
}

export interface ReferralsResponseDataReferralRewardTerms {
  [field: string]: unknown;
  "referral_reward_share"?: string | null;
  "per_referred_owner_reward_paid_limit_usd"?: string | null;
}

export interface ReferralsResponseDataReferralsItem {
  [field: string]: unknown;
  "seq"?: string | null;
  "referred_owner"?: string | null;
  "created_ts_ms"?: string | null;
  "referral_reward_share"?: string | null;
  "reward_paid_usd"?: string | null;
  "reward_paid_limit_usd"?: string | null;
}

export interface ReferralsResponseDataReferrer {
  [field: string]: unknown;
  "owner"?: string | null;
  "username"?: string | null;
  "referral_code"?: string | null;
}

export interface RegisterADepositAddressParams {
  [field: string]: unknown;
  "body": RegisterADepositAddressParamsBody;
}

export interface RegisterADepositAddressParamsBody {
  [field: string]: unknown;
  "owner": string;
  "referral_code"?: string | null;
  "auth"?: Record<string, HttpValue> | null;
}

export interface RegisterADepositAddressResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: RegisterADepositAddressResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface RegisterADepositAddressResponseData {
  [field: string]: unknown;
  "owner"?: string | null;
  "deposit_address"?: string | null;
  "registered"?: boolean | null;
}

export interface ResolvedMarketsParams {
  [field: string]: unknown;
  "lookback": string;
  "order_by": string;
  "limit": number | bigint | string;
}

export interface ResolvedMarketsResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<ResolvedMarketsResponseDataItem> | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface ResolvedMarketsResponseDataItem {
  [field: string]: unknown;
  "symbol"?: string | null;
  "taker_fee_rate"?: string | null;
  "maker_rebate_share"?: string | null;
  "tick_size_min"?: string | null;
  "tick_sig_figs"?: number | bigint | string | null;
  "is_mece"?: boolean | null;
  "display_attributes"?: ResolvedMarketsResponseDataItemDisplayAttributes | null;
  "listing_ts_ms"?: string | null;
  "rules_url"?: string | null;
  "stats"?: ResolvedMarketsResponseDataItemStats | null;
  "mark_price"?: string | null;
  "open_interest"?: string | null;
  "long_interest"?: string | null;
  "short_interest"?: string | null;
  "resolution"?: ResolvedMarketsResponseDataItemResolution | null;
}

export interface ResolvedMarketsResponseDataItemDisplayAttributes {
  [field: string]: unknown;
  "event_description"?: string | null;
  "expected_resolution_time_ms"?: string | null;
  "market_description"?: string | null;
  "reverse_description"?: string | null;
  "sort_order"?: string | null;
  "tags"?: Array<string> | null;
  "topic"?: ResolvedMarketsResponseDataItemDisplayAttributesTopic | null;
}

export interface ResolvedMarketsResponseDataItemDisplayAttributesTopic {
  [field: string]: unknown;
  "id"?: string | null;
  "description"?: string | null;
}

export interface ResolvedMarketsResponseDataItemResolution {
  [field: string]: unknown;
  "price"?: string | null;
  "seq"?: string | null;
  "market_resolution_ts_ms"?: string | null;
}

export interface ResolvedMarketsResponseDataItemStats {
  [field: string]: unknown;
  "last_24h"?: ResolvedMarketsResponseDataItemStatsLast24h | null;
  "all_time"?: ResolvedMarketsResponseDataItemStatsAllTime | null;
}

export interface ResolvedMarketsResponseDataItemStatsAllTime {
  [field: string]: unknown;
  "taker_buy_volume"?: string | null;
  "taker_sell_volume"?: string | null;
  "taker_buy_volume_notional"?: string | null;
  "taker_sell_volume_notional"?: string | null;
  "taker_buy_trade_count"?: string | null;
  "taker_sell_trade_count"?: string | null;
  "window_end_ts_ms_exclusive"?: string | null;
}

export interface ResolvedMarketsResponseDataItemStatsLast24h {
  [field: string]: unknown;
  "mark_price_start"?: string | null;
  "taker_buy_volume"?: string | null;
  "taker_sell_volume"?: string | null;
  "taker_buy_volume_notional"?: string | null;
  "taker_sell_volume_notional"?: string | null;
  "taker_buy_trade_count"?: string | null;
  "taker_sell_trade_count"?: string | null;
  "window_end_ts_ms_exclusive"?: string | null;
}

export interface RevokeATradingKeyParams {
  [field: string]: unknown;
  "body": RevokeATradingKeyParamsBody;
}

export interface RevokeATradingKeyParamsBody {
  [field: string]: unknown;
  "trading_key": string;
  "auth"?: Record<string, HttpValue> | null;
}

export interface RevokeATradingKeyResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: RevokeATradingKeyResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface RevokeATradingKeyResponseData {
  [field: string]: unknown;
  "trading_key"?: string | null;
}

export interface ServerTimeParams {
  [field: string]: unknown;
}

export interface ServerTimeResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: ServerTimeResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface ServerTimeResponseData {
  [field: string]: unknown;
  "server_time_ms"?: string | null;
}

export interface SetAUsernameParams {
  [field: string]: unknown;
  "body": SetAUsernameParamsBody;
}

export interface SetAUsernameParamsBody {
  [field: string]: unknown;
  "username": string;
  "auth"?: Record<string, HttpValue> | null;
}

export interface SetAUsernameResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: SetAUsernameResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface SetAUsernameResponseData {
  [field: string]: unknown;
  "username"?: string | null;
}

export interface SetAnAccountBioParams {
  [field: string]: unknown;
  "body": SetAnAccountBioParamsBody;
}

export interface SetAnAccountBioParamsBody {
  [field: string]: unknown;
  "account_bio": string;
  "auth"?: Record<string, HttpValue> | null;
}

export interface SetAnAccountBioResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: SetAnAccountBioResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface SetAnAccountBioResponseData {
  [field: string]: unknown;
  "account_bio"?: string | null;
}

export interface TakerDiscountTermsParams {
  [field: string]: unknown;
  "owner": string;
}

export interface TakerDiscountTermsResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: TakerDiscountTermsResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface TakerDiscountTermsResponseData {
  [field: string]: unknown;
  "taker_discount_terms"?: TakerDiscountTermsResponseDataTakerDiscountTerms | null;
}

export interface TakerDiscountTermsResponseDataTakerDiscountTerms {
  [field: string]: unknown;
  "taker_fee_discount_share"?: string | null;
  "created_ts_ms"?: string | null;
  "fee_savings_limit_usd"?: string | null;
  "fee_savings_usd"?: string | null;
}

export interface TopHoldersParams {
  [field: string]: unknown;
  "symbols": string;
  "limit"?: number | bigint | string | null;
}

export interface TopHoldersResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: TopHoldersResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface TopHoldersResponseData {
  [field: string]: unknown;
  "holders"?: TopHoldersResponseDataHolders | null;
}

export interface TopHoldersResponseDataHolders {
  [field: string]: unknown;
  "SIM_EVENT_1.MARKET_1"?: TopHoldersResponseDataHoldersSIMEVENT1MARKET1 | null;
  "SIM_EVENT_1.MARKET_2"?: TopHoldersResponseDataHoldersSIMEVENT1MARKET2 | null;
}

export interface TopHoldersResponseDataHoldersSIMEVENT1MARKET1 {
  [field: string]: unknown;
  "longs"?: Array<TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItem> | null;
  "shorts"?: Array<HttpValue> | null;
}

export interface TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItem {
  [field: string]: unknown;
  "owner"?: string | null;
  "position"?: TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItemPosition | null;
}

export interface TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItemPosition {
  [field: string]: unknown;
  "symbol"?: string | null;
  "size"?: string | null;
  "cumulative_increasing_contracts"?: string | null;
  "cumulative_increasing_notional_usd"?: string | null;
  "cumulative_reducing_contracts"?: string | null;
  "cumulative_reducing_notional_usd"?: string | null;
  "cumulative_fees_usd"?: string | null;
  "realized_pnl_usd"?: string | null;
  "unrealized_pnl_usd"?: string | null;
  "mark_price"?: string | null;
  "average_entry_price"?: string | null;
  "seq"?: string | null;
  "update_seq"?: string | null;
  "open_ts_ms"?: string | null;
  "update_ts_ms"?: string | null;
}

export interface TopHoldersResponseDataHoldersSIMEVENT1MARKET2 {
  [field: string]: unknown;
  "longs"?: Array<HttpValue> | null;
  "shorts"?: Array<TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItem> | null;
}

export interface TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItem {
  [field: string]: unknown;
  "owner"?: string | null;
  "position"?: TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItemPosition | null;
}

export interface TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItemPosition {
  [field: string]: unknown;
  "symbol"?: string | null;
  "size"?: string | null;
  "cumulative_increasing_contracts"?: string | null;
  "cumulative_increasing_notional_usd"?: string | null;
  "cumulative_reducing_contracts"?: string | null;
  "cumulative_reducing_notional_usd"?: string | null;
  "cumulative_fees_usd"?: string | null;
  "realized_pnl_usd"?: string | null;
  "unrealized_pnl_usd"?: string | null;
  "mark_price"?: string | null;
  "average_entry_price"?: string | null;
  "seq"?: string | null;
  "update_seq"?: string | null;
  "open_ts_ms"?: string | null;
  "update_ts_ms"?: string | null;
}

export interface TradeCandlesParams {
  [field: string]: unknown;
  "symbol": string;
  "interval": string;
  "end_time_ms": number | bigint | string;
  "start_time_ms"?: number | bigint | string | null;
  "count_back"?: number | bigint | string | null;
}

export interface TradeCandlesResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<TradeCandlesResponseDataItem> | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface TradeCandlesResponseDataItem {
  [field: string]: unknown;
  "time"?: string | null;
  "open"?: string | null;
  "high"?: string | null;
  "low"?: string | null;
  "close"?: string | null;
  "volume"?: string | null;
  "taker_buy_volume"?: string | null;
  "taker_sell_volume"?: string | null;
  "volume_notional"?: string | null;
  "taker_buy_volume_notional"?: string | null;
  "taker_sell_volume_notional"?: string | null;
  "trade_count"?: string | null;
  "taker_buy_count"?: string | null;
  "taker_sell_count"?: string | null;
}

export interface TradesParams {
  [field: string]: unknown;
  "symbols": string;
  "before_cursor"?: string | null;
  "at_or_before_seq"?: string | null;
  "limit"?: number | bigint | string | null;
}

export interface TradesResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: TradesResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface TradesResponseData {
  [field: string]: unknown;
  "items"?: Array<TradesResponseDataItemsItem> | null;
  "next_cursor"?: string | null;
}

export interface TradesResponseDataItemsItem {
  [field: string]: unknown;
  "trade_id"?: string | null;
  "symbol"?: string | null;
  "taker_side"?: string | null;
  "price"?: string | null;
  "size"?: string | null;
  "maker"?: string | null;
  "taker"?: string | null;
  "maker_reduce_only"?: boolean | null;
  "taker_reduce_only"?: boolean | null;
  "maker_position_size_prev"?: string | null;
  "taker_position_size_prev"?: string | null;
  "seq"?: string | null;
  "trade_ts_ms"?: string | null;
}

export interface TradingKeyAccountsParams {
  [field: string]: unknown;
  "trading_key_pubkey": string;
}

export interface TradingKeyAccountsResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<string> | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface TradingKeysParams {
  [field: string]: unknown;
  "owner": string;
}

export interface TradingKeysResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: Array<TradingKeysResponseDataItem> | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface TradingKeysResponseDataItem {
  [field: string]: unknown;
  "trading_key"?: string | null;
  "name"?: string | null;
  "expiration_ts_ms"?: string | null;
  "open_order_count"?: string | null;
}

export interface TransferHistoryParams {
  [field: string]: unknown;
  "before_cursor"?: string | null;
  "at_or_before_seq"?: string | null;
  "limit"?: number | bigint | string | null;
  "owner": string;
}

export interface TransferHistoryResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: TransferHistoryResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface TransferHistoryResponseData {
  [field: string]: unknown;
  "items"?: Array<TransferHistoryResponseDataItemsItem> | null;
}

export interface TransferHistoryResponseDataItemsItem {
  [field: string]: unknown;
  "type"?: string | null;
  "status"?: string | null;
  "collateral_change_usd"?: string | null;
  "fee_usd"?: string | null;
  "net_usd"?: string | null;
  "seq"?: string | null;
  "transfer_ts_ms"?: string | null;
}

export interface UsernamesParams {
  [field: string]: unknown;
  "owners": Array<string>;
}

export interface UsernamesResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: UsernamesResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface UsernamesResponseData {
  [field: string]: unknown;
  "usernames"?: UsernamesResponseDataUsernames | null;
}

export interface UsernamesResponseDataUsernames {
  [field: string]: unknown;
  "GmaDrppBC7P5ARKV8g3djiwP89vz1jLK23V2GBjuAEGB"?: string | null;
}

export interface WithdrawCollateralParams {
  [field: string]: unknown;
  "body": WithdrawCollateralParamsBody;
}

export interface WithdrawCollateralParamsBody {
  [field: string]: unknown;
  "amount": string;
  "destination_authority": string;
  "destination_token_account": string;
  "auth"?: Record<string, HttpValue> | null;
}

export interface WithdrawCollateralResponse {
  [field: string]: unknown;
  "status"?: string | null;
  "data"?: WithdrawCollateralResponseData | null;
  "seq"?: string | null;
  "state_time_ms"?: string | null;
  "server_time_ms"?: string | null;
}

export interface WithdrawCollateralResponseData {
  [field: string]: unknown;
  "owner"?: string | null;
  "amount"?: string | null;
  "destination_authority"?: string | null;
  "destination_token_account"?: string | null;
}
