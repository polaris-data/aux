const std = @import("std");
const core = @import("../../core/exchange_stream.zig");
const http = @import("../../core/http.zig");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const State = core.State;
const eq = std.mem.eql;
const in = core.in;
const same = core.same;
const requiredText = core.requiredText;
const stringify = core.stringify;

// The official JavaScript and Python SDKs share this public feed identifier.
const public_stream_id = "OqdphuyCtYWxwzhxyLLjOWNdFP7sQt8RPWzmb5xY";

pub fn init(s: *State, a: w.A, input: w.V, params: w.V, channel: []const u8, _: ?[]const u8, override: ?[]const u8, testnet: bool) !void {
    s.private = in(channel, &.{ "executionreport", "algoexecutionreport", "orders", "fills", "balance", "account", "position" });
    const account = p.get(p.get(input, "auth"), "account_id");
    const request_id = p.get(p.get(input, "auth"), "ws_id");
    s.endpoint = override orelse try std.mem.concat(a, u8, &.{
        if (s.private) (if (testnet) "wss://testnet-ws-private-evm.orderly.org/v2/ws/private/stream/" else "wss://ws-private-evm.orderly.org/v2/ws/private/stream/") else (if (testnet) "wss://testnet-ws-evm.orderly.org/ws/stream/" else "wss://ws-evm.orderly.org/ws/stream/"),
        try http.escape(a, if (!s.private and account == .null) public_stream_id else try p.credential(account), false),
    });
    const family = if (in(channel, &.{ "book", "order_book" })) "orderbook" else if (eq(u8, channel, "trades")) "trade" else if (in(channel, &.{ "orders", "fills" })) "executionreport" else channel;
    s.book = in(family, &.{ "orderbook", "orderbookupdate" });
    s.topic = if (s.private) family else try std.fmt.allocPrint(a, "{s}@{s}", .{ try requiredText(params, "symbol"), family });
    s.frame = try stringify(a, .{ .id = if (request_id == .null) "1" else try p.credential(request_id), .event = "subscribe", .topic = s.topic });
}

pub fn event(s: *State, _: w.A, frame: w.V) !core.Event {
    if (same(frame, "topic", s.topic) and p.get(frame, "data") != .null) return .deliver;
    return .skip;
}
