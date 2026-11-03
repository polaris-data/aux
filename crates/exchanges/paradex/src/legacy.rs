//! Paradex's revision-0 domain orders name, chainId, version. The generic
//! starknet-core encoder requires a different canonical domain order, so the
//! native legacy codec preserves the published field order explicitly.
use crate::{Error, felt, short_string};
use serde_json::Value;
use starknet_core::{crypto::compute_hash_on_elements, types::Felt, utils::starknet_keccak};
use std::collections::BTreeSet;
fn invalid() -> Error {
    Error::Signing("invalid Paradex legacy typed data")
}
fn fields<'a>(types: &'a Value, name: &str) -> Result<&'a Vec<Value>, Error> {
    types
        .get(name)
        .and_then(Value::as_array)
        .ok_or_else(invalid)
}
fn type_signature(types: &Value, name: &str) -> Result<String, Error> {
    let mut dependencies = BTreeSet::new();
    let mut pending = vec![name.to_owned()];
    while let Some(current) = pending.pop() {
        if dependencies.len() > 128 {
            return Err(invalid());
        }
        for field in fields(types, &current)? {
            let child = field["type"]
                .as_str()
                .ok_or_else(invalid)?
                .trim_end_matches('*');
            if types.get(child).is_some() && child != name && dependencies.insert(child.to_owned())
            {
                pending.push(child.into());
            }
        }
    }
    let encode = |name: &str| -> Result<String, Error> {
        let params = fields(types, name)?
            .iter()
            .map(|field| {
                Ok(format!(
                    "{}:{}",
                    field["name"].as_str().ok_or_else(invalid)?,
                    field["type"].as_str().ok_or_else(invalid)?
                ))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(format!("{name}({})", params.join(",")))
    };
    let mut signature = encode(name)?;
    for dependency in dependencies {
        signature.push_str(&encode(&dependency)?);
    }
    Ok(signature)
}
fn encode(types: &Value, kind: &str, value: &Value, depth: usize) -> Result<Felt, Error> {
    if depth > 128 {
        return Err(invalid());
    }
    if let Some(inner) = kind.strip_suffix('*') {
        let values = value
            .as_array()
            .ok_or_else(invalid)?
            .iter()
            .map(|v| encode(types, inner, v, depth + 1))
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(compute_hash_on_elements(&values));
    }
    if types.get(kind).is_some() {
        let mut values = vec![starknet_keccak(type_signature(types, kind)?.as_bytes())];
        for field in fields(types, kind)? {
            let name = field["name"].as_str().ok_or_else(invalid)?;
            let kind = field["type"].as_str().ok_or_else(invalid)?;
            values.push(encode(
                types,
                kind,
                value.get(name).ok_or_else(invalid)?,
                depth + 1,
            )?);
        }
        return Ok(compute_hash_on_elements(&values));
    }
    if kind != "felt" && kind != "shortstring" {
        return Err(invalid());
    }
    match value {
        Value::String(s) => {
            if s.starts_with("0x") || !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) {
                felt(s)
            } else {
                short_string(s)
            }
        }
        Value::Number(n) => felt(&n.to_string()),
        _ => Err(invalid()),
    }
}
pub(crate) fn message_hash(account: &str, data: &Value) -> Result<Felt, Error> {
    let types = data.get("types").ok_or_else(invalid)?;
    let primary = data["primaryType"].as_str().ok_or_else(invalid)?;
    Ok(compute_hash_on_elements(&[
        short_string("StarkNet Message")?,
        encode(
            types,
            "StarkNetDomain",
            data.get("domain").ok_or_else(invalid)?,
            0,
        )?,
        felt(account)?,
        encode(types, primary, data.get("message").ok_or_else(invalid)?, 0)?,
    ]))
}
