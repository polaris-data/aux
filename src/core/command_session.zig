// A command owns its ordered connection until the matching reply arrives.
// A timeout leaves the read outstanding so reconciliation never replays a write.
const std = @import("std");
const w = @import("wire.zig");
const Socket = @import("websocket.zig").Session;
const Dial = @import("host_stream.zig").Dial;
const A = w.A;
const V = w.V;
const gpa = std.heap.c_allocator;
const eq = std.mem.eql;
const Mode = enum { kalshi, kalshi_perps, pascal, paradex, derive, polymarket, polymarket_perps, aevo, binance_spot, bybit, deribit, hibachi, kraken_spot, okx };
const Exchange = @import("exchange_rpc.zig");
const TradeVenue = @import("routes.zig").Venue;
const poly = @import("../exchanges/polymarket/rpc.zig");
const Pending = struct { id: u64, method: []const u8 };
fn text(v: V, key: []const u8) ![]const u8 {
    return w.string(try w.field(v, key));
}
fn optNum(v: V, key: []const u8, default: u32) !u32 {
    const n = if (v.object.get(key)) |f| try w.unsigned(u32, f) else default;
    if (n == 0) return error.InvalidParameter;
    return n;
}
fn now(io: std.Io) !u64 {
    const ms = std.Io.Timestamp.now(io, .real).toMilliseconds();
    return std.math.cast(u64, ms) orelse error.InvalidTimestamp;
}
pub const Session = struct {
    arena: std.heap.ArenaAllocator,
    ws: *Socket,
    mode: Mode,
    auth: V,
    timeout_ms: u32,
    capacity: u32,
    endpoint: []const u8,
    exchange: ?TradeVenue = null,
    exchange_token: ?[]const u8 = null,
    correlation: ?[]u8 = null,
    quote_id: ?[]u8 = null,
    next_id: u64 = 1,
    pending: ?Pending = null,
    // Commands are serialized. The output slot lives for the invoking command only.
    failure_out: ?*V = null,
    events: std.ArrayList([]u8) = .empty,
    worker: ?std.Thread = null,
    ready: std.Io.Event = .unset,
    result: anyerror!?[]u8 = null,
    fn isJsonRpc(self: *Session) bool {
        return self.mode == .derive or self.mode == .paradex;
    }
    pub fn open(a: A, input: V, dial: ?Dial) !*Session {
        const mode = std.meta.stringToEnum(Mode, try text(input, "venue")) orelse return error.InvalidOperation;
        const options = try w.field(input, "params");
        if (options != .object) return error.InvalidParameter;
        const timeout = try optNum(options, "timeout_ms", 10000);
        const capacity = try optNum(options, "capacity", 128);
        if (std.meta.stringToEnum(TradeVenue, @tagName(mode))) |venue| {
            if (Exchange.supports(venue)) return openExchange(a, input, mode, venue, timeout, capacity, dial);
        }
        const endpoint = if (input.object.get("origin")) |v| try w.string(v) else switch (mode) {
            .kalshi => "wss://external-api-ws.kalshi.com/trade-api/ws/v2",
            .kalshi_perps => "wss://external-api-margin-ws.kalshi.com/trade-api/ws/v2/margin",
            .derive => "wss://api.derive.xyz/v3/ws",
            .polymarket => "wss://combos-rfq-gateway-quoter.polymarket.com/ws/rfq",
            .polymarket_perps => "wss://ws.perpetuals.polymarket.com/v1/ws",
            .paradex => "wss://ws.api.prod.paradex.trade/v1",
            .pascal => "wss://trade.pascal.trade/ws",
            else => unreachable,
        };
        const uri = try std.Uri.parse(endpoint);
        const host = try (uri.host orelse return error.InvalidEndpoint).toRawMaybeAlloc(a);
        var arena = std.heap.ArenaAllocator.init(gpa);
        errdefer arena.deinit();
        var headers: std.ArrayList(std.http.Header) = .empty;
        var credentials: V = .null;
        var login: ?V = null;
        if (mode == .derive) {
            const owner = poly.get(input, "api_key");
            const seed = poly.get(input, "api_secret");
            if (owner != .null or seed != .null) {
                if (owner != .string or seed != .string) return error.AuthenticationRequired;
                var rt: std.Io.Threaded = .init_single_threaded;
                login = try @import("../exchanges/derive/signing.zig").login(a, owner.string, seed.string, try now(rt.io()));
                credentials = try w.parse(arena.allocator(), try std.json.Stringify.valueAlloc(a, input, .{}));
            }
        } else if (mode == .polymarket or mode == .polymarket_perps) {
            var rt: std.Io.Threaded = .init_single_threaded;
            const auth = try @import("../exchanges/polymarket/stream.zig").commandLogin(a, input, mode == .polymarket_perps, rt.io());
            login = try w.parse(a, auth);
            credentials = try w.parse(arena.allocator(), try std.json.Stringify.valueAlloc(a, input, .{}));
        } else if (mode == .paradex) {
            const auth = input.object.get("auth") orelse return error.AuthenticationRequired;
            var rt: std.Io.Threaded = .init_single_threaded;
            const token = try @import("venue_auth.zig").bearer(a, auth, std.Io.Timestamp.now(rt.io(), .real).toSeconds());
            credentials = try w.parse(arena.allocator(), try std.json.Stringify.valueAlloc(a, auth, .{}));
            login = .{ .object = .empty };
            try login.?.object.put(a, "bearer", w.text(token));
        } else if (mode == .pascal) {
            const auth = input.object.get("auth") orelse return error.AuthenticationRequired;
            if (auth != .object) return error.AuthenticationRequired;
            const deployment = if (auth.object.get("deployment_id")) |v| try w.unsigned(u8, v) else 3;
            if ((std.ascii.eqlIgnoreCase(host, "trade.pascal.trade") and deployment != 3) or (std.ascii.eqlIgnoreCase(host, "trade.canary.pascal.trade") and deployment != 2)) return error.InvalidNetwork;
            credentials = try w.parse(arena.allocator(), try std.json.Stringify.valueAlloc(a, auth, .{}));
        } else {
            const key = input.object.get("api_key") orelse return error.AuthenticationRequired;
            const seed = input.object.get("api_secret") orelse return error.AuthenticationRequired;
            var runtime: std.Io.Threaded = .init_single_threaded;
            const path = switch (uri.path) {
                .raw, .percent_encoded => |s| s,
            };
            var signed = try @import("auth.zig").kalshiHeaders(a, try w.string(key), try w.string(seed), "GET", if (path.len == 0) "/" else path, try now(runtime.io()));
            var it = signed.object.iterator();
            while (it.next()) |f| try headers.append(a, .{ .name = f.key_ptr.*, .value = try w.string(f.value_ptr.*) });
        }
        const saved_endpoint = try arena.allocator().dupe(u8, endpoint);
        const self = try gpa.create(Session);
        errdefer gpa.destroy(self);
        self.* = .{ .arena = arena, .ws = try Socket.dedicatedHeaders(endpoint, headers.items, dial), .mode = mode, .endpoint = saved_endpoint, .auth = credentials, .timeout_ms = timeout, .capacity = capacity };
        if (login) |params| {
            errdefer self.releaseSocket();
            switch (mode) {
                .derive => {
                    const result = try self.call(a, "public/login", params);
                    if (result != .array) return error.AuthenticationRequired;
                    for (result.array.items) |id| if (id != .number_string) return error.AuthenticationRequired;
                },
                .polymarket, .polymarket_perps => {
                    const ack = try self.polySend(a, .{ .method = "auth", .body = params });
                    const accepted = if (mode == .polymarket) blk: {
                        const v = poly.get(ack, "success");
                        break :blk v == .bool and v.bool;
                    } else blk: {
                        const v = poly.get(poly.get(ack, "data"), "status");
                        break :blk v == .string and eq(u8, v.string, "ok");
                    };
                    if (!accepted) return error.AuthenticationRequired;
                },
                else => _ = try self.call(a, "auth", params),
            }
        }
        return self;
    }
    fn openExchange(a: A, input: V, mode: Mode, venue: TradeVenue, timeout: u32, capacity: u32, dial: ?Dial) !*Session {
        const ws_protocol = @import("exchange_stream.zig");
        const credentials = @import("exchange_http.zig");
        var arena = std.heap.ArenaAllocator.init(gpa);
        errdefer arena.deinit();
        const owned = arena.allocator();
        const endpoint = try Exchange.endpoint(owned, venue, input);
        const config = try w.parse(owned, try std.json.Stringify.valueAlloc(owned, input, .{}));
        var headers: []const std.http.Header = &.{};
        if (venue == .hibachi) headers = &.{.{ .name = "Authorization", .value = try credentials.apiKey(config) }};
        if (venue == .binance_spot) {
            _ = try credentials.apiKey(config);
            _ = try credentials.secret(config);
        }
        const token = if (venue == .kraken_spot) try ws_protocol.krakenToken(owned, config, dial) else null;
        const self = try gpa.create(Session);
        errdefer gpa.destroy(self);
        self.* = .{
            .arena = arena,
            .ws = try Socket.dedicatedHeaders(endpoint, headers, dial),
            .mode = mode,
            .auth = config,
            .timeout_ms = timeout,
            .capacity = capacity,
            .endpoint = endpoint,
            .exchange = venue,
            .exchange_token = token,
        };
        errdefer self.releaseSocket();
        if (Exchange.requiresLogin(venue)) {
            const login = try ws_protocol.login(a, venue, config, self.next_id, self.ws.io.io());
            _ = try self.send(a, "aux_exchange_login", login);
        }
        if (ws_protocol.heartbeat(venue)) |beat| try self.ws.beginHeartbeat(self.ws.conn orelse return error.Network, beat);
        if (venue == .deribit) try self.ws.sendText("{\"jsonrpc\":\"2.0\",\"id\":0,\"method\":\"public/set_heartbeat\",\"params\":{\"interval\":30}}");
        return self;
    }

    fn exchangeControl(self: *Session, raw: []const u8, frame: V) !bool {
        const venue = self.exchange orelse return false;
        switch (Exchange.control(venue, raw, frame)) {
            .pass => return false,
            .skip => return true,
            .reply => |data| {
                try self.ws.sendText(data);
                return true;
            },
        }
    }

    fn exchangeReply(self: *Session, a: A, raw: []const u8, accepted: bool, login: bool) !V {
        self.pending = null;
        if (accepted) return w.parse(a, raw);
        if (self.failure_out) |out| {
            var detail = V{ .object = .empty };
            try detail.object.put(a, "venue", w.text(@tagName(self.mode)));
            try detail.object.put(a, "body", try w.parse(a, raw));
            out.* = detail;
        }
        if (login) {
            // Authentication frames may echo credentials in a rejection.
            return error.AuthenticationRequired;
        }
        return error.VenueRejected;
    }

    fn receive(self: *Session) void {
        self.result = self.ws.message();
        self.ready.set(self.ws.io.io());
    }
    fn read(self: *Session, limit: std.Io.Timeout) !?[]u8 {
        if (self.worker == null) {
            if (limit == .none) return self.ws.message();
            self.ready = .unset;
            self.worker = try std.Thread.spawn(.{}, receive, .{self});
        }
        try self.ready.waitTimeout(self.ws.io.io(), limit);
        self.worker.?.join();
        self.worker = null;
        const result = self.result;
        self.result = null;
        return result;
    }
    fn reconcile(self: *Session, a: A) !V {
        const p = self.pending orelse return error.InvalidOperation;
        const timeout: std.Io.Timeout = .{ .duration = .{ .clock = .awake, .raw = .fromMilliseconds(self.timeout_ms) } };
        const limit: std.Io.Timeout = .{ .deadline = timeout.toTimestamp(self.ws.io.io()).? };
        while (true) {
            const raw = (self.read(limit) catch return error.PendingOutcome) orelse return error.PendingOutcome;
            if ((self.isJsonRpc() or self.exchange != null) and eq(u8, raw, "pong")) {
                gpa.free(raw);
                continue;
            }
            var scratch = std.heap.ArenaAllocator.init(gpa);
            defer scratch.deinit();
            const plain_ping = self.exchange != null and eq(u8, raw, "ping");
            const frame = if (plain_ping) V.null else w.parse(scratch.allocator(), raw) catch |e| {
                gpa.free(raw);
                return if (self.exchange != null) error.PendingOutcome else e;
            };
            if (self.exchange) |venue| {
                const handled = self.exchangeControl(raw, frame) catch {
                    gpa.free(raw);
                    return error.PendingOutcome;
                };
                if (handled) {
                    gpa.free(raw);
                    continue;
                }
                const answer = Exchange.match(venue, frame, p.id, p.method) catch |err| {
                    gpa.free(raw);
                    return err;
                };
                if (answer) |accepted| {
                    defer gpa.free(raw);
                    return self.exchangeReply(a, raw, accepted, eq(u8, p.method, "aux_exchange_login"));
                }
            }
            const id = w.field(frame, "id") catch V.null;
            if (self.isJsonRpc() and id != .null) {
                defer gpa.free(raw);
                if (id != .number_string or (w.unsigned(u64, id) catch 0) != p.id) return error.UnexpectedResponse;
                if (frame.object.get("result") == null and frame.object.get("error") == null) return error.InvalidResponse;
                self.pending = null;
                if ((frame.object.get("error") orelse V.null) != .null) {
                    if (self.failure_out) |out| {
                        var detail = V{ .object = .empty };
                        try detail.object.put(a, "venue", w.text(@tagName(self.mode)));
                        try detail.object.put(a, "body", try w.parse(a, raw));
                        out.* = detail;
                    }
                    return error.VenueRejected;
                }
                const decoded = try w.parse(a, raw);
                return decoded.object.get("result") orelse .null;
            }
            const matches = if (self.exchange != null) false else if (self.mode == .polymarket) poly.matches(frame, p.method, self.correlation, self.quote_id) else (w.unsigned(u64, id) catch 0) == p.id;
            if (matches) {
                defer gpa.free(raw);
                self.pending = null;
                return w.parse(a, raw);
            }
            if (self.events.items.len >= self.capacity) {
                gpa.free(raw);
                self.close();
                return error.StreamOverflow;
            }
            self.events.append(gpa, raw) catch |e| {
                gpa.free(raw);
                self.close();
                return e;
            };
        }
    }
    pub fn next(self: *Session) !?[]u8 {
        if (self.isJsonRpc() and self.pending != null) return error.PendingOutcome;
        if (self.events.items.len != 0) return self.events.orderedRemove(0);
        if (self.pending != null) return error.PendingOutcome;
        if (self.ws.canceled.load(.acquire)) return null;
        const raw = while (true) {
            const msg = try self.read(.none) orelse return null;
            if (self.exchange != null) {
                errdefer gpa.free(msg);
                var scratch = std.heap.ArenaAllocator.init(gpa);
                defer scratch.deinit();
                const frame = if (eq(u8, msg, "ping") or eq(u8, msg, "pong")) V.null else try w.parse(scratch.allocator(), msg);
                if (!try self.exchangeControl(msg, frame)) break msg;
            } else if (!self.isJsonRpc() or !eq(u8, msg, "pong")) break msg;
            gpa.free(msg);
        };
        errdefer gpa.free(raw);
        var scratch = std.heap.ArenaAllocator.init(gpa);
        defer scratch.deinit();
        _ = try w.parse(scratch.allocator(), raw);
        return raw;
    }
    pub fn commandDetailed(self: *Session, a: A, input: V, detail: *V) !V {
        self.failure_out = detail;
        defer self.failure_out = null;
        return self.command(a, input);
    }
    pub fn command(self: *Session, a: A, input: V) !V {
        const op = try text(input, "operation");
        if (eq(u8, op, "pending")) {
            const p = self.pending orelse return .null;
            if (self.mode == .polymarket or self.mode == .polymarket_perps)
                return w.parse(a, try std.json.Stringify.valueAlloc(a, .{ .id = p.id, .method = p.method, .correlation = self.correlation, .quote_id = self.quote_id }, .{}));
            return w.parse(a, try std.json.Stringify.valueAlloc(a, p, .{}));
        }
        if (eq(u8, op, "reconcile")) return self.reconcile(a);
        if (self.pending != null) return error.PendingOutcome;
        if (self.ws.canceled.load(.acquire)) return error.Network;
        var params = try w.field(input, "params");
        if (self.exchange) |venue| {
            const prepared = try Exchange.prepare(a, venue, self.auth, self.exchange_token, op, params, self.next_id, self.ws.io.io(), self.ws.io.dial);
            return self.send(a, prepared.method, prepared.frame);
        }
        if (self.mode == .derive) return self.deriveCommand(a, op, params);
        if (self.mode == .polymarket_perps) return self.polySend(a, try poly.perps(a, op, params, self.auth, self.ws.io.io()));
        if (self.mode == .polymarket) return self.polySend(a, try poly.rfq(a, op, params, self.auth));
        if (self.mode == .paradex) {
            const cmd = try @import("../exchanges/paradex/rpc.zig").prepare(a, op, params, self.auth, self.endpoint, try now(self.ws.io.io()));
            return self.call(a, cmd.method, cmd.params);
        }
        const id = self.next_id;
        var method: []const u8 = undefined;
        const frame = if (self.mode == .pascal) payload: {
            const Kind = enum { place_orders, cancel_orders, mass_cancel, ping };
            const kind = std.meta.stringToEnum(Kind, op) orelse return error.InvalidOperation;
            const cmd: []const u8 = switch (kind) {
                .place_orders => "place",
                .cancel_orders => "cancel",
                .mass_cancel => "mass_cancel",
                .ping => "ping",
            };
            method = cmd;
            const id_text = try std.fmt.allocPrint(a, "{d}", .{id});
            if (kind == .ping) break :payload try std.json.Stringify.valueAlloc(a, .{ .type = cmd, .id = id_text }, .{});
            const name: []const u8 = switch (kind) {
                .place_orders => "write.place_and_replace_orders",
                .cancel_orders => "write.cancel_orders",
                .mass_cancel => "write.mass_cancel_orders",
                .ping => unreachable,
            };
            try @import("../exchanges/pascal/permits.zig").sign(a, name, &params, self.auth, try now(self.ws.io.io()), false);
            break :payload try std.json.Stringify.valueAlloc(a, .{ .type = cmd, .payload = params, .id = id_text }, .{});
        } else payload: {
            const Kind = enum { subscribe, unsubscribe, list_subscriptions, update_subscription };
            method = @tagName(std.meta.stringToEnum(Kind, op) orelse return error.InvalidOperation);
            break :payload try std.json.Stringify.valueAlloc(a, .{ .cmd = method, .params = params, .id = id }, .{});
        };
        return self.send(a, method, frame);
    }
    fn deriveMethod(op: []const u8) ![]const u8 {
        var lines = std.mem.tokenizeScalar(u8, @embedFile("../exchanges/derive/rpc-routes.txt"), '\n');
        while (lines.next()) |line| {
            const sep = std.mem.findScalar(u8, line, ' ') orelse unreachable;
            if (eq(u8, line[0..sep], op)) return line[sep + 1 ..];
        }
        return error.InvalidOperation;
    }
    fn deriveCommand(self: *Session, a: A, op: []const u8, p: V) !V {
        const method = try deriveMethod(op);
        if (p != .object) return error.ExpectedObject;
        if (std.mem.startsWith(u8, method, "private/") and self.auth == .null) return error.AuthenticationRequired;
        const orders = @import("../exchanges/derive/orders.zig");
        var body = p;
        if (orders.supported(op)) {
            var config = V{ .object = try self.auth.object.clone(a) };
            try config.object.put(a, "origin", w.text(self.endpoint));
            try config.object.put(a, "operation", w.text(op));
            try config.object.put(a, "params", p);
            const chain = poly.get(try w.field(self.auth, "params"), "chain_id");
            if (chain != .null) try config.object.put(a, "chain_id", chain);
            var deps = try orders.dependency(a, config);
            var metadata = V{ .object = .empty };
            var it = deps.object.iterator();
            while (it.next()) |item| {
                const job = item.value_ptr.*;
                const result = try self.call(a, try deriveMethod(try text(job, "operation")), try w.field(job, "params"));
                try metadata.object.put(a, item.key_ptr.*, result);
            }
            try config.object.put(a, "metadata", metadata);
            try orders.sign(a, config, &body, try now(self.ws.io.io()));
        }
        return self.call(a, method, body);
    }
    fn clearCorrelation(self: *Session) void {
        if (self.correlation) |s| gpa.free(s);
        if (self.quote_id) |s| gpa.free(s);
        self.correlation = null;
        self.quote_id = null;
    }
    fn polySend(self: *Session, a: A, msg: poly.Frame) !V {
        var frame = msg.body;
        self.clearCorrelation();
        if (self.mode == .polymarket_perps) {
            try frame.object.put(a, "id", .{ .integer = @intCast(self.next_id) });
        } else {
            const id = poly.get(frame, "rfq_id");
            const q = poly.get(frame, "quote_id");
            if (id == .string) self.correlation = try gpa.dupe(u8, id.string);
            if (q == .string) self.quote_id = try gpa.dupe(u8, q.string);
        }
        return self.send(a, msg.method, try std.json.Stringify.valueAlloc(a, frame, .{}));
    }
    fn call(self: *Session, a: A, method: []const u8, params: V) !V {
        return self.send(a, method, try std.json.Stringify.valueAlloc(a, .{ .jsonrpc = "2.0", .id = self.next_id, .method = method, .params = params }, .{}));
    }
    fn send(self: *Session, a: A, method: []const u8, frame: []const u8) !V {
        const id = self.next_id;
        self.next_id = try std.math.add(u64, id, 1);
        self.pending = .{ .id = id, .method = method };
        self.ws.sendText(frame) catch return error.PendingOutcome;
        return self.reconcile(a);
    }
    pub fn close(self: *Session) void {
        self.ws.close();
    }
    fn releaseSocket(self: *Session) void {
        self.close();
        if (self.worker) |t| {
            t.join();
            if (self.result catch null) |b| gpa.free(b);
        }
        for (self.events.items) |b| gpa.free(b);
        self.events.deinit(gpa);
        self.clearCorrelation();
        self.ws.destroy();
    }
    pub fn destroy(self: *Session) void {
        self.releaseSocket();
        self.arena.deinit();
        gpa.destroy(self);
    }
};
