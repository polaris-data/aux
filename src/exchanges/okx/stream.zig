const std = @import("std");
const core = @import("../../core/exchange_stream.zig");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const State = core.State;
const eq = std.mem.eql;
const in = core.in;
const same = core.same;
const optionalText = core.optionalText;
const stringify = core.stringify;
const socket = @import("../../core/websocket.zig");

const Argument = struct {
    channel: []const u8,
    instId: ?[]const u8,
    instType: ?[]const u8,
    instFamily: ?[]const u8,
};

fn argument(params: w.V, topic: []const u8) !Argument {
    return .{
        .channel = topic,
        .instId = try optionalText(params, "instId"),
        .instType = (try optionalText(params, "instType")) orelse if (eq(u8, topic, "orders")) @as(?[]const u8, "ANY") else null,
        .instFamily = try optionalText(params, "instFamily"),
    };
}

fn topicFor(channel: []const u8) []const u8 {
    return if (in(channel, &.{ "book", "order_book" })) "books" else if (eq(u8, channel, "ticker")) "tickers" else channel;
}

fn private(topic: []const u8) bool {
    return in(topic, &.{ "orders", "account", "positions", "balance_and_position", "fills", "orders-algo", "algo-advance" });
}

fn business(topic: []const u8) bool {
    return in(topic, &.{ "orders-algo", "algo-advance" }) or std.mem.startsWith(u8, topic, "candle");
}

fn endpoint(a: w.A, testnet: bool, path: []const u8) ![]const u8 {
    const base = if (testnet) "wss://wspap.okx.com/ws/v5/" else "wss://ws.okx.com/ws/v5/";
    return std.mem.concat(a, u8, &.{ base, path });
}

fn frame(a: w.A, op: []const u8, args: []const Argument) ![]const u8 {
    return stringify(a, .{ .op = op, .args = args });
}

pub fn init(s: *State, a: w.A, _: w.V, params: w.V, channel: []const u8, t: ?[]const u8, override: ?[]const u8, testnet: bool) !void {
    s.topic = t orelse topicFor(channel);
    s.private = private(s.topic);
    s.endpoint = override orelse try endpoint(a, testnet, if (business(s.topic)) "business" else if (s.private) "private" else "public");
    const arg = try argument(params, s.topic);
    s.symbol = arg.instId;
    s.book = std.mem.startsWith(u8, s.topic, "books") or eq(u8, s.topic, "bbo-tbt");
    s.frame = try frame(a, "subscribe", &.{arg});
}

pub fn prepare(a: w.A, config: w.V, override: ?[]const u8, items: []const w.V) !socket.PreparedSubscription {
    const args = try a.alloc(Argument, items.len);
    for (items, args, 0..) |item, *arg, i| {
        const channel = try core.requiredText(item, "channel");
        const topic = (try optionalText(item, "topic")) orelse topicFor(channel);
        if (private(topic) or private(channel)) return error.AuthenticationRequired;
        if (!in(topic, &.{ "books", "trades", "candle1m" })) return error.InvalidSubscription;
        _ = try core.requiredText(item, "instId");
        if (i != 0 and business(topic) != business(args[0].channel)) return error.MixedEndpoints;
        arg.* = try argument(item, topic);
    }
    return .{
        .endpoint = override orelse try endpoint(a, try p.testnet(config), if (business(args[0].channel)) "business" else "public"),
        .subscribe = try a.dupe([]const u8, &.{try frame(a, "subscribe", args)}),
        .unsubscribe = try a.dupe([]const u8, &.{try frame(a, "unsubscribe", args)}),
    };
}

pub fn event(s: *State, _: w.A, message: w.V) !core.Event {
    const arg = p.get(message, "arg");
    if (same(arg, "channel", s.topic) and p.get(message, "data") != .null and (s.symbol == null or same(arg, "instId", s.symbol.?))) return .deliver;
    return .skip;
}
