# TypeScript SDK from Rust connectors — evaluation

Date: 2026-11-04
Scope: aux-rs workspace (pyo3 Python SDK exists at `crates/python`)

## Summary

Ship a native Node SDK first. A later browser SDK can reuse the protocol core, with separate transport and runtime work:

- **Browser (future)**: wasm-bindgen + wasm-pack build (`wasm32-unknown-unknown`). Audit clocks, randomness, crypto dependencies, binary frames, and browser scheduling before claiming portability.
- **Node/server**: napi-rs native module — the direct pyo3 equivalent. Full tokio/reqwest/tungstenite stack, no porting, auto-generated `.d.ts`.

Skip: UniFFI-TS (experimental), WASI-in-browser (too early), hand-rewritten TS connectors (drift across eight exchanges).

## Why the workspace is well prepared

- Exchange crates (`aux-rs-hyperliquid` etc.) are pure protocol logic: core + serde + crypto deps only, no tokio outside dev-deps. Wasm compatibility of the complete crypto dependency graph still needs a target build and runtime verification; several adapters also use native clocks and randomness.
- `aux-rs-core` already defines the I/O seams needed for a wasm port:
  - `HttpTransport` — `crates/core/src/transport.rs:50`
  - `WebSocketConnector` — `crates/core/src/transport.rs:88`
  - `WebSocketTransport` — `crates/core/src/transport.rs:185`
- The binding crate for TS mirrors `crates/python` structurally: concrete clients, error mapping, subscription handles.

## Wasm path — how it maps

1. **Verify exchange portability.** Signing, message construction, and matching logic are reusable, but `SystemTime`, `Instant`, randomness, and dependency features need a browser-target audit.
2. **Extract the session engine when browser work starts.** The pooling engine in `crates/core/src/standard/ws.rs` depends on tungstenite sockets, binary/control frames, Tokio scheduling, timers, and native clocks. The existing text-only `WebSocketConnection` contract is not sufficient as its replacement. Design a frame-capable internal connection seam plus browser scheduling/timer/clock support, preserving pooling, authentication replay, bounded queues, and gap behavior. Node bindings need none of this extraction.
3. **Binding crate** (`crates/wasm`, mirroring `crates/python`):
   - Expose concrete client instantiations (same surface as the pyo3 `pyclass` layer — no exported generics).
   - Implement `HttpTransport` over `web_sys::fetch`.
   - Browser-driven futures and timers. Tokio's wasm-compatible feature flags do not establish browser executor/timer compatibility; choose and verify the scheduling and clock adapters explicitly.
   - `wasm_bindgen_futures::future_to_promise` is the analog of `pyo3-async-runtimes`.

### Mechanical gotchas

- **`Send` bounds vs web-sys**: trait contracts require `Send` futures, but `JsValue`/`web_sys::WebSocket` are `!Send`. A single-thread wrapper may help satisfy bounds after auditing thread confinement. It does not solve browser runtime integration, clocks, timers, or binary/control-frame handling.
- **Rich TS types**: wasm-bindgen's generated `.d.ts` covers the API surface; use **tsify** or **ts-rs** to derive real interfaces/unions for event payloads from serde types. Cross the boundary with `serde-wasm-bindgen` rather than double-JSON.
- **u64/i64**: keep `.d.ts` `bigint` enabled (wasm-bindgen ≥0.2.9x) and bridge amounts as strings/BigInt — never default financial values to `f64`.

## Frontend constraints — what codegen does not fix

Wasm makes logic portable; browser platform constraints remain and determine whether this works "nicely on frontends":

- **CORS**: most exchange REST APIs send no CORS headers. Hyperliquid is browser-first and fine; the rest need a thin proxy (the `http_url` override already enables this). WebSocket upgrades generally work, but some exchanges reject browser `Origin` headers — verify per connector.
- **Secret custody**: keep L1/private signers out of web pages. Sane browser scope = public market data + explicitly delegated sessions (Hyperliquid agent wallets / approveBuilder; scoped JWTs on Paradex). Full private trading belongs in Node (napi-rs artifact) or behind a backend.
- **Worker topology**: run wasm in a **Web Worker**, no SharedArrayBuffer needed (single-threaded — avoids COOP/COEP cross-origin-isolation headers). Pump event batches to the main thread via `postMessage`, drained on rAF. Per-message JS↔wasm boundary crossings throttle hot orderbook feeds — batch in the subscription handle before crossing.
- **Bundle size**: TLS/HTTP come from the browser, so rustls is not compiled in. Measure real compressed artifacts. JavaScript submodules do not automatically remove exchange code from one compiled Wasm module; per-exchange Wasm artifacts or separate builds may be needed.

## Implementation sequence

1. Stand up `crates/node` with napi-rs, reusing standard transports and existing Rust/Python API inventories.
2. Generate model declarations with optional ts-rs derives; verify exact numeric conversions, callbacks, errors, streams, and cleanup.
3. Build and test supported native binaries and the installed npm package. Snapshot generated public declarations in CI.
4. Treat browser support as a separate portability project, beginning with target builds and transport/runtime design.
## Effort estimates (rough)

- Node SDK: estimate against eight-exchange coverage, model declarations, exact numeric conversions, async callbacks, subscription lifecycle, and multi-platform packaging. A weekend prototype does not establish release readiness.
- ws engine extraction: bounded, medium; benefits all targets.
- wasm crate + worker harness: the bulk of the work; dominated by browser-constraint verification per exchange (CORS, Origin headers).


## Implementation update: Node first

The initial implementation is `crates/node` (`aux-ts`, Node 22+), covering the eight exchanges currently present in the workspace. napi-rs provides native clients and promises; optional ts-rs derives generate payload declarations. The existing transport engine remains unchanged. The Node build workflow validates platform binaries and an installable npm tarball without publishing it.

Browser support remains a separate project. Beyond the WebSocket dialer, it must address binary/control frames, browser scheduling and timers, exchange clocks and randomness, and per-exchange browser network restrictions. A text-only `WebSocketConnector` is insufficient for the current pooling engine. Tokio's wasm-compatible feature flags do not establish browser runtime/timer compatibility. Bundle size and per-exchange elimination require measured builds. The initial weekend estimate does not cover eight-exchange API coverage, exact conversion, callbacks, cleanup tests, or platform packaging.

See `crates/node/README.md` for the implemented API, numeric rules, build commands, and platform limits.
