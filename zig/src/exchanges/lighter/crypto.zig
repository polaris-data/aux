// ECgFp5 arithmetic and Poseidon2 over the Goldilocks field.
// Lighter Go signing vectors pin the generator, order, permutation and byte order.
const std = @import("std");
const gold: u64 = 0xffffffff00000001;
const Element = [5]u64;
const zero: Element = @splat(0);
const one: Element = .{ 1, 0, 0, 0, 0 };
fn add(x: u64, y: u64) u64 {
    return @intCast((@as(u128, x) + y) % gold);
}
fn sub(x: u64, y: u64) u64 {
    return @intCast((@as(u128, x) + gold - y) % gold);
}
fn mul(x: u64, y: u64) u64 {
    return @intCast((@as(u128, x) * y) % gold);
}
fn sbox(x: u64) u64 {
    const cube = mul(mul(x, x), x);
    return mul(mul(cube, cube), x);
}
fn mix(s: *[12]u64) void {
    for (0..3) |i| {
        const v = s[i * 4 ..][0..4];
        const a = v[0];
        const b = v[1];
        const c = v[2];
        const d = v[3];
        const ab = add(a, b);
        const cd = add(c, d);
        const sum = add(ab, cd);
        v.* = .{ add(add(sum, b), ab), add(add(sum, b), add(c, c)), add(add(sum, d), cd), add(add(sum, d), add(a, a)) };
    }
    for (0..4) |i| {
        const sum = add(add(s[i], s[i + 4]), s[i + 8]);
        for (0..3) |j| s[i + 4 * j] = add(s[i + 4 * j], sum);
    }
}
fn permute(s: *[12]u64) void {
    mix(s);
    for (external_rounds, 0..) |round, i| {
        if (i == 4) for (partial_rounds) |constant| {
            s[0] = sbox(add(s[0], constant));
            var sum: u64 = 0;
            for (s) |n| sum = add(sum, n);
            for (s, diagonal) |*n, d| n.* = add(mul(n.*, d), sum);
        };
        for (s, round) |*n, k| n.* = sbox(add(n.*, k));
        mix(s);
    }
}
pub fn hash(words: []const u64) Element {
    var state: [12]u64 = @splat(0);
    var rest = words;
    while (rest.len > 0) {
        const n = @min(rest.len, 8);
        @memcpy(state[0..n], rest[0..n]);
        permute(&state);
        rest = rest[n..];
    }
    return state[0..5].*;
}
pub fn bytes(f: Element) [40]u8 {
    var out: [40]u8 = undefined;
    for (f, 0..) |n, i| std.mem.writeInt(u64, out[i * 8 ..][0..8], n, .little);
    return out;
}
fn plus(x: Element, y: Element) Element {
    var z: Element = undefined;
    for (&z, x, y) |*n, a, b| n.* = add(a, b);
    return z;
}
fn minus(x: Element, y: Element) Element {
    var z: Element = undefined;
    for (&z, x, y) |*n, a, b| n.* = sub(a, b);
    return z;
}
fn times(x: Element, y: Element) Element {
    var z = zero;
    for (x, 0..) |a, i| for (y, 0..) |b, j| {
        const k = (i + j) % 5;
        const prod = mul(a, b);
        z[k] = add(z[k], if (i + j >= 5) mul(prod, 3) else prod);
    };
    return z;
}
fn sq(x: Element) Element {
    return times(x, x);
}
fn twice(x: Element) Element {
    return plus(x, x);
}
fn invert(x: Element) Element {
    const p: comptime_int = gold;
    const power: u320 = p * p * p * p * p - 2;
    var z = one;
    for (0..320) |i| {
        z = sq(z);
        if ((power >> @intCast(319 - i)) & 1 != 0) z = times(z, x);
    }
    return z;
}

const curve_b: Element = .{ 0, 263, 0, 0, 0 };
const Point = struct {
    x: Element = zero,
    z: Element = one,
    u: Element = zero,
    t: Element = one,
    fn double(p: Point) Point {
        const zt = times(p.z, p.t);
        const ztt = times(zt, p.t);
        const zu = times(zt, p.u);
        const w = minus(ztt, times(sq(p.u), twice(plus(p.x, p.z))));
        const z2 = sq(zu);
        const w2 = sq(w);
        return .{ .x = times(z2, twice(twice(curve_b))), .z = w2, .u = minus(sq(plus(w, zu)), plus(z2, w2)), .t = minus(twice(sq(ztt)), plus(twice(twice(z2)), w2)) };
    }
    fn sum(p: Point, q: Point) Point {
        const xx = times(p.x, q.x);
        const zz = times(p.z, q.z);
        const uu = times(p.u, q.u);
        const tt = times(p.t, q.t);
        const xz = minus(times(plus(p.x, p.z), plus(q.x, q.z)), plus(xx, zz));
        const ut = minus(times(plus(p.u, p.t), plus(q.u, q.t)), plus(uu, tt));
        const bzz = times(curve_b, zz);
        const g = plus(xx, bzz);
        const h = times(tt, g);
        const j = times(uu, twice(plus(times(xz, curve_b), g)));
        const k = times(plus(tt, twice(uu)), plus(xz, g));
        return .{ .x = times(minus(k, h), curve_b), .z = minus(h, j), .u = times(ut, minus(bzz, xx)), .t = plus(h, j) };
    }
    fn choose(mask: u64, left: Point, right: Point) Point {
        var out: Point = undefined;
        inline for (.{ "x", "z", "u", "t" }) |k| for (&@field(out, k), @field(left, k), @field(right, k)) |*r, l, h| {
            r.* = l ^ ((l ^ h) & mask);
        };
        return out;
    }
};
fn generatorMul(secret: [40]u8) Element {
    var base = Point{ .x = .{ 12883135586176881569, 4356519642755055268, 5248930565894896907, 2165973894480315022, 2448410071095648785 }, .u = one, .t = .{ 4, 0, 0, 0, 0 } };
    var point = Point{};
    for (secret) |byte| for (0..8) |i| {
        const mask: u64 = 0 -% @as(u64, (byte >> @intCast(i)) & 1);
        point = Point.choose(mask, point, point.sum(base));
        base = base.double();
    };
    return times(point.t, invert(point.u));
}
const order: u320 = 0x7ffffffd800000077ffffff1000000167fffffe6cfb80639e8885c39d724a09ce80fd996948bffe1;
const Field = std.crypto.ff.Modulus(320);
fn signWithNonce(message: Element, key: [40]u8, nonce: [40]u8) ![80]u8 {
    const e = bytes(hash(&(generatorMul(nonce) ++ message)));
    var eb: [40]u8 = undefined;
    // The challenge is public. Secret scalar arithmetic uses Zig's fixed-width field.
    std.mem.writeInt(u320, &eb, std.mem.readInt(u320, &e, .little) % order, .little);
    var mod_bytes: [40]u8 = undefined;
    std.mem.writeInt(u320, &mod_bytes, order, .little);
    const modulus = try Field.fromBytes(&mod_bytes, .little);
    const challenge = try Field.Fe.fromBytes(modulus, &eb, .little);
    var scalar = try Field.Fe.fromBytes(modulus, &key, .little);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&scalar));
    var random = try Field.Fe.fromBytes(modulus, &nonce, .little);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&random));
    var result: [80]u8 = undefined;
    try modulus.sub(random, modulus.mul(challenge, scalar)).toBytes(result[0..40], .little);
    result[40..].* = eb;
    return result;
}
pub fn sign(io: std.Io, message: Element, key: [40]u8) ![80]u8 {
    const k = std.mem.readInt(u320, &key, .little);
    if (k == 0 or k >= order) return error.InvalidKey;
    var nonce: [40]u8 = undefined;
    defer std.crypto.secureZero(u8, &nonce);
    while (true) {
        try io.randomSecure(&nonce);
        const n = std.mem.readInt(u320, &nonce, .little);
        if (n != 0 and n < order) break;
    }
    return signWithNonce(message, key, nonce);
}

// Poseidon2 round parameters from ZkLighter Protocol (Apache-2.0).
const external_rounds = [8][12]u64{ .{ 15492826721047263190, 11728330187201910315, 8836021247773420868, 16777404051263952451, 5510875212538051896, 6173089941271892285, 2927757366422211339, 10340958981325008808, 8541987352684552425, 9739599543776434497, 15073950188101532019, 12084856431752384512 }, .{ 4584713381960671270, 8807052963476652830, 54136601502601741, 4872702333905478703, 5551030319979516287, 12889366755535460989, 16329242193178844328, 412018088475211848, 10505784623379650541, 9758812378619434837, 7421979329386275117, 375240370024755551 }, .{ 3331431125640721931, 15684937309956309981, 578521833432107983, 14379242000670861838, 17922409828154900976, 8153494278429192257, 15904673920630731971, 11217863998460634216, 3301540195510742136, 9937973023749922003, 3059102938155026419, 1895288289490976132 }, .{ 5580912693628927540, 10064804080494788323, 9582481583369602410, 10186259561546797986, 247426333829703916, 13193193905461376067, 6386232593701758044, 17954717245501896472, 1531720443376282699, 2455761864255501970, 11234429217864304495, 4746959618548874102 }, .{ 13571697342473846203, 17477857865056504753, 15963032953523553760, 16033593225279635898, 14252634232868282405, 8219748254835277737, 7459165569491914711, 15855939513193752003, 16788866461340278896, 7102224659693946577, 3024718005636976471, 13695468978618890430 }, .{ 8214202050877825436, 2670727992739346204, 16259532062589659211, 11869922396257088411, 3179482916972760137, 13525476046633427808, 3217337278042947412, 14494689598654046340, 15837379330312175383, 8029037639801151344, 2153456285263517937, 8301106462311849241 }, .{ 13294194396455217955, 17394768489610594315, 12847609130464867455, 14015739446356528640, 5879251655839607853, 9747000124977436185, 8950393546890284269, 10765765936405694368, 14695323910334139959, 16366254691123000864, 15292774414889043182, 10910394433429313384 }, .{ 17253424460214596184, 3442854447664030446, 3005570425335613727, 10859158614900201063, 9763230642109343539, 6647722546511515039, 909012944955815706, 18101204076790399111, 11588128829349125809, 15863878496612806566, 5201119062417750399, 176665553780565743 } };
const partial_rounds = [22]u64{ 11921381764981422944, 10318423381711320787, 8291411502347000766, 229948027109387563, 9152521390190983261, 7129306032690285515, 15395989607365232011, 8641397269074305925, 17256848792241043600, 6046475228902245682, 12041608676381094092, 12785542378683951657, 14546032085337914034, 3304199118235116851, 16499627707072547655, 10386478025625759321, 13475579315436919170, 16042710511297532028, 1411266850385657080, 9024840976168649958, 14047056970978379368, 838728605080212101 };
const diagonal = [12]u64{ 14102670999874605824, 15585654191999307702, 940187017142450255, 8747386241522630711, 6750641561540124747, 7440998025584530007, 6136358134615751536, 12413576830284969611, 11675438539028694709, 17580553691069642926, 892707462476851331, 15167485180850043744 };

test "official Lighter create-order Schnorr vector" {
    const decode = @import("../../core/evm.zig").hex;
    const raw = try decode(40, "7f73617b0f1a02623f8bcb8b00f8f2fe99ac0dcd8330ddf1f6c7472c3fed142b6a2b13d013d54f20");
    var digest: Element = undefined;
    for (&digest, 0..) |*v, i| v.* = std.mem.readInt(u64, raw[i * 8 ..][0..8], .little);
    const actual = try signWithNonce(digest, try decode(40, "01000000000000000000000000000000000000000000000000000000000000000000000000000000"), try decode(40, "07000000000000000000000000000000000000000000000000000000000000000000000000000000"));
    try std.testing.expectEqual(try decode(80, "6f3a319b5c0e39e391677d59bef760876d9129e25d7de365b1defe62aa7b6253029d90f36c117d4579c55af939cbd6040b39a77d7b642761cc748eed88821c1a6521019d46849d2c05636f8c90ee823a"), actual);
}
