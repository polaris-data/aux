//! A Serde value serializer preserving integer widths and arbitrary JSON numbers.
use serde::ser::Error as _;
use serde::{Serialize, ser::*};
use serde_value::{SerializerError as Error, Value};
use std::collections::BTreeMap;
pub fn to_value<T: Serialize>(value: T) -> Result<Value, Error> {
    let name = std::any::type_name::<T>().trim_start_matches('&');
    if name == "serde_json::value::Value" || name == "serde_json::number::Number" {
        return serde_json::to_value(value).map(raw).map_err(Error::custom);
    }
    value.serialize(Serializer)
}
fn raw(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Unit,
        serde_json::Value::Bool(v) => Value::Bool(v),
        serde_json::Value::String(v) => Value::String(v),
        serde_json::Value::Number(v) => Value::Newtype(Box::new(Value::Seq(vec![
            Value::Bool(false),
            Value::String(v.to_string()),
        ]))),
        serde_json::Value::Array(v) => Value::Seq(v.into_iter().map(raw).collect()),
        serde_json::Value::Object(v) => Value::Map(
            v.into_iter()
                .map(|(k, v)| (Value::String(k), raw(v)))
                .collect(),
        ),
    }
}
struct Serializer;
macro_rules! primitive {($($fn:ident($t:ty) => $variant:ident),*)=>{$(fn $fn(self,v:$t)->Result<Value,Error>{Ok(Value::$variant(v))})*}}
impl serde::Serializer for Serializer {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = Sequence;
    type SerializeTuple = Sequence;
    type SerializeTupleStruct = Sequence;
    type SerializeTupleVariant = Sequence;
    type SerializeMap = Mapping;
    type SerializeStruct = Mapping;
    type SerializeStructVariant = Mapping;
    primitive!(serialize_bool(bool)=>Bool,serialize_u8(u8)=>U8,serialize_u16(u16)=>U16,serialize_u32(u32)=>U32,serialize_u64(u64)=>U64,serialize_i8(i8)=>I8,serialize_i16(i16)=>I16,serialize_i32(i32)=>I32,serialize_i64(i64)=>I64,serialize_f32(f32)=>F32,serialize_f64(f64)=>F64,serialize_char(char)=>Char);
    fn serialize_i128(self, v: i128) -> Result<Value, Error> {
        Ok(wide(v.to_string()))
    }
    fn serialize_u128(self, v: u128) -> Result<Value, Error> {
        Ok(wide(v.to_string()))
    }
    fn serialize_str(self, v: &str) -> Result<Value, Error> {
        Ok(Value::String(v.into()))
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Value, Error> {
        Ok(Value::Bytes(v.into()))
    }
    fn serialize_none(self) -> Result<Value, Error> {
        Ok(Value::Unit)
    }
    fn serialize_some<T: Serialize + ?Sized>(self, v: &T) -> Result<Value, Error> {
        to_value(v)
    }
    fn serialize_unit(self) -> Result<Value, Error> {
        Ok(Value::Unit)
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<Value, Error> {
        Ok(Value::Unit)
    }
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        v: &'static str,
    ) -> Result<Value, Error> {
        Ok(Value::String(v.into()))
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        v: &T,
    ) -> Result<Value, Error> {
        Ok(Value::Newtype(Box::new(to_value(v)?)))
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        _: u32,
        k: &'static str,
        v: &T,
    ) -> Result<Value, Error> {
        Ok(Value::Map(BTreeMap::from([(
            Value::String(k.into()),
            to_value(v)?,
        )])))
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Sequence, Error> {
        Ok(Sequence {
            values: Vec::with_capacity(len.unwrap_or(0)),
            tag: None,
        })
    }
    fn serialize_tuple(self, len: usize) -> Result<Sequence, Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(self, _: &'static str, len: usize) -> Result<Sequence, Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        tag: &'static str,
        len: usize,
    ) -> Result<Sequence, Error> {
        Ok(Sequence {
            values: Vec::with_capacity(len),
            tag: Some(tag),
        })
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Mapping, Error> {
        Ok(Mapping {
            values: BTreeMap::new(),
            key: None,
            tag: None,
        })
    }
    fn serialize_struct(self, _: &'static str, len: usize) -> Result<Mapping, Error> {
        self.serialize_map(Some(len))
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        tag: &'static str,
        _: usize,
    ) -> Result<Mapping, Error> {
        Ok(Mapping {
            values: BTreeMap::new(),
            key: None,
            tag: Some(tag),
        })
    }
}
fn wide(text: String) -> Value {
    Value::Newtype(Box::new(Value::Seq(vec![
        Value::Bool(true),
        Value::String(text),
    ])))
}
struct Sequence {
    values: Vec<Value>,
    tag: Option<&'static str>,
}
impl Sequence {
    fn push<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), Error> {
        self.values.push(to_value(v)?);
        Ok(())
    }
    fn finish(self) -> Result<Value, Error> {
        let v = Value::Seq(self.values);
        Ok(if let Some(k) = self.tag {
            Value::Map(BTreeMap::from([(Value::String(k.into()), v)]))
        } else {
            v
        })
    }
}
macro_rules! sequence {
    ($trait:ident,$method:ident) => {
        impl $trait for Sequence {
            type Ok = Value;
            type Error = Error;
            fn $method<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), Error> {
                self.push(v)
            }
            fn end(self) -> Result<Value, Error> {
                self.finish()
            }
        }
    };
}
sequence!(SerializeSeq, serialize_element);
sequence!(SerializeTuple, serialize_element);
sequence!(SerializeTupleStruct, serialize_field);
sequence!(SerializeTupleVariant, serialize_field);
struct Mapping {
    values: BTreeMap<Value, Value>,
    key: Option<Value>,
    tag: Option<&'static str>,
}
impl Mapping {
    fn field<T: Serialize + ?Sized>(&mut self, k: &'static str, v: &T) -> Result<(), Error> {
        self.values.insert(Value::String(k.into()), to_value(v)?);
        Ok(())
    }
    fn finish(self) -> Result<Value, Error> {
        let v = Value::Map(self.values);
        Ok(if let Some(k) = self.tag {
            Value::Map(BTreeMap::from([(Value::String(k.into()), v)]))
        } else {
            v
        })
    }
}
impl SerializeMap for Mapping {
    type Ok = Value;
    type Error = Error;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, k: &T) -> Result<(), Error> {
        self.key = Some(to_value(k)?);
        Ok(())
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, v: &T) -> Result<(), Error> {
        let k = self
            .key
            .take()
            .ok_or_else(|| Error::custom("map value without key"))?;
        self.values.insert(k, to_value(v)?);
        Ok(())
    }
    fn end(self) -> Result<Value, Error> {
        self.finish()
    }
}
macro_rules! mapping {
    ($trait:ident) => {
        impl $trait for Mapping {
            type Ok = Value;
            type Error = Error;
            fn serialize_field<T: Serialize + ?Sized>(
                &mut self,
                k: &'static str,
                v: &T,
            ) -> Result<(), Error> {
                self.field(k, v)
            }
            fn end(self) -> Result<Value, Error> {
                self.finish()
            }
        }
    };
}
mapping!(SerializeStruct);
mapping!(SerializeStructVariant);
