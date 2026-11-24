// Hold the exact reviewed submission across transport failures and concurrent
// calls. A successful RPC response also remains pending until reconciliation.
const std = @import("std");
const w = @import("wire.zig");
const rpc = @import("rpc.zig");
const tx = @import("phoenix_transaction.zig");
const sol = @import("solana.zig");
const A = w.A;
const V = w.V;
const Op = enum { new, latest_blockhash, block_height, prepare, simulate, submit, pending, clear_reconciled_submission, signature_status, transaction };
const Ticket = struct { bytes: [1644]u8, len: usize, signature: [64]u8, height: u64, sending: bool };
var mutex: std.atomic.Mutex = .unlocked;
var pending: std.AutoHashMapUnmanaged(u64, Ticket) = .empty;
var next_id = std.atomic.Value(u64).init(1);
fn lock() void {
    while (!mutex.tryLock()) std.atomic.spinLoopHint();
}
fn reserve(id: u64, token: Ticket) !void {
    lock();
    defer mutex.unlock();
    const slot = try pending.getOrPut(std.heap.c_allocator, id);
    if (slot.found_existing) return error.PendingOutcome;
    slot.value_ptr.* = token;
}
fn finish(id: u64, sent: bool) void {
    lock();
    defer mutex.unlock();
    if (sent) pending.getPtr(id).?.sending = false else _ = pending.remove(id);
}
fn string(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn call(a: A, url: []const u8, method: []const u8, args: anytype, attempted: ?*bool) !V {
    const params = try w.parse(a, try std.json.Stringify.valueAlloc(a, args, .{}));
    return rpc.send(a, try rpc.envelope(a, url, method, params), attempted);
}
const confirmed = .{ .commitment = "confirmed" };
fn height(a: A, url: []const u8) !u64 {
    const value = try call(a, url, "getBlockHeight", .{confirmed}, null);
    if (value != .number_string) return error.InvalidHeight;
    return w.unsigned(u64, value);
}
pub fn execute(a: A, input: V) !V {
    const url = try rpc.endpoint(input);
    const op = std.meta.stringToEnum(Op, try string(input, "operation")) orelse return error.InvalidOperation;
    if (op == .new) return w.text(try std.fmt.allocPrint(a, "{d}", .{next_id.fetchAdd(1, .monotonic)}));
    const id = try w.unsigned(u64, try w.field(input, "client"));
    const p = try w.field(input, "params");
    switch (op) {
        .new => unreachable,
        .pending, .clear_reconciled_submission => {
            lock();
            defer mutex.unlock();
            const entry = pending.get(id) orelse return .null;
            if (op == .clear_reconciled_submission) {
                if (entry.sending) return error.PendingOutcome;
                _ = pending.remove(id);
                return .null;
            }
            return w.parse(a, try std.json.Stringify.valueAlloc(a, .{ .transaction_base64 = entry.bytes[0..entry.len], .signature = try sol.base58(a, 64, entry.signature), .last_valid_block_height = entry.height }, .{}));
        },
        .block_height => return .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{try height(a, url)}) },
        .latest_blockhash, .prepare => {
            const reply = try call(a, url, "getLatestBlockhash", .{confirmed}, null);
            if (op == .latest_blockhash) return reply;
            const latest = try w.field(reply, "value");
            if (p != .object) return error.InvalidRequest;
            var req: V = .{ .object = try p.object.clone(a) };
            try req.object.put(a, "recent_blockhash", try w.field(latest, "blockhash"));
            try req.object.put(a, "last_valid_block_height", try w.field(latest, "lastValidBlockHeight"));
            return tx.execute(a, .prepare_transaction, req);
        },
        .signature_status, .transaction => {
            const signature = try string(p, "signature");
            _ = try sol.decode58(64, signature);
            if (op == .signature_status) return call(a, url, "getSignatureStatuses", .{ .{signature}, .{ .searchTransactionHistory = true } }, null);
            return call(a, url, "getTransaction", .{ signature, .{ .encoding = "json", .commitment = "confirmed", .maxSupportedTransactionVersion = @as(u8, 0) } }, null);
        },
        .simulate, .submit => {
            try tx.validate(a, p);
            const bytes = try string(p, "transaction_base64");
            if (op == .simulate) return call(a, url, "simulateTransaction", .{ bytes, .{ .encoding = "base64", .sigVerify = true, .commitment = "confirmed" } }, null);
            const sig = try string(p, "signature");
            var entry = Ticket{ .bytes = undefined, .len = bytes.len, .signature = try sol.decode58(64, sig), .height = try w.unsigned(u64, try w.field(p, "last_valid_block_height")), .sending = true };
            if (try height(a, url) > entry.height) return error.Expired;
            @memcpy(entry.bytes[0..bytes.len], bytes);
            try reserve(id, entry);
            var attempted = false;
            defer finish(id, attempted);
            const reply = call(a, url, "sendTransaction", .{ bytes, .{ .encoding = "base64", .maxRetries = @as(u8, 0), .skipPreflight = false } }, &attempted) catch |err| return if (attempted) error.PendingOutcome else err;
            if (reply != .string or !std.mem.eql(u8, reply.string, sig)) return error.PendingOutcome;
            return reply;
        },
    }
}
