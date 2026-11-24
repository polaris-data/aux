// Generated from src/data/birdeye/schema/birdeye.json.
import type { HttpValue } from "./index";

export interface PriceRequest {
  "check_liquidity"?: number | bigint | string | null;
  "include_liquidity"?: boolean | null;
  "address": string;
  "ui_amount_mode"?: string | null;
}

export interface MultiPriceRequest {
  "check_liquidity"?: number | bigint | string | null;
  "include_liquidity"?: boolean | null;
  "list_address": string;
  "ui_amount_mode"?: string | null;
}

export interface MultiPricePostBody {
  "list_address": string;
}

export interface MultiPricePostRequest {
  "check_liquidity"?: number | bigint | string | null;
  "include_liquidity"?: boolean | null;
  "ui_amount_mode"?: string | null;
  "body": MultiPricePostBody;
}

export interface TokenTradesRequest {
  "address": string;
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "tx_type"?: string | null;
  "sort_type"?: string | null;
  "ui_amount_mode"?: string | null;
}

export interface PairTradesRequest {
  "address": string;
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "tx_type"?: string | null;
  "sort_type"?: string | null;
  "ui_amount_mode"?: string | null;
}

export interface TokenTradesByTimeRequest {
  "address": string;
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "tx_type"?: string | null;
  "before_time"?: number | bigint | null;
  "after_time"?: number | bigint | null;
  "ui_amount_mode"?: string | null;
}

export interface PairTradesByTimeRequest {
  "address": string;
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "tx_type"?: string | null;
  "before_time"?: number | bigint | null;
  "after_time"?: number | bigint | null;
  "ui_amount_mode"?: string | null;
}

export interface TradesV3Request {
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "sort_by"?: string | null;
  "sort_type"?: string | null;
  "tx_type"?: string | null;
  "source"?: string | null;
  "owner"?: string | null;
  "pool_id"?: string | null;
  "before_time"?: number | bigint | null;
  "after_time"?: number | bigint | null;
  "before_block_number"?: number | bigint | null;
  "after_block_number"?: number | bigint | null;
  "ui_amount_mode"?: string | null;
}

export interface TokenTradesV3Request {
  "address": string;
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "sort_by"?: string | null;
  "sort_type"?: string | null;
  "tx_type"?: string | null;
  "source"?: string | null;
  "owner"?: string | null;
  "pool_id"?: string | null;
  "before_time"?: number | bigint | null;
  "after_time"?: number | bigint | null;
  "before_block_number"?: number | bigint | null;
  "after_block_number"?: number | bigint | null;
  "ui_amount_mode"?: string | null;
}

export interface RecentTradesRequest {
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "tx_type"?: string | null;
  "owner"?: string | null;
  "before_time"?: number | bigint | null;
  "after_time"?: number | bigint | null;
  "before_block_number"?: number | bigint | null;
  "after_block_number"?: number | bigint | null;
  "ui_amount_mode"?: string | null;
}

export interface OhlcvRequest {
  "address": string;
  "type": string;
  "currency"?: string | null;
  "chart_type"?: string | null;
  "time_from": number | bigint;
  "time_to": number | bigint;
  "ui_amount_mode"?: string | null;
}

export interface OhlcvPairRequest {
  "address": string;
  "type": string;
  "time_from": number | bigint;
  "time_to": number | bigint;
}

export interface OhlcvBaseQuoteRequest {
  "base_address": string;
  "quote_address": string;
  "type": string;
  "time_from": number | bigint;
  "time_to": number | bigint;
  "ui_amount_mode"?: string | null;
}

export interface OhlcvV3Request {
  "address": string;
  "type": string;
  "currency"?: string | null;
  "chart_type"?: string | null;
  "time_from": number | bigint;
  "time_to": number | bigint;
  "ui_amount_mode"?: string | null;
  "mode"?: string | null;
  "count_limit"?: number | bigint | null;
  "padding"?: boolean | null;
  "outlier"?: boolean | null;
}

export interface OhlcvPairV3Request {
  "address": string;
  "type": string;
  "time_from": number | bigint;
  "time_to": number | bigint;
  "mode"?: string | null;
  "count_limit"?: number | bigint | null;
  "padding"?: boolean | null;
  "outlier"?: boolean | null;
  "inversion"?: boolean | null;
}

export interface LiquidityOhlcPairRequest {
  "address": string;
  "time"?: number | bigint | null;
  "direction"?: string | null;
  "count"?: number | bigint | null;
}

export interface LiquidityOhlcTokenRequest {
  "address": string;
  "resolution"?: string | null;
  "time"?: number | bigint | null;
  "direction"?: string | null;
  "count"?: number | bigint | null;
}

export interface LiquidityHistoryTokenRequest {
  "address": string;
  "resolution"?: string | null;
  "time"?: number | bigint | null;
  "direction"?: string | null;
  "count"?: number | bigint | null;
}

export interface LiquidityHistoryPairRequest {
  "address": string;
  "unix_time"?: number | bigint | null;
  "block_number"?: number | bigint | null;
  "step"?: number | bigint | null;
  "skip_empty"?: boolean | null;
  "direction"?: string | null;
  "count"?: number | bigint | null;
}

export interface LiquidityLatestTokenBody {
  "addresses": Array<string>;
}

export interface LiquidityLatestTokenRequest {
  "body": LiquidityLatestTokenBody;
}

export interface HistoryPriceRequest {
  "address": string;
  "address_type": string;
  "type": string;
  "time_from": number | bigint;
  "time_to": number | bigint;
  "ui_amount_mode"?: string | null;
}

export interface HistoricalPriceUnixRequest {
  "address": string;
  "unixtime"?: number | bigint | null;
  "ui_amount_mode"?: string | null;
}

export interface PriceVolumeSingleRequest {
  "address": string;
  "type"?: string | null;
  "ui_amount_mode"?: string | null;
}

export interface PriceVolumeMultiBody {
  "list_address": string;
  "type"?: string | null;
}

export interface PriceVolumeMultiRequest {
  "ui_amount_mode"?: string | null;
  "body": PriceVolumeMultiBody;
}

export interface PairOverviewRequest {
  "address": string;
  "ui_amount_mode"?: string | null;
}

export interface PairOverviewMultipleRequest {
  "list_address": string;
  "ui_amount_mode"?: string | null;
}

export interface PriceStatsRequest {
  "list_timeframe"?: string | null;
  "address": string;
  "ui_amount_mode"?: string | null;
}

export interface PriceStatsMultipleBody {
  "list_address": string;
}

export interface PriceStatsMultipleRequest {
  "list_timeframe"?: string | null;
  "ui_amount_mode"?: string | null;
  "body": PriceStatsMultipleBody;
}

export interface TokenTradesByVolumeRequest {
  "token_address": string;
  "volume_type": string;
  "min_volume"?: number | bigint | string | null;
  "max_volume"?: number | bigint | string | null;
  "sort_by"?: string | null;
  "tx_type"?: string | null;
  "source"?: string | null;
  "owner"?: string | null;
  "before_time"?: number | bigint | null;
  "after_time"?: number | bigint | null;
  "ui_amount_mode"?: string | null;
  "sort_type"?: string | null;
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
}

export interface TokenListRequest {
  "sort_by"?: string | null;
  "sort_type"?: string | null;
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "min_liquidity"?: number | bigint | string | null;
  "max_liquidity"?: number | bigint | string | null;
  "ui_amount_mode"?: string | null;
}

export interface TokenListV3Request {
  "sort_by"?: string | null;
  "sort_type"?: string | null;
  "platform_name"?: string | null;
  "min_liquidity"?: number | bigint | string | null;
  "max_liquidity"?: number | bigint | string | null;
  "min_market_cap"?: number | bigint | string | null;
  "max_market_cap"?: number | bigint | string | null;
  "min_fdv"?: number | bigint | string | null;
  "max_fdv"?: number | bigint | string | null;
  "min_recent_listing_time"?: number | bigint | null;
  "max_recent_listing_time"?: number | bigint | null;
  "min_last_trade_unix_time"?: number | bigint | null;
  "max_last_trade_unix_time"?: number | bigint | null;
  "min_holder"?: number | bigint | null;
  "min_volume_1m_usd"?: number | bigint | string | null;
  "min_volume_5m_usd"?: number | bigint | string | null;
  "min_volume_30m_usd"?: number | bigint | string | null;
  "min_volume_1h_usd"?: number | bigint | string | null;
  "min_volume_2h_usd"?: number | bigint | string | null;
  "min_volume_4h_usd"?: number | bigint | string | null;
  "min_volume_8h_usd"?: number | bigint | string | null;
  "min_volume_24h_usd"?: number | bigint | string | null;
  "min_volume_7d_usd"?: number | bigint | string | null;
  "min_volume_30d_usd"?: number | bigint | string | null;
  "min_volume_1m_change_percent"?: number | bigint | string | null;
  "min_volume_5m_change_percent"?: number | bigint | string | null;
  "min_volume_30m_change_percent"?: number | bigint | string | null;
  "min_volume_1h_change_percent"?: number | bigint | string | null;
  "min_volume_2h_change_percent"?: number | bigint | string | null;
  "min_volume_4h_change_percent"?: number | bigint | string | null;
  "min_volume_8h_change_percent"?: number | bigint | string | null;
  "min_volume_24h_change_percent"?: number | bigint | string | null;
  "min_volume_7d_change_percent"?: number | bigint | string | null;
  "min_volume_30d_change_percent"?: number | bigint | string | null;
  "min_price_change_1m_percent"?: number | bigint | string | null;
  "min_price_change_5m_percent"?: number | bigint | string | null;
  "min_price_change_30m_percent"?: number | bigint | string | null;
  "min_price_change_1h_percent"?: number | bigint | string | null;
  "min_price_change_2h_percent"?: number | bigint | string | null;
  "min_price_change_4h_percent"?: number | bigint | string | null;
  "min_price_change_8h_percent"?: number | bigint | string | null;
  "min_price_change_24h_percent"?: number | bigint | string | null;
  "min_price_change_7d_percent"?: number | bigint | string | null;
  "min_price_change_30d_percent"?: number | bigint | string | null;
  "min_trade_1m_count"?: number | bigint | null;
  "min_trade_5m_count"?: number | bigint | null;
  "min_trade_30m_count"?: number | bigint | null;
  "min_trade_1h_count"?: number | bigint | null;
  "min_trade_2h_count"?: number | bigint | null;
  "min_trade_4h_count"?: number | bigint | null;
  "min_trade_8h_count"?: number | bigint | null;
  "min_trade_24h_count"?: number | bigint | null;
  "min_trade_7d_count"?: number | bigint | null;
  "min_trade_30d_count"?: number | bigint | null;
  "min_global_fees_paid"?: number | bigint | string | null;
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "ui_amount_mode"?: string | null;
}

export interface TokenListScrollRequest {
  "limit"?: number | bigint | null;
  "scroll_id"?: string | null;
  "sort_by"?: string | null;
  "sort_type"?: string | null;
  "platform_name"?: string | null;
  "min_liquidity"?: number | bigint | string | null;
  "max_liquidity"?: number | bigint | string | null;
  "min_market_cap"?: number | bigint | string | null;
  "max_market_cap"?: number | bigint | string | null;
  "min_fdv"?: number | bigint | string | null;
  "max_fdv"?: number | bigint | string | null;
  "min_recent_listing_time"?: number | bigint | null;
  "max_recent_listing_time"?: number | bigint | null;
  "min_last_trade_unix_time"?: number | bigint | null;
  "max_last_trade_unix_time"?: number | bigint | null;
  "min_holder"?: number | bigint | null;
  "min_volume_1m_usd"?: number | bigint | string | null;
  "min_volume_5m_usd"?: number | bigint | string | null;
  "min_volume_30m_usd"?: number | bigint | string | null;
  "min_volume_1h_usd"?: number | bigint | string | null;
  "min_volume_2h_usd"?: number | bigint | string | null;
  "min_volume_4h_usd"?: number | bigint | string | null;
  "min_volume_8h_usd"?: number | bigint | string | null;
  "min_volume_24h_usd"?: number | bigint | string | null;
  "min_volume_7d_usd"?: number | bigint | string | null;
  "min_volume_30d_usd"?: number | bigint | string | null;
  "min_global_fees_paid"?: number | bigint | string | null;
  "min_volume_1m_change_percent"?: number | bigint | string | null;
  "min_volume_5m_change_percent"?: number | bigint | string | null;
  "min_volume_30m_change_percent"?: number | bigint | string | null;
  "min_volume_1h_change_percent"?: number | bigint | string | null;
  "min_volume_2h_change_percent"?: number | bigint | string | null;
  "min_volume_4h_change_percent"?: number | bigint | string | null;
  "min_volume_8h_change_percent"?: number | bigint | string | null;
  "min_volume_24h_change_percent"?: number | bigint | string | null;
  "min_volume_7d_change_percent"?: number | bigint | string | null;
  "min_volume_30d_change_percent"?: number | bigint | string | null;
  "min_price_change_1m_percent"?: number | bigint | string | null;
  "min_price_change_5m_percent"?: number | bigint | string | null;
  "min_price_change_30m_percent"?: number | bigint | string | null;
  "min_price_change_1h_percent"?: number | bigint | string | null;
  "min_price_change_2h_percent"?: number | bigint | string | null;
  "min_price_change_4h_percent"?: number | bigint | string | null;
  "min_price_change_8h_percent"?: number | bigint | string | null;
  "min_price_change_24h_percent"?: number | bigint | string | null;
  "min_price_change_7d_percent"?: number | bigint | string | null;
  "min_price_change_30d_percent"?: number | bigint | string | null;
  "min_trade_1m_count"?: number | bigint | null;
  "min_trade_5m_count"?: number | bigint | null;
  "min_trade_30m_count"?: number | bigint | null;
  "min_trade_1h_count"?: number | bigint | null;
  "min_trade_2h_count"?: number | bigint | null;
  "min_trade_4h_count"?: number | bigint | null;
  "min_trade_8h_count"?: number | bigint | null;
  "min_trade_24h_count"?: number | bigint | null;
  "min_trade_7d_count"?: number | bigint | null;
  "min_trade_30d_count"?: number | bigint | null;
  "ui_amount_mode"?: string | null;
}

export interface TokenOverviewRequest {
  "address": string;
  "frames"?: string | null;
  "includes"?: string | null;
  "ui_amount_mode"?: string | null;
}

export interface TokenMetadataRequest {
  "address": string;
}

export interface TokenMetadataMultipleRequest {
  "list_address": string;
}

export interface TokenMarketDataRequest {
  "address": string;
  "ui_amount_mode"?: string | null;
}

export interface TokenMarketDataMultipleRequest {
  "list_address": string;
  "ui_amount_mode"?: string | null;
}

export interface TokenTradeDataRequest {
  "address": string;
  "frames"?: string | null;
  "ui_amount_mode"?: string | null;
  "includes"?: string | null;
}

export interface TokenTradeDataMultipleRequest {
  "list_address": string;
  "frames"?: string | null;
  "ui_amount_mode"?: string | null;
  "includes"?: string | null;
}

export interface TokenNewListingRequest {
  "time_to"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "meme_platform_enabled"?: boolean | null;
}

export interface TokenMarketsRequest {
  "address": string;
  "sort_type"?: string | null;
  "sort_by"?: string | null;
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
}

export interface TokenSecurityRequest {
  "include"?: Array<string> | null;
  "address": string;
}

export interface TokenMintBurnRequest {
  "address": string;
  "sort_by"?: string | null;
  "sort_type"?: string | null;
  "type"?: string | null;
  "after_time"?: number | bigint | null;
  "before_time"?: number | bigint | null;
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
}

export interface TokenExitLiquidityRequest {
  "address": string;
}

export interface TokenExitLiquidityMultipleRequest {
  "list_address": string;
}

export interface TraderTradesByTimeRequest {
  "address": string;
  "offset"?: number | bigint | null;
  "limit"?: number | bigint | null;
  "tx_type"?: string | null;
  "before_time"?: number | bigint | null;
  "after_time"?: number | bigint | null;
  "ui_amount_mode"?: string | null;
}

export interface ApiResponse {
  [field: string]: unknown;
  "success": boolean;
  "data": HttpValue;
}
