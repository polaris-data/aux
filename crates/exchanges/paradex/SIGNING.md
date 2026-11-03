# Paradex signing provenance

The [provenance record](reference/provenance.json) pins the official
[`paradex-py@4ae46b5`](https://github.com/tradeparadex/paradex-py/tree/4ae46b533fc99dbf5fbf33b46611c829fa38b12a)
reference (0.8.2), schema hashes, and MIT notice. No exchange SDK is used at
runtime. Focused primitives are `starknet-crypto`, `starknet-core`, `k256`, and
`sha3`; key bytes are zeroized.

Starknet account/subkey authentication uses the native revision-0 typed message.
Account identity is supplied independently from signing identity. The legacy
domain's field order is `name, chainId, version`; a local codec preserves it
because the generic Starknet encoder requires another canonical domain order.
Signatures use the official reference's RFC6979 extra seed of 32. That seed is
confirmed in the pinned `starknet-crypto-py` wrapper identified in provenance.

Orders and modifications retain the native market/type/side semantics and exact
e8 price/size conversion. Administrative workflows accept explicit native typed
data. Revision-1 block/offer signing uses canonical market ordering, typed
Poseidon hashing, and the reference Merkle convention: a singleton root is the
leaf itself. This differs from the generic upstream encoder's singleton padding.
Offer signatures bind the parent block ID. Both top-level reference constraints
and native nested `trade_constraints` are accepted by the block helper.

EVM authentication/onboarding use SIWE and EIP-191 with explicit L1 chain,
domain, URI, purpose, issuance, and expiry. Order/block/admin signatures for EVM
accounts use the native Argent EIP-191 signature envelope over the finalized
Starknet typed-message digest. `evm_account_address` performs only deterministic
address calculation from the system-config class hash; deployment and chain
transaction handling belong to the application.

Local and external signatures are verified against the configured signing key.
Account, chain, signer, and network context are checked separately. Neither JWT
refresh nor transaction submission is hidden inside a signer callback.

`tests/fixtures/signing.json` contains 14 vectors generated with the pinned SDK:
auth, onboarding, LIMIT/MARKET order, modify, block, and offer on both appchains.
Each records the native typed data, digest, Starknet signature, and EVM/Argent
signature. `siwe.json` adds four SIWE auth/onboarding vectors and the deterministic
EVM account-address result. `administrative.json` records ten official session/key-management
hash vectors from the SDK's session test corpus. Tests also reject altered
payloads, wrong signatures/keys, unsupported precision, and expired/read-only
credentials. Synthetic fixture keys must never be funded.

The tests establish reference compatibility, not an audit. Funded onboarding,
subkey permissions, administrative actions, and trading verification remain
pending without suitable credentials. The exchange validates account access,
JWT authenticity, and authorization scopes.
