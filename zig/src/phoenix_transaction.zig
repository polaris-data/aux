// Canonical Solana legacy/v0 message compilation and multi-signer assembly.
const std = @import("std");
const w = @import("wire.zig");
const sol = @import("solana.zig");
const A = w.A;
const V = w.V;
const Key = [32]u8;
const Bytes = std.ArrayList(u8);
const Ed = std.crypto.sign.Ed25519;
const packet = 1232;
pub const Op = enum { build_instruction, prepare_transaction, assemble_transaction, validate_server_transaction, validate_signed_transaction, sign_transaction };
fn fieldString(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn key(v: V, k: []const u8) !Key {
    return sol.publicKey(try fieldString(v, k));
}
fn array(v: V, k: []const u8) ![]const V {
    const x = try w.field(v, k);
    return if (x == .array) x.array.items else error.InvalidArray;
}
fn flag(v: V, k: []const u8) !bool {
    const b = try w.field(v, k);
    return if (b == .bool) b.bool else error.InvalidBool;
}
fn b64(a: A, b: []const u8) ![]const u8 {
    const enc = std.base64.standard.Encoder;
    return enc.encode(try a.alloc(u8, enc.calcSize(b.len)), b);
}
fn unbase64(a: A, s: []const u8) ![]u8 {
    const dec = std.base64.standard.Decoder;
    if (s.len > 1644) return error.InvalidPacket;
    const out = try a.alloc(u8, try dec.calcSizeForSlice(s));
    if (out.len > packet) return error.InvalidPacket;
    try dec.decode(out, s);
    if (!std.mem.eql(u8, s, try b64(a, out))) return error.InvalidEncoding;
    return out;
}
fn short(a: A, b: *Bytes, n: usize) !void {
    var value = std.math.cast(u16, n) orelse return error.InvalidSize;
    while (value >= 128) {
        try b.append(a, @as(u8, @truncate(value)) | 128);
        value >>= 7;
    }
    try b.append(a, @intCast(value));
}
fn vector(a: A, b: *Bytes, data: []const u8) !void {
    try short(a, b, data.len);
    try b.appendSlice(a, data);
}
const Meta = struct { address: Key, signer: bool = false, writable: bool = false, invoked: bool = false, nonce: bool = false, loaded: bool = false };
const Map = std.AutoHashMapUnmanaged(Key, Meta);
fn entry(a: A, map: *Map, address: Key) !*Meta {
    const kv = try map.getOrPut(a, address);
    if (!kv.found_existing) {
        if (map.count() > 256) return error.TooManyAccounts;
        kv.value_ptr.* = .{ .address = address };
    }
    return kv.value_ptr;
}
fn less(_: void, x: Key, y: Key) bool {
    return std.mem.lessThan(u8, &x, &y);
}
const Lookup = struct { key: Key, write: Bytes = .empty, read: Bytes = .empty };
fn prepare(a: A, p: V) !V {
    const payer = try key(p, "fee_payer");
    const block = try key(p, "recent_blockhash");
    const height = try w.unsigned(u64, try w.field(p, "last_valid_block_height"));
    const instructions = try array(p, "instructions");
    if (instructions.len == 0 or instructions.len > packet / 3) return error.InvalidInstruction;
    const v = p.object.get("version") orelse w.text("legacy");
    const v0 = std.mem.eql(u8, try w.string(v), "v0");
    if (!v0 and !std.mem.eql(u8, v.string, "legacy")) return error.InvalidVersion;
    var map: Map = .empty;
    const fee = try entry(a, &map, payer);
    fee.signer = true;
    fee.writable = true;
    for (instructions, 0..) |ix, i| {
        const program = try key(ix, "program_id");
        (try entry(a, &map, program)).invoked = true;
        const data = try array(ix, "data");
        const accounts = try array(ix, "accounts");
        if (data.len > packet or accounts.len > packet) return error.InvalidInstruction;
        var nonce = i == 0 and std.mem.allEqual(u8, &program, 0) and data.len >= 4;
        if (nonce) for (data[0..4], [_]u8{ 4, 0, 0, 0 }) |b, k| {
            if (try w.unsigned(u8, b) != k) nonce = false;
        };
        for (accounts, 0..) |row, j| {
            const meta = try entry(a, &map, try key(row, "pubkey"));
            const signer = try flag(row, "is_signer");
            const writable = try flag(row, "is_writable");
            meta.signer = meta.signer or signer;
            meta.writable = meta.writable or writable;
            meta.nonce = meta.nonce or (nonce and j == 0);
        }
    }
    const keys = try a.alloc(Key, map.count());
    var it = map.keyIterator();
    for (keys) |*k| k.* = it.next().?.*;
    std.mem.sort(Key, keys, {}, less);
    var loaded: [2]std.ArrayList(Key) = @splat(.empty);
    var lookups: std.ArrayList(Lookup) = .empty;
    if (v0 and p.object.contains("lookup_tables")) for (try array(p, "lookup_tables")) |row| {
        var table: Lookup = .{ .key = try key(row, "key") };
        var indexes: std.AutoHashMapUnmanaged(Key, usize) = .empty;
        for (try array(row, "addresses"), 0..) |s, i| {
            const item = try indexes.getOrPut(a, try sol.publicKey(try w.string(s)));
            if (!item.found_existing) item.value_ptr.* = i;
        }
        for (keys) |k| {
            const m = map.getPtr(k).?;
            if (m.loaded or m.invoked or m.nonce or m.signer) continue;
            const idx = indexes.get(k) orelse continue;
            const octet = std.math.cast(u8, idx) orelse return error.InvalidLookup;
            try (if (m.writable) &table.write else &table.read).append(a, octet);
            try loaded[@intFromBool(!m.writable)].append(a, k);
            m.loaded = true;
        }
        if (table.write.items.len + table.read.items.len > 0) try lookups.append(a, table);
    };
    var sigs: usize = 0;
    var ro_signs: usize = 0;
    var ro_keys: usize = 0;
    var static: std.ArrayList(Key) = .empty;
    try static.append(a, payer);
    for (0..4) |group| for (keys) |k| {
        const meta = map.get(k).?;
        if (meta.loaded or std.mem.eql(u8, &k, &payer)) continue;
        const g: usize = @as(usize, @intFromBool(!meta.signer)) * 2 + @intFromBool(!meta.writable);
        if (g == group) {
            try static.append(a, k);
            if (meta.signer) sigs += 1;
            if (g == 1) ro_signs += 1;
            if (g == 3) ro_keys += 1;
        }
    };
    sigs += 1;
    if (sigs > 18) return error.InvalidPacket;
    var index: std.AutoHashMapUnmanaged(Key, u8) = .empty;
    var at: usize = 0;
    for ([_][]const Key{ static.items, loaded[0].items, loaded[1].items }) |group| for (group) |k| {
        try index.put(a, k, @intCast(at));
        at += 1;
    };
    var message: Bytes = .empty;
    if (v0) try message.append(a, 128);
    try message.appendSlice(a, &.{ @intCast(sigs), @intCast(ro_signs), @intCast(ro_keys) });
    try short(a, &message, static.items.len);
    try message.appendSlice(a, std.mem.sliceAsBytes(static.items));
    try message.appendSlice(a, &block);
    try short(a, &message, instructions.len);
    for (instructions) |ix| {
        const program = index.get(try key(ix, "program_id")).?;
        if (program == 0) return error.InvalidProgram;
        try message.append(a, program);
        const metas = try array(ix, "accounts");
        try short(a, &message, metas.len);
        for (metas) |row| try message.append(a, index.get(try key(row, "pubkey")).?);
        const data = try array(ix, "data");
        try short(a, &message, data.len);
        for (data) |b| try message.append(a, try w.unsigned(u8, b));
        if (1 + sigs * 64 + message.items.len > packet) return error.InvalidPacket;
    }
    if (v0) {
        try short(a, &message, lookups.items.len);
        for (lookups.items) |table| {
            try message.appendSlice(a, &table.key);
            try vector(a, &message, table.write.items);
            try vector(a, &message, table.read.items);
        }
    }
    if (1 + sigs * 64 + message.items.len > packet) return error.InvalidPacket;
    var names: std.array_list.Managed(V) = .init(a);
    for (static.items[0..sigs]) |k| try names.append(w.text(try sol.base58(a, 32, k)));
    return w.parse(a, try std.json.Stringify.valueAlloc(a, .{ .required_signers = V{ .array = names }, .message_base64 = try b64(a, message.items), .recent_blockhash = try sol.base58(a, 32, block), .last_valid_block_height = height }, .{}));
}
const Cursor = struct {
    bytes: []const u8,
    fn take(self: *Cursor, n: usize) ![]const u8 {
        if (n > self.bytes.len) return error.InvalidPacket;
        const b = self.bytes[0..n];
        self.bytes = self.bytes[n..];
        return b;
    }
    fn octet(self: *Cursor) !u8 {
        return (try self.take(1))[0];
    }
    fn length(self: *Cursor) !usize {
        var n: usize = 0;
        for (0..3) |i| {
            const b = try self.octet();
            if ((i != 0 and b == 0) or (i == 2 and b > 3)) return error.InvalidLength;
            n |= @as(usize, b & 127) << @intCast(i * 7);
            if (b < 128) return n;
        }
        return error.InvalidLength;
    }
    fn vec(self: *Cursor) ![]const u8 {
        return self.take(try self.length());
    }
};
const Message = struct {
    bytes: []const u8,
    signers: []const Key,
    block: Key,
    fn parse(bytes: []const u8) !Message {
        var r = Cursor{ .bytes = bytes };
        const first = try r.octet();
        const v0 = first == 128;
        const count = if (v0) try r.octet() else first;
        const readonly = try r.octet();
        const ro_non = try r.octet();
        const keys = try r.length();
        if (count == 0 or count > 18 or readonly >= count or 1 + @as(usize, count) * 64 + bytes.len > packet or keys > 256 or @as(usize, count) + ro_non > keys) return error.InvalidHeader;
        const addresses = std.mem.bytesAsSlice(Key, try r.take(keys * 32));
        const block = (try r.take(32))[0..32].*;
        const ixs = try r.length();
        var max_index: usize = 0;
        for (0..ixs) |_| {
            const program = try r.octet();
            if (program == 0 or program >= keys) return error.InvalidProgram;
            for (try r.vec()) |idx| max_index = @max(max_index, idx);
            _ = try r.vec();
        }
        var total = keys;
        if (v0) {
            const lookups = try r.length();
            for (0..lookups) |_| {
                _ = try r.take(32);
                const write = try r.vec();
                const read = try r.vec();
                if (write.len + read.len == 0) return error.InvalidLookup;
                total += write.len + read.len;
            }
        }
        if (total > 256 or max_index >= total or r.bytes.len != 0) return error.InvalidMessage;
        return .{ .bytes = bytes, .signers = addresses[0..count], .block = block };
    }
    fn reviewed(a: A, v: V) !Message {
        const self = try parse(try unbase64(a, try fieldString(v, "message_base64")));
        const signers = try array(v, "required_signers");
        if (self.signers.len != signers.len or !std.mem.eql(u8, &self.block, &try key(v, "recent_blockhash"))) return error.InvalidMetadata;
        for (self.signers, signers) |k, n| if (!std.mem.eql(u8, &k, &try sol.publicKey(try w.string(n)))) return error.InvalidMetadata;
        _ = try w.unsigned(u64, try w.field(v, "last_valid_block_height"));
        return self;
    }
};
fn verify(k: Key, sig: [64]u8, msg: []const u8) !void {
    try Ed.Signature.fromBytes(sig).verify(msg, try Ed.PublicKey.fromBytes(k));
}
pub fn validate(a: A, signed: V) !void {
    const bytes = try unbase64(a, try fieldString(signed, "transaction_base64"));
    if (bytes.len == 0 or bytes[0] == 0 or bytes[0] > 18) return error.InvalidSignature;
    const end = 1 + @as(usize, bytes[0]) * 64;
    if (end >= bytes.len) return error.InvalidPacket;
    const m = try Message.parse(bytes[end..]);
    if (m.signers.len != bytes[0]) return error.InvalidSignature;
    for (m.signers, 0..) |k, i| try verify(k, bytes[1 + 64 * i ..][0..64].*, m.bytes);
    if (!std.mem.eql(u8, try fieldString(signed, "signature"), try sol.base58(a, 64, bytes[1..65].*))) return error.InvalidSignature;
    _ = try w.unsigned(u64, try w.field(signed, "last_valid_block_height"));
}
pub fn execute(a: A, op: Op, p: V) !V {
    if (op == .build_instruction) return @import("phoenix_instructions.zig").build(a, try fieldString(p, "name"), try w.field(p, "request"));
    if (op == .prepare_transaction) return prepare(a, p);
    if (op == .validate_signed_transaction) {
        try validate(a, p);
        return .null;
    }
    const prepared = try w.field(p, "prepared");
    const m = try Message.reviewed(a, prepared);
    if (op == .validate_server_transaction) {
        const bytes = try unbase64(a, try fieldString(p, "transaction_base64"));
        const end = 1 + 64 * m.signers.len;
        if (bytes.len < end or bytes[0] != m.signers.len or !std.mem.eql(u8, bytes[end..], m.bytes)) return error.InvalidMessage;
        return .null;
    }
    if (op == .sign_transaction) {
        const external = p.object.get("public_key");
        var pair: Ed.KeyPair = undefined;
        defer std.crypto.secureZero(u8, std.mem.asBytes(&pair));
        const pk = if (external) |pubkey| try sol.publicKey(try w.string(pubkey)) else blk: {
            var seed = try @import("evm.zig").hex(32, try fieldString(p, "key"));
            defer std.crypto.secureZero(u8, &seed);
            pair = try Ed.KeyPair.generateDeterministic(seed);
            break :blk pair.public_key.toBytes();
        };
        for (m.signers) |k| {
            if (std.mem.eql(u8, &k, &pk)) break;
        } else return error.UnexpectedSigner;
        var out = p.object.get("signatures") orelse V{ .object = .empty };
        if (out != .object) return error.InvalidSignatures;
        const sig = if (external != null) blk: {
            const proof = p.object.get("signature") orelse return w.text(try b64(a, m.bytes));
            const supplied = try sol.decode58(64, try w.string(proof));
            try verify(pk, supplied, m.bytes);
            break :blk supplied;
        } else (try pair.sign(m.bytes, null)).toBytes();
        try out.object.put(a, try sol.base58(a, 32, pk), w.text(try sol.base58(a, 64, sig)));
        return out;
    }
    var data: Bytes = .empty;
    try data.append(a, @intCast(m.signers.len));
    const signatures = try w.field(p, "signatures");
    for (m.signers) |k| {
        const sig = try sol.decode58(64, try fieldString(signatures, try sol.base58(a, 32, k)));
        try verify(k, sig, m.bytes);
        try data.appendSlice(a, &sig);
    }
    try data.appendSlice(a, m.bytes);
    return w.parse(a, try std.json.Stringify.valueAlloc(a, .{ .transaction_base64 = try b64(a, data.items), .signature = try sol.base58(a, 64, data.items[1..65].*), .last_valid_block_height = try w.field(prepared, "last_valid_block_height") }, .{}));
}
