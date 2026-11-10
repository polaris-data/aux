"""Generated Dune API wrappers. Protocol behavior remains in Rust."""
from __future__ import annotations
from typing import Any
from . import _native
from ._data import _DataClient

class DuneClient(_DataClient):
    """Dune execution/results APIs; every method performs one request."""

    def __init__(self, api_key: str, *, api_url: str = "https://api.dune.com/api/v1",
                 timeout_ms: int = 10_000) -> None:
        self._inner = _native.DuneClient(api_key, api_url=api_url, timeout_ms=timeout_ms)

    async def execute_sql(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.execute_sql(request)

    async def execute_query(self, query_id: int, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.execute_query(query_id, request)

    async def execute_query_pipeline(self, query_id: int, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.execute_query_pipeline(query_id, request)

    async def get_execution_status(self, execution_id: str) -> dict[str, Any]:
        return await self._inner.get_execution_status(execution_id)

    async def get_execution_results(self, execution_id: str, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.get_execution_results(execution_id, request)

    async def get_execution_results_csv(self, execution_id: str, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.get_execution_results_csv(execution_id, request)

    async def get_latest_query_results(self, query_id: int, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.get_latest_query_results(query_id, request)

    async def get_latest_query_results_csv(self, query_id: int, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.get_latest_query_results_csv(query_id, request)

    async def cancel_execution(self, execution_id: str) -> dict[str, Any]:
        return await self._inner.cancel_execution(execution_id)
