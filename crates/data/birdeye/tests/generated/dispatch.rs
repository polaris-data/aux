// Generated typed fixture dispatch. Assertions live in contracts.rs.
async fn invoke(client: &BirdeyeClient<Mock>, name: &str, request: &str) -> Result<ApiResponse> {
    match name {
        "price" => client.price(serde_json::from_str(request).unwrap()).await,
        "multi_price" => {
            client
                .multi_price(serde_json::from_str(request).unwrap())
                .await
        }
        "multi_price_post" => {
            client
                .multi_price_post(serde_json::from_str(request).unwrap())
                .await
        }
        "token_trades" => {
            client
                .token_trades(serde_json::from_str(request).unwrap())
                .await
        }
        "pair_trades" => {
            client
                .pair_trades(serde_json::from_str(request).unwrap())
                .await
        }
        "token_trades_by_time" => {
            client
                .token_trades_by_time(serde_json::from_str(request).unwrap())
                .await
        }
        "pair_trades_by_time" => {
            client
                .pair_trades_by_time(serde_json::from_str(request).unwrap())
                .await
        }
        "trades_v3" => {
            client
                .trades_v3(serde_json::from_str(request).unwrap())
                .await
        }
        "token_trades_v3" => {
            client
                .token_trades_v3(serde_json::from_str(request).unwrap())
                .await
        }
        "recent_trades" => {
            client
                .recent_trades(serde_json::from_str(request).unwrap())
                .await
        }
        "ohlcv" => client.ohlcv(serde_json::from_str(request).unwrap()).await,
        "ohlcv_pair" => {
            client
                .ohlcv_pair(serde_json::from_str(request).unwrap())
                .await
        }
        "ohlcv_base_quote" => {
            client
                .ohlcv_base_quote(serde_json::from_str(request).unwrap())
                .await
        }
        "ohlcv_v3" => {
            client
                .ohlcv_v3(serde_json::from_str(request).unwrap())
                .await
        }
        "ohlcv_pair_v3" => {
            client
                .ohlcv_pair_v3(serde_json::from_str(request).unwrap())
                .await
        }
        "liquidity_ohlc_pair" => {
            client
                .liquidity_ohlc_pair(serde_json::from_str(request).unwrap())
                .await
        }
        "liquidity_ohlc_token" => {
            client
                .liquidity_ohlc_token(serde_json::from_str(request).unwrap())
                .await
        }
        "liquidity_history_token" => {
            client
                .liquidity_history_token(serde_json::from_str(request).unwrap())
                .await
        }
        "liquidity_history_pair" => {
            client
                .liquidity_history_pair(serde_json::from_str(request).unwrap())
                .await
        }
        "liquidity_latest_token" => {
            client
                .liquidity_latest_token(serde_json::from_str(request).unwrap())
                .await
        }
        "history_price" => {
            client
                .history_price(serde_json::from_str(request).unwrap())
                .await
        }
        "historical_price_unix" => {
            client
                .historical_price_unix(serde_json::from_str(request).unwrap())
                .await
        }
        "price_volume_single" => {
            client
                .price_volume_single(serde_json::from_str(request).unwrap())
                .await
        }
        "price_volume_multi" => {
            client
                .price_volume_multi(serde_json::from_str(request).unwrap())
                .await
        }
        "pair_overview" => {
            client
                .pair_overview(serde_json::from_str(request).unwrap())
                .await
        }
        "pair_overview_multiple" => {
            client
                .pair_overview_multiple(serde_json::from_str(request).unwrap())
                .await
        }
        "price_stats" => {
            client
                .price_stats(serde_json::from_str(request).unwrap())
                .await
        }
        "price_stats_multiple" => {
            client
                .price_stats_multiple(serde_json::from_str(request).unwrap())
                .await
        }
        "token_trades_by_volume" => {
            client
                .token_trades_by_volume(serde_json::from_str(request).unwrap())
                .await
        }
        "token_list" => {
            client
                .token_list(serde_json::from_str(request).unwrap())
                .await
        }
        "token_list_v3" => {
            client
                .token_list_v3(serde_json::from_str(request).unwrap())
                .await
        }
        "token_list_scroll" => {
            client
                .token_list_scroll(serde_json::from_str(request).unwrap())
                .await
        }
        "token_overview" => {
            client
                .token_overview(serde_json::from_str(request).unwrap())
                .await
        }
        "token_metadata" => {
            client
                .token_metadata(serde_json::from_str(request).unwrap())
                .await
        }
        "token_metadata_multiple" => {
            client
                .token_metadata_multiple(serde_json::from_str(request).unwrap())
                .await
        }
        "token_market_data" => {
            client
                .token_market_data(serde_json::from_str(request).unwrap())
                .await
        }
        "token_market_data_multiple" => {
            client
                .token_market_data_multiple(serde_json::from_str(request).unwrap())
                .await
        }
        "token_trade_data" => {
            client
                .token_trade_data(serde_json::from_str(request).unwrap())
                .await
        }
        "token_trade_data_multiple" => {
            client
                .token_trade_data_multiple(serde_json::from_str(request).unwrap())
                .await
        }
        "token_new_listing" => {
            client
                .token_new_listing(serde_json::from_str(request).unwrap())
                .await
        }
        "token_markets" => {
            client
                .token_markets(serde_json::from_str(request).unwrap())
                .await
        }
        "token_security" => {
            client
                .token_security(serde_json::from_str(request).unwrap())
                .await
        }
        "token_mint_burn" => {
            client
                .token_mint_burn(serde_json::from_str(request).unwrap())
                .await
        }
        "token_exit_liquidity" => {
            client
                .token_exit_liquidity(serde_json::from_str(request).unwrap())
                .await
        }
        "token_exit_liquidity_multiple" => {
            client
                .token_exit_liquidity_multiple(serde_json::from_str(request).unwrap())
                .await
        }
        "trader_trades_by_time" => {
            client
                .trader_trades_by_time(serde_json::from_str(request).unwrap())
                .await
        }
        _ => panic!("uncovered Birdeye operation {name}"),
    }
}
