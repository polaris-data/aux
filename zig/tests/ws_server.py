"""Local RFC 6455 peer for all three SDK integration tests."""

import base64
import hashlib
import json
from pathlib import Path
import socket
import struct
import sys


def take(reader, n):
    data = reader.read(n)
    if len(data) != n:
        raise EOFError("peer closed")
    return data


def read_frame(reader):
    op, size = take(reader, 2)
    assert op & 128 and size & 128, "expected a final masked client frame"
    size &= 127
    if size == 126:
        size = int.from_bytes(take(reader, 2), "big")
    elif size == 127:
        size = int.from_bytes(take(reader, 8), "big")
    assert size < 4096
    mask = take(reader, 4)
    return op & 15, bytes(b ^ mask[i % 4] for i, b in enumerate(take(reader, size)))


def frame(op, data, fin=True):
    n = len(data)
    length = bytes([n]) if n < 126 else b"\x7e" + struct.pack(">H", n)
    return bytes([op + (128 if fin else 0)]) + length + data


def upgrade(conn, reader, expected, api_key=None, kalshi=False, path="/stream", read_sub=True):
    assert reader.readline() == f"GET {path} HTTP/1.1\r\n".encode()
    headers = {}
    while True:
        line = reader.readline()
        if line == b"\r\n":
            break
        key, value = line.split(b":", 1)
        headers[key.lower()] = value.strip()
    assert headers[b"upgrade"].lower() == b"websocket"
    assert headers[b"sec-websocket-version"] == b"13"
    if api_key is not None:
        assert headers[b"x-api-key"].decode() == api_key
    if kalshi:
        assert headers[b"kalshi-access-key"] == b"fixture"
        assert headers[b"kalshi-access-timestamp"].isdigit()
        assert len(base64.b64decode(headers[b"kalshi-access-signature"], validate=True)) == 64
    key = headers[b"sec-websocket-key"]
    assert len(base64.b64decode(key, validate=True)) == 16
    accept = base64.b64encode(
        hashlib.sha1(key + b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11").digest()
    )
    conn.sendall(
        b"HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\n"
        b"Connection: Upgrade\r\nSec-WebSocket-Accept: " + accept + b"\r\n\r\n"
    )
    if not read_sub:
        return
    op, data = read_frame(reader)
    assert op == 1
    if expected is None:
        return json.loads(data)
    assert json.loads(data) == expected


def paradex(server):
    rows = json.loads(Path(__file__).with_name("paradex-ws.json").read_text())
    packets = json.loads(Path(__file__).with_name("paradex-sbe.json").read_text())["frames"]
    frames = {r["template"]: bytes.fromhex(r["hex"]) for r in packets}
    public = {"bbo","trades","funding_data","funding_rate_comparison","markets","markets_summary","order_book"}
    for kind, row in sorted(rows.items()):
        for attempt in range(2 if kind in {"orders","order_book"} else 1):
            conn, _ = server.accept()
            with conn, conn.makefile("rb") as reader:
                conn.settimeout(5)
                sub = {"method":"subscribe","jsonrpc":"2.0","id":1,"params":{"channel":row["channel"]}}
                if kind in public:
                    upgrade(conn, reader, sub)
                else:
                    login = upgrade(conn, reader, None)
                    assert login == {"jsonrpc":"2.0","id":"aux-auth","method":"auth",
                                     "params":{"bearer":"test.eyJleHAiOjQxMDI0NDQ4MDB9.test"}}
                    conn.sendall(frame(1, b'{"id":"aux-auth","result":true}'))
                    op, data = read_frame(reader)
                    assert op == 1 and json.loads(data) == sub
                if kind in {"orders","order_book"} and attempt == 0:
                    continue
                conn.sendall(frame(1, b'{"params":{"channel":"elsewhere","data":{}}}'))
                if kind == "order_book":
                    delta = {"params":{"channel":row["channel"],"data":{"update_type":"d"}}}
                    conn.sendall(frame(1,json.dumps(delta).encode()))
                if row["template"]:
                    data = frames[row["template"]]
                    conn.sendall(frame(2, data[:7], False) + frame(0, data[7:]))
                else:
                    msg = {"method":"subscription","params":{"channel":row["channel"],"data":row["payload"]}}
                    conn.sendall(frame(1,json.dumps(msg).encode()))
                if kind == "account":
                    conn.sendall(frame(2,frames[40]) + frame(2,frames[41]))
                conn.sendall(frame(8,b"\x03\xe8"))
                assert read_frame(reader)[0] == 8


def derive(server):
    rows = json.loads(Path(__file__).with_name("derive-ws.json").read_text())
    for row in (rows[k] for k in sorted(rows)):
        spec = row["specification"]
        channel = row["channel"]
        private = spec["type"] in {"balances", "best_quotes", "orders", "quotes", "account_trades", "rfqs"}
        # A dropped private socket must log in again before replaying the subscription.
        for attempt in range(2 if spec["type"] == "orders" else 1):
            conn, _ = server.accept()
            with conn, conn.makefile("rb") as reader:
                conn.settimeout(5)
                expected = {"id": 1, "method": "subscribe", "params": {"channels": [channel]}}
                if private:
                    login = upgrade(conn, reader, None)
                    assert login["method"] == "public/login" and login["id"] == "aux-login"
                    proof = login["params"]
                    assert proof["wallet"] == "0x" + "22" * 20 and proof["timestamp"].isdigit()
                    assert len(bytes.fromhex(proof["signature"][2:])) == 65
                    conn.sendall(frame(1, b'{"id":"aux-login","result":[]}'))
                    op, raw = read_frame(reader)
                    assert op == 1 and json.loads(raw) == expected
                else:
                    upgrade(conn, reader, expected)
                if spec["type"] == "orders" and attempt == 0:
                    continue
                conn.sendall(frame(1, b'{"params":{"channel":"other","data":{}}}'))
                event = {"method": "subscription", "params": {"channel": channel, "data": row["payload"]}}
                conn.sendall(frame(1, json.dumps(event).encode()))
                conn.sendall(frame(8, b""))
                assert read_frame(reader) == (8, b"")


def phoenix(server):
    rows = json.loads(Path(__file__).with_name("phoenix-ws.json").read_text())
    for name in sorted(rows):
        row = rows[name]
        expected = {"type": "subscribe", "subscription": row["specification"]}

        def session(replay):
            conn, _ = server.accept()
            with conn, conn.makefile("rb") as reader:
                conn.settimeout(6)
                upgrade(conn, reader, expected, "fixture-key")
                if replay:
                    payload = base64.b64encode(
                        Path(__file__).with_name("http-body.zst").read_bytes()
                    ).decode()
                    event = dict(
                        row["frame"],
                        messageType="encodedSnapshot",
                        encoding="base64+zstd",
                        payload=payload,
                    )
                    conn.sendall(frame(1, json.dumps(event).encode()))
                else:
                    conn.sendall(frame(1, json.dumps(row["frame"]).encode()))
                    if name == "exchange":
                        event = dict(
                            row["frame"], messageType="update", sequenceNumber=3
                        )
                        conn.sendall(frame(1, json.dumps(event).encode()))
                        assert reader.read(1) == b""
                        return
                conn.sendall(frame(8, b""))
                assert read_frame(reader) == (8, b"")

        session(False)
        if name == "exchange":
            session(True)


def prediction(server, venue):
    kalshi = venue == "kalshi"
    sub = ({"id":1, "cmd":"subscribe", "params":{"channels":["orderbook_delta"]}} if kalshi
           else {"type":"subscribe", "channels":[{"channel":"book"}]})
    snapshot = "orderbook_snapshot" if kalshi else "snapshot"
    delta = "orderbook_delta" if kalshi else "update"
    for attempt in range(2):
        conn, _ = server.accept()
        with conn, conn.makefile("rb") as reader:
            conn.settimeout(5)
            upgrade(conn, reader, sub, kalshi=kalshi)
            def send(kind, seq):
                conn.sendall(frame(1,json.dumps({"type":kind,"seq":seq}).encode()))
            send(delta, 100)
            send(snapshot, attempt + 1)
            if attempt == 0:
                if kalshi:
                    send(delta, 5)
                    assert reader.read(1) == b""
            else:
                send(delta,3)
                conn.sendall(frame(8,b""))
                assert read_frame(reader) == (8,b"")


def extended(server):
    rows = json.loads(Path(__file__).with_name("extended-ws.json").read_text())
    for name, row in sorted(rows.items()):
        spec = row["specification"]
        for attempt in range(2 if name == "orderbooks_rpc" else 1):
            conn, _ = server.accept()
            with conn, conn.makefile("rb") as reader:
                conn.settimeout(4)
                key = "fixture" if row["private"] else None
                sub = upgrade(conn, reader, None, key, path=row["path"], read_sub=spec["rpc"])
                if spec["rpc"]:
                    assert sub["method"] == "subscribe" and sub["jsonrpc"] == "2.0" and sub["id"] == "1"
                    scopes = {"funding":"funding-rates","mark_price":"prices","index_price":"prices","rfq_orderbooks":"orderbooks"}
                    selector = dict(spec["params"].get("selector",{}))
                    if spec["market"]:
                        selector["market"] = spec["market"]
                    if "orderbooks" in name:
                        selector.update(depth="full",rfqOnly=name.startswith("rfq"))
                    if spec["channel"] in ("mark_price","index_price"):
                        selector["type"]=spec["channel"].split("_")[0]
                    params = {"scope":scopes.get(spec["channel"],spec["channel"]), "selector":selector}
                    if key:
                        params["apiKey"]=key
                    assert sub["params"] == params
                    conn.sendall(frame(1,b'{"subscription":"unrelated","seq":999}'))
                event = row["frame"]
                if name == "orderbooks_rpc":
                    conn.sendall(frame(1,json.dumps({**event,"type":"DELTA","seq":0}).encode()))
                conn.sendall(frame(1,json.dumps(event).encode()))
                if name == "orderbooks_rpc" and attempt == 0:
                    conn.sendall(frame(1,json.dumps({**event,"type":"DELTA","seq":4}).encode()))
                    assert reader.read(1) == b""
                else:
                    conn.sendall(frame(8,b""))
                    assert read_frame(reader) == (8,b"")
                    
def serve():
    with socket.socket() as server:
        server.bind(("127.0.0.1", 0))
        server.listen()
        server.settimeout(8)
        print(f"ws://127.0.0.1:{server.getsockname()[1]}/stream", flush=True)
        if sys.argv[1:] == ["extended"]:
            return extended(server)
        if sys.argv[1:] == ["derive-reject"]:
            conn, _ = server.accept()
            with conn, conn.makefile("rb") as reader:
                conn.settimeout(3)
                login = upgrade(conn, reader, None)
                assert login["method"] == "public/login"
                conn.sendall(frame(1, b'{"id":"aux-login","error":{"code":401}}'))
                assert reader.read(1) == b"", "sent subscription after rejected login"
            return
        if sys.argv[1:] in (["pascal"],["kalshi"]):
            return prediction(server,sys.argv[1])
        if sys.argv[1:] == ["paradex"]:
            return paradex(server)
        if sys.argv[1:] == ["derive"]:
            return derive(server)
        if len(sys.argv) > 1 and sys.argv[1] == "phoenix":
            return phoenix(server)
        for attempt in range(2):
            conn, _ = server.accept()
            with conn, conn.makefile("rb") as reader:
                conn.settimeout(6)
                upgrade(
                    conn,
                    reader,
                    {"type": "subscribe", "channels": ["trade"], "symbols": ["BTC"]},
                )
                if attempt == 0:
                    conn.sendall(frame(1, b'{"type":"trade","symbol":"ETH"}'))
                    conn.sendall(
                        frame(1, b'{"type":"trade","symbol":"BTC","seq":', False)
                    )
                    conn.sendall(frame(9, b"test"))
                    assert read_frame(reader) == (10, b"test")
                    conn.sendall(frame(0, b"9999999999999999999}"))
                    # EOF triggers a new handshake and a fresh subscribe frame.
                else:
                    conn.sendall(frame(1, b'{"type":"trade","symbol":"BTC","seq":2}'))
                    conn.sendall(frame(8, b"\x03\xe8"))
                    assert read_frame(reader) == (8, b"\x03\xe8")


if __name__ == "__main__":
    serve()
