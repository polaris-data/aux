// Signed actions use protocol declaration order regardless of SDK map ordering.
const std = @import("std");
const w = @import("wire.zig");
const evm = @import("evm.zig");
const A = w.A;
const V = w.V;
const eq = std.mem.eql;
const Bytes = std.ArrayList(u8);
const field = w.field;
fn get(v: V, k: []const u8) V {
    return if (v == .object) v.object.get(k) orelse .null else .null;
}
fn put(a: A, v: *V, k: []const u8, x: V) !void {
    try v.object.put(a, k, x);
}
fn obj() V {
    return .{ .object = .empty };
}
fn uint(comptime T: type, v: V) !V {
    if (v != .number_string) return error.InvalidInteger;
    _ = try w.unsigned(T, v);
    return v;
}
fn boolean(v: V) !V {
    if (v != .bool) return error.InvalidBool;
    return v;
}
fn choice(v: V, choices: []const []const u8) !V {
    const s = try w.string(v);
    for (choices) |x| if (eq(u8, x, s)) return v;
    return error.InvalidOption;
}
fn hex(comptime n: usize, a: A, v: V) !V {
    const s = try w.string(v);
    if (!std.mem.startsWith(u8, s, "0x")) return error.InvalidHex;
    return w.text(try std.fmt.allocPrint(a, "0x{x}", .{try evm.hex(n, s)}));
}
fn decimal(v: V) !V {
    const s = try w.string(v);
    if (s.len == 0) return error.InvalidDecimal;
    var dot = false;
    var positive = false;
    for (s, 0..) |c, i| {
        if (c == '.') {
            if (dot or i == 0 or i + 1 == s.len or s[s.len - 1] == '0') return error.InvalidDecimal;
            dot = true;
        } else if (!std.ascii.isDigit(c)) return error.InvalidDecimal else if (c != '0') positive = true;
    }
    if (!positive or (s.len > 1 and s[0] == '0' and s[1] != '.')) return error.InvalidDecimal;
    return v;
}
fn order(a: A, v: V) !V {
    var o = obj();
    try put(a, &o, "a", try uint(u32, try field(v, "a")));
    try put(a, &o, "b", try boolean(try field(v, "b")));
    try put(a, &o, "p", try decimal(try field(v, "p")));
    try put(a, &o, "s", try decimal(try field(v, "s")));
    try put(a, &o, "r", try boolean(try field(v, "r")));
    const t = try field(v, "t");
    if (t != .object or t.object.count() != 1) return error.InvalidOrderType;
    var typ = obj();
    var detail = obj();
    if (t.object.get("limit")) |limit| {
        try put(a, &detail, "tif", try choice(try field(limit, "tif"), &.{ "Ioc", "Alo", "Gtc" }));
        try put(a, &typ, "limit", detail);
    } else {
        const trigger = try field(t, "trigger");
        try put(a, &detail, "isMarket", try boolean(try field(trigger, "isMarket")));
        try put(a, &detail, "triggerPx", try decimal(try field(trigger, "triggerPx")));
        try put(a, &detail, "tpsl", try choice(try field(trigger, "tpsl"), &.{ "tp", "sl" }));
        try put(a, &typ, "trigger", detail);
    }
    try put(a, &o, "t", typ);
    if (get(v, "c") != .null) try put(a, &o, "c", try hex(16, a, get(v, "c")));
    return o;
}
pub fn action(a: A, op: []const u8, p: V) !V {
    const orders = eq(u8, op, "order");
    const by_cloid = eq(u8, op, "cancel_by_cloid");
    if (!orders and !by_cloid and !eq(u8, op, "cancel")) return error.InvalidOperation;
    const k = if (orders) "orders" else "cancels";
    const items = try field(p, k);
    if (items != .array or items.array.items.len == 0) return error.EmptyBatch;
    var list = V{ .array = .init(a) };
    for (items.array.items) |v| {
        const item = if (orders) try order(a, v) else item: {
            var c = obj();
            if (by_cloid) {
                try put(a, &c, "asset", try uint(u32, try field(v, "asset")));
                try put(a, &c, "cloid", try hex(16, a, try field(v, "cloid")));
            } else {
                try put(a, &c, "a", try uint(u32, try field(v, "a")));
                try put(a, &c, "o", try uint(u64, try field(v, "o")));
            }
            break :item c;
        };
        try list.array.append(item);
    }
    var out = obj();
    try put(a, &out, "type", w.text(if (by_cloid) "cancelByCloid" else op));
    try put(a, &out, k, list);
    if (orders) {
        const g = get(p, "grouping");
        if (g == .object) {
            var group = obj();
            try put(a, &group, "p", try uint(u32, try field(g, "p")));
            try put(a, &out, "grouping", group);
        } else try put(a, &out, "grouping", if (g == .null) w.text("na") else try choice(g, &.{ "na", "normalTpsl", "positionTpsl" }));
        const b = get(p, "builder");
        if (b != .null) {
            var builder = obj();
            try put(a, &builder, "b", try hex(20, a, try field(b, "b")));
            try put(a, &builder, "f", try uint(u32, try field(b, "f")));
            const shares = get(b, "s");
            if (shares != .null) {
                if (shares != .array or shares.array.items.len > 5) return error.InvalidShares;
                var total: u64 = 0;
                var sub = V{ .array = .init(a) };
                for (shares.array.items) |s| {
                    var entry = obj();
                    try put(a, &entry, "b", try hex(20, a, try field(s, "b")));
                    const share = try uint(u32, try field(s, "s"));
                    const amount = try w.unsigned(u32, share);
                    if (amount == 0) return error.InvalidShares;
                    total += amount;
                    try put(a, &entry, "s", share);
                    try sub.array.append(entry);
                }
                if (total > 100000) return error.InvalidShares;
                try put(a, &builder, "s", sub);
            }
            try put(a, &out, "builder", builder);
        }
    }
    return out;
}

fn be(comptime T: type, a: A, out: *Bytes, n: u64) !void {
    var bytes: [@sizeOf(T)]u8 = undefined;
    std.mem.writeInt(T, &bytes, @intCast(n), .big);
    try out.appendSlice(a, &bytes);
}
fn sized(a: A, out: *Bytes, n: usize, fix: u8, max: u8, short: ?u8, medium: u8, large: u8) !void {
    if (n <= max) return out.append(a, fix + @as(u8, @intCast(n)));
    if (short != null and n <= 255) {
        try out.append(a, short.?);
        return be(u8, a, out, n);
    }
    if (n <= 65535) {
        try out.append(a, medium);
        return be(u16, a, out, n);
    }
    try out.append(a, large);
    try be(u32, a, out, n);
}
fn pack(a: A, out: *Bytes, v: V) anyerror!void {
    switch (v) {
        .object => |map| {
            try sized(a, out, map.count(), 0x80, 15, null, 0xde, 0xdf);
            var it = map.iterator();
            while (it.next()) |pair| {
                try pack(a, out, w.text(pair.key_ptr.*));
                try pack(a, out, pair.value_ptr.*);
            }
        },
        .array => |arr| {
            try sized(a, out, arr.items.len, 0x90, 15, null, 0xdc, 0xdd);
            for (arr.items) |item| try pack(a, out, item);
        },
        .string => |s| {
            try sized(a, out, s.len, 0xa0, 31, 0xd9, 0xda, 0xdb);
            try out.appendSlice(a, s);
        },
        .bool => |b| try out.append(a, if (b) 0xc3 else 0xc2),
        .number_string => {
            const n = try w.unsigned(u64, v);
            if (n < 128) return out.append(a, @intCast(n));
            inline for (.{ u8, u16, u32, u64 }, 0..) |T, i| if (n <= std.math.maxInt(T)) {
                try out.append(a, 0xcc + i);
                return be(T, a, out, n);
            };
        },
        else => return error.InvalidAction,
    }
}
fn digest(name: []const u8, chain_id: u64, body: [32]u8) [32]u8 {
    const domain = evm.hash(&(evm.hash("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)") ++ evm.hash(name) ++ evm.hash("1") ++ evm.encodeWord(chain_id) ++ evm.encodeWord(0)));
    return evm.hash(&([2]u8{ 0x19, 1 } ++ domain ++ body));
}
var last_nonce = std.atomic.Value(u64).init(0);
fn allocate(ms: u64) !u64 {
    var old = last_nonce.load(.monotonic);
    while (true) {
        const next = @max(try std.math.add(u64, old, 1), ms);
        old = last_nonce.cmpxchgWeak(old, next, .monotonic, .monotonic) orelse return next;
    }
}
pub fn envelope(a: A, body: V, config: V, io: std.Io) !V {
    return orderEnvelope(a, body, config, io, false);
}
pub fn signingPayload(a: A, config: V, io: std.Io) !V {
    const op = try w.string(try field(config, "operation"));
    return orderEnvelope(a, try action(a, op, try field(config, "params")), config, io, true);
}
fn orderEnvelope(a: A, body: V, config: V, io: std.Io, prepare: bool) !V {
    const auth = get(config, "auth");
    if (auth != .object) return error.AuthenticationRequired;
    const net = try choice(try field(auth, "network"), &.{ "Mainnet", "Testnet" });
    const main = eq(u8, net.string, "Mainnet");
    const p = try field(config, "params");
    const options = get(p, "options");
    if (options != .null and options != .object) return error.InvalidOptions;
    const n = get(options, "nonce");
    const nonce = if (n == .null) try allocate(@intCast(std.Io.Timestamp.now(io, .real).toMilliseconds())) else try w.unsigned(u64, try uint(u64, n));
    var bytes: Bytes = .empty;
    try pack(a, &bytes, body);
    try be(u64, a, &bytes, nonce);
    var out = obj();
    try put(a, &out, "action", body);
    try put(a, &out, "nonce", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{nonce}) });
    const vault = get(auth, "vault_address");
    if (vault == .null) {
        try bytes.append(a, 0);
    } else {
        const addr = try hex(20, a, vault);
        try bytes.append(a, 1);
        try bytes.appendSlice(a, &try evm.hex(20, addr.string));
        try put(a, &out, "vaultAddress", addr);
    }
    const expiry = get(options, "expires_after");
    if (expiry != .null) {
        const ms = try w.unsigned(u64, try uint(u64, expiry));
        if (ms <= nonce) return error.InvalidExpiry;
        try bytes.append(a, 0);
        try be(u64, a, &bytes, ms);
        try put(a, &out, "expiresAfter", expiry);
    }
    const agent = evm.hash(&(evm.hash("Agent(string source,bytes32 connectionId)") ++ evm.hash(if (main) "a" else "b") ++ evm.hash(bytes.items)));
    const hash = digest("Exchange", 1337, agent);
    if (prepare) {
        var payload = obj();
        try put(a, &payload, "nonce", try field(out, "nonce"));
        try put(a, &payload, "digest", w.text(try std.fmt.allocPrint(a, "0x{x}", .{&hash})));
        return payload;
    }
    try put(a, &out, "signature", try sign(a, auth, hash));
    return out;
}
fn sign(a: A, auth: V, hash: [32]u8) !V {
    if (get(auth, "key") == .null) {
        const signature = get(auth, "signature");
        if (signature != .object) return error.AuthenticationRequired;
        const tx = @import("transaction.zig");
        const r = try tx.quantity(u256, try field(signature, "r"));
        const s = try tx.quantity(u256, try field(signature, "s"));
        const v = try w.unsigned(u8, try field(signature, "v"));
        if (v != 27 and v != 28) return error.InvalidSignature;
        const signer = try tx.recoverAddress(hash, r, s, v - 27);
        const expected = try evm.hex(20, try w.string(try field(auth, "address")));
        if (!std.mem.eql(u8, &expected, &signer)) return error.InvalidSignature;
        var result = obj();
        try put(a, &result, "r", w.text(try std.fmt.allocPrint(a, "0x{x}", .{r})));
        try put(a, &result, "s", w.text(try std.fmt.allocPrint(a, "0x{x}", .{s})));
        try put(a, &result, "v", .{ .integer = v });
        return result;
    }
    var key = try evm.hex(32, try w.string(try field(auth, "key")));
    defer std.crypto.secureZero(u8, &key);
    const sig = try @import("transaction.zig").recoverable(key, hash);
    var signature = obj();
    try put(a, &signature, "r", w.text(try std.fmt.allocPrint(a, "0x{x}", .{std.mem.readInt(u256, sig[0..32], .big)})));
    try put(a, &signature, "s", w.text(try std.fmt.allocPrint(a, "0x{x}", .{std.mem.readInt(u256, sig[32..64], .big)})));
    try put(a, &signature, "v", .{ .integer = sig[64] });
    return signature;
}
/// Offline user-signed scheme. The domain is distinct from L1 exchange orders.
pub fn userAction(a: A, input: V) !V {
    const auth = get(input, "auth");
    const network = try choice(try field(input, "network"), &.{ "Mainnet", "Testnet" });
    const chain = try w.unsigned(u64, try uint(u64, try field(input, "signature_chain_id")));
    const action_data = try field(input, "action");
    const typ = try choice(try field(action_data, "type"), &.{ "usdSend", "approveAgent" });
    if (action_data.object.count() != 4) return error.UnexpectedField;
    const domain = evm.hash(network.string);
    const body = if (eq(u8, typ.string, "usdSend")) blk: {
        const destination = try hex(20, a, try field(action_data, "destination"));
        const amount = try decimal(try field(action_data, "amount"));
        const time = try w.unsigned(u64, try uint(u64, try field(action_data, "time")));
        break :blk evm.hash(&(evm.hash("HyperliquidTransaction:UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)") ++ domain ++ evm.hash(destination.string) ++ evm.hash(amount.string) ++ evm.encodeWord(time)));
    } else blk: {
        const address = try hex(20, a, try field(action_data, "agentAddress"));
        const bytes = try evm.hex(20, address.string);
        const name = try w.string(try field(action_data, "agentName"));
        const nonce = try w.unsigned(u64, try uint(u64, try field(action_data, "nonce")));
        break :blk evm.hash(&(evm.hash("HyperliquidTransaction:ApproveAgent(string hyperliquidChain,address agentAddress,string agentName,uint64 nonce)") ++ domain ++ evm.encodeWord(std.mem.readInt(u160, &bytes, .big)) ++ evm.hash(name) ++ evm.encodeWord(nonce)));
    };
    const hash = digest("HyperliquidSignTransaction", chain, body);
    if (auth == .null) return w.text(try std.fmt.allocPrint(a, "0x{x}", .{&hash}));
    if (!eq(u8, try w.string(try field(auth, "network")), network.string)) return error.AuthenticationRequired;
    return sign(a, auth, hash);
}
