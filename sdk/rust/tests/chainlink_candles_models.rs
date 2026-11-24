#![cfg(feature = "chainlink-candles-models")]
// Generated from src/data/chainlink/schema/chainlink_candles.json.
#[rustfmt::skip]
mod fixtures {
    use aux_rs::chainlink_candles_models::*;
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
        let mut rows: Vec<Value> = serde_json::from_str(include_str!("../../../tests/data-http.json")).unwrap();
        rows.extend(serde_json::from_str::<Vec<Value>>(include_str!("../../../tests/text-http.json")).unwrap());
        for row in rows.iter().filter(|r| r["venue"] == "chainlink_candles") {
            if row["status"].as_u64().unwrap_or(200) >= 300 { continue; }
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &row.get("expected").cloned().unwrap_or_else(|| serde_json::json!({"status":row["status"],"data":read("response"),"headers":[]}));
            match row["operation"].as_str().unwrap() {
                "authorize" => { check::<AuthorizeRequest>(request); check::<NativeResponse>(response); },
                "candle_history" => { check::<CandleHistoryRequest>(request); check::<NativeResponse>(response); },
                "candle_history_rows" => { check::<CandleHistoryRowsRequest>(request); check::<NativeResponse>(response); },
                "groups" => { check::<GroupsRequest>(request); check::<NativeResponse>(response); },
                "symbol_info" => { check::<SymbolInfoRequest>(request); check::<NativeResponse>(response); },
                _ => continue,
            }
        }
    }
}
