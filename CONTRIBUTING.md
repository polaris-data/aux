# Contributing

This guide covers workspace setup, architecture, contribution checks, and remaining
work. Read the [domain terminology](CONTEXT.md) before changing shared interfaces.
Use the [SDK guide](README.md), [exchange guides](README.md#features), and
[Python guide](crates/python/README.md) for working examples and supported operations.

## Workspace

Requires Rust 1.92 or later. Run commands below from the repository root, which
contains the Cargo workspace configuration and committed lockfile.

| Directory | Purpose |
| --- | --- |
| `crates/aux-rs/` | Public SDK facade, examples, and integration tests |
| `crates/core/` | Shared client and transport contracts |
| `crates/exchanges/` | Native exchange clients, types, and signing |
| `crates/python/` | Separate Python bindings |

The workspace is independent of frontend and RFQ services. Rust-only default
members exclude the Python crate.

## Architecture

The SDK preserves each exchange's native operations, request types, responses,
errors, identifiers, and batching semantics. It owns protocol adaptation,
connection handling, request preparation, and Rust signing. Strategies, order
routing, risk management, portfolio accounting, persistent storage, funding and
withdrawal workflows, and frontend integration are outside v1. It does not
normalize venues into a common trading model or maintain an application order book.

### Clients and contracts

Each exchange exposes a concrete client with inherent native methods; consumers
should not need extension traits to use native operations. Exchange request,
response, subscription, and error types stay in their respective namespaces.

| Component | Responsibility |
| --- | --- |
| Exchange client | Combine exchange configuration, transport, authentication, and native operations |
| Exchange configuration | Select native endpoints and signing network/chain context |
| `Exchange` | Supply endpoint URLs and the native error type for typed request execution |
| Typed endpoint contracts | Associate native requests with responses, codecs, and access requirements |
| Transport traits | Execute HTTP requests and manage WebSocket connections independently of exchange codecs |
| Auth handlers | Prepare native nonces, credentials, and signed envelopes |
| Signers | Produce native signatures without owning transport or order execution |
| Subscription handles | Deliver native events and expose unsubscribe/close |

Clients support explicit networks and custom endpoints, configurable built-in
transports, and caller-supplied I/O. For Hyperliquid and Lighter, `testnet()` and
`mainnet()` use default built-in transports, `new(exchange)` accepts exchange
configuration, `with_config(exchange, config)` configures the standard stack,
and `with_transport(exchange, transport)` injects custom I/O. `with_auth(auth)`
consumes a client and returns its authenticated state while retaining its transport.

Public methods work in both authentication states. Trading methods require native
authentication bounds, preserving compile-time checks. Public account queries do
not require a signer; account streams follow each venue's access rules.

Clones share HTTP connection pools and compatible WebSocket sessions. Transport
and signer contracts keep explicit `Send` guarantees without requiring
`async-trait` or boxing every call. Core retains a shared public-endpoint executor;
there is no generalized client builder or provider type.

### Request preparation, signing, and nonces

```text
Native request
  -> validate
  -> prepare nonce / timestamp / context
  -> authenticate or sign
  -> execute through transport
  -> decode native response
```

Preparation is exchange-specific. Finalize all signed fields before authentication;
transport handlers must not mutate signed content. Timeout, retry, or observation
wrappers operate around execution without changing the signed envelope. Keep
handler order explicit and documented.

A signer produces a native signature; an auth handler knows how that signature,
a token, or other credentials belong in the protocol. Do not assume every signer
accepts an Ethereum-style digest or returns the same signature representation.
Local and caller-supplied signers implement the same venue-specific contract.
Account provisioning, key approval, and vault/subaccount context must be explicit.

Within one process, clones and independently constructed auth handlers using the
same signing identity share nonce coordination:

- Hyperliquid scopes its counter to the signer, including when trading for multiple
  accounts. Explicit caller-managed nonces bypass automatic allocation.
- Lighter scopes sequencing to network, account, and API key, serializing nonce
  lookup through submission. Cancellation, rejection, malformed responses, and
  transport failures quarantine writes until reconciliation and `resync_nonce()`.
  Offline envelopes with explicit nonces leave sequencing to the application.
- Coordination across independent processes is the caller's responsibility.

Implement signing against pinned official references with provenance and license
attribution. Commit sanitized compatibility vectors and source hashes. Normal
tests consume fixtures offline; reference-generation tools are not included in
this repository. Compare deterministic outputs exactly and verify reference
acceptance for nondeterministic signatures. Reference compatibility does not imply
a cryptographic audit. See the exchange guides and
[Lighter signing notes](crates/exchanges/lighter/SIGNING.md) for references and limits.

### Values, errors, and submission outcomes

- Preserve exact native decimal strings or scaled integers for prices and
  quantities. Reject invalid precision rather than silently rounding or routing
  values through floating point.
- Keep queried/traded account identity distinct from signer identity, including
  agent keys, vaults, and subaccounts.
- Preserve native error codes, messages, and batch item outcomes. A successful HTTP
  response may contain rejected operations and does not imply trading success.
- Distinguish transport, decoding, authentication/signing, exchange rejection, and
  subscription failures. Preserve useful structured context while excluding
  credentials, auth tokens, and signing secrets from diagnostics.
- Separate ambiguous submission outcomes from definite rejection. A timeout after
  sending an order does not prove rejection. Never automatically replay trading
  submissions or allocate a new nonce to disguise a replay.

### Transport lifecycle

Standard transports provide configurable timeouts and bounded buffers, safe-read
opt-in retries, heartbeats, bounded reconnect/resubscribe, and explicit shutdown.
Custom transports own their recovery and lifecycle policies. Shared sessions must
respect native account, signer, channel, and protocol scopes so events and errors
cannot reach an unrelated consumer.

Report connection discontinuities and overflow explicitly. Clear stale queued
frames after continuity is lost, deliver a recoverable gap before recovery data,
and require fresh snapshots where the native delta protocol needs them. Callers
own book reconstruction and reconciliation. Late consumers must not receive
cached snapshots as though they were fresh.

Meaningful native data or a quiet stability period resets the reconnect budget;
handshakes and acknowledgements alone must not permit unlimited rapid failures.
Overflow, protocol/size faults, expired replay credentials, and exhausted recovery
are terminal. Private token refresh remains application-owned.

Subscriptions support unsubscribe/close, cancellation, and drop cleanup. Explicit
shutdown drains handshakes, requests, and socket work and releases shared pools and
registries within bounded deadlines. QFEX trading uses isolated authenticated
WebSocket sessions: interrupted commands remain pending until their native
responses are reconciled, and trade sessions never reconnect or replay commands.

### Python boundary

Keep Python-specific annotations and objects out of core and exchange types. Owned,
concrete PyO3 clients wrap the same Rust clients, auth, signing, nonce coordination,
and transports; Python contains asyncio/context-manager glue rather than protocol
implementations. PyO3 cannot directly expose generic or lifetime-parameterized
Rust classes. Keep `pyo3-async-runtimes` confined to the binding crate and use Maturin
for local extension builds.

Preserve native schemas and batch outcomes. Non-integral JSON numbers become
Python `Decimal`, large IDs remain integers, and request floats/cycles are rejected.
Map structured errors to Python exceptions and release interpreter access during
Rust work. Custom async signer callbacks use the initiating asyncio loop and its
context and are cancelled with native work. Drop credential/client owners outside
Rust locks so Python callback destructors can reenter safely.

Cancellation closes the Rust subscription and releases resources. Terminal errors
close iterators; recoverable `GapError` leaves them open for continued polling.
Explicit context managers provide deterministic cleanup. Runtime Python
dependencies remain standard-library-only; build and test tools are development
dependencies. See the [Python guide](crates/python/README.md) for coverage and
lifecycle examples.

## Rust checks

```sh
bash scripts/check.sh
```

This runs formatting, Clippy with warnings denied, feature-isolated compilation
and tests, offline fixture and local-server tests, and dependency isolation checks.
It also checks that unsigned clients cannot trade. Normal checks make no exchange
requests and place no orders.

Rust-only workspace defaults and this script exclude the Python crate. PyO3 must
remain absent from every Rust-only dependency profile, and exchange features must
not pull in unselected adapters or their signing dependencies.

## Python checks

Use a dedicated Python 3.10+ virtual environment. For example, with Python 3.12:

```sh
python3.12 -m venv .venv-python
.venv-python/bin/pip install -r crates/python/dev-requirements.txt
bash scripts/check-python.sh
```

The script runs formatting, Python-crate Clippy, a local Maturin development
build, Python tests, and dependency checks. It uses fixtures and local mock
servers. Set `AUX_PYTHON=/path/to/venv/bin/python` to select another environment.

See the [Python guide](crates/python/README.md) for API coverage, examples, and
platform limits. CPython 3.12.7 on macOS arm64 has been validated locally; the
[release workflow](crates/python/README.md#releases) checks the wheel matrix and
source distribution before publishing to PyPI.

## Dependency changes

Default facade features are `http` and `ws`; no exchange is selected automatically.
Exchange features (`hyperliquid`, `lighter`, `qfex`, and `arcus`) are additive and
must not force standard transport features. Keep internal adapter dependencies'
default features disabled. With `default-features = false`, selected adapters
remain usable with caller-supplied transports. Core contracts must not require
Tokio, HTTP/WebSocket clients, crypto implementations, or Python. Cargo features
are compile-time plugins; runtime discovery and plugin loading are outside v1.

Use the standard Tokio, Reqwest, Tokio Tungstenite, Serde/Serde JSON, and focused
futures utilities where needed. Disable unnecessary upstream defaults and share
TLS policy across HTTP and WebSockets. Keep codecs and cryptographic dependencies
with the adapters that need them. Reuse focused maintained primitives and
appropriate secret-memory handling; do not introduce an Alloy umbrella dependency
merely for its architecture. Do not make Tower, tracing, configuration frameworks,
generated SDKs, existing exchange clients, or PyO3 mandatory dependencies.

Explain each new production dependency by the capability it supplies. Avoid an
arbitrary dependency-count cap that encourages unnecessary custom code. Keep crate
versions compatible and the minimum supported Rust version accurate.

Inspect feature activation and run the dependency isolation checks after changing
dependencies:

```sh
cargo tree --locked -p aux-rs --no-default-features --features hyperliquid -e features
python3 scripts/dependencies.py --check
```

Review changes to the manifests and lockfile before committing. The checks use
`aarch64-apple-darwin` by default; pass `--target` to check another platform.
Profiles cover core, each exchange, exchange combinations, custom and standard
transports, and the separate Python binding. Automated checks reject
cross-exchange signing and Rust-only Python dependency leaks.

Core has zero dependencies without features. Native identifiers and signatures
use shared hex helpers; Lighter transaction signatures use a private Base64
encoder. QFEX's private HMAC-SHA256 helper uses sha2, and standard HTTP uses a
private HTTP-date parser for `Retry-After`. These implementations record pinned
upstream sources, protocol references, and license notices. Use `cargo tree` to
inspect transitive dependencies, including Reqwest's Tower and tracing crates.

## Adding or extending an exchange

Track native protocol/authentication research, a Rust adapter and selectable Cargo
feature, public reads and streams, Rust signing and account/trading operations,
Python bindings, fixture/reference tests, dependency isolation, and testnet
verification. Keep native codecs and signing in Rust; do not wrap an official
Go shared library or invoke another SDK at runtime.

Apply these acceptance checks to each adapter and shared-stack change:

| Area | Required scenarios |
| --- | --- |
| Build and features | Core without defaults; each venue alone and combined; HTTP-only and WebSocket-only stacks; custom transports without standard networking; no PyO3 in Rust-only dependency trees |
| Native protocols | Sanitized request/response fixtures, exact decimals and scaled integers, invalid precision rejection, native errors, HTTP success containing rejected items, mixed batch outcomes |
| Signing | Pinned reference and negative vectors, altered payloads, network/domain separation, signer/account separation, concurrent nonces, native submission sequencing |
| HTTP failures | Rate limits, bounded read retries, pre-send failures, timeouts after submission, rejection versus unknown outcome, no trading replay |
| WebSocket lifecycle | Acknowledgements, malformed frames, heartbeats, disconnects, bounded backoff, resubscription, fresh snapshots, explicit gaps, overflow, expiry, unsubscribe, cancellation, drop, shutdown during handshakes/recovery, cleanup |
| Python boundary | Exact values, structured errors, nonblocking asyncio integration, callback context/cancellation, single-reader enforcement, deterministic cleanup |
| Live verification | Public testnet reads/streams and opt-in authenticated submit/cancel; no live mainnet orders in CI |

Normal CI uses committed fixtures and local mock servers. Availability of testnet
credentials must not determine whether the offline suite passes.

## Live verification

Public testnet examples are explicit read-only network checks, separate from the
normal scripts. See the [examples](crates/aux-rs/examples) and exchange guides for
commands and supported endpoints. Public Rust examples include `hyperliquid_public`,
`hyperliquid_account`, `lighter_public`, `qfex_public`, and `arcus_public`; the
[Python public example](crates/python/examples/public_testnet.py) covers
Hyperliquid and Lighter.

Authenticated trading examples require registered, funded testnet accounts and an
explicit `--execute` flag. Invoke `hyperliquid_testnet_trade -- --execute` or
`lighter_testnet_trade -- --execute` with the corresponding credentials and Cargo
features. Inspect native outcomes and reconcile uncertain submissions. These live
checks never run in offline CI.

## Roadmap and release gates

Hyperliquid, Lighter, QFEX, and Arcus have native Rust adapters and Python bindings
for core market metadata, book snapshots, public trades/book streams, account
state/positions, native order reads, signing/authentication, order submission and
cancellation, and account streams. Coverage is limited to each adapter's documented
operations; consult its guide before adding extended workflows.

QFEX uses HMAC authentication and dedicated WebSocket trade sessions. Arcus uses
native Ed25519 typed order payloads, exact tick/quantum conversion, and monotonic
nanosecond timestamps with single-attempt REST writes. Its public account streams
are scoped by address and account index; delta books require fresh snapshots after
gaps. Both adapters have protocol/signing, lifecycle, and Python parity tests.

Remaining release gates and work:

- Funded authenticated submit/cancel verification remains pending for every venue,
  including QFEX UAT and Arcus testnet. Real authenticated Lighter order-stream
  verification also needs credentials.
- Lighter's local ECgFp5/Poseidon primitives remain experimental and unaudited;
  cryptographic review is required before production use. See its
  [signing notes](crates/exchanges/lighter/SIGNING.md).
- Private token refresh requires a new application-managed authenticated
  subscription. Lighter nonce skipping, grouped/batch submission, and key
  registration are outside the current surface; other extended operations are
  tracked in the exchange guides.
- Follow the [Python release checklist](crates/python/README.md#releases) for
  publisher setup, wheel/platform verification, and distribution. Rust package
  publication remains a separate release task.

### Exchange backlog

Venues remain in the requested order. Checked entries have Rust adapters and
Python bindings, with the release gates above still outstanding. Scheduling for
additional venues is undecided.

- [x] Hyperliquid
- [x] Lighter
- [ ] Aster
- [ ] GRVT
- [ ] Antarctic
- [x] QFEX
- [ ] EdgeX
- [x] Arcus
- [ ] Ondo Perps
- [ ] Extended
- [ ] Pacifica
- [ ] Bulk
- [ ] Paradex
