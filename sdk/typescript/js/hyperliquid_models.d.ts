// Generated from src/exchanges/hyperliquid/schema/hyperliquid.json.
import type { HttpValue } from "./index";
export type Tif = "Alo" | "Ioc" | "Gtc";
export type Tpsl = "tp" | "sl";
export type NamedGrouping = "na" | "normalTpsl" | "positionTpsl";
export type CrossKind = "cross";
export type IsolatedKind = "isolated";
export type OkStatus = "ok";
export type ErrStatus = "err";
export type OrderType = LimitOrder | TriggerOrder;
export type Grouping = NamedGrouping | PriorityGrouping;
export type ItemStatus = RestingStatus | FilledStatus | ErrorStatus | string | HttpValue;
export type TradingResponse = OkResponse | ErrResponse;
export type Leverage = CrossLeverage | IsolatedLeverage;
export type Cloid = string;
export type Address = string;

export interface Meta {
  [field: string]: unknown;
  "universe": Array<Asset>;
  "marginTables"?: Array<[number, MarginTable]>;
}

export interface Asset {
  [field: string]: unknown;
  "name": string;
  "szDecimals": number;
  "maxLeverage": number;
  "onlyIsolated"?: boolean | null;
  "isDelisted"?: boolean | null;
  "marginTableId"?: number | null;
  "marginMode"?: string | null;
}

export interface MarginTable {
  [field: string]: unknown;
  "description": string;
  "marginTiers": Array<MarginTier>;
}

export interface MarginTier {
  [field: string]: unknown;
  "lowerBound": string;
  "maxLeverage": number;
}

export interface L2Book {
  [field: string]: unknown;
  "coin": string;
  "time": number | bigint;
  "levels": [Array<Level>, Array<Level>];
}

export interface Level {
  [field: string]: unknown;
  "px": string;
  "sz": string;
  "n": number | bigint;
}

export interface MetaRequest {
  [field: string]: unknown;
  "dex"?: string | null;
}

export interface L2BookRequest {
  [field: string]: unknown;
  "coin": string;
  "nSigFigs"?: number | null;
  "mantissa"?: number | null;
}

export interface Order {
  [field: string]: unknown;
  "a": number;
  "b": boolean;
  "p": string;
  "s": string;
  "r": boolean;
  "t": OrderType;
  "c"?: Cloid | null;
}

export interface SubBuilder {
  [field: string]: unknown;
  "b": Address;
  "s": number;
}

export interface BuilderFee {
  [field: string]: unknown;
  "b": Address;
  "f": number;
  "s"?: Array<SubBuilder> | null;
}

export interface OrderRequest {
  [field: string]: unknown;
  "orders": Array<Order>;
  "grouping"?: Grouping | null;
  "builder"?: BuilderFee | null;
  "options"?: ExecutionOptions | null;
}

export interface Cancel {
  [field: string]: unknown;
  "a": number;
  "o": number | bigint;
}

export interface CancelRequest {
  [field: string]: unknown;
  "cancels": Array<Cancel>;
  "options"?: ExecutionOptions | null;
}

export interface CancelByCloid {
  [field: string]: unknown;
  "asset": number;
  "cloid": Cloid;
}

export interface CancelByCloidRequest {
  [field: string]: unknown;
  "cancels": Array<CancelByCloid>;
  "options"?: ExecutionOptions | null;
}

export interface ExecutionOptions {
  [field: string]: unknown;
  "nonce"?: number | bigint | null;
  "expires_after"?: number | bigint | null;
}

export interface BatchResponse {
  [field: string]: unknown;
  "type": string;
  "data": BatchData;
}

export interface BatchData {
  [field: string]: unknown;
  "statuses": Array<ItemStatus>;
}

export interface RestingOrder {
  [field: string]: unknown;
  "oid": number | bigint;
  "cloid"?: Cloid | null;
}

export interface FilledOrder {
  [field: string]: unknown;
  "totalSz": string;
  "avgPx": string;
  "oid": number | bigint;
  "cloid"?: Cloid | null;
}

export interface UserStateRequest {
  [field: string]: unknown;
  "user": Address;
  "dex"?: string | null;
}

export interface OpenOrdersRequest {
  [field: string]: unknown;
  "user": Address;
  "dex"?: string | null;
}

export interface UserState {
  [field: string]: unknown;
  "assetPositions": Array<AssetPosition>;
  "crossMaintenanceMarginUsed": string;
  "crossMarginSummary": MarginSummary;
  "marginSummary": MarginSummary;
  "withdrawable": string;
  "time"?: number | bigint | null;
}

export interface MarginSummary {
  [field: string]: unknown;
  "accountValue": string;
  "totalMarginUsed": string;
  "totalNtlPos": string;
  "totalRawUsd": string;
}

export interface AssetPosition {
  [field: string]: unknown;
  "type": string;
  "position": Position;
}

export interface Position {
  [field: string]: unknown;
  "coin": string;
  "cumFunding": CumulativeFunding;
  "entryPx"?: string | null;
  "leverage": Leverage;
  "liquidationPx"?: string | null;
  "marginUsed": string;
  "maxLeverage": number;
  "positionValue": string;
  "returnOnEquity": string;
  "szi": string;
  "unrealizedPnl": string;
}

export interface CumulativeFunding {
  [field: string]: unknown;
  "allTime": string;
  "sinceChange": string;
  "sinceOpen": string;
}

export interface OpenOrder {
  [field: string]: unknown;
  "coin": string;
  "limitPx": string;
  "oid": number | bigint;
  "side": string;
  "sz": string;
  "timestamp": number | bigint;
  "cloid"?: Cloid | null;
  "origSz"?: string | null;
}

export interface LimitDetails {
  "tif": Tif;
}

export interface TriggerDetails {
  "isMarket": boolean;
  "triggerPx": string;
  "tpsl": Tpsl;
}

export interface LimitOrder {
  "limit": LimitDetails;
}

export interface TriggerOrder {
  "trigger": TriggerDetails;
}

export interface PriorityGrouping {
  "p": number;
}

export interface RestingStatus {
  "resting": RestingOrder;
}

export interface FilledStatus {
  "filled": FilledOrder;
}

export interface ErrorStatus {
  "error": string;
}

export interface OkResponse {
  "status": OkStatus;
  "response": BatchResponse;
}

export interface ErrResponse {
  "status": ErrStatus;
  "response": string;
}

export interface CrossLeverage {
  "type": CrossKind;
  "value": number;
}

export interface IsolatedLeverage {
  "type": IsolatedKind;
  "value": number;
  "rawUsd": string;
}
