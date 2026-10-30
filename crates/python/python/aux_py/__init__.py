"""Native Hyperliquid and Lighter APIs backed by the same Rust providers."""

from __future__ import annotations

import asyncio
from typing import Any

from . import _native
from ._native import (
    BusyError,
    ClosedError,
    ConnectorError,
    DecodeError,
    ExchangeError,
    TransportError,
    GapError,
    ValidationError,
    AuthenticationError,
    SigningError,
    NonceUncertainError,
    HyperliquidSigner,
    HyperliquidSigningPayload,
    LighterSigner,
    LighterSigningPayload,
    hyperliquid_action_hash,
    validate_hyperliquid_order,
    scale_lighter_decimal,
    hyperliquid_signing_payload,
    hyperliquid_user_signing_payload,
    lighter_auth_signing_payload,
    lighter_transaction_signing_payload,
    __version__,
)

__all__ = [
    "BookSubscription", "HyperliquidClient", "ConnectorError", "TransportError",
    "ExchangeError", "DecodeError", "ValidationError", "ClosedError", "BusyError",
    "LighterClient", "Subscription", "GapError", "AuthenticationError", "SigningError",
    "NonceUncertainError", "HyperliquidSigner", "LighterSigner",
    "HyperliquidSigningPayload", "LighterSigningPayload",
    "hyperliquid_action_hash", "validate_hyperliquid_order", "scale_lighter_decimal",
    "hyperliquid_signing_payload", "hyperliquid_user_signing_payload",
    "lighter_auth_signing_payload", "lighter_transaction_signing_payload",
    "__version__",
]


class HyperliquidClient:
    """Native reads/streams and optional authenticated trading. Defaults to testnet."""

    def __init__(
        self,
        *,
        network: str = "testnet",
        http_url: str | None = None,
        ws_url: str | None = None,
        timeout_ms: int = 10_000,
        subscription_capacity: int = 128,
        signer: HyperliquidSigner | None = None,
        account_address: str | None = None,
        vault_address: str | None = None,
        read_retries: int = 2, reconnect_retries: int = 3,
        initial_backoff_ms: int = 100, max_backoff_ms: int = 2000,
        heartbeat_interval_ms: int = 20_000, idle_timeout_ms: int = 60_000,
        max_message_bytes: int = 8_388_608, reconnect_stable_ms: int = 30_000,
    ) -> None:
        self._inner = _native.HyperliquidClient(
            network=network, http_url=http_url, ws_url=ws_url,
            timeout_ms=timeout_ms, subscription_capacity=subscription_capacity,
            signer=signer, account_address=account_address, vault_address=vault_address,
            read_retries=read_retries, reconnect_retries=reconnect_retries,
            initial_backoff_ms=initial_backoff_ms, max_backoff_ms=max_backoff_ms,
            heartbeat_interval_ms=heartbeat_interval_ms, idle_timeout_ms=idle_timeout_ms,
            max_message_bytes=max_message_bytes, reconnect_stable_ms=reconnect_stable_ms,
        )

    @property
    def closed(self) -> bool:
        return self._inner.closed

    @property
    def authenticated(self) -> bool:
        return self._inner.authenticated

    @property
    def signer_address(self) -> str | None:
        return self._inner.signer_address

    @property
    def account_address(self) -> str | None:
        return self._inner.account_address

    async def meta(self, *, dex: str | None = None) -> dict[str, Any]:
        return await self._inner.meta(dex=dex)

    async def l2_book(
        self, coin: str, *, n_sig_figs: int | None = None, mantissa: int | None = None,
    ) -> dict[str, Any]:
        return await self._inner.l2_book(coin, n_sig_figs=n_sig_figs, mantissa=mantissa)

    async def subscribe_l2_book(self, coin: str) -> BookSubscription:
        return BookSubscription(await self._inner.subscribe_l2_book(coin))

    async def clearinghouse_state(self, user: str, *, dex: str | None = None) -> dict[str, Any]:
        return await self._inner.clearinghouse_state(user, dex=dex)

    async def open_orders(self, user: str, *, dex: str | None = None) -> list[dict[str, Any]]:
        return await self._inner.open_orders(user, dex=dex)

    async def order(self, request: dict[str, Any], *, nonce: int | None = None,
                    expires_after: int | None = None) -> dict[str, Any]:
        return await self._inner.order(request, nonce=nonce, expires_after=expires_after)

    async def cancel(self, request: dict[str, Any], *, nonce: int | None = None,
                     expires_after: int | None = None) -> dict[str, Any]:
        return await self._inner.cancel(request, nonce=nonce, expires_after=expires_after)

    async def cancel_by_cloid(self, request: dict[str, Any], *, nonce: int | None = None,
                              expires_after: int | None = None) -> dict[str, Any]:
        return await self._inner.cancel_by_cloid(request, nonce=nonce, expires_after=expires_after)

    async def subscribe(self, specification: dict[str, Any]) -> Subscription:
        """Yield native envelopes, including subscription acknowledgements."""
        return Subscription(await self._inner.subscribe(specification))

    async def subscribe_trades(self, coin: str) -> Subscription:
        return await self.subscribe({"type": "trades", "coin": coin})

    async def subscribe_order_updates(self, user: str) -> Subscription:
        return await self.subscribe({"type": "orderUpdates", "user": user})

    async def subscribe_user_fills(self, user: str, *, aggregate_by_time: bool | None = None) -> Subscription:
        spec: dict[str, Any] = {"type": "userFills", "user": user}
        if aggregate_by_time is not None:
            spec["aggregateByTime"] = aggregate_by_time
        return await self.subscribe(spec)

    async def subscribe_clearinghouse_state(self, user: str, *, dex: str | None = None) -> Subscription:
        spec = {"type": "clearinghouseState", "user": user}
        if dex is not None:
            spec["dex"] = dex
        return await self.subscribe(spec)

    async def subscribe_open_orders(self, user: str, *, dex: str | None = None) -> Subscription:
        spec = {"type": "openOrders", "user": user}
        if dex is not None:
            spec["dex"] = dex
        return await self.subscribe(spec)

    async def aclose(self) -> None:
        await self._inner.aclose()

    async def __aenter__(self) -> HyperliquidClient:
        if self.closed:
            raise ClosedError("client is closed")
        return self

    async def __aexit__(self, *_: Any) -> None:
        await self.aclose()


class Subscription:
    """Cancellable async iterator of venue-native events with deterministic close."""

    def __init__(self, inner: _native.Subscription) -> None:
        self._inner = inner

    @property
    def closed(self) -> bool:
        return self._inner.closed

    def __aiter__(self) -> Subscription:
        return self

    async def __anext__(self) -> dict[str, Any]:
        try:
            return await self._inner.next()
        except asyncio.CancelledError:
            # Covers cancellation even after Rust produced a result but before
            # asyncio delivered it. Native cleanup also survives a second cancel.
            try:
                await asyncio.shield(self._inner.aclose())
            except ConnectorError:
                pass  # Preserve cancellation; explicit aclose can report errors.
            raise

    async def aclose(self) -> None:
        await self._inner.aclose()

    async def __aenter__(self) -> Subscription:
        if self.closed:
            raise ClosedError("subscription is closed")
        return self

    async def __aexit__(self, *_: Any) -> None:
        await self.aclose()


class BookSubscription(Subscription):
    """Compatibility iterator yielding bare Hyperliquid L2 snapshots."""


class LighterClient:
    """Lighter native APIs; money uses strings or scaled integers throughout."""

    def __init__(self, *, network: str = "testnet", http_url: str | None = None,
                 ws_url: str | None = None, chain_id: int | None = None,
                 timeout_ms: int = 10_000, subscription_capacity: int = 128,
                 signer: LighterSigner | None = None, account_index: int | None = None,
                 api_key_index: int | None = None, read_retries: int = 2,
                 reconnect_retries: int = 3, initial_backoff_ms: int = 100,
                 max_backoff_ms: int = 2000, heartbeat_interval_ms: int = 20_000,
                 idle_timeout_ms: int = 60_000, max_message_bytes: int = 8_388_608, reconnect_stable_ms: int = 30_000) -> None:
        self._inner = _native.LighterClient(
            network=network, http_url=http_url, ws_url=ws_url, chain_id=chain_id,
            timeout_ms=timeout_ms, subscription_capacity=subscription_capacity,
            signer=signer, account_index=account_index, api_key_index=api_key_index,
            read_retries=read_retries, reconnect_retries=reconnect_retries,
            initial_backoff_ms=initial_backoff_ms, max_backoff_ms=max_backoff_ms,
            heartbeat_interval_ms=heartbeat_interval_ms, idle_timeout_ms=idle_timeout_ms,
            max_message_bytes=max_message_bytes, reconnect_stable_ms=reconnect_stable_ms,
        )

    @property
    def closed(self) -> bool:
        return self._inner.closed

    @property
    def authenticated(self) -> bool:
        return self._inner.authenticated

    async def order_books(self, *, market_id: int | None = None) -> dict[str, Any]:
        return await self._inner.order_books(market_id=market_id)

    async def order_book_orders(self, market_id: int, *, limit: int = 50) -> dict[str, Any]:
        return await self._inner.order_book_orders(market_id, limit=limit)

    async def recent_trades(self, market_id: int, *, limit: int = 50) -> dict[str, Any]:
        return await self._inner.recent_trades(market_id, limit=limit)

    async def account(self, *, account_index: int | None = None,
                      l1_address: str | None = None) -> dict[str, Any]:
        return await self._inner.account(account_index=account_index, l1_address=l1_address)

    async def next_nonce(self, account_index: int, api_key_index: int) -> dict[str, Any]:
        """Read native nextNonce; this does not reset quarantined submission state."""
        return await self._inner.next_nonce(account_index, api_key_index)

    async def active_orders(self, *, market_id: int | None = None) -> dict[str, Any]:
        return await self._inner.active_orders(market_id=market_id)

    async def create_order(self, request: dict[str, Any], *, expired_at: int | None = None,
                           attributes: dict[int, int] | None = None) -> dict[str, Any]:
        return await self._inner.create_order(request, expired_at=expired_at, attributes=attributes)

    async def cancel_order(self, request: dict[str, Any], *, expired_at: int | None = None,
                           attributes: dict[int, int] | None = None) -> dict[str, Any]:
        return await self._inner.cancel_order(request, expired_at=expired_at, attributes=attributes)

    async def resync_nonce(self) -> int:
        """Explicit recovery: reconcile pending transactions before calling."""
        return await self._inner.resync_nonce()

    async def auth_token(self, deadline: int) -> str:
        """Return a bearer credential; deadline is Unix seconds."""
        return await self._inner.auth_token(deadline)

    async def subscribe_order_book(self, market_id: int) -> Subscription:
        return Subscription(await self._inner.subscribe_order_book(market_id))

    async def subscribe_trades(self, market_id: int) -> Subscription:
        return Subscription(await self._inner.subscribe_trades(market_id))

    async def subscribe_account(self, account_index: int) -> Subscription:
        return Subscription(await self._inner.subscribe_account(account_index))

    async def subscribe_account_orders(self) -> Subscription:
        return Subscription(await self._inner.subscribe_account_orders())

    async def aclose(self) -> None:
        await self._inner.aclose()

    async def __aenter__(self) -> LighterClient:
        if self.closed:
            raise ClosedError("client is closed")
        return self

    async def __aexit__(self, *_: Any) -> None:
        await self.aclose()


async def _call_signer(callback: Any, payload: Any) -> Any:
    # Execute user code on its asyncio loop, with the initiating contextvars.
    return await callback(payload)


async def _await_signer(future: Any) -> Any:
    return await asyncio.wrap_future(future)
