const std = @import("std");
const w = @import("../../core/wire.zig");
const Socket = @import("../../core/websocket.zig").Session;
const session = @import("session.zig");
const E = @import("../derive/stream.zig").State.Event;
const A = w.A;
const V = w.V;
const eq = std.mem.eql;
const Kind = enum { trades, orderbook, depth, bbo, balances, positions, orders, account };
fn text(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn get(v: V, k: []const u8) V {
    return if (v == .object) v.object.get(k) orelse .null else .null;
}
fn in(s: []const u8, choices: []const []const u8) bool {
    for (choices) |x| if (eq(u8, x, s)) return true;
    return false;
}
fn endpoint(a: A, base: []const u8) ![]const u8 {
    const uri = try std.Uri.parse(base);
    if (uri.user != null or uri.password != null or uri.query != null or uri.fragment != null or uri.host == null) return error.InvalidEndpoint;
    if (eq(u8, uri.scheme, "ws") or eq(u8, uri.scheme, "wss")) return base;
    const scheme = if (eq(u8, uri.scheme, "https")) "wss" else if (eq(u8, uri.scheme, "http")) "ws" else return error.InvalidEndpoint;
    if (eq(u8, std.mem.trimEnd(u8, base, "/"), "https://apimonaco.xyz")) return "wss://ws.apimonaco.xyz/ws";
    if (eq(u8, std.mem.trimEnd(u8, base, "/"), "https://staging.apimonaco.xyz")) return "wss://ws-staging.apimonaco.xyz/ws";
    return std.mem.concat(a, u8, &.{ scheme, std.mem.trimEnd(u8, base[uri.scheme.len..], "/"), "/ws" });
}
fn sameChannel(pattern: []const u8, actual: []const u8) bool {
    var wanted = std.mem.splitScalar(u8, pattern, ':');
    var got = std.mem.splitScalar(u8, actual, ':');
    while (wanted.next()) |part| {
        const x = got.next() orelse return false;
        if (!eq(u8, part, "*") and !eq(u8, part, x)) return false;
    }
    return true;
}
pub const State = struct {
    kind: Kind,
    channel: []const u8,
    endpoint: []const u8,
    frame: []const u8,
    auth: V = .null,
    lease: ?*session.Lease = null,
    pending: ?[]u8 = null,
    authenticating: bool = false,
    snapshot: bool = false,
    sequence: ?u64 = null,
    pub fn init(a: A, config: V, p: V, socket: *Socket) !State {
        const kind = std.meta.stringToEnum(Kind, try text(p, "type")) orelse return error.UnknownChannel;
        const acct = switch (kind) {
            .orders, .account, .balances, .positions => true,
            else => false,
        };
        const auth = get(config, "auth");
        if (acct and auth != .object) return error.AuthenticationRequired;
        const explicit = get(config, "origin");
        const rest = if (acct) try text(auth, "origin") else "https://apimonaco.xyz";
        const pinned = if (acct and get(auth, "ws_origin") != .null) try text(auth, "ws_origin") else try endpoint(a, rest);
        const url = if (explicit == .null or (acct and eq(u8, try w.string(explicit), rest))) pinned else try endpoint(a, try w.string(explicit));
        if (acct and !eq(u8, url, pinned)) return error.AuthenticationRequired;
        var chan: std.Io.Writer.Allocating = .init(a);
        try chan.writer.writeAll(@tagName(kind));
        const pair = get(p, "tradingPairId");
        const mode = get(p, "mode");
        if (!acct or kind == .orders or kind == .positions) {
            if (pair != .null) {
                const id = try w.string(pair);
                if (kind != .trades or !eq(u8, id, "*")) try @import("orders.zig").uuid(pair);
                try chan.writer.print(":{s}", .{try std.ascii.allocLowerString(a, id)});
            } else if (!acct) return error.InvalidMarket else if (kind == .orders and mode != .null) try chan.writer.writeAll(":*");
        } else if (pair != .null) return error.InvalidSubscription;
        if (mode != .null) {
            if (kind != .orders and kind != .orderbook and kind != .depth and kind != .bbo) return error.InvalidSubscription;
            const m = try w.string(mode);
            if (!in(m, &.{ "MARGIN", "SPOT" })) return error.InvalidMode;
            try chan.writer.print(":{s}", .{m});
        } else if (!acct and kind != .trades) return error.InvalidMode;
        const mag = get(p, "magnitude");
        const denom = get(p, "denomination");
        if (mag != .null or denom != .null) {
            if (kind != .orderbook) return error.InvalidSubscription;
            const m = if (mag == .null) "*" else try w.string(mag);
            if (!in(m, &.{ "*", "0.000001", "0.00001", "0.0001", "0.001", "0.01", "0.1", "1", "10", "100", "1000", "10000" })) return error.InvalidMagnitude;
            try chan.writer.print(":{s}", .{m});
            if (denom != .null) {
                const d = try w.string(denom);
                if (!in(d, &.{ "*", "base", "quote" })) return error.InvalidDenomination;
                try chan.writer.print(":{s}", .{d});
            }
        }
        for (p.object.keys()) |k| if (!in(k, &.{ "type", "tradingPairId", "mode", "magnitude", "denomination" })) return error.UnknownParameter;
        const channel = chan.written();
        const frame = try std.json.Stringify.valueAlloc(a, .{ .type = "Subscribe", .channels = .{channel} }, .{});
        return .{ .kind = kind, .channel = channel, .endpoint = url, .frame = frame, .auth = if (acct) auth else .null, .lease = if (acct) try session.Lease.attach(auth, socket) else null };
    }
    pub fn deinit(self: *State) void {
        if (self.lease) |l| l.release();
        if (self.pending) |p| std.heap.c_allocator.free(p);
    }
    pub fn active(self: *State) bool {
        return if (self.lease) |l| !l.invalid.load(.acquire) else true;
    }
    pub fn start(self: *State, a: A, io: std.Io) ![]const u8 {
        self.sequence = null;
        self.snapshot = eq(u8, self.channel, "trades:*");
        if (self.lease == null) return self.frame;
        if (!self.active()) return error.AuthenticationRequired;
        self.authenticating = true;
        return @import("../../core/auth.zig").monacoSocket(a, self.auth, io);
    }
    pub fn event(self: *State, a: A, raw: []const u8) !E {
        if (!self.active()) return error.AuthenticationRequired;
        const v = try w.parse(a, raw);
        const tag = try text(v, "type");
        if (eq(u8, tag, "Ping")) return .{ .reply = "{\"type\":\"Pong\"}" };
        if (eq(u8, tag, "Pong")) return .skip;
        if (eq(u8, tag, "Error")) {
            const c = get(v, "code");
            const code = if (c == .string) c.string else "";
            if (self.lease != null and (self.authenticating or in(code, &.{ "AUTH_FAILED", "SESSION_INVALID", "ACCOUNT_DEACTIVATED", "ACCOUNT_RESTRICTED" }))) {
                try self.lease.?.reject();
                return error.AuthenticationRequired;
            }
            if (eq(u8, code, "SNAPSHOT_UNAVAILABLE")) {
                self.snapshot = self.kind != .depth and self.kind != .bbo;
                self.sequence = null;
                self.pending = try std.heap.c_allocator.dupe(u8, raw);
                return error.RecoverableStreamGap;
            }
            return error.ExchangeApi;
        }
        if (self.authenticating) {
            if (eq(u8, tag, "Authenticated")) {
                self.authenticating = false;
                return .subscribe;
            }
            return .skip;
        }
        if (eq(u8, tag, "Authenticated")) return .skip;
        const c = get(v, "channel");
        const cs = get(v, "channels");
        if (c != .null) {
            if (!sameChannel(self.channel, try w.string(c))) return .skip;
        } else if (cs == .array) {
            for (cs.array.items) |key| {
                if (eq(u8, self.channel, try w.string(key))) break;
            } else return .skip;
        } else return .skip;
        if (eq(u8, tag, "Snapshot")) {
            self.snapshot = true;
            if (self.kind == .depth) self.sequence = w.unsigned(u64, get(get(v, "data"), "book_seq")) catch null;
        } else if (eq(u8, tag, "Event")) {
            if (!self.snapshot) return .skip;
            if (self.kind == .depth) {
                const data = get(v, "data");
                const prev = w.unsigned(u64, get(data, "prev_book_seq")) catch return error.StreamGap;
                const next = w.unsigned(u64, get(data, "book_seq")) catch return error.StreamGap;
                if (self.sequence == null or prev > self.sequence.?) return error.StreamGap;
                self.sequence = @max(self.sequence.?, next);
            }
        }
        return .{ .deliver = raw };
    }
};
