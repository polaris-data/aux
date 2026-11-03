#!/usr/bin/env python3
"""Compile the pinned Paradex SBE XML into safe decoder layouts (offline)."""
import importlib.util
import json
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BASE = ROOT / 'crates/exchanges/paradex'
module = importlib.util.spec_from_file_location('native_apis', ROOT / 'scripts/generate-native-apis.py')
native = importlib.util.module_from_spec(module)
module.loader.exec_module(native)
root = ET.parse(BASE / 'reference/paradex_1_0.xml').getroot()
types = {x.attrib['name']: x for x in root.find('types')}
sizes = {'int64': 8, 'uint64': 8, 'uint32': 4, 'int32': 4, 'uint16': 2, 'int16': 2, 'uint8': 1, 'int8': 1}
out = ['// Generated from reference/paradex_1_0.xml by scripts/generate-paradex-sbe.py.', 'use super::{Field, Kind, Layout};']
models = []
variants = []


def kind(t):
    if t in sizes:
        return f'Kind::Integer {{ bytes: {sizes[t]}, signed: {str(t.startswith("int")).lower()} }}', sizes[t]
    typ = types[t]
    if t == 'Timestamp':
        return 'Kind::Timestamp', 8
    if t == 'AccountAddress':
        return 'Kind::Address', 32
    if typ.tag == 'composite':
        exp = int(next(x for x in typ if x.attrib['name'] == 'exponent').text)
        return f'Kind::Decimal {{ places: {-exp}, nullable: {str(t.endswith("NULL")).lower()} }}', 8
    if typ.tag in ('enum', 'set'):
        pairs = ','.join(f'({x.text.strip()}, {json.dumps(x.attrib["name"])})' for x in typ)
        tag = 'Flags' if typ.tag == 'set' else 'Enum'
        return f'Kind::{tag}(&[{pairs}])', sizes[typ.attrib['encodingType']]
    raise ValueError(t)


def field_list(element):
    fields = []
    offset = 0
    for x in element:
        if x.tag != 'field':
            continue
        k, size = kind(x.attrib['type'])
        if x.attrib.get('presence') == 'optional' and x.attrib['type'] == 'int64':
            k = 'Kind::OptionalI64'
        offset = int(x.attrib.get('offset', offset))
        fields.append(f'Field {{ name: {json.dumps(x.attrib["name"])}, offset: {offset}, size: {size}, kind: {k} }}')
        offset += size
    return '&[' + ','.join(fields) + ']', offset


for m in root.findall('{*}message'):
    name, mid = m.attrib['name'], int(m.attrib['id'])
    fields, size = field_list(m)
    variable = []
    props = {}
    required = []
    for x in m:
        field = x.attrib['name']
        required.append(field)
        if x.tag == 'field':
            typ = x.attrib['type']
            if typ in sizes or typ == 'Timestamp':
                schema = {'type': ['integer','null']} if x.attrib.get('presence') == 'optional' else {'type': 'integer'}
            elif types[typ].tag == 'set':
                schema = {'type': 'array', 'items': {'type': 'string'}}
            else:
                schema = {'type': ['string', 'null']} if typ.endswith('NULL') or types[typ].tag == 'enum' else {'type': 'string'}
        elif x.tag == 'data':
            variable.append(f'("{field}", None)')
            schema = {'type': 'string'}
        elif x.tag == 'group':
            gf, gs = field_list(x)
            variable.append(f'("{field}", Some(({gs}, {gf})))')
            schema = {'type': 'array', 'items': {'type': 'object', 'properties': {f.attrib['name']: {'type': 'string'} for f in x}, 'required': [f.attrib['name'] for f in x]}}
        else:
            raise ValueError(x.tag)
        props[field] = schema
    out.append(f'pub const {native.snake(name).upper()}: Layout = Layout {{ template: {mid}, minimum: {size}, fields: {fields}, variable: &[{",".join(variable)}] }};')
    gen = native.Generator('paradex')
    gen.declare(name, {'type': 'object', 'properties': props, 'required': required})
    models.extend(gen.output)
    variants.append((name, mid))
out.append('pub fn layout(template:u16)->Option<&\'static Layout>{Some(match template {' + ','.join(f'{i}=>&{native.snake(n).upper()}' for n,i in variants) + ',_=>return None})}')
native.write_file(BASE/'src/sbe_layout.rs', '\n'.join(out)+'\n')
model_code = '\n'.join(models)
model_code += '\n#[derive(Clone, serde::Serialize, serde::Deserialize)]\n#[serde(tag="template",content="data")]\npub enum SbeMessage {\n'
model_code += '\n'.join(f'#[serde(rename="{i}")] {n}({n}),' for n,i in variants) + '\n}\n'
native.write_file(BASE/'src/sbe_models.rs', '// Generated from the pinned SBE schema.\n#![allow(clippy::derivable_impls)]\n'+model_code)
print('Generated',len(variants),'SBE templates')
