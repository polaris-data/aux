#!/usr/bin/env python3
"""Offline native operation/fixture/language parity for Phoenix and Extended."""
from pathlib import Path
import ast,hashlib,json,re,subprocess,sys
ROOT=Path(__file__).resolve().parent.parent
api=json.loads((ROOT/'crates/node/src/api.json').read_text())
for venue in ['phoenix','extended']:
    base=ROOT/'crates/exchanges'/venue
    coverage=json.loads((base/'coverage.json').read_text());pins=json.loads((base/'API_SOURCES.json').read_text())
    assert coverage['spec_sha256']==pins['input_sha256']['openapi.json'],venue
    source=(base/'src/operations.rs').read_text();fixtures=json.loads((base/'tests/fixtures/contracts.json').read_text())
    rust=set(re.findall(r'pub async fn (\w+)\(',source))
    expected={r['method'] for r in coverage['rest']};assert rust==expected==set(fixtures),(venue,'native parity')
    assert len(expected)==len(coverage['rest'])==len({(r['verb'],r['path']) for r in coverage['rest']}),(venue,'duplicate route')
    py=(ROOT/'crates/python/src'/venue/'operations.rs').read_text();node=(ROOT/'crates/node/src'/venue/'operations.rs').read_text()
    for row in coverage['rest']:
        assert re.search(r'fn '+row['method']+r'(?:<|\()',py),(venue,row['method'],'Python')
        assert re.search(r'fn '+row['method']+r'\(',node),(venue,row['method'],'Node')
        assert 'async fn '+row['test']+'()' in (base/'tests/contracts.rs').read_text(),(venue,row['method'],'test')
        assert row['method'] in {m['name'] for m in api['classes'][venue.title()+'Client']['methods']}
    for file,expected_hash in pins.get('reference_vectors',{}).items():assert hashlib.sha256((base/file).read_bytes()).hexdigest()==expected_hash,(venue,file)
    if 'live_read_samples' in pins:assert hashlib.sha256((base/'tests/fixtures/public_markets.json').read_bytes()).hexdigest()==pins['live_read_samples']['sha256']
    for file, expected_hash in pins.get('fixture_sha256',{}).items():assert hashlib.sha256((base/file).read_bytes()).hexdigest()==expected_hash,(venue,file)
    for row in coverage.get('protocol_tests',[]):assert row['test'] in (base/row['file']).read_text(),(venue,row['test'])
    ws=json.loads((base/'tests/fixtures/ws.json').read_text())
    assert {row['name'] for row in coverage['websocket']}==set(ws),(venue,'stream inventory')
    assert coverage.get('signing') and coverage.get('helpers'),(venue,'missing helpers/signing inventory')
    assert {r['method'] for r in coverage['helper_coverage']} == set(coverage['helpers'])
    for row in coverage['helper_coverage']:
        owner, method = row['python'].split('.')
        assert method in {m['name'] for m in api['classes'][owner]['methods']}, (venue, row['python'])
        assert row['test'] in (base/row['test_file']).read_text(), (venue, row['test'])
    if venue == 'phoenix':
        subprocess.run([sys.executable, str(ROOT/'scripts/generate-phoenix-instructions.py'), '--check'], check=True)
        instruction_fixtures = json.loads((base/'tests/fixtures/instruction_dispatch.json').read_text())
        assert {r['method'] for r in coverage['instruction_builders']} == set(instruction_fixtures), 'instruction fixture parity'
        for row in coverage['instruction_builders']:
            assert 'instruction' in instruction_fixtures[row['method']], row['method']
            assert row['test'] in (base/row['test_file']).read_text()

    print(f'{venue}: {len(expected)} REST operations, {len(ws)} stream contracts, Rust/Python/Node parity')
