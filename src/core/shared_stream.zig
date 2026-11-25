//! Pool compatible subscriptions. Stateful readers start from their own snapshot.
const std = @import("std");
const w = @import("wire.zig");
const Socket = @import("websocket.zig").Session;
const gpa = std.heap.c_allocator;
const Key = [32]u8;
var runtime: std.Io.Threaded = .init_single_threaded;
var mutex: std.Io.Mutex = .init;
var registry: std.AutoHashMapUnmanaged(Key, *Group) = .empty;
// Serialize first handshakes for equal keys without blocking existing readers.
var connecting: [32]std.Io.Mutex = @splat(.init);

pub fn eligible(venue: []const u8, p: w.V) bool {
    if (std.mem.eql(u8, venue, "chainlink_streams")) return true;
    if (p != .object) return false;
    const channel = p.object.get(if (std.mem.eql(u8, venue, "hyperliquid")) "type" else "channel") orelse return false;
    if (channel != .string) return false;
    if (std.mem.eql(u8, venue, "hyperliquid")) return true;
    if (std.mem.eql(u8, venue, "arcus")) return !std.mem.eql(u8, channel.string, "l2OrderbookUpdates");
    if (std.mem.eql(u8, venue, "lighter")) return std.mem.eql(u8, channel.string, "trade") or std.mem.eql(u8, channel.string, "order_book") or std.mem.eql(u8, channel.string, "account_all");
    return std.mem.eql(u8, venue, "qfex");
}

fn canonical(a: w.A, value: w.V) !w.V {
    return switch (value) {
        .object => |map| blk: {
            const keys = try a.dupe([]const u8, map.keys());
            std.mem.sort([]const u8, keys, {}, struct {
                fn less(_: void, x: []const u8, y: []const u8) bool {
                    return std.mem.lessThan(u8, x, y);
                }
            }.less);
            var out: w.V = .{ .object = .empty };
            for (keys) |key| try out.object.put(a, key, try canonical(a, map.get(key).?));
            break :blk out;
        },
        .array => |list| blk: {
            var out: std.array_list.Managed(w.V) = .init(a);
            for (list.items) |v| try out.append(try canonical(a, v));
            break :blk .{ .array = out };
        },
        else => value,
    };
}
const Topic = union(enum) {
    qfex: @import("../exchanges/qfex/stream.zig").State,
    hyperliquid: @import("../exchanges/hyperliquid/stream.zig").State,
    lighter: @import("../exchanges/lighter/stream.zig").State,

    fn frame(self: Topic) []const u8 {
        return switch (self) {
            inline else => |state| state.frame,
        };
    }
    fn event(self: *Topic, a: w.A, message: []const u8) !@import("../exchanges/derive/stream.zig").State.Event {
        return switch (self.*) {
            inline else => |*state| state.event(a, message),
        };
    }
};

const Group = struct {
    key: Key,
    socket: *Socket,
    members: std.ArrayList(*Reader) = .empty,
    worker: ?std.Thread = null,
    ended: bool = false,

    multiplex: bool = false,

    fn run(self: *Group) void {
        while (true) {
            const item = if (self.multiplex) self.socket.subscriptionMessage() else self.socket.next();
            const io = runtime.io();
            mutex.lockUncancelable(io);
            defer mutex.unlock(io);
            const message = item catch |err| {
                self.finish(err);
                return;
            } orelse {
                self.finish(null);
                return;
            };
            defer gpa.free(message);
            if (self.multiplex) self.socket.retries = 0;
            for (self.members.items) |reader| {
                if (reader.stopped or reader.failure != null) continue;
                if (reader.topic) |*topic| {
                    if (topic.* == .lighter and reader.generation != self.socket.generation) {
                        topic.lighter.snapshot = false;
                        reader.generation = self.socket.generation;
                    }
                    var scratch = std.heap.ArenaAllocator.init(gpa);
                    defer scratch.deinit();
                    const matched = topic.event(scratch.allocator(), message) catch |e| {
                        if (e == error.ExchangeApi) reader.rejection = Socket.rejectionFrame(self.socket.venue, message);
                        reader.failure = e;
                        reader.available.signal(io);
                        continue;
                    };
                    if (matched != .deliver) continue;
                }
                if (reader.queue.items.len == 64) {
                    reader.failure = error.StreamOverflow;
                } else {
                    reader.enqueue(message) catch |err| {
                        reader.failure = err;
                    };
                }
                reader.available.signal(io);
            }
            // A terminal rejection for every remaining reader ends this connection.
            // Otherwise the worker can reconnect and replay a rejected subscription.
            for (self.members.items) |reader| {
                if (!reader.stopped and reader.failure == null) break;
            } else {
                self.finish(null);
                return;
            }
        }
    }
    fn finish(self: *Group, failure: ?anyerror) void {
        self.ended = true;
        for (self.members.items) |r| {
            if (r.failure == null) {
                r.failure = failure;
                if (self.socket.rejection) |detail| r.rejection = gpa.dupe(u8, detail) catch null;
            }
            r.available.broadcast(runtime.io());
        }
    }
    fn destroy(self: *Group) void {
        self.socket.close();
        if (self.worker) |worker| worker.join();
        self.socket.destroy();
        self.members.deinit(gpa);
        gpa.destroy(self);
    }
};
pub const Reader = struct {
    group: ?*Group = null,
    key: Key = undefined,
    stopped: bool = false,
    failure: ?anyerror = null,
    rejection: ?[]u8 = null,
    queue: std.ArrayList([]u8) = .empty,
    available: std.Io.Condition = .init,

    arena: std.heap.ArenaAllocator,
    topic: ?Topic = null,
    generation: u64 = 0,
    unsubscribe: []const u8 = "",

    pub fn open(a: w.A, config: []const u8, input: w.V) !*Reader {
        const r = try gpa.create(Reader);
        errdefer gpa.destroy(r);
        r.* = .{ .arena = .init(gpa) };
        errdefer r.arena.deinit();
        var identity = try canonical(a, input);
        const venue = try w.string(try w.field(input, "venue"));
        const lighter = std.mem.eql(u8, venue, "lighter");
        const qfex = std.mem.eql(u8, venue, "qfex");
        const multiplex = qfex or lighter or std.mem.eql(u8, venue, "hyperliquid");
        if (multiplex) {
            const ra = r.arena.allocator();
            const saved = try w.parse(ra, try ra.dupe(u8, config));
            const params = try w.field(saved, "params");
            if (qfex) {
                const state = try @import("../exchanges/qfex/stream.zig").State.init(ra, params);
                r.unsubscribe = try state.unsubscribeFrame(ra);
                r.topic = .{ .qfex = state };
            } else if (lighter) {
                const state = try @import("../exchanges/lighter/stream.zig").State.init(ra, saved, params, runtime.io());
                var frame = try w.parse(ra, state.frame);
                try frame.object.put(ra, "type", w.text("unsubscribe"));
                r.unsubscribe = try std.json.Stringify.valueAlloc(ra, frame, .{});
                r.topic = .{ .lighter = state };
            } else {
                const state = try @import("../exchanges/hyperliquid/stream.zig").State.init(ra, saved, params);
                // Order updates omit the account. Fill frames omit aggregation.
                // Only filters that can disambiguate every response share a socket.
                const user = state.sub.object.get("user");
                var scope: w.V = .null;
                if (user) |address| {
                    scope = .{ .object = .empty };
                    try scope.object.put(a, "user", address);
                    try scope.object.put(a, "aggregate", state.sub.object.get("aggregateByTime") orelse .{ .bool = false });
                }
                try identity.object.put(a, "connection_scope", scope);
                r.unsubscribe = try std.json.Stringify.valueAlloc(ra, .{ .method = "unsubscribe", .subscription = state.sub }, .{});
                r.topic = .{ .hyperliquid = state };
            }
            _ = identity.object.swapRemove("params");
        }
        const canonical_json = try std.json.Stringify.valueAlloc(a, identity, .{});
        var key: Key = undefined;
        std.crypto.hash.sha2.Sha256.hash(canonical_json, &key, .{});
        r.key = key;
        const stripe = &connecting[key[0] % connecting.len];
        const io = runtime.io();
        stripe.lockUncancelable(io);
        defer stripe.unlock(io);
        mutex.lockUncancelable(io);
        var register = true;
        if (registry.get(key)) |g| attach: {
            if (g.ended) break :attach;
            // A second reader of a live Lighter channel must receive a new snapshot.
            if (lighter) for (g.members.items) |member| {
                if (std.mem.eql(u8, member.topic.?.frame(), r.topic.?.frame())) {
                    register = false;
                    break :attach;
                }
            };
            g.members.append(gpa, r) catch |e| {
                mutex.unlock(io);
                return e;
            };
            r.group = g;
            mutex.unlock(io);
            if (r.topic) |topic| g.socket.joinTopic(topic.frame()) catch |e| {
                mutex.lockUncancelable(io);
                for (g.members.items, 0..) |member, n| if (member == r) {
                    _ = g.members.swapRemove(n);
                    break;
                };
                mutex.unlock(io);
                for (r.queue.items) |msg| gpa.free(msg);
                r.queue.deinit(gpa);
                if (r.rejection) |detail| gpa.free(detail);
                return e;
            };
            return r;
        }
        mutex.unlock(io);
        const g = try gpa.create(Group);
        errdefer gpa.destroy(g);
        g.* = .{ .key = key, .socket = try Socket.open(config, null), .multiplex = multiplex };
        errdefer g.socket.destroy();
        if (r.topic) |topic| try g.socket.startMultiplex(topic.frame());
        try g.members.append(gpa, r);
        errdefer g.members.deinit(gpa);
        r.group = g;
        mutex.lockUncancelable(io);
        defer mutex.unlock(io);
        if (register) try registry.put(gpa, key, g);
        errdefer if (register) {
            _ = registry.remove(key);
        };
        g.worker = try std.Thread.spawn(.{}, Group.run, .{g});
        return r;
    }
    fn enqueue(self: *Reader, message: []const u8) !void {
        const copy = try gpa.dupe(u8, message);
        errdefer gpa.free(copy);
        try self.queue.append(gpa, copy);
    }
    pub fn next(self: *Reader) !?[]u8 {
        const io = runtime.io();
        mutex.lockUncancelable(io);
        defer mutex.unlock(io);
        while (!self.stopped) {
            if (self.queue.items.len > 0) return self.queue.orderedRemove(0);
            if (self.failure) |err| return err;
            if (self.group.?.ended) return null;
            self.available.waitUncancelable(io, &mutex);
        }
        return null;
    }
    pub fn close(self: *Reader) void {
        const io = runtime.io();
        const stripe = &connecting[self.key[0] % connecting.len];
        stripe.lockUncancelable(io);
        defer stripe.unlock(io);
        mutex.lockUncancelable(io);
        if (self.stopped) {
            mutex.unlock(io);
            return;
        }
        self.stopped = true;
        self.available.broadcast(io);
        const g = self.group.?;
        self.group = null;
        for (g.members.items, 0..) |r, i| if (r == self) {
            _ = g.members.swapRemove(i);
            break;
        };
        const last = g.members.items.len == 0;
        if (last and registry.get(g.key) == g) _ = registry.remove(g.key);
        mutex.unlock(io);
        if (last) g.destroy() else if (self.topic) |topic| g.socket.leaveTopic(topic.frame(), self.unsubscribe);
    }
    pub fn destroy(self: *Reader) void {
        self.close();
        for (self.queue.items) |v| gpa.free(v);
        self.queue.deinit(gpa);
        if (self.rejection) |detail| gpa.free(detail);
        self.arena.deinit();
        gpa.destroy(self);
    }
};
