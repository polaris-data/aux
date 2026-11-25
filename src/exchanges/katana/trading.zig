const std = @import("std");
const p = @import("../../core/perps.zig");
const common = @import("../../core/perps_connectors.zig");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const Hmac = std.crypto.auth.hmac.sha2.HmacSha256;

const Nonce = struct {
    bytes: [16]u8,

    fn fresh(io: std.Io) !Nonce {
        const now = std.Io.Timestamp.now(io, .real).toNanoseconds();
        const ticks = std.math.cast(u60, @divTrunc(now, 100) + 122_192_928_000_000_000) orelse return error.InvalidTimestamp;
        var bytes: [16]u8 = undefined;
        try io.randomSecure(bytes[8..16]);
        std.mem.writeInt(u32, bytes[0..4], @truncate(ticks), .big);
        std.mem.writeInt(u16, bytes[4..6], @truncate(ticks >> 32), .big);
        std.mem.writeInt(u16, bytes[6..8], @as(u16, @intCast(ticks >> 48)) | 0x1000, .big);
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        bytes[10] |= 1;
        return .{ .bytes = bytes };
    }

    fn text(self: Nonce, a: p.A) ![]const u8 {
        return std.fmt.allocPrint(a, "{x}-{x}-{x}-{x}-{x}", .{ self.bytes[0..4], self.bytes[4..6], self.bytes[6..8], self.bytes[8..10], self.bytes[10..16] });
    }
};

pub fn query(a: p.A, path: []const u8, io: std.Io) ![]const u8 {
    const nonce = try Nonce.fresh(io);
    return std.fmt.allocPrint(a, "{s}{s}nonce={s}", .{ path, if (std.mem.findScalar(u8, path, '?') == null) "?" else "&", try nonce.text(a) });
}

pub fn headers(a: p.A, config: p.V, method: []const u8, path: []const u8, payload: []const u8, output: *p.V) !void {
    const key = try p.credential(p.get(config, "api_key"));
    const secret = try p.credential(p.get(config, "api_secret"));
    const bytes = if (p.eq(u8, method, "GET")) path[(std.mem.findScalar(u8, path, '?') orelse return error.InvalidPath) + 1 ..] else payload;
    var mac: [Hmac.mac_length]u8 = undefined;
    Hmac.create(&mac, bytes, secret);
    try p.put(output, a, "KP-API-KEY", w.text(key));
    try p.put(output, a, "KP-HMAC-SIGNATURE", w.text(try std.fmt.allocPrint(a, "{x}", .{&mac})));
}

fn hashOrder(a: p.A, spec: *p.V, nonce: p.Word, wallet: p.Word) !p.Word {
    const market = try p.text(spec.*, "market");
    const kind = try common.enumIndex(try p.text(spec.*, "type"), &.{ "market", "limit" });
    const side = try common.enumIndex(try p.text(spec.*, "side"), &.{ "buy", "sell" });
    const precision = 8;
    const quantity = try p.text(spec.*, "quantity");
    if (market.len == 0 or try common.decimal(u128, quantity, precision) == 0) return error.InvalidQuantity;
    const zero = "0.00000000";
    const price = if (kind == 1) try p.text(spec.*, "price") else zero;
    if (kind == 1 and try common.decimal(u128, price, precision) == 0) return error.InvalidQuantity;
    if (kind == 0 and p.get(spec.*, "price") != .null) return error.InvalidParameter;
    const tif = if (p.get(spec.*, "timeInForce") == .null) 0 else try common.enumIndex(try p.text(spec.*, "timeInForce"), &.{ "gtc", "gtx", "ioc", "fok" });
    const stp = if (p.get(spec.*, "selfTradePrevention") == .null) 0 else try common.enumIndex(try p.text(spec.*, "selfTradePrevention"), &.{ "dc", "co", "cn", "cb" });
    if ((kind == 0 and tif != 0) or (tif == 3 and stp != 2)) return error.InvalidParameter;
    const client_id = if (p.get(spec.*, "clientOrderId") == .null) "" else try p.text(spec.*, "clientOrderId");
    if (client_id.len > 40) return error.InvalidParameter;
    const reduce = try common.boolean(spec.*, "reduceOnly");
    switch (spec.*) {
        .object => for (spec.object.keys()) |field| {
            if (!allowed(field)) return error.InvalidParameter;
        },
        else => return error.InvalidParameter,
    }
    // Katana expects string quantities in the signed payload with eight places.
    const quantity_text = try decimal8(a, quantity);
    const price_text = try decimal8(a, price);
    try p.put(spec, a, "quantity", w.text(quantity_text));
    if (kind == 1) try p.put(spec, a, "price", w.text(price_text));
    const type_hash = evm.hash("Order(uint128 nonce,address wallet,string marketSymbol,uint8 orderType,uint8 orderSide,string quantity,string limitPrice,string triggerPrice,uint8 triggerType,string callbackRate,uint128 conditionalOrderId,bool isReduceOnly,uint8 timeInForce,uint8 selfTradePrevention,bool isLiquidationAcquisitionOnly,address delegatedPublicKey,string clientOrderId)");
    return evm.hash(&(type_hash ++ nonce ++ wallet ++ evm.hash(market) ++ evm.encodeWord(kind) ++ evm.encodeWord(side) ++
        evm.hash(quantity_text) ++ evm.hash(price_text) ++ evm.hash(zero) ++ evm.encodeWord(0) ++ evm.hash(zero) ++ evm.encodeWord(0) ++
        evm.encodeWord(@intFromBool(reduce)) ++ evm.encodeWord(tif) ++ evm.encodeWord(stp) ++ evm.encodeWord(0) ++ evm.encodeWord(0) ++ evm.hash(client_id)));
}

fn allowed(field: []const u8) bool {
    for ([_][]const u8{ "wallet", "market", "type", "side", "quantity", "price", "clientOrderId", "reduceOnly", "timeInForce", "selfTradePrevention" }) |name|
        if (p.eq(u8, name, field)) return true;
    return false;
}

fn decimal8(a: p.A, text: []const u8) ![]const u8 {
    const units = try common.decimal(u128, text, 8);
    return std.fmt.allocPrint(a, "{d}.{d:0>8}", .{ units / 100_000_000, units % 100_000_000 });
}

pub fn order(a: p.A, config: p.V, name: []const u8, input: p.V, io: std.Io) !p.V {
    return signedOrder(a, config, name, input, try Nonce.fresh(io));
}

fn signedOrder(a: p.A, config: p.V, name: []const u8, input: p.V, nonce: Nonce) !p.V {
    if (input != .object) return error.InvalidParameter;
    const auth = p.get(config, "auth");
    const wallet = try p.text(input, "wallet");
    const address = try evm.hex(20, wallet);
    var key = try p.evmKey(auth);
    defer std.crypto.secureZero(u8, &key);
    if (!std.mem.eql(u8, &address, &try p.evmAddress(key))) return error.InvalidIdentity;
    const nonce_word = evm.encodeWord(std.mem.readInt(u128, &nonce.bytes, .big));
    const address_word = try evm.address(wallet);
    var parameters = input;
    const hash = if (p.eq(u8, name, "place_order")) try hashOrder(a, &parameters, nonce_word, address_word) else blk: {
        const id = try p.text(input, "orderId");
        if (id.len == 0 or input.object.count() != 2) return error.InvalidParameter;
        parameters = try p.toValue(a, .{ .wallet = wallet, .orderIds = .{id} });
        break :blk evm.hash(&(evm.hash("OrderCancellationByOrderId(uint128 nonce,address wallet,address delegatedKey,string[] orderIds)") ++
            nonce_word ++ address_word ++ evm.encodeWord(0) ++ evm.hash(&evm.hash(id))));
    };
    try p.put(&parameters, a, "nonce", w.text(try nonce.text(a)));
    const sandbox = try p.testnet(config);
    const chain: u64 = if (sandbox) 737373 else 747474;
    const contract = if (sandbox) "0x92d3072dDe1aD3e9B7895500F504aA5e664E71d3" else "0x62230CeA619F734cc215bB8074bbF07bE4Eb633e";
    const digest = p.typedHash("KatanaPerps", if (sandbox) "2.0.0-sandbox" else "2.0.0", chain, try evm.address(contract), hash);
    return p.toValue(a, .{ .parameters = parameters, .signature = try p.signature(a, key, digest) });
}

test "Katana order and cancellation signatures match eth-account" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const config = try p.toValue(a, .{ .auth = .{ .key = "0x0000000000000000000000000000000000000000000000000000000000000001" } });
    const nonce: Nonce = .{ .bytes = try evm.hex(16, "075996e0b7cb11eebe5d2f6099650fc5") };
    const wallet = "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf";
    const place = try signedOrder(a, config, "place_order", try p.toValue(a, .{
        .wallet = wallet,
        .type = "limit",
        .side = "buy",
        .market = "ETH-USD",
        .price = "500",
        .quantity = "1",
        .clientOrderId = "fixture-1",
    }), nonce);
    const cancel = try signedOrder(a, config, "cancel_order", try p.toValue(a, .{
        .wallet = wallet,
        .orderId = "3a9ef9c0-a779-11ea-907d-23e999279287",
    }), nonce);
    try std.testing.expectEqualStrings("0x4782390f1d2681598c9726cb86eee248822de3b1f1625871b3099c7a9c725a2f58b6cbe12bc31d42e682c9fc1cb0bcfa90f86597b8685d65232b1cd6e4b711441b", try p.text(place, "signature"));
    try std.testing.expectEqualStrings("0x6717e4c53686a44b7788d596dd2d1481c1a1d1b56771fb632913e78798d6e57f2e70982861e177120d373c280222255abbc7a863f6850499b7bc9bc58d9cb8181b", try p.text(cancel, "signature"));
}
