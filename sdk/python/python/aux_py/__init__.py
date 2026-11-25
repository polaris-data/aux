"""Python bindings for the native aux Zig protocols."""

from importlib.metadata import version as _distribution_version
from decimal import Decimal
import json
from . import _native
from .lifecycle import ClosedError, ClientLifecycle, ensure_open
from ._native import (
    AuthenticationError, ConnectorError, DecodeError, ExchangeError,
    OutcomeUncertainError, StreamGapError, TransportError, ValidationError,
)


__version__ = _distribution_version("aux-py")


def decode_report(full_report: str) -> dict:
    return json.loads(_native.invoke(1, full_report))


def _encode(value):
    if isinstance(value, Decimal):
        if not value.is_finite():
            raise ValueError("Expected a finite decimal")
        return str(value)
    if isinstance(value, dict):
        if any(not isinstance(k, str) for k in value):
            raise TypeError("JSON object keys must be strings")
        return (
            "{"
            + ",".join(json.dumps(k) + ":" + _encode(v) for k, v in value.items())
            + "}"
        )
    if isinstance(value, (list, tuple)):
        return "[" + ",".join(map(_encode, value)) + "]"
    return json.dumps(value, allow_nan=False)


def _call(op, **args):
    ensure_open()
    return json.loads(_native.invoke(op, _encode(args)), parse_float=Decimal)


class ContractCodec:
    def __init__(self, encode_op, input_op, output_op):
        self._encode, self._input, self._output = encode_op, input_op, output_op

    def encode_contract_call(self, contract, function, args):
        return _call(self._encode, contract=contract, function=function, args=args)

    def decode_contract_input(self, contract, function, data):
        return _call(self._input, contract=contract, function=function, data=data)

    def decode_contract_output(self, contract, function, data):
        return _call(self._output, contract=contract, function=function, data=data)


class MetricCodec(ContractCodec):
    def create_uniform_bins(self, params):
        return _call(44, operation="create_uniform_bins", params=params)

    def prepare_factory_create_pool_params(self, params):
        return _call(44, operation="prepare_factory_create_pool_params", params=params)

    def prepare_create_pool(self, params):
        return _call(44, operation="prepare_create_pool", params=params)

    def prepare_liquidity_addition(self, params):
        return _call(44, operation="prepare_liquidity_addition", params=params)

    def liquidity_native_payment(self, params):
        return _call(44, operation="liquidity_native_payment", params=params)

    def router_multicall(self, params):
        return _call(44, operation="router_multicall", params=params)

    def native_payment(self, params):
        return _call(44, operation="native_payment", params=params)

    def unwrap_output(self, params):
        return _call(44, operation="unwrap_output", params=params)

    def prepare_external_swap(self, params):
        return _call(44, operation="prepare_external_swap", params=params)

    def prepare_swap_with_fallback(self, params):
        return _call(44, operation="prepare_swap_with_fallback", params=params)

    def addition_plan(self, params):
        return _call(44, operation="addition_plan", params=params)

    def weighted_plan(self, params):
        return _call(44, operation="weighted_plan", params=params)

    def removal_plan(self, params):
        return _call(44, operation="removal_plan", params=params)


    def codebook(self):
        return _call(44, operation="codebook", params={})

    def encode_spread(self, params):
        return _call(44, operation="encode_spread", params=params)

    def encode_u64x32(self, params):
        return _call(44, operation="encode_u64x32", params=params)

    def decode_u64x32(self, params):
        return _call(44, operation="decode_u64x32", params=params)

    def pack_oracle_slot(self, params):
        return _call(44, operation="pack_oracle_slot", params=params)

    def oracle_push_calldata(self, params):
        return _call(44, operation="oracle_push_calldata", params=params)

    def feed_id(self, params):
        return _call(44, operation="feed_id", params=params)

    def slot_update_digest(self, params):
        return _call(44, operation="slot_update_digest", params=params)

    def pusher_consent_digest(self, params):
        return _call(44, operation="pusher_consent_digest", params=params)

    def sign_personal_digest(self, params):
        return _call(44, operation="sign_personal_digest", params=params)

    def mul_div(self, params):
        return _call(44, operation="mul_div", params=params)

    def price_to_q64(self, params):
        return _call(44, operation="price_to_q64", params=params)

    def price_x64_to_e18(self, params):
        return _call(44, operation="price_x64_to_e18", params=params)

    def slippage_limit(self, params):
        return _call(44, operation="slippage_limit", params=params)

    def percentage_removal(self, params):
        return _call(44, operation="percentage_removal", params=params)

    def decayed_high_watermark(self, params):
        return _call(44, operation="decayed_high_watermark", params=params)

    def stop_loss_threshold(self, params):
        return _call(44, operation="stop_loss_threshold", params=params)

    def stop_loss_breached(self, params):
        return _call(44, operation="stop_loss_breached", params=params)

    def seconds_until_stop_loss_clears(self, params):
        return _call(44, operation="seconds_until_stop_loss_clears", params=params)

    def stop_loss_metrics(self, params):
        return _call(44, operation="stop_loss_metrics", params=params)

    def evaluate_oracle_stop_loss(self, params):
        return _call(44, operation="evaluate_oracle_stop_loss", params=params)

    def extension_order(self, params):
        return _call(44, operation="extension_order", params=params)

    def direction_bitmap(self, params):
        return _call(44, operation="direction_bitmap", params=params)

    def minimum_operational_liquidity(self, params):
        return _call(44, operation="minimum_operational_liquidity", params=params)


    def decode_contract_error(self, contract, data):
        return _call(5, contract=contract, data=data)

    def decode_contract_event(self, contract, event, topics, data):
        return _call(6, contract=contract, event=event, topics=topics, data=data)


metric = MetricCodec(2, 3, 4)
class PolymarketCodec(ContractCodec):
    def decode_contract_error(self, contract, data):
        return _call(55, contract=contract, data=data)

    def decode_contract_event(self, contract, event, topics, data):
        return _call(56, contract=contract, event=event, topics=topics, data=data)


    def market_asset_id(self, market, outcome):
        return _call(46, operation="market_asset_id", params=dict(market=market, outcome=outcome))

    def market_order_domain(self, market):
        return _call(46, operation="market_order_domain", params=dict(market=market))

    def deployments(self):
        return _call(46, operation="deployments", params=dict())

    def typed_data_hash(self, payload):
        return _call(46, operation="typed_data_hash", params={"payload":payload})

    def order_typed_data(self, order, domain):
        return _call(46, operation="order_typed_data", params={"order":order,"domain":domain})

    def wallet_auth_typed_data(self, chain_id, address, timestamp, nonce=0):
        return _call(46, operation="wallet_auth_typed_data", params=dict(chain_id=chain_id, address=address, timestamp=timestamp, nonce=nonce))

    def perps_op_typed_data(self, chain_id, operation, salt, timestamp):
        return _call(46, operation="perps_op_typed_data", params=dict(chain_id=chain_id, operation=operation, salt=salt, timestamp=timestamp))

    def deposit_wallet_batch_typed_data(self, chain_id, wallet, nonce, deadline, calls):
        return _call(46, operation="deposit_wallet_batch_typed_data", params=dict(chain_id=chain_id, wallet=wallet, nonce=nonce, deadline=deadline, calls=calls))

    def wrap_session_signature(self, session_signer, signature):
        return _call(46, operation="wrap_session_signature", params=dict(session_signer=session_signer, signature=signature))


polymarket = PolymarketCodec(7, 8, 9)


def prepare_request(venue: str, operation: str, params: dict) -> dict:
    """Prepare the wire request, including whether it requires authentication."""
    return _call(10, venue=venue, operation=operation, params=params)


def decode_response(venue: str, status: int, body: str):
    return _call(11, venue=venue, status=status, body=body)


async def execute(
    venue: str,
    operation: str,
    params: dict,
    *,
    origin: str | None = None,
    api_key: str | None = None,
    api_secret: str | None = None,
    chain_id: int | None = None,
    clob: dict | None = None,
    auth: dict | None = None,
    services: dict | None = None,
):
    import asyncio

    if auth and callable(auth.get("sign")):
        import inspect
        auth = dict(auth)
        sign = auth.pop("sign")
        if venue == "pascal" and operation.startswith("write."):
            from .signers import sign_permit_body
            params = {**params,"body":await sign_permit_body(operation, params["body"], auth, sign)}
        arcus = venue == "arcus" and operation in ("place_order","cancel_order")
        if venue == "kalshi" and not operation.startswith("clearing."):
            proof = _call(53, venue=venue, operation=operation, params=params, api_key=api_key, **({"origin":origin} if origin else {}))
            signed = sign(proof["payload"].encode())
            signature = await signed if inspect.isawaitable(signed) else signed
            if not isinstance(signature,(bytes,bytearray,memoryview)):
                raise TypeError("Kalshi signer must return signature bytes")
            auth.update(proof, signature=bytes(signature).hex())
        if arcus or (venue == "hyperliquid" and operation in ("order","cancel","cancel_by_cloid")):
            proof = _call(43 if arcus else 45, operation=operation, params=params, auth=auth)
            signature = sign(proof["payload" if arcus else "digest"])
            auth["signature"] = await signature if inspect.isawaitable(signature) else signature
            if arcus:
                auth["timestamp"] = proof["timestamp"]
            else:
                params = {**params, "options":{**(params.get("options") or {}), "nonce":proof["nonce"]}}
    args = dict(venue=venue, operation=operation, params=params)
    if auth is not None:
        args["auth"] = auth
    if services is not None:
        args["services"] = services
    if clob is not None:
        args["clob"] = clob
    if chain_id is not None:
        args["chain_id"] = chain_id
    if api_secret is not None:
        args["api_secret"] = api_secret
    if api_key is not None:
        args["api_key"] = api_key
    if origin is not None:
        args["origin"] = origin
    return await asyncio.to_thread(_call, 12, **args)


def kalshi_headers(key: str, seed: str, method: str, path: str, timestamp: int) -> dict:
    return _call(22, key=key, seed=seed, method=method, path=path, timestamp=timestamp)


def qfex_signature(secret: str, nonce: str, timestamp: int) -> str:
    return _call(13, secret=secret, nonce=nonce, timestamp=timestamp)


def databento_cram(key: str, challenge: str) -> str:
    return _call(14, key=key, challenge=challenge)


def arcus_sign(seed: str, payload: str) -> dict:
    return _call(15, seed=seed, payload=payload)


def monaco_sign(seed: str, method: str, target: str, body: str, created: int) -> dict:
    return _call(
        16, seed=seed, method=method, target=target, body=body, created=created
    )


def sign_transaction(key: str, prepared: dict) -> dict:
    return _call(17, key=key, prepared=prepared)


class EventStream:
    def __init__(self, native):
        self._native = native
        self.closed = False

    def __aiter__(self):
        return self

    async def __anext__(self):
        import asyncio

        try:
            text = await asyncio.to_thread(self._native.next_json)
        except asyncio.CancelledError:
            await self.close()
            raise
        if text is None:
            await self.close()
            raise StopAsyncIteration
        return json.loads(text, parse_float=Decimal)

    async def close(self):
        import asyncio

        self.closed = True
        await asyncio.to_thread(self._native.close)

    async def aclose(self):
        await self.close()

    async def __aenter__(self):
        return self

    async def __aexit__(self, *_):
        await self.close()


class BookSubscription:
    def __init__(self, stream):
        self._stream = stream

    @property
    def closed(self):
        return self._stream.closed

    def __aiter__(self):
        return self

    async def __anext__(self):
        async for event in self._stream:
            if event.get("channel") == "l2Book":
                return event["data"]
        raise StopAsyncIteration

    async def close(self):
        await self._stream.close()

    aclose = close

    async def __aenter__(self):
        return self

    async def __aexit__(self, *_):
        await self.close()


from .signers import MetricSigner, MonacoSessionSigner, MonacoWalletSigner, PascalSigner, PhoenixSigner, PolymarketSigner
from .arguments import request_params


class ExtendedSigner:
    __slots__ = ("_signature", "_key", "public_key", "_kind")

    def __init__(self, private_key, kind="starknet"):
        if kind not in {"starknet", "evm"}:
            raise ValueError("kind must be starknet or evm")
        self._kind = kind
        self.public_key = _call(59, operation="stark_public" if kind == "starknet" else "ethereum_address", key=private_key)
        self._key, self._signature = private_key, None

    @classmethod
    def from_wallet_signature(cls, signature):
        signer = cls.__new__(cls)
        signer._key, signer._kind = None, "starknet"
        signer.public_key = _call(49, operation="derived_public_key", params={"signature":signature})
        signer._signature = signature
        return signer

    def sign_digest(self, digest):
        if self._kind != "starknet":
            raise ValueError("a Stark signer is required")
        if self._key is not None:
            return _call(59, operation="stark_sign", key=self._key, digest=digest)
        return _call(49, operation="derived_sign", params={"signature":self._signature,"digest":digest})

    async def sign_hash(self, hash):
        r, s = self.sign_digest(hash)
        return {"r":hex(int(r)), "s":hex(int(s))}

    def _order_key(self):
        if self._kind != "starknet":
            raise ValueError("settlement requires a Stark signer")
        return {"stark_key":self._key} if self._key is not None else {"wallet_signature":self._signature}


class _RpcSession(EventStream):
    async def _command(self, operation, params=None):
        import asyncio
        raw = await asyncio.to_thread(self._native.command_json, _encode(dict(operation=operation, params=params)))
        return json.loads(raw, parse_float=Decimal)

    @property
    def pending(self):
        return json.loads(self._native.command_json('{"operation":"pending"}'))

    async def reconcile(self):
        return await self._command("reconcile")

    async def next_event(self):
        return await anext(self, None)

    aclose = EventStream.close


class KalshiRpcSession(_RpcSession):
    async def subscribe(self, request):
        return await self._command("subscribe", request)

    async def unsubscribe(self, request):
        return await self._command("unsubscribe", request)

    async def list_subscriptions(self, request):
        return await self._command("list_subscriptions", request)

    async def update_subscription(self, request):
        return await self._command("update_subscription", request)


class ParadexRpcSession(_RpcSession):
    async def subscribe_channel(self, specification):
        return await self._command("subscribe_channel", specification)

    async def unsubscribe_channel(self, specification):
        return await self._command("unsubscribe_channel", specification)

    async def modify_subscription(self, request):
        return await self._command("modify_subscription", request)

    async def create_order(self, request):
        return await self._command("create_order", request)

    async def create_batch(self, requests):
        return await self._command("create_batch", requests)

    async def modify_order(self, id, request):
        return await self._command("modify_order", {"id": id, "body": request})

    async def cancel_order(self, id):
        return await self._command("cancel_order", {"id": id})

    async def cancel_by_client_id(self, client_id, market):
        return await self._command("cancel_by_client_id", {"client_id": client_id, "market": market})

    async def cancel_all(self, market=None):
        return await self._command("cancel_all", {} if market is None else {"market": market})

    async def cancel_batch(self, order_ids):
        return await self._command("cancel_batch", {"order_ids": order_ids})

    async def set_cancel_on_disconnect(self, enabled):
        return await self._command("set_cancel_on_disconnect", {"enabled": enabled})


class PascalRpcSession(_RpcSession):
    def __init__(self, handle, permit=None):
        super().__init__(handle)
        self._permit = permit

    async def _command(self, operation, params=None):
        if operation in ("place_orders", "cancel_orders", "mass_cancel") and self._permit and callable(self._permit.get("sign")):
            from .signers import public_permit, sign_permit_body
            params = await sign_permit_body(operation, params, public_permit(self._permit), self._permit["sign"])
        return await super()._command(operation, params)

    async def place_orders(self, request):
        return await self._command("place_orders", request)

    async def cancel_orders(self, request):
        return await self._command("cancel_orders", request)

    async def mass_cancel(self, request):
        return await self._command("mass_cancel", request)

    async def ping(self):
        return await self._command("ping")


class HyperdashStream(EventStream):
    async def _send(self, operation, **params):
        import asyncio
        await asyncio.to_thread(self._native.command_json, _encode({"operation":operation, **params}))

    async def subscribe(self, subscription):
        await self._send("subscribe", subscription=subscription)

    async def ack(self, sub, batch):
        await self._send("ack", sub=sub, batch=batch)

    async def unsubscribe(self, sub):
        await self._send("unsubscribe", sub=sub)


class ByteStream(EventStream):
    async def _save(self, path):
        import asyncio
        import os
        try:
            return int(await asyncio.to_thread(self._native.command_json, _encode(dict(operation="save", path=os.fsdecode(path)))))
        except asyncio.CancelledError:
            self._native.close()
            raise

    async def __anext__(self) -> bytes:
        import asyncio
        try:
            chunk = await asyncio.to_thread(self._native.next_bytes)
        except asyncio.CancelledError:
            self._native.close()
            raise
        if chunk is None:
            raise StopAsyncIteration
        return chunk


class DataStream(EventStream):
    async def read_batch(self, limit: int = 256) -> list:
        import asyncio
        try:
            reply = await asyncio.to_thread(self._native.command_json, _encode(dict(operation="read_batch", limit=limit)))
        except asyncio.CancelledError:
            self._native.close()
            raise
        return json.loads(reply, parse_float=Decimal)


class MachineStream(DataStream):
    async def send(self, message):
        import asyncio
        try:
            await asyncio.to_thread(self._native.command_json, _encode({"operation":"send","message":message}))
        except asyncio.CancelledError:
            self._native.close()
            raise


PriceStream = EventStream


async def _open_stream(venue, operation, params, origin, key=None, secret=None, auth=None, clob=None,endpoints=None):
    import asyncio

    config = dict(venue=venue, operation=operation, params=params)
    if origin is not None:
        config["origin"] = origin
    if key is not None:
        config["api_key"] = key
    if secret is not None:
        config["api_secret"] = secret
    if auth is not None:
        config["auth"] = {k:v for k,v in auth.items() if k != "sign"}
    if clob is not None:
        config["clob"] = clob
    if endpoints is not None:
        config["endpoints"] = endpoints
    handle = await asyncio.to_thread(_native.PriceStream.open, _encode(config))
    if operation == "open_rpc":
        if venue == "pascal":
            return PascalRpcSession(handle, auth)
        return {"derive":DeriveRpcSession, "polymarket":PolymarketRfqRpcSession, "polymarket_perps":PolymarketPerpsRpcSession, "paradex":ParadexRpcSession, "kalshi":KalshiRpcSession, "kalshi_perps":KalshiRpcSession}[venue](handle)
    if venue == "qfex" and operation == "trading_session":
        return QfexTradingSession(handle)
    if venue == "hyperdash" and operation == "open_stream":
        return HyperdashStream(handle)
    if venue == "tardis":
        if operation == "dataset":
            return ByteStream(handle)
        if operation in ("ws_replay","ws_replay_normalized","stream_normalized"):
            return MachineStream(handle)
        return DataStream(handle)
    if venue == "databento" and operation in ("timeseries", "download"):
        return ByteStream(handle)
    return DataStream(handle) if venue == "databento" else EventStream(handle)


from .rpc_generated import DeriveRpcSession, PolymarketPerpsRpcSession, PolymarketRfqRpcSession
from .clients import *

EventDecoder = _native.EventDecoder


def decode_dbn_file(data: bytes):
    """Yield the metadata event and each DBN record (versions 1 through 3) in file order."""
    stream = _native.PriceStream.open_dbn(data)
    try:
        while (record := stream.next_json()) is not None:
            yield json.loads(record)
    finally:
        stream.close()


def decode_dbn_record(record: bytes, *, ts_out: bool = False) -> dict:
    return _call(18, data=record.hex(), ts_out=ts_out)


def decode_sbe(frame: bytes) -> dict:
    return _call(19, data=frame.hex())

from .derive import Action as DeriveAction, LoginProof as DeriveLoginProof, SignedAction as DeriveSignedAction


def derive_login(key: str, owner: str, timestamp: int) -> DeriveLoginProof:
    return _call(23, key=key, owner=owner, timestamp=timestamp)


def derive_sign_action(key: str, action: DeriveAction) -> DeriveSignedAction:
    return _call(24, **action, key=key)


class EvmReader(ClientLifecycle):
    """Read an EVM chain at the given JSON-RPC endpoint."""

    def __init__(self, endpoint: str):
        super().__init__()
        self._endpoint = endpoint

    async def _read(self, operation: str, params: dict):
        import asyncio
        return await asyncio.to_thread(_call, 27, endpoint=self._endpoint, operation=operation, params=params)

    async def chain_id(self) -> int:
        return await self._read("chain_id", {})

    async def code(self, address: str, block: str = "latest") -> str:
        return await self._read("code", {"address": address, "block": block})

    async def eth_call(self, to: str, data: str, block: str, sender: str | None = None, value: str = "0x0") -> str:
        return await self._read("eth_call", {"to": to, "data": data, "block": block, "from": sender, "value": value})

    async def logs(self, filter: dict) -> list:
        return await self._read("logs", {"filter": filter})

    async def transaction_count(self, address: str) -> int:
        return await self._read("transaction_count", {"address": address})

    async def receipt(self, hash: str) -> dict | None:
        return await self._read("receipt", {"hash": hash})

    async def transaction(self, hash: str) -> dict | None:
        return await self._read("transaction", {"hash": hash})

    async def estimate_gas(self, request: dict) -> int:
        return await self._read("estimate_gas", {"request": request})

    async def block_number(self) -> int:
        return await self._read("block_number", {})

    async def block(self, number=None, *, block=None) -> dict | None:
        value = request_params(number, block) if number is not None or block is not None else "latest"
        return await self._read("block", {"block": value})

    async def native_balance(self, address: str, block: str = "latest") -> int:
        return await self._read("native_balance", {"address": address, "block": block})

    async def gas_price(self) -> int:
        return await self._read("gas_price", {})


from . import solana
from .solana import PascalChainClient


class EvmChainClient(EvmReader):
    """Submit to an explicit write RPC and reconcile ambiguous sends by sender and hash."""

    def __init__(self, chain_id: int, read_rpc: str, write_rpc: str, *, code_hashes: dict[str, str] | None = None):
        self._config = dict(chain_id=chain_id, read_rpc=read_rpc, write_rpc=write_rpc, code_hashes=dict(code_hashes or {}))
        super().__init__(read_rpc)

    @property
    def reader(self):
        return self

    async def _rpc(self, operation, **params):
        import asyncio
        return await asyncio.to_thread(_call, 38, operation=operation, params=params, **self._config)

    async def validate_chain(self) -> None:
        return await self._rpc("validate_chain")

    async def send_transaction(self, prepared: dict, key: str) -> str:
        return await self._rpc("send_transaction", prepared=prepared, key=key)

    async def submit_raw(self, data=None, *, signed=None) -> str:
        return await self._rpc("submit_raw", signed=request_params(data, signed))

    def pending(self, address: str) -> dict | None:
        return _call(38, operation="pending", params={"address":address}, **self._config)

    async def reconcile(self, address: str) -> bool:
        return await self._rpc("reconcile", address=address)

    async def resync_nonce(self, address: str, expected_hash=None, *, expected_pending_hash=None) -> int:
        return await self._rpc("resync_nonce", address=address, hash=request_params(expected_hash, expected_pending_hash))


class _ContractChainClient(EvmChainClient):
    def __init__(self, chain_id, read_rpc, write_rpc=None, *, signer=None, code_hashes=None):
        super().__init__(chain_id, read_rpc, write_rpc or read_rpc, code_hashes=code_hashes)
        self._signer = signer

    async def send_transaction(self, request=None, key=None, *, prepared=None):
        payload = request_params(request, prepared)
        if key is not None:
            return await super().send_transaction(payload, key)
        signer = self._signer
        if signer is None:
            raise ValueError("an EVM signer is required")
        if signer._key is not None:
            return await super().send_transaction(payload, signer._key)
        digest = await self._rpc("prepare_external", address=signer.address, prepared=payload)
        signature = await signer.sign_digest(digest)
        return await self._rpc("send_external", address=signer.address, prepared=payload, signature=signature)

    def encode_call(self, contract, function, args=()):
        return self._codec.encode_contract_call(contract, function, args)

    def prepare_call(self, contract, function, to, args=(), *, value="0x0"):
        return _call(self._prepare_op, contract=contract, function=function, to=to, args=args, value=value)

    def decode_input(self, contract, function, data):
        return self._codec.decode_contract_input(contract, function, data)

    def decode_output(self, contract, function, data):
        return self._codec.decode_contract_output(contract, function, data)

    def decode_error(self, contract, data):
        return self._codec.decode_contract_error(contract, data)

    def decode_event(self, contract, event, topics, data):
        return self._codec.decode_contract_event(contract, event, topics, data)

    async def call(self, contract, function, to, args=(), *, block="latest", from_address=None, value="0x0"):
        prepared = self.prepare_call(contract, function, to, args, value=value)
        data = await self.reader.eth_call(prepared["to"], prepared["data"], block, from_address, prepared["value"])
        return self.decode_output(contract, function, data)

class PolymarketChainClient(_ContractChainClient):
    _codec = polymarket
    _prepare_op = 54

    @property
    def authenticated(self):
        self._require_open()
        return self._signer is not None


class MetricChainClient(_ContractChainClient):
    _codec = metric
    _prepare_op = 52

    def contract(self, name, address):
        from .metric_contracts import CONTRACTS
        return CONTRACTS[name](self, address)

    async def _metric_read(self, operation, params):
        import asyncio
        return await asyncio.to_thread(_call, 51, endpoint=self._config["read_rpc"], operation=operation, params=params)

    async def liquidity_snapshot(self, pool, lens, bins, *, block="latest"):
        return await self._metric_read("liquidity_snapshot",dict(pool=pool,lens=lens,bins=bins,block=block))

    async def oracle_stop_loss_status(self, request):
        return await self._metric_read("oracle_stop_loss_status", request)

    async def prepare_route(self, request):
        return await self._metric_read("prepare_route", request)


class LiveClient(ClientLifecycle):
    """Connect to a Databento Live gateway and authenticate with CRAM."""

    def __init__(self, api_key: str, *, timeout_ms=10_000, idle_timeout_ms=60_000):
        super().__init__()
        if any(type(ms) is not int or not 0 < ms < 2**64 for ms in (timeout_ms, idle_timeout_ms)):
            raise ValidationError("Databento timeouts require positive milliseconds")
        self._key = api_key
        self._timeout_ms = timeout_ms
        self._idle_timeout_ms = idle_timeout_ms

    async def connect(self, options: dict | None = None, **parameters):
        from .arguments import merge
        options = merge(options, parameters)
        import asyncio
        config = dict(venue="databento", operation="connect", api_key=self._key, params=options,
                      timeout_ms=self._timeout_ms, idle_timeout_ms=self._idle_timeout_ms)
        handle = await asyncio.to_thread(_native.PriceStream.open, _encode(config))
        session = LiveSession(handle)
        session.session_id = await session._command("session_id")
        return session


class _CommandStream(EventStream):

    async def _command(self, operation: str, **params):
        import asyncio
        try:
            reply = await asyncio.to_thread(self._native.command_json, _encode(dict(operation=operation, **params)))
        except asyncio.CancelledError:
            self._native.close()
            raise
        return json.loads(reply, parse_float=Decimal)

class QfexTradingSession(_CommandStream):
    async def subscribe_account(self):
        await self._command("subscribe_account")

    async def add_order(self, order):
        return await self._command("add_order", params=order)

    async def cancel_order(self, order):
        return await self._command("cancel_order", params=order)

    async def user_orders(self, request):
        return await self._command("user_orders", params=request)


class LiveSession(_CommandStream):
    session_id: str

    async def subscribe(self, request=None, *, subscription=None, **parameters):
        from .arguments import merge, request_params
        await self._command("subscribe", subscription=merge(request_params(request, subscription), parameters))

    async def unsubscribe(self, request=None, *, subscription=None, **parameters):
        from .arguments import merge, request_params
        await self._command("unsubscribe", subscription=merge(request_params(request, subscription), parameters))

    async def start_session(self):
        await self._command("start_session")

    async def read_batch(self, limit: int = 256) -> list:
        return await self._command("read_batch", limit=limit)

    async def metadata(self):
        return await self._command("metadata")


def extended_settlement(kind: str, chain_id: str, public_key: str, message: dict, stark_key: str | None = None) -> dict:
    """Hash an Extended settlement and optionally sign it with its matching Stark key."""
    args = dict(kind=kind, chain_id=chain_id, public_key=public_key, message=message)
    if stark_key is not None:
        args["stark_key"] = stark_key
    return _call(39, **args)


def extended_prepare_order(chain_id: str, public_key: str, body: dict, context: dict) -> dict:
    return _call(40, chain_id=chain_id, public_key=public_key, body=body, context=context)


def extended_sign_order(chain_id: str, stark_key: str, body: dict, context: dict) -> dict:
    return _call(40, chain_id=chain_id, stark_key=stark_key, body=body, context=context)

from .phoenix import PhoenixChainClient

DatabentoHistoricalClient = HistoricalClient
DatabentoReferenceClient = ReferenceClient
DatabentoLiveClient = LiveClient
