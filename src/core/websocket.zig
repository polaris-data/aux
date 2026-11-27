// One market-data subscription per connection. Only the subscription
// is replayed after a disconnect. The SDK serializes calls to next().
const host = @import("host_stream.zig");
const exchange_stream = @import("exchange_stream.zig");
const perps_stream = @import("perps_stream.zig");
const std = @import("std");
const wire = @import("wire.zig");
const gpa = std.heap.c_allocator;
const eq = std.mem.eql;
const max_bytes = 8 * 1024 * 1024;

pub const MessageKind = enum { text, binary };
pub const PreparedSubscription = struct {
    endpoint: []const u8,
    subscribe: []const []const u8,
    unsubscribe: []const []const u8,
    headers: []const std.http.Header = &.{},
};
pub const SubscriptionFrames = struct {
    subscribe: []const u8,
    unsubscribe: []const u8,
};

fn headerToken(list: []const u8, wanted: []const u8) bool {
    var it = std.mem.splitScalar(u8, list, ',');
    while (it.next()) |part| if (std.ascii.eqlIgnoreCase(std.mem.trim(u8, part, " \t"), wanted)) return true;
    return false;
}

fn headerValid(head: std.http.Client.Response.Head, key: []const u8, inflate: ?*Deflate, protocol: ?[]const u8) bool {
    if (head.status != .switching_protocols) return false;
    var hash = std.crypto.hash.Sha1.init(.{});
    hash.update(key);
    hash.update("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    var encoded: [28]u8 = undefined;
    const accept = std.base64.standard.Encoder.encode(&encoded, &hash.finalResult());
    var upgrade = false;
    var connection = false;
    var matched = false;
    var compression = false;
    var selected_protocol = false;
    var it = head.iterateHeaders();
    while (it.next()) |h| {
        if (std.ascii.eqlIgnoreCase(h.name, "upgrade")) upgrade = headerToken(h.value, "websocket");
        if (std.ascii.eqlIgnoreCase(h.name, "connection")) connection = headerToken(h.value, "upgrade");
        if (std.ascii.eqlIgnoreCase(h.name, "sec-websocket-accept")) {
            if (matched or !eq(u8, h.value, accept)) return false;
            matched = true;
        }
        if (std.ascii.eqlIgnoreCase(h.name, "sec-websocket-extensions")) {
            const state = inflate orelse return false;
            if (compression) return false;
            var parts = std.mem.splitScalar(u8, h.value, ';');
            if (!eq(u8, std.mem.trim(u8, parts.first(), " \t"), "permessage-deflate")) return false;
            var server_no_takeover = false;
            var client_no_takeover = false;
            while (parts.next()) |part| {
                const parameter = std.mem.trim(u8, part, " \t");
                if (eq(u8, parameter, "server_no_context_takeover") and !server_no_takeover) {
                    server_no_takeover = true;
                } else if (eq(u8, parameter, "client_no_context_takeover") and !client_no_takeover) {
                    client_no_takeover = true;
                } else return false;
            }
            state.reset(server_no_takeover);
            compression = true;
        }
        if (std.ascii.eqlIgnoreCase(h.name, "sec-websocket-protocol")) {
            if (selected_protocol or !eq(u8, h.value, protocol orelse return false)) return false;
            selected_protocol = true;
        }
    }
    return upgrade and connection and matched and (inflate == null or compression) and (protocol == null or selected_protocol);
}

const Deflate = @import("websocket_deflate.zig").Inflate;
const Heartbeat = exchange_stream.Heartbeat;

const Topic = struct { frame: []u8, references: usize };

pub const Session = struct {
    arena: std.heap.ArenaAllocator,
    io: host.Runtime,
    client: std.http.Client,
    url: std.Uri,
    subscribe: []const u8,
    channel: []const u8,
    qfex: ?@import("../exchanges/qfex/stream.zig").State = null,
    api_key: ?[]const u8 = null,
    credential_expiry: ?u64 = null,
    binary: bool = false,
    perps: ?perps_stream.State = null,
    exchange: ?exchange_stream.State = null,
    exchange_heartbeat: ?[]u8 = null,
    frame_remainder: ?[]u8 = null,
    frame_cursor: usize = 0,
    opening_headers: []const std.http.Header = &.{},
    bearer: ?[]const u8 = null,
    paradex: ?@import("../exchanges/paradex/stream.zig").State = null,
    derive: ?@import("../exchanges/derive/stream.zig").State = null,
    extended: ?@import("../exchanges/extended/stream.zig").State = null,
    prediction: ?@import("prediction_stream.zig").State = null,
    arcus: ?@import("../exchanges/arcus/stream.zig").State = null,
    chainlink: ?@import("../data/chainlink/stream.zig").State = null,
    monaco: ?@import("../exchanges/monaco/stream.zig").State = null,
    hyperliquid: ?@import("../exchanges/hyperliquid/stream.zig").State = null,
    lighter: ?@import("../exchanges/lighter/stream.zig").State = null,
    nado: ?@import("../exchanges/nado/stream.zig").State = null,
    ondo: ?@import("../exchanges/ondo_perps/stream.zig").State = null,
    pacifica: ?@import("../exchanges/pacifica/stream.zig").State = null,
    aster: ?@import("../exchanges/aster/stream.zig").State = null,
    deflate: ?Deflate = null,
    polymarket: ?@import("../exchanges/polymarket/stream.zig").State = null,
    multiplex: bool = false,
    topics: std.ArrayList(Topic) = .empty,
    topic_lock: std.Io.Mutex = .init,
    write_lock: std.Io.Mutex = .init,
    heartbeat_stop: std.Io.Event = .unset,
    heartbeat_thread: ?std.Thread = null,
    phoenix: ?@import("../exchanges/phoenix/stream.zig").State = null,
    conn: ?*std.http.Client.Connection = null,
    socket_lock: std.Io.Mutex = .init,
    socket: ?std.Io.net.Stream = null,
    canceled: std.atomic.Value(bool) = .init(false),
    ended: bool = false,
    retries: u8 = 0,
    generation: u64 = 0,
    venue: []const u8 = "",
    rejection: ?[]u8 = null,

    pub fn open(config: []const u8, dial: ?host.Dial) !*Session {
        const self = try gpa.create(Session);
        errdefer gpa.destroy(self);
        self.* = .{ .arena = .init(gpa), .io = .{ .dial = dial }, .client = undefined, .url = undefined, .subscribe = undefined, .channel = undefined };
        errdefer self.arena.deinit();
        const a = self.arena.allocator();
        const args = try wire.parse(a, try a.dupe(u8, config));
        const params = try wire.field(args, "params");
        const venue = try wire.string(try wire.field(args, "venue"));
        self.venue = venue;
        const is_derive = eq(u8, venue, "derive");
        errdefer if (self.monaco) |*state| state.deinit();
        errdefer if (self.aster) |*state| state.deinit(dial);
        errdefer if (self.perps) |*state| state.deinit();
        if (exchange_stream.venue(venue)) |kind| {
            self.exchange = try exchange_stream.State.init(a, kind, args, params);
            self.channel = self.exchange.?.channel;
            self.subscribe = self.exchange.?.frame;
        } else if (perps_stream.venue(venue)) |kind| {
            self.perps = try perps_stream.State.init(a, kind, args, params);
            self.channel = self.perps.?.channel;
            self.subscribe = self.perps.?.request;
        } else if (eq(u8, venue, "arcus")) {
            self.arcus = try @import("../exchanges/arcus/stream.zig").State.init(a, params);
            self.channel = @tagName(self.arcus.?.channel);
            self.subscribe = self.arcus.?.frame;
        } else if (eq(u8, venue, "chainlink_streams")) {
            self.chainlink = try @import("../data/chainlink/stream.zig").State.init(a, args, params);
            self.channel = "reports";
            self.subscribe = "";
        } else if (eq(u8, venue, "monaco")) {
            self.monaco = try @import("../exchanges/monaco/stream.zig").State.init(a, args, params, self);
            self.channel = self.monaco.?.channel;
            self.subscribe = self.monaco.?.frame;
        } else if (eq(u8, venue, "nado")) {
            self.nado = try @import("../exchanges/nado/stream.zig").State.init(a, args, params);
            self.deflate = .{ .history = try a.alloc(u8, 32 * 1024) };
            self.channel = @tagName(self.nado.?.channel);
            self.subscribe = self.nado.?.frame;
        } else if (eq(u8, venue, "ondo_perps")) {
            self.ondo = try @import("../exchanges/ondo_perps/stream.zig").State.init(a, args, params);
            self.channel = @tagName(self.ondo.?.channel);
            self.subscribe = self.ondo.?.frame;
        } else if (eq(u8, venue, "pacifica")) {
            self.pacifica = try @import("../exchanges/pacifica/stream.zig").State.init(a, args, params);
            self.channel = @tagName(self.pacifica.?.channel);
            self.subscribe = self.pacifica.?.frame;
        } else if (eq(u8, venue, "aster")) {
            self.aster = try @import("../exchanges/aster/stream.zig").State.init(a, args, params);
            self.channel = @tagName(self.aster.?.channel);
            self.subscribe = "";
        } else if (eq(u8, venue, "paragon") or eq(u8, venue, "trade_xyz")) {
            self.hyperliquid = try @import("../exchanges/hyperliquid/stream.zig").State.scoped(a, args, params, if (eq(u8, venue, "paragon")) "para" else "xyz");
            self.subscribe = self.hyperliquid.?.frame;
            self.channel = @tagName(self.hyperliquid.?.channel);
        } else if (eq(u8, venue, "hyperliquid")) {
            self.hyperliquid = try @import("../exchanges/hyperliquid/stream.zig").State.init(a, args, params);
            self.subscribe = self.hyperliquid.?.frame;
            self.channel = @tagName(self.hyperliquid.?.channel);
        } else if (eq(u8, venue, "lighter")) {
            self.lighter = try @import("../exchanges/lighter/stream.zig").State.init(a, args, params, self.io.io());
            self.subscribe = self.lighter.?.frame;
            self.channel = self.lighter.?.incoming;
        } else if (eq(u8, venue, "polymarket") or eq(u8, venue, "polymarket_perps")) {
            self.polymarket = try @import("../exchanges/polymarket/stream.zig").State.init(a, args, params, self.io.io());
            self.channel = @tagName(self.polymarket.?.service);
            self.subscribe = self.polymarket.?.subscription;
        } else if (eq(u8, venue, "kalshi") or eq(u8, venue, "pascal")) {
            self.prediction = try @import("prediction_stream.zig").State.init(a, venue, args, params);
            self.channel = self.prediction.?.channel;
            self.subscribe = self.prediction.?.frame;
        } else if (eq(u8, venue, "paradex")) {
            self.paradex = try @import("../exchanges/paradex/stream.zig").State.init(a, params, args.object.get("auth") orelse .null);
            self.channel = self.paradex.?.name;
            self.subscribe = try @import("../exchanges/paradex/stream.zig").subscriptionFrame(a, "subscribe", 1, self.channel);
        } else if (is_derive) {
            const spec = try @import("../exchanges/derive/stream.zig").channel(a, params);
            self.channel = spec.name;
            self.derive = if (spec.private) .{ .owner = try wire.string(args.object.get("api_key") orelse return error.AuthenticationRequired), .secret = try wire.string(args.object.get("api_secret") orelse return error.AuthenticationRequired) } else .{};
            self.subscribe = try std.json.Stringify.valueAlloc(a, .{ .id = @as(u32, 1), .method = "subscribe", .params = .{ .channels = .{self.channel} } }, .{});
        } else self.channel = try wire.string(try wire.field(params, "channel"));
        if (eq(u8, venue, "phoenix") or eq(u8, venue, "extended")) {
            const credentials = try @import("venue_auth.zig").HeaderCredentials.parse(a, args);
            self.api_key = credentials.key;
            self.bearer = credentials.bearer_token;
            self.credential_expiry = credentials.expires;
            const authenticated = self.api_key != null or self.bearer != null;
            if (eq(u8, venue, "extended")) {
                self.extended = try @import("../exchanges/extended/stream.zig").State.init(a, args, params, authenticated);
                self.subscribe = self.extended.?.frame;
            } else {
                self.phoenix = try @import("../exchanges/phoenix/stream.zig").State.init(params, authenticated);
                self.subscribe = try std.json.Stringify.valueAlloc(a, .{ .type = "subscribe", .subscription = params }, .{});
            }
        } else if (self.exchange == null and self.perps == null and self.arcus == null and self.chainlink == null and !is_derive and self.paradex == null and self.prediction == null and self.polymarket == null and self.hyperliquid == null and self.lighter == null and self.monaco == null and self.nado == null and self.ondo == null and self.pacifica == null and self.aster == null) {
            self.qfex = try @import("../exchanges/qfex/stream.zig").State.init(a, params);
            self.subscribe = self.qfex.?.frame;
        }
        const endpoint = if (self.exchange) |state| state.endpoint else if (self.aster) |state| state.endpoint else if (self.chainlink) |c| c.endpoint else if (self.monaco) |m| m.endpoint else if (self.extended) |state| state.endpoint else if (args.object.get("origin")) |v| try wire.string(v) else if (self.perps) |state| state.endpoint else if (self.nado) |state| state.endpoint else if (self.ondo) |state| state.endpoint else if (self.pacifica) |state| state.endpoint else if (self.arcus != null) "wss://api.arcus.xyz/v1/ws" else if (self.hyperliquid) |state| state.endpoint else if (self.lighter) |state| state.endpoint else if (self.polymarket) |state| state.endpoint() else if (self.prediction) |state| state.endpoint() else if (self.paradex != null) @import("../exchanges/paradex/stream.zig").endpoint else if (is_derive) "wss://api.derive.xyz/v3/ws" else if (self.phoenix != null) "wss://perp-api.phoenix.trade/v1/ws" else @import("../exchanges/qfex/stream.zig").endpoint;
        try self.openEndpoint(endpoint);
        return self;
    }

    /// A dedicated socket with no automatic subscription or reconnect.
    pub fn dedicated(endpoint: []const u8, dial: ?host.Dial) !*Session {
        return bare(endpoint, null, "", &.{}, dial);
    }
    pub fn dedicatedHeaders(endpoint: []const u8, headers: []const std.http.Header, dial: ?host.Dial) !*Session {
        return bare(endpoint, null, "", headers, dial);
    }
    pub fn pyth(endpoint: []const u8, token: []const u8, frame: []const u8, dial: ?host.Dial) !*Session {
        return bare(endpoint, token, frame, &.{}, dial);
    }
    fn bare(endpoint: []const u8, token: ?[]const u8, frame: []const u8, headers: []const std.http.Header, dial: ?host.Dial) !*Session {
        const self = try gpa.create(Session);
        errdefer gpa.destroy(self);
        self.* = .{ .arena = .init(gpa), .io = .{ .dial = dial }, .client = undefined, .url = undefined, .subscribe = "", .channel = "" };
        errdefer self.arena.deinit();
        const a = self.arena.allocator();
        if (token) |t| self.bearer = try std.mem.concat(a, u8, &.{ "Bearer ", t });
        self.subscribe = try a.dupe(u8, frame);
        self.opening_headers = headers;
        try self.openEndpoint(try a.dupe(u8, endpoint));
        self.opening_headers = &.{};
        return self;
    }
    pub fn sendText(self: *Session, payload: []const u8) !void {
        try self.sendData(.text, payload);
    }
    pub fn sendData(self: *Session, kind: MessageKind, payload: []const u8) !void {
        if (self.canceled.load(.acquire)) return error.Network;
        try self.write(self.conn orelse return error.Network, switch (kind) {
            .text => 1,
            .binary => 2,
        }, payload);
    }
    /// Called before the reader thread starts. The initial subscription was sent on upgrade.
    pub fn startMultiplex(self: *Session, frame: []const u8) !void {
        const saved = try gpa.dupe(u8, frame);
        errdefer gpa.free(saved);
        try self.topics.append(gpa, .{ .frame = saved, .references = 1 });
        self.multiplex = true;
    }
    pub fn joinTopic(self: *Session, frame: []const u8) !void {
        const io = self.io.io();
        self.topic_lock.lockUncancelable(io);
        defer self.topic_lock.unlock(io);
        for (self.topics.items) |*topic| if (eq(u8, topic.frame, frame)) {
            topic.references += 1;
            return;
        };
        try self.topics.ensureUnusedCapacity(gpa, 1);
        const saved = try gpa.dupe(u8, frame);
        errdefer gpa.free(saved);
        if (self.conn != null) try self.sendText(frame);
        self.topics.appendAssumeCapacity(.{ .frame = saved, .references = 1 });
    }
    pub fn leaveTopic(self: *Session, frame: []const u8, unsubscribe: []const u8) void {
        const io = self.io.io();
        self.topic_lock.lockUncancelable(io);
        defer self.topic_lock.unlock(io);
        for (self.topics.items, 0..) |*topic, i| if (eq(u8, topic.frame, frame)) {
            topic.references -= 1;
            if (topic.references == 0) {
                gpa.free(self.topics.swapRemove(i).frame);
                if (self.conn != null) self.sendText(unsubscribe) catch {};
            }
            return;
        };
    }
    fn reconnectSubscription(self: *Session) !void {
        self.topic_lock.lockUncancelable(self.io.io());
        defer self.topic_lock.unlock(self.io.io());
        try self.connect();
    }
    fn disconnectSubscription(self: *Session) void {
        self.topic_lock.lockUncancelable(self.io.io());
        defer self.topic_lock.unlock(self.io.io());
        self.disconnect();
    }
    pub fn parseEndpoint(endpoint: []const u8) !std.Uri {
        for (endpoint) |b| if (b <= 32 or b == 127 or b == '\\') return error.InvalidEndpoint;
        var url = try std.Uri.parse(endpoint);
        if (url.host == null or url.user != null or url.password != null or url.fragment != null) return error.InvalidEndpoint;
        url.scheme = if (eq(u8, url.scheme, "wss")) "https" else if (eq(u8, url.scheme, "ws")) "http" else return error.InvalidEndpoint;
        return url;
    }
    fn openEndpoint(self: *Session, endpoint: []const u8) !void {
        self.url = try parseEndpoint(endpoint);
        self.client = .{ .io = self.io.io(), .allocator = gpa };
        errdefer self.client.deinit();
        try self.connect();
    }

    fn connect(self: *Session) !void {
        const io = self.io.io();
        try @import("venue_auth.zig").HeaderCredentials.checkExpiry(self.credential_expiry, io);
        var random_key: [16]u8 = undefined;
        try io.randomSecure(&random_key);
        var encoded: [24]u8 = undefined;
        const key = std.base64.standard.Encoder.encode(&encoded, &random_key);
        var scratch = std.heap.ArenaAllocator.init(gpa);
        defer scratch.deinit();
        const a = scratch.allocator();
        var headers: std.ArrayList(std.http.Header) = .empty;
        try headers.appendSlice(a, &.{ .{ .name = "Upgrade", .value = "websocket" }, .{ .name = "Sec-WebSocket-Version", .value = "13" }, .{ .name = "Sec-WebSocket-Key", .value = key } });
        if (self.extended) |*state| {
            state.snapshot = false;
            state.sequence = null;
            try headers.append(a, .{ .name = "User-Agent", .value = "aux-zig/0.0.1" });
        }
        if (self.arcus) |*s| s.snapshot = false;
        if (self.chainlink) |s| try s.headers(a, io, &headers);
        try headers.appendSlice(a, self.opening_headers);
        if (self.deflate != null) try headers.append(a, .{ .name = "Sec-WebSocket-Extensions", .value = "permessage-deflate; server_no_context_takeover; client_no_context_takeover" });
        if (self.bearer) |t| try headers.append(a, .{ .name = "Authorization", .value = t });
        if (self.api_key) |k| try headers.append(a, .{ .name = "X-Api-Key", .value = k });
        if (self.prediction) |*state| {
            state.reset();
            if (state.key) |k| {
                const ms = std.Io.Timestamp.now(io, .real).toMilliseconds();
                if (ms < 0) return error.InvalidTimestamp;
                const path = switch (self.url.path) {
                    .raw, .percent_encoded => |s| s,
                };
                var fields = try @import("auth.zig").kalshiHeaders(a, k, state.seed.?, "GET", if (path.len == 0) "/" else path, @intCast(ms));
                var it = fields.object.iterator();
                while (it.next()) |h| try headers.append(a, .{ .name = h.key_ptr.*, .value = try wire.string(h.value_ptr.*) });
            }
        }
        const opening = if (self.exchange) |*state| try state.start(a, io, self.io.dial, self.url, &headers) else if (self.perps) |*state| try state.start(a, io, self.io.dial, self.url, &headers) else perps_stream.Opening{ .frame = self.subscribe };
        const url = if (opening.url) |custom| custom else if (self.aster) |*state| if (state.hasListenKey()) try parseEndpoint(try state.connect(a, self.io.dial)) else self.url else self.url;
        const has_user_agent = for (self.opening_headers) |header| {
            if (std.ascii.eqlIgnoreCase(header.name, "user-agent")) break true;
        } else false;
        var req = try host.request(&self.client, .GET, url, .{ .redirect_behavior = .unhandled, .headers = .{ .connection = .{ .override = "Upgrade" }, .user_agent = if (self.extended != null or has_user_agent) .omit else .default }, .extra_headers = headers.items });
        // We own the connection after 101. Avoid HTTP body draining on cleanup.
        defer {
            if (req.connection) |c| c.closing = true;
            req.deinit();
        }
        const conn = req.connection.?;
        self.socket_lock.lockUncancelable(io);
        if (self.canceled.load(.acquire)) {
            self.socket_lock.unlock(io);
            return error.Closed;
        }
        self.socket = conn.stream_reader.stream;
        self.socket_lock.unlock(io);
        errdefer self.clearSocket();
        req.sendBodiless() catch return error.Network;
        var response = req.receiveHead(&.{}) catch return error.Network;
        if (response.head.status != .switching_protocols and @backingInt(response.head.status) >= 300) {
            @import("http_failure.zig").recordStreamResponse(&response, headers.items);
            return error.ExchangeApi;
        }
        if (!headerValid(response.head, key, if (self.deflate) |*state| state else null, opening.protocol)) return error.InvalidHandshake;
        const first = if (self.monaco) |*state| try state.start(a, io) else if (self.lighter) |*state| try state.start(io) else if (self.polymarket) |*state| try state.start(io) else if (self.paradex) |*state| try state.start(scratch.allocator(), io, self.subscribe) else if (self.derive) |*state| try state.start(scratch.allocator(), io, self.subscribe) else if (self.nado) |*state| try state.start(a, io, self.io.dial) else if (self.ondo) |*state| state.start() else opening.frame;
        if (self.multiplex) {
            for (self.topics.items) |topic| try self.write(conn, 1, topic.frame);
        } else if (first.len > 0) try self.write(conn, 1, first);
        if (self.perps) |*state| try state.connected(a, self.io.dial);
        if (self.exchange != null and self.exchange.?.kind == .deribit) {
            try self.write(conn, 1, "{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"public/set_heartbeat\",\"params\":{\"interval\":30}}");
        }
        const beat: ?Heartbeat = if (self.exchange) |state| exchange_stream.heartbeat(state.kind) else if (self.perps != null and self.perps.?.kind == .perpl and self.perps.?.private)
            .{ .payload = "", .perpl = true, .seconds = 30 }
        else if (self.monaco != null)
            .{ .payload = "{\"type\":\"Ping\"}" }
        else if (self.polymarket) |state| (if (state.heartbeat()) |payload| .{ .payload = payload } else null) else if (self.hyperliquid != null or self.pacifica != null)
            .{ .payload = "{\"method\":\"ping\"}" }
        else if (self.nado != null)
            .{ .payload = "", .opcode = 9, .seconds = 20 }
        else if (self.ondo != null)
            .{ .payload = "{\"op\":\"ping\"}", .seconds = 30 }
        else if (self.aster != null and self.aster.?.hasListenKey())
            .{ .payload = "", .opcode = 0, .seconds = 30 * 60 }
        else
            null;
        if (beat) |policy| {
            try self.beginHeartbeat(conn, policy);
        }
        self.generation += 1;
        self.conn = conn;
        if (self.phoenix) |*state| state.reset();
        req.connection = null;
    }
    pub fn beginHeartbeat(self: *Session, conn: *std.http.Client.Connection, policy: Heartbeat) !void {
        if (self.heartbeat_thread != null) return;
        self.heartbeat_stop = .unset;
        self.heartbeat_thread = try std.Thread.spawn(.{}, heartbeat, .{ self, conn, policy });
    }

    fn heartbeat(self: *Session, conn: *std.http.Client.Connection, policy: Heartbeat) void {
        const io = self.io.io();
        while (!self.canceled.load(.acquire)) {
            self.heartbeat_stop.waitTimeout(io, .{ .duration = .{ .clock = .awake, .raw = .fromSeconds(policy.seconds) } }) catch |e| switch (e) {
                error.Timeout => {
                    const result = if (policy.perpl)
                        self.perplPing(conn)
                    else if (policy.apex_omni)
                        self.apexPing(conn)
                    else if (policy.opcode == 0)
                        self.aster.?.renew(self.io.dial)
                    else
                        self.write(conn, policy.opcode, policy.payload);
                    result catch {
                        conn.stream_reader.stream.shutdown(io, .both) catch {};
                        return;
                    };
                    continue;
                },
                else => return,
            };
            return;
        }
    }
    fn clearSocket(self: *Session) void {
        const io = self.io.io();
        self.socket_lock.lockUncancelable(io);
        defer self.socket_lock.unlock(io);
        self.socket = null;
    }
    fn perplPing(self: *Session, conn: *std.http.Client.Connection) !void {
        var buffer: [80]u8 = undefined;
        const time = try @import("perps.zig").epoch(self.io.io(), .ms);
        const text = try std.fmt.bufPrint(&buffer, "{{\"mt\":1,\"t\":{d}}}", .{time});
        try self.write(conn, 1, text);
    }

    fn apexPing(self: *Session, conn: *std.http.Client.Connection) !void {
        var buffer: [80]u8 = undefined;
        const text = try @import("../exchanges/apex_omni/stream.zig").heartbeat(&buffer, self.io.io(), "ping");
        try self.write(conn, 1, text);
    }

    fn katanaMessage(self: *Session) ![]u8 {
        const frame = self.frame_remainder.?;
        errdefer {
            gpa.free(frame);
            self.frame_remainder = null;
        }
        var arena = std.heap.ArenaAllocator.init(gpa);
        defer arena.deinit();
        const remaining = std.mem.trimStart(u8, frame[self.frame_cursor..], " \t\r\n");
        const event = try perps_stream.jsonPrefix(arena.allocator(), remaining);
        const rest = std.mem.trimStart(u8, remaining[event.len..], " \t\r\n");
        if (rest.len == 0 and event.len == frame.len) {
            self.frame_remainder = null;
            return frame;
        }
        const copy = try gpa.dupe(u8, event);
        self.frame_cursor = frame.len - rest.len;
        if (rest.len == 0) {
            gpa.free(frame);
            self.frame_remainder = null;
        }
        return copy;
    }

    fn disconnect(self: *Session) void {
        if (self.frame_remainder) |frame| gpa.free(frame);
        self.frame_remainder = null;
        if (self.perps) |*state| state.reset();
        if (self.heartbeat_thread) |thread| {
            self.heartbeat_stop.set(self.io.io());
            if (self.conn) |conn| conn.stream_reader.stream.shutdown(self.io.io(), .both) catch {};
            thread.join();
            self.heartbeat_thread = null;
        }
        if (self.exchange_heartbeat) |payload| gpa.free(payload);
        self.exchange_heartbeat = null;
        self.clearSocket();
        if (self.conn) |conn| {
            conn.closing = true;
            self.client.connection_pool.release(conn, self.io.io());
            self.conn = null;
        }
    }
    pub fn close(self: *Session) void {
        const io = self.io.io();
        self.socket_lock.lockUncancelable(io);
        defer self.socket_lock.unlock(io);
        if (!self.canceled.swap(true, .acq_rel)) if (self.socket) |s| s.shutdown(io, .both) catch {};
    }
    pub fn rejectionFrame(venue: []const u8, raw: []const u8) ?[]u8 {
        var arena = std.heap.ArenaAllocator.init(gpa);
        defer arena.deinit();
        const body = wire.parse(arena.allocator(), raw) catch return null;
        return std.json.Stringify.valueAlloc(gpa, .{ .venue = venue, .body = body }, .{}) catch null;
    }
    fn recordRejection(self: *Session, err: anyerror, raw: []const u8) void {
        if (err == error.ExchangeApi and self.rejection == null) self.rejection = rejectionFrame(self.venue, raw);
    }
    pub fn destroy(self: *Session) void {
        self.close();
        if (self.monaco) |*state| state.deinit();
        self.disconnect();
        if (self.aster) |*state| state.deinit(self.io.dial);
        if (self.perps) |*state| state.deinit();
        for (self.topics.items) |topic| gpa.free(topic.frame);
        self.topics.deinit(gpa);
        if (self.rejection) |payload| gpa.free(payload);
        self.client.deinit();
        self.arena.deinit();
        gpa.destroy(self);
    }

    fn write(self: *Session, conn: *std.http.Client.Connection, op: u8, payload: []const u8) !void {
        self.write_lock.lockUncancelable(self.io.io());
        defer self.write_lock.unlock(self.io.io());
        if (payload.len > max_bytes) return error.FrameTooLarge;
        const writer = conn.writer();
        writer.writeByte(0x80 | op) catch return error.Network;
        if (payload.len < 126) writer.writeByte(0x80 | @as(u8, @intCast(payload.len))) catch return error.Network else if (payload.len <= 65535) {
            writer.writeByte(0xfe) catch return error.Network;
            writer.writeInt(u16, @intCast(payload.len), .big) catch return error.Network;
        } else {
            writer.writeByte(0xff) catch return error.Network;
            writer.writeInt(u64, payload.len, .big) catch return error.Network;
        }
        var mask: [4]u8 = undefined;
        try self.io.io().randomSecure(&mask);
        writer.writeAll(&mask) catch return error.Network;
        var scratch: [1024]u8 = undefined;
        var pos: usize = 0;
        while (pos < payload.len) {
            const n = @min(scratch.len, payload.len - pos);
            for (payload[pos..][0..n], 0..) |b, i| scratch[i] = b ^ mask[(pos + i) % 4];
            writer.writeAll(scratch[0..n]) catch return error.Network;
            pos += n;
        }
        conn.flush() catch return error.Network;
    }

    pub fn message(self: *Session) !?[]u8 {
        return self.readMessage(gpa, false);
    }

    /// Caller-managed readers accept both application opcodes.
    pub fn rawMessage(self: *Session, allocator: std.mem.Allocator) !?[]u8 {
        return self.readMessage(allocator, true);
    }

    fn readMessage(self: *Session, allocator: std.mem.Allocator, allow_binary: bool) !?[]u8 {
        const conn = self.conn.?;
        const reader = conn.reader();
        var fragments: std.ArrayList(u8) = .empty;
        defer fragments.deinit(allocator);
        var fragmented = false;
        var compressed = false;
        while (true) {
            const first = reader.takeByte() catch return error.Network;
            const size = reader.takeByte() catch return error.Network;
            const op = first & 15;
            const fin = first & 128 != 0;
            const rsv1 = first & 0x40 != 0;
            if (first & 0x30 != 0 or size & 128 != 0 or (rsv1 and (op == 0 or op >= 8 or self.deflate == null))) return error.InvalidFrame;
            const n: u64 = switch (size) {
                126 => reader.takeInt(u16, .big) catch return error.Network,
                127 => reader.takeInt(u64, .big) catch return error.Network,
                else => size,
            };
            if ((size == 126 and n < 126) or (size == 127 and n < 65536) or n > max_bytes) return error.InvalidLength;
            if (op >= 8) {
                if (!fin or n > 125) return error.InvalidControl;
                var buf: [125]u8 = undefined;
                const data = buf[0..@intCast(n)];
                reader.readSliceAll(data) catch return error.Network;
                switch (op) {
                    8 => {
                        if (n == 1) return error.InvalidClose;
                        if (n >= 2) {
                            const code = std.mem.readInt(u16, data[0..2], .big);
                            if (self.polymarket != null) switch (code) {
                                4001, 1008 => return error.AuthenticationRequired,
                                4002, 4008 => return error.ExchangeApi,
                                else => {},
                            };
                            if (code < 1000 or code >= 5000 or (code >= 1016 and code < 3000) or code == 1004 or code == 1005 or code == 1006 or code == 1015 or !std.unicode.utf8ValidateSlice(data[2..])) return error.InvalidClose;
                        }
                        self.write(conn, 8, data) catch {};
                        return null;
                    },
                    9 => try self.write(conn, 10, data),
                    10 => {},
                    else => return error.InvalidOpcode,
                }
                continue;
            }
            if (op == 0) {
                if (!fragmented) return error.InvalidOpcode;
            } else {
                const binary_allowed = allow_binary or self.paradex != null or self.bearer != null or (self.exchange != null and self.exchange.?.kuru != null);
                if (fragmented or (op != 1 and !(op == 2 and binary_allowed))) return error.InvalidOpcode;
                self.binary = op == 2;
                compressed = rsv1;
            }
            if (n > max_bytes - fragments.items.len) return error.FrameTooLarge;
            const dest = try fragments.addManyAsSlice(allocator, @intCast(n));
            reader.readSliceAll(dest) catch return error.Network;
            if (fin) return if (compressed) try self.deflate.?.message(allocator, fragments.items, max_bytes) else try fragments.toOwnedSlice(allocator);
            fragmented = true;
        }
    }

    /// Retry only subscriptions. Dedicated trading sockets never enter this path.
    pub fn subscriptionMessage(self: *Session) !?[]u8 {
        while (!self.canceled.load(.acquire)) {
            if (self.conn == null) {
                if (self.retries >= 3) return error.Network;
                self.retries += 1;
                self.reconnectSubscription() catch |err| switch (err) {
                    error.Network => continue,
                    error.Closed => break,
                    else => return err,
                };
            }
            if (self.frame_remainder != null) return try self.katanaMessage();
            if (self.exchange) |state| if (state.book or !state.auto_reconnect) return self.message();
            const raw = self.message() catch |err| {
                if (err != error.Network) return err;
                self.disconnectSubscription();
                continue;
            } orelse {
                const new_venue = self.exchange != null or self.perps != null or self.nado != null or self.ondo != null or self.pacifica != null or self.aster != null or (self.hyperliquid != null and self.hyperliquid.?.namespace != null);
                if (!new_venue) return null;
                self.disconnectSubscription();
                continue;
            };
            if (self.perps != null and self.perps.?.kind == .katana) {
                self.frame_remainder = raw;
                self.frame_cursor = 0;
                return try self.katanaMessage();
            }
            if (self.multiplex and self.lighter != null) {
                var arena = std.heap.ArenaAllocator.init(gpa);
                defer arena.deinit();
                const value = wire.parse(arena.allocator(), raw) catch return raw;
                const tag = if (value == .object) value.object.get("type") else null;
                if (tag != null and tag.? == .string and eq(u8, tag.?.string, "ping")) {
                    gpa.free(raw);
                    self.sendText("{\"type\":\"pong\"}") catch {
                        self.disconnectSubscription();
                    };
                    continue;
                }
            }
            return raw;
        }
        return null;
    }
    pub fn next(self: *Session) !?[]u8 {
        if (self.ended) return null;
        var recoverable = false;
        errdefer self.ended = !recoverable;
        if (self.monaco) |*state| {
            if (!state.active()) return error.AuthenticationRequired;
            if (state.pending) |event| {
                state.pending = null;
                return event;
            }
        }
        while (!self.canceled.load(.acquire)) {
            if (self.perps) |*state| {
                if (state.ready) if (state.pending_snapshot) |snapshot| {
                    state.pending_snapshot = null;
                    return snapshot;
                };
            }
            const raw = try self.subscriptionMessage() orelse break;
            var arena = std.heap.ArenaAllocator.init(gpa);
            defer arena.deinit();
            if (self.exchange) |*state| {
                if (self.binary) {
                    defer gpa.free(raw);
                    const decoded = try @import("../exchanges/kuru/binary.zig").decode(arena.allocator(), state.kuru.?, raw) orelse continue;
                    return try gpa.dupe(u8, decoded);
                }
                const event = state.event(arena.allocator(), raw, self.io.io()) catch |err| {
                    self.recordRejection(err, raw);
                    gpa.free(raw);
                    return err;
                };
                if (event == .deliver or event == .deliver_final) {
                    self.retries = 0;
                    if (event == .deliver_final) {
                        self.ended = true;
                        self.disconnectSubscription();
                    }
                    return raw;
                }
                defer gpa.free(raw);
                switch (event) {
                    .skip => {},
                    .reply => |msg| try self.sendText(msg),
                    .subscribe => try self.sendText(state.frame),
                    .heartbeat => |payload| {
                        if (self.exchange_heartbeat == null) {
                            const saved = try gpa.dupe(u8, payload);
                            errdefer gpa.free(saved);
                            try self.beginHeartbeat(self.conn orelse return error.Network, .{ .payload = saved, .seconds = 4 });
                            self.exchange_heartbeat = saved;
                        }
                    },
                    .deliver, .deliver_final => unreachable,
                }
                continue;
            }
            if (self.perps != null or self.qfex != null or self.arcus != null or self.chainlink != null or self.monaco != null or self.hyperliquid != null or self.lighter != null or self.polymarket != null or self.derive != null or self.paradex != null or self.nado != null or self.ondo != null or self.pacifica != null or self.aster != null) {
                const parsed = if (self.perps) |*state| state.event(arena.allocator(), raw) else if (self.qfex) |*s| s.event(arena.allocator(), raw) else if (self.arcus) |*s| s.event(arena.allocator(), raw) else if (self.chainlink) |s| s.event(arena.allocator(), raw) else if (self.monaco) |*state| state.event(arena.allocator(), raw) else if (self.hyperliquid) |*state| state.event(arena.allocator(), raw) else if (self.lighter) |*state| state.event(arena.allocator(), raw) else if (self.polymarket) |*state| state.event(arena.allocator(), raw) else if (self.paradex) |*state| state.event(arena.allocator(), raw, self.binary) else if (self.nado) |*state| state.event(arena.allocator(), raw) else if (self.ondo) |*state| state.event(arena.allocator(), raw) else if (self.pacifica) |*state| state.event(arena.allocator(), raw) else if (self.aster) |*state| state.event(arena.allocator(), raw) else self.derive.?.event(arena.allocator(), self.channel, raw);
                const event = parsed catch |err| {
                    self.recordRejection(err, raw);
                    recoverable = err == error.RecoverableStreamGap;
                    gpa.free(raw);
                    if (err == error.SequenceGap) {
                        self.disconnect();
                        continue;
                    }
                    return err;
                };
                switch (event) {
                    .skip => gpa.free(raw),
                    .reply => |msg| {
                        defer gpa.free(raw);
                        self.write(self.conn.?, 1, msg) catch |err| {
                            if (err != error.Network) return err;
                            self.disconnect();
                        };
                    },
                    .subscribe => {
                        gpa.free(raw);
                        self.write(self.conn.?, 1, self.subscribe) catch |err| {
                            if (err != error.Network) return err;
                            self.disconnect();
                        };
                    },
                    .deliver => |event_bytes| {
                        self.retries = 0;
                        if (event_bytes.ptr == raw.ptr) return raw;
                        defer gpa.free(raw);
                        return try gpa.dupe(u8, event_bytes);
                    },
                }
                continue;
            }
            if (self.extended != null or self.phoenix != null or self.prediction != null) {
                const parsed = if (self.extended) |*state| state.process(arena.allocator(), raw) else if (self.phoenix) |*state| state.process(arena.allocator(), raw) else self.prediction.?.process(arena.allocator(), raw);
                const decoded = (parsed catch |err| {
                    self.recordRejection(err, raw);
                    gpa.free(raw);
                    if (err == error.SequenceGap) {
                        self.disconnect();
                        continue;
                    }
                    return err;
                }) orelse {
                    gpa.free(raw);
                    continue;
                };
                self.retries = 0;
                if (decoded.ptr == raw.ptr) return raw;
                defer gpa.free(raw);
                return try gpa.dupe(u8, decoded);
            }
            gpa.free(raw);
            return error.InvalidSubscription;
        }
        if (self.monaco) |*state| if (!state.active()) return error.AuthenticationRequired;
        self.ended = true;
        return null;
    }
};
