//! Borsh order packets and the Hawkeye read-only instruction family.
const std = @import("std");
const w = @import("../../core/wire.zig");
const base = @import("instructions.zig");
const Ix = base.Instruction;
const V = w.V;
const Kind = enum { place_limit_order, place_market_order, place_market_order_delegated, place_multi_limit_order, place_multi_limit_order_v2, hawkeye_view_margin, hawkeye_view_margin_for_asset, hawkeye_view_liquidation_price, hawkeye_view_funding, hawkeye_view_bbo, hawkeye_view_order_quote };

pub fn tag(v: V, names: []const []const u8) !u8 {
    const name = try w.string(v);
    for (names, 0..) |candidate, i| if (std.mem.eql(u8, name, candidate)) return @intCast(i);
    return error.InvalidVariant;
}
pub fn side(v: V) !u8 {
    return (try tag(v, &.{ "Bid", "Ask", "bid", "ask" })) % 2;
}
pub fn flags(v: V) !u8 {
    return (try tag(v, &.{ "None", "ReduceOnly" })) << 7;
}
pub fn behavior(v: V) !u8 {
    return tag(v, &.{ "Abort", "CancelProvide", "DecrementTake" });
}
pub fn nestedPacket(ix: *Ix, packet: V) !void {
    const k = try w.field(packet, "kind");
    if (k != .object or k.object.count() != 1) return error.InvalidOrder;
    const kind = try tag(w.text(k.object.keys()[0]), &.{ "PostOnly", "Limit", "ImmediateOrCancel" });
    try encodePacket(ix, kind, k.object.values()[0], true);
}
fn encodePacket(ix: *Ix, kind: u8, p: V, byte_id: bool) !void {
    try ix.num(u8, kind);
    try ix.num(u8, try side(try w.field(p, "side")));
    if (kind == 2) try ix.option(u64, p, "price_in_ticks") else try ix.quantity(u64, p, "price_in_ticks");
    try ix.quantity(u64, p, "num_base_lots");
    if (kind == 2) {
        try ix.option(u64, p, "num_quote_lots");
        try ix.quantity(u64, p, "min_base_lots_to_fill");
        try ix.quantity(u64, p, "min_quote_lots_to_fill");
    }
    if (kind != 0) {
        try ix.num(u8, try behavior(try w.field(p, "self_trade_behavior")));
        try ix.option(u64, p, "match_limit");
    }
    if (byte_id) try ix.bytes(try w.field(p, "client_order_id"), 16) else try ix.quantity(u128, p, "client_order_id");
    if (kind == 0) try ix.boolean(p, "slide");
    try ix.option(u64, p, "last_valid_slot");
    try ix.num(u8, try flags(try w.field(p, "order_flags")));
    try ix.boolean(p, "cancel_existing");
}
pub fn array(v: V) ![]const V {
    if (v != .array or v.array.items.len > std.math.maxInt(u32)) return error.InvalidArray;
    return v.array.items;
}
fn multi(ix: *Ix, v2: bool) !void {
    for ([_][]const u8{ "bids", "asks" }) |name| {
        const rows = try array(try w.field(ix.p, name));
        try ix.num(u32, @intCast(rows.len));
        for (rows) |row| {
            try ix.quantity(u64, row, "price_in_ticks");
            try ix.quantity(u64, row, "size_in_base_lots");
            if (v2) {
                try ix.quantity(u64, row, "last_valid_slot");
                try ix.quantity(u8, row, "flags");
            } else try ix.option(u64, row, "last_valid_slot");
        }
    }
    try ix.option(u128, ix.p, "client_order_id");
    if (v2) {
        const id = try w.unsigned(u8, try w.field(ix.p, "scale_set_id"));
        const cont = ix.p.object.get("scale_set_continuation") orelse w.V{ .bool = false };
        if (cont != .bool or id > 127 or (id == 0 and cont.bool)) return error.InvalidScaleSet;
        try ix.num(u8, if (cont.bool) id | 128 else id);
    } else try ix.boolean(ix.p, "slide");
}
fn hawkeye(ix: *Ix, kind: Kind) !void {
    ix.target = "RiSeVw3ZjNfsaXPRb4mgaqYaEEt41pNNJoDvVh7pgQj";
    const p = ix.p;
    const quote = kind == .hawkeye_view_order_quote;
    const market = quote or kind == .hawkeye_view_bbo;
    const asset = !market and kind != .hawkeye_view_margin;
    const accounts = if (quote or asset) try w.field(p, "accounts") else p;
    ix.p = if (quote) try w.field(accounts, "market") else accounts;
    try ix.fields(&.{ "phoenix_program_id", "global_config" }, .readonly);
    for ([_][]const u8{ "global_trader_index", "active_trader_buffer" }) |k| {
        for (try array(try w.field(ix.p, k))) |addr| try ix.address(try w.string(addr), .readonly);
    }
    try ix.field("perp_asset_map", .readonly);
    if (market) try ix.fields(&.{ "orderbook", "spline_collection" }, .readonly) else try ix.field("trader", .readonly);
    try ix.discriminator(@tagName(kind)["hawkeye_".len..]);
    if (asset) {
        try ix.quantity(u32, p, "asset_id");
        try ix.num(u32, 0);
    }
    if (quote) {
        ix.p = accounts;
        try ix.field("trader", .readonly);
        try ix.field("scratch", .writable);
        try ix.quantity(u64, p, "reference_price_ticks");
        try nestedPacket(ix, try w.field(p, "order_packet"));
    }
}
pub fn build(ix: *Ix, name: []const u8) !bool {
    const kind = std.meta.stringToEnum(Kind, name) orelse return false;
    if (std.mem.startsWith(u8, name, "hawkeye_")) {
        try hawkeye(ix, kind);
        return true;
    }
    try ix.discriminator(name);
    try ix.logAccounts();
    try ix.address(base.config, .writable);
    if (kind == .place_market_order_delegated) {
        try ix.field("trader_wallet", .signer);
        try ix.field("permission_account", .writable);
        ix.p = try w.field(ix.p, "market_order");
    } else try ix.field("trader", .signer);
    try ix.fields(&.{ "trader_account", "perp_asset_map" }, .writable);
    try ix.indexes(true);
    try ix.fields(&.{ "orderbook", "spline_collection" }, .writable);
    if (kind == .place_multi_limit_order or kind == .place_multi_limit_order_v2)
        try multi(ix, kind == .place_multi_limit_order_v2)
    else
        try encodePacket(ix, if (kind == .place_limit_order) 1 else 2, ix.p, false);
    return true;
}
