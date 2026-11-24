// Generated from src/data/databento/schema/databento_historical.json.

use serde_json::json;
use crate::{HistoricalClient, Error, Result};

pub type Params = serde_json::Map<String,serde_json::Value>;

pub struct TypedHistoricalClient(HistoricalClient);
impl HistoricalClient {
    pub fn typed(self) -> TypedHistoricalClient { TypedHistoricalClient(self) }
}
impl TypedHistoricalClient {
    pub fn get_billable_size(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_billable_size(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn get_cost(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_cost(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn get_dataset_condition(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_dataset_condition(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn get_dataset_range(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_dataset_range(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn get_job_details(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_job_details(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn get_record_count(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_record_count(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn list_datasets(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_datasets(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn list_fields(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_fields(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn list_files(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_files(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn list_jobs(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_jobs(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn list_publishers(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_publishers(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn list_schemas(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_schemas(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn list_unit_prices(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_unit_prices(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn resolve(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.resolve(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub fn submit_job(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.submit_job(&json!(request))?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedHistoricalClient(crate::AsyncClient<HistoricalClient>);
#[cfg(feature = "async")]
impl TypedHistoricalClient {
    pub fn into_async(self) -> AsyncTypedHistoricalClient { AsyncTypedHistoricalClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedHistoricalClient {
    pub async fn get_billable_size(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_billable_size(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn get_cost(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_cost(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn get_dataset_condition(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_dataset_condition(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn get_dataset_range(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_dataset_range(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn get_job_details(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_job_details(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn get_record_count(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.get_record_count(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn list_datasets(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_datasets(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn list_fields(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_fields(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn list_files(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_files(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn list_jobs(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_jobs(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn list_publishers(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_publishers(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn list_schemas(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_schemas(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn list_unit_prices(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.list_unit_prices(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn resolve(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.resolve(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
    pub async fn submit_job(&self, request: &Params) -> Result<serde_json::Value> {
        serde_json::from_value(self.0.submit_job(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid DatabentoHistorical response model"))
    }
}
