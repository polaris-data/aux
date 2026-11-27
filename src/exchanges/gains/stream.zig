const std = @import("std");
const wire = @import("../../core/wire.zig");
const exchange = @import("../../core/exchange_stream.zig");
const socket = @import("../../core/websocket.zig");
const p = @import("../../core/perps.zig");
const endpoint = "wss://api.gains.trade/v1/arbitrum/ws";
const eq = std.mem.eql;

pub const Route = enum { prices, book, trades, websocket_control };
pub const Frame = struct { route: Route, reply: ?[]const u8 = null };

/// Classify the parsed frame while the caller retains the original bytes.
pub fn classifyFrame(value: wire.V) Frame {
    if (value == .object and value.object.contains("op")) {
        return .{ .route = .websocket_control, .reply = if (exchange.same(value, "op", "ping")) "{\"op\":\"pong\"}" else null };
    }
    const channel = p.get(value, "channel");
    return .{ .route = if (channel == .string) std.meta.stringToEnum(Route, channel.string) orelse .websocket_control else .websocket_control };
}

pub fn prepare(a: wire.A, override: ?[]const u8, items: []const wire.V) !socket.PreparedSubscription {
    if (items.len > 200) return error.TooManySubscriptions;
    const subscribe = try a.alloc([]const u8, items.len);
    const unsubscribe = try a.alloc([]const u8, items.len);
    var seen_prices = false;
    for (items, subscribe, unsubscribe) |item, *sub, *unsub| {
        const channel = try exchange.requiredText(item, "channel");
        const market = if (eq(u8, channel, "prices")) blk: {
            if (seen_prices or (try exchange.optionalText(item, "market")) != null) return error.InvalidSubscription;
            seen_prices = true;
            break :blk null;
        } else if (exchange.in(channel, &.{ "book", "trades" })) blk: {
            const value = try exchange.requiredText(item, "market");
            const divider = std.mem.indexOfScalar(u8, value, '/') orelse return error.InvalidMarket;
            if (divider == 0 or divider + 1 == value.len or std.mem.indexOfScalar(u8, value[divider + 1 ..], '/') != null) return error.InvalidMarket;
            for (value) |c| if (c <= 32 or c == 127) return error.InvalidMarket;
            break :blk value;
        } else return error.InvalidSubscription;
        sub.* = try frame(a, "subscribe", channel, market);
        unsub.* = try frame(a, "unsubscribe", channel, market);
    }
    return .{ .endpoint = override orelse endpoint, .subscribe = subscribe, .unsubscribe = unsubscribe };
}

fn frame(a: wire.A, op: []const u8, channel: []const u8, market: ?[]const u8) ![]const u8 {
    return exchange.stringify(a, .{ .op = op, .channel = channel, .market = market });
}
