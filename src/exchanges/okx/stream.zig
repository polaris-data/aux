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
    const native = if (in(channel, &.{ "book", "order_book" })) "books" else if (eq(u8, channel, "ticker")) "tickers" else channel;
    s.topic = t orelse native;
    s.private = in(s.topic, &.{ "orders", "account", "positions", "balance_and_position", "fills", "orders-algo", "algo-advance" });
    const base = if (testnet) "wss://wspap.okx.com/ws/v5/" else "wss://ws.okx.com/ws/v5/";
    s.endpoint = override orelse try std.mem.concat(a, u8, &.{ base, if (in(s.topic, &.{ "orders-algo", "algo-advance" })) "business" else if (s.private) "private" else "public" });
    s.symbol = try optionalText(params, "instId");
    s.book = std.mem.startsWith(u8, s.topic, "books") or eq(u8, s.topic, "bbo-tbt");
    const instrument_type = (try optionalText(params, "instType")) orelse if (eq(u8, s.topic, "orders")) @as(?[]const u8, "ANY") else null;
    s.frame = try stringify(a, .{ .op = "subscribe", .args = .{.{ .channel = s.topic, .instId = s.symbol, .instType = instrument_type, .instFamily = try optionalText(params, "instFamily") }} });
}

pub fn event(s: *State, _: w.A, frame: w.V) !core.Event {
    const arg = p.get(frame, "arg");
    if (same(arg, "channel", s.topic) and p.get(frame, "data") != .null and (s.symbol == null or same(arg, "instId", s.symbol.?))) return .deliver;
    return .skip;
}
