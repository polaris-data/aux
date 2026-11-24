// Nonces are coordinated across clients for the same chain and sending address.
const std = @import("std");
const w = @import("wire.zig");
const evm = @import("evm.zig");
const tx = @import("transaction.zig");
const rpc = @import("rpc.zig");
const A = w.A;
const V = w.V;
const Op = enum { validate_chain, submit_raw, send_transaction, prepare_external, send_external, pending, reconcile, resync_nonce };
const Identity = struct { chain: u64, address: [20]u8 };
const Pending = struct { hash: [32]u8, nonce: u64 };
const State = struct { next: u64 = 0, pending: ?Pending = null, busy: bool = false };
var mutex: std.atomic.Mutex = .unlocked;
var states: std.AutoHashMapUnmanaged(Identity, State) = .empty;
fn lock() void {
    while (!mutex.tryLock()) std.atomic.spinLoopHint();
}
fn reserve(id: Identity, op: Op, expected: ?[32]u8) !State {
    lock();
    defer mutex.unlock();
    const entry = try states.getOrPut(std.heap.c_allocator, id);
    if (!entry.found_existing) entry.value_ptr.* = .{};
    const state = entry.value_ptr;
    if (state.busy or ((op == .send_transaction or op == .prepare_external or op == .send_external) and state.pending != null)) return error.PendingOutcome;
    if (expected) |h| if (state.pending == null or !std.mem.eql(u8, &h, &state.pending.?.hash)) return error.PendingOutcome;
    state.busy = true;
    return state.*;
}
fn release(id: Identity, new: State) void {
    lock();
    defer mutex.unlock();
    const s = states.getPtr(id).?;
    s.* = new;
    s.busy = false;
}
fn string(v: V, key: []const u8) ![]const u8 {
    return w.string(try w.field(v, key));
}
fn fixed(comptime n: usize, v: V) ![n]u8 {
    const s = try w.string(v);
    if (!std.mem.startsWith(u8, s, "0x")) return error.InvalidHex;
    return evm.hex(n, s);
}
fn outputHex(a: A, b: []const u8) !V {
    return w.text(try std.fmt.allocPrint(a, "0x{x}", .{b}));
}
const Client = struct {
    a: A,
    read: []const u8,
    write: []const u8,
    chain: u64,
    fn call(c: Client, url: []const u8, method: []const u8, params: []const V, attempted: ?*bool) !V {
        var list = std.array_list.Managed(V).init(c.a);
        try list.appendSlice(params);
        return rpc.send(c.a, try rpc.envelope(c.a, url, method, .{ .array = list }), attempted);
    }
    fn validate(c: Client) !void {
        for ([_][]const u8{ c.read, c.write }) |url| {
            if (try rpc.quantity(u64, try c.call(url, "eth_chainId", &.{}, null)) != c.chain) return error.InvalidChain;
        }
    }
    fn nonce(c: Client, addr: V) !u64 {
        return rpc.quantity(u64, try c.call(c.read, "eth_getTransactionCount", &.{ addr, w.text("pending") }, null));
    }
};
pub fn execute(a: A, input: V) !V {
    const chain = try w.unsigned(u64, try w.field(input, "chain_id"));
    if (chain == 0) return error.InvalidChain;
    const c = Client{ .a = a, .read = try rpc.checkedUrl(try string(input, "read_rpc")), .write = try rpc.checkedUrl(try string(input, "write_rpc")), .chain = chain };
    const op = std.meta.stringToEnum(Op, try string(input, "operation")) orelse return error.InvalidOperation;
    if (op == .validate_chain) {
        try c.validate();
        return .null;
    }
    const params = try w.field(input, "params");
    if (op == .submit_raw) {
        const raw = try string(params, "signed");
        if (!std.mem.startsWith(u8, raw, "0x")) return error.InvalidHex;
        const bytes = try evm.fromHex(a, raw);
        _ = try tx.rawSender(a, bytes, chain);
        try c.validate();
        const h = evm.hash(bytes);
        const result = try c.call(c.write, "eth_sendRawTransaction", &.{w.text(raw)}, null);
        if (!std.mem.eql(u8, &h, &try fixed(32, result))) return error.InvalidHash;
        return outputHex(a, &h);
    }
    const addr = if (op == .send_transaction) blk: {
        var secret = try evm.hex(32, try string(params, "key"));
        defer std.crypto.secureZero(u8, &secret);
        const pubkey = (try std.crypto.ecc.Secp256k1.basePoint.mul(secret, .big)).toUncompressedSec1();
        break :blk evm.hash(pubkey[1..])[12..].*;
    } else try fixed(20, try w.field(params, "address"));
    const id = Identity{ .chain = chain, .address = addr };
    const sender = try outputHex(a, &addr);
    if (op == .pending) {
        const s = blk: {
            lock();
            defer mutex.unlock();
            break :blk states.get(id) orelse State{};
        };
        if (s.pending) |p| {
            var obj: V = .{ .object = .empty };
            try obj.object.put(a, "hash", try outputHex(a, &p.hash));
            try obj.object.put(a, "nonce", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{p.nonce}) });
            return obj;
        }
        return .null;
    }
    var state = try reserve(id, op, if (op == .resync_nonce) try fixed(32, try w.field(params, "hash")) else null);
    defer release(id, state);
    if (op == .reconcile) {
        const pending = state.pending orelse return .{ .bool = true };
        try c.validate();
        const observation = try c.call(c.read, "eth_getTransactionByHash", &.{try outputHex(a, &pending.hash)}, null);
        if (observation == .null) return .{ .bool = false };
        if (!std.mem.eql(u8, &pending.hash, &try fixed(32, try w.field(observation, "hash"))) or !std.mem.eql(u8, &addr, &try fixed(20, try w.field(observation, "from")))) return error.InvalidIdentity;
        state.next = pending.nonce + 1;
        state.pending = null;
        return .{ .bool = true };
    }
    if (op == .resync_nonce) {
        try c.validate();
        state.next = try c.nonce(sender);
        state.pending = null;
        return .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{state.next}) };
    }
    var prepared = try w.field(params, "prepared");
    if (try tx.quantity(u64, try w.field(prepared, "chainId")) != chain) return error.InvalidChain;
    const to = try fixed(20, try w.field(prepared, "to"));
    const targets = input.object.get("code_hashes") orelse V{ .object = .empty };
    if (targets != .object) return error.InvalidContract;
    var pins = targets.object.iterator();
    var pinned: ?[32]u8 = null;
    while (pins.next()) |pin| {
        const key = try fixed(20, w.text(pin.key_ptr.*));
        const digest = try fixed(32, pin.value_ptr.*);
        if (std.mem.eql(u8, &key, &to)) {
            if (pinned != null) return error.InvalidContract;
            pinned = digest;
        }
    }
    try c.validate();
    const code = try w.string(try c.call(c.read, "eth_getCode", &.{ try outputHex(a, &to), w.text("latest") }, null));
    if (!std.mem.startsWith(u8, code, "0x") or code.len == 2) return error.InvalidContract;
    const bytecode = try evm.fromHex(a, code);
    if (pinned) |h| if (!std.mem.eql(u8, &h, &evm.hash(bytecode))) return error.InvalidContract;
    const nonce = @max(state.next, try c.nonce(sender));
    if (nonce == std.math.maxInt(u64)) return error.InvalidNonce;
    try prepared.object.put(a, "nonce", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{nonce}) });
    var sign: V = .{ .object = .empty };
    try sign.object.put(a, "prepared", prepared);
    if (op == .prepare_external) {
        try sign.object.put(a, "digest_only", .{ .bool = true });
        return tx.sign(a, sign);
    }
    if (op == .send_external) {
        // Re-check chain, contract and current nonce after the callback. A stale
        // signature fails address recovery before a write can leave this process.
        try sign.object.put(a, "signature", try w.field(params, "signature"));
        try sign.object.put(a, "address", sender);
    } else try sign.object.put(a, "key", try w.field(params, "key"));
    const signed = try tx.sign(a, sign);
    const hash = try fixed(32, try w.field(signed, "hash"));
    const p = Pending{ .hash = hash, .nonce = nonce };
    state.pending = p;
    {
        lock();
        defer mutex.unlock();
        states.getPtr(id).?.pending = p;
    }
    var attempted = false;
    const result = c.call(c.write, "eth_sendRawTransaction", &.{try w.field(signed, "signed")}, &attempted) catch |err| {
        if (!attempted) state.pending = null;
        return err;
    };
    if (!std.mem.eql(u8, &hash, &try fixed(32, result))) return error.InvalidHash;
    state.next = nonce + 1;
    state.pending = null;
    return w.field(signed, "hash");
}
