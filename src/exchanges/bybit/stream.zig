const std = @import("std");
const core = @import("../../core/exchange_stream.zig");
const w = @import("../../core/wire.zig");
const State = core.State;
const eq = std.mem.eql;
const in = core.in;
const same = core.same;
const optionalText = core.optionalText;
const requiredText = core.requiredText;
const optionalDepth = core.optionalDepth;
const stringify = core.stringify;

pub fn init(s: *State, a: w.A, _: w.V, params: w.V, channel: []const u8, t: ?[]const u8, override: ?[]const u8, testnet: bool) !void {
    s.private = in(channel, &.{ "orders", "order", "fills", "execution", "position", "wallet" });
    const base = if (testnet) "wss://stream-testnet.bybit.com" else "wss://stream.bybit.com";
    const category = try optionalText(params, "category");
    if (category) |c| if (!in(c, &.{ "spot", "linear", "inverse", "option" })) return error.InvalidSubscription;
    s.endpoint = override orelse if (s.private) try std.mem.concat(a, u8, &.{ base, "/v5/private" }) else try std.mem.concat(a, u8, &.{ base, "/v5/public/", category orelse return error.InvalidSubscription });
    s.topic = if (s.private) topic: {
        const family = if (eq(u8, channel, "orders")) "order" else if (eq(u8, channel, "fills")) "execution" else channel;
        break :topic if (category) |c| try std.fmt.allocPrint(a, "{s}.{s}", .{ family, c }) else family;
    } else topic: {
        const symbol = try requiredText(params, "symbol");
        if (in(channel, &.{ "book", "order_book", "orderbook" })) break :topic try std.fmt.allocPrint(a, "orderbook.{d}.{s}", .{ try optionalDepth(params, if (category != null and eq(u8, category.?, "option")) 25 else 50), symbol });
        break :topic try std.fmt.allocPrint(a, "{s}.{s}", .{ if (eq(u8, channel, "trades")) "publicTrade" else if (eq(u8, channel, "ticker")) "tickers" else channel, symbol });
    };
    s.topic = t orelse s.topic;
    s.book = std.mem.startsWith(u8, s.topic, "orderbook.");
    s.frame = try stringify(a, .{ .op = "subscribe", .args = .{s.topic} });
}

pub fn event(s: *State, _: w.A, frame: w.V) !core.Event {
    if (same(frame, "topic", s.topic)) return .deliver;
    return .skip;
}
