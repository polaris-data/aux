// Generated from src/exchanges/lighter/schema/lighter.json.
import type { HttpValue } from "./index";
export type AccountSelector = "index" | "l1_address";

export interface OrderBook {
  [field: string]: unknown;
  "symbol": string;
  "market_id": number | bigint;
  "market_type": string;
  "status": string;
  "supported_size_decimals": number;
  "supported_price_decimals": number;
  "min_base_amount": string;
  "min_quote_amount": string;
}

export interface OrderBooks {
  [field: string]: unknown;
  "code": number | bigint;
  "order_books": Array<OrderBook>;
}

export interface SimpleOrder {
  [field: string]: unknown;
  "order_index": number | bigint;
  "price": string;
  "remaining_base_amount": string;
}

export interface OrderBookOrders {
  [field: string]: unknown;
  "code": number | bigint;
  "total_asks": number | bigint;
  "asks": Array<SimpleOrder>;
  "total_bids": number | bigint;
  "bids": Array<SimpleOrder>;
}

export interface Trade {
  [field: string]: unknown;
  "trade_id": number | bigint;
  "market_id": number | bigint;
  "size": string;
  "price": string;
  "is_maker_ask": boolean;
  "timestamp": number | bigint;
}

export interface Trades {
  [field: string]: unknown;
  "code": number | bigint;
  "trades": Array<Trade>;
}

export interface Position {
  [field: string]: unknown;
  "market_id": number | bigint;
  "symbol": string;
  "sign": number;
  "position": string;
  "avg_entry_price": string;
  "unrealized_pnl": string;
  "realized_pnl": string;
  "liquidation_price": string;
}

export interface Account {
  [field: string]: unknown;
  "index": number | bigint;
  "account_index": number | bigint;
  "l1_address": string;
  "available_balance": string;
  "collateral": string;
  "positions": Array<Position>;
}

export interface Accounts {
  [field: string]: unknown;
  "code": number | bigint;
  "total": number | bigint;
  "accounts": Array<Account>;
}

export interface Order {
  [field: string]: unknown;
  "order_index": number | bigint;
  "client_order_index": number | bigint;
  "market_index": number | bigint;
  "owner_account_index": number | bigint;
  "price": string;
  "remaining_base_amount": string;
  "is_ask": boolean;
  "status": string;
}

export interface Orders {
  [field: string]: unknown;
  "code": number | bigint;
  "orders": Array<Order>;
}

export interface NextNonce {
  [field: string]: unknown;
  "code": number | bigint;
  "nonce": number | bigint;
}

export interface OrderBooksRequest {
  "market_id"?: number | bigint | null;
}

export interface OrderBookOrdersRequest {
  "market_id": number | bigint;
  "limit": number;
}

export interface RecentTradesRequest {
  "market_id": number | bigint;
  "limit": number;
}

export interface NextNonceRequest {
  "account_index": number | bigint;
  "api_key_index": number;
}

export interface ActiveOrdersRequest {
  "account_index"?: number | bigint | null;
  "market_id"?: number | bigint | null;
}

export interface AccountRequest {
  "by": AccountSelector;
  "value": HttpValue;
}
