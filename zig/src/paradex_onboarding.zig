//! Paradex onboarding, JWT challenges and Sign-In with Ethereum.
const std = @import("std");
const w = @import("wire.zig");
const st = @import("stark.zig");
const evm = @import("evm.zig");
const A = w.A;
const V = w.V;
const get = @import("polymarket_rpc.zig").get;
fn text(v: V, k: []const u8) ![]const u8 {
    return w.string(try w.field(v, k));
}
fn json(a: A, v: anytype) !V {
    return w.parse(a, try std.json.Stringify.valueAlloc(a, v, .{}));
}
fn network(a: A, input: V, auth: V) ![]const u8 {
    const origin = get(input, "origin");
    const url = if (origin == .null) "https://api.prod.paradex.trade" else try w.string(origin);
    const uri = try std.Uri.parse(url);
    const host = try (uri.host orelse return error.InvalidOrigin).toRawMaybeAlloc(a);
    const known: ?[]const u8 = if (std.ascii.eqlIgnoreCase(host, "api.prod.paradex.trade")) "PRIVATE_SN_PARACLEAR_MAINNET" else if (std.ascii.eqlIgnoreCase(host, "api.testnet.paradex.trade")) "PRIVATE_SN_PARACLEAR_TESTNET" else null;
    const configured = get(auth, "network");
    if (configured == .null) return known orelse error.InvalidNetwork;
    const n = try w.string(configured);
    if (known) |k| if (!std.mem.eql(u8, n, k)) return error.InvalidNetwork;
    return n;
}
fn legacy(a: A, n: []const u8, primary: []const u8, message: V) !V {
    const defs = try w.parse(a, @embedFile("paradex-signing-types.json"));
    var types = V{ .object = .empty };
    for ([_][]const u8{ "StarkNetDomain", primary }) |k| try types.object.put(a, k, try w.field(defs, k));
    return json(a, .{ .domain = .{ .name = "Paradex", .version = "1", .chainId = try std.fmt.allocPrint(a, "0x{x}", .{try st.short(n)}) }, .types = types, .primaryType = primary, .message = message });
}
fn rfc3339(a: A, sec: u64) ![]const u8 {
    const time: std.time.epoch.EpochSeconds = .{ .secs = sec };
    const day = time.getEpochDay().calculateYearDay();
    const month = day.calculateMonthDay();
    const clock = time.getDaySeconds();
    return std.fmt.allocPrint(a, "{d:0>4}-{d:0>2}-{d:0>2}T{d:0>2}:{d:0>2}:{d:0>2}Z", .{ day.year, @backingInt(month.month), month.day_index + 1, clock.getHoursIntoDay(), clock.getMinutesIntoHour(), clock.getSecondsIntoMinute() });
}
fn siwe(a: A, n: []const u8, auth: V, p: V) !V {
    const testnet = std.mem.eql(u8, n, "PRIVATE_SN_PARACLEAR_TESTNET");
    const custom = !testnet and !std.mem.eql(u8, n, "PRIVATE_SN_PARACLEAR_MAINNET");
    const domain = if (get(auth, "siwe_domain") != .null) try text(auth, "siwe_domain") else if (custom) return error.InvalidNetwork else if (testnet) "app.testnet.paradex.trade" else "app.paradex.trade";
    const uri = if (get(auth, "siwe_uri") != .null) try text(auth, "siwe_uri") else if (custom) return error.InvalidNetwork else try std.mem.concat(a, u8, &.{ "https://", domain });
    const chain: u64 = if (get(auth, "evm_chain_id") != .null) try w.unsigned(u64, try w.field(auth, "evm_chain_id")) else if (custom) return error.InvalidNetwork else if (testnet) 11155111 else 1;
    const nonce = try text(p, "nonce");
    if (nonce.len < 8) return error.InvalidNonce;
    for (nonce) |c| if (!std.ascii.isAlphanumeric(c)) return error.InvalidNonce;
    const statement = try text(p, "statement");
    const issued = try text(p, "issued_at");
    const exp = get(p, "expiration");
    const until = if (exp == .null) "" else try w.string(exp);
    for ([_][]const u8{ statement, domain, uri, issued, until }) |s| if (std.mem.findAny(u8, s, "\r\n") != null) return error.InvalidParameter;
    var key = try evm.hex(32, try text(auth, "evm_key"));
    defer std.crypto.secureZero(u8, &key);
    const pk = (try std.crypto.ecc.Secp256k1.basePoint.mul(key, .big)).toUncompressedSec1();
    const addr = try @import("clob_auth.zig").checksum(a, try std.fmt.allocPrint(a, "0x{x}", .{evm.hash(pk[1..])[12..]}));
    const message = try std.fmt.allocPrint(a, "{s} wants you to sign in with your Ethereum account:\n{s}\n\n{s}\n\nURI: {s}\nVersion: 1\nChain ID: {d}\nNonce: {s}\nIssued At: {s}{s}{s}", .{ domain, addr, statement, uri, chain, nonce, issued, if (exp == .null) "" else "\nExpiration Time: ", until });
    const hash = evm.hash(try std.fmt.allocPrint(a, "\x19Ethereum Signed Message:\n{d}{s}", .{ message.len, message }));
    const sig = try @import("transaction.zig").recoverable(key, hash);
    const b64 = std.base64.standard.Encoder;
    return json(a, .{ .message = message, .message_base64 = b64.encode(try a.alloc(u8, b64.calcSize(message.len)), message), .signature = try std.fmt.allocPrint(a, "0x{x}", .{&sig}) });
}
pub fn execute(a: A, input: V) !V {
    const op = std.meta.stringToEnum(enum { onboarding_typed_data, evm_account_address, sign_siwe, prepare_auth, validate_token, block_signature }, try text(input, "operation")) orelse return error.InvalidOperation;
    const p = try w.field(input, "params");
    const auth = get(input, "auth");
    if (op == .evm_account_address) {
        const addr = std.mem.readInt(u256, &try evm.address(try text(p, "address")), .big);
        const class = try st.felt(try text(p, "class_hash"));
        return w.text(try std.fmt.allocPrint(a, "0x{x}", .{st.legacy(&.{ try st.short("STARKNET_CONTRACT_ADDRESS"), 0, addr, class, st.legacy(&.{ 3, addr, 1 }) }) % ((@as(u256, 1) << 251) - 256)}));
    }
    var rt: std.Io.Threaded = .init_single_threaded;
    const time = std.Io.Timestamp.now(rt.io(), .real).toSeconds();
    if (op == .validate_token) return w.text(try @import("venue_auth.zig").bearer(a, p, time));
    const n = try network(a, input, auth);
    if (op == .block_signature) {
        const parent = get(p, "parent");
        if (parent != .null and parent != .string) return error.InvalidParameter;
        const ms = std.math.cast(u64, std.Io.Timestamp.now(rt.io(), .real).toMilliseconds()) orelse return error.InvalidTimestamp;
        return @import("paradex_sign.zig").blockSignature(a, auth, n, w.text(try text(p, "nonce")), try w.unsigned(u64, try w.field(p, "expiration")), try w.field(p, "trades"), if (parent == .null) null else parent, ms);
    }
    if (op == .onboarding_typed_data) return legacy(a, n, "Constant", try json(a, .{ .action = "Onboarding" }));
    if (op == .sign_siwe) return siwe(a, n, auth, p);
    const sec = std.math.cast(u64, time) orelse return error.InvalidTimestamp;
    const exp = sec + 300;
    var headers = V{ .object = .empty };
    try headers.object.put(a, "PARADEX-STARKNET-ACCOUNT", w.text(try text(auth, "account")));
    var method: []const u8 = "get_jwt";
    if (get(auth, "evm_key") != .null) {
        var nonce: [16]u8 = undefined;
        try rt.io().randomSecure(&nonce);
        const proof = try siwe(a, n, auth, try json(a, .{ .statement = "Paradex Auth", .nonce = try std.fmt.allocPrint(a, "{x}", .{&nonce}), .issued_at = try rfc3339(a, sec), .expiration = try rfc3339(a, exp) }));
        try headers.object.put(a, "PARADEX-EVM-SIGNATURE", try w.field(proof, "signature"));
        try headers.object.put(a, "PARADEX-SIWE-MESSAGE", try w.field(proof, "message_base64"));
        method = "evm_auth";
    } else {
        const td = try legacy(a, n, "Request", try json(a, .{ .method = "POST", .path = "/v1/auth", .body = "", .timestamp = sec, .expiration = exp }));
        var signing = V{ .object = try auth.object.clone(a) };
        try signing.object.put(a, "network", w.text(n));
        const proof = try @import("paradex_sign.zig").typed(a, try json(a, .{ .auth = signing, .typed_data = td }));
        try headers.object.put(a, "PARADEX-STARKNET-SIGNATURE", w.text(try std.json.Stringify.valueAlloc(a, try w.field(proof, "signature"), .{})));
        try headers.object.put(a, "PARADEX-TIMESTAMP", w.text(try std.fmt.allocPrint(a, "{d}", .{sec})));
        try headers.object.put(a, "PARADEX-SIGNATURE-EXPIRATION", w.text(try std.fmt.allocPrint(a, "{d}", .{exp})));
        if (get(auth, "subkey") != .null) {
            try headers.object.put(a, "public_key", try w.field(auth, "subkey"));
            method = "get_subkey_jwt";
        }
    }
    return json(a, .{ .operation = method, .params = headers });
}
