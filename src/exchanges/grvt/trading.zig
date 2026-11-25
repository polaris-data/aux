const std = @import("std");
const p = @import("../../core/perps.zig");
const common = @import("../../core/perps_connectors.zig");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");

pub fn sessionHeaders(a: p.A, config: p.V, headers: *p.V) !void {
    const auth = p.get(config, "auth");
    const cookie = try p.credential(p.get(auth, "cookie"));
    if (!std.mem.startsWith(u8, cookie, "gravity=") or cookie.len == "gravity=".len or std.mem.findScalar(u8, cookie, ';') != null)
        return error.AuthenticationRequired;
    const account_id = try p.credential(p.get(auth, "account_id"));
    try p.put(headers, a, "Cookie", w.text(cookie));
    try p.put(headers, a, "X-Grvt-Account-Id", w.text(account_id));
}

pub fn loginRequest(a: p.A, config: p.V, headers: *p.V) !p.V {
    const key = try p.credential(p.get(config, "api_key"));
    try p.put(headers, a, "Cookie", w.text("rm=true"));
    return p.toValue(a, .{ .api_key = key });
}

pub fn loginResponse(a: p.A, result: p.V, headers: []const std.http.Header) !p.V {
    if (!p.eq(u8, try p.text(result, "status"), "success")) return error.AuthenticationRequired;
    var cookie: ?[]const u8 = null;
    var account: ?[]const u8 = null;
    for (headers) |header| {
        if (std.ascii.eqlIgnoreCase(header.name, "set-cookie")) {
            const first = std.mem.trim(u8, header.value[0 .. std.mem.findScalar(u8, header.value, ';') orelse header.value.len], " \t");
            if (std.mem.startsWith(u8, first, "gravity=")) cookie = try p.credential(w.text(first));
        }
        if (std.ascii.eqlIgnoreCase(header.name, "x-grvt-account-id")) account = std.mem.trim(u8, header.value, " \t");
    }
    var response = result;
    try p.put(&response, a, "cookie", w.text(cookie orelse return error.InvalidResponse));
    try p.put(&response, a, "account_id", w.text(account orelse return error.InvalidResponse));
    return response;
}

pub fn order(a: p.A, config: p.V, source: p.V, io: std.Io) !p.V {
    var nonce_data: [4]u8 = undefined;
    try io.randomSecure(&nonce_data);
    const nonce = std.mem.readInt(u32, &nonce_data, .little);
    const now = try p.epoch(io, .ms);
    const duration: u64 = if (try common.boolean(source, "is_market")) 60_000 else 86_400_000;
    return signedOrder(a, config, source, nonce, (now + duration) * 1_000_000);
}

fn signedOrder(a: p.A, config: p.V, source: p.V, nonce: u32, expiration: u64) !p.V {
    if (source != .object or p.get(source, "signature") != .null or p.get(source, "builder") != .null or p.get(source, "builder_fee") != .null)
        return error.InvalidParameter;
    const sub_account = try w.unsigned(u64, try w.field(source, "sub_account_id"));
    const is_market = try common.boolean(source, "is_market");
    const post_only = try common.boolean(source, "post_only");
    const reduce_only = try common.boolean(source, "reduce_only");
    const tif = 1 + try common.enumIndex(try p.text(source, "time_in_force"), &.{ "GOOD_TILL_TIME", "ALL_OR_NONE", "IMMEDIATE_OR_CANCEL", "FILL_OR_KILL" });
    if (is_market and (post_only or tif < 3)) return error.InvalidParameter;
    const legs = try w.field(source, "legs");
    if (legs != .array or legs.array.items.len != 1) return error.InvalidParameter;
    const leg = legs.array.items[0];
    const metadata = p.get(p.get(p.get(config, "metadata"), "instrument"), "result");
    const instrument = try p.text(leg, "instrument");
    if (!p.eq(u8, instrument, try p.text(metadata, "instrument"))) return error.InvalidMarket;
    const asset = try @import("../../core/transaction.zig").quantity(u256, try w.field(metadata, "instrument_hash"));
    const scale = try w.field(metadata, "base_decimals");
    const decimals = try std.fmt.parseInt(i8, if (scale == .number_string) scale.number_string else try w.string(scale), 10);
    const size_text = try p.text(leg, "size");
    const size = if (decimals < 0) scaledSize: {
        const unit = try std.math.powi(u64, 10, @abs(decimals));
        const whole = try common.decimal(u64, size_text, 0);
        if (whole % unit != 0) return error.InvalidQuantity;
        break :scaledSize whole / unit;
    } else try common.decimal(u64, size_text, @intCast(decimals));
    const price = try common.decimal(u64, try p.text(leg, "limit_price"), 9);
    if (size == 0 or (!is_market and price == 0)) return error.InvalidQuantity;
    const buying = try w.field(leg, "is_buying_asset");
    if (buying != .bool) return error.InvalidParameter;
    if (p.eq(u8, try p.text(metadata, "kind"), "STABLE_PERP") and price == 0) return error.InvalidQuantity;
    const client_id = try p.text(p.get(source, "metadata"), "client_order_id");
    if ((try std.fmt.parseUnsigned(u64, client_id, 10)) == 0) return error.InvalidParameter;

    const order_leg = "OrderLeg(uint256 assetID,uint64 contractSize,uint64 limitPrice,bool isBuyingContract)";
    const leg_hash = evm.hash(&(evm.hash(order_leg) ++ evm.encodeWord(asset) ++ evm.encodeWord(size) ++ evm.encodeWord(price) ++ evm.encodeWord(@intFromBool(buying.bool))));
    const order_type = "Order(uint64 subAccountID,bool isMarket,uint8 timeInForce,bool postOnly,bool reduceOnly,OrderLeg[] legs,uint32 nonce,int64 expiration)";
    const order_hash = evm.hash(&(evm.hash(order_type ++ order_leg) ++ evm.encodeWord(sub_account) ++ evm.encodeWord(@intFromBool(is_market)) ++
        evm.encodeWord(tif) ++ evm.encodeWord(@intFromBool(post_only)) ++ evm.encodeWord(@intFromBool(reduce_only)) ++
        evm.hash(&leg_hash) ++ evm.encodeWord(nonce) ++ evm.encodeWord(expiration)));
    const chain: u64 = if (try p.testnet(config)) 326 else 325;
    const domain = evm.hash(&(evm.hash("EIP712Domain(string name,string version,uint256 chainId)") ++
        evm.hash("GRVT Exchange") ++ evm.hash("0") ++ evm.encodeWord(chain)));
    const digest = evm.hash(&(.{ 0x19, 0x01 } ++ domain ++ order_hash));
    var key = try p.evmKey(p.get(config, "auth"));
    defer std.crypto.secureZero(u8, &key);
    const signer = try p.evmAddress(key);
    const signature = try @import("../../core/transaction.zig").recoverable(key, digest);
    var payload = source;
    try p.put(&payload, a, "signature", try p.toValue(a, .{
        .signer = try std.fmt.allocPrint(a, "0x{x}", .{&signer}),
        .r = try std.fmt.allocPrint(a, "0x{x}", .{signature[0..32]}),
        .s = try std.fmt.allocPrint(a, "0x{x}", .{signature[32..64]}),
        .v = signature[64],
        .nonce = nonce,
        .expiration = try std.fmt.allocPrint(a, "{d}", .{expiration}),
        .chain_id = try std.fmt.allocPrint(a, "{d}", .{chain}),
    }));
    return p.toValue(a, .{ .order = payload });
}

pub fn cancel(body: p.V) !void {
    if (body != .object or (p.get(body, "order_id") == .null) == (p.get(body, "client_order_id") == .null))
        return error.InvalidParameter;
    _ = try w.unsigned(u64, try w.field(body, "sub_account_id"));
}

test "GRVT order signature matches eth-account" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const config = try p.toValue(a, .{
        .auth = .{ .key = "0x0000000000000000000000000000000000000000000000000000000000000001" },
        .metadata = .{ .instrument = .{ .result = .{
            .instrument = "BTC_USDT_Perp",
            .instrument_hash = "0x11",
            .kind = "PERPETUAL",
            .base_decimals = 9,
        } } },
    });
    const input = try p.toValue(a, .{
        .sub_account_id = "123456789",
        .is_market = false,
        .time_in_force = "GOOD_TILL_TIME",
        .legs = .{.{ .instrument = "BTC_USDT_Perp", .size = "0.01", .limit_price = "50000", .is_buying_asset = true }},
        .metadata = .{ .client_order_id = "123" },
    });
    const proof = p.get(p.get(try signedOrder(a, config, input, 4242, 1780000000000000000), "order"), "signature");
    try std.testing.expectEqualStrings("0x3effed61b4ace1fa0fa1fcaaaf853ca7eb0e5b0d6b16e7b6ddf01ca935452216", try p.text(proof, "r"));
    try std.testing.expectEqualStrings("0x24ecdf188740f0c363974715cc2a8676c718f90dbaf493f35070e8bf5160fa21", try p.text(proof, "s"));
    try std.testing.expectEqual(@as(u8, 28), try w.unsigned(u8, p.get(proof, "v")));
}
