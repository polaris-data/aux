// Starknet field and curve operations shared by Paradex's two SNIP-12 revisions.
const std = @import("std");
const evm = @import("evm.zig");
const w = @import("wire.zig");
pub const prime: u256 = (@as(u256, 1) << 251) + (@as(u256, 17) << 192) + 1;
pub const order: u256 = 0x0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f;
const M = std.crypto.ff.Modulus(256);
fn Field(comptime p: u256) type {
    return struct {
        n: M.Fe,
        const Self = @This();
        const m = blk: {
            @setEvalBranchQuota(100000);
            break :blk M.fromPrimitive(u256, p) catch unreachable;
        };
        fn init(x: u256) Self {
            var n = M.Fe.fromBytes(m, &evm.encodeWord(x), .big) catch unreachable;
            m.toMontgomery(&n) catch unreachable;
            return .{ .n = n };
        }
        fn value(self: Self) u256 {
            var n = self.n;
            m.fromMontgomery(&n) catch unreachable;
            var bytes: [32]u8 = undefined;
            n.toBytes(&bytes, .big) catch unreachable;
            return std.mem.readInt(u256, &bytes, .big);
        }
        fn add(a: Self, b: Self) Self {
            return .{ .n = m.add(a.n, b.n) };
        }
        fn sub(a: Self, b: Self) Self {
            return .{ .n = m.sub(a.n, b.n) };
        }
        fn mul(a: Self, b: Self) Self {
            return .{ .n = m.mul(a.n, b.n) };
        }
        fn cube(a: Self) Self {
            return a.mul(a).mul(a);
        }
        fn inv(a: Self) Self {
            return .{ .n = m.powWithEncodedPublicExponent(a.n, &evm.encodeWord(p - 2), .big) catch unreachable };
        }
        fn select(a: Self, b: Self, bit: u1) Self {
            var r = a;
            for (&r.n.v.limbs_buffer, b.n.v.limbs_buffer) |*x, y| {
                const mask = @as(@TypeOf(y), 0) -% @as(@TypeOf(y), bit);
                x.* ^= (x.* ^ y) & mask;
            }
            return r;
        }
    };
}
const F = Field(prime);
const S = Field(order);
const Point = struct {
    x: F,
    y: F,
    z: F,
    fn affine(x: u256, y: u256) Point {
        return .{ .x = F.init(x), .y = F.init(y), .z = F.init(1) };
    }
    fn xValue(p: Point) u256 {
        return p.x.mul(p.z.inv()).value();
    }
    // Complete homogeneous addition, Renes–Costello–Batina algorithm 1, with a=1.
    fn add(p: Point, q: Point) Point {
        const b3 = comptime F.init((3 * @as(comptime_int, 0x06f21413efbe40de150e596d72f7a8c5609ad26c15c915c1f4cdfcb99cee9e89)) % prime);
        const xx = p.x.mul(q.x);
        const yy = p.y.mul(q.y);
        const zz = p.z.mul(q.z);
        const xy = p.x.add(p.y).mul(q.x.add(q.y)).sub(xx).sub(yy);
        const xz = p.x.add(p.z).mul(q.x.add(q.z)).sub(xx).sub(zz);
        const yz = p.y.add(p.z).mul(q.y.add(q.z)).sub(yy).sub(zz);
        const h = xz.add(b3.mul(zz));
        const minus = yy.sub(h);
        const plus = yy.add(h);
        const t = xx.add(xx).add(xx).add(zz);
        const u = b3.mul(xz).add(xx).sub(zz);
        return .{ .x = xy.mul(minus).sub(yz.mul(u)), .y = minus.mul(plus).add(t.mul(u)), .z = yz.mul(plus).add(xy.mul(t)) };
    }
};
pub fn publicKey(k: u256) !u256 {
    if (k == 0 or k >= order) return error.InvalidSecret;
    const base = Point.affine(0x01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca, 0x005668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f);
    var r = Point{ .x = F.init(0), .y = F.init(1), .z = F.init(0) };
    var i: usize = 252;
    while (i > 0) {
        i -= 1;
        const doubled = r.add(r);
        const plus = doubled.add(base);
        const bit: u1 = @truncate(k >> @intCast(i));
        r = .{ .x = doubled.x.select(plus.x, bit), .y = doubled.y.select(plus.y, bit), .z = doubled.z.select(plus.z, bit) };
    }
    return r.xValue();
}
const table = @embedFile("stark-pedersen.bin");
pub fn pedersen(a: u256, b: u256) u256 {
    var sum = Point.affine(0x049ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804, 0x03ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a);
    for ([_]u256{ a, b }, 0..) |input, half| {
        var bits = input;
        var i: usize = half * 252;
        while (bits != 0) : (i += 1) {
            if (bits & 1 != 0) {
                const buf = table[i * 64 ..][0..64];
                sum = sum.add(Point.affine(std.mem.readInt(u256, buf[0..32], .big), std.mem.readInt(u256, buf[32..64], .big)));
            }
            bits >>= 1;
        }
    }
    return sum.xValue();
}
pub fn legacy(values: []const u256) u256 {
    var acc: u256 = 0;
    for (values) |v| acc = pedersen(acc, v);
    return pedersen(acc, values.len);
}
const keys = @embedFile("stark-poseidon.bin");
fn permute(s: *[3]F) void {
    for (0..91) |round| {
        for (s, 0..) |*x, j| {
            const i = (3 * round + j) * 32;
            x.* = x.add(F.init(std.mem.readInt(u256, keys[i..][0..32], .big)));
            if (round < 4 or round >= 87 or j == 2) x.* = x.cube();
        }
        const sum = s[0].add(s[1]).add(s[2]);
        s.* = .{ sum.add(s[0]).add(s[0]), sum.sub(s[1]).sub(s[1]), sum.sub(s[2]).sub(s[2]).sub(s[2]) };
    }
}
pub fn poseidon(values: []const u256) u256 {
    var s = [_]F{ F.init(0), F.init(0), F.init(0) };
    for (values, 0..) |v, i| {
        s[i % 2] = s[i % 2].add(F.init(v));
        if (i % 2 == 1) permute(&s);
    }
    s[values.len % 2] = s[values.len % 2].add(F.init(1));
    permute(&s);
    return s[0].value();
}
pub fn branch(a: u256, b: u256) u256 {
    var s = [_]F{ F.init(@min(a, b)), F.init(@max(a, b)), F.init(2) };
    permute(&s);
    return s[0].value();
}
pub fn selector(name: []const u8) u256 {
    return std.mem.readInt(u256, &evm.hash(name), .big) & ((@as(u256, 1) << 250) - 1);
}
pub fn felt(s: []const u8) !u256 {
    const hex = std.mem.startsWith(u8, s, "0x");
    const digits = if (hex) s[2..] else s;
    if (digits.len == 0) return error.InvalidFelt;
    for (digits) |c| if (!(if (hex) std.ascii.isHex(c) else std.ascii.isDigit(c))) return error.InvalidFelt;
    const n = try std.fmt.parseInt(u256, digits, if (hex) 16 else 10);
    if (n >= prime) return error.InvalidFelt;
    return n;
}
pub fn short(s: []const u8) !u256 {
    if (s.len > 31) return error.InvalidFelt;
    var n: u256 = 0;
    for (s) |c| {
        if (c > 127 or c == 0) return error.InvalidFelt;
        n = (n << 8) | c;
    }
    return n;
}
fn nonce(key: u256, digest: u256, entropy: []const u8) u256 {
    const H = std.crypto.auth.hmac.sha2.HmacSha256;
    var k: [32]u8 = @splat(0);
    defer std.crypto.secureZero(u8, &k);
    var v: [32]u8 = @splat(1);
    defer std.crypto.secureZero(u8, &v);
    var secret = evm.encodeWord(key);
    defer std.crypto.secureZero(u8, &secret);
    for ([_]u8{ 0, 1 }) |c| {
        var mac = H.init(&k);
        mac.update(&v);
        mac.update(&.{c});
        mac.update(&secret);
        mac.update(&evm.encodeWord(digest));
        mac.update(entropy);
        mac.final(&k);
        H.create(&v, &v, &k);
    }
    while (true) {
        H.create(&v, &v, &k);
        const n = std.mem.readInt(u256, &v, .big) >> 4;
        if (n > 0 and n < order) return n;
        var mac = H.init(&k);
        mac.update(&v);
        mac.update(&.{0});
        mac.final(&k);
        H.create(&v, &v, &k);
    }
}
pub fn sign(key: u256, digest: u256) ![2]u256 {
    return signWithEntropy(key, digest, &.{32}); // Paradex Stark signatures use seed 32.
}

pub fn signWithEntropy(key: u256, digest: u256, entropy: []const u8) ![2]u256 {
    if (key == 0 or key >= order or digest >= (@as(u256, 1) << 251)) return error.InvalidSignature;
    const k = nonce(key, digest, entropy);
    const r = try publicKey(k);
    if (r == 0 or r >= (@as(u256, 1) << 251)) return error.InvalidSignature;
    const s = S.init(r).mul(S.init(key)).add(S.init(digest)).mul(S.init(k).inv()).value();
    if (s == 0 or s >= (@as(u256, 1) << 251)) return error.InvalidSignature;
    return .{ r, s };
}

test "Paradex Stark ECDSA reference vector" {
    const sig = try sign(1, 0x3c8f39166329a6766aaa5eb48b1dba6bedb76ac995508284d947e3c30b1f84);
    try std.testing.expectEqual(@as(u256, 1154183141323667323280265752925798550773463557984282764477476921993358429864), sig[0]);
    try std.testing.expectEqual(@as(u256, 727620137390441845403436438575639177860900042916183862179820125017775962094), sig[1]);
}

test "Extended Stark ECDSA without additional entropy" {
    const sig = try signWithEntropy(1, 0x2ea4df899ff6f61f06de948fb534ebfa6c43472cb02cbe13a67dcd52c1e8288, &.{});
    try std.testing.expectEqual(@as(u256, 0x30b28b1a8ba5f51183229537dd1460c53b2fcba5c55af34b0d2859d3e6db2de), sig[0]);
    try std.testing.expectEqual(@as(u256, 0x233c33780ad1e0e21d96f6a267248cf67318e804b71535c54b507b277fba5f7), sig[1]);
}
