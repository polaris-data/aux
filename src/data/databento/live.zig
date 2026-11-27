// Databento Live text control protocol followed by a DBN byte stream.
const std = @import("std");
const w = @import("../../core/wire.zig");
const Cursor = @import("dbn_file.zig").Cursor;
const gpa = std.heap.c_allocator;
const net = std.Io.net;
const eq = std.mem.eql;
const zstd = std.compress.zstd;
const V = w.V;

fn optional(v: V, key: []const u8) V {
    return if (v == .object) v.object.get(key) orelse .null else .null;
}
fn field(v: V) ![]const u8 {
    const s = try w.string(v);
    if (s.len == 0 or std.mem.findAny(u8, s, "|\n\r\x00") != null) return error.InvalidField;
    return s;
}
fn flag(v: V, key: []const u8) !bool {
    return switch (optional(v, key)) {
        .null => false,
        .bool => |b| b,
        else => error.ExpectedBool,
    };
}
fn milliseconds(config: V, name: []const u8, default: u64) !u64 {
    const value = optional(config, name);
    if (value == .null) return default;
    const ms = try w.unsigned(u64, value);
    if (ms == 0) return error.InvalidTimeout;
    return ms;
}
fn socketTimeout(fd: c_int, opt: u32, ms: u64) !void {
    const timeout: std.c.timeval = .{ .sec = @intCast(ms / 1000), .usec = @intCast(ms % 1000 * 1000) };
    if (std.c.setsockopt(fd, std.c.SOL.SOCKET, opt, &timeout, @sizeOf(std.c.timeval)) != 0) return error.Network;
}

fn connect(io: std.Io, gateway: []const u8, timeout_ms: u64) !net.Stream {
    const deadline = @as(i128, std.Io.Timestamp.now(io, .awake).toMilliseconds()) + timeout_ms;
    const url = try std.mem.concat(gpa, u8, &.{ "tcp://", gateway });
    defer gpa.free(url);
    const dest = try std.Uri.parse(url);
    if (dest.user != null or dest.password != null or dest.query != null or dest.fragment != null or !dest.path.isEmpty()) return error.InvalidGateway;
    const host = switch (dest.host orelse return error.InvalidGateway) {
        .raw, .percent_encoded => |s| s,
    };
    const port = dest.port orelse return error.InvalidGateway;
    const hostname = std.mem.trim(u8, host, "[]");
    if (hostname.len == 0 or std.mem.findAny(u8, hostname, "\\%\x00") != null) return error.InvalidGateway;
    const node = try gpa.dupeSentinel(u8, hostname, 0);
    defer gpa.free(node);
    var service_buf: [6]u8 = undefined;
    const service = try std.fmt.bufPrintSentinel(&service_buf, "{d}", .{port}, 0);
    var hints = std.mem.zeroes(std.c.addrinfo);
    hints.socktype = std.c.SOCK.STREAM;
    var first: ?*std.c.addrinfo = null;
    if (@backingInt(std.c.getaddrinfo(node.ptr, service.ptr, &hints, &first)) != 0) return error.Network;
    defer if (first) |x| std.c.freeaddrinfo(x);
    if (std.Io.Timestamp.now(io, .awake).toMilliseconds() >= deadline) return error.Timeout;
    var address = first;
    while (address) |entry| : (address = entry.next) {
        const peer = entry.addr orelse continue;
        const fd = std.c.socket(@intCast(entry.family), @intCast(entry.socktype), @intCast(entry.protocol));
        if (fd < 0) continue;
        var owned = true;
        defer if (owned) {
            _ = std.c.close(fd);
        };
        if (std.c.fcntl(fd, std.c.F.SETFD, @as(c_int, std.c.FD_CLOEXEC)) < 0) continue;
        const flags: c_int = @bitCast(std.c.O{ .NONBLOCK = true });
        if (std.c.fcntl(fd, std.c.F.SETFL, flags) < 0) continue;
        if (std.c.connect(fd, peer, entry.addrlen) != 0) {
            switch (std.c.errno(-1)) {
                .INPROGRESS, .INTR => {},
                else => continue,
            }
            var polls = [_]std.c.pollfd{.{ .fd = fd, .events = std.c.POLL.OUT, .revents = 0 }};
            while (true) {
                const left = deadline - std.Io.Timestamp.now(io, .awake).toMilliseconds();
                if (left <= 0) return error.Timeout;
                const n = std.c.poll(&polls, 1, @intCast(@min(left, std.math.maxInt(c_int))));
                if (n < 0 and std.c.errno(n) == .INTR) continue;
                if (n == 0) continue;
                if (n < 0) return error.Network;
                break;
            }
            var result: c_int = 0;
            var len: std.c.socklen_t = @sizeOf(c_int);
            if (std.c.getsockopt(fd, std.c.SOL.SOCKET, std.c.SO.ERROR, &result, &len) != 0 or result != 0) continue;
        }
        if (std.c.fcntl(fd, std.c.F.SETFL, @as(c_int, 0)) < 0) continue;
        owned = false;
        return .{ .socket = .{ .handle = fd, .address = std.Io.Threaded.addressFromPosix(@ptrCast(@alignCast(peer))) } };
    }
    return error.Network;
}
pub const Session = struct {
    io: std.Io.Threaded,
    socket: net.Stream,
    input: std.Io.Reader,
    input_buffer: [16385]u8,
    arena: std.heap.ArenaAllocator,
    compressed: bool,
    decoder: zstd.Decompress,
    cursor: Cursor = .{},
    reader: ?*std.Io.Reader = null,
    id: []const u8,
    meta: V = .null,
    ended: bool = false,
    read_error: ?anyerror = null,
    socket_read_failed: bool = false,
    canceled: std.atomic.Value(bool) = .init(false),

    fn line(self: *Session) ![]const u8 {
        const raw = self.input.takeDelimiterInclusive('\n') catch return if (self.socket_read_failed) error.Network else error.InvalidHandshake;
        const line_data = std.mem.trimEnd(u8, raw[0 .. raw.len - 1], "\r");
        if (!std.unicode.utf8ValidateSlice(line_data)) return error.InvalidHandshake;
        return line_data;
    }
    pub fn open(a: w.A, config: V) !*Session {
        const key = try w.field(config, "api_key");
        try @import("../../core/auth.zig").validateDatabentoKey(try w.string(key));
        const options = try w.field(config, "params");
        const dataset = try field(try w.field(options, "dataset"));
        const compression: []const u8 = if (optional(options, "compression") == .null) "none" else try field(try w.field(options, "compression"));
        if (!eq(u8, compression, "zstd") and !eq(u8, compression, "none")) return error.InvalidCompression;
        const ts = try flag(options, "ts_out");
        var extra: std.ArrayList(u8) = .empty;
        const heartbeat = optional(options, "heartbeat_interval_s");
        if (heartbeat != .null) {
            const seconds = try w.unsigned(u16, heartbeat);
            if (seconds == 0) return error.InvalidInterval;
            try extra.print(a, "|heartbeat_interval_s={d}", .{seconds});
        }
        const slow = optional(options, "slow_reader_behavior");
        if (slow != .null) {
            const value = try field(slow);
            if (!eq(u8, value, "warn") and !eq(u8, value, "skip")) return error.InvalidBehavior;
            try extra.print(a, "|slow_reader_behavior={s}", .{value});
        }
        const gateway = if (optional(options, "gateway") != .null) try field(try w.field(options, "gateway")) else blk: {
            const host = try std.ascii.allocLowerString(a, dataset);
            for (host) |*c| if (c.* == '.') {
                c.* = '-';
            };
            break :blk try std.fmt.allocPrint(a, "{s}.lsg.databento.com:13000", .{host});
        };
        const self = try gpa.create(Session);
        errdefer gpa.destroy(self);
        self.* = .{ .io = .init_single_threaded, .socket = undefined, .input = undefined, .input_buffer = undefined, .arena = .init(gpa), .compressed = eq(u8, compression, "zstd"), .decoder = undefined, .id = "" };
        errdefer self.arena.deinit();
        const timeout_ms = try milliseconds(config, "timeout_ms", 10000);
        const idle_timeout_ms = try milliseconds(config, "idle_timeout_ms", 60000);
        self.socket = try connect(self.io.io(), gateway, timeout_ms);
        errdefer self.socket.close(self.io.io());
        try socketTimeout(self.socket.socket.handle, std.c.SO.RCVTIMEO, idle_timeout_ms);
        try socketTimeout(self.socket.socket.handle, std.c.SO.SNDTIMEO, timeout_ms);
        self.input = .{ .vtable = &.{ .stream = receive }, .buffer = &self.input_buffer, .seek = 0, .end = 0 };
        _ = try self.line();
        const challenge = try self.line();
        if (!std.mem.startsWith(u8, challenge, "cram=")) return error.InvalidHandshake;
        var args: V = .{ .object = .empty };
        try args.object.put(a, "key", key);
        try args.object.put(a, "challenge", w.text(challenge[5..]));
        const auth = try @import("../../core/auth.zig").databento(a, args);
        const line_data = try std.fmt.allocPrint(a, "auth={s}|dataset={s}|encoding=dbn|compression={s}|ts_out={d}|client=aux-zig/0.0.4{s}\n", .{ auth.string, dataset, compression, @intFromBool(ts), extra.items });
        try self.write(line_data);
        const reply = try self.line();
        var fields = std.mem.splitScalar(u8, reply, '|');
        var ok = false;
        while (fields.next()) |v| {
            if (std.mem.startsWith(u8, v, "success=")) ok = eq(u8, v[8..], "1");
            if (std.mem.startsWith(u8, v, "session_id=")) self.id = try self.arena.allocator().dupe(u8, v[11..]);
        }
        if (!ok) return error.AuthenticationRequired;
        return self;
    }
    fn receive(reader: *std.Io.Reader, writer: *std.Io.Writer, limit: std.Io.Limit) std.Io.Reader.StreamError!usize {
        const self: *Session = @fieldParentPtr("input", reader);
        const dest = limit.slice(try writer.writableSliceGreedy(1));
        if (dest.len == 0) return 0;
        while (true) {
            const n = std.c.recv(self.socket.socket.handle, dest.ptr, dest.len, 0);
            if (n == 0) return error.EndOfStream;
            if (n < 0) {
                if (std.c.errno(n) == .INTR) continue;
                self.socket_read_failed = true;
                return error.ReadFailed;
            }
            writer.advance(@intCast(n));
            return @intCast(n);
        }
    }
    fn write(self: *Session, bytes: []const u8) !void {
        errdefer self.cancel();
        var pending = bytes;
        while (pending.len > 0) {
            const n = std.c.send(self.socket.socket.handle, pending.ptr, pending.len, std.c.MSG.NOSIGNAL);
            if (n <= 0) {
                if (n < 0 and std.c.errno(n) == .INTR) continue;
                return error.Network;
            }
            pending = pending[@intCast(n)..];
        }
    }
    fn subscription(self: *Session, a: w.A, p: V, unsubscribe: bool) !void {
        const schema = try field(try w.field(p, "schema"));
        const stype: []const u8 = if (optional(p, "stype_in") == .null) "raw_symbol" else try field(try w.field(p, "stype_in"));
        const symbols = try w.field(p, "symbols");
        if (symbols != .array or symbols.array.items.len == 0) return error.InvalidSymbols;
        const snapshot = try flag(p, "snapshot");
        const start = optional(p, "start");
        if (start != .null) {
            if (snapshot) return error.InvalidStart;
            _ = try w.unsigned(u64, start);
        }
        const id = optional(p, "id");
        if (id != .null) _ = try w.unsigned(u32, id);
        var out: std.ArrayList(u8) = .empty;
        for (symbols.array.items, 0..) |v, i| {
            const symbol = try field(v);
            if (std.mem.findScalar(u8, symbol, ',') != null) return error.InvalidSymbols;
            if (i % 500 == 0) try out.print(a, "{s}schema={s}|stype_in={s}|symbols=", .{ if (unsubscribe) "unsubscribe|" else "", schema, stype }) else try out.append(a, ',');
            try out.appendSlice(a, symbol);
            const last = i + 1 == symbols.array.items.len;
            if ((i + 1) % 500 == 0 or last) {
                if (!unsubscribe) try out.print(a, "|snapshot={d}", .{@intFromBool(snapshot)});
                try out.print(a, "|is_last={d}", .{@intFromBool(last)});
                if (!unsubscribe and start != .null) try out.print(a, "|start={d}", .{try w.unsigned(u64, start)});
                if (!unsubscribe and id != .null) try out.print(a, "|id={d}", .{try w.unsigned(u32, id)});
                try out.append(a, '\n');
            }
        }
        try self.write(out.items);
    }
    pub fn next(self: *Session) !?[]u8 {
        if (self.canceled.load(.acquire)) return null;
        if (self.read_error) |e| {
            self.read_error = null;
            return e;
        }
        if (self.ended) return null;
        const r = self.reader orelse return error.SessionNotStarted;
        const first = self.cursor.first;
        errdefer self.ended = true;
        const event = (self.cursor.next(r) catch |err| return if (self.socket_read_failed) error.Network else err) orelse {
            self.ended = true;
            return null;
        };
        errdefer gpa.free(event);
        if (first) {
            const a = self.arena.allocator();
            self.meta = try w.field(try w.parse(a, try a.dupe(u8, event)), "metadata");
        }
        return event;
    }
    pub fn command(self: *Session, a: w.A, input: V) !V {
        const op = try w.string(try w.field(input, "operation"));
        if (eq(u8, op, "metadata")) return self.meta;
        if (eq(u8, op, "session_id")) return w.text(self.id);
        if (self.canceled.load(.acquire) or self.ended and self.read_error == null) return error.Closed;
        if (eq(u8, op, "start_session")) {
            if (self.reader != null) return error.AlreadyStarted;
            if (self.compressed) self.decoder = zstd.Decompress.init(&self.input, try self.arena.allocator().alloc(u8, zstd.default_window_len + zstd.block_size_max), .{});
            try self.write("start_session\n");
            self.reader = if (self.compressed) &self.decoder.reader else &self.input;
        } else if (eq(u8, op, "read_batch")) {
            const n = try w.unsigned(u16, try w.field(input, "limit"));
            if (n == 0 or n > 4096 or self.reader == null) return error.InvalidBatch;
            var out = std.array_list.Managed(V).init(a);
            for (0..n) |_| {
                const event = (self.next() catch |e| {
                    if (out.items.len == 0) return e;
                    self.read_error = e;
                    break;
                }) orelse break;
                defer gpa.free(event);
                try out.append(try w.parse(a, try a.dupe(u8, event)));
            }
            return .{ .array = out };
        } else if (eq(u8, op, "subscribe") or eq(u8, op, "unsubscribe")) {
            try self.subscription(a, try w.field(input, "subscription"), eq(u8, op, "unsubscribe"));
        } else return error.InvalidOperation;
        return .null;
    }
    pub fn cancel(self: *Session) void {
        if (!self.canceled.swap(true, .acq_rel)) self.socket.shutdown(self.io.io(), .both) catch {};
    }
    pub fn destroy(self: *Session) void {
        self.cancel();
        self.socket.close(self.io.io());
        self.arena.deinit();
        gpa.destroy(self);
    }
};
