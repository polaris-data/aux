"""Generate the SDK wire models and typed methods from the local REST schema snapshots."""
import json
import keyword
import re
from graphlib import TopologicalSorter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATHS = {}
for path in sorted((ROOT / "src").glob("*/*/schema/*.json"), key=lambda path: path.stem):
    if path.stem in SCHEMA_PATHS:
        raise ValueError(f"Duplicate SDK schema name in {SCHEMA_PATHS[path.stem]} and {path}")
    SCHEMA_PATHS[path.stem] = path
SCHEMAS = {venue: json.loads(path.read_text()) for venue, path in SCHEMA_PATHS.items()}


def typename(ty, lang, prefix=""):
    ty = ty.replace(", ", ",")
    scalar = {"String": ("string", "str"), "bool": ("boolean", "bool"),
              "serde_json::Number": ("number | bigint | string", "int | Decimal"),
              "serde_json::Value": ("HttpValue", "Any")}
    for number in ("i128", "i64", "u64"):
        scalar[number] = ("number | bigint", "int")
    for number in ("u8", "u16", "u32", "i8", "i16", "i32"):
        scalar[number] = ("number", "int")
    scalar["serde_json::Map<String,serde_json::Value>"] = ("Record<string, HttpValue>", "dict[str, Any]")
    i = int(lang == "py")
    if ty in scalar:
        return scalar[ty][i]
    if ty.startswith("Box<"):
        return typename(ty[4:-1], lang, prefix)
    if ty.startswith("Option<"):
        inner = typename(ty[7:-1], lang, prefix)
        suffix = " | None" if i else " | null"
        return inner if inner.endswith(suffix) else inner + suffix
    if ty.startswith("(") and ty.endswith(")"):
        items = [typename(t.strip(), lang, prefix) for t in ty[1:-1].split(",")]
        return "tuple[" + ", ".join(items) + "]" if i else "[" + ", ".join(items) + "]"
    if ty.startswith("[") and ty.endswith("]"):
        item, count = ty[1:-1].rsplit(";", 1)
        value = typename(item, lang, prefix)
        return f"list[{value}]" if i else "[" + ", ".join([value] * int(count)) + "]"
    if ty.startswith("Vec<"):
        value = typename(ty[4:-1], lang, prefix)
        return f"list[{value}]" if i else f"Array<{value}>"
    container = "std::collections::BTreeMap<String,"
    if ty.startswith(container):
        value = typename(ty[len(container):-1], lang, prefix)
        return f"dict[str, {value}]" if i else f"Record<string, {value}>"
    if ty == "NativeNumber":
        return "int | Decimal | str" if i else "number | bigint | string"
    return prefix + ty


def emit_models(emit, venue, exposed):
    schema = SCHEMAS[venue]
    entity = "".join(word.title() for word in venue.split("_"))
    client = schema.get("client", entity + "Client")
    fixture = schema.get("fixture", "http")
    service_venue = schema.get("venue", venue)
    source = SCHEMA_PATHS[venue].relative_to(ROOT).as_posix()
    feature = f"{venue.replace('_', '-')}-models"
    data_models = venue in {"hyperdash", "birdeye", "dune", "qfex", "metric", "lighter", "monaco", "hyperliquid"}
    native_number = venue in {"extended", "phoenix"}

    ts = [f'// Generated from {source}.', 'import type { HttpValue } from "./index";']
    py = [f'"""{entity} REST wire models generated from {source}."""',
          'from __future__ import annotations', 'from decimal import Decimal',
          'from typing import Any, TypedDict', '']
    rs = [f'// Generated from {source}.', 'use serde::{Deserialize, Serialize};',
          'use serde_json::{Value, json};', f'use crate::{{{client}, Error, Result}};',
          '']
    if not schema["models"] and not schema.get("enums"):
        rs[1] = ""
        rs[2] = "use serde_json::json;"
    if schema.get("oracle"):
        rs[2] = "use serde_json::json;"
    if any(f[3] == 'zero' for fields in schema['models'].values() for f in fields):
        rs.append('fn zero_number() -> serde_json::Number { 0.into() }')
    if native_number:
        rs.append('pub use crate::model_number::NativeNumber;')
    modes = {f[3] for fields in schema["models"].values() for f in fields}
    if "nullable_vec" in modes:
        rs.append("fn nullable_vec<'de,D:serde::Deserializer<'de>,T:Deserialize<'de>>(d:D) -> std::result::Result<Vec<T>,D::Error> { Ok(Option::<Vec<T>>::deserialize(d)?.unwrap_or_default()) }")
    if "present" in modes or "nullable" in modes:
        rs.append("fn present<'de,D:serde::Deserializer<'de>,T:Deserialize<'de>>(d:D) -> std::result::Result<Option<T>,D::Error> { Deserialize::deserialize(d) }")
    if "nullable" in modes:
        rs.append("fn optional_nullable<'de,D:serde::Deserializer<'de>,T:Deserialize<'de>>(d:D) -> std::result::Result<Option<Option<T>>,D::Error> { present(d).map(Some) }")
    if schema.get("enums"):
        py.append("from typing import Literal")
    for name, variants in schema.get("enums", {}).items():
        values = [json.dumps(wire) for wire, _ in variants]
        ts.append(f"export type {name} = " + " | ".join(values) + ";")
        py.append(f"{name} = Literal[" + ", ".join(values) + "]")
        rs.extend(["#[derive(Clone, Serialize, Deserialize)]", f"pub enum {name} {{"])
        for (wire, variant), value in zip(variants, values):
            rs.extend([f"    #[serde(rename = {value})]", f"    {variant},"])
        rs.append("}")
    py_aliases = []
    for name, variants in schema.get("unions", {}).items():
        ts.append(f"export type {name} = " + " | ".join(typename(t, "ts") for _, t in variants) + ";")
        py_aliases.append(f"{name} = " + " | ".join(typename(t, "py") for _, t in variants))
        rs.extend(["#[derive(Clone, Serialize, Deserialize)]", "#[serde(untagged)]", f"pub enum {name} {{"])
        rs.extend(f"    {variant}({ty})," for variant, ty in variants)
        rs.append("}")
    aliases = schema["aliases"]
    dependencies = {name: sorted(set(re.findall(r"\b\w+\b", ty)) & aliases.keys()) for name, ty in aliases.items()}
    for name in TopologicalSorter(dependencies).static_order():
        ty = aliases[name]
        ts.append(f"export type {name} = {typename(ty, 'ts')};")
        py_aliases.append(f"{name} = {typename(ty, 'py')}")
        rs.append(f"pub type {name} = {ty};")
    for name, fields in schema["models"].items():
        strict = name in schema.get("strict", [])
        ts.extend(["", f"export interface {name} {{"])
        if not strict:
            ts.append("  [field: string]: unknown;")
        # Functional syntax preserves wire keys such as "from". Inherited keys
        # support mixed required/optional fields on Python 3.10.
        optional = {wire: typename(ty, "py") for wire, _, ty, mode in fields if mode not in ("required", "present")}
        parent = "TypedDict"
        if optional:
            parent = f"_{name}Optional"
            py.append(f'{parent} = TypedDict("{parent}", {optional!r}, total=False)')
        unusual = {wire: typename(ty, "py") for wire, _, ty, mode in fields
                   if mode in ("required", "present") and (not wire.isidentifier() or keyword.iskeyword(wire))}
        if unusual:
            base = f"_{name}WireKeys"
            py.append(f'{base} = TypedDict("{base}", {unusual!r})')
            parent = base if parent == "TypedDict" else f"{base}, {parent}"
        py.append(f"class {name}({parent}):")
        needs_zero = any(ty == "serde_json::Number" for _, _, ty, _ in fields)
        derives = "Clone, Serialize, Deserialize" if needs_zero else "Clone, Default, Serialize, Deserialize"
        if data_models or (venue == "arcus" and name in {"PlaceOrderParams", "OrderRequest"}):
            derives = "Clone, Serialize, Deserialize"
        rs.extend(["", f"#[derive({derives})]"])
        if strict:
            rs.append("#[serde(deny_unknown_fields)]")
        rs.append(f"pub struct {name} {{")
        for wire, rust, ty, mode in fields:
            ts.append(f'  {json.dumps(wire)}{"?" if mode not in ("required", "present") else ""}: {typename(ty, "ts")};')
            if mode in ("required", "present") and wire not in unusual:
                py.append(f"    {wire}: {typename(ty, 'py')}")
            attrs = [f'rename = "{wire}"']
            if mode == "zero":
                attrs.append('default = "zero_number"')
            elif mode == "nullable_vec":
                attrs.extend(["default", 'deserialize_with = "nullable_vec"'])
            elif mode == "default":
                attrs.append("default")
            elif mode == "nullable":
                attrs += ["default", 'deserialize_with = "optional_nullable"']
            elif mode == "present":
                attrs.append('deserialize_with = "present"')
            if ty.startswith("Option<") and mode != "present":
                attrs.append('skip_serializing_if = "Option::is_none"')
            rs.extend(["    #[serde(" + ", ".join(attrs) + ")]", f"    pub {rust}: {ty},"])
        if all(mode not in ("required", "present") or k in unusual for k, _, _, mode in fields):
            py.append("    pass")
        py.append("")
        ts.append("}")
        if not strict:
            rs.extend(["    #[serde(flatten)]", "    pub extra: serde_json::Map<String, Value>,"])
        rs.append("}")
        if data_models or not needs_zero:
            continue
        rs.extend([f"impl Default for {name} {{", "    fn default() -> Self {", "        Self {"])
        for _, rust, ty, _ in fields:
            default = "0.into()" if ty == "serde_json::Number" else "Default::default()"
            rs.append(f"            {rust}: {default},")
        rs.extend(["            extra: Default::default(),", "        }", "    }", "}"])
    py.extend(py_aliases)
    rs.extend(["", f"pub struct Typed{client}({client});", f"impl {client} {{",
               f"    pub fn typed(self) -> Typed{client} {{ Typed{client}(self) }}", "}",
               f"impl Typed{client} {{"])
    for route, name in sorted(exposed.items()):
        arg, out = schema["methods"][name]
        rs.extend([f"    pub fn {name}(&self, request: &{arg}) -> Result<{out}> {{",
                   f"        serde_json::from_value(self.0.{name}(&json!(request))?)",
                   f'            .map_err(|_| Error::Decode("invalid {entity} response model"))', "    }"])
    rs.append("}")
    rs.extend(['#[cfg(feature = "async")]', f'pub struct AsyncTyped{client}(crate::AsyncClient<{client}>);',
               '#[cfg(feature = "async")]', f'impl Typed{client} {{',
               f'    pub fn into_async(self) -> AsyncTyped{client} {{ AsyncTyped{client}(self.0.into_async()) }}', '}',
               '#[cfg(feature = "async")]', f'impl AsyncTyped{client} {{'])
    for name in sorted(exposed.values()):
        arg, out = schema["methods"][name]
        rs.extend([f'    pub async fn {name}(&self, request: &{arg}) -> Result<{out}> {{',
                   f'        serde_json::from_value(self.0.{name}(&json!(request)).await?)',
                   f'            .map_err(|_| Error::Decode("invalid {entity} response model"))', '    }'])
    rs.append('}')
    tests = [f'#![cfg(feature = "{feature}")]', f'// Generated from {source}.',
             f'use aux_rs::{venue}_models::*;', 'use serde::{de::DeserializeOwned, Serialize};',
             'use serde_json::Value;', '',
             'fn wire_eq(actual: &Value, wire: &Value) {',
             '    match (actual, wire) {',
             '        (Value::Object(a),Value::Object(b)) => {',
             '            for (k,v) in b { if v.is_null() && !a.contains_key(k) { continue; } wire_eq(&a[k],v); }',
             '            assert!(a.keys().all(|k| b.contains_key(k)));',
             '        },',
             '        (Value::Array(a),Value::Array(b)) => { assert_eq!(a.len(),b.len()); for (x,y) in a.iter().zip(b) { wire_eq(x,y); } },',
             '        _ => assert_eq!(actual,wire),',
             '    }',
             '}',
             'fn check<T: DeserializeOwned + Serialize>(v: &Value) {',
             '    let model: T = serde_json::from_value(v.clone()).unwrap();',
             '    wire_eq(&serde_json::to_value(model).unwrap(), v);', '}', '',
             '#[test]', 'fn models_round_trip_exposed_rest_fixtures() {',
             f'    let rows: Vec<Value> = serde_json::from_str(include_str!("../../../tests/{fixture}.json")).unwrap();',
             f'    for row in rows.iter().filter(|r| r["venue"] == "{service_venue}") {{',
             '        let read = |k: &str| row[format!("{k}_json")].as_str().map(|s| serde_json::from_str::<Value>(s).unwrap()).unwrap_or_else(|| row[k].clone());',
             '        let request = &read("request");',
             '        let response = &read("response");',
             '        match row["operation"].as_str().unwrap() {']
    if venue == "metric":
        at = tests.index('            assert!(a.keys().all(|k| b.contains_key(k)));')
        tests[at] = '            assert!(a.keys().all(|k| b.contains_key(k) || (k == "pending" && b.contains_key("provider") && a[k] == false)));'
    if venue == "hyperdash":
        index = next(i for i, line in enumerate(tests) if "let response = &read" in line)
        tests[index] = '        let response = &serde_json::json!({"status":200,"data":read("response"),"metadata":{}});'
    if schema.get("oracle") or schema.get("text_fixtures"):
        index = next(i for i, line in enumerate(tests) if "let rows: Vec<Value>" in line)
        tests[index] = tests[index].replace("let rows:", "let mut rows:")
        tests.insert(index + 2, '        if row["status"].as_u64().unwrap_or(200) >= 300 { continue; }')
        tests.insert(index + 1, '    rows.extend(serde_json::from_str::<Vec<Value>>(include_str!("../../../tests/text-http.json")).unwrap());')
        index = next(i for i, line in enumerate(tests) if "let response = &read" in line)
        tests[index] = '        let response = &row.get("expected").cloned().unwrap_or_else(|| serde_json::json!({"status":row["status"],"data":read("response"),"headers":[]}));'
        if not schema.get("oracle"):
            tests[index] = '        let response = &row.get("expected").cloned().unwrap_or_else(|| read("response"));'
    for route, name in sorted(exposed.items()):
        if venue == "hyperdash" and route.endswith(("_csv", "_ndjson")):
            continue
        arg, out = schema["methods"][name]
        tests.append(f'            "{route}" => {{ check::<{arg}>(request); check::<{out}>(response); }},')
    tests.extend(['            _ => continue,','        }','    }','}'])
    tests = [tests[0], tests[1], "#[rustfmt::skip]", "mod fixtures {",
             *["    " + line if line else "" for line in tests[2:]], "}"]
    emit(ROOT / f"sdk/rust/tests/{venue}_models.rs", "\n".join(tests) + "\n")
    # rustfmt skips this generated file through the module declaration.
    emit(ROOT / f"sdk/rust/src/{venue}_models.rs", "\n".join(rs) + "\n")
    emit(ROOT / f"sdk/typescript/js/{venue}_models.d.ts", "\n".join(ts) + "\n")
    emit(ROOT / f"sdk/python/python/aux_py/{venue}_models.py", "\n".join(py) + "\n")
