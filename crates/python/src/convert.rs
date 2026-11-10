use pyo3::{
    IntoPyObjectExt,
    prelude::*,
    types::{PyBool, PyDict, PyInt, PyList, PyString, PyTuple},
};
use serde_json::Value;

/// Native JSON shape with decimal strings and integer IDs preserved. Never
/// convert integer values through f64 or decimal strings through Python float.
pub fn to_python(py: Python<'_>, value: Value) -> PyResult<Py<PyAny>> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(value) => value.into_py_any(py),
        Value::String(value) => value.into_py_any(py),
        Value::Number(value) => {
            if let Some(value) = value.as_i64() {
                value.into_py_any(py)
            } else if let Some(value) = value.as_u64() {
                value.into_py_any(py)
            } else {
                let text = value.to_string();
                let constructor = if text.contains(['.', 'e', 'E']) {
                    py.import("decimal")?.getattr("Decimal")?
                } else {
                    py.import("builtins")?.getattr("int")?
                };
                Ok(constructor.call1((text,))?.unbind())
            }
        }
        Value::Array(values) => {
            let list = PyList::empty(py);
            for value in values {
                list.append(to_python(py, value)?)?;
            }
            Ok(list.into_any().unbind())
        }
        Value::Object(values) => {
            let dict = PyDict::new(py);
            for (key, value) in values {
                dict.set_item(key, to_python(py, value)?)?;
            }
            Ok(dict.into_any().unbind())
        }
    }
}

/// Strict native JSON input. Floating point and implicit numeric coercion are
/// rejected before parsing the exchange's typed Rust request.
pub fn from_python(value: &Bound<'_, PyAny>) -> PyResult<Value> {
    from_python_at(value, 0, false)
}
/// Data request parameters can opt into exact JSON numeric tokens supplied as Decimal.
pub fn from_python_with_decimal(value: &Bound<'_, PyAny>) -> PyResult<Value> {
    from_python_at(value, 0, true)
}
fn from_python_at(value: &Bound<'_, PyAny>, depth: usize, allow_decimal: bool) -> PyResult<Value> {
    if depth >= 128 {
        return Err(crate::errors::ValidationError::new_err(
            "native request nesting exceeds 128 levels (possibly cyclic input)",
        ));
    }
    if value.is_none() {
        return Ok(Value::Null);
    }
    if value.is_instance_of::<PyBool>() {
        return Ok(Value::Bool(value.extract()?));
    }
    if let Ok(text) = value.cast::<PyString>() {
        return Ok(Value::String(text.to_str()?.to_owned()));
    }
    if value.is_instance_of::<PyInt>() {
        return value
            .str()?
            .to_str()?
            .parse::<serde_json::Number>()
            .map(Value::Number)
            .map_err(|_| crate::errors::ValidationError::new_err("invalid integer"));
    }
    if let Ok(dict) = value.cast::<PyDict>() {
        let mut result = serde_json::Map::new();
        for (key, value) in dict {
            let key = key
                .cast::<PyString>()
                .map_err(|_| {
                    crate::errors::ValidationError::new_err("native request keys must be strings")
                })?
                .to_str()?
                .to_owned();
            result.insert(key, from_python_at(&value, depth + 1, allow_decimal)?);
        }
        return Ok(Value::Object(result));
    }
    if let Ok(list) = value.cast::<PyList>() {
        return list
            .iter()
            .map(|v| from_python_at(&v, depth + 1, allow_decimal))
            .collect::<PyResult<Vec<_>>>()
            .map(Value::Array);
    }
    if let Ok(tuple) = value.cast::<PyTuple>() {
        return tuple
            .iter()
            .map(|v| from_python_at(&v, depth + 1, allow_decimal))
            .collect::<PyResult<Vec<_>>>()
            .map(Value::Array);
    }
    if allow_decimal {
        let decimal = value.py().import("decimal")?.getattr("Decimal")?;
        // Exact built-in type only: never invoke arbitrary user coercion hooks.
        if value.get_type().is(&decimal) {
            return value
                .str()?
                .to_str()?
                .parse::<serde_json::Number>()
                .map(Value::Number)
                .map_err(|_| {
                    crate::errors::ValidationError::new_err("Decimal must be a finite JSON number")
                });
        }
    }
    Err(crate::errors::ValidationError::new_err(
        "use native dictionaries, sequences, strings, integers, booleans or None; prices and quantities must use exact native strings or scaled integers",
    ))
}
pub fn parse<T: serde::de::DeserializeOwned>(value: &Bound<'_, PyAny>) -> PyResult<T> {
    serde_json::from_value(from_python(value)?).map_err(|_| {
        crate::errors::ValidationError::new_err("request does not match the native Rust schema")
    })
}
pub fn output(value: impl serde::Serialize) -> PyResult<Py<PyAny>> {
    let value = serde_json::to_value(value)
        .map_err(|_| crate::errors::DecodeError::new_err("unable to convert native response"))?;
    Python::attach(|py| to_python(py, value))
}
