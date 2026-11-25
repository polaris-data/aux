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
    try exchanges(a, args[5]);
    try qfexSubscriptions(a, args[5]);
}

fn waitForClose(stream: *aux.Stream) void {
    std.debug.assert((stream.next() catch @panic("stream read failed")) == null);
}

fn exchanges(a: std.mem.Allocator, origin: []const u8) !void {
    const rest = try std.mem.concat(a, u8, &.{ origin, "/new-exchanges/variational" });
    defer a.free(rest);
    const client = aux.clients.VariationalClient.init(a, .{ .origin = rest });
    var stats = try client.market_stats(.{});
    defer stats.deinit();
    try std.testing.expectEqualStrings("perpetual", stats.value.listings[0].contract_type);
    try std.testing.expectEqualStrings("swap", stats.value.listings[1].contract_type);
    try std.testing.expect(stats.value.listings[1].funding == null);

    const colon = std.mem.findScalar(u8, origin, ':').?;
    const endpoint = try std.mem.concat(a, u8, &.{ "ws", origin[colon..], "/new-exchanges/perpl/ws/book" });
    defer a.free(endpoint);
    const perpl = aux.clients.PerplClient.init(a, .{ .endpoint = endpoint });
    var book = try perpl.subscribe(.{ .channel = "order_book", .market_id = @as(u32, 1) });
    defer book.deinit();
    var snapshot = (try book.next()).?;
    defer snapshot.deinit();
    const bids = snapshot.value.object.get("bid").?.array.items;
    try std.testing.expectEqualStrings("9007199254740993", bids[0].object.get("p").?.number_string);
}

fn qfexSubscriptions(a: std.mem.Allocator, origin: []const u8) !void {
    const colon = std.mem.findScalar(u8, origin, ':').?;
    const invalid_endpoint = try std.mem.concat(a, u8, &.{ "ws", origin[colon..], "/qfex-candles/invalid" });
    defer a.free(invalid_endpoint);
    const invalid = aux.clients.QfexClient.init(a, .{ .endpoint = invalid_endpoint });
    const symbol = "AAPL-USD";
    try std.testing.expectError(error.InvalidRequest, invalid.subscribe(.{ .channel = "candle", .symbol = symbol }));
    inline for (.{ [0][]const u8{}, .{ "1MIN", "5MINS" }, "1MIN", .{1}, .{""}, .{"2MIN"} }) |intervals| {
        try std.testing.expectError(error.InvalidRequest, invalid.subscribe(.{ .channel = "candle", .symbol = symbol, .intervals = intervals }));
    }

    const endpoint = try std.mem.concat(a, u8, &.{ "ws", origin[colon..], "/qfex-candles/zig" });
    defer a.free(endpoint);
    const client = aux.clients.QfexClient.init(a, .{ .endpoint = endpoint });
    var funding = try client.subscribe(.{ .channel = "funding", .symbol = symbol });
    defer funding.deinit();
    var alias = try client.subscribe(.{ .channel = "funding_rate", .symbol = symbol });
    defer alias.deinit();
    var one_minute = try client.subscribe(.{ .channel = "candle", .symbol = symbol, .intervals = .{"1MIN"} });
    defer one_minute.deinit();

    const funding_ack = "{\"type\":\"subscribed\",\"channels\":[\"funding\"],\"symbols\":[\"AAPL-USD\"]}";
    const funding_update = "{\"type\":\"funding\",\"symbol\":\"AAPL-USD\",\"funding_rate\":\"0.00015\",\"sequence\":1}";
    for ([_]*const aux.Stream{ &funding, &alias }) |reader| {
        try expectStreamBytes(a, reader, funding_ack);
        try expectStreamBytes(a, reader, funding_update);
    }
    try expectStreamBytes(a, &one_minute, "{\"type\":\"subscribed\",\"channels\":[\"candle\"],\"symbols\":[\"AAPL-USD\"],\"intervals\":[\"1MIN\"]}");
    try expectStreamSequence(&one_minute, "2");

    funding.deinit();
    var five_minutes = try client.subscribe(.{ .channel = "candle", .symbol = symbol, .intervals = .{"5MINS"} });
    defer five_minutes.deinit();
    try expectStreamBytes(a, &five_minutes, "{\"type\":\"subscribed\",\"channels\":[\"candle\"],\"symbols\":[\"AAPL-USD\"],\"intervals\":[\"5MINS\"]}");
    try expectStreamSequence(&five_minutes, "3");
    try expectStreamSequence(&one_minute, "4");
    try expectStreamSequence(&alias, "5");

    try expectStreamSequence(&five_minutes, "6");
    try expectStreamSequence(&one_minute, "7");
    try expectStreamSequence(&alias, "8");
    alias.deinit();
    one_minute.deinit();
    try expectStreamSequence(&five_minutes, "9");
}

fn expectStreamBytes(a: std.mem.Allocator, reader: *const aux.Stream, expected: []const u8) !void {
    const bytes = try reader.nextBytes() orelse return error.UnexpectedEndOfStream;
    defer a.free(bytes);
    try std.testing.expectEqualStrings(expected, bytes);
}

fn expectStreamSequence(reader: *const aux.Stream, expected: []const u8) !void {
    var event = try reader.next() orelse return error.UnexpectedEndOfStream;
    defer event.deinit();
    const sequence = event.value.object.get("sequence") orelse return error.MissingSequence;
    try std.testing.expectEqualStrings(expected, sequence.number_string);
}
