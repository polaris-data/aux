// Local signer primitives and public-key verification for language bindings.
const std = @import("std");
const w = @import("wire.zig");
const evm = @import("evm.zig");
const tx = @import("transaction.zig");
const sol = @import("solana.zig");
const stark = @import("stark.zig");
const Op = enum { ethereum_address, sign_digest, verify_digest, personal_hash, ed25519_public, ed25519_sign, ed25519_verify, solana_public, solana_sign, solana_verify, solana_validate, stark_public, stark_sign };

fn text(v: w.V, name: []const u8) ![]const u8 {
    return w.string(try w.field(v, name));
}

pub fn execute(a: w.A, v: w.V) !w.V {
    const op = std.meta.stringToEnum(Op, try text(v, "operation")) orelse return error.InvalidOperation;
    switch (op) {
        .personal_hash => {
            const message = try evm.fromHex(a, try text(v, "message"));
            const prefixed = try std.fmt.allocPrint(a, "\x19Ethereum Signed Message:\n{d}{s}", .{ message.len, message });
            return w.text(try std.fmt.allocPrint(a, "0x{x}", .{&evm.hash(prefixed)}));
        },
        .verify_digest => {
            const sig = try evm.hex(65, try text(v, "signature"));
            const parity: u8 = switch (sig[64]) {
                0, 1 => sig[64],
                27, 28 => sig[64] - 27,
                else => return error.InvalidSignature,
            };
            const actual = try tx.recoverAddress(try evm.hex(32, try text(v, "digest")), std.mem.readInt(u256, sig[0..32], .big), std.mem.readInt(u256, sig[32..64], .big), parity);
            if (!std.mem.eql(u8, &actual, &try evm.hex(20, try text(v, "address")))) return error.InvalidSignature;
            return w.text(try std.fmt.allocPrint(a, "0x{x}{x:0>2}", .{ sig[0..64], @as(u8, 27) + parity }));
        },
        .solana_verify => {
            const ed = std.crypto.sign.Ed25519;
            const public = try ed.PublicKey.fromBytes(try sol.publicKey(try text(v, "public_key")));
            const sig = try sol.decode58(64, try text(v, "signature"));
            try ed.Signature.fromBytes(sig).verify(try evm.fromHex(a, try text(v, "message")), public);
            return w.text(try sol.base58(a, 64, sig));
        },
        .solana_validate => return w.text(try sol.base58(a, 32, try sol.publicKey(try text(v, "public_key")))),
        .ed25519_verify => {
            const ed = std.crypto.sign.Ed25519;
            const key = try ed.PublicKey.fromBytes(try evm.hex(32, try text(v, "public_key")));
            const sig = ed.Signature.fromBytes(try evm.hex(64, try text(v, "signature")));
            try sig.verify(try evm.fromHex(a, try text(v, "message")), key);
            return w.text(try std.fmt.allocPrint(a, "{x}", .{&sig.toBytes()}));
        },
        .stark_sign, .stark_public => {
            var secret = try stark.felt(try text(v, "key"));
            defer std.crypto.secureZero(u8, std.mem.asBytes(&secret));
            if (op == .stark_public) return w.text(try std.fmt.allocPrint(a, "0x{x}", .{try stark.publicKey(secret)}));
            const result = try stark.sign(secret, try stark.felt(try text(v, "digest")));
            var values: w.V = .{ .array = std.array_list.Managed(w.V).init(a) };
            for (result) |n| try values.array.append(w.text(try std.fmt.allocPrint(a, "{d}", .{n})));
            return values;
        },
        .ethereum_address, .sign_digest, .ed25519_public, .ed25519_sign, .solana_public, .solana_sign => {
            var secret = try evm.hex(32, try text(v, "key"));
            defer std.crypto.secureZero(u8, &secret);
            if (op == .sign_digest) return w.text(try std.fmt.allocPrint(a, "0x{x}", .{&try tx.recoverable(secret, try evm.hex(32, try text(v, "digest")))}));
            if (op == .ethereum_address) {
                const point = (try std.crypto.ecc.Secp256k1.basePoint.mul(secret, .big)).toUncompressedSec1();
                return w.text(try std.fmt.allocPrint(a, "0x{x}", .{evm.hash(point[1..])[12..]}));
            }
            var pair = try std.crypto.sign.Ed25519.KeyPair.generateDeterministic(secret);
            defer std.crypto.secureZero(u8, std.mem.asBytes(&pair));
            if (op == .solana_public) return w.text(try sol.base58(a, 32, pair.public_key.toBytes()));
            if (op == .ed25519_public) return w.text(try std.fmt.allocPrint(a, "{x}", .{&pair.public_key.toBytes()}));
            const signature = try pair.sign(try evm.fromHex(a, try text(v, "message")), null);
            if (op == .solana_sign) return w.text(try sol.base58(a, 64, signature.toBytes()));
            return w.text(try std.fmt.allocPrint(a, "{x}", .{&signature.toBytes()}));
        },
    }
}
