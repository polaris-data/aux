"""Exercise public Hyperliquid filters and replay on one socket per endpoint."""
import json
from threading import Lock

from .trading_peer import upgrade
from .ws_server import frame, read_frame

_attempts = {}
_lock = Lock()

def serve(peer):
    with _lock:
        count = _attempts.get(peer.path, 0) + 1
        _attempts[peer.path] = count
    assert count <= 2, "unnecessary connection"
    upgrade(peer)
    peer.connection.settimeout(8)

    def receive(method):
        opcode, raw = read_frame(peer.rfile)
        message = json.loads(raw)
        assert opcode == 1 and message["method"] == method, message
        return (message["subscription"]["type"],message["subscription"]["coin"])

    def send(channel,data):
        peer.connection.sendall(frame(1,json.dumps({"channel":channel,"data":data}).encode()))

    expected = {("l2Book","ETH"),("trades","BTC"),("l2Book","SOL")}
    assert {receive("subscribe") for _ in range(3)} == expected
    if count == 1:
        send("l2Book",{"coin":"ETH","time":1,"levels":[[],[]]})
        peer.close_connection = True
        return  # drop without sending a WebSocket close frame
    send("trades",[{"coin":"BTC","px":"1","sz":"2","hash":"test","side":"B","time":2,"tid":3,"users":["u","v"]}])
    send("l2Book",{"coin":"SOL","time":2,"levels":[[],[]]})
    assert receive("unsubscribe") == ("l2Book","ETH")
    assert receive("unsubscribe") == ("trades","BTC")
    send("l2Book",{"coin":"SOL","time":3,"levels":[[],[]]})
    peer.connection.sendall(frame(8,b"\x03\xe8"))


def serve_lighter(peer):
    """Check independent snapshots, reconnect barriers, and a late same-book reader."""
    with _lock:
        attempt = _attempts.get(peer.path, 0) + 1
        _attempts[peer.path] = attempt
    assert attempt <= 3, "unnecessary connection"
    upgrade(peer)
    peer.connection.settimeout(15)

    def receive():
        opcode, payload = read_frame(peer.rfile)
        assert opcode == 1
        return json.loads(payload)

    def send(value):
        peer.connection.sendall(frame(1, json.dumps(value).encode()))

    def event(kind, channel, sequence):
        name = channel.split("/")[0]
        message = {"channel": channel.replace("/", ":"), "type": f"{kind}/{name}", "sequence": sequence}
        if name == "order_book":
            message["order_book"] = {"offset": sequence, "nonce": sequence, "asks": [], "bids": []}
        else:
            message.update(positions={}, assets={}, trades={})
        send(message)

    if attempt == 3:
        assert receive() == {"type": "subscribe", "channel": "order_book/4"}
        event("update", "order_book/4", 0)
        event("subscribed", "order_book/4", 3)
    else:
        channels = {"order_book/4", "account_all/18"}
        messages = [receive(), receive()]
        assert all(message["type"] == "subscribe" for message in messages)
        assert {message["channel"] for message in messages} == channels
        send({"type": "ping"})
        assert receive() == {"type": "pong"}
        for channel in sorted(channels):
            event("update", channel, 0)
            event("subscribed", channel, attempt)
        if attempt == 1:
            peer.close_connection = True
            return
        assert receive() == {"type": "unsubscribe", "channel": "order_book/4"}
        event("update", "account_all/18", 4)
    peer.connection.sendall(frame(8, b"\x03\xe8"))


def serve_qfex(peer):
    with _lock:
        attempt = _attempts.get(peer.path, 0) + 1
        _attempts[peer.path] = attempt
    assert attempt <= 2, "QFEX opened an extra connection for a public filter"
    upgrade(peer)
    peer.connection.settimeout(6)

    def receive(operation):
        opcode, payload = read_frame(peer.rfile)
        req = json.loads(payload)
        assert opcode == 1 and req["type"] == operation
        return req["channels"][0], req["symbols"][0]

    def send(channel, symbol, seq):
        data = {"type":channel, "symbol":symbol, "seq":seq}
        peer.connection.sendall(frame(1, json.dumps(data).encode()))

    assert {receive("subscribe"), receive("subscribe")} == {("trade","*"), ("bbo","BTC")}
    send("bbo", "ETH", 100)
    send("trade", "SOL", attempt)
    if attempt == 1:
        peer.close_connection = True
        return
    send("bbo", "BTC", 9007199254740993)
    assert receive("unsubscribe") == ("trade","*")
    send("bbo", "BTC", 3)
    peer.connection.sendall(frame(8, b"\x03\xe8"))


def serve_qfex_candles(peer):
    with _lock:
        attempt = _attempts.get(peer.path, 0) + 1
        _attempts[peer.path] = attempt
    assert attempt <= 2, "QFEX opened an extra connection for a public filter"
    upgrade(peer)
    peer.connection.settimeout(8)
    symbol = "AAPL-USD"

    def receive():
        opcode, raw = read_frame(peer.rfile)
        assert opcode == 1
        return json.loads(raw)

    def send(message):
        peer.connection.sendall(frame(1, json.dumps(message, separators=(",", ":")).encode()))

    def candle(interval, sequence, ticker=symbol):
        send({"type": "candle", "symbol": ticker, "resolution": interval, "sequence": sequence})

    funding = {"type": "subscribe", "channels": ["funding"], "symbols": [symbol]}
    one_minute = {"type": "subscribe", "channels": ["candle"], "symbols": [symbol], "intervals": ["1MIN"]}
    five_minutes = {**one_minute, "intervals": ["5MINS"]}
    if attempt == 1:
        assert receive() == funding
        assert receive() == one_minute
        send({**funding, "type": "subscribed"})
        send({"type": "funding", "symbol": "MSFT-USD", "sequence": 91})
        send({"type": "funding", "symbol": symbol, "funding_rate": "0.00015", "sequence": 1})
        send({**one_minute, "type": "subscribed", "symbols": ["MSFT-USD"]})
        send({**one_minute, "type": "subscribed", "symbols": ["*"]})
        send({**five_minutes, "type": "subscribed"})
        send({**one_minute, "type": "subscribed"})
        candle("1MIN", 92, "MSFT-USD")
        candle("5MINS", 93)
        candle("1MIN", 2)

        # The alias still holds the funding subscription when the first reader closes.
        assert receive() == five_minutes
        send({**five_minutes, "type": "subscribed"})
        candle("5MINS", 3)
        candle("1MIN", 4)
        send({"type": "funding", "symbol": symbol, "sequence": 5})
        peer.close_connection = True
        return

    replay = [funding, one_minute, five_minutes]
    for _ in range(len(replay)):
        replay.remove(receive())
    candle("1MIN", 94, "MSFT-USD")
    candle("5MINS", 6)
    candle("1MIN", 7)
    send({"type": "funding", "symbol": symbol, "sequence": 8})
    assert receive() == {**funding, "type": "unsubscribe"}
    assert receive() == {**one_minute, "type": "unsubscribe"}
    candle("1MIN", 95)
    candle("5MINS", 9)
    peer.connection.sendall(frame(8, b"\x03\xe8"))


def serve_hyperliquid_accounts(peer):
    """Order updates omit their owner. Each connection represents exactly one account."""
    with _lock:
        attempt = _attempts.get(peer.path, 0) + 1
        _attempts[peer.path] = attempt
    assert attempt <= 3
    upgrade(peer)
    peer.connection.settimeout(8)
    first = "0x" + "11"*20
    second = "0x" + "22"*20

    def receive():
        opcode, raw = read_frame(peer.rfile)
        value = json.loads(raw)
        assert opcode == 1 and value["method"] == "subscribe"
        return value["subscription"]

    def send(channel, data):
        peer.connection.sendall(frame(1, json.dumps({"channel":channel,"data":data}).encode()))

    def orders(seq):
        return [{"order":{"coin":"BTC","side":"A","sz":"1","limitPx":"1","oid":seq,"timestamp":12},
                 "status":"open","statusTimestamp":15}]

    if attempt == 1:
        a, b = receive(), receive()
        assert {a["type"], b["type"]} == {"orderUpdates","openOrders"} and a["user"] == b["user"] == first
        send("openOrders", {"user":second,"dex":"","orders":[]})
        send("openOrders", {"user":first,"dex":"","orders":[]})
        send("orderUpdates", orders(1))
    elif attempt == 2:
        assert receive() == {"type":"orderUpdates","user":second}
        send("orderUpdates", orders(2))
    else:
        assert receive() == {"type":"userFills","user":first,"aggregateByTime":True}
        send("userFills", {"user":first,"fills":[]})
    # Hold connections open so a later subscription cannot pass by reusing a closed group.
    while True:
        try:
            if not peer.connection.recv(2048):
                break
        except (ConnectionError, TimeoutError):
            break
