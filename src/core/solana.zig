// Pascal collateral transfers use the SPL Token program's legacy TransferChecked instruction.
const std = @import("std");
const w = @import("wire.zig");
const A = w.A;
const V = w.V;
const alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
pub const token_program = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ata_program = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

pub fn publicKey(s: []const u8) ![32]u8 {
    return decode58(32, s);
}

pub fn decode58(comptime n: usize, s: []const u8) ![n]u8 {
    if (s.len < n or s.len > (n * 138 / 100 + 1)) return error.InvalidPublicKey;
    var out: [n]u8 = @splat(0);
    for (s) |c| {
        var carry: u16 = @intCast(std.mem.findScalar(u8, alphabet, c) orelse return error.InvalidPublicKey);
        var i = out.len;
        while (i > 0) {
            i -= 1;
            carry += @as(u16, out[i]) * 58;
            out[i] = @truncate(carry);
            carry >>= 8;
        }
        if (carry != 0) return error.InvalidPublicKey;
    }
    const zeroes = out.len - std.mem.trimStart(u8, &out, "\x00").len;
    if (s.len - std.mem.trimStart(u8, s, "1").len != zeroes) return error.InvalidPublicKey;
    return out;
}

pub fn base58(a: A, comptime n: usize, value: [n]u8) ![]const u8 {
    var digits: [2 * n]u8 = undefined;
    var offset = digits.len;
    var bytes = value;
    const zeroes = bytes.len - std.mem.trimStart(u8, &bytes, "\x00").len;
    var start = zeroes;
    while (start < n) {
        var carry: u16 = 0;
        for (bytes[start..]) |*b| {
            carry = carry * 256 + b.*;
            b.* = @intCast(carry / 58);
            carry %= 58;
        }
        offset -= 1;
        digits[offset] = alphabet[carry];
        while (start < n and bytes[start] == 0) start += 1;
    }
    offset -= zeroes;
    @memset(digits[offset..][0..zeroes], '1');
    return a.dupe(u8, digits[offset..]);
}

fn key(v: V, name: []const u8) ![32]u8 {
    return publicKey(try w.string(try w.field(v, name)));
}

pub fn associated(a: A, v: V) !V {
    const owner = try key(v, "owner");
    const mint = try key(v, "mint");
    const token = try publicKey(token_program);
    return w.text(try programAddress(a, ata_program, &.{ &owner, &token, &mint }));
}

pub fn programAddress(a: A, program_id: []const u8, seeds: []const []const u8) ![]const u8 {
    const program = try publicKey(program_id);
    if (seeds.len >= 16) return error.InvalidSeeds;
    var prefix = std.crypto.hash.sha2.Sha256.init(.{});
    for (seeds) |seed| {
        if (seed.len > 32) return error.InvalidSeeds;
        prefix.update(seed);
    }
    var bump: usize = 256;
    while (bump > 0) {
        bump -= 1;
        var hash = prefix;
        hash.update(&.{@intCast(bump)});
        hash.update(&program);
        hash.update("ProgramDerivedAddress");
        const digest = hash.finalResult();
        _ = std.crypto.ecc.Edwards25519.fromBytes(digest) catch return base58(a, 32, digest);
    }
    return error.InvalidProgramAddress;
}

const Message = [214]u8;
fn transfer(v: V) !Message {
    const accounts = [_][32]u8{ try key(v, "owner"), try key(v, "source"), try key(v, "destination"), try key(v, "mint"), try publicKey(token_program) };
    for (accounts, 0..) |account, i| for (accounts[0..i]) |other| if (std.mem.eql(u8, &account, &other)) return error.DuplicateAccount;
    const amount = try w.unsigned(u64, try w.field(v, "amount"));
    const decimals = try w.unsigned(u8, try w.field(v, "decimals"));
    _ = try w.unsigned(u64, try w.field(v, "last_valid_block_height"));
    if (amount == 0) return error.InvalidAmount;
    const blockhash = try key(v, "recent_blockhash");
    var amount_le: [8]u8 = undefined;
    std.mem.writeInt(u64, &amount_le, amount, .little);
    // One signature. The fee payer, source and destination are writable.
    // Instruction accounts are source, mint, destination and signing authority.
    return [_]u8{ 1, 0, 2, 5 } ++ std.mem.asBytes(&accounts).* ++ blockhash ++ [_]u8{ 1, 4, 4, 1, 3, 2, 0, 10, 12 } ++ amount_le ++ [_]u8{decimals};
}

pub fn prepare(a: A, v: V) !V {
    const message = try transfer(v);
    var out: V = .{ .object = .empty };
    for ([_][]const u8{ "owner", "source", "destination", "mint", "recent_blockhash" }) |name| try out.object.put(a, name, try w.field(v, name));
    for ([_][]const u8{ "amount", "decimals", "last_valid_block_height" }) |name| {
        const n = try w.unsigned(u64, try w.field(v, name));
        try out.object.put(a, name, .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{n}) });
    }
    var bytes = std.array_list.Managed(V).init(a);
    for (message) |b| try bytes.append(.{ .integer = b });
    try out.object.put(a, "message", .{ .array = bytes });
    return out;
}
pub fn sign(a: A, v: V) !V {
    const prepared = try w.field(v, "prepared");
    const canonical = try transfer(prepared);
    const message = try w.field(prepared, "message");
    if (message != .array or message.array.items.len != canonical.len) return error.InvalidMessage;
    for (message.array.items, canonical) |b, expected| {
        if (b != .number_string or try w.unsigned(u8, b) != expected) return error.InvalidMessage;
    }
    const owner = try key(prepared, "owner");
    const ed = std.crypto.sign.Ed25519;
    const sig = if (v.object.get("public_key")) |signer| blk: {
        if (!std.mem.eql(u8, &owner, &try publicKey(try w.string(signer)))) return error.InvalidSigner;
        const proof = v.object.get("signature") orelse return w.text(try std.fmt.allocPrint(a, "{x}", .{&canonical}));
        const signature = try decode58(64, try w.string(proof));
        try ed.Signature.fromBytes(signature).verify(&canonical, try ed.PublicKey.fromBytes(owner));
        break :blk signature;
    } else blk: {
        var seed = try @import("evm.zig").hex(32, try w.string(try w.field(v, "seed")));
        defer std.crypto.secureZero(u8, &seed);
        var kp = try ed.KeyPair.generateDeterministic(seed);
        defer std.crypto.secureZero(u8, std.mem.asBytes(&kp.secret_key));
        if (!std.mem.eql(u8, &kp.public_key.toBytes(), &owner)) return error.InvalidSigner;
        break :blk (try kp.sign(&canonical, null)).toBytes();
    };
    const tx = [_]u8{1} ++ sig ++ canonical;
    const enc = std.base64.standard.Encoder;
    var out: V = .{ .object = .empty };
    try out.object.put(a, "signature", w.text(try base58(a, 64, sig)));
    try out.object.put(a, "transaction_base64", w.text(enc.encode(try a.alloc(u8, enc.calcSize(tx.len)), &tx)));
    const height = try w.unsigned(u64, try w.field(prepared, "last_valid_block_height"));
    try out.object.put(a, "last_valid_block_height", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{height}) });
    return out;
}
