//! Resolve a committed block once for each composite read.
const std = @import("std");
const w = @import("wire.zig");
const evm = @import("evm.zig");
const rpc = @import("rpc.zig");
const math = @import("metric_math.zig");
const quantity = @import("transaction.zig").quantity;
const A = w.A;
const V = w.V;
const eq = std.mem.eql;
fn obj(a: A, fields: anytype) !V {
    var out = V{ .object = .empty };
    inline for (@typeInfo(@TypeOf(fields)).@"struct".field_names) |k| try out.object.put(a, k, @field(fields, k));
    return out;
}
fn list(a: A, fields: []const V) !V {
    var out = std.json.Array.init(a);
    try out.appendSlice(fields);
    return .{ .array = out };
}
fn text(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn at(v: V, i: usize) !V {
    if (v != .array or v.array.items.len <= i) return error.InvalidResponse;
    return v.array.items[i];
}
fn arr(v: V) ![]const V {
    return if (v == .array) v.array.items else error.InvalidArray;
}
fn index(v: V) !i16 {
    const s = if (v == .number_string) v.number_string else try w.string(v);
    return std.fmt.parseInt(i16, s, 10);
}
fn num(a: A, n: anytype) !V {
    return .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{n}) };
}
fn hex(a: A, n: u256) !V {
    return w.text(try std.fmt.allocPrint(a, "0x{x}", .{n}));
}
fn hexV(a: A, v: V) !V {
    return hex(a, try quantity(u256, v));
}
fn addr(a: A, v: V) !V {
    const bytes = try evm.hex(20, try w.string(v));
    if (std.mem.allEqual(u8, &bytes, 0)) return error.InvalidAddress;
    return w.text(try std.fmt.allocPrint(a, "0x{x}", .{&bytes}));
}
fn equal(x: V, y: V) bool {
    return x == .string and y == .string and eq(u8, x.string, y.string);
}
const Reader = struct {
    a: A,
    url: V,
    block: V,
    fn call(self: Reader, method: []const u8, p: V) !V {
        return rpc.execute(self.a, try obj(self.a, .{ .endpoint = self.url, .operation = w.text(method), .params = p }));
    }
    fn pin(self: *Reader) !void {
        const tag = try w.string(self.block);
        if (eq(u8, tag, "latest")) self.block = try hex(self.a, try quantity(u64, try self.call("block_number", .{ .object = .empty }))) else if (eq(u8, tag, "safe") or eq(u8, tag, "finalized")) self.block = try w.field(try self.call("block", try obj(self.a, .{ .block = self.block })), "number") else if (eq(u8, tag, "earliest")) self.block = w.text("0x0");
        _ = try rpc.quantity(u64, self.block);
    }
    fn read(self: Reader, contract: []const u8, method: []const u8, to: V, args: []const V) !V {
        var arena = std.heap.ArenaAllocator.init(std.heap.c_allocator);
        defer arena.deinit();
        const a = arena.allocator();
        const tmp = Reader{ .a = a, .url = self.url, .block = self.block };
        var spec = try obj(a, .{ .contract = w.text(contract), .function = w.text(method), .args = try list(a, args) });
        const data = try evm.encodeCall(a, .metric, spec);
        const result = try tmp.call("eth_call", try obj(a, .{ .to = to, .data = data, .block = self.block, .value = w.text("0x0") }));
        try spec.object.put(a, "data", result);
        const decoded = try evm.decodeCall(a, .metric, spec, true);
        return w.parse(self.a, try std.json.Stringify.valueAlloc(a, decoded, .{}));
    }
    fn imm(self: Reader, pool: V) !V {
        return at(try self.read("MetricOmmPool", "getImmutables", pool, &.{}), 0);
    }
    fn bins(self: Reader, pool: V, lens: V, ids: V) !std.json.Array {
        const args = [_]V{ pool, ids };
        const words = try arr(try at(try self.read("MetricOmmPoolDataProvider", "binStates", lens, &args), 0));
        const shares = try arr(try at(try self.read("MetricOmmPoolDataProvider", "binTotalShares(address,int16[])", lens, &args), 0));
        const indices = try arr(ids);
        if (words.len != indices.len or shares.len != indices.len) return error.InvalidResponse;
        var out = std.json.Array.init(self.a);
        for (indices, words, shares) |id, word, share| {
            const n = try quantity(u256, word);
            try out.append(try obj(self.a, .{ .bin = id, .token0BalanceScaled = try hex(self.a, @as(u104, @truncate(n))), .token1BalanceScaled = try hex(self.a, @as(u104, @truncate(n >> 104))), .totalShares = try hexV(self.a, share) }));
        }
        return out;
    }
};
fn snapshot(r: Reader, p: V) !V {
    const a = r.a;
    const pool = try addr(a, try w.field(p, "pool"));
    const lens = try addr(a, try w.field(p, "lens"));
    const ids = try w.field(p, "bins");
    if ((try arr(ids)).len > 512) return error.TooManyBins;
    for (ids.array.items) |id| _ = try index(id);
    const imm = try r.imm(pool);
    const slot = try r.read("MetricOmmPoolDataProvider", "slot0", lens, &.{pool});
    const result = try obj(a, .{ .block = r.block, .bins = V{ .array = try r.bins(pool, lens, ids) }, .token0ScaleMultiplier = try hexV(a, try at(imm, 3)), .token1ScaleMultiplier = try hexV(a, try at(imm, 4)), .initialScaledToken0PerShareE18 = try hexV(a, try at(imm, 5)), .initialScaledToken1PerShareE18 = try hexV(a, try at(imm, 6)), .lowestBin = try num(a, try index(try at(imm, 9))), .highestBin = try num(a, try index(try at(imm, 10))), .curBinIdx = try num(a, try index(try at(slot, 1))), .curPosInBin = try hexV(a, try at(slot, 2)) });
    try @import("metric_planning.zig").validateSnapshot(a, result);
    return result;
}
fn stopLoss(r: Reader, p: V) !V {
    const a = r.a;
    const pool = try addr(a, try w.field(p, "pool"));
    const lens = try addr(a, try w.field(p, "dataProvider"));
    const ext = try addr(a, try w.field(p, "extension"));
    const bid = try quantity(u256, try w.field(p, "bidPriceX64"));
    const ask = try quantity(u256, try w.field(p, "askPriceX64"));
    const max = try quantity(u32, p.object.get("maxBins") orelse w.text("512"));
    if (bid == 0 or bid >= ask or max == 0 or max > 65536) return error.InvalidParameter;
    const imm = try r.imm(pool);
    const config = try r.read("OracleValueStopLossExtension", "oracleStopLossConfig", ext, &.{pool});
    const slot = try r.read("MetricOmmPoolDataProvider", "slot0", lens, &.{pool});
    const low = try index(try at(imm, 9));
    const high = try index(try at(imm, 10));
    if (low > high or @as(i32, high) - low + 1 > max) return error.InvalidBinRange;
    var inputs = std.json.Array.init(a);
    var next: i32 = low;
    while (next <= high) {
        var ids = std.json.Array.init(a);
        const end = @min(next + 512, @as(i32, high) + 1);
        while (next < end) : (next += 1) try ids.append(try num(a, next));
        const batch = try r.bins(pool, lens, .{ .array = ids });
        for (batch.items) |row| {
            if (try quantity(u256, try w.field(row, "totalShares")) == 0) continue;
            const id = try w.field(row, "bin");
            const hwm = try r.read("OracleValueStopLossExtension", "currentHighWatermarks", ext, &.{ pool, id });
            try inputs.append(try obj(a, .{ .binIdx = id, .token0BalanceScaled = try w.field(row, "token0BalanceScaled"), .token1BalanceScaled = try w.field(row, "token1BalanceScaled"), .totalShares = try w.field(row, "totalShares"), .highWatermark0 = try hexV(a, try at(hwm, 0)), .highWatermark1 = try hexV(a, try at(hwm, 1)) }));
        }
    }
    const dd = try num(a, try quantity(u32, try at(config, 0)));
    const decay = try hexV(a, try at(config, 1));
    const minimum = try hexV(a, try at(imm, 7));
    const b = try hex(a, bid);
    const ask_v = try hex(a, ask);
    var eval = try math.execute(a, .evaluate_oracle_stop_loss, try obj(a, .{ .bins = V{ .array = inputs }, .bidPriceX64 = b, .askPriceX64 = ask_v, .drawdownE6 = dd, .decayPerSecondE18 = decay, .minimalOperationalLiquidity = minimum, .skipEmptyBins = V{ .bool = true } }));
    var other = try obj(a, .{ .config = try obj(a, .{ .drawdownE6 = dd, .decayPerSecondE18 = decay, .timelock = try num(a, try quantity(u64, try at(config, 2))), .initialized = try at(config, 3) }), .bidPriceX64 = b, .askPriceX64 = ask_v, .curBinIdx = try num(a, try index(try at(slot, 1))), .lowestBin = try num(a, low), .highestBin = try num(a, high), .minimalOperationalLiquidity = minimum, .block = r.block });
    var fields = other.object.iterator();
    while (fields.next()) |field| try eval.object.put(a, field.key_ptr.*, field.value_ptr.*);
    return eval;
}
fn route(r: Reader, p: V) !V {
    const a = r.a;
    const res = try w.field(p, "route");
    const routes = try arr(try w.field(res, "routes"));
    if (routes.len != 1) return error.InvalidRoute;
    const option = routes[0];
    const legs = try arr(try w.field(option, "legs"));
    const trade = try w.field(res, "tradeType");
    const exact_out = equal(trade, w.text("exact_out"));
    if ((!exact_out and !equal(trade, w.text("exact_in"))) or !equal(trade, try w.field(option, "tradeType")) or legs.len == 0 or legs.len > 256) return error.InvalidRoute;
    const first = try addr(a, try w.field(res, "tokenIn"));
    const last = try addr(a, try w.field(res, "tokenOut"));
    if (equal(first, last)) return error.InvalidRoute;
    const input = try quantity(u256, try w.field(option, "amountIn"));
    const output = try quantity(u256, try w.field(option, "amountOut"));
    const key = if (exact_out) @as([]const u8, "amountOut") else "amountIn";
    if (input == 0 or output == 0 or try quantity(u256, try w.field(res, key)) != (if (exact_out) output else input)) return error.PartialFill;
    const bps = try quantity(u16, try w.field(p, "slippageBps"));
    if (bps > 10000) return error.InvalidBps;
    const limit = if (exact_out) try math.mulDiv(input, 10000 + @as(u256, bps), 10000, .up) else try math.mulDiv(output, 10000 - @as(u256, bps), 10000, .down);
    const exts = p.object.get("extensionDatas") orelse .null;
    if (exts != .null and (try arr(exts)).len != legs.len) return error.InvalidExtensions;
    var pools = std.json.Array.init(a);
    var tokens = std.json.Array.init(a);
    var extensions = std.json.Array.init(a);
    try tokens.append(first);
    var bitmap: u256 = 0;
    var current = first;
    for (legs, 0..) |leg, i| {
        const pool = try addr(a, try w.field(leg, "poolAddress"));
        const imm = try r.imm(pool);
        const t0 = try addr(a, try at(imm, 1));
        const t1 = try addr(a, try at(imm, 2));
        const direction = equal(current, t0);
        if (!direction and !equal(current, t1)) return error.InvalidRoute;
        const to = if (direction) t1 else t0;
        const d = try w.field(leg, "zeroForOne");
        if (d != .bool or d.bool != direction or !equal(trade, try w.field(leg, "tradeType")) or !equal(current, try addr(a, try w.field(leg, "tokenIn"))) or !equal(to, try addr(a, try w.field(leg, "tokenOut")))) return error.InvalidRoute;
        if (i > 0 and try quantity(u256, try w.field(legs[i - 1], "amountOut")) != try quantity(u256, try w.field(leg, "amountIn"))) return error.InvalidRoute;
        if (direction) bitmap |= @as(u256, 1) << @intCast(i);
        try pools.append(pool);
        try tokens.append(to);
        try extensions.append(if (exts == .null) w.text("0x") else exts.array.items[i]);
        current = to;
    }
    if (!equal(current, last)) return error.InvalidRoute;
    var params = try obj(a, .{ .recipient = try addr(a, try w.field(p, "recipient")), .deadline = try hexV(a, try w.field(p, "deadline")) });
    try params.object.put(a, key, try hex(a, if (exact_out) output else input));
    try params.object.put(a, if (exact_out) "amountInMaximum" else "amountOutMinimum", try hex(a, limit));
    const single = legs.len == 1;
    if (single) {
        try params.object.put(a, "pool", pools.items[0]);
        try params.object.put(a, "tokenIn", first);
        try params.object.put(a, "tokenOut", last);
        try params.object.put(a, "zeroForOne", .{ .bool = bitmap != 0 });
        try params.object.put(a, "priceLimitX64", try w.field(legs[0], "priceLimitX64"));
        try params.object.put(a, "extensionData", extensions.items[0]);
    } else {
        try params.object.put(a, "tokens", .{ .array = tokens });
        try params.object.put(a, "pools", .{ .array = pools });
        try params.object.put(a, "extensionDatas", .{ .array = extensions });
        try params.object.put(a, "zeroForOneBitMap", try hex(a, bitmap));
    }
    const method: []const u8 = if (exact_out) (if (single) "exactOutputSingle" else "exactOutput") else if (single) "exactInputSingle" else "exactInput";
    const data = try evm.encodeCall(a, .metric, try obj(a, .{ .contract = w.text("MetricOmmSimpleRouter"), .function = w.text(method), .args = try obj(a, .{ .params = params }) }));
    return obj(a, .{ .to = try addr(a, try w.field(p, "router")), .data = data, .value = w.text("0x0") });
}
pub fn execute(a: A, c: V) !V {
    const kind = std.meta.stringToEnum(enum { liquidity_snapshot, oracle_stop_loss_status, prepare_route }, try text(c, "operation")) orelse return error.InvalidOperation;
    const p = try w.field(c, "params");
    if (p != .object) return error.InvalidParameter;
    var reader = Reader{ .a = a, .url = w.text(try rpc.checkedUrl(try text(c, "endpoint"))), .block = p.object.get("block") orelse w.text("latest") };
    try reader.pin();
    return switch (kind) {
        .liquidity_snapshot => snapshot(reader, p),
        .oracle_stop_loss_status => stopLoss(reader, p),
        .prepare_route => route(reader, p),
    };
}
