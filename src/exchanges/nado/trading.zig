const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");

pub fn queryType(operation: []const u8) ?[]const u8 {
    if (p.eq(u8, operation, "order_book")) return "market_liquidity";
    if (p.eq(u8, operation, "account") or p.eq(u8, operation, "positions")) return "subaccount_info";
    if (p.eq(u8, operation, "open_orders")) return "subaccount_orders";
    if (p.eq(u8, operation, "contracts")) return "contracts";
    return null;
}

fn signedWord(value: p.V) !p.Word {
    const s = if (value == .number_string) value.number_string else try w.string(value);
    const n = try std.fmt.parseInt(i128, s, 10);
    return evm.encodeWord(@bitCast(@as(i256, n)));
}

fn unsignedWord(comptime T: type, value: p.V) !p.Word {
    return evm.encodeWord(try w.unsigned(T, value));
}

fn arrayHash(comptime T: type, value: p.V) !p.Word {
    if (value != .array or value.array.items.len == 0) return error.InvalidParameter;
    var hash = std.crypto.hash.sha3.Keccak256.init(.{});
    for (value.array.items) |item| {
        const word = if (T == p.Word) try evm.hex(32, try w.string(item)) else try unsignedWord(T, item);
        hash.update(&word);
    }
    return hash.finalResult();
}

fn domain(chain: p.V, address: p.Word, message: p.Word) !p.Word {
    return p.typedHash("Nado", "0.0.1", try w.unsigned(u64, chain), address, message);
}

fn validateSender(auth: p.V, sender: p.V) !p.Word {
    const identifier = try evm.hex(32, try w.string(sender));
    const expected = p.get(auth, "subaccount");
    if (expected != .null and !p.eq(u8, &identifier, &try evm.hex(32, try w.string(expected)))) return error.InvalidIdentity;
    return identifier;
}

pub fn sign(a: p.A, op: []const u8, auth: p.V, contracts: p.V, body: p.V) !p.V {
    if (body != .object or body.object.contains("signature")) return error.InvalidParameter;
    const data = try w.field(contracts, "data");
    const chain = try w.field(data, "chain_id");
    var key = try p.evmKey(auth);
    defer std.crypto.secureZero(u8, &key);
    var signed = p.object();
    var result = p.object();
    const place = p.eq(u8, op, "place_order");
    for (body.object.keys(), body.object.values()) |name, value| try p.put(&signed, a, name, value);
    const digest = if (place) blk: {
        const product = try w.unsigned(u32, try w.field(body, "product_id"));
        const order = try w.field(body, "order");
        const sender = try validateSender(auth, try w.field(order, "sender"));
        const message = evm.hash(&(evm.hash("Order(bytes32 sender,int128 priceX18,int128 amount,uint64 expiration,uint64 nonce,uint128 appendix)") ++
            sender ++ try signedWord(try w.field(order, "priceX18")) ++ try signedWord(try w.field(order, "amount")) ++
            try unsignedWord(u64, try w.field(order, "expiration")) ++ try unsignedWord(u64, try w.field(order, "nonce")) ++
            try unsignedWord(u128, try w.field(order, "appendix"))));
        break :blk try domain(chain, evm.encodeWord(product), message);
    } else blk: {
        const tx = try w.field(body, "tx");
        const sender = try validateSender(auth, try w.field(tx, "sender"));
        const ids = try w.field(tx, "productIds");
        const digests = try w.field(tx, "digests");
        if (ids != .array or digests != .array or ids.array.items.len != digests.array.items.len) return error.InvalidParameter;
        const message = evm.hash(&(evm.hash("Cancellation(bytes32 sender,uint32[] productIds,bytes32[] digests,uint64 nonce)") ++
            sender ++ try arrayHash(u32, ids) ++ try arrayHash(p.Word, digests) ++ try unsignedWord(u64, try w.field(tx, "nonce"))));
        break :blk try domain(chain, try evm.address(try p.text(data, "endpoint_addr")), message);
    };
    try p.put(&signed, a, "signature", w.text(try p.signature(a, key, digest)));
    try p.put(&result, a, if (place) "place_order" else "cancel_orders", signed);
    return result;
}

pub fn authenticate(a: p.A, auth: p.V, sender: p.V, contracts: p.V, now: u64) ![]const u8 {
    const account = try validateSender(auth, sender);
    const expiration = try std.math.add(u64, now, 30_000);
    const chain = try w.field(contracts, "chain_id");
    const endpoint = try evm.address(try p.text(contracts, "endpoint_addr"));
    const message = evm.hash(&(evm.hash("StreamAuthentication(bytes32 sender,uint64 expiration)") ++ account ++ evm.encodeWord(expiration)));
    var key = try p.evmKey(auth);
    defer std.crypto.secureZero(u8, &key);
    return std.json.Stringify.valueAlloc(a, .{
        .method = "authenticate",
        .id = 1,
        .tx = .{ .sender = sender, .expiration = try std.fmt.allocPrint(a, "{d}", .{expiration}) },
        .signature = try p.signature(a, key, try domain(chain, endpoint, message)),
    }, .{});
}
