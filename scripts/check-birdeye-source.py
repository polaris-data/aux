#!/usr/bin/env python3
"""Exercise all public Birdeye Python wrappers with a fake native boundary, without a build.

These checks validate generated dispatch/arguments; Rust protocol integration tests
remain separate and require compilation.
"""
import asyncio
from decimal import Decimal
import importlib.util
import json
from pathlib import Path
import sys
import types

ROOT=Path(__file__).resolve().parent.parent
calls=[]
class Native:
    def __init__(self,key,**options):
        calls.append(('new',key,options));self.closed=False
    def __getattr__(self,name):
        async def invoke(request=None):
            calls.append((name,request));return request
        return invoke
    async def aclose(self):self.closed=True
class Base:
    @property
    def closed(self):return self._inner.closed
    async def aclose(self):await self._inner.aclose()
package=types.ModuleType('_birdeye_source_test');package.__path__=[]
package._native=types.SimpleNamespace(BirdeyeClient=Native)
base=types.ModuleType(package.__name__+'._data');base._DataClient=Base
sys.modules[package.__name__]=package;sys.modules[base.__name__]=base
spec=importlib.util.spec_from_file_location(package.__name__+'._birdeye',ROOT/'crates/python/python/aux_py/_birdeye.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
async def main():
    client=module.BirdeyeClient('fixture-secret',chain='base')
    assert calls[-1]==('new','fixture-secret',{'chain':'base','api_url':'https://public-api.birdeye.so','timeout_ms':10000})
    fixtures=json.loads((ROOT/'crates/data/birdeye/tests/fixtures/http.json').read_text())
    for row in fixtures:
        request=json.loads(row['request_json'],parse_float=Decimal)
        result=await getattr(client,row['operation'])(request)
        assert result is request
        assert calls[-1]==(row['operation'],request)
    await client.token_list_scroll({'scroll_id':'cursor-1'})
    assert calls[-1]==('token_list_scroll',{'scroll_id':'cursor-1'})
    await client.aclose();assert client.closed
    print('Birdeye Python source: 46 wrappers, constructor options and lifecycle dispatch passed (fake native boundary)')
asyncio.run(main())
