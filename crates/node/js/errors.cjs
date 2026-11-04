"use strict";
class ConnectorError extends Error {constructor(message){super(message);this.name=this.constructor.name;}}
class TransportError extends ConnectorError {}
class GapError extends TransportError {}
class ExchangeError extends ConnectorError {}
class DecodeError extends ConnectorError {}
class ValidationError extends ConnectorError {}
class ClosedError extends ConnectorError {}
class BusyError extends ConnectorError {}
class AuthenticationError extends ConnectorError {}
class SigningError extends ConnectorError {}
class NonceUncertainError extends ConnectorError {}
class OutcomeUncertainError extends ConnectorError {}
const errors={ConnectorError,TransportError,GapError,ExchangeError,DecodeError,ValidationError,ClosedError,BusyError,AuthenticationError,SigningError,NonceUncertainError,OutcomeUncertainError};
module.exports={...errors};
Object.defineProperty(module.exports,"errors",{value:errors});
