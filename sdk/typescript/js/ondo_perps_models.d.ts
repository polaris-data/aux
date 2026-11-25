// Generated from src/exchanges/ondo_perps/schema/ondo_perps.json.
import type { HttpValue } from "./index";

export interface MarketsRequest {
  [field: string]: unknown;
  "sparkline"?: boolean | null;
}

export interface OrderBookRequest {
  [field: string]: unknown;
  "market": string;
  "depth"?: number | bigint | null;
}

export interface TradesRequest {
  [field: string]: unknown;
  "market": string;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
}

export interface AccountRequest {
  [field: string]: unknown;
}

export interface PositionsRequest {
  [field: string]: unknown;
}

export interface OpenOrdersRequest {
  [field: string]: unknown;
  "market"?: string | null;
  "limit"?: number | bigint | null;
  "cursor"?: string | null;
}

export interface OrderStatusRequest {
  [field: string]: unknown;
  "orderID": string;
}

export interface PlaceOrderRequest {
  [field: string]: unknown;
  "body": Order;
}

export interface CancelOrderRequest {
  [field: string]: unknown;
  "orderID": string;
}

export interface Subscription {
  [field: string]: unknown;
  "channel": string;
  "markets"?: Array<string> | null;
  "numPastTrades"?: number | bigint | null;
  "depthLevels"?: HttpValue | null;
  "limit"?: number | bigint | null;
}

export interface Order {
  [field: string]: unknown;
  "market": string;
  "side": string;
  "type": string;
  "size": string;
  "price"?: string | null;
  "clientOrderId"?: string | null;
  "reduceOnly"?: boolean | null;
  "postOnly"?: boolean | null;
  "builderCode"?: string | null;
}

export interface OndoReply {
  [field: string]: unknown;
  "success": boolean;
  "result"?: HttpValue | null;
  "error"?: HttpValue | null;
}
