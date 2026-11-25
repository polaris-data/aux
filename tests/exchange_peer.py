"""Wire checks for the Perpl, RISEx, Decibel, EdgeX, GRVT, Katana and Variational clients."""

import base64
import hashlib
import hmac
import json
import io
from collections import Counter
from pathlib import Path
import socket
import subprocess
from urllib.parse import parse_qs, urlsplit
import uuid
from threading import Lock

from .trading_peer import upgrade, verify_ed25519
from .ws_server import frame, read_frame

VENUES = {"perpl", "risex", "decibel", "edgex", "grvt", "katana", "variational"}
ROWS = [
    row for row in json.loads(Path(__file__).with_name("perps-http.json").read_text())
    if row["venue"] in VENUES
]
ED_PUBLIC = "03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8"
WALLET = "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf"
MARKET = "0x" + "33" * 32
STATE_LOCK = Lock()
LOST = Counter()
RISE_PERMITS = set()


def respond(peer, value, status=200, headers=()):
    raw = json.dumps(value, separators=(",", ":")).encode()
    peer.send_response(status)
    for name, value in headers:
        peer.send_header(name, value)
    peer.send_header("Content-Type", "application/json")
    peer.send_header("Content-Length", str(len(raw)))
    peer.end_headers()
    peer.wfile.write(raw)


def verify_aptos(raw):
    domain = hashlib.sha3_256(b"APTOS::RawTransaction").digest()
    assert raw[40:43] == bytes([4, 0, 1]) and raw[-99:-97] == bytes([0, 32])
    if b"place_order_to_subaccount" in raw:
        tx = io.BytesIO(raw)
        tx.seek(75)  # Sender, sequence, payload tags and package address.
        vector = lambda: tx.read(tx.read(1)[0])  # All fixture vectors fit in one ULEB128 byte.
        assert vector() == b"dex_accounts_entry"
        assert vector() == b"place_order_to_subaccount"
        assert tx.read(2) == bytes([0, 16])
        args = [vector() for _ in range(16)]
        assert args[:2] == [bytes.fromhex("22" * 32), bytes.fromhex("33" * 32)]
        assert int.from_bytes(args[2], "little") == 5000010
        assert int.from_bytes(args[3], "little") == 1000
        assert args[4:8] == [b"\x01", b"\x02", b"\x00", b"\x01\x09fixture-1"]
        assert args[8:] == [b"\x00"] * 8
        assert tx.read(3) == bytes([0, 0, 1])
    pk, sig = raw[-97:-65], raw[-64:]
    assert pk.hex() == ED_PUBLIC and raw[:32] == hashlib.sha3_256(pk + b"\0").digest()
    subprocess.run(
        ["node", "-e",
         """const c=require("node:crypto");const [pk,msg,sig]=process.argv.slice(1);
         const key=c.createPublicKey({key:Buffer.from("302a300506032b6570032100"+pk,"hex"),type:"spki",format:"der"});
         if(!c.verify(null,Buffer.from(msg,"hex"),key,Buffer.from(sig,"hex"))) process.exit(1);""",
         pk.hex(), (domain + raw[:-99]).hex(), sig.hex()],
        check=True,
    )
    salt = hashlib.sha3_256(b"APTOS::Transaction").digest()
    return "0x" + hashlib.sha3_256(salt + b"\0" + raw).hexdigest()


def flatten(value):
    if isinstance(value, dict):
        return "&".join(f"{key}={flatten(value[key])}" for key in sorted(value))
    if isinstance(value, list):
        return "&".join(map(flatten, value))
    if isinstance(value, bool):
        return str(value).lower()
    return "" if value is None else str(value)


def check_edge(peer, path, query, body):
    timestamp = peer.headers["X-edgeX-Timestamp"]
    canonical = "&".join(sorted(f"{key}={value}" for key, values in query.items() for value in values)) if query else flatten(body)
    signed = (timestamp + peer.command + path + canonical).encode()
    assert peer.headers["X-edgeX-Signature"] == hmac.new(base64.b64encode(b"fixture-secret"), signed, "sha256").hexdigest()
    assert peer.headers["X-edgeX-Api-Key"] == "fixture-key"
    assert peer.headers["X-edgeX-Passphrase"] == "fixture-passphrase"


def check_perpl(peer, path, query, raw):
    stamp = peer.headers["X-API-Timestamp"]
    nonce = peer.headers["X-API-Nonce"]
    target = path.removeprefix("/api") + ("?" + query if query else "")
    message = "\n".join(["143", peer.command, target, stamp, nonce, hashlib.sha256(raw).hexdigest()])
    assert peer.headers["X-API-Key"] == "fixture-key"
    verify_ed25519(ED_PUBLIC, message, base64.urlsafe_b64decode(peer.headers["X-API-Signature"] + "==").hex())


def verify_rest(peer, venue, path, raw_query, raw, body, row):
    query = parse_qs(raw_query, keep_blank_values=True)
    if venue == "decibel":
        assert peer.headers["Authorization"] == "Bearer fixture-key"
    elif venue == "perpl" and row["private"]:
        check_perpl(peer, path, raw_query, raw)
    elif venue == "edgex" and row["private"]:
        check_edge(peer, path, query, body)
    elif venue == "grvt" and row["private"]:
        if row["operation"] == "login":
            assert body == {"api_key": "fixture-key"}
            return
        assert peer.headers["Cookie"] == "gravity=fixture" and peer.headers["X-Grvt-Account-Id"] == "123"
    elif venue == "katana" and row["private"]:
        stamp = query.pop("nonce")[0] if peer.command == "GET" else body["parameters"]["nonce"]
        assert uuid.UUID(stamp).version == 1
        canonical = raw_query.encode() if peer.command == "GET" else raw
        assert peer.headers["KP-HMAC-SIGNATURE"] == hmac.new(b"fixture-secret", canonical, "sha256").hexdigest()
        assert peer.headers["KP-API-KEY"] == "fixture-key"
    expected = {key: value if isinstance(value, list) else [value] for key, value in row["query"].items()}
    if venue == "katana" and "wallet" in expected:
        expected["wallet"] = [WALLET]
    if peer.command == "GET" and path != "/v1/orderbook":
        assert query == expected or (row["operation"] in {"nonce_state", "ws_token", "order_status"} and set(query) <= set(expected))
    if row["operation"] not in {"place_order", "cancel_order"} and peer.command == "POST":
        assert body == row["body"]


def serve(peer):
    url = urlsplit(peer.path)
    _, _, venue, route = url.path.split("/", 3)
    path = "/" + route
    if route.startswith("ws/"):
        return serve_ws(peer, venue, route[3:], parse_qs(url.query))
    if venue == "variational" and path == "/rate-limited/metadata/v2/stats":
        return respond(peer, {"retry_after_seconds": 4}, 429)
    if venue == "decibel" and path == "/lost-count":
        with STATE_LOCK:
            count = LOST[peer.server.server_address]
        return respond(peer, {"count": count})
    drop = peer.command == "POST" and path.startswith("/lost/")
    path = path.removeprefix("/lost") if path.startswith("/lost/") else path
    raw = peer.rfile.read(int(peer.headers.get("Content-Length", "0")))
    binary = venue == "decibel" and peer.command == "POST"
    body = json.loads(raw) if raw and not binary else None
    candidates = [r for r in ROWS if r["venue"] == venue and r["verb"] == peer.command and
                  (r["path"] == path or (r["operation"] == "nonce_state" and path.startswith("/v1/nonce-state/")))]
    if venue == "perpl" and len(candidates) == 2:
        candidates = [r for r in candidates if (r["operation"] == "cancel_order") == (body["d"][0]["t"] == 5)]
    if binary:
        candidates = [r for r in candidates if (b"cancel_order_to_subaccount" in raw) == (r["operation"] == "cancel_order")]
    if len(candidates) > 1:
        supplied = set(parse_qs(url.query)) - {"nonce"}
        candidates.sort(key=lambda r: len(supplied.symmetric_difference(r["query"])))
    row = candidates[0]
    verify_rest(peer, venue, path, url.query, raw, body, row)
    op = row["operation"]
    if venue == "grvt" and op == "login":
        value = {k: v for k, v in row["response"].items() if k not in {"cookie", "account_id"}}
        return respond(peer, value, headers=[("Set-Cookie", "other=fixture; Path=/"), ("Set-Cookie", "gravity=fixture; HttpOnly; Path=/"), ("X-Grvt-Account-Id", "123")])
    if venue == "risex" and op in {"place_order", "cancel_order"}:
        assert len(base64.b64decode(body["permit"]["signature"], validate=True)) == 65
        assert body["permit"]["signer"].lower() == WALLET
        assert body["permit"]["nonce_bitmap_index"] < 208
        nonce = (peer.server.server_address, body["permit"]["nonce_anchor"], body["permit"]["nonce_bitmap_index"])
        with STATE_LOCK:
            assert nonce not in RISE_PERMITS
            RISE_PERMITS.add(nonce)
        assert "resting_order_id" not in body
    if venue == "perpl" and op in {"place_order", "cancel_order"}:
        assert len(body["d"]) == 1 and body["d"][0]["rq"] > 2**53
        if body["d"][0]["rq"] == 9007199254740995:
            return respond(peer, {"mt": 31, "status": {"code": 0}, "statuses": [{"code": 403}]})
    if venue == "edgex":
        if op == "markets":
            return respond(peer, {"code": "SUCCESS", "data": {
                "global": {"nativeChainId": "3343", "contractAddress": "0x" + "44"*20},
                "contractList": [{"contractId":"10000001", "quoteCoinId":"1000", "tickSize":"0.1",
                                  "stepSize":"0.001", "resolution":"10000000000",
                                  "defaultMakerFeeRate":"0.0004", "defaultTakerFeeRate":"0.00045"}],
                "coinList": [{"coinId":"1000", "resolution":"0xf4240"}],
            }})
        if op == "place_order":
            assert len(bytes.fromhex(body["l2Signature"])) == 65 and body["l2Value"] == "97.3938"
    if venue == "grvt" and op == "place_order":
        order = body["order"]
        assert order["signature"]["signer"].lower() == WALLET
        assert order["signature"]["chain_id"] == "325"
    if venue == "katana" and op in {"place_order", "cancel_order"}:
        assert body["parameters"]["wallet"].lower() == WALLET
        assert len(bytes.fromhex(body["signature"][2:])) == 65
        if op == "cancel_order":
            assert body["parameters"]["orderIds"] == [row["body"]["orderId"]]
    if binary:
        assert peer.headers["Content-Type"] == "application/x.aptos.signed_transaction+bcs"
        digest = verify_aptos(raw)
        if not drop:
            return respond(peer, {**row["response"], "hash": digest})
    if drop:
        with STATE_LOCK:
            LOST[peer.server.server_address] += 1
        peer.connection.shutdown(socket.SHUT_RDWR)
        return
    return respond(peer, row["response"])


def upgrade_decibel(peer):
    assert peer.headers["Sec-WebSocket-Protocol"] == "decibel, fixture-key"
    sec = peer.headers["Sec-WebSocket-Key"]
    accept = base64.b64encode(hashlib.sha1((sec + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").encode()).digest())
    peer.connection.sendall(b"HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade"
                            b"\r\nSec-WebSocket-Protocol: decibel\r\nSec-WebSocket-Accept: " + accept + b"\r\n\r\n")
    peer.close_connection = True
    peer.connection.settimeout(5)


def serve_ws(peer, venue, name, query):
    if venue == "edgex" and name == "account":
        assert set(query) == {"accountId", "timestamp"}
        check_edge(peer, "/new-exchanges/edgex/ws/account", query, None)
    if venue == "grvt" and name == "account":
        assert peer.headers["Cookie"] == "gravity=fixture" and peer.headers["X-Grvt-Account-Id"] == "123"
    (upgrade_decibel if venue == "decibel" else upgrade)(peer)

    def send(*messages):
        raw = b"".join(json.dumps(item, separators=(",", ":")).encode() for item in messages)
        peer.connection.sendall(frame(1, raw))

    def receive():
        opcode, data = read_frame(peer.rfile)
        assert opcode == 1
        return json.loads(data)

    if venue == "perpl":
        sub = receive()
        if name in {"account", "account-denied"}:
            assert sub["mt"] == 29 and sub["chain_id"] == 143 and sub["api_key"] == "fixture-key"
            proof = "\n".join(["143", "trading-ws-signin", sub["timestamp"], sub["nonce"]])
            verify_ed25519(ED_PUBLIC, proof, base64.urlsafe_b64decode(sub["signature"] + "==").hex())
            if name == "account-denied":
                send({"mt":3,"status":{"code":401,"error":"API key revoked"}})
            else:
                send({"mt":19, "sn":10, "addr":WALLET})
            return
        stream = "order-book@1" if name in {"book", "reject"} else "trades@1"
        assert sub == {"mt":5, "subs":[{"stream":stream,"subscribe":True}]}
        if name == "reject":
            send({"mt":6, "subs":[{"stream":stream,"status":{"code":404}}]})
            return
        send({"mt":6, "subs":[{"stream":stream,"sid":42}]})
        send({"mt":16 if name == "book" else 18,"sid":11})
        if name == "book":
            send({"mt":16,"sid":42,"bid":[],"ask":[]})
            send({"mt":15,"sid":42,"bid":[{"p":9007199254740993,"s":1,"o":1}],"ask":[]})
        else:
            send({"mt":18,"sid":42,"d":[]})
    elif venue == "risex":
        if name == "account":
            sign_in = receive()
            assert sign_in["method"] == "auth_v2"
            params = sign_in["params"]
            assert params["account"] == "0x" + "11" * 20 and params["signer"].lower() == WALLET
            send({"method":"auth_v2","status":"success","data":{"account":params["account"]}})
            assert receive() == {"method":"subscribe","params":{"channel":"account"}}
            send({"channel":"account","type":"snapshot","data":{"account":params["account"]}})
            return
        channel = "orderbook" if name == "book" else "trades"
        assert receive() == {"method":"subscribe","params":{"channel":channel,"market_ids":[1]}}
        event = {"channel":channel,"market_id":"1","type":"update","data":{"market_id":1}}
        send({**event,"market_id":"2"})
        if name == "book":
            send(event)
            event["type"] = "snapshot"
        send(event)
    elif venue == "decibel":
        sub = receive()
        assert sub["method"] == "subscribe"
        topic = sub["topic"]
        send({"success":True,"method":"subscribe","topic":topic})
        send({"topic":topic + ":wrong"})
        send({"topic":topic,"unix_ms":9007199254740993})
    elif venue == "edgex":
        send({"type":"connected"})
        send({"type":"ping","time":"123"})
        if name == "account":
            assert receive() == {"type":"pong","time":"123"}
            send({"type":"trade-event","content":{"event":"Snapshot","data":{"order":[]}}})
            return
        channel = "depth.10000001.200" if name == "book" else "trades.10000001"
        assert receive() == {"type":"subscribe","channel":channel}
        assert receive() == {"type":"pong","time":"123"}
        event = {"type":"quote-event","channel":channel,"content":{"dataType":"Snapshot","data":[{"contractId":10000001}]}}
        send({**event,"channel":"other"})
        send(event)
    elif venue == "grvt":
        sub = receive()
        stream = "v1.order" if name == "account" else "v1.book.s" if name == "book" else "v1.trade"
        selector = "123456789" if name == "account" else "BTC_USDT_Perp@500-50" if name == "book" else "BTC_USDT_Perp@500"
        assert sub == {"jsonrpc":"2.0","method":"subscribe","params":{"stream":stream,"selectors":[selector]},"id":1}
        send({"jsonrpc":"2.0","id":1,"result":{"stream":stream}})
        send({"stream":stream,"selector":"other","feed":{}})
        send({"stream":stream,"selector":selector,"sequence_number":"0","feed":{"instrument":"BTC_USDT_Perp"}})
    elif venue == "katana":
        sub = receive()
        channel = "l2orderbook" if name == "book" else "trades" if name == "trades" else "orders"
        if name == "account":
            assert sub == {"method":"subscribe","token":"fixture-token","subscriptions":["orders"]}
            send({"type":channel,"data":{"w":WALLET}})
        else:
            assert sub == {"method":"subscribe","markets":["ETH-USD"],"subscriptions":[channel]}
            ack = {"type":"subscriptions","subscriptions":[{"name":channel,"markets":["ETH-USD"]}]}
            old = {"type":channel,"data":{"m":"ETH-USD","u":9007199254740993}}
            new = {"type":channel,"data":{"m":"ETH-USD","u":9007199254740994}}
            send(ack, {**old,"data":{"m":"OTHER","u":100}}, old, new)
