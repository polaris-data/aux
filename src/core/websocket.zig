// One market-data subscription per connection. Only the subscription
// is replayed after a disconnect. The SDK serializes calls to next().
const host = @import("host_stream.zig");
const std = @import("std");
const wire = @import("wire.zig");
const gpa = std.heap.c_allocator;
const eq = std.mem.eql;
const max_bytes = 8 * 1024 * 1024;

fn headerToken(list: []const u8, wanted: []const u8) bool {
    var it = std.mem.splitScalar(u8, list, ',');
    while (it.next()) |part| if (std.ascii.eqlIgnoreCase(std.mem.trim(u8, part, " \t"), wanted)) return true;
    return false;
}

fn headerValid(head: std.http.Client.Response.Head, key: []const u8) bool {
    if (head.status != .switching_protocols) return false;
    var hash = std.crypto.hash.Sha1.init(.{});
    hash.update(key);
    hash.update("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    var encoded: [28]u8 = undefined;
    const accept = std.base64.standard.Encoder.encode(&encoded, &hash.finalResult());
    var upgrade = false;
    var connection = false;
    var matched = false;
    var it = head.iterateHeaders();
    while (it.next()) |h| {
        if (std.ascii.eqlIgnoreCase(h.name, "upgrade")) upgrade = headerToken(h.value, "websocket");
        if (std.ascii.eqlIgnoreCase(h.name, "connection")) connection = headerToken(h.value, "upgrade");
        if (std.ascii.eqlIgnoreCase(h.name, "sec-websocket-accept")) {
            if (matched or !eq(u8, h.value, accept)) return false;
            matched = true;
        }
        if (std.ascii.eqlIgnoreCase(h.name, "sec-websocket-extensions") or std.ascii.eqlIgnoreCase(h.name, "sec-websocket-protocol")) return false;
    }
    return upgrade and connection and matched;
}

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
        if (eq(u8, venue, "arcus")) {
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
            self.subscribe = try std.json.Stringify.valueAlloc(a, .{ .jsonrpc = "2.0", .id = @as(u32, 1), .method = "subscribe", .params = .{ .channel = self.channel } }, .{});
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
        } else if (self.arcus == null and self.chainlink == null and !is_derive and self.paradex == null and self.prediction == null and self.polymarket == null and self.hyperliquid == null and self.lighter == null and self.monaco == null) {
            self.qfex = try @import("../exchanges/qfex/stream.zig").State.init(a, params);
            self.subscribe = self.qfex.?.frame;
        }
        const endpoint = if (self.chainlink) |c| c.endpoint else if (self.monaco) |m| m.endpoint else if (self.extended) |state| state.endpoint else if (args.object.get("origin")) |v| try wire.string(v) else if (self.arcus != null) "wss://api.arcus.xyz/v1/ws" else if (self.hyperliquid) |state| state.endpoint else if (self.lighter) |state| state.endpoint else if (self.polymarket) |state| state.endpoint() else if (self.prediction) |state| state.endpoint() else if (self.paradex != null) "wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=3" else if (is_derive) "wss://api.derive.xyz/v3/ws" else if (self.phoenix != null) "wss://perp-api.phoenix.trade/v1/ws" else "wss://mds.qfex.com/";
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
        if (self.canceled.load(.acquire)) return error.Network;
        try self.write(self.conn orelse return error.Network, 1, payload);
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
    fn openEndpoint(self: *Session, endpoint: []const u8) !void {
        for (endpoint) |b| if (b <= 32 or b == 127 or b == '\\') return error.InvalidEndpoint;
        self.url = try std.Uri.parse(endpoint);
        if (self.url.host == null or self.url.user != null or self.url.password != null or self.url.fragment != null) return error.InvalidEndpoint;
        self.url.scheme = if (eq(u8, self.url.scheme, "wss")) "https" else if (eq(u8, self.url.scheme, "ws")) "http" else return error.InvalidEndpoint;
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
        var req = try host.request(&self.client, .GET, self.url, .{ .redirect_behavior = .unhandled, .headers = .{ .connection = .{ .override = "Upgrade" }, .user_agent = if (self.extended != null) .omit else .default }, .extra_headers = headers.items });
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
        if (!headerValid(response.head, key)) return error.InvalidHandshake;
        const first = if (self.monaco) |*state| try state.start(a, io) else if (self.lighter) |*state| try state.start(io) else if (self.polymarket) |*state| try state.start(io) else if (self.paradex) |*state| try state.start(scratch.allocator(), io, self.subscribe) else if (self.derive) |*state| try state.start(scratch.allocator(), io, self.subscribe) else self.subscribe;
        if (self.multiplex) {
            for (self.topics.items) |topic| try self.write(conn, 1, topic.frame);
        } else if (first.len > 0) try self.write(conn, 1, first);
        const beat: ?[]const u8 = if (self.monaco != null) "{\"type\":\"Ping\"}" else if (self.polymarket) |state| state.heartbeat() else if (self.hyperliquid != null) "{\"method\":\"ping\"}" else null;
        if (beat) |payload| {
            self.heartbeat_stop = .unset;
            self.heartbeat_thread = try std.Thread.spawn(.{}, heartbeat, .{ self, conn, payload });
        }
        self.generation += 1;
        self.conn = conn;
        if (self.phoenix) |*state| state.reset();
        req.connection = null;
    }
    fn heartbeat(self: *Session, conn: *std.http.Client.Connection, payload: []const u8) void {
        const io = self.io.io();
        while (!self.canceled.load(.acquire)) {
            self.heartbeat_stop.waitTimeout(io, .{ .duration = .{ .clock = .awake, .raw = .fromSeconds(10) } }) catch |e| switch (e) {
                error.Timeout => {
                    self.write(conn, 1, payload) catch {
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
    fn disconnect(self: *Session) void {
        if (self.heartbeat_thread) |thread| {
            self.heartbeat_stop.set(self.io.io());
            if (self.conn) |conn| conn.stream_reader.stream.shutdown(self.io.io(), .both) catch {};
            thread.join();
            self.heartbeat_thread = null;
        }
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
        const conn = self.conn.?;
        const reader = conn.reader();
        var fragments: std.ArrayList(u8) = .empty;
        defer fragments.deinit(gpa);
        var fragmented = false;
        while (true) {
            const first = reader.takeByte() catch return error.Network;
            const size = reader.takeByte() catch return error.Network;
            const op = first & 15;
            const fin = first & 128 != 0;
            if (first & 0x70 != 0 or size & 128 != 0) return error.InvalidFrame;
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
                if (fragmented or (op != 1 and !(op == 2 and (self.paradex != null or self.bearer != null)))) return error.InvalidOpcode;
                self.binary = op == 2;
            }
            if (n > max_bytes - fragments.items.len) return error.FrameTooLarge;
            const dest = try fragments.addManyAsSlice(gpa, @intCast(n));
            reader.readSliceAll(dest) catch return error.Network;
            if (fin) return try fragments.toOwnedSlice(gpa);
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
            const raw = self.message() catch |err| {
                if (err != error.Network) return err;
                self.disconnectSubscription();
                continue;
            } orelse return null;
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
            const raw = try self.subscriptionMessage() orelse break;
            var arena = std.heap.ArenaAllocator.init(gpa);
            defer arena.deinit();
            if (self.qfex != null or self.arcus != null or self.chainlink != null or self.monaco != null or self.hyperliquid != null or self.lighter != null or self.polymarket != null or self.derive != null or self.paradex != null) {
                const parsed = if (self.qfex) |*s| s.event(arena.allocator(), raw) else if (self.arcus) |*s| s.event(arena.allocator(), raw) else if (self.chainlink) |s| s.event(arena.allocator(), raw) else if (self.monaco) |*state| state.event(arena.allocator(), raw) else if (self.hyperliquid) |*state| state.event(arena.allocator(), raw) else if (self.lighter) |*state| state.event(arena.allocator(), raw) else if (self.polymarket) |*state| state.event(arena.allocator(), raw) else if (self.paradex) |*state| state.event(arena.allocator(), raw, self.binary) else self.derive.?.event(arena.allocator(), self.channel, raw);
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
                        try self.write(self.conn.?, 1, msg);
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
