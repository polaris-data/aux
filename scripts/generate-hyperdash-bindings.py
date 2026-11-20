#!/usr/bin/env python3
"""Generate Hyperdash models, method bindings, and offline dispatch fixtures."""
import argparse
import hashlib
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DATA = ROOT / "crates/data/hyperdash"
PYTHON = ROOT / "crates/python/python/aux_py"
NODE = ROOT / "crates/node/js"
RUST_KEYWORDS = set((
    "as break const continue crate else enum extern false fn for if impl in let loop match mod "
    "move mut pub ref return self Self static struct super trait true type unsafe use where while "
    "async await dyn abstract become box do final gen macro override priv try typeof unsized virtual yield"
).split())
HEADERS = {
    "X-Max-Credits": "max_credits",
    "X-Credit-Price-Version": "credit_price_version",
    "Idempotency-Key": "idempotency_key",
}
CHANNELS = [
    "fills", "ledger", "twap", "orders", "funding", "account", "trades", "liquidations",
    "candles", "ctx", "book", "bbo", "liquidation-levels", "stop-levels", "mids", "markets",
]


def quoted(value):
    return json.dumps(value, ensure_ascii=False)


def snake(name):
    name = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    return re.sub(r"([a-z\d])([A-Z])", r"\1_\2", name).lower()


def pascal(name):
    return "".join(word[0].upper() + word[1:] for word in re.findall(r"[a-zA-Z0-9]+", name))


def enum_variants(values):
    units = {"s": "Seconds", "m": "Minutes", "h": "Hours", "d": "Days",
             "w": "Weeks", "M": "Months"}
    used = set()
    for value in values:
        interval = re.fullmatch(r"(\d+)([smhdwM])", value)
        if interval:
            name = units[interval[2]] + interval[1]
        else:
            name = pascal(value)
            if not name or name[0].isdigit() or name in RUST_KEYWORDS:
                name = "Value" + name
        candidate = name
        suffix = 2
        while candidate in used:
            candidate = name + str(suffix)
            suffix += 1
        used.add(candidate)
        yield candidate


def rust_field(name):
    safe = re.sub(r"[^A-Za-z0-9_]", "_", name)
    if not safe or not re.match(r"[A-Za-z_]", safe[0]) or safe == "_":
        safe = "field_" + safe
    if safe in RUST_KEYWORDS:
        safe += "_"
    return safe


def rustfmt(text):
    result = subprocess.run(
        ["rustfmt", "--edition", "2024", "--emit", "stdout"],
        input=text, text=True, capture_output=True,
    )
    if result.returncode:
        raise SystemExit(result.stderr)
    return result.stdout


def pinned(source):
    path = DATA / source["path"]
    content = path.read_bytes()
    if hashlib.sha256(content).hexdigest() != source["sha256"]:
        raise SystemExit(f"Hyperdash source hash mismatch: {path}")
    return json.loads(content)


def inventory():
    pins = json.loads((DATA / "API_SOURCES.json").read_text())
    spec = pinned(pins["openapi"])
    supplement = pinned(pins["supplement"])
    result = []
    for path, methods in spec["paths"].items():
        for verb, info in methods.items():
            if verb not in ("get", "post", "delete"):
                raise ValueError((verb, path))
            responses = {
                status: schema["content"]["application/json"]["schema"]
                for status, schema in info["responses"].items() if status.startswith("2")
            }
            if any(schema != responses["200"] for schema in responses.values()):
                raise ValueError(f"distinct success schemas for {path}")
            result.append({
                "id": info["operationId"], "path": path, "verb": verb,
                "auth": bool(info.get("security", spec.get("security", []))),
                "parameters": info.get("parameters", []),
                "body": info["requestBody"]["content"]["application/json"]["schema"]
                if "requestBody" in info else None,
                "response": responses["200"],
                "status": 202 if "202" in responses else 200,
            })
    for operation_id, entry in supplement.items():
        result.append({
            "id": operation_id, "verb": "get", "auth": False, "parameters": [],
            "body": None, "status": 200, **entry,
        })
    for operation in result:
        operation["method"] = snake(operation["id"])
        operation["name"] = pascal(operation["method"])
        operation["batch"] = "batch" in operation["path"].split("/")
        fields = operation["response"].get("properties", {})
        operation["text"] = {"data", "has_more", "next_cursor", "meta"} <= fields.keys()
    assert len({op["method"] for op in result}) == len(result) == len({
        (op["verb"], op["path"]) for op in result
    })
    return sorted(result, key=lambda op: op["method"])


class Models:
    def __init__(self):
        self.rust = {}
        self.python = {}
        self.ts = {}

    def add(self, name, rust, python, ts):
        if name in self.rust:
            if (rust, python, ts) != (self.rust[name], self.python[name], self.ts[name]):
                raise ValueError(f"conflicting Hyperdash model {name}")
            return
        self.rust[name], self.python[name], self.ts[name] = rust, python, ts

    def types(self, name, schema, request):
        kind = schema.get("type")
        if isinstance(kind, list):
            kinds = set(kind) - {"null"}
            if len(kinds) != 1:
                raise ValueError((name, kind))
            kind = kinds.pop()

        if kind == "array":
            rust, python, ts = self.types(name + "Item", schema["items"], request)
            return f"Vec<{rust}>", f"list[{python}]", f"Array<{ts}>"

        if kind == "object":
            if schema.get("properties"):
                return self.record(name, schema, request)
            return "Map<String, Value>", "dict[str, Any]", "Record<string, NativeValue>" if request else "Record<string, LosslessValue>"

        if kind == "string" and request and schema.get("enum"):
            values = schema["enum"]
            variants = enum_variants(values)
            rust = [
                "#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]",
                f"pub enum {name} {{",
            ]
            rust += [f"#[serde(rename = {quoted(value)})] {variant}," for value, variant in zip(values, variants)]
            rust += ["}"]
            self.add(
                name, "\n".join(rust),
                f"{name} = Literal[{', '.join(map(quoted, values))}]",
                f"export type {name} = {' | '.join(map(quoted, values))};",
            )
            return name, name, name

        primitives = {
            "string": ("String", "str", "string"),
            "integer": ("i128", "int", "bigint"),
            "boolean": ("bool", "bool", "boolean"),
            None: ("Value", "Any", "NativeValue" if request else "LosslessValue"),
        }
        if kind not in primitives:
            raise ValueError((name, kind))
        return primitives[kind]

    def record(self, name, schema, request):
        properties = schema.get("properties", {})
        required = set(schema.get("required", []))
        rust = ["#[derive(Clone, Debug, serde::Deserialize, serde::Serialize" +
                (", Default" if not required else "") + ")]"]
        if request and not schema.get("extra_query"):
            rust.append("#[serde(deny_unknown_fields)]")
        rust.append(f"pub struct {name} {{")
        py_fields = {True: [], False: []}
        ts = [f"export interface {name} {{"]
        field_names = set()
        for key, options in properties.items():
            field = rust_field(key)
            if field in field_names:
                raise ValueError((name, key))
            field_names.add(field)
            is_required = key in required
            nullable = isinstance(options.get("type"), list) and "null" in options["type"]
            r, p, t = self.types(name + pascal(key), options, request)
            if key != field:
                rust.append(f"#[serde(rename = {quoted(key)})]")
            if nullable:
                p += " | None"
                t += " | null"
                r = f"Option<{r}>"
                if is_required:
                    rust.append('#[serde(deserialize_with = "crate::present")]')
            if not is_required:
                r = f"Option<{r}>"
                if nullable:
                    rust.append('#[serde(default, deserialize_with = "crate::optional_nullable")]')
                rust.append('#[serde(skip_serializing_if = "Option::is_none")]')
            rust.append(f"pub {field}: {r},")
            py_fields[is_required].append(f"{quoted(key)}: {p}")
            ts.append(f"  {quoted(key)}{'' if is_required else '?'}: {t};")
        if not request:
            rust += ["#[serde(flatten)]", "pub extra: Map<String, Value>,"]
            ts.append("  [field: string]: unknown;")
        rust.append("}")
        ts.append("}")
        py = []
        for needed in (True, False):
            fields = py_fields[needed]
            if fields:
                suffix = ("Required" if needed else "Optional") if all(py_fields.values()) else ""
                dict_name = f"_{name}{suffix}" if suffix else name
                total = "" if needed else ", total=False"
                py.append(f'{dict_name} = TypedDict("{dict_name}", {{{", ".join(fields)}}}{total})')
        if all(py_fields.values()):
            py.append(f"class {name}(_{name}Required, _{name}Optional): ...")
        if not py:
            py.append(f'{name} = TypedDict("{name}", {{}})')
        self.add(name, "\n".join(rust), "\n".join(py), "\n".join(ts))
        return name, name, name


def sample(schema):
    kind = schema.get("type")
    if isinstance(kind, list):
        return None
    if "enum" in schema:
        return schema["enum"][0]
    if kind == "object":
        properties = schema.get("properties", {})
        return {key: sample(properties[key]) for key in schema.get("required", [])}
    if kind == "array":
        return [sample(schema["items"])]
    if kind == "string":
        return "value"
    if kind == "integer":
        return 42
    if kind == "boolean":
        return False
    return None


def generate(operations):
    models = Models()
    rust_operations = []
    dispatch = []
    response_variants = []
    python = []
    pyi = []
    js = []
    ts = []
    fixtures = []
    for op in operations:
        name, method, js_name = op["name"], op["method"], op["id"]
        request_name, response_name = name + "Request", name + "Response"
        fields = {}
        required = []
        wire_path, wire_query, wire_headers = [], [], []
        fixture_request, fixture_query, fixture_headers = {}, {}, {}
        fixture_path = op["path"]
        for parameter in op["parameters"]:
            key, location, shape = parameter["name"], parameter["in"], parameter["schema"]
            field = HEADERS[key] if location == "header" else key
            if field in fields:
                raise ValueError((name, field))
            fields[field] = shape
            value = sample(shape)
            fixture_request[field] = value
            if parameter.get("required"):
                required.append(field)
            if location == "path":
                wire_path.append(key)
                fixture_path = fixture_path.replace("{" + key + "}", str(value))
            elif location == "query":
                wire_query.append(key)
                fixture_query[key] = str(value)
            elif location == "header":
                wire_headers.append((field, key))
                fixture_headers[key] = str(value)
            else:
                raise ValueError((name, location))
        if op["body"] is not None:
            fields["body"] = op["body"]
            required.append("body")
            fixture_request["body"] = sample(op["body"])
        if op.get("extra_query"):
            fields["query"] = {"type": "object"}
            fixture_request["query"] = {"coin": "BTC", "limit": 10}
            fixture_query.update({"coin": "BTC", "limit": "10"})
        models.record(request_name, {"properties": fields, "required": required}, True)
        models.record(response_name, op["response"], False)
        operation = method.upper()
        rust_operations.append(f'''const {operation}: Operation = Operation {{
            method: HttpMethod::{op["verb"].title()}, path: {quoted(op["path"])},
            auth: {str(op["auth"]).lower()}, path_fields: &{quoted(wire_path)},
            query_fields: &{quoted(wire_query)}, headers: &[{", ".join(f"({quoted(k)}, {quoted(v)})" for k, v in wire_headers)}],
            batch: {str(op["batch"]).lower()},
            body_limit: {1024 * 1024 if op["batch"] else 128 * 1024 if method == "lookup_alo_priority" else 64 * 1024},
            extra_query: {str(op.get("extra_query", False)).lower()},
        }};''')
        response_variants.append(f"{name}(Box<HyperdashResponse<{response_name}>>),")
        for suffix, js_suffix, fmt in [("", "", "Json")] + (
            [("_csv", "Csv", "Csv"), ("_ndjson", "Ndjson", "Ndjson")] if op["text"] else []
        ):
            rust_name = method + suffix
            js_method = js_name + js_suffix
            text = bool(suffix)
            result = "String" if text else response_name
            call = (f"self.text(&{operation}, request, ResponseFormat::{fmt})"
                    if text else f"self.request(&{operation}, request)")
            rust_operations.append(f'''impl<T: StreamingHttpTransport> HyperdashClient<T> {{
                /// `{op["verb"].upper()} {op["path"]}` ({fmt}).
                pub async fn {rust_name}(&self, request: {request_name}) -> Result<HyperdashResponse<{result}>> {{
                    {call}.await
                }}
            }}''')
            variant = "Text" if text else name
            dispatch.append(
                f'{quoted(rust_name)} => Box::pin(async move {{ '
                f'self.{rust_name}(decode(request)?).await.map(|response| DispatchResponse::{variant}(Box::new(response))) '
                f'}}),'
            )
            py_request = "request" if required else "request=None"
            python.append(f'    async def {rust_name}(self, {py_request}):\n        return await self._native.request("{rust_name}", request)')
            py_type = request_name if required else request_name + " | None = None"
            pyi.append(f"    async def {rust_name}(self, request: {py_type}) -> HyperdashResponse[{'str' if text else result}]: ...")
            js.append(f'  {js_method}(request) {{ return this.#request("{rust_name}", request); }}')
            ts.append(f"  {js_method}(request{'' if required else '?'}: {request_name}): Promise<HyperdashResponse<{'string' if text else result}>>;")
        fixtures.append({
            "operation": method, "node": js_name, "method": op["verb"].upper(), "path": fixture_path,
            "auth": op["auth"], "query": fixture_query, "headers": fixture_headers,
            "body": fixture_request.get("body"), "request": fixture_request,
            "status": op["status"], "text": op["text"],
            "response": sample(op["response"]),
        })
    rust = [
        "//! Generated by scripts/generate-hyperdash-bindings.py.",
        "use crate::{HyperdashClient, HyperdashResponse, Error, ResponseFormat, Result, client::Operation};",
        "use aux_rs_core::{HttpMethod, StreamingHttpTransport};",
        "use serde_json::{Map, Value};",
        *models.rust.values(), *rust_operations,
        '#[doc(hidden)] #[derive(serde::Serialize)] #[serde(untagged)] pub enum DispatchResponse {',
        *response_variants, "Text(Box<HyperdashResponse<String>>),", "}",
        "impl<T: StreamingHttpTransport> HyperdashClient<T> {",
        "#[doc(hidden)]",
        "pub async fn dispatch(&self, method: &str, request: Value) -> Result<DispatchResponse> {",
        "fn decode<T: serde::de::DeserializeOwned>(request: Value) -> Result<T> {",
        'let value = if request.is_null() { Value::Object(Map::new()) } else { request };',
        'serde_json::from_value(value).map_err(|_| Error::InvalidRequest("request does not match the Hyperdash schema"))',
        "}",
        "let response: std::pin::Pin<Box<dyn Future<Output = Result<DispatchResponse>> + Send + '_>> = match method {",
        *dispatch,
        '_ => return Err(Error::InvalidRequest("unknown Hyperdash operation")),',
        "};", "response.await", "}", "}",

    ]
    return {
        DATA / "src/generated.rs": rustfmt("\n".join(rust)),
        DATA / "tests/dispatch.json": json.dumps(fixtures, indent=2, ensure_ascii=False) + "\n",
        PYTHON / "hyperdash.py": PY_RUNTIME + "\n".join(python) + "\n",
        PYTHON / "hyperdash.pyi": PY_TYPES + "\n\n".join(models.python.values()) + "\n\n" + PY_CLIENT + "\n".join(pyi) + "\n",
        NODE / "hyperdash.cjs": JS_RUNTIME + "\n".join(js) + "\n}\nmodule.exports = { HyperdashClient, HyperdashStream };\n",
        NODE / "hyperdash.d.ts": TS_TYPES + "\n\n".join(models.ts.values()) + "\n\n" + TS_CLIENT + "\n".join(ts) + "\n}\n",
    }

PY_RUNTIME = '''"""Hyperdash Data API client. Protocol handling runs in the Rust connector."""
# Generated by scripts/generate-hyperdash-bindings.py.
from . import _native


class HyperdashStream:
    def __init__(self, native):
        self._native = native

    @property
    def closed(self):
        return self._native.closed

    async def subscribe(self, request):
        await self._native.subscribe(request)

    async def next(self):
        return await self._native.next()

    async def ack(self, sub, batch):
        await self._native.ack(sub, batch)

    async def unsubscribe(self, sub):
        await self._native.unsubscribe(sub)

    async def aclose(self):
        await self._native.aclose()

    close = aclose

    def __aiter__(self):
        return self

    async def __anext__(self):
        message = await self.next()
        if message is None:
            raise StopAsyncIteration
        return message

    async def __aenter__(self):
        return self

    async def __aexit__(self, *_):
        await self.aclose()


class HyperdashClient:
    def __init__(self, api_key=None, *, api_url="https://data.hyperdash.com",
                 ws_url="wss://data.hyperdash.com/v1/stream", timeout_ms=10_000,
                 stream_capacity=64):
        self._native = _native.HyperdashClient(api_key, api_url, ws_url, timeout_ms, stream_capacity)

    @property
    def closed(self):
        return self._native.closed

    async def aclose(self):
        await self._native.aclose()

    async def open_stream(self):
        return HyperdashStream(await self._native.open_stream())

    async def __aenter__(self):
        return self

    async def __aexit__(self, *_):
        await self.aclose()

'''

METADATA = {
    "request_id": ("str", "string"), "credits_charged": ("int", "bigint"),
    "credit_price_version": ("int", "bigint"), "credits_low": ("bool", "boolean"),
    "problem_code": ("str", "string"), "retry_after": ("str", "string"),
    "has_more": ("bool", "boolean"), "next_cursor": ("str", "string"),
    "as_of": ("int", "bigint"), "complete_to": ("int", "bigint"),
    "truncated": ("bool", "boolean"), "max_rows": ("int", "bigint"),
    "incomplete": ("bool", "boolean"),
}
STREAM_MESSAGES = [
    ("Ready", "ready", {"key_id": ("str", "string"), "environment": ("str", "string")}, {}),
    ("Subscribed", "subscribed", {"sub": ("StreamId", "StreamId"), "channel": ("str", "string")}, {
        "address": ("str", "string"), "coin": ("str", "string"), "cursor": ("str | None", "string | null"),
    }),
    ("Batch", "data", {
        "sub": ("StreamId", "StreamId"), "batch": ("StreamId", "StreamId"),
        "cursor": ("str", "string"), "events": ("list[Any]", "LosslessValue[]"), "credits": ("int", "bigint"),
    }, {}),
    ("Acked", "acked", {
        "sub": ("StreamId", "StreamId"), "batch": ("StreamId", "StreamId"), "credits_charged": ("int", "bigint"),
    }, {}),
    ("Unsubscribed", "unsubscribed", {"sub": ("StreamId", "StreamId")}, {}),
    ("Gap", "gap", {
        "sub": ("StreamId", "StreamId"), "from": ("Any", "LosslessValue"), "to": ("Any", "LosslessValue"),
        "detail": ("str", "string"),
    }, {}),
    ("CreditsLow", "credits_low", {"remaining": ("int", "bigint")}, {}),
    ("Problem", "error", {"code": ("str", "string"), "detail": ("str", "string")}, {
        "sub": ("StreamId", "StreamId"), "retry_after_s": ("int", "bigint"),
    }),
]


def py_stream_messages():
    result = []
    for name, op, required, optional in STREAM_MESSAGES:
        result.append(f'_{name}Required = TypedDict("_{name}Required", {{"op": Literal[{quoted(op)}], ' +
                      ", ".join(f"{quoted(key)}: {kind[0]}" for key, kind in required.items()) + "})")
        if optional:
            result.append(f"class Stream{name}(_{name}Required, total=False):")
            result.extend(f"    {name}: {kind[0]}" for name, kind in optional.items())
        else:
            result.append(f"Stream{name} = _{name}Required")
    result += [
        "UnknownStreamMessage = TypedDict(\"UnknownStreamMessage\", {\"op\": Literal[\"unknown\"], \"native\": Any})",
        "StreamMessage = " + " | ".join(f"Stream{name}" for name, *_ in STREAM_MESSAGES) + " | UnknownStreamMessage",
    ]
    return "\n".join(result) + "\n\n"


def ts_stream_messages():
    result = []
    for name, op, required, optional in STREAM_MESSAGES:
        fields = [f"op: {quoted(op)}"]
        fields += [f"{quoted(key)}: {kind[1]}" for key, kind in required.items()]
        fields += [f"{quoted(key)}?: {kind[1]}" for key, kind in optional.items()]
        fields.append("[field: string]: unknown")
        result.append(f"export interface Stream{name} {{ {'; '.join(fields)} }}")
    names = " | ".join(f"Stream{name}" for name, *_ in STREAM_MESSAGES)
    result.append(f'export type StreamMessage = {names} | {{op: "unknown"; native: LosslessValue}};')
    return "\n".join(result) + "\n\n"


PY_TYPES = '''# Generated by scripts/generate-hyperdash-bindings.py.
from typing import Any, Generic, Literal, TypeVar, TypedDict

T = TypeVar("T")

class HyperdashResponseMetadata(TypedDict, total=False):
''' + "\n".join(f"    {key}: {kind[0]}" for key, kind in METADATA.items()) + '''

class HyperdashResponse(TypedDict, Generic[T]):
    status: int
    metadata: HyperdashResponseMetadata
    data: T

StreamId = str | int
Channel = Literal[''' + ", ".join(map(quoted, CHANNELS)) + ''']
CandleInterval = Literal["1m", "5m", "15m", "1h", "4h", "1d"]

class _SubscribeRequired(TypedDict):
    channel: Channel

class SubscribeRequest(_SubscribeRequired, total=False):
    address: str
    coin: str
    interval: CandleInterval
    cursor: str

''' + py_stream_messages()

PY_CLIENT = '''class HyperdashStream:
    @property
    def closed(self) -> bool: ...
    async def subscribe(self, request: SubscribeRequest) -> None: ...
    async def next(self) -> StreamMessage | None: ...
    async def ack(self, sub: StreamId, batch: StreamId) -> None: ...
    async def unsubscribe(self, sub: StreamId) -> None: ...
    async def aclose(self) -> None: ...
    async def close(self) -> None: ...
    def __aiter__(self) -> HyperdashStream: ...
    async def __anext__(self) -> StreamMessage: ...
    async def __aenter__(self) -> HyperdashStream: ...
    async def __aexit__(self, *args: object) -> None: ...

class HyperdashClient:
    def __init__(self, api_key: str | None = None, *, api_url: str = ..., ws_url: str = ...,
                 timeout_ms: int = ..., stream_capacity: int = ...) -> None: ...
    @property
    def closed(self) -> bool: ...
    async def aclose(self) -> None: ...
    async def open_stream(self) -> HyperdashStream: ...
    async def __aenter__(self) -> HyperdashClient: ...
    async def __aexit__(self, *args: object) -> None: ...
'''

JS_RUNTIME = '''// Generated by scripts/generate-hyperdash-bindings.py.
"use strict";
const { native, encodeInput, decodeOutput, invoke } = require("./runtime.cjs");

class HyperdashStream {
  #native;
  constructor(inner) { this.#native = inner; }
  get closed() { return this.#native.closed; }
  subscribe(request) { return invoke(() => this.#native.subscribe(encodeInput(request))); }
  async next() {
    const value = await invoke(() => this.#native.next(), decodeOutput);
    return value === null ? {done: true, value: undefined} : {done: false, value};
  }
  ack(sub, batch) { return invoke(() => this.#native.ack(encodeInput({sub, batch}))); }
  unsubscribe(sub) { return invoke(() => this.#native.unsubscribe(encodeInput(sub))); }
  close() { return invoke(() => this.#native.close()); }
  async return() { await this.close(); return {done: true, value: undefined}; }
  [Symbol.asyncIterator]() { return this; }
}

class HyperdashClient {
  #native;
  constructor({apiKey, apiUrl, wsUrl, timeoutMs, streamCapacity} = {}) {
    const options = encodeInput({
      api_key: apiKey, api_url: apiUrl, ws_url: wsUrl, timeout_ms: timeoutMs, stream_capacity: streamCapacity
    });
    this.#native = invoke(() => new native.HyperdashClient(options));
  }
  get closed() { return this.#native.closed; }
  close() { return invoke(() => this.#native.close()); }
  openStream() { return invoke(() => this.#native.openStream(), inner => new HyperdashStream(inner)); }
  #request(method, request) {
    return invoke(() => this.#native.request(method, encodeInput(request)), decodeOutput);
  }
'''

TS_TYPES = '''// Generated by scripts/generate-hyperdash-bindings.py.
import type { LosslessValue, NativeValue } from "./index.js";

export interface HyperdashResponseMetadata {
''' + "\n".join(f"  {key}?: {kind[1]};" for key, kind in METADATA.items()) + '''
}

export interface HyperdashResponse<T> {
  status: number;
  metadata: HyperdashResponseMetadata;
  data: T;
}
export type StreamId = string | bigint;
export type Channel = ''' + " | ".join(map(quoted, CHANNELS)) + ''';
export type CandleInterval = "1m" | "5m" | "15m" | "1h" | "4h" | "1d";
export interface SubscribeRequest {
  channel: Channel;
  address?: string;
  coin?: string;
  interval?: CandleInterval;
  cursor?: string;
}
''' + ts_stream_messages()

TS_CLIENT = '''export class HyperdashStream implements AsyncIterableIterator<StreamMessage> {
  private constructor();
  readonly closed: boolean;
  subscribe(request: SubscribeRequest): Promise<void>;
  next(): Promise<IteratorResult<StreamMessage>>;
  ack(sub: StreamId, batch: StreamId): Promise<void>;
  unsubscribe(sub: StreamId): Promise<void>;
  close(): Promise<void>;
  return(): Promise<IteratorResult<StreamMessage>>;
  [Symbol.asyncIterator](): HyperdashStream;
}
export class HyperdashClient {
  constructor(options?: {apiKey?: string; apiUrl?: string; wsUrl?: string; timeoutMs?: number; streamCapacity?: number});
  readonly closed: boolean;
  openStream(): Promise<HyperdashStream>;
  close(): Promise<void>;
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    operations = inventory()
    for path, text in generate(operations).items():
        if path.exists() and path.read_text() == text:
            continue
        if args.check:
            raise SystemExit(f"Regenerate {path.relative_to(ROOT)} with {Path(__file__).name}")
        path.write_text(text)
    print(f"Hyperdash: {len(operations)} REST operations, {sum(op['text'] for op in operations)} paginated operations")


if __name__ == "__main__":
    main()
