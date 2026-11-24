// Unsigned Metric contract workflows. Canonical ABI encoding and decoding use
// the same pinned contract definitions as the public low-level codec.
const std = @import("std");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const quantity = @import("../../core/transaction.zig").quantity;
const A = w.A;
const V = w.V;
const field = w.field;
const t = w.text;
const router = "MetricOmmSimpleRouter";
const adder = "MetricOmmPoolLiquidityAdder";
const zero = w.text("0x0");

pub const Op = enum {
    create_uniform_bins,
    prepare_factory_create_pool_params,
    prepare_create_pool,
    prepare_liquidity_addition,
    liquidity_native_payment,
    router_multicall,
    native_payment,
    unwrap_output,
    prepare_external_swap,
    prepare_swap_with_fallback,
};
fn obj(a: A, fields: anytype) !V {
    var out: V = .{ .object = .empty };
    inline for (@typeInfo(@TypeOf(fields)).@"struct".field_names) |name| try out.object.put(a, name, @field(fields, name));
    return out;
}
fn list(a: A, values: []const V) !V {
    var out: std.array_list.Managed(V) = .init(a);
    try out.appendSlice(values);
    return .{ .array = out };
}
fn items(v: V) ![]V {
    return if (v == .array) v.array.items else error.InvalidArray;
}
fn get(v: V, k: []const u8, default: V) V {
    return if (v == .object) v.object.get(k) orelse default else default;
}
fn hex(a: A, n: u256) !V {
    return t(try std.fmt.allocPrint(a, "0x{x}", .{n}));
}
fn number(a: A, n: anytype) !V {
    return .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{n}) };
}
fn address(a: A, v: V, nonzero: bool) !V {
    const bytes = try evm.hex(20, try w.string(v));
    if (nonzero and std.mem.allEqual(u8, &bytes, 0)) return error.InvalidAddress;
    return t(try std.fmt.allocPrint(a, "0x{x}", .{&bytes}));
}
fn same(x: V, y: V) bool {
    return x == .string and y == .string and std.ascii.eqlIgnoreCase(x.string, y.string);
}
fn encode(a: A, contract: []const u8, method: []const u8, args: []const V) !V {
    return evm.encodeCall(a, .metric, try obj(a, .{ .contract = t(contract), .function = t(method), .args = try list(a, args) }));
}
fn decode(a: A, contract: []const u8, method: []const u8, data: V) !V {
    return evm.decodeCall(a, .metric, try obj(a, .{ .contract = t(contract), .function = t(method), .data = data }), false);
}
fn prepared(a: A, to: V, data: V, value: V) !V {
    return obj(a, .{ .to = try address(a, to, false), .data = data, .value = try hex(a, try quantity(u256, value)) });
}
fn multi(a: A, contract: []const u8, to: V, calls: V, value: V) !V {
    return prepared(a, to, try encode(a, contract, "multicall", &.{calls}), value);
}
fn pay(a: A, contract: []const u8, to: V, data: V, value: V) !V {
    const refund = try encode(a, contract, "refundETH", &.{});
    return multi(a, contract, to, try list(a, &.{ data, refund }), value);
}
fn uniform(a: A, v: V) !V {
    const count = try quantity(u32, try field(v, "count"));
    const length = try quantity(u16, try field(v, "lengthE6"));
    if (count > 32768 or length == 0) return error.InvalidBin;
    const buy = try quantity(u16, get(v, "addFeeBuyE6", zero));
    const sell = try quantity(u16, get(v, "addFeeSellE6", zero));
    const lane: u256 = length | (@as(u256, buy) << 16) | (@as(u256, sell) << 32);
    var out: std.array_list.Managed(V) = .init(a);
    var i: u32 = 0;
    while (i < count) : (i += 5) {
        var word: u256 = 0;
        for (0..@min(count - i, 5)) |j| word |= lane << @intCast(j * 48);
        try out.append(try hex(a, word));
    }
    return .{ .array = out };
}
fn distance(v: V) !i32 {
    const s = try w.string(v);
    const neg = std.mem.startsWith(u8, s, "-");
    var it = std.mem.splitScalar(u8, s[@intFromBool(neg)..], '.');
    const whole = it.next().?;
    const fraction = it.next() orelse "";
    if (it.next() != null or whole.len == 0) return error.InvalidDistance;
    for (whole) |c| if (!std.ascii.isDigit(c)) return error.InvalidDistance;
    var magnitude = try std.math.mul(u32, try std.fmt.parseInt(u32, whole, 10), 10000);
    const powers = [_]u32{ 1000, 100, 10, 1 };
    for (fraction, 0..) |c, i| {
        if (!std.ascii.isDigit(c) or (i >= 4 and c != '0')) return error.InvalidPrecision;
        if (i < 4) magnitude = try std.math.add(u32, magnitude, (c - '0') * powers[i]);
    }
    if (magnitude > @as(u32, 8388607) + @intFromBool(neg)) return error.InvalidDistance;
    const n: i32 = @intCast(magnitude);
    return if (neg) -n else n;
}
fn factoryParams(a: A, v: V) !V {
    const empty = try list(a, &.{});
    var out = try obj(a, .{});
    for ([_][]const u8{ "token0", "token1", "admin", "adminFeeDestination", "priceProvider" }) |k| try out.object.put(a, k, try address(a, try field(v, k), false));
    for ([_][]const u8{ "initialScaledAmount0PerE18Shares", "initialScaledAmount1PerE18Shares", "minimalOperationalLiquidity" }) |k| try out.object.put(a, k, try hex(a, try quantity(u256, try field(v, k))));
    const x = try quantity(u128, try field(v, "initialScaledAmount0PerE18Shares"));
    const y = try quantity(u128, try field(v, "initialScaledAmount1PerE18Shares"));
    if (@min(x, y) == 0 or @max(x, y) == std.math.maxInt(u128) or try quantity(u256, try field(v, "minimalOperationalLiquidity")) < @max(1, @as(u128, 1000000000000000000000000) / @min(x, y))) return error.InvalidLiquidity;
    const asym = try quantity(u16, try field(v, "bidAskRefAsymmetryX8"));
    if (asym == 0) return error.InvalidAsymmetry;
    try out.object.put(a, "bidAskRefAsymmetryX8", try number(a, asym));
    for ([_][]const u8{ "protocolSpreadFeeE6", "adminSpreadFeeE6", "protocolNotionalFeeE8", "adminNotionalFeeE8" }) |k| try out.object.put(a, k, try number(a, try quantity(u24, try field(v, k))));
    try out.object.put(a, "curBinDistFromProvidedPriceE6", try number(a, try distance(try field(v, "curBinDistFromProvidedPricePercent"))));
    try out.object.put(a, "priceProviderTimelock", try hex(a, try quantity(u256, get(v, "priceProviderTimelock", zero))));
    for ([_][]const u8{ "nonNegativeBins", "negativeBins" }, [_][]const u8{ "nonNegativeBinDataArray", "negativeBinDataArray" }) |src, dest| {
        var arr: std.array_list.Managed(V) = .init(a);
        for (try items(try field(v, src))) |b| try arr.append(try hex(a, try quantity(u256, b)));
        try out.object.put(a, dest, .{ .array = arr });
    }
    const exts = try items(get(v, "extensions", empty));
    const init_data = try items(get(v, "extensionInitData", empty));
    if (exts.len > 7 or exts.len != init_data.len) return error.InvalidExtensions;
    var normalized: std.array_list.Managed(V) = .init(a);
    var data: std.array_list.Managed(V) = .init(a);
    for (exts, init_data) |ext, d| {
        const addr = try address(a, ext, true);
        for (normalized.items) |prev| if (same(prev, addr)) return error.InvalidExtensions;
        try normalized.append(addr);
        try data.append(t(try std.fmt.allocPrint(a, "0x{x}", .{try evm.fromHex(a, try w.string(d))})));
    }
    try out.object.put(a, "extensions", .{ .array = normalized });
    try out.object.put(a, "extensionInitData", .{ .array = data });
    var orders = try obj(a, .{});
    for ([_][]const u8{ "beforeAddLiquidity", "afterAddLiquidity", "beforeRemoveLiquidity", "afterRemoveLiquidity", "beforeSwap", "afterSwap" }) |k| {
        const supplied = get(v, "extensionOrders", .null);
        const n = try quantity(u21, if (supplied == .null) zero else try field(supplied, k));
        for (0..7) |i| if (((n >> @intCast(i * 3)) & 7) > exts.len) return error.InvalidExtensions;
        try orders.object.put(a, k, try hex(a, n));
    }
    try out.object.put(a, "extensionOrders", orders);
    const supplied_salt = get(v, "salt", .null);
    var salt: [32]u8 = undefined;
    if (supplied_salt == .null) {
        var runtime: std.Io.Threaded = .init_single_threaded;
        try runtime.io().randomSecure(&salt);
    } else salt = try evm.hex(32, try w.string(supplied_salt));
    try out.object.put(a, "salt", t(try std.fmt.allocPrint(a, "0x{x}", .{&salt})));
    return out;
}
fn creation(a: A, v: V) !V {
    const to = try address(a, try field(v, "target"), true);
    const params = try factoryParams(a, try field(v, "params"));
    const over = get(v, "helperAsymmetryOverride", .null);
    const data = if (over == .null) try encode(a, "MetricOmmPoolFactory", "createPool", &.{params}) else data: {
        if (try quantity(u16, over) == 0) return error.InvalidAsymmetry;
        break :data try encode(a, "ImmutableAppPoolHelper", "createPool", &.{ params, over });
    };
    return obj(a, .{ .params = params, .call = try prepared(a, to, data, zero) });
}
fn signature(a: A, weighted: bool, owner: bool) ![]const u8 {
    return std.mem.concat(a, u8, &.{ if (weighted) "addLiquidityWeighted" else "addLiquidityExactShares", if (owner) "(address,address," else "(address,", "uint80,(int256[],uint256[]),uint256,uint256,(int16,uint104,int16,uint104),bytes)" });
}
fn signedBin(v: V) !i16 {
    if (v != .number_string) return error.InvalidBin;
    return std.fmt.parseInt(i16, v.number_string, 10);
}
fn liquidity(a: A, v: V) !V {
    const to = try address(a, try field(v, "liquidityAdder"), true);
    const pool = try address(a, try field(v, "pool"), true);
    const owner = get(v, "owner", .null);
    const kind = try w.string(get(v, "kind", t("exactShares")));
    const weighted = std.mem.eql(u8, kind, "weighted");
    if (!weighted and !std.mem.eql(u8, kind, "exactShares")) return error.InvalidKind;
    var bounds = get(v, "binPositionBounds", .null);
    if (bounds == .null) {
        if (weighted) return error.InvalidBounds;
        bounds = try list(a, &.{ t("-32768"), zero, t("32767"), try hex(a, std.math.maxInt(u104)) });
    } else {
        const low = try signedBin(try field(bounds, "minimalCurBin"));
        const high = try signedBin(try field(bounds, "maximalCurBin"));
        const pos = try quantity(u104, try field(bounds, "minimalPosition"));
        const end = try quantity(u104, try field(bounds, "maximalPosition"));
        if (low > high or (low == high and pos > end)) return error.InvalidBounds;
    }
    const deltas = try field(v, "deltas");
    const bins = try items(try field(deltas, "binIdxs"));
    const shares = try items(try field(deltas, "shares"));
    if (bins.len == 0 or bins.len != shares.len) return error.InvalidDeltas;
    var seen: std.AutoHashMapUnmanaged(i16, void) = .empty;
    for (bins) |bin| if ((try seen.getOrPut(a, try signedBin(bin))).found_existing) return error.InvalidDeltas;
    var args: std.array_list.Managed(V) = .init(a);
    try args.append(pool);
    if (owner != .null) try args.append(try address(a, owner, true));
    try args.appendSlice(&.{ try field(v, "salt"), deltas, try field(v, "maxAmountToken0"), try field(v, "maxAmountToken1"), bounds, get(v, "extensionData", t("0x")) });
    const data = try encode(a, adder, try signature(a, weighted, owner != .null), args.items);
    const value = get(v, "nativeValue", .null);
    return if (value == .null) prepared(a, to, data, zero) else pay(a, adder, to, data, value);
}
const Payment = enum { wrapped, nativeInput, unwrapOutput };
fn mode(v: V) !Payment {
    if (v == .null) return .wrapped;
    return std.meta.stringToEnum(Payment, try w.string(try field(v, "type"))) orelse error.InvalidPayment;
}
fn external(a: A, to: V, executor: V, p: V, payment: V, value: V) !V {
    var params = p;
    const input = try address(a, try field(p, "tokenIn"), true);
    const output = try address(a, try field(p, "tokenOut"), true);
    _ = try address(a, try field(p, "externalRouter"), true);
    const recipient = try address(a, try field(p, "recipient"), true);
    if (same(input, output)) return error.InvalidTokens;
    const m = try mode(payment);
    if (m != .nativeInput and try quantity(u256, value) != 0) return error.InvalidPayment;
    if (m != .wrapped) {
        const wrapped = try address(a, try field(payment, "wrappedNative"), true);
        if (!same(if (m == .nativeInput) input else output, wrapped)) return error.InvalidPayment;
    }
    if (m == .unwrapOutput) params.object.getPtr("recipient").?.* = to;
    const data = try encode(a, router, "externalSwap", &.{ try address(a, executor, true), .{ .bool = false }, .{ .bool = m == .nativeInput }, params });
    return switch (m) {
        .wrapped => prepared(a, to, data, value),
        .nativeInput => pay(a, router, to, data, value),
        .unwrapOutput => multi(a, router, to, try list(a, &.{ data, try encode(a, router, "unwrapWETH9", &.{ try field(p, "amountOutMinimum"), recipient }) }), value),
    };
}
const Swap = struct {
    method: []const u8,
    args: V,
    input: V,
    output: V,
    maximum: V,
    minimum: V,
    recipient: V,
    deadline: V,
    fn parse(a: A, data: V) !Swap {
        for ([_][]const u8{ "exactInputSingle", "exactOutputSingle", "exactInput", "exactOutput" }) |method| {
            const args = decode(a, router, method, data) catch |err| if (err == error.SelectorMismatch) continue else return err;
            const p = args.array.items[0].array.items;
            const single = std.mem.endsWith(u8, method, "Single");
            var input: V = undefined;
            var output: V = undefined;
            if (single) {
                input = p[1];
                output = p[2];
            } else {
                const tokens = try items(p[0]);
                const pools = try items(p[1]);
                if (tokens.len < 2 or tokens.len != pools.len + 1 or (try items(p[2])).len != pools.len) return error.InvalidPath;
                input = tokens[0];
                output = tokens[tokens.len - 1];
            }
            const out = std.mem.startsWith(u8, method, "exactOutput");
            return .{ .method = method, .args = args, .input = input, .output = output, .maximum = p[if (out) @as(usize, 5) else 4], .minimum = p[if (out) @as(usize, 4) else 5], .recipient = p[6], .deadline = p[7] };
        }
        return error.InvalidSwap;
    }
    fn withPayment(self: Swap, a: A, to: V, data: V, value: V, payment: Payment, wrapped: V, recipient: V) !V {
        return switch (payment) {
            .wrapped => multi(a, router, to, try list(a, &.{data}), value),
            .nativeInput => if (same(self.input, wrapped)) pay(a, router, to, data, value) else error.InvalidPayment,
            .unwrapOutput => blk: {
                if (!same(self.output, wrapped)) return error.InvalidPayment;
                const dest = try address(a, recipient, true);
                self.args.array.items[0].array.items[6] = to;
                const swap = try encode(a, router, self.method, self.args.array.items);
                const unwrap = try encode(a, router, "unwrapWETH9", &.{ self.minimum, dest });
                break :blk multi(a, router, to, try list(a, &.{ swap, unwrap }), value);
            },
        };
    }
};
fn fallback(a: A, v: V, call: V, swap: Swap) !V {
    const to = try address(a, try field(call, "to"), true);
    const data = try field(call, "data");
    const value = try field(call, "value");
    const config = try field(v, "fallback");
    const payment = get(v, "payment", .null);
    const m = try mode(payment);
    const params = try obj(a, .{ .tokenIn = swap.input, .tokenOut = swap.output, .amountInMaximum = swap.maximum, .amountOutMinimum = swap.minimum, .externalRouter = try field(config, "externalRouter"), .externalRouterCalldata = try field(config, "externalRouterCalldata"), .recipient = swap.recipient, .deadline = swap.deadline });
    const alt = try external(a, to, try field(config, "executor"), params, payment, value);
    const primary = try swap.withPayment(a, to, data, value, m, get(payment, "wrappedNative", .null), swap.recipient);
    const alt_data = try field(alt, "data");
    const atomic = if (m == .wrapped) try encode(a, router, "multicall", &.{try list(a, &.{alt_data})}) else alt_data;
    const steps = try list(a, &.{ try list(a, &.{ try field(primary, "data"), t("1"), zero }), try list(a, &.{ atomic, t("1"), t("2") }) });
    return prepared(a, to, try encode(a, router, "sequence", &.{steps}), value);
}
pub fn execute(a: A, op: Op, v: V) !V {
    if (v != .object) return error.InvalidParameter;
    switch (op) {
        .create_uniform_bins => return uniform(a, v),
        .prepare_factory_create_pool_params => return factoryParams(a, v),
        .prepare_create_pool => return creation(a, v),
        .prepare_liquidity_addition => return liquidity(a, v),
        .router_multicall => return multi(a, router, try field(v, "router"), try field(v, "calls"), try field(v, "value")),
        .prepare_external_swap => return external(a, try address(a, try field(v, "router"), true), try field(v, "executor"), try field(v, "params"), get(v, "payment", .null), get(v, "value", zero)),
        else => {},
    }
    const call = try field(v, "call");
    const data = try field(call, "data");
    const target = try field(call, "to");
    if (op == .liquidity_native_payment) {
        const to = try address(a, target, true);
        for ([_]bool{ false, true }) |weighted| for ([_]bool{ false, true }) |owner| {
            _ = decode(a, adder, try signature(a, weighted, owner), data) catch |err| if (err == error.SelectorMismatch) continue else return err;
            return pay(a, adder, to, data, try field(v, "value"));
        };
        return error.InvalidLiquidity;
    }
    const swap = try Swap.parse(a, data);
    return switch (op) {
        .prepare_swap_with_fallback => fallback(a, v, call, swap),
        .native_payment => swap.withPayment(a, target, data, try field(v, "value"), .nativeInput, try field(v, "wrappedNative"), .null),
        .unwrap_output => swap.withPayment(a, target, data, try field(call, "value"), .unwrapOutput, try field(v, "wrappedNative"), try field(v, "recipient")),
        else => unreachable,
    };
}
