"""Build an independent Zig consumer and run it against the SDK fixtures."""

import gzip
import json
import os
import re
import shutil
import subprocess
import tarfile
import tempfile
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from threading import Event, Thread
from urllib.parse import parse_qs, urlsplit

from tests.fixture_peer import fixture_peer
from tools.rest_models import SCHEMAS

ROOT = Path(__file__).resolve().parent


def wire_field(row, name):
    # Preserve the reference's original decimal and integer tokens.
    return row.get(name + "_json") or json.dumps(row.get(name))


def pair(group, operation, request, response):
    return f'{{"group":{json.dumps(group)},"operation":{json.dumps(operation)},"request":{request},"response":{response}}}'


def model_rows():
    files = {
        name: json.loads((ROOT / f"tests/{name}.json").read_text())
        for name in ("http", "data-http", "prediction-http", "text-http")
    }
    for group, schema in SCHEMAS.items():
        venue = schema.get("venue", group)
        covered = set()
        for row in files[schema.get("fixture", "http")] + files["text-http"]:
            op = row["operation"]
            name = op.rsplit(".", 1)[-1]
            if row["venue"] != venue or name not in schema["methods"]:
                continue
            if venue in ("kalshi", "polymarket") and (
                op.startswith(("perps.", "clearing.")) != group.endswith("_perps")
            ):
                continue
            request = wire_field(row, "request")
            response = wire_field(row, "response")
            if "expected" in row:
                response = json.dumps(row["expected"])
            elif schema.get("oracle"):
                if row.get("status", 200) >= 300:
                    response = json.dumps(row.get("response_json", row.get("body")))
                response = (
                    f'{{"headers":[],"status":{row["status"]},"data":{response}}}'
                )
            elif group == "hyperdash":
                response = f'{{"metadata":{{}},"status":200,"data":{response}}}'
            yield pair(group, op, request, response)
            covered.add(name)
            if group == "hyperdash":
                for suffix in ("_csv", "_ndjson"):
                    if name + suffix in schema["methods"]:
                        yield pair(
                            group,
                            op + suffix,
                            request,
                            '{"metadata":{},"status":200,"data":"sample text"}',
                        )
                        covered.add(name + suffix)
        if covered != set(schema["methods"]):
            raise ValueError(
                f"Uncovered {group} models: {sorted(set(schema['methods']) - covered)}"
            )


class Hermes(BaseHTTPRequestHandler):
    def do_GET(self):
        path = urlsplit(self.path)
        query = parse_qs(path.query)
        assert query.pop("ids[]") == ["42" * 32]
        if path.path.endswith("/stream"):
            assert not query and self.headers["accept"] == "text/event-stream"
            self.send_response(200)
            self.send_header("Content-Type", "text/event-stream")
            self.end_headers()
            for part in (b'data: {"pri', b'ce":9007199254740993}\r\n', b"\r\n"):
                self.wfile.write(part)
                self.wfile.flush()
            self.server.done.wait(20)
        else:
            assert path.path == "/v2/updates/price/latest" and query == {
                "parsed": ["true"]
            }
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b'{"price":9007199254740993}')

    def do_POST(self):
        request = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
        contract = self.path.removeprefix("/abi/")
        tx, block = request["params"]
        assert request["method"] == "eth_call" and block == "finalized"
        assert tx["to"] == "0x" + "11" * 20 and tx["value"] == "0x0"
        result = self.server.vectors[contract, tx["data"]]["output"]
        self.send_response(200)
        self.end_headers()
        self.wfile.write(
            json.dumps(
                {"jsonrpc": "2.0", "id": request["id"], "result": result}
            ).encode()
        )

    def log_message(self, *args):
        pass


def package(directory):
    manifest = (ROOT / "build.zig.zon").read_text()
    version = re.search(r'\.version\s*=\s*"([^"]+)"', manifest)[1]
    paths = re.search(r"\.paths\s*=\s*\.\{([^}]*)\}", manifest, re.DOTALL)[1]
    archive = directory / f"aux-zig-native-{version}.tar.gz"

    def reproducible(member):
        member.uid = member.gid = member.mtime = 0
        member.uname = member.gname = ""
        member.mode = 0o755 if member.isdir() else 0o644
        return member

    with (
        archive.open("wb") as raw,
        gzip.GzipFile(filename="", fileobj=raw, mode="wb", mtime=0) as compressed,
        tarfile.open(fileobj=compressed, mode="w") as tar,
    ):
        for path in sorted(re.findall(r'"([^"]+)"', paths)):
            tar.add(ROOT / path, arcname=path, filter=reproducible)
    return archive


def main():
    with tempfile.TemporaryDirectory() as tmp:
        out = Path(tmp)
        zig = os.environ.get("ZIG", "zig")
        archive = package(out)
        consumer = out / "consumer"
        shutil.copytree(
            ROOT / "tests/zig_consumer",
            consumer,
            ignore=shutil.ignore_patterns(".zig-cache", "zig-out", ".gitignore"),
        )
        # Use the same example sources as the path dependency, then build them
        # with only the fetched, hashed package present in the temporary project.
        manifest = consumer / "build.zig.zon"
        manifest.write_text(
            manifest.read_text().replace('.aux = .{ .path = "../.." }', "")
        )
        subprocess.run(
            [zig, "fetch", "--save=aux", archive.as_uri()], cwd=consumer, check=True
        )
        subprocess.run(
            [zig, "build", "--cache-dir", ROOT / ".zig-cache", "-p", out],
            cwd=consumer,
            check=True,
        )
        fixtures = out / "models.json"
        fixtures.write_text("[" + ",\n".join(model_rows()) + "]")
        subprocess.run([out / "bin/zig-models", fixtures], check=True, timeout=180)
        abi = json.loads((ROOT / "tests/metric-abi.json").read_text())
        vectors = {
            (r["contract"], r["signature"]): r for r in abi if r["kind"] == "function"
        }
        cases = []
        for line in (
            (ROOT / "src/exchanges/metric/schema/metric-contract-methods.tsv")
            .read_text()
            .splitlines()
        ):
            contract, signature, name, _, _ = line.split("\t")
            cases.append(dict(vectors[contract, signature], method=name))
        (out / "contracts.json").write_text(json.dumps(cases))
        with (
            fixture_peer() as (origin, gateway),
            ThreadingHTTPServer(("127.0.0.1", 0), Hermes) as server,
        ):
            server.vectors = {(r["contract"], r["input"]): r for r in cases}
            server.done = Event()
            thread = Thread(target=server.serve_forever)
            thread.start()
            try:
                subprocess.run(
                    [
                        out / "bin/zig-consumer",
                        f"http://127.0.0.1:{server.server_port}",
                        ROOT / "tests",
                        out,
                        origin.replace("http:", "ws:") + "/rpc/derive-public",
                        origin,
                        gateway,
                    ],
                    check=True,
                    timeout=120,
                )
            finally:
                server.done.set()
                server.shutdown()
                thread.join()
        dist = ROOT / "dist"
        dist.mkdir(exist_ok=True)
        shutil.copy2(archive, dist / archive.name)
        print(f"Verified Zig SDK package {archive.name}")


if __name__ == "__main__":
    main()
