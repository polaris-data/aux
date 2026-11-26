const std = @import("std");
const x = @import("../../core/exchange_stream.zig");
const w = @import("../../core/wire.zig");
const p = @import("../../core/perps.zig");

pub const subscription_id = 1;

pub const Subscription = struct {
    address: [20]u8,
    expected_epoch: ?u64,
    epoch: ?u64 = null,
    book: bool,
};

fn cursor(params: w.V, key: []const u8) !?u64 {
    const field = p.get(params, key);
    if (field == .null) return null;
    return w.unsigned(u64, field) catch return error.InvalidSubscription;
}

pub fn init(s: *x.State, a: w.A, params: w.V, channel: []const u8, endpoint: ?[]const u8, testnet: bool) !void {
    const book = x.in(channel, &.{ "book", "l2Delta@f" });
    if (!book and !x.in(channel, &.{ "trades", "trades@f" })) return error.InvalidSubscription;
    s.topic = if (book) "l2Delta@f" else "trades@f";
    s.auto_reconnect = false;
    s.endpoint = endpoint orelse if (testnet) "wss://gateway.testnet.kuru.io/ws" else "wss://gateway.mainnet.kuru.io/ws";
    const market = try x.requiredText(params, "marketAddress");
    if (market.len != 42 or !std.mem.startsWith(u8, market, "0x")) return error.InvalidSubscription;
    var address: [20]u8 = undefined;
    _ = std.fmt.hexToBytes(&address, market[2..]) catch return error.InvalidSubscription;
    s.symbol = try std.fmt.allocPrint(a, "0x{x}", .{&address});
    const epoch = try cursor(params, "feedEpoch");
    const seq = try cursor(params, "seq");
    if (seq != null and (epoch == null or seq.? == 0)) return error.InvalidSubscription;
    s.kuru = .{ .address = address, .expected_epoch = epoch, .book = book };
    s.frame = try x.stringify(a, .{ .op = "sub", .id = subscription_id, .t = s.topic, .m = s.symbol.?, .feedEpoch = epoch, .seq = seq });
}

pub fn event(s: *x.State, frame: w.V) !x.Event {
    const id = p.get(frame, "id");
    const matches = (w.unsigned(u64, id) catch 0) == subscription_id;
    if (x.same(frame, "op", "err") and (matches or id == .null)) return error.ExchangeApi;
    if (!x.same(frame, "t", s.topic)) return .skip;
    if (matches and x.same(frame, "op", "ack")) {
        const epoch = w.unsigned(u64, p.get(frame, "feedEpoch")) catch return error.InvalidResponse;
        const state = &s.kuru.?;
        if (state.expected_epoch) |expected| if (expected != epoch) return error.ExchangeApi;
        state.epoch = epoch;
    } else if (matches and x.same(frame, "op", "resub")) {
        return .deliver_final;
    } else if (x.same(frame, "op", "resync")) {
        const market = p.get(frame, "m");
        if (market == .string and std.ascii.eqlIgnoreCase(market.string, s.symbol.?)) return .deliver_final;
    }
    return .skip;
}
