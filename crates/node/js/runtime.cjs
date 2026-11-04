"use strict";
const path = require("node:path");
const {ConnectorError, ValidationError, errors} = require("./errors.cjs");
const platforms = {"darwin-arm64":"darwin-arm64", "darwin-x64":"darwin-x64", "linux-x64":"linux-x64-gnu"};
const target = platforms[`${process.platform}-${process.arch}`];
if (!target) throw new Error(`aux-ts does not provide a native binary for ${process.platform}/${process.arch}. Supported: macOS arm64/x64 and Linux x64 GNU.`);
if (process.platform === "linux" && !process.report?.getReport().header.glibcVersionRuntime) throw new Error("aux-ts requires GNU libc on Linux; musl is not supported.");
let native;
try { native = require(path.join(__dirname, "..", `aux.${target}.node`)); }
catch (cause) { throw new Error(`aux-ts native binary for ${target} is missing or could not be loaded. Reinstall the package for this platform.`, {cause}); }
// Explicit JSON numeric tokens support exchange fields that are numbers on the wire.
class ExactNumber {
  constructor(text) {
    if (typeof text!=="string" || !/^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?$/.test(text)) throw new ValidationError("Invalid exact JSON number");
    Object.defineProperty(this,"text",{value:text});Object.freeze(this);
  }
}
const exactNumber=text=>new ExactNumber(text);
// A dedicated encoder writes bigint tokens directly. JSON.stringify(bigint) throws,
// while converting them to Number would silently corrupt IDs and signing payloads.
function encodeInput(value, depth=0, seen=new Set()) {
  if (value instanceof ExactNumber) return value.text;
  if (depth>=128) throw new ValidationError("Native request nesting exceeds 128 levels");
  if (value===null || value===undefined) return "null";
  switch (typeof value) {
    case "string": return JSON.stringify(value);
    case "boolean": return String(value);
    case "bigint": return String(value);
    case "number":
      if (!Number.isSafeInteger(value)) throw new ValidationError("Use bigint for large integers and strings for decimal values");
      return String(value);
    case "object": {
      if (seen.has(value)) throw new ValidationError("Native requests cannot contain cycles");
      const proto=Object.getPrototypeOf(value);
      if (!Array.isArray(value) && !(value instanceof Uint8Array) && proto!==Object.prototype && proto!==null) throw new ValidationError("Native requests must use plain objects, arrays, strings, integers, booleans or null");
      seen.add(value);
      try {
        if (Array.isArray(value) || value instanceof Uint8Array) return "["+Array.from(value,v=>encodeInput(v,depth+1,seen)).join(",")+"]";
        return "{"+Object.keys(value).filter(k=>value[k]!==undefined).map(k=>JSON.stringify(k)+":"+encodeInput(value[k],depth+1,seen)).join(",")+"}";
      } finally {seen.delete(value);}
    }
    default: throw new ValidationError("Unsupported native request value");
  }
}
function decodeNode(node) {
  switch(node[0]) {
    case "null": return null;
    case "boolean": case "number": case "string": return node[1];
    case "bigint": return BigInt(node[1]);
    case "bytes": return Uint8Array.from(node[1]);
    case "array": return node[1].map(decodeNode);
    case "object": return Object.fromEntries(node[1].map(([key,value])=>[key,decodeNode(value)]));
    default: throw new ConnectorError("Invalid native response tag");
  }
}
function decodeOutput(text) { return text===undefined ? undefined : decodeNode(JSON.parse(text)); }
function nativeError(error) {
  if (!(error instanceof Error) || !error.message.startsWith("AUX_ERROR:")) return error;
  try {
    const data=decodeOutput(error.message.slice("AUX_ERROR:".length));
    const Ctor=errors[data.name] || ConnectorError;
    const result=new Ctor(data.message);
    for (const [key,value] of Object.entries(data)) if(key!=="message" && key!=="name") result[key.replace(/_([a-z])/g,(_,x)=>x.toUpperCase())]=value;
    return result;
  } catch { return new ConnectorError("Unable to decode native error"); }
}
function invoke(fn, transform=value=>value) {
  try {
    const result=fn();
    return result instanceof Promise ? result.then(transform,error=>{throw nativeError(error);}) : transform(result);
  } catch(error) { throw nativeError(error); }
}
function wrap(Ctor, value) { const result=Object.create(Ctor.prototype);result.__native=value;return result; }
function callbackAdapter(callback) {
  if (typeof callback!=="function") throw new ValidationError("Signer callback must be a function");
  return async text=>encodeInput(await callback(decodeOutput(text)));
}
class Subscription {
  get closed(){return this.__native.closed;}
  async next(){const [done,value]=await invoke(()=>this.__native.next(),decodeOutput);return done?{value:undefined,done:true}:{value,done:false};}
  async close(){return invoke(()=>this.__native.close());}
  async return(){await this.close();return {value:undefined,done:true};}
  [Symbol.asyncIterator](){return this;}
}
module.exports={native,encodeInput,decodeOutput,invoke,wrap,callbackAdapter,Subscription,ExactNumber,exactNumber};
