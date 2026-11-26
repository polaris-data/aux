#![cfg(feature = "kuru-models")]
// Generated from src/exchanges/kuru/schema/kuru.json.
#[rustfmt::skip]
mod fixtures {
    use aux_rs::kuru_models::*;
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
        for row in rows.iter().filter(|r| r["venue"] == "kuru") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "market" => { check::<MarketRequest>(request); check::<MarketResponse>(response); },
                "markets" => { check::<MarketsRequest>(request); check::<MarketsResponse>(response); },
                "trades" => { check::<TradesRequest>(request); check::<TradesResponse>(response); },
                _ => continue,
            }
        }
    }
}
