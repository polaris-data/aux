"""Hosted Tardis capture slices and a local Machine instance."""

import gzip
import json
from urllib.parse import parse_qs, urlsplit
from .trading_peer import upgrade
from .ws_server import frame, read_frame

OPTIONS = {"exchange": "bit/mex", "symbols": ["BTC/USD"]}
DATASET = gzip.compress(b"timestamp,symbol\n123,BTC/USD\n", mtime=0)


def serve(peer):
    url = urlsplit(peer.path)
    query = parse_qs(url.query)
    if url.path.startswith("/machine/ws-"):
        upgrade(peer)
        if url.path == "/machine/ws-replay":
            assert query == {"exchange": ["bit/mex"], "from": ["a"], "to": ["b"]}
            assert read_frame(peer.rfile) == (1, b'{"subscribe":1}')
        else:
            assert json.loads(query["options"][0]) == OPTIONS
        op, data = read_frame(peer.rfile)
        assert op == 1 and json.loads(data) == {"ping": True}
        peer.connection.sendall(
            frame(1, b'{"seq":9007199254740993}') + frame(8, b"\x03\xe8")
        )
        return
    if url.path.startswith("/machine/"):
        assert json.loads(query["options"][0]) == OPTIONS
        data = b'{"sequence":9007199254740993}\r\n\r\n'
    elif url.path == "/v1/data-feeds/bit%2Fmex":
        assert peer.headers.get("Authorization") == "Bearer fixture"
        assert query["from"] == ["2024-02-29T23:59:00Z"] and query["sliceSize"] == ["1"]
        assert json.loads(query["filters"][0]) == [
            {"channel": "trade", "symbols": ["BTC/USD"]}
        ]
        offset = int(query["offset"][0])
        assert 0 <= offset <= 2
        raw = [b'1700 {"n":9007199254740993}\r\n\r\n', b"", b'1702 {"n":3}'][offset]
        data = gzip.compress(raw)
    else:
        assert url.path == "/v1/bit%2Fmex/trades/2024/02/29/BTC%2FUSD.csv.gz"
        assert peer.headers.get("range") == "bytes=0-3"
        data = DATASET[:4]
    peer.send_response(206 if peer.headers.get("range") else 200)
    if url.path.startswith("/v1/data-feeds/"):
        peer.send_header("Content-Encoding", "gzip")
    peer.send_header("Content-Length", str(len(data)))
    peer.end_headers()
    for i in range(0, len(data), 7):
        peer.wfile.write(data[i : i + 7])
