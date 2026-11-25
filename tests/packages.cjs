"use strict";
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const http = require("node:http");
const sdk = require(require.resolve("aux-connectors", {paths: [process.cwd()]}));

function lossless(text, inputs = false) {
  return JSON.parse(text, (_, value, context) => {
    if (typeof value !== "number") return value;
    if (/[.eE]/.test(context.source)) return inputs ? JSON.rawJSON(context.source) : context.source;
    return Number.isSafeInteger(value) ? value : BigInt(context.source);
  });
}
const file = name => fs.readFileSync(path.join(__dirname, name + ".json"), "utf8");
const fixture = name => lossless(file(name));
const stringify = value => JSON.stringify(value, (_, v) => typeof v === "bigint" ? JSON.rawJSON(String(v)) : v);
function compare(actual, expected) {
  if (Array.isArray(actual)) {
    assert.equal(actual.length, expected.length);
    actual.forEach((value, i) => compare(value, expected[i]));
  } else if (["number", "bigint"].includes(typeof actual) && typeof expected === "string") {
    assert.equal(String(actual), expected);
  } else assert.deepEqual(actual, expected);
}

{
  const f = fixture("pascal-solana");
  const params = {owner:f.owner,source:f.source,destination:f.destination,mint:f.mint,amount:f.amount,decimals:f.decimals,
    recent_blockhash:f.blockhash,last_valid_block_height:18446744073709551615n};
  assert.equal(sdk.solana.associatedTokenAddress(f.owner,f.mint), f.associated_token_address);
  const prepared = sdk.solana.prepareDeposit(params);
  assert.equal(Buffer.from(prepared.message).toString("hex"),f.message_hex);
  const signed = sdk.solana.signDeposit("07".repeat(32),prepared);
  assert.deepEqual(signed,{signature:f.signature,transaction_base64:f.transaction_base64,last_valid_block_height:params.last_valid_block_height});
  assert.throws(()=>sdk.solana.signDeposit("08".repeat(32),prepared));
}

for (const dataset of ["http", "prediction-http", "data-http", "perps-http"]) {
  for (const row of lossless(file(dataset), true)) {
    const {venue, operation} = row;
    const params = row.request_json ? lossless(row.request_json, true) : row.request;
    const req = sdk.prepareRequest(venue, operation, params);
    if ("route" in row) assert.equal(req.path, row.route, operation);
    else {
      const url = new URL(req.path, "http://fixture");
      assert.equal(url.pathname, row.path, operation);
      const pairs = Object.fromEntries(url.searchParams);
      for (const key of Object.keys(pairs)) if (Array.isArray(row.query[key])) pairs[key] = url.searchParams.getAll(key);
      assert.deepEqual(pairs, row.query);
    }
    const body = row.body_json || stringify(row.body ?? null);
    assert.deepEqual(req.body, lossless(body), operation);
    assert.equal(req.method, row.verb);
    assert.equal(req.authenticated, row.private);
    for (const [k, v] of Object.entries(row.headers || {})) assert.equal(req.headers[k], v);
    const text = row.response_json || stringify(row.response);
    if (row.status >= 300) {
      assert.throws(() => sdk.decodeResponse(venue, row.status, text));
      continue;
    }
    assert.deepEqual(sdk.decodeResponse(venue, 200, venue === "derive" ? '{"result":' + text + '}' : text), lossless(text));
  }
}
for (const row of fixture("polymarket-helpers")) {
  const op = row.operation.replace(/_(.)/g, (_,c) => c.toUpperCase());
  assert.deepEqual(sdk.polymarket[op](...Object.values(row.request)), row.expected, op);
}
for (const row of fixture("metric-helpers")) {
  const op = row.operation.replace(/_(.)/g, (_, c) => c.toUpperCase());
  assert.deepEqual(sdk.metric[op](row.request), row.expected, op);
}
for (const row of fixture("metric-abi")) {
  const c = row.contract, sig = row.signature;
  if (row.kind === "function") {
    assert.equal(sdk.metric.encodeContractCall(c,sig,row.args),row.input);
    compare(sdk.metric.decodeContractInput(c,sig,row.input),row.args);
    compare(sdk.metric.decodeContractOutput(c,sig,row.output),row.outputArgs);
  } else if (row.kind === "event") {
    const result = sdk.metric.decodeContractEvent(c,sig,row.topics,row.data);
    compare(result.indexed,row.indexed);
    compare(result.body,row.body);
  } else compare(sdk.metric.decodeContractError(c,row.input).arguments,row.args);
}
for (const row of fixture("polymarket-abi")) {
  assert.equal(sdk.polymarket.encodeContractCall(row.interface,row.function,row.args),row.calldata);
  compare(sdk.polymarket.decodeContractInput(row.interface,row.function,row.calldata),row.args);
}
for (const row of fixture("chainlink-reports")) {
  const result = sdk.decodeReport(row.full_report);
  assert.equal(result.version,row.version);
  assert.equal(result.report_body,row.report_body);
  if (result.kind === "known") assert.equal(result.fields[row.key_field],row.key_value);
}
for (const {key,prepared,signed,hash,address} of fixture("evm-signing")) assert.deepEqual(sdk.signTransaction(key,prepared),{signed,hash,address});
for (const row of fixture("paradex-sbe").frames)
  assert.deepEqual(sdk.decodeSbe(Buffer.from(row.hex,"hex")).data,row.native);
{
  const records = fs.readFileSync(path.join(__dirname,"schema-records.dbn"));
  let offset = 8 + records.readUInt32LE(4);
  for (const row of fixture("schema-records")) {
    const size = 4 * records[offset];
    assert.deepEqual(sdk.decodeDbnRecord(records.subarray(offset,offset+size)),row.record);
    offset += size;
  }
  assert.equal(offset,records.length);
  const ts = fs.readFileSync(path.join(__dirname,"ts-out.dbn"));
  assert.equal(sdk.decodeDbnRecord(ts.subarray(8+ts.readUInt32LE(4)),true).ts_out,"1704067200000000999");
}
const {qfex, databento, arcus, monaco, kalshi} = fixture("auth");
assert.equal(sdk.kalshiHeaders(kalshi.key,kalshi.seed,kalshi.method,kalshi.path,kalshi.timestamp)["KALSHI-ACCESS-SIGNATURE"],kalshi.signature);
assert.equal(sdk.qfexSignature(qfex.secret,qfex.nonce,qfex.timestamp),qfex.hmac);
assert.equal(sdk.databentoCram(databento.key,databento.challenge),databento.response);
assert.deepEqual(sdk.arcusSign(arcus.seed,arcus.payload),{signature:arcus.signature,public_key:arcus.public_key});
for (const f of monaco) assert.equal(sdk.monacoSign(f.privateKey,f.method,f.target,f.body,f.created).Signature,f.signature);

for (const r of fixture("extended-settlement")) {
  assert.deepEqual(sdk.extendedSettlement(r.kind,r.chain_id,r.public_key,r.message,r.stark_key),r.expected);
}
for (const r of fixture("extended-orders")) {
  assert.deepEqual(sdk.extendedPrepareOrder(r.chain_id,r.public_key,r.body,r.context),r.prepared);
  assert.deepEqual(sdk.extendedSignOrder(r.chain_id,r.stark_key,r.body,r.context),r.signed);
}

async function testSocket(venue = "qfex") {
  const {spawn}=require("node:child_process");
  const peer=spawn("python3",[path.join(__dirname,"ws_server.py"),venue],{stdio:["ignore","pipe","inherit"]});
  const ended=new Promise((resolve,reject)=>{peer.once("error",reject);peer.once("exit",resolve);});
  try {
    const lines=require("node:readline").createInterface({input:peer.stdout});
    const endpoint=(await lines[Symbol.asyncIterator]().next()).value;
    if (venue === "extended") {
      for (const [name,row] of Object.entries(fixture("extended-ws")).sort(([a],[b])=>a<b?-1:1)) {
        const client = new sdk.ExtendedClient(row.private ? {apiKey:"fixture"} : {});
        const stream = await client.subscribe(row.specification,endpoint.replace(/[/]stream$/,"/v1"));
        assert.deepEqual(await Array.fromAsync(stream),Array(name==="orderbooks_rpc"?2:1).fill(row.frame));
        stream.close();
      }
      assert.equal(await ended,0);
      return;
    }
    if (venue === "kalshi" || venue === "pascal") {
      const kalshi = venue === "kalshi";
      const client = kalshi ? new sdk.KalshiClient({apiKey:"fixture",apiSecret:"07".repeat(32)}) : new sdk.PascalClient();
      const stream = await client.subscribe({service:kalshi ? "predictions" : "read",params:{channel:kalshi ? "orderbook_delta" : "book"}},endpoint);
      const rows = [];
      for await (const event of stream) rows.push(event.seq);
      assert.deepEqual(rows,[1,2,3]);
      assert.equal(await ended,0);
      return;
    }
    if (venue === "paradex") {
      const client = new sdk.ParadexClient({bearer:{account:"0x1234",token:"test.eyJleHAiOjQxMDI0NDQ4MDB9.test"}});
      const frames = new Map(fixture("paradex-sbe").frames.map(row=>[row.template,row.native]));
      for (const [kind,row] of Object.entries(fixture("paradex-ws")).sort(([a],[b])=>a < b ? -1 : 1)) {
        const stream = await client.subscribe(row.specification,endpoint);
        const event = (await stream.next()).value;
        assert.deepEqual(event.params,{channel:row.channel,data:frames.get(row.template) ?? row.payload});
        if (kind === "account") {
          assert.equal((await stream.next()).value.method,"heartbeat");
          assert.equal((await stream.next()).value.method,"subscribed");
        }
        assert.equal((await stream.next()).done,true);
        stream.close();
      }
      assert.equal(await ended,0);
      return;
    }
    if (venue === "derive") {
      const {owner, key} = fixture("derive-signing");
      const client = new sdk.DeriveClient({apiKey: owner, apiSecret: key});
      const rows = fixture("derive-ws");
      for (const name of Object.keys(rows).sort()) {
        const row = rows[name];
        const stream = await client.subscribe(row.specification, endpoint);
        assert.deepEqual((await stream.next()).value.params, {channel:row.channel,data:row.payload});
        assert.equal((await stream.next()).done,true);
        stream.close();
      }
      assert.equal(await ended,0);
      return;
    }
    if (venue==="phoenix") {
      const client=new sdk.PhoenixClient({apiKey:"fixture-key"});
      const rows=fixture("phoenix-ws");
      for (const name of Object.keys(rows).sort()) {
        const stream=await client.subscribe(rows[name].specification,endpoint);
        assert.deepEqual((await stream.next()).value,rows[name].frame);
        if(name==="exchange") assert.deepEqual((await stream.next()).value.decodedPayload.data,[1,2,3]);
        assert.equal((await stream.next()).done,true);
        stream.close();
      }
      assert.equal(await ended,0);
      return;
    }
    const stream=await new sdk.QfexClient().subscribe({channel:"trade",symbol:"BTC"},endpoint);
    assert.equal((await stream.next()).value.seq,9999999999999999999n);
    assert.equal((await stream.next()).value.seq,2);
    assert.equal((await stream.next()).done,true);
    stream.close();stream.close();
    assert.equal(await ended,0);
  } finally {peer.kill();}
}

async function testPerpsAndParadex() {
  const golden = fixture("polymarket-perps-signing");
  const token = "test.eyJleHAiOjQxMDI0NDQ4MDB9.test";
  let expected;
  const server = http.createServer(async (req,res) => {
    const {row, body, headers} = expected;
    assert.equal(req.method, row.verb);
    assert.equal(req.url, row.route);
    const chunks = [];
    for await (const chunk of req) chunks.push(chunk);
    assert.deepEqual(chunks.length ? lossless(Buffer.concat(chunks).toString()) : null, body);
    for (const name of ["authorization","polymarket-secret","polymarket-proxy"]) assert.equal(req.headers[name], headers[name]);
    res.end('{"ok":true}');
  });
  await new Promise(resolve => server.listen(0,"127.0.0.1",resolve));
  try {
    const origin = "http://127.0.0.1:"+server.address().port;
    const pd = new sdk.ParadexClient({origin,bearer:{token,account:"0x1234",read_only:false}});
    const perps = new sdk.PolymarketPerpsClient({origin,perps:{
      proxy:golden.proxy,secret:"fixture",proxy_key:golden.key,owner_key:golden.key,withdrawal_contract:golden.withdrawal_contract
    }});
    const kinds = name => new Map(fs.readFileSync(path.join(__dirname,"../src",name),"utf8").trim().split("\n").map(line => line.split(" ")));
    const scopes = kinds("exchanges/paradex/bearer-routes.txt"), actions = kinds("exchanges/polymarket/perps-routes.txt");
    for (const row of [...fixture("http"),...fixture("prediction-http")]) {
      const paradex = row.venue === "paradex" && scopes.has(row.operation);
      if (!paradex && (row.venue !== "polymarket" || !actions.has(row.operation))) continue;
      const name = row.operation.split(".").at(-1).replace(/_(.)/g,(_,c)=>c.toUpperCase());
      const params = row.request_json ? lossless(row.request_json,true) : row.request;
      expected = {row,body:row.body_json ? lossless(row.body_json) : row.body,headers:{}};
      if (paradex && row.operation === "block_trades_create") {
        params.body.signatures = expected.body.signatures = {"0x1234":expected.body.signatures.fixture};
      }
      if (paradex) expected.headers.authorization = "Bearer "+token;
      else if (actions.get(row.operation) === "proxy") {
        expected.headers["polymarket-secret"] = "fixture";
        expected.headers["polymarket-proxy"] = golden.proxy;
      } else {
        delete params.body.sig;
        expected.body = {...row.body,sig:golden.signatures[row.operation]};
      }
      assert.deepEqual(await (paradex ? pd : perps)[name](params),{ok:true},row.operation);
    }
    for (const row of fixture("polymarket-clob-orders")) {
      const client = new sdk.PolymarketClient({origin,
        wallet:{key:golden.key,order_domain:row.domain},
        clob:{address:golden.proxy,key:"api",secret:"YQ==",passphrase:"pass"}});
      expected = {row:{verb:"POST",route:"/order"},body:{order:{...row.order,signature:row.signature}},headers:{}};
      assert.deepEqual(await client.clobPostOrder({body:{order:row.order}}),{ok:true});
    }
  } finally { await new Promise(resolve => server.close(resolve)); }
}

function testTradingSignatures() {
  const f = fixture("paradex-signing");
  for (const row of f.vectors) {
    for (const [key, secret, expected] of [["stark_key",f.key,row.signature],["evm_key",f.evm_key,row.evm_signature]]) {
      const client = new sdk.ParadexClient({bearer:{account:f.account,network:row.network,[key]:secret}});
      assert.deepEqual(client.signTypedData(row.typed_data),{digest:row.digest, signature:expected},row.name);
    }
  }
  for (const row of fixture("pascal-permits")) {
    const client = new sdk.PascalClient({permit:row.auth});
    const signed = client.signPermitAt(row.operation,row.body,1731536000000);
    assert.equal(signed.auth.signature,row.signature,row.operation);
  }
}

async function testNetwork() {
  testTradingSignatures();
  for (const version of [1,2]) {
    const base = path.join(__dirname, "legacy-v"+version);
    const expected = fs.readFileSync(base+".jsonl","utf8").trim().split("\n").map(line=>lossless(line));
    const [header,...records] = await Array.fromAsync(sdk.decodeDbnFile(fs.readFileSync(base+".dbn")));
    assert.deepEqual([header.metadata,...records], expected);
  }
  for (const file of ["schema-records.dbn","schema-records.dbn.zst"]) {
    const decoded = await Array.fromAsync(sdk.decodeDbnFile(fs.readFileSync(path.join(__dirname,file))));
    assert.equal(decoded[0].metadata.dataset,"GLBX.MDP3");
    assert.deepEqual(decoded.slice(1),fixture("schema-records").map(row=>row.record));
  }
  const crypto = require("node:crypto");
  const rsa = crypto.generateKeyPairSync("rsa", {modulusLength:2048});
  const pss = {padding:crypto.constants.RSA_PKCS1_PSS_PADDING,saltLength:32};
  const server = http.createServer((req,res) => {
    if (req.url.startsWith("/rsa/")) {
      const stamp = req.headers["kalshi-access-timestamp"];
      assert.match(stamp, /^\d+$/);
      assert.equal(req.headers["kalshi-access-key"], "fixture-rsa");
      const signed = Buffer.from(stamp + req.method + req.url.split("?")[0]);
      assert.ok(crypto.verify("sha256",signed,{key:rsa.publicKey,...pss},Buffer.from(req.headers["kalshi-access-signature"],"base64")));
      assert.ok(["/rsa/margin/balance?compute_available_balance=true","/rsa/portfolio/balance?subaccount=1"].includes(req.url));
      res.end('{"balance":123}');
      return;
    }
    if (req.url === "/order") {
      let body = "";
      req.on("data", chunk => {body += chunk;});
      req.on("end", () => {
        assert.equal(req.method,"DELETE");
        assert.deepEqual(JSON.parse(body),{orderID:"test"});
        const tag = require("node:crypto").createHmac("sha256","public-test-secret")
          .update(req.headers.poly_timestamp+"DELETE/order"+body).digest("base64").replaceAll("+","-").replaceAll("/","_");
        assert.equal(tag,req.headers.poly_signature);
        res.end('{"ok":true}');
      });
      return;
    }
    if (req.url.startsWith("/v3/")) {
      let data = "";
      req.on("data", chunk => { data += chunk; });
      req.on("end", () => {
        const body = JSON.parse(data);
        const name = req.url.split("/").at(-1);
        const f = fixture("derive-orders");
        if (req.url.startsWith("/v3/public/")) {
          assert.equal(req.headers["x-derivesignature"],undefined);
          res.end(JSON.stringify({result:f.metadata[body.instrument_name ?? "currencies"]}));
        } else {
          const expected = f.orders[name];
          if (name === "transfer_positions" || name === "transfer_positions_debug") {
            assert.equal(body.maker_params.signature,expected.maker);
            assert.equal(body.taker_params.signature,expected.taker);
          } else assert.equal(body.signature,expected.signature);
          res.end(JSON.stringify({result:name}));
        }
      });
      return;
    }
    if (req.url === "/rpc") {
      let data = "";
      req.on("data", chunk => { data += chunk; });
      req.on("end", () => {
        const body = JSON.parse(data);
        const row = fixture("evm-rpc").find(r => r.method === body.method);
        assert.deepEqual(body.params, row.args);
        res.end(JSON.stringify({jsonrpc:"2.0",id:body.id,result:row.reply}));
      });
      return;
    }
    if (req.url.startsWith("/v2/updates/price/stream?")) {
      assert.equal(req.headers.accept,"text/event-stream");
      res.writeHead(200, {"content-type":"text/event-stream"});
      res.write('data: {"price":9999999999999999999}\r\n\r\n');
      return;
    }
    assert.equal(req.headers["x-api-key"], "fixture");
    assert.equal(req.url, "/defi/price?address=Test");
    res.end('{"success":true,"data":0.1234567890123456789}');
  });
  await new Promise(resolve => server.listen(0,"127.0.0.1",resolve));
  try {
    const rsaOptions = {apiKey:"fixture-rsa",origin:"http://127.0.0.1:"+server.address().port+"/rsa",
      signer:{sign:message => crypto.sign("sha256",message,{key:rsa.privateKey,...pss})}};
    assert.deepEqual(await new sdk.KalshiClient(rsaOptions).getBalance({subaccount:1}),{balance:123});
    assert.deepEqual(await new sdk.KalshiPerpsClient(rsaOptions).getMarginBalance({compute_available_balance:true}),{balance:123});
    const f = fixture("derive-signing");
    const clob = {address:f.owner,key:"test-key",passphrase:"pass",secret:"cHVibGljLXRlc3Qtc2VjcmV0"};
    const pm = new sdk.PolymarketClient({origin:"http://127.0.0.1:"+server.address().port,clob});
    assert.deepEqual(await pm.clobCancelOrder({body:{orderID:"test"}}),{ok:true});
    const trading = new sdk.DeriveClient({origin:"http://127.0.0.1:"+server.address().port,apiKey:f.owner,apiSecret:f.key,chainId:1});
    for (const [name, row] of Object.entries(fixture("derive-orders").orders)) {
      const method = ("private_" + name).replace(/_(.)/g, (_,c) => c.toUpperCase());
      assert.equal(await trading[method](row.params), name);
    }
    const reader = new sdk.EvmReader("http://127.0.0.1:"+server.address().port+"/rpc");
    for (const row of fixture("evm-rpc")) {
      const method = row.operation.replace(/_(.)/g, (_,c) => c.toUpperCase());
      const expected = typeof row.expected === "number" ? BigInt(row.expected) : row.expected;
      assert.deepEqual(await reader[method](...Object.values(row.params)), expected);
    }
    const result = await new sdk.BirdeyeClient({origin:"http://127.0.0.1:"+server.address().port,apiKey:"fixture"}).price({address:"Test"});
    assert.equal(result.data,"0.1234567890123456789");
    const client=new sdk.PythHermesClient({origin:"http://127.0.0.1:"+server.address().port});
    const stream=await client.streamPriceUpdates({request:{ids:["42".repeat(32)]}});
    assert.deepEqual(await stream.next(), {done:false,value:{price:9999999999999999999n}});
    const reading=stream.next();
    stream.close();
    assert.equal((await reading).done,true);
    stream.close();
  } finally { await new Promise(resolve => server.close(resolve)); }
}
async function testPerpsConnectors(origin) {
  for (const [venue, Client] of Object.entries({
    injective: sdk.InjectiveClient, nado: sdk.NadoClient, ondo_perps: sdk.OndoPerpsClient,
    pacifica: sdk.PacificaClient, paragon: sdk.ParagonClient, trade_xyz: sdk.TradeXyzClient, aster: sdk.AsterClient,
  })) {
    const row = fixture("perps-http").find(item => item.venue === venue && item.operation === "markets");
    assert.deepEqual(await new Client({origin:origin + "/perps/" + venue}).markets(row.request), row.response);
  }
  const injective = new sdk.InjectiveClient({origin:origin + "/perps/injective"});
  const book = await injective.subscribe({channel:"order_book",market_id:"0x" + "33".repeat(32)});
  try {
    assert.equal((await book.next()).value.orderbook.sequence, 9007199254740993n);
    assert.equal((await book.next()).done, true);
  } finally {
    book.close();
  }
  const root = origin + "/perps/aster";
  const {key, owner} = fixture("perps-signing");
  const aster = new sdk.AsterClient({auth:{user:owner.toLowerCase(), key, rest_origin:root + "/background"}});
  const account = await aster.subscribe({channel:"account"}, root.replace("http:", "ws:") + "/ws");
  try {
    assert.equal((await account.next()).value.E, 1);
    assert.equal((await account.next()).value.E, 2);
  } finally {
    account.close();
  }
  assert.equal((await account.next()).done, true);
  let lifecycle;
  for (let attempt = 0; attempt < 150; attempt++) {
    lifecycle = await fetch(root + "/background/listen-key-test").then(response => response.json());
    if (lifecycle.includes("DELETE")) break;
    await new Promise(resolve => setTimeout(resolve, 20));
  }
  assert.deepEqual(lifecycle, ["POST", "PUT", "DELETE"]);
}
async function testSharedPeer() {
  const origin = process.env.AUX_FIXTURE_ORIGIN;
  if (!origin) return;
  await testPerpsConnectors(origin);
  const deniedHermes = new sdk.PythHermesClient({origin:origin+"/stream-refused",apiKey:"fixture-stream-credential"});
  for (const [venue,open,message] of [
    ["pyth_hermes", () => deniedHermes.streamPriceUpdates({ids:["ab".repeat(32)]}), "invalid Bearer [redacted]"],
    ["qfex", () => new sdk.QfexClient().subscribe({channel:"bbo",symbol:"BTC"},origin.replace("http:","ws:")+"/stream-refused/ws"), "subscription denied"],
  ]) {
    await assert.rejects(open, error => error instanceof sdk.ExchangeError && error.venue === venue &&
      error.status === 403 && error.body.seq === 9007199254740993n && error.body.message === message);
  }
  assert.equal(new sdk.PascalClient().walletOutcomePending(), false);
  for (const vector of fixture("extended-orders")) {
    const ext = new sdk.ExtendedClient({chainId:vector.chain_id});
    const signer = new sdk.ExtendedSigner({privateKey:vector.stark_key});
    assert.deepEqual(ext.prepareOrder({publicKey:signer.publicKey,order:vector.body,context:vector.context}),vector.prepared);
    assert.deepEqual(await ext.signOrder({signer,order:vector.body,context:vector.context}),vector.signed);
  }
  const duneClient = new sdk.DuneClient({origin:origin+"/dune-args",apiKey:"test-key"});
  for (const row of [...fixture("data-http"), ...fixture("text-http")].filter(r => r.venue === "dune")) {
    const {execution_id,query_id,body,...query} = row.request_json ? lossless(row.request_json,true) : row.request;
    const options = {};
    if (execution_id !== undefined) options.executionId = execution_id;
    if (query_id !== undefined) options.queryId = query_id;
    if (body !== undefined || Object.keys(query).length) options.request = body ?? query;
    const name = row.operation.replace(/_(.)/g, (_,ch) => ch.toUpperCase());
    compare(await duneClient[name](options), row.response_json ? lossless(row.response_json) : row.expected);
  }
  const blocks = fixture("paradex-blocks");
  const proofs = fixture("paradex-signing");
  for (const vector of proofs.vectors.filter(v => Object.hasOwn(blocks, v.name))) {
    const {nonce, expiration, trades, parent} = blocks[vector.name];
    for (const [kind, field, sig] of [["stark_key","key","signature"],["evm_key","evm_key","evm_signature"]]) {
      const client = new sdk.ParadexClient({origin,bearer:{[kind]:proofs[field],network:vector.network,account:proofs.account}});
      const response = await client.blockSignature(nonce,expiration,trades,parent);
      assert.deepEqual(JSON.parse(response.signature_data),vector[sig]);
      assert.equal(response.signature_expiration,expiration);
    }
  }
  const deriveVectors = fixture("derive-signing");
  for (const row of deriveVectors.actions) {
    for (const proof of row.vectors) {
      const client = new sdk.DeriveClient({origin:"https://fixture.invalid",chainId:proof.chain_id,
        apiKey:deriveVectors.owner,apiSecret:deriveVectors.key});
      const body = {nonce:deriveVectors.nonce,subaccount_id:deriveVectors.subaccount_id,signature_expiry_sec:deriveVectors.expiry,extra:"kept"};
      assert.deepEqual(await client.prepareSigned(body,row.data),{...body,signature:proof.signature,signer:deriveVectors.signer.toLowerCase()});
      const raw = {module:row.module,encoded_data:[...Buffer.from(row.encoded.slice(2),"hex")],
        signer:deriveVectors.signer,owner:deriveVectors.owner,expiry:deriveVectors.expiry,nonce:body.nonce,subaccount_id:body.subaccount_id};
      assert.equal(await client.signAction(raw),proof.signature);
      await assert.rejects(client.signAction({...raw,signer:"0x"+"00".repeat(20)}),sdk.ConnectorError);
    }
  }
  const ixClient = new sdk.PhoenixClient();
  for (const [name,row] of Object.entries(fixture("phoenix-instructions"))) {
    assert.deepEqual(ixClient.buildInstruction(name,row.request),row.instruction,name);
  }
  const transactions = fixture("phoenix-transactions");
  for (const row of transactions) {
    const prepared=new sdk.PhoenixClient().prepareTransaction(row.request);
    assert.deepEqual(prepared,row.prepared);
    let signatures=sdk.phoenix.signTransaction("01".repeat(32),prepared);
    assert.throws(()=>sdk.phoenix.assembleTransaction(prepared,signatures));
    signatures=sdk.phoenix.signTransaction("02".repeat(32),prepared,signatures);
    assert.deepEqual(signatures,row.signatures);
    assert.deepEqual(sdk.phoenix.assembleTransaction(prepared,signatures),row.signed);
    sdk.phoenix.validateServerTransaction(prepared,row.signed.transaction_base64);
    sdk.phoenix.validateSignedTransaction(row.signed);
    assert.throws(()=>sdk.phoenix.validateSignedTransaction({...row.signed,signature:"1".repeat(64)}));
  }
  const phoenixTx=transactions[0], chainP=new sdk.PhoenixChainClient(origin+"/phoenix-rpc/node");
  assert.deepEqual(await chainP.prepare(phoenixTx.request.instructions,phoenixTx.request.fee_payer),phoenixTx.prepared);
  assert.equal((await chainP.simulate(phoenixTx.signed)).value.err,null);
  assert.equal(await chainP.submit(phoenixTx.signed),phoenixTx.signed.signature);
  assert.deepEqual(chainP.pending(),phoenixTx.signed);
  assert.equal((await chainP.signatureStatus(phoenixTx.signed.signature)).value[0].confirmationStatus,"confirmed");
  assert.equal((await chainP.transaction(phoenixTx.signed.signature)).slot,9007199254740993n);
  chainP.clearReconciledSubmission();assert.equal(chainP.pending(),null);
  const lostP=new sdk.PhoenixChainClient(origin+"/phoenix-rpc/node/lost");
  await assert.rejects(lostP.submit(phoenixTx.signed),sdk.OutcomeUncertainError);
  await assert.rejects(lostP.submit(phoenixTx.signed),sdk.OutcomeUncertainError);
  assert.deepEqual(lostP.pending(),phoenixTx.signed);lostP.clearReconciledSubmission();
  const rpcUrl=origin.replace("http:","ws:")+"/rpc/";
  const externalKalshi=new sdk.KalshiClient({origin:origin+"/external-kalshi",apiKey:"fixture-k",signer:{
    async sign(bytes) {
      const message=Buffer.from(bytes).toString();
      assert.match(message,/^\d+GET\/external-kalshi\/portfolio\/balance$/);
      return Buffer.from(sdk.arcusSign("07".repeat(32),message).signature,"hex");
    }
  }});
  assert.deepEqual(await externalKalshi.getBalance({subaccount:1}),{balance:123});
  const feed = new sdk.HyperliquidClient();
  const mux = {endpoint:origin.replace("http:","ws:")+"/multiplex/node"};
  const eth = await feed.subscribeL2Book("ETH",mux);
  const btc = await feed.subscribeTrades("BTC",mux);
  const duplicate = await feed.subscribeTrades("BTC",mux);
  const solBook = await feed.subscribeL2Book("SOL",mux);
  assert.equal((await eth.next()).value.time,1);
  assert.equal((await btc.next()).value.data[0].coin,"BTC");
  assert.equal((await duplicate.next()).value.data[0].coin,"BTC");
  assert.equal((await solBook.next()).value.time,2);
  btc.close(); eth.close(); duplicate.close();
  assert.equal((await solBook.next()).value.time,3);
  feed.close();
  for (const [venue, feed, params] of [
    ["lighter", new sdk.LighterClient(), {channel:"order_book",market_id:4}],
    ["qfex", new sdk.QfexClient(), {channel:"bbo",symbol:"BTC"}],
  ]) {
    const rejected = await feed.subscribe(params, origin.replace("http:","ws:")+"/rejected-feed/"+venue);
    await assert.rejects(rejected.next(), e => e instanceof sdk.ExchangeError && e.venue === venue && e.body.error.sequence === 9007199254740993n);
    feed.close();
  }
  const hlAccount = new sdk.HyperliquidClient();
  const accountMux = origin.replace("http:", "ws:") + "/account-scope/node";
  const address1 = "0x"+"11".repeat(20), address2 = "0x"+"22".repeat(20);
  const updates1 = await hlAccount.subscribe({type:"orderUpdates",user:address1},accountMux);
  const openOrders1 = await hlAccount.subscribe({type:"openOrders",user:address1},accountMux);
  const updates2 = await hlAccount.subscribe({type:"orderUpdates",user:address2},accountMux);
  const fills1 = await hlAccount.subscribe({type:"userFills",user:address1,aggregateByTime:true},accountMux);
  assert.equal((await updates1.next()).value.data[0].order.oid, 1);
  assert.equal((await openOrders1.next()).value.data.user, address1);
  assert.equal((await updates2.next()).value.data[0].order.oid, 2);
  assert.deepEqual((await fills1.next()).value.data.fills, []);
  hlAccount.close();
  const qfexFeed = new sdk.QfexClient();
  const qfexMux = origin.replace("http:", "ws:") + "/qfex-multiplex/node";
  const qfexTrades = await qfexFeed.subscribe({channel:"trade",symbol:"*"}, qfexMux);
  const qfexBook = await qfexFeed.subscribe({channel:"bbo",symbol:"BTC"}, qfexMux);
  assert.equal((await qfexTrades.next()).value.seq, 1);
  assert.equal((await qfexTrades.next()).value.seq, 2);
  assert.equal((await qfexBook.next()).value.seq, 9007199254740993n);
  qfexTrades.close();
  assert.equal((await qfexBook.next()).value.seq, 3);
  qfexFeed.close();
  const lighterFeed = new sdk.LighterClient();
  const lighterMux = origin.replace("http:", "ws:") + "/lighter-multiplex/node";
  const firstBook = await lighterFeed.subscribeOrderBook(4, lighterMux);
  const accountFeed = await lighterFeed.subscribeAccount(18, lighterMux);
  for (const generation of [1, 2]) {
    assert.equal((await firstBook.next()).value.sequence, generation);
    assert.equal((await accountFeed.next()).value.sequence, generation);
  }
  const lateBook = await lighterFeed.subscribeOrderBook(4, lighterMux);
  assert.equal((await lateBook.next()).value.sequence, 3);
  lateBook.close();
  firstBook.close();
  assert.equal((await accountFeed.next()).value.sequence, 4);
  lighterFeed.close();
  const owner = new sdk.HyperliquidClient();
  const ownedBook = await owner.subscribeL2Book("ETH",{endpoint:origin.replace("http:","ws:")+"/lifecycle/node"});
  assert.deepEqual((await ownedBook.next()).value,{coin:"ETH",time:23,levels:[[],[]]});
  owner.close(); owner.close();
  assert.equal(owner.closed,true);
  assert.equal((await ownedBook.next()).done,true);
  await assert.rejects(owner.meta({}),sdk.ClosedError);
  const started=Promise.withResolvers(), release=Promise.withResolvers();
  const waiting=new sdk.KalshiClient({origin:origin+"/never-send",apiKey:"fixture-k",signer:{
    async sign() { started.resolve(); await release.promise; return Buffer.alloc(64); }
  }});
  const work=waiting.getBalance({});
  await started.promise;
  waiting.close();
  await assert.rejects(work,sdk.ClosedError);
  release.resolve();
  const metricChain = new sdk.MetricChainClient(1,origin+"/metric-chain/node");
  const metricRow=fixture("metric-chain");
  const polyChain=new sdk.PolymarketChainClient(1,origin+"/poly-contract/node");
  for (const entry of fixture("polymarket-abi")) {
    const options={contract:entry.interface,function:entry.function,args:entry.args,to:"0x"+"11".repeat(20)};
    assert.equal(polyChain.prepareCall(options).data,entry.calldata);
    if (entry.function === "allowance") assert.deepEqual(await polyChain.call({...options,block:"safe"}),[2n**88n+9n]);
  }
  assert.throws(()=>polyChain.decodeError({contract:"IERC20",data:"0xffffffff"}),sdk.ValidationError);
  const contracts = new sdk.MetricChainClient(1,origin+"/metric-contract/node");
  const address=metricRow.request.pool;
  compare(await contracts.contract("AnchoredPriceProvider",address).bpsBase({block:"safe"}),["-3"]);
  const vectors = new Map(fixture("metric-abi").filter(r=>r.kind==="function").map(r=>[r.contract+"."+r.signature,r]));
  const methods = fs.readFileSync(path.join(__dirname,"../src/exchanges/metric/schema/metric-contract-methods.tsv"),"utf8").trim().split("\n");
  assert.equal(methods.length,vectors.size);
  for (const line of methods) {
    const [contract,signature,,method,mode] = line.split("\t");
    const vector=vectors.get(contract+"."+signature);
    const api=contracts.contract(contract,address);
    if (mode === "prepare") {
      assert.deepEqual(api[method]({args:vector.args,value:"0x10"}),{to:address.toLowerCase(),data:vector.input,value:"0x10"});
    } else {
      contracts.reader.ethCall = async (to,data,block,sender,value) => {
        assert.deepEqual([to,data,block,sender,value],[address.toLowerCase(),vector.input,"finalized",undefined,"0x0"]);
        return vector.output;
      };
      compare(await api[method]({args:vector.args,block:"finalized"}),vector.outputArgs);
    }
  }

  assert.deepEqual(await metricChain.oracleStopLossStatus(metricRow.request),metricRow.expected);
  assert.deepEqual(await metricChain.liquiditySnapshot(metricRow.request.pool,metricRow.request.dataProvider,[-1,0,1]),metricRow.snapshot);
  for (const path of metricRow.routes) assert.deepEqual(await metricChain.prepareRoute(path.request),path.expected);
  for (const row of fixture("eip712")) {
    if (row.order) assert.deepEqual(sdk.polymarket.orderTypedData(row.order,row.domain),row.typed_data);
    assert.equal(sdk.polymarket.typedDataHash(row.typed_data),row.digest);
  }
  const xreg=fixture("extended-onboarding"), extReg=new sdk.ExtendedClient({origin});
  assert.deepEqual(extReg.onboardingPayload(xreg.registration),xreg.AccountRegistration);
  assert.equal(sdk.ExtendedSigner.fromWalletSignature(xreg.wallet_signature).publicKey,xreg.public_key);
  const siwe=fixture("paradex-siwe"), sv=siwe.vectors[0];
  const pdAuth=new sdk.ParadexClient({origin,bearer:{account:"0x1234",network:"PRIVATE_SN_PARACLEAR_TESTNET",evm_key:siwe.key}});
  assert.equal(pdAuth.evmAccountAddress(siwe.address,siwe.class_hash),siwe.account);
  assert.equal((await pdAuth.signSiwe(sv.purpose,sv.nonce,sv.issued_at,sv.expiration)).signature,sv.signature);
  await pdAuth.authenticate(); assert.equal(pdAuth.bearer.read_only,false);
  const publicDerive=await new sdk.DeriveClient().openRpc(rpcUrl+"derive-public");
  await assert.rejects(publicDerive.subscribeChannels({channels:["rejection-fixture"]}), error =>
    error instanceof sdk.ExchangeError && error.venue === "derive" && error.body.error.data.seq === 9007199254740993n);
  await assert.rejects(publicDerive.privateCancel({}));
  await publicDerive.unsubscribeChannels({channels:[]}); publicDerive.close();
  const polyKey=fixture("polymarket-perps-signing").proxy;
  const perpsRpc=await new sdk.PolymarketPerpsClient({perps:{proxy:polyKey,secret:"fixture"}}).openRpc(rpcUrl+"perps");
  assert.equal((await perpsRpc.ping()).data,"ping");perpsRpc.close();
  const rfqRpc=await new sdk.PolymarketClient({clob:{address:polyKey,key:"key",secret:"c2VjcmV0",passphrase:"pass"}}).openRpc(rpcUrl+"rfq");
  assert.equal((await rfqRpc.cancelQuote({rfq_id:"r",quote_id:"q"})).rfq_id,"r");
  assert.equal((await rfqRpc.nextEvent()).rfq_id,"other");rfqRpc.close();
  const ro=await new sdk.ParadexClient({bearer:{token:"fixture.eyJleHAiOjQxMDI0NDQ4MDB9.fixture",read_only:true}}).openRpc(rpcUrl+"paradex/readonly");
  await assert.rejects(ro.cancelOrder("forbidden"));
  const channel={type:"orders",market:"ETH-USD-PERP"};
  assert.equal((await ro.subscribeChannel(channel)).ok,"subscribe");
  assert.equal((await ro.nextEvent()).params.seq,9007199254740993n);
  assert.equal((await ro.unsubscribeChannel(channel)).ok,"unsubscribe");
  ro.close();
  for (const [Client,service] of [[sdk.KalshiClient,"kalshi"],[sdk.KalshiPerpsClient,"kalshi_perps"]]) {
    const rpc=await new Client({apiKey:"fixture-k",apiSecret:"07".repeat(32)}).openRpc(rpcUrl+service,{timeout_ms:80});
    await assert.rejects(rpc.subscribe({step:1})); assert.deepEqual(await rpc.pending(),{id:1,method:"subscribe"});
    await assert.rejects(rpc.unsubscribe({step:2}));
    assert.equal((await rpc.nextEvent()).value,9007199254740993n);
    await new Promise(r=>setTimeout(r,300)); assert.equal((await rpc.reconcile()).ack,"subscribe");
    assert.equal(await rpc.pending(),null);
    for (const [i,m] of ["listSubscriptions","updateSubscription","unsubscribe"].entries()) {
      assert.equal((await rpc[m]({step:i+2})).id,String(i+2));
    }
    rpc.close();rpc.close();
  }
  const permits=fixture("pascal-permits");
  const crypto = require("node:crypto");
  const {seed:permitSeed,owner:permitOwner}=permits[0].auth;
  const key = crypto.createPrivateKey({key:Buffer.from("302e020100300506032b657004220420"+permitSeed,"hex"),format:"der",type:"pkcs8"});
  const sign = async message => {
    const bytes = crypto.sign(null,message,key);
    const alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let result = "";
    for (let n=BigInt("0x"+bytes.toString("hex")); n>0n; n/=58n) result = alphabet[Number(n%58n)] + result;
    for (const byte of bytes) {
      if (byte !== 0) break;
      result = "1" + result;
    }
    return result;
  };
  for (const permit of [permits[0].auth, {owner:permitOwner,public_key:permits[0].signer,sign}]) {
    const pascalRpc=await new sdk.PascalClient({permit}).openRpc(rpcUrl+"pascal");
    assert.equal((await pascalRpc.ping()).ack,"ping"); assert.equal((await pascalRpc.nextEvent()).event,"queued");
    for(const [i,m] of ["placeOrders","cancelOrders","massCancel"].entries()) assert.equal((await pascalRpc[m](permits[i].body)).id,String(i+2));
    pascalRpc.close();
  }
  const vectorL=fixture("lighter-signing").vectors[1];
  const identity = {address:"0xabcdef0000000000000000000000000000000001",account_index:2};
  const arcus = new sdk.ArcusClient({origin,signer:{...identity, seed:"07".repeat(32)}});
  for (const row of fixture("data-http").filter(r => r.venue === "arcus" && r.private)) {
    const fn = row.operation === "place_order" ? "placeOrder" : "cancelOrder";
    assert.deepEqual(await arcus[fn](row.request), row.response);
    const payload = arcus.signingPayload(row.operation,row.request);
    const signer = {...identity,...sdk.arcusSign("07".repeat(32),payload.payload),timestamp:payload.timestamp};
    assert.deepEqual(await new sdk.ArcusClient({origin,signer})[fn](row.request), row.response);
  }
  const socketOrigin = origin.replace("http:", "ws:");
  const hdSession = await new sdk.HyperdashClient({apiKey:"hyperdash-key"}).openStream(socketOrigin+"/hd/stream");
  assert.equal((await hdSession.next()).value.op,"ready");
  const pendingData = hdSession.next();
  await hdSession.subscribe({channel:"trades",coin:"BTC"});
  const data = (await pendingData).value;
  assert.equal(data.batch,9007199254740993n);
  await assert.rejects(hdSession.ack(data.sub,String(data.batch)));
  await hdSession.ack(data.sub,data.batch);
  assert.equal((await hdSession.next()).value.credits_charged,2);
  await hdSession.unsubscribe(data.sub);
  assert.equal((await hdSession.next()).value.detail,"[redacted]");
  assert.equal((await hdSession.next()).value.op,"unsubscribed");
  hdSession.close();
  const sharedReader = key => new sdk.ChainlinkDataStreamsClient({apiKey:key,apiSecret:"secret"}).subscribeReports({feedIDs:["ab".repeat(32)]},socketOrigin+"/shared/node/");
  const [one,another,two]=await Promise.all([sharedReader("one"),sharedReader("one"),sharedReader("two")]);
  const step = n => new sdk.MetricClient({origin:origin+"/shared/node",apiKey:String(n)}).chains({});
  assert.deepEqual(await step(1),{one:1,two:1});
  assert.deepEqual((await one.next()).value,(await another.next()).value);
  assert.equal((await two.next()).value.report.fullReport,"two");
  one.close();
  await step(2);
  assert.equal((await another.next()).value.report.seq,2);
  another.close();two.close();
  const reports = await new sdk.ChainlinkDataStreamsClient({apiKey:"fixture", apiSecret:"secret"}).subscribeReports({feedIDs:["0x"+"01".repeat(32),"ab".repeat(32)]}, socketOrigin+"/reports/");
  assert.equal((await reports.next()).value.report.sequence,9007199254740993n);
  await assert.rejects(reports.next());
  reports.close();
  const arcusBook = await new sdk.ArcusClient().subscribe({channel:"l2OrderbookUpdates",id:"BTC",nLevels:10},socketOrigin+"/arcus/ws");
  assert.deepEqual((await arcusBook.next()).value.contents,{bids:[],asks:[]});
  assert.equal((await arcusBook.next()).value.contents.seq,9007199254740993n);
  arcusBook.close();
  const ligh=new sdk.LighterClient({origin,signer:{chain_id:300,account_index:37,api_key_index:3,key:vectorL.key}});
  assert.deepEqual((await ligh.activeOrders({market_id:4096})).orders, []);
  assert.equal((await ligh.nextNonce({account_index:37,api_key_index:3})).nonce, 42);
  const metric = new sdk.MetricClient({origin, apiKey:"fixture"});
  assert.equal(await metric.chains({}), await new sdk.MetricClient({origin, apiKey:"last"}).chains({}));
  await assert.rejects(new sdk.MetricClient({origin,apiKey:"fail"}).chains({}), err =>
    err instanceof sdk.ExchangeError && err.venue === "metric" && err.status === 429 && err.body === "fixture rate limit");
  for (const row of fixture("data-http").filter(r => r.venue === "metric" && r.verb === "POST")) {
    const name = row.operation.replace(/_(.)/g, (_, s) => s.toUpperCase());
    assert.deepEqual(await metric[name](row.request), row.response);
  }
  const perpsws = origin.replace("http:","ws:")+"/perps-ws/";
  const fills=await new sdk.HyperliquidClient().subscribe({type:"userFills",user:"0x"+"12".repeat(20),aggregateByTime:true},perpsws+"hyperliquid/userFills");
  try { assert.deepEqual((await fills.next()).value.data.fills,[]); } finally { fills.close(); }
  const book = await ligh.subscribeOrderBook(4,perpsws+"lighter/order_book");
  try { assert.equal((await book.next()).value.order_book.nonce,9007199254740993n); } finally { book.close(); }
  for (const row of fixture("perps-ws")) {
    const client = row.venue === "hyperliquid" ? new sdk.HyperliquidClient() : ligh;
    const stream = await client.subscribe(row.params, perpsws + row.venue + "/" + row.name);
    try {
      assert.deepEqual((await stream.next()).value, row.event);
      assert.ok((await stream.next()).done);
    } finally { stream.close(); }
  }
  const cancelTx={MarketIndex:vectorL.tx_info.MarketIndex,Index:vectorL.tx_info.Index};
  assert.equal((await ligh.prepareTransaction("cancel_order",cancelTx,{expired_at:vectorL.tx_info.ExpiredAt},42)).digest,vectorL.digest);
  const hl=fixture("hyperliquid-signing"), vector=hl.l1[0];
  for (const row of hl.user_signed) {
    const {signatureChainId,hyperliquidChain,...action}=row.action;
    action.type=row.name==="usd_send"?"usdSend":"approveAgent";
    assert.equal(sdk.HyperliquidClient.userSigningPayload(action,row.network,row.signature_chain_id),row.digest);
    assert.deepEqual(new sdk.HyperliquidClient({signer:{key:hl.public_test_private_key,network:row.network}}).signUserAction(action,row.signature_chain_id),row.signature);
  }
  const walletClient = new sdk.MonacoClient({origin});
  const account = fixture("evm-signing")[0];
  const localWallet = new sdk.MonacoWalletSigner({privateKey:account.key});
  const wallet = sdk.MonacoWalletSigner.remote({address:localWallet.address,callback:message=>localWallet.signMessage({message})});
  const sessionSigner = new sdk.MonacoSessionSigner({seed:Buffer.from([...Array(32).keys()]).toString("hex")});
  const named = new sdk.MonacoClient({origin,chainId:1329n});
  const info = await named.login({wallet,clientId:"named-login",sessionSigner});
  assert.equal(info.address,account.address);
  assert.equal(named.sessionSigner.publicKey,sessionSigner.publicKey);
  assert.equal(info.expiresAt,4102444800n);
  const restored = new sdk.MonacoClient({origin,chainId:1329n});
  assert.throws(()=>restored.restoreSession({signer:new sdk.MonacoSessionSigner({seed:"ff".repeat(32)}),info}));
  restored.restoreSession({signer:sessionSigner,info});
  assert.equal(restored.authenticated,true);
  assert.deepEqual(restored.sessionInfo,info);

  const logged=await walletClient.login({client_id:"login",chain_id:1329,address:"0x"+"21".repeat(20),seed:Buffer.from(Array.from({length:32},(_,i)=>i)).toString("hex")},async msg=>{
    assert.match(msg,/^Authorize 03a107/);
    return "0xabcd";
  });
  assert.equal((await logged.refreshSession()).expiresAt,4102444801);
  for (const row of fixture("monaco-ws")) {
    const url = origin.replace("http:", "ws:") + "/monaco-ws/" + row.params.type;
    const session = {origin, seed:Buffer.from([...Array(32).keys()]).toString("hex"), expires_at:4102444800, ws_origin:url};
    const client = new sdk.MonacoClient({origin, session});
    const stream = await client.subscribe(row.params, url);
    try {
      const event = (await stream.next()).value;
      assert.equal(event.channel, row.channel.replaceAll("*", "ETH"));
      assert.equal(event.data.book_seq, 9007199254740993n);
      if (row.params.type === "depth" || row.params.type === "positions")
        await assert.rejects(stream.next(), sdk.StreamGapError);
      if (row.params.type === "positions")
        assert.equal((await stream.next()).value.code, "SNAPSHOT_UNAVAILABLE");
    } finally { stream.close(); }
  }
  await logged.revokeSession();
  await assert.rejects(logged.balances({}),sdk.AuthenticationError);
  const externalHL = {address:"0x14791697260e4c9a71f18484c9f997b308e59325",network:vector.network,sign:async digest => {
    assert.equal(digest,vector.digest); return vector.signature;
  }};
  const externalOrder = {orders:vector.action.orders,options:{nonce:vector.nonce}};
  assert.equal((await new sdk.HyperliquidClient({origin,signer:externalHL}).order(externalOrder)).status,"ok");
  await assert.rejects(new sdk.HyperliquidClient({origin,signer:{...externalHL,address:"0x"+"22".repeat(20)}}).order(externalOrder));
  const signer={key:hl.public_test_private_key,network:vector.network};
  const orderReply=await new sdk.HyperliquidClient({origin,signer}).order({orders:vector.action.orders,options:{nonce:vector.nonce}});
  assert.deepEqual(orderReply.response.data.statuses[1],{error:"fixture rejection"});
  const urls=["a","b"].map(id=>origin.replace("http:","ws:")+"/pro/"+id);
  const pyth=await new sdk.PythProClient({apiKey:"fixture"}).subscribe({channel:"real_time",symbols:["BTC/USD"],formats:["evm"],subscriptionId:42},urls);
  try {
    const updates=[(await pyth.next()).value,(await pyth.next()).value];
    assert.ok(updates.some(v=>v.price===9007199254740993n));
    assert.ok(updates.some(v=>v.parsed?.price==="1.234567890123456789"));
  } finally { pyth.close(); }
  const tardis = new sdk.TardisClient({origin,apiKey:"fixture"});
  const replay = await tardis.replay({exchange:"bit/mex",from:"2024-02-29T23:59:00Z",to:"2024-03-01T00:02:00Z",filters:[{channel:"trade",symbols:["BTC/USD"]}]});
  try { assert.deepEqual(await replay.readBatch(3),[{localTimestamp:"1700",message:{n:9007199254740993n}},{type:"disconnect"},{localTimestamp:"1702",message:{n:3}}]); }
  finally { replay.close(); }
  const mach = await new sdk.TardisMachineClient({origin:origin+"/machine"}).streamNormalized({exchange:"bit/mex",symbols:["BTC/USD"]});
  try { await mach.send({ping:true}); assert.deepEqual(await mach.readBatch(2),[{seq:9007199254740993n}]); }
  finally { mach.close(); }
  for(const row of fixture("polymarket-ws")) {
    const address="0x"+"21".repeat(20);
    const client=row.service==="perps" ? new sdk.PolymarketPerpsClient({perps:{proxy:address,secret:"fixture-secret"}})
      : new sdk.PolymarketClient({clob:{address,key:"fixture",secret:"c2VjcmV0",passphrase:"fixture-pass"}});
    const handle=await client.subscribe({service:row.service,params:row.params},origin.replace("http:","ws:")+"/poly-ws/"+row.service);
    try { assert.deepEqual((await handle.next()).value,row.event); assert.ok((await handle.next()).done); }
    finally { handle.close(); }
  }
  const client=new sdk.MonacoClient({origin,session:{origin,seed:Buffer.from([...Array(32).keys()]).toString("hex"),expires_at:4102444800}});
  const bal=fixture("data-http").find(r=>r.venue==="monaco" && r.operation==="balances");
  assert.deepEqual(await client.balances(bal.request),bal.response);
  const trade=await new sdk.QfexClient({apiKey:"fixture",apiSecret:"secret"}).tradingSession({timeout_ms:180},origin.replace("http:","ws:")+"/trade");
  try {
    await trade.subscribeAccount();
    const order={symbol:"BTC",side:"BUY",order_type:"LIMIT",order_time_in_force:"GTC",quantity:0.5,price:123,client_order_id:"cid"};
    assert.equal((await trade.addOrder(order)).order_id,"abc");
    assert.equal((await trade.next()).value.total,9007199254740993n);
    await assert.rejects(trade.cancelOrder({symbol:"BTC",order_id:"abc",cancel_order_id_type:"order_id"}),sdk.OutcomeUncertainError);
    await assert.rejects(trade.addOrder(order),sdk.OutcomeUncertainError);
    assert.equal((await trade.next()).value.order_response.status,"canceled");
    assert.deepEqual(await trade.userOrders({offset:0,limit:100}),{orders:[],twaps:[]});
  } finally { trade.close(); }
  await assert.rejects(new sdk.HistoricalClient({origin}).listJobs({}), sdk.AuthenticationError);
  assert.throws(() => sdk.decodeResponse("extended",429,"{}"), sdk.ExchangeError);
  assert.throws(() => sdk.extendedSettlement("invalid","SN_MAIN","0x1",{}),sdk.ValidationError);
  assert.ok(sdk.TransportError.prototype instanceof sdk.ConnectorError);
  const apiKey = fixture("auth").databento.key;
  const hist = new sdk.HistoricalClient({origin,apiKey});
  const params = {symbols:["ETH","BTC"], compression:"zstd",encoding:"csv"};
  assert.deepEqual(Buffer.concat(await Array.fromAsync(await hist.timeseries(params))),fs.readFileSync(path.join(__dirname,"reference-lines.zst")));
  const dest = fs.mkdtempSync(path.join(require("node:os").tmpdir(),"aux-download-"));
  try {
    const output = path.join(dest,"batch.dbn.zst");
    const url = origin+"/files/batch?sig=a%2Fb";
    const bytes = fs.readFileSync(path.join(__dirname,"reference-lines.zst"));
    assert.equal(await hist.downloadTo(url,output),bytes.length);
    assert.deepEqual(fs.readFileSync(output),bytes);
    await assert.rejects(hist.downloadTo(url,output));
    assert.deepEqual(Buffer.concat(await Array.fromAsync(await hist.download(url))),bytes);
  } finally { fs.rmSync(dest,{recursive:true,force:true}); }
  const dbn = await hist.getRange(params);
  const records = await dbn.readBatch(64);
  assert.equal(records[0].metadata.version,3);
  assert.equal(records.at(-1).hd.instrument_id,17);
  dbn.close();
  const refs = new sdk.ReferenceClient({origin,apiKey});
  for (const r of fixture("databento-streams").filter(r=>r.client==="ReferenceClient")) {
    const name = r.operation.replace(/_(.)/g, (_,c)=>c.toUpperCase());
    assert.deepEqual(await Array.fromAsync(await refs[name]({symbols:["ETH","BTC"],compression:"zstd"})),[{id:9007199254740993n,price:"1.25000000000000000001"},{id:2}]);
  }
  const impatient = new sdk.DatabentoLiveClient(fixture("auth").databento.key, {timeoutMs:400,idleTimeoutMs:100});
  await assert.rejects(() => impatient.connect({dataset:"IDLE",gateway:process.env.AUX_DBN_GATEWAY}), sdk.TransportError);
  const live = await new sdk.LiveClient(fixture("auth").databento.key).connect({dataset:"DBN",gateway:process.env.AUX_DBN_GATEWAY,compression:"zstd"});
  assert.equal(live.sessionId,"fixture-session");
  await live.subscribe({schema:"mbo",symbols:["ETH"]});
  await live.startSession();
  const [header] = await live.readBatch(1);
  assert.deepEqual(header.metadata,await live.metadata());
  assert.deepEqual(await live.readBatch(24),fixture("schema-records").map(r=>r.record));
  live.close();
  const options = {origin, apiKey:"test-key"};
  const clients = {dune:new sdk.DuneClient(options),pyth_hermes:new sdk.PythHermesClient(options),pyth_pro:new sdk.PythProClient(options)};
  for (const row of fixture("text-http")) {
    const method = row.operation.replace(/_(.)/g, (_,c) => c.toUpperCase());
    assert.deepEqual(await clients[row.venue][method](row.venue === "dune" ? row.request : {request:row.request}),row.expected);
  }
  const stream = await new sdk.ChainlinkCandlestickClient(options).streamPrices({request:{symbol:["ETHUSD","BTCUSD"]}});
  const events = [];
  for await (const item of stream) events.push(item);
  assert.deepEqual(events, [{heartbeat:1748525528},{p:"1.0000000000000000001e+21",f:'"}{',items:[{},[]]}]);
  stream.close();
  const [tx] = fixture("evm-signing");
  const chain = new sdk.EvmChainClient(1,origin+"/read",origin+"/write");
  assert.equal(await chain.submitRaw(tx.signed),tx.hash);
  const localEvm = new sdk.MetricSigner({privateKey:tx.key});
  const remoteEvm = sdk.MetricSigner.remote({address:localEvm.address,callback:proof=>localEvm.signDigest(proof)});
  const signedMetricChain = new sdk.MetricChainClient({chainId:1n,readRpc:origin+"/read",writeRpc:origin+"/write",signer:remoteEvm});
  assert.equal((await remoteEvm.signTransaction({request:tx.prepared})).bytes,tx.signed);
  assert.equal(await signedMetricChain.code({address:tx.prepared.to}),"0x6000");
  assert.equal(await signedMetricChain.transactionCount({address:tx.address}),3n);
  assert.equal(await signedMetricChain.sendTransaction({request:tx.prepared}),tx.hash);
  signedMetricChain.close();
  assert.equal(signedMetricChain.closed,true);
  await assert.rejects(signedMetricChain.transactionCount({address:tx.address}),sdk.ClosedError);
  const sol = fixture("pascal-solana");
  assert.equal(await new sdk.PascalChainClient(origin+"/solana",sol.mint,sol.decimals).blockHeight(),987654321n);
}
testSharedPeer().then(testNetwork).then(testPerpsAndParadex).then(()=>testSocket()).then(()=>testSocket("extended")).then(()=>testSocket("kalshi")).then(()=>testSocket("pascal")).then(()=>testSocket("phoenix")).then(()=>testSocket("paradex")).then(()=>testSocket("derive")).then(() => console.log("Installed npm package passed wire, codec, authentication, and loopback fixtures")).catch(err => { console.error(err); process.exitCode=1; });
{
  const decoder = new sdk.EventDecoder(64);
  assert.deepEqual(decoder.feed(Buffer.from("data: one\r")), []);
  assert.deepEqual(decoder.feed(Buffer.from("\n\r\ndata: two\n\n")), ["one","two"]);
  decoder.finish(); decoder.close(); decoder.close();
}

{
  const f = fixture("derive-signing");
  assert.deepEqual(sdk.deriveLogin(f.key, f.owner, BigInt(f.login.timestamp)), {wallet:f.owner, ...f.login});
  for (const {data, encoded, module, vectors} of f.actions) {
    for (const {chain_id, digest, signature} of vectors) {
      const action = {owner:f.owner, expiry:f.expiry, nonce:f.nonce, subaccount_id:f.subaccount_id, data, chain_id};
      assert.deepEqual(sdk.deriveSignAction(f.key, action), {encoded, module, signature, digest, signer:f.signer.toLowerCase()});
    }
  }
}
