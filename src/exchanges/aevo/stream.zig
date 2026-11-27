const std = @import("std");
const core = @import("../../core/exchange_stream.zig");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const State = core.State;
const eq = std.mem.eql;
const in = core.in;
const same = core.same;
const optionalText = core.optionalText;
const requiredText = core.requiredText;
const stringify = core.stringify;

pub fn init(s: *State, a: w.A, _: w.V, params: w.V, channel: []const u8, t: ?[]const u8, override: ?[]const u8, testnet: bool) !void {
    s.endpoint = override orelse if (testnet) "wss://ws-testnet.aevo.xyz" else "wss://ws.aevo.xyz";
    s.private = in(t orelse channel, &.{ "orders", "fills", "positions" });
    const is_book = in(channel, &.{ "book", "order_book", "orderbook" });
    const interval = (try optionalText(params, "interval")) orelse "100ms";
    if (is_book and !in(interval, &.{ "100ms", "500ms" })) return error.InvalidSubscription;
    const native = if (is_book) try std.fmt.allocPrint(a, "orderbook-{s}", .{interval}) else channel;
    s.topic = t orelse if (s.private) channel else if (eq(u8, channel, "ticker")) topic: {
        if (try optionalText(params, "symbol")) |symbol| break :topic try std.fmt.allocPrint(a, "ticker-500ms:{s}", .{symbol});
        break :topic try std.fmt.allocPrint(a, "ticker-500ms:{s}:{s}", .{
            try requiredText(params, "asset"),
            (try optionalText(params, "instrument_type")) orelse "PERPETUAL",
        });
    } else try std.fmt.allocPrint(a, "{s}:{s}", .{ native, try requiredText(params, "symbol") });
    s.book = std.mem.startsWith(u8, s.topic, "orderbook-") or std.mem.startsWith(u8, s.topic, "orderbook:");
    s.frame = try subscriptionFrame(a, true, &.{s.topic});
}

pub fn subscriptionFrame(a: w.A, subscribe: bool, topics: []const []const u8) ![]const u8 {
    return stringify(a, .{ .id = @as(u64, if (subscribe) 1 else 2), .op = if (subscribe) "subscribe" else "unsubscribe", .data = topics });
}

pub fn event(s: *State, _: w.A, frame: w.V) !core.Event {
    if (same(frame, "channel", s.topic) and p.get(frame, "data") != .null) return .deliver;
    return .skip;
}
