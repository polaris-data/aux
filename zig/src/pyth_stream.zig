const std = @import("std");
const w = @import("wire.zig");
const Socket = @import("websocket.zig").Session;
const gpa = std.heap.c_allocator;
const eq = std.mem.eql;
const V = w.V;
const A = w.A;

fn b64(a: A, bytes: []const u8) ![]const u8 {
    const enc = std.base64.standard.Encoder;
    return enc.encode(try a.alloc(u8, enc.calcSize(bytes.len)), bytes);
}
fn binary(a: A, data: []const u8) !V {
    if (data.len <= 12 or std.mem.readInt(u32, data[0..4], .little) != 461928307) return error.InvalidFrame;
    var v = V{ .object = .empty };
    try v.object.put(a, "type", w.text("streamUpdated"));
    try v.object.put(a, "subscriptionId", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{std.mem.readInt(u64, data[4..12], .big)}) });
    try v.object.put(a, "binary", w.text(try b64(a, data)));
    var rest = data[12..];
    while (rest.len != 0) {
        if (rest.len < 2) return error.InvalidFrame;
        const n = std.mem.readInt(u16, rest[0..2], .big);
        rest = rest[2..];
        if (n < 4 or n > rest.len) return error.InvalidFrame;
        const msg = rest[0..n];
        rest = rest[n..];
        const name = switch (std.mem.readInt(u32, msg[0..4], .little)) {
            3302625434 => {
                try v.object.put(a, "parsed", try w.parse(a, msg[4..]));
                continue;
            },
            2593727018 => "evm",
            2182742457 => "solana",
            1296547300 => "leEcdsa",
            1499680012 => "leUnsigned",
            else => continue,
        };
        var format = V{ .object = .empty };
        try format.object.put(a, "encoding", w.text("base64"));
        try format.object.put(a, "data", w.text(try b64(a, msg)));
        try v.object.put(a, name, format);
    }
    return v;
}
fn less(_: void, left: []const u8, right: []const u8) bool {
    return std.mem.lessThan(u8, left, right);
}
fn sorted(a: A, v: V) anyerror!V {
    return switch (v) {
        .object => blk: {
            var out = V{ .object = .empty };
            const keys = try a.dupe([]const u8, v.object.keys());
            std.mem.sort([]const u8, keys, {}, less);
            for (keys) |k| try out.object.put(a, k, try sorted(a, v.object.get(k).?));
            break :blk out;
        },
        .array => blk: {
            var out = V{ .array = .init(a) };
            for (v.array.items) |item| try out.array.append(try sorted(a, item));
            break :blk out;
        },
        else => v,
    };
}
fn setup(a: A, p: V) !struct { frame: []const u8, id: u64 } {
    if (p != .object) return error.InvalidParameter;
    var msg = V{ .object = .empty };
    try msg.object.put(a, "type", w.text("subscribe"));
    var feed = false;
    var it = p.object.iterator();
    while (it.next()) |kv| {
        const k = kv.key_ptr.*;
        const v = kv.value_ptr.*;
        if (v == .null) continue;
        if (eq(u8, k, "priceFeedIds") or eq(u8, k, "symbols") or eq(u8, k, "formats") or eq(u8, k, "properties")) {
            if (v != .array) return error.InvalidParameter;
            for (v.array.items) |item| {
                if (eq(u8, k, "priceFeedIds")) {
                    if (item != .number_string) return error.InvalidParameter;
                    _ = try w.unsigned(u32, item);
                } else _ = try w.string(item);
            }
            if (v.array.items.len > 0 and (eq(u8, k, "priceFeedIds") or eq(u8, k, "symbols"))) feed = true;
        } else if (eq(u8, k, "channel") or eq(u8, k, "deliveryFormat") or eq(u8, k, "jsonBinaryEncoding")) {
            _ = try w.string(v);
        } else if (eq(u8, k, "ignoreInvalidFeeds") or eq(u8, k, "parsed")) {
            if (v != .bool) return error.InvalidParameter;
        } else if (!eq(u8, k, "subscriptionId")) return error.UnknownParameter;
        try msg.object.put(a, k, v);
    }
    if (!feed or !msg.object.contains("channel") or !msg.object.contains("formats")) return error.InvalidParameter;
    const id = try w.field(msg, "subscriptionId");
    if (id != .number_string) return error.InvalidParameter;
    return .{ .frame = try std.json.Stringify.valueAlloc(a, msg, .{}), .id = try w.unsigned(u64, id) };
}

const Item = struct { bytes: []u8, hash: ?[32]u8 };
const Source = struct { socket: *Socket, thread: ?std.Thread = null, item: ?Item = null, ended: bool = false, failure: ?anyerror = null };
pub const Session = struct {
    sources: []Source,
    opened: usize = 0,
    io: std.Io.Threaded = .init_single_threaded,
    mutex: std.Io.Mutex = .init,
    changed: std.Io.Condition = .init,
    canceled: std.atomic.Value(bool) = .init(false),
    finished: bool = false,
    cursor: usize = 0,
    request_id: ?u64 = null,
    hashes: std.AutoHashMapUnmanaged([32]u8, void) = .empty,
    recent: std.ArrayList([32]u8) = .empty,
    oldest: usize = 0,

    pub fn open(a: A, config: V, merkle: bool, dial: ?@import("host_stream.zig").Dial) !*Session {
        const token = try w.string(config.object.get("api_key") orelse return error.AuthenticationRequired);
        if (token.len == 0) return error.AuthenticationRequired;
        for (token) |c| if (c <= 32 or c >= 127) return error.InvalidCredential;
        const sub = if (merkle) null else try setup(a, try w.field(config, "params"));
        var urls = V{ .array = .init(a) };
        if (config.object.get("endpoints")) |v| urls = v else if (config.object.get("origin")) |v| try urls.array.append(v) else {
            for (0..3) |i| try urls.array.append(w.text(try std.fmt.allocPrint(a, "wss://pyth-lazer-{d}.dourolabs.app/v1/stream", .{i})));
        }
        if (urls != .array or urls.array.items.len == 0 or urls.array.items.len > 64) return error.InvalidEndpoint;
        const self = try gpa.create(Session);
        errdefer gpa.destroy(self);
        self.* = .{ .sources = try gpa.alloc(Source, urls.array.items.len), .request_id = if (sub) |s| s.id else null };
        errdefer self.release();
        for (urls.array.items, 0..) |v, i| {
            const url = try w.string(v);
            const endpoint = if (merkle) blk: {
                var uri = try std.Uri.parse(url);
                uri.path = .{ .raw = "/v1/merkle/root/stream" };
                break :blk try std.fmt.allocPrint(a, "{f}", .{uri});
            } else url;
            self.sources[i] = .{ .socket = try Socket.pyth(endpoint, token, if (sub) |s| s.frame else "", dial) };
            self.opened += 1;
        }
        for (self.sources) |*s| s.thread = try std.Thread.spawn(.{}, run, .{ self, s });
        return self;
    }
    fn decode(self: *Session, a: A, s: *Source, bytes: []const u8) !?Item {
        const v = if (s.socket.binary) try binary(a, bytes) else try w.parse(a, bytes);
        if (v != .object) return error.InvalidEvent;
        if (self.request_id) |id| if (v.object.get("subscriptionId")) |v_id| if (v_id != .number_string or try w.unsigned(u64, v_id) != id) return null;
        const tag = if (v.object.get("type")) |t| if (t == .string) t.string else "" else "";
        if (self.request_id != null and tag.len == 0) return error.InvalidEvent;
        if (std.ascii.findIgnoreCase(tag, "error") != null) return error.ExchangeApi;
        var hash: ?[32]u8 = null;
        if (self.request_id != null and eq(u8, tag, "streamUpdated")) {
            var digest: [32]u8 = undefined;
            std.crypto.hash.sha2.Sha256.hash(try std.json.Stringify.valueAlloc(a, try sorted(a, v), .{}), &digest, .{});
            hash = digest;
        }
        return .{ .bytes = if (s.socket.binary) try std.json.Stringify.valueAlloc(gpa, v, .{}) else try gpa.dupe(u8, bytes), .hash = hash };
    }
    fn pump(self: *Session, s: *Source) !void {
        const io = self.io.io();
        while (!self.canceled.load(.acquire)) {
            const bytes = try s.socket.subscriptionMessage() orelse return;
            defer gpa.free(bytes);
            var arena = std.heap.ArenaAllocator.init(gpa);
            defer arena.deinit();
            const item = (try self.decode(arena.allocator(), s, bytes)) orelse continue;
            self.mutex.lockUncancelable(io);
            defer self.mutex.unlock(io);
            while (s.item != null and !self.canceled.load(.acquire)) self.changed.waitUncancelable(io, &self.mutex);
            if (self.canceled.load(.acquire)) {
                gpa.free(item.bytes);
                return;
            }
            s.socket.retries = 0;
            s.item = item;
            self.changed.broadcast(io);
        }
    }
    fn run(self: *Session, s: *Source) void {
        const failure: ?anyerror = if (self.pump(s)) |_| null else |e| e;
        const io = self.io.io();
        self.mutex.lockUncancelable(io);
        defer self.mutex.unlock(io);
        s.failure = failure;
        s.ended = true;
        self.changed.broadcast(io);
    }
    pub fn next(self: *Session) !?[]u8 {
        const io = self.io.io();
        self.mutex.lockUncancelable(io);
        defer self.mutex.unlock(io);
        if (self.finished) return null;
        errdefer self.finished = true;
        scan: while (!self.canceled.load(.acquire)) {
            var live = false;
            for (0..self.sources.len) |offset| {
                const index = (self.cursor + offset) % self.sources.len;
                const s = &self.sources[index];
                if (s.item) |item| {
                    s.item = null;
                    self.cursor = (index + 1) % self.sources.len;
                    self.changed.broadcast(io);
                    errdefer gpa.free(item.bytes);
                    if (item.hash) |h| {
                        if (self.hashes.contains(h)) {
                            gpa.free(item.bytes);
                            continue :scan;
                        }
                        if (self.recent.items.len == 4096) {
                            _ = self.hashes.remove(self.recent.items[self.oldest]);
                            self.recent.items[self.oldest] = h;
                            self.oldest = (self.oldest + 1) % 4096;
                        } else try self.recent.append(gpa, h);
                        try self.hashes.put(gpa, h, {});
                    }
                    return item.bytes;
                }
                if (s.failure) |e| if (e != error.Network) return e;
                if (!s.ended) live = true;
            }
            if (!live) return error.Network;
            self.changed.waitUncancelable(io, &self.mutex);
        }
        return null;
    }
    pub fn close(self: *Session) void {
        const io = self.io.io();
        self.mutex.lockUncancelable(io);
        defer self.mutex.unlock(io);
        self.canceled.store(true, .release);
        self.changed.broadcast(io);
        for (self.sources[0..self.opened]) |s| s.socket.close();
    }
    fn release(self: *Session) void {
        self.close();
        for (self.sources[0..self.opened]) |*s| {
            if (s.thread) |t| t.join();
            if (s.item) |item| gpa.free(item.bytes);
            s.socket.destroy();
        }
        gpa.free(self.sources);
        self.recent.deinit(gpa);
        self.hashes.deinit(gpa);
    }
    pub fn destroy(self: *Session) void {
        self.release();
        gpa.destroy(self);
    }
};
