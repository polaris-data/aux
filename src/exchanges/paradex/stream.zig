const std = @import("std");
const w = @import("../../core/wire.zig");
const p = @import("../../core/perps.zig");
const websocket = @import("../../core/websocket.zig");
const Event = @import("../derive/stream.zig").State.Event;
const eq = std.mem.eql;
const Kind = enum { account, balance_events, bbo, block_trades, fills, funding_data, funding_payments, funding_rate_comparison, markets, markets_summary, mmp, order_book, orders, positions, rfq, tradebusts, trades, transaction, transfers };
const mainnet_endpoint = "wss://ws.api.prod.paradex.trade/v1";
const testnet_endpoint = "wss://ws.api.testnet.paradex.trade/v1";
pub const endpoint = mainnet_endpoint ++ "?sbeSchemaId=1&sbeSchemaVersion=3";

pub fn subscriptionFrame(a: w.A, method: []const u8, id: u64, channel: []const u8) ![]const u8 {
    return std.json.Stringify.valueAlloc(a, .{ .jsonrpc = "2.0", .id = id, .method = method, .params = .{ .channel = channel } }, .{});
}

pub fn preparedEndpoint(a: w.A, config: w.V, spec: w.V) ![]const u8 {
    const override = p.get(config, "origin");
    const testnet = try p.testnet(config);
    const version = p.get(spec, "sbe_version");
    const url = if (override != .null) try w.string(override) else if (testnet) testnet_endpoint else mainnet_endpoint;
    _ = try websocket.Session.parseEndpoint(url);
    if (override != .null and version == .null) return url;
    const number = if (version == .null) 3 else try w.unsigned(u16, version);
    if (number == 0) return error.UnsupportedSchema;
    const path_end = std.mem.findScalar(u8, url, '?') orelse url.len;
    var result: std.ArrayList(u8) = .empty;
    try result.appendSlice(a, url[0..path_end]);
    try result.append(a, '?');
    if (path_end < url.len) {
        var query = std.mem.splitScalar(u8, url[path_end + 1 ..], '&');
        while (query.next()) |part| {
            const key = part[0 .. std.mem.findScalar(u8, part, '=') orelse part.len];
            if (part.len == 0 or oneOf(key, &.{ "sbeSchemaId", "sbeSchemaVersion" })) continue;
            try result.appendSlice(a, part);
            try result.append(a, '&');
        }
    }
    try result.print(a, "sbeSchemaId=1&sbeSchemaVersion={d}", .{number});
    return result.toOwnedSlice(a);
}

pub fn prepareFrames(a: w.A, params: w.V, subscribe_id: u64, unsubscribe_id: u64) !websocket.SubscriptionFrames {
    const topic = p.get(params, "topic");
    const name = if (topic == .null) blk: {
        const state = try State.initRpc(a, params);
        if (state.requires_auth) return error.AuthenticationRequired;
        break :blk state.name;
    } else blk: {
        const native = try w.string(topic);
        const base = native[0 .. std.mem.findAny(u8, native, ".@") orelse native.len];
        if (!oneOf(base, &.{ "bbo", "funding_data", "funding_rate_comparison", "markets", "markets_summary", "order_book", "trades" }) or
            std.mem.findAny(u8, native, &std.ascii.whitespace) != null) return error.InvalidChannel;
        break :blk native;
    };
    return .{
        .subscribe = try subscriptionFrame(a, "subscribe", subscribe_id, name),
        .unsubscribe = try subscriptionFrame(a, "unsubscribe", unsubscribe_id, name),
    };
}

fn field(v: w.V, name: []const u8) ![]const u8 {
    return w.string(try w.field(v, name));
}
fn component(v: w.V, name: []const u8) ![]const u8 {
    const s = try field(v, name);
    if (s.len == 0) return error.InvalidChannel;
    for (s) |c| if (!std.ascii.isAlphanumeric(c) and c != '-' and c != '_') return error.InvalidChannel;
    return s;
}
fn oneOf(s: []const u8, options: []const []const u8) bool {
    for (options) |option| if (eq(u8, s, option)) return true;
    return false;
}
fn felt(s: []const u8) !u256 {
    const hex = std.mem.startsWith(u8, s, "0x");
    const digits = if (hex) s[2..] else s;
    if (digits.len == 0) return error.InvalidAccount;
    for (digits) |c| if (!(if (hex) std.ascii.isHex(c) else std.ascii.isDigit(c))) return error.InvalidAccount;
    const x = try std.fmt.parseInt(u256, digits, if (hex) 16 else 10);
    if (x >= (@as(u256, 1) << 251) + (@as(u256, 17) << 192) + 1) return error.InvalidAccount;
    return x;
}

pub const State = struct {
    name: []const u8,
    kind: Kind,
    market: ?[]const u8,
    auth: w.V = .null,
    account: ?u256 = null,
    requires_auth: bool = false,
    awaiting_login: bool = false,
    awaiting_snapshot: bool = false,

    pub fn init(a: w.A, params: w.V, auth: w.V) !State {
        return initChannel(a, params, auth, false);
    }
    pub fn initRpc(a: w.A, params: w.V) !State {
        return initChannel(a, params, .null, true);
    }
    fn initChannel(a: w.A, params: w.V, auth: w.V, rpc: bool) !State {
        const kind = std.meta.stringToEnum(Kind, try field(params, "type")) orelse return error.InvalidChannel;
        const base = @tagName(kind);
        const market = if ((params.object.get("market") orelse w.V.null) != .null) try component(params, "market") else null;
        const name = switch (kind) {
            .bbo, .trades, .fills, .orders, .funding_data, .funding_payments => try std.fmt.allocPrint(a, "{s}.{s}", .{ base, market orelse return error.InvalidChannel }),
            .markets, .markets_summary => if (market) |m| try std.fmt.allocPrint(a, "{s}.{s}", .{ base, m }) else base,
            .rfq => if ((params.object.get("rfq_id") orelse w.V.null) != .null) try std.fmt.allocPrint(a, "rfq.{s}", .{try component(params, "rfq_id")}) else base,
            .funding_rate_comparison => blk: {
                const rate = try field(params, "refresh_rate");
                if (!oneOf(rate, &.{ "500ms", "1000ms" })) return error.InvalidChannel;
                break :blk try std.fmt.allocPrint(a, "funding_rate_comparison.ALL@{s}", .{rate});
            },
            .order_book => blk: {
                const feed = try field(params, "feed_type");
                const rate = try field(params, "refresh_rate");
                if (!oneOf(feed, &.{ "snapshot", "deltas", "interactive", "interactive_deltas" }) or !oneOf(rate, &.{ "50ms", "100ms", "200ms", "500ms", "1000ms" })) return error.InvalidChannel;
                const prefix = try std.fmt.allocPrint(a, "order_book.{s}.{s}@15@{s}", .{ market orelse return error.InvalidChannel, feed, rate });
                const tick = params.object.get("price_tick") orelse .null;
                if (tick == .null) break :blk prefix;
                const s = try w.string(tick);
                if (s.len == 0) return error.InvalidChannel;
                for (s) |c| if (c != '.' and !std.ascii.isDigit(c)) return error.InvalidChannel;
                break :blk try std.mem.concat(a, u8, &.{ prefix, "@", s });
            },
            else => base,
        };
        var state = State{ .kind = kind, .name = name, .market = market };
        switch (kind) {
            .account, .balance_events, .block_trades, .fills, .funding_payments, .mmp, .orders, .positions, .rfq, .tradebusts, .transaction, .transfers => {
                if (rpc) {
                    state.requires_auth = true;
                    return state;
                }
                if (auth != .object) return error.AuthenticationRequired;
                state.account = try felt(try w.string(auth.object.get("account") orelse return error.AuthenticationRequired));
                state.auth = auth;
            },
            else => {},
        }
        return state;
    }
    pub fn start(self: *State, a: w.A, io: std.Io, subscribe: []const u8) ![]const u8 {
        self.awaiting_snapshot = self.kind == .order_book;
        self.awaiting_login = self.auth != .null;
        if (!self.awaiting_login) return subscribe;
        const token = try @import("../../core/venue_auth.zig").bearer(a, self.auth, std.Io.Timestamp.now(io, .real).toSeconds());
        return std.json.Stringify.valueAlloc(a, .{ .jsonrpc = "2.0", .id = "aux-auth", .method = "auth", .params = .{ .bearer = token } }, .{});
    }

    fn snapshot(self: *State, data: w.V) bool {
        if (!self.awaiting_snapshot) return true;
        if (data != .object) return false;
        for ([_][]const u8{ "pkgType", "update_type" }) |key| {
            const v = data.object.get(key) orelse continue;
            if (v == .string and (eq(u8, v.string, "SNAPSHOT") or (eq(u8, key, "update_type") and eq(u8, v.string, "s")))) {
                self.awaiting_snapshot = false;
                return true;
            }
        }
        return false;
    }
    pub fn event(self: *State, a: w.A, raw: []const u8, binary: bool) !Event {
        if (binary) {
            if (self.awaiting_login) return .skip;
            const frame = try @import("sbe.zig").decodeBytes(a, raw);
            const id = try std.fmt.parseInt(u16, try field(frame, "template"), 10);
            const data = try w.field(frame, "data");
            if (id == 40 or id == 41) return .{ .deliver = try std.json.Stringify.valueAlloc(a, .{ .method = if (id == 40) @as([]const u8, "heartbeat") else "subscribed", .sbe = frame }, .{}) };
            const expected: Kind = switch (id) {
                1 => .trades,
                2 => .bbo,
                3 => .order_book,
                4 => .markets_summary,
                5 => .funding_data,
                6 => .funding_rate_comparison,
                20 => .orders,
                21 => .fills,
                22 => .positions,
                23 => .account,
                else => return error.InvalidEvent,
            };
            if (self.kind != expected) return error.InvalidChannel;
            if (self.account) |account| if (data.object.get("account")) |v| if (try felt(try w.string(v)) != account) return error.InvalidAccount;
            if (self.market) |market| if (data.object.get("market")) |v| if (!eq(u8, market, "ALL") and !eq(u8, market, try w.string(v))) return error.InvalidChannel;
            if (!self.snapshot(data)) return .skip;
            return .{ .deliver = try std.json.Stringify.valueAlloc(a, .{ .jsonrpc = "2.0", .method = "subscription", .params = .{ .channel = self.name, .data = data }, .sbe = frame }, .{}) };
        }
        const v = try w.parse(a, raw);
        if (v != .object) return error.InvalidEvent;
        const id = v.object.get("id");
        const result = v.object.get("result");
        const failed = (v.object.get("error") orelse w.V.null) != .null;
        if (self.awaiting_login) {
            if (id == null or id.? != .string or !eq(u8, id.?.string, "aux-auth")) return .skip;
            if (failed or result == null) return error.AuthenticationRequired;
            self.awaiting_login = false;
            return .subscribe;
        }
        if (id) |n| if (n != .number_string or !eq(u8, n.number_string, "1")) return .skip;
        if (failed) return error.ExchangeApi;
        if (result == null) {
            const method = v.object.get("method") orelse w.V.null;
            if (method != .string or !oneOf(method.string, &.{ "heartbeat", "subscribed" })) {
                const params = try w.field(v, "params");
                if (!eq(u8, try field(params, "channel"), self.name)) return .skip;
                if (!self.snapshot(try w.field(params, "data"))) return .skip;
            }
        }
        return .{ .deliver = raw };
    }
};

test "private login must succeed before subscription" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const params = try w.parse(a, "{\"type\":\"account\"}");
    try std.testing.expectError(error.AuthenticationRequired, State.init(a, params, .null));
    var state = try State.init(a, params, try w.parse(a, "{\"account\":\"42\",\"token\":\"test.eyJleHAiOjQxMDI0NDQ4MDB9.test\"}"));
    var io: std.Io.Threaded = .init_single_threaded;
    _ = try state.start(a, io.io(), "subscribe");
    try std.testing.expectEqual(.skip, try state.event(a, "{\"id\":1,\"result\":true}", false));
    try std.testing.expectError(error.AuthenticationRequired, state.event(a, "{\"id\":\"aux-auth\",\"error\":{\"code\":401}}", false));
}
