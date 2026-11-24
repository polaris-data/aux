const std = @import("std");
const w = @import("wire.zig");
const V = w.V;
const eq = std.mem.eql;
fn number(v: V) !u64 {
    if (v != .number_string) return error.ExpectedInteger;
    for (v.number_string) |c| if (!std.ascii.isDigit(c)) return error.ExpectedInteger;
    return std.fmt.parseInt(u64, v.number_string, 10);
}
fn address(v: V) !void {
    const s = try w.string(v);
    if (s.len != 42 or !std.mem.startsWith(u8, s, "0x")) return error.InvalidAddress;
    for (s[2..]) |b| if (!std.ascii.isHex(b)) return error.InvalidAddress;
}
pub fn hyperliquid(a: w.A, name: []const u8, params: V) !V {
    const book = eq(u8, name, "l2_book");
    const meta = eq(u8, name, "meta");
    const tag = if (book) "l2Book" else if (meta) "meta" else if (eq(u8, name, "open_orders")) "openOrders" else "clearinghouseState";
    var out: V = .{ .object = .empty };
    try out.object.put(a, "type", w.text(tag));
    if (book) {
        const coin = try w.string(try w.field(params, "coin"));
        if (std.mem.trim(u8, coin, " \r\n\t").len == 0) return error.InvalidCoin;
        const figs = if (params.object.get("nSigFigs")) |v| if (v != .null) try number(v) else null else null;
        if (figs) |n| if (n < 2 or n > 5) return error.InvalidPrecision;
        if (params.object.get("mantissa")) |v| if (v != .null) {
            const n = try number(v);
            if (figs != 5 or (n != 1 and n != 2 and n != 5)) return error.InvalidPrecision;
        };
    } else if (!meta) try address(try w.field(params, "user"));
    var iter = params.object.iterator();
    while (iter.next()) |entry| {
        const k = entry.key_ptr.*;
        const allowed = if (book) eq(u8, k, "coin") or eq(u8, k, "nSigFigs") or eq(u8, k, "mantissa") else eq(u8, k, "dex") or (!meta and eq(u8, k, "user"));
        if (!allowed) return error.UnknownParameter;
        if (entry.value_ptr.* == .null) continue;
        if (eq(u8, k, "dex")) _ = try w.string(entry.value_ptr.*);
        try out.object.put(a, k, entry.value_ptr.*);
    }
    return out;
}
pub fn lighter(name: []const u8, params: V) !void {
    try bounded(params, "account_index", 0, (1 << 48) - 2);
    try bounded(params, "api_key_index", 0, 254);
    if (params.object.get("market_id")) |v| if (v != .null) {
        const n = try number(v);
        if (n >= 1 << 48 or n == 255) return error.InvalidMarket;
    };
    if (params.object.get("limit")) |v| {
        const n = try number(v);
        if (n == 0 or n > @as(u64, if (eq(u8, name, "recent_trades")) 100 else 250)) return error.InvalidLimit;
    }
    if (eq(u8, name, "account")) {
        const by = try w.string(try w.field(params, "by"));
        const v = try w.field(params, "value");
        if (eq(u8, by, "index")) {
            if (try number(v) >= (1 << 48) - 1) return error.InvalidAccount;
        } else if (eq(u8, by, "l1_address")) try address(v) else return error.InvalidAccount;
    }
}
pub fn dune(params: V) !void {
    if (params.object.get("query_id")) |v| if (try number(v) == 0) return error.InvalidQuery;
    if (params.object.get("execution_id")) |v| {
        for (try w.string(v)) |b| if (std.ascii.isWhitespace(b)) return error.InvalidPath;
    }
    if (params.object.get("sample_count")) |v| if (v != .null) {
        for ([_][]const u8{ "filters", "limit", "offset" }) |k| if (params.object.get(k)) |x| if (x != .null) return error.InvalidQuery;
    };
}

fn bounded(params: V, k: []const u8, min: u64, max: u64) !void {
    if (params.object.get(k)) |v| if (v != .null) {
        const n = try number(v);
        if (n < min or n > max) return error.InvalidLimit;
    };
}
pub fn validateSpot(a: w.A, venue: @import("routes.zig").Venue, name: []const u8, params: *V) !void {
    for ([_][]const u8{ "ticker", "market", "symbol" }) |k| if (params.object.get(k)) |v| if (v != .null) {
        if (std.mem.trim(u8, try w.string(v), " \r\n\t").len == 0) return error.InvalidSymbol;
    };
    if (venue == .arcus) {
        if (eq(u8, name, "trades")) try bounded(params.*, "limit", 1, 1000);
        try bounded(params.*, "nLevels", 1, 100);
        try bounded(params.*, "accountIndex", 0, 9);
        if (params.object.get("address")) |v| {
            var s = try w.string(v);
            if (std.mem.startsWith(u8, s, "0x") or std.mem.startsWith(u8, s, "0X")) s = s[2..];
            if (s.len != 40) return error.InvalidAddress;
            for (s) |c| if (!std.ascii.isHex(c)) return error.InvalidAddress;
            try params.object.put(a, "address", w.text(try std.mem.concat(a, u8, &.{ "0x", try std.ascii.allocLowerString(a, s) })));
        }
    } else if (venue == .metric) {
        try bounded(params.*, "chainId", 1, std.math.maxInt(u64));
        try @import("../exchanges/metric/http.zig").validate(a, name, params.*);
        if (params.object.get("pool")) |v| try address(v);
    } else if (venue == .monaco) {
        try bounded(params.*, "levels", 1, 1000);
        try bounded(params.*, "pageSize", 1, if (eq(u8, name, "trading_pairs")) 100 else 1000);
        try bounded(params.*, "page", 1, std.math.maxInt(u64));
        for ([_]([]const u8){ "marketType", "tradingMode" }) |k| if (params.object.get(k)) |v| if (v != .null) {
            const s = try w.string(v);
            if (!eq(u8, s, "SPOT") and !eq(u8, s, "MARGIN")) return error.InvalidMarket;
        };
        try bounded(params.*, "startTime", 0, 4102444800000);
        try bounded(params.*, "endTime", 0, 4102444800000);
        for ([_][]const u8{ "tradingPairId", "marginAccountId", "orderId" }) |key| if (params.object.get(key)) |v| {
            if (v == .null) continue;
            const s = try w.string(v);
            if (s.len != 36) return error.InvalidUuid;
            for (s, 0..) |c, i| if (if (i == 8 or i == 13 or i == 18 or i == 23) c != '-' else !std.ascii.isHex(c)) return error.InvalidUuid;
        };
    }
}
