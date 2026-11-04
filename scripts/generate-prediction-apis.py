#!/usr/bin/env python3
"""Generate native Rust declarations and Python method inventories from pinned sources.

Inputs are external: --reference-root DIR containing the downloaded source files.
Authentication, stream codecs, signing and lifecycle are handwritten. --check is read-only.
"""
import argparse,ast,copy,hashlib,json,re,subprocess
from pathlib import Path
from urllib.parse import quote
ROOT=Path(__file__).resolve().parent.parent
KEYWORDS=set('type ref self Self match loop move async await use mod fn pub impl where in as crate super dyn enum struct const static return default box abstract final override virtual try yield gen'.split())
def snake(s):
 s=re.sub(r'([A-Z]+)([A-Z][a-z])',r'\1_\2',s)
 return re.sub(r'[^a-zA-Z0-9]+','_',re.sub(r'([a-z0-9])([A-Z])',r'\1_\2',s)).strip('_').lower()
def pascal(s):return ''.join(w[:1].upper()+w[1:] for w in re.split('[^a-zA-Z0-9]+',s) if w)
def ident(s):
 s=snake(s)
 if not s or s[0].isdigit():s='n_'+s
 return s+'_' if s in KEYWORDS else s
# Reuse the established exact-number model emitter without executing its CLI.
_old=ast.parse((ROOT/'scripts/generate-native-apis.py').read_text())
_cls=next(n for n in _old.body if isinstance(n,ast.ClassDef) and n.name=='Generator')
_methods=[n for n in _cls.body if isinstance(n,ast.FunctionDef) and n.name in ['typ','declare','sample']]
exec(compile(ast.fix_missing_locations(ast.Module(body=[ast.ClassDef(name='Models',bases=[],keywords=[],body=_methods,decorator_list=[])],type_ignores=[])),'models','exec'))
class Models(Models):
 def __init__(self,spec,prefix):
  self.spec=spec;self.defs=spec.get('components',{}).get('schemas',{});self.names={};used=set()
  for k in sorted(self.defs):
   n=prefix+pascal(k)
   while n in used:n+='Field'
   used.add(n);self.names[k]=n
  self.done=set();self.output=[];self.prefix=prefix
 def resolve(self,s):
  while isinstance(s,dict) and '$ref' in s:
   p=s['$ref']; v=self.spec
   if not p.startswith('#/'):raise ValueError('external reference requires a pinned input: '+p)
   for k in p[2:].split('/'):v=v[k.replace('~1','/').replace('~0','~')]
   s=v
  return s
 def merged(self,s,seen=()):
  if not isinstance(s,dict):return {}
  if '$ref' in s:
   if s['$ref'] in seen:return s
   return self.merged(self.resolve(s),seen+(s['$ref'],))
  if 'allOf' not in s:return s
  parts=[self.merged(part,seen) for part in s['allOf']]
  out={k:v for k,v in s.items() if k!='allOf'}
  if not any(p.get('properties') or p.get('type')=='object' for p in parts) and not out.get('properties'):
   merged={}
   for part in parts:merged.update(part)
   merged.update(out);return merged
  out.setdefault('properties',{});out.setdefault('required',[])
  for part in parts:
   out['properties'].update(part.get('properties',{}));out['required']+=part.get('required',[])
  out['type']='object';out['required']=list(dict.fromkeys(out['required']));return out
 def typ(self,s,name):
  if isinstance(s,dict) and '$ref' in s:
   resolved=self.merged(self.resolve(s))
   if resolved.get('type') in ['number','integer','string','boolean']:return super().typ(resolved,name)
   target=self.names[s['$ref'].split('/')[-1]]
   if resolved.get('type')=='array':return target
   return 'Box<'+target+'>'
  result=super().typ(self.merged(s),name)
  if result.startswith('Vec<Box<') and result.endswith('>>'):return 'Vec<'+result[8:-2]+'>'
  return result
 def declare(self,n,s):
  s=copy.deepcopy(self.merged(s))
  auto=set()
  if 'Params' in n and 'auth' in s.get('properties',{}):auto.add('auth')
  if 'op' in s.get('properties',{}) and 'sig' in s.get('properties',{}):auto.update(['sig','salt','ts'])
  if all(k in s.get('properties',{})for k in ['signature','maker','tokenId']):auto.add('signature')
  if auto:s['required']=[k for k in s.get('required',[])if k not in auto]
  for k,v in list(s.get('properties',{}).items()):
   if '$ref' in v:
    resolved=self.merged(self.resolve(v))
    if resolved.get('type') in ['number','integer','string','boolean']:s['properties'][k]={**resolved,**{a:b for a,b in v.items() if a!='$ref'}}
  start=len(self.output)
  result=super().declare(n,s)
  optional=[k for k in s.get('properties',{}) if k not in s.get('required',[])]
  optional_names=','.join(json.dumps(k) for k in optional)
  if optional:
   for i in range(start,len(self.output)):
    # Retain explicit optional nulls in extra; the typed None field is skipped
    # on serialization, so null and absent native fields remain distinguishable.
    if 'pub struct '+n+' ' in self.output[i]:
     self.output[i]=self.output[i].replace('if let Some(value)=extra.remove(name)',f'if [{optional_names}].contains(&name) && extra.get(name).is_some_and(serde_json::Value::is_null){{continue;}} if let Some(value)=extra.remove(name)')
  return result
 def sample(self,s,depth=0):
  if depth>40:return None
  s=self.merged(s)
  if 'example' in s:return s['example']
  if s.get('type')=='object' or 'properties' in s:
   return {k:self.sample(v,depth+1)for k,v in s.get('properties',{}).items()if depth<4 or k in s.get('required',[])}
  if s.get('type')=='array' and depth>=4 and not s.get('minItems'):return []
  return super().sample(s,depth)

def write(p,s,check):
 if p.suffix=='.rs':s=subprocess.run(['rustfmt','--edition','2024','--emit','stdout'],input=s,text=True,capture_output=True,check=True).stdout
 if check:
  if not p.is_file() or p.read_text()!=s:raise SystemExit('stale generated artifact: '+str(p.relative_to(ROOT)))
 else:
  if p.is_file() and p.read_text()==s:return
  p.parent.mkdir(parents=True,exist_ok=True);p.write_text(s)

def generate(venue,reference,check):
 base=ROOT/'crates/exchanges'/venue
 config=json.loads((base/'schema/services.json').read_text())
 all_ops=[];inventory=[]
 for service in config:
  spec=json.loads((reference/venue/service['input']).read_text())
  supplemental=base/'schema/supplemental.json'
  if supplemental.is_file():
   for path,verbs in json.loads(supplemental.read_text()).get(service['name'],{}).items():
    spec['paths'].setdefault(path,{}).update(verbs)
  if venue=='polymarket' and service['name']=='relayer':
   for path,verbs in spec['paths'].items():
    for verb,o in verbs.items():
     if verb in ['get','post','put','delete','patch'] and (verb!='get' or path=='/relayer/api/keys'):o['security']=[{'relayer':[]}]
  m=Models(spec,pascal(service['name']));ops=[];used=set()
  for path,verbs in sorted(spec['paths'].items()):
   for verb,op in sorted(verbs.items()):
    if verb not in ['get','post','put','patch','delete']:continue
    method=snake(op.get('operationId') or verb+'_'+path)
    if method in used:method+='_'+verb
    used.add(method);name=pascal(method);params=[]
    for p in verbs.get('parameters',[])+op.get('parameters',[]):
     p=m.resolve(p);params=[x for x in params if (x['in'],x['name'])!=(p['in'],p['name'])];params.append(p)
    req={'type':'object','properties':{},'required':[]}
    for p in params:
     req['properties'][p['name']]=p.get('schema',{'type':'string'})
     if p.get('required'):req['required'].append(p['name'])
    rb=m.resolve(op.get('requestBody',{}));body_schema=rb.get('content',{}).get('application/json',{}).get('schema')
    if body_schema is not None:
     req['properties']['body']=body_schema
     if rb.get('required'):req['required'].append('body')
    m.declare(name+'Params',req)
    resp=next((m.resolve(r) for status,r in op.get('responses',{}).items() if str(status).startswith('2')),{});rs=resp.get('content',{}).get('application/json',{}).get('schema',{})
    rt=m.typ(rs,name+'Response');m.declare(name+'Response',rs) if rt!=name+'Response' else None
    security=op.get('security',spec.get('security',[]));auth='none'
    if security:auth=service.get('auth','native')
    desc=op.get('description','').lower()
    if venue=='polymarket' and service['name']=='perps' and ('requires proxy signature' in desc or 'requires eoa signature' in desc):auth='perps'
    if venue=='pascal' and service['name']=='write':auth='native'
    if service.get('auth_all'):auth=service['auth_all']
    if venue=='polymarket' and service['name']=='clob':
     if '/auth/' in path and ('polyNonce' in str(security) or path=='/auth/api-key' and verb=='post' or path=='/auth/derive-api-key'):auth='wallet'
     if path=='/auth/api-keys':auth='wallet_or_native'
    safe=verb=='get' and auth=='none' and path not in ['/balance-allowance/update']
    wire_params=[{'name':p['name'],'location':p['in'],'required':bool(p.get('required')),'explode':p.get('explode',True),'style':p.get('style','form')} for p in params]
    row=dict(service=service['name'],client=service['client'],method=method,verb=verb.upper(),path=path,auth=auth,safe=safe,parameters=wire_params,has_body=body_schema is not None)
    req_sample=m.sample(req) or {};resp_sample=m.sample(rs)
    for ex in rb.get('content',{}).get('application/json',{}).get('examples',{}).values():
     if isinstance(ex,dict) and 'value' in ex:req_sample['body']=ex['value'];break
    examples=resp.get('content',{}).get('application/json',{}).get('examples',{})
    for ex in examples.values():
     if isinstance(ex,dict) and 'value' in ex:resp_sample=ex['value'];break
    row['fixture']=method;ops.append((row,name,rt));inventory.append(row);all_ops.append((service,row,name,rt,req_sample,resp_sample))
  for key,s in list(m.defs.items()):m.declare(m.names[key],s)
  out='// Generated by scripts/generate-prediction-apis.py.\n#![allow(clippy::derivable_impls)]\nuse crate::*;\n#[allow(dead_code)] fn zero_number()->serde_json::Number{0.into()}\n'+''.join(m.output)
  public=[];private=[]
  for row,name,rt in ops:
   const='OP_'+row['method'].upper();encoded=json.dumps(row,separators=(',',':'))
   out+=f'pub const {const}: &str = {json.dumps(encoded)};\n'
   fn=f'pub async fn {row["method"]}(&self, request: {name}Params) -> Result<{rt}, aux_rs_core::Error<Error>> {{ self.execute({const}, serde_json::to_value(request).map_err(|e| aux_rs_core::Error::Exchange(e.into()))?).await }}\n'
   (private if row['auth']!='none' else public).append(fn)
  # A public method can run on either authentication state; execute_public never attaches credentials.
  for row,name,rt in ops:
   if row['auth']=='none':public[sum(1 for r,_,_ in ops[:ops.index((row,name,rt))] if r['auth']=='none')]=public[sum(1 for r,_,_ in ops[:ops.index((row,name,rt))] if r['auth']=='none')].replace('self.execute(', 'self.execute_public(')
  client=service['client'];out+=f'impl<T: aux_rs_core::HttpTransport,A> {client}<T,A> {{'+''.join(public)+'}\n'
  out+=f'impl<T: aux_rs_core::HttpTransport,A: Authentication> {client}<T,A> {{'+''.join(private)+'}\n'
  write(base/'src'/f'{service["name"]}.rs',out,check)
 # Fixture inventory uses upstream examples when present, otherwise explicitly synthetic schema samples.
 fixtures={f'{s["name"]}.{r["method"]}':{'request':rq,'response':rs} for s,r,n,t,rq,rs in all_ops}
 write(base/'tests/fixtures/contracts.json',json.dumps(fixtures,indent=2,ensure_ascii=False)+'\n',check)
 manifest_rest=[dict(r,rust=r['client']+'::'+r['method'],rust_module=s['name'],request_type=n+'Params',response_type=t,python='aux_py.'+r['client']+'.'+r['method'],rust_test='tests/contracts.rs::'+ident(s['name']+'.'+r['method']),python_test='tests/test_prediction_connectors.py::test_every_published_method_and_public_fixture')for s,r,n,t,rq,rs in all_ops]
 write(base/'coverage.json',json.dumps({'rest':manifest_rest,'websocket':json.loads((base/'coverage.json').read_text()).get('websocket',[]) if (base/'coverage.json').is_file() else [],'contracts': [dict(r,rust='contracts::'+r['interface'],python='aux_py.polymarket.'+r['interface']+'.'+snake(r['signature'].split('(')[0].split()[-1]),fixture='tests/fixtures/contracts-abi.json',test='tests/chain.rs::every_published_contract_call_matches_independent_abi_encoder')for r in json.loads((base/'schema/contracts.json').read_text())]if venue=='polymarket'else [],'source_policy':'Published source union, pinned in API_SOURCES.json. Synthetic fixtures establish codecs, not funded execution.'},indent=2)+'\n',check)
 tests=['// Generated named-method contracts; no exchange requests.','use aux_rs_'+venue+' as v;','use aux_rs_core::{HttpTransport,HttpRequest,HttpResponse,TransportError};','use serde_json::Value;','use std::sync::{Arc,Mutex};',
 '#[derive(Clone)] struct Mock{response:Value,sent:Arc<Mutex<Vec<HttpRequest>>>}',
 'impl HttpTransport for Mock{async fn execute(&self,r:HttpRequest)->Result<HttpResponse,TransportError>{self.sent.lock().unwrap().push(r);Ok(HttpResponse{status:200,headers:vec![],body:serde_json::to_vec(&self.response).unwrap()})}}',
 '#[derive(Clone)]struct Auth;',
 'impl v::Authentication for Auth{fn identity(&self)->String{"fixture".into()}async fn prepare(&self,_:&v::'+venue.title()+',_:&v::Operation,r:&mut HttpRequest)->Result<(),v::Error>{r.headers.push(("x-fixture-auth".into(),"true".into()));Ok(())}}']
 for service,row,name,rt,rq,rs in all_ops:
  key=service['name']+'.'+row['method']; fn=ident(key)
  expected_path=row['path']
  for parameter in row['parameters']:
   if parameter['location']=='path':
    v=rq[parameter['name']];value=v if isinstance(v,str)else json.dumps(v,separators=(',',':'))
    expected_path=expected_path.replace('{'+parameter['name']+'}',quote(value,safe='-._~'))
  prefix=json.dumps(expected_path)
  tests.append(f'#[tokio::test] async fn {fn}() {{let fixtures:Value=serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();let fixture=&fixtures[{json.dumps(key)}];let mock=Mock{{response:fixture["response"].clone(),sent:Default::default()}};let client=v::{service["client"]}::with_transport(v::{venue.title()}::mainnet(),mock.clone()).with_auth(Auth);let params:v::{service["name"]}::{name}Params=serde_json::from_value(fixture["request"].clone()).unwrap();let result=client.{row["method"]}(params).await.unwrap_or_else(|e|{{if let aux_rs_core::Error::Exchange(v::Error::Decode(d))=e{{panic!("fixture decode: {{d}}");}}panic!("{{e}}");}});let value=serde_json::to_value(result).unwrap();assert_eq!(value,fixture["response"]);let sent=mock.sent.lock().unwrap();assert_eq!(sent.len(),1);assert_eq!(sent[0].method,aux_rs_core::HttpMethod::{row["verb"].title()});let parsed=url::Url::parse(&sent[0].url).unwrap();assert!(!parsed.path().contains("{{"));assert!(parsed.path().ends_with({prefix}));}}')
 write(base/'tests/contracts.rs','\n'.join(tests)+'\n',check)
 # Named Python/Rust methods share the same request types and exact conversion path.
 for client in dict.fromkeys(s['client'] for s in config):
  binding=['// Generated by scripts/generate-prediction-apis.py.','use super::*;',f'#[pymethods] impl {client} {{'];wrapper=[f'class {client}(_ExchangeClient):',f'    def __init__(self, **configuration: Any) -> None:',f'        self._inner = _native.{client}(**configuration)', '    async def subscribe(self, specification: Mapping[str, Any]) -> Subscription:', '        return Subscription(await self._inner.subscribe(specification))']
  for s,r,n,t,rq,rs in all_ops:
   if s['client']!=client:continue
   method=r['method'];binding.append(f'#[pyo3(signature=(request=None))] fn {method}<\'py>(&self,py:Python<\'py>,request:Option<Bound<\'py,PyAny>>) -> PyResult<Bound<\'py,PyAny>> {{ let request:v::{s["name"]}::{n}Params=input(request)?; let client=self.state.client()?.scoped(py)?; self.state.run(py,async move {{ client.{method}(request).await.map_err(native) }}) }}')
   wrapper.extend([f'    async def {method}(self, request: Mapping[str, Any] | None = None) -> Any:',f'        return await self._inner.{method}(request)'])
  binding.append('}');write(ROOT/'crates/python/src'/venue/(snake(client)+'_operations.rs'),'\n'.join(binding)+'\n',check)
  rpc='PolymarketRfqSession' if client=='PolymarketClient' else 'PolymarketPerpsRpcSession' if client=='PolymarketPerpsClient' else venue.title()+'RpcSession'
  wrapper.extend(['    async def open_rpc(self) -> Any:', f'        from ._prediction_rpc import {rpc}', f'        return {rpc}(await self._inner.open_rpc())'])
  if client=='PascalClient':wrapper.extend(['    @property','    def wallet_outcome_pending(self) -> bool:','        return self._inner.wallet_outcome_pending','    def reconcile_wallet(self) -> None:','        self._inner.reconcile_wallet()'])
  write(ROOT/'crates/python/python/aux_py'/('_'+snake(client)+'.py'),'# Generated by scripts/generate-prediction-apis.py.\nfrom __future__ import annotations\nfrom typing import Any, Mapping\nfrom . import _native, _ExchangeClient, Subscription\n'+'\n'.join(wrapper)+'\n',check)
 print(venue,len(inventory),'REST operations')

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--reference-root',type=Path,required=True);p.add_argument('--check',action='store_true');p.add_argument('--venue',choices=['kalshi','polymarket','pascal'],action='append');a=p.parse_args()
 for v in a.venue or ['kalshi','polymarket','pascal']:generate(v,a.reference_root,a.check)
if __name__=='__main__':main()
