// Generated from src/exchanges/pacifica/schema/pacifica.json.
import type { HttpValue } from "./index";

export interface MarketsRequest {
  [field: string]: unknown;
}

export interface OrderBookRequest {
  [field: string]: unknown;
  "symbol": string;
  "agg_level"?: number | bigint | null;
}

export interface TradesRequest {
  [field: string]: unknown;
  "symbol": string;
}

export interface AccountRequest {
  [field: string]: unknown;
  "account": string;
}

export interface PositionsRequest {
  [field: string]: unknown;
  "account": string;
}

export interface OpenOrdersRequest {
  [field: string]: unknown;
  "account": string;
}

export interface OrderStatusRequest {
  [field: string]: unknown;
  "order_id": number | bigint;
}

export interface PlaceOrderRequest {
  [field: string]: unknown;
  "body": LimitOrder;
}

export interface PlaceMarketOrderRequest {
  [field: string]: unknown;
  "body": MarketOrder;
}

export interface CancelOrderRequest {
  [field: string]: unknown;
  "body": CancelOrder;
}

export interface Subscription {
  [field: string]: unknown;
  "source": string;
  "symbol"?: string | null;
  "account"?: string | null;
  "agg_level"?: number | bigint | null;
}

export interface LimitOrder {
  [field: string]: unknown;
  "symbol": string;
  "side": string;
  "amount": string;
  "reduce_only"?: boolean | null;
  "client_order_id"?: string | null;
  "price": string;
  "tif": string;
}

export interface MarketOrder {
  [field: string]: unknown;
  "symbol": string;
  "side": string;
  "amount": string;
  "reduce_only"?: boolean | null;
  "client_order_id"?: string | null;
  "slippage_percent": string;
}

export interface CancelOrder {
  [field: string]: unknown;
  "symbol": string;
  "order_id"?: number | bigint | null;
  "client_order_id"?: string | null;
}

export interface PacificaReply {
  [field: string]: unknown;
  "success": boolean;
  "data"?: HttpValue | null;
  "code"?: number | bigint | string | null;
  "error"?: HttpValue | null;
}
