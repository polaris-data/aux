"""Verify that every compiled REST operation has a fixture and that SDK names are current."""

from pathlib import Path
import json
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
groups = ("http", "prediction-http", "data-http", "text-http")
fixtures = {
    (r["venue"], r["operation"])
    for group in groups
    for r in json.loads((ROOT / f"tests/{group}.json").read_text())
}
# Exercised with split records and a blocked-reader cancellation in the three installed SDKs.
http_streams = {("pyth_hermes", "stream_price_updates"), ("chainlink_candles", "stream_prices")}
http_streams.update(("databento", r["operation"]) for r in json.loads((ROOT / "tests/databento-streams.json").read_text()))
fixtures.update(http_streams)
lines = (ROOT / "src/routes.zig").read_text().splitlines()
routes = [
    re.search(r'\.venue = \.(\w+), \.name = "([^"]+)"', s).groups()
    for s in lines
    if ".venue = ." in s
]
if set(routes) != fixtures or len(set(routes)) != len(routes):
    sys.exit(f"HTTP catalog and fixtures disagree: {sorted(set(routes) ^ fixtures)}")
subprocess.run(
    [sys.executable, ROOT / "tools/generate_clients.py", "--check"], check=True
)
exports = (ROOT / "src/hyperdash-text-routes.txt").read_text().splitlines()
if len(set(exports)) != len(exports) or any(("hyperdash", name) not in fixtures for name in exports):
    sys.exit("Hyperdash export variants must refer to unique fixture-backed routes")
unary = len(routes) - len(http_streams)
variants = 2 * len(exports)
models = sum(len(json.loads(path.read_text())["methods"]) for path in (ROOT / "schema").glob("*.json"))
print(f"{unary} unary routes, {variants} Hyperdash export variants and {len(http_streams)} HTTP streams covered")
print(f"{models} of {unary + variants} named unary methods have schema models")
