const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");

pub fn digest(testnet: bool, maker: [32]u8, buy: bool, price: u256, amount: u256, salt: u256, instrument: u256, stamp: u256) [32]u8 {
    const order = evm.hash(&(evm.hash("Order(address maker,bool isBuy,uint256 limitPrice,uint256 amount,uint256 salt,uint256 instrument,uint256 timestamp)") ++
        maker ++ evm.encodeWord(@intFromBool(buy)) ++ evm.encodeWord(price) ++ evm.encodeWord(amount) ++ evm.encodeWord(salt) ++
        evm.encodeWord(instrument) ++ evm.encodeWord(stamp)));
    const domain = evm.hash(&(evm.hash("EIP712Domain(string name,string version,uint256 chainId)") ++
        evm.hash(if (testnet) "Aevo Testnet" else "Aevo Mainnet") ++ evm.hash("1") ++ evm.encodeWord(if (testnet) 11155111 else 1)));
    return evm.hash(&(.{ 0x19, 0x01 } ++ domain ++ order));
}

pub fn sign(a: w.A, input: w.V, source: w.V, io: std.Io) !w.V {
    var body = source;
    const auth = p.get(input, "auth");
    const wallet = try p.credential(p.get(auth, "wallet"));
    const maker = try evm.address(wallet);
    const supplied = p.get(body, "maker");
    if (supplied != .null and !std.ascii.eqlIgnoreCase(try w.string(supplied), wallet)) return error.InvalidCredential;
    const direction = try w.field(body, "is_buy");
    if (direction != .bool) return error.InvalidParameter;
    const price = try w.unsigned(u256, try w.field(body, "limit_price"));
    const amount = try w.unsigned(u256, try w.field(body, "amount"));
    if (amount == 0) return error.InvalidQuantity;
    const instrument = try w.unsigned(u256, try w.field(body, "instrument"));
    const timestamp = try p.epoch(io, .ms) / 1000;
    var random: [8]u8 = undefined;
    try std.Io.randomSecure(io, &random);
    const salt = std.mem.readInt(u64, &random, .big);
    var key = try p.evmKey(auth);
    defer std.crypto.secureZero(u8, &key);
    const msg = digest(try p.testnet(input), maker, direction.bool, price, amount, salt, instrument, timestamp);
    try p.put(&body, a, "maker", w.text(wallet));
    try p.put(&body, a, "salt", w.text(try std.fmt.allocPrint(a, "{d}", .{salt})));
    try p.put(&body, a, "timestamp", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{timestamp}) });
    try p.put(&body, a, "signature", w.text(try p.signature(a, key, msg)));
    return body;
}

test "Aevo mainnet EIP-712 order digest" {
    const maker = try evm.address("0x1111111111111111111111111111111111111111");
    const expected = try evm.hex(32, "91d79adba90fb4851322ac58cff14f81ae4d3725b036cd3968d3a5301b9b9f23");
    try std.testing.expectEqual(expected, digest(false, maker, true, 100000000, 10000, 12345, 1, 1700000000));
}
