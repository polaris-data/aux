const std = @import("std");
const p = @import("../../core/perps.zig");
const common = @import("../../core/perps_connectors.zig");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");

const Account = struct { address: [20]u8, chain: u32 };
const Nonce = struct { anchor: u48, index: u8 };
var nonce_lock: std.Io.Mutex = .init;
var nonces: std.AutoHashMapUnmanaged(Account, Nonce) = .empty;

fn nextNonce(account: Account, state: p.V, io: std.Io) !Nonce {
    const anchor = try w.unsigned(u48, try w.field(state, "nonce_anchor"));
    const index = try w.unsigned(u8, try w.field(state, "current_bitmap_index"));
    if (index > 208) return error.InvalidResponse;
    const last = try std.math.add(u48, anchor, 1);
    var next: Nonce = if (index == 208) .{ .anchor = last, .index = 0 } else .{ .anchor = anchor, .index = index };
    nonce_lock.lockUncancelable(io);
    defer nonce_lock.unlock(io);
    const entry = try nonces.getOrPut(std.heap.c_allocator, account);
    if (entry.found_existing) {
        const reserved = entry.value_ptr.*;
        if (reserved.anchor > next.anchor or (reserved.anchor == next.anchor and reserved.index > next.index))
            next = reserved;
    }
    if (next.index == 208 and next.anchor < last) next = .{ .anchor = last, .index = 0 };
    if (next.index >= 208 or next.anchor > last) return error.InvalidNonce;
    entry.value_ptr.* = .{ .anchor = next.anchor, .index = next.index + 1 };
    return next;
}

fn data(value: p.V) p.V {
    const inner = p.get(value, "data");
    return if (inner == .null) value else inner;
}

fn chainId(config: p.V, domain: p.V) !u32 {
    const chain = try w.unsigned(u32, try w.field(domain, "chain_id"));
    if (chain != (if (try p.testnet(config)) @as(u32, 11155931) else 4153)) return error.InvalidNetwork;
    return chain;
}

fn optionalValue(comptime T: type, object: p.V, key: []const u8) !T {
    const field = p.get(object, key);
    return if (field == .null) 0 else w.unsigned(T, field);
}

fn selector(a: p.A, order: *p.V, field: []const u8, comptime names: []const []const u8) !u8 {
    const v = p.get(order.*, field);
    if (v == .null) return 0;
    if (v == .string) {
        const index = try common.enumIndex(v.string, names);
        try p.put(order, a, field, try common.number(a, index));
        return index;
    }
    const index = try w.unsigned(u8, v);
    if (index >= names.len) return error.InvalidParameter;
    return index;
}

fn action(a: p.A, order: *p.V, place: bool) !p.Word {
    const market = try w.unsigned(u16, try w.field(order.*, "market_id"));
    if (market == 0) return error.InvalidMarket;
    if (!place) {
        _ = try evm.hex(24, try p.text(order.*, "order_id"));
        const resting = try w.unsigned(u64, try w.field(order.*, "resting_order_id"));
        _ = order.object.swapRemove("resting_order_id");
        return evm.hash(&(evm.hash("RISE_PERPS_CANCEL_ORDER_V1") ++ evm.encodeWord(market) ++ evm.encodeWord(resting)));
    }
    const size = try w.unsigned(u32, try w.field(order.*, "size_steps"));
    const price = try w.unsigned(u24, try w.field(order.*, "price_ticks"));
    const side = try selector(a, order, "side", &.{ "BUY", "SELL" });
    const stp = try selector(a, order, "stp_mode", &.{ "EXPIRE_MAKER", "EXPIRE_TAKER", "EXPIRE_BOTH" });
    const order_type = try selector(a, order, "order_type", &.{ "MARKET", "LIMIT" });
    const tif = try selector(a, order, "time_in_force", &.{ "GTC", "GTT", "FOK", "IOC" });
    const id = try optionalValue(u64, order.*, "client_order_id");
    const ttl = try optionalValue(u16, order.*, "ttl_units");
    const post_only = try common.boolean(order.*, "post_only");
    const reduce_only = try common.boolean(order.*, "reduce_only");
    if (size == 0 or price == 0 or (order_type == 0 and (tif < 2 or post_only)) or ((ttl != 0) != (tif == 1)))
        return error.InvalidParameter;
    if (p.get(order.*, "builder_id") != .null or p.get(order.*, "builder_fee_bps") != .null) return error.InvalidParameter;
    const flags = side | (@as(u8, @intFromBool(post_only)) << 1) | (@as(u8, @intFromBool(reduce_only)) << 2) | (stp << 3) | (order_type << 5) | (tif << 6);
    const packed_order = (@as(u88, market) << 70) | (@as(u88, size) << 38) | (@as(u88, price) << 14) | (@as(u88, flags) << 6) | 2;
    const header: u8 = 1 | (if (id != 0) @as(u8, 4) else 0) | (if (ttl != 0) @as(u8, 16) else 0);
    return evm.hash(&(evm.hash("RISE_PERPS_PLACE_ORDER_V1") ++ evm.encodeWord(header) ++ evm.encodeWord(packed_order) ++ evm.encodeWord(0) ++ evm.encodeWord(id) ++ evm.encodeWord(ttl)));
}

pub fn sign(a: p.A, config: p.V, io: std.Io, name: []const u8, input: p.V) !p.V {
    const auth = p.get(config, "auth");
    const account = try p.credential(p.get(auth, "account"));
    const address = try evm.hex(20, account);
    var key = try p.evmKey(auth);
    defer std.crypto.secureZero(u8, &key);
    const signer = try p.evmAddress(key);
    const metadata = p.get(config, "metadata");
    const domain = data(p.get(metadata, "signing_domain"));
    const chain = try chainId(config, domain);
    const target = try evm.address(try p.text(p.get(data(p.get(metadata, "system_config")), "addresses"), "router"));
    var order = input;
    if (order != .object or p.get(order, "permit") != .null) return error.InvalidParameter;
    const action_hash = try action(a, &order, p.eq(u8, name, "place_order"));
    const nonce = try nextNonce(.{ .address = address, .chain = chain }, data(p.get(metadata, "nonce_state")), io);
    const now = std.Io.Timestamp.now(io, .real).toSeconds();
    const deadline = std.math.cast(u32, now + 60) orelse return error.InvalidTimestamp;
    const witness = evm.hash(&(evm.hash("VerifyWitness(address account,address target,bytes32 hash,uint48 nonceAnchor,uint8 nonceBitmap,uint32 deadline)") ++
        try evm.address(account) ++ target ++ action_hash ++ evm.encodeWord(nonce.anchor) ++ evm.encodeWord(nonce.index) ++ evm.encodeWord(deadline)));
    const digest = p.typedHash(try p.text(domain, "name"), try p.text(domain, "version"), chain, try evm.address(try p.text(domain, "verifying_contract")), witness);
    const sig = try @import("../../core/transaction.zig").recoverable(key, digest);
    const base64 = std.base64.standard.Encoder;
    try p.put(&order, a, "permit", try p.toValue(a, .{
        .account = account,
        .signer = try std.fmt.allocPrint(a, "0x{x}", .{&signer}),
        .nonce_anchor = try std.fmt.allocPrint(a, "{d}", .{nonce.anchor}),
        .nonce_bitmap_index = nonce.index,
        .deadline = deadline,
        .signature = base64.encode(try a.alloc(u8, base64.calcSize(sig.len)), &sig),
    }));
    return order;
}

pub fn signIn(a: p.A, config: p.V, dial: ?@import("../../core/host_stream.zig").Dial) ![]const u8 {
    const auth = p.get(config, "auth");
    const account = try p.credential(p.get(auth, "account"));
    _ = try evm.hex(20, account);
    var key = try p.evmKey(auth);
    defer std.crypto.secureZero(u8, &key);
    const signer = try p.evmAddress(key);
    const domain = data(try common.lookup(a, config, .risex, "signing_domain", .{}, dial));
    const chain = try chainId(config, domain);
    const nonce = try p.text(data(try common.lookup(a, config, .risex, "auth_nonce", .{}, dial)), "nonce");
    const nonce_bytes = try evm.hex(32, nonce);
    const message = "WebSocket Authentication";
    const hash = evm.hash(&(evm.hash("RegisterV2(address signer,string message,uint256 nonce)") ++
        evm.encodeWord(std.mem.readInt(u160, &signer, .big)) ++ evm.hash(message) ++ nonce_bytes));
    const digest = p.typedHash(try p.text(domain, "name"), try p.text(domain, "version"), chain, try evm.address(try p.text(domain, "verifying_contract")), hash);
    return std.json.Stringify.valueAlloc(a, .{ .method = "auth_v2", .params = .{
        .account = account,
        .signer = try std.fmt.allocPrint(a, "0x{x}", .{&signer}),
        .message = message,
        .nonce = nonce,
        .signature = try p.signature(a, key, digest),
    } }, .{});
}

test "RISEx router action hashes match eth-abi" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    var place = try p.toValue(a, .{
        .market_id = 1,
        .size_steps = 200,
        .price_ticks = 550248,
        .side = "BUY",
        .order_type = "LIMIT",
        .time_in_force = "GTC",
        .client_order_id = "9007199254740993",
    });
    var cancellation = try p.toValue(a, .{
        .market_id = 1,
        .order_id = "0x000000000000000000000000000000000000000000000001",
        .resting_order_id = "123",
    });
    try std.testing.expectEqual(try evm.hex(32, "8b567fd156948595ef7659a6a20eea6d87e8bc945a9efe6ea9cfac10a0e7f6f6"), try action(a, &place, true));
    try std.testing.expectEqual(try evm.hex(32, "c7707b01aa51c089c9f4ca742a74928fc6c9d6a77dc15d6fd7fe8c3c2b6c46de"), try action(a, &cancellation, false));
    try std.testing.expect(p.get(cancellation, "resting_order_id") == .null);
}
