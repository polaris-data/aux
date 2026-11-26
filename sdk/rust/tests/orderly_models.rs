#![cfg(feature = "orderly-models")]
// Generated from src/exchanges/orderly/schema/orderly.json.
#[rustfmt::skip]
mod fixtures {
    use aux_rs::orderly_models::*;
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
        let rows: Vec<Value> = serde_json::from_str(include_str!("../../../tests/exchange-http.json")).unwrap();
        for row in rows.iter().filter(|r| r["venue"] == "orderly") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "account" => { check::<AccountRequest>(request); check::<AccountResponse>(response); },
                "algo_order_by_client_id" => { check::<AlgoOrderByClientIdRequest>(request); check::<AlgoOrderByClientIdResponse>(response); },
                "algo_order_status" => { check::<AlgoOrderStatusRequest>(request); check::<AlgoOrderStatusResponse>(response); },
                "algo_orders" => { check::<AlgoOrdersRequest>(request); check::<AlgoOrdersResponse>(response); },
                "amend_algo_order" => { check::<AmendAlgoOrderRequest>(request); check::<AmendAlgoOrderResponse>(response); },
                "amend_order" => { check::<AmendOrderRequest>(request); check::<AmendOrderResponse>(response); },
                "balances" => { check::<BalancesRequest>(request); check::<BalancesResponse>(response); },
                "cancel_algo_by_client_id" => { check::<CancelAlgoByClientIdRequest>(request); check::<CancelAlgoByClientIdResponse>(response); },
                "cancel_algo_order" => { check::<CancelAlgoOrderRequest>(request); check::<CancelAlgoOrderResponse>(response); },
                "cancel_all_algo_orders" => { check::<CancelAllAlgoOrdersRequest>(request); check::<CancelAllAlgoOrdersResponse>(response); },
                "cancel_all_orders" => { check::<CancelAllOrdersRequest>(request); check::<CancelAllOrdersResponse>(response); },
                "cancel_by_client_id" => { check::<CancelByClientIdRequest>(request); check::<CancelByClientIdResponse>(response); },
                "cancel_order" => { check::<CancelOrderRequest>(request); check::<CancelOrderResponse>(response); },
                "cancel_orders" => { check::<CancelOrdersRequest>(request); check::<CancelOrdersResponse>(response); },
                "cancel_orders_by_client_id" => { check::<CancelOrdersByClientIdRequest>(request); check::<CancelOrdersByClientIdResponse>(response); },
                "fills" => { check::<FillsRequest>(request); check::<FillsResponse>(response); },
                "market_summary" => { check::<MarketSummaryRequest>(request); check::<MarketSummaryResponse>(response); },
                "market_trades" => { check::<MarketTradesRequest>(request); check::<MarketTradesResponse>(response); },
                "markets" => { check::<MarketsRequest>(request); check::<MarketsResponse>(response); },
                "open_orders" => { check::<OpenOrdersRequest>(request); check::<OpenOrdersResponse>(response); },
                "order_by_client_id" => { check::<OrderByClientIdRequest>(request); check::<OrderByClientIdResponse>(response); },
                "order_history" => { check::<OrderHistoryRequest>(request); check::<OrderHistoryResponse>(response); },
                "order_status" => { check::<OrderStatusRequest>(request); check::<OrderStatusResponse>(response); },
                "place_algo_order" => { check::<PlaceAlgoOrderRequest>(request); check::<PlaceAlgoOrderResponse>(response); },
                "place_order" => { check::<PlaceOrderRequest>(request); check::<PlaceOrderResponse>(response); },
                "place_orders" => { check::<PlaceOrdersRequest>(request); check::<PlaceOrdersResponse>(response); },
                "positions" => { check::<PositionsRequest>(request); check::<PositionsResponse>(response); },
                "ticker" => { check::<TickerRequest>(request); check::<TickerResponse>(response); },
                "trades" => { check::<TradesRequest>(request); check::<TradesResponse>(response); },
                _ => continue,
            }
        }
    }
}
