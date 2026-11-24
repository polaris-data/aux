// Build native settlement amounts without converting decimal prices or fees to floats.
const std = @import("std");
const w = @import("wire.zig");
const stark = @import("stark.zig");
const Big = std.math.big.int.Managed;
const A = w.A;
const V = w.V;
fn asValue(a: A, value: anytype) !V {
    return w.parse(a, try std.json.Stringify.valueAlloc(a, value, .{}));
}
fn number(a: A, n: anytype) !V {
    return w.text(try std.fmt.allocPrint(a, "{d}", .{n}));
}
fn text(v: V, key: []const u8) ![]const u8 {
    return w.string(try w.field(v, key));
}
fn is(v: V, key: []const u8, value: []const u8) bool {
    const x = v.object.get(key) orelse return false;
    return x == .string and std.mem.eql(u8, x.string, value);
}
fn product(a: A, x: Big, y: Big) !Big {
    var r = try Big.init(a);
    try r.mul(&x, &y);
    return r;
}
fn power(a: A, n: u32) !Big {
    var r = try Big.init(a);
    try r.pow(&try Big.initSet(a, 10), n);
    return r;
}
const Decimal = struct {
    units: Big,
    scale: u32,
    fn parse(a: A, s: []const u8) !Decimal {
        if (s.len == 0 or s.len > 128) return error.InvalidDecimal;
        var it = std.mem.splitScalar(u8, s, '.');
        const whole = it.next().?;
        const fraction = it.next() orelse "";
        if (whole.len == 0 or fraction.len > 38 or it.next() != null) return error.InvalidDecimal;
        const digits = try std.mem.concat(a, u8, &.{ whole, fraction });
        for (digits) |c| if (!std.ascii.isDigit(c)) return error.InvalidDecimal;
        var n = try Big.init(a);
        try n.setString(10, digits);
        return .{ .units = n, .scale = @intCast(fraction.len) };
    }
    fn mul(x: Decimal, a: A, y: Decimal) !Decimal {
        return .{ .units = try product(a, x.units, y.units), .scale = x.scale + y.scale };
    }
    fn add(x: Decimal, a: A, y: Decimal) !Decimal {
        const scale = @max(x.scale, y.scale);
        var sum = try Big.init(a);
        try sum.add(&try product(a, x.units, try power(a, scale - x.scale)), &try product(a, y.units, try power(a, scale - y.scale)));
        return .{ .units = sum, .scale = scale };
    }
    fn native(x: Decimal, a: A, res: u64, up: bool, exact: bool) !u64 {
        if (res == 0) return error.InvalidResolution;
        var q = try Big.init(a);
        var rem = try Big.init(a);
        try q.divTrunc(&rem, &try product(a, x.units, try Big.initSet(a, res)), &try power(a, x.scale));
        if (!rem.eqlZero()) {
            if (exact) return error.InvalidPrecision;
            if (up) try q.addScalar(&q, 1);
        }
        return q.toInt(u64);
    }
};
fn positionQty(a: A, ctx: V, price: Decimal) !V {
    const precision = try w.unsigned(u32, try w.field(ctx, "quantity_precision"));
    if (precision > 18 or price.units.eqlZero()) return error.InvalidPrecision;
    const value = try Decimal.parse(a, try text(ctx, "max_position_value"));
    const n = try product(a, try product(a, value.units, try Big.initSet(a, 50)), try power(a, price.scale + precision));
    var q = try Big.init(a);
    var rem = try Big.init(a);
    try q.divTrunc(&rem, &n, &try product(a, price.units, try power(a, value.scale)));
    const digits = try q.toString(a, 10, .lower);
    if (precision == 0) return w.text(digits);
    if (digits.len > precision) return w.text(try std.mem.concat(a, u8, &.{ digits[0 .. digits.len - precision], ".", digits[digits.len - precision ..] }));
    const zeros = try a.alloc(u8, precision - digits.len);
    @memset(zeros, '0');
    return w.text(try std.mem.concat(a, u8, &.{ "0.", zeros, digits }));
}
fn prepare(a: A, input: V, raw: V, sign: bool) !V {
    const ctx = try w.field(input, "context");
    var body = try asValue(a, raw);
    if (body != .object) return error.InvalidOrder;
    const qty = try Decimal.parse(a, try text(body, "qty"));
    const price = try Decimal.parse(a, try text(body, "price"));
    const fee = try Decimal.parse(a, try text(body, "fee"));
    const builder = body.object.get("builderFee") orelse .null;
    const total_fee = try fee.add(a, try Decimal.parse(a, if (builder == .string) builder.string else "0"));
    const buy = is(body, "side", "BUY");
    if (!buy and !is(body, "side", "SELL")) return error.InvalidSide;
    const base_res = try w.unsigned(u64, try w.field(ctx, "synthetic_resolution"));
    const quote_res = try w.unsigned(u64, try w.field(ctx, "collateral_resolution"));
    const notional = try qty.mul(a, price);
    const base = std.math.cast(i64, try qty.native(a, base_res, buy, true)) orelse return error.Overflow;
    const quote = std.math.cast(i64, try notional.native(a, quote_res, buy, false)) orelse return error.Overflow;
    if ((base == 0 or quote == 0) and !is(body, "type", "TPSL")) return error.InvalidOrder;
    const fee_amount = try (try notional.mul(a, total_fee)).native(a, quote_res, true, false);
    const position = try w.unsigned(u32, try w.field(ctx, "collateral_position"));
    const collateral = try text(ctx, "collateral_asset_id");
    const ms = try w.unsigned(u64, try w.field(body, "expiryEpochMillis"));
    const msg = try asValue(a, .{ .position_id = .{ .value = position }, .base_asset_id = .{ .value = try text(ctx, "synthetic_asset_id") }, .base_amount = if (buy) base else -base, .quote_asset_id = .{ .value = collateral }, .quote_amount = if (buy) -quote else quote, .fee_asset_id = .{ .value = collateral }, .fee_amount = fee_amount, .expiration = .{ .seconds = try std.math.divCeil(u64, ms, 1000) + 14 * 86400 }, .salt = try text(body, "nonce") });
    const pubkey = try text(input, "public_key");
    var args = try asValue(a, .{ .kind = "order", .chain_id = try text(input, "chain_id"), .public_key = pubkey, .message = msg });
    if (sign) try args.object.put(a, "stark_key", try w.field(input, "stark_key"));
    const result = try @import("extended_settlement.zig").settle(a, args);
    const digest = try text(result, "hash");
    if (!body.object.contains("id")) try body.object.put(a, "id", try number(a, try stark.felt(digest)));
    const debugging = try asValue(a, .{ .collateralAmount = try number(a, if (buy) -quote else quote), .syntheticAmount = try number(a, if (buy) base else -base), .feeAmount = try number(a, fee_amount) });
    try body.object.put(a, "debuggingAmounts", debugging);
    var settlement = try asValue(a, .{ .starkKey = try std.fmt.allocPrint(a, "0x{x}", .{try stark.felt(pubkey)}), .collateralPosition = try number(a, position) });
    if (sign) try settlement.object.put(a, "signature", try w.field(result, "signature"));
    try body.object.put(a, "settlement", settlement);
    return asValue(a, .{ .hash = digest, .body = body });
}
pub fn order(a: A, input: V) !V {
    const raw = try w.field(input, "body");
    if (raw != .object) return error.InvalidOrder;
    var secret = if (input.object.get("stark_key")) |key| try stark.felt(try w.string(key)) else if (input.object.get("wallet_signature")) |sig| try @import("extended_onboarding.zig").grind(try w.string(sig)) else return prepare(a, input, raw, false);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&secret));
    const public = try stark.publicKey(secret);
    var args: V = .{ .object = .empty };
    const key_text = try std.fmt.allocPrint(a, "0x{x}", .{secret});
    defer std.crypto.secureZero(u8, key_text);
    try args.object.put(a, "stark_key", w.text(key_text));
    for ([_][]const u8{ "chain_id", "context" }) |name|
        try args.object.put(a, name, try w.field(input, name));
    try args.object.put(a, "public_key", w.text(try std.fmt.allocPrint(a, "0x{x}", .{public})));
    const tpsl = is(raw, "type", "TPSL");
    var body = try w.field(try prepare(a, args, raw, !tpsl), "body");
    if (tpsl) try body.object.put(a, "settlement", .null);
    for ([_][]const u8{ "takeProfit", "stopLoss" }) |name| {
        const original = raw.object.get(name) orelse continue;
        if (original == .null) continue;
        if (original != .object) return error.InvalidOrder;
        var leg = try asValue(a, original);
        var child = try asValue(a, raw);
        _ = child.object.swapRemove("id");
        try child.object.put(a, "type", w.text("LIMIT"));
        const buy = is(raw, "side", "BUY");
        try child.object.put(a, "side", w.text(if (buy == tpsl) "BUY" else "SELL"));
        try child.object.put(a, "price", try w.field(leg, "price"));
        if (is(raw, "tpSlType", "POSITION")) {
            const qty = try positionQty(a, try w.field(input, "context"), try Decimal.parse(a, try text(leg, "price")));
            try child.object.put(a, "qty", qty);
        } else if (!is(raw, "tpSlType", "ORDER")) return error.InvalidOrder;
        const signed = try w.field(try prepare(a, args, child, true), "body");
        for ([_][]const u8{ "settlement", "debuggingAmounts" }) |k| try leg.object.put(a, k, try w.field(signed, k));
        try body.object.put(a, name, leg);
    }
    return body;
}
