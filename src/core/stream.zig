// Pull-based HTTP records. Cancellation can run concurrently with the single reader.
// Destruction is exclusive and occurs after all in-flight reads have returned.
const host = @import("host_stream.zig");
const std = @import("std");
const wire = @import("wire.zig");
const http = @import("transport.zig");
const sse = @import("../data/pyth/hermes.zig");
const gpa = std.heap.c_allocator;
const Buffer = @import("native.zig").Buffer;
const failure = @import("http_failure.zig");
const injective = @import("../exchanges/injective/stream.zig");

const Stream = struct {
    arena: std.heap.ArenaAllocator,
    io: host.Runtime,
    client: std.http.Client,
    req: std.http.Client.Request,
    response: std.http.Client.Response,
    socket: std.Io.net.Stream,
    reader: *std.Io.Reader,
    decompress: std.http.Decompress,
    buffer: [8192]u8,
    events: ?*sse.Sse,
    framing: @import("http.zig").Framing,
    cursor: @import("../data/databento/dbn_file.zig").Cursor,
    batch_error: ?anyerror,
    grpc: ?injective.Decoder,
    canceled: std.atomic.Value(bool) = .init(false),
    finished: bool = false,

    fn open(config: []const u8, dial: ?host.Dial) !*Stream {
        if (config.len > 1024 * 1024) return error.InvalidConfig;
        const self = try gpa.create(Stream);
        errdefer gpa.destroy(self);
        self.arena = .init(gpa);
        errdefer self.arena.deinit();
        const a = self.arena.allocator();
        // The request and headers borrow from this document throughout the stream.
        const args = try wire.parse(a, try a.dupe(u8, config));
        self.io = .{ .dial = dial };
        self.client = .{ .io = self.io.io(), .allocator = gpa };
        errdefer self.client.deinit();
        const protocol = @import("http.zig");
        self.framing = protocol.streamFraming(try protocol.venue(args), try wire.string(try wire.field(args, "operation"))) orelse return error.InvalidOperation;
        self.cursor = .{};
        self.batch_error = null;
        const params = try wire.field(args, "params");
        if (params != .object) return error.InvalidConfig;
        const compression = params.object.get("compression");
        self.grpc = if (self.framing == .grpc_injective) .{ .sub = try injective.Subscription.init(params) } else null;
        self.req = if (self.grpc) |decoder| try decoder.sub.open(a, args, &self.client) else try http.send(a, args, &self.client, true);
        errdefer self.req.deinit();
        self.response = self.req.receiveHead(&.{}) catch return error.Network;
        const status = @backingInt(self.response.head.status);
        if (status < 200 or status >= 300) {
            // Do not drain an oversized error response during request cleanup.
            if (self.req.connection) |connection| connection.closing = true;
            failure.recordStreamResponse(&self.response, self.req.extra_headers);
            return error.ExchangeApi;
        }
        self.socket = self.req.connection.?.stream_reader.stream;
        if (self.framing == .lines and compression != null and self.response.head.content_encoding == .identity and std.mem.eql(u8, try wire.string(compression.?), "zstd")) {
            // Reference responses can omit Content-Encoding for requested Zstd data.
            var it = self.response.head.iterateHeaders();
            while (it.next()) |h| {
                if (std.ascii.eqlIgnoreCase(h.name, "content-encoding")) break;
            } else self.response.head.content_encoding = .zstd;
        }
        self.finished = if (self.grpc) |*state| try state.responseHead(a, self.response.head) else false;
        self.reader = if (self.framing == .bytes or self.framing == .download or self.framing == .dataset) self.response.reader(&self.buffer) else try http.bodyReader(a, &self.response, &self.buffer, &self.decompress);
        self.events = if (self.framing == .sse)
            sse.aux_v1_hermes_sse_create(16 * 1024 * 1024) orelse return error.OutOfMemory
        else
            null;
        self.canceled = .init(false);
        return self;
    }
    fn nextObject(self: *Stream) !?[]u8 {
        var record: std.ArrayList(u8) = .empty;
        defer record.deinit(gpa);
        var depth: usize = 0;
        var quoted = false;
        var escape = false;
        while (!self.canceled.load(.acquire)) {
            const bytes = self.reader.peekGreedy(1) catch |err| {
                if (self.canceled.load(.acquire)) break;
                if (err != error.EndOfStream) return error.Network;
                if (depth != 0) return error.Truncated;
                break;
            };
            var start: usize = 0;
            if (depth == 0) {
                while (start < bytes.len and std.ascii.isWhitespace(bytes[start])) start += 1;
                if (start != bytes.len and bytes[start] != '{') return error.InvalidEvent;
            }
            var end = start;
            while (end < bytes.len) {
                const byte = bytes[end];
                end += 1;
                if (quoted) {
                    if (escape) escape = false else if (byte == '\\') escape = true else if (byte == '"') quoted = false;
                } else switch (byte) {
                    '"' => quoted = true,
                    '{', '[' => depth += 1,
                    '}', ']' => {
                        depth -= 1;
                        if (depth == 0) break;
                    },
                    else => {},
                }
            }
            if (record.items.len + end - start > 16 * 1024 * 1024) return error.RecordTooLarge;
            try record.appendSlice(gpa, bytes[start..end]);
            self.reader.toss(end);
            if (depth == 0 and record.items.len > 0) {
                var scratch = std.heap.ArenaAllocator.init(gpa);
                defer scratch.deinit();
                _ = try wire.parse(scratch.allocator(), record.items);
                return try record.toOwnedSlice(gpa);
            }
        }
        self.finished = true;
        return null;
    }
    fn nextLine(self: *Stream) !?[]u8 {
        var line: std.ArrayList(u8) = .empty;
        defer line.deinit(gpa);
        while (!self.canceled.load(.acquire)) {
            const bytes = self.reader.peekGreedy(1) catch |e| {
                if (e != error.EndOfStream) return error.Network;
                self.finished = true;
                if (line.items.len == 0) return null;
                break;
            };
            const newline = std.mem.findScalar(u8, bytes, '\n');
            const n = newline orelse bytes.len;
            if (n + line.items.len > 16 * 1024 * 1024) return error.RecordTooLarge;
            try line.appendSlice(gpa, bytes[0..n]);
            self.reader.toss(n + @intFromBool(newline != null));
            if (newline != null) break;
        }
        if (self.canceled.load(.acquire)) return null;
        var arena = std.heap.ArenaAllocator.init(gpa);
        defer arena.deinit();
        if (self.framing == .hosted_feed or self.framing == .machine_feed) return try gpa.dupe(u8, try @import("../data/tardis/replay.zig").line(arena.allocator(), line.items, self.framing == .hosted_feed));
        _ = try wire.parse(arena.allocator(), line.items);
        return try line.toOwnedSlice(gpa);
    }
    fn save(self: *Stream, input: wire.V) !wire.V {
        if (self.framing != .download and self.framing != .dataset) return error.InvalidOperation;
        const path = try wire.string(try wire.field(input, "path"));
        if (path.len == 0 or std.mem.findScalar(u8, path, 0) != null) return error.InvalidPath;
        const io = self.io.io();
        var file = try std.Io.Dir.cwd().createFileAtomic(io, path, .{});
        defer file.deinit(io);
        var size: u64 = 0;
        while (try self.next()) |chunk| {
            defer gpa.free(chunk);
            try file.file.writeStreamingAll(io, chunk);
            size = try std.math.add(u64, size, chunk.len);
        }
        if (self.canceled.load(.acquire)) return error.Canceled;
        if (self.response.head.content_length) |length| if (length != size) return error.Truncated;
        try file.file.sync(io);
        const count = std.math.cast(i64, size) orelse return error.Overflow;
        try file.link(io);
        return .{ .integer = count };
    }

    fn command(self: *Stream, a: wire.A, input: wire.V) !wire.V {
        if (std.mem.eql(u8, try wire.string(try wire.field(input, "operation")), "save")) return self.save(input);
        return self.batch(a, input);
    }

    fn batch(self: *Stream, a: wire.A, input: wire.V) !wire.V {
        if (self.framing != .dbn and self.framing != .lines and self.framing != .hosted_feed and self.framing != .machine_feed) return error.InvalidOperation;
        return batchNext(self, a, input);
    }
    fn close(self: *Stream) void {
        if (!self.canceled.swap(true, .acq_rel)) self.socket.shutdown(self.io.io(), .both) catch {};
    }
    fn destroy(self: *Stream) void {
        self.close();
        if (self.events) |events| sse.aux_v1_hermes_sse_destroy(events);
        self.req.deinit();
        self.client.deinit();
        self.arena.deinit();
        gpa.destroy(self);
    }
    fn next(self: *Stream) !?[]u8 {
        if (self.batch_error) |e| {
            self.batch_error = null;
            return e;
        }
        if (self.finished or self.canceled.load(.acquire)) return null;
        errdefer self.finished = true;
        switch (self.framing) {
            .grpc_injective => return self.grpc.?.next(self.reader, self.arena.allocator()),
            .objects => return self.nextObject(),
            .hosted_feed, .machine_feed, .lines => return self.nextLine(),
            .dbn => return self.cursor.next(self.reader),
            .bytes, .download, .dataset => {
                const bytes = self.reader.peekGreedy(1) catch |e| return if (e == error.EndOfStream) null else error.Network;
                const out = try gpa.dupe(u8, bytes);
                self.reader.toss(bytes.len);
                return out;
            },
            .sse => {},
        }
        const events = self.events.?;
        while (!self.canceled.load(.acquire)) {
            var view: sse.View = .{};
            switch (sse.aux_v1_hermes_sse_next(events, &view)) {
                .ok => {
                    var scratch = std.heap.ArenaAllocator.init(gpa);
                    defer scratch.deinit();
                    const raw = view.ptr.?[0..view.len];
                    _ = try wire.parse(scratch.allocator(), raw);
                    return try gpa.dupe(u8, raw);
                },
                .need_input => {},
                else => return error.InvalidEvent,
            }
            const available = self.reader.peekGreedy(1) catch |err| {
                if (self.canceled.load(.acquire)) break;
                if (err != error.EndOfStream) return error.Network;
                if (sse.aux_v1_hermes_sse_finish(events) != .ok) return error.Truncated;
                break;
            };
            // Stop at a line boundary before draining events to respect the
            // per-record bound even when the HTTP reader coalesces messages.
            const n = if (std.mem.findAny(u8, available, "\r\n")) |i| i + 1 else available.len;
            if (sse.aux_v1_hermes_sse_push(events, available.ptr, n) != .ok) return error.InvalidEvent;
            self.reader.toss(n);
        }
        self.finished = true;
        return null;
    }
};

pub fn batchNext(self: anytype, a: wire.A, input: wire.V) !wire.V {
    if (!std.mem.eql(u8, try wire.string(try wire.field(input, "operation")), "read_batch")) return error.InvalidOperation;
    const n = try wire.unsigned(u16, try wire.field(input, "limit"));
    if (n == 0 or n > 4096) return error.InvalidBatch;
    var list = std.array_list.Managed(wire.V).init(a);
    for (0..n) |_| {
        const raw = (self.next() catch |e| {
            if (list.items.len == 0) return e;
            self.batch_error = e;
            break;
        }) orelse break;
        defer gpa.free(raw);
        try list.append(try wire.parse(a, try a.dupe(u8, raw)));
    }
    return .{ .array = list };
}

const Socket = @import("websocket.zig").Session;
const Replay = struct {
    dial: ?host.Dial,
    arena: std.heap.ArenaAllocator,
    input: wire.V,
    offset: u64 = 0,
    count: u64,
    current: ?*Stream = null,
    mutex: std.Io.Mutex = .init,
    io: std.Io.Threaded = .init_single_threaded,
    canceled: std.atomic.Value(bool) = .init(false),
    done: bool = false,
    batch_error: ?anyerror = null,

    fn open(config: []const u8, dial: ?host.Dial) !*Replay {
        const self = try gpa.create(Replay);
        errdefer gpa.destroy(self);
        self.* = .{ .arena = .init(gpa), .input = undefined, .count = undefined, .dial = dial };
        errdefer self.arena.deinit();
        const a = self.arena.allocator();
        self.input = try wire.parse(a, try a.dupe(u8, config));
        var p = try wire.field(self.input, "params");
        const tardis = @import("../data/tardis/replay.zig");
        const start = try tardis.minute(try wire.string(try wire.field(p, "from")));
        const end = try tardis.minute(try wire.string(try wire.field(p, "to")));
        if (end <= start) return error.InvalidDate;
        self.count = end - start;
        for ([_]([]const u8){ "to", "offset", "sliceSize" }) |k| _ = p.object.swapRemove(k);
        _ = try tardis.feedPath(a, p);
        try self.input.object.put(a, "params", p);
        try self.input.object.put(a, "operation", wire.text("data_feeds"));
        return self;
    }
    fn close(self: *Replay) void {
        self.mutex.lockUncancelable(self.io.io());
        defer self.mutex.unlock(self.io.io());
        self.canceled.store(true, .release);
        if (self.current) |stream| stream.close();
    }
    fn destroy(self: *Replay) void {
        self.close();
        if (self.current) |stream| stream.destroy();
        self.arena.deinit();
        gpa.destroy(self);
    }
    fn next(self: *Replay) !?[]u8 {
        if (self.batch_error) |e| {
            self.batch_error = null;
            return e;
        }
        if (self.done) return null;
        errdefer self.done = true;
        while (self.offset < self.count and !self.canceled.load(.acquire)) {
            if (self.current == null) {
                var arena = std.heap.ArenaAllocator.init(gpa);
                defer arena.deinit();
                const a = arena.allocator();
                const param = self.input.object.getPtr("params").?;
                try param.object.put(self.arena.allocator(), "offset", .{ .integer = @intCast(self.offset) });
                const stream = try Stream.open(try std.json.Stringify.valueAlloc(a, self.input, .{}), self.dial);
                self.mutex.lockUncancelable(self.io.io());
                defer self.mutex.unlock(self.io.io());
                self.current = stream;
                if (self.canceled.load(.acquire)) {
                    stream.close();
                    return null;
                }
            }
            if (try self.current.?.next()) |data| return data;
            self.mutex.lockUncancelable(self.io.io());
            const old = self.current.?;
            self.current = null;
            self.mutex.unlock(self.io.io());
            old.destroy();
            self.offset += 1;
        }
        self.done = true;
        return null;
    }
};
const InjectiveStream = struct {
    config: []const u8,
    dial: ?host.Dial,
    current: ?*Stream,
    mutex: std.Io.Mutex = .init,
    canceled: std.atomic.Value(bool) = .init(false),
    retries: u8 = 0,
    finished: bool = false,
    rejection: ?[]u8 = null,

    fn open(config: []const u8, dial: ?host.Dial) !*InjectiveStream {
        const result = try gpa.create(InjectiveStream);
        errdefer gpa.destroy(result);
        const owned = try gpa.dupe(u8, config);
        errdefer gpa.free(owned);
        result.* = .{ .config = owned, .dial = dial, .current = try Stream.open(config, dial) };
        return result;
    }

    fn replace(self: *InjectiveStream, replacement: ?*Stream) void {
        var runtime: host.Runtime = .{};
        self.mutex.lockUncancelable(runtime.io());
        defer self.mutex.unlock(runtime.io());
        if (self.current) |stream| stream.destroy();
        self.current = replacement;
        if (self.canceled.load(.acquire)) if (replacement) |stream| stream.close();
    }

    fn next(self: *InjectiveStream) !?[]u8 {
        if (self.finished) return null;
        errdefer self.finished = true;
        while (!self.canceled.load(.acquire)) {
            if (self.current == null) {
                if (self.retries == 3) return error.Network;
                self.retries += 1;
                self.replace(Stream.open(self.config, self.dial) catch |err| switch (err) {
                    error.Network, error.Timeout => continue,
                    else => return err,
                });
            }
            const stream = self.current.?;
            const item = stream.next() catch |err| switch (err) {
                error.Network, error.Timeout, error.Truncated => {
                    self.replace(null);
                    continue;
                },
                else => {
                    if (stream.grpc.?.rejection) |frame| self.rejection = try gpa.dupe(u8, frame);
                    return err;
                },
            };
            if (item) |data| {
                self.retries = 0;
                return data;
            }
            self.finished = true;
            return null;
        }
        return null;
    }

    fn close(self: *InjectiveStream) void {
        self.canceled.store(true, .release);
        var runtime: host.Runtime = .{};
        self.mutex.lockUncancelable(runtime.io());
        defer self.mutex.unlock(runtime.io());
        if (self.current) |stream| stream.close();
    }

    fn destroy(self: *InjectiveStream) void {
        self.close();
        if (self.current) |stream| stream.destroy();
        if (self.rejection) |frame| gpa.free(frame);
        gpa.free(self.config);
        gpa.destroy(self);
    }
};

pub const Handle = union(enum) { rpc: *@import("command_session.zig").Session, shared: *@import("shared_stream.zig").Reader, hyperdash: *@import("../data/hyperdash/stream.zig").Session, pyth: *@import("../data/pyth/pro_stream.zig").Session, replay: *Replay, machine: *@import("../data/tardis/replay.zig").Machine, trading: *@import("../exchanges/qfex/trading.zig").Session, http_stream: *Stream, injective: *InjectiveStream, websocket: *Socket, live: *@import("../data/databento/live.zig").Session, dbn: *@import("../data/databento/dbn_file.zig").File };

fn open(config: []const u8, dial: ?host.Dial) !*Handle {
    if (config.len > 1024 * 1024) return error.ConfigTooLarge;
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();
    const input = try wire.parse(arena.allocator(), config);
    const venue = try wire.string(try wire.field(input, "venue"));
    const previous_config = if (failure.current) |c| c.stream_config else null;
    if (failure.current) |c| c.stream_config = input;
    defer if (failure.current) |c| {
        c.stream_config = previous_config;
    };
    const operation = try wire.string(try wire.field(input, "operation"));
    const handle = try gpa.create(Handle);
    errdefer gpa.destroy(handle);
    if (std.mem.eql(u8, venue, "pyth_pro") and (std.mem.eql(u8, operation, "subscribe") or std.mem.eql(u8, operation, "stream_merkle_roots"))) {
        handle.* = .{ .pyth = try @import("../data/pyth/pro_stream.zig").Session.open(arena.allocator(), input, std.mem.eql(u8, operation, "stream_merkle_roots"), dial) };
    } else if (std.mem.eql(u8, venue, "tardis") and std.mem.eql(u8, operation, "replay")) {
        handle.* = .{ .replay = try Replay.open(config, dial) };
    } else if (std.mem.eql(u8, venue, "tardis") and (std.mem.eql(u8, operation, "ws_replay") or std.mem.eql(u8, operation, "ws_replay_normalized") or std.mem.eql(u8, operation, "stream_normalized"))) {
        const p = try wire.field(input, "params");
        if (p != .object) return error.InvalidConfig;
        handle.* = .{ .machine = try @import("../data/tardis/replay.zig").Machine.open(arena.allocator(), input, operation, p, dial) };
    } else if (std.mem.eql(u8, venue, "hyperdash") and std.mem.eql(u8, operation, "open_stream")) {
        handle.* = .{ .hyperdash = try @import("../data/hyperdash/stream.zig").Session.open(arena.allocator(), input, dial) };
    } else if (std.mem.eql(u8, operation, "open_rpc")) {
        handle.* = .{ .rpc = try @import("command_session.zig").Session.open(arena.allocator(), input, dial) };
    } else if (std.mem.eql(u8, venue, "qfex") and std.mem.eql(u8, operation, "trading_session")) {
        handle.* = .{ .trading = try @import("../exchanges/qfex/trading.zig").Session.open(arena.allocator(), input, dial) };
    } else if ((@import("perps_stream.zig").venue(venue) != null or @import("exchange_stream.zig").venue(venue) != null or std.mem.eql(u8, venue, "n1")) and std.mem.eql(u8, operation, "subscribe")) {
        handle.* = .{ .websocket = try Socket.open(config, dial) };
    } else if ((std.mem.eql(u8, venue, "arcus") or std.mem.eql(u8, venue, "chainlink_streams") or std.mem.eql(u8, venue, "monaco") or std.mem.eql(u8, venue, "hyperliquid") or std.mem.eql(u8, venue, "paragon") or std.mem.eql(u8, venue, "trade_xyz") or std.mem.eql(u8, venue, "aster") or std.mem.eql(u8, venue, "nado") or std.mem.eql(u8, venue, "ondo_perps") or std.mem.eql(u8, venue, "pacifica") or std.mem.eql(u8, venue, "lighter") or std.mem.eql(u8, venue, "polymarket") or std.mem.eql(u8, venue, "polymarket_perps") or std.mem.eql(u8, venue, "extended") or std.mem.eql(u8, venue, "pascal") or std.mem.eql(u8, venue, "kalshi") or std.mem.eql(u8, venue, "paradex") or std.mem.eql(u8, venue, "qfex") or std.mem.eql(u8, venue, "phoenix") or std.mem.eql(u8, venue, "derive")) and std.mem.eql(u8, operation, "subscribe")) {
        const shared = @import("shared_stream.zig");
        handle.* = if (dial == null and shared.eligible(venue, try wire.field(input, "params")))
            .{ .shared = try shared.Reader.open(arena.allocator(), config, input) }
        else
            .{ .websocket = try Socket.open(config, dial) };
    } else if (std.mem.eql(u8, venue, "injective") and std.mem.eql(u8, operation, "subscribe")) {
        handle.* = .{ .injective = try InjectiveStream.open(config, dial) };
    } else if (std.mem.eql(u8, venue, "databento") and std.mem.eql(u8, operation, "connect")) {
        if (dial != null) return error.UnsupportedTransport;
        handle.* = .{ .live = try @import("../data/databento/live.zig").Session.open(arena.allocator(), input) };
    } else handle.* = .{ .http_stream = try Stream.open(config, dial) };
    return handle;
}

fn errorStatus(err: anyerror) u32 {
    return switch (err) {
        error.OutOfMemory => 2,
        error.Network, error.Timeout => 3,
        error.PendingOutcome => 7,
        error.RecoverableStreamGap => 8,
        error.StreamGap, error.StreamOverflow => 9,
        error.ExchangeApi => 4,
        error.AuthenticationRequired => 5,
        else => 1,
    };
}
export fn aux_v1_stream_open(ptr: [*]const u8, len: usize, out: *?*Handle) u32 {
    return aux_v1_stream_open_with_transport(ptr, len, null, out);
}
export fn aux_v1_stream_open_with_transport(ptr: [*]const u8, len: usize, dial: ?*const host.Dial, out: *?*Handle) u32 {
    out.* = open(ptr[0..len], if (dial) |d| d.* else null) catch |err| {
        out.* = null;
        return errorStatus(err);
    };
    return 0;
}
pub export fn aux_v1_stream_open_detailed(ptr: [*]const u8, len: usize, dial: ?*const host.Dial, out: *?*Handle, detail: *Buffer) u32 {
    detail.* = .{};
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();
    var capture = failure.Capture{ .allocator = arena.allocator() };
    const previous = failure.current;
    failure.current = &capture;
    defer failure.current = previous;
    const code = aux_v1_stream_open_with_transport(ptr, len, dial, out);
    if (code == 4) if (capture.value) |body| {
        const bytes = gpa.dupe(u8, body) catch return 2;
        detail.* = .{ .ptr = bytes.ptr, .len = bytes.len };
    };
    return code;
}
pub export fn aux_v1_dbn_open(ptr: [*]const u8, len: usize, out: *?*Handle) u32 {
    out.* = null;
    const handle = gpa.create(Handle) catch return 2;
    handle.* = .{ .dbn = @import("../data/databento/dbn_file.zig").File.open(ptr[0..len]) catch |e| {
        gpa.destroy(handle);
        return errorStatus(e);
    } };
    out.* = handle;
    return 0;
}
pub export fn aux_v1_stream_concurrent_commands(handle: *const Handle) u8 {
    return @intFromBool(handle.* == .hyperdash);
}
pub export fn aux_v1_stream_next_detailed(handle: *Handle, out: *Buffer) u32 {
    const code = aux_v1_stream_next(handle, out);
    if (code != 4) return code;
    // Reads are serialized. Terminal errors retain the frame until handle destruction.
    const detail: ?[]const u8 = switch (handle.*) {
        inline .websocket, .shared, .injective => |s| s.rejection,
        else => null,
    };
    const bytes = gpa.dupe(u8, detail orelse return code) catch return 2;
    out.* = .{ .ptr = bytes.ptr, .len = bytes.len };
    return 10;
}
export fn aux_v1_stream_next(handle: *Handle, out: *Buffer) u32 {
    out.* = .{};
    const result = switch (handle.*) {
        inline .http_stream, .injective => |s| s.next(),
        .shared => |s| s.next(),
        .websocket => |s| s.next(),
        .pyth => |s| s.next(),
        .replay => |s| s.next(),
        .machine => |s| s.next(),
        .hyperdash => |s| s.next(),
        inline .rpc, .trading => |s| s.next(),
        .dbn => |s| s.next(),
        .live => |s| s.next(),
    };
    const bytes = (result catch |err| {
        const canceled = switch (handle.*) {
            inline .http_stream, .injective => |s| s.canceled.load(.acquire),
            .shared => false,
            .websocket => |s| s.canceled.load(.acquire),
            .hyperdash => |s| s.ws.canceled.load(.acquire),
            inline .rpc, .trading => |s| s.ws.canceled.load(.acquire),
            .pyth => |s| s.canceled.load(.acquire),
            .replay => |s| s.canceled.load(.acquire),
            .machine => |s| s.ws.canceled.load(.acquire),
            .dbn => false,
            .live => |s| s.canceled.load(.acquire),
        };
        return if (canceled and err != error.AuthenticationRequired) 6 else errorStatus(err);
    }) orelse return 6;
    out.* = .{ .ptr = bytes.ptr, .len = bytes.len };
    return 0;
}
pub export fn aux_v1_stream_cancel(handle: *Handle) void {
    switch (handle.*) {
        .pyth => |s| s.close(),
        .replay => |s| s.close(),
        .machine => |s| s.ws.close(),
        inline .http_stream, .injective => |s| s.close(),
        .shared => |s| s.close(),
        .websocket => |s| s.close(),
        .hyperdash => |s| s.close(),
        inline .rpc, .trading => |s| s.close(),
        .dbn => |s| s.canceled.store(true, .release),
        .live => |s| s.cancel(),
    }
}
pub export fn aux_v1_stream_destroy(handle: *Handle) void {
    switch (handle.*) {
        .pyth => |s| s.destroy(),
        .replay => |s| s.destroy(),
        .machine => |s| s.destroy(),
        inline .http_stream, .injective => |s| s.destroy(),
        .shared => |s| s.destroy(),
        .websocket => |s| s.destroy(),
        .hyperdash => |s| s.destroy(),
        inline .rpc, .trading => |s| s.destroy(),
        .dbn => |s| s.destroy(),
        .live => |s| s.destroy(),
    }
    gpa.destroy(handle);
}

// Commands and reads are serialized by the consumer. Cancellation can run concurrently.
export fn aux_v1_stream_command(handle: *Handle, ptr: [*]const u8, len: usize, out: *Buffer) u32 {
    return streamCommand(handle, ptr, len, out, false);
}
pub export fn aux_v1_stream_command_detailed(handle: *Handle, ptr: [*]const u8, len: usize, out: *Buffer) u32 {
    return streamCommand(handle, ptr, len, out, true);
}
fn streamCommand(handle: *Handle, ptr: [*]const u8, len: usize, out: *Buffer, detailed: bool) u32 {
    out.* = .{};
    if (len > 1024 * 1024) return 1;
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();
    const a = arena.allocator();
    const input = wire.parse(a, ptr[0..len]) catch |e| return errorStatus(e);
    var detail: wire.V = .null;
    const result = switch (handle.*) {
        .replay => |s| batchNext(s, a, input),
        .machine => |s| s.command(a, input),
        .live => |session| session.command(a, input),
        .hyperdash => |s| s.command(a, input),
        .rpc => |s| if (detailed) s.commandDetailed(a, input, &detail) else s.command(a, input),
        .trading => |s| s.command(a, input),
        .http_stream => |s| s.command(a, input),
        else => return 1,
    };
    const value = result catch |e| {
        if (detail == .null) return errorStatus(e);
        const bytes = std.json.Stringify.valueAlloc(gpa, detail, .{}) catch return 2;
        out.* = .{ .ptr = bytes.ptr, .len = bytes.len };
        return 10;
    };
    const bytes = std.json.Stringify.valueAlloc(gpa, value, .{}) catch return 2;
    out.* = .{ .ptr = bytes.ptr, .len = bytes.len };
    return 0;
}
