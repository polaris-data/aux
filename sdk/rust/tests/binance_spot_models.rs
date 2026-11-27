#![cfg(feature = "binance-spot-models")]
// Generated from src/exchanges/binance_spot/schema/binance_spot.json.
#[rustfmt::skip]
mod fixtures {
    use aux_rs::binance_spot_models::*;
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
        for row in rows.iter().filter(|r| r["venue"] == "binance_spot") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "account" => { check::<AccountRequest>(request); check::<AccountResponse>(response); },
                "agg_trades" => { check::<AggTradesRequest>(request); check::<AggTradesResponse>(response); },
                "amend_order" => { check::<AmendOrderRequest>(request); check::<AmendOrderResponse>(response); },
                "cancel_all_orders" => { check::<CancelAllOrdersRequest>(request); check::<CancelAllOrdersResponse>(response); },
                "cancel_order" => { check::<CancelOrderRequest>(request); check::<CancelOrderResponse>(response); },
                "cancel_order_list" => { check::<CancelOrderListRequest>(request); check::<CancelOrderListResponse>(response); },
                "cancel_replace_order" => { check::<CancelReplaceOrderRequest>(request); check::<CancelReplaceOrderResponse>(response); },
                "depth" => { check::<DepthRequest>(request); check::<DepthResponse>(response); },
                "fills" => { check::<FillsRequest>(request); check::<FillsResponse>(response); },
                "markets" => { check::<MarketsRequest>(request); check::<MarketsResponse>(response); },
                "open_order_lists" => { check::<OpenOrderListsRequest>(request); check::<OpenOrderListsResponse>(response); },
                "open_orders" => { check::<OpenOrdersRequest>(request); check::<OpenOrdersResponse>(response); },
                "order_history" => { check::<OrderHistoryRequest>(request); check::<OrderHistoryResponse>(response); },
                "order_list_history" => { check::<OrderListHistoryRequest>(request); check::<OrderListHistoryResponse>(response); },
                "order_list_status" => { check::<OrderListStatusRequest>(request); check::<OrderListStatusResponse>(response); },
                "order_status" => { check::<OrderStatusRequest>(request); check::<OrderStatusResponse>(response); },
                "place_oco" => { check::<PlaceOcoRequest>(request); check::<PlaceOcoResponse>(response); },
                "place_opo" => { check::<PlaceOpoRequest>(request); check::<PlaceOpoResponse>(response); },
                "place_opoco" => { check::<PlaceOpocoRequest>(request); check::<PlaceOpocoResponse>(response); },
                "place_order" => { check::<PlaceOrderRequest>(request); check::<PlaceOrderResponse>(response); },
                "place_oto" => { check::<PlaceOtoRequest>(request); check::<PlaceOtoResponse>(response); },
                "place_otoco" => { check::<PlaceOtocoRequest>(request); check::<PlaceOtocoResponse>(response); },
                "ticker" => { check::<TickerRequest>(request); check::<TickerResponse>(response); },
                "trades" => { check::<TradesRequest>(request); check::<TradesResponse>(response); },
                _ => continue,
            }
        }
    }
}
