"""Generate named Metric contract calls from the pinned public method inventory."""
from collections import defaultdict
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]

def generate():
    py = ['"""Named Metric reads and transaction builders."""', '']
    js = ['"use strict";', '']
    ts = ['import type { MetricChainClient, HttpValue } from "./index";', '']
    contracts = defaultdict(list)
    for line in (ROOT / "schema/metric-contract-methods.tsv").read_text().splitlines():
        contract, signature, python, node, mode = line.split("\t")
        contracts[contract].append((signature, python, node, mode))
    rust = ['// Named Metric contract calls backed by the Zig ABI registry.',
            'use crate::{MetricChainClient, Result};', 'use serde_json::Value;', '',
            "pub struct Contracts<'a>(pub(crate) &'a MetricChainClient);",
            "pub struct ReadOptions<'a> { pub block: &'a str, pub sender: Option<&'a str>, pub value: &'a str }",
            """impl Default for ReadOptions<'_> { fn default() -> Self { Self { block: "latest", sender: None, value: "0x0" } } }""",
            "impl MetricChainClient { pub fn contracts(&self) -> Contracts<'_> { Contracts(self) } }", '']
    for contract, methods in contracts.items():
        accessor = re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", contract).lower()
        rust.extend([f"impl<'a> Contracts<'a> {{ pub fn {accessor}(&self, address: &'a str) -> {contract}<'a> {{ {contract}(self.0,address) }} }}",
                     f"pub struct {contract}<'a>(&'a MetricChainClient,&'a str);", f"impl {contract}<'_> {{"])
        py.extend([f'class {contract}:', '    def __init__(self, client, address):', '        self._client = client', '        self.address = address', ''])
        js.extend([f'class {contract} {{', '  constructor(client, address) { this.client = client; this.address = address; }'])
        ts.extend([f'export class {contract} {{', '  constructor(client: MetricChainClient, address: string);', '  readonly address: string;'])
        for signature, python, node, mode in methods:
            read = mode == 'call'
            method = "r#" + python if python in {"match","type","move","ref","return","impl","loop","in","as","use","fn"} else python
            if read:
                rust.append(f'    pub fn {method}(&self, args: &Value, options: ReadOptions) -> Result<Value> {{ self.0.call("{contract}","{signature}",self.1,args,options.block,options.sender,options.value) }}')
            else:
                rust.append(f'    pub fn {method}(&self, args: &Value, value: &str) -> Result<Value> {{ self.0.prepare_call("{contract}","{signature}",self.1,args,value) }}')
            args = f'"{contract}", "{signature}", self.address, args'
            if read:
                py.extend([f'    async def {python}(self, args=(), *, block="latest", from_address=None, value="0x0"):', f'        return await self._client.call({args}, block=block, from_address=from_address, value=value)', ''])
            else:
                py.extend([f'    def {python}(self, args=(), *, value="0x0"):', f'        return self._client.prepare_call({args}, value=value)', ''])
            js.append(f'  {node}(options = {{}}) {{ return this.client.{"call" if read else "prepareCall"}({{...options, contract:"{contract}", function:"{signature}", to:this.address}}); }}')
            fields = 'block?:string; fromAddress?:string;' if read else ''
            ts.append(f'  {node}(options?:{{args?:HttpValue[] | Record<string,HttpValue>; value?:string; {fields}}}): {"Promise<HttpValue>" if read else "PreparedCall"};')
        rust.extend(['}', ''])
        js.extend(['}',f'exports.{contract} = {contract};',''])
        ts.extend(['}', ''])
    py.append('CONTRACTS = {'+', '.join(f'"{c}": {c}' for c in contracts)+'}')
    ts.insert(1,'type PreparedCall = {to:string;data:string;value:string};')
    return {ROOT / 'sdk/rust/src/metric_contracts_generated.rs':'\n'.join(rust)+'\n',
            ROOT / 'sdk/python/python/aux_zig/metric_contracts.py':'\n'.join(py)+'\n',
            ROOT / 'sdk/typescript/js/contracts.cjs':'\n'.join(js)+'\n',
            ROOT / 'sdk/typescript/js/contracts.d.ts':'\n'.join(ts)+'\n'}
