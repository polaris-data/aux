const std = @import("std");
const w = @import("../../core/wire.zig");
const A = w.A;
const V = w.V;
const E = @import("../derive/stream.zig").State.Event;
const get = @import("../../core/perps.zig").get;
const Channel = enum { order_book, trade, candle, account_all, account_all_orders };
const mainnet_endpoint = "wss://mainnet.zklighter.elliot.ai/stream";
const testnet_endpoint = "wss://testnet.zklighter.elliot.ai/stream";
const eq = std.mem.eql;

fn text(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn integer(v: V, k: []const u8) !i64 {
    const val = try w.field(v, k);
    if (val != .number_string) return error.InvalidNumber;
    return std.fmt.parseInt(i64, val.number_string, 10);
}
fn array(v: V, k: []const u8) ![]const V {
    const val = try w.field(v, k);
    return if (val == .array) val.array.items else error.InvalidArray;
}
fn strings(v: V, keys: []const []const u8) !void {
    for (keys) |k| _ = try text(v, k);
}
fn trade(v: V) !void {
    try strings(v, &.{ "size", "price" });
    for ([_][]const u8{ "trade_id", "market_id", "timestamp" }) |k| _ = try integer(v, k);
    if (try w.field(v, "is_maker_ask") != .bool) return error.InvalidTrade;
}

fn subscriptionChannel(params: V) !Channel {
    return std.meta.stringToEnum(Channel, try text(params, "channel")) orelse error.InvalidChannel;
}

fn subscriptionId(params: V, field: []const u8, account: bool) !i64 {
    const id = try integer(params, field);
    const max = (1 << 48) - 1;
    if (id < 0 or id > max or (account and id == max) or (!account and id == 255)) return error.InvalidId;
    return id;
}

fn topicName(a: A, channel: Channel, id: i64, separator: u8) ![]const u8 {
    if (channel == .candle) return std.fmt.allocPrint(a, "candle{c}{d}{c}1m", .{ separator, id, separator });
    return std.fmt.allocPrint(a, "{s}{c}{d}", .{ @tagName(channel), separator, id });
}

fn subscriptionFrame(a: A, operation: enum { subscribe, unsubscribe }, topic: []const u8, auth: ?V) ![]const u8 {
    return std.json.Stringify.valueAlloc(a, .{ .type = @tagName(operation), .channel = topic, .auth = auth }, .{ .emit_null_optional_fields = false });
}

pub fn preparedEndpoint(config: V) ![]const u8 {
    const chain_id = get(get(config, "auth"), "chain_id");
    return if (chain_id != .null and try w.unsigned(u32, chain_id) == 300) testnet_endpoint else mainnet_endpoint;
}

pub fn prepareFrames(a: A, params: V) !@import("../../core/websocket.zig").SubscriptionFrames {
    const channel = try subscriptionChannel(params);
    if (channel == .account_all or channel == .account_all_orders) return error.InvalidChannel;
    const topic = try topicName(a, channel, try subscriptionId(params, "market_id", false), '/');
    return .{ .subscribe = try subscriptionFrame(a, .subscribe, topic, null), .unsubscribe = try subscriptionFrame(a, .unsubscribe, topic, null) };
}

pub const State = struct {
    channel: Channel,
    topic: []const u8,
    incoming: []const u8,
    frame: []const u8,
    endpoint: []const u8,
    expiry: ?i64 = null,
    snapshot: bool = false,
    pub fn init(a: A, config: V, p: V, io: std.Io) !State {
        const channel = try subscriptionChannel(p);
        const auth = config.object.get("auth") orelse V.null;
        const account = channel == .account_all or channel == .account_all_orders;
        if (channel == .account_all_orders and auth != .object) return error.AuthenticationRequired;
        const id = try subscriptionId(if (channel == .account_all_orders) auth else p, if (account) "account_index" else "market_id", account);
        const topic = try topicName(a, channel, id, '/');
        var deadline: ?i64 = null;
        var token: ?V = null;
        const testnet = auth == .object and try integer(auth, "chain_id") == 300;
        const endpoint = if (testnet) testnet_endpoint else mainnet_endpoint;
        if (channel == .account_all_orders) {
            deadline = std.Io.Timestamp.now(io, .real).toSeconds() + 600;
            const url = if (config.object.get("origin")) |v| try w.string(v) else endpoint;
            var uri = try std.Uri.parse(url);
            uri.scheme = if (std.mem.eql(u8, uri.scheme, "wss")) "https" else if (std.mem.eql(u8, uri.scheme, "ws")) "http" else return error.InvalidOrigin;
            uri.query = null;
            uri.path = .{ .raw = "" };
            var cfg = V{ .object = .empty };
            try cfg.object.put(a, "auth", auth);
            try cfg.object.put(a, "origin", w.text(try std.fmt.allocPrint(a, "{f}", .{uri})));
            var params = V{ .object = .empty };
            try params.object.put(a, "deadline", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{deadline.?}) });
            try cfg.object.put(a, "params", params);
            token = try @import("trading.zig").execute(a, cfg, "auth_token", null);
        }
        return .{
            .channel = channel,
            .topic = topic,
            .incoming = try topicName(a, channel, id, ':'),
            .frame = try subscriptionFrame(a, .subscribe, topic, token),
            .endpoint = endpoint,
            .expiry = deadline,
        };
    }
    pub fn unsubscribeFrame(self: State, a: A) ![]const u8 {
        return subscriptionFrame(a, .unsubscribe, self.topic, null);
    }
    pub fn start(self: *State, io: std.Io) ![]const u8 {
        if (self.expiry) |deadline| if (std.Io.Timestamp.now(io, .real).toSeconds() >= deadline) return error.AuthenticationRequired;
        self.snapshot = false;
        return self.frame;
    }
    pub fn event(self: *State, a: A, raw: []const u8) !E {
        const v = try w.parse(a, raw);
        if (v != .object) return error.InvalidEvent;
        if (v.object.contains("error")) return error.ExchangeApi;
        const t = try text(v, "type");
        if (std.mem.eql(u8, t, "ping")) return .{ .reply = "{\"type\":\"pong\"}" };
        if (std.mem.eql(u8, t, "pong") or std.mem.eql(u8, t, "connected")) return .skip;
        if (self.channel == .candle) {
            if (!eq(u8, t, "subscribed/candle") and !eq(u8, t, "update/candle")) return .skip;
            const ch = text(v, "channel") catch return .skip;
            return if (eq(u8, ch, self.incoming) or eq(u8, ch, self.topic)) .{ .deliver = raw } else .skip;
        }
        if (v.object.get("channel")) |ch| if (!std.mem.eql(u8, try w.string(ch), self.incoming)) return .skip;
        const split = std.mem.findScalar(u8, t, '/') orelse return .{ .deliver = raw };
        const snapshot = std.mem.eql(u8, t[0..split], "subscribed");
        if (!snapshot and !std.mem.eql(u8, t[0..split], "update")) return .{ .deliver = raw };
        const kind = std.meta.stringToEnum(Channel, t[split + 1 ..]) orelse return .{ .deliver = raw };
        if (kind != self.channel or (!snapshot and !self.snapshot)) return .skip;
        _ = try text(v, "channel");
        switch (kind) {
            .candle => unreachable,
            .order_book => {
                const book = try w.field(v, "order_book");
                _ = try integer(book, "offset");
                _ = try integer(book, "nonce");
                for ([_][]const u8{ "asks", "bids" }) |k| for (try array(book, k)) |x| try strings(x, &.{ "price", "size" });
            },
            .trade => {
                for (try array(v, "trades")) |x| try trade(x);
                if (v.object.contains("liquidation_trades")) for (try array(v, "liquidation_trades")) |x| try trade(x);
            },
            .account_all => {
                for ([_][]const u8{ "positions", "assets", "trades" }) |k| if (try w.field(v, k) != .object) return error.InvalidAccount;
            },
            .account_all_orders => {
                const orders = try w.field(v, "orders");
                if (orders != .object) return error.InvalidOrders;
                for (orders.object.values()) |list| {
                    if (list != .array) return error.InvalidOrders;
                    for (list.array.items) |x| {
                        try strings(x, &.{ "price", "remaining_base_amount", "status" });
                        for ([_][]const u8{ "order_index", "client_order_index", "market_index", "owner_account_index" }) |k| _ = try integer(x, k);
                        if (try w.field(x, "is_ask") != .bool) return error.InvalidOrder;
                    }
                }
            },
        }
        self.snapshot = true;
        return .{ .deliver = raw };
    }
};
