// Generated from schema/pyth_pro.json.
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{PythProClient, Error, Result};


#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiveRequest {
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
pub struct ReadyRequest {
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolsRequest {
    #[serde(rename = "query", default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "asset_type", default, skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<Vec<String>>,
    #[serde(rename = "instrument_type", default, skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<Vec<String>>,
    #[serde(rename = "symbol_chain_id", default, skip_serializing_if = "Option::is_none")]
    pub symbol_chain_id: Option<Vec<String>>,
    #[serde(rename = "entitled_only", default, skip_serializing_if = "Option::is_none")]
    pub entitled_only: Option<bool>,
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
pub struct ConfigRequest {
    #[serde(rename = "channel")]
    pub channel: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchRequest {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "query")]
    pub query: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "exchange", default, skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolveSymbolRequest {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryRequest {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "from")]
    pub from: u64,
    #[serde(rename = "to")]
    pub to: u64,
    #[serde(rename = "resolution")]
    pub resolution: String,
    #[serde(rename = "split_adjusted", default, skip_serializing_if = "Option::is_none")]
    pub split_adjusted: Option<bool>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceRequest {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "ids")]
    pub ids: Vec<u32>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "split_adjusted", default, skip_serializing_if = "Option::is_none")]
    pub split_adjusted: Option<bool>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceRangeRequest {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "ids")]
    pub ids: Vec<u32>,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: u64,
    #[serde(rename = "end_timestamp")]
    pub end_timestamp: u64,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(rename = "after", default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(rename = "split_adjusted", default, skip_serializing_if = "Option::is_none")]
    pub split_adjusted: Option<bool>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublisherUpdatesRequest {
    #[serde(rename = "price_feed_id", default, skip_serializing_if = "Option::is_none")]
    pub price_feed_id: Option<Vec<u32>>,
    #[serde(rename = "min_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub min_timestamp: Option<String>,
    #[serde(rename = "max_timestamp", default, skip_serializing_if = "Option::is_none")]
    pub max_timestamp: Option<String>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "status_reason", default, skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "status_reason_unset", default, skip_serializing_if = "Option::is_none")]
    pub status_reason_unset: Option<bool>,
    #[serde(rename = "sort")]
    pub sort: String,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LatestPriceRequest {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "priceFeedIds", default, skip_serializing_if = "Option::is_none")]
    pub price_feed_ids: Option<Vec<u32>>,
    #[serde(rename = "properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<String>>,
    #[serde(rename = "formats")]
    pub formats: Vec<String>,
    #[serde(rename = "parsed", default, skip_serializing_if = "Option::is_none")]
    pub parsed: Option<bool>,
    #[serde(rename = "jsonBinaryEncoding", default, skip_serializing_if = "Option::is_none")]
    pub json_binary_encoding: Option<String>,
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceAtTimestampRequest {
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "priceFeedIds", default, skip_serializing_if = "Option::is_none")]
    pub price_feed_ids: Option<Vec<u32>>,
    #[serde(rename = "properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<String>>,
    #[serde(rename = "formats")]
    pub formats: Vec<String>,
    #[serde(rename = "parsed", default, skip_serializing_if = "Option::is_none")]
    pub parsed: Option<bool>,
    #[serde(rename = "jsonBinaryEncoding", default, skip_serializing_if = "Option::is_none")]
    pub json_binary_encoding: Option<String>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "symbols", default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReducePriceRequest {
    #[serde(rename = "payload")]
    pub payload: serde_json::Value,
    #[serde(rename = "priceFeedIds")]
    pub price_feed_ids: Vec<u32>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublisherStakeCapsRequest {
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuardianSetUpgradeRequest {
}

pub struct TypedPythProClient(PythProClient);
impl PythProClient {
    pub fn typed(self) -> TypedPythProClient { TypedPythProClient(self) }
}
impl TypedPythProClient {
    pub fn config(&self, request: &ConfigRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.config(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn guardian_set_upgrade(&self, request: &GuardianSetUpgradeRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.guardian_set_upgrade(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn history(&self, request: &HistoryRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.history(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn latest_price(&self, request: &LatestPriceRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.latest_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn live(&self, request: &LiveRequest) -> Result<HealthResponse> {
        serde_json::from_value(self.0.live(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn price(&self, request: &PriceRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn price_at_timestamp(&self, request: &PriceAtTimestampRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.price_at_timestamp(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn price_range(&self, request: &PriceRangeRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.price_range(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn publisher_stake_caps(&self, request: &PublisherStakeCapsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.publisher_stake_caps(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn publisher_updates(&self, request: &PublisherUpdatesRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.publisher_updates(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn ready(&self, request: &ReadyRequest) -> Result<HealthResponse> {
        serde_json::from_value(self.0.ready(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn reduce_price(&self, request: &ReducePriceRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.reduce_price(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn resolve_symbol(&self, request: &ResolveSymbolRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.resolve_symbol(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn search(&self, request: &SearchRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.search(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub fn symbols(&self, request: &SymbolsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.symbols(&json!(request))?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedPythProClient(crate::AsyncClient<PythProClient>);
#[cfg(feature = "async")]
impl TypedPythProClient {
    pub fn into_async(self) -> AsyncTypedPythProClient { AsyncTypedPythProClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedPythProClient {
    pub async fn config(&self, request: &ConfigRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.config(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn guardian_set_upgrade(&self, request: &GuardianSetUpgradeRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.guardian_set_upgrade(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn history(&self, request: &HistoryRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.history(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn latest_price(&self, request: &LatestPriceRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.latest_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn live(&self, request: &LiveRequest) -> Result<HealthResponse> {
        serde_json::from_value(self.0.live(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn price(&self, request: &PriceRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn price_at_timestamp(&self, request: &PriceAtTimestampRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.price_at_timestamp(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn price_range(&self, request: &PriceRangeRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.price_range(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn publisher_stake_caps(&self, request: &PublisherStakeCapsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.publisher_stake_caps(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn publisher_updates(&self, request: &PublisherUpdatesRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.publisher_updates(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn ready(&self, request: &ReadyRequest) -> Result<HealthResponse> {
        serde_json::from_value(self.0.ready(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn reduce_price(&self, request: &ReducePriceRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.reduce_price(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn resolve_symbol(&self, request: &ResolveSymbolRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.resolve_symbol(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn search(&self, request: &SearchRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.search(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
    pub async fn symbols(&self, request: &SymbolsRequest) -> Result<NativeResponse> {
        serde_json::from_value(self.0.symbols(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid PythPro response model"))
    }
}
