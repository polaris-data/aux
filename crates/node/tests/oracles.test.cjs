"use strict";
const { test } = require("node:test");
const assert = require("node:assert/strict");
const http = require("node:http");
const { once } = require("node:events");
const { createHash, createHmac } = require("node:crypto");
const { WebSocketServer } = require("ws");
const sdk = require(process.env.AUX_NODE_PACKAGE || "../js/index.cjs");
const { decodeOutput, native } = require("../js/runtime.cjs");
const pyth = require("../../data/pyth/tests/fixtures/operations.json");
const chainlink = require("../../data/chainlink/tests/fixtures/operations.json");
const reports = require("../../data/chainlink/tests/fixtures/reports.json");
const camel = name => name.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());

async function listen(t, server) {
  server.listen(0, "127.0.0.1");
  await once(server, "listening");
  t.after(() => new Promise(resolve => { server.closeAllConnections(); server.close(resolve); }));
  return `http://127.0.0.1:${server.address().port}`;
}

test("Oracle inventory preserves exact data and native wire parameters", async t => {
  let row, received;
  const base = await listen(t, http.createServer(async (request, response) => {
    const chunks = [];
    for await (const chunk of request) chunks.push(chunk);
    received = { method: request.method, url: new URL(request.url, "http://fixture"), body: Buffer.concat(chunks), headers: request.headers };
    response.writeHead(row.status, row.headers);
    for (let index = 0; index < row.response.length; index += 3) response.write(row.response.slice(index, index + 3));
    response.end();
  }));
  const clients = {
    PythProClient: new sdk.PythProClient({config: {api_url: base + "/v1", router_url: base, api_key: "fixture-key"}}),
    PythHermesClient: new sdk.PythHermesClient({config: {hermes_url: base + "/hermes", api_key: "fixture-key"}}),
    ChainlinkDataStreamsClient: new sdk.ChainlinkDataStreamsClient({config: {network: "testnet", api_url: base, api_key: "fixture-key", api_secret: "fixture-secret"}}),
    ChainlinkCandlestickClient: new sdk.ChainlinkCandlestickClient({config: {network: "testnet", candles_url: base, token: "fixture-token"}}),
  };
  try {
    for (row of [...pyth, ...chainlink].filter(row => !row.kind.endsWith("_ws"))) {
      const result = clients[row.client][camel(row.method)]({request: row.request});
      if (row.status === 400) { await assert.rejects(result, sdk.DataVendorError); continue; }
      const value = await result;
      if (row.kind === "sse" || row.kind === "objects") {
        if (row.kind === "sse") assert.deepEqual((await value.next()).value.binary.data, ["abcd"]);
        else {
          assert.equal((await value.next()).value.heartbeat, 1748525528);
          assert.equal((await value.next()).value.p, "1.0000000000000000001e+21");
        }
        await value.close();
      } else {
        assert.equal(value.status, row.status);
        if (row.method === "authorize") {
          assert.equal(value.data.d.expiration, 1747203979n);
          assert.equal(value.data.d.access_token, "fixture-token");
        } else assert.deepEqual(value.data, row.kind === "text" ? row.response : decodeOutput(native.roundTrip(row.response)));
      }
      assert.equal(received.method, row.verb);
      assert.equal(received.url.pathname, row.path);
      assert.deepEqual(Object.fromEntries([...new Set(received.url.searchParams.keys())].map(key => [key, received.url.searchParams.getAll(key)])), row.query);
      if (row.kind === "post") assert.deepEqual(JSON.parse(received.body), row.body);
      if (row.kind === "form") assert.deepEqual(Object.fromEntries(new URLSearchParams(received.body.toString())), row.body);
    }
    for (const row of reports) {
      const decoded = clients.ChainlinkDataStreamsClient.decodeReport({fullReport: row.full_report});
      assert.equal(decoded.version, row.version);
      assert.equal(decoded.report_body, row.report_body);
      if (row.key_field) assert.equal(decoded.fields[row.key_field], row.key_value);
      else assert.equal(decoded.kind, "unknown");
    }
  } finally { await Promise.all(Object.values(clients).map(client => client.close())); }
});

test("Chainlink WebSocket reconnect signs each upgrade and signals a gap", async t => {
  const server = http.createServer();
  const base = await listen(t, server);
  const sockets = new WebSocketServer({server});
  const times = [];
  const fixture = chainlink.find(row => row.kind === "chainlink_ws");
  const report = {feedID: fixture.request.feedIDs[0], fullReport: reports[1].full_report};
  sockets.on("connection", (ws, request) => {
    assert.equal(request.url, `/api/v1/ws?feedIDs=${report.feedID}`);
    const timestamp = request.headers["x-authorization-timestamp"];
    const digest = createHash("sha256").digest("hex");
    const signature = createHmac("sha256", "fixture-secret").update(`GET ${request.url} ${digest} fixture-key ${timestamp}`).digest("hex");
    assert.equal(request.headers["x-authorization-signature-sha256"], signature);
    times.push(Number(timestamp));
    ws.send(JSON.stringify({report}));
    if (times.length === 1) setTimeout(() => ws.close(1000), 50);
  });
  const client = new sdk.ChainlinkDataStreamsClient({config: {network: "testnet", ws_url: base.replace("http", "ws"), api_key:"fixture-key", api_secret:"fixture-secret"}});
  try {
    const stream = await client.subscribeReports({request: fixture.request});
    assert.deepEqual((await stream.next()).value.report, report);
    await assert.rejects(stream.next(), sdk.GapError);
    assert.deepEqual((await stream.next()).value.report, report);
    assert.ok(times[1] > times[0]);
    await stream.close();
  } finally { await client.close(); sockets.close(); }
});

test("Pyth binary messages and Merkle roots retain their native payloads", async t => {
  const server = http.createServer();
  const base = await listen(t, server);
  const sockets = new WebSocketServer({server});
  const nativePrice = {timestampUs: "1700000000000001", priceFeeds: [{priceFeedId: 1, price: "9007199254740993"}]};
  const parsed = Buffer.from(JSON.stringify(nativePrice));
  const frame = Buffer.alloc(18 + parsed.length);
  frame.writeUInt32LE(461928307);
  frame.writeBigUInt64BE(1n, 4);
  frame.writeUInt16BE(4 + parsed.length, 12);
  frame.writeUInt32LE(3302625434, 14);
  parsed.copy(frame, 18);
  const root = {root: "001122", slot: "9007199254740993", timestamp: 1700000000, channel: "fixed_rate@200ms", signature: "abcd", messages: ["1234"]};
  sockets.on("connection", (ws, req) => {
    assert.equal(req.headers.authorization, "Bearer fixture-key");
    if (req.url === "/v1/merkle/root/stream") { ws.send(JSON.stringify(root)); return; }
    ws.once("message", raw => {
      const request = JSON.parse(raw);
      assert.equal(request.deliveryFormat, "binary");
      ws.send(frame);
    });
  });
  const client = new sdk.PythProClient({config: {api_key:"fixture-key", ws_urls:[base.replace("http", "ws") + "/v1/stream"]}});
  try {
    const fixture = pyth.find(row => row.kind === "pyth_ws");
    const prices = await client.subscribe({request: {...fixture.request, deliveryFormat:"binary"}});
    const update = (await prices.next()).value;
    assert.equal(update.binary, frame.toString("base64"));
    assert.deepEqual(update.parsed, nativePrice);
    await prices.close();
    const roots = await client.streamMerkleRoots();
    assert.deepEqual((await roots.next()).value, root);
    await roots.close();
  } finally { await client.close(); sockets.close(); }
});

test("Chainlink signed requests reject redirects and redact vendor errors", async t => {
  let status = 302;
  let calls = 0;
  const base = await listen(t, http.createServer((_req, res) => {
    calls++;
    res.writeHead(status, {location: "/other"});
    res.end('{"detail":"fixture-key / fixture-secret"}');
  }));
  const client = new sdk.ChainlinkDataStreamsClient({config: {network: "testnet", api_url:base, api_key:"fixture-key", api_secret:"fixture-secret"}});
  try {
    const request = chainlink.find(row => row.method === "latest_report").request;
    for (status of [302, 403, 429]) {
      await assert.rejects(client.latestReport({request}), error => {
        assert.equal(error.httpStatus, status);
        assert.equal(error.native.detail, "[redacted] / [redacted]");
        return true;
      });
    }
    assert.equal(calls, 3);
  } finally { await client.close(); }
});
