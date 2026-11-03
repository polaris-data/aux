# Metric verification

Recorded 2026-11-03 on macOS arm64, Rust workspace toolchain and CPython 3.12.7. This records observed checks separately from the interface inventory.

| Area | Evidence / status |
| --- | --- |
| HTTP interfaces | All eight documented operations exercised against offline Rust/Python HTTP peers; native fields, per-pool failures, optional bearer authentication, precision rejection and closed clients |
| ABI compatibility | 987 independently generated function/event/error cases from viem 2.38.5; byte encoding, result decoding, event topics/data and native errors in Rust and Python; all 504 typed Rust calls referenced at compile time |
| Transactions | Six byte-for-byte ethers 6.16.0 signed-envelope references: legacy/EIP-1559 on chain IDs 1, 56 and 4663; local and async external signatures, identity checks, cancellation and callback context |
| Arithmetic and liquidity | SDK vectors for bin packing, cursor bounds, initial scaled amounts, empty/funded/cursor-boundary additions and weighted plans; removal percentages/caps, duplicate rejection and precision failures |
| Oracle format | Pinned pusher slot, feed ID, update/consent digest and personal-signature references; representability and spread constraints |
| Submission lifecycle | Shared nonce allocation, stale RPC nonce protection, wrong chain/code/hash, timeout/cancellation quarantine, reconciliation and process-state retention after clients close; no automatic write replay |
| Public reads | Python connector: chain discovery (3), Ethereum metadata (42 pools), WETH/USDC bid/ask, native nullable depth and explicit client cleanup |
| Local EVM integration | Anvil chain 31337; pinned pusher artifact `demo/artifacts/CompressedOracleV1.json` deployed locally; connector signed and submitted a raw oracle-slot update, observed receipt status `0x1`, read `getOracleData`, obtained one log and reconciled the pending hash |
| Repository checks | `scripts/check.sh` and `scripts/check-python.sh` passed: formatting, Clippy, full Rust feature matrix/workspace tests, 83 Python tests, coverage/generation and dependency isolation. Metric has 25 integration tests plus one compile-fail doctest. Optional SDK fixture regeneration/comparison also passed against package 0.7.0 |
| SDK workflow helpers | 58 independently generated official-SDK cases for creation/defaults, uniform bins, all liquidity variants, all twelve swap fallbacks, external payments, pure evaluation and block-consistent native stop-loss reads; Rust and Python parity tests |
| Full SDK export audit | The four documented workflow groups have reference coverage; an exhaustive audit of every remaining SDK export is separate |
| Stateful workflow integration | **Pending:** local/fork swap, liquidity, factory creation/administration, approvals/callbacks and native-payment workflows |
| Funded live verification | **Not performed:** authenticated propAMM builder submission/replacement/cancellation or funded on-chain operations |

Normal repository checks remain offline and submit no mainnet transactions:

```sh
bash scripts/check.sh
bash scripts/check-python.sh
```

These cover formatting, Clippy, tests, standard/custom transport feature combinations, checked-in generation and production dependency isolation. Public reads run separately using `metric_public` or `MetricClient`. Metric has no invented testnet endpoint or streaming feed.

The local oracle check used public Anvil test credentials, an unlocked local deployment transaction and our connector for the signed update/read/receipt/log/reconciliation flow. It establishes that workflow on local bytecode only. It does not establish permissions or successful state transitions for the other inventoried contracts.
