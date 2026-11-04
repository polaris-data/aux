#!/usr/bin/env python3
"""Check pinned vendor API inventories, methods, fixtures and tests offline."""
import ast
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def require(condition, message):
    if not condition:
        raise SystemExit(message)


def main():
    ast.parse((ROOT / 'crates/python/python/aux_py/_native.pyi').read_text())
    wrappers = ast.parse((ROOT / 'crates/python/python/aux_py/_data.py').read_text())
    python_methods = {
        node.name: {method.name for method in node.body if isinstance(method, ast.AsyncFunctionDef)}
        for node in wrappers.body if isinstance(node, ast.ClassDef)
    }
    python_tests = (ROOT / 'crates/python/tests/test_data.py').read_text()
    native_binding = (ROOT / 'crates/python/src/data.rs').read_text()
    for vendor in ('tardis', 'databento'):
        base = ROOT / 'crates/data' / vendor
        inventory = json.loads((base / 'coverage.json').read_text())
        pins = json.loads((base / 'API_SOURCES.json').read_text())
        digest = hashlib.sha256(json.dumps(inventory, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
        require(digest == pins['inventory_sha256'], f'{vendor}: pinned inventory changed')
        source = '\n'.join(p.read_text() for p in (base / 'src').glob('*.rs'))
        require(len({row['id'] for row in inventory['operations']}) == len(inventory['operations']), f'{vendor}: duplicate operations')
        for path, digest in pins['fixture_sha256'].items():
            require(hashlib.sha256((base / path).read_bytes()).hexdigest() == digest, f'{vendor}: fixture changed: {path}')
        for row in inventory['operations'] + inventory['schemas']:
            method = row['rust'].split('::')[-1]
            require(re.search(r'pub (?:async )?fn\s+' + method + r'\b', source) or re.search(r'\b' + method + r',\s*(?:Get|Post),', source), f'{vendor}: missing Rust method {method}')
            cls, method = row['python'].split('.')
            require(method in python_methods.get(cls, set()), f'{vendor}: missing Python wrapper {cls}.{method}')
            require(re.search(r'fn\s+' + method + r"<'py>", native_binding) or cls == 'DatabentoLiveSession', f'{vendor}: missing native Python binding {method}')
            path, _, selector = row['fixture'].partition('#')
            require((base / path).is_file(), f'{vendor}: missing fixture {path}')
            if selector:
                fixture = json.loads((base / path).read_text())
                require(any(selector in (entry.get('operation'), entry.get('schema'), entry.get('type')) for entry in fixture), f'{vendor}: missing fixture entry {selector}')
            path, name = row['test'].split('#')
            require(re.search(r'fn\s+' + name + r'\(', (base / path).read_text()), f'{vendor}: missing Rust test {name}')
            require('async def ' + row['python_test'] + '(' in python_tests, f'{vendor}: missing Python test {row["python_test"]}')
        print(f'{vendor}: {len(inventory["operations"])} operations and {len(inventory["schemas"])} schema/control entries covered')


if __name__ == '__main__':
    main()
