const std = @import("std");
const p = @import("../../core/perps.zig");
const w = @import("../../core/wire.zig");
const evm = @import("../../core/evm.zig");
const Ed25519 = std.crypto.sign.Ed25519;
const base64 = std.base64.url_safe_no_pad.Encoder;

fn signature(a: p.A, config: p.V, canonical: []const u8) ![]const u8 {
    var seed = try evm.hex(32, try p.credential(p.get(config, "api_secret")));
    defer std.crypto.secureZero(u8, &seed);
    var pair = try Ed25519.KeyPair.generateDeterministic(seed);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&pair));
    const sig = (try pair.sign(canonical, null)).toBytes();
    return base64.encode(try a.alloc(u8, base64.calcSize(sig.len)), &sig);
}

const Authorization = struct {
    key: []const u8,
    nonce: []const u8,
    timestamp: []const u8,
    chain: u32,

    fn fresh(a: p.A, config: p.V, io: std.Io) !Authorization {
        var random: [16]u8 = undefined;
        try io.randomSecure(&random);
        return .{
            .key = try p.credential(p.get(config, "api_key")),
            .nonce = base64.encode(try a.alloc(u8, base64.calcSize(random.len)), &random),
            .timestamp = try std.fmt.allocPrint(a, "{d}", .{try p.epoch(io, .ms)}),
            .chain = if (try p.testnet(config)) 10143 else 143,
        };
    }
};

pub fn headers(a: p.A, config: p.V, io: std.Io, request: *@import("../../core/http.zig").Request, payload: []const u8) !void {
    const auth = try Authorization.fresh(a, config, io);
    var hash: [32]u8 = undefined;
    std.crypto.hash.sha2.Sha256.hash(payload, &hash, .{});
    // The gateway mounts the API at /api. The upstream signs the /v1 target.
    const target = if (std.mem.startsWith(u8, request.path, "/api/")) request.path[4..] else return error.InvalidPath;
    const canonical = try std.fmt.allocPrint(a, "{d}\n{s}\n{s}\n{s}\n{s}\n{x}", .{ auth.chain, request.method, target, auth.timestamp, auth.nonce, &hash });
    try p.put(&request.headers, a, "X-API-Key", w.text(auth.key));
    try p.put(&request.headers, a, "X-API-Nonce", w.text(auth.nonce));
    try p.put(&request.headers, a, "X-API-Timestamp", w.text(auth.timestamp));
    try p.put(&request.headers, a, "X-API-Signature", w.text(try signature(a, config, canonical)));
}

pub fn signIn(a: p.A, config: p.V, io: std.Io) ![]const u8 {
    const auth = try Authorization.fresh(a, config, io);
    const canonical = try std.fmt.allocPrint(a, "{d}\ntrading-ws-signin\n{s}\n{s}", .{ auth.chain, auth.timestamp, auth.nonce });
    return std.json.Stringify.valueAlloc(a, .{
        .mt = 29,
        .chain_id = auth.chain,
        .api_key = auth.key,
        .timestamp = auth.timestamp,
        .nonce = auth.nonce,
        .signature = try signature(a, config, canonical),
    }, .{});
}

pub fn order(a: p.A, name: []const u8, spec: p.V) !p.V {
    const cancel = p.eq(u8, name, "cancel_order");
    const kind = try w.unsigned(u8, try w.field(spec, "t"));
    const request_id = try w.unsigned(u64, try w.field(spec, "rq"));
    if (request_id == 0) return error.InvalidParameter;
    _ = try w.unsigned(u32, try w.field(spec, "mkt"));
    _ = try w.unsigned(u64, try w.field(spec, "acc"));
    if (cancel) {
        if (kind != 5 or (try w.unsigned(u64, try w.field(spec, "oid"))) == 0) return error.InvalidParameter;
    } else {
        if (kind < 1 or kind > 4 or p.get(spec, "oid") != .null or (try w.unsigned(u64, try w.field(spec, "s"))) == 0)
            return error.InvalidParameter;
    }
    var result = p.object();
    try p.put(&result, a, "d", try p.array(a, &.{spec}));
    return result;
}

pub fn checkOrder(value: p.V) !void {
    if (try status(p.get(value, "status")) != 0) return error.ExchangeApi;
    const items = p.get(value, "statuses");
    if (items != .array or items.array.items.len != 1) return error.PendingOutcome;
    if (try status(items.array.items[0]) != 0) return error.ExchangeApi;
}

fn status(value: p.V) !u32 {
    return w.unsigned(u32, p.get(value, "code")) catch error.PendingOutcome;
}
