const std = @import("std");
const wire = @import("../../core/wire.zig");
const exchange = @import("../../core/exchange_stream.zig");
const socket = @import("../../core/websocket.zig");
const perps = @import("../../core/perps.zig");
const endpoint = "wss://api.gatedex.com/v1/ws/dex";

pub fn prepare(a: wire.A, override: ?[]const u8, spec: wire.V, items: []const wire.V) !socket.PreparedSubscription {
    const timestamp = perps.get(spec, "timestamp_s");
    var clock: std.Io.Threaded = .init_single_threaded;
    const seconds = if (timestamp == .null) (try perps.epoch(clock.io(), .ms)) / 1000 else try wire.unsigned(u64, timestamp);
    const subscribe = try a.alloc([]const u8, items.len);
    const unsubscribe = try a.alloc([]const u8, items.len);
    for (items, subscribe, unsubscribe) |item, *sub, *unsub| {
        const channel = try exchange.requiredText(item, "channel");
        const symbol = try exchange.requiredText(item, "symbol");
        for (symbol) |c| if (!std.ascii.isAlphanumeric(c) and c != '_') return error.InvalidSymbol;
        const depth = perps.get(item, "depth");
        var payload = [_][]const u8{ symbol, "", "0" };
        var payload_len: usize = 1;
        if (std.mem.eql(u8, channel, "futures.order_book")) {
            const levels = if (depth == .null) 100 else try wire.unsigned(u8, depth);
            switch (levels) {
                1, 5, 10, 20, 50, 100 => {},
                else => return error.InvalidDepth,
            }
            payload[1] = try std.fmt.allocPrint(a, "{d}", .{levels});
            payload_len = payload.len;
        } else if (!std.mem.eql(u8, channel, "futures.trades") or depth != .null) return error.InvalidSubscription;
        sub.* = try frame(a, seconds, channel, "subscribe", payload[0..payload_len]);
        unsub.* = try frame(a, seconds, channel, "unsubscribe", payload[0..payload_len]);
    }
    return .{ .endpoint = override orelse endpoint, .subscribe = subscribe, .unsubscribe = unsubscribe };
}

fn frame(a: wire.A, seconds: u64, channel: []const u8, action: []const u8, payload: []const []const u8) ![]const u8 {
    return exchange.stringify(a, .{ .time = seconds, .channel = channel, .event = action, .payload = payload });
}
