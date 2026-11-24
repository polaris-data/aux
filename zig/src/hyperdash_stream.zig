// A Hyperdash session multiplexes subscriptions and maintains one pending batch per ID.
const std = @import("std");
const w = @import("wire.zig");
const Socket = @import("websocket.zig").Session;
const gpa = std.heap.c_allocator;
const eq = std.mem.eql;
const A = w.A;
const V = w.V;
fn text(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn signed(v: V) !void {
    if (v != .number_string) return error.InvalidInteger;
    _ = try std.fmt.parseInt(i128, v.number_string, 10);
}
fn id(a: A, value: V) ![]const u8 {
    if (value != .string) try signed(value);
    return std.json.Stringify.valueAlloc(a, value, .{});
}
fn strOpt(v: V, k: []const u8) !?[]const u8 {
    const x = v.object.get(k) orelse return null;
    return if (x == .null) null else try w.string(x);
}
fn oneOf(s: []const u8, choices: []const []const u8) bool {
    for (choices) |choice| if (eq(u8, s, choice)) return true;
    return false;
}
fn subscription(a: A, p: V) !V {
    const channel = try text(p, "channel");
    var frame: V = .{ .object = .empty };
    var it = p.object.iterator();
    while (it.next()) |field| {
        if (!oneOf(field.key_ptr.*, &.{ "channel", "coin", "address", "cursor", "interval" })) return error.InvalidSubscription;
        if (field.value_ptr.* == .null) continue;
        _ = try w.string(field.value_ptr.*);
        try frame.object.put(a, field.key_ptr.*, field.value_ptr.*);
    }
    const address = try strOpt(p, "address");
    const coin = try strOpt(p, "coin");
    if (oneOf(channel, &.{ "fills", "ledger", "twap", "orders", "funding", "account" })) {
        const value = address orelse return error.InvalidAddress;
        if (coin != null or value.len != 42 or !std.mem.startsWith(u8, value, "0x")) return error.InvalidSubscription;
        for (value[2..]) |c| if (!std.ascii.isHex(c)) return error.InvalidAddress;
    } else {
        if (address != null) return error.InvalidSubscription;
        if (oneOf(channel, &.{ "mids", "markets" })) {
            if (coin) |c| if (!eq(u8, c, "*")) return error.InvalidSubscription;
        } else {
            const value = coin orelse return error.InvalidSubscription;
            if (value.len == 0 or !oneOf(channel, &.{ "trades", "liquidations", "candles", "ctx", "book", "bbo", "liquidation-levels", "stop-levels" }) or (eq(u8, value, "*") and !eq(u8, channel, "liquidations"))) return error.InvalidSubscription;
        }
    }
    if (try strOpt(p, "interval")) |i| if (!eq(u8, channel, "candles") or !oneOf(i, &.{ "1m", "5m", "15m", "1h", "4h", "1d" })) return error.InvalidSubscription;
    try frame.object.put(a, "op", w.text("subscribe"));
    return frame;
}

const Batch = struct { sub: []const u8, batch: []const u8 };
pub const Session = struct {
    ws: *Socket,
    worker: ?std.Thread = null,
    mutex: std.Io.Mutex = .init,
    available: std.Io.Condition = .init,
    ready: std.Io.Event = .unset,
    queue: std.ArrayList([]u8) = .empty,
    pending: std.ArrayList(Batch) = .empty,
    capacity: usize,
    terminal: ?anyerror = null,
    authenticated: bool = false,
    closed: std.atomic.Value(bool) = .init(false),

    pub fn open(a: A, input: V, dial: ?@import("host_stream.zig").Dial) !*Session {
        const key = try w.string(input.object.get("api_key") orelse return error.AuthenticationRequired);
        if (key.len == 0) return error.AuthenticationRequired;
        for (key) |c| if (c < 33 or c > 126) return error.InvalidCredential;
        const p = try w.field(input, "params");
        if (p != .object) return error.InvalidParameter;
        const capacity = if (p.object.get("capacity")) |v| try w.unsigned(usize, v) else 64;
        const ms = if (p.object.get("ready_timeout_ms")) |v| try w.unsigned(u32, v) else 10000;
        if (capacity == 0 or ms == 0) return error.InvalidParameter;
        const endpoint = if (input.object.get("origin")) |v| try w.string(v) else "wss://data.hyperdash.com/v1/stream";
        _ = a;
        const self = try gpa.create(Session);
        errdefer gpa.destroy(self);
        self.* = .{ .ws = try Socket.pyth(endpoint, key, "", dial), .capacity = capacity };
        errdefer self.release();
        self.worker = try std.Thread.spawn(.{}, drive, .{self});
        try self.ready.waitTimeout(self.ws.io.io(), .{ .duration = .{ .clock = .awake, .raw = .fromMilliseconds(ms) } });
        self.mutex.lockUncancelable(self.ws.io.io());
        defer self.mutex.unlock(self.ws.io.io());
        if (!self.authenticated) return self.terminal orelse error.Network;
        return self;
    }

    fn drive(self: *Session) void {
        self.pump() catch |err| {
            const io = self.ws.io.io();
            self.mutex.lockUncancelable(io);
            defer self.mutex.unlock(io);
            self.terminal = err;
            self.ready.set(io);
            self.available.broadcast(io);
        };
    }
    fn pump(self: *Session) !void {
        while (!self.closed.load(.acquire)) {
            const raw = try self.ws.message() orelse return error.Network;
            defer gpa.free(raw);
            var arena = std.heap.ArenaAllocator.init(gpa);
            defer arena.deinit();
            const a = arena.allocator();
            const v = try w.parse(a, raw);
            const op = try text(v, "op");
            if (eq(u8, op, "ping")) {
                try self.ws.sendText("{\"op\":\"pong\"}");
                continue;
            }
            const io = self.ws.io.io();
            self.mutex.lockUncancelable(io);
            defer self.mutex.unlock(io);
            if (!self.authenticated and !eq(u8, op, "ready")) return if (eq(u8, op, "error")) error.AuthenticationRequired else error.InvalidResponse;
            const message = try self.event(a, op, v);
            if (self.queue.items.len >= self.capacity) return error.StreamOverflow;
            const bytes = try std.json.Stringify.valueAlloc(gpa, message, .{});
            errdefer gpa.free(bytes);
            try self.queue.append(gpa, bytes);
            self.authenticated = true;
            self.ready.set(io);
            self.available.signal(io);
        }
    }

    fn find(self: *Session, sub: []const u8) ?usize {
        for (self.pending.items, 0..) |p, i| if (eq(u8, p.sub, sub)) return i;
        return null;
    }
    fn remove(self: *Session, i: usize) void {
        const pending = self.pending.swapRemove(i);
        gpa.free(pending.sub);
        gpa.free(pending.batch);
    }
    fn event(self: *Session, a: A, op: []const u8, v: V) !V {
        if (eq(u8, op, "ready")) {
            _ = try text(v, "key_id");
            _ = try text(v, "environment");
        } else if (eq(u8, op, "credits_low")) {
            try signed(try w.field(v, "remaining"));
        } else if (oneOf(op, &.{ "subscribed", "data", "acked", "unsubscribed", "gap" })) {
            const sub = try id(a, try w.field(v, "sub"));
            if (eq(u8, op, "subscribed")) {
                _ = try text(v, "channel");
                for ([_][]const u8{ "address", "coin", "cursor" }) |k| _ = try strOpt(v, k);
            } else if (eq(u8, op, "data")) {
                const batch = try id(a, try w.field(v, "batch"));
                _ = try text(v, "cursor");
                if (try w.field(v, "events") != .array) return error.InvalidResponse;
                try signed(try w.field(v, "credits"));
                if (self.pending.items.len >= 20 or self.find(sub) != null) return error.InvalidResponse;
                const owned_sub = try gpa.dupe(u8, sub);
                errdefer gpa.free(owned_sub);
                const owned_batch = try gpa.dupe(u8, batch);
                errdefer gpa.free(owned_batch);
                try self.pending.append(gpa, .{ .sub = owned_sub, .batch = owned_batch });
            } else if (eq(u8, op, "acked")) {
                _ = try id(a, try w.field(v, "batch"));
                try signed(try w.field(v, "credits_charged"));
            } else if (eq(u8, op, "gap")) {
                _ = try text(v, "detail");
                _ = try w.field(v, "from");
                _ = try w.field(v, "to");
            } else if (self.find(sub)) |i| self.remove(i);
        } else if (eq(u8, op, "error")) {
            const code = try text(v, "code");
            _ = try text(v, "detail");
            if (v.object.get("retry_after_s")) |val| if (val != .null) try signed(val);
            if (v.object.get("sub")) |val| if (val != .null) {
                const sub = try id(a, val);
                if (oneOf(code, &.{ "ack_timeout", "subscription_superseded" })) if (self.find(sub)) |i| self.remove(i);
            };
            return @import("metadata.zig").redact(a, v, self.ws.bearer.?);
        } else {
            var out: V = .{ .object = .empty };
            try out.object.put(a, "op", w.text("unknown"));
            try out.object.put(a, "native", v);
            return out;
        }
        return v;
    }

    pub fn command(self: *Session, a: A, input: V) !V {
        const op = try text(input, "operation");
        const subscribe = eq(u8, op, "subscribe");
        var frame = if (subscribe) try subscription(a, try w.field(input, "subscription")) else V{ .object = .empty };
        const io = self.ws.io.io();
        self.mutex.lockUncancelable(io);
        defer self.mutex.unlock(io);
        if (self.closed.load(.acquire) or self.terminal != null) return error.Closed;
        var ack: ?usize = null;
        if (!subscribe) {
            const sub = try w.field(input, "sub");
            const index = self.find(try id(a, sub));
            if (eq(u8, op, "ack")) {
                ack = index orelse return error.InvalidBatch;
                const batch = try w.field(input, "batch");
                if (!eq(u8, self.pending.items[ack.?].batch, try id(a, batch))) return error.InvalidBatch;
                try frame.object.put(a, "batch", batch);
            } else if (!eq(u8, op, "unsubscribe")) return error.InvalidOperation;
            try frame.object.put(a, "op", w.text(op));
            try frame.object.put(a, "sub", sub);
        }
        const bytes = try std.json.Stringify.valueAlloc(a, frame, .{});
        if (bytes.len > 65536) return error.InvalidCommand;
        self.ws.sendText(bytes) catch |err| {
            self.terminal = err;
            self.ws.close();
            return err;
        };
        if (ack) |i| self.remove(i);
        return .null;
    }
    pub fn next(self: *Session) !?[]u8 {
        const io = self.ws.io.io();
        self.mutex.lockUncancelable(io);
        defer self.mutex.unlock(io);
        while (!self.closed.load(.acquire)) {
            if (self.queue.items.len > 0) return self.queue.orderedRemove(0);
            if (self.terminal) |err| {
                self.closed.store(true, .release);
                return err;
            }
            self.available.waitUncancelable(io, &self.mutex);
        }
        return null;
    }
    pub fn close(self: *Session) void {
        self.closed.store(true, .release);
        self.ws.close();
        const io = self.ws.io.io();
        self.mutex.lockUncancelable(io);
        defer self.mutex.unlock(io);
        self.available.broadcast(io);
    }
    fn release(self: *Session) void {
        self.close();
        if (self.worker) |worker| worker.join();
        for (self.queue.items) |s| gpa.free(s);
        self.queue.deinit(gpa);
        while (self.pending.items.len > 0) self.remove(0);
        self.pending.deinit(gpa);
        self.ws.destroy();
    }
    pub fn destroy(self: *Session) void {
        self.release();
        gpa.destroy(self);
    }
};
