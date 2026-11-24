//! Flicker TWAP instructions and their fixed-size child IOC packet.
const std = @import("std");
const w = @import("wire.zig");
const b = @import("phoenix_instructions.zig");
const o = @import("phoenix_orders.zig");
const util = @import("phoenix_spline.zig");
const Ix = b.Instruction;
const Kind = enum { twap_account, place_twap_order, execute_twap_order, cancel_twap_order, close_inactive_twap_account };

pub fn metas(ix: *Ix, value: w.V) !void {
    for (try o.array(value)) |m| {
        const signer = try util.flag(m, "is_signer");
        const writable = try util.flag(m, "is_writable");
        try ix.address(try w.string(try w.field(m, "pubkey")), if (signer) (if (writable) .payer else .signer) else (if (writable) .writable else .readonly));
    }
}
fn raw(ix: *Ix, p: w.V, name: []const u8) !void {
    const val = try util.optionalValue(u64, p, name);
    if (val != null and val.? == 0) return error.InvalidPriceOrLimit;
    try ix.num(u64, val orelse 0);
}
fn packet(ix: *Ix, p: w.V) !void {
    try ix.num(u64, 1);
    try ix.num(u8, try o.side(try w.field(p, "side")));
    try ix.data.appendNTimes(ix.a, 0, 7);
    try raw(ix, p, "price_in_ticks");
    try ix.quantity(u64, p, "num_base_lots");
    try raw(ix, p, "num_quote_lots");
    try ix.quantity(u64, p, "min_base_lots_to_fill");
    try ix.quantity(u64, p, "min_quote_lots_to_fill");
    try ix.num(u8, try o.behavior(try w.field(p, "self_trade_behavior")));
    try ix.data.appendNTimes(ix.a, 0, 7);
    try raw(ix, p, "match_limit");
    try ix.quantity(u128, p, "client_order_id");
    try raw(ix, p, "last_valid_slot");
    try ix.num(u8, try o.flags(try w.field(p, "order_flags")));
    try ix.boolean(p, "cancel_existing");
    try ix.data.appendNTimes(ix.a, 0, 6);
    try ix.num(u64, (try util.optionalValue(u64, p, "dust_order_size")) orelse 0);
    try ix.num(u64, (try util.optionalValue(u64, p, "n_dust_orders")) orelse 0);
    try ix.data.appendNTimes(ix.a, 0, 32);
}
pub fn build(ix: *Ix, name: []const u8) !bool {
    const kind = std.meta.stringToEnum(Kind, name) orelse return false;
    const p = ix.p;
    ix.target = try w.string(try w.field(p, "flicker_program_id"));
    try ix.discriminator(if (kind == .twap_account) "create_twap_account" else name);
    try ix.fields(&.{ "twap_global_state", "phoenix_program_id" }, .readonly);
    switch (kind) {
        .twap_account => {
            try ix.field("twap_account", .writable);
            try ix.field("trader_account", .readonly);
            try ix.field("payer", .payer);
            try ix.address(b.system, .readonly);
            try ix.field("twap_log_authority", .readonly);
            try ix.quantity(u32, p, "market_id");
        },
        .close_inactive_twap_account => {
            try ix.fields(&.{ "twap_account", "recipient" }, .writable);
            try ix.field("twap_log_authority", .readonly);
        },
        else => {
            if (kind != .cancel_twap_order) try ix.field("hawkeye_program_id", .readonly);
            try ix.field("twap_log_authority", .readonly);
            try ix.field("twap_account", .writable);
            if (kind != .execute_twap_order) try ix.field("authority", .signer);
        },
    }
    try ix.field("flicker_program_id", .readonly);
    if (kind == .execute_twap_order or kind == .place_twap_order) {
        const transfer = try w.field(p, "transfer_accounts");
        const count = std.math.cast(u8, (try o.array(transfer)).len) orelse return error.TooManyAccounts;
        if (kind == .place_twap_order) {
            try ix.quantity(u64, p, "cooldown_slots");
            try ix.quantity(u64, p, "n_child_orders");
            try ix.quantity(u64, p, "child_order_max_slippage_bps");
            try ix.optional("child_order_min_price_in_ticks");
            try ix.optional("child_order_max_price_in_ticks");
            try packet(ix, try w.field(p, "child_order_packet"));
            try ix.optional("child_order_collateral_quote_lots_to_transfer");
            try ix.optional("last_valid_slot");
        }
        try ix.num(u8, count);
        try metas(ix, transfer);
        try metas(ix, try w.field(p, "order_accounts"));
    }
    return true;
}
