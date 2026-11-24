// One authenticated connection. A timed-out read remains the next read on this
// socket, so a late acknowledgement can reconcile its command without resending.
const std = @import("std");
const w = @import("../../core/wire.zig");
const Socket = @import("../../core/websocket.zig").Session;
const gpa = std.heap.c_allocator;
const A = w.A;
const V = w.V;
const eq = std.mem.eql;
fn text(v: V, key: []const u8) ![]const u8 {
    return w.string(try w.field(v, key));
}
fn nonempty(v: V, key: []const u8) ![]const u8 {
    const s = try text(v, key);
    if (std.mem.trim(u8, s, " \t\r\n").len == 0) return error.InvalidParameter;
    return s;
}
fn choices(v: V, key: []const u8, items: []const []const u8) ![]const u8 {
    const s = try text(v, key);
    for (items) |allowed| if (eq(u8, s, allowed)) return s;
    return error.InvalidParameter;
}
fn keys(v: V, allowed: []const []const u8) !void {
    if (v != .object) return error.InvalidParameter;
    for (v.object.keys()) |k| {
        for (allowed) |valid| {
            if (eq(u8, k, valid)) break;
        } else return error.UnknownParameter;
    }
}
fn positive(v: V) !void {
    if (v != .number_string) return error.InvalidQuantity;
    const s = v.number_string;
    var dot = false;
    var nonzero = false;
    for (s) |c| {
        if (c == '.' and !dot) {
            dot = true;
            continue;
        }
        if (!std.ascii.isDigit(c)) return error.InvalidQuantity;
        nonzero = nonzero or c != '0';
    }
    if (!nonzero) return error.InvalidQuantity;
}
fn uuid(s: []const u8) !void {
    if (s.len != 36) return error.InvalidUuid;
    for (s, 0..) |c, i| if (if (i == 8 or i == 13 or i == 18 or i == 23) c != '-' else !std.ascii.isHex(c)) return error.InvalidUuid;
}
fn event(a: A, raw: []const u8) !V {
    const v = try w.parse(a, raw);
    if (v != .object) return error.InvalidEvent;
    if (v.object.contains("error") or v.object.contains("error_code") or v.object.contains("err_response")) return error.ExchangeApi;
    return v;
}
fn response(v: V, id: []const u8) ?V {
    if (id.len == 0) return v.object.get("all_orders_response");
    const order = v.object.get("order_response") orelse return null;
    if (order != .object) return null;
    for ([_]([]const u8){ "order_id", "client_order_id" }) |k| if (order.object.get(k)) |x| {
        if (x == .string and eq(u8, x.string, id)) return order;
    };
    return null;
}
fn validateOrderResponse(order: V) !void {
    _ = try text(order, "symbol");
    _ = try text(order, "status");
    for ([_]([]const u8){ "order_id", "client_order_id" }) |k| if (order.object.get(k)) |v| {
        _ = try w.string(v);
    };
}
fn decodeResponse(a: A, v: V, list: bool) !V {
    if (v != .object) return error.InvalidEvent;
    var out = v;
    if (list) {
        for ([_]([]const u8){ "orders", "twaps" }) |k| {
            const value = out.object.get(k) orelse .null;
            if (value == .null) {
                try out.object.put(a, k, .{ .array = .init(a) });
                continue;
            }
            if (value != .array) return error.InvalidEvent;
            if (eq(u8, k, "orders")) for (value.array.items) |order| try validateOrderResponse(order);
        }
    } else try validateOrderResponse(v);
    return out;
}
pub const Session = struct {
    ws: *Socket,
    timeout_ms: u32,
    queue: std.ArrayList([]u8) = .empty,
    pending: ?[]u8 = null,
    worker: ?std.Thread = null,
    ready: std.Io.Event = .unset,
    result: anyerror!?[]u8 = null,

    pub fn open(a: A, input: V, dial: ?@import("../../core/host_stream.zig").Dial) !*Session {
        const params = try w.field(input, "params");
        try keys(params, &.{ "timeout_ms", "account_id", "builder_code" });
        const key = try nonempty(input, "api_key");
        const secret = try nonempty(input, "api_secret");
        var io: std.Io.Threaded = .init_single_threaded;
        var h: V = .{ .object = .empty };
        try @import("../../core/auth.zig").qfexHeaders(a, &h, io.io(), key, secret);
        const nonce = try text(h, "x-qfex-nonce");
        const time = try w.unsigned(u64, try w.field(h, "x-qfex-timestamp"));
        var auth = try w.parse(a, try std.json.Stringify.valueAlloc(a, .{ .hmac = .{ .public_key = key, .nonce = nonce, .unix_ts = time, .signature = try text(h, "x-qfex-hmac-signature") } }, .{}));
        for ([_]([]const u8){ "account_id", "builder_code" }) |k| if (params.object.get(k)) |v| {
            try uuid(try w.string(v));
            try auth.object.put(a, k, v);
        };
        const timeout = if (params.object.get("timeout_ms")) |v| try w.unsigned(u32, v) else 10000;
        if (timeout == 0) return error.InvalidTimeout;
        const endpoint = if (input.object.get("origin")) |v| try w.string(v) else "wss://trade.qfex.com/";
        if (std.mem.findAny(u8, endpoint, "?#") != null) return error.InvalidEndpoint;
        const self = try gpa.create(Session);
        errdefer gpa.destroy(self);
        self.* = .{ .ws = try Socket.dedicated(try std.mem.concat(a, u8, &.{ endpoint, "?api_key=", key }), dial), .timeout_ms = timeout };
        errdefer self.release();
        try self.ws.sendText(try std.json.Stringify.valueAlloc(a, .{ .type = "auth", .params = auth }, .{}));
        const limit = self.deadline();
        while (true) {
            const raw = try self.read(limit) orelse return error.AuthenticationRequired;
            defer gpa.free(raw);
            var scratch = std.heap.ArenaAllocator.init(gpa);
            defer scratch.deinit();
            const msg = try event(scratch.allocator(), raw);
            if (msg.object.get("authenticated")) |ok| {
                if (ok != .bool or !ok.bool) return error.AuthenticationRequired;
                break;
            }
        }
        try self.ws.sendText("{\"type\":\"subscribe\",\"params\":{\"channels\":[\"order_responses\"]}}");
        return self;
    }
    fn deadline(self: *Session) std.Io.Timeout {
        const duration: std.Io.Timeout = .{ .duration = .{ .clock = .awake, .raw = .fromMilliseconds(self.timeout_ms) } };
        return .{ .deadline = duration.toTimestamp(self.ws.io.io()).? };
    }
    fn receive(self: *Session) void {
        self.result = self.ws.message();
        self.ready.set(self.ws.io.io());
    }
    fn read(self: *Session, timeout: std.Io.Timeout) !?[]u8 {
        if (self.worker == null and timeout == .none) return self.ws.message();
        if (self.worker == null) {
            self.ready = .unset;
            self.worker = try std.Thread.spawn(.{}, receive, .{self});
        }
        try self.ready.waitTimeout(self.ws.io.io(), timeout);
        self.worker.?.join();
        self.worker = null;
        const r = self.result;
        self.result = null;
        return r;
    }
    fn clear(self: *Session) void {
        if (self.pending) |id| gpa.free(id);
        self.pending = null;
    }
    pub fn next(self: *Session) !?[]u8 {
        if (self.ws.canceled.load(.acquire)) return null;
        if (self.queue.items.len > 0) return self.queue.orderedRemove(0);
        const raw = try self.read(.none) orelse return null;
        errdefer gpa.free(raw);
        var arena = std.heap.ArenaAllocator.init(gpa);
        defer arena.deinit();
        const v = try event(arena.allocator(), raw);
        if (self.pending) |id| if (response(v, id) != null) self.clear();
        return raw;
    }
    pub fn command(self: *Session, a: A, input: V) !V {
        const op = try text(input, "operation");
        if (self.pending != null) return error.PendingOutcome;
        if (self.ws.canceled.load(.acquire)) return error.Network;
        if (eq(u8, op, "subscribe_account")) {
            try self.ws.sendText("{\"type\":\"subscribe\",\"params\":{\"channels\":[\"balances\",\"positions\",\"fills\"]}}");
            return .null;
        }
        var p = try w.field(input, "params");
        const id = if (eq(u8, op, "add_order")) id: {
            try keys(p, &.{ "symbol", "side", "order_type", "order_time_in_force", "quantity", "price", "client_order_id", "reduce_only" });
            _ = try nonempty(p, "symbol");
            _ = try choices(p, "side", &.{ "BUY", "SELL" });
            const kind = try choices(p, "order_type", &.{ "LIMIT", "MARKET", "ALO" });
            _ = try choices(p, "order_time_in_force", &.{ "GTC", "IOC", "FOK" });
            try positive(try w.field(p, "quantity"));
            const price = p.object.get("price") orelse .null;
            if (price != .null) try positive(price) else if (!eq(u8, kind, "MARKET")) return error.InvalidPrice else _ = p.object.swapRemove("price");
            if (p.object.get("reduce_only")) |v| {
                if (v != .number_string or try w.unsigned(u8, v) > 1) return error.InvalidParameter;
            } else try p.object.put(a, "reduce_only", .{ .integer = 0 });
            break :id try nonempty(p, "client_order_id");
        } else if (eq(u8, op, "cancel_order")) id: {
            try keys(p, &.{ "symbol", "order_id", "cancel_order_id_type" });
            _ = try nonempty(p, "symbol");
            _ = try choices(p, "cancel_order_id_type", &.{ "order_id", "client_order_id" });
            break :id try nonempty(p, "order_id");
        } else if (eq(u8, op, "user_orders")) id: {
            try keys(p, &.{ "symbol", "limit", "offset" });
            if (try w.unsigned(u32, try w.field(p, "limit")) == 0) return error.InvalidLimit;
            _ = try w.unsigned(u32, try w.field(p, "offset"));
            if (p.object.get("symbol")) |v| {
                if (v == .null) _ = p.object.swapRemove("symbol") else _ = try nonempty(p, "symbol");
            }
            break :id "";
        } else return error.InvalidOperation;
        const payload = try std.json.Stringify.valueAlloc(a, .{ .type = if (id.len == 0) "get_user_orders" else op, .params = p }, .{});
        self.pending = try gpa.dupe(u8, id);
        self.ws.sendText(payload) catch return error.PendingOutcome;
        const limit = self.deadline();
        while (true) {
            const raw = (self.read(limit) catch return error.PendingOutcome) orelse return error.PendingOutcome;
            var scratch = std.heap.ArenaAllocator.init(gpa);
            defer scratch.deinit();
            const v = event(scratch.allocator(), raw) catch |e| {
                gpa.free(raw);
                return e;
            };
            if (response(v, id)) |r| {
                defer gpa.free(raw);
                self.clear();
                return decodeResponse(a, try w.parse(a, try std.json.Stringify.valueAlloc(a, r, .{})), id.len == 0);
            }
            if (self.queue.items.len >= 128) {
                gpa.free(raw);
                self.ws.close();
                return error.PendingOutcome;
            }
            self.queue.append(gpa, raw) catch |e| {
                gpa.free(raw);
                return e;
            };
        }
    }
    pub fn close(self: *Session) void {
        self.ws.close();
    }
    fn release(self: *Session) void {
        self.close();
        if (self.worker) |t| {
            t.join();
            if (self.result catch null) |bytes| gpa.free(bytes);
        }
        for (self.queue.items) |bytes| gpa.free(bytes);
        self.queue.deinit(gpa);
        self.clear();
        self.ws.destroy();
    }
    pub fn destroy(self: *Session) void {
        self.release();
        gpa.destroy(self);
    }
};
