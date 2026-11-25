// Only the detailed ABI captures response data. Nested calls on the same
// thread restore their caller's destination when they return.
const std = @import("std");
const w = @import("wire.zig");
pub const Capture = struct { allocator: w.A, value: ?[]const u8 = null, stream_config: ?w.V = null };
pub threadlocal var current: ?*Capture = null;

pub fn record(venue: []const u8, status: u16, bytes: []const u8) void {
    const capture = current orelse return;
    const body = w.parse(capture.allocator, bytes) catch
        if (std.unicode.utf8ValidateSlice(bytes)) w.text(bytes) else w.V.null;
    capture.value = std.json.Stringify.valueAlloc(capture.allocator, .{ .venue = venue, .status = status, .body = body }, .{}) catch null;
}

fn redactCredentials(a: w.A, config: w.V, body: w.V) !w.V {
    if (config != .object) return body;
    var result = body;
    var it = config.object.iterator();
    while (it.next()) |entry| {
        const field = entry.key_ptr.*;
        const value = entry.value_ptr.*;
        if (value == .object) {
            result = try redactCredentials(a, value, result);
        } else if (value == .string and value.string.len != 0) {
            for ([_][]const u8{ "api_key", "api_secret", "access_token", "token", "key", "seed", "secret", "passphrase", "stark_key", "evm_key", "owner_key", "proxy_key", "cookie" }) |name| {
                if (std.mem.eql(u8, field, name)) {
                    result = try @import("redact.zig").redact(a, result, value.string);
                    break;
                }
            }
        }
    }
    return result;
}

/// A rejected HTTP stream or WebSocket upgrade can carry a venue error body.
/// Preserve the HTTP status even for an oversized or malformed body.
pub fn recordStreamResponse(response: *std.http.Client.Response, headers: []const std.http.Header) void {
    const capture = current orelse return;
    const config = capture.stream_config orelse return;
    const venue = w.string(w.field(config, "venue") catch return) catch return;
    const status = @backingInt(response.head.status);
    record(venue, status, "null");
    const limit = 64 * 1024;
    if (response.head.content_length) |count| {
        if (count > limit) return;
    } else if (response.head.keep_alive and response.head.transfer_encoding == .none) return;
    var buffer: [8192]u8 = undefined;
    var decompressor: std.http.Decompress = undefined;
    const a = capture.allocator;
    const reader = @import("transport.zig").bodyReader(a, response, &buffer, &decompressor) catch return;
    const bytes = reader.allocRemaining(a, .limited(limit)) catch return;
    const body = w.parse(a, bytes) catch if (std.unicode.utf8ValidateSlice(bytes)) w.text(bytes) else w.V.null;
    var safe = redactCredentials(a, config, body) catch return;
    for (headers) |header| {
        if (std.ascii.eqlIgnoreCase(header.name, "authorization")) {
            // Basic credentials are encoded before they reach the wire.
            const split = std.mem.findScalar(u8, header.value, ' ');
            const credential = if (split) |i| header.value[i + 1 ..] else header.value;
            safe = @import("redact.zig").redact(a, safe, credential) catch return;
        } else if (std.ascii.findIgnoreCase(header.name, "signature") != null) {
            safe = @import("redact.zig").redact(a, safe, header.value) catch return;
        }
    }
    capture.value = std.json.Stringify.valueAlloc(a, .{ .venue = venue, .status = status, .body = safe }, .{}) catch capture.value;
}
