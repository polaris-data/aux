// Generated from src/exchanges/arcus/schema/arcus.json.
import type { HttpValue } from "./index";
export type Side = "BUY" | "SELL";
export type OrderType = "LIMIT" | "MARKET";
export type TimeInForce = "GTT" | "FOK" | "IOC" | "ALO";

export interface MarketInfo {
  [field: string]: unknown;
  "marketId": number;
  "marketDisplayName": string;
  "tickSize": string;
  "stepSize": string;
  "status"?: string;
}

export interface Markets {
  [field: string]: unknown;
  "markets": Array<MarketInfo>;
}

export interface OrderBook {
  [field: string]: unknown;
  "bids": Array<[string, string]>;
  "asks": Array<[string, string]>;
  "lastSequenceId"?: number | bigint;
  "globalSequenceId"?: number | bigint;
  "timestamp": number | bigint;
}

export interface Trade {
  [field: string]: unknown;
  "marketId": number;
  "marketDisplayName": string;
  "side": string;
  "price": string;
  "size": string;
  "tradeId": string;
  "timestamp": number | bigint;
  "sequenceNumber"?: number | bigint;
}

export interface Trades {
  [field: string]: unknown;
  "trades": Array<Trade>;
  "total"?: number | bigint;
}

export interface Position {
  [field: string]: unknown;
  "marketId": number;
  "marketDisplayName": string;
  "side": string;
  "size": string;
  "averageEntryPrice": string;
}

export interface Positions {
  [field: string]: unknown;
  "positions": Record<string, Position>;
  "total"?: number | bigint;
}

export interface Account {
  [field: string]: unknown;
  "address": string;
  "accountIndex": number;
  "equity": string;
  "freeCollateral": string;
  "netQuoteBalance": string;
  "positions": Record<string, Position>;
  "sequenceNumber": number | bigint;
}

export interface OpenOrders {
  [field: string]: unknown;
  "orders": Array<HttpValue>;
  "total"?: number | bigint;
}

export interface MarketsRequest {
  "market"?: string | null;
}

export interface OrderBookRequest {
  "market": string;
  "nLevels": number;
}

export interface AccountRequest {
  "address": string;
  "accountIndex": number;
}

export interface TradesRequest {
  "market": string;
  "limit": number;
}

export interface OrderRequest {
  "marketId": number;
  "orderSide": Side;
  "orderType": OrderType;
  "quantity": string;
  "price": string;
  "timeInForce": TimeInForce;
  "goodTilTime": string;
  "clientId"?: string | null;
  "reduceOnly"?: boolean | null;
}

export interface PlaceOrderParams {
  "market": MarketInfo;
  "body": OrderRequest;
}

export interface CancelOrderRequest {
  "marketId": number;
  "orderId"?: string | null;
  "clientId"?: string | null;
}

export interface OrderResponse {
  [field: string]: unknown;
  "status": string;
  "orderId"?: string | null;
  "clientId"?: string | null;
}
