const std = @import("std");
const w = @import("wire.zig");
const crypto = @import("lighter_crypto.zig");
const http = @import("http.zig");
const transport = @import("transport.zig");
const gpa = std.heap.c_allocator;
const A = w.A;
const V = w.V;
const eq = std.mem.eql;
const max48 = (1 << 48) - 1;
fn get(v: V, k: []const u8) V {
    return if (v == .object) v.object.get(k) orelse .null else .null;
}
fn number(v: V, k: []const u8, max: u64) !u64 {
    const value = try w.field(v, k);
    if (value != .number_string) return error.InvalidInteger;
    const n = try w.unsigned(u64, value);
    if (n > max) return error.InvalidParameter;
    return n;
}
fn num(a: A, n: u64) !V {
    return .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{n}) };
}
const Identity = struct { chain: u32, account: u64, key: u8 };
const Sequence = struct { mutex: std.Io.Mutex = .init, nonce: ?u64 = null, pending: bool = false };
var registry: std.AutoHashMapUnmanaged(Identity, *Sequence) = .empty;
var lock: std.atomic.Mutex = .unlocked;
fn sequence(id: Identity) !*Sequence {
    while (!lock.tryLock()) std.atomic.spinLoopHint();
    defer lock.unlock();
    if (registry.get(id)) |s| return s;
    const s = try gpa.create(Sequence);
    errdefer gpa.destroy(s);
    s.* = .{};
    try registry.put(gpa, id, s);
    return s;
}
fn attributes(v: V) ![8]u64 {
    var out: [8]u64 = @splat(0);
    if (v == .null) return out;
    if (v != .object or v.object.count() > 4) return error.InvalidAttributes;
    var used: usize = 0;
    var attr: [9]u64 = @splat(0);
    const keys = [_]([]const u8){ "1", "2", "3", "6", "7", "8" };
    const max = [_]u64{ max48 - 1, 1000000, 1000000, 3, 1, max48 };
    var count: usize = 0;
    for (keys, max) |k, limit| {
        if (!v.object.contains(k)) continue;
        count += 1;
        const n = try number(v, k, limit);
        const index = k[0] - '0';
        attr[index] = n;
        if (n != 0) {
            out[used] = index;
            out[used + 1] = n;
            used += 2;
        }
    }
    if (count != v.object.count() or ((attr[2] != 0 or attr[3] != 0) and (attr[1] == 0 or attr[6] != 0 or attr[7] != 0)) or (attr[6] == 3 and attr[7] == 1)) return error.InvalidAttributes;
    return out;
}
const Prepared = struct { info: V, values: [10]u64, len: usize, expiry: u64, attrs: [8]u64, kind: u8 };
fn prepare(a: A, op: []const u8, p: V) !Prepared {
    const create = eq(u8, op, "create_order");
    const order = try w.field(p, "order");
    const opts = try w.field(p, "options");
    const expiry = try number(opts, "expired_at", max48);
    var t = Prepared{ .info = .{ .object = .empty }, .values = @splat(0), .len = if (create) 10 else 2, .expiry = expiry, .attrs = try attributes(get(opts, "attributes")), .kind = if (create) 14 else 15 };
    const keys = if (create) &[_]([]const u8){ "MarketIndex", "ClientOrderIndex", "BaseAmount", "Price", "IsAsk", "Type", "TimeInForce", "ReduceOnly", "TriggerPrice", "OrderExpiry" } else &[_]([]const u8){ "MarketIndex", "Index" };
    const bounds = if (create) &[_]u64{ max48, max48, max48, 0xffffffff, 1, 6, 2, 1, 0xffffffff, std.math.maxInt(i64) } else &[_]u64{ max48, (1 << 60) - 1 };
    for (keys, bounds, 0..) |k, max, i| {
        t.values[i] = try number(order, k, max);
        try t.info.object.put(a, k, try w.field(order, k));
    }
    const v = t.values;
    if (v[0] == 255 or (!create and v[1] == 0)) return error.InvalidOrder;
    if (create) {
        if (v[3] == 0 or (v[2] == 0 and v[7] != 1)) return error.InvalidOrder;
        const valid = switch (v[5]) {
            0 => v[8] == 0 and ((v[6] == 0) == (v[9] == 0)),
            1 => v[6] == 0 and v[9] == 0 and v[8] == 0,
            2, 4 => v[6] == 0 and v[8] != 0 and v[9] != 0,
            3, 5 => v[8] != 0 and v[9] != 0,
            6 => v[6] == 1 and v[8] == 0 and v[9] != 0,
            else => false,
        };
        if (!valid) return error.InvalidOrder;
    }
    const attrs = get(opts, "attributes");
    try t.info.object.put(a, "L2TxAttributes", if (attrs == .object and attrs.object.count() > 0) attrs else .null);
    return t;
}
fn digest(t: Prepared, id: Identity, nonce: u64) [5]u64 {
    var words = [_]u64{ id.chain, t.kind, nonce, t.expiry, id.account, id.key } ++ t.values;
    const base = crypto.hash(words[0 .. 6 + t.len]);
    for (t.attrs) |n| if (n != 0) return crypto.hash(&(base ++ crypto.hash(&t.attrs)));
    return base;
}
fn request(a: A, client: *std.http.Client, method: []const u8, url: []const u8, payload: ?[]const u8, attempted: ?*bool) !V {
    var headers = V{ .object = .empty };
    if (payload != null) try headers.object.put(a, "content-type", w.text("application/x-www-form-urlencoded"));
    var req = try transport.sendTracked(a, client, method, url, headers, payload, attempted);
    defer req.deinit();
    var resp = req.receiveHead(&.{}) catch return error.Network;
    var buf: [8192]u8 = undefined;
    var decompress: std.http.Decompress = undefined;
    const r = try transport.bodyReader(a, &resp, &buf, &decompress);
    return http.decode(a, .lighter, @backingInt(resp.head.status), r.allocRemaining(a, .limited(16 * 1024 * 1024)) catch return error.Network);
}
pub fn execute(a: A, cfg: V, op: []const u8, dial: ?@import("host_stream.zig").Dial) !V {
    const auth = get(cfg, "auth");
    if (auth != .object) return error.AuthenticationRequired;
    const id = Identity{ .chain = @intCast(try number(auth, "chain_id", std.math.maxInt(u32))), .account = try number(auth, "account_index", max48 - 1), .key = @intCast(try number(auth, "api_key_index", 254)) };
    const base = if (cfg.object.get("origin")) |v| try w.string(v) else switch (id.chain) {
        300 => "https://testnet.zklighter.elliot.ai",
        304 => "https://mainnet.zklighter.elliot.ai",
        else => return error.InvalidNetwork,
    };
    const uri = try std.Uri.parse(base);
    if (uri.host == null or uri.user != null or uri.password != null or uri.query != null or uri.fragment != null or !(eq(u8, uri.scheme, "https") or eq(u8, uri.scheme, "http"))) return error.InvalidOrigin;
    for (base) |c| if (c <= 32 or c == 127 or c == 92) return error.InvalidOrigin;
    const host = try uri.host.?.toRawMaybeAlloc(a);
    if ((std.ascii.eqlIgnoreCase(host, "mainnet.zklighter.elliot.ai") and id.chain != 304) or (std.ascii.eqlIgnoreCase(host, "testnet.zklighter.elliot.ai") and id.chain != 300)) return error.InvalidNetwork;
    if (eq(u8, op, "prepare_transaction")) {
        const p = try w.field(cfg, "params");
        const kind = try w.string(try w.field(p, "kind"));
        if (!eq(u8, kind, "create_order") and !eq(u8, kind, "cancel_order")) return error.InvalidOperation;
        const t = try prepare(a, kind, p);
        const nonce = try number(p, "nonce", std.math.maxInt(i64));
        var out = V{ .object = .empty };
        try out.object.put(a, "tx_type", try num(a, t.kind));
        try out.object.put(a, "digest", w.text(try std.fmt.allocPrint(a, "{x}", .{crypto.bytes(digest(t, id, nonce))})));
        return out;
    }
    var key = try @import("evm.zig").hex(40, try w.string(auth.object.get("key") orelse return error.AuthenticationRequired));
    defer std.crypto.secureZero(u8, &key);
    var threaded: @import("host_stream.zig").Runtime = .{ .dial = dial };
    const io = threaded.io();
    const p = try w.field(cfg, "params");
    if (eq(u8, op, "auth_token")) {
        const deadline = try number(p, "deadline", std.math.maxInt(i64));
        const now = std.Io.Timestamp.now(io, .real).toSeconds();
        if (now < 0 or deadline <= now or deadline > now + 8 * 3600) return error.InvalidDeadline;
        const text = try std.fmt.allocPrint(a, "{d}:{d}:{d}", .{ deadline, id.account, id.key });
        var words: [8]u64 = @splat(0);
        for (text, 0..) |c, i| words[i / 8] |= @as(u64, c) << @intCast((i % 8) * 8);
        return w.text(try std.fmt.allocPrint(a, "{s}:{x}", .{ text, try crypto.sign(io, crypto.hash(words[0 .. (text.len + 7) / 8]), key) }));
    }
    const resync = eq(u8, op, "resync_nonce");
    var t = if (resync) null else @as(?Prepared, try prepare(a, op, p));
    const state = try sequence(id);
    state.mutex.lockUncancelable(io);
    defer state.mutex.unlock(io);
    if (state.pending and !resync) return error.PendingOutcome;
    var client = std.http.Client{ .io = io, .allocator = gpa };
    defer client.deinit();
    if (state.nonce == null or resync) {
        const url = try std.fmt.allocPrint(a, "{s}/api/v1/nextNonce?account_index={d}&api_key_index={d}", .{ std.mem.trimEnd(u8, base, "/"), id.account, id.key });
        const resp = try request(a, &client, "GET", url, null, null);
        const next = try number(resp, "nonce", std.math.maxInt(i64));
        state.nonce = next;
        state.pending = false;
        if (resync) return num(a, next);
    }
    const nonce = state.nonce.?;
    const sig = try crypto.sign(io, digest(t.?, id, nonce), key);
    const info = &t.?.info;
    const enc = std.base64.standard.Encoder;
    try info.object.put(a, "Sig", w.text(enc.encode(try a.alloc(u8, enc.calcSize(sig.len)), &sig)));
    try info.object.put(a, "AccountIndex", try num(a, id.account));
    try info.object.put(a, "ApiKeyIndex", try num(a, id.key));
    try info.object.put(a, "Nonce", try num(a, nonce));
    try info.object.put(a, "ExpiredAt", try num(a, t.?.expiry));
    const body = try std.fmt.allocPrint(a, "tx_type={d}&tx_info={s}", .{ t.?.kind, try http.escape(a, try std.json.Stringify.valueAlloc(a, info.*, .{}), false) });
    const url = try std.mem.concat(a, u8, &.{ std.mem.trimEnd(u8, base, "/"), "/api/v1/sendTx" });
    var attempted = false;
    errdefer if (attempted) {
        state.pending = true;
    };
    const reply = request(a, &client, "POST", url, body, &attempted) catch |e| {
        if (attempted and e != error.ExchangeApi) return error.PendingOutcome;
        return e;
    };
    _ = number(reply, "code", std.math.maxInt(i64)) catch return error.PendingOutcome;
    _ = w.string(get(reply, "tx_hash")) catch return error.PendingOutcome;
    _ = number(reply, "predicted_execution_time_ms", std.math.maxInt(i64)) catch return error.PendingOutcome;
    if (nonce == std.math.maxInt(i64)) {
        state.pending = true;
    } else state.nonce = nonce + 1;
    return reply;
}
