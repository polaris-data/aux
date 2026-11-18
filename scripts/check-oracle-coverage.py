#!/usr/bin/env python3
"""Check oracle operation, fixture, and SDK parity without contacting providers."""
import ast
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
api = json.loads((ROOT / 'crates/node/src/api.json').read_text())
def python_methods(relative):
    syntax = ast.parse((ROOT / 'crates/python/python/aux_py' / relative).read_text())
    return {cls.name: {method.name for method in cls.body
                       if isinstance(method, (ast.FunctionDef, ast.AsyncFunctionDef))}
            for cls in syntax.body if isinstance(cls, ast.ClassDef)}


wrappers = python_methods('_oracles.py')
stubs = python_methods('_native.pyi')

for vendor in ('pyth', 'chainlink'):
    base = ROOT / 'crates/data' / vendor
    pins = json.loads((base / 'API_SOURCES.json').read_text())
    assert len(pins['sources']) == len(pins['source_sha256']), vendor
    for relative, digest in {'operations.json': pins['inventory_sha256'], **pins['fixture_sha256']}.items():
        assert hashlib.sha256((base / relative).read_bytes()).hexdigest() == digest, (vendor, relative)
    operations = json.loads((base / 'operations.json').read_text())
    fixtures = json.loads((base / 'tests/fixtures/operations.json').read_text())
    keys = [(row['client'], row['method'], row['kind']) for row in operations]
    assert len(set(keys)) == len(keys), vendor
    assert set(keys) == {(row['client'], row['method'], row['kind']) for row in fixtures}, vendor
    source = (base / 'src/operations.rs').read_text()
    for client, name, _ in keys:
        assert re.search(rf'pub async fn {name}\b', source), (client, name)
        assert name in wrappers[client] and name in stubs[client], (client, name)
        assert name in {row['name'] for row in api['classes'][client]['methods']}, (client, name)
    if vendor == 'chainlink':
        versions = {row['version'] for row in json.loads((base / 'tests/fixtures/reports.json').read_text())}
        assert versions == {2, 3, 7, 8, 9, 10, 11, 14, 42}
    print(f'{vendor}: {len(keys)} operations match Rust, Python, Node and offline fixtures')
