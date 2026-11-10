// Exercise generated Node dispatch and its real input encoder without a native binary.
import fs from "node:fs";
import vm from "node:vm";
import assert from "node:assert/strict";
import {fileURLToPath} from "node:url";
import {createRequire} from "node:module";
const require=createRequire(import.meta.url);
const root=fileURLToPath(new URL("../",import.meta.url));
const errors=require(root+"crates/node/js/errors.cjs");
const calls=[];
class Native {
 constructor(args){
  calls.push(["new",JSON.parse(args)]);this.closed=false;
  return new Proxy(this,{get(target,key){
   if(key==="closed")return target.closed;
   if(key==="close")return async()=>{target.closed=true;};
   if(key in target)return target[key];
   return async args=>{calls.push([key,args]);return null;};
  }});
 }
}
let runtime;
const exports={};
const context=vm.createContext({exports,ValidationError:errors.ValidationError,require:id=>id==="./runtime.cjs"?runtime:id==="./errors.cjs"?errors:{}});
const runtimeSource=fs.readFileSync(root+"crates/node/js/runtime.cjs","utf8");
const begin=runtimeSource.indexOf("class ExactNumber");
const end=runtimeSource.indexOf("function decodeNode",begin);
vm.runInContext('"use strict"; {'+runtimeSource.slice(begin,end)+";this.encodeInput=encodeInput;this.exactNumber=exactNumber;this.ExactNumber=ExactNumber;}",context);
runtime={native:{BirdeyeClient:Native,version:()=>"source-test"},encodeInput:context.encodeInput,decodeOutput:value=>value,invoke:(fn,transform=value=>value)=>{const value=fn();return value instanceof Promise?value.then(transform):transform(value);},wrap:()=>{},callbackAdapter:()=>{},exactNumber:context.exactNumber,ExactNumber:context.ExactNumber};
vm.runInContext(fs.readFileSync(root+"crates/node/js/index.cjs","utf8"),context);
const client=new exports.BirdeyeClient({apiKey:"fixture-secret",chain:"base"});
assert.deepEqual(calls[0],["new",{api_key:"fixture-secret",api_url:"https://public-api.birdeye.so",timeout_ms:10000,chain:"base"}]);
const rows=JSON.parse(fs.readFileSync(root+"crates/data/birdeye/tests/fixtures/http.json"));
for(const row of rows){
 const name=row.operation.replace(/_([a-z])/g,(_,c)=>c.toUpperCase());
 const request=vm.runInContext('JSON.parse('+JSON.stringify(row.request_json)+',(_k,v,c)=>typeof v!=="number"?v:/[.eE]/.test(c.source)?exactNumber(c.source):Number.isSafeInteger(v)?v:BigInt(c.source))',context);
 await client[name]({request});
 assert.equal(calls.at(-1)[0],name);
 assert.deepEqual(JSON.parse(calls.at(-1)[1]).request,JSON.parse(row.request_json));
 if(row.query.check_liquidity)assert.ok(calls.at(-1)[1].includes('0.1000000000000000001'));
}
await client.close();assert.equal(client.closed,true);
console.log("Birdeye Node source: 46 generated dispatches, exact input encoding and close passed (fake native boundary)");
