# Node TypeScript SDK

`aux-ts` exposes the existing Rust connectors through napi-rs. It includes Hyperliquid, Lighter, QFEX, Arcus, Monaco, Derive, Paradex, and Metric, plus signer classes, RPC sessions, and offline signing helpers. Protocol encoding, cryptography, authentication, nonce coordination, and networking run in Rust.

Node 22+ is required. Release artifacts contain native binaries for macOS arm64/x64 and Linux x64 GNU. Browser, musl, Windows, Linux arm64, and edge-runtime support are outside this release.

## Use

```ts
import { HyperliquidClient } from "aux-ts";

const client = new HyperliquidClient(); // testnet
try {
  const book = await client.l2Book({ coin: "ETH" });
  console.log(book.time, book.levels[0][0]?.px);

  const subscription = await client.subscribeL2Book({ coin: "ETH" });
  try {
    for await (const update of subscription) {
      console.log(update);
      break; // iterator return() closes the subscription
    }
  } finally {
    await subscription.close();
  }
} finally {
  await client.close();
}
```

CommonJS is also supported: `const { HyperliquidClient } = require("aux-ts")`.

Methods and constructors accept one object with camelCase argument names. Native exchange payload fields retain their Rust serde names: for example Hyperliquid order fields remain `a`, `b`, `p`, `s`, `r`, and `t`. Use the generated declarations for each method's options. Metric clients require `chainId`; custom Monaco endpoints also require `chainId`.

## Exact values

Prices and quantities that are strings in Rust remain strings. Typed `u64`/`i64` and wider integers return `bigint`; smaller integer types return `number`. Use `bigint` for large input integers. Unsafe JavaScript integers, fractional `number` inputs, cyclic structures, and implicit object coercion are rejected before native calls.

Arbitrary JSON integers return safe numbers or bigints. Fractional JSON numeric tokens return exact strings. Use `exactNumber("0.1000000000000000001")` when an exchange request requires a numeric JSON token rather than a quoted decimal string. `NativeInput<T>` describes these request values separately from decoded response types. `JSON.stringify` cannot directly serialize output containing bigints.

The private native interface uses an owned, tagged value encoding so Tokio tasks never hold JavaScript handles and integer widths survive conversion. This introduces serialization work; no throughput or latency claim has been established for high-volume feeds.

## Signers and errors

Local and async custom signers are supported. For example, `HyperliquidSigner.remote({ address, callback })` calls the async callback with the digest and signing context and expects a native signature object. Return values are validated by the existing Rust signer implementation. Callbacks execute on Node's thread; local cryptography remains native. Closing a client stops waiting for a pending callback; JavaScript promises themselves are not cancellable.

Errors preserve categories such as `ValidationError`, `ExchangeError`, `TransportError`, `GapError`, `NonceUncertainError`, and `OutcomeUncertainError`. Available metadata includes `kind`, `outcome`, `httpStatus`, and exchange details. Error messages omit raw server bodies; explicitly inspecting error metadata can expose server text. Failed or ambiguous writes are never retried by the binding.

A subscription permits one pending `next()` call. Concurrent reads fail with `BusyError`. Overflow terminates the stream; reconnect gaps raise `GapError` before fresh events. To recover after a gap, catch the error from `next()` and continue reading the same subscription. Ordinary `for await` exits when `next()` rejects. Explicit subscription/client close wakes pending reads. Do not rely on garbage collection for prompt shutdown.

Metric contract interfaces are available under `contracts`, generated from the same pinned ABIs as the Rust and Python bindings. `metricTools` exposes named Rust arithmetic and transaction helpers; `metricTool({name, request})` is also available.

## Build and verify

From the repository root:

```sh
npm ci --prefix crates/node
python3 scripts/generate-node-bindings.py --models
npm --prefix crates/node run build
npm --prefix crates/node test
npm --prefix crates/node run typecheck
node scripts/check-node-package.mjs
```

`bash scripts/check-node.sh` runs the full Node check suite. No tests require exchange access, credentials, or funded accounts. The ordinary Rust dependency profiles do not enable `typescript`, napi-rs, or ts-rs.

`src/api.json` records the concrete binding surface. `src/models.json` records the exported Rust model paths. Update these inventories when adding public operations/models, then regenerate. Derive/Paradex operation wrappers and Metric contract helpers are generated alongside their existing Rust/Python counterparts. Source schemas are not needed to build the checked-in SDK.

The build workflow produces and tests an npm tarball containing all supported native binaries. It does not publish to npm. Package naming, registry ownership, and publication are release decisions; `aux-ts` is the local package name.

The Node crate denies handwritten unsafe code. napi-rs' generated FFI requires its macro-scoped allowance, so this boundary uses `deny` rather than the workspace's `forbid`; the protocol crates retain their existing lint policy.
