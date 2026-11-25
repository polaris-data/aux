const std = @import("std");
const p = @import("../../core/perps.zig");
const common = @import("../../core/perps_connectors.zig");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const Sha256 = std.crypto.hash.sha2.Sha256;
const Hmac = std.crypto.auth.hmac.sha2.HmacSha256;

fn less(_: void, left: []const u8, right: []const u8) bool {
    return std.mem.lessThan(u8, left, right);
}

fn flatten(a: p.A, value: p.V, out: *std.ArrayList(u8)) anyerror!void {
    switch (value) {
        .object => {
            const keys = try a.dupe([]const u8, value.object.keys());
            std.mem.sort([]const u8, keys, {}, less);
            for (keys, 0..) |key, i| {
                if (i != 0) try out.append(a, '&');
                try out.appendSlice(a, key);
                try out.append(a, '=');
                try flatten(a, value.object.get(key).?, out);
            }
        },
        .array => for (value.array.items, 0..) |item, i| {
            if (i != 0) try out.append(a, '&');
            try flatten(a, item, out);
        },
        .string => |s| try out.appendSlice(a, s),
        .number_string => |s| try out.appendSlice(a, s),
        .bool => |flag| try out.appendSlice(a, if (flag) "true" else "false"),
        .null => {},
        else => return error.InvalidParameter,
    }
}

pub fn headers(a: p.A, config: p.V, method: []const u8, path: []const u8, body: p.V, timestamp: []const u8, into: *p.V) !void {
    const auth = p.get(config, "auth");
    const key = try p.credential(p.get(config, "api_key"));
    const secret = try p.credential(p.get(config, "api_secret"));
    const passphrase = try p.credential(p.get(auth, "passphrase"));
    var target = std.mem.splitScalar(u8, path, '?');
    const uri = target.first();
    var bytes: std.ArrayList(u8) = .empty;
    if (target.next()) |query| {
        var pairs: std.ArrayList([]const u8) = .empty;
        var iterator = std.mem.splitScalar(u8, query, '&');
        while (iterator.next()) |part| try pairs.append(a, try (std.Uri.Component{ .percent_encoded = part }).toRawMaybeAlloc(a));
        std.mem.sort([]const u8, pairs.items, {}, less);
        try bytes.appendSlice(a, try std.mem.join(a, "&", pairs.items));
    } else try flatten(a, body, &bytes);
    const b64 = std.base64.standard.Encoder;
    const encoded = try a.alloc(u8, b64.calcSize(secret.len));
    defer std.crypto.secureZero(u8, encoded);
    const message = try std.mem.concat(a, u8, &.{ timestamp, method, uri, bytes.items });
    var mac: [Hmac.mac_length]u8 = undefined;
    Hmac.create(&mac, message, b64.encode(encoded, secret));
    try p.put(into, a, "X-edgeX-Api-Key", w.text(key));
    try p.put(into, a, "X-edgeX-Passphrase", w.text(passphrase));
    try p.put(into, a, "X-edgeX-Timestamp", w.text(timestamp));
    try p.put(into, a, "X-edgeX-Signature", w.text(try std.fmt.allocPrint(a, "{x}", .{&mac})));
}

fn lookup(items: p.V, field: []const u8, id: []const u8) !p.V {
    if (items != .array) return error.InvalidResponse;
    for (items.array.items) |item| if (p.eq(u8, try p.text(item, field), id)) return item;
    return error.InvalidMarket;
}

fn quantity(comptime T: type, value: p.V) !T {
    if (value == .string and std.mem.startsWith(u8, value.string, "0x"))
        return std.fmt.parseInt(T, value.string[2..], 16);
    return w.unsigned(T, value);
}

fn decimalText(a: p.A, units: u256, scale: u8) ![]const u8 {
    const factor = try std.math.powi(u256, 10, scale);
    const base = units / factor;
    const fraction = units % factor;
    if (fraction == 0) return std.fmt.allocPrint(a, "{d}", .{base});
    const digits = try std.fmt.allocPrint(a, "{d}", .{fraction});
    const zeros = try a.alloc(u8, scale - digits.len);
    @memset(zeros, '0');
    return std.fmt.allocPrint(a, "{d}.{s}{s}", .{ base, zeros, std.mem.trimEnd(u8, digits, "0") });
}

pub fn order(a: p.A, config: p.V, input: p.V, timestamp: u64) !p.V {
    if (input != .object) return error.InvalidParameter;
    const auth = p.get(config, "auth");
    const account = try w.unsigned(u64, try w.field(input, "accountId"));
    if (account != try w.unsigned(u64, p.get(auth, "account_id"))) return error.InvalidIdentity;
    const contract_id = try p.text(input, "contractId");
    const metadata = p.get(p.get(p.get(config, "metadata"), "markets"), "data");
    const contract = try lookup(p.get(metadata, "contractList"), "contractId", contract_id);
    const quote_id = try p.text(contract, "quoteCoinId");
    const quote_coin = try lookup(p.get(metadata, "coinList"), "coinId", quote_id);
    const size_string = try p.text(input, "size");
    const order_type = try p.text(input, "type");
    const market = p.eq(u8, order_type, "MARKET");
    if (!market and !p.eq(u8, order_type, "LIMIT")) return error.InvalidParameter;
    const price_string = try p.text(input, if (market) "price_bound" else "price");
    const side = try common.enumIndex(try p.text(input, "side"), &.{ "SELL", "BUY" });
    const scale: u8 = 18;
    const factor: u256 = 1_000_000_000_000_000_000;
    const size = try common.decimal(u256, size_string, scale);
    const price = try common.decimal(u256, price_string, scale);
    const tick = try common.decimal(u256, try p.text(contract, "tickSize"), scale);
    const step = try common.decimal(u256, try p.text(contract, "stepSize"), scale);
    if (size == 0 or price == 0 or tick == 0 or step == 0 or size % step != 0 or price % tick != 0)
        return error.InvalidQuantity;
    const notional = try std.math.mul(u256, size, price);
    const syn_res = try quantity(u256, try w.field(contract, "resolution"));
    const col_res = try quantity(u256, try w.field(quote_coin, "resolution"));
    if (syn_res == 0 or col_res == 0) return error.InvalidResponse;
    const synthetic = try std.math.mul(u256, size, syn_res) / factor;
    const collateral = try std.math.mul(u256, notional, col_res) / (factor * factor);
    const taker_fee = try common.decimal(u256, try p.text(contract, "defaultTakerFeeRate"), scale);
    const maker_text = try p.text(contract, "defaultMakerFeeRate");
    const maker_fee = if (std.mem.startsWith(u8, maker_text, "-")) 0 else try common.decimal(u256, maker_text, scale);
    const max_fee = try std.math.divCeil(u256, try std.math.mul(u256, notional, @max(taker_fee, maker_fee)), factor * factor * factor);
    const fee = try std.math.mul(u256, max_fee, col_res);
    if (synthetic == 0 or collateral == 0 or synthetic > std.math.maxInt(i256) or collateral > std.math.maxInt(i256))
        return error.InvalidQuantity;
    const client_id = try p.text(input, "clientOrderId");
    if (client_id.len == 0) return error.InvalidParameter;
    var digest: [32]u8 = undefined;
    Sha256.hash(client_id, &digest, .{});
    const nonce = std.mem.readInt(u32, digest[0..4], .big);
    const l2_expiry = timestamp + 30 * 24 * 60 * 60 * 1000;
    const expiry = l2_expiry - 8 * 24 * 60 * 60 * 1000;
    var key = try p.evmKey(auth);
    defer std.crypto.secureZero(u8, &key);
    const signer = evm.encodeWord(std.mem.readInt(u160, &try p.evmAddress(key), .big));
    const base_type = "OrderBase(uint256 nonce,address signer,uint64 accountId,uint256 expirationTimestamp)";
    const base = evm.hash(&(evm.hash(base_type) ++ evm.encodeWord(nonce) ++ signer ++ evm.encodeWord(account) ++ evm.encodeWord(expiry / 1000)));
    const order_type_hash = evm.hash("LimitOrderParams(OrderBase base,int256 amountSynthetic,int256 amountCollateral,uint256 amountFee,uint64 assetIdSynthetic,uint64 assetIdCollateral,bool isBuyingSynthetic,bytes32 parentOrderHash,uint256 subOrderIndex)" ++ base_type);
    const hash = evm.hash(&(order_type_hash ++ base ++ evm.encodeWord(synthetic) ++ evm.encodeWord(collateral) ++ evm.encodeWord(fee) ++
        evm.encodeWord(try std.fmt.parseInt(u64, contract_id, 10)) ++ evm.encodeWord(try std.fmt.parseInt(u64, quote_id, 10)) ++
        evm.encodeWord(side) ++ evm.encodeWord(0) ++ evm.encodeWord(0)));
    const global = p.get(metadata, "global");
    const native_chain = p.get(global, "nativeChainId");
    const chain = try quantity(u64, if (native_chain == .null or (native_chain == .string and native_chain.string.len == 0)) p.get(global, "chainId") else native_chain);
    const domain = try evm.address(try p.text(global, "contractAddress"));
    const signature = try @import("../../core/transaction.zig").recoverable(key, p.typedHash("EdgeX", "1", chain, domain, hash));

    const tif = if (p.get(input, "timeInForce") != .null) try p.text(input, "timeInForce") else if (market) "IMMEDIATE_OR_CANCEL" else "GOOD_TIL_CANCEL";
    if (market and !p.eq(u8, tif, "IMMEDIATE_OR_CANCEL") and !p.eq(u8, tif, "FILL_OR_KILL")) return error.InvalidParameter;
    return p.toValue(a, .{
        .accountId = try p.text(input, "accountId"),
        .contractId = contract_id,
        .side = try p.text(input, "side"),
        .price = if (market) "0" else price_string,
        .size = size_string,
        .type = order_type,
        .timeInForce = tif,
        .clientOrderId = client_id,
        .reduceOnly = try common.boolean(input, "reduceOnly"),
        .expireTime = try std.fmt.allocPrint(a, "{d}", .{expiry}),
        .l2Nonce = try std.fmt.allocPrint(a, "{d}", .{nonce}),
        .l2Signature = try std.fmt.allocPrint(a, "{x}", .{&signature}),
        .l2ExpireTime = try std.fmt.allocPrint(a, "{d}", .{l2_expiry}),
        .l2Value = try decimalText(a, notional, 36),
        .l2Size = size_string,
        .l2LimitFee = try std.fmt.allocPrint(a, "{d}", .{max_fee}),
    });
}

pub fn cancel(config: p.V, body: p.V) !void {
    const bound = try w.unsigned(u64, p.get(p.get(config, "auth"), "account_id"));
    if (bound != try w.unsigned(u64, try w.field(body, "accountId"))) return error.InvalidIdentity;
    const ids = try w.field(body, "orderIdList");
    if (ids != .array or ids.array.items.len != 1 or (try w.string(ids.array.items[0])).len == 0) return error.InvalidParameter;
}

test "EdgeX V2 limit order signature matches eth-account" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const config = try p.toValue(a, .{
        .auth = .{ .key = "0x0000000000000000000000000000000000000000000000000000000000000001", .account_id = "123456789012345678" },
        .metadata = .{ .markets = .{ .data = .{
            .global = .{ .nativeChainId = "3343", .contractAddress = "0x4444444444444444444444444444444444444444" },
            .coinList = .{.{ .coinId = "1000", .resolution = "0xf4240" }},
            .contractList = .{.{
                .contractId = "10000001",
                .quoteCoinId = "1000",
                .resolution = "10000000000",
                .tickSize = "0.1",
                .stepSize = "0.001",
                .defaultTakerFeeRate = "0.00045",
                .defaultMakerFeeRate = "0.0004",
            }},
        } } },
    });
    const payload = try order(a, config, try p.toValue(a, .{
        .type = "LIMIT",
        .side = "BUY",
        .contractId = "10000001",
        .accountId = "123456789012345678",
        .price = "97393.8",
        .size = "0.001",
        .clientOrderId = "fixture-1",
    }), 1780000000000);
    try std.testing.expectEqualStrings("ef5a71a13ca376e25950c45f0f8b46dfa75931af99ae4512fc3af88a3b2d89a3723679c47ca21a67268d1dd94a5a35b4d6355bb661c9d2d72b94937a63bd3dbe1c", try p.text(payload, "l2Signature"));
}
