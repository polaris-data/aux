const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const Event = @import("../derive/stream.zig").State.Event;

const Channel = enum { tradesPerps, depthBooksPerps, ordersPerps, fillsPerps, positionsPerps, balancePerps };

pub const State = struct {
    endpoint: []const u8,
    frame: []const u8,
    login: ?[]const u8,
    channel: Channel,
    markets: p.V,
    authenticated: bool = false,

    pub fn init(a: p.A, config: p.V, params: p.V) !State {
        const channel = std.meta.stringToEnum(Channel, try p.text(params, "channel")) orelse return error.InvalidSubscription;
        const private = channel != .tradesPerps and channel != .depthBooksPerps;
        const login = if (private) try std.json.Stringify.valueAlloc(a, .{
            .op = "login",
            .args = .{ .token = try p.credential(p.get(p.get(config, "auth"), "token")) },
        }, .{}) else null;
        var subscription = p.object();
        try p.put(&subscription, a, "op", w.text("subscribe"));
        try p.put(&subscription, a, "channel", w.text(@tagName(channel)));
        const markets = p.get(params, "markets");
        if (markets != .null) {
            if (markets != .array or markets.array.items.len == 0) return error.InvalidSubscription;
            for (markets.array.items) |item| if ((try w.string(item)).len == 0) return error.InvalidMarket;
            try p.put(&subscription, a, "markets", markets);
        }
        for ([_][]const u8{ "numPastTrades", "depthLevels", "limit" }) |field| {
            const value = p.get(params, field);
            if (value != .null) try p.put(&subscription, a, field, value);
        }
        return .{
            .endpoint = if (try p.testnet(config)) "wss://api.ondoperps-sandbox.xyz/ws" else "wss://api.ondoperps.xyz/ws",
            .frame = try std.json.Stringify.valueAlloc(a, subscription, .{}),
            .login = login,
            .channel = channel,
            .markets = markets,
        };
    }

    pub fn start(self: *State) []const u8 {
        self.authenticated = false;
        return self.login orelse self.frame;
    }

    fn matches(self: State, data: p.V) !bool {
        if (self.markets == .null) return true;
        const market = p.get(data, "market");
        const symbol = if (market == .null) p.get(data, "m") else market;
        if (symbol == .null) return self.channel != .tradesPerps and self.channel != .depthBooksPerps;
        const name = try w.string(symbol);
        for (self.markets.array.items) |item| if (p.eq(u8, item.string, name)) return true;
        return false;
    }

    pub fn event(self: *State, a: p.A, raw: []const u8) !Event {
        const value = try w.parse(a, raw);
        const kind = try p.text(value, "type");
        if (p.eq(u8, kind, "error")) return error.ExchangeApi;
        if (self.login != null and !self.authenticated) {
            if (!p.eq(u8, kind, "loggedIn")) return .skip;
            self.authenticated = true;
            return .subscribe;
        }
        if (!p.eq(u8, kind, "update")) return .skip;
        if (!p.eq(u8, try p.text(value, "channel"), @tagName(self.channel))) return .skip;
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
