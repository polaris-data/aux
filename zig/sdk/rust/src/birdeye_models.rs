// Generated from schema/birdeye.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{BirdeyeClient, Error, Result};


#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceRequest {
    #[serde(rename = "check_liquidity", default, skip_serializing_if = "Option::is_none")]
    pub check_liquidity: Option<serde_json::Number>,
    #[serde(rename = "include_liquidity", default, skip_serializing_if = "Option::is_none")]
    pub include_liquidity: Option<bool>,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultiPriceRequest {
    #[serde(rename = "check_liquidity", default, skip_serializing_if = "Option::is_none")]
    pub check_liquidity: Option<serde_json::Number>,
    #[serde(rename = "include_liquidity", default, skip_serializing_if = "Option::is_none")]
    pub include_liquidity: Option<bool>,
    #[serde(rename = "list_address")]
    pub list_address: String,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultiPricePostBody {
    #[serde(rename = "list_address")]
    pub list_address: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultiPricePostRequest {
    #[serde(rename = "check_liquidity", default, skip_serializing_if = "Option::is_none")]
    pub check_liquidity: Option<serde_json::Number>,
    #[serde(rename = "include_liquidity", default, skip_serializing_if = "Option::is_none")]
    pub include_liquidity: Option<bool>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
    #[serde(rename = "body")]
    pub body: MultiPricePostBody,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenTradesRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "tx_type", default, skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    #[serde(rename = "sort_type", default, skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PairTradesRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "tx_type", default, skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    #[serde(rename = "sort_type", default, skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenTradesByTimeRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "tx_type", default, skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    #[serde(rename = "before_time", default, skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    #[serde(rename = "after_time", default, skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PairTradesByTimeRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "tx_type", default, skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    #[serde(rename = "before_time", default, skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    #[serde(rename = "after_time", default, skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TradesV3Request {
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sort_type", default, skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    #[serde(rename = "tx_type", default, skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "pool_id", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    #[serde(rename = "before_time", default, skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    #[serde(rename = "after_time", default, skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    #[serde(rename = "before_block_number", default, skip_serializing_if = "Option::is_none")]
    pub before_block_number: Option<i64>,
    #[serde(rename = "after_block_number", default, skip_serializing_if = "Option::is_none")]
    pub after_block_number: Option<i64>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenTradesV3Request {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sort_type", default, skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    #[serde(rename = "tx_type", default, skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "pool_id", default, skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,
    #[serde(rename = "before_time", default, skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    #[serde(rename = "after_time", default, skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    #[serde(rename = "before_block_number", default, skip_serializing_if = "Option::is_none")]
    pub before_block_number: Option<i64>,
    #[serde(rename = "after_block_number", default, skip_serializing_if = "Option::is_none")]
    pub after_block_number: Option<i64>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecentTradesRequest {
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "tx_type", default, skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "before_time", default, skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    #[serde(rename = "after_time", default, skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    #[serde(rename = "before_block_number", default, skip_serializing_if = "Option::is_none")]
    pub before_block_number: Option<i64>,
    #[serde(rename = "after_block_number", default, skip_serializing_if = "Option::is_none")]
    pub after_block_number: Option<i64>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OhlcvRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "chart_type", default, skip_serializing_if = "Option::is_none")]
    pub chart_type: Option<String>,
    #[serde(rename = "time_from")]
    pub time_from: i64,
    #[serde(rename = "time_to")]
    pub time_to: i64,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OhlcvPairRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "time_from")]
    pub time_from: i64,
    #[serde(rename = "time_to")]
    pub time_to: i64,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OhlcvBaseQuoteRequest {
    #[serde(rename = "base_address")]
    pub base_address: String,
    #[serde(rename = "quote_address")]
    pub quote_address: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "time_from")]
    pub time_from: i64,
    #[serde(rename = "time_to")]
    pub time_to: i64,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OhlcvV3Request {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "chart_type", default, skip_serializing_if = "Option::is_none")]
    pub chart_type: Option<String>,
    #[serde(rename = "time_from")]
    pub time_from: i64,
    #[serde(rename = "time_to")]
    pub time_to: i64,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
    #[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "count_limit", default, skip_serializing_if = "Option::is_none")]
    pub count_limit: Option<i64>,
    #[serde(rename = "padding", default, skip_serializing_if = "Option::is_none")]
    pub padding: Option<bool>,
    #[serde(rename = "outlier", default, skip_serializing_if = "Option::is_none")]
    pub outlier: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OhlcvPairV3Request {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "time_from")]
    pub time_from: i64,
    #[serde(rename = "time_to")]
    pub time_to: i64,
    #[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "count_limit", default, skip_serializing_if = "Option::is_none")]
    pub count_limit: Option<i64>,
    #[serde(rename = "padding", default, skip_serializing_if = "Option::is_none")]
    pub padding: Option<bool>,
    #[serde(rename = "outlier", default, skip_serializing_if = "Option::is_none")]
    pub outlier: Option<bool>,
    #[serde(rename = "inversion", default, skip_serializing_if = "Option::is_none")]
    pub inversion: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiquidityOhlcPairRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(rename = "direction", default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiquidityOhlcTokenRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "resolution", default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(rename = "direction", default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiquidityHistoryTokenRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "resolution", default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(rename = "direction", default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiquidityHistoryPairRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "unix_time", default, skip_serializing_if = "Option::is_none")]
    pub unix_time: Option<i64>,
    #[serde(rename = "block_number", default, skip_serializing_if = "Option::is_none")]
    pub block_number: Option<i64>,
    #[serde(rename = "step", default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i64>,
    #[serde(rename = "skip_empty", default, skip_serializing_if = "Option::is_none")]
    pub skip_empty: Option<bool>,
    #[serde(rename = "direction", default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(rename = "count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiquidityLatestTokenBody {
    #[serde(rename = "addresses")]
    pub addresses: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiquidityLatestTokenRequest {
    #[serde(rename = "body")]
    pub body: LiquidityLatestTokenBody,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryPriceRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "address_type")]
    pub address_type: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "time_from")]
    pub time_from: i64,
    #[serde(rename = "time_to")]
    pub time_to: i64,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoricalPriceUnixRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "unixtime", default, skip_serializing_if = "Option::is_none")]
    pub unixtime: Option<i64>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceVolumeSingleRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceVolumeMultiBody {
    #[serde(rename = "list_address")]
    pub list_address: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceVolumeMultiRequest {
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
    #[serde(rename = "body")]
    pub body: PriceVolumeMultiBody,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PairOverviewRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PairOverviewMultipleRequest {
    #[serde(rename = "list_address")]
    pub list_address: String,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceStatsRequest {
    #[serde(rename = "list_timeframe", default, skip_serializing_if = "Option::is_none")]
    pub list_timeframe: Option<String>,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceStatsMultipleBody {
    #[serde(rename = "list_address")]
    pub list_address: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceStatsMultipleRequest {
    #[serde(rename = "list_timeframe", default, skip_serializing_if = "Option::is_none")]
    pub list_timeframe: Option<String>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
    #[serde(rename = "body")]
    pub body: PriceStatsMultipleBody,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenTradesByVolumeRequest {
    #[serde(rename = "token_address")]
    pub token_address: String,
    #[serde(rename = "volume_type")]
    pub volume_type: String,
    #[serde(rename = "min_volume", default, skip_serializing_if = "Option::is_none")]
    pub min_volume: Option<serde_json::Number>,
    #[serde(rename = "max_volume", default, skip_serializing_if = "Option::is_none")]
    pub max_volume: Option<serde_json::Number>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "tx_type", default, skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "before_time", default, skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    #[serde(rename = "after_time", default, skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
    #[serde(rename = "sort_type", default, skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenListRequest {
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sort_type", default, skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "min_liquidity", default, skip_serializing_if = "Option::is_none")]
    pub min_liquidity: Option<serde_json::Number>,
    #[serde(rename = "max_liquidity", default, skip_serializing_if = "Option::is_none")]
    pub max_liquidity: Option<serde_json::Number>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenListV3Request {
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sort_type", default, skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    #[serde(rename = "platform_name", default, skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "min_liquidity", default, skip_serializing_if = "Option::is_none")]
    pub min_liquidity: Option<serde_json::Number>,
    #[serde(rename = "max_liquidity", default, skip_serializing_if = "Option::is_none")]
    pub max_liquidity: Option<serde_json::Number>,
    #[serde(rename = "min_market_cap", default, skip_serializing_if = "Option::is_none")]
    pub min_market_cap: Option<serde_json::Number>,
    #[serde(rename = "max_market_cap", default, skip_serializing_if = "Option::is_none")]
    pub max_market_cap: Option<serde_json::Number>,
    #[serde(rename = "min_fdv", default, skip_serializing_if = "Option::is_none")]
    pub min_fdv: Option<serde_json::Number>,
    #[serde(rename = "max_fdv", default, skip_serializing_if = "Option::is_none")]
    pub max_fdv: Option<serde_json::Number>,
    #[serde(rename = "min_recent_listing_time", default, skip_serializing_if = "Option::is_none")]
    pub min_recent_listing_time: Option<i64>,
    #[serde(rename = "max_recent_listing_time", default, skip_serializing_if = "Option::is_none")]
    pub max_recent_listing_time: Option<i64>,
    #[serde(rename = "min_last_trade_unix_time", default, skip_serializing_if = "Option::is_none")]
    pub min_last_trade_unix_time: Option<i64>,
    #[serde(rename = "max_last_trade_unix_time", default, skip_serializing_if = "Option::is_none")]
    pub max_last_trade_unix_time: Option<i64>,
    #[serde(rename = "min_holder", default, skip_serializing_if = "Option::is_none")]
    pub min_holder: Option<i64>,
    #[serde(rename = "min_volume_1m_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_1m_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_5m_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_5m_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_30m_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_30m_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_1h_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_1h_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_2h_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_2h_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_4h_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_4h_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_8h_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_8h_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_24h_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_24h_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_7d_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_7d_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_30d_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_30d_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_1m_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_1m_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_5m_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_5m_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_30m_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_30m_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_1h_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_1h_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_2h_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_2h_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_4h_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_4h_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_8h_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_8h_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_24h_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_24h_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_7d_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_7d_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_30d_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_30d_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_1m_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_1m_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_5m_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_5m_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_30m_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_30m_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_1h_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_1h_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_2h_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_2h_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_4h_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_4h_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_8h_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_8h_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_24h_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_24h_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_7d_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_7d_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_30d_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_30d_percent: Option<serde_json::Number>,
    #[serde(rename = "min_trade_1m_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_1m_count: Option<i64>,
    #[serde(rename = "min_trade_5m_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_5m_count: Option<i64>,
    #[serde(rename = "min_trade_30m_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_30m_count: Option<i64>,
    #[serde(rename = "min_trade_1h_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_1h_count: Option<i64>,
    #[serde(rename = "min_trade_2h_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_2h_count: Option<i64>,
    #[serde(rename = "min_trade_4h_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_4h_count: Option<i64>,
    #[serde(rename = "min_trade_8h_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_8h_count: Option<i64>,
    #[serde(rename = "min_trade_24h_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_24h_count: Option<i64>,
    #[serde(rename = "min_trade_7d_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_7d_count: Option<i64>,
    #[serde(rename = "min_trade_30d_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_30d_count: Option<i64>,
    #[serde(rename = "min_global_fees_paid", default, skip_serializing_if = "Option::is_none")]
    pub min_global_fees_paid: Option<serde_json::Number>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenListScrollRequest {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "scroll_id", default, skip_serializing_if = "Option::is_none")]
    pub scroll_id: Option<String>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sort_type", default, skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    #[serde(rename = "platform_name", default, skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "min_liquidity", default, skip_serializing_if = "Option::is_none")]
    pub min_liquidity: Option<serde_json::Number>,
    #[serde(rename = "max_liquidity", default, skip_serializing_if = "Option::is_none")]
    pub max_liquidity: Option<serde_json::Number>,
    #[serde(rename = "min_market_cap", default, skip_serializing_if = "Option::is_none")]
    pub min_market_cap: Option<serde_json::Number>,
    #[serde(rename = "max_market_cap", default, skip_serializing_if = "Option::is_none")]
    pub max_market_cap: Option<serde_json::Number>,
    #[serde(rename = "min_fdv", default, skip_serializing_if = "Option::is_none")]
    pub min_fdv: Option<serde_json::Number>,
    #[serde(rename = "max_fdv", default, skip_serializing_if = "Option::is_none")]
    pub max_fdv: Option<serde_json::Number>,
    #[serde(rename = "min_recent_listing_time", default, skip_serializing_if = "Option::is_none")]
    pub min_recent_listing_time: Option<i64>,
    #[serde(rename = "max_recent_listing_time", default, skip_serializing_if = "Option::is_none")]
    pub max_recent_listing_time: Option<i64>,
    #[serde(rename = "min_last_trade_unix_time", default, skip_serializing_if = "Option::is_none")]
    pub min_last_trade_unix_time: Option<i64>,
    #[serde(rename = "max_last_trade_unix_time", default, skip_serializing_if = "Option::is_none")]
    pub max_last_trade_unix_time: Option<i64>,
    #[serde(rename = "min_holder", default, skip_serializing_if = "Option::is_none")]
    pub min_holder: Option<i64>,
    #[serde(rename = "min_volume_1m_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_1m_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_5m_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_5m_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_30m_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_30m_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_1h_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_1h_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_2h_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_2h_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_4h_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_4h_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_8h_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_8h_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_24h_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_24h_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_7d_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_7d_usd: Option<serde_json::Number>,
    #[serde(rename = "min_volume_30d_usd", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_30d_usd: Option<serde_json::Number>,
    #[serde(rename = "min_global_fees_paid", default, skip_serializing_if = "Option::is_none")]
    pub min_global_fees_paid: Option<serde_json::Number>,
    #[serde(rename = "min_volume_1m_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_1m_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_5m_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_5m_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_30m_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_30m_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_1h_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_1h_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_2h_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_2h_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_4h_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_4h_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_8h_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_8h_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_24h_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_24h_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_7d_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_7d_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_volume_30d_change_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_volume_30d_change_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_1m_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_1m_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_5m_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_5m_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_30m_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_30m_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_1h_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_1h_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_2h_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_2h_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_4h_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_4h_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_8h_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_8h_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_24h_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_24h_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_7d_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_7d_percent: Option<serde_json::Number>,
    #[serde(rename = "min_price_change_30d_percent", default, skip_serializing_if = "Option::is_none")]
    pub min_price_change_30d_percent: Option<serde_json::Number>,
    #[serde(rename = "min_trade_1m_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_1m_count: Option<i64>,
    #[serde(rename = "min_trade_5m_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_5m_count: Option<i64>,
    #[serde(rename = "min_trade_30m_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_30m_count: Option<i64>,
    #[serde(rename = "min_trade_1h_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_1h_count: Option<i64>,
    #[serde(rename = "min_trade_2h_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_2h_count: Option<i64>,
    #[serde(rename = "min_trade_4h_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_4h_count: Option<i64>,
    #[serde(rename = "min_trade_8h_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_8h_count: Option<i64>,
    #[serde(rename = "min_trade_24h_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_24h_count: Option<i64>,
    #[serde(rename = "min_trade_7d_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_7d_count: Option<i64>,
    #[serde(rename = "min_trade_30d_count", default, skip_serializing_if = "Option::is_none")]
    pub min_trade_30d_count: Option<i64>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenOverviewRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "frames", default, skip_serializing_if = "Option::is_none")]
    pub frames: Option<String>,
    #[serde(rename = "includes", default, skip_serializing_if = "Option::is_none")]
    pub includes: Option<String>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenMetadataRequest {
    #[serde(rename = "address")]
    pub address: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenMetadataMultipleRequest {
    #[serde(rename = "list_address")]
    pub list_address: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenMarketDataRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenMarketDataMultipleRequest {
    #[serde(rename = "list_address")]
    pub list_address: String,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenTradeDataRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "frames", default, skip_serializing_if = "Option::is_none")]
    pub frames: Option<String>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
    #[serde(rename = "includes", default, skip_serializing_if = "Option::is_none")]
    pub includes: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenTradeDataMultipleRequest {
    #[serde(rename = "list_address")]
    pub list_address: String,
    #[serde(rename = "frames", default, skip_serializing_if = "Option::is_none")]
    pub frames: Option<String>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
    #[serde(rename = "includes", default, skip_serializing_if = "Option::is_none")]
    pub includes: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenNewListingRequest {
    #[serde(rename = "time_to", default, skip_serializing_if = "Option::is_none")]
    pub time_to: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "meme_platform_enabled", default, skip_serializing_if = "Option::is_none")]
    pub meme_platform_enabled: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenMarketsRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "sort_type", default, skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenSecurityRequest {
    #[serde(rename = "include", default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(rename = "address")]
    pub address: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenMintBurnRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sort_type", default, skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "after_time", default, skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    #[serde(rename = "before_time", default, skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenExitLiquidityRequest {
    #[serde(rename = "address")]
    pub address: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenExitLiquidityMultipleRequest {
    #[serde(rename = "list_address")]
    pub list_address: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraderTradesByTimeRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "tx_type", default, skip_serializing_if = "Option::is_none")]
    pub tx_type: Option<String>,
    #[serde(rename = "before_time", default, skip_serializing_if = "Option::is_none")]
    pub before_time: Option<i64>,
    #[serde(rename = "after_time", default, skip_serializing_if = "Option::is_none")]
    pub after_time: Option<i64>,
    #[serde(rename = "ui_amount_mode", default, skip_serializing_if = "Option::is_none")]
    pub ui_amount_mode: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "data")]
    pub data: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedBirdeyeClient(BirdeyeClient);
impl BirdeyeClient {
    pub fn typed(self) -> TypedBirdeyeClient { TypedBirdeyeClient(self) }
}
impl TypedBirdeyeClient {
    pub fn historical_price_unix(&self, request: &HistoricalPriceUnixRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.historical_price_unix(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn history_price(&self, request: &HistoryPriceRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.history_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn liquidity_history_pair(&self, request: &LiquidityHistoryPairRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.liquidity_history_pair(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn liquidity_history_token(&self, request: &LiquidityHistoryTokenRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.liquidity_history_token(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn liquidity_latest_token(&self, request: &LiquidityLatestTokenRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.liquidity_latest_token(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn liquidity_ohlc_pair(&self, request: &LiquidityOhlcPairRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.liquidity_ohlc_pair(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn liquidity_ohlc_token(&self, request: &LiquidityOhlcTokenRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.liquidity_ohlc_token(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn multi_price(&self, request: &MultiPriceRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.multi_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn multi_price_post(&self, request: &MultiPricePostRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.multi_price_post(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn ohlcv(&self, request: &OhlcvRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.ohlcv(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn ohlcv_base_quote(&self, request: &OhlcvBaseQuoteRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.ohlcv_base_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn ohlcv_pair(&self, request: &OhlcvPairRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.ohlcv_pair(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn ohlcv_pair_v3(&self, request: &OhlcvPairV3Request) -> Result<ApiResponse> {
        serde_json::from_value(self.0.ohlcv_pair_v3(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn ohlcv_v3(&self, request: &OhlcvV3Request) -> Result<ApiResponse> {
        serde_json::from_value(self.0.ohlcv_v3(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn pair_overview(&self, request: &PairOverviewRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.pair_overview(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn pair_overview_multiple(&self, request: &PairOverviewMultipleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.pair_overview_multiple(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn pair_trades(&self, request: &PairTradesRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.pair_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn pair_trades_by_time(&self, request: &PairTradesByTimeRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.pair_trades_by_time(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn price(&self, request: &PriceRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn price_stats(&self, request: &PriceStatsRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.price_stats(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn price_stats_multiple(&self, request: &PriceStatsMultipleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.price_stats_multiple(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn price_volume_multi(&self, request: &PriceVolumeMultiRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.price_volume_multi(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn price_volume_single(&self, request: &PriceVolumeSingleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.price_volume_single(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn recent_trades(&self, request: &RecentTradesRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.recent_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_exit_liquidity(&self, request: &TokenExitLiquidityRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_exit_liquidity(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_exit_liquidity_multiple(&self, request: &TokenExitLiquidityMultipleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_exit_liquidity_multiple(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_list(&self, request: &TokenListRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_list(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_list_scroll(&self, request: &TokenListScrollRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_list_scroll(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_list_v3(&self, request: &TokenListV3Request) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_list_v3(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_market_data(&self, request: &TokenMarketDataRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_market_data(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_market_data_multiple(&self, request: &TokenMarketDataMultipleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_market_data_multiple(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_markets(&self, request: &TokenMarketsRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_metadata(&self, request: &TokenMetadataRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_metadata(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_metadata_multiple(&self, request: &TokenMetadataMultipleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_metadata_multiple(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_mint_burn(&self, request: &TokenMintBurnRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_mint_burn(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_new_listing(&self, request: &TokenNewListingRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_new_listing(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_overview(&self, request: &TokenOverviewRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_overview(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_security(&self, request: &TokenSecurityRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_security(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_trade_data(&self, request: &TokenTradeDataRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_trade_data(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_trade_data_multiple(&self, request: &TokenTradeDataMultipleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_trade_data_multiple(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_trades(&self, request: &TokenTradesRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_trades_by_time(&self, request: &TokenTradesByTimeRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_trades_by_time(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_trades_by_volume(&self, request: &TokenTradesByVolumeRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_trades_by_volume(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn token_trades_v3(&self, request: &TokenTradesV3Request) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_trades_v3(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn trader_trades_by_time(&self, request: &TraderTradesByTimeRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.trader_trades_by_time(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub fn trades_v3(&self, request: &TradesV3Request) -> Result<ApiResponse> {
        serde_json::from_value(self.0.trades_v3(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedBirdeyeClient(crate::AsyncClient<BirdeyeClient>);
#[cfg(feature = "async")]
impl TypedBirdeyeClient {
    pub fn into_async(self) -> AsyncTypedBirdeyeClient { AsyncTypedBirdeyeClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedBirdeyeClient {
    pub async fn historical_price_unix(&self, request: &HistoricalPriceUnixRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.historical_price_unix(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn history_price(&self, request: &HistoryPriceRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.history_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn liquidity_history_pair(&self, request: &LiquidityHistoryPairRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.liquidity_history_pair(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn liquidity_history_token(&self, request: &LiquidityHistoryTokenRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.liquidity_history_token(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn liquidity_latest_token(&self, request: &LiquidityLatestTokenRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.liquidity_latest_token(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn liquidity_ohlc_pair(&self, request: &LiquidityOhlcPairRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.liquidity_ohlc_pair(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn liquidity_ohlc_token(&self, request: &LiquidityOhlcTokenRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.liquidity_ohlc_token(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn multi_price(&self, request: &MultiPriceRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.multi_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn multi_price_post(&self, request: &MultiPricePostRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.multi_price_post(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn ohlcv(&self, request: &OhlcvRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.ohlcv(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn ohlcv_base_quote(&self, request: &OhlcvBaseQuoteRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.ohlcv_base_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn ohlcv_pair(&self, request: &OhlcvPairRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.ohlcv_pair(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn ohlcv_pair_v3(&self, request: &OhlcvPairV3Request) -> Result<ApiResponse> {
        serde_json::from_value(self.0.ohlcv_pair_v3(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn ohlcv_v3(&self, request: &OhlcvV3Request) -> Result<ApiResponse> {
        serde_json::from_value(self.0.ohlcv_v3(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn pair_overview(&self, request: &PairOverviewRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.pair_overview(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn pair_overview_multiple(&self, request: &PairOverviewMultipleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.pair_overview_multiple(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn pair_trades(&self, request: &PairTradesRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.pair_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn pair_trades_by_time(&self, request: &PairTradesByTimeRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.pair_trades_by_time(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn price(&self, request: &PriceRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn price_stats(&self, request: &PriceStatsRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.price_stats(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn price_stats_multiple(&self, request: &PriceStatsMultipleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.price_stats_multiple(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn price_volume_multi(&self, request: &PriceVolumeMultiRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.price_volume_multi(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn price_volume_single(&self, request: &PriceVolumeSingleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.price_volume_single(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn recent_trades(&self, request: &RecentTradesRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.recent_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_exit_liquidity(&self, request: &TokenExitLiquidityRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_exit_liquidity(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_exit_liquidity_multiple(&self, request: &TokenExitLiquidityMultipleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_exit_liquidity_multiple(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_list(&self, request: &TokenListRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_list(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_list_scroll(&self, request: &TokenListScrollRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_list_scroll(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_list_v3(&self, request: &TokenListV3Request) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_list_v3(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_market_data(&self, request: &TokenMarketDataRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_market_data(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_market_data_multiple(&self, request: &TokenMarketDataMultipleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_market_data_multiple(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_markets(&self, request: &TokenMarketsRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_metadata(&self, request: &TokenMetadataRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_metadata(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_metadata_multiple(&self, request: &TokenMetadataMultipleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_metadata_multiple(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_mint_burn(&self, request: &TokenMintBurnRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_mint_burn(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_new_listing(&self, request: &TokenNewListingRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_new_listing(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_overview(&self, request: &TokenOverviewRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_overview(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_security(&self, request: &TokenSecurityRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_security(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_trade_data(&self, request: &TokenTradeDataRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_trade_data(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_trade_data_multiple(&self, request: &TokenTradeDataMultipleRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_trade_data_multiple(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_trades(&self, request: &TokenTradesRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_trades_by_time(&self, request: &TokenTradesByTimeRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_trades_by_time(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_trades_by_volume(&self, request: &TokenTradesByVolumeRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_trades_by_volume(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn token_trades_v3(&self, request: &TokenTradesV3Request) -> Result<ApiResponse> {
        serde_json::from_value(self.0.token_trades_v3(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn trader_trades_by_time(&self, request: &TraderTradesByTimeRequest) -> Result<ApiResponse> {
        serde_json::from_value(self.0.trader_trades_by_time(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
    pub async fn trades_v3(&self, request: &TradesV3Request) -> Result<ApiResponse> {
        serde_json::from_value(self.0.trades_v3(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Birdeye response model"))
    }
}
