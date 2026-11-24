#![cfg(feature = "pyth-pro-models")]
// Generated from src/data/pyth/schema/pyth_pro.json.
#[rustfmt::skip]
mod fixtures {
    use aux_rs::pyth_pro_models::*;
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
        for row in rows.iter().filter(|r| r["venue"] == "pyth_pro") {
            if row["status"].as_u64().unwrap_or(200) >= 300 { continue; }
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &row.get("expected").cloned().unwrap_or_else(|| serde_json::json!({"status":row["status"],"data":read("response"),"headers":[]}));
            match row["operation"].as_str().unwrap() {
                "config" => { check::<ConfigRequest>(request); check::<NativeResponse>(response); },
                "guardian_set_upgrade" => { check::<GuardianSetUpgradeRequest>(request); check::<NativeResponse>(response); },
                "history" => { check::<HistoryRequest>(request); check::<NativeResponse>(response); },
                "latest_price" => { check::<LatestPriceRequest>(request); check::<NativeResponse>(response); },
                "live" => { check::<LiveRequest>(request); check::<HealthResponse>(response); },
                "price" => { check::<PriceRequest>(request); check::<NativeResponse>(response); },
                "price_at_timestamp" => { check::<PriceAtTimestampRequest>(request); check::<NativeResponse>(response); },
                "price_range" => { check::<PriceRangeRequest>(request); check::<NativeResponse>(response); },
                "publisher_stake_caps" => { check::<PublisherStakeCapsRequest>(request); check::<NativeResponse>(response); },
                "publisher_updates" => { check::<PublisherUpdatesRequest>(request); check::<NativeResponse>(response); },
                "ready" => { check::<ReadyRequest>(request); check::<HealthResponse>(response); },
                "reduce_price" => { check::<ReducePriceRequest>(request); check::<NativeResponse>(response); },
                "resolve_symbol" => { check::<ResolveSymbolRequest>(request); check::<NativeResponse>(response); },
                "search" => { check::<SearchRequest>(request); check::<NativeResponse>(response); },
                "symbols" => { check::<SymbolsRequest>(request); check::<NativeResponse>(response); },
                _ => continue,
            }
        }
    }
}
