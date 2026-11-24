// Canonical Solidity ABI encoding and decoding using native 256-bit integers.
const std = @import("std");
const registry = @import("contracts.zig");
const Param = registry.Param;
const A = std.mem.Allocator;
const V = std.json.Value;
const Bytes = std.ArrayList(u8);
const eq = std.mem.eql;
const limit = 16 * 1024 * 1024;

const Word = [32]u8;
pub fn hex(comptime n: usize, value: []const u8) ![n]u8 {
    const digits = if (std.mem.startsWith(u8, value, "0x")) value[2..] else value;
    var bytes: [n]u8 = undefined;
    if (digits.len != n * 2) return error.InvalidLength;
    _ = try std.fmt.hexToBytes(&bytes, digits);
    return bytes;
}
pub fn encodeWord(n: u256) Word {
    var bytes: Word = undefined;
    std.mem.writeInt(u256, &bytes, n, .big);
    return bytes;
}
pub fn address(value: []const u8) !Word {
    return @as([12]u8, @splat(0)) ++ try hex(20, value);
}

pub fn fromHex(a: A, input: []const u8) ![]u8 {
    const s = if (std.mem.startsWith(u8, input, "0x") or std.mem.startsWith(u8, input, "0X")) input[2..] else input;
    if (s.len % 2 != 0 or s.len / 2 > limit) return error.InvalidHex;
    return std.fmt.hexToBytes(try a.alloc(u8, s.len / 2), s);
}
pub fn hash(s: []const u8) [32]u8 {
    var result: [32]u8 = undefined;
    std.crypto.hash.sha3.Keccak256.hash(s, &result, .{});
    return result;
}
fn field(v: V, key: []const u8) !V {
    if (v != .object) return error.InvalidObject;
    return v.object.get(key) orelse error.MissingField;
}
fn str(v: V) ![]const u8 {
    return if (v == .string) v.string else error.ExpectedString;
}
fn decimal(v: V) ![]const u8 {
    return if (v == .number_string) v.number_string else str(v);
}
fn parseInteger(comptime T: type, s: []const u8) !T {
    if (s.len == 0 or s.len > 79) return error.InvalidInteger;
    if (std.mem.startsWith(u8, s, "0x")) {
        if (s.len == 2 or s.len > 66) return error.InvalidInteger;
        for (s[2..]) |b| if (!std.ascii.isHex(b)) return error.InvalidInteger;
        return std.fmt.parseInt(T, s[2..], 16);
    }
    const start: usize = @intFromBool(s[0] == '-' and @typeInfo(T).int.signedness == .signed);
    if (start == s.len) return error.InvalidInteger;
    for (s[start..]) |b| if (!std.ascii.isDigit(b)) return error.InvalidInteger;
    return std.fmt.parseInt(T, s, 10);
}
fn inner(p: Param) ?Param {
    if (!std.mem.endsWith(u8, p.ty, "]")) return null;
    const i = std.mem.findScalarLast(u8, p.ty, '[').?;
    return .{ .ty = p.ty[0..i], .name = "", .components = p.components };
}
fn fixedSize(p: Param) ?usize {
    if (inner(p)) |child| {
        const text = p.ty[child.ty.len + 1 .. p.ty.len - 1];
        if (text.len == 0) return null;
        return (fixedSize(child) orelse return null) * (std.fmt.parseInt(usize, text, 10) catch unreachable);
    }
    if (eq(u8, p.ty, "bytes") or eq(u8, p.ty, "string")) return null;
    if (!eq(u8, p.ty, "tuple")) return 32;
    var n: usize = 0;
    for (p.components) |child| n += fixedSize(child) orelse return null;
    return n;
}
fn putWord(a: A, out: *Bytes, n: u256) !void {
    var bytes: [32]u8 = undefined;
    std.mem.writeInt(u256, &bytes, n, .big);
    try out.appendSlice(a, &bytes);
}
fn padded(a: A, out: *Bytes, bytes: []const u8) !void {
    try out.appendSlice(a, bytes);
    try out.appendNTimes(a, 0, (32 - bytes.len % 32) % 32);
}
fn checkCount(p: Param, child: Param, n: usize) !void {
    const text = p.ty[child.ty.len + 1 .. p.ty.len - 1];
    if (text.len != 0 and n != try std.fmt.parseInt(usize, text, 10)) return error.ArrayLength;
}
fn series(a: A, out: *Bytes, params: []const Param, repeat: ?Param, n: usize, values: V) anyerror!void {
    const count = switch (values) {
        .array => |arr| arr.items.len,
        .object => |obj| obj.count(),
        else => return error.ExpectedArguments,
    };
    if (count != n or (values == .object and repeat != null)) return error.WrongArity;
    var head_len: usize = 0;
    for (0..n) |i| head_len += fixedSize(repeat orelse params[i]) orelse 32;
    if (head_len > limit) return error.TooLarge;
    var tail: Bytes = .empty;
    defer tail.deinit(a);
    for (0..n) |i| {
        const p = repeat orelse params[i];
        const v = if (values == .array) values.array.items[i] else try field(values, p.name);
        if (fixedSize(p) == null) {
            try putWord(a, out, head_len + tail.items.len);
            try encode(a, &tail, p, v);
        } else try encode(a, out, p, v);
        if (out.items.len + tail.items.len > limit) return error.TooLarge;
    }
    try out.appendSlice(a, tail.items);
}
fn encode(a: A, out: *Bytes, p: Param, v: V) anyerror!void {
    const t = p.ty;
    if (inner(p)) |child| {
        if (v != .array) return error.ExpectedArray;
        const n = v.array.items.len;
        try checkCount(p, child, n);
        if (std.mem.endsWith(u8, t, "[]")) try putWord(a, out, n);
        return series(a, out, &.{}, child, n, v);
    }
    if (eq(u8, t, "tuple")) return series(a, out, p.components, null, p.components.len, v);
    if (eq(u8, t, "bool")) return putWord(a, out, if (v == .bool) @intFromBool(v.bool) else return error.ExpectedBoolean);
    if (std.mem.startsWith(u8, t, "uint") or std.mem.startsWith(u8, t, "int")) {
        const signed = t[0] == 'i';
        const width = try std.fmt.parseInt(u9, t[if (signed) @as(usize, 3) else 4..], 10);
        const n: u256 = if (signed) @bitCast(try parseInteger(i256, try decimal(v))) else try parseInteger(u256, try decimal(v));
        if (width < 256) {
            const shift: u8 = @intCast(256 - width);
            if (signed) {
                if ((@as(i256, @bitCast(n << shift)) >> shift) != @as(i256, @bitCast(n))) return error.OutOfRange;
            } else if (n >> @as(u8, @intCast(width)) != 0) return error.OutOfRange;
        }
        return putWord(a, out, n);
    }
    const bytes = if (eq(u8, t, "string")) try str(v) else try fromHex(a, try str(v));
    if (eq(u8, t, "address")) {
        if (bytes.len != 20) return error.AddressLength;
        try out.appendNTimes(a, 0, 12);
        return out.appendSlice(a, bytes);
    }
    if (eq(u8, t, "bytes") or eq(u8, t, "string")) {
        try putWord(a, out, bytes.len);
    } else {
        const n = if (eq(u8, t, "function")) 24 else try std.fmt.parseInt(usize, t["bytes".len..], 10);
        if (bytes.len != n) return error.BytesLength;
    }
    return padded(a, out, bytes);
}
fn lookup(venue: registry.Venue, contract: []const u8, kind: registry.Kind, name: []const u8) !registry.Entry {
    var found: ?registry.Entry = null;
    for (&registry.entries) |*e| {
        if (e.venue != venue or e.kind != kind or !eq(u8, contract, e.contract)) continue;
        if (!eq(u8, name, e.name) and !eq(u8, name, e.signature)) continue;
        if (found != null) return error.AmbiguousSignature;
        found = e.*;
    }
    return found orelse error.UnknownContractMember;
}
pub fn encodeCall(a: A, venue: registry.Venue, input: V) !V {
    const f = try lookup(venue, try str(try field(input, "contract")), .function, try str(try field(input, "function")));
    var out: Bytes = .empty;
    try out.appendSlice(a, hash(f.signature)[0..4]);
    try series(a, &out, f.inputs, null, f.inputs.len, try field(input, "args"));
    return .{ .string = try std.fmt.allocPrint(a, "0x{x}", .{out.items}) };
}

pub fn prepareCall(a: A, venue: registry.Venue, input: V) !V {
    const to = try hex(20, try str(try field(input, "to")));
    const value = try @import("transaction.zig").quantity(u256, input.object.get("value") orelse .{ .string = "0x0" });
    var out = V{ .object = .empty };
    try out.object.put(a, "to", .{ .string = try std.fmt.allocPrint(a, "0x{x}", .{&to}) });
    try out.object.put(a, "data", try encodeCall(a, venue, input));
    try out.object.put(a, "value", .{ .string = try std.fmt.allocPrint(a, "0x{x}", .{value}) });
    return out;
}

const Decoded = struct { value: V, used: usize };
fn word(data: []const u8) !u256 {
    if (data.len < 32) return error.Truncated;
    return std.mem.readInt(u256, data[0..32], .big);
}
fn size(data: []const u8) !usize {
    return std.math.cast(usize, try word(data)) orelse error.TooLarge;
}
fn decodeSeries(a: A, data: []const u8, params: []const Param, repeat: ?Param, count: usize) anyerror!Decoded {
    if (count > data.len / 32) return error.Truncated;
    var head_size: usize = 0;
    for (0..count) |i| head_size += fixedSize(repeat orelse params[i]) orelse 32;
    if (head_size > data.len) return error.Truncated;
    var head: usize = 0;
    var tail = head_size;
    var values = std.array_list.Managed(V).init(a);
    for (0..count) |i| {
        const p = repeat orelse params[i];
        const decoded = if (fixedSize(p)) |n| blk: {
            const value = try decode(a, data[head..][0..n], p);
            head += n;
            break :blk value;
        } else blk: {
            if (try size(data[head..]) != tail) return error.NonCanonicalOffset;
            head += 32;
            const value = try decode(a, data[tail..], p);
            tail += value.used;
            break :blk value;
        };
        try values.append(decoded.value);
    }
    return .{ .value = .{ .array = values }, .used = tail };
}
fn decode(a: A, data: []const u8, p: Param) anyerror!Decoded {
    const t = p.ty;
    if (inner(p)) |child| {
        const dyn = std.mem.endsWith(u8, t, "[]");
        const n = if (dyn) try size(data) else try std.fmt.parseInt(usize, t[child.ty.len + 1 .. t.len - 1], 10);
        const prefix: usize = if (dyn) 32 else 0;
        const result = try decodeSeries(a, data[prefix..], &.{}, child, n);
        return .{ .value = result.value, .used = prefix + result.used };
    }
    if (eq(u8, t, "tuple")) return decodeSeries(a, data, p.components, null, p.components.len);
    const n = try word(data);
    var used: usize = 32;
    const v: V = blk: {
        if (eq(u8, t, "bool")) {
            if (n > 1) return error.InvalidBoolean;
            break :blk .{ .bool = n == 1 };
        }
        if (std.mem.startsWith(u8, t, "int") or std.mem.startsWith(u8, t, "uint")) {
            const signed = t[0] == 'i';
            const width = try std.fmt.parseInt(u9, t[if (signed) @as(usize, 3) else 4..], 10);
            if (width < 256) {
                const shift: u8 = @intCast(256 - width);
                if (signed) {
                    if (@as(i256, @bitCast(n << shift)) >> shift != @as(i256, @bitCast(n))) return error.OutOfRange;
                } else if (n >> @as(u8, @intCast(width)) != 0) return error.OutOfRange;
            }
            const number = if (signed) try std.fmt.allocPrint(a, "{d}", .{@as(i256, @bitCast(n))}) else try std.fmt.allocPrint(a, "{d}", .{n});
            break :blk .{ .number_string = number };
        }
        if (eq(u8, t, "address")) {
            if (n >> 160 != 0) return error.InvalidPadding;
            break :blk .{ .string = try std.fmt.allocPrint(a, "0x{x}", .{data[12..32]}) };
        }
        if (eq(u8, t, "string") or eq(u8, t, "bytes")) {
            const len = try size(data);
            if (len > data.len - 32) return error.Truncated;
            used += std.mem.alignForward(usize, len, 32);
            if (used > data.len or !std.mem.allEqual(u8, data[32 + len .. used], 0)) return error.InvalidPadding;
            const b = data[32..][0..len];
            if (eq(u8, t, "string")) {
                if (!std.unicode.utf8ValidateSlice(b)) return error.InvalidString;
                break :blk .{ .string = b };
            }
            break :blk .{ .string = try std.fmt.allocPrint(a, "0x{x}", .{b}) };
        }
        const len = if (eq(u8, t, "function")) 24 else try std.fmt.parseInt(usize, t[5..], 10);
        if (!std.mem.allEqual(u8, data[len..32], 0)) return error.InvalidPadding;
        break :blk .{ .string = try std.fmt.allocPrint(a, "0x{x}", .{data[0..len]}) };
    };
    return .{ .value = v, .used = used };
}

fn all(a: A, data: []const u8, params: []const Param) !V {
    const decoded = try decodeSeries(a, data, params, null, params.len);
    if (decoded.used != data.len) return error.TrailingBytes;
    return decoded.value;
}

pub fn decodeCall(a: A, venue: registry.Venue, input: V, result: bool) !V {
    const f = try lookup(venue, try str(try field(input, "contract")), .function, try str(try field(input, "function")));
    const data = try fromHex(a, try str(try field(input, "data")));
    if (result) return all(a, data, f.outputs);
    if (!std.mem.startsWith(u8, data, hash(f.signature)[0..4])) return error.SelectorMismatch;
    return all(a, data[4..], f.inputs);
}
pub fn decodeError(a: A, venue: registry.Venue, input: V) !V {
    const contract = try str(try field(input, "contract"));
    const data = try fromHex(a, try str(try field(input, "data")));
    for (&registry.entries) |*e| {
        if (e.venue != venue or e.kind != .revert or !eq(u8, contract, e.contract)) continue;
        if (!std.mem.startsWith(u8, data, hash(e.signature)[0..4])) continue;
        var v: V = .{ .object = .empty };
        try v.object.put(a, "name", .{ .string = e.name });
        try v.object.put(a, "arguments", try all(a, data[4..], e.inputs));
        return v;
    }
    return error.UnknownSelector;
}
pub fn decodeEvent(a: A, venue: registry.Venue, input: V) !V {
    const e = try lookup(venue, try str(try field(input, "contract")), .event, try str(try field(input, "event")));
    const topics = try field(input, "topics");
    if (topics != .array or topics.array.items.len == 0) return error.MissingTopics;
    const first = try fromHex(a, try str(topics.array.items[0]));
    if (!eq(u8, first, &hash(e.signature))) return error.SelectorMismatch;
    var index: usize = 1;
    var body: std.ArrayList(Param) = .empty;
    var indexed = std.array_list.Managed(V).init(a);
    for (e.inputs) |p| {
        if (!p.indexed) {
            try body.append(a, p);
            continue;
        }
        if (index == topics.array.items.len) return error.MissingTopics;
        const bytes = try fromHex(a, try str(topics.array.items[index]));
        if (bytes.len != 32) return error.InvalidTopic;
        index += 1;
        if (inner(p) != null or eq(u8, p.ty, "tuple") or fixedSize(p) == null) {
            try indexed.append(.{ .string = try std.fmt.allocPrint(a, "0x{x}", .{bytes}) });
        } else try indexed.append((try decode(a, bytes, p)).value);
    }
    if (index != topics.array.items.len) return error.InvalidTopic;
    var v: V = .{ .object = .empty };
    try v.object.put(a, "name", .{ .string = e.name });
    try v.object.put(a, "indexed", .{ .array = indexed });
    try v.object.put(a, "body", try all(a, try fromHex(a, try str(try field(input, "data"))), body.items));
    return v;
}
