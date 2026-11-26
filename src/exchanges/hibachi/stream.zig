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
const hibachiAccount = core.hibachiAccount;
const statusOk = core.statusOk;

pub fn init(s: *State, a: w.A, input: w.V, params: w.V, channel: []const u8, t: ?[]const u8, override: ?[]const u8, testnet: bool) !void {
    if (testnet and override == null) return error.InvalidNetwork;
    s.private = in(channel, &.{ "account", "orders", "fills", "balance", "positions" });
    s.endpoint = override orelse if (s.private) "wss://api.hibachi.xyz/ws/account" else "wss://data-api.hibachi.xyz/ws/market";
    if (s.private) {
        const id = try hibachiAccount(params, input);
        s.account = id;
        s.frame = try stringify(a, .{ .id = 1, .method = "stream.start", .params = .{ .accountId = id } });
    } else {
        s.symbol = try requiredText(params, "symbol");
        s.topic = t orelse if (in(channel, &.{ "book", "order_book" })) "orderbook" else if (eq(u8, channel, "ticker")) "mark_price" else channel;
        s.book = eq(u8, s.topic, "orderbook");
        s.depth = if (s.book and p.get(params, "depth") != .null) try optionalDepth(params, 10) else null;
        const granularity = p.get(params, "granularity");
        s.frame = try stringify(a, .{ .method = "subscribe", .parameters = .{ .subscriptions = .{.{ .symbol = s.symbol, .topic = s.topic, .depth = s.depth, .granularity = if (granularity == .null) @as(?w.V, null) else granularity }} } });
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
