"""Exact Metric arithmetic and native transaction helpers implemented in Rust.

Functions accept native mappings; integer inputs use Python int or exact strings.
Floats are rejected. Rounding defaults to exact unless an operation defines native rounding.
"""
from . import _native
from ._metric import MetricClient, MetricChainClient, MetricSigner


def pack_bins(bins):
    return _native.metric_tool("pack_bins", {"bins": bins})


def codebook():
    return _native.metric_tool("codebook", {})


def addresses(chain_id):
    return _native.metric_tool("addresses", {"chainId": chain_id})


def unpack_bins(request):
    return _native.metric_tool("unpack_bins", request)


def decimal_to_raw(request):
    return _native.metric_tool("decimal_to_raw", request)


def mul_div(request):
    return _native.metric_tool("mul_div", request)


def price_to_q64(request):
    return _native.metric_tool("price_to_q64", request)


def price_x64_to_e18(request):
    return _native.metric_tool("price_x64_to_e18", request)


def slippage_limit(request):
    return _native.metric_tool("slippage_limit", request)


def position_bin_key(request):
    return _native.metric_tool("position_bin_key", request)


def encode_bin_state(request):
    return _native.metric_tool("encode_bin_state", request)


def decode_bin_state(request):
    return _native.metric_tool("decode_bin_state", request)


def unpack_swap_details(request):
    return _native.metric_tool("unpack_swap_details", request)


def unpack_two_uint128(request):
    return _native.metric_tool("unpack_two_uint128", request)


def unpack_two_int128(request):
    return _native.metric_tool("unpack_two_int128", request)


def direction_bitmap(request):
    return _native.metric_tool("direction_bitmap", request)


def liquidity_cursor_bounds(request):
    return _native.metric_tool("liquidity_cursor_bounds", request)


def minimum_operational_liquidity(request):
    return _native.metric_tool("minimum_operational_liquidity", request)


def recommended_initial_scaled_amounts(request):
    return _native.metric_tool("recommended_initial_scaled_amounts", request)


def extension_order(request):
    return _native.metric_tool("extension_order", request)


def percentage_removal(request):
    return _native.metric_tool("percentage_removal", request)


def scale_weighted_deltas(request):
    return _native.metric_tool("scale_weighted_deltas", request)


def stop_loss_metrics(request):
    return _native.metric_tool("stop_loss_metrics", request)


def decayed_high_watermark(request):
    return _native.metric_tool("decayed_high_watermark", request)


def stop_loss_threshold(request):
    return _native.metric_tool("stop_loss_threshold", request)


def stop_loss_breached(request):
    return _native.metric_tool("stop_loss_breached", request)


def seconds_until_stop_loss_clears(request):
    return _native.metric_tool("seconds_until_stop_loss_clears", request)


def encode_u64x32(request):
    return _native.metric_tool("encode_u64x32", request)


def decode_u64x32(request):
    return _native.metric_tool("decode_u64x32", request)


def encode_spread(request):
    return _native.metric_tool("encode_spread", request)


def pack_oracle_slot(request):
    return _native.metric_tool("pack_oracle_slot", request)


def oracle_push_calldata(request):
    return _native.metric_tool("oracle_push_calldata", request)


def feed_id(request):
    return _native.metric_tool("feed_id", request)


def slot_update_digest(request):
    return _native.metric_tool("slot_update_digest", request)


def pusher_consent_digest(request):
    return _native.metric_tool("pusher_consent_digest", request)


def native_payment(request):
    return _native.metric_tool("native_payment", request)


def unwrap_output(request):
    return _native.metric_tool("unwrap_output", request)

def addition_plan(request):
    return _native.metric_tool("addition_plan", request)

def weighted_plan(request):
    return _native.metric_tool("weighted_plan", request)

def removal_plan(request):
    return _native.metric_tool("removal_plan", request)


def prepare_factory_create_pool_params(request):
    """Resolve factory defaults, exact percentage scaling, validation and pool salt."""
    return _native.metric_tool("prepare_factory_create_pool_params", request)


def prepare_create_pool(request):
    """Return resolved parameters and unsigned factory/helper calldata; never submits."""
    return _native.metric_tool("prepare_create_pool", request)


def create_uniform_bins(request):
    """Pack uniform bins with native length and optional buy/sell fee fields."""
    return _native.metric_tool("create_uniform_bins", request)


def prepare_liquidity_addition(request):
    """Build exact-shares/weighted addition; nativeValue appends an ETH refund."""
    return _native.metric_tool("prepare_liquidity_addition", request)


def liquidity_native_payment(request):
    """Wrap a prepared liquidity-adder call with refundETH and explicit value."""
    return _native.metric_tool("liquidity_native_payment", request)


def prepare_external_swap(request):
    """Build an external swap with optional native input or output unwrapping."""
    return _native.metric_tool("prepare_external_swap", request)


def prepare_swap_with_fallback(request):
    """Build an atomic native/external fallback sequence preserving swap limits."""
    return _native.metric_tool("prepare_swap_with_fallback", request)


def evaluate_oracle_stop_loss(request):
    """Evaluate per-bin breaches and time until both swap directions resume."""
    return _native.metric_tool("evaluate_oracle_stop_loss", request)
