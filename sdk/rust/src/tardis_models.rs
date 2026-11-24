// Generated from src/data/tardis/schema/tardis.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{TardisClient, Error, Result};

pub type Params = serde_json::Map<String,serde_json::Value>;

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmptyRequest {
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExchangeRequest {
    #[serde(rename = "exchange")]
    pub exchange: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InstrumentRequest {
    #[serde(rename = "exchange")]
    pub exchange: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct InstrumentsRequest {
    #[serde(rename = "exchange")]
    pub exchange: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedTardisClient(TardisClient);
impl TardisClient {
    pub fn typed(self) -> TypedTardisClient { TypedTardisClient(self) }
}
impl TypedTardisClient {
    pub fn api_key_info(&self, request: &EmptyRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.api_key_info(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
    pub fn exchange_details(&self, request: &ExchangeRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.exchange_details(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
    pub fn exchange_summary(&self, request: &ExchangeRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.exchange_summary(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
    pub fn exchanges(&self, request: &EmptyRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.exchanges(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
    pub fn instrument(&self, request: &InstrumentRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.instrument(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
    pub fn instruments(&self, request: &InstrumentsRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.instruments(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
    pub fn symbols(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.symbols(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedTardisClient(crate::AsyncClient<TardisClient>);
#[cfg(feature = "async")]
impl TypedTardisClient {
    pub fn into_async(self) -> AsyncTypedTardisClient { AsyncTypedTardisClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedTardisClient {
    pub async fn api_key_info(&self, request: &EmptyRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.api_key_info(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
    pub async fn exchange_details(&self, request: &ExchangeRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.exchange_details(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
    pub async fn exchange_summary(&self, request: &ExchangeRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.exchange_summary(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
    pub async fn exchanges(&self, request: &EmptyRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.exchanges(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
    pub async fn instrument(&self, request: &InstrumentRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.instrument(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
    pub async fn instruments(&self, request: &InstrumentsRequest) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.instruments(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
    pub async fn symbols(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.symbols(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Tardis response model"))
    }
}
