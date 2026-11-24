// Each service has an independent socket, authentication exchange and snapshot cursor.
const std = @import("std");
const w = @import("../../core/wire.zig");
const Event = @import("../derive/stream.zig").State.Event;
const Service = enum { market, user, sports, polybolt, perps, rtds, rfq };
const encode = std.json.Stringify.valueAlloc;

fn get(v: w.V, k: []const u8) w.V {
    return if (v == .object) v.object.get(k) orelse .null else .null;
}
fn is(v: w.V, k: []const u8, value: []const u8) bool {
    const s = get(v, k);
    return s == .string and std.mem.eql(u8, s.string, value);
}
fn flag(v: w.V, k: []const u8) bool {
    const b = get(v, k);
    return b == .bool and b.bool;
}
fn credential(v: w.V, k: []const u8) ![]const u8 {
    const s = get(v, k);
    if (s != .string or s.string.len == 0) return error.AuthenticationRequired;
    return s.string;
}
fn book(v: w.V) bool {
    return is(v, "event_type", "book") or flag(v, "snapshot");
}

pub const State = struct {
    service: Service,
    subscription: []const u8,
    login: ?[]const u8 = null,
    expires: ?u64 = null,
    authenticating: bool = false,
    snapshot: bool = false,
    seq: ?u64 = null,

    pub fn init(a: w.A, config: w.V, sub: w.V, io: std.Io) !State {
        const service = std.meta.stringToEnum(Service, try w.string(try w.field(sub, "service"))) orelse return error.UnknownService;
        const p = get(sub, "params");
        var state = State{ .service = service, .subscription = "" };
        const clob = get(config, "clob");
        const auth = get(config, "auth");
        const perps = is(config, "venue", "polymarket_perps");
        if (perps and service != .perps) return error.UnknownService;
        switch (service) {
            .market => {
                const assets = try w.field(p, "assets_ids");
                if (assets != .array or assets.array.items.len == 0) return error.InvalidSubscription;
                for (assets.array.items) |v| _ = try w.string(v);
                const custom = p.object.get("custom_feature_enabled") orelse w.V{ .bool = true };
                if (custom != .bool) return error.InvalidSubscription;
                state.subscription = try encode(a, .{ .type = "market", .assets_ids = assets, .custom_feature_enabled = custom.bool }, .{});
            },
            .rtds => state.subscription = try encode(a, .{ .action = "subscribe", .subscriptions = .{p} }, .{}),
            .sports => {},
            .perps => {
                state.subscription = try encode(a, .{ .id = @as(u8, 1), .req = "sub", .chs = .{try w.string(try w.field(p, "channel"))} }, .{});
                if (perps and (get(auth, "proxy") != .null or get(auth, "secret") != .null)) {
                    state.login = try commandLogin(a, config, true, io);
                    if (get(auth, "expires_at_ms") != .null) state.expires = try w.unsigned(u64, get(auth, "expires_at_ms"));
                }
            },
            .user, .polybolt, .rfq => {
                const creds = .{ .apiKey = try credential(clob, "key"), .secret = try credential(clob, "secret"), .passphrase = try credential(clob, "passphrase") };
                switch (service) {
                    .user => {
                        const markets = if (get(p, "markets") == .null) w.V{ .array = .init(a) } else get(p, "markets");
                        if (markets != .array) return error.InvalidSubscription;
                        state.subscription = try encode(a, .{ .type = "user", .markets = markets, .auth = creds }, .{});
                    },
                    .polybolt => {
                        state.login = try encode(a, .{ .op = "auth", .auth = creds }, .{});
                        state.subscription = try encode(a, .{ .op = "subscribe", .subscriptions = .{p} }, .{});
                    },
                    .rfq => {
                        state.login = try commandLogin(a, config, false, io);
                    },
                    else => unreachable,
                }
            },
        }
        return state;
    }
    pub fn endpoint(self: State) []const u8 {
        return switch (self.service) {
            .market => "wss://ws-subscriptions-clob.polymarket.com/ws/market",
            .user => "wss://ws-subscriptions-clob.polymarket.com/ws/user",
            .sports => "wss://sports-api.polymarket.com/ws",
            .polybolt => "wss://ws-live-v2.polymarket.com/ws",
            .rtds => "wss://ws-live-data.polymarket.com",
            .rfq => "wss://combos-rfq-gateway-quoter.polymarket.com/ws/rfq",
            .perps => "wss://ws.perpetuals.polymarket.com/v1/ws",
        };
    }
    pub fn heartbeat(self: State) ?[]const u8 {
        return switch (self.service) {
            .market, .user, .sports, .rtds => "PING",
            .perps => "{\"req\":\"post\",\"op\":{\"type\":\"ping\"}}",
            else => null,
        };
    }
    pub fn start(self: *State, io: std.Io) ![]const u8 {
        if (self.expires) |ms| if (std.Io.Timestamp.now(io, .real).toMilliseconds() >= ms) return error.AuthenticationRequired;
        self.authenticating = self.login != null;
        self.snapshot = false;
        self.seq = null;
        return self.login orelse self.subscription;
    }
    pub fn event(self: *State, a: w.A, raw: []const u8) !Event {
        if (std.ascii.eqlIgnoreCase(raw, "PING") or std.ascii.eqlIgnoreCase(raw, "PONG")) return if (self.authenticating) .skip else .{ .deliver = try encode(a, .{ .type = raw }, .{}) };
        const v = try w.parse(a, raw);
        if (self.authenticating) {
            switch (self.service) {
                .polybolt => {
                    if (is(v, "op", "error")) return error.AuthenticationRequired;
                    if (!is(v, "op", "authed")) return .skip;
                },
                .perps => {
                    const id = get(v, "id");
                    if (id != .number_string or !std.mem.eql(u8, id.number_string, "0")) return .skip;
                    if (!is(get(v, "data"), "status", "ok")) return error.AuthenticationRequired;
                },
                .rfq => {
                    if (!is(v, "type", "auth")) return .skip;
                    if (!flag(v, "success")) return error.AuthenticationRequired;
                },
                else => unreachable,
            }
            self.authenticating = false;
            return if (self.subscription.len != 0) .subscribe else .skip;
        }
        if (is(v, "op", "error") or is(v, "event_type", "error") or is(v, "type", "error")) return error.ExchangeApi;
        if (self.service == .polybolt) {
            if (get(v, "dropped") != .null and try w.unsigned(u64, get(v, "dropped")) > 0) return error.SequenceGap;
            if (get(v, "seq") != .null) {
                const seq = try w.unsigned(u64, get(v, "seq"));
                if (self.seq) |last| if (seq != last +| 1) return error.SequenceGap;
                self.seq = seq;
            }
        }
        if (!self.snapshot and (self.service == .market or self.service == .polybolt)) {
            self.snapshot = book(v);
            if (v == .array) for (v.array.items) |item| {
                if (book(item)) self.snapshot = true;
            };
            if (!self.snapshot and get(v, "op") == .null and get(v, "error") == .null) return .skip;
        }
        return .{ .deliver = raw };
    }
};

pub fn commandLogin(a: w.A, config: w.V, perps: bool, io: std.Io) ![]const u8 {
    const clob = get(config, "clob");
    const auth = get(config, "auth");
    if (perps) {
        var headers = w.V{ .object = .empty };
        try @import("../../core/venue_auth.zig").proxyHeaders(a, &headers, auth, @intCast(std.Io.Timestamp.now(io, .real).toMilliseconds()));
        return try encode(a, .{ .id = @as(u8, 0), .req = "post", .op = .{ .type = "auth", .args = .{ .proxy = get(headers, "POLYMARKET-PROXY"), .secret = get(headers, "POLYMARKET-SECRET") } } }, .{});
    }
    const creds = .{ .apiKey = try credential(clob, "key"), .secret = try credential(clob, "secret"), .passphrase = try credential(clob, "passphrase") };
    const checksum = @import("clob_auth.zig").checksum;
    const signer = try checksum(a, try credential(clob, "address"));
    const maker = if (get(auth, "address") != .null) try checksum(a, try credential(auth, "address")) else signer;
    const same = std.mem.eql(u8, signer, maker);
    const mode: u8 = if (get(auth, "rfq_signature_type") != .null) try w.unsigned(u8, get(auth, "rfq_signature_type")) else if (same) 0 else 3;
    if (mode > 3 or (!same and mode == 0)) return error.AuthenticationRequired;
    return try encode(a, .{ .type = "auth", .auth = creds, .identity = .{ .signer_address = signer, .maker_address = maker, .signature_type = mode } }, .{});
}
