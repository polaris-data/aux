"""Generated clients for the native oracle APIs."""
from typing import Any
from . import _native, Subscription
from ._data import _DataClient

class PythProClient(_DataClient):
    def __init__(self, config: dict[str, Any] | None = None, *, timeout_ms: int = 10_000) -> None:
        self._inner = _native.PythProClient(config, timeout_ms=timeout_ms)

    async def live(self) -> dict[str, Any]:
        return await self._inner.live()

    async def ready(self) -> dict[str, Any]:
        return await self._inner.ready()

    async def symbols(self, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.symbols(request)

    async def config(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.config(request)

    async def search(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.search(request)

    async def resolve_symbol(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.resolve_symbol(request)

    async def history(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.history(request)

    async def price(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.price(request)

    async def price_range(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.price_range(request)

    async def publisher_updates(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.publisher_updates(request)

    async def latest_price(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.latest_price(request)

    async def price_at_timestamp(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.price_at_timestamp(request)

    async def subscribe(self, request: dict[str, Any]) -> Subscription:
        return Subscription(await self._inner.subscribe(request))

    async def stream_merkle_roots(self) -> Subscription:
        return Subscription(await self._inner.stream_merkle_roots())

    async def reduce_price(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.reduce_price(request)

    async def publisher_stake_caps(self) -> dict[str, Any]:
        return await self._inner.publisher_stake_caps()

    async def guardian_set_upgrade(self) -> dict[str, Any]:
        return await self._inner.guardian_set_upgrade()

class PythHermesClient(_DataClient):
    def __init__(self, config: dict[str, Any] | None = None, *, timeout_ms: int = 10_000) -> None:
        self._inner = _native.PythHermesClient(config, timeout_ms=timeout_ms)

    async def live(self) -> dict[str, Any]:
        return await self._inner.live()

    async def ready(self) -> dict[str, Any]:
        return await self._inner.ready()

    async def price_feeds(self, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.price_feeds(request)

    async def latest_price_updates(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.latest_price_updates(request)

    async def price_updates_at_timestamp(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.price_updates_at_timestamp(request)

    async def latest_twaps(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.latest_twaps(request)

    async def latest_publisher_stake_caps(self, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.latest_publisher_stake_caps(request)

    async def stream_price_updates(self, request: dict[str, Any]) -> Subscription:
        return Subscription(await self._inner.stream_price_updates(request))

class ChainlinkDataStreamsClient(_DataClient):
    def __init__(self, config: dict[str, Any], *, timeout_ms: int = 10_000) -> None:
        self._inner = _native.ChainlinkDataStreamsClient(config, timeout_ms=timeout_ms)

    async def discovery(self, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.discovery(request)

    async def report(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.report(request)

    async def latest_report(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.latest_report(request)

    async def bulk_reports(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.bulk_reports(request)

    async def report_page(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.report_page(request)

    async def subscribe_reports(self, request: dict[str, Any]) -> Subscription:
        return Subscription(await self._inner.subscribe_reports(request))

    def decode_report(self, full_report: str) -> dict[str, Any]:
        return self._inner.decode_report(full_report)

class ChainlinkCandlestickClient(_DataClient):
    def __init__(self, config: dict[str, Any], *, timeout_ms: int = 10_000) -> None:
        self._inner = _native.ChainlinkCandlestickClient(config, timeout_ms=timeout_ms)

    async def authorize(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.authorize(request)

    async def symbol_info(self, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.symbol_info(request)

    async def groups(self) -> dict[str, Any]:
        return await self._inner.groups()

    async def candle_history(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.candle_history(request)

    async def candle_history_rows(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.candle_history_rows(request)

    async def stream_prices(self, request: dict[str, Any] | None = None) -> Subscription:
        return Subscription(await self._inner.stream_prices(request))
