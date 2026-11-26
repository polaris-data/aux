const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const transport = @import("../../core/transport.zig");
const host = @import("../../core/host_stream.zig");
const exchange = @import("../../core/exchange_http.zig");
const Proto = @import("../injective/proto.zig").Encoder;
const identity = @import("identity.zig");
const Sha256 = std.crypto.hash.sha2.Sha256;
const gpa = std.heap.c_allocator;
const eq = std.mem.eql;
const A = w.A;
const V = w.V;

const Pending = struct { hash: [32]u8, timeout: u64 };
const Stripe = struct { mutex: std.Io.Mutex = .init, submissions: std.AutoHashMapUnmanaged([32]u8, Pending) = .empty };
var accounts: [32]Stripe = @splat(.{});

fn number(comptime T: type, v: V, field: []const u8) !T {
    return w.unsigned(T, try w.field(v, field));
}

fn optional(comptime T: type, v: V, field: []const u8) !T {
    const raw = p.get(v, field);
    return if (raw == .null) 0 else w.unsigned(T, raw);
}

fn any(a: A, url: []const u8, value: Proto) !Proto {
    var envelope: Proto = .{ .allocator = a };
    try envelope.data(1, url);
    try envelope.data(2, value.slice());
    return envelope;
}

fn subaccount(a: A, v: V, sender: []const u8) !Proto {
    const provided = p.get(v, "address");
    if (provided != .null and !eq(u8, try w.string(provided), sender)) return error.InvalidIdentity;
    var sub: Proto = .{ .allocator = a };
    try sub.data(1, sender);
    try sub.integer(2, try number(u32, v, "subaccount_number"));
    return sub;
}

fn orderId(a: A, v: V, sender: []const u8, flags: u32) !Proto {
    var id: Proto = .{ .allocator = a };
    try id.data(1, (try subaccount(a, v, sender)).slice());
    try id.fixed32(2, try number(u32, v, "client_id"));
    try id.integer(3, flags);
    try id.integer(4, try number(u32, v, "clob_pair_id"));
    return id;
}

const Expiry = union(enum) { block: u32, time: u32 };

fn expiry(v: V, flags: u32, height: u64, chain_time: u64) !Expiry {
    const block = try optional(u32, v, "good_til_block");
    const time = try optional(u32, v, "good_til_block_time");
    if (flags == 0 and block > height and block <= height +| 20 and time == 0) return .{ .block = block };
    if ((flags == 32 or flags == 64) and block == 0 and time > chain_time) return .{ .time = time };
    return error.InvalidOrderExpiry;
}

fn orderMessage(a: A, v: V, sender: []const u8, flags: u32, until: Expiry, cancel: bool) !Proto {
    var msg: Proto = .{ .allocator = a };
    const id = try orderId(a, v, sender, flags);
    if (cancel) {
        try msg.data(1, id.slice());
        switch (until) {
            .block => |block| try msg.integer(2, block),
            .time => |time| try msg.fixed32(3, time),
        }
        return any(a, "/dydxprotocol.clob.MsgCancelOrder", msg);
    }

    var order: Proto = .{ .allocator = a };
    const side = try number(u32, v, "side");
    const quantums = try number(u64, v, "quantums");
    const subticks = try number(u64, v, "subticks");
    if ((side != 1 and side != 2) or quantums == 0 or subticks == 0) return error.InvalidParameter;
    try order.data(1, id.slice());
    try order.integer(2, side);
    try order.integer(3, quantums);
    try order.integer(4, subticks);
    switch (until) {
        .block => |block| try order.integer(5, block),
        .time => |time| try order.fixed32(6, time),
    }
    const tif = try optional(u32, v, "time_in_force");
    if (tif > 2) return error.InvalidParameter;
    try order.integer(7, tif);
    const reduce_only = p.get(v, "reduce_only");
    if (reduce_only != .null and reduce_only != .bool) return error.InvalidParameter;
    try order.integer(8, @intFromBool(reduce_only == .bool and reduce_only.bool));
    try order.integer(9, try optional(u32, v, "client_metadata"));
    const condition = try optional(u32, v, "condition_type");
    const trigger = try optional(u64, v, "conditional_order_trigger_subticks");
    if (flags == 32) {
        if ((condition != 1 and condition != 2) or trigger == 0) return error.InvalidParameter;
    } else if (condition != 0 or trigger != 0) return error.InvalidParameter;
    try order.integer(10, condition);
    try order.integer(11, trigger);
    try msg.data(1, order.slice());
    return any(a, "/dydxprotocol.clob.MsgPlaceOrder", msg);
}

fn batchCancel(a: A, v: V, sender: []const u8, block: u32) !Proto {
    var msg: Proto = .{ .allocator = a };
    const groups = try w.field(v, "short_term_cancels");
    if (groups != .array or groups.array.items.len == 0) return error.InvalidParameter;
    try msg.data(1, (try subaccount(a, v, sender)).slice());
    for (groups.array.items) |group| {
        var item: Proto = .{ .allocator = a };
        try item.integer(1, try number(u32, group, "clob_pair_id"));
        const ids = try w.field(group, "client_ids");
        if (ids != .array or ids.array.items.len == 0) return error.InvalidParameter;
        const values = try a.alloc(u32, ids.array.items.len);
        for (ids.array.items, values) |id, *value| value.* = try w.unsigned(u32, id);
        try item.packedU32(2, values);
        try msg.data(2, item.slice());
    }
    try msg.integer(3, block);
    return any(a, "/dydxprotocol.clob.MsgBatchCancel", msg);
}

fn encode(a: A, tx_body: Proto, config: V, public: [33]u8, key: [32]u8, chain: []const u8, account: u64, sequence: u64) ![]const u8 {
    var pk: Proto = .{ .allocator = a };
    try pk.data(1, &public);
    const pk_any = try any(a, "/cosmos.crypto.secp256k1.PubKey", pk);
    var single: Proto = .{ .allocator = a };
    try single.integer(1, 1); // SIGN_MODE_DIRECT
    var mode: Proto = .{ .allocator = a };
    try mode.data(1, single.slice());
    var signer: Proto = .{ .allocator = a };
    try signer.data(1, pk_any.slice());
    try signer.data(2, mode.slice());
    try signer.integer(3, sequence);
    var fee: Proto = .{ .allocator = a };
    const gas = try optional(u64, config, "gas_limit");
    const amount = p.get(config, "fee_amount");
    if (amount != .null) {
        const fee_units = try w.unsigned(u256, amount);
        if (fee_units != 0) {
            const denom = try p.text(config, "fee_denom");
            if (denom.len == 0 or gas == 0) return error.InvalidParameter;
            var coin: Proto = .{ .allocator = a };
            try coin.data(1, denom);
            try coin.data(2, try std.fmt.allocPrint(a, "{d}", .{fee_units}));
            try fee.data(1, coin.slice());
        }
    }
    try fee.integer(2, gas);

    var auth: Proto = .{ .allocator = a };
    try auth.data(1, signer.slice());
    // AuthInfo requires the Fee message even for gasless short-term orders.
    if (fee.slice().len == 0) try auth.bytes.appendSlice(a, &.{ 0x12, 0 }) else try auth.data(2, fee.slice());
    var sign_doc: Proto = .{ .allocator = a };
    try sign_doc.data(1, tx_body.slice());
    try sign_doc.data(2, auth.slice());
    try sign_doc.data(3, chain);
    try sign_doc.integer(4, account);
    var digest: [32]u8 = undefined;
    Sha256.hash(sign_doc.slice(), &digest, .{});
    const sig = try @import("../../core/transaction.zig").recoverable(key, digest);
    var signed: Proto = .{ .allocator = a };
    try signed.data(1, tx_body.slice());
    try signed.data(2, auth.slice());
    try signed.data(3, sig[0..64]);
    return signed.slice();
}

const Response = struct { status: u16, body: []const u8 };

fn request(a: A, client: *std.http.Client, base: []const u8, path: []const u8, payload: ?[]const u8, attempted: ?*bool) !Response {
    const url = try std.mem.concat(a, u8, &.{ std.mem.trimEnd(u8, base, "/"), path });
    var headers = p.object();
    try exchange.set(a, &headers, "content-type", "application/json");
    var req = try transport.sendTracked(a, client, if (payload == null) "GET" else "POST", url, headers, payload, attempted);
    defer req.deinit();
    errdefer if (req.connection) |conn| {
        conn.closing = true;
    };
    var head = req.receiveHead(&.{}) catch return error.Network;
    var buffer: [8192]u8 = undefined;
    var decompression: std.http.Decompress = undefined;
    const reader = try transport.bodyReader(a, &head, &buffer, &decompression);
    return .{
        .status = @backingInt(head.head.status),
        .body = try reader.allocRemaining(a, .limited(4 * 1024 * 1024)),
    };
}

fn response(a: A, reply: Response) !V {
    return @import("../../core/http.zig").decode(a, .dydx_v4, reply.status, reply.body);
}

fn txPath(a: A, hash: [32]u8) ![]const u8 {
    return std.fmt.allocPrint(a, "/cosmos/tx/v1beta1/txs/{s}", .{try hashText(a, hash)});
}

fn hashText(a: A, hash: [32]u8) ![]const u8 {
    return std.ascii.allocUpperString(a, try std.fmt.allocPrint(a, "{x}", .{&hash}));
}

fn pending(a: A, hash: [32]u8) error{PendingOutcome} {
    const text = hashText(a, hash) catch return error.PendingOutcome;
    const detail = std.json.Stringify.valueAlloc(a, .{ .hash = text }, .{}) catch return error.PendingOutcome;
    @import("../../core/http_failure.zig").record("dydx_v4", 0, detail);
    return error.PendingOutcome;
}

pub fn execute(a: A, input: V, op: []const u8, dial: ?host.Dial) !V {
    const auth = p.get(input, "auth");
    var key = try p.evmKey(auth);
    defer std.crypto.secureZero(u8, &key);
    const pub_key = (try std.crypto.ecc.Secp256k1.basePoint.mul(key, .big)).toCompressedSec1();
    const sender = identity.address(pub_key);
    const configured = p.get(auth, "chain_id");
    const chain = if (configured != .null) try p.credential(configured) else if (try p.testnet(input)) "dydx-testnet-4" else "dydx-mainnet-1";
    const origin = try @import("../../core/rpc.zig").checkedUrl(try p.credential(p.get(auth, "node_origin")));
    var wallet_id: [32]u8 = undefined;
    var hasher = Sha256.init(.{});
    hasher.update(&sender);
    hasher.update(chain);
    hasher.final(&wallet_id);
    var runtime: host.Runtime = .{ .dial = dial };
    const io = runtime.io();
    const shard = &accounts[wallet_id[0] % accounts.len];
    shard.mutex.lockUncancelable(io);
    defer shard.mutex.unlock(io);
    var client: std.http.Client = .{ .io = io, .allocator = gpa };
    defer client.deinit();

    const params = try w.field(input, "params");
    const body = try w.field(params, "body");
    const batch = eq(u8, op, "place_orders") or eq(u8, op, "cancel_stateful_orders");
    const multi_cancel = eq(u8, op, "cancel_orders");
    const flags = if (batch or multi_cancel) 0 else try number(u32, body, "order_flags");
    const stateful = batch or flags != 0;
    const latest = try response(a, try request(a, &client, origin, "/cosmos/base/tendermint/v1beta1/blocks/latest", null, null));
    const block = p.get(latest, "block");
    const header = try w.field(if (block == .null) try w.field(latest, "sdk_block") else block, "header");
    if (!eq(u8, try p.text(header, "chain_id"), chain)) return error.InvalidNetwork;
    const height = try number(u64, header, "height");
    const wall_ms = try p.epoch(io, .ms);
    const chain_time = wall_ms / 1000;
    if (stateful) if (shard.submissions.get(wallet_id)) |old| {
        const lookup = try request(a, &client, origin, try txPath(a, old.hash), null, null);
        if (lookup.status == 404) {
            if (height <= old.timeout) return pending(a, old.hash);
        } else {
            const found = try w.field(try response(a, lookup), "tx_response");
            if (!std.ascii.eqlIgnoreCase(try p.text(found, "txhash"), try hashText(a, old.hash))) return error.InvalidIdentity;
        }
        _ = shard.submissions.remove(wallet_id);
    };

    var tx: Proto = .{ .allocator = a };
    if (multi_cancel) {
        const until = try expiry(body, 0, height, chain_time);
        try tx.data(1, (try batchCancel(a, body, &sender, until.block)).slice());
    } else if (batch) {
        const orders = try w.field(body, "orders");
        if (orders != .array or orders.array.items.len == 0 or orders.array.items.len > 100) return error.InvalidParameter;
        for (orders.array.items) |item| {
            const kinds = try number(u32, item, "order_flags");
            if (kinds == 0) return error.InvalidParameter;
            const until = try expiry(item, kinds, height, chain_time);
            try tx.data(1, (try orderMessage(a, item, &sender, kinds, until, eq(u8, op, "cancel_stateful_orders"))).slice());
        }
    } else {
        if (eq(u8, op, "replace_order") and flags != 0) return error.InvalidParameter;
        const until = try expiry(body, flags, height, chain_time);
        try tx.data(1, (try orderMessage(a, body, &sender, flags, until, eq(u8, op, "cancel_order"))).slice());
    }
    const timeout = height +| 20;
    if (stateful) try tx.integer(3, timeout);

    const account_path = try std.fmt.allocPrint(a, "/cosmos/auth/v1beta1/accounts/{s}", .{&sender});
    const account = try w.field(try response(a, try request(a, &client, origin, account_path, null, null)), "account");
    if (!eq(u8, try p.text(account, "address"), &sender)) return error.InvalidIdentity;
    const tx_bytes = try encode(a, tx, body, pub_key, key, chain, try number(u64, account, "account_number"), try number(u64, account, "sequence"));
    var hash: [32]u8 = undefined;
    Sha256.hash(tx_bytes, &hash, .{});
    const payload = try std.json.Stringify.valueAlloc(a, .{
        .tx_bytes = try exchange.encode(a, std.base64.standard.Encoder, tx_bytes),
        .mode = "BROADCAST_MODE_SYNC",
    }, .{});
    if (stateful) try shard.submissions.put(gpa, wallet_id, .{ .hash = hash, .timeout = timeout });
    var sent = false;
    const reply = request(a, &client, origin, "/cosmos/tx/v1beta1/txs", payload, &sent) catch |err| {
        if (sent) return pending(a, hash);
        if (stateful) _ = shard.submissions.remove(wallet_id);
        return err;
    };
    if (reply.status == 408 or reply.status >= 500) return pending(a, hash);
    if (reply.status >= 400) {
        if (stateful) _ = shard.submissions.remove(wallet_id);
        return response(a, reply);
    }
    const result = response(a, reply) catch return pending(a, hash);
    const broadcast = w.field(result, "tx_response") catch return pending(a, hash);
    const tx_hash = p.text(broadcast, "txhash") catch return pending(a, hash);
    if (!std.ascii.eqlIgnoreCase(tx_hash, try hashText(a, hash))) return pending(a, hash);
    if ((number(u32, broadcast, "code") catch return pending(a, hash)) != 0) {
        if (stateful) _ = shard.submissions.remove(wallet_id);
        @import("../../core/http_failure.zig").record("dydx_v4", reply.status, reply.body);
        return error.ExchangeApi;
    }
    return result;
}
