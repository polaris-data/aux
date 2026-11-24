"""Language-native signer callbacks backed by the Zig key and signature codecs."""
from inspect import isawaitable
from secrets import token_hex
from . import _call


def _sign(operation, **fields):
    return _call(59, operation=operation, **fields)


async def _invoke(callback, payload):
    result = callback(payload)
    return await result if isawaitable(result) else result


def _identity(value, size):
    if not isinstance(value, str):
        raise ValueError("expected a hexadecimal signer identity")
    raw = value.removeprefix("0x")
    if len(raw) != size * 2 or len(bytes.fromhex(raw)) != size:
        raise ValueError("invalid signer identity")
    return raw.lower()


class _EthereumSigner:
    def __init__(self, private_key):
        self._address = _sign("ethereum_address", key=private_key)
        self._key = private_key
        self._callback = None

    @property
    def address(self):
        return self._address

    @classmethod
    def remote(cls, address, callback):
        if not callable(callback):
            raise TypeError("signer callback must be callable")
        signer = cls.__new__(cls)
        signer._address = "0x" + _identity(address, 20)
        signer._key = None
        signer._callback = callback
        return signer

    async def _signed(self, digest, payload):
        if self._callback is None:
            return _sign("sign_digest", key=self._key, digest=digest)
        proof = await _invoke(self._callback, payload)
        return _sign("verify_digest", address=self.address, digest=digest, signature=proof)


class MetricSigner(_EthereumSigner):
    async def sign_digest(self, digest):
        return await self._signed(digest, {"digest":digest, "address":self.address, "scheme":"evm_digest"})

    async def sign_personal_digest(self, digest):
        return await self.sign_digest(_sign("personal_hash", message=_identity(digest, 32)))

    async def sign_transaction(self, request):
        digest = _call(17, prepared=request, digest_only=True)
        signature = await self.sign_digest(digest)
        signed = _call(17, prepared=request, address=self.address, signature=signature)
        return {"hash":signed["hash"], "bytes":signed["signed"], "signer":signed["address"],
                "nonce":request["nonce"], "chain_id":request["chainId"]}


class MonacoWalletSigner(_EthereumSigner):
    async def sign_message(self, message):
        digest = _sign("personal_hash", message=message.encode().hex())
        return await self._signed(digest, message)


class MonacoSessionSigner:
    def __init__(self, seed=None):
        self._seed = token_hex(32) if seed is None else _identity(seed, 32)
        self._public_key = _sign("ed25519_public", key=self._seed)
        self._callback = None

    @classmethod
    def remote(cls, public_key, callback):
        if not callable(callback):
            raise TypeError("signer callback must be callable")
        signer = cls.__new__(cls)
        signer._public_key = _identity(public_key, 32)
        signer._callback = callback
        signer._seed = None
        return signer

    @property
    def public_key(self):
        return self._public_key

    def export_seed(self):
        if self._seed is None:
            raise ValueError("remote signer has no local seed")
        return self._seed

    async def sign(self, message):
        data = bytes(message)
        if self._callback is None:
            return _sign("ed25519_sign", key=self._seed, message=data.hex())
        proof = await _invoke(self._callback, data)
        return _sign("ed25519_verify", public_key=self._public_key, signature=proof, message=data.hex())


class PolymarketSigner(MetricSigner):
    async def sign_typed_data(self, payload):
        from . import polymarket
        return await self.sign_digest(polymarket.typed_data_hash(payload))

    async def sign_order(self, order, *, system="ctf", negative_risk=False):
        from . import polymarket
        if system not in {"ctf", "v1", "v2"}:
            raise ValueError("system must be ctf or v2")
        domain = polymarket.market_order_domain({"version":"v2" if system == "v2" else "v1", "negRisk":negative_risk})
        proof = await self.sign_typed_data(polymarket.order_typed_data(order, domain))
        return _call(46, operation="order_signature",
                     params={"address":self.address,"order":order,"domain":domain,"signature":proof})


class _SolanaSigner:
    def __init__(self, key):
        self._public_key = _sign("solana_public", key=key)
        self._key = key
        self._callback = None

    @property
    def public_key(self):
        return self._public_key

    @classmethod
    def remote(cls, public_key, callback):
        if not callable(callback):
            raise TypeError("signer callback must be callable")
        public_key = _sign("solana_validate", public_key=public_key)
        signer = cls.__new__(cls)
        signer._key = None
        signer._public_key = public_key
        signer._callback = callback
        return signer

    async def _message(self, data):
        import base64
        if self._key is not None:
            return _sign("solana_sign", key=self._key, message=data.hex())
        payload = {"message_base64":base64.b64encode(data).decode("ascii"),
                   "public_key":self.public_key,"scheme":self._scheme}
        if self._scheme == "pascal_ed25519":
            payload["message_hex"] = data.hex()
        proof = await _invoke(self._callback, payload)
        return _sign("solana_verify", public_key=self.public_key, message=data.hex(), signature=proof)


class PascalSigner(_SolanaSigner):
    _scheme = "pascal_ed25519"

    def __init__(self, private_key_hex):
        super().__init__(private_key_hex)

    async def sign_request(self, owner, deployment_id, kind, request, *, client_ts_ms, recv_window_ms=5000):
        auth = {"public_key":self.public_key,"owner":owner,"deployment_id":deployment_id,"recv_window_ms":recv_window_ms}
        args = {"operation":kind,"body":request,"timestamp":client_ts_ms,"auth":auth}
        message = _call(32, **args)
        auth["signature"] = await self._message(bytes.fromhex(message))
        return _call(32, **args)


class PhoenixSigner(_SolanaSigner):
    _scheme = "phoenix_ed25519"

    def __init__(self, private_key):
        super().__init__(private_key)

    async def sign_message(self, message_base64):
        import base64
        return await self._message(base64.b64decode(message_base64, validate=True))

    async def sign_transaction(self, prepared, signatures=None):
        args = {"prepared":prepared,"public_key":self.public_key,"signatures":signatures or {}}
        message = _call(47, operation="sign_transaction", params=args)
        args["signature"] = await self.sign_message(message)
        return _call(47, operation="sign_transaction", params=args)


def public_permit(permit):
    return {k:v for k,v in permit.items() if k != "sign"} if permit is not None else None


async def sign_permit_body(operation, body, identity, sign):
    from .lifecycle import ensure_open
    proof = _call(32, step="prepare_external", operation=operation, body=body, auth=identity)
    rows = body if isinstance(body, list) else [body]
    signed = []
    for row, payload in zip(rows, proof["messages"], strict=True):
        ensure_open()
        signature = await _invoke(sign, bytes.fromhex(payload))
        signed.append(_call(32, operation=operation, body=row, timestamp=proof["timestamp"],
                            auth={**identity,"signature":signature}))
    return signed if isinstance(body, list) else signed[0]
