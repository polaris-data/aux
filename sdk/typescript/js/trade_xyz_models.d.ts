// Generated from src/exchanges/trade_xyz/schema/trade_xyz.json.
import type { HttpValue } from "./index";

export interface MarketsRequest {
  [field: string]: unknown;
}

export interface OrderBookRequest {
  [field: string]: unknown;
  "coin": string;
}

export interface TradesRequest {
  [field: string]: unknown;
  "coin": string;
}

export interface AccountRequest {
  [field: string]: unknown;
  "user": string;
}

export interface PositionsRequest {
  [field: string]: unknown;
  "user": string;
}

export interface OpenOrdersRequest {
  [field: string]: unknown;
  "user": string;
}

export interface OrderStatusRequest {
  [field: string]: unknown;
  "user": string;
  "coin": string;
  "oid": HttpValue;
}

export interface PlaceOrderRequest {
  [field: string]: unknown;
  "coin": string;
  "body": Order;
  "options"?: HttpValue | null;
}

export interface CancelOrderRequest {
  [field: string]: unknown;
  "coin": string;
  "body": CancelOrder;
  "options"?: HttpValue | null;
}

export interface PerpDexsRequest {
  [field: string]: unknown;
}

export interface Subscription {
  [field: string]: unknown;
  "type": string;
  "coin"?: string | null;
  "user"?: string | null;
  "aggregateByTime"?: boolean | null;
}

export interface Order {
  [field: string]: unknown;
  "b": boolean;
  "p": string;
  "s": string;
  "r": boolean;
  "t": Limit;
  "c"?: string | null;
}

export interface CancelOrder {
  [field: string]: unknown;
  "o"?: number | bigint | null;
  "cloid"?: string | null;
}

export interface Markets {
  [field: string]: unknown;
  "universe": Array<Market>;
}

export interface Account {
  [field: string]: unknown;
  "assetPositions": Array<HttpValue>;
}

export interface Orderbook {
  [field: string]: unknown;
  "coin": string;
  "levels": Array<Array<BookLevel>>;
}

export interface Market {
  [field: string]: unknown;
  "name": string;
  "szDecimals"?: number | null;
  "maxLeverage"?: number | null;
  "isDelisted"?: boolean | null;
}

export interface BookLevel {
  [field: string]: unknown;
  "px": string;
  "sz": string;
  "n": number | bigint;
}

export interface Limit {
  [field: string]: unknown;
  "limit": TimeInForce;
}

export interface TimeInForce {
  [field: string]: unknown;
  "tif": string;
}
