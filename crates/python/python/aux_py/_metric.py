"""Metric clients backed by Rust HTTP, ABI and transaction implementations."""
from __future__ import annotations

from . import _native
from ._metric_contracts import CONTRACTS


class _Client:
    @property
    def closed(self):
        return self._inner.closed

    async def aclose(self):
        await self._inner.aclose()

    async def __aenter__(self):
        if self.closed:
            raise _native.ClosedError("client is closed")
        return self

    async def __aexit__(self, *_):
        await self.aclose()


class MetricClient(_Client):
    def __init__(self, *, chain_id, api_url=None, trading_api_key=None, timeout_ms=10_000):
        self._inner = _native.MetricClient(chain_id=chain_id, api_url=api_url, trading_api_key=trading_api_key, timeout_ms=timeout_ms)

    async def chains(self):
        return await self._inner.chains()

    async def factory_settings(self):
        return await self._inner.factory_settings()

    async def metadata(self, request=None):
        return await self._inner.metadata(request)

    async def cross_chain_metadata(self, request=None):
        return await self._inner.cross_chain_metadata(request)

    async def bid_ask(self, pool, *, depth=None):
        return await self._inner.bid_ask(pool, depth=depth)

    async def batch_bid_ask(self, request):
        return await self._inner.batch_bid_ask(request)

    async def route(self, request):
        return await self._inner.route(request)

    async def pamm_quote(self, request):
        return await self._inner.pamm_quote(request)


class MetricChainClient(_Client):
    def __init__(self, *, chain_id, read_rpc, write_rpc=None, signer=None, timeout_ms=10_000, code_hashes=None):
        self._inner = _native.MetricChainClient(chain_id=chain_id, read_rpc=read_rpc, write_rpc=write_rpc, signer=signer, timeout_ms=timeout_ms, code_hashes=code_hashes)

    async def validate_chain(self):
        return await self._inner.validate_chain()

    async def block_number(self):
        return await self._inner.block_number()

    async def block(self, number="latest"):
        return await self._inner.block(number)

    async def native_balance(self, address, *, block="latest"):
        return await self._inner.native_balance(address, block=block)

    async def code(self, address, *, block="latest"):
        return await self._inner.code(address, block=block)

    def contract(self, name, address):
        """Named native reads and transaction builders for a pinned contract ABI."""
        return CONTRACTS[name](self, address)

    def encode_call(self, contract, function, args=()):
        return self._inner.encode_call(contract, function, args)

    def prepare_call(self, contract, function, to, args=(), *, value="0x0"):
        return self._inner.prepare_call(contract, function, to, args, value=value)

    def decode_input(self, contract, function, data):
        return self._inner.decode_input(contract, function, data)

    def decode_output(self, contract, function, data):
        return self._inner.decode_output(contract, function, data)

    def decode_event(self, contract, event, topics, data):
        return self._inner.decode_event(contract, event, topics, data)

    def decode_error(self, contract, data):
        return self._inner.decode_error(contract, data)

    async def call(self, contract, function, to, args=(), *, block="latest", from_address=None, value="0x0"):
        return await self._inner.call(contract, function, to, args, block=block, from_address=from_address, value=value)

    async def liquidity_snapshot(self, pool, lens, bins, *, block="latest"):
        return await self._inner.liquidity_snapshot(pool, lens, bins, block=block)

    async def oracle_stop_loss_status(self, request):
        """Read pool/config/bins/watermarks at one block and evaluate both directions."""
        return await self._inner.oracle_stop_loss_status(request)

    async def prepare_route(self, request):
        return await self._inner.prepare_route(request)

    async def send_transaction(self, request):
        return await self._inner.send_transaction(request)

    async def submit_raw(self, data):
        return await self._inner.submit_raw(data)

    async def receipt(self, hash):
        return await self._inner.receipt(hash)

    async def transaction(self, hash):
        return await self._inner.transaction(hash)

    async def logs(self, filter):
        return await self._inner.logs(filter)

    def pending(self, address):
        return self._inner.pending(address)

    async def reconcile(self, address):
        return await self._inner.reconcile(address)

    async def resync_nonce(self, address, expected_hash):
        return await self._inner.resync_nonce(address, expected_hash)

    async def transaction_count(self, address):
        return await self._inner.transaction_count(address)

    async def gas_price(self):
        return await self._inner.gas_price()

    async def estimate_gas(self, request):
        return await self._inner.estimate_gas(request)


MetricSigner = _native.MetricSigner
