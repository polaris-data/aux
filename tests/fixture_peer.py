"""Shared loopback peer for the three packaged SDK consumers."""

from contextlib import contextmanager
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import base64
import time
import gzip
from urllib.parse import parse_qs, urlsplit
from decimal import Decimal
from pathlib import Path
from threading import Thread
import sys
from socketserver import ThreadingTCPServer, StreamRequestHandler

HERE = Path(__file__).resolve().parent
TEXT = {r["path"]: r for r in json.loads((HERE / "text-http.json").read_text())}
MONACO_PRIVATE = [r for r in json.loads((HERE/"data-http.json").read_text()) if r["venue"]=="monaco" and r["private"]]
DUNE = [r for r in json.loads((HERE / "data-http.json").read_text()) if r["venue"] == "dune"]
METRIC_POSTS = [r for r in json.loads((HERE / "data-http.json").read_text()) if r["venue"] == "metric" and r["verb"] == "POST"]
EVM = json.loads((HERE / "evm-signing.json").read_text())[0]


class Peer(BaseHTTPRequestHandler):
    def dune_request(self):
        path = urlsplit(self.path.removeprefix("/dune-args"))
        if path.path.endswith("/csv"):
            self.path = self.path.removeprefix("/dune-args")
            return self.do_GET()
        row = next(r for r in DUNE if r["path"] == path.path)
        assert self.command == row["verb"]
        assert self.headers["X-Dune-API-Key"] == "test-key"
        assert parse_qs(path.query) == {k:[v] for k,v in row["query"].items()}
        length = int(self.headers.get("content-length", 0))
        body = json.loads(self.rfile.read(length), parse_float=Decimal) if length else None
        expected = json.loads(row["body_json"], parse_float=Decimal) if row["body_json"] else None
        assert body == expected
        self.send_response(200)
        self.end_headers()
        self.wfile.write(row["response_json"].encode())

    def do_GET(self):
        if self.path == "/v2/exchangeInfo":
            assert self.headers["xrestservermm"] == "restserver7"
            response = b'{"symbols":[{"symbol":"BTC-PERP","priceDecimals":2,"tickSizes":["0.01"]}]}'
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(response)))
            self.end_headers()
            self.wfile.write(response)
            return
        if self.path.startswith("/caller-websocket/"):
            from .raw_websocket_peer import serve
            return serve(self)
        if self.path.startswith("/native-exchanges/"):
            from .native_exchange_peer import serve
            return serve(self)
        if self.path.startswith("/new-exchanges/"):
            from .exchange_peer import serve
            return serve(self)
        if self.path.startswith("/perps/"):
            from .perps_peer import serve
            return serve(self)
        if self.path.startswith("/stream-refused/"):
            if self.path == "/stream-refused/ws":
                assert self.headers["Upgrade"].lower() == "websocket"
                nested = {"message": "subscription denied"}
            else:
                assert self.path.startswith("/stream-refused/v2/updates/price/stream?")
                nested = {"message": "invalid " + self.headers["Authorization"]}
            body = json.dumps({"seq":9007199254740993, **nested}).encode()
            self.send_response(403)
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
            return
        if self.path.startswith("/dune-args/"):
            return self.dune_request()
        if self.path.startswith("/rejected-feed/"):
            from .trading_peer import upgrade
            from .ws_server import frame, read_frame
            upgrade(self)
            opcode, raw = read_frame(self.rfile)
            assert opcode == 1
            message = json.loads(raw)
            assert "channels" in message or message.get("channel") == "order_book/4"
            error = {"error": {"sequence": 9007199254740993, "message": "unknown market"}}
            self.connection.sendall(frame(1, json.dumps(error).encode()))
            return
        if self.path.startswith("/account-scope/"):
            from .multiplex_peer import serve_hyperliquid_accounts
            return serve_hyperliquid_accounts(self)
        if self.path.startswith("/qfex-multiplex/"):
            from .multiplex_peer import serve_qfex
            return serve_qfex(self)
        if self.path == "/qfex-candles/invalid":
            raise AssertionError("Invalid QFEX candle subscription reached the peer")
        if self.path == "/qfex-candles/zig":
            from .multiplex_peer import serve_qfex_candles
            return serve_qfex_candles(self)
        if self.path.startswith("/lighter-multiplex/"):
            from .multiplex_peer import serve_lighter
            return serve_lighter(self)
        if self.path.startswith("/multiplex/"):
            from .multiplex_peer import serve
            return serve(self)
        if self.path.startswith("/lifecycle/"):
            from .trading_peer import upgrade
            from .ws_server import frame, read_frame
            upgrade(self)
            opcode, raw = read_frame(self.rfile)
            assert opcode == 1 and json.loads(raw) == {"method":"subscribe","subscription":{"type":"l2Book","coin":"ETH"}}
            for msg in ({"channel":"subscriptionResponse","data":json.loads(raw)},
                        {"channel":"l2Book","data":{"coin":"ETH","time":23,"levels":[[],[]]}}):
                self.connection.sendall(frame(1,json.dumps(msg).encode()))
            # EOF or a close frame releases the peer as the client closes the subscription.
            self.connection.settimeout(8)
            self.connection.recv(2048)
            return
        if self.path.startswith("/external-kalshi/"):
            from .trading_peer import verify_ed25519
            path, query = self.path.split("?")
            assert path == "/external-kalshi/portfolio/balance" and query == "subaccount=1"
            stamp = self.headers["KALSHI-ACCESS-TIMESTAMP"]
            assert abs(time.time()*1000-int(stamp)) < 30000
            assert self.headers["KALSHI-ACCESS-KEY"] == "fixture-k"
            verify_ed25519("ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c",
                           stamp + "GET" + path,base64.b64decode(self.headers["KALSHI-ACCESS-SIGNATURE"],validate=True).hex())
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b'{"balance":123}')
            return
        if self.path.startswith("/rpc/"):
            from .rpc_peer import serve
            return serve(self)
        if self.path.startswith("/shared/"):
            from .report_book_peer import shared_reports
            return shared_reports(self)
        if self.path == "/hd/stream":
            from .report_book_peer import hyperdash
            return hyperdash(self)
        if self.path == "/arcus/ws" or self.path.startswith("/reports/api/v1/ws?"):
            from .report_book_peer import serve
            return serve(self)
        if self.path == "/public/v1/chains":
            key = self.headers.get("authorization")
            assert key in ("Bearer fixture", "Bearer last", "Bearer fail")
            self.protocol_version = "HTTP/1.1"
            self.close_connection = key != "Bearer fixture"
            body = str(self.client_address[1]).encode() if key != "Bearer fail" else b"fixture rate limit"
            self.send_response(429 if key == "Bearer fail" else 200)
            self.send_header("Content-Length", str(len(body)))
            if key != "Bearer fixture":
                self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(body)
            return
        if self.path.startswith(("/api/v1/nextNonce?", "/api/v1/accountActiveOrders?")):
            from .lighter_peer import serve
            return serve(self)
        if self.path.startswith("/pro/") or self.path == "/v1/merkle/root/stream":
            from .trading_peer import pyth
            return pyth(self)
        if self.path.startswith(("/machine/","/v1/data-feeds/","/v1/bit%2Fmex/")):
            from .tardis_peer import serve
            return serve(self)
        if self.path.startswith("/monaco-ws/"):
            from .trading_peer import monaco_subscription
            return monaco_subscription(self)
        if self.path.startswith("/perps-ws/"):
            from .trading_peer import perps_subscription
            return perps_subscription(self)
        if self.path.startswith("/poly-ws/"):
            from .trading_peer import polymarket
            return polymarket(self)
        if self.path.startswith("/trade?") or self.path.startswith("/api/v1/market/pairs/") or self.path.split("?")[0] in {r["path"] for r in MONACO_PRIVATE}:
            from . import trading_peer
            if self.path.startswith("/trade?"):
                return trading_peer.qfex(self)
            return trading_peer.monaco(self)
        if self.path.startswith("/files/"):
            path, query = self.path.split("?")
            assert query == "sig=a%2Fb"
            if path == "/files/storage":
                assert self.headers.get("Authorization") is None
            else:
                key = json.loads((HERE / "auth.json").read_text())["databento"]["key"]
                assert self.headers["Authorization"] == "Basic " + base64.b64encode((key + ":").encode()).decode()
            if path == "/files/redirect":
                self.send_response(302)
                self.send_header("Location", "/files/batch?sig=a%2Fb")
                self.end_headers()
                return
            data = (HERE / "reference-lines.zst").read_bytes()
            self.send_response(200)
            self.send_header("Content-Type", "application/octet-stream")
            self.send_header("Content-Encoding", "zstd")
            self.send_header("Content-Length", str(len(data) + (1 if path == "/files/partial" else 0)))
            self.end_headers()
            self.wfile.write(data)
            return
        if self.path.startswith("/api/v1/streaming?symbol="):
            assert self.headers["Accept"] == "application/json"
            assert self.headers["Authorization"] == "Bearer test-key"
            self.send_response(200)
            self.end_headers()
            symbol = self.path.split("=")[1]
            if symbol == "TRUNCATED":
                self.wfile.write(b'{"p":')
            elif symbol == "INVALID":
                self.wfile.write(b'{"p":[}')
            else:
                assert symbol == "ETHUSD%2CBTCUSD", symbol
                for chunk in (b' {"heartbeat":174852', b'5528}{"p":1.0000000000000000001e+21,"f":"', b'\\"}{","items":[{},[]]}\n'):
                    self.wfile.write(chunk)
                    self.wfile.flush()
            return
        row = TEXT[self.path]
        assert self.headers["Accept"] == row["accept"]
        if row["venue"] == "dune":
            assert self.headers["X-Dune-API-Key"] == "test-key"
        self.send_response(200)
        for key, value in row["headers"].items():
            self.send_header(key, value)
        self.end_headers()
        self.wfile.write(row["body"].encode())

    def do_DELETE(self):
        if self.path.startswith("/native-exchanges/"):
            from .native_exchange_peer import serve
            return serve(self)
        if self.path.startswith("/new-exchanges/"):
            from .exchange_peer import serve
            return serve(self)
        if self.path.startswith("/perps/"):
            from .perps_peer import serve
            return serve(self)
        self.send_error(404)

    def do_PUT(self):
        if self.path.startswith("/native-exchanges/"):
            from .native_exchange_peer import serve
            return serve(self)
        if self.path.startswith("/perps/"):
            from .perps_peer import serve
            return serve(self)
        from .trading_peer import monaco
        return monaco(self)

    def do_POST(self):
        if self.path.startswith("/native-exchanges/"):
            from .native_exchange_peer import serve
            return serve(self)
        if self.path.startswith("/new-exchanges/"):
            from .exchange_peer import serve
            return serve(self)
        if self.path.startswith("/perps/"):
            from .perps_peer import serve
            return serve(self)
        if self.path.startswith("/dune-args/"):
            return self.dune_request()
        if self.path.startswith("/poly-contract/"):
            req = json.loads(self.rfile.read(int(self.headers["content-length"])))
            vector = next(v for v in json.loads((HERE/"polymarket-abi.json").read_text()) if v["function"]=="allowance")
            assert req["method"]=="eth_call" and req["params"][0]["data"]==vector["calldata"] and req["params"][1]=="safe"
            self.send_response(200)
            self.end_headers()
            self.wfile.write(json.dumps({"id":req["id"],"jsonrpc":"2.0","result":f"0x{2**88+9:064x}"}).encode())
            return
        if self.path.startswith("/metric-contract/"):
            req = json.loads(self.rfile.read(int(self.headers["content-length"])))
            assert req["method"] == "eth_call"
            payload, block = req["params"]
            assert block == "safe" and payload["data"] == "0x9eaa1b09"
            result = "0x" + "ff"*31 + "fd"
            self.send_response(200)
            self.end_headers()
            self.wfile.write(json.dumps({"id":req["id"],"jsonrpc":"2.0","result":result}).encode())
            return
        if self.path.startswith("/metric-chain/"):
            row = json.loads(Path(__file__).with_name("metric-chain.json").read_text())
            request = json.loads(self.rfile.read(int(self.headers["content-length"])))
            if request["method"] == "eth_blockNumber":
                result = row["blockNumber"]
            elif request["method"] == "eth_getBlockByNumber":
                result = {"number":row["blockNumber"]}
            else:
                assert request["method"] == "eth_call" and request["params"][1] == row["blockNumber"]
                call = request["params"][0]
                result = next(c["result"] for c in row["calls"] if c["to"].lower() == call["to"].lower() and c["data"] == call["data"])
            self.send_response(200)
            self.end_headers()
            self.wfile.write(json.dumps({"id":request["id"],"jsonrpc":"2.0","result":result}).encode())
            return
        if self.path in ("/v1/auth", "/v2/auth", "/v1/auth/0x4321"):
            assert self.headers["PARADEX-STARKNET-ACCOUNT"] == "0x1234"
            if self.path == "/v2/auth":
                message = base64.b64decode(self.headers["PARADEX-SIWE-MESSAGE"]).decode()
                assert "app.testnet.paradex.trade wants you to sign in" in message
                assert "Paradex Auth" in message and len(self.headers["PARADEX-EVM-SIGNATURE"]) == 132
            else:
                t = int(self.headers["PARADEX-TIMESTAMP"])
                assert abs(time.time()-t)<30 and int(self.headers["PARADEX-SIGNATURE-EXPIRATION"]) == t+300
                assert len(json.loads(self.headers["PARADEX-STARKNET-SIGNATURE"])) == 2
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b'{"jwt_token":"fixture.eyJleHAiOjQxMDI0NDQ4MDB9.fixture"}')
            return
        if self.path in ("/v1/placeOrder", "/v1/cancelOrder"):
            from .report_book_peer import arcus_order
            return arcus_order(self)
        if self.path.startswith("/public/v1/evm/"):
            row = next(r for r in METRIC_POSTS if r["path"] == self.path)
            assert self.headers["authorization"] == "Bearer fixture"
            assert self.headers["content-type"] == "application/json"
            assert json.loads(self.rfile.read(int(self.headers["content-length"]))) == row["body"]
            self.send_response(200)
            self.end_headers()
            self.wfile.write(json.dumps(row["response"]).encode())
            return
        if self.path.startswith(("/api/v1/orders","/api/v1/auth/")):
            from .trading_peer import monaco
            return monaco(self)
        if self.path == "/api/v1/sendTx":
            from .lighter_peer import serve
            return serve(self)
        if self.path == "/exchange":
            from .trading_peer import hyperliquid
            return hyperliquid(self)
        if self.path.startswith("/v0/"):
            raw = self.rfile.read(int(self.headers["Content-Length"]))
            params = parse_qs(raw.decode())
            key = json.loads((HERE / "auth.json").read_text())["databento"]["key"]
            assert self.headers["authorization"] == "Basic " + base64.b64encode((key+":").encode()).decode()
            assert self.headers["content-type"] == "application/x-www-form-urlencoded"
            assert params["symbols"] == ["ETH,BTC"]
            self.send_response(200)
            if self.path == "/v0/timeseries.get_range":
                assert self.headers["accept"] == "application/octet-stream"
                if params["encoding"] == ["dbn"]:
                    assert params["compression"] == ["none"]
                    self.send_header("content-encoding", "gzip")
                    body = gzip.compress((HERE/"legacy-v1.dbn").read_bytes())
                else:
                    assert params["encoding"] == ["csv"] and params["compression"] == ["zstd"]
                    self.send_header("content-encoding","zstd")
                    body = (HERE/"reference-lines.zst").read_bytes()
            else:
                routes = json.loads((HERE/"databento-streams.json").read_text())
                assert self.path in [r["path"] for r in routes if r["client"]=="ReferenceClient"] and self.headers["accept"] == "application/json"
                if params["compression"] == ["zstd"]:
                    body = (HERE/"reference-lines.zst").read_bytes()
                    if self.path.endswith(".get_last"):
                        self.send_header("content-encoding","zstd")
                else:
                    body = b'{"id":9007199254740993,"price":1.25000000000000000001}\r\n{"id":2}'
                if params.get("schema") == ["invalid"]:
                    body += b'\n{'
            self.end_headers()
            for i in range(0,len(body),11):
                self.wfile.write(body[i:i+11])
            return
        payload = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
        assert payload["jsonrpc"] == "2.0"
        method = payload["method"]
        if self.path.startswith("/phoenix-rpc/"):
            rows = json.loads((HERE/"phoenix-transactions.json").read_text())
            row = rows[0]
            signed = row["signed"]
            args = payload["params"]
            if method == "getLatestBlockhash":
                assert args == [{"commitment":"confirmed"}]
                value = {"value":{"blockhash":row["request"]["recent_blockhash"],"lastValidBlockHeight":500}}
            elif method == "getBlockHeight":
                assert args == [{"commitment":"confirmed"}]
                value = 400
            elif method == "simulateTransaction":
                assert args == [signed["transaction_base64"],{"encoding":"base64","sigVerify":True,"commitment":"confirmed"}]
                value = {"value":{"err":None}}
            elif method == "sendTransaction":
                assert args == [signed["transaction_base64"],{"encoding":"base64","skipPreflight":False,"maxRetries":0}]
                if self.path.endswith("/lost"):
                    self.close_connection = True
                    return
                value = signed["signature"]
            elif method == "getSignatureStatuses":
                assert args == [[signed["signature"]],{"searchTransactionHistory":True}]
                value = {"value":[{"confirmationStatus":"confirmed"}]}
            elif method == "getTransaction":
                assert args == [signed["signature"],{"encoding":"json","commitment":"confirmed","maxSupportedTransactionVersion":0}]
                value = {"slot":9007199254740993}
            else:
                raise AssertionError(method)
        elif method == "eth_chainId":
            assert self.path in ("/read", "/write")
            value = "0x1"
        elif method == "eth_getTransactionCount":
            assert self.path == "/read" and payload["params"] == [EVM["address"], "pending"]
            value = hex(EVM["prepared"]["nonce"])
        elif method == "eth_getCode":
            assert self.path == "/read" and payload["params"] == [EVM["prepared"]["to"], "latest"]
            value = "0x6000"
        elif method == "eth_sendRawTransaction":
            assert self.path == "/write" and payload["params"] == [EVM["signed"]]
            value = EVM["hash"]
        elif method == "getBlockHeight":
            assert self.path == "/solana"
            assert payload["params"] == [{"commitment": "confirmed"}]
            value = 987654321
        else:
            raise AssertionError(method)
        self.send_response(200)
        self.end_headers()
        self.wfile.write(json.dumps({"jsonrpc": "2.0", "id": payload["id"], "result": value}).encode())

    def log_message(self, *_):
        pass


class LiveGateway(StreamRequestHandler):
    def handle(self):
        self.connection.settimeout(15)
        auth = json.loads((HERE / "auth.json").read_text())["databento"]
        self.wfile.write(b"lsg-version=1\r\ncram=")
        self.wfile.write(auth["challenge"].encode() + b"\n")
        line = self.rfile.readline().decode().rstrip("\n")
        fields = dict(pair.split("=", 1) for pair in line.split("|"))
        assert fields["auth"] == auth["response"]
        assert fields["encoding"] == "dbn" and fields["ts_out"] == "0"
        assert fields["client"].startswith("aux-zig/")
        if fields["dataset"] == "IDLE":
            self.connection.settimeout(2)
            assert self.rfile.read(1) == b"", "idle reader should close before handshake completes"
            return
        if fields["dataset"] == "DENY":
            self.wfile.write(b"success=0|error=fixture denial\n")
            return
        self.wfile.write(b"success=1|session_id=fixture-session\r\n")
        subscriptions = []
        while True:
            line = self.rfile.readline()
            if line == b"start_session\n":
                break
            if not line:
                return
            subscriptions.append(line.decode().rstrip("\n"))
        if fields["dataset"] == "BATCH":
            symbols = ",".join(str(i) for i in range(500))
            assert subscriptions == [
                f"schema=mbo|stype_in=raw_symbol|symbols={symbols}|snapshot=0|is_last=0|start=1741000000000000000|id=7",
                "schema=mbo|stype_in=raw_symbol|symbols=500|snapshot=0|is_last=1|start=1741000000000000000|id=7",
                "unsubscribe|schema=mbo|stype_in=raw_symbol|symbols=500|is_last=1",
            ]
            assert fields["heartbeat_interval_s"] == "3" and fields["slow_reader_behavior"] == "warn"
        if fields["dataset"] == "BLOCK":
            assert self.rfile.read(1) == b""
            return
        payload = (HERE / ("schema-records.dbn.zst" if fields["compression"] == "zstd" else "schema-records.dbn")).read_bytes()
        for i in range(0, len(payload), 11):
            self.wfile.write(payload[i:i+11])


@contextmanager
def fixture_peer():
    # Clients deliberately exercise lost acknowledgements. A failing assertion in
    # a handler must fail the package test, even if EOF was the expected response.
    failures = []
    def record_error(request, address):
        error = sys.exc_info()[1]
        if not isinstance(error, (BrokenPipeError, ConnectionResetError)):
            failures.append(error)

    with ThreadingHTTPServer(("127.0.0.1", 0), Peer) as peer, ThreadingTCPServer(("127.0.0.1", 0), LiveGateway) as live:
        for server in (peer, live):
            server.daemon_threads = False
            server.handle_error = record_error
        threads = [Thread(target=server.serve_forever, daemon=True) for server in (peer,live)]
        for thread in threads:
            thread.start()
        try:
            yield f"http://127.0.0.1:{peer.server_port}", f"127.0.0.1:{live.server_address[1]}"
        finally:
            peer.shutdown()
            live.shutdown()
            for thread in threads:
                thread.join()
    if failures:
        raise AssertionError("loopback peer failed a protocol check") from failures[0]


if __name__ == "__main__":
    import argparse
    import os
    import subprocess

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cwd", type=Path)
    parser.add_argument("command", nargs="+")
    args = parser.parse_args()
    with fixture_peer() as (origin, gateway):
        env = {**os.environ, "AUX_FIXTURE_ORIGIN": origin, "AUX_DBN_GATEWAY": gateway}
        result = subprocess.run(args.command, cwd=args.cwd, env=env)
    sys.exit(result.returncode)
