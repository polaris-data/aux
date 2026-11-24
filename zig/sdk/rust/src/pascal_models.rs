// Generated from schema/pascal.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{PascalClient, Error, Result};

pub type CancelOrdersResponse = Vec<CancelOrdersResponseValueItem>;
pub type MassCancelOrdersResponse = Vec<MassCancelOrdersResponseValueItem>;
pub type PlaceAndReplaceOrdersResponse = Vec<PlaceAndReplaceOrdersResponseValueItem>;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountBioParams {
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountBioResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<AccountBioResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountBioResponseData {
    #[serde(rename = "account_bio", default, skip_serializing_if = "Option::is_none")]
    pub account_bio: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountStateSnapshotParams {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "exclude_orders", default, skip_serializing_if = "Option::is_none")]
    pub exclude_orders: Option<bool>,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountStateSnapshotResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<AccountStateSnapshotResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountStateSnapshotResponseData {
    #[serde(rename = "has_deposited", default, skip_serializing_if = "Option::is_none")]
    pub has_deposited: Option<bool>,
    #[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "twitter_account_connection", default, skip_serializing_if = "Option::is_none")]
    pub twitter_account_connection: Option<AccountStateSnapshotResponseDataTwitterAccountConnection>,
    #[serde(rename = "collateral_usd", default, skip_serializing_if = "Option::is_none")]
    pub collateral_usd: Option<String>,
    #[serde(rename = "rewards", default, skip_serializing_if = "Option::is_none")]
    pub rewards: Option<AccountStateSnapshotResponseDataRewards>,
    #[serde(rename = "lifetime_stats", default, skip_serializing_if = "Option::is_none")]
    pub lifetime_stats: Option<AccountStateSnapshotResponseDataLifetimeStats>,
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<AccountStateSnapshotResponseDataOrdersItem>>,
    #[serde(rename = "positions", default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<AccountStateSnapshotResponseDataPositionsItem>>,
    #[serde(rename = "pending_deposits", default, skip_serializing_if = "Option::is_none")]
    pub pending_deposits: Option<Vec<AccountStateSnapshotResponseDataPendingDepositsItem>>,
    #[serde(rename = "pending_withdrawals", default, skip_serializing_if = "Option::is_none")]
    pub pending_withdrawals: Option<Vec<AccountStateSnapshotResponseDataPendingWithdrawalsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountStateSnapshotResponseDataLifetimeStats {
    #[serde(rename = "maker", default, skip_serializing_if = "Option::is_none")]
    pub maker: Option<AccountStateSnapshotResponseDataLifetimeStatsMaker>,
    #[serde(rename = "taker", default, skip_serializing_if = "Option::is_none")]
    pub taker: Option<AccountStateSnapshotResponseDataLifetimeStatsTaker>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountStateSnapshotResponseDataLifetimeStatsMaker {
    #[serde(rename = "volume_contracts", default, skip_serializing_if = "Option::is_none")]
    pub volume_contracts: Option<String>,
    #[serde(rename = "weighted_volume", default, skip_serializing_if = "Option::is_none")]
    pub weighted_volume: Option<String>,
    #[serde(rename = "volume_notional_usd", default, skip_serializing_if = "Option::is_none")]
    pub volume_notional_usd: Option<String>,
    #[serde(rename = "fees_paid_usd", default, skip_serializing_if = "Option::is_none")]
    pub fees_paid_usd: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountStateSnapshotResponseDataLifetimeStatsTaker {
    #[serde(rename = "volume_contracts", default, skip_serializing_if = "Option::is_none")]
    pub volume_contracts: Option<String>,
    #[serde(rename = "weighted_volume", default, skip_serializing_if = "Option::is_none")]
    pub weighted_volume: Option<String>,
    #[serde(rename = "volume_notional_usd", default, skip_serializing_if = "Option::is_none")]
    pub volume_notional_usd: Option<String>,
    #[serde(rename = "fees_paid_usd", default, skip_serializing_if = "Option::is_none")]
    pub fees_paid_usd: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountStateSnapshotResponseDataOrdersItem {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "size_original", default, skip_serializing_if = "Option::is_none")]
    pub size_original: Option<String>,
    #[serde(rename = "size_remaining", default, skip_serializing_if = "Option::is_none")]
    pub size_remaining: Option<String>,
    #[serde(rename = "size_filled", default, skip_serializing_if = "Option::is_none")]
    pub size_filled: Option<String>,
    #[serde(rename = "notional_filled", default, skip_serializing_if = "Option::is_none")]
    pub notional_filled: Option<String>,
    #[serde(rename = "tif", default, skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "update_seq", default, skip_serializing_if = "Option::is_none")]
    pub update_seq: Option<String>,
    #[serde(rename = "place_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub place_ts_ms: Option<String>,
    #[serde(rename = "update_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub update_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountStateSnapshotResponseDataPendingDepositsItem {
    #[serde(rename = "deposit_id", default, skip_serializing_if = "Option::is_none")]
    pub deposit_id: Option<String>,
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "transfer_status", default, skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountStateSnapshotResponseDataPendingWithdrawalsItem {
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "destination_authority", default, skip_serializing_if = "Option::is_none")]
    pub destination_authority: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountStateSnapshotResponseDataPositionsItem {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "cumulative_increasing_contracts", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_increasing_contracts: Option<String>,
    #[serde(rename = "cumulative_increasing_notional_usd", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_increasing_notional_usd: Option<String>,
    #[serde(rename = "cumulative_reducing_contracts", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_reducing_contracts: Option<String>,
    #[serde(rename = "cumulative_reducing_notional_usd", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_reducing_notional_usd: Option<String>,
    #[serde(rename = "cumulative_fees_usd", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_fees_usd: Option<String>,
    #[serde(rename = "realized_pnl_usd", default, skip_serializing_if = "Option::is_none")]
    pub realized_pnl_usd: Option<String>,
    #[serde(rename = "unrealized_pnl_usd", default, skip_serializing_if = "Option::is_none")]
    pub unrealized_pnl_usd: Option<String>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "average_entry_price", default, skip_serializing_if = "Option::is_none")]
    pub average_entry_price: Option<String>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "update_seq", default, skip_serializing_if = "Option::is_none")]
    pub update_seq: Option<String>,
    #[serde(rename = "open_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub open_ts_ms: Option<String>,
    #[serde(rename = "update_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub update_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountStateSnapshotResponseDataRewards {
    #[serde(rename = "referral_rewards_received_usd", default, skip_serializing_if = "Option::is_none")]
    pub referral_rewards_received_usd: Option<String>,
    #[serde(rename = "builder_fees_received_usd", default, skip_serializing_if = "Option::is_none")]
    pub builder_fees_received_usd: Option<String>,
    #[serde(rename = "user_rewards_received_usd", default, skip_serializing_if = "Option::is_none")]
    pub user_rewards_received_usd: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AccountStateSnapshotResponseDataTwitterAccountConnection {
    #[serde(rename = "twitter_user_id", default, skip_serializing_if = "Option::is_none")]
    pub twitter_user_id: Option<String>,
    #[serde(rename = "twitter_handle", default, skip_serializing_if = "Option::is_none")]
    pub twitter_handle: Option<String>,
    #[serde(rename = "connect_seq", default, skip_serializing_if = "Option::is_none")]
    pub connect_seq: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApplyAReferralCodeParams {
    #[serde(rename = "body")]
    pub body: ApplyAReferralCodeParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApplyAReferralCodeParamsBody {
    #[serde(rename = "referral_code")]
    pub referral_code: String,
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ApplyAReferralCodeResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchMarkPriceCandlesParams {
    #[serde(rename = "symbols")]
    pub symbols: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "end_time_ms")]
    pub end_time_ms: serde_json::Number,
    #[serde(rename = "start_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub start_time_ms: Option<serde_json::Number>,
    #[serde(rename = "count_back", default, skip_serializing_if = "Option::is_none")]
    pub count_back: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for BatchMarkPriceCandlesParams {
    fn default() -> Self {
        Self {
            symbols: Default::default(),
            interval: Default::default(),
            end_time_ms: 0.into(),
            start_time_ms: Default::default(),
            count_back: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchMarkPriceCandlesResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchMarkPriceCandlesResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchMarkPriceCandlesResponseData {
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BatchMarkPriceCandlesResponseDataItemsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchMarkPriceCandlesResponseDataItemsItem {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "candles", default, skip_serializing_if = "Option::is_none")]
    pub candles: Option<Vec<BatchMarkPriceCandlesResponseDataItemsItemCandlesItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchMarkPriceCandlesResponseDataItemsItemCandlesItem {
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "open", default, skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,
    #[serde(rename = "high", default, skip_serializing_if = "Option::is_none")]
    pub high: Option<String>,
    #[serde(rename = "low", default, skip_serializing_if = "Option::is_none")]
    pub low: Option<String>,
    #[serde(rename = "close", default, skip_serializing_if = "Option::is_none")]
    pub close: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchTradesParams {
    #[serde(rename = "symbols")]
    pub symbols: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchTradesResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BatchTradesResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchTradesResponseData {
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BatchTradesResponseDataItemsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchTradesResponseDataItemsItem {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "trades", default, skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<BatchTradesResponseDataItemsItemTradesItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BatchTradesResponseDataItemsItemTradesItem {
    #[serde(rename = "trade_id", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "taker_side", default, skip_serializing_if = "Option::is_none")]
    pub taker_side: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "maker", default, skip_serializing_if = "Option::is_none")]
    pub maker: Option<String>,
    #[serde(rename = "taker", default, skip_serializing_if = "Option::is_none")]
    pub taker: Option<String>,
    #[serde(rename = "maker_reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub maker_reduce_only: Option<bool>,
    #[serde(rename = "taker_reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub taker_reduce_only: Option<bool>,
    #[serde(rename = "maker_position_size_prev", default, skip_serializing_if = "Option::is_none")]
    pub maker_position_size_prev: Option<String>,
    #[serde(rename = "taker_position_size_prev", default, skip_serializing_if = "Option::is_none")]
    pub taker_position_size_prev: Option<String>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "trade_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub trade_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuilderStatsParams {
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuilderStatsResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BuilderStatsResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BuilderStatsResponseData {
    #[serde(rename = "builder_volume_contracts", default, skip_serializing_if = "Option::is_none")]
    pub builder_volume_contracts: Option<String>,
    #[serde(rename = "builder_volume_notional_usd", default, skip_serializing_if = "Option::is_none")]
    pub builder_volume_notional_usd: Option<String>,
    #[serde(rename = "builder_fee_base_usd", default, skip_serializing_if = "Option::is_none")]
    pub builder_fee_base_usd: Option<String>,
    #[serde(rename = "builder_fees_received_usd", default, skip_serializing_if = "Option::is_none")]
    pub builder_fees_received_usd: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersParams {
    #[serde(rename = "body")]
    pub body: Vec<CancelOrdersParamsBodyItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersParamsBodyItem {
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersResponseItem {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<CancelOrdersResponseItemData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersResponseItemData {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "size_original", default, skip_serializing_if = "Option::is_none")]
    pub size_original: Option<String>,
    #[serde(rename = "size_remaining", default, skip_serializing_if = "Option::is_none")]
    pub size_remaining: Option<String>,
    #[serde(rename = "size_filled", default, skip_serializing_if = "Option::is_none")]
    pub size_filled: Option<String>,
    #[serde(rename = "notional_filled", default, skip_serializing_if = "Option::is_none")]
    pub notional_filled: Option<String>,
    #[serde(rename = "tif", default, skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "update_seq", default, skip_serializing_if = "Option::is_none")]
    pub update_seq: Option<String>,
    #[serde(rename = "place_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub place_ts_ms: Option<String>,
    #[serde(rename = "update_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub update_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersResponseValueItem {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<CancelOrdersResponseValueItemData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CancelOrdersResponseValueItemData {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "size_original", default, skip_serializing_if = "Option::is_none")]
    pub size_original: Option<String>,
    #[serde(rename = "size_remaining", default, skip_serializing_if = "Option::is_none")]
    pub size_remaining: Option<String>,
    #[serde(rename = "size_filled", default, skip_serializing_if = "Option::is_none")]
    pub size_filled: Option<String>,
    #[serde(rename = "notional_filled", default, skip_serializing_if = "Option::is_none")]
    pub notional_filled: Option<String>,
    #[serde(rename = "tif", default, skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "update_seq", default, skip_serializing_if = "Option::is_none")]
    pub update_seq: Option<String>,
    #[serde(rename = "place_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub place_ts_ms: Option<String>,
    #[serde(rename = "update_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub update_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClaimACustomReferralCodeParams {
    #[serde(rename = "body")]
    pub body: ClaimACustomReferralCodeParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClaimACustomReferralCodeParamsBody {
    #[serde(rename = "referral_code")]
    pub referral_code: String,
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClaimACustomReferralCodeResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ClaimACustomReferralCodeResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClaimACustomReferralCodeResponseData {
    #[serde(rename = "custom_referral_code", default, skip_serializing_if = "Option::is_none")]
    pub custom_referral_code: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClosedPositionHistoryParams {
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(rename = "order_by", default, skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "sort_order", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "before_cursor", default, skip_serializing_if = "Option::is_none")]
    pub before_cursor: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClosedPositionHistoryResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ClosedPositionHistoryResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClosedPositionHistoryResponseData {
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ClosedPositionHistoryResponseDataItemsItem>>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ClosedPositionHistoryResponseDataItemsItem {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "position_side", default, skip_serializing_if = "Option::is_none")]
    pub position_side: Option<String>,
    #[serde(rename = "position_instance_count", default, skip_serializing_if = "Option::is_none")]
    pub position_instance_count: Option<String>,
    #[serde(rename = "held_until_resolution", default, skip_serializing_if = "Option::is_none")]
    pub held_until_resolution: Option<bool>,
    #[serde(rename = "realized_pnl_usd", default, skip_serializing_if = "Option::is_none")]
    pub realized_pnl_usd: Option<String>,
    #[serde(rename = "cumulative_increasing_contracts", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_increasing_contracts: Option<String>,
    #[serde(rename = "cumulative_increasing_notional_usd", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_increasing_notional_usd: Option<String>,
    #[serde(rename = "cumulative_reducing_notional_usd", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_reducing_notional_usd: Option<String>,
    #[serde(rename = "average_increasing_price", default, skip_serializing_if = "Option::is_none")]
    pub average_increasing_price: Option<String>,
    #[serde(rename = "average_reducing_price", default, skip_serializing_if = "Option::is_none")]
    pub average_reducing_price: Option<String>,
    #[serde(rename = "open_seq", default, skip_serializing_if = "Option::is_none")]
    pub open_seq: Option<String>,
    #[serde(rename = "open_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub open_ts_ms: Option<String>,
    #[serde(rename = "close_seq", default, skip_serializing_if = "Option::is_none")]
    pub close_seq: Option<String>,
    #[serde(rename = "close_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub close_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateATradingKeyParams {
    #[serde(rename = "body")]
    pub body: CreateATradingKeyParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateATradingKeyParamsBody {
    #[serde(rename = "trading_key")]
    pub trading_key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "expiration_ts_ms")]
    pub expiration_ts_ms: String,
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateATradingKeyResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateATradingKeyResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CreateATradingKeyResponseData {
    #[serde(rename = "trading_key", default, skip_serializing_if = "Option::is_none")]
    pub trading_key: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "expiration_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub expiration_ts_ms: Option<String>,
    #[serde(rename = "open_order_count", default, skip_serializing_if = "Option::is_none")]
    pub open_order_count: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DepositAddressParams {
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DepositAddressResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<DepositAddressResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DepositAddressResponseData {
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "deposit_address", default, skip_serializing_if = "Option::is_none")]
    pub deposit_address: Option<String>,
    #[serde(rename = "registered", default, skip_serializing_if = "Option::is_none")]
    pub registered: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillHistoryParams {
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(rename = "before_cursor", default, skip_serializing_if = "Option::is_none")]
    pub before_cursor: Option<String>,
    #[serde(rename = "at_or_before_seq", default, skip_serializing_if = "Option::is_none")]
    pub at_or_before_seq: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillHistoryResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<FillHistoryResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillHistoryResponseData {
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<FillHistoryResponseDataItemsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillHistoryResponseDataItemsItem {
    #[serde(rename = "trade_id", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "fill_price", default, skip_serializing_if = "Option::is_none")]
    pub fill_price: Option<String>,
    #[serde(rename = "fill_size", default, skip_serializing_if = "Option::is_none")]
    pub fill_size: Option<String>,
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    #[serde(rename = "counterparty", default, skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<String>,
    #[serde(rename = "order_id", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "order_progress", default, skip_serializing_if = "Option::is_none")]
    pub order_progress: Option<FillHistoryResponseDataItemsItemOrderProgress>,
    #[serde(rename = "fee_usd", default, skip_serializing_if = "Option::is_none")]
    pub fee_usd: Option<String>,
    #[serde(rename = "collateral_change_usd", default, skip_serializing_if = "Option::is_none")]
    pub collateral_change_usd: Option<String>,
    #[serde(rename = "position_size_prev", default, skip_serializing_if = "Option::is_none")]
    pub position_size_prev: Option<String>,
    #[serde(rename = "realized_pnl_usd", default, skip_serializing_if = "Option::is_none")]
    pub realized_pnl_usd: Option<String>,
    #[serde(rename = "position_open_seq", default, skip_serializing_if = "Option::is_none")]
    pub position_open_seq: Option<String>,
    #[serde(rename = "position_open_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub position_open_ts_ms: Option<String>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "trade_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub trade_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FillHistoryResponseDataItemsItemOrderProgress {
    #[serde(rename = "size_original", default, skip_serializing_if = "Option::is_none")]
    pub size_original: Option<String>,
    #[serde(rename = "size_filled", default, skip_serializing_if = "Option::is_none")]
    pub size_filled: Option<String>,
    #[serde(rename = "size_remaining", default, skip_serializing_if = "Option::is_none")]
    pub size_remaining: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GenerateAReferralCodeParams {
    #[serde(rename = "body")]
    pub body: GenerateAReferralCodeParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GenerateAReferralCodeParamsBody {
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GenerateAReferralCodeResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<GenerateAReferralCodeResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GenerateAReferralCodeResponseData {
    #[serde(rename = "generated_referral_code", default, skip_serializing_if = "Option::is_none")]
    pub generated_referral_code: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListMarketsParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListMarketsResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ListMarketsResponseDataItem>>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListMarketsResponseDataItem {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "taker_fee_rate", default, skip_serializing_if = "Option::is_none")]
    pub taker_fee_rate: Option<String>,
    #[serde(rename = "maker_rebate_share", default, skip_serializing_if = "Option::is_none")]
    pub maker_rebate_share: Option<String>,
    #[serde(rename = "tick_size_min", default, skip_serializing_if = "Option::is_none")]
    pub tick_size_min: Option<String>,
    #[serde(rename = "tick_sig_figs", default, skip_serializing_if = "Option::is_none")]
    pub tick_sig_figs: Option<serde_json::Number>,
    #[serde(rename = "is_mece", default, skip_serializing_if = "Option::is_none")]
    pub is_mece: Option<bool>,
    #[serde(rename = "display_attributes", default, skip_serializing_if = "Option::is_none")]
    pub display_attributes: Option<ListMarketsResponseDataItemDisplayAttributes>,
    #[serde(rename = "listing_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub listing_ts_ms: Option<String>,
    #[serde(rename = "rules_url", default, skip_serializing_if = "Option::is_none")]
    pub rules_url: Option<String>,
    #[serde(rename = "stats", default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<ListMarketsResponseDataItemStats>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "best_bid", default, skip_serializing_if = "Option::is_none")]
    pub best_bid: Option<String>,
    #[serde(rename = "best_ask", default, skip_serializing_if = "Option::is_none")]
    pub best_ask: Option<String>,
    #[serde(rename = "open_interest", default, skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<String>,
    #[serde(rename = "long_interest", default, skip_serializing_if = "Option::is_none")]
    pub long_interest: Option<String>,
    #[serde(rename = "short_interest", default, skip_serializing_if = "Option::is_none")]
    pub short_interest: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListMarketsResponseDataItemDisplayAttributes {
    #[serde(rename = "event_description", default, skip_serializing_if = "Option::is_none")]
    pub event_description: Option<String>,
    #[serde(rename = "expected_resolution_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub expected_resolution_time_ms: Option<String>,
    #[serde(rename = "market_description", default, skip_serializing_if = "Option::is_none")]
    pub market_description: Option<String>,
    #[serde(rename = "reverse_description", default, skip_serializing_if = "Option::is_none")]
    pub reverse_description: Option<String>,
    #[serde(rename = "sort_order", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "topic", default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<ListMarketsResponseDataItemDisplayAttributesTopic>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListMarketsResponseDataItemDisplayAttributesTopic {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListMarketsResponseDataItemStats {
    #[serde(rename = "last_24h", default, skip_serializing_if = "Option::is_none")]
    pub last_24h: Option<ListMarketsResponseDataItemStatsLast24h>,
    #[serde(rename = "all_time", default, skip_serializing_if = "Option::is_none")]
    pub all_time: Option<ListMarketsResponseDataItemStatsAllTime>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListMarketsResponseDataItemStatsAllTime {
    #[serde(rename = "taker_buy_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume: Option<String>,
    #[serde(rename = "taker_sell_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume: Option<String>,
    #[serde(rename = "taker_buy_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume_notional: Option<String>,
    #[serde(rename = "taker_sell_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume_notional: Option<String>,
    #[serde(rename = "taker_buy_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_trade_count: Option<String>,
    #[serde(rename = "taker_sell_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_trade_count: Option<String>,
    #[serde(rename = "window_end_ts_ms_exclusive", default, skip_serializing_if = "Option::is_none")]
    pub window_end_ts_ms_exclusive: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ListMarketsResponseDataItemStatsLast24h {
    #[serde(rename = "mark_price_start", default, skip_serializing_if = "Option::is_none")]
    pub mark_price_start: Option<String>,
    #[serde(rename = "taker_buy_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume: Option<String>,
    #[serde(rename = "taker_sell_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume: Option<String>,
    #[serde(rename = "taker_buy_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume_notional: Option<String>,
    #[serde(rename = "taker_sell_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume_notional: Option<String>,
    #[serde(rename = "taker_buy_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_trade_count: Option<String>,
    #[serde(rename = "taker_sell_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_trade_count: Option<String>,
    #[serde(rename = "window_end_ts_ms_exclusive", default, skip_serializing_if = "Option::is_none")]
    pub window_end_ts_ms_exclusive: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MarkPriceCandlesParams {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "end_time_ms")]
    pub end_time_ms: serde_json::Number,
    #[serde(rename = "start_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub start_time_ms: Option<serde_json::Number>,
    #[serde(rename = "count_back", default, skip_serializing_if = "Option::is_none")]
    pub count_back: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for MarkPriceCandlesParams {
    fn default() -> Self {
        Self {
            symbol: Default::default(),
            interval: Default::default(),
            end_time_ms: 0.into(),
            start_time_ms: Default::default(),
            count_back: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarkPriceCandlesResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MarkPriceCandlesResponseDataItem>>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarkPriceCandlesResponseDataItem {
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "open", default, skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,
    #[serde(rename = "high", default, skip_serializing_if = "Option::is_none")]
    pub high: Option<String>,
    #[serde(rename = "low", default, skip_serializing_if = "Option::is_none")]
    pub low: Option<String>,
    #[serde(rename = "close", default, skip_serializing_if = "Option::is_none")]
    pub close: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketPnLMoversParams {
    #[serde(rename = "window")]
    pub window: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(rename = "order_by", default, skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(rename = "sort_order", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketPnLMoversResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<MarketPnLMoversResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketPnLMoversResponseData {
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "window", default, skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
    #[serde(rename = "start_ms", default, skip_serializing_if = "Option::is_none")]
    pub start_ms: Option<String>,
    #[serde(rename = "as_of_ms", default, skip_serializing_if = "Option::is_none")]
    pub as_of_ms: Option<String>,
    #[serde(rename = "total_markets", default, skip_serializing_if = "Option::is_none")]
    pub total_markets: Option<String>,
    #[serde(rename = "total_pnl_change_usd", default, skip_serializing_if = "Option::is_none")]
    pub total_pnl_change_usd: Option<String>,
    #[serde(rename = "entries", default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<MarketPnLMoversResponseDataEntriesItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketPnLMoversResponseDataEntriesItem {
    #[serde(rename = "market_id", default, skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "pnl_change_usd", default, skip_serializing_if = "Option::is_none")]
    pub pnl_change_usd: Option<String>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "mark_price_start", default, skip_serializing_if = "Option::is_none")]
    pub mark_price_start: Option<String>,
    #[serde(rename = "position_size_start", default, skip_serializing_if = "Option::is_none")]
    pub position_size_start: Option<String>,
    #[serde(rename = "position_size", default, skip_serializing_if = "Option::is_none")]
    pub position_size: Option<String>,
    #[serde(rename = "volume_contracts", default, skip_serializing_if = "Option::is_none")]
    pub volume_contracts: Option<String>,
    #[serde(rename = "resolved", default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSearchParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSearchResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MarketSearchResponseDataItem>>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSearchResponseDataItem {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "taker_fee_rate", default, skip_serializing_if = "Option::is_none")]
    pub taker_fee_rate: Option<String>,
    #[serde(rename = "maker_rebate_share", default, skip_serializing_if = "Option::is_none")]
    pub maker_rebate_share: Option<String>,
    #[serde(rename = "tick_size_min", default, skip_serializing_if = "Option::is_none")]
    pub tick_size_min: Option<String>,
    #[serde(rename = "tick_sig_figs", default, skip_serializing_if = "Option::is_none")]
    pub tick_sig_figs: Option<serde_json::Number>,
    #[serde(rename = "is_mece", default, skip_serializing_if = "Option::is_none")]
    pub is_mece: Option<bool>,
    #[serde(rename = "display_attributes", default, skip_serializing_if = "Option::is_none")]
    pub display_attributes: Option<MarketSearchResponseDataItemDisplayAttributes>,
    #[serde(rename = "listing_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub listing_ts_ms: Option<String>,
    #[serde(rename = "rules_url", default, skip_serializing_if = "Option::is_none")]
    pub rules_url: Option<String>,
    #[serde(rename = "stats", default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<MarketSearchResponseDataItemStats>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "best_bid", default, skip_serializing_if = "Option::is_none")]
    pub best_bid: Option<String>,
    #[serde(rename = "best_ask", default, skip_serializing_if = "Option::is_none")]
    pub best_ask: Option<String>,
    #[serde(rename = "open_interest", default, skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<String>,
    #[serde(rename = "long_interest", default, skip_serializing_if = "Option::is_none")]
    pub long_interest: Option<String>,
    #[serde(rename = "short_interest", default, skip_serializing_if = "Option::is_none")]
    pub short_interest: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSearchResponseDataItemDisplayAttributes {
    #[serde(rename = "event_description", default, skip_serializing_if = "Option::is_none")]
    pub event_description: Option<String>,
    #[serde(rename = "expected_resolution_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub expected_resolution_time_ms: Option<String>,
    #[serde(rename = "market_description", default, skip_serializing_if = "Option::is_none")]
    pub market_description: Option<String>,
    #[serde(rename = "reverse_description", default, skip_serializing_if = "Option::is_none")]
    pub reverse_description: Option<String>,
    #[serde(rename = "sort_order", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "topic", default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<MarketSearchResponseDataItemDisplayAttributesTopic>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSearchResponseDataItemDisplayAttributesTopic {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSearchResponseDataItemStats {
    #[serde(rename = "last_24h", default, skip_serializing_if = "Option::is_none")]
    pub last_24h: Option<MarketSearchResponseDataItemStatsLast24h>,
    #[serde(rename = "all_time", default, skip_serializing_if = "Option::is_none")]
    pub all_time: Option<MarketSearchResponseDataItemStatsAllTime>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSearchResponseDataItemStatsAllTime {
    #[serde(rename = "taker_buy_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume: Option<String>,
    #[serde(rename = "taker_sell_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume: Option<String>,
    #[serde(rename = "taker_buy_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume_notional: Option<String>,
    #[serde(rename = "taker_sell_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume_notional: Option<String>,
    #[serde(rename = "taker_buy_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_trade_count: Option<String>,
    #[serde(rename = "taker_sell_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_trade_count: Option<String>,
    #[serde(rename = "window_end_ts_ms_exclusive", default, skip_serializing_if = "Option::is_none")]
    pub window_end_ts_ms_exclusive: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSearchResponseDataItemStatsLast24h {
    #[serde(rename = "mark_price_start", default, skip_serializing_if = "Option::is_none")]
    pub mark_price_start: Option<String>,
    #[serde(rename = "taker_buy_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume: Option<String>,
    #[serde(rename = "taker_sell_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume: Option<String>,
    #[serde(rename = "taker_buy_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume_notional: Option<String>,
    #[serde(rename = "taker_sell_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume_notional: Option<String>,
    #[serde(rename = "taker_buy_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_trade_count: Option<String>,
    #[serde(rename = "taker_sell_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_trade_count: Option<String>,
    #[serde(rename = "window_end_ts_ms_exclusive", default, skip_serializing_if = "Option::is_none")]
    pub window_end_ts_ms_exclusive: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketTextSearchParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketTextSearchResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MarketTextSearchResponseDataItem>>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketTextSearchResponseDataItem {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "taker_fee_rate", default, skip_serializing_if = "Option::is_none")]
    pub taker_fee_rate: Option<String>,
    #[serde(rename = "maker_rebate_share", default, skip_serializing_if = "Option::is_none")]
    pub maker_rebate_share: Option<String>,
    #[serde(rename = "tick_size_min", default, skip_serializing_if = "Option::is_none")]
    pub tick_size_min: Option<String>,
    #[serde(rename = "tick_sig_figs", default, skip_serializing_if = "Option::is_none")]
    pub tick_sig_figs: Option<serde_json::Number>,
    #[serde(rename = "is_mece", default, skip_serializing_if = "Option::is_none")]
    pub is_mece: Option<bool>,
    #[serde(rename = "display_attributes", default, skip_serializing_if = "Option::is_none")]
    pub display_attributes: Option<MarketTextSearchResponseDataItemDisplayAttributes>,
    #[serde(rename = "listing_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub listing_ts_ms: Option<String>,
    #[serde(rename = "rules_url", default, skip_serializing_if = "Option::is_none")]
    pub rules_url: Option<String>,
    #[serde(rename = "stats", default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<MarketTextSearchResponseDataItemStats>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "best_bid", default, skip_serializing_if = "Option::is_none")]
    pub best_bid: Option<String>,
    #[serde(rename = "best_ask", default, skip_serializing_if = "Option::is_none")]
    pub best_ask: Option<String>,
    #[serde(rename = "open_interest", default, skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<String>,
    #[serde(rename = "long_interest", default, skip_serializing_if = "Option::is_none")]
    pub long_interest: Option<String>,
    #[serde(rename = "short_interest", default, skip_serializing_if = "Option::is_none")]
    pub short_interest: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketTextSearchResponseDataItemDisplayAttributes {
    #[serde(rename = "event_description", default, skip_serializing_if = "Option::is_none")]
    pub event_description: Option<String>,
    #[serde(rename = "expected_resolution_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub expected_resolution_time_ms: Option<String>,
    #[serde(rename = "market_description", default, skip_serializing_if = "Option::is_none")]
    pub market_description: Option<String>,
    #[serde(rename = "reverse_description", default, skip_serializing_if = "Option::is_none")]
    pub reverse_description: Option<String>,
    #[serde(rename = "sort_order", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "topic", default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<MarketTextSearchResponseDataItemDisplayAttributesTopic>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketTextSearchResponseDataItemDisplayAttributesTopic {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketTextSearchResponseDataItemStats {
    #[serde(rename = "last_24h", default, skip_serializing_if = "Option::is_none")]
    pub last_24h: Option<MarketTextSearchResponseDataItemStatsLast24h>,
    #[serde(rename = "all_time", default, skip_serializing_if = "Option::is_none")]
    pub all_time: Option<MarketTextSearchResponseDataItemStatsAllTime>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketTextSearchResponseDataItemStatsAllTime {
    #[serde(rename = "taker_buy_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume: Option<String>,
    #[serde(rename = "taker_sell_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume: Option<String>,
    #[serde(rename = "taker_buy_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume_notional: Option<String>,
    #[serde(rename = "taker_sell_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume_notional: Option<String>,
    #[serde(rename = "taker_buy_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_trade_count: Option<String>,
    #[serde(rename = "taker_sell_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_trade_count: Option<String>,
    #[serde(rename = "window_end_ts_ms_exclusive", default, skip_serializing_if = "Option::is_none")]
    pub window_end_ts_ms_exclusive: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketTextSearchResponseDataItemStatsLast24h {
    #[serde(rename = "mark_price_start", default, skip_serializing_if = "Option::is_none")]
    pub mark_price_start: Option<String>,
    #[serde(rename = "taker_buy_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume: Option<String>,
    #[serde(rename = "taker_sell_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume: Option<String>,
    #[serde(rename = "taker_buy_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume_notional: Option<String>,
    #[serde(rename = "taker_sell_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume_notional: Option<String>,
    #[serde(rename = "taker_buy_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_trade_count: Option<String>,
    #[serde(rename = "taker_sell_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_trade_count: Option<String>,
    #[serde(rename = "window_end_ts_ms_exclusive", default, skip_serializing_if = "Option::is_none")]
    pub window_end_ts_ms_exclusive: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MassCancelOrdersParams {
    #[serde(rename = "body")]
    pub body: Vec<MassCancelOrdersParamsBodyItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MassCancelOrdersParamsBodyItem {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MassCancelOrdersResponseItem {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<MassCancelOrdersResponseItemData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MassCancelOrdersResponseItemData {
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<MassCancelOrdersResponseItemDataOrdersItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MassCancelOrdersResponseItemDataOrdersItem {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "size_original", default, skip_serializing_if = "Option::is_none")]
    pub size_original: Option<String>,
    #[serde(rename = "size_remaining", default, skip_serializing_if = "Option::is_none")]
    pub size_remaining: Option<String>,
    #[serde(rename = "size_filled", default, skip_serializing_if = "Option::is_none")]
    pub size_filled: Option<String>,
    #[serde(rename = "notional_filled", default, skip_serializing_if = "Option::is_none")]
    pub notional_filled: Option<String>,
    #[serde(rename = "tif", default, skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "update_seq", default, skip_serializing_if = "Option::is_none")]
    pub update_seq: Option<String>,
    #[serde(rename = "place_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub place_ts_ms: Option<String>,
    #[serde(rename = "update_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub update_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MassCancelOrdersResponseValueItem {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<MassCancelOrdersResponseValueItemData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MassCancelOrdersResponseValueItemData {
    #[serde(rename = "orders", default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<MassCancelOrdersResponseValueItemDataOrdersItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MassCancelOrdersResponseValueItemDataOrdersItem {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "size_original", default, skip_serializing_if = "Option::is_none")]
    pub size_original: Option<String>,
    #[serde(rename = "size_remaining", default, skip_serializing_if = "Option::is_none")]
    pub size_remaining: Option<String>,
    #[serde(rename = "size_filled", default, skip_serializing_if = "Option::is_none")]
    pub size_filled: Option<String>,
    #[serde(rename = "notional_filled", default, skip_serializing_if = "Option::is_none")]
    pub notional_filled: Option<String>,
    #[serde(rename = "tif", default, skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "update_seq", default, skip_serializing_if = "Option::is_none")]
    pub update_seq: Option<String>,
    #[serde(rename = "place_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub place_ts_ms: Option<String>,
    #[serde(rename = "update_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub update_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBooksParams {
    #[serde(rename = "symbols")]
    pub symbols: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBooksResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<OrderBooksResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBooksResponseData {
    #[serde(rename = "books", default, skip_serializing_if = "Option::is_none")]
    pub books: Option<OrderBooksResponseDataBooks>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBooksResponseDataBooks {
    #[serde(rename = "SIM_EVENT_1.MARKET_1", default, skip_serializing_if = "Option::is_none")]
    pub sim_event_1_market_1: Option<OrderBooksResponseDataBooksSIMEVENT1MARKET1>,
    #[serde(rename = "SIM_EVENT_1.MARKET_2", default, skip_serializing_if = "Option::is_none")]
    pub sim_event_1_market_2: Option<OrderBooksResponseDataBooksSIMEVENT1MARKET2>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBooksResponseDataBooksSIMEVENT1MARKET1 {
    #[serde(rename = "asks", default, skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<Vec<String>>>,
    #[serde(rename = "bids", default, skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    #[serde(rename = "spec", default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<OrderBooksResponseDataBooksSIMEVENT1MARKET1Spec>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBooksResponseDataBooksSIMEVENT1MARKET1Spec {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "taker_fee_rate", default, skip_serializing_if = "Option::is_none")]
    pub taker_fee_rate: Option<String>,
    #[serde(rename = "maker_rebate_share", default, skip_serializing_if = "Option::is_none")]
    pub maker_rebate_share: Option<String>,
    #[serde(rename = "tick_size_min", default, skip_serializing_if = "Option::is_none")]
    pub tick_size_min: Option<String>,
    #[serde(rename = "tick_sig_figs", default, skip_serializing_if = "Option::is_none")]
    pub tick_sig_figs: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct OrderBooksResponseDataBooksSIMEVENT1MARKET2 {
    #[serde(rename = "asks", default, skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<Vec<String>>>,
    #[serde(rename = "bids", default, skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAndReplaceOrdersParams {
    #[serde(rename = "body")]
    pub body: Vec<PlaceAndReplaceOrdersParamsBodyItem>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAndReplaceOrdersParamsBodyItem {
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    #[serde(rename = "replace_client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub replace_client_order_id: Option<String>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "size")]
    pub size: String,
    #[serde(rename = "tif", default, skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "allow_missing_replace", default, skip_serializing_if = "Option::is_none")]
    pub allow_missing_replace: Option<bool>,
    #[serde(rename = "replace_with_open_size", default, skip_serializing_if = "Option::is_none")]
    pub replace_with_open_size: Option<bool>,
    #[serde(rename = "expires_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub expires_ts_ms: Option<String>,
    #[serde(rename = "builder_info", default, skip_serializing_if = "Option::is_none")]
    pub builder_info: Option<String>,
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAndReplaceOrdersResponseItem {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PlaceAndReplaceOrdersResponseItemData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAndReplaceOrdersResponseItemData {
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<PlaceAndReplaceOrdersResponseItemDataOrder>,
    #[serde(rename = "fills", default, skip_serializing_if = "Option::is_none")]
    pub fills: Option<Vec<PlaceAndReplaceOrdersResponseItemDataFillsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAndReplaceOrdersResponseItemDataFillsItem {
    #[serde(rename = "trade_id", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "fill_price", default, skip_serializing_if = "Option::is_none")]
    pub fill_price: Option<String>,
    #[serde(rename = "fill_size", default, skip_serializing_if = "Option::is_none")]
    pub fill_size: Option<String>,
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    #[serde(rename = "counterparty", default, skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<String>,
    #[serde(rename = "order_id", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "order_progress", default, skip_serializing_if = "Option::is_none")]
    pub order_progress: Option<PlaceAndReplaceOrdersResponseItemDataFillsItemOrderProgress>,
    #[serde(rename = "fee_usd", default, skip_serializing_if = "Option::is_none")]
    pub fee_usd: Option<String>,
    #[serde(rename = "collateral_change_usd", default, skip_serializing_if = "Option::is_none")]
    pub collateral_change_usd: Option<String>,
    #[serde(rename = "position_size_prev", default, skip_serializing_if = "Option::is_none")]
    pub position_size_prev: Option<String>,
    #[serde(rename = "realized_pnl_usd", default, skip_serializing_if = "Option::is_none")]
    pub realized_pnl_usd: Option<String>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "trade_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub trade_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAndReplaceOrdersResponseItemDataFillsItemOrderProgress {
    #[serde(rename = "size_original", default, skip_serializing_if = "Option::is_none")]
    pub size_original: Option<String>,
    #[serde(rename = "size_filled", default, skip_serializing_if = "Option::is_none")]
    pub size_filled: Option<String>,
    #[serde(rename = "size_remaining", default, skip_serializing_if = "Option::is_none")]
    pub size_remaining: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAndReplaceOrdersResponseItemDataOrder {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "size_original", default, skip_serializing_if = "Option::is_none")]
    pub size_original: Option<String>,
    #[serde(rename = "size_remaining", default, skip_serializing_if = "Option::is_none")]
    pub size_remaining: Option<String>,
    #[serde(rename = "size_filled", default, skip_serializing_if = "Option::is_none")]
    pub size_filled: Option<String>,
    #[serde(rename = "notional_filled", default, skip_serializing_if = "Option::is_none")]
    pub notional_filled: Option<String>,
    #[serde(rename = "tif", default, skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "update_seq", default, skip_serializing_if = "Option::is_none")]
    pub update_seq: Option<String>,
    #[serde(rename = "place_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub place_ts_ms: Option<String>,
    #[serde(rename = "update_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub update_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAndReplaceOrdersResponseValueItem {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PlaceAndReplaceOrdersResponseValueItemData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAndReplaceOrdersResponseValueItemData {
    #[serde(rename = "order", default, skip_serializing_if = "Option::is_none")]
    pub order: Option<PlaceAndReplaceOrdersResponseValueItemDataOrder>,
    #[serde(rename = "fills", default, skip_serializing_if = "Option::is_none")]
    pub fills: Option<Vec<PlaceAndReplaceOrdersResponseValueItemDataFillsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAndReplaceOrdersResponseValueItemDataFillsItem {
    #[serde(rename = "trade_id", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "fill_price", default, skip_serializing_if = "Option::is_none")]
    pub fill_price: Option<String>,
    #[serde(rename = "fill_size", default, skip_serializing_if = "Option::is_none")]
    pub fill_size: Option<String>,
    #[serde(rename = "liquidity", default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    #[serde(rename = "counterparty", default, skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<String>,
    #[serde(rename = "order_id", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "order_progress", default, skip_serializing_if = "Option::is_none")]
    pub order_progress: Option<PlaceAndReplaceOrdersResponseValueItemDataFillsItemOrderProgress>,
    #[serde(rename = "fee_usd", default, skip_serializing_if = "Option::is_none")]
    pub fee_usd: Option<String>,
    #[serde(rename = "collateral_change_usd", default, skip_serializing_if = "Option::is_none")]
    pub collateral_change_usd: Option<String>,
    #[serde(rename = "position_size_prev", default, skip_serializing_if = "Option::is_none")]
    pub position_size_prev: Option<String>,
    #[serde(rename = "realized_pnl_usd", default, skip_serializing_if = "Option::is_none")]
    pub realized_pnl_usd: Option<String>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "trade_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub trade_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAndReplaceOrdersResponseValueItemDataFillsItemOrderProgress {
    #[serde(rename = "size_original", default, skip_serializing_if = "Option::is_none")]
    pub size_original: Option<String>,
    #[serde(rename = "size_filled", default, skip_serializing_if = "Option::is_none")]
    pub size_filled: Option<String>,
    #[serde(rename = "size_remaining", default, skip_serializing_if = "Option::is_none")]
    pub size_remaining: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PlaceAndReplaceOrdersResponseValueItemDataOrder {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "client_order_id", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "side", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "size_original", default, skip_serializing_if = "Option::is_none")]
    pub size_original: Option<String>,
    #[serde(rename = "size_remaining", default, skip_serializing_if = "Option::is_none")]
    pub size_remaining: Option<String>,
    #[serde(rename = "size_filled", default, skip_serializing_if = "Option::is_none")]
    pub size_filled: Option<String>,
    #[serde(rename = "notional_filled", default, skip_serializing_if = "Option::is_none")]
    pub notional_filled: Option<String>,
    #[serde(rename = "tif", default, skip_serializing_if = "Option::is_none")]
    pub tif: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "post_only", default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "update_seq", default, skip_serializing_if = "Option::is_none")]
    pub update_seq: Option<String>,
    #[serde(rename = "place_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub place_ts_ms: Option<String>,
    #[serde(rename = "update_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub update_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PnLHistoryParams {
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PnLHistoryResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PnLHistoryResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PnLHistoryResponseData {
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PnLHistoryResponseDataItemsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PnLHistoryResponseDataItemsItem {
    #[serde(rename = "timeframe", default, skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PnLHistoryResponseDataItemsItemData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PnLHistoryResponseDataItemsItemData {
    #[serde(rename = "account_value_history", default, skip_serializing_if = "Option::is_none")]
    pub account_value_history: Option<Vec<Vec<String>>>,
    #[serde(rename = "total_pnl_history", default, skip_serializing_if = "Option::is_none")]
    pub total_pnl_history: Option<Vec<Vec<String>>>,
    #[serde(rename = "trading_pnl_history", default, skip_serializing_if = "Option::is_none")]
    pub trading_pnl_history: Option<Vec<Vec<String>>>,
    #[serde(rename = "referral_rewards_received_history", default, skip_serializing_if = "Option::is_none")]
    pub referral_rewards_received_history: Option<Vec<Vec<String>>>,
    #[serde(rename = "builder_fees_received_history", default, skip_serializing_if = "Option::is_none")]
    pub builder_fees_received_history: Option<Vec<Vec<String>>>,
    #[serde(rename = "user_rewards_received_history", default, skip_serializing_if = "Option::is_none")]
    pub user_rewards_received_history: Option<Vec<Vec<String>>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionResolutionHistoryParams {
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(rename = "before_cursor", default, skip_serializing_if = "Option::is_none")]
    pub before_cursor: Option<String>,
    #[serde(rename = "at_or_before_seq", default, skip_serializing_if = "Option::is_none")]
    pub at_or_before_seq: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionResolutionHistoryResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PositionResolutionHistoryResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionResolutionHistoryResponseData {
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PositionResolutionHistoryResponseDataItemsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PositionResolutionHistoryResponseDataItemsItem {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "resolution", default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "collateral_change_usd", default, skip_serializing_if = "Option::is_none")]
    pub collateral_change_usd: Option<String>,
    #[serde(rename = "realized_pnl_usd", default, skip_serializing_if = "Option::is_none")]
    pub realized_pnl_usd: Option<String>,
    #[serde(rename = "position_open_seq", default, skip_serializing_if = "Option::is_none")]
    pub position_open_seq: Option<String>,
    #[serde(rename = "position_open_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub position_open_ts_ms: Option<String>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "position_resolution_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub position_resolution_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReferralCodeParams {
    #[serde(rename = "referral_code")]
    pub referral_code: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReferralCodeResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ReferralCodeResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReferralCodeResponseData {
    #[serde(rename = "exists", default, skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    #[serde(rename = "taker_fee_discount_share", default, skip_serializing_if = "Option::is_none")]
    pub taker_fee_discount_share: Option<String>,
    #[serde(rename = "fee_savings_limit_usd", default, skip_serializing_if = "Option::is_none")]
    pub fee_savings_limit_usd: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReferralsParams {
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReferralsResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ReferralsResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReferralsResponseData {
    #[serde(rename = "referral_reward_terms", default, skip_serializing_if = "Option::is_none")]
    pub referral_reward_terms: Option<ReferralsResponseDataReferralRewardTerms>,
    #[serde(rename = "reward_paid_total_usd", default, skip_serializing_if = "Option::is_none")]
    pub reward_paid_total_usd: Option<String>,
    #[serde(rename = "referral_count_total", default, skip_serializing_if = "Option::is_none")]
    pub referral_count_total: Option<serde_json::Number>,
    #[serde(rename = "referrals", default, skip_serializing_if = "Option::is_none")]
    pub referrals: Option<Vec<ReferralsResponseDataReferralsItem>>,
    #[serde(rename = "referrer", default, skip_serializing_if = "Option::is_none")]
    pub referrer: Option<ReferralsResponseDataReferrer>,
    #[serde(rename = "generated_referral_code", default, skip_serializing_if = "Option::is_none")]
    pub generated_referral_code: Option<String>,
    #[serde(rename = "custom_referral_code", default, skip_serializing_if = "Option::is_none")]
    pub custom_referral_code: Option<String>,
    #[serde(rename = "custom_referral_code_revoked", default, skip_serializing_if = "Option::is_none")]
    pub custom_referral_code_revoked: Option<bool>,
    #[serde(rename = "can_generate_referral_code", default, skip_serializing_if = "Option::is_none")]
    pub can_generate_referral_code: Option<bool>,
    #[serde(rename = "can_claim_custom_referral_code", default, skip_serializing_if = "Option::is_none")]
    pub can_claim_custom_referral_code: Option<bool>,
    #[serde(rename = "can_apply_referral_code", default, skip_serializing_if = "Option::is_none")]
    pub can_apply_referral_code: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReferralsResponseDataReferralRewardTerms {
    #[serde(rename = "referral_reward_share", default, skip_serializing_if = "Option::is_none")]
    pub referral_reward_share: Option<String>,
    #[serde(rename = "per_referred_owner_reward_paid_limit_usd", default, skip_serializing_if = "Option::is_none")]
    pub per_referred_owner_reward_paid_limit_usd: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReferralsResponseDataReferralsItem {
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "referred_owner", default, skip_serializing_if = "Option::is_none")]
    pub referred_owner: Option<String>,
    #[serde(rename = "created_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub created_ts_ms: Option<String>,
    #[serde(rename = "referral_reward_share", default, skip_serializing_if = "Option::is_none")]
    pub referral_reward_share: Option<String>,
    #[serde(rename = "reward_paid_usd", default, skip_serializing_if = "Option::is_none")]
    pub reward_paid_usd: Option<String>,
    #[serde(rename = "reward_paid_limit_usd", default, skip_serializing_if = "Option::is_none")]
    pub reward_paid_limit_usd: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ReferralsResponseDataReferrer {
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RegisterADepositAddressParams {
    #[serde(rename = "body")]
    pub body: RegisterADepositAddressParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RegisterADepositAddressParamsBody {
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "referral_code", default, skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RegisterADepositAddressResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<RegisterADepositAddressResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RegisterADepositAddressResponseData {
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "deposit_address", default, skip_serializing_if = "Option::is_none")]
    pub deposit_address: Option<String>,
    #[serde(rename = "registered", default, skip_serializing_if = "Option::is_none")]
    pub registered: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResolvedMarketsParams {
    #[serde(rename = "lookback")]
    pub lookback: String,
    #[serde(rename = "order_by")]
    pub order_by: String,
    #[serde(rename = "limit")]
    pub limit: serde_json::Number,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for ResolvedMarketsParams {
    fn default() -> Self {
        Self {
            lookback: Default::default(),
            order_by: Default::default(),
            limit: 0.into(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResolvedMarketsResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ResolvedMarketsResponseDataItem>>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResolvedMarketsResponseDataItem {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "taker_fee_rate", default, skip_serializing_if = "Option::is_none")]
    pub taker_fee_rate: Option<String>,
    #[serde(rename = "maker_rebate_share", default, skip_serializing_if = "Option::is_none")]
    pub maker_rebate_share: Option<String>,
    #[serde(rename = "tick_size_min", default, skip_serializing_if = "Option::is_none")]
    pub tick_size_min: Option<String>,
    #[serde(rename = "tick_sig_figs", default, skip_serializing_if = "Option::is_none")]
    pub tick_sig_figs: Option<serde_json::Number>,
    #[serde(rename = "is_mece", default, skip_serializing_if = "Option::is_none")]
    pub is_mece: Option<bool>,
    #[serde(rename = "display_attributes", default, skip_serializing_if = "Option::is_none")]
    pub display_attributes: Option<ResolvedMarketsResponseDataItemDisplayAttributes>,
    #[serde(rename = "listing_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub listing_ts_ms: Option<String>,
    #[serde(rename = "rules_url", default, skip_serializing_if = "Option::is_none")]
    pub rules_url: Option<String>,
    #[serde(rename = "stats", default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<ResolvedMarketsResponseDataItemStats>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "open_interest", default, skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<String>,
    #[serde(rename = "long_interest", default, skip_serializing_if = "Option::is_none")]
    pub long_interest: Option<String>,
    #[serde(rename = "short_interest", default, skip_serializing_if = "Option::is_none")]
    pub short_interest: Option<String>,
    #[serde(rename = "resolution", default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<ResolvedMarketsResponseDataItemResolution>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResolvedMarketsResponseDataItemDisplayAttributes {
    #[serde(rename = "event_description", default, skip_serializing_if = "Option::is_none")]
    pub event_description: Option<String>,
    #[serde(rename = "expected_resolution_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub expected_resolution_time_ms: Option<String>,
    #[serde(rename = "market_description", default, skip_serializing_if = "Option::is_none")]
    pub market_description: Option<String>,
    #[serde(rename = "reverse_description", default, skip_serializing_if = "Option::is_none")]
    pub reverse_description: Option<String>,
    #[serde(rename = "sort_order", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "topic", default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<ResolvedMarketsResponseDataItemDisplayAttributesTopic>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResolvedMarketsResponseDataItemDisplayAttributesTopic {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResolvedMarketsResponseDataItemResolution {
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "market_resolution_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub market_resolution_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResolvedMarketsResponseDataItemStats {
    #[serde(rename = "last_24h", default, skip_serializing_if = "Option::is_none")]
    pub last_24h: Option<ResolvedMarketsResponseDataItemStatsLast24h>,
    #[serde(rename = "all_time", default, skip_serializing_if = "Option::is_none")]
    pub all_time: Option<ResolvedMarketsResponseDataItemStatsAllTime>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResolvedMarketsResponseDataItemStatsAllTime {
    #[serde(rename = "taker_buy_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume: Option<String>,
    #[serde(rename = "taker_sell_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume: Option<String>,
    #[serde(rename = "taker_buy_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume_notional: Option<String>,
    #[serde(rename = "taker_sell_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume_notional: Option<String>,
    #[serde(rename = "taker_buy_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_trade_count: Option<String>,
    #[serde(rename = "taker_sell_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_trade_count: Option<String>,
    #[serde(rename = "window_end_ts_ms_exclusive", default, skip_serializing_if = "Option::is_none")]
    pub window_end_ts_ms_exclusive: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResolvedMarketsResponseDataItemStatsLast24h {
    #[serde(rename = "mark_price_start", default, skip_serializing_if = "Option::is_none")]
    pub mark_price_start: Option<String>,
    #[serde(rename = "taker_buy_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume: Option<String>,
    #[serde(rename = "taker_sell_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume: Option<String>,
    #[serde(rename = "taker_buy_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume_notional: Option<String>,
    #[serde(rename = "taker_sell_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume_notional: Option<String>,
    #[serde(rename = "taker_buy_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_trade_count: Option<String>,
    #[serde(rename = "taker_sell_trade_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_trade_count: Option<String>,
    #[serde(rename = "window_end_ts_ms_exclusive", default, skip_serializing_if = "Option::is_none")]
    pub window_end_ts_ms_exclusive: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RevokeATradingKeyParams {
    #[serde(rename = "body")]
    pub body: RevokeATradingKeyParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RevokeATradingKeyParamsBody {
    #[serde(rename = "trading_key")]
    pub trading_key: String,
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RevokeATradingKeyResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<RevokeATradingKeyResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RevokeATradingKeyResponseData {
    #[serde(rename = "trading_key", default, skip_serializing_if = "Option::is_none")]
    pub trading_key: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ServerTimeParams {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ServerTimeResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ServerTimeResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ServerTimeResponseData {
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetAUsernameParams {
    #[serde(rename = "body")]
    pub body: SetAUsernameParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetAUsernameParamsBody {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetAUsernameResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<SetAUsernameResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetAUsernameResponseData {
    #[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetAnAccountBioParams {
    #[serde(rename = "body")]
    pub body: SetAnAccountBioParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetAnAccountBioParamsBody {
    #[serde(rename = "account_bio")]
    pub account_bio: String,
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetAnAccountBioResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<SetAnAccountBioResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SetAnAccountBioResponseData {
    #[serde(rename = "account_bio", default, skip_serializing_if = "Option::is_none")]
    pub account_bio: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TakerDiscountTermsParams {
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TakerDiscountTermsResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<TakerDiscountTermsResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TakerDiscountTermsResponseData {
    #[serde(rename = "taker_discount_terms", default, skip_serializing_if = "Option::is_none")]
    pub taker_discount_terms: Option<TakerDiscountTermsResponseDataTakerDiscountTerms>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TakerDiscountTermsResponseDataTakerDiscountTerms {
    #[serde(rename = "taker_fee_discount_share", default, skip_serializing_if = "Option::is_none")]
    pub taker_fee_discount_share: Option<String>,
    #[serde(rename = "created_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub created_ts_ms: Option<String>,
    #[serde(rename = "fee_savings_limit_usd", default, skip_serializing_if = "Option::is_none")]
    pub fee_savings_limit_usd: Option<String>,
    #[serde(rename = "fee_savings_usd", default, skip_serializing_if = "Option::is_none")]
    pub fee_savings_usd: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TopHoldersParams {
    #[serde(rename = "symbols")]
    pub symbols: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TopHoldersResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<TopHoldersResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TopHoldersResponseData {
    #[serde(rename = "holders", default, skip_serializing_if = "Option::is_none")]
    pub holders: Option<TopHoldersResponseDataHolders>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TopHoldersResponseDataHolders {
    #[serde(rename = "SIM_EVENT_1.MARKET_1", default, skip_serializing_if = "Option::is_none")]
    pub sim_event_1_market_1: Option<TopHoldersResponseDataHoldersSIMEVENT1MARKET1>,
    #[serde(rename = "SIM_EVENT_1.MARKET_2", default, skip_serializing_if = "Option::is_none")]
    pub sim_event_1_market_2: Option<TopHoldersResponseDataHoldersSIMEVENT1MARKET2>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TopHoldersResponseDataHoldersSIMEVENT1MARKET1 {
    #[serde(rename = "longs", default, skip_serializing_if = "Option::is_none")]
    pub longs: Option<Vec<TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItem>>,
    #[serde(rename = "shorts", default, skip_serializing_if = "Option::is_none")]
    pub shorts: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItem {
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "position", default, skip_serializing_if = "Option::is_none")]
    pub position: Option<TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItemPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TopHoldersResponseDataHoldersSIMEVENT1MARKET1LongsItemPosition {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "cumulative_increasing_contracts", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_increasing_contracts: Option<String>,
    #[serde(rename = "cumulative_increasing_notional_usd", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_increasing_notional_usd: Option<String>,
    #[serde(rename = "cumulative_reducing_contracts", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_reducing_contracts: Option<String>,
    #[serde(rename = "cumulative_reducing_notional_usd", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_reducing_notional_usd: Option<String>,
    #[serde(rename = "cumulative_fees_usd", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_fees_usd: Option<String>,
    #[serde(rename = "realized_pnl_usd", default, skip_serializing_if = "Option::is_none")]
    pub realized_pnl_usd: Option<String>,
    #[serde(rename = "unrealized_pnl_usd", default, skip_serializing_if = "Option::is_none")]
    pub unrealized_pnl_usd: Option<String>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "average_entry_price", default, skip_serializing_if = "Option::is_none")]
    pub average_entry_price: Option<String>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "update_seq", default, skip_serializing_if = "Option::is_none")]
    pub update_seq: Option<String>,
    #[serde(rename = "open_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub open_ts_ms: Option<String>,
    #[serde(rename = "update_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub update_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TopHoldersResponseDataHoldersSIMEVENT1MARKET2 {
    #[serde(rename = "longs", default, skip_serializing_if = "Option::is_none")]
    pub longs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "shorts", default, skip_serializing_if = "Option::is_none")]
    pub shorts: Option<Vec<TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItem {
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "position", default, skip_serializing_if = "Option::is_none")]
    pub position: Option<TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItemPosition>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TopHoldersResponseDataHoldersSIMEVENT1MARKET2ShortsItemPosition {
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "cumulative_increasing_contracts", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_increasing_contracts: Option<String>,
    #[serde(rename = "cumulative_increasing_notional_usd", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_increasing_notional_usd: Option<String>,
    #[serde(rename = "cumulative_reducing_contracts", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_reducing_contracts: Option<String>,
    #[serde(rename = "cumulative_reducing_notional_usd", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_reducing_notional_usd: Option<String>,
    #[serde(rename = "cumulative_fees_usd", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_fees_usd: Option<String>,
    #[serde(rename = "realized_pnl_usd", default, skip_serializing_if = "Option::is_none")]
    pub realized_pnl_usd: Option<String>,
    #[serde(rename = "unrealized_pnl_usd", default, skip_serializing_if = "Option::is_none")]
    pub unrealized_pnl_usd: Option<String>,
    #[serde(rename = "mark_price", default, skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<String>,
    #[serde(rename = "average_entry_price", default, skip_serializing_if = "Option::is_none")]
    pub average_entry_price: Option<String>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "update_seq", default, skip_serializing_if = "Option::is_none")]
    pub update_seq: Option<String>,
    #[serde(rename = "open_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub open_ts_ms: Option<String>,
    #[serde(rename = "update_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub update_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TradeCandlesParams {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "end_time_ms")]
    pub end_time_ms: serde_json::Number,
    #[serde(rename = "start_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub start_time_ms: Option<serde_json::Number>,
    #[serde(rename = "count_back", default, skip_serializing_if = "Option::is_none")]
    pub count_back: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}
impl Default for TradeCandlesParams {
    fn default() -> Self {
        Self {
            symbol: Default::default(),
            interval: Default::default(),
            end_time_ms: 0.into(),
            start_time_ms: Default::default(),
            count_back: Default::default(),
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradeCandlesResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TradeCandlesResponseDataItem>>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradeCandlesResponseDataItem {
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "open", default, skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,
    #[serde(rename = "high", default, skip_serializing_if = "Option::is_none")]
    pub high: Option<String>,
    #[serde(rename = "low", default, skip_serializing_if = "Option::is_none")]
    pub low: Option<String>,
    #[serde(rename = "close", default, skip_serializing_if = "Option::is_none")]
    pub close: Option<String>,
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(rename = "taker_buy_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume: Option<String>,
    #[serde(rename = "taker_sell_volume", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume: Option<String>,
    #[serde(rename = "volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub volume_notional: Option<String>,
    #[serde(rename = "taker_buy_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_volume_notional: Option<String>,
    #[serde(rename = "taker_sell_volume_notional", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_volume_notional: Option<String>,
    #[serde(rename = "trade_count", default, skip_serializing_if = "Option::is_none")]
    pub trade_count: Option<String>,
    #[serde(rename = "taker_buy_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_buy_count: Option<String>,
    #[serde(rename = "taker_sell_count", default, skip_serializing_if = "Option::is_none")]
    pub taker_sell_count: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesParams {
    #[serde(rename = "symbols")]
    pub symbols: String,
    #[serde(rename = "before_cursor", default, skip_serializing_if = "Option::is_none")]
    pub before_cursor: Option<String>,
    #[serde(rename = "at_or_before_seq", default, skip_serializing_if = "Option::is_none")]
    pub at_or_before_seq: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<TradesResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesResponseData {
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TradesResponseDataItemsItem>>,
    #[serde(rename = "next_cursor", default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesResponseDataItemsItem {
    #[serde(rename = "trade_id", default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename = "symbol", default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "taker_side", default, skip_serializing_if = "Option::is_none")]
    pub taker_side: Option<String>,
    #[serde(rename = "price", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "maker", default, skip_serializing_if = "Option::is_none")]
    pub maker: Option<String>,
    #[serde(rename = "taker", default, skip_serializing_if = "Option::is_none")]
    pub taker: Option<String>,
    #[serde(rename = "maker_reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub maker_reduce_only: Option<bool>,
    #[serde(rename = "taker_reduce_only", default, skip_serializing_if = "Option::is_none")]
    pub taker_reduce_only: Option<bool>,
    #[serde(rename = "maker_position_size_prev", default, skip_serializing_if = "Option::is_none")]
    pub maker_position_size_prev: Option<String>,
    #[serde(rename = "taker_position_size_prev", default, skip_serializing_if = "Option::is_none")]
    pub taker_position_size_prev: Option<String>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "trade_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub trade_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradingKeyAccountsParams {
    #[serde(rename = "trading_key_pubkey")]
    pub trading_key_pubkey: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradingKeyAccountsResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradingKeysParams {
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradingKeysResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TradingKeysResponseDataItem>>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradingKeysResponseDataItem {
    #[serde(rename = "trading_key", default, skip_serializing_if = "Option::is_none")]
    pub trading_key: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "expiration_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub expiration_ts_ms: Option<String>,
    #[serde(rename = "open_order_count", default, skip_serializing_if = "Option::is_none")]
    pub open_order_count: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferHistoryParams {
    #[serde(rename = "before_cursor", default, skip_serializing_if = "Option::is_none")]
    pub before_cursor: Option<String>,
    #[serde(rename = "at_or_before_seq", default, skip_serializing_if = "Option::is_none")]
    pub at_or_before_seq: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Number>,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferHistoryResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<TransferHistoryResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferHistoryResponseData {
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TransferHistoryResponseDataItemsItem>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TransferHistoryResponseDataItemsItem {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "collateral_change_usd", default, skip_serializing_if = "Option::is_none")]
    pub collateral_change_usd: Option<String>,
    #[serde(rename = "fee_usd", default, skip_serializing_if = "Option::is_none")]
    pub fee_usd: Option<String>,
    #[serde(rename = "net_usd", default, skip_serializing_if = "Option::is_none")]
    pub net_usd: Option<String>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "transfer_ts_ms", default, skip_serializing_if = "Option::is_none")]
    pub transfer_ts_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UsernamesParams {
    #[serde(rename = "owners")]
    pub owners: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UsernamesResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<UsernamesResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UsernamesResponseData {
    #[serde(rename = "usernames", default, skip_serializing_if = "Option::is_none")]
    pub usernames: Option<UsernamesResponseDataUsernames>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UsernamesResponseDataUsernames {
    #[serde(rename = "GmaDrppBC7P5ARKV8g3djiwP89vz1jLK23V2GBjuAEGB", default, skip_serializing_if = "Option::is_none")]
    pub gma_drpp_bc7_p5_arkv8g3djiw_p89vz1j_lk23_v2_g_bju_aegb: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WithdrawCollateralParams {
    #[serde(rename = "body")]
    pub body: WithdrawCollateralParamsBody,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WithdrawCollateralParamsBody {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "destination_authority")]
    pub destination_authority: String,
    #[serde(rename = "destination_token_account")]
    pub destination_token_account: String,
    #[serde(rename = "auth", default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Map<String,serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WithdrawCollateralResponse {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<WithdrawCollateralResponseData>,
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<String>,
    #[serde(rename = "state_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub state_time_ms: Option<String>,
    #[serde(rename = "server_time_ms", default, skip_serializing_if = "Option::is_none")]
    pub server_time_ms: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WithdrawCollateralResponseData {
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "amount", default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "destination_authority", default, skip_serializing_if = "Option::is_none")]
    pub destination_authority: Option<String>,
    #[serde(rename = "destination_token_account", default, skip_serializing_if = "Option::is_none")]
    pub destination_token_account: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedPascalClient(PascalClient);
impl PascalClient {
    pub fn typed(self) -> TypedPascalClient { TypedPascalClient(self) }
}
impl TypedPascalClient {
    pub fn account_bio(&self, request: &AccountBioParams) -> Result<AccountBioResponse> {
        serde_json::from_value(self.0.account_bio(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn account_state_snapshot(&self, request: &AccountStateSnapshotParams) -> Result<AccountStateSnapshotResponse> {
        serde_json::from_value(self.0.account_state_snapshot(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn batch_mark_price_candles(&self, request: &BatchMarkPriceCandlesParams) -> Result<BatchMarkPriceCandlesResponse> {
        serde_json::from_value(self.0.batch_mark_price_candles(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn batch_trades(&self, request: &BatchTradesParams) -> Result<BatchTradesResponse> {
        serde_json::from_value(self.0.batch_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn builder_stats(&self, request: &BuilderStatsParams) -> Result<BuilderStatsResponse> {
        serde_json::from_value(self.0.builder_stats(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn closed_position_history(&self, request: &ClosedPositionHistoryParams) -> Result<ClosedPositionHistoryResponse> {
        serde_json::from_value(self.0.closed_position_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn deposit_address(&self, request: &DepositAddressParams) -> Result<DepositAddressResponse> {
        serde_json::from_value(self.0.deposit_address(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn fill_history(&self, request: &FillHistoryParams) -> Result<FillHistoryResponse> {
        serde_json::from_value(self.0.fill_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn list_markets(&self, request: &ListMarketsParams) -> Result<ListMarketsResponse> {
        serde_json::from_value(self.0.list_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn mark_price_candles(&self, request: &MarkPriceCandlesParams) -> Result<MarkPriceCandlesResponse> {
        serde_json::from_value(self.0.mark_price_candles(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn market_pn_l_movers(&self, request: &MarketPnLMoversParams) -> Result<MarketPnLMoversResponse> {
        serde_json::from_value(self.0.market_pn_l_movers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn market_search(&self, request: &MarketSearchParams) -> Result<MarketSearchResponse> {
        serde_json::from_value(self.0.market_search(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn market_text_search(&self, request: &MarketTextSearchParams) -> Result<MarketTextSearchResponse> {
        serde_json::from_value(self.0.market_text_search(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn order_books(&self, request: &OrderBooksParams) -> Result<OrderBooksResponse> {
        serde_json::from_value(self.0.order_books(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn pn_l_history(&self, request: &PnLHistoryParams) -> Result<PnLHistoryResponse> {
        serde_json::from_value(self.0.pn_l_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn position_resolution_history(&self, request: &PositionResolutionHistoryParams) -> Result<PositionResolutionHistoryResponse> {
        serde_json::from_value(self.0.position_resolution_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn referral_code(&self, request: &ReferralCodeParams) -> Result<ReferralCodeResponse> {
        serde_json::from_value(self.0.referral_code(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn referrals(&self, request: &ReferralsParams) -> Result<ReferralsResponse> {
        serde_json::from_value(self.0.referrals(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn resolved_markets(&self, request: &ResolvedMarketsParams) -> Result<ResolvedMarketsResponse> {
        serde_json::from_value(self.0.resolved_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn server_time(&self, request: &ServerTimeParams) -> Result<ServerTimeResponse> {
        serde_json::from_value(self.0.server_time(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn taker_discount_terms(&self, request: &TakerDiscountTermsParams) -> Result<TakerDiscountTermsResponse> {
        serde_json::from_value(self.0.taker_discount_terms(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn top_holders(&self, request: &TopHoldersParams) -> Result<TopHoldersResponse> {
        serde_json::from_value(self.0.top_holders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn trade_candles(&self, request: &TradeCandlesParams) -> Result<TradeCandlesResponse> {
        serde_json::from_value(self.0.trade_candles(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn trades(&self, request: &TradesParams) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn trading_key_accounts(&self, request: &TradingKeyAccountsParams) -> Result<TradingKeyAccountsResponse> {
        serde_json::from_value(self.0.trading_key_accounts(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn trading_keys(&self, request: &TradingKeysParams) -> Result<TradingKeysResponse> {
        serde_json::from_value(self.0.trading_keys(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn transfer_history(&self, request: &TransferHistoryParams) -> Result<TransferHistoryResponse> {
        serde_json::from_value(self.0.transfer_history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn usernames(&self, request: &UsernamesParams) -> Result<UsernamesResponse> {
        serde_json::from_value(self.0.usernames(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn apply_a_referral_code(&self, request: &ApplyAReferralCodeParams) -> Result<ApplyAReferralCodeResponse> {
        serde_json::from_value(self.0.apply_a_referral_code(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn cancel_orders(&self, request: &CancelOrdersParams) -> Result<Vec<CancelOrdersResponseItem>> {
        serde_json::from_value(self.0.cancel_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn claim_a_custom_referral_code(&self, request: &ClaimACustomReferralCodeParams) -> Result<ClaimACustomReferralCodeResponse> {
        serde_json::from_value(self.0.claim_a_custom_referral_code(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn create_a_trading_key(&self, request: &CreateATradingKeyParams) -> Result<CreateATradingKeyResponse> {
        serde_json::from_value(self.0.create_a_trading_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn generate_a_referral_code(&self, request: &GenerateAReferralCodeParams) -> Result<GenerateAReferralCodeResponse> {
        serde_json::from_value(self.0.generate_a_referral_code(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn mass_cancel_orders(&self, request: &MassCancelOrdersParams) -> Result<Vec<MassCancelOrdersResponseItem>> {
        serde_json::from_value(self.0.mass_cancel_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn place_and_replace_orders(&self, request: &PlaceAndReplaceOrdersParams) -> Result<Vec<PlaceAndReplaceOrdersResponseItem>> {
        serde_json::from_value(self.0.place_and_replace_orders(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn register_a_deposit_address(&self, request: &RegisterADepositAddressParams) -> Result<RegisterADepositAddressResponse> {
        serde_json::from_value(self.0.register_a_deposit_address(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn revoke_a_trading_key(&self, request: &RevokeATradingKeyParams) -> Result<RevokeATradingKeyResponse> {
        serde_json::from_value(self.0.revoke_a_trading_key(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn set_a_username(&self, request: &SetAUsernameParams) -> Result<SetAUsernameResponse> {
        serde_json::from_value(self.0.set_a_username(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn set_an_account_bio(&self, request: &SetAnAccountBioParams) -> Result<SetAnAccountBioResponse> {
        serde_json::from_value(self.0.set_an_account_bio(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub fn withdraw_collateral(&self, request: &WithdrawCollateralParams) -> Result<WithdrawCollateralResponse> {
        serde_json::from_value(self.0.withdraw_collateral(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedPascalClient(crate::AsyncClient<PascalClient>);
#[cfg(feature = "async")]
impl TypedPascalClient {
    pub fn into_async(self) -> AsyncTypedPascalClient { AsyncTypedPascalClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedPascalClient {
    pub async fn account_bio(&self, request: &AccountBioParams) -> Result<AccountBioResponse> {
        serde_json::from_value(self.0.account_bio(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn account_state_snapshot(&self, request: &AccountStateSnapshotParams) -> Result<AccountStateSnapshotResponse> {
        serde_json::from_value(self.0.account_state_snapshot(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn apply_a_referral_code(&self, request: &ApplyAReferralCodeParams) -> Result<ApplyAReferralCodeResponse> {
        serde_json::from_value(self.0.apply_a_referral_code(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn batch_mark_price_candles(&self, request: &BatchMarkPriceCandlesParams) -> Result<BatchMarkPriceCandlesResponse> {
        serde_json::from_value(self.0.batch_mark_price_candles(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn batch_trades(&self, request: &BatchTradesParams) -> Result<BatchTradesResponse> {
        serde_json::from_value(self.0.batch_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn builder_stats(&self, request: &BuilderStatsParams) -> Result<BuilderStatsResponse> {
        serde_json::from_value(self.0.builder_stats(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn cancel_orders(&self, request: &CancelOrdersParams) -> Result<Vec<CancelOrdersResponseItem>> {
        serde_json::from_value(self.0.cancel_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn claim_a_custom_referral_code(&self, request: &ClaimACustomReferralCodeParams) -> Result<ClaimACustomReferralCodeResponse> {
        serde_json::from_value(self.0.claim_a_custom_referral_code(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn closed_position_history(&self, request: &ClosedPositionHistoryParams) -> Result<ClosedPositionHistoryResponse> {
        serde_json::from_value(self.0.closed_position_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn create_a_trading_key(&self, request: &CreateATradingKeyParams) -> Result<CreateATradingKeyResponse> {
        serde_json::from_value(self.0.create_a_trading_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn deposit_address(&self, request: &DepositAddressParams) -> Result<DepositAddressResponse> {
        serde_json::from_value(self.0.deposit_address(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn fill_history(&self, request: &FillHistoryParams) -> Result<FillHistoryResponse> {
        serde_json::from_value(self.0.fill_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn generate_a_referral_code(&self, request: &GenerateAReferralCodeParams) -> Result<GenerateAReferralCodeResponse> {
        serde_json::from_value(self.0.generate_a_referral_code(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn list_markets(&self, request: &ListMarketsParams) -> Result<ListMarketsResponse> {
        serde_json::from_value(self.0.list_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn mark_price_candles(&self, request: &MarkPriceCandlesParams) -> Result<MarkPriceCandlesResponse> {
        serde_json::from_value(self.0.mark_price_candles(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn market_pn_l_movers(&self, request: &MarketPnLMoversParams) -> Result<MarketPnLMoversResponse> {
        serde_json::from_value(self.0.market_pn_l_movers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn market_search(&self, request: &MarketSearchParams) -> Result<MarketSearchResponse> {
        serde_json::from_value(self.0.market_search(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn market_text_search(&self, request: &MarketTextSearchParams) -> Result<MarketTextSearchResponse> {
        serde_json::from_value(self.0.market_text_search(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn mass_cancel_orders(&self, request: &MassCancelOrdersParams) -> Result<Vec<MassCancelOrdersResponseItem>> {
        serde_json::from_value(self.0.mass_cancel_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn order_books(&self, request: &OrderBooksParams) -> Result<OrderBooksResponse> {
        serde_json::from_value(self.0.order_books(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn place_and_replace_orders(&self, request: &PlaceAndReplaceOrdersParams) -> Result<Vec<PlaceAndReplaceOrdersResponseItem>> {
        serde_json::from_value(self.0.place_and_replace_orders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn pn_l_history(&self, request: &PnLHistoryParams) -> Result<PnLHistoryResponse> {
        serde_json::from_value(self.0.pn_l_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn position_resolution_history(&self, request: &PositionResolutionHistoryParams) -> Result<PositionResolutionHistoryResponse> {
        serde_json::from_value(self.0.position_resolution_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn referral_code(&self, request: &ReferralCodeParams) -> Result<ReferralCodeResponse> {
        serde_json::from_value(self.0.referral_code(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn referrals(&self, request: &ReferralsParams) -> Result<ReferralsResponse> {
        serde_json::from_value(self.0.referrals(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn register_a_deposit_address(&self, request: &RegisterADepositAddressParams) -> Result<RegisterADepositAddressResponse> {
        serde_json::from_value(self.0.register_a_deposit_address(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn resolved_markets(&self, request: &ResolvedMarketsParams) -> Result<ResolvedMarketsResponse> {
        serde_json::from_value(self.0.resolved_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn revoke_a_trading_key(&self, request: &RevokeATradingKeyParams) -> Result<RevokeATradingKeyResponse> {
        serde_json::from_value(self.0.revoke_a_trading_key(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn server_time(&self, request: &ServerTimeParams) -> Result<ServerTimeResponse> {
        serde_json::from_value(self.0.server_time(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn set_a_username(&self, request: &SetAUsernameParams) -> Result<SetAUsernameResponse> {
        serde_json::from_value(self.0.set_a_username(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn set_an_account_bio(&self, request: &SetAnAccountBioParams) -> Result<SetAnAccountBioResponse> {
        serde_json::from_value(self.0.set_an_account_bio(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn taker_discount_terms(&self, request: &TakerDiscountTermsParams) -> Result<TakerDiscountTermsResponse> {
        serde_json::from_value(self.0.taker_discount_terms(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn top_holders(&self, request: &TopHoldersParams) -> Result<TopHoldersResponse> {
        serde_json::from_value(self.0.top_holders(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn trade_candles(&self, request: &TradeCandlesParams) -> Result<TradeCandlesResponse> {
        serde_json::from_value(self.0.trade_candles(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn trades(&self, request: &TradesParams) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn trading_key_accounts(&self, request: &TradingKeyAccountsParams) -> Result<TradingKeyAccountsResponse> {
        serde_json::from_value(self.0.trading_key_accounts(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn trading_keys(&self, request: &TradingKeysParams) -> Result<TradingKeysResponse> {
        serde_json::from_value(self.0.trading_keys(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn transfer_history(&self, request: &TransferHistoryParams) -> Result<TransferHistoryResponse> {
        serde_json::from_value(self.0.transfer_history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn usernames(&self, request: &UsernamesParams) -> Result<UsernamesResponse> {
        serde_json::from_value(self.0.usernames(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
    pub async fn withdraw_collateral(&self, request: &WithdrawCollateralParams) -> Result<WithdrawCollateralResponse> {
        serde_json::from_value(self.0.withdraw_collateral(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Pascal response model"))
    }
}
