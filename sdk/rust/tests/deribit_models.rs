#![cfg(feature = "deribit-models")]
// Generated from src/exchanges/deribit/schema/deribit.json.
#[rustfmt::skip]
mod fixtures {
    use aux_rs::deribit_models::*;
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
        for row in rows.iter().filter(|r| r["venue"] == "deribit") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "account" => { check::<AccountRequest>(request); check::<AccountResponse>(response); },
                "amend_by_label" => { check::<AmendByLabelRequest>(request); check::<AmendByLabelResponse>(response); },
                "amend_order" => { check::<AmendOrderRequest>(request); check::<AmendOrderResponse>(response); },
                "authenticate" => { check::<AuthenticateRequest>(request); check::<AuthenticateResponse>(response); },
                "buy" => { check::<BuyRequest>(request); check::<BuyResponse>(response); },
                "cancel_all_orders" => { check::<CancelAllOrdersRequest>(request); check::<CancelAllOrdersResponse>(response); },
                "cancel_by_label" => { check::<CancelByLabelRequest>(request); check::<CancelByLabelResponse>(response); },
                "cancel_currency_orders" => { check::<CancelCurrencyOrdersRequest>(request); check::<CancelCurrencyOrdersResponse>(response); },
                "cancel_currency_pair_orders" => { check::<CancelCurrencyPairOrdersRequest>(request); check::<CancelCurrencyPairOrdersResponse>(response); },
                "cancel_instrument_orders" => { check::<CancelInstrumentOrdersRequest>(request); check::<CancelInstrumentOrdersResponse>(response); },
                "cancel_kind_orders" => { check::<CancelKindOrdersRequest>(request); check::<CancelKindOrdersResponse>(response); },
                "cancel_order" => { check::<CancelOrderRequest>(request); check::<CancelOrderResponse>(response); },
                "cancel_quotes" => { check::<CancelQuotesRequest>(request); check::<CancelQuotesResponse>(response); },
                "fills" => { check::<FillsRequest>(request); check::<FillsResponse>(response); },
                "markets" => { check::<MarketsRequest>(request); check::<MarketsResponse>(response); },
                "open_orders" => { check::<OpenOrdersRequest>(request); check::<OpenOrdersResponse>(response); },
                "order_history" => { check::<OrderHistoryRequest>(request); check::<OrderHistoryResponse>(response); },
                "order_status" => { check::<OrderStatusRequest>(request); check::<OrderStatusResponse>(response); },
                "positions" => { check::<PositionsRequest>(request); check::<PositionsResponse>(response); },
                "sell" => { check::<SellRequest>(request); check::<SellResponse>(response); },
                "ticker" => { check::<TickerRequest>(request); check::<TickerResponse>(response); },
                "trades" => { check::<TradesRequest>(request); check::<TradesResponse>(response); },
                _ => continue,
            }
        }
    }
}
