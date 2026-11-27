"""Run against the installed Python wheel. All network peers are loopback fixtures."""

import asyncio
import base64
import gzip
import hashlib
import hmac
from decimal import Decimal
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import time
import threading
import sys
from urllib.parse import parse_qsl, urlsplit
import urllib.request

import aux_py as sdk

HERE = Path(__file__).resolve().parent


def loads(text):
    return json.loads(text, parse_float=Decimal)


def dump(value):
    if isinstance(value, Decimal):
        return str(value)
    if isinstance(value, dict):
        return (
            "{"
            + ",".join(json.dumps(k) + ":" + dump(v) for k, v in value.items())
            + "}"
        )
    if isinstance(value, list):
        return "[" + ",".join(map(dump, value)) + "]"
    return json.dumps(value, allow_nan=False)


def fixture(name):
    return loads((HERE / (name + ".json")).read_text())


def remote_pascal(vector, **options):
    local = sdk.PascalSigner(vector["auth"]["seed"])

    async def sign(payload):
        assert payload["public_key"] == local.public_key and payload["scheme"] == "pascal_ed25519"
        return await local._message(bytes.fromhex(payload["message_hex"]))

    return sdk.PascalClient(owner=vector["auth"]["owner"], signer=sdk.PascalSigner.remote(local.public_key, sign), **options)


def test_solana():
    f = fixture("pascal-solana")
    params = {k:f[k] for k in ("owner","source","destination","mint","amount","decimals")}
    params.update(recent_blockhash=f["blockhash"],last_valid_block_height=2**64-1)
    assert sdk.solana.associated_token_address(f["owner"],f["mint"]) == f["associated_token_address"]
    tx = sdk.solana.prepare_deposit(params)
    assert bytes(tx["message"]).hex() == f["message_hex"]
    signed = sdk.solana.sign_deposit("07"*32,tx)
    assert signed == {"signature":f["signature"],"transaction_base64":f["transaction_base64"],"last_valid_block_height":2**64-1}
    for bad in ({**tx,"amount":1},{**tx,"message":[0]*214}):
        try:
            sdk.solana.sign_deposit("07"*32,bad)
        except ValueError:
            pass
        else:
            raise AssertionError("signed an inconsistent message")


def test_hyperdash_exports():
    base = {r["operation"]:r for r in fixture("data-http") if r["venue"] == "hyperdash"}
    supported = (HERE.parent / "src/data/hyperdash/text-routes.txt").read_text().splitlines()

    class Handler(BaseHTTPRequestHandler):
        def respond(self):
            row, accept, payload = self.server.expected
            target = urlsplit(self.path)
            assert self.command == row["verb"] and target.path == row["path"]
            assert dict(parse_qsl(target.query)) == row["query"]
            assert self.headers["accept"] == accept
            if row["private"]:
                assert self.headers["Authorization"] == "Bearer test-key"
            self.rfile.read(int(self.headers.get("Content-Length",0)))
            self.send_response(200)
            self.send_header("X-Next-Cursor","after/+=")
            self.end_headers()
            self.wfile.write(payload)
        do_GET = do_POST = do_DELETE = respond
        def log_message(self, *_):
            pass

    async def run(server):
        c=sdk.HyperdashClient(origin=f"http://127.0.0.1:{server.server_port}",api_key="test-key")
        for name in supported:
            row=base[name]
            for suffix,accept,payload in (("_csv","text/csv",b'id,name\r\n1,"a,b"\r\n'),("_ndjson","application/x-ndjson",b'{"id":1}\n{"id":2}\n')):
                server.expected=row,accept,payload
                result=await getattr(c,name+suffix)(row["request"])
                assert result == {"status":200,"metadata":{"next_cursor":"after/+="},"data":payload.decode()}, name
    with ThreadingHTTPServer(("127.0.0.1",0),Handler) as server:
        worker=threading.Thread(target=server.serve_forever,daemon=True)
        worker.start()
        try:
            asyncio.run(run(server))
        finally:
            server.shutdown()
            worker.join()


async def test_live(gateway):
    key = fixture("auth")["databento"]["key"]
    try:
        await sdk.DatabentoLiveClient(key, timeout_ms=400, idle_timeout_ms=100).connect(dataset="IDLE", gateway=gateway)
    except sdk.TransportError:
        pass
    else:
        raise AssertionError("Databento idle timeout did not interrupt the native handshake")
    client = sdk.LiveClient(key)
    async def connect(dataset, **opts):
        return await client.connect({"dataset": dataset}, gateway=gateway, **opts)
    try:
        await connect("DENY")
    except ValueError:
        pass
    else:
        raise AssertionError("missing gateway authentication error")
    expected = [r["record"] for r in fixture("schema-records")]
    for compression in ("none", "zstd"):
        async with await connect("BATCH", compression=compression, heartbeat_interval_s=3, slow_reader_behavior="warn") as session:
            assert session.session_id == "fixture-session" and await session.metadata() is None
            for invalid in ({"symbols":["A,B"]}, {"symbols":["ok","bad|field"]}, {"symbols":[]}, {"symbols":["ok"],"snapshot":True,"start":"1"}):
                try:
                    await session.subscribe(dict(schema="mbo", **invalid))
                except ValueError:
                    continue
                raise AssertionError(invalid)
            await session.subscribe(dict(schema="mbo", symbols=list(map(str,range(501))), start="1741000000000000000", id=7))
            await session.unsubscribe(schema="mbo", symbols=["500"])
            await session.start_session()
            first = await anext(session)
            assert first["metadata"] == await session.metadata()
            assert await session.read_batch(3) == expected[:3]
            assert [record async for record in session] == expected[3:]
    session = await connect("BLOCK")
    await session.start_session()
    task = asyncio.create_task(anext(session, None))
    await asyncio.sleep(0.03)
    await session.close()
    assert await asyncio.wait_for(task, 2) is None
    await session.close()


async def test_named_arguments():
    from unittest.mock import AsyncMock, patch
    from aux_py import clients

    cases = [
        (sdk.HyperliquidClient, "l2_book", ("ETH",), {"n_sig_figs": 4, "mantissa": 2}, {"coin":"ETH", "nSigFigs":4,"mantissa":2}),
        (sdk.HyperliquidClient, "order", ({"orders":[]},), {"nonce":7,"expires_after":8}, {"orders":[],"options":{"nonce":7,"expires_after":8}}),
        (sdk.ArcusClient, "order_book", ("BTC-USD",), {}, {"market":"BTC-USD","nLevels":20}),
        (sdk.ArcusClient, "open_orders", (), {"address":"example"}, {"address":"example","accountIndex":0}),
        (sdk.ArcusClient, "place_order", ({"marketId":1},{"quantity":"0.5"}), {}, {"market":{"marketId":1},"body":{"quantity":"0.5"}}),
        (sdk.LighterClient, "next_nonce", (37,3), {}, {"account_index":37,"api_key_index":3}),
        (sdk.LighterClient, "account", (), {"account_index":0}, {"by":"index","value":0}),
        (sdk.LighterClient, "account", (), {"l1_address":"example"}, {"by":"l1_address","value":"example"}),
        (sdk.LighterClient, "recent_trades", (12,), {}, {"market_id":12,"limit":50}),
        (sdk.LighterClient, "cancel_order", ({"MarketIndex":1,"Index":2},), {"expired_at":1234,"attributes":{1:4}},
         {"order":{"MarketIndex":1,"Index":2},"options":{"expired_at":1234,"attributes":{"1":4}}}),
        (sdk.MonacoClient, "cancel_all", (), {"trading_pair_id":"spot"}, {"tradingPairId":"spot"}),
        (sdk.MonacoClient, "replace_order", ({"id":"spot"},"old",{"quantity":"1.00"}), {},
         {"market":{"id":"spot"},"orderId":"old","body":{"quantity":"1.00"}}),
        (sdk.TardisClient, "instruments", ("bitmex", {"symbol":"BTC"}), {"type":"spot"}, {"exchange":"bitmex","symbol":"BTC","type":"spot"}),
        (sdk.HistoricalClient, "list_schemas", ({"dataset":"BATCH"},), {"start":"yesterday"}, {"dataset":"BATCH","start":"yesterday"}),
    ]
    with patch.object(clients, "execute", new_callable=AsyncMock) as native:
        native.return_value = {}
        for cls, method, args, kwargs, expected in cases:
            client = cls()
            await getattr(client, method)(*args, **kwargs)
            assert native.await_args.args[2] == expected, (cls, method, native.await_args)
            await client.aclose()
        try:
            await sdk.HistoricalClient().list_schemas({"dataset":"one"}, dataset="two")
        except TypeError:
            pass
        else:
            raise AssertionError("duplicate Databento parameters accepted")


async def test_shared_peer(origin):
    n1_endpoint = origin.replace("http:", "ws:") + "/caller-websocket/n1/sdk"
    async with await sdk.N1Client().subscribe(
        {"subscriptions": [{"channel": "liquidations"}]}, endpoint=n1_endpoint
    ) as stream:
        assert (await anext(stream))["liquidation"]["update_id"] == 42
    await test_named_arguments()
    hermes = sdk.PythHermesClient(origin=origin + "/stream-refused", api_key="fixture-stream-credential")
    qfex = sdk.QfexClient()
    for venue, request, detail in (
        ("pyth_hermes", hermes.stream_price_updates({"ids":["ab"*32]}), "invalid Bearer [redacted]"),
        ("qfex", qfex.subscribe({"channel":"bbo","symbol":"BTC"}, endpoint=origin.replace("http:", "ws:")+"/stream-refused/ws"), "subscription denied"),
    ):
        try:
            await request
        except sdk.ExchangeError as error:
            assert (error.venue, error.status, error.body) == (venue, 403, {"message":detail, "seq":9007199254740993})
        else:
            raise AssertionError("stream opened after an HTTP rejection")
    phoenix_client = sdk.PhoenixClient()
    for name, row in fixture("phoenix-instructions").items():
        assert phoenix_client.build_instruction(name, row["request"]) == row["instruction"], name
    rows = fixture("phoenix-transactions")
    for row in rows:
        prepared = sdk.PhoenixClient().prepare_transaction(row["request"])
        assert prepared == row["prepared"]
        signatures = sdk.phoenix.sign_transaction("01"*32, prepared)
        local_key = sdk.PhoenixSigner("01"*32)
        async def sign_phoenix(proof):
            assert proof["public_key"] == local_key.public_key and proof["scheme"] == "phoenix_ed25519"
            return await local_key.sign_message(proof["message_base64"])
        external = sdk.PhoenixSigner.remote(local_key.public_key, sign_phoenix)
        assert await external.sign_transaction(prepared) == signatures
        try:
            sdk.phoenix.assemble_transaction(prepared, signatures)
            raise AssertionError("missing signer accepted")
        except sdk.ValidationError:
            pass
        signatures = sdk.phoenix.sign_transaction("02"*32, prepared, signatures)
        assert signatures == row["signatures"]
        signed = sdk.phoenix.assemble_transaction(prepared, signatures)
        assert signed == row["signed"]
        sdk.phoenix.validate_server_transaction(prepared, signed["transaction_base64"])
        sdk.phoenix.validate_signed_transaction(signed)
        try:
            sdk.phoenix.validate_signed_transaction({**signed,"signature":"1"*64})
            raise AssertionError("invalid signature accepted")
        except sdk.ConnectorError:
            pass
    row = rows[0]
    chain = sdk.PhoenixChainClient(origin+"/phoenix-rpc/python")
    assert await chain.prepare(row["request"]["instructions"],row["request"]["fee_payer"]) == row["prepared"]
    assert (await chain.simulate(row["signed"]))["value"]["err"] is None
    assert await chain.submit(row["signed"]) == row["signed"]["signature"]
    assert chain.pending == row["signed"]
    assert (await chain.signature_status(row["signed"]["signature"]))["value"][0]["confirmationStatus"] == "confirmed"
    assert (await chain.transaction(row["signed"]["signature"]))["slot"] == 9007199254740993
    chain.clear_reconciled_submission()
    assert chain.pending is None
    lost = sdk.PhoenixChainClient(origin+"/phoenix-rpc/python/lost")
    for _ in range(2):
        try:
            await lost.submit(row["signed"])
            raise AssertionError("lost acknowledgement accepted")
        except sdk.OutcomeUncertainError:
            assert lost.pending == row["signed"]
    lost.clear_reconciled_submission()
    ws_url = origin.replace("http:", "ws:") + "/rpc/"
    async def sign_kalshi(message):
        assert message.endswith(b"GET/external-kalshi/portfolio/balance") and b"?" not in message
        return bytes.fromhex(sdk.arcus_sign("07"*32,message.decode())["signature"])
    external_kalshi = sdk.KalshiClient(origin=origin+"/external-kalshi",api_key="fixture-k",signer={"sign":sign_kalshi})
    assert await external_kalshi.get_balance({"subaccount":1}) == {"balance":123}
    async with sdk.HyperliquidClient() as feed:
        endpoint = origin.replace("http:", "ws:")+"/multiplex/python"
        eth = await feed.subscribe_l2_book("ETH",endpoint=endpoint)
        btc = await feed.subscribe_trades("BTC",endpoint=endpoint)
        duplicate = await feed.subscribe_trades("BTC",endpoint=endpoint)
        sol = await feed.subscribe_l2_book("SOL",endpoint=endpoint)
        assert (await anext(eth))["time"] == 1
        assert (await anext(btc))["data"][0]["coin"] == "BTC"
        assert (await anext(duplicate))["data"][0]["coin"] == "BTC"
        assert (await anext(sol))["time"] == 2
        await btc.close()
        await eth.close()
        await duplicate.close()
        assert (await anext(sol))["time"] == 3
    for name, feed, params in (
        ("lighter", sdk.LighterClient(), {"channel":"order_book","market_id":4}),
        ("qfex", sdk.QfexClient(), {"channel":"bbo","symbol":"BTC"}),
    ):
        endpoint = origin.replace("http:", "ws:") + "/rejected-feed/" + name
        async with feed:
            rejected = await feed.subscribe(params, endpoint=endpoint)
            try:
                await anext(rejected)
            except sdk.ExchangeError as err:
                assert err.venue == name and err.body == {"error":{"sequence":9007199254740993,"message":"unknown market"}}
            else:
                raise AssertionError("missing venue rejection")
    async with sdk.HyperliquidClient() as feed:
        endpoint = origin.replace("http:", "ws:") + "/account-scope/python"
        first, second = "0x"+"11"*20, "0x"+"22"*20
        updates = await feed.subscribe({"type":"orderUpdates","user":first}, endpoint=endpoint)
        orders = await feed.subscribe({"type":"openOrders","user":first}, endpoint=endpoint)
        other = await feed.subscribe({"type":"orderUpdates","user":second}, endpoint=endpoint)
        fills = await feed.subscribe({"type":"userFills","user":first,"aggregateByTime":True}, endpoint=endpoint)
        assert (await anext(updates))["data"][0]["order"]["oid"] == 1
        assert (await anext(orders))["data"]["user"] == first
        assert (await anext(other))["data"][0]["order"]["oid"] == 2
        assert (await anext(fills))["data"]["fills"] == []
    async with sdk.QfexClient() as feed:
        endpoint = origin.replace("http:", "ws:") + "/qfex-multiplex/python"
        trades = await feed.subscribe("trade", "*", endpoint=endpoint)
        books = await feed.subscribe("bbo", "BTC", endpoint=endpoint)
        assert [(await anext(trades))["seq"] for _ in range(2)] == [1, 2]
        assert (await anext(books))["seq"] == 9007199254740993
        await trades.close()
        assert (await anext(books))["seq"] == 3
    async with sdk.LighterClient() as lighter_feed:
        endpoint = origin.replace("http:", "ws:") + "/lighter-multiplex/python"
        book = await lighter_feed.subscribe_order_book(4, endpoint=endpoint)
        account = await lighter_feed.subscribe_account(18, endpoint=endpoint)
        candles = await lighter_feed.subscribe({"channel": "candle", "market_id": 4}, endpoint=endpoint)
        for generation in (1, 2):
            assert (await anext(book))["sequence"] == generation
            assert (await anext(account))["sequence"] == generation
            for kind in ("subscribed", "update"):
                candle = await anext(candles)
                assert (candle["type"], candle["sequence"]) == (f"{kind}/candle", generation)
                assert candle["candles"][0]["i"] == 9007199254740993
        late = await lighter_feed.subscribe_order_book(4, endpoint=endpoint)
        assert (await anext(late))["sequence"] == 3
        await late.close()
        await book.close()
        assert (await anext(account))["sequence"] == 4
    client = sdk.HyperliquidClient()
    async with client:
        book = await client.subscribe_l2_book("ETH",endpoint=ws_url.replace("/rpc/","/lifecycle/python"))
        assert await anext(book) == {"coin":"ETH","time":23,"levels":[[],[]]}
    assert client.closed and book.closed
    await client.aclose()
    try:
        await client.meta({})
        raise AssertionError("closed client dispatched a read")
    except sdk.ClosedError:
        pass
    import asyncio
    started = asyncio.Event()
    async def blocked_signer(_):
        started.set()
        await asyncio.Future()
    waiting = sdk.KalshiClient(origin=origin+"/never-send",api_key="fixture-k",signer={"sign":blocked_signer})
    work = asyncio.create_task(waiting.get_balance({}))
    await asyncio.wait_for(started.wait(),2)
    await waiting.aclose()
    try:
        await asyncio.wait_for(work,2)
        raise AssertionError("pending signature survived client shutdown")
    except sdk.ClosedError:
        pass
    chain_row = fixture("metric-chain")
    metric_chain = sdk.MetricChainClient(chain_id=1, read_rpc=origin+"/metric-chain/python")
    async with sdk.PolymarketChainClient(1,origin+"/poly-contract/python") as poly_chain:
        for entry in fixture("polymarket-abi"):
            fn = (entry["interface"],entry["function"],"0x"+"11"*20,entry["args"])
            assert poly_chain.prepare_call(*fn)["data"] == entry["calldata"]
            if entry["function"] == "allowance":
                assert await poly_chain.call(*fn,block="safe") == [2**88+9]
        try:
            poly_chain.decode_error("IERC20","0xffffffff")
        except sdk.ValidationError:
            pass
        else:
            raise AssertionError("unknown revert selector accepted")
    assert poly_chain.closed
    contracts = sdk.MetricChainClient(1, origin+"/metric-contract/python")
    address = chain_row["request"]["pool"]
    compare(await contracts.contract("AnchoredPriceProvider",address).bps_base(block="safe"), ["-3"])
    vectors = {(r["contract"],r["signature"]):r for r in fixture("metric-abi") if r["kind"] == "function"}
    methods = (HERE.parent/"src/exchanges/metric/schema/metric-contract-methods.tsv").read_text().splitlines()
    assert len(vectors) == len(methods)
    for line in methods:
        contract, signature, method, _, mode = line.split("\t")
        vector = vectors[contract,signature]
        fn = getattr(contracts.contract(contract,address),method)
        if mode == "prepare":
            result = fn(vector["args"],value="0x10")
            assert result == {"to":address.lower(),"data":vector["input"],"value":"0x10"}
        else:
            async def read(to,data,block,sender,value):
                assert to == address.lower() and data == vector["input"] and block == "finalized" and sender is None and value == "0x0"
                return vector["output"]
            contracts.reader.eth_call = read
            compare(await fn(vector["args"],block="finalized"),vector["outputArgs"])
    state = chain_row["request"]
    assert await metric_chain.oracle_stop_loss_status(state) == chain_row["expected"]
    assert await metric_chain.liquidity_snapshot(state["pool"],state["dataProvider"],[-1,0,1],block="safe") == chain_row["snapshot"]
    for path in chain_row["routes"]:
        assert await metric_chain.prepare_route(path["request"]) == path["expected"]
    for row in fixture("eip712"):
        td=row["typed_data"]
        if "order" in row:
            assert sdk.polymarket.order_typed_data(row["order"],row["domain"]) == td
        assert sdk.polymarket.typed_data_hash(td) == row["digest"]
    x=fixture("extended-onboarding")
    ext=sdk.ExtendedClient(origin=origin)
    assert ext.onboarding_payload(x["registration"]) == x["AccountRegistration"]
    assert ext.key_derivation_payload(x["registration"]) == x["AccountCreation"]
    assert ext.api_request_payload("/action","2024-01-05T01:08:56Z")["digest"] == x["api_digest"]
    assert ext.scale_decimal("-1.23",3) == -1230
    derived=sdk.ExtendedSigner.from_wallet_signature(x["wallet_signature"])
    assert derived.public_key == x["public_key"]
    assert len(derived.sign_digest(ext.onboarding_l2_hash(x["registration"]["wallet"],derived.public_key))) == 2
    siwe=fixture("paradex-siwe")
    for row in siwe["vectors"]:
        signing=sdk.ParadexClient(origin=origin,bearer={"account":"0x1234","network":"PRIVATE_SN_PARACLEAR_"+row["network"].upper(),"evm_key":siwe["key"]})
        assert signing.evm_account_address(siwe["address"],siwe["class_hash"]) == siwe["account"]
        proof=await signing.sign_siwe(row["purpose"],row["nonce"],row["issued_at"],row["expiration"])
        assert proof["message"] == row["message"] and proof["signature"] == row["signature"]
    for key in ("stark_key","evm_key"):
        signing=sdk.ParadexClient(origin=origin,bearer={"account":"0x1234","network":"PRIVATE_SN_PARACLEAR_TESTNET",key:siwe["key"]})
        await signing.authenticate()
        assert signing._bearer["token"].endswith(".fixture") and signing._bearer["read_only"] is False
        assert signing.onboarding_typed_data() == next(v["typed_data"] for v in fixture("paradex-signing")["vectors"] if v["name"]=="onboarding")
    dune = sdk.DuneClient(origin=origin+"/dune-args", api_key="test-key")
    for row in (r for r in fixture("data-http") + fixture("text-http") if r["venue"] == "dune"):
        params = loads(row["request_json"]) if "request_json" in row else dict(row["request"])
        options = {}
        for key in ("execution_id", "query_id"):
            if key in params:
                options[key] = params.pop(key)
        if params:
            options["request"] = params.get("body", params)
        response = await getattr(dune, row["operation"])(**options)
        assert response == (loads(row["response_json"]) if "response_json" in row else row["expected"])
    hl_row = fixture("evm-signing")[0]
    hl = sdk.HyperliquidClient(signer={"key":hl_row["key"],"vault_address":"0x"+"02"*20,"network":"Testnet"})
    assert hl.authenticated and hl.signer_address == hl_row["address"] and hl.account_address == "0x"+"02"*20
    assert not sdk.HyperliquidClient().authenticated
    blocks = fixture("paradex-blocks")
    proofs = fixture("paradex-signing")
    for vector in proofs["vectors"]:
        if vector["name"] not in blocks:
            continue
        block = blocks[vector["name"]]
        for kind, field, signature in (("stark_key", "key", "signature"), ("evm_key", "evm_key", "evm_signature")):
            auth = {kind:proofs[field], "account":proofs["account"], "network":vector["network"]}
            response = await sdk.ParadexClient(origin=origin,bearer=auth).block_signature(**block)
            assert json.loads(response["signature_data"]) == vector[signature]
            assert response["signature_expiration"] == block["expiration"]
            assert response["signer_account"] == proofs["account"]
            assert abs(response["signature_timestamp"] - 1000*time.time()) < 5000
    assert sdk.PascalClient().wallet_outcome_pending is False
    d = fixture("derive-signing")
    for row in d["actions"]:
        for proof in row["vectors"]:
            client = sdk.DeriveClient(api_key=d["owner"], api_secret=d["key"], chain_id=proof["chain_id"],
                                      origin="https://fixture.invalid")
            body = dict(nonce=d["nonce"], subaccount_id=d["subaccount_id"], signature_expiry_sec=d["expiry"], extra="kept")
            signed = await client.prepare_signed(body, row["data"])
            assert signed == {**body, "signature": proof["signature"], "signer": d["signer"].lower()}
            raw = dict(owner=d["owner"], signer=d["signer"], module=row["module"], encoded_data=list(bytes.fromhex(row["encoded"][2:])),
                       subaccount_id=d["subaccount_id"], nonce=d["nonce"], expiry=d["expiry"])
            assert await client.sign_action(raw) == proof["signature"]
    for name in ("owner", "signer"):
        try:
            await client.sign_action({**raw, name: "0x"+"00"*20})
        except sdk.ConnectorError:
            pass
        else:
            raise AssertionError("accepted mismatched " + name)
    generated = await client.prepare_signed({"nonce": 0}, {"type": "encoded", "module": raw["module"], "data": []})
    assert isinstance(generated["nonce"], int) and generated["nonce"] > 0
    assert abs(generated["signature_expiry_sec"] - time.time() - 300) < 5
    refs = [r for r in fixture("http") if r["venue"] == "derive"]
    orders = fixture("derive-orders")["orders"]
    methods = dict(line.split() for line in (HERE.parent / "src/exchanges/derive/rpc-routes.txt").read_text().splitlines())
    async with await sdk.DeriveClient(api_key=d["owner"],api_secret=d["key"],chain_id=1).open_rpc(endpoint=ws_url+"derive/full") as rpc:
        for row in refs:
            name = row["operation"]
            if name not in methods:
                continue
            base = name.removeprefix("private_")
            if name.startswith("private_") and (base in orders or base.removesuffix("_debug") in orders):
                vector = orders.get(base) or orders[base.removesuffix("_debug")]
                assert await getattr(rpc,name)(vector["params"]) == {"signed":methods[name]}
            elif name == "public_get_all_currencies" and not row["request"]:
                assert await rpc.public_get_all_currencies() == fixture("derive-orders")["metadata"]["currencies"]
            else:
                assert await getattr(rpc,name)(row["request"]) == row["response"],name
        assert await rpc.set_cancel_on_disconnect({"enabled":True}) == {"enabled":True}
        assert await rpc.subscribe_channels({"channels":["trades"]}) == {"channels":["trades"]}
        assert (await rpc.next_event())["params"]["seq"] == 9007199254740993
        await rpc.unsubscribe_channels({"channels":["trades"]})
    async with await sdk.DeriveClient().open_rpc(endpoint=ws_url+"derive-public") as rpc:
        try:
            await rpc.subscribe_channels({"channels":["rejection-fixture"]})
        except sdk.ExchangeError as error:
            assert error.venue == "derive" and error.body["error"]["data"]["seq"] == 9007199254740993
        else:
            raise AssertionError("venue rejection was lost")
        try:
            await rpc.private_cancel({})
        except sdk.AuthenticationError:
            pass
        else:
            raise AssertionError("public session sent private command")
        await rpc.unsubscribe_channels({"channels":[]})
    golden = fixture("polymarket-perps-signing")
    perps = sdk.PolymarketPerpsClient(perps={"proxy":golden["proxy"],"secret":"fixture","proxy_key":golden["key"]})
    perps_rows = {r["request"]["body"]["op"]["type"]: r["request"]["body"]["op"]["args"]
                  for r in fixture("prediction-http") if r["venue"]=="polymarket" and "op" in r["request"].get("body",{}) and "args" in r["request"]["body"]["op"]}
    async with await perps.open_rpc(endpoint=ws_url+"perps/full") as rpc:
        for op,method in (("create_orders","createOrders"),("modify_orders","modifyOrders"),("modify_orders_coid","modifyOrdersCOID"),
                          ("cancel_orders","cancelOrders"),("cancel_orders_coid","cancelOrdersCOID"),("cancel_all","cancelAll"),
                          ("auto_cancel","autoCancel"),("update_leverage","updateLeverage"),("update_leverages","updateLeverages"),("update_margin","updateMargin")):
            assert (await getattr(rpc,op)(perps_rows[method]))["data"] == method
        await rpc.subscribe(["trades"])
        await rpc.unsubscribe(["trades"])
        await rpc.ping()
    row = fixture("polymarket-clob-orders")[0]
    clob={"address":golden["proxy"],"key":"key","secret":"c2VjcmV0","passphrase":"pass"}
    async with await sdk.PolymarketClient(clob=clob,wallet={"key":golden["key"],"order_domain":row["domain"]}).open_rpc(endpoint=ws_url+"rfq/full") as rpc:
        for op,method in (("quote","RFQ_QUOTE"),("cancel_quote","RFQ_QUOTE_CANCEL"),("last_look","RFQ_CONFIRMATION_RESPONSE")):
            params={"rfq_id":"r","quote_id":"q"}
            if op=="quote":
                params["signed_order"]=row["order"]
            assert (await getattr(rpc,op)(params))["type"]=="ACK_"+method
            assert (await rpc.next_event())["rfq_id"]=="other"
            assert (await rpc.next_event())["quote_id"]=="different"
    pd = sdk.ParadexClient(bearer={"token":"fixture.eyJleHAiOjQxMDI0NDQ4MDB9.fixture","read_only":True})
    async with await pd.open_rpc(endpoint=ws_url+"paradex/readonly") as rpc:
        try:
            await rpc.cancel_order("forbidden")
        except sdk.AuthenticationError:
            pass
        else:
            raise AssertionError("read-only Paradex session sent an order")
        ch = {"type":"orders","market":"ETH-USD-PERP"}
        assert (await rpc.subscribe_channel(ch))["ok"] == "subscribe"
        assert (await rpc.next_event())["params"]["seq"] == 9007199254740993
        assert (await rpc.unsubscribe_channel(ch))["ok"] == "unsubscribe"
    for client, path in [(sdk.KalshiClient,"kalshi"),(sdk.KalshiPerpsClient,"kalshi_perps")]:
        rpc = await client(api_key="fixture-k",api_secret="07"*32).open_rpc(endpoint=ws_url+path,timeout_ms=80)
        try:
            await rpc.subscribe({"step":1})
            raise AssertionError("expected an acknowledgement timeout")
        except sdk.ConnectorError:
            pass
        assert rpc.pending == {"id":1,"method":"subscribe"}
        try:
            await rpc.unsubscribe({"step":2})
            raise AssertionError("unexpected second dispatch")
        except sdk.ConnectorError:
            pass
        assert (await rpc.next_event())["value"] == 9007199254740993
        await asyncio.sleep(.3)
        assert (await rpc.reconcile())["ack"] == "subscribe" and rpc.pending is None
        for idx, method in enumerate(("list_subscriptions","update_subscription","unsubscribe"),2):
            assert (await getattr(rpc,method)({"step":idx}))["ack"] == method
        await rpc.aclose()
        await rpc.aclose()
    permits = fixture("pascal-permits")
    for client in (sdk.PascalClient(permit=permits[0]["auth"]), remote_pascal(permits[0])):
        rpc = await client.open_rpc(endpoint=ws_url+"pascal")
        assert (await rpc.ping())["ack"] == "ping"
        assert (await rpc.next_event())["event"] == "queued"
        for i,method in enumerate(("place_orders","cancel_orders","mass_cancel")):
            assert (await getattr(rpc,method)(permits[i]["body"]))["ack"] == ("place","cancel","mass_cancel")[i]
        await rpc.aclose()
    identity = {"address":"0xABcdef0000000000000000000000000000000001", "account_index":2}
    arcus = sdk.ArcusClient(origin=origin, signer={**identity, "seed":"07"*32})
    for r in fixture("data-http"):
        if r["venue"] == "arcus" and r["private"]:
            assert await getattr(arcus, r["operation"])(r["request"]) == r["response"]
            unsigned = sdk.ArcusClient(signer=identity).signing_payload(r["operation"], r["request"])
            signed = sdk.arcus_sign("07"*32, unsigned["payload"])
            external = sdk.ArcusClient(origin=origin, signer={**identity, **signed, "timestamp":unsigned["timestamp"]})
            assert await getattr(external,r["operation"])(r["request"]) == r["response"]
    ws = origin.replace("http:", "ws:")
    async with await sdk.HyperdashClient(api_key="hyperdash-key").open_stream(endpoint=ws+"/hd/stream") as stream:
        assert (await anext(stream))["op"] == "ready"
        pending = asyncio.create_task(anext(stream))
        await stream.subscribe({"channel":"trades","coin":"BTC"})
        batch = await asyncio.wait_for(pending, 3)
        assert batch["batch"] == 9007199254740993
        try:
            await stream.ack(batch["sub"], str(batch["batch"]))
        except ValueError:
            pass
        else:
            raise AssertionError("accepted an acknowledgement with a different ID type")
        await stream.ack(batch["sub"], batch["batch"])
        assert (await anext(stream))["credits_charged"] == 2
        await stream.unsubscribe(batch["sub"])
        assert (await anext(stream))["detail"] == "[redacted]"
        assert (await anext(stream))["op"] == "unsubscribed"
    async def reader(key):
        return await sdk.ChainlinkDataStreamsClient(api_key=key,api_secret="secret").subscribe_reports({"feedIDs":["ab"*32]},endpoint=ws+"/shared/python/")
    one, other, two = await reader("one"), await reader("one"), await reader("two")
    def step(n):
        return sdk.MetricClient(origin=origin+"/shared/python",api_key=str(n)).chains({})
    try:
        assert await step(1) == {"one":1,"two":1}
        assert (await anext(one)) == (await anext(other))
        assert (await anext(two))["report"]["fullReport"] == "two"
        await one.close()
        await step(2)
        assert (await anext(other))["report"]["seq"] == 2
    finally:
        for sub in (one,other,two):
            await sub.close()
    chainlink = sdk.ChainlinkDataStreamsClient(api_key="fixture", api_secret="secret")
    async with await chainlink.subscribe_reports(request={"feedIDs":["0x"+"01"*32, "ab"*32]},endpoint=ws+"/reports/") as stream:
        assert (await anext(stream))["report"]["sequence"] == 9007199254740993
        try:
            await anext(stream)
        except ValueError:
            pass
        else:
            raise AssertionError("invalid report envelope")
    async with await sdk.ArcusClient().subscribe({"channel":"l2OrderbookUpdates","id":"BTC","nLevels":10},endpoint=ws+"/arcus/ws") as book:
        assert (await anext(book))["contents"] == {"bids":[],"asks":[]}
        assert (await anext(book))["contents"]["seq"] == 9007199254740993
    lighter = fixture("lighter-signing")["vectors"]
    def lighter_order(tx):
        return {k:v for k,v in tx.items() if k not in ("AccountIndex","ApiKeyIndex","Sig","Nonce","ExpiredAt","L2TxAttributes")}
    for row in lighter:
        if row["kind"] == "auth":
            continue
        tx = row["tx_info"]
        client = sdk.LighterClient(origin=origin,signer=dict(key=row["key"],chain_id=row["chain_id"],account_index=tx["AccountIndex"],api_key_index=tx["ApiKeyIndex"]))
        params = dict(expired_at=tx["ExpiredAt"],attributes=tx["L2TxAttributes"])
        prepared = await client.prepare_transaction(row["kind"]+"_order",lighter_order(tx),params,tx["Nonce"])
        assert prepared["digest"] == row["digest"]
    first = lighter[0]
    client = sdk.LighterClient(origin=origin,signer=dict(key=first["key"],chain_id=300,account_index=37,api_key_index=3))
    assert (await client.active_orders({"market_id":4096}))["orders"] == []
    assert (await client.next_nonce({"account_index":37,"api_key_index":3}))["nonce"] == 42
    metric = sdk.MetricClient(origin=origin, api_key="fixture")
    last = sdk.MetricClient(origin=origin, api_key="last")
    assert await metric.chains(request={}) == await last.chains(params={})
    try:
        await metric.chains({}, params={})
    except TypeError:
        pass
    else:
        raise AssertionError("accepted two request objects")
    try:
        await sdk.MetricClient(origin=origin, api_key="fail").chains({})
    except sdk.ExchangeError as err:
        assert (err.venue, err.status, err.body) == ("metric", 429, "fixture rate limit")
    else:
        raise AssertionError("missing HTTP error")
    for row in fixture("data-http"):
        if row["venue"] == "metric" and row["verb"] == "POST":
            assert await getattr(metric, row["operation"])(row["request"]) == row["response"]
    params = {"expired_at":1800000600000}
    assert (await client.create_order(lighter_order(first["tx_info"]),params))["tx_hash"] == "fixture-hash"
    for _ in range(2):
        try:
            await client.cancel_order(lighter_order(lighter[1]["tx_info"]),params)
        except sdk.OutcomeUncertainError:
            pass
        else:
            raise AssertionError("ambiguous order requires nonce reconciliation")
    assert await client.resync_nonce() == 42
    token = await client.auth_token(int(time.time())+60)
    assert len(bytes.fromhex(token.split(":")[3])) == 80
    for row in fixture("perps-ws"):
        client = sdk.HyperliquidClient() if row["venue"]=="hyperliquid" else sdk.LighterClient(signer=dict(chain_id=300,account_index=37,api_key_index=3,key="01"+"00"*39))
        url = origin.replace("http:","ws:")+"/perps-ws/"+row["venue"]+"/"+row["name"]
        async with await client.subscribe(row["params"],endpoint=url) as sub:
            assert await anext(sub) == row["event"]
            assert await anext(sub,None) is None
    vectors = fixture("hyperliquid-signing")
    for row in vectors["user_signed"]:
        action = {k:v for k,v in row["action"].items() if k not in ("hyperliquidChain", "signatureChainId")}
        action["type"] = "usdSend" if row["name"] == "usd_send" else "approveAgent"
        client = sdk.HyperliquidClient(signer={"key":vectors["public_test_private_key"],"network":row["network"]})
        assert client.user_signing_payload(action,row["network"],row["signature_chain_id"]) == row["digest"]
        assert client.sign_user_action(action,row["signature_chain_id"]) == row["signature"]
    row = vectors["l1"][0]
    async def sign_order(digest):
        assert digest == row["digest"]
        return row["signature"]
    cfg = {"address":"0x14791697260e4c9a71f18484c9f997b308e59325","network":"Mainnet","sign":sign_order}
    order = {"orders":row["action"]["orders"],"options":{"nonce":row["nonce"]}}
    assert (await sdk.HyperliquidClient(origin=origin,signer=cfg).order(order))["status"] == "ok"
    try:
        await sdk.HyperliquidClient(origin=origin, signer={**cfg,"address":"0x"+"22"*20}).order(order)
    except ValueError:
        pass
    else:
        raise AssertionError("accepted an external signature for another address")
    for row in vectors["l1"]:
        client = sdk.HyperliquidClient(origin=origin, signer=dict(key=vectors["public_test_private_key"],network=row["network"],vault_address=row["vault_address"]))
        action = dict(row["action"])
        name = action.pop("type")
        action["options"] = {"nonce":row["nonce"], "expires_after":row["expires_after"]}
        method = getattr(client, "cancel_by_cloid" if name == "cancelByCloid" else name)
        assert (await method(action))["response"]["data"]["statuses"][1] == {"error":"fixture rejection"}
    urls = [origin.replace("http:", "ws:")+"/pro/"+name for name in ("a","b")]
    pro = sdk.PythProClient(api_key="fixture")
    subscription = dict(channel="real_time",symbols=["BTC/USD"],formats=["evm"],subscriptionId=42)
    async with await pro.subscribe(subscription,endpoints=urls) as stream:
        events = [await anext(stream), await anext(stream)]
        assert sum(v.get("price")==9007199254740993 for v in events)==1
        binary = next(v for v in events if "binary" in v)
        assert binary["parsed"]["price"]=="1.234567890123456789" and base64.b64decode(binary["evm"]["data"])[4:]==b"abc"
        try:
            await anext(stream)
        except sdk.TransportError:
            pass
        else:
            raise AssertionError("loss of all Pyth endpoints must surface")
    async with await pro.subscribe(subscription,endpoints=[urls[0][:-1]+"reconnect-python"]) as stream:
        assert (await anext(stream))["price"] == 9007199254740993
        assert (await anext(stream))["price"] == 10
    async with await pro.stream_merkle_roots(endpoints=urls[:1]) as stream:
        assert await anext(stream)=={"root":"abc"}
    hosted = sdk.TardisClient(origin=origin,api_key="fixture")
    feed = dict(exchange="bit/mex", **{"from":"2024-02-29T23:59:00Z"}, filters=[{"channel":"trade","symbols":["BTC/USD"]}])
    async with await hosted.data_feeds(feed) as stream:
        assert (await anext(stream))["message"]["n"] == 9007199254740993
    async with await hosted.replay({**feed,"to":"2024-03-01T00:02:00+00:00"}) as stream:
        assert await stream.read_batch(4) == [{"localTimestamp":"1700","message":{"n":9007199254740993}},{"type":"disconnect"},{"localTimestamp":"1702","message":{"n":3}}]
    dataset = dict(exchange="bit/mex",data_type="trades",date="2024-02-29",symbol="BTC/USD",range="bytes=0-3")
    async with await hosted.dataset(dataset) as stream:
        assert b"".join([chunk async for chunk in stream]) == bytes.fromhex("1f8b0800")
    import tempfile
    with tempfile.TemporaryDirectory() as folder:
        dest = Path(folder)/"dataset.gz"
        assert await hosted.download_dataset(dataset,dest) == 4 and dest.read_bytes() == bytes.fromhex("1f8b0800")
    machine = sdk.TardisMachineClient(origin=origin+"/machine")
    options = {"exchange":"bit/mex","symbols":["BTC/USD"]}
    for method in (machine.replay,machine.replay_normalized):
        async with await method(options) as stream:
            assert await stream.read_batch(8) == [{"sequence":9007199254740993},{"type":"disconnect"}]
    for method,params in ((machine.ws_replay,{"exchange":"bit/mex","from":"a","to":"b","subscribe_messages":[{"subscribe":1}]}),
                          (machine.ws_replay_normalized,options),(machine.stream_normalized,options)):
        async with await method(params) as stream:
            await stream.send({"ping":True})
            assert await stream.read_batch(2) == [{"seq":9007199254740993}]
    addr = "0x" + "21" * 20
    for row in fixture("polymarket-ws"):
        service = row["service"]
        client = sdk.PolymarketPerpsClient(perps=dict(proxy=addr, secret="fixture-secret")) if service == "perps" else sdk.PolymarketClient(
            clob=dict(address=addr, key="fixture",secret="c2VjcmV0",passphrase="fixture-pass"))
        url = origin.replace("http:", "ws:") + "/poly-ws/" + service
        async with await client.subscribe({"service":service, "params":row["params"]}, endpoint=url) as stream:
            assert await anext(stream) == row["event"]
            assert await anext(stream, None) is None
    options=dict(client_id="login", address="0x"+"21"*20, seed=bytes(range(32)).hex(), chain_id=1329)
    signer_messages=[]
    async def sign(message):
        signer_messages.append(message)
        return "0xabcd"
    wallet_client=sdk.MonacoClient(origin=origin)
    for client_id in ("unbound","wallet-mismatch"):
        try:
            await wallet_client.login({**options,"client_id":client_id},sign)
        except sdk.AuthenticationError:
            pass
        else:
            raise AssertionError("Monaco accepted a mismatched wallet or session key")
    assert len(signer_messages)==1
    wallet = sdk.MonacoWalletSigner(fixture("evm-signing")[0]["key"])
    named = sdk.MonacoClient(origin=origin, chain_id=1329)
    session_key = sdk.MonacoSessionSigner(options["seed"])
    async def remote_wallet(message):
        return await wallet.sign_message(message)
    info = await named.login(sdk.MonacoWalletSigner.remote(wallet.address, remote_wallet), "named-login", session_signer=session_key)
    assert info == {"address":wallet.address, "clientId":"named-login", "sessionPublicKey":session_key.public_key,
                    "chainId":1329, "expiresAt":4102444800}
    assert named.authenticated and named.session_signer.public_key == session_key.public_key
    recovered = sdk.MonacoClient(origin=origin, chain_id=1329)
    try:
        recovered.restore_session(sdk.MonacoSessionSigner("ff"*32),info)
    except ValueError:
        pass
    else:
        raise AssertionError("another Monaco session key accepted")
    assert not recovered.authenticated
    recovered.restore_session(session_key,info,request_signing="rfc9421")
    assert recovered.session_info == info and (await recovered.refresh_session())["expiresAt"] == 4102444801
    assert recovered.session_info["expiresAt"] == 4102444801
    logged=await wallet_client.login(options,sign)
    assert (await logged.refresh_session())["expiresAt"]==4102444801
    await logged.revoke_session()
    try:
        await logged.balances({})
    except sdk.AuthenticationError:
        pass
    else:
        raise AssertionError("revoked session accepted a read")
    for row in fixture("monaco-ws"):
        url=origin.replace("http:","ws:")+"/monaco-ws/"+row["params"]["type"]
        client=sdk.MonacoClient(origin=origin,session=dict(origin=origin,seed=options["seed"],expires_at=4102444800,ws_origin=url))
        async with await client.subscribe(row["params"],endpoint=url) as sub:
            first=await anext(sub)
            assert first["channel"]==row["channel"].replace("*","ETH")
            assert first["data"]["book_seq"]==9007199254740993
            if row["params"]["type"]=="depth":
                try: await anext(sub)
                except sdk.StreamGapError: pass
                else: raise AssertionError("depth sequence gap required")
            if row["params"]["type"]=="positions":
                try: await anext(sub)
                except sdk.StreamGapError: pass
                else: raise AssertionError("lost baseline must surface")
                assert (await anext(sub))["code"]=="SNAPSHOT_UNAVAILABLE"
                assert (await anext(sub))["type"]=="Event"
    for scenario in ("hold","reject","expire"):
        url=origin.replace("http:","ws:")+"/monaco-ws/"+scenario
        deadline=int(time.time())+3 if scenario=="expire" else 4102444800
        client=sdk.MonacoClient(origin=origin,session=dict(origin=origin,seed=options["seed"],expires_at=deadline,ws_origin=url))
        async with await client.subscribe({"type":"balances"}) as sub:
            if scenario=="reject":
                try: await anext(sub)
                except sdk.AuthenticationError: pass
                else: raise AssertionError("failed authentication reached consumer")
            else:
                assert (await anext(sub))["type"]=="Snapshot"
                pending=asyncio.create_task(anext(sub))
                if scenario=="hold": await client.revoke_session()
                try: await asyncio.wait_for(pending,5)
                except sdk.AuthenticationError: pass
                else: raise AssertionError("private read survived expiry or revocation")
        try: await client.balances({})
        except sdk.AuthenticationError: pass
        else: raise AssertionError("invalid session reused on HTTP")
    for mode in ("legacy","rfc9421"):
        session=dict(origin=origin, seed=bytes(range(32)).hex(), expires_at=4102444800, signing=mode)
        client=sdk.MonacoClient(origin=origin,session=session)
        for r in fixture("data-http"):
            if r["venue"]=="monaco" and (r["private"] or r["verb"]=="POST"):
                assert await getattr(client,r["operation"])(r["request"])==r["response"]
    # The catalog fixture above includes revocation. Use a fresh session for order failures.
    client = sdk.MonacoClient(origin=origin, session=dict(origin=origin, seed=options["seed"], expires_at=4102444800))
    request = next(r["request"] for r in fixture("data-http") if r["venue"] == "monaco" and r["operation"] == "place_order")
    for extra, error in (({"postonly": True}, sdk.ValidationError),
                         ({"clientOrderId": "lost-ack"}, sdk.OutcomeUncertainError)):
        try:
            await client.place_order({**request, "body": {**request["body"], **extra}})
        except error:
            pass
        else:
            raise AssertionError(f"expected {error.__name__} for {extra}")
    trade=await sdk.QfexClient(api_key="fixture",api_secret="secret").trading_session({"timeout_ms":180},endpoint=origin.replace("http:","ws:")+"/trade")
    async with trade:
        await trade.subscribe_account()
        order=dict(symbol="BTC", side="BUY", order_type="LIMIT", order_time_in_force="GTC", quantity=Decimal("0.5"),price=123,client_order_id="cid")
        assert (await trade.add_order(order))["order_id"]=="abc"
        assert (await anext(trade))["total"]==9007199254740993
        for op,data in ((trade.cancel_order,dict(symbol="BTC",order_id="abc",cancel_order_id_type="order_id")),(trade.add_order,order)):
            try:
                await op(data)
            except sdk.OutcomeUncertainError:
                pass
            else:
                raise AssertionError("late QFEX acknowledgement requires reconciliation")
        assert (await anext(trade))["order_response"]["status"]=="canceled"
        assert await trade.user_orders({"offset":0,"limit":100})=={"orders":[],"twaps":[]}

    try:
        await sdk.HistoricalClient(origin=origin).list_jobs({})
    except sdk.AuthenticationError as error:
        assert isinstance(error, sdk.ConnectorError)
    else:
        raise AssertionError("missing credential must fail before dispatch")
    try:
        sdk.decode_response("extended", 429, "{}")
    except sdk.ExchangeError as error:
        assert isinstance(error, ValueError)
    else:
        raise AssertionError("HTTP failure requires ExchangeError")
    key = fixture("auth")["databento"]["key"]
    hist = sdk.HistoricalClient(origin=origin, api_key=key)
    params = {"symbols":["ETH","BTC"],"encoding":"csv","compression":"zstd"}
    from tempfile import TemporaryDirectory
    batch = origin + "/files/batch?sig=a%2Fb"
    expected = (HERE / "reference-lines.zst").read_bytes()
    with TemporaryDirectory() as directory:
        dest = Path(directory) / "batch.dbn.zst"
        assert await hist.download_to(batch, dest) == len(expected)
        assert dest.read_bytes() == expected
        for url, path in ((batch, dest), (batch.replace("/batch", "/partial"), dest.with_suffix(".partial"))):
            try:
                await hist.download_to(url, path)
            except ValueError:
                pass
            else:
                raise AssertionError("incomplete or conflicting download succeeded")
        assert list(Path(directory).iterdir()) == [dest] and dest.read_bytes() == expected
    external = batch.replace("127.0.0.1", "localhost").replace("/batch", "/storage")
    stream = await hist.download(external)
    assert b"".join([chunk async for chunk in stream]) == expected
    for url, client in ((batch, sdk.HistoricalClient(api_key=key)), (batch+"#frag",hist),
                        (batch.replace("://", "://someone@"),hist), (batch.replace("/batch","/redirect"),hist)):
        try:
            await client.download(url)
        except ValueError:
            pass
        else:
            raise AssertionError("disallowed download URL")
    stream = await hist.timeseries(**params)
    assert b"".join([chunk async for chunk in stream]) == (HERE/"reference-lines.zst").read_bytes()
    stream = await hist.get_range(params)
    meta, *records = [event async for event in stream]
    golden = [loads(line) for line in (HERE/"legacy-v1.jsonl").read_text().splitlines()]
    assert [meta["metadata"],*records] == golden

    ref = sdk.ReferenceClient(origin=origin,api_key=key)
    for row in fixture("databento-streams"):
        if row["client"] != "ReferenceClient":
            continue
        for compression in ("none","zstd"):
            stream = await getattr(ref,row["operation"])({"symbols":["ETH","BTC"],"compression":compression})
            assert await stream.read_batch(4) == [{"id":9007199254740993,"price":Decimal("1.25000000000000000001")},{"id":2}]
            assert await anext(stream,None) is None
    stream = await ref.security_master_get_last({"symbols":["ETH","BTC"],"schema":"invalid"})
    assert len(await stream.read_batch(5)) == 2
    try:
        await anext(stream)
    except ValueError:
        pass
    else:
        raise AssertionError("partial batch lost its trailing decode error")
    clients = {"dune": sdk.DuneClient, "pyth_hermes": sdk.PythHermesClient, "pyth_pro": sdk.PythProClient}
    for row in fixture("text-http"):
        client = clients[row["venue"]](origin=origin, api_key="test-key")
        assert await getattr(client, row["operation"])(request=row["request"]) == row["expected"]
    client = sdk.ChainlinkCandlestickClient(origin=origin, api_key="test-key")
    stream = await client.stream_prices(request={"symbol":["ETHUSD","BTCUSD"]})
    assert [event async for event in stream] == [
        {"heartbeat":1748525528}, {"p":Decimal("1.0000000000000000001e+21"), "f":'"}{', "items":[{},[]]}]
    for symbol in ("TRUNCATED", "INVALID"):
        stream = await client.stream_prices({"symbol":[symbol]})
        try:
            await anext(stream)
        except ValueError:
            continue
        raise AssertionError(symbol)


def test_evm_send():
    row = fixture("evm-signing")[0]
    raw, tx_hash, address = row["signed"], row["hash"], row["address"]
    class Handler(BaseHTTPRequestHandler):
        def do_POST(self):
            wire = loads(self.rfile.read(int(self.headers["Content-Length"])))
            method, params = wire["method"], wire["params"]
            if method == "eth_chainId":
                result = "0x2" if self.path == "/write" and self.server.wrong_chain else "0x1"
            elif method == "eth_sendRawTransaction":
                assert self.path == "/write" and params == [self.server.expected_raw]
                self.server.attempts += 1
                if self.server.drop:
                    return
                result = self.server.expected_hash
            else:
                assert self.path == "/read", method
                if method == "eth_getCode":
                    assert params == [row["prepared"]["to"],"latest"]
                    result = "0x6000"
                elif method == "eth_getTransactionCount":
                    assert params == [address,"pending"]
                    result = hex(self.server.nonce)
                elif method == "eth_getTransactionByHash":
                    assert params == [tx_hash]
                    result = self.server.observation
                else:
                    raise AssertionError(method)
            self.send_response(200)
            self.end_headers()
            self.wfile.write(dump({"id":wire["id"],"jsonrpc":"2.0","result":result}).encode())

        def log_message(self, *_):
            pass

    async def run(server):
        origin = f"http://127.0.0.1:{server.server_port}"
        def new(**kw):
            return sdk.EvmChainClient(1, origin+"/read", origin+"/write", **kw)
        client, other = new(),new()
        async def rejected(task):
            try:
                await task
            except ValueError:
                return
            raise AssertionError("invalid or uncertain transaction accepted")
        server.wrong_chain = True
        await rejected(client.submit_raw(raw))
        server.wrong_chain = False
        for malformed in ("0x", raw+"00", fixture("evm-signing")[2]["signed"]):
            await rejected(client.submit_raw(malformed))
        await rejected(new(code_hashes={row["prepared"]["to"]:"0x"+"00"*32}).send_transaction(row["prepared"], row["key"]))
        assert server.attempts == 0
        assert await client.submit_raw(raw) == tx_hash
        server.drop = True
        await rejected(client.send_transaction({**row["prepared"],"nonce":123}, row["key"]))
        assert other.pending(address) == {"hash":tx_hash,"nonce":3}
        await rejected(other.send_transaction(row["prepared"], row["key"]))
        assert server.attempts == 2 and not await other.reconcile(address)
        server.observation = {"hash":tx_hash,"from":"0x"+"00"*20}
        await rejected(other.reconcile(address))
        await rejected(other.resync_nonce(address,"0x"+"ff"*32))
        assert await other.resync_nonce(address,tx_hash) == 3
        await rejected(client.send_transaction(row["prepared"],row["key"]))
        server.observation = {"hash":tx_hash,"from":address}
        assert await other.reconcile(address) and client.pending(address) is None
        assert server.attempts == 3
        local = sdk.MetricSigner(row["key"])
        assert local.address == address
        async def sign(payload):
            assert payload["scheme"] == "evm_digest" and payload["address"] == address
            return await local.sign_digest(payload["digest"])
        remote = sdk.MetricSigner.remote(address,sign)
        chain = sdk.MetricChainClient(chain_id=1, read_rpc=origin+"/read", write_rpc=origin+"/write", signer=remote)
        signed = await remote.sign_transaction(row["prepared"])
        assert signed == {"bytes":raw,"hash":tx_hash,"signer":address,"nonce":3,"chain_id":1}
        server.drop = False
        async def stale(payload):
            server.nonce = 5
            return await sign(payload)
        changed = sdk.MetricChainClient(1,origin+"/read",origin+"/write",signer=sdk.MetricSigner.remote(address,stale))
        await rejected(changed.send_transaction(request=row["prepared"]))
        assert server.attempts == 3 and chain.pending(address) is None
        # The successful reconciliation already advanced the coordinator to nonce 4.
        # cast mktx / cast keccak, with the original fixture and that nonce.
        server.nonce = 4
        server.expected_raw = "0x02f86f0104843b9aca008504a817c800830249f09411111111111111111111111111111111111111110783123456c080a0fdee5934fcfb07922928cfcc72c573db602212f6c4d55924d300073704a0e624a0530d33091cd7e71dc5e8ec5a2c285bdf72863cb1026518b1961b73a0fefba92b"
        server.expected_hash = "0x8c91d2758f90357e521cb80f5e3a1bb8101ce35b6c152d9e639c983e1203e7fd"
        assert await chain.send_transaction(request=row["prepared"]) == server.expected_hash
        assert server.attempts == 4

    with ThreadingHTTPServer(("127.0.0.1",0),Handler) as server:
        server.expected_raw, server.expected_hash = raw, tx_hash
        server.attempts,server.drop,server.observation,server.nonce = 0,False,None,3
        worker=threading.Thread(target=server.serve_forever,daemon=True)
        worker.start()
        try:
            asyncio.run(run(server))
        finally:
            server.shutdown()
            worker.join()


def test_solana_chain():
    f = fixture("pascal-solana")
    token = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"

    class Handler(BaseHTTPRequestHandler):
        def do_POST(self):
            wire = loads(self.rfile.read(int(self.headers["Content-Length"])))
            method, params = wire["method"], wire["params"]
            if method == "getAccountInfo":
                address, opts = params
                assert opts == {"commitment":"confirmed","encoding":"jsonParsed"}
                info = ({"decimals":6} if address == f["mint"] else
                        {"mint":f["mint"],"owner":f["owner"] if not self.server.mismatch else f["source"]})
                result = {"value":{"owner":token,"data":{"parsed":{"info":info}}}}
            elif method == "getLatestBlockhash":
                result = {"value":{"blockhash":f["blockhash"], "lastValidBlockHeight":100}}
            elif method == "getBlockHeight":
                result = self.server.height
            elif method == "sendTransaction":
                assert params == [f["transaction_base64"],{"encoding":"base64","skipPreflight":False,"preflightCommitment":"confirmed","maxRetries":0}]
                self.server.attempts += 1
                if self.server.drop:
                    return
                result = f["signature"]
            elif method == "getSignatureStatuses":
                assert params == [[f["signature"]],{"searchTransactionHistory":True}]
                result = {"value":[{"err":None,"confirmationStatus":self.server.level}]}
            elif method == "getTokenAccountBalance":
                assert params == [f["source"],{"commitment":"confirmed"}]
                result = {"value":{"amount":"25000000"}}
            else:
                raise AssertionError(method)
            self.send_response(200)
            self.end_headers()
            self.wfile.write(dump({"jsonrpc":"2.0","id":wire["id"],"result":result}).encode())

        def log_message(self, *_):
            pass

    async def run(server):
        c = sdk.PascalChainClient(f"http://127.0.0.1:{server.server_port}",f["mint"],6)
        assert await c.block_height() == 90
        assert (await c.token_balance(f["source"]))["value"]["amount"] == "25000000"
        deposit = lambda: c.prepare_deposit(f["owner"],f["source"],f["destination"],f["amount"])
        async def rejected(task):
            try:
                await task
            except ValueError:
                return
            raise AssertionError("invalid or pending deposit was accepted")
        server.mismatch = True
        await rejected(deposit())
        server.mismatch = False
        prepared = await deposit()
        signed = sdk.solana.sign_deposit("07"*32,prepared)
        assert signed["signature"] == f["signature"]
        local = sdk.PascalSigner("07"*32)
        assert local.public_key == f["owner"]
        called = []
        async def sign_pascal(proof):
            assert proof["scheme"] == "pascal_ed25519" and proof["public_key"] == local.public_key
            called.append(proof["message_hex"])
            assert bytes.fromhex(proof["message_hex"]) == base64.b64decode(proof["message_base64"])
            return await local._message(bytes.fromhex(proof["message_hex"]))
        configured = sdk.PascalChainClient(rpc_url=c._config["endpoint"],mint=f["mint"],signer=sdk.PascalSigner.remote(local.public_key,sign_pascal))
        assert configured.authenticated and await configured.sign_deposit(prepared) == signed
        assert configured.associated_token_address(f["owner"],f["mint"]) == f["associated_token_address"]
        await rejected(configured.sign_deposit({**prepared,"amount":1}))
        assert len(called) == 1
        server.height = 101
        await rejected(c.submit(signed))
        assert c.pending == [] and server.attempts == 0
        server.height = 90
        await rejected(c.submit({**signed,"signature":"1"*64}))
        server.drop = True
        await rejected(c.submit(signed))
        await rejected(c.submit(signed))
        await rejected(deposit())
        assert c.pending == [f["signature"]] and server.attempts == 1
        await c.reconcile(f["signature"])
        assert c.pending == [f["signature"]]
        server.level = "confirmed"
        await c.reconcile(f["signature"])
        assert c.pending == []
        server.drop = False
        assert await c.submit(signed) == f["signature"]
        c.clear_reconciled_submission(f["signature"])
        assert c.pending == [] and server.attempts == 2
        class History:
            async def transfer_history(self, request):
                assert request == {"owner":f["owner"]}
                return ["matched"]
        credit = await c.reconcile_deposit_credit(History(), owner=f["owner"], signature=f["signature"])
        assert credit["signature"] == f["signature"] and credit["transfers"] == ["matched"]

    with ThreadingHTTPServer(("127.0.0.1",0), Handler) as server:
        server.attempts, server.height, server.drop, server.level = 0, 90, False, "processed"
        worker = threading.Thread(target=server.serve_forever,daemon=True)
        worker.start()
        try:
            asyncio.run(run(server))
        finally:
            server.shutdown()
            worker.join()


def test_trading_signatures():
    f = fixture("paradex-signing")
    for row in f["vectors"]:
        for key, field, expected in (("stark_key", "key", "signature"), ("evm_key", "evm_key", "evm_signature")):
            client = sdk.ParadexClient(bearer={key:f[field], "account":f["account"], "network":row["network"]})
            assert client.sign_typed_data(row["typed_data"]) == {"digest": row["digest"], "signature":row[expected]}, row["name"]
    for row in fixture("paradex-admin-hashes")["vectors"]:
        client = sdk.ParadexClient(bearer={"stark_key":f["key"], "account":row["account"], "network":row["typed_data"]["domain"]["chainId"]})
        assert client.sign_typed_data(row["typed_data"])["digest"] == row["digest"]
    for row in fixture("pascal-permits"):
        local = sdk.PascalSigner(row["auth"]["seed"])
        async def sign(proof):
            return await local._message(base64.b64decode(proof["message_base64"]))
        remote = sdk.PascalSigner.remote(local.public_key,sign)
        args = (row["auth"]["owner"],3,row["operation"],row["body"])
        signed = asyncio.run(remote.sign_request(*args,client_ts_ms=1731536000000))
        assert signed["auth"]["signature"] == row["signature"]
        result = sdk.PascalClient(permit=row["auth"]).sign_permit_at(row["operation"], row["body"], 1731536000000)
        assert result == {**row["body"], "auth":{"owner":row["auth"]["owner"], "signer":row["signer"],
            "signature":row["signature"], "recv_window_ms":"5000", "client_ts_ms":"1731536000000"}}


def test_derive():
    from typing import get_type_hints
    from aux_py import derive_models as models

    assert get_type_hints(sdk.DeriveClient.private_order)["return"] is models.OrderCreatedResponse
    assert "amount" in models.CreateOrderRequest.__required_keys__
    assert "signature" in models.CreateOrderRequest.__optional_keys__
    f = fixture("derive-signing")
    assert sdk.derive_login(f["key"], f["owner"], int(f["login"]["timestamp"])) == {"wallet": f["owner"], **f["login"]}
    for row in f["actions"]:
        for vector in row["vectors"]:
            action = {k: f[k] for k in ("owner", "expiry", "nonce", "subaccount_id")}
            action.update(data=row["data"], chain_id=vector["chain_id"])
            signed = sdk.derive_sign_action(f["key"], action)
            assert signed == {"signer": f["signer"].lower(), "encoded": row["encoded"], "module": row["module"], "digest": vector["digest"], "signature": vector["signature"]}
    action["data"] = f["actions"][0]["data"]
    for override in ({"data": {**action["data"], "amount": "0.0000000000001"}}, {"chain_id": 10}, {"nonce": "-1"}, {"owner": "0x00"}):
        try:
            sdk.derive_sign_action(f["key"], {**action, **override})
        except ValueError:
            continue
        raise AssertionError("invalid action accepted")


def test_http():
    for row in fixture("http") + fixture("prediction-http") + fixture("data-http") + fixture("perps-http"):
        venue, name = row["venue"], row["operation"]
        params = (
            loads(row["request_json"]) if row.get("request_json") else row["request"]
        )
        req = sdk.prepare_request(venue, name, params)
        if "route" in row:
            assert req["path"] == row["route"], (venue, name)
        else:
            url = urlsplit(req["path"])
            pairs = {}
            for key, val in parse_qsl(url.query):
                if isinstance(row["query"].get(key), list):
                    pairs.setdefault(key, []).append(val)
                else:
                    pairs[key] = val
            assert url.path == row["path"] and pairs == row["query"], (venue, name)
        body = loads(row["body_json"]) if row.get("body_json") else row.get("body")
        assert (req["body"], req["authenticated"], req["method"]) == (
            body,
            row["private"],
            row["verb"],
        ), (venue, name)
        for key, value in row.get("headers", {}).items():
            assert req["headers"][key] == value
        text = row.get("response_json") or dump(row["response"])
        if row.get("status", 200) >= 300:
            try:
                sdk.decode_response(venue, row["status"], text)
            except ValueError:
                continue
            raise AssertionError((venue, name, "expected an exchange error"))
        expected = loads(text)
        if venue == "derive":
            text = '{"result":' + text + "}"
        assert sdk.decode_response(venue, 200, text) == expected


def compare(actual, expected):
    if isinstance(actual, list):
        assert len(actual) == len(expected)
        for a, e in zip(actual, expected):
            compare(a, e)
    elif isinstance(actual, int) and isinstance(expected, str):
        assert str(actual) == expected
    else:
        assert actual == expected


def test_protocols():
    decoder = sdk.EventDecoder(64)
    assert decoder.feed(b"data: one\r") == []
    assert decoder.feed(b"\n\r\ndata: two\n\n") == ["one", "two"]
    decoder.finish()
    decoder.close()
    decoder.close()
    for row in fixture("polymarket-helpers"):
        assert getattr(sdk.polymarket, row["operation"])(**row["request"]) == row["expected"]
    for row in fixture("metric-helpers"):
        op = row["operation"]
        actual = sdk.metric.codebook() if op == "codebook" else getattr(sdk.metric, op)(row["request"])
        assert actual == row["expected"], op
    for row in fixture("metric-abi"):
        c, sig = row["contract"], row["signature"]
        if row["kind"] == "function":
            assert sdk.metric.encode_contract_call(c, sig, row["args"]) == row["input"]
            compare(sdk.metric.decode_contract_input(c, sig, row["input"]), row["args"])
            compare(
                sdk.metric.decode_contract_output(c, sig, row["output"]),
                row["outputArgs"],
            )
        elif row["kind"] == "event":
            event = sdk.metric.decode_contract_event(c, sig, row["topics"], row["data"])
            compare(event["indexed"], row["indexed"])
            compare(event["body"], row["body"])
        else:
            compare(
                sdk.metric.decode_contract_error(c, row["input"])["arguments"],
                row["args"],
            )
    for row in fixture("polymarket-abi"):
        assert (
            sdk.polymarket.encode_contract_call(
                row["interface"], row["function"], row["args"]
            )
            == row["calldata"]
        )
        compare(
            sdk.polymarket.decode_contract_input(
                row["interface"], row["function"], row["calldata"]
            ),
            row["args"],
        )
    for row in fixture("chainlink-reports"):
        report = sdk.decode_report(row["full_report"])
        assert (
            report["report_body"] == row["report_body"]
            and report["version"] == row["version"]
        )
        if report["kind"] == "known":
            assert report["fields"][row["key_field"]] == row["key_value"]
    for row in fixture("evm-signing"):
        assert sdk.sign_transaction(row["key"], row["prepared"]) == {
            k: row[k] for k in ("hash", "signed", "address")
        }
    for f in fixture("paradex-sbe")["frames"]:
        assert sdk.decode_sbe(bytes.fromhex(f["hex"]))["data"] == f["native"]
    for version in (1, 2):
        prefix = HERE / f"legacy-v{version}"
        metadata, *records = sdk.decode_dbn_file(prefix.with_suffix(".dbn").read_bytes())
        expected = [loads(line) for line in prefix.with_suffix(".jsonl").read_text().splitlines()]
        assert metadata["metadata"] == expected[0] and records == expected[1:]
    for name in ("schema-records.dbn","schema-records.dbn.zst"):
        metadata, *records = sdk.decode_dbn_file((HERE / name).read_bytes())
        assert metadata["metadata"]["dataset"] == "GLBX.MDP3" and metadata["type"] == "metadata"
        assert records == [row["record"] for row in fixture("schema-records")]
    data = (HERE / "schema-records.dbn").read_bytes()
    offset = 8 + int.from_bytes(data[4:8], "little")
    for row in fixture("schema-records"):
        size = 4 * data[offset]
        assert sdk.decode_dbn_record(data[offset : offset + size]) == row["record"]
        offset += size
    assert offset == len(data)
    data = (HERE / "ts-out.dbn").read_bytes()
    offset = 8 + int.from_bytes(data[4:8], "little")
    assert (
        sdk.decode_dbn_record(data[offset:], ts_out=True)["ts_out"]
        == "1704067200000000999"
    )
    auth = fixture("auth")
    f = auth["kalshi"]
    assert (
        sdk.kalshi_headers(f["key"], f["seed"], f["method"], f["path"], f["timestamp"])[
            "KALSHI-ACCESS-SIGNATURE"
        ]
        == f["signature"]
    )
    f = auth["qfex"]
    assert sdk.qfex_signature(f["secret"], f["nonce"], f["timestamp"]) == f["hmac"]
    f = auth["databento"]
    assert sdk.databento_cram(f["key"], f["challenge"]) == f["response"]
    f = auth["arcus"]
    assert sdk.arcus_sign(f["seed"], f["payload"]) == {
        k: f[k] for k in ("signature", "public_key")
    }
    for f in auth["monaco"]:
        headers = sdk.monaco_sign(
            f["privateKey"], f["method"], f["target"], f["body"], f["created"]
        )
        assert headers["Signature"] == f["signature"]


async def test_socket(venue="qfex"):
    peer = await asyncio.create_subprocess_exec(
        sys.executable, HERE / "ws_server.py", venue, stdout=asyncio.subprocess.PIPE
    )
    try:
        endpoint = (await peer.stdout.readline()).decode().strip()
        if venue == "extended":
            for name, row in sorted(fixture("extended-ws").items()):
                client = sdk.ExtendedClient(api_key="fixture" if row["private"] else None)
                base = endpoint.removesuffix("/stream") + "/v1"
                stream = await client.subscribe(row["specification"], endpoint=base)
                assert [event async for event in stream] == [row["frame"]] * (2 if name == "orderbooks_rpc" else 1)
                await stream.close()
            assert await peer.wait() == 0
            return
        if venue in ("pascal","kalshi"):
            kalshi = venue == "kalshi"
            client = sdk.KalshiClient(api_key="fixture",api_secret="07"*32) if kalshi else sdk.PascalClient()
            stream = await client.subscribe({"service":"predictions" if kalshi else "read",
                "params":{"channel":"orderbook_delta" if kalshi else "book"}},endpoint=endpoint)
            assert [(await anext(stream))["seq"] for _ in range(3)] == [1,2,3]
            assert await anext(stream,None) is None
            await stream.close()
            assert await peer.wait() == 0
            return
        if venue == "paradex":
            client = sdk.ParadexClient(bearer={"token":"test.eyJleHAiOjQxMDI0NDQ4MDB9.test","account":"0x001234"})
            frames = {r["template"]: r["native"] for r in fixture("paradex-sbe")["frames"]}
            for name, row in sorted(fixture("paradex-ws").items()):
                stream = await client.subscribe(row["specification"], endpoint=endpoint)
                event = await asyncio.wait_for(anext(stream),5)
                assert event["params"] == {"channel":row["channel"],"data":frames.get(row["template"],row.get("payload"))}
                if name == "account":
                    assert (await anext(stream))["method"] == "heartbeat"
                    assert (await anext(stream))["method"] == "subscribed"
                assert await anext(stream,None) is None
                await stream.close()
                await stream.close()
            assert await peer.wait() == 0
            return
        if venue == "derive-reject":
            f = fixture("derive-signing")
            client = sdk.DeriveClient(api_key=f["owner"], api_secret=f["key"])
            stream = await client.subscribe({"type": "orders", "subaccount_id": 9}, endpoint=endpoint)
            try:
                await asyncio.wait_for(anext(stream), 2)
            except ValueError as err:
                assert "auth" in str(err).lower()
            else:
                raise AssertionError("rejected login exposed a subscription")
            await stream.close()
            assert await peer.wait() == 0
            return
        if venue == "derive":
            f = fixture("derive-signing")
            client = sdk.DeriveClient(api_key=f["owner"], api_secret=f["key"])
            for _, row in sorted(fixture("derive-ws").items()):
                stream = await client.subscribe(row["specification"], endpoint=endpoint)
                event = await anext(stream)
                assert event["params"] == {"channel": row["channel"], "data": row["payload"]}
                assert await anext(stream, None) is None
                await stream.close()
            assert await peer.wait() == 0
            return
        if venue == "phoenix":
            client = sdk.PhoenixClient(api_key="fixture-key")
            for name, row in sorted(fixture("phoenix-ws").items()):
                stream = await client.subscribe(row["specification"], endpoint=endpoint)
                assert await anext(stream) == row["frame"]
                if name == "exchange":
                    assert (await anext(stream))["decodedPayload"]["data"] == [1, 2, 3]
                assert await anext(stream, None) is None
                await stream.close()
            assert await peer.wait() == 0
            return
        stream = await sdk.QfexClient().subscribe(
            {"channel": "trade", "symbol": "BTC"}, endpoint=endpoint
        )
        assert (await anext(stream))["seq"] == 9999999999999999999
        assert (await anext(stream))["seq"] == 2
        assert await anext(stream, None) is None
        await stream.close()
        await stream.close()
        assert await peer.wait() == 0
    finally:
        if peer.returncode is None:
            peer.kill()
            await peer.wait()


def test_clob():
    credentials = {"address": "0x" + "22" * 20, "key": "test-key", "passphrase": "test-pass", "secret": "cHVibGljLXRlc3Qtc2VjcmV0"}
    signed_ops = set((HERE.parent / "src/exchanges/polymarket/l2-routes.txt").read_text().splitlines())
    rows = [row for row in fixture("prediction-http") if row["operation"] in signed_ops]
    assert len(rows) == len(signed_ops)

    class Handler(BaseHTTPRequestHandler):
        def sign(self):
            row = self.server.row
            assert self.command == row["verb"] and self.path == "/proxy" + row["route"]
            body = self.rfile.read(int(self.headers.get("Content-Length", 0)))
            if row["body"] is not None:
                assert loads(body) == row["body"]
            for header, k in [("POLY_API_KEY","key"),("POLY_PASSPHRASE","passphrase"),("POLY_ADDRESS","address")]:
                assert self.headers[header] == credentials[k] and len(self.headers.get_all(header)) == 1
            path = self.path.split("?")[0]
            message = (self.headers["POLY_TIMESTAMP"] + self.command + path).encode() + body
            signature = base64.urlsafe_b64encode(hmac.digest(b"public-test-secret",message,"sha256")).decode()
            assert signature == self.headers["POLY_SIGNATURE"]
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b'{"ok":true}')
        do_GET = do_POST = do_PUT = do_DELETE = sign

        def log_message(self, *_):
            pass

    with ThreadingHTTPServer(("127.0.0.1", 0), Handler) as server:
        worker = threading.Thread(target=server.serve_forever,daemon=True)
        worker.start()
        try:
            client = sdk.PolymarketClient(origin=f"http://127.0.0.1:{server.server_port}/proxy",clob=credentials)
            for row in rows:
                server.row = row
                name = row["operation"].split(".")[1]
                assert asyncio.run(getattr(client,name)(row["request"])) == {"ok":True}
        finally:
            server.shutdown()
            worker.join()


def test_paradex_and_perps():
    routes = HERE.parent / "src"
    paradex = dict(line.split() for line in (routes / "exchanges/paradex/bearer-routes.txt").read_text().splitlines())
    perps = dict(line.split() for line in (routes / "exchanges/polymarket/perps-routes.txt").read_text().splitlines())
    golden = fixture("polymarket-perps-signing")
    token = "fixture." + base64.urlsafe_b64encode(b'{"exp":4102444800}').decode().rstrip("=") + ".fixture"
    credentials = dict(owner_key=golden["key"], proxy_key=golden["key"], proxy=golden["proxy"],
                       secret="fixture", withdrawal_contract=golden["withdrawal_contract"])

    class Handler(BaseHTTPRequestHandler):
        def check(self):
            row, body, headers = self.server.expected
            assert self.command == row["verb"] and self.path == row["route"], row["operation"]
            raw = self.rfile.read(int(self.headers.get("Content-Length", 0)))
            received = loads(raw) if raw else None
            if "check_signed_block" in row:
                row["check_signed_block"](received)
            elif row.get("pascal"):
                vector = row["pascal"]
                item = received[0] if isinstance(received, list) else received
                signed = sdk.PascalClient(permit=vector["auth"]).sign_permit_at(
                    row["operation"], vector["body"], int(item["auth"]["client_ts_ms"]))
                assert received == ([signed] if isinstance(body, list) else signed)
            else:
                assert received == body, row["operation"]
            for name in ("Authorization", "POLYMARKET-PROXY", "POLYMARKET-SECRET", "POLY_API_KEY", "POLY_BUILDER_API_KEY", "RELAYER_API_KEY", "RELAYER_API_KEY_ADDRESS"):
                assert (self.headers.get_all(name) or []) == ([headers[name]] if name in headers else [])
            if "POLY_API_KEY" in headers:
                message = (self.headers["POLY_TIMESTAMP"] + self.command + urlsplit(self.path).path).encode() + raw
                assert self.headers["POLY_SIGNATURE"] == base64.urlsafe_b64encode(hmac.digest(b"a",message,"sha256")).decode()
            if "POLY_BUILDER_API_KEY" in headers:
                message = (self.headers["POLY_BUILDER_TIMESTAMP"] + self.command + urlsplit(self.path).path).encode() + raw
                assert self.headers["POLY_BUILDER_SIGNATURE"] == base64.urlsafe_b64encode(hmac.digest(b"b",message,"sha256")).decode()
            self.send_response(200)
            self.end_headers()
            self.wfile.write(row.get("response_json", dump(row["response"])).encode())
        do_GET = do_POST = do_PATCH = do_PUT = do_DELETE = check

        def log_message(self, *_):
            pass

    async def run(server):
        origin = f"http://127.0.0.1:{server.server_port}"
        bearer = sdk.ParadexClient(origin=origin, bearer={"token":token, "account":"0x1234", "read_only":False})
        client = sdk.PolymarketPerpsClient(origin=origin, perps=credentials)
        for row in fixture("http") + fixture("prediction-http"):
            name = row["operation"]
            params = loads(row["request_json"]) if "request_json" in row else row["request"]
            body = loads(row["body_json"]) if "body_json" in row else row["body"]
            if row["venue"] == "paradex" and (name in paradex or name == "get_bbo"):
                if name == "block_trades_create":
                    params["body"]["signatures"] = body["signatures"] = {"0x1234":body["signatures"]["fixture"]}
                method = getattr(bearer, name)
                headers = {"Authorization":"Bearer " + token} if name in paradex else {}
            elif row["venue"] == "polymarket" and name in perps:
                method = getattr(client, name.split(".")[1])
                if perps[name] == "proxy":
                    headers = {"POLYMARKET-PROXY":golden["proxy"], "POLYMARKET-SECRET":"fixture"}
                else:
                    params["body"].pop("sig", None)
                    body = {**body, "sig":golden["signatures"][name]}
                    headers = {}
            else:
                continue
            server.expected = row, body, headers
            assert await method(request=params) == loads(row.get("response_json", dump(row["response"]))), name

        vectors = {row["operation"]:row for row in fixture("pascal-permits")}
        services = dict(line.split() for line in (routes / "exchanges/polymarket/service-routes.txt").read_text().splitlines())
        clob = {"address":golden["proxy"], "key":"api", "secret":"YQ==", "passphrase":"pass"}
        builder = {"key":"builder", "secret":"Yg==", "passphrase":"builder-pass"}
        for row in fixture("prediction-http"):
            name = row["operation"]
            if row["venue"] == "pascal" and name in vectors:
                vector = vectors[name]
                client = sdk.PascalClient(origin=origin, permit=vector["auth"])
                body = [vector["body"]] if isinstance(row["body"],list) else vector["body"]
                server.expected = {**row,"pascal":vector}, body, {}
                for configured in (client, remote_pascal(vector, origin=origin)):
                    await getattr(configured, name.split(".")[1])({"body":body})
            elif row["venue"] == "polymarket" and name in services:
                params = loads(row.get("request_json", dump(row["request"])))
                body = loads(row.get("body_json", dump(row["body"])))
                for relayer in ([False, True] if services[name] == "relayer" else [False]):
                    service = {"builder":builder}
                    headers = {}
                    if relayer:
                        service["relayer"] = {"key":"relay","address":golden["proxy"]}
                        headers = {"RELAYER_API_KEY":"relay", "RELAYER_API_KEY_ADDRESS":golden["proxy"]}
                    elif services[name] == "relayer":
                        headers["POLY_BUILDER_API_KEY"] = "builder"
                    else:
                        headers["POLY_API_KEY"] = "api"
                        if services[name] == "gateway" and row["verb"] != "GET":
                            headers["POLY_BUILDER_API_KEY"] = "builder"
                    client = sdk.PolymarketClient(origin=origin, services=service, clob=clob)
                    server.expected = row, body, headers
                    await getattr(client, name.split(".")[1])(params)

        f = fixture("paradex-signing")
        for vector in f["vectors"]:
            if not vector["inputs"]:
                continue
            client = sdk.ParadexClient(origin=origin, bearer={"token":token, "read_only":False, "account":f["account"],
                "network":vector["network"], "stark_key":f["key"]})
            body = vector["inputs"]
            expected = {**body, "signature":json.dumps(vector["signature"], separators=(",",":"))}
            if vector["name"] == "modify":
                order_id = vector["typed_data"]["message"]["id"]
                server.expected = {"verb":"PUT","route":"/v1/orders/"+order_id,"response":{}}, expected, {"Authorization":"Bearer "+token}
                await client.orders_modify({"order_id":order_id,"body":body})
            else:
                server.expected = {"verb":"POST","route":"/v1/orders","response":{}}, expected, {"Authorization":"Bearer "+token}
                await client.orders_new({"body":body})
                server.expected = {"verb":"POST","route":"/v1/orders/batch","response":[]}, [expected], {"Authorization":"Bearer "+token}
                await client.orders_new_batch({"body":[body]})

        # Exercise the actual unsigned block and offer entry points with both key types.
        def scaled(value):
            return format(Decimal(value).scaleb(-8), "f")

        for vector in f["vectors"]:
            if vector["name"] not in ("block", "offer"):
                continue
            typed = vector["typed_data"]
            message = typed["message"]
            is_offer = vector["name"] == "offer"
            body = {"nonce":message["nonce"], "trades":{}}
            if not is_offer:
                body["block_expiration"] = int(message["expiration"])
            for leaf in reversed(message["trades"]):
                trade = {key:scaled(leaf[key]) for key in ("price","size")}
                trade["trade_constraints"] = {key:scaled(leaf[key]) for key in ("min_size","max_size","min_price","max_price","oracle_tolerance")}
                for key in ("maker_order","taker_order"):
                    order = leaf[key]
                    if order["account"] != "0":
                        trade["offerer_order" if is_offer else key] = dict(account=order["account"], side={"1":"BUY","2":"SELL"}[order["side"]],
                            type=order["type"], size=scaled(order["size"]), price=scaled(order["price"]))
                body["trades"][leaf["market"]] = trade

            for key_name, secret, public, expected in (("evm_key", f["evm_key"], f["evm_address"].lower(), vector["evm_signature"]),
                                                     ("stark_key", f["key"], f["public_key"], vector["signature"])):
                client = sdk.ParadexClient(origin=origin, bearer={"token":token, "account":f["account"], "read_only":False, "network":vector["network"], key_name:secret})
                def check_signed_block(received):
                    signed = received.pop("signature") if is_offer else received.pop("signatures")[f["account"]]
                    assert received == body and signed["nonce"] == message["nonce"]
                    assert (signed["signer_public_key"], signed["signer_account"], signed["signature_type"]) == (public,f["account"],"STARKNET")
                    if is_offer:
                        expiry = signed["signature_expiration"]
                        assert expiry == signed["signature_timestamp"] + 300000
                        refreshed = {**typed, "message":{**message, "expiration":str(expiry)}}
                        proof = client.sign_typed_data(refreshed)["signature"]
                    else:
                        assert signed["signature_expiration"] == int(message["expiration"])
                        proof = expected
                    assert json.loads(signed["signature_data"]) == proof
                path = "/v1/block-trades" + ("/"+message["block_trade_id"]+"/offers" if is_offer else "")
                server.expected = {"verb":"POST","route":path,"operation":vector["name"],"response":{},"check_signed_block":check_signed_block}, body, {"Authorization":"Bearer "+token}
                if is_offer:
                    await client.block_offers_create({"block_trade_id":message["block_trade_id"],"body":body})
                else:
                    await client.block_trades_create({"body":body})

        # Compare both order-domain versions and deposit-wallet signatures before
        # checking the HMAC computed over exactly those signed order bytes.
        for vector in fixture("polymarket-clob-orders"):
            order = vector["order"]
            local = sdk.PolymarketSigner(golden["key"])
            async def sign(proof):
                return await local.sign_digest(proof["digest"])
            external = sdk.PolymarketSigner.remote(local.address,sign)
            negative = vector["domain"]["exchange"].lower().startswith("0xe222")
            assert await external.sign_order(order, system="v2" if vector["domain"]["version"] == 3 else "ctf",
                                             negative_risk=negative) == vector["signature"]
            wallet = {"key": golden["key"], "order_domain": vector["domain"]}
            clob = {"address": golden["proxy"], "key": "api", "secret": "YQ==", "passphrase": "pass"}
            signer = sdk.PolymarketClient(origin=origin, clob=clob, wallet=wallet)
            for name, path in (("clob_post_order","/order"), ("clob_post_orders","/orders")):
                entry = {"order":order}
                expected = {"order":{**order,"signature": vector["signature"]}}
                batch = path == "/orders"
                server.expected = ({"verb":"POST","route":path,"response":True},
                                   [expected, expected] if batch else expected, {"POLY_API_KEY":"api"})
                assert await getattr(signer,name)({"body":[entry,entry] if batch else entry}) is True

            if order["signatureType"] == 3:
                signature = bytes.fromhex(vector["signature"][2:])
                # ERC-6492 wraps ABI-encoded (address word, zero word, bytes),
                # followed by the magic suffix.
                address = bytes.fromhex(golden["proxy"][2:]).rjust(32, b"\x00")
                wrapped = address + bytes(32) + (96).to_bytes(32,"big") + len(signature).to_bytes(32,"big")
                wrapped += signature + bytes(-len(signature) % 32) + bytes.fromhex("6492"*16)
                session = sdk.PolymarketClient(origin=origin, clob=clob, wallet={**wallet,"session_signer":True})
                server.expected = ({"verb":"POST","route":"/order","response":True},
                                   {"order":{**order,"signature":"0x"+wrapped.hex()}},{"POLY_API_KEY":"api"})
                assert await session.clob_post_order({"body":{"order":order}}) is True

        for bearer_config, name in [
            ({"token":token}, "orders_cancel_all"),
            ({"token":"fixture.eyJleHAiOjF9.fixture"}, "get_account"),
            ({"token":token + "\r\nX: bad"}, "get_account"),
            ({"token":"invalid"}, "get_account"),
        ]:
            bad = sdk.ParadexClient(origin=origin, bearer=bearer_config)
            try:
                await getattr(bad, name)({})
            except Exception as exc:
                assert "auth" in str(exc).lower(), exc
            else:
                raise AssertionError("invalid bearer reached dispatch")
        for bad_config in ({"proxy":golden["proxy"],"secret":"secret","expires_at_ms":1},
                           {"proxy":golden["proxy"],"secret":"secret\n"}):
            try:
                await sdk.PolymarketPerpsClient(origin=origin, perps=bad_config).get_balances({})
            except Exception as exc:
                assert "auth" in str(exc).lower(), exc
            else:
                raise AssertionError("invalid proxy credentials reached dispatch")

    with ThreadingHTTPServer(("127.0.0.1", 0), Handler) as server:
        worker = threading.Thread(target=server.serve_forever, daemon=True)
        worker.start()
        try:
            asyncio.run(run(server))
        finally:
            server.shutdown()
            worker.join()


def test_network():
    orders = fixture("derive-orders")
    login_ops = set((HERE.parent / "src/exchanges/derive/login-routes.txt").read_text().splitlines())
    login_rows = {r["route"]: r for r in fixture("http") if r["venue"] == "derive" and r["operation"] in login_ops}
    seen_nonces = set()
    rpc_rows = {r["method"]: r for r in fixture("evm-rpc")}
    f = fixture("derive-signing")

    class Handler(BaseHTTPRequestHandler):
        def do_POST(self):
            body = loads(self.rfile.read(int(self.headers["Content-Length"])))
            if self.path.startswith("/rpc"):
                assert self.headers["Content-Type"] == "application/json"
                row = rpc_rows[body["method"]]
                assert body["params"] == row["args"] and body["jsonrpc"] == "2.0"
                id_ = body["id"] + (self.path == "/rpc/wrong-id")
                result = {"id": id_, "jsonrpc": "2.0", "result": row["reply"]}
            elif self.path.startswith("/v3/public/"):
                assert self.headers["X-DeriveSignature"] is None
                key = body["instrument_name"] if self.path.endswith("/get_instrument") else "currencies"
                result = {"result": orders["metadata"][key]}
            elif self.path in login_rows and body == login_rows[self.path]["request"]:
                proof = sdk.derive_login(f["key"], f["owner"], int(self.headers["X-DeriveTimestamp"]))
                assert self.headers["X-DeriveWallet"] == proof["wallet"]
                assert self.headers["X-DeriveSignature"] == proof["signature"]
                result = {"result": login_rows[self.path]["response"]}
            elif self.path.startswith("/v3/private/") and not self.path.endswith("/cancel"):
                name = self.path.split("/")[-1]
                row = orders["orders"][name]
                if name in {"transfer_positions", "transfer_positions_debug"}:
                    assert body["maker_params"]["signature"] == row["maker"] and body["taker_params"]["signature"] == row["taker"]
                else:
                    assert body == {**row["params"], "signature": row["signature"], "signer": f["signer"].lower()}
                assert self.headers["X-DeriveSignature"] is not None
                result = {"result": {"accepted": name}}
            else:
                assert self.path == "/v3/private/cancel"
                proof = sdk.derive_login(f["key"], f["owner"], int(self.headers["X-DeriveTimestamp"]))
                assert self.headers["X-DeriveWallet"] == proof["wallet"]
                assert self.headers["X-DeriveSignature"] == proof["signature"]
                assert body == {"subaccount_id": 9, "order_id": "fixture"}
                result = {"result": {"status": "cancelled"}}
            self.send_response(200)
            self.end_headers()
            self.wfile.write(dump(result).encode())

        def do_GET(self):
            if self.path.startswith("/md/contracts?"):
                assert self.path == "/md/contracts?api_version=2026-10-29&symbol=ETH"
                self.send_response(200)
                self.end_headers()
                self.wfile.write(b'{"data":[{"symbol":"ETH"}]}')
                return
            if self.path in ("/user/positions?api_version=2026-10-29","/user/subaccounts/balance?api_version=2026-10-29"):
                nonce, ts = self.headers["x-qfex-nonce"], self.headers["x-qfex-timestamp"]
                assert self.headers["x-qfex-public-key"] == "api-key" and len(bytes.fromhex(nonce)) == 16
                assert self.headers["x-qfex-hmac-signature"] == hmac.new(b"secret", f"{nonce}:{ts}".encode(), hashlib.sha256).hexdigest()
                assert nonce not in seen_nonces
                seen_nonces.add(nonce)
                self.send_response(200)
                self.end_headers()
                self.wfile.write(b'{"positions":[]}' if self.path.startswith("/user/positions?") else b'{"cash":"44.001"}')
                return
            if self.path.startswith("/v2/updates/price/stream?"):
                assert self.headers["accept"] == "text/event-stream"
                self.send_response(200)
                self.send_header("content-type", "text/event-stream")
                self.end_headers()
                self.wfile.write(b'data: {"price":9999999999999999999}\r\n\r\n')
                self.wfile.flush()
                self.connection.settimeout(5)
                assert self.connection.recv(1) == b""
                return
            if self.path == "/api/v1/user/accounts":
                assert self.headers["X-Api-Key"] == "fixture" or self.headers["Authorization"] == "Bearer replacement"
                self.send_response(200)
                self.send_header("content-encoding", "zstd")
                self.end_headers()
                self.wfile.write((HERE / "http-body.zst").read_bytes())
                return
            if self.path.startswith("/custom/api/v1/reports/latest?"):
                key = self.headers["Authorization"]
                assert key == "fixture"
                timestamp = self.headers["X-Authorization-Timestamp"]
                signed = f"GET {self.path} {hashlib.sha256(b'').hexdigest()} {key} {timestamp}"
                tag = hmac.new(b"secret", signed.encode(), hashlib.sha256).hexdigest()
                assert tag == self.headers["X-Authorization-Signature-SHA256"]
                body = gzip.compress(b'{"price":12345678901234567890.125}')
                self.send_response(200)
                self.send_header("content-encoding", "gzip")
                self.send_header("retry-after", "7")
                self.send_header("content-length", str(len(body)))
                self.end_headers()
                self.wfile.write(body)
                return
            assert self.headers["X-API-KEY"] == "fixture"
            assert self.headers["x-chain"] == "solana"
            assert self.path == "/defi/price?address=Test"
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b'{"success":true,"data":0.1234567890123456789}')

        def log_message(self, *_):
            pass

    with ThreadingHTTPServer(("127.0.0.1", 0), Handler) as server:
        worker = threading.Thread(target=server.serve_forever, daemon=True)
        worker.start()
        try:
            origin = f"http://127.0.0.1:{server.server_port}"
            reader = sdk.EvmReader(origin + "/rpc")
            for row in rpc_rows.values():
                result = asyncio.run(getattr(reader, row["operation"])(**row["params"]))
                assert result == row["expected"]
            try:
                asyncio.run(sdk.EvmReader(origin + "/rpc/wrong-id").chain_id())
            except ValueError:
                pass
            else:
                raise AssertionError("mismatched RPC id accepted")
            derive = sdk.DeriveClient(origin=origin, api_key=f["owner"], api_secret=f["key"])
            assert asyncio.run(derive.private_cancel({"subaccount_id": 9, "order_id": "fixture"})) == {"status": "cancelled"}
            for row in login_rows.values():
                result = asyncio.run(getattr(derive, row["operation"])(row["request"]))
                assert result == row["response"]
            trading = sdk.DeriveClient(origin=origin, api_key=f["owner"], api_secret=f["key"], chain_id=1)
            for name, row in orders["orders"].items():
                assert asyncio.run(getattr(trading, "private_" + name)(row["params"])) == {"accepted": name}
            qfex = sdk.QfexClient(origin=origin, api_key="api-key", api_secret="secret")
            assert asyncio.run(qfex.account_balances()) == {"cash":"44.001"}
            assert asyncio.run(qfex.contracts(symbol="ETH")) == {"data":[{"symbol":"ETH"}]}
            for _ in range(2):
                assert asyncio.run(qfex.positions()) == {"positions": []}
            result = asyncio.run(
                sdk.BirdeyeClient(
                    origin=f"http://127.0.0.1:{server.server_port}", api_key="fixture"
                ).price({"address": "Test"})
            )
            assert result["data"] == Decimal("0.1234567890123456789")
            extended = sdk.ExtendedClient(
                origin=f"http://127.0.0.1:{server.server_port}", api_key="fixture"
            )
            assert asyncio.run(extended.get_accounts())["data"] == [1, 2, 3]
            extended.set_credentials({"access_token":"replacement","expires_at":4102444800})
            assert extended.authenticated and asyncio.run(extended.get_accounts())["data"] == [1,2,3]
            extended.set_credentials({"access_token":"expired","expires_at":1})
            try:
                asyncio.run(extended.get_accounts())
            except sdk.AuthenticationError:
                pass
            else:
                raise AssertionError("expired bearer accepted")
            client = sdk.ChainlinkDataStreamsClient(
                origin=f"http://127.0.0.1:{server.server_port}/custom",
                api_key="fixture",
                api_secret="secret",
            )
            report = asyncio.run(client.latest_report({"feedID": "0x" + "42" * 32}))
            assert report["data"]["price"] == Decimal("12345678901234567890.125")
            assert ["retry-after", "7"] in report["headers"]

            async def stream_fixture():
                client = sdk.PythHermesClient(
                    origin=f"http://127.0.0.1:{server.server_port}"
                )
                stream = await client.stream_price_updates({"ids": ["42" * 32]})
                assert (await anext(stream))["price"] == 9999999999999999999
                reading = asyncio.create_task(anext(stream, None))
                await asyncio.sleep(0)
                await stream.close()
                assert await asyncio.wait_for(reading, 2) is None
                await stream.close()

            asyncio.run(stream_fixture())
        finally:
            server.shutdown()
            worker.join()


async def test_perps_connectors(origin):
    sign = fixture("perps-signing")
    routes = fixture("perps-http")
    ws = origin.replace("http:", "ws:")
    address = sign["owner"].lower()
    market = "0x" + "33" * 32

    def row(venue, operation):
        return next(r for r in routes if r["venue"] == venue and r["operation"] == operation)

    clients = {}
    for venue, cls in [
        ("injective", sdk.InjectiveClient), ("nado", sdk.NadoClient),
        ("ondo_perps", sdk.OndoPerpsClient), ("pacifica", sdk.PacificaClient),
        ("paragon", sdk.ParagonClient), ("trade_xyz", sdk.TradeXyzClient),
        ("aster", sdk.AsterClient),
    ]:
        clients[venue] = cls(origin=f"{origin}/perps/{venue}")
        assert await clients[venue].markets(row(venue, "markets")["request"]) == row(venue, "markets")["response"]

    injective = sdk.InjectiveClient(origin=origin + "/perps/injective", auth={"key": sign["key"]})
    async with await injective.subscribe({"channel": "order_book", "market_id": market}) as stream:
        event = await anext(stream)
        assert event["orderbook"] == {"buys": [], "sells": [], "sequence": 9007199254740993}
        assert await anext(stream, None) is None
    async with await injective.subscribe({"channel": "order_book", "market_id": market}, endpoint=origin + "/perps/injective/trailer-error") as stream:
        try:
            await anext(stream)
        except sdk.ExchangeError as err:
            assert err.venue == "injective" and err.body["grpc_status"] == 13
        else:
            raise AssertionError("expected an Injective gRPC error trailer")
    subaccount = address + "00" * 12
    body = dict(row("injective", "place_order")["request"]["body"], subaccount_id=subaccount, client_order_id="lost-ack")
    try:
        await injective.place_order({"body": body})
    except sdk.OutcomeUncertainError:
        pending = await injective.pending()
        assert len(pending["hash"]) == 64 and pending["timeout_height"] == 9020
    else:
        raise AssertionError("expected uncertain broadcast")
    tx = await injective.transaction({"hash": pending["hash"]})
    assert tx["tx_response"]["txhash"] == pending["hash"]
    cancel = dict(row("injective", "cancel_order")["request"]["body"], subaccount_id=subaccount)
    assert (await injective.cancel_order({"body": cancel}))["tx_response"]["code"] == 0

    nado = sdk.NadoClient(origin=origin + "/perps/nado", auth={"key": sign["key"]})
    for operation in ("place_order", "cancel_order"):
        assert await getattr(nado, operation)(row("nado", operation)["request"]) == row("nado", operation)["response"]
    async with await nado.subscribe({"type": "book_depth", "product_id": 4, "snapshot_timestamp":"100"}, endpoint=ws + "/perps/nado/ws") as stream:
        assert (await anext(stream))["bids"][0] == ["123000000000000000000", "4200000000000000000"]
        assert (await anext(stream))["max_timestamp"] == "120"
        try:
            await anext(stream)
        except sdk.StreamGapError:
            pass
        else:
            raise AssertionError("expected Nado depth gap")

    nado = sdk.NadoClient(auth={"key": sign["key"], "rest_origin": origin + "/perps/nado"})
    async with await nado.subscribe({"type": "order_update", "product_id": 4, "subaccount": "0x" + "22" * 32}, endpoint=ws + "/perps/nado/private") as stream:
        first = await anext(stream)
        second = await anext(stream)
        assert int(second["timestamp"]) == int(first["timestamp"]) + 1

    ondo = sdk.OndoPerpsClient(origin=origin + "/perps/ondo_perps", api_key="fixture-key", api_secret="fixture-secret")
    assert (await ondo.account())["success"]
    assert (await ondo.place_order(row("ondo_perps", "place_order")["request"]))["success"]
    request = dict(row("ondo_perps", "place_order")["request"]["body"], clientOrderId="lost-ack")
    try:
        await ondo.place_order({"body": request})
    except sdk.OutcomeUncertainError:
        assert (await ondo.order_status(row("ondo_perps", "order_status")["request"]))["success"]
    else:
        raise AssertionError("expected ambiguous Ondo write")
    ondo = sdk.OndoPerpsClient(auth={"token": "fixture-token"})
    async with await ondo.subscribe({"channel": "fillsPerps"}, endpoint=ws + "/perps/ondo_perps/ws") as stream:
        first = (await anext(stream))["data"]
        second = (await anext(stream))["data"]
        assert first["id"] == 9007199254740993 and second["seq"] == first["seq"] + 1
    async with await ondo.subscribe({"channel": "fillsPerps"}, endpoint=ws + "/perps/ondo_perps/denied") as stream:
        try:
            await anext(stream)
        except sdk.ExchangeError as err:
            assert err.venue == "ondo_perps" and err.body["code"] == "invalid_token"
        else:
            raise AssertionError("expected rejected login")

    pacifica = sdk.PacificaClient(origin=origin + "/perps/pacifica", auth={
        "account": "FAe4sisG95oZ42w7buUn5qEE4TAnfTTFPiguZUHmhiF", "seed": bytes(range(32)).hex(),
    })
    for operation in ("place_order", "place_market_order", "cancel_order"):
        assert await getattr(pacifica, operation)(row("pacifica", operation)["request"]) == row("pacifica", operation)["response"]
    async with await pacifica.subscribe({"source": "trades", "symbol": "BTC"}, endpoint=ws + "/perps/pacifica/ws") as stream:
        assert (await anext(stream))["data"] == [{"s": "BTC", "p": "10000.001"}]
    async with await pacifica.subscribe({"source": "book", "symbol": "BTC"}, endpoint=ws + "/perps/pacifica/book") as stream:
        assert (await anext(stream))["data"]["l"] == [[], []]

    for venue, cls, coin in [
        ("paragon", sdk.ParagonClient, "para:TOTAL2"), ("trade_xyz", sdk.TradeXyzClient, "xyz:TSLA"),
    ]:
        client = cls(origin=f"{origin}/perps/{venue}", auth={"key": sign["key"]})
        for operation in ("place_order", "cancel_order"):
            params = dict(row(venue, operation)["request"], options={"nonce": 1754450974231})
            assert await getattr(client, operation)(params) == row(venue, operation)["response"]
        async with await client.subscribe({"type": "orderUpdates", "user": address}, endpoint=f"{ws}/perps/{venue}/ws") as stream:
            assert [r["order"]["coin"] for r in (await anext(stream))["data"]] == [coin]
        try:
            sdk.prepare_request(venue, "order_book", {"coin": "OTHER:BTC"})
        except sdk.ValidationError:
            pass
        else:
            raise AssertionError("expected rejection for a foreign market")

    aster = sdk.AsterClient(origin=origin + "/perps/aster", auth={"user": address, "key": sign["key"], "rest_origin": origin + "/perps/aster"})
    for operation in ("account", "place_order", "cancel_order"):
        assert await getattr(aster, operation)(row("aster", operation)["request"]) == row("aster", operation)["response"]
    async with await aster.subscribe({"channel": "account"}, endpoint=ws + "/perps/aster/ws") as stream:
        assert (await anext(stream))["E"] == 1
        assert (await anext(stream))["E"] == 2



async def test_new_exchanges(origin):
    origin += "/new-exchanges"
    websocket = origin.replace("http:", "ws:", 1)
    key = "0x" + "00" * 31 + "01"
    wallet = "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf"
    seed = bytes(range(32)).hex()
    market = "0x" + "33" * 32
    account = "0x" + "22" * 32
    credentials = dict(api_key="fixture-key", api_secret="fixture-secret")
    clients = {
        "perpl": sdk.PerplClient(origin=origin + "/perpl", api_key="fixture-key", api_secret=seed),
        "risex": sdk.RisexClient(origin=origin + "/risex", auth=dict(key=key, account="0x" + "11" * 20, rest_origin=origin + "/risex")),
        "decibel": sdk.DecibelClient(origin=origin + "/decibel", api_key="fixture-key", auth=dict(seed=seed, subaccount=account, rest_origin=origin + "/decibel")),
        "edgex": sdk.EdgeXClient(origin=origin + "/edgex", auth=dict(key=key, passphrase="fixture-passphrase", account_id="123456789012345678"), **credentials),
        "grvt": sdk.GrvtClient(origin=origin + "/grvt", auth=dict(key=key, cookie="gravity=fixture", account_id="123"), **credentials),
        "katana": sdk.KatanaClient(origin=origin + "/katana", auth=dict(key=key, wallet=wallet, rest_origin=origin + "/katana"), **credentials),
        "variational": sdk.VariationalClient(origin=origin + "/variational"),
    }
    for row in fixture("perps-http"):
        venue, method = row["venue"], row["operation"]
        if venue not in clients:
            continue
        request = row["request"]
        if venue == "katana":
            if "wallet" in request:
                request = {**request, "wallet": wallet}
            elif "body" in request:
                request = {"body": {**request["body"], "wallet": wallet}}
        result = await getattr(clients[venue], method)(request)
        if venue == "edgex" and method == "markets":
            assert result["data"]["contractList"][0]["resolution"] == "10000000000"
        elif venue == "decibel" and method in {"place_order", "cancel_order"}:
            assert result["type"] == "pending_transaction" and len(result["hash"]) == 66
        else:
            assert result == row["response"], (venue, method, result)

    streams = {
        "perpl": [{"channel":"order_book","market_id":1},{"channel":"trades","market_id":1},{"channel":"account"}],
        "risex": [{"channel":"orderbook","market_ids":[1]},{"channel":"trades","market_ids":[1]},{"channel":"account"}],
        "decibel": [{"channel":"depth","market":market},{"channel":"trades","market":market},{"channel":"order_updates","account":account}],
        "edgex": [{"channel":"depth","contract_id":"10000001"},{"channel":"trades","contract_id":"10000001"},{"channel":"account"}],
        "grvt": [{"channel":"v1.book.s","instrument":"BTC_USDT_Perp"},{"channel":"v1.trade","instrument":"BTC_USDT_Perp"},{"channel":"v1.order","sub_account_id":"123456789"}],
        "katana": [{"channel":"l2orderbook","market":"ETH-USD"},{"channel":"trades","market":"ETH-USD"},{"channel":"orders"}],
    }
    for venue, specs in streams.items():
        for name, spec in zip(("book", "trades", "account"), specs):
            async with await clients[venue].subscribe(spec, endpoint=f"{websocket}/{venue}/ws/{name}") as stream:
                event = await asyncio.wait_for(anext(stream), 8)
                if venue == "perpl" and name == "book":
                    assert event["mt"] == 15 and event["bid"][0]["p"] == 9007199254740993
                if venue == "risex" and name == "book":
                    assert event["type"] == "snapshot" and event["market_id"] == "1"
                if venue == "katana" and name == "book":
                    assert event["snapshot"] and event["data"]["sequence"] == 9007199254740993
                    assert (await asyncio.wait_for(anext(stream), 8))["data"]["u"] == 9007199254740994
                    assert (await asyncio.wait_for(anext(stream), 8))["snapshot"]
                if venue == "decibel":
                    assert event["unix_ms"] == 9007199254740993

    async with await clients["perpl"].subscribe({"channel":"order_book","market_id":1}, endpoint=websocket + "/perpl/ws/reject") as stream:
        try:
            await anext(stream)
        except sdk.ExchangeError as error:
            assert error.venue == "perpl" and error.body["subs"][0]["status"]["code"] == 404
        else:
            raise AssertionError("accepted rejected subscription")

    async with await clients["perpl"].subscribe({"channel":"account"}, endpoint=websocket + "/perpl/ws/account-denied") as stream:
        try:
            await anext(stream)
        except sdk.ExchangeError as error:
            assert error.venue == "perpl" and error.body["status"]["code"] == 401
        else:
            raise AssertionError("private feed accepted revoked credentials")

    rise_order = next(row["request"]["body"] for row in fixture("perps-http") if row["venue"] == "risex" and row["operation"] == "place_order")
    placed = await asyncio.gather(*(
        clients["risex"].place_order({"body": {**rise_order, "client_order_id": str(request_id)}})
        for request_id in (9007199254740997, 9007199254740998)
    ))
    assert all(result["data"]["tx_hash"].startswith("0x") for result in placed)

    perpl_order = next(row["request"]["body"] for row in fixture("perps-http") if row["venue"] == "perpl" and row["operation"] == "place_order")
    try:
        await clients["perpl"].place_order({"body": {**perpl_order, "rq":9007199254740995}})
    except sdk.ExchangeError as error:
        assert error.body["statuses"][0]["code"] == 403
    else:
        raise AssertionError("accepted rejected order")

    lost = sdk.DecibelClient(origin=origin + "/decibel/lost", api_key="fixture-key", auth=dict(seed=seed, subaccount=account, rest_origin=origin + "/decibel"))
    order = next(row["request"] for row in fixture("perps-http") if row["venue"] == "decibel" and row["operation"] == "place_order")
    try:
        await lost.place_order(order)
    except sdk.OutcomeUncertainError as error:
        assert error.transaction_hash.startswith("0x") and len(error.transaction_hash) == 66
    else:
        raise AssertionError("lost Aptos acknowledgement treated as success")
    from urllib.request import urlopen
    with urlopen(origin + "/decibel/lost-count", timeout=5) as reply:
        assert json.load(reply)["count"] == 1

    try:
        await sdk.VariationalClient(origin=origin + "/variational/rate-limited").market_stats()
    except sdk.ExchangeError as error:
        assert error.status == 429 and error.body["retry_after_seconds"] == 4
    else:
        raise AssertionError("rate limit treated as success")

async def test_native_exchange_clients(origin):
    base = origin + "/native-exchanges/"
    ws = base.replace("http:", "ws:", 1)
    markets = {row["venue"]: row for row in fixture("exchange-http") if row["operation"] == "markets"}
    for row in fixture("exchange-ws"):
        venue = row["venue"]
        market = markets[venue]
        client = getattr(sdk, row["client"])(origin=base + venue + "/rest/public", api_key="fixture-key", api_secret="fixture-secret")
        assert await client.markets(market["request"]) == market["response"], venue
        async with await client.subscribe(row["params"], endpoint=ws + venue + "/ws/book") as stream:
            assert await anext(stream) == row["event"], venue
            assert await anext(stream, None) is None, venue

    market_clients = {"kuru": sdk.KuruClient, "reya": sdk.ReyaClient, "apex_omni": sdk.ApexOmniClient}
    backpack = sdk.BackpackClient(origin=base + "backpack/rest/public")
    binance_public = sdk.BinanceSpotClient(origin=base + "binance_spot/rest/public", api_key="fixture-key", api_secret="fixture-secret")
    for row in fixture("exchange-http"):
        venue = row["venue"]
        if venue in market_clients:
            client = market_clients[venue](origin=base + venue + "/rest/public")
            assert await getattr(client, row["operation"])(row["request"]) == row["response"]
        elif venue == "backpack" and row["operation"] == "depth":
            assert await backpack.depth(row["request"]) == row["response"]
            assert await backpack.depth({"symbol": row["request"]["symbol"]}) == row["response"]
        elif venue == "binance_spot" and row["operation"] == "depth":
            assert await binance_public.depth(row["request"]) == row["response"]
    market_streams = fixture("market-data-ws")
    for row in market_streams:
        client = getattr(sdk, row["client"])()
        async with await client.subscribe(row["params"], endpoint=ws + row["venue"] + "/ws/" + row["case"]) as stream:
            for event in row["events"]:
                assert await anext(stream) == event, row["case"]
            if not row.get("keep_open"):
                assert await anext(stream, None) is None

    kuru = sdk.KuruClient()
    book = next(row["params"] for row in market_streams if row["case"] == "l2-bootstrap")
    async with await kuru.subscribe(dict(book, feedEpoch=12, seq=2**53+1), endpoint=ws + "kuru/ws/epoch-mismatch") as stream:
        try:
            await anext(stream)
        except sdk.ExchangeError as error:
            assert error.body["c"] == "feed_epoch_mismatch" and error.body["feedEpoch"] == 13
        else:
            raise AssertionError("stale cursor accepted")
    async with await kuru.subscribe(book, endpoint=ws + "kuru/ws/truncated") as stream:
        try:
            await anext(stream)
        except sdk.ValidationError:
            pass
        else:
            raise AssertionError("truncated Kuru frame accepted")

    orderly = sdk.OrderlyClient(origin=base + "orderly/rest/public")
    for name in ("market_summary", "market_trades"):
        row = next(item for item in fixture("exchange-http") if item["venue"] == "orderly" and item["operation"] == name)
        assert await getattr(orderly, name)(row["request"]) == row["response"]

    key = dict(api_key="fixture-key", api_secret="fixture-secret")
    order = {"symbol": "ETHUSDT", "side": "BUY", "type": "LIMIT", "quantity": "0.01", "price": "100"}
    binance = sdk.BinanceSpotClient(origin=base + "binance_spot/rest/private", **key)
    assert (await binance.place_order(order))["orderId"] == 2**53 + 1
    unsigned = sdk.BinanceSpotClient(origin=base + "binance_spot/rest/private")
    try:
        await unsigned.place_order(order)
    except sdk.ConnectorError:
        pass
    else:
        raise AssertionError("unsigned order reached Binance")
    lost_order = sdk.BinanceSpotClient(origin=base + "binance_spot/rest/lost", **key)
    with urllib.request.urlopen(base + "binance_spot/count/lost") as reply:
        count = json.load(reply)["count"]
    try:
        await lost_order.place_order(order)
    except sdk.ConnectorError:
        pass
    else:
        raise AssertionError("lost REST response treated as confirmed")
    with urllib.request.urlopen(base + "binance_spot/count/lost") as reply:
        assert json.load(reply)["count"] == count + 1
    async with await binance.subscribe({"channel":"fills"}, endpoint=ws + "binance_spot/ws/fills") as stream:
        assert (await anext(stream))["event"]["i"] == 2**53 + 1
    async with await binance.open_rpc(endpoint=ws + "binance_spot/ws/partial") as session:
        try:
            await session.cancel_replace_order(order)
        except sdk.ExchangeError as error:
            assert error.body["error"]["data"]["cancelResult"] == "SUCCESS"
        else:
            raise AssertionError("partial cancel-replace reported full success")

    async with await binance.open_rpc(endpoint=ws + "binance_spot/ws/late", timeout_ms=50) as session:
        for _ in range(2):
            try:
                await session.place_order(order)
            except sdk.ConnectorError:
                assert session.pending["id"] == 1
            else:
                raise AssertionError("pending request was resent")
        for _ in range(10):
            try:
                result = await session.reconcile()
                break
            except sdk.ConnectorError:
                continue
        else:
            raise AssertionError("late acknowledgement was not reconciled")
        assert result["result"]["orderId"] == 2**53 + 1


    def base58(data):
        alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
        zeros = len(data) - len(data.lstrip(b"\0"))
        value = int.from_bytes(data, "big")
        digits = []
        while value:
            value, digit = divmod(value, 58)
            digits.append(alphabet[digit])
        return "1" * zeros + "".join(reversed(digits))

    ed_seed = bytes(range(32))
    ed_public = bytes.fromhex("03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8")
    orderly = sdk.OrderlyClient(origin=base + "orderly/rest/private", api_key="ed25519:" + base58(ed_public),
                                api_secret=base58(ed_seed), auth={"account_id": "fixture-account"})
    assert (await orderly.balances())["data"]["holding"] == []
    backpack = sdk.BackpackClient(origin=base + "backpack/rest/private",
                                  api_key=base64.b64encode(ed_public).decode(), api_secret=base64.b64encode(ed_seed).decode())
    assert (await backpack.balances())["USDC"]["available"] == "1.23"
    deribit = sdk.DeribitClient(origin=base + "deribit/rest/private", **key)
    assert (await deribit.account({"currency":"ETH"}))["result"]["equity"] == 1
    okx = sdk.OkxClient(origin=base + "okx/rest/private", **key,
                        auth={"network":"Testnet", "passphrase":"fixture-passphrase"})
    assert (await okx.balances({"ccy":"USDT"}))["data"][0]["totalEq"] == "1.23"
    hibachi_body = {"symbol":"ETH/USDT-P", "side":"BID", "orderType":"LIMIT",
                    "quantity":"0.01", "price":"100", "maxFeesPercent":"0.00045"}
    hibachi = sdk.HibachiClient(origin=base + "hibachi/rest/private", **key, auth={"account_id":123})
    assert (await hibachi.place_order({"body":hibachi_body}))["orderId"] == 2**53 + 1
    trustless = sdk.HibachiClient(origin=base + "hibachi/rest/private", api_key="fixture-key", api_secret="0" * 63 + "1",
                                  auth={"account_id":123, "signing":"ecdsa"})
    assert (await trustless.place_order({"body":hibachi_body}))["orderId"] == 2**53 + 1
    aevo = sdk.AevoClient(origin=base + "aevo/rest/private", **key, auth={"wallet":"0x" + "11" * 20, "key":"0" * 63 + "1"})
    aevo_order = {"instrument":1, "is_buy":True, "amount":"10000", "limit_price":"100000000"}
    assert (await aevo.place_order({"body":aevo_order}))["order_id"] == "fixture-order"
    assert (await aevo.amend_order({"order_id":"old-order","body":aevo_order}))["order_id"] == "fixture-order"
    assert (await aevo.cancel_orders({"body":{"order_ids":["old-order","other-order"]}}))["order_ids"] == ["old-order"]
    aevo_public = sdk.AevoClient(origin=base + "aevo/rest/public")
    assert (await aevo_public.trades({"instrument_name":"ETH-PERP"}))["trade_history"] == []
    bybit_rest = sdk.BybitClient(origin=base + "bybit/rest/private", **key)
    assert (await bybit_rest.place_order({"body":{"category":"linear", "symbol":"ETHUSDT", "side":"Buy",
                                                  "orderType":"Limit", "qty":"0.01", "price":"100"}}))["result"]["orderId"] == "fixture-order"
    bybit = sdk.BybitClient(**key)
    async with await bybit.open_rpc(endpoint=ws + "bybit/ws/mixed") as session:
        result = await session.place_orders({"category": "linear", "request": [order, order]})
        assert [item["code"] for item in result["retExtInfo"]["list"]] == [0, 10001]

    kraken = sdk.KrakenSpotClient(api_key="fixture-key", api_secret="Zml4dHVyZS1zZWNyZXQ=",
                                  origin=base + "kraken_spot/rest/concurrent")
    assert all(response["result"]["fixture"] for response in await asyncio.gather(*(kraken.balances() for _ in range(8))))

    async with await deribit.open_rpc(endpoint=ws + "deribit/ws/orders") as session:
        result = await session.buy({"instrument_name":"ETH-PERPETUAL","amount":1,"type":"market"})
        assert result["result"]["order"]["order_id"] == "test"
    async with await okx.open_rpc(endpoint=ws + "okx/ws/orders") as session:
        item = {"instId":"ETH-USDT","tdMode":"cash","side":"buy","ordType":"limit","px":"100","sz":"0.01"}
        result = await session.place_orders([item,item])
        assert [entry["sCode"] for entry in result["data"]] == ["0","51000"]
    kraken_auth = sdk.KrakenSpotClient(api_key="fixture-key", api_secret="Zml4dHVyZS1zZWNyZXQ=",
                                       auth={"rest_origin":base + "kraken_spot/rest/token"})
    async with await kraken_auth.open_rpc(endpoint=ws + "kraken_spot/ws/orders") as session:
        result = await session.place_order({"symbol":"ETH/USD","side":"buy","order_type":"limit",
                                            "order_qty":"0.01","limit_price":"100"})
        assert result["result"]["order_id"] == "test"
    async with await aevo.open_rpc(endpoint=ws + "aevo/ws/orders") as session:
        result = await session.place_order({"instrument":1,"is_buy":True,"amount":"10000","limit_price":"100000000"})
        assert result["data"]["order_id"] == "test"
    hibachi = sdk.HibachiClient(**key, auth={"account_id":123, "market_origin":base + "hibachi/rest/private"})
    async with await hibachi.open_rpc(endpoint=ws + "hibachi/ws/orders") as session:
        result = await session.place_order({"symbol":"ETH/USDT-P","side":"BID","orderType":"LIMIT",
                                            "quantity":"0.01","price":"100","maxFeesPercent":"0.00045"})
        assert result["result"]["orderId"] == 2**53 + 1

    signer = "0" * 63 + "1"
    body = {"subaccount_number": 0, "client_id": 42, "clob_pair_id": 0, "order_flags": 0,
            "good_til_block": 120, "side": 1, "quantums": 1000, "subticks": 10000}
    dydx = sdk.DydxV4Client(auth={"key": signer, "node_origin": base + "dydx_v4/rest/private"})
    assert (await dydx.place_order({"body": body}))["tx_response"]["code"] == 0
    broken = sdk.DydxV4Client(auth={"key": signer, "node_origin": base + "dydx_v4/rest/wrong-chain"})
    try:
        await broken.place_order({"body": body})
    except sdk.ConnectorError:
        pass
    else:
        raise AssertionError("wrong dYdX chain was accepted")
    lost = sdk.DydxV4Client(auth={"key": signer, "node_origin": base + "dydx_v4/rest/lost"})
    stateful = dict(body, order_flags=64, good_til_block_time=4102444800)
    stateful.pop("good_til_block")
    with urllib.request.urlopen(base + "dydx_v4/count/lost") as reply:
        before = json.load(reply)["count"]
    for _ in range(2):
        try:
            await lost.place_order({"body": stateful})
        except sdk.ConnectorError:
            pass
        else:
            raise AssertionError("lost transaction treated as confirmed")
    with urllib.request.urlopen(base + "dydx_v4/count/lost") as reply:
        assert json.load(reply)["count"] == before + 1


if __name__ == "__main__":
    for row in fixture("extended-settlement"):
        args = (row["kind"], row["chain_id"], row["public_key"], row["message"])
        assert sdk.extended_settlement(*args) == {"hash": row["expected"]["hash"]}
        assert sdk.ExtendedClient(chain_id=row["chain_id"]).settlement_hash(args[0],args[2],args[3]) == row["expected"]["hash"]
        assert sdk.extended_settlement(*args, row["stark_key"]) == row["expected"]
    for row in fixture("extended-orders"):
        assert sdk.extended_prepare_order(row["chain_id"],row["public_key"],row["body"],row["context"]) == row["prepared"]
        assert sdk.extended_sign_order(row["chain_id"],row["stark_key"],row["body"],row["context"]) == row["signed"]
        ext = sdk.ExtendedClient(chain_id=row["chain_id"])
        signer = sdk.ExtendedSigner(row["stark_key"])
        assert ext.prepare_order(signer.public_key,row["body"],row["context"]) == row["prepared"]
        assert asyncio.run(ext.sign_order(signer,row["body"],row["context"])) == row["signed"]
    test_http()
    test_solana()
    test_solana_chain()
    test_evm_send()
    test_hyperdash_exports()
    if origin := os.environ.get("AUX_FIXTURE_ORIGIN"):
        asyncio.run(test_shared_peer(origin))
        asyncio.run(test_perps_connectors(origin))
        asyncio.run(test_new_exchanges(origin))
        asyncio.run(test_native_exchange_clients(origin))
        asyncio.run(test_live(os.environ["AUX_DBN_GATEWAY"]))
    test_trading_signatures()
    test_derive()
    test_protocols()
    test_clob()
    test_paradex_and_perps()
    test_network()
    asyncio.run(test_socket("extended"))
    asyncio.run(test_socket("kalshi"))
    asyncio.run(test_socket("pascal"))
    asyncio.run(test_socket())
    asyncio.run(test_socket("phoenix"))
    asyncio.run(test_socket("paradex"))
    asyncio.run(test_socket("derive"))
    asyncio.run(test_socket("derive-reject"))
    print("Installed wheel passed wire, codec, authentication, and loopback fixtures")
