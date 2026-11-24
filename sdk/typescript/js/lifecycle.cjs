"use strict";
const {AsyncLocalStorage} = require("node:async_hooks");
const {ClosedError} = require("./errors.cjs").errors;
const active = new AsyncLocalStorage();
const states = new WeakMap();

function state(client) {
  if (!states.has(client)) states.set(client, {closed:false, pending:new Set(), streams:new Set()});
  return states.get(client);
}

function ensureOpen(s = active.getStore()) {
  if (s?.closed) throw new ClosedError("client is closed");
}

function track(s, result) {
  if (typeof result?.close === "function") {
    if (s.closed) result.close();
    else {
      for (const ref of s.streams) if (!ref.deref()) s.streams.delete(ref);
      s.streams.add(new WeakRef(result));
    }
  }
  ensureOpen(s);
  return result;
}

function managed(Client, synchronous) {
  const syncMethods = new Set(synchronous);
  for (const name of Object.getOwnPropertyNames(Client.prototype)) {
    if (name === "constructor" || name.startsWith("_")) continue;
    const descriptor = Object.getOwnPropertyDescriptor(Client.prototype,name);
    const method = descriptor.value;
    if (typeof method !== "function") continue;
    descriptor.value = function(...args) {
      const s = state(this);
      if (syncMethods.has(name)) {
        ensureOpen(s);
        return active.run(s, () => method.apply(this,args));
      }
      if (s.closed) return Promise.reject(new ClosedError("client is closed"));
      return new Promise((resolve,reject) => {
        s.pending.add(reject);
        let result;
        try { result = active.run(s, () => method.apply(this,args)); }
        catch (error) { s.pending.delete(reject); reject(error); return; }
        Promise.resolve(result).then(value => track(s,value)).then(resolve,reject)
          .finally(() => s.pending.delete(reject));
      });
    };
    Object.defineProperty(Client.prototype,name,descriptor);
  }
  Object.defineProperties(Client.prototype,{
    closed:{get() { return state(this).closed; }},
    close:{value() {
      const s=state(this);
      if (!s.closed) {
        s.closed=true;
        for (const reject of s.pending) reject(new ClosedError("client is closed"));
        s.pending.clear();
        for (const ref of s.streams) ref.deref()?.close();
        s.streams.clear();
      }
    }}
  });
}
module.exports = {managed,ensureOpen};
