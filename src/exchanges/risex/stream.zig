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
    const i = try connectors.enumIndex(channel, &.{ "orderbook", "trades", "orders", "positions", "fills", "account" });
    self.private = i >= 2;
    self.book = i == 0;
    self.endpoint = if (testnet) "wss://api.testnet.rise.trade/ws/" else "wss://ws.rise.trade/ws";
    const filters = p.get(params, "market_ids");
    if (filters != .null) {
        if (filters != .array or filters.array.items.len > 4096 or i == 5) return error.InvalidSubscription;
        const ids = try a.alloc(u64, filters.array.items.len);
        for (filters.array.items, ids) |id, *dest| dest.* = try w.unsigned(u64, id);
        self.ids = ids;
    }
    if (self.private) self.account = try p.credential(p.get(p.get(self.config, "auth"), "account"));
    self.request = try std.json.Stringify.valueAlloc(a, .{ .method = "subscribe", .params = .{ .channel = channel, .market_ids = if (i == 5) null else self.ids } }, .{ .emit_null_optional_fields = false });
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
