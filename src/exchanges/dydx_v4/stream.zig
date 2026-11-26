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
const stringify = core.stringify;

pub fn init(s: *State, a: w.A, _: w.V, params: w.V, channel: []const u8, _: ?[]const u8, override: ?[]const u8, testnet: bool) !void {
    s.endpoint = override orelse if (testnet) "wss://indexer.v4testnet.dydx.exchange/v4/ws" else "wss://indexer.dydx.trade/v4/ws";
    s.topic = if (in(channel, &.{ "book", "order_book", "orderbook" })) "v4_orderbook" else if (eq(u8, channel, "trades")) "v4_trades" else if (in(channel, &.{ "ticker", "markets" })) "v4_markets" else if (in(channel, &.{ "account", "orders", "fills" })) "v4_subaccounts" else channel;
    s.book = eq(u8, s.topic, "v4_orderbook");
    const explicit_id = try optionalText(params, "id");
    s.symbol = explicit_id orelse if (eq(u8, s.topic, "v4_markets")) null else if (eq(u8, s.topic, "v4_subaccounts"))
        try std.fmt.allocPrint(a, "{s}/{d}", .{ try requiredText(params, "address"), try w.unsigned(u32, try w.field(params, "subaccount_number")) })
    else
        try requiredText(params, "ticker");
    const flag = p.get(params, "batched");
    s.frame = try stringify(a, .{ .type = "subscribe", .channel = s.topic, .id = s.symbol, .batched = flag == .bool and flag.bool });
}

pub fn event(s: *State, _: w.A, frame: w.V) !core.Event {
    if (same(frame, "channel", s.topic) and (s.symbol == null or same(frame, "id", s.symbol.?)) and
        (same(frame, "type", "channel_data") or same(frame, "type", "channel_batch_data") or same(frame, "type", "subscribed"))) return .deliver;
    return .skip;
}
