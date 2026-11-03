"""An isolated native Paradex command session with explicit reconciliation."""
from __future__ import annotations
from typing import Any, Mapping, Sequence


class ParadexRpcSession:
    def __init__(self, inner: Any) -> None:
        self._inner = inner

    @property
    def pending(self) -> Mapping[str, Any] | None:
        return self._inner.pending

    async def reconcile(self) -> Any:
        return await self._inner.reconcile()

    async def next_event(self) -> Any:
        return await self._inner.next_event()

    async def create_order(self, request: Mapping[str, Any]) -> Any:
        return await self._inner.create_order(request)

    async def create_batch(self, requests: Sequence[Mapping[str, Any]]) -> Any:
        return await self._inner.create_batch(requests)

    async def modify_order(self, order_id: str, request: Mapping[str, Any]) -> Any:
        return await self._inner.modify_order(order_id, request)

    async def cancel_order(self, order_id: str) -> Any:
        return await self._inner.cancel_order(order_id)

    async def cancel_by_client_id(self, client_id: str, market: str) -> Any:
        return await self._inner.cancel_by_client_id(client_id, market)

    async def cancel_all(self, market: str | None = None) -> Any:
        return await self._inner.cancel_all(market)

    async def cancel_batch(self, order_ids: list[str]) -> Any:
        return await self._inner.cancel_batch(order_ids)

    async def set_cancel_on_disconnect(self, enabled: bool) -> Any:
        return await self._inner.set_cancel_on_disconnect(enabled)

    async def modify_subscription(self, request: Mapping[str, Any]) -> Any:
        return await self._inner.modify_subscription(request)

    async def subscribe_channel(self, specification: Mapping[str, Any]) -> Any:
        return await self._inner.subscribe_channel(specification)

    async def unsubscribe_channel(self, specification: Mapping[str, Any]) -> Any:
        return await self._inner.unsubscribe_channel(specification)

    async def aclose(self) -> None:
        await self._inner.aclose()

    async def __aenter__(self):
        return self

    async def __aexit__(self, *_: Any) -> None:
        await self.aclose()
