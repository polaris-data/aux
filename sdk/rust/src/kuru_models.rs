// Generated from src/exchanges/kuru/schema/kuru.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{KuruClient, Error, Result};

pub type MarketsResponse = serde_json::Value;
pub type MarketResponse = serde_json::Value;
pub type TradesResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "marketAddress", skip_serializing_if = "Option::is_none")]
    pub market_address: Option<String>,
    #[serde(rename = "feedEpoch", skip_serializing_if = "Option::is_none")]
    pub feed_epoch: Option<u64>,
    #[serde(rename = "seq", skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketRequest {
    #[serde(rename = "marketAddress")]
    pub market_address: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TradesRequest {
    #[serde(rename = "marketAddress")]
    pub market_address: String,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<u64>,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedKuruClient(KuruClient);
impl KuruClient {
    pub fn typed(self) -> TypedKuruClient { TypedKuruClient(self) }
}
impl TypedKuruClient {
    pub fn market(&self, request: &MarketRequest) -> Result<MarketResponse> {
        serde_json::from_value(self.0.market(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kuru response model"))
    }
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kuru response model"))
    }
    pub fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Kuru response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedKuruClient(crate::AsyncClient<KuruClient>);
#[cfg(feature = "async")]
impl TypedKuruClient {
    pub fn into_async(self) -> AsyncTypedKuruClient { AsyncTypedKuruClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedKuruClient {
    pub async fn market(&self, request: &MarketRequest) -> Result<MarketResponse> {
        serde_json::from_value(self.0.market(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kuru response model"))
    }
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kuru response model"))
    }
    pub async fn trades(&self, request: &TradesRequest) -> Result<TradesResponse> {
        serde_json::from_value(self.0.trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Kuru response model"))
    }
}
