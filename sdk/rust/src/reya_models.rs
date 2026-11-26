// Generated from src/exchanges/reya/schema/reya.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{ReyaClient, Error, Result};

pub type MarketsResponse = serde_json::Value;
pub type MarketSummariesResponse = serde_json::Value;
pub type MarketSummaryResponse = serde_json::Value;
pub type MarketExecutionsResponse = serde_json::Value;
pub type WalletExecutionsResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSummariesRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketSummaryRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketExecutionsRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct WalletExecutionsRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedReyaClient(ReyaClient);
impl ReyaClient {
    pub fn typed(self) -> TypedReyaClient { TypedReyaClient(self) }
}
impl TypedReyaClient {
    pub fn market_executions(&self, request: &MarketExecutionsRequest) -> Result<MarketExecutionsResponse> {
        serde_json::from_value(self.0.market_executions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Reya response model"))
    }
    pub fn market_summaries(&self, request: &MarketSummariesRequest) -> Result<MarketSummariesResponse> {
        serde_json::from_value(self.0.market_summaries(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Reya response model"))
    }
    pub fn market_summary(&self, request: &MarketSummaryRequest) -> Result<MarketSummaryResponse> {
        serde_json::from_value(self.0.market_summary(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Reya response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Reya response model"))
    }
    pub fn wallet_executions(&self, request: &WalletExecutionsRequest) -> Result<WalletExecutionsResponse> {
        serde_json::from_value(self.0.wallet_executions(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Reya response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedReyaClient(crate::AsyncClient<ReyaClient>);
#[cfg(feature = "async")]
impl TypedReyaClient {
    pub fn into_async(self) -> AsyncTypedReyaClient { AsyncTypedReyaClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedReyaClient {
    pub async fn market_executions(&self, request: &MarketExecutionsRequest) -> Result<MarketExecutionsResponse> {
        serde_json::from_value(self.0.market_executions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Reya response model"))
    }
    pub async fn market_summaries(&self, request: &MarketSummariesRequest) -> Result<MarketSummariesResponse> {
        serde_json::from_value(self.0.market_summaries(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Reya response model"))
    }
    pub async fn market_summary(&self, request: &MarketSummaryRequest) -> Result<MarketSummaryResponse> {
        serde_json::from_value(self.0.market_summary(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Reya response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Reya response model"))
    }
    pub async fn wallet_executions(&self, request: &WalletExecutionsRequest) -> Result<WalletExecutionsResponse> {
        serde_json::from_value(self.0.wallet_executions(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Reya response model"))
    }
}
