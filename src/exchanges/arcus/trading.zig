const std = @import("std");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const A = w.A;
const V = w.V;
const eq = std.mem.eql;
var clock: std.atomic.Value(u64) = .init(0);

fn text(v: V, name: []const u8) ![]const u8 {
    return w.string(try w.field(v, name));
}
fn number(a: A, n: u64) !V {
    return .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{n}) };
}
fn integer(comptime T: type, v: V) !T {
    if (v != .number_string) return error.InvalidInteger;
    return w.unsigned(T, v);
}
fn timestamp(io: std.Io) !u64 {
    const now = std.math.cast(u64, std.Io.Timestamp.now(io, .real).toNanoseconds()) orelse return error.InvalidTimestamp;
    var old = clock.load(.monotonic);
    while (true) {
        const next = @max(try std.math.add(u64, old, 1), now);
        old = clock.cmpxchgWeak(old, next, .monotonic, .monotonic) orelse return next;
    }
}
const Decimal = struct { n: u128, scale: usize };
fn decimal(s: []const u8) !Decimal {
    if (s.len == 0 or s[0] == '.' or s[s.len - 1] == '.') return error.InvalidDecimal;
    var n: u128 = 0;
    var dot: ?usize = null;
    for (s, 0..) |c, i| {
        if (c == '.' and dot == null) {
            dot = i;
        } else {
            if (!std.ascii.isDigit(c)) return error.InvalidDecimal;
            n = try std.math.add(u128, try std.math.mul(u128, n, 10), c - '0');
        }
    }
    if (n == 0) return error.InvalidDecimal;
    return .{ .n = n, .scale = if (dot) |i| s.len - i - 1 else 0 };
}
fn units(s: []const u8, increment: []const u8) !u64 {
    var value = try decimal(s);
    var step = try decimal(increment);
    const scale = @max(value.scale, step.scale);
    for (value.scale..scale) |_| value.n = try std.math.mul(u128, value.n, 10);
    for (step.scale..scale) |_| step.n = try std.math.mul(u128, step.n, 10);
    if (value.n % step.n != 0 or value.n / step.n > std.math.maxInt(i64)) return error.InvalidIncrement;
    return @intCast(value.n / step.n);
}
fn choice(s: []const u8, values: []const []const u8) !u64 {
    for (values, 0..) |v, i| if (eq(u8, v, s)) return i;
    return error.InvalidOrder;
}
const Order = struct { body: V, market_id: V, ticks: u64 = 0, quantums: u64 = 0, expiry: u64 = 0, side: u64 = 0, tif: u64 = 0, reduce: bool = false };
pub fn order(a: A, place: bool, params: V) !Order {
    const source = if (place) try w.field(params, "body") else params;
    if (place and params.object.count() != 2) return error.InvalidOrder;
    var body: V = .{ .object = .empty };
    const market_id = try w.field(source, "marketId");
    _ = try integer(u16, market_id);
    var o: Order = .{ .body = undefined, .market_id = market_id };
    var it = source.object.iterator();
    const keys: []const []const u8 = if (place) &.{ "marketId", "orderSide", "orderType", "quantity", "price", "timeInForce", "goodTilTime", "clientId", "reduceOnly" } else &.{ "marketId", "orderId", "clientId" };
    while (it.next()) |f| {
        _ = try choice(f.key_ptr.*, keys);
        if (eq(u8, f.key_ptr.*, "clientId") or eq(u8, f.key_ptr.*, "orderId")) {
            if (f.value_ptr.* == .null) continue;
            _ = try w.string(f.value_ptr.*);
        }
        try body.object.put(a, f.key_ptr.*, f.value_ptr.*);
    }
    if (place) {
        const market = try w.field(params, "market");
        if (try integer(u16, try w.field(market, "marketId")) != try integer(u16, market_id)) return error.InvalidMarket;
        o.ticks = try units(try text(body, "price"), try text(market, "tickSize"));
        o.quantums = try units(try text(body, "quantity"), try text(market, "stepSize"));
        o.side = try choice(try text(body, "orderSide"), &.{ "BUY", "SELL" });
        o.tif = try choice(try text(body, "timeInForce"), &.{ "GTT", "FOK", "IOC", "ALO" });
        _ = try choice(try text(body, "orderType"), &.{ "LIMIT", "MARKET" });
        o.expiry = try std.math.mul(u64, try w.unsigned(u64, w.text(try text(body, "goodTilTime"))), 1000);
        const reduce = body.object.get("reduceOnly") orelse V{ .bool = false };
        if (reduce != .bool) return error.InvalidOrder;
        o.reduce = reduce.bool;
        try body.object.put(a, "reduceOnly", reduce);
    } else if ((id(body, "clientId") == null) == (id(body, "orderId") == null)) return error.InvalidOrder;
    o.body = body;
    return o;
}
fn id(body: V, name: []const u8) ?V {
    const v = body.object.get(name) orelse return null;
    return if (v.string.len == 0) null else v;
}
fn payload(a: A, cfg: V, place: bool, o: Order, ts: u64) !V {
    const s = try text(cfg, "address");
    const hex = if (std.ascii.startsWithIgnoreCase(s, "0x")) s[2..] else s;
    const address = w.text(try std.fmt.allocPrint(a, "0x{x}", .{try evm.hex(20, hex)}));
    const account = try w.field(cfg, "account_index");
    if (try integer(u8, account) > 9) return error.InvalidAccount;
    var p: V = .{ .object = .empty };
    try p.object.put(a, "ad", address);
    try p.object.put(a, "ai", account);
    if (id(o.body, "clientId")) |c| try p.object.put(a, "c", c);
    try p.object.put(a, "ct", try number(a, ts));
    if (place) {
        if (o.expiry <= ts) return error.InvalidExpiry;
        try p.object.put(a, "g", try number(a, o.expiry));
    } else if (id(o.body, "orderId")) |v| try p.object.put(a, "id", v);
    try p.object.put(a, "m", o.market_id);
    try p.object.put(a, "op", try number(a, if (place) 1 else 2));
    if (place) {
        try p.object.put(a, "p", try number(a, o.ticks));
        try p.object.put(a, "q", try number(a, o.quantums));
        try p.object.put(a, "r", try number(a, @intFromBool(o.reduce)));
        try p.object.put(a, "s", try number(a, o.side));
        try p.object.put(a, "t", try number(a, o.tif));
    }
    try p.object.put(a, "v", try number(a, 1));
    return p;
}
pub fn signing(a: A, input: V, io: std.Io, request: ?*@import("../../core/http.zig").Request) !V {
    const cfg = input.object.get("auth") orelse return error.AuthenticationRequired;
    const op = try choice(try text(input, "operation"), &.{ "place_order", "cancel_order" });
    const placed = op == 0;
    var o = try order(a, placed, try w.field(input, "params"));
    // Caller-owned signers return the timestamp allocated with their payload.
    const ts = if (request != null and cfg.object.get("seed") == null) try integer(u64, try w.field(cfg, "timestamp")) else try timestamp(io);
    const p = try payload(a, cfg, placed, o, ts);
    const bytes = try std.json.Stringify.valueAlloc(a, p, .{});
    var output: V = .{ .object = .empty };
    try output.object.put(a, "payload", w.text(bytes));
    try output.object.put(a, "timestamp", try w.field(p, "ct"));
    const req = request orelse return output;
    const signed = if (cfg.object.get("seed")) |seed| blk: {
        try output.object.put(a, "seed", seed);
        break :blk try @import("../../core/auth.zig").arcus(a, output);
    } else cfg;
    const public = try evm.hex(32, try text(signed, "public_key"));
    const sig = try evm.hex(64, try text(signed, "signature"));
    const Ed = std.crypto.sign.Ed25519;
    try Ed.Signature.fromBytes(sig).verify(bytes, try Ed.PublicKey.fromBytes(public));
    try req.headers.object.put(a, "X-API-Key", w.text(try std.fmt.allocPrint(a, "{x}", .{&public})));
    try req.headers.object.put(a, "X-Timestamp", w.text((try w.field(p, "ct")).number_string));
    try req.headers.object.put(a, "X-Signature", w.text(try std.fmt.allocPrint(a, "{x}", .{&sig})));
    try o.body.object.put(a, "address", try w.field(p, "ad"));
    try o.body.object.put(a, "accountIndex", try w.field(p, "ai"));
    try o.body.object.put(a, "timestamp", try w.field(p, "ct"));
    req.body = o.body;
    return .null;
}
