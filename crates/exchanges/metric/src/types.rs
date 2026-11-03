use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::collections::BTreeMap;

pub type Extra = BTreeMap<String, Value>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MetadataRequest {
    pub count: Option<u16>,
    pub offset: Option<u64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub include24h: Option<bool>,
    pub min_tvl_fiat: Option<Number>,
    pub tier: Option<String>,
    pub locked_pricing_only: Option<bool>,
    pub deposit_allowlist_enabled: Option<bool>,
    pub blacklisted_only: Option<bool>,
    pub addresses: Option<String>,
    #[serde(rename = "contract_version")]
    pub contract_version: Option<String>,
    pub fiat: Option<String>,
    pub chain_ids: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chain {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    pub name: String,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainsResponse {
    pub chains: Vec<Chain>,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolMetadata {
    pub pool_address: String,
    pub pair: String,
    pub chain: String,
    pub token0: String,
    pub token1: String,
    pub token0_decimals: u8,
    pub token1_decimals: u8,
    #[serde(default)]
    pub price_provider_address: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataResponse {
    pub data: Vec<PoolMetadata>,
    pub total: u64,
    pub next_offset: Option<u64>,
    #[serde(flatten)]
    pub extra: Extra,
}
/// Factory policy is keyed by native chain/version names; preserve its native envelope.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FactorySettingsResponse {
    #[serde(flatten)]
    pub fields: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthLevel {
    pub bin_idx: i16,
    pub price: String,
    pub cumulative_volume: String,
    pub cumulative_input_volume: String,
    pub price_impact_e6: String,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Depth {
    pub bids: Vec<DepthLevel>,
    pub asks: Vec<DepthLevel>,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BidAskResponse {
    #[serde(default)]
    pub pool_address: Option<String>,
    #[serde(default)]
    pub pair: Option<String>,
    pub price_provider_status: String,
    pub bid_adj: Option<String>,
    pub ask_adj: Option<String>,
    #[serde(default)]
    pub total_token0_available: Option<String>,
    #[serde(default)]
    pub total_token1_available: Option<String>,
    #[serde(default)]
    pub server_ts: Option<u64>,
    pub depth: Option<Depth>,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchBidAskRequest {
    pub pools: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<bool>,
}
/// The native batch envelope retains each pool's quote or error without flattening failures.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchBidAskResponse {
    pub results: Vec<PoolQuote>,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolQuote {
    pub pool_address: String,
    pub quote: Option<BidAskResponse>,
    pub error: Option<PoolQuoteError>,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PoolQuoteError {
    pub status: u16,
    pub error: String,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TradeType {
    ExactIn,
    ExactOut,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RouteRequest {
    pub token_in: String,
    pub token_out: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_type: Option<TradeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_out: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_routes: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_address: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteLeg {
    pub trade_type: TradeType,
    pub pair: String,
    pub effective_price_x64: String,
    pub price_impact_e6: String,
    pub pool_address: String,
    pub zero_for_one: bool,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
    pub amount_out: String,
    pub price_limit_x64: String,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteOption {
    pub trade_type: TradeType,
    pub route_type: String,
    pub effective_price_x64: String,
    pub price_impact_e6: String,
    pub amount_in: String,
    pub amount_out: String,
    pub legs: Vec<RouteLeg>,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteResponse {
    pub token_in: String,
    pub token_out: String,
    pub effective_price_x64: String,
    pub price_impact_best_route: String,
    pub candidates_considered: u64,
    pub trade_type: TradeType,
    pub amount_in: String,
    pub amount_out: String,
    pub routes: Vec<RouteOption>,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PammQuoteRequest {
    pub tx: String,
    pub block_number: u64,
    pub replacement_uuid: String,
    pub replacement_seq_number: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_cross_region_sharing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_timeout_ms: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_timestamp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_include: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_addresses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_pairs: Option<Vec<String>>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuilderOutcome {
    pub provider: String,
    pub accepted: bool,
    #[serde(default)]
    pub pending: bool,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub bundle_hash: Option<String>,
    #[serde(flatten)]
    pub extra: Extra,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PammQuoteResponse {
    pub accepted: bool,
    pub providers: Vec<BuilderOutcome>,
    #[serde(flatten)]
    pub extra: Extra,
}
