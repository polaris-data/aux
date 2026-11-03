# Metric workflow preparation

These helpers return inspectable parameters, evaluation results or unsigned calls. They do not sign, submit, grant approvals or move funds. Rust owns the implementation; Python's `aux_py.metric` functions expose the same native requests and results.

## Pool creation

`creation::prepare_factory_create_pool_params(CreatePoolParams)` supplies empty extension lists/orders, zero provider timelock and a fresh salt when omitted. It validates minimum operational liquidity, initial scaled amounts, positive bid/ask/reference asymmetry, extension slots/init-data lengths and ABI integer bounds. The fee fields retain the supplied native values; factory defaults are applied by the contract.

Use `creation::prepare_create_pool(PoolCreationRequest { target, params, helper_asymmetry_override })` to produce both resolved parameters and a `PreparedCall`. `target` is the factory when the override is absent. Supplying an override selects `ImmutableAppPoolHelper` as the target ABI and validates the additional positive asymmetry bound. Python exposes `prepare_factory_create_pool_params` and `prepare_create_pool` with camel-case mappings.

`curBinDistFromProvidedPricePercent` is an exact decimal string, for example `"-0.0125"`. Conversion to pool E6 distance multiplies the percentage by 10,000 and checks signed int24 bounds. Excess precision is rejected; unlike the JavaScript SDK, the connector does not silently round a floating-point percentage. `create_uniform_bins({count, lengthE6, addFeeBuyE6?, addFeeSellE6?})` creates the packed bin arrays.

An omitted salt uses OS entropy. The result includes the resolved salt alongside the calldata. Save that result to inspect or reuse the same deployment request; calling preparation again without a salt creates another request. Supplying an explicit salt makes preparation deterministic.

## Liquidity additions with native ETH

`workflows::prepare_liquidity_addition` accepts exact-shares or weighted deltas, self or explicit position owner, token caps, cursor bounds and extension data. Supplying `nativeValue` appends the native `refundETH()` call inside the LiquidityAdder multicall and sets transaction value, including when the contribution is zero. Omitting it returns the ordinary ERC-20 addition.

```python
from aux_py import metric

call = metric.prepare_liquidity_addition({
    "liquidityAdder": adder_address,
    "pool": pool_address,
    "salt": 7,
    "kind": "exactShares",  # "weighted" uses the same deltas field for weights
    "deltas": plan["deltas"],
    "maxAmountToken0": plan["maxAmountToken0"],
    "maxAmountToken1": plan["maxAmountToken1"],
    "binPositionBounds": cursor_bounds,
    "nativeValue": native_contribution,
})
```

Set `owner` to credit another position owner while the caller pays. The pool must contain the chain's wrapped-native token, and any ERC-20 remainder still requires the native allowance. Weighted additions require explicit cursor bounds. Exact-shares additions use the SDK's unconstrained bounds when omitted; pass bounds from `liquidity_cursor_bounds` to constrain cursor movement. To explicitly disable bounds for a weighted addition, supply minimum bin `-32768`, minimum position `0`, maximum bin `32767`, maximum position `2**104-1`.

`liquidity_native_payment({call, value})` also wraps an existing canonical exact-shares/weighted LiquidityAdder call. It rejects other selectors and already wrapped multicalls.

## External swaps and fallback sequences

`workflows::prepare_external_swap` prepares external execution, while `prepare_swap_with_fallback` wraps an original `exactInput`, `exactOutput`, `exactInputSingle` or `exactOutputSingle` call. Supply an executor, external router and external router calldata. The helper carries the original input cap, output floor, recipient and deadline into the external leg.

```python
fallback_call = metric.prepare_swap_with_fallback({
    "call": original_swap_call,
    "fallback": {
        "executor": executor_address,
        "externalRouter": external_router_address,
        "externalRouterCalldata": external_calldata,
    },
    "payment": {"type": "wrapped"},
})
```

Payment can instead be `{"type": "nativeInput", "wrappedNative": address}` or `{"type": "unwrapOutput", "wrappedNative": address}`. Native input uses the original call's `value`, validates the input token and appends refunds on both branches. Unwrapping validates the output token, directs output to the router and appends the unwrap transfer to the original recipient. Wrapped and output-only modes reject accidental native input value.

Each branch runs in an atomic multicall. The sequence stops on primary success, continues after primary failure and reverts if the fallback fails. This is one on-chain transaction; there is no client retry or resubmission. External calldata is preserved verbatim. Its embedded approvals/recipients and executor compatibility remain caller-supplied native requirements.

## Combined stop-loss status

`stop_loss::evaluate_oracle_stop_loss` / `metric.evaluate_oracle_stop_loss` accepts bin balances/shares, current already-decayed watermarks, bid/ask Q64 prices, drawdown, decay and minimum liquidity. It returns per-bin metrics, thresholds, directional breach flags, breached bins and the maximum resume time for each direction. A `None`/`null` resume time means a breach cannot clear through decay. Empty bins are skipped by default. Disabled drawdown yields clear status without attempting price arithmetic.

`MetricChainClient.oracle_stop_loss_status(request)` obtains the native configuration, balances, shares and watermarks before evaluating them:

```python
status = await chain.oracle_stop_loss_status({
    "pool": pool_address,
    "extension": stop_loss_extension_address,
    "dataProvider": data_provider_address,
    "bidPriceX64": bid_price_x64,
    "askPriceX64": ask_price_x64,
})
```

The reader resolves `latest`/`safe`/`finalized` to one block and anchors all subsequent calls to it. It rejects pending-block reads and crossed/equal bid/ask prices. It returns no partial status on a failed read. `maxBins` defaults to 512; increase it explicitly for larger pool ranges, up to 65,536. Bin-array reads are chunked at 512. A breach in any bin does not imply that every swap path crosses a breached bin; use the directional per-bin flags for path-specific decisions.

## Reference checks

[Helper fixtures](tests/fixtures/helpers.json) contain 58 cases generated from SDK 0.7.0: factory/helper creation, uniform bins, all liquidity variants, twelve fallback variants, external payments, combined evaluation and native status reads. Both Rust and Python consume those cases. Negative tests cover precision/range rejection, invalid payment modes, missing weighted bounds and failed/bounded state reads.

Regenerate/compare with an extracted pinned SDK and its generation-time Node dependencies:

```sh
python3 scripts/check-native-coverage.py --metric-sdk-root /path/to/metric-omm-sdk-v1/package
```

Normal CI reads the checked-in fixtures without downloading or importing the official SDK. These checks establish helper parity for the listed groups; funded workflow execution is recorded separately in [VERIFICATION.md](VERIFICATION.md).
