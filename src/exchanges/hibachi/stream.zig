const std = @import("std");
const core = @import("../../core/exchange_stream.zig");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const websocket = @import("../../core/websocket.zig");
const State = core.State;
const eq = std.mem.eql;
const in = core.in;
const same = core.same;
const optionalText = core.optionalText;
const requiredText = core.requiredText;
const optionalDepth = core.optionalDepth;
const stringify = core.stringify;
const hibachiAccount = core.hibachiAccount;
const statusOk = core.statusOk;

pub const market_endpoint = "wss://data-api.hibachi.xyz/ws/market";
const MarketTopic = struct { symbol: []const u8, topic: []const u8, depth: ?u32 = null, granularity: ?w.V = null };

fn marketFrame(a: w.A, method: []const u8, topics: []const MarketTopic) ![]const u8 {
    return stringify(a, .{ .method = method, .parameters = .{ .subscriptions = topics } });
}

fn marketTopic(channel: []const u8) []const u8 {
    if (in(channel, &.{ "book", "order_book" })) return "orderbook";
    return if (eq(u8, channel, "ticker")) "mark_price" else channel;
}

fn preparedTopic(channel: []const u8) []const u8 {
    const trimmed = std.mem.trim(u8, channel, &std.ascii.whitespace);
    const aliases = [_]struct { name: []const u8, spellings: []const []const u8 }{
        .{ .name = "trades", .spellings = &.{ "trades", "trade" } },
        .{ .name = "orderbook", .spellings = &.{ "orderbook", "order_book", "book", "depth" } },
        .{ .name = "mark_price", .spellings = &.{ "mark_price", "markprice", "mark-price", "ticker" } },
        .{ .name = "funding_rate_estimation", .spellings = &.{ "funding_rate_estimation", "funding", "funding_rate", "funding-rate", "funding-rate-estimation" } },
    };
    for (aliases) |group| {
        for (group.spellings) |alias| if (std.ascii.eqlIgnoreCase(trimmed, alias)) return group.name;
    }
    return trimmed;
}

pub fn prepareFrames(a: w.A, subscriptions: []const w.V) !websocket.SubscriptionFrames {
    const on = try a.alloc(MarketTopic, subscriptions.len);
    const off = try a.alloc(MarketTopic, subscriptions.len);
    for (subscriptions, on, off) |params, *entry, *remove| {
        const topic = preparedTopic(try requiredText(params, "topic"));
        if (!in(topic, &.{ "mark_price", "spot_price", "ask_bid_price", "funding_rate_estimation", "trades", "klines", "orderbook" })) return error.InvalidSubscription;
        const symbol = try requiredText(params, "symbol");
        const granularity = p.get(params, "granularity");
        const has_depth = p.get(params, "depth") != .null;
        if (!eq(u8, topic, "orderbook") and (has_depth or granularity != .null)) return error.InvalidSubscription;
        remove.* = .{ .symbol = symbol, .topic = topic };
        entry.* = remove.*;
        if (has_depth) entry.depth = try optionalDepth(params, 10);
        if (granularity != .null) entry.granularity = granularity;
    }
    return .{ .subscribe = try marketFrame(a, "subscribe", on), .unsubscribe = try marketFrame(a, "unsubscribe", off) };
}

pub fn init(s: *State, a: w.A, input: w.V, params: w.V, channel: []const u8, t: ?[]const u8, override: ?[]const u8, testnet: bool) !void {
    if (testnet and override == null) return error.InvalidNetwork;
    s.private = in(channel, &.{ "account", "orders", "fills", "balance", "positions" });
    s.endpoint = override orelse if (s.private) "wss://api.hibachi.xyz/ws/account" else market_endpoint;
    if (s.private) {
        const id = try hibachiAccount(params, input);
        s.account = id;
        s.frame = try stringify(a, .{ .id = 1, .method = "stream.start", .params = .{ .accountId = id } });
    } else {
        s.symbol = try requiredText(params, "symbol");
        s.topic = t orelse marketTopic(channel);
        s.book = eq(u8, s.topic, "orderbook");
        s.depth = if (s.book and p.get(params, "depth") != .null) try optionalDepth(params, 10) else null;
        const granularity = p.get(params, "granularity");
        s.frame = try marketFrame(a, "subscribe", &.{.{ .symbol = s.symbol.?, .topic = s.topic, .depth = s.depth, .granularity = if (granularity == .null) null else granularity }});
    }
}

pub fn event(s: *State, a: w.A, frame: w.V) !core.Event {
    if (s.private) {
        const status = p.get(frame, "status");
        if (status != .null and !statusOk(frame, "status")) return error.ExchangeApi;
        const listen = p.get(p.get(frame, "result"), "listenKey");
        if (listen == .string) return .{ .heartbeat = try stringify(a, .{ .id = 2, .method = "stream.ping", .params = .{ .accountId = s.account.?, .listenKey = listen.string } }) };
        const data = p.get(frame, "event");
        const tag = if (data == .string) data.string else (try optionalText(frame, "eventType")) orelse return .skip;
        if (eq(u8, s.channel, "account") or
            (eq(u8, s.channel, "orders") and std.mem.startsWith(u8, tag, "ORDER_")) or
            (eq(u8, s.channel, "fills") and eq(u8, tag, "trade_update")) or
            (eq(u8, s.channel, "positions") and eq(u8, tag, "position_update")) or
            (eq(u8, s.channel, "balance") and eq(u8, tag, "balance_update"))) return .deliver;
    } else if (same(frame, "topic", s.topic) and (p.get(frame, "symbol") == .null or same(frame, "symbol", s.symbol.?))) return .deliver;
    return .skip;
}
