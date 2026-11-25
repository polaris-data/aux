const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const Request = @import("../../core/http.zig").Request;

pub fn sign(a: p.A, config: p.V, request: *Request, target: []const u8, body: []const u8, now: u64) !void {
    const auth = p.get(config, "auth");
    const jwt = p.get(auth, "token");
    if (jwt != .null) {
        const authorization = try std.mem.concat(a, u8, &.{ "Bearer ", try p.credential(jwt) });
        return p.put(&request.headers, a, "Authorization", w.text(authorization));
    }
    const key = try p.credential(p.get(config, "api_key"));
    const secret = try p.credential(p.get(config, "api_secret"));
    const timestamp = try std.fmt.allocPrint(a, "{d}", .{now});
    const Hmac = std.crypto.auth.hmac.sha2.HmacSha256;
    var mac = Hmac.init(secret);
    for ([_][]const u8{ timestamp, request.method, target, body }) |part| mac.update(part);
    var digest: [Hmac.mac_length]u8 = undefined;
    mac.final(&digest);
    try p.put(&request.headers, a, "ONDO-KEY-ID", w.text(key));
    try p.put(&request.headers, a, "ONDO-TIMESTAMP", w.text(timestamp));
    try p.put(&request.headers, a, "ONDO-SIGN", w.text(try std.fmt.allocPrint(a, "{x}", .{&digest})));
}
