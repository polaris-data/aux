// Extended path-based and JSON-RPC market and account subscriptions.
const std = @import("std");
const wire = @import("wire.zig");
const http = @import("http.zig");
const A = wire.A;
const V = wire.V;
const equal = std.mem.eql;
const Channel = enum {
    orderbooks,
    rfq_orderbooks,
    trades,
    funding,
    candles,
    mark_price,
    index_price,
    account,

    fn book(self: Channel) bool {
        return self == .orderbooks or self == .rfq_orderbooks;
    }
    fn path(self: Channel) []const u8 {
        return switch (self) {
            .orderbooks, .funding, .candles, .account => @tagName(self),
            .rfq_orderbooks => "orderbooks/rfq",
            .trades => "publicTrades",
            .mark_price => "prices/mark",
            .index_price => "prices/index",
        };
    }
    fn scope(self: Channel) []const u8 {
        return switch (self) {
            .orderbooks, .rfq_orderbooks => "orderbooks",
            .mark_price, .index_price => "prices",
            .funding => "funding-rates",
            else => @tagName(self),
        };
    }
};
fn get(v: V, key: []const u8) V {
    return if (v == .object) v.object.get(key) orelse .null else .null;
}
fn str(v: V, fallback: []const u8) []const u8 {
    return if (v == .string) v.string else fallback;
}
fn is(v: V, value: []const u8) bool {
    return v == .string and equal(u8, v.string, value);
}
pub const State = struct {
    endpoint: []const u8,
    frame: []const u8 = "",
    topic: ?[]const u8 = null,
    book: bool,
    snapshot: bool = false,
    sequence: ?u64 = null,

    pub fn init(a: A, args: V, sub: V, authenticated: bool) !State {
        const channel = std.meta.stringToEnum(Channel, try wire.string(try wire.field(sub, "channel"))) orelse return error.UnknownChannel;
        if (channel == .account and !authenticated) return error.AuthenticationRequired;
        const origin = switch (get(args, "origin")) {
            .null => "wss://api.starknet.extended.exchange/stream.extended.exchange/v1",
            .string => |s| s,
            else => return error.InvalidEndpoint,
        };
        const root = std.mem.trimEnd(u8, origin, "/");
        const market = switch (get(sub, "market")) {
            .null => null,
            .string => |s| s,
            else => return error.InvalidMarket,
        };
        if (market) |m| if (m.len == 0 or equal(u8, m, ".") or equal(u8, m, "..") or std.mem.findAny(u8, m, "/\\") != null) return error.InvalidMarket;
        if (channel == .candles and market == null) return error.InvalidMarket;
        var params = get(sub, "params");
        if (params == .null) params = .{ .object = .empty };
        if (params != .object) return error.InvalidParameter;
        const rpc = switch (get(sub, "rpc")) {
            .null => false,
            .bool => |flag| flag,
            else => return error.InvalidParameter,
        };
        if (!rpc) {
            var path: std.ArrayList(u8) = .empty;
            try path.appendSlice(a, root);
            try path.append(a, '/');
            try path.appendSlice(a, channel.path());
            if (market) |m| {
                try path.append(a, '/');
                try path.appendSlice(a, try http.escape(a, m, false));
            }
            if (channel == .candles) {
                const kind = get(params, "candle_type");
                if (!(is(kind, "trades") or is(kind, "mark-prices") or is(kind, "index-prices"))) return error.InvalidCandleType;
                try path.append(a, '/');
                try path.appendSlice(a, kind.string);
            }
            var query: std.ArrayList(u8) = .empty;
            var it = params.object.iterator();
            while (it.next()) |entry| {
                if (!equal(u8, entry.key_ptr.*, "candle_type")) try http.pair(a, &query, entry.key_ptr.*, try http.scalar(entry.value_ptr.*), true);
            }
            try path.appendSlice(a, query.items);
            return .{ .endpoint = path.items, .book = channel.book() };
        }
        var selector = params.object.get("selector") orelse V{ .object = .empty };
        if (selector != .object) return error.InvalidParameter;
        if (market) |m| try selector.object.put(a, "market", wire.text(m));
        if (channel.book()) {
            if (!selector.object.contains("depth")) try selector.object.put(a, "depth", wire.text("full"));
            try selector.object.put(a, "rfqOnly", .{ .bool = channel == .rfq_orderbooks });
        }
        if (channel == .mark_price or channel == .index_price) try selector.object.put(a, "type", wire.text(if (channel == .mark_price) "mark" else "index"));
        const m = str(get(selector, "market"), "all");
        const topic = switch (channel) {
            .orderbooks, .rfq_orderbooks => try std.mem.concat(a, u8, &.{ "orderbooks.", if (is(get(selector, "depth"), "1")) "1." else "", m, if (channel == .rfq_orderbooks) ".rfq" else "" }),
            .candles => try std.mem.concat(a, u8, &.{ "candles.", str(get(selector, "type"), "last"), ".", m, ".", str(get(selector, "interval"), "") }),
            .mark_price, .index_price => try std.mem.concat(a, u8, &.{ "prices.", str(get(selector, "type"), ""), ".", m }),
            .account => try std.mem.concat(a, u8, &.{ "account.", try wire.string(try wire.field(selector, "account")) }),
            else => try std.mem.concat(a, u8, &.{ channel.scope(), ".", m }),
        };
        try params.object.put(a, "selector", selector);
        try params.object.put(a, "scope", wire.text(channel.scope()));
        if (channel == .account) if (args.object.get("api_key")) |key| try params.object.put(a, "apiKey", key);
        const rpc_root = if (std.mem.endsWith(u8, root, "/v1")) root[0 .. root.len - 3] else root;
        return .{ .topic = topic, .book = channel.book(), .endpoint = try std.mem.concat(a, u8, &.{ rpc_root, "/v2/rpc" }), .frame = try std.json.Stringify.valueAlloc(a, .{ .jsonrpc = "2.0", .id = "1", .method = "subscribe", .params = params }, .{}) };
    }
    pub fn process(self: *State, a: A, raw: []const u8) !?[]const u8 {
        const v = try wire.parse(a, raw);
        if (v != .object) return error.InvalidEvent;
        if (self.topic) |topic| {
            const id = get(v, "id");
            if (id != .null) {
                if (!(is(id, "1") or (id == .number_string and equal(u8, id.number_string, "1")) or is(id, "ping-1") or is(id, "close-1"))) return null;
            } else if (get(v, "subscription") != .null) {
                if (!is(get(v, "subscription"), topic)) return null;
            } else if (get(v, "error") == .null and !is(get(v, "type"), "ERROR")) return null;
        }
        const kind = get(v, "type");
        if (get(v, "error") != .null or is(kind, "ERROR")) return error.ExchangeApi;
        const data = get(v, "data");
        if (self.book and !self.snapshot and data != .null) {
            const flag = get(data, "isSnapshot");
            if (!(is(kind, "SNAPSHOT") or is(kind, "ORDERBOOK.SNAPSHOT") or is(kind, "ORDERBOOKS.SNAPSHOT") or is(get(data, "t"), "SNAPSHOT") or (flag == .bool and flag.bool))) return null;
            self.snapshot = true;
        }
        if (get(v, "seq") != .null) {
            const seq = try wire.unsigned(u64, get(v, "seq"));
            if (self.sequence) |prev| if (prev == std.math.maxInt(u64) or seq != prev + 1) return error.SequenceGap;
            self.sequence = seq;
        }
        return raw;
    }
};
