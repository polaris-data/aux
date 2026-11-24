// Generated from schema/qfex.json.
import type { HttpValue } from "./index";

export interface Symbol {
  [field: string]: unknown;
  "clobPairId": string;
  "symbol": string;
  "tick_size": string;
  "lot_size": string;
  "min_quantity"?: string;
  "max_quantity"?: string;
  "status"?: string;
}

export interface Refdata {
  [field: string]: unknown;
  "type": string;
  "data"?: Array<Symbol>;
}

export interface Contracts {
  [field: string]: unknown;
  "data"?: Array<HttpValue>;
}

export interface OrderBook {
  [field: string]: unknown;
  "ticker_id": string;
  "timestamp": number | bigint;
  "bids"?: Array<[string, string]>;
  "asks"?: Array<[string, string]>;
}

export interface Position {
  [field: string]: unknown;
  "symbol": string;
  "position": number | bigint | string;
  "realised_pnl": number | bigint | string;
  "unrealised_pnl": number | bigint | string;
  "average_price": number | bigint | string;
}

export interface Balance {
  [field: string]: unknown;
  "available_balance": number | bigint | string;
  "deposit": number | bigint | string;
}

export interface PositionsBalance {
  [field: string]: unknown;
  "positions"?: Array<Position>;
  "balance": Balance;
}

export interface RefdataRequest {
  "ticker"?: string | null;
}

export interface ContractsRequest {
  "symbol"?: string | null;
}

export interface OrderBookRequest {
  "symbol": string;
}

export interface EmptyRequest {
}
