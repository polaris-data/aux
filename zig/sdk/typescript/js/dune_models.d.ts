// Generated from src/data/dune/schema/dune.json.
import type { HttpValue } from "./index";
export type Performance = "small" | "medium" | "large";

export interface ExecuteSqlRequest {
  "sql": string;
  "performance"?: Performance | null;
}

export interface ExecuteQueryRequest {
  "performance"?: Performance | null;
  "query_parameters"?: Record<string, HttpValue> | null;
}

export interface ExecuteQueryPipelineRequest {
  "performance": Performance;
  "query_parameters"?: Record<string, HttpValue> | null;
}

export interface ResultOptions {
  "allow_partial_results"?: boolean | null;
  "columns"?: string | null;
  "filters"?: string | null;
  "ignore_max_credits_per_request"?: boolean | null;
  "limit"?: number | bigint | null;
  "offset"?: number | bigint | null;
  "sample_count"?: number | bigint | null;
  "sort_by"?: string | null;
}

export interface ExecutionResponse {
  [field: string]: unknown;
  "execution_id": string;
  "state": string;
}

export interface PipelineExecutionResponse {
  [field: string]: unknown;
  "pipeline_execution_id": string;
}

export interface QueryResultError {
  [field: string]: unknown;
  "message"?: string | null;
  "type"?: string | null;
  "metadata"?: HttpValue | null;
}

export interface ExecutionStatus {
  [field: string]: unknown;
  "execution_id": string;
  "state": string;
  "query_id"?: number | bigint | null;
  "is_execution_finished"?: boolean | null;
  "submitted_at"?: string | null;
  "execution_started_at"?: string | null;
  "execution_ended_at"?: string | null;
  "cancelled_at"?: string | null;
  "expires_at"?: string | null;
  "error"?: QueryResultError | null;
}

export interface ResultMetadata {
  [field: string]: unknown;
  "column_names"?: Array<string> | null;
  "column_types"?: Array<string> | null;
  "row_count"?: number | bigint | null;
  "total_row_count"?: number | bigint | null;
  "result_set_bytes"?: number | bigint | null;
}

export interface QueryResultData {
  [field: string]: unknown;
  "rows"?: Array<Record<string, HttpValue>> | null;
  "metadata"?: ResultMetadata | null;
  "update_type"?: string | null;
}

export interface QueryResults {
  [field: string]: unknown;
  "execution_id": string;
  "state": string;
  "query_id"?: number | bigint | null;
  "is_execution_finished"?: boolean | null;
  "submitted_at"?: string | null;
  "execution_started_at"?: string | null;
  "execution_ended_at"?: string | null;
  "cancelled_at"?: string | null;
  "expires_at"?: string | null;
  "error"?: QueryResultError | null;
  "result"?: QueryResultData | null;
  "next_offset"?: number | bigint | null;
  "next_uri"?: string | null;
}

export interface CsvResults {
  "data": string;
  "next_offset": number | bigint | null;
  "next_uri": string | null;
}

export interface CancelExecutionResponse {
  [field: string]: unknown;
  "success": boolean;
}

export interface ExecuteSqlParams {
  "body": ExecuteSqlRequest;
}

export interface ExecuteQueryParams {
  "query_id": number | bigint;
  "body": ExecuteQueryRequest;
}

export interface ExecuteQueryPipelineParams {
  "query_id": number | bigint;
  "body": ExecuteQueryPipelineRequest;
}

export interface GetExecutionStatusParams {
  "execution_id": string;
}

export interface GetExecutionResultsParams {
  "execution_id": string;
  "allow_partial_results"?: boolean | null;
  "columns"?: string | null;
  "filters"?: string | null;
  "ignore_max_credits_per_request"?: boolean | null;
  "limit"?: number | bigint | null;
  "offset"?: number | bigint | null;
  "sample_count"?: number | bigint | null;
  "sort_by"?: string | null;
}

export interface GetExecutionResultsCsvParams {
  "execution_id": string;
  "allow_partial_results"?: boolean | null;
  "columns"?: string | null;
  "filters"?: string | null;
  "ignore_max_credits_per_request"?: boolean | null;
  "limit"?: number | bigint | null;
  "offset"?: number | bigint | null;
  "sample_count"?: number | bigint | null;
  "sort_by"?: string | null;
}

export interface GetLatestQueryResultsParams {
  "query_id": number | bigint;
  "allow_partial_results"?: boolean | null;
  "columns"?: string | null;
  "filters"?: string | null;
  "ignore_max_credits_per_request"?: boolean | null;
  "limit"?: number | bigint | null;
  "offset"?: number | bigint | null;
  "sample_count"?: number | bigint | null;
  "sort_by"?: string | null;
}

export interface GetLatestQueryResultsCsvParams {
  "query_id": number | bigint;
  "allow_partial_results"?: boolean | null;
  "columns"?: string | null;
  "filters"?: string | null;
  "ignore_max_credits_per_request"?: boolean | null;
  "limit"?: number | bigint | null;
  "offset"?: number | bigint | null;
  "sample_count"?: number | bigint | null;
  "sort_by"?: string | null;
}

export interface CancelExecutionParams {
  "execution_id": string;
}
