// Generated from src/data/chainlink/schema/chainlink_candles.json.
import type { HttpValue } from "./index";

export interface AuthorizeRequest {
  "login": string;
  "password": string;
}

export interface NativeResponse {
  "status": number;
  "headers": Array<[string, string]>;
  "data": HttpValue;
}

export interface SymbolInfoRequest {
  "group"?: string | null;
}

export interface GroupsRequest {
}

export interface CandleHistoryRequest {
  "symbol": string;
  "resolution": string;
  "from": number | bigint;
  "to": number | bigint;
}

export interface CandleHistoryRowsRequest {
  "symbol": string;
  "resolution": string;
  "from": number | bigint;
  "to": number | bigint;
}
