"""Dune REST wire models generated from src/data/dune/schema/dune.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

from typing import Literal
Performance = Literal["small", "medium", "large"]
_ExecuteSqlRequestOptional = TypedDict("_ExecuteSqlRequestOptional", {'performance': 'Performance | None'}, total=False)
class ExecuteSqlRequest(_ExecuteSqlRequestOptional):
    sql: str

_ExecuteQueryRequestOptional = TypedDict("_ExecuteQueryRequestOptional", {'performance': 'Performance | None', 'query_parameters': 'dict[str, Any] | None'}, total=False)
class ExecuteQueryRequest(_ExecuteQueryRequestOptional):
    pass

_ExecuteQueryPipelineRequestOptional = TypedDict("_ExecuteQueryPipelineRequestOptional", {'query_parameters': 'dict[str, Any] | None'}, total=False)
class ExecuteQueryPipelineRequest(_ExecuteQueryPipelineRequestOptional):
    performance: Performance

_ResultOptionsOptional = TypedDict("_ResultOptionsOptional", {'allow_partial_results': 'bool | None', 'columns': 'str | None', 'filters': 'str | None', 'ignore_max_credits_per_request': 'bool | None', 'limit': 'int | None', 'offset': 'int | None', 'sample_count': 'int | None', 'sort_by': 'str | None'}, total=False)
class ResultOptions(_ResultOptionsOptional):
    pass

class ExecutionResponse(TypedDict):
    execution_id: str
    state: str

class PipelineExecutionResponse(TypedDict):
    pipeline_execution_id: str

_QueryResultErrorOptional = TypedDict("_QueryResultErrorOptional", {'message': 'str | None', 'type': 'str | None', 'metadata': 'Any | None'}, total=False)
class QueryResultError(_QueryResultErrorOptional):
    pass

_ExecutionStatusOptional = TypedDict("_ExecutionStatusOptional", {'query_id': 'int | None', 'is_execution_finished': 'bool | None', 'submitted_at': 'str | None', 'execution_started_at': 'str | None', 'execution_ended_at': 'str | None', 'cancelled_at': 'str | None', 'expires_at': 'str | None', 'error': 'QueryResultError | None'}, total=False)
class ExecutionStatus(_ExecutionStatusOptional):
    execution_id: str
    state: str

_ResultMetadataOptional = TypedDict("_ResultMetadataOptional", {'column_names': 'list[str] | None', 'column_types': 'list[str] | None', 'row_count': 'int | None', 'total_row_count': 'int | None', 'result_set_bytes': 'int | None'}, total=False)
class ResultMetadata(_ResultMetadataOptional):
    pass

_QueryResultDataOptional = TypedDict("_QueryResultDataOptional", {'rows': 'list[dict[str, Any]] | None', 'metadata': 'ResultMetadata | None', 'update_type': 'str | None'}, total=False)
class QueryResultData(_QueryResultDataOptional):
    pass

_QueryResultsOptional = TypedDict("_QueryResultsOptional", {'query_id': 'int | None', 'is_execution_finished': 'bool | None', 'submitted_at': 'str | None', 'execution_started_at': 'str | None', 'execution_ended_at': 'str | None', 'cancelled_at': 'str | None', 'expires_at': 'str | None', 'error': 'QueryResultError | None', 'result': 'QueryResultData | None', 'next_offset': 'int | None', 'next_uri': 'str | None'}, total=False)
class QueryResults(_QueryResultsOptional):
    execution_id: str
    state: str

class CsvResults(TypedDict):
    data: str
    next_offset: int | None
    next_uri: str | None

class CancelExecutionResponse(TypedDict):
    success: bool

class ExecuteSqlParams(TypedDict):
    body: ExecuteSqlRequest

class ExecuteQueryParams(TypedDict):
    query_id: int
    body: ExecuteQueryRequest

class ExecuteQueryPipelineParams(TypedDict):
    query_id: int
    body: ExecuteQueryPipelineRequest

class GetExecutionStatusParams(TypedDict):
    execution_id: str

_GetExecutionResultsParamsOptional = TypedDict("_GetExecutionResultsParamsOptional", {'allow_partial_results': 'bool | None', 'columns': 'str | None', 'filters': 'str | None', 'ignore_max_credits_per_request': 'bool | None', 'limit': 'int | None', 'offset': 'int | None', 'sample_count': 'int | None', 'sort_by': 'str | None'}, total=False)
class GetExecutionResultsParams(_GetExecutionResultsParamsOptional):
    execution_id: str

_GetExecutionResultsCsvParamsOptional = TypedDict("_GetExecutionResultsCsvParamsOptional", {'allow_partial_results': 'bool | None', 'columns': 'str | None', 'filters': 'str | None', 'ignore_max_credits_per_request': 'bool | None', 'limit': 'int | None', 'offset': 'int | None', 'sample_count': 'int | None', 'sort_by': 'str | None'}, total=False)
class GetExecutionResultsCsvParams(_GetExecutionResultsCsvParamsOptional):
    execution_id: str

_GetLatestQueryResultsParamsOptional = TypedDict("_GetLatestQueryResultsParamsOptional", {'allow_partial_results': 'bool | None', 'columns': 'str | None', 'filters': 'str | None', 'ignore_max_credits_per_request': 'bool | None', 'limit': 'int | None', 'offset': 'int | None', 'sample_count': 'int | None', 'sort_by': 'str | None'}, total=False)
class GetLatestQueryResultsParams(_GetLatestQueryResultsParamsOptional):
    query_id: int

_GetLatestQueryResultsCsvParamsOptional = TypedDict("_GetLatestQueryResultsCsvParamsOptional", {'allow_partial_results': 'bool | None', 'columns': 'str | None', 'filters': 'str | None', 'ignore_max_credits_per_request': 'bool | None', 'limit': 'int | None', 'offset': 'int | None', 'sample_count': 'int | None', 'sort_by': 'str | None'}, total=False)
class GetLatestQueryResultsCsvParams(_GetLatestQueryResultsCsvParamsOptional):
    query_id: int

class CancelExecutionParams(TypedDict):
    execution_id: str

