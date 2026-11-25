const std = @import("std");
const p = @import("perps.zig");
const w = @import("wire.zig");
const http = @import("http.zig");
const Venue = @import("routes.zig").Venue;
const Dial = @import("host_stream.zig").Dial;

pub fn supports(venue: Venue) bool {
    return switch (venue) {
        .perpl, .risex, .decibel, .edgex, .grvt, .katana => true,
        else => false,
    };
}

pub fn origin(venue: Venue, op: []const u8, config: p.V, fallback: []const u8) ![]const u8 {
    const testnet = try p.testnet(config);
    switch (venue) {
        .perpl => if (testnet) return "https://testnet.perpl.xyz",
        .risex => if (testnet) return "https://api.testnet.rise.trade",
        .decibel => {
            const chain = p.eq(u8, op, "transaction") or p.isWrite(venue, op) or p.eq(u8, op, "ledger") or p.eq(u8, op, "gas_price");
            if (chain) {
                const override = p.get(p.get(config, "auth"), "aptos_origin");
                if (override != .null) return w.string(override);
                if (testnet) return "https://api.testnet.aptoslabs.com";
            } else if (testnet) return "https://api.testnet.aptoslabs.com/decibel";
        },
        .edgex => if (testnet) return error.InvalidNetwork,
        .grvt => if (testnet) {
            if (p.eq(u8, op, "login")) return "https://edge.testnet.grvt.io";
            if (p.eq(u8, fallback, "https://trades.grvt.io")) return "https://trades.testnet.grvt.io";
            return "https://market-data.testnet.grvt.io";
        },
        .katana => if (testnet) return "https://api-perps-sandbox.katana.network",
        else => {},
    }
    return fallback;
}

pub fn apiRequest(a: p.A, config: p.V, venue: Venue, name: []const u8, params: p.V, endpoint: p.V) !p.V {
    var request = p.object();
    try p.put(&request, a, "venue", w.text(@tagName(venue)));
    try p.put(&request, a, "operation", w.text(name));
    try p.put(&request, a, "params", params);
    const auth = p.get(config, "auth");
    if (auth != .null) try p.put(&request, a, "auth", auth);
    const key = p.get(config, "api_key");
    if (key != .null) try p.put(&request, a, "api_key", key);
    const secret = p.get(config, "api_secret");
    if (secret != .null) try p.put(&request, a, "api_secret", secret);
    if (endpoint != .null) try p.put(&request, a, "origin", endpoint);
    return request;
}

pub fn lookup(a: p.A, config: p.V, venue: Venue, name: []const u8, params: anytype, dial: ?Dial) !p.V {
    const endpoint = p.get(p.get(config, "auth"), "rest_origin");
    const request = try apiRequest(a, config, venue, name, try paramsValue(a, params), endpoint);
    return @import("transport.zig").executeWithDial(a, request, dial);
}

pub fn decimal(comptime T: type, value: []const u8, scale: u8) !T {
    var parts = std.mem.splitScalar(u8, value, '.');
    const whole = parts.first();
    const frac = parts.next() orelse "";
    if (whole.len == 0 or parts.next() != null or scale > 38) return error.InvalidQuantity;
    var result: T = 0;
    for (whole) |c| {
        if (!std.ascii.isDigit(c)) return error.InvalidQuantity;
        result = try std.math.add(T, try std.math.mul(T, result, 10), c - '0');
    }
    for (0..scale) |i| {
        const c = if (i < frac.len) frac[i] else '0';
        if (!std.ascii.isDigit(c)) return error.InvalidQuantity;
        result = try std.math.add(T, try std.math.mul(T, result, 10), c - '0');
    }
    for (frac[@min(frac.len, scale)..]) |c| if (c != '0') return error.InvalidQuantity;
    return result;
}

pub fn number(a: p.A, value: anytype) !p.V {
    return .{ .number_string = try std.fmt.allocPrint(a, "{d}", .{value}) };
}

pub fn boolean(value: p.V, field: []const u8) !bool {
    return switch (p.get(value, field)) {
        .bool => |flag| flag,
        .null => false,
        else => error.InvalidParameter,
    };
}

pub fn enumIndex(name: []const u8, comptime names: []const []const u8) !u8 {
    inline for (names, 0..) |candidate, i| if (p.eq(u8, name, candidate)) return i;
    return error.InvalidParameter;
}

pub fn json(a: p.A, value: p.V) !?[]const u8 {
    return if (value == .null) null else try std.json.Stringify.valueAlloc(a, value, .{});
}

pub fn bearer(a: p.A, config: p.V, headers: *p.V) !void {
    const key = try p.credential(p.get(config, "api_key"));
    try p.put(headers, a, "authorization", w.text(try std.mem.concat(a, u8, &.{ "Bearer ", key })));
}

pub fn dependencies(a: p.A, venue: Venue, operation: []const u8, input: p.V) !p.V {
    var jobs = p.object();
    const place = p.eq(u8, operation, "place_order");
    const auth = p.get(input, "auth");
    const source = p.get(input, "origin");
    const rest = p.get(auth, "rest_origin");
    const rest_origin = if (rest == .null) source else rest;
    switch (venue) {
        .risex => {
            const account = try p.credential(p.get(auth, "account"));
            _ = try @import("evm.zig").hex(20, account);
            try addJob(a, &jobs, input, venue, "signing_domain", .{}, rest_origin);
            try addJob(a, &jobs, input, venue, "system_config", .{}, rest_origin);
            try addJob(a, &jobs, input, venue, "nonce_state", .{ .account = account }, rest_origin);
        },
        .decibel => {
            if (place) try addJob(a, &jobs, input, venue, "markets", .{}, rest);
            try addJob(a, &jobs, input, venue, "ledger", .{}, source);
            try addJob(a, &jobs, input, venue, "gas_price", .{}, source);
        },
        .edgex => if (place) try addJob(a, &jobs, input, venue, "markets", .{}, rest_origin),
        .grvt => if (place) {
            const body = p.get(p.get(input, "params"), "body");
            const legs = try w.field(body, "legs");
            if (legs != .array or legs.array.items.len != 1) return error.InvalidParameter;
            try addJob(a, &jobs, input, venue, "instrument", .{ .instrument = try p.text(legs.array.items[0], "instrument") }, rest_origin);
        },
        else => {},
    }
    return jobs;
}

fn addJob(a: p.A, jobs: *p.V, config: p.V, venue: Venue, name: []const u8, params: anytype, endpoint: p.V) !void {
    // Public metadata calls do not need order credentials, except Decibel's Geomi gateway.
    var job = try apiRequest(a, config, venue, name, try paramsValue(a, params), endpoint);
    if (venue != .decibel) {
        _ = job.object.swapRemove("api_key");
        _ = job.object.swapRemove("api_secret");
    }
    try p.put(jobs, a, name, job);
}

pub fn checkWrite(venue: Venue, op: []const u8, value: p.V) !void {
    const place = p.eq(u8, op, "place_order");
    switch (venue) {
        .perpl => return @import("../exchanges/perpl/trading.zig").checkOrder(value),
        .risex => {
            const data = p.get(value, "data");
            if (p.get(if (data == .null) value else data, "tx_hash") != .string) return error.PendingOutcome;
        },
        .decibel => {
            if (p.get(value, "hash") != .string) return error.PendingOutcome;
        },
        .edgex => {
            if (p.get(value, "code") != .string) return error.PendingOutcome;
            const data = p.get(value, "data");
            if (place) {
                if (p.get(data, "orderId") != .string) return error.PendingOutcome;
            } else {
                const results = p.get(data, "cancelResultMap");
                if (results != .object or results.object.count() != 1) return error.PendingOutcome;
                const result = w.string(results.object.values()[0]) catch return error.PendingOutcome;
                if (!p.eq(u8, result, "SUCCESS")) return error.ExchangeApi;
            }
        },
        .grvt => {
            const data = p.get(value, "result");
            if (place) {
                if (p.get(data, "order_id") != .string) return error.PendingOutcome;
            } else {
                const ack = p.get(data, "ack");
                if (ack != .bool) return error.PendingOutcome;
                if (!ack.bool) return error.ExchangeApi;
            }
        },
        .katana => if (place) {
            if (p.get(value, "orderId") != .string) return error.PendingOutcome;
        } else {
            if (value != .array or value.array.items.len != 1) return error.PendingOutcome;
            const status = p.text(value.array.items[0], "status") catch return error.PendingOutcome;
            if (!p.eq(u8, status, "canceled")) return error.ExchangeApi;
        },
        else => unreachable,
    }
}

fn paramsValue(a: p.A, params: anytype) !p.V {
    // An untyped empty Zig literal is a tuple, while HTTP params are an object.
    return if (@TypeOf(params) == @TypeOf(.{})) p.object() else p.toValue(a, params);
}
