// Generated from src/exchanges/injective/schema/injective.json.
import type { HttpValue } from "./index";

export interface MarketsRequest {
  [field: string]: unknown;
  "marketStatus"?: string | null;
  "quoteDenom"?: string | null;
}

export interface OrderBookRequest {
  [field: string]: unknown;
  "marketId": string;
  "depth"?: number | bigint | null;
}

export interface TradesRequest {
  [field: string]: unknown;
  "marketId": string;
  "limit"?: number | bigint | null;
  "subaccountId"?: string | null;
}

export interface AccountRequest {
  [field: string]: unknown;
  "subaccountId": string;
}

export interface PositionsRequest {
  [field: string]: unknown;
  "subaccountId": string;
  "marketId"?: string | null;
  "skip"?: number | bigint | null;
  "limit"?: number | bigint | null;
}

export interface OpenOrdersRequest {
  [field: string]: unknown;
  "subaccountId": string;
  "marketId"?: string | null;
  "skip"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "cid"?: string | null;
}

export interface TransactionRequest {
  [field: string]: unknown;
  "hash": string;
}

export interface PendingRequest {
  [field: string]: unknown;
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
  "market_id"?: string | null;
  "subaccount_id"?: string | null;
}

export interface Order {
  [field: string]: unknown;
  "market_id": string;
  "subaccount_id": string;
  "gas_limit": string;
  "fee_amount": string;
  "client_order_id"?: string | null;
  "quote_decimals": number;
  "price": string;
  "quantity": string;
  "margin": string;
  "side": string;
  "kind": string;
  "fee_recipient"?: string | null;
}

export interface CancelOrder {
  [field: string]: unknown;
  "market_id": string;
  "subaccount_id": string;
  "gas_limit": string;
  "fee_amount": string;
  "client_order_id"?: string | null;
  "order_hash"?: string | null;
}

export interface Markets {
  [field: string]: unknown;
  "markets": Array<HttpValue>;
}

export interface Trades {
  [field: string]: unknown;
  "trades": Array<HttpValue>;
}

export interface Positions {
  [field: string]: unknown;
  "positions": Array<HttpValue>;
}

export interface Orders {
  [field: string]: unknown;
  "orders": Array<HttpValue>;
}

export interface Orderbook {
  [field: string]: unknown;
  "orderbook": HttpValue;
}

export interface TransactionResult {
  [field: string]: unknown;
  "tx_response": HttpValue;
}
