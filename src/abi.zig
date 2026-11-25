// Do not initialize a process-wide panic reporter inside the host runtime.
pub const panic = @import("std").debug.FullPanic(abort);
fn abort(_: []const u8, _: ?usize) noreturn {
    @trap();
}

comptime {
    _ = @import("core/native.zig");
    _ = @import("core/stream.zig");
    _ = @import("core/encoding.zig");
    _ = @import("data/pyth/hermes.zig");
    _ = @import("core/json.zig");
}

test "native JSON depth and exact token preservation" {
    const std = @import("std");
    const wire = @import("core/wire.zig");
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

test "Aster v3 typed data signing vector" {
    const std = @import("std");
    const aster = @import("exchanges/aster/trading.zig");
    const evm = @import("core/evm.zig");
    const actual = aster.digest("symbol=BTCUSDT&user=0x7e5f4552091a69125d5dfcb7b8c2659029395bdf&signer=0x7e5f4552091a69125d5dfcb7b8c2659029395bdf&nonce=1754450974231000");
    try std.testing.expectEqual(try evm.hex(32, "b6255e3a5fe88b27bd4b7d98fe7e8133ddf0b9811a3aeeafdbbf5827a4018a2b"), actual);
}
