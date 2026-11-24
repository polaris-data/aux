"use strict";
const {randomBytes,randomUUID} = require("node:crypto");

module.exports = function signers(call) {
  const external = Symbol();
  function remoteIdentity(value, pattern, callback) {
    if (typeof value !== "string" || !pattern.test(value) || typeof callback !== "function") throw new TypeError("invalid external signer");
    return value.toLowerCase();
  }
  class MetricSigner {
    #key; #callback; #address;
    constructor({privateKey,address,callback}, mode) {
      this.#address=mode === external ? remoteIdentity(address,/^0x[0-9a-f]{40}$/i,callback) : call(59,{operation:"ethereum_address",key:privateKey});
      this.#callback=mode === external ? callback : undefined;
      this.#key=privateKey;
    }
    get address() { return this.#address; }
    static remote(options) { return new this(options,external); }
    async signDigest({digest}) {
      if (!this.#callback) return call(59,{operation:"sign_digest",key:this.#key,digest});
      const signature=await this.#callback({digest,address:this.address,scheme:"evm_digest"});
      return call(59,{operation:"verify_digest",signature,digest,address:this.address});
    }
    async signPersonalDigest({digest}) {
      if (!/^0x[0-9a-fA-F]{64}$/.test(digest)) throw new TypeError("expected a 32-byte digest");
      return this.signDigest({digest:call(59,{operation:"personal_hash",message:digest})});
    }
    async signTransaction({request}) {
      const digest=call(17,{prepared:request,digest_only:true});
      const signature=await this.signDigest({digest});
      const signed=call(17,{prepared:request,signature,address:this.address});
      return {hash:signed.hash,bytes:signed.signed,signer:signed.address,nonce:request.nonce,chain_id:request.chainId};
    }
  }
  class MonacoSessionSigner {
    #seed; #callback; #public;
    constructor({seed,publicKey,callback}={}, mode) {
      if (mode === external) {
        this.#public=remoteIdentity(publicKey,/^[0-9a-f]{64}$/i,callback);
        this.#callback=callback;
      } else {
        this.#seed=seed ?? randomBytes(32).toString("hex");
        this.#public=call(59,{operation:"ed25519_public",key:this.#seed});
      }
    }
    static remote(options) { return new this(options,external); }
    get publicKey() { return this.#public; }
    exportSeed() {
      if (!this.#seed) throw new Error("remote signer has no local seed");
      return this.#seed.replace(/^0x/,"");
    }
    async sign({message}) {
      const bytes=Array.from(message);
      if (bytes.some(b=>!Number.isInteger(b) || b<0 || b>255)) throw new TypeError("expected message bytes");
      const hex=Buffer.from(bytes).toString("hex");
      if (!this.#callback) return call(59,{operation:"ed25519_sign",key:this.#seed,message:hex});
      const signature=await this.#callback(bytes);
      return call(59,{operation:"ed25519_verify",public_key:this.#public,message:hex,signature});
    }
  }
  class MonacoWalletSigner {
    #key; #callback; #address;
    constructor({privateKey,address,callback},mode) {
      this.#address=mode === external ? remoteIdentity(address,/^0x[0-9a-f]{40}$/i,callback) : call(59,{operation:"ethereum_address",key:privateKey});
      this.#callback=mode === external ? callback : undefined;
      this.#key=privateKey;
    }
    static remote(options) { return new this(options,external); }
    get address() { return this.#address; }
    async signMessage({message}) {
      const digest=call(59,{operation:"personal_hash",message:Buffer.from(message).toString("hex")});
      if (!this.#callback) return call(59,{operation:"sign_digest",digest,key:this.#key});
      const signature=await this.#callback(message);
      return call(59,{operation:"verify_digest",address:this.address,digest,signature});
    }
  }
  return {MetricSigner,MonacoSessionSigner,MonacoWalletSigner};
};

module.exports.restoreSession = function restoreSession(client,call,{signer,info,requestSigning="legacy"}) {
  const origin=client.origin ?? "https://staging.apimonaco.xyz";
  const options={address:info.address,client_id:info.clientId,seed:signer.exportSeed(),chain_id:client.chainId,signing:requestSigning,ws_origin:client.wsUrl};
  const identity=call(42,{step:"challenge",options,origin});
  if (identity.sessionPublicKey !== info.sessionPublicKey || BigInt(identity.chainId)!==BigInt(info.chainId)) throw new Error("Monaco session context or key mismatch");
  client.session={...call(42,{step:"finish",options,origin,response:info}),info:{...identity,chainId:BigInt(identity.chainId)},instance:randomUUID()};
  client.origin=origin;
};

module.exports.publicPermit = function publicPermit(permit) {
  if (!permit) return permit;
  const {sign, ...identity} = permit;
  return identity;
};

module.exports.signPermitBody = async function signPermitBody(call,operation,body,identity,sign) {
  const proof = call(32,{step:"prepare_external",operation,body,auth:identity});
  const rows = Array.isArray(body) ? body : [body];
  const signed = [];
  for (const [index,row] of rows.entries()) {
    require("./lifecycle.cjs").ensureOpen();
    const signature = await sign(Buffer.from(proof.messages[index],"hex"));
    signed.push(call(32,{operation,body:row,timestamp:proof.timestamp,auth:{...identity,signature}}));
  }
  return Array.isArray(body) ? signed : signed[0];
};
