const std = @import("std");
const w = @import("wire.zig");
const p = @import("perps.zig");
const http = @import("http.zig");
const auth = @import("exchange_http.zig");
const apex = @import("../exchanges/apex_omni/stream.zig");
const kuru = @import("../exchanges/kuru/stream.zig");
const reya = @import("../exchanges/reya/stream.zig");
const Venue = @import("routes.zig").Venue;
const Dial = @import("host_stream.zig").Dial;
const A = w.A;
const V = w.V;
const eq = std.mem.eql;

pub const Heartbeat = struct {
    payload: []const u8 = "",
    opcode: u8 = 1,
    seconds: u32 = 10,
    perpl: bool = false,
    apex_omni: bool = false,
};
pub const Event = union(enum) { skip, deliver, deliver_final, reply: []const u8, subscribe, heartbeat: []const u8 };

pub fn venue(name: []const u8) ?Venue {
    const v = std.meta.stringToEnum(Venue, name) orelse return null;
    return if (auth.supports(v)) v else null;
}

pub fn stringify(a: A, value: anytype) ![]const u8 {
    return std.json.Stringify.valueAlloc(a, value, .{ .emit_null_optional_fields = false });
}

pub fn same(v: V, field: []const u8, value: []const u8) bool {
    const item = p.get(v, field);
    return item == .string and eq(u8, item.string, value);
}

pub fn optionalText(v: V, field: []const u8) !?[]const u8 {
    const item = p.get(v, field);
    return if (item == .null) null else try w.string(item);
}

pub fn requiredText(v: V, field: []const u8) ![]const u8 {
    const value = (try optionalText(v, field)) orelse return error.InvalidSubscription;
    if (value.len == 0 or value.len > 200) return error.InvalidSubscription;
    return value;
}

pub fn optionalDepth(v: V, fallback: u32) !u32 {
    const value = p.get(v, "depth");
    return if (value == .null) fallback else w.unsigned(u32, value);
}

pub fn binanceSigned(a: A, input: V, params: V, io: std.Io) !V {
    var signed = V{ .object = try params.object.clone(a) };
    _ = signed.object.swapRemove("signature");
    try auth.set(a, &signed, "apiKey", try auth.apiKey(input));
    try p.put(&signed, a, "timestamp", .{ .number_string = try auth.timeText(a, io) });
    const msg = try auth.sortedFields(a, signed, false);
    try auth.set(a, &signed, "signature", try auth.hmac(a, try auth.secret(input), msg, false));
    return signed;
}

pub fn login(a: A, v: Venue, input: V, id: u64, io: std.Io) ![]const u8 {
    return switch (v) {
        .aevo => stringify(a, .{ .id = id, .op = "auth", .data = .{ .key = try auth.apiKey(input), .secret = try auth.secret(input) } }),
        .binance_spot => stringify(a, .{ .id = id, .method = "userDataStream.subscribe.signature", .params = try binanceSigned(a, input, p.object(), io) }),
        .bybit => blk: {
            const expiry = try p.epoch(io, .ms) + 10_000;
            const prehash = try std.fmt.allocPrint(a, "GET/realtime{d}", .{expiry});
            break :blk stringify(a, .{ .op = "auth", .args = .{ try auth.apiKey(input), expiry, try auth.hmac(a, try auth.secret(input), prehash, false) } });
        },
        .deribit => stringify(a, .{ .jsonrpc = "2.0", .id = id, .method = "public/auth", .params = .{ .grant_type = "client_credentials", .client_id = try auth.apiKey(input), .client_secret = try auth.secret(input) } }),
        .okx => blk: {
            const timestamp = try std.fmt.allocPrint(a, "{d}", .{try p.epoch(io, .ms) / 1000});
            const prehash = try std.mem.concat(a, u8, &.{ timestamp, "GET/users/self/verify" });
            const passphrase = try p.credential(p.get(p.get(input, "auth"), "passphrase"));
            break :blk stringify(a, .{ .op = "login", .args = .{.{ .apiKey = try auth.apiKey(input), .passphrase = passphrase, .timestamp = timestamp, .sign = try auth.hmac(a, try auth.secret(input), prehash, true) }} });
        },
        .orderly => blk: {
            const stamp = try auth.timeText(a, io);
            break :blk stringify(a, .{ .id = "auth", .event = "auth", .params = .{ .orderly_key = try auth.orderlyPublicKey(a, input), .sign = try auth.orderlySignature(a, input, stamp), .timestamp = stamp } });
        },
        else => error.InvalidOperation,
    };
}

pub fn acknowledged(v: Venue, frame: V, id: u64) ?bool {
    const frame_id = p.get(frame, "id");
    const matches = (w.unsigned(u64, frame_id) catch 0) == id;
    return switch (v) {
        .aevo => if (same(frame, "op", "auth") or matches) p.get(frame, "error") == .null and !same(frame, "type", "error") and !(p.get(frame, "success") == .bool and !p.get(frame, "success").bool) else null,
        .binance_spot => if (matches) statusOk(frame, "status") and p.get(frame, "error") == .null else null,
        .bybit => if (same(frame, "op", "auth")) (p.get(frame, "success") == .bool and p.get(frame, "success").bool) or (w.unsigned(u32, p.get(frame, "retCode")) catch 1) == 0 else null,
        .deribit => if (matches) p.get(p.get(frame, "result"), "access_token") == .string else null,
        .okx => if (same(frame, "event", "login") or same(frame, "event", "error")) same(frame, "event", "login") and same(frame, "code", "0") else null,
        .orderly => if (same(frame, "event", "auth")) p.get(frame, "success") == .bool and p.get(frame, "success").bool else null,
        else => null,
    };
}

pub fn statusOk(frame: V, field: []const u8) bool {
    const status = w.unsigned(u16, p.get(frame, field)) catch return false;
    return status >= 200 and status < 300;
}

pub fn heartbeat(v: Venue) ?Heartbeat {
    return switch (v) {
        .bybit => .{ .payload = "{\"op\":\"ping\"}", .seconds = 20 },
        .okx => .{ .payload = "ping", .seconds = 20 },
        .aevo => .{ .payload = "{\"op\":\"ping\"}", .seconds = 20 },
        .apex_omni => .{ .apex_omni = true, .seconds = 15 },
        else => null,
    };
}

pub fn krakenToken(a: A, input: V, dial: ?Dial) ![]const u8 {
    const origin = p.get(p.get(input, "auth"), "rest_origin");
    const job = try @import("perps_connectors.zig").apiRequest(a, input, .kraken_spot, "websocket_token", p.object(), origin);
    const result = try @import("transport.zig").executeWithDial(a, job, dial);
    return p.credential(p.get(p.get(result, "result"), "token"));
}

pub fn hibachiAccount(params: V, input: V) !u64 {
    const configured = p.get(p.get(input, "auth"), "account_id");
    const supplied = p.get(params, "accountId");
    if (supplied != .null and configured != .null and try w.unsigned(u64, supplied) != try w.unsigned(u64, configured)) return error.InvalidCredential;
    return w.unsigned(u64, if (supplied == .null) configured else supplied);
}

pub const State = struct {
    kind: Venue,
    input: V,
    endpoint: []const u8,
    channel: []const u8,
    topic: []const u8 = "",
    symbol: ?[]const u8 = null,
    frame: []const u8 = "",
    symbols: ?V = null,
    depth: ?u32 = null,
    snapshot: ?bool = null,
    account: ?u64 = null,
    private: bool = false,
    ready: bool = true,
    book: bool = false,
    auto_reconnect: bool = true,
    kuru: ?kuru.Subscription = null,

    pub fn init(a: A, v: Venue, input: V, params: V) !State {
        if (params != .object) return error.InvalidSubscription;
        const channel = try requiredText(params, "channel");
        const testnet = try p.testnet(input);
        const override = try optionalText(input, "origin");
        var s: State = .{ .kind = v, .input = input, .channel = channel, .endpoint = undefined };
        s.topic = channel;
        const t = try optionalText(params, "topic");
        switch (v) {
            .orderly => try @import("../exchanges/orderly/stream.zig").init(&s, a, input, params, channel, t, override, testnet),
            .binance_spot => try @import("../exchanges/binance_spot/stream.zig").init(&s, a, input, params, channel, t, override, testnet),
            .deribit => try @import("../exchanges/deribit/stream.zig").init(&s, a, input, params, channel, t, override, testnet),
            .bybit => try @import("../exchanges/bybit/stream.zig").init(&s, a, input, params, channel, t, override, testnet),
            .okx => try @import("../exchanges/okx/stream.zig").init(&s, a, input, params, channel, t, override, testnet),
            .backpack => try @import("../exchanges/backpack/stream.zig").init(&s, a, input, params, channel, t, override, testnet),
            .dydx_v4 => try @import("../exchanges/dydx_v4/stream.zig").init(&s, a, input, params, channel, t, override, testnet),
            .kraken_spot => try @import("../exchanges/kraken_spot/stream.zig").init(&s, a, input, params, channel, t, override, testnet),
            .aevo => try @import("../exchanges/aevo/stream.zig").init(&s, a, input, params, channel, t, override, testnet),
            .hibachi => try @import("../exchanges/hibachi/stream.zig").init(&s, a, input, params, channel, t, override, testnet),
            .apex_omni => try apex.init(&s, a, params, channel, override, testnet),
            .kuru => try kuru.init(&s, a, params, channel, override, testnet),
            .reya => try reya.init(&s, a, params, channel, override, testnet),
            else => unreachable,
        }
        return s;
    }

    pub fn start(s: *State, a: A, io: std.Io, dial: ?Dial, url: std.Uri, headers: *std.ArrayList(std.http.Header)) !@import("perps_stream.zig").Opening {
        s.ready = true;
        var opening: @import("perps_stream.zig").Opening = .{ .frame = s.frame };
        if (s.kind == .apex_omni) opening.url = try apex.openingUrl(a, s.input, io, url);
        if (!s.private) return opening;
        switch (s.kind) {
            .aevo, .bybit, .deribit, .okx, .orderly => {
                s.ready = false;
                opening.frame = try login(a, s.kind, s.input, 2, io);
            },
            .binance_spot => {
                s.ready = false;
                opening.frame = try login(a, s.kind, s.input, 2, io);
            },
            .backpack => {
                const time = try auth.timeText(a, io);
                const key = try auth.apiKey(s.input);
                const msg = try std.fmt.allocPrint(a, "instruction=subscribe&timestamp={s}&window=5000", .{time});
                opening.frame = try stringify(a, .{ .method = "SUBSCRIBE", .params = .{s.topic}, .id = "1", .signature = .{ key, try auth.backpackSignature(a, s.input, msg), time, "5000" } });
            },
            .kraken_spot => opening.frame = try stringify(a, .{ .method = "subscribe", .req_id = 1, .params = .{ .channel = s.topic, .token = try krakenToken(a, s.input, dial) } }),
            .hibachi => {
                try headers.append(a, .{ .name = "Authorization", .value = try auth.apiKey(s.input) });
                var endpoint = url;
                if (url.query == null) endpoint.query = .{ .raw = try std.fmt.allocPrint(a, "accountId={d}", .{s.account.?}) };
                opening.url = endpoint;
            },
            else => {},
        }
        return opening;
    }

    pub fn event(s: *State, a: A, raw: []const u8, io: std.Io) !Event {
        if (eq(u8, raw, "pong")) return .skip;
        if (eq(u8, raw, "ping")) return .{ .reply = "pong" };
        const frame = try w.parse(a, raw);
        if (frame != .object) return error.InvalidResponse;
        if (s.kind == .orderly and same(frame, "event", "ping")) return .{ .reply = "{\"event\":\"pong\"}" };
        if (s.kind == .deribit and same(frame, "method", "heartbeat")) {
            if (same(p.get(frame, "params"), "type", "test_request"))
                return .{ .reply = "{\"jsonrpc\":\"2.0\",\"id\":0,\"method\":\"public/test\",\"params\":{}}" };
            return .skip;
        }
        if (!s.ready) {
            const accepted = acknowledged(s.kind, frame, 2) orelse return .skip;
            if (!accepted) return error.AuthenticationRequired;
            s.ready = true;
            return if (s.kind == .binance_spot) .skip else .subscribe;
        }
        const fault = p.get(frame, "error");
        const success = p.get(frame, "success");
        if ((fault != .null and fault != .array) or (success == .bool and !success.bool) or
            same(frame, "type", "error") or same(frame, "event", "error") or same(frame, "op", "error"))
            return error.ExchangeApi;
        return switch (s.kind) {
            .orderly => @import("../exchanges/orderly/stream.zig").event(s, a, frame),
            .binance_spot => @import("../exchanges/binance_spot/stream.zig").event(s, a, frame),
            .deribit => @import("../exchanges/deribit/stream.zig").event(s, a, frame),
            .bybit => @import("../exchanges/bybit/stream.zig").event(s, a, frame),
            .okx => @import("../exchanges/okx/stream.zig").event(s, a, frame),
            .backpack => @import("../exchanges/backpack/stream.zig").event(s, a, frame),
            .dydx_v4 => @import("../exchanges/dydx_v4/stream.zig").event(s, a, frame),
            .kraken_spot => @import("../exchanges/kraken_spot/stream.zig").event(s, a, frame),
            .aevo => @import("../exchanges/aevo/stream.zig").event(s, a, frame),
            .hibachi => @import("../exchanges/hibachi/stream.zig").event(s, a, frame),
            .apex_omni => apex.event(s, a, frame, io),
            .kuru => kuru.event(s, frame),
            .reya => reya.event(s, frame),
            else => unreachable,
        };
    }
};

pub fn in(value: []const u8, values: []const []const u8) bool {
    for (values) |v| if (eq(u8, value, v)) return true;
    return false;
}
