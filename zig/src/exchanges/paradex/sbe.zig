// Paradex schema 1:3. All reads honor the transmitted block lengths.
const std = @import("std");
const schema = @import("sbe_layout.zig");
const w = @import("../../core/wire.zig");

const Cursor = struct {
    rest: []const u8,
    fn take(c: *Cursor, count: usize) ![]const u8 {
        if (count > c.rest.len) return error.TruncatedFrame;
        const slice = c.rest[0..count];
        c.rest = c.rest[count..];
        return slice;
    }
    fn word(c: *Cursor) !u16 {
        return std.mem.readInt(u16, (try c.take(2))[0..2], .little);
    }
};

fn scaled(a: w.A, n: i64, places: u8) !w.V {
    const scale = std.math.pow(u64, 10, places);
    const abs = @abs(n);
    const fraction = try std.fmt.allocPrint(a, "{d}", .{abs % scale});
    const padding = try a.alloc(u8, places - fraction.len);
    @memset(padding, '0');
    return w.text(try std.fmt.allocPrint(a, "{s}{d}.{s}{s}", .{ if (n < 0) "-" else "", abs / scale, padding, fraction }));
}

fn value(a: w.A, f: schema.Field, raw: []const u8) !w.V {
    if (f.kind == .address) {
        const hex = try std.fmt.allocPrint(a, "{x}", .{raw});
        const digits = std.mem.trimStart(u8, hex, "0");
        return w.text(try std.mem.concat(a, u8, &.{ "0x", if (digits.len == 0) "0" else digits }));
    }
    var buf: [8]u8 = @splat(0);
    @memcpy(buf[0..raw.len], raw);
    const u = std.mem.readInt(u64, &buf, .little);
    const shift: u6 = @intCast((8 - raw.len) * 8);
    const n = @as(i64, @bitCast(u << shift)) >> shift;
    return switch (f.kind) {
        .address => unreachable,
        .signed => .{ .integer = n },
        .unsigned => .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{u}) },
        .optional => if (n == std.math.minInt(i64)) .null else .{ .integer = n },
        .decimal => if (n == std.math.minInt(i64)) (if (f.nullable) w.V.null else error.InvalidDecimal) else scaled(a, n, f.places),
        .label => blk: {
            if (u == 254) break :blk .null;
            for (f.values) |v| if (v.number == u) break :blk w.text(v.text);
            return error.InvalidEnum;
        },
        .flags => blk: {
            var remaining = u;
            var values: std.array_list.Managed(w.V) = .init(a);
            for (f.values) |v| {
                const bit = @as(u64, 1) << @intCast(v.number);
                if (remaining & bit != 0) try values.append(w.text(v.text));
                remaining &= ~bit;
            }
            if (remaining != 0) return error.InvalidFlags;
            break :blk .{ .array = values };
        },
    };
}

fn fixed(a: w.A, fields: []const schema.Field, block: []const u8) !w.V {
    var out: w.V = .{ .object = .empty };
    for (fields) |f| {
        if (f.offset + f.size > block.len) return error.TruncatedFrame;
        try out.object.put(a, f.name, try value(a, f, block[f.offset..][0..f.size]));
    }
    return out;
}

pub fn decode(a: w.A, input: w.V) !w.V {
    const hex = try w.string(try w.field(input, "data"));
    if (hex.len > 2 * 1024 * 1024) return error.FrameTooLarge;
    return decodeBytes(a, try @import("../../core/evm.zig").fromHex(a, hex));
}

pub fn decodeBytes(a: w.A, bytes: []const u8) !w.V {
    if (bytes.len > 1024 * 1024) return error.FrameTooLarge;
    var cursor = Cursor{ .rest = bytes };
    const block = try cursor.word();
    const template = try cursor.word();
    if (try cursor.word() != 1 or try cursor.word() != 3) return error.UnsupportedSchema;
    const layout = for (schema.layouts) |l| {
        if (l.id == template) break l;
    } else return error.UnknownTemplate;
    if (block < layout.minimum) return error.TruncatedFrame;
    var data = try fixed(a, layout.fields, try cursor.take(block));
    for (layout.variable) |v| {
        const item: w.V = if (v.minimum > 0) blk: {
            const stride: usize = try cursor.word();
            const count: usize = try cursor.word();
            if (stride < v.minimum or count > cursor.rest.len / stride) return error.TruncatedFrame;
            var entries: std.array_list.Managed(w.V) = .init(a);
            for (0..count) |_| try entries.append(try fixed(a, v.fields, try cursor.take(stride)));
            break :blk .{ .array = entries };
        } else blk: {
            const len = (try cursor.take(1))[0];
            const s = try cursor.take(len);
            if (!std.unicode.utf8ValidateSlice(s)) return error.InvalidText;
            break :blk w.text(s);
        };
        try data.object.put(a, v.name, item);
    }
    if (cursor.rest.len != 0 or (template == 1 and (try w.string(try w.field(data, "tradeIdStr"))).len == 0)) return error.MalformedFrame;
    var out: w.V = .{ .object = .empty };
    try out.object.put(a, "schema_id", .{ .integer = 1 });
    try out.object.put(a, "schema_version", .{ .integer = 3 });
    try out.object.put(a, "block_length", .{ .integer = block });
    try out.object.put(a, "template", w.text(try std.fmt.allocPrint(a, "{d}", .{template})));
    try out.object.put(a, "data", data);
    return out;
}
