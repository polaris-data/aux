//! Conditional-order and legacy stop-loss encoders.
const std = @import("std");
const w = @import("../../core/wire.zig");
const b = @import("instructions.zig");
const o = @import("orders.zig");
const sol = @import("../../core/solana.zig");
const Ix = b.Instruction;
const Kind = enum { cancel_stop_loss, place_stop_loss, create_conditional_orders_account, cancel_conditional_order, place_position_conditional_order, place_attached_conditional_order, place_limit_order_with_conditionals };

fn opt(p: w.V, name: []const u8) w.V {
    return p.object.get(name) orelse .null;
}
fn triggers(ix: *Ix) !void {
    var count: usize = 0;
    for ([_][]const u8{ "greater_trigger_order", "less_trigger_order" }, 0..) |name, index| {
        const val = opt(ix.p, name);
        try ix.num(u8, @intFromBool(val != .null));
        if (val == .null) continue;
        const direction = try o.tag(try w.field(val, "trigger_direction"), &.{ "GreaterThan", "LessThan" });
        if (direction != index) return error.InvalidTriggerDirection;
        count += 1;
        try ix.num(u8, direction);
        try ix.num(u8, try o.side(try w.field(val, "trade_side")));
        try ix.num(u8, try o.tag(try w.field(val, "order_kind"), &.{ "IOC", "Limit" }));
        try ix.quantity(u64, val, "trigger_price");
        try ix.quantity(u64, val, "execution_price");
    }
    if (count == 0) return error.MissingTrigger;
}
fn stop(ix: *Ix, place: bool) !void {
    const p = ix.p;
    const direction = try o.tag(try w.field(p, "execution_direction"), &.{ "GreaterThan", "LessThan" });
    if (place) {
        try ix.quantity(u64, p, "trigger_price");
        try ix.quantity(u64, p, "execution_price");
        try ix.num(u64, 0);
        try ix.num(u8, try o.side(try w.field(p, "trade_side")));
        try ix.num(u8, direction);
        try ix.num(u8, try o.tag(try w.field(p, "order_kind"), &.{ "IOC", "Limit" }));
        try ix.field("funder", .payer);
        try ix.fields(&.{ "trader_account", "perp_asset_map" }, .writable);
        try ix.indexes(true);
        try ix.fields(&.{ "orderbook", "spline_collection" }, .writable);
        try ix.field("position_authority", .signer);
        try ix.field("stop_loss_account", .writable);
    } else {
        try ix.num(u8, direction);
        try ix.field("funder", .writable);
        try ix.field("trader_account", .readonly);
        try ix.field("position_authority", .signer);
        var asset: [8]u8 = undefined;
        std.mem.writeInt(u64, &asset, try w.unsigned(u64, try w.field(p, "asset_id")), .little);
        const trader = try @import("collateral.zig").key(ix, "trader_account");
        try ix.address(try sol.programAddress(ix.a, b.program, &.{ "stoploss", &trader, &asset }), .writable);
    }
    try ix.address(b.system, .readonly);
}
pub fn build(ix: *Ix, name: []const u8) !bool {
    const kind = std.meta.stringToEnum(Kind, name) orelse return false;
    const p = ix.p;
    try ix.discriminator(name);
    try ix.logAccounts();
    try ix.address(b.config, if (kind == .place_limit_order_with_conditionals) .writable else .readonly);
    switch (kind) {
        .cancel_stop_loss, .place_stop_loss => try stop(ix, kind == .place_stop_loss),
        .create_conditional_orders_account => {
            const cap = try w.unsigned(u8, try w.field(p, "capacity"));
            if (cap == 0) return error.InvalidCapacity;
            try ix.num(u8, cap);
            try ix.field("payer", .payer);
            try ix.fields(&.{ "trader_wallet", "trader_account" }, .readonly);
            try ix.field("trader_conditional_orders", .writable);
            try ix.address(b.system, .readonly);
        },
        .cancel_conditional_order, .place_attached_conditional_order => {
            try ix.field("trader_account", .writable);
            try ix.field("trader_wallet", .signer);
            try ix.fields(&.{ "orderbook", "trader_conditional_orders" }, .writable);
            if (kind == .cancel_conditional_order) {
                const index = try w.unsigned(u8, try w.field(p, "conditional_order_index"));
                if (index == 0 or index > 191) return error.InvalidOrderIndex;
                try ix.num(u8, index);
                try ix.boolean(p, "disable_first");
                try ix.boolean(p, "disable_second");
                if (ix.data.items[ix.data.items.len - 2] == 0 and ix.data.items[ix.data.items.len - 1] == 0) return error.InvalidDisableFlags;
            } else {
                const id = try w.field(p, "order_id");
                try ix.quantity(u64, id, "price_in_ticks");
                try ix.quantity(u64, id, "order_sequence_number");
                try ix.quantity(u32, p, "asset_id");
                try triggers(ix);
                try ix.field("payer", .payer);
                try ix.indexes(true);
                try ix.address(b.system, .readonly);
            }
        },
        .place_position_conditional_order, .place_limit_order_with_conditionals => {
            const position = kind == .place_position_conditional_order;
            if (position) {
                try ix.quantity(u32, p, "asset_id");
                try ix.field("payer", .payer);
            } else {
                try o.nestedPacket(ix, try w.field(p, "order_packet"));
                try ix.quantity(u64, p, "slot");
                try ix.field("trader_wallet", .signer);
            }
            try triggers(ix);
            if (position) {
                const percent = opt(p, "size_percent");
                if ((percent == .null) == (opt(p, "size_base_lots") == .null)) return error.InvalidSize;
                if (percent != .null) {
                    const v = try w.unsigned(u8, percent);
                    if (v == 0 or v > 100) return error.InvalidSize;
                }
                try ix.option(u64, p, "size_base_lots");
                try ix.option(u8, p, "size_percent");
            }
            try ix.fields(&.{ "trader_account", "perp_asset_map" }, .writable);
            try ix.indexes(true);
            try ix.fields(&.{ "orderbook", "spline_collection" }, .writable);
            if (position) try ix.field("trader_wallet", .signer) else try ix.field("payer", .payer);
            try ix.field("trader_conditional_orders", .writable);
            try ix.address(b.system, .readonly);
        },
    }
    return true;
}
