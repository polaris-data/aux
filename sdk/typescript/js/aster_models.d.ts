// Generated from src/exchanges/aster/schema/aster.json.
import type { HttpValue } from "./index";

export interface MarketsRequest {
  [field: string]: unknown;
}

export interface OrderBookRequest {
  [field: string]: unknown;
  "symbol": string;
  "limit"?: number | bigint | null;
}

export interface TradesRequest {
  [field: string]: unknown;
  "symbol": string;
  "limit"?: number | bigint | null;
}

export interface AccountRequest {
  [field: string]: unknown;
}

export interface PositionsRequest {
  [field: string]: unknown;
  "symbol"?: string | null;
}

export interface OpenOrdersRequest {
  [field: string]: unknown;
  "symbol"?: string | null;
}

export interface OrderStatusRequest {
  [field: string]: unknown;
  "symbol": string;
  "orderId"?: number | bigint | null;
  "origClientOrderId"?: string | null;
}

export interface PlaceOrderRequest {
  [field: string]: unknown;
  "body": Order;
}

export interface CancelOrderRequest {
  [field: string]: unknown;
  "body": CancelOrder;
}

export interface Subscription {
  [field: string]: unknown;
  "channel": string;
  "symbol"?: string | null;
}

export interface Order {
  [field: string]: unknown;
  "symbol": string;
  "side": string;
  "type": string;
  "timeInForce"?: string | null;
  "price"?: string | null;
  "quantity": string;
  "reduceOnly"?: string | null;
  "positionSide"?: string | null;
  "newClientOrderId"?: string | null;
}

export interface CancelOrder {
  [field: string]: unknown;
  "symbol": string;
  "orderId"?: number | bigint | null;
  "origClientOrderId"?: string | null;
}

export interface Markets {
  [field: string]: unknown;
  "symbols": Array<HttpValue>;
}

export interface Orderbook {
  [field: string]: unknown;
  "lastUpdateId": number | bigint;
  "bids": Array<Array<string>>;
  "asks": Array<Array<string>>;
}

export interface Account {
  [field: string]: unknown;
  "assets": Array<HttpValue>;
  "positions": Array<HttpValue>;
}

export interface OrderReply {
  [field: string]: unknown;
  "orderId": number | bigint;
}
