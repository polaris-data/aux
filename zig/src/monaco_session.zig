// Active subscriptions share expiry and revocation with their originating client.
// The registry stores a digest for identity and retains rejected IDs only to expiry.
const std = @import("std");
const w = @import("wire.zig");
const Socket = @import("websocket.zig").Session;
const gpa = std.heap.c_allocator;
const Key = [32]u8;
var mutex: std.Io.Mutex = .init;
var changed: std.Io.Condition = .init;
var first: ?*Lease = null;
var rejected: std.AutoHashMapUnmanaged(Key, i64) = .empty;
fn now(io: std.Io) i64 {
    return std.Io.Timestamp.now(io, .real).toSeconds();
}
fn identity(v: w.V) !Key {
    var hash = std.crypto.hash.sha2.Sha256.init(.{});
    hash.update(std.mem.trimEnd(u8, try w.string(try w.field(v, "origin")), "/"));
    hash.update(&.{0});
    var key = try @import("evm.zig").hex(32, try w.string(try w.field(v, "seed")));
    defer std.crypto.secureZero(u8, &key);
    hash.update(&key);
    if (v.object.get("instance")) |id| hash.update(try w.string(id));
    return hash.finalResult();
}
fn expiration(v: w.V) !i64 {
    return try w.unsigned(u63, try w.field(v, "expires_at"));
}
pub fn check(v: w.V, io: std.Io) !void {
    const id = try identity(v);
    mutex.lockUncancelable(io);
    defer mutex.unlock(io);
    if (rejected.get(id)) |t| if (t > now(io)) return error.AuthenticationRequired;
}
fn signal(id: Key, until: i64, revoking: bool, io: std.Io) !void {
    mutex.lockUncancelable(io);
    defer mutex.unlock(io);
    const clock = now(io);
    var it = rejected.iterator();
    while (it.next()) |kv| if (kv.value_ptr.* <= clock) {
        _ = rejected.remove(kv.key_ptr.*);
    };
    var expires = until;
    var entry = first;
    while (entry) |lease| : (entry = lease.next) {
        if (!std.mem.eql(u8, &id, &lease.id)) continue;
        if (revoking) {
            expires = @max(expires, lease.expires);
            lease.invalid.store(true, .release);
            lease.socket.close();
        } else if (!lease.invalid.load(.acquire)) lease.expires = until;
    }
    changed.broadcast(io);
    if (revoking and expires > clock) try rejected.put(gpa, id, @max(expires, rejected.get(id) orelse 0));
}
pub fn revoke(v: w.V, io: std.Io) !void {
    try signal(try identity(v), try expiration(v), true, io);
}
pub fn refresh(v: w.V, expiry: u64, io: std.Io) !void {
    try signal(try identity(v), @intCast(expiry), false, io);
}
pub const Lease = struct {
    id: Key,
    expires: i64,
    socket: *Socket,
    thread: ?std.Thread = null,
    next: ?*Lease = null,
    invalid: std.atomic.Value(bool) = .init(false),
    stop: bool = false,
    pub fn attach(v: w.V, socket: *Socket) !*Lease {
        const io = socket.io.io();
        const id = try identity(v);
        const expires = try expiration(v);
        const self = try gpa.create(Lease);
        errdefer gpa.destroy(self);
        mutex.lockUncancelable(io);
        defer mutex.unlock(io);
        const clock = now(io);
        if (expires <= clock or (rejected.get(id) orelse 0) > clock) return error.AuthenticationRequired;
        self.* = .{ .id = id, .expires = expires, .socket = socket, .next = first };
        self.thread = try std.Thread.spawn(.{}, watch, .{self});
        first = self;
        return self;
    }
    fn watch(self: *Lease) void {
        const io = self.socket.io.io();
        mutex.lockUncancelable(io);
        defer mutex.unlock(io);
        while (!self.stop and !self.invalid.load(.acquire)) {
            const seconds = self.expires - now(io);
            if (seconds <= 0) {
                self.invalid.store(true, .release);
                self.socket.close();
                return;
            }
            changed.waitTimeout(io, &mutex, .{ .duration = .{ .clock = .awake, .raw = .fromSeconds(@min(seconds, 86400)) } }) catch {};
        }
    }
    pub fn reject(self: *Lease) !void {
        // An active lease supplies the largest current expiry under the registry lock.
        try signal(self.id, 0, true, self.socket.io.io());
    }
    pub fn release(self: *Lease) void {
        const io = self.socket.io.io();
        mutex.lockUncancelable(io);
        var cursor = &first;
        while (cursor.*) |node| {
            if (node == self) {
                cursor.* = self.next;
                break;
            }
            cursor = &node.next;
        }
        self.stop = true;
        changed.broadcast(io);
        mutex.unlock(io);
        if (self.thread) |t| t.join();
        gpa.destroy(self);
    }
};
