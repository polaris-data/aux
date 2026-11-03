# Metric signing and submission

`LocalEvmSigner` holds a zeroized secp256k1 key. Implement `EvmSigner` for asynchronous remote signing; the callback receives the finalized prehash. Both use the same `PreparedTransaction` format. Python exposes `MetricSigner(key)` and `MetricSigner.remote(address, async_callback)`; callbacks receive a mapping with `digest`, `address` and `scheme`. Returned signatures are recovered and checked against the expected address.

Legacy EIP-155 and EIP-1559 transaction envelopes are supported. Set either `gasPrice` or both `maxFeePerGas`/`maxPriorityFeePerGas`; mixed fee modes are rejected. Supply gas limit, chain, target, calldata and native value explicitly. Offline `sign_transaction` preserves the nonce. `send_transaction` replaces the supplied nonce using the remote pending count and process-local coordinated state.

Nonce state is shared across clients for `(chain_id, signer address)` and retained after clients close. One reservation can be in flight per identity. Cancellation or an uncertain transmission retains the transaction hash and blocks another coordinated submission for that identity. `pending`, `transaction`, `receipt` and `reconcile` expose that state. `resync_nonce(address, expected_hash)` is an explicit application decision after reconciling a dropped, rejected or replaced transaction. Coordinate multiple processes in the application. `submit_raw` validates a signed envelope but does not allocate or coordinate its nonce.

Both configured RPCs must report the selected chain. A target must have contract code; optional runtime code hashes constrain its identity. Network chain IDs alone do not prove that an address hosts the intended Metric version. The deployment defaults and source-version discrepancies are in `API_SOURCES.json`.

A write is transmitted once. After a response that cannot establish the submitted hash, `UncertainSubmission` retains the expected hash and cause. Transport errors preserve their submission outcome. Python maps uncertain writes to `OutcomeUncertainError` with structured `hash`, `outcome` and `cause`. Do not re-sign or replay an uncertain transaction automatically. Query the transaction and receipt or explicitly reconcile its replacement before resubmitting.

A returned hash means submission, not successful execution. Inspect the native receipt `status` and logs. RPC native errors retain code/data for ABI decoding. Approval transactions, native payments, callback-capable callers and administrative permissions remain explicit native prerequisites. The connector never provisions accounts, grants approvals, funds wallets or chains administrative operations automatically.

Compressed oracle slots use the pinned pusher's U64x32 price encoding and spread codebook. Truncation is explicit; exact mode rejects unrepresentable values. `slot_update_digest` and `pusher_consent_digest` implement the native ABI-encoded hashes. `sign_personal_digest` applies the EIP-191 32-byte prefix before signing. Do not apply that prefix to an EVM transaction signing hash.

Byte-for-byte fixtures use viem 2.38.5, ethers 6.16.0 and pinned Metric SDK/pusher sources. Fixed private keys in test fixtures are public test credentials.
