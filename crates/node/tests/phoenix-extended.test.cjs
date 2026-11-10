const test = require('node:test');
const assert = require('node:assert/strict');
const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');
const sdk = require('../js/index.cjs');
const root = path.resolve(__dirname,'../../exchanges');
const fixture = (venue,name) => JSON.parse(fs.readFileSync(path.join(root,venue,name),'utf8'));
const camel = name => name.replace(/_([a-z])/g,(_,c)=>c.toUpperCase());
test('Phoenix and Extended expose every native REST contract',async t=>{
 for(const [venue,Client] of [['phoenix',sdk.PhoenixClient],['extended',sdk.ExtendedClient]]){
  let expected;const failures=[];
  const server=http.createServer(async(req,res)=>{
   try{const chunks=[];for await(const chunk of req)chunks.push(chunk);const raw=Buffer.concat(chunks).toString();assert.equal(req.method,expected.op.verb.toUpperCase());assert.equal(req.url,expected.contract.route);assert.deepEqual(raw?JSON.parse(raw):null,expected.contract.body);if(venue==='extended')assert.ok(req.headers['user-agent']);res.setHeader('content-type','application/json');res.end(JSON.stringify(expected.contract.response));}
   catch(e){failures.push(e);res.statusCode=500;res.end('{}');}
  });
  await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  const client=new Client({httpUrl:`http://127.0.0.1:${server.address().port}`,chainId:venue==='extended'?'SN_SEPOLIA':'mainnet-beta',apiKey:'fixture',accessToken:'fixture'});
  try{const contracts=fixture(venue,'tests/fixtures/contracts.json');for(const op of fixture(venue,'coverage.json').rest){expected={op,contract:contracts[op.method]};const result=await client[camel(op.method)]({request:expected.contract.request});if(expected.contract.response?.future_field)assert.deepEqual(result.future_field,expected.contract.response.future_field);}assert.deepEqual(failures,[]);}
  finally{await client.close();server.closeAllConnections();await new Promise(resolve=>server.close(resolve));}
  assert.equal(client.closed,true);
 }
});
test('Phoenix native and remote signers produce matching message signatures',async()=>{
 const signer=new sdk.PhoenixSigner({privateKey:'01'.repeat(32)});
 const messageBase64=Buffer.from('hello').toString('base64');
 const remote=sdk.PhoenixSigner.remote({publicKey:signer.publicKey,callback:async payload=>{assert.equal(payload.scheme,'phoenix_ed25519');return signer.signMessage({messageBase64:payload.message_base64});}});
 assert.equal(await signer.signMessage({messageBase64}),await remote.signMessage({messageBase64}));
});
test('Extended signing uses the same Rust preparation in Node',async()=>{
 const client=new sdk.ExtendedClient();
 try{const signer=new sdk.ExtendedSigner({privateKey:'0x1'});const order={market:'BTC-USD',side:'BUY',qty:'0.001',price:'100',fee:'0.0005',nonce:'7',expiryEpochMillis:1705626537000n};const context={synthetic_asset_id:'0x2',collateral_asset_id:'0x1',synthetic_resolution:1000000n,collateral_resolution:1000000n,collateral_position:10002};const prepared=client.prepareOrder({publicKey:signer.publicKey,order,context});const signed=await client.signOrder({signer,order,context});assert.equal(signed.id,BigInt(prepared.hash).toString());assert.match(signed.settlement.signature.r,/^0x/);}
 finally{await client.close();}
});
test('wallet onboarding helpers and Stark signature envelopes remain native',async()=>{
 const signature='0x9ef64d5936681edf44b4a7ad713f3bc24065d4039562af03fccf6a08d6996eab367df11439169b417b6a6d8ce81d409edb022597ce193916757c7d5d9cbf97301c';
 const signer=sdk.ExtendedSigner.fromWalletSignature({signature});assert.equal(signer.publicKey,'0x78298687996aff29a0bbcb994e1305db082d084f85ec38bb78c41e6787740ec');assert.match((await signer.signHash({hash:'0x1'})).r,/^0x/);
 const client=new sdk.ExtendedClient();try{const registration={accountIndex:0,wallet:'0x'+'01'.repeat(20),tosAccepted:false};const payload=client.keyDerivationPayload({registration});assert.equal(payload.typed_data.primaryType,'AccountCreation');assert.equal(payload.typed_data.domain.name,'starknet.sepolia.extended.exchange');assert.equal(payload.typed_data.message.tosAccepted,false);assert.equal(client.apiRequestPayload({path:'/api/v1/user/account/api-key',timestamp:'2026-11-09T00:00:00Z'}).personal_message,'/api/v1/user/account/api-key@2026-11-09T00:00:00Z');}finally{await client.close();}
});
test('Phoenix v0 transaction preparation and signing cross the Node boundary exactly',async()=>{
 const client=new sdk.PhoenixClient();try{const signer=new sdk.PhoenixSigner({privateKey:'01'.repeat(32)});const prepared=client.prepareTransaction({request:{version:'v0',instructions:[{program_id:'11111111111111111111111111111111',accounts:[],data:[1]}],fee_payer:signer.publicKey,recent_blockhash:'11111111111111111111111111111111',last_valid_block_height:500n,lookup_tables:[]}});const signatures=await signer.signTransaction({prepared});const signed=client.assembleTransaction({prepared,signatures});client.validateServerTransaction({prepared,transactionBase64:signed.transaction_base64});assert.equal(signed.last_valid_block_height,500n);}finally{await client.close();}
});
test('captured live public payloads preserve native string-encoded integers',async t=>{
 for(const [venue,Client,method] of [['phoenix',sdk.PhoenixClient,'listMarkets'],['extended',sdk.ExtendedClient,'getMarkets']]){
  const raw=fs.readFileSync(path.join(root,venue,'tests/fixtures/public_markets.json'));
  const server=http.createServer((req,res)=>res.end(raw));await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
  const client=new Client({httpUrl:`http://127.0.0.1:${server.address().port}`,chainId:venue==='extended'?'SN_SEPOLIA':'mainnet-beta'});
  try{const result=await client[method]();if(venue==='phoenix')assert.equal(result[0].maxLiquidationSizeBaseLots,'1820000');else assert.equal(result.data[0].tradingConfig.maxNumOrders,'200');}
  finally{await client.close();server.closeAllConnections();await new Promise(resolve=>server.close(resolve));}
 }
});
test('every Phoenix instruction builder preserves native bytes and account metadata',async()=>{
 const client=new sdk.PhoenixClient();
 try{for(const [name,contract] of Object.entries(fixture('phoenix','tests/fixtures/instruction_dispatch.json'))){assert.deepEqual(client.buildInstruction({name,request:contract.request}),contract.instruction,name);}}
 finally{await client.close();}
});
test('native stream bindings correlate RPC replies and close with their parent',{timeout:10000},async()=>{
 const {WebSocketServer}=require('ws');const {once}=require('node:events');
 for(const [venue,Client] of [['phoenix',sdk.PhoenixClient],['extended',sdk.ExtendedClient]]){
  const wss=new WebSocketServer({host:'127.0.0.1',port:0});await once(wss,'listening');const peers=[];
  wss.on('connection',socket=>{peers.push(socket);socket.once('message',raw=>{const request=JSON.parse(String(raw));let frame;if(venue==='extended'){socket.send(JSON.stringify({id:'unrelated',result:{wrong:true}}));socket.send(JSON.stringify({id:request.id,result:{}}));frame={subscription:'orderbooks.BTC-USD',type:'ORDERBOOKS.SNAPSHOT',seq:1,data:{m:'BTC-USD',b:[],a:[]}};}else{frame={channel:'orderbook',symbol:'SOL',sequenceNumber:1,data:{bids:[],asks:[]}};}frame.exact='0.123456789012345678901';socket.send(JSON.stringify(frame));});});
  const client=new Client({wsUrl:`ws://127.0.0.1:${wss.address().port}`,chainId:venue==='extended'?'SN_SEPOLIA':'mainnet-beta'});
  try{const stream=await client.subscribe({specification:venue==='extended'?{channel:'orderbooks',market:'BTC-USD',rpc:true}:{channel:'orderbook',symbol:'SOL'}});if(venue==='extended')assert.deepEqual((await stream.next()).value.result,{});assert.equal((await stream.next()).value.exact,'0.123456789012345678901');await client.close();assert.equal((await stream.next()).done,true);}
  finally{await client.close();for(const socket of peers)socket.terminate();await new Promise(resolve=>wss.close(resolve));}
 }
});
