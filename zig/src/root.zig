// Do not initialize a process-wide panic reporter inside the host runtime.
pub const panic = @import("std").debug.FullPanic(abort);
fn abort(_: []const u8, _: ?usize) noreturn {
    @trap();
}

comptime {
    _ = @import("native.zig");
    _ = @import("stream.zig");
    _ = @import("encoding.zig");
    _ = @import("hermes.zig");
    _ = @import("json.zig");
}

test "native JSON depth and exact token preservation" {
    const std = @import("std");
    const wire = @import("wire.zig");
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const token = "123456789012345678901.00000000000000001";
    try std.testing.expectEqualStrings(token, (try wire.parse(a, token)).number_string);
    var nested: [257]u8 = undefined;
    @memset(nested[0..128], '[');
    nested[128] = '0';
    @memset(nested[129..], ']');
    try std.testing.expectError(error.TooDeep, wire.parse(a, &nested));
}
