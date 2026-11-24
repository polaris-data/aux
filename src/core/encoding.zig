const std = @import("std");
const upper = "0123456789ABCDEF";
const lower = "0123456789abcdef";

// The Rust bindings allocate exact-size output for hex, or at most 3x input
// for URL escaping. A length mismatch is reported before writing any bytes.
pub export fn aux_v1_hex_encode(input: [*]const u8, len: usize, output: [*]u8, capacity: usize) bool {
    if (len > capacity / 2) return false;
    for (input[0..len], 0..) |byte, i| {
        output[2 * i] = lower[byte >> 4];
        output[2 * i + 1] = lower[byte & 15];
    }
    return true;
}

pub export fn aux_v1_hex_decode(input: [*]const u8, len: usize, output: [*]u8, capacity: usize, bad: *usize) u8 {
    if (len % 2 != 0) return 1;
    if (len / 2 != capacity) return 2;
    for (output[0..capacity], 0..) |*byte, i| {
        const high = std.fmt.charToDigit(input[i * 2], 16) catch {
            bad.* = i * 2;
            return 3;
        };
        const low = std.fmt.charToDigit(input[i * 2 + 1], 16) catch {
            bad.* = i * 2 + 1;
            return 3;
        };
        byte.* = (high << 4) | low;
    }
    return 0;
}

pub export fn aux_v1_url_component(input: [*]const u8, len: usize, output: [*]u8, capacity: usize) usize {
    if (len > capacity / 3) return 0;
    var n: usize = 0;
    for (input[0..len]) |b| {
        switch (b) {
            '0'...'9', 'a'...'z', 'A'...'Z', '-', '.', '_', '~' => {
                output[n] = b;
                n += 1;
            },
            else => {
                output[n..][0..3].* = .{ '%', upper[b >> 4], upper[b & 15] };
                n += 3;
            },
        }
    }
    return n;
}
