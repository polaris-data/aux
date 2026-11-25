const std = @import("std");
const p = @import("../../core/perps.zig");
const common = @import("../../core/perps_connectors.zig");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const Bytes = std.ArrayList(u8);
const Sha3 = std.crypto.hash.sha3.Sha3_256;
const Ed25519 = std.crypto.sign.Ed25519;

const mainnet_package = "50ead22afd6ffd9769e3b3d6e0e64a2a350d68e8b102c4e72e33d0b8cfdfdb06";
const testnet_package = "d368b820fdfbfe004679cdb0facef4b920c4fc0c2c80bd2fecf3843a93427bd1";

pub fn address(text: []const u8) ![32]u8 {
    if (!std.mem.startsWith(u8, text, "0x") or text.len < 3 or text.len > 66) return error.InvalidIdentity;
    var padded: [64]u8 = @splat('0');
    @memcpy(padded[66 - text.len ..], text[2..]);
    return evm.hex(32, &padded);
}

fn int(out: *Bytes, a: p.A, value: anytype) !void {
    var bytes: [@sizeOf(@TypeOf(value))]u8 = undefined;
    std.mem.writeInt(@TypeOf(value), &bytes, value, .little);
    try out.appendSlice(a, &bytes);
}

fn length(out: *Bytes, a: p.A, n: usize) !void {
    var value = std.math.cast(u32, n) orelse return error.BodyTooLarge;
    while (value >= 128) : (value >>= 7) try out.append(a, @as(u8, @truncate(value)) | 128);
    try out.append(a, @intCast(value));
}

fn vector(out: *Bytes, a: p.A, bytes: []const u8) !void {
    try length(out, a, bytes.len);
    try out.appendSlice(a, bytes);
}

fn argument(out: *Bytes, a: p.A, value: anytype) !void {
    try length(out, a, @sizeOf(@TypeOf(value)));
    try int(out, a, value);
}

fn marketConfig(metadata: p.V, market: [32]u8) !p.V {
    const markets = p.get(metadata, "markets");
    if (markets != .array) return error.InvalidResponse;
    for (markets.array.items) |item| {
        if (p.eq(u8, &(try address(try p.text(item, "market_addr"))), &market)) {
            if (!p.eq(u8, try p.text(item, "asset_type"), "perp")) return error.InvalidMarket;
            return item;
        }
    }
    return error.InvalidMarket;
}

fn placeArgs(out: *Bytes, a: p.A, subaccount: [32]u8, market: [32]u8, spec: p.V, metadata: p.V) !void {
    const market_config = try marketConfig(metadata, market);
    const price = try common.decimal(u64, try p.text(spec, "price"), try w.unsigned(u8, try w.field(market_config, "px_decimals")));
    const size = try common.decimal(u64, try p.text(spec, "size"), try w.unsigned(u8, try w.field(market_config, "sz_decimals")));
    const tick = try w.unsigned(u64, try w.field(market_config, "tick_size"));
    const lot = try w.unsigned(u64, try w.field(market_config, "lot_size"));
    const minimum = try w.unsigned(u64, try w.field(market_config, "min_size"));
    const side = try w.field(spec, "is_buy");
    const tif_value = p.get(spec, "time_in_force");
    const tif = if (tif_value == .null) 0 else try w.unsigned(u8, tif_value);
    const reduce = try common.boolean(spec, "reduce_only");
    if (tick == 0 or lot == 0 or price == 0 or size < minimum or size == 0 or price % tick != 0 or size % lot != 0 or side != .bool or tif > 2)
        return error.InvalidQuantity;
    const client_id = p.get(spec, "client_order_id");
    for (spec.object.keys()) |key| {
        const valid = for ([_][]const u8{ "market", "price", "size", "is_buy", "time_in_force", "reduce_only", "client_order_id" }) |name| {
            if (p.eq(u8, key, name)) break true;
        } else false;
        if (!valid) return error.InvalidParameter;
    }
    try length(out, a, 16);
    try vector(out, a, &subaccount);
    try vector(out, a, &market);
    try argument(out, a, price);
    try argument(out, a, size);
    try argument(out, a, @as(u8, @intFromBool(side.bool)));
    try argument(out, a, tif);
    try argument(out, a, @as(u8, @intFromBool(reduce)));
    if (client_id == .null) {
        try vector(out, a, &.{0});
    } else {
        const id = try w.string(client_id);
        if (id.len == 0 or id.len > 32) return error.InvalidParameter;
        var option: Bytes = .empty;
        try option.append(a, 1);
        try vector(&option, a, id);
        try vector(out, a, option.items);
    }
    // Stop, take-profit, stop-loss and builder options.
    for (0..8) |_| try vector(out, a, &.{0});
}

pub fn build(a: p.A, config: p.V, input: p.V, operation: []const u8, io: std.Io) ![]const u8 {
    if (input != .object) return error.InvalidParameter;
    const auth = p.get(config, "auth");
    const metadata = p.get(config, "metadata");
    const testnet = try p.testnet(config);
    const chain = try w.unsigned(u8, try w.field(p.get(metadata, "ledger"), "chain_id"));
    if (chain != (if (testnet) @as(u8, 2) else 1)) return error.InvalidNetwork;
    const package = try evm.hex(32, if (testnet) testnet_package else mainnet_package);
    const subaccount = try address(try p.credential(p.get(auth, "subaccount")));
    const market = try address(try p.text(input, "market"));
    const gas_price = try w.unsigned(u64, try w.field(p.get(metadata, "gas_price"), "gas_estimate"));
    const max_gas_value = p.get(auth, "max_gas_amount");
    const max_gas = if (max_gas_value == .null) 200_000 else try w.unsigned(u64, max_gas_value);
    if (gas_price == 0 or max_gas == 0) return error.InvalidQuantity;

    var seed = try evm.hex(32, try p.credential(p.get(auth, "seed")));
    defer std.crypto.secureZero(u8, &seed);
    var key = try Ed25519.KeyPair.generateDeterministic(seed);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&key));
    const public = key.public_key.toBytes();
    var sender: [32]u8 = undefined;
    Sha3.hash(&(public ++ .{0}), &sender, .{});

    var out: Bytes = .empty;
    try out.appendSlice(a, &sender);
    try int(&out, a, @as(u64, 0xdeadbeef));
    // TransactionPayload::Payload(V1(EntryFunction(...), ExtraConfig::V1(...))).
    try out.appendSlice(a, &.{ 4, 0, 1 });
    try out.appendSlice(a, &package);
    try vector(&out, a, "dex_accounts_entry");
    const place = p.eq(u8, operation, "place_order");
    try vector(&out, a, if (place) "place_order_to_subaccount" else "cancel_order_to_subaccount");
    try length(&out, a, 0);
    if (place) {
        try placeArgs(&out, a, subaccount, market, input, metadata);
    } else {
        if (input.object.count() != 2) return error.InvalidParameter;
        const id = try w.unsigned(u128, try w.field(input, "order_id"));
        if (id == 0) return error.InvalidParameter;
        try length(&out, a, 3);
        try vector(&out, a, &subaccount);
        try argument(&out, a, id);
        try vector(&out, a, &market);
    }
    // ExtraConfig V1, absent multisig address, present replay protection nonce.
    try out.appendSlice(a, &.{ 0, 0, 1 });
    var random: [8]u8 = undefined;
    try io.randomSecure(&random);
    try out.appendSlice(a, &random);
    try int(&out, a, max_gas);
    try int(&out, a, gas_price);
    const now = std.math.cast(u64, std.Io.Timestamp.now(io, .real).toSeconds()) orelse return error.InvalidTimestamp;
    try int(&out, a, now + 30);
    try out.append(a, chain);
    var domain: [32]u8 = undefined;
    Sha3.hash("APTOS::RawTransaction", &domain, .{});
    const message = try std.mem.concat(a, u8, &.{ &domain, out.items });
    const signature = (try key.sign(message, null)).toBytes();
    try out.append(a, 0); // TransactionAuthenticator::Ed25519
    try vector(&out, a, &public);
    try vector(&out, a, &signature);
    return out.items;
}

pub fn hash(a: p.A, signed: []const u8) ![]const u8 {
    var domain: [32]u8 = undefined;
    Sha3.hash("APTOS::Transaction", &domain, .{});
    var digest = Sha3.init(.{});
    digest.update(&domain);
    digest.update(&.{0}); // Transaction::UserTransaction
    digest.update(signed);
    return std.fmt.allocPrint(a, "0x{x}", .{&digest.finalResult()});
}
