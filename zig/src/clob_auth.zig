// Polymarket CLOB L2 HMAC covers the request path and exact serialized body.
const std = @import("std");
const w = @import("wire.zig");
const A = w.A;
const V = w.V;
pub fn supported(op: []const u8) bool {
    var routes = std.mem.tokenizeScalar(u8, @embedFile("polymarket-l2-routes.txt"), '\n');
    while (routes.next()) |name| if (std.mem.eql(u8, name, op)) return true;
    return false;
}
fn field(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn decode(a: A, s: []const u8) ![]u8 {
    if (s.len == 0) return error.InvalidCredential;
    for ([_]std.base64.Codecs{ std.base64.url_safe, std.base64.url_safe_no_pad, std.base64.standard }) |codec| {
        const n = codec.Decoder.calcSizeForSlice(s) catch continue;
        const bytes = try a.alloc(u8, n);
        if (codec.Decoder.decode(bytes, s)) |_| {
            if (bytes.len == 0) return error.InvalidCredential;
            return bytes;
        } else |_| std.crypto.secureZero(u8, bytes);
    }
    return error.InvalidCredential;
}
pub fn checksum(a: A, addr: []const u8) ![]const u8 {
    const evm = @import("evm.zig");
    const b = try evm.fromHex(a, addr);
    if (b.len != 20) return error.InvalidAddress;
    const hex = try std.fmt.allocPrint(a, "0x{x}", .{b});
    const hash = evm.hash(hex[2..]);
    for (hex[2..], 0..) |*c, i| if ((if (i % 2 == 0) hash[i / 2] >> 4 else hash[i / 2] & 15) >= 8) {
        c.* = std.ascii.toUpper(c.*);
    };
    return hex;
}
pub fn removeAuthHeaders(a: A, h: *V) !void {
    var existing: std.ArrayList([]const u8) = .empty;
    var it = h.object.iterator();
    while (it.next()) |entry| {
        const name = entry.key_ptr.*;
        if (std.ascii.startsWithIgnoreCase(name, "poly_") or std.ascii.startsWithIgnoreCase(name, "relayer_api_key")) try existing.append(a, name);
    }
    for (existing.items) |name| _ = h.object.swapRemove(name);
}

fn hmacHeaders(a: A, h: *V, credentials: V, comptime prefix: []const u8, method: []const u8, path: []const u8, body: []const u8, ts: []const u8) !void {
    const key = try field(credentials, "key");
    const pass = try field(credentials, "passphrase");
    if (key.len == 0) return error.InvalidCredential;
    for ([_][]const u8{ key, pass }) |value| for (value) |c| if (c < 32 or c > 126) return error.InvalidHeader;
    const secret = try decode(a, try field(credentials, "secret"));
    defer std.crypto.secureZero(u8, secret);
    var mac = std.crypto.auth.hmac.sha2.HmacSha256.init(secret);
    mac.update(ts);
    mac.update(method);
    mac.update(path);
    mac.update(body);
    var digest: [32]u8 = undefined;
    mac.final(&digest);
    const enc = std.base64.url_safe.Encoder;
    const signature = enc.encode(try a.alloc(u8, enc.calcSize(digest.len)), &digest);
    try h.object.put(a, prefix ++ "_TIMESTAMP", w.text(ts));
    try h.object.put(a, prefix ++ "_API_KEY", w.text(key));
    try h.object.put(a, prefix ++ "_PASSPHRASE", w.text(pass));
    try h.object.put(a, prefix ++ "_SIGNATURE", w.text(signature));
}

pub fn headers(a: A, h: *V, credentials: V, method: []const u8, path: []const u8, body: []const u8, seconds: u64) !void {
    try removeAuthHeaders(a, h);
    try h.object.put(a, "POLY_ADDRESS", w.text(try checksum(a, try field(credentials, "address"))));
    try hmacHeaders(a, h, credentials, "POLY", method, path, body, try std.fmt.allocPrint(a, "{d}", .{seconds}));
}

pub fn serviceHeaders(a: A, h: *V, config: V, kind: []const u8, method: []const u8, path: []const u8, body: []const u8, seconds: u64) !void {
    const service = config.object.get("services") orelse V{ .object = .empty };
    if (service != .object) return error.InvalidCredential;
    const builder = service.object.get("builder");
    const ts = try std.fmt.allocPrint(a, "{d}", .{seconds});
    if (std.mem.eql(u8, kind, "relayer")) {
        try removeAuthHeaders(a, h);
        if (service.object.get("relayer")) |creds| {
            const key = try field(creds, "key");
            if (key.len == 0) return error.InvalidCredential;
            for (key) |c| if (c < 32 or c > 126) return error.InvalidHeader;
            try h.object.put(a, "RELAYER_API_KEY", w.text(key));
            try h.object.put(a, "RELAYER_API_KEY_ADDRESS", w.text(try checksum(a, try field(creds, "address"))));
        } else try hmacHeaders(a, h, builder orelse return error.AuthenticationRequired, "POLY_BUILDER", method, path, body, ts);
        return;
    }
    try headers(a, h, config.object.get("clob") orelse return error.AuthenticationRequired, method, path, body, seconds);
    if (std.mem.eql(u8, kind, "gateway") and !std.mem.eql(u8, method, "GET")) {
        if (builder) |b| try hmacHeaders(a, h, b, "POLY_BUILDER", method, path, body, ts);
    }
}

test "CLOB reference POST signature preserves the body and padding" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const credentials = try w.parse(a,
        \\{"address":"0x7e5f4552091a69125d5dfcb7b8c2659029395bdf","key":"test","passphrase":"pass","secret":"cHVibGljLXRlc3Qtc2VjcmV0"}
    );
    var h: V = .{ .object = .empty };
    try headers(a, &h, credentials, "POST", "/order", "{\"price\":\"0.550000\",\"size\":\"10\"}", 1731536000);
    try std.testing.expectEqualStrings("YnJIqeRTYC12GTuysXh19APpp3yzt-7l_h4M5c5sTNs=", try field(h, "POLY_SIGNATURE"));
    try std.testing.expectEqualStrings("0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf", try field(h, "POLY_ADDRESS"));
}
