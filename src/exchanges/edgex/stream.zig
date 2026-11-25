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
    if (testnet) return error.InvalidNetwork;
    self.private = eq(u8, channel, "account");
    self.book = eq(u8, channel, "depth");
    if (!self.private and !self.book and !eq(u8, channel, "trades")) return error.InvalidSubscription;
    self.endpoint = if (self.private) "wss://edgex-quote-prod-v2.edgex.exchange/api/v1/private/ws" else "wss://edgex-quote-prod-v2.edgex.exchange/api/v1/public/ws";
    if (!self.private) {
        const id = try w.unsigned(u64, try w.field(params, "contract_id"));
        self.market = try std.fmt.allocPrint(a, "{d}", .{id});
        if (self.book) {
            const depth = try shared.optional(u16, params, "depth", 200);
            if (depth != 15 and depth != 200) return error.InvalidSubscription;
            self.selector = try std.fmt.allocPrint(a, "depth.{d}.{d}", .{ id, depth });
        } else self.selector = try std.fmt.allocPrint(a, "trades.{d}", .{id});
        self.request = try std.json.Stringify.valueAlloc(a, .{ .type = "subscribe", .channel = self.selector }, .{});
    }
}

pub fn event(self: *shared.State, a: p.A, message: p.V, raw: []const u8) !Event {
    if (shared.same(message, "type", "error")) return error.ExchangeApi;
    if (shared.same(message, "type", "ping")) return .{
        .reply = try std.json.Stringify.valueAlloc(a, .{ .type = "pong", .time = try w.field(message, "time") }, .{}),
    };
    if (shared.same(message, "type", "connected") and !self.private and !self.ready) {
        self.ready = true;
        return .subscribe;
    }
    const content = p.get(message, "content");
    if (self.private) {
        if (!shared.same(message, "type", "trade-event")) return .skip;
        if (shared.same(content, "event", "Snapshot")) self.snapshot = true;
        return if (self.snapshot) .{ .deliver = raw } else .skip;
    }
    if (!self.ready or !shared.same(message, "type", "quote-event") or !shared.same(message, "channel", self.selector)) return .skip;
    const data = p.get(content, "data");
    if (data != .array) return error.InvalidResponse;
    for (data.array.items) |row| if (!shared.sameScalar(row, "contractId", self.market.?)) return .skip;
    if (shared.same(content, "dataType", "Snapshot")) self.snapshot = true;
    return if (self.snapshot or !self.book) .{ .deliver = raw } else .skip;
}
