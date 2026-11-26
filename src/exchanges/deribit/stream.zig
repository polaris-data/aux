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
const optionalDepth = core.optionalDepth;
const stringify = core.stringify;

pub fn init(s: *State, a: w.A, _: w.V, params: w.V, channel: []const u8, t: ?[]const u8, override: ?[]const u8, testnet: bool) !void {
    s.endpoint = override orelse if (testnet) "wss://test.deribit.com/ws/api/v2" else "wss://www.deribit.com/ws/api/v2";
    const name = try optionalText(params, "instrument_name");
    const interval = (try optionalText(params, "interval")) orelse "100ms";
    const family = if (eq(u8, channel, "orders")) "user.orders" else if (eq(u8, channel, "fills")) "user.trades" else if (eq(u8, channel, "order_book")) "book" else channel;
    s.topic = t orelse if (in(family, &.{ "book", "ticker", "trades", "user.orders", "user.trades", "user.changes" })) topic: {
        const instrument = name orelse break :topic try std.fmt.allocPrint(a, "{s}.{s}.{s}.{s}", .{ family, (try optionalText(params, "kind")) orelse "any", try requiredText(params, "currency"), interval });
        const group = try optionalText(params, "group");
        if (group != null and eq(u8, family, "book")) break :topic try std.fmt.allocPrint(a, "book.{s}.{s}.{d}.{s}", .{ instrument, group.?, try optionalDepth(params, 10), interval });
        break :topic try std.fmt.allocPrint(a, "{s}.{s}.{s}", .{ family, instrument, interval });
    } else channel;
    s.private = std.mem.startsWith(u8, s.topic, "user.") or std.mem.endsWith(u8, s.topic, ".raw");
    s.book = std.mem.startsWith(u8, s.topic, "book.");
    s.frame = try stringify(a, .{ .jsonrpc = "2.0", .id = 1, .method = if (s.private) "private/subscribe" else "public/subscribe", .params = .{ .channels = .{s.topic} } });
}

pub fn event(s: *State, _: w.A, frame: w.V) !core.Event {
    if (same(frame, "method", "subscription") and same(p.get(frame, "params"), "channel", s.topic)) return .deliver;
    return .skip;
}
