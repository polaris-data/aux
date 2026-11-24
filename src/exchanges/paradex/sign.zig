const std = @import("std");
const w = @import("../../core/wire.zig");
const st = @import("../../core/stark.zig");
const evm = @import("../../core/evm.zig");
const A = w.A;
const V = w.V;
const eq = std.mem.eql;
fn get(v: V, k: []const u8) V {
    return if (v == .object) v.object.get(k) orelse .null else .null;
}
fn text(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn str(v: V) ![]const u8 {
    return if (v == .number_string) v.number_string else w.string(v);
}
fn decimal(a: A, n: u256) !V {
    return w.text(try std.fmt.allocPrint(a, "{d}", .{n}));
}
fn scalar(v: V) !u256 {
    const s = try str(v);
    if (std.mem.startsWith(u8, s, "0x")) return st.felt(s);
    for (s) |c| if (!std.ascii.isDigit(c)) return st.short(s);
    return if (s.len == 0) 0 else st.felt(s);
}
fn less(_: void, a: []const u8, b: []const u8) bool {
    return std.mem.order(u8, a, b) == .lt;
}
const Encoder = struct {
    a: A,
    types: V,
    rev: bool,
    fn hash(self: Encoder, values: []const u256) u256 {
        return if (self.rev) st.poseidon(values) else st.legacy(values);
    }
    fn collect(self: Encoder, name: []const u8, seen: *std.StringHashMapUnmanaged(void), depth: usize) anyerror!void {
        if (depth > 32 or seen.count() > 128) return error.InvalidType;
        if ((try seen.getOrPut(self.a, name)).found_existing) return;
        const fields = try w.field(self.types, name);
        if (fields != .array) return error.InvalidType;
        for (fields.array.items) |f| {
            const child = std.mem.trimEnd(u8, try text(f, "type"), "*");
            if (self.types.object.contains(child)) try self.collect(child, seen, depth + 1);
        }
    }
    fn signature(self: Encoder, out: *std.ArrayList(u8), name: []const u8) !void {
        const fields = try w.field(self.types, name);
        if (fields != .array) return error.InvalidType;
        if (self.rev) try out.print(self.a, "\"{s}\"(", .{name}) else try out.print(self.a, "{s}(", .{name});
        for (fields.array.items, 0..) |f, i| {
            if (i > 0) try out.append(self.a, ',');
            const key = try text(f, "name");
            const ty = try text(f, "type");
            if (self.rev) try out.print(self.a, "\"{s}\":\"{s}\"", .{ key, ty }) else try out.print(self.a, "{s}:{s}", .{ key, ty });
        }
        try out.append(self.a, ')');
    }
    fn typeHash(self: Encoder, name: []const u8) !u256 {
        var seen: std.StringHashMapUnmanaged(void) = .empty;
        try self.collect(name, &seen, 0);
        _ = seen.remove(name);
        var names: std.ArrayList([]const u8) = .empty;
        var it = seen.keyIterator();
        while (it.next()) |key| try names.append(self.a, key.*);
        std.mem.sort([]const u8, names.items, {}, less);
        var sig: std.ArrayList(u8) = .empty;
        try self.signature(&sig, name);
        for (names.items) |n| try self.signature(&sig, n);
        return st.selector(sig.items);
    }
    fn encode(self: Encoder, name: []const u8, v: V, depth: usize) anyerror!u256 {
        if (depth > 32) return error.InvalidType;
        if (self.types.object.get(name)) |fields| {
            if (fields != .array or v != .object) return error.InvalidType;
            var vals: std.ArrayList(u256) = .empty;
            try vals.append(self.a, try self.typeHash(name));
            for (fields.array.items) |f| {
                const ty = try text(f, "type");
                const val = try w.field(v, try text(f, "name"));
                if (eq(u8, ty, "merkletree")) {
                    if (!self.rev or val != .array or val.array.items.len == 0) return error.InvalidTree;
                    const layer = try self.a.alloc(u256, val.array.items.len);
                    for (val.array.items, layer) |item, *n| n.* = try self.encode(try text(f, "contains"), item, depth + 1);
                    var count = layer.len;
                    while (count > 1) {
                        for (0..(count + 1) / 2) |i| layer[i] = st.branch(layer[2 * i], if (2 * i + 1 < count) layer[2 * i + 1] else 0);
                        count = (count + 1) / 2;
                    }
                    try vals.append(self.a, layer[0]);
                } else try vals.append(self.a, try self.encode(ty, val, depth + 1));
            }
            return self.hash(vals.items);
        }
        if (std.mem.endsWith(u8, name, "*")) {
            if (v != .array) return error.InvalidArray;
            const vals = try self.a.alloc(u256, v.array.items.len);
            for (vals, v.array.items) |*n, it| n.* = try self.encode(name[0 .. name.len - 1], it, depth + 1);
            return self.hash(vals);
        }
        if (eq(u8, name, "felt") or eq(u8, name, "shortstring")) return scalar(v);
        if (!self.rev) return error.InvalidType;
        if (eq(u8, name, "ContractAddress")) return st.felt(try str(v));
        if (eq(u8, name, "timestamp") or eq(u8, name, "u128")) return try w.unsigned(u128, v);
        if (eq(u8, name, "bool") and v == .bool) return @intFromBool(v.bool);
        if (eq(u8, name, "string")) {
            const s = try w.string(v);
            const full = s.len / 31;
            const words = try self.a.alloc(u256, full + 3);
            words[0] = full;
            for (0..full) |i| words[i + 1] = std.mem.readInt(u248, s[31 * i ..][0..31], .big);
            var tail: u256 = 0;
            for (s[31 * full ..]) |c| tail = tail << 8 | c;
            words[full + 1] = tail;
            words[full + 2] = s.len % 31;
            return st.poseidon(words);
        }
        return error.InvalidType;
    }
};
pub fn digest(a: A, account: u256, td: V) !u256 {
    const domain = try w.field(td, "domain");
    const revision = get(domain, "revision");
    const rev = if (revision == .null) 0 else try w.unsigned(u8, revision);
    if (rev > 1) return error.InvalidRevision;
    const e = Encoder{ .a = a, .types = try w.field(td, "types"), .rev = rev == 1 };
    if (e.types != .object) return error.InvalidType;
    return e.hash(&.{ try st.short("StarkNet Message"), try e.encode(if (e.rev) "StarknetDomain" else "StarkNetDomain", domain, 0), account, try e.encode(try text(td, "primaryType"), try w.field(td, "message"), 0) });
}
const Proof = struct { signature: V, public: V };
fn proof(a: A, auth: V, hash: u256) !Proof {
    var s = std.array_list.Managed(V).init(a);
    if (get(auth, "stark_key") != .null) {
        if (get(auth, "evm_key") != .null) return error.InvalidCredential;
        var key = try st.felt(try text(auth, "stark_key"));
        defer std.crypto.secureZero(u8, std.mem.asBytes(&key));
        for (try st.sign(key, hash)) |n| try s.append(try decimal(a, n));
        return .{ .signature = .{ .array = s }, .public = w.text(try std.fmt.allocPrint(a, "0x{x}", .{try st.publicKey(key)})) };
    }
    if (get(auth, "evm_key") == .null) return error.AuthenticationRequired;
    var key = try evm.hex(32, try text(auth, "evm_key"));
    defer std.crypto.secureZero(u8, &key);
    const pubkey = (try std.crypto.ecc.Secp256k1.basePoint.mul(key, .big)).toUncompressedSec1();
    const addr = evm.hash(pubkey[1..])[12..].*;
    const sig = try @import("../../core/transaction.zig").recoverable(key, evm.hash(&("\x19Ethereum Signed Message:\n32".* ++ evm.encodeWord(hash))));
    for ([_]u256{ 1, 3, std.mem.readInt(u160, &addr, .big), std.mem.readInt(u128, sig[16..32], .big), std.mem.readInt(u128, sig[0..16], .big), std.mem.readInt(u128, sig[48..64], .big), std.mem.readInt(u128, sig[32..48], .big), sig[64] - 27 }) |n| try s.append(try decimal(a, n));
    return .{ .signature = .{ .array = s }, .public = w.text(try std.fmt.allocPrint(a, "0x{x}", .{&addr})) };
}
pub fn typed(a: A, v: V) !V {
    const td = try w.field(v, "typed_data");
    const auth = try w.field(v, "auth");
    const account = try st.felt(try text(auth, "account"));
    const network = try st.short(try text(auth, "network"));
    if (network != try scalar(try w.field(try w.field(td, "domain"), "chainId"))) return error.InvalidNetwork;
    const d = try digest(a, account, td);
    var r: V = .{ .object = .empty };
    try r.object.put(a, "digest", w.text(try std.fmt.allocPrint(a, "0x{x}", .{d})));
    try r.object.put(a, "signature", (try proof(a, auth, d)).signature);
    return r;
}
fn scale(a: A, s: []const u8) !V {
    const i = std.mem.findScalar(u8, s, '.') orelse s.len;
    const frac = if (i < s.len) s[i + 1 ..] else "";
    if (i == 0 or frac.len > 8) return error.InvalidAmount;
    var n = try w.unsigned(u256, w.text(s[0..i]));
    for (0..8) |j| {
        const c: u8 = if (j < frac.len) frac[j] else '0';
        if (!std.ascii.isDigit(c)) return error.InvalidAmount;
        n = try std.math.add(u256, try std.math.mul(u256, n, 10), c - '0');
    }
    if (n >= st.prime) return error.InvalidAmount;
    return decimal(a, n);
}
fn nonempty(v: V) bool {
    return v == .string and v.string.len != 0;
}
fn objectOf(a: A, pairs: anytype) !V {
    var out: V = .{ .object = .empty };
    inline for (comptime std.meta.fieldNames(@TypeOf(pairs))) |name| try out.object.put(a, name, @field(pairs, name));
    return out;
}
fn side(s: []const u8) !V {
    if (eq(u8, s, "BUY")) return w.text("1");
    if (eq(u8, s, "SELL")) return w.text("2");
    return error.InvalidSide;
}
fn signMessage(a: A, auth: V, network: []const u8, primary: []const u8, msg: V, rev: bool) !Proof {
    var dom = try objectOf(a, .{ .name = w.text("Paradex"), .version = w.text("1"), .chainId = w.text(network) });
    if (rev) try dom.object.put(a, "revision", w.text("1"));
    const td = try objectOf(a, .{ .domain = dom, .types = try w.parse(a, @embedFile("signing-types.json")), .primaryType = w.text(primary), .message = msg });
    return proof(a, auth, try digest(a, try st.felt(try text(auth, "account")), td));
}
fn signOrder(a: A, r: *V, auth: V, network: []const u8, id: ?V, ms: u64) !void {
    if (r.* != .object) return error.InvalidOrder;
    if (nonempty(get(r.*, "signature"))) return;
    const t = get(r.*, "signature_timestamp");
    if (t != .number_string or (w.unsigned(u64, t) catch 0) == 0) try r.object.put(a, "signature_timestamp", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{ms}) });
    const ty = try text(r.*, "type");
    var m = try objectOf(a, .{ .timestamp = try w.field(r.*, "signature_timestamp"), .market = try w.field(r.*, "market"), .side = try side(try text(r.*, "side")), .orderType = w.text(ty), .size = try scale(a, try text(r.*, "size")), .price = if (eq(u8, ty, "MARKET")) w.text("0") else try scale(a, try text(r.*, "price")) });
    if (id) |x| try m.object.put(a, "id", x);
    const sig = try signMessage(a, auth, network, if (id == null) "Order" else "ModifyOrder", m, false);
    try r.object.put(a, "signature", w.text(try std.json.Stringify.valueAlloc(a, sig.signature, .{})));
}
fn defaultText(v: V, k: []const u8) []const u8 {
    const item = get(v, k);
    return if (nonempty(item)) item.string else "0";
}
fn blockOrder(a: A, v: V) !V {
    const s = get(v, "side");
    return objectOf(a, .{ .account = w.text(defaultText(v, "account")), .side = if (nonempty(s)) try side(s.string) else w.text("0"), .type = w.text(defaultText(v, "type")), .size = try scale(a, defaultText(v, "size")), .price = try scale(a, defaultText(v, "price")) });
}
pub fn blockSignature(a: A, auth: V, network: []const u8, nonce: V, expiration: u64, trades: V, parent: ?V, ms: u64) !V {
    const account = try std.fmt.allocPrint(a, "0x{x}", .{try st.felt(try text(auth, "account"))});
    if (trades != .object or trades.object.count() == 0) return error.InvalidTrades;
    const names = try a.dupe([]const u8, trades.object.keys());
    std.mem.sort([]const u8, names, {}, less);
    var leaves = std.array_list.Managed(V).init(a);
    for (names) |market| {
        const t = trades.object.get(market).?;
        var leaf = try objectOf(a, .{ .market = w.text(market), .maker_order = try blockOrder(a, get(t, if (parent == null) "maker_order" else "offerer_order")), .taker_order = try blockOrder(a, if (parent == null) get(t, "taker_order") else .null) });
        for ([_][]const u8{ "price", "size", "min_size", "max_size", "min_price", "max_price", "oracle_tolerance" }) |name| {
            const v = if (t == .object) t.object.get(name) orelse get(get(t, "trade_constraints"), name) else return error.InvalidTrades;
            try leaf.object.put(a, name, try scale(a, if (v == .string) v.string else "0"));
        }
        try leaves.append(leaf);
    }
    var msg = try objectOf(a, .{ .version = w.text("2"), .nonce = nonce, .expiration = try decimal(a, expiration), .trades = V{ .array = leaves } });
    if (parent) |id| try msg.object.put(a, "block_trade_id", id);
    const sig = try signMessage(a, auth, network, if (parent == null) "BlockTrade" else "BlockTradeOffer", msg, true);
    return objectOf(a, .{ .nonce = nonce, .signature_data = w.text(try std.json.Stringify.valueAlloc(a, sig.signature, .{})), .signature_expiration = V{ .number_string = try std.fmt.allocPrint(a, "{d}", .{expiration}) }, .signature_timestamp = V{ .number_string = try std.fmt.allocPrint(a, "{d}", .{ms}) }, .signature_type = w.text("STARKNET"), .signer_account = w.text(account), .signer_public_key = sig.public });
}
fn block(a: A, r: *V, auth: V, network: []const u8, parent: ?V, ms: u64) !void {
    if (r.* != .object) return error.InvalidOrder;
    const account = try std.fmt.allocPrint(a, "0x{x}", .{try st.felt(try text(auth, "account"))});
    if (parent != null and nonempty(get(get(r.*, "signature"), "signature_data"))) return;
    var signatures = get(r.*, "signatures");
    if (parent == null and signatures == .object and signatures.object.contains(account)) return;
    const expiration = if (parent == null) try w.unsigned(u64, try w.field(r.*, "block_expiration")) else try std.math.add(u64, ms, 300000);
    const signed = try blockSignature(a, auth, network, try w.field(r.*, "nonce"), expiration, try w.field(r.*, "trades"), parent, ms);
    if (parent != null) try r.object.put(a, "signature", signed) else {
        if (signatures == .null) signatures = .{ .object = .empty };
        if (signatures != .object) return error.InvalidSignature;
        try signatures.object.put(a, account, signed);
        try r.object.put(a, "signatures", signatures);
    }
}
pub fn dispatch(a: A, op: []const u8, input: V, path: []const u8, body: *V, auth: V, ms: u64) !void {
    const supported = eq(u8, op, "orders_new") or eq(u8, op, "orders_modify") or eq(u8, op, "orders_new_batch") or eq(u8, op, "block_trades_create") or eq(u8, op, "block_offers_create");
    if (!supported) return;
    const origin = if (get(input, "origin") != .null) try text(input, "origin") else "https://api.prod.paradex.trade";
    const host = (try std.Uri.parse(origin)).host orelse return error.InvalidOrigin;
    const name = switch (host) {
        .raw, .percent_encoded => |s| s,
    };
    const known: ?[]const u8 = if (std.ascii.eqlIgnoreCase(name, "api.prod.paradex.trade") or std.ascii.eqlIgnoreCase(name, "ws.api.prod.paradex.trade")) "PRIVATE_SN_PARACLEAR_MAINNET" else if (std.ascii.eqlIgnoreCase(name, "api.testnet.paradex.trade") or std.ascii.eqlIgnoreCase(name, "ws.api.testnet.paradex.trade")) "PRIVATE_SN_PARACLEAR_TESTNET" else null;
    const network = if (get(auth, "network") != .null) try text(auth, "network") else known orelse "";
    if (known) |n| if (!eq(u8, network, n)) return error.InvalidNetwork;
    if (network.len == 0 and (get(auth, "evm_key") != .null or get(auth, "stark_key") != .null)) return error.InvalidNetwork;
    // The protocol signs the identifier in the encoded request path.
    const id: ?V = if (eq(u8, op, "orders_modify"))
        w.text(path["/v1/orders/".len..])
    else if (eq(u8, op, "block_offers_create")) blk: {
        const rest = path["/v1/block-trades/".len..];
        break :blk w.text(rest[0 .. std.mem.findScalar(u8, rest, '/') orelse return error.InvalidPath]);
    } else null;
    if (eq(u8, op, "orders_new_batch")) {
        if (body.* != .array or body.array.items.len == 0) return error.InvalidBatch;
        for (body.array.items) |*r| try signOrder(a, r, auth, network, null, ms);
    } else if (eq(u8, op, "orders_new") or eq(u8, op, "orders_modify")) {
        try signOrder(a, body, auth, network, id, ms);
    } else try block(a, body, auth, network, id, ms);
}
