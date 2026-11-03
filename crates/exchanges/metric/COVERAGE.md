# Metric coverage

The inventory describes implemented interfaces and codecs. It does not establish funded operation success. See [verification](VERIFICATION.md) and [source pins](API_SOURCES.json).

## HTTP operations

| Route | Rust and Python method |
| --- | --- |
| `GET /public/v1/chains` | `chains` |
| `GET /public/v1/evm/factory-settings` | `factory_settings` |
| `GET /public/v1/evm/{chain_id}/metadata` | `metadata` |
| `GET /public/v1/evm/metadata` | `cross_chain_metadata` |
| `GET /public/v1/evm/{chain_id}/{pool_address}/bid_ask` | `bid_ask` |
| `POST /public/v1/evm/{chain_id}/bid_ask/batch` | `batch_bid_ask` |
| `POST /public/v1/evm/{chain_id}/route` | `route` |
| `POST /public/v1/evm/{chain_id}/pamm-quote` | `pamm_quote` |

All eight operations have offline HTTP fixtures and Rust/Python local-peer tests. Authentication, exact numbers, nullable values, per-pool errors and native builder outcomes remain separate.

## Contract bindings

504 function entries, 151 event entries and 332 error entries across 26 ABI files; inherited signatures repeat across contracts. These are codec inventory counts. The function entries represent 330 distinct signatures. Fallback/receive entries are also inventoried.

| ABI | Functions | Events | Errors |
| --- | ---: | ---: | ---: |
| [AnchoredPriceProvider](abi/AnchoredPriceProvider.json) | 33 | 4 | 11 |
| [AnchoredProviderFactory](abi/AnchoredProviderFactory.json) | 37 | 13 | 15 |
| [B20AssetParams](abi/B20AssetParams.json) | 4 | 0 | 2 |
| [ChainlinkOracle](abi/ChainlinkOracle.json) | 53 | 20 | 30 |
| [CompressedOracleV1](abi/CompressedOracleV1.json) | 37 | 20 | 28 |
| [DepositAllowlistExtension](abi/DepositAllowlistExtension.json) | 13 | 2 | 5 |
| [ExternalSwapExecutor](abi/ExternalSwapExecutor.json) | 2 | 0 | 4 |
| [IERC20](abi/IERC20.json) | 6 | 2 | 0 |
| [ImmutableAppPoolHelper](abi/ImmutableAppPoolHelper.json) | 4 | 0 | 4 |
| [MetricOmmPool](abi/MetricOmmPool.json) | 16 | 12 | 33 |
| [MetricOmmPoolDataProvider](abi/MetricOmmPoolDataProvider.json) | 13 | 0 | 10 |
| [MetricOmmPoolDeployer](abi/MetricOmmPoolDeployer.json) | 2 | 0 | 1 |
| [MetricOmmPoolFactory](abi/MetricOmmPoolFactory.json) | 62 | 23 | 38 |
| [MetricOmmPoolLiquidityAdder](abi/MetricOmmPoolLiquidityAdder.json) | 9 | 0 | 25 |
| [MetricOmmSimpleRouter](abi/MetricOmmSimpleRouter.json) | 15 | 1 | 35 |
| [MetricOmmSwapQuoter](abi/MetricOmmSwapQuoter.json) | 12 | 0 | 10 |
| [Multicall3](abi/Multicall3.json) | 16 | 0 | 0 |
| [MultiplierOracle](abi/MultiplierOracle.json) | 27 | 8 | 10 |
| [OracleProvider](abi/OracleProvider.json) | 4 | 2 | 0 |
| [OracleValueStopLossExtension](abi/OracleValueStopLossExtension.json) | 25 | 12 | 15 |
| [PriceProvider](abi/PriceProvider.json) | 18 | 1 | 6 |
| [PriceProviderFactory](abi/PriceProviderFactory.json) | 25 | 9 | 9 |
| [PriceVelocityGuardExtension](abi/PriceVelocityGuardExtension.json) | 11 | 2 | 5 |
| [PythOracle](abi/PythOracle.json) | 44 | 18 | 31 |
| [RobinhoodStockTokenParams](abi/RobinhoodStockTokenParams.json) | 3 | 0 | 0 |
| [SwapAllowlistExtension](abi/SwapAllowlistExtension.json) | 13 | 2 | 5 |

[contracts.json](contracts.json) maps every operation to its generated Rust/Python interface. Every function/event/error has an independent wire fixture in [abi.json](tests/fixtures/abi.json). Rust compiles references to every typed call; both languages compare against the fixture bytes. This verifies ABI compatibility, not stateful business workflows or permissions.

## SDK helpers

Native helpers cover packing, exact arithmetic, cursor bounds, addition/weighted/removal plans, stop-loss mathematics, oracle packet signing, route preparation and native swap payments. Python helper inventory:

`addition_plan`, `addresses`, `codebook`, `create_uniform_bins`, `decayed_high_watermark`, `decimal_to_raw`, `decode_bin_state`, `decode_u64x32`, `direction_bitmap`, `encode_bin_state`, `encode_spread`, `encode_u64x32`, `evaluate_oracle_stop_loss`, `extension_order`, `feed_id`, `liquidity_cursor_bounds`, `liquidity_native_payment`, `minimum_operational_liquidity`, `mul_div`, `native_payment`, `oracle_push_calldata`, `pack_bins`, `pack_oracle_slot`, `percentage_removal`, `position_bin_key`, `prepare_create_pool`, `prepare_external_swap`, `prepare_factory_create_pool_params`, `prepare_liquidity_addition`, `prepare_swap_with_fallback`, `price_to_q64`, `price_x64_to_e18`, `pusher_consent_digest`, `recommended_initial_scaled_amounts`, `removal_plan`, `scale_weighted_deltas`, `seconds_until_stop_loss_clears`, `slippage_limit`, `slot_update_digest`, `stop_loss_breached`, `stop_loss_metrics`, `stop_loss_threshold`, `unpack_bins`, `unpack_swap_details`, `unpack_two_int128`, `unpack_two_uint128`, `unwrap_output`, `weighted_plan`.

The pool-creation, native-liquidity, external-fallback and combined stop-loss workflow groups are implemented through typed Rust requests and matching Python helpers. [WORKFLOWS.md](WORKFLOWS.md) documents preparation, defaults, constraints and examples. Their 58 official-SDK reference cases cover eight factory/helper creation cases, five uniform-bin cases, eight liquidity variants, twelve fallback variants, twelve external-payment cases, twelve stop-loss evaluations and one native state-reader case. The state-reader anchors all requests to one block and fails without returning partial status.

`coverage.json` maps these groups to SDK exports, Rust/Python interfaces and tests. Generated contract calls also expose low-level calldata builders. These workflow groups and ABI codecs have explicit reference coverage; a complete audit of every remaining public SDK export is a separate scope from these named groups.

## Checks

`scripts/check-native-coverage.py` verifies pinned snapshot/fixture hashes, equality of ABI inventory and reference cases, named HTTP and Python contract interfaces, helper exports and deterministic checked-in generation. `scripts/generate-metric-bindings.py --check` runs offline. The optional `--metric-sdk-root` check reruns the helper reference generator against the pinned SDK package. Funded workflow execution remains separate from codec/helper compatibility.
