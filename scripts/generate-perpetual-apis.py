#!/usr/bin/env python3
"""Generate Phoenix/Extended native operations, models, fixtures and thin bindings.

Inputs are externally prepared OpenAPI unions pinned in API_SOURCES.json. Builds
and ordinary checks use the committed output and do not require these inputs.
Uses the same exact-number model renderer as the other native exchange adapters.
"""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location('native_generator', ROOT/'scripts/generate-native-apis.py')
native = importlib.util.module_from_spec(spec)
spec.loader.exec_module(native)
ARGS = native.ARGS

def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.suffix == ".rs":
        value = subprocess.run(["rustfmt", "--edition", "2024", "--emit", "stdout"], input=value, text=True, capture_output=True, check=True).stdout
    native.write_file(path, value)

def sample(g,schema,depth=0):
    if not isinstance(schema,dict) or depth>64:return None
    if '$ref' in schema:return sample(g,g.defs[schema['$ref'].split('/')[-1]],depth+1)
    for union in ['anyOf','oneOf','allOf']:
        if union in schema:
            choices=[v for v in schema[union] if v.get('type')!='null']
            return sample(g,choices[0],depth+1) if choices else None
    if 'enum' in schema:return schema['enum'][0]
    t=schema.get('type')
    if isinstance(t,list):t=next((v for v in t if v!='null'),'null')
    if t=='string':return 'fixture'
    if t in ['integer','number']:return 1
    if t=='boolean':return True
    if t=='array':
        if schema.get('prefixItems'):return [sample(g,item,depth+1) for item in schema['prefixItems']]
        return [] if depth>16 else [sample(g,schema.get('items',{}),depth+1)]
    if t=='object' or schema.get('properties'):return {k:sample(g,v,depth+1) for k,v in schema.get('properties',{}).items()}
    return None

class ContractGenerator(native.Generator):
    def typ(self,schema,name):
        if isinstance(schema,dict):
            if self.venue=='phoenix' and schema.get('type') in ['number','integer']:return 'NativeNumber'
            options=schema.get('oneOf',schema.get('anyOf',[]))
            kinds={item.get('type') for item in options}
            if 'string' in kinds and kinds & {'number','integer'} and kinds <= {'string','number','integer','null'}:
                return 'NativeNumber'
        return super().typ(schema,name)

def generate(venue):
    name = venue.title()
    folder = ROOT/'crates/exchanges'/venue
    reference = ARGS.reference_root/venue/'openapi.json'
    pins = json.loads((folder/'API_SOURCES.json').read_text())
    digest = hashlib.sha256(reference.read_bytes()).hexdigest()
    if digest != pins['input_sha256']['openapi.json']:
        raise SystemExit(f'{venue}: reference input differs from source pin')
    api = json.loads(reference.read_text())
    g = object.__new__(ContractGenerator)
    g.venue=venue; g.defs=api.get('components',{}).get('schemas',{})
    g.names={k:native.pascal(k) for k in g.defs};g.done=set();g.output=[]
    for k,schema in g.defs.items():g.declare(g.names[k],schema)
    operations=[];used=set()
    for path, entries in api['paths'].items():
        for verb, op in entries.items():
            if verb not in ('get','post','put','patch','delete'):continue
            method=native.snake(re.sub(r'\s*\(deprecated.*\)$','',op.get('summary',''),flags=re.I) if venue=='phoenix' else op.get('operationId') or verb+'_'+path)
            if method in used:method=native.snake(op.get('operationId',verb+'_'+path))
            if method in used:raise ValueError(f'duplicate method: {method}')
            used.add(method)
            params=[]
            for p in entries.get('parameters',[])+op.get('parameters',[]):
                if '$ref' in p:p=api['components']['parameters'][p['$ref'].split('/')[-1]]
                if p['in']=='header' and p['name'].lower() in ('authorization','x-api-key','user-agent'):continue
                params.append(p)
            body=op.get('requestBody',{})
            if '$ref' in body:body=api['components']['requestBodies'][body['$ref'].split('/')[-1]]
            if body:params.append({'name':'body','in':'body','required':body.get('required',False),'schema':body.get('content',{}).get('application/json',{}).get('schema',{})})
            req=native.pascal(method)+'Params'
            req_schema={'type':'object','properties':{p['name']:p.get('schema',{}) for p in params},'required':[p['name'] for p in params if p.get('required')]}
            g.declare(req,req_schema)
            result=next((r for status,r in op.get('responses',{}).items() if str(status).startswith('2')), {})
            if '$ref' in result:result=api['components'].get('responses',{}).get(result['$ref'].split('/')[-1],{})
            result_schema=result.get('content',{}).get('application/json',{}).get('schema',{})
            response=g.typ(result_schema,native.pascal(method)+'Result')
            private=bool(op.get('security'))
            if venue=='phoenix':
                # Published auth guide explicitly allows anonymous market/trader reads.
                private='/notifications' in path or path in ['/v1/auth/logout','/v1/referral/activate-tx','/v1/referral/activate']
            operations.append(dict(method=method,path=path,verb=verb,request=req,response=response,private=private,parameters=params,request_schema=req_schema,response_schema=result_schema,source=op.get('x-source','openapi.json'),example=op.get('x-response-example')))
    number_types={'serde_json::Number'}
    for declaration in g.output:
        match=re.match(r'pub type (\w+) = serde_json::Number;',declaration)
        if match:number_types.add(match[1])
    native_number_types={'NativeNumber'}
    for declaration in g.output:
        match=re.match(r'pub type (\w+) = NativeNumber;',declaration)
        if match:native_number_types.add(match[1])
    for i,declaration in enumerate(g.output):
        for field,typ in re.findall(r'pub (\w+): ([\w:]+),',declaration):
            if typ in native_number_types:declaration=re.sub(r'\b'+field+r': serde_json::Number::from\(0\)',field+': Default::default()',declaration)
            if typ in number_types:declaration=re.sub(r'\b'+field+r': Default::default\(\)',field+': serde_json::Number::from(0)',declaration)
        g.output[i]=declaration
    model='// Generated by scripts/generate-perpetual-apis.py.\n#![allow(clippy::derivable_impls)]\n#[allow(dead_code)]\nfn zero_number()->serde_json::Number{0.into()}\n'+ '\n'.join(g.output)
    number_model='\n/// Native numeric tokens and string-encoded integers retain their original representation.\n#[cfg_attr(feature="typescript",derive(ts_rs::TS))]\n#[derive(Clone,serde::Serialize)]\n#[serde(untagged)]\npub enum NativeNumber{\n    Number(#[cfg_attr(feature="typescript",ts(type="number | bigint | string"))] serde_json::Number),\n    String(String),\n}\nimpl Default for NativeNumber{fn default()->Self{Self::Number(0.into())}}\nimpl<\'de> serde::Deserialize<\'de> for NativeNumber{\n    fn deserialize<D:serde::Deserializer<\'de>>(d:D)->Result<Self,D::Error>{\n        match <serde_json::Value as serde::Deserialize>::deserialize(d)?{\n            serde_json::Value::Number(n)=>Ok(Self::Number(n)),\n            serde_json::Value::String(s)=>Ok(Self::String(s)),\n            _=>Err(serde::de::Error::custom("native number or numeric string required")),\n        }\n    }\n}\n'
    write(folder/'src/native_number.rs',number_model)
    model=model.replace('#[allow(dead_code)]', 'use crate::NativeNumber;\n#[allow(dead_code)]')
    write(folder/'src/models.rs',model)
    lines=['// Generated by scripts/generate-perpetual-apis.py.','use crate::*;','use aux_rs_core::{HttpTransport,Error as ClientError};']
    for private in [False,True]:
        lines.append(f'impl<T:HttpTransport,A:{"Authentication" if private else "PublicAuthentication"}> {name}Client<T,A> {{')
        for o in operations:
            if o['private']!=private:continue
            m=o['method'];lines.append(f'pub async fn {m}(&self,request:{o["request"]})->Result<{o["response"]},ClientError<Error>>{{self.execute(&{m.upper()}_OPERATION,request).await}}')
        lines.append('}')
    for o in operations:
        ps=','.join('Parameter{name:'+json.dumps(p['name'])+',location:Location::'+p['in'].title()+',required:'+str(p.get('required',False)).lower()+',collection:'+json.dumps('multi' if p.get('explode',True) else 'csv')+'}' for p in o['parameters'])
        lines.append(f'pub const {o["method"].upper()}_OPERATION:Operation=Operation{{method:aux_rs_core::HttpMethod::{o["verb"].title()},path:{json.dumps(o["path"])},authenticated:{str(o["private"]).lower()},safe_read:{str(o["verb"]=="get").lower()},parameters:&[{ps}]}};')
    write(folder/'src/operations.rs','\n'.join(lines)+'\n')
    inventory=[{k:v for k,v in o.items() if k not in ['parameters','request_schema','response_schema','example']} for o in operations]
    fixtures={};tests=['// Generated contract fixtures; synthetic, not funded verification.',f'use aux_rs_{venue}::*;','use aux_rs_core::{HttpRequest,HttpResponse,HttpTransport,TransportError};','use serde_json::Value;','use std::sync::{Arc,Mutex};','#[derive(Clone)]struct Mock{response:Vec<u8>,sent:Arc<Mutex<Vec<HttpRequest>>>}','impl HttpTransport for Mock{async fn execute(&self,r:HttpRequest)->Result<HttpResponse,TransportError>{self.sent.lock().unwrap().push(r);Ok(HttpResponse{status:200,headers:vec![],body:self.response.clone()})}}']
    for o,row in zip(operations,inventory):
        m=o['method'];request=sample(g,o['request_schema']);response=sample(g,o['response_schema'])
        for p in o['parameters']:
            if p['in']=='body' and p.get('required') and request.get('body') is None:request['body']={}
        if venue=='extended' and isinstance(response,dict):response['status']='OK';response['error']=None
        if isinstance(response,dict):response['future_field']={'exact':'0.12345678901234567890123456789'}
        route=o['path'];query=[]
        from urllib.parse import quote
        for parameter in o['parameters']:
            key=parameter['name'];value=request.get(key)
            if value is None:continue
            scalar=lambda v:str(v).lower() if isinstance(v,bool) else str(v)
            if parameter['in']=='path':route=route.replace('{'+key+'}',quote(scalar(value),safe=''))
            if parameter['in']=='query':
                if isinstance(value,list):
                    if parameter.get('explode',True):query.extend((key,scalar(v)) for v in value)
                    else:query.append((key,','.join(map(scalar,value))))
                else:query.append((key,scalar(value)))
        if query:route+='?'+'&'.join(quote(k,safe='')+'='+quote(v,safe='') for k,v in query)
        fixtures[m]={'request':request,'response':response,'route':route,'body':request.get('body')}

        row.update(test='contract_'+m,fixture='tests/fixtures/contracts.json#'+m,rust=name+'Client::'+m,python='aux_py.'+name+'Client.'+m,node=name+'Client.'+re.sub('_([a-z])',lambda x:x[1].upper(),m))
        tests.extend(['#[tokio::test]',f'async fn contract_{m}(){{',f'let fixtures:Value=serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();let f=&fixtures[{json.dumps(m)}];',f'let request:{o["request"]}=serde_json::from_value(f["request"].clone()).unwrap();','let sent=Arc::new(Mutex::new(Vec::new()));let transport=Mock{response:serde_json::to_vec(&f["response"]).unwrap(),sent:sent.clone()};',f'let client={name}Client::with_transport({name}::mainnet(),transport).with_auth(Credentials::new(None,Some(zeroize::Zeroizing::new("fixture".into())),None).unwrap());',f'let result=client.{m}(request).await.unwrap();let result=serde_json::to_value(result).unwrap();', 'if f["response"].get("future_field").is_some(){assert_eq!(result["future_field"],f["response"]["future_field"]);}','let requests=sent.lock().unwrap();assert_eq!(requests.len(),1);',f'assert_eq!(requests[0].method,aux_rs_core::HttpMethod::{o["verb"].title()});','assert!(requests[0].url.ends_with(f["route"].as_str().unwrap()));let body=if requests[0].body.is_empty(){Value::Null}else{serde_json::from_slice(&requests[0].body).unwrap()};assert_eq!(body,f["body"]);','}'])
    write(folder/'tests/contracts.rs','\n'.join(tests)+'\n')
    write(folder/'tests/fixtures/contracts.json',json.dumps(fixtures,indent=2)+'\n')
    # Binding declarations use the same typed Rust methods and existing lifecycle owners.
    py=['// Generated by scripts/generate-perpetual-apis.py.','use super::*;','#[pymethods]',f'impl {name}Client {{']
    for o in operations:
        py.extend(['#[pyo3(signature=(request=None))]',f'    fn {o["method"]}<\'py>(&self,py:Python<\'py>,request:Option<Bound<\'py,PyAny>>)->PyResult<Bound<\'py,PyAny>>{{',f'let request:v::{o["request"]}=input(request)?;let client=self.state.client()?.public;',f'self.state.run(py,async move{{client.{o["method"]}(request).await.map_err(native)}})','}'])
    py.append('}')
    write(ROOT/'crates/python/src'/venue/'operations.rs','\n'.join(py)+'\n')
    wrapper=['# Generated by scripts/generate-perpetual-apis.py.','from typing import Any, Mapping','from . import _native, _ExchangeClient, Subscription',f'class {name}Client(_ExchangeClient):','    def __init__(self, **configuration: Any) -> None:',f'        self._inner = _native.{name}Client(**configuration)','    async def subscribe(self, specification: Mapping[str, Any]) -> Subscription:','        return Subscription(await self._inner.subscribe(specification))','    def set_credentials(self, credentials: Mapping[str, Any]) -> None:','        self._inner.set_credentials(credentials)']
    for o in operations:wrapper.extend([f'    async def {o["method"]}(self, request: Mapping[str, Any] | None = None) -> Any:',f'        return await self._inner.{o["method"]}(request)'])
    wrapper_path=ROOT/'crates/python/python/aux_py'/f'_{venue}.py'
    marker='    # Native signing and transaction helpers.'
    helpers=marker+wrapper_path.read_text().split(marker,1)[1] if wrapper_path.exists() and marker in wrapper_path.read_text() else ''
    write(wrapper_path,'\n'.join(wrapper)+'\n'+helpers)
    manifest={'spec_sha256':digest,'rest':inventory}
    existing=folder/'coverage.json'
    if existing.exists():manifest.update({k:v for k,v in json.loads(existing.read_text()).items() if k not in ['spec_sha256','rest']})
    write(existing,json.dumps(manifest,indent=2)+'\n')
    table=[f'# {name} native API coverage','', 'Contract examples are synthetic; live verification is recorded separately.','', '| Operation | Rust / Python method | TypeScript method |','| --- | --- | --- |']
    table += [f'| `{o["verb"].upper()} {o["path"]}` | `{o["method"]}` | `{o["node"]}` |' for o in inventory]
    document='\n'.join(table)+'\n'
    document+='\n## Native streams\n\n| Contract | Rust/Python | Node |\n| --- | --- | --- |\n'+'\n'.join('| `'+r['name']+'` | `'+r['python']+'` | `'+r['node']+'` |' for r in manifest.get('websocket',[]))+'\n'
    if venue=='phoenix':document+='\n## Instruction builders\n\nThe binding method `build_instruction` (`buildInstruction` in Node) dispatches these typed Rust builders.\n\n| Builder name | Native parameter type |\n| --- | --- |\n'+'\n'.join('| `'+r['method']+'` | `'+r['params']+'` |' for r in manifest.get('instruction_builders',[]))+'\n'
    document+='\n## Signing and transaction helpers\n\n| Rust | Python | Node | Test |\n| --- | --- | --- | --- |\n'+'\n'.join('| `'+r['rust']+'` | `'+r['python']+'` | `'+r['node']+'` | `'+r['test']+'` |' for r in manifest.get('helper_coverage',[]))+'\n'
    write(folder/'COVERAGE.md',document)
    print(venue,len(operations),'REST operations',len(g.done),'models')

if __name__=='__main__':
    if ARGS.reference_root is None:raise SystemExit('--reference-root PATH required')
    for venue in ['phoenix','extended']:generate(venue)
