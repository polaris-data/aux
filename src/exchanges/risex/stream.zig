const std = @import("std");
const w = @import("../../core/wire.zig");
const p = @import("../../core/perps.zig");
const shared = @import("../../core/perps_stream.zig");
const Event = shared.Event;
const eq = std.mem.eql;
const gpa = std.heap.c_allocator;

const Channel = enum { orderbook, trades, oracle, orders, positions, fills, account };

pub fn endpoint(testnet: bool) []const u8 {
    return if (testnet) "wss://api.testnet.rise.trade/ws/" else "wss://ws.rise.trade/ws";
}

pub fn marketIds(a: p.A, params: p.V) ![]const u64 {
    const filters = p.get(params, "market_ids");
    if (filters == .null) return &.{};
    if (filters != .array or filters.array.items.len > 4096) return error.InvalidSubscription;
    const ids = try a.alloc(u64, filters.array.items.len);
    errdefer a.free(ids);
    for (filters.array.items, ids) |id, *dest| dest.* = try w.unsigned(u64, id);
    return ids;
}

pub fn subscriptionFrame(a: p.A, subscribe: bool, channel: []const u8, ids: ?[]const u64) ![]const u8 {
    const method: []const u8 = if (subscribe) "subscribe" else "unsubscribe";
    return std.json.Stringify.valueAlloc(a, .{ .method = method, .params = .{ .channel = channel, .market_ids = ids } }, .{ .emit_null_optional_fields = false });
}

pub fn init(self: *shared.State, a: p.A, testnet: bool, params: p.V) !void {
    const channel = std.meta.stringToEnum(Channel, self.channel) orelse return error.InvalidParameter;
    self.private = switch (channel) {
        .orders, .positions, .fills, .account => true,
        else => false,
    };
    self.book = channel == .orderbook;
    self.endpoint = endpoint(testnet);
    if (channel == .account and p.get(params, "market_ids") != .null) return error.InvalidSubscription;
    self.ids = try marketIds(a, params);
    if (self.private) self.account = try p.credential(p.get(p.get(self.config, "auth"), "account"));
    self.request = try subscriptionFrame(a, true, self.channel, if (channel == .account) null else self.ids);
}

pub fn event(self: *shared.State, message: p.V, raw: []const u8) !Event {
    const method = p.get(message, "method");
    if (shared.same(message, "status", "error")) return error.ExchangeApi;
    if (method == .string and eq(u8, method.string, "auth_v2")) {
        if (!self.private or !shared.same(message, "status", "success")) return error.ExchangeApi;
        const account = try p.text(try w.field(message, "data"), "account");
        if (!std.ascii.eqlIgnoreCase(account, self.account.?)) return error.InvalidIdentity;
        self.ready = true;
        return .subscribe;
    }
    if (!shared.same(message, "channel", self.channel) or (self.private and !self.ready)) return .skip;
    const snapshot = shared.same(message, "type", "snapshot");
    if (!snapshot and !shared.same(message, "type", "update")) return .skip;
    const market = p.get(message, "market_id");
    if (market != .null) {
        const id = try w.unsigned(u64, market);
        if (self.ids.len != 0 and std.mem.findScalar(u64, self.ids, id) == null) return .skip;
        if (self.book) {
            const nested = p.get(p.get(message, "data"), "market_id");
            if (nested != .null and try w.unsigned(u64, nested) != id) return .skip;
            if (snapshot) {
                if (self.rise_books.count() >= 4096 and !self.rise_books.contains(id)) return error.InvalidSubscription;
                try self.rise_books.put(gpa, id, {});
            } else if (!self.rise_books.contains(id)) return .skip;
        }
    } else if (self.book or eq(u8, self.channel, "trades")) return .skip;
    return .{ .deliver = raw };
}
