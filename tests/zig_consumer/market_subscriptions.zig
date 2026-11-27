const std = @import("std");
const aux = @import("aux");
const t = std.testing;
const A = std.mem.Allocator;
const verify = @import("subscription_prepare.zig").verify;

pub fn run(a: A, io: std.Io, fixture_dir: []const u8, base: []const u8) !void {
    const endpoint = try std.mem.concat(a, u8, &.{ base, "prepare/messages" });
    defer a.free(endpoint);
    const options: aux.Options = .{ .endpoint = endpoint };
    try additionalVenues(a, options);

    try check(a, aux.clients.AevoClient, options, .{ .subscriptions = .{.{
        .channel = "book",
        .symbol = "BTC-PERP",
        .interval = "500ms",
    }} }, "wss://ws.aevo.xyz", &.{
        "{\"id\":1,\"op\":\"subscribe\",\"data\":[\"orderbook-500ms:BTC-PERP\"]}",
    }, &.{
        "{\"id\":2,\"op\":\"unsubscribe\",\"data\":[\"orderbook-500ms:BTC-PERP\"]}",
    });
    try check(a, aux.clients.ApexOmniClient, options, .{ .timestamp_ms = 1700000000123, .subscriptions = .{.{
        .channel = "book",
        .symbol = "BTCUSDT",
        .depth = 200,
        .frequency = "M",
    }} }, "wss://quote.omni.apex.exchange/realtime_public?v=2&timestamp=1700000000123", &.{
        "{\"op\":\"subscribe\",\"args\":[\"orderBook200.M.BTCUSDT\"]}",
    }, &.{
        "{\"op\":\"unsubscribe\",\"args\":[\"orderBook200.M.BTCUSDT\"]}",
    });
    var apex = try aux.clients.ApexOmniClient.init(a, .{}).prepareSubscription(.{ .subscriptions = .{.{ .channel = "book", .symbol = "BTCUSDT" }} });
    defer apex.deinit();
    const stamp = std.mem.cutPrefix(u8, apex.value.endpoint, "wss://quote.omni.apex.exchange/realtime_public?v=2&timestamp=") orelse return error.MissingTimestamp;
    try t.expect((try std.fmt.parseInt(u64, stamp, 10)) > 1_700_000_000_000);

    try check(a, aux.clients.ArcusClient, options, .{ .subscriptions = .{.{
        .channel = "l2OrderbookUpdates",
        .id = "SOL-USD",
        .nLevels = 20,
    }} }, "wss://api.arcus.xyz/v1/ws", &.{
        "{\"channel\":\"l2OrderbookUpdates\",\"id\":\"SOL-USD\",\"nLevels\":20,\"type\":\"subscribe\"}",
    }, &.{
        "{\"type\":\"unsubscribe\",\"channel\":\"l2OrderbookUpdates\",\"id\":\"SOL-USD\"}",
    });
    try check(a, aux.clients.BinanceSpotClient, options, .{ .subscriptions = .{
        .{ .channel = "depth", .symbol = "BTCUSDT" },
        .{ .channel = "kline", .symbol = "ETHUSDT", .interval = "5m" },
    } }, "wss://stream.binance.com:443/ws", &.{
        "{\"method\":\"SUBSCRIBE\",\"params\":[\"btcusdt@depth@100ms\",\"ethusdt@kline_5m\"],\"id\":1}",
    }, &.{
        "{\"method\":\"UNSUBSCRIBE\",\"params\":[\"btcusdt@depth@100ms\",\"ethusdt@kline_5m\"],\"id\":2}",
    });
    try check(a, aux.clients.MonacoClient, options, .{ .subscriptions = .{
        .{ .type = "orderbook", .tradingPairId = pair, .mode = "SPOT", .magnitude = "0.1", .denomination = "quote" },
        .{ .type = "trades", .tradingPairId = "*" },
        candle,
    } }, "wss://ws.apimonaco.xyz/ws", &.{
        "{\"type\":\"Subscribe\",\"channels\":[\"orderbook:550e8400-e29b-41d4-a716-446655440000:SPOT:0.1:quote\",\"trades:*\",\"ohlcv:550e8400-e29b-41d4-a716-446655440000:SPOT:1m\"]}",
    }, &.{
        "{\"type\":\"Unsubscribe\",\"channels\":[\"orderbook:550e8400-e29b-41d4-a716-446655440000:SPOT:0.1:quote\",\"trades:*\",\"ohlcv:550e8400-e29b-41d4-a716-446655440000:SPOT:1m\"]}",
    });
    try check(a, aux.clients.PerplClient, options, .{ .subscriptions = .{
        .{ .channel = "order_book", .market_id = 3 },
        .{ .channel = "trades", .market_id = 9 },
    } }, "wss://app.perpl.xyz/ws/v1/market-data", &.{
        "{\"mt\":5,\"subs\":[{\"stream\":\"order-book@3\",\"subscribe\":true},{\"stream\":\"trades@9\",\"subscribe\":true}]}",
    }, &.{
        "{\"mt\":5,\"subs\":[{\"stream\":\"order-book@3\",\"subscribe\":false},{\"stream\":\"trades@9\",\"subscribe\":false}]}",
    });
    try check(a, aux.clients.PhoenixClient, options, .{ .subscriptions = .{
        .{ .channel = "market", .symbol = "SOL" },
        .{ .channel = "orderbook", .symbol = "BTC", .bypassExecutionBand = true },
    } }, "wss://perp-api.phoenix.trade/v1/ws", &.{
        "{\"type\":\"subscribe\",\"subscription\":{\"channel\":\"market\",\"symbol\":\"SOL\"}}",
        "{\"type\":\"subscribe\",\"subscription\":{\"channel\":\"orderbook\",\"symbol\":\"BTC\",\"bypassExecutionBand\":true}}",
    }, &.{
        "{\"type\":\"unsubscribe\",\"subscription\":{\"channel\":\"market\",\"symbol\":\"SOL\"}}",
        "{\"type\":\"unsubscribe\",\"subscription\":{\"channel\":\"orderbook\",\"symbol\":\"BTC\",\"bypassExecutionBand\":true}}",
    });
    try check(a, aux.clients.RisexClient, options, .{ .subscriptions = .{oracle} }, "wss://ws.rise.trade/ws", &.{
        "{\"method\":\"subscribe\",\"params\":{\"channel\":\"oracle\",\"market_ids\":[1,2]}}",
    }, &.{
        "{\"method\":\"unsubscribe\",\"params\":{\"channel\":\"oracle\"}}",
    });

    try invalidAndOwned(a, options);
    try n1(a, base);
    try kuru(a, io, fixture_dir, base);
    try managed(a, base);
}

fn check(a: A, comptime Client: type, options: aux.Options, spec: anytype, default_endpoint: []const u8, subscribe: []const []const u8, unsubscribe: []const []const u8) !void {
    var defaults = try Client.init(a, .{}).prepareSubscription(spec);
    defer defaults.deinit();
    try t.expectEqualStrings(default_endpoint, defaults.value.endpoint);
    var plan = try Client.init(a, options).prepareSubscription(spec);
    defer plan.deinit();
    try t.expectEqualStrings(options.endpoint.?, plan.value.endpoint);
    try verify(a, plan.value, subscribe, unsubscribe);
}

fn additionalVenues(a: A, options: aux.Options) !void {
    const book = .{ .channel = "book", .instId = "BTC-USDT" };
    const trades = .{ .channel = "trades", .instId = "ETH-USDT" };
    const candle1m = .{ .channel = "candle1m", .instId = "BTC-USDT" };
    try check(a, aux.clients.OkxClient, options, .{ .subscriptions = .{ book, trades } }, "wss://ws.okx.com/ws/v5/public", &.{
        "{\"op\":\"subscribe\",\"args\":[{\"channel\":\"books\",\"instId\":\"BTC-USDT\"},{\"channel\":\"trades\",\"instId\":\"ETH-USDT\"}]}",
    }, &.{
        "{\"op\":\"unsubscribe\",\"args\":[{\"channel\":\"books\",\"instId\":\"BTC-USDT\"},{\"channel\":\"trades\",\"instId\":\"ETH-USDT\"}]}",
    });
    try check(a, aux.clients.OkxClient, options, .{ .subscriptions = .{candle1m} }, "wss://ws.okx.com/ws/v5/business", &.{
        "{\"op\":\"subscribe\",\"args\":[{\"channel\":\"candle1m\",\"instId\":\"BTC-USDT\"}]}",
    }, &.{
        "{\"op\":\"unsubscribe\",\"args\":[{\"channel\":\"candle1m\",\"instId\":\"BTC-USDT\"}]}",
    });
    const okx = aux.clients.OkxClient.init(a, options);
    try t.expectError(error.MixedEndpoints, okx.prepareSubscription(.{ .subscriptions = .{ book, candle1m } }));
    try t.expectError(error.AuthenticationRequired, okx.prepareSubscription(.{ .subscriptions = .{.{ .channel = "orders" }} }));
    try t.expectError(error.InvalidSubscription, okx.prepareSubscription(.{ .subscriptions = .{.{ .channel = "trades" }} }));

    try check(a, aux.clients.GateDexClient, options, .{ .timestamp_s = 1700000000, .subscriptions = .{
        .{ .channel = "futures.order_book", .symbol = "BTC_USDT", .depth = 20 },
        .{ .channel = "futures.trades", .symbol = "ETH_USDT" },
    } }, "wss://api.gatedex.com/v1/ws/dex", &.{
        "{\"time\":1700000000,\"channel\":\"futures.order_book\",\"event\":\"subscribe\",\"payload\":[\"BTC_USDT\",\"20\",\"0\"]}",
        "{\"time\":1700000000,\"channel\":\"futures.trades\",\"event\":\"subscribe\",\"payload\":[\"ETH_USDT\"]}",
    }, &.{
        "{\"time\":1700000000,\"channel\":\"futures.order_book\",\"event\":\"unsubscribe\",\"payload\":[\"BTC_USDT\",\"20\",\"0\"]}",
        "{\"time\":1700000000,\"channel\":\"futures.trades\",\"event\":\"unsubscribe\",\"payload\":[\"ETH_USDT\"]}",
    });
    const gatedex = aux.clients.GateDexClient.init(a, options);
    try t.expectError(error.InvalidDepth, gatedex.prepareSubscription(.{ .subscriptions = .{.{ .channel = "futures.order_book", .symbol = "BTC_USDT", .depth = 2 }} }));

    try check(a, aux.clients.VestClient, options, .{ .subscriptions = .{
        .{ .channel = "depth", .symbol = "BTC-PERP" },
        .{ .channel = "tickers" },
        .{ .channel = "kline_1m", .symbol = "ETH-PERP" },
    } }, "wss://ws-prod.hz.vestmarkets.com/ws", &.{
        "{\"method\":\"SUBSCRIBE\",\"params\":[\"BTC-PERP@depth\",\"tickers\",\"ETH-PERP@kline_1m\"],\"id\":1}",
    }, &.{
        "{\"method\":\"UNSUBSCRIBE\",\"params\":[\"BTC-PERP@depth\",\"tickers\",\"ETH-PERP@kline_1m\"],\"id\":2}",
    });
    const vest = aux.clients.VestClient.init(a, options);
    try t.expectError(error.InvalidSubscription, vest.prepareSubscription(.{ .subscriptions = .{.{ .channel = "tickers", .symbol = "BTC-PERP" }} }));
}

const pair = "550E8400-E29B-41D4-A716-446655440000";
const candle = .{ .type = "ohlcv", .tradingPairId = pair, .mode = "SPOT", .interval = "1m" };
const oracle = .{ .channel = "oracle", .market_ids = .{ 1, 2 } };

fn invalidAndOwned(a: A, options: aux.Options) !void {
    const aevo = aux.clients.AevoClient.init(a, .{});
    try t.expectError(error.EmptySubscription, aevo.prepareSubscription(.{ .subscriptions = @as([]const aux.Value, &.{}) }));
    try t.expectError(error.AuthenticationRequired, aevo.prepareSubscription(.{ .subscriptions = .{.{ .channel = "fills" }} }));
    const arcus = aux.clients.ArcusClient.init(a, options);
    try t.expectError(error.InvalidSubscription, arcus.prepareSubscription(.{ .subscriptions = .{.{ .channel = "l2OrderbookUpdates", .id = "BTC-USD", .nLevels = 101 }} }));
    const monaco = aux.clients.MonacoClient.init(a, .{});
    try t.expectError(error.InvalidInterval, monaco.prepareSubscription(.{ .subscriptions = .{.{ .type = "ohlcv", .tradingPairId = pair, .mode = "SPOT", .interval = "1min" }} }));
    try t.expectError(error.AuthenticationRequired, monaco.prepareSubscription(.{ .subscriptions = .{.{ .type = "balances" }} }));
    const risex = aux.clients.RisexClient.init(a, .{});
    try t.expectError(error.InvalidSubscription, risex.prepareSubscription(.{ .subscriptions = .{ oracle, oracle } }));
    var all_oracles = try risex.prepareSubscription(.{ .subscriptions = .{.{ .channel = "oracle" }} });
    defer all_oracles.deinit();
    try t.expectEqualStrings("{\"method\":\"subscribe\",\"params\":{\"channel\":\"oracle\",\"market_ids\":[]}}", all_oracles.value.subscribe[0]);
    const binance = aux.clients.BinanceSpotClient.init(a, .{ .endpoint = "https://example.invalid/ws" });
    try t.expectError(error.InvalidEndpoint, binance.prepareSubscription(.{ .subscriptions = .{.{ .channel = "depth", .symbol = "BTCUSDT" }} }));

    var testnet = try aux.parse(a, "{\"network\":\"Testnet\"}");
    defer testnet.deinit();
    var testnet_plan = try aux.clients.ApexOmniClient.init(a, .{ .auth = testnet.value }).prepareSubscription(.{
        .timestamp_ms = 42,
        .subscriptions = .{.{ .channel = "book", .symbol = "BTCUSDT" }},
    });
    defer testnet_plan.deinit();
    try t.expectEqualStrings("wss://qa-quote.omni.apex.exchange/realtime_public?v=2&timestamp=42", testnet_plan.value.endpoint);

    var owned = blk: {
        const endpoint = try a.dupe(u8, options.endpoint.?);
        defer a.free(endpoint);
        const symbol = try a.dupe(u8, "BTC-PERP");
        defer a.free(symbol);
        break :blk try aux.clients.AevoClient.init(a, .{ .endpoint = endpoint, .origin = "ws://example.invalid" }).prepareSubscription(.{
            .subscriptions = .{.{ .channel = "book", .symbol = symbol, .interval = "500ms" }},
        });
    };
    defer owned.deinit();
    try t.expectEqualStrings(options.endpoint.?, owned.value.endpoint);
    try t.expectEqualStrings("{\"id\":1,\"op\":\"subscribe\",\"data\":[\"orderbook-500ms:BTC-PERP\"]}", owned.value.subscribe[0]);
}

fn n1(a: A, base: []const u8) !void {
    const spec = .{ .subscriptions = .{
        .{ .channel = "trades", .symbol = "BTCUSD" },
        .{ .channel = "deltas", .symbol = "BTCUSD" },
        .{ .channel = "candle", .symbol = "BTCUSD", .resolution = "1" },
        .{ .channel = "rfq_fills", .symbol = "AAPLUSD" },
        .{ .channel = "liquidations" },
    } };
    const path = "/ws/trades@BTCUSD&deltas@BTCUSD&candle@BTCUSD:1&rfq-fills@AAPLUSD&liquidations";
    const defaults = aux.clients.N1Client.init(a, .{});
    var default_plan = try defaults.prepareSubscription(spec);
    defer default_plan.deinit();
    try t.expectEqualStrings("wss://api-mainnet.n1.xyz" ++ path, default_plan.value.endpoint);
    try t.expectError(error.InvalidSymbol, defaults.prepareSubscription(.{ .subscriptions = .{.{ .channel = "trades", .symbol = "BTCUSD&liquidations" }} }));
    try t.expectError(error.InvalidResolution, defaults.prepareSubscription(.{ .subscriptions = .{.{ .channel = "candle", .symbol = "BTCUSD", .resolution = "1m" }} }));
    try t.expectError(error.UnknownChannel, defaults.prepareSubscription(.{ .subscriptions = .{.{ .channel = "account" }} }));
    try t.expectError(error.EmptySubscription, defaults.prepareSubscription(.{ .subscriptions = @as([]const aux.Value, &.{}) }));

    const endpoint = try std.mem.concat(a, u8, &.{ base, "n1/raw/" });
    defer a.free(endpoint);
    var plan = try aux.clients.N1Client.init(a, .{ .endpoint = endpoint }).prepareSubscription(spec);
    defer plan.deinit();
    try t.expectEqual(@as(usize, 0), plan.value.subscribe.len);
    try t.expectEqual(@as(usize, 0), plan.value.unsubscribe.len);
    var socket = try aux.RawWebSocket.connect(a, plan.value.endpoint, .{ .headers = plan.value.headers });
    defer socket.deinit();
    const message = try socket.next() orelse return error.MissingMessage;
    defer a.free(message.payload);
    try t.expectEqualStrings(" {\"trades\":{\"market_symbol\":\"BTCUSD\",\"price\":1.000}} ", message.payload);
    try t.expect((try socket.next()) == null);

    const reconnect = try std.mem.concat(a, u8, &.{ base, "n1/managed" });
    defer a.free(reconnect);
    var stream = try aux.clients.N1Client.init(a, .{ .endpoint = reconnect }).subscribe(spec);
    defer stream.deinit();
    for ([_][]const u8{ "trades", "delta", "mid", "rfqFills", "liquidation" }) |key| {
        var event = try stream.next() orelse return error.MissingMessage;
        defer event.deinit();
        try t.expect(event.value.object.contains(key));
    }
}

fn kuru(a: A, io: std.Io, fixture_dir: []const u8, base: []const u8) !void {
    const spec = .{ .subscriptions = .{
        .{ .channel = "l2Delta@f", .marketAddress = "0x8121EE960066E4344A213F9AC7502395502AEE7A", .feedEpoch = 13, .seq = 9007199254740993 },
        .{ .channel = "trades", .marketAddress = "0x8121ee960066e4344a213f9ac7502395502aee7a" },
    } };
    var defaults = try aux.clients.KuruClient.init(a, .{}).prepareSubscription(spec);
    defer defaults.deinit();
    try t.expectEqualStrings("wss://gateway.mainnet.kuru.io/ws", defaults.value.endpoint);
    const endpoint = try std.mem.concat(a, u8, &.{ base, "kuru" });
    defer a.free(endpoint);
    const client = aux.clients.KuruClient.init(a, .{ .endpoint = endpoint });
    try t.expectError(error.InvalidSubscription, client.prepareSubscription(.{ .subscriptions = .{.{
        .channel = "l2Delta@f",
        .marketAddress = "0x8121ee960066e4344a213f9ac7502395502aee7a",
        .seq = 9,
    }} }));
    var plan = try client.prepareSubscription(spec);
    defer plan.deinit();
    var socket = try aux.RawWebSocket.connect(a, plan.value.endpoint, .{ .headers = plan.value.headers });
    defer socket.deinit();
    for (plan.value.subscribe) |message| try socket.send(.text, message);
    const message = try socket.next() orelse return error.MissingMessage;
    defer a.free(message.payload);
    const path = try std.fs.path.join(a, &.{ fixture_dir, "market-data-ws.json" });
    defer a.free(path);
    const contents = try std.Io.Dir.cwd().readFileAlloc(io, path, a, .limited(64 * 1024));
    defer a.free(contents);
    var rows = try aux.parse(a, contents);
    defer rows.deinit();
    const hex = for (rows.value.array.items) |row| {
        const obj = row.object;
        if (std.mem.eql(u8, obj.get("venue").?.string, "kuru") and std.mem.eql(u8, obj.get("case").?.string, "l2-bootstrap")) {
            break obj.get("binary").?.array.items[0].string;
        }
    } else return error.MissingBinaryFixture;
    const expected = try a.alloc(u8, hex.len / 2);
    defer a.free(expected);
    try t.expectEqual(aux.RawWebSocket.Kind.binary, message.kind);
    try t.expectEqualSlices(u8, try std.fmt.hexToBytes(expected, hex), message.payload);
    for (plan.value.unsubscribe) |payload| try socket.send(.text, payload);
    try t.expect((try socket.next()) == null);
}

fn managed(a: A, base: []const u8) !void {
    const monaco_endpoint = try std.mem.concat(a, u8, &.{ base, "monaco/ohlcv" });
    defer a.free(monaco_endpoint);
    var monaco = try aux.clients.MonacoClient.init(a, .{ .endpoint = monaco_endpoint }).subscribe(candle);
    defer monaco.deinit();
    var update = try monaco.next() orelse return error.MissingMessage;
    defer update.deinit();
    try t.expectEqualStrings("Event", update.value.object.get("type").?.string);
    try t.expectEqualStrings("1.000", update.value.object.get("data").?.object.get("open").?.number_string);

    const risex_endpoint = try std.mem.concat(a, u8, &.{ base, "risex/oracle" });
    defer a.free(risex_endpoint);
    var risex = try aux.clients.RisexClient.init(a, .{ .endpoint = risex_endpoint }).subscribe(oracle);
    defer risex.deinit();
    var oracle_update = try risex.next() orelse return error.MissingMessage;
    defer oracle_update.deinit();
    try t.expect(oracle_update.value.object.get("market_id").? == .null);
    try t.expectEqualStrings("63126084821834306372811", oracle_update.value.object.get("data").?.array.items[0].object.get("price").?.string);
}
