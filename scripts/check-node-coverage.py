#!/usr/bin/env python3
"""Check that the Node SDK exposes the existing native binding coverage."""
import json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parent.parent
api=json.loads((ROOT/'crates/node/src/api.json').read_text())
for venue,cls in [('hyperliquid','HyperliquidClient'),('lighter','LighterClient'),('qfex','QfexClient'),('arcus','ArcusClient'),('monaco','MonacoClient'),('derive','DeriveClient'),('paradex','ParadexClient'),('metric','MetricClient')]:
    assert cls in api['classes'],cls
for venue in ('derive','paradex'):
    coverage=json.loads((ROOT/'crates/exchanges'/venue/'coverage.json').read_text())
    methods={m['name'] for m in api['classes'][venue.title()+'Client']['methods']}
    missing={row['method'] for row in coverage['rest']}-methods
    assert not missing,(venue,missing)
# The independent Python declarations are an additional regression oracle.
python_methods={}
for file in (ROOT/'crates/python/src').rglob('*.rs'):
    text=file.read_text()
    for match in re.finditer(r'#\[pymethods\]\s*impl (\w+)\s*\{',text):
        tail=text[match.end():];level=1;end=0
        for end,ch in enumerate(tail):
            if ch=='{':level+=1
            elif ch=='}':level-=1
            if level==0:break
        names=set(re.findall(r'\bfn (\w+)(?:<[^\n]*?>)?\(',tail[:end]))
        python_methods.setdefault(match[1],set()).update(n for n in names if not n.startswith('__'))
# Market-data clients are outside the eight-exchange Node SDK; compare its bound classes.
for cls,binding in api['classes'].items():
    assert cls in python_methods,cls
    names=python_methods[cls]
    actual={m['name'] for m in binding['methods']}
    assert names==actual,(cls,'missing',names-actual,'extra',actual-names)
print(f"Node coverage: {len(api['classes'])} classes, {sum(len(c['methods']) for c in api['classes'].values())} methods, all eight exchanges")
# Regeneration must preserve checked-in model annotations without schema downloads.
import importlib.util
import subprocess
spec=importlib.util.spec_from_file_location('typescript_models',ROOT/'scripts/typescript-models.py')
renderer=importlib.util.module_from_spec(spec)
spec.loader.exec_module(renderer)
for relative in ('derive/src/models.rs','paradex/src/models.rs','paradex/src/sbe_models.rs'):
    file=ROOT/'crates/exchanges'/relative
    rendered=subprocess.run(['rustfmt','--edition','2024','--emit','stdout'],input=renderer.annotate(file.read_text()),text=True,capture_output=True,check=True).stdout
    assert rendered==file.read_text(),f'TypeScript annotations need regeneration: {relative}'
metadata=json.loads(subprocess.check_output(['cargo','metadata','--locked','--no-deps','--format-version','1'],cwd=ROOT,text=True))
rust_version=next(package['version'] for package in metadata['packages'] if package['name']=='aux-rs-node')
node_version=json.loads((ROOT/'crates/node/package.json').read_text())['version']
assert node_version==rust_version,(node_version,rust_version)
print('Node model annotations and package version verified')
