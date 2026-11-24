"use strict";
const targets = { "darwin-arm64": "darwin-arm64", "darwin-x64": "darwin-x64", "linux-x64": "linux-x64-gnu" };
const target = targets[process.platform + "-" + process.arch];
if (!target) throw new Error("No aux-connectors binary for " + process.platform + "/" + process.arch);
const {errors,bind,parseHttp} = require("./errors.cjs");
const native = bind(require("../aux." + target + ".node"));

function call(op, args) {
  const input = (op === 42 || op === 32) ? stringifyHttp(args) : JSON.stringify(args, (_, value) => {
    if (typeof value === "bigint") return value.toString();
    if (typeof value === "number" && !Number.isSafeInteger(value)) throw new RangeError("Supply an exact integer as bigint or a string");
    return value;
  });
  return JSON.parse(native.invoke(op, input), (_, value, context) => {
    if (typeof value === "number" && !Number.isSafeInteger(value)) return BigInt(context.source);
    return value;
  });
}
function codec(encode, input, output) {
  return {
    encodeContractCall: (contract, fn, args) => call(encode, {contract, function: fn, args}),
    decodeContractInput: (contract, fn, data) => call(input, {contract, function: fn, data}),
    decodeContractOutput: (contract, fn, data) => call(output, {contract, function: fn, data}),
  };
}
const metric = {
  createUniformBins: params => call(44, {operation:"create_uniform_bins", params}),
  prepareFactoryCreatePoolParams: params => call(44, {operation:"prepare_factory_create_pool_params", params}),
  prepareCreatePool: params => call(44, {operation:"prepare_create_pool", params}),
  prepareLiquidityAddition: params => call(44, {operation:"prepare_liquidity_addition", params}),
  liquidityNativePayment: params => call(44, {operation:"liquidity_native_payment", params}),
  routerMulticall: params => call(44, {operation:"router_multicall", params}),
  nativePayment: params => call(44, {operation:"native_payment", params}),
  unwrapOutput: params => call(44, {operation:"unwrap_output", params}),
  prepareExternalSwap: params => call(44, {operation:"prepare_external_swap", params}),
  prepareSwapWithFallback: params => call(44, {operation:"prepare_swap_with_fallback", params}),
  additionPlan: params => call(44, {operation:"addition_plan", params}),
  weightedPlan: params => call(44, {operation:"weighted_plan", params}),
  removalPlan: params => call(44, {operation:"removal_plan", params}),

  codebook: () => call(44, {operation:"codebook", params:{}}),
  encodeSpread: params => call(44, {operation:"encode_spread", params:params}),
  encodeU64x32: params => call(44, {operation:"encode_u64x32", params:params}),
  decodeU64x32: params => call(44, {operation:"decode_u64x32", params:params}),
  packOracleSlot: params => call(44, {operation:"pack_oracle_slot", params:params}),
  oraclePushCalldata: params => call(44, {operation:"oracle_push_calldata", params:params}),
  feedId: params => call(44, {operation:"feed_id", params:params}),
  slotUpdateDigest: params => call(44, {operation:"slot_update_digest", params:params}),
  pusherConsentDigest: params => call(44, {operation:"pusher_consent_digest", params:params}),
  signPersonalDigest: params => call(44, {operation:"sign_personal_digest", params:params}),
  mulDiv: params => call(44, {operation:"mul_div", params:params}),
  priceToQ64: params => call(44, {operation:"price_to_q64", params:params}),
  priceX64ToE18: params => call(44, {operation:"price_x64_to_e18", params:params}),
  slippageLimit: params => call(44, {operation:"slippage_limit", params:params}),
  percentageRemoval: params => call(44, {operation:"percentage_removal", params:params}),
  decayedHighWatermark: params => call(44, {operation:"decayed_high_watermark", params:params}),
  stopLossThreshold: params => call(44, {operation:"stop_loss_threshold", params:params}),
  stopLossBreached: params => call(44, {operation:"stop_loss_breached", params:params}),
  secondsUntilStopLossClears: params => call(44, {operation:"seconds_until_stop_loss_clears", params:params}),
  stopLossMetrics: params => call(44, {operation:"stop_loss_metrics", params:params}),
  evaluateOracleStopLoss: params => call(44, {operation:"evaluate_oracle_stop_loss", params:params}),
  extensionOrder: params => call(44, {operation:"extension_order", params:params}),
  directionBitmap: params => call(44, {operation:"direction_bitmap", params:params}),
  minimumOperationalLiquidity: params => call(44, {operation:"minimum_operational_liquidity", params:params}),

  ...codec(2, 3, 4),
  decodeContractError: (contract, data) => call(5, {contract, data}),
  decodeContractEvent: (contract, event, topics, data) => call(6, {contract, event, topics, data}),
};
const polymarket = {
  decodeContractError: (contract,data) => call(55,{contract,data}),
  decodeContractEvent: (contract,event,topics,data) => call(56,{contract,event,topics,data}),
  ...codec(7, 8, 9),
  marketAssetId: (market, outcome) => call(46, {operation:"market_asset_id", params:{market, outcome}}),
  marketOrderDomain: (market) => call(46, {operation:"market_order_domain", params:{market}}),
  deployments: () => call(46, {operation:"deployments", params:{}}),
  typedDataHash: payload => parseHttp(native.invoke(46,stringifyHttp({operation:"typed_data_hash", params:{payload}}))),
  orderTypedData: (order,domain) => parseHttp(native.invoke(46,stringifyHttp({operation:"order_typed_data", params:{order,domain}}))),
  walletAuthTypedData: (chainId, address, timestamp, nonce = 0) => call(46, {operation:"wallet_auth_typed_data", params:{chain_id:chainId, address, timestamp, nonce}}),
  perpsOpTypedData: (chainId, operation, salt, timestamp) => parseHttp(native.invoke(46, stringifyHttp({operation:"perps_op_typed_data", params:{chain_id:chainId, operation, salt, timestamp}}))),
  depositWalletBatchTypedData: (chainId, wallet, nonce, deadline, calls) => call(46, {operation:"deposit_wallet_batch_typed_data", params:{chain_id:chainId, wallet, nonce, deadline, calls}}),
  wrapSessionSignature: (sessionSigner, signature) => call(46, {operation:"wrap_session_signature", params:{session_signer:sessionSigner, signature}}),
};
const decodeReport = fullReport => JSON.parse(native.invoke(1, fullReport));
module.exports = {metric, polymarket, decodeReport, ...errors};

// Keep decimal response tokens exact even when IEEE-754 cannot represent them.

function httpInput(venue, operation, params, origin, apiKey, apiSecret, chainId, clob, auth, services) {
  return stringifyHttp({venue, operation, params, origin, api_key: apiKey, api_secret: apiSecret, chain_id:chainId, clob, auth, services});
}
function stringifyHttp(input) {
  return JSON.stringify(input, (_, v) => {
    if (typeof v === "bigint") return JSON.rawJSON(v.toString());
    if (typeof v === "number" && (!Number.isFinite(v) || (Number.isInteger(v) && !Number.isSafeInteger(v)))) {
      throw new RangeError("Supply an exact integer as bigint");
    }
    return v;
  });
}
module.exports.prepareRequest = (venue, operation, params) => parseHttp(native.invoke(10, httpInput(venue, operation, params)));
module.exports.decodeResponse = (venue, status, body) => parseHttp(native.invoke(11, JSON.stringify({venue, status, body})));
module.exports.execute = async (venue, operation, params, origin, apiKey, apiSecret, chainId, clob, auth, services) => {
  if (typeof auth?.sign === "function") {
    const {sign, ...identity} = auth;
    auth = identity;
    if (venue === "pascal" && operation.startsWith("write.")) {
      const body = await require("./signers.cjs").signPermitBody(call,operation,params.body,auth,sign);
      params = {...params,body};
    }
    const arcus = venue === "arcus" && ["place_order","cancel_order"].includes(operation);
    if (venue === "kalshi" && !operation.startsWith("clearing.")) {
      const proof = call(53,{venue,operation,params,api_key:apiKey,origin});
      const signature=await sign(Buffer.from(proof.payload,"utf8"));
      if (!(signature instanceof Uint8Array)) throw new TypeError("Kalshi signer must return signature bytes");
      Object.assign(auth,proof,{signature:Buffer.from(signature).toString("hex")});
    }
    if (arcus || (venue === "hyperliquid" && ["order","cancel","cancel_by_cloid"].includes(operation))) {
      const proof = call(arcus ? 43 : 45, {operation,params,auth});
      auth.signature = await sign(arcus ? proof.payload : proof.digest);
      if (arcus) auth.timestamp = proof.timestamp;
      else params = {...params,options:{...params.options,nonce:proof.nonce}};
    }
  }
  require("./lifecycle.cjs").ensureOpen();
  return parseHttp(await native.executeHttp(httpInput(venue, operation, params, origin, apiKey, apiSecret, chainId, clob, auth, services)));
};
module.exports.kalshiHeaders = (key,seed,method,path,timestamp) => call(22,{key,seed,method,path,timestamp});
module.exports.qfexSignature = (secret, nonce, timestamp) => call(13, {secret, nonce, timestamp});
module.exports.databentoCram = (key, challenge) => call(14, {key, challenge});
module.exports.arcusSign = (seed, payload) => call(15, {seed, payload});
module.exports.monacoSign = (seed, method, target, body, created) => call(16, {seed, method, target, body, created});
Object.assign(module.exports, require("./signers.cjs")(call));
module.exports.signEvmDigest = (key,digest) => call(59,{operation:"sign_digest",key,digest});
module.exports.signTransaction = (key, prepared) => call(17, {key, prepared});
async function openStream(venue, operation, params, origin, apiKey, apiSecret, auth, clob, endpoints) {
  const sign = auth?.sign;
  const identity = require("./signers.cjs").publicPermit(auth);
  const stream = await native.openStream(stringifyHttp({venue,operation,params,origin,api_key:apiKey,api_secret:apiSecret,clob,auth:identity,endpoints}));
  const tradeCommand = async (operation, params) => {
    if (venue === "pascal" && typeof sign === "function" && ["place_orders","cancel_orders","mass_cancel"].includes(operation)) {
      params = await require("./signers.cjs").signPermitBody(call,operation,params,identity,sign);
    }
    return parseHttp(await stream.commandJson(stringifyHttp({operation,params})));
  };
  const bytes = operation === "dataset" || operation === "timeseries" || operation === "download";
  return {
    ...(operation === "open_rpc" ? {
      pending: () => tradeCommand("pending", {}),
      reconcile: () => tradeCommand("reconcile", {}),
      async nextEvent() { return (await this.next()).value; },
      ...(Object.hasOwn(require("./rpc.cjs").commands, venue) ? require("./rpc.cjs").commands[venue](tradeCommand) : venue === "paradex" ? {
        subscribeChannel: (specification) => tradeCommand("subscribe_channel", specification),
        unsubscribeChannel: (specification) => tradeCommand("unsubscribe_channel", specification),
        modifySubscription: (request) => tradeCommand("modify_subscription", request),
        createOrder: (request) => tradeCommand("create_order", request),
        createBatch: (requests) => tradeCommand("create_batch", requests),
        modifyOrder: (id, request) => tradeCommand("modify_order", {"id": id, "body": request}),
        cancelOrder: (id) => tradeCommand("cancel_order", {"id": id}),
        cancelByClientId: (client_id, market) => tradeCommand("cancel_by_client_id", {"client_id": client_id, "market": market}),
        cancelAll: (market) => tradeCommand("cancel_all", market == null ? {} : {market}),
        cancelBatch: (order_ids) => tradeCommand("cancel_batch", {"order_ids": order_ids}),
        setCancelOnDisconnect: (enabled) => tradeCommand("set_cancel_on_disconnect", {"enabled": enabled}),
      } : venue === "pascal" ? {
        placeOrders: p => tradeCommand("place_orders", p),
        cancelOrders: p => tradeCommand("cancel_orders", p),
        massCancel: p => tradeCommand("mass_cancel", p),
        ping: () => tradeCommand("ping", {}),
      } : {
        subscribe: p => tradeCommand("subscribe", p),
        unsubscribe: p => tradeCommand("unsubscribe", p),
        listSubscriptions: p => tradeCommand("list_subscriptions", p),
        updateSubscription: p => tradeCommand("update_subscription", p),
      }),
    } : {}),
    ...(operation === "trading_session" ? {
      subscribeAccount:() => tradeCommand("subscribe_account",{}).then(() => undefined),
      addOrder:order => tradeCommand("add_order",order),
      cancelOrder:order => tradeCommand("cancel_order",order),
      userOrders:params => tradeCommand("user_orders",params),
    } : {}),
    ...(venue === "hyperdash" && operation === "open_stream" ? {
      subscribe: subscription => stream.commandJson(stringifyHttp({operation:"subscribe",subscription})).then(() => undefined),
      ack: (sub,batch) => stream.commandJson(stringifyHttp({operation:"ack",sub,batch})).then(() => undefined),
      unsubscribe: sub => stream.commandJson(stringifyHttp({operation:"unsubscribe",sub})).then(() => undefined),
    } : {}),
    ...(["ws_replay","ws_replay_normalized","stream_normalized"].includes(operation) ? { async send(message) { await stream.commandJson(stringifyHttp({operation:"send",message})); } } : {}),
    ...(["dataset","download"].includes(operation) ? { async save(path) { return parseHttp(await stream.commandJson(stringifyHttp({operation:"save",path}))); } } : {}),
    ...(["databento","tardis"].includes(venue) && !bytes ? { async readBatch(limit) { return parseHttp(await stream.commandJson(stringifyHttp({operation:"read_batch",limit}))); } } : {}),
    [Symbol.asyncIterator]() { return this; },
    async next() {
      if (bytes) {
        const chunk = await stream.nextBytes();
        return chunk == null ? {done:true,value:undefined} : {done:false,value:chunk};
      }
      const event = await stream.nextJson();
      return event == null ? {done:true, value:undefined} : {done:false, value:parseHttp(event)};
    },
    async return() { stream.close(); return {done:true, value:undefined}; },
    close() { stream.close(); },
  };
}


module.exports.EventDecoder = native.EventDecoder;
module.exports.decodeDbnRecord = (record, tsOut = false) => parseHttp(native.invoke(18, JSON.stringify({data:Buffer.from(record).toString("hex"),ts_out:tsOut})));
module.exports.decodeSbe = frame => parseHttp(native.invoke(19, JSON.stringify({data:Buffer.from(frame).toString("hex")})));

module.exports.deriveLogin = (key, owner, timestamp) => call(23, {key, owner, timestamp});
module.exports.deriveSignAction = (key, action) => call(24, {...action, key});

module.exports.EvmReader = class EvmReader {
  constructor(endpoint) { this.endpoint = endpoint; }
  async #read(operation, params) {
    const input = stringifyHttp({endpoint: this.endpoint, operation, params});
    return parseHttp(await native.executeRpc(input));
  }
  async chainId() { return BigInt(await this.#read("chain_id", {})); }
  async code(address, block) { return await this.#read("code", {address, block}); }
  async ethCall(to, data, block, sender = null, value = "0x0") { return await this.#read("eth_call", {to, data, block, from: sender, value}); }
  async logs(filter) { return await this.#read("logs", {filter}); }
  async transactionCount(address) { return BigInt(await this.#read("transaction_count", {address})); }
  async receipt(hash) { return await this.#read("receipt", {hash}); }
  async transaction(hash) { return await this.#read("transaction", {hash}); }
  async estimateGas(request) { return BigInt(await this.#read("estimate_gas", {request})); }
  async blockNumber() { return BigInt(await this.#read("block_number", {})); }
  async block(block) { return await this.#read("block", {block}); }
  async nativeBalance(address, block) { return BigInt(await this.#read("native_balance", {address, block})); }
  async gasPrice() { return BigInt(await this.#read("gas_price", {})); }
};

module.exports.solana = {
  associatedTokenAddress: (owner, mint) => call(29, {owner, mint}),
  prepareDeposit: params => call(30, params),
  signDeposit: (seed, prepared) => call(31, {seed, prepared}),
};

const phoenix = {
  buildInstruction: (name, request) => parseHttp(native.invoke(47, stringifyHttp({operation:"build_instruction", params:{name, request}}))),
  prepareTransaction: request => call(47,{operation:"prepare_transaction",params:request}),
  signTransaction: (key,prepared,signatures={}) => call(47,{operation:"sign_transaction",params:{key,prepared,signatures}}),
  assembleTransaction: (prepared,signatures) => call(47,{operation:"assemble_transaction",params:{prepared,signatures}}),
  validateServerTransaction: (prepared,transactionBase64) => {call(47,{operation:"validate_server_transaction",params:{prepared,transaction_base64:transactionBase64}})},
  validateSignedTransaction: signed => {call(47,{operation:"validate_signed_transaction",params:signed})},
};
module.exports.phoenix = phoenix;
Object.assign(module.exports, require("./clients.cjs")(module.exports.execute, openStream, call, phoenix));

module.exports.PhoenixChainClient = class PhoenixChainClient {
  #config;
  constructor(endpoint) {
    if (typeof endpoint === "object") endpoint=endpoint.rpcUrl;
    this.#config = {endpoint,client:call(48,{endpoint,operation:"new"})};
  }
  async #rpc(operation,params={}) { return parseHttp(await native.executePhoenixChain(stringifyHttp({...this.#config,operation,params}))); }
  latestBlockhash() { return this.#rpc("latest_blockhash"); }
  async blockHeight() { return BigInt(await this.#rpc("block_height")); }
  prepare(instructions,feePayer) { return this.#rpc("prepare",{instructions,fee_payer:feePayer}); }
  simulate(signed) { return this.#rpc("simulate",signed); }
  submit(signed) { return this.#rpc("submit",signed); }
  pending() { return call(48,{...this.#config,operation:"pending",params:{}}); }
  clearReconciledSubmission() { call(48,{...this.#config,operation:"clear_reconciled_submission",params:{}}); }
  signatureStatus(signature) { return this.#rpc("signature_status",{signature}); }
  transaction(signature) { return this.#rpc("transaction",{signature}); }
};
module.exports.PascalChainClient = class PascalChainClient {
  #config;
  constructor(endpoint, mint, decimals) {
    this.#config = {endpoint, mint, decimals};
    this.#config.client = call(37,{...this.#config,operation:"new"});
  }
  async #rpc(operation, params = {}) {
    return parseHttp(await native.executeSolana(stringifyHttp({...this.#config,operation,params})));
  }
  accountInfo(address) { return this.#rpc("account_info",{address}); }
  tokenBalance(address) { return this.#rpc("token_balance",{address}); }
  async blockHeight() { return BigInt(await this.#rpc("block_height")); }
  signatureStatus(signature) { return this.#rpc("signature_status",{signature}); }
  prepareDeposit(owner,source,destination,amount) { return this.#rpc("prepare_deposit",{owner,source,destination,amount}); }
  submit(signed) { return this.#rpc("submit",signed); }
  pending() { return call(37,{...this.#config,operation:"pending",params:{}}); }
  reconcile(signature) { return this.#rpc("reconcile",{signature}); }
  clearReconciledSubmission(signature) { return call(37,{...this.#config,operation:"clear_reconciled_submission",params:{signature}}); }
  async reconcileDepositCredit(api,signature,request) {
    const confirmation = await this.reconcile(signature);
    return {signature,confirmation,transfers:await api.transferHistory(request)};
  }
};

module.exports.decodeDbnFile = async function* decodeDbnFile(data) {
  const stream = native.openDbn(data);
  try {
    for (;;) {
      const record = await stream.nextJson();
      if (record == null) return;
      yield parseHttp(record);
    }
  } finally { stream.close(); }
};

module.exports.EvmChainClient = class EvmChainClient {
  #config;
  constructor(chainId, readRpc, writeRpc, codeHashes = {}) {
    this.#config = {chain_id:chainId, read_rpc:readRpc, write_rpc:writeRpc, code_hashes:{...codeHashes}};
    this.reader = new module.exports.EvmReader(readRpc);
  }
  async #rpc(operation,params = {}) {
    require("./lifecycle.cjs").ensureOpen();
    return parseHttp(await native.executeEvmChain(stringifyHttp({...this.#config,operation,params})));
  }
  validateChain() { return this.#rpc("validate_chain"); }
  sendTransaction(prepared,key) { return this.#rpc("send_transaction",{prepared,key}); }
  async sendTransactionWithSigner(prepared,address,sign) {
    const params = {prepared,address};
    const digest = await this.#rpc("prepare_external",params);
    require("./lifecycle.cjs").ensureOpen();
    const signature = await sign(digest);
    return this.#rpc("send_external",{...params,signature});
  }
  submitRaw(input) { return this.#rpc("submit_raw",{signed:typeof input === "string" ? input : input.data}); }
  pending(address) { if (typeof address === "object") address=address.address; return call(38,{...this.#config,operation:"pending",params:{address}}); }
  reconcile(address) { return this.#rpc("reconcile",{address:typeof address === "string" ? address : address.address}); }
  async resyncNonce(address,expectedPendingHash) {
    if (typeof address === "object") { expectedPendingHash=address.expectedHash; address=address.address; }
    return BigInt(await this.#rpc("resync_nonce",{address, hash:expectedPendingHash}));
  }
};

class LiveSession {
  #stream;
  constructor(stream, id) { this.#stream = stream; this.sessionId = id; }
  async #command(operation, params = {}) {
    return parseHttp(await this.#stream.commandJson(stringifyHttp({operation, ...params})));
  }
  subscribe(subscription) { return this.#command("subscribe", {subscription}); }
  unsubscribe(subscription) { return this.#command("unsubscribe", {subscription}); }
  startSession() { return this.#command("start_session"); }
  readBatch(limit) { return this.#command("read_batch", {limit}); }
  metadata() { return this.#command("metadata"); }
  [Symbol.asyncIterator]() { return this; }
  async next() {
    const record = await this.#stream.nextJson();
    return record == null ? {done:true,value:undefined} : {done:false,value:parseHttp(record)};
  }
  async return() { this.close(); return {done:true,value:undefined}; }
  close() { this.#stream.close(); }
}
module.exports.LiveClient = class LiveClient {
  #key;
  #timeouts;
  constructor(apiKey, {timeoutMs=10000,idleTimeoutMs=60000}={}) {
    for (const ms of [timeoutMs,idleTimeoutMs]) {
      if (!Number.isSafeInteger(ms) || ms <= 0) throw new RangeError("Databento timeouts require positive milliseconds");
    }
    this.#key = apiKey;
    this.#timeouts = {timeout_ms:timeoutMs, idle_timeout_ms:idleTimeoutMs};
  }
  async connect(params) {
    const config = stringifyHttp({venue:"databento", operation:"connect", api_key:this.#key, params, ...this.#timeouts});
    const stream = await native.openStream(config);
    return new LiveSession(stream, parseHttp(await stream.commandJson('{"operation":"session_id"}')));
  }
};

module.exports.extendedSettlement = (kind,chainId,publicKey,message,starkKey) => call(39,{kind,chain_id:chainId,public_key:publicKey,message,...(starkKey == null ? {} : {stark_key:starkKey})});
module.exports.ExtendedSigner = class ExtendedSigner {
  #signature; #key; #kind;
  constructor(signature) {
    if (signature && typeof signature === "object") {
      const {privateKey,kind="starknet"}=signature;
      if (!["starknet","evm"].includes(kind)) throw new TypeError("invalid Extended signer kind");
      this.#kind=kind;
      this.#key=privateKey;
      this.publicKey=call(59,{operation:kind==="starknet"?"stark_public":"ethereum_address",key:privateKey});
      return;
    }
    this.#kind="starknet";
    this.publicKey = call(49,{operation:"derived_public_key",params:{signature}});
    this.#signature = signature;
  }
  static fromWalletSignature(input) { return new this(typeof input === "string" ? input : input.signature); }
  signDigest(digest) {
    if (this.#kind !== "starknet") throw new TypeError("a Stark signer is required");
    return this.#key ? call(59,{operation:"stark_sign",key:this.#key,digest}) : call(49,{operation:"derived_sign",params:{signature:this.#signature,digest}});
  }
  async signHash({hash}) {
    const [r,s]=this.signDigest(hash);
    return {r:"0x"+BigInt(r).toString(16),s:"0x"+BigInt(s).toString(16)};
  }
  async signSettlement(chain,body,context) {
    if (this.#kind !== "starknet") throw new TypeError("a Stark signer is required");
    return call(40,{chain_id:chain,body,context,...(this.#key ? {stark_key:this.#key} : {wallet_signature:this.#signature})});
  }
};
module.exports.extendedPrepareOrder = (chainId,publicKey,body,context) => call(40,{chain_id:chainId,public_key:publicKey,body,context});
module.exports.extendedSignOrder = (chainId,starkKey,body,context) => call(40,{chain_id:chainId,stark_key:starkKey,body,context});

class ContractChainClient extends module.exports.EvmChainClient {
  constructor(chainId,readRpc,writeRpc,codeHashes,codec,prepareOp) {
    super(chainId,readRpc,writeRpc,codeHashes);
    this.codec=codec;
    this.prepareOp=prepareOp;
  }
  encodeCall({contract,function:fn,args=[]}) { return this.codec.encodeContractCall(contract,fn,args); }
  prepareCall({contract,function:fn,to,args=[],value="0x0"}) { return call(this.prepareOp,{contract,function:fn,to,args,value}); }
  decodeInput({contract,function:fn,data}) { return this.codec.decodeContractInput(contract,fn,data); }
  decodeOutput({contract,function:fn,data}) { return this.codec.decodeContractOutput(contract,fn,data); }
  decodeError({contract,data}) { return this.codec.decodeContractError(contract,data); }
  decodeEvent({contract,event,topics,data}) { return this.codec.decodeContractEvent(contract,event,topics,data); }
  async call(options) {
    const {to,data,value} = this.prepareCall(options);
    const result = await this.reader.ethCall(to,data,options.block ?? "latest",options.fromAddress,value);
    return this.decodeOutput({...options,data:result});
  }
}
module.exports.PolymarketChainClient = class PolymarketChainClient extends ContractChainClient {
  constructor(chainId,readRpc,writeRpc=readRpc,codeHashes={}) {
    super(chainId,readRpc,writeRpc,codeHashes,polymarket,54);
  }
};
module.exports.MetricChainClient = class MetricChainClient extends ContractChainClient {
  #endpoint; #signer;
  constructor(chainId,readRpc,writeRpc=readRpc,codeHashes={}) {
    const options=typeof chainId === "object" ? chainId : {chainId,readRpc,writeRpc,codeHashes};
    super(options.chainId,options.readRpc,options.writeRpc ?? options.readRpc,options.codeHashes ?? {},metric,52);
    this.#endpoint=options.readRpc;
    this.#signer=options.signer;
  }
  blockNumber() { return this.reader.blockNumber(); }
  block({number="latest"}={}) { return this.reader.block(number); }
  async nativeBalance({address,block="latest"}) { return "0x"+(await this.reader.nativeBalance(address,block)).toString(16); }
  code({address,block="latest"}) { return this.reader.code(address,block); }
  async gasPrice() { return "0x"+(await this.reader.gasPrice()).toString(16); }
  estimateGas({request}) { return this.reader.estimateGas(request); }
  receipt({hash}) { return this.reader.receipt(hash); }
  transaction({hash}) { return this.reader.transaction(hash); }
  transactionCount({address}) { return this.reader.transactionCount(address); }
  logs({filter}) { return this.reader.logs(filter); }
  sendTransaction(options,key) {
    if (key !== undefined) return super.sendTransaction(options,key);
    if (!this.#signer) throw new Error("MetricSigner required");
    return this.sendTransactionWithSigner(options.request,this.#signer.address,digest=>this.#signer.signDigest({digest}));
  }
  async #read(operation,params) {
    return parseHttp(await native.executeMetricChain(stringifyHttp({endpoint:this.#endpoint,operation,params})));
  }
  contract(name,address) {
    if (typeof name === "object") {address=name.address;name=name.name;}
    if (!Object.hasOwn(module.exports.contracts,name)) throw new RangeError("Unknown Metric contract " + name);
    return new module.exports.contracts[name](this,address);
  }
  liquiditySnapshot(pool,lens,bins,block="latest") { if(typeof pool === "object") ({pool,lens,bins,block="latest"}=pool); return this.#read("liquidity_snapshot",{pool,lens,bins,block}); }
  oracleStopLossStatus(request) { return this.#read("oracle_stop_loss_status",request.request ?? request); }
  prepareRoute(request) { return this.#read("prepare_route",request.request ?? request); }
};

const {managed} = require("./lifecycle.cjs");
managed(module.exports.PhoenixChainClient,["pending","clearReconciledSubmission"]);
managed(module.exports.EvmChainClient,["pending"]);
managed(ContractChainClient,["encodeCall","prepareCall","decodeInput","decodeOutput","decodeError","decodeEvent"]);
managed(module.exports.MetricChainClient,["contract"]);

module.exports.contracts = require("./contracts.cjs");

module.exports.DatabentoHistoricalClient = module.exports.HistoricalClient;
module.exports.DatabentoReferenceClient = module.exports.ReferenceClient;
module.exports.DatabentoLiveClient = module.exports.LiveClient;
