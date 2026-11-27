const std = @import("std");
const wire = @import("../../core/wire.zig");
const websocket = @import("../../core/websocket.zig");
const eq = std.mem.eql;

pub const endpoint = "wss://mds.qfex.com/";

pub const SubscriptionSpec = struct {
    channels: []const []const u8,
    symbols: []const []const u8,
    intervals: []const []const u8 = &.{},
};

pub fn prepare(a: wire.A, override: ?[]const u8, spec: SubscriptionSpec) !websocket.PreparedSubscription {
    const url = override orelse endpoint;
    _ = try websocket.Session.parseEndpoint(url);
    if (spec.channels.len == 0 or spec.symbols.len == 0) return error.EmptySubscription;
    for (spec.symbols) |symbol| try validateSymbol(symbol);

    const market = try a.alloc([]const u8, spec.channels.len);
    var market_count: usize = 0;
    var candle = false;
    for (spec.channels) |requested| {
        const channel = try wireChannel(requested);
        if (eq(u8, channel, "candle")) {
            candle = true;
        } else {
            market[market_count] = channel;
            market_count += 1;
        }
    }
    if (candle != (spec.intervals.len > 0)) return error.InvalidInterval;
    for (spec.intervals) |interval| try validateInterval(interval);

    const count = @as(usize, @intFromBool(market_count > 0)) + @intFromBool(candle);
    const subscribe = try a.alloc([]const u8, count);
    const unsubscribe = try a.alloc([]const u8, count);
    if (market_count > 0) {
        subscribe[0] = try frame(a, "subscribe", market[0..market_count], spec.symbols, null);
        unsubscribe[0] = try frame(a, "unsubscribe", market[0..market_count], spec.symbols, null);
    }
    if (candle) {
        subscribe[count - 1] = try frame(a, "subscribe", &.{"candle"}, spec.symbols, spec.intervals);
        unsubscribe[count - 1] = try frame(a, "unsubscribe", &.{"candle"}, spec.symbols, spec.intervals);
    }
    return .{ .endpoint = try a.dupe(u8, url), .subscribe = subscribe, .unsubscribe = unsubscribe };
}

fn validateSymbol(symbol: []const u8) !void {
    if (std.mem.trim(u8, symbol, " \t\r\n").len == 0) return error.InvalidSymbol;
}

fn wireChannel(requested: []const u8) ![]const u8 {
    const channel = if (eq(u8, requested, "funding_rate")) "funding" else requested;
    for ([_][]const u8{ "level2", "trade", "bbo", "mark_price", "funding", "open_interest", "market_stats", "refdata", "candle" }) |supported| {
        if (eq(u8, channel, supported)) return channel;
    }
    return error.UnknownChannel;
}

fn frame(a: wire.A, operation: []const u8, channels: []const []const u8, symbols: []const []const u8, intervals: ?[]const []const u8) ![]const u8 {
    return std.json.Stringify.valueAlloc(a, .{
        .type = operation,
        .channels = channels,
        .symbols = symbols,
        .intervals = intervals,
    }, .{ .emit_null_optional_fields = false });
}

pub const State = struct {
    channel: []const u8,
    symbol: []const u8,
    interval: ?[]const u8,
    frame: []const u8,

    pub fn init(a: wire.A, params: wire.V) !State {
        const requested = try wire.string(try wire.field(params, "channel"));
        const symbol = try wire.string(try wire.field(params, "symbol"));
        try validateSymbol(symbol);
        const channel = try wireChannel(requested);

        var state: State = .{
            .channel = channel,
            .symbol = symbol,
            .interval = if (eq(u8, channel, "candle")) try candleInterval(params) else null,
            .frame = undefined,
        };
        state.frame = try state.makeFrame(a, "subscribe");
        return state;
    }

    pub fn unsubscribeFrame(self: *const State, a: wire.A) ![]const u8 {
        return self.makeFrame(a, "unsubscribe");
    }

    fn makeFrame(self: *const State, a: wire.A, operation: []const u8) ![]const u8 {
        return frame(a, operation, &.{self.channel}, &.{self.symbol}, if (self.interval) |interval| &.{interval} else null);
    }

    pub fn event(self: *const State, a: wire.A, raw: []const u8) !@import("../derive/stream.zig").State.Event {
        return if (try self.matches(a, raw)) .{ .deliver = raw } else .skip;
    }
    fn matches(self: *const State, arena: wire.A, raw: []const u8) !bool {
        const val = try wire.parse(arena, raw);
        if (val != .object) return error.InvalidEvent;
        for ([_][]const u8{ "error", "error_code", "err_response" }) |key| if (val.object.contains(key)) return error.ExchangeApi;
        const channel = if (val.object.get("type")) |v| try wire.string(v) else "";
        if (val.object.get("symbol")) |v| {
            if (!eq(u8, channel, self.channel)) return false;
            if (!eq(u8, self.symbol, "*") and !eq(u8, try wire.string(v), self.symbol)) return false;
            if (self.interval) |interval| {
                const resolution = val.object.get("resolution") orelse return false;
                return v == .string and resolution == .string and eq(u8, resolution.string, interval);
            }
            return true;
        }
        if (val.object.get("channels")) |v| {
            if (!contains(v, self.channel)) return false;
            const symbols = val.object.get("symbols");
            if (self.interval) |interval| {
                const ack_symbols = symbols orelse return false;
                const ack_intervals = val.object.get("intervals") orelse return false;
                return contains(ack_symbols, self.symbol) and contains(ack_intervals, interval);
            }
            return if (symbols) |items| contains(items, self.symbol) or contains(items, "*") else true;
        }
        if (self.interval != null) return false;
        return eq(u8, channel, self.channel) or if (val.object.get("channel")) |v| eq(u8, try wire.string(v), self.channel) else false;
    }
};

fn candleInterval(params: wire.V) ![]const u8 {
    const intervals = try wire.field(params, "intervals");
    if (intervals != .array or intervals.array.items.len != 1) return error.InvalidInterval;
    const interval = try wire.string(intervals.array.items[0]);
    try validateInterval(interval);
    return interval;
}

fn validateInterval(interval: []const u8) !void {
    for ([_][]const u8{ "1MIN", "5MINS", "15MINS", "30MINS", "1HOUR", "4HOURS", "1DAY" }) |supported| {
        if (eq(u8, interval, supported)) return;
    }
    return error.InvalidInterval;
}

fn contains(v: wire.V, s: []const u8) bool {
    if (v != .array) return false;
    for (v.array.items) |item| if (item == .string and eq(u8, item.string, s)) return true;
    return false;
}
