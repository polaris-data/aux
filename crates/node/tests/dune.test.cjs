"use strict";
const {test} = require("node:test");
const assert = require("node:assert/strict");
const http = require("node:http");
const {once} = require("node:events");
const fs = require("node:fs");
const path = require("node:path");
const sdk = require(process.env.AUX_NODE_PACKAGE || "../js/index.cjs");
const runtime = require("../js/runtime.cjs");
const rows = JSON.parse(fs.readFileSync(path.join(__dirname,"../../data/dune/tests/fixtures/http.json")));
async function server(t, handler) {
  const s = http.createServer(handler);
  s.listen(0, "127.0.0.1"); await once(s,"listening");
  t.after(() => new Promise(resolve => { s.closeAllConnections(); s.close(resolve); }));
  return `http://127.0.0.1:${s.address().port}/api/v1`;
}
function decode(text) { return runtime.decodeOutput(runtime.native.roundTrip(text)); }
function input(text) {
  return JSON.parse(text, (_key,value,context) => {
    if(typeof value!=="number")return value;
    return /[.eE]/.test(context.source) ? sdk.exactNumber(context.source) : Number.isSafeInteger(value) ? value : BigInt(context.source);
  });
}
test("Dune HTTP inventory and precision", async t => {
  let row; let requests=0;
  const apiUrl=await server(t, async (req,res) => {
    requests++;
    const url=new URL(req.url,"http://peer");
    assert.equal(req.method,row.verb);assert.equal(url.pathname,row.path);
    assert.equal(req.headers["x-dune-api-key"],"fixture-secret");
    assert.equal(req.headers.authorization,undefined);
    assert.equal(url.searchParams.has("api_key"),false);
    assert.deepEqual(Object.fromEntries(url.searchParams),Object.fromEntries(Object.entries(row.query).map(([k,v])=>[k,String(v)])));
    const chunks=[];for await(const chunk of req)chunks.push(chunk);
    const body=Buffer.concat(chunks).toString();
    if(row.verb==="POST"&&row.request_json!==null){assert.equal(req.headers["content-type"],"application/json");assert.deepEqual(decode(body),decode(row.request_json));}
    else assert.equal(body,"");
    for(const [key,value] of Object.entries(row.response_headers))res.setHeader(key,value);
    res.end(row.response_body);
  });
  const client=new sdk.DuneClient({apiKey:"fixture-secret",apiUrl});t.after(()=>client.close());
  for(row of rows){
    const options={};
    if(row.path.includes("/query/"))options.queryId=123n;
    else if(row.path.includes("/execution/"))options.executionId="exec-1";
    if(row.request_json!==null)options.request=input(row.request_json);
    const name=row.operation.replace(/_([a-z])/g,(_,c)=>c.toUpperCase());
    const result=await client[name](options);
    if(row.operation.endsWith("_csv"))assert.deepEqual(result,{data:row.response_body,next_offset:1n,next_uri:row.response_headers["X-Dune-Next-Uri"]});
    else {
      const expected=decode(row.response_body);
      // Typed Rust integer fields are bigint, even for small values.
      if(expected.is_execution_finished!==undefined&&expected.result){expected.next_offset=BigInt(expected.next_offset);expected.result.metadata.row_count=BigInt(expected.result.metadata.row_count);expected.result.metadata.total_row_count=BigInt(expected.result.metadata.total_row_count);}
      assert.deepEqual(result,expected);
    }
    if(result.result){assert.equal(result.result.rows[0].id,9007199254740993n);assert.equal(result.result.rows[0].amount,"0.1000000000000000001");assert.equal(result.result.rows[0].nested.x,123456789012345678901234567890n);}
  }
  assert.equal(requests,9);
  await client.close();assert.equal(client.closed,true);
  await assert.rejects(client.getExecutionStatus({executionId:"x"}),sdk.ClosedError);
});
test("Dune defaults, errors, validation and explicit close",async t=>{
  let status=200,body='{"execution_id":"x","state":"QUERY_STATE_PENDING"}',requests=0;
  const apiUrl=await server(t,async(req,res)=>{requests++;const chunks=[];for await(const chunk of req)chunks.push(chunk);assert.equal(Buffer.concat(chunks).toString(),"{}");res.statusCode=status;res.end(body);});
  const client=new sdk.DuneClient({apiKey:"fixture-secret",apiUrl});t.after(()=>client.close());
  await client.executeQuery({queryId:123n});
  for(const [code,kind] of [[401,"authentication"],[402,"vendor"],[403,"entitlement"],[429,"rate_limit"],[500,"vendor"]]){
    status=code;body='{"error":"fixture-secret rejected"}';const before=requests;
    await assert.rejects(client.executeQuery({queryId:123n}),e=>e instanceof sdk.DataVendorError&&e.httpStatus===code&&e.kind===kind&&!JSON.stringify(e).includes("fixture-secret"));
    assert.equal(requests,before+1);
  }
  status=200;body="{}";
  await assert.rejects(client.executeQuery({queryId:123n}),sdk.DecodeError);
  assert.throws(()=>client.executeQuery({queryId:123n,request:{query_parameters:{amount:0.1}}}),sdk.ValidationError);
  await assert.rejects(client.executeQuery({queryId:123n,request:{performance:"invalid"}}),sdk.ValidationError);
  await assert.rejects(client.getLatestQueryResults({queryId:123n,request:{sample_count:1n,offset:0n}}),sdk.ValidationError);
  assert.throws(()=>new sdk.DuneClient({apiKey:""}),sdk.ValidationError);
  assert.throws(()=>new sdk.DuneClient({apiKey:"key",timeoutMs:0n}),sdk.ValidationError);
});
test("Dune close interrupts an in-flight request",async t=>{
  let signal;const started=new Promise(resolve=>{signal=resolve;});
  const apiUrl=await server(t,()=>signal());
  const client=new sdk.DuneClient({apiKey:"key",apiUrl});t.after(()=>client.close());
  const pending=client.getExecutionStatus({executionId:"x"});
  const rejected=assert.rejects(pending,sdk.ClosedError);
  await started;await client.close();
  await rejected;
  await client.close();
});
test("Dune configured timeout is not retried",async t=>{
  let requests=0;const apiUrl=await server(t,()=>{requests++;});
  const client=new sdk.DuneClient({apiKey:"key",apiUrl,timeoutMs:50n});t.after(()=>client.close());
  await assert.rejects(client.getExecutionStatus({executionId:"x"}),e=>e instanceof sdk.TransportError&&e.kind==="timeout");
  assert.equal(requests,1);
});
