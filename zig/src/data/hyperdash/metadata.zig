// Hyperdash response headers are optional and case-insensitive.
const std = @import("std");
const w = @import("../../core/wire.zig");
const Kind = enum { text, bool, integer };
const Field = struct { header: []const u8, key: []const u8, kind: Kind };
const fields = [_]Field{
    .{ .header = "x-request-id", .key = "request_id", .kind = .text },
    .{ .header = "x-credits-charged", .key = "credits_charged", .kind = .integer },
    .{ .header = "x-credit-price-version", .key = "credit_price_version", .kind = .integer },
    .{ .header = "x-credits-low", .key = "credits_low", .kind = .bool },
    .{ .header = "x-problem-code", .key = "problem_code", .kind = .text },
    .{ .header = "retry-after", .key = "retry_after", .kind = .text },
    .{ .header = "x-has-more", .key = "has_more", .kind = .bool },
    .{ .header = "x-next-cursor", .key = "next_cursor", .kind = .text },
    .{ .header = "x-as-of", .key = "as_of", .kind = .integer },
    .{ .header = "x-complete-to", .key = "complete_to", .kind = .integer },
    .{ .header = "x-truncated", .key = "truncated", .kind = .bool },
    .{ .header = "x-max-rows", .key = "max_rows", .kind = .integer },
    .{ .header = "x-incomplete", .key = "incomplete", .kind = .bool },
};
pub fn read(a: w.A, headers: []const std.http.Header, auth: ?w.V) !w.V {
    var out: w.V = .{ .object = .empty };
    for (headers) |h| for (fields) |f| {
        if (!std.ascii.eqlIgnoreCase(h.name, f.header) or out.object.contains(f.key)) continue;
        const v: w.V = switch (f.kind) {
            .bool => if (std.mem.eql(u8, h.value, "true")) .{ .bool = true } else if (std.mem.eql(u8, h.value, "false")) .{ .bool = false } else continue,
            .integer => blk: {
                const n = std.fmt.parseInt(i128, h.value, 10) catch continue;
                break :blk .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{n}) };
            },
            .text => blk: {
                if (auth) |key| if (key == .string and key.string.len > 0) {
                    const bearer = try std.mem.concat(a, u8, &.{ "Bearer ", key.string });
                    const redacted = try std.mem.replaceOwned(u8, a, h.value, bearer, "[redacted]");
                    break :blk w.text(try std.mem.replaceOwned(u8, a, redacted, key.string, "[redacted]"));
                };
                break :blk w.text(try a.dupe(u8, h.value));
            },
        };
        try out.object.put(a, f.key, v);
        break;
    };
    return out;
}
