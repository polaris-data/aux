// Generated from src/exchanges/vest/schema/vest.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{VestClient, Error, Result};

pub type ExchangeInfoResponse = serde_json::Value;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ExchangeInfoRequest {
    #[serde(rename = "account_group", skip_serializing_if = "Option::is_none")]
    pub account_group: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

pub struct TypedVestClient(VestClient);
impl VestClient {
    pub fn typed(self) -> TypedVestClient { TypedVestClient(self) }
}
impl TypedVestClient {
    pub fn exchange_info(&self, request: &ExchangeInfoRequest) -> Result<ExchangeInfoResponse> {
        serde_json::from_value(self.0.exchange_info(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Vest response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedVestClient(crate::AsyncClient<VestClient>);
#[cfg(feature = "async")]
impl TypedVestClient {
    pub fn into_async(self) -> AsyncTypedVestClient { AsyncTypedVestClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedVestClient {
    pub async fn exchange_info(&self, request: &ExchangeInfoRequest) -> Result<ExchangeInfoResponse> {
        serde_json::from_value(self.0.exchange_info(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Vest response model"))
    }
}
