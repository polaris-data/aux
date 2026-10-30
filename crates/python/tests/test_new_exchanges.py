"""QFEX and Arcus bindings against local HTTP/WebSocket exchange peers."""
from __future__ import annotations

import asyncio
import hashlib
import hmac
import json
import unittest
from decimal import Decimal
from pathlib import Path
from urllib.parse import urlsplit

from websockets.asyncio.server import serve
from websockets.exceptions import ConnectionClosed
from aux_py import (
    ArcusClient, ArcusSigner, QfexClient, AuthenticationError, ClosedError,
    ConnectorError, ExchangeError, ValidationError,
)

ROOT = Path(__file__).resolve().parents[2] / "exchanges"
ADDRESS = "0xabcdef0000000000000000000000000000000001"


def fixture(venue, name):
    return (ROOT / venue / "tests/fixtures" / name).read_text()


class NewExchangeTests(unittest.IsolatedAsyncioTestCase):
    async def asyncSetUp(self):
        self.clients = []
        self.requests = []
        self.commands = []
        self.drop_order = False
        self.auth_rejected = False
        self.http_failure = False
        self.server_errors = []
        self.http_tasks = set()
        self.http_server = await asyncio.start_server(self.http, "127.0.0.1", 0)
        self.http_url = f"http://127.0.0.1:{self.http_server.sockets[0].getsockname()[1]}"
        self.ws_server = await serve(self.ws, "127.0.0.1", 0)
        self.ws_url = f"ws://127.0.0.1:{self.ws_server.sockets[0].getsockname()[1]}"

    async def asyncTearDown(self):
        for c in self.clients:
            await asyncio.wait_for(c.aclose(), 3)
        self.http_server.close()
        await self.http_server.wait_closed()
        self.ws_server.close()
        await asyncio.wait_for(self.ws_server.wait_closed(), 3)
        tasks = list(self.http_tasks)
        for task in tasks:
            task.cancel()
        await asyncio.gather(*tasks, return_exceptions=True)
        self.assertEqual(self.server_errors, [])

    def qfex(self, **kwargs):
        c = QfexClient(http_url=self.http_url, ws_url=self.ws_url,
                       trade_ws_url=self.ws_url, timeout_ms=500, **kwargs)
        self.clients.append(c)
        return c

    def arcus(self, **kwargs):
        c = ArcusClient(http_url=self.http_url, ws_url=self.ws_url,
                        timeout_ms=500, **kwargs)
        self.clients.append(c)
        return c

    async def http(self, reader, writer):
        task = asyncio.current_task()
        self.http_tasks.add(task)
        try:
            head = (await reader.readuntil(b"\r\n\r\n")).decode()
            rows = head.split("\r\n")
            headers = dict((k.lower(), v.strip()) for line in rows[1:] if ":" in line for k, v in [line.split(":", 1)])
            raw = await reader.readexactly(int(headers.get("content-length", 0)))
            body = json.loads(raw) if raw else None
            path = urlsplit(rows[0].split()[1]).path
            self.requests.append((path, headers, body))
            paths = {
                "/refdata": ("qfex", "markets.json"),
                "/md/orderbook/AAPL-USD": ("qfex", "book.json"),
                "/user/positions": ("qfex", "positions.json"),
                "/v1/markets": ("arcus", "markets.json"),
                "/v1/l2OrderBook/BTC-USD": ("arcus", "book.json"),
                "/v1/account": ("arcus", "account.json"),
            }
            if self.http_failure:
                status, payload = 403, '{"error":"secret-echo","errorType":"Denied"}'
            elif path in paths:
                status, payload = 200, fixture(*paths[path])
            elif path in ("/v1/placeOrder", "/v1/cancelOrder"):
                status, payload = 202, '{"status":"ACK","orderId":"123","clientId":"MixedCaseId"}'
            else:
                status, payload = 200, '{}'
            data = payload.encode()
            writer.write(f"HTTP/1.1 {status} Response\r\nContent-Length: {len(data)}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n".encode() + data)
            await writer.drain()
        except (asyncio.IncompleteReadError, ConnectionError):
            pass
        except Exception as error:
            self.server_errors.append(error)
        finally:
            writer.close()
            await writer.wait_closed()
            self.http_tasks.discard(task)

    async def ws(self, socket):
        authenticated = False
        try:
            async for raw in socket:
                msg = json.loads(raw)
                self.commands.append((raw, msg))
                if msg["type"] == "auth":
                    auth = msg["params"]["hmac"]
                    expected = hmac.new(b"public-test-secret", f'{auth["nonce"]}:{auth["unix_ts"]}'.encode(), hashlib.sha256).hexdigest()
                    self.assertEqual(auth["signature"], expected)
                    authenticated = not self.auth_rejected
                    await socket.send(json.dumps({"authenticated": authenticated}))
                elif msg["type"] == "subscribe" and "params" in msg:
                    self.assertTrue(authenticated)
                    await socket.send(json.dumps({"ack_response": msg}))
                elif msg["type"] == "get_user_orders":
                    self.assertTrue(authenticated)
                    await socket.send(json.dumps({"all_orders_response": {"orders": [], "twaps": []}}))
                elif msg["type"] in ("add_order", "cancel_order"):
                    self.assertTrue(authenticated)
                    if self.drop_order:
                        await socket.close()
                        return
                    p = msg["params"]
                    await socket.send(json.dumps({"order_response": {
                        "order_id": p.get("order_id", "123"), "client_order_id": p.get("client_order_id", "MixedCaseId"),
                        "symbol": "AAPL-USD", "status": "ACK" if msg["type"] == "add_order" else "CANCELLED",
                    }}))
                elif msg["type"] == "subscribe" and "channels" in msg:
                    frames = json.loads(fixture("qfex", "stream.json"))
                    for frame in frames:
                        await socket.send(json.dumps(frame))
                elif msg["type"] == "subscribe" and "channel" in msg:
                    frame = json.loads(fixture("arcus", "stream.json"))[1]
                    await socket.send(json.dumps(frame))
        except ConnectionClosed:
            pass
        except Exception as error:
            self.server_errors.append(error)

    async def test_public_reads_and_exact_account_values(self):
        q, a = self.qfex(), self.arcus()
        self.assertEqual((await q.refdata(ticker="AAPL-USD"))["data"][0]["symbol"], "AAPL-USD")
        self.assertEqual((await q.order_book("AAPL-USD"))["bids"][0][0], "300")
        self.assertEqual((await a.markets())["markets"][0]["marketId"], 1)
        self.assertEqual((await a.order_book("BTC-USD"))["bids"][0][0], "70072")
        self.assertEqual((await a.account("0x0000000000000000000000000000000000000001"))["sequenceNumber"], 2**64 - 1)
        with self.assertRaises(AuthenticationError):
            await q.positions()
        await q.aclose()
        with self.assertRaises(ClosedError):
            await q.refdata()

    async def test_qfex_auth_native_numeric_orders_and_cancel(self):
        c = self.qfex(public_key="qfex_pub_test", secret_key="public-test-secret")
        self.assertTrue(c.authenticated)
        positions = await c.positions()
        self.assertEqual(positions["balance"]["available_balance"], Decimal("1000.000000000000000001"))
        self.assertEqual((await c.user_orders(symbol="AAPL-USD"))["orders"], [])
        order = {"symbol": "AAPL-USD", "side": "BUY", "order_type": "LIMIT", "order_time_in_force": "GTC", "quantity": "1.230000000000000001", "price": "200.01", "client_order_id": "MixedCaseId"}
        self.assertEqual((await c.add_order(order))["status"], "ACK")
        raw = next(raw for raw, m in self.commands if m["type"] == "add_order")
        self.assertIn('"quantity":1.230000000000000001', raw)
        result = await c.cancel_order({"symbol": "AAPL-USD", "order_id": "123", "cancel_order_id_type": "order_id"})
        self.assertEqual(result["status"], "CANCELLED")
        self.assertNotIn("public-test-secret", repr(c._inner))

    async def test_arcus_authenticated_order_and_cancel_headers(self):
        signer = ArcusSigner("07" * 32)
        expected = json.loads(fixture("arcus", "signing.json"))
        self.assertEqual(signer.public_key, expected["public_key"])
        self.assertNotIn("07" * 32, repr(signer))
        c = self.arcus(signer=signer, address=ADDRESS.upper().replace("0X", "0x"), account_index=2)
        market = (await c.markets())["markets"][0]
        order = {"marketId": 1, "orderSide": "BUY", "orderType": "LIMIT", "quantity": "0.001", "price": "50000.5", "timeInForce": "ALO", "goodTilTime": "4102444800000000", "clientId": "MixedCaseId"}
        self.assertEqual((await c.place_order(market, order))["status"], "ACK")
        path, headers, body = self.requests[-1]
        self.assertEqual(path, "/v1/placeOrder")
        self.assertEqual(body["address"], ADDRESS)
        self.assertEqual(body["timestamp"], int(headers["x-timestamp"]))
        self.assertGreater(body["timestamp"], 10**18)
        self.assertEqual(len(headers["x-signature"]), 128)
        self.assertEqual(body["accountIndex"], 2)
        self.assertEqual((await c.cancel_order({"marketId": 1, "orderId": "123"}))["status"], "ACK")

    async def test_native_stream_envelopes_and_client_close(self):
        q, a = self.qfex(), self.arcus()
        async with await q.subscribe("level2", "AAPL-USD") as stream:
            self.assertEqual((await anext(stream))["type"], "subscribed")
            self.assertEqual((await anext(stream))["bid"][0], ["300", "0.01"])
        stream = await a.subscribe({"channel": "l2Orderbook", "id": "BTC-USD", "nLevels": 2})
        self.assertEqual((await anext(stream))["contents"]["bids"][0][0], "70072")
        await a.aclose()
        self.assertTrue(stream.closed)

    async def test_auth_failures_and_ambiguous_orders_are_not_replayed(self):
        c = self.qfex(public_key="qfex_pub_test", secret_key="public-test-secret")
        order = {"symbol": "AAPL-USD", "side": "BUY", "order_type": "LIMIT", "order_time_in_force": "GTC", "quantity": "1", "price": "200", "client_order_id": "MixedCaseId"}
        self.auth_rejected = True
        with self.assertRaises(ExchangeError):
            await c.add_order(order)
        self.assertFalse(any(m["type"] == "add_order" for _, m in self.commands))
        self.auth_rejected = False
        self.drop_order = True
        with self.assertRaises(ConnectorError):
            await c.add_order(order)
        self.assertEqual(sum(m["type"] == "add_order" for _, m in self.commands), 1)

    async def test_error_details_remain_inspectable_and_redacted(self):
        c = self.arcus()
        self.http_failure = True
        with self.assertRaises(ExchangeError) as caught:
            await c.markets()
        self.assertEqual(caught.exception.venue, "Arcus")
        self.assertEqual(caught.exception.http_status, 403)
        self.assertIn("secret-echo", caught.exception.body)
        self.assertNotIn("secret-echo", str(caught.exception))
        self.assertNotIn("secret-echo", repr(caught.exception))

    async def test_invalid_credentials_and_native_fields_fail_locally(self):
        with self.assertRaises(AuthenticationError):
            self.qfex(public_key="qfex_pub_test")
        with self.assertRaises(ValidationError):
            ArcusSigner("wrong")
        c = self.arcus()
        with self.assertRaises(ValidationError):
            await c.account(ADDRESS, account_index=10)
        with self.assertRaises(ValidationError):
            await c.subscribe({"channel": "orders", "id": ADDRESS, "accountIndex": 10})
        self.assertEqual(self.requests, [])
