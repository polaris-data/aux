"""An ordered command peer rejects duplicate submissions and checks signed wire frames."""
import base64
import json
from pathlib import Path
import time

from .trading_peer import upgrade, verify_ed25519
from .ws_server import read_frame, frame

VECTORS = json.loads(Path(__file__).with_name("pascal-permits.json").read_text())[:3]


def serve(peer):
    if peer.path.split("/")[2] in {"derive","derive-public","rfq","perps"}:
        from .command_parity_peer import serve as venue_session
        return venue_session(peer)
    if peer.path.startswith("/rpc/paradex"):
        return paradex(peer)
    pascal = peer.path == "/rpc/pascal"
    if not pascal:
        assert peer.path in ("/rpc/kalshi", "/rpc/kalshi_perps")
        timestamp = peer.headers["KALSHI-ACCESS-TIMESTAMP"]
        assert abs(time.time()*1000-int(timestamp)) < 30000
        assert peer.headers["KALSHI-ACCESS-KEY"] == "fixture-k"
        verify_ed25519("ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c",
                       timestamp + "GET" + peer.path, base64.b64decode(peer.headers["KALSHI-ACCESS-SIGNATURE"]).hex())
    upgrade(peer)

    def send(message):
        peer.connection.sendall(frame(1, json.dumps(message).encode()))

    commands = ["ping", "place", "cancel", "mass_cancel"] if pascal else ["subscribe", "list_subscriptions", "update_subscription", "unsubscribe"]
    for idx, method in enumerate(commands, 1):
        opcode, data = read_frame(peer.rfile)
        message = json.loads(data)
        assert opcode == 1
        assert message["id"] == (str(idx) if pascal else idx)
        assert message["type" if pascal else "cmd"] == method
        if pascal and idx > 1:
            payload = message["payload"]
            auth = payload.pop("auth")
            row = VECTORS[idx-2]
            assert auth["owner"] == row["auth"]["owner"] and auth["signer"] == row["signer"]
            assert abs(int(auth["client_ts_ms"]) - time.time()*1000) < 30000
            assert len(auth["signature"]) >= 86
            for key, val in row["body"].items():
                assert payload[key] == val
        if not pascal:
            assert message["params"] == {"step": idx}
        if idx == 1:
            send({"event": "queued", "value": 9007199254740993})
            if not pascal:
                # Let the SDK return a pending command before delivering its reply.
                time.sleep(.25)
        send({"id":str(idx),"ack":method})
    peer.connection.sendall(frame(8,b"\x03\xe8"))


def paradex(peer):
    assert not peer.headers.get("Authorization")
    upgrade(peer)
    def step(i, method, expected):
        opcode, wire = read_frame(peer.rfile)
        msg = json.loads(wire)
        assert opcode == 1 and msg == {"jsonrpc": "2.0", "id": i, "method": method, "params": expected}, (msg, method)
        peer.connection.sendall(frame(1,b"pong"))
        if i == 2:
            peer.connection.sendall(frame(1,b'{"method":"subscription","params":{"seq":9007199254740993}}'))
        peer.connection.sendall(frame(1,json.dumps({"id":i,"jsonrpc":"2.0","result":{"ok":method}}).encode()))
    step(1, "auth", {"bearer":"fixture.eyJleHAiOjQxMDI0NDQ4MDB9.fixture"})
    step(2, "subscribe", {"channel":"orders.ETH-USD-PERP"})
    step(3, "unsubscribe", {"channel":"orders.ETH-USD-PERP"})
    if peer.path.endswith("/readonly"):
        peer.connection.sendall(frame(8,b"\x03\xe8"))
        return
    vectors=json.loads(Path(__file__).with_name("paradex-signing.json").read_text())["vectors"]
    def signed(name):
        row=next(v for v in vectors if v["name"]==name and v["network"].endswith("TESTNET"))
        return {**row["inputs"], "signature":json.dumps(row["signature"],separators=(",",":"))}
    order,modify=signed("order_LIMIT"),signed("modify")
    for idx,(cmd,params) in enumerate([
        ("order.create",order),("order.create_batch",[order]),("order.modify",{**modify,"id":"123456"}),
        ("order.cancel",{"id":"order"}),("order.cancel",{"client_id":"client","market":"ETH"}),
        ("order.cancel_all",{}),("order.cancel_batch",{"order_ids":["a","b"]}),
        ("order.cancel_on_disconnect",{"enabled":True}),("modify_subscription",{"channel":"orders","add":["BTC"]})
    ],4):step(idx,cmd,params)
    peer.connection.sendall(frame(8,b"\x03\xe8"))
