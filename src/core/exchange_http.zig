const std = @import("std");
const http = @import("http.zig");
const p = @import("perps.zig");
const w = @import("wire.zig");
const Venue = @import("routes.zig").Venue;
const aevo = @import("../exchanges/aevo/trading.zig");
const hibachi = @import("../exchanges/hibachi/trading.zig");
const ed = std.crypto.sign.Ed25519;
const eq = std.mem.eql;
const Hmac = std.crypto.auth.hmac.sha2.HmacSha256;
const Bytes = std.ArrayList(u8);
const A = w.A;
const V = w.V;

pub fn supports(venue: Venue) bool {
    return switch (venue) {
        .aevo, .apex_omni, .backpack, .binance_spot, .bybit, .deribit, .dydx_v4, .hibachi, .kraken_spot, .kuru, .okx, .orderly, .reya => true,
        else => false,
    };
}

pub fn isWrite(venue: Venue, op: []const u8) bool {
    if (!supports(venue)) return false;
    for ([_][]const u8{ "place_", "cancel_", "amend_", "replace_" }) |prefix| {
        if (std.mem.startsWith(u8, op, prefix)) return true;
    }
    return venue == .deribit and (eq(u8, op, "buy") or eq(u8, op, "sell"));
}

pub fn origin(venue: Venue, operation: []const u8, config: V, fallback: []const u8) ![]const u8 {
    const auth = p.get(config, "auth");
    if (venue == .dydx_v4 and (isWrite(venue, operation) or eq(u8, operation, "transaction"))) {
        const node = p.get(auth, "node_origin");
        return if (node == .null) fallback else p.credential(node);
    }
    if (!try p.testnet(config)) return fallback;
    return switch (venue) {
        .aevo => "https://api-testnet.aevo.xyz",
        .apex_omni => "https://qa.omni.apex.exchange",
        .binance_spot => "https://testnet.binance.vision",
        .bybit => "https://api-testnet.bybit.com",
        .deribit => "https://test.deribit.com",
        .dydx_v4 => "https://indexer.v4testnet.dydx.exchange",
        .kuru => "https://api.testnet.kuru.io",
        .reya => "https://api-test.reya.xyz",
        .orderly => if (std.mem.startsWith(u8, operation, "market_")) fallback else "https://testnet-api.orderly.org",
        .okx => fallback,
        else => error.InvalidNetwork,
    };
}

pub fn encode(a: A, enc: std.base64.Base64Encoder, bytes: []const u8) ![]const u8 {
    return enc.encode(try a.alloc(u8, enc.calcSize(bytes.len)), bytes);
}

pub fn decodeBase64(a: A, text: []const u8) ![]u8 {
    const decoder = std.base64.standard.Decoder;
    const data = try a.alloc(u8, decoder.calcSizeForSlice(text) catch return error.InvalidCredential);
    decoder.decode(data, text) catch return error.InvalidCredential;
    return data;
}

pub fn hmac(a: A, key: []const u8, msg: []const u8, base64: bool) ![]const u8 {
    var sig: [Hmac.mac_length]u8 = undefined;
    Hmac.create(&sig, msg, key);
    return if (base64) encode(a, std.base64.standard.Encoder, &sig) else std.fmt.allocPrint(a, "{x}", .{&sig});
}

pub fn set(a: A, obj: *V, k: []const u8, value: []const u8) !void {
    try p.put(obj, a, k, w.text(value));
}

pub fn apiKey(input: V) ![]const u8 {
    return p.credential(p.get(input, "api_key"));
}

pub fn secret(input: V) ![]const u8 {
    return p.credential(p.get(input, "api_secret"));
}

pub fn timeText(a: A, io: std.Io) ![]const u8 {
    return std.fmt.allocPrint(a, "{d}", .{try p.epoch(io, .ms)});
}

pub fn okxTime(a: A, millis: u64) ![]const u8 {
    const time: std.time.epoch.EpochSeconds = .{ .secs = millis / 1000 };
    const date = time.getEpochDay().calculateYearDay();
    const month = date.calculateMonthDay();
    const hours = time.getDaySeconds();
    return std.fmt.allocPrint(a, "{d:0>4}-{d:0>2}-{d:0>2}T{d:0>2}:{d:0>2}:{d:0>2}.{d:0>3}Z", .{
        date.year, month.month.numeric(), month.day_index + 1, hours.getHoursIntoDay(), hours.getMinutesIntoHour(), hours.getSecondsIntoMinute(), millis % 1000,
    });
}

fn timestampedPath(a: A, path: []const u8, time: []const u8) !Bytes {
    var out: Bytes = .empty;
    const start = std.mem.indexOfScalar(u8, path, '?') orelse path.len;
    if (start < path.len) try out.appendSlice(a, path[start..]);
    try http.pair(a, &out, "timestamp", time, false);
    return out;
}

pub fn sortedFields(a: A, value: V, escaped: bool) ![]const u8 {
    if (value != .object) return error.ExpectedObject;
    const keys = try a.dupe([]const u8, value.object.keys());
    std.mem.sort([]const u8, keys, {}, less);
    var out: Bytes = .empty;
    for (keys) |k| {
        const v = value.object.get(k).?;
        if (v == .null) continue;
        const s = if (v == .array or v == .object) try std.json.Stringify.valueAlloc(a, v, .{}) else try http.scalar(v);
        if (escaped) {
            try http.pair(a, &out, k, s, false);
        } else {
            if (std.mem.findAny(u8, s, "&=") != null) return error.InvalidParameter;
            if (out.items.len != 0) try out.append(a, '&');
            try out.appendSlice(a, k);
            try out.append(a, '=');
            try out.appendSlice(a, s);
        }
    }
    return if (escaped and out.items.len != 0) out.items[1..] else out.items;
}

fn less(_: void, l: []const u8, r: []const u8) bool {
    return std.mem.lessThan(u8, l, r);
}

fn seed58(text: []const u8) ![32]u8 {
    const key = if (std.mem.startsWith(u8, text, "ed25519:")) text[8..] else text;
    if (key.len == 0 or key.len > 44) return error.InvalidCredential;
    const alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    var n: u256 = 0;
    for (key) |c| {
        const index = std.mem.indexOfScalar(u8, alphabet, c) orelse return error.InvalidCredential;
        n = std.math.add(u256, std.math.mul(u256, n, 58) catch return error.InvalidCredential, index) catch return error.InvalidCredential;
    }
    var out: [32]u8 = undefined;
    std.mem.writeInt(u256, &out, n, .big);
    var leading: usize = 0;
    for (key) |c| {
        if (c != '1') break;
        leading += 1;
    }
    const actual = if (n == 0) 0 else (256 - @as(usize, @clz(n)) + 7) / 8;
    if (actual + leading != out.len) return error.InvalidCredential;
    return out;
}

pub fn orderlyKey(a: A, input: V) !ed.KeyPair {
    var seed = try seed58(try secret(input));
    defer std.crypto.secureZero(u8, &seed);
    var pair = try ed.KeyPair.generateDeterministic(seed);
    errdefer std.crypto.secureZero(u8, std.mem.asBytes(&pair));
    const pub_key = try @import("solana.zig").base58(a, 32, pair.public_key.toBytes());
    const supplied = try apiKey(input);
    if (!eq(u8, pub_key, if (std.mem.startsWith(u8, supplied, "ed25519:")) supplied[8..] else supplied)) return error.InvalidCredential;
    return pair;
}

pub fn orderlySignature(a: A, input: V, message: []const u8) ![]const u8 {
    var pair = try orderlyKey(a, input);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&pair));
    const sig = (try pair.sign(message, null)).toBytes();
    return encode(a, std.base64.url_safe_no_pad.Encoder, &sig);
}

pub fn orderlyPublicKey(a: A, input: V) ![]const u8 {
    const key = try apiKey(input);
    if (std.mem.startsWith(u8, key, "ed25519:")) return key;
    return std.mem.concat(a, u8, &.{ "ed25519:", key });
}

pub fn backpackSignature(a: A, input: V, message: []const u8) ![]const u8 {
    const raw = try decodeBase64(a, try secret(input));
    defer std.crypto.secureZero(u8, raw);
    if (raw.len != 32) return error.InvalidCredential;
    var pair = try ed.KeyPair.generateDeterministic(raw[0..32].*);
    defer std.crypto.secureZero(u8, std.mem.asBytes(&pair));
    const public = try encode(a, std.base64.standard.Encoder, &pair.public_key.toBytes());
    if (!eq(u8, try apiKey(input), public)) return error.InvalidCredential;
    const sig = (try pair.sign(message, null)).toBytes();
    return encode(a, std.base64.standard.Encoder, &sig);
}

fn backpackInstruction(op: []const u8) ![]const u8 {
    const map = .{
        .account = "accountQuery",
        .balances = "balanceQuery",
        .positions = "positionQuery",
        .order_status = "orderQuery",
        .open_orders = "orderQueryAll",
        .order_history = "orderHistoryQueryAll",
        .fills = "fillHistoryQueryAll",
        .place_order = "orderExecute",
        .place_orders = "orderExecute",
        .cancel_order = "orderCancel",
        .cancel_all_orders = "orderCancelAll",
    };
    inline for (@typeInfo(@TypeOf(map)).@"struct".field_names) |field| {
        if (eq(u8, op, field)) return @field(map, field);
    }
    return error.InvalidOperation;
}

fn backpackMessage(a: A, op: []const u8, params: V, body: V, time: []const u8) ![]const u8 {
    const instruction = try backpackInstruction(op);
    const prefix = try std.mem.concat(a, u8, &.{ "instruction=", instruction });
    var out: Bytes = .empty;
    const inputs = if (body == .array) body.array.items else &.{if (body == .null) params else body};
    if (inputs.len == 0) return error.InvalidParameter;
    for (inputs, 0..) |item, i| {
        if (i != 0) try out.append(a, '&');
        try out.appendSlice(a, prefix);
        const sorted = try sortedFields(a, item, false);
        if (sorted.len != 0) {
            try out.append(a, '&');
            try out.appendSlice(a, sorted);
        }
    }
    try out.appendSlice(a, try std.mem.concat(a, u8, &.{ "&timestamp=", time, "&window=5000" }));
    return out.items;
}

const KrakenStripe = struct { lock: std.Io.Mutex = .init, nonce: std.atomic.Value(u64) = .init(0) };
var kraken_stripes: [64]KrakenStripe = @splat(.{});

fn krakenStripe(input: V) !*KrakenStripe {
    return &kraken_stripes[std.hash.Wyhash.hash(0, try apiKey(input)) % kraken_stripes.len];
}

pub fn lockKraken(input: V, io: std.Io) !*std.Io.Mutex {
    const stripe = try krakenStripe(input);
    stripe.lock.lockUncancelable(io);
    return &stripe.lock;
}

fn krakenNonce(stripe: *KrakenStripe, io: std.Io) !u64 {
    const now = try p.epoch(io, .us);
    var previous = stripe.nonce.load(.monotonic);
    while (true) {
        const next = @max(now, previous + 1);
        previous = stripe.nonce.cmpxchgWeak(previous, next, .monotonic, .monotonic) orelse return next;
    }
}

pub fn krakenSignature(a: A, key: []const u8, path: []const u8, nonce: []const u8, body: []const u8) ![]const u8 {
    const raw = try decodeBase64(a, key);
    defer std.crypto.secureZero(u8, raw);
    var sha = std.crypto.hash.sha2.Sha256.init(.{});
    sha.update(nonce);
    sha.update(body);
    const hash = sha.finalResult();
    const Mac = std.crypto.auth.hmac.sha2.HmacSha512;
    var mac = Mac.init(raw);
    mac.update(path);
    mac.update(&hash);
    var sig: [Mac.mac_length]u8 = undefined;
    mac.final(&sig);
    return encode(a, std.base64.standard.Encoder, &sig);
}

fn json(a: A, value: V) !?[]const u8 {
    return if (value == .null) null else try std.json.Stringify.valueAlloc(a, value, .{});
}

pub fn authorize(a: A, venue: Venue, op: []const u8, input: V, request: *http.Request, io: std.Io) !?[]const u8 {
    var payload = try json(a, request.body);
    const auth = p.get(input, "auth");
    if (venue == .okx and try p.testnet(input)) try set(a, &request.headers, "x-simulated-trading", "1");
    if (!request.authenticated) return payload;
    const time = try timeText(a, io);
    switch (venue) {
        .aevo => {
            try set(a, &request.headers, "AEVO-KEY", try apiKey(input));
            try set(a, &request.headers, "AEVO-SECRET", try secret(input));
            if (eq(u8, op, "place_order") or eq(u8, op, "amend_order")) {
                request.body = try aevo.sign(a, input, request.body, io);
                payload = try json(a, request.body);
            }
        },
        .backpack => {
            const params = try w.field(input, "params");
            // Read parameters are reconstructed from the encoded target by the caller before route preparation.
            const values = p.get(input, "wire_params");
            const msg = try backpackMessage(a, op, if (values == .null) params else values, request.body, time);
            try set(a, &request.headers, "X-API-Key", try apiKey(input));
            try set(a, &request.headers, "X-Timestamp", time);
            try set(a, &request.headers, "X-Window", "5000");
            try set(a, &request.headers, "X-Signature", try backpackSignature(a, input, msg));
        },
        .binance_spot => {
            const bare = std.mem.sliceTo(request.path, '?');
            var query = try timestampedPath(a, request.path, time);
            const signature = try hmac(a, try secret(input), query.items[1..], false);
            try http.pair(a, &query, "signature", signature, false);
            request.path = try std.mem.concat(a, u8, &.{ bare, query.items });
            try set(a, &request.headers, "X-MBX-APIKEY", try apiKey(input));
        },
        .bybit => {
            const extra = p.get(auth, "recv_window_ms");
            const window = if (extra == .null) "5000" else try http.scalar(extra);
            const key = try apiKey(input);
            const q = std.mem.indexOfScalar(u8, request.path, '?');
            const target = payload orelse if (q) |j| request.path[j + 1 ..] else "";
            const msg = try std.mem.concat(a, u8, &.{ time, key, window, target });
            try set(a, &request.headers, "X-BAPI-API-KEY", key);
            try set(a, &request.headers, "X-BAPI-TIMESTAMP", time);
            try set(a, &request.headers, "X-BAPI-RECV-WINDOW", window);
            try set(a, &request.headers, "X-BAPI-SIGN", try hmac(a, try secret(input), msg, false));
        },
        .deribit => {
            if (eq(u8, op, "authenticate")) {
                var query: Bytes = .empty;
                try http.pair(a, &query, "grant_type", "client_credentials", false);
                try http.pair(a, &query, "client_id", try apiKey(input), false);
                try http.pair(a, &query, "client_secret", try secret(input), false);
                request.path = try std.mem.concat(a, u8, &.{ request.path, query.items });
            } else {
                const metadata = p.get(p.get(input, "metadata"), "authenticate");
                const token = try p.credential(p.get(p.get(metadata, "result"), "access_token"));
                try set(a, &request.headers, "Authorization", try std.mem.concat(a, u8, &.{ "Bearer ", token }));
            }
        },
        .hibachi => {
            try set(a, &request.headers, "Authorization", try apiKey(input));
            if (isWrite(venue, op)) {
                const meta = p.get(p.get(input, "metadata"), "markets");
                request.body = try hibachi.sign(a, input, op, request.body, meta, io);
                payload = try json(a, request.body);
            }
        },
        .kraken_spot => {
            const stripe = try krakenStripe(input);
            // Native dispatch holds the stripe across the request and response.
            const nonce = try std.fmt.allocPrint(a, "{d}", .{try krakenNonce(stripe, io)});
            const path = std.mem.sliceTo(request.path, '?');
            if (request.body == .null) {
                const tail = if (path.len < request.path.len) request.path[path.len + 1 ..] else "";
                payload = try std.mem.concat(a, u8, &.{ "nonce=", nonce, if (tail.len != 0) "&" else "", tail });
                _ = request.headers.object.swapRemove("Content-Type");
                try set(a, &request.headers, "Content-Type", "application/x-www-form-urlencoded");
            } else {
                try p.put(&request.body, a, "nonce", .{ .number_string = nonce });
                payload = try json(a, request.body);
            }
            request.path = path;
            try set(a, &request.headers, "API-Key", try apiKey(input));
            try set(a, &request.headers, "API-Sign", try krakenSignature(a, try secret(input), path, nonce, payload.?));
        },
        .okx => {
            const stamp = try okxTime(a, try p.epoch(io, .ms));
            const msg = try std.mem.concat(a, u8, &.{ stamp, request.method, request.path, payload orelse "" });
            try set(a, &request.headers, "OK-ACCESS-KEY", try apiKey(input));
            try set(a, &request.headers, "OK-ACCESS-PASSPHRASE", try p.credential(p.get(auth, "passphrase")));
            try set(a, &request.headers, "OK-ACCESS-TIMESTAMP", stamp);
            try set(a, &request.headers, "OK-ACCESS-SIGN", try hmac(a, try secret(input), msg, true));
        },
        .orderly => {
            const msg = try std.mem.concat(a, u8, &.{ time, request.method, request.path, payload orelse "" });
            try set(a, &request.headers, "orderly-account-id", try p.credential(p.get(auth, "account_id")));
            try set(a, &request.headers, "orderly-key", try orderlyPublicKey(a, input));
            try set(a, &request.headers, "orderly-timestamp", time);
            try set(a, &request.headers, "orderly-signature", try orderlySignature(a, input, msg));
            if (payload == null) try set(a, &request.headers, "Content-Type", "application/x-www-form-urlencoded");
        },
        else => return error.InvalidOperation,
    }
    return payload;
}

pub fn dependencies(a: A, input: V, venue: Venue, op: []const u8) !V {
    var jobs = p.object();
    if (venue == .deribit and !eq(u8, op, "authenticate") and try http.operationPrivate(input)) {
        try p.put(&jobs, a, "authenticate", try @import("perps_connectors.zig").apiRequest(a, input, venue, "authenticate", p.object(), p.get(input, "origin")));
    } else if (venue == .hibachi and (eq(u8, op, "place_order") or eq(u8, op, "amend_order") or eq(u8, op, "place_orders"))) {
        const custom = p.get(p.get(input, "auth"), "market_origin");
        const base = if (custom != .null) custom else p.get(input, "origin");
        var job = try @import("perps_connectors.zig").apiRequest(a, input, venue, "markets", p.object(), base);
        _ = job.object.swapRemove("api_key");
        _ = job.object.swapRemove("api_secret");
        try p.put(&jobs, a, "markets", job);
    }
    return jobs;
}

pub fn validateResponse(venue: Venue, value: V) !V {
    const err = p.get(value, "error");
    switch (venue) {
        .binance_spot => if (p.get(value, "code") != .null and p.get(value, "msg") != .null) return error.ExchangeApi,
        .bybit => if (try w.unsigned(u32, try w.field(value, "retCode")) != 0) return error.ExchangeApi,
        .kraken_spot => {
            if (err != .array) return error.InvalidResponse;
            for (err.array.items) |entry| if (std.mem.startsWith(u8, try w.string(entry), "E")) return error.ExchangeApi;
        },
        .okx => {
            const code = try w.string(try w.field(value, "code"));
            if (!eq(u8, code, "0") and !eq(u8, code, "2")) return error.ExchangeApi;
        },
        .orderly => {
            const success = try w.field(value, "success");
            if (success != .bool) return error.InvalidResponse;
            if (!success.bool) return error.ExchangeApi;
        },
        .aevo => if (err != .null or (p.get(value, "success") == .bool and !p.get(value, "success").bool)) return error.ExchangeApi,
        else => if (err != .null) return error.ExchangeApi,
    }
    return value;
}

test "Kraken Spot published AddOrder signature" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    const a = arena.allocator();
    const key = "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==";
    const body = "nonce=1616492376594&ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25";
    const expected = "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==";
    try std.testing.expectEqualStrings(expected, try krakenSignature(a, key, "/0/private/AddOrder", "1616492376594", body));
}

test "OKX timestamp preserves milliseconds and UTC" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    try std.testing.expectEqualStrings("2023-11-14T22:13:20.017Z", try okxTime(arena.allocator(), 1700000000017));
}
