const std = @import("std");
const allocator = std.heap.c_allocator;

const Kind = enum(u32) { object, end_object, array, end_array, string, number, boolean_true, boolean_false, json_null };
const Status = enum(u32) { ok, invalid, out_of_memory, visitor_error };
const Visitor = *const fn (*anyopaque, Kind, [*]const u8, usize) callconv(.c) bool;

// The scanner validates grammar, UTF-8 and escapes. Number text crosses the
// boundary verbatim, including exponents and values outside integer ranges.
export fn aux_v1_json_visit(input: [*]const u8, len: usize, ctx: *anyopaque, visit: Visitor) Status {
    var scanner = std.json.Scanner.initCompleteInput(allocator, input[0..len]);
    defer scanner.deinit();
    var depth: usize = 0;
    while (true) {
        const token = scanner.nextAllocMax(allocator, .alloc_if_needed, len) catch |err|
            return if (err == error.OutOfMemory) .out_of_memory else .invalid;
        defer switch (token) {
            .allocated_string, .allocated_number => |slice| allocator.free(slice),
            else => {},
        };
        const kind: Kind = switch (token) {
            .object_begin => .object,
            .object_end => .end_object,
            .array_begin => .array,
            .array_end => .end_array,
            .true => .boolean_true,
            .false => .boolean_false,
            .null => .json_null,
            .number, .allocated_number => .number,
            .string, .allocated_string => .string,
            .end_of_document => return .ok,
            else => unreachable, // nextAllocMax coalesces partial tokens.
        };
        switch (kind) {
            .array, .object => {
                depth += 1;
                if (depth >= 128) return .invalid;
            },
            .end_array, .end_object => depth -= 1,
            else => {},
        }
        const bytes: []const u8 = switch (token) {
            .number, .string => |s| s,
            .allocated_number, .allocated_string => |s| s,
            else => "",
        };
        if (!visit(ctx, kind, bytes.ptr, bytes.len)) return .visitor_error;
    }
}
