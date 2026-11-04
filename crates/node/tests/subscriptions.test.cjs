"use strict";
const {test}=require("node:test");
const assert=require("node:assert/strict");
const {WebSocketServer}=require("ws");
const {once}=require("node:events");
const fs=require("node:fs");
const path=require("node:path");
const sdk=require(process.env.AUX_NODE_PACKAGE || "../js/index.cjs");
const book=JSON.parse(fs.readFileSync(path.join(__dirname,"../../exchanges/hyperliquid/tests/fixtures/book.json")));
async function setup(t,options={}){
 const peers=[];const wss=new WebSocketServer({host:"127.0.0.1",port:0});await once(wss,"listening");
 let subscribedResolve;let subscribed=new Promise(resolve=>subscribedResolve=resolve);const subscriptions=[];
 wss.on("connection",socket=>{peers.push(socket);socket.on("message",raw=>{const message=JSON.parse(String(raw));if(message.method==="subscribe"){subscriptions.push(message);socket.send(JSON.stringify({channel:"subscriptionResponse",data:message}));subscribedResolve(socket);}if(message.method==="ping")socket.send('{"channel":"pong"}');});});
 const c=new sdk.HyperliquidClient({httpUrl:"http://127.0.0.1:1",wsUrl:`ws://127.0.0.1:${wss.address().port}`,timeoutMs:1000,initialBackoffMs:10,maxBackoffMs:10,...options});
 t.after(async()=>{await c.close();for(const s of peers)s.terminate();await new Promise(resolve=>wss.close(resolve));});
 return {c,peers,subscriptions,async subscribe(){const s=await c.subscribeL2Book({coin:"ETH"});const socket=await subscribed;return {s,socket};},fresh(){subscribed=new Promise(resolve=>subscribedResolve=resolve);return subscribed;}};
}
function send(socket,n=1){for(let i=0;i<n;i++)socket.send(JSON.stringify({channel:"l2Book",data:{...book,time:book.time+i}}));}
test("subscriptions yield typed data and early iteration exit closes the handle",{timeout:5000},async t=>{
 const peer=await setup(t);const {s,socket}=await peer.subscribe();send(socket);
 for await(const value of s){assert.equal(value.coin,"ETH");assert.equal(typeof value.time,"bigint");break;}
 assert.equal(s.closed,true);assert.deepEqual(await s.next(),{value:undefined,done:true});
});
test("a second pending read fails and client close wakes the first",{timeout:5000},async t=>{
 const peer=await setup(t);const {s}=await peer.subscribe();const first=s.next();await assert.rejects(s.next(),sdk.BusyError);await peer.c.close();assert.deepEqual(await first,{value:undefined,done:true});assert.equal(s.closed,true);
});
test("overflow is terminal and remains visible to a slow consumer",{timeout:5000},async t=>{
 const peer=await setup(t,{subscriptionCapacity:1});const {s,socket}=await peer.subscribe();send(socket,20);await new Promise(resolve=>setTimeout(resolve,50));await assert.rejects(s.next(),error=>error instanceof sdk.TransportError&&error.kind==="overflow");assert.equal(s.closed,true);
});
test("reconnection reports a gap before fresh data",{timeout:5000},async t=>{
 const peer=await setup(t);const {s,socket}=await peer.subscribe();send(socket);assert.equal((await s.next()).value.coin,"ETH");const connected=peer.fresh();socket.close();const gap=assert.rejects(s.next(),sdk.GapError);const fresh=await connected;await gap;send(fresh);assert.equal((await s.next()).value.coin,"ETH");assert.equal(s.closed,false);
});
