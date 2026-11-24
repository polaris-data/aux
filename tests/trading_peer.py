"""Independent wire checks for private Monaco reads and QFEX trade sessions."""

import base64
import hashlib
import hmac
import json
from pathlib import Path
import subprocess
import time
from urllib.parse import parse_qs, urlsplit

from .ws_server import frame, read_frame

HERE = Path(__file__).parent
MONACO = [
    r
    for r in json.loads((HERE / "data-http.json").read_text())
    if r["venue"] == "monaco"
]


def monaco(peer):
    url = urlsplit(peer.path)
    row = next(r for r in MONACO if r["path"] == url.path and r["verb"] == peer.command)
    assert parse_qs(url.query) == {k: [v] for k, v in row["query"].items()}
    payload = peer.rfile.read(int(peer.headers.get("Content-Length", 0)))
    sent = json.loads(payload) if payload else None
    if row["operation"] in ("create_challenge", "verify_signature") and sent.get(
        "clientId"
    ) in ("login", "unbound", "wallet-mismatch", "named-login"):
        public = "03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8"
        named = sent["clientId"] == "named-login"
        wallet = "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf" if named else "0x" + "21" * 20
        assert (
            sent["address"] == wallet
            and sent["sessionPublicKey"] == public
            and sent["chainId"] == "1329"
        )
        assert "seed" not in sent and "options" not in sent
        if row["operation"] == "create_challenge":
            reply = {
                "message": "Authorize "
                + (public if sent["clientId"] != "unbound" else "other key"),
                "nonce": "challenge",
                "expiresAt": 4102444800,
            }
        else:
            # EIP-191 vector from Foundry cast, signing the literal challenge above.
            expected = "0x9091f7524476548410b4b467060a7ed72ba720636ef5f19b27cefb065d6065261575bc3e3e0de144043592d5d2406c35d538dccd8dc5d2e1cd8d10d357dd1f3e1c" if named else "0xabcd"
            assert sent["nonce"] == "challenge" and sent["signature"] == expected
            reply = {
                "user": {
                    "address": wallet
                    if sent["clientId"] != "wallet-mismatch"
                    else "0x" + "00" * 20
                },
                "expiresAt": 4102444800,
            }
        peer.send_response(200)
        peer.end_headers()
        peer.wfile.write(json.dumps(reply).encode())
        return
    lost_ack = (
        row["operation"] == "place_order" and sent.get("clientOrderId") == "lost-ack"
    )
    assert sent == (
        {**row["body"], "clientOrderId": "lost-ack"} if lost_ack else row["body"]
    )
    if row["private"]:
        pk = "03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8"
        if peer.headers.get("Signature"):
            digest = (
                "sha-256=:"
                + base64.b64encode(hashlib.sha256(payload).digest()).decode()
                + ":"
            )
            assert peer.headers["Content-Digest"] == digest
            params = peer.headers["Signature-Input"].removeprefix("monaco=")
            timestamp = int(params.split(";created=")[1].split(";")[0])
            assert (
                params
                == f'("@method" "@path" "@query" "content-digest");created={timestamp};keyid="{pk}";alg="ed25519"'
            )
            message = f'"@method": {peer.command}\n"@path": {url.path}\n"@query": ?{url.query}\n"content-digest": {digest}\n"@signature-params": {params}'
            sig = base64.b64decode(
                peer.headers["Signature"].removeprefix("monaco=:")[:-1]
            ).hex()
        else:
            assert peer.headers["X-Monaco-PublicKey"] == pk
            ms = int(peer.headers["X-Monaco-Timestamp"])
            timestamp = ms // 1000
            message = f"{peer.command}\n{peer.path}\n{ms}\n{hashlib.sha256(payload).hexdigest()}"
            sig = peer.headers["X-Monaco-Signature"]
        assert abs(timestamp - time.time()) < 60
        verify_ed25519(pk, message, sig)
    if lost_ack:
        peer.close_connection = True
        return
    peer.send_response(200)
    peer.end_headers()
    peer.wfile.write(json.dumps(row["response"]).encode())


def verify_ed25519(pk, message, sig):
    subprocess.run(
        [
            "node",
            "-e",
            """const c=require("node:crypto");const [pk,msg,sig]=process.argv.slice(1);
    const key=c.createPublicKey({key:Buffer.from("302a300506032b6570032100"+pk,"hex"),type:"spki",format:"der"});
    if(!c.verify(null,Buffer.from(msg),key,Buffer.from(sig,"hex")))process.exit(1);""",
            pk,
            message,
            sig,
        ],
        check=True,
    )


def upgrade(peer):
    assert peer.headers["upgrade"].lower() == "websocket"
    sec = peer.headers["sec-websocket-key"]
    accept = base64.b64encode(
        hashlib.sha1((sec + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").encode()).digest()
    )
    peer.connection.sendall(
        b"HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: "
        + accept
        + b"\r\n\r\n"
    )
    peer.close_connection = True
    peer.connection.settimeout(5)


def qfex(peer):
    assert peer.path == "/trade?api_key=fixture" and "X-Api-Key" not in peer.headers
    upgrade(peer)

    def recv():
        op, payload = read_frame(peer.rfile)
        assert op == 1
        return json.loads(payload)

    def send(msg):
        peer.connection.sendall(frame(1, json.dumps(msg).encode()))

    auth = recv()
    proof = auth["params"]["hmac"]
    assert auth["type"] == "auth" and proof["public_key"] == "fixture"
    assert (
        len(bytes.fromhex(proof["nonce"])) == 16
        and abs(time.time() - proof["unix_ts"]) < 60
    )
    expected = hmac.digest(
        b"secret", f"{proof['nonce']}:{proof['unix_ts']}".encode(), "sha256"
    ).hex()
    assert hmac.compare_digest(expected, proof["signature"])
    send({"authenticated": True})
    assert recv() == {"type": "subscribe", "params": {"channels": ["order_responses"]}}
    assert recv() == {
        "type": "subscribe",
        "params": {"channels": ["balances", "positions", "fills"]},
    }
    order = {
        "side": "BUY",
        "order_time_in_force": "GTC",
        "symbol": "BTC",
        "quantity": 0.5,
        "price": 123,
        "order_type": "LIMIT",
        "client_order_id": "cid",
        "reduce_only": 0,
    }
    assert recv() == {"type": "add_order", "params": order}
    send({"type": "balance", "total": 9007199254740993})
    ack = {
        "client_order_id": "cid",
        "symbol": "BTC",
        "status": "open",
        "order_id": "abc",
    }
    send({"order_response": ack})
    assert recv() == {
        "type": "cancel_order",
        "params": {
            "symbol": "BTC",
            "order_id": "abc",
            "cancel_order_id_type": "order_id",
        },
    }
    time.sleep(0.4)
    send({"order_response": {**ack, "status": "canceled"}})
    assert recv() == {"type": "get_user_orders", "params": {"limit": 100, "offset": 0}}
    send({"all_orders_response": {"orders": None, "twaps": []}})
    assert peer.rfile.read(1) == b""


def polymarket(peer):
    service = urlsplit(peer.path).path.rsplit("/", 1)[1]
    row = next(
        r
        for r in json.loads((HERE / "polymarket-ws.json").read_text())
        if r["service"] == service
    )
    upgrade(peer)

    def receive(expected):
        op, payload = read_frame(peer.rfile)
        assert op == 1 and json.loads(payload) == expected

    def send(event):
        peer.connection.sendall(frame(1, json.dumps(event).encode()))

    if row["login"]:
        receive(row["login"])
        send(row["ack"])
    if row["subscribe"]:
        receive(row["subscribe"])
    if service == "market":
        send({"event_type": "price_change", "price": "stale"})
    send(row["event"])
    peer.connection.sendall(frame(8, b"\x03\xe8"))


def hyperliquid(peer):
    actual = json.loads(peer.rfile.read(int(peer.headers["Content-Length"])))
    rows = json.loads((HERE / "hyperliquid-signing.json").read_text())["l1"]
    expected = next(r for r in rows if r["signature"] == actual["signature"])
    body = {
        "action": expected["action"],
        "nonce": expected["nonce"],
        "signature": expected["signature"],
    }
    if expected["vault_address"]:
        body["vaultAddress"] = expected["vault_address"].lower()
    if expected["expires_after"]:
        body["expiresAfter"] = expected["expires_after"]
    assert actual == body
    peer.send_response(200)
    peer.end_headers()
    peer.wfile.write(
        b'{"status":"ok","response":{"type":"order","data":{"statuses":["success",{"error":"fixture rejection"}]}}}'
    )


def pyth(peer):
    assert peer.headers["Authorization"] == "Bearer fixture"
    upgrade(peer)

    def send(obj):
        peer.connection.sendall(frame(1, json.dumps(obj).encode()))

    if peer.path == "/v1/merkle/root/stream":
        send({"root": "abc"})
    else:
        op, raw = read_frame(peer.rfile)
        assert op == 1 and json.loads(raw) == {
            "type": "subscribe",
            "channel": "real_time",
            "symbols": ["BTC/USD"],
            "formats": ["evm"],
            "subscriptionId": 42,
        }
        event = {
            "type": "streamUpdated",
            "subscriptionId": 42,
            "price": 9007199254740993,
        }
        if "/reconnect-" in peer.path:
            # An abrupt first close must replay this source's original subscription.
            sent = peer.server.__dict__.setdefault("pyth_disconnects", set())
            send(event)
            if peer.path not in sent:
                sent.add(peer.path)
                return
            send({**event, "price": 10})
        elif peer.path.endswith("/a"):
            send({**event, "subscriptionId": 41})
            send(event)
        else:
            send(dict(reversed(list(event.items()))))
            parsed = (3302625434).to_bytes(
                4, "little"
            ) + b'{"price":"1.234567890123456789"}'
            evm = (2593727018).to_bytes(4, "little") + b"abc"
            data = (
                (461928307).to_bytes(4, "little")
                + (42).to_bytes(8, "big")
                + b"".join(len(x).to_bytes(2, "big") + x for x in (parsed, evm))
            )
            peer.connection.sendall(frame(2, data))
    peer.connection.sendall(frame(8, b"\x03\xe8"))


def perps_subscription(peer):
    venue, name = peer.path.split("/")[-2:]
    rows = json.loads((HERE / "perps-ws.json").read_text())
    row = next(r for r in rows if (r["venue"], r["name"]) == (venue, name))
    upgrade(peer)
    opcode, raw = read_frame(peer.rfile)
    assert opcode == 1
    request = json.loads(raw)
    if name == "account_all_orders":
        deadline, account, key, signature = request.pop("auth").split(":")
        assert (account, key) == ("37", "3") and len(bytes.fromhex(signature)) == 80
        assert 590 <= int(deadline) - time.time() <= 600
    assert request == row["frame"]

    def send(value):
        peer.connection.sendall(frame(1, json.dumps(value).encode()))

    if venue == "hyperliquid":
        send({"channel": "pong"})
        send(
            {
                "channel": "subscriptionResponse",
                "data": {
                    "method": "subscribe",
                    "subscription": {"type": "l2Book", "coin": "OTHER"},
                },
            }
        )
    else:
        send({"type": "connected"})
        send({"type": "ping"})
        opcode, raw = read_frame(peer.rfile)
        assert opcode == 1 and json.loads(raw) == {"type": "pong"}
        # Both a pre-snapshot update and a different market must be filtered.
        send({**row["event"], "type": "update/" + name})
        send({**row["event"], "channel": name + ":999"})
    send(row["event"])
    peer.connection.sendall(frame(8, b"\x03\xe8"))


def monaco_subscription(peer):
    kind = peer.path.rsplit("/", 1)[1]
    rows = json.loads((HERE / "monaco-ws.json").read_text())
    row = next(
        r
        for r in rows
        if r["params"]["type"]
        == ("balances" if kind in ("hold", "reject", "expire") else kind)
    )
    upgrade(peer)

    def recv():
        op, payload = read_frame(peer.rfile)
        assert op == 1
        return json.loads(payload)

    def send(value):
        peer.connection.sendall(frame(1, json.dumps(value).encode()))

    if row["private"]:
        auth = recv()
        assert auth["type"] == "Authenticate"
        pk = "03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8"
        assert (
            auth["public_key"] == pk
            and abs(time.time() * 1000 - auth["timestamp"]) < 60000
        )
        subprocess.run(
            [
                "node",
                "-e",
                """const c=require("node:crypto");const [key,t,s]=process.argv.slice(1);
        const pk=c.createPublicKey({key:Buffer.from("302a300506032b6570032100"+key,"hex"),type:"spki",format:"der"});
        if(!c.verify(null,Buffer.from(["WS-AUTH",key,t].join(String.fromCharCode(10))),pk,Buffer.from(s,"hex")))process.exit(1);""",
                pk,
                str(auth["timestamp"]),
                auth["signature"],
            ],
            check=True,
        )
        if kind == "reject":
            send({"type": "Error", "code": "SESSION_INVALID"})
            return
        send({"type": "Authenticated"})
    assert recv() == {"type": "Subscribe", "channels": [row["channel"]]}
    send({"type": "Ping"})
    assert recv() == {"type": "Pong"}
    channel = row["channel"].replace("*", "ETH")
    send({"type": "Snapshot", "channel": "stale:market", "data": {}})
    if kind != "trades":
        send({"type": "Event", "channel": channel, "data": {"stale": True}})
    data = {
        "type": "Snapshot" if kind != "trades" else "Event",
        "channel": channel,
        "data": {"book_seq": 9007199254740993},
    }
    send(data)
    if kind in ("hold", "expire"):
        assert peer.rfile.read(1) == b""
        return
    if kind == "depth":
        send(
            {
                "type": "Event",
                "channel": channel,
                "data": {
                    "prev_book_seq": 9007199254740999,
                    "book_seq": 9007199254741000,
                },
            }
        )
    if kind == "positions":
        send({"type": "Error", "code": "SNAPSHOT_UNAVAILABLE"})
        send({**data, "type": "Event"})
    peer.connection.sendall(frame(8, b"\x03\xe8"))
