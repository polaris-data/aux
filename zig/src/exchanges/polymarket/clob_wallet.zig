const std = @import("std");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const clob = @import("clob_auth.zig");
const recoverable = @import("../../core/transaction.zig").recoverable;
const uint = w.unsigned;
const h = evm.hash;
const word = evm.encodeWord;
const Word = [32]u8;
const order_type = "Order(uint256 salt,address maker,address signer,uint256 tokenId,uint256 makerAmount,uint256 takerAmount,uint8 side,uint8 signatureType,uint256 timestamp,bytes32 metadata,bytes32 builder)";
const zero: Word = @splat(0);

pub fn isLogin(op: []const u8, has_api: bool) bool {
    return std.mem.eql(u8, op, "clob.clob_create_api_key") or std.mem.eql(u8, op, "clob.clob_derive_api_key") or (!has_api and std.mem.eql(u8, op, "clob.clob_get_api_keys"));
}
pub fn isOrder(op: []const u8) bool {
    return std.mem.eql(u8, op, "clob.clob_post_order") or std.mem.eql(u8, op, "clob.clob_post_orders");
}

fn keyFrom(wallet: w.V) ![32]u8 {
    if (wallet != .object) return error.AuthenticationRequired;
    return evm.hex(32, try w.string(wallet.object.get("key") orelse return error.AuthenticationRequired));
}
fn signer(key: [32]u8) !Word {
    const public = (try std.crypto.ecc.Secp256k1.basePoint.mul(key, .big)).toUncompressedSec1();
    return @as([12]u8, @splat(0)) ++ h(public[1..])[12..].*;
}
fn addr(v: w.V, name: []const u8) !Word {
    return evm.address(try w.string(try w.field(v, name)));
}
fn typed(domain: Word, contents: Word) Word {
    return h(&([_]u8{ 0x19, 0x01 } ++ domain ++ contents));
}
pub fn login(a: w.A, headers: *w.V, wallet: w.V, seconds: u64) !void {
    var key = try keyFrom(wallet);
    defer std.crypto.secureZero(u8, &key);
    const account = try signer(key);
    const nonce = if (wallet.object.get("nonce")) |v| try uint(u64, v) else 0;
    const timestamp = try std.fmt.allocPrint(a, "{d}", .{seconds});
    const domain = h(&(h("EIP712Domain(string name,string version,uint256 chainId)") ++ h("ClobAuthDomain") ++ h("1") ++ word(137)));
    const contents = h(&(h("ClobAuth(address address,string timestamp,uint256 nonce,string message)") ++ account ++ h(timestamp) ++ word(nonce) ++ h("This message attests that I control the given wallet")));
    const sig = try recoverable(key, typed(domain, contents));
    try clob.removeAuthHeaders(a, headers);
    try headers.object.put(a, "POLY_SIGNATURE", w.text(try std.fmt.allocPrint(a, "0x{x}", .{&sig})));
    try headers.object.put(a, "POLY_ADDRESS", w.text(try clob.checksum(a, try std.fmt.allocPrint(a, "0x{x}", .{account[12..]}))));
    try headers.object.put(a, "POLY_TIMESTAMP", w.text(timestamp));
    try headers.object.put(a, "POLY_NONCE", w.text(try std.fmt.allocPrint(a, "{d}", .{nonce})));
}

fn quantity(order: w.V, name: []const u8) !Word {
    const v = try w.field(order, name);
    if (v == .string and std.mem.startsWith(u8, v.string, "0x")) {
        for (v.string[2..]) |b| if (!std.ascii.isHex(b)) return error.InvalidQuantity;
        return word(try std.fmt.parseInt(u256, v.string[2..], 16));
    }
    return word(try uint(u256, v));
}
fn bytes32(order: w.V, name: []const u8) !Word {
    const v = order.object.get(name) orelse return zero;
    const s = try w.string(v);
    return if (s.len == 0) zero else try evm.hex(32, s);
}

fn signRow(a: w.A, row: *w.V, wallet: w.V, api_address: Word) !void {
    if (row.* != .object) return error.ExpectedObject;
    const order = row.object.getPtr("order") orelse return error.MissingField;
    if (order.* != .object) return error.ExpectedObject;
    if (order.object.get("signature")) |existing| if (existing == .string and existing.string.len > 0) return;
    var key = try keyFrom(wallet);
    defer std.crypto.secureZero(u8, &key);
    const account = try signer(key);
    const funding = if (wallet.object.contains("address")) try addr(wallet, "address") else account;
    const mode = try uint(u8, try w.field(order.*, "signatureType"));
    if (mode > 3) return error.InvalidSignatureType;
    const maker = try addr(order.*, "maker");
    const declared = try addr(order.*, "signer");
    if (!std.mem.eql(u8, &api_address, &account) or !std.mem.eql(u8, &funding, &maker) or !std.mem.eql(u8, &declared, if (mode == 3) &funding else &account)) return error.AuthenticationRequired;
    const d = try w.field(wallet, "order_domain");
    const version = try uint(u8, try w.field(d, "version"));
    const chain = try uint(u64, try w.field(d, "chain_id"));
    if ((version != 2 and version != 3) or chain == 0) return error.InvalidDomain;
    const domain = h(&(h("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)") ++ h("Polymarket CTF Exchange") ++ (if (version == 2) h("2") else h("3")) ++ word(chain) ++ try addr(d, "exchange")));
    const side_val = try w.field(order.*, "side");
    const side: u8 = if (side_val == .string) blk: {
        if (std.mem.eql(u8, side_val.string, "BUY")) break :blk 0;
        if (std.mem.eql(u8, side_val.string, "SELL")) break :blk 1;
        return error.InvalidSide;
    } else try uint(u8, side_val);
    if (side > 1) return error.InvalidSide;
    const fields = [_]Word{
        h(order_type),                    try quantity(order.*, "salt"),        maker,                                declared,
        try quantity(order.*, "tokenId"), try quantity(order.*, "makerAmount"), try quantity(order.*, "takerAmount"), word(side),
        word(mode),                       try quantity(order.*, "timestamp"),   try bytes32(order.*, "metadata"),     try bytes32(order.*, "builder"),
    };
    const contents = h(std.mem.asBytes(&fields));
    const message = if (mode == 3) h(&(h("TypedDataSign(Order contents,string name,string version,uint256 chainId,address verifyingContract,bytes32 salt)" ++ order_type) ++ contents ++ h("DepositWallet") ++ h("1") ++ word(chain) ++ declared ++ zero)) else contents;
    const sig = try recoverable(key, typed(domain, message));
    var data: []const u8 = &sig;
    if (mode == 3) data = try depositSignature(a, &sig, domain, contents);
    if (wallet.object.get("session_signer")) |flag| {
        if (flag != .bool) return error.InvalidFlag;
        if (flag.bool) {
            if (mode != 3) return error.InvalidSignatureType;
            data = try @import("chain.zig").wrap(a, account, data);
        }
    }
    try order.object.put(a, "signature", w.text(try std.fmt.allocPrint(a, "0x{x}", .{data})));
}

pub fn orders(a: w.A, body: *w.V, wallet: w.V, api: w.V, batch: bool) !void {
    const address = try addr(api, "address");
    if (batch) {
        if (body.* != .array or body.array.items.len == 0) return error.ExpectedArray;
        for (body.array.items) |*row| try signRow(a, row, wallet, address);
    } else try signRow(a, body, wallet, address);
}

test "CLOB L1 proof matches the wallet reference" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const wallet = try w.parse(a, "{\"key\":\"" ++ "0707070707070707070707070707070707070707070707070707070707070707" ++ "\"}");
    var headers: w.V = .{ .object = .empty };
    try login(a, &headers, wallet, 1731536000);
    try std.testing.expectEqualStrings("0xbba33a080eb7ccca5f2b7c453046018d1cdeabc0f66af56d7cfdebc94727e6401bcc5ce27ea2f62ec9673a03ecf2890b3f25365c6fc4f50a919c26ebde42fecb1c", try w.string(try w.field(headers, "POLY_SIGNATURE")));
}

pub fn depositSignature(a: w.A, signature: []const u8, domain: Word, contents: Word) ![]const u8 {
    return std.mem.concat(a, u8, &.{ signature, &domain, &contents, order_type, &std.mem.toBytes(std.mem.nativeToBig(u16, order_type.len)) });
}
