# Derive verification

Recorded 2026-11-03 on macOS arm64, Rust workspace toolchain and CPython 3.12.7.

| Check | Evidence |
| --- | --- |
| API coverage | 128 REST contracts and 17 WebSocket entries; pinned schemas/hashes, named Rust/Python methods and reproducible generation checked by `scripts/check-native-coverage.py` |
| Signing | 17 module cases on two chains, login proof, byte-for-byte reference digests/signatures, altered payloads, precision rejection, owner/network separation, concurrent shared-identity nonce allocation |
| REST and Python | Every operation through local HTTP peers; exact numbers and additional fields; native error mappings; native metadata order preparation and quote signing; remote callback context/cancellation |
| RPC and streams | All 128 REST methods over an isolated local RPC peer; private login/account stream; bounded native transport lifecycle, cancelled reader cleanup, single-reader enforcement and pending command reconciliation |
| Repository checks | `scripts/check.sh` and `scripts/check-python.sh`: formatting, Clippy, offline Rust/Python tests, isolated feature combinations, coverage parity and dependency isolation |
| Public testnet, Rust | `derive_public`: `public/get_time`, ETH-PERP instrument (tick `0.01`), WebSocket subscribe acknowledgement and orderbook snapshot, explicit close/shutdown |
| Public testnet, Python | `derive_paradex_public.py`: ETH-PERP metadata and orderbook subscription through the Rust binding, async context cleanup |
| Funded verification | **Pending:** submit/cancel, session registration/scopes, transfers, withdrawal requests, RFQ, liquidations and vault actions; no credentials supplied |

Contract fixtures are deterministic synthetic protocol cases. Signing vectors use
public test keys. They do not establish successful live private operations or a
cryptographic audit. The public checks run separately from offline CI; testnet
liquidity and network availability can change their output.

To repeat the public check:

```sh
cargo run --locked -p aux-rs --features derive --example derive_public
```

The opt-in Rust and Python testnet trading examples require `--execute` and
caller-provided funded credentials, price, and size. They were not executed.
