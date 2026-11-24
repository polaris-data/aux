// Normalize the five record families whose wire layout changed before DBN v3.
const std = @import("std");
const w = @import("../../core/wire.zig");
const old = @import("dbn_legacy_layout.zig");
const current = @import("dbn_layout.zig");
const eq = std.mem.eql;
fn errorCode(s: []const u8) u8 {
    if (eq(u8, s, "User or API key deactivated")) return 2;
    if (eq(u8, s, "User has reached their open connection limit")) return 3;
    if (std.mem.startsWith(u8, s, "Failed to resolve symbol")) return 4;
    if (eq(u8, s, "Internal error")) return 6;
    if (std.mem.startsWith(u8, s, "Slow client detected for ")) return 7;
    return 255;
}
fn systemCode(s: []const u8) u8 {
    if (eq(u8, s, "Heartbeat")) return 0;
    if (std.mem.startsWith(u8, s, "Subscription request ") and std.mem.endsWith(u8, s, " succeeded")) return 1;
    if (std.mem.startsWith(u8, s, "Warning: slow reading")) return 2;
    if (std.mem.startsWith(u8, s, "Finished ") and std.mem.endsWith(u8, s, " replay")) return 3;
    if (std.mem.startsWith(u8, s, "End of interval for ")) return 4;
    return 255;
}
fn convert(comptime Source: type, comptime Target: type, a: w.A, data: []const u8, ts_out: bool) !w.V {
    const trailer: usize = if (ts_out) 8 else 0;
    if (data.len != @sizeOf(Source) + trailer) return error.InvalidRecordLength;
    var bytes: [@sizeOf(Target) + 8]u8 = @splat(0);
    inline for (@typeInfo(Target).@"struct".field_names) |name| {
        if (comptime @hasField(Source, name) and !std.mem.startsWith(u8, name, "_")) {
            const n = @min(@sizeOf(@FieldType(Source, name)), @sizeOf(@FieldType(Target, name)));
            const from = @offsetOf(Source, name);
            const to = @offsetOf(Target, name);
            @memcpy(bytes[to..][0..n], data[from..][0..n]);
        }
    }
    // Newly introduced fields have protocol-defined sentinels. The v1/v2
    // instrument ID widens unsigned, already handled by the zero extension.
    if (Target == current.InstrumentDefMsg) {
        for ([_]usize{ @offsetOf(Target, "leg_price"), @offsetOf(Target, "leg_delta") }) |i| std.mem.writeInt(i64, bytes[i..][0..8], std.math.maxInt(i64), .little);
        bytes[@offsetOf(Target, "leg_side")] = 'N';
    } else if (Target == current.StatMsg) {
        const quantity = std.mem.readInt(i32, data[@offsetOf(Source, "quantity")..][0..4], .little);
        std.mem.writeInt(i64, bytes[@offsetOf(Target, "quantity")..][0..8], if (quantity == std.math.maxInt(i32)) std.math.maxInt(i64) else quantity, .little);
    } else if (Target == current.ErrorMsg) {
        bytes[@offsetOf(Target, "code")] = errorCode(std.mem.sliceTo(bytes[@offsetOf(Target, "err")..][0..64], 0));
        bytes[@offsetOf(Target, "is_last")] = 255;
    } else if (Target == current.SystemMsg) {
        bytes[@offsetOf(Target, "code")] = systemCode(std.mem.sliceTo(bytes[@offsetOf(Target, "msg")..][0..64], 0));
    } else if (Target == current.SymbolMappingMsg) {
        bytes[@offsetOf(Target, "stype_in")] = 255;
        bytes[@offsetOf(Target, "stype_out")] = 255;
    }
    const n = @sizeOf(Target) + trailer;
    bytes[0] = @intCast(n / 4);
    @memcpy(bytes[@sizeOf(Target)..n], data[@sizeOf(Source)..]);
    return @import("dbn.zig").decodeBytes(a, bytes[0..n], ts_out);
}
pub fn decode(a: w.A, version: u8, data: []const u8, ts_out: bool) !w.V {
    if (version < 3) switch (data[1]) {
        19 => return if (version == 1) convert(old.InstrumentDefMsgV1, current.InstrumentDefMsg, a, data, ts_out) else convert(old.InstrumentDefMsgV2, current.InstrumentDefMsg, a, data, ts_out),
        20 => {},
        21 => if (version == 1) return convert(old.ErrorMsgV1, current.ErrorMsg, a, data, ts_out),
        22 => if (version == 1) return convert(old.SymbolMappingMsgV1, current.SymbolMappingMsg, a, data, ts_out),
        23 => if (version == 1) return convert(old.SystemMsgV1, current.SystemMsg, a, data, ts_out),
        24 => return convert(old.StatMsgV1, current.StatMsg, a, data, ts_out),
        else => {},
    };
    return @import("dbn.zig").decodeBytes(a, data, ts_out);
}
