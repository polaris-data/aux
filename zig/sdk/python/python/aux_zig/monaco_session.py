"""Manage Monaco session identity and challenge callbacks for the Python client."""
from uuid import uuid4

from . import _call
from .signers import MonacoSessionSigner, _invoke


class MonacoSessions:
    @property
    def authenticated(self):
        self._require_open()
        return bool(self._session and self._session.get("expires_at"))

    @property
    def session_info(self):
        self._require_open()
        if not self._session or "info" not in self._session:
            raise ValueError("Monaco session identity required")
        return {**self._session["info"], "expiresAt": self._session["expires_at"]}

    @property
    def session_signer(self):
        self._require_open()
        if not self._session:
            raise ValueError("Monaco session signer required")
        return MonacoSessionSigner(self._session["seed"])

    def restore_session(self, signer, info, *, request_signing="legacy"):
        self._require_open()
        origin = self._origin or "https://staging.apimonaco.xyz"
        options = dict(address=info["address"], client_id=info["clientId"], seed=signer.export_seed(),
                       signing=request_signing)
        if self._chain_id is not None:
            options["chain_id"] = self._chain_id
        identity = _call(42, step="challenge", options=options, origin=origin)
        if info["sessionPublicKey"] != identity["sessionPublicKey"] or info["chainId"] != int(identity["chainId"]):
            raise ValueError("Monaco session context or key mismatch")
        session = _call(42, step="finish", origin=origin, options=options, response=info)
        self._origin = origin
        self._session = dict(session, info={k:info[k] for k in identity}, instance=uuid4().hex)
        if self._ws_origin:
            self._session["ws_origin"] = self._ws_origin

    async def login(self, wallet=None, client_id=None, *, session_signer=None, invite_code=None,
                    referral_code=None, request_signing="legacy", options=None, sign=None):
        legacy = options is not None or isinstance(wallet, dict)
        if legacy:
            if options is not None and wallet is not None:
                raise TypeError("supply login options once")
            config = dict(wallet if options is None else options)
            callback = sign if sign is not None else client_id
            origin = self._origin or "https://apimonaco.xyz"
        else:
            session_signer = session_signer if session_signer is not None else MonacoSessionSigner()
            config = dict(address=wallet.address, client_id=client_id, seed=session_signer.export_seed(),
                          signing=request_signing, invite_code=invite_code, referral_code=referral_code)
            if self._chain_id is not None:
                config["chain_id"] = self._chain_id
            if self._ws_origin:
                config["ws_origin"] = self._ws_origin
            callback = wallet.sign_message
            origin = self._origin or "https://staging.apimonaco.xyz"
        if not callable(callback):
            raise TypeError("wallet signing callback required")
        context = dict(options=config, origin=origin)
        identity = _call(42, step="challenge", **context)
        # Route challenge and verification through the same origin as the signed session.
        from . import execute
        context["challenge"] = await execute("monaco", "create_challenge", identity, origin=origin)
        context["signature"] = await _invoke(callback, _call(42, step="message", **context))
        context["response"] = await execute("monaco", "verify_signature", _call(42, step="verify", **context), origin=origin)
        session = _call(42, step="finish", **context)
        session["info"] = {**identity, "chainId":int(identity["chainId"])}
        if legacy:
            return type(self)(origin=origin, session=session)
        self._origin = origin
        self._session = dict(session, instance=uuid4().hex)
        return self.session_info
