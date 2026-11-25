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
    const index = try connectors.enumIndex(channel, &.{ "v1.book.s", "v1.trade", "v1.order", "v1.fill", "v1.position" });
    self.private = index >= 2;
    const instrument = p.get(params, "instrument");
    if (instrument != .null) {
        self.market = try w.string(instrument);
        if (self.market.?.len == 0 or std.mem.findAny(u8, self.market.?, "@-") != null) return error.InvalidMarket;
    }
    if (self.private) {
        const id = try w.unsigned(u64, try w.field(params, "sub_account_id"));
        self.account = try std.fmt.allocPrint(a, "{d}", .{id});
        self.selector = if (self.market) |market| try std.fmt.allocPrint(a, "{d}-{s}", .{ id, market }) else self.account.?;
        self.endpoint = if (testnet) "wss://trades.testnet.grvt.io/ws/full" else "wss://trades.grvt.io/ws/full";
    } else {
        const market = self.market orelse return error.InvalidMarket;
        const rate = try shared.optional(u16, params, "rate", 500);
        if (index == 0) {
            const depth = try shared.optional(u16, params, "depth", 50);
            if (rate != 200 and rate != 500 and rate != 1000 and rate != 5000) return error.InvalidSubscription;
            if (depth != 10 and depth != 50 and depth != 100 and depth != 500) return error.InvalidSubscription;
            self.selector = try std.fmt.allocPrint(a, "{s}@{d}-{d}", .{ market, rate, depth });
        } else {
            self.selector = try std.fmt.allocPrint(a, "{s}@{d}", .{ market, rate });
        }
        self.endpoint = if (testnet) "wss://market-data.testnet.grvt.io/ws/full" else "wss://market-data.grvt.io/ws/full";
    }
    self.request = try std.json.Stringify.valueAlloc(a, .{ .jsonrpc = "2.0", .method = "subscribe", .params = .{ .stream = channel, .selectors = .{self.selector} }, .id = 1 }, .{});
}

pub fn event(self: *shared.State, message: p.V, raw: []const u8) !Event {
    if (p.get(message, "code") != .null) return error.ExchangeApi;
    if (!shared.same(message, "stream", self.channel)) return .skip;
    const feed = p.get(message, "feed");
    if (feed == .null or !shared.same(message, "selector", self.selector)) return .skip;
    if (self.market) |name| {
        const instrument = p.get(feed, "instrument");
        if (instrument != .null and !shared.same(feed, "instrument", name)) return .skip;
    }
    if (self.account) |account| {
        const field = p.get(feed, "sub_account_id");
        if (field != .null and !shared.sameScalar(feed, "sub_account_id", account)) return .skip;
    }
    const next = try w.unsigned(u64, try w.field(message, "sequence_number"));
    if (self.sequence) |previous| {
        if (previous == next and next != 0) return .skip;
        if (next != 0 and next != previous +| 1) return error.SequenceGap;
    }
    self.sequence = next;
    return .{ .deliver = raw };
}
