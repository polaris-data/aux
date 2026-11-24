"""Reference request, signing and reply fixtures for dedicated trading connections."""
import json
from pathlib import Path

from .trading_peer import upgrade
from .ws_server import frame, read_frame

HERE = Path(__file__).parent


def fixture(name):
    return json.loads((HERE / (name + ".json")).read_text())


def serve(peer):
    upgrade(peer)
    route = peer.path.split("/")[2]
    full = peer.path.endswith("/full")
    rows = {r["route"].removeprefix("/v3/"): r for r in fixture("http") if r["venue"] == "derive"}
    orders = fixture("derive-orders")
    req_id = 0

    def read():
        nonlocal req_id
        code, wire = read_frame(peer.rfile)
        assert code == 1
        msg = json.loads(wire)
        req_id += 1
        if route != "rfq":
            assert msg["id"] == req_id, msg
        else:
            assert "id" not in msg
        return msg

    def send(msg):
        peer.connection.sendall(frame(1, json.dumps(msg).encode()))

    if route.startswith("derive"):
        if route == "derive":
            login = read()
            assert login["method"] == "public/login"
            creds = login["params"]
            assert creds["wallet"] == fixture("derive-signing")["owner"] and len(creds["signature"]) == 132
            send({"id": req_id, "result": [9]})
        while True:
            msg = read()
            assert msg["jsonrpc"] == "2.0"
            method, args = msg["method"], msg["params"]
            if method == "subscribe" and args == {"channels":["rejection-fixture"]}:
                send({"jsonrpc":"2.0","id":req_id,"error":{"code":-32001,"message":"channel denied","data":{"seq":9007199254740993}}})
                continue
            if method in ("unsubscribe", "subscribe", "private/set_cancel_on_disconnect"):
                result = args
            elif method == "public/get_instrument" and args.get("instrument_name") in orders["metadata"]:
                result = orders["metadata"][args["instrument_name"]]
            elif method == "public/get_all_currencies" and not args:
                result = orders["metadata"]["currencies"]
            elif method.startswith("private/") and "signature" in args and args.get("signature","").startswith("0x") and method[8:].removesuffix("_debug") in orders["orders"]:
                assert args["signature"] == (orders["orders"].get(method[8:]) or orders["orders"][method[8:].removesuffix("_debug")])["signature"], method
                result = {"signed": method}
            elif method in ("private/transfer_positions","private/transfer_positions_debug"):
                row = orders["orders"][method[8:]]
                assert args["maker_params"]["signature"] == row["maker"]
                assert args["taker_params"]["signature"] == row["taker"]
                result = {"signed": method}
            else:
                row = rows[method]
                assert args == row["request"], method
                result = row["response"]
            if method == "subscribe":
                send({"method":"subscription","params":{"seq":9007199254740993}})
            send({"jsonrpc":"2.0","id":req_id,"result":result})
            if method == "unsubscribe":
                break
    elif route == "rfq":
        auth = read()
        assert auth["type"] == "auth" and auth["auth"] == {"apiKey":"key","secret":"c2VjcmV0","passphrase":"pass"}
        send({"type":"auth","success":True})
        for method in (["RFQ_QUOTE","RFQ_QUOTE_CANCEL","RFQ_CONFIRMATION_RESPONSE"] if full else ["RFQ_QUOTE_CANCEL"]):
            msg = read()
            assert msg["type"] == method and msg["rfq_id"] == "r" and msg["quote_id"] == "q"
            if method == "RFQ_QUOTE":
                assert msg["signed_order"]["signature"] == fixture("polymarket-clob-orders")[0]["signature"]
            send({"type":"ACK_"+method,"rfq_id":"other","quote_id":"q"})
            send({"type":"ACK_"+method,"rfq_id":"r","quote_id":"different"})
            send({"type":"ACK_"+method,"rfq_id":"r","quote_id":"q"})
    else:
        assert route == "perps"
        msg=read()
        assert msg["op"] == {"type":"auth","args":{"proxy":fixture("polymarket-perps-signing")["proxy"],"secret":"fixture"}}
        send({"id":req_id,"data":{"status":"ok"}})
        methods = ["createOrders","modifyOrders","modifyOrdersCOID","cancelOrders","cancelOrdersCOID","cancelAll","autoCancel","updateLeverage","updateLeverages","updateMargin","sub","unsub","ping"] if full else ["ping"]
        for method in methods:
            msg = read()
            if method in ("sub","unsub"):
                assert msg == {"id":req_id,"req":method,"chs":["trades"]}
            else:
                assert msg["req"] == "post" and msg["op"]["type"] == method
                if method != "ping":
                    expected = next(r["request"]["body"]["op"]["args"] for r in fixture("prediction-http") if r["venue"]=="polymarket" and isinstance(r["request"].get("body"),dict) and r["request"]["body"].get("op",{}).get("type")==method)
                    assert msg["op"]["args"] == expected and len(msg["sig"]) == 132 and msg["ts"]>0
            send({"id":str(req_id),"data":method})
    peer.connection.sendall(frame(8,b"\x03\xe8"))
