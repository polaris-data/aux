// Generated from schema/pyth_pro.json.
import type { HttpValue } from "./index";

export interface LiveRequest {
}

export interface HealthResponse {
  "status": number;
  "headers": Array<[string, string]>;
  "data": string;
}

export interface ReadyRequest {
}

export interface SymbolsRequest {
  "query"?: string | null;
  "asset_type"?: Array<string> | null;
  "instrument_type"?: Array<string> | null;
  "symbol_chain_id"?: Array<string> | null;
  "entitled_only"?: boolean | null;
}

export interface NativeResponse {
  "status": number;
  "headers": Array<[string, string]>;
  "data": HttpValue;
}

export interface ConfigRequest {
  "channel": string;
}

export interface SearchRequest {
  "channel": string;
  "query": string;
  "type"?: string | null;
  "exchange"?: string | null;
  "limit"?: number | null;
}

export interface ResolveSymbolRequest {
  "channel": string;
  "symbol": string;
}

export interface HistoryRequest {
  "channel": string;
  "symbol": string;
  "from": number | bigint;
  "to": number | bigint;
  "resolution": string;
  "split_adjusted"?: boolean | null;
}

export interface PriceRequest {
  "channel": string;
  "ids": Array<number>;
  "timestamp": number | bigint;
  "split_adjusted"?: boolean | null;
}

export interface PriceRangeRequest {
  "channel": string;
  "ids": Array<number>;
  "start_timestamp": number | bigint;
  "end_timestamp": number | bigint;
  "limit"?: number | null;
  "after"?: string | null;
  "split_adjusted"?: boolean | null;
}

export interface PublisherUpdatesRequest {
  "price_feed_id"?: Array<number> | null;
  "min_timestamp"?: string | null;
  "max_timestamp"?: string | null;
  "status"?: string | null;
  "status_reason"?: string | null;
  "status_reason_unset"?: boolean | null;
  "sort": string;
  "limit"?: number | null;
}

export interface LatestPriceRequest {
  "channel": string;
  "priceFeedIds"?: Array<number> | null;
  "properties"?: Array<string> | null;
  "formats": Array<string>;
  "parsed"?: boolean | null;
  "jsonBinaryEncoding"?: string | null;
  "symbols"?: Array<string> | null;
}

export interface PriceAtTimestampRequest {
  "channel": string;
  "priceFeedIds"?: Array<number> | null;
  "properties"?: Array<string> | null;
  "formats": Array<string>;
  "parsed"?: boolean | null;
  "jsonBinaryEncoding"?: string | null;
  "timestamp": number | bigint;
  "symbols"?: Array<string> | null;
}

export interface ReducePriceRequest {
  "payload": HttpValue;
  "priceFeedIds": Array<number>;
}

export interface PublisherStakeCapsRequest {
}

export interface GuardianSetUpgradeRequest {
}
