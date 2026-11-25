const std = @import("std");
const core = @import("aux_core");
const w = core.wire;

pub const Request = struct {
    method: []const u8,
    url: []const u8,
    headers: w.V,
    body: ?[]const u8 = null,
};

pub const Response = struct {
    status: u16,
    headers: []const [2][]const u8 = &.{},
    body: []const u8,
};

/// A synchronous HTTP exchange. Use the provided arena for response storage.
/// Headers and decompressed response bytes remain valid until the SDK call returns.
pub const Transport = struct {
    context: ?*anyopaque = null,
    send: *const fn (?*anyopaque, w.A, Request) anyerror!Response,

    pub fn execute(self: Transport, a: w.A, input: w.V) !w.V {
        var args = input;
        const jobs = try core.dependencies(a, input);
        if (jobs.object.count() != 0 or try core.venue(input) == .derive) {
            var it = jobs.object.iterator();
            var metadata: w.V = .{ .object = .empty };
            while (it.next()) |job| try metadata.object.put(a, job.key_ptr.*, try self.prepared(a, job.value_ptr.*));
            try args.object.put(a, "metadata", metadata);
        }
        return self.prepared(a, args);
    }

    fn prepared(self: Transport, a: w.A, input: w.V) !w.V {
        const venue = try w.string(try w.field(input, "venue"));
        const operation = try w.string(try w.field(input, "operation"));
        const scope_coin = if (std.mem.eql(u8, operation, "order_status") and
            (std.mem.eql(u8, venue, "paragon") or std.mem.eql(u8, venue, "trade_xyz")))
            try w.field(try w.field(input, "params"), "coin")
        else
            null;
        const req = try core.transport.prepareDispatch(a, input);
        var value = req;
        const binary = value.object.fetchSwapRemove("body_hex");
        const transaction_hash = value.object.fetchSwapRemove("transaction_hash");
        var request = try std.json.parseFromValueLeaky(Request, a, value, .{});
        if (binary) |hex| {
            const text = try w.string(hex.value);
            const bytes = try a.alloc(u8, text.len / 2);
            request.body = try std.fmt.hexToBytes(bytes, text);
        }
        const reservation = try core.reserve(input);
        var confirmed = false;
        defer if (reservation) |r| r.finish(confirmed);
        const response = self.send(self.context, a, request) catch |err| {
            if (err == error.Network and ((std.mem.eql(u8, venue, "monaco") and core.monacoWrite(operation)) or core.perpsWrite(try core.venue(input), operation))) {
                recordPending(a, if (transaction_hash) |hash| hash.value else .null);
                return error.PendingOutcome;
            }
            return err;
        };
        confirmed = response.status >= 400 and response.status < 500 and response.status != 408;
        const fields = .{ .venue = venue, .operation = operation, .scope_coin = scope_coin, .api_key = input.object.get("api_key"), .auth = input.object.get("auth"), .status = response.status, .headers = response.headers, .body = response.body };
        const result = core.transport.finishDispatch(a, try @import("signing.zig").value(a, fields)) catch |err| {
            if (err == error.PendingOutcome) recordPending(a, if (transaction_hash) |hash| hash.value else .null);
            return err;
        };
        confirmed = true;
        return result;
    }

    pub fn evmRead(self: Transport, a: w.A, args: w.V) !w.V {
        const prepared_rpc = try core.rpc.prepare(a, args);
        var headers: w.V = .{ .object = .empty };
        try headers.object.put(a, "content-type", w.text("application/json"));
        const reply = try self.send(self.context, a, .{ .method = "POST", .headers = headers, .url = try w.string(try w.field(prepared_rpc, "url")), .body = try w.string(try w.field(prepared_rpc, "body")) });
        return core.rpc.finish(a, try @import("signing.zig").value(a, .{ .id = try w.field(prepared_rpc, "id"), .operation = try w.field(args, "operation"), .status = reply.status, .body = reply.body }));
    }
};

fn recordPending(a: w.A, hash: w.V) void {
    if (hash == .null) return;
    const bytes = std.json.Stringify.valueAlloc(a, .{ .hash = hash }, .{}) catch return;
    core.failure.record("decibel", 0, bytes);
}
