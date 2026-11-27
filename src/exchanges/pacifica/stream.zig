const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const websocket = @import("../../core/websocket.zig");
const Event = @import("../derive/stream.zig").State.Event;

const Channel = enum { trades, book, bbo, prices, candle, account_info, account_positions, account_order_updates, account_trades };

pub fn endpoint(config: p.V) ![]const u8 {
    return if (try p.testnet(config)) "wss://test-ws.pacifica.fi/ws" else "wss://ws.pacifica.fi/ws";
}

fn frame(a: p.A, method: []const u8, request: p.V) ![]const u8 {
    return std.json.Stringify.valueAlloc(a, .{ .method = method, .params = request }, .{});
}

pub fn prepareFrames(a: p.A, config: p.V, params: p.V) !websocket.SubscriptionFrames {
    const state = try State.init(a, config, params);
    if (state.account != null) return error.AuthenticationRequired;
    return .{ .subscribe = state.frame, .unsubscribe = try frame(a, "unsubscribe", state.request) };
}

pub const State = struct {
    endpoint: []const u8,
    channel: Channel,
    symbol: ?[]const u8,
    account: ?[]const u8,
    request: p.V,
    frame: []const u8,

    pub fn init(a: p.A, config: p.V, params: p.V) !State {
        const channel = std.meta.stringToEnum(Channel, try p.text(params, "source")) orelse return error.InvalidSubscription;
        const account_channel = switch (channel) {
            .account_info, .account_positions, .account_order_updates, .account_trades => true,
            else => false,
        };
        var request = p.object();
        try p.put(&request, a, "source", w.text(@tagName(channel)));
        const account = if (account_channel) try p.text(params, "account") else null;
        const symbol = if (!account_channel and channel != .prices) try p.text(params, "symbol") else null;
        if (account) |address| {
            _ = try @import("../../core/solana.zig").publicKey(address);
            try p.put(&request, a, "account", w.text(address));
        }
        if (symbol) |name| {
            if (name.len == 0) return error.InvalidMarket;
            try p.put(&request, a, "symbol", w.text(name));
        }
        const aggregate_level = p.get(params, "agg_level");
        if (channel == .book) {
            const level = if (aggregate_level == .null) 1 else try w.unsigned(u16, aggregate_level);
            switch (level) {
                1, 2, 5, 10, 100, 1000 => {},
                else => return error.InvalidSubscription,
            }
            try p.put(&request, a, "agg_level", .{ .integer = level });
        } else if (aggregate_level != .null) return error.InvalidSubscription;
        if (channel == .candle) {
            const interval = try p.text(params, "interval");
            for ([_][]const u8{ "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "8h", "12h", "1d", "1w", "1M" }) |valid| {
                if (p.eq(u8, valid, interval)) break;
            } else return error.InvalidInterval;
            try p.put(&request, a, "interval", w.text(interval));
        }
        return .{
            .endpoint = try endpoint(config),
            .channel = channel,
            .symbol = symbol,
            .account = account,
            .request = request,
            .frame = try frame(a, "subscribe", request),
        };
    }

    fn matches(self: State, value: p.V) !bool {
        if (self.symbol) |symbol| if (!p.eq(u8, symbol, try p.text(value, "s"))) return false;
        if (self.account) |account| {
            const user = p.get(value, "u");
            if (user != .null and !p.eq(u8, account, try w.string(user))) return false;
        }
        return true;
    }

    pub fn event(self: *State, a: p.A, raw: []const u8) !Event {
        const value = try w.parse(a, raw);
        if (p.get(value, "error") != .null or p.get(value, "err") != .null) return error.ExchangeApi;
        const tag = try p.text(value, "channel");
        if (p.eq(u8, tag, "error")) return error.ExchangeApi;
        if (!p.eq(u8, tag, @tagName(self.channel))) return .skip;
        const data = try w.field(value, "data");
        if (data == .array) {
            var items = try p.array(a, &.{});
            for (data.array.items) |item| if (try self.matches(item)) try items.array.append(item);
            if (items.array.items.len == 0 and data.array.items.len != 0) return .skip;
            if (items.array.items.len != data.array.items.len) {
                var updated = value;
                try p.put(&updated, a, "data", items);
                return .{ .deliver = try std.json.Stringify.valueAlloc(a, updated, .{}) };
            }
        } else if (!try self.matches(data)) return .skip;
        return .{ .deliver = raw };
    }
};
