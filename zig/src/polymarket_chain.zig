// Explicit helpers for resolving a market and constructing a wallet's signing data.
const std = @import("std");
const w = @import("wire.zig");
const evm = @import("evm.zig");
const tx = @import("transaction.zig");
const A = w.A;
const V = w.V;
const Op = enum { order_signature, order_typed_data, typed_data_hash, market_asset_id, market_order_domain, deployments, wallet_auth_typed_data, perps_op_typed_data, deposit_wallet_batch_typed_data, wrap_session_signature };
fn string(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn decimal(a: A, v: V) ![]const u8 {
    return std.fmt.allocPrint(a, "{d}", .{try tx.quantity(u256, v)});
}
fn addr(a: A, v: V, k: []const u8) ![]const u8 {
    return std.fmt.allocPrint(a, "0x{x}", .{try evm.hex(20, try string(v, k))});
}
fn list(a: A, v: V, encoded: bool) ![]const V {
    const items = if (v == .string and encoded) try w.parse(a, v.string) else v;
    if (items != .array) return error.InvalidArray;
    for (items.array.items) |val| if (val != .string) return error.ExpectedString;
    return items.array.items;
}
fn version(v: V) !bool {
    const ver = try string(v, "version");
    if (std.mem.eql(u8, ver, "v1")) return false;
    if (std.mem.eql(u8, ver, "v2")) return true;
    return error.InvalidVersion;
}
const domain = [_][2][]const u8{ .{ "name", "string" }, .{ "version", "string" }, .{ "chainId", "uint256" } };
fn types(a: A, rows: []const [2][]const u8) !V {
    var fields: std.array_list.Managed(V) = .init(a);
    for (rows) |r| {
        var f: V = .{ .object = .empty };
        try f.object.put(a, "name", w.text(r[0]));
        try f.object.put(a, "type", w.text(r[1]));
        try fields.append(f);
    }
    return .{ .array = fields };
}
fn encode(a: A, value: anytype) !V {
    return w.parse(a, try std.json.Stringify.valueAlloc(a, value, .{}));
}
pub fn wrap(a: A, account: [32]u8, signature: []const u8) ![]const u8 {
    if (signature.len == 0) return error.InvalidSignature;
    const pad = (32 - signature.len % 32) % 32;
    const zero: [32]u8 = @splat(0);
    const magic: [16][2]u8 = @splat(.{ 0x64, 0x92 });
    return std.mem.concat(a, u8, &.{ &account, &zero, &evm.encodeWord(96), &evm.encodeWord(signature.len), signature, zero[0..pad], std.mem.asBytes(&magic) });
}
pub fn execute(a: A, input: V) !V {
    const op = std.meta.stringToEnum(Op, try string(input, "operation")) orelse return error.InvalidOperation;
    const p = try w.field(input, "params");
    if (p != .object) return error.InvalidParameter;
    switch (op) {
        .typed_data_hash => return w.text(try std.fmt.allocPrint(a, "0x{x}", .{&try @import("eip712.zig").hash(a, try w.field(p, "payload"))})),
        .order_signature => {
            const td = try orderTypedData(a, try w.field(p, "order"), try w.field(p, "domain"));
            const typed = @import("eip712.zig");
            const sig = try evm.hex(65, try string(p, "signature"));
            const parity: u8 = switch (sig[64]) {
                0, 1 => sig[64],
                27, 28 => sig[64] - 27,
                else => return error.InvalidSignature,
            };
            const sender = try tx.recoverAddress(try typed.hash(a, td), std.mem.readInt(u256, sig[0..32], .big), std.mem.readInt(u256, sig[32..64], .big), parity);
            if (!std.mem.eql(u8, &sender, &try evm.hex(20, try string(p, "address")))) return error.InvalidSignature;
            if (!std.mem.eql(u8, try string(td, "primaryType"), "TypedDataSign")) return w.text(try std.fmt.allocPrint(a, "0x{x}", .{&sig}));
            const fields = try w.field(td, "types");
            const separator = try typed.structureHash(a, fields, "EIP712Domain", try w.field(td, "domain"));
            const contents = try typed.structureHash(a, fields, "Order", try w.field(try w.field(td, "message"), "contents"));
            return w.text(try std.fmt.allocPrint(a, "0x{x}", .{try @import("clob_wallet.zig").depositSignature(a, &sig, separator, contents)}));
        },
        .order_typed_data => return orderTypedData(a, try w.field(p, "order"), try w.field(p, "domain")),
        .deployments => return w.parse(a, @embedFile("polymarket-deployments.json")),
        .market_asset_id, .market_order_domain => {
            const market = try w.field(p, "market");
            const v2 = try version(market);
            if (op == .market_order_domain) {
                const negative = try w.field(market, "negRisk");
                if (negative != .bool) return error.InvalidRisk;
                const d = try w.parse(a, @embedFile("polymarket-deployments.json"));
                const exchange = try addr(a, try w.field(d, "contracts"), if (v2) "exchangeV3" else if (negative.bool) "negRiskExchange" else "standardExchange");
                return encode(a, .{ .chain_id = @as(u64, 137), .version = @as(u8, if (v2) 3 else 2), .exchange = exchange });
            }
            const outcome = try string(p, "outcome");
            const labels = try list(a, try w.field(market, "outcomes"), true);
            const ids = try list(a, try w.field(market, if (v2) "positionIds" else "clobTokenIds"), !v2);
            if (labels.len != ids.len) return error.MismatchedOutcomes;
            for (labels, ids) |label, id| if (std.mem.eql(u8, label.string, outcome)) {
                _ = try w.unsigned(u256, id);
                return id;
            };
            return error.UnknownOutcome;
        },
        .wrap_session_signature => {
            const wrapped = try wrap(a, try evm.address(try string(p, "session_signer")), try evm.fromHex(a, try string(p, "signature")));
            return w.text(try std.fmt.allocPrint(a, "0x{x}", .{wrapped}));
        },
        else => {},
    }
    const chain = try tx.quantity(u64, try w.field(p, "chain_id"));
    if (op == .wallet_auth_typed_data) {
        const message = .{ .address = try addr(a, p, "address"), .timestamp = try std.fmt.allocPrint(a, "{d}", .{try tx.quantity(u64, try w.field(p, "timestamp"))}), .nonce = try tx.quantity(u64, p.object.get("nonce") orelse w.text("0")), .message = "This message attests that I control the given wallet" };
        return encode(a, .{ .domain = .{ .name = "ClobAuthDomain", .version = "1", .chainId = chain }, .primaryType = "ClobAuth", .types = .{ .EIP712Domain = try types(a, &domain), .ClobAuth = try types(a, &.{ .{ "address", "address" }, .{ "timestamp", "string" }, .{ "nonce", "uint256" }, .{ "message", "string" } }) }, .message = message });
    }
    if (op == .perps_op_typed_data) {
        const hash = try @import("perps_sign.zig").operationHash(a, try w.field(p, "operation"));
        return encode(a, .{ .domain = .{ .name = "Polymarket", .version = "1", .chainId = chain }, .primaryType = "Op", .types = .{ .EIP712Domain = try types(a, &domain), .Op = try types(a, &.{ .{ "data", "bytes32" }, .{ "salt", "uint64" }, .{ "ts", "uint64" } }) }, .message = .{ .data = try std.fmt.allocPrint(a, "0x{x}", .{&hash}), .salt = try tx.quantity(u64, try w.field(p, "salt")), .ts = try tx.quantity(u64, try w.field(p, "timestamp")) } });
    }
    const wallet = try addr(a, p, "wallet");
    const calls = try w.field(p, "calls");
    if (chain == 0 or calls != .array or calls.array.items.len == 0) return error.InvalidWalletBatch;
    var batch: std.array_list.Managed(V) = .init(a);
    for (calls.array.items) |call| {
        try batch.append(try encode(a, .{ .target = try addr(a, call, "to"), .value = try decimal(a, try w.field(call, "value")), .data = try std.fmt.allocPrint(a, "0x{x}", .{try evm.fromHex(a, try string(call, "data"))}) }));
    }
    return encode(a, .{ .domain = .{ .name = "DepositWallet", .version = "1", .chainId = chain, .verifyingContract = wallet }, .primaryType = "Batch", .types = .{ .EIP712Domain = try types(a, &(domain ++ .{.{ "verifyingContract", "address" }})), .Call = try types(a, &.{ .{ "target", "address" }, .{ "value", "uint256" }, .{ "data", "bytes" } }), .Batch = try types(a, &.{ .{ "wallet", "address" }, .{ "nonce", "uint256" }, .{ "deadline", "uint256" }, .{ "calls", "Call[]" } }) }, .message = .{ .wallet = wallet, .nonce = try decimal(a, try w.field(p, "nonce")), .deadline = try decimal(a, try w.field(p, "deadline")), .calls = V{ .array = batch } } });
}

fn orderTypedData(a: A, order: V, d: V) !V {
    const version_id = try tx.quantity(u8, try w.field(d, "version"));
    const chain = try tx.quantity(u64, try w.field(d, "chain_id"));
    if ((version_id != 2 and version_id != 3) or chain == 0) return error.InvalidDomain;
    const zero = "0x0000000000000000000000000000000000000000000000000000000000000000";
    var message = V{ .object = .empty };
    const field_types = [_][2][]const u8{ .{ "salt", "uint256" }, .{ "maker", "address" }, .{ "signer", "address" }, .{ "tokenId", "uint256" }, .{ "makerAmount", "uint256" }, .{ "takerAmount", "uint256" }, .{ "side", "uint8" }, .{ "signatureType", "uint8" }, .{ "timestamp", "uint256" }, .{ "metadata", "bytes32" }, .{ "builder", "bytes32" } };
    for (field_types) |f| {
        const defaulted = std.mem.eql(u8, f[0], "builder") or std.mem.eql(u8, f[0], "metadata");
        var v = if (defaulted) (if (order == .object) order.object.get(f[0]) orelse w.text(zero) else return error.InvalidOrder) else try w.field(order, f[0]);
        if (defaulted and v == .string and v.string.len == 0) v = w.text(zero);
        try message.object.put(a, f[0], v);
    }
    const side = try w.field(order, "side");
    const n: u8 = if (side == .string) blk: {
        if (std.mem.eql(u8, side.string, "BUY")) break :blk 0;
        if (std.mem.eql(u8, side.string, "SELL")) break :blk 1;
        return error.InvalidSide;
    } else try tx.quantity(u8, side);
    if (n > 1) return error.InvalidSide;
    try message.object.put(a, "side", .{ .integer = n });
    const mode = try tx.quantity(u8, try w.field(order, "signatureType"));
    if (mode > 3) return error.InvalidSignatureType;
    var td = try encode(a, .{ .domain = .{ .name = "Polymarket CTF Exchange", .version = try std.fmt.allocPrint(a, "{d}", .{version_id}), .chainId = chain, .verifyingContract = try addr(a, d, "exchange") }, .primaryType = "Order", .types = .{ .EIP712Domain = try types(a, &(domain ++ .{.{ "verifyingContract", "address" }})), .Order = try types(a, &field_types) }, .message = message });
    if (mode == 3) {
        try td.object.put(a, "primaryType", w.text("TypedDataSign"));
        const all = td.object.getPtr("types").?;
        try all.object.put(a, "TypedDataSign", try types(a, &.{ .{ "contents", "Order" }, .{ "name", "string" }, .{ "version", "string" }, .{ "chainId", "uint256" }, .{ "verifyingContract", "address" }, .{ "salt", "bytes32" } }));
        try td.object.put(a, "message", try encode(a, .{ .contents = message, .name = "DepositWallet", .version = "1", .chainId = chain, .verifyingContract = try w.field(order, "signer"), .salt = zero }));
    }
    _ = try @import("eip712.zig").hash(a, td);
    return td;
}
