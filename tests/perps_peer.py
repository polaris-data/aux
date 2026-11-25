"""Loopback transport and signed request checks for the seven perpetuals exchanges."""

import base64
import hashlib
import hmac
import json
from pathlib import Path
import struct
from threading import Event
import time
from urllib.parse import parse_qs, urlsplit
import zlib

from .trading_peer import upgrade, verify_ed25519
from .ws_server import frame, read_frame

HERE = Path(__file__).parent
ROWS = json.loads((HERE / "perps-http.json").read_text())
SIGNING = json.loads((HERE / "perps-signing.json").read_text())
MARKET = "0x" + "33" * 32
KEY = SIGNING["key"]
OWNER = SIGNING["owner"].lower()
ED_PUBLIC = "03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8"


def base58(data):
    alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
    number = int.from_bytes(data, "big")
    result = ""
    while number:
        number, rem = divmod(number, 58)
        result = alphabet[rem] + result
    return alphabet[0] * (len(data) - len(data.lstrip(b"\0"))) + result


def unbase58(text):
    alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
    number = 0
    for char in text:
        number = number * 58 + alphabet.index(char)
    return b"\0" * (len(text) - len(text.lstrip("1"))) + number.to_bytes((number.bit_length() + 7) // 8, "big")


ED_ACCOUNT = base58(bytes.fromhex(ED_PUBLIC))


def respond(peer, body, status=200):
    data = json.dumps(body, separators=(",", ":")).encode()
    peer.send_response(status)
    peer.send_header("Content-Type", "application/json")
    peer.send_header("Content-Length", str(len(data)))
    peer.end_headers()
    peer.wfile.write(data)


def read_body(peer):
    return peer.rfile.read(int(peer.headers.get("Content-Length", "0")))


def query_dict(query):
    return {key: value[-1] for key, value in parse_qs(query).items()}


def verify_aster(fields):
    params = query_dict(fields)
    assert params["user"] == OWNER
    assert params["signer"] == OWNER
    assert len(bytes.fromhex(params["signature"][2:])) == 65
    assert abs(time.time() - int(params["nonce"]) / 1_000_000) < 30
    assert fields.endswith("&signature=" + params["signature"])
    return params


def serve(peer):
    url = urlsplit(peer.path)
    _, _, venue, rest = url.path.split("/", 3)
    path = "/" + rest
    background_close = venue == "aster" and path.startswith("/background/")
    if background_close:
        path = path.removeprefix("/background")
    if "upgrade" in (peer.headers.get("Connection") or "").lower():
        return serve_ws(peer, venue, path)
    raw = read_body(peer)
    if venue == "injective":
        if "injective_derivative_exchange_rpc." in path:
            return serve_grpc(peer, path, raw)
        if path == "/cosmos/base/tendermint/v1beta1/blocks/latest":
            return respond(peer, {"block": {"header": {"chain_id": "injective-1", "height": "9000"}}})
        if path.startswith("/cosmos/auth/v1beta1/accounts/"):
            sender = path.rsplit("/", 1)[1]
            assert sender == SIGNING["injective"]["sender"]
            sequence = "3" if getattr(peer.server, "injective_tx", None) else "2"
            return respond(peer, {"account": {"base_account": {"address": sender, "sequence": sequence, "account_number": "17"}}})
        if path.startswith("/cosmos/tx/v1beta1/txs"):
            if peer.command == "POST":
                data = json.loads(raw)
                assert data["mode"] == "BROADCAST_MODE_SYNC"
                tx = base64.b64decode(data["tx_bytes"], validate=True)
                assert b"/injective.crypto.v1beta1.ethsecp256k1.PubKey" in tx
                assert b"MsgCreateDerivativeLimitOrder" in tx or b"MsgCancelDerivativeOrder" in tx
                assert b"inj1" in tx and MARKET.encode() in tx
                digest = hashlib.sha256(tx).hexdigest().upper()
                operation = "place_hash" if b"lost-ack" in tx else "cancel_hash"
                assert digest == SIGNING["injective"][operation]
                peer.server.injective_tx = digest
                if b"lost-ack" in tx:
                    peer.close_connection = True
                    return
            else:
                digest = path.rsplit("/", 1)[1]
                if digest != getattr(peer.server, "injective_tx", None):
                    return respond(peer, {"code": 5}, 404)
            return respond(peer, {"tx_response": {"txhash": digest, "code": 0}})
    if venue == "aster" and path == "/listen-key-test":
        lifecycle = peer.server.aster_lifecycle
        if background_close and "DELETE" in lifecycle:
            peer.server.aster_release.set()
        return respond(peer, lifecycle)
    if venue == "aster" and path == "/fapi/v3/listenKey":
        verify_aster(raw.decode())
        if peer.command == "POST":
            peer.server.aster_lifecycle = ["POST"]
            if background_close:
                peer.server.aster_release = Event()
            return respond(peer, {"listenKey": "fixture-listen-key"})
        peer.server.aster_lifecycle.append(peer.command)
        if peer.command == "DELETE":
            assert peer.server.aster_lifecycle == ["POST", "PUT", "DELETE"]
            if background_close:
                assert peer.server.aster_release.wait(5), "the Node event loop waited for the DELETE response"
        return respond(peer, {})
    params = query_dict(url.query)
    is_aster_form = venue == "aster" and peer.headers.get("Content-Type") == "application/x-www-form-urlencoded"
    body = query_dict(raw.decode()) if is_aster_form else json.loads(raw) if raw else None
    candidates = [r for r in ROWS if r["venue"] == venue and r["path"] == path and r["verb"] == peer.command]
    if venue == "nado" and path == "/gateway/v1/query":
        candidates = [r for r in candidates if r["query"]["type"] == params["type"]]
    if venue in ("paragon", "trade_xyz") and path == "/info":
        candidates = [r for r in candidates if r["body"]["type"] == body["type"]]
    if venue == "nado" and path == "/gateway/v1/execute":
        op = "place_order" if "place_order" in body else "cancel_order"
        candidates = [r for r in candidates if r["operation"] == op]
    if venue in ("paragon", "trade_xyz") and path == "/exchange":
        op = "place_order" if body["action"]["type"] == "order" else "cancel_order"
        candidates = [r for r in candidates if r["operation"] == op]
    if venue == "pacifica" and path == "/api/v1/orders":
        assert params["account"]
    assert candidates, (venue, peer.command, path, params)
    row = candidates[0]
    reply = row["response"]
    if not row["private"]:
        if path == "/info":
            assert row["body"] == body, (row, body)
        return respond(peer, reply)
    if venue == "ondo_perps":
        if peer.headers.get("Authorization"):
            assert peer.headers["Authorization"] == "Bearer fixture-token"
        else:
            stamp = peer.headers["ONDO-TIMESTAMP"]
            assert abs(time.time() - int(stamp) / 1000) < 30
            assert peer.headers["ONDO-KEY-ID"] == "fixture-key"
            signed = (stamp + peer.command + peer.path).encode() + raw
            assert hmac.new(b"fixture-secret", signed, "sha256").hexdigest() == peer.headers["ONDO-SIGN"]
        if body is not None and body.get("clientOrderId") == "lost-ack":
            return respond(peer, {"success": False}, 503)
    elif venue == "nado":
        place = row["operation"] == "place_order"
        payload = body["place_order" if place else "cancel_orders"]
        assert payload.pop("signature") == SIGNING["nado_order" if place else "nado_cancel"]
        assert payload == row["body"], (payload, row["body"])
    elif venue in ("paragon", "trade_xyz"):
        expected = SIGNING["hyperliquid"][f"{venue}.{row['operation']}"]
        assert body["nonce"] == expected["nonce"]
        assert body["action"] == expected["action"]
        for field in ("r", "s"):
            assert int(body["signature"][field], 16) == int(expected["signature"][field], 16)
        assert body["signature"]["v"] == expected["signature"]["v"]
    elif venue == "pacifica":
        stamp = body.pop("timestamp")
        expiry = body.pop("expiry_window")
        assert abs(time.time() - stamp / 1000) < 30
        assert body.pop("account") == ED_ACCOUNT
        sig = unbase58(body.pop("signature")).hex()
        typ = {"place_order": "create_order", "place_market_order": "create_market_order", "cancel_order": "cancel_order"}[row["operation"]]
        message = json.dumps(dict(data=body, timestamp=stamp, expiry_window=expiry, type=typ), sort_keys=True, separators=(",", ":"))
        verify_ed25519(ED_PUBLIC, message, sig)
        assert body == row["body"]
    elif venue == "aster":
        fields = url.query if peer.command == "GET" else raw.decode()
        verify_aster(fields)
    return respond(peer, reply)


def packet(flags, data):
    return bytes([flags]) + struct.pack(">I", len(data)) + data


def proto_text(tag, text):
    return bytes([tag * 8 + 2, len(text)]) + text


def serve_grpc(peer, path, raw):
    assert path.endswith("/StreamOrderbookV2")
    assert peer.headers["Content-Type"] == "application/grpc-web-text+proto"
    assert base64.b64decode(raw, validate=True) == packet(0, proto_text(1, MARKET.encode()))
    # orderbook.sequence = 9007199254740993 (unsigned protobuf varint).
    book = b"\x18\x81\x80\x80\x80\x80\x80\x80\x10"
    data = packet(0, proto_text(1, book) + proto_text(4, MARKET.encode()))
    if path.startswith("/trailer-error/"):
        raw = base64.b64encode(packet(128, b"grpc-status: 13\r\ngrpc-message: fixture error\r\n"))
    else:
        trailer = packet(128, b"grpc-status: 0\r\n")
        raw = base64.b64encode(data) + base64.b64encode(trailer)
    peer.send_response(200)
    peer.send_header("Content-Type", "application/grpc-web-text+proto")
    peer.send_header("Content-Length", str(len(raw)))
    peer.end_headers()
    for index in range(0, len(raw), 7):
        peer.wfile.write(raw[index:index + 7])


def serve_ws(peer, venue, path):
    def receive():
        code, raw = read_frame(peer.rfile)
        assert code == 1
        return json.loads(raw)

    def send(value):
        peer.connection.sendall(frame(1, json.dumps(value).encode()))

    if venue == "nado":
        assert "permessage-deflate" in peer.headers["Sec-WebSocket-Extensions"]
        request = peer.headers["Sec-WebSocket-Key"]
        accept = base64.b64encode(hashlib.sha1((request + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").encode()).digest())
        peer.connection.sendall(b"HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\n"
                                b"Connection: Upgrade\r\nSec-WebSocket-Extensions: permessage-deflate\r\n"
                                b"Sec-WebSocket-Accept: " + accept + b"\r\n\r\n")
        if path == "/private":
            auth = receive()
            assert auth["method"] == "authenticate" and auth["id"] == 1
            account = "0x" + "22" * 32
            assert auth["tx"]["sender"] == account
            assert 20 < int(auth["tx"]["expiration"]) / 1000 - time.time() <= 31
            assert len(bytes.fromhex(auth["signature"][2:])) == 65
            send({"id": 1, "result": None})
            request = receive()
            assert request["stream"] == {"type": "order_update", "product_id": 4, "subaccount": account}
            calls = getattr(peer.server, "nado_connections", 0) + 1
            peer.server.nado_connections = calls
            send(dict(type="order_update", product_id=4, timestamp=str(100 + calls)))
            peer.close_connection = True
            return
        request = receive()
        assert request["stream"] == {"type": "book_depth", "product_id": 4}
        compressor = zlib.compressobj(wbits=-15)
        for index, (previous, current) in enumerate([(100, 110), (110, 120), (130, 140)]):
            msg = dict(type="book_depth", product_id=4, last_max_timestamp=str(previous), max_timestamp=str(current), bids=[["123000000000000000000", "4200000000000000000"]], asks=[])
            data = compressor.compress(json.dumps(msg).encode()) + compressor.flush(zlib.Z_SYNC_FLUSH)
            data = data[:-4]
            if index == 0:
                peer.connection.sendall(frame(0x41, data[:8], fin=False) + frame(9, b"hi") + frame(0, data[8:]))
            else:
                peer.connection.sendall(frame(0x41, data))
        assert read_frame(peer.rfile) == (10, b"hi")
        peer.close_connection = True
        return

    upgrade(peer)
    if venue == "ondo_perps":
        assert receive() == {"op": "login", "args": {"token": "fixture-token"}}
        if path == "/denied":
            return send(dict(type="error", code="invalid_token", message="denied"))
        send(dict(type="loggedIn"))
        assert receive() == {"op": "subscribe", "channel": "fillsPerps"}
        calls = getattr(peer.server, "ondo_connections", 0) + 1
        peer.server.ondo_connections = calls
        send(dict(type="update", channel="fillsPerps", data=dict(seq=calls, id=9007199254740993)))
    elif venue == "pacifica":
        request = receive()
        if path == "/book":
            assert request == {"method": "subscribe", "params": {"source": "book", "symbol": "BTC", "agg_level": 1}}
            send(dict(channel="book", data={"s": "BTC", "l": [[], []]}))
        else:
            assert request == {"method": "subscribe", "params": {"source": "trades", "symbol": "BTC"}}
            send(dict(channel="trades", data=[{"s": "SOL"}, {"s": "BTC", "p": "10000.001"}]))
    elif venue in ("paragon", "trade_xyz"):
        assert receive()["subscription"]["type"] == "orderUpdates"
        order = {"side": "B", "limitPx": "1", "sz": "2", "origSz": "2", "oid": 9007199254740993, "timestamp": 1}
        data = [dict(order={**order, "coin": coin}, status="open", statusTimestamp=1) for coin in ("para:TOTAL2", "xyz:TSLA")]
        send(dict(channel="orderUpdates", data=data))
    elif venue == "aster":
        assert path == "/ws/fixture-listen-key"
        lifecycle = peer.server.aster_lifecycle
        send(dict(e="ACCOUNT_UPDATE", E=len(lifecycle)))
    else:
        raise AssertionError(venue)
