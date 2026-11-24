// Pascal's Solana RPC client. Only ambiguous or acknowledged submissions retain state.
const std = @import("std");
const w = @import("../../core/wire.zig");
const rpc = @import("../../core/rpc.zig");
const sol = @import("../../core/solana.zig");
const V = w.V;
const A = w.A;
const eq = std.mem.eql;
const Op = enum { new, account_info, token_balance, block_height, signature_status, prepare_deposit, submit, pending, reconcile, clear_reconciled_submission };
const Pending = struct { signature: [64]u8, sending: bool };
var entries: std.AutoHashMapUnmanaged([32]u8, Pending) = .empty;
var mutex: std.atomic.Mutex = .unlocked;
var next_client = std.atomic.Value(u64).init(1);
fn lock() void {
    while (!mutex.tryLock()) std.atomic.spinLoopHint();
}
fn pending(id: [32]u8) ?Pending {
    lock();
    defer mutex.unlock();
    return entries.get(id);
}
fn claim(id: [32]u8, sig: [64]u8) !void {
    lock();
    defer mutex.unlock();
    const entry = try entries.getOrPut(std.heap.c_allocator, id);
    if (entry.found_existing) return error.PendingOutcome;
    entry.value_ptr.* = .{ .signature = sig, .sending = true };
}
fn sent(id: [32]u8, attempted: bool) void {
    lock();
    defer mutex.unlock();
    if (attempted) entries.getPtr(id).?.sending = false else _ = entries.remove(id);
}
fn clear(id: [32]u8, sig: [64]u8, required: bool) !void {
    lock();
    defer mutex.unlock();
    if (entries.get(id)) |p| {
        if (eq(u8, &sig, &p.signature)) {
            if (p.sending) return error.PendingOutcome;
            _ = entries.remove(id);
            return;
        }
    }
    if (required) return error.InvalidSignature;
}
fn string(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn parsed(a: A, value: anytype) !V {
    return w.parse(a, try std.json.Stringify.valueAlloc(a, value, .{}));
}
const Rpc = struct {
    a: A,
    url: []const u8,
    mint: []const u8,
    decimals: u8,
    fn call(self: Rpc, method: []const u8, args: anytype, attempted: ?*bool) !V {
        return rpc.send(self.a, try rpc.envelope(self.a, self.url, method, try parsed(self.a, args)), attempted);
    }
    fn info(self: Rpc, address: []const u8) !V {
        _ = try sol.publicKey(address);
        return self.call("getAccountInfo", .{ address, .{ .encoding = "jsonParsed", .commitment = "confirmed" } }, null);
    }
    fn height(self: Rpc) !V {
        const v = try self.call("getBlockHeight", .{.{ .commitment = "confirmed" }}, null);
        if (v != .number_string) return error.InvalidHeight;
        _ = try w.unsigned(u64, v);
        return v;
    }
    fn token(self: Rpc, address: []const u8) !V {
        const value = try w.field(try self.info(address), "value");
        if (!eq(u8, try string(value, "owner"), sol.token_program)) return error.InvalidTokenProgram;
        return w.field(try w.field(try w.field(value, "data"), "parsed"), "info");
    }
    fn prepare(self: Rpc, p: V) !V {
        var args = p;
        const owner = try string(p, "owner");
        _ = try sol.publicKey(owner);
        if (try w.unsigned(u64, try w.field(p, "amount")) == 0) return error.InvalidAmount;
        for ([_][]const u8{ "source", "destination" }, 0..) |k, i| {
            const account = try self.token(try string(p, k));
            if (!eq(u8, try string(account, "mint"), self.mint) or (i == 0 and !eq(u8, try string(account, "owner"), owner))) return error.InvalidTokenAccount;
        }
        const decimals = try w.field(try self.token(self.mint), "decimals");
        if (decimals != .number_string or try w.unsigned(u8, decimals) != self.decimals) return error.InvalidMint;
        const block = try w.field(try self.call("getLatestBlockhash", .{.{ .commitment = "confirmed" }}, null), "value");
        try args.object.put(self.a, "recent_blockhash", try w.field(block, "blockhash"));
        try args.object.put(self.a, "last_valid_block_height", try w.field(block, "lastValidBlockHeight"));
        try args.object.put(self.a, "mint", w.text(self.mint));
        try args.object.put(self.a, "decimals", decimals);
        return sol.prepare(self.a, args);
    }
    fn submit(self: Rpc, id: [32]u8, p: V) !V {
        const b64 = try string(p, "transaction_base64");
        var bytes: [279]u8 = undefined;
        const decoder = std.base64.standard.Decoder;
        if (try decoder.calcSizeForSlice(b64) != bytes.len) return error.InvalidDeposit;
        try decoder.decode(&bytes, b64);
        const msg = bytes[65..];
        const sig = bytes[1..65].*;
        const name = try sol.base58(self.a, 64, sig);
        if (bytes[0] != 1 or !eq(u8, name, try string(p, "signature")) or !eq(u8, msg[0..4], &.{ 1, 0, 2, 5 }) or
            !eq(u8, msg[100..132], &try sol.publicKey(self.mint)) or !eq(u8, msg[132..164], &try sol.publicKey(sol.token_program)) or
            !eq(u8, msg[196..205], &.{ 1, 4, 4, 1, 3, 2, 0, 10, 12 }) or std.mem.readInt(u64, msg[205..213], .little) == 0 or msg[213] != self.decimals) return error.InvalidDeposit;
        const Ed = std.crypto.sign.Ed25519;
        try Ed.Signature.fromBytes(sig).verify(msg, try Ed.PublicKey.fromBytes(msg[4..36].*));
        const expiry = try w.unsigned(u64, try w.field(p, "last_valid_block_height"));
        if (try w.unsigned(u64, try self.height()) > expiry) return error.Expired;
        try claim(id, sig);
        var attempted = false;
        defer sent(id, attempted);
        const reply = try self.call("sendTransaction", .{ b64, .{ .encoding = "base64", .skipPreflight = false, .preflightCommitment = "confirmed", .maxRetries = @as(u8, 0) } }, &attempted);
        if (reply != .string or !eq(u8, reply.string, name)) return error.InvalidSignature;
        return reply;
    }
};
pub fn execute(a: A, input: V) !V {
    const op = std.meta.stringToEnum(Op, try string(input, "operation")) orelse return error.InvalidOperation;
    const client = Rpc{ .a = a, .url = try rpc.endpoint(input), .mint = try string(input, "mint"), .decimals = try w.unsigned(u8, try w.field(input, "decimals")) };
    _ = try sol.publicKey(client.mint);
    if (client.decimals > 18) return error.InvalidDecimals;
    if (op == .new) return w.text(try std.fmt.allocPrint(a, "{d}", .{next_client.fetchAdd(1, .monotonic)}));
    const number = try w.unsigned(u64, try w.field(input, "client"));
    var id: [32]u8 = undefined;
    std.crypto.hash.sha2.Sha256.hash(try std.fmt.allocPrint(a, "{d}:{s}:{s}:{d}", .{ number, client.url, client.mint, client.decimals }), &id, .{});
    const params = try w.field(input, "params");
    switch (op) {
        .new => unreachable,
        .account_info => return client.info(try string(params, "address")),
        .token_balance => {
            const address = try string(params, "address");
            _ = try sol.publicKey(address);
            return client.call("getTokenAccountBalance", .{ address, .{ .commitment = "confirmed" } }, null);
        },
        .block_height => return client.height(),
        .pending => {
            var list = std.array_list.Managed(V).init(a);
            if (pending(id)) |p| try list.append(w.text(try sol.base58(a, 64, p.signature)));
            return .{ .array = list };
        },
        .prepare_deposit, .submit => {
            if (pending(id) != null) return error.PendingOutcome;
            return if (op == .submit) client.submit(id, params) else client.prepare(params);
        },
        .clear_reconciled_submission, .reconcile, .signature_status => {
            const name = try string(params, "signature");
            if (name.len < 64 or name.len > 88) return error.InvalidSignature;
            for (name) |c| if (std.mem.findScalar(u8, "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz", c) == null) return error.InvalidSignature;
            const outstanding = pending(id);
            const matches = if (outstanding) |p| eq(u8, try sol.base58(a, 64, p.signature), name) else false;
            if (op == .clear_reconciled_submission) {
                if (!matches) return error.InvalidSignature;
                try clear(id, outstanding.?.signature, true);
                return .null;
            }
            const status = try client.call("getSignatureStatuses", .{ .{name}, .{ .searchTransactionHistory = true } }, null);
            if (op == .reconcile and matches) {
                const list = try w.field(status, "value");
                if (list != .array or list.array.items.len != 1) return error.InvalidResponse;
                const row = list.array.items[0];
                if (row != .null) {
                    const err = try w.field(row, "err");
                    const level = row.object.get("confirmationStatus") orelse .null;
                    if (err != .null or (level == .string and (eq(u8, level.string, "confirmed") or eq(u8, level.string, "finalized")))) try clear(id, outstanding.?.signature, false);
                }
            }
            return status;
        },
    }
}
