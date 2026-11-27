// Generated from src/exchanges/synfutures/schema/synfutures.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{SynFuturesClient, Error, Result};

pub type GraphResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct VirtualTradesRequest {
    #[serde(rename = "since")]
    pub since: u64,
    #[serde(rename = "until")]
    pub until: u64,
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
    pub first: Option<u16>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CandlesRequest {
    #[serde(rename = "since")]
    pub since: u64,
    #[serde(rename = "until")]
    pub until: u64,
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[serde(rename = "first", skip_serializing_if = "Option::is_none")]
    pub first: Option<u16>,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "amm_ids")]
    pub amm_ids: Vec<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedSynFuturesClient(SynFuturesClient);
impl SynFuturesClient {
    pub fn typed(self) -> TypedSynFuturesClient { TypedSynFuturesClient(self) }
}
impl TypedSynFuturesClient {
    pub fn candles(&self, request: &CandlesRequest) -> Result<GraphResponse> {
        serde_json::from_value(self.0.candles(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Synfutures response model"))
    }
    pub fn virtual_trades(&self, request: &VirtualTradesRequest) -> Result<GraphResponse> {
        serde_json::from_value(self.0.virtual_trades(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Synfutures response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedSynFuturesClient(crate::AsyncClient<SynFuturesClient>);
#[cfg(feature = "async")]
impl TypedSynFuturesClient {
    pub fn into_async(self) -> AsyncTypedSynFuturesClient { AsyncTypedSynFuturesClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedSynFuturesClient {
    pub async fn candles(&self, request: &CandlesRequest) -> Result<GraphResponse> {
        serde_json::from_value(self.0.candles(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Synfutures response model"))
    }
    pub async fn virtual_trades(&self, request: &VirtualTradesRequest) -> Result<GraphResponse> {
        serde_json::from_value(self.0.virtual_trades(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Synfutures response model"))
    }
}
