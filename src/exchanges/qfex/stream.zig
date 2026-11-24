const std = @import("std");
const wire = @import("../../core/wire.zig");
const eq = std.mem.eql;

pub const State = struct {
    channel: []const u8,
    symbol: []const u8,
    frame: []const u8,

    pub fn init(a: wire.A, params: wire.V) !State {
        const channel = try wire.string(try wire.field(params, "channel"));
        const symbol = try wire.string(try wire.field(params, "symbol"));
        if (std.mem.trim(u8, symbol, " \t\r\n").len == 0) return error.InvalidSymbol;
        for ([_][]const u8{ "level2", "trade", "bbo", "mark_price", "funding_rate", "open_interest", "market_stats", "refdata" }) |c| {
            if (eq(u8, c, channel)) break;
        } else return error.UnknownChannel;
        const frame = try std.json.Stringify.valueAlloc(a, .{ .type = "subscribe", .channels = .{channel}, .symbols = .{symbol} }, .{});

        return .{ .channel = channel, .symbol = symbol, .frame = frame };
    }

    pub fn event(self: *const State, a: wire.A, raw: []const u8) !@import("../derive/stream.zig").State.Event {
        return if (try self.matches(a, raw)) .{ .deliver = raw } else .skip;
    }
    fn matches(self: *const State, arena: wire.A, raw: []const u8) !bool {
        const val = try wire.parse(arena, raw);
        if (val != .object) return error.InvalidEvent;
        for ([_][]const u8{ "error", "error_code", "err_response" }) |key| if (val.object.contains(key)) return error.ExchangeApi;
        const channel = if (val.object.get("type")) |v| try wire.string(v) else "";
        if (val.object.get("symbol")) |v| return eq(u8, channel, self.channel) and (eq(u8, self.symbol, "*") or eq(u8, try wire.string(v), self.symbol));
        if (val.object.get("channels")) |v| {
            if (!contains(v, self.channel)) return false;
            return if (val.object.get("symbols")) |symbols| contains(symbols, self.symbol) or contains(symbols, "*") else true;
        }
        return eq(u8, channel, self.channel) or if (val.object.get("channel")) |v| eq(u8, try wire.string(v), self.channel) else false;
    }
};

fn contains(v: wire.V, s: []const u8) bool {
    if (v != .array) return false;
    for (v.array.items) |item| if (item == .string and eq(u8, item.string, s)) return true;
    return false;
}
