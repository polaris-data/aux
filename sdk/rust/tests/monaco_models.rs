#![cfg(feature = "monaco-models")]
// Generated from src/exchanges/monaco/schema/monaco.json.
#[rustfmt::skip]
mod fixtures {
    use aux_rs::monaco_models::*;
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
        for row in rows.iter().filter(|r| r["venue"] == "monaco") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "balances" => { check::<BalancesRequest>(request); check::<GetBalancesResponse>(response); },
                "batch_cancel" => { check::<BatchCancelOrdersRequest>(request); check::<BatchCancelOrdersResponse>(response); },
                "batch_create" => { check::<BatchCreateParams>(request); check::<BatchCreateOrdersResponse>(response); },
                "batch_replace" => { check::<BatchReplaceParams>(request); check::<BatchReplaceOrdersResponse>(response); },
                "cancel_all" => { check::<CancelAllParams>(request); check::<BatchCancelAllResponse>(response); },
                "cancel_order" => { check::<CancelOrderRequest>(request); check::<CancelOrderResponse>(response); },
                "create_challenge" => { check::<ChallengeRequest>(request); check::<ChallengeResponse>(response); },
                "funding" => { check::<FundingRequest>(request); check::<GetFundingStateResponse>(response); },
                "funding_history" => { check::<FundingHistoryRequest>(request); check::<ListFundingHistoryResponse>(response); },
                "index_price" => { check::<IndexPriceRequest>(request); check::<GetIndexPriceResponse>(response); },
                "margin_accounts" => { check::<MarginAccountsRequest>(request); check::<ListMarginAccountsResponse>(response); },
                "mark_price" => { check::<MarkPriceRequest>(request); check::<GetMarkPriceResponse>(response); },
                "market_metadata" => { check::<MarketMetadataRequest>(request); check::<GetMarketMetadataResponse>(response); },
                "order" => { check::<OrderRequest>(request); check::<GetOrderResponse>(response); },
                "order_book" => { check::<OrderBookRequest>(request); check::<GetOrderbookResponse>(response); },
                "orders" => { check::<OrdersRequest>(request); check::<ListOrdersResponse>(response); },
                "parent_margin_account" => { check::<ParentMarginAccountRequest>(request); check::<GetMarginAccountSummaryResponse>(response); },
                "place_order" => { check::<PlaceOrderParams>(request); check::<CreateOrderResponse>(response); },
                "positions" => { check::<PositionsRequest>(request); check::<ListPositionsResponse>(response); },
                "refresh_session" => { check::<EmptyRequest>(request); check::<RefreshResponse>(response); },
                "replace_order" => { check::<ReplaceOrderParams>(request); check::<ReplaceOrderResponse>(response); },
                "revoke_session" => { check::<EmptyRequest>(request); check::<RevokeResponse>(response); },
                "trades" => { check::<TradesRequest>(request); check::<GetTradesResponse>(response); },
                "trading_pair" => { check::<TradingPairRequest>(request); check::<GetTradingPairResponse>(response); },
                "trading_pairs" => { check::<TradingPairsRequest>(request); check::<ListTradingPairsResponse>(response); },
                "verify_signature" => { check::<VerifyRequest>(request); check::<VerifyResponse>(response); },
                _ => continue,
            }
        }
    }
}
