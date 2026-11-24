// Generated from src/data/databento/schema/databento_reference.json.

use serde_json::json;
use crate::{ReferenceClient, Error, Result};

pub type Params = serde_json::Map<String,serde_json::Value>;

pub struct TypedReferenceClient(ReferenceClient);
impl ReferenceClient {
    pub fn typed(self) -> TypedReferenceClient { TypedReferenceClient(self) }
}
impl TypedReferenceClient {
    pub fn list_enums(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_enums(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoReference response model"))
    }
    pub fn list_events(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_events(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoReference response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedReferenceClient(crate::AsyncClient<ReferenceClient>);
#[cfg(feature = "async")]
impl TypedReferenceClient {
    pub fn into_async(self) -> AsyncTypedReferenceClient { AsyncTypedReferenceClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedReferenceClient {
    pub async fn list_enums(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_enums(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoReference response model"))
    }
    pub async fn list_events(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_events(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoReference response model"))
    }
}
