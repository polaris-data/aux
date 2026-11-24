// Generated from schema/pyth_hermes.json.
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{PythHermesClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HermesLiveRequest {
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HealthResponse {
    #[serde(rename = "status")]
    pub status: u16,
    #[serde(rename = "headers")]
    pub headers: Vec<[String;2]>,
    #[serde(rename = "data")]
    pub data: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HermesReadyRequest {
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceFeedsRequest {
    #[serde(rename = "query", default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "asset_type", default, skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeResponse {
    #[serde(rename = "status")]
    pub status: u16,
    #[serde(rename = "headers")]
    pub headers: Vec<[String;2]>,
    #[serde(rename = "data")]
    pub data: serde_json::Value,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LatestPriceUpdatesRequest {
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
    #[serde(rename = "encoding", default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(rename = "parsed", default, skip_serializing_if = "Option::is_none")]
    pub parsed: Option<bool>,
    #[serde(rename = "ignore_invalid_price_ids", default, skip_serializing_if = "Option::is_none")]
    pub ignore_invalid_price_ids: Option<bool>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceUpdatesAtTimestampRequest {
    #[serde(rename = "publish_time")]
    pub publish_time: u64,
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
    #[serde(rename = "encoding", default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(rename = "parsed", default, skip_serializing_if = "Option::is_none")]
    pub parsed: Option<bool>,
    #[serde(rename = "ignore_invalid_price_ids", default, skip_serializing_if = "Option::is_none")]
    pub ignore_invalid_price_ids: Option<bool>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LatestTwapsRequest {
    #[serde(rename = "window_seconds")]
    pub window_seconds: u64,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LatestPublisherStakeCapsRequest {
    #[serde(rename = "encoding", default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(rename = "parsed", default, skip_serializing_if = "Option::is_none")]
    pub parsed: Option<bool>,
}

pub struct TypedPythHermesClient(PythHermesClient);
impl PythHermesClient {
    pub fn typed(self) -> TypedPythHermesClient { TypedPythHermesClient(self) }
}
impl TypedPythHermesClient {
    pub fn latest_price_updates(&self, request: &LatestPriceUpdatesRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.latest_price_updates(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
    pub fn latest_publisher_stake_caps(&self, request: &LatestPublisherStakeCapsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.latest_publisher_stake_caps(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
    pub fn latest_twaps(&self, request: &LatestTwapsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.latest_twaps(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
    pub fn live(&self, request: &HermesLiveRequest) -> Result<HealthResponse> {
        serde_json::from_value(self.0.live(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
    pub fn price_feeds(&self, request: &PriceFeedsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.price_feeds(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
    pub fn price_updates_at_timestamp(&self, request: &PriceUpdatesAtTimestampRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.price_updates_at_timestamp(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
    pub fn ready(&self, request: &HermesReadyRequest) -> Result<HealthResponse> {
        serde_json::from_value(self.0.ready(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedPythHermesClient(crate::AsyncClient<PythHermesClient>);
#[cfg(feature = "async")]
impl TypedPythHermesClient {
    pub fn into_async(self) -> AsyncTypedPythHermesClient { AsyncTypedPythHermesClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedPythHermesClient {
    pub async fn latest_price_updates(&self, request: &LatestPriceUpdatesRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.latest_price_updates(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
    pub async fn latest_publisher_stake_caps(&self, request: &LatestPublisherStakeCapsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.latest_publisher_stake_caps(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
    pub async fn latest_twaps(&self, request: &LatestTwapsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.latest_twaps(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
    pub async fn live(&self, request: &HermesLiveRequest) -> Result<HealthResponse> {
        serde_json::from_value(self.0.live(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
    pub async fn price_feeds(&self, request: &PriceFeedsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.price_feeds(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
    pub async fn price_updates_at_timestamp(&self, request: &PriceUpdatesAtTimestampRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.price_updates_at_timestamp(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
    pub async fn ready(&self, request: &HermesReadyRequest) -> Result<HealthResponse> {
        serde_json::from_value(self.0.ready(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythHermes response model"))
    }
}
