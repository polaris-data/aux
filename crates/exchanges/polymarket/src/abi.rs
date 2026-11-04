//! Exact, schema-checked ABI access for native mappings at the Python boundary.
use crate::Error;
use alloy_dyn_abi::{DynSolType, DynSolValue, EventExt, FunctionExt, JsonAbiExt, Specifier};
use alloy_json_abi::{Function, JsonAbi, Param};
use alloy_primitives::{Address, B256, Bytes, I256, U256};
use serde_json::{Value, json};

pub fn contract_abi(contract: &str) -> Result<JsonAbi, Error> {
    let inventory: Value = serde_json::from_str(include_str!("../schema/contracts.json"))?;
    let signatures: Vec<&str> = inventory
        .as_array()
        .ok_or(Error::InvalidRequest("invalid contract inventory"))?
        .iter()
        .filter(|r| r["interface"] == contract)
        .filter_map(|r| r["signature"].as_str())
        .collect();
    if signatures.is_empty() {
        return Err(Error::InvalidRequest(
            "unknown Polymarket contract interface",
        ));
    }
    // Alloy's human-readable parser accepts tuple types but not names inside tuples.
    // Keep the published component names in the JSON ABI for mapping inputs.
    let signatures: Vec<String> = signatures
        .iter()
        .map(|s| {
            s.replace(
                "(uint8 typeCode, address to, uint256 value, bytes data)[]",
                "(uint8,address,uint256,bytes)[]",
            )
        })
        .collect();
    let mut abi = JsonAbi::parse(signatures.iter().map(String::as_str))
        .map_err(|e| Error::Abi(e.to_string()))?;
    if contract == "IProxyFactory" {
        let input = &mut abi
            .functions
            .get_mut("proxy")
            .ok_or(Error::InvalidRequest("missing proxy interface"))?[0]
            .inputs[0];
        for (component, name) in input
            .components
            .iter_mut()
            .zip(["typeCode", "to", "value", "data"])
        {
            component.name = name.into();
        }
    }
    Ok(abi)
}
fn function(abi: &JsonAbi, name: &str) -> Result<Function, Error> {
    let mut matches = abi
        .functions
        .values()
        .flatten()
        .filter(|f| f.name == name || f.signature() == name);
    let f = matches
        .next()
        .ok_or(Error::InvalidRequest("unknown native contract function"))?
        .clone();
    if matches.next().is_some() {
        return Err(Error::InvalidRequest(
            "overloaded function requires its full Solidity signature",
        ));
    }
    Ok(f)
}
fn text(v: &Value) -> Result<String, Error> {
    match v {
        Value::String(s) => Ok(s.clone()),
        Value::Number(n) if !n.to_string().contains(['.', 'e', 'E']) => Ok(n.to_string()),
        _ => Err(Error::InvalidRequest(
            "exact integer or native string required",
        )),
    }
}
fn value(ty: &DynSolType, v: &Value, depth: usize) -> Result<DynSolValue, Error> {
    if depth > 64 {
        return Err(Error::InvalidRequest("ABI nesting exceeds limit"));
    }
    let invalid = || Error::InvalidRequest("value does not match native Solidity type");
    Ok(match ty {
        DynSolType::Bool => DynSolValue::Bool(v.as_bool().ok_or_else(invalid)?),
        DynSolType::Uint(bits) => {
            let n: U256 = text(v)?.parse().map_err(|_| invalid())?;
            if n.bit_len() > *bits {
                return Err(invalid());
            }
            DynSolValue::Uint(n, *bits)
        }
        DynSolType::Int(bits) => {
            let n: I256 = text(v)?.parse().map_err(|_| invalid())?;
            if n.bits() as usize > *bits {
                return Err(invalid());
            }
            DynSolValue::Int(n, *bits)
        }
        DynSolType::Address => {
            DynSolValue::Address(text(v)?.parse::<Address>().map_err(|_| invalid())?)
        }
        DynSolType::Bytes => {
            DynSolValue::Bytes(alloy_primitives::hex::decode(text(v)?).map_err(|_| invalid())?)
        }
        DynSolType::FixedBytes(n) => {
            let b = alloy_primitives::hex::decode(text(v)?).map_err(|_| invalid())?;
            if b.len() != *n {
                return Err(invalid());
            }
            let mut full = [0u8; 32];
            full[..*n].copy_from_slice(&b);
            DynSolValue::FixedBytes(B256::from(full), *n)
        }
        DynSolType::String => DynSolValue::String(v.as_str().ok_or_else(invalid)?.into()),
        DynSolType::Function => DynSolValue::Function(text(v)?.parse().map_err(|_| invalid())?),
        DynSolType::Array(item) => DynSolValue::Array(
            v.as_array()
                .ok_or_else(invalid)?
                .iter()
                .map(|v| value(item, v, depth + 1))
                .collect::<Result<_, _>>()?,
        ),
        DynSolType::FixedArray(item, n) => {
            let values = v.as_array().ok_or_else(invalid)?;
            if values.len() != *n {
                return Err(invalid());
            }
            DynSolValue::FixedArray(
                values
                    .iter()
                    .map(|v| value(item, v, depth + 1))
                    .collect::<Result<_, _>>()?,
            )
        }
        DynSolType::CustomStruct {
            name,
            prop_names,
            tuple,
        } => {
            let values = tuple
                .iter()
                .enumerate()
                .map(|(i, ty)| {
                    let input = if let Some(array) = v.as_array() {
                        array.get(i)
                    } else {
                        prop_names.get(i).and_then(|key| v.get(key))
                    }
                    .ok_or_else(invalid)?;
                    value(ty, input, depth + 1)
                })
                .collect::<Result<Vec<_>, _>>()?;
            DynSolValue::CustomStruct {
                name: name.clone(),
                prop_names: prop_names.clone(),
                tuple: values,
            }
        }
        DynSolType::Tuple(items) => {
            let values = v.as_array().ok_or_else(invalid)?;
            if values.len() != items.len() {
                return Err(invalid());
            }
            DynSolValue::Tuple(
                items
                    .iter()
                    .zip(values)
                    .map(|(ty, v)| value(ty, v, depth + 1))
                    .collect::<Result<_, _>>()?,
            )
        }
    })
}
/// Tuple mappings may use ABI component names; positional arrays also preserve overloaded tuples.
fn param_value(param: &Param, v: &Value) -> Result<DynSolValue, Error> {
    let mut v = v.clone();
    normalize_tuple(param, &mut v)?;
    let ty = param.resolve().map_err(|e| Error::Abi(e.to_string()))?;
    value(&ty, &v, 0)
}
fn normalize_tuple(param: &Param, v: &mut Value) -> Result<(), Error> {
    if param.components.is_empty() {
        return Ok(());
    }
    if param.ty.ends_with(']') {
        let arr = v
            .as_array_mut()
            .ok_or(Error::InvalidRequest("tuple array required"))?;
        let mut inner = param.clone();
        inner.ty = inner.ty[..inner
            .ty
            .rfind('[')
            .ok_or(Error::InvalidRequest("invalid array type"))?]
            .into();
        for item in arr {
            normalize_tuple(&inner, item)?;
        }
        return Ok(());
    }
    if let Value::Object(map) = v {
        if map.len() != param.components.len()
            || param
                .components
                .iter()
                .any(|p| p.name.is_empty() || !map.contains_key(&p.name))
        {
            return Err(Error::InvalidRequest(
                "tuple mapping must contain all ABI component names",
            ));
        }
        *v = Value::Array(
            param
                .components
                .iter()
                .map(|p| map[&p.name].clone())
                .collect(),
        );
    }
    let values = v.as_array_mut().ok_or(Error::InvalidRequest(
        "tuple requires native mapping or array",
    ))?;
    if values.len() != param.components.len() {
        return Err(Error::InvalidRequest("tuple arity mismatch"));
    }
    for (p, v) in param.components.iter().zip(values) {
        normalize_tuple(p, v)?;
    }
    Ok(())
}
pub fn encode_contract_call(contract: &str, name: &str, args: &Value) -> Result<Bytes, Error> {
    let abi = contract_abi(contract)?;
    let f = function(&abi, name)?;
    let values = if let Some(map) = args.as_object() {
        if map.len() != f.inputs.len()
            || f.inputs
                .iter()
                .any(|p| p.name.is_empty() || !map.contains_key(&p.name))
        {
            return Err(Error::InvalidRequest(
                "arguments must contain all ABI parameter names",
            ));
        }
        f.inputs
            .iter()
            .map(|p| param_value(p, &map[&p.name]))
            .collect::<Result<Vec<_>, _>>()?
    } else {
        let values = args.as_array().ok_or(Error::InvalidRequest(
            "contract arguments require an array or mapping",
        ))?;
        if values.len() != f.inputs.len() {
            return Err(Error::InvalidRequest("contract argument count mismatch"));
        }
        f.inputs
            .iter()
            .zip(values)
            .map(|(p, v)| param_value(p, v))
            .collect::<Result<Vec<_>, _>>()?
    };
    f.abi_encode_input(&values)
        .map(Bytes::from)
        .map_err(|e| Error::Abi(e.to_string()))
}
fn json_value(v: DynSolValue) -> Value {
    match v {
        DynSolValue::Bool(b) => json!(b),
        DynSolValue::Uint(n, _) => serde_json::from_str(&n.to_string()).expect("integer JSON"),
        DynSolValue::Int(n, _) => serde_json::from_str(&n.to_string()).expect("integer JSON"),
        DynSolValue::Address(a) => json!(a),
        DynSolValue::Bytes(b) => json!(alloy_primitives::hex::encode_prefixed(b)),
        DynSolValue::FixedBytes(b, n) => json!(alloy_primitives::hex::encode_prefixed(&b[..n])),
        DynSolValue::String(s) => json!(s),
        DynSolValue::Function(f) => json!(f.to_string()),
        DynSolValue::CustomStruct { tuple: a, .. }
        | DynSolValue::Array(a)
        | DynSolValue::FixedArray(a)
        | DynSolValue::Tuple(a) => Value::Array(a.into_iter().map(json_value).collect()),
    }
}
pub fn decode_contract_output(contract: &str, name: &str, bytes: &[u8]) -> Result<Value, Error> {
    let abi = contract_abi(contract)?;
    let f = function(&abi, name)?;
    let values = f
        .abi_decode_output(bytes)
        .map_err(|e| Error::Abi(e.to_string()))?;
    Ok(Value::Array(values.into_iter().map(json_value).collect()))
}
pub fn decode_contract_input(contract: &str, name: &str, bytes: &[u8]) -> Result<Value, Error> {
    let abi = contract_abi(contract)?;
    let f = function(&abi, name)?;
    if !bytes.starts_with(f.selector().as_slice()) {
        return Err(Error::InvalidRequest("native function selector mismatch"));
    }
    let values = f
        .abi_decode_input(&bytes[4..])
        .map_err(|e| Error::Abi(e.to_string()))?;
    if f.abi_encode_input(&values)
        .map_err(|e| Error::Abi(e.to_string()))?
        != bytes
    {
        return Err(Error::InvalidRequest(
            "noncanonical or trailing ABI input bytes",
        ));
    }
    Ok(Value::Array(values.into_iter().map(json_value).collect()))
}
pub fn decode_contract_event(
    contract: &str,
    name: &str,
    topics: &[B256],
    data: &[u8],
) -> Result<Value, Error> {
    let abi = contract_abi(contract)?;
    let mut events = abi
        .events
        .values()
        .flatten()
        .filter(|e| e.name == name || e.signature() == name);
    let event = events
        .next()
        .ok_or(Error::InvalidRequest("unknown native event"))?;
    if events.next().is_some() {
        return Err(Error::InvalidRequest(
            "overloaded event requires full signature",
        ));
    }
    let result = event
        .decode_log_parts(topics.iter().copied(), data)
        .map_err(|e| Error::Abi(e.to_string()))?;
    Ok(
        json!({"name":event.name,"indexed":result.indexed.into_iter().map(json_value).collect::<Vec<_>>(),"body":result.body.into_iter().map(json_value).collect::<Vec<_>>()}),
    )
}
pub fn decode_contract_error(contract: &str, bytes: &[u8]) -> Result<Value, Error> {
    let abi = contract_abi(contract)?;
    for e in abi.errors.values().flatten() {
        if bytes.starts_with(&e.selector()[..]) {
            let values = e
                .abi_decode_input(&bytes[4..])
                .map_err(|e| Error::Abi(e.to_string()))?;
            return Ok(
                json!({"name":e.name,"arguments":values.into_iter().map(json_value).collect::<Vec<_>>()}),
            );
        }
    }
    Err(Error::InvalidRequest("unknown native error selector"))
}
