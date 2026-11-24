// Pascal Ed25519 permits. A wallet-scoped reservation survives an unknown submission outcome.
const std = @import("std");
const w = @import("../../core/wire.zig");
const sol = @import("../../core/solana.zig");
const V = w.V;
const A = w.A;
const Bytes = std.ArrayList(u8);
const Ed25519 = std.crypto.sign.Ed25519;
const Kind = enum { place_orders, cancel_orders, mass_cancel, create_trading_key, revoke_trading_key, withdrawal, register_deposit_address, generate_referral_code, claim_custom_referral_code, apply_referral_code, set_username, set_account_bio };
fn kindFor(op: []const u8) !Kind {
    if (std.meta.stringToEnum(Kind, op)) |kind| return kind;
    const name = try @import("../../core/venue_auth.zig").routeKind(@embedFile("permit-routes.txt"), op);
    return std.meta.stringToEnum(Kind, name) orelse error.InvalidOperation;
}
fn guarded(k: Kind) bool {
    return switch (k) {
        .create_trading_key, .revoke_trading_key, .withdrawal, .register_deposit_address => true,
        else => false,
    };
}
fn field(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn num(v: V, k: []const u8) !u64 {
    return w.unsigned(u64, try w.field(v, k));
}
fn optional(r: V, k: []const u8, fallback: u64) !u64 {
    const v = r.object.get(k) orelse .null;
    return if (v == .null) fallback else w.unsigned(u64, v);
}
fn number(a: A, n: u64) !V {
    return .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{n}) };
}
fn micros(s: []const u8) !u64 {
    const point = std.mem.findScalar(u8, s, '.') orelse s.len;
    const fraction = if (point < s.len) s[point + 1 ..] else "";
    if (point == 0 or fraction.len > 6) return error.InvalidAmount;
    const whole = try w.unsigned(u64, w.text(s[0..point]));
    var part: u64 = 0;
    for (fraction) |c| {
        if (!std.ascii.isDigit(c)) return error.InvalidAmount;
        part = part * 10 + (c - '0');
    }
    for (fraction.len..6) |_| part *= 10;
    return std.math.add(u64, try std.math.mul(u64, whole, 1_000_000), part);
}
fn le(comptime T: type, n: T) [@sizeOf(T)]u8 {
    var buf: [@sizeOf(T)]u8 = undefined;
    std.mem.writeInt(T, &buf, n, .little);
    return buf;
}
fn symbol(r: V) ![32]u8 {
    const s = try field(r, "symbol");
    if (s.len > 32) return error.InvalidSymbol;
    var out: [32]u8 = @splat(0);
    for (s, 0..) |c, i| {
        if (c == 0 or c > 127) return error.InvalidSymbol;
        out[i] = c;
    }
    return out;
}
fn choice(s: []const u8, values: []const []const u8) !u8 {
    for (values, 0..) |x, i| if (std.mem.eql(u8, s, x)) return @intCast(i);
    return error.InvalidChoice;
}
const Wallet = struct { id: u8, owner: [32]u8 };
const Config = struct {
    wallet: Wallet,
    window: u32,
    fn init(auth: V) !Config {
        if (auth != .object) return error.AuthenticationRequired;
        const id = if (auth.object.get("deployment_id")) |v| try w.unsigned(u8, v) else 3;
        const window = if (auth.object.get("recv_window_ms")) |v| try w.unsigned(u32, v) else 5000;
        if (window > 60000) return error.InvalidWindow;
        return .{ .wallet = .{ .id = id, .owner = try sol.publicKey(try field(auth, "owner")) }, .window = window };
    }
};

const WalletState = struct { last: u64 = 0, ticket: u64 = 0, pending: bool = false };
var states: std.AutoHashMapUnmanaged(Wallet, WalletState) = .empty;
var lock: std.atomic.Mutex = .unlocked;
var ticket_counter: u64 = 0;
fn acquire() void {
    while (!lock.tryLock()) std.atomic.spinLoopHint();
}
fn entry(key: Wallet) !*WalletState {
    const item = try states.getOrPut(std.heap.c_allocator, key);
    if (!item.found_existing) item.value_ptr.* = .{};
    return item.value_ptr;
}
fn timestamp(wallet: Wallet, now: u64) !u64 {
    acquire();
    defer lock.unlock();
    const state = try entry(wallet);
    state.last = @max(now, try std.math.add(u64, state.last, 1));
    return state.last;
}
pub const Reservation = struct {
    wallet: Wallet,
    ticket: u64,
    pub fn finish(self: Reservation, confirmed: bool) void {
        acquire();
        defer lock.unlock();
        if (states.getPtr(self.wallet)) |state| {
            if (self.ticket == 0 or state.ticket != self.ticket) return;
            state.ticket = 0;
            state.pending = !confirmed;
        }
    }
};
pub fn reserve(input: V) !?Reservation {
    if (!std.mem.eql(u8, try field(input, "venue"), "pascal")) return null;
    const name = try field(input, "operation");
    if (!std.mem.startsWith(u8, name, "write.")) return null;
    if (!guarded(try kindFor(name))) return null;
    const cfg = try Config.init(try w.field(input, "auth"));
    acquire();
    defer lock.unlock();
    const state = try entry(cfg.wallet);
    if (state.pending or state.ticket != 0) return error.PendingOutcome;
    ticket_counter = try std.math.add(u64, ticket_counter, 1);
    state.ticket = ticket_counter;
    return .{ .wallet = cfg.wallet, .ticket = ticket_counter };
}
pub fn walletPending(auth: V) !V {
    if (auth == .null) return .{ .bool = false };
    const cfg = try Config.init(auth);
    acquire();
    defer lock.unlock();
    const state = states.get(cfg.wallet) orelse return .{ .bool = false };
    return .{ .bool = state.pending };
}
pub fn reconcile(auth: V) !V {
    const cfg = try Config.init(auth);
    acquire();
    defer lock.unlock();
    if (states.getPtr(cfg.wallet)) |state| {
        if (state.ticket != 0) return error.PendingOutcome;
        state.pending = false;
    }
    return .null;
}
// The host transport obtains its reservation after successful request preparation.
pub fn beginHost(a: A, input: V) !V {
    const res = (try reserve(input)) orelse return .null;
    errdefer res.finish(true);
    var v: V = .{ .object = .empty };
    try v.object.put(a, "owner", w.text(try sol.base58(a, 32, res.wallet.owner)));
    try v.object.put(a, "deployment_id", try number(a, res.wallet.id));
    try v.object.put(a, "ticket", try number(a, res.ticket));
    return v;
}
pub fn endHost(v: V) !V {
    const cfg = try Config.init(v);
    const confirmed = try w.field(v, "confirmed");
    if (confirmed != .bool) return error.InvalidOutcome;
    (Reservation{ .wallet = cfg.wallet, .ticket = try num(v, "ticket") }).finish(confirmed.bool);
    return .null;
}

fn message(a: A, k: Kind, r: V, cfg: Config, signer: [32]u8, ts: u64) ![]const u8 {
    if (r != .object) return error.InvalidRequest;
    const cmd: u8 = switch (k) {
        .place_orders => 1,
        .cancel_orders => 2,
        .mass_cancel => 10,
        else => 0,
    };
    if (cmd != 0) {
        var b: Bytes = .empty;
        try b.appendSlice(a, &([_]u8{ 1, cmd, cfg.wallet.id, 0 } ++ le(u32, cfg.window) ++ le(u64, ts) ++ cfg.wallet.owner ++ signer));
        switch (k) {
            .place_orders => {
                const size = try num(r, "size");
                const price = try micros(try field(r, "price"));
                const id = try num(r, "client_order_id");
                const maximum = std.math.maxInt(u64);
                if (size == 0 or price > 1_000_000 or id == maximum) return error.InvalidOrder;
                const replace = try optional(r, "replace_client_order_id", maximum);
                const expiry = try optional(r, "expires_ts_ms", 0);
                const tif = try choice(try w.string(r.object.get("tif") orelse w.text("GTC")), &.{ "GTC", "GTT", "IOC" });
                const side = try choice(try field(r, "side"), &.{ "BID", "ASK" });
                const ty = try choice(try w.string(r.object.get("type") orelse r.object.get("order_type") orelse w.text("LIMIT")), &.{ "LIMIT", "MARKET" });
                if ((tif == 1) != (expiry != 0)) return error.InvalidOrder;
                var flags: u8 = 0;
                for ([_][]const u8{ "post_only", "reduce_only", "allow_missing_replace", "replace_with_open_size" }, 0..) |name, i| {
                    if (r.object.get(name)) |v| {
                        if (v != .bool) return error.InvalidFlag;
                        if (v.bool) flags |= @as(u8, 1) << @intCast(i);
                    }
                }
                if ((flags & 12 != 0 and replace == maximum) or flags & 12 == 12) return error.InvalidReplacement;
                for ([_]u64{ size, price, id, replace, expiry }) |n| try b.appendSlice(a, &le(u64, n));
                try b.appendNTimes(a, 0, 32);
                try b.appendSlice(a, &try symbol(r));
                try b.appendNTimes(a, 0, 4);
                const hash = try @import("../../core/evm.zig").hex(32, "dcaf53738f1d5f06164660498a75888e7f0c78ed5f1bc17be1dd1cb3ccf53ce3");
                try b.appendSlice(a, &hash);
                try b.appendSlice(a, &.{ 0, side, ty, tif, flags });
                try b.appendNTimes(a, 0, 7);
            },
            .cancel_orders => {
                const client = r.object.get("client_order_id");
                const exchange = r.object.get("order_id");
                if ((client == null) == (exchange == null)) return error.InvalidOrder;
                try b.appendSlice(a, &le(u64, @intFromBool(client == null)));
                try b.appendSlice(a, &le(u64, try w.unsigned(u64, client orelse exchange.?)));
            },
            .mass_cancel => {
                try b.appendNTimes(a, 0, 8);
                try b.appendSlice(a, &try symbol(r));
            },
            else => unreachable,
        }
        return b.items;
    }
    if (guarded(k) and !std.mem.eql(u8, &signer, &cfg.wallet.owner)) return error.AuthenticationRequired;
    var items = std.array_list.Managed(V).init(a);
    const owner = w.text(try sol.base58(a, 32, cfg.wallet.owner));
    try items.append(w.text(@tagName(k)));
    switch (k) {
        .create_trading_key, .revoke_trading_key => {
            if (k == .create_trading_key) {
                const name = try field(r, "name");
                if (name.len > 16) return error.InvalidName;
                try items.append(w.text(name));
            }
            const key = try field(r, "trading_key");
            _ = try sol.publicKey(key);
            try items.append(w.text(key));
            if (k == .create_trading_key) try items.append(try number(a, try num(r, "expiration_ts_ms")));
        },
        .withdrawal => {
            try items.append(try number(a, try micros(try field(r, "amount"))));
            for ([_][]const u8{ "destination_authority", "destination_token_account" }) |name| {
                const key = try field(r, name);
                _ = try sol.publicKey(key);
                try items.append(w.text(key));
            }
        },
        .register_deposit_address => try items.append(owner),
        .generate_referral_code => {},
        .apply_referral_code, .claim_custom_referral_code => try items.append(w.text(try field(r, "referral_code"))),
        .set_username => try items.append(w.text(try field(r, "username"))),
        .set_account_bio => try items.append(w.text(try field(r, "account_bio"))),
        else => unreachable,
    }
    for ([_]u64{ 1, cfg.wallet.id, cfg.window, ts }) |n| try items.append(try number(a, n));
    try items.append(owner);
    try items.append(w.text(try sol.base58(a, 32, signer)));
    if (k == .register_deposit_address) if (r.object.get("referral_code")) |code| if (code != .null) {
        try items.append(w.text("referral_code"));
        try items.append(w.text(try w.string(code)));
    };
    return std.json.Stringify.valueAlloc(a, V{ .array = items }, .{});
}
fn signOne(a: A, k: Kind, r: *V, cfg: Config, kp: Ed25519.KeyPair, ts: u64) !void {
    const signer = kp.public_key.toBytes();
    const data = try message(a, k, r.*, cfg, signer, ts);
    const sig = (try kp.sign(data, null)).toBytes();
    return attachAuth(a, r, cfg, signer, ts, sig);
}
fn attachAuth(a: A, r: *V, cfg: Config, signer: [32]u8, ts: u64, sig: [64]u8) !void {
    var auth: V = .{ .object = .empty };
    try auth.object.put(a, "signer", w.text(try sol.base58(a, 32, signer)));
    try auth.object.put(a, "owner", w.text(try sol.base58(a, 32, cfg.wallet.owner)));
    try auth.object.put(a, "client_ts_ms", w.text(try std.fmt.allocPrint(a, "{d}", .{ts})));
    try auth.object.put(a, "recv_window_ms", w.text(try std.fmt.allocPrint(a, "{d}", .{cfg.window})));
    try auth.object.put(a, "signature", w.text(try sol.base58(a, 64, sig)));
    try r.object.put(a, "auth", auth);
}
pub fn sign(a: A, op: []const u8, body: *V, auth: V, now: u64, offline: bool) !void {
    const k = try kindFor(op);
    const cfg = try Config.init(auth);
    const rows = if (body.* == .array) body.array.items else @as(*[1]V, body)[0..];
    if (rows.len == 0 or rows.len > @as(usize, if (k == .mass_cancel) 10 else 50)) return error.InvalidBatch;
    if (auth.object.get("public_key")) |pubkey| {
        const signer = try sol.publicKey(try w.string(pubkey));
        for (rows) |*row| {
            const proof = try w.field(row.*, "auth");
            const ts = try num(proof, "client_ts_ms");
            if (!offline and (now > ts +| cfg.window or ts > now +| cfg.window)) return error.ExpiredPermit;
            try verifyAndAttach(a, k, row, cfg, signer, ts, try field(proof, "signature"));
        }
        return;
    }
    var seed = try @import("../../core/evm.zig").hex(32, try field(auth, "seed"));
    defer std.crypto.secureZero(u8, &seed);
    var kp = try Ed25519.KeyPair.generateDeterministic(seed);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&kp.secret_key));
    const ts = if (!offline and std.mem.eql(u8, &cfg.wallet.owner, &kp.public_key.toBytes())) try timestamp(cfg.wallet, now) else now;
    for (rows) |*r| try signOne(a, k, r, cfg, kp, ts);
}
fn verifyAndAttach(a: A, k: Kind, row: *V, cfg: Config, signer: [32]u8, ts: u64, proof: []const u8) !void {
    const data = try message(a, k, row.*, cfg, signer, ts);
    const sig = try sol.decode58(64, proof);
    try Ed25519.Signature.fromBytes(sig).verify(data, try Ed25519.PublicKey.fromBytes(signer));
    try attachAuth(a, row, cfg, signer, ts, sig);
}
fn prepareExternal(a: A, op: []const u8, body: *V, auth: V, requested: ?V) !V {
    const cfg = try Config.init(auth);
    const k = try kindFor(op);
    const signer = try sol.publicKey(try field(auth, "public_key"));
    var runtime: std.Io.Threaded = .init_single_threaded;
    const ts = if (requested) |v| try w.unsigned(u64, v) else blk: {
        if (guarded(k) and (try walletPending(auth)).bool) return error.PendingOutcome;
        const now = std.math.cast(u64, std.Io.Timestamp.now(runtime.io(), .real).toMilliseconds()) orelse return error.InvalidTimestamp;
        break :blk if (std.mem.eql(u8, &cfg.wallet.owner, &signer)) try timestamp(cfg.wallet, now) else now;
    };
    const rows = if (body.* == .array) body.array.items else @as(*[1]V, body)[0..];
    if (rows.len == 0 or rows.len > @as(usize, if (k == .mass_cancel) 10 else 50)) return error.InvalidBatch;
    var payloads = V{ .array = .init(a) };
    for (rows) |row| {
        const payload = try message(a, k, row, cfg, signer, ts);
        try payloads.array.append(w.text(try std.fmt.allocPrint(a, "{x}", .{payload})));
    }
    var out = V{ .object = .empty };
    try out.object.put(a, "messages", payloads);
    try out.object.put(a, "timestamp", try number(a, ts));
    return out;
}
pub fn signAt(a: A, v: V) !V {
    var body = try w.field(v, "body");
    const auth = try w.field(v, "auth");
    const op = try field(v, "operation");
    if (v.object.get("step")) |step| {
        if (!std.mem.eql(u8, try w.string(step), "prepare_external")) return error.InvalidOperation;
        return prepareExternal(a, op, &body, auth, v.object.get("timestamp"));
    }
    const ts = try num(v, "timestamp");
    if (auth == .object and auth.object.get("public_key") != null) {
        const cfg = try Config.init(auth);
        const signer = try sol.publicKey(try field(auth, "public_key"));
        const k = try kindFor(op);
        const proof = auth.object.get("signature") orelse return w.text(try std.fmt.allocPrint(a, "{x}", .{try message(a, k, body, cfg, signer, ts)}));
        try verifyAndAttach(a, k, &body, cfg, signer, ts, try w.string(proof));
    } else try sign(a, op, &body, auth, ts, true);
    return body;
}
