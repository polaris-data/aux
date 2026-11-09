from pathlib import Path
import json,re,ast,hashlib
import yaml
from bs4 import BeautifulSoup
import argparse
parser=argparse.ArgumentParser(description='Prepare the pinned Extended OpenAPI/SDK/documentation union from externally downloaded references.')
parser.add_argument('--reference-root',type=Path,required=True)
args=parser.parse_args()
ROOT=Path(__file__).resolve().parent.parent; base=args.reference_root.resolve()
def snake(s):return re.sub(r'[^a-zA-Z0-9]+','_',re.sub(r'([a-z0-9])([A-Z])',r'\1_\2',s)).strip('_').lower()
def camel(s):
 a=s.split('_');return a[0]+''.join(x.title() for x in a[1:])
def inferred(v):
 if isinstance(v,dict):return {'type':'object','properties':{k:inferred(x) for k,x in v.items()}}
 if isinstance(v,list):return {'type':'array','items':inferred(v[0]) if v else {}}
 if isinstance(v,bool):return {'type':'boolean'}
 if isinstance(v,int):return {'type':'integer'}
 if isinstance(v,float):return {'type':'number'}
 if isinstance(v,str):return {'type':'string'}
 return {}
p=base/'extended'; upstream=p/'upstream'
pins=json.loads((ROOT/'crates/exchanges/extended/API_SOURCES.json').read_text())
spec=yaml.safe_load((upstream/'specs/rest-api.openapi.yaml').read_text()); spec['servers']=[{'url':'https://api.starknet.extended.exchange'}]
# Read the source models without importing the SDK or any runtime dependencies.
classes={}; aliases={}
for file in [upstream/path for path in pins['sdk_model_inputs']]:
 tree=ast.parse(file.read_text())
 for n in tree.body:
  if isinstance(n,ast.ClassDef):classes[n.name]=n
  elif isinstance(n,ast.Assign) and isinstance(n.targets[0],ast.Name):aliases[n.targets[0].id]=n.value
schemas=spec.setdefault('components',{}).setdefault('schemas',{})
def typ(n,seen=()):
 if isinstance(n,ast.Constant):return {'type':'string'} if n.value is not None else {'nullable':True}
 if isinstance(n,ast.Name):
  k=n.id
  if k in ['str','HexString','HexValue','Decimal','datetime','date']:return {'type':'string'}
  if k=='int':return {'oneOf':[{'type':'integer'},{'type':'string'}]}
  if k in ['bool','float']:return {'type':{'bool':'boolean','float':'number'}[k]}
  if k in classes:return {'$ref':'#/components/schemas/'+k}
  if k in aliases and k not in seen:return typ(aliases[k],seen+(k,))
  return {}
 if isinstance(n,ast.BinOp) and isinstance(n.op,ast.BitOr):return {'anyOf':[typ(n.left),typ(n.right)]}
 if isinstance(n,ast.Subscript):
  k=ast.unparse(n.value);args=n.slice.elts if isinstance(n.slice,ast.Tuple) else [n.slice]
  if k in ['List','list','Sequence','Set','set']:return {'type':'array','items':typ(args[0])}
  if k in ['Dict','dict','Mapping']:return {'type':'object','additionalProperties':typ(args[-1])}
  if k=='Optional':return dict(typ(args[0]),nullable=True)
  if k=='Annotated':return typ(args[0])
  if k=='Literal':return {'type':'string','enum':[ast.literal_eval(x) for x in args]}
  if k in ['Union']:return {'anyOf':[typ(x) for x in args]}
 return {}
def model(k,stack=()):
 if k in stack:return {'type':'object'}
 n=classes[k]
 if any('Enum' in ast.unparse(b) for b in n.bases):return {'type':'string'}
 props={};required=[]
 for b in n.bases:
  if isinstance(b,ast.Name) and b.id in classes:
   parent=model(b.id,stack+(k,));props.update(parent.get('properties',{}));required+=parent.get('required',[])
 for f in n.body:
  if isinstance(f,ast.AnnAssign) and isinstance(f.target,ast.Name) and f.target.id not in ['model_config']:
   key=camel(f.target.id);props[key]=typ(f.annotation)
   if f.value is None and not props[key].get('nullable'):required.append(key)
 return {'type':'object','properties':props,'required':required}
for k in classes:schemas[k]=model(k)
# SDK methods supply exact paths, query names and authoritative response models.
paths={}
for oldpath,methods in spec['paths'].items():paths[oldpath if oldpath.startswith('/api/') else '/api/v1'+oldpath]=methods
spec['paths']=paths
sdk_sources={}
for folder in ['rest','onboarding']:
 for file in [upstream/path for path in pins['sdk_module_inputs'] if path.startswith(f'x10/clients/{folder}/modules/')]:
  tree=ast.parse(file.read_text())
  for cls in [x for x in tree.body if isinstance(x,ast.ClassDef)]:
   for fn in [x for x in cls.body if isinstance(x,ast.AsyncFunctionDef)]:
    literals={n.targets[0].id:n.value.value for n in ast.walk(fn) if isinstance(n,ast.Assign) and len(n.targets)==1 and isinstance(n.targets[0],ast.Name) and isinstance(n.value,ast.Constant)}
    urls=[n for n in ast.walk(fn) if isinstance(n,ast.Call) and isinstance(n.func,ast.Attribute) and n.func.attr=='_get_url' and n.args and (isinstance(n.args[0],ast.Constant) or isinstance(n.args[0],ast.Name) and n.args[0].id in literals)]
    sends=[n for n in ast.walk(fn) if isinstance(n,ast.Call) and isinstance(n.func,ast.Name) and re.match('send_(get|post|delete|patch|put)_request',n.func.id)]
    if not urls or not sends:continue
    call=urls[0];send=sends[0];verb=send.func.id.split('_')[1];path=call.args[0].value if isinstance(call.args[0],ast.Constant) else literals[call.args[0].id]
    path=re.sub(r'<([^>]+)>',r'{\1}',path);path=('/api/v1' if folder=='rest' and not path.startswith('/api/') else '')+path
    op=paths.setdefault(path,{}).setdefault(verb,{})
    op['operationId']=fn.name;op['summary']=fn.name.replace('_',' ');op['tags']=[folder]
    op['security']=[{'ApiKeyAuth':[]}] if folder=='rest' and 'info_module' not in str(file) else []
    annotations={a.arg:a.annotation for a in fn.args.args+fn.args.kwonlyargs}
    params=[]
    for k in re.findall(r'{([^}]+)}',path):params.append({'name':k,'in':'path','required':True,'schema':{'type':'string'}})
    for kw in call.keywords:
     if kw.arg=='query' and isinstance(kw.value,ast.Dict):
      for key,val in zip(kw.value.keys,kw.value.values):
       if isinstance(key,ast.Constant):
        var=val.id if isinstance(val,ast.Name) else None
        params.append({'name':key.value,'in':'query','required':False,'schema':typ(annotations.get(var,ast.Name(id='str')))})
    if folder=='onboarding' and fn.name in ['create_api_key','onboard_subaccount']:
     for header in ['L1_SIGNATURE','L1_MESSAGE_TIME']+(['X-X10-ACTIVE-ACCOUNT'] if fn.name=='create_api_key' else []):
      params.append({'name':header,'in':'header','required':True,'schema':{'type':'string'}})
    if params:op['parameters']=params
    if len(send.args)>=3:
     data_schema=typ(send.args[2])
     op['responses']={'200':{'description':'Native SDK response','content':{'application/json':{'schema':{'type':'object','properties':{'status':{'type':'string'},'data':data_schema,'error':{},'pagination':{'type':'object'}}}}}}}
    for kw in send.keywords:
     if kw.arg=='json' and 'requestBody' not in op:
      body={}
      if isinstance(kw.value,ast.Call) and isinstance(kw.value.func,ast.Attribute) and isinstance(kw.value.func.value,ast.Name):body=typ(annotations.get(kw.value.func.value.id,ast.Name(id='Any')))
      if not body and isinstance(kw.value,ast.Call) and isinstance(kw.value.func,ast.Attribute) and isinstance(kw.value.func.value,ast.Name):
       variable=kw.value.func.value.id
       assignments=[n.value for n in ast.walk(fn) if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id==variable for t in n.targets)]
       if assignments and isinstance(assignments[0],ast.Call) and isinstance(assignments[0].func,ast.Name):body=typ(assignments[0].func)
      op['requestBody']={'required':True,'content':{'application/json':{'schema':body}}}
    sdk_sources[(verb,path)]=str(file.relative_to(upstream))
# Documentation union: includes published APIs newer than the SDK snapshot.
soup=BeautifulSoup((p/'docs.html').read_text(),'html.parser');docs=[]
for head in soup.find_all('h2'):
 block=[]
 for n in head.next_siblings:
  if getattr(n,'name',None) in ['h1','h2']:break
  block.append(n)
 text='\n'.join(n.get_text(' ',strip=True) for n in block if hasattr(n,'get_text'))
 route=re.search(r'\b(GET|POST|DELETE|PATCH|PUT) (/api/[^\s`]+)',text)
 if not route:continue
 verb,path=route.groups();verb=verb.lower();path=path.split('?')[0].rstrip('.');docs.append((verb,path))
 op=paths.setdefault(path,{}).setdefault(verb,{})
 op.setdefault('operationId',snake(head.get_text()));op['summary']=head.get_text();op.setdefault('tags',['documentation'])
 op.setdefault('security',[{'ApiKeyAuth':[]}] if any(s in path for s in ['/user/','/portfolio/']) else [])
 params={v['name']:v for v in op.get('parameters',[])}
 section=''
 for n in block:
  if not hasattr(n,'name'):continue
  if n.name=='h3':section=n.get_text().lower()
  if n.name!='table' or 'parameter' not in section:continue
  for row in n.find_all('tr')[1:]:
   cells=[c.get_text(' ',strip=True) for c in row.find_all('td')]
   if len(cells)<3:continue
   name,req,kind=cells[:3]
   if name in ['Parameter'] or any(x in name for x in ['[',']','.',' ']):continue
   location='path' if '{'+name+'}' in path else 'query'
   if 'body' in section or 'request' in section:continue
   t={'type':'boolean' if kind=='boolean' else 'integer' if kind in ['number','integer','long'] else 'string'}
   if 'array' in kind or 'Repeat' in ' '.join(cells):t={'type':'array','items':t}
   params.setdefault(name,{'name':name,'in':location,'required':req.lower()=='yes','schema':t})
 for name in re.findall(r'{([^}]+)}',path):params[name]={'name':name,'in':'path','required':True,'schema':{'type':'string'}}
 op['parameters']=list(params.values())
 # Use documented native response examples to supplement newly published models.
 examples=[]
 for n in block:
  if getattr(n,'name',None)=='div':
   try:examples.append(json.loads(n.get_text()))
   except (ValueError,TypeError):pass
 responses=[e for e in examples if isinstance(e,dict) and e.get('status')=='OK']
 if 'responses' not in op and responses:op['responses']={'200':{'description':'Published response','content':{'application/json':{'schema':inferred(responses[0])}}}}
 if responses:op['x-response-example']=responses[0]
 if verb in ['post','patch','put']:
  bodies=[e for e in examples if isinstance(e,dict) and 'status' not in e]
  if bodies and 'requestBody' not in op:op['requestBody']={'required':True,'content':{'application/json':{'schema':inferred(bodies[0])}}}
# All RFQ order fields match the native order schema, plus rfqStartPrice.
if '/api/v1/user/order/rfq' in paths:
 import copy
 rfq_schema=copy.deepcopy(schemas['CreateOrderRequest'])
 rfq_schema.setdefault('properties',{})['rfqStartPrice']={'type':'string','description':'Optional RFQ market-order quote-walk start price; excluded from the settlement signature.'}
 schemas['CreateRFQOrderRequest']=rfq_schema
 paths['/api/v1/user/order/rfq']['post']['requestBody']={'required':True,'content':{'application/json':{'schema':{'$ref':'#/components/schemas/CreateRFQOrderRequest'}}}}
# Distinct deterministic names for every verb/path pair.
used=set()
for path,methods in paths.items():
 for verb,op in methods.items():
  if verb not in ['get','post','put','patch','delete']:continue
  name=snake(op.get('operationId') or verb+'_'+path)
  if name in used:name=verb+'_'+snake(path.removeprefix('/api/v1/'))
  used.add(name);op['operationId']=name;op['x-source']=sdk_sources.get((verb,path),'docs.html' if (verb,path) in docs else 'upstream/specs/rest-api.openapi.yaml')
(p/'openapi.json').write_text(json.dumps(spec,indent=2)+'\n')
print('Extended',sum(sum(v in ['get','post','put','patch','delete'] for v in m) for m in paths.values()),'operations; SDK models',len(classes),'documented routes',len(docs))
