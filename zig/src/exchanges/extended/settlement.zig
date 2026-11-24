// Settlement message layouts from fast-stark-crypto 0.5.0 (notices/extended).
const std = @import("std");
const w = @import("../../core/wire.zig");
const stark = @import("../../core/stark.zig");
const Kind = enum { order, limit_order, transfer, withdrawal };
const types = @import("settlement_types.zig");

fn felt(v: w.V) !u256 {
    return stark.felt(if (v == .number_string) v.number_string else try w.string(v));
}
fn position(v: w.V, key: []const u8) !u32 {
    return w.unsigned(u32, try w.field(try w.field(v, key), "value"));
}
fn asset(v: w.V, key: []const u8) !u256 {
    return felt(try w.field(try w.field(v, key), "value"));
}
fn amount(v: w.V, key: []const u8) !u256 {
    const s = try w.field(v, key);
    const n = if (s == .number_string) s.number_string else try w.string(s);
    const signed = try std.fmt.parseInt(i64, n, 10);
    return if (signed < 0) stark.prime - @abs(signed) else @intCast(signed);
}
fn hex(a: w.A, n: u256) !w.V {
    return w.text(try std.fmt.allocPrint(a, "0x{x}", .{n}));
}
pub fn settle(a: w.A, input: w.V) !w.V {
    const kind = std.meta.stringToEnum(Kind, try w.string(try w.field(input, "kind"))) orelse return error.InvalidOperation;
    const msg = try w.field(input, "message");
    const chain = try w.string(try w.field(input, "chain_id"));
    if (chain.len == 0) return error.InvalidNetwork;
    const pk = try felt(try w.field(input, "public_key"));
    const expiration = try w.unsigned(u64, try w.field(try w.field(msg, "expiration"), "seconds"));
    const salt = try felt(try w.field(msg, "salt"));
    var fields: std.ArrayList(u256) = .empty;
    if (kind == .order or kind == .limit_order) {
        if (kind == .order) {
            try fields.appendSlice(a, &.{ stark.selector(types.order), try position(msg, "position_id") });
        } else {
            try fields.appendSlice(a, &.{ stark.selector(types.limit_order), try position(msg, "source_position"), try position(msg, "receive_position") });
        }
        try fields.appendSlice(a, &.{ try asset(msg, "base_asset_id"), try amount(msg, "base_amount"), try asset(msg, "quote_asset_id"), try amount(msg, "quote_amount"), try asset(msg, "fee_asset_id"), try w.unsigned(u64, try w.field(msg, "fee_amount")) });
    } else {
        const recipient = if (kind == .transfer) try position(msg, "recipient") else try felt(try w.field(msg, "recipient"));
        try fields.appendSlice(a, &.{ stark.selector(if (kind == .transfer) types.transfer else types.withdrawal), recipient, try position(msg, "position_id"), try asset(msg, "collateral_id"), try w.unsigned(u64, try w.field(msg, "amount")) });
    }
    try fields.appendSlice(a, &.{ expiration, salt });
    const domain = stark.poseidon(&.{ stark.selector(types.domain), try stark.short("Perpetuals"), try stark.short("v0"), try stark.short(chain), 1 });
    const digest = stark.poseidon(&.{ try stark.short("StarkNet Message"), domain, pk, stark.poseidon(fields.items) });
    var out: w.V = .{ .object = .empty };
    try out.object.put(a, "hash", try hex(a, digest));
    if (input.object.get("stark_key")) |value| {
        const sk = try felt(value);
        if (try stark.publicKey(sk) != pk) return error.SignerMismatch;
        const sig = try stark.signWithEntropy(sk, digest, &.{});
        var signature: w.V = .{ .object = .empty };
        try signature.object.put(a, "r", try hex(a, sig[0]));
        try signature.object.put(a, "s", try hex(a, sig[1]));
        try out.object.put(a, "signature", signature);
    }
    return out;
}
