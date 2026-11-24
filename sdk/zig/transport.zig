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
        if (std.mem.eql(u8, try w.string(try w.field(input, "venue")), "derive")) {
            const jobs = try core.dependencies(a, input);
            var it = jobs.object.iterator();
            var metadata: w.V = .{ .object = .empty };
            while (it.next()) |job| try metadata.object.put(a, job.key_ptr.*, try self.prepared(a, job.value_ptr.*));
            try args.object.put(a, "metadata", metadata);
        }
        return self.prepared(a, args);
    }

    fn prepared(self: Transport, a: w.A, input: w.V) !w.V {
        const req = try core.transport.prepareDispatch(a, input);
        const venue = try w.string(try w.field(input, "venue"));
        const operation = try w.string(try w.field(input, "operation"));
        const request = try std.json.parseFromValueLeaky(Request, a, req, .{});
        const reservation = try core.reserve(input);
        var confirmed = false;
        defer if (reservation) |r| r.finish(confirmed);
        const response = self.send(self.context, a, request) catch |err| {
            if (err == error.Network and std.mem.eql(u8, venue, "monaco") and core.monacoWrite(operation)) return error.PendingOutcome;
            return err;
        };
        confirmed = response.status >= 400 and response.status < 500 and response.status != 408;
        const fields = .{ .venue = venue, .operation = operation, .api_key = input.object.get("api_key"), .auth = input.object.get("auth"), .status = response.status, .headers = response.headers, .body = response.body };
        const result = try core.transport.finishDispatch(a, try @import("signing.zig").value(a, fields));
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
