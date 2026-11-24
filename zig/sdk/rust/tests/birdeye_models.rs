#![cfg(feature = "birdeye-models")]
// Generated from src/data/birdeye/schema/birdeye.json.
#[rustfmt::skip]
mod fixtures {
    use aux_zig::birdeye_models::*;
    use serde::{de::DeserializeOwned, Serialize};
    use serde_json::Value;

    fn wire_eq(actual: &Value, wire: &Value) {
        match (actual, wire) {
            (Value::Object(a),Value::Object(b)) => {
                for (k,v) in b { if v.is_null() && !a.contains_key(k) { continue; } wire_eq(&a[k],v); }
                assert!(a.keys().all(|k| b.contains_key(k)));
            },
            (Value::Array(a),Value::Array(b)) => { assert_eq!(a.len(),b.len()); for (x,y) in a.iter().zip(b) { wire_eq(x,y); } },
            _ => assert_eq!(actual,wire),
        }
    }
    fn check<T: DeserializeOwned + Serialize>(v: &Value) {
        let model: T = serde_json::from_value(v.clone()).unwrap();
        wire_eq(&serde_json::to_value(model).unwrap(), v);
    }

    #[test]
    fn models_round_trip_exposed_rest_fixtures() {
        let rows: Vec<Value> = serde_json::from_str(include_str!("../../../tests/data-http.json")).unwrap();
        for row in rows.iter().filter(|r| r["venue"] == "birdeye") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "historical_price_unix" => { check::<HistoricalPriceUnixRequest>(request); check::<ApiResponse>(response); },
                "history_price" => { check::<HistoryPriceRequest>(request); check::<ApiResponse>(response); },
                "liquidity_history_pair" => { check::<LiquidityHistoryPairRequest>(request); check::<ApiResponse>(response); },
                "liquidity_history_token" => { check::<LiquidityHistoryTokenRequest>(request); check::<ApiResponse>(response); },
                "liquidity_latest_token" => { check::<LiquidityLatestTokenRequest>(request); check::<ApiResponse>(response); },
                "liquidity_ohlc_pair" => { check::<LiquidityOhlcPairRequest>(request); check::<ApiResponse>(response); },
                "liquidity_ohlc_token" => { check::<LiquidityOhlcTokenRequest>(request); check::<ApiResponse>(response); },
                "multi_price" => { check::<MultiPriceRequest>(request); check::<ApiResponse>(response); },
                "multi_price_post" => { check::<MultiPricePostRequest>(request); check::<ApiResponse>(response); },
                "ohlcv" => { check::<OhlcvRequest>(request); check::<ApiResponse>(response); },
                "ohlcv_base_quote" => { check::<OhlcvBaseQuoteRequest>(request); check::<ApiResponse>(response); },
                "ohlcv_pair" => { check::<OhlcvPairRequest>(request); check::<ApiResponse>(response); },
                "ohlcv_pair_v3" => { check::<OhlcvPairV3Request>(request); check::<ApiResponse>(response); },
                "ohlcv_v3" => { check::<OhlcvV3Request>(request); check::<ApiResponse>(response); },
                "pair_overview" => { check::<PairOverviewRequest>(request); check::<ApiResponse>(response); },
                "pair_overview_multiple" => { check::<PairOverviewMultipleRequest>(request); check::<ApiResponse>(response); },
                "pair_trades" => { check::<PairTradesRequest>(request); check::<ApiResponse>(response); },
                "pair_trades_by_time" => { check::<PairTradesByTimeRequest>(request); check::<ApiResponse>(response); },
                "price" => { check::<PriceRequest>(request); check::<ApiResponse>(response); },
                "price_stats" => { check::<PriceStatsRequest>(request); check::<ApiResponse>(response); },
                "price_stats_multiple" => { check::<PriceStatsMultipleRequest>(request); check::<ApiResponse>(response); },
                "price_volume_multi" => { check::<PriceVolumeMultiRequest>(request); check::<ApiResponse>(response); },
                "price_volume_single" => { check::<PriceVolumeSingleRequest>(request); check::<ApiResponse>(response); },
                "recent_trades" => { check::<RecentTradesRequest>(request); check::<ApiResponse>(response); },
                "token_exit_liquidity" => { check::<TokenExitLiquidityRequest>(request); check::<ApiResponse>(response); },
                "token_exit_liquidity_multiple" => { check::<TokenExitLiquidityMultipleRequest>(request); check::<ApiResponse>(response); },
                "token_list" => { check::<TokenListRequest>(request); check::<ApiResponse>(response); },
                "token_list_scroll" => { check::<TokenListScrollRequest>(request); check::<ApiResponse>(response); },
                "token_list_v3" => { check::<TokenListV3Request>(request); check::<ApiResponse>(response); },
                "token_market_data" => { check::<TokenMarketDataRequest>(request); check::<ApiResponse>(response); },
                "token_market_data_multiple" => { check::<TokenMarketDataMultipleRequest>(request); check::<ApiResponse>(response); },
                "token_markets" => { check::<TokenMarketsRequest>(request); check::<ApiResponse>(response); },
                "token_metadata" => { check::<TokenMetadataRequest>(request); check::<ApiResponse>(response); },
                "token_metadata_multiple" => { check::<TokenMetadataMultipleRequest>(request); check::<ApiResponse>(response); },
                "token_mint_burn" => { check::<TokenMintBurnRequest>(request); check::<ApiResponse>(response); },
                "token_new_listing" => { check::<TokenNewListingRequest>(request); check::<ApiResponse>(response); },
                "token_overview" => { check::<TokenOverviewRequest>(request); check::<ApiResponse>(response); },
                "token_security" => { check::<TokenSecurityRequest>(request); check::<ApiResponse>(response); },
                "token_trade_data" => { check::<TokenTradeDataRequest>(request); check::<ApiResponse>(response); },
                "token_trade_data_multiple" => { check::<TokenTradeDataMultipleRequest>(request); check::<ApiResponse>(response); },
                "token_trades" => { check::<TokenTradesRequest>(request); check::<ApiResponse>(response); },
                "token_trades_by_time" => { check::<TokenTradesByTimeRequest>(request); check::<ApiResponse>(response); },
                "token_trades_by_volume" => { check::<TokenTradesByVolumeRequest>(request); check::<ApiResponse>(response); },
                "token_trades_v3" => { check::<TokenTradesV3Request>(request); check::<ApiResponse>(response); },
                "trader_trades_by_time" => { check::<TraderTradesByTimeRequest>(request); check::<ApiResponse>(response); },
                "trades_v3" => { check::<TradesV3Request>(request); check::<ApiResponse>(response); },
                _ => continue,
            }
        }
    }
}
