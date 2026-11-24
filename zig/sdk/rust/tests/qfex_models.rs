#![cfg(feature = "qfex-models")]
// Generated from src/exchanges/qfex/schema/qfex.json.
#[rustfmt::skip]
mod fixtures {
    use aux_zig::qfex_models::*;
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
        for row in rows.iter().filter(|r| r["venue"] == "qfex") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "account_balances" => { check::<EmptyRequest>(request); check::<serde_json::Value>(response); },
                "contracts" => { check::<ContractsRequest>(request); check::<Contracts>(response); },
                "order_book" => { check::<OrderBookRequest>(request); check::<OrderBook>(response); },
                "positions" => { check::<EmptyRequest>(request); check::<PositionsBalance>(response); },
                "refdata" => { check::<RefdataRequest>(request); check::<Refdata>(response); },
                _ => continue,
            }
        }
    }
}
