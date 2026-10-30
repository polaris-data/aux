"""Exercise the installed native extension against offline Python servers."""
from __future__ import annotations

import asyncio
import copy
import gc
import json
import unittest
from pathlib import Path

from websockets.asyncio.server import serve
from websockets.exceptions import ConnectionClosed

from aux_py import (
    BusyError, ClosedError, ConnectorError, DecodeError, ExchangeError,
    HyperliquidClient, TransportError, ValidationError,
)

FIXTURES = Path(__file__).resolve().parents[2] / "exchanges/hyperliquid/tests/fixtures"
META = json.loads((FIXTURES / "meta.json").read_text())
BOOK = json.loads((FIXTURES / "book.json").read_text())


async def bounded(awaitable):
    return await asyncio.wait_for(awaitable, timeout=3)


class Peer:
    def __init__(self, socket):
        self.socket = socket
        self.unsubscribed: dict[str, asyncio.Event] = {}
        self.closed = asyncio.Event()

    async def book(self, coin="ETH"):
        data = copy.deepcopy(BOOK)
        data["coin"] = coin
        await self.socket.send(json.dumps({"channel": "l2Book", "data": data}))


class BindingTests(unittest.IsolatedAsyncioTestCase):
    async def asyncSetUp(self):
        self.requests = []
        self.http_tasks = set()
        self.http_seen = asyncio.Event()
        self.http_delay = 0
        self.http_status = 200
        self.http_body = None
        self.http_hang = False
        self.meta = copy.deepcopy(META)
        self.meta["futureId"] = 2**64 - 1
        self.meta["nullable"] = None
        self.meta["flags"] = [True, False]
        self.meta["marginTables"][0][1]["marginTiers"][0]["lowerBound"] = "0.00000000000000000100"
        self.new_subscriptions = asyncio.Queue()
        self.peers = []
        self.server_errors = []
        self.http_server = await asyncio.start_server(self.handle_http, "127.0.0.1", 0)
        self.http_url = f"http://127.0.0.1:{self.http_server.sockets[0].getsockname()[1]}"
        self.ws_server = await serve(self.handle_ws, "127.0.0.1", 0)
        self.ws_url = f"ws://127.0.0.1:{self.ws_server.sockets[0].getsockname()[1]}/ws"
        self.clients = []
        self.client = self.make_client()

    def make_client(self, **options):
        options.setdefault("http_url", self.http_url)
        options.setdefault("ws_url", self.ws_url)
        options.setdefault("timeout_ms", 500)
        options.setdefault("read_retries", 0)
        options.setdefault("reconnect_retries", 0)
        client = HyperliquidClient(**options)
        self.clients.append(client)
        return client

    async def asyncTearDown(self):
        for client in self.clients:
            await bounded(client.aclose())
        self.http_server.close()
        await self.http_server.wait_closed()
        self.ws_server.close()
        await bounded(self.ws_server.wait_closed())
        tasks = list(self.http_tasks)
        for task in tasks:
            task.cancel()
        await asyncio.gather(*tasks, return_exceptions=True)
        self.assertEqual(self.server_errors, [])

    async def handle_http(self, reader, writer):
        task = asyncio.current_task()
        self.http_tasks.add(task)
        try:
            raw = await reader.readuntil(b"\r\n\r\n")
            headers = raw.decode("ascii").split("\r\n")
            self.assertTrue(headers[0].startswith("POST /info HTTP/1.1"))
            length = next(int(line.split(":", 1)[1]) for line in headers if line.lower().startswith("content-length:"))
            request = json.loads(await reader.readexactly(length))
            self.requests.append(request)
            self.http_seen.set()
            if self.http_hang:
                await reader.read()
                return
            if self.http_delay:
                await asyncio.sleep(self.http_delay)
            body = self.http_body
            if body is None:
                body = json.dumps(self.meta if request["type"] == "meta" else BOOK)
            encoded = body.encode()
            writer.write(
                f"HTTP/1.1 {self.http_status} Response\r\nContent-Length: {len(encoded)}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n".encode()
                + encoded
            )
            await writer.drain()
        except (ConnectionError, asyncio.IncompleteReadError):
            pass
        except Exception as error:
            self.server_errors.append(error)
        finally:
            writer.close()
            try:
                await writer.wait_closed()
            except ConnectionError:
                pass
            self.http_tasks.discard(task)

    async def handle_ws(self, socket):
        peer = Peer(socket)
        self.peers.append(peer)
        try:
            async for raw in socket:
                message = json.loads(raw)
                method = message["method"]
                if method == "subscribe":
                    specification = message["subscription"]
                    self.assertEqual(specification["type"], "l2Book")
                    peer.unsubscribed[specification["coin"]] = asyncio.Event()
                    await socket.send(json.dumps({"channel": "subscriptionResponse", "data": message}))
                    await self.new_subscriptions.put((peer, specification))
                elif method == "unsubscribe":
                    peer.unsubscribed[message["subscription"]["coin"]].set()
                elif method == "ping":
                    await socket.send('{"channel":"pong"}')
                else:
                    self.fail(f"unexpected method {method}")
        except ConnectionClosed:
            pass
        except Exception as error:
            self.server_errors.append(error)
        finally:
            peer.closed.set()

    async def subscribe(self, coin="ETH", client=None):
        stream = await bounded((client or self.client).subscribe_l2_book(coin))
        peer, specification = await bounded(self.new_subscriptions.get())
        self.assertEqual(specification, {"type": "l2Book", "coin": coin})
        return stream, peer

    async def assert_cleaned_up(self, peer, coin="ETH", *, closed=True):
        await bounded(peer.unsubscribed[coin].wait())
        if closed:
            await bounded(peer.closed.wait())

    async def test_exact_native_values_and_responsive_asyncio_loop(self):
        self.http_delay = 0.05
        ticks = 0
        done = False

        async def ticker():
            nonlocal ticks
            while not done:
                ticks += 1
                await asyncio.sleep(0.005)

        ticking = asyncio.create_task(ticker())
        try:
            meta = await bounded(self.client.meta())
            book = await bounded(self.client.l2_book("ETH", n_sig_figs=5, mantissa=2))
        finally:
            done = True
            await ticking
        self.assertGreater(ticks, 2)
        self.assertEqual(meta["futureId"], 2**64 - 1)
        self.assertIsInstance(meta["futureId"], int)
        self.assertIsNone(meta["nullable"])
        self.assertEqual(meta["flags"], [True, False])
        self.assertEqual(meta["marginTables"][0][1]["marginTiers"][0]["lowerBound"], "0.00000000000000000100")
        self.assertEqual(book["levels"][0][0]["px"], "3000.00")
        self.assertEqual(book["levels"][0][0]["sz"], "1.2300")
        self.assertEqual(self.requests, [{"type": "meta"}, {"type": "l2Book", "coin": "ETH", "nSigFigs": 5, "mantissa": 2}])

    async def test_validation_happens_before_io(self):
        for options in [{"network": "unknown"}, {"http_url": self.http_url}, {"timeout_ms": 0}, {"subscription_capacity": 0}]:
            with self.assertRaises(ValidationError):
                HyperliquidClient(**options)
        with self.assertRaises(ValidationError):
            await self.client.l2_book("ETH", n_sig_figs=1)
        with self.assertRaises(ValidationError):
            await self.client.subscribe_l2_book(" ")
        self.assertEqual(self.requests, [])
        self.assertEqual(self.peers, [])

    async def test_structured_http_native_and_decode_errors(self):
        self.http_status = 429
        self.http_body = "rate limited"
        with self.assertRaises(ExchangeError) as caught:
            await bounded(self.client.meta())
        self.assertIsInstance(caught.exception, ConnectorError)
        self.assertEqual(caught.exception.http_status, 429)
        self.assertEqual(caught.exception.body, "rate limited")
        self.http_status = 200
        self.http_body = '{"status":"err","response":"native rejection"}'
        with self.assertRaises(ExchangeError) as caught:
            await bounded(self.client.meta())
        self.assertIsNone(caught.exception.http_status)
        self.assertEqual(caught.exception.native_message, "native rejection")
        self.http_body = "not JSON"
        with self.assertRaises(DecodeError):
            await bounded(self.client.meta())
        self.assertEqual(len(self.requests), 3)  # No automatic retry.

    async def test_transport_timeout_has_kind_and_submission_outcome(self):
        self.http_hang = True
        client = self.make_client(timeout_ms=100)
        with self.assertRaises(TransportError) as caught:
            await bounded(client.meta())
        self.assertEqual(caught.exception.kind, "timeout")
        self.assertEqual(caught.exception.outcome, "unknown")
        self.assertNotIn(self.http_url, str(caught.exception))

    async def test_async_iteration_and_context_close(self):
        stream, peer = await self.subscribe()
        async with stream:
            await peer.book()
            async for book in stream:
                self.assertEqual(book["coin"], "ETH")
                self.assertEqual(book["levels"][0][0]["px"], "3000.00")
                self.assertEqual(book["time"], BOOK["time"])
                break
        self.assertTrue(stream.closed)
        await self.assert_cleaned_up(peer)
        with self.assertRaises(StopAsyncIteration):
            await anext(stream)
        await stream.aclose()  # Idempotent.

    async def test_cancelling_pending_read_closes_native_subscription(self):
        stream, peer = await self.subscribe()
        pending = asyncio.create_task(anext(stream))
        await asyncio.sleep(0)  # Start the Python read, before any book data.
        pending.cancel()
        with self.assertRaises(asyncio.CancelledError):
            await bounded(pending)
        self.assertTrue(stream.closed)
        await self.assert_cleaned_up(peer)
        await bounded(stream.aclose())

    async def test_close_wakes_pending_read_and_can_be_awaited_concurrently(self):
        stream, peer = await self.subscribe()
        pending = asyncio.create_task(anext(stream))
        await asyncio.sleep(0)
        await bounded(asyncio.gather(stream.aclose(), stream.aclose()))
        with self.assertRaises(StopAsyncIteration):
            await bounded(pending)
        await self.assert_cleaned_up(peer)

    async def test_competing_read_does_not_steal_or_close_the_active_read(self):
        stream, peer = await self.subscribe()
        pending = asyncio.create_task(anext(stream))
        await asyncio.sleep(0)
        with self.assertRaises(BusyError):
            await anext(stream)
        self.assertFalse(stream.closed)
        await peer.book()
        self.assertEqual((await bounded(pending))["coin"], "ETH")
        await stream.aclose()
        await self.assert_cleaned_up(peer)

    async def test_dropping_idle_python_object_releases_native_resources(self):
        stream, peer = await self.subscribe()
        del stream
        gc.collect()
        await self.assert_cleaned_up(peer)

    async def test_client_close_closes_all_streams_and_pending_reads(self):
        first, peer = await self.subscribe("ETH")
        second, same_peer = await self.subscribe("BTC")
        self.assertIs(peer, same_peer)
        pending = [asyncio.create_task(anext(first)), asyncio.create_task(anext(second))]
        await asyncio.sleep(0)
        await bounded(self.client.aclose())
        for task in pending:
            with self.assertRaises(StopAsyncIteration):
                await bounded(task)
        self.assertTrue(self.client.closed)
        self.assertTrue(first.closed and second.closed)
        await self.assert_cleaned_up(peer, "ETH")
        await self.assert_cleaned_up(peer, "BTC")
        with self.assertRaises(ClosedError):
            await self.client.meta()

    async def test_client_close_cancels_pending_http_read(self):
        self.http_hang = True
        pending = asyncio.create_task(self.client.meta())
        await bounded(self.http_seen.wait())
        await bounded(self.client.aclose())
        with self.assertRaises(ClosedError):
            await bounded(pending)

    async def test_overflow_is_explicit_and_terminal(self):
        client = self.make_client(subscription_capacity=1)
        stream, peer = await self.subscribe(client=client)
        try:
            for _ in range(3):
                await peer.book()
        except ConnectionClosed:
            pass
        await self.assert_cleaned_up(peer)
        with self.assertRaises(TransportError) as caught:
            await bounded(anext(stream))
        self.assertEqual(caught.exception.kind, "overflow")
        self.assertTrue(stream.closed)

    async def test_malformed_stream_is_reported_and_closed(self):
        stream, peer = await self.subscribe()
        await peer.socket.send('{"channel":"l2Book","data":{"coin":"ETH"}}')
        with self.assertRaises(DecodeError):
            await bounded(anext(stream))
        self.assertTrue(stream.closed)
        await self.assert_cleaned_up(peer)

    async def test_native_stream_error_is_reported_and_closed(self):
        stream, peer = await self.subscribe()
        await peer.socket.send('{"channel":"error","data":"invalid subscription"}')
        with self.assertRaises(ExchangeError) as caught:
            await bounded(anext(stream))
        self.assertEqual(caught.exception.native_message, "invalid subscription")
        self.assertTrue(stream.closed)
        await self.assert_cleaned_up(peer)

    async def test_disconnect_is_reported_and_terminal(self):
        stream, peer = await self.subscribe()
        await peer.socket.close()
        with self.assertRaises(TransportError) as caught:
            await bounded(anext(stream))
        self.assertEqual(caught.exception.kind, "disconnected")
        self.assertTrue(stream.closed)


class ConstructorTests(unittest.TestCase):
    def test_construct_outside_asyncio_then_close_inside_a_loop(self):
        client = HyperliquidClient()
        self.assertFalse(client.closed)
        asyncio.run(client.aclose())
        self.assertTrue(client.closed)


if __name__ == "__main__":
    unittest.main()
