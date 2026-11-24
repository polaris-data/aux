//! Extended account registration and wallet-derived Stark signatures.
const std = @import("std");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const st = @import("../../core/stark.zig");
const A = w.A;
const V = w.V;
const eq = std.mem.eql;
fn str(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn json(a: A, v: anytype) !V {
    return w.parse(a, try std.json.Stringify.valueAlloc(a, v, .{}));
}
fn signingDomain(input: V, r: V) ![]const u8 {
    const origin = input.object.get("origin") orelse .null;
    if (origin == .null) return "extended.exchange";
    const uri = try std.Uri.parse(try w.string(origin));
    const host = uri.host orelse return error.InvalidOrigin;
    const h = switch (host) {
        .raw, .percent_encoded => |s| s,
    };
    if (std.ascii.eqlIgnoreCase(h, "api.starknet.extended.exchange")) return "extended.exchange";
    if (std.ascii.eqlIgnoreCase(h, "api.starknet.sepolia.extended.exchange")) return "starknet.sepolia.extended.exchange";
    return str(r, "signingDomain");
}
pub fn grind(raw: []const u8) !u256 {
    var signature = try evm.hex(65, raw);
    defer std.crypto.secureZero(u8, &signature);
    const seed = std.mem.trimStart(u8, signature[0..32], &.{0});
    const order: u256 = st.order;
    const limit: u256 = @intCast(((@as(u257, 1) << 256) / order) * order);
    for (0..1024) |i| {
        var hasher = std.crypto.hash.sha2.Sha256.init(.{});
        hasher.update(if (seed.len == 0) &.{0} else seed);
        var counter: [2]u8 = undefined;
        std.mem.writeInt(u16, &counter, @intCast(i), .big);
        hasher.update(if (i < 256) counter[1..] else &counter);
        var bytes: [32]u8 = undefined;
        defer std.crypto.secureZero(u8, &bytes);
        hasher.final(&bytes);
        const value = std.mem.readInt(u256, &bytes, .big);
        if (value < limit) {
            const key = value % order;
            if (key == 0) return error.InvalidKey;
            return key;
        }
    }
    return error.InvalidKey;
}
fn scaled(value: []const u8, places: u8) !i64 {
    if (places > 18 or value.len == 0) return error.InvalidQuantity;
    const negative = value[0] == '-';
    const s = value[@intFromBool(negative)..];
    const dot = std.mem.findScalar(u8, s, '.') orelse s.len;
    if (dot == 0) return error.InvalidQuantity;
    const fraction = if (dot == s.len) 0 else s.len - dot - 1;
    if (fraction > places) return error.InvalidQuantity;
    var num: i64 = 0;
    for (s, 0..) |ch, i| {
        if (i == dot and ch == '.') continue;
        if (!std.ascii.isDigit(ch)) return error.InvalidQuantity;
        num = try std.math.add(i64, try std.math.mul(i64, num, 10), if (negative) -@as(i64, ch - '0') else @as(i64, ch - '0'));
    }
    for (fraction..places) |_| num = try std.math.mul(i64, num, 10);
    return num;
}
pub fn execute(a: A, input: V) !V {
    const kind = std.meta.stringToEnum(enum { onboarding_payload, key_derivation_payload, api_request_payload, onboarding_l2_hash, scale_decimal, derived_public_key, derived_sign }, try str(input, "operation")) orelse return error.InvalidOperation;
    const p = try w.field(input, "params");
    if (p != .object) return error.InvalidParameter;
    switch (kind) {
        .derived_sign, .derived_public_key => {
            var key = try grind(try str(p, "signature"));
            defer std.crypto.secureZero(u8, std.mem.asBytes(&key));
            if (kind == .derived_public_key) return w.text(try std.fmt.allocPrint(a, "0x{x}", .{try st.publicKey(key)}));
            const sig = try st.sign(key, try st.felt(try str(p, "digest")));
            return json(a, .{ try std.fmt.allocPrint(a, "{d}", .{sig[0]}), try std.fmt.allocPrint(a, "{d}", .{sig[1]}) });
        },
        .scale_decimal => return json(a, try scaled(try str(p, "value"), try w.unsigned(u8, try w.field(p, "decimals")))),
        .onboarding_l2_hash => return w.text(try std.fmt.allocPrint(a, "0x{x}", .{st.pedersen(try st.felt(try str(p, "wallet")), try st.felt(try str(p, "public_key")))})),
        .api_request_payload => {
            const path = try str(p, "path");
            const time = try str(p, "timestamp");
            if (!std.mem.startsWith(u8, path, "/") or std.mem.findAny(u8, path, "\r\n") != null or std.mem.findAny(u8, time, "\r\n") != null) return error.InvalidPath;
            const message = try std.mem.concat(a, u8, &.{ path, "@", time });
            const hash = evm.hash(try std.fmt.allocPrint(a, "\x19Ethereum Signed Message:\n{d}{s}", .{ message.len, message }));
            return json(a, .{ .digest = try std.fmt.allocPrint(a, "0x{x}", .{&hash}), .scheme = "evm_personal", .personal_message = message });
        },
        else => {},
    }
    const idx = try w.unsigned(u8, try w.field(p, "accountIndex"));
    if (idx > 127) return error.InvalidIndex;
    const primary: []const u8 = if (kind == .onboarding_payload) "AccountRegistration" else "AccountCreation";
    var message = V{ .object = .empty };
    var fields = std.json.Array.init(a);
    const names = [_][2][]const u8{ .{ "accountIndex", "int8" }, .{ "wallet", "address" }, .{ "tosAccepted", "bool" }, .{ "time", "string" }, .{ "action", "string" }, .{ "host", "string" } };
    for (names[0..if (kind == .onboarding_payload) @as(usize, 6) else 3]) |row| {
        try fields.append(try json(a, .{ .name = row[0], .type = row[1] }));
        try message.object.put(a, row[0], try w.field(p, row[0]));
    }
    var types = try w.parse(a, "{\"EIP712Domain\":[{\"name\":\"name\",\"type\":\"string\"}]}");
    try types.object.put(a, primary, .{ .array = fields });
    const td = try json(a, .{ .types = types, .domain = .{ .name = try signingDomain(input, p) }, .message = message, .primaryType = primary });
    return json(a, .{ .digest = try std.fmt.allocPrint(a, "0x{x}", .{&try @import("../../core/eip712.zig").hash(a, td)}), .scheme = "evm_typed", .typed_data = td });
}
