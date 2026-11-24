const std = @import("std");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const V = w.V;

fn optional(v: V, key: []const u8) ?V {
    const value = v.object.get(key) orelse return null;
    return if (value == .null) null else value;
}

fn addr(v: V) ![20]u8 {
    return evm.hex(20, try w.string(v));
}

fn addrList(values: V, limit: ?usize) !void {
    if (values != .array) return error.InvalidArray;
    if (limit) |n| if (values.array.items.len == 0 or values.array.items.len > n) return error.InvalidBatch;
    for (values.array.items) |value| _ = try addr(value);
}

fn integer(comptime T: type, v: V) !T {
    if (v != .number_string) return error.InvalidInteger;
    return w.unsigned(T, v);
}

pub fn validate(a: w.A, name: []const u8, v: V) !void {
    if (std.mem.eql(u8, name, "batch_bid_ask")) {
        try addrList(try w.field(v, "pools"), 100);
        if (optional(v, "depth")) |b| if (b != .bool) return error.InvalidBool;
    } else if (std.mem.eql(u8, name, "route")) {
        const src = try addr(try w.field(v, "tokenIn"));
        const dst = try addr(try w.field(v, "tokenOut"));
        if (std.mem.allEqual(u8, &src, 0) or std.mem.allEqual(u8, &dst, 0) or std.mem.eql(u8, &src, &dst)) return error.InvalidToken;
        const amount_in = optional(v, "amountIn");
        const amount_out = optional(v, "amountOut");
        if (amount_in) |x| _ = try w.string(x);
        if (amount_out) |x| _ = try w.string(x);
        const exact_in = if (optional(v, "tradeType")) |value| blk: {
            const t = try w.string(value);
            if (std.mem.eql(u8, t, "exact_in")) break :blk true;
            if (std.mem.eql(u8, t, "exact_out")) break :blk false;
            return error.InvalidTradeType;
        } else if ((amount_in == null) != (amount_out == null)) amount_in != null else return error.InvalidTradeType;
        const amount = try w.string((if (exact_in) amount_in else amount_out) orelse return error.InvalidAmount);
        if (amount.len == 0 or std.mem.allEqual(u8, amount, '0')) return error.InvalidAmount;
        for (amount) |b| if (!std.ascii.isDigit(b)) return error.InvalidAmount;
        if (optional(v, "poolAddress")) |p| _ = try addr(p);
        if (optional(v, "maxRoutes")) |n| _ = try integer(u16, n);
    } else if (std.mem.eql(u8, name, "pamm_quote")) {
        const chain = try integer(u64, try w.field(v, "chainId"));
        if (chain != 1 and chain != 56) return error.InvalidChain;
        if (try integer(u64, try w.field(v, "replacementSeqNumber")) == 0) return error.InvalidSequence;
        _ = try evm.hex(16, try w.string(try w.field(v, "replacementUuid")));
        const block = try integer(u64, try w.field(v, "blockNumber"));
        const raw = try w.string(try w.field(v, "tx"));
        const hex = if (std.mem.startsWith(u8, raw, "0x")) raw[2..] else raw;
        if (hex.len != 0) {
            if (hex.len % 2 != 0 or (chain == 56 and block == 0)) return error.InvalidTransaction;
            const bytes = try a.alloc(u8, hex.len / 2);
            _ = try std.fmt.hexToBytes(bytes, hex);
            _ = try @import("../../core/transaction.zig").rawSender(a, bytes, chain);
        }
        if (optional(v, "responseTimeoutMs")) |t| if (try integer(u16, t) > 500) return error.InvalidTimeout;
        if (optional(v, "maxTimestamp")) |t| {
            const n = try integer(u64, t);
            var io: std.Io.Threaded = .init_single_threaded;
            const now = std.Io.Timestamp.now(io.io(), .real).toSeconds();
            if (n == 0 or n >= 1_000_000_000_000 or (chain == 56 and hex.len != 0 and (now < 0 or n > @as(u64, @intCast(now)) +| 5))) return error.InvalidTimestamp;
        }
        if (optional(v, "poolAddresses")) |p| try addrList(p, null);
        if (optional(v, "quoteAddress")) |p| _ = try addr(p);
        if (optional(v, "assetPairs")) |p| {
            if (p != .array) return error.InvalidArray;
            for (p.array.items) |pair| _ = try evm.hex(40, try w.string(pair));
        }
        for ([_][]const u8{ "disableCrossRegionSharing", "alwaysInclude" }) |k| if (optional(v, k)) |b| if (b != .bool) return error.InvalidBool;
    }
}
