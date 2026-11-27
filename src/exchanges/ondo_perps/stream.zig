const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const websocket = @import("../../core/websocket.zig");
const Event = @import("../derive/stream.zig").State.Event;

const Channel = enum {
    tradesPerps,
    depthBooksPerps,
    topOfBooksPerps,
    markPricesPerps,
    fundingRatesPerps,
    kLinePerps,
    ordersPerps,
    fillsPerps,
    positionsPerps,
    balancePerps,

    fn public(self: Channel) bool {
        return switch (self) {
            .tradesPerps, .depthBooksPerps, .topOfBooksPerps, .markPricesPerps, .fundingRatesPerps, .kLinePerps => true,
            else => false,
        };
    }
};

pub fn endpoint(config: p.V) ![]const u8 {
    return if (try p.testnet(config)) "wss://api.ondoperps-sandbox.xyz/ws" else "wss://api.ondoperps.xyz/ws";
}

fn channelFrom(params: p.V) !Channel {
    const name = try p.text(params, "channel");
    for ([_][]const u8{ "perpsTrades", "perpsDepthBook", "perpsTopOfBook", "perpsMarkPrices", "perpsFundingRates", "perpsKLine", "kline", "candles" }, [_]Channel{ .tradesPerps, .depthBooksPerps, .topOfBooksPerps, .markPricesPerps, .fundingRatesPerps, .kLinePerps, .kLinePerps, .kLinePerps }) |alias, channel| {
        if (p.eq(u8, name, alias)) return channel;
    }
    return std.meta.stringToEnum(Channel, name) orelse error.InvalidSubscription;
}

fn marketsFrom(params: p.V) !p.V {
    const markets = p.get(params, "markets");
    if (markets != .null) {
        if (markets != .array or markets.array.items.len == 0) return error.InvalidSubscription;
        for (markets.array.items) |item| if ((try w.string(item)).len == 0) return error.InvalidMarket;
    }
    return markets;
}

fn frame(a: p.A, operation: []const u8, channel: Channel, markets: p.V, params: p.V, fields: []const []const u8) ![]const u8 {
    var subscription = p.object();
    try p.put(&subscription, a, "op", w.text(operation));
    try p.put(&subscription, a, "channel", w.text(@tagName(channel)));
    if (markets != .null) try p.put(&subscription, a, "markets", markets);
    for (fields) |field| {
        const value = p.get(params, field);
        if (value != .null) try p.put(&subscription, a, field, value);
    }
    return std.json.Stringify.valueAlloc(a, subscription, .{});
}

fn candleResolution(params: p.V) !void {
    const resolution = try p.text(params, "resolution");
    for ([_][]const u8{ "1", "5", "15", "1H", "4H", "1D", "1W" }) |valid| {
        if (p.eq(u8, valid, resolution)) return;
    }
    return error.InvalidInterval;
}

pub fn prepareFrames(a: p.A, params: p.V) ![]const websocket.SubscriptionFrames {
    const channel = try channelFrom(params);
    if (!channel.public()) return error.AuthenticationRequired;
    const markets = try marketsFrom(params);
    const fields: []const []const u8 = switch (channel) {
        .tradesPerps => &.{"numPastTrades"},
        .depthBooksPerps => &.{ "depthLevels", "limit" },
        .kLinePerps => &.{"resolution"},
        else => &.{},
    };
    if (channel == .kLinePerps) {
        try candleResolution(params);
        if (markets == .null) return error.InvalidMarket;
        const pairs = try a.alloc(websocket.SubscriptionFrames, markets.array.items.len);
        for (markets.array.items, pairs) |market, *pair| {
            const single = try p.array(a, &.{market});
            pair.* = .{
                .subscribe = try frame(a, "subscribe", channel, single, params, fields),
                .unsubscribe = try frame(a, "unsubscribe", channel, single, params, fields),
            };
        }
        return pairs;
    }
    const pairs = try a.alloc(websocket.SubscriptionFrames, 1);
    pairs[0] = .{
        .subscribe = try frame(a, "subscribe", channel, markets, params, fields),
        .unsubscribe = try frame(a, "unsubscribe", channel, markets, params, fields),
    };
    return pairs;
}

pub const State = struct {
    endpoint: []const u8,
    frame: []const u8,
    login: ?[]const u8,
    channel: Channel,
    markets: p.V,
    authenticated: bool = false,

    pub fn init(a: p.A, config: p.V, params: p.V) !State {
        const channel = try channelFrom(params);
        const private = !channel.public();
        const login = if (private) try std.json.Stringify.valueAlloc(a, .{
            .op = "login",
            .args = .{ .token = try p.credential(p.get(p.get(config, "auth"), "token")) },
        }, .{}) else null;
        const markets = try marketsFrom(params);
        if (channel == .kLinePerps) {
            if (markets == .null or markets.array.items.len != 1) return error.InvalidMarket;
            try candleResolution(params);
        }
        return .{
            .endpoint = try endpoint(config),
            .frame = try frame(a, "subscribe", channel, markets, params, if (channel == .kLinePerps) &.{"resolution"} else &.{ "numPastTrades", "depthLevels", "limit" }),
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
