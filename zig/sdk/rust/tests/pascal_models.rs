#![cfg(feature = "pascal-models")]
// Generated from src/exchanges/pascal/schema/pascal.json.
#[rustfmt::skip]
mod fixtures {
    use aux_zig::pascal_models::*;
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
        let rows: Vec<Value> = serde_json::from_str(include_str!("../../../tests/prediction-http.json")).unwrap();
        for row in rows.iter().filter(|r| r["venue"] == "pascal") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "read.account_bio" => { check::<AccountBioParams>(request); check::<AccountBioResponse>(response); },
                "read.account_state_snapshot" => { check::<AccountStateSnapshotParams>(request); check::<AccountStateSnapshotResponse>(response); },
                "read.batch_mark_price_candles" => { check::<BatchMarkPriceCandlesParams>(request); check::<BatchMarkPriceCandlesResponse>(response); },
                "read.batch_trades" => { check::<BatchTradesParams>(request); check::<BatchTradesResponse>(response); },
                "read.builder_stats" => { check::<BuilderStatsParams>(request); check::<BuilderStatsResponse>(response); },
                "read.closed_position_history" => { check::<ClosedPositionHistoryParams>(request); check::<ClosedPositionHistoryResponse>(response); },
                "read.deposit_address" => { check::<DepositAddressParams>(request); check::<DepositAddressResponse>(response); },
                "read.fill_history" => { check::<FillHistoryParams>(request); check::<FillHistoryResponse>(response); },
                "read.list_markets" => { check::<ListMarketsParams>(request); check::<ListMarketsResponse>(response); },
                "read.mark_price_candles" => { check::<MarkPriceCandlesParams>(request); check::<MarkPriceCandlesResponse>(response); },
                "read.market_pn_l_movers" => { check::<MarketPnLMoversParams>(request); check::<MarketPnLMoversResponse>(response); },
                "read.market_search" => { check::<MarketSearchParams>(request); check::<MarketSearchResponse>(response); },
                "read.market_text_search" => { check::<MarketTextSearchParams>(request); check::<MarketTextSearchResponse>(response); },
                "read.order_books" => { check::<OrderBooksParams>(request); check::<OrderBooksResponse>(response); },
                "read.pn_l_history" => { check::<PnLHistoryParams>(request); check::<PnLHistoryResponse>(response); },
                "read.position_resolution_history" => { check::<PositionResolutionHistoryParams>(request); check::<PositionResolutionHistoryResponse>(response); },
                "read.referral_code" => { check::<ReferralCodeParams>(request); check::<ReferralCodeResponse>(response); },
                "read.referrals" => { check::<ReferralsParams>(request); check::<ReferralsResponse>(response); },
                "read.resolved_markets" => { check::<ResolvedMarketsParams>(request); check::<ResolvedMarketsResponse>(response); },
                "read.server_time" => { check::<ServerTimeParams>(request); check::<ServerTimeResponse>(response); },
                "read.taker_discount_terms" => { check::<TakerDiscountTermsParams>(request); check::<TakerDiscountTermsResponse>(response); },
                "read.top_holders" => { check::<TopHoldersParams>(request); check::<TopHoldersResponse>(response); },
                "read.trade_candles" => { check::<TradeCandlesParams>(request); check::<TradeCandlesResponse>(response); },
                "read.trades" => { check::<TradesParams>(request); check::<TradesResponse>(response); },
                "read.trading_key_accounts" => { check::<TradingKeyAccountsParams>(request); check::<TradingKeyAccountsResponse>(response); },
                "read.trading_keys" => { check::<TradingKeysParams>(request); check::<TradingKeysResponse>(response); },
                "read.transfer_history" => { check::<TransferHistoryParams>(request); check::<TransferHistoryResponse>(response); },
                "read.usernames" => { check::<UsernamesParams>(request); check::<UsernamesResponse>(response); },
                "write.apply_a_referral_code" => { check::<ApplyAReferralCodeParams>(request); check::<ApplyAReferralCodeResponse>(response); },
                "write.cancel_orders" => { check::<CancelOrdersParams>(request); check::<Vec<CancelOrdersResponseItem>>(response); },
                "write.claim_a_custom_referral_code" => { check::<ClaimACustomReferralCodeParams>(request); check::<ClaimACustomReferralCodeResponse>(response); },
                "write.create_a_trading_key" => { check::<CreateATradingKeyParams>(request); check::<CreateATradingKeyResponse>(response); },
                "write.generate_a_referral_code" => { check::<GenerateAReferralCodeParams>(request); check::<GenerateAReferralCodeResponse>(response); },
                "write.mass_cancel_orders" => { check::<MassCancelOrdersParams>(request); check::<Vec<MassCancelOrdersResponseItem>>(response); },
                "write.place_and_replace_orders" => { check::<PlaceAndReplaceOrdersParams>(request); check::<Vec<PlaceAndReplaceOrdersResponseItem>>(response); },
                "write.register_a_deposit_address" => { check::<RegisterADepositAddressParams>(request); check::<RegisterADepositAddressResponse>(response); },
                "write.revoke_a_trading_key" => { check::<RevokeATradingKeyParams>(request); check::<RevokeATradingKeyResponse>(response); },
                "write.set_a_username" => { check::<SetAUsernameParams>(request); check::<SetAUsernameResponse>(response); },
                "write.set_an_account_bio" => { check::<SetAnAccountBioParams>(request); check::<SetAnAccountBioResponse>(response); },
                "write.withdraw_collateral" => { check::<WithdrawCollateralParams>(request); check::<WithdrawCollateralResponse>(response); },
                _ => continue,
            }
        }
    }
}
