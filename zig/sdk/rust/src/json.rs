//! Convert the Zig decoder's synchronous token stream into the Rust SDK value.
use crate::{Error, Result};
use serde_json::{Map, Value};
use std::{ffi::c_void, slice, str};

unsafe extern "C" {
    fn aux_v1_json_visit(
        input: *const u8,
        len: usize,
        context: *mut c_void,
        visit: extern "C" fn(*mut c_void, u32, *const u8, usize) -> bool,
    ) -> u32;
}

enum Container {
    Array(Vec<Value>),
    Object(Map<String, Value>, Option<String>),
}

#[derive(Default)]
struct Builder {
    containers: Vec<Container>,
    root: Option<Value>,
}

impl Builder {
    fn token(&mut self, kind: u32, text: &str) -> Option<()> {
        let value = match kind {
            0 => {
                self.containers.push(Container::Object(Map::new(), None));
                return Some(());
            }
            2 => {
                self.containers.push(Container::Array(Vec::new()));
                return Some(());
            }
            1 | 3 => match (kind, self.containers.pop()?) {
                (1, Container::Object(map, None)) => Value::Object(map),
                (3, Container::Array(values)) => Value::Array(values),
                _ => return None,
            },
            4 => {
                if let Some(Container::Object(_, key @ None)) = self.containers.last_mut() {
                    *key = Some(text.to_owned());
                    return Some(());
                }
                Value::String(text.to_owned())
            }
            5 => Value::Number(text.parse().ok()?),
            6 => Value::Bool(true),
            7 => Value::Bool(false),
            8 => Value::Null,
            _ => return None,
        };
        match self.containers.last_mut() {
            Some(Container::Array(values)) => values.push(value),
            Some(Container::Object(map, key)) => {
                map.insert(key.take()?, value);
            }
            None if self.root.is_none() => self.root = Some(value),
            None => return None,
        }
        Some(())
    }
}

extern "C" fn visit(context: *mut c_void, kind: u32, ptr: *const u8, len: usize) -> bool {
    // SAFETY: aux_v1_json_visit calls back synchronously with the exclusive Builder
    // and a borrowed token. Neither pointer escapes this callback.
    let builder = unsafe { &mut *context.cast::<Builder>() };
    let bytes = if len == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(ptr, len) }
    };
    str::from_utf8(bytes)
        .ok()
        .and_then(|text| builder.token(kind, text))
        .is_some()
}

pub fn decode(input: &[u8]) -> Result<Value> {
    let mut builder = Builder::default();
    // SAFETY: The input and Builder outlive this call. No pointers are retained.
    let status = unsafe {
        aux_v1_json_visit(
            input.as_ptr(),
            input.len(),
            (&raw mut builder).cast(),
            visit,
        )
    };
    match status {
        0 if builder.containers.is_empty() => {
            builder.root.ok_or(Error::Decode("empty JSON document"))
        }
        2 => Err(Error::OutOfMemory),
        _ => Err(Error::Decode("invalid JSON document")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_public_values_and_rejections() {
        for input in [
            r#"{"n":9007199254740993123456789012,"decimal":1.00000000000000000001,"x":1E-999,"negative_zero":-0}"#,
            r#"[true,false,null,{},[],{"\u0061":"\ud83d\ude03","a":"b"}]"#,
            r#"[null]"#,
            r#"42"#,
            r#" "a" "#,
            "",
            "  ",
            "1 2",
            "[1,]",
            r#""\ud800""#,
            "1.",
            "01",
        ] {
            assert_eq!(
                decode(input.as_bytes()).ok(),
                serde_json::from_str::<Value>(input).ok(),
                "{input}"
            );
        }
        assert!(decode(b"\"\xff\"").is_err());
        assert!(decode(format!("{}0{}", "[".repeat(128), "]".repeat(128)).as_bytes()).is_err());
    }
}
