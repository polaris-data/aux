const std = @import("std");
const aux = @import("aux");

pub fn compare(actual: aux.Value, wire: aux.Value) anyerror!void {
    switch (wire) {
        .object => |obj| {
            if (actual != .object) return error.ExpectedObject;
            var it = obj.iterator();
            while (it.next()) |kv| {
                const field = actual.object.get(kv.key_ptr.*) orelse {
                    if (kv.value_ptr.* == .null) continue;
                    return error.MissingWireField;
                };
                try compare(field, kv.value_ptr.*);
            }
            var fields = actual.object.iterator();
            while (fields.next()) |kv| {
                if (obj.contains(kv.key_ptr.*)) continue;
                if (std.mem.eql(u8, kv.key_ptr.*, "pending") and obj.contains("provider") and kv.value_ptr.* == .bool and !kv.value_ptr.bool) continue;
                return error.UnexpectedWireField;
            }
        },
        .array => |arr| {
            if (actual != .array or actual.array.items.len != arr.items.len) return error.UnexpectedArray;
            for (actual.array.items, arr.items) |a, w| try compare(a, w);
        },
        .number_string => |s| {
            if (actual != .number_string or !std.mem.eql(u8, s, actual.number_string)) return error.ChangedNumber;
        },
        .string => |s| if (actual != .string or !std.mem.eql(u8, s, actual.string)) return error.ChangedString,
        .null => if (actual != .null) return error.ExpectedNull,
        .bool => |b| if (actual != .bool or b != actual.bool) return error.ChangedBool,
        else => unreachable,
    }
}
