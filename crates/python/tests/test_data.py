"""Local protocol servers; no vendor requests, credentials, or paid jobs."""
import asyncio
from contextlib import suppress
from decimal import Decimal
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from urllib.parse import urlsplit, parse_qs

import aux_py
import websockets

ROOT = Path(__file__).resolve().parents[3]
TD = ROOT / "crates/data/tardis/tests/fixtures"
DB = ROOT / "crates/data/databento/tests/fixtures"
KEY = "a" * 27 + "12345"


class DataTests(unittest.IsolatedAsyncioTestCase):
    async def asyncSetUp(self):
        self.routes = {}
        self.requests = []
        self.tasks = set()
        self.server = await asyncio.start_server(self.http, "127.0.0.1", 0)
        self.base = "http://127.0.0.1:" + str(self.server.sockets[0].getsockname()[1])

    async def asyncTearDown(self):
        self.server.close()
        await self.server.wait_closed()
        for task in self.tasks:
            task.cancel()
        await asyncio.gather(*self.tasks, return_exceptions=True)

    async def http(self, reader, writer):
        task = asyncio.current_task()
        self.tasks.add(task)
        try:
            head = await reader.readuntil(b"\r\n\r\n")
            first, *headers = head.decode().split("\r\n")
            method, target, _ = first.split(" ")
            headers = {key.lower(): value for key, value in (line.split(": ", 1) for line in headers if ": " in line)}
            body = await reader.readexactly(int(headers.get("content-length", 0)))
            request = (method, urlsplit(target), headers, body)
            self.requests.append(request)
            route = self.routes.get((method, request[1].path), (404, {}, b'{}'))
            if callable(route):
                route = route(request)
            if asyncio.iscoroutine(route):
                await route
                return
            status, response_headers, response_body = route
            response_headers = {"Content-Length": str(len(response_body)), "Connection": "close", **response_headers}
            writer.write((f"HTTP/1.1 {status} Fixture\r\n" + "".join(f"{k}: {v}\r\n" for k, v in response_headers.items()) + "\r\n").encode() + response_body)
            await writer.drain()
        except (asyncio.IncompleteReadError, ConnectionError):
            pass
        finally:
            writer.close()
            with suppress(ConnectionError):
                await writer.wait_closed()
            self.tasks.discard(task)

    async def test_tardis_http_inventory_and_precision(self):
        fixtures = json.loads((TD / "http.json").read_text())
        for row in fixtures:
            self.routes["GET", row["path"]] = (200, {}, json.dumps(row["response"]).encode())
        async with aux_py.TardisClient(api_key="fixture-secret", api_url=self.base + "/v1", datasets_url=self.base + "/datasets/v1") as client:
            for row in fixtures:
                name = row["operation"]
                if name in ("exchange_details", "exchange_summary"):
                    value = await getattr(client, name)("bitmex")
                elif name == "instrument":
                    value = await client.instrument("bitmex", "BTC/USD:PERP")
                elif name == "instruments":
                    value = await client.instruments("bitmex", filter={"baseCurrency": "BTC", "active": True})
                elif name == "symbols":
                    value = await client.symbols(exchanges=["bitmex", "deribit"], baseCurrency="BTC", limit=2)
                else:
                    value = await getattr(client, name)()
                self.assertEqual(value, row["response"])
            self.routes["GET", "/v1/data-feeds/bitmex"] = (200, {"Content-Encoding": "gzip"}, (TD / "feed.ndjson.gz").read_bytes())
            async with await client.data_feeds({"exchange": "bitmex", "from": "2024-01-01"}) as stream:
                rows = await stream.read_batch()
                self.assertEqual(rows[0]["message"]["price"], Decimal("123.123456789123456789"))
                self.assertEqual(rows[0]["message"]["id"], 2**64 - 1)
                self.assertEqual(rows[1], {"type": "disconnect"})
            self.assertTrue(stream.closed)
        self.assertTrue(client.closed)
        self.assertTrue(all(r[2]["authorization"] == "Bearer fixture-secret" for r in self.requests))

    async def test_replay_empty_minutes_and_exclusive_end(self):
        def replay(request):
            offset = parse_qs(request[1].query)["offset"][0]
            return 200, {}, b"" if offset == "0" else (TD / "feed.ndjson").read_bytes()
        self.routes["GET", "/v1/data-feeds/bitmex"] = replay
        async with aux_py.TardisClient(api_url=self.base + "/v1") as client:
            async with await client.replay({"exchange": "bitmex", "from": "2024-01-01", "to": "2024-01-01T00:02:00Z"}) as stream:
                self.assertEqual(len(await stream.read_batch()), 2)
                self.assertEqual(await stream.read_batch(), [])
        self.assertEqual([parse_qs(r[1].query)["offset"][0] for r in self.requests], ["0", "1"])

    async def test_databento_http_inventory(self):
        fixtures = json.loads((DB / "http.json").read_text())
        async with aux_py.DatabentoHistoricalClient(KEY, api_url=self.base + "/v0") as history, aux_py.DatabentoReferenceClient(KEY, api_url=self.base + "/v0") as reference:
            for row in fixtures:
                self.routes[row["verb"].upper(), row["path"]] = (200, {}, json.dumps(row["response"]).encode())
                name = row["operation"]
                client = reference if row["path"].split("/")[-1].startswith(("corporate_actions", "security_master", "adjustment_factors")) else history
                value = await getattr(client, name)(row["request"])
                if isinstance(value, aux_py.DataStream):
                    async with value:
                        value = await value.__anext__()
                self.assertEqual(value, json.loads(json.dumps(row["response"]), parse_float=Decimal), name)
                request = self.requests[-1]
                encoded = request[3].decode() if request[3] else request[1].query
                pairs = parse_qs(encoded)
                for key, expected in row["wire_params"].items():
                    self.assertEqual(pairs[key], [expected], (name, key))
        self.assertTrue(all(r[2]["authorization"] == "Basic YWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhYWFhMTIzNDU6" for r in self.requests))

    async def test_dbn_schema_parity_and_raw_timeseries(self):
        raw = (DB / "schemas.dbn").read_bytes()
        self.routes["POST", "/v0/timeseries.get_range"] = (200, {"Content-Encoding": "zstd"}, (DB / "schemas.dbn.zst").read_bytes())
        async with aux_py.DatabentoHistoricalClient(KEY, api_url=self.base + "/v0") as client:
            async with await client.get_range(dataset="GLBX.MDP3", schema="trades", symbols=["ES.c.0"], start="2024-01-01") as stream:
                rows = await stream.read_batch(100)
                self.assertEqual(rows[0]["type"], "metadata")
                expected = json.loads((DB / "schemas.json").read_text())
                self.assertEqual(rows[1:], [row["record"] for row in expected])
                self.assertEqual(await stream.read_batch(), [])
            self.routes["POST", "/v0/timeseries.get_range"] = (200, {}, raw)
            async with await client.timeseries(encoding="dbn", compression="none") as stream:
                self.assertEqual(b"".join([chunk async for chunk in stream]), raw)

    async def test_databento_downloads(self):
        data = (DB / "schemas.dbn").read_bytes()
        storage_headers = []
        async def storage(reader, writer):
            head = await reader.readuntil(b"\r\n\r\n")
            storage_headers.append(head.lower())
            writer.write(f"HTTP/1.1 200 OK\r\nContent-Length: {len(data)}\r\nConnection: close\r\n\r\n".encode() + data)
            await writer.drain()
            writer.close()
        server = await asyncio.start_server(storage, "127.0.0.1", 0)
        location = "http://127.0.0.1:" + str(server.sockets[0].getsockname()[1]) + "/file.dbn?signed=fixture"
        self.routes["GET", "/download"] = (302, {"Location": location}, b"")
        try:
            async with aux_py.DatabentoHistoricalClient(KEY, api_url=self.base + "/v0") as client:
                async with await client.download(self.base + "/download") as stream:
                    self.assertEqual(b"".join([chunk async for chunk in stream]), data)
                with tempfile.TemporaryDirectory() as directory:
                    path = Path(directory) / "data.dbn"
                    self.assertEqual(await client.download_to(self.base + "/download", path), len(data))
                    self.assertEqual(path.read_bytes(), data)
            self.assertEqual(len(storage_headers), 2)
            self.assertTrue(all(b"authorization:" not in head for head in storage_headers))
        finally:
            server.close()
            await server.wait_closed()

    async def test_errors_are_structured_redacted_and_not_retried(self):
        async with aux_py.DatabentoHistoricalClient(KEY, api_url=self.base + "/v0") as client:
            for status, kind in [(401, "authentication"), (403, "entitlement"), (429, "rate_limit"), (500, "vendor")]:
                self.routes["POST", "/v0/batch.submit_job"] = (status, {}, json.dumps({"code": 100, "message": "rejected " + KEY}).encode())
                with self.assertRaises(aux_py.DataVendorError) as caught:
                    await client.submit_job(dataset="GLBX.MDP3")
                error = caught.exception
                self.assertEqual(error.http_status, status)
                self.assertEqual(error.kind, kind)
                self.assertEqual(error.native["message"], "rejected [redacted]")
                self.assertNotIn(KEY, repr(error))
        self.assertEqual(len(self.requests), 4)

    async def test_all_tardis_csv_types(self):
        async with aux_py.TardisClient(datasets_url=self.base + "/v1") as client:
            for fixture in json.loads((TD / "datasets.json").read_text()):
                data = (TD / fixture["file"]).read_bytes()
                self.routes["GET", "/v1/bitmex/" + fixture["data_type"] + "/2024/01/01/XBTUSD.csv.gz"] = (200, {}, data)
                async with await client.dataset({"exchange": "bitmex", "data_type": fixture["data_type"], "date": "2024-01-01", "symbol": "XBTUSD"}) as stream:
                    self.assertEqual(b"".join([chunk async for chunk in stream]), data)

    async def test_download_publication_and_interruption(self):
        data = (TD / "feed.ndjson.gz").read_bytes()
        path = "/v1/bitmex/trades/2024/01/01/XBTUSD.csv.gz"
        self.routes["GET", path] = (200, {}, data)
        async with aux_py.TardisClient(datasets_url=self.base + "/v1") as client:
            request = {"exchange": "bitmex", "data_type": "trades", "date": "2024-01-01", "symbol": "XBTUSD"}
            with tempfile.TemporaryDirectory() as directory:
                destination = Path(directory) / "trades.csv.gz"
                self.assertEqual(await client.download_dataset(request, destination), len(data))
                self.assertEqual(destination.read_bytes(), data)
                self.routes["GET", path] = (200, {"Content-Length": "999"}, b"broken")
                with self.assertRaises(aux_py.TransportError):
                    await client.download_dataset(request, Path(directory) / "incomplete.gz")
                self.assertEqual([p.name for p in Path(directory).iterdir()], ["trades.csv.gz"])

    async def test_cancel_and_client_shutdown_close_pending_read(self):
        arrived = asyncio.Event()
        # Send no body, so next() remains pending after response metadata arrives.
        async def serve(reader, writer):
            await reader.readuntil(b"\r\n\r\n")
            writer.write(b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\n\r\n")
            await writer.drain()
            await reader.read()
            arrived.set()
            writer.close()
        server = await asyncio.start_server(serve, "127.0.0.1", 0)
        url = "http://127.0.0.1:" + str(server.sockets[0].getsockname()[1])
        try:
            for cancel in (True, False):
                arrived.clear()
                client = aux_py.TardisMachineClient(url)
                stream = await client.replay({"exchange": "bitmex"})
                pending = asyncio.create_task(stream.__anext__())
                await asyncio.sleep(0.02)
                with self.assertRaises(aux_py.BusyError):
                    await stream.read_batch(1)
                if cancel:
                    pending.cancel()
                    with self.assertRaises(asyncio.CancelledError):
                        await pending
                    await stream.aclose()
                else:
                    await client.aclose()
                    with self.assertRaises(StopAsyncIteration):
                        await pending
                self.assertTrue(stream.closed)
                await client.aclose()
                await asyncio.wait_for(arrived.wait(), 2)
        finally:
            server.close()
            await server.wait_closed()

    async def test_machine_http_and_websocket_inventory(self):
        normalized = json.loads((TD / "normalized.json").read_text())
        options = [{"exchange": "bitmex", "dataTypes": ["trade"]}]
        for path in ("replay", "replay-normalized"):
            self.routes["GET", "/" + path] = (200, {}, b"\n".join(json.dumps(row).encode() for row in normalized))
        async with aux_py.TardisMachineClient(self.base) as client:
            for method in ("replay", "replay_normalized"):
                async with await getattr(client, method)(options) as stream:
                    self.assertEqual(await stream.read_batch(100), json.loads((TD / "normalized.json").read_text(), parse_float=Decimal))
        paths = []
        async def socket(connection):
            path = urlsplit(connection.request.path)
            paths.append(path.path)
            if path.path == "/ws-replay":
                self.assertEqual(parse_qs(path.query)["session"], ["fixture-session"])
                self.assertEqual(json.loads(await connection.recv()), {"op": "subscribe", "args": ["trade:XBTUSD"]})
            else:
                self.assertEqual(json.loads(parse_qs(path.query)["options"][0]), options)
            for row in normalized:
                await connection.send(json.dumps(row))
        async with websockets.serve(socket, "127.0.0.1", 0) as server:
            url = "http://127.0.0.1:" + str(server.sockets[0].getsockname()[1])
            async with aux_py.TardisMachineClient(self.base, ws_url=url.replace("http://", "ws://")) as client:
                for method in ("ws_replay", "ws_replay_normalized", "stream_normalized"):
                    stream = await client.ws_replay("bitmex", "2024-01-01", "2024-01-02", [{"op": "subscribe", "args": ["trade:XBTUSD"]}], session="fixture-session") if method == "ws_replay" else await getattr(client, method)(options)
                    async with stream:
                        self.assertEqual(await stream.read_batch(len(normalized)), json.loads((TD / "normalized.json").read_text(), parse_float=Decimal))
        self.assertEqual(paths, ["/ws-replay", "/ws-replay-normalized", "/ws-stream-normalized"])

    async def test_live_tcp_protocol_and_parent_cleanup(self):
        commands = []
        closed = asyncio.Event()
        async def serve(reader, writer):
            try:
                writer.write(b"lsg_version=1\ncram=fixture-challenge\n")
                await writer.drain()
                commands.append((await reader.readline()).decode())
                expected = hashlib.sha256(("fixture-challenge|" + KEY).encode()).hexdigest()
                self.assertTrue(commands[-1].startswith("auth=" + expected + "-12345|"))
                writer.write(b"success=1|session_id=python-fixture\n")
                await writer.drain()
                commands.append((await reader.readline()).decode())
                commands.append((await reader.readline()).decode())
                for chunk in [(DB / "ts_out.dbn").read_bytes()[i:i+5] for i in range(0, (DB / "ts_out.dbn").stat().st_size, 5)]:
                    writer.write(chunk)
                    await writer.drain()
                commands.append((await reader.readline()).decode())
                await reader.read()
            finally:
                closed.set()
                writer.close()
        server = await asyncio.start_server(serve, "127.0.0.1", 0)
        gateway = "127.0.0.1:" + str(server.sockets[0].getsockname()[1])
        try:
            async with aux_py.DatabentoLiveClient(KEY) as client:
                session = await client.connect(dataset="GLBX.MDP3", gateway=gateway, ts_out=True)
                self.assertEqual(session.session_id, "python-fixture")
                await session.subscribe(schema="trades", symbols=["ESZ4"], id=7)
                await session.start_session()
                rows = await session.read_batch(2)
                self.assertEqual(rows[0]["type"], "metadata")
                self.assertEqual(rows[1]["ts_out"], "1704067200000000999")
                await session.unsubscribe(schema="trades", symbols=["ESZ4"])
            self.assertTrue(session.closed)
            await asyncio.wait_for(closed.wait(), 2)
            self.assertEqual(commands[1], "schema=trades|stype_in=raw_symbol|symbols=ESZ4|snapshot=0|is_last=1|id=7\n")
            self.assertEqual(commands[2], "start_session\n")
            self.assertEqual(commands[3], "unsubscribe|schema=trades|stype_in=raw_symbol|symbols=ESZ4|is_last=1\n")
        finally:
            server.close()
            await server.wait_closed()


if __name__ == "__main__":
    unittest.main()
