# Metric

Enable `features = ["metric"]` on `aux-rs` for native Metric HTTP APIs and EVM contract operations. Metric requires an explicit chain ID. `MetricClient` covers the eight documented HTTP routes; `MetricChainClient` reads contracts, builds transactions, signs and submits caller-approved operations, queries receipts and reads block-range logs. Metric does not publish a native streaming feed.

```rust
use aux_rs::metric::{Metric, MetricClient, MetadataRequest};

let client = MetricClient::new(Metric::new(1)?)?;
let pools = client.metadata(MetadataRequest {
    count: Some(10), ..Default::default()
}).await?;
client.transport().shutdown().await?;
```

The default HTTP origin is `https://api.metric.xyz`. Set `Metric::custom` for an explicit alternate API deployment. `with_transport` supports caller-defined HTTP transports without enabling a networking runtime. Facade defaults enable HTTP and WebSocket transports; Metric only uses HTTP. No gRPC or Flint dependency is added.

`metadata` and `cross_chain_metadata` preserve native pagination and additional fields. `bid_ask` retains nullable prices/depth and scaled integer strings. `batch_bid_ask` preserves errors per pool. `route` retains split routes and native partial outcomes. `prepare_route` maps one complete route to router calldata, verifying token direction against native pool state; request `maxRoutes=1` for this mapper. Split routes remain inspectable through the HTTP response and caller-composed contract calls.

## Contracts and exact values

[ABI files](abi) and [the operation inventory](contracts.json) cover pools, factories, routers, quoters, liquidity helpers, extensions, oracles and price providers, plus ERC-20 and Multicall3 interfaces. Rust exposes typed `SolCall`, `SolEvent` and `SolError` bindings under `metric::contracts`. `encode_contract_call`, `decode_contract_input`, `decode_contract_output`, `decode_contract_event` and `decode_contract_error` also accept an explicit ABI name and signature. Use full signatures for overloads.

Amounts use native `U256`/`I256`, exact decimal strings or rational integer inputs. Excess precision is rejected. Arithmetic helpers expose explicit rounding. `math`, `liquidity`, `planning` and `oracle` implement packing, scaled arithmetic, liquidity distributions, cursor bounds, stop-loss calculations and compressed oracle packets. `liquidity_snapshot` anchors all reads to one block before calculating a plan. Equal bin targets produce a uniform addition; weighted additions and percentage/absolute removals are also available.

A prepared call contains `to`, `data` and `value`. Supply chain, gas, fees and nonce in an inspectable `PreparedTransaction`; preparation does not submit. Calls to pool creation/configuration, swaps, liquidity, fee collection, oracle updates and privileged administration use the native contract parameters and permissions. The caller chooses approvals, slippage, deadlines, recipients and extension data. Direct pool swaps and additions require the native payment callback implementation; use the router/liquidity adder for ordinary wallet workflows. See [signing and submission](SIGNING.md).

`Addresses::current(chain_id)` uses the deployment documentation. The documented liquidity-adder address differs from SDK 0.7.0; the discrepancy is recorded in [source pins](API_SOURCES.json). Configure custom deployments explicitly. Submission checks both RPC chain IDs and target code; configure runtime code hashes for contract identity pins.

## Workflow helpers

[WORKFLOWS.md](WORKFLOWS.md) covers validated pool creation with resolved defaults/salts, exact-shares and weighted liquidity additions with native ETH refunds, all twelve native/external swap-fallback variants, and combined stop-loss evaluation with a block-consistent native state reader. Python exposes these through `aux_py.metric` and `MetricChainClient.oracle_stop_loss_status`. Preparation returns unsigned calls; submission is explicit.

## HTTP authentication and propAMM

Attach a zeroized `TradingKey` with `with_auth` for optional bearer authentication. The unsigned Rust client cannot call `pamm_quote`; Python raises `AuthenticationError` when no trading key is configured. HTTP bearer authentication and wallet signing are separate credentials.

PropAMM submission accepts a signed transaction. An empty `tx` requests cancellation with the native replacement UUID and sequence. Replacement and cancellation are explicit calls. Builder outcomes, hashes and partial failures remain native response fields; builder acceptance does not establish on-chain inclusion. BSC cancellation cannot revoke an already accepted BlockRazor bundle. Writes are sent once and never automatically replayed.

## Python

```python
from aux_py import MetricClient, MetricChainClient
from aux_py import metric

async with MetricClient(chain_id=1) as api:
    pools = await api.metadata({"count": 1})

async with MetricChainClient(chain_id=1, read_rpc=rpc_url) as chain:
    pool = chain.contract("MetricOmmPool", pool_address)
    immutables = await pool.get_immutables()
    call = pool.prepare_swap(swap_arguments)
```

The generated contract classes expose native reads and `prepare_*` write builders. Supply native argument sequences or named mappings; exact integers become Python `int` and HTTP fractional numbers become `Decimal`. `MetricSigner` supports local keys and asynchronous external callbacks. Python delegates encoding, arithmetic, signing and transport to Rust and has no third-party runtime Python dependencies. Clients support `aclose` and async context management; callbacks retain the initiating asyncio context and support cancellation.

## Coverage and verification

[Coverage](COVERAGE.md) distinguishes HTTP operations, ABI codec coverage and SDK helpers. [Verification](VERIFICATION.md) separates offline compatibility checks from public reads and funded integration checks. Run the read-only example with:

```sh
METRIC_CHAIN_ID=1 cargo run --locked -p aux-rs --features metric --example metric_public
```

`metric_contract transaction.json` prints a caller-supplied unsigned request. Passing `--execute` explicitly signs and submits once, using `METRIC_PRIVATE_KEY`, `METRIC_READ_RPC`, optional `METRIC_WRITE_RPC` and optional `METRIC_CODE_HASH`. No funded transaction is part of normal CI.
