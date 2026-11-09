#!/usr/bin/env python3
"""Refresh Phoenix/Extended Node binding inventories from the owned Python declarations."""
from pathlib import Path
import json,re
ROOT=Path(__file__).resolve().parent.parent

def split_args(s):
    result=[];depth=0;start=0
    for i,c in enumerate(s):
        if c in '<([':depth+=1
        elif c in '>)]':depth-=1
        elif c==',' and depth==0:result.append(s[start:i].strip());start=i+1
    if s[start:].strip():result.append(s[start:].strip())
    return result

def classes(source):
    for match in re.finditer(r'#\[pymethods\]\s*impl (\w+)\s*\{',source):
        start=match.end();depth=1;end=start
        while depth:
            if source[end]=='{':depth+=1
            elif source[end]=='}':depth-=1
            end+=1
        yield match[1],source[start:end-1]

api_path=ROOT/'crates/node/src/api.json';api=json.loads(api_path.read_text())
models_path=ROOT/'crates/node/src/models.json';models=json.loads(models_path.read_text())
for venue in ['phoenix','extended']:
    exported={}
    for path in sorted((ROOT/'crates/python/src'/venue).glob('*.rs')):
        for cls,body in classes(path.read_text()):
            methods=exported.setdefault(cls,[]);last=0
            for m in re.finditer(r'\bfn (\w+)(?:<[^>\n]+>)?\s*\((.*?)\)\s*->\s*([^\{]+)\{',body,re.S):
                name,args,ret=m.groups();header=body[last:m.start()];last=m.end()
                if name.startswith('__'):continue
                args=[arg.split(':',1) for arg in split_args(args) if ':' in arg and not arg.strip().startswith('py:')]
                args=[[k.strip(),re.sub(r',(?=\S)',', ',t.strip())] for k,t in args]
                defaults={}
                attr=re.search(r'#\[pyo3\(signature\s*=\s*\((.*?)\)\)\]',header,re.S)
                if attr:
                    for item in split_args(attr[1]):
                        if '=' in item:k,v=item.split('=',1);defaults[k.strip()]=v.strip()
                methods.append(dict(name=name,args=args,**{'return':' '.join(ret.split()),'async':"Bound<'py" in ret or 'Bound<\'py' in ret,'getter':'#[getter]' in header,'static':'#[staticmethod]' in header,'constructor':'#[new]' in header,'defaults':defaults,'resource':'Subscription' if name=='subscribe' else '()' if name=='aclose' else None}))
    for cls,methods in exported.items():api['classes'][cls]={'module':venue,'methods':methods}
    source=(ROOT/'crates/exchanges'/venue/'src/models.rs').read_text()
    models[venue]={n:f'aux_rs::{venue}::{n}' for n in re.findall(r'^pub (?:struct|enum) (\w+)',source,re.M)}
    for extra in (['NativeNumber','PreparedTransaction','SignedTransaction','LookupTable'] if venue=='phoenix' else ['NativeNumber','OrderContext','PreparedOrder']):models[venue][extra]=f'aux_rs::{venue}::{extra}'
api_path.write_text(json.dumps(api,indent=2)+'\n');models_path.write_text(json.dumps(models,indent=2)+'\n')
print('Updated Phoenix and Extended binding/model inventories')
# Keep the native Python declarations discoverable alongside the generated public wrappers.
stub_path=ROOT/'crates/python/python/aux_py/_native.pyi'
marker='# Phoenix and Extended native declarations.'
prefix=stub_path.read_text().split(marker)[0].rstrip()
lines=[prefix,'',marker]
for cls,binding in api['classes'].items():
    if binding['module'] not in ['phoenix','extended']:continue
    lines.append('class '+cls+':')
    for method in binding['methods']:
        if method['getter']:
            lines.append('    @property')
            lines.append('    def '+method['name']+'(self) -> '+('bool' if method['return']=='bool' else 'Any')+': ...')
            continue
        if method['static']:lines.append('    @staticmethod')
        args=[] if method['static'] else ['self']
        for name,typ in method['args']:
            py_type='Any'
            if typ in ['String','&str']:py_type='str'
            elif typ=='bool':py_type='bool'
            elif typ in ['u64','u32','usize','i64']:py_type='int'
            if typ.startswith('Option<'):py_type='Any | None'
            default=' = ...' if name in method['defaults'] or typ.startswith('Option<') else ''
            args.append(f'{name}: {py_type}{default}')
        name='__init__' if method['constructor'] else method['name']
        ret='None' if method['constructor'] else cls if method['static'] else 'Any'
        lines.append('    '+('async ' if method['async'] else '')+'def '+name+'('+', '.join(args)+') -> '+ret+': ...')
    lines.append('')
stub_path.write_text('\n'.join(lines).rstrip()+'\n')
