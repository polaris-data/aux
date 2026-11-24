"""Prepare and verify Solana transaction messages and submit through a Phoenix RPC client."""
import asyncio
from . import _call

def build_instruction(name, request):
    return _call(47, operation="build_instruction", params={"name": name, "request": request})


def prepare_transaction(request):
    return _call(47,operation="prepare_transaction",params=request)

def sign_transaction(key, prepared, signatures=None):
    return _call(47,operation="sign_transaction",params=dict(key=key,prepared=prepared,signatures=signatures or {}))

def assemble_transaction(prepared, signatures):
    return _call(47,operation="assemble_transaction",params=dict(prepared=prepared,signatures=signatures))

def validate_server_transaction(prepared, transaction_base64):
    _call(47,operation="validate_server_transaction",params=dict(prepared=prepared,transaction_base64=transaction_base64))

def validate_signed_transaction(signed):
    _call(47,operation="validate_signed_transaction",params=signed)

from .lifecycle import ClientLifecycle


class PhoenixChainClient(ClientLifecycle):
    def __init__(self, rpc_url=None, *, endpoint=None):
        from .arguments import alias
        endpoint = alias(rpc_url, endpoint)
        super().__init__()
        self._config = dict(endpoint=endpoint,client=_call(48,operation="new",endpoint=endpoint))

    async def _rpc(self, operation, params):
        return await asyncio.to_thread(_call,48,operation=operation,params=params,**self._config)

    @property
    def authenticated(self):
        self._require_open()
        return False  # This RPC client receives externally signed Solana transactions.

    async def latest_blockhash(self):
        return await self._rpc("latest_blockhash",{})

    async def block_height(self):
        return await self._rpc("block_height",{})

    async def prepare(self, instructions, fee_payer):
        return await self._rpc("prepare",dict(instructions=instructions,fee_payer=fee_payer))

    async def simulate(self, signed):
        return await self._rpc("simulate",signed)

    async def submit(self, signed):
        return await self._rpc("submit",signed)

    @property
    def pending(self):
        self._require_open()
        return _call(48,operation="pending",params={},**self._config)

    def clear_reconciled_submission(self):
        _call(48,operation="clear_reconciled_submission",params={},**self._config)

    async def signature_status(self, signature):
        return await self._rpc("signature_status",dict(signature=signature))

    async def transaction(self, signature):
        return await self._rpc("transaction",dict(signature=signature))
