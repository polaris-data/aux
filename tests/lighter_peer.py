"""A loopback matching engine with known nonces and an ambiguous cancellation."""

import base64
import json
import time
from urllib.parse import parse_qs, urlsplit


def serve(peer):
    path = urlsplit(peer.path)
    if path.path == "/api/v1/nextNonce":
        assert parse_qs(path.query) == {"account_index": ["37"], "api_key_index": ["3"]}
        result = {"code": 200, "nonce": 42}
    elif path.path == "/api/v1/accountActiveOrders":
        assert parse_qs(path.query) == {"account_index": ["37"], "market_id": ["4096"]}
        deadline, account, index, sig = peer.headers["authorization"].split(":")
        assert int(time.time()) < int(deadline) <= int(time.time()) + 600
        assert (account, index) == ("37", "3") and len(bytes.fromhex(sig)) == 80
        result = {"code": 200, "orders": []}
    else:
        assert peer.headers["content-type"] == "application/x-www-form-urlencoded"
        fields = parse_qs(peer.rfile.read(int(peer.headers["Content-Length"])).decode())
        tx = json.loads(fields["tx_info"][0])
        assert tx["AccountIndex"] == 37 and tx["ApiKeyIndex"] == 3
        assert len(base64.b64decode(tx["Sig"], validate=True)) == 80
        if fields["tx_type"] == ["14"]:
            assert tx["Nonce"] == 42 and tx["BaseAmount"] == 123456789
        else:
            assert fields["tx_type"] == ["15"] and tx["Nonce"] == 43
            return
        result = {
            "code": 200,
            "tx_hash": "fixture-hash",
            "predicted_execution_time_ms": 5,
        }
    peer.send_response(200)
    peer.end_headers()
    peer.wfile.write(json.dumps(result).encode())
