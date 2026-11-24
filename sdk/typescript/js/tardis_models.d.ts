// Generated from src/data/tardis/schema/tardis.json.
import type { HttpValue } from "./index";
export type Params = Record<string, HttpValue>;

export interface EmptyRequest {
}

export interface ExchangeRequest {
  "exchange": string;
}

export interface InstrumentRequest {
  "exchange": string;
  "symbol": string;
}

export interface InstrumentsRequest {
  [field: string]: unknown;
  "exchange": string;
}
