const std = @import("std");
const w = @import("../../core/wire.zig");
const A = w.A;
const V = w.V;

pub const Frame = struct { method: []const u8, body: V };
pub fn get(v: V, k: []const u8) V {
    return if (v == .object) v.object.get(k) orelse .null else .null;
}
fn obj(a: A, fields: anytype) !V {
    var v = V{ .object = .empty };
    inline for (@typeInfo(@TypeOf(fields)).@"struct".field_names) |field| try v.object.put(a, field, @field(fields, field));
    return v;
}
pub fn perps(a: A, op: []const u8, params: V, config: V, io: std.Io) !Frame {
    const commands = .{ .create_orders = "createOrders", .modify_orders = "modifyOrders", .modify_orders_coid = "modifyOrdersCOID", .cancel_orders = "cancelOrders", .cancel_orders_coid = "cancelOrdersCOID", .cancel_all = "cancelAll", .auto_cancel = "autoCancel", .update_leverage = "updateLeverage", .update_leverages = "updateLeverages", .update_margin = "updateMargin" };
    inline for (@typeInfo(@TypeOf(commands)).@"struct".field_names) |field| {
        if (std.mem.eql(u8, op, field)) {
            const name = @field(commands, field);
            var body = try obj(a, .{ .req = w.text("post"), .op = try obj(a, .{ .type = w.text(name), .args = params }) });
            try @import("perps_sign.zig").sign(a, &body, try w.field(config, "auth"), name, @intCast(std.Io.Timestamp.now(io, .real).toMilliseconds()), io);
            return .{ .method = name, .body = body };
        }
    }
    const kind = std.meta.stringToEnum(enum { ping, subscribe, unsubscribe }, op) orelse return error.InvalidOperation;
    if (kind == .ping) return .{ .method = "ping", .body = try obj(a, .{ .req = w.text("post"), .op = try obj(a, .{ .type = w.text("ping") }) }) };
    if (params != .array) return error.ExpectedArray;
    for (params.array.items) |v| _ = try w.string(v);
    return .{ .method = @tagName(kind), .body = try obj(a, .{ .req = w.text(if (kind == .subscribe) "sub" else "unsub"), .chs = params }) };
}
pub fn rfq(a: A, op: []const u8, payload: V, config: V) !Frame {
    const kind = std.meta.stringToEnum(enum { quote, cancel_quote, last_look }, op) orelse return error.InvalidOperation;
    const name: []const u8 = switch (kind) {
        .quote => "RFQ_QUOTE",
        .cancel_quote => "RFQ_QUOTE_CANCEL",
        .last_look => "RFQ_CONFIRMATION_RESPONSE",
    };
    var body = payload;
    if ((try w.string(try w.field(body, "rfq_id"))).len == 0) return error.InvalidRequest;
    if (get(body, "quote_id") != .null) _ = try w.string(get(body, "quote_id"));
    if (kind == .quote) {
        var order = try obj(a, .{ .order = try w.field(body, "signed_order") });
        try @import("clob_wallet.zig").orders(a, &order, get(config, "auth"), try w.field(config, "clob"), false);
        try body.object.put(a, "signed_order", try w.field(order, "order"));
    }
    try body.object.put(a, "type", w.text(name));
    return .{ .method = name, .body = body };
}
fn same(v: V, s: []const u8) bool {
    return v == .string and std.mem.eql(u8, v.string, s);
}
pub fn matches(v: V, method: []const u8, correlation: ?[]const u8, quote: ?[]const u8) bool {
    const kind = get(v, "type");
    if (std.mem.eql(u8, method, "auth")) return same(kind, "auth");
    if (!same(get(v, "rfq_id"), correlation orelse return false)) return false;
    if (same(kind, "RFQ_ERROR")) return same(get(v, "request_type"), method);
    if (kind != .string or !std.mem.startsWith(u8, kind.string, "ACK_") or !std.mem.eql(u8, kind.string[4..], method)) return false;
    return if (quote) |q| same(get(v, "quote_id"), q) else true;
}
