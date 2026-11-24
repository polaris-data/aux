"""Mutual TLS fixture. The test process holds stdin open for the peer's lifetime."""
from http.server import ThreadingHTTPServer
from pathlib import Path
import ssl
import sys
from threading import Thread

here = Path(__file__).resolve().parent
sys.path.insert(0, str(here.parents[1]))
from tests.fixture_peer import Peer

context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
context.set_alpn_protocols(["http/1.1"])
context.load_cert_chain(here / "server.pem", here / "server-key.pem")
context.load_verify_locations(here / "ca.pem")
context.verify_mode = ssl.CERT_REQUIRED
with ThreadingHTTPServer(("127.0.0.1", 0), Peer) as peer:
    peer.socket = context.wrap_socket(peer.socket, server_side=True)
    thread = Thread(target=peer.serve_forever, daemon=True)
    thread.start()
    print(peer.server_port, flush=True)
    sys.stdin.buffer.read()
    peer.shutdown()
    thread.join()
