# Derive v3 signing provenance

The [provenance record](reference/provenance.json) pins source commits, API hashes,
and license notices. The local signer uses `k256` for secp256k1 and `sha3` for
Keccak, with zeroized private-key storage. Official SDKs are compatibility
references and are absent from runtime dependencies.

Login signs the decimal millisecond timestamp with EIP-191. HTTP authentication
uses the owner wallet, timestamp, and signature headers; WebSockets use
`public/login`. Login identifies the signing key, while the explicitly supplied
wallet identifies the account owner. It is separate from action authorization.

Actions use the v3 EIP-712 `Action` struct, including subaccount ID, nonce,
module address, encoded module data, expiry, owner, and signer. The domain binds
the chain ID and matching contract. `DeriveAuth` rejects mismatched networks,
contracts, owners, and signers. Returned external signatures are recovered and
checked before use. Session-key scopes, expiry, and access restrictions are
enforced by the exchange; the connector never treats a delegated key as its owner.

The native ABI codec implements static words, dynamic bytes, tuples, and arrays.
Typed modules cover trades, internal/external transfers, withdrawal requests,
session registration, recipient whitelists, RFQ maker/taker actions, deletion,
liquidation, and seven vault actions. `Encoded` accepts explicit caller-prepared
module bytes. Native asset metadata resolves instrument addresses/sub-IDs and
underlying withdrawal decimals. Decimal conversion rejects rounding, exponent
notation, overflow, and unsupported precision.

`tests/fixtures/signing.json` was generated against
[`derive-ts@f15f1b8`](https://github.com/derivexyz/derive-ts/tree/f15f1b8776a7535f2609bf9130265d1a67704077)
using its module encoders and EIP-712 implementation with ethers 6.16.0 and
tsx 4.20.6. It records module bytes, digests, and deterministic signatures for
17 module cases on chain 1 and 11155111, plus an EIP-191 login vector. Liquidation
uses the separately checked public v3 decoder layout recorded in
`liquidation-decoder.json`; the offline vector uses the same ethers ABI encoder.
All private keys are public synthetic test inputs and must never be funded.

Tests compare every byte/digest/signature, reject altered messages and precision
loss, and allocate nonces concurrently across separately constructed auth states
sharing a signer. The process-wide allocator guarantees uniqueness and monotonic
allocation within one network/signing identity. Cross-process coordination,
submission ordering, and deliberate caller-supplied nonces remain caller-owned.

These checks establish reference compatibility. Funded delegated-key, scope,
withdrawal, liquidation, and vault verification remains pending without suitable
credentials. They do not constitute a cryptographic audit.
