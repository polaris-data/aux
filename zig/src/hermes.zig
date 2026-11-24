const std = @import("std");
const allocator = std.heap.c_allocator;
const Bytes = std.ArrayList(u8);

const Status = enum(u32) {
    ok,
    need_input,
    invalid_request,
    invalid_json,
    vendor_error,
    limit,
    truncated,
    out_of_memory,
};

const Buffer = extern struct {
    ptr: ?[*]u8 = null,
    len: usize = 0,

    fn take(bytes: []u8) Buffer {
        return .{ .ptr = bytes.ptr, .len = bytes.len };
    }
};

pub const View = extern struct {
    ptr: ?[*]const u8 = null,
    len: usize = 0,

    fn bytes(self: View) []const u8 {
        return if (self.ptr) |ptr| ptr[0..self.len] else &.{};
    }
};

const Request = extern struct {
    ids: ?[*]const View,
    ids_len: usize,
    encoding: View,
    parsed: u8,
    ignore_invalid_price_ids: u8,
    allow_unordered: u8,
    benchmarks_only: u8,
    asset_type: View,
    query: View,
    path_number: u64,
};

fn formValue(out: *Bytes, value: []const u8) error{OutOfMemory}!void {
    const digits = "0123456789ABCDEF";
    for (value) |byte| {
        switch (byte) {
            '0'...'9', 'A'...'Z', 'a'...'z', '*', '-', '.', '_' => try out.append(allocator, byte),
            ' ' => try out.append(allocator, '+'),
            else => try out.appendSlice(allocator, &.{ '%', digits[byte >> 4], digits[byte & 15] }),
        }
    }
}

fn appendParam(out: *Bytes, path_len: usize, key: []const u8, value: []const u8) error{OutOfMemory}!void {
    try out.append(allocator, if (out.items.len == path_len) '?' else '&');
    try formValue(out, key);
    try out.append(allocator, '=');
    try formValue(out, value);
}

fn appendFlag(out: *Bytes, path_len: usize, key: []const u8, flag: u8) !void {
    switch (flag) {
        0 => {},
        1 => try appendParam(out, path_len, key, "false"),
        2 => try appendParam(out, path_len, key, "true"),
        else => return error.InvalidRequest,
    }
}

const Operation = enum(u8) { latest, stream, live, ready, feeds, at_timestamp, twaps, stake_caps };

fn requestPath(op: Operation, request: *const Request) ![]u8 {
    var path_buffer: [80]u8 = undefined;
    const path = switch (op) {
        .latest => "v2/updates/price/latest",
        .stream => "v2/updates/price/stream",
        .live => "live",
        .ready => "ready",
        .feeds => "v2/price_feeds",
        .at_timestamp => std.fmt.bufPrint(&path_buffer, "v2/updates/price/{d}", .{request.path_number}) catch unreachable,
        .twaps => std.fmt.bufPrint(&path_buffer, "v2/updates/twap/{d}/latest", .{request.path_number}) catch unreachable,
        .stake_caps => "v2/updates/publisher_stake_caps/latest",
    };
    var out: Bytes = .empty;
    errdefer out.deinit(allocator);
    try out.appendSlice(allocator, path);
    if (op == .live or op == .ready or op == .twaps) return out.toOwnedSlice(allocator);
    if (op == .feeds) {
        if (request.asset_type.ptr != null) try appendParam(&out, path.len, "asset_type", request.asset_type.bytes());
        if (request.query.ptr != null) try appendParam(&out, path.len, "query", request.query.bytes());
        return out.toOwnedSlice(allocator);
    }
    try appendFlag(&out, path.len, "allow_unordered", request.allow_unordered);
    try appendFlag(&out, path.len, "benchmarks_only", request.benchmarks_only);
    if (request.encoding.ptr != null) {
        try appendParam(&out, path.len, "encoding", request.encoding.bytes());
    }
    try appendFlag(&out, path.len, "ignore_invalid_price_ids", request.ignore_invalid_price_ids);
    try appendFlag(&out, path.len, "parsed", request.parsed);

    if (op == .stake_caps) return out.toOwnedSlice(allocator);
    const ids = request.ids orelse return error.InvalidRequest;
    if (request.ids_len == 0) return error.InvalidRequest;
    for (ids[0..request.ids_len]) |view| {
        const id = view.bytes();
        const hex = if (std.mem.startsWith(u8, id, "0x")) id[2..] else id;
        if (hex.len != 64) return error.InvalidRequest;
        for (hex) |ch| {
            if (!std.ascii.isHex(ch)) return error.InvalidRequest;
        }
        try out.append(allocator, if (out.items.len == path.len) '?' else '&');
        try out.appendSlice(allocator, "ids%5B%5D=");
        try out.appendSlice(allocator, id);
    }
    return out.toOwnedSlice(allocator);
}

export fn aux_v1_hermes_request(operation: u8, request: *const Request, out: *Buffer) Status {
    out.* = .{};
    const op = std.enums.fromInt(Operation, operation) orelse return .invalid_request;
    const path = requestPath(op, request) catch |err| {
        return if (err == error.OutOfMemory) .out_of_memory else .invalid_request;
    };
    out.* = .take(path);
    return .ok;
}

export fn aux_v1_hermes_buffer_free(buffer: Buffer) void {
    if (buffer.ptr) |ptr| allocator.free(ptr[0..buffer.len]);
}

pub const Sse = struct {
    limit: usize,
    pending: Bytes = .empty,
    event: Bytes = .empty,
    consumed: usize = 0,
    scanned: usize = 0,
    first: bool = true,
    skip_lf: bool = false,
    event_borrowed: bool = false,

    fn advance(self: *Sse, offset: usize) void {
        self.consumed = offset;
        self.scanned = offset;
        if (offset == self.pending.items.len) {
            self.pending.clearRetainingCapacity();
            self.consumed = 0;
            self.scanned = 0;
        }
    }

    fn next(self: *Sse, out: *View) Status {
        if (self.event_borrowed) {
            self.event.clearRetainingCapacity();
            self.event_borrowed = false;
        }

        while (self.consumed < self.pending.items.len) {
            if (self.skip_lf) {
                self.skip_lf = false;
                if (self.pending.items[self.consumed] == '\n') {
                    self.advance(self.consumed + 1);
                    continue;
                }
            }
            const pos = std.mem.findAnyPos(u8, self.pending.items, self.scanned, "\r\n") orelse {
                self.scanned = self.pending.items.len;
                return .need_input;
            };
            var line = self.pending.items[self.consumed..pos];
            self.skip_lf = self.pending.items[pos] == '\r';
            // Advancing can clear and poison the list's backing bytes.
            defer self.advance(pos + 1);
            if (self.first) {
                self.first = false;
                if (std.mem.startsWith(u8, line, "\xef\xbb\xbf")) line = line[3..];
            }
            if (line.len == 0 and self.event.items.len != 0) {
                const bytes = self.event.items[0 .. self.event.items.len - 1];
                out.* = .{ .ptr = bytes.ptr, .len = bytes.len };
                self.event_borrowed = true;
                return .ok;
            }
            if (std.mem.startsWith(u8, line, "data:")) {
                var data = line[5..];
                if (data.len != 0 and data[0] == ' ') data = data[1..];
                if (data.len >= self.limit - self.event.items.len) return .limit;
                self.event.appendSlice(allocator, data) catch return .out_of_memory;
                self.event.append(allocator, '\n') catch return .out_of_memory;
            }
        }
        return .need_input;
    }
};

pub export fn aux_v1_hermes_sse_create(limit: usize) ?*Sse {
    const stream = allocator.create(Sse) catch return null;
    stream.* = .{ .limit = limit };
    return stream;
}

pub export fn aux_v1_hermes_sse_destroy(stream: *Sse) void {
    stream.pending.deinit(allocator);
    stream.event.deinit(allocator);
    allocator.destroy(stream);
}

pub export fn aux_v1_hermes_sse_push(stream: *Sse, ptr: [*]const u8, len: usize) Status {
    const remaining = stream.pending.items[stream.consumed..];
    if (len > stream.limit - remaining.len) return .limit;
    if (stream.consumed != 0) {
        std.mem.copyForwards(u8, stream.pending.items, remaining);
        stream.pending.items.len = remaining.len;
        stream.scanned -= stream.consumed;
        stream.consumed = 0;
    }
    stream.pending.appendSlice(allocator, ptr[0..len]) catch return .out_of_memory;
    return .ok;
}

pub export fn aux_v1_hermes_sse_next(stream: *Sse, out: *View) Status {
    out.* = .{};
    return stream.next(out);
}

pub export fn aux_v1_hermes_sse_finish(stream: *const Sse) Status {
    if (stream.event.items.len != 0) return .truncated;
    for (stream.pending.items[stream.consumed..]) |byte| {
        if (!std.ascii.isWhitespace(byte)) return .truncated;
    }
    return .ok;
}
