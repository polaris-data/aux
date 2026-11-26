const std = @import("std");
const core = @import("../../core/exchange_stream.zig");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const State = core.State;
const eq = std.mem.eql;
const in = core.in;
const same = core.same;
const optionalText = core.optionalText;
const stringify = core.stringify;

pub fn init(s: *State, a: w.A, _: w.V, params: w.V, channel: []const u8, t: ?[]const u8, override: ?[]const u8, testnet: bool) !void {
    if (testnet and override == null) return error.InvalidNetwork;
    s.endpoint = override orelse "wss://ws.backpack.exchange";
    s.symbol = try optionalText(params, "symbol");
    s.private = in(channel, &.{ "orders", "fills", "orderUpdate" });
    s.book = in(channel, &.{ "book", "order_book", "depth" });
    const native = if (s.private) "account.orderUpdate" else if (s.book) "depth" else if (eq(u8, channel, "trades")) "trade" else channel;
    const prefix = if ((try optionalText(params, "interval"))) |pace| try std.fmt.allocPrint(a, "{s}.{s}", .{ native, pace }) else native;
    s.topic = t orelse if (s.symbol) |symbol| try std.fmt.allocPrint(a, "{s}.{s}", .{ prefix, symbol }) else prefix;
    s.book = std.mem.startsWith(u8, s.topic, "depth.");
    if (!s.private and s.symbol == null and t == null) return error.InvalidSubscription;
    s.frame = try stringify(a, .{ .method = "SUBSCRIBE", .params = .{s.topic}, .id = "1" });
}

pub fn event(s: *State, _: w.A, frame: w.V) !core.Event {
    if (same(frame, "stream", s.topic) and p.get(frame, "data") != .null) return .deliver;
    return .skip;
}
