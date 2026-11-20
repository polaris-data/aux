"use strict";

const assert = require("node:assert/strict");
const { once } = require("node:events");
const fs = require("node:fs");
const http = require("node:http");
const path = require("node:path");
const test = require("node:test");
const { WebSocketServer } = require("ws");
const sdk = require("../js/index.cjs");

const key = "hdk_sk_fixture_secret";
const fixtures = JSON.parse(fs.readFileSync(path.join(__dirname, "../../data/hyperdash/tests/dispatch.json"), "utf8"));

async function listen(t, handler) {
  const server = http.createServer(handler);
  server.listen(0, "127.0.0.1");
  await once(server, "listening");
  t.after(() => new Promise(resolve => {
    server.closeAllConnections();
    server.close(resolve);
  }));
  return server;
}

test("Hyperdash REST inventory, typed precision, formats and problem details", async t => {
  const requests = [];
  let response;
  const server = await listen(t, async (req, res) => {
    const chunks = [];
    for await (const chunk of req) chunks.push(chunk);
    requests.push([req.method, new URL(req.url, "http://local"), req.headers, Buffer.concat(chunks).toString()]);
    res.writeHead(response.status, response.headers);
    res.end(response.body);
  });
  const client = new sdk.HyperdashClient({
    apiKey: key, apiUrl: `http://127.0.0.1:${server.address().port}`,
  });
  t.after(() => client.close());
  assert.equal(fixtures.length, 259);
  for (const row of fixtures) {
    response = {status: row.status, headers: {}, body: JSON.stringify(row.response)};
    assert.equal(typeof client[row.node], "function", row.node);
    const output = await client[row.node](row.request);
    assert.equal(output.status, row.status);
    const [method, url, headers, body] = requests.at(-1);
    assert.equal(method, row.method);
    assert.equal(url.pathname, row.path);
    assert.deepEqual(Object.fromEntries(url.searchParams), row.query);
    assert.equal(headers.authorization, row.auth ? `Bearer ${key}` : undefined);
    if (row.body !== null) assert.deepEqual(JSON.parse(body), row.body);
    else assert.equal(body, "");
  }
  response = {
    status: 200, headers: {"X-Credits-Charged": "3"},
    body: '{"data":[{"t":18446744073709551615,"o":"1.2300","q":null,"future":true},{"o":"2.000"}],"has_more":false,"next_cursor":null,"meta":{}}',
  };
  const result = await client.getCandles({coin: "xyz:TSLA", max_credits: 20n});
  assert.equal(requests.at(-1)[1].pathname, "/v1/markets/xyz%3ATSLA/candles");
  assert.equal(requests.at(-1)[2]["x-max-credits"], "20");
  assert.equal(result.metadata.credits_charged, 3n);
  assert.deepEqual(result.data.data, [{t: 18446744073709551615n, o: "1.2300", q: null, future: true}, {o: "2.000"}]);
  response = {status: 200, headers: {"X-Next-Cursor": "after"}, body: '{"c":"1.2300"}\n'};
  const text = await client.getCandlesNdjson({coin: "BTC"});
  assert.equal(requests.at(-1)[2].accept, "application/x-ndjson");
  assert.equal(text.data, response.body);
  assert.equal(text.metadata.next_cursor, "after");
  response = {status: 429, headers: {"Retry-After": "3"}, body: JSON.stringify({code: "rate_limited", detail: key})};
  await assert.rejects(client.getCandles({coin: "BTC"}), error => {
    assert.ok(error instanceof sdk.DataVendorError);
    assert.equal(error.httpStatus, 429);
    assert.equal(error.code, "rate_limited");
    assert.equal(error.metadata.retry_after, "3");
    assert.equal(error.native.detail, "[redacted]");
    return true;
  });
  const esm = await import("../js/index.mjs");
  assert.equal(esm.HyperdashClient, sdk.HyperdashClient);
});

test("Hyperdash WebSocket connects, acknowledges explicitly and exposes close metadata", {timeout: 10_000}, async t => {
  const server = await listen(t);
  const wss = new WebSocketServer({server});
  t.after(() => new Promise(resolve => {
    for (const socket of wss.clients) socket.terminate();
    wss.close(resolve);
  }));
  const client = new sdk.HyperdashClient({
    apiKey: key,
    wsUrl: `ws://127.0.0.1:${server.address().port}/v1/stream`,
  });
  t.after(() => client.close());
  const connection = once(wss, "connection");
  const pendingStream = client.openStream();
  const [socket, request] = await connection;
  assert.equal(request.headers.authorization, `Bearer ${key}`);
  socket.send('{"op":"ready","key_id":"test","environment":"live"}');
  const stream = await pendingStream;
  t.after(() => stream.close());
  assert.equal((await stream.next()).value.op, "ready");
  const subscribed = once(socket, "message");
  await stream.subscribe({channel: "trades", coin: "BTC", cursor: "saved"});
  assert.deepEqual(JSON.parse((await subscribed)[0]), {
    op: "subscribe", channel: "trades", coin: "BTC", cursor: "saved",
  });
  socket.send('{"op":"data","sub":"trades","batch":18446744073709551615,"cursor":"next","events":[],"credits":1}');
  const {value: batch} = await stream.next();
  assert.equal(batch.batch, 18446744073709551615n);
  const acknowledgement = once(socket, "message");
  await stream.ack(batch.sub, batch.batch);
  assert.match(String((await acknowledgement)[0]), /"batch":18446744073709551615/);
  await assert.rejects(stream.ack(batch.sub, batch.batch), sdk.ValidationError);
  socket.send('{"op":"acked","sub":"trades","batch":18446744073709551615,"credits_charged":1}');
  socket.close(1008, "credits_exhausted");
  assert.equal((await stream.next()).value.credits_charged, 1n);
  await assert.rejects(stream.next(), error => error.closeCode === 1008 && error.closeReason === "credits_exhausted");
  assert.equal((await stream.next()).done, true);
});
