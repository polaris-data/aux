export type AbiValue = boolean | string | number | bigint | readonly AbiValue[] | { readonly [key: string]: AbiValue };
export interface ContractCodec {
  encodeContractCall(contract: string, fn: string, args: readonly AbiValue[] | Record<string, AbiValue>): string;
  decodeContractInput(contract: string, fn: string, data: string): AbiValue[];
  decodeContractOutput(contract: string, fn: string, data: string): AbiValue[];
}
export const polymarket: ContractCodec & {
  marketAssetId(market: Record<string,HttpValue>, outcome: string): string;
  marketOrderDomain(market: Record<string,HttpValue>): {chain_id:number;version:number;exchange:string};
  deployments(): Record<string,HttpValue>;
  typedDataHash(payload:HttpValue):string;
  orderTypedData(order:HttpValue,domain:HttpValue):Record<string,HttpValue>;
  walletAuthTypedData(chainId: number|bigint|string, address: string, timestamp: number|bigint|string, nonce?: number|bigint|string): Record<string,HttpValue>;
  perpsOpTypedData(chainId: number|bigint|string, operation: HttpValue, salt: number|bigint|string, timestamp: number|bigint|string): Record<string,HttpValue>;
  depositWalletBatchTypedData(chainId: number|bigint|string, wallet: string, nonce: number|bigint|string, deadline: number|bigint|string, calls: {to:string;value:string|bigint|number;data:string}[]): Record<string,HttpValue>;
  wrapSessionSignature(sessionSigner: string, signature: string): string;
};
export const metric: ContractCodec & {
  createUniformBins(params:Record<string,HttpValue>): string[];
  prepareFactoryCreatePoolParams(params:Record<string,HttpValue>): Record<string,HttpValue>;
  prepareCreatePool(params:Record<string,HttpValue>): Record<string,HttpValue>;
  prepareLiquidityAddition(params:Record<string,HttpValue>): {to:string;data:string;value:string};
  liquidityNativePayment(params:Record<string,HttpValue>): {to:string;data:string;value:string};
  routerMulticall(params:Record<string,HttpValue>): {to:string;data:string;value:string};
  nativePayment(params:Record<string,HttpValue>): {to:string;data:string;value:string};
  unwrapOutput(params:Record<string,HttpValue>): {to:string;data:string;value:string};
  prepareExternalSwap(params:Record<string,HttpValue>): {to:string;data:string;value:string};
  prepareSwapWithFallback(params:Record<string,HttpValue>): {to:string;data:string;value:string};
  additionPlan(params:Record<string,HttpValue>): Record<string,HttpValue>;
  weightedPlan(params:Record<string,HttpValue>): Record<string,HttpValue>;
  removalPlan(params:Record<string,HttpValue>): Record<string,HttpValue>;

  codebook(): number[];
  encodeSpread(params: Record<string,HttpValue>): number;
  encodeU64x32(params: Record<string,HttpValue>): number;
  decodeU64x32(params: Record<string,HttpValue>): number|bigint;
  packOracleSlot(params: Record<string,HttpValue>): number|bigint;
  oraclePushCalldata(params: Record<string,HttpValue>): string;
  feedId(params: Record<string,HttpValue>): string;
  slotUpdateDigest(params: Record<string,HttpValue>): string;
  pusherConsentDigest(params: Record<string,HttpValue>): string;
  signPersonalDigest(params: Record<string,HttpValue>): string;
  mulDiv(params: Record<string,HttpValue>): number|bigint;
  priceToQ64(params: Record<string,HttpValue>): number|bigint;
  priceX64ToE18(params: Record<string,HttpValue>): number|bigint;
  slippageLimit(params: Record<string,HttpValue>): number|bigint;
  percentageRemoval(params: Record<string,HttpValue>): number|bigint;
  decayedHighWatermark(params: Record<string,HttpValue>): number|bigint;
  stopLossThreshold(params: Record<string,HttpValue>): number|bigint;
  stopLossBreached(params: Record<string,HttpValue>): boolean;
  secondsUntilStopLossClears(params: Record<string,HttpValue>): number|bigint|null;
  stopLossMetrics(params: Record<string,HttpValue>): {metricToken0:string;metricToken1:string};
  evaluateOracleStopLoss(params: Record<string,HttpValue>): Record<string,HttpValue>;
  extensionOrder(params: Record<string,HttpValue>): number|bigint;
  directionBitmap(params: Record<string,HttpValue>): number|bigint;
  minimumOperationalLiquidity(params: Record<string,HttpValue>): number|bigint;

  decodeContractError(contract: string, data: string): { name: string; arguments: AbiValue[] };
  decodeContractEvent(contract: string, event: string, topics: readonly string[], data: string): { name: string; indexed: AbiValue[]; body: AbiValue[] };
};
interface Report { version: number; full_report: string; report_body: string }
export function decodeReport(fullReport: string): (Report & {kind: "known"; fields: Record<string, string | number>}) | (Report & {kind: "unknown"});

export type HttpValue = null | boolean | string | number | bigint | HttpValue[] | { [key: string]: HttpValue };
export interface WireRequest {
  method: string;
  path: string;
  headers: Record<string, string>;
  body: HttpValue;
  authenticated: boolean;
}
export type HttpVenue = "derive" | "extended" | "paradex" | "phoenix" | "kalshi" | "pascal" | "polymarket" | "hyperdash" | "birdeye" | "dune" | "tardis" | "lighter" | "hyperliquid" | "databento" | "pyth_hermes" | "pyth_pro" | "chainlink_streams" | "chainlink_candles" | "qfex" | "arcus" | "metric" | "monaco";
export function prepareRequest(venue: HttpVenue, operation: string, params: Record<string, HttpValue>): WireRequest;
export function decodeResponse(venue: HttpVenue, status: number, body: string): HttpValue;
export function execute(venue: HttpVenue, operation: string, params: Record<string, HttpValue>, origin?: string, apiKey?: string, apiSecret?: string): Promise<HttpValue>;
export function kalshiHeaders(key: string, seed: string, method: string, path: string, timestamp: number | bigint): Record<string,string>;
export function qfexSignature(secret: string, nonce: string, timestamp: number | bigint): string;
export function databentoCram(key: string, challenge: string): string;
export function arcusSign(seed: string, payload: string): {public_key: string; signature: string};
export function monacoSign(seed: string, method: string, target: string, body: string, created: number | bigint): Record<string, string>;
export function signEvmDigest(key:string, digest:string):string;
export function signTransaction(key: string, prepared: Record<string, HttpValue>): {address: string; hash: string; signed: string};
export * from "./clients";

export class EventDecoder {
  constructor(limit?: number);
  feed(chunk: Uint8Array): string[];
  finish(): void;
  close(): void;
}
export function decodeDbnRecord(record: Uint8Array, tsOut?: boolean): Record<string, HttpValue>;
export function decodeSbe(frame: Uint8Array): {schema_id: number; schema_version: number; block_length: number; template: string; data: Record<string, HttpValue>};

export type { Action as DeriveAction, ActionData as DeriveActionData, SignedAction as DeriveSignedAction, LoginProof as DeriveLoginProof } from "./derive";
export function deriveLogin(key: string, owner: string, timestamp: number | bigint): import("./derive").LoginProof;
export function deriveSignAction(key: string, action: import("./derive").Action): import("./derive").SignedAction;

export class EvmReader {
  constructor(endpoint: string);
  chainId(): Promise<bigint>;
  code(address: string, block: string): Promise<string>;
  ethCall(to: string, data: string, block: string, sender?: string | null, value?: string): Promise<string>;
  logs(filter: Record<string, HttpValue>): Promise<HttpValue[]>;
  transactionCount(address: string): Promise<bigint>;
  receipt(hash: string): Promise<Record<string, HttpValue> | null>;
  transaction(hash: string): Promise<Record<string, HttpValue> | null>;
  estimateGas(request: Record<string, HttpValue>): Promise<bigint>;
  blockNumber(): Promise<bigint>;
  block(block: string): Promise<Record<string, HttpValue> | null>;
  nativeBalance(address: string, block: string): Promise<bigint>;
  gasPrice(): Promise<bigint>;
}

export type * as DeriveModels from "./derive_models";
export type * as ParadexModels from "./paradex_models";
export type * as PhoenixModels from "./phoenix_models";
export type * as ExtendedModels from "./extended_models";

export interface DepositParams {
  owner: string;
  source: string;
  destination: string;
  mint: string;
  amount: number | bigint;
  decimals: number;
  recent_blockhash: string;
  last_valid_block_height: number | bigint;
}
export interface PreparedDeposit extends DepositParams { message: number[] }
export interface SignedDeposit { signature: string; transaction_base64: string; last_valid_block_height: number | bigint }
export const solana: {
  associatedTokenAddress(owner: string, mint: string): string;
  prepareDeposit(params: DepositParams): PreparedDeposit;
  signDeposit(seed: string, prepared: PreparedDeposit): SignedDeposit;
};
export type * as PascalModels from "./pascal_models";
export type * as KalshiModels from "./kalshi_models";
export type * as KalshiPerpsModels from "./kalshi_perps_models";
export type * as PolymarketModels from "./polymarket_models";
export type * as PolymarketPerpsModels from "./polymarket_perps_models";

export interface ParadexRpcCommands {
  subscribeChannel(specification:HttpValue):Promise<HttpValue>;
  unsubscribeChannel(specification:HttpValue):Promise<HttpValue>;
  modifySubscription(request:HttpValue):Promise<HttpValue>;
  createOrder(request:HttpValue):Promise<HttpValue>;
  createBatch(requests:HttpValue[]):Promise<HttpValue>;
  modifyOrder(id:string,request:HttpValue):Promise<HttpValue>;
  cancelOrder(id:string):Promise<HttpValue>;
  cancelByClientId(client_id:string,market:string):Promise<HttpValue>;
  cancelAll(market?:string):Promise<HttpValue>;
  cancelBatch(order_ids:string[]):Promise<HttpValue>;
  setCancelOnDisconnect(enabled:boolean):Promise<HttpValue>;
}

export interface PhoenixInstruction {program_id:string;accounts:{pubkey:string;is_signer:boolean;is_writable:boolean}[];data:number[];}
export interface PhoenixPrepareRequest {fee_payer:string;instructions:PhoenixInstruction[];recent_blockhash:string;last_valid_block_height:number|bigint;version?:"legacy"|"v0";lookup_tables?:{key:string;addresses:string[]}[];}
export interface PhoenixPreparedTransaction {required_signers:string[];message_base64:string;recent_blockhash:string;last_valid_block_height:number|bigint;}
export interface PhoenixSignedTransaction {signature:string;transaction_base64:string;last_valid_block_height:number|bigint;}
export const phoenix: {
  buildInstruction(name:string,request:Record<string,HttpValue>):PhoenixInstruction | PhoenixInstruction[];
  prepareTransaction(request:PhoenixPrepareRequest):PhoenixPreparedTransaction;
  signTransaction(key:string,prepared:PhoenixPreparedTransaction,signatures?:Record<string,string>):Record<string,string>;
  assembleTransaction(prepared:PhoenixPreparedTransaction,signatures:Record<string,string>):PhoenixSignedTransaction;
  validateServerTransaction(prepared:PhoenixPreparedTransaction,transactionBase64:string):void;
  validateSignedTransaction(signed:PhoenixSignedTransaction):void;
};
export class MetricSigner {
  constructor(options:{privateKey:string});
  static remote(options:{address:string;callback:(proof:{digest:string;address:string;scheme:"evm_digest"})=>string|Promise<string>}):MetricSigner;
  readonly address:string;
  signDigest(options:{digest:string}):Promise<string>;
  signPersonalDigest(options:{digest:string}):Promise<string>;
  signTransaction(options:{request:Record<string,HttpValue>}):Promise<HttpValue>;
}
export class MonacoWalletSigner {
  constructor(options:{privateKey:string});
  static remote(options:{address:string;callback:(message:string)=>string|Promise<string>}):MonacoWalletSigner;
  readonly address:string;
  signMessage(options:{message:string}):Promise<string>;
}
export class MonacoSessionSigner {
  constructor(options?:{seed?:string});
  static remote(options:{publicKey:string;callback:(message:number[])=>string|Promise<string>}):MonacoSessionSigner;
  readonly publicKey:string;
  exportSeed():string;
  sign(options:{message:ArrayLike<number>}):Promise<string>;
}
export class ExtendedSigner {
  constructor(options:{privateKey:string;kind?:"starknet"|"evm"});
  signHash(options:{hash:string}):Promise<{r:string;s:string}>;
  signSettlement(chain:string,body:HttpValue,context:HttpValue):Promise<HttpValue>;
  constructor(signature:string);
  readonly publicKey:string;
  static fromWalletSignature(signature:string|{signature:string}):ExtendedSigner;
  signDigest(digest:string):[string,string];
}
export class PhoenixChainClient {
  readonly closed:boolean;
  close():void;
  constructor(options:{rpcUrl:string});
  constructor(endpoint:string);
  latestBlockhash():Promise<HttpValue>;
  blockHeight():Promise<bigint>;
  prepare(instructions:PhoenixInstruction[],feePayer:string):Promise<PhoenixPreparedTransaction>;
  simulate(signed:PhoenixSignedTransaction):Promise<HttpValue>;
  submit(signed:PhoenixSignedTransaction):Promise<string>;
  pending():PhoenixSignedTransaction|null;
  clearReconciledSubmission():void;
  signatureStatus(signature:string):Promise<HttpValue>;
  transaction(signature:string):Promise<HttpValue>;
}

export class PascalChainClient {
  constructor(endpoint: string, mint: string, decimals: number);
  accountInfo(address: string): Promise<HttpValue>;
  tokenBalance(address: string): Promise<HttpValue>;
  blockHeight(): Promise<bigint>;
  signatureStatus(signature: string): Promise<HttpValue>;
  prepareDeposit(owner: string, source: string, destination: string, amount: number | bigint): Promise<PreparedDeposit>;
  submit(signed: SignedDeposit): Promise<string>;
  pending(): string[];
  reconcile(signature: string): Promise<HttpValue>;
  clearReconciledSubmission(signature: string): void;
  reconcileDepositCredit(api: import("./clients").PascalClient, signature: string, request: import("./pascal_models").TransferHistoryParams): Promise<{signature: string; confirmation: HttpValue; transfers: import("./pascal_models").TransferHistoryResponse}>;
}

export function decodeDbnFile(data: Uint8Array): AsyncGenerator<Record<string,HttpValue>>;

export class EvmChainClient {
  readonly closed:boolean;
  close():void;
  readonly reader: EvmReader;
  constructor(chainId: number | bigint, readRpc: string, writeRpc: string, codeHashes?: Record<string,string>);
  validateChain(): Promise<void>;
  sendTransaction(prepared: Record<string,HttpValue>, key: string): Promise<string>;
  sendTransactionWithSigner(prepared: Record<string,HttpValue>, address:string, sign:(digest:string)=>string|Promise<string>): Promise<string>;
  submitRaw(signed: string | {data:string}): Promise<string>;
  pending(address: string | {address:string}): {hash: string; nonce: number | bigint} | null;
  reconcile(address: string | {address:string}): Promise<boolean>;
  resyncNonce(options:{address:string;expectedHash:string}):Promise<bigint>;
  resyncNonce(address: string, expectedPendingHash: string): Promise<bigint>;
}
export type * as HyperdashModels from "./hyperdash_models";
export type * as BirdeyeModels from "./birdeye_models";

export interface SessionOptions {
  dataset: string;
  gateway?: string;
  compression?: "none" | "zstd";
  ts_out?: boolean;
  heartbeat_interval_s?: number;
  slow_reader_behavior?: "warn" | "skip";
}
export interface Subscription {
  schema: string;
  stype_in?: string;
  symbols: string[];
  snapshot?: boolean;
  start?: string;
  id?: number;
}
export interface LiveSession extends AsyncIterableIterator<HttpValue> {
  readonly sessionId: string;
  subscribe(subscription: Subscription): Promise<void>;
  unsubscribe(subscription: Subscription): Promise<void>;
  startSession(): Promise<void>;
  metadata(): Promise<HttpValue>;
  readBatch(limit: number): Promise<HttpValue[]>;
  close(): void;
}
export class LiveClient {
  constructor(apiKey: string, options?: {timeoutMs?: number; idleTimeoutMs?: number});
  connect(options: SessionOptions): Promise<LiveSession>;
}
export type * as PythHermesModels from "./pyth_hermes_models";
export type * as PythProModels from "./pyth_pro_models";
export type * as ChainlinkStreamsModels from "./chainlink_streams_models";
export type * as ChainlinkCandlesModels from "./chainlink_candles_models";
export type * as DuneModels from "./dune_models";
export function extendedSettlement(kind:"order"|"limit_order"|"transfer"|"withdrawal",chainId:string,publicKey:string,message:HttpValue,starkKey?:string): {hash:string;signature?:{r:string;s:string}};
export interface ExtendedOrderContext {
  synthetic_asset_id:string; collateral_asset_id:string;
  synthetic_resolution:number|bigint; collateral_resolution:number|bigint;
  collateral_position:number; max_position_value?:string|null; quantity_precision?:number|null;
}
export function extendedPrepareOrder(chainId:string,publicKey:string,body:HttpValue,context:ExtendedOrderContext): {hash:string;body:Record<string,HttpValue>};
export function extendedSignOrder(chainId:string,starkKey:string,body:HttpValue,context:ExtendedOrderContext): Record<string,HttpValue>;
export type * as ArcusModels from "./arcus_models";

export class ConnectorError extends Error {}
export class ClosedError extends ConnectorError {}
export class AuthenticationError extends ConnectorError {}
export class TransportError extends ConnectorError {}
export class StreamGapError extends TransportError {}
export class ExchangeError extends ConnectorError { readonly venue?: string; readonly status?: number; readonly body?: HttpValue; }
export class DecodeError extends ConnectorError {}
export class ValidationError extends ConnectorError {}
export class OutcomeUncertainError extends ConnectorError {}

export interface QfexTradingSession extends AsyncIterableIterator<HttpValue> {
  subscribeAccount():Promise<void>;
  addOrder(order:Record<string,HttpValue>):Promise<HttpValue>;
  cancelOrder(order:Record<string,HttpValue>):Promise<HttpValue>;
  userOrders(request:Record<string,HttpValue>):Promise<HttpValue>;
  close():void;
}
export type * as DatabentoHistoricalModels from "./databento_historical_models";
export type * as DatabentoReferenceModels from "./databento_reference_models";
export type * as MetricModels from "./metric_models";
export type * as QfexModels from "./qfex_models";
export type * as TardisModels from "./tardis_models";
export type * as LighterModels from "./lighter_models";
export type * as MonacoModels from "./monaco_models";
export type * as HyperliquidModels from "./hyperliquid_models";
export type * as AsterModels from "./aster_models";
export type * as InjectiveModels from "./injective_models";
export type * as NadoModels from "./nado_models";
export type * as OndoPerpsModels from "./ondo_perps_models";
export type * as PacificaModels from "./pacifica_models";
export type * as ParagonModels from "./paragon_models";
export type * as TradeXyzModels from "./trade_xyz_models";

export * as contracts from "./contracts";
type MetricContracts = typeof import("./contracts");
type ContractArgs = {contract:string;function:string;args?:HttpValue[] | Record<string,HttpValue>};
type CallOptions = ContractArgs & {to:string;value?:string;fromAddress?:string;block?:string};
declare class ContractChainClient extends EvmChainClient {
  encodeCall(options:ContractArgs):string;
  prepareCall(options:CallOptions):{to:string;data:string;value:string};
  decodeInput(options:ContractArgs & {data:string}):HttpValue;
  decodeOutput(options:ContractArgs & {data:string}):HttpValue;
  decodeError(options:{contract:string;data:string}):HttpValue;
  decodeEvent(options:{contract:string;event:string;topics:string[];data:string}):HttpValue;
  call(options:CallOptions):Promise<HttpValue>;
}
export class PolymarketChainClient extends ContractChainClient {
  constructor(chainId:number|bigint,readRpc:string,writeRpc?:string,codeHashes?:Record<string,string>);
}
export class MetricChainClient extends ContractChainClient {
  constructor(options:{chainId:number|bigint;readRpc:string;writeRpc?:string;codeHashes?:Record<string,string>;signer?:MetricSigner});
  blockNumber():Promise<bigint>;
  block(options?:{number?:string}):Promise<HttpValue|null>;
  nativeBalance(options:{address:string;block?:string}):Promise<string>;
  code(options:{address:string;block?:string}):Promise<string>;
  gasPrice():Promise<string>;
  estimateGas(options:{request:Record<string,HttpValue>}):Promise<bigint>;
  receipt(options:{hash:string}):Promise<HttpValue|null>;
  transaction(options:{hash:string}):Promise<HttpValue|null>;
  transactionCount(options:{address:string}):Promise<bigint>;
  logs(options:{filter:Record<string,HttpValue>}):Promise<HttpValue[]>;
  sendTransaction(options:{request:Record<string,HttpValue>}):Promise<string>;
  sendTransaction(prepared:Record<string,HttpValue>,key:string):Promise<string>;
  contract<K extends keyof MetricContracts>(options:{name:K;address:string}):InstanceType<MetricContracts[K]>;
  liquiditySnapshot(options:{pool:string;lens:string;bins:number[];block?:string}):Promise<HttpValue>;

  constructor(chainId:number|bigint,readRpc:string,writeRpc?:string,codeHashes?:Record<string,string>);
  contract<K extends keyof MetricContracts>(name:K,address:string):InstanceType<MetricContracts[K]>;
  liquiditySnapshot(pool:string,lens:string,bins:number[],block?:string):Promise<HttpValue>;
  oracleStopLossStatus(request:HttpValue):Promise<HttpValue>;
  prepareRoute(request:HttpValue):Promise<HttpValue>;
}

export { HistoricalClient as DatabentoHistoricalClient, ReferenceClient as DatabentoReferenceClient } from "./clients";
export { LiveClient as DatabentoLiveClient };
