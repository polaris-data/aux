# Lighter signing compatibility

The SDK performs hashing, ECgFp5/Schnorr signing and auth-token generation in Rust. Go is a development oracle only. No Go FFI, shared library, subprocess, Python SDK or exchange client package ships with this adapter.

The local signer is **experimental and unaudited**. `goldilocks-crypto` and `poseidon-hash` explicitly describe themselves as unaudited prototypes. Reference compatibility is not a security audit or a constant-time guarantee. Their scalar/point operations use variable-time arithmetic, and internal temporary copies are not guaranteed to be zeroized. A cryptographic review is required before production use. The replaceable native `Signer` boundary also supports application-owned signing services.

## Pinned sources and attribution

- Official [lighter-go](https://github.com/elliottech/lighter-go/tree/ebae8edecfee9986b0a5fcde3cb9a06bd4af3a3a), revision `ebae8edecfee9986b0a5fcde3cb9a06bd4af3a3a`.
- Its pinned dependency [poseidon_crypto v0.0.15](https://github.com/elliottech/poseidon_crypto/tree/v0.0.15). The official module's `go.sum` pins content hashes. Fixture metadata records SHA-256 hashes of the official module and signing/transaction sources.
- Rust primitives: [`goldilocks-crypto =0.1.2`](https://docs.rs/goldilocks-crypto/0.1.2/goldilocks_crypto/) and [`poseidon-hash =0.1.4`](https://docs.rs/poseidon-hash/0.1.4/poseidon_hash/), MIT OR Apache-2.0. Exact versions are intentional compatibility pins, not endorsements of an audit.
- REST schema, endpoint profiles and WebSocket examples: official [lighter-python](https://github.com/elliottech/lighter-python/tree/d30dc2296d27f1584faeb22a590486aded1f03b1), revision `d30dc2296d27f1584faeb22a590486aded1f03b1`. This SDK does not depend on it. Public protocol fixtures were captured from Lighter testnet on 2026-10-29.

`src/poseidon.rs` adapts the Plonky2 transaction permutation, round constants and overwrite sponge from `poseidon_crypto v0.0.15/hash/poseidon2_goldilocks_plonky2/{config,poseidon2}.go`. Copyright © 2023 ZkLighter Protocol; [Apache-2.0 license](LICENSE-APACHE). The upstream matrix constants originate in [Plonky3 at eeb4e37](https://github.com/Plonky3/Plonky3/blob/eeb4e37b20127c4daa871b2bad0df30a7c7380db/goldilocks/src/poseidon2.rs#L28). This port uses the Rust Goldilocks arithmetic primitive and ordinary field operations in place of the Go implementation's 128-bit optimization. Hash ordering and overwrite-sponge behavior are preserved.

## Native schemes

| Payload | Encoding and hash |
|---|---|
| Create order | Plonky2 Poseidon2 over chain ID, tx type `14`, nonce, transaction expiry, account index, API-key index, then the eleven native order fields in official order |
| Cancel order | Plonky2 Poseidon2 over chain ID, tx type `15`, nonce, transaction expiry, account index, API-key index, market index, order/client order index |
| Attributes | Up to four nonzero type/value pairs in ascending type order, padded with zeros; hash attributes and then hash the concatenated transaction/attribute field digests. All-zero attributes leave the original hash unchanged. |
| Auth token | Original Poseidon2 over little-endian 8-byte words of `deadline:account_index:api_key_index`, with a zero-padded final word |
| Schnorr challenge | Original Poseidon2 over the encoded nonce point followed by the message's five field elements |

Both hash variants produce five canonical Goldilocks elements encoded as 40 little-endian bytes. They have different constants and must not be interchanged. Fp5 raw limb encodings from the pinned Rust primitive are normalized before public-key/auth-digest encoding. The adapter verifies signatures using the primitive point/scalar operations directly, avoiding the upstream helper's debug logging of signature material.

Private keys are nonzero, canonical 40-byte **little-endian native scalars**, optionally hex-prefixed with `0x`; Ethereum 32-byte keys and seed strings are not accepted. Local Schnorr nonces are nonzero canonical scalars sampled with `getrandom` from the OS; callers cannot supply an ephemeral signing nonce. Key input buffers and nonce byte buffers use `Zeroizing`. Native sequential transaction nonces are separate from these secret Schnorr nonces.

The signature is 80 bytes, `s || e`, with both scalars encoded little-endian. Transactions encode it as base64 in PascalCase `tx_info` JSON. `sendTx` uses URL-encoded `tx_type` and `tx_info` form fields. Auth tokens use `deadline:account:key:hex_signature`, without `0x`; deadlines are Unix **seconds** and must be within eight hours. The default private read/stream token is ten minutes. Transaction `ExpiredAt` and `OrderExpiry` use **milliseconds**.

Auth tokens do not cryptographically include the chain ID. This is native behavior, not a missing domain field to add. The SDK binds the token/auth handler to an explicit chain context, while transaction hashes include the chain ID. Custom endpoints therefore require a chain ID argument.

## Nonce and outcome rules

All independently constructed handlers in this process share one coordinator for `(chain_id, account_index, api_key_index)`. Different signers attached to the same API-key identity also share it. Custom endpoints with the same chain ID share that identity conservatively. Different processes, external clients and key rotation require caller coordination and reconciliation.

The coordinator holds a runtime-independent async mutex across nonce lookup, preparation, signing and submission. It fetches `nextNonce` on first use, increments only after decoding native code `200`, and never automatically resubmits. Once transport execution begins, cancellation, transport failure, native rejection or malformed responses quarantine the sequence. Later writes return `NonceUncertain` without hitting the server. Even a reported `NotSent` is handled conservatively.

`resync_nonce()` is an explicit recovery operation after the application reconciles pending transactions. It cannot prove that a transaction timed out before acceptance; immediately fetching a stale server nonce is not reconciliation. Reads remain usable while writes are quarantined. An accepted transaction can still fail during execution: `SendTxResponse` is submission acceptance, not proof of a resting order or a fill.

The initial adapter supports create/cancel, integrator fee attributes, self-trade controls and order-version attributes. Nonce skipping, cancel-all attributes and other transaction families are rejected or unexposed. Batch/grouped submission and key registration are not implemented.

## Reproduce compatibility checks

Normal tests run offline and need only Rust:

```sh
cargo test --locked -p aux-rs-lighter
```

The 66 Go fixtures cover create/cancel on both networks, three keys including the scalar-order boundary, maximum integer fields, fee/self-trade/version attributes, trigger orders, and auth tokens. Rust checks native fields, exact hashes, public keys, Go signatures and negative mutations. Signature bytes vary with fresh randomness. An explicit oracle run also verifies Rust-generated signatures with Go:

```sh
# From connectors/. Requires Go 1.26 and network access; never normal CI.
python3 scripts/lighter_vectors.py
CONNECTORS_LIGHTER_SIGNATURE_OUTPUT=/tmp/lighter-rust-signatures.json \
  cargo test --locked -p aux-rs-lighter --test signing pinned_go_hashes
python3 scripts/lighter_vectors.py --verify-rust /tmp/lighter-rust-signatures.json
```

`--reference /path/to/lighter-go` reuses a checkout only if its HEAD matches the pin and tracked files are unchanged. The fixed private keys and ephemeral nonces in that generator are public test data and must never be used for accounts.

On 2026-10-29, all 66 Go signatures passed Rust verification and all 66 Rust-generated signatures passed the official Go verifier. Public HTTP and book/trade/account WebSocket smoke checks passed testnet. Funded authenticated testnet submit/cancel and real private order-stream verification remain pending because credentials are not configured. Those checks and a cryptographic review remain release gates.
