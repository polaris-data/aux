// RFC 7692 raw DEFLATE with per-connection server context takeover.
const std = @import("std");
const gpa = std.heap.c_allocator;
const flate = std.compress.flate;

pub const Inflate = struct {
    history: []u8,
    size: usize = 0,
    no_takeover: bool = false,

    pub fn reset(self: *Inflate, no_takeover: bool) void {
        self.size = 0;
        self.no_takeover = no_takeover;
    }

    pub fn message(self: *Inflate, allocator: std.mem.Allocator, bytes: []const u8, limit: usize) ![]u8 {
        var arena = std.heap.ArenaAllocator.init(gpa);
        defer arena.deinit();
        const a = arena.allocator();

        // A non-final stored block restores the prior 32 KiB dictionary. The
        // message starts at a block boundary after the prior sync flush.
        const size: u16 = @intCast(self.size);
        var prefix: [5]u8 = @splat(0);
        std.mem.writeInt(u16, prefix[1..3], size, .little);
        std.mem.writeInt(u16, prefix[3..5], ~size, .little);
        const trailer = [_]u8{ 0, 0, 255, 255, 1, 0, 0, 255, 255 };
        const input = try std.mem.concat(a, u8, &.{ &prefix, self.history[0..self.size], bytes, &trailer });
        var reader = std.Io.Reader.fixed(input);
        var decoder = flate.Decompress.init(&reader, .raw, try a.alloc(u8, flate.max_window_len));
        const output = decoder.reader.allocRemaining(a, .limited(limit + self.size)) catch return error.InvalidCompressedFrame;
        if (output.len < self.size or reader.seek != reader.end) return error.InvalidCompressedFrame;
        const message_bytes = try allocator.dupe(u8, output[self.size..]);
        if (!self.no_takeover) {
            self.size = @min(self.history.len, output.len);
            @memcpy(self.history[0..self.size], output[output.len - self.size ..]);
        }
        return message_bytes;
    }
};
