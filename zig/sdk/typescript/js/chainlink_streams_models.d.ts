// Generated from src/data/chainlink/schema/chainlink_streams.json.
import type { HttpValue } from "./index";

export interface DiscoveryRequest {
  "base_asset"?: string | null;
  "quote_asset"?: string | null;
  "asset_class"?: string | null;
  "feed_type"?: string | null;
  "attribute_type"?: string | null;
  "status"?: string | null;
  "network_type"?: string | null;
  "created_after"?: string | null;
  "created_before"?: string | null;
  "expected_decommission_after"?: string | null;
  "expected_decommission_before"?: string | null;
  "decommissioned_after"?: string | null;
  "decommissioned_before"?: string | null;
  "hidden"?: boolean | null;
}

export interface NativeResponse {
  "status": number;
  "headers": Array<[string, string]>;
  "data": HttpValue;
}

export interface ReportRequest {
  "feedID": string;
  "timestamp": number | bigint;
}

export interface LatestReportRequest {
  "feedID": string;
}

export interface BulkReportsRequest {
  "feedIDs": Array<string>;
  "timestamp": number | bigint;
}

export interface ReportPageRequest {
  "feedID": string;
  "startTimestamp": number | bigint;
  "limit"?: number | null;
}
