const std = @import("std");
const aux = @import("aux");
const t = std.testing;
const Allocator = std.mem.Allocator;
const RawWebSocket = aux.RawWebSocket;

pub fn run(a: Allocator, io: std.Io, origin: []const u8) !void {
    const base = try std.mem.concat(a, u8, &.{ "ws:", origin[5..], "/caller-websocket/" });
    defer a.free(base);
    try qfex(a, base);
    try deribit(a, base);
    try rawFrames(a, base);
    try connectionLifecycle(a, io, base);
    try @import("subscription_prepare.zig").run(a, base);
}

fn expectMessage(ws: *RawWebSocket, kind: RawWebSocket.Kind, bytes: []const u8, index: u64) !void {
    const message = try ws.next() orelse return error.MissingMessage;
    defer ws.allocator.free(message.payload);
    try t.expectEqual(kind, message.kind);
    try t.expectEqualSlices(u8, bytes, message.payload);
    try t.expectEqual(index, message.receive_index);
}

fn qfex(a: Allocator, base: []const u8) !void {
    const spec: aux.clients.qfex.SubscriptionSpec = .{
        .channels = &.{ "trade", "candle", "funding_rate" },
        .symbols = &.{ "ABSI-USD", "CYPH-USD" },
        .intervals = &.{ "1MIN", "5MINS" },
    };
    const defaults = aux.clients.QfexClient.init(a, .{ .origin = "https://api.qfex.com" });
    var default_plan = try defaults.prepareSubscription(.{ .channels = &.{"trade"}, .symbols = &.{"*"} });
    defer default_plan.deinit();
    try t.expectEqualStrings("wss://mds.qfex.com/", default_plan.value.endpoint);
    try t.expectEqual(@as(usize, 1), default_plan.value.subscribe.len);
    try t.expectError(error.InvalidInterval, defaults.prepareSubscription(.{ .channels = &.{"candle"}, .symbols = spec.symbols }));
    try t.expectError(error.InvalidInterval, defaults.prepareSubscription(.{ .channels = &.{"candle"}, .symbols = spec.symbols, .intervals = &.{"1min"} }));
    try t.expectError(error.InvalidInterval, defaults.prepareSubscription(.{ .channels = &.{"trade"}, .symbols = spec.symbols, .intervals = spec.intervals }));
    try t.expectError(error.UnknownChannel, defaults.prepareSubscription(.{ .channels = &.{"unknown"}, .symbols = spec.symbols }));
    try t.expectError(error.InvalidSymbol, defaults.prepareSubscription(.{ .channels = &.{"trade"}, .symbols = &.{" "} }));
    try t.expectError(error.EmptySubscription, defaults.prepareSubscription(.{ .channels = &.{"trade"}, .symbols = &.{} }));

    const invalid = aux.clients.QfexClient.init(a, .{ .endpoint = "https://example.invalid" });
    try t.expectError(error.InvalidEndpoint, invalid.prepareSubscription(spec));
    var plan = blk: {
        const endpoint = try std.mem.concat(a, u8, &.{ base, "qfex?feed=public" });
        defer a.free(endpoint);
        const client = aux.clients.QfexClient.init(a, .{ .endpoint = endpoint, .origin = "ws://example.invalid" });
        break :blk try client.prepareSubscription(spec);
    };
    defer plan.deinit();
    try t.expectEqual(@as(usize, 2), plan.value.subscribe.len);
    try t.expectEqual(@as(usize, 2), plan.value.unsubscribe.len);

    var ws = try RawWebSocket.connect(a, plan.value.endpoint, .{});
    defer ws.deinit();
    for (plan.value.subscribe) |message| try ws.send(.text, message);
    const prefix = [_][]const u8{
        "{\"type\":\"subscribed\"}",
        " {\"channel\":\"candle\",\"n\":1.000} ",
        "{\"type\":\"funding\",\"symbol\":\"ABSI-USD\",\"funding_rate\":\"0.0001500\"}",
        "{\"error\":\"fixture\"}",
        "{broken",
    };
    for (prefix, 1..) |message, index| try expectMessage(&ws, .text, message, index);
    for (0..70) |i| {
        var buf: [4]u8 = undefined;
        try expectMessage(&ws, .text, try std.fmt.bufPrint(&buf, "{d}", .{i}), prefix.len + 1 + i);
    }
    for (plan.value.unsubscribe) |message| try ws.send(.text, message);
    try t.expect((try ws.next()) == null);
    try t.expect((try ws.next()) == null);
}

fn deribit(a: Allocator, base: []const u8) !void {
    const Topic = aux.clients.deribit.SubscriptionTopic;
    var topics: [501]Topic = undefined;
    var names: [498][40]u8 = undefined;
    topics[0] = .{ .market = .{ .channel = .ticker, .instrument_name = "BTC-PERPETUAL" } };
    topics[1] = .{ .market = .{ .channel = .trades, .instrument_name = "BTC-PERPETUAL", .interval = "agg2" } };
    topics[2] = .{ .market = .{ .channel = .book, .instrument_name = "BTC-PERPETUAL", .book = .{ .group = "none", .depth = 20 } } };
    for (topics[3..], &names, 3..) |*topic, *name, i| {
        topic.* = .{ .native = try std.fmt.bufPrint(name, "ticker.TEST-{d}.100ms", .{i}) };
    }

    var auth = try aux.parse(a, "{\"network\":\"Testnet\"}");
    defer auth.deinit();
    const testnet = aux.clients.DeribitClient.init(a, .{ .auth = auth.value });
    var one_batch = try testnet.prepareSubscription(.{ .topics = topics[0..500], .subscribe_id = 1, .unsubscribe_id = 2 });
    defer one_batch.deinit();
    try t.expectEqualStrings("wss://test.deribit.com/ws/api/v2", one_batch.value.endpoint);
    try t.expectEqual(@as(usize, 1), one_batch.value.subscribe.len);
    try t.expectError(error.InvalidRequestId, testnet.prepareSubscription(.{ .topics = &topics, .subscribe_id = std.math.maxInt(u64), .unsubscribe_id = 10 }));
    try t.expectError(error.AuthenticationRequired, testnet.prepareSubscription(.{ .topics = &.{.{ .native = "book.BTC-PERPETUAL.raw" }}, .subscribe_id = 1, .unsubscribe_id = 2 }));
    try t.expectError(error.InvalidSubscription, testnet.prepareSubscription(.{ .topics = &.{.{ .native = "" }}, .subscribe_id = 1, .unsubscribe_id = 2 }));

    const endpoint = try std.mem.concat(a, u8, &.{ base, "deribit" });
    defer a.free(endpoint);
    const client = aux.clients.DeribitClient.init(a, .{ .origin = endpoint, .auth = auth.value });
    var plan = try client.prepareSubscription(.{ .topics = &topics, .subscribe_id = 7, .unsubscribe_id = 40 });
    defer plan.deinit();
    try t.expectEqualStrings(endpoint, plan.value.endpoint);
    try t.expectEqual(@as(usize, 2), plan.value.subscribe.len);
    try t.expectEqual(@as(usize, 2), plan.value.unsubscribe.len);

    var ws = try RawWebSocket.connect(a, plan.value.endpoint, .{});
    defer ws.deinit();
    for (plan.value.subscribe) |message| try ws.send(.text, message);
    for (plan.value.unsubscribe) |message| try ws.send(.text, message);
    try expectMessage(&ws, .text, "{\"method\":\"heartbeat\",\"params\":{\"type\":\"test_request\"}}", 1);
    try t.expect((try ws.next()) == null);
}

fn open(a: Allocator, base: []const u8, path: []const u8) !RawWebSocket {
    const endpoint = try std.mem.concat(a, u8, &.{ base, path });
    defer a.free(endpoint);
    return RawWebSocket.connect(a, endpoint, .{});
}

fn rawFrames(a: Allocator, base: []const u8) !void {
    var extended = try open(a, base, "extended");
    defer extended.deinit();
    try expectMessage(&extended, .text, "{\"type\":\"DELTA\",\"seq\":1,\"data\":{\"m\":\"BTC-USD\",\"b\":[],\"a\":[]}}", 1);
    try t.expect((try extended.next()) == null);

    var paradex = try open(a, base, "paradex");
    defer paradex.deinit();
    try paradex.send(.binary, "\x00\xff");
    const message = try paradex.next() orelse return error.MissingMessage;
    defer a.free(message.payload);
    const hex = "3000020001000300402e63a75e5f06007b00000000000000000000000000008015cd5b0700000000000000000000008015cd5b07000000000c4554482d5553442d50455250";
    var expected: [hex.len / 2]u8 = undefined;
    try t.expectEqualSlices(u8, try aux.codec.hexDecode(hex, &expected), message.payload);
    try t.expectEqual(RawWebSocket.Kind.binary, message.kind);
    try t.expectEqual(@as(u64, 1), message.receive_index);
    var decoded = try aux.codec.decodeSbe(.{ .allocator = a }, message.payload);
    defer decoded.deinit();
    const data = decoded.value.object.get("data").?;
    try t.expectEqualStrings("ETH-USD-PERP", data.object.get("market").?.string);
    try t.expect((try paradex.next()) == null);
}

fn waitForCancel(ws: *RawWebSocket) void {
    std.debug.assert((ws.next() catch @panic("raw read failed")) == null);
}

fn connectionLifecycle(a: Allocator, io: std.Io, base: []const u8) !void {
    var dial_called = false;
    const dial: aux.Dial = .{ .context = &dial_called, .connect = probeDial };
    try t.expectError(error.UnsupportedTransport, RawWebSocket.connect(a, "wss://example.invalid/", .{ .dial = dial }));
    try t.expect(dial_called);

    var canceled = try open(a, base, "cancel");
    defer canceled.deinit();
    try expectMessage(&canceled, .text, "ready", 1);
    const reader = try std.Thread.spawn(.{}, waitForCancel, .{&canceled});
    {
        defer reader.join();
        defer canceled.cancel();
        try std.Io.sleep(io, .fromMilliseconds(25), .awake);
    }
    canceled.cancel();
    try t.expectError(error.Closed, canceled.send(.text, "late"));

    var reconnected = try open(a, base, "cancel");
    defer reconnected.deinit();
    try expectMessage(&reconnected, .text, "ready", 1);

    var dropped = try open(a, base, "drop");
    defer dropped.deinit();
    try t.expectError(error.Network, dropped.next());
    try t.expect((try dropped.next()) == null);
}

fn probeDial(context: *anyopaque, host: [*]const u8, len: usize, port: u16, tls: bool, _: *c_int) callconv(.c) u32 {
    const called: *bool = @ptrCast(@alignCast(context));
    called.* = std.mem.eql(u8, host[0..len], "example.invalid") and port == 443 and tls;
    return 1;
}
