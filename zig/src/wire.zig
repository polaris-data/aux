const std = @import("std");
pub const V = std.json.Value;
pub const A = std.mem.Allocator;
pub fn text(s: []const u8) V {
    return .{ .string = s };
}
pub fn string(v: V) ![]const u8 {
    return if (v == .string) v.string else error.ExpectedString;
}
pub fn field(v: V, key: []const u8) !V {
    if (v != .object) return error.ExpectedObject;
    return v.object.get(key) orelse error.MissingField;
}
pub fn parse(a: A, data: []const u8) !V {
    // Bound recursion before parsing or serializing an arbitrary JSON document.
    var scanner = std.json.Scanner.initCompleteInput(a, data);
    defer scanner.deinit();
    var depth: usize = 0;
    while (true) switch (try scanner.next()) {
        .object_begin, .array_begin => {
            depth += 1;
            if (depth >= 128) return error.TooDeep;
        },
        .object_end, .array_end => depth -= 1,
        .end_of_document => break,
        else => {},
    };
    return std.json.parseFromSliceLeaky(V, a, data, .{ .parse_numbers = false, .duplicate_field_behavior = .use_last });
}

pub fn unsigned(comptime T: type, value: V) !T {
    const digits = if (value == .number_string) value.number_string else try string(value);
    if (digits.len == 0) return error.InvalidQuantity;
    for (digits) |c| if (!std.ascii.isDigit(c)) return error.InvalidQuantity;
    return std.fmt.parseInt(T, digits, 10);
}
