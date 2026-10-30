# Connectors SDK

Architecture and implementation blueprint for a standalone Rust exchange SDK, starting with Hyperliquid and Lighter. The SDK adapts exchange protocols into typed Rust clients. Python bindings will reuse the Rust implementation through PyO3.

**Status: milestones 1–5 implementations and the Python API parity follow-up are present; funded testnet verification remains pending.** Hyperliquid and Lighter expose native market/account reads and streams, Rust signing, replaceable auth handlers, and order/cancel submission. Hyperliquid supports native batches; Lighter starts with single create/cancel transactions and serializes submission per chain/account/API-key identity. Python now covers these implemented exchange operations, offline signing/verification, local/custom signers and cancellable streams using the same Rust clients. Public Rust and Python smoke checks passed both testnets on 2026-10-29; Hyperliquid matches 44 reference vectors and Lighter 66 pinned Go vectors. Both funded submit/cancel checks require unavailable credentials. Lighter's local cryptographic primitives are unaudited; security review and authenticated testnet checks remain release gates. See [README.md](README.md), [Python coverage](crates/python/README.md), [Lighter signing notes](crates/exchanges/lighter/SIGNING.md), and [DEPENDENCIES.md](DEPENDENCIES.md). Transport hardening is implemented; Python wheel distribution remains planned.

## Goals and boundaries

- Preserve each exchange's native operations, request types, responses, and errors.
- Provide ready-to-use HTTP and WebSocket clients with a small, deliberate dependency set.
- Let consumers select exchanges through Cargo features and replace transports, handlers, or signers.
- Implement exchange protocols and signing in Rust, including Lighter signing.
- Keep the Rust API suitable for thin Python bindings, with an early PyO3 feasibility check.

The SDK owns protocol adaptation, connection handling, request preparation, and signing. Strategies, order routing, risk management, portfolio accounting, persistent storage, funding and withdrawal workflows, and frontend integration are outside v1. It does not maintain an application order book or normalize exchanges into a common trading model.

The first usable release covers market metadata, order-book snapshots, public trades and book streams, account state, positions, open orders, order submission, cancellation, and account streams for both venues. Account stream access follows each venue's rules; an account-related stream is not necessarily authenticated.

## Architecture

Each exchange exposes a concrete client with inherent native methods. Shared endpoint execution and transport contracts live in core; construction and authentication belong to the exchange client.

| Component | Responsibility |
|---|---|
| `HyperliquidClient<T, A>`, `LighterClient<T, A>` | Combine exchange configuration, transport, authentication, and inherent native operations |
| `Hyperliquid`, `Lighter` | Configure native endpoints and signing network/chain context |
| `Exchange` | Supplies endpoint URLs and the native error type for typed request execution |
| Typed endpoint contracts | Associate a native request with its response, encoding, decoding, and access requirements |
| Transport traits | Execute HTTP requests and manage WebSocket connections independently of exchange codecs |
| Auth handlers | Prepare native nonces, credentials, and signed envelopes |
| Signers | Produce native signatures without owning transport or order execution |
| Subscription handles | Deliver native events and expose unsubscribe/close |

### Exchange clients and native operations

Clients expose operations directly, without importing extension traits. Exchange request, response, subscription, and error types remain in their respective namespaces; native batching, identifiers, and account semantics stay visible.

`testnet()` and `mainnet()` select built-in transports with default settings. `new(exchange)` accepts explicit endpoint configuration, `with_config(exchange, config)` controls the built-in transport, and `with_transport(exchange, transport)` accepts custom I/O. `with_auth(auth)` consumes a client and returns its authenticated state while retaining the transport.

Public methods work in both authentication states. Trading methods require native authentication bounds, preserving compile-time checks. Public account queries do not require a signer.

Clients share connection pools and WebSocket sessions across clones. Generic transport and signer contracts keep explicit `Send` guarantees without requiring `async-trait` or boxing every call. Core retains a shared public-endpoint executor; no generalized client builder or provider type is exposed.

### Request execution and handlers

```text
Native request
  -> validate
  -> prepare nonce / timestamp / context
  -> authenticate or sign
  -> execute through transport
  -> decode native response
```

Preparation is exchange-specific. Authentication runs after all signed fields have been finalized; subsequent transport handlers cannot mutate signed content. Optional timeout, retry, or observation wrappers operate around execution without changing the signed envelope. Handler order must be explicit and documented.

Keep signing independent from authentication: a signer produces a signature, while an auth handler knows how that signature, a token, or other credentials belong in the exchange protocol. Do not assume every signer accepts an Ethereum-style digest or returns the same signature representation. Local and caller-supplied signers implement the same venue-specific contract.

The default stack shares one nonce coordinator for clones and independently constructed auth handlers using the same signing identity. Hyperliquid nonce state is scoped to the signer, including when that signer trades for multiple accounts. Lighter sequencing is scoped to network, account, and API key, and coordinates allocation with submission ordering. Hyperliquid supports explicit caller-managed nonces. Lighter exposes offline preparation with explicit nonces; applications executing those envelopes outside the automatic client path own sequencing. Coordination across independent processes is the caller's responsibility.

### Values, errors, and outcomes

- Preserve exact native decimal strings or scaled integers for prices and quantities. Do not route them through floating point or silently round values to fit venue precision.
- Keep queried/traded account identity distinct from signer identity, including agent keys, vaults, and subaccounts.
- Preserve native error codes, messages, and batch item outcomes. A successful HTTP response may contain rejected operations; do not turn it into unconditional trading success.
- Distinguish transport, decoding, authentication/signing, exchange rejection, and subscription failures. Retain useful status and request context while excluding secrets.
- Surface an ambiguous submission outcome separately from a definite rejection. A timeout after sending an order does not prove that the exchange rejected it.

## Packaging and dependencies

This repository is a self-contained Cargo workspace, independent of frontend and RFQ services.

```text
connectors/
  PLANNING.md
  README.md
  Cargo.toml                    # aux-rs facade and workspace
  src/lib.rs                    # facade and re-exports
  examples/
  tests/
  crates/
    core/                       # aux-rs-core: contracts, clients, shared errors
    exchanges/
      hyperliquid/              # aux-rs-hyperliquid: native adapter and signer
      lighter/                  # aux-rs-lighter: native adapter and signer
    python/                     # aux-rs-python: PyO3 bindings
      python/aux_py/            # aux-py distribution; import aux_py
```

The workspace contains the facade, core, both implemented adapters, and Python bindings. Python is excluded from default workspace members. Publication is a later release task.

### Cargo feature contract

| Facade feature | Effect |
|---|---|
| `http` | Enables the standard Tokio/Reqwest HTTP implementation |
| `ws` | Enables the standard Tokio/Tokio Tungstenite WebSocket implementation |
| `hyperliquid` | Enables native Hyperliquid types, codecs, Rust signing, auth handlers, trading, and account methods |
| `lighter` | Enables the Lighter adapter, native types, codecs, and Rust signer |

Default features are `http` and `ws`; no exchange is selected automatically. Exchange features are additive and must not implicitly force the standard transport features. The facade forwards transport features to the relevant implementation modules and keeps internal adapter dependencies' default features disabled.

With `default-features = false`, core client contracts and selected adapters remain usable with caller-supplied transports. Standard transport implementations live behind features in the shared crate; core contracts must not require Tokio, HTTP clients, WebSocket clients, crypto implementations, or Python.

Cargo features are compile-time plugins. Dynamic discovery and runtime plugin loading are outside v1.

### Dependency policy

- Use Tokio, Reqwest, Tokio Tungstenite, Serde/Serde JSON, and only the necessary futures utilities for the standard stack. Disable unnecessary upstream default features and share TLS policy and configuration across HTTP and WebSockets.
- Keep Serde-based wire codecs and cryptographic dependencies with the adapters that need them. Selecting only Hyperliquid must not compile Lighter signing dependencies, and vice versa.
- Reuse focused cryptographic primitives and appropriate secret-memory handling. Do not depend on the Alloy umbrella package merely to obtain the architecture.
- Do not make Tower, tracing, configuration frameworks, generated SDKs, existing exchange client packages, or PyO3 mandatory dependencies.
- Record direct and transitive dependency baselines for core without defaults, Hyperliquid, Lighter, and both exchanges with standard transports. Review `cargo tree` and `cargo tree -e features` changes against those baselines.
- Explain each new production dependency by the capability it supplies. Avoid an arbitrary dependency-count cap that encourages replacing maintained primitives with unnecessary custom code.
- Choose compatible crate versions and record the minimum supported Rust version when the first manifests are added. This blueprint does not claim a tested version combination.

## Proposed usage

**These examples describe the full roadmap's API intent.** Working public, account, signing, and trading examples are documented in [README.md](README.md). Snippets below that refer to caller-supplied services or requests are illustrative. No example requires a secret committed to source control.

### Installation

For a consumer whose relative directory layout matches this repository:

```toml
[dependencies]
aux-rs = { path = "../connectors", features = ["hyperliquid"] }
# Select both exchanges with features = ["hyperliquid", "lighter"].
```

For custom networking, use `default-features = false` with the desired exchange features. PyO3 is supplied by the separate binding crate, rather than a feature that every Rust consumer must build.

### Public reads

```rust,ignore
use aux_rs::hyperliquid::HyperliquidClient;

let client = HyperliquidClient::testnet()?;
let metadata = client.meta().await?;
```

The constructors create reusable clients; network operations occur when calling endpoints or opening subscriptions. Network selection is explicit, and custom endpoints are supported for proxies and tests.

### Authenticated order submission

```rust,ignore
use aux_rs::hyperliquid::HyperliquidClient;

use aux_rs::hyperliquid::{HyperliquidAuth, LocalSigner};

let signer: LocalSigner = std::env::var("HL_AGENT_PRIVATE_KEY")?.parse()?;
let account = std::env::var("HL_ACCOUNT_ADDRESS")?.parse()?;
let client = HyperliquidClient::testnet()?.with_auth(HyperliquidAuth::agent(signer, account));

// Construct this request from Hyperliquid's native order/batch types.
let result = client.order(native_order_request).await?;
// Inspect each native item outcome; transport success is not order acceptance.
```

This assumes an already authorized agent key. Account provisioning and key approval workflows are not prerequisites that the client silently performs. Vault or subaccount context must be explicit where required by the venue.

### Subscriptions

```rust,ignore
use aux_rs::hyperliquid::Subscription;

let mut subscription = client
    .subscribe(Subscription::L2Book { coin: "ETH".into() })
    .await?;

while let Some(event) = subscription.next().await {
    // Handle native data, connection lifecycle, or stream errors explicitly.
    handle(event?);
}
subscription.unsubscribe().await?;
```

The handle must also support explicit close while the stream is active. Dropping it releases local resources; explicit unsubscribe/close allows callers to await remote and local cleanup.

### Custom signing

```rust,ignore
use aux_rs::hyperliquid::HyperliquidClient;

// Implement the venue's Signer contract on this application-owned type.
let signer = RemoteHyperliquidSigner::new(signing_service);
let client = HyperliquidClient::testnet()?.with_auth(HyperliquidAuth::agent(signer, account));
```

A custom Hyperliquid signer supplies its signing identity and a `Send`-compatible asynchronous signing operation over the typed Hyperliquid signing payload, returning its native signature or a signing error. Request encoding and account/nonce preparation stay in the adapter. Lighter defines its own signing payload and signature contract instead of inheriting an Ethereum signer API.

## Delivery milestones

### 1. Foundation and Hyperliquid public APIs

**Implemented and verified.** The workspace includes the facade, dependency-free core contracts, and Hyperliquid adapter. Offline fixtures/local-server tests cover both transport implementations; a public testnet smoke example validates metadata, snapshots, and both stream types. See `scripts/check.sh` for the supported feature matrix. Milestone 5 now adds safe-read retries, recoverable gaps and reconnect/resubscribe on top of this foundation.

Implement the workspace, feature forwarding, client construction, typed endpoint contracts, injectable transports, and Hyperliquid native types. Deliver market metadata, book snapshots, public trade streams, and public book streams.

Add sanitized fixtures and mock HTTP/WebSocket tests before authenticated operations. Validate exchange-client constructor and inherent-method ergonomics in examples. Establish dependency baselines and check the standard-stack and custom-transport configurations.

Exit criteria: public reads and streams work against testnet, fixture tests run offline, and the Hyperliquid build contains no Lighter or Python dependencies.

### 2. Hyperliquid trading and account streams

**Implemented, with authenticated testnet verification outstanding.** Typed account/position and open-order reads remain unsigned. Account streams cover state, open orders, order updates, and fills, with account/aggregation connection scopes to avoid ambiguous routing. Native L1 action signing uses canonical ordered MessagePack, K256, and Keccak; offline user-signed primitives cover the second scheme without adding transfer/provisioning submission workflows. The pinned reference revision is `2fdb18f9517675ea03695a0962bd19eece9c83f0`.

Native `Signer` and `AuthHandler` contracts are replaceable. Signed envelopes are immutable and verified against signer/network context. One process-lifetime nonce counter per signer is shared by independent handlers and vault clones; explicit nonces bypass it. Offline tests cover reference vectors, altered signing context, partial batch outcomes, concurrency, custom handlers, ambiguous outcomes, and account isolation. Public account smoke checks passed; the opt-in authenticated submit/cancel example has not been run because credentials are unavailable.

Add account state, positions, open orders, order submission, cancellation, and account streams. Support the native account and signer context without conflating agent addresses with account addresses.

Implement Rust signing against a pinned revision of the official SDK. Reference vectors must cover field ordering, exact numeric representation, network/domain context, vault/subaccount context, and both native signing schemes. The [official signing documentation](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/signing) calls out these compatibility pitfalls. Follow the venue's [signer-scoped nonce rules](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/nonces-and-api-wallets).

Exit criteria: signed fixtures match the reference, concurrent clones do not collide on nonces, and explicitly configured testnet credentials can submit and cancel an order while exposing native outcomes.

### 3. Early Python feasibility check

**Implemented and verified locally.** `aux-rs-python` contains concrete owned PyO3 wrappers over the existing Rust clients, with thin Python asyncio/context-manager methods. Metadata and L2 HTTP reads preserve native keys, exact decimal strings, and integer IDs. The book iterator consumes subscription acknowledgements, surfaces native/transport/decode errors, and closes on cancellation, explicit close, client shutdown, or idle-object drop.

Maturin local installation passed on CPython 3.12.7/macOS arm64. The initial sixteen offline tests cover exact values, responsive asyncio scheduling during Rust I/O, structured errors, single-reader enforcement, cancellation, concurrent shutdown, drop cleanup, overflow, and disconnects. The Python public testnet example read metadata and an HTTP book, consumed a WebSocket snapshot, and closed both objects. Rust-only default members/checks exclude Python; dependency profiles enforce that PyO3 cannot enter Rust-only builds. Signing/trading/account and Lighter bindings are now implemented in the Python parity follow-up below; a release wheel matrix remains deferred.

Add a thin PyO3 crate and validate one async public HTTP call plus one cancellable subscription. Expose owned, concrete Python client wrappers over the Rust clients; keep Python-specific annotations and objects out of the SDK's core and exchange types.

PyO3 classes cannot directly expose generic or lifetime-parameterized Rust types; see its [class restrictions](https://pyo3.rs/main/class.html). Bridge the Tokio and asyncio lifecycles using the documented [async integration](https://pyo3.rs/main/ecosystem/async-await.html), with `pyo3-async-runtimes` confined to the binding crate. Use Maturin for the initial local extension build.

Preserve exact prices and quantities, map structured errors to Python exceptions, and release interpreter access during Rust work. Python cancellation must close the Rust subscription and release its resources.

Exit criteria: a local Python test awaits a read, consumes a stream event, cancels the stream, and completes cleanup. Full client coverage, distribution packaging, and wheels follow after the Rust foundation.

### 4. Lighter public APIs and Rust signing

**Implemented, with funded authenticated testnet verification and cryptographic review outstanding.** The opt-in adapter covers metadata, native book/trade/account reads and streams, active orders, single create/cancel submission, private order streams, and Rust auth tokens. The Go reference is pinned to `ebae8edecfee9986b0a5fcde3cb9a06bd4af3a3a`, with 66 checked-in vectors and Go verification of Rust signatures. The transaction Poseidon2 variant is ported with attribution; auth tokens and Schnorr challenges use the original variant. Local ECgFp5/Poseidon primitives are explicitly unaudited and remain a production-review gate. See [the adapter guide](crates/exchanges/lighter/README.md) and [signing notes](crates/exchanges/lighter/SIGNING.md).

One coordinator per chain/account/API-key identity serializes nonce lookup through submission. Cancellation, rejection, malformed responses and transport failure quarantine writes until explicit reconciliation/resync. Native integer units and exact scaling, custom signers/handlers/transports, HTTP form encoding, token deadlines, JSON pong handling, fresh snapshots for late stream consumers, overflow, and cleanup are tested offline. All public HTTP and book/trade/account WebSocket flows passed testnet. Funded submit/cancel and real private order-stream checks have not run; the opt-in example is ready. Nonce skipping, grouped/batch submission, key registration and other Lighter transaction families are not exposed.

Add Lighter-native types, metadata, book snapshots, public streams, account reads, order submission, cancellation, and account streams. Implement signing and auth-token generation in Rust. Do not wrap the official Go shared library or invoke a Python SDK at runtime.

Pin the [official Go signing and hashing reference](https://github.com/elliottech/lighter-go) to a specific revision. Reuse reviewed Rust primitives where available and port missing protocol logic with provenance and license attribution. Committed fixtures record the pinned reference revision and source hashes; normal tests use those fixtures offline. Reference-generation tools are not included in this repository.

Validate encodings, hashes, signatures, token expiry, network separation, and account/API-key nonce sequencing before enabling trading. Compare deterministic outputs exactly; for nondeterministic signatures, verify reference acceptance rather than requiring identical signature bytes. Treat signer compatibility as a separate milestone gate, not an assumption about generic EVM signing.

Exit criteria: reference compatibility tests and testnet trading pass; both venue features coexist without requiring either venue's signing implementation for the other.

### Python API parity follow-up

**Implemented and verified locally.** Owned concrete PyO3 wrappers now cover the implemented Rust HTTP, account/position/order, trading and WebSocket operations for both venues. Native requests/responses retain their schemas, batch outcomes, exact values and error codes. The original bare Hyperliquid book iterator is compatible; generic iterators preserve native envelopes and acknowledgements. Rust SDK types remain free of Python annotations/dependencies.

Local and custom async signers expose immutable signing payloads, offline action/transaction signing, auth tokens and signature verification. Callbacks run on the initiating asyncio loop with its context and are cancelled with native work. Non-integral JSON numbers become Python `Decimal`, large integer IDs stay Python integers, and request floats/cycles are rejected. Shared Rust nonce coordination, ambiguous-submission quarantine, explicit recovery and no-replay rules remain in force. Credential/client owners are dropped outside Rust locks so Python callback destructors can reenter safely.

All 37 offline Python tests pass, including 44 Hyperliquid and 66 Lighter reference vectors, API/stream coverage, callback context/identity/cancellation, partial batch failures, exact conversion, nonce concurrency, uncertain outcomes, stream faults and cleanup. The expanded Python public smoke example passed both testnets on 2026-10-29. Funded trading/private order-stream checks still need credentials. Python runtime dependencies remain standard-library-only; the binding crate selects both Rust adapters, while Rust-only dependency profiles remain isolated. See [the Python guide](crates/python/README.md) for coverage, examples, stubs and remaining distribution scope.

### 5. Release hardening

**Implemented, with funded live checks and cryptographic review still separate release gates.** Explicit networks/endpoints, timeouts and bounded buffers are complemented by safe-read opt-in retries, RFC `Retry-After` handling, no-replay trading, bounded reconnect/resubscribe, out-of-band gap reporting with stale-buffer invalidation, fresh Lighter snapshot enforcement, expiry checks and deterministic shutdown across clones. Meaningful data or a quiet stable period resets the reconnect budget; handshakes/acks alone do not reset rapid failures. Overflow/protocol/size faults remain terminal.

The standard transport exposes synchronous shutdown signalling and an awaited shutdown that drains handshakes/HTTP/socket work, performs bounded unsubscribe/close and releases pools/registries. Python preserves recoverable `GapError` handling without closing the iterator, while cancellation and terminal errors close it. All 42 Python tests and the Rust feature/fault matrix pass. Tests simulate rate limits, connection loss, uncertain writes, malformed frames, replay prevention, expiry, reconnect budgets, shutdown during recovery/initial dialing and cleanup. See [HARDENING.md](HARDENING.md) for defaults and application reconciliation responsibilities. Private token refresh, funded authenticated testnet checks, Lighter cryptographic review and release wheels remain explicit later work.

- Support explicit networks, custom endpoints, configurable timeouts, and bounded stream buffers.
- Handle heartbeats and reconnect/resubscribe with bounded backoff. Report connection discontinuities and overflow explicitly; do not imply uninterrupted delivery or silently discard book deltas.
- Deliver native snapshots and deltas without assembling a persistent local book. Callers own book reconstruction and reconciliation after gaps.
- Provide unsubscribe, explicit shutdown, and cleanup on cancellation or drop.
- Allow bounded retries for safe reads. Never automatically replay trading submissions after ambiguous failures or consume a new nonce to disguise a replay.
- Keep credentials, auth tokens, and signing secrets out of debug output and errors.
- Maintain a tested feature matrix and update dependency baselines before release.

Exit criteria: the fault scenarios below pass and the venue checklist accurately reflects implemented coverage.

## Exchange backlog

Target venues are listed in the requested order. Checked items have implemented Rust adapters and Python operation bindings; outstanding release checks remain tracked separately below. Priority and scheduling for additional venues are TBD.

- [x] **Hyperliquid** — implemented; funded authenticated testnet verification remains pending.
- [x] **Lighter** — implemented; funded authenticated testnet verification and cryptographic review remain pending.
- [ ] **Aster**
- [ ] **GRVT**
- [ ] **Antarctic**
- [x] **QFEX** — core native Rust/Python operations implemented; funded UAT trading verification pending.
- [ ] **EdgeX**
- [x] **Arcus** — core perpetuals Rust/Python operations implemented; funded testnet trading verification pending.
- [ ] **Ondo Perps**
- [ ] **Extended**
- [ ] **Pacifica**
- [ ] **Bulk**
- [ ] **Paradex**

For each new venue, track protocol/authentication research, a native Rust adapter and selectable Cargo feature, public reads and streams, Rust signing and account/trading operations, Python bindings, fixture/reference tests, dependency isolation, and testnet verification. Apply the existing architecture, dependency policy, and acceptance checks to each adapter.

## Venue coverage checklist

The new adapters cover core native operations; extended workflows are listed in each exchange guide.

| Capability | Hyperliquid | Lighter | QFEX | Arcus |
|---|---|---|---|---|
| Market metadata and book snapshots | [x] | [x] | [x] | [x] |
| Public trades and book streams | [x] | [x] | [x] | [x] |
| Account state and positions | [x] | [x] | [x] | [x] |
| Native order reads | [x] | [x] | [x] | [x] |
| Rust signing and native authentication | [x] | [x] | [x] | [x] |
| Order submission and cancellation | [x] | [x] | [x] | [x] |
| Account streams | [x] | [x] | [x] | [x] |
| Authenticated live testnet trading | Pending | Pending | Pending | Pending |

QFEX uses isolated authenticated WebSocket trade sessions. No commands replay on reconnect; an interrupted command remains pending until its native response is reconciled. Public MDS streams reuse the standard subscription transport. Arcus uses native Ed25519 typed order payloads, exact market tick/quantum conversion, monotonic nanosecond timestamps, and single-attempt REST writes. Public account subscriptions are scoped by address and account index; delta books require fresh snapshots after gaps.

Both new exchanges have Rust protocol/signing tests, local connection lifecycle tests, Python bindings, and offline Python parity tests. Public testnet fixtures were captured on 2026-10-30. The source contracts and narrower current coverage are recorded in their exchange READMEs. Live funded trading remains unverified for all venues; reference compatibility does not imply a cryptographic audit. No signing reference-generation scripts were reintroduced.

## Acceptance checks

| Area | Required scenarios |
|---|---|
| Build and features | Core without defaults; each venue alone; both venues together; HTTP-only and WebSocket-only standard stacks; custom transports without standard-stack features; no PyO3 in Rust-only dependency trees |
| Native protocols | Sanitized request/response fixtures, exact decimals and scaled integers, invalid precision rejection, native errors, successful HTTP responses containing rejected items, and mixed batch outcomes |
| Signing | Pinned reference vectors, negative vectors, altered payloads, network/domain separation, signer/account separation, concurrent nonce use, and Lighter submission sequencing |
| HTTP failures | Rate limits, bounded read retries, pre-send failures, timeouts after submission, definite rejection versus unknown outcome, and no automatic trading replay |
| WebSocket lifecycle | Subscription acknowledgements, malformed frames, heartbeats, disconnects, backoff, resubscription, explicit gap reporting, overflow, unsubscribe, cancellation, and resource cleanup |
| Live verification | Public testnet reads/streams and opt-in authenticated testnet submit/cancel flows; no live mainnet orders in CI |
| Python boundary | Exact value preservation, structured error mapping, nonblocking async integration, cancellation propagation, and deterministic stream cleanup |

Use offline fixtures and local mock servers for normal CI. Authenticated testnet checks require explicit credentials and invocation; their availability must not determine whether the offline suite passes. Once the crates exist, run formatting, Clippy, and tests for the supported feature combinations.

Run `bash scripts/check.sh` for Rust formatting, Clippy, offline protocol/signing/trading tests, feature combinations, and dependency baselines. Run `bash scripts/check-python.sh` with a configured Python venv for binding checks and local-server tests. Public testnet verification explicitly invokes the Rust public/account examples (`hyperliquid_public`, `hyperliquid_account`, `lighter_public`) or `crates/python/examples/public_testnet.py`. Authenticated testnet submit/cancel verification explicitly invokes `hyperliquid_testnet_trade -- --execute` or `lighter_testnet_trade -- --execute` with credentials. None of these live checks run in offline CI. Automatic reconnection and fault checks run offline in milestone 5; Lighter's fixture provenance is documented separately in its signing notes.

## Chosen defaults

The SDK is standalone, starts with Hyperliquid, exposes native client methods, selects exchanges through Cargo features, and includes ready-to-use networking by default. Exchange signing stays in Rust throughout. Python gets an early feasibility check and a separate binding crate; full distribution follows the Rust foundation.
