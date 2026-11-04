# Shared EVM transactions

`aux-rs-evm` supplies `ChainConfig`, `EvmChainClient`, local/caller-supplied `EvmSigner`, prepared/signed transactions, isolated read/write RPC and sender-scoped nonce coordination. It has no venue adapter dependency and no networking defaults; select `http` for standard I/O or inject `HttpTransport`.

Preparation, signing and submission are distinct. Validate chain/target code and optional runtime code hashes, supply explicit gas/fees/recipients/value, and submit once. Ambiguous submissions retain their hash and reserved nonce. `reconcile` checks pending transaction/receipt state; `resync_nonce` requires the specific pending hash after external reconciliation. Never automatically refresh, re-sign or replay an uncertain transaction. Metric preserves its prior public client through a compatibility wrapper; Polymarket directly uses this crate. Venue ABIs and funding workflows remain outside this layer.

Metric's existing EVM fixture/local-RPC regression tests exercise this shared implementation, including concurrent clients, stale RPC pending counts, rejections, receipts and explicit recovery.
