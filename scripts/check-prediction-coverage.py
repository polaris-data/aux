#!/usr/bin/env python3
"""Check pinned prediction REST, stream and contract coverage without network access.

External normalized schemas are optional regeneration inputs, never CI downloads.
"""
import argparse
import ast
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
VENUES = ('kalshi', 'polymarket', 'pascal')

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def require(value, message):
    if not value:
        raise SystemExit(message)

def check(venue, reference):
    base = ROOT / 'crates/exchanges' / venue
    pins = json.loads((base / 'API_SOURCES.json').read_text())
    coverage = json.loads((base / 'coverage.json').read_text())
    for relative, expected in pins['artifacts_sha256'].items():
        require(digest(base / relative) == expected, f'{venue}: changed pinned artifact: {relative}')
    if reference:
        for filename, expected in pins['normalized_sha256'].items():
            require(digest(reference / venue / filename) == expected, f'{venue}: normalized schema changed: {filename}')
        for filename, info in pins['sources'].items():
            candidate = reference / venue / filename
            if candidate.exists():
                require(digest(candidate) == info['sha256'], f'{venue}: raw source changed: {filename}')
    rest = coverage['rest']
    fixtures = json.loads((base / 'tests/fixtures/contracts.json').read_text())
    require(set(fixtures) == {r['service'] + '.' + r['method'] for r in rest}, f'{venue}: REST fixture inventory differs')
    for service in {r['service'] for r in rest}:
        source = (base / 'src' / (service + '.rs')).read_text()
        methods = set(re.findall(r'pub async fn (\w+)\(', source))
        require(methods == {r['method'] for r in rest if r['service'] == service}, f'{venue}/{service}: named Rust method inventory differs')
    tests = (base / 'tests/contracts.rs').read_text()
    for row in rest:
        wrapper = ROOT / 'crates/python/python/aux_py' / ('_' + re.sub(r'(?<!^)(?=[A-Z])', '_', row['client']).lower() + '.py')
        tree = ast.parse(wrapper.read_text())
        methods = {node.name for node in ast.walk(tree) if isinstance(node, ast.AsyncFunctionDef)}
        require(row['method'] in methods, f'{venue}: missing Python method: {row["method"]}')
        require('async fn ' + row['service'] + '_' + row['method'] + '()' in tests, f'{venue}: missing named Rust test: {row["method"]}')
        require(row['method'] in (ROOT / 'crates/python/python/aux_py/_native.pyi').read_text(), f'{venue}: missing Python stub')
    streams = coverage['websocket']
    fixtures = json.loads((base / 'tests/fixtures/ws.json').read_text())
    require(set(fixtures) == {r['fixture'] for r in streams}, f'{venue}: stream fixture inventory differs')
    declarations = (base / 'src/ws_types.rs').read_text()
    stream_tests = (base / 'tests/websocket_contracts.rs').read_text()
    for row in streams:
        require(row['rust'].split('::')[-1] in declarations, f'{venue}: missing native event model')
        require(json.dumps(row['fixture']) in stream_tests, f'{venue}: missing named event fixture test')
    if venue == 'polymarket':
        contracts = json.loads((base / 'schema/contracts.json').read_text())
        vectors = json.loads((base / 'tests/fixtures/contracts-abi.json').read_text())
        expected = {(r['interface'], r['signature'].split('(')[0].split()[-1]) for r in contracts}
        require(expected == {(r['interface'], r['function']) for r in vectors}, 'Polymarket: missing independent contract vector')
    print(f'{venue}: {len(rest)} REST operations, {len(streams)} stream messages; Rust/Python inventories and fixtures pinned')

if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--reference-root', type=Path)
    a = p.parse_args()
    for venue in VENUES:
        check(venue, a.reference_root)
    scripts = ['generate-polymarket-contracts.py', 'generate-prediction-python.py']
    if a.reference_root:
        scripts += ['generate-prediction-apis.py', 'generate-prediction-streams.py']
    for script in scripts:
        command = [sys.executable, str(ROOT / 'scripts' / script), '--check']
        if script in ['generate-prediction-apis.py', 'generate-prediction-streams.py']:
            command += ['--reference-root', str(a.reference_root)]
        subprocess.run(command, cwd=ROOT, check=True)
