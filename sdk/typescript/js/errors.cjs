"use strict";

class ConnectorError extends Error {
  constructor(message, options) {
    super(message, options);
    this.name = this.constructor.name;
  }
}
class AuthenticationError extends ConnectorError {}
class TransportError extends ConnectorError {}
class StreamGapError extends TransportError {}
class ExchangeError extends ConnectorError {}
class DecodeError extends ConnectorError {}
class ClosedError extends ConnectorError {}
class ValidationError extends ConnectorError {}
class OutcomeUncertainError extends ConnectorError {}
const errors = {ClosedError,ConnectorError,AuthenticationError,TransportError,StreamGapError,ExchangeError,DecodeError,ValidationError,OutcomeUncertainError};

function parseHttp(text) {
  return JSON.parse(text, (_, value, context) => {
    if (typeof value !== "number") return value;
    const source = context.source;
    if (/[.eE]/.test(source)) return source;
    return Number.isSafeInteger(value) ? value : BigInt(source);
  });
}
function rethrow(error) {
  const match = /^\[aux:(\w+)\] ([\s\S]*)$/.exec(error.message);
  if (match && match[1] === "SessionResponse") {
    const {venue,body} = parseHttp(match[2]);
    throw Object.assign(new ExchangeError(venue + " returned a session error"), {venue,body});
  }
  if (match && match[1] === "ExchangeResponse") {
    const {venue,status,body} = parseHttp(match[2]);
    const exchange = new ExchangeError(venue + " returned an exchange error (HTTP " + status + ")");
    Object.assign(exchange, {venue,status,body});
    throw exchange;
  }
  if (match && Object.hasOwn(errors, match[1])) throw new errors[match[1]](match[2], {cause:error});
  throw error;
}
function sync(fn) {
  return (...args) => {
    try { return fn(...args); } catch (error) { rethrow(error); }
  };
}
function task(fn) {
  return async (...args) => {
    try { return await fn(...args); } catch (error) { rethrow(error); }
  };
}
function stream(handle) {
  return {
    nextBytes:task(() => handle.nextBytes()),
    nextJson:task(() => handle.nextJson()),
    commandJson:task(command => handle.commandJson(command)),
    close:background => handle.close(background),
  };
}
function bind(native) {
  return {
    invoke:sync(native.invoke),
    executeHttp:task(native.executeHttp),
    executeRpc:task(native.executeRpc),
    executeSolana:task(native.executeSolana),
    executeMetricChain:task(native.executeMetricChain),
    executePhoenixChain:task(native.executePhoenixChain),
    executeEvmChain:task(native.executeEvmChain),
    openDbn:sync(input => stream(native.openDbn(input))),
    openStream:task(async input => stream(await native.openStream(input))),
    EventDecoder:class EventDecoder {
      #decoder;
      constructor(...args) { this.#decoder = sync(() => new native.EventDecoder(...args))(); }
      feed(chunk) { return sync(() => this.#decoder.feed(chunk))(); }
      finish() { return sync(() => this.#decoder.finish())(); }
      close() { this.#decoder.close(); }
    },
  };
}
module.exports = {errors,bind,parseHttp};
