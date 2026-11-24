const std = @import("std");
const w = @import("wire.zig");
const tx = @import("transaction.zig");
const A = w.A;
const V = w.V;
const wad = 1_000_000_000_000_000_000;
const q64 = 1 << 64;
pub const Op = enum { mul_div, price_to_q64, price_x64_to_e18, slippage_limit, percentage_removal, decayed_high_watermark, stop_loss_threshold, stop_loss_breached, seconds_until_stop_loss_clears, stop_loss_metrics, evaluate_oracle_stop_loss, extension_order, direction_bitmap, minimum_operational_liquidity };
pub const Round = enum { down, up, exact };

fn qty(v: V, k: []const u8) !u256 {
    return tx.quantity(u256, try w.field(v, k));
}
fn num(a: A, n: anytype) !V {
    return .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{n}) };
}
fn hex(a: A, n: u256) !V {
    return w.text(try std.fmt.allocPrint(a, "0x{x}", .{n}));
}
fn boolean(v: V, k: []const u8, default: bool) !bool {
    const b = v.object.get(k) orelse return default;
    return if (b == .bool) b.bool else error.InvalidBool;
}
fn array(v: V, k: []const u8) ![]const V {
    const list = try w.field(v, k);
    return if (list == .array) list.array.items else error.InvalidArray;
}

// Restoring division keeps all 512 product bits on every packaged target.
pub fn ratio(n: u512, d: u512, round: Round) !u256 {
    if (d == 0) return error.ZeroDenominator;
    var rem = n;
    var q: u512 = 0;
    if (d <= n) {
        var bit: u9 = @intCast(@clz(d) - @clz(n));
        var aligned = d << bit;
        while (true) {
            if (rem >= aligned) {
                rem -= aligned;
                q |= @as(u512, 1) << bit;
            }
            if (bit == 0) break;
            bit -= 1;
            aligned >>= 1;
        }
    }
    if (rem != 0) switch (round) {
        .exact => return error.InvalidPrecision,
        .up => q += 1,
        .down => {},
    };
    return std.math.cast(u256, q) orelse error.Overflow;
}
pub fn mulDiv(x: u256, y: u256, d: u256, round: Round) !u256 {
    return ratio(@as(u512, x) * y, d, round);
}
fn threshold(hwm: u256, dd: u256) !u256 {
    if (dd > 1000000) return error.InvalidDrawdown;
    return mulDiv(hwm, 1000000 - dd, 1000000, .down);
}
fn decay(hwm: u256, rate: u256, seconds: u64) !u256 {
    const factor = try std.math.mul(u256, rate, seconds);
    return if (factor >= wad) 0 else hwm - try mulDiv(hwm, factor, wad, .down);
}
fn clears(metric: u256, hwm: u256, dd: u256, rate: u256) !?u64 {
    if (dd == 0 or metric >= try threshold(hwm, dd)) return 0;
    if (rate == 0) return null;
    var hi: u64 = @intCast(try mulDiv(wad, 1, rate, .up));
    var lo: u64 = 0;
    while (lo < hi) {
        const mid = lo + (hi - lo) / 2;
        if (metric < try threshold(try decay(hwm, rate, mid), dd)) lo = mid + 1 else hi = mid;
    }
    return lo;
}
fn metrics(v: V, bid: u256, ask: u256, min: u256) ![2]u256 {
    if (bid == 0 or ask == 0) return error.InvalidPrice;
    const count = @max(try qty(v, "totalShares"), min, 1);
    const x = try mulDiv(try qty(v, "token0BalanceScaled"), wad, count, .down);
    const y = try mulDiv(try qty(v, "token1BalanceScaled"), wad, count, .down);
    const cap = std.math.maxInt(u104);
    return .{ @min(cap, try std.math.add(u256, x, try mulDiv(y, q64, bid, .down))), @min(cap, try std.math.add(u256, y, try mulDiv(x, ask, q64, .down))) };
}
fn maybe(a: A, n: ?u64) !V {
    return if (n) |v| num(a, v) else .null;
}
fn evaluate(a: A, v: V) !V {
    const dd = try qty(v, "drawdownE6");
    if (dd > 1000000) return error.InvalidDrawdown;
    const rate = try qty(v, "decayPerSecondE18");
    const bid = try qty(v, "bidPriceX64");
    const ask = try qty(v, "askPriceX64");
    const min = try qty(v, "minimalOperationalLiquidity");
    const skip = try boolean(v, "skipEmptyBins", true);
    var bins: std.array_list.Managed(V) = .init(a);
    var breached: std.array_list.Managed(V) = .init(a);
    var resumes: [2]?u64 = .{ 0, 0 };
    const names = [_][]const u8{ "secondsUntilZeroForOneResumes", "secondsUntilOneForZeroResumes" };
    for (try array(v, "bins")) |bin| {
        const idx = try w.field(bin, "binIdx");
        if (idx != .number_string) return error.InvalidBin;
        _ = try std.fmt.parseInt(i16, idx.number_string, 10);
        const hwm = [2]u256{ try qty(bin, "highWatermark0"), try qty(bin, "highWatermark1") };
        const shares = try qty(bin, "totalShares");
        if (skip and shares == 0) continue;
        const metric = if (dd != 0) try metrics(bin, bid, ask, min) else [2]u256{ 0, 0 };
        var item: V = .{ .object = .empty };
        try item.object.put(a, "binIdx", idx);
        var any = false;
        for (0..2) |i| {
            const t = if (dd != 0) try threshold(hwm[i], dd) else 0;
            const hit = metric[i] < t;
            const wait = try clears(metric[i], hwm[i], dd, rate);
            try item.object.put(a, if (i == 0) "metricToken0" else "metricToken1", try hex(a, metric[i]));
            try item.object.put(a, if (i == 0) "highWatermark0" else "highWatermark1", try hex(a, hwm[i]));
            try item.object.put(a, if (i == 0) "threshold0" else "threshold1", try hex(a, t));
            try item.object.put(a, if (i == 0) "breachedToken0" else "breachedToken1", .{ .bool = hit });
            try item.object.put(a, if (i == 0) "blocksZeroForOne" else "blocksOneForZero", .{ .bool = hit });
            try item.object.put(a, names[i], try maybe(a, wait));
            if (hit) {
                any = true;
                resumes[i] = if (resumes[i] != null and wait != null) @max(resumes[i].?, wait.?) else null;
            }
        }
        try bins.append(item);
        if (any) try breached.append(item);
    }
    var out: V = .{ .object = .empty };
    try out.object.put(a, "enabled", .{ .bool = dd != 0 });
    try out.object.put(a, "isAnyBinStopLossBreached", .{ .bool = breached.items.len != 0 });
    try out.object.put(a, "bins", .{ .array = bins });
    try out.object.put(a, "breachedBins", .{ .array = breached });
    for (names, resumes) |name, seconds| try out.object.put(a, name, try maybe(a, seconds));
    return out;
}
pub fn execute(a: A, op: Op, v: V) !V {
    if (v != .object) return error.InvalidParameter;
    const round = if (v.object.get("rounding")) |r| std.meta.stringToEnum(Round, try w.string(r)) orelse return error.InvalidRounding else .exact;
    const result: u256 = switch (op) {
        .evaluate_oracle_stop_loss => return evaluate(a, v),
        .mul_div => try mulDiv(try qty(v, "a"), try qty(v, "b"), try qty(v, "denominator"), round),
        .price_to_q64 => try mulDiv(try qty(v, "numerator"), q64, try qty(v, "denominator"), round),
        .price_x64_to_e18 => blk: {
            const p = try mulDiv(try qty(v, "price"), wad, q64, .down);
            if (p == 0) return error.InvalidPrice;
            break :blk p;
        },
        .slippage_limit, .percentage_removal => blk: {
            const bps = try qty(v, "bps");
            if (bps > 10000) return error.InvalidBps;
            if (op == .percentage_removal) break :blk try mulDiv(try qty(v, "shares"), bps, 10000, .down);
            const output = try boolean(v, "exactOutput", false);
            break :blk try mulDiv(try qty(v, "amount"), if (output) 10000 + bps else 10000 - bps, 10000, if (output) .up else .down);
        },
        .minimum_operational_liquidity => blk: {
            const x = try qty(v, "amount0");
            const y = try qty(v, "amount1");
            if (@min(x, y) == 0 or @max(x, y) >= std.math.maxInt(u128)) return error.InvalidAmount;
            break :blk @max(1, try mulDiv(1000000, wad, @min(x, y), .down));
        },
        .extension_order, .direction_bitmap => blk: {
            const ext = op == .extension_order;
            const items = try array(v, if (ext) "slots" else "directions");
            if (items.len > @as(usize, if (ext) 7 else 256)) return error.InvalidArray;
            var value: u256 = 0;
            for (items, 0..) |item, i| {
                const k: u256 = if (ext) try tx.quantity(u3, item) else if (item == .bool) @intFromBool(item.bool) else return error.InvalidBool;
                value |= k << @intCast(if (ext) i * 3 else i);
            }
            break :blk value;
        },
        .decayed_high_watermark => try decay(try qty(v, "hwm"), try qty(v, "rate"), try tx.quantity(u64, try w.field(v, "seconds"))),
        .stop_loss_threshold => try threshold(try qty(v, "hwm"), try qty(v, "drawdownE6")),
        .stop_loss_breached => {
            const dd = try qty(v, "drawdownE6");
            return .{ .bool = dd != 0 and try qty(v, "metric") < try threshold(try qty(v, "hwm"), dd) };
        },
        .seconds_until_stop_loss_clears => return maybe(a, try clears(try qty(v, "metric"), try qty(v, "hwm"), try qty(v, "drawdownE6"), try qty(v, "rate"))),
        .stop_loss_metrics => {
            const pair = try metrics(v, try qty(v, "bidPriceX64"), try qty(v, "askPriceX64"), try qty(v, "minimalOperationalLiquidity"));
            var out: V = .{ .object = .empty };
            try out.object.put(a, "metricToken0", w.text(try std.fmt.allocPrint(a, "{d}", .{pair[0]})));
            try out.object.put(a, "metricToken1", w.text(try std.fmt.allocPrint(a, "{d}", .{pair[1]})));
            return out;
        },
    };
    return num(a, result);
}
