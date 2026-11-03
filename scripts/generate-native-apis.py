#!/usr/bin/env python3
"""Generate native declarations from pinned external Derive/Paradex specifications.

Deterministic, standard-library only. Schemas are supplied externally through
--reference-root, with derive/ and paradex/ subdirectories. Does not generate
authentication, signing, transport, or lifecycle implementations.
"""

import argparse
import hashlib
import json
import re
import subprocess
import sys
import xml.etree.ElementTree as ET
from urllib.parse import quote
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
KEYWORDS = set("type ref self Self match loop move async await use mod fn pub impl where in as crate super dyn enum struct const static return default box abstract final override virtual try yield gen".split())
CHECK = '--check' in sys.argv

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--reference-root', type=Path, help='external schema directory containing derive/ and paradex/ inputs')
parser.add_argument('--check', action='store_true', help='verify generated output without writing it')
ARGS = parser.parse_args()


def reference_root(venue):
    if ARGS.reference_root is None:
        raise SystemExit('Generation requires --reference-root PATH; schemas are not bundled in the repository.')
    directory = ARGS.reference_root.expanduser().resolve() / venue
    metadata = json.loads((ROOT/'crates/exchanges'/venue/'API_SOURCES.json').read_text())
    for name, expected in metadata['input_sha256'].items():
        path = directory/name
        if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise SystemExit(f'{venue}: missing or changed external schema input: {path}')
    return directory


def write_file(path, contents):
    if path.suffix == '.rs':
        contents = subprocess.run(['rustfmt','--edition','2024','--emit','stdout'],input=contents,text=True,capture_output=True,check=True).stdout
    if CHECK:
        if not path.exists() or path.read_text() != contents:
            raise SystemExit(f'Generated file is stale: {path.relative_to(ROOT)}')
    else:
        path.write_text(contents)


def snake(s):
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s)
    return re.sub(r"[^a-zA-Z0-9]+", "_", s).strip("_").lower()


def pascal(s):
    return "".join(w[:1].upper() + w[1:] for w in re.split(r"[^a-zA-Z0-9]+", s) if w)


def ident(s):
    s = snake(s)
    if s[:1].isdigit():
        s = "n_" + s
    return s + "_" if s in KEYWORDS else s


class Generator:
    def __init__(self, venue):
        self.venue = venue
        self.base = ROOT / "crates/exchanges" / venue
        self.references = reference_root(venue)
        self.spec = json.loads((self.references / "openapi.json").read_text())
        supplement = self.references / 'supplemental.json'
        if supplement.exists():
            for path, methods in json.loads(supplement.read_text())['paths'].items():
                self.spec['paths'].setdefault(path, {}).update(methods)
        self.defs = self.spec.get("definitions", self.spec.get("components", {}).get("schemas", {}))
        self.ws_documents = []
        if venue == 'derive':
            for filename in ['websocket.asyncapi.json', 'subscriptions.asyncapi.json']:
                document = json.loads((self.references / filename).read_text())
                for key, schema in document.get('components', {}).get('schemas', {}).items():
                    self.defs.setdefault(key, schema)
                self.ws_documents.append((filename, document))
        else:
            for path in sorted((self.references / 'ws').glob('*.json')):
                doc = json.loads(path.read_text())
                schemas = doc.get('components', {}).get('schemas', {})
                prefix = 'Ws' + pascal(path.stem)
                def rename(node):
                    if isinstance(node, dict):
                        return {k: '#/components/schemas/' + prefix + v.rsplit('/', 1)[-1] if k == '$ref' else rename(v) for k, v in node.items()}
                    if isinstance(node, list):
                        return [rename(x) for x in node]
                    return node
                for key, schema in schemas.items():
                    self.defs[prefix + key] = rename(schema)
                self.ws_documents.append((path.name, rename(doc)))
        self.names = {k: pascal(k) for k in self.defs}
        self.output = []
        self.done = set()
        self.ops = []

    def typ(self, schema, name):
        if not isinstance(schema, dict):
            return 'serde_json::Value'
        if 'anyOf' in schema:
            choices = [v for v in schema['anyOf'] if v.get('type') != 'null']
            if len(choices) == 1:
                return self.typ(choices[0], name)
        if "$ref" in schema:
            return self.names[schema["$ref"].split("/")[-1]]
        if "allOf" in schema and len(schema["allOf"]) == 1:
            return self.typ(schema["allOf"][0], name)
        t = schema.get("type")
        if isinstance(t, list):
            t = next((v for v in t if v != "null"), "null")
        if not t and schema.get("oneOf"):
            if all(v.get("type") == "string" for v in schema["oneOf"]):
                return "String"
        if t == "string":
            return "String"
        if t == "boolean":
            return "bool"
        if t in ("number", "integer"):
            # Arbitrary-precision Number preserves IDs larger than u64 and exact
            # decimal JSON numbers. Never pass through a binary float.
            return "serde_json::Number"
        if t == "array":
            if schema.get('prefixItems'):
                types = [self.typ(v, name + str(i)) for i,v in enumerate(schema['prefixItems'])]
                if len(set(types)) == 1:
                    return f'[{types[0]}; {len(types)}]'
                return '(' + ','.join(types) + ',)'
            return "Vec<" + self.typ(schema.get("items", {}), name + "Item") + ">"
        if t == "object" or "properties" in schema:
            if schema.get("properties"):
                self.declare(name, schema)
                return name
            additional = schema.get("additionalProperties", {})
            if isinstance(additional, dict) and additional:
                return "std::collections::BTreeMap<String, " + self.typ(additional, name + "Value") + ">"
            return "serde_json::Map<String, serde_json::Value>"
        return "serde_json::Value"

    def declare(self, name, schema):
        if name in self.done:
            return
        self.done.add(name)
        if schema.get("properties") or schema.get("type") == "object" and not schema.get("additionalProperties"):
            fields = []
            required = schema.get("required", [])
            field_names = {}
            used = set()
            for k in schema.get('properties', {}):
                n = ident(k)
                if n in used:
                    n += '_upper' if k.isupper() else '_value'
                used.add(n)
                field_names[k] = n
            for k, v in schema.get("properties", {}).items():
                ty = self.typ(v, name + pascal(k))
                optional = k not in required or v.get("nullable") or isinstance(v.get("type"), list) and "null" in v["type"]
                attrs = [f'rename = {json.dumps(k)}']
                if optional:
                    ty = f"Option<{ty}>"
                    if k not in required:
                        attrs.extend(['default', 'skip_serializing_if = "Option::is_none"'])
                elif k in ('signature', 'signer', 'nonce', 'signature_expiry_sec', 'signature_timestamp') and any(w in name for w in ['Request','Params','Publish']):
                    attrs.append('default = "zero_number"' if ty == 'serde_json::Number' else 'default')
                fields.append(f'    #[serde({", ".join(attrs)})]\n    pub {field_names[k]}: {ty},')
            # Unknown native fields survive round trips. Do not Debug secret-bearing
            # schemas: errors/client diagnostics deliberately omit wire payloads.
            known_fields = fields.copy()
            fields.append('    #[serde(flatten)]\n    pub extra: serde_json::Map<String, serde_json::Value>,')
            defaults = []
            for k, v in schema.get("properties", {}).items():
                optional = k not in required or v.get("nullable") or isinstance(v.get("type"), list) and "null" in v["type"]
                init = 'serde_json::Number::from(0)' if not optional and v.get('type') in ('number', 'integer') else 'Default::default()'
                defaults.append(f'{field_names[k]}: {init}')
            defaults.append('extra: Default::default()')
            # serde's flatten Content visitor rejects u128 values in extra
            # fields. Split the native map before typed deserialization so no
            # number passes through that intermediate representation.
            names = ','.join(json.dumps(k) for k in field_names)
            moves = ','.join(f'{n}: known.{n}' for n in field_names.values())
            if moves:
                moves += ','
            deserialize = f'''impl<'de> serde::Deserialize<'de> for {name} {{
                fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{{
                    #[derive(serde::Deserialize)]
                    struct Known {{ {''.join(known_fields)} }}
                    let mut extra=serde_json::Map::<String,serde_json::Value>::deserialize(deserializer)?;
                    let mut fields=serde_json::Map::new();
                    for name in [{names}] {{if let Some(value)=extra.remove(name){{fields.insert(name.to_owned(),value);}}}}
                    let known:Known=serde_json::from_value(serde_json::Value::Object(fields)).map_err(serde::de::Error::custom)?;
                    Ok(Self{{{moves}extra}})
                }}
            }}\n'''
            if not field_names:
                deserialize = f'''impl<'de> serde::Deserialize<'de> for {name} {{fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{{Ok(Self{{extra:serde_json::Map::<String,serde_json::Value>::deserialize(deserializer)?}})}}}}\n'''
            self.output.append(f'#[derive(Clone, serde::Serialize)]\npub struct {name} {{\n' + "\n".join(fields) + f'\n}}\nimpl Default for {name} {{ fn default() -> Self {{ Self {{ ' + ', '.join(defaults) + ' } } }\n' + deserialize)
        else:
            self.output.append(f'pub type {name} = {self.typ(schema, name + "Value")};\n')

    def operations(self):
        for path, methods in self.spec["paths"].items():
            for verb, op in methods.items():
                if verb not in ("get", "post", "put", "delete", "patch"):
                    continue
                method = snake(op.get("operationId") or verb + "_" + path)
                if method == 'auth':
                    method = 'get_jwt'
                name = pascal(method)
                req_name = name + "Params"
                params = op.get("parameters", [])
                if self.venue == "derive":
                    req = op.get("requestBody", {}).get("content", {}).get("application/json", {}).get("schema", {"type": "object"})
                    req_type = self.typ(req, req_name)
                else:
                    schema = {"type": "object", "properties": {}, "required": []}
                    for p in params:
                        key = "body" if p["in"] == "body" else p["name"]
                        schema["properties"][key] = p.get("schema", {k: v for k, v in p.items() if k not in ("name", "in", "required")})
                        if p.get("required"):
                            schema["required"].append(key)
                    self.declare(req_name, schema)
                    req_type = req_name
                response = next((r for status, r in op.get("responses", {}).items() if status.startswith("2")), {})
                resp_schema = response.get("schema", response.get("content", {}).get("application/json", {}).get("schema", {}))
                resp_type = self.typ(resp_schema, name + "Result")
                self.ops.append(dict(method=method, request=req_type, response=resp_type, path=path, verb=verb, private=bool(op.get("security")), parameters=params, schema=op, family=op.get("tags", ["Other"])[0]))

    def sample(self, schema, depth=0):
        if not isinstance(schema, dict) or depth > 12:
            return None
        if '$ref' in schema:
            return self.sample(self.defs[schema['$ref'].split('/')[-1]], depth+1)
        for key in ['anyOf', 'oneOf', 'allOf']:
            if key in schema:
                options = [v for v in schema[key] if v.get('type') != 'null']
                return self.sample(options[0], depth+1) if options else None
        if 'enum' in schema:
            return schema['enum'][0]
        if 'const' in schema:
            return schema['const']
        typ = schema.get('type')
        if isinstance(typ, list):
            typ = next((t for t in typ if t != 'null'), 'null')
        if typ == 'string':
            return '0.12345678' if schema.get('format') == 'decimal' else 'fixture'
        if typ in ('number', 'integer'):
            return 18446744073709551617 if schema.get('format') == 'uint128' else 1
        if typ == 'boolean':
            return True
        if typ == 'array':
            if schema.get('prefixItems'):
                return [self.sample(s, depth+1) for s in schema['prefixItems']]
            count = schema.get('minItems', 1)
            return [self.sample(schema.get('items', {}), depth+1) for _ in range(count)]
        if typ == 'object' or schema.get('properties'):
            # Include all native properties so optional typed fields also get coverage.
            result = {k:self.sample(v, depth+1) for k,v in schema.get('properties',{}).items()}
            if not result and isinstance(schema.get('additionalProperties'), dict):
                result['fixture'] = self.sample(schema['additionalProperties'], depth+1)
            return result
        return None

    def fixtures(self):
        fixtures = {}
        tests = ['// Generated from the pinned specification. Fixtures are synthetic contract examples.', 'use aux_rs_core::{HttpTransport,HttpRequest,HttpResponse,Error as ClientError};', 'use aux_rs_' + self.venue + '::*;', 'use serde_json::{Value,json};' if self.venue == 'derive' else 'use serde_json::Value;', 'use std::sync::{Arc,Mutex};',
            '#[derive(Clone)] struct Mock { response:Vec<u8>, sent:Arc<Mutex<Vec<HttpRequest>>> }',
            'impl HttpTransport for Mock { async fn execute(&self,request:HttpRequest)->Result<HttpResponse,aux_rs_core::TransportError>{self.sent.lock().unwrap().push(request);Ok(HttpResponse{status:200,headers:vec![],body:self.response.clone()})} }']
        if self.venue == 'derive':
            tests += ['#[derive(Clone)] struct Auth;', 'impl Authentication for Auth { fn wallet(&self)->&str{"0x0000000000000000000000000000000000000001"} fn signer_address(&self)->[u8;20]{[1;20]} fn nonce(&self)->Result<String,Error>{Ok("1".into())} async fn login(&self,_:&Derive)->Result<Value,Error>{Ok(json!({"wallet":self.wallet(),"timestamp":"1","signature":"fixture"}))} async fn sign_action(&self,_:&Derive,_:&ActionPayload)->Result<String,Error>{Ok("fixture".into())} }']
        else:
            tests += ['#[derive(Clone)] struct Auth { token:TokenState }', 'impl Authentication for Auth { fn account(&self)->&str{"0x1"} fn token(&self)->&TokenState{&self.token} async fn prepare(&self,_:&Paradex,_:&Operation,request:&mut HttpRequest)->Result<(),Error>{request.headers.push(("Authorization".into(),"Bearer fixture".into()));Ok(())} }']
        for o in self.ops:
            op = o['schema']
            if self.venue == 'derive':
                schema = op.get('requestBody',{}).get('content',{}).get('application/json',{}).get('schema',{'type':'object'})
                request = self.sample(schema)
                route = '/v3' + o['path']
                body = request
                headers = {'Content-Type':'application/json'}
            else:
                request = {}
                body = None
                route = ('' if o['path'].startswith('/v2/') else '/v1') + o['path']
                query = []
                headers = {'Content-Type':'application/json'}
                for p in o['parameters']:
                    key = 'body' if p['in'] == 'body' else p['name']
                    value = self.sample(p.get('schema',p))
                    request[key] = value
                    scalar = lambda x: str(x).lower() if isinstance(x,bool) else str(x)
                    if p['in'] == 'body':
                        body = value
                    elif p['in'] == 'path':
                        route = route.replace('{'+p['name']+'}', quote(scalar(value),safe=''))
                    elif p['in'] == 'query':
                        if isinstance(value,list):
                            sep = {'ssv':' ','tsv':'\t','pipes':'|'}.get(p.get('collectionFormat'),',')
                            if p.get('collectionFormat') == 'multi':
                                query.extend((p['name'],scalar(x)) for x in value)
                            else:
                                query.append((p['name'],sep.join(map(scalar,value))))
                        elif value is not None:
                            query.append((p['name'],scalar(value)))
                    elif p['in'] == 'header':
                        headers[p['name']] = scalar(value)
                if query:
                    route += '?' + '&'.join(quote(k,safe='')+'='+quote(v,safe='') for k,v in query)
            response = next((r for k,r in op.get('responses',{}).items() if k.startswith('2')), {})
            response = self.sample(response.get('schema',response.get('content',{}).get('application/json',{}).get('schema',{})))
            if isinstance(response,dict) and not o['response'].startswith('std::collections::BTreeMap'):
                response['future_field']={'exact':'123456789012345678901234567890.12345678'}
            fixtures[o['method']] = {'request':request,'response':response,'route':route,'body':body,'headers':headers}
            fixture = f'fixtures[{json.dumps(o["method"])}]'
            response_body = 'json!({"result":f["response"]})' if self.venue == 'derive' else 'f["response"].clone()'
            config = 'Derive::testnet()' if self.venue == 'derive' else 'Paradex::testnet()'
            client = pascal(self.venue)+'Client'
            auth = '.with_auth(Auth)' if self.venue == 'derive' else '.with_auth(Auth{token:TokenState::default()})'
            if not o['private']:
                auth = ''
            tests += ['#[tokio::test]',f'async fn contract_{o["method"]}() {{',
                'let fixtures:Value=serde_json::from_str(include_str!("fixtures/contracts.json")).unwrap();',f'let f=&{fixture};',
                f'let request:{o["request"]}=serde_json::from_value(f["request"].clone()).unwrap();',
                'let sent=Arc::new(Mutex::new(Vec::new()));',f'let transport=Mock{{response:serde_json::to_vec(&{response_body}).unwrap(),sent:sent.clone()}};',
                f'let client={client}::with_transport({config},transport){auth};',
                f'let response:Result<{o["response"]},ClientError<Error>>=client.{o["method"]}(request).await;',
                'let value=serde_json::to_value(response.unwrap()).unwrap();',
                'if f["response"].get("future_field").is_some(){assert_eq!(value["future_field"],f["response"]["future_field"]);}',
                'let requests=sent.lock().unwrap();assert_eq!(requests.len(),1);let wire=&requests[0];',
                'assert!(wire.url.ends_with(f["route"].as_str().unwrap()),"{}",wire.url);',
                f'assert_eq!(wire.method,aux_rs_core::HttpMethod::{pascal(o["verb"])});',
                'let body=if wire.body.is_empty(){Value::Null}else{serde_json::from_slice(&wire.body).unwrap()};assert_eq!(body,f["body"]);',
                'for (key,value) in f["headers"].as_object().unwrap(){assert!(wire.headers.iter().any(|(k,v)|k==key&&v==value.as_str().unwrap()));}',
                '}']
        folder=self.base/'tests/fixtures'
        folder.mkdir(parents=True,exist_ok=True)
        write_file(folder/'contracts.json', json.dumps(fixtures,indent=2)+'\n')
        write_file(self.base/'tests/contracts.rs', '\n'.join(tests)+'\n')

    def websocket(self):
        rows, fixtures = [], {}
        tests = ['// Generated from pinned native WebSocket specifications.', f'use aux_rs_{self.venue}::*;', 'use serde_json::{Value,json};']
        derive_types = {'auctionsWatch':'auctions_watch','marginWatch':'margin_watch','orderbook':'orderbook','spotFeed':'spot_feed','subaccountBalances':'balances','subaccountBestQuotes':'best_quotes','subaccountOrders':'orders','subaccountQuotes':'quotes','subaccountTrades':'account_trades','tickerSlim':'ticker_slim','tradesByInstrument':'trades','tradesByInstrumentTypeCurrency':'trades_by_type','walletRfqs':'rfqs'}
        for filename, doc in self.ws_documents:
            for key, channel in doc['channels'].items():
                if self.venue == 'derive':
                    command = filename == 'websocket.asyncapi.json'
                    message = doc['components']['messages'][key + ('Request' if command else 'Notification')]
                    if command:
                        method = message['payload']['properties']['method']['const']
                        schema = message['payload']['properties']['params']
                        spec, address = None, method
                        native = {'public/login':'DeriveClient::open_rpc','subscribe':'RpcSession::subscribe_channels','unsubscribe':'RpcSession::unsubscribe_channels','private/set_cancel_on_disconnect':'RpcSession::set_cancel_on_disconnect'}[method]
                    else:
                        schema = message['payload']
                        address = re.search(r'Channel name: `([^`]+)`', channel['description']).group(1)
                        spec = {'type':derive_types[key]}
                        for variable in re.findall(r'\{([^}]+)\}', address):
                            spec[variable] = {'instrument_name':'ETH-PERP','currency':'ETH','instrument_type':'perp','group':10,'depth':20,'interval':100,'subaccount_id':9,'wallet':'0x2222222222222222222222222222222222222222'}[variable]
                        native = 'Subscription::' + pascal(derive_types[key])
                    payload_type = self.typ(schema, 'Ws'+pascal(key)+'Payload')
                    payload = self.sample(schema)
                    if not command and message.get('summary','').startswith('Pushed as a JSON array'):
                        payload, payload_type = [payload], f'Vec<{payload_type}>'
                else:
                    address = key.lstrip('/')
                    command = address in ['auth','subscribe','unsubscribe','modify_subscription'] or address.startswith('order.')
                    schema = channel['subscribe' if command else 'publish']['message']['payload']
                    payload_type = self.typ(schema, 'Ws'+pascal(filename)+'Payload')
                    payload = self.sample(schema)
                    spec = None
                    if command:
                        native = {'auth':'ParadexClient::open_rpc','subscribe':'RpcSession::subscribe_channel','unsubscribe':'RpcSession::unsubscribe_channel','modify_subscription':'RpcSession::modify_subscription','order.create':'RpcSession::create_order','order.create_batch':'RpcSession::create_batch','order.modify':'RpcSession::modify_order','order.cancel':'RpcSession::cancel_order','order.cancel_all':'RpcSession::cancel_all','order.cancel_batch':'RpcSession::cancel_batch','order.cancel_on_disconnect':'RpcSession::set_cancel_on_disconnect'}[address]
                    else:
                        base=address.split('.')[0].split('[')[0]
                        spec={'type':base}
                        if '{market_symbol}' in address:spec['market']='ETH-USD-PERP'
                        if base=='order_book':spec.update(feed_type='snapshot',refresh_rate='100ms',price_tick='1')
                        if base=='rfq':spec['rfq_id']='1'
                        native='Subscription::'+pascal(base)
                rendered = address
                if spec:
                    for variable in re.findall(r'\{([^}]+)\}',address):
                        rendered=rendered.replace('{'+variable+'}',str(spec.get('market' if variable=='market_symbol' else variable)))
                    rendered=rendered.replace('[','').replace(']','')
                method = snake(address)
                test = 'websocket_' + method
                fixtures[method]={'address':rendered,'specification':spec,'payload':payload}
                response_type = None
                if command and self.venue == 'paradex':
                    response_schema = channel['publish']['message']['payload']
                    response_type = self.typ(response_schema, 'Ws'+pascal(filename)+'Response')
                    fixtures[method]['response'] = self.sample(response_schema)
                python_api = native.replace('::','.') if command else pascal(self.venue)+'Client.subscribe'
                if command and native.startswith('RpcSession::'):
                    python_api = pascal(self.venue)+python_api
                rows.append({'kind':'command' if command else 'channel','address':address,'rust':native,'python':python_api,'payload':payload_type,'fixture':'tests/fixtures/ws.json#'+method,'test':test,'source':filename})
                tests += ['#[test]',f'fn {test}(){{','let fixtures:Value=serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();',f'let f=&fixtures[{json.dumps(method)}];',f'let payload:{payload_type}=serde_json::from_value(f["payload"].clone()).unwrap();','let value=serde_json::to_value(payload).unwrap();']
                if command:
                    tests += [f'let wire=json!({{"jsonrpc":"2.0","id":1,"method":{json.dumps(address)},"params":value}});',f'assert_eq!(wire["method"],{json.dumps(address)});']
                    if response_type:
                        tests += [f'let _response:{response_type}=serde_json::from_value(f["response"].clone()).unwrap();']
                else:
                    tests += ['let specification:Subscription=serde_json::from_value(f["specification"].clone()).unwrap();assert_eq!(specification.channel().unwrap(),f["address"]);','let wire=json!({"method":"subscription","params":{"channel":f["address"],"data":value}});','let event=Event::decode(&wire.to_string()).unwrap();',f'let _:{payload_type}=event.payload().unwrap();']
                tests += ['}']
        if self.venue == 'paradex':
            for kind, address, spec, payload_type, payload in [
                ('funding_rate_comparison','funding_rate_comparison.ALL@1000ms',{'type':'funding_rate_comparison','refresh_rate':'1000ms'},'serde_json::Value',{'market':'ETH-USD-PERP','funding_rate':'0.000000123456'}),
                ('block_trades','block_trades',{'type':'block_trades'},'serde_json::Value',{'block_id':'123','status':'OPEN'}),
            ]:
                test='websocket_'+kind
                fixtures[kind]={'address':address,'specification':spec,'payload':payload}
                rows.append({'kind':'channel','address':address,'rust':'Subscription::'+pascal(kind),'python':'ParadexClient.subscribe','payload':payload_type,'fixture':'tests/fixtures/ws.json#'+kind,'test':test,'source':'tradeparadex/paradex-py@4ae46b533fc99dbf5fbf33b46611c829fa38b12a/paradex_py/api/ws_client.py'})
                tests += ['#[test]',f'fn {test}(){{',f'let spec:Subscription=serde_json::from_value(serde_json::json!({json.dumps(spec)})).unwrap();assert_eq!(spec.channel().unwrap(),{json.dumps(address)});','}']
        directory=self.base/'tests/fixtures'
        directory.mkdir(parents=True,exist_ok=True)
        write_file(directory/'ws.json',json.dumps(fixtures,indent=2)+'\n')
        write_file(self.base/'tests/websocket_contracts.rs','\n'.join(tests)+'\n')
        return rows

    def run(self):
        for k, v in self.defs.items():
            self.declare(self.names[k], v)
        self.operations()
        ws_inventory = self.websocket()
        src = self.base / "src"
        src.mkdir(exist_ok=True)
        write_file(src / "models.rs", "// Generated by scripts/generate-native-apis.py. Do not edit.\n#![allow(clippy::large_enum_variant, clippy::derivable_impls)]\nfn zero_number()->serde_json::Number {serde_json::Number::from(0)}\n" + "\n".join(self.output))
        venue = pascal(self.venue)
        out = ['// Generated by scripts/generate-native-apis.py. Do not edit.', 'use crate::*;', 'use aux_rs_core::{HttpTransport, Error as ClientError};']
        for private in [False, True]:
            bound = "Authentication" if private else "Sync"
            out.append(f'impl<T: HttpTransport, A: {bound}> {venue}Client<T,A> {{')
            for o in self.ops:
                if o["private"] != private:
                    continue
                out.append(f'    pub async fn {o["method"]}(&self, request: {o["request"]}) -> Result<{o["response"]}, ClientError<Error>> {{')
                if self.venue == "derive":
                    call = "execute_authenticated" if private else "execute_public"
                    out.append(f'        self.{call}({json.dumps(o["path"])}, request).await')
                else:
                    call = "execute_authenticated" if private else "execute_public"
                    out.append(f'        self.{call}(&{o["method"].upper()}_OPERATION, request).await')
                out.append('    }')
            out.append('}')
        if self.venue == "paradex":
            for o in self.ops:
                ps = []
                for p in o["parameters"]:
                    ps.append(f'Parameter {{ name: {json.dumps("body" if p["in"] == "body" else p["name"])}, location: Location::{pascal(p["in"])}, required: {str(p.get("required", False)).lower()}, collection: {json.dumps(p.get("collectionFormat", "csv"))} }}')
                safe = str(o["verb"] == "get").lower()
                out.append(f'pub const {o["method"].upper()}_OPERATION: Operation = Operation {{ method: aux_rs_core::HttpMethod::{pascal(o["verb"])}, path: {json.dumps(o["path"])}, authenticated: {str(o["private"]).lower()}, safe_read: {safe}, parameters: &[{", ".join(ps)}] }};')
        write_file(src / "operations.rs", "\n".join(out) + "\n")
        rpc = ['// Generated by scripts/generate-native-apis.py. Do not edit.']
        if self.venue == 'derive':
            rpc += ['use crate::*;', 'use aux_rs_core::{WebSocketConnection, Error as ClientError};']
            for private in [False, True]:
                rpc.append(f'impl<C:WebSocketConnection,A:{"Authentication + Clone" if private else "Send + Sync"}> RpcSession<C,A> {{')
                for o in self.ops:
                    if o['private'] == private:
                        prepare = f'let mut request=serde_json::to_value(request).map_err(Error::from).map_err(ClientError::Exchange)?;self.prepare_command({json.dumps(o["path"])},&mut request).await?;' if private else ''
                        rpc.append(f'pub async fn {o["method"]}(&mut self, request:{o["request"]})->Result<{o["response"]},ClientError<Error>> {{ {prepare} self.call({json.dumps(o["path"].lstrip("/"))},request).await }}')
                rpc.append('}')
        write_file(src / 'rpc_operations.rs', '\n'.join(rpc)+'\n')
        inventory = []
        for o in self.ops:
            inventory.append({k: o[k] for k in ['method','request','response','path','verb','private','family']})
        for row in inventory:
            row.update(rust=f'{venue}Client::{row["method"]}',python=f'aux_py.{venue}Client.{row["method"]}',fixture=f'tests/fixtures/contracts.json#{row["method"]}',test=f'contract_{row["method"]}')
        manifest = {"spec_sha256": hashlib.sha256((self.references / "openapi.json").read_bytes()).hexdigest(), "rest": inventory, "websocket": ws_inventory, "websocket_documents": [name for name, _ in self.ws_documents]}
        if self.venue == 'paradex':
            schema=ET.parse(self.references/'paradex_1_0.xml').getroot()
            manifest['sbe']={'schema':'1:3','sha256':hashlib.sha256((self.references/'paradex_1_0.xml').read_bytes()).hexdigest(),'templates':[{'id':int(m.attrib['id']),'rust':'sbe::'+m.attrib['name'],'fixture':'tests/fixtures/sbe.json#'+m.attrib['id'],'test':'all_templates_preserve_native_wire_values'} for m in schema.findall('{*}message')]}
        write_file(self.base / "coverage.json", json.dumps(manifest, indent=2) + "\n")
        table = [f'# {venue} API coverage', '', 'Generated from the pinned specification and documentation union. See [source pins](API_SOURCES.json) and [verification](VERIFICATION.md). Fixtures are synthetic offline contracts; coverage does not imply funded verification.', '', f'{len(inventory)} REST operations; {len(ws_inventory)} WebSocket entries.', '', '| REST operation | Rust method | Python method | Fixture / test |', '| --- | --- | --- | --- |']
        for row in inventory:
            table.append(f'| `{row["verb"].upper()} {row["path"]}` | `{row["rust"]}` | `{row["python"]}` | [{row["test"]}](tests/contracts.rs) / [fixture](tests/fixtures/contracts.json) |')
        table += ['', '| WebSocket command/channel | Rust API | Python API | Fixture / test |', '| --- | --- | --- | --- |']
        for row in ws_inventory:
            table.append(f'| `{row["address"]}` | `{row["rust"]}` | `{row["python"]}` | [{row["test"]}](tests/websocket_contracts.rs) / [fixture](tests/fixtures/ws.json) |')
        if self.venue == 'paradex':
            table += ['', '| SBE template (schema 1:3) | Rust event | Fixture |', '| --- | --- | --- |']
            for row in manifest['sbe']['templates']:
                table.append(f'| {row["id"]} | `{row["rust"]}` | [frame](tests/fixtures/sbe.json) |')
        write_file(self.base/'COVERAGE.md', '\n'.join(table)+'\n')
        # The generated binding calls the same typed Rust methods; Python performs
        # no protocol encoding, signing, or numerical coercion.
        py = ['// Generated by scripts/generate-native-apis.py. Do not edit.', 'use super::*;', '#[pymethods]', f'impl {venue}Client {{']
        for o in self.ops:
            m = o['method']
            client = 'self.state.client()?.authenticated(py)?' if o['private'] else 'self.state.client()?.public'
            py += [f'    #[pyo3(signature=(request=None))]',f'    fn {m}<\'py>(&self, py: Python<\'py>, request: Option<Bound<\'py, PyAny>>) -> PyResult<Bound<\'py, PyAny>> {{',f'        let request: v::{o["request"]} = input(request)?;', f'        let client = {client};',f'        self.state.run(py, async move {{ client.{m}(request).await.map_err(native) }})','    }']
        py += ['}']
        binding = ROOT / "crates/python/src" / self.venue
        binding.mkdir(exist_ok=True)
        write_file(binding / 'operations.rs', "\n".join(py) + "\n")
        if self.venue == 'derive':
            methods = [(o['method'],o['request'],o['private']) for o in self.ops]
            methods += [('set_cancel_on_disconnect','SetCancelOnDisconnectRequest',True),('subscribe_channels','SubscribeParams',False),('unsubscribe_channels','UnsubscribeParams',False)]
            lines=['// Generated by scripts/generate-native-apis.py.', 'use super::*;', '#[pymethods]', 'impl DeriveRpcSession {']
            for method,request,private in methods:
                unsigned='Err(errors::AuthenticationError::new_err("private RPC method requires authenticated session"))' if private else f'session.{method}(request).await.map_err(native)'
                lines += ['#[pyo3(signature=(request=None))]', f'fn {method}<\'py>(&self,py:Python<\'py>,request:Option<Bound<\'py,PyAny>>)->PyResult<Bound<\'py,PyAny>>{{',f'let request:v::{request}=input(request)?;let mut guard=self.acquire()?;',f'self.run(py,async move{{match &mut *guard{{NativeSession::Public('+('_session' if private else 'session')+f')=>{unsigned},NativeSession::Authenticated(session)=>session.{method}(request).await.map_err(native)}}}})','}']
            lines += ['}']
            folder=binding/'rpc'
            folder.mkdir(exist_ok=True)
            write_file(folder/'operations.rs', '\n'.join(lines)+'\n')
        wrapper = ['# Generated by scripts/generate-native-apis.py.', 'from __future__ import annotations', 'from typing import Any, Mapping', 'from . import _native, _ExchangeClient, Subscription', f'class {venue}Client(_ExchangeClient):', f'    """Native {venue} APIs; defaults to testnet."""', '    def __init__(self, **configuration: Any) -> None:',f'        self._inner = _native.{venue}Client(**configuration)', '    async def subscribe(self, specification: Mapping[str, Any]) -> Subscription:', '        return Subscription(await self._inner.subscribe(specification))']
        if self.venue == 'derive':
            wrapper += ['    async def sign_action(self, action: Mapping[str, Any]) -> str:', '        return await self._inner.sign_action(action)']
        else:
            wrapper += ['    async def authenticate(self) -> None:', '        await self._inner.authenticate()', '    def set_token(self, token: str, *, read_only: bool = True) -> None:', '        self._inner.set_token(token, read_only=read_only)', '    async def sign_typed_data(self, data: Mapping[str, Any]) -> list[str]:', '        return await self._inner.sign_typed_data(data)']
            wrapper += ['    def onboarding_typed_data(self) -> Mapping[str, Any]:', '        return self._inner.onboarding_typed_data()', '    async def sign_siwe(self, statement: str, nonce: str, issued_at: str, expiration: str | None = None) -> Any:', '        return await self._inner.sign_siwe(statement, nonce, issued_at, expiration)', '    async def evm_account_address(self, address: str, class_hash: str) -> str:', '        return await self._inner.evm_account_address(address, class_hash)']
        for o in self.ops:
            wrapper += [f'    async def {o["method"]}(self, request: Mapping[str, Any] | None = None) -> Any:',f'        return await self._inner.{o["method"]}(request)']
        wrapper += [f'    async def open_rpc(self) -> {venue}RpcSession:', f'        return {venue}RpcSession(await self._inner.open_rpc())']
        if self.venue == 'derive':
            wrapper += ['    async def prepare_signed(self, request: Mapping[str, Any], module_data: Mapping[str, Any]) -> Any:', '        return await self._inner.prepare_signed(request, module_data)']
            wrapper += ['class DeriveRpcSession:', '    def __init__(self, inner: Any) -> None:', '        self._inner = inner', '    @property', '    def pending(self) -> Mapping[str, Any] | None:', '        return self._inner.pending', '    async def reconcile(self) -> Any:', '        return await self._inner.reconcile()', '    async def next_event(self) -> Any:', '        return await self._inner.next_event()', '    async def aclose(self) -> None:', '        await self._inner.aclose()', '    async def __aenter__(self):', '        return self', '    async def __aexit__(self, *_: Any) -> None:', '        await self.aclose()']
            for method,_,_ in methods:
                wrapper += [f'    async def {method}(self, request: Mapping[str, Any] | None = None) -> Any:',f'        return await self._inner.{method}(request)']
        else:
            wrapper += ['    async def block_signature(self, nonce: str, expiration: int, trades: Mapping[str, Any], *, parent: str | None = None) -> Any:', '        return await self._inner.block_signature(nonce, expiration, trades, parent=parent)', 'from ._paradex_rpc import ParadexRpcSession']
        write_file(ROOT/'crates/python/python/aux_py'/f'_{self.venue}.py', '\n'.join(wrapper)+'\n')
        self.fixtures()
        print(self.venue, len(self.ops), 'operations;',len(self.done),'native declarations')


def native_stubs(generators):
    path = ROOT/'crates/python/python/aux_py/_native.pyi'
    prefix = path.read_text().split('# Generated native exchange declarations.')[0].split('class DeriveSigner:')[0].rstrip()
    prefix = prefix.replace('    code: int | None\n', '    code: str | int | None\n    native: Native | None\n')
    sections = [prefix, '# Generated native exchange declarations. See generate-native-apis.py.']
    for generator in generators:
        venue = generator.venue
        method_stubs = '\n'.join(f'    async def {op["method"]}(self, request: Mapping[str, Any] | None = None) -> Any: ...' for op in generator.ops)
        name = venue.title()
        extra = '' if venue == 'derive' else ', kind: str = "starknet"'
        identity = 'address' if venue == 'derive' else 'public_key'
        result = 'str' if venue == 'derive' else 'list[str]'
        sections += [f'class {name}Signer:', f'    def __init__(self, private_key: str{extra}) -> None: ...', '    @staticmethod', f'    def remote({identity}: str, callback: Callable[[Native], Awaitable[Any]]{extra}) -> {name}Signer: ...', '    @property', f'    def {identity}(self) -> str: ...', f'    async def sign(self, payload: Mapping[str, Any]) -> {result}: ...', '', f'class {name}Client:', '    def __init__(self, **configuration: Any) -> None: ...', '    @property', '    def closed(self) -> bool: ...', '    @property', '    def authenticated(self) -> bool: ...', '    async def aclose(self) -> None: ...', '    async def subscribe(self, specification: Mapping[str, Any]) -> Subscription: ...', f'    async def open_rpc(self) -> {name}RpcSession: ...']
        if venue == 'derive':
            sections += ['    async def sign_action(self, action: Mapping[str, Any]) -> str: ...', '    async def prepare_signed(self, request: Mapping[str, Any], module_data: Mapping[str, Any]) -> Any: ...']
        else:
            sections += ['    async def authenticate(self) -> None: ...', '    def set_token(self, token: str, *, read_only: bool = True) -> None: ...', '    async def sign_typed_data(self, data: Mapping[str, Any]) -> list[str]: ...', '    async def block_signature(self, nonce: str, expiration: int, trades: Mapping[str, Any], *, parent: str | None = None) -> Any: ...']
            sections += ['    def onboarding_typed_data(self) -> Mapping[str, Any]: ...', '    async def sign_siwe(self, statement: str, nonce: str, issued_at: str, expiration: str | None = None) -> Any: ...', '    async def evm_account_address(self, address: str, class_hash: str) -> str: ...']
        sections.append(method_stubs)
        sections += ['', f'class {name}RpcSession:', '    @property', '    def pending(self) -> Mapping[str, Any] | None: ...', '    async def reconcile(self) -> Any: ...', '    async def next_event(self) -> Any: ...', '    async def aclose(self) -> None: ...']
        if venue == 'derive':
            sections += [method_stubs]
            for method in ['set_cancel_on_disconnect','subscribe_channels','unsubscribe_channels']:
                sections += [f'    async def {method}(self, request: Mapping[str, Any] | None = None) -> Any: ...']
        else:
            # Signatures follow the thin public wrapper; implementations live in Rust.
            import ast
            module = ast.parse((ROOT/'crates/python/python/aux_py/_paradex_rpc.py').read_text())
            cls = next(n for n in module.body if isinstance(n, ast.ClassDef))
            for node in cls.body:
                if isinstance(node, ast.AsyncFunctionDef) and node.name not in ('reconcile','next_event','aclose','__aenter__','__aexit__'):
                    sections.append('    async def '+node.name+'('+ast.unparse(node.args)+') -> '+ast.unparse(node.returns)+': ...')
        sections += ['']
    write_file(path, '\n'.join(sections).rstrip()+'\n')


if __name__ == '__main__':
    generators = []
    for venue in ['derive', 'paradex']:
        generator = Generator(venue)
        generator.run()
        generators.append(generator)
    native_stubs(generators)
