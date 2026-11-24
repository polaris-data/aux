"""Offline Pascal collateral transfers using the Solana SPL Token program."""

from typing import TypedDict

from . import _call


class DepositParams(TypedDict):
    owner: str
    source: str
    destination: str
    mint: str
    amount: int
    decimals: int
    recent_blockhash: str
    last_valid_block_height: int


class PreparedDeposit(DepositParams):
    message: list[int]


class SignedDeposit(TypedDict):
    signature: str
    transaction_base64: str
    last_valid_block_height: int


def associated_token_address(owner: str, mint: str) -> str:
    return _call(29, owner=owner, mint=mint)


def prepare_deposit(params: DepositParams) -> PreparedDeposit:
    return _call(30, **params)


def sign_deposit(seed: str, prepared: PreparedDeposit) -> SignedDeposit:
    return _call(31, seed=seed, prepared=prepared)

from .lifecycle import ClientLifecycle


class PascalChainClient(ClientLifecycle):
    """Verify SPL token accounts and reconcile each signed deposit on its RPC endpoint."""

    def __init__(self, endpoint=None, mint=None, decimals=6, *, rpc_url=None, signer=None):
        from .arguments import alias
        endpoint = alias(endpoint, rpc_url)
        self._signer = signer
        super().__init__()
        self._config = {"endpoint": endpoint, "mint": mint, "decimals": decimals}
        self._config["client"] = _call(37, operation="new", **self._config)

    async def _rpc(self, operation, **params):
        import asyncio
        return await asyncio.to_thread(_call, 37, operation=operation, params=params, **self._config)

    @property
    def authenticated(self):
        self._require_open()
        return self._signer is not None

    def associated_token_address(self, owner, mint):
        return associated_token_address(owner, mint)

    async def sign_deposit(self, prepared):
        if self._signer is None:
            raise ValueError("PascalSigner required")
        args = {"public_key":self._signer.public_key,"prepared":prepared}
        reviewed = _call(31, **args)
        proof = await self._signer._message(bytes.fromhex(reviewed))
        return _call(31, **args, signature=proof)

    async def account_info(self, address: str) -> dict:
        return await self._rpc("account_info", address=address)

    async def token_balance(self, address: str) -> dict:
        return await self._rpc("token_balance", address=address)

    async def block_height(self) -> int:
        return await self._rpc("block_height")

    async def signature_status(self, signature: str) -> dict:
        return await self._rpc("signature_status", signature=signature)

    async def prepare_deposit(self, owner, source_token_account=None, destination_token_account=None, amount=None, *, source=None, destination=None) -> PreparedDeposit:
        from .arguments import alias
        source = alias(source_token_account, source)
        destination = alias(destination_token_account, destination)
        return await self._rpc("prepare_deposit", owner=owner, source=source, destination=destination, amount=amount)

    async def submit(self, signed: SignedDeposit) -> str:
        return await self._rpc("submit", **signed)

    @property
    def pending(self) -> list[str]:
        self._require_open()
        return _call(37, operation="pending", params={}, **self._config)

    async def reconcile(self, signature: str) -> dict:
        return await self._rpc("reconcile", signature=signature)

    def clear_reconciled_submission(self, signature: str) -> None:
        return _call(37, operation="clear_reconciled_submission", params={"signature": signature}, **self._config)

    async def reconcile_deposit_credit(self, api, owner=None, signature=None, *, request=None) -> dict:
        if isinstance(signature, dict) and request is None:
            signature, request, owner = owner, signature, None
        if request is None:
            request = {"owner": owner}
        elif owner is not None:
            if signature is not None:
                raise TypeError("supply owner or request once")
            signature = owner
        confirmation = await self.reconcile(signature)
        return {"signature": signature, "confirmation": confirmation, "transfers": await api.transfer_history(request)}
