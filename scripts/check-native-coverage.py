#!/usr/bin/env python3
"""Verify API inventory pins and Rust/Python fixture parity offline.

Pass --reference-root PATH to additionally verify regeneration from external
schemas. Normal repository checks require no schema downloads.
"""
import argparse
import ast
import hashlib
import json
import re
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent


def require(condition, message):
    if not condition:
        raise SystemExit(message)


def check(venue):
    base = ROOT / 'crates/exchanges' / venue
    manifest = json.loads((base / 'coverage.json').read_text())
    pins = json.loads((base / 'API_SOURCES.json').read_text())
    require(pins['input_sha256']['openapi.json'] == manifest['spec_sha256'], f'{venue}: specification hash changed')
    inventory = sorted((row['verb'], row['path'], row['method']) for row in manifest['rest'])
    digest = hashlib.sha256(json.dumps(inventory, separators=(',', ':')).encode()).hexdigest()
    require(digest == pins['inventory_sha256'], f'{venue}: REST coverage differs from the pinned API inventory')
    actual = {(row['verb'], row['path']) for row in manifest['rest']}
    require(len(actual) == len(inventory) == pins['rest_operations'], f'{venue}: duplicate or missing REST operations')
    websocket = sorted((row['kind'], row['address'], row['rust'], row['python']) for row in manifest['websocket'])
    digest = hashlib.sha256(json.dumps(websocket, separators=(',', ':')).encode()).hexdigest()
    require(digest == pins['websocket_inventory_sha256'], f'{venue}: WebSocket inventory changed')
    require(len(websocket) == pins['websocket_entries'], f'{venue}: missing WebSocket entries')
    source = (base / 'src/operations.rs').read_text()
    native_binding = (ROOT / 'crates/python/src' / venue / 'operations.rs').read_text()
    wrapper = ast.parse((ROOT / 'crates/python/python/aux_py' / f'_{venue}.py').read_text())
    cls = next(node for node in wrapper.body if isinstance(node, ast.ClassDef) and node.name == venue.title() + 'Client')
    python_methods = {node.name for node in cls.body if isinstance(node, ast.AsyncFunctionDef)}
    fixtures = json.loads((base / 'tests/fixtures/contracts.json').read_text())
    tests = (base / 'tests/contracts.rs').read_text()
    methods = {row['method'] for row in manifest['rest']}
    require(methods == set(re.findall(r'pub async fn (\w+)\(', source)) == set(fixtures), f'{venue}: named Rust methods, inventory and fixtures differ')
    for row in manifest['rest']:
        method = row['method']
        require(f'pub async fn {method}(' in source, f'{venue}: missing named Rust method {method}')
        require(f'fn {method}<' in native_binding and method in python_methods, f'{venue}: missing typed Python binding {method}')
        require(method in fixtures and f'fn {row["test"]}()' in tests, f'{venue}: missing fixture/test for {method}')
    ws_fixtures = json.loads((base / 'tests/fixtures/ws.json').read_text())
    ws_tests = (base / 'tests/websocket_contracts.rs').read_text()
    for row in manifest['websocket']:
        require(row['fixture'].split('#')[-1] in ws_fixtures, f'{venue}: missing WebSocket fixture {row["address"]}')
        require(f'fn {row["test"]}()' in ws_tests, f'{venue}: missing WebSocket test {row["address"]}')
        cls_name, method = row['python'].split('.')
        python_source = ROOT/'crates/python/python/aux_py'/('_paradex_rpc.py' if cls_name == 'ParadexRpcSession' else f'_{venue}.py')
        classes = ast.parse(python_source.read_text()).body
        api = next(node for node in classes if isinstance(node, ast.ClassDef) and node.name == cls_name)
        require(any(isinstance(node, ast.AsyncFunctionDef) and node.name == method for node in api.body), f'{venue}: missing Python WebSocket API {row["python"]}')
    if venue == 'paradex':
        require(pins['input_sha256']['paradex_1_0.xml'] == manifest['sbe']['sha256'], 'Paradex: SBE schema hash changed')
        frames = json.loads((base / 'tests/fixtures/sbe.json').read_text())['frames']
        require({frame['template'] for frame in frames} == {row['id'] for row in manifest['sbe']['templates']}, 'Paradex: missing SBE reference templates')
    print(f'{venue}: {len(actual)} REST operations and {len(manifest["websocket"])} WebSocket entries covered')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reference-root', type=Path, help='add regeneration checks using external schema inputs')
    args = parser.parse_args()
    if args.reference_root is not None:
        for script in ['generate-native-apis.py', 'generate-paradex-sbe.py']:
            subprocess.run([sys.executable, str(ROOT / 'scripts' / script), '--reference-root', str(args.reference_root), '--check'], cwd=ROOT, check=True)
    for venue in ['derive', 'paradex']:
        check(venue)
