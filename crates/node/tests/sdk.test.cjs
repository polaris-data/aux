"use strict";
const {test}=require("node:test");
const assert=require("node:assert/strict");
const http=require("node:http");
const {once}=require("node:events");
const fs=require("node:fs");
const path=require("node:path");
const sdk=require(process.env.AUX_NODE_PACKAGE || "../js/index.cjs");
const runtime=require("../js/runtime.cjs");
const fixtures=(venue,file)=>fs.readFileSync(path.join(__dirname,"../../exchanges",venue,"tests/fixtures",file),"utf8");
const fixture=(v,f)=>JSON.parse(fixtures(v,f),(key,value,context)=>{if(typeof value!=="number")return value;const text=context.source;return text.includes(".")||/[eE]/.test(text)?runtime.exactNumber(text):Number.isSafeInteger(value)?value:BigInt(text);});
async function server(t,handler){const s=http.createServer(handler);s.listen(0,"127.0.0.1");await once(s,"listening");t.after(()=>new Promise(resolve=>{s.closeAllConnections();s.close(resolve);}));return `http://127.0.0.1:${s.address().port}`;}
function client(t,Ctor,opts){const c=new Ctor(opts);t.after(()=>c.close());return c;}
test("all eight native clients and generated operations are present",async t=>{
  for(const name of ["HyperliquidClient","LighterClient","QfexClient","ArcusClient","MonacoClient","DeriveClient","ParadexClient","MetricClient"]){const c=client(t,sdk[name],name==="MetricClient"?{chainId:1n}:{});assert.equal(c.closed,false,name);}
  for(const venue of ["derive","paradex"]){const inventory=JSON.parse(fs.readFileSync(path.join(__dirname,"../../exchanges",venue,"coverage.json")));for(const row of inventory.rest){const method=row.method.replace(/_([a-z])/g,(_,x)=>x.toUpperCase());assert.equal(typeof sdk[venue==="derive"?"DeriveClient":"ParadexClient"].prototype[method],"function",method);}}
});
test("input rejects rounding, floats, cycles, and implicit coercion",()=>{
 for(const v of [1.25,Number.MAX_SAFE_INTEGER+1,NaN,Infinity,new Date(),()=>{}])assert.throws(()=>runtime.encodeInput(v),sdk.ValidationError);
 const cycle={};cycle.x=cycle;assert.throws(()=>runtime.encodeInput(cycle),sdk.ValidationError);
 assert.equal(runtime.encodeInput({small:1,wide:18446744073709551615n,decimal:"0.000000000000000001"}),'{"small":1,"wide":18446744073709551615,"decimal":"0.000000000000000001"}');
 const data=runtime.decodeOutput(runtime.native.roundTrip('{"small":1,"wide":18446744073709551615,"decimal":0.1000000000000000001,"$serde_json::private::Number":"123"}'));
 assert.equal(data.small,1);assert.equal(data.wide,18446744073709551615n);assert.equal(data.decimal,"0.1000000000000000001");assert.equal(data["$serde_json::private::Number"],"123");
});
test("typed book integers are bigint and unknown numbers remain lossless",async t=>{
 const url=await server(t,(_req,res)=>res.end(fixtures("hyperliquid","book.json").replace(/}\s*$/,',"wide":18446744073709551615,"fraction":0.1000000000000000001}')));
 const c=client(t,sdk.HyperliquidClient,{httpUrl:url,wsUrl:"ws://127.0.0.1:1"});
 const book=await c.l2Book({coin:"ETH"});assert.equal(book.time,1754450974231n);assert.equal(book.levels[0][0].n,2n);assert.equal(book.levels[0][0].px,"3000.00");assert.equal(book.wide,18446744073709551615n);assert.equal(book.fraction,"0.1000000000000000001");
 await c.close();assert.equal(c.closed,true);await assert.rejects(c.meta(),sdk.ClosedError);
});
test("public HTTP fixtures pass through each exchange's Rust adapter",async t=>{
 const cases=[
  ["HyperliquidClient","hyperliquid","meta.json","meta",{}],
  ["LighterClient","lighter","markets.json","orderBooks",{}],
  ["QfexClient","qfex","markets.json","refdata",{}],
  ["ArcusClient","arcus","markets.json","markets",{}],
  ["MonacoClient","monaco","markets.json","tradingPairs",{}],
  ["MetricClient","metric","chains.json","chains",{}],
 ];
 for(const [name,venue,file,method,args] of cases){await t.test(venue,async t=>{
   const url=await server(t,(_req,res)=>res.end(fixtures(venue,file)));
   const c=client(t,sdk[name],name==="MetricClient"?{apiUrl:url,chainId:1n}:{httpUrl:url,wsUrl:"ws://127.0.0.1:1",...(name==="MonacoClient"?{chainId:10143n}:{}),...(name==="QfexClient"?{tradeWsUrl:"ws://127.0.0.1:1"}:{})});
   assert.ok(await c[method](args));
 });}
 for(const venue of ["derive","paradex"]){await t.test(venue,async t=>{
   const coverage=JSON.parse(fs.readFileSync(path.join(__dirname,"../../exchanges",venue,"coverage.json")));const vectors=fixture(venue,"contracts.json");let current;let requests=0;
   const url=await server(t,async(req,res)=>{requests++;assert.equal(req.method.toLowerCase(),current.verb.toLowerCase());assert.equal(new URL(req.url,"http://peer").pathname,new URL(current.route,"http://peer").pathname);res.end(runtime.encodeInput(venue==="derive"?{result:current.response}:current.response));});
   const token=[Buffer.from("{}").toString("base64url"),Buffer.from(JSON.stringify({exp:Math.floor(Date.now()/1000)+3600})).toString("base64url"),"test"].join(".");
   const configuration=venue==="derive"?{httpUrl:url+"/v3",chainId:11155111n,matching:"0xeB8d770ec18DB98Db922E9D83260A585b9F0DeAD",signer:new sdk.DeriveSigner({privateKey:"0x"+"01".padStart(64,"0")})}:{httpUrl:url,account:"0x1234",token,readOnly:false};
   const c=client(t,sdk[venue==="derive"?"DeriveClient":"ParadexClient"],{...configuration,readRetries:0});
   for(const row of coverage.rest){const contract=vectors[row.method];if(venue==="paradex"&&row.method==="block_trades_create"){const signature=Object.values(contract.body.signatures)[0];contract.body.signatures["0x1234"]=signature;contract.request.body=contract.body;}
     current={...contract,verb:row.verb};const method=row.method.replace(/_([a-z])/g,(_,x)=>x.toUpperCase());await assert.doesNotReject(c[method]({request:contract.request}),row.method);
   }
   assert.equal(requests,coverage.rest.length);
 });}
});
test("Hyperliquid reference signing vectors and external callbacks",async()=>{
 const vectors=runtime.decodeOutput(runtime.native.roundTrip(fixtures("hyperliquid","signing_vectors.json")));const signer=new sdk.HyperliquidSigner({privateKey:vectors.public_test_private_key});assert.equal(signer.address.toLowerCase(),vectors.signer_address.toLowerCase());
 for(const v of vectors.l1){const options={action:v.action,nonce:BigInt(v.nonce),network:v.network.toLowerCase(),vaultAddress:v.vault_address,expiresAfter:v.expires_after===null?undefined:BigInt(v.expires_after)};const signature=await signer.signAction(options);assert.deepEqual(signature.signature,v.signature,v.name);}
 const v=vectors.l1[0];const options={action:v.action,nonce:BigInt(v.nonce),network:v.network.toLowerCase()};let calls=0;
 const remote=sdk.HyperliquidSigner.remote({address:signer.address,callback:async payload=>{calls++;assert.ok(payload.digest);return (await signer.signAction(options)).signature;}});
 assert.deepEqual(await remote.signAction(options),await signer.signAction(options));assert.equal(calls,1);
 const rejected=sdk.HyperliquidSigner.remote({address:signer.address,callback:async()=>{throw new Error("private secret");}});await assert.rejects(rejected.signAction(options),error=>error instanceof sdk.SigningError && !error.message.includes("private secret"));
});
test("exact scaled values and validation preserve integer ranges",()=>{
 assert.equal(sdk.scaleLighterDecimal({value:"18446744073709551615",decimals:0}),18446744073709551615n);
 assert.throws(()=>sdk.scaleLighterDecimal({value:"18446744073709551616",decimals:0}),sdk.ValidationError);
 assert.throws(()=>sdk.scaleLighterDecimal({value:"1.001",decimals:2}),sdk.ValidationError);
});
test("structured exchange errors retain details without exposing raw response in message",async t=>{
 const url=await server(t,(_req,res)=>{res.statusCode=403;res.end('{"error":"secret-echo"}');});const c=client(t,sdk.HyperliquidClient,{httpUrl:url,wsUrl:"ws://127.0.0.1:1",readRetries:0});
 await assert.rejects(c.meta(),error=>error instanceof sdk.ExchangeError&&error.httpStatus===403&&error.body.includes("secret-echo")&&!error.message.includes("secret-echo"));
});
test("Lighter reference payloads, local signatures, and async remote signers",async()=>{
 for(const v of fixture("lighter","signing_vectors.json").vectors){
  const local=new sdk.LighterSigner({privateKey:v.key});assert.equal(local.publicKey,v.public_key);
  if(v.kind==="auth"){
   const options={accountIndex:BigInt(v.account_index),apiKeyIndex:v.api_key_index,deadline:BigInt(v.deadline),chainId:v.chain_id};const payload=sdk.lighterAuthSigningPayload(options);assert.equal(Buffer.from(payload.digest).toString("hex"),v.digest);assert.equal(payload.verify({signature:v.signature,publicKey:v.public_key}),true);assert.equal(payload.verify({signature:await local.sign({payload}),publicKey:local.publicKey}),true);continue;
  }
  const info=v.tx_info;const options={kind:v.kind,request:info,accountIndex:BigInt(info.AccountIndex),apiKeyIndex:info.ApiKeyIndex,nonce:BigInt(info.Nonce),chainId:v.chain_id,expiredAt:BigInt(info.ExpiredAt),attributes:info.L2TxAttributes||{}};
  const remote=sdk.LighterSigner.remote({publicKey:v.public_key,callback:async payload=>{assert.equal(Buffer.from(payload.digest).toString("hex"),v.digest);assert.equal(payload.purpose.type,"transaction");assert.equal(payload.purpose.nonce,info.Nonce);return v.signature;}});
  assert.equal((await remote.signTransaction(options)).digest,v.digest);
  const signed=await local.signTransaction(options);assert.equal(signed.digest,v.digest);
  const payload=sdk.lighterTransactionSigningPayload(options);assert.equal(Buffer.from(payload.digest).toString("hex"),v.digest);assert.equal(payload.verify({signature:Buffer.from(signed.tx_info.Sig,"base64").toString("hex"),publicKey:local.publicKey}),true);
 }
});
test("concurrent clients share native nonces and ambiguous writes are sent once",async t=>{
 const nonces=[];let disconnect=false;
 const url=await server(t,async(req,res)=>{let body="";for await(const chunk of req)body+=chunk;const data=JSON.parse(body);nonces.push(data.nonce);if(disconnect){req.socket.destroy();return;}res.end('{"status":"ok","response":{"type":"order","data":{"statuses":[{"resting":{"oid":9007199254740993}}]}}}');});
 const privateKey="0x"+"02".repeat(32);const signer=new sdk.HyperliquidSigner({privateKey});
 const opts={httpUrl:url,wsUrl:"ws://127.0.0.1:1",signer};const first=client(t,sdk.HyperliquidClient,opts),second=client(t,sdk.HyperliquidClient,opts);
 const action=fixture("hyperliquid","signing_vectors.json").l1[0].action;const {type,...request}=action;
 const results=await Promise.all([first.order({request}),second.order({request})]);assert.equal(new Set(nonces).size,2);assert.equal(results[0].response.data.statuses[0].resting.oid,9007199254740993n);
 disconnect=true;await assert.rejects(first.order({request}),error=>error instanceof sdk.TransportError&&error.outcome==="unknown");assert.equal(nonces.length,3);
});
test("closing during an async signer callback prevents submission",async t=>{
 let requests=0;const url=await server(t,(_req,res)=>{requests++;res.end("{}");});let release,entered;const started=new Promise(resolve=>entered=resolve);
 const local=new sdk.HyperliquidSigner({privateKey:"0x"+"03".repeat(32)});const signer=sdk.HyperliquidSigner.remote({address:local.address,callback:async()=>{entered();return new Promise(resolve=>release=resolve);}});
 const c=client(t,sdk.HyperliquidClient,{httpUrl:url,wsUrl:"ws://127.0.0.1:1",signer});const action=fixture("hyperliquid","signing_vectors.json").l1[0].action;const {type,...request}=action;
 const pending=assert.rejects(c.order({request}),sdk.ClosedError);await started;await c.close();await pending;release({r:"0x"+"00".repeat(32),s:"0x"+"00".repeat(32),v:27});assert.equal(requests,0);
});
test("Metric named contract builders and helper wrappers call Rust",async t=>{
 const c=client(t,sdk.MetricChainClient,{chainId:1n,readRpc:"http://127.0.0.1:1"});const address="0x1111111111111111111111111111111111111111";
 const contract=c.contract({name:"AnchoredPriceProvider",address});const actual=contract.prepareSetConfidenceParam({args:[1n]});const expected=c.prepareCall({contract:"AnchoredPriceProvider",function:"setConfidenceParam(uint256)",to:address,args:[1n]});assert.deepEqual(actual,expected);assert.match(actual.data,/^0x/);
 assert.ok(sdk.metricTools.codebook());assert.ok(sdk.metricTools.addresses({chainId:1n}));
});
test("Derive, Paradex, Monaco and Metric async signer callbacks retain native behavior",async()=>{
 const key="0x"+"01".repeat(32),digest="0x"+"02".repeat(32);
 const derive=new sdk.DeriveSigner({privateKey:key});const deriveRemote=sdk.DeriveSigner.remote({address:derive.address,callback:payload=>derive.sign({payload})});const derivePayload={digest,scheme:"action",chain_id:11155111n};assert.equal(await deriveRemote.sign({payload:derivePayload}),await derive.sign({payload:derivePayload}));
 const paradex=new sdk.ParadexSigner({privateKey:key});const paradexRemote=sdk.ParadexSigner.remote({publicKey:paradex.publicKey,kind:"starknet",callback:payload=>paradex.sign({payload})});const paradexPayload={digest,scheme:"starknet_message"};assert.deepEqual(await paradexRemote.sign({payload:paradexPayload}),await paradex.sign({payload:paradexPayload}));
 const wallet=new sdk.MonacoWalletSigner({privateKey:key});const walletRemote=sdk.MonacoWalletSigner.remote({address:wallet.address,callback:message=>wallet.signMessage({message})});assert.equal(await walletRemote.signMessage({message:"test"}),await wallet.signMessage({message:"test"}));
 const session=new sdk.MonacoSessionSigner({seed:"01".repeat(32)});const sessionRemote=sdk.MonacoSessionSigner.remote({publicKey:session.publicKey,callback:message=>session.sign({message})});assert.equal(await sessionRemote.sign({message:[1,2,3]}),await session.sign({message:[1,2,3]}));
 const metric=new sdk.MetricSigner({privateKey:key});const metricRemote=sdk.MetricSigner.remote({address:metric.address,callback:payload=>metric.signDigest({digest:payload.digest})});assert.equal(await metricRemote.signDigest({digest}),await metric.signDigest({digest}));
});
