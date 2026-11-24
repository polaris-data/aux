// Metric CompressedOracleV1 encodes four 48-bit lanes followed by a 56-bit
// timestamp and an 8-bit slot. Its spread table is the pinned venue codebook.
const std = @import("std");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const tx = @import("../../core/transaction.zig");
const table = @import("spreads.zig").codebook;
const Op = enum { codebook, encode_spread, encode_u64x32, decode_u64x32, pack_oracle_slot, oracle_push_calldata, feed_id, slot_update_digest, pusher_consent_digest, sign_personal_digest };
fn fieldNum(comptime T: type, v: w.V, k: []const u8) !T {
    return tx.quantity(T, try w.field(v, k));
}
fn addr(v: w.V, k: []const u8) ![32]u8 {
    return evm.address(try w.string(try w.field(v, k)));
}
fn num(a: w.A, n: anytype) !w.V {
    return .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{n}) };
}
fn hex(a: w.A, bytes: anytype) !w.V {
    return w.text(try std.fmt.allocPrint(a, "0x{x}", .{bytes}));
}
fn spread(n: u16) !u8 {
    if (n > 10000) return error.InvalidSpread;
    for (table, 0..) |value, i| if (value >= n) return @intCast(i);
    unreachable;
}
fn priceWord(n: u64, truncate: bool) !u32 {
    const mask: u64 = (1 << 27) - 1;
    if (n > mask << 31) return error.InvalidPrice;
    const exponent: u5 = if (n <= mask) 0 else @intCast(37 - @clz(n));
    const mantissa: u32 = @intCast(n >> exponent);
    if (!truncate and @as(u64, mantissa) << exponent != n) return error.InvalidPrecision;
    return @as(u32, exponent) << 27 | mantissa;
}
pub fn execute(a: w.A, input: w.V) !w.V {
    const operation = std.meta.stringToEnum(Op, try w.string(try w.field(input, "operation"))) orelse return error.InvalidOperation;
    const v = try w.field(input, "params");
    if (v != .object) return error.InvalidParameter;
    const rounding = if (v.object.get("rounding")) |r| try w.string(r) else "exact";
    const truncate = std.mem.eql(u8, rounding, "down");
    if (!truncate and !std.mem.eql(u8, rounding, "exact") and !std.mem.eql(u8, rounding, "up")) return error.InvalidRounding;
    switch (operation) {
        .codebook => {
            var values: std.array_list.Managed(w.V) = .init(a);
            for (table) |n| try values.append(try num(a, n));
            return .{ .array = values };
        },
        .encode_spread => return num(a, try spread(try fieldNum(u16, v, "units"))),
        .encode_u64x32 => return num(a, try priceWord(try fieldNum(u64, v, "value"), truncate)),
        .decode_u64x32 => {
            const n = try fieldNum(u32, v, "word");
            return num(a, @as(u64, n & 0x7ffffff) << @intCast(n >> 27));
        },
        .pack_oracle_slot => {
            const lanes = try w.field(v, "lanes");
            if (lanes != .array or lanes.array.items.len != 4) return error.InvalidLanes;
            var slot: u256 = @as(u256, try fieldNum(u56, v, "timestampMs")) << 8 | try fieldNum(u8, v, "slotId");
            for (lanes.array.items, 0..) |lane, i| {
                const bits: u48 = if (lane == .null) 65535 else @as(u48, try priceWord(try fieldNum(u64, lane, "price"), truncate)) << 16 |
                    @as(u48, try spread(try fieldNum(u16, lane, "spread0"))) << 8 |
                    try spread(try fieldNum(u16, lane, "spread1"));
                slot |= @as(u256, bits) << @intCast(208 - i * 48);
            }
            return num(a, slot);
        },
        .oracle_push_calldata => {
            const words = try w.field(v, "words");
            if (words != .array or words.array.items.len == 0) return error.InvalidSlots;
            const bytes = try a.alloc(u8, 1 + 32 * words.array.items.len);
            bytes[0] = 0;
            for (words.array.items, 0..) |value, i| bytes[1 + i * 32 ..][0..32].* = evm.encodeWord(try tx.quantity(u256, value));
            return hex(a, bytes);
        },
        .feed_id => {
            const creator = try addr(v, "creator");
            if (std.mem.allEqual(u8, &creator, 0)) return error.InvalidAddress;
            const word = (std.mem.readInt(u256, &creator, .big) << 96) | (@as(u256, try fieldNum(u80, v, "chainId")) << 16) |
                (@as(u256, try fieldNum(u8, v, "slot")) << 8) | try fieldNum(u2, v, "position");
            return hex(a, &evm.encodeWord(word));
        },
        .slot_update_digest, .pusher_consent_digest => {
            var bytes: [160]u8 = undefined;
            bytes[0..32].* = evm.encodeWord(try fieldNum(u256, v, "chainId"));
            bytes[32..64].* = try addr(v, "oracle");
            if (operation == .slot_update_digest) {
                bytes[64..96].* = try addr(v, "creator");
                bytes[96..128].* = evm.encodeWord(try fieldNum(u256, v, "word"));
                return hex(a, &evm.hash(bytes[0..128]));
            }
            bytes[64..96].* = evm.encodeWord(try fieldNum(u256, v, "deadline"));
            bytes[96..128].* = try addr(v, "pusher");
            bytes[128..160].* = try addr(v, "creator");
            return hex(a, &evm.hash(&bytes));
        },
        .sign_personal_digest => {
            const digest = try evm.hex(32, try w.string(try w.field(v, "digest")));
            const message = "\x19Ethereum Signed Message:\n32".* ++ digest;
            var key = try evm.hex(32, try w.string(try w.field(v, "key")));
            defer std.crypto.secureZero(u8, &key);
            return hex(a, &try tx.recoverable(key, evm.hash(&message)));
        },
    }
}
