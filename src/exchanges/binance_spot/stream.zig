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
    s.private = in(channel, &.{ "orders", "fills", "user_data", "executionReport" });
    s.endpoint = override orelse if (s.private) (if (testnet) "wss://ws-api.testnet.binance.vision/ws-api/v3" else "wss://ws-api.binance.com/ws-api/v3") else (if (testnet) "wss://stream.testnet.binance.vision/ws" else "wss://stream.binance.com:443/ws");
    if (!s.private) {
        const name = try requiredText(params, "symbol");
        s.symbol = name;
        const symbol = try std.ascii.allocLowerString(a, name);
        const pace = (try optionalText(params, "interval")) orelse "100ms";
        const suffix = if (in(channel, &.{ "book", "depth", "order_book" })) try std.fmt.allocPrint(a, "depth@{s}", .{pace}) else if (eq(u8, channel, "partial_depth")) try std.fmt.allocPrint(a, "depth{d}@{s}", .{ try optionalDepth(params, 20), pace }) else if (eq(u8, channel, "trades")) "trade" else channel;
        s.topic = t orelse try std.fmt.allocPrint(a, "{s}@{s}", .{ symbol, suffix });
        const stream_name = std.mem.indexOfScalar(u8, s.topic, '@') orelse return error.InvalidSubscription;
        s.book = std.mem.startsWith(u8, s.topic[stream_name + 1 ..], "depth");
        s.frame = try stringify(a, .{ .method = "SUBSCRIBE", .params = .{s.topic}, .id = 1 });
    }
}

pub fn event(s: *State, _: w.A, frame: w.V) !core.Event {
    if (s.private) {
        const data = p.get(frame, "event");
        if (eq(u8, s.channel, "user_data") and data == .object) return .deliver;
        if (same(data, "e", "executionReport") and (!eq(u8, s.channel, "fills") or same(data, "x", "TRADE"))) return .deliver;
    } else {
        const stream = p.get(frame, "stream");
        if (stream != .null) return if (same(frame, "stream", s.topic)) .deliver else .skip;
        const symbol = p.get(frame, "s");
        if (symbol == .string and !std.ascii.eqlIgnoreCase(s.symbol.?, symbol.string)) return .skip;
        if (s.book and (same(frame, "e", "depthUpdate") or p.get(frame, "lastUpdateId") != .null)) return .deliver;
        if (same(frame, "e", if (eq(u8, s.channel, "ticker")) "24hrTicker" else if (eq(u8, s.channel, "trades")) "trade" else s.channel)) return .deliver;
        if (p.get(frame, "code") != .null) return error.ExchangeApi;
    }
    return .skip;
}
