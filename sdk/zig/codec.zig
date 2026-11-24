const core = @import("aux_core");
const std = @import("std");

pub fn hexEncode(input: []const u8, output: []u8) ![]u8 {
    if (!core.encoding.aux_v1_hex_encode(input.ptr, input.len, output.ptr, output.len)) return error.NoSpaceLeft;
    return output[0 .. input.len * 2];
}
pub fn hexDecode(input: []const u8, output: []u8) ![]u8 {
    if (input.len % 2 != 0) return error.OddLength;
    if (output.len < input.len / 2) return error.NoSpaceLeft;
    var bad: usize = 0;
    return switch (core.encoding.aux_v1_hex_decode(input.ptr, input.len, output.ptr, input.len / 2, &bad)) {
        0 => output[0 .. input.len / 2],
        1 => error.OddLength,
        2 => error.NoSpaceLeft,
        else => error.InvalidHex,
    };
}
pub fn urlComponent(input: []const u8, output: []u8) ![]u8 {
    if (input.len > output.len / 3) return error.NoSpaceLeft;
    return output[0..core.encoding.aux_v1_url_component(input.ptr, input.len, output.ptr, output.len)];
}

/// Frame SSE from caller-supplied bytes. Drain next() after each push().
pub const EventDecoder = struct {
    handle: ?*core.hermes.Sse,

    pub fn init(limit: usize) !EventDecoder {
        if (limit == 0) return error.InvalidLimit;
        return .{ .handle = core.hermes.aux_v1_hermes_sse_create(limit) orelse return error.OutOfMemory };
    }
    pub fn push(self: EventDecoder, bytes: []const u8) !void {
        return switch (core.hermes.aux_v1_hermes_sse_push(self.handle orelse return error.Closed, bytes.ptr, bytes.len)) {
            .ok => {},
            .limit => error.EventTooLarge,
            .out_of_memory => error.OutOfMemory,
            else => error.InvalidFrame,
        };
    }
    /// Borrowed until the next decoder call.
    pub fn next(self: EventDecoder) !?[]const u8 {
        var view: core.hermes.View = .{};
        return switch (core.hermes.aux_v1_hermes_sse_next(self.handle orelse return error.Closed, &view)) {
            .ok => if (view.ptr) |ptr| ptr[0..view.len] else "",
            .need_input => null,
            .out_of_memory => error.OutOfMemory,
            .limit => error.EventTooLarge,
            else => error.InvalidFrame,
        };
    }
    pub fn finish(self: EventDecoder) !void {
        if (core.hermes.aux_v1_hermes_sse_finish(self.handle orelse return error.Closed) != .ok) return error.Truncated;
    }
    pub fn deinit(self: *EventDecoder) void {
        if (self.handle) |h| core.hermes.aux_v1_hermes_sse_destroy(h);
        self.handle = null;
    }
};

const rt = @import("runtime.zig");
pub fn decodeDbnRecord(client: rt.Client, bytes: []const u8, ts_out: bool) !rt.Owned(rt.Value) {
    const hex = try std.fmt.allocPrint(client.allocator, "{x}", .{bytes});
    defer client.allocator.free(hex);
    return client.invoke(.dbn_decode_record, .{ .data = hex, .ts_out = ts_out });
}
pub fn decodeSbe(client: rt.Client, bytes: []const u8) !rt.Owned(rt.Value) {
    const hex = try std.fmt.allocPrint(client.allocator, "{x}", .{bytes});
    defer client.allocator.free(hex);
    return client.invoke(.paradex_decode_sbe, .{ .data = hex });
}
