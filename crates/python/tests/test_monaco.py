"""Monaco Rust/Python parity against local REST and WebSocket peers."""
from __future__ import annotations

import asyncio
import base64
import contextvars
import hashlib
import json
import time
import unittest
from decimal import Decimal
from pathlib import Path
from urllib.parse import urlsplit

from websockets.asyncio.server import serve
from websockets.exceptions import ConnectionClosed
from aux_py import (
    MonacoClient, MonacoSessionSigner, MonacoWalletSigner, AuthenticationError,
    BusyError, ClosedError, ExchangeError, GapError, OutcomeUncertainError, SigningError,
    TransportError, ValidationError,
)

VECTORS = json.loads((Path(__file__).resolve().parents[2] /
                      "exchanges/monaco/tests/fixtures/signing.json").read_text())
PAIR = "550e8400-e29b-41d4-a716-446655440000"
ORDER = "123e4567-e89b-12d3-a456-426614174000"
MARKET = {"id": PAIR, "marketType": "SPOT", "tickSize": "0.01", "quantityStepSize": "0.001"}


class MonacoTests(unittest.IsolatedAsyncioTestCase):
    async def asyncSetUp(self):
        self.clients = []
        self.requests = []
        self.commands = []
        self.errors = []
        self.tasks = set()
        self.peers = asyncio.Queue()
        self.status = 200
        self.body = None
        self.challenge_bound = True
        self.local = MonacoSessionSigner(VECTORS["seed"])
        self.http_server = await asyncio.start_server(self.http, "127.0.0.1", 0)
        self.http_url = f"http://127.0.0.1:{self.http_server.sockets[0].getsockname()[1]}"
        self.ws_server = await serve(self.ws, "127.0.0.1", 0)
        self.ws_url = f"ws://127.0.0.1:{self.ws_server.sockets[0].getsockname()[1]}/ws"

    async def asyncTearDown(self):
        for client in self.clients:
            await asyncio.wait_for(client.aclose(), 3)
        self.http_server.close()
        await self.http_server.wait_closed()
        self.ws_server.close()
        await asyncio.wait_for(self.ws_server.wait_closed(), 3)
        tasks = list(self.tasks)
        for task in tasks:
            task.cancel()
        await asyncio.gather(*tasks, return_exceptions=True)
        self.assertEqual(self.errors, [])

    def client(self, **kwargs):
        kwargs.setdefault("timeout_ms", 500)
        kwargs.setdefault("read_retries", 0)
        kwargs.setdefault("reconnect_retries", 2)
        kwargs.setdefault("initial_backoff_ms", 10)
        kwargs.setdefault("max_backoff_ms", 20)
        client = MonacoClient(http_url=self.http_url + "/api/v1/", ws_url=self.ws_url,
                              chain_id=1328, **kwargs)
        self.clients.append(client)
        return client

    def restore(self, client, signer=None, protocol="legacy", expiry=None):
        signer = signer or self.local
        client.restore_session(signer, {
            "address": VECTORS["wallet"]["address"], "clientId": "fixture",
            "sessionPublicKey": signer.public_key,
            "expiresAt": expiry or int(time.time()) + 3600, "chainId": 1328,
        }, request_signing=protocol)

    async def http(self, reader, writer):
        task = asyncio.current_task()
        self.tasks.add(task)
        try:
            head = (await reader.readuntil(b"\r\n\r\n")).decode()
            lines = head.split("\r\n")
            headers = dict((k.lower(), v.strip()) for line in lines[1:] if ":" in line
                           for k, v in [line.split(":", 1)])
            raw = await reader.readexactly(int(headers.get("content-length", 0)))
            body = json.loads(raw) if raw else None
            method, target, _ = lines[0].split()
            path = urlsplit(target).path
            self.requests.append((method, target, headers, body))
            self.assertTrue(headers["user-agent"].startswith("aux-rs-monaco/"))
            if "x-monaco-signature" in headers:
                message = f'{method}\n{target}\n{headers["x-monaco-timestamp"]}\n{hashlib.sha256(raw).hexdigest()}'
                self.assertEqual(headers["x-monaco-signature"], await self.local.sign(message.encode()))
            elif "signature" in headers:
                params = headers["signature-input"].removeprefix("monaco=")
                digest = "sha-256=:" + base64.b64encode(hashlib.sha256(raw).digest()).decode() + ":"
                self.assertEqual(headers["content-digest"], digest)
                query = "?" + target.split("?", 1)[1] if "?" in target else "?"
                message = (f'"@method": {method}\n"@path": {path}\n"@query": {query}\n'
                           f'"content-digest": {digest}\n"@signature-params": {params}')
                signature = base64.b64encode(bytes.fromhex(await self.local.sign(message.encode()))).decode()
                self.assertEqual(headers["signature"], f"monaco=:{signature}:")
            if path.endswith("/auth/challenge"):
                pub = body["sessionPublicKey"] if self.challenge_bound else "wrong-key"
                payload = {"nonce": "fixture-nonce", "message": f"Authorize session {pub}",
                           "expiresAt": int(time.time()) + 300}
            elif path.endswith("/auth/verify"):
                self.assertEqual(body["chainId"], "1328")
                self.assertEqual(body["sessionPublicKey"], self.local.public_key)
                payload = {"expiresAt": int(time.time()) + 3600,
                           "user": {"address": VECTORS["wallet"]["address"]}}
            elif path.endswith("/auth/refresh"):
                payload = {"expiresAt": int(time.time()) + 7200}
            elif path.endswith("/auth/revoke"):
                payload = {"message": "revoked"}
            elif path.endswith("/market/pairs"):
                payload = {"tradingPairs": [MARKET | {"futureDecimal": 0.123456789, "futureId": 2**64 - 1}],
                           "page": 1, "total": 1}
            elif path.endswith("/accounts/balances"):
                payload = {"balances": [{"availableBalance": "1.000000000000000001", "assetId": PAIR}],
                           "nextPageToken": "opaque/cursor+"}
            elif path.endswith("/orders") and method == "POST":
                payload = {"orderId": ORDER, "status": "SUCCESS", "matchResult": {"status": "REJECTED"}}
            elif path.endswith("/orders/batch-create"):
                payload = {"results": [{"success": True, "orderId": ORDER},
                                       {"success": False, "error": {"code": "POST_ONLY_WOULD_CROSS"}}]}
            else:
                payload = {}
            data = json.dumps(self.body if self.body is not None else payload).encode()
            writer.write(f"HTTP/1.1 {self.status} Response\r\nContent-Length: {len(data)}\r\nConnection: close\r\n\r\n".encode() + data)
            await writer.drain()
        except (asyncio.IncompleteReadError, ConnectionError):
            pass
        except Exception as error:
            self.errors.append(error)
        finally:
            writer.close()
            await writer.wait_closed()
            self.tasks.discard(task)

    async def ws(self, socket):
        authenticated = False
        try:
            async for raw in socket:
                msg = json.loads(raw)
                self.commands.append(msg)
                if msg["type"] == "Authenticate":
                    message = f'WS-AUTH\n{msg["public_key"]}\n{msg["timestamp"]}'
                    self.assertEqual(msg["signature"], await self.local.sign(message.encode()))
                    authenticated = True
                    await socket.send(json.dumps({"type": "Authenticated"}))
                elif msg["type"] == "Subscribe":
                    channel = msg["channels"][0]
                    if channel.split(":")[0] in ("orders", "balances", "positions", "account"):
                        self.assertTrue(authenticated)
                    await socket.send(json.dumps({"type": "Subscribed", "channels": [channel]}))
                    await socket.send(json.dumps({"type": "Snapshot", "channel": channel,
                                                 "data": [{"quantity": "0.000000000000000001", "version": 2**64 - 1}]}))
                    await self.peers.put((socket, channel))
                elif msg["type"] == "Ping":
                    await socket.send(json.dumps({"type": "Pong"}))
        except ConnectionClosed:
            pass
        except Exception as error:
            self.errors.append(error)

    async def snapshot(self, stream):
        while True:
            event = await asyncio.wait_for(anext(stream), 3)
            if event["type"] == "Snapshot":
                return event

    async def test_reference_vectors_and_secret_export_are_explicit(self):
        self.assertEqual(self.local.public_key, VECTORS["publicKey"])
        self.assertEqual(self.local.export_seed(), VECTORS["seed"])
        self.assertNotIn(VECTORS["seed"], repr(self.local))
        wallet = MonacoWalletSigner(VECTORS["wallet"]["privateKey"])
        for row in VECTORS["wallet"]["messages"]:
            self.assertEqual(await wallet.sign_message(row["message"]), row["signature"])
        self.assertEqual(await self.local.sign(VECTORS["websocket"]["message"].encode()),
                         VECTORS["websocket"]["signature"])

    async def test_public_private_reads_exact_values_validation_and_errors(self):
        client = self.client()
        result = await client.trading_pairs({"pageSize": 20})
        self.assertEqual(result["tradingPairs"][0]["futureId"], 2**64 - 1)
        self.assertEqual(result["tradingPairs"][0]["futureDecimal"], Decimal("0.123456789"))
        with self.assertRaises(AuthenticationError):
            await client.orders()
        self.restore(client)
        balances = await client.balances({"pageToken": "a/b+"})
        self.assertEqual(balances["balances"][0]["availableBalance"], "1.000000000000000001")
        self.assertIn("pageToken=a%2Fb%2B", self.requests[-1][1])
        for method in ("trading_pair", "market_metadata", "order_book", "trades", "mark_price", "index_price", "funding", "funding_history"):
            await getattr(client, method)({"tradingPairId": PAIR})
        for method in ("parent_margin_account", "margin_accounts", "positions", "orders"):
            await getattr(client, method)()
        await client.order({"orderId": ORDER})
        for bad in ({"pageSize": 1.2}, {"pageSize": 0}, {"invalid": 1}):
            with self.assertRaises(ValidationError):
                await client.trading_pairs(bad)
        self.status, self.body = 400, {"code": "POST_ONLY_WOULD_CROSS", "message": "secret-echo"}
        with self.assertRaises(ExchangeError) as caught:
            await client.balances()
        self.assertEqual(caught.exception.code, "POST_ONLY_WOULD_CROSS")
        self.assertNotIn("secret-echo", str(caught.exception))
        await client.aclose()
        with self.assertRaises(ClosedError):
            await client.trading_pairs()

    async def test_login_refresh_restore_revoke_and_challenge_key_binding(self):
        client = self.client()
        wallet = MonacoWalletSigner(VECTORS["wallet"]["privateKey"])
        info = await client.login(wallet, "fixture", session_signer=self.local, request_signing="rfc9421")
        self.assertTrue(client.authenticated)
        self.assertNotIn("sessionPrivateKey", info)
        self.assertEqual(client.session_signer.public_key, self.local.public_key)
        old = client.session_info["expiresAt"]
        await client.refresh_session()
        self.assertGreater(client.session_info["expiresAt"], old)
        await client.revoke_session()
        self.assertFalse(client.authenticated)
        with self.assertRaises(AuthenticationError):
            await client.orders()
        self.challenge_bound = False
        with self.assertRaises(AuthenticationError):
            await client.login(wallet, "fixture", session_signer=self.local)
        with self.assertRaises(AuthenticationError):
            self.restore(client, expiry=1)

    async def test_native_trading_batches_and_uncertain_outcomes_send_once(self):
        client = self.client()
        self.restore(client)
        request = {"tradingPairId": PAIR, "orderType": "LIMIT", "side": "BUY",
                   "price": "100.0100", "quantity": "1.000", "idempotencyKey": "saved-key"}
        result = await client.place_order(MARKET, request)
        self.assertEqual(result["matchResult"]["status"], "REJECTED")
        self.assertEqual(self.requests[-1][3]["idempotencyKey"], "saved-key")
        result = await client.batch_create([MARKET], {"orders": [request, request]})
        self.assertEqual(result["results"][1]["error"]["code"], "POST_ONLY_WOULD_CROSS")
        await client.replace_order(MARKET, ORDER, {"quantity": "2.000"})
        await client.cancel_order({"orderId": ORDER})
        await client.batch_replace([MARKET], {"orders": [{"orderId": ORDER, "price": "1.00"}]})
        await client.batch_cancel({"orderIds": [ORDER]})
        await client.cancel_all(PAIR)
        await client.cancel_all()
        with self.assertRaises(ValidationError):
            await client.place_order(MARKET, request | {"price": "1.001"})
        with self.assertRaises(ValidationError):
            await client.place_order(MARKET, request | {"quantity": 1.0})
        before = len(self.requests)
        self.status, self.body = 503, {"code": "ORDER_SUBMISSION_UNAVAILABLE"}
        with self.assertRaises(OutcomeUncertainError) as caught:
            await client.place_order(MARKET, request)
        self.assertEqual(caught.exception.outcome, "unknown")
        self.assertEqual(len(self.requests), before + 1)

    async def test_async_signer_context_survives_ws_driver_and_reconnect(self):
        context = contextvars.ContextVar("monaco_context", default="wrong")
        context.set("subscription-context")
        seen = []

        async def sign(message):
            self.assertEqual(asyncio.get_running_loop(), loop)
            self.assertEqual(context.get(), "subscription-context")
            seen.append(message)
            return await self.local.sign(message)

        loop = asyncio.get_running_loop()
        remote = MonacoSessionSigner.remote(self.local.public_key, sign)
        client = self.client()
        self.restore(client, remote)
        stream = await client.subscribe({"type": "orders"})
        self.assertEqual((await self.snapshot(stream))["data"][0]["version"], 2**64 - 1)
        socket, _ = await asyncio.wait_for(self.peers.get(), 3)
        await socket.close()
        with self.assertRaises(GapError):
            await asyncio.wait_for(anext(stream), 3)
        await self.snapshot(stream)
        self.assertEqual(len(seen), 2)
        self.assertTrue(all(message.startswith(b"WS-AUTH\n") for message in seen))
        await stream.aclose()

    async def test_callback_cancellation_and_invalid_signatures_never_submit(self):
        entered, cancelled = asyncio.Event(), asyncio.Event()

        async def blocked(message):
            entered.set()
            try:
                await asyncio.Future()
            finally:
                cancelled.set()

        wallet = MonacoWalletSigner.remote(VECTORS["wallet"]["address"], blocked)
        client = self.client()
        task = asyncio.create_task(client.login(wallet, "fixture", session_signer=self.local))
        await asyncio.wait_for(entered.wait(), 3)
        task.cancel()
        with self.assertRaises(asyncio.CancelledError):
            await task
        await asyncio.wait_for(cancelled.wait(), 3)
        self.assertEqual(len(self.requests), 1)

        async def invalid(message):
            return "00" * 64

        self.restore(client, MonacoSessionSigner.remote(self.local.public_key, invalid))
        with self.assertRaises(SigningError):
            await client.orders()
        self.assertEqual(len(self.requests), 1)

    async def test_stream_cleanup_and_overflow(self):
        client = self.client(subscription_capacity=4)
        self.restore(client)
        stream = await client.subscribe({"type": "balances"})
        await self.snapshot(stream)
        socket, channel = await asyncio.wait_for(self.peers.get(), 3)
        try:
            for n in range(20):
                await socket.send(json.dumps({"type": "Event", "channel": channel, "data": {"version": n}}))
        except ConnectionClosed:
            pass  # Bounded overflow can close the socket during the burst.
        await asyncio.sleep(0.05)
        with self.assertRaises(TransportError) as caught:
            await anext(stream)
        self.assertEqual(caught.exception.kind, "overflow")
        self.assertTrue(stream.closed)
        await stream.aclose()

    async def test_single_reader_cancellation_closes_subscription(self):
        client = self.client()
        self.restore(client)
        stream = await client.subscribe({"type": "balances"})
        await self.snapshot(stream)
        socket, _ = await asyncio.wait_for(self.peers.get(), 3)
        pending = asyncio.create_task(anext(stream))
        await asyncio.sleep(0.01)
        with self.assertRaises(BusyError):
            await anext(stream)
        pending.cancel()
        with self.assertRaises(asyncio.CancelledError):
            await pending
        await asyncio.wait_for(socket.wait_closed(), 3)
        self.assertTrue(stream.closed)
