// Generated from src/exchanges/metric/schema/metric.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{MetricClient, Error, Result};

#[derive(Clone, Serialize, Deserialize)]
pub enum TradeType {
    #[serde(rename = "exact_in")]
    ExactIn,
    #[serde(rename = "exact_out")]
    ExactOut,
}
pub type FactorySettingsResponse = serde_json::Map<String,serde_json::Value>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetadataRequest {
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u16>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "include24h", skip_serializing_if = "Option::is_none")]
    pub include24h: Option<bool>,
    #[serde(rename = "minTvlFiat", skip_serializing_if = "Option::is_none")]
    pub min_tvl_fiat: Option<serde_json::Number>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "lockedPricingOnly", skip_serializing_if = "Option::is_none")]
    pub locked_pricing_only: Option<bool>,
    #[serde(rename = "depositAllowlistEnabled", skip_serializing_if = "Option::is_none")]
    pub deposit_allowlist_enabled: Option<bool>,
    #[serde(rename = "blacklistedOnly", skip_serializing_if = "Option::is_none")]
    pub blacklisted_only: Option<bool>,
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<String>,
    #[serde(rename = "contract_version", skip_serializing_if = "Option::is_none")]
    pub contract_version: Option<String>,
    #[serde(rename = "fiat", skip_serializing_if = "Option::is_none")]
    pub fiat: Option<String>,
    #[serde(rename = "chainIds", skip_serializing_if = "Option::is_none")]
    pub chain_ids: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Chain {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ChainsResponse {
    #[serde(rename = "chains")]
    pub chains: Vec<Chain>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PoolMetadata {
    #[serde(rename = "poolAddress")]
    pub pool_address: String,
    #[serde(rename = "pair")]
    pub pair: String,
    #[serde(rename = "chain")]
    pub chain: String,
    #[serde(rename = "token0")]
    pub token0: String,
    #[serde(rename = "token1")]
    pub token1: String,
    #[serde(rename = "token0Decimals")]
    pub token0_decimals: u8,
    #[serde(rename = "token1Decimals")]
    pub token1_decimals: u8,
    #[serde(rename = "priceProviderAddress", skip_serializing_if = "Option::is_none")]
    pub price_provider_address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MetadataResponse {
    #[serde(rename = "data")]
    pub data: Vec<PoolMetadata>,
    #[serde(rename = "total")]
    pub total: u64,
    #[serde(rename = "nextOffset", skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DepthLevel {
    #[serde(rename = "binIdx")]
    pub bin_idx: i16,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "cumulativeVolume")]
    pub cumulative_volume: String,
    #[serde(rename = "cumulativeInputVolume")]
    pub cumulative_input_volume: String,
    #[serde(rename = "priceImpactE6")]
    pub price_impact_e6: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Depth {
    #[serde(rename = "bids")]
    pub bids: Vec<DepthLevel>,
    #[serde(rename = "asks")]
    pub asks: Vec<DepthLevel>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BidAskResponse {
    #[serde(rename = "poolAddress", skip_serializing_if = "Option::is_none")]
    pub pool_address: Option<String>,
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    #[serde(rename = "priceProviderStatus")]
    pub price_provider_status: String,
    #[serde(rename = "bidAdj", skip_serializing_if = "Option::is_none")]
    pub bid_adj: Option<String>,
    #[serde(rename = "askAdj", skip_serializing_if = "Option::is_none")]
    pub ask_adj: Option<String>,
    #[serde(rename = "totalToken0Available", skip_serializing_if = "Option::is_none")]
    pub total_token0_available: Option<String>,
    #[serde(rename = "totalToken1Available", skip_serializing_if = "Option::is_none")]
    pub total_token1_available: Option<String>,
    #[serde(rename = "serverTs", skip_serializing_if = "Option::is_none")]
    pub server_ts: Option<u64>,
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<Depth>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchBidAskRequest {
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "pools")]
    pub pools: Vec<String>,
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BatchBidAskResponse {
    #[serde(rename = "results")]
    pub results: Vec<PoolQuote>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PoolQuote {
    #[serde(rename = "poolAddress")]
    pub pool_address: String,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<BidAskResponse>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<PoolQuoteError>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PoolQuoteError {
    #[serde(rename = "status")]
    pub status: u16,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteRequest {
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "tokenIn")]
    pub token_in: String,
    #[serde(rename = "tokenOut")]
    pub token_out: String,
    #[serde(rename = "tradeType", skip_serializing_if = "Option::is_none")]
    pub trade_type: Option<TradeType>,
    #[serde(rename = "amountIn", skip_serializing_if = "Option::is_none")]
    pub amount_in: Option<String>,
    #[serde(rename = "amountOut", skip_serializing_if = "Option::is_none")]
    pub amount_out: Option<String>,
    #[serde(rename = "maxRoutes", skip_serializing_if = "Option::is_none")]
    pub max_routes: Option<u16>,
    #[serde(rename = "poolAddress", skip_serializing_if = "Option::is_none")]
    pub pool_address: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RouteLeg {
    #[serde(rename = "tradeType")]
    pub trade_type: TradeType,
    #[serde(rename = "pair")]
    pub pair: String,
    #[serde(rename = "effectivePriceX64")]
    pub effective_price_x64: String,
    #[serde(rename = "priceImpactE6")]
    pub price_impact_e6: String,
    #[serde(rename = "poolAddress")]
    pub pool_address: String,
    #[serde(rename = "zeroForOne")]
    pub zero_for_one: bool,
    #[serde(rename = "tokenIn")]
    pub token_in: String,
    #[serde(rename = "tokenOut")]
    pub token_out: String,
    #[serde(rename = "amountIn")]
    pub amount_in: String,
    #[serde(rename = "amountOut")]
    pub amount_out: String,
    #[serde(rename = "priceLimitX64")]
    pub price_limit_x64: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RouteOption {
    #[serde(rename = "tradeType")]
    pub trade_type: TradeType,
    #[serde(rename = "routeType")]
    pub route_type: String,
    #[serde(rename = "effectivePriceX64")]
    pub effective_price_x64: String,
    #[serde(rename = "priceImpactE6")]
    pub price_impact_e6: String,
    #[serde(rename = "amountIn")]
    pub amount_in: String,
    #[serde(rename = "amountOut")]
    pub amount_out: String,
    #[serde(rename = "legs")]
    pub legs: Vec<RouteLeg>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RouteResponse {
    #[serde(rename = "tokenIn")]
    pub token_in: String,
    #[serde(rename = "tokenOut")]
    pub token_out: String,
    #[serde(rename = "effectivePriceX64")]
    pub effective_price_x64: String,
    #[serde(rename = "priceImpactBestRoute")]
    pub price_impact_best_route: String,
    #[serde(rename = "candidatesConsidered")]
    pub candidates_considered: u64,
    #[serde(rename = "tradeType")]
    pub trade_type: TradeType,
    #[serde(rename = "amountIn")]
    pub amount_in: String,
    #[serde(rename = "amountOut")]
    pub amount_out: String,
    #[serde(rename = "routes")]
    pub routes: Vec<RouteOption>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PammQuoteRequest {
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "tx")]
    pub tx: String,
    #[serde(rename = "blockNumber")]
    pub block_number: u64,
    #[serde(rename = "replacementUuid")]
    pub replacement_uuid: String,
    #[serde(rename = "replacementSeqNumber")]
    pub replacement_seq_number: u64,
    #[serde(rename = "disableCrossRegionSharing", skip_serializing_if = "Option::is_none")]
    pub disable_cross_region_sharing: Option<bool>,
    #[serde(rename = "responseTimeoutMs", skip_serializing_if = "Option::is_none")]
    pub response_timeout_ms: Option<u16>,
    #[serde(rename = "maxTimestamp", skip_serializing_if = "Option::is_none")]
    pub max_timestamp: Option<u64>,
    #[serde(rename = "alwaysInclude", skip_serializing_if = "Option::is_none")]
    pub always_include: Option<bool>,
    #[serde(rename = "quoteAddress", skip_serializing_if = "Option::is_none")]
    pub quote_address: Option<String>,
    #[serde(rename = "poolAddresses", skip_serializing_if = "Option::is_none")]
    pub pool_addresses: Option<Vec<String>>,
    #[serde(rename = "assetPairs", skip_serializing_if = "Option::is_none")]
    pub asset_pairs: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BuilderOutcome {
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "accepted")]
    pub accepted: bool,
    #[serde(rename = "pending", default)]
    pub pending: bool,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "bundleHash", skip_serializing_if = "Option::is_none")]
    pub bundle_hash: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PammQuoteResponse {
    #[serde(rename = "accepted")]
    pub accepted: bool,
    #[serde(rename = "providers")]
    pub providers: Vec<BuilderOutcome>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmptyRequest {
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BidAskRequest {
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "pool")]
    pub pool: String,
    #[serde(rename = "depth", skip_serializing_if = "Option::is_none")]
    pub depth: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrossChainMetadataRequest {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u16>,
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "include24h", skip_serializing_if = "Option::is_none")]
    pub include24h: Option<bool>,
    #[serde(rename = "minTvlFiat", skip_serializing_if = "Option::is_none")]
    pub min_tvl_fiat: Option<serde_json::Number>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "lockedPricingOnly", skip_serializing_if = "Option::is_none")]
    pub locked_pricing_only: Option<bool>,
    #[serde(rename = "depositAllowlistEnabled", skip_serializing_if = "Option::is_none")]
    pub deposit_allowlist_enabled: Option<bool>,
    #[serde(rename = "blacklistedOnly", skip_serializing_if = "Option::is_none")]
    pub blacklisted_only: Option<bool>,
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<String>,
    #[serde(rename = "contract_version", skip_serializing_if = "Option::is_none")]
    pub contract_version: Option<String>,
    #[serde(rename = "fiat", skip_serializing_if = "Option::is_none")]
    pub fiat: Option<String>,
    #[serde(rename = "chainIds", skip_serializing_if = "Option::is_none")]
    pub chain_ids: Option<String>,
}

pub struct TypedMetricClient(MetricClient);
impl MetricClient {
    pub fn typed(self) -> TypedMetricClient { TypedMetricClient(self) }
}
impl TypedMetricClient {
    pub fn batch_bid_ask(&self, request: &BatchBidAskRequest) -> Result<BatchBidAskResponse> {
        serde_json::from_value(self.0.batch_bid_ask(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub fn bid_ask(&self, request: &BidAskRequest) -> Result<BidAskResponse> {
        serde_json::from_value(self.0.bid_ask(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub fn chains(&self, request: &EmptyRequest) -> Result<ChainsResponse> {
        serde_json::from_value(self.0.chains(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub fn cross_chain_metadata(&self, request: &CrossChainMetadataRequest) -> Result<MetadataResponse> {
        serde_json::from_value(self.0.cross_chain_metadata(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub fn factory_settings(&self, request: &EmptyRequest) -> Result<FactorySettingsResponse> {
        serde_json::from_value(self.0.factory_settings(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub fn metadata(&self, request: &MetadataRequest) -> Result<MetadataResponse> {
        serde_json::from_value(self.0.metadata(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub fn pamm_quote(&self, request: &PammQuoteRequest) -> Result<PammQuoteResponse> {
        serde_json::from_value(self.0.pamm_quote(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub fn route(&self, request: &RouteRequest) -> Result<RouteResponse> {
        serde_json::from_value(self.0.route(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedMetricClient(crate::AsyncClient<MetricClient>);
#[cfg(feature = "async")]
impl TypedMetricClient {
    pub fn into_async(self) -> AsyncTypedMetricClient { AsyncTypedMetricClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedMetricClient {
    pub async fn batch_bid_ask(&self, request: &BatchBidAskRequest) -> Result<BatchBidAskResponse> {
        serde_json::from_value(self.0.batch_bid_ask(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub async fn bid_ask(&self, request: &BidAskRequest) -> Result<BidAskResponse> {
        serde_json::from_value(self.0.bid_ask(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub async fn chains(&self, request: &EmptyRequest) -> Result<ChainsResponse> {
        serde_json::from_value(self.0.chains(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub async fn cross_chain_metadata(&self, request: &CrossChainMetadataRequest) -> Result<MetadataResponse> {
        serde_json::from_value(self.0.cross_chain_metadata(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub async fn factory_settings(&self, request: &EmptyRequest) -> Result<FactorySettingsResponse> {
        serde_json::from_value(self.0.factory_settings(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub async fn metadata(&self, request: &MetadataRequest) -> Result<MetadataResponse> {
        serde_json::from_value(self.0.metadata(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub async fn pamm_quote(&self, request: &PammQuoteRequest) -> Result<PammQuoteResponse> {
        serde_json::from_value(self.0.pamm_quote(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
    pub async fn route(&self, request: &RouteRequest) -> Result<RouteResponse> {
        serde_json::from_value(self.0.route(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Metric response model"))
    }
}
