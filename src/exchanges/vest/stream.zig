const std = @import("std");
const wire = @import("../../core/wire.zig");
const exchange = @import("../../core/exchange_stream.zig");
const socket = @import("../../core/websocket.zig");
const endpoint = "wss://ws-prod.hz.vestmarkets.com/ws";
const eq = std.mem.eql;

pub fn prepare(a: wire.A, override: ?[]const u8, items: []const wire.V) !socket.PreparedSubscription {
    const topics = try a.alloc([]const u8, items.len);
    for (items, topics) |item, *topic| {
        const channel = try exchange.requiredText(item, "channel");
        if (eq(u8, channel, "tickers")) {
            if ((try exchange.optionalText(item, "symbol")) != null) return error.InvalidSubscription;
            topic.* = channel;
            continue;
        }
        if (!exchange.in(channel, &.{ "depth", "kline_1m" })) return error.InvalidSubscription;
        const symbol = try exchange.requiredText(item, "symbol");
        for (symbol) |c| if (!std.ascii.isAlphanumeric(c) and c != '-' and c != '_') return error.InvalidSymbol;
        topic.* = try std.mem.concat(a, u8, &.{ symbol, "@", channel });
    }
    return .{
        .endpoint = override orelse endpoint,
        .subscribe = try a.dupe([]const u8, &.{try frame(a, "SUBSCRIBE", topics, 1)}),
        .unsubscribe = try a.dupe([]const u8, &.{try frame(a, "UNSUBSCRIBE", topics, 2)}),
    };
}

fn frame(a: wire.A, method: []const u8, topics: []const []const u8, id: u64) ![]const u8 {
    return exchange.stringify(a, .{ .method = method, .params = topics, .id = id });
}
