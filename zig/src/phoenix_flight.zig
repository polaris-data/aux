//! Flight builder fees, delegated onboarding and instruction forwarding.
const std = @import("std");
const w = @import("wire.zig");
const b = @import("phoenix_instructions.zig");
const sol = @import("solana.zig");
const key = @import("phoenix_collateral.zig").key;
const Kind = enum { onboard_trader, proxy_instruction, register_builder, update_fee, set_onboarder_signer };
const flight = "F1ightu9cujFYo34k9CabifLrJT8qzfDVM2Q7BqhJn2W";

fn permission(ix: *b.Instruction, parent: []const u8, delegate: []const u8) !void {
    try ix.address(try sol.programAddress(ix.a, b.program, &.{ "permission", &try key(ix, parent), &try sol.publicKey(delegate) }), .writable);
}
pub fn build(ix: *b.Instruction, name: []const u8) !bool {
    const kind = std.meta.stringToEnum(Kind, name) orelse return false;
    const phoenix = try sol.publicKey(b.program);
    const p = ix.p;
    ix.target = flight;
    try ix.address(try sol.programAddress(ix.a, flight, &.{ &phoenix, "global_state" }), .readonly);
    try ix.address(b.program, .readonly);
    const owner = if (kind == .register_builder or kind == .update_fee) "trader_authority" else "builder_authority";
    const state = try sol.programAddress(ix.a, flight, &.{ &phoenix, &try key(ix, owner), "builder_state" });
    const fee = p.object.get("fee_bps_override") orelse .null;
    try ix.discriminator(if (kind == .proxy_instruction and fee != .null) "proxy_instruction_with_fee_override" else name);
    switch (kind) {
        .register_builder, .update_fee => {
            const register = kind == .register_builder;
            try ix.field(owner, if (register) .payer else .signer);
            if (register) {
                try ix.field("trader_account", .writable);
                try ix.quantity(u8, p, "trader_pda_index");
                try ix.quantity(u8, p, "subaccount_index");
            }
            try ix.address(state, .writable);
            if (register) try ix.address(b.system, .readonly);
            try ix.quantity(u64, p, "fee_bps");
        },
        .set_onboarder_signer => {
            try ix.field(owner, .signer);
            try ix.address(state, .writable);
            try ix.data.appendSlice(ix.a, &try key(ix, "signer"));
        },
        .proxy_instruction => {
            const inner = try w.field(p, "inner_instruction");
            if (!std.mem.eql(u8, try w.string(try w.field(inner, "program_id")), b.program)) return error.InvalidInnerProgram;
            if (fee != .null) {
                try ix.num(u8, 1);
                try ix.num(u64, try w.unsigned(u64, fee));
            }
            const data = try w.field(inner, "data");
            try ix.bytes(data, (try @import("phoenix_orders.zig").array(data)).len);
            try ix.field(owner, .readonly);
            try ix.field("builder_trader_account", .writable);
            try ix.address(state, .readonly);
            try ix.field("trader_wallet", .readonly);
            try @import("phoenix_twap.zig").metas(ix, try w.field(inner, "accounts"));
            if ((p.object.get("root_authority") orelse .null) != .null) {
                const delegate = try sol.programAddress(ix.a, flight, &.{ "collateral_transfer_authority", &phoenix });
                try ix.address(delegate, .readonly);
                try permission(ix, "root_authority", delegate);
            }
        },
        .onboard_trader => {
            try ix.quantity(u32, p, "max_positions");
            try ix.quantity(u32, p, "trader_preference_bits");
            try ix.field(owner, .readonly);
            try ix.address(state, .writable);
            try ix.field("onboarder_signer", .signer);
            const delegate = try sol.programAddress(ix.a, flight, &.{ "trader_onboarding_authority", &phoenix });
            try ix.address(delegate, .readonly);
            try ix.address(b.authority, .readonly);
            try ix.address(b.config, .readonly);
            try ix.field("payer", .payer);
            try ix.field("trader_wallet", .readonly);
            try ix.address(try sol.programAddress(ix.a, b.program, &.{ "trader", &try key(ix, "trader_wallet"), &.{ 0, 0 } }), .writable);
            try ix.address(b.system, .readonly);
            try permission(ix, "risk_authority", delegate);
            try permission(ix, "market_authority", delegate);
            try ix.indexes(false);
        },
    }
    return true;
}
