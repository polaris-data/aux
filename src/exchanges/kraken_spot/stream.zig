const std = @import("std");
const core = @import("../../core/exchange_stream.zig");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const State = core.State;
const eq = std.mem.eql;
const in = core.in;
const same = core.same;
const requiredText = core.requiredText;
const optionalDepth = core.optionalDepth;
const stringify = core.stringify;

pub fn init(s: *State, a: w.A, _: w.V, params: w.V, channel: []const u8, _: ?[]const u8, override: ?[]const u8, testnet: bool) !void {
    if (testnet and override == null) return error.InvalidNetwork;
    s.topic = if (in(channel, &.{ "orders", "fills" })) "executions" else if (eq(u8, channel, "trades")) "trade" else if (eq(u8, channel, "order_book")) "book" else channel;
    s.private = in(s.topic, &.{ "executions", "balances" });
    s.book = eq(u8, s.topic, "book");
    s.endpoint = override orelse if (s.private) "wss://ws-auth.kraken.com/v2" else "wss://ws.kraken.com/v2";
    if (!s.private) {
        const symbols = p.get(params, "symbols");
        if (symbols != .null) {
            if (symbols != .array or symbols.array.items.len == 0) return error.InvalidSubscription;
            for (symbols.array.items) |value| _ = try w.string(value);
            s.symbols = symbols;
        } else s.symbols = try p.array(a, &.{w.text(try requiredText(params, "symbol"))});
        s.depth = if (s.book) try optionalDepth(params, 10) else null;
        const snapshot = p.get(params, "snapshot");
        s.snapshot = if (snapshot == .bool) snapshot.bool else null;
        s.frame = try stringify(a, .{ .method = "subscribe", .req_id = 1, .params = .{ .channel = s.topic, .symbol = s.symbols, .depth = s.depth, .snapshot = s.snapshot } });
    }
}

pub fn event(s: *State, _: w.A, frame: w.V) !core.Event {
    if (!same(frame, "channel", s.topic)) return .skip;
    const data = p.get(frame, "data");
    if (data != .array or data.array.items.len == 0) return .skip;
    if (s.symbols) |wanted| {
        for (data.array.items) |item| {
            const symbol = p.get(item, "symbol");
            if (symbol != .string) return .skip;
            for (wanted.array.items) |filter| {
                if (eq(u8, symbol.string, filter.string)) break;
            } else return .skip;
        }
    }
    return .deliver;
}
