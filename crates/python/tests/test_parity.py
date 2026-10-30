"""Native API/signing parity with Rust, using only local servers and fixtures."""
from __future__ import annotations

import asyncio
import base64
import contextvars
import copy
import itertools
import json
import time
import unittest
import weakref
from decimal import Decimal
from pathlib import Path
from urllib.parse import parse_qs, urlsplit

from websockets.asyncio.server import serve
from websockets.exceptions import ConnectionClosed

from aux_py import (
    AuthenticationError, BusyError, ClosedError, DecodeError, ExchangeError, GapError,
    HyperliquidClient, HyperliquidSigner, LighterClient, LighterSigner,
    NonceUncertainError, SigningError, TransportError, ValidationError,
    hyperliquid_action_hash, scale_lighter_decimal, validate_hyperliquid_order,
    hyperliquid_signing_payload, hyperliquid_user_signing_payload,
    lighter_auth_signing_payload, lighter_transaction_signing_payload,
)

CRATES = Path(__file__).resolve().parents[2]
HL = CRATES / "exchanges/hyperliquid/tests/fixtures"
LT = CRATES / "exchanges/lighter/tests/fixtures"
HL_VECTORS = json.loads((HL / "signing_vectors.json").read_text())
LT_VECTORS = json.loads((LT / "signing_vectors.json").read_text())["vectors"]
HL_KEY = HL_VECTORS["public_test_private_key"]  # Public reference data only.
LT_KEY = LT_VECTORS[0]["key"]
ADDRESS = "0x0000000000000000000000000000000000000037"
ACCOUNTS = itertools.count(8_000_000)  # Isolate process-wide Rust nonce state.


def fixture(root, name):
    return json.loads((root / name).read_text())


async def bounded(awaitable):
    return await asyncio.wait_for(awaitable, 5)


class SigningParity(unittest.IsolatedAsyncioTestCase):
    async def test_all_hyperliquid_reference_actions_and_both_signing_schemes(self):
        signer = HyperliquidSigner(HL_KEY)
        self.assertEqual(signer.address, HL_VECTORS["signer_address"])
        for v in HL_VECTORS["l1"]:
            with self.subTest(v["name"]):
                options = dict(nonce=v["nonce"], vault_address=v["vault_address"], expires_after=v["expires_after"])
                signed = await signer.sign_action(v["action"], network=v["network"].lower(), **options)
                self.assertEqual(signed["signature"], v["signature"])
                self.assertEqual(signed["action"], v["action"])
                self.assertEqual(hyperliquid_action_hash(v["action"], **options), v["action_hash"].removeprefix("0x"))
                payload = hyperliquid_signing_payload(v["action"], network=v["network"].lower(), **options)
                self.assertEqual(payload.digest.hex(), v["digest"].removeprefix("0x"))
                self.assertEqual(payload.recover_address(v["signature"]), signer.address)
        for v in HL_VECTORS["user_signed"]:
            with self.subTest(v["name"], network=v["network"]):
                action = dict(v["action"], type="usdSend" if v["name"] == "usd_send" else "approveAgent")
                signature = await signer.sign_user_action(action, network=v["network"].lower(), signature_chain_id=v["signature_chain_id"])
                self.assertEqual(signature, v["signature"])
                payload = hyperliquid_user_signing_payload(action, network=v["network"].lower(), signature_chain_id=v["signature_chain_id"])
                self.assertEqual(payload.digest.hex(), v["digest"].removeprefix("0x"))
                self.assertEqual(payload.recover_address(signature), signer.address)

    async def test_lighter_reference_hashes_public_keys_and_go_signatures(self):
        for v in LT_VECTORS:
            if v["kind"] == "auth":
                payload = lighter_auth_signing_payload(account_index=v["account_index"], api_key_index=v["api_key_index"], deadline=v["deadline"], chain_id=v["chain_id"])
                self.assertEqual(payload.digest.hex(), v["digest"])
                self.assertTrue(payload.verify(v["signature"], v["public_key"]))
                local = LighterSigner(v["key"])
                self.assertTrue(payload.verify(await local.sign(payload), local.public_key))
                continue
            with self.subTest(v["kind"], chain=v["chain_id"], key=v["public_key"]):
                local = LighterSigner(v["key"])
                self.assertEqual(local.public_key, v["public_key"])
                seen = []

                async def go_signature(payload):
                    self.assertEqual(payload.digest.hex(), v["digest"])
                    self.assertEqual(payload.chain_id, v["chain_id"])
                    self.assertEqual(payload.account_index, v["tx_info"]["AccountIndex"])
                    self.assertEqual(payload.purpose["nonce"], v["tx_info"]["Nonce"])
                    seen.append(payload)
                    return v["signature"]

                remote = LighterSigner.remote(v["public_key"], go_signature)
                info = v["tx_info"]
                opts = dict(account_index=info["AccountIndex"], api_key_index=info["ApiKeyIndex"], nonce=info["Nonce"], chain_id=v["chain_id"], expired_at=info["ExpiredAt"], attributes={int(k): val for k, val in (info["L2TxAttributes"] or {}).items()})
                signed = await remote.sign_transaction(v["kind"], info, **opts)
                self.assertEqual(signed["digest"], v["digest"])
                self.assertEqual(signed["tx_info"], info)
                self.assertEqual(len(seen), 1)
                local_signed = await local.sign_transaction(v["kind"], info, **opts)
                self.assertEqual(local_signed["digest"], v["digest"])
                self.assertEqual(len(base64.b64decode(local_signed["tx_info"]["Sig"])), 80)
                payload = lighter_transaction_signing_payload(v["kind"], info, **opts)
                self.assertEqual(payload.digest.hex(), v["digest"])
                self.assertTrue(payload.verify(base64.b64decode(local_signed["tx_info"]["Sig"]).hex(), local.public_key))

    async def test_custom_signers_run_on_loop_with_context_and_cancel_cleanly(self):
        local = HyperliquidSigner(HL_KEY)
        context = contextvars.ContextVar("signer_context", default="missing")
        context.set("request-context")
        seen = []

        async def sign(payload):
            self.assertIs(asyncio.get_running_loop(), loop)
            seen.append((context.get(), payload.network, payload.scheme))
            return await local.sign(payload)

        loop = asyncio.get_running_loop()
        remote = HyperliquidSigner.remote(local.address, sign)
        action = HL_VECTORS["l1"][0]["action"]
        await remote.sign_action(action, nonce=123, network="testnet")
        self.assertEqual(seen, [("request-context", "testnet", "l1_action")])
        entered, cleaned = asyncio.Event(), asyncio.Event()

        async def hanging(payload):
            entered.set()
            try:
                await asyncio.Future()
            finally:
                cleaned.set()

        remote = HyperliquidSigner.remote(local.address, hanging)
        pending = asyncio.ensure_future(remote.sign_action(action, nonce=124))
        await bounded(entered.wait())
        pending.cancel()
        with self.assertRaises(asyncio.CancelledError):
            await pending
        await bounded(cleaned.wait())

    async def test_immutable_payloads_negative_signatures_and_token_expiry(self):
        signer = LighterSigner(LT_KEY)
        deadline = int(time.time()) + 600
        payload = lighter_auth_signing_payload(account_index=37, api_key_index=3, deadline=deadline)
        token = await signer.auth_token(account_index=37, api_key_index=3, deadline=deadline)
        self.assertTrue(payload.verify(token.rsplit(":", 1)[1], signer.public_key))
        with self.assertRaises(AttributeError):
            payload.account_index = 38
        with self.assertRaises(AttributeError):
            payload.digest = b"changed"
        wrong = lighter_auth_signing_payload(account_index=38, api_key_index=3, deadline=deadline)
        with self.assertRaises(SigningError):
            wrong.verify(token.rsplit(":", 1)[1], signer.public_key)
        for invalid in [int(time.time()) - 1, int(time.time()) + 9 * 3600]:
            with self.assertRaises(ValidationError):
                await signer.auth_token(account_index=37, api_key_index=3, deadline=invalid)

    async def test_custom_signer_failures_and_wrong_identity_are_redacted(self):
        async def bad(payload):
            raise RuntimeError("secret-from-signing-service")

        local = HyperliquidSigner(HL_KEY)
        remote = HyperliquidSigner.remote(local.address, bad)
        with self.assertRaises(SigningError) as caught:
            await remote.sign_action(HL_VECTORS["l1"][0]["action"], nonce=123)
        self.assertNotIn("secret-from-signing-service", repr(caught.exception))

        async def other_key(payload):
            return await local.sign(payload)

        remote = HyperliquidSigner.remote(ADDRESS, other_key)
        with self.assertRaises(SigningError):
            await remote.sign_action(HL_VECTORS["l1"][0]["action"], nonce=123)
        lt_local = LighterSigner(LT_KEY)

        async def bad_lighter(payload):
            return "00" * 80

        with self.assertRaises(SigningError):
            await LighterSigner.remote(lt_local.public_key, bad_lighter).sign_transaction(
                "cancel", {"MarketIndex": 4096, "Index": 123}, account_index=37,
                api_key_index=3, nonce=1, expired_at=1800000000000,
            )
        self.assertNotIn(HL_KEY.removeprefix("0x"), repr(local))
        self.assertNotIn(LT_KEY, repr(lt_local))
        for constructor in [HyperliquidSigner, LighterSigner]:
            with self.assertRaises(SigningError) as caught:
                constructor("private-secret-invalid")
            self.assertNotIn("private-secret-invalid", str(caught.exception))

    async def test_exact_scaling_precision_and_float_rejection(self):
        self.assertEqual(scale_lighter_decimal("0.000000000000000001", 18), 1)
        self.assertEqual(scale_lighter_decimal(str(2**64 - 1), 0), 2**64 - 1)
        for s in ["1.00001", "-1", "1e3"]:
            with self.assertRaises(ValidationError):
                scale_lighter_decimal(s, 4)
        order = HL_VECTORS["l1"][0]["action"]["orders"][0]
        validate_hyperliquid_order(order, sz_decimals=4)
        with self.assertRaises(ValidationError):
            validate_hyperliquid_order(dict(order, s="0.00001"), sz_decimals=4)
        with self.assertRaises(ValidationError):
            await HyperliquidSigner(HL_KEY).sign_action({"type": "order", "orders": [dict(order, p=1670.1)]}, nonce=123)
        cyclic = {"type": "order"}
        cyclic["orders"] = [cyclic]
        with self.assertRaises(ValidationError):
            await HyperliquidSigner(HL_KEY).sign_action(cyclic, nonce=123)


class Peer:
    def __init__(self, socket):
        self.socket = socket
        self.unsubscribed = asyncio.Event()
        self.closed = asyncio.Event()


class ApiParity(unittest.IsolatedAsyncioTestCase):
    async def asyncSetUp(self):
        self.requests, self.clients, self.errors, self.tasks = [], [], [], set()
        self.connections, self.subscriptions = [], asyncio.Queue()
        self.pongs = asyncio.Queue()
        self.response_override = None
        self.hold_submission = False
        self.submitted, self.release = asyncio.Event(), asyncio.Event()
        self.http = await asyncio.start_server(self.http_handler, "127.0.0.1", 0)
        self.ws = await serve(self.ws_handler, "127.0.0.1", 0)
        self.http_url = f"http://127.0.0.1:{self.http.sockets[0].getsockname()[1]}"
        self.ws_url = f"ws://127.0.0.1:{self.ws.sockets[0].getsockname()[1]}"
        self.account = next(ACCOUNTS)

    def hl(self, **kwargs):
        kwargs.setdefault("read_retries", 0)
        kwargs.setdefault("reconnect_retries", 0)
        c = HyperliquidClient(http_url=self.http_url + "/hl", ws_url=self.ws_url + "/hl", timeout_ms=500, **kwargs)
        self.clients.append(c)
        return c

    def lt(self, **kwargs):
        kwargs.setdefault("read_retries", 0)
        kwargs.setdefault("reconnect_retries", 0)
        c = LighterClient(http_url=self.http_url + "/lt", ws_url=self.ws_url + "/lt", timeout_ms=500, **kwargs)
        self.clients.append(c)
        return c

    def signed_lt(self, **kwargs):
        return self.lt(signer=LighterSigner(LT_KEY), account_index=self.account, api_key_index=3, **kwargs)

    async def asyncTearDown(self):
        for client in self.clients:
            await bounded(client.aclose())
        self.http.close()
        await self.http.wait_closed()
        self.ws.close()
        await bounded(self.ws.wait_closed())
        tasks = list(self.tasks)
        for task in tasks:
            task.cancel()
        await asyncio.gather(*tasks, return_exceptions=True)
        self.assertEqual(self.errors, [])

    async def http_handler(self, reader, writer):
        task = asyncio.current_task()
        self.tasks.add(task)
        try:
            raw = await reader.readuntil(b"\r\n\r\n")
            first, *lines = raw.decode().split("\r\n")
            method, target, _ = first.split()
            headers = dict(line.split(": ", 1) for line in lines if ": " in line)
            headers = {k.lower(): v for k, v in headers.items()}
            body = await reader.readexactly(int(headers.get("content-length", 0)))
            url = urlsplit(target)
            data = json.loads(body) if target.startswith("/hl/") else parse_qs(body.decode())
            record = {"path": url.path, "query": parse_qs(url.query), "data": data, "headers": headers}
            self.requests.append(record)
            status, result = self.reply(record)
            if url.path.endswith("sendTx"):
                self.submitted.set()
                if self.hold_submission:
                    await self.release.wait()
            if self.response_override is not None:
                status, result = self.response_override
            encoded = (result if isinstance(result, str) else json.dumps(result)).encode()
            writer.write(f"HTTP/1.1 {status} Response\r\nContent-Type: application/json\r\nContent-Length: {len(encoded)}\r\nConnection: close\r\n\r\n".encode() + encoded)
            await writer.drain()
        except (ConnectionError, asyncio.IncompleteReadError):
            pass
        except Exception as e:
            self.errors.append(e)
        finally:
            writer.close()
            try:
                await writer.wait_closed()
            except ConnectionError:
                pass
            self.tasks.discard(task)

    def reply(self, record):
        path, data = record["path"], record["data"]
        if path == "/hl/info":
            names = {"meta": "meta.json", "l2Book": "book.json", "clearinghouseState": "account.json", "openOrders": "open_orders.json"}
            return 200, fixture(HL, names[data["type"]])
        if path == "/hl/exchange":
            return 200, {"status": "ok", "response": {"type": data["action"]["type"], "data": {"statuses": [{"resting": {"oid": 2**53 + 1}}, {"error": "native rejection"}]}}}
        name = path.rsplit("/", 1)[-1]
        if name == "nextNonce":
            return 200, {"code": 200, "nonce": 42}
        if name == "sendTx":
            return 200, {"code": 200, "tx_hash": "test-hash", "predicted_execution_time_ms": 1}
        if name == "accountActiveOrders":
            return 200, {"code": 200, "orders": []}
        names = {"orderBooks": "markets.json", "orderBookOrders": "book.json", "recentTrades": "trades.json", "account": "account.json"}
        return 200, fixture(LT, names[name])

    async def ws_handler(self, socket):
        peer = Peer(socket)
        self.connections.append(peer)
        lighter = socket.request.path == "/lt"
        try:
            if lighter:
                await socket.send('{"type":"connected"}')
            async for raw in socket:
                message = json.loads(raw)
                method = message.get("type") if lighter else message["method"]
                if method == "subscribe":
                    await self.subscriptions.put((peer, message))
                    if not lighter:
                        await socket.send(json.dumps({"channel": "subscriptionResponse", "data": message}))
                elif method == "unsubscribe":
                    peer.unsubscribed.set()
                elif method == "pong":
                    await self.pongs.put(message)
                elif method == "ping":
                    await socket.send('{"channel":"pong"}')
        except ConnectionClosed:
            pass
        except Exception as e:
            self.errors.append(e)
        finally:
            peer.closed.set()

    async def test_hyperliquid_account_reads_dex_and_agent_vault_context(self):
        public = self.hl()
        state = await public.clearinghouse_state(ADDRESS, dex="xyz")
        self.assertEqual(state["assetPositions"][0]["position"]["szi"], "-0.0335")
        orders = await public.open_orders(ADDRESS)
        self.assertEqual(orders[0]["oid"], 2**53 + 1)
        await public.meta(dex="xyz")
        self.assertEqual(self.requests[0]["data"], {"type": "clearinghouseState", "user": ADDRESS, "dex": "xyz"})
        vault = "0x0000000000000000000000000000000000000042"
        c = self.hl(signer=HyperliquidSigner(HL_KEY), account_address=ADDRESS, vault_address=vault)
        self.assertTrue(c.authenticated)
        self.assertEqual(c.account_address, vault)
        self.assertEqual(c.signer_address, HL_VECTORS["signer_address"])
        result = await c.order({k: v for k, v in HL_VECTORS["l1"][0]["action"].items() if k != "type"}, nonce=1677777606040, expires_after=1677777607000)
        self.assertEqual(result["response"]["data"]["statuses"], [{"resting": {"oid": 2**53 + 1}}, {"error": "native rejection"}])
        await c.cancel({"cancels": [{"a": 4, "o": 2**53 + 1}]})
        await c.cancel_by_cloid({"cancels": [{"asset": 4, "cloid": "0x00000000000000000000000000000001"}]})
        sent = [r["data"] for r in self.requests if r["path"].endswith("exchange")]
        self.assertEqual([v["action"]["type"] for v in sent], ["order", "cancel", "cancelByCloid"])
        self.assertEqual(sent[0]["vaultAddress"], vault)
        self.assertEqual(sent[0]["expiresAfter"], 1677777607000)
        self.assertGreater(sent[-1]["nonce"], sent[-2]["nonce"])

    async def test_lighter_public_reads_native_values_and_private_auth(self):
        c = self.lt()
        self.assertFalse(c.authenticated)
        self.assertEqual((await c.order_books())["order_books"][0]["market_id"], 4099)
        self.assertEqual((await c.order_book_orders(4099, limit=2))["bids"][0]["price"], "3.0000")
        self.assertTrue((await c.recent_trades(4099))["trades"])
        await c.account(account_index=1)
        await c.account(l1_address=ADDRESS)
        self.assertEqual((await c.next_nonce(self.account, 3))["nonce"], 42)
        private = self.signed_lt()
        self.assertEqual((await private.active_orders(market_id=4096))["orders"], [])
        token = self.requests[-1]["headers"]["authorization"]
        self.assertEqual(token.split(":")[1:3], [str(self.account), "3"])
        self.assertEqual((await private.auth_token(int(time.time()) + 600)).split(":")[1:3], [str(self.account), "3"])

    async def test_validation_and_authentication_errors_before_network(self):
        hl, lt = self.hl(), self.lt()
        with self.assertRaises(AuthenticationError):
            await hl.cancel({"cancels": [{"a": 4, "o": 1}]})
        for operation in [lt.active_orders(), lt.cancel_order({"MarketIndex": 4096, "Index": 1}), lt.subscribe_account_orders()]:
            with self.assertRaises(AuthenticationError):
                await operation
        for operation in [lt.order_book_orders(255), lt.recent_trades(4096, limit=101), lt.account(), lt.account(account_index=1, l1_address=ADDRESS), hl.clearinghouse_state("bad"), hl.subscribe({"type": "unknown"})]:
            with self.assertRaises(ValidationError):
                await operation
        with self.assertRaises(ValidationError):
            await self.signed_lt().cancel_order({"MarketIndex": 4096, "Index": 1.0})
        self.assertEqual(self.requests, [])
        self.assertEqual(self.connections, [])

    async def test_exact_unknown_json_numbers_and_sanitized_native_errors(self):
        c = self.lt()
        self.response_override = (200, '{"code":200,"nonce":42,"precise":0.1234567890123456789012300,"huge":1234567890123456789012345678901234567890}')
        result = await c.next_nonce(self.account, 3)
        self.assertEqual(result["precise"], Decimal("0.1234567890123456789012300"))
        self.assertIsInstance(result["precise"], Decimal)
        self.assertEqual(str(result["precise"]), "0.1234567890123456789012300")
        self.assertEqual(result["huge"], 1234567890123456789012345678901234567890)
        self.response_override = (429, {"code": 21104, "message": "secret-token-echo", "request_id": "id"})
        with self.assertRaises(ExchangeError) as caught:
            await c.order_books()
        self.assertEqual(caught.exception.code, 21104)
        self.assertEqual(caught.exception.http_status, 429)
        self.assertEqual(caught.exception.extra["request_id"], "id")
        self.assertEqual(caught.exception.native_message, "secret-token-echo")
        self.assertNotIn("secret-token-echo", repr(caught.exception))

    async def test_lighter_concurrent_clients_share_sequence_and_emit_signed_forms(self):
        a, b = self.signed_lt(), self.signed_lt()
        order = {k: v for k, v in LT_VECTORS[0]["tx_info"].items() if k not in {"AccountIndex", "ApiKeyIndex", "ExpiredAt", "Nonce", "Sig", "L2TxAttributes"}}
        await a.create_order(order, expired_at=1800000000000, attributes={6: 2, 7: 1})
        await asyncio.gather(*(c.cancel_order({"MarketIndex": 4096, "Index": 123}) for c in [a, b] * 3))
        sends = [r for r in self.requests if r["path"].endswith("sendTx")]
        infos = [json.loads(r["data"]["tx_info"][0]) for r in sends]
        self.assertEqual([v["Nonce"] for v in infos], list(range(42, 49)))
        self.assertEqual([v["AccountIndex"] for v in infos], [self.account] * 7)
        self.assertEqual(infos[0]["L2TxAttributes"], {"6": 2, "7": 1})
        self.assertTrue(all(len(base64.b64decode(v["Sig"])) == 80 for v in infos))
        self.assertTrue(all(r["headers"]["content-type"] == "application/x-www-form-urlencoded" for r in sends))
        self.assertEqual(sum(r["path"].endswith("nextNonce") for r in self.requests), 1)

    async def test_lighter_cancelled_submission_quarantines_until_explicit_recovery(self):
        c = self.signed_lt()
        self.hold_submission = True
        pending = asyncio.create_task(c.cancel_order({"MarketIndex": 4096, "Index": 123}))
        await bounded(self.submitted.wait())
        pending.cancel()
        with self.assertRaises(asyncio.CancelledError):
            await pending
        # The Rust cancellation notification crosses a thread boundary.
        await asyncio.sleep(0.03)
        with self.assertRaises(NonceUncertainError):
            await c.cancel_order({"MarketIndex": 4096, "Index": 123})
        self.assertEqual(sum(r["path"].endswith("sendTx") for r in self.requests), 1)
        self.release.set()
        self.hold_submission = False
        self.assertEqual(await c.resync_nonce(), 42)  # Mock reconciliation only.
        await c.cancel_order({"MarketIndex": 4096, "Index": 123})

    async def test_lighter_rejection_and_malformed_response_quarantine_without_replay(self):
        for result, error in [({"code": 21104, "message": "rejected"}, ExchangeError), ("malformed", DecodeError)]:
            self.account = next(ACCOUNTS)
            c = self.signed_lt()
            # nextNonce must succeed before overriding the submission response.
            await c.resync_nonce()
            self.response_override = (200, result)
            with self.assertRaises(error):
                await c.cancel_order({"MarketIndex": 4096, "Index": 123})
            count = len(self.requests)
            with self.assertRaises(NonceUncertainError):
                await c.cancel_order({"MarketIndex": 4096, "Index": 123})
            self.assertEqual(len(self.requests), count)
            self.response_override = None

    async def test_all_hyperliquid_native_stream_types_and_cancellation(self):
        c = self.hl()
        order = fixture(HL, "open_orders.json")[0]
        cases = [
            (c.subscribe({"type": "l2Book", "coin": "ETH"}), "l2Book", fixture(HL, "book.json")),
            (c.subscribe_trades("ETH"), "trades", fixture(HL, "trades.json")["data"]),
            (c.subscribe_order_updates(ADDRESS), "orderUpdates", [{"order": order, "status": "open", "statusTimestamp": 1}]),
            (c.subscribe_user_fills(ADDRESS, aggregate_by_time=True), "userFills", {"user": ADDRESS, "isSnapshot": True, "fills": []}),
            (c.subscribe_clearinghouse_state(ADDRESS, dex="xyz"), "clearinghouseState", {"user": ADDRESS, "dex": "xyz", "clearinghouseState": fixture(HL, "account.json")}),
            (c.subscribe_open_orders(ADDRESS), "openOrders", {"user": ADDRESS, "dex": "", "orders": [order]}),
        ]
        for operation, channel, data in cases:
            stream = await operation
            peer, spec = await bounded(self.subscriptions.get())
            ack = await bounded(anext(stream))
            self.assertEqual(ack["channel"], "subscriptionResponse")
            self.assertEqual(ack["data"]["subscription"], spec["subscription"])
            await peer.socket.send(json.dumps({"channel": channel, "data": data}))
            event = await bounded(anext(stream))
            self.assertEqual(event, {"channel": channel, "data": data})
            pending = asyncio.create_task(anext(stream))
            await asyncio.sleep(0.01)
            pending.cancel()
            with self.assertRaises(asyncio.CancelledError):
                await pending
            await bounded(peer.unsubscribed.wait())
            self.assertTrue(stream.closed)

    async def test_all_lighter_streams_native_snapshots_deltas_auth_and_pong(self):
        c = self.signed_lt()
        cases = [
            (c.subscribe_order_book(4099), fixture(LT, "book_stream.json"), False),
            (c.subscribe_trades(4099), fixture(LT, "trades_stream.json"), False),
            (c.subscribe_account(1), fixture(LT, "account_stream.json"), False),
            (c.subscribe_account_orders(), {"type": "subscribed/account_all_orders", "channel": f"account_all_orders:{self.account}", "orders": {}}, True),
        ]
        for operation, event, private in cases:
            stream = await operation
            peer, spec = await bounded(self.subscriptions.get())
            self.assertEqual("auth" in spec, private)
            await peer.socket.send('{"type":"ping"}')
            self.assertEqual(await bounded(self.pongs.get()), {"type": "pong"})
            await peer.socket.send(json.dumps(event))
            actual = await bounded(anext(stream))
            self.assertEqual(actual, event)
            if "order_book" in event:
                delta = copy.deepcopy(event)
                delta["type"] = "update/order_book"
                delta["order_book"]["bids"][0]["size"] = "0.0000"
                await peer.socket.send(json.dumps(delta))
                self.assertEqual((await bounded(anext(stream)))["order_book"]["bids"][0]["size"], "0.0000")
            await stream.aclose()
            await bounded(peer.unsubscribed.wait())

    async def test_lighter_late_consumers_get_fresh_snapshots_and_cleanup_on_client_close(self):
        c = self.lt()
        first = await c.subscribe_order_book(4099)
        peer1, _ = await bounded(self.subscriptions.get())
        await peer1.socket.send((LT / "book_stream.json").read_text())
        await bounded(anext(first))
        second = await c.subscribe_order_book(4099)
        peer2, _ = await bounded(self.subscriptions.get())
        self.assertIsNot(peer1, peer2)
        await peer2.socket.send((LT / "book_stream.json").read_text())
        self.assertEqual((await bounded(anext(second)))["type"], "subscribed/order_book")
        pending = asyncio.create_task(anext(first))
        await asyncio.sleep(0.01)
        with self.assertRaises(BusyError):
            await anext(first)
        await c.aclose()
        with self.assertRaises(StopAsyncIteration):
            await pending
        await bounded(peer1.closed.wait())
        await bounded(peer2.closed.wait())
        self.assertTrue(first.closed and second.closed)
        with self.assertRaises(ClosedError):
            await c.order_books()

    async def test_lighter_errors_close_streams_and_retain_structured_codes(self):
        c = self.lt()
        stream = await c.subscribe_order_book(4099)
        peer, _ = await bounded(self.subscriptions.get())
        await peer.socket.send('{"error":{"code":30005,"message":"secret-stream-token"}}')
        with self.assertRaises(ExchangeError) as caught:
            await anext(stream)
        self.assertEqual(caught.exception.code, 30005)
        self.assertNotIn("secret-stream-token", repr(caught.exception))
        self.assertTrue(stream.closed)
        await bounded(peer.closed.wait())

    async def test_lighter_overflow_disconnect_malformed_and_cancelled_reads_cleanup(self):
        c = self.lt(subscription_capacity=1)
        for mode in ["malformed", "disconnect", "overflow", "cancel"]:
            stream = await c.subscribe_order_book(4099)
            peer, _ = await bounded(self.subscriptions.get())
            if mode == "malformed":
                await peer.socket.send("{")
                with self.assertRaises(DecodeError):
                    await anext(stream)
            elif mode == "disconnect":
                await peer.socket.close()
                with self.assertRaises(TransportError) as caught:
                    await anext(stream)
                self.assertEqual(caught.exception.kind, "disconnected")
            elif mode == "overflow":
                for _ in range(12):
                    try:
                        await peer.socket.send((LT / "book_stream.json").read_text())
                    except ConnectionClosed:
                        break
                await asyncio.sleep(0.03)
                with self.assertRaises(TransportError) as caught:
                    await anext(stream)
                self.assertEqual(caught.exception.kind, "overflow")
            else:
                pending = asyncio.create_task(anext(stream))
                await asyncio.sleep(0.01)
                pending.cancel()
                with self.assertRaises(asyncio.CancelledError):
                    await pending
            self.assertTrue(stream.closed)
            await bounded(peer.closed.wait())

    async def test_lighter_timeout_persists_quarantine_across_client_reconstruction(self):
        c = self.signed_lt()
        self.hold_submission = True
        with self.assertRaises(TransportError) as caught:
            await c.cancel_order({"MarketIndex": 4096, "Index": 123})
        self.assertEqual(caught.exception.kind, "timeout")
        self.assertEqual(caught.exception.outcome, "unknown")
        await c.aclose()
        replacement = self.signed_lt()
        with self.assertRaises(NonceUncertainError):
            await replacement.cancel_order({"MarketIndex": 4096, "Index": 123})
        self.assertEqual(sum(r["path"].endswith("sendTx") for r in self.requests), 1)
        self.release.set()

    async def test_client_close_cancels_custom_signer_without_submitting(self):
        local = LighterSigner(LT_KEY)
        entered, cleaned = asyncio.Event(), asyncio.Event()

        async def sign(payload):
            entered.set()
            try:
                await asyncio.Future()
            finally:
                cleaned.set()

        c = self.lt(signer=LighterSigner.remote(local.public_key, sign), account_index=self.account, api_key_index=3)
        pending = asyncio.create_task(c.cancel_order({"MarketIndex": 4096, "Index": 123}))
        await bounded(entered.wait())
        await c.aclose()
        with self.assertRaises(ClosedError):
            await bounded(pending)
        await bounded(cleaned.wait())
        self.assertFalse(any(r["path"].endswith("sendTx") for r in self.requests))

    async def test_callback_destructor_can_reenter_closed_client(self):
        released = []

        class Service:
            async def __call__(self, payload):
                raise RuntimeError("not used")

            def __del__(self):
                client = reference()
                if client is not None:
                    try:
                        client._inner.authenticated
                    except ClosedError:
                        released.append(True)

        c = self.lt(signer=LighterSigner.remote(LighterSigner(LT_KEY).public_key, Service()), account_index=self.account, api_key_index=3)
        reference = weakref.ref(c)
        await c.aclose()
        self.assertEqual(released, [True])

    async def test_hardening_safe_read_retries_and_writes_remain_single_attempt(self):
        c = self.signed_lt(read_retries=2, initial_backoff_ms=5, max_backoff_ms=10)
        await c.resync_nonce()
        self.response_override = (503, {"code": 21104, "message": "busy"})
        before = len(self.requests)
        with self.assertRaises(ExchangeError):
            await c.order_books()
        self.assertEqual(len(self.requests) - before, 3)
        before = len(self.requests)
        with self.assertRaises(ExchangeError):
            await c.cancel_order({"MarketIndex": 4096, "Index": 123})
        self.assertEqual(len(self.requests) - before, 1)
        with self.assertRaises(NonceUncertainError):
            await c.cancel_order({"MarketIndex": 4096, "Index": 123})
        self.assertEqual(len(self.requests) - before, 1)

    async def test_hardening_gap_is_recoverable_and_precedes_fresh_snapshots(self):
        for lighter in [False, True]:
            c = (self.lt if lighter else self.hl)(reconnect_retries=2, initial_backoff_ms=10, max_backoff_ms=20)
            stream = await (c.subscribe_order_book(4099) if lighter else c.subscribe_l2_book("ETH"))
            first, _ = await bounded(self.subscriptions.get())
            await first.socket.close()
            second, spec = await bounded(self.subscriptions.get())
            with self.assertRaises(GapError) as caught:
                await bounded(anext(stream))
            self.assertEqual(caught.exception.kind, "gap")
            self.assertFalse(stream.closed)
            if lighter:
                event = fixture(LT, "book_stream.json")
                event["order_book"]["nonce"] = 12345
                await second.socket.send(json.dumps(event))
                self.assertEqual((await bounded(anext(stream)))["order_book"]["nonce"], 12345)
            else:
                book = fixture(HL, "book.json")
                book["time"] = 12345
                await second.socket.send(json.dumps({"channel": "l2Book", "data": book}))
                self.assertEqual((await bounded(anext(stream)))["time"], 12345)
            pending = asyncio.create_task(anext(stream))
            await asyncio.sleep(0.01)
            pending.cancel()
            with self.assertRaises(asyncio.CancelledError):
                await pending
            await bounded(second.closed.wait())
            self.assertTrue(stream.closed)

    async def test_hardening_reconnect_exhaustion_is_terminal(self):
        c = self.lt(reconnect_retries=2, initial_backoff_ms=5, max_backoff_ms=10)
        stream = await c.subscribe_order_book(4099)
        for _ in range(3):
            peer, _ = await bounded(self.subscriptions.get())
            await peer.socket.close()
        # Exhaustion may supersede an unread gap; it always closes the iterator.
        try:
            await bounded(anext(stream))
        except GapError:
            pass
        except TransportError as error:
            self.assertEqual(error.kind, "disconnected")
        if not stream.closed:
            with self.assertRaises(TransportError) as caught:
                await bounded(anext(stream))
            self.assertEqual(caught.exception.kind, "disconnected")
        self.assertTrue(stream.closed)
        with self.assertRaises(asyncio.TimeoutError):
            await asyncio.wait_for(self.subscriptions.get(), 0.05)

    async def test_hardening_close_interrupts_reconnect_and_read_backoff(self):
        c = self.lt(reconnect_retries=2, initial_backoff_ms=2000, max_backoff_ms=2000)
        stream = await c.subscribe_order_book(4099)
        peer, _ = await bounded(self.subscriptions.get())
        await peer.socket.close()
        with self.assertRaises(GapError):
            await bounded(anext(stream))
        await asyncio.wait_for(c.aclose(), 0.5)
        self.assertTrue(stream.closed)
        c = self.hl(read_retries=2, initial_backoff_ms=2000, max_backoff_ms=2000)
        self.response_override = (503, "busy")
        pending = asyncio.create_task(c.meta())
        async def seen():
            while not self.requests:
                await asyncio.sleep(0.001)
        await bounded(seen())
        await asyncio.sleep(0.01)
        await asyncio.wait_for(c.aclose(), 0.5)
        with self.assertRaises(ClosedError):
            await bounded(pending)
        self.assertEqual(len(self.requests), 1)

    async def test_hardening_invalid_policies_fail_before_network(self):
        for options in [{"read_retries": 17}, {"reconnect_retries": 17}, {"initial_backoff_ms": 0},
                        {"max_backoff_ms": 1}, {"heartbeat_interval_ms": 0},
                        {"idle_timeout_ms": 100}, {"max_message_bytes": 0}]:
            with self.assertRaises(ValidationError):
                LighterClient(**options)
        self.assertEqual(self.requests, [])
