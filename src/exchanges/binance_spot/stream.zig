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
const kline_intervals = &.{ "1s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w", "1M" };

pub fn init(s: *State, a: w.A, _: w.V, params: w.V, channel: []const u8, t: ?[]const u8, override: ?[]const u8, testnet: bool) !void {
    s.private = in(channel, &.{ "orders", "fills", "user_data", "executionReport" });
    s.endpoint = override orelse if (s.private) (if (testnet) "wss://ws-api.testnet.binance.vision/ws-api/v3" else "wss://ws-api.binance.com/ws-api/v3") else (if (testnet) "wss://stream.testnet.binance.vision/ws" else "wss://stream.binance.com:443/ws");
    if (!s.private) {
        const name = try requiredText(params, "symbol");
        s.symbol = name;
        s.topic = t orelse topic: {
            const symbol = try std.ascii.allocLowerString(a, name);
            const interval = try optionalText(params, "interval");
            const suffix = try topicSuffix(a, params, channel, interval);
            break :topic try std.fmt.allocPrint(a, "{s}@{s}", .{ symbol, suffix });
        };
        const stream_name = std.mem.indexOfScalar(u8, s.topic, '@') orelse return error.InvalidSubscription;
        s.book = std.mem.startsWith(u8, s.topic[stream_name + 1 ..], "depth");
        s.frame = try stringify(a, .{ .method = "SUBSCRIBE", .params = .{s.topic}, .id = 1 });
    }
}

fn topicSuffix(a: w.A, params: w.V, channel: []const u8, interval: ?[]const u8) ![]const u8 {
    const pace = interval orelse "100ms";
    if (in(channel, &.{ "book", "depth", "order_book" })) return std.fmt.allocPrint(a, "depth@{s}", .{pace});
    if (eq(u8, channel, "partial_depth")) return std.fmt.allocPrint(a, "depth{d}@{s}", .{ try optionalDepth(params, 20), pace });
    if (eq(u8, channel, "trades")) return "trade";
    if (eq(u8, channel, "kline")) {
        const period = interval orelse "1m";
        if (!in(period, kline_intervals)) return error.InvalidSubscription;
        return std.fmt.allocPrint(a, "kline_{s}", .{period});
    }
    return channel;
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
        if (same(frame, "e", "kline")) {
            if (symbol != .string) return .skip;
            const topic = s.topic[std.mem.indexOfScalar(u8, s.topic, '@').? + 1 ..];
            const period = std.mem.cutPrefix(u8, topic, "kline_") orelse return .skip;
            const end = std.mem.indexOfScalar(u8, period, '@') orelse period.len;
            return if (same(p.get(frame, "k"), "i", period[0..end])) .deliver else .skip;
        }
        if (same(frame, "e", if (eq(u8, s.channel, "ticker")) "24hrTicker" else if (eq(u8, s.channel, "trades")) "trade" else s.channel)) return .deliver;
        if (p.get(frame, "code") != .null) return error.ExchangeApi;
    }
    return .skip;
}
