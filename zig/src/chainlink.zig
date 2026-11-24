// Chainlink Data Streams report envelope and version-specific ABI words.
const std = @import("std");
const Allocator = std.mem.Allocator;
const Value = std.json.Value;

const Type = enum { bytes32, uint32, uint64, uint192, int192 };
const Field = struct { name: []const u8, ty: Type };
const common = [_]Field{
    .{ .name = "feedId", .ty = .bytes32 },
    .{ .name = "validFromTimestamp", .ty = .uint32 },
    .{ .name = "observationsTimestamp", .ty = .uint32 },
    .{ .name = "nativeFee", .ty = .uint192 },
    .{ .name = "linkFee", .ty = .uint192 },
    .{ .name = "expiresAt", .ty = .uint32 },
};
fn fields(version: u16) ?[]const Field {
    return switch (version) {
        2 => &.{.{ .name = "price", .ty = .int192 }},
        3 => &.{ .{ .name = "price", .ty = .int192 }, .{ .name = "bid", .ty = .int192 }, .{ .name = "ask", .ty = .int192 } },
        7 => &.{.{ .name = "exchangeRate", .ty = .int192 }},
        8 => &.{ .{ .name = "lastUpdateTimestamp", .ty = .uint64 }, .{ .name = "midPrice", .ty = .int192 }, .{ .name = "marketStatus", .ty = .uint32 } },
        9 => &.{ .{ .name = "navPerShare", .ty = .int192 }, .{ .name = "navDate", .ty = .uint64 }, .{ .name = "aum", .ty = .int192 }, .{ .name = "ripcord", .ty = .uint32 } },
        10 => &.{ .{ .name = "lastUpdateTimestamp", .ty = .uint64 }, .{ .name = "price", .ty = .int192 }, .{ .name = "marketStatus", .ty = .uint32 }, .{ .name = "currentMultiplier", .ty = .int192 }, .{ .name = "newMultiplier", .ty = .int192 }, .{ .name = "activationDateTime", .ty = .uint32 }, .{ .name = "tokenizedPrice", .ty = .int192 } },
        11 => &.{ .{ .name = "mid", .ty = .int192 }, .{ .name = "lastSeenTimestampNs", .ty = .uint64 }, .{ .name = "bid", .ty = .int192 }, .{ .name = "bidVolume", .ty = .int192 }, .{ .name = "ask", .ty = .int192 }, .{ .name = "askVolume", .ty = .int192 }, .{ .name = "lastTradedPrice", .ty = .int192 }, .{ .name = "marketStatus", .ty = .uint32 } },
        14 => &.{ .{ .name = "midPrice", .ty = .int192 }, .{ .name = "bidPrice", .ty = .int192 }, .{ .name = "askPrice", .ty = .int192 }, .{ .name = "expiryTime", .ty = .uint64 }, .{ .name = "firstDayOfNotice", .ty = .uint64 }, .{ .name = "lastSeenTimestampNs", .ty = .uint64 }, .{ .name = "marketStatus", .ty = .uint32 }, .{ .name = "contractMonth", .ty = .uint32 }, .{ .name = "goldmanRollPrice", .ty = .int192 }, .{ .name = "currentBusinessDay", .ty = .uint32 }, .{ .name = "interpolatedGoldmanRollPrice", .ty = .int192 } },
        else => null,
    };
}

fn length(word: []const u8) !usize {
    return std.math.cast(usize, std.mem.readInt(u256, word[0..32], .big)) orelse error.InvalidOffset;
}

fn fieldValue(a: Allocator, ty: Type, word: *const [32]u8) !Value {
    const n = std.mem.readInt(u256, word, .big);
    switch (ty) {
        .bytes32 => return .{ .string = try std.fmt.allocPrint(a, "0x{x}", .{word}) },
        .uint32 => return .{ .integer = std.math.cast(u32, n) orelse return error.InvalidWidth },
        .uint64 => if (n > std.math.maxInt(u64)) return error.InvalidWidth,
        .uint192 => if (n > std.math.maxInt(u192)) return error.InvalidWidth,
        .int192 => {
            const signed: i256 = @bitCast(n);
            const narrowed = std.math.cast(i192, signed) orelse return error.InvalidWidth;
            return .{ .string = try std.fmt.allocPrint(a, "{d}", .{narrowed}) };
        },
    }
    return .{ .string = try std.fmt.allocPrint(a, "{d}", .{n}) };
}

/// Decodes the canonical ABI envelope. The arena holds the result and scratch.
pub fn decode(a: Allocator, report: []const u8) !Value {
    const hex = if (std.mem.startsWith(u8, report, "0x") or std.mem.startsWith(u8, report, "0X")) report[2..] else report;
    if (hex.len > 2 * 16 * 1024 * 1024 or hex.len % 2 != 0) return error.InvalidLength;
    const raw = try a.alloc(u8, hex.len / 2);
    _ = try std.fmt.hexToBytes(raw, hex);
    const head = 7 * 32;
    if (raw.len < head + 32 or try length(raw[96..]) != head) return error.InvalidEnvelope;
    const n = try length(raw[head..]);
    if (n < 32 or n > raw.len - head - 32) return error.InvalidBody;
    const body = raw[head + 32 ..][0..n];
    const end = head + 32 + n;
    const rs = std.mem.alignForward(usize, end, 32);
    if (rs > raw.len - 32 or try length(raw[128..]) != rs or !std.mem.allEqual(u8, raw[end..rs], 0)) return error.InvalidEnvelope;
    const count = try length(raw[rs..]);
    if (count > 32) return error.InvalidSignatures;
    const ss = rs + 32 + 32 * count;
    if (ss > raw.len - 32 or try length(raw[160..]) != ss or try length(raw[ss..]) != count or raw.len != ss + 32 + 32 * count) return error.InvalidEnvelope;
    const version = std.mem.readInt(u16, body[0..2], .big);
    const tail = fields(version);
    var result: Value = .{ .object = .empty };
    try result.object.put(a, "kind", .{ .string = if (tail == null) "unknown" else "known" });
    try result.object.put(a, "version", .{ .integer = version });
    try result.object.put(a, "full_report", .{ .string = report });
    try result.object.put(a, "report_body", .{ .string = try std.fmt.allocPrint(a, "0x{x}", .{body}) });
    if (tail) |native| {
        if (body.len != (common.len + native.len) * 32) return error.WrongSchema;
        var values: Value = .{ .object = .empty };
        var offset: usize = 0;
        for ([_][]const Field{ &common, native }) |schema| {
            for (schema) |field| {
                try values.object.put(a, field.name, try fieldValue(a, field.ty, body[offset..][0..32]));
                offset += 32;
            }
        }
        try result.object.put(a, "fields", values);
    }
    return result;
}
