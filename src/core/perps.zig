// Shared configuration and signing primitives for perpetuals exchanges.
const std = @import("std");
const w = @import("wire.zig");
const evm = @import("evm.zig");
const Venue = @import("routes.zig").Venue;
pub const eq = std.mem.eql;
pub const A = w.A;
pub const V = w.V;
pub const Word = [32]u8;

pub fn get(value: V, name: []const u8) V {
    return if (value == .object) value.object.get(name) orelse .null else .null;
}

pub fn text(value: V, name: []const u8) ![]const u8 {
    return w.string(try w.field(value, name));
}

pub fn object() V {
    return .{ .object = .empty };
}

pub fn array(a: A, items: []const V) !V {
    var value = V{ .array = .init(a) };
    try value.array.appendSlice(items);
    return value;
}

pub fn put(value: *V, a: A, key: []const u8, field: V) !void {
    if (value.* != .object) return error.ExpectedObject;
    try value.object.put(a, key, field);
}

pub fn toValue(a: A, value: anytype) !V {
    return w.parse(a, try std.json.Stringify.valueAlloc(a, value, .{}));
}

pub fn credential(value: V) ![]const u8 {
    if (value != .string or value.string.len == 0) return error.AuthenticationRequired;
    for (value.string) |c| if (c < 0x21 or c > 0x7e) return error.InvalidHeader;
    return value.string;
}

pub fn testnet(config: V) !bool {
    return testnetAuth(get(config, "auth"));
}

pub fn testnetAuth(auth: V) !bool {
    const network = get(auth, "network");
    if (network == .null) return false;
    const name = try w.string(network);
    if (eq(u8, name, "Mainnet")) return false;
    if (eq(u8, name, "Testnet")) return true;
    return error.InvalidNetwork;
}

pub fn origin(venue: Venue, op: []const u8, config: V, fallback: []const u8) ![]const u8 {
    const test_network = try testnet(config);
    switch (venue) {
        .injective => {
            if (isWrite(venue, op) or eq(u8, op, "transaction") or eq(u8, op, "pending")) {
                const lcd = get(get(config, "auth"), "lcd_origin");
                if (lcd != .null) return try w.string(lcd);
                return if (test_network) "https://testnet.sentry.lcd.injective.network" else "https://sentry.lcd.injective.network";
            }
            return if (test_network) "https://k8s.testnet.exchange.grpc-web.injective.network" else fallback;
        },
        .nado => return if (test_network) "https://api.test.nado.xyz" else fallback,
        .ondo_perps => return if (test_network) "https://api.ondoperps-sandbox.xyz" else fallback,
        .pacifica => return if (test_network) "https://test-api.pacifica.fi" else fallback,
        .paragon, .aster => if (test_network) return error.InvalidNetwork,
        .trade_xyz => return if (test_network) "https://api.hyperliquid-testnet.xyz" else fallback,
        else => if (@import("perps_connectors.zig").supports(venue)) return @import("perps_connectors.zig").origin(venue, op, config, fallback),
    }
    return fallback;
}

pub fn isVenue(venue: Venue) bool {
    return switch (venue) {
        .injective, .nado, .ondo_perps, .pacifica, .paragon, .trade_xyz, .aster, .perpl, .risex, .decibel, .edgex, .grvt, .katana => true,
        else => false,
    };
}

pub fn isWrite(venue: Venue, operation: []const u8) bool {
    return isVenue(venue) and (eq(u8, operation, "place_order") or eq(u8, operation, "place_market_order") or eq(u8, operation, "cancel_order"));
}

pub fn epoch(io: std.Io, comptime unit: enum { ms, us }) !u64 {
    const time = std.Io.Timestamp.now(io, .real);
    const value = switch (unit) {
        .ms => time.toMilliseconds(),
        .us => time.toMicroseconds(),
    };
    return std.math.cast(u64, value) orelse error.InvalidTimestamp;
}

pub fn evmKey(auth: V) ![32]u8 {
    return evm.hex(32, try credential(get(auth, "key")));
}

pub fn evmAddress(key: [32]u8) ![20]u8 {
    const point = (try std.crypto.ecc.Secp256k1.basePoint.mul(key, .big)).toUncompressedSec1();
    return evm.hash(point[1..])[12..].*;
}

pub fn signature(a: A, key: [32]u8, hash: Word) ![]const u8 {
    const bytes = try @import("transaction.zig").recoverable(key, hash);
    return std.fmt.allocPrint(a, "0x{x}", .{&bytes});
}

pub fn typedHash(name: []const u8, version: []const u8, chain: u64, contract: Word, message: Word) Word {
    const domain_type = evm.hash("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
    const domain = evm.hash(&(domain_type ++ evm.hash(name) ++ evm.hash(version) ++ evm.encodeWord(chain) ++ contract));
    return evm.hash(&(.{ 0x19, 0x01 } ++ domain ++ message));
}

pub fn replyError(venue: Venue, result: V) bool {
    const success = get(result, "success");
    const status = get(result, "status");
    const code = get(result, "code");
    return switch (venue) {
        .nado => status == .string and eq(u8, status.string, "failure"),
        .ondo_perps, .pacifica => success == .bool and !success.bool,
        .aster, .injective => code == .number_string and code.number_string.len > 0 and code.number_string[0] == '-',
        .paragon, .trade_xyz => result == .string or (status == .string and eq(u8, status.string, "err")),
        .edgex => code == .string and !eq(u8, code.string, "SUCCESS"),
        .grvt => code != .null and get(result, "result") == .null,
        .risex => (code == .number_string and !eq(u8, code.number_string, "0")) or (code == .string and !eq(u8, code.string, "OK") and !eq(u8, code.string, "0")),
        .katana => get(result, "errorCode") != .null and get(result, "orderId") == .null,
        else => false,
    };
}
