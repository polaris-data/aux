//! Swap account layouts for SOL, USDC and other collateral mints.
const std = @import("std");
const w = @import("../../core/wire.zig");
const b = @import("instructions.zig");
const o = @import("orders.zig");
const c = @import("collateral.zig");
const sol = @import("../../core/solana.zig");
const Ix = b.Instruction;
const Kind = enum { swap_native, swap_spot_with_usdc, swap_spot_with_sol, swap_spot_with_spot, liquidate_spot, liquidate_native_sol };

fn encodePacked(ix: *Ix, venue: w.V) !void {
    const instructions = try o.array(try w.field(venue, "instructions"));
    const accounts = try o.array(try w.field(venue, "accounts"));
    try ix.num(u32, @intCast(instructions.len));
    for (instructions) |instruction| {
        const index = try w.unsigned(u8, try w.field(instruction, "program_id_index"));
        if (index >= accounts.len) return error.InvalidVenueAccount;
        try ix.num(u8, index);
        for ([_][]const u8{ "data", "account_metas" }, 0..) |name, i| {
            const values = try o.array(try w.field(instruction, name));
            try ix.num(u32, @intCast(values.len));
            for (values) |val| {
                const byte = try w.unsigned(u8, val);
                if (i == 1 and byte >> 2 >= accounts.len) return error.InvalidVenueAccount;
                try ix.num(u8, byte);
            }
        }
    }
}
fn liquidate(ix: *Ix, spot: bool) !void {
    const p = ix.p;
    const venue = try w.field(p, "venue");
    const extra = try o.array(try w.field(p, "extra_trader_accounts"));
    const maximum = try w.unsigned(u64, try w.field(p, if (spot) "max_spot_amount" else "max_native_sol_amount"));
    try ix.num(u64, maximum);
    try ix.num(u64, @intCast(extra.len));
    try encodePacked(ix, venue);
    try ix.logAccounts();
    try ix.address(b.config, .writable);
    try ix.field("perp_asset_map", .writable);
    if (spot) try ix.field("spot_mint", .readonly);
    const mint = try c.key(ix, if (spot) "quote_mint" else "mint");
    try ix.address(try sol.programAddress(ix.a, b.program, &.{ "vault", &mint }), .writable);
    try ix.address(b.token, .readonly);
    const authority = if (spot) try c.wallet(ix, "liquidatee_account") else try sol.programAddress(ix.a, b.program, &.{"native_sol"});
    try ix.address(authority, .readonly);
    try ix.field("signer", .payer);
    const permission = p.object.get("permission_account") orelse .null;
    try ix.address(try w.string(if (permission == .null) try w.field(p, "signer") else permission), .writable);
    try ix.field("signer_quote_token_account", .writable);
    if (spot) try ix.field("signer_spot_token_account", .writable);
    try ix.field("liquidatee_account", .writable);
    if (spot) try ix.address(try c.associated(ix, authority, "spot_mint"), .writable) else try ix.field("withdraw_queue", .writable);
    try ix.indexes(false);
    for (extra) |addr| try ix.address(try w.string(addr), .writable);
    try @import("twap.zig").metas(ix, try w.field(venue, "accounts"));
}
pub fn build(ix: *Ix, name: []const u8) !bool {
    const kind = std.meta.stringToEnum(Kind, name) orelse return false;
    const p = ix.p;
    try ix.discriminator(name);
    if (kind == .liquidate_spot or kind == .liquidate_native_sol) {
        try liquidate(ix, kind == .liquidate_spot);
        return true;
    }
    if (kind != .swap_spot_with_spot) try ix.num(u8, try o.tag(try w.field(p, "direction"), &.{ "Sell", "Buy" }));
    const amount = try w.unsigned(u64, try w.field(p, "amount_in"));
    if (amount == 0) return error.InvalidAmount;
    try ix.num(u64, amount);
    const slippage = try w.field(p, "slippage");
    if (slippage == .string and std.mem.eql(u8, slippage.string, "Unprotected")) try ix.num(u64, 0) else try ix.quantity(u64, slippage, "MinAmountOut");
    const venue = try w.field(p, "venue");
    try encodePacked(ix, venue);
    try ix.logAccounts();
    try ix.address(b.config, .writable);
    try ix.field("perp_asset_map", .writable);
    if (kind == .swap_spot_with_spot) {
        try ix.fields(&.{ "src_mint", "dst_mint" }, .readonly);
    } else if (kind != .swap_native) try ix.field("spot_mint", .readonly);
    if (kind == .swap_native or kind == .swap_spot_with_usdc) {
        const mint = try c.key(ix, if (kind == .swap_native) "mint" else "quote_mint");
        try ix.address(try sol.programAddress(ix.a, b.program, &.{ "vault", &mint }), .writable);
    }
    try ix.address(b.token, .readonly);
    if (kind == .swap_native or kind == .swap_spot_with_sol) {
        try ix.address(b.system, .readonly);
        try ix.address(try sol.programAddress(ix.a, b.program, &.{"native_sol"}), .readonly);
    }
    const owner = if (kind == .swap_native) "" else try c.wallet(ix, "trader_account");
    if (kind != .swap_native) try ix.address(owner, .readonly);
    try ix.field("signer", .payer);
    if (kind == .swap_native or kind == .swap_spot_with_usdc) try ix.field("signer_quote_token_account", .writable);
    if (kind == .swap_spot_with_sol or kind == .swap_spot_with_usdc) try ix.field("signer_spot_token_account", .writable);
    try ix.fields(&.{ "trader_account", "withdraw_queue" }, .writable);
    if (kind == .swap_spot_with_spot) {
        try ix.address(try c.associated(ix, owner, "src_mint"), .writable);
        try ix.fields(&.{ "signer_src_token_account", "signer_dst_token_account" }, .writable);
        try ix.address(try c.associated(ix, owner, "dst_mint"), .writable);
    } else if (kind != .swap_native) try ix.address(try c.associated(ix, owner, "spot_mint"), .writable);
    try ix.indexes(false);
    try @import("twap.zig").metas(ix, try w.field(venue, "accounts"));
    return true;
}
