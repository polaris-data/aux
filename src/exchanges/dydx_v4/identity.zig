const std = @import("std");

const left_index = [80]u4{
    0, 1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
    7, 4,  13, 1,  10, 6,  15, 3,  12, 0, 9,  5,  2,  14, 11, 8,
    3, 10, 14, 4,  9,  15, 8,  1,  2,  7, 0,  6,  13, 11, 5,  12,
    1, 9,  11, 10, 0,  8,  12, 4,  13, 3, 7,  15, 14, 5,  6,  2,
    4, 0,  5,  9,  7,  12, 2,  10, 14, 1, 3,  8,  11, 6,  15, 13,
};
const right_index = [80]u4{
    5,  14, 7,  0, 9, 2,  11, 4,  13, 6,  15, 8,  1,  10, 3,  12,
    6,  11, 3,  7, 0, 13, 5,  10, 14, 15, 8,  12, 4,  9,  1,  2,
    15, 5,  1,  3, 7, 14, 6,  9,  11, 8,  12, 2,  10, 0,  4,  13,
    8,  6,  4,  1, 3, 11, 15, 0,  5,  12, 2,  13, 9,  7,  10, 14,
    12, 15, 10, 4, 1, 5,  8,  7,  6,  2,  13, 14, 0,  3,  9,  11,
};
const left_rotation = [80]u5{
    11, 14, 15, 12, 5,  8,  7,  9,  11, 13, 14, 15, 6,  7,  9,  8,
    7,  6,  8,  13, 11, 9,  7,  15, 7,  12, 15, 9,  11, 7,  13, 12,
    11, 13, 6,  7,  14, 9,  13, 15, 14, 8,  13, 6,  5,  12, 7,  5,
    11, 12, 14, 15, 14, 15, 9,  8,  9,  14, 5,  6,  8,  6,  5,  12,
    9,  15, 5,  11, 6,  8,  13, 12, 5,  12, 13, 14, 11, 8,  5,  6,
};
const right_rotation = [80]u5{
    8,  9,  9,  11, 13, 15, 15, 5,  7,  7,  8,  11, 14, 14, 12, 6,
    9,  13, 15, 7,  12, 8,  9,  11, 7,  7,  12, 7,  6,  15, 13, 11,
    9,  7,  15, 11, 8,  6,  6,  14, 12, 13, 5,  14, 13, 13, 7,  5,
    15, 5,  8,  11, 14, 14, 6,  14, 6,  9,  12, 9,  12, 5,  15, 8,
    8,  5,  12, 9,  12, 5,  14, 6,  8,  13, 6,  5,  15, 13, 11, 11,
};

fn round(which: usize, b: u32, c: u32, d: u32) u32 {
    return switch (which) {
        0 => b ^ c ^ d,
        1 => (b & c) | (~b & d),
        2 => (b | ~c) ^ d,
        3 => (b & d) | (c & ~d),
        4 => b ^ (c | ~d),
        else => unreachable,
    };
}

fn step(reg: *[5]u32, msg: u32, rotation: u5, stage: usize, constant: u32) void {
    const a, const b, const c, const d, const e = reg.*;
    const next = std.math.rotl(u32, a +% round(stage, b, c, d) +% msg +% constant, rotation) +% e;
    reg.* = .{ e, next, b, std.math.rotl(u32, c, 10), d };
}

// Cosmos addresses use RIPEMD-160 of a 32-byte SHA-256 digest. The padded message fits one block.
fn ripemd(input: [32]u8) [20]u8 {
    const initial = [5]u32{ 0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0 };
    const left_constants = [5]u32{ 0, 0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xa953fd4e };
    const right_constants = [5]u32{ 0x50a28be6, 0x5c4dd124, 0x6d703ef3, 0x7a6d76e9, 0 };
    var words: [16]u32 = @splat(0);
    for (0..8) |i| words[i] = std.mem.readInt(u32, input[i * 4 ..][0..4], .little);
    words[8] = 0x80;
    words[14] = 256;
    var left = initial;
    var right = initial;
    for (0..80) |i| {
        step(&left, words[left_index[i]], left_rotation[i], i / 16, left_constants[i / 16]);
        step(&right, words[right_index[i]], right_rotation[i], 4 - i / 16, right_constants[i / 16]);
    }
    var result: [20]u8 = undefined;
    for (0..5) |i| {
        const value = initial[(i + 1) % 5] +% left[(i + 2) % 5] +% right[(i + 3) % 5];
        std.mem.writeInt(u32, result[i * 4 ..][0..4], value, .little);
    }
    return result;
}

fn polymod(prev: u32, value: u8) u32 {
    const generators = [5]u32{ 0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3 };
    const top = prev >> 25;
    var result = ((prev & 0x1ffffff) << 5) ^ value;
    for (generators, 0..) |generator, i| if (((top >> @as(u5, @intCast(i))) & 1) != 0) {
        result ^= generator;
    };
    return result;
}

pub fn address(public_key: [33]u8) [43]u8 {
    const prefix = "dydx";
    const charset = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";
    var sha: [32]u8 = undefined;
    std.crypto.hash.sha2.Sha256.hash(&public_key, &sha, .{});
    const hash = ripemd(sha);
    const number = std.mem.readInt(u160, &hash, .big);
    var checksum: u32 = 1;
    for (prefix) |c| checksum = polymod(checksum, c >> 5);
    checksum = polymod(checksum, 0);
    for (prefix) |c| checksum = polymod(checksum, c & 31);
    var out: [43]u8 = undefined;
    out[0..5].* = "dydx1".*;
    for (0..32) |i| {
        const digit: u5 = @truncate(number >> @as(u8, @intCast(155 - i * 5)));
        out[5 + i] = charset[digit];
        checksum = polymod(checksum, digit);
    }
    for (0..6) |_| checksum = polymod(checksum, 0);
    checksum ^= 1;
    for (0..6) |i| out[37 + i] = charset[(checksum >> @as(u5, @intCast(25 - 5 * i))) & 31];
    return out;
}

test "Cosmos key compressed generator hash and address" {
    const key = try @import("../../core/evm.zig").hex(33, "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    var sha: [32]u8 = undefined;
    std.crypto.hash.sha2.Sha256.hash(&key, &sha, .{});
    try std.testing.expectEqual(try @import("../../core/evm.zig").hex(20, "751e76e8199196d454941c45d1b3a323f1433bd6"), ripemd(sha));
    try std.testing.expectEqualStrings("dydx1w508d6qejxtdg4y5r3zarvary0c5xw7knye700", &address(key));
}
