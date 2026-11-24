const std = @import("std");
const w = @import("wire.zig");

fn redactText(a: w.A, input: []const u8, authorization: []const u8) ![]const u8 {
    if (authorization.len == 0) return input;
    const scrubbed = try std.mem.replaceOwned(u8, a, input, authorization, "[redacted]");
    const token = if (std.mem.startsWith(u8, authorization, "Bearer ")) authorization[7..] else return scrubbed;
    if (token.len == 0) return scrubbed;
    return std.mem.replaceOwned(u8, a, scrubbed, token, "[redacted]");
}
pub fn redact(a: w.A, value: w.V, authorization: []const u8) !w.V {
    switch (value) {
        .string => |s| return w.text(try redactText(a, s, authorization)),
        .object => |obj| {
            var result: w.V = .{ .object = .empty };
            var it = obj.iterator();
            while (it.next()) |entry| try result.object.put(a, try redactText(a, entry.key_ptr.*, authorization), try redact(a, entry.value_ptr.*, authorization));
            return result;
        },
        .array => |items| {
            var result: std.array_list.Managed(w.V) = .init(a);
            for (items.items) |item| try result.append(try redact(a, item, authorization));
            return .{ .array = result };
        },
        else => return value,
    }
}
