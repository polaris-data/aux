// Native HTTP(S) dispatch and optional application-owned transport.
const std = @import("std");
const wire = @import("wire.zig");
const perps = @import("perps.zig");
const hip3 = @import("../exchanges/hyperliquid/hip3.zig");
const http = @import("http.zig");
const routes = @import("routes.zig");
const connectors = @import("perps_connectors.zig");
const exchange = @import("exchange_http.zig");
const A = wire.A;
const V = wire.V;

fn origin(v: routes.Venue) []const u8 {
    return switch (v) {
        else => unreachable, // These catalogs provide a per-operation service origin.
        .derive => "https://api.derive.xyz",
        .paradex => "https://api.prod.paradex.trade",
        .extended => "https://api.starknet.extended.exchange",
        .phoenix => "https://perp-api.phoenix.trade",
    };
}

const Dispatch = struct {
    request: http.Request,
    url: []const u8,
    payload: ?[]const u8,
    binary: bool = false,
};

fn kalshiTarget(a: A, uri: std.Uri, request: http.Request) ![]const u8 {
    const prefix = switch (uri.path) {
        .raw, .percent_encoded => |path| path,
    };
    return std.mem.concat(a, u8, &.{ std.mem.trimEnd(u8, prefix, "/"), request.path });
}
pub fn kalshiProof(a: A, input: V) !V {
    if (try http.venue(input) != .kalshi) return error.InvalidVenue;
    const op = try wire.string(try wire.field(input, "operation"));
    if (std.mem.startsWith(u8, op, "clearing.")) return error.InvalidOperation;
    const request = try http.prepare(a, input);
    const base = if (input.object.get("origin")) |v| try wire.string(v) else request.origin;
    const uri = try std.Uri.parse(base);
    if (uri.host == null or uri.user != null or uri.password != null or uri.query != null or uri.fragment != null or
        !(std.mem.eql(u8, uri.scheme, "http") or std.mem.eql(u8, uri.scheme, "https"))) return error.InvalidOrigin;
    var rt: std.Io.Threaded = .init_single_threaded;
    const ms = std.math.cast(u64, std.Io.Timestamp.now(rt.io(), .real).toMilliseconds()) orelse return error.InvalidTimestamp;
    const target = try kalshiTarget(a, uri, request);
    const message = try @import("auth.zig").kalshiMessage(a, try wire.string(try wire.field(input, "api_key")), request.method, target, ms);
    var out = V{ .object = .empty };
    try out.object.put(a, "payload", wire.text(message));
    try out.object.put(a, "timestamp", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{ms}) });
    return out;
}
fn dispatch(a: A, input: V, io: std.Io, stream: bool) !Dispatch {
    const venue = try http.venue(input);
    var prepared_input = input;
    const operation_name = try wire.string(try wire.field(input, "operation"));
    const active_orders = venue == .lighter and std.mem.eql(u8, operation_name, "active_orders");
    if (active_orders) {
        const auth = input.object.get("auth") orelse return error.AuthenticationRequired;
        const account = try wire.field(auth, "account_index");
        var params = try wire.field(input, "params");
        if (params != .object) return error.InvalidParameter;
        if (params.object.get("account_index")) |supplied| if (try wire.unsigned(u64, supplied) != try wire.unsigned(u64, account)) return error.AuthenticationRequired;
        try params.object.put(a, "account_index", account);
        try prepared_input.object.put(a, "params", params);
    }
    if (venue == .backpack) {
        const params = try wire.field(input, "params");
        try perps.put(&prepared_input, a, "wire_params", try perps.toValue(a, params));
    }
    var request = try http.prepare(a, prepared_input);
    const lighter_chain: ?u32 = if (venue == .lighter and (active_orders or std.mem.eql(u8, operation_name, "next_nonce")) and input.object.get("auth") != null)
        try wire.unsigned(u32, try wire.field(try wire.field(input, "auth"), "chain_id"))
    else
        null;
    if (lighter_chain == 300) request.origin = "https://testnet.zklighter.elliot.ai";
    const framing = http.streamFraming(venue, operation_name);
    if ((framing != null) != stream) return error.InvalidOperation;
    if (framing == .sse) try request.headers.object.put(a, "accept", wire.text("text/event-stream"));
    const hl_trade = venue == .hyperliquid and request.authenticated;
    if (hl_trade) {
        const auth = input.object.get("auth") orelse return error.AuthenticationRequired;
        const network = try wire.string(try wire.field(auth, "network"));
        if (std.mem.eql(u8, network, "Testnet")) request.origin = "https://api.hyperliquid-testnet.xyz";
    }
    const base = if (venue == .dydx_v4 and std.mem.eql(u8, operation_name, "transaction"))
        try perps.credential(perps.get(perps.get(input, "auth"), "node_origin"))
    else if (input.object.get("origin")) |v| try wire.string(v) else if (request.origin.len > 0) request.origin else origin(venue);
    const uri = try std.Uri.parse(base);
    if (!(std.mem.eql(u8, uri.scheme, "http") or std.mem.eql(u8, uri.scheme, "https")) or uri.host == null or
        uri.user != null or uri.password != null or uri.query != null or uri.fragment != null) return error.InvalidOrigin;
    for (base) |b| if (b <= 0x20 or b == 0x7f or b == '\\') return error.InvalidOrigin;
    if (lighter_chain) |chain| {
        const host = try uri.host.?.toRawMaybeAlloc(a);
        if ((std.ascii.eqlIgnoreCase(host, "testnet.zklighter.elliot.ai") and chain != 300) or
            (std.ascii.eqlIgnoreCase(host, "mainnet.zklighter.elliot.ai") and chain != 304)) return error.InvalidNetwork;
    }
    const url = if (framing == .download) target: {
        const target = try std.Uri.parse(request.path);
        const secure = std.mem.eql(u8, uri.scheme, "https");
        if (secure and !std.mem.eql(u8, target.scheme, "https")) return error.InvalidDownloadUrl;
        // Keep the original target intact: storage signatures cover its encoded query.
        const default_port: u16 = if (secure) 443 else 80;
        request.authenticated = std.mem.eql(u8, uri.scheme, target.scheme) and
            (uri.port orelse default_port) == (target.port orelse default_port) and
            std.ascii.eqlIgnoreCase(try uri.host.?.toRawMaybeAlloc(a), try target.host.?.toRawMaybeAlloc(a));
        break :target request.path;
    } else try std.mem.concat(a, u8, &.{ std.mem.trimEnd(u8, base, "/"), request.path });
    if (exchange.supports(venue)) {
        const payload = try exchange.authorize(a, venue, operation_name, prepared_input, &request, io);
        return .{ .request = request, .url = try targetUrl(a, base, request.path), .payload = payload };
    }
    if (connectors.supports(venue))
        return connectorDispatch(a, input, venue, operation_name, base, request, io);
    if (perps.isVenue(venue) and request.authenticated) {
        const auth = perps.get(input, "auth");
        switch (venue) {
            .aster => {
                const signed = try @import("../exchanges/aster/trading.zig").sign(a, auth, request.method, request.path, request.body, try perps.epoch(io, .us));
                try perps.put(&request.headers, a, "content-type", wire.text("application/x-www-form-urlencoded"));
                request.path = signed.path;
                return .{ .request = request, .url = try std.mem.concat(a, u8, &.{ std.mem.trimEnd(u8, base, "/"), signed.path }), .payload = signed.body };
            },
            .ondo_perps => {
                const target = try kalshiTarget(a, uri, request);
                const payload = if (request.body == .null) null else try std.json.Stringify.valueAlloc(a, request.body, .{});
                try @import("../exchanges/ondo_perps/auth.zig").sign(a, input, &request, target, payload orelse "", try perps.epoch(io, .ms));
                return .{ .request = request, .url = url, .payload = payload };
            },
            .pacifica => request.body = try @import("../exchanges/pacifica/trading.zig").sign(a, operation_name, auth, request.body, try perps.epoch(io, .ms)),
            .nado => {
                const metadata = try wire.field(input, "metadata");
                request.body = try @import("../exchanges/nado/trading.zig").sign(a, operation_name, auth, try wire.field(metadata, "contracts"), request.body);
            },
            .paragon, .trade_xyz => {
                const name = hip3.dex(venue).?;
                var config = input;
                var signing_auth = perps.object();
                if (auth == .object) for (auth.object.keys(), auth.object.values()) |key, value| {
                    try perps.put(&signing_auth, a, key, value);
                };
                try perps.put(&signing_auth, a, "network", wire.text(if (try perps.testnet(input)) "Testnet" else "Mainnet"));
                const host = try uri.host.?.toRawMaybeAlloc(a);
                const expected = (try std.Uri.parse(request.origin)).host.?;
                if ((std.ascii.eqlIgnoreCase(host, "api.hyperliquid.xyz") or std.ascii.eqlIgnoreCase(host, "api.hyperliquid-testnet.xyz")) and
                    !std.ascii.eqlIgnoreCase(host, try expected.toRawMaybeAlloc(a))) return error.InvalidNetwork;
                try perps.put(&config, a, "auth", signing_auth);
                const action = try hip3.action(a, name, operation_name, request.body, try wire.field(input, "metadata"));
                request.body = try @import("../exchanges/hyperliquid/trading.zig").envelope(a, action, config, io);
            },
            .injective => return error.InvalidOperation,
            else => unreachable,
        }
        return .{ .request = request, .url = url, .payload = if (request.body == .null) null else try std.json.Stringify.valueAlloc(a, request.body, .{}) };
    }
    const supplied_key = input.object.get("api_key");
    const signing_kalshi = venue == .kalshi and (request.authenticated or supplied_key != null);
    if (venue == .arcus and request.authenticated) {
        _ = try @import("../exchanges/arcus/trading.zig").signing(a, input, io, &request);
    } else if (hl_trade) {
        const host = try uri.host.?.toRawMaybeAlloc(a);
        const expected = (try std.Uri.parse(request.origin)).host.?;
        if ((std.ascii.eqlIgnoreCase(host, "api.hyperliquid.xyz") or std.ascii.eqlIgnoreCase(host, "api.hyperliquid-testnet.xyz")) and !std.ascii.eqlIgnoreCase(host, try expected.toRawMaybeAlloc(a))) return error.InvalidNetwork;
        request.body = try @import("../exchanges/hyperliquid/trading.zig").envelope(a, request.body, input, io);
    } else if (active_orders) {
        var token_cfg = V{ .object = .empty };
        var token_params = V{ .object = .empty };
        const now = std.Io.Timestamp.now(io, .real).toSeconds();
        try token_params.object.put(a, "deadline", .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{now + 600}) });
        try token_cfg.object.put(a, "auth", try wire.field(input, "auth"));
        try token_cfg.object.put(a, "origin", wire.text(base));
        try token_cfg.object.put(a, "params", token_params);
        try request.headers.object.put(a, "authorization", try @import("../exchanges/lighter/trading.zig").execute(a, token_cfg, "auth_token", null));
    } else if (venue == .monaco and request.authenticated) {
        const session = input.object.get("auth") orelse return error.AuthenticationRequired;
        const prefix = switch (uri.path) {
            .raw, .percent_encoded => |v| v,
        };
        const target = try std.mem.concat(a, u8, &.{ std.mem.trimEnd(u8, prefix, "/"), request.path });
        const body = if (request.body == .null) "" else try std.json.Stringify.valueAlloc(a, request.body, .{});
        var signed = try @import("auth.zig").monacoSession(a, session, base, request.method, target, body, io);
        if (std.mem.eql(u8, operation_name, "revoke_session")) try @import("../exchanges/monaco/session.zig").revoke(session, io);
        var iter = signed.object.iterator();
        while (iter.next()) |h| try request.headers.object.put(a, h.key_ptr.*, h.value_ptr.*);
    } else if (signing_kalshi and !std.mem.startsWith(u8, try wire.string(try wire.field(input, "operation")), "clearing.")) {
        const key = try wire.string(supplied_key orelse return error.AuthenticationRequired);
        const target = try kalshiTarget(a, uri, request);
        const auth = input.object.get("auth") orelse .null;
        var headers = if (auth != .null and auth == .object and auth.object.get("signature") != null) external: {
            const ms = try wire.unsigned(u64, try wire.field(auth, "timestamp"));
            const message = try @import("auth.zig").kalshiMessage(a, key, request.method, target, ms);
            if (!std.mem.eql(u8, message, try wire.string(try wire.field(auth, "payload")))) return error.InvalidSignature;
            const hex = try wire.string(try wire.field(auth, "signature"));
            if (hex.len > 2048 or hex.len % 2 != 0) return error.InvalidSignature;
            const bytes = try a.alloc(u8, hex.len / 2);
            _ = try std.fmt.hexToBytes(bytes, hex);
            break :external try @import("auth.zig").kalshiSignature(a, key, ms, bytes);
        } else local: {
            const seed = try wire.string(input.object.get("api_secret") orelse return error.AuthenticationRequired);
            const ms = std.math.cast(u64, std.Io.Timestamp.now(io, .real).toMilliseconds()) orelse return error.InvalidTimestamp;
            break :local try @import("auth.zig").kalshiHeaders(a, key, seed, request.method, target, ms);
        };
        var iter = headers.object.iterator();
        while (iter.next()) |h| try request.headers.object.put(a, h.key_ptr.*, h.value_ptr.*);
    } else if (request.authenticated and (venue == .qfex or venue == .derive)) {
        const key = try wire.string(supplied_key orelse return error.AuthenticationRequired);
        const secret = try wire.string(input.object.get("api_secret") orelse return error.AuthenticationRequired);
        if (venue == .qfex) {
            try @import("auth.zig").qfexHeaders(a, &request.headers, io, key, secret);
        } else {
            const derive = @import("../exchanges/derive/signing.zig");
            const operation = try wire.string(try wire.field(input, "operation"));
            const ms = std.Io.Timestamp.now(io, .real).toMilliseconds();
            if (ms < 0) return error.InvalidTimestamp;
            if (!derive.loginOnly(operation)) {
                const orders = @import("../exchanges/derive/orders.zig");
                if (!orders.supported(operation)) return error.AuthenticationRequired;
                try orders.sign(a, input, &request.body, @intCast(ms));
            }
            const proof = try derive.login(a, key, secret, @intCast(ms));
            try request.headers.object.put(a, "X-DeriveWallet", try wire.field(proof, "wallet"));
            try request.headers.object.put(a, "X-DeriveTimestamp", try wire.field(proof, "timestamp"));
            try request.headers.object.put(a, "X-DeriveSignature", try wire.field(proof, "signature"));
        }
    } else if (request.authenticated and venue == .pascal) {
        const now = std.Io.Timestamp.now(io, .real).toMilliseconds();
        if (now < 0) return error.InvalidTimestamp;
        const auth = input.object.get("auth") orelse return error.AuthenticationRequired;
        const hostname = switch (uri.host.?) {
            .raw, .percent_encoded => |s| s,
        };
        if (auth != .object) return error.AuthenticationRequired;
        const deployment = if (auth.object.get("deployment_id")) |n| try wire.unsigned(u8, n) else 3;
        if ((std.ascii.eqlIgnoreCase(hostname, "trade.canary.pascal.trade") and deployment != 2) or (std.ascii.eqlIgnoreCase(hostname, "trade.pascal.trade") and deployment != 3)) return error.InvalidNetwork;
        try @import("../exchanges/pascal/permits.zig").sign(a, operation_name, &request.body, auth, @intCast(now), false);
    } else if (request.authenticated and venue == .paradex) {
        const auth = input.object.get("auth") orelse return error.AuthenticationRequired;
        const ms = std.Io.Timestamp.now(io, .real).toMilliseconds();
        if (ms < 0) return error.InvalidTimestamp;
        try @import("venue_auth.zig").paradex(a, &request.headers, operation_name, auth, @divTrunc(ms, 1000));
        try @import("../exchanges/paradex/sign.zig").dispatch(a, operation_name, input, request.path, &request.body, auth, @intCast(ms));
    } else if (venue == .polymarket and std.mem.startsWith(u8, operation_name, "perps.") and (request.authenticated or (input.object.contains("auth") and @import("venue_auth.zig").hasPerpsSigner(operation_name)))) {
        const auth = @import("venue_auth.zig");
        const kind = try auth.routeKind(@embedFile("../exchanges/polymarket/perps-routes.txt"), try wire.string(try wire.field(input, "operation")));
        const config = input.object.get("auth") orelse return error.AuthenticationRequired;
        const ms = std.Io.Timestamp.now(io, .real).toMilliseconds();
        if (ms < 0) return error.InvalidTimestamp;
        if (std.mem.eql(u8, kind, "proxy")) {
            try auth.proxyHeaders(a, &request.headers, config, @intCast(ms));
        } else {
            try @import("../exchanges/polymarket/perps_sign.zig").sign(a, &request.body, config, kind, @intCast(ms), io);
        }
    } else if (request.authenticated and venue == .polymarket) {
        const clob = @import("../exchanges/polymarket/clob_auth.zig");
        const wallet = @import("../exchanges/polymarket/clob_wallet.zig");
        const sec = std.Io.Timestamp.now(io, .real).toSeconds();
        if (sec < 0) return error.InvalidTimestamp;
        if (wallet.isLogin(operation_name, input.object.contains("clob"))) {
            try wallet.login(a, &request.headers, input.object.get("auth") orelse return error.AuthenticationRequired, @intCast(sec));
            return .{ .request = request, .url = url, .payload = null };
        }
        if (@import("venue_auth.zig").routeKind(@embedFile("../exchanges/polymarket/service-routes.txt"), operation_name)) |kind| {
            const path = switch ((try std.Uri.parse(url)).path) {
                .raw, .percent_encoded => |s| s,
            };
            const bytes = if (request.body == .null) null else try std.json.Stringify.valueAlloc(a, request.body, .{});
            try clob.serviceHeaders(a, &request.headers, input, kind, request.method, path, bytes orelse "", @intCast(sec));
            return .{ .request = request, .url = url, .payload = bytes };
        } else |_| {}
        const credentials = input.object.get("clob") orelse return error.AuthenticationRequired;
        if (wallet.isOrder(operation_name)) {
            try wallet.orders(a, &request.body, input.object.get("auth") orelse .null, credentials, std.mem.eql(u8, operation_name, "clob.clob_post_orders"));
        } else if (!clob.supported(operation_name)) return error.AuthenticationRequired;
        const target = switch ((try std.Uri.parse(url)).path) {
            .raw, .percent_encoded => |s| s,
        };
        const bytes = if (request.body == .null) null else try std.json.Stringify.valueAlloc(a, request.body, .{});
        try clob.headers(a, &request.headers, credentials, request.method, target, bytes orelse "", @intCast(sec));
        return .{ .request = request, .url = url, .payload = bytes };
    } else if (venue == .chainlink_streams and (request.authenticated or supplied_key != null or input.object.get("api_secret") != null)) {
        const key = try wire.string(supplied_key orelse return error.AuthenticationRequired);
        const secret = try wire.string(input.object.get("api_secret") orelse return error.AuthenticationRequired);
        for ([_][]const u8{ key, secret }) |part| {
            if (part.len == 0) return error.AuthenticationRequired;
            for (part) |b| if (b < 0x21 or b > 0x7e) return error.InvalidHeader;
        }
        const now = std.Io.Timestamp.now(io, .real).toMilliseconds();
        if (now < 0) return error.InvalidTimestamp;
        const base_path = if (input.object.get("origin")) |v| switch ((try std.Uri.parse(try wire.string(v))).path) {
            .raw, .percent_encoded => |path| path,
        } else "";
        const target = try std.mem.concat(a, u8, &.{ std.mem.trimEnd(u8, base_path, "/"), request.path });
        try @import("auth.zig").chainlinkHeaders(a, &request.headers, key, secret, request.method, target, @intCast(now));
    } else if (venue == .extended or venue == .phoenix) {
        const HeaderCredentials = @import("venue_auth.zig").HeaderCredentials;
        const credentials = try HeaderCredentials.parse(a, input);
        try HeaderCredentials.checkExpiry(credentials.expires, io);
        if (request.authenticated and credentials.key == null and credentials.bearer_token == null) return error.AuthenticationRequired;
        if (credentials.key) |key| try request.headers.object.put(a, "X-Api-Key", wire.text(key));
        if (credentials.bearer_token) |token| try request.headers.object.put(a, "Authorization", wire.text(token));
    } else if (request.authenticated or (supplied_key != null and (venue == .metric or (http.oracleVenue(venue) and venue != .chainlink_streams and !std.mem.eql(u8, request.path, "/api/v1/authorize")) or venue == .tardis))) {
        const key_name: []const u8 = switch (venue) {
            .kalshi => "Authorization",
            .birdeye => "X-API-KEY",
            .dune => "X-Dune-API-Key",
            .metric, .hyperdash, .tardis, .databento, .chainlink_candles, .pyth_pro, .pyth_hermes => "authorization",
            else => return error.AuthenticationRequired,
        };
        const key = try wire.string(input.object.get("api_key") orelse return error.AuthenticationRequired);
        if (key.len == 0) return error.AuthenticationRequired;
        for (key) |b| if (b < 0x21 or b > 0x7e) return error.InvalidHeader;
        const value = if (venue == .databento) blk: {
            if (key.len != 32 or std.mem.findScalar(u8, key, '|') != null) return error.InvalidCredential;
            const tagged = try std.mem.concat(a, u8, &.{ key, ":" });
            const encoder = std.base64.standard.Encoder;
            const encoded = encoder.encode(try a.alloc(u8, encoder.calcSize(tagged.len)), tagged);
            break :blk try std.mem.concat(a, u8, &.{ "Basic ", encoded });
        } else if (venue == .metric or venue == .kalshi or venue == .hyperdash or venue == .tardis or http.oracleVenue(venue)) try std.mem.concat(a, u8, &.{ "Bearer ", key }) else key;
        try request.headers.object.put(a, key_name, wire.text(value));
    }
    const payload = if (request.body == .null) null else if (request.body == .string and (venue == .databento or venue == .chainlink_candles)) request.body.string else try std.json.Stringify.valueAlloc(a, request.body, .{});
    return .{ .request = request, .url = url, .payload = payload };
}

fn connectorDispatch(a: A, input: V, venue: routes.Venue, op: []const u8, base: []const u8, source: http.Request, io: std.Io) !Dispatch {
    var request = source;
    const write = perps.isWrite(venue, op);
    switch (venue) {
        .perpl => if (write) {
            request.body = try @import("../exchanges/perpl/trading.zig").order(a, op, request.body);
        },
        .risex => if (write) {
            request.body = try @import("../exchanges/risex/trading.zig").sign(a, input, io, op, request.body);
        },
        .decibel => {
            if (perps.get(input, "api_key") != .null or !std.mem.startsWith(u8, request.path, "/v1")) try connectors.bearer(a, input, &request.headers);
            if (write) {
                try perps.put(&request.headers, a, "content-type", wire.text("application/x.aptos.signed_transaction+bcs"));
                const payload = try @import("../exchanges/decibel/trading.zig").build(a, input, request.body, op, io);
                return .{ .request = request, .url = try targetUrl(a, base, request.path), .payload = payload, .binary = true };
            }
        },
        .edgex => if (request.authenticated) {
            const trade = @import("../exchanges/edgex/trading.zig");
            const now = try perps.epoch(io, .ms);
            if (write) {
                if (perps.eq(u8, op, "place_order")) request.body = try trade.order(a, input, request.body, now) else try trade.cancel(input, request.body);
            }
            try trade.headers(a, input, request.method, request.path, request.body, try std.fmt.allocPrint(a, "{d}", .{now}), &request.headers);
        },
        .grvt => if (request.authenticated) {
            const trade = @import("../exchanges/grvt/trading.zig");
            if (perps.eq(u8, op, "login")) {
                request.body = try trade.loginRequest(a, input, &request.headers);
            } else {
                try trade.sessionHeaders(a, input, &request.headers);
                if (write) {
                    if (perps.eq(u8, op, "place_order")) request.body = try trade.order(a, input, request.body, io) else try trade.cancel(request.body);
                }
            }
        },
        .katana => if (request.authenticated) {
            const trade = @import("../exchanges/katana/trading.zig");
            if (perps.eq(u8, request.method, "GET")) request.path = try trade.query(a, request.path, io);
            if (write) request.body = try trade.order(a, input, op, request.body, io);
        },
        else => unreachable,
    }
    const payload = try connectors.json(a, request.body);
    if (request.authenticated) {
        if (venue == .perpl) try @import("../exchanges/perpl/trading.zig").headers(a, input, io, &request, payload orelse "");
        if (venue == .katana) try @import("../exchanges/katana/trading.zig").headers(a, input, request.method, request.path, payload orelse "", &request.headers);
    }
    return .{ .request = request, .url = try targetUrl(a, base, request.path), .payload = payload };
}

fn targetUrl(a: A, base: []const u8, path: []const u8) ![]const u8 {
    return std.mem.concat(a, u8, &.{ std.mem.trimEnd(u8, base, "/"), path });
}

// Produce the exact bytes and headers for a host-provided TLS policy.
pub fn prepareDispatch(a: A, input: V) !V {
    var io: std.Io.Threaded = .init_single_threaded;
    const prepared = try dispatch(a, input, io.io(), false);
    var out: V = .{ .object = .empty };
    try out.object.put(a, "method", wire.text(prepared.request.method));
    try out.object.put(a, "url", wire.text(prepared.url));
    try out.object.put(a, "headers", prepared.request.headers);
    if (prepared.binary) {
        const body = prepared.payload.?;
        try out.object.put(a, "body_hex", wire.text(try std.fmt.allocPrint(a, "{x}", .{body})));
        try out.object.put(a, "transaction_hash", wire.text(try @import("../exchanges/decibel/trading.zig").hash(a, body)));
    } else {
        try out.object.put(a, "body", if (prepared.payload) |body| wire.text(body) else .null);
    }
    return out;
}

pub fn send(a: A, input: V, client: *std.http.Client, stream: bool) !std.http.Client.Request {
    const prepared = try dispatch(a, input, client.io, stream);
    return sendRequest(a, client, prepared.request.method, prepared.url, prepared.request.headers, prepared.payload);
}

pub fn sendRequest(a: A, client: *std.http.Client, verb: []const u8, url: []const u8, headers: V, payload: ?[]const u8) !std.http.Client.Request {
    return sendTracked(a, client, verb, url, headers, payload, null);
}

pub fn sendTracked(a: A, client: *std.http.Client, verb: []const u8, url: []const u8, headers: V, payload: ?[]const u8, attempted: ?*bool) !std.http.Client.Request {
    return sendWithOptions(a, client, verb, url, headers, payload, attempted, false);
}
fn sendWithOptions(a: A, client: *std.http.Client, verb: []const u8, url: []const u8, headers: V, payload: ?[]const u8, attempted: ?*bool, keep_alive: bool) !std.http.Client.Request {
    var extra: std.ArrayList(std.http.Header) = .empty;
    var iter = headers.object.iterator();
    while (iter.next()) |entry| {
        try extra.append(a, .{ .name = entry.key_ptr.*, .value = entry.value_ptr.string });
    }
    const method = std.meta.stringToEnum(std.http.Method, verb) orelse return error.InvalidVerb;
    var req = try @import("host_stream.zig").request(client, method, try std.Uri.parse(url), .{
        .redirect_behavior = .unhandled,
        .keep_alive = keep_alive,
        .headers = .{ .content_type = .omit, .user_agent = .omit },
        .extra_headers = extra.items,
    });
    errdefer req.deinit();
    req.accept_encoding[@backingInt(std.http.ContentEncoding.zstd)] = true;
    if (attempted) |p| p.* = true;
    if (payload) |bytes| {
        req.transfer_encoding = .{ .content_length = bytes.len };
        var body = req.sendBodyUnflushed(&.{}) catch return error.Network;
        body.writer.writeAll(bytes) catch return error.Network;
        body.end() catch return error.Network;
        req.connection.?.flush() catch return error.Network;
    } else req.sendBodiless() catch return error.Network;
    return req;
}

pub fn bodyReader(a: A, response: *std.http.Client.Response, buffer: []u8, decompressor: *std.http.Decompress) !*std.Io.Reader {
    const capacity: usize = switch (response.head.content_encoding) {
        .identity => 0,
        .zstd => std.compress.zstd.default_window_len + std.compress.zstd.block_size_max,
        .gzip, .deflate => std.compress.flate.max_window_len,
        else => return error.Network,
    };
    return response.readerDecompressing(buffer, decompressor, try a.alloc(u8, capacity));
}

pub fn execute(a: A, config: V) !V {
    return executeWithDial(a, config, null);
}
pub fn dependencies(a: A, input: V) !V {
    const venue = try http.venue(input);
    if (venue == .derive) return @import("../exchanges/derive/orders.zig").dependency(a, input);
    const operation = try perps.text(input, "operation");
    var jobs = perps.object();
    if (exchange.supports(venue)) return exchange.dependencies(a, input, venue, operation);
    if (!perps.isWrite(venue, operation)) return jobs;
    if (connectors.supports(venue)) return connectors.dependencies(a, venue, operation, input);
    const names: []const []const u8 = switch (venue) {
        .nado => &.{"contracts"},
        .paragon, .trade_xyz => &.{ "perp_dexs", "markets" },
        else => &.{},
    };
    for (names) |name| {
        var job = perps.object();
        try perps.put(&job, a, "venue", wire.text(@tagName(venue)));
        try perps.put(&job, a, "operation", wire.text(name));
        try perps.put(&job, a, "params", perps.object());
        const base = perps.get(input, "origin");
        if (base != .null) try perps.put(&job, a, "origin", base);
        if (try perps.testnet(input)) try perps.put(&job, a, "auth", try perps.toValue(a, .{ .network = "Testnet" }));
        try perps.put(&jobs, a, name, job);
    }
    return jobs;
}

pub fn executeWithDial(a: A, config: V, dial: ?@import("host_stream.zig").Dial) !V {
    var input = config;
    const venue = try http.venue(input);
    const operation = try perps.text(input, "operation");
    if (venue == .dydx_v4 and exchange.isWrite(venue, operation))
        return @import("../exchanges/dydx_v4/trading.zig").execute(a, input, operation, dial);
    if (venue == .injective and (perps.isWrite(venue, operation) or std.mem.eql(u8, operation, "pending")))
        return @import("../exchanges/injective/trading.zig").execute(a, input, operation, dial);
    if (venue == .lighter) {
        for ([_]([]const u8){ "auth_token", "create_order", "cancel_order", "resync_nonce", "prepare_transaction" }) |op| if (std.mem.eql(u8, op, operation)) return @import("../exchanges/lighter/trading.zig").execute(a, input, operation, dial);
    }
    const jobs = try dependencies(a, input);
    if (jobs.object.count() != 0 or venue == .derive) {
        var metadata = perps.object();
        var it = jobs.object.iterator();
        while (it.next()) |entry| try perps.put(&metadata, a, entry.key_ptr.*, try executePrepared(a, entry.value_ptr.*, dial));
        try perps.put(&input, a, "metadata", metadata);
    }
    const result = try executePrepared(a, input, dial);
    if (hip3.dex(venue)) |name| {
        if (std.mem.eql(u8, operation, "order_status")) try hip3.validateOrderStatus(name, try perps.text(try wire.field(input, "params"), "coin"), result);
    }
    return result;
}
fn executePrepared(a: A, input: V, dial: ?@import("host_stream.zig").Dial) !V {
    const venue = try http.venue(input);
    var io: @import("host_stream.zig").Runtime = .{ .dial = dial };
    const nonce_lock = if (venue == .kraken_spot and try http.operationPrivate(input)) try exchange.lockKraken(input, io.io()) else null;
    defer if (nonce_lock) |lock| lock.unlock(io.io());
    var local: std.http.Client = .{ .io = io.io(), .allocator = std.heap.c_allocator };
    defer local.deinit();
    const prepared = try dispatch(a, input, local.io, false);
    const lease = if (dial == null) @import("http_pool.zig").acquire(prepared.url) else null;
    defer if (lease) |l| l.release();
    const client = if (lease) |l| l.client else &local;
    const reservation = try @import("../exchanges/pascal/permits.zig").reserve(input);
    var attempted = false;
    var confirmed = false;
    defer if (reservation) |r| r.finish(!attempted or confirmed);
    const operation = try perps.text(input, "operation");
    const uncertain_write = (venue == .monaco and @import("../exchanges/monaco/orders.zig").isWrite(operation)) or perps.isWrite(venue, operation) or exchange.isWrite(venue, operation);
    var req = sendWithOptions(a, client, prepared.request.method, prepared.url, prepared.request.headers, prepared.payload, &attempted, lease != null) catch |err| {
        if (!uncertain_write or !attempted) return err;
        recordUncertain(a, prepared);
        return error.PendingOutcome;
    };
    defer req.deinit();
    errdefer if (req.connection) |conn| {
        conn.closing = true;
    };
    var response = req.receiveHead(&.{}) catch {
        if (!uncertain_write) return error.Network;
        recordUncertain(a, prepared);
        return error.PendingOutcome;
    };
    const status = @backingInt(response.head.status);
    confirmed = status >= 400 and status < 500 and status != 408;
    // Header strings belong to the HTTP read buffer until the body is read.
    var headers: std.ArrayList(std.http.Header) = .empty;
    if (http.oracleVenue(venue) or venue == .hyperdash or venue == .dune or (venue == .grvt and perps.eq(u8, operation, "login"))) {
        var it = response.head.iterateHeaders();
        while (it.next()) |h| try headers.append(a, .{ .name = try a.dupe(u8, h.name), .value = try a.dupe(u8, h.value) });
    }
    var buffer: [8192]u8 = undefined;
    var decompressor: std.http.Decompress = undefined;
    const reader = bodyReader(a, &response, &buffer, &decompressor) catch |err| {
        if (!uncertain_write or confirmed) return err;
        recordUncertain(a, prepared);
        return error.PendingOutcome;
    };
    const bytes = reader.allocRemaining(a, .limited(16 * 1024 * 1024)) catch {
        if (!uncertain_write or confirmed) return error.Network;
        recordUncertain(a, prepared);
        return error.PendingOutcome;
    };
    const result = finish(a, venue, status, headers.items, bytes, input.object.get("api_key"), input.object.get("operation")) catch |err| {
        if (err == error.PendingOutcome) recordUncertain(a, prepared);
        return err;
    };
    confirmed = true;
    try refreshMonaco(input, result, client.io);
    return result;
}

fn recordUncertain(a: A, prepared: Dispatch) void {
    if (!prepared.binary) return;
    const hash = @import("../exchanges/decibel/trading.zig").hash(a, prepared.payload.?) catch return;
    const detail = std.json.Stringify.valueAlloc(a, .{ .hash = hash }, .{}) catch return;
    @import("http_failure.zig").record("decibel", 0, detail);
}

pub fn finish(a: A, venue: routes.Venue, status: u16, headers: []const std.http.Header, bytes: []const u8, key: ?V, operation: ?V) !V {
    if (bytes.len > 16 * 1024 * 1024) return error.BodyTooLarge;
    if (operation) |raw_op| {
        const op = try wire.string(raw_op);
        if (exchange.isWrite(venue, op)) {
            if (status == 408 or status >= 500) return error.PendingOutcome;
            if (status >= 200 and status < 300) {
                const value = wire.parse(a, bytes) catch return error.PendingOutcome;
                if (value != .object and value != .array) return error.PendingOutcome;
            }
        }
        if (perps.isWrite(venue, op)) {
            if (status == 408 or status >= 500) return error.PendingOutcome;
            if (status >= 200 and status < 300) {
                const value = wire.parse(a, bytes) catch return error.PendingOutcome;
                if (connectors.supports(venue) and !perps.replyError(venue, value)) {
                    connectors.checkWrite(venue, op, value) catch |err| {
                        if (err == error.ExchangeApi) @import("http_failure.zig").record(@tagName(venue), status, bytes);
                        return err;
                    };
                } else if (hip3.dex(venue) != null and !perps.replyError(venue, value)) {
                    const statuses = perps.get(perps.get(perps.get(value, "response"), "data"), "statuses");
                    if (statuses != .array or statuses.array.items.len != 1) return error.PendingOutcome;
                    if (perps.get(statuses.array.items[0], "error") != .null) {
                        @import("http_failure.zig").record(@tagName(venue), status, bytes);
                        return error.ExchangeApi;
                    }
                } else if (value != .object and !perps.replyError(venue, value)) return error.PendingOutcome;
            }
        }
    }
    if (venue == .hyperliquid and operation != null and status >= 200 and status < 300) {
        const op = try wire.string(operation.?);
        if (std.mem.eql(u8, op, "order") or std.mem.eql(u8, op, "cancel") or std.mem.eql(u8, op, "cancel_by_cloid")) {
            const value = try wire.parse(a, bytes);
            const state = try wire.string(try wire.field(value, "status"));
            const response = try wire.field(value, "response");
            if (std.mem.eql(u8, state, "err")) {
                _ = try wire.string(response);
            } else {
                if (!std.mem.eql(u8, state, "ok")) return error.InvalidResponse;
                _ = try wire.string(try wire.field(response, "type"));
                const statuses = try wire.field(try wire.field(response, "data"), "statuses");
                if (statuses != .array) return error.InvalidResponse;
            }
            return value;
        }
    }
    const format = if (operation) |op| try http.responseFormat(venue, try wire.string(op)) else .json;
    if (venue == .monaco and operation != null) {
        const name = try wire.string(operation.?);
        if (std.mem.eql(u8, name, "refresh_session") and status >= 200 and status < 300) {
            const result = try wire.parse(a, bytes);
            var runtime: std.Io.Threaded = .init_single_threaded;
            _ = try @import("../exchanges/monaco/login.zig").expiry(result, runtime.io());
            return result;
        }
        if (@import("../exchanges/monaco/orders.zig").isWrite(name)) {
            if (status == 408 or status >= 500) return error.PendingOutcome;
            if (status >= 200 and status < 300) {
                const reply = wire.parse(a, bytes) catch return error.PendingOutcome;
                if (reply != .object) return error.PendingOutcome;
            }
        }
    }
    if (venue == .hyperdash and (status < 200 or status >= 300)) {
        if (key) |token| if (token == .string and token.string.len > 0) {
            const bearer = try std.mem.concat(a, u8, &.{ "Bearer ", token.string });
            const detail = wire.parse(a, bytes) catch if (std.unicode.utf8ValidateSlice(bytes)) wire.text(bytes) else V.null;
            const scrubbed = try @import("redact.zig").redact(a, detail, bearer);
            @import("http_failure.zig").record(@tagName(venue), status, try std.json.Stringify.valueAlloc(a, scrubbed, .{}));
            return error.ExchangeApi;
        };
    }
    const result = if (format == .json or status < 200 or status >= 300) try http.decode(a, venue, status, bytes) else if (std.unicode.utf8ValidateSlice(bytes)) wire.text(bytes) else return error.InvalidUtf8;
    if (venue == .grvt and operation != null and perps.eq(u8, try wire.string(operation.?), "login"))
        return @import("../exchanges/grvt/trading.zig").loginResponse(a, result, headers);
    if (venue == .dune and format == .csv) {
        var out: V = .{ .object = .empty };
        var offset: V = .null;
        var uri: V = .null;
        for (headers) |h| {
            if (offset == .null and std.ascii.eqlIgnoreCase(h.name, "x-dune-next-offset")) {
                const n = try std.fmt.parseUnsigned(u64, h.value, 10);
                offset = .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{n}) };
            }
            if (uri == .null and std.ascii.eqlIgnoreCase(h.name, "x-dune-next-uri")) uri = wire.text(h.value);
        }
        try out.object.put(a, "data", result);
        try out.object.put(a, "next_offset", offset);
        try out.object.put(a, "next_uri", uri);
        return out;
    }
    const metadata = if (venue == .hyperdash) try @import("../data/hyperdash/metadata.zig").read(a, headers, key) else V.null;
    var selected: std.array_list.Managed(V) = .init(a);
    if (http.oracleVenue(venue) and format != .text) for (headers) |h| {
        if (std.ascii.eqlIgnoreCase(h.name, "retry-after") or std.ascii.eqlIgnoreCase(h.name, "pyth-split-version")) {
            var pair: std.array_list.Managed(V) = .init(a);
            try pair.append(wire.text(h.name));
            try pair.append(wire.text(h.value));
            try selected.append(.{ .array = pair });
        }
    };
    if (http.oracleVenue(venue) or venue == .hyperdash) {
        var out: V = .{ .object = .empty };
        try out.object.put(a, "status", .{ .integer = status });
        try out.object.put(a, "data", result);
        if (venue == .hyperdash) try out.object.put(a, "metadata", metadata) else try out.object.put(a, "headers", .{ .array = selected });
        return out;
    }
    return result;
}

fn refreshMonaco(input: V, result: V, io: std.Io) !void {
    const auth = input.object.get("auth") orelse return;
    if (try http.venue(input) == .monaco and std.mem.eql(u8, try wire.string(try wire.field(input, "operation")), "refresh_session"))
        try @import("../exchanges/monaco/session.zig").refresh(auth, try wExpiry(result), io);
}
fn wExpiry(v: V) !u64 {
    return wire.unsigned(u64, try wire.field(v, "expiresAt"));
}
pub fn finishDispatch(a: A, args: V) !V {
    const status = try wire.field(args, "status");
    if (status != .number_string) return error.InvalidStatus;
    const list = try wire.field(args, "headers");
    if (list != .array) return error.InvalidHeaders;
    var headers: std.ArrayList(std.http.Header) = .empty;
    for (list.array.items) |pair| {
        if (pair != .array or pair.array.items.len != 2) return error.InvalidHeaders;
        try headers.append(a, .{ .name = try wire.string(pair.array.items[0]), .value = try wire.string(pair.array.items[1]) });
    }
    const result = try finish(a, try http.venue(args), try std.fmt.parseInt(u16, status.number_string, 10), headers.items, try wire.string(try wire.field(args, "body")), args.object.get("api_key"), args.object.get("operation"));
    var io: std.Io.Threaded = .init_single_threaded;
    if (hip3.dex(try http.venue(args))) |name| {
        const coin = perps.get(args, "scope_coin");
        if (coin != .null and std.mem.eql(u8, try perps.text(args, "operation"), "order_status"))
            try hip3.validateOrderStatus(name, try wire.string(coin), result);
    }
    try refreshMonaco(args, result, io.io());
    return result;
}
