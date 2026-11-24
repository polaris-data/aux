// Generated from schema/metric.json.
import type { HttpValue } from "./index";
export type TradeType = "exact_in" | "exact_out";
export type FactorySettingsResponse = Record<string, HttpValue>;

export interface MetadataRequest {
  "chainId": number | bigint;
  "count"?: number | null;
  "offset"?: number | bigint | null;
  "sortBy"?: string | null;
  "sortOrder"?: string | null;
  "include24h"?: boolean | null;
  "minTvlFiat"?: number | bigint | string | null;
  "tier"?: string | null;
  "lockedPricingOnly"?: boolean | null;
  "depositAllowlistEnabled"?: boolean | null;
  "blacklistedOnly"?: boolean | null;
  "addresses"?: string | null;
  "contract_version"?: string | null;
  "fiat"?: string | null;
  "chainIds"?: string | null;
}

export interface Chain {
  [field: string]: unknown;
  "type": string;
  "chainId": number | bigint;
  "name": string;
}

export interface ChainsResponse {
  [field: string]: unknown;
  "chains": Array<Chain>;
}

export interface PoolMetadata {
  [field: string]: unknown;
  "poolAddress": string;
  "pair": string;
  "chain": string;
  "token0": string;
  "token1": string;
  "token0Decimals": number;
  "token1Decimals": number;
  "priceProviderAddress"?: string | null;
}

export interface MetadataResponse {
  [field: string]: unknown;
  "data": Array<PoolMetadata>;
  "total": number | bigint;
  "nextOffset"?: number | bigint | null;
}

export interface DepthLevel {
  [field: string]: unknown;
  "binIdx": number;
  "price": string;
  "cumulativeVolume": string;
  "cumulativeInputVolume": string;
  "priceImpactE6": string;
}

export interface Depth {
  [field: string]: unknown;
  "bids": Array<DepthLevel>;
  "asks": Array<DepthLevel>;
}

export interface BidAskResponse {
  [field: string]: unknown;
  "poolAddress"?: string | null;
  "pair"?: string | null;
  "priceProviderStatus": string;
  "bidAdj"?: string | null;
  "askAdj"?: string | null;
  "totalToken0Available"?: string | null;
  "totalToken1Available"?: string | null;
  "serverTs"?: number | bigint | null;
  "depth"?: Depth | null;
}

export interface BatchBidAskRequest {
  "chainId": number | bigint;
  "pools": Array<string>;
  "depth"?: boolean | null;
}

export interface BatchBidAskResponse {
  [field: string]: unknown;
  "results": Array<PoolQuote>;
}

export interface PoolQuote {
  [field: string]: unknown;
  "poolAddress": string;
  "quote"?: BidAskResponse | null;
  "error"?: PoolQuoteError | null;
}

export interface PoolQuoteError {
  [field: string]: unknown;
  "status": number;
  "error": string;
}

export interface RouteRequest {
  "chainId": number | bigint;
  "tokenIn": string;
  "tokenOut": string;
  "tradeType"?: TradeType | null;
  "amountIn"?: string | null;
  "amountOut"?: string | null;
  "maxRoutes"?: number | null;
  "poolAddress"?: string | null;
}

export interface RouteLeg {
  [field: string]: unknown;
  "tradeType": TradeType;
  "pair": string;
  "effectivePriceX64": string;
  "priceImpactE6": string;
  "poolAddress": string;
  "zeroForOne": boolean;
  "tokenIn": string;
  "tokenOut": string;
  "amountIn": string;
  "amountOut": string;
  "priceLimitX64": string;
}

export interface RouteOption {
  [field: string]: unknown;
  "tradeType": TradeType;
  "routeType": string;
  "effectivePriceX64": string;
  "priceImpactE6": string;
  "amountIn": string;
  "amountOut": string;
  "legs": Array<RouteLeg>;
}

export interface RouteResponse {
  [field: string]: unknown;
  "tokenIn": string;
  "tokenOut": string;
  "effectivePriceX64": string;
  "priceImpactBestRoute": string;
  "candidatesConsidered": number | bigint;
  "tradeType": TradeType;
  "amountIn": string;
  "amountOut": string;
  "routes": Array<RouteOption>;
}

export interface PammQuoteRequest {
  "chainId": number | bigint;
  "tx": string;
  "blockNumber": number | bigint;
  "replacementUuid": string;
  "replacementSeqNumber": number | bigint;
  "disableCrossRegionSharing"?: boolean | null;
  "responseTimeoutMs"?: number | null;
  "maxTimestamp"?: number | bigint | null;
  "alwaysInclude"?: boolean | null;
  "quoteAddress"?: string | null;
  "poolAddresses"?: Array<string> | null;
  "assetPairs"?: Array<string> | null;
}

export interface BuilderOutcome {
  [field: string]: unknown;
  "provider": string;
  "accepted": boolean;
  "pending"?: boolean;
  "error"?: string | null;
  "bundleHash"?: string | null;
}

export interface PammQuoteResponse {
  [field: string]: unknown;
  "accepted": boolean;
  "providers": Array<BuilderOutcome>;
}

export interface EmptyRequest {
}

export interface BidAskRequest {
  "chainId": number | bigint;
  "pool": string;
  "depth"?: boolean | null;
}

export interface CrossChainMetadataRequest {
  "count"?: number | null;
  "offset"?: number | bigint | null;
  "sortBy"?: string | null;
  "sortOrder"?: string | null;
  "include24h"?: boolean | null;
  "minTvlFiat"?: number | bigint | string | null;
  "tier"?: string | null;
  "lockedPricingOnly"?: boolean | null;
  "depositAllowlistEnabled"?: boolean | null;
  "blacklistedOnly"?: boolean | null;
  "addresses"?: string | null;
  "contract_version"?: string | null;
  "fiat"?: string | null;
  "chainIds"?: string | null;
}
