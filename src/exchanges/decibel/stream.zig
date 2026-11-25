const std = @import("std");
const w = @import("../../core/wire.zig");
const p = @import("../../core/perps.zig");
const connectors = @import("../../core/perps_connectors.zig");
const shared = @import("../../core/perps_stream.zig");
const Event = shared.Event;
const eq = std.mem.eql;
const gpa = std.heap.c_allocator;

pub fn init(self: *shared.State, a: p.A, testnet: bool, params: p.V) !void {
    const channel = self.channel;
    const index = try connectors.enumIndex(channel, &.{ "depth", "trades", "account_overview", "account_positions", "account_open_orders", "user_trades", "order_updates" });
    const field = if (index < 2) "market" else "account";
    const id = try p.text(params, field);
    _ = try @import("trading.zig").address(id);
    if (index < 2) self.market = id;
    self.endpoint = if (testnet) "wss://api.testnet.aptoslabs.com/decibel/ws" else "wss://api.mainnet.aptoslabs.com/decibel/ws";
    self.selector = try std.fmt.allocPrint(a, "{s}:{s}", .{ channel, id });
    if (index == 0) {
        const level = try shared.optional(u16, params, "aggregation_level", 1);
        switch (level) {
            1, 2, 5, 10, 100, 1000 => {},
            else => return error.InvalidSubscription,
        }
        self.selector = try std.fmt.allocPrint(a, "{s}:{d}", .{ self.selector, level });
    }
    self.request = try std.json.Stringify.valueAlloc(a, .{ .method = "subscribe", .topic = self.selector }, .{});
}

pub fn event(self: *shared.State, message: p.V, raw: []const u8) !Event {
    if (!shared.same(message, "topic", self.selector)) return .skip;
    const success = p.get(message, "success");
    if (success == .bool) return if (success.bool) .skip else error.ExchangeApi;
    if (self.market) |expected| {
        const market = p.get(message, "market");
        if (market != .null and !try matchesMarket(market, expected)) return .skip;
        const trades = p.get(message, "trades");
        if (trades == .array) for (trades.array.items) |row| {
            if (!try matchesMarket(p.get(row, "market"), expected)) return .skip;
        };
    }
    return .{ .deliver = raw };
}

fn matchesMarket(value: p.V, expected: []const u8) !bool {
    const address = @import("trading.zig").address;
    if (value != .string) return false;
    return std.mem.eql(u8, &try address(value.string), &try address(expected));
}
