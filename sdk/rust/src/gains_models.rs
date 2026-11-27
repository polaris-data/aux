// Generated from src/exchanges/gains/schema/gains.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{GainsClient, Error, Result};

pub type MarketsResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct MarketsRequest {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedGainsClient(GainsClient);
impl GainsClient {
    pub fn typed(self) -> TypedGainsClient { TypedGainsClient(self) }
}
impl TypedGainsClient {
    pub fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Gains response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedGainsClient(crate::AsyncClient<GainsClient>);
#[cfg(feature = "async")]
impl TypedGainsClient {
    pub fn into_async(self) -> AsyncTypedGainsClient { AsyncTypedGainsClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedGainsClient {
    pub async fn markets(&self, request: &MarketsRequest) -> Result<MarketsResponse> {
        serde_json::from_value(self.0.markets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Gains response model"))
    }
}
