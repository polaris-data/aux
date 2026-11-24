// The wallet remains with the caller. Only a validated, unexpired challenge
// binding the requested session public key reaches its signing callback.
const std = @import("std");
const w = @import("wire.zig");
const evm = @import("evm.zig");
const A = w.A;
const V = w.V;
const eq = std.mem.eql;
fn get(v: V, k: []const u8) V {
    return if (v == .object) v.object.get(k) orelse .null else .null;
}
fn text(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn address(a: A, v: V) ![]const u8 {
    const raw = try w.string(v);
    if (!std.mem.startsWith(u8, raw, "0x")) return error.InvalidWallet;
    return std.fmt.allocPrint(a, "0x{x}", .{try evm.hex(20, raw)});
}
pub fn expiry(v: V, io: std.Io) !u64 {
    const unix = std.Io.Timestamp.now(io, .real).toSeconds();
    if (get(v, "expiresAt") != .number_string) return error.AuthenticationRequired;
    const n = w.unsigned(u63, get(v, "expiresAt")) catch return error.AuthenticationRequired;
    if (unix < 0 or n <= unix) return error.AuthenticationRequired;
    return n;
}
pub fn step(a: A, input: V) !V {
    var runtime: std.Io.Threaded = .init_single_threaded;
    const io = runtime.io();
    const phase = try text(input, "step");
    const opts = try w.field(input, "options");
    const client = try text(opts, "client_id");
    if (std.mem.trim(u8, client, " \t\r\n").len == 0) return error.InvalidClientId;
    const wallet = try address(a, try w.field(opts, "address"));
    var key = try evm.hex(32, try text(opts, "seed"));
    defer std.crypto.secureZero(u8, &key);
    var pair = try std.crypto.sign.Ed25519.KeyPair.generateDeterministic(key);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&pair));
    const pubkey = try std.fmt.allocPrint(a, "{x}", .{&pair.public_key.toBytes()});
    const origin = try text(input, "origin");
    const uri = try std.Uri.parse(origin);
    if (uri.host == null or uri.user != null or uri.password != null or uri.query != null or uri.fragment != null or !(eq(u8, uri.scheme, "https") or eq(u8, uri.scheme, "http"))) return error.InvalidOrigin;
    const host = try uri.host.?.toRawMaybeAlloc(a);
    const default_chain: ?u64 = if (std.ascii.eqlIgnoreCase(host, "apimonaco.xyz")) 1329 else if (std.ascii.eqlIgnoreCase(host, "staging.apimonaco.xyz")) 1328 else null;
    const chain = if (get(opts, "chain_id") != .null) try w.unsigned(u64, get(opts, "chain_id")) else default_chain orelse return error.InvalidNetwork;
    if (chain == 0 or (default_chain != null and chain != default_chain.?)) return error.InvalidNetwork;
    const mode = if (get(opts, "signing") == .null) "legacy" else try text(opts, "signing");
    if (!eq(u8, mode, "legacy") and !eq(u8, mode, "rfc9421")) return error.InvalidSigningMode;
    var request = V{ .object = .empty };
    try request.object.put(a, "address", w.text(wallet));
    try request.object.put(a, "clientId", w.text(client));
    try request.object.put(a, "chainId", w.text(try std.fmt.allocPrint(a, "{d}", .{chain})));
    try request.object.put(a, "sessionPublicKey", w.text(pubkey));
    if (eq(u8, phase, "challenge")) return request;
    if (eq(u8, phase, "finish")) {
        const verified = try w.field(input, "response");
        const reported = get(get(verified, "user"), "address");
        if (reported != .null and !eq(u8, try address(a, reported), wallet)) return error.AuthenticationRequired;
        var session = V{ .object = .empty };
        try session.object.put(a, "expires_at", .{ .integer = @intCast(try expiry(verified, io)) });
        try session.object.put(a, "origin", w.text(origin));
        try session.object.put(a, "seed", try w.field(opts, "seed"));
        try session.object.put(a, "signing", w.text(mode));
        if (get(opts, "ws_origin") != .null) try session.object.put(a, "ws_origin", w.text(try text(opts, "ws_origin")));
        return session;
    }
    const challenge = try w.field(input, "challenge");
    _ = try expiry(challenge, io);
    const message = try text(challenge, "message");
    if (std.mem.find(u8, message, pubkey) == null) return error.AuthenticationRequired;
    const nonce = try text(challenge, "nonce");
    if (std.mem.trim(u8, nonce, " \t\r\n").len == 0) return error.InvalidNonce;
    if (eq(u8, phase, "message")) return w.text(message);
    if (!eq(u8, phase, "verify")) return error.InvalidOperation;
    try request.object.put(a, "nonce", w.text(nonce));
    try request.object.put(a, "signature", try w.field(input, "signature"));
    if (get(opts, "invite_code") != .null) try request.object.put(a, "inviteCode", w.text(try text(opts, "invite_code")));
    if (get(opts, "referral_code") != .null) try request.object.put(a, "referralCode", w.text(try text(opts, "referral_code")));
    return request;
}
