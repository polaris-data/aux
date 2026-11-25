const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const sol = @import("../../core/solana.zig");
const evm = @import("../../core/evm.zig");

fn less(_: void, left: []const u8, right: []const u8) bool {
    return std.mem.lessThan(u8, left, right);
}

fn sorted(a: p.A, value: p.V) anyerror!p.V {
    switch (value) {
        .object => {
            var result = p.object();
            const keys = try a.dupe([]const u8, value.object.keys());
            std.mem.sort([]const u8, keys, {}, less);
            for (keys) |key| try p.put(&result, a, key, try sorted(a, value.object.get(key).?));
            return result;
        },
        .array => {
            var result = try p.array(a, &.{});
            for (value.array.items) |item| try result.array.append(try sorted(a, item));
            return result;
        },
        else => return value,
    }
}

pub fn sign(a: p.A, operation: []const u8, auth: p.V, body: p.V, now: u64) !p.V {
    const operation_type = if (p.eq(u8, operation, "place_order")) "create_order" else if (p.eq(u8, operation, "place_market_order")) "create_market_order" else "cancel_order";
    const account = try p.credential(p.get(auth, "account"));
    const account_key = try sol.publicKey(account);
    var seed = try evm.hex(32, try p.credential(p.get(auth, "seed")));
    defer std.crypto.secureZero(u8, &seed);
    var key_pair = try std.crypto.sign.Ed25519.KeyPair.generateDeterministic(seed);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&key_pair));
    const signing_key = key_pair.public_key.toBytes();
    const agent = p.get(auth, "agent_wallet");
    if (agent == .null) {
        if (!p.eq(u8, &account_key, &signing_key)) return error.InvalidIdentity;
    } else if (!p.eq(u8, &try sol.publicKey(try w.string(agent)), &signing_key)) return error.InvalidIdentity;

    if (body != .object) return error.ExpectedObject;
    for ([_][]const u8{ "account", "agent_wallet", "signature", "timestamp", "expiry_window" }) |reserved| {
        if (body.object.contains(reserved)) return error.InvalidParameter;
    }
    const expiry = p.get(auth, "expiry_window");
    const expiry_window = if (expiry == .null) 5000 else try w.unsigned(u64, expiry);
    if (expiry_window == 0) return error.InvalidTimestamp;
    const message = try std.json.Stringify.valueAlloc(a, .{
        .data = try sorted(a, body),
        .expiry_window = expiry_window,
        .timestamp = now,
        .type = operation_type,
    }, .{});
    var request = p.object();
    for (body.object.keys(), body.object.values()) |key, value| try p.put(&request, a, key, value);
    try p.put(&request, a, "account", w.text(account));
    if (agent != .null) try p.put(&request, a, "agent_wallet", agent);
    try p.put(&request, a, "expiry_window", .{ .integer = @intCast(expiry_window) });
    try p.put(&request, a, "timestamp", .{ .integer = @intCast(now) });
    const signature = try key_pair.sign(message, null);
    try p.put(&request, a, "signature", w.text(try sol.base58(a, 64, signature.toBytes())));
    return request;
}
