const std = @import("std");
const aux = @import("aux");
const t = std.testing;
const A = std.mem.Allocator;

pub fn run(a: A, base: []const u8) !void {
    const endpoint = try std.mem.concat(a, u8, &.{ base, "prepare/messages" });
    defer a.free(endpoint);
    const options: aux.Options = .{ .endpoint = endpoint };
    try pacifica(a, options);
    try hibachi(a, options);
    try hyperliquid(a, options);
    try ondo(a, options);
    try lighter(a, options);
    try polymarketPerps(a, options);
    try extended(a, base);
    try paradex(a, base);
}

pub fn verify(a: A, plan: aux.PreparedSubscription, subscribe: []const []const u8, unsubscribe: []const []const u8) !void {
    try t.expectEqual(subscribe.len, plan.subscribe.len);
    try t.expectEqual(unsubscribe.len, plan.unsubscribe.len);
    var socket = try aux.RawWebSocket.connect(a, plan.endpoint, .{ .headers = plan.headers });
    defer socket.deinit();
    var index: u64 = 1;
    for ([_][]const []const u8{ plan.subscribe, plan.unsubscribe }, [_][]const []const u8{ subscribe, unsubscribe }) |messages, expected| {
        for (messages, expected) |message, bytes| {
            try socket.send(.text, message);
            const reply = try socket.next() orelse return error.MissingMessage;
            defer a.free(reply.payload);
            try t.expectEqualStrings(bytes, reply.payload);
            try t.expectEqual(index, reply.receive_index);
            index += 1;
        }
    }
    try socket.send(.text, "done");
    try t.expect((try socket.next()) == null);
}

fn lighter(a: A, options: aux.Options) !void {
    const spec = .{ .subscriptions = .{
        .{ .channel = "order_book", .market_id = 4 },
        .{ .channel = "trade", .market_id = 7 },
        .{ .channel = "candle", .market_id = 4 },
    } };
    const client = aux.clients.LighterClient.init(a, options);
    var plan = try client.prepareSubscription(spec);
    defer plan.deinit();
    try t.expectEqualStrings(options.endpoint.?, plan.value.endpoint);
    try verify(a, plan.value, &.{
        "{\"type\":\"subscribe\",\"channel\":\"order_book/4\"}",
        "{\"type\":\"subscribe\",\"channel\":\"trade/7\"}",
        "{\"type\":\"subscribe\",\"channel\":\"candle/4/1m\"}",
    }, &.{
        "{\"type\":\"unsubscribe\",\"channel\":\"order_book/4\"}",
        "{\"type\":\"unsubscribe\",\"channel\":\"trade/7\"}",
        "{\"type\":\"unsubscribe\",\"channel\":\"candle/4/1m\"}",
    });
    const defaults = aux.clients.LighterClient.init(a, .{});
    var default_plan = try defaults.prepareSubscription(spec);
    defer default_plan.deinit();
    try t.expectEqualStrings("wss://mainnet.zklighter.elliot.ai/stream", default_plan.value.endpoint);
    try t.expectError(error.InvalidChannel, defaults.prepareSubscription(.{ .subscriptions = .{.{ .channel = "account_all", .account_index = 7 }} }));
    try t.expectError(error.InvalidId, defaults.prepareSubscription(.{ .subscriptions = .{.{ .channel = "candle", .market_id = -1 }} }));
    const invalid = aux.clients.LighterClient.init(a, .{ .endpoint = "https://example.invalid" });
    try t.expectError(error.InvalidEndpoint, invalid.prepareSubscription(spec));
}

fn polymarketPerps(a: A, options: aux.Options) !void {
    const spec = .{ .channels = .{ "book::42", "klines::42::1m", "trades::42", "tickers::all" } };
    const client = aux.clients.PolymarketPerpsClient.init(a, options);
    var plan = try client.prepareSubscription(spec);
    defer plan.deinit();
    try t.expectEqualStrings(options.endpoint.?, plan.value.endpoint);
    try verify(a, plan.value, &.{
        "{\"req\":\"sub\",\"chs\":[\"book::42\",\"klines::42::1m\",\"trades::42\",\"tickers::all\"]}",
    }, &.{
        "{\"req\":\"unsub\",\"chs\":[\"book::42\",\"klines::42::1m\",\"trades::42\",\"tickers::all\"]}",
    });
    const defaults = aux.clients.PolymarketPerpsClient.init(a, .{});
    var default_plan = try defaults.prepareSubscription(spec);
    defer default_plan.deinit();
    try t.expectEqualStrings("wss://ws.perpetuals.polymarket.com/v1/ws", default_plan.value.endpoint);
    try t.expectError(error.EmptySubscription, defaults.prepareSubscription(.{ .channels = @as([]const []const u8, &.{}) }));
    try t.expectError(error.InvalidSubscription, defaults.prepareSubscription(.{ .channels = .{ "book::42", " \t" } }));
    const invalid = aux.clients.PolymarketPerpsClient.init(a, .{ .endpoint = "https://example.invalid" });
    try t.expectError(error.InvalidEndpoint, invalid.prepareSubscription(spec));
}

fn pacifica(a: A, options: aux.Options) !void {
    const client = aux.clients.PacificaClient.init(a, options);
    var plan = try client.prepareSubscription(.{ .subscriptions = .{
        .{ .source = "prices" },
        .{ .source = "trades", .symbol = "BTC" },
        .{ .source = "book", .symbol = "BTC", .agg_level = 10 },
        .{ .source = "bbo", .symbol = "BTC" },
        .{ .source = "candle", .symbol = "BTC", .interval = "1m" },
    } });
    defer plan.deinit();
    try verify(a, plan.value, &.{
        "{\"method\":\"subscribe\",\"params\":{\"source\":\"prices\"}}",
        "{\"method\":\"subscribe\",\"params\":{\"source\":\"trades\",\"symbol\":\"BTC\"}}",
        "{\"method\":\"subscribe\",\"params\":{\"source\":\"book\",\"symbol\":\"BTC\",\"agg_level\":10}}",
        "{\"method\":\"subscribe\",\"params\":{\"source\":\"bbo\",\"symbol\":\"BTC\"}}",
        "{\"method\":\"subscribe\",\"params\":{\"source\":\"candle\",\"symbol\":\"BTC\",\"interval\":\"1m\"}}",
    }, &.{
        "{\"method\":\"unsubscribe\",\"params\":{\"source\":\"prices\"}}",
        "{\"method\":\"unsubscribe\",\"params\":{\"source\":\"trades\",\"symbol\":\"BTC\"}}",
        "{\"method\":\"unsubscribe\",\"params\":{\"source\":\"book\",\"symbol\":\"BTC\",\"agg_level\":10}}",
        "{\"method\":\"unsubscribe\",\"params\":{\"source\":\"bbo\",\"symbol\":\"BTC\"}}",
        "{\"method\":\"unsubscribe\",\"params\":{\"source\":\"candle\",\"symbol\":\"BTC\",\"interval\":\"1m\"}}",
    });
    try t.expectError(error.InvalidInterval, client.prepareSubscription(.{ .subscriptions = .{.{ .source = "candle", .symbol = "BTC", .interval = "1minute" }} }));
}

fn hibachi(a: A, options: aux.Options) !void {
    const client = aux.clients.HibachiClient.init(a, options);
    var plan = try client.prepareSubscription(.{ .subscriptions = .{
        .{ .symbol = "BTC/USDT-P", .topic = "book", .depth = 20, .granularity = "0.1" },
        .{ .symbol = "ETH/USDT-P", .topic = " trade " },
    } });
    defer plan.deinit();
    try verify(a, plan.value, &.{
        "{\"method\":\"subscribe\",\"parameters\":{\"subscriptions\":[{\"symbol\":\"BTC/USDT-P\",\"topic\":\"orderbook\",\"depth\":20,\"granularity\":\"0.1\"},{\"symbol\":\"ETH/USDT-P\",\"topic\":\"trades\"}]}}",
    }, &.{
        "{\"method\":\"unsubscribe\",\"parameters\":{\"subscriptions\":[{\"symbol\":\"BTC/USDT-P\",\"topic\":\"orderbook\"},{\"symbol\":\"ETH/USDT-P\",\"topic\":\"trades\"}]}}",
    });
}

fn hyperliquid(a: A, options: aux.Options) !void {
    const client = aux.clients.HyperliquidClient.init(a, options);
    var plan = try client.prepareSubscription(.{ .subscriptions = .{
        .{ .type = "l2Book", .coin = "BTC", .dex = "Perp", .nSigFigs = 5, .mantissa = 2 },
        .{ .type = "trades", .coin = "@107", .dex = "Spot" },
        .{ .type = "activeAssetCtx", .coin = "#10" },
        .{ .type = "candle", .coin = "BTC", .interval = "1m" },
    } });
    defer plan.deinit();
    try verify(a, plan.value, &.{
        "{\"method\":\"subscribe\",\"subscription\":{\"type\":\"l2Book\",\"coin\":\"BTC\",\"dex\":\"Perp\",\"nSigFigs\":5,\"mantissa\":2}}",
        "{\"method\":\"subscribe\",\"subscription\":{\"type\":\"trades\",\"coin\":\"@107\",\"dex\":\"Spot\"}}",
        "{\"method\":\"subscribe\",\"subscription\":{\"type\":\"activeAssetCtx\",\"coin\":\"#10\"}}",
        "{\"method\":\"subscribe\",\"subscription\":{\"type\":\"candle\",\"coin\":\"BTC\",\"interval\":\"1m\"}}",
    }, &.{
        "{\"method\":\"unsubscribe\",\"subscription\":{\"type\":\"l2Book\",\"coin\":\"BTC\",\"dex\":\"Perp\",\"nSigFigs\":5,\"mantissa\":2}}",
        "{\"method\":\"unsubscribe\",\"subscription\":{\"type\":\"trades\",\"coin\":\"@107\",\"dex\":\"Spot\"}}",
        "{\"method\":\"unsubscribe\",\"subscription\":{\"type\":\"activeAssetCtx\",\"coin\":\"#10\"}}",
        "{\"method\":\"unsubscribe\",\"subscription\":{\"type\":\"candle\",\"coin\":\"BTC\",\"interval\":\"1m\"}}",
    });

    const paragon_client = aux.clients.ParagonClient.init(a, options);
    var paragon = try paragon_client.prepareSubscription(.{ .subscriptions = .{.{ .type = "trades", .coin = "para:ETH" }} });
    defer paragon.deinit();
    try verify(a, paragon.value, &.{
        "{\"method\":\"subscribe\",\"subscription\":{\"type\":\"trades\",\"coin\":\"para:ETH\",\"dex\":\"para\"}}",
    }, &.{
        "{\"method\":\"unsubscribe\",\"subscription\":{\"type\":\"trades\",\"coin\":\"para:ETH\",\"dex\":\"para\"}}",
    });

    const xyz_client = aux.clients.TradeXyzClient.init(a, options);
    var xyz = try xyz_client.prepareSubscription(.{ .subscriptions = .{
        .{ .type = "candle", .coin = "xyz:NVDA", .interval = "1m" },
        .{ .type = "trades", .coin = "@107" },
        .{ .type = "activeAssetCtx", .coin = "#10" },
    } });
    defer xyz.deinit();
    try verify(a, xyz.value, &.{
        "{\"method\":\"subscribe\",\"subscription\":{\"type\":\"candle\",\"coin\":\"xyz:NVDA\",\"interval\":\"1m\"}}",
        "{\"method\":\"subscribe\",\"subscription\":{\"type\":\"trades\",\"coin\":\"@107\",\"dex\":\"Spot\"}}",
        "{\"method\":\"subscribe\",\"subscription\":{\"type\":\"activeAssetCtx\",\"coin\":\"#10\"}}",
    }, &.{
        "{\"method\":\"unsubscribe\",\"subscription\":{\"type\":\"candle\",\"coin\":\"xyz:NVDA\",\"interval\":\"1m\"}}",
        "{\"method\":\"unsubscribe\",\"subscription\":{\"type\":\"trades\",\"coin\":\"@107\",\"dex\":\"Spot\"}}",
        "{\"method\":\"unsubscribe\",\"subscription\":{\"type\":\"activeAssetCtx\",\"coin\":\"#10\"}}",
    });
    try t.expectError(error.InvalidMarket, xyz_client.prepareSubscription(.{ .subscriptions = .{.{ .type = "trades", .coin = "para:ETH" }} }));
}

fn ondo(a: A, options: aux.Options) !void {
    const client = aux.clients.OndoPerpsClient.init(a, options);
    var plan = try client.prepareSubscription(.{ .subscriptions = .{
        .{ .channel = "perpsTrades", .numPastTrades = 50 },
        .{ .channel = "depthBooksPerps", .markets = .{ "NVDA-USD.P", "TSLA-USD.P" }, .depthLevels = "0.01", .limit = 20 },
        .{ .channel = "topOfBooksPerps" },
        .{ .channel = "markPricesPerps" },
        .{ .channel = "fundingRatesPerps" },
        .{ .channel = "candles", .markets = .{ "NVDA-USD.P", "TSLA-USD.P" }, .resolution = "1" },
    } });
    defer plan.deinit();
    try verify(a, plan.value, &.{
        "{\"op\":\"subscribe\",\"channel\":\"tradesPerps\",\"numPastTrades\":50}",
        "{\"op\":\"subscribe\",\"channel\":\"depthBooksPerps\",\"markets\":[\"NVDA-USD.P\",\"TSLA-USD.P\"],\"depthLevels\":\"0.01\",\"limit\":20}",
        "{\"op\":\"subscribe\",\"channel\":\"topOfBooksPerps\"}",
        "{\"op\":\"subscribe\",\"channel\":\"markPricesPerps\"}",
        "{\"op\":\"subscribe\",\"channel\":\"fundingRatesPerps\"}",
        "{\"op\":\"subscribe\",\"channel\":\"kLinePerps\",\"markets\":[\"NVDA-USD.P\"],\"resolution\":\"1\"}",
        "{\"op\":\"subscribe\",\"channel\":\"kLinePerps\",\"markets\":[\"TSLA-USD.P\"],\"resolution\":\"1\"}",
    }, &.{
        "{\"op\":\"unsubscribe\",\"channel\":\"tradesPerps\",\"numPastTrades\":50}",
        "{\"op\":\"unsubscribe\",\"channel\":\"depthBooksPerps\",\"markets\":[\"NVDA-USD.P\",\"TSLA-USD.P\"],\"depthLevels\":\"0.01\",\"limit\":20}",
        "{\"op\":\"unsubscribe\",\"channel\":\"topOfBooksPerps\"}",
        "{\"op\":\"unsubscribe\",\"channel\":\"markPricesPerps\"}",
        "{\"op\":\"unsubscribe\",\"channel\":\"fundingRatesPerps\"}",
        "{\"op\":\"unsubscribe\",\"channel\":\"kLinePerps\",\"markets\":[\"NVDA-USD.P\"],\"resolution\":\"1\"}",
        "{\"op\":\"unsubscribe\",\"channel\":\"kLinePerps\",\"markets\":[\"TSLA-USD.P\"],\"resolution\":\"1\"}",
    });
    try t.expectError(error.InvalidMarket, client.prepareSubscription(.{ .subscriptions = .{.{ .channel = "candles", .resolution = "1" }} }));
}

fn extended(a: A, base: []const u8) !void {
    const endpoint = try std.mem.concat(a, u8, &.{ base, "prepare/extended" });
    defer a.free(endpoint);
    const client = aux.clients.ExtendedClient.init(a, .{ .endpoint = endpoint });
    var plan = try client.prepareSubscription(.{ .channel = "orderbook", .market = "BTC-USD", .params = .{ .depth = 1 } });
    defer plan.deinit();
    try t.expectEqual(@as(usize, 0), plan.value.subscribe.len);
    try t.expectEqual(@as(usize, 0), plan.value.unsubscribe.len);
    var socket = try aux.RawWebSocket.connect(a, plan.value.endpoint, .{ .headers = plan.value.headers });
    defer socket.deinit();
    const reply = try socket.next() orelse return error.MissingMessage;
    defer a.free(reply.payload);
    try t.expectEqualStrings("{\"type\":\"DELTA\",\"seq\":1}", reply.payload);
    try t.expect((try socket.next()) == null);
    try t.expectError(error.InvalidHeader, aux.RawWebSocket.connect(a, plan.value.endpoint, .{ .headers = &.{.{ .name = "Origin", .value = "valid\r\ninjected" }} }));
    try t.expectError(error.InvalidHeader, aux.RawWebSocket.connect(a, plan.value.endpoint, .{ .headers = &.{.{ .name = "Sec-WebSocket-Key", .value = "invalid" }} }));
}

fn paradex(a: A, base: []const u8) !void {
    const endpoint = try std.mem.concat(a, u8, &.{ base, "prepare/paradex?sbeSchemaId=99&fixture=1&sbeSchemaVersion=3" });
    defer a.free(endpoint);
    const spec = .{
        .sbe_version = 1,
        .subscribe_id = 7,
        .unsubscribe_id = 20,
        .subscriptions = .{
            .{ .type = "bbo", .market = "ETH-USD-PERP" },
            .{ .topic = "order_book.ETH-USD-PERP.deltas" },
        },
    };
    const client = aux.clients.ParadexClient.init(a, .{ .endpoint = endpoint });
    var plan = try client.prepareSubscription(spec);
    defer plan.deinit();
    try verify(a, plan.value, &.{
        "{\"jsonrpc\":\"2.0\",\"id\":7,\"method\":\"subscribe\",\"params\":{\"channel\":\"bbo.ETH-USD-PERP\"}}",
        "{\"jsonrpc\":\"2.0\",\"id\":8,\"method\":\"subscribe\",\"params\":{\"channel\":\"order_book.ETH-USD-PERP.deltas\"}}",
    }, &.{
        "{\"jsonrpc\":\"2.0\",\"id\":20,\"method\":\"unsubscribe\",\"params\":{\"channel\":\"bbo.ETH-USD-PERP\"}}",
        "{\"jsonrpc\":\"2.0\",\"id\":21,\"method\":\"unsubscribe\",\"params\":{\"channel\":\"order_book.ETH-USD-PERP.deltas\"}}",
    });
    const default_client = aux.clients.ParadexClient.init(a, .{});
    var defaults = try default_client.prepareSubscription(.{ .subscribe_id = 2, .subscriptions = .{.{ .type = "markets_summary" }} });
    defer defaults.deinit();
    try t.expectEqualStrings("wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=3", defaults.value.endpoint);
    try t.expectEqualStrings("{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"unsubscribe\",\"params\":{\"channel\":\"markets_summary\"}}", defaults.value.unsubscribe[0]);
    try t.expectError(error.AuthenticationRequired, client.prepareSubscription(.{ .subscriptions = .{.{ .type = "account" }} }));
}
