const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const common = @import("../../core/exchange_http.zig");
const evm = @import("../../core/evm.zig");
const eq = std.mem.eql;
const A = w.A;
const V = w.V;
const Bytes = std.ArrayList(u8);
var nonce_counter: std.atomic.Value(u64) = .init(0);

fn freshNonce(io: std.Io) !u64 {
    const now = try p.epoch(io, .us);
    var old = nonce_counter.load(.monotonic);
    while (true) {
        const next = @max(now, old + 1);
        old = nonce_counter.cmpxchgWeak(old, next, .monotonic, .monotonic) orelse return next;
    }
}

fn number(value: V, field: []const u8) !u64 {
    return w.unsigned(u64, try w.field(value, field));
}

fn append(a: A, out: *Bytes, comptime T: type, value: T) !void {
    var bytes: [@sizeOf(T)]u8 = undefined;
    std.mem.writeInt(T, &bytes, value, .big);
    try out.appendSlice(a, &bytes);
}

fn scaled(raw: V, exponent: i16, multiplier: u64) !u64 {
    const text = try @import("../../core/http.zig").scalar(raw);
    const dot = std.mem.indexOfScalar(u8, text, '.') orelse text.len;
    const precision = if (dot == text.len) 0 else text.len - dot - 1;
    if (precision > 38) return error.InvalidQuantity;
    const units = try @import("../../core/perps_connectors.zig").decimal(u256, text, @intCast(precision));
    const shift = exponent - @as(i16, @intCast(precision));
    if (shift > 76 or shift < -76) return error.InvalidQuantity;
    const power = try std.math.powi(u256, 10, @abs(shift));
    const product = try std.math.mul(u256, units, multiplier);
    const result = if (shift < 0) product / power else try std.math.mul(u256, product, power);
    return std.math.cast(u64, result) orelse error.InvalidQuantity;
}

fn contract(meta: V, symbol: []const u8) !V {
    const contracts = try w.field(meta, "futureContracts");
    if (contracts != .array) return error.InvalidResponse;
    for (contracts.array.items) |item| {
        if (eq(u8, try p.text(item, "symbol"), symbol)) return item;
    }
    return error.InvalidMarket;
}

fn choose(body: V, key: []const u8, alt: []const u8) V {
    const value = p.get(body, key);
    return if (value == .null) p.get(body, alt) else value;
}

fn signPayload(a: A, input: V, payload: []const u8) ![]const u8 {
    const mode = p.get(p.get(input, "auth"), "signing");
    const key = try common.secret(input);
    if (mode == .null or eq(u8, try w.string(mode), "hmac")) return common.hmac(a, key, payload, false);
    if (!eq(u8, try w.string(mode), "ecdsa")) return error.InvalidSigningMode;
    var private_key = try evm.hex(32, key);
    defer std.crypto.secureZero(u8, &private_key);
    var hash: [32]u8 = undefined;
    std.crypto.hash.sha2.Sha256.hash(payload, &hash, .{});
    var signature = try @import("../../core/transaction.zig").recoverable(private_key, hash);
    signature[64] -= 27;
    return std.fmt.allocPrint(a, "{x}", .{&signature});
}

fn account(a: A, input: V, body: *V) !void {
    const configured = p.get(p.get(input, "auth"), "account_id");
    const provided = p.get(body.*, "accountId");
    if (provided != .null and configured != .null and try w.unsigned(u64, provided) != try w.unsigned(u64, configured))
        return error.InvalidCredential;
    if (provided == .null) {
        _ = try w.unsigned(u64, configured);
        try p.put(body, a, "accountId", configured);
    }
}

fn order(a: A, input: V, op: []const u8, body: *V, meta: V, io: std.Io) !void {
    var payload: Bytes = .empty;
    if (eq(u8, op, "cancel_order")) {
        const id = p.get(body.*, "orderId");
        const nonce = p.get(body.*, "nonce");
        if ((id == .null) == (nonce == .null)) return error.InvalidParameter;
        try append(a, &payload, u64, try w.unsigned(u64, if (id == .null) nonce else id));
    } else {
        const value = p.get(body.*, "nonce");
        const nonce = if (value == .null) try freshNonce(io) else try w.unsigned(u64, value);
        try p.put(body, a, "nonce", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{nonce}) });
        try append(a, &payload, u64, nonce);
        if (!eq(u8, op, "cancel_all_orders")) {
            const c = try contract(meta, try p.text(body.*, "symbol"));
            const underlying = try w.unsigned(u8, try w.field(c, "underlyingDecimals"));
            const settlement = try w.unsigned(u8, try w.field(c, "settlementDecimals"));
            const quantity = try scaled(choose(body.*, "updatedQuantity", "quantity"), underlying, 1);
            if (quantity == 0) return error.InvalidQuantity;
            const side = try p.text(body.*, "side");
            const bid = eq(u8, side, "BID");
            if (!bid and !eq(u8, side, "ASK")) return error.InvalidParameter;
            try append(a, &payload, u32, try w.unsigned(u32, try w.field(c, "id")));
            try append(a, &payload, u64, quantity);
            try append(a, &payload, u32, @intFromBool(bid));
            const price = choose(body.*, "updatedPrice", "price");
            if (price != .null) try append(a, &payload, u64, try scaled(price, @as(i16, settlement) - underlying, 1 << 32));
            try append(a, &payload, u64, try scaled(try w.field(body.*, "maxFeesPercent"), 8, 1));
        }
    }
    try common.set(a, body, "signature", try signPayload(a, input, payload.items));
}

pub fn sign(a: A, input: V, op: []const u8, source: V, meta: V, io: std.Io) !V {
    var body = source;
    try account(a, input, &body);
    if (eq(u8, op, "place_orders")) {
        const items = try w.field(body, "orders");
        if (items != .array or items.array.items.len == 0) return error.InvalidParameter;
        for (items.array.items) |*item| {
            const action = try p.text(item.*, "action");
            const operation = if (eq(u8, action, "place")) "place_order" else if (eq(u8, action, "modify")) "amend_order" else if (eq(u8, action, "cancel")) "cancel_order" else return error.InvalidParameter;
            try order(a, input, operation, item, meta, io);
        }
    } else {
        try order(a, input, op, &body, meta, io);
    }
    return body;
}
