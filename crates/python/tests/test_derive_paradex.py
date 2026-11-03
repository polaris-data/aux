import asyncio
import base64
import contextvars
from collections import deque
from copy import deepcopy
from decimal import Decimal
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
from pathlib import Path
import threading
import time
import unittest

import websockets.asyncio.server
import aux_py as aux

ROOT = Path(__file__).resolve().parents[3]


def fixture(venue, name):
    return json.loads((ROOT / "crates/exchanges" / venue / "tests/fixtures" / name).read_text())


def jwt(expiration=None):
    claims = base64.urlsafe_b64encode(json.dumps({"exp": expiration or int(time.time()) + 3600}).encode()).decode().rstrip("=")
    return f"e30.{claims}.fixture"


class HttpPeer:
    def __init__(self):
        self.expected = deque()
        self.failures = []
        peer = self

        class Handler(BaseHTTPRequestHandler):
            protocol_version = "HTTP/1.1"

            def log_message(self, *_):
                pass

            def handle_request(self):
                try:
                    expected = peer.expected.popleft()
                    body = self.rfile.read(int(self.headers.get("Content-Length", "0")))
                    body = json.loads(body) if body else None
                    assert self.command.lower() == expected["verb"]
                    assert self.path == expected["route"], (self.path, expected["route"])
                    assert body == expected["body"], (body, expected["body"])
                    if expected.get("private"):
                        assert self.headers.get("X-DeriveSignature") or self.headers.get("Authorization")
                    if expected.get("disconnect"):
                        self.close_connection = True
                        return
                    if "authorization" in expected:
                        assert self.headers.get("Authorization") == "Bearer " + expected["authorization"]
                    raw = expected.get("raw") or json.dumps(expected["response"]).encode()
                    self.send_response(expected.get("status", 200))
                    self.send_header("Content-Type", "application/json")
                    self.send_header("Content-Length", str(len(raw)))
                    self.end_headers()
                    self.wfile.write(raw)
                except BaseException as error:
                    peer.failures.append(error)
                    self.send_response(500)
                    self.send_header("Content-Length", "0")
                    self.end_headers()

            do_GET = do_POST = do_PUT = do_DELETE = handle_request

        self.server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)
        self.thread.start()
        self.url = f"http://127.0.0.1:{self.server.server_port}"

    def close(self):
        self.server.shutdown()
        self.server.server_close()
        self.thread.join()
        if self.failures:
            raise self.failures[0]
        assert not self.expected, len(self.expected)


class NativeApiParity(unittest.IsolatedAsyncioTestCase):
    async def test_every_rest_operation_uses_the_rust_contract(self):
        for venue in ("derive", "paradex"):
            peer = HttpPeer()
            inventory = json.loads((ROOT / "crates/exchanges" / venue / "coverage.json").read_text())["rest"]
            contracts = fixture(venue, "contracts.json")
            if venue == "derive":
                client = aux.DeriveClient(http_url=peer.url + "/v3", chain_id=11155111,
                                         matching="0xeB8d770ec18DB98Db922E9D83260A585b9F0DeAD",
                                         signer=aux.DeriveSigner("0x" + "01".zfill(64)), read_retries=0)
            else:
                client = aux.ParadexClient(http_url=peer.url, account="0x1234", token=jwt(), read_only=False, read_retries=0)
            try:
                async with client:
                    for operation in inventory:
                        with self.subTest(venue=venue, method=operation["method"]):
                            contract = deepcopy(contracts[operation["method"]])
                            if venue == "paradex" and operation["method"] == "block_trades_create":
                                signature = next(iter(contract["body"]["signatures"].values()))
                                contract["body"]["signatures"]["0x1234"] = signature
                                contract["request"]["body"] = contract["body"]
                            expected = {**contract, "verb": operation["verb"], "private": operation["private"]}
                            if venue == "derive":
                                expected["response"] = {"result": expected["response"]}
                            peer.expected.append(expected)
                            response = await getattr(client, operation["method"])(contract["request"])
                            if isinstance(contract["response"], dict) and "future_field" in contract["response"]:
                                self.assertEqual(response["future_field"], contract["response"]["future_field"])
                self.assertTrue(client.closed)
            finally:
                await client.aclose()
                await asyncio.to_thread(peer.close)

    async def test_exact_values_structured_errors_and_float_rejection(self):
        peer = HttpPeer()
        try:
            async with aux.ParadexClient(http_url=peer.url, read_retries=0) as client:
                configuration = json.dumps(fixture("paradex", "contracts.json")["get_system_config"]["response"])[:-1]
                raw = (configuration + ',"exact":1.00000000000000000001,"id":1234567890123456789012345678}').encode()
                peer.expected.append({"verb": "get", "route": "/v1/system/config", "body": None, "raw": raw})
                result = await client.get_system_config()
                self.assertEqual(result["exact"], Decimal("1.00000000000000000001"))
                self.assertEqual(result["id"], 1234567890123456789012345678)
                peer.expected.append({"verb": "get", "route": "/v1/system/config", "body": None, "status": 401,
                                      "response": {"error": "INVALID_TOKEN", "message": "fixture rejection"}})
                with self.assertRaises(aux.ExchangeError) as caught:
                    await client.get_system_config()
                self.assertEqual(caught.exception.http_status, 401)
                self.assertEqual(caught.exception.code, "INVALID_TOKEN")
                self.assertEqual(caught.exception.native_message, "fixture rejection")
                self.assertNotIn("fixture rejection", str(caught.exception))
                with self.assertRaises(aux.ValidationError):
                    await client.get_orderbook({"market": "ETH-USD-PERP", "depth": 1.5})
        finally:
            await asyncio.to_thread(peer.close)

    async def test_explicit_authentication_and_ambiguous_write_is_not_replayed(self):
        peer = HttpPeer()
        token = jwt()
        try:
            async with aux.ParadexClient(http_url=peer.url, chain_id="PRIVATE_SN_PARACLEAR_TESTNET",
                                        signer=aux.ParadexSigner("0x1"), account="0x1234", read_retries=3) as client:
                with self.assertRaises(aux.AuthenticationError):
                    await client.get_balance()
                peer.expected.append({"verb": "post", "route": "/v1/auth", "body": None,
                                      "response": {"jwt_token": token}})
                await client.authenticate()
                self.assertTrue(client.authenticated)
                contract = fixture("paradex", "contracts.json")["get_balance"]
                peer.expected.append({**contract, "verb": "get", "authorization": token})
                await client.get_balance(contract["request"])
                cancel = fixture("paradex", "contracts.json")["orders_cancel"]
                peer.expected.append({**cancel, "verb": "delete", "disconnect": True})
                with self.assertRaises(aux.TransportError) as caught:
                    await client.orders_cancel(cancel["request"])
                self.assertEqual(caught.exception.outcome, "unknown")
                peer.expected.append({**cancel, "verb": "delete", "status": 429,
                                      "response": {"error": "RATE_LIMIT", "message": "retry later"}})
                with self.assertRaises(aux.ExchangeError) as caught:
                    await client.orders_cancel(cancel["request"])
                self.assertEqual(caught.exception.http_status, 429)
        finally:
            await asyncio.to_thread(peer.close)

    async def test_reference_signatures_callbacks_and_cancellation(self):
        vectors = fixture("derive", "signing.json")
        local = aux.DeriveSigner(vectors["key"])
        context = contextvars.ContextVar("signing_context")
        context.set("initiating-call")
        seen = []

        async def sign(payload):
            seen.append(context.get())
            return await local.sign(payload)

        remote = aux.DeriveSigner.remote(local.address, sign)
        data = vectors["actions"][0]
        action = {"subaccount_id": "9", "nonce": vectors["nonce"], "module": data["module"],
                  "encoded_data": list(bytes.fromhex(data["encoded"][2:])), "expiry": vectors["expiry"],
                  "owner": vectors["owner"], "signer": vectors["signer"]}
        async with aux.DeriveClient(signer=remote, wallet=vectors["owner"]) as client:
            signature = await client.sign_action(action)
            self.assertEqual(signature, data["vectors"][1]["signature"])
        self.assertEqual(seen, ["initiating-call"])

        entered, cancelled = asyncio.Event(), asyncio.Event()

        async def blocked(_):
            entered.set()
            try:
                await asyncio.Future()
            finally:
                cancelled.set()

        async with aux.DeriveClient(signer=aux.DeriveSigner.remote(local.address, blocked), wallet=vectors["owner"]) as client:
            task = asyncio.create_task(client.sign_action(action))
            await asyncio.wait_for(entered.wait(), 2)
            task.cancel()
            with self.assertRaises(asyncio.CancelledError):
                await task
            await asyncio.wait_for(cancelled.wait(), 2)

    async def test_derive_automatic_metadata_and_native_action_signing(self):
        vectors = fixture("derive", "signing.json")
        action = vectors["actions"][0]
        data = action["data"]
        contracts = fixture("derive", "contracts.json")
        request = deepcopy(contracts["private_order"]["request"])
        request.update(instrument_name="ETH-PERP", direction="buy", amount=data["amount"], limit_price=data["limit_price"],
                       max_fee=data["max_fee"], subaccount_id=9, nonce=vectors["nonce"],
                       signature_expiry_sec=vectors["expiry"], signer="", signature="")
        metadata = deepcopy(contracts["public_get_instrument"]["response"])
        metadata.update(base_asset_address=data["asset"], base_asset_sub_id=data["sub_id"])
        expected = deepcopy(request)
        expected.update(signer=vectors["signer"].lower(), signature=action["vectors"][1]["signature"])
        peer = HttpPeer()
        try:
            async with aux.DeriveClient(http_url=peer.url + "/v3", chain_id=11155111,
                                       matching="0xeB8d770ec18DB98Db922E9D83260A585b9F0DeAD",
                                       wallet=vectors["owner"], signer=aux.DeriveSigner(vectors["key"])) as client:
                for route in ("private/order", "private/order_quote"):
                    peer.expected.append({"verb": "post", "route": "/v3/public/get_instrument", "body": {"instrument_name": "ETH-PERP"},
                                          "response": {"result": metadata}})
                    contract = contracts[route.replace("/", "_")]
                    peer.expected.append({"verb": "post", "route": "/v3/" + route, "body": expected,
                                          "response": {"result": contract["response"]}})
                    # The quote schema omits algo-only fields.
                    if route.endswith("order_quote"):
                        for field in ("algo_duration_sec", "algo_num_slices", "algo_type"):
                            request.pop(field, None)
                            expected.pop(field, None)
                    await getattr(client, route.replace("/", "_"))(request)
                peer.expected.append({"verb": "post", "route": "/v3/public/get_instrument", "body": {"instrument_name": "ETH-PERP"},
                                      "response": {"result": metadata}})
                request["limit_price"] = "0.0000000000001"
                with self.assertRaises(aux.ValidationError):
                    await client.private_order(request)
        finally:
            await asyncio.to_thread(peer.close)

    async def test_paradex_signing_reference_in_both_languages(self):
        fixture_data = fixture("paradex", "signing.json")
        local = aux.ParadexSigner(fixture_data["key"])
        context = contextvars.ContextVar("paradex_signing_context", default="missing")
        context.set("native-loop")
        seen = []

        async def sign(payload):
            seen.append(context.get())
            return await local.sign(payload)

        signer = aux.ParadexSigner.remote(local.public_key, sign)
        for network in ("testnet", "mainnet"):
            async with aux.ParadexClient(network=network, signer=signer, account=fixture_data["account"]) as client:
                for vector in fixture_data["vectors"]:
                    if vector["network"].endswith(network.upper()):
                        if vector["name"] == "onboarding":
                            self.assertEqual(client.onboarding_typed_data(), vector["typed_data"])
                        self.assertEqual(await client.sign_typed_data(vector["typed_data"]), vector["signature"])
        self.assertEqual(set(seen), {"native-loop"})

    async def test_paradex_native_evm_onboarding_and_siwe(self):
        data = fixture("paradex", "siwe.json")
        local = aux.ParadexSigner(data["key"], kind="evm")

        async def sign(payload):
            self.assertIn("personal_message", payload)
            return await local.sign(payload)

        remote = aux.ParadexSigner.remote(local.public_key, sign, kind="evm")
        for network in ("testnet", "mainnet"):
            async with aux.ParadexClient(network=network, signer=remote, account=data["account"]) as client:
                self.assertEqual(await client.evm_account_address(data["address"], data["class_hash"]), data["account"])
                for vector in data["vectors"]:
                    if vector["network"] == network:
                        proof = await client.sign_siwe(vector["purpose"], vector["nonce"], vector["issued_at"], vector["expiration"])
                        self.assertEqual(proof["message"], vector["message"])
                        self.assertEqual(proof["signature"], vector["signature"])
                        self.assertEqual(base64.b64decode(proof["message_base64"]).decode(), proof["message"])


class NativeStreamsAndCommands(unittest.IsolatedAsyncioTestCase):
    async def test_all_paradex_rpc_commands_and_mixed_batch_results(self):
        fixtures = fixture("paradex", "ws.json")
        commands = deque()
        received = []

        async def handler(socket):
            login = json.loads(await socket.recv())
            self.assertEqual(login["method"], "auth")
            await socket.send(json.dumps({"id": login["id"], "result": fixtures["auth"]["response"]}))
            async for text in socket:
                request = json.loads(text)
                method, params, response = commands.popleft()
                self.assertEqual((request["method"], request["params"]), (method, params))
                received.append(method)
                await socket.send(json.dumps({"id": request["id"], "result": response}))

        async with websockets.asyncio.server.serve(handler, "127.0.0.1", 0) as server:
            url = f"ws://127.0.0.1:{server.sockets[0].getsockname()[1]}"
            async with aux.ParadexClient(ws_url=url, account="0x1234", token=jwt(), read_only=False) as client:
                async with await client.open_rpc() as session:
                    async def call(native_method, python_method, params, *args):
                        response = deepcopy(fixtures[native_method.replace(".", "_")]["response"])
                        if native_method == "order.create_batch":
                            response["results"][0].pop("error", None)
                            response["results"].append({"error": "INSUFFICIENT_MARGIN"})
                        commands.append((native_method, params, response))
                        result = await getattr(session, python_method)(*args)
                        if native_method == "order.create_batch":
                            self.assertEqual(result["results"][1]["error"], "INSUFFICIENT_MARGIN")
                            self.assertIn("order", result["results"][0])
                        return result

                    order = deepcopy(fixtures["order_create"]["payload"])
                    await call("order.create", "create_order", order, order)
                    # The official SDK sends an array for create_batch.
                    await call("order.create_batch", "create_batch", [order], [order])
                    modify = deepcopy(fixtures["order_modify"]["payload"])
                    await call("order.modify", "modify_order", modify, "fixture", modify)
                    await call("order.cancel", "cancel_order", {"id": "fixture"}, "fixture")
                    await call("order.cancel", "cancel_by_client_id", {"client_id": "fixture", "market": "fixture"}, "fixture", "fixture")
                    await call("order.cancel_all", "cancel_all", {})
                    await call("order.cancel_batch", "cancel_batch", {"order_ids": ["fixture"]}, ["fixture"])
                    await call("order.cancel_on_disconnect", "set_cancel_on_disconnect", {"enabled": True}, True)
                    request = fixtures["modify_subscription"]["payload"]
                    await call("modify_subscription", "modify_subscription", request, request)
                    for command in ("subscribe", "unsubscribe"):
                        await call(command, command + "_channel", {"channel": "orders.ALL"}, {"type": "orders", "market": "ALL"})
        self.assertFalse(commands)
        self.assertEqual(len(received), 11)

    async def test_expired_token_keeps_socket_alive_but_blocks_rest_and_reconnect(self):
        expiration = int(time.time()) + 2
        send_event, consumed = asyncio.Event(), asyncio.Event()
        connections = []

        async def handler(socket):
            connections.append(socket)
            if len(connections) > 1:
                await socket.wait_closed()
                return
            auth = json.loads(await socket.recv())
            await socket.send(json.dumps({"id": auth["id"], "result": {}}))
            request = json.loads(await socket.recv())
            await socket.send(json.dumps({"id": request["id"], "result": {"channel": "orders.ALL"}}))
            await send_event.wait()
            await socket.send(json.dumps({"method": "subscription", "params": {"channel": "orders.ALL", "data": {"id": "after-expiry"}}}))
            await consumed.wait()
            await socket.close(code=1012)

        async with websockets.asyncio.server.serve(handler, "127.0.0.1", 0) as server:
            url = f"ws://127.0.0.1:{server.sockets[0].getsockname()[1]}"
            async with aux.ParadexClient(ws_url=url, account="0x1234", token=jwt(expiration), initial_backoff_ms=1, max_backoff_ms=1) as client:
                async with await client.subscribe({"type": "orders", "market": "ALL"}) as stream:
                    await asyncio.sleep(max(0, expiration - time.time() + 0.1))
                    with self.assertRaises(aux.AuthenticationError):
                        await client.get_balance()
                    send_event.set()
                    while True:
                        event = await asyncio.wait_for(anext(stream), 2)
                        if "params" in event:
                            self.assertEqual(event["params"]["data"]["id"], "after-expiry")
                            consumed.set()
                            break
                    with self.assertRaises(aux.GapError):
                        await anext(stream)
                    with self.assertRaises(aux.AuthenticationError):
                        await asyncio.wait_for(anext(stream), 2)
                client.set_token(jwt())

    async def test_every_derive_rpc_operation_uses_native_parameters(self):
        contracts = fixture("derive", "contracts.json")
        inventory = json.loads((ROOT / "crates/exchanges/derive/coverage.json").read_text())["rest"]
        native_methods = {op["path"].lstrip("/"): op["method"] for op in inventory}
        methods = []

        async def handler(socket):
            login = json.loads(await socket.recv())
            await socket.send(json.dumps({"id": login["id"], "result": [9]}))
            async for text in socket:
                request = json.loads(text)
                method = native_methods[request["method"]]
                methods.append(method)
                self.assertEqual(request["params"], contracts[method]["request"])
                await socket.send(json.dumps({"id": request["id"], "result": contracts[method]["response"]}))

        async with websockets.asyncio.server.serve(handler, "127.0.0.1", 0) as server:
            url = f"ws://127.0.0.1:{server.sockets[0].getsockname()[1]}"
            async with aux.DeriveClient(ws_url=url, chain_id=11155111, matching="0xeB8d770ec18DB98Db922E9D83260A585b9F0DeAD",
                                       signer=aux.DeriveSigner("0x" + "01".zfill(64))) as client:
                async with await client.open_rpc() as session:
                    for method, contract in contracts.items():
                        with self.subTest(method=method):
                            await getattr(session, method)(contract["request"])
        self.assertEqual(set(methods), set(contracts))

    async def test_rpc_close_interrupts_a_waiting_reader(self):
        for venue in ("derive", "paradex"):
            entered = asyncio.Event()

            async def handler(socket):
                request = json.loads(await socket.recv())
                await socket.send(json.dumps({"id": request["id"], "result": [9] if venue == "derive" else {}}))
                entered.set()
                await socket.wait_closed()

            async with websockets.asyncio.server.serve(handler, "127.0.0.1", 0) as server:
                url = f"ws://127.0.0.1:{server.sockets[0].getsockname()[1]}"
                options = {"ws_url": url}
                if venue == "derive":
                    options.update(chain_id=11155111, matching="0xeB8d770ec18DB98Db922E9D83260A585b9F0DeAD",
                                   signer=aux.DeriveSigner("0x" + "01".zfill(64)))
                else:
                    options.update(account="0x1234", token=jwt(), read_only=False)
                async with getattr(aux, venue.title() + "Client")(**options) as client:
                    session = await client.open_rpc()
                    await entered.wait()
                    reader = asyncio.create_task(session.next_event())
                    await asyncio.sleep(0.01)
                    await asyncio.wait_for(session.aclose(), 2)
                    with self.assertRaises(aux.ClosedError):
                        await reader
                    await session.aclose()

    async def test_paradex_recovery_reports_gap_and_requires_fresh_snapshot(self):
        book = next(f for f in fixture("paradex", "sbe.json")["frames"] if f["template"] == 3)
        connections = []
        first_snapshot_consumed = asyncio.Event()

        async def handler(socket):
            connections.append(socket)
            request = json.loads(await socket.recv())
            await socket.send(json.dumps({"id": request["id"], "result": {"channel": request["params"]["channel"]}}))
            await socket.send(bytes.fromhex(book["hex"]))
            if len(connections) == 1:
                await first_snapshot_consumed.wait()
                await socket.close(code=1012)
            else:
                await socket.recv()

        async with websockets.asyncio.server.serve(handler, "127.0.0.1", 0) as server:
            url = f"ws://127.0.0.1:{server.sockets[0].getsockname()[1]}"
            async with aux.ParadexClient(ws_url=url, initial_backoff_ms=1, max_backoff_ms=1) as client:
                stream = await client.subscribe({"type": "order_book", "market": "ETH-USD-PERP", "feed_type": "deltas", "refresh_rate": "100ms"})
                snapshots, gaps = 0, 0
                try:
                    while snapshots < 2:
                        try:
                            event = await asyncio.wait_for(anext(stream), 3)
                        except aux.GapError:
                            gaps += 1
                            continue
                        if "params" in event:
                            self.assertNotIn("fixture_delta", event["params"]["data"])
                            self.assertEqual(event["params"]["data"]["pkgType"], "SNAPSHOT")
                            snapshots += 1
                            first_snapshot_consumed.set()
                finally:
                    await stream.aclose()
                self.assertEqual(gaps, 1)
        self.assertEqual(len(connections), 2)

    async def test_paradex_rejects_delta_before_snapshot(self):
        async def handler(socket):
            request = json.loads(await socket.recv())
            await socket.send(json.dumps({"method": "subscription", "params": {
                "channel": request["params"]["channel"], "data": {"update_type": "d"}}}))
            await socket.wait_closed()

        async with websockets.asyncio.server.serve(handler, "127.0.0.1", 0) as server:
            url = f"ws://127.0.0.1:{server.sockets[0].getsockname()[1]}"
            async with aux.ParadexClient(ws_url=url) as client:
                async with await client.subscribe({"type": "order_book", "market": "ETH-USD-PERP", "feed_type": "deltas", "refresh_rate": "100ms"}) as stream:
                    with self.assertRaises(aux.TransportError) as caught:
                        await asyncio.wait_for(anext(stream), 2)
                    self.assertEqual(caught.exception.kind, "protocol")

    async def test_paradex_mixed_json_and_sbe_stream(self):
        frames = fixture("paradex", "sbe.json")["frames"]
        trade = next(f for f in frames if f["template"] == 1)
        requests = []

        async def handler(socket):
            self.assertIn("sbeSchemaVersion=3", socket.request.path)
            request = json.loads(await socket.recv())
            requests.append(request)
            await socket.send(json.dumps({"id": request["id"], "result": {"channel": request["params"]["channel"]}}))
            await socket.send(bytes.fromhex(trade["hex"]))
            request = json.loads(await socket.recv())
            self.assertEqual(request["method"], "unsubscribe")

        async with websockets.asyncio.server.serve(handler, "127.0.0.1", 0) as server:
            url = f"ws://127.0.0.1:{server.sockets[0].getsockname()[1]}"
            async with aux.ParadexClient(ws_url=url) as client:
                async with await client.subscribe({"type": "trades", "market": "ETH-USD-PERP"}) as stream:
                    async for event in stream:
                        if "params" in event:
                            self.assertEqual(event["params"]["data"]["tradeIdStr"], "1234567890123456789012345678")
                            self.assertEqual(event["params"]["data"]["price"], trade["native"]["price"])
                            self.assertEqual(event["sbe"]["schema_version"], 3)
                            break
        self.assertEqual(len(requests), 1)

    async def test_paradex_cancelled_command_reconciles_without_replay(self):
        received, release = asyncio.Event(), asyncio.Event()
        commands = []

        async def handler(socket):
            request = json.loads(await socket.recv())
            self.assertEqual(request["method"], "auth")
            await socket.send(json.dumps({"id": request["id"], "result": {"node_id": "fixture"}}))
            request = json.loads(await socket.recv())
            commands.append(request)
            received.set()
            await release.wait()
            await socket.send(json.dumps({"id": request["id"], "result": {"order_id": "123", "status": "CANCELED"}}))
            request = json.loads(await socket.recv())
            commands.append(request)
            await socket.send(json.dumps({"id": request["id"], "result": {"status": "ok"}}))
            await socket.wait_closed()

        async with websockets.asyncio.server.serve(handler, "127.0.0.1", 0) as server:
            url = f"ws://127.0.0.1:{server.sockets[0].getsockname()[1]}"
            async with aux.ParadexClient(ws_url=url, account="0x1234", token=jwt(), read_only=False) as client:
                async with await client.open_rpc() as session:
                    task = asyncio.create_task(session.cancel_order("123"))
                    await asyncio.wait_for(received.wait(), 2)
                    task.cancel()
                    with self.assertRaises(asyncio.CancelledError):
                        await task
                    for _ in range(100):
                        try:
                            pending = session.pending
                            break
                        except aux.BusyError:
                            await asyncio.sleep(0.001)
                    self.assertEqual(pending["method"], "order.cancel")
                    with self.assertRaises(aux.OutcomeUncertainError):
                        await session.cancel_order("456")
                    release.set()
                    self.assertEqual((await session.reconcile())["order_id"], "123")
                    self.assertIsNone(session.pending)
                    # Renewing REST credentials does not change this socket's
                    # already established permissions or require a re-login.
                    client.set_token(jwt(), read_only=True)
                    await session.cancel_all()
        self.assertEqual([c["method"] for c in commands], ["order.cancel", "order.cancel_all"])

    async def test_derive_native_rpc_and_account_stream(self):
        requests = []

        async def handler(socket):
            request = json.loads(await socket.recv())
            requests.append(request)
            self.assertEqual(request["method"], "public/login")
            await socket.send(json.dumps({"id": request["id"], "result": [9]}))
            request = json.loads(await socket.recv())
            requests.append(request)
            if request["method"] == "subscribe":
                channel = request["params"]["channels"][0]
                await socket.send(json.dumps({"id": request["id"], "result": {"status": {channel: "ok"}, "current_subscriptions": [channel]}}))
                await socket.send(json.dumps({"method": "subscription", "params": {"channel": channel, "data": [{"order_id": "fixture"}]}}))
                await socket.recv()
            else:
                self.assertEqual(request["method"], "public/get_time")
                await socket.send(json.dumps({"id": request["id"], "result": fixture("derive", "contracts.json")["public_get_time"]["response"]}))
                await socket.wait_closed()

        async with websockets.asyncio.server.serve(handler, "127.0.0.1", 0) as server:
            url = f"ws://127.0.0.1:{server.sockets[0].getsockname()[1]}"
            async with aux.DeriveClient(ws_url=url, chain_id=11155111, matching="0xeB8d770ec18DB98Db922E9D83260A585b9F0DeAD",
                                       signer=aux.DeriveSigner("0x" + "01".zfill(64))) as client:
                async with await client.open_rpc() as session:
                    self.assertIsNotNone(await session.public_get_time())
                async with await client.subscribe({"type": "orders", "subaccount_id": 9}) as stream:
                    async for event in stream:
                        if "params" in event:
                            self.assertEqual(event["params"]["channel"], "9.orders")
                            break
        self.assertEqual(sum(r["method"] == "public/login" for r in requests), 2)


if __name__ == "__main__":
    unittest.main()
