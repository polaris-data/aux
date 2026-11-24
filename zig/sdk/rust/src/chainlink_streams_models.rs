// Generated from schema/chainlink_streams.json.
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{ChainlinkDataStreamsClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiscoveryRequest {
    #[serde(rename = "base_asset", default, skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(rename = "quote_asset", default, skip_serializing_if = "Option::is_none")]
    pub quote_asset: Option<String>,
    #[serde(rename = "asset_class", default, skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<String>,
    #[serde(rename = "feed_type", default, skip_serializing_if = "Option::is_none")]
    pub feed_type: Option<String>,
    #[serde(rename = "attribute_type", default, skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "network_type", default, skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "created_after", default, skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(rename = "created_before", default, skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    #[serde(rename = "expected_decommission_after", default, skip_serializing_if = "Option::is_none")]
    pub expected_decommission_after: Option<String>,
    #[serde(rename = "expected_decommission_before", default, skip_serializing_if = "Option::is_none")]
    pub expected_decommission_before: Option<String>,
    #[serde(rename = "decommissioned_after", default, skip_serializing_if = "Option::is_none")]
    pub decommissioned_after: Option<String>,
    #[serde(rename = "decommissioned_before", default, skip_serializing_if = "Option::is_none")]
    pub decommissioned_before: Option<String>,
    #[serde(rename = "hidden", default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
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
pub struct ReportRequest {
    #[serde(rename = "feedID")]
    pub feed_id: String,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LatestReportRequest {
    #[serde(rename = "feedID")]
    pub feed_id: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BulkReportsRequest {
    #[serde(rename = "feedIDs")]
    pub feed_ids: Vec<String>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportPageRequest {
    #[serde(rename = "feedID")]
    pub feed_id: String,
    #[serde(rename = "startTimestamp")]
    pub start_timestamp: u64,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

pub struct TypedChainlinkDataStreamsClient(ChainlinkDataStreamsClient);
impl ChainlinkDataStreamsClient {
    pub fn typed(self) -> TypedChainlinkDataStreamsClient { TypedChainlinkDataStreamsClient(self) }
}
impl TypedChainlinkDataStreamsClient {
    pub fn bulk_reports(&self, request: &BulkReportsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.bulk_reports(&json!(request))?)
            .map_err(|_| Error::Decode("invalid ChainlinkStreams response model"))
    }
    pub fn discovery(&self, request: &DiscoveryRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.discovery(&json!(request))?)
            .map_err(|_| Error::Decode("invalid ChainlinkStreams response model"))
    }
    pub fn latest_report(&self, request: &LatestReportRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.latest_report(&json!(request))?)
            .map_err(|_| Error::Decode("invalid ChainlinkStreams response model"))
    }
    pub fn report(&self, request: &ReportRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.report(&json!(request))?)
            .map_err(|_| Error::Decode("invalid ChainlinkStreams response model"))
    }
    pub fn report_page(&self, request: &ReportPageRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.report_page(&json!(request))?)
            .map_err(|_| Error::Decode("invalid ChainlinkStreams response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedChainlinkDataStreamsClient(crate::AsyncClient<ChainlinkDataStreamsClient>);
#[cfg(feature = "async")]
impl TypedChainlinkDataStreamsClient {
    pub fn into_async(self) -> AsyncTypedChainlinkDataStreamsClient { AsyncTypedChainlinkDataStreamsClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedChainlinkDataStreamsClient {
    pub async fn bulk_reports(&self, request: &BulkReportsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.bulk_reports(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid ChainlinkStreams response model"))
    }
    pub async fn discovery(&self, request: &DiscoveryRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.discovery(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid ChainlinkStreams response model"))
    }
    pub async fn latest_report(&self, request: &LatestReportRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.latest_report(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid ChainlinkStreams response model"))
    }
    pub async fn report(&self, request: &ReportRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.report(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid ChainlinkStreams response model"))
    }
    pub async fn report_page(&self, request: &ReportPageRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.report_page(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid ChainlinkStreams response model"))
    }
}
