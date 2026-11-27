const std = @import("std");
const w = @import("../../core/wire.zig");
const p = @import("../../core/perps.zig");
const connectors = @import("../../core/perps_connectors.zig");
const shared = @import("../../core/perps_stream.zig");
const Event = shared.Event;
const eq = std.mem.eql;
const gpa = std.heap.c_allocator;

pub fn publicEndpoint(testnet: bool) []const u8 {
    return if (testnet) "wss://testnet.perpl.xyz/ws/v1/market-data" else "wss://app.perpl.xyz/ws/v1/market-data";
}

pub fn publicSelector(a: p.A, params: p.V) ![]const u8 {
    const channel = try p.text(params, "channel");
    if (eq(u8, channel, "account")) return error.AuthenticationRequired;
    const name = if (eq(u8, channel, "order_book")) "order-book" else if (eq(u8, channel, "trades")) "trades" else return error.InvalidSubscription;
    return std.fmt.allocPrint(a, "{s}@{d}", .{ name, try w.unsigned(u32, try w.field(params, "market_id")) });
}

pub const Subscription = struct { stream: []const u8, subscribe: bool };

pub fn subscriptionFrame(a: p.A, subs: []const Subscription) ![]const u8 {
    return std.json.Stringify.valueAlloc(a, .{ .mt = 5, .subs = subs }, .{});
}

pub fn init(self: *shared.State, a: p.A, testnet: bool, params: p.V) !void {
    const channel = self.channel;
    self.private = eq(u8, channel, "account");
    self.book = eq(u8, channel, "order_book");
    if (self.private) {
        self.endpoint = if (testnet) "wss://testnet.perpl.xyz/ws/v1/trading" else "wss://app.perpl.xyz/ws/v1/trading";
        return;
    }
    self.endpoint = publicEndpoint(testnet);
    self.selector = try publicSelector(a, params);
    self.request = try subscriptionFrame(a, &.{.{ .stream = self.selector, .subscribe = true }});
}

pub fn event(self: *shared.State, message: p.V, raw: []const u8) !Event {
    const kind = try w.unsigned(u16, try w.field(message, "mt"));
    if (kind == 3) {
        const code = p.get(p.get(message, "status"), "code");
        const order_status = p.get(message, "st");
        if ((code != .null and try w.unsigned(u32, code) != 0) or (self.private and !self.ready and order_status != .null and try w.unsigned(u32, order_status) != 0)) return error.ExchangeApi;
        return .skip;
    }
    if (kind == 100) {
        const sn = try w.unsigned(u64, try w.field(message, "sn"));
        if (self.sequence) |previous| if (sn != previous +| 1) return error.SequenceGap;
        self.sequence = sn;
        return .skip;
    }
    if (self.private) {
        if (kind == 19) {
            self.ready = true;
            self.sequence = try w.unsigned(u64, try w.field(message, "sn"));
        }
        return if (self.ready and kind >= 19 and kind <= 28 and kind != 22) .{ .deliver = raw } else .skip;
    }
    if (kind == 6) {
        const subs = try w.field(message, "subs");
        if (subs != .array) return error.InvalidResponse;
        for (subs.array.items) |sub| {
            if (!shared.same(sub, "stream", self.selector)) continue;
            const code = p.get(p.get(sub, "status"), "code");
            if (code != .null and try w.unsigned(u32, code) != 0) return error.ExchangeApi;
            self.subscription_id = try w.unsigned(u64, try w.field(sub, "sid"));
        }
        return .skip;
    }
    const full: u16 = if (self.book) 15 else 17;
    if (kind != full and kind != full + 1) return .skip;
    const sid = self.subscription_id orelse return .skip;
    if (try w.unsigned(u64, try w.field(message, "sid")) != sid) return .skip;
    if (kind == full) self.snapshot = true;
    return if (self.snapshot or !self.book) .{ .deliver = raw } else .skip;
}
