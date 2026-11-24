//! Phoenix collateral, trader authority and SPL vault instructions.
const std = @import("std");
const w = @import("wire.zig");
const sol = @import("solana.zig");
const b = @import("phoenix_instructions.zig");
const orders = @import("phoenix_orders.zig");
const Ix = b.Instruction;
const Kind = enum { sync_spot, withdraw_spot, withdraw_native_sol, transfer_collateral, transfer_native_sol, transfer_spot, withdraw_to_target_leverage, transfer_collateral_child_to_parent, transfer_spot_from_child_to_parent, transfer_native_sol_from_child_to_parent, ember_deposit, ember_withdraw, update_trader_preference_bits, onboard_trader_delegated, register_spline, deactivate_spline };

pub fn key(ix: *Ix, name: []const u8) ![32]u8 {
    return sol.publicKey(try w.string(try w.field(ix.p, name)));
}
pub fn wallet(ix: *Ix, name: []const u8) ![]const u8 {
    return sol.programAddress(ix.a, b.program, &.{ "wallet", &try key(ix, name) });
}
pub fn associated(ix: *Ix, owner: []const u8, mint: []const u8) ![]const u8 {
    return sol.programAddress(ix.a, "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL", &.{ &try sol.publicKey(owner), &try sol.publicKey(b.token), &try key(ix, mint) });
}
fn withdrawal(ix: *Ix, spot: bool) !void {
    try ix.address(b.config, .writable);
    try ix.field("trader", .signer);
    try ix.fields(&.{ "trader_account", "perp_asset_map" }, .writable);
    const dest = try key(ix, "destination");
    if (spot) {
        const owner = try wallet(ix, "trader_account");
        const vault = try associated(ix, owner, "mint");
        if (std.mem.eql(u8, &dest, &try sol.publicKey(vault))) return error.InvalidDestination;
        try ix.field("mint", .readonly);
        try ix.address(owner, .readonly);
        try ix.address(vault, .writable);
    } else if (std.mem.eql(u8, &dest, &try key(ix, "trader_account"))) return error.InvalidDestination;
    try ix.field("destination", .writable);
    if (spot) try ix.address(b.token, .readonly);
    try ix.field("withdraw_queue", .writable);
    try ix.indexes(false);
    const action = try w.field(ix.p, "action");
    if (action == .string and std.mem.eql(u8, action.string, "AllExcess")) return ix.num(u8, 0);
    if (action != .object or action.object.count() != 1) return error.InvalidWithdrawal;
    try ix.num(u8, 1 + try orders.tag(w.text(action.object.keys()[0]), &.{ "WithExcess", "WithoutExcess" }));
    const amount = try w.unsigned(u64, try w.field(action.object.values()[0], "amount"));
    if (amount == 0) return error.InvalidAmount;
    try ix.num(u64, amount);
}
fn transfer(ix: *Ix, kind: Kind) !void {
    const child = switch (kind) {
        .transfer_collateral_child_to_parent, .transfer_spot_from_child_to_parent, .transfer_native_sol_from_child_to_parent => true,
        else => false,
    };
    const src = if (child) "child_trader_account" else "src_trader_account";
    const dst = if (child) "parent_trader_account" else "dst_trader_account";
    if (kind == .transfer_spot or kind == .transfer_spot_from_child_to_parent) {
        const owner = try wallet(ix, src);
        try ix.field("mint", .readonly);
        try ix.address(owner, .readonly);
        try ix.address(try associated(ix, owner, "mint"), .writable);
        try ix.address(try associated(ix, try wallet(ix, dst), "mint"), .writable);
        try ix.address(b.token, .readonly);
    }
    try ix.address(b.config, .readonly);
    var signs = true;
    if (child and kind != .transfer_collateral_child_to_parent) {
        const flag = try w.field(ix.p, "trader_signs");
        if (flag != .bool) return error.InvalidBoolean;
        signs = flag.bool;
    }
    try ix.field("trader", if (signs) .signer else .readonly);
    try ix.fields(&.{ src, dst }, .writable);
    try ix.field("perp_asset_map", .readonly);
    try ix.indexes(false);
    if (!child) {
        const permission = ix.p.object.get("permission_account") orelse .null;
        if (permission != .null) try ix.address(try w.string(permission), .writable);
        const amount = try w.unsigned(u64, try w.field(ix.p, switch (kind) {
            .withdraw_to_target_leverage => "target_leverage_bps",
            .transfer_native_sol => "lamports",
            else => "amount",
        }));
        if (amount == 0) return error.InvalidAmount;
        try ix.num(u64, amount);
    }
}
fn ember(ix: *Ix, deposit: bool) !void {
    ix.target = "EMBERpYNE6ehWmXymZZS2skiFmCa9V5dp14e1iduM5qy";
    const phoenix = try sol.publicKey(b.program);
    try ix.discriminator(if (deposit) "deposit" else "withdraw");
    try ix.field("trader", .signer);
    try ix.address(try sol.programAddress(ix.a, ix.target, &.{ &phoenix, "state" }), .readonly);
    try ix.field("usdc_mint", .readonly);
    try ix.fields(&.{ "canonical_mint", "trader_usdc_account", "trader_phoenix_account" }, .writable);
    try ix.address(try sol.programAddress(ix.a, ix.target, &.{ &phoenix, "vault" }), .writable);
    try ix.address(b.token, .readonly);
    if (deposit) try ix.quantity(u64, ix.p, "amount") else try ix.optional("amount");
}
pub fn build(ix: *Ix, name: []const u8) !bool {
    const kind = std.meta.stringToEnum(Kind, name) orelse return false;
    if (kind == .ember_deposit or kind == .ember_withdraw) {
        try ember(ix, kind == .ember_deposit);
        return true;
    }
    try ix.logAccounts();
    try ix.discriminator(if (kind == .onboard_trader_delegated) "set_trader_capabilities_delegated" else name);
    switch (kind) {
        .sync_spot => {
            try ix.address(b.config, .readonly);
            try ix.field("perp_asset_map", .writable);
            try ix.field("mint", .readonly);
            try ix.field("trader_account", .writable);
            try ix.address(try associated(ix, try wallet(ix, "trader_account"), "mint"), .readonly);
            try ix.indexes(false);
        },
        .withdraw_spot, .withdraw_native_sol => try withdrawal(ix, kind == .withdraw_spot),
        .register_spline, .deactivate_spline => {
            try ix.address(b.config, .readonly);
            try ix.field("authority", .signer);
            const permission = ix.p.object.get("permission_account") orelse .null;
            try ix.address(try w.string(if (permission == .null) try w.field(ix.p, "authority") else permission), .writable);
            if (kind == .register_spline) try ix.field("payer", .payer);
            try ix.field("spline_collection", .writable);
            try ix.field("market_account", .readonly);
            try ix.indexes(false);
            try ix.field("trader_account", .writable);
            try ix.field("perp_asset_map", .readonly);
            try ix.address(b.system, .readonly);
        },
        .onboard_trader_delegated => {
            try ix.address(b.config, .readonly);
            try ix.field("authority", .signer);
            try ix.fields(&.{ "permission_account", "trader_account" }, .writable);
            try ix.indexes(false);
            try ix.num(u32, 6);
            for ([_]u8{ 0, 1, 3, 2, 4, 5 }) |capability| {
                try ix.num(u8, capability);
                try ix.num(u8, 1);
            }
        },
        .update_trader_preference_bits => {
            try ix.address(b.config, .readonly);
            try ix.field("trader_wallet", .signer);
            try ix.field("trader_wallet", .readonly);
            try ix.field("trader_account", .writable);
            const toggles = try orders.array(try w.field(ix.p, "toggles"));
            try ix.num(u32, @intCast(toggles.len));
            for (toggles) |toggle| {
                try ix.num(u8, try orders.tag(try w.field(toggle, "target"), &.{ "CollateralSweep", "PositionAuthoritySwap" }));
                try ix.boolean(toggle, "set");
            }
        },
        .ember_deposit, .ember_withdraw => unreachable,
        else => try transfer(ix, kind),
    }
    return true;
}
