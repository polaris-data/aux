"""Vendor-native data APIs. Protocol handling remains in Rust."""
from __future__ import annotations

import asyncio
import os
from typing import Any

from . import _native
from . import Subscription, ConnectorError


def _parameters(request: dict[str, Any] | None, parameters: dict[str, Any]) -> dict[str, Any]:
    result = dict(request or {})
    if result.keys() & parameters.keys():
        raise TypeError("duplicate native parameters")
    result.update(parameters)
    return result


class DataStream(Subscription):
    """Native records with bounded batching and deterministic cleanup."""

    async def read_batch(self, limit: int = 256) -> list[Any]:
        try:
            return await self._inner.read_batch(limit)
        except asyncio.CancelledError:
            try:
                await asyncio.shield(self._inner.aclose())
            except ConnectorError:
                pass
            raise


class ByteStream(DataStream):
    """Async iterator over bounded byte chunks, without decoding file contents."""

    async def __anext__(self) -> bytes:
        return await super().__anext__()


class _DataClient:
    @property
    def closed(self) -> bool:
        return self._inner.closed

    async def aclose(self) -> None:
        await self._inner.aclose()

    async def __aenter__(self):
        if self.closed:
            raise _native.ClosedError("client is closed")
        return self

    async def __aexit__(self, *_: Any) -> None:
        await self.aclose()


class TardisClient(_DataClient):
    """Hosted Tardis metadata, raw historical feeds and CSV datasets."""

    def __init__(self, *, api_key: str | None = None,
                 api_url: str = "https://api.tardis.dev/v1",
                 datasets_url: str = "https://datasets.tardis.dev/v1",
                 timeout_ms: int = 10_000) -> None:
        self._inner = _native.TardisClient(api_key=api_key, api_url=api_url,
                                         datasets_url=datasets_url, timeout_ms=timeout_ms)

    async def exchanges(self) -> Any:
        return await self._inner.exchanges()

    async def api_key_info(self) -> Any:
        return await self._inner.api_key_info()

    async def exchange_details(self, exchange: str) -> Any:
        return await self._inner.exchange_details(exchange)

    async def exchange_summary(self, exchange: str) -> Any:
        return await self._inner.exchange_summary(exchange)

    async def symbols(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.symbols(_parameters(request, parameters))

    async def instruments(self, exchange: str, request: dict[str, Any] | None = None,
                          **parameters: Any) -> Any:
        return await self._inner.instruments(_parameters(request, parameters), exchange)

    async def instrument(self, exchange: str, symbol: str) -> Any:
        return await self._inner.instrument(exchange, symbol)

    async def data_feeds(self, request: dict[str, Any]) -> DataStream:
        return DataStream(await self._inner.data_feeds(request))

    async def replay(self, request: dict[str, Any]) -> DataStream:
        return DataStream(await self._inner.replay(request))

    async def dataset(self, request: dict[str, Any]) -> ByteStream:
        return ByteStream(await self._inner.dataset(request))

    async def download_dataset(self, request: dict[str, Any], path: str | os.PathLike[str]) -> int:
        return await self._inner.download_dataset(request, os.fspath(path))


class TardisMachineClient(_DataClient):
    """Caller-managed Tardis Machine; this client never launches the service."""

    def __init__(self, url: str, *, ws_url: str | None = None, timeout_ms: int = 10_000) -> None:
        self._inner = _native.TardisMachineClient(url, ws_url=ws_url, timeout_ms=timeout_ms)

    async def replay(self, options: dict[str, Any] | list[dict[str, Any]]) -> DataStream:
        return DataStream(await self._inner.replay(options))

    async def replay_normalized(self, options: dict[str, Any] | list[dict[str, Any]]) -> DataStream:
        return DataStream(await self._inner.replay_normalized(options))

    async def ws_replay_normalized(self, options: dict[str, Any] | list[dict[str, Any]]) -> DataStream:
        return DataStream(await self._inner.ws_replay_normalized(options))

    async def stream_normalized(self, options: dict[str, Any] | list[dict[str, Any]]) -> DataStream:
        return DataStream(await self._inner.stream_normalized(options))

    async def ws_replay(self, exchange: str, from_date: str, to_date: str,
                        subscribe_messages: list[dict[str, Any]], *, session: str | None = None) -> DataStream:
        return DataStream(await self._inner.ws_replay(exchange, from_date, to_date, subscribe_messages, session))


class DatabentoHistoricalClient(_DataClient):
    """Databento native historical HTTP APIs with exact JSON values."""

    def __init__(self, api_key: str, *, api_url: str = "https://hist.databento.com/v0",
                 timeout_ms: int = 10_000) -> None:
        self._inner = _native.DatabentoHistoricalClient(api_key, api_url=api_url, timeout_ms=timeout_ms)

    async def list_publishers(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.list_publishers(_parameters(request, parameters))

    async def list_datasets(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.list_datasets(_parameters(request, parameters))

    async def list_schemas(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.list_schemas(_parameters(request, parameters))

    async def list_fields(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.list_fields(_parameters(request, parameters))

    async def list_unit_prices(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.list_unit_prices(_parameters(request, parameters))

    async def get_dataset_condition(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.get_dataset_condition(_parameters(request, parameters))

    async def get_dataset_range(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.get_dataset_range(_parameters(request, parameters))

    async def get_record_count(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.get_record_count(_parameters(request, parameters))

    async def get_billable_size(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.get_billable_size(_parameters(request, parameters))

    async def get_cost(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.get_cost(_parameters(request, parameters))

    async def resolve(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.resolve(_parameters(request, parameters))

    async def submit_job(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.submit_job(_parameters(request, parameters))

    async def list_jobs(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.list_jobs(_parameters(request, parameters))

    async def get_job_details(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.get_job_details(_parameters(request, parameters))

    async def list_files(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.list_files(_parameters(request, parameters))

    async def get_range(self, request: dict[str, Any] | None = None, **parameters: Any) -> DataStream:
        return DataStream(await self._inner.get_range(_parameters(request, parameters)))

    async def timeseries(self, request: dict[str, Any] | None = None, **parameters: Any) -> ByteStream:
        return ByteStream(await self._inner.timeseries(_parameters(request, parameters)))

    async def download(self, url: str) -> ByteStream:
        return ByteStream(await self._inner.download(url))

    async def download_to(self, url: str, path: str | os.PathLike[str]) -> int:
        return await self._inner.download_to(url, os.fspath(path))


class DatabentoReferenceClient(_DataClient):
    """Databento native reference HTTP APIs with exact JSON values."""

    def __init__(self, api_key: str, *, api_url: str = "https://hist.databento.com/v0",
                 timeout_ms: int = 10_000) -> None:
        self._inner = _native.DatabentoReferenceClient(api_key, api_url=api_url, timeout_ms=timeout_ms)

    async def list_events(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.list_events(_parameters(request, parameters))

    async def list_enums(self, request: dict[str, Any] | None = None, **parameters: Any) -> Any:
        return await self._inner.list_enums(_parameters(request, parameters))

    async def security_master_get_range(self, request: dict[str, Any] | None = None, **parameters: Any) -> DataStream:
        return DataStream(await self._inner.security_master_get_range(_parameters(request, parameters)))

    async def security_master_get_last(self, request: dict[str, Any] | None = None, **parameters: Any) -> DataStream:
        return DataStream(await self._inner.security_master_get_last(_parameters(request, parameters)))

    async def corporate_actions_get_range(self, request: dict[str, Any] | None = None, **parameters: Any) -> DataStream:
        return DataStream(await self._inner.corporate_actions_get_range(_parameters(request, parameters)))

    async def adjustment_factors_get_range(self, request: dict[str, Any] | None = None, **parameters: Any) -> DataStream:
        return DataStream(await self._inner.adjustment_factors_get_range(_parameters(request, parameters)))


class DatabentoLiveClient(_DataClient):
    """Dedicated native TCP live gateway sessions, with optional intraday replay."""

    def __init__(self, api_key: str, *, timeout_ms: int = 10_000,
                 idle_timeout_ms: int = 60_000) -> None:
        self._inner = _native.DatabentoLiveClient(api_key, timeout_ms=timeout_ms,
                                                idle_timeout_ms=idle_timeout_ms)

    async def connect(self, options: dict[str, Any] | None = None, **parameters: Any) -> DatabentoLiveSession:
        return DatabentoLiveSession(await self._inner.connect(_parameters(options, parameters)))


class DatabentoLiveSession(DataStream):
    """Subscribe, start the session, then iterate metadata and native DBN records."""

    @property
    def session_id(self) -> str:
        return self._inner.session_id or ""

    async def subscribe(self, request: dict[str, Any] | None = None, **parameters: Any) -> None:
        await self._inner.live_subscribe(_parameters(request, parameters), False)

    async def unsubscribe(self, request: dict[str, Any] | None = None, **parameters: Any) -> None:
        await self._inner.live_subscribe(_parameters(request, parameters), True)

    async def start_session(self) -> None:
        await self._inner.live_start()
