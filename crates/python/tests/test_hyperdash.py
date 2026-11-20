"""Hyperdash contracts through the public Python SDK."""
import asyncio
import json
from pathlib import Path
import unittest
from urllib.parse import parse_qs

from websockets.asyncio.server import serve

import aux_py
import test_data

FIXTURES = Path(__file__).resolve().parents[2] / "data/hyperdash/tests/dispatch.json"
KEY = "hdk_sk_fixture_secret"
CANDLES = (
    b'{"data":[{"t":18446744073709551615,"o":"1.2300","q":null,"extra":true},'
    b'{"o":"2.000"}],"has_more":false,"next_cursor":null,"meta":{}}'
)


class HyperdashTests(unittest.IsolatedAsyncioTestCase):
    asyncSetUp = test_data.DataTests.asyncSetUp
    http = test_data.DataTests.http

    async def asyncTearDown(self):
        self.server.close()
        pending = list(self.tasks)
        for task in pending:
            task.cancel()
        await asyncio.gather(*pending, return_exceptions=True)
        await self.server.wait_closed()

    async def test_public_rest_methods_and_protocol(self):
        fixtures = json.loads(FIXTURES.read_text())
        async with aux_py.HyperdashClient(KEY, api_url=self.base) as client:
            for row in fixtures:
                self.routes[row["method"], row["path"]] = (
                    row["status"], {"X-Credits-Charged": "2"},
                    json.dumps(row["response"]).encode(),
                )
                result = await getattr(client, row["operation"])(row["request"])
                self.assertEqual(result["status"], row["status"])
                self.assertEqual(result["data"], row["response"])
                self.assertEqual(result["metadata"]["credits_charged"], 2)
                method, url, headers, body = self.requests[-1]
                self.assertEqual((method, url.path), (row["method"], row["path"]))
                self.assertEqual(parse_qs(url.query), {key: [value] for key, value in row["query"].items()})
                self.assertEqual(headers.get("authorization"), f"Bearer {KEY}" if row["auth"] else None)
                if row["body"] is not None:
                    self.assertEqual(json.loads(body), row["body"])
                else:
                    self.assertEqual(body, b"")
            route = ("GET", "/v1/markets/xyz%3ATSLA/candles")
            self.routes[route] = (200, {}, CANDLES)
            response = await client.get_candles({"coin": "xyz:TSLA"})
            first, second = response["data"]["data"]
            self.assertEqual(first["t"], 18_446_744_073_709_551_615)
            self.assertEqual(first["o"], "1.2300")
            self.assertIsNone(first["q"])
            self.assertTrue(first["extra"])
            self.assertNotIn("q", second)
            self.routes[route] = (200, {"X-Next-Cursor": "next"}, b"c\r\n1.2300\r\n")
            response = await client.get_candles_csv({"coin": "xyz:TSLA"})
            self.assertEqual(response["data"], "c\r\n1.2300\r\n")
            self.assertEqual(response["metadata"]["next_cursor"], "next")
            self.assertEqual(self.requests[-1][2]["accept"], "text/csv")
            self.routes[route] = (
                429, {"Retry-After": "3"},
                json.dumps({"code": "rate_limited", "detail": f"echoed {KEY}"}).encode(),
            )
            with self.assertRaises(aux_py.DataVendorError) as raised:
                await client.get_candles({"coin": "xyz:TSLA"})
            self.assertEqual(raised.exception.http_status, 429)
            self.assertEqual(raised.exception.code, "rate_limited")
            self.assertEqual(raised.exception.metadata["retry_after"], "3")
            self.assertNotIn(KEY, str(raised.exception.native))
        self.assertTrue(client.closed)

    async def test_multiplexed_stream_ack_and_cancel(self):
        peer_done = asyncio.get_running_loop().create_future()
        finish = asyncio.Event()

        async def peer(socket):
            try:
                self.assertEqual(socket.request.headers["Authorization"], f"Bearer {KEY}")
                await socket.send('{"op":"ready","key_id":"test","environment":"live"}')
                request = json.loads(await socket.recv())
                self.assertEqual(request, {"op": "subscribe", "channel": "trades", "coin": "ETH", "cursor": "resume"})
                await socket.send('{"op":"subscribed","sub":"trades","channel":"trades","coin":"ETH"}')
                await socket.send('{"op":"data","sub":"trades","batch":18446744073709551615,"cursor":"checkpoint","events":[{"px":"1.2300"}],"credits":1}')
                self.assertEqual(json.loads(await socket.recv()), {
                    "op": "ack", "sub": "trades", "batch": 18_446_744_073_709_551_615,
                })
                await socket.send('{"op":"acked","sub":"trades","batch":18446744073709551615,"credits_charged":1}')
                await finish.wait()
                await socket.close(1012, "restart")
            except Exception as error:
                peer_done.set_exception(error)
                raise
            else:
                peer_done.set_result(None)

        async with serve(peer, "127.0.0.1", 0) as server:
            port = server.sockets[0].getsockname()[1]
            try:
                async with aux_py.HyperdashClient(KEY, ws_url=f"ws://127.0.0.1:{port}/v1/stream") as client:
                    async with await client.open_stream() as stream:
                        self.assertEqual((await stream.next())["op"], "ready")
                        await stream.subscribe({"channel": "trades", "coin": "ETH", "cursor": "resume"})
                        self.assertEqual((await stream.next())["op"], "subscribed")
                        batch = await stream.next()
                        self.assertEqual(batch["events"], [{"px": "1.2300"}])
                        self.assertEqual(batch["cursor"], "checkpoint")
                        await stream.ack(batch["sub"], batch["batch"])
                        self.assertEqual((await stream.next())["credits_charged"], 1)
                        with self.assertRaises(aux_py.ValidationError):
                            await stream.ack(batch["sub"], batch["batch"])
                        with self.assertRaises(TimeoutError):
                            await asyncio.wait_for(stream.next(), 0.02)
                        finish.set()
                        with self.assertRaises(aux_py.ConnectorError) as raised:
                            await stream.next()
                        self.assertEqual(raised.exception.close_code, 1012)
                        self.assertEqual(raised.exception.close_reason, "restart")
                        self.assertIsNone(await stream.next())
            finally:
                finish.set()
            await peer_done
