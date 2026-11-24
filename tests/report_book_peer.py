"""Arcus snapshots and Chainlink report handshake fixtures for installed clients."""
import hashlib
import hmac
import json
import time
from threading import Lock

SHARED_LOCK = Lock()
SHARED = {}
from .trading_peer import upgrade
from .ws_server import frame, read_frame


def serve(peer):
    def send(value):
        peer.connection.sendall(frame(1, json.dumps(value).encode()))

    if peer.path.startswith("/reports/"):
        assert peer.path == "/reports/api/v1/ws?feedIDs=0x" + "01"*32 + "%2C" + "ab"*32
        assert peer.headers["Authorization"] == "fixture"
        timestamp = peer.headers["X-Authorization-Timestamp"]
        assert abs(int(timestamp) - time.time()*1000) < 30000
        msg = f"GET {peer.path} {hashlib.sha256(b'').hexdigest()} fixture {timestamp}"
        assert hmac.compare_digest(peer.headers["X-Authorization-Signature-SHA256"], hmac.digest(b"secret", msg.encode(), "sha256").hex())
        upgrade(peer)
        send({"report":{"fullReport":"0xbeef","sequence":9007199254740993}})
        send({"report":{"other":"malformed"}})
        return

    upgrade(peer)
    opcode, body = read_frame(peer.rfile)
    expected = {"type":"subscribe","channel":"l2OrderbookUpdates","id":"BTC","nLevels":10}
    assert opcode == 1 and json.loads(body) == expected
    def book(kind, market, contents):
        send({"type":kind,"channel":expected["channel"],"id":market,"contents":contents})
    book("channel_data","BTC",{"early":True})
    book("subscribed","OTHER",{"bids":[],"asks":[]})
    book("subscribed","BTC",{"bids":[],"asks":[]})
    book("channel_data","BTC",{"seq":9007199254740993})
    peer.connection.sendall(frame(8, b"\x03\xe8"))


def arcus_order(peer):
    from .trading_peer import verify_ed25519
    req = json.loads(peer.rfile.read(int(peer.headers["Content-Length"])))
    ts = req.pop("timestamp")
    assert int(peer.headers["X-Timestamp"]) == ts
    assert peer.headers["content-type"] == "application/json"
    assert req.pop("address") == "0xabcdef0000000000000000000000000000000001"
    assert req.pop("accountIndex") == 2
    pk = peer.headers["X-API-Key"]
    assert pk == "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c"
    canonical = dict(ad="0xabcdef0000000000000000000000000000000001", ai=2, ct=ts, c="MixedCaseId", m=1, v=1)
    if peer.path == "/v1/placeOrder":
        assert req == dict(marketId=1, orderSide="BUY", orderType="LIMIT", quantity="0.001", price="50000.5", timeInForce="ALO", goodTilTime="4102444800000000", clientId="MixedCaseId", reduceOnly=False)
        canonical.update(op=1,g=4102444800000000000,p=500005,q=100000,r=0,s=0,t=3)
    else:
        assert req == dict(marketId=1,clientId="MixedCaseId")
        canonical["op"] = 2
    verify_ed25519(pk, json.dumps(canonical,sort_keys=True,separators=(",",":")),peer.headers["X-Signature"])
    peer.send_response(200)
    peer.end_headers()
    peer.wfile.write(b'{"status":"ACK","orderId":"123"}')


def hyperdash(peer):
    assert peer.headers["authorization"] == "Bearer hyperdash-key"
    upgrade(peer)

    def send(**fields):
        peer.connection.sendall(frame(1, json.dumps(fields).encode()))

    def read():
        op, buf = read_frame(peer.rfile)
        assert op == 1
        return json.loads(buf)

    send(op="ping")
    assert read() == {"op":"pong"}
    send(op="ready", key_id="id", environment="test")
    assert read() == {"op":"subscribe","channel":"trades","coin":"BTC"}
    send(op="data",sub="orders",batch=9007199254740993,credits=2,cursor="next",events=[])
    assert read() == {"op":"ack","sub":"orders","batch":9007199254740993}
    send(op="acked",sub="orders",batch=9007199254740993,credits_charged=2)
    assert read() == {"op":"unsubscribe","sub":"orders"}
    send(op="error",code="test",detail="Bearer hyperdash-key")
    send(op="unsubscribed",sub="orders")


def shared_reports(peer):
    # Each SDK owns its own two-step fixture. An HTTP call releases the next
    # message only after the caller has opened (or closed) its other reader.
    from threading import Event
    sdk = peer.path.split("/")[2]
    with SHARED_LOCK:
        state = SHARED.setdefault(sdk, {"connections":{}, "steps":[Event(), Event()]})
        if peer.path.endswith("/public/v1/chains"):
            step = int(peer.headers["authorization"].split()[-1]) - 1
            state["steps"][step].set()
            peer.send_response(200)
            peer.end_headers()
            peer.wfile.write(json.dumps(state["connections"]).encode())
            return
        key = peer.headers["authorization"]
        assert key in ("one", "two")
        state["connections"][key] = state["connections"].get(key, 0) + 1
    upgrade(peer)
    for seq, step in enumerate(state["steps"], start=1):
        if not step.wait(10):
            return
        try:
            peer.connection.sendall(frame(1, json.dumps({"report":{"fullReport":key,"seq":seq}}).encode()))
        except OSError:
            return
    peer.connection.sendall(frame(8,b"\x03\xe8"))
