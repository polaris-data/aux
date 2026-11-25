const std = @import("std");
const w = @import("../../core/wire.zig");
const p = @import("../../core/perps.zig");
const connectors = @import("../../core/perps_connectors.zig");
const shared = @import("../../core/perps_stream.zig");
const Event = shared.Event;
const eq = std.mem.eql;
const gpa = std.heap.c_allocator;

pub fn init(self: *shared.State, a: p.A, testnet: bool, params: p.V) !void {
    const channel = self.channel;
    const index = try connectors.enumIndex(channel, &.{ "l2orderbook", "trades", "orders", "positions" });
    self.private = index >= 2;
    self.book = index == 0;
    self.endpoint = if (testnet) "wss://websocket-perps-sandbox.katana.network/v1" else "wss://websocket-perps.katana.network/v1";
    if (self.private) {
        const wallet = p.get(params, "wallet");
        self.account = try p.credential(if (wallet == .null) p.get(p.get(self.config, "auth"), "wallet") else wallet);
        _ = try @import("../../core/evm.zig").hex(20, self.account.?);
    } else {
        self.market = try p.text(params, "market");
        if (self.market.?.len == 0) return error.InvalidMarket;
        self.request = try std.json.Stringify.valueAlloc(a, .{ .method = "subscribe", .markets = .{self.market.?}, .subscriptions = .{channel} }, .{});
    }
}

pub fn event(self: *shared.State, message: p.V, raw: []const u8) !Event {
    if (shared.same(message, "type", "error") or shared.same(message, "type", "subscriptions") and p.get(message, "code") != .null) return error.ExchangeApi;
    if (shared.same(message, "type", "subscriptions")) {
        const subscriptions = try w.field(message, "subscriptions");
        if (subscriptions != .array) return error.InvalidResponse;
        for (subscriptions.array.items) |subscription| {
            if (!shared.same(subscription, "name", self.channel)) continue;
            if (self.market) |market| {
                const markets = p.get(subscription, "markets");
                if (markets != .array) continue;
                for (markets.array.items) |value| {
                    if (value == .string and eq(u8, value.string, market)) {
                        self.ready = true;
                        return .skip;
                    }
                }
            } else {
                self.ready = true;
                return .skip;
            }
        }
        return error.ExchangeApi;
    }
    if (!shared.same(message, "type", self.channel)) return .skip;
    const data = try w.field(message, "data");
    if (self.market) |market| if (!shared.same(data, "m", market)) return .skip;
    if (self.account) |account| {
        const wallet = try p.text(data, "w");
        if (!std.ascii.eqlIgnoreCase(wallet, account)) return .skip;
    }
    if (self.book) {
        // Promote a queued snapshot before delivering any buffered deltas.
        if (self.pending_snapshot != null) return .skip;
        const next = try w.unsigned(u64, try w.field(data, "u"));
        const current = self.sequence orelse return error.SequenceGap;
        if (next <= current) return .skip;
        if (next != current +| 1) return error.SequenceGap;
        self.sequence = next;
    }
    return .{ .deliver = raw };
}
