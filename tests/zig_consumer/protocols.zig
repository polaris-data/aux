const std = @import("std");
const aux = @import("aux");
const check = @import("compare.zig").compare;

fn fixture(init: std.process.Init, dir: []const u8, file: []const u8) !aux.Owned(aux.Value) {
    const a = init.arena.allocator();
    const path = try std.fs.path.join(a, &.{ dir, file });
    return aux.parse(a, try std.Io.Dir.cwd().readFileAlloc(init.io, path, a, .limited(16 * 1024 * 1024)));
}

pub fn run(init: std.process.Init, client: aux.Client, args: []const []const u8) !void {
    @setEvalBranchQuota(200000);
    const a = init.arena.allocator();
    var instructions = try fixture(init, args[2], "phoenix-instructions.json");
    defer instructions.deinit();
    inline for (@typeInfo(aux.workflows.instructions).@"struct".decl_names) |name| {
        const row = instructions.value.object.get(name).?;
        var ix = try @field(aux.workflows.instructions, name)(client, row.object.get("request").?);
        defer ix.deinit();
        try check(ix.value, row.object.get("instruction").?);
    }
    var abi = try fixture(init, args[3], "contracts.json");
    defer abi.deinit();
    var methods: std.StringHashMapUnmanaged(ContractTest) = .empty;
    for (contract_tests) |entry| try methods.put(a, entry[0], entry[1]);
    for (abi.value.array.items) |row| {
        const o = row.object;
        const address = "0x1111111111111111111111111111111111111111";
        const endpoint = try std.mem.concat(a, u8, &.{ args[1], "/abi/", o.get("contract").?.string });
        const chain: aux.chain.Evm = .{ .client = client, .chain_id = 1, .read_rpc = endpoint, .write_rpc = endpoint };
        const key = try std.mem.concat(a, u8, &.{ o.get("contract").?.string, ".", o.get("method").?.string });
        try (methods.get(key) orelse return error.MissingContractMethod)(chain, address, row);
    }
    var diagnostics: aux.Diagnostic = .{};
    defer diagnostics.deinit();
    try offline(init, client, args[2]);
    try pascalPermits(init, client, args[2]);
    try authChecks(client, args[5]);
    try liveCheck(init, client, args);
    try perpsCheck(init, client.allocator, args[5], args[2]);
    try feedsCheck(client.allocator, args[5]);
    var decoder = try aux.codec.EventDecoder.init(64);
    defer decoder.deinit();
    try decoder.push("data: first\r");
    try std.testing.expect((try decoder.next()) == null);
    try decoder.push("\n\r\ndata: second\n\n");
    try std.testing.expectEqualStrings("first", (try decoder.next()).?);
    try std.testing.expectEqualStrings("second", (try decoder.next()).?);
    try std.testing.expect((try decoder.next()) == null);
    try decoder.finish();
    var hexbuf: [4]u8 = undefined;
    const encoded = try aux.codec.hexEncode(&.{ 0xca, 0xfe }, &hexbuf);
    try std.testing.expectEqualStrings("cafe", encoded);
    var decoded: [2]u8 = undefined;
    try std.testing.expectEqualSlices(u8, &.{ 0xca, 0xfe }, try aux.codec.hexDecode(encoded, &decoded));
    const derive = aux.clients.DeriveClient.init(client.allocator, .{ .endpoint = args[4], .diagnostic = &diagnostics });
    var rpc = try derive.open_rpc(.{});
    defer rpc.deinit();
    try std.testing.expectError(error.ExchangeApi, aux.sessions.derive.subscribe_channels(&rpc, .{ .channels = .{"rejection-fixture"} }));
    const frame = diagnostics.detail.?.value.object.get("body").?;
    try std.testing.expectEqualStrings("9007199254740993", frame.object.get("error").?.object.get("data").?.object.get("seq").?.number_string);
    var unsubscribed = try aux.sessions.derive.unsubscribe_channels(&rpc, .{ .channels = .{"trades"} });
    defer unsubscribed.deinit();
    const dbn_file = try std.Io.Dir.cwd().readFileAlloc(init.io, try std.fs.path.join(a, &.{ args[2], "schema-records.dbn.zst" }), a, .limited(1024 * 1024));
    var dbn = try client.openDbn(dbn_file);
    defer dbn.deinit();
    var records = try fixture(init, args[2], "schema-records.json");
    defer records.deinit();
    var metadata = (try dbn.next()).?;
    defer metadata.deinit();
    try std.testing.expectEqualStrings("metadata", metadata.value.object.get("type").?.string);
    for (records.value.array.items) |row| {
        var record = (try dbn.next()).?;
        defer record.deinit();
        try check(record.value, row.object.get("record").?);
    }
    try std.testing.expect((try dbn.next()) == null);
}

const ContractTest = *const fn (aux.chain.Evm, []const u8, aux.Value) anyerror!void;
fn verifyContract(comptime C: type, comptime method: []const u8) ContractTest {
    return struct {
        fn run(chain: aux.chain.Evm, address: []const u8, row: aux.Value) anyerror!void {
            const f = @field(C, method);
            const contract = C.init(chain, address);
            const args = row.object.get("args").?;
            if (@typeInfo(@TypeOf(f)).@"fn".param_types[2].? == aux.chain.ReadOptions) {
                var decoded = try f(contract, args, .{ .block = "finalized" });
                defer decoded.deinit();
                try compareAbi(decoded.value, row.object.get("outputArgs").?);
            } else {
                var tx = try f(contract, args, "0x10");
                defer tx.deinit();
                try std.testing.expectEqualStrings(address, tx.value.object.get("to").?.string);
                try std.testing.expectEqualStrings("0x10", tx.value.object.get("value").?.string);
                try check(tx.value.object.get("data").?, row.object.get("input").?);
            }
        }
    }.run;
}

const contract_tests = blk: {
    @setEvalBranchQuota(100000);
    var count = 0;
    for (@typeInfo(aux.contracts).@"struct".decl_names) |name| count += @typeInfo(@field(aux.contracts, name)).@"struct".decl_names.len - 1;
    var result: [count]struct { []const u8, ContractTest } = undefined;
    var i = 0;
    for (@typeInfo(aux.contracts).@"struct".decl_names) |name| {
        const C = @field(aux.contracts, name);
        for (@typeInfo(C).@"struct".decl_names) |method| {
            if (std.mem.eql(u8, method, "init")) continue;
            result[i] = .{ name ++ "." ++ method, verifyContract(C, method) };
            i += 1;
        }
    }
    break :blk result;
};

fn compareAbi(actual: aux.Value, vector: aux.Value) anyerror!void {
    if (actual == .array and vector == .array) {
        try std.testing.expectEqual(vector.array.items.len, actual.array.items.len);
        for (actual.array.items, vector.array.items) |item, expected| try compareAbi(item, expected);
    } else if (actual == .number_string and vector == .string) {
        try std.testing.expectEqualStrings(vector.string, actual.number_string);
    } else try check(actual, vector);
}

fn kalshiSign(_: ?*anyopaque, a: std.mem.Allocator, payload: []const u8) !aux.Value {
    try std.testing.expect(std.mem.endsWith(u8, payload, "GET/external-kalshi/portfolio/balance"));
    const signer: aux.Client = .{ .allocator = a };
    var proof = try signer.invoke(.arcus_sign, .{ .payload = payload, .seed = "0707070707070707070707070707070707070707070707070707070707070707" });
    defer proof.deinit();
    return .{ .string = try a.dupe(u8, proof.value.object.get("signature").?.string) };
}
fn walletSign(_: ?*anyopaque, _: std.mem.Allocator, message: []const u8) !aux.Value {
    try std.testing.expect(message.len != 0);
    return .{ .string = "0xabcd" };
}
fn authChecks(base: aux.Client, origin: []const u8) !void {
    const a = base.allocator;
    const url = try std.mem.concat(a, u8, &.{ origin, "/external-kalshi" });
    defer a.free(url);
    const kalshi = aux.clients.KalshiClient.init(a, .{ .origin = url, .api_key = "fixture-k", .signer = .{ .sign = kalshiSign } });
    var balance = try kalshi.runtime.request(aux.Value, "kalshi", "predictions.get_balance", .{ .subaccount = 1 });
    defer balance.deinit();
    try std.testing.expectEqualStrings("123", balance.value.object.get("balance").?.number_string);
    const monaco = aux.clients.MonacoClient.init(a, .{ .origin = origin });
    const options = .{ .client_id = "login", .address = "0x2121212121212121212121212121212121212121", .seed = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f", .chain_id = 1329 };
    var session = try monaco.login(options, .{ .sign = walletSign });
    defer session.deinit();
    var info = try session.info();
    defer info.deinit();
    var restored = try monaco.restore_session(options, info.value);
    defer restored.deinit();
    try restored.refresh();
    try restored.revoke();
}

fn liveCheck(init: std.process.Init, client: aux.Client, args: []const []const u8) !void {
    var auth = try fixture(init, args[2], "auth.json");
    defer auth.deinit();
    const key = auth.value.object.get("databento").?.object.get("key").?.string;
    const live = aux.clients.DatabentoLiveClient.init(client.allocator, .{ .api_key = key, .timeout_ms = 5000, .idle_timeout_ms = 5000 });
    var stream = try live.connect(.{ .dataset = "DBN", .gateway = args[6] });
    defer stream.deinit();
    var id = try aux.sessions.databento.session_id(&stream);
    defer id.deinit();
    try std.testing.expectEqualStrings("fixture-session", id.value.string);
    var subscribed = try aux.sessions.databento.subscribe(&stream, .{ .schema = "mbo", .symbols = .{"ETH"} });
    defer subscribed.deinit();
    var start = try aux.sessions.databento.start_session(&stream);
    defer start.deinit();
    var header = try aux.sessions.databento.read_batch(&stream, 1);
    defer header.deinit();
    var meta = try aux.sessions.databento.metadata(&stream);
    defer meta.deinit();
    try check(meta.value, header.value.array.items[0].object.get("metadata").?);
    var records = try aux.sessions.databento.read_batch(&stream, 24);
    defer records.deinit();
    try std.testing.expectEqual(@as(usize, 23), records.value.array.items.len);
}

fn hostSend(_: ?*anyopaque, a: std.mem.Allocator, request: aux.http.Request) !aux.http.Response {
    if (std.mem.eql(u8, request.method, "POST")) {
        var parsed = try aux.parse(a, request.body.?);
        defer parsed.deinit();
        try std.testing.expectEqualStrings("eth_chainId", parsed.value.object.get("method").?.string);
        const body = try std.json.Stringify.valueAlloc(a, .{ .jsonrpc = "2.0", .result = "0x539", .id = parsed.value.object.get("id").? }, .{});
        return .{ .status = 200, .body = body };
    }
    try std.testing.expectEqualStrings("GET", request.method);
    try std.testing.expectEqualStrings("https://fixture.invalid/v2/price_feeds?query=ETH%2FUSD", request.url);
    return .{ .status = 200, .body = "[{\"id\":\"ETH/USD\"}]" };
}

fn offline(init: std.process.Init, client: aux.Client, dir: []const u8) !void {
    const a = client.allocator;
    const host: aux.Client = .{ .allocator = a, .options = .{ .origin = "https://fixture.invalid", .transport = .{ .send = hostSend } } };
    const hermes: aux.clients.PythHermesClient = .{ .runtime = host };
    var feeds = try hermes.price_feeds(.{ .query = "ETH/USD" });
    defer feeds.deinit();
    try std.testing.expectEqual(@as(u16, 200), feeds.value.status);
    const reader: aux.chain.Reader = .{ .client = host, .endpoint = "https://fixture.invalid/" };
    var chain = try reader.read("chain_id", .{});
    defer chain.deinit();
    try std.testing.expectEqualStrings("1337", chain.value.number_string);
    var evm = try fixture(init, dir, "evm-signing.json");
    defer evm.deinit();
    for (evm.value.array.items) |v| {
        const eth: aux.wallet.Ethereum = .{ .client = client, .identity = .{ .key = v.object.get("key").?.string } };
        var signed = try eth.signTransaction(v.object.get("prepared").?);
        defer signed.deinit();
        try check(signed.value.object.get("hash").?, v.object.get("hash").?);
        try check(signed.value.object.get("signed").?, v.object.get("signed").?);
    }
    var poly = try fixture(init, dir, "polymarket-abi.json");
    defer poly.deinit();
    for (poly.value.array.items) |v| {
        const chain_cfg: aux.chain.Evm = .{ .client = client, .read_rpc = "", .write_rpc = "", .chain_id = 137 };
        const contract: aux.chain.Contract = .{ .chain = chain_cfg, .venue = .polymarket, .name = v.object.get("interface").?.string, .address = "0x1111111111111111111111111111111111111111" };
        var tx = try contract.prepare(v.object.get("function").?.string, v.object.get("args").?, "0x0");
        defer tx.deinit();
        try check(tx.value.object.get("data").?, v.object.get("calldata").?);
    }
    var solana = try aux.chain.Solana.phoenix(client, "http://127.0.0.1:1");
    defer solana.deinit();
    var pending = try aux.workflows.phoenix_chain.pending(solana, .{});
    defer pending.deinit();
    try std.testing.expect(pending.value == .null);
}

fn feedsCheck(a: std.mem.Allocator, origin: []const u8) !void {
    const first = try std.mem.concat(a, u8, &.{ "ws", origin[4..], "/pro/a" });
    defer a.free(first);
    const second = try std.mem.concat(a, u8, &.{ "ws", origin[4..], "/pro/b" });
    defer a.free(second);
    const pro = aux.clients.PythProClient.init(a, .{ .api_key = "fixture", .endpoints = &.{ first, second } });
    var prices = try pro.subscribe(.{ .channel = "real_time", .symbols = .{"BTC/USD"}, .formats = .{"evm"}, .subscriptionId = 42 });
    defer prices.deinit();
    var got_price = false;
    var got_binary = false;
    for (0..2) |_| {
        var event = (try prices.next()).?;
        defer event.deinit();
        if (event.value.object.get("price")) |price| {
            try std.testing.expectEqualStrings("9007199254740993", price.number_string);
            got_price = true;
        } else {
            try std.testing.expectEqualStrings("1.234567890123456789", event.value.object.get("parsed").?.object.get("price").?.string);
            got_binary = true;
        }
    }
    try std.testing.expect(got_price and got_binary);
    const tardis = aux.clients.TardisClient.init(a, .{ .origin = origin, .api_key = "fixture" });
    var replay = try tardis.replay(.{ .exchange = "bit/mex", .from = "2024-02-29T23:59:00Z", .to = "2024-03-01T00:02:00+00:00", .filters = .{.{ .channel = "trade", .symbols = .{"BTC/USD"} }} });
    defer replay.deinit();
    var events = try replay.readBatch(4);
    defer events.deinit();
    try std.testing.expectEqual(@as(usize, 3), events.value.array.items.len);
    try std.testing.expectEqualStrings("disconnect", events.value.array.items[1].object.get("type").?.string);
    try std.testing.expectEqualStrings("9007199254740993", events.value.array.items[0].object.get("message").?.object.get("n").?.number_string);
}

fn signPermit(ctx: ?*anyopaque, a: std.mem.Allocator, message: []const u8) !aux.Value {
    const seed: *const []const u8 = @ptrCast(@alignCast(ctx.?));
    var proof = try aux.workflows.signer.solana_sign(.{ .allocator = a }, .{ .key = seed.*, .message = message });
    defer proof.deinit();
    return .{ .string = try a.dupe(u8, proof.value.string) };
}

fn pascalPermits(init: std.process.Init, base: aux.Client, dir: []const u8) !void {
    var vectors = try fixture(init, dir, "pascal-permits.json");
    defer vectors.deinit();
    for (vectors.value.array.items) |row| {
        const auth = row.object.get("auth").?;
        const operation = row.object.get("operation").?.string;
        const body = row.object.get("body").?;
        const timestamp = 1731536000000;
        const local = aux.clients.PascalClient.init(base.allocator, .{ .auth = auth });
        var expected = try local.sign_permit_at(operation, body, timestamp);
        defer expected.deinit();
        var seed = auth.object.get("seed").?.string;
        var identity = aux.Value{ .object = .empty };
        defer identity.object.deinit(base.allocator);
        try identity.object.put(base.allocator, "public_key", row.object.get("signer").?);
        try identity.object.put(base.allocator, "owner", auth.object.get("owner").?);
        const remote = aux.clients.PascalClient.init(base.allocator, .{ .auth = identity, .signer = .{ .context = @ptrCast(&seed), .sign = signPermit } });
        var signed = try remote.sign_permit_at(operation, body, timestamp);
        defer signed.deinit();
        try check(signed.value, expected.value);
        try check(signed.value.object.get("auth").?.object.get("signature").?, row.object.get("signature").?);
    }
}

fn perpsCheck(init: std.process.Init, a: std.mem.Allocator, origin: []const u8, fixtures: []const u8) !void {
    var rows = try fixture(init, fixtures, "perps-http.json");
    defer rows.deinit();
    const clients = .{
        .{ "injective", aux.clients.InjectiveClient },  .{ "nado", aux.clients.NadoClient },
        .{ "ondo_perps", aux.clients.OndoPerpsClient }, .{ "pacifica", aux.clients.PacificaClient },
        .{ "paragon", aux.clients.ParagonClient },      .{ "trade_xyz", aux.clients.TradeXyzClient },
        .{ "aster", aux.clients.AsterClient },
    };
    inline for (clients) |entry| {
        const venue = entry[0];
        const Client = entry[1];
        const url = try std.mem.concat(a, u8, &.{ origin, "/perps/", venue });
        defer a.free(url);
        const client = Client.init(a, .{ .origin = url });
        for (rows.value.array.items) |row| {
            if (std.mem.eql(u8, row.object.get("venue").?.string, venue) and
                std.mem.eql(u8, row.object.get("operation").?.string, "markets"))
            {
                const model = @field(aux.clients, venue).MarketsRequest;
                const request_bytes = try std.json.Stringify.valueAlloc(a, row.object.get("request").?, .{});
                defer a.free(request_bytes);
                var request = try aux.parseAs(model, a, request_bytes);
                defer request.deinit();
                var reply = try client.markets(request.value);
                defer reply.deinit();
                const bytes = try std.json.Stringify.valueAlloc(a, reply.value, .{});
                defer a.free(bytes);
                var wire = try aux.parse(a, bytes);
                defer wire.deinit();
                try check(wire.value, row.object.get("response").?);
            }
        }
    }
    const url = try std.mem.concat(a, u8, &.{ origin, "/perps/injective" });
    defer a.free(url);
    const injective = aux.clients.InjectiveClient.init(a, .{ .origin = url });
    var stream = try injective.subscribe(.{ .channel = "order_book", .market_id = "0x3333333333333333333333333333333333333333333333333333333333333333" });
    defer stream.deinit();
    var event = (try stream.next()).?;
    defer event.deinit();
    try std.testing.expectEqualStrings("9007199254740993", event.value.object.get("orderbook").?.object.get("sequence").?.number_string);
    try std.testing.expect((try stream.next()) == null);
}
