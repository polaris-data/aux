"""Generated Birdeye wrappers. Protocol and exact values are handled by Rust."""
from __future__ import annotations
from typing import Any
from . import _native
from ._data import _DataClient

class BirdeyeClient(_DataClient):
    def __init__(self, api_key: str, *, chain: str = "solana",
                 api_url: str = "https://public-api.birdeye.so", timeout_ms: int = 10_000) -> None:
        self._inner = _native.BirdeyeClient(api_key, chain=chain, api_url=api_url, timeout_ms=timeout_ms)

    async def price(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.price(request)

    async def multi_price(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.multi_price(request)

    async def multi_price_post(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.multi_price_post(request)

    async def token_trades(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_trades(request)

    async def pair_trades(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.pair_trades(request)

    async def token_trades_by_time(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_trades_by_time(request)

    async def pair_trades_by_time(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.pair_trades_by_time(request)

    async def trades_v3(self, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.trades_v3(request)

    async def token_trades_v3(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_trades_v3(request)

    async def recent_trades(self, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.recent_trades(request)

    async def ohlcv(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.ohlcv(request)

    async def ohlcv_pair(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.ohlcv_pair(request)

    async def ohlcv_base_quote(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.ohlcv_base_quote(request)

    async def ohlcv_v3(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.ohlcv_v3(request)

    async def ohlcv_pair_v3(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.ohlcv_pair_v3(request)

    async def liquidity_ohlc_pair(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.liquidity_ohlc_pair(request)

    async def liquidity_ohlc_token(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.liquidity_ohlc_token(request)

    async def liquidity_history_token(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.liquidity_history_token(request)

    async def liquidity_history_pair(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.liquidity_history_pair(request)

    async def liquidity_latest_token(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.liquidity_latest_token(request)

    async def history_price(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.history_price(request)

    async def historical_price_unix(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.historical_price_unix(request)

    async def price_volume_single(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.price_volume_single(request)

    async def price_volume_multi(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.price_volume_multi(request)

    async def pair_overview(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.pair_overview(request)

    async def pair_overview_multiple(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.pair_overview_multiple(request)

    async def price_stats(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.price_stats(request)

    async def price_stats_multiple(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.price_stats_multiple(request)

    async def token_trades_by_volume(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_trades_by_volume(request)

    async def token_list(self, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.token_list(request)

    async def token_list_v3(self, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.token_list_v3(request)

    async def token_list_scroll(self, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.token_list_scroll(request)

    async def token_overview(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_overview(request)

    async def token_metadata(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_metadata(request)

    async def token_metadata_multiple(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_metadata_multiple(request)

    async def token_market_data(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_market_data(request)

    async def token_market_data_multiple(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_market_data_multiple(request)

    async def token_trade_data(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_trade_data(request)

    async def token_trade_data_multiple(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_trade_data_multiple(request)

    async def token_new_listing(self, request: dict[str, Any] | None = None) -> dict[str, Any]:
        return await self._inner.token_new_listing(request)

    async def token_markets(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_markets(request)

    async def token_security(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_security(request)

    async def token_mint_burn(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_mint_burn(request)

    async def token_exit_liquidity(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_exit_liquidity(request)

    async def token_exit_liquidity_multiple(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.token_exit_liquidity_multiple(request)

    async def trader_trades_by_time(self, request: dict[str, Any]) -> dict[str, Any]:
        return await self._inner.trader_trades_by_time(request)
