//! Spline configuration and fixed-layout price updates.
const std = @import("std");
const w = @import("../../core/wire.zig");
const Ix = @import("instructions.zig").Instruction;
const rows = @import("orders.zig").array;
const Kind = enum { update_spline_price, update_spline_price_with_ordering, update_spline_parameters_with_ordering, update_spline_price_with_metadata, update_spline_position_limits_config };

pub fn flag(p: w.V, name: []const u8) !bool {
    const value = try w.field(p, name);
    if (value != .bool) return error.InvalidBoolean;
    return value.bool;
}
pub fn optionalValue(comptime T: type, p: w.V, name: []const u8) !?T {
    if (p != .object) return error.InvalidInstruction;
    const v = p.object.get(name) orelse .null;
    return if (v == .null) null else try w.unsigned(T, v);
}
fn meta(ix: *Ix, p: w.V) !void {
    const slot = try optionalValue(u64, p, "user_update_slot");
    const bid = try optionalValue(u32, p, "bid_width");
    const ask = try optionalValue(u32, p, "ask_width");
    const flags: std.bit_set.IntegerBitSet(5) = blk: {
        var bits = std.bit_set.IntegerBitSet(5).empty;
        bits.setValue(0, try flag(p, "refresh_regions"));
        bits.setValue(1, try flag(p, "override_sequence_number"));
        bits.setValue(2, slot != null);
        bits.setValue(3, bid != null);
        bits.setValue(4, ask != null);
        break :blk bits;
    };
    try ix.quantity(u32, p, "new_mid_price");
    try ix.num(u32, flags.mask);
    try ix.quantity(u64, p, "user_sequence_number");
    try ix.num(u64, slot orelse 0);
    try ix.num(u32, bid orelse 0);
    try ix.num(u32, ask orelse 0);
    try ix.bytes(try w.field(p, "client_order_id"), 16);
}
pub fn build(ix: *Ix, name: []const u8) !bool {
    const kind = std.meta.stringToEnum(Kind, name) orelse return false;
    const p = try w.field(ix.p, "params");
    if (p != .object) return error.InvalidInstruction;
    ix.p = try w.field(ix.p, "accounts");
    try ix.logAccounts();
    try ix.field("signer", .signer);
    try ix.field("trader_account", .readonly);
    try ix.field("spline_collection", .writable);
    try ix.discriminator(switch (kind) {
        .update_spline_price_with_ordering => "update_spline_price",
        .update_spline_parameters_with_ordering => "update_spline_parameters",
        else => name,
    });
    switch (kind) {
        .update_spline_price_with_metadata => try meta(ix, p),
        .update_spline_position_limits_config => {
            const val = p.object.get("max_position_size") orelse .null;
            try ix.num(u8, @intFromBool(val != .null));
            if (val != .null) {
                if (val == .string and std.mem.eql(u8, val.string, "Disabled")) {
                    try ix.num(u8, 0);
                } else {
                    const limit = try w.field(val, "Limit");
                    try ix.num(u8, 1);
                    try ix.quantity(u32, limit, "long");
                    try ix.quantity(u32, limit, "short");
                }
            }
            try ix.option(u32, p, "leverage_decrease_in_bps");
        },
        else => {
            if (kind == .update_spline_parameters_with_ordering) {
                for ([_][]const u8{ "bid_regions", "ask_regions" }) |name_| {
                    const regions = try rows(try w.field(p, name_));
                    try ix.num(u32, @intCast(regions.len));
                    for (regions) |r| {
                        try ix.quantity(u64, r, "start_offset");
                        try ix.quantity(u64, r, "end_offset");
                        try ix.quantity(u32, r, "density");
                        try ix.quantity(u32, r, "top_level_hidden_take_size");
                        try ix.quantity(u64, r, "lifespan");
                    }
                }
            } else {
                try ix.quantity(u64, p, "new_mid_price");
                try ix.option(u64, p, "user_update_slot");
            }
            try ix.boolean(p, "refresh_regions");
            if (kind != .update_spline_price) {
                try ix.quantity(u64, p, "user_sequence_number");
                try ix.bytes(try w.field(p, "client_order_id"), 16);
                try ix.boolean(p, "override_sequence_number");
            }
        },
    }
    return true;
}
