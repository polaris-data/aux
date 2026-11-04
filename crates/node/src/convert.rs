//! Owned values cross the worker boundary; no JavaScript handles enter Tokio tasks.
use crate::prelude::Result;
use serde::Serialize;
pub type Input = serde_json::Value;
pub type Output = serde_value::Value;
pub fn parse<T: serde::de::DeserializeOwned>(value: &Input) -> Result<T> {
    serde_json::from_value(value.clone()).map_err(|_| {
        crate::errors::ValidationError::new_err("request does not match the native Rust schema")
    })
}
pub fn from_input(value: &Input) -> Result<Input> {
    Ok(value.clone())
}
pub fn input(text: &str) -> Result<Input> {
    serde_json::from_str(text)
        .map_err(|_| crate::errors::ValidationError::new_err("invalid native request"))
}
pub fn arg<T: serde::de::DeserializeOwned>(args: &Input, key: &str) -> Result<T> {
    parse(args.get(key).unwrap_or(&Input::Null))
}
pub fn output(value: impl Serialize) -> Result<Output> {
    crate::value::to_value(value)
        .map_err(|_| crate::errors::DecodeError::new_err("unable to convert native response"))
}
pub async fn future_output<T: Serialize>(
    future: impl Future<Output = Result<T>>,
) -> Result<Output> {
    output(future.await?)
}
/// Every node is tagged, so arbitrary exchange fields cannot collide with integer tags.
pub fn encode(value: impl Serialize) -> Result<String> {
    let value = output(value)?;
    serde_json::to_string(&wire(value)?)
        .map_err(|_| crate::errors::DecodeError::new_err("unable to encode native response"))
}
fn wire(v: Output) -> Result<Input> {
    use serde_json::json;
    use serde_value::Value::*;
    Ok(match v {
        Unit | Option(None) => json!(["null"]),
        Option(Some(v)) => return wire(*v),
        Newtype(v) => {
            if let Seq(xs) = &*v
                && let [Bool(wide), String(text)] = xs.as_slice()
            {
                return Ok(if *wide {
                    json!(["bigint", text])
                } else if text.contains(['.', 'e', 'E']) {
                    json!(["string", text])
                } else if text
                    .parse::<i64>()
                    .is_ok_and(|n| n.unsigned_abs() <= 9_007_199_254_740_991)
                {
                    json!(["number", text.parse::<i64>().unwrap()])
                } else {
                    json!(["bigint", text])
                });
            }
            return wire(*v);
        }
        Bool(v) => json!(["boolean", v]),
        String(v) => json!(["string", v]),
        Char(v) => json!(["string", v.to_string()]),
        U8(v) => json!(["number", v]),
        U16(v) => json!(["number", v]),
        U32(v) => json!(["number", v]),
        I8(v) => json!(["number", v]),
        I16(v) => json!(["number", v]),
        I32(v) => json!(["number", v]),
        U64(v) => json!(["bigint", v.to_string()]),
        I64(v) => json!(["bigint", v.to_string()]),
        F32(v) => json!(["string", v.to_string()]),
        F64(v) => json!(["string", v.to_string()]),
        Bytes(v) => json!(["bytes", v]),
        Seq(v) => json!([
            "array",
            v.into_iter().map(wire).collect::<Result<Vec<_>>>()?
        ]),
        Map(values) => {
            let pairs = values
                .into_iter()
                .map(|(k, v)| {
                    let key = match k {
                        String(k) => k,
                        U8(k) => k.to_string(),
                        U16(k) => k.to_string(),
                        U32(k) => k.to_string(),
                        U64(k) => k.to_string(),
                        _ => return Err(crate::errors::DecodeError::new_err("invalid object key")),
                    };
                    Ok((key, wire(v)?))
                })
                .collect::<Result<Vec<_>>>()?;
            json!(["object", pairs])
        }
    })
}
