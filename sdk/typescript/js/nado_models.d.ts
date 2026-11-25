// Generated from src/exchanges/nado/schema/nado.json.
import type { HttpValue } from "./index";

export interface MarketsRequest {
  [field: string]: unknown;
}

export interface OrderBookRequest {
  [field: string]: unknown;
  "product_id": number | bigint;
  "depth": number | bigint;
}

export interface TradesRequest {
  [field: string]: unknown;
  "ticker_id": string;
  "limit"?: number | bigint | null;
}

export interface AccountRequest {
  [field: string]: unknown;
  "subaccount": string;
}

export interface PositionsRequest {
  [field: string]: unknown;
  "subaccount": string;
}

export interface OpenOrdersRequest {
  [field: string]: unknown;
  "sender": string;
  "product_id": number | bigint;
}

export interface ContractsRequest {
  [field: string]: unknown;
}

export interface OrderStatusRequest {
  [field: string]: unknown;
  "body": HttpValue;
}

export interface PlaceOrderRequest {
  [field: string]: unknown;
  "body": PlaceOrder;
}

export interface CancelOrderRequest {
  [field: string]: unknown;
  "body": CancelOrder;
}

export interface Subscription {
  [field: string]: unknown;
  "type": string;
  "subaccount"?: string | null;
  "product_id"?: number | null;
  "snapshot_timestamp"?: string | null;
}

export interface PlaceOrder {
  [field: string]: unknown;
  "product_id": number;
  "order": Order;
  "id"?: number | bigint | null;
}

export interface CancelOrder {
  [field: string]: unknown;
  "tx": Cancellation;
}

export interface GatewayReply {
  [field: string]: unknown;
  "status": string;
  "data"?: HttpValue | null;
  "error"?: HttpValue | null;
  "request_type"?: string | null;
}

export interface Order {
  [field: string]: unknown;
  "sender": string;
  "priceX18": string;
  "amount": string;
  "expiration": string;
  "nonce": string;
  "appendix": string;
}

export interface Cancellation {
  [field: string]: unknown;
  "sender": string;
  "productIds": Array<number>;
  "digests": Array<string>;
  "nonce": string;
}
