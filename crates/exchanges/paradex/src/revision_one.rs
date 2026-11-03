//! Paradex uses the Python SNIP-12 merkle convention: a singleton tree's root
//! is its sole leaf. starknet-core 0.16 hashes that leaf with zero instead.
//! Reuse its type/primitive encoders while preserving the venue's tree rule.
use crate::Error;
use starknet_core::types::{
    Felt, TypedData,
    typed_data::{
        ElementTypeReference, Encoder, FullTypeReference, InlineTypeReference, TypeDefinition,
        Value,
    },
};
use starknet_crypto::{poseidon_hash, poseidon_hash_many};
fn invalid() -> Error {
    Error::Signing("invalid Paradex revision-1 typed data")
}
fn named(encoder: &Encoder, name: &str, value: &Value, depth: usize) -> Result<Felt, Error> {
    if depth > 128 {
        return Err(invalid());
    }
    let Some(TypeDefinition::Struct(def)) = encoder.types().get_type(name) else {
        return encoder
            .encode_value(&InlineTypeReference::Custom(name.into()), value)
            .map_err(|_| invalid());
    };
    let Value::Object(object) = value else {
        return Err(invalid());
    };
    let mut elements = vec![encoder.types().get_type_hash(name).map_err(|_| invalid())?];
    for field in &def.fields {
        elements.push(encode(
            encoder,
            &field.r#type,
            object.fields.get(&field.name).ok_or_else(invalid)?,
            depth + 1,
        )?);
    }
    Ok(poseidon_hash_many(&elements))
}
fn encode(
    encoder: &Encoder,
    kind: &FullTypeReference,
    value: &Value,
    depth: usize,
) -> Result<Felt, Error> {
    if depth > 128 {
        return Err(invalid());
    }
    match kind {
        FullTypeReference::Object(name) => named(encoder, name, value, depth + 1),
        FullTypeReference::Array(ElementTypeReference::Custom(name)) => {
            let Value::Array(xs) = value else {
                return Err(invalid());
            };
            let values = xs
                .elements
                .iter()
                .map(|v| named(encoder, name, v, depth + 1))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(poseidon_hash_many(&values))
        }
        FullTypeReference::MerkleTree(leaf_type) => {
            let Value::Array(xs) = value else {
                return Err(invalid());
            };
            if xs.elements.is_empty() {
                return Err(invalid());
            }
            let mut layer = xs
                .elements
                .iter()
                .map(|v| match leaf_type {
                    InlineTypeReference::Custom(name) => named(encoder, name, v, depth + 1),
                    _ => encoder.encode_value(leaf_type, v).map_err(|_| invalid()),
                })
                .collect::<Result<Vec<_>, _>>()?;
            while layer.len() > 1 {
                let mut next = Vec::with_capacity(layer.len().div_ceil(2));
                for pair in layer.chunks(2) {
                    let a = pair[0];
                    let b = pair.get(1).copied().unwrap_or(Felt::ZERO);
                    next.push(if a <= b {
                        poseidon_hash(a, b)
                    } else {
                        poseidon_hash(b, a)
                    });
                }
                layer = next;
            }
            Ok(layer[0])
        }
        _ => encoder.encode_value(kind, value).map_err(|_| invalid()),
    }
}
pub(crate) fn message_hash(typed: &TypedData, account: Felt) -> Result<Felt, Error> {
    let encoder = typed.encoder();
    let message = match typed.primary_type() {
        InlineTypeReference::Custom(name) => named(encoder, name, typed.message(), 0)?,
        kind => encoder
            .encode_value(kind, typed.message())
            .map_err(|_| invalid())?,
    };
    Ok(poseidon_hash_many(&[
        crate::short_string("StarkNet Message")?,
        encoder.domain().encoded_hash(),
        account,
        message,
    ]))
}
