const std = @import("std");
const core = @import("aux_core");
const signing = @import("signing.zig");
const streams = core.stream;
const Allocator = std.mem.Allocator;
pub const Value = std.json.Value;
pub const Operation = core.native.Operation;
pub const Dial = core.Dial;

/// Owns the value and everything it references. Call deinit once.
pub fn Owned(comptime T: type) type {
    return struct {
        arena: std.heap.ArenaAllocator,
        value: T,

        pub fn deinit(self: *@This()) void {
            self.arena.deinit();
            self.* = undefined;
        }
    };
}

/// Opt-in venue error detail. Use a separate instance for concurrent calls.
pub const Diagnostic = struct {
    detail: ?Owned(Value) = null,

    pub fn deinit(self: *Diagnostic) void {
        if (self.detail) |*item| item.deinit();
        self.detail = null;
    }

    fn record(self: *Diagnostic, allocator: Allocator, bytes: []const u8) !void {
        self.deinit();
        self.detail = try parse(allocator, bytes);
    }
};

pub fn parse(allocator: Allocator, bytes: []const u8) !Owned(Value) {
    return parseAs(Value, allocator, bytes);
}

pub fn parseAs(comptime T: type, allocator: Allocator, bytes: []const u8) !Owned(T) {
    var arena = std.heap.ArenaAllocator.init(allocator);
    errdefer arena.deinit();
    const a = arena.allocator();
    const wire = try core.wire.parse(a, try a.dupe(u8, bytes));
    return .{ .arena = arena, .value = try @import("model.zig").decode(T, a, wire, .{}) };
}

/// An untyped empty literal in a request denotes an empty JSON object. A
/// typed empty slice or array still represents an empty JSON array.
pub fn paramsObject(input: anytype) if (@TypeOf(input) == @TypeOf(.{})) struct {} else @TypeOf(input) {
    if (@TypeOf(input) == @TypeOf(.{})) return .{};
    return input;
}

pub const Options = struct {
    origin: ?[]const u8 = null,
    endpoint: ?[]const u8 = null,
    endpoints: ?[]const []const u8 = null,
    timeout_ms: ?u64 = null,
    idle_timeout_ms: ?u64 = null,
    api_key: ?[]const u8 = null,
    api_secret: ?[]const u8 = null,
    chain_id: ?u64 = null,
    auth: ?Value = null,
    clob: ?Value = null,
    services: ?Value = null,
    dial: ?Dial = null,
    transport: ?@import("transport.zig").Transport = null,
    signer: ?signing.Signer = null,
    diagnostic: ?*Diagnostic = null,
};

/// Options and callback contexts are borrowed. Results have independent ownership.
pub const Client = struct {
    allocator: Allocator,
    options: Options = .{},

    fn config(self: Client, venue: []const u8, operation: []const u8, params: anytype, streaming: bool) Config(@TypeOf(paramsObject(params))) {
        const o = self.options;
        return .{ .venue = venue, .operation = operation, .params = paramsObject(params), .origin = if (streaming) o.endpoint orelse o.origin else o.origin, .endpoints = if (streaming) o.endpoints else null, .timeout_ms = if (streaming) o.timeout_ms else null, .idle_timeout_ms = if (streaming) o.idle_timeout_ms else null, .api_key = o.api_key, .api_secret = o.api_secret, .chain_id = o.chain_id, .auth = o.auth, .clob = o.clob, .services = o.services };
    }

    pub fn request(self: Client, comptime T: type, venue: []const u8, operation: []const u8, params: anytype) !Owned(T) {
        if (self.options.transport != null and std.mem.eql(u8, venue, "lighter")) {
            for ([_][]const u8{ "auth_token", "create_order", "cancel_order", "resync_nonce", "prepare_transaction" }) |workflow| {
                if (std.mem.eql(u8, operation, workflow)) {
                    if (self.options.dial == null) return error.StreamTransportRequired;
                    var native = self;
                    native.options.transport = null;
                    return native.invokeAs(T, .http_execute, self.config(venue, operation, params, false));
                }
            }
        }
        const cfg = self.config(venue, operation, params, false);
        if (self.options.signer) |signer| {
            var arena = std.heap.ArenaAllocator.init(self.allocator);
            defer arena.deinit();
            var args = try signing.value(arena.allocator(), cfg);
            try signing.apply(arena.allocator(), signer, venue, operation, &args);
            return self.invokeAs(T, .http_execute, args);
        }
        return self.invokeAs(T, .http_execute, cfg);
    }

    pub fn prepare(self: Client, venue: []const u8, operation: []const u8, params: anytype) !Owned(Value) {
        return self.invoke(.http_prepare, self.config(venue, operation, params, false));
    }

    pub fn withFields(self: Client, op: Operation, method: []const u8, fields: anytype) !Owned(Value) {
        var scratch = std.heap.ArenaAllocator.init(self.allocator);
        defer scratch.deinit();
        var args = try signing.value(scratch.allocator(), paramsObject(fields));
        if (args != .object) return error.ExpectedObject;
        try args.object.put(scratch.allocator(), "operation", .{ .string = method });
        return self.invoke(op, args);
    }

    pub fn invoke(self: Client, operation: Operation, input: anytype) !Owned(Value) {
        return self.invokeAs(Value, operation, input);
    }

    pub fn invokeAs(self: Client, comptime T: type, operation: Operation, input: anytype) !Owned(T) {
        const bytes = try std.json.Stringify.valueAlloc(self.allocator, input, .{ .emit_null_optional_fields = false });
        defer self.allocator.free(bytes);
        return self.invokeBytes(T, operation, bytes);
    }

    /// Raw text for Chainlink reports, UTF-8 JSON for other operations.
    pub fn invokeBytes(self: Client, comptime T: type, operation: Operation, bytes: []const u8) !Owned(T) {
        if (bytes.len > 34 * 1024 * 1024) return error.InputTooLarge;
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        errdefer arena.deinit();
        const a = arena.allocator();
        // Parsed strings can borrow their input. Keep it alive with the result.
        const input = try a.dupe(u8, bytes);
        var capture = core.failure.Capture{ .allocator = a };
        const previous = core.failure.current;
        if (self.options.diagnostic) |detail| detail.deinit();
        core.failure.current = if (self.options.diagnostic == null) null else &capture;
        defer core.failure.current = previous;
        const evaluation = if (self.options.transport) |transport| switch (operation) {
            .http_execute => transport.execute(a, try core.wire.parse(a, input)),
            .evm_execute_rpc => transport.evmRead(a, try core.wire.parse(a, input)),
            else => core.native.evaluate(a, operation, input, self.options.dial),
        } else core.native.evaluate(a, operation, input, self.options.dial);
        const result = evaluation catch |err| {
            if (capture.value) |detail| try self.options.diagnostic.?.record(self.allocator, detail);
            return err;
        };
        const value = try @import("model.zig").decode(T, a, result, .{});
        return .{ .arena = arena, .value = value };
    }

    pub fn open(self: Client, venue: []const u8, operation: []const u8, params: anytype) !Stream {
        if (self.options.transport != null and self.options.dial == null) return error.StreamTransportRequired;
        const bytes = try std.json.Stringify.valueAlloc(self.allocator, self.config(venue, operation, params, true), .{ .emit_null_optional_fields = false });
        defer self.allocator.free(bytes);
        var handle: ?*streams.Handle = null;
        var detail: core.native.Buffer = .{};
        const code = streams.aux_v1_stream_open_detailed(bytes.ptr, bytes.len, if (self.options.dial) |*d| d else null, &handle, &detail);
        defer release(detail);
        try self.check(code, detail);
        return .{ .client = self, .handle = handle.?, .pascal_rpc = std.mem.eql(u8, venue, "pascal") and std.mem.eql(u8, operation, "open_rpc") };
    }

    pub fn openDbn(self: Client, bytes: []const u8) !Stream {
        var handle: ?*streams.Handle = null;
        try self.check(streams.aux_v1_dbn_open(bytes.ptr, bytes.len, &handle), .{});
        return .{ .client = self, .handle = handle.? };
    }

    fn check(self: Client, code: u32, detail: core.native.Buffer) !void {
        if (self.options.diagnostic) |d| {
            d.deinit();
            if ((code == 4 or code == 10) and detail.ptr != null) try d.record(self.allocator, detail.ptr.?[0..detail.len]);
        }
        return switch (code) {
            0 => {},
            2 => error.OutOfMemory,
            3 => error.Network,
            4, 10 => error.ExchangeApi,
            5 => error.AuthenticationRequired,
            7 => error.PendingOutcome,
            8 => error.RecoverableStreamGap,
            9 => error.StreamGap,
            else => error.InvalidRequest,
        };
    }
};

fn Config(comptime P: type) type {
    return struct {
        venue: []const u8,
        operation: []const u8,
        params: P,
        origin: ?[]const u8,
        endpoints: ?[]const []const u8,
        timeout_ms: ?u64,
        idle_timeout_ms: ?u64,
        api_key: ?[]const u8,
        api_secret: ?[]const u8,
        chain_id: ?u64,
        auth: ?Value,
        clob: ?Value,
        services: ?Value,
    };
}

fn release(buffer: core.native.Buffer) void {
    if (buffer.ptr) |p| std.heap.c_allocator.free(p[0..buffer.len]);
}

/// One reader. cancel is repeatable and may interrupt a concurrent read.
/// Join users before deinit. Returned records outlive the stream.
pub const Stream = struct {
    client: Client,
    handle: ?*streams.Handle,
    pascal_rpc: bool = false,

    pub fn cancel(self: *const Stream) void {
        if (self.handle) |h| streams.aux_v1_stream_cancel(h);
    }
    pub fn deinit(self: *Stream) void {
        if (self.handle) |h| streams.aux_v1_stream_destroy(h);
        self.handle = null;
    }
    pub fn concurrentCommands(self: *const Stream) bool {
        return if (self.handle) |h| streams.aux_v1_stream_concurrent_commands(h) != 0 else false;
    }
    /// For binary streams. Free the returned slice with the client allocator.
    pub fn nextBytes(self: *const Stream) !?[]u8 {
        const h = self.handle orelse return null;
        var buffer: core.native.Buffer = .{};
        const code = streams.aux_v1_stream_next_detailed(h, &buffer);
        defer release(buffer);
        if (code == 6) return null;
        try self.client.check(code, buffer);
        return try self.client.allocator.dupe(u8, if (buffer.ptr) |p| p[0..buffer.len] else "");
    }
    pub fn next(self: *const Stream) !?Owned(Value) {
        const bytes = try self.nextBytes() orelse return null;
        defer self.client.allocator.free(bytes);
        return try parse(self.client.allocator, bytes);
    }
    /// Drain a download into a caller-owned writer. On error the writer may
    /// contain a partial download. The caller owns flushing and closing it.
    pub fn copyTo(self: *const Stream, writer: *std.Io.Writer) !usize {
        var size: usize = 0;
        while (try self.nextBytes()) |bytes| {
            defer self.client.allocator.free(bytes);
            try writer.writeAll(bytes);
            size = try std.math.add(usize, size, bytes.len);
        }
        return size;
    }

    pub fn readBatch(self: *const Stream, limit: usize) !Owned(Value) {
        return self.command(.{ .operation = "read_batch", .limit = limit });
    }

    pub fn command(self: *const Stream, input: anytype) !Owned(Value) {
        const h = self.handle orelse return error.Closed;
        const bytes = try std.json.Stringify.valueAlloc(self.client.allocator, input, .{});
        defer self.client.allocator.free(bytes);
        var reply: core.native.Buffer = .{};
        const code = streams.aux_v1_stream_command_detailed(h, bytes.ptr, bytes.len, &reply);
        defer release(reply);
        try self.client.check(code, reply);
        return parse(self.client.allocator, reply.ptr.?[0..reply.len]);
    }
    pub fn rpc(self: *const Stream, method: []const u8, params: anytype) !Owned(Value) {
        if (self.pascal_rpc and (std.mem.eql(u8, method, "place_orders") or std.mem.eql(u8, method, "cancel_orders") or std.mem.eql(u8, method, "mass_cancel"))) {
            if (self.client.options.signer) |signer| {
                var arena = std.heap.ArenaAllocator.init(self.client.allocator);
                defer arena.deinit();
                const a = arena.allocator();
                const signed = try signing.pascal(a, signer, method, try signing.value(a, params), self.client.options.auth orelse return error.AuthenticationRequired, null);
                return self.command(.{ .operation = method, .params = signed });
            }
        }
        return self.command(.{ .operation = method, .params = paramsObject(params) });
    }
};
