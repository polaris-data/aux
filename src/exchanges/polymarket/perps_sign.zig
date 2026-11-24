// Polymarket Perps signs canonical MessagePack inside a Polygon EIP-712 domain.
const std = @import("std");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const unsigned = w.unsigned;
const V = w.V;
const A = w.A;
const Word = [32]u8;
const Bytes = std.ArrayList(u8);
const eq = std.mem.eql;

fn array(a: A) V {
    return .{ .array = std.array_list.Managed(V).init(a) };
}

fn pick(a: A, v: V, fields: []const []const u8) !V {
    if (v != .object) return error.ExpectedObject;
    var result = array(a);
    for (fields) |field| {
        if (v.object.get(field)) |value| if (value != .null) try result.array.append(value);
    }
    return result;
}

fn orders(a: A, args: V) !V {
    if (args != .array) return error.ExpectedArray;
    var rows = array(a);
    for (args.array.items) |order| {
        if (order != .object) return error.ExpectedObject;
        var row = array(a);
        for ([_][]const u8{ "iid", "buy", "p", "qty", "tif", "po", "ro", "c", "tr", "gtd_expiry", "builder" }) |field| {
            const v = order.object.get(field) orelse .null;
            if (v == .null) {
                if (eq(u8, field, "po")) try row.array.append(.{ .bool = false });
                continue;
            }
            if (eq(u8, field, "ro") and v == .bool and !v.bool) continue;
            const value = if (eq(u8, field, "tr")) try pick(a, v, &.{ "market", "trp", "tpsl", "trail_bps", "act" }) else if (eq(u8, field, "builder")) try pick(a, v, &.{ "address", "fee_rate" }) else v;
            try row.array.append(value);
        }
        try rows.array.append(row);
    }
    return rows;
}

const Kind = enum {
    createOrders,
    cancelOrders,
    cancelOrdersCOID,
    modifyOrders,
    modifyOrdersCOID,
    createChase,
    cancelChase,
    createTwap,
    controlTwap,
    cancelTwap,
    cancelAll,
    autoCancel,
    updateLeverage,
    updateLeverages,
    updateMargin,
    deleteProxy,
    createProxy,
    withdraw,
    updateBuilder,
    approveBuilder,
    internalTransfer,
};

fn canonical(a: A, kind: Kind, op: V, args: V) !V {
    const values = switch (kind) {
        .createOrders => try orders(a, args),
        .cancelOrders, .cancelOrdersCOID => args,
        .modifyOrders, .modifyOrdersCOID, .updateLeverages => blk: {
            if (args != .array) return error.ExpectedArray;
            const fields: []const []const u8 = switch (kind) {
                .modifyOrders => &.{ "oid", "p", "qty" },
                .modifyOrdersCOID => &.{ "coid", "p", "qty" },
                else => &.{ "iid", "lev", "cross" },
            };
            var rows = array(a);
            for (args.array.items) |row| try rows.array.append(try pick(a, row, fields));
            break :blk rows;
        },
        .createChase => try pick(a, args, &.{ "iid", "buy", "qty", "lim", "max_dist", "max_dist_bps", "po", "ro", "c" }),
        .cancelChase => try pick(a, args, &.{"chid"}),
        .createTwap => try pick(a, args, &.{ "iid", "buy", "qty", "dur", "ivl", "rnd", "slip_bps", "min_px", "max_px", "ro", "c" }),
        .controlTwap => try pick(a, args, &.{ "twid", "act" }),
        .cancelTwap => try pick(a, args, &.{"twid"}),
        .cancelAll => try pick(a, args, &.{"iid"}),
        .autoCancel => try pick(a, args, &.{"time"}),
        .updateLeverage => try pick(a, args, &.{ "iid", "lev", "cross" }),
        .updateMargin => try pick(a, args, &.{ "iid", "amt" }),
        .updateBuilder => try pick(a, args, &.{"name"}),
        .approveBuilder => try pick(a, args, &.{ "builder", "max_fee_rate", "approval_version" }),
        .internalTransfer => try pick(a, args, &.{ "account", "token", "amount", "to" }),
        .deleteProxy => try pick(a, args, &.{"proxy"}),
        .createProxy, .withdraw => unreachable,
    };
    var result = array(a);
    try result.array.append(w.text(@tagName(kind)));
    try result.array.append(values);
    if (op.object.get("grp")) |grp| if (grp != .null) try result.array.append(grp);
    return result;
}

// Prefix a fixed-width MessagePack value with its format byte.
fn tagged(a: A, out: *Bytes, tag: u8, comptime T: type, value: anytype) !void {
    var b: [@sizeOf(T)]u8 = undefined;
    std.mem.writeInt(T, &b, @intCast(value), .big);
    try out.append(a, tag);
    try out.appendSlice(a, &b);
}

fn pack(a: A, out: *Bytes, v: V) anyerror!void {
    switch (v) {
        .null => try out.append(a, 0xc0),
        .bool => |b| try out.append(a, if (b) 0xc3 else 0xc2),
        .string => |s| {
            if (s.len < 32) {
                try out.append(a, 0xa0 + @as(u8, @intCast(s.len)));
            } else if (s.len <= 255) {
                try tagged(a, out, 0xd9, u8, s.len);
            } else if (s.len <= 65535) {
                try tagged(a, out, 0xda, u16, s.len);
            } else {
                try tagged(a, out, 0xdb, u32, s.len);
            }
            try out.appendSlice(a, s);
        },
        .array => |xs| {
            const n = xs.items.len;
            if (n < 16) {
                try out.append(a, 0x90 + @as(u8, @intCast(n)));
            } else if (n <= 65535) {
                try tagged(a, out, 0xdc, u16, n);
            } else {
                try tagged(a, out, 0xdd, u32, n);
            }
            for (xs.items) |x| try pack(a, out, x);
        },
        .number_string => |digits| {
            if (digits[0] == '-') {
                const n = try std.fmt.parseInt(i64, digits, 10);
                if (n >= -32) {
                    try out.append(a, @bitCast(@as(i8, @intCast(n))));
                } else if (n >= -128) {
                    try tagged(a, out, 0xd0, i8, n);
                } else if (n >= -32768) {
                    try tagged(a, out, 0xd1, i16, n);
                } else if (n >= std.math.minInt(i32)) {
                    try tagged(a, out, 0xd2, i32, n);
                } else {
                    try tagged(a, out, 0xd3, i64, n);
                }
            } else {
                const n = try unsigned(u64, v);
                if (n < 128) {
                    try out.append(a, @intCast(n));
                } else if (n <= 255) {
                    try tagged(a, out, 0xcc, u8, n);
                } else if (n <= 65535) {
                    try tagged(a, out, 0xcd, u16, n);
                } else if (n <= std.math.maxInt(u32)) {
                    try tagged(a, out, 0xce, u32, n);
                } else {
                    try tagged(a, out, 0xcf, u64, n);
                }
            }
        },
        else => return error.InvalidSignableValue,
    }
}

fn addr(v: V, name: []const u8) !Word {
    return evm.address(try w.string(try w.field(v, name)));
}

pub fn operationHash(a: A, op: V) !Word {
    const signable = if (op == .array) op else block: {
        const kind = std.meta.stringToEnum(Kind, try w.string(try w.field(op, "type"))) orelse return error.InvalidOperation;
        break :block try canonical(a, kind, op, try w.field(op, "args"));
    };
    var bytes: Bytes = .empty;
    try pack(a, &bytes, signable);
    return evm.hash(bytes.items);
}

fn digest(a: A, kind: Kind, op: V, auth: V, salt: u64, ts: u64) !Word {
    const h = evm.hash;
    const word = evm.encodeWord;
    const domain_type = if (kind == .withdraw) h("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)") else h("EIP712Domain(string name,string version,uint256 chainId)");
    const prefix = domain_type ++ h("Polymarket") ++ h("1") ++ word(137);
    const domain = if (kind == .withdraw) h(&(prefix ++ try addr(auth, "withdrawal_contract"))) else h(&prefix);
    const args = try w.field(op, "args");
    const suffix = word(salt) ++ word(ts);
    const body = switch (kind) {
        .createProxy => h(&(h("CreateProxy(address addr,uint64 exp,uint64 salt,uint64 ts)") ++ try addr(args, "proxy") ++ word(try unsigned(u64, try w.field(args, "expiry"))) ++ suffix)),
        .withdraw => blk: {
            const v = try w.field(args, "amount");
            const amount = if (v == .string and std.mem.startsWith(u8, v.string, "0x")) try std.fmt.parseInt(u256, v.string[2..], 16) else try unsigned(u256, v);
            break :blk h(&(h("Withdraw(address account,address token,uint256 amount,uint256 fee,address to,uint64 salt,uint64 ts)") ++ try addr(args, "account") ++ try addr(args, "token") ++ word(amount) ++ word(0) ++ try addr(args, "to") ++ suffix));
        },
        else => h(&(h("Op(bytes32 data,uint64 salt,uint64 ts)") ++ try operationHash(a, op) ++ suffix)),
    };
    return h(&([_]u8{ 0x19, 0x01 } ++ domain ++ body));
}

pub fn sign(a: A, body: *V, auth: V, expected: []const u8, now_ms: u64, io: std.Io) !void {
    const op = try w.field(body.*, "op");
    const action = try w.string(try w.field(op, "type"));
    if (!eq(u8, expected, action)) return error.InvalidOperation;
    const kind = std.meta.stringToEnum(Kind, action) orelse return error.InvalidOperation;
    if (auth != .object) return error.AuthenticationRequired;
    if (body.object.get("sig")) |v| if (v == .string and v.string.len > 0) {
        _ = try evm.hex(65, v.string);
        return;
    };
    const owner = kind == .createProxy or kind == .deleteProxy or kind == .withdraw or kind == .updateBuilder or kind == .approveBuilder or (kind == .internalTransfer and auth.object.contains("owner_key"));
    const secret = auth.object.get(if (owner) "owner_key" else "proxy_key") orelse return error.AuthenticationRequired;
    var key = try evm.hex(32, try w.string(secret));
    defer std.crypto.secureZero(u8, &key);
    if (!owner and (auth.object.contains("proxy") or auth.object.contains("secret") or auth.object.contains("expires_at_ms"))) {
        var checked: V = .{ .object = .empty };
        try @import("../../core/venue_auth.zig").proxyHeaders(a, &checked, auth, now_ms);
        const public = (try std.crypto.ecc.Secp256k1.basePoint.mul(key, .big)).toUncompressedSec1();
        if (!eq(u8, evm.hash(public[1..])[12..], (try addr(auth, "proxy"))[12..])) return error.AuthenticationRequired;
    }
    const salt_val = body.object.get("salt") orelse .null;
    const salt = if (salt_val != .null) try unsigned(u64, salt_val) else blk: {
        var bytes: [4]u8 = undefined;
        try io.randomSecure(&bytes);
        break :blk @as(u64, std.mem.readInt(u32, &bytes, .little));
    };
    const ts_val = body.object.get("ts") orelse .null;
    const ts = if (ts_val != .null) try unsigned(u64, ts_val) else if (kind == .withdraw) now_ms / 1000 else now_ms;
    const sig = try @import("../../core/transaction.zig").recoverable(key, try digest(a, kind, op, auth, salt, ts));
    try body.object.put(a, "sig", w.text(try std.fmt.allocPrint(a, "0x{x}", .{&sig})));
    try body.object.put(a, "salt", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{salt}) });
    try body.object.put(a, "ts", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{ts}) });
}

test "MessagePack preserves signed and unsigned integer widths" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    var out: Bytes = .empty;
    try pack(a, &out, try w.parse(a, "[-33,-32,127,128,256,65536,4294967296,18446744073709551615]"));
    try std.testing.expectEqualSlices(u8, try evm.fromHex(a, "98d0dfe07fcc80cd0100ce00010000cf0000000100000000cfffffffffffffffff"), out.items);
    try std.testing.expectError(error.InvalidQuantity, pack(a, &out, try w.parse(a, "1.25")));
}
