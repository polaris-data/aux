# Monaco signing and protocol references

The connector implements the native protocol in Rust. It does not invoke the
Monaco SDK or a shared library at runtime. Cryptographic primitives come from
K256, Ed25519-dalek, SHA-2 and SHA-3; Base64 supplies RFC structured byte sequences,
and URL supplies canonical final request URLs. All are adapter-local dependencies.

## Pinned references

- `@0xmonaco/core@1.1.5`, downloaded from
  `https://registry.npmjs.org/@0xmonaco/core/-/core-1.1.5.tgz`.
  Archive SHA-256: `7118b4933c4cbe13329613d3f56325a7374ab89e400583a699949fcca3a9b5b6`.
  Its package metadata does not declare a license; no TypeScript source is
  vendored or executed by the connector. It was used only as an independent
  compatibility reference to produce synthetic test vectors.
- `monaco-sdk@1.1.5`, downloaded from
  `https://crates.io/api/v1/crates/monaco-sdk/1.1.5/download`.
  Archive SHA-256: `61def5d3352672dcf9080d8c0a7f799f934f81bd23beec9d4799881b121a1695`.
  Published VCS revision: `6c07eb310e139ad7df4d56bf3f9b6679efc4654e`, `rust-sdk`.
  Published author: Monaco Protocol Team. Declared license: MIT.
  `tests/fixtures/rfc9421.json` reproduces its published
  `tests/fixtures/http-message-signatures.json` (synthetic RFC 8032 test key).
  The attribution/license notice is in `LICENSE-MONACO`.
- Official OpenAPI and WebSocket/signing documentation retrieved 2026-11-03:
  `https://docs.0xmonaco.com/proto-openapi/api/openapi.yaml`,
  `https://docs.0xmonaco.com/reference/websockets`, and
  `https://docs.0xmonaco.com/reference/http-message-signatures`.
  REST types are a scoped translation of native schemas, not an official SDK
  dependency. The hashes below identify the exact reference bytes inspected.

| Reference file | SHA-256 |
| --- | --- |
| `package/dist/crypto/session.js` | `66692421717c31923fdad44ba2e62579b44333375f2948f15ddc878691ba7c7a` |
| `package/dist/crypto/http-signatures.js` | `aebb8b5fb02d1618b32a0c28d8b59db4b1263bfcc22728856c495591fe750d2c` |
| `package/dist/api/auth/api.js` | `3b7aabb1e2bf204a970190f81870e90062cfb0223c40482cafef8219b0fd2489` |
| `package/dist/api/websocket/websocket.js` | `cda03f894a49ead0527d857af053865a78eb6277befe72cc799e346313fdf6ff` |
| `openapi.yaml` | `d068ab22934796c22491e2171582efeac0a6037fe07a1f623b85e54a75f35cf6` |
| `websockets.md` | `eebf1dd07ffe1ddc6e90a8f5f2eb112325af4dff5e9e799cb3d6cc5be28076be` |
| `signatures.md` | `0221aec4495698d5fba71a7a3adf446afcf604d1dfc80b4d22514220ea09cc28` |
| Published Rust RFC fixture | `d4eafb47a3395d264cccb7e73f92e3f03701264c0bef495de07b11c2307f9cf8` |

## Compatibility vectors

`tests/fixtures/signing.json` was produced with the pinned official TypeScript
`composeSigningString`, `composeWsSigningString`, `signMessage`, and
`buildHttpSignatureHeaders` helpers, Noble curves/hashes 2.4.0, and Viem 2.45.2
for wallet `personal_sign`. All keys, messages, identities and idempotency keys
in signing fixtures are public synthetic test data and must never fund an account.

Vectors cover empty bodies/queries, duplicate and percent-encoded query values,
exact JSON whitespace and decimal strings, WebSocket authentication, and Unicode
wallet messages whose UTF-8 byte length differs from their character count.
Deterministic signatures are compared byte-for-byte. Negative tests change
messages, keys and domains and reject invalid external session signatures.
Reference-generation tools and JavaScript packages are not committed or needed
by the offline tests.

Other JSON fixtures are public staging market reads captured 2026-11-03; the
markets fixture is reduced to one spot and one margin market. Public maker/taker
wallets in trade data are replaced with synthetic addresses. Private sessions,
account results, batches and stream failure scenarios use synthetic local peers.

## Native signing rules and limits

The wallet authorizes an Ed25519 session by signing the complete challenge with
EIP-191. The challenge must bind the requested public key; its server-provided
expiry is checked before and after the wallet callback. Session registration,
wallet/account eligibility, application permissions and native risk checks remain
server-owned. External contract-wallet authorization is passed unchanged; the
server decides whether that wallet/deployment supports it.

Legacy REST signs `METHOD\npath?query\ntimestamp_ms\nSHA256_hex(body)` without a
trailing newline, omitting `?` for an empty query. RFC 9421 signs the method,
escaped path, raw query (including `?` when empty), and SHA-256 Content-Digest in
the documented order, with `created` seconds, lowercase public-key `keyid`, and
`alg="ed25519"`. Body bytes are serialized once. Standard HTTP redirects are
disabled. Neither host nor scheme is part of the native signature; auth handlers
therefore reject requests outside their configured endpoint and chain context.
Custom transports must preserve finalized bytes and disable redirects too.

WebSocket signatures cover `WS-AUTH\n<public_key>\n<timestamp_ms>` and are rebuilt
on each connection. The server's clock-skew window is ±30 seconds; this connector
uses the local clock and does not estimate server-clock offset or retry a stale
signature automatically. Keep the host clock synchronized. Timestamps are
freshness controls, not nonces: identical signed requests can be replayed inside
the window. Explicit caller-owned idempotency keys and reconciliation determine
safe recovery; there is no automatic order replay.

Compatibility tests are not a cryptographic audit. Funded authenticated live
verification remains pending. Documented depth/BBO channels have offline coverage;
the staging deployment rejected `depth` during public verification.
