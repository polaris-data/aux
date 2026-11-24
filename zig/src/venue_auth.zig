// Operation-specific authorization for Paradex and Polymarket Perps.
const std = @import("std");
const w = @import("wire.zig");
const unsigned = w.unsigned;

pub fn routeKind(comptime table: []const u8, operation: []const u8) ![]const u8 {
    var lines = std.mem.tokenizeScalar(u8, table, '\n');
    while (lines.next()) |line| {
        const n = std.mem.findScalar(u8, line, ' ') orelse unreachable;
        if (std.mem.eql(u8, line[0..n], operation)) return line[n + 1 ..];
    }
    return error.AuthenticationRequired;
}

pub fn hasPerpsSigner(operation: []const u8) bool {
    const kind = routeKind(@embedFile("polymarket-perps-routes.txt"), operation) catch return false;
    return !std.mem.eql(u8, kind, "proxy");
}

pub fn paradex(a: w.A, headers: *w.V, operation: []const u8, auth: w.V, seconds: i64) !void {
    const scope = try routeKind(@embedFile("paradex-bearer-routes.txt"), operation);
    if (auth != .object) return error.AuthenticationRequired;
    const read_only = auth.object.get("read_only") orelse w.V{ .bool = true };
    if (read_only != .bool or (read_only.bool and std.mem.eql(u8, scope, "write"))) return error.AuthenticationRequired;
    const token = try bearer(a, auth, seconds);
    try headers.object.put(a, "Authorization", w.text(try std.mem.concat(a, u8, &.{ "Bearer ", token })));
}

pub fn bearer(a: w.A, auth: w.V, seconds: i64) ![]const u8 {
    if (auth != .object) return error.AuthenticationRequired;
    const token = w.string(auth.object.get("token") orelse return error.AuthenticationRequired) catch return error.AuthenticationRequired;
    for (token) |b| if (b < 0x21 or b > 0x7e) return error.AuthenticationRequired;
    var parts = std.mem.splitScalar(u8, token, '.');
    _ = parts.next();
    const payload = parts.next() orelse return error.AuthenticationRequired;
    const decoder = std.base64.url_safe_no_pad.Decoder;
    const n = decoder.calcSizeForSlice(payload) catch return error.AuthenticationRequired;
    const bytes = try a.alloc(u8, n);
    decoder.decode(bytes, payload) catch return error.AuthenticationRequired;
    const claims = w.parse(a, bytes) catch return error.AuthenticationRequired;
    const exp = w.field(claims, "exp") catch return error.AuthenticationRequired;
    if (seconds < 0 or exp != .number_string or (unsigned(u64, exp) catch return error.AuthenticationRequired) <= @as(u64, @intCast(seconds))) return error.AuthenticationRequired;
    return token;
}

pub fn proxyHeaders(a: w.A, headers: *w.V, auth: w.V, now: u64) !void {
    if (auth != .object) return error.AuthenticationRequired;
    if (auth.object.get("expires_at_ms")) |value| {
        if (now >= try unsigned(u64, value)) return error.AuthenticationRequired;
    }
    const secret = try w.string(auth.object.get("secret") orelse return error.AuthenticationRequired);
    if (secret.len == 0) return error.AuthenticationRequired;
    for (secret) |b| if (b < 0x20 or b > 0x7e) return error.AuthenticationRequired;
    const proxy = try @import("clob_auth.zig").checksum(a, try w.string(auth.object.get("proxy") orelse return error.AuthenticationRequired));
    try headers.object.put(a, "POLYMARKET-PROXY", w.text(proxy));
    try headers.object.put(a, "POLYMARKET-SECRET", w.text(secret));
}

// Extended and Phoenix accept independent API-key and bearer headers.
pub const HeaderCredentials = struct {
    key: ?[]const u8,
    bearer_token: ?[]const u8,
    expires: ?u64,

    fn optional(v: ?w.V) !?[]const u8 {
        const value = v orelse return null;
        if (value == .null) return null;
        const bytes = try w.string(value);
        if (bytes.len == 0) return error.InvalidCredential;
        for (bytes) |b| if (b < 32 or b > 126) return error.InvalidCredential;
        return bytes;
    }
    pub fn parse(a: w.A, input: w.V) !HeaderCredentials {
        const auth = input.object.get("auth") orelse w.V.null;
        if (auth != .object and auth != .null) return error.InvalidCredential;
        const token = try optional(if (auth == .object) auth.object.get("access_token") else null);
        const expiry = if (auth == .object) auth.object.get("expires_at") orelse w.V.null else w.V.null;
        return .{
            .key = try optional(input.object.get("api_key")),
            .bearer_token = if (token) |t| try std.mem.concat(a, u8, &.{ "Bearer ", t }) else null,
            .expires = if (expiry == .null) null else try w.unsigned(u64, expiry),
        };
    }
    pub fn checkExpiry(expires: ?u64, io: std.Io) !void {
        if (expires) |deadline| if (std.Io.Timestamp.now(io, .real).toSeconds() >= @as(i128, deadline)) return error.AuthenticationRequired;
    }
};
