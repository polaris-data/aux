// Each checked-out client has one owner. std.http performs the actual origin
// match (scheme, host, port) and evicts a connection rejected by the peer.
const std = @import("std");
const Slot = struct {
    busy: std.atomic.Value(bool) = .init(false),
    runtime: std.Io.Threaded = .init_single_threaded,
    client: ?std.http.Client = null,
    last_used: i64 = 0,
};
var slots: [8]Slot = @splat(.{});

pub const Lease = struct {
    client: *std.http.Client,
    slot: *Slot,

    pub fn release(self: Lease) void {
        self.slot.last_used = std.Io.Timestamp.now(self.client.io, .awake).toSeconds();
        self.slot.busy.store(false, .release);
    }
};

pub fn acquire(url: []const u8) ?Lease {
    const authority = std.mem.find(u8, url, "://") orelse return null;
    const end = std.mem.findScalarPos(u8, url, authority + 3, '/') orelse url.len;
    const first = std.hash.Wyhash.hash(0, url[0..end]) % slots.len;
    for (0..slots.len) |index| {
        const slot = &slots[(first + index) % slots.len];
        if (slot.busy.cmpxchgStrong(false, true, .acq_rel, .monotonic) != null) continue;
        const io = slot.runtime.io();
        const now = std.Io.Timestamp.now(io, .awake).toSeconds();
        if (slot.client) |*client| {
            if (now - slot.last_used >= 30) {
                client.deinit();
                slot.client = null;
            }
        }
        if (slot.client == null) slot.client = .{ .io = io, .allocator = std.heap.c_allocator, .connection_pool = .{ .free_size = 1 } };
        slot.client.?.now = null;
        return .{ .client = &slot.client.?, .slot = slot };
    }
    return null;
}
