const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const http = @import("../../core/http.zig");

var nonce: std.atomic.Value(u64) = .init(0);

fn nextNonce(now: u64) u64 {
    var current = nonce.load(.monotonic);
    while (true) {
        const next = @max(now, current + 1);
        current = nonce.cmpxchgWeak(current, next, .monotonic, .monotonic) orelse return next;
    }
}

pub fn digest(fields: []const u8) p.Word {
    const message = evm.hash(&(evm.hash("Message(string msg)") ++ evm.hash(fields)));
    return p.typedHash("AsterSignTransaction", "1", 1666, @splat(0), message);
}

pub fn sign(a: p.A, auth: p.V, method: []const u8, path: []const u8, body: p.V, now: u64) !struct { path: []const u8, body: ?[]const u8 } {
    const user = try p.credential(p.get(auth, "user"));
    _ = try evm.address(user);
    var key = try p.evmKey(auth);
    defer std.crypto.secureZero(u8, &key);
    const address = try p.evmAddress(key);
    const supplied_signer = p.get(auth, "signer");
    if (supplied_signer != .null and !p.eq(u8, &address, &try evm.hex(20, try w.string(supplied_signer)))) return error.InvalidIdentity;
    const signer = try std.fmt.allocPrint(a, "0x{x}", .{&address});
    const is_get = p.eq(u8, method, "GET");
    const prefix = if (std.mem.findScalar(u8, path, '?')) |i| path[0..i] else path;
    var form: std.ArrayList(u8) = .empty;
    if (is_get) {
        try form.appendSlice(a, path[prefix.len..]);
    } else if (body != .null) {
        if (body != .object) return error.ExpectedObject;
        for (body.object.keys(), body.object.values()) |name, value| {
            if (value == .null) continue;
            if (p.eq(u8, name, "user") or p.eq(u8, name, "signer") or p.eq(u8, name, "signature") or p.eq(u8, name, "nonce")) return error.InvalidParameter;
            try http.pair(a, &form, name, try http.scalar(value), true);
        }
    }
    try http.pair(a, &form, "user", user, true);
    try http.pair(a, &form, "signer", signer, true);
    try http.pair(a, &form, "nonce", try std.fmt.allocPrint(a, "{d}", .{nextNonce(now)}), true);
    const signature = try p.signature(a, key, digest(form.items[1..]));
    try http.pair(a, &form, "signature", signature, true);
    return if (is_get) .{ .path = try std.mem.concat(a, u8, &.{ prefix, form.items }), .body = null } else .{ .path = prefix, .body = form.items[1..] };
}
