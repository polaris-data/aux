const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const Venue = @import("../../core/routes.zig").Venue;
const trading = @import("trading.zig");

pub fn dex(venue: Venue) ?[]const u8 {
    return switch (venue) {
        .paragon => "para",
        .trade_xyz => "xyz",
        else => null,
    };
}

pub fn own(name: []const u8, coin: []const u8) bool {
    return coin.len > name.len + 1 and std.mem.startsWith(u8, coin, name) and coin[name.len] == ':';
}

fn validateCoin(name: []const u8, value: p.V) !void {
    if (!own(name, try w.string(value))) return error.InvalidMarket;
}

pub fn request(a: p.A, name: []const u8, op: []const u8, params: p.V) !p.V {
    var body = p.object();
    if (p.eq(u8, op, "place_order") or p.eq(u8, op, "cancel_order")) {
        const coin = try w.field(params, "coin");
        try validateCoin(name, coin);
        const order = try w.field(params, "body");
        if (order != .object or order.object.contains("a") or order.object.contains("asset")) return error.InvalidParameter;
        try p.put(&body, a, "coin", coin);
        try p.put(&body, a, "body", order);
        return body;
    }
    if (p.eq(u8, op, "order_book") or p.eq(u8, op, "trades")) {
        const coin = try w.field(params, "coin");
        try validateCoin(name, coin);
        try p.put(&body, a, "type", w.text(if (p.eq(u8, op, "trades")) "recentTrades" else "l2Book"));
        try p.put(&body, a, "coin", coin);
        return body;
    }
    if (p.eq(u8, op, "markets") or p.eq(u8, op, "perp_dexs")) {
        const meta = p.eq(u8, op, "markets");
        try p.put(&body, a, "type", w.text(if (meta) "meta" else "perpDexs"));
        if (meta) try p.put(&body, a, "dex", w.text(name));
        return body;
    }
    const user = try w.field(params, "user");
    _ = try evm.address(try w.string(user));
    if (p.eq(u8, op, "order_status")) {
        try validateCoin(name, try w.field(params, "coin"));
        const oid = try w.field(params, "oid");
        if (oid == .string and std.mem.startsWith(u8, oid.string, "0x")) {
            _ = try evm.hex(16, oid.string);
        } else _ = try w.unsigned(u64, oid);
        try p.put(&body, a, "type", w.text("orderStatus"));
        try p.put(&body, a, "oid", oid);
    } else {
        try p.put(&body, a, "type", w.text(if (p.eq(u8, op, "open_orders")) "openOrders" else "clearinghouseState"));
        try p.put(&body, a, "dex", w.text(name));
    }
    try p.put(&body, a, "user", user);
    return body;
}

fn assetId(name: []const u8, coin: []const u8, dexs: p.V, meta: p.V) !u64 {
    if (dexs != .array or !own(name, coin)) return error.InvalidMarket;
    const universe = try w.field(meta, "universe");
    if (universe != .array or universe.array.items.len > 10_000) return error.InvalidResponse;
    var index: ?usize = null;
    for (dexs.array.items, 0..) |dex_record, i| {
        const value = p.get(dex_record, "name");
        if (value == .string and p.eq(u8, name, value.string)) {
            if (i == 0 or index != null) return error.InvalidResponse;
            index = i;
        }
    }
    const dex_index = index orelse return error.InvalidMarket;
    for (universe.array.items, 0..) |market, i| {
        if (p.eq(u8, try p.text(market, "name"), coin)) return 100_000 + @as(u64, dex_index) * 10_000 + @as(u64, i);
    }
    return error.InvalidMarket;
}

pub fn action(a: p.A, name: []const u8, operation: []const u8, params: p.V, metadata: p.V) !p.V {
    const coin = try p.text(params, "coin");
    const asset = try assetId(name, coin, try w.field(metadata, "perp_dexs"), try w.field(metadata, "markets"));
    var item = p.object();
    const source = try w.field(params, "body");
    const place = p.eq(u8, operation, "place_order");
    const by_cloid = !place and p.get(source, "cloid") != .null;
    if (place and p.get(p.get(source, "t"), "limit") == .null) return error.InvalidParameter;
    if (!place and ((p.get(source, "o") != .null) == by_cloid)) return error.InvalidParameter;
    try p.put(&item, a, if (by_cloid) "asset" else "a", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{asset}) });
    for (source.object.keys(), source.object.values()) |key, value| try p.put(&item, a, key, value);
    var body = p.object();
    try p.put(&body, a, if (place) "orders" else "cancels", try p.array(a, &.{item}));
    return trading.action(a, if (place) "order" else if (by_cloid) "cancel_by_cloid" else "cancel", body);
}

pub fn validateOrderStatus(name: []const u8, requested_coin: []const u8, result: p.V) !void {
    if (p.get(result, "order") == .null) return;
    const order = try w.field(try w.field(result, "order"), "order");
    const coin = try p.text(order, "coin");
    if (!own(name, coin) or !p.eq(u8, coin, requested_coin)) return error.InvalidMarket;
}
