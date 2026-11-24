const std = @import("std");
const w = @import("wire.zig");
const V = w.V;
const A = w.A;
const Sha256 = std.crypto.hash.sha2.Sha256;
const Ed25519 = std.crypto.sign.Ed25519;

fn text(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn timestamp(v: V, k: []const u8) !u64 {
    const value = try w.field(v, k);
    const s = if (value == .number_string) value.number_string else try w.string(value);
    if (s.len == 0) return error.InvalidTimestamp;
    for (s) |b| if (!std.ascii.isDigit(b)) return error.InvalidTimestamp;
    return std.fmt.parseInt(u64, s, 10);
}
pub fn qfex(a: A, v: V) !V {
    const nonce = try text(v, "nonce");
    return w.text(try qfexMac(a, try text(v, "secret"), nonce, try timestamp(v, "timestamp")));
}
fn qfexMac(a: A, secret: []const u8, nonce: []const u8, seconds: u64) ![]const u8 {
    if (nonce.len == 0 or nonce.len > 100 or secret.len == 0) return error.InvalidCredential;
    for (nonce) |b| if (!std.ascii.isHex(b)) return error.InvalidNonce;
    const msg = try std.fmt.allocPrint(a, "{s}:{d}", .{ nonce, seconds });
    var mac: [32]u8 = undefined;
    std.crypto.auth.hmac.sha2.HmacSha256.create(&mac, msg, secret);
    return std.fmt.allocPrint(a, "{x}", .{&mac});
}
pub fn qfexHeaders(a: A, headers: *V, io: std.Io, key: []const u8, secret: []const u8) !void {
    if (key.len == 0 or secret.len == 0) return error.InvalidCredential;
    for (key) |b| if (!std.ascii.isAlphanumeric(b) and b != '_' and b != '-') return error.InvalidCredential;
    var random: [16]u8 = undefined;
    try io.randomSecure(&random);
    const nonce = try std.fmt.allocPrint(a, "{x}", .{&random});
    const seconds = std.Io.Timestamp.now(io, .real).toSeconds();
    if (seconds < 0) return error.InvalidTimestamp;
    try headers.object.put(a, "x-qfex-public-key", w.text(key));
    try headers.object.put(a, "x-qfex-nonce", w.text(nonce));
    try headers.object.put(a, "x-qfex-timestamp", w.text(try std.fmt.allocPrint(a, "{d}", .{seconds})));
    try headers.object.put(a, "x-qfex-hmac-signature", w.text(try qfexMac(a, secret, nonce, @intCast(seconds))));
}
pub fn validateDatabentoKey(key: []const u8) !void {
    if (key.len != 32) return error.InvalidCredential;
    for (key) |b| if (b < 0x21 or b > 0x7e or b == '|') return error.InvalidCredential;
}
pub fn databento(a: A, v: V) !V {
    const key = try text(v, "key");
    try validateDatabentoKey(key);
    const challenge = try text(v, "challenge");
    if (challenge.len == 0 or std.mem.findAny(u8, challenge, "|\n\r\x00") != null) return error.InvalidChallenge;
    var hash = Sha256.init(.{});
    hash.update(challenge);
    hash.update("|");
    hash.update(key);
    const digest = hash.finalResult();
    return w.text(try std.fmt.allocPrint(a, "{x}-{s}", .{ &digest, key[27..] }));
}
fn keypair(input: V) !Ed25519.KeyPair {
    return keypairHex(try text(input, "seed"));
}
fn keypairHex(hex: []const u8) !Ed25519.KeyPair {
    const bytes = if (std.mem.startsWith(u8, hex, "0x")) hex[2..] else hex;
    var seed: [32]u8 = undefined;
    defer std.crypto.secureZero(u8, &seed);
    if (bytes.len != 64) return error.InvalidSeed;
    _ = try std.fmt.hexToBytes(&seed, bytes);
    return Ed25519.KeyPair.generateDeterministic(seed);
}
pub fn arcus(a: A, input: V) !V {
    var kp = try keypair(input);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&kp));
    const sig = (try kp.sign(try text(input, "payload"), null)).toBytes();
    var out: V = .{ .object = .empty };
    try out.object.put(a, "signature", w.text(try std.fmt.allocPrint(a, "{x}", .{&sig})));
    try out.object.put(a, "public_key", w.text(try std.fmt.allocPrint(a, "{x}", .{&kp.public_key.toBytes()})));
    return out;
}
fn b64(a: A, bytes: []const u8) ![]const u8 {
    const enc = std.base64.standard.Encoder;
    return enc.encode(try a.alloc(u8, enc.calcSize(bytes.len)), bytes);
}
pub fn monaco(a: A, input: V) !V {
    const method = try text(input, "method");
    const allowed = [_][]const u8{ "GET", "POST", "PUT", "PATCH", "DELETE" };
    for (allowed) |m| {
        if (std.mem.eql(u8, method, m)) break;
    } else return error.InvalidMethod;
    const target = try text(input, "target");
    if (!std.mem.startsWith(u8, target, "/api/v1/") or std.mem.findScalar(u8, target, '#') != null) return error.InvalidPath;
    for (target) |b| if (b <= 32 or b == 127) return error.InvalidPath;
    return monacoSignature(a, method, target, try text(input, "body"), try text(input, "seed"), try timestamp(input, "created"), false);
}

pub fn monacoSession(a: A, session: V, origin: []const u8, method: []const u8, target: []const u8, body: []const u8, io: std.Io) !V {
    if (!std.mem.eql(u8, std.mem.trimEnd(u8, try text(session, "origin"), "/"), std.mem.trimEnd(u8, origin, "/"))) return error.AuthenticationRequired;
    try @import("monaco_session.zig").check(session, io);
    const time = std.Io.Timestamp.now(io, .real).toMilliseconds();
    if (time < 0 or @divTrunc(time, 1000) >= try timestamp(session, "expires_at")) return error.AuthenticationRequired;
    const mode = if (session.object.get("signing")) |v| try w.string(v) else "legacy";
    const legacy = std.mem.eql(u8, mode, "legacy");
    if (!legacy and !std.mem.eql(u8, mode, "rfc9421")) return error.InvalidSigningMode;
    return monacoSignature(a, method, target, body, try text(session, "seed"), @intCast(if (legacy) time else @divTrunc(time, 1000)), legacy);
}
fn monacoSignature(a: A, method: []const u8, target: []const u8, body: []const u8, seed: []const u8, created: u64, legacy: bool) !V {
    var kp = try keypairHex(seed);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&kp));
    const public = kp.public_key.toBytes();
    var digest: [32]u8 = undefined;
    Sha256.hash(body, &digest, .{});
    if (legacy) {
        const data = try std.fmt.allocPrint(a, "{s}\n{s}\n{d}\n{x}", .{ method, target, created, &digest });
        const signature = (try kp.sign(data, null)).toBytes();
        var out: V = .{ .object = .empty };
        try out.object.put(a, "X-Monaco-PublicKey", w.text(try std.fmt.allocPrint(a, "{x}", .{&public})));
        try out.object.put(a, "X-Monaco-Timestamp", w.text(try std.fmt.allocPrint(a, "{d}", .{created})));
        try out.object.put(a, "X-Monaco-Signature", w.text(try std.fmt.allocPrint(a, "{x}", .{&signature})));
        return out;
    }
    const params = try std.fmt.allocPrint(a, "(\"@method\" \"@path\" \"@query\" \"content-digest\");created={d};keyid=\"{x}\";alg=\"ed25519\"", .{ created, &public });
    const content = try std.fmt.allocPrint(a, "sha-256=:{s}:", .{try b64(a, &digest)});
    const i = std.mem.findScalar(u8, target, '?') orelse target.len;
    const msg = try std.fmt.allocPrint(a, "\"@method\": {s}\n\"@path\": {s}\n\"@query\": {s}\n\"content-digest\": {s}\n\"@signature-params\": {s}", .{ method, target[0..i], if (i == target.len) "?" else target[i..], content, params });
    const sig = (try kp.sign(msg, null)).toBytes();
    var out: V = .{ .object = .empty };
    try out.object.put(a, "Content-Digest", w.text(content));
    try out.object.put(a, "Signature-Input", w.text(try std.fmt.allocPrint(a, "monaco={s}", .{params})));
    try out.object.put(a, "Signature", w.text(try std.fmt.allocPrint(a, "monaco=:{s}:", .{try b64(a, &sig)})));
    return out;
}

pub fn chainlinkHeaders(a: A, headers: *V, key: []const u8, secret: []const u8, method: []const u8, path: []const u8, timestamp_ms: u64) !void {
    const digest = Sha256.hash;
    var empty: [32]u8 = undefined;
    digest("", &empty, .{});
    const message = try std.fmt.allocPrint(a, "{s} {s} {x} {s} {d}", .{ method, path, &empty, key, timestamp_ms });
    var mac: [32]u8 = undefined;
    std.crypto.auth.hmac.sha2.HmacSha256.create(&mac, message, secret);
    try headers.object.put(a, "Authorization", w.text(key));
    try headers.object.put(a, "X-Authorization-Timestamp", w.text(try std.fmt.allocPrint(a, "{d}", .{timestamp_ms})));
    try headers.object.put(a, "X-Authorization-Signature-SHA256", w.text(try std.fmt.allocPrint(a, "{x}", .{&mac})));
}

pub fn kalshiMessage(a: A, key: []const u8, method: []const u8, target: []const u8, ms: u64) ![]const u8 {
    if (key.len == 0) return error.InvalidCredential;
    for (key) |b| if (!std.ascii.isAlphanumeric(b) and b != '-' and b != '_') return error.InvalidCredential;
    for ([_][]const u8{ "GET", "POST", "PUT", "PATCH", "DELETE" }) |m| {
        if (std.mem.eql(u8, m, method)) break;
    } else return error.InvalidMethod;
    if (target.len == 0 or target[0] != '/' or std.mem.findAny(u8, target, "#\r\n") != null) return error.InvalidPath;
    const path = target[0 .. std.mem.findScalar(u8, target, '?') orelse target.len];
    return std.fmt.allocPrint(a, "{d}{s}{s}", .{ ms, method, path });
}
pub fn kalshiHeaders(a: A, key: []const u8, seed: []const u8, method: []const u8, target: []const u8, ms: u64) !V {
    const message = try kalshiMessage(a, key, method, target, ms);
    var kp = try keypairHex(seed);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&kp));
    const signature = (try kp.sign(message, null)).toBytes();
    return kalshiSignature(a, key, ms, &signature);
}
pub fn kalshiSignature(a: A, key: []const u8, ms: u64, signature: []const u8) !V {
    if (signature.len != 64 and (signature.len < 256 or signature.len > 1024)) return error.InvalidSignature;
    const time = try std.fmt.allocPrint(a, "{d}", .{ms});
    var out: V = .{ .object = .empty };
    try out.object.put(a, "KALSHI-ACCESS-KEY", w.text(key));
    try out.object.put(a, "KALSHI-ACCESS-TIMESTAMP", w.text(time));
    try out.object.put(a, "KALSHI-ACCESS-SIGNATURE", w.text(try b64(a, signature)));
    return out;
}

pub fn kalshi(a: A, v: V) !V {
    return kalshiHeaders(a, try text(v, "key"), try text(v, "seed"), try text(v, "method"), try text(v, "path"), try timestamp(v, "timestamp"));
}

pub fn monacoSocket(a: A, session: V, io: std.Io) ![]const u8 {
    var pair = try keypairHex(try text(session, "seed"));
    defer std.crypto.secureZero(u8, std.mem.asBytes(&pair));
    const public = try std.fmt.allocPrint(a, "{x}", .{&pair.public_key.toBytes()});
    const ts = std.Io.Timestamp.now(io, .real).toMilliseconds();
    const message = try std.fmt.allocPrint(a, "WS-AUTH\n{s}\n{d}", .{ public, ts });
    const sig = try pair.sign(message, null);
    return std.json.Stringify.valueAlloc(a, .{ .type = "Authenticate", .public_key = public, .timestamp = ts, .signature = try std.fmt.allocPrint(a, "{x}", .{&sig.toBytes()}) }, .{});
}
