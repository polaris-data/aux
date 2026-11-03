# Paradex verification

Recorded 2026-11-03 on macOS arm64, Rust workspace toolchain and CPython 3.12.7.

| Check | Evidence |
| --- | --- |
| API coverage | 139 Swagger operations plus three documented additions; 30 WebSocket entries and 12 SBE templates; hashes and Rust/Python fixture parity checked by `scripts/check-native-coverage.py` |
| Signing | 14 native typed-message cases across two appchains, Starknet and Argent/EVM signatures, ten administrative hashes, four SIWE auth/onboarding vectors and deterministic EVM account address; altered payloads, network/domain separation and exact precision |
| REST and Python | Every operation through local HTTP peers, native error details and exact large numbers, explicit JWT authentication, read-only/expired credentials, rate-limit errors and disconnected writes without replay |
| SBE | All 12 official schema 1:3 templates; authoritative full trade IDs; exact decimals/microseconds; nullable fields, groups, extended block lengths, truncated/malformed frames, unsupported schema/template/version rejection |
| RPC and streams | Every native RPC command through a local JSON peer, mixed batch outcomes, cancellation/pending response reconciliation without replay, mixed JSON/SBE, reconnect gaps/fresh snapshots, pre-snapshot delta rejection and reader cleanup |
| JWT lifetime | Local private socket still delivers after JWT expiry; REST and reconnect fail until explicit renewal; changing REST permissions leaves the active RPC session's established permissions intact |
| Repository checks | `scripts/check.sh` and `scripts/check-python.sh`: formatting, Clippy, offline Rust/Python tests, isolated feature combinations, coverage parity and dependency isolation |
| Public testnet, Rust | `paradex_public`: system configuration reports `PRIVATE_SN_PARACLEAR_TESTNET`, subscribe acknowledgement and ETH-USD-PERP orderbook snapshot decoded as SBE 1:3, close/shutdown |
| Public testnet, Python | `derive_paradex_public.py`: system configuration and SBE book subscription through the Rust binding, async context cleanup |
| Funded verification | **Pending:** EVM/Starknet onboarding, subkeys/scopes, submit/cancel, RFQ/block execution, vaults, transfers and administrative actions; no credentials supplied |

Contract fixtures are synthetic, while signing vectors and SBE expectations are
cross-checked with the pinned official SDK. No runtime SDK dependency is used.
These tests do not establish successful live private operations or an audit.

To repeat the public check separately from offline CI:

```sh
cargo run --locked -p aux-rs --features paradex --example paradex_public
```

The opt-in Rust and Python testnet trading examples require `--execute` and
caller-provided funded credentials, price, and size. They were not executed.
