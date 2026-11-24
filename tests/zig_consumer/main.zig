const std = @import("std");
const aux = @import("aux");

pub fn main(init: std.process.Init) !void {
    var debug: std.heap.DebugAllocator(.{}) = .init;
    defer if (debug.deinit() == .leak) @panic("SDK allocation leak");
    const a = debug.allocator();
    const args = try init.minimal.args.toSlice(init.arena.allocator());
    const client = aux.clients.PythHermesClient.init(a, .{ .origin = args[1] });
    try std.testing.expectError(error.UnexpectedToken, aux.parseAs(u64, a, "\"42\""));
    try std.testing.expectError(error.UnexpectedToken, aux.parseAs([]const u8, a, "[65]"));
    const ids: []const []const u8 = &.{"4242424242424242424242424242424242424242424242424242424242424242"};
    var latest = try client.latest_price_updates(.{ .ids = ids, .parsed = true });
    defer latest.deinit();
    try std.testing.expectEqual(@as(u16, 200), latest.value.status);
    try std.testing.expectEqualStrings("9007199254740993", latest.value.data.object.get("price").?.number_string);
    var stream = try client.stream_price_updates(.{ .ids = ids });
    defer stream.deinit();
    var event = (try stream.next()).?;
    defer event.deinit();
    try std.testing.expectEqualStrings("9007199254740993", event.value.object.get("price").?.number_string);
    const worker = try std.Thread.spawn(.{}, waitForClose, .{&stream});
    try std.Io.sleep(init.io, .fromMilliseconds(25), .awake);
    stream.cancel();
    stream.cancel();
    worker.join();
    stream.deinit();
    try std.testing.expect((try stream.next()) == null);
    try @import("protocols.zig").run(init, client.runtime, args);
}

fn waitForClose(stream: *aux.Stream) void {
    std.debug.assert((stream.next() catch @panic("stream read failed")) == null);
}
