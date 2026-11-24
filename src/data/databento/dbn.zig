// Decode a single DBN v3 record. Bytes are unaligned and little-endian on wire.
const std = @import("std");
const w = @import("../../core/wire.zig");
const layout = @import("dbn_layout.zig");
fn read(comptime T: type, a: w.A, data: []const u8) anyerror!w.V {
    if (T == layout.Char) {
        if (data[0] == 0) return .null;
        if (data[0] > 127) return error.InvalidCharacter;
        return w.text(data[0..1]);
    }
    switch (@typeInfo(T)) {
        .int => |int| {
            const n = std.mem.readInt(T, data[0..@sizeOf(T)], .little);
            return if (int.bits == 64) w.text(try std.fmt.allocPrint(a, "{d}", .{n})) else .{ .integer = n };
        },
        .array => |arr| {
            if (arr.child == u8) {
                const bytes = std.mem.sliceTo(data[0..arr.len], 0);
                return w.text(if (std.unicode.utf8ValidateSlice(bytes)) bytes else "");
            }
            var values: std.array_list.Managed(w.V) = .init(a);
            for (0..arr.len) |i| try values.append(try read(arr.child, a, data[i * @sizeOf(arr.child) ..]));
            return .{ .array = values };
        },
        .@"struct" => |s| {
            var object: w.V = .{ .object = .empty };
            inline for (s.field_names, s.field_types) |name, Field| {
                if (comptime !std.mem.startsWith(u8, name, "_") and !(T == layout.RecordHeader and std.mem.eql(u8, name, "length"))) {
                    try object.object.put(a, name, try read(Field, a, data[@offsetOf(T, name)..]));
                }
            }
            return object;
        },
        else => @compileError("unknown DBN field type"),
    }
}
fn record(comptime T: type, a: w.A, bytes: []const u8, ts_out: bool) !w.V {
    if (bytes.len != @sizeOf(T) + @as(usize, if (ts_out) 8 else 0)) return error.InvalidRecordLength;
    var out = try read(T, a, bytes);
    if (ts_out) try out.object.put(a, "ts_out", try read(u64, a, bytes[@sizeOf(T)..]));
    return out;
}
pub fn decode(a: w.A, input: w.V) !w.V {
    const hex = try w.string(try w.field(input, "data"));
    if (hex.len > 2040) return error.InvalidRecordLength;
    const bytes = try @import("../../core/evm.zig").fromHex(a, hex);
    const flag = try w.field(input, "ts_out");
    if (flag != .bool) return error.InvalidRecord;
    return decodeBytes(a, bytes, flag.bool);
}
pub fn decodeBytes(a: w.A, bytes: []const u8, ts_out: bool) !w.V {
    if (bytes.len < 16 or @as(usize, bytes[0]) * 4 != bytes.len) return error.InvalidRecord;
    return switch (bytes[1]) {
        0 => record(layout.TradeMsg, a, bytes, ts_out),
        1 => record(layout.Mbp1Msg, a, bytes, ts_out),
        10 => record(layout.Mbp10Msg, a, bytes, ts_out),
        18 => record(layout.StatusMsg, a, bytes, ts_out),
        19 => record(layout.InstrumentDefMsg, a, bytes, ts_out),
        20 => record(layout.ImbalanceMsg, a, bytes, ts_out),
        21 => record(layout.ErrorMsg, a, bytes, ts_out),
        22 => record(layout.SymbolMappingMsg, a, bytes, ts_out),
        23 => record(layout.SystemMsg, a, bytes, ts_out),
        24 => record(layout.StatMsg, a, bytes, ts_out),
        17, 32...36 => record(layout.OhlcvMsg, a, bytes, ts_out),
        160 => record(layout.MboMsg, a, bytes, ts_out),
        177, 194 => record(layout.Cmbp1Msg, a, bytes, ts_out),
        192, 193 => record(layout.CbboMsg, a, bytes, ts_out),
        195, 196 => record(layout.BboMsg, a, bytes, ts_out),
        else => error.UnsupportedRecord,
    };
}
