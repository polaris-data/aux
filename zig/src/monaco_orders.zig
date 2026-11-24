// Wire validation for session-signed order workflows. Market metadata stays local.
const std = @import("std");
const w = @import("wire.zig");
const A = w.A;
const V = w.V;
const eq = std.mem.eql;
const Op = enum { refresh_session, revoke_session, create_challenge, verify_signature, place_order, replace_order, cancel_order, batch_create, batch_replace, batch_cancel, cancel_all };
fn get(v: V, k: []const u8) V {
    return if (v == .object) v.object.get(k) orelse .null else .null;
}
fn text(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn matches(v: V, s: []const u8) bool {
    return v == .string and eq(u8, v.string, s);
}
fn choice(v: V, choices: []const []const u8) !void {
    for (choices) |s| if (matches(v, s)) return;
    return error.InvalidOption;
}
pub fn uuid(v: V) !void {
    const s = try w.string(v);
    if (s.len != 36) return error.InvalidUuid;
    for (s, 0..) |c, i| if (if (i == 8 or i == 13 or i == 18 or i == 23) c != '-' else !std.ascii.isHex(c)) return error.InvalidUuid;
}
fn decimal(v: V) ![]const u8 {
    const s = try w.string(v);
    if (s.len == 0 or s[0] == '.' or s[s.len - 1] == '.') return error.InvalidDecimal;
    var dot = false;
    var positive = false;
    for (s) |c| {
        if (c == '.' and !dot) {
            dot = true;
            continue;
        }
        if (!std.ascii.isDigit(c)) return error.InvalidDecimal;
        if (c != '0') positive = true;
    }
    if (!positive) return error.InvalidDecimal;
    return s;
}
fn compare(x: []const u8, y: []const u8) std.math.Order {
    var xs = std.mem.splitScalar(u8, x, '.');
    var ys = std.mem.splitScalar(u8, y, '.');
    const a = std.mem.trimStart(u8, xs.next().?, "0");
    const b = std.mem.trimStart(u8, ys.next().?, "0");
    if (a.len != b.len) return std.math.order(a.len, b.len);
    const first = std.mem.order(u8, a, b);
    if (first != .eq) return first;
    return std.mem.order(u8, std.mem.trimEnd(u8, xs.next() orelse "", "0"), std.mem.trimEnd(u8, ys.next() orelse "", "0"));
}
fn units(x: []const u8, y: []const u8) !void {
    var numbers = [2]u128{ 0, 0 };
    var scales = [2]usize{ 0, 0 };
    for ([_][]const u8{ x, y }, 0..) |value, i| {
        var dot = false;
        for (value) |c| {
            if (c == '.') {
                dot = true;
                continue;
            }
            if (dot) scales[i] += 1;
            numbers[i] = try std.math.add(u128, try std.math.mul(u128, numbers[i], 10), c - '0');
        }
    }
    const i: usize = @intFromBool(scales[1] < scales[0]);
    for (0..@max(scales[0], scales[1]) - scales[i]) |_| numbers[i] = try std.math.mul(u128, numbers[i], 10);
    if (numbers[0] % numbers[1] != 0 or numbers[0] / numbers[1] > std.math.maxInt(i64)) return error.InvalidIncrement;
}
fn range(m: V, x: []const u8, min: []const u8, max: []const u8) !void {
    if (get(m, min) != .null and compare(x, try decimal(get(m, min))) == .lt) return error.BelowMinimum;
    if (get(m, max) != .null and compare(x, try decimal(get(m, max))) == .gt) return error.AboveMaximum;
}
fn identifier(v: V, trim: bool) !void {
    const raw = try w.string(v);
    const s = if (trim) std.mem.trim(u8, raw, " \r\n\t") else raw;
    if ((s.len == 0 and !trim) or s.len > 64) return error.InvalidId;
    for (s) |c| if (!std.ascii.isAlphanumeric(c) and std.mem.findScalar(u8, "._:-", c) == null) return error.InvalidId;
}
fn allow(body: V, keys: []const []const u8) !void {
    if (body != .object) return error.InvalidOrder;
    for (body.object.keys()) |key| {
        for (keys) |allowed| {
            if (eq(u8, key, allowed)) break;
        } else return error.UnknownParameter;
    }
}

pub fn isWrite(name: []const u8) bool {
    const op = std.meta.stringToEnum(Op, name) orelse return false;
    return switch (op) {
        .place_order, .replace_order, .cancel_order, .batch_create, .batch_replace, .batch_cancel, .cancel_all => true,
        else => false,
    };
}

const create_fields = [_][]const u8{ "tradingPairId", "orderType", "side", "price", "quantity", "tradingMode", "slippageToleranceBps", "useMasterBalance", "expirationDate", "timeInForce", "marginAccountId", "riskBucketId", "riskBucketCollateral", "positionSide", "leverage", "reduceOnly", "strategyKey", "marginMode", "postOnly", "clientOrderId", "idempotencyKey", "selfTradePreventionMode" };
const single_create_fields = create_fields ++ [_][]const u8{ "takeProfit", "stopLoss", "trailingStop" };
const replace_fields = [_][]const u8{ "selfTradePreventionMode", "price", "quantity", "useMasterBalance", "postOnly", "clientOrderId" };
const batch_replace_fields = replace_fields ++ [_][]const u8{"orderId"};

fn checked(body: V, m: V, create: bool, batched: bool) !void {
    try allow(body, if (create) (if (batched) &create_fields else &single_create_fields) else (if (batched) &batch_replace_fields else &replace_fields));
    if (body != .object or m != .object) return error.InvalidOrder;
    for ([_][]const u8{ "marginAccountId", "riskBucketId" }) |k| if (get(body, k) != .null) try uuid(get(body, k));
    if (get(body, "idempotencyKey") != .null) try identifier(get(body, "idempotencyKey"), false);
    if (get(body, "clientOrderId") != .null) try identifier(get(body, "clientOrderId"), true);
    if (get(body, "selfTradePreventionMode") != .null) try choice(get(body, "selfTradePreventionMode"), &.{ "CANCEL_MAKER", "CANCEL_TAKER", "CANCEL_BOTH", "SKIP" });
    if (get(body, "marginMode") != .null) try choice(get(body, "marginMode"), &.{ "ISOLATED", "CROSS" });
    for ([_][]const u8{ "useMasterBalance", "reduceOnly", "postOnly" }) |k| if (get(body, k) != .null and get(body, k) != .bool) return error.InvalidBool;
    for ([_][]const u8{ "leverage", "riskBucketCollateral" }) |k| if (get(body, k) != .null) {
        _ = try decimal(get(body, k));
    };
    for ([_][]const u8{ "takeProfit", "stopLoss", "trailingStop" }) |k| if (get(body, k) != .null) {
        const leg = get(body, k);
        if (leg != .object) return error.InvalidOrder;
        for ([_][]const u8{ "triggerPrice", "limitPrice", "quantity", "trailingDelta", "activationPrice" }) |k2| if (get(leg, k2) != .null) {
            _ = try decimal(get(leg, k2));
        };
    };
    if (get(body, "price") != .null) try units(try decimal(get(body, "price")), try decimal(get(m, "tickSize")));
    if (get(body, "quantity") != .null) {
        const qty = try decimal(get(body, "quantity"));
        try units(qty, try decimal(get(m, "quantityStepSize")));
        try range(m, qty, "minOrderSize", "maxOrderSize");
    } else if (create) return error.InvalidOrder;
    if (!create) return;
    const pair = try w.field(body, "tradingPairId");
    try uuid(pair);
    if (!matches(get(m, "id"), pair.string)) return error.InvalidMarket;
    const kind = try w.field(body, "orderType");
    try choice(kind, &.{ "LIMIT", "MARKET" });
    const side = try w.field(body, "side");
    try choice(side, &.{ "BUY", "SELL" });
    const tif = get(body, "timeInForce");
    const post = get(body, "postOnly") == .bool and get(body, "postOnly").bool;
    if (matches(kind, "LIMIT") and get(body, "price") == .null) return error.InvalidPrice;
    if (matches(kind, "MARKET") and (get(body, "price") != .null or tif != .null or post)) return error.InvalidOrder;
    if (tif != .null) {
        try choice(tif, &.{ "GTC", "IOC", "FOK" });
        if (post and !matches(tif, "GTC")) return error.InvalidOrder;
    }
    if (get(body, "slippageToleranceBps") != .null) {
        const n = get(body, "slippageToleranceBps");
        if (n != .number_string or try w.unsigned(u32, n) > 10000) return error.InvalidSlippage;
    }
    const mode = if (get(body, "tradingMode") == .null) get(m, "marketType") else get(body, "tradingMode");
    try choice(mode, &.{ "MARGIN", "SPOT" });
    if (get(m, "marketType") != .null and !matches(get(m, "marketType"), mode.string)) return error.InvalidMarket;
    if (get(body, "positionSide") != .null and !matches(get(body, "positionSide"), if (matches(side, "BUY")) "LONG" else "SHORT")) return error.InvalidSide;
    if (matches(mode, "MARGIN") and !(get(body, "reduceOnly") == .bool and get(body, "reduceOnly").bool)) {
        if (get(m, "isolatedOnly") == .bool and get(m, "isolatedOnly").bool and matches(get(body, "marginMode"), "CROSS")) return error.InvalidMargin;
        try range(m, try decimal(get(body, "leverage")), "minLeverage", "maxLeverage");
    }
}
fn batch(p: V, k: []const u8) ![]V {
    const items = try w.field(p, k);
    if (items != .array or items.array.items.len == 0 or items.array.items.len > 100) return error.InvalidBatch;
    return items.array.items;
}
pub fn prepare(a: A, op_name: []const u8, p: V, req: *@import("http.zig").Request) !void {
    const op = std.meta.stringToEnum(Op, op_name) orelse return error.InvalidOperation;
    if (p != .object) return error.InvalidParameter;
    const body = if (op == .place_order or op == .replace_order or op == .batch_create or op == .batch_replace) try w.field(p, "body") else p;
    if (body != .object) return error.InvalidOrder;
    req.body = body;
    switch (op) {
        .refresh_session, .revoke_session => if (body.object.count() != 0) return error.UnknownParameter,
        .create_challenge, .verify_signature => {
            try allow(body, if (op == .create_challenge) &.{ "address", "clientId", "chainId", "sessionPublicKey" } else &.{ "address", "signature", "nonce", "clientId", "chainId", "sessionPublicKey", "referralCode", "inviteCode" });
            const address = try text(body, "address");
            if (address.len != 42 or !std.mem.startsWith(u8, address, "0x")) return error.InvalidAddress;
            _ = try @import("evm.zig").hex(20, address);
            const key = try text(body, "sessionPublicKey");
            if (key.len != 64) return error.InvalidKey;
            _ = try @import("evm.zig").hex(32, key);
            if (op == .verify_signature) {
                if (std.mem.trim(u8, try text(body, "nonce"), " \t\r\n").len == 0) return error.InvalidNonce;
                const sig = try text(body, "signature");
                const raw = if (std.mem.startsWith(u8, sig, "0x")) sig[2..] else sig;
                if (raw.len == 0 or raw.len > 16384 or raw.len % 2 != 0) return error.InvalidSignature;
                for (raw) |c| if (!std.ascii.isHex(c)) return error.InvalidSignature;
            }
        },
        .place_order, .replace_order => {
            try allow(p, if (op == .place_order) &.{ "body", "market" } else &.{ "orderId", "body", "market" });
            try checked(body, try w.field(p, "market"), op == .place_order, false);
            if (op == .replace_order) {
                const id = try w.field(p, "orderId");
                try uuid(id);
                req.path = try std.mem.concat(a, u8, &.{ "/api/v1/orders/", id.string });
            }
        },
        .batch_create, .batch_replace => {
            try allow(p, &.{ "body", "markets" });
            try allow(body, &.{"orders"});
            const ms = try w.field(p, "markets");
            const list = try batch(body, "orders");
            if (ms != .array or (op == .batch_replace and ms.array.items.len != list.len)) return error.InvalidMarket;
            for (list, 0..) |item, i| {
                if (op == .batch_replace) {
                    try uuid(try w.field(item, "orderId"));
                    try checked(item, ms.array.items[i], false, true);
                } else {
                    const id = try text(item, "tradingPairId");
                    for (ms.array.items) |m| {
                        if (matches(get(m, "id"), id)) {
                            try checked(item, m, true, true);
                            break;
                        }
                    } else return error.InvalidMarket;
                }
            }
        },
        .cancel_order => {
            try allow(body, &.{ "orderId", "remainingQuantityTarget" });
            try uuid(try w.field(body, "orderId"));
            const q = get(body, "remainingQuantityTarget");
            if (q != .null and !matches(q, "0")) _ = try decimal(q);
        },
        .batch_cancel => {
            try allow(body, &.{"orderIds"});
            for (try batch(body, "orderIds")) |id| try uuid(id);
        },
        .cancel_all => {
            try allow(p, &.{"tradingPairId"});
            req.body = .null;
            const id = get(p, "tradingPairId");
            if (id != .null) {
                try uuid(id);
                req.path = try std.mem.concat(a, u8, &.{ req.path, "/", id.string });
            }
        },
    }
}
