const std = @import("std");
const aux = @import("aux");
const t = std.testing;
const A = std.mem.Allocator;
const gains = aux.clients.gains;
const synfutures = aux.clients.synfutures;
const instrument = "0xec6c44e704eb1932ec5fe1e4aba58db6fee71460";

pub fn run(a: A, origin: []const u8) !void {
    try gainsFeeds(a, origin);
    try graphPages(a, origin);
    try baseLogs(a, origin);
}

fn gainsFeeds(a: A, origin: []const u8) !void {
    const rest = aux.clients.GainsClient.init(a, .{ .origin = origin });
    var markets = try rest.markets(.{});
    defer markets.deinit();
    try t.expectEqualStrings("BTC/USD", markets.value.object.get("data").?.array.items[0].object.get("symbol").?.string);

    const spec = .{ .subscriptions = .{
        .{ .channel = "prices" },
        .{ .channel = "book", .market = "BTC/USD" },
        .{ .channel = "trades", .market = "BTC/USD" },
    } };
    const defaults = aux.clients.GainsClient.init(a, .{});
    var default_plan = try defaults.prepareSubscription(spec);
    defer default_plan.deinit();
    try t.expectEqualStrings("wss://api.gains.trade/v1/arbitrum/ws", default_plan.value.endpoint);
    try t.expectError(error.InvalidSubscription, defaults.prepareSubscription(.{ .subscriptions = .{.{ .channel = "websocket_control" }} }));
    try t.expectError(error.InvalidSubscription, defaults.prepareSubscription(.{ .subscriptions = .{ .{ .channel = "prices" }, .{ .channel = "prices" } } }));
    try t.expectError(error.InvalidMarket, defaults.prepareSubscription(.{ .subscriptions = .{.{ .channel = "book", .market = "BTC/USD/" }} }));
    const Entry = struct { channel: []const u8, market: []const u8 };
    const too_many: [201]Entry = @splat(.{ .channel = "book", .market = "BTC/USD" });
    try t.expectError(error.TooManySubscriptions, defaults.prepareSubscription(.{ .subscriptions = &too_many }));

    const url = try std.mem.concat(a, u8, &.{ "ws", origin[4..], "/caller-websocket/gains" });
    defer a.free(url);
    var plan = try aux.clients.GainsClient.init(a, .{ .endpoint = url }).prepareSubscription(spec);
    defer plan.deinit();
    var socket = try aux.RawWebSocket.connect(a, plan.value.endpoint, .{ .headers = plan.value.headers });
    defer socket.deinit();
    for (plan.value.subscribe) |frame| try socket.send(.text, frame);
    for ([_][]const u8{ "websocket_control", "prices", "book", "trades", "websocket_control" }) |route| {
        const message = try socket.next() orelse return error.MissingMessage;
        defer a.free(message.payload);
        var parsed = try aux.parse(a, message.payload);
        defer parsed.deinit();
        const frame = gains.classifyFrame(parsed.value);
        try t.expectEqualStrings(route, @tagName(frame.route));
        if (frame.reply) |reply| {
            try t.expectEqualStrings(" {\"op\":\"ping\",\"timestamp\":1791680400000} ", message.payload);
            try socket.send(.text, reply);
        }
    }
    for (plan.value.unsubscribe) |frame| try socket.send(.text, frame);
    try t.expect((try socket.next()) == null);
}

fn graphPages(a: A, origin: []const u8) !void {
    const client = aux.clients.SynFuturesClient.init(a, .{ .origin = origin });
    var trades = try client.virtual_trades(.{ .since = 100, .until = 200, .first = 2 });
    defer trades.deinit();
    const cursor = (try synfutures.nextCursor(trades.value, .virtual_trades, "", 2)) orelse return error.MissingCursor;
    try t.expectEqualStrings("trade-2", cursor);
    var last_page = try client.virtual_trades(.{ .since = 100, .until = 200, .first = 2, .last = cursor });
    defer last_page.deinit();
    try t.expect((try synfutures.nextCursor(last_page.value, .virtual_trades, cursor, 2)) == null);
    try t.expectError(error.InvalidCursor, synfutures.nextCursor(trades.value, .virtual_trades, cursor, 2));

    var candles = try client.candles(.{ .since = 100, .until = 200, .first = 1, .interval = "4h", .amm_ids = &.{instrument ++ "-4294967295"} });
    defer candles.deinit();
    try t.expectEqualStrings("candle-1", (try synfutures.nextCursor(candles.value, .{ .candles = .@"4h" }, "", 1)).?);
    var failed = try aux.parse(a, "{\"errors\":[{\"message\":\"unindexed\"}],\"data\":{\"virtualTrades\":[{\"id\":\"partial\"}]}}");
    defer failed.deinit();
    try t.expectError(error.GraphQlError, synfutures.nextCursor(failed.value, .virtual_trades, "", 1));
    try t.expectError(error.InvalidTimeRange, client.virtual_trades(.{ .since = 200, .until = 100 }));
    try t.expectError(error.InvalidInterval, client.candles(.{ .since = 100, .until = 200, .interval = "1m", .amm_ids = &.{instrument ++ "-4294967295"} }));
}

fn baseLogs(a: A, origin: []const u8) !void {
    const url = try std.mem.concat(a, u8, &.{ origin, "/synfutures-base-rpc" });
    defer a.free(url);
    const reader: aux.chain.Reader = .{ .client = .{ .allocator = a }, .endpoint = url };
    const addresses = &.{instrument};
    var page = try synfutures.prepareConfirmedLogs(reader, .{ .addresses = addresses, .from_block = 100 }) orelse return error.MissingLogPage;
    defer page.deinit();
    try t.expectEqual(@as(u64, 199), page.value.to_block);
    var tail = try synfutures.prepareConfirmedLogs(reader, .{ .addresses = addresses, .from_block = 200 }) orelse return error.MissingLogPage;
    defer tail.deinit();
    try t.expectEqual(@as(u64, 220), tail.value.to_block);
    try t.expectEqualStrings(url, tail.value.request.url);
    var request = try aux.parse(a, tail.value.request.body);
    defer request.deinit();
    const body = request.value.object;
    try t.expectEqualStrings("eth_getLogs", body.get("method").?.string);
    const filter = body.get("params").?.array.items[0].object;
    try t.expectEqualStrings("0xc8", filter.get("fromBlock").?.string);
    try t.expectEqualStrings("0xdc", filter.get("toBlock").?.string);
    try t.expectEqualStrings(instrument, filter.get("address").?.array.items[0].string);
    try t.expect((try synfutures.prepareConfirmedLogs(reader, .{ .addresses = addresses, .from_block = 221 })) == null);
    try t.expectError(error.InvalidAddress, synfutures.prepareConfirmedLogs(reader, .{ .addresses = &.{"0x01"}, .from_block = 1 }));

    const wrong = try std.mem.concat(a, u8, &.{ origin, "/synfutures-wrong-rpc" });
    defer a.free(wrong);
    const wrong_reader: aux.chain.Reader = .{ .client = reader.client, .endpoint = wrong };
    try t.expectError(error.UnexpectedChain, synfutures.prepareConfirmedLogs(wrong_reader, .{ .addresses = addresses, .from_block = 1 }));
}
