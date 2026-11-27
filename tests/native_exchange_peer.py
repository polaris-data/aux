"""Loopback checks for the native exchange catalogs, signatures, and command protocols."""

import base64
from collections import Counter
import hashlib
import hmac
import json
from pathlib import Path
import time
import subprocess
from threading import Lock
from urllib.parse import parse_qs, urlsplit

from .exchange_peer import respond
from .trading_peer import upgrade, verify_ed25519
from .ws_server import frame, read_frame

HERE = Path(__file__).parent
HTTP = json.loads((HERE / "exchange-http.json").read_text())
WS = {row["venue"]: row for row in json.loads((HERE / "exchange-ws.json").read_text())}
MARKET_STREAMS = {
    (row["venue"], row["case"]): row for row in json.loads((HERE / "market-data-ws.json").read_text())
}
ED_PUBLIC = "03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8"
ACCOUNT = "dydx1w508d6qejxtdg4y5r3zarvary0c5xw7knye700"
NODE = "/cosmos/tx/v1beta1/txs"
LAST_NONCE = {}
LOST = Counter()
LOCK = Lock()


def mac(secret, message):
    return hmac.new(secret, message, "sha256").hexdigest()


def socket_json(peer):
    opcode, value = read_frame(peer.rfile)
    assert opcode == 1
    return json.loads(value)


def send(peer, value):
    peer.connection.sendall(frame(1, json.dumps(value, separators=(",", ":")).encode()))


def check_binance_signature(params):
    signature = params.pop("signature")
    assert params["apiKey"] == "fixture-key"
    message = "&".join(f"{key}={params[key]}" for key in sorted(params))
    assert signature == mac(b"fixture-secret", message.encode())


def books(peer, venue):
    fixture = WS[venue]
    assert socket_json(peer) == fixture["subscribe"]
    peer.connection.sendall(frame(9, b"test ping"))
    if venue == "deribit":
        heartbeat = socket_json(peer)
        assert heartbeat["method"] == "public/set_heartbeat"
        send(peer, {"jsonrpc": "2.0", "id": heartbeat["id"], "result": "ok"})
        send(peer, {"jsonrpc": "2.0", "method": "heartbeat", "params": {"type": "test_request"}})
    elif venue == "orderly":
        send(peer, {"event": "ping"})
    assert read_frame(peer.rfile) == (10, b"test ping")
    if venue == "deribit":
        assert socket_json(peer)["method"] == "public/test"
    elif venue == "orderly":
        assert socket_json(peer) == {"event": "pong"}
    send(peer, fixture["event"])
    peer.connection.sendall(frame(8, b"\x03\xe8"))
    assert read_frame(peer.rfile)[0] == 8


def market_stream(peer, venue, scenario):
    if scenario in ("epoch-mismatch", "truncated"):
        spec = MARKET_STREAMS["kuru", "l2-bootstrap"]
        sub = socket_json(peer)
        if scenario == "epoch-mismatch":
            assert sub == dict(spec["subscribe"], feedEpoch=12, seq=9007199254740993)
            send(peer, {"op": "err", "feedEpoch": 13, "id": 1, "c": "feed_epoch_mismatch", "msg": "stale epoch"})
        else:
            assert sub == spec["subscribe"]
            send(peer, {"op": "ack", "id": 1, "t": sub["t"], "feedEpoch": 13})
            peer.connection.sendall(frame(2, bytes.fromhex(spec["binary"][0])[:-1]))
        return

    spec = MARKET_STREAMS[venue, scenario]
    sub = socket_json(peer)
    assert sub == spec["subscribe"]
    assert peer.headers.get("Authorization") is None
    peer.connection.sendall(frame(9, b"market data ping"))
    assert read_frame(peer.rfile) == (10, b"market data ping")
    if venue == "kuru":
        send(peer, {"op": "ack", "id": 1, "t": sub["t"], "feedEpoch": 13})
        send(peer, {"op": "pong", "feedEpoch": 13})
        for index, encoded in enumerate(spec.get("binary", [])):
            packet = bytes.fromhex(encoded)
            if index == 0:
                peer.connection.sendall(frame(2, (2).to_bytes(8, "big") + packet[8:]))
                peer.connection.sendall(frame(2, packet[:7], fin=False) + frame(0, packet[7:]))
            else:
                peer.connection.sendall(frame(2, packet))
        if "final" in spec:
            send(peer, spec["final"])
            assert not peer.rfile.read(1)
            return
    elif venue == "reya":
        send(peer, {"type": "subscribed", "channel": sub["channel"]})
        send(peer, {"type": "pong"})
        send(peer, {"type": "channel_data", "channel": "/v2/spotMarkets/summary", "data": []})
        for event in spec["events"]:
            send(peer, event)
    else:
        send(peer, {"success": True, "request": sub})
        send(peer, {"op": "ping", "args": [str(time.time_ns() // 1_000_000)]})
        pong = socket_json(peer)
        assert pong["op"] == "pong" and abs(int(pong["args"][0]) - time.time_ns() // 1_000_000) < 10000
        send(peer, {"topic": "instrumentInfo.H.ETHUSDT", "data": []})
        for event in spec["events"]:
            send(peer, event)

    if spec.get("keep_open"):
        assert not peer.rfile.read(1)
    else:
        peer.connection.sendall(frame(8, b"\x03\xe8"))
        assert read_frame(peer.rfile)[0] == 8


def commands(peer, venue, scenario):
    command = socket_json(peer)
    if venue == "binance_spot":
        assert command["method"] == ("order.cancelReplace" if scenario == "partial" else "order.place")
        check_binance_signature(command["params"])
        if scenario == "lost":
            with LOCK:
                LOST["binance_spot"] += 1
            return
        if scenario == "late":
            time.sleep(0.15)
        if scenario == "partial":
            send(peer, {"id": command["id"], "status": 409, "error": {"code": -2021, "data": {
                "cancelResult": "SUCCESS", "newOrderResult": "FAILURE",
            }}})
        else:
            send(peer, {"id": command["id"], "status": 200, "result": {"orderId": 9007199254740993}})
        return
    if venue == "bybit":
        expiry = command["args"][1]
        assert command == {"op": "auth", "args": ["fixture-key", expiry, mac(b"fixture-secret", f"GET/realtime{expiry}".encode())]}
        send(peer, {"op": "auth", "retCode": 0})
        order = socket_json(peer)
        assert order["op"] == "order.create-batch" and order["args"][0]["category"] == "linear"
        send(peer, {"op": "pong"})
        send(peer, {"reqId": order["reqId"], "op": order["op"], "retCode": 0,
                    "data": {"list": [{"orderId": "supported"}, {"orderId": ""}]},
                    "retExtInfo": {"list": [{"code": 0}, {"code": 10001}]}})
    elif venue == "deribit":
        assert command["method"] == "public/auth"
        assert command["params"] == {"grant_type":"client_credentials","client_id":"fixture-key","client_secret":"fixture-secret"}
        send(peer, {"jsonrpc":"2.0","id":command["id"],"result":{"access_token":"fixture-token"}})
        assert socket_json(peer)["method"] == "public/set_heartbeat"
        order = socket_json(peer)
        assert order["method"] == "private/buy" and order["params"]["instrument_name"] == "ETH-PERPETUAL"
        send(peer, {"jsonrpc":"2.0","id":order["id"],"result":{"order":{"order_id":"test"}}})
    elif venue == "okx":
        assert command["op"] == "login"
        auth = command["args"][0]
        assert auth["apiKey"] == "fixture-key" and auth["passphrase"] == "fixture-passphrase"
        message = (auth["timestamp"] + "GET/users/self/verify").encode()
        assert auth["sign"] == base64.b64encode(hmac.new(b"fixture-secret", message, "sha256").digest()).decode()
        send(peer, {"event":"login","code":"0"})
        order = socket_json(peer)
        assert order["op"] == "batch-orders" and order["args"][0]["instId"] == "ETH-USDT"
        send(peer, {"id":order["id"],"op":order["op"],"code":"2","data":[{"sCode":"0","ordId":"test"},{"sCode":"51000"}]})
    elif venue == "kraken_spot":
        assert command["method"] == "add_order" and command["params"]["token"] == "fixture-token"
        send(peer, {"method":"add_order","req_id":command["req_id"],"success":True,"result":{"order_id":"test"}})
    elif venue == "aevo":
        assert command["op"] == "auth" and command["data"] == {"key":"fixture-key","secret":"fixture-secret"}
        send(peer, {"id":command["id"],"op":"auth","success":True})
        order = socket_json(peer)
        assert order["op"] == "create_order" and order["data"]["maker"] == "0x" + "11" * 20
        assert len(order["data"]["signature"]) == 132
        send(peer, {"id":order["id"],"op":order["op"],"data":{"order_id":"test"}})
    else:
        assert venue == "hibachi" and command["method"] == "order.place"
        assert peer.headers["Authorization"] == "fixture-key"
        assert parse_qs(urlsplit(peer.path).query) == {"accountId":["123"]}
        order = command["params"]
        data = (order["nonce"].to_bytes(8, "big") + (1).to_bytes(4, "big") +
                (10_000_000).to_bytes(8, "big") + (1).to_bytes(4, "big") +
                (429_496_729).to_bytes(8, "big") + (45_000).to_bytes(8, "big"))
        assert command["signature"] == mac(b"fixture-secret", data)
        send(peer, {"id":command["id"],"status":200,"result":{"orderId":9007199254740993}})


def user_stream(peer):
    command = socket_json(peer)
    assert command["method"] == "userDataStream.subscribe.signature"
    check_binance_signature(command["params"])
    send(peer, {"id": command["id"], "status": 200, "result": {"subscriptionId": 0}})
    send(peer, {"subscriptionId": 0, "event": {"e": "executionReport", "x": "NEW", "i": 2}})
    send(peer, {"subscriptionId": 0, "event": {"e": "executionReport", "x": "TRADE", "i": 9007199254740993}})
    try:
        read_frame(peer.rfile)
    except EOFError:
        pass


def protobuf(buffer):
    """Decode the subset of protobuf wire types used by a Cosmos transaction."""
    it = iter(buffer)

    def integer(first):
        value = 0
        for shift in range(0, 70, 7):
            value |= (first & 127) << shift
            if first < 128:
                return value
            first = next(it)
        raise AssertionError("oversized varint")

    result = {}
    for first in it:
        tag = integer(first)
        number, kind = tag >> 3, tag & 7
        if kind == 0:
            value = integer(next(it))
        elif kind == 2:
            value = bytes(next(it) for _ in range(integer(next(it))))
        elif kind == 5:
            value = int.from_bytes(bytes(next(it) for _ in range(4)), "little")
        else:
            raise AssertionError(f"unexpected protobuf wire type {kind}")
        result.setdefault(number, []).append(value)
    return result


def verify_dydx_transaction(transaction):
    def field(number, value):
        assert len(value) < 128
        return bytes([number << 3 | 2, len(value)]) + value

    tx = protobuf(transaction)
    body = protobuf(tx[1][0])
    assert len(body[1]) == 1
    message = protobuf(body[1][0])
    assert message[1] == [b"/dydxprotocol.clob.MsgPlaceOrder"]
    order = protobuf(protobuf(message[2][0])[1][0])
    order_id = protobuf(order[1][0])
    assert protobuf(order_id[1][0])[1] == [ACCOUNT.encode()]
    assert order_id[2] == [42] and order[2] == [1]
    assert order[3] == [1000] and order[4] == [10000]
    if 3 in order_id:
        assert order_id[3] == [64] and order[6] == [4102444800] and body[3] == [120]
    else:
        assert order[5] == [120] and 3 not in body
    auth = protobuf(tx[2][0])
    signer = protobuf(auth[1][0])
    assert signer[3] == [9]
    public = protobuf(signer[1][0])
    assert public[1] == [b"/cosmos.crypto.secp256k1.PubKey"]
    key = protobuf(public[2][0])[1][0]
    assert key.hex() == "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"
    msg = field(1, tx[1][0]) + field(2, tx[2][0]) + field(3, b"dydx-mainnet-1") + bytes([0x20, 7])
    subprocess.run(["node", "-e", """
const c = require("node:crypto");
const [key, msg, sig] = process.argv.slice(1).map(x => Buffer.from(x, "hex"));
const spki = Buffer.concat([Buffer.from("3036301006072a8648ce3d020106052b8104000a032200", "hex"), key]);
const publicKey = c.createPublicKey({key: spki, format: "der", type: "spki"});
if (!c.verify("sha256", msg, {key: publicKey, dsaEncoding: "ieee-p1363"}, sig)) process.exit(1);
""", key.hex(), msg.hex(), tx[3][0].hex()], check=True)


def dydx(peer, path, raw, scenario):
    if path == "/cosmos/base/tendermint/v1beta1/blocks/latest":
        chain = "unrecognised" if scenario == "wrong-chain" else "dydx-mainnet-1"
        return respond(peer, {"block": {"header": {"chain_id": chain, "height": "100"}}})
    if path.startswith("/cosmos/auth/v1beta1/accounts/"):
        assert path.endswith("/" + ACCOUNT)
        return respond(peer, {"account": {"address": ACCOUNT, "account_number": "7", "sequence": "9"}})
    if path.startswith(NODE + "/"):
        assert len(path.split("/")[-1]) == 64
        return respond(peer, {"code": 5, "message": "not found"}, 404)
    assert path == NODE and peer.command == "POST"
    payload = json.loads(raw)
    assert payload["mode"] == "BROADCAST_MODE_SYNC"
    transaction = base64.b64decode(payload["tx_bytes"], validate=True)
    verify_dydx_transaction(transaction)
    txhash = hashlib.sha256(transaction).hexdigest().upper()
    if scenario == "lost":
        with LOCK:
            LOST["dydx_v4"] += 1
        return
    return respond(peer, {"tx_response": {"txhash": txhash, "code": 0, "height": "0"}})


def authenticated_rest(peer, venue, path, query_string, query, raw):
    secret = b"fixture-secret"
    if venue == "aevo" and path.startswith("/orders"):
        assert peer.headers["AEVO-KEY"] == "fixture-key" and peer.headers["AEVO-SECRET"] == "fixture-secret"
        order = json.loads(raw)
        if peer.command == "DELETE":
            assert order == {"order_ids": ["old-order", "other-order"]}
            return {"success": True, "order_ids": ["old-order"]}
        assert peer.command == "POST" and path in ("/orders", "/orders/old-order")
        assert order["maker"] == "0x" + "11" * 20 and len(order["signature"]) == 132
        return {"order_id": "fixture-order"}
    if venue == "backpack":
        assert path == "/api/v1/capital" and not query
        stamp = peer.headers["X-Timestamp"]
        window = peer.headers["X-Window"]
        assert base64.b64decode(peer.headers["X-API-Key"]).hex() == ED_PUBLIC
        message = f"instruction=balanceQuery&timestamp={stamp}&window={window}"
        verify_ed25519(ED_PUBLIC, message, base64.b64decode(peer.headers["X-Signature"]).hex())
        return {"USDC": {"available": "1.23"}}
    if venue == "bybit":
        assert path == "/v5/order/create" and not query
        stamp = peer.headers["X-BAPI-TIMESTAMP"]
        key = peer.headers["X-BAPI-API-KEY"]
        window = peer.headers["X-BAPI-RECV-WINDOW"]
        assert key == "fixture-key" and window == "5000"
        assert peer.headers["X-BAPI-SIGN"] == mac(secret, (stamp + key + window).encode() + raw)
        return {"retCode": 0, "retMsg": "OK", "result": {"orderId": "fixture-order"}}
    if venue == "deribit":
        if path == "/api/v2/public/auth":
            assert query == {"grant_type": ["client_credentials"], "client_id": ["fixture-key"], "client_secret": ["fixture-secret"]}
            return {"jsonrpc": "2.0", "result": {"access_token": "fixture-token", "expires_in": 900}}
        assert path == "/api/v2/private/get_account_summary"
        assert peer.headers["Authorization"] == "Bearer fixture-token" and query == {"currency": ["ETH"]}
        return {"jsonrpc": "2.0", "result": {"equity": 1}}
    if venue == "hibachi":
        if path == "/market/exchange-info":
            assert peer.headers.get("Authorization") is None
            return {"futureContracts": [{"id": 1, "symbol": "ETH/USDT-P", "underlyingDecimals": 9, "settlementDecimals": 6}]}
        assert path == "/trade/order" and peer.headers["Authorization"] == "fixture-key"
        order = json.loads(raw)
        assert order["accountId"] == 123 and order["side"] == "BID"
        data = (order["nonce"].to_bytes(8, "big") + (1).to_bytes(4, "big") +
                (10_000_000).to_bytes(8, "big") + (1).to_bytes(4, "big") +
                (429_496_729).to_bytes(8, "big") + (45_000).to_bytes(8, "big"))
        signature = bytes.fromhex(order["signature"])
        if len(signature) == 65:
            assert signature[64] in (0, 1)
            subprocess.run(["node", "-e", """
const c = require("node:crypto");
const [msg, sig] = process.argv.slice(1).map(x => Buffer.from(x, "hex"));
const key = c.createPublicKey({key: Buffer.from("3036301006072a8648ce3d020106052b8104000a0322000279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", "hex"), format: "der", type: "spki"});
if (!c.verify("sha256", msg, {key, dsaEncoding: "ieee-p1363"}, sig)) process.exit(1);
""", data.hex(), signature[:64].hex()], check=True)
        else:
            assert order["signature"] == mac(secret, data)
        return {"orderId": 9007199254740993}
    if venue == "okx":
        assert path == "/api/v5/account/balance" and query == {"ccy": ["USDT"]}
        stamp = peer.headers["OK-ACCESS-TIMESTAMP"]
        assert peer.headers["x-simulated-trading"] == "1"
        assert peer.headers["OK-ACCESS-KEY"] == "fixture-key" and peer.headers["OK-ACCESS-PASSPHRASE"] == "fixture-passphrase"
        message = (stamp + peer.command + path + "?" + query_string).encode()
        assert peer.headers["OK-ACCESS-SIGN"] == base64.b64encode(hmac.new(secret, message, "sha256").digest()).decode()
        return {"code": "0", "data": [{"totalEq": "1.23"}]}
    assert venue == "orderly" and path == "/v1/client/holding"
    assert peer.headers["orderly-account-id"] == "fixture-account"
    stamp = peer.headers["orderly-timestamp"]
    message = stamp + peer.command + path + ("?" + query_string if query_string else "")
    signature = peer.headers["orderly-signature"]
    verify_ed25519(ED_PUBLIC, message, base64.urlsafe_b64decode(signature + "==").hex())
    return {"success": True, "data": {"holding": []}}


def rest(peer, venue, path, query_string, scenario):
    raw = peer.rfile.read(int(peer.headers.get("Content-Length", 0)))
    query = parse_qs(query_string, keep_blank_values=True)
    if venue == "dydx_v4" and path.startswith("/cosmos/"):
        return dydx(peer, path, raw, scenario)
    if venue == "binance_spot" and path == "/api/v3/order":
        canonical, signature = query_string.rsplit("&signature=", 1)
        assert peer.headers["X-MBX-APIKEY"] == "fixture-key"
        assert signature == mac(b"fixture-secret", canonical.encode())
        assert query["symbol"] == ["ETHUSDT"] and query["timestamp"][0].isdigit()
        if scenario == "lost":
            with LOCK:
                LOST["binance_spot"] += 1
            return
        return respond(peer, {"orderId": 9007199254740993, "status": "NEW"})
    if venue == "kraken_spot" and path.startswith("/0/private/"):
        assert peer.headers["API-Key"] == "fixture-key"
        body = json.loads(raw) if peer.headers["Content-Type"] == "application/json" else parse_qs(raw.decode())
        nonce = body["nonce"]
        if isinstance(nonce, list):
            nonce = nonce[0]
        digest = hashlib.sha256(str(nonce).encode() + raw).digest()
        expected = base64.b64encode(hmac.new(b"fixture-secret", path.encode() + digest, "sha512").digest()).decode()
        assert not query and peer.headers["API-Sign"] == expected
        with LOCK:
            account = (peer.path.split("/")[4], peer.headers["API-Key"])
            assert int(nonce) > LAST_NONCE.get(account, 0)
            LAST_NONCE[account] = int(nonce)
        if path.endswith("/GetWebSocketsToken"):
            return respond(peer, {"error": [], "result": {"token": "fixture-token", "expires": 900}})
        return respond(peer, {"error": [], "result": {"fixture": True}})
    if scenario == "private":
        return respond(peer, authenticated_rest(peer, venue, path, query_string, query, raw))
    choices = [r for r in HTTP if r["venue"] == venue and r["path"] == path and r["verb"] == peer.command]
    row = next(r for r in choices if not r["private"])
    expected_query = {key: [value] for key, value in row["query"].items()}
    if venue == "backpack" and path == "/api/v1/depth":
        if "limit" not in query:
            del expected_query["limit"]
        assert all(header not in peer.headers for header in ("X-Signature", "X-Timestamp", "X-Window"))
    assert query == expected_query
    for header in ("Authorization", "X-MBX-APIKEY", "orderly-key", "X-BAPI-API-KEY", "AEVO-KEY", "OK-ACCESS-KEY", "X-Api-Key"):
        assert peer.headers.get(header) is None
    if path == "/v1/public/query":
        body = json.loads(raw)
        name = {"marketSummary": "market_summary", "marketTrades": "market_trades"}[body["type"]]
        row = next(r for r in choices if r["operation"] == name)
        assert body == row["body"]
    else:
        assert not raw
    return respond(peer, row["response"])


def serve(peer):
    url = urlsplit(peer.path)
    _, _, venue, mode, scenario, *parts = url.path.split("/")
    if mode == "count":
        with LOCK:
            return respond(peer, {"count": LOST[venue]})
    if mode == "ws":
        upgrade(peer)
        if (venue, scenario) in MARKET_STREAMS or (venue == "kuru" and scenario in ("epoch-mismatch", "truncated")):
            return market_stream(peer, venue, scenario)
        if scenario == "book":
            return books(peer, venue)
        if scenario == "fills":
            return user_stream(peer)
        return commands(peer, venue, scenario)
    assert mode == "rest"
    return rest(peer, venue, "/" + "/".join(parts), url.query, scenario)
