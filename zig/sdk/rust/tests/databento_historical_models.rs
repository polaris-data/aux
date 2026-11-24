#![cfg(feature = "databento-historical-models")]
// Generated from schema/databento_historical.json.
#[rustfmt::skip]
mod fixtures {
    use aux_zig::databento_historical_models::*;
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
        for row in rows.iter().filter(|r| r["venue"] == "databento") {
            let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());
            let request = &read("request");
            let response = &read("response");
            match row["operation"].as_str().unwrap() {
                "get_billable_size" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "get_cost" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "get_dataset_condition" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "get_dataset_range" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "get_job_details" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "get_record_count" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "list_datasets" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "list_fields" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "list_files" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "list_jobs" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "list_publishers" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "list_schemas" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "list_unit_prices" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "resolve" => { check::<Params>(request); check::<serde_json::Value>(response); },
                "submit_job" => { check::<Params>(request); check::<serde_json::Value>(response); },
                _ => continue,
            }
        }
    }
}
