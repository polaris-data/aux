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


def check_metric():
    base = ROOT / 'crates/exchanges/metric'
    pins = json.loads((base / 'API_SOURCES.json').read_text())
    coverage = json.loads((base / 'coverage.json').read_text())
    digest = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    for field in ['snapshot_sha256', 'fixture_sha256']:
        for path, expected in pins[field].items():
            require(digest(base / path) == expected, f'Metric: pinned artifact changed: {path}')
    require(digest(base / 'contracts.json') == pins['inventory_sha256'], 'Metric: contract inventory changed')
    require(digest(base / 'coverage.json') == pins['coverage_sha256'], 'Metric: coverage manifest changed')
    contracts = json.loads((base / 'contracts.json').read_text())
    require(coverage['contracts'] == contracts, 'Metric: contract inventories differ')
    vectors = json.loads((base / 'tests/fixtures/abi.json').read_text())
    expected = {(r['contract'], r['kind'], r['signature']) for r in contracts if r['kind'] in ['function', 'event', 'error']}
    actual = {(r['contract'], r['kind'], r['signature']) for r in vectors}
    require(expected == actual and len(vectors) == len(expected), 'Metric: ABI fixture coverage differs from inventory')
    native = (base / 'src/client.rs').read_text()
    binding = (ROOT / 'crates/python/src/metric.rs').read_text()
    wrapper = ast.parse((ROOT / 'crates/python/python/aux_py/_metric.py').read_text())
    cls = next(n for n in wrapper.body if isinstance(n, ast.ClassDef) and n.name == 'MetricClient')
    methods = {n.name for n in cls.body if isinstance(n, ast.AsyncFunctionDef)}
    for row in coverage['rest']:
        method = row['method']
        require(re.search(r'pub async fn ' + method + r'\b', native) is not None, f'Metric: missing Rust HTTP method {method}')
        require(re.search(r'fn ' + method + r'\b', binding) is not None and method in methods, f'Metric: missing Python HTTP method {method}')
    require(len(coverage['rest']) == pins['http_operations'] == 8, 'Metric: HTTP inventory changed')
    tree = ast.parse((ROOT / 'crates/python/python/aux_py/_metric_contracts.py').read_text())
    classes = {c.name: {m.name for m in c.body if isinstance(m, (ast.FunctionDef, ast.AsyncFunctionDef))} for c in tree.body if isinstance(c, ast.ClassDef)}
    for row in contracts:
        if 'python' in row:
            cls, method = row['python'].split('.')
            require(method in classes.get(cls, set()), f'Metric: missing Python contract method {cls}.{method}')
    helpers = ast.parse((ROOT / 'crates/python/python/aux_py/metric.py').read_text())
    require(set(coverage['python_helpers']) == {n.name for n in helpers.body if isinstance(n, ast.FunctionDef)}, 'Metric: helper inventory changed')
    for method in coverage['python_helpers']:
        require('"' + method + '" =>' in binding, f'Metric: Python helper missing native implementation: {method}')
    helper_vectors = json.loads((base / 'tests/fixtures/helpers.json').read_text())
    require({k: len(v) for k, v in helper_vectors.items()} == pins['sdk_workflow_groups'], 'Metric: SDK workflow fixture inventory changed')
    require({row['group'] for row in coverage['sdk_workflows']} == set(helper_vectors), 'Metric: SDK workflow coverage groups differ')
    helper_tests = (base / 'tests/helpers.rs').read_text()
    python_tests = (ROOT / 'crates/python/tests/test_metric.py').read_text()
    chain_class = next(n for n in wrapper.body if isinstance(n, ast.ClassDef) and n.name == 'MetricChainClient')
    chain_methods = {n.name for n in chain_class.body if isinstance(n, ast.AsyncFunctionDef)}
    for row in coverage['sdk_workflows']:
        require(row['fixture'] == 'tests/fixtures/helpers.json#' + row['group'], 'Metric: incorrect SDK fixture mapping')
        require(f"fn {row['rust_test']}()" in helper_tests and f"def {row['python_test']}(" in python_tests, 'Metric: missing SDK workflow tests')
        for native in row['rust']:
            module, method = native.split('::')
            source = (base / 'src' / ('stop_loss.rs' if module == 'MetricChainClient' else module + '.rs')).read_text()
            require(re.search(r'pub (?:async )?fn ' + method + r'\b', source) is not None, f'Metric: missing SDK workflow implementation {native}')
        for method in row['python']:
            if method.startswith('MetricChainClient.'):
                name = method.split('.')[1]
                require(name in chain_methods and re.search(r'fn ' + name + r'\b', binding), f'Metric: missing SDK chain workflow {method}')
            else:
                require(method in coverage['python_helpers'], f'Metric: missing SDK helper {method}')
    require(digest(ROOT / 'scripts/generate-metric-helper-fixtures.mjs') == pins['reference_tools']['helper_generator_sha256'], 'Metric: SDK reference generator changed')
    subprocess.run([sys.executable, str(ROOT / 'scripts/generate-metric-bindings.py'), '--check'], cwd=ROOT, check=True)
    workflow_count = sum(len(rows) for rows in helper_vectors.values())
    print(f"metric: 8 HTTP operations, {len(vectors)} ABI codec vectors and {workflow_count} SDK workflow reference cases covered")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--reference-root', type=Path, help='add regeneration checks using external schema inputs')
    parser.add_argument('--metric-sdk-root', type=Path, help='compare helper fixtures with an extracted pinned Metric SDK package')
    args = parser.parse_args()
    if args.reference_root is not None:
        for script in ['generate-native-apis.py', 'generate-paradex-sbe.py']:
            subprocess.run([sys.executable, str(ROOT / 'scripts' / script), '--reference-root', str(args.reference_root), '--check'], cwd=ROOT, check=True)
    for venue in ['derive', 'paradex']:
        check(venue)
    check_metric()
    if args.metric_sdk_root is not None:
        package = json.loads((args.metric_sdk_root / 'package.json').read_text())
        sdk = json.loads((ROOT / 'crates/exchanges/metric/API_SOURCES.json').read_text())['sdk']
        require(package['name'] == sdk['package'] and package['version'] == sdk['version'], 'Metric: SDK package version differs from the pin')
        subprocess.run(['node', str(ROOT / 'scripts/generate-metric-helper-fixtures.mjs'), str(args.metric_sdk_root.resolve()), '--check'], cwd=ROOT, check=True)
