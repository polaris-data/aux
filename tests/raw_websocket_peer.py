"""Wire checks for caller-managed Zig WebSockets."""

import json
from pathlib import Path

from .trading_peer import upgrade
from .ws_server import frame, read_frame

CLOSE = frame(8, b"\x03\xe8")


def qfex(peer):
    def expect(action):
        for channels, intervals in [(["trade", "funding"], []), (["candle"], ["1MIN", "5MINS"])]:
            request = {"type": action, "channels": channels, "symbols": ["ABSI-USD", "CYPH-USD"]}
            if intervals:
                request["intervals"] = intervals
            assert read_frame(peer.rfile) == (1, json.dumps(request, separators=(",", ":")).encode())

    expect("subscribe")
    messages = [b'{"type":"subscribed"}', b' {"channel":"candle","n":1.000} ',
                b'{"type":"funding","symbol":"ABSI-USD","funding_rate":"0.0001500"}', b'{"error":"fixture"}', b"{broken"]
    messages.extend(str(i).encode() for i in range(70))
    peer.connection.sendall(b"".join(frame(1, payload) for payload in messages))
    expect("unsubscribe")


def deribit(peer):
    topics = ["ticker.BTC-PERPETUAL.100ms", "trades.BTC-PERPETUAL.agg2", "book.BTC-PERPETUAL.none.20.100ms"]
    topics.extend(f"ticker.TEST-{i}.100ms" for i in range(3, 501))
    for method, first_id in [("public/subscribe", 7), ("public/unsubscribe", 40)]:
        for index, batch in enumerate([topics[:500], topics[500:]]):
            request = {"jsonrpc": "2.0", "id": first_id + index, "method": method, "params": {"channels": batch}}
            assert read_frame(peer.rfile, limit=32768) == (1, json.dumps(request, separators=(",", ":")).encode())
    peer.connection.sendall(frame(1, b'{"method":"heartbeat","params":{"type":"test_request"}}'))


def paradex(peer):
    assert read_frame(peer.rfile) == (2, b"\x00\xff")
    fixtures = json.loads(Path(__file__).with_name("paradex-sbe.json").read_text())
    payload = bytes.fromhex(next(row["hex"] for row in fixtures["frames"] if row["template"] == 2))
    peer.connection.sendall(frame(2, payload[:7], fin=False) + frame(9, b"raw-ping"))
    assert read_frame(peer.rfile) == (10, b"raw-ping")
    peer.connection.sendall(frame(0, payload[7:]))


def n1(peer, path):
    selectors = "trades@BTCUSD&deltas@BTCUSD&candle@BTCUSD:1&rfq-fills@AAPLUSD&liquidations"
    case, target = path.removeprefix("n1/").split("/ws/")
    assert target == ("liquidations" if case == "sdk" else selectors)
    assert peer.headers.get("Authorization") is None
    peer.connection.sendall(frame(9, b"n1-ping"))
    assert read_frame(peer.rfile) == (10, b"n1-ping")
    trades = b' {"trades":{"market_symbol":"BTCUSD","price":1.000}} '
    liquidation = b'{"liquidation":{"update_id":42}}'
    if case == "raw":
        peer.connection.sendall(frame(1, trades) + CLOSE)
        assert read_frame(peer.rfile) == (8, b"\x03\xe8")
        return
    if case == "sdk":
        peer.connection.sendall(frame(1, liquidation))
        return
    assert case == "managed"
    count = getattr(peer.server, "n1_managed_connections", 0)
    peer.server.n1_managed_connections = count + 1
    if count == 0:
        peer.connection.sendall(frame(1, b'{"type":"connected"}') + frame(1, trades) + CLOSE)
        assert read_frame(peer.rfile) == (8, b"\x03\xe8")
    else:
        assert count == 1
        peer.connection.sendall(b"".join(frame(1, payload) for payload in [
            b'{"delta":{"market_symbol":"BTCUSD","bids":[],"asks":[]}}',
            b'{"res":"1","mid":1,"c":2.000}',
            b'{"rfqFills":{"market_id":14,"requests":[]}}',
            liquidation,
        ]))


def kuru(peer):
    rows = json.loads(Path(__file__).with_name("market-data-ws.json").read_text())
    fixture = next(row for row in rows if row["venue"] == "kuru" and row["case"] == "l2-bootstrap")
    book = fixture["subscribe"]
    for expected in [dict(book, feedEpoch=13, seq=9007199254740993),
                     dict(book, id=2, t="trades@f")]:
        opcode, packet = read_frame(peer.rfile)
        assert opcode == 1 and json.loads(packet) == expected
    packet = bytes.fromhex(fixture["binary"][0])
    peer.connection.sendall(frame(2, packet[:7], fin=False) + frame(0, packet[7:]))
    for subscription_id in (1, 2):
        opcode, packet = read_frame(peer.rfile)
        assert opcode == 1 and json.loads(packet) == {"op": "unsub", "id": subscription_id}


def serve(peer):
    upgrade(peer)
    path = peer.path.removeprefix("/caller-websocket/")
    if path == "prepare/extended/orderbooks/BTC-USD?depth=1":
        assert peer.headers.get_all("Origin") == ["https://app.extended.exchange"]
        assert peer.headers.get_all("User-Agent") == ["Mozilla/5.0"]
        peer.connection.sendall(frame(1, b'{"type":"DELTA","seq":1}'))
    elif path in ("prepare/messages", "prepare/paradex?fixture=1&sbeSchemaId=1&sbeSchemaVersion=1"):
        while True:
            opcode, payload = read_frame(peer.rfile)
            assert opcode == 1
            if payload == b"done":
                break
            peer.connection.sendall(frame(1, payload))
    elif path == "qfex?feed=public":
        qfex(peer)
    elif path == "deribit":
        deribit(peer)
    elif path == "extended":
        peer.connection.sendall(frame(1, b'{"type":"DELTA","seq":1,"data":{"m":"BTC-USD","b":[],"a":[]}}'))
    elif path == "paradex":
        paradex(peer)
    elif path.startswith("n1/"):
        return n1(peer, path)
    elif path == "kuru":
        kuru(peer)
    elif path == "monaco/ohlcv":
        topic = "ohlcv:550e8400-e29b-41d4-a716-446655440000:SPOT:1m"
        opcode, packet = read_frame(peer.rfile)
        assert opcode == 1 and json.loads(packet) == {"type": "Subscribe", "channels": [topic]}
        peer.connection.sendall(frame(1, b'{"type":"Event","channel":"' + topic.encode() + b'","data":{"open":1.000}}'))
        return
    elif path == "risex/oracle":
        opcode, packet = read_frame(peer.rfile)
        assert opcode == 1 and json.loads(packet) == {"method": "subscribe", "params": {"channel": "oracle", "market_ids": [1, 2]}}
        peer.connection.sendall(frame(1, b'{"channel":"oracle","type":"subscribed","status":"success"}'))
        peer.connection.sendall(frame(1, b'{"channel":"oracle","type":"update","market_id":null,"data":[{"market_id":"1","price":"63126084821834306372811"}]}'))
        return
    elif path == "cancel":
        peer.connection.sendall(frame(1, b"ready"))
        assert peer.rfile.read(1) == b""
        return
    else:
        assert path == "drop"
        return
    peer.connection.sendall(CLOSE)
    assert read_frame(peer.rfile) == (8, b"\x03\xe8")
