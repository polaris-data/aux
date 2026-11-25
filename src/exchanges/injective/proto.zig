// Minimal protobuf wire codec for the selected Injective and Cosmos messages.
const std = @import("std");

pub const Encoder = struct {
    allocator: std.mem.Allocator,
    bytes: std.ArrayList(u8) = .empty,

    fn appendVarint(self: *Encoder, value: u64) !void {
        var n = value;
        while (n >= 128) : (n >>= 7) try self.bytes.append(self.allocator, @as(u8, @truncate(n)) | 0x80);
        try self.bytes.append(self.allocator, @intCast(n));
    }

    pub fn integer(self: *Encoder, field: u32, value: u64) !void {
        if (value == 0) return;
        try self.appendVarint(@as(u64, field) << 3);
        try self.appendVarint(value);
    }

    pub fn data(self: *Encoder, field: u32, value: []const u8) !void {
        if (value.len == 0) return;
        try self.appendVarint((@as(u64, field) << 3) | 2);
        try self.appendVarint(value.len);
        try self.bytes.appendSlice(self.allocator, value);
    }

    pub fn slice(self: Encoder) []const u8 {
        return self.bytes.items;
    }
};

pub const Reader = struct {
    bytes: []const u8,

    fn take(self: *Reader, size: usize) ![]const u8 {
        if (self.bytes.len < size) return error.InvalidProtobuf;
        const value = self.bytes[0..size];
        self.bytes = self.bytes[size..];
        return value;
    }

    fn varint(self: *Reader) !u64 {
        var result: u64 = 0;
        for (0..10) |i| {
            const b = (try self.take(1))[0];
            if (i == 9 and b > 1) return error.InvalidProtobuf;
            result |= @as(u64, b & 0x7f) << @as(u6, @intCast(i * 7));
            if (b < 128) return result;
        }
        return error.InvalidProtobuf;
    }

    pub fn next(self: *Reader) !?Field {
        if (self.bytes.len == 0) return null;
        const tag = try self.varint();
        const number = std.math.cast(u29, tag >> 3) orelse return error.InvalidProtobuf;
        if (number == 0) return error.InvalidProtobuf;
        return .{ .number = number, .value = switch (tag & 7) {
            0 => .{ .integer = try self.varint() },
            1 => .{ .fixed = try self.take(8) },
            2 => .{ .data = try self.take(std.math.cast(usize, try self.varint()) orelse return error.InvalidProtobuf) },
            5 => .{ .fixed = try self.take(4) },
            else => return error.InvalidProtobuf,
        } };
    }
};

pub const Field = struct {
    number: u29,
    value: union(enum) { integer: u64, data: []const u8, fixed: []const u8 },
};
