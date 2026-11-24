const std = @import("std");
const Value = std.json.Value;
/// Preserve the JSON types accepted by the shared serde schemas. Zig's default
/// JSON decoder also accepts quoted integers and arrays of bytes for strings.
pub fn decode(comptime T: type, a: std.mem.Allocator, v: Value, options: std.json.ParseOptions) std.json.ParseFromValueError!T {
    if (T == Value) return v;
    switch (@typeInfo(T)) {
        .int => return switch (v) {
            .integer => |n| std.math.cast(T, n) orelse error.Overflow,
            .number_string => |s| std.fmt.parseInt(T, s, 10),
            else => error.UnexpectedToken,
        },
        .float => if (v != .integer and v != .float and v != .number_string) return error.UnexpectedToken,
        .@"enum" => if (v != .string) return error.UnexpectedToken,
        .optional => |opt| return if (v == .null) null else try decode(opt.child, a, v, options),
        .pointer => |ptr| switch (ptr.size) {
            .one => {
                const item = try a.create(ptr.child);
                item.* = try decode(ptr.child, a, v, options);
                return item;
            },
            .slice => {
                if (ptr.child == u8) return if (v == .string) v.string else error.UnexpectedToken;
                if (v != .array) return error.UnexpectedToken;
                const items = try a.alloc(ptr.child, v.array.items.len);
                for (items, v.array.items) |*item, raw| item.* = try decode(ptr.child, a, raw, options);
                return items;
            },
            else => {},
        },
        .array => |arr| {
            if (v != .array or v.array.items.len != arr.len) return error.LengthMismatch;
            var items: T = undefined;
            for (&items, v.array.items) |*item, raw| item.* = try decode(arr.child, a, raw, options);
            return items;
        },
        else => {},
    }
    return std.json.innerParseFromValue(T, a, v, options);
}

pub const Mode = enum { required, present, optional, default, zero, nullable, nullable_vec };

/// Schema-directed serde behavior shared by every generated model.
pub fn Codec(comptime T: type, comptime modes: []const Mode, comptime open: bool) type {
    const names = std.meta.fieldNames(T)[0..modes.len];
    return struct {
        pub fn read(a: std.mem.Allocator, object: Value, options: std.json.ParseOptions) std.json.ParseFromValueError!T {
            if (object != .object) return error.UnexpectedToken;
            var result: T = undefined;
            if (open) result.extra = .empty;
            inline for (names, modes, 0..) |name, mode, i| {
                const F = @FieldType(T, name);
                if (object.object.get(name)) |v| {
                    @field(result, name) = if (mode == .nullable_vec and v == .null) &.{} else if (mode == .nullable)
                        @as(F, try decode(@typeInfo(F).optional.child, a, v, options))
                    else
                        try decode(F, a, v, options);
                } else if (mode == .required or mode == .present) return error.MissingField else {
                    @field(result, name) = @typeInfo(T).@"struct".field_attrs[i].defaultValue(F).?;
                }
            }
            var it = object.object.iterator();
            while (it.next()) |kv| {
                inline for (names) |name| {
                    if (std.mem.eql(u8, kv.key_ptr.*, name)) break;
                } else {
                    if (!open) return error.UnknownField;
                    try result.extra.put(a, kv.key_ptr.*, kv.value_ptr.*);
                }
            }
            return result;
        }
        pub fn write(self: T, writer: anytype) !void {
            try writer.beginObject();
            inline for (names, modes) |name, mode| {
                const v = @field(self, name);
                if (@typeInfo(@TypeOf(v)) != .optional or mode == .present or v != null) {
                    try writer.objectField(name);
                    try writer.write(v);
                }
            }
            if (open) {
                var it = self.extra.iterator();
                while (it.next()) |kv| {
                    // Declared fields always own their wire names.
                    inline for (names) |name| {
                        if (std.mem.eql(u8, kv.key_ptr.*, name)) break;
                    } else {
                        try writer.objectField(kv.key_ptr.*);
                        try writer.write(kv.value_ptr.*);
                    }
                }
            }
            try writer.endObject();
        }
    };
}

pub fn Untagged(comptime T: type) type {
    return struct {
        pub fn read(a: std.mem.Allocator, value: Value, options: std.json.ParseOptions) std.json.ParseFromValueError!T {
            inline for (comptime std.meta.fieldNames(T)) |name| {
                if (decode(@FieldType(T, name), a, value, options)) |v|
                    return @unionInit(T, name, v)
                else |err| if (err == error.OutOfMemory) return err;
            }
            return error.UnexpectedToken;
        }
        pub fn write(self: T, writer: anytype) !void {
            switch (self) {
                inline else => |v| try writer.write(v),
            }
        }
    };
}

pub const Number = struct {
    text: []const u8 = "0",
    pub fn jsonParseFromValue(a: std.mem.Allocator, v: Value, _: std.json.ParseOptions) std.json.ParseFromValueError!Number {
        return .{ .text = switch (v) {
            .number_string => |s| s,
            .integer, .float => try std.json.Stringify.valueAlloc(a, v, .{}),
            else => return error.UnexpectedToken,
        } };
    }
    pub fn jsonStringify(self: Number, writer: anytype) !void {
        try writer.write(Value{ .number_string = self.text });
    }
};
pub const NativeNumber = union(enum) {
    number: Number,
    string: []const u8,
    pub const jsonStringify = Untagged(@This()).write;
    pub const jsonParseFromValue = Untagged(@This()).read;
};
