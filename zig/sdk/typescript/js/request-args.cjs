"use strict";

function unwrap(options, key="request") {
  if (!options || !Object.hasOwn(options,key)) return options;
  if (Object.keys(options).length !== 1) throw new TypeError("supply the request once");
  return options[key] ?? {};
}

function assign(input, fields) {
  const output={...input};
  for (const [key,value] of Object.entries(fields)) {
    if (value === undefined || value === null) continue;
    if (Object.hasOwn(output,key)) throw new TypeError("duplicate native parameter " + key);
    output[key]=value;
  }
  return output;
}

function marketOrder(options) {
  if (!Object.hasOwn(options,"request")) return options;
  const {request,orderId,market,markets,...extra}=options;
  if (Object.keys(extra).length) throw new TypeError("unexpected order parameter");
  return assign({body:request},{orderId,market,markets});
}

function hyperliquidAction(options) {
  if (!Object.hasOwn(options,"request")) return options;
  const {request,nonce,expiresAfter,...extra}=options;
  if (Object.keys(extra).length) throw new TypeError("unexpected action parameter");
  if (nonce == null && expiresAfter == null) return request;
  return {...request,options:assign(request.options,{nonce,expires_after:expiresAfter})};
}

function lighter(options) {
  const {accountIndex,apiKeyIndex,marketId,l1Address,...rest}=options;
  if (l1Address != null && accountIndex != null) throw new TypeError("supply a single account selector");
  return assign(rest,{account_index:accountIndex,api_key_index:apiKeyIndex,market_id:marketId,l1_address:l1Address});
}
function lighterAccount(options) {
  const input=lighter(options);
  if (input.account_index != null) return assign(Object.fromEntries(Object.entries(input).filter(([k])=>k!=="account_index")),{by:"index",value:input.account_index});
  if (input.l1_address != null) return assign(Object.fromEntries(Object.entries(input).filter(([k])=>k!=="l1_address")),{by:"l1_address",value:input.l1_address});
  return input;
}
function lighterOrder(first,options) {
  if (Object.hasOwn(first,"request")) {
    if (options !== undefined) throw new TypeError("supply order options once");
    const {request,expiredAt,attributes}=first;
    return {order:request,options:assign({expired_at:expiredAt ?? Date.now()+600000},{attributes})};
  }
  return {order:first,options:options ?? {expired_at:Date.now()+600000}};
}
function metric(options, chainId) {
  const request=unwrap(options);
  if (chainId == null) return request;
  if (request.chainId != null && BigInt(request.chainId)!==BigInt(chainId)) throw new RangeError("request chain differs from client chain");
  return {...request,chainId};
}

module.exports={unwrap,marketOrder,hyperliquidAction,lighter,lighterAccount,lighterOrder,metric};
