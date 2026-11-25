// Generated from src/exchanges/variational/schema/variational.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{VariationalClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketStatsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundingLast {
    #[serde(rename = "annualized_rate")]
    pub annualized_rate: String,
    #[serde(rename = "interval_rate")]
    pub interval_rate: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct FundingNext {
    #[serde(rename = "annualized_rate")]
    pub annualized_rate: String,
    #[serde(rename = "interval_rate")]
    pub interval_rate: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Funding {
    #[serde(rename = "last")]
    pub last: FundingLast,
    #[serde(rename = "next")]
    pub next: FundingNext,
    #[serde(rename = "interval_seconds")]
    pub interval_seconds: u64,
    #[serde(rename = "next_timestamp")]
    pub next_timestamp: u64,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketQuotesBase {
    #[serde(rename = "bid")]
    pub bid: String,
    #[serde(rename = "ask")]
    pub ask: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketQuotesSize1K {
    #[serde(rename = "bid")]
    pub bid: String,
    #[serde(rename = "ask")]
    pub ask: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketQuotesSize100K {
    #[serde(rename = "bid")]
    pub bid: String,
    #[serde(rename = "ask")]
    pub ask: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketQuotes {
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "base", skip_serializing_if = "Option::is_none")]
    pub base: Option<MarketQuotesBase>,
    #[serde(rename = "size_1k", skip_serializing_if = "Option::is_none")]
    pub size_1k: Option<MarketQuotesSize1K>,
    #[serde(rename = "size_100k", skip_serializing_if = "Option::is_none")]
    pub size_100k: Option<MarketQuotesSize100K>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketListingOpenInterest {
    #[serde(rename = "long_open_interest")]
    pub long_open_interest: String,
    #[serde(rename = "short_open_interest")]
    pub short_open_interest: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketListing {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "contract_type")]
    pub contract_type: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "mark_price")]
    pub mark_price: String,
    #[serde(rename = "volume_24h")]
    pub volume_24h: String,
    #[serde(rename = "open_interest")]
    pub open_interest: MarketListingOpenInterest,
    #[serde(rename = "funding", skip_serializing_if = "Option::is_none")]
    pub funding: Option<Funding>,
    #[serde(rename = "base_spread_bps")]
    pub base_spread_bps: String,
    #[serde(rename = "quotes", skip_serializing_if = "Option::is_none")]
    pub quotes: Option<MarketQuotes>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketStatsResponseLossRefund {
    #[serde(rename = "pool_size")]
    pub pool_size: String,
    #[serde(rename = "refunded_24h")]
    pub refunded_24h: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketStatsResponse {
    #[serde(rename = "total_volume_24h")]
    pub total_volume_24h: String,
    #[serde(rename = "cumulative_volume")]
    pub cumulative_volume: String,
    #[serde(rename = "tvl")]
    pub tvl: String,
    #[serde(rename = "open_interest")]
    pub open_interest: String,
    #[serde(rename = "num_markets")]
    pub num_markets: u64,
    #[serde(rename = "loss_refund")]
    pub loss_refund: MarketStatsResponseLossRefund,
    #[serde(rename = "listings")]
    pub listings: Vec<MarketListing>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedVariationalClient(VariationalClient);
impl VariationalClient {
    pub fn typed(self) -> TypedVariationalClient { TypedVariationalClient(self) }
}
impl TypedVariationalClient {
    pub fn market_stats(&self, request: &MarketStatsRequest) -> Result<MarketStatsResponse> {
        serde_json::from_value(self.0.market_stats(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Variational response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedVariationalClient(crate::AsyncClient<VariationalClient>);
#[cfg(feature = "async")]
impl TypedVariationalClient {
    pub fn into_async(self) -> AsyncTypedVariationalClient { AsyncTypedVariationalClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedVariationalClient {
    pub async fn market_stats(&self, request: &MarketStatsRequest) -> Result<MarketStatsResponse> {
        serde_json::from_value(self.0.market_stats(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Variational response model"))
    }
}
