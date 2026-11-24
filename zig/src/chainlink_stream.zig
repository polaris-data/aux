const std = @import("std");
const w = @import("wire.zig");
const Event = @import("derive_stream.zig").State.Event;

pub const State = struct {
    endpoint: []const u8,
    target: []const u8,
    key: []const u8,
    secret: []const u8,

    pub fn init(a: w.A, cfg: w.V, params: w.V) !State {
        const feeds = try w.field(params, "feedIDs");
        if (params.object.count() != 1 or feeds != .array or feeds.array.items.len == 0) return error.InvalidSubscription;
        var path: std.ArrayList(u8) = .empty;
        try path.appendSlice(a, "api/v1/ws?feedIDs=");
        for (feeds.array.items, 0..) |value, i| {
            const id = try w.string(value);
            const digits = if (std.mem.startsWith(u8, id, "0x")) id[2..] else id;
            if (digits.len != 64) return error.InvalidFeed;
            for (digits) |c| if (!std.ascii.isHex(c)) return error.InvalidFeed;
            if (i > 0) try path.appendSlice(a, "%2C");
            try path.appendSlice(a, id);
        }
        const base = if (cfg.object.get("origin")) |v| try w.string(v) else "wss://ws.dataengine.chain.link/";
        const uri = try std.Uri.parse(base);
        var scratch = try a.alloc(u8, base.len + 2 * path.items.len + 8);
        @memcpy(scratch[0..path.items.len], path.items);
        const url = try uri.resolveInPlace(path.items.len, &scratch);
        return .{
            .endpoint = try std.fmt.allocPrint(a, "{f}", .{url}),
            .target = try std.fmt.allocPrint(a, "{f}", .{url.fmt(.{ .path = true, .query = true })}),
            .key = try credential(cfg, "api_key"),
            .secret = try credential(cfg, "api_secret"),
        };
    }

    fn credential(cfg: w.V, name: []const u8) ![]const u8 {
        const s = try w.string(cfg.object.get(name) orelse return error.AuthenticationRequired);
        if (s.len == 0) return error.AuthenticationRequired;
        for (s) |c| if (c < 33 or c > 126) return error.InvalidHeader;
        return s;
    }

    pub fn headers(self: State, a: w.A, io: std.Io, out: *std.ArrayList(std.http.Header)) !void {
        const ms = std.Io.Timestamp.now(io, .real).toMilliseconds();
        if (ms < 0) return error.InvalidTimestamp;
        var fields: w.V = .{ .object = .empty };
        try @import("auth.zig").chainlinkHeaders(a, &fields, self.key, self.secret, "GET", self.target, @intCast(ms));
        var it = fields.object.iterator();
        while (it.next()) |h| try out.append(a, .{ .name = h.key_ptr.*, .value = try w.string(h.value_ptr.*) });
    }

    pub fn event(_: State, a: w.A, raw: []const u8) !Event {
        const report = try w.field(try w.parse(a, raw), "report");
        _ = try w.string(try w.field(report, "fullReport"));
        return .{ .deliver = raw };
    }
};
