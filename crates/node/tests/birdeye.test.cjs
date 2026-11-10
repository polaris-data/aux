"use strict";
const {test}=require("node:test");
const assert=require("node:assert/strict");
const http=require("node:http");
const {once}=require("node:events");
const fs=require("node:fs");
const path=require("node:path");
const sdk=require(process.env.AUX_NODE_PACKAGE || "../js/index.cjs");
const runtime=require("../js/runtime.cjs");
const rows=JSON.parse(fs.readFileSync(path.join(__dirname,"../../data/birdeye/tests/fixtures/http.json")));
function parseInput(text){return JSON.parse(text,(_key,value,context)=>typeof value!=="number"?value:/[.eE]/.test(context.source)?sdk.exactNumber(context.source):Number.isSafeInteger(value)?value:BigInt(context.source));}
function decode(text){return runtime.decodeOutput(runtime.native.roundTrip(text));}
async function server(t,handler){const s=http.createServer(handler);s.listen(0,"127.0.0.1");await once(s,"listening");t.after(()=>new Promise(resolve=>{s.closeAllConnections();s.close(resolve);}));return `http://127.0.0.1:${s.address().port}`;}
test("Birdeye HTTP inventory and precision",async t=>{
 let row,requests=0;
 const apiUrl=await server(t,async(req,res)=>{
   requests++;const url=new URL(req.url,"http://peer");
   assert.equal(req.method,row.verb);assert.equal(url.pathname,row.path);
   assert.deepEqual(Object.fromEntries(url.searchParams),row.query);
   assert.equal(req.headers["x-api-key"],"fixture-secret");assert.equal(req.headers["x-chain"],"base");
   assert.equal(req.headers.authorization,undefined);
   const chunks=[];for await(const chunk of req)chunks.push(chunk);const body=Buffer.concat(chunks).toString();
   if(row.body_json!==null){assert.equal(req.headers["content-type"],"application/json");assert.deepEqual(decode(body),decode(row.body_json));}
   else assert.equal(body,"");
   res.end(row.response_body);
 });
 const client=new sdk.BirdeyeClient({apiKey:"fixture-secret",chain:"base",apiUrl});t.after(()=>client.close());
 for(row of rows){const result=await client[row.operation.replace(/_([a-z])/g,(_,c)=>c.toUpperCase())]({request:parseInput(row.request_json)});assert.deepEqual(result,decode(row.response_body));assert.equal(result.data.value,"0.1000000000000000001");assert.equal(result.data.rawAmount,123456789012345678901234567890n);}
 assert.equal(requests,46);await client.close();assert.equal(client.closed,true);await assert.rejects(client.price({request:{address:"TokenA"}}),sdk.ClosedError);
});
test("Birdeye native failures preserve codes and redact keys",async t=>{
 let status=200,requests=0;
 const apiUrl=await server(t,(_req,res)=>{requests++;res.statusCode=status;res.end('{"success":false,"message":"fixture-secret rejected","code":"native"}');});
 const client=new sdk.BirdeyeClient({apiKey:"fixture-secret",apiUrl});t.after(()=>client.close());
 for(status of [200,400,401,403,422,429,500]){const before=requests;await assert.rejects(client.price({request:{address:"TokenA"}}),e=>e instanceof sdk.DataVendorError&&e.httpStatus===status&&e.native.code==="native"&&!JSON.stringify(e).includes("fixture-secret"));assert.equal(requests,before+1);}
 assert.throws(()=>client.price({request:{address:"TokenA",check_liquidity:0.1}}),sdk.ValidationError);
 await assert.rejects(client.price({request:{}}),sdk.ValidationError);
});
