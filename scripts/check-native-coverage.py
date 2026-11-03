#!/usr/bin/env python3
"""Verify pinned API inventories, named Rust/Python methods, and fixture coverage."""
import ast
import hashlib
import json
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
    provenance = json.loads((base / 'reference/provenance.json').read_text())
    for record in provenance['files']:
        require(hashlib.sha256((base / record['file']).read_bytes()).hexdigest() == record['sha256'], f'{venue}: pinned reference changed: {record["file"]}')
    raw = (base / 'reference/openapi.json').read_bytes()
    require(hashlib.sha256(raw).hexdigest() == manifest['spec_sha256'], f'{venue}: specification hash changed')
    spec = json.loads(raw)
    supplement = base / 'reference/supplemental.json'
    if supplement.exists():
        for path, methods in json.loads(supplement.read_text())['paths'].items():
            spec['paths'].setdefault(path, {}).update(methods)
    expected = {(method, path) for path, methods in spec['paths'].items() for method in methods if method in ('get', 'post', 'put', 'delete')}
    actual = {(row['verb'], row['path']) for row in manifest['rest']}
    require(expected == actual, f'{venue}: REST coverage differs from the pinned API union')
    source = (base / 'src/operations.rs').read_text()
    native_binding = (ROOT / 'crates/python/src' / venue / 'operations.rs').read_text()
    wrapper = ast.parse((ROOT / 'crates/python/python/aux_py' / f'_{venue}.py').read_text())
    cls = next(node for node in wrapper.body if isinstance(node, ast.ClassDef) and node.name == venue.title() + 'Client')
    python_methods = {node.name for node in cls.body if isinstance(node, ast.AsyncFunctionDef)}
    fixtures = json.loads((base / 'tests/fixtures/contracts.json').read_text())
    tests = (base / 'tests/contracts.rs').read_text()
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
        schema = (base / 'reference/paradex_1_0.xml').read_bytes()
        require(hashlib.sha256(schema).hexdigest() == manifest['sbe']['sha256'], 'Paradex: SBE schema hash changed')
        frames = json.loads((base / 'tests/fixtures/sbe.json').read_text())['frames']
        require({frame['template'] for frame in frames} == {row['id'] for row in manifest['sbe']['templates']}, 'Paradex: missing SBE reference templates')
    print(f'{venue}: {len(actual)} REST operations and {len(manifest["websocket"])} WebSocket entries covered')


if __name__ == '__main__':
    for script in ['generate-native-apis.py', 'generate-paradex-sbe.py']:
        subprocess.run([sys.executable, str(ROOT / 'scripts' / script), '--check'], cwd=ROOT, check=True)
    for venue in ['derive', 'paradex']:
        check(venue)
