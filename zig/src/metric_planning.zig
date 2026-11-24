// Pure snapshot planning. All products use the same 512-bit intermediate
// and explicit rounding as the on-chain share calculations.
const std = @import("std");
const w = @import("wire.zig");
const math = @import("metric_math.zig");
const quantity = @import("transaction.zig").quantity;
const A = w.A;
const V = w.V;
const wad = 1_000_000_000_000_000_000;
const max104 = std.math.maxInt(u104);
const mulDiv = math.mulDiv;
const Map = std.AutoHashMapUnmanaged;
pub const Op = enum { addition_plan, weighted_plan, removal_plan };
fn val(o: V, k: []const u8) !u256 {
    return quantity(u256, try w.field(o, k));
}
fn small(v: V) !i16 {
    if (v != .number_string) return error.InvalidBin;
    return std.fmt.parseInt(i16, v.number_string, 10);
}
fn arr(o: V, k: []const u8) ![]const V {
    const v = try w.field(o, k);
    return if (v == .array) v.array.items else error.InvalidArray;
}
fn hex(a: A, n: u256) !V {
    return w.text(try std.fmt.allocPrint(a, "0x{x}", .{n}));
}
fn add(x: u256, y: u256) !u256 {
    return std.math.add(u256, x, y);
}
fn prod(xs: []const u256) !u512 {
    var p: u512 = 1;
    for (xs) |x| p = try std.math.mul(u512, p, x);
    return p;
}
fn ratio(x: []const u256, y: []const u256, r: math.Round) !u256 {
    return math.ratio(try prod(x), try prod(y), r);
}

const Bin = struct { balance: [2]u256, shares: u256 };
const Snapshot = struct {
    cur: i16,
    pos: u256,
    scale: [2]u256,
    initial: [2]u256,
    bins: Map(i16, Bin) = .empty,
    fn parse(a: A, v: V) !Snapshot {
        const low = try small(try w.field(v, "lowestBin"));
        const high = try small(try w.field(v, "highestBin"));
        _ = try w.string(try w.field(v, "block"));
        var s = Snapshot{ .cur = try small(try w.field(v, "curBinIdx")), .pos = try val(v, "curPosInBin"), .scale = .{ try val(v, "token0ScaleMultiplier"), try val(v, "token1ScaleMultiplier") }, .initial = .{ try val(v, "initialScaledToken0PerShareE18"), try val(v, "initialScaledToken1PerShareE18") } };
        if (s.cur < low or s.cur > high or s.pos > max104 or @min(s.scale[0], s.scale[1], s.initial[0], s.initial[1]) == 0) return error.InvalidSnapshot;
        for (try arr(v, "bins")) |row| {
            const i = try small(try w.field(row, "bin"));
            const b = Bin{ .shares = try val(row, "totalShares"), .balance = .{ try val(row, "token0BalanceScaled"), try val(row, "token1BalanceScaled") } };
            if (i < low or i > high or s.bins.contains(i) or @max(b.balance[0], b.balance[1]) > max104) return error.InvalidBin;
            try s.bins.put(a, i, b);
        }
        return s;
    }
};
const Deltas = struct {
    shares: Map(i16, u256) = .empty,
    fn encode(self: Deltas, a: A) !V {
        const keys = try a.alloc(i16, self.shares.count());
        var it = self.shares.keyIterator();
        for (keys) |*k| k.* = it.next().?.*;
        std.mem.sort(i16, keys, {}, std.sort.asc(i16));
        var idx: std.array_list.Managed(V) = .init(a);
        var values: std.array_list.Managed(V) = .init(a);
        for (keys) |i| {
            try idx.append(.{ .integer = i });
            try values.append(try hex(a, self.shares.get(i).?));
        }
        var o: V = .{ .object = .empty };
        try o.object.put(a, "binIdxs", .{ .array = idx });
        try o.object.put(a, "shares", .{ .array = values });
        return o;
    }
};
const Plan = struct {
    caps: [2]u256 = .{ 0, 0 },
    deltas: Deltas = .{},
    fn target(self: *Plan, a: A, s: Snapshot, i: i16, value: u256, in0: bool, price: u256) !void {
        const b = s.bins.get(i) orelse return error.UnknownBin;
        if (value == 0 or self.deltas.shares.contains(i)) return error.InvalidTarget;
        const values = [2]u256{ if (in0) value else try ratio(&.{ value, s.scale[1], wad }, &.{ s.scale[0], price }, .down), if (!in0) value else try ratio(&.{ value, s.scale[0], price }, &.{ s.scale[1], wad }, .down) };
        const t = [2]u256{ try std.math.mul(u256, values[0], s.scale[0]), try std.math.mul(u256, values[1], s.scale[1]) };
        const side: ?usize = if (i < s.cur or (i == s.cur and s.pos == max104)) 1 else if (i > s.cur or (i == s.cur and s.pos == 0)) 0 else null;
        const amount = if (side) |j| amount: {
            if (t[j] == 0) return error.InvalidTarget;
            break :amount if (b.shares != 0) try mulDiv(t[j], b.shares, b.balance[j], .up) else try mulDiv(t[j], wad, s.initial[j], .up);
        } else if (b.shares != 0) try mulDiv(t[1], b.shares, try add(b.balance[1], try mulDiv(b.balance[0], price, wad, .down)), .down) else amount: {
            const v0 = try ratio(&.{ s.initial[0], max104 - s.pos, price }, &.{wad}, .down);
            const v1 = try std.math.mul(u256, s.initial[1], s.pos);
            const total = try math.ratio(try add(v0, v1), max104, .down);
            break :amount try mulDiv(t[1], wad, total, .down);
        };
        const shares = @max(1, amount);
        if (shares > std.math.maxInt(i104)) return error.InvalidShares;
        for (0..2) |j| {
            const need = if (b.shares != 0) try mulDiv(shares, b.balance[j], b.shares, .up) else if (side) |k|
                if (k == j) try mulDiv(shares, s.initial[j], wad, .up) else 0
            else
                try ratio(&.{ shares, s.initial[j], if (j == 0) max104 - s.pos else s.pos }, &.{ wad, max104 }, .up);
            self.caps[j] = try add(self.caps[j], try math.ratio(need, s.scale[j], .up));
            if (self.caps[j] > std.math.maxInt(i128)) return error.InvalidCap;
        }
        try self.deltas.shares.put(a, i, shares);
    }
};
fn weighted(a: A, v: V, s: Snapshot, price: u256, plan: *Plan) !void {
    const ids = try arr(v, "bins");
    const weights = try arr(v, "weights");
    if (ids.len == 0 or ids.len != weights.len) return error.InvalidWeights;
    const caps = [2]u256{ try val(v, "maxAmountToken0"), try val(v, "maxAmountToken1") };
    var needed = [2]bool{ false, false };
    var minimum: u256 = std.math.maxInt(u256);
    for (ids, weights) |id, weight| {
        const i = try small(id);
        const n = try quantity(u256, weight);
        if (n == 0) return error.InvalidWeights;
        minimum = @min(minimum, n);
        needed[0] = needed[0] or i > s.cur or (i == s.cur and s.pos < max104);
        needed[1] = needed[1] or i < s.cur or (i == s.cur and s.pos > 0);
    }
    for (needed, caps) |need, cap| if (need and cap == 0) return error.InvalidCap;
    const unit = [2]u256{ try std.math.mul(u256, s.scale[0], price), try std.math.mul(u256, s.scale[1], wad) };
    const use0 = needed[0] and (!needed[1] or @as(u512, caps[0]) * unit[0] <= @as(u512, caps[1]) * unit[1]);
    const j: usize = if (use0) 0 else 1;
    const per = if (needed[1 - j]) try math.ratio(unit[1 - j], unit[j], .up) else 1;
    const units = try a.alloc(u256, ids.len);
    var total: u256 = 0;
    for (weights, units) |weight, *out| {
        out.* = try mulDiv(try quantity(u256, weight), 1000000, minimum, .up);
        total = try add(total, out.*);
    }
    const budget = @max(caps[j], try std.math.mul(u256, total, per));
    for (ids, units) |id, u| try plan.target(a, s, try small(id), try mulDiv(budget, u, total, .down), use0, price);
    plan.caps = caps;
}
fn removal(a: A, v: V) !V {
    var positions: Map(i16, u256) = .empty;
    for (try arr(v, "positions")) |p| try positions.put(a, try small(try w.field(p, "bin")), try val(p, "shares"));
    var seen: Map(i16, void) = .empty;
    var delta: Deltas = .{};
    for (try arr(v, "specs")) |spec| {
        const i = try small(try w.field(spec, "bin"));
        if (seen.contains(i)) return error.DuplicateBin;
        try seen.put(a, i, {});
        const shares = positions.get(i) orelse 0;
        const amount = spec.object.get("sharesToRemove") orelse .null;
        const pct = spec.object.get("bps") orelse .null;
        if ((amount == .null) == (pct == .null)) return error.InvalidRemoval;
        const result = if (amount != .null) @min(shares, try quantity(u256, amount)) else blk: {
            const bps = try quantity(u16, pct);
            if (bps == 0 or bps > 10000) return error.InvalidBps;
            break :blk try mulDiv(shares, bps, 10000, .down);
        };
        if (result != 0) try delta.shares.put(a, i, result);
    }
    return delta.encode(a);
}
pub fn execute(a: A, op: Op, v: V) !V {
    if (op == .removal_plan) return removal(a, v);
    const s = try Snapshot.parse(a, try w.field(v, "snapshot"));
    const price = try val(v, "priceE18");
    const salt = try val(v, "salt");
    if (price == 0 or salt > std.math.maxInt(u80)) return error.InvalidParameter;
    var plan: Plan = .{};
    if (op == .weighted_plan) try weighted(a, v, s, price, &plan) else {
        for (try arr(v, "targets")) |target| {
            const in0 = try w.field(target, "targetValueInToken0");
            if (in0 != .bool) return error.InvalidBool;
            try plan.target(a, s, try small(try w.field(target, "bin")), try val(target, "targetValueInToken"), in0.bool, price);
        }
        const bps: u64 = if (v.object.get("bufferBps")) |b| try quantity(u32, b) else 0;
        for (&plan.caps) |*cap| {
            cap.* = try mulDiv(cap.*, 10000 + bps, 10000, .down);
            if (cap.* > std.math.maxInt(i128)) return error.InvalidCap;
        }
    }
    var out: V = .{ .object = .empty };
    try out.object.put(a, "salt", try hex(a, salt));
    try out.object.put(a, "deltas", try plan.deltas.encode(a));
    try out.object.put(a, "maxAmountToken0", try hex(a, plan.caps[0]));
    try out.object.put(a, "maxAmountToken1", try hex(a, plan.caps[1]));
    return out;
}

pub fn validateSnapshot(a: A, value: V) !void {
    _ = try Snapshot.parse(a, value);
}
