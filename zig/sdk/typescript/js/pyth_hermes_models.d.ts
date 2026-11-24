// Generated from schema/pyth_hermes.json.
import type { HttpValue } from "./index";

export interface HermesLiveRequest {
}

export interface HealthResponse {
  "status": number;
  "headers": Array<[string, string]>;
  "data": string;
}

export interface HermesReadyRequest {
}

export interface PriceFeedsRequest {
  "query"?: string | null;
  "asset_type"?: string | null;
}

export interface NativeResponse {
  "status": number;
  "headers": Array<[string, string]>;
  "data": HttpValue;
}

export interface LatestPriceUpdatesRequest {
  "ids": Array<string>;
  "encoding"?: string | null;
  "parsed"?: boolean | null;
  "ignore_invalid_price_ids"?: boolean | null;
}

export interface PriceUpdatesAtTimestampRequest {
  "publish_time": number | bigint;
  "ids": Array<string>;
  "encoding"?: string | null;
  "parsed"?: boolean | null;
  "ignore_invalid_price_ids"?: boolean | null;
}

export interface LatestTwapsRequest {
  "window_seconds": number | bigint;
}

export interface LatestPublisherStakeCapsRequest {
  "encoding"?: string | null;
  "parsed"?: boolean | null;
}
