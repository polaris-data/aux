//! Phoenix, SPL and System Program instruction encoders. See notices/phoenix.
const std = @import("std");
const w = @import("../../core/wire.zig");
const sol = @import("../../core/solana.zig");
const A = w.A;
const V = w.V;
pub const program = "EtrnLzgbS7nMMy5fbD42kXiUzGg8XQzJ972Xtk1cjWih";
pub const authority = "GdxfTLSsdSY37G6fZoYtdGDSfgFnbT2EmRpuePZxWShS";
pub const config = "2zskx2iyCvb6Stg7RBZkt1f6MrF4dpYtMG3yMvKwqtUZ";
pub const system = "11111111111111111111111111111111";
pub const token = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ata = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
const Kind = enum { cancel_all, cancel_up_to, cancel_orders_by_id, uncross_crank, register_trader, realloc_trader, sync_native, delegate_trader, sync_parent_to_child, deposit_funds, withdraw_funds, claim_fees, spl_approve, associated_token_account_idempotent };
pub const Access = enum { readonly, writable, signer, payer };
const Account = struct { pubkey: []const u8, is_signer: bool, is_writable: bool };

pub const Instruction = struct {
    a: A,
    p: V,
    target: []const u8 = program,
    accounts: std.ArrayList(Account) = .empty,
    data: std.ArrayList(u8) = .empty,

    pub fn address(ix: *Instruction, key: []const u8, access: Access) !void {
        _ = try sol.publicKey(key);
        try ix.accounts.append(ix.a, .{ .pubkey = key, .is_signer = access == .payer or access == .signer, .is_writable = access == .payer or access == .writable });
    }
    pub fn field(ix: *Instruction, name: []const u8, access: Access) !void {
        try ix.address(try w.string(try w.field(ix.p, name)), access);
    }
    pub fn list(ix: *Instruction, name: []const u8, nonempty: bool) !void {
        const v = try w.field(ix.p, name);
        if (v != .array or (nonempty and v.array.items.len == 0)) return error.InvalidAccountList;
        for (v.array.items) |k| try ix.address(try w.string(k), .writable);
    }
    pub fn indexes(ix: *Instruction, nonempty: bool) !void {
        try ix.list("global_trader_index", nonempty);
        try ix.list("active_trader_buffer", nonempty);
    }
    pub fn num(ix: *Instruction, comptime T: type, value: T) !void {
        var encoded: [@sizeOf(T)]u8 = undefined;
        std.mem.writeInt(T, &encoded, value, .little);
        try ix.data.appendSlice(ix.a, &encoded);
    }
    pub fn quantity(ix: *Instruction, comptime T: type, obj: V, name: []const u8) !void {
        try ix.num(T, try w.unsigned(T, try w.field(obj, name)));
    }
    pub fn optional(ix: *Instruction, name: []const u8) !void {
        const value = ix.p.object.get(name) orelse .null;
        try ix.num(u8, @intFromBool(value != .null));
        if (value != .null) try ix.num(u64, try w.unsigned(u64, value));
    }
    pub fn discriminator(ix: *Instruction, name: []const u8) !void {
        var hash = std.crypto.hash.sha2.Sha256.init(.{});
        hash.update("global:");
        hash.update(name);
        try ix.data.appendSlice(ix.a, hash.finalResult()[0..8]);
    }
    pub fn logAccounts(ix: *Instruction) !void {
        try ix.address(program, .readonly);
        try ix.address(authority, .readonly);
    }
    pub fn fields(ix: *Instruction, names: []const []const u8, access: Access) !void {
        for (names) |name| try ix.field(name, access);
    }
    pub fn boolean(ix: *Instruction, obj: V, name: []const u8) !void {
        const value = try w.field(obj, name);
        if (value != .bool) return error.InvalidBoolean;
        try ix.num(u8, @intFromBool(value.bool));
    }
    pub fn bytes(ix: *Instruction, value: V, length: usize) !void {
        if (value != .array or value.array.items.len != length) return error.InvalidLength;
        for (value.array.items) |byte| try ix.num(u8, try w.unsigned(u8, byte));
    }
    pub fn option(ix: *Instruction, comptime T: type, obj: V, name: []const u8) !void {
        if (obj != .object) return error.InvalidInstruction;
        const value = obj.object.get(name) orelse .null;
        try ix.num(u8, @intFromBool(value != .null));
        if (value != .null) try ix.num(T, try w.unsigned(T, value));
    }
    pub fn finish(ix: Instruction) !V {
        // Solana instruction data is a JSON byte array, including one-byte opcodes.
        var data = std.json.Array.init(ix.a);
        for (ix.data.items) |byte| try data.append(.{ .integer = byte });
        return w.parse(ix.a, try std.json.Stringify.valueAlloc(ix.a, .{ .program_id = ix.target, .accounts = ix.accounts.items, .data = V{ .array = data } }, .{}));
    }
};

fn buildOne(a: A, kind: Kind, p: V) !V {
    if (p != .object) return error.InvalidInstruction;
    var ix = Instruction{ .a = a, .p = p };
    if (kind == .spl_approve) {
        ix.target = token;
        try ix.field("source", .writable);
        try ix.field("delegate", .readonly);
        try ix.field("owner", .signer);
        try ix.num(u8, 4);
        try ix.quantity(u64, p, "amount");
        return ix.finish();
    }
    if (kind == .associated_token_account_idempotent) {
        ix.target = ata;
        try ix.field("payer", .payer);
        try ix.address(try w.string(try sol.associated(a, p)), .writable);
        try ix.field("owner", .readonly);
        try ix.field("mint", .readonly);
        try ix.address(system, .readonly);
        try ix.address(token, .readonly);
        try ix.num(u8, 1);
        return ix.finish();
    }
    try ix.discriminator(@tagName(kind));
    try ix.logAccounts();
    const cfg_access: Access = switch (kind) {
        .sync_native, .cancel_all, .cancel_up_to, .cancel_orders_by_id, .uncross_crank, .deposit_funds, .withdraw_funds => .writable,
        else => .readonly,
    };
    try ix.address(config, cfg_access);
    switch (kind) {
        .cancel_all, .cancel_up_to, .cancel_orders_by_id, .uncross_crank => {
            if (kind != .uncross_crank) {
                try ix.field("trader", .signer);
                try ix.field("trader_account", .writable);
            }
            try ix.field("perp_asset_map", .writable);
            try ix.indexes(true);
            try ix.field("orderbook", .writable);
            try ix.field("spline_collection", .writable);
            if (kind == .uncross_crank) try ix.quantity(u64, p, "match_limit");
            if (kind == .cancel_up_to) {
                const side = try w.string(try w.field(p, "side"));
                const s: u8 = if (std.mem.eql(u8, side, "bid") or std.mem.eql(u8, side, "Bid")) 0 else if (std.mem.eql(u8, side, "ask") or std.mem.eql(u8, side, "Ask")) 1 else return error.InvalidSide;
                try ix.num(u8, s);
                try ix.optional("num_orders_to_cancel");
                try ix.optional("tick_limit");
            }
            if (kind == .cancel_orders_by_id) {
                const ids = try w.field(p, "order_ids");
                if (ids != .array or ids.array.items.len == 0 or ids.array.items.len > 100) return error.InvalidOrderCount;
                try ix.num(u32, @intCast(ids.array.items.len));
                for (ids.array.items) |id| {
                    try ix.quantity(u32, id, "node_pointer");
                    const order = try w.field(id, "order_id");
                    try ix.quantity(u64, order, "price_in_ticks");
                    try ix.quantity(u64, order, "order_sequence_number");
                }
            }
        },
        .register_trader, .realloc_trader => {
            try ix.field("payer", .payer);
            try ix.field(if (kind == .register_trader) "trader" else "trader_wallet", .readonly);
            try ix.field("trader_account", .writable);
            try ix.address(system, .readonly);
            if (kind == .register_trader) {
                const max = try w.unsigned(u32, try w.field(p, "max_positions"));
                const sub = try w.unsigned(u8, try w.field(p, "subaccount_index"));
                if (max == 0 or max > 128 or sub > 100) return error.InvalidTrader;
                try ix.num(u32, max);
                try ix.quantity(u32, p, "trader_preference_bits");
                try ix.quantity(u8, p, "trader_pda_index");
                try ix.num(u8, sub);
            }
        },
        .sync_native => {
            try ix.field("trader_account", .writable);
            try ix.indexes(false);
        },
        .delegate_trader => {
            try ix.field("trader_wallet", .signer);
            try ix.field("trader_account", .writable);
            try ix.field("new_position_authority", .readonly);
        },
        .sync_parent_to_child => {
            try ix.field("trader_wallet", .signer);
            try ix.field("parent_trader_account", .readonly);
            try ix.field("child_trader_account", .writable);
            try ix.list("global_trader_index", false);
        },
        .claim_fees => {
            try ix.field("authority", .signer);
            try ix.field("global_vault", .writable);
            try ix.field("destination", .writable);
            try ix.address(token, .readonly);
            try ix.quantity(u64, p, "amount");
        },
        .deposit_funds, .withdraw_funds => {
            try ix.field("trader", .signer);
            if (kind == .deposit_funds) try ix.field("trader_token_account", .writable);
            try ix.field("trader_account", .writable);
            if (kind == .withdraw_funds) try ix.field("perp_asset_map", .writable);
            try ix.field("global_vault", .writable);
            if (kind == .withdraw_funds) try ix.field("trader_token_account", .writable);
            try ix.address(token, .readonly);
            try ix.indexes(false);
            if (kind == .withdraw_funds) try ix.field("withdraw_queue", .writable) else if (p.object.get("permission_account")) |addr| {
                if (addr != .null) try ix.address(try w.string(addr), .writable);
            }
            try ix.quantity(u64, p, "amount");
        },
        .associated_token_account_idempotent, .spl_approve => unreachable,
    }
    return ix.finish();
}

pub fn build(a: A, name: []const u8, p: V) !V {
    if (std.mem.eql(u8, name, "create_deposit_native_sol_ixs")) {
        const amount = try w.unsigned(u64, try w.field(p, "lamports"));
        if (amount == 0) return error.InvalidDeposit;
        const sync = try w.field(p, "sync");
        var accounts = V{ .object = .empty };
        for ([_][]const u8{ "payer", "trader_wallet" }) |k| try accounts.object.put(a, k, try w.field(p, k));
        try accounts.object.put(a, "trader_account", try w.field(sync, "trader_account"));
        var tx = Instruction{ .a = a, .p = accounts, .target = system };
        try tx.field("trader_wallet", .payer);
        try tx.field("trader_account", .writable);
        try tx.num(u32, 2);
        try tx.num(u64, amount);
        var out = std.json.Array.init(a);
        try out.append(try buildOne(a, .realloc_trader, accounts));
        try out.append(try tx.finish());
        try out.append(try buildOne(a, .sync_native, sync));
        return .{ .array = out };
    }
    if (!std.mem.startsWith(u8, name, "create_") or !std.mem.endsWith(u8, name, "_ix") or name.len <= 10) return error.UnknownInstruction;
    const operation = name[7 .. name.len - 3];
    var ix = Instruction{ .a = a, .p = p };
    if (p != .object) return error.InvalidInstruction;
    if (try @import("orders.zig").build(&ix, operation)) return ix.finish();
    if (try @import("collateral.zig").build(&ix, operation)) return ix.finish();
    if (try @import("conditional.zig").build(&ix, operation)) return ix.finish();
    if (try @import("spline.zig").build(&ix, operation)) return ix.finish();
    if (try @import("twap.zig").build(&ix, operation)) return ix.finish();
    if (try @import("swap.zig").build(&ix, operation)) return ix.finish();
    if (try @import("flight.zig").build(&ix, operation)) return ix.finish();
    return buildOne(a, std.meta.stringToEnum(Kind, name[7 .. name.len - 3]) orelse return error.UnknownInstruction, p);
}
