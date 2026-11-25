const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const transport = @import("../../core/transport.zig");
const host = @import("../../core/host_stream.zig");
const Proto = @import("proto.zig").Encoder;
const sha256 = std.crypto.hash.sha2.Sha256;
const gpa = std.heap.c_allocator;
const Derivative = enum { limit, market };

const Identity = struct { address: [20]u8, testnet: bool };
const Pending = struct { hash: [32]u8, timeout_height: u64 };
var states: std.AutoHashMapUnmanaged(Identity, Pending) = .empty;
var states_mutex: std.Io.Mutex = .init;
var locks: [32]std.Io.Mutex = @splat(.init);

fn state(io: std.Io, identity: Identity, update: ?Pending, remove: bool) !?Pending {
    states_mutex.lockUncancelable(io);
    defer states_mutex.unlock(io);
    if (remove) {
        _ = states.remove(identity);
        return null;
    }
    if (update) |value| try states.put(gpa, identity, value);
    return states.get(identity);
}

fn polymod(previous: u32, value: u5) u32 {
    const generators = [_]u32{ 0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3 };
    var result = ((previous & 0x01ffffff) << 5) ^ value;
    for (generators, 0..) |generator, i| {
        if ((previous >> @as(u5, @intCast(25 + i))) & 1 != 0) result ^= generator;
    }
    return result;
}

fn bech32(address: [20]u8) [42]u8 {
    const alphabet = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";
    var result: [42]u8 = undefined;
    result[0..4].* = "inj1".*;
    var checksum: u32 = 1;
    for ("inj") |char| checksum = polymod(checksum, @intCast(char >> 5));
    checksum = polymod(checksum, 0);
    for ("inj") |char| checksum = polymod(checksum, @truncate(char));
    var bits: u16 = 0;
    var count: u4 = 0;
    var position: usize = 4;
    for (address) |byte| {
        bits = (bits << 8) | byte;
        count += 8;
        while (count >= 5) {
            count -= 5;
            const value: u5 = @truncate(bits >> count);
            checksum = polymod(checksum, value);
            result[position] = alphabet[value];
            position += 1;
        }
    }
    for (0..6) |_| checksum = polymod(checksum, 0);
    checksum ^= 1;
    for (0..6) |i| result[position + i] = alphabet[@as(u5, @truncate(checksum >> @as(u5, @intCast(5 * (5 - i)))))];
    return result;
}

fn fixed(a: p.A, value: p.V, scale: u8) ![]const u8 {
    const number = try w.string(value);
    var point: ?usize = null;
    for (number, 0..) |char, i| {
        if (char == '.' and point == null and i != 0 and i + 1 < number.len) {
            point = i;
        } else if (!std.ascii.isDigit(char)) return error.InvalidQuantity;
    }
    if (number.len == 0 or number.len > 77 or scale > 36) return error.InvalidQuantity;
    const integer = number[0 .. point orelse number.len];
    const fractional = if (point) |i| std.mem.trimEnd(u8, number[i + 1 ..], "0") else "";
    if (fractional.len > scale) return error.InvalidQuantity;
    const pad = try a.alloc(u8, scale - fractional.len);
    @memset(pad, '0');
    const untrimmed = try std.mem.concat(a, u8, &.{ integer, fractional, pad });
    const digits = std.mem.trimStart(u8, untrimmed, "0");
    if (digits.len == 0) return "0";
    _ = try std.fmt.parseInt(u256, digits, 10);
    return digits;
}

fn any(a: p.A, type_name: []const u8, value: []const u8) !Proto {
    var result = Proto{ .allocator = a };
    try result.data(1, type_name);
    try result.data(2, value);
    return result;
}

fn message(a: p.A, operation: []const u8, body: p.V, sender: []const u8, identity: Identity) !Proto {
    const market = try p.text(body, "market_id");
    _ = try evm.hex(32, market);
    const subaccount = try p.text(body, "subaccount_id");
    const account_id = try evm.hex(32, subaccount);
    if (!p.eq(u8, account_id[0..20], &identity.address)) return error.InvalidIdentity;
    const client_id = p.get(body, "client_order_id");
    const cid = if (client_id == .null) "" else try w.string(client_id);
    var msg = Proto{ .allocator = a };
    try msg.data(1, sender);
    if (p.eq(u8, operation, "cancel_order")) {
        const raw = p.get(body, "order_hash");
        const order_hash = if (raw == .null) "" else try w.string(raw);
        if ((order_hash.len == 0) == (cid.len == 0)) return error.InvalidParameter;
        if (order_hash.len != 0) _ = try evm.hex(32, order_hash);
        try msg.data(2, market);
        try msg.data(3, subaccount);
        try msg.data(4, order_hash);
        try msg.data(6, cid);
        return any(a, "/injective.exchange.v1beta1.MsgCancelDerivativeOrder", msg.slice());
    }
    const kind = std.meta.stringToEnum(Derivative, try p.text(body, "kind")) orelse return error.InvalidParameter;
    const side = try p.text(body, "side");
    const order_type: u64 = if (p.eq(u8, side, "buy")) 1 else if (p.eq(u8, side, "sell")) 2 else return error.InvalidParameter;
    const quote_decimals = try w.unsigned(u8, try w.field(body, "quote_decimals"));
    if (quote_decimals > 18) return error.InvalidQuantity;
    const price = try fixed(a, try w.field(body, "price"), 18 + quote_decimals);
    const quantity = try fixed(a, try w.field(body, "quantity"), 18);
    const margin = try fixed(a, try w.field(body, "margin"), 18 + quote_decimals);
    if (p.eq(u8, price, "0") or p.eq(u8, quantity, "0")) return error.InvalidQuantity;
    var info = Proto{ .allocator = a };
    try info.data(1, subaccount);
    const raw_fee_recipient = p.get(body, "fee_recipient");
    try info.data(2, if (raw_fee_recipient == .null) sender else try w.string(raw_fee_recipient));
    try info.data(3, price);
    try info.data(4, quantity);
    try info.data(5, cid);
    var order = Proto{ .allocator = a };
    try order.data(1, market);
    try order.data(2, info.slice());
    try order.integer(3, order_type);
    try order.data(4, margin);
    try msg.data(2, order.slice());
    return any(a, if (kind == .limit) "/injective.exchange.v1beta1.MsgCreateDerivativeLimitOrder" else "/injective.exchange.v1beta1.MsgCreateDerivativeMarketOrder", msg.slice());
}

fn authorization(a: p.A, key: [32]u8, body: p.V, sequence: u64) !Proto {
    const key_bytes = (try std.crypto.ecc.Secp256k1.basePoint.mul(key, .big)).toCompressedSec1();
    var pub_key = Proto{ .allocator = a };
    try pub_key.data(1, &key_bytes);
    const key_any = try any(a, "/injective.crypto.v1beta1.ethsecp256k1.PubKey", pub_key.slice());
    var single = Proto{ .allocator = a };
    try single.integer(1, 1);
    var mode = Proto{ .allocator = a };
    try mode.data(1, single.slice());
    var signer = Proto{ .allocator = a };
    try signer.data(1, key_any.slice());
    try signer.data(2, mode.slice());
    try signer.integer(3, sequence);

    const fee_amount = try p.text(body, "fee_amount");
    const gas = try w.unsigned(u64, try w.field(body, "gas_limit"));
    if (gas == 0 or try w.unsigned(u256, w.text(fee_amount)) == 0) return error.InvalidQuantity;
    var coin = Proto{ .allocator = a };
    try coin.data(1, "inj");
    try coin.data(2, fee_amount);
    var fee = Proto{ .allocator = a };
    try fee.data(1, coin.slice());
    try fee.integer(2, gas);

    var auth = Proto{ .allocator = a };
    try auth.data(1, signer.slice());
    try auth.data(2, fee.slice());
    return auth;
}

fn encode(a: p.A, msg: Proto, auth: Proto, chain: []const u8, account: u64, timeout: u64, key: [32]u8) ![]const u8 {
    var body = Proto{ .allocator = a };
    try body.data(1, msg.slice());
    try body.integer(3, timeout);
    var doc = Proto{ .allocator = a };
    try doc.data(1, body.slice());
    try doc.data(2, auth.slice());
    try doc.data(3, chain);
    try doc.integer(4, account);
    const signature = try @import("../../core/transaction.zig").recoverable(key, evm.hash(doc.slice()));
    var raw = Proto{ .allocator = a };
    try raw.data(1, body.slice());
    try raw.data(2, auth.slice());
    try raw.data(3, signature[0..64]);
    return raw.slice();
}

const Response = struct { status: u16, body: []const u8 };
fn http(a: p.A, client: *std.http.Client, base: []const u8, method: []const u8, path: []const u8, value: ?[]const u8, attempted: ?*bool) !Response {
    const url = try std.mem.concat(a, u8, &.{ std.mem.trimEnd(u8, base, "/"), path });
    var headers = p.object();
    try p.put(&headers, a, "content-type", w.text("application/json"));
    var req = try transport.sendTracked(a, client, method, url, headers, value, attempted);
    defer req.deinit();
    errdefer if (req.connection) |conn| {
        conn.closing = true;
    };
    var res = req.receiveHead(&.{}) catch return error.Network;
    var buffer: [8192]u8 = undefined;
    var decoder: std.http.Decompress = undefined;
    const reader = try transport.bodyReader(a, &res, &buffer, &decoder);
    return .{ .status = @backingInt(res.head.status), .body = reader.allocRemaining(a, .limited(4 * 1024 * 1024)) catch return error.Network };
}

fn decodeReply(a: p.A, reply: Response) !p.V {
    return @import("../../core/http.zig").decode(a, .injective, reply.status, reply.body);
}

fn txPath(a: p.A, hash: [32]u8) ![]const u8 {
    const encoded = try std.fmt.allocPrint(a, "{x}", .{&hash});
    return std.fmt.allocPrint(a, "/cosmos/tx/v1beta1/txs/{s}", .{try std.ascii.allocUpperString(a, encoded)});
}

pub fn execute(a: p.A, input: p.V, op: []const u8, dial: ?host.Dial) !p.V {
    const auth = p.get(input, "auth");
    var key = try p.evmKey(auth);
    defer std.crypto.secureZero(u8, &key);
    const identity = Identity{ .address = try p.evmAddress(key), .testnet = try p.testnet(input) };
    const chain = if (identity.testnet) "injective-888" else "injective-1";
    const sender = bech32(identity.address);
    const override = p.get(input, "origin");
    const base = if (override == .null) try p.origin(.injective, op, input, "") else try w.string(override);
    const uri = try std.Uri.parse(base);
    if (uri.host == null or uri.user != null or uri.password != null or uri.query != null or uri.fragment != null or
        !(p.eq(u8, uri.scheme, "http") or p.eq(u8, uri.scheme, "https"))) return error.InvalidOrigin;
    var runtime: host.Runtime = .{ .dial = dial };
    const io = runtime.io();
    const mutex = &locks[std.hash.Wyhash.hash(0, &identity.address) % locks.len];
    mutex.lockUncancelable(io);
    defer mutex.unlock(io);
    const previous = try state(io, identity, null, false);
    if (p.eq(u8, op, "pending")) {
        if (previous) |pending| {
            const hash = try std.fmt.allocPrint(a, "{x}", .{&pending.hash});
            return p.toValue(a, .{ .hash = try std.ascii.allocUpperString(a, hash), .timeout_height = pending.timeout_height });
        }
        return .null;
    }
    var client: std.http.Client = .{ .io = io, .allocator = gpa };
    defer client.deinit();
    const latest = try decodeReply(a, try http(a, &client, base, "GET", "/cosmos/base/tendermint/v1beta1/blocks/latest", null, null));
    const header = try w.field(try w.field(latest, "block"), "header");
    if (!p.eq(u8, try p.text(header, "chain_id"), chain)) return error.InvalidNetwork;
    const height = try w.unsigned(u64, try w.field(header, "height"));
    if (previous) |pending| {
        const reply = try http(a, &client, base, "GET", try txPath(a, pending.hash), null, null);
        if (reply.status == 404) {
            if (height <= pending.timeout_height) return error.PendingOutcome;
        } else {
            const confirmed = try decodeReply(a, reply);
            const hash = try p.text(try w.field(confirmed, "tx_response"), "txhash");
            if (!p.eq(u8, &try evm.hex(32, hash), &pending.hash)) return error.InvalidIdentity;
        }
        _ = try state(io, identity, null, true);
    }
    const body = try w.field(try w.field(input, "params"), "body");
    const msg = try message(a, op, body, &sender, identity);
    const account_path = try std.fmt.allocPrint(a, "/cosmos/auth/v1beta1/accounts/{s}", .{sender});
    const account = try w.field(try decodeReply(a, try http(a, &client, base, "GET", account_path, null, null)), "account");
    const nested = p.get(account, "base_account");
    const fields = if (nested == .null) account else nested;
    if (!p.eq(u8, try p.text(fields, "address"), &sender)) return error.InvalidIdentity;
    const sequence = try w.unsigned(u64, try w.field(fields, "sequence"));
    const account_number = try w.unsigned(u64, try w.field(fields, "account_number"));
    const timeout = try std.math.add(u64, height, 20);
    const tx = try encode(a, msg, try authorization(a, key, body, sequence), chain, account_number, timeout, key);
    const b64 = std.base64.standard.Encoder;
    const tx_bytes = b64.encode(try a.alloc(u8, b64.calcSize(tx.len)), tx);
    const payload = try std.json.Stringify.valueAlloc(a, .{ .tx_bytes = tx_bytes, .mode = "BROADCAST_MODE_SYNC" }, .{});
    var hash: [32]u8 = undefined;
    sha256.hash(tx, &hash, .{});
    _ = try state(io, identity, .{ .hash = hash, .timeout_height = timeout }, false);
    var attempted = false;
    const reply = http(a, &client, base, "POST", "/cosmos/tx/v1beta1/txs", payload, &attempted) catch |err| {
        if (attempted) return error.PendingOutcome;
        _ = try state(io, identity, null, true);
        return err;
    };
    if (reply.status >= 500 or reply.status == 408) return error.PendingOutcome;
    const result = decodeReply(a, reply) catch |err| {
        if (reply.status >= 200 and reply.status < 300) return error.PendingOutcome;
        _ = try state(io, identity, null, true);
        return err;
    };
    const tx_reply = w.field(result, "tx_response") catch return error.PendingOutcome;
    const tx_hash = p.text(tx_reply, "txhash") catch return error.PendingOutcome;
    const returned_hash = evm.hex(32, tx_hash) catch return error.PendingOutcome;
    if (!p.eq(u8, &returned_hash, &hash)) return error.PendingOutcome;
    const code = w.unsigned(u32, w.field(tx_reply, "code") catch return error.PendingOutcome) catch return error.PendingOutcome;
    if (code != 0) {
        _ = try state(io, identity, null, true);
        @import("../../core/http_failure.zig").record("injective", reply.status, reply.body);
        return error.ExchangeApi;
    }
    return result;
}
