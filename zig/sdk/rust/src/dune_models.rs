// Generated from src/data/dune/schema/dune.json.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use crate::{DuneClient, Error, Result};

fn present<'de,D:serde::Deserializer<'de>,T:Deserialize<'de>>(d:D) -> std::result::Result<Option<T>,D::Error> { Deserialize::deserialize(d) }
#[derive(Clone, Serialize, Deserialize)]
pub enum Performance {
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecuteSqlRequest {
    #[serde(rename = "sql")]
    pub sql: String,
    #[serde(rename = "performance", default, skip_serializing_if = "Option::is_none")]
    pub performance: Option<Performance>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecuteQueryRequest {
    #[serde(rename = "performance", default, skip_serializing_if = "Option::is_none")]
    pub performance: Option<Performance>,
    #[serde(rename = "query_parameters", default, skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<serde_json::Map<String,serde_json::Value>>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecuteQueryPipelineRequest {
    #[serde(rename = "performance")]
    pub performance: Performance,
    #[serde(rename = "query_parameters", default, skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<serde_json::Map<String,serde_json::Value>>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultOptions {
    #[serde(rename = "allow_partial_results", default, skip_serializing_if = "Option::is_none")]
    pub allow_partial_results: Option<bool>,
    #[serde(rename = "columns", default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<String>,
    #[serde(rename = "filters", default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
    #[serde(rename = "ignore_max_credits_per_request", default, skip_serializing_if = "Option::is_none")]
    pub ignore_max_credits_per_request: Option<bool>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(rename = "sample_count", default, skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<u64>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExecutionResponse {
    #[serde(rename = "execution_id")]
    pub execution_id: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PipelineExecutionResponse {
    #[serde(rename = "pipeline_execution_id")]
    pub pipeline_execution_id: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryResultError {
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExecutionStatus {
    #[serde(rename = "execution_id")]
    pub execution_id: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "query_id", default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<u64>,
    #[serde(rename = "is_execution_finished", default, skip_serializing_if = "Option::is_none")]
    pub is_execution_finished: Option<bool>,
    #[serde(rename = "submitted_at", default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
    #[serde(rename = "execution_started_at", default, skip_serializing_if = "Option::is_none")]
    pub execution_started_at: Option<String>,
    #[serde(rename = "execution_ended_at", default, skip_serializing_if = "Option::is_none")]
    pub execution_ended_at: Option<String>,
    #[serde(rename = "cancelled_at", default, skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<String>,
    #[serde(rename = "expires_at", default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<QueryResultError>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResultMetadata {
    #[serde(rename = "column_names", default, skip_serializing_if = "Option::is_none")]
    pub column_names: Option<Vec<String>>,
    #[serde(rename = "column_types", default, skip_serializing_if = "Option::is_none")]
    pub column_types: Option<Vec<String>>,
    #[serde(rename = "row_count", default, skip_serializing_if = "Option::is_none")]
    pub row_count: Option<u64>,
    #[serde(rename = "total_row_count", default, skip_serializing_if = "Option::is_none")]
    pub total_row_count: Option<u64>,
    #[serde(rename = "result_set_bytes", default, skip_serializing_if = "Option::is_none")]
    pub result_set_bytes: Option<u64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryResultData {
    #[serde(rename = "rows", default, skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<serde_json::Map<String,serde_json::Value>>>,
    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResultMetadata>,
    #[serde(rename = "update_type", default, skip_serializing_if = "Option::is_none")]
    pub update_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QueryResults {
    #[serde(rename = "execution_id")]
    pub execution_id: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "query_id", default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<u64>,
    #[serde(rename = "is_execution_finished", default, skip_serializing_if = "Option::is_none")]
    pub is_execution_finished: Option<bool>,
    #[serde(rename = "submitted_at", default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
    #[serde(rename = "execution_started_at", default, skip_serializing_if = "Option::is_none")]
    pub execution_started_at: Option<String>,
    #[serde(rename = "execution_ended_at", default, skip_serializing_if = "Option::is_none")]
    pub execution_ended_at: Option<String>,
    #[serde(rename = "cancelled_at", default, skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<String>,
    #[serde(rename = "expires_at", default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<QueryResultError>,
    #[serde(rename = "result", default, skip_serializing_if = "Option::is_none")]
    pub result: Option<QueryResultData>,
    #[serde(rename = "next_offset", default, skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<u64>,
    #[serde(rename = "next_uri", default, skip_serializing_if = "Option::is_none")]
    pub next_uri: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CsvResults {
    #[serde(rename = "data")]
    pub data: String,
    #[serde(rename = "next_offset", deserialize_with = "present")]
    pub next_offset: Option<u64>,
    #[serde(rename = "next_uri", deserialize_with = "present")]
    pub next_uri: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CancelExecutionResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecuteSqlParams {
    #[serde(rename = "body")]
    pub body: ExecuteSqlRequest,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecuteQueryParams {
    #[serde(rename = "query_id")]
    pub query_id: u64,
    #[serde(rename = "body")]
    pub body: ExecuteQueryRequest,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecuteQueryPipelineParams {
    #[serde(rename = "query_id")]
    pub query_id: u64,
    #[serde(rename = "body")]
    pub body: ExecuteQueryPipelineRequest,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetExecutionStatusParams {
    #[serde(rename = "execution_id")]
    pub execution_id: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetExecutionResultsParams {
    #[serde(rename = "execution_id")]
    pub execution_id: String,
    #[serde(rename = "allow_partial_results", default, skip_serializing_if = "Option::is_none")]
    pub allow_partial_results: Option<bool>,
    #[serde(rename = "columns", default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<String>,
    #[serde(rename = "filters", default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
    #[serde(rename = "ignore_max_credits_per_request", default, skip_serializing_if = "Option::is_none")]
    pub ignore_max_credits_per_request: Option<bool>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(rename = "sample_count", default, skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<u64>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetExecutionResultsCsvParams {
    #[serde(rename = "execution_id")]
    pub execution_id: String,
    #[serde(rename = "allow_partial_results", default, skip_serializing_if = "Option::is_none")]
    pub allow_partial_results: Option<bool>,
    #[serde(rename = "columns", default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<String>,
    #[serde(rename = "filters", default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
    #[serde(rename = "ignore_max_credits_per_request", default, skip_serializing_if = "Option::is_none")]
    pub ignore_max_credits_per_request: Option<bool>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(rename = "sample_count", default, skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<u64>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetLatestQueryResultsParams {
    #[serde(rename = "query_id")]
    pub query_id: u64,
    #[serde(rename = "allow_partial_results", default, skip_serializing_if = "Option::is_none")]
    pub allow_partial_results: Option<bool>,
    #[serde(rename = "columns", default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<String>,
    #[serde(rename = "filters", default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
    #[serde(rename = "ignore_max_credits_per_request", default, skip_serializing_if = "Option::is_none")]
    pub ignore_max_credits_per_request: Option<bool>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(rename = "sample_count", default, skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<u64>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetLatestQueryResultsCsvParams {
    #[serde(rename = "query_id")]
    pub query_id: u64,
    #[serde(rename = "allow_partial_results", default, skip_serializing_if = "Option::is_none")]
    pub allow_partial_results: Option<bool>,
    #[serde(rename = "columns", default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<String>,
    #[serde(rename = "filters", default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
    #[serde(rename = "ignore_max_credits_per_request", default, skip_serializing_if = "Option::is_none")]
    pub ignore_max_credits_per_request: Option<bool>,
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(rename = "offset", default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    #[serde(rename = "sample_count", default, skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<u64>,
    #[serde(rename = "sort_by", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelExecutionParams {
    #[serde(rename = "execution_id")]
    pub execution_id: String,
}

pub struct TypedDuneClient(DuneClient);
impl DuneClient {
    pub fn typed(self) -> TypedDuneClient { TypedDuneClient(self) }
}
impl TypedDuneClient {
    pub fn cancel_execution(&self, request: &CancelExecutionParams) -> Result<CancelExecutionResponse> {
        serde_json::from_value(self.0.cancel_execution(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub fn execute_query(&self, request: &ExecuteQueryParams) -> Result<ExecutionResponse> {
        serde_json::from_value(self.0.execute_query(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub fn execute_query_pipeline(&self, request: &ExecuteQueryPipelineParams) -> Result<PipelineExecutionResponse> {
        serde_json::from_value(self.0.execute_query_pipeline(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub fn execute_sql(&self, request: &ExecuteSqlParams) -> Result<ExecutionResponse> {
        serde_json::from_value(self.0.execute_sql(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub fn get_execution_results(&self, request: &GetExecutionResultsParams) -> Result<QueryResults> {
        serde_json::from_value(self.0.get_execution_results(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub fn get_execution_results_csv(&self, request: &GetExecutionResultsCsvParams) -> Result<CsvResults> {
        serde_json::from_value(self.0.get_execution_results_csv(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub fn get_execution_status(&self, request: &GetExecutionStatusParams) -> Result<ExecutionStatus> {
        serde_json::from_value(self.0.get_execution_status(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub fn get_latest_query_results(&self, request: &GetLatestQueryResultsParams) -> Result<QueryResults> {
        serde_json::from_value(self.0.get_latest_query_results(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub fn get_latest_query_results_csv(&self, request: &GetLatestQueryResultsCsvParams) -> Result<CsvResults> {
        serde_json::from_value(self.0.get_latest_query_results_csv(&json!(request))?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
}
#[cfg(feature = "async")]
pub struct AsyncTypedDuneClient(crate::AsyncClient<DuneClient>);
#[cfg(feature = "async")]
impl TypedDuneClient {
    pub fn into_async(self) -> AsyncTypedDuneClient { AsyncTypedDuneClient(self.0.into_async()) }
}
#[cfg(feature = "async")]
impl AsyncTypedDuneClient {
    pub async fn cancel_execution(&self, request: &CancelExecutionParams) -> Result<CancelExecutionResponse> {
        serde_json::from_value(self.0.cancel_execution(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub async fn execute_query(&self, request: &ExecuteQueryParams) -> Result<ExecutionResponse> {
        serde_json::from_value(self.0.execute_query(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub async fn execute_query_pipeline(&self, request: &ExecuteQueryPipelineParams) -> Result<PipelineExecutionResponse> {
        serde_json::from_value(self.0.execute_query_pipeline(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub async fn execute_sql(&self, request: &ExecuteSqlParams) -> Result<ExecutionResponse> {
        serde_json::from_value(self.0.execute_sql(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub async fn get_execution_results(&self, request: &GetExecutionResultsParams) -> Result<QueryResults> {
        serde_json::from_value(self.0.get_execution_results(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub async fn get_execution_results_csv(&self, request: &GetExecutionResultsCsvParams) -> Result<CsvResults> {
        serde_json::from_value(self.0.get_execution_results_csv(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub async fn get_execution_status(&self, request: &GetExecutionStatusParams) -> Result<ExecutionStatus> {
        serde_json::from_value(self.0.get_execution_status(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub async fn get_latest_query_results(&self, request: &GetLatestQueryResultsParams) -> Result<QueryResults> {
        serde_json::from_value(self.0.get_latest_query_results(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
    pub async fn get_latest_query_results_csv(&self, request: &GetLatestQueryResultsCsvParams) -> Result<CsvResults> {
        serde_json::from_value(self.0.get_latest_query_results_csv(&json!(request)).await?)
            .map_err(|_| Error::Decode("invalid Dune response model"))
    }
}
