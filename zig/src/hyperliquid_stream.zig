const std = @import("std");
const w = @import("wire.zig");
const A = w.A;
const V = w.V;
const E = @import("derive_stream.zig").State.Event;
const encode = std.json.Stringify.valueAlloc;
const Channel = enum { l2Book, trades, orderUpdates, userFills, clearinghouseState, openOrders };

fn text(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn optional(v: V, k: []const u8) V {
    return if (v == .object) v.object.get(k) orelse .null else .null;
}
fn array(v: V) ![]const V {
    return if (v == .array) v.array.items else error.ExpectedArray;
}
fn number(v: V, k: []const u8) !u64 {
    const x = try w.field(v, k);
    if (x != .number_string) return error.InvalidNumber;
    return w.unsigned(u64, x);
}
fn strings(v: V, fields: []const []const u8) !void {
    for (fields) |k| _ = try text(v, k);
}
fn order(v: V) !void {
    try strings(v, &.{ "coin", "side", "sz", "limitPx" });
    _ = try number(v, "oid");
    _ = try number(v, "timestamp");
}
fn address(a: A, s: []const u8) ![]const u8 {
    return std.fmt.allocPrint(a, "0x{x}", .{try @import("evm.zig").hex(20, s)});
}
pub const State = struct {
    channel: Channel,
    sub: V,
    frame: []const u8,
    endpoint: []const u8,
    pub fn init(a: A, config: V, p: V) !State {
        const channel = std.meta.stringToEnum(Channel, try text(p, "type")) orelse return error.InvalidSubscription;
        var sub = V{ .object = .empty };
        try sub.object.put(a, "type", w.text(@tagName(channel)));
        switch (channel) {
            .trades, .l2Book => {
                const coin = try text(p, "coin");
                if (std.mem.trim(u8, coin, " \r\t\n").len == 0) return error.InvalidCoin;
                try sub.object.put(a, "coin", w.text(coin));
            },
            else => try sub.object.put(a, "user", w.text(try address(a, try text(p, "user")))),
        }
        if (channel == .userFills and optional(p, "aggregateByTime") != .null) {
            const flag = try w.field(p, "aggregateByTime");
            if (flag != .bool) return error.InvalidParameter;
            try sub.object.put(a, "aggregateByTime", flag);
        }
        if ((channel == .clearinghouseState or channel == .openOrders) and optional(p, "dex") != .null) try sub.object.put(a, "dex", w.text(try text(p, "dex")));
        const auth = optional(config, "auth");
        const testnet = optional(auth, "network");
        return .{ .channel = channel, .sub = sub, .frame = try encode(a, .{ .method = "subscribe", .subscription = sub }, .{}), .endpoint = if (testnet == .string and std.mem.eql(u8, testnet.string, "Testnet")) "wss://api.hyperliquid-testnet.xyz/ws" else "wss://api.hyperliquid.xyz/ws" };
    }
    fn matches(self: *State, a: A, p: V) !bool {
        const theirs = try init(a, .null, p);
        if (theirs.channel != self.channel) return false;
        for ([_][]const u8{ "user", "coin", "dex" }) |k| {
            const x = optional(self.sub, k);
            const y = optional(theirs.sub, k);
            if (!std.mem.eql(u8, if (x == .string) x.string else "", if (y == .string) y.string else "")) return false;
        }
        const x = optional(self.sub, "aggregateByTime");
        const y = optional(theirs.sub, "aggregateByTime");
        return (x == .bool and x.bool) == (y == .bool and y.bool);
    }
    pub fn event(self: *State, a: A, raw: []const u8) !E {
        const v = try w.parse(a, raw);
        const tag = try text(v, "channel");
        if (std.mem.eql(u8, tag, "error")) return error.ExchangeApi;
        if (std.mem.eql(u8, tag, "pong")) return .skip;
        const data = optional(v, "data");
        if (std.mem.eql(u8, tag, "subscriptionResponse")) {
            _ = try text(data, "method");
            return if (try self.matches(a, try w.field(data, "subscription"))) .{ .deliver = raw } else .skip;
        }
        const channel = std.meta.stringToEnum(Channel, tag) orelse return .{ .deliver = raw };
        if (channel != self.channel) return .skip;
        if (optional(self.sub, "user") == .string and channel != .orderUpdates) {
            if (!std.mem.eql(u8, try address(a, try text(data, "user")), self.sub.object.get("user").?.string)) return .skip;
        }
        switch (channel) {
            .l2Book => {
                if (!std.mem.eql(u8, try text(data, "coin"), try text(self.sub, "coin"))) return .skip;
                _ = try number(data, "time");
                const sides = try array(try w.field(data, "levels"));
                if (sides.len != 2) return error.InvalidBook;
                for (sides) |side| for (try array(side)) |level| {
                    try strings(level, &.{ "px", "sz" });
                    _ = try number(level, "n");
                };
            },
            .trades => for (try array(data)) |trade| {
                if (!std.mem.eql(u8, try text(trade, "coin"), try text(self.sub, "coin"))) return .skip;
                try strings(trade, &.{ "px", "sz", "hash", "side" });
                _ = try number(trade, "time");
                _ = try number(trade, "tid");
                const users = try array(try w.field(trade, "users"));
                if (users.len != 2) return error.InvalidTrade;
                for (users) |u| _ = try w.string(u);
            },
            .orderUpdates => for (try array(data)) |update| {
                try order(try w.field(update, "order"));
                _ = try text(update, "status");
                _ = try number(update, "statusTimestamp");
            },
            .userFills => for (try array(try w.field(data, "fills"))) |fill| {
                try strings(fill, &.{ "coin", "px", "sz", "side", "startPosition", "dir", "closedPnl", "hash", "fee", "feeToken" });
                for ([_][]const u8{ "time", "oid", "tid" }) |k| _ = try number(fill, k);
                if (optional(fill, "crossed") != .bool) return error.InvalidFill;
            },
            .openOrders, .clearinghouseState => {
                const dex = optional(self.sub, "dex");
                if (!std.mem.eql(u8, try text(data, "dex"), if (dex == .string) dex.string else "")) return .skip;
                if (channel == .openOrders) {
                    for (try array(try w.field(data, "orders"))) |o| try order(o);
                } else {
                    const account = try w.field(data, "clearinghouseState");
                    _ = try array(try w.field(account, "assetPositions"));
                    try strings(account, &.{ "crossMaintenanceMarginUsed", "withdrawable" });
                    for ([_][]const u8{ "crossMarginSummary", "marginSummary" }) |k| try strings(try w.field(account, k), &.{ "accountValue", "totalMarginUsed", "totalNtlPos", "totalRawUsd" });
                }
            },
        }
        return .{ .deliver = raw };
    }
};
