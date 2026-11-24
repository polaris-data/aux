// EIP-155 legacy and EIP-1559 envelopes for a prepared contract transaction.
const std = @import("std");
const w = @import("wire.zig");
const evm = @import("evm.zig");
const A = w.A;
const V = w.V;
const Bytes = std.ArrayList(u8);
const Curve = std.crypto.ecc.Secp256k1;
const Scalar = Curve.scalar.Scalar;
const Mac = std.crypto.auth.hmac.sha2.HmacSha256;

fn minimal(a: A, n: u256) ![]const u8 {
    var b: [32]u8 = undefined;
    std.mem.writeInt(u256, &b, n, .big);
    return a.dupe(u8, std.mem.trimStart(u8, &b, &.{0}));
}
fn appendRlp(a: A, out: *Bytes, payload: []const u8, list: bool) !void {
    const base: u8 = if (list) 0xc0 else 0x80;
    if (!list and payload.len == 1 and payload[0] < 0x80) return out.append(a, payload[0]);
    if (payload.len <= 55) {
        try out.append(a, base + @as(u8, @intCast(payload.len)));
    } else {
        const len = try minimal(a, payload.len);
        try out.append(a, base + 55 + @as(u8, @intCast(len.len)));
        try out.appendSlice(a, len);
    }
    try out.appendSlice(a, payload);
}
fn integer(a: A, out: *Bytes, n: u256) !void {
    try appendRlp(a, out, try minimal(a, n), false);
}
pub fn quantity(comptime T: type, v: V) !T {
    const s = if (v == .number_string) v.number_string else try w.string(v);
    const hex = std.mem.startsWith(u8, s, "0x");
    const digits = if (hex) s[2..] else s;
    if (digits.len == 0) return error.InvalidQuantity;
    for (digits) |c| if (if (hex) !std.ascii.isHex(c) else !std.ascii.isDigit(c)) return error.InvalidQuantity;
    return std.fmt.parseInt(T, digits, if (hex) 16 else 10);
}
fn member(v: V, k: []const u8) V {
    return v.object.get(k) orelse .null;
}
const Sig = struct { r: u256, s: u256, parity: u1 };
fn signDigest(secret: [32]u8, hash: [32]u8) !Sig {
    // RFC 6979 section 3.2 using HMAC-SHA256 and bits2octets(hash).
    const priv = try Scalar.fromBytes(secret, .big);
    if (priv.isZero()) return error.InvalidKey;
    const zero: [16]u8 = @splat(0);
    const z = Scalar.fromBytes48(zero ++ hash, .big);
    var k: [32]u8 = @splat(0);
    var v: [32]u8 = @splat(1);
    defer std.crypto.secureZero(u8, &k);
    defer std.crypto.secureZero(u8, &v);
    for ([_]u8{ 0, 1 }) |separator| {
        var mac = Mac.init(&k);
        mac.update(&v);
        mac.update(&.{separator});
        mac.update(&secret);
        mac.update(&z.toBytes(.big));
        mac.final(&k);
        Mac.create(&v, &v, &k);
    }
    while (true) {
        Mac.create(&v, &v, &k);
        if (Scalar.fromBytes(v, .big)) |nonce| {
            if (!nonce.isZero()) {
                const p = (try Curve.basePoint.mul(v, .big)).toCompressedSec1();
                const r = std.mem.readInt(u256, p[1..33], .big);
                if (r > 0 and r < Curve.scalar.field_order) {
                    const rs = try Scalar.fromBytes(p[1..33].*, .big);
                    const s = nonce.invert().mul(z.add(rs.mul(priv)));
                    if (!s.isZero()) {
                        const high = std.mem.readInt(u256, &s.toBytes(.big), .big) > Curve.scalar.field_order / 2;
                        return .{ .r = r, .s = std.mem.readInt(u256, &(if (high) s.neg() else s).toBytes(.big), .big), .parity = @as(u1, @truncate(p[0])) ^ @intFromBool(high) };
                    }
                }
            }
        } else |_| {}
        var mac = Mac.init(&k);
        mac.update(&v);
        mac.update(&.{0});
        mac.final(&k);
        Mac.create(&v, &v, &k);
    }
}
pub fn recoverable(secret: [32]u8, digest: [32]u8) ![65]u8 {
    const sig = try signDigest(secret, digest);
    var result: [65]u8 = undefined;
    std.mem.writeInt(u256, result[0..32], sig.r, .big);
    std.mem.writeInt(u256, result[32..64], sig.s, .big);
    result[64] = 27 + @as(u8, sig.parity);
    return result;
}
fn envelope(a: A, fields: []const u8, dynamic: bool) ![]const u8 {
    var out: Bytes = .empty;
    if (dynamic) try out.append(a, 2);
    try appendRlp(a, &out, fields, true);
    return out.items;
}
pub fn sign(a: A, input: V) !V {
    const tx = try w.field(input, "prepared");
    const chain = try quantity(u64, try w.field(tx, "chainId"));
    const gas = try quantity(u64, try w.field(tx, "gasLimit"));
    const nonce = try quantity(u64, try w.field(tx, "nonce"));
    if (chain == 0 or gas == 0 or nonce == std.math.maxInt(u64)) return error.InvalidTransaction;
    const value = if (tx.object.get("value")) |v| try quantity(u256, v) else 0;
    const address = try evm.fromHex(a, try w.string(try w.field(tx, "to")));
    if (address.len != 20) return error.InvalidAddress;
    const data = try evm.fromHex(a, try w.string(try w.field(tx, "data")));
    const price = member(tx, "gasPrice");
    const max = member(tx, "maxFeePerGas");
    const tip = member(tx, "maxPriorityFeePerGas");
    const dynamic = price == .null;
    var f: Bytes = .empty;
    if (dynamic) try integer(a, &f, chain);
    try integer(a, &f, nonce);
    if (dynamic) {
        const priority = try quantity(u128, tip);
        const maximum = try quantity(u128, max);
        if (priority > maximum) return error.InvalidFee;
        try integer(a, &f, priority);
        try integer(a, &f, maximum);
    } else {
        if (max != .null or tip != .null) return error.InvalidFee;
        try integer(a, &f, try quantity(u128, price));
    }
    try integer(a, &f, gas);
    try appendRlp(a, &f, address, false);
    try integer(a, &f, value);
    try appendRlp(a, &f, data, false);
    if (dynamic) try appendRlp(a, &f, "", true);
    const common_len = f.items.len;
    if (!dynamic) {
        try integer(a, &f, chain);
        try integer(a, &f, 0);
        try integer(a, &f, 0);
    }
    const digest = evm.hash(try envelope(a, f.items, dynamic));
    if (input.object.get("digest_only")) |flag| {
        if (flag != .bool or !flag.bool) return error.InvalidTransaction;
        return w.text(try std.fmt.allocPrint(a, "0x{x}", .{&digest}));
    }
    var sender: [20]u8 = undefined;
    const sig: Sig = if (input.object.get("signature")) |proof| blk: {
        const bytes = try evm.hex(65, try w.string(proof));
        const parity: u1 = switch (bytes[64]) {
            0, 27 => 0,
            1, 28 => 1,
            else => return error.InvalidSignature,
        };
        const r = std.mem.readInt(u256, bytes[0..32], .big);
        const s = std.mem.readInt(u256, bytes[32..64], .big);
        sender = try recoverAddress(digest, r, s, parity);
        const expected = try evm.hex(20, try w.string(try w.field(input, "address")));
        if (!std.mem.eql(u8, &sender, &expected)) return error.InvalidSignature;
        break :blk .{ .r = r, .s = s, .parity = parity };
    } else blk: {
        var key = try evm.hex(32, try w.string(try w.field(input, "key")));
        defer std.crypto.secureZero(u8, &key);
        const public = (try Curve.basePoint.mul(key, .big)).toUncompressedSec1();
        sender = evm.hash(public[1..])[12..].*;
        break :blk try signDigest(key, digest);
    };
    f.shrinkRetainingCapacity(common_len);
    try integer(a, &f, if (dynamic) sig.parity else 35 + 2 * @as(u256, chain) + sig.parity);
    try integer(a, &f, sig.r);
    try integer(a, &f, sig.s);
    const bytes = try envelope(a, f.items, dynamic);
    var out: V = .{ .object = .empty };
    try out.object.put(a, "address", w.text(try std.fmt.allocPrint(a, "0x{x}", .{&sender})));
    try out.object.put(a, "signed", w.text(try std.fmt.allocPrint(a, "0x{x}", .{bytes})));
    try out.object.put(a, "hash", w.text(try std.fmt.allocPrint(a, "0x{x}", .{&evm.hash(bytes)})));
    return out;
}

const Element = struct { payload: []const u8, list: bool };
fn takeRlp(input: *[]const u8) !Element {
    const b = input.*;
    if (b.len == 0) return error.InvalidRlp;
    const tag = b[0];
    if (tag < 128) {
        input.* = b[1..];
        return .{ .payload = b[0..1], .list = false };
    }
    const list = tag >= 192;
    const n = tag - @as(u8, if (list) 192 else 128);
    var offset: usize = 1;
    const len: usize = if (n <= 55) n else blk: {
        const width = n - 55;
        if (width > @sizeOf(usize) or b.len <= width or b[1] == 0) return error.InvalidRlp;
        offset += width;
        const length = std.mem.readVarInt(usize, b[1..offset], .big);
        if (length <= 55) return error.InvalidRlp;
        break :blk length;
    };
    if (len > b.len - offset or (!list and len == 1 and b[offset] < 128)) return error.InvalidRlp;
    input.* = b[offset + len ..];
    return .{ .payload = b[offset..][0..len], .list = list };
}
fn rlpInt(comptime T: type, e: Element) !T {
    if (e.list or e.payload.len > @sizeOf(T) or (e.payload.len != 0 and e.payload[0] == 0)) return error.InvalidQuantity;
    return std.mem.readVarInt(T, e.payload, .big);
}
fn accessList(e: Element) !void {
    if (!e.list) return error.InvalidAccessList;
    var data = e.payload;
    while (data.len > 0) {
        const pair = try takeRlp(&data);
        if (!pair.list) return error.InvalidAccessList;
        var fields = pair.payload;
        const address = try takeRlp(&fields);
        const keys = try takeRlp(&fields);
        if (fields.len != 0 or address.list or address.payload.len != 20 or !keys.list) return error.InvalidAccessList;
        var items = keys.payload;
        while (items.len > 0) {
            const key = try takeRlp(&items);
            if (key.list or key.payload.len != 32) return error.InvalidAccessList;
        }
    }
}
/// Validate canonical legacy EIP-155, type-1 or type-2 bytes and recover the signer.
pub fn rawSender(a: A, bytes: []const u8, chain: u64) ![20]u8 {
    if (bytes.len == 0) return error.InvalidTransaction;
    const kind: u8 = if (bytes[0] >= 192) 0 else bytes[0];
    if (kind > 2 or bytes[0] == 0) return error.InvalidTransaction;
    var head = bytes[@intFromBool(kind != 0)..];
    const top = try takeRlp(&head);
    if (head.len != 0 or !top.list) return error.InvalidTransaction;
    var data = top.payload;
    if (kind != 0 and try rlpInt(u64, try takeRlp(&data)) != chain) return error.InvalidChain;
    _ = try rlpInt(u64, try takeRlp(&data)); // nonce
    const fee = try rlpInt(u128, try takeRlp(&data));
    if (kind == 2 and try rlpInt(u128, try takeRlp(&data)) < fee) return error.InvalidFee;
    _ = try rlpInt(u64, try takeRlp(&data)); // gas
    const target = try takeRlp(&data);
    if (target.list or (target.payload.len != 0 and target.payload.len != 20)) return error.InvalidAddress;
    _ = try rlpInt(u256, try takeRlp(&data));
    if ((try takeRlp(&data)).list) return error.InvalidTransaction;
    if (kind != 0) try accessList(try takeRlp(&data));
    var fields: Bytes = .empty;
    try fields.appendSlice(a, top.payload[0 .. top.payload.len - data.len]);
    const v = try rlpInt(u256, try takeRlp(&data));
    const parity: u8 = if (kind == 0) blk: {
        if (v < 35 or (v - 35) / 2 != chain) return error.InvalidChain;
        try integer(a, &fields, chain);
        try integer(a, &fields, 0);
        try integer(a, &fields, 0);
        break :blk @intCast((v - 35) % 2);
    } else if (v <= 1) @intCast(v) else return error.InvalidSignature;
    const r = try rlpInt(u256, try takeRlp(&data));
    const s = try rlpInt(u256, try takeRlp(&data));
    if (data.len != 0) return error.InvalidSignature;
    var signing: Bytes = .empty;
    if (kind != 0) try signing.append(a, kind);
    try appendRlp(a, &signing, fields.items, true);
    return recoverAddress(evm.hash(signing.items), r, s, parity);
}
pub fn recoverAddress(digest: [32]u8, r: u256, s: u256, parity: u8) ![20]u8 {
    if (parity > 1 or r == 0 or s == 0 or r >= Curve.scalar.field_order or s > Curve.scalar.field_order / 2) return error.InvalidSignature;
    const R = try Curve.fromSec1(&([_]u8{2 + parity} ++ evm.encodeWord(r)));
    const inv = (try Scalar.fromBytes(evm.encodeWord(r), .big)).invert();
    const z = Scalar.fromBytes48(@as([16]u8, @splat(0)) ++ digest, .big);
    const sr = (try Scalar.fromBytes(evm.encodeWord(s), .big)).mul(inv);
    const q = try R.mul(sr.toBytes(.big), .big);
    const public = if (z.isZero()) q else q.sub(try Curve.basePoint.mul(z.mul(inv).toBytes(.big), .big));
    try public.rejectIdentity();
    return evm.hash(public.toUncompressedSec1()[1..])[12..].*;
}
