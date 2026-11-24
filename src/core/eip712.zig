//! EIP-712 encodeType, hashStruct and the EIP-191 signing envelope.
const std = @import("std");
const w = @import("wire.zig");
const evm = @import("evm.zig");
const A = w.A;
const V = w.V;
const Word = [32]u8;
const Keccak = std.crypto.hash.sha3.Keccak256;
const eq = std.mem.eql;
fn text(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn identifier(name: []const u8) bool {
    if (name.len == 0 or std.ascii.isDigit(name[0])) return false;
    for (name) |c| if (c != '_' and !std.ascii.isAlphanumeric(c)) return false;
    return true;
}
fn width(s: []const u8, max: u16) !u16 {
    if (s.len == 0 or s[0] == '0') return error.InvalidType;
    for (s) |c| if (!std.ascii.isDigit(c)) return error.InvalidType;
    const n = try std.fmt.parseInt(u16, s, 10);
    if (n == 0 or n > max) return error.InvalidType;
    return n;
}
fn less(_: void, a: []const u8, b: []const u8) bool {
    return std.mem.lessThan(u8, a, b);
}
const Encoder = struct {
    a: A,
    types: V,
    hashes: std.StringHashMapUnmanaged(Word) = .empty,
    fn collect(self: *Encoder, name: []const u8, seen: *std.StringHashMapUnmanaged(void), depth: u8) !void {
        if (seen.contains(name)) return;
        if (depth > 96 or !identifier(name)) return error.InvalidType;
        const fields = try w.field(self.types, name);
        if (fields != .array) return error.InvalidType;
        try seen.put(self.a, name, {});
        var names: std.StringHashMapUnmanaged(void) = .empty;
        for (fields.array.items) |field| {
            const key = try text(field, "name");
            const t = try text(field, "type");
            if (!identifier(key) or names.contains(key)) return error.InvalidType;
            try names.put(self.a, key, {});
            const base = t[0 .. std.mem.findScalar(u8, t, '[') orelse t.len];
            if (self.types.object.contains(base)) try self.collect(base, seen, depth + 1);
        }
    }
    fn appendType(self: *Encoder, h: *Keccak, name: []const u8) !void {
        const fields = try w.field(self.types, name);
        h.update(name);
        h.update("(");
        for (fields.array.items, 0..) |field, i| {
            if (i != 0) h.update(",");
            h.update(try text(field, "type"));
            h.update(" ");
            h.update(try text(field, "name"));
        }
        h.update(")");
    }
    fn typeHash(self: *Encoder, name: []const u8) !Word {
        if (self.hashes.get(name)) |result| return result;
        var dependencies: std.StringHashMapUnmanaged(void) = .empty;
        try self.collect(name, &dependencies, 0);
        _ = dependencies.remove(name);
        var names: std.ArrayList([]const u8) = .empty;
        var it = dependencies.keyIterator();
        while (it.next()) |k| try names.append(self.a, k.*);
        std.mem.sort([]const u8, names.items, {}, less);
        var hasher = Keccak.init(.{});
        try self.appendType(&hasher, name);
        for (names.items) |n| try self.appendType(&hasher, n);
        var result: Word = undefined;
        hasher.final(&result);
        try self.hashes.put(self.a, name, result);
        return result;
    }
    fn value(self: *Encoder, t: []const u8, v: V, depth: u8) anyerror!Word {
        if (depth > 96 or t.len == 0) return error.InvalidType;
        if (t[t.len - 1] == ']') {
            const open = std.mem.findScalarLast(u8, t, '[') orelse return error.InvalidType;
            const size = t[open + 1 .. t.len - 1];
            if (v != .array or (size.len > 0 and try std.fmt.parseInt(usize, size, 10) != v.array.items.len)) return error.InvalidArray;
            var h = Keccak.init(.{});
            for (v.array.items) |item| h.update(&try self.value(t[0..open], item, depth + 1));
            var result: Word = undefined;
            h.final(&result);
            return result;
        }
        if (self.types.object.contains(t)) return self.structure(t, v, depth + 1);
        if (eq(u8, t, "bool")) return evm.encodeWord(if (v == .bool) @intFromBool(v.bool) else return error.InvalidBool);
        if (eq(u8, t, "address")) return evm.address(try w.string(v));
        if (eq(u8, t, "string")) return evm.hash(try w.string(v));
        if (std.mem.startsWith(u8, t, "bytes")) {
            const bytes = try evm.fromHex(self.a, try w.string(v));
            if (t.len == 5) return evm.hash(bytes);
            if (bytes.len != try width(t[5..], 32)) return error.InvalidLength;
            var result: Word = @splat(0);
            @memcpy(result[0..bytes.len], bytes);
            return result;
        }
        const signed = std.mem.startsWith(u8, t, "int");
        if (!signed and !std.mem.startsWith(u8, t, "uint")) return error.InvalidType;
        const bits = try width(t[if (signed) @as(usize, 3) else 4..], 256);
        if (bits % 8 != 0) return error.InvalidType;
        const raw = if (v == .number_string) v.number_string else try w.string(v);
        const neg = raw.len > 0 and raw[0] == '-';
        const n = try @import("transaction.zig").quantity(u256, w.text(if (neg) raw[1..] else raw));
        const max: u256 = if (signed) (@as(u256, 1) << @intCast(bits - 1)) else @as(u256, std.math.maxInt(u256)) >> @intCast(256 - bits);
        if (neg and (!signed or n > max) or !neg and (if (signed) n >= max else n > max)) return error.OutOfRange;
        return evm.encodeWord(if (neg) 0 -% n else n);
    }
    fn structure(self: *Encoder, name: []const u8, v: V, depth: u8) anyerror!Word {
        if (v != .object or depth > 96) return error.InvalidMessage;
        var h = Keccak.init(.{});
        h.update(&try self.typeHash(name));
        for ((try w.field(self.types, name)).array.items) |field| h.update(&try self.value(try text(field, "type"), try w.field(v, try text(field, "name")), depth + 1));
        var result: Word = undefined;
        h.final(&result);
        return result;
    }
};
pub fn hash(a: A, td: V) !Word {
    const types = try w.field(td, "types");
    if (types != .object) return error.InvalidType;
    var encoder = Encoder{ .a = a, .types = types };
    const primary = try text(td, "primaryType");
    const domain = try encoder.structure("EIP712Domain", try w.field(td, "domain"), 0);
    var h = Keccak.init(.{});
    h.update("\x19\x01");
    h.update(&domain);
    if (!eq(u8, primary, "EIP712Domain")) h.update(&try encoder.structure(primary, try w.field(td, "message"), 0));
    var out: Word = undefined;
    h.final(&out);
    return out;
}

pub fn structureHash(a: A, types: V, name: []const u8, value: V) !Word {
    if (types != .object) return error.InvalidType;
    var encoder = Encoder{ .a = a, .types = types };
    return encoder.structure(name, value, 0);
}
